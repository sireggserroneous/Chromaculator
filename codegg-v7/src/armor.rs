//! armor.rs -- eggv6's shield, lifted whole (codegg-v6/src/main.rs, kept
//! untouched over there; this is the extraction the v7 plan called for).
//!
//! Three nested scales, the Spectrometer lesson:
//!   BIT SCALE     per-square residues, V mod 8219 / 8221 (injective for
//!                 +-2^k, k<4096, verified by enumeration in v6): repair 1-2
//!                 flipped bits locally and SELF-DIAGNOSE bad squares.
//!   SQUARE SCALE  T Reed-Solomon parity squares per group of G over GF(256)
//!                 (the Wub reading: a group is a polynomial, parities are
//!                 extra evaluations); group striping so a contiguous wound
//!                 touches each group at most once per stripe length.
//!   FILE SCALE    three headers at far-apart offsets, byte-voted; FNV-64 of
//!                 the armored payload plus a retry ladder whose last rung
//!                 (ParityOnly) kills fake bit-repairs from random wound fill.
//!
//! v7 changes exactly two things and adds nothing to the machinery:
//!   - the header grows 40 -> 64 bytes and carries the CONSERVATION fields:
//!     original length, FNV-64 of the ORIGINAL bytes (the first-law check --
//!     the information never moves, only the form does), and a model byte.
//!     The old fields (armored length + FNV of the armored stream) stay and
//!     localize which stage failed.
//!   - g == 0 means NO ARMOR: [hdr][payload][hdr][hdr], checks + hash only,
//!     zero parity, zero residues. For pure-weight benchmarks.
//!
//! Attribution: residues Avizienis 1971 / Mandelbaum 1976; Reed-Solomon 1960
//! via Lagrange evaluation; the site supplied the geometry and the nesting.

pub const BLOCK: usize = 512; // 4096-bit squares, residue floor 0.78%
pub const P: u32 = 8219; // injectivity of +-2^k, k<4096, verified in v6
pub const Q: u32 = 8221;
pub const HDR: usize = 64;
pub const GMAX: usize = 128; // data points x = 0..G-1; parity x = GMAX + t

// ---------- bits ----------
#[inline]
fn get_bit(b: &[u8], i: u64) -> u8 {
    (b[(i >> 3) as usize] >> (7 - (i & 7))) & 1
}
#[inline]
fn set_bit(b: &mut [u8], i: u64, v: u8) {
    let m = 1u8 << (7 - (i & 7));
    if v != 0 {
        b[(i >> 3) as usize] |= m;
    } else {
        b[(i >> 3) as usize] &= !m;
    }
}

/// slot order at the square scale: GROUP STRIPING. Stored slot j holds the
/// r-th member of group (j mod nGroups), so any contiguous run of up to
/// nGroups slots touches each group AT MOST ONCE -- not probably, provably.
fn stripe_order(s: usize, g: usize, t: usize) -> Vec<u32> {
    let ng = if s == 0 { 0 } else { (s + g - 1) / g };
    let p = ng * t;
    let nsq = s + p;
    let mut groups: Vec<Vec<u32>> = vec![Vec::new(); ng.max(1)];
    for i in 0..s {
        groups[i / g].push(i as u32);
    }
    for pg in 0..ng {
        for pt in 0..t {
            groups[pg].push((s + pg * t + pt) as u32);
        }
    }
    let mut out = Vec::with_capacity(nsq);
    let maxlen = groups.iter().map(|m| m.len()).max().unwrap_or(0);
    for r in 0..maxlen {
        for grp in groups.iter() {
            if r < grp.len() {
                out.push(grp[r]);
            }
        }
    }
    debug_assert_eq!(out.len(), nsq);
    out
}

// ---------- residues, syndromes, hash ----------
fn residue(sq: &[u8], m: u32) -> u32 {
    let mut acc: u32 = 0;
    for i in 0..BLOCK {
        let b = if i < sq.len() { sq[i] as u32 } else { 0 };
        acc = (acc * 256 + b) % m;
    }
    acc
}
struct Syn {
    pow: [u32; 4096],
    map: Vec<i32>,
    m: u32,
}
fn syn(m: u32) -> Syn {
    let mut pow = [0u32; 4096];
    let mut x = 1u32;
    for w in 0..4096 {
        pow[4095 - w] = x;
        x = (x * 2) % m;
    }
    let mut map = vec![-1i32; m as usize];
    for i in 0..4096usize {
        for (d, neg) in [(pow[i] % m, 0i32), ((m - pow[i]) % m, 1i32)] {
            assert!(map[d as usize] == -1, "modulus {} not injective", m);
            map[d as usize] = ((i as i32) << 1) | neg;
        }
    }
    Syn { pow, map, m }
}
#[inline]
fn lookup(t: &Syn, s: u32) -> Option<(u16, i8)> {
    if s == 0 {
        return None;
    }
    let e = t.map[s as usize];
    if e < 0 { None } else { Some(((e >> 1) as u16, if e & 1 == 1 { -1 } else { 1 })) }
}
#[inline]
fn syndrome_of(t: &Syn, i: u16, d: i8) -> u32 {
    if d > 0 { t.pow[i as usize] } else { (t.m - t.pow[i as usize]) % t.m }
}
pub fn fnv64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// bit-scale repair of one square against its (trusted) residues.
fn bit_repair(sq: &mut [u8], rp: u32, rq: u32, sp_tab: &Syn, sq_tab: &Syn, doubles: bool) -> Option<usize> {
    let sp = (residue(sq, P) + P - rp) % P;
    let s2 = (residue(sq, Q) + Q - rq) % Q;
    if sp == 0 && s2 == 0 {
        return Some(0);
    }
    if let Some((i, d)) = lookup(sp_tab, sp) {
        let bit = i as u64;
        if (bit as usize) < sq.len() * 8
            && get_bit(sq, bit) == if d > 0 { 1 } else { 0 }
            && syndrome_of(sq_tab, i, d) == s2
        {
            set_bit(sq, bit, if d > 0 { 0 } else { 1 });
            return Some(1);
        }
    }
    if doubles {
        let mut sol: Option<(u16, u16)> = None;
        for i1 in 0..(sq.len() * 8).min(4096) as u16 {
            let d1: i8 = if get_bit(sq, i1 as u64) == 1 { 1 } else { -1 };
            let rem = (sp + P - syndrome_of(sp_tab, i1, d1)) % P;
            if rem == 0 {
                continue;
            }
            if let Some((i2, d2)) = lookup(sp_tab, rem) {
                if i2 == i1 || (i2 as usize) >= sq.len() * 8 {
                    continue;
                }
                if get_bit(sq, i2 as u64) != if d2 > 0 { 1 } else { 0 } {
                    continue;
                }
                if (syndrome_of(sq_tab, i1, d1) + syndrome_of(sq_tab, i2, d2)) % Q != s2 {
                    continue;
                }
                let key = (i1.min(i2), i1.max(i2));
                match sol {
                    Some(k) if k != key => return None, // ambiguous
                    _ => sol = Some(key),
                }
            }
        }
        if let Some((i1, i2)) = sol {
            for i in [i1, i2] {
                let v = get_bit(sq, i as u64);
                set_bit(sq, i as u64, 1 - v);
            }
            return Some(2);
        }
    }
    None
}

// ---------- geometry ----------
// armored layout: [hdr0 meta0][slots2: shielded CT][hdr1 meta1][slots1: shielded data][meta2 hdr2]
// no-armor (g==0): [hdr0][payload][hdr1][hdr2]
#[derive(Clone)]
pub struct Geom {
    pub len: usize,
    pub g: usize,
    pub t: usize,
    pub s: usize,    // level-1 data squares
    pub p: usize,    // level-1 parity squares
    pub nsq: usize,  // s + p: level-1 slots
    pub ct: usize,   // level-1 check bytes = 4 * nsq
    pub c: usize,    // level-2 data squares (the CT cut into BLOCK pieces)
    pub p2: usize,   // level-2 parity squares
    pub nsq2: usize, // c + p2: level-2 slots
    pub m: usize,    // level-2 check bytes = 4 * nsq2, stored in triplicate
    pub hash: u64,   // FNV-64 of the armored payload (localizes stage failure)
    pub ex: Extras,  // the conservation fields, v7's addition
    pub total: usize,
}
/// the conservation check rides in the voted header: what the ORIGINAL bytes
/// were (length + FNV-64), and which model transmuted them.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Extras {
    pub orig_len: u64,
    pub orig_fnv: u64,
    pub model: u8,
}
pub fn geom(len: usize, g: usize, t: usize, hash: u64, ex: Extras) -> Geom {
    if g == 0 {
        return Geom { len, g: 0, t: 0, s: 0, p: 0, nsq: 0, ct: 0, c: 0, p2: 0, nsq2: 0, m: 0,
                      hash, ex, total: 3 * HDR + len };
    }
    let s = (len + BLOCK - 1) / BLOCK;
    let p = if s == 0 { 0 } else { ((s + g - 1) / g) * t };
    let nsq = s + p;
    let ct = 4 * nsq;
    let c = (ct + BLOCK - 1) / BLOCK;
    let p2 = if c == 0 { 0 } else { ((c + g - 1) / g) * t };
    let nsq2 = c + p2;
    let m = 4 * nsq2;
    Geom { len, g, t, s, p, nsq, ct, c, p2, nsq2, m, hash, ex,
           total: 3 * HDR + 3 * m + nsq2 * BLOCK + nsq * BLOCK }
}

pub struct Off {
    pub h0: usize,
    pub m0: usize,
    pub slots2: usize,
    pub h1: usize,
    pub m1: usize,
    pub slots: usize,
    pub m2: usize,
    pub h2: usize,
}
pub fn offsets(g: &Geom) -> Off {
    if g.g == 0 {
        // [hdr0][payload][hdr1][hdr2]; slots names the payload region
        let h0 = 0;
        let slots = HDR;
        let h1 = slots + g.len;
        let h2 = h1 + HDR;
        return Off { h0, m0: HDR, slots2: HDR, h1, m1: h1 + HDR, slots, m2: h1, h2 };
    }
    let h0 = 0;
    let m0 = HDR;
    let slots2 = m0 + g.m;
    let h1 = slots2 + g.nsq2 * BLOCK;
    let m1 = h1 + HDR;
    let slots = m1 + g.m;
    let m2 = slots + g.nsq * BLOCK;
    let h2 = m2 + g.m;
    Off { h0, m0, slots2, h1, m1, slots, m2, h2 }
}

fn write_header(g: &Geom, out: &mut [u8]) {
    out[0..4].copy_from_slice(b"EGG7");
    out[4] = 1;
    out[5] = g.g as u8;
    out[6..8].copy_from_slice(&(P as u16).to_le_bytes());
    out[8..10].copy_from_slice(&(Q as u16).to_le_bytes());
    out[10..18].copy_from_slice(&(g.len as u64).to_le_bytes());
    out[18..22].copy_from_slice(&(g.s as u32).to_le_bytes());
    out[22..30].copy_from_slice(&g.hash.to_le_bytes());
    out[30] = g.t as u8;
    out[31] = g.ex.model;
    out[32..40].copy_from_slice(&g.ex.orig_len.to_le_bytes());
    out[40..48].copy_from_slice(&g.ex.orig_fnv.to_le_bytes());
}
fn parse_header(b: &[u8]) -> Option<Geom> {
    if b.len() < HDR || &b[0..4] != b"EGG7" || b[4] != 1 {
        return None;
    }
    let grp = b[5] as usize;
    let t = b[30] as usize;
    if grp == 0 {
        if t != 0 {
            return None;
        }
    } else if t == 0 || t > 8 || grp + t > GMAX {
        return None;
    }
    if u16::from_le_bytes([b[6], b[7]]) as u32 != P
        || u16::from_le_bytes([b[8], b[9]]) as u32 != Q
    {
        return None;
    }
    let len = u64::from_le_bytes(b[10..18].try_into().unwrap()) as usize;
    let s = u32::from_le_bytes(b[18..22].try_into().unwrap()) as usize;
    let hash = u64::from_le_bytes(b[22..30].try_into().unwrap());
    let ex = Extras {
        model: b[31],
        orig_len: u64::from_le_bytes(b[32..40].try_into().unwrap()),
        orig_fnv: u64::from_le_bytes(b[40..48].try_into().unwrap()),
    };
    let g = geom(len, grp, t, hash, ex);
    if g.s != s {
        return None; // the v5.0 audit's lesson: validate internal consistency
    }
    Some(g)
}

// ---------- GF(256) and Lagrange-systematic Reed-Solomon ----------
struct Gf {
    exp: [u8; 512],
    log: [u8; 256],
}
fn gf_new() -> Gf {
    let mut exp = [0u8; 512];
    let mut log = [0u8; 256];
    let mut x: u32 = 1;
    for i in 0..255 {
        exp[i] = x as u8;
        log[x as usize] = i as u8;
        x <<= 1;
        if x & 0x100 != 0 {
            x ^= 0x11d;
        }
    }
    for i in 255..512 {
        exp[i] = exp[i - 255];
    }
    Gf { exp, log }
}
impl Gf {
    #[inline]
    fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
    #[inline]
    fn div(&self, a: u8, b: u8) -> u8 {
        if a == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + 255 - self.log[b as usize] as usize]
        }
    }
    fn lagrange(&self, xs: &[u8], x: u8) -> Vec<u8> {
        let mut w = vec![0u8; xs.len()];
        for i in 0..xs.len() {
            let mut num = 1u8;
            let mut den = 1u8;
            for j in 0..xs.len() {
                if i == j {
                    continue;
                }
                num = self.mul(num, x ^ xs[j]);
                den = self.mul(den, xs[i] ^ xs[j]);
            }
            w[i] = self.div(num, den);
        }
        w
    }
}

// ---------- the shield, shared by both levels ----------
fn shield_squares(src: &[u8], s: usize, g: usize, t: usize, gf: &Gf) -> Vec<[u8; BLOCK]> {
    let ng = if s == 0 { 0 } else { (s + g - 1) / g };
    let p = ng * t;
    let mut sq = vec![[0u8; BLOCK]; s + p];
    for i in 0..s {
        sq[i] = square_of(src, i);
    }
    for pg in 0..ng {
        let lo = pg * g;
        let m = ((pg + 1) * g).min(s) - lo;
        let xs: Vec<u8> = (0..m as u8).collect();
        for pt in 0..t {
            let w = gf.lagrange(&xs, (GMAX + pt) as u8);
            let mut acc = [0u8; BLOCK];
            for (i, &wi) in w.iter().enumerate() {
                if wi == 0 {
                    continue;
                }
                for b in 0..BLOCK {
                    acc[b] ^= gf.mul(wi, sq[lo + i][b]);
                }
            }
            sq[s + pg * t + pt] = acc;
        }
    }
    sq
}
fn checks_of(sq: &[[u8; BLOCK]]) -> Vec<u8> {
    let mut ct = vec![0u8; 4 * sq.len()];
    for (i, s) in sq.iter().enumerate() {
        let rp = residue(s, P);
        let rq = residue(s, Q);
        ct[4 * i..4 * i + 2].copy_from_slice(&(rp as u16).to_le_bytes());
        ct[4 * i + 2..4 * i + 4].copy_from_slice(&(rq as u16).to_le_bytes());
    }
    ct
}
fn interleave(sq: &[[u8; BLOCK]], s: usize, g: usize, t: usize, out: &mut [u8]) {
    for (j, &orig) in stripe_order(s, g, t).iter().enumerate() {
        out[j * BLOCK..(j + 1) * BLOCK].copy_from_slice(&sq[orig as usize]);
    }
}
/// slots region -> repaired squares. Bit scale first (residues), then square
/// scale (RS parity); wounded slots are marked dead before anyone believes
/// their bytes. One routine, used for the data and for its own check table.
#[allow(clippy::too_many_arguments)]
fn recover(
    region: &[u8],
    region_off: usize,
    wounds: &[(usize, usize)],
    checks: &dyn Fn(usize) -> (u32, u32),
    s: usize,
    g: usize,
    tpar: usize,
    gf: &Gf,
    sp_tab: &Syn,
    sq_tab: &Syn,
    mode: Repair,
    t: &mut Tally,
) -> Vec<[u8; BLOCK]> {
    let ng = if s == 0 { 0 } else { (s + g - 1) / g };
    let p = ng * tpar;
    let nsq = s + p;
    let order = stripe_order(s, g, tpar);
    let mut sq = vec![[0u8; BLOCK]; nsq];
    let mut deadslot = vec![false; nsq];
    let in_wound = |lo: usize, hi: usize| wounds.iter().any(|&(a, l)| a < hi && a + l > lo);
    for (j, &orig) in order.iter().enumerate() {
        let a = j * BLOCK;
        sq[orig as usize].copy_from_slice(&region[a..a + BLOCK]);
        if in_wound(region_off + a, region_off + a + BLOCK) {
            deadslot[orig as usize] = true;
        }
    }
    let mut bad = vec![false; nsq];
    for i in 0..nsq {
        let (rp, rq) = checks(i);
        if deadslot[i] {
            let ok = residue(&sq[i], P) == rp && residue(&sq[i], Q) == rq;
            if ok { t.clean += 1 } else { bad[i] = true }
            continue;
        }
        if mode == Repair::ParityOnly {
            // trust nothing repaired: random wound-fill can fake a 1-2 bit fix
            // (~2.4e-4 per square) and poison the rebuilds; the parity path
            // verifies every rebuilt square against residues, which random
            // content cannot fake (~2.4e-7)
            let ok = residue(&sq[i], P) == rp && residue(&sq[i], Q) == rq;
            if ok { t.clean += 1 } else { bad[i] = true }
        } else {
            match bit_repair(&mut sq[i], rp, rq, sp_tab, sq_tab, mode == Repair::Full) {
                Some(0) => t.clean += 1,
                Some(1) => t.bitfixed += 1,
                Some(_) => t.bitfixed2 += 1,
                None => bad[i] = true,
            }
        }
    }
    for pg in 0..ng {
        let lo = pg * g;
        let m = ((pg + 1) * g).min(s) - lo;
        let members: Vec<(u8, usize)> = (0..m)
            .map(|i| (i as u8, lo + i))
            .chain((0..tpar).map(|pt| ((GMAX + pt) as u8, s + pg * tpar + pt)))
            .collect();
        let bads: Vec<usize> = (0..members.len()).filter(|&k| bad[members[k].1]).collect();
        if bads.is_empty() {
            continue;
        }
        if bads.len() > tpar {
            t.detected += bads.len();
            continue;
        }
        let survivors: Vec<usize> = (0..members.len()).filter(|&k| !bad[members[k].1]).collect();
        let base: Vec<usize> = survivors.iter().copied().take(m).collect();
        let xs: Vec<u8> = base.iter().map(|&k| members[k].0).collect();
        let mut ok_all = true;
        let mut rebuilt: Vec<(usize, [u8; BLOCK])> = Vec::new();
        for &k in &bads {
            let (x, idx) = members[k];
            let w = gf.lagrange(&xs, x);
            let mut acc = [0u8; BLOCK];
            for (wi, &bk) in w.iter().zip(base.iter()) {
                if *wi == 0 {
                    continue;
                }
                let src = &sq[members[bk].1];
                for b in 0..BLOCK {
                    acc[b] ^= gf.mul(*wi, src[b]);
                }
            }
            let (rp, rq) = checks(idx);
            if residue(&acc, P) == rp && residue(&acc, Q) == rq {
                rebuilt.push((idx, acc));
            } else {
                ok_all = false;
            }
        }
        if ok_all {
            for (idx, acc) in rebuilt {
                sq[idx] = acc;
                bad[idx] = false;
                t.rebuilt += 1;
            }
        } else {
            t.detected += bads.len();
        }
    }
    sq
}

// ---------- armor (encode) ----------
fn square_of(src: &[u8], i: usize) -> [u8; BLOCK] {
    let mut b = [0u8; BLOCK];
    let base = i * BLOCK;
    if base < src.len() {
        let n = (src.len() - base).min(BLOCK);
        b[..n].copy_from_slice(&src[base..base + n]);
    }
    b
}

pub fn armor(inner: &[u8], grp: usize, tpar: usize, ex: Extras) -> Vec<u8> {
    let g = geom(inner.len(), grp, if grp == 0 { 0 } else { tpar }, fnv64(inner), ex);
    let off = offsets(&g);
    let mut out = vec![0u8; g.total];
    if grp == 0 {
        for o in [off.h0, off.h1, off.h2] {
            write_header(&g, &mut out[o..o + HDR]);
        }
        out[off.slots..off.slots + g.len].copy_from_slice(inner);
        return out;
    }
    let gf = gf_new();
    let sq1 = shield_squares(inner, g.s, g.g, g.t, &gf);
    let ct1 = checks_of(&sq1);
    let sq2 = shield_squares(&ct1, g.c, g.g, g.t, &gf);
    let meta = checks_of(&sq2);
    for o in [off.h0, off.h1, off.h2] {
        write_header(&g, &mut out[o..o + HDR]);
    }
    for o in [off.m0, off.m1, off.m2] {
        out[o..o + g.m].copy_from_slice(&meta);
    }
    interleave(&sq2, g.c, g.g, g.t, &mut out[off.slots2..off.slots2 + g.nsq2 * BLOCK]);
    interleave(&sq1, g.s, g.g, g.t, &mut out[off.slots..off.slots + g.nsq * BLOCK]);
    out
}

// ---------- dearmor (decode) ----------
#[derive(Clone, Copy, PartialEq)]
enum Repair {
    Full,       // singles + doubles
    NoDoubles,  // singles only
    ParityOnly, // trust no bit fix; residues judge, parity rebuilds
}
pub struct Tally {
    pub clean: usize,
    pub bitfixed: usize,
    pub bitfixed2: usize,
    pub rebuilt: usize,
    pub detected: usize,
}
pub struct DearmorOut {
    pub inner: Vec<u8>,
    pub ex: Extras,
    pub t: Tally,
    pub ct_report: String,
    pub hash_ok: bool, // FNV of the armored payload matched (stage-local truth)
    pub retried: bool,
    pub padded: usize,
}

pub fn dearmor(cont_in: &[u8], wounds_in: &[(usize, usize)], doubles: bool) -> Result<DearmorOut, String> {
    let g = parse_header(cont_in)
        .or_else(|| {
            if cont_in.len() >= HDR {
                parse_header(&cont_in[cont_in.len() - HDR..])
            } else {
                None
            }
        })
        .ok_or("no valid header at either end")?;
    if cont_in.len() > g.total {
        return Err("container longer than its own geometry".into());
    }
    // truncation is not special: pad back to size, call the missing tail a wound
    let mut wounds: Vec<(usize, usize)> = wounds_in.to_vec();
    let padded = g.total - cont_in.len();
    let mut cont = cont_in.to_vec();
    if padded > 0 {
        cont.resize(g.total, 0);
        wounds.push((cont_in.len(), padded));
    }
    let off = offsets(&g);

    let vote3 = |a: usize, b: usize, c: usize, n: usize| -> Vec<u8> {
        (0..n)
            .map(|i| {
                let (x, y, z) = (cont[a + i], cont[b + i], cont[c + i]);
                if x == y || x == z { x } else if y == z { y } else { x }
            })
            .collect()
    };
    // vote first; if the vote itself is unparseable (e.g. two headers gone the
    // same way), fall back to each header alone before giving up
    let g = parse_header(&vote3(off.h0, off.h1, off.h2, HDR))
        .or_else(|| parse_header(&cont[off.h0..off.h0 + HDR]))
        .or_else(|| parse_header(&cont[off.h1..(off.h1 + HDR).min(cont.len())]))
        .or_else(|| parse_header(&cont[off.h2..(off.h2 + HDR).min(cont.len())]))
        .ok_or("headers unrecoverable after vote")?;

    if g.g == 0 {
        // no armor: checks + hash only, honestly incapable of repair
        let inner = cont[off.slots..off.slots + g.len].to_vec();
        let hash_ok = fnv64(&inner) == g.hash;
        return Ok(DearmorOut {
            inner,
            ex: g.ex,
            t: Tally { clean: 0, bitfixed: 0, bitfixed2: 0, rebuilt: 0, detected: 0 },
            ct_report: "no armor (g=0): headers + hash only".into(),
            hash_ok,
            retried: false,
            padded,
        });
    }

    let meta = vote3(off.m0, off.m1, off.m2, g.m);
    let meta_checks = move |i: usize| -> (u32, u32) {
        let o = 4 * i;
        let rp = u16::from_le_bytes([meta[o], meta[o + 1]]) as u32;
        let rq = u16::from_le_bytes([meta[o + 2], meta[o + 3]]) as u32;
        (rp, rq)
    };

    let sp_tab = syn(P);
    let sq_tab = syn(Q);
    let gf = gf_new();

    let run = |mode: Repair| -> (Vec<u8>, Tally, String) {
        let mut t = Tally { clean: 0, bitfixed: 0, bitfixed2: 0, rebuilt: 0, detected: 0 };
        let mut t2 = Tally { clean: 0, bitfixed: 0, bitfixed2: 0, rebuilt: 0, detected: 0 };
        let sq2 = recover(
            &cont[off.slots2..off.slots2 + g.nsq2 * BLOCK],
            off.slots2, &wounds, &meta_checks, g.c, g.g, g.t, &gf, &sp_tab, &sq_tab, mode, &mut t2,
        );
        let mut ct = vec![0u8; g.ct];
        for i in 0..g.c {
            let base = i * BLOCK;
            let n = (g.ct - base).min(BLOCK);
            ct[base..base + n].copy_from_slice(&sq2[i][..n]);
        }
        let checks = move |i: usize| -> (u32, u32) {
            let o = 4 * i;
            let rp = u16::from_le_bytes([ct[o], ct[o + 1]]) as u32;
            let rq = u16::from_le_bytes([ct[o + 2], ct[o + 3]]) as u32;
            (rp, rq)
        };
        let sq1 = recover(
            &cont[off.slots..off.slots + g.nsq * BLOCK],
            off.slots, &wounds, &checks, g.s, g.g, g.t, &gf, &sp_tab, &sq_tab, mode, &mut t,
        );
        let mut data = vec![0u8; g.len];
        for i in 0..g.s {
            let base = i * BLOCK;
            let n = (g.len - base).min(BLOCK);
            data[base..base + n].copy_from_slice(&sq1[i][..n]);
        }
        let ct_report = format!(
            "{} clean, {} fixed, {} rebuilt, {} bad",
            t2.clean, t2.bitfixed + t2.bitfixed2, t2.rebuilt, t2.detected
        );
        (data, t, ct_report)
    };

    let ladder: &[Repair] = if doubles {
        &[Repair::Full, Repair::NoDoubles, Repair::ParityOnly]
    } else {
        &[Repair::NoDoubles, Repair::ParityOnly]
    };
    let mut best: Option<DearmorOut> = None;
    for (rung, &mode) in ladder.iter().enumerate() {
        let (data, t, ct_report) = run(mode);
        let hash_ok = fnv64(&data) == g.hash;
        let out = DearmorOut { inner: data, ex: g.ex, t, ct_report, hash_ok, retried: rung > 0, padded };
        if hash_ok {
            return Ok(out);
        }
        if best.is_none() {
            best = Some(out);
        }
    }
    Ok(best.unwrap())
}
