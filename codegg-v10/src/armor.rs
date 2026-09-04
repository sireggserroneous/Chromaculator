//! armor.rs -- eggv8 armor v2: the same three nested scales as eggv6/v7
//! (bit-scale residues -> square-scale RS groups -> file-scale voted
//! replicas), with two geometry defects found in v7's design FIXED here and
//! PROVEN by `eggv8 audit`:
//!
//!   1. THE RAGGED-TAIL PIGEONHOLE. stripe_order leaves one short group when
//!      s % G != 0; in the tail rows only the full groups appear, so a
//!      contiguous run of k slots can hit one group ceil(k/(ng-1)) times.
//!      v7's fixed rib ladder never checked this. v2's rib policy is an
//!      argmin over (G, T) subject to ceil(9/ng_eff) <= T, where ng_eff is
//!      the number of maximum-length groups in the merged stripe and 9 is
//!      the most slots a 4096-byte scratch can straddle. The naive
//!      continuous policy G = ceil(s/5) FAILS this (s=59: ceil(9/4) = 3 > 2)
//!      and the audit keeps it failing as a negative control.
//!   2. CLUSTERED REPLICAS. v7 packed hdr0, meta0, the whole check table,
//!      hdr1 and meta1 into the first ~2 KB of small artifacts; one 4 KB
//!      head scratch killed two of three copies and vote3 returned garbage.
//!      v2 spreads the three [header meta] sites to head / middle (slot
//!      aligned) / end, appends an FNV-32 to every header and meta copy, and
//!      selects any checksum-verified copy; two verified copies that
//!      DISAGREE cause a refusal (never choose); byte-vote is the last
//!      resort only.
//!
//! The check table also moves: when ct <= 1024 B the CT itself IS the meta
//! (stored triplicate at the three sites, level-2 RS dropped); larger check
//! tables keep level-2 RS but their slots are interleaved among the payload
//! rows -- ONE merged stripe, both levels, the same pigeonhole, audited.
//!
//! The physics floor, printed and never papered over: the 4 KB-scratch
//! guarantee needs parity for >= 9 dead slots (9 slots at T=3, 10-12 at
//! T=2), i.e. ~4.6-6.1 KB of ribs regardless of payload size. Below ~16-24
//! KB of artifact that guarantee cannot be cheap, and below ~6 KB of inner
//! payload (s <= 11) it cannot be given at all -- rib_policy says so.
//!
//! Attribution: residues Avizienis 1971 / Mandelbaum 1976; Reed-Solomon 1960
//! via Lagrange evaluation; FNV hashes by Fowler-Noll-Vo. The site supplied
//! the geometry and the nesting.

use std::sync::OnceLock;

pub const BLOCK: usize = 512; // 4096-bit squares, residue floor 0.78%
pub const P: u32 = 8219; // injectivity of +-2^k, k<4096, audited every run
pub const Q: u32 = 8221;
pub const HDR: usize = 64;
pub const GMAX: usize = 128; // data points x = 0..G-1; parity x = GMAX + t
pub const CT_TRIPLE_MAX: usize = 1024; // ct <= this: triplicate CT, no level-2
pub const DEAD9: usize = 9; // most slots a 4096-byte scratch can straddle
const CAP_PCT: usize = 35; // rib overhead cap for the argmin (relaxed if needed)

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

// ---------- hashes ----------
pub fn fnv64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
pub fn fnv32(data: &[u8]) -> u32 {
    let mut h: u32 = 0x811c9dc5;
    for &b in data {
        h ^= b as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h
}

// ---------- the rib policy: argmin, not formula ----------
/// how many groups still appear in the sparsest stripe rows of the MERGED
/// order (both levels): the number of maximum-length groups. Group lengths
/// are G+T for full groups and n%G+T for the one short group per level, so
/// when any full group exists the maxima are exactly the full groups.
fn level_shape(n: usize, g: usize, t: usize) -> (usize, usize) {
    // (full-length group count, short group length or 0)
    if n == 0 {
        return (0, 0);
    }
    let ng = (n + g - 1) / g;
    if n % g == 0 {
        (ng, 0)
    } else {
        (ng - 1, n % g + t)
    }
}
fn ng_eff_of(s: usize, c: usize, g: usize, t: usize) -> usize {
    let (f1, l1) = level_shape(s, g, t);
    let (f2, l2) = level_shape(c, g, t);
    if f1 + f2 > 0 {
        return f1 + f2;
    }
    // no full group: at most one (short) group per level
    if l1 == 0 && l2 == 0 {
        0
    } else if l1 == l2 {
        2
    } else {
        1
    }
}
/// c (level-2 data squares) for a given s, following geom()'s own rule
fn c_of(s: usize, g: usize, t: usize) -> usize {
    if s == 0 {
        return 0;
    }
    let ng1 = (s + g - 1) / g;
    let ct = 4 * (s + ng1 * t);
    if ct <= CT_TRIPLE_MAX {
        0
    } else {
        (ct + BLOCK - 1) / BLOCK
    }
}
/// does (G, T) guarantee that any contiguous 4 KB scratch (<= 9 dead slots)
/// leaves every group with <= T dead members? The claim the audit proves.
pub fn guaranteed_st(s: usize, g: usize, t: usize) -> bool {
    if s == 0 || g == 0 {
        return false;
    }
    let ng_eff = ng_eff_of(s, c_of(s, g, t), g, t);
    ng_eff > 0 && (DEAD9 + ng_eff - 1) / ng_eff <= t
}

pub struct Rib {
    pub g: usize,
    pub t: usize,
    pub guaranteed: bool,
}
/// argmin container total over G in 4..=126, T in {2,3}, subject to the
/// 4 KB-scratch pigeonhole, under a 35% overhead cap; the cap is relaxed
/// (guarantee kept) when nothing fits it, and only when NO (G,T) can give
/// the guarantee (s <= 11: fewer than 3 usable groups) does the policy fall
/// back to densest parity with the guarantee honestly absent.
pub fn rib_policy(inner_len: usize) -> Rib {
    let dummy = Extras { orig_len: 0, orig_fnv: 0, model: 0, filter_id: 0, filter_param: 0 };
    let s = (inner_len + BLOCK - 1) / BLOCK;
    // (total, t, g); better = smaller total, then smaller t, then larger g
    let better = |cur: &Option<(usize, usize, usize)>, cand: (usize, usize, usize)| match cur {
        None => true,
        Some((bt, btt, bg)) => {
            cand.0 < *bt || (cand.0 == *bt && (cand.1 < *btt || (cand.1 == *btt && cand.2 > *bg)))
        }
    };
    let mut capped: Option<(usize, usize, usize)> = None;
    let mut open: Option<(usize, usize, usize)> = None;
    for g in 4..=(GMAX - 2) {
        for t in [2usize, 3] {
            if g + t > GMAX || !guaranteed_st(s, g, t) {
                continue;
            }
            let total = geom(inner_len, g, t, 0, dummy).total;
            let cand = (total, t, g);
            if (total - inner_len) * 100 <= inner_len * CAP_PCT && better(&capped, cand) {
                capped = Some(cand);
            }
            if better(&open, cand) {
                open = Some(cand);
            }
        }
    }
    if let Some((_, t, g)) = capped.or(open) {
        return Rib { g, t, guaranteed: true };
    }
    Rib { g: 4, t: 3, guaranteed: false } // the physics floor, said out loud
}

// ---------- geometry ----------
// armored layout v2 (one merged slot region, three spread replica sites):
//   [hdr0 meta0][slots 0..mid][hdr1 meta1][slots mid..][meta2 hdr2]
// meta copy = [body][fnv32], body = CT itself (ct <= 1024) or level-2 checks.
// no-armor (g==0): [hdr0][payload][hdr1][hdr2]
#[derive(Clone)]
pub struct Geom {
    pub len: usize,
    pub g: usize,
    pub t: usize,
    pub s: usize,        // level-1 data squares
    pub p: usize,        // level-1 parity squares
    pub nsq: usize,      // s + p: level-1 slots
    pub ct: usize,       // level-1 check bytes = 4 * nsq
    pub ct_triple: bool, // ct <= 1024: CT is the meta body, no level-2
    pub c: usize,        // level-2 data squares (0 when ct_triple)
    pub p2: usize,       // level-2 parity squares
    pub nsq2: usize,     // c + p2: level-2 slots
    pub m: usize,        // meta BODY bytes: ct (triple) or 4 * nsq2
    pub msize: usize,    // stored meta copy bytes = m + 4 (fnv32)
    pub nslots: usize,   // nsq + nsq2: the merged slot region
    pub mid: usize,      // slot index the [hdr1 meta1] site sits before
    pub hash: u64,       // FNV-64 of the armored payload (stage-local truth)
    pub ex: Extras,      // the conservation fields
    pub total: usize,
}
/// the conservation check rides in the voted header: what the ORIGINAL bytes
/// were (length + FNV-64), which model transmuted them, and (v8) which
/// filter respelled them first.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Extras {
    pub orig_len: u64,
    pub orig_fnv: u64,
    pub model: u8,
    pub filter_id: u8,
    pub filter_param: u32,
}
pub fn geom(len: usize, g: usize, t: usize, hash: u64, ex: Extras) -> Geom {
    if g == 0 {
        return Geom {
            len, g: 0, t: 0, s: 0, p: 0, nsq: 0, ct: 0, ct_triple: true,
            c: 0, p2: 0, nsq2: 0, m: 0, msize: 0, nslots: 0, mid: 0,
            hash, ex, total: 3 * HDR + len,
        };
    }
    let s = (len + BLOCK - 1) / BLOCK;
    let ng1 = if s == 0 { 0 } else { (s + g - 1) / g };
    let p = ng1 * t;
    let nsq = s + p;
    let ct = 4 * nsq;
    let (ct_triple, c, p2, nsq2, m) = if ct <= CT_TRIPLE_MAX {
        (true, 0, 0, 0, ct)
    } else {
        let c = (ct + BLOCK - 1) / BLOCK;
        let ng2 = (c + g - 1) / g;
        let p2 = ng2 * t;
        let nsq2 = c + p2;
        (false, c, p2, nsq2, 4 * nsq2)
    };
    let msize = m + 4;
    let nslots = nsq + nsq2;
    let site = HDR + msize;
    let total = 3 * site + nslots * BLOCK;
    let mid = if nslots == 0 {
        0
    } else {
        let want = total / 2;
        let j = if want > site { (want - site + BLOCK / 2) / BLOCK } else { 0 };
        j.clamp(1, nslots)
    };
    Geom { len, g, t, s, p, nsq, ct, ct_triple, c, p2, nsq2, m, msize, nslots, mid, hash, ex, total }
}
/// the 4 KB-scratch claim for an existing container's geometry
pub fn scratch_guaranteed(g: &Geom) -> bool {
    g.g != 0 && guaranteed_st(g.s, g.g, g.t)
}

pub struct Off {
    pub h0: usize,
    pub m0: usize,
    pub h1: usize,
    pub m1: usize,
    pub m2: usize,
    pub h2: usize,
    pub slot_base: usize, // byte offset of slot 0
}
pub fn offsets(g: &Geom) -> Off {
    if g.g == 0 {
        // [hdr0][payload][hdr1][hdr2]; slot_base names the payload start
        let h1 = HDR + g.len;
        return Off { h0: 0, m0: HDR, h1, m1: h1 + HDR, m2: h1, h2: h1 + HDR, slot_base: HDR };
    }
    let site = HDR + g.msize;
    let h1 = site + g.mid * BLOCK;
    let m2 = 2 * site + g.nslots * BLOCK;
    Off { h0: 0, m0: HDR, h1, m1: h1 + HDR, m2, h2: m2 + g.msize, slot_base: site }
}
/// byte offset of merged slot j -- THE shared wound/placement map. The
/// [hdr1 meta1] site opens a gap before slot `mid`; everything that touches
/// slots (armor, dearmor, info, audit, the drill harness via `info`) walks
/// this one function.
pub fn slot_off(g: &Geom, j: usize) -> usize {
    let site = HDR + g.msize;
    site + j * BLOCK + if j >= g.mid { site } else { 0 }
}

/// one merged stripe across BOTH levels: groups (level-1 first, then
/// level-2) emitted round-robin, row-major, group order fixed. Any
/// contiguous run of k slots touches each group at most ceil(k/q) times,
/// where q is the number of groups present in the sparsest row the run
/// crosses -- the pigeonhole the rib policy budgets for and the audit
/// slides every window to prove.
#[derive(Clone, Copy)]
pub struct Slot {
    pub level: u8,  // 0 = payload squares, 1 = check-table squares
    pub idx: u32,   // square index within its level (data then parity)
    pub group: u32, // merged group id (level-1 groups first)
}
pub fn slot_order(g: &Geom) -> Vec<Slot> {
    let mut groups: Vec<Vec<Slot>> = Vec::new();
    let mut add_level = |level: u8, n: usize| {
        if n == 0 {
            return;
        }
        let ng = (n + g.g - 1) / g.g;
        let gbase = groups.len() as u32;
        for pg in 0..ng {
            let mut grp = Vec::new();
            for i in pg * g.g..((pg + 1) * g.g).min(n) {
                grp.push(Slot { level, idx: i as u32, group: gbase + pg as u32 });
            }
            for pt in 0..g.t {
                grp.push(Slot { level, idx: (n + pg * g.t + pt) as u32, group: gbase + pg as u32 });
            }
            groups.push(grp);
        }
    };
    add_level(0, g.s);
    if !g.ct_triple {
        add_level(1, g.c);
    }
    let maxlen = groups.iter().map(|m| m.len()).max().unwrap_or(0);
    let mut out = Vec::with_capacity(g.nslots);
    for r in 0..maxlen {
        for grp in groups.iter() {
            if r < grp.len() {
                out.push(grp[r]);
            }
        }
    }
    debug_assert_eq!(out.len(), g.nslots);
    out
}

// ---------- residues, syndromes ----------
fn residue(sq: &[u8], m: u32) -> u32 {
    let mut acc: u32 = 0;
    for i in 0..BLOCK {
        let b = if i < sq.len() { sq[i] as u32 } else { 0 };
        acc = (acc * 256 + b) % m;
    }
    acc
}
pub struct Syn {
    pow: [u32; 4096],
    map: Vec<i32>,
    m: u32,
}
pub fn syn(m: u32) -> Syn {
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
// the syndrome tables are pure functions of P and Q: built once, kept for
// the process (v7 rebuilt them on every dearmor call; the audit's boundary
// maps convicted that as the dominant cost)
static SYN_P: OnceLock<Syn> = OnceLock::new();
static SYN_Q: OnceLock<Syn> = OnceLock::new();
pub fn syn_p() -> &'static Syn {
    SYN_P.get_or_init(|| syn(P))
}
pub fn syn_q() -> &'static Syn {
    SYN_Q.get_or_init(|| syn(Q))
}
pub fn syn_distinct(m: u32) -> usize {
    // audit(b): how many of the 2*4096 signed syndromes are distinct entries
    let t = syn(m);
    t.map.iter().filter(|&&e| e >= 0).count()
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
pub fn syndrome_pair(i: u16, d: i8) -> (u32, u32) {
    (syndrome_of(syn_p(), i, d), syndrome_of(syn_q(), i, d))
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

// ---------- headers ----------
fn write_header(g: &Geom, out: &mut [u8]) {
    out[0..4].copy_from_slice(b"EG10");
    out[4] = 4;
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
    out[48] = g.ex.filter_id;
    out[49..53].copy_from_slice(&g.ex.filter_param.to_le_bytes());
    for b in out[53..60].iter_mut() {
        *b = 0;
    }
    let sum = fnv32(&out[0..60]);
    out[60..64].copy_from_slice(&sum.to_le_bytes());
}
/// parse validity and checksum are separate verdicts: a header can be
/// internally consistent yet unverified (its fnv32 wounded) -- usable only
/// down the ladder, with the payload hashes still gating everything.
fn parse_header(b: &[u8]) -> Option<(Geom, bool)> {
    // v10 writes EG10 v4; EGG9 v3 and EGG8 v2 containers (same layout)
    // restore for free -- eggv10 reads both ancestors, writes neither
    let v8_compat = &b[0..4] == b"EGG8" && b[4] == 2;
    let v9_compat = &b[0..4] == b"EGG9" && b[4] == 3;
    if b.len() < HDR || (!v8_compat && !v9_compat && (&b[0..4] != b"EG10" || b[4] != 4)) {
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
    if b[48] > if v8_compat { 7 } else { 12 } {
        return None; // filter ids: 0..=7 in EGG8, 0..=12 in EGG9
    }
    let len = u64::from_le_bytes(b[10..18].try_into().unwrap()) as usize;
    let s = u32::from_le_bytes(b[18..22].try_into().unwrap()) as usize;
    let hash = u64::from_le_bytes(b[22..30].try_into().unwrap());
    let ex = Extras {
        model: b[31],
        orig_len: u64::from_le_bytes(b[32..40].try_into().unwrap()),
        orig_fnv: u64::from_le_bytes(b[40..48].try_into().unwrap()),
        filter_id: b[48],
        filter_param: u32::from_le_bytes(b[49..53].try_into().unwrap()),
    };
    let g = geom(len, grp, t, hash, ex);
    if g.s != s {
        return None; // the v5.0 audit's lesson: validate internal consistency
    }
    let verified = fnv32(&b[0..60]) == u32::from_le_bytes(b[60..64].try_into().unwrap());
    Some((g, verified))
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
fn square_of(src: &[u8], i: usize) -> [u8; BLOCK] {
    let mut b = [0u8; BLOCK];
    let base = i * BLOCK;
    if base < src.len() {
        let n = (src.len() - base).min(BLOCK);
        b[..n].copy_from_slice(&src[base..base + n]);
    }
    b
}
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

// ---------- armor (encode) ----------
pub fn armor(inner: &[u8], grp: usize, tpar: usize, ex: Extras) -> Vec<u8> {
    let g = geom(inner.len(), grp, if grp == 0 { 0 } else { tpar }, fnv64(inner), ex);
    let off = offsets(&g);
    let mut out = vec![0u8; g.total];
    if grp == 0 {
        for o in [off.h0, off.h1, off.h2] {
            write_header(&g, &mut out[o..o + HDR]);
        }
        out[off.slot_base..off.slot_base + g.len].copy_from_slice(inner);
        return out;
    }
    let gf = gf_new();
    let sq1 = shield_squares(inner, g.s, g.g, g.t, &gf);
    let ct1 = checks_of(&sq1);
    let (meta_body, sq2) = if g.ct_triple {
        (ct1, Vec::new())
    } else {
        let sq2 = shield_squares(&ct1, g.c, g.g, g.t, &gf);
        (checks_of(&sq2), sq2)
    };
    debug_assert_eq!(meta_body.len(), g.m);
    let sum = fnv32(&meta_body).to_le_bytes();
    // three spread sites: [hdr0 meta0] head, [hdr1 meta1] mid, [meta2 hdr2] end
    for (ho, mo) in [(off.h0, off.m0), (off.h1, off.m1), (off.h2, off.m2)] {
        write_header(&g, &mut out[ho..ho + HDR]);
        out[mo..mo + g.m].copy_from_slice(&meta_body);
        out[mo + g.m..mo + g.m + 4].copy_from_slice(&sum);
    }
    for (j, sl) in slot_order(&g).iter().enumerate() {
        let sq = if sl.level == 0 { &sq1[sl.idx as usize] } else { &sq2[sl.idx as usize] };
        let o = slot_off(&g, j);
        out[o..o + BLOCK].copy_from_slice(sq);
    }
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

/// pick one 64-byte header from up to three candidate offsets: any
/// checksum-VERIFIED copy wins outright; two verified copies that disagree
/// REFUSE (never choose between witnesses that both claim to be sworn);
/// with none verified, byte-vote then any individually parseable copy --
/// the payload FNV-64 and the conservation FNV-64 still gate everything.
fn select_header(cont: &[u8], offs: &[usize]) -> Result<Geom, String> {
    let cand: Vec<Option<(Geom, bool, [u8; HDR])>> = offs
        .iter()
        .map(|&o| {
            if o + HDR > cont.len() {
                return None;
            }
            let mut raw = [0u8; HDR];
            raw.copy_from_slice(&cont[o..o + HDR]);
            parse_header(&raw).map(|(g, v)| (g, v, raw))
        })
        .collect();
    let verified: Vec<&(Geom, bool, [u8; HDR])> =
        cand.iter().flatten().filter(|(_, v, _)| *v).collect();
    if verified.len() >= 2 {
        for w in verified.windows(2) {
            if w[0].2 != w[1].2 {
                return Err("two checksum-verified headers disagree -- refusing to choose".into());
            }
        }
    }
    if let Some((g, _, _)) = verified.first() {
        return Ok(g.clone());
    }
    // last resorts: vote, then individuals
    if offs.len() == 3 && offs.iter().all(|&o| o + HDR <= cont.len()) {
        let voted: Vec<u8> = (0..HDR)
            .map(|i| {
                let (x, y, z) = (cont[offs[0] + i], cont[offs[1] + i], cont[offs[2] + i]);
                if x == y || x == z { x } else if y == z { y } else { x }
            })
            .collect();
        if let Some((g, _)) = parse_header(&voted) {
            return Ok(g);
        }
    }
    for c in cand.iter().flatten() {
        return Ok(c.0.clone());
    }
    Err("no valid header at any site".into())
}

pub fn dearmor(cont_in: &[u8], wounds_in: &[(usize, usize)], doubles: bool) -> Result<DearmorOut, String> {
    // bootstrap: a header at the head or at the raw end names the geometry
    // (and so the third site's true offsets after truncation padding); with
    // BOTH end sites dead, scan for the magic -- the surviving mid copy sits
    // at a geometry-dependent offset only it can name, so a hit only counts
    // if its own geometry puts a site exactly where it was found
    let seed = select_header(cont_in, &[0, cont_in.len().saturating_sub(HDR)]).or_else(|e| {
        let mut fallback: Option<Geom> = None;
        let mut i = 0usize;
        while i + HDR <= cont_in.len() {
            if &cont_in[i..i + 4] == b"EG10"
                || &cont_in[i..i + 4] == b"EGG9"
                || &cont_in[i..i + 4] == b"EGG8"
            {
                if let Some((g, v)) = parse_header(&cont_in[i..i + HDR]) {
                    let off = offsets(&g);
                    let site_consistent = i == off.h0 || i == off.h1 || i == off.h2;
                    if v && site_consistent {
                        return Ok(g);
                    }
                    if fallback.is_none() {
                        fallback = Some(g);
                    }
                }
            }
            i += 1;
        }
        fallback.ok_or(e)
    })?;
    if cont_in.len() > seed.total {
        return Err("container longer than its own geometry".into());
    }
    // truncation is not special: pad back to size, call the missing tail a wound
    let mut wounds: Vec<(usize, usize)> = wounds_in.to_vec();
    let padded = seed.total - cont_in.len();
    let mut cont = cont_in.to_vec();
    if padded > 0 {
        cont.resize(seed.total, 0);
        wounds.push((cont_in.len(), padded));
    }
    let off = offsets(&seed);
    // final selection across all three true sites
    let g = select_header(&cont, &[off.h0, off.h1, off.h2])?;
    let off = offsets(&g);
    if g.total != seed.total {
        // a differing survivor names a different layout; re-derive the padding
        return Err("surviving headers name inconsistent geometry".into());
    }

    if g.g == 0 {
        // no armor: checks + hash only, honestly incapable of repair
        let inner = cont[off.slot_base..off.slot_base + g.len].to_vec();
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

    // ---- meta copy selection (same law as headers) ----
    let copy_at = |o: usize| -> (&[u8], bool) {
        let body = &cont[o..o + g.m];
        let sum = u32::from_le_bytes(cont[o + g.m..o + g.m + 4].try_into().unwrap());
        (body, fnv32(body) == sum)
    };
    let copies = [copy_at(off.m0), copy_at(off.m1), copy_at(off.m2)];
    let ver: Vec<&(&[u8], bool)> = copies.iter().filter(|(_, v)| *v).collect();
    if ver.len() >= 2 {
        for w in ver.windows(2) {
            if w[0].0 != w[1].0 {
                return Err("two checksum-verified meta copies disagree -- refusing to choose".into());
            }
        }
    }
    let (meta_body, meta_src): (Vec<u8>, &str) = if let Some((b, _)) = ver.first() {
        (b.to_vec(), "verified copy")
    } else {
        let voted: Vec<u8> = (0..g.m)
            .map(|i| {
                let (x, y, z) = (cont[off.m0 + i], cont[off.m1 + i], cont[off.m2 + i]);
                if x == y || x == z { x } else if y == z { y } else { x }
            })
            .collect();
        (voted, "voted, unverified")
    };

    let sp_tab = syn_p();
    let sq_tab = syn_q();
    let gf = gf_new();
    let order = slot_order(&g);
    let in_wound = |lo: usize, hi: usize| wounds.iter().any(|&(a, l)| a < hi && a + l > lo);

    // slots region -> squares of one level, wounded slots marked dead before
    // anyone believes their bytes; placement comes from the one slot_off map
    let gather = |level: u8, n: usize| -> (Vec<[u8; BLOCK]>, Vec<bool>) {
        let mut sq = vec![[0u8; BLOCK]; n];
        let mut dead = vec![false; n];
        for (j, sl) in order.iter().enumerate() {
            if sl.level != level {
                continue;
            }
            let a = slot_off(&g, j);
            sq[sl.idx as usize].copy_from_slice(&cont[a..a + BLOCK]);
            if in_wound(a, a + BLOCK) {
                dead[sl.idx as usize] = true;
            }
        }
        (sq, dead)
    };
    // bit scale first (residues), then square scale (RS parity); one routine
    // for both levels, exactly v7's repair semantics
    let repair = |sq: &mut Vec<[u8; BLOCK]>,
                  dead: &[bool],
                  checks: &dyn Fn(usize) -> (u32, u32),
                  n_data: usize,
                  mode: Repair,
                  t: &mut Tally| {
        let nsq = sq.len();
        let mut bad = vec![false; nsq];
        for i in 0..nsq {
            let (rp, rq) = checks(i);
            if dead[i] || mode == Repair::ParityOnly {
                // trust nothing repaired here: random wound-fill can fake a
                // 1-2 bit fix (~2.4e-4/square) and poison the rebuilds
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
        let ng = if n_data == 0 { 0 } else { (n_data + g.g - 1) / g.g };
        for pg in 0..ng {
            let lo = pg * g.g;
            let m = ((pg + 1) * g.g).min(n_data) - lo;
            let members: Vec<(u8, usize)> = (0..m)
                .map(|i| (i as u8, lo + i))
                .chain((0..g.t).map(|pt| ((GMAX + pt) as u8, n_data + pg * g.t + pt)))
                .collect();
            let bads: Vec<usize> = (0..members.len()).filter(|&k| bad[members[k].1]).collect();
            if bads.is_empty() {
                continue;
            }
            if bads.len() > g.t {
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
    };

    let run = |mode: Repair| -> (Vec<u8>, Tally, String) {
        let mut t = Tally { clean: 0, bitfixed: 0, bitfixed2: 0, rebuilt: 0, detected: 0 };
        let mut t2 = Tally { clean: 0, bitfixed: 0, bitfixed2: 0, rebuilt: 0, detected: 0 };
        let (ct, ct_report) = if g.ct_triple {
            (meta_body.clone(), format!("CT triplicate ({})", meta_src))
        } else {
            let meta_checks = |i: usize| -> (u32, u32) {
                let o = 4 * i;
                let rp = u16::from_le_bytes([meta_body[o], meta_body[o + 1]]) as u32;
                let rq = u16::from_le_bytes([meta_body[o + 2], meta_body[o + 3]]) as u32;
                (rp, rq)
            };
            let (mut sq2, dead2) = gather(1, g.nsq2);
            repair(&mut sq2, &dead2, &meta_checks, g.c, mode, &mut t2);
            let mut ct = vec![0u8; g.ct];
            for i in 0..g.c {
                let base = i * BLOCK;
                let n = (g.ct - base).min(BLOCK);
                ct[base..base + n].copy_from_slice(&sq2[i][..n]);
            }
            let rep = format!(
                "{} clean, {} fixed, {} rebuilt, {} bad (meta {})",
                t2.clean, t2.bitfixed + t2.bitfixed2, t2.rebuilt, t2.detected, meta_src
            );
            (ct, rep)
        };
        let checks = |i: usize| -> (u32, u32) {
            let o = 4 * i;
            let rp = u16::from_le_bytes([ct[o], ct[o + 1]]) as u32;
            let rq = u16::from_le_bytes([ct[o + 2], ct[o + 3]]) as u32;
            (rp, rq)
        };
        let (mut sq1, dead1) = gather(0, g.nsq);
        repair(&mut sq1, &dead1, &checks, g.s, mode, &mut t);
        let mut data = vec![0u8; g.len];
        for i in 0..g.s {
            let base = i * BLOCK;
            let n = (g.len - base).min(BLOCK);
            data[base..base + n].copy_from_slice(&sq1[i][..n]);
        }
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
