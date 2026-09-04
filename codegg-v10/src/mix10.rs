//! mix10.rs -- the v9 literal model. A CLONE of v8's LitMix that grows;
//! dyadic.rs::LitMix is a frozen trial entrant and is never edited (the
//! never-lose insurance). Mix10 is coder-free: it predicts and learns,
//! dyadic::encode9/decode9 walk the trees and drive the range coder.
//!
//! (v10-M0: byte-copy of mix9.rs, renamed -- mix9.rs is now frozen tier.
//! Everything below grows HERE from v10-M1 on.) v9-M1 added THE REMAINDER READING (stalk.js:429-430: the site computes
//! "the index the repeat starts at" and the exact shortfall of the finite
//! reading; spec.md:153-154: "R ... is exactly what the repeating digits
//! would have been"): a general match model that predicts the next bits as
//! the continuation of the longest previous occurrence -- active on every
//! literal, not only right after an LZ match. Attribution: Matt Mahoney's
//! lpaq match model. And the mixer's weight-vector selection widens to
//! (phase x node) x match-state x after-match x o1-top-3-bits = 1536
//! vectors (the site's chain reading, spec.md:105-106: the output of one
//! reading feeds the next).
//!
//! p is P(bit==0) EVERYWHERE in this codebase; y = 4096 when the bit is 0.
//! lpaq-lineage code is P(1) -- everything ported here is flipped, and the
//! polarity unit test (all-zeros input drives every predictor's p toward
//! 4095) stands guard in tests below.

use crate::squash_tab::SQUASH;
use crate::state_tab::{NEX, NSTATE};

const PBITS: u32 = 12;
const PINIT: u16 = 1 << (PBITS - 1);
const RATE: u32 = 5;
pub const MIX10_LR: u32 = 11; // 11 won the M6 sweep {8..12}; the contested binaries rule (vim's CM10 alone prefers 12)
const WCLAMP: i32 = 1 << 20;
const NINPUT: usize = 10; // o0 chain-final o2-tap sp13s sp24s ind1 ind2 mb mm + bias
// v10-M4, the settle-chain reading (spec.md:124 squash-settle-push;
// spec.md:105-106 each grid read back out as the next step's operand): the
// dense orders o1->o2->o3->o4->o6 become an ISSE chain -- each stage a
// 2-weight learned mixer over the PREVIOUS stage's prediction, selected by
// its own bit-history state. Attribution: Mahoney's zpaq ISSE, restated for
// P(bit==0). The chain replaces five flat inputs with two taps: deeper, not
// wider.
const ISSE_LR: u32 = 10;

#[inline]
fn upd(c: &mut u16, b: u32) {
    if b == 0 {
        *c += ((1 << PBITS) - *c) >> RATE;
    } else {
        *c -= *c >> RATE;
    }
}
/// the keeps-R reading made real (spec.md:204-205: "floating point rounds
/// inside the band and forgets, and this keeps R"): a per-(node x state) map
/// from a bit-history state to a probability, with a COUNT-adaptive step --
/// early observations move it a lot, seasoned ones barely. Attribution:
/// Mahoney's StateMap; the update constant flipped for P(bit==0).
/// Entry: (p22 << 10) | count10. POLARITY: y22 is HIGH on bit 0.
const SM_LIMIT: u32 = 1023; // 1023 won the M6 sweep {127,255,1023}
pub struct StateMap {
    t: Vec<u32>, // node*256 + state
}
impl StateMap {
    fn new() -> Self {
        let mut t = vec![0u32; 16 * 256];
        for node in 0..16 {
            for s in 0..256 {
                // init at the (n0,n1)-implied KT estimate of P(0): zpaq/lpaq
                // put n1 in the numerator because theirs is P(1) -- FLIPPED
                let n0 = NEX[s][2] as u64;
                let n1 = NEX[s][3] as u64;
                let p22 = ((2 * n0 + 1) << 21) / (n0 + n1 + 1);
                t[node * 256 + s] = (p22 as u32) << 10;
            }
        }
        StateMap { t }
    }
    #[inline]
    fn p(&self, cx: usize) -> u16 {
        (((self.t[cx] >> 20) as u16)).clamp(1, 4095)
    }
    #[inline]
    fn update(&mut self, cx: usize, b: u32, recip: &[u16]) {
        let v = self.t[cx];
        let n = (v & 1023) as usize;
        let p22 = (v >> 10) as i64;
        let y22: i64 = if b == 0 { (1 << 22) - 1 } else { 0 };
        let p22 = p22 + (((y22 - p22) * recip[n] as i64) >> 16);
        debug_assert!(p22 >= 0 && p22 < (1 << 22));
        let n2 = (n as u32) + ((n < SM_LIMIT as usize) as u32);
        self.t[cx] = ((p22 as u32) << 10) | n2;
    }
    fn hash_state(&self, feed: &mut dyn FnMut(u64)) {
        for &v in self.t.iter() {
            feed(v as u64);
        }
    }
}

/// the fast twin: same counter, RATE 3 -- remembers the last ~8 outcomes
/// hard and forgets fast, so the mixer can weigh "lately" against "always"
/// (a poor man's bit history; the keeps-R reading, priced honestly)
#[inline]
fn updf(c: &mut u16, b: u32) {
    if b == 0 {
        *c += ((1 << PBITS) - *c) >> 3;
    } else {
        *c -= *c >> 3;
    }
}
#[inline]
pub fn tail8(buf: &[u8], pos: usize) -> u64 {
    let mut v = 0u64;
    for k in 0..8.min(pos) {
        v |= (buf[pos - 1 - k] as u64) << (8 * k);
    }
    v
}
#[inline]
pub fn tail4(buf: &[u8], pos: usize) -> u32 {
    let mut v = 0u32;
    for k in 0..4.min(pos) {
        v |= (buf[pos - 1 - k] as u32) << (8 * k);
    }
    v
}

// ---------------- the match model (the remainder reading) ----------------
pub struct MatchModel {
    ht: Vec<u32>, // 2^22 positions, keyed by a hash of the last 6 bytes
    mptr: usize,  // where the continuation reads from
    mlen: u32,    // agreement run, capped 63
    // instrumentation (integers only): bits offered / bits agreed
    pub offered: u64,
    pub agreed: u64,
}
const HT_BITS: u32 = 22;
impl MatchModel {
    pub fn new() -> Self {
        MatchModel { ht: vec![0u32; 1 << HT_BITS], mptr: 0, mlen: 0, offered: 0, agreed: 0 }
    }
    #[inline]
    fn hash6(buf: &[u8], end: usize) -> usize {
        // key over buf[end-6..end]
        let mut k = 0u64;
        for i in 0..6 {
            k = (k << 8) | buf[end - 6 + i] as u64;
        }
        ((k.wrapping_mul(0x9E3779B97F4A7C15) >> (64 - HT_BITS)) & ((1 << HT_BITS) - 1)) as usize
    }
    /// called after the byte at position `pos_new - 1` is FINAL (encoder
    /// walks src, decoder walks out -- the SAME position discipline; the
    /// EGG_STATEHASH mirror gate proves it per file)
    pub fn update(&mut self, buf: &[u8], pos_new: usize) {
        if self.mlen > 0 {
            if buf[self.mptr] == buf[pos_new - 1] {
                self.mptr += 1;
                if self.mlen < 63 {
                    self.mlen += 1;
                }
            } else {
                self.mlen = 0;
            }
        }
        if pos_new >= 6 {
            let h = Self::hash6(buf, pos_new);
            if self.mlen == 0 {
                let cand = self.ht[h] as usize;
                // adopt only if the 2 trailing bytes verify: a hash collision
                // would otherwise inject a wrong predicted byte (ratio
                // poison, not corruption -- but poison still)
                if cand >= 2
                    && cand < pos_new
                    && buf[cand - 1] == buf[pos_new - 1]
                    && buf[cand - 2] == buf[pos_new - 2]
                {
                    self.mptr = cand;
                    self.mlen = 1;
                }
            }
            self.ht[h] = pos_new as u32;
        }
    }
    /// the predicted next byte and the agreement strength, if any
    #[inline]
    pub fn predicted(&self, buf: &[u8]) -> Option<(u8, u32)> {
        if self.mlen > 0 && self.mptr < buf.len() {
            Some((buf[self.mptr], self.mlen))
        } else {
            None
        }
    }
    pub fn hash_state(&self, feed: &mut dyn FnMut(u64)) {
        for &v in self.ht.iter() {
            feed(v as u64);
        }
        feed(self.mptr as u64);
        feed(self.mlen as u64);
    }
}
#[inline]
fn len_bucket(mlen: u32) -> usize {
    // 1,2,3,4-5,6-7,8-15,16-31,32+ -> 0..7
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
#[inline]
pub fn match_state(mlen: u32) -> usize {
    // weight-selection coarse state: none / short / long
    if mlen == 0 {
        0
    } else if mlen < 8 {
        1
    } else {
        2
    }
}

// ---------------- the mixed model ----------------
pub struct Mix10 {
    o0: Vec<u16>,  // 2 phases x 16 nodes
    o1s: Vec<u8>,  // 256 x 16 BIT-HISTORY STATES (v10-M1)
    o2s: Vec<u8>,  // 65536 x 16 states
    sm1: StateMap, // node x state -> p, count-adaptive
    sm2: StateMap,
    recip: Vec<u16>, // (1<<17)/(2n+3), n in 0..1024 -- integer, built at init
    o3s: Vec<u8>,   // 2^20 x 16 states (hash of last 3 bytes + phase/hi)
    o4s: Vec<u8>,   // 2^18 x 16 states
    o6s: Vec<u8>,   // 2^20 x 16 states
    sp13s: Vec<u8>, // 2^18 x 16 states: sparse (b[-1], b[-3])
    sp24s: Vec<u8>, // 2^18 x 16 states: sparse (b[-2], b[-4])
    sm3: StateMap,
    sm4: StateMap,
    sm6: StateMap,
    sms1: StateMap,
    sms2: StateMap,
    // v10-M3, the dial reading (spectrometer.html:874: "Which rule decides"):
    // the context selects a HISTORY, the history selects the probability.
    // h1[c] = the byte that most recently followed byte value c;
    // h2[pair] = the byte that most recently followed that pair.
    // Attribution: Mahoney's paq indirect models.
    h1: [u8; 256],
    h2: Vec<u8>,     // 65536
    ind1s: Vec<u8>,  // 2^17 x 16 states
    ind2s: Vec<u8>,  // 2^20 x 16 states
    smi1: StateMap,
    smi2: StateMap,
    // ISSE stage weights: 4 stages (o2,o3,o4,o6), each (state x node) pairs
    // of (w_in, w_bias) at 2^16 scale; init pass-through (w_in=1<<16, w_bias=0)
    isse_w: Vec<i32>, // 4 * 4096 * 2
    mb: Vec<u16>,   // v8's after-LZ-match bank (mbit x prevnib x node)
    mm: Vec<u16>,   // match-model bank: len_bucket(8) x expected_bit(2) x node(16)
    w: Vec<i32>,    // 1536 vectors x NINPUT
    stretch: Vec<i16>,
    apm: Vec<u16>,  // stage 1: 256 o1-byte contexts x 33
    apm2: Vec<u16>, // stage 2: (match bucket 9 x phase 2 x prevnib 16) x 33 -- v9-M2
    pub mmodel: MatchModel,
}

/// everything one bit's learn() needs, handed back by predict()
pub struct Bit10 {
    pub p: u16, // the FINAL probability to code with (after both APM stages)
    p_mix: u16, // the mixer's own output (its learning target)
    wsel: usize,
    idx: [usize; 12], // o0 o1s o2s o3s o4s o6s sp13s sp24s ind1s ind2s mb mm
    smcx: [usize; 9], // node*256+state (dense chain states + sparse + indirect)
    isse_wi: [usize; 4], // weight-pair index per chain stage (o2,o3,o4,o6)
    isse_in: [i32; 4],   // each stage's input stretch
    isse_p: [u16; 4],    // each stage's output p (its own learning target)
    sts: [i32; NINPUT],
    mb_on: bool,
    mm_on: bool,
    apm_base: usize,
    apm_w: u32,
    apm2_base: usize,
    apm2_w: u32,
}

impl Mix10 {
    pub fn new() -> Self {
        let mut stretch = vec![0i16; 4096];
        let mut j = 0usize;
        for p in 0..4096usize {
            while j < 4095 && (SQUASH[j] as usize) < p {
                j += 1;
            }
            let x = if j > 0 && (p as i32 - SQUASH[j - 1] as i32) <= (SQUASH[j] as i32 - p as i32) {
                j - 1
            } else {
                j
            };
            stretch[p] = ((x as i32) - 2047).clamp(-2047, 2047) as i16;
        }
        let mut w = vec![0i32; 1536 * NINPUT];
        for v in w.chunks_mut(NINPUT) {
            for k in 0..NINPUT - 1 {
                v[k] = (1 << 16) / (NINPUT as i32 - 1); // even blend; bias 0
            }
        }
        let mut axis = [0u16; 33];
        for (i, a) in axis.iter_mut().enumerate() {
            let st = ((i as i32) - 16) * 128;
            *a = SQUASH[(st + 2047).clamp(0, 4095) as usize];
        }
        let mut apm = vec![0u16; 256 * 33];
        for c in 0..256 {
            apm[c * 33..c * 33 + 33].copy_from_slice(&axis);
        }
        let mut apm2 = vec![0u16; 288 * 33];
        for c in 0..288 {
            apm2[c * 33..c * 33 + 33].copy_from_slice(&axis);
        }
        Mix10 {
            o0: vec![PINIT; 32],
            o1s: vec![0u8; 256 * 16],
            o2s: vec![0u8; 65536 * 16],
            sm1: StateMap::new(),
            sm2: StateMap::new(),
            recip: (0..1024u32).map(|n| ((1u32 << 17) / (2 * n + 3)) as u16).collect(),
            o3s: vec![0u8; (1 << 20) * 16],
            o4s: vec![0u8; (1 << 18) * 16],
            o6s: vec![0u8; (1 << 20) * 16],
            sp13s: vec![0u8; (1 << 18) * 16],
            sp24s: vec![0u8; (1 << 18) * 16],
            sm3: StateMap::new(),
            sm4: StateMap::new(),
            sm6: StateMap::new(),
            sms1: StateMap::new(),
            sms2: StateMap::new(),
            h1: [0u8; 256],
            h2: vec![0u8; 65536],
            ind1s: vec![0u8; (1 << 17) * 16],
            ind2s: vec![0u8; (1 << 20) * 16],
            smi1: StateMap::new(),
            smi2: StateMap::new(),
            isse_w: {
                let mut w = vec![0i32; 4 * 4096 * 2];
                for k in 0..4 * 4096 {
                    w[k * 2] = 1 << 16; // pass-through until each state learns
                }
                w
            },
            mb: vec![PINIT; 512],
            mm: vec![PINIT; 8 * 2 * 16],
            w,
            stretch,
            apm,
            apm2,
            mmodel: MatchModel::new(),
        }
    }
    /// THE one place per-byte model state moves -- called once for EVERY
    /// byte that lands (literals AND match-copied bytes), encoder over src,
    /// decoder over out, same positions. Mirror correct by construction.
    pub fn byte_update(&mut self, buf: &[u8], pos_new: usize) {
        self.mmodel.update(buf, pos_new);
        let b = buf[pos_new - 1];
        if pos_new >= 2 {
            self.h1[buf[pos_new - 2] as usize] = b;
        }
        if pos_new >= 3 {
            self.h2[((buf[pos_new - 3] as usize) << 8) | buf[pos_new - 2] as usize] = b;
        }
    }
    /// indirect context 1: (previous byte, what last followed it)
    #[inline]
    pub fn ctx_ind1(&self, c1: u8, phase: usize, hi: u32) -> usize {
        let key = (c1 as u64)
            | ((self.h1[c1 as usize] as u64) << 8)
            | (0xA1u64 << 56)
            | if phase == 1 { ((0x10 | hi) as u64) << 48 } else { 0 };
        (((key.wrapping_mul(0x9E3779B97F4A7C15) >> 47) as usize) & 0x1ffff) * 16
    }
    /// indirect context 2: (previous pair, what last followed it)
    #[inline]
    pub fn ctx_ind2(&self, pair: usize, phase: usize, hi: u32) -> usize {
        let key = (pair as u64)
            | ((self.h2[pair] as u64) << 16)
            | (0xA2u64 << 56)
            | if phase == 1 { ((0x10 | hi) as u64) << 48 } else { 0 };
        (((key.wrapping_mul(0x9E3779B97F4A7C15) >> 44) as usize) & 0xfffff) * 16
    }
    #[inline]
    pub fn ctx18(&self, prev4: u32, phase: usize, hi: u32) -> usize {
        let key: u64 = if phase == 0 {
            prev4 as u64
        } else {
            prev4 as u64 | ((0x10 | hi) as u64) << 32
        };
        (((key.wrapping_mul(0x9E3779B1) >> 14) as usize) & 0x3ffff) * 16
    }
    /// sparse skip-gram context (paq lineage): two non-adjacent history
    /// bytes -- record-shaped binaries repeat fields the dense orders miss
    #[inline]
    pub fn ctx_sparse(&self, tail: u64, pick: u32, phase: usize, hi: u32) -> usize {
        // pick 0: bytes at -1 and -3; pick 1: bytes at -2 and -4
        let two: u64 = if pick == 0 {
            (tail & 0xff) | (((tail >> 16) & 0xff) << 8)
        } else {
            ((tail >> 8) & 0xff) | (((tail >> 24) & 0xff) << 8)
        };
        let key = two
            | ((pick as u64) << 16)
            | if phase == 1 { ((0x10 | hi) as u64) << 24 } else { 0 };
        (((key.wrapping_mul(0x9E3779B97F4A7C15) >> 46) as usize) & 0x3ffff) * 16
    }
    /// v9-M2, the chain reading: further nesting depths, hashed. tag folds
    /// in the nib phase (and the hi nib on the low phase).
    #[inline]
    pub fn ctx20(&self, tail: u64, nbytes: u32, phase: usize, hi: u32) -> usize {
        let key = (tail & ((1u64 << (8 * nbytes)) - 1))
            ^ ((nbytes as u64) << 56)
            ^ if phase == 1 { ((0x10 | hi) as u64) << 48 } else { 0 };
        (((key.wrapping_mul(0x9E3779B97F4A7C15) >> 44) as usize) & 0xfffff) * 16
    }
    /// one bit: gather the readings, mix, refine -- returns the final p and
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
        steer: Option<u32>,        // v8 Pavlov bank: match bit, while still
        mm_exp: Option<(u32, u32)>, // (expected bit, mlen) while alive
        am: usize,
    ) -> Bit10 {
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
                // hard prior toward the expected bit: st > 0 pushes P(0) up
                let mag = (mlen.min(32) as i32) * 48;
                let prior = if ebit == 0 { mag } else { -mag };
                (true, idx, prior, match_state(mlen))
            }
            None => (false, 0, 0, 0),
        };
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
        ];
        // the settle chain: seed = o1's StateMap; each stage refines the
        // previous prediction through its own state-selected 2-weight mixer
        let mut isse_wi = [0usize; 4];
        let mut isse_in = [0i32; 4];
        let mut isse_p = [0u16; 4];
        let mut p_chain = self.sm1.p(smcx[0]);
        let mut o2_tap = 2048u16;
        for (k, &cx) in smcx[1..5].iter().enumerate() {
            let st_in = self.stretch[p_chain as usize] as i32;
            let wi = (k * 4096 + cx) * 2;
            let t = ((self.isse_w[wi] as i64 * st_in as i64
                + self.isse_w[wi + 1] as i64 * 256)
                >> 16)
                .clamp(-2047, 2047) as i32;
            p_chain = SQUASH[(t + 2047) as usize].clamp(1, 4095);
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
            if mb_on { self.mb[imb] } else { 2048 },
            if mm_on { self.mm[imm] } else { 2048 },
        ];
        let mut sts = [0i32; NINPUT];
        for k in 0..9 {
            sts[k] = self.stretch[ps[k] as usize] as i32;
        }
        if mm_on {
            sts[8] = (sts[8] + mm_prior).clamp(-2047, 2047);
        }
        sts[9] = 256; // bias
        // weight vector: (phase x node)[32] x mstate[3] x am[2] x o1top3[8]
        let o1top3 = (hist >> 5) & 7;
        let wsel = ((((phase * 16 + node) * 3 + mstate) * 2 + am) * 8 + o1top3) * NINPUT;
        let mut t: i64 = 0;
        for k in 0..NINPUT {
            t += self.w[wsel + k] as i64 * sts[k] as i64;
        }
        let t = (t >> 16).clamp(-2047, 2047) as i32;
        let p_mix = SQUASH[(t + 2047) as usize].clamp(1, 4095);
        // APM stage 1 (o1 byte context), as v8
        let actx = hist & 0xff;
        let s = self.stretch[p_mix as usize] as i32 + 2048;
        let pos = (s >> 7) as usize;
        let apm_w = (s & 127) as u32;
        let apm_base = actx * 33 + pos;
        let pa = ((self.apm[apm_base] as u32 * (128 - apm_w)
            + self.apm[apm_base + 1] as u32 * apm_w)
            >> 7) as u16;
        let p1 = (((pa.clamp(1, 4095) as u32) * 3 + p_mix as u32) >> 2).clamp(1, 4095) as u16;
        // APM stage 2 (the settle reading, spec.md:124: squash, settle, push):
        // context = match bucket x phase x previous nib
        let mmb = match mm_exp {
            Some((_, mlen)) => 1 + len_bucket(mlen),
            None => 0,
        };
        let a2ctx = (mmb * 2 + phase) * 16 + prevnib;
        let s2 = self.stretch[p1 as usize] as i32 + 2048;
        let pos2 = (s2 >> 7) as usize;
        let apm2_w = (s2 & 127) as u32;
        let apm2_base = a2ctx * 33 + pos2;
        let pa2 = ((self.apm2[apm2_base] as u32 * (128 - apm2_w)
            + self.apm2[apm2_base + 1] as u32 * apm2_w)
            >> 7) as u16;
        let p = (((pa2.clamp(1, 4095) as u32) * 3 + p1 as u32) >> 2).clamp(1, 4095) as u16;
        Bit10 {
            p,
            p_mix,
            wsel,
            idx: [i0, i1, i2, i3, i4, i6, is1, is2, ii1, ii2, imb, imm],
            smcx,
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
    pub fn learn(&mut self, b9: &Bit10, b: u32) {
        let y: i32 = if b == 0 { 4096 } else { 0 };
        let err = y - b9.p_mix as i32;
        for k in 0..NINPUT {
            let w = &mut self.w[b9.wsel + k];
            *w = (*w + ((err * b9.sts[k]) >> MIX10_LR)).clamp(-WCLAMP, WCLAMP);
        }
        upd(&mut self.o0[b9.idx[0]], b);
        self.sm1.update(b9.smcx[0], b, &self.recip);
        // chain stages: each learns against its own output (same polarity law)
        let y: i32 = if b == 0 { 4096 } else { 0 };
        for k in 0..4 {
            let err = y - b9.isse_p[k] as i32;
            let wi = b9.isse_wi[k];
            self.isse_w[wi] =
                (self.isse_w[wi] + ((err * b9.isse_in[k]) >> ISSE_LR)).clamp(-WCLAMP, WCLAMP);
            self.isse_w[wi + 1] =
                (self.isse_w[wi + 1] + ((err * 256) >> ISSE_LR)).clamp(-WCLAMP, WCLAMP);
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
        if b9.mb_on {
            upd(&mut self.mb[b9.idx[10]], b);
        }
        if b9.mm_on {
            upd(&mut self.mm[b9.idx[11]], b);
        }
        for (i, share) in [(b9.apm_base, 128 - b9.apm_w), (b9.apm_base + 1, b9.apm_w)] {
            if share == 0 {
                continue;
            }
            let c = &mut self.apm[i];
            if b == 0 {
                *c += ((1 << PBITS) - *c) >> 7;
            } else {
                *c -= *c >> 7;
            }
        }
        for (i, share) in [(b9.apm2_base, 128 - b9.apm2_w), (b9.apm2_base + 1, b9.apm2_w)] {
            if share == 0 {
                continue;
            }
            let c = &mut self.apm2[i];
            if b == 0 {
                *c += ((1 << PBITS) - *c) >> 7;
            } else {
                *c -= *c >> 7;
            }
        }
    }
    /// FNV-64 over every counter, weight and the match table: the mirror gate
    pub fn state_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let mut feed = |v: u64| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        };
        for t in [&self.o0, &self.mb, &self.mm, &self.apm, &self.apm2] {
            for &c in t.iter() {
                feed(c as u64);
            }
        }
        for &wv in self.w.iter() {
            feed(wv as u64);
        }
        for t in [&self.o1s, &self.o2s, &self.o3s, &self.o4s, &self.o6s, &self.sp13s, &self.sp24s, &self.ind1s, &self.ind2s] {
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

    /// the generated table's invariants, re-verified Rust-side (CI cannot
    /// run the python generator): targets in range, the all-zeros walk keeps
    /// n1 == 0 with n0 growing -- the polarity tripwire at the table level
    #[test]
    fn state_tab_invariants() {
        for s in 0..NSTATE {
            assert!((NEX[s][0] as usize) < NSTATE && (NEX[s][1] as usize) < NSTATE);
        }
        let mut s = 0usize;
        for _ in 0..20 {
            s = NEX[s][0] as usize;
            assert_eq!(NEX[s][3], 0, "all-zeros walk grew n1: state {}", s);
            assert!(NEX[s][2] >= 1);
        }
        // and the StateMap init polarity: heavy-n0 states must map high
        let sm = StateMap::new();
        let mut deep0 = 0usize;
        for _ in 0..20 {
            deep0 = NEX[deep0][0] as usize;
        }
        assert!(sm.p(deep0) > 3000, "state-map init polarity broken: {}", sm.p(deep0));
    }

    /// the polarity guard: p is P(bit==0); an all-zeros stream must drive
    /// every reading's p toward 4095. A ported P(1)-lineage table would
    /// anti-learn silently -- this test is the tripwire.
    #[test]
    fn polarity_all_zeros() {
        let mut m = Mix10::new();
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
                    node = node << 1; // bit 0
                }
                let _ = node;
                hist = (hist << 4) & 0xffff;
            }
            m.byte_update(&buf, pos + 1);
        }
        // after 4 KB of zeros every live reading must be near-certain of 0
        let b9 = m.predict(1, 0, 0, m.ctx18(0, 0, 0), m.ctx20(0, 3, 0, 0), m.ctx20(0, 6, 0, 0), m.ctx_sparse(0, 0, 0, 0), m.ctx_sparse(0, 1, 0, 0), m.ctx_ind1(0, 0, 0), m.ctx_ind2(0, 0, 0), None, Some((0, 63)), 0);
        assert!(b9.p > 3900, "polarity broken: p = {} (must approach 4095 = P(bit 0))", b9.p);
    }

    /// the match model must find and follow a repeat, and its instrumentation
    /// must see the agreement
    #[test]
    fn match_model_follows_repeats() {
        let mut mm = MatchModel::new();
        let pattern = b"the quick brown fox jumps over the lazy dog. ";
        let mut buf = Vec::new();
        for _ in 0..8 {
            buf.extend_from_slice(pattern);
        }
        let mut predicted_right = 0usize;
        let mut offered = 0usize;
        for pos in 0..buf.len() {
            if let Some((pb, _)) = mm.predicted(&buf[..pos]) {
                offered += 1;
                if pb == buf[pos] {
                    predicted_right += 1;
                }
            }
            mm.update(&buf, pos + 1);
        }
        assert!(offered > buf.len() / 2, "match model never engaged: {}", offered);
        assert!(
            predicted_right * 10 > offered * 9,
            "match model wrong too often: {}/{}",
            predicted_right,
            offered
        );
    }
}
