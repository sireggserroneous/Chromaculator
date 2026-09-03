//! jcoef.rs -- the JPEG COEFFICIENT MODEL (v13-M1, WS-J). This is the
//! milestone; the peel in jpeg.rs is only the door.
//!
//! v12 proved the door opens and then measured what is behind it: a RAW
//! coefficient dump (64 x i16 per block, 55,296,000 B for wallpaper.jpg) is
//! 2,192,684 B under xz -9 and 2,208,961 B under our own MIX12 arm -- both
//! HEAVIER than the JPEG's own 1,602,311 B of Huffman. A dump is not a model.
//! The model is what makes the value underneath lighter than its colours.
//!
//! The contexts, exactly as the charter plan names them: the band index (DC /
//! the first ACs / the tail), the same position in the block ABOVE and to the
//! LEFT, the count of nonzeros already placed in this block, the QUANTISATION
//! value at that position, and the COMPONENT.
//!
//! Shape: one adaptive 16-bit probability per context, count-adaptive rate, no
//! mixer -- packJPG's shape rather than Lepton's, chosen for the house speed
//! floor (0.25 MB/s) and printed as that choice. Attribution: packJPG (Matthias
//! Stirner) and Lepton (Dropbox) for the context shape; Matt Mahoney's
//! StateMap (zpaq/lpaq/paq8) for the count-adaptive probability; ITU T.81 for
//! everything the coefficients mean.
//!
//! Encoder and decoder are ONE routine driven through a `Coder` trait, so the
//! two sides cannot drift: every decision is taken in the same order with the
//! same context, and the decoder's returned bit is what the shared code walks
//! on. (v12's M3 lesson, generalised: the two sides must read one constant --
//! here, one function.)

use crate::dyadic::{WDec, WEnc};
use crate::jpeg::{block_at, Jpeg};

// ---------------------------------------------------------------- the counter

/// P(bit == 0) in 1..65535, with a count-adaptive step: the first observations
/// move it far, later ones little. `n` caps at NLIMIT so the model can still
/// track a change of régime inside one image.
#[derive(Clone, Copy)]
struct Pr {
    p: u16,
    n: u16,
}
const NLIMIT: u16 = 127;
const PLO: i32 = 64;
const PHI: i32 = 65_472;
/// RATE[n] = 65536 / (n + 2): the reciprocal, so the update is a multiply
static RATE: [u32; NLIMIT as usize + 1] = {
    let mut t = [0u32; NLIMIT as usize + 1];
    let mut i = 0usize;
    while i <= NLIMIT as usize {
        t[i] = 65536 / (i as u32 + 2);
        i += 1;
    }
    t
};
impl Pr {
    #[inline]
    fn new() -> Pr {
        Pr { p: 32768, n: 0 }
    }
    #[inline]
    fn update(&mut self, b: u32) {
        let target: i32 = if b == 0 { 65535 } else { 0 };
        let d = ((target - self.p as i32) * RATE[self.n as usize] as i32) >> 16;
        self.p = (self.p as i32 + d).clamp(PLO, PHI) as u16;
        if self.n < NLIMIT {
            self.n += 1;
        }
    }
}

// ---------------------------------------------------------------- one routine, two directions

trait Coder {
    const ENCODING: bool;
    /// code `want` (encoding) or decode (decoding) at this counter; returns the
    /// bit that actually went through, which is what the shared walk follows
    fn bit(&mut self, c: &mut Pr, want: u32) -> u32;
}
struct Enc(WEnc);
impl Coder for Enc {
    const ENCODING: bool = true;
    #[inline]
    fn bit(&mut self, c: &mut Pr, want: u32) -> u32 {
        self.0.bit(c.p, want);
        c.update(want);
        want
    }
}
struct Dec<'a>(WDec<'a>);
impl Coder for Dec<'_> {
    const ENCODING: bool = false;
    #[inline]
    fn bit(&mut self, c: &mut Pr, _want: u32) -> u32 {
        let b = self.0.bit(c.p);
        c.update(b);
        b
    }
}

/// an `nbits`-deep binary tree over `tab` (length 1 << nbits); returns the value
#[inline]
fn tree<C: Coder>(co: &mut C, tab: &mut [Pr], nbits: u32, want: u32) -> u32 {
    let mut node = 1usize;
    for i in (0..nbits).rev() {
        let b = (want >> i) & 1;
        let got = co.bit(&mut tab[node], b);
        node = (node << 1) | got as usize;
    }
    node as u32 - (1 << nbits)
}

// ---------------------------------------------------------------- the contexts

/// the band index: every one of the first nine AC positions gets its own, the
/// tail is bucketed -- the low bands carry nearly all the value
#[inline]
fn kbucket(k: usize) -> usize {
    match k {
        1..=8 => k - 1,
        9..=10 => 8,
        11..=13 => 9,
        14..=17 => 10,
        18..=22 => 11,
        23..=28 => 12,
        29..=36 => 13,
        37..=47 => 14,
        _ => 15,
    }
}
const KB: usize = 16;
/// the magnitude of a neighbour coefficient, in four steps
#[inline]
fn mbucket(v: i16) -> usize {
    let a = v.unsigned_abs() as u32;
    if a == 0 {
        0
    } else if a == 1 {
        1
    } else if a <= 3 {
        2
    } else {
        3
    }
}
/// the quantisation value at this position, in four steps
#[inline]
fn qbucket(q: u16) -> usize {
    if q <= 2 {
        0
    } else if q <= 8 {
        1
    } else if q <= 32 {
        2
    } else {
        3
    }
}
/// the last-nonzero index of a neighbouring block, in eight steps
#[inline]
fn lbucket(l: usize) -> usize {
    match l {
        0 => 0,
        1 => 1,
        2 => 2,
        3..=4 => 3,
        5..=8 => 4,
        9..=16 => 5,
        17..=32 => 6,
        _ => 7,
    }
}
#[inline]
fn sbucket(v: i16) -> usize {
    match v.cmp(&0) {
        std::cmp::Ordering::Less => 0,
        std::cmp::Ordering::Equal => 1,
        std::cmp::Ordering::Greater => 2,
    }
}
#[inline]
fn abucket(a: i32) -> usize {
    match a {
        0 => 0,
        1..=1 => 1,
        2..=3 => 2,
        4..=7 => 3,
        8..=15 => 4,
        16..=31 => 5,
        32..=95 => 6,
        _ => 7,
    }
}
#[inline]
fn nbits_of(v: i32) -> u32 {
    let mut a = v.unsigned_abs();
    let mut s = 0u32;
    while a != 0 {
        s += 1;
        a >>= 1;
    }
    s
}

const NC: usize = 3; // component classes: luma, chroma, everything else

struct Model {
    /// the last-nonzero index of the block: a 6-bit tree
    last: Vec<Pr>, // [NC][8][8][64]
    /// is this coefficient nonzero?
    nz: Vec<Pr>, // [NC][KB][4][4][4][4]
    /// the magnitude class, a 4-bit tree over (bits-1)
    mag: Vec<Pr>, // [NC][KB][4][4][16]
    /// the bits below the leading one
    mbits: Vec<Pr>, // [NC][KB][16][16]
    /// the sign
    sign: Vec<Pr>, // [NC][KB][3][3]
    /// the DC difference from the 2D predictor: a 5-bit category tree
    dcmag: Vec<Pr>, // [NC][4][8][32]
    dcbits: Vec<Pr>, // [NC][18][16]
    /// the sign of that difference, by component and by the neighbourhood's
    /// activity -- NEVER by the value being coded (a context the decoder
    /// cannot rebuild is a broken model, not a clever one)
    dcsign: Vec<Pr>, // [NC][8]
}
impl Model {
    fn new() -> Box<Model> {
        Box::new(Model {
            last: vec![Pr::new(); NC * 8 * 8 * 64],
            nz: vec![Pr::new(); NC * KB * 4 * 4 * 4 * 4],
            mag: vec![Pr::new(); NC * KB * 4 * 4 * 16],
            mbits: vec![Pr::new(); NC * KB * 16 * 16],
            sign: vec![Pr::new(); NC * KB * 3 * 3],
            dcmag: vec![Pr::new(); NC * 4 * 8 * 32],
            dcbits: vec![Pr::new(); NC * 18 * 16],
            dcsign: vec![Pr::new(); NC * 8],
        })
    }
}

/// the MED predictor of LOCO-I / JPEG-LS, run on the DC plane: the DC of a
/// block is the average of its 64 samples, so the DCs form a picture of their
/// own and the picture's own predictor is the right one. (JPEG spends its DC
/// on a 1-D difference along the scan and resets it at every restart; this one
/// reads two dimensions and never resets.)
#[inline]
fn med(l: i32, a: i32, al: i32) -> i32 {
    if al >= l.max(a) {
        l.min(a)
    } else if al <= l.min(a) {
        l.max(a)
    } else {
        l + a - al
    }
}

/// the shared walk: one component plane, raster order, every block. Refuses
/// with a number the moment a decoded symbol cannot be a coefficient -- a
/// wounded or foreign stream stops here rather than writing nonsense that the
/// FNV-64 gate would have to catch downstream.
fn walk<C: Coder>(co: &mut C, m: &mut Model, j: &mut Jpeg, ci: usize) -> Result<(), String> {
    let cc = ci.min(NC - 1);
    let c = j.comps[ci].clone();
    let q = j.qt[c.tq];
    let qb: Vec<usize> = (0..64).map(|k| qbucket(q[k])).collect();
    let mut lastplane = vec![0u8; c.bw * c.bh];
    for by in 0..c.bh {
        for bx in 0..c.bw {
            let base = block_at(&c, bx, by);
            let left = if bx > 0 { Some(block_at(&c, bx - 1, by)) } else { None };
            let above = if by > 0 { Some(block_at(&c, bx, by - 1)) } else { None };
            let al = if bx > 0 && by > 0 { Some(block_at(&c, bx - 1, by - 1)) } else { None };

            // ---- the DC, against the two-dimensional predictor
            let lv = left.map(|o| j.coef[ci][o] as i32);
            let av = above.map(|o| j.coef[ci][o] as i32);
            let alv = al.map(|o| j.coef[ci][o] as i32);
            let pred = match (lv, av, alv) {
                (Some(l), Some(a), Some(x)) => med(l, a, x),
                (Some(l), Some(a), None) => (l + a) >> 1,
                (Some(l), None, _) => l,
                (None, Some(a), _) => a,
                (None, None, _) => 0,
            };
            let act = abucket(match (lv, av, alv) {
                (Some(l), Some(a), Some(x)) => (l - x).abs() + (a - x).abs(),
                (Some(l), Some(a), None) => (l - a).abs(),
                _ => 0,
            });
            let dq = qbucket(q[0]);
            let want_d = if C::ENCODING { j.coef[ci][base] as i32 - pred } else { 0 };
            let want_t = nbits_of(want_d);
            let tbase = ((cc * 4 + dq) * 8 + act) * 32;
            let t = tree(co, &mut m.dcmag[tbase..tbase + 32], 5, want_t.min(31));
            if t > 17 {
                return Err(format!("coefficient stream: DC category {} is not a category", t));
            }
            let mut d = 0i32;
            if t > 0 {
                let sbase = cc * 8 + act;
                let wsign = if C::ENCODING && want_d < 0 { 1 } else { 0 };
                let neg = co.bit(&mut m.dcsign[sbase], wsign);
                let bbase = (cc * 18 + t as usize) * 16;
                let mut node = 1usize;
                let mut low = 0u32;
                for i in (0..t - 1).rev() {
                    let w = ((want_d.unsigned_abs()) >> i) & 1;
                    let got = co.bit(&mut m.dcbits[bbase + node.min(15)], w);
                    low = (low << 1) | got;
                    node = ((node << 1) | got as usize).min(15);
                }
                let mag = (1i32 << (t - 1)) | low as i32;
                d = if neg == 1 { -mag } else { mag };
            }
            let dcv = pred + d;
            if !C::ENCODING {
                if !(-32768..=32767).contains(&dcv) {
                    return Err("coefficient stream: a DC leaves the 16-bit range".into());
                }
                j.coef[ci][base] = dcv as i16;
            }

            // ---- the ACs: the last nonzero first, then the coefficients
            let la = lbucket(above.map(|_| lastplane[(by - 1) * c.bw + bx] as usize).unwrap_or(0));
            let ll = lbucket(left.map(|_| lastplane[by * c.bw + bx - 1] as usize).unwrap_or(0));
            let lb = ((cc * 8 + la) * 8 + ll) * 64;
            let want_last = if C::ENCODING {
                let mut l = 0usize;
                for k in (1..64).rev() {
                    if j.coef[ci][base + k] != 0 {
                        l = k;
                        break;
                    }
                }
                l as u32
            } else {
                0
            };
            let last = tree(co, &mut m.last[lb..lb + 64], 6, want_last) as usize;
            lastplane[by * c.bw + bx] = last as u8;

            let mut nzc = 0usize;
            #[allow(clippy::needless_range_loop)] // k is the zigzag INDEX: it addresses
            // the block, the quantisation table and both neighbours at once
            for k in 1..=last {
                let kb = kbucket(k);
                let ba = mbucket(above.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let bl = mbucket(left.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let nzb = nzc.min(3);
                let v = if C::ENCODING { j.coef[ci][base + k] } else { 0 };
                let is_nz = if k == last {
                    // the last coefficient is nonzero by construction: free
                    1u32
                } else {
                    let idx = ((((cc * KB + kb) * 4 + ba) * 4 + bl) * 4 + nzb) * 4 + qb[k];
                    let w = if C::ENCODING && v != 0 { 1 } else { 0 };
                    co.bit(&mut m.nz[idx], w)
                };
                if is_nz == 0 {
                    continue;
                }
                nzc += 1;
                // the magnitude class, then the bits below the leading one
                let mbase = (((cc * KB + kb) * 4 + ba) * 4 + bl) * 16;
                let want_m = if C::ENCODING { nbits_of(v as i32) - 1 } else { 0 };
                let mm = tree(co, &mut m.mag[mbase..mbase + 16], 4, want_m.min(15)) + 1;
                if mm > 15 {
                    return Err(format!("coefficient stream: magnitude class {} is not a class", mm));
                }
                let bbase = ((cc * KB + kb) * 16 + mm as usize) * 16;
                let mut node = 1usize;
                let mut low = 0u32;
                for i in (0..mm - 1).rev() {
                    let w = ((v.unsigned_abs() as u32) >> i) & 1;
                    let got = co.bit(&mut m.mbits[bbase + node.min(15)], w);
                    low = (low << 1) | got;
                    node = ((node << 1) | got as usize).min(15);
                }
                let mag = (1i32 << (mm - 1)) | low as i32;
                let sa = sbucket(above.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let sl = sbucket(left.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let sidx = ((cc * KB + kb) * 3 + sa) * 3 + sl;
                let wsign = if C::ENCODING && v < 0 { 1 } else { 0 };
                let neg = co.bit(&mut m.sign[sidx], wsign);
                if !C::ENCODING {
                    if mag > 32767 {
                        return Err("coefficient stream: a magnitude leaves the 16-bit range".into());
                    }
                    j.coef[ci][base + k] = if neg == 1 { -(mag as i16) } else { mag as i16 };
                }
            }
        }
    }
    Ok(())
}

/// the coefficient planes -> one stream. `j` is taken by &mut only so encode and
/// decode can share ONE walk; the encoding direction never writes a coefficient.
pub fn encode(j: &mut Jpeg) -> Vec<u8> {
    let mut m = Model::new();
    let mut co = Enc(WEnc::new());
    for ci in 0..j.comps.len() {
        walk(&mut co, &mut m, j, ci).expect("the encoding direction cannot refuse its own coefficients");
    }
    co.0.finish()
}

/// one stream -> the coefficient planes of `j` (whose skeleton the recipe made)
pub fn decode(stream: &[u8], j: &mut Jpeg) -> Result<(), String> {
    let mut m = Model::new();
    let mut co = Dec(WDec::new(stream));
    for ci in 0..j.comps.len() {
        walk(&mut co, &mut m, j, ci)?;
    }
    Ok(())
}

/// the raw weight of what the model codes: 2 bytes per coefficient, the size of
/// the dump v12 measured. Printed so the recipe-and-values arithmetic is honest.
pub fn raw_len(j: &Jpeg) -> usize {
    j.nblocks() * 128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_stays_inside_the_coder_range() {
        let mut c = Pr::new();
        for _ in 0..100_000 {
            c.update(0);
        }
        assert!(c.p as i32 <= PHI && c.p as i32 >= PLO);
        let mut c2 = Pr::new();
        for _ in 0..100_000 {
            c2.update(1);
        }
        assert!(c2.p as i32 >= PLO && c2.p as i32 <= PHI);
    }

    /// the whole model round-trips the corpus JPEG's coefficients
    #[test]
    fn coefficients_round_trip() {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wallpaper.jpg");
        let src = std::fs::read(p).expect("corpus-real/wallpaper.jpg present");
        let mut j = crate::jpeg::peel(&src).expect("peels");
        let stream = encode(&mut j);
        let recipe = crate::jpeg::recipe_bytes(&j);
        let mut back = crate::jpeg::from_recipe(&recipe).expect("recipe");
        decode(&stream, &mut back).expect("decode");
        assert_eq!(back.coef, j.coef, "the coefficient planes differ");
        assert!(crate::jpeg::respell(&back).expect("respell") == src);
    }
}
