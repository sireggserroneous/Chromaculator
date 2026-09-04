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
//! v13-M3c (S2a) added three more, and the measurement is the reason each is
//! here rather than the reading that suggested it. On wallpaper.jpg's
//! 1,233,099 B of modelled values:
//!   - the block's own LAST NONZERO INDEX, in `mag` (-36,167 B) and, as the
//!     DISTANCE `last - k`, in `nz` (-18,315 B). Both are known to the decoder
//!     before the AC loop starts, because `last` is coded first.
//!   - the running nonzero count in `mag` (-4,962 B), widened from 4 steps to
//!     8 (-1,941 B), and the neighbour magnitude buckets widened from 4 steps
//!     to 8 (-1,840 B).
//!   - the neighbour magnitudes in `mbits`, which LOSE 993 B on their own and
//!     WIN 1,811 B behind the count gate.
//!
//! Two inputs the plan named were measured and DELETED: `qb[k]` in `mag`
//! (+14 B -- the quantisation table is nearly a function of the band, and the
//! band was already there) and the block's DC magnitude in `last` (+92 B).
//!
//! Shape: one adaptive 16-bit probability per context, count-adaptive rate, no
//! mixer -- packJPG's shape rather than Lepton's, chosen for the house speed
//! floor (0.25 MB/s) and printed as that choice; M3c re-priced a mixer against
//! a measured clock and refused it again. What M3c DID add is the COUNT GATE
//! (see `blend` below), which is what makes contexts this fine affordable.
//! Attribution: packJPG (Matthias Stirner) and Lepton (Dropbox) for the
//! context shape; Matt Mahoney's StateMap (zpaq/lpaq/paq8) for the
//! count-adaptive probability and for the idea of backing a sparse context
//! with a dense one; ITU T.81 for everything the coefficients mean.
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
/// THE COUNT GATE (v13-M3c, S2a-4c). A fine context is trusted in proportion
/// to how much it has SEEN: the coded probability is the COARSE parent's,
/// pulled toward the fine child by `n / (n + KGATE)`. Both counters update
/// either way, so the fine one warms up while the coarse one carries the
/// decision. This is what makes the fine contexts affordable -- `nz` reaches
/// 786,432 contexts over 3.5M codings (4.5 each) and `mag` 196,608 over 1.6M
/// (8.2 each), and the SAME widening measured a LOSS before the gate existed.
///
/// KGATE = 4 is the sweep winner, measured on wallpaper.jpg's values:
/// 1 -> 1,160,670; 2 -> 1,159,728; **4 -> 1,159,378**; 8 -> 1,160,200. The
/// hard-step form of the same gate (coarse below the threshold, fine above)
/// was measured beside it and is worse at every threshold (best 1,162,201 at
/// 4), so the proportional law ships.
///
/// No mixer, no stretch, no squash -- one multiply, one table read and one
/// extra counter update, which is what keeps this model inside the 0.25 MB/s
/// house floor. (A mixer was priced and refused: `peel_arm` already runs at
/// PARITY with the roster on this row, 4,749 ms against 4,528 ms, so 2-3x the
/// model work would put the row at 0.11-0.17 MB/s.)
const KGATE: u32 = 4;
static WGT: [u32; NLIMIT as usize + 1] = {
    let mut t = [0u32; NLIMIT as usize + 1];
    let mut i = 0usize;
    while i <= NLIMIT as usize {
        t[i] = (65536 * i as u32) / (i as u32 + KGATE);
        i += 1;
    }
    t
};
#[inline]
fn blend(f: &Pr, c: &Pr) -> u16 {
    // i64, and it is not decoration: the difference reaches +-65,408 and the
    // weight 65,536, so the product overflows i32 and wraps SILENTLY in
    // release. The first cut of this function was i32 and measured a 163,116 B
    // loss for the gate; the control that caught it was KGATE = 0, which must
    // reproduce the ungated build to the byte and did not. It does now.
    let w = WGT[f.n as usize] as i64;
    let p = c.p as i64 + (((f.p as i64 - c.p as i64) * w) >> 16);
    p.clamp(PLO as i64, PHI as i64) as u16
}

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
    /// code against the count gate: the pair (fine, coarse), both updated
    fn bit2(&mut self, f: &mut Pr, c: &mut Pr, want: u32) -> u32;
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
    #[inline]
    fn bit2(&mut self, f: &mut Pr, c: &mut Pr, want: u32) -> u32 {
        self.0.bit(blend(f, c), want);
        f.update(want);
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
    #[inline]
    fn bit2(&mut self, f: &mut Pr, c: &mut Pr, _want: u32) -> u32 {
        let b = self.0.bit(blend(f, c));
        f.update(b);
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

/// the same tree, gated: `fine` and `coarse` are two slices of equal shape and
/// every node is decided by the pair
#[inline]
fn tree2<C: Coder>(co: &mut C, fine: &mut [Pr], coarse: &mut [Pr], nbits: u32, want: u32) -> u32 {
    let mut node = 1usize;
    for i in (0..nbits).rev() {
        let b = (want >> i) & 1;
        let got = co.bit2(&mut fine[node], &mut coarse[node], b);
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
    match a {
        0 => 0,
        1 => 1,
        2..=3 => 2,
        4..=7 => 3,
        8..=15 => 4,
        16..=31 => 5,
        32..=95 => 6,
        _ => 7,
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
/// the DC gradient ACROSS this block -- right minus left, below minus above --
/// in GS signed steps. Only a walk that decides every DC before any AC can
/// take it, which is what the scatter bought.
const GS: usize = 9;
#[inline]
fn gbucket(d: i32) -> usize {
    match d {
        i32::MIN..=-33 => 0,
        -32..=-9 => 1,
        -8..=-3 => 2,
        -2..=-1 => 3,
        0 => 4,
        1..=2 => 5,
        3..=8 => 6,
        9..=32 => 7,
        _ => 8,
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
/// a neighbour's magnitude bucket (`mbucket`), the running nonzero count of the
/// block capped at 3, and the quantisation bucket (`qbucket`). Named so the
/// table dimensions below and the census read ONE number each.
const MB: usize = 8;
const NZB: usize = 8;
const QB: usize = 4;
/// how many CONTEXTS each of the two tables the M3c lever touches carries --
/// the tree width (16) multiplies these, and the census divides by them
/// how far a position is from the block's own last nonzero, in eight steps
const NR: usize = 8;
const LAST_CTX: usize = NC * 8 * 8;
const NZC_CTX: usize = NC * KB * NZB * NR;
const MAGC_CTX: usize = NC * KB * NR;
const NZ_CTX: usize = NC * KB * MB * MB * NZB * QB * NR;
const MAG_CTX: usize = NC * KB * MB * MB * NZB * NR;
const MBITSC_CTX: usize = NC * KB * 16;
const MBITS_CTX: usize = NC * KB * 16 * MB * MB;

struct Model {
    /// the last-nonzero index of the block: a 6-bit tree
    last: Vec<Pr>, // [NC][8][8][64]
    /// is this coefficient nonzero?
    nz: Vec<Pr>, // [NC][KB][4][4][4][4]
    /// the magnitude class, a 4-bit tree over (bits-1)
    mag: Vec<Pr>, // [NC][KB][4][4][16]
    /// the bits below the leading one, with the two neighbour magnitude
    /// buckets and a coarse parent behind the count gate
    mbits: Vec<Pr>, // [NC][KB][16][MB][MB][16]
    mbits_c: Vec<Pr>, // [NC][KB][16][16]
    /// the COARSE parents of `nz` and `mag`: the same decisions over a small
    /// dense index, which the count gate falls back on while the fine
    /// context is cold
    nz_c: Vec<Pr>,
    mag_c: Vec<Pr>,
    /// the sign
    sign: Vec<Pr>, // [NC][KB][3][3][GS][GS]
    /// the DC difference from the 2D predictor: a 5-bit category tree
    dcmag: Vec<Pr>, // [NC][4][8][NR][32]
    dcbits: Vec<Pr>, // [NC][18][16]
    /// the sign of that difference, by component and by the neighbourhood's
    /// activity -- NEVER by the value being coded (a context the decoder
    /// cannot rebuild is a broken model, not a clever one)
    dcsign: Vec<Pr>, // [NC][8]
    /// EGG_JSTATS: the census. An INSTRUMENT, not a context -- it is written
    /// by the walk and read by nobody inside it, so both directions agree.
    /// (blocks, nonzero ACs = also the `sign` codings, `mag` codings,
    /// `mbits` bit codings)
    census: [u64; 5],
}
impl Model {
    fn new() -> Box<Model> {
        Box::new(Model {
            last: vec![Pr::new(); LAST_CTX * 64],
            nz: vec![Pr::new(); NZ_CTX],
            mag: vec![Pr::new(); MAG_CTX * 16],
            mbits: vec![Pr::new(); MBITS_CTX * 16],
            mbits_c: vec![Pr::new(); MBITSC_CTX * 16],
            nz_c: vec![Pr::new(); NZC_CTX],
            mag_c: vec![Pr::new(); MAGC_CTX * 16],
            sign: vec![Pr::new(); NC * KB * 3 * 3 * GS * GS],
            dcmag: vec![Pr::new(); NC * 4 * 8 * NR * 32],
            dcbits: vec![Pr::new(); NC * 18 * 16],
            dcsign: vec![Pr::new(); NC * 8],
            census: [0; 5],
        })
    }
    /// every adaptive counter this model carries -- printed by the census so
    /// the table budget is a measured number and not a remembered one
    fn counters(&self) -> usize {
        self.last.len()
            + self.nz.len()
            + self.mag.len()
            + self.mbits.len()
            + self.mbits_c.len()
            + self.nz_c.len()
            + self.mag_c.len()
            + self.sign.len()
            + self.dcmag.len()
            + self.dcbits.len()
            + self.dcsign.len()
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

/// THE SHARED WALK, in THREE PASSES over one component plane (v13-M3c, S2b --
/// the scatter). Reading: `spectrometer.html:464-466`, disjoint regions summed
/// independently with every symbol carrying its index, so the merge is exact.
///
/// The pure re-ordering buys NOTHING and was measured as the control before
/// this was built. What it buys is LEGALITY: a decision taken in a later pass
/// may read anything an earlier pass decided, in any direction, so the model
/// gets a TWO-SIDED reading of the neighbourhood where a block-order walk can
/// only ever look up and left.
///
///   pass A -- the last-nonzero index of every block;
///   pass B -- the DC plane, which may now read its own block's `last`;
///   pass C -- the ACs, band by band, whose coarse contexts may now read the
///             `last` of the blocks to the RIGHT and BELOW.
///
/// Refuses with a number the moment a decoded symbol cannot be a coefficient --
/// a wounded or foreign stream stops here rather than writing nonsense that the
/// FNV-64 gate would have to catch downstream.
fn walk<C: Coder>(co: &mut C, m: &mut Model, j: &mut Jpeg, ci: usize) -> Result<(), String> {
    let cc = ci.min(NC - 1);
    let c = j.comps[ci].clone();
    let q = j.qt[c.tq];
    let qb: Vec<usize> = (0..64).map(|k| qbucket(q[k])).collect();
    let mut lastplane = vec![0u8; c.bw * c.bh];

    // ---------------- pass A: the last-nonzero plane
    for by in 0..c.bh {
        for bx in 0..c.bw {
            m.census[0] += 1;
            let base = block_at(&c, bx, by);
            let la = lbucket(if by > 0 { lastplane[(by - 1) * c.bw + bx] as usize } else { 0 });
            let ll = lbucket(if bx > 0 { lastplane[by * c.bw + bx - 1] as usize } else { 0 });
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
            lastplane[by * c.bw + bx] = tree(co, &mut m.last[lb..lb + 64], 6, want_last) as u8;
        }
    }

    // ---------------- pass B: the DC plane, against the two-dimensional predictor
    for by in 0..c.bh {
        for bx in 0..c.bw {
            let base = block_at(&c, bx, by);
            let left = if bx > 0 { Some(block_at(&c, bx - 1, by)) } else { None };
            let above = if by > 0 { Some(block_at(&c, bx, by - 1)) } else { None };
            let al = if bx > 0 && by > 0 { Some(block_at(&c, bx - 1, by - 1)) } else { None };
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
            // legal ONLY because pass A ran first: this block's own AC activity
            let own = lbucket(lastplane[by * c.bw + bx] as usize);
            let tbase = (((cc * 4 + dq) * 8 + act) * NR + own) * 32;
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
        }
    }

    // ---------------- pass C: the ACs, one band at a time
    let mut nzcplane = vec![0u8; c.bw * c.bh];
    #[allow(clippy::needless_range_loop)] // k is the zigzag INDEX: it addresses
    // the block, the quantisation table and both neighbours at once
    for k in 1..64 {
        let kb = kbucket(k);
        for by in 0..c.bh {
            for bx in 0..c.bw {
                let bi = by * c.bw + bx;
                let last = lastplane[bi] as usize;
                if k > last {
                    continue;
                }
                let base = block_at(&c, bx, by);
                let left = if bx > 0 { Some(block_at(&c, bx - 1, by)) } else { None };
                let above = if by > 0 { Some(block_at(&c, bx, by - 1)) } else { None };
                let ba = mbucket(above.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let bl = mbucket(left.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let nzb = (nzcplane[bi] as usize).min(NZB - 1);
                let v = if C::ENCODING { j.coef[ci][base + k] } else { 0 };
                let is_nz = if k == last {
                    // the last coefficient is nonzero by construction: free
                    1u32
                } else {
                    let idx = (((((cc * KB + kb) * MB + ba) * MB + bl) * NZB + nzb) * QB + qb[k]) * NR
                        + lbucket(last - k);
                    m.census[4] += 1;
                    let cidx = ((cc * KB + kb) * NZB + nzb) * NR + lbucket(last - k);
                    let w = if C::ENCODING && v != 0 { 1 } else { 0 };
                    co.bit2(&mut m.nz[idx], &mut m.nz_c[cidx], w)
                };
                if is_nz == 0 {
                    continue;
                }
                nzcplane[bi] = nzcplane[bi].saturating_add(1);
                m.census[1] += 1;
                m.census[2] += 1;
                // the magnitude class, then the bits below the leading one
                let mbase =
                    (((((cc * KB + kb) * MB + ba) * MB + bl) * NZB + nzb) * NR + lbucket(last)) * 16;
                let want_m = if C::ENCODING { nbits_of(v as i32) - 1 } else { 0 };
                let cbase = ((cc * KB + kb) * NR + lbucket(last)) * 16;
                let mm = tree2(
                    co,
                    &mut m.mag[mbase..mbase + 16],
                    &mut m.mag_c[cbase..cbase + 16],
                    4,
                    want_m.min(15),
                ) + 1;
                if mm > 15 {
                    return Err(format!("coefficient stream: magnitude class {} is not a class", mm));
                }
                let cb = ((cc * KB + kb) * 16 + mm as usize) * 16;
                let bbase = ((((cc * KB + kb) * 16 + mm as usize) * MB + ba) * MB + bl) * 16;
                let mut node = 1usize;
                let mut low = 0u32;
                m.census[3] += mm as u64 - 1;
                for i in (0..mm - 1).rev() {
                    let w = ((v.unsigned_abs() as u32) >> i) & 1;
                    let got =
                        co.bit2(&mut m.mbits[bbase + node.min(15)], &mut m.mbits_c[cb + node.min(15)], w);
                    low = (low << 1) | got;
                    node = ((node << 1) | got as usize).min(15);
                }
                let mag = (1i32 << (mm - 1)) | low as i32;
                let sa = sbucket(above.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                let sl = sbucket(left.map(|o| j.coef[ci][o + k]).unwrap_or(0));
                // the two-sided DC gradient, legal ONLY because pass B decided
                // every DC before any AC: a block-order walk has no right and
                // no below, and this is the single biggest context in the model
                let dcl = if bx > 0 { j.coef[ci][block_at(&c, bx - 1, by)] as i32 } else { 0 };
                let dcr = if bx + 1 < c.bw { j.coef[ci][block_at(&c, bx + 1, by)] as i32 } else { 0 };
                let dca = if by > 0 { j.coef[ci][block_at(&c, bx, by - 1)] as i32 } else { 0 };
                let dcb = if by + 1 < c.bh { j.coef[ci][block_at(&c, bx, by + 1)] as i32 } else { 0 };
                let gx = gbucket(dcr - dcl);
                let gy = gbucket(dcb - dca);
                let sidx = ((((cc * KB + kb) * 3 + sa) * 3 + sl) * GS + gx) * GS + gy;
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
    if std::env::var_os("EGG_JSTATS").is_some() {
        let [blocks, nz, mags, mbits, nzcode] = m.census;
        eprintln!(
            "jstats: {} blocks, {} nonzero ACs ({:.2}/block), {} mag codings over {} contexts ({:.1} each), {} mbits bits over {} contexts ({:.1} each), {} nz codings over {} contexts ({:.1} each), tables {} Pr = {} B",
            blocks,
            nz,
            nz as f64 / blocks.max(1) as f64,
            mags,
            MAG_CTX,
            mags as f64 / MAG_CTX as f64,
            mbits,
            MBITS_CTX,
            mbits as f64 / MBITS_CTX as f64,
            nzcode,
            NZ_CTX,
            nzcode as f64 / NZ_CTX as f64,
            m.counters(),
            m.counters() * std::mem::size_of::<Pr>(),
        );
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
