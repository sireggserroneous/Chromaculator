//! mix12 -- v12's live model: mix11 forked at M2a and WIDENED to a 16-bit
//! pipeline. The reading is glossary.js:164, "kept rather than rounded away":
//! v11's whole path was 12-bit -- coder p in 1..4095, mixer clamp +-2047,
//! APM entries 12-bit under a fixed >>7 step -- and the probe (M2 brief)
//! found the mixer pinned at the clamp on 10-62% of decisions, each costing
//! 0.009-0.016 bits against an empirical miss rate worth 0.0004-0.0008.
//! Nothing here changes WHAT is read; every reading is kept to sixteen bits
//! instead of twelve:
//!   - probabilities are u16 in 1..65535 (P(bit==0), as everywhere);
//!   - stretch domain +-4095 through STRETCH16 (exact logits) and SQUASH16
//!     (8,191 entries; e^-16 < 2^-16 so the domain saturates by construction);
//!   - the StateMap's p22 feeds stretch as p22 >> 6 (not >> 10);
//!   - the APM entries carry (p22 << 10 | count) with the StateMap's
//!     count-adaptive step, so a bucket can learn 0.9999 (v11's fixed >>7 on a
//!     12-bit entry could not hold a bucket above 3,969/4,096 = 0.969);
//!   - the learning shifts rise by 4 because the error is at 16-bit scale --
//!     the v11 dynamics are kept at the same operating points.
//!
//! v12-M2c(b), the two-voices reading (chroma-ui.js:569): every HASHED
//! context bucket carries a check byte (Mahoney's lpaq/paq8 HashTable), so two
//! contexts landing on one bucket are told apart instead of sharing a
//! history. The check lives at index 0 of the 16-state bucket (the nibble
//! tree uses nodes 1..15; the byte was free), the probe is 2-way (the bucket
//! and its neighbour base ^ 16), and on a double miss the bucket whose
//! order-1 node has seen fewer bits is reclaimed (paq8's priority).
//!
//! Attribution: Matt Mahoney's zpaq (15-bit squash, 16-bit coder, StateMap,
//! APM), Byron Knoll's cmix (16-bit coder); the model's readings are v11's
//! (mix11.rs, frozen as a trial entrant beside this one).
//!
//! p is P(bit==0) EVERYWHERE in this codebase; y = 65536 when the bit is 0.
//! The polarity test (all-zeros input drives every p toward 65535) stands
//! guard below.

use crate::squash_tab::{SQUASH16, STRETCH16};
use crate::state_tab::NEX;

const PINIT: u16 = 1 << 15;
const RATE: u32 = 5;
pub const MIX12_LR: u32 = 11; // v11's sweep winner; LR_WIDEN is added inside learn()
/// the error is now at 16-bit scale (y = 65536): four more shift bits keep
/// every learning step exactly where v11's was
const LR_WIDEN: u32 = 4;
const WCLAMP: i32 = 1 << 20;
const NINPUT: usize = 12; // o0 chain-final o2-tap sp13s sp24s ind1 ind2 mb mm lat1 lat2 + bias
const LAT_SMAX: usize = 384;
const LAT_LOCK1: usize = 1 << 16;
const LAT_LOCK2: usize = 1 << 18;
const ISSE_LR: u32 = 9 + LR_WIDEN;
/// the stretch domain: SQUASH16 is indexed by t + SCLAMP
const SCLAMP: i32 = 4095;
const SM_LIMIT: u32 = 1023;
/// the APM's count-adaptive step: a prior count (the identity init is worth
/// this many observations) and a limit (steady rate ~1/256)
const APM_N0: u32 = 4;
const APM_LIMIT: u32 = 255;

#[inline]
fn upd(c: &mut u16, b: u32) {
    if b == 0 {
        *c += ((65536u32 - *c as u32) >> RATE) as u16;
    } else {
        *c -= *c >> RATE;
    }
}
#[inline]
fn sq(t: i32) -> u16 {
    SQUASH16[(t + SCLAMP) as usize].clamp(1, 65535)
}
#[inline]
fn st(p: u16) -> i32 {
    STRETCH16[p as usize] as i32
}

/// the keeps-R reading (spec.md:204-205): a per-(node x state) map from a
/// bit-history state to a probability with a COUNT-adaptive step.
/// Attribution: Mahoney's StateMap, flipped for P(bit==0).
/// Entry: (p22 << 10) | count10. POLARITY: y22 is HIGH on bit 0.
pub struct StateMap {
    t: Vec<u32>,
}
impl StateMap {
    pub fn seed(&mut self, src: &[u32], count_cap: u32) {
        for (dst, &v) in self.t.iter_mut().zip(src.iter()) {
            let p22 = v >> 10;
            let cnt = (v & 1023).min(count_cap);
            *dst = (p22 << 10) | cnt;
        }
    }
    fn new() -> Self {
        let mut t = vec![0u32; 16 * 256];
        for node in 0..16 {
            for s in 0..256 {
                let n0 = NEX[s][2] as u64;
                let n1 = NEX[s][3] as u64;
                let p22 = ((2 * n0 + 1) << 21) / (n0 + n1 + 1);
                t[node * 256 + s] = (p22 as u32) << 10;
            }
        }
        StateMap { t }
    }
    /// sixteen bits of the 22 (v11 read twelve)
    #[inline]
    fn p(&self, cx: usize) -> u16 {
        ((self.t[cx] >> 16) as u16).clamp(1, 65535)
    }
    #[inline]
    fn update(&mut self, cx: usize, b: u32, recip: &[u16]) {
        let v = self.t[cx];
        let n = (v & 1023) as usize;
        let p22 = (v >> 10) as i64;
        let y22: i64 = if b == 0 { (1 << 22) - 1 } else { 0 };
        let p22 = p22 + (((y22 - p22) * recip[n] as i64) >> 16);
        debug_assert!((0..(1 << 22)).contains(&p22));
        let n2 = (n as u32) + ((n < SM_LIMIT as usize) as u32);
        self.t[cx] = ((p22 as u32) << 10) | n2;
    }
    fn hash_state(&self, feed: &mut dyn FnMut(u64)) {
        for &v in self.t.iter() {
            feed(v as u64);
        }
    }
}

/// the APM / SSE stage at sixteen bits: 33 interpolated buckets over the
/// +-4095 stretch axis (width 256) per context; entries (p22 << 10 | count)
/// learn with the StateMap's count-adaptive step. Attribution: Mahoney's APM
/// (paq6+, zpaq), Shkarin's SSE before it; the count-adaptive step is the
/// StateMap's, applied here so a bucket can hold 0.9999.
/// The APM's index domain and bucket width -- probe C of the M2a amendment
/// (PREDICTIONS.md): 33 buckets of 128 logit-units over +-2047, the lineage's
/// resolution (paq8/lpaq/zpaq), with the mixer's +-4095 and the 16-bit
/// entries kept. The built A (33 @256 over +-4095) lost the LZ arm's
/// mid-domain calibration: MIX12 +0.8% vs MIX11; under C it is within +0.07%
/// and the CM arms gained a further 0.2-0.9%. B (65 @128 over +-4095) tied C
/// within 53 B over four rows and costs twice the table.
const APM_BUCKETS: usize = 33;
const APM_SHIFT: u32 = 7; // bucket width 128
const APM_CLAMP: i32 = 2047;
struct Apm16 {
    t: Vec<u32>,
}
impl Apm16 {
    fn new(nctx: usize) -> Self {
        let w = 1i32 << APM_SHIFT;
        let mut axis = [0u32; APM_BUCKETS];
        for (i, a) in axis.iter_mut().enumerate() {
            let s = ((i as i32) - (APM_BUCKETS as i32 / 2)) * w;
            let p16 = SQUASH16[(s + SCLAMP).clamp(0, 2 * SCLAMP) as usize] as u32;
            *a = ((p16 << 6) << 10) | APM_N0;
        }
        let mut t = vec![0u32; nctx * APM_BUCKETS];
        for k in 0..nctx {
            t[k * APM_BUCKETS..(k + 1) * APM_BUCKETS].copy_from_slice(&axis);
        }
        Apm16 { t }
    }
    #[inline]
    fn p16(v: u32) -> u32 {
        v >> 16
    }
    /// (refined p, bucket base, interpolation weight 0..127)
    #[inline]
    fn refine(&self, ctx: usize, stretch: i32) -> (u16, usize, u32) {
        let s = stretch.clamp(-APM_CLAMP, APM_CLAMP) + APM_CLAMP + 1; // 1..=4095
        let pos = (s >> APM_SHIFT) as usize; // 0..=31
        let wmax = 1u32 << APM_SHIFT;
        let w = (s as u32) & (wmax - 1);
        let base = ctx * APM_BUCKETS + pos;
        let pa = ((Self::p16(self.t[base]) * (wmax - w) + Self::p16(self.t[base + 1]) * w) >> APM_SHIFT) as u16;
        (pa.clamp(1, 65535), base, w)
    }
    #[inline]
    fn learn(&mut self, base: usize, w: u32, b: u32, recip: &[u16]) {
        let wmax = 1u32 << APM_SHIFT;
        for (i, share) in [(base, wmax - w), (base + 1, w)] {
            if share == 0 {
                continue;
            }
            let v = self.t[i];
            let n = (v & 1023) as usize;
            let p22 = (v >> 10) as i64;
            let y22: i64 = if b == 0 { (1 << 22) - 1 } else { 0 };
            let p22 = p22 + (((y22 - p22) * recip[n] as i64) >> 16);
            debug_assert!((0..(1 << 22)).contains(&p22));
            let n2 = (n as u32) + ((n < APM_LIMIT as usize) as u32);
            self.t[i] = ((p22 as u32) << 10) | n2;
        }
    }
    fn hash_state(&self, feed: &mut dyn FnMut(u64)) {
        for &v in self.t.iter() {
            feed(v as u64);
        }
    }
}

/// the multiplicative hash every hashed context shares (Fibonacci hashing)
const GOLD: u64 = 0x9E3779B97F4A7C15;

/// M2c(b): claim a checksummed bucket. `base` is the hashed bucket (a
/// multiple of 16), `chk` an independent 8-bit slice of the same hash (0 is
/// reserved for "empty", so a live check is 1..=255). Returns the bucket the
/// caller may use: its own if the check matches at `base` or `base ^ 16`,
/// else the less-experienced of the two, reclaimed (check set, states
/// zeroed). Encoder and decoder call this in the same order with the same
/// arguments -- the mirror -- and EGG_STATEHASH covers the check bytes.
/// Attribution: Mahoney's lpaq1 / paq8 HashTable (check byte + priority).
#[inline]
fn claim(t: &mut [u8], base: usize, chk: u8) -> usize {
    let chk = chk.max(1);
    if t[base] == chk {
        return base;
    }
    let alt = base ^ 16;
    if t[alt] == chk {
        return alt;
    }
    let seen = |s: u8| NEX[s as usize][2] as u32 + NEX[s as usize][3] as u32;
    let victim = if seen(t[base + 1]) <= seen(t[alt + 1]) { base } else { alt };
    t[victim] = chk;
    t[victim + 1..victim + 16].fill(0);
    victim
}

pub use crate::mix11::{match_state, MatchModel};
#[inline]
fn len_bucket(mlen: u32) -> usize {
    match mlen {
        1 => 0,
        2 => 1,
        3 => 2,
        4..=5 => 3,
        6..=7 => 4,
        8..=15 => 5,
        16..=31 => 6,
        _ => 7,
    }
}

// ---------------- the mixed model ----------------
pub struct Mix12 {
    o0: Vec<u16>, // 2 phases x 16 nodes, 16-bit
    o1s: Vec<u8>,
    o2s: Vec<u8>,
    sm1: StateMap,
    recip: Vec<u16>,
    o3s: Vec<u8>,
    o4s: Vec<u8>,
    o6s: Vec<u8>,
    sp13s: Vec<u8>,
    sp24s: Vec<u8>,
    sms1: StateMap,
    sms2: StateMap,
    h1: [u8; 256],
    h2: Vec<u8>,
    ind1s: Vec<u8>,
    ind2s: Vec<u8>,
    smi1: StateMap,
    smi2: StateMap,
    lat_cnt: Vec<u32>,
    lat_state: u8,
    lat_s: u32,
    lat_cl1: usize,
    lat_cl2: usize,
    lat1s: Vec<u8>,
    lat2s: Vec<u8>,
    sml1: StateMap,
    sml2: StateMap,
    lr: u32,
    isse_w: Vec<i32>,
    mb: Vec<u16>,
    mm: Vec<u16>,
    w: Vec<i32>,
    apm: Apm16,
    apm2: Apm16,
    pub mmodel: MatchModel,
}

/// v12-M2c(c), the free-to-guess reading (glossary.js:104) in another
/// dialect: a book is the site-prior's shape -- mixer weights, the h1/h2
/// followers, the o1 states, the o1 StateMap -- trained by `gen-prior --book`
/// on files that sit in NO corpus of this repo (the test rows are excluded by
/// construction; the arial and segoe families whole). Shipped as trial arms
/// with their own MODEL bytes; a book that loses is simply not chosen.
/// Attribution: v11-M5's site book; Mahoney's zpaq config lineage.
pub struct Book {
    pub on: bool,
    pub w: &'static [i32],
    pub h1: &'static [u8],
    pub h2: &'static [u8],
    pub o1s: &'static [u8],
    pub sm1: &'static [u32],
}
pub static BOOK_PE: Book = Book {
    on: crate::prior_pe::PE_ON,
    w: &crate::prior_pe::PE_W,
    h1: &crate::prior_pe::PE_H1,
    h2: &crate::prior_pe::PE_H2,
    o1s: &crate::prior_pe::PE_O1S,
    sm1: &crate::prior_pe::PE_SM1,
};
pub static BOOK_TTF: Book = Book {
    on: crate::prior_ttf::TTF_ON,
    w: &crate::prior_ttf::TTF_W,
    h1: &crate::prior_ttf::TTF_H1,
    h2: &crate::prior_ttf::TTF_H2,
    o1s: &crate::prior_ttf::TTF_O1S,
    sm1: &crate::prior_ttf::TTF_SM1,
};

/// everything one bit's learn() needs, handed back by predict()
pub struct Bit12 {
    pub p: u16, // the FINAL 16-bit probability to code with
    p_mix: u16,
    wsel: usize,
    idx: [usize; 14],
    smcx: [usize; 11],
    lat_on: bool,
    isse_wi: [usize; 4],
    isse_in: [i32; 4],
    isse_p: [u16; 4],
    sts: [i32; NINPUT],
    mb_on: bool,
    mm_on: bool,
    apm_base: usize,
    apm_w: u32,
    apm2_base: usize,
    apm2_w: u32,
}

impl Mix12 {
    pub fn new() -> Self {
        let mut w = vec![0i32; 1536 * NINPUT];
        for v in w.chunks_mut(NINPUT) {
            for vk in v.iter_mut().take(9) {
                *vk = (1 << 16) / 9;
            }
        }
        Mix12 {
            o0: vec![PINIT; 32],
            o1s: vec![0u8; 256 * 16],
            o2s: vec![0u8; 65536 * 16],
            sm1: StateMap::new(),
            recip: (0..1024u32).map(|n| ((1u32 << 17) / (2 * n + 3)) as u16).collect(),
            o3s: vec![0u8; (1 << 20) * 16],
            o4s: vec![0u8; (1 << 18) * 16],
            o6s: vec![0u8; (1 << 20) * 16],
            sp13s: vec![0u8; (1 << 18) * 16],
            sp24s: vec![0u8; (1 << 18) * 16],
            sms1: StateMap::new(),
            sms2: StateMap::new(),
            h1: [0u8; 256],
            h2: vec![0u8; 65536],
            ind1s: vec![0u8; (1 << 17) * 16],
            ind2s: vec![0u8; (1 << 20) * 16],
            smi1: StateMap::new(),
            smi2: StateMap::new(),
            lat_cnt: vec![0u32; LAT_SMAX + 1],
            lat_state: 0,
            lat_s: 0,
            lat_cl1: 0,
            lat_cl2: 0,
            lat1s: vec![0u8; (1 << 18) * 16],
            lat2s: vec![0u8; (1 << 18) * 16],
            sml1: StateMap::new(),
            sml2: StateMap::new(),
            lr: MIX12_LR,
            isse_w: {
                let mut w = vec![0i32; 4 * 4096 * 2];
                for k in 0..4 * 4096 {
                    w[k * 2] = 1 << 16;
                }
                w
            },
            mb: vec![PINIT; 512],
            mm: vec![PINIT; 8 * 2 * 16],
            w,
            apm: Apm16::new(256),
            apm2: Apm16::new(288),
            mmodel: MatchModel::new(),
        }
    }
    /// THE one place per-byte model state moves (mix11's, verbatim)
    pub fn byte_update(&mut self, buf: &[u8], pos_new: usize) {
        self.mmodel.update(buf, pos_new);
        let b = buf[pos_new - 1];
        if pos_new >= 2 {
            self.h1[buf[pos_new - 2] as usize] = b;
        }
        if pos_new >= 3 {
            self.h2[((buf[pos_new - 3] as usize) << 8) | buf[pos_new - 2] as usize] = b;
        }
        let p = pos_new;
        if self.lat_state == 0 {
            let smax = LAT_SMAX.min(p.saturating_sub(1));
            // v14-N2c item 2.2: the SAME 382 comparisons and the same increments,
            // written so both sides are unit-stride and the compare is branchless --
            // `st` ascending walks `lat_cnt` forward while the window is read in
            // reverse, which is the pairing the index arithmetic already implied.
            // Addition commutes, so the counters are identical byte for byte; the
            // point is that LLVM can vectorise the compare instead of emitting 382
            // bounds-checked loads behind an unpredictable branch. Measured against
            // `lat_state = 2` (the detector removed entirely), which is worth 8.4%
            // on the round-trip -- this is the bit-exact share of it.
            if smax == LAT_SMAX {
                // the steady state, and it is almost the whole file: both slices are
                // exactly LAT_SMAX - 2 long at COMPILE time, so there is no bounds
                // check and no runtime trip count left to defeat the vectoriser.
                let win = &buf[p - 1 - LAT_SMAX..p - 3];
                for (c, &v) in self.lat_cnt[3..=LAT_SMAX].iter_mut().zip(win.iter().rev()) {
                    *c += (v == b) as u32;
                }
            } else if smax >= 3 {
                let win = &buf[p - 1 - smax..p - 3];
                for (st, &v) in (3..=smax).zip(win.iter().rev()) {
                    self.lat_cnt[st] += (v == b) as u32;
                }
            }
            if p == LAT_LOCK1 || p == LAT_LOCK2 {
                let mut bs = 0usize;
                let mut bc = 0u32;
                let mut sum = 0u64;
                for st in 3..=LAT_SMAX {
                    let c = self.lat_cnt[st];
                    sum += c as u64;
                    if c > bc {
                        bc = c;
                        bs = st;
                    }
                }
                for st in 3..bs {
                    if self.lat_cnt[st] * 20 >= bc * 17 {
                        bs = st;
                        break;
                    }
                }
                let avg = (sum / (LAT_SMAX as u64 - 2)).max(1) as u32;
                let n = (p.saturating_sub(3)) as u32;
                if bc > avg.saturating_mul(3) && bc > n / 8 && bs >= 3 {
                    self.lat_state = 1;
                    self.lat_s = bs as u32;
                } else if p >= LAT_LOCK2 {
                    self.lat_state = 2;
                }
            }
        }
        if self.lat_state == 1 {
            let st = self.lat_s as usize;
            if p >= 2 * st {
                let a = buf[p - st] as u64;
                let b2 = buf[p - 2 * st] as u64;
                let k1 = a | (0xA3u64 << 56);
                let h1 = k1.wrapping_mul(GOLD);
                self.lat_cl1 = claim(&mut self.lat1s, (((h1 >> 46) as usize) & 0x3ffff) * 16, (h1 >> 38) as u8);
                let k2 = a | (b2 << 8) | (0xA4u64 << 56);
                let h2 = k2.wrapping_mul(GOLD);
                self.lat_cl2 = claim(&mut self.lat2s, (((h2 >> 46) as usize) & 0x3ffff) * 16, (h2 >> 38) as u8);
            } else {
                self.lat_cl1 = 0;
                self.lat_cl2 = 0;
            }
        }
    }
    #[inline]
    pub fn ctx_ind1(&mut self, c1: u8, phase: usize, hi: u32) -> usize {
        let key = (c1 as u64)
            | ((self.h1[c1 as usize] as u64) << 8)
            | (0xA1u64 << 56)
            | if phase == 1 { ((0x10 | hi) as u64) << 48 } else { 0 };
        let h = key.wrapping_mul(GOLD);
        claim(&mut self.ind1s, (((h >> 47) as usize) & 0x1ffff) * 16, (h >> 39) as u8)
    }
    #[inline]
    pub fn ctx_ind2(&mut self, pair: usize, phase: usize, hi: u32) -> usize {
        let key = (pair as u64)
            | ((self.h2[pair] as u64) << 16)
            | (0xA2u64 << 56)
            | if phase == 1 { ((0x10 | hi) as u64) << 48 } else { 0 };
        let h = key.wrapping_mul(GOLD);
        claim(&mut self.ind2s, (((h >> 44) as usize) & 0xfffff) * 16, (h >> 36) as u8)
    }
    #[inline]
    pub fn ctx18(&mut self, prev4: u32, phase: usize, hi: u32) -> usize {
        let key: u64 = if phase == 0 { prev4 as u64 } else { prev4 as u64 | ((0x10 | hi) as u64) << 32 };
        let base = (((key.wrapping_mul(0x9E3779B1) >> 14) as usize) & 0x3ffff) * 16;
        claim(&mut self.o4s, base, (key.wrapping_mul(GOLD) >> 56) as u8)
    }
    #[inline]
    /// the same two sparse tables, entered by an ARBITRARY key instead of by
    /// the byte tail (v13-M3c, WS-N). Nothing about the model changes: the
    /// caller decides what the context means, the tables and the mixer inputs
    /// are the ones that were already there, and every existing caller still
    /// goes through `ctx_sparse` untouched.
    pub fn ctx_key(&mut self, key: u64, pick: u32, phase: usize, hi: u32) -> usize {
        let key = key ^ ((pick as u64) << 40) ^ if phase == 1 { ((0x10 | hi) as u64) << 44 } else { 0 };
        let h = key.wrapping_mul(GOLD);
        let base = (((h >> 46) as usize) & 0x3ffff) * 16;
        let t = if pick == 0 { &mut self.sp13s } else { &mut self.sp24s };
        claim(t, base, (h >> 38) as u8)
    }
    pub fn ctx_sparse(&mut self, tail: u64, pick: u32, phase: usize, hi: u32) -> usize {
        let two: u64 = if pick == 0 {
            (tail & 0xff) | (((tail >> 16) & 0xff) << 8)
        } else {
            ((tail >> 8) & 0xff) | (((tail >> 24) & 0xff) << 8)
        };
        let key = two | ((pick as u64) << 16) | if phase == 1 { ((0x10 | hi) as u64) << 24 } else { 0 };
        let h = key.wrapping_mul(GOLD);
        let base = (((h >> 46) as usize) & 0x3ffff) * 16;
        let t = if pick == 0 { &mut self.sp13s } else { &mut self.sp24s };
        claim(t, base, (h >> 38) as u8)
    }
    #[inline]
    pub fn ctx20(&mut self, tail: u64, nbytes: u32, phase: usize, hi: u32) -> usize {
        let key = (tail & ((1u64 << (8 * nbytes)) - 1))
            ^ ((nbytes as u64) << 56)
            ^ if phase == 1 { ((0x10 | hi) as u64) << 48 } else { 0 };
        let h = key.wrapping_mul(GOLD);
        let base = (((h >> 44) as usize) & 0xfffff) * 16;
        let t = if nbytes == 3 { &mut self.o3s } else { &mut self.o6s };
        claim(t, base, (h >> 36) as u8)
    }
    /// one bit: gather the readings, mix, refine -- the final 16-bit p and
    /// the learn() handle. Identical on encoder and decoder (the mirror).
    #[allow(clippy::too_many_arguments)]
    pub fn predict(
        &mut self,
        node: usize,
        phase: usize,
        hist: usize,
        ctx18: usize,
        ctx3: usize,
        ctx6: usize,
        cs13: usize,
        cs24: usize,
        ci1: usize,
        ci2: usize,
        steer: Option<u32>,
        mm_exp: Option<(u32, u32)>,
        am: usize,
    ) -> Bit12 {
        let prevnib = hist & 15;
        let i0 = phase * 16 + node;
        let i1 = (hist & 0xff) * 16 + node;
        let i2 = (hist & 0xffff) * 16 + node;
        let i3 = ctx3 + node;
        let i4 = ctx18 + node;
        let i6 = ctx6 + node;
        let is1 = cs13 + node;
        let is2 = cs24 + node;
        let ii1 = ci1 + node;
        let ii2 = ci2 + node;
        let (mb_on, imb) = match steer {
            Some(mbit) => (true, ((mbit as usize) * 16 + prevnib) * 16 + node),
            None => (false, 0),
        };
        let (mm_on, imm, mm_prior, mstate) = match mm_exp {
            Some((ebit, mlen)) => {
                let idx = (len_bucket(mlen) * 2 + ebit as usize) * 16 + node;
                let mag = (mlen.min(32) as i32) * 48;
                let prior = if ebit == 0 { mag } else { -mag };
                (true, idx, prior, match_state(mlen))
            }
            None => (false, 0, 0, 0),
        };
        let lat_on = self.lat_state == 1;
        let (il1, il2) = (self.lat_cl1 + node, self.lat_cl2 + node);
        let nb = node * 256;
        let smcx = [
            nb + self.o1s[i1] as usize,
            nb + self.o2s[i2] as usize,
            nb + self.o3s[i3] as usize,
            nb + self.o4s[i4] as usize,
            nb + self.o6s[i6] as usize,
            nb + self.sp13s[is1] as usize,
            nb + self.sp24s[is2] as usize,
            nb + self.ind1s[ii1] as usize,
            nb + self.ind2s[ii2] as usize,
            nb + self.lat1s[il1] as usize,
            nb + self.lat2s[il2] as usize,
        ];
        // the settle chain at sixteen bits
        let mut isse_wi = [0usize; 4];
        let mut isse_in = [0i32; 4];
        let mut isse_p = [0u16; 4];
        let mut p_chain = self.sm1.p(smcx[0]);
        let mut o2_tap = PINIT;
        for (k, &cx) in smcx[1..5].iter().enumerate() {
            let st_in = st(p_chain);
            let wi = (k * 4096 + cx) * 2;
            let t = ((self.isse_w[wi] as i64 * st_in as i64 + self.isse_w[wi + 1] as i64 * 256) >> 16)
                .clamp(-(SCLAMP as i64), SCLAMP as i64) as i32;
            p_chain = sq(t);
            isse_wi[k] = wi;
            isse_in[k] = st_in;
            isse_p[k] = p_chain;
            if k == 0 {
                o2_tap = p_chain;
            }
        }
        let ps = [
            self.o0[i0],
            p_chain,
            o2_tap,
            self.sms1.p(smcx[5]),
            self.sms2.p(smcx[6]),
            self.smi1.p(smcx[7]),
            self.smi2.p(smcx[8]),
            if mb_on { self.mb[imb] } else { PINIT },
            if mm_on { self.mm[imm] } else { PINIT },
            if lat_on { self.sml1.p(smcx[9]) } else { PINIT },
            if lat_on { self.sml2.p(smcx[10]) } else { PINIT },
        ];
        let mut sts = [0i32; NINPUT];
        for k in 0..NINPUT - 1 {
            sts[k] = st(ps[k]);
        }
        if mm_on {
            sts[8] = (sts[8] + mm_prior).clamp(-SCLAMP, SCLAMP);
        }
        sts[NINPUT - 1] = 256; // bias
        let o1top3 = (hist >> 5) & 7;
        let wsel = ((((phase * 16 + node) * 3 + mstate) * 2 + am) * 8 + o1top3) * NINPUT;
        let mut t: i64 = 0;
        for (&wv, &s) in self.w[wsel..wsel + NINPUT].iter().zip(sts.iter()) {
            t += wv as i64 * s as i64;
        }
        let t = (t >> 16).clamp(-(SCLAMP as i64), SCLAMP as i64) as i32;
        let p_mix = sq(t);
        // APM stage 1 (o1 byte context)
        let actx = hist & 0xff;
        let (pa, apm_base, apm_w) = self.apm.refine(actx, st(p_mix));
        let p1 = (((pa as u32) * 3 + p_mix as u32) >> 2).clamp(1, 65535) as u16;
        // APM stage 2: match bucket x phase x previous nib
        let mmb = match mm_exp {
            Some((_, mlen)) => 1 + len_bucket(mlen),
            None => 0,
        };
        let a2ctx = (mmb * 2 + phase) * 16 + prevnib;
        let (pa2, apm2_base, apm2_w) = self.apm2.refine(a2ctx, st(p1));
        let p = (((pa2 as u32) * 3 + p1 as u32) >> 2).clamp(1, 65535) as u16;
        Bit12 {
            p,
            p_mix,
            wsel,
            idx: [i0, i1, i2, i3, i4, i6, is1, is2, ii1, ii2, imb, imm, il1, il2],
            smcx,
            lat_on,
            isse_wi,
            isse_in,
            isse_p,
            sts,
            mb_on,
            mm_on,
            apm_base,
            apm_w,
            apm2_base,
            apm2_w,
        }
    }
    #[inline]
    pub fn learn(&mut self, b9: &Bit12, b: u32) {
        let y: i32 = if b == 0 { 65536 } else { 0 };
        let err = y - b9.p_mix as i32;
        let sh = self.lr + LR_WIDEN;
        for (w, &s) in self.w[b9.wsel..b9.wsel + NINPUT].iter_mut().zip(b9.sts.iter()) {
            *w = (*w + ((err * s) >> sh)).clamp(-WCLAMP, WCLAMP);
        }
        upd(&mut self.o0[b9.idx[0]], b);
        self.sm1.update(b9.smcx[0], b, &self.recip);
        for k in 0..4 {
            let err = y - b9.isse_p[k] as i32;
            let wi = b9.isse_wi[k];
            self.isse_w[wi] = (self.isse_w[wi] + ((err * b9.isse_in[k]) >> ISSE_LR)).clamp(-WCLAMP, WCLAMP);
            self.isse_w[wi + 1] = (self.isse_w[wi + 1] + ((err * 256) >> ISSE_LR)).clamp(-WCLAMP, WCLAMP);
        }
        self.sms1.update(b9.smcx[5], b, &self.recip);
        self.sms2.update(b9.smcx[6], b, &self.recip);
        self.smi1.update(b9.smcx[7], b, &self.recip);
        self.smi2.update(b9.smcx[8], b, &self.recip);
        self.o1s[b9.idx[1]] = NEX[self.o1s[b9.idx[1]] as usize][b as usize];
        self.o2s[b9.idx[2]] = NEX[self.o2s[b9.idx[2]] as usize][b as usize];
        self.o3s[b9.idx[3]] = NEX[self.o3s[b9.idx[3]] as usize][b as usize];
        self.o4s[b9.idx[4]] = NEX[self.o4s[b9.idx[4]] as usize][b as usize];
        self.o6s[b9.idx[5]] = NEX[self.o6s[b9.idx[5]] as usize][b as usize];
        self.sp13s[b9.idx[6]] = NEX[self.sp13s[b9.idx[6]] as usize][b as usize];
        self.sp24s[b9.idx[7]] = NEX[self.sp24s[b9.idx[7]] as usize][b as usize];
        self.ind1s[b9.idx[8]] = NEX[self.ind1s[b9.idx[8]] as usize][b as usize];
        self.ind2s[b9.idx[9]] = NEX[self.ind2s[b9.idx[9]] as usize][b as usize];
        if b9.lat_on {
            self.sml1.update(b9.smcx[9], b, &self.recip);
            self.sml2.update(b9.smcx[10], b, &self.recip);
            self.lat1s[b9.idx[12]] = NEX[self.lat1s[b9.idx[12]] as usize][b as usize];
            self.lat2s[b9.idx[13]] = NEX[self.lat2s[b9.idx[13]] as usize][b as usize];
        }
        if b9.mb_on {
            upd(&mut self.mb[b9.idx[10]], b);
        }
        if b9.mm_on {
            upd(&mut self.mm[b9.idx[11]], b);
        }
        self.apm.learn(b9.apm_base, b9.apm_w, b, &self.recip);
        self.apm2.learn(b9.apm2_base, b9.apm2_w, b, &self.recip);
    }
    pub fn set_lr(&mut self, lr: u32) {
        self.lr = lr;
    }
    /// the site-book prior (v11-M5): weights are scale-free (they multiply a
    /// logit that kept its scale), the StateMap entries are p22 either way --
    /// the same book primes both the v11 and the v12 twin
    pub fn apply_prior(&mut self) {
        if !crate::prior_tab::PRIOR_ON {
            return;
        }
        self.w.copy_from_slice(&crate::prior_tab::PRIOR_W);
        self.h1.copy_from_slice(&crate::prior_tab::PRIOR_H1);
        self.h2.copy_from_slice(&crate::prior_tab::PRIOR_H2);
        self.o1s.copy_from_slice(&crate::prior_tab::PRIOR_O1S);
        self.sm1.seed(&crate::prior_tab::PRIOR_SM1, 64);
    }
    /// v12-M2c(c): a dialect book primes the same tables the site book does
    pub fn apply_book(&mut self, b: &Book) {
        if !b.on {
            return;
        }
        self.w.copy_from_slice(b.w);
        self.h1.copy_from_slice(b.h1);
        self.h2.copy_from_slice(b.h2);
        self.o1s.copy_from_slice(b.o1s);
        self.sm1.seed(b.sm1, 64);
    }
    /// FNV-64 over every counter, weight, APM entry and the match table: the
    /// mirror gate (EGG_STATEHASH)
    pub fn state_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let mut feed = |v: u64| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        };
        for t in [&self.o0, &self.mb, &self.mm] {
            for &c in t.iter() {
                feed(c as u64);
            }
        }
        self.apm.hash_state(&mut feed);
        self.apm2.hash_state(&mut feed);
        for &wv in self.w.iter() {
            feed(wv as u64);
        }
        for t in [&self.o1s, &self.o2s, &self.o3s, &self.o4s, &self.o6s, &self.sp13s, &self.sp24s, &self.ind1s, &self.ind2s, &self.lat1s, &self.lat2s] {
            for &s in t.iter() {
                feed(s as u64);
            }
        }
        self.sm1.hash_state(&mut feed);
        for &wv in self.isse_w.iter() {
            feed(wv as u64);
        }
        self.sms1.hash_state(&mut feed);
        self.sms2.hash_state(&mut feed);
        self.smi1.hash_state(&mut feed);
        self.smi2.hash_state(&mut feed);
        self.sml1.hash_state(&mut feed);
        self.sml2.hash_state(&mut feed);
        feed(self.lat_state as u64);
        feed(self.lat_s as u64);
        for &c in self.lat_cnt.iter() {
            feed(c as u64);
        }
        for &v in self.h1.iter() {
            feed(v as u64);
        }
        for &v in self.h2.iter() {
            feed(v as u64);
        }
        self.mmodel.hash_state(&mut feed);
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mix11::{tail4, tail8};

    /// the generated 16-bit tables: monotone, centred, exact at the tails,
    /// and squash16 inverts stretch16 to within one step
    #[test]
    fn tables_16bit() {
        assert_eq!(SQUASH16.len(), 8191);
        assert_eq!(SQUASH16[4095], 32768);
        assert!(SQUASH16.windows(2).all(|w| w[0] <= w[1]));
        assert!(STRETCH16.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(STRETCH16[32768], 0);
        assert_eq!(STRETCH16[1], -2839);
        assert_eq!(STRETCH16[65535], 2839);
        assert_eq!(sq(STRETCH16[65535] as i32), 65535);
        assert_eq!(sq(STRETCH16[1] as i32), 1);
        for p in [2u16, 100, 1000, 8000, 32768, 50000, 65000, 65530] {
            let back = sq(st(p)) as i32;
            assert!((back - p as i32).abs() <= (p as i32 / 256).max(1), "squash16(stretch16({p})) = {back}");
        }
    }

    /// M2c(b): the checksummed bucket -- a fresh bucket is claimed without
    /// touching anything (byte-identity where nothing collides), a foreign
    /// check reclaims the less-experienced neighbour and zeroes its states,
    /// and an owner finds its bucket again at either of the two ways
    #[test]
    fn claim_semantics() {
        let mut t = vec![0u8; 64];
        // a fresh claim: no state moves, the check is stamped
        assert_eq!(claim(&mut t, 16, 7), 16);
        assert_eq!(t[16], 7);
        assert!(t[17..32].iter().all(|&s| s == 0));
        // the owner comes back
        t[20] = 200; // some history
        assert_eq!(claim(&mut t, 16, 7), 16);
        assert_eq!(t[20], 200);
        // a stranger with an empty neighbour takes the neighbour (seen 0 <= seen 0 -> base first)
        assert_eq!(claim(&mut t, 0, 9), 0);
        assert_eq!(t[0], 9);
        // give both ways history; a third context reclaims the less-experienced one
        t[1] = NEX[NEX[0][0] as usize][0]; // bucket 0: node 1 has seen 2 bits
        t[17] = NEX[NEX[NEX[0][0] as usize][0] as usize][0]; // bucket 16: 3 bits
        assert_eq!(claim(&mut t, 16, 11), 0);
        assert_eq!(t[0], 11);
        assert!(t[1..16].iter().all(|&s| s == 0));
        assert_eq!(t[16], 7);
        assert_eq!(t[20], 200, "the survivor keeps its history");
        // the owner of the other way is found through base ^ 16
        assert_eq!(claim(&mut t, 0, 7), 16);
        // check 0 is never live: it maps to 1
        let mut u = vec![0u8; 32];
        assert_eq!(claim(&mut u, 0, 0), 0);
        assert_eq!(u[0], 1);
    }

    /// the StateMap init polarity at sixteen bits
    #[test]
    fn statemap_polarity_16() {
        let sm = StateMap::new();
        let mut deep0 = 0usize;
        for _ in 0..20 {
            deep0 = NEX[deep0][0] as usize;
        }
        assert!(sm.p(deep0) > 48000, "state-map init polarity broken: {}", sm.p(deep0));
    }

    /// the polarity guard at sixteen bits: an all-zeros stream must drive the
    /// final p toward 65535 (P(bit==0)) -- and past the twelve-bit ceiling
    #[test]
    fn polarity_all_zeros_16() {
        let mut m = Mix12::new();
        let buf = vec![0u8; 4096];
        let mut hist = 0usize;
        for pos in 0..buf.len() {
            let prev4 = tail4(&buf, pos);
            let mm_pred = m.mmodel.predicted(&buf[..pos]);
            for phase in 0..2 {
                let hi = 0u32;
                let ctx18 = m.ctx18(prev4, phase, hi);
                let t8 = tail8(&buf, pos);
                let c3 = m.ctx20(t8, 3, phase, hi);
                let c6 = m.ctx20(t8, 6, phase, hi);
                let mut node = 1usize;
                for i in (0..4u32).rev() {
                    let mm_exp = mm_pred.map(|(pb, ml)| {
                        let pnib = if phase == 0 { pb >> 4 } else { pb & 15 } as u32;
                        ((pnib >> i) & 1, ml)
                    });
                    let s1 = m.ctx_sparse(t8, 0, phase, hi);
                    let s2 = m.ctx_sparse(t8, 1, phase, hi);
                    let ci1 = m.ctx_ind1((t8 & 0xff) as u8, phase, hi);
                    let ci2 = m.ctx_ind2((t8 & 0xffff) as usize, phase, hi);
                    let b9 = m.predict(node, phase, hist, ctx18, c3, c6, s1, s2, ci1, ci2, None, mm_exp, 0);
                    m.learn(&b9, 0);
                    node <<= 1;
                }
                hist = (hist << 4) & 0xffff;
            }
            m.byte_update(&buf, pos + 1);
        }
        let (c18, c3, c6) = (m.ctx18(0, 0, 0), m.ctx20(0, 3, 0, 0), m.ctx20(0, 6, 0, 0));
        let (s1, s2) = (m.ctx_sparse(0, 0, 0, 0), m.ctx_sparse(0, 1, 0, 0));
        let (i1, i2) = (m.ctx_ind1(0, 0, 0), m.ctx_ind2(0, 0, 0));
        let b9 = m.predict(1, 0, 0, c18, c3, c6, s1, s2, i1, i2, None, Some((0, 63)), 0);
        assert!(b9.p > 62400, "polarity broken: p = {} (must approach 65535 = P(bit 0))", b9.p);
        // the point of M2a: the twelve-bit ceiling (4095/4096 = 65520/65536) is passable
        assert!(b9.p > 65520, "sixteen bits did not reach past the twelve-bit ceiling: p = {}", b9.p);
    }
}
