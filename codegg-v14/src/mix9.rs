//! mix9.rs -- the v9 literal model. A CLONE of v8's LitMix that grows;
//! dyadic.rs::LitMix is a frozen trial entrant and is never edited (the
//! never-lose insurance). Mix9 is coder-free: it predicts and learns,
//! dyadic::encode9/decode9 walk the trees and drive the range coder.
//!
//! v9-M1 adds THE REMAINDER READING (stalk.js:429-430: the site computes
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

// FROZEN TIER (v9). Behavior must not move; style rewrites are barred by the freeze.
#![allow(dead_code, clippy::needless_range_loop, clippy::assign_op_pattern, clippy::manual_range_contains)]
use crate::squash_tab::SQUASH;

const PBITS: u32 = 12;
const PINIT: u16 = 1 << (PBITS - 1);
const RATE: u32 = 5;
pub const MIX9_LR: u32 = 9; // 9 won both sweeps ({8,9,10}, at 9 and at 14 inputs)
const WCLAMP: i32 = 1 << 20;
const NINPUT: usize = 14; // o0 o1 o2 o3 o4 o6 sp13 sp24 o1f o2f o4f mb mm + bias (f: fast-rate twins)

#[inline]
fn upd(c: &mut u16, b: u32) {
    if b == 0 {
        *c += ((1 << PBITS) - *c) >> RATE;
    } else {
        *c -= *c >> RATE;
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
pub struct Mix9 {
    o0: Vec<u16>,   // 2 phases x 16 nodes
    o1: Vec<u16>,   // 256 x 16
    o2: Vec<u16>,   // 65536 x 16
    o3: Vec<u16>,   // 2^20 x 16 (hash of last 3 bytes + phase/hi) -- v9-M2
    o4: Vec<u16>,   // 2^18 x 16 (hash of last 4 bytes + phase/hi)
    o6: Vec<u16>,   // 2^20 x 16 (hash of last 6 bytes + phase/hi) -- v9-M2
    sp13: Vec<u16>, // 2^18 x 16: sparse (b[-1], b[-3]) skip-gram -- try-again
    sp24: Vec<u16>, // 2^18 x 16: sparse (b[-2], b[-4]) skip-gram -- try-again
    o1f: Vec<u16>,  // fast-rate twins of o1/o2/o4 -- try-again
    o2f: Vec<u16>,
    o4f: Vec<u16>,
    mb: Vec<u16>,   // v8's after-LZ-match bank (mbit x prevnib x node)
    mm: Vec<u16>,   // match-model bank: len_bucket(8) x expected_bit(2) x node(16)
    w: Vec<i32>,    // 1536 vectors x NINPUT
    stretch: Vec<i16>,
    apm: Vec<u16>,  // stage 1: 256 o1-byte contexts x 33
    apm2: Vec<u16>, // stage 2: (match bucket 9 x phase 2 x prevnib 16) x 33 -- v9-M2
    pub mmodel: MatchModel,
}

/// everything one bit's learn() needs, handed back by predict()
pub struct Bit9 {
    pub p: u16, // the FINAL probability to code with (after both APM stages)
    p_mix: u16, // the mixer's own output (its learning target)
    wsel: usize,
    idx: [usize; 10], // o0 o1 o2 o3 o4 o6 sp13 sp24 mb mm counter indices
    sts: [i32; NINPUT],
    mb_on: bool,
    mm_on: bool,
    apm_base: usize,
    apm_w: u32,
    apm2_base: usize,
    apm2_w: u32,
}

impl Mix9 {
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
        Mix9 {
            o0: vec![PINIT; 32],
            o1: vec![PINIT; 256 * 16],
            o2: vec![PINIT; 65536 * 16],
            o3: vec![PINIT; (1 << 20) * 16],
            o4: vec![PINIT; (1 << 18) * 16],
            o6: vec![PINIT; (1 << 20) * 16],
            sp13: vec![PINIT; (1 << 18) * 16],
            sp24: vec![PINIT; (1 << 18) * 16],
            o1f: vec![PINIT; 256 * 16],
            o2f: vec![PINIT; 65536 * 16],
            o4f: vec![PINIT; (1 << 18) * 16],
            mb: vec![PINIT; 512],
            mm: vec![PINIT; 8 * 2 * 16],
            w,
            stretch,
            apm,
            apm2,
            mmodel: MatchModel::new(),
        }
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
        steer: Option<u32>,        // v8 Pavlov bank: match bit, while still
        mm_exp: Option<(u32, u32)>, // (expected bit, mlen) while alive
        am: usize,
    ) -> Bit9 {
        let prevnib = hist & 15;
        let i0 = phase * 16 + node;
        let i1 = (hist & 0xff) * 16 + node;
        let i2 = (hist & 0xffff) * 16 + node;
        let i3 = ctx3 + node;
        let i4 = ctx18 + node;
        let i6 = ctx6 + node;
        let is1 = cs13 + node;
        let is2 = cs24 + node;
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
        let ps = [
            self.o0[i0],
            self.o1[i1],
            self.o2[i2],
            self.o3[i3],
            self.o4[i4],
            self.o6[i6],
            self.sp13[is1],
            self.sp24[is2],
            self.o1f[i1],
            self.o2f[i2],
            self.o4f[i4],
            if mb_on { self.mb[imb] } else { 2048 },
            if mm_on { self.mm[imm] } else { 2048 },
        ];
        let mut sts = [0i32; NINPUT];
        for k in 0..13 {
            sts[k] = self.stretch[ps[k] as usize] as i32;
        }
        if mm_on {
            sts[12] = (sts[12] + mm_prior).clamp(-2047, 2047);
        }
        sts[13] = 256; // bias
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
        Bit9 {
            p,
            p_mix,
            wsel,
            idx: [i0, i1, i2, i3, i4, i6, is1, is2, imb, imm],
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
    pub fn learn(&mut self, b9: &Bit9, b: u32) {
        let y: i32 = if b == 0 { 4096 } else { 0 };
        let err = y - b9.p_mix as i32;
        for k in 0..NINPUT {
            let w = &mut self.w[b9.wsel + k];
            *w = (*w + ((err * b9.sts[k]) >> MIX9_LR)).clamp(-WCLAMP, WCLAMP);
        }
        upd(&mut self.o0[b9.idx[0]], b);
        upd(&mut self.o1[b9.idx[1]], b);
        upd(&mut self.o2[b9.idx[2]], b);
        upd(&mut self.o3[b9.idx[3]], b);
        upd(&mut self.o4[b9.idx[4]], b);
        upd(&mut self.o6[b9.idx[5]], b);
        upd(&mut self.sp13[b9.idx[6]], b);
        upd(&mut self.sp24[b9.idx[7]], b);
        updf(&mut self.o1f[b9.idx[1]], b);
        updf(&mut self.o2f[b9.idx[2]], b);
        updf(&mut self.o4f[b9.idx[4]], b);
        if b9.mb_on {
            upd(&mut self.mb[b9.idx[8]], b);
        }
        if b9.mm_on {
            upd(&mut self.mm[b9.idx[9]], b);
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
        for t in [&self.o0, &self.o1, &self.o2, &self.o3, &self.o4, &self.o6, &self.sp13, &self.sp24, &self.o1f, &self.o2f, &self.o4f, &self.mb, &self.mm, &self.apm, &self.apm2] {
            for &c in t.iter() {
                feed(c as u64);
            }
        }
        for &wv in self.w.iter() {
            feed(wv as u64);
        }
        self.mmodel.hash_state(&mut feed);
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the polarity guard: p is P(bit==0); an all-zeros stream must drive
    /// every reading's p toward 4095. A ported P(1)-lineage table would
    /// anti-learn silently -- this test is the tripwire.
    #[test]
    fn polarity_all_zeros() {
        let mut m = Mix9::new();
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
                    let b9 = m.predict(node, phase, hist, ctx18, c3, c6, s1, s2, None, mm_exp, 0);
                    m.learn(&b9, 0);
                    node = node << 1; // bit 0
                }
                let _ = node;
                hist = (hist << 4) & 0xffff;
            }
            m.mmodel.update(&buf, pos + 1);
        }
        // after 4 KB of zeros every live reading must be near-certain of 0
        let b9 = m.predict(1, 0, 0, m.ctx18(0, 0, 0), m.ctx20(0, 3, 0, 0), m.ctx20(0, 6, 0, 0), m.ctx_sparse(0, 0, 0, 0), m.ctx_sparse(0, 1, 0, 0), None, Some((0, 63)), 0);
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
