//! eggv5 v4 (codegg v6) -- the Wub reading.
//!
//! The last unmined page of the site is Wub: phasors summed tip to tail, one
//! closed curve from many rotating parts. Read as coding theory, that is
//! EVALUATION: a group of squares as points of a polynomial, redundancy as
//! extra evaluations of the same polynomial -- which is Reed-Solomon, the
//! industrial tool this series has been benchmarking against all along. The
//! site's Inspirations page says the rooms were furnished when we got there;
//! this is the final room. So v6 replaces the single XOR parity square with
//! T Reed-Solomon parity squares per group over GF(256): at G=32, T=2 the
//! cost stays ~9% and any TWO dead squares per group are rebuilt -- killing
//! v5.2's one documented caveat (two wounds a stripe-length apart sharing a
//! group). T=1 over GF(256) degenerates to something XOR-equivalent; T is on
//! the label (--parity).
//!
//! ---- the v5.2 story, kept because each rung was bought with a failure ----
//!
//! eggv5 v3 (codegg v5.2) -- protection with shape.
//!
//! The 256 MiB drills convicted v5.1 (container v2) on three counts:
//!   - a 16 MiB wound was hopeless: 64 erasures/square against a cap of 20,
//!     because ALL protection lived at one scale, the bit inside a square;
//!   - a 1 MiB truncation was refused outright while a raw file kept 99.6%;
//!   - the bit-granularity Atlas walk ran at ~5 MB/s -- two billion random
//!     single-bit accesses across a 256 MB buffer, pure cache death.
//!
//! The site's landing page says NUMBERS HAVE A SHAPE, and its Spectrometer
//! shows one number as nested readings -- stalk, square, regions -- one thing
//! at every scale at once. v3 protects the same way, at three nested scales:
//!
//!   BIT SCALE     per-square residues, V mod 2053 / 2063 (codegg-v1's move,
//!                 kept): repair 1-2 flipped bits locally, and -- crucially --
//!                 SELF-DIAGNOSE: a bad square announces itself (miss
//!                 probability ~2.4e-7), so nothing above needs a map.
//!   SQUARE SCALE  group parity (new): every G=16 squares fold into one
//!                 parity square, cell-wise XOR -- the site's oldest move,
//!                 the sum of the rack, one level up. Any ONE dead square per
//!                 group is rebuilt from its siblings, blind.
//!   FILE SCALE    three headers at far-apart offsets, byte-voted; FNV-64
//!                 whole-file hash that catches lying repairs (kept from v2).
//!
//! And the interleave moves from bit to SQUARE granularity: stored slot j
//! holds original square bitrev(j), a 128-byte memcpy instead of a single
//! bit. A contiguous wound now scatters WHOLE squares across DIFFERENT
//! groups (at most ~one per group up to (S+P)/G slots), where the parity
//! rebuilds them -- and the encoder stops fighting the cache.
//!
//! Truncation stops being special: if the front header is valid but the file
//! is short, the container is padded back to its expected size and the
//! missing tail becomes an ordinary wound. The parity then rebuilds what the
//! suffix took, up to capacity.
//!
//! Costs on the label: 2.35% residues + 1/G parity (6.25% at G=16) + meta and
//! headers ~= 9%. Capacity on the label too: about one square per group --
//! (S+P)/G slots ~= file/G bytes of contiguous damage, blind. Beyond it:
//! detected, never silent. Still not compression, still not novelty (RAID
//! did square-scale parity first; Avizienis 1971 did the residues), and still
//! not tamper-proof (residues and FNV stop accidents, not adversaries).

use std::env;
use std::fs;
use std::process::ExitCode;
use std::time::Instant;

const BLOCK: usize = 512; // v6 slim: 4096-bit squares, residue floor 3x lower
const P: u32 = 8219; // injectivity of +-2^k, k<4096, verified by enumeration
const Q: u32 = 8221;
const HDR: usize = 40;
const DEFAULT_G: usize = 32;

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
#[inline]
fn bitrev(i: u64, k: u32) -> u64 {
    i.reverse_bits() >> (64 - k)
}
fn k_for(n: u64) -> u32 {
    let mut k = 1;
    while (1u64 << k) < n {
        k += 1;
    }
    k
}

/// slot order at the square scale: GROUP STRIPING. Stored slot j holds the
/// r-th member of group (j mod nGroups), so any contiguous run of up to
/// nGroups slots touches each group AT MOST ONCE -- not probably, provably.
/// (The first draft used the Atlas/van der Corput order here; measured on a
/// 1 MiB tail truncation it collided 810 pairs into shared groups. The vdC
/// order is a low-discrepancy promise; a stripe is a pigeonhole guarantee,
/// and for the contiguous-wound model the guarantee wins. Multiple wounds
/// spaced near a multiple of the stripe length can still share groups --
/// detected honestly when they do.)
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

#[allow(dead_code)]
fn atlas_bits(src: &[u8]) -> Vec<u8> {
    let nbits = (src.len() as u64) * 8;
    let mut out = vec![0u8; src.len()];
    if nbits <= 1 {
        out.copy_from_slice(src);
        return out;
    }
    let k = k_for(nbits);
    let mut j: u64 = 0;
    for i in 0..(1u64 << k) {
        let p = bitrev(i, k);
        if p < nbits {
            set_bit(&mut out, j, get_bit(src, p));
            j += 1;
        }
    }
    out
}
#[allow(dead_code)]
fn un_atlas_bits(src: &[u8]) -> Vec<u8> {
    let nbits = (src.len() as u64) * 8;
    let mut out = vec![0u8; src.len()];
    if nbits <= 1 {
        out.copy_from_slice(src);
        return out;
    }
    let k = k_for(nbits);
    let mut j: u64 = 0;
    for i in 0..(1u64 << k) {
        let p = bitrev(i, k);
        if p < nbits {
            set_bit(&mut out, p, get_bit(src, j));
            j += 1;
        }
    }
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
fn fnv64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// bit-scale repair of one square against its (trusted) residues.
/// Returns cells fixed (0 = clean), or None when the square is beyond the bit
/// scale -- at which point it becomes the square scale's problem.
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
// layout: [hdr0][meta0][CT~ bit-permuted][hdr1][meta1][slots (S+P)*128][meta2][hdr2]
#[derive(Clone)]
struct Geom {
    len: usize,
    g: usize,
    t: usize,
    s: usize,      // level-1 data squares
    p: usize,      // level-1 parity squares
    nsq: usize,    // s + p: level-1 slots
    ct: usize,     // level-1 check bytes = 3 * nsq
    c: usize,      // level-2 data squares (the CT cut into 128 B pieces)
    p2: usize,     // level-2 parity squares
    nsq2: usize,   // c + p2: level-2 slots
    m: usize,      // level-2 check bytes = 3 * nsq2, stored in triplicate
    hash: u64,
    total: usize,
}
fn geom(len: usize, g: usize, t: usize, hash: u64) -> Geom {
    let s = (len + BLOCK - 1) / BLOCK;
    let p = if s == 0 { 0 } else { ((s + g - 1) / g) * t };
    let nsq = s + p;
    let ct = 4 * nsq;
    let c = (ct + BLOCK - 1) / BLOCK;
    let p2 = if c == 0 { 0 } else { ((c + g - 1) / g) * t };
    let nsq2 = c + p2;
    let m = 4 * nsq2;
    Geom { len, g, t, s, p, nsq, ct, c, p2, nsq2, m, hash,
           total: 3 * HDR + 3 * m + nsq2 * BLOCK + nsq * BLOCK }
}

// layout: [hdr0 meta0][slots2: shielded CT][hdr1 meta1][slots1: shielded data][meta2 hdr2]
struct Off {
    h0: usize,
    m0: usize,
    slots2: usize,
    h1: usize,
    m1: usize,
    slots: usize,
    m2: usize,
    h2: usize,
}
fn offsets(g: &Geom) -> Off {
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
    out[0..4].copy_from_slice(b"EGG5");
    out[4] = 5;
    out[5] = g.g as u8;
    out[30] = g.t as u8;
    out[6..8].copy_from_slice(&(P as u16).to_le_bytes());
    out[8..10].copy_from_slice(&(Q as u16).to_le_bytes());
    out[10..18].copy_from_slice(&(g.len as u64).to_le_bytes());
    out[18..22].copy_from_slice(&(g.s as u32).to_le_bytes());
    out[22..30].copy_from_slice(&g.hash.to_le_bytes());
}
fn parse_header(b: &[u8]) -> Option<Geom> {
    if b.len() < HDR || &b[0..4] != b"EGG5" || b[4] != 5 || b[5] == 0 {
        return None;
    }
    let t = b[30] as usize;
    if t == 0 || t > 8 || (b[5] as usize) + t > GMAX {
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
    let g = geom(len, b[5] as usize, t, hash);
    if g.s != s {
        return None; // the audit's lesson: internal consistency is validated
    }
    Some(g)
}

// ---------- GF(256) and Lagrange-systematic Reed-Solomon ----------
// polynomial 0x11d, generator 2. Parity square t of a group is the group's
// interpolating polynomial (through (i, data_i)) evaluated at x = GMAX + t.
// Any (members - dead) >= m survivors re-interpolate; residues name the dead.
const GMAX: usize = 128; // data points use x = 0..G-1; parity x = GMAX + t

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
    /// Lagrange weights: value at x from points xs -> w_i with f(x)=sum w_i*f(xs_i)
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
/// data -> squares + T Reed-Solomon parity squares per group (the Wub
/// reading: the group is a polynomial, the parities are extra evaluations)
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
/// scale (group parity); wounded slots are marked dead before anyone believes
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
        // members: (x point, square index) -- data at x=i, parity at x=GMAX+t
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
        // any m survivors re-interpolate the group's polynomial (the Wub
        // reading: enough phasors recover the whole curve)
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

// ---------- encode ----------
fn square_of<'a>(src: &'a [u8], i: usize) -> [u8; BLOCK] {
    let mut b = [0u8; BLOCK];
    let base = i * BLOCK;
    if base < src.len() {
        let n = (src.len() - base).min(BLOCK);
        b[..n].copy_from_slice(&src[base..base + n]);
    }
    b
}

fn encode(src: &[u8], grp: usize, t: usize) -> Vec<u8> {
    let gf = gf_new();
    let g = geom(src.len(), grp, t, fnv64(src));
    let off = offsets(&g);
    let mut out = vec![0u8; g.total];

    // level 1: the data, shielded
    let sq1 = shield_squares(src, g.s, g.g, g.t, &gf);
    let ct1 = checks_of(&sq1);
    // level 2: the check table itself, shielded the same way -- the shape,
    // one level down; its checks (the meta) are small and ride in triplicate
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

// ---------- decode ----------
#[derive(Clone, Copy, PartialEq)]
enum Repair {
    Full,       // singles + doubles
    NoDoubles,  // singles only
    ParityOnly, // trust no bit fix; residues judge, parity rebuilds
}
struct Tally {
    clean: usize,
    bitfixed: usize,
    bitfixed2: usize,
    rebuilt: usize,
    detected: usize,
}
struct DecodeOut {
    data: Vec<u8>,
    t: Tally,
    ct_report: String,
    hash_ok: bool,
    retried: bool,
    padded: usize,
}

fn decode(cont_in: &[u8], wounds_in: &[(usize, usize)], doubles: bool) -> Result<DecodeOut, String> {
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
    let hdr = vote3(off.h0, off.h1, off.h2, HDR);
    let g = parse_header(&hdr).ok_or("headers unrecoverable after vote")?;
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
        // level 2: recover the check table from its own shielded slots
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
        // level 1: recover the data with the recovered checks
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
    let mut best: Option<DecodeOut> = None;
    for (rung, &mode) in ladder.iter().enumerate() {
        let (data, t, ct_report) = run(mode);
        let hash_ok = fnv64(&data) == g.hash;
        let out = DecodeOut { data, t, ct_report, hash_ok, retried: rung > 0, padded };
        if hash_ok {
            return Ok(out);
        }
        if best.is_none() {
            best = Some(out);
        }
    }
    Ok(best.unwrap())
}

// ---------- CLI ----------
fn xorshift(state: &mut u64) -> u8 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state & 0xff) as u8
}
fn report(label: &str, o: &DecodeOut, ms: u128) {
    println!(
        "  {:26} {} clean, {} bit-fixed, {} low-confidence, {} REBUILT from parity, {} detected;\n  {:26} checks: {}; {}hash {}{} [{} ms]",
        label,
        o.t.clean,
        o.t.bitfixed,
        o.t.bitfixed2,
        o.t.rebuilt,
        o.t.detected,
        "",
        o.ct_report,
        if o.padded > 0 { format!("truncated by {} B, treated as a wound; ", o.padded) } else { String::new() },
        if o.hash_ok { "OK" } else { "MISMATCH" },
        if o.retried { " (retried without doubles)" } else { "" },
        ms
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let get = |name: &str| -> Option<String> {
        args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    let bare: Vec<&String> = {
        let mut out = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if args[i] == "--no-doubles" {
                i += 1;
            } else if args[i].starts_with("--") || args[i] == "-o" {
                i += 2;
            } else {
                out.push(&args[i]);
                i += 1;
            }
        }
        out
    };
    let usage = || {
        eprintln!("usage: eggv5 encode <file> [-o out] [--group 32] [--parity 2]");
        eprintln!("       eggv5 decode <file.egg5> [-o out] [--wound start:len] [--no-doubles]");
        eprintln!("       eggv5 scratch <file> [--len 4096] [--at payload|checks|head|end|<off>] [--group 16]");
        ExitCode::from(2)
    };
    if bare.len() < 2 {
        return usage();
    }
    let cmd = bare[0].as_str();
    let path = bare[1];
    let grp: usize = get("--group").map(|s| s.parse().unwrap()).unwrap_or(DEFAULT_G);
    let tpar: usize = get("--parity").map(|s| s.parse().unwrap()).unwrap_or(2);

    match cmd {
        "encode" => {
            let src = fs::read(path).expect("read input");
            let t0 = Instant::now();
            let out = encode(&src, grp, tpar);
            let dst = get("-o").unwrap_or(format!("{}.egg5", path));
            fs::write(&dst, &out).expect("write output");
            let ms = t0.elapsed().as_millis().max(1);
            println!(
                "{}: {} B -> {} B ({:.2}% of input) in {} ms ({} MB/s)",
                path,
                src.len(),
                out.len(),
                100.0 * out.len() as f64 / src.len().max(1) as f64,
                ms,
                src.len() as u128 / ms / 1000
            );
            println!(
                "  bit scale: residues; square scale: {} RS parities per {} squares; file scale: 3 voted headers + hash",
                tpar, grp
            );
            println!("  wrote {}", dst);
            ExitCode::SUCCESS
        }
        "decode" => {
            let cont = fs::read(path).expect("read container");
            let mut wounds = Vec::new();
            let mut i = 0;
            while i < args.len() {
                if args[i] == "--wound" {
                    let (a, b) = args[i + 1].split_once(':').expect("--wound start:len");
                    wounds.push((a.parse().unwrap(), b.parse().unwrap()));
                    i += 1;
                }
                i += 1;
            }
            let t0 = Instant::now();
            match decode(&cont, &wounds, !has("--no-doubles")) {
                Ok(o) => {
                    let dst = get("-o").unwrap_or_else(|| {
                        path.strip_suffix(".egg5").unwrap_or(path).to_string() + ".out"
                    });
                    fs::write(&dst, &o.data).expect("write output");
                    report(path, &o, t0.elapsed().as_millis());
                    println!("  wrote {}", dst);
                    if o.hash_ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
                }
                Err(e) => {
                    eprintln!("refused: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        "scratch" => {
            let src = fs::read(path).expect("read input");
            let len: usize = get("--len").map(|s| s.parse().unwrap()).unwrap_or(4096);
            let cont = encode(&src, grp, tpar);
            let g = geom(src.len(), grp, tpar, 0);
            let off = offsets(&g);
            let at: usize = match get("--at").as_deref() {
                None | Some("payload") => off.slots + (g.nsq * BLOCK).saturating_sub(len) / 2,
                Some("checks") => off.slots2 + (g.nsq2 * BLOCK).saturating_sub(len) / 2,
                Some("head") => 0,
                Some("end") => cont.len().saturating_sub(len),
                Some(x) => x.parse().expect("--at offset"),
            };
            let mut hurt = cont.clone();
            let mut st = 0x1489u64;
            for i in at..(at + len).min(hurt.len()) {
                hurt[i] = xorshift(&mut st);
            }
            println!(
                "{}: {} B, group {} parity {}, one contiguous {} B scratch at offset {} ({})",
                path,
                src.len(),
                grp, tpar,
                len,
                at,
                if at >= off.slots { "slots" } else if at >= off.slots2 { "check table" } else { "head" }
            );
            let mut all_ok = true;
            for (label, wounds) in [
                ("blind (location unknown)", vec![]),
                ("wound location known", vec![(at, len)]),
            ] {
                let t0 = Instant::now();
                match decode(&hurt, &wounds, true) {
                    Ok(o) => {
                        let exact = o.data == src && o.hash_ok;
                        report(label, &o, t0.elapsed().as_millis());
                        println!(
                            "  {:26} -> {}",
                            "",
                            if exact { "EXACT (hash-verified)" } else { "NOT recovered (reported, not silent)" }
                        );
                        if !exact {
                            all_ok = false;
                        }
                    }
                    Err(e) => {
                        println!("  {:26} refused: {}", label, e);
                        all_ok = false;
                    }
                }
            }
            if all_ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        _ => usage(),
    }
}
