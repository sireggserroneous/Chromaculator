//! dyadic.rs -- the hidden transmutation, the centerpiece.
//!
//! The Atlas page draws the Poincare disc of the DYADIC TREE; every value on
//! the site is a dyadic rational in the unit interval. An arithmetic coder
//! is a machine that walks down that tree and emits the address of the
//! interval where the message lands -- the site's numbers ARE arithmetic
//! coder outputs. So the whole token stream becomes ONE dyadic rational:
//! a single point on the site's own disc. Structured files land in wide
//! intervals (short addresses); random files land in intervals as narrow as
//! themselves (the pigeonhole, kept -- the suite asserts >100% as a PASS).
//!
//! Mechanism: carry-handling binary range coder, 32-bit, classical
//! (Elias -> Rissanen -> Witten-Neal-Cleary; this implementation follows
//! the shape Igor Pavlov fixed in LZMA, including the slot/align coding of
//! offsets and the rep-offset). Models, all adaptive 12-bit counters:
//!   - literal nibs: context = previous 2 OR 4 nibs of the restored stream
//!     (the transmuter codes both depths and keeps the lighter point; the
//!     model byte remembers which), each nib as 4 binary decisions down a
//!     15-node tree, with the tree steered by the match byte for the first
//!     literal after a match (Pavlov's trick). The nib is the site's atom.
//!   - token type (literal vs match): context = last three token types.
//!   - rep flag + 4-deep offset history: a recent stride is ~3 bits to name.
//!   - match length / offset: log2-bucketed adaptive models; buckets up to
//!     64 wide get adaptive trees, wider get raw bits (+ 4 align bits).

use crate::squash_tab::SQUASH;
use crate::token::{rep_update, Tok, MIN_MATCH};

const PBITS: u32 = 12; // probability precision
const PINIT: u16 = 1 << (PBITS - 1);
const RATE: u32 = 5; // adaptation shift
const TOP: u32 = 1 << 24;

// ---------------- stats: exact prices, -log2(p) per decision ----------------
pub const CAT_TYP: usize = 0;
pub const CAT_LIT: usize = 1;
pub const CAT_REP: usize = 2;
pub const CAT_LEN: usize = 3;
pub const CAT_SLOT: usize = 4;
pub const CAT_EXTRA: usize = 5;
pub const CAT_NAMES: [&str; 6] = ["type", "literals", "rep-flag", "length", "dist-slot", "dist-extra"];
#[derive(Default)]
pub struct Stats {
    pub bits: [f64; 6],
    pub lits: u64,
    pub matches: u64,
    pub reps: u64,
    pub match_bytes: u64,
    pub slot_hist: [u64; 32],
}

// ---------------- range encoder (carry-handling, LZMA shape) ----------------
struct REnc {
    low: u64,
    range: u32,
    cache: u8,
    cache_n: u64,
    out: Vec<u8>,
    stats: Option<Box<Stats>>,
    /// v12-M2a: the 16-bit coder. `bound = (range * p) >> 16` in u64 with p
    /// in 1..65535 (Mahoney zpaq, Knoll cmix); the 12-bit token counters of
    /// a wide arm enter the same coder as p << 4. false = v8..v11's coder,
    /// untouched (the frozen arms must write byte-identical eggs).
    wide: bool,
}
impl REnc {
    fn new(want_stats: bool) -> Self {
        REnc {
            low: 0,
            range: !0u32,
            cache: 0,
            cache_n: 1,
            out: Vec::new(),
            stats: if want_stats { Some(Box::default()) } else { None },
            wide: false,
        }
    }
    fn new_wide() -> Self {
        REnc { wide: true, ..Self::new(false) }
    }
    /// code a bit at a GIVEN 16-bit probability (P(0) in 1..65535); no
    /// counter update -- the mixer owns its own learning. Wide arms only.
    #[inline]
    fn bit_p16(&mut self, p: u16, b: u32, cat: usize) {
        debug_assert!(self.wide && p != 0);
        if let Some(s) = self.stats.as_deref_mut() {
            let pf = p as f64 / 65536.0;
            s.bits[cat] -= if b == 0 { pf.log2() } else { (1.0 - pf).log2() };
        }
        let bound = ((self.range as u64 * p as u64) >> 16) as u32;
        if b == 0 {
            self.range = bound;
        } else {
            self.low += bound as u64;
            self.range -= bound;
        }
        while self.range < TOP {
            self.range <<= 8;
            self.shift_low();
        }
    }
    #[inline]
    fn shift_low(&mut self) {
        if self.low < 0xFF00_0000 || self.low > 0xFFFF_FFFF {
            let carry = (self.low >> 32) as u8;
            let mut c = self.cache;
            while self.cache_n > 0 {
                self.out.push(c.wrapping_add(carry));
                c = 0xFF;
                self.cache_n -= 1;
            }
            self.cache = (self.low >> 24) as u8;
        }
        self.cache_n += 1;
        self.low = (self.low << 8) & 0xFFFF_FFFF;
    }
    #[inline]
    fn bit(&mut self, p: &mut u16, b: u32, cat: usize) {
        if let Some(s) = self.stats.as_deref_mut() {
            let pf = *p as f64 / 4096.0;
            s.bits[cat] -= if b == 0 { pf.log2() } else { (1.0 - pf).log2() };
        }
        let bound = if self.wide {
            ((self.range as u64 * ((*p as u64) << 4)) >> 16) as u32
        } else {
            (self.range >> PBITS) * (*p as u32)
        };
        if b == 0 {
            self.range = bound;
            *p += ((1 << PBITS) - *p) >> RATE;
        } else {
            self.low += bound as u64;
            self.range -= bound;
            *p -= *p >> RATE;
        }
        while self.range < TOP {
            self.range <<= 8;
            self.shift_low();
        }
    }
    #[inline]
    fn direct(&mut self, v: u32, n: u32, cat: usize) {
        if let Some(s) = self.stats.as_deref_mut() {
            s.bits[cat] += n as f64;
        }
        for i in (0..n).rev() {
            self.range >>= 1;
            if (v >> i) & 1 == 1 {
                self.low += self.range as u64;
            }
            while self.range < TOP {
                self.range <<= 8;
                self.shift_low();
            }
        }
    }
    /// code a bit at a GIVEN probability (12-bit P(0)); no counter update --
    /// the mixer owns its own learning
    #[inline]
    fn bit_p(&mut self, p: u16, b: u32, cat: usize) {
        if let Some(s) = self.stats.as_deref_mut() {
            let pf = p as f64 / 4096.0;
            s.bits[cat] -= if b == 0 { pf.log2() } else { (1.0 - pf).log2() };
        }
        let bound = (self.range >> PBITS) * (p as u32);
        if b == 0 {
            self.range = bound;
        } else {
            self.low += bound as u64;
            self.range -= bound;
        }
        while self.range < TOP {
            self.range <<= 8;
            self.shift_low();
        }
    }
    fn flush(mut self) -> (Vec<u8>, Option<Box<Stats>>) {
        for _ in 0..5 {
            self.shift_low();
        }
        (self.out, self.stats)
    }
}

// ---------------- range decoder ----------------
struct RDec<'a> {
    range: u32,
    code: u32,
    inp: &'a [u8],
    pos: usize,
    wide: bool, // the mirror of REnc::wide, set by the arm's constructor
}
impl<'a> RDec<'a> {
    fn new(inp: &'a [u8]) -> Self {
        let mut d = RDec { range: !0u32, code: 0, inp, pos: 0, wide: false };
        for _ in 0..5 {
            let b = d.byte();
            d.code = (d.code << 8) | b;
        }
        d
    }
    fn new_wide(inp: &'a [u8]) -> Self {
        RDec { wide: true, ..Self::new(inp) }
    }
    #[inline]
    fn bit_p16(&mut self, p: u16) -> u32 {
        debug_assert!(self.wide && p != 0);
        let bound = ((self.range as u64 * p as u64) >> 16) as u32;
        let b = if self.code < bound {
            self.range = bound;
            0
        } else {
            self.code -= bound;
            self.range -= bound;
            1
        };
        while self.range < TOP {
            self.range <<= 8;
            let nb = self.byte();
            self.code = (self.code << 8) | nb;
        }
        b
    }
    #[inline]
    fn byte(&mut self) -> u32 {
        // running off the end decodes zeros; the conservation hash convicts it
        let b = if self.pos < self.inp.len() { self.inp[self.pos] } else { 0 };
        self.pos += 1;
        b as u32
    }
    #[inline]
    fn bit(&mut self, p: &mut u16) -> u32 {
        let bound = if self.wide {
            ((self.range as u64 * ((*p as u64) << 4)) >> 16) as u32
        } else {
            (self.range >> PBITS) * (*p as u32)
        };
        let b = if self.code < bound {
            self.range = bound;
            *p += ((1 << PBITS) - *p) >> RATE;
            0
        } else {
            self.code -= bound;
            self.range -= bound;
            *p -= *p >> RATE;
            1
        };
        while self.range < TOP {
            self.range <<= 8;
            let nb = self.byte();
            self.code = (self.code << 8) | nb;
        }
        b
    }
    #[inline]
    fn bit_p(&mut self, p: u16) -> u32 {
        let bound = (self.range >> PBITS) * (p as u32);
        let b = if self.code < bound {
            self.range = bound;
            0
        } else {
            self.code -= bound;
            self.range -= bound;
            1
        };
        while self.range < TOP {
            self.range <<= 8;
            let nb = self.byte();
            self.code = (self.code << 8) | nb;
        }
        b
    }
    #[inline]
    fn direct(&mut self, n: u32) -> u32 {
        let mut v = 0u32;
        for _ in 0..n {
            self.range >>= 1;
            let b = if self.code >= self.range {
                self.code -= self.range;
                1
            } else {
                0
            };
            v = (v << 1) | b;
            while self.range < TOP {
                self.range <<= 8;
                let nb = self.byte();
                self.code = (self.code << 8) | nb;
            }
        }
        v
    }
}

// ---------------- the 16-bit coder, for models that are not byte models ----------------
// v13-M1: the JPEG coefficient model codes coefficients, not nibbles, so it
// needs the coder and nothing else -- one bit at a probability it owns. p is
// P(bit == 0) in 1..65535, as everywhere in this house.

pub struct WEnc(REnc);
impl WEnc {
    pub fn new() -> WEnc {
        WEnc(REnc::new_wide())
    }
    #[inline]
    pub fn bit(&mut self, p: u16, b: u32) {
        self.0.bit_p16(p.max(1), b, CAT_LIT);
    }
    pub fn finish(self) -> Vec<u8> {
        self.0.flush().0
    }
}
impl Default for WEnc {
    fn default() -> Self {
        WEnc::new()
    }
}
pub struct WDec<'a>(RDec<'a>);
impl<'a> WDec<'a> {
    pub fn new(inp: &'a [u8]) -> WDec<'a> {
        WDec(RDec::new_wide(inp))
    }
    #[inline]
    pub fn bit(&mut self, p: u16) -> u32 {
        self.0.bit_p16(p.max(1))
    }
}

// ---------------- adaptive trees ----------------
#[inline]
fn enc_tree(rc: &mut REnc, tree: &mut [u16], nbits: u32, v: u32, cat: usize) {
    let mut node = 1usize;
    for i in (0..nbits).rev() {
        let b = (v >> i) & 1;
        rc.bit(&mut tree[node], b, cat);
        node = (node << 1) | b as usize;
    }
}
#[inline]
fn dec_tree(rd: &mut RDec, tree: &mut [u16], nbits: u32) -> u32 {
    let mut node = 1usize;
    for _ in 0..nbits {
        let b = rd.bit(&mut tree[node]);
        node = (node << 1) | b as usize;
    }
    (node as u32) - (1 << nbits)
}

// log2 bucket: value+1 has bit-length b+1, i.e. value+1 in [2^b, 2^(b+1))
#[inline]
fn bucket_of(v1: u32) -> u32 {
    31 - v1.leading_zeros()
}

// ---------------- length model: unary buckets + adaptive trees ----------------
// extras were raw bits above bucket 3; measured on big.xml the match lengths
// cluster hard inside wide buckets (element strides), so buckets up to 6 get
// adaptive trees too -- the "bucket shapes" lever the plan reserved.
const LEN_BUCKETS: usize = 26;
struct LenModel {
    unary: [u16; LEN_BUCKETS],
    lo: [u16; 126], // trees for buckets 1..6: 2+4+8+16+32+64
}
const LEN_LO_OFF: [usize; 7] = [0, 0, 2, 6, 14, 30, 62];
impl LenModel {
    fn new() -> Self {
        LenModel { unary: [PINIT; LEN_BUCKETS], lo: [PINIT; 126] }
    }
    fn encode(&mut self, rc: &mut REnc, v: u32) {
        let b = bucket_of(v + 1) as usize;
        for i in 0..b {
            rc.bit(&mut self.unary[i], 1, CAT_LEN);
        }
        rc.bit(&mut self.unary[b], 0, CAT_LEN);
        let r = (v + 1) - (1 << b);
        if b == 0 {
        } else if b <= 6 {
            enc_tree(rc, &mut self.lo[LEN_LO_OFF[b]..LEN_LO_OFF[b] + (1 << b)], b as u32, r, CAT_LEN);
        } else {
            rc.direct(r, b as u32, CAT_LEN);
        }
    }
    fn decode(&mut self, rd: &mut RDec) -> Result<u32, String> {
        let mut b = 0usize;
        while rd.bit(&mut self.unary[b]) == 1 {
            b += 1;
            if b >= LEN_BUCKETS {
                return Err("length bucket ran away".into());
            }
        }
        let r = if b == 0 {
            0
        } else if b <= 6 {
            dec_tree(rd, &mut self.lo[LEN_LO_OFF[b]..LEN_LO_OFF[b] + (1 << b)], b as u32)
        } else {
            rd.direct(b as u32)
        };
        Ok((1u32 << b) + r - 1)
    }
}

// ---------------- the models ----------------
const DIST_LO_OFF: [usize; 5] = [0, 0, 2, 6, 14];
struct Models {
    typ: [u16; 8],             // literal vs match; ctx = last three token types
    is_rep: [u16; 4],          // ctx = last two token types
    rep_idx: [u16; 4],         // which of the four recent offsets (2-bit tree)
    // (1<<lit_bits) nib contexts x 48 nodes: bank 0 = plain tree; banks 1..2 =
    // the tree steered by the match byte (Pavlov's trick: the first literal
    // after a match is what the match source FAILED to predict -- code it
    // against the byte the source would have continued with)
    lit: Vec<u16>,
    lit_mask: usize,           // lit context: 8 bits (2 nibs) or 16 (4 nibs)
    len_m: [LenModel; 2],      // ctx = previous token type
    len_r: [LenModel; 2],
    dist_slot: [[u16; 32]; 4], // 5-bit slot tree; ctx = match length class
    dist_lo: [u16; 30],        // adaptive trees for slots 1..4 (2+4+8+16)
    align: [u16; 16],          // low 4 bits of wide offsets
}
impl Models {
    fn new(lit_bits: u32) -> Self {
        Models {
            typ: [PINIT; 8],
            is_rep: [PINIT; 4],
            rep_idx: [PINIT; 4],
            lit: vec![PINIT; (1usize << lit_bits) * 48],
            lit_mask: (1usize << lit_bits) - 1,
            len_m: [LenModel::new(), LenModel::new()],
            len_r: [LenModel::new(), LenModel::new()],
            dist_slot: [[PINIT; 32]; 4],
            dist_lo: [PINIT; 30],
            align: [PINIT; 16],
        }
    }
}

// one nib through the literal tree; while the decoded prefix still tracks the
// match byte's prefix, the tree node is picked from the bank the match bit
// names -- diverge once and it falls back to the plain bank
#[inline]
fn enc_nib(rc: &mut REnc, tree48: &mut [u16], nib: u32, mnib: u32, still: &mut bool) {
    let mut node = 1usize;
    for i in (0..4).rev() {
        let b = (nib >> i) & 1;
        let idx = if *still {
            let mbit = (mnib >> i) & 1;
            16 * (1 + mbit as usize) + node
        } else {
            node
        };
        rc.bit(&mut tree48[idx], b, CAT_LIT);
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
}
#[inline]
fn dec_nib(rd: &mut RDec, tree48: &mut [u16], mnib: u32, still: &mut bool) -> u32 {
    let mut node = 1usize;
    for i in (0..4).rev() {
        let idx = if *still {
            let mbit = (mnib >> i) & 1;
            16 * (1 + mbit as usize) + node
        } else {
            node
        };
        let b = rd.bit(&mut tree48[idx]);
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
    (node as u32) - 16
}

// ---------------- the mixed literal model (the Spectrometer, part two) ----------------
// spectrometer.html:7: "One integer in full -- its stalk, its square, its
// three regions, its value as a light wave"; :67: "the stalk, with its four
// readings nested underneath". v7 chose ONE reading per file (8- or 16-bit
// literal context, the lighter kept); v8 reads ALL the depths at once, each
// weighted by how well it has been predicting -- adaptive logistic mixing on
// integers only. Attribution: Matt Mahoney's PAQ/lpaq lineage.
//
// Five predictors per literal-tree bit (node 1..15, shared):
//   o0  node + nib phase                                   (32 counters)
//   o1  8-bit context (last 2 nibs)  x 16 nodes            (8 KB)
//   o2  16-bit context (last 4 nibs) x 16 nodes            (2 MB)
//   o4  hash of last 4 bytes (+ phase/hi nib) -> 18 bits   (8 MB)
//   mb  match bank: (match bit x previous nib) x node      (512), active
//       while the literal still tracks the match byte's prefix
// All counters are today's 12-bit kind with today's update. The mixer is 60
// weight vectors -- (nib phase) x (node) x (after-match) -- of 6 i32 weights
// at 2^16 scale (5 predictors + bias). p is P(bit==0) EVERYWHERE, as in the
// whole codebase: y = 4096 when the bit is 0.
const MIX_LR: u32 = 9; // weight learning shift, 9 won the M3 sweep over {9,10,11}
const MIX_WCLAMP: i32 = 1 << 20;

/// the APM / SSE stage (gated M4 extra): a per-context map from the mixer's
/// own probability to the OBSERVED probability, 33 interpolated buckets over
/// the stretch axis per 8-bit byte context; the coded p is (3 apm + mix)/4.
/// Attribution: Mahoney's APM (paq6+), Shkarin's SSE before it.
struct Apm {
    t: Vec<u16>,
}
impl Apm {
    fn new(stretch_axis_init: &[u16]) -> Self {
        let mut t = vec![0u16; 256 * 33];
        for c in 0..256 {
            t[c * 33..c * 33 + 33].copy_from_slice(stretch_axis_init);
        }
        Apm { t }
    }
    #[inline]
    fn refine(&self, ctx: usize, st: i32) -> (u16, usize, u32) {
        let s = st + 2048; // 1..=4095
        let pos = (s >> 7) as usize;
        let wgt = (s & 127) as u32;
        let base = ctx * 33 + pos;
        let pa = ((self.t[base] as u32 * (128 - wgt) + self.t[base + 1] as u32 * wgt) >> 7) as u16;
        (pa.clamp(1, 4095), base, wgt)
    }
    #[inline]
    fn learn(&mut self, base: usize, wgt: u32, b: u32) {
        // update both bucket ends toward the outcome, nearer end faster
        for (i, share) in [(base, 128 - wgt), (base + 1, wgt)] {
            if share == 0 {
                continue;
            }
            let c = &mut self.t[i];
            if b == 0 {
                *c += ((1 << PBITS) - *c) >> 7;
            } else {
                *c -= *c >> 7;
            }
        }
    }
}

pub struct LitMix {
    o0: Vec<u16>,
    o1: Vec<u16>,
    o2: Vec<u16>,
    o4: Vec<u16>,
    mb: Vec<u16>,
    w: Vec<i32>,
    stretch: Vec<i16>,
    apm: Apm,
}
#[inline]
fn upd(c: &mut u16, b: u32) {
    if b == 0 {
        *c += ((1 << PBITS) - *c) >> RATE;
    } else {
        *c -= *c >> RATE;
    }
}
#[inline]
fn tail4(buf: &[u8], pos: usize) -> u32 {
    let mut v = 0u32;
    for k in 0..4.min(pos) {
        v |= (buf[pos - 1 - k] as u32) << (8 * k);
    }
    v
}
impl LitMix {
    pub fn new() -> Self {
        // STRETCH derived from the checked-in SQUASH by integer scan -- the
        // monotone inverse, no floats, identical on every machine
        let mut stretch = vec![0i16; 4096];
        let mut j = 0usize;
        for (p, st) in stretch.iter_mut().enumerate() {
            while j < 4095 && (SQUASH[j] as usize) < p {
                j += 1;
            }
            let x = if j > 0 && (p as i32 - SQUASH[j - 1] as i32) <= (SQUASH[j] as i32 - p as i32) {
                j - 1
            } else {
                j
            };
            *st = ((x as i32) - 2047).clamp(-2047, 2047) as i16;
        }
        let mut w = vec![0i32; 64 * 2 * 6];
        for v in w.chunks_mut(6) {
            for vk in v.iter_mut().take(5) {
                *vk = (1 << 16) / 5; // start as an even blend; bias 0
            }
        }
        let mut axis = [0u16; 33];
        for (i, a) in axis.iter_mut().enumerate() {
            let st = ((i as i32) - 16) * 128; // bucket centers on the stretch axis
            *a = SQUASH[(st + 2047).clamp(0, 4095) as usize];
        }
        LitMix {
            o0: vec![PINIT; 32],
            o1: vec![PINIT; 256 * 16],
            o2: vec![PINIT; 65536 * 16],
            o4: vec![PINIT; (1 << 18) * 16],
            mb: vec![PINIT; 512],
            w,
            stretch,
            apm: Apm::new(&axis),
        }
    }
    /// one bit: gather the five readings, mix -- shared by encoder and
    /// decoder verbatim (the mirror is sacred)
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn mix(
        &self,
        node: usize,
        phase: usize,
        hist: usize,
        ctx18: usize,
        mbit: u32,
        prevnib: usize,
        still: bool,
        am: usize,
    ) -> (u16, usize, [usize; 5], [i32; 6]) {
        let i0 = phase * 16 + node;
        let i1 = (hist & 0xff) * 16 + node;
        let i2 = (hist & 0xffff) * 16 + node;
        let i4 = ctx18 + node;
        let im = ((mbit as usize) * 16 + prevnib) * 16 + node;
        let pm = if still { self.mb[im] } else { 2048 };
        let ps = [self.o0[i0], self.o1[i1], self.o2[i2], self.o4[i4], pm];
        let sts = [
            self.stretch[ps[0] as usize] as i32,
            self.stretch[ps[1] as usize] as i32,
            self.stretch[ps[2] as usize] as i32,
            self.stretch[ps[3] as usize] as i32,
            self.stretch[ps[4] as usize] as i32,
            256, // bias input
        ];
        let wsel = ((phase * 16 + node) * 2 + am) * 6;
        let mut t: i64 = 0;
        for (&wv, &st) in self.w[wsel..wsel + 6].iter().zip(sts.iter()) {
            t += wv as i64 * st as i64;
        }
        let t = (t >> 16).clamp(-2047, 2047) as i32;
        let p = SQUASH[(t + 2047) as usize].clamp(1, 4095);
        (p, wsel, [i0, i1, i2, i4, im], sts)
    }
    #[inline]
    fn learn(&mut self, p: u16, b: u32, wsel: usize, idx: [usize; 5], sts: [i32; 6], still: bool) {
        let y: i32 = if b == 0 { 4096 } else { 0 };
        let err = y - p as i32;
        for (w, &st) in self.w[wsel..wsel + 6].iter_mut().zip(sts.iter()) {
            *w = (*w + ((err * st) >> MIX_LR)).clamp(-MIX_WCLAMP, MIX_WCLAMP);
        }
        upd(&mut self.o0[idx[0]], b);
        upd(&mut self.o1[idx[1]], b);
        upd(&mut self.o2[idx[2]], b);
        upd(&mut self.o4[idx[3]], b);
        if still {
            upd(&mut self.mb[idx[4]], b);
        }
    }
    #[inline]
    fn ctx18(&self, prev4: u32, phase: usize, hi: u32) -> usize {
        let key: u64 = if phase == 0 {
            prev4 as u64
        } else {
            prev4 as u64 | ((0x10 | hi) as u64) << 32
        };
        (((key.wrapping_mul(0x9E3779B1) >> 14) as usize) & 0x3ffff) * 16
    }
    #[allow(clippy::too_many_arguments)]
    fn enc_nib(
        &mut self,
        rc: &mut REnc,
        nib: u32,
        mnib: u32,
        still: &mut bool,
        hist: usize,
        prev4: u32,
        phase: usize,
        hi: u32,
        am: usize,
    ) {
        let ctx18 = self.ctx18(prev4, phase, hi);
        let prevnib = hist & 15;
        let actx = hist & 0xff;
        let mut node = 1usize;
        for i in (0..4).rev() {
            let b = (nib >> i) & 1;
            let mbit = (mnib >> i) & 1;
            let (p, wsel, idx, sts) = self.mix(node, phase, hist, ctx18, mbit, prevnib, *still, am);
            let (pa, abase, awgt) = self.apm.refine(actx, self.stretch[p as usize] as i32);
            let pf = (((pa as u32) * 3 + p as u32) >> 2).clamp(1, 4095) as u16;
            rc.bit_p(pf, b, CAT_LIT);
            self.learn(p, b, wsel, idx, sts, *still);
            self.apm.learn(abase, awgt, b);
            if *still && b != mbit {
                *still = false;
            }
            node = (node << 1) | b as usize;
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn dec_nib(
        &mut self,
        rd: &mut RDec,
        mnib: u32,
        still: &mut bool,
        hist: usize,
        prev4: u32,
        phase: usize,
        hi: u32,
        am: usize,
    ) -> u32 {
        let ctx18 = self.ctx18(prev4, phase, hi);
        let prevnib = hist & 15;
        let actx = hist & 0xff;
        let mut node = 1usize;
        for i in (0..4).rev() {
            let mbit = (mnib >> i) & 1;
            let (p, wsel, idx, sts) = self.mix(node, phase, hist, ctx18, mbit, prevnib, *still, am);
            let (pa, abase, awgt) = self.apm.refine(actx, self.stretch[p as usize] as i32);
            let pf = (((pa as u32) * 3 + p as u32) >> 2).clamp(1, 4095) as u16;
            let b = rd.bit_p(pf);
            self.learn(p, b, wsel, idx, sts, *still);
            self.apm.learn(abase, awgt, b);
            if *still && b != mbit {
                *still = false;
            }
            node = (node << 1) | b as usize;
        }
        (node as u32) - 16
    }
    /// FNV-64 over every counter and weight: the encoder/decoder mirror gate
    pub fn state_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let mut feed = |v: u64| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        };
        for t in [&self.o0, &self.o1, &self.o2, &self.o4, &self.mb, &self.apm.t] {
            for &c in t.iter() {
                feed(c as u64);
            }
        }
        for &wv in self.w.iter() {
            feed(wv as u64);
        }
        h
    }
}

fn enc_dist(rc: &mut REnc, m: &mut Models, dist: u32, cls: usize) {
    let slot = bucket_of(dist) as usize; // dist >= 1
    if let Some(s) = rc.stats.as_deref_mut() {
        s.slot_hist[slot] += 1;
    }
    enc_tree(rc, &mut m.dist_slot[cls], 5, slot as u32, CAT_SLOT);
    let r = dist - (1 << slot);
    if slot == 0 {
    } else if slot <= 4 {
        enc_tree(rc, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32, r, CAT_EXTRA);
    } else {
        rc.direct(r >> 4, slot as u32 - 4, CAT_EXTRA);
        enc_tree(rc, &mut m.align, 4, r & 15, CAT_EXTRA);
    }
}
fn dec_dist(rd: &mut RDec, m: &mut Models, cls: usize) -> Result<u32, String> {
    let slot = dec_tree(rd, &mut m.dist_slot[cls], 5) as usize;
    // 31 is the 5-bit tree's own ceiling. This guard (and its twins below)
    // sat at 25 through v7-v10: a 2^26-byte wall no corpus under 64 MB ever
    // reached -- the encoder emitted farther matches happily and the artifact
    // was written healthy and unrestorable. Found by the big arena 2026-09-01.
    if slot > 31 {
        return Err("offset slot ran away".into());
    }
    let r = if slot == 0 {
        0
    } else if slot <= 4 {
        dec_tree(rd, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32)
    } else {
        let hi = rd.direct(slot as u32 - 4);
        let lo = dec_tree(rd, &mut m.align, 4);
        (hi << 4) | lo
    };
    Ok((1u32 << slot) + r)
}

// ---------------- the walk down and back up the tree ----------------
/// lit_bits: 8 or 16 = v7's single-depth tree; 0 = the mixed model (v8)
pub fn encode(src: &[u8], toks: &[Tok], lit_bits: u32) -> Vec<u8> {
    encode_stats(src, toks, lit_bits, false).0
}
pub fn encode_stats(src: &[u8], toks: &[Tok], lit_bits: u32, want_stats: bool) -> (Vec<u8>, Option<Box<Stats>>) {
    let mixed = lit_bits == 0;
    let mut lm = if mixed { Some(Box::new(LitMix::new())) } else { None };
    let mut rc = REnc::new(want_stats);
    let mut m = Models::new(if mixed { 16 } else { lit_bits });
    let mut hist: usize = 0; // last nibs of the stream so far, lit_bits wide
    let mut tstate: usize = 0; // last two token types
    let mut reps: [u32; 4] = [0; 4];
    let mut pos = 0usize;
    for t in toks {
        match *t {
            Tok::Lit(b) => {
                rc.bit(&mut m.typ[tstate], 0, CAT_TYP);
                // the first literal after a match is steered by the byte the
                // match source would have continued with (Pavlov's trick)
                let am = tstate & 1;
                let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                    let mb = src[pos - reps[0] as usize];
                    ((mb >> 4) as u32, (mb & 15) as u32, true)
                } else {
                    (0, 0, false)
                };
                let hi = (b >> 4) as u32;
                let lo = (b & 15) as u32;
                if let Some(lm) = lm.as_deref_mut() {
                    let prev4 = tail4(src, pos);
                    lm.enc_nib(&mut rc, hi, mhi, &mut still, hist, prev4, 0, 0, am);
                    hist = ((hist << 4) | hi as usize) & m.lit_mask;
                    lm.enc_nib(&mut rc, lo, mlo, &mut still, hist, prev4, 1, hi, am);
                    hist = ((hist << 4) | lo as usize) & m.lit_mask;
                } else {
                    enc_nib(&mut rc, &mut m.lit[hist * 48..hist * 48 + 48], hi, mhi, &mut still);
                    hist = ((hist << 4) | hi as usize) & m.lit_mask;
                    enc_nib(&mut rc, &mut m.lit[hist * 48..hist * 48 + 48], lo, mlo, &mut still);
                    hist = ((hist << 4) | lo as usize) & m.lit_mask;
                }
                tstate = (tstate << 1) & 7;
                pos += 1;
                if let Some(s) = rc.stats.as_deref_mut() {
                    s.lits += 1;
                }
            }
            Tok::Match { len, dist } => {
                rc.bit(&mut m.typ[tstate], 1, CAT_TYP);
                let rep_k = reps.iter().position(|&r| r == dist);
                rc.bit(&mut m.is_rep[tstate & 3], rep_k.is_some() as u32, CAT_REP);
                let v = len - MIN_MATCH as u32;
                let is_rep = if let Some(k) = rep_k {
                    enc_tree(&mut rc, &mut m.rep_idx, 2, k as u32, CAT_REP);
                    m.len_r[tstate & 1].encode(&mut rc, v);
                    true
                } else {
                    m.len_m[tstate & 1].encode(&mut rc, v);
                    let cls = (len as usize - MIN_MATCH).min(3);
                    enc_dist(&mut rc, &mut m, dist, cls);
                    false
                };
                rep_update(&mut reps, dist);
                pos += len as usize;
                // the trailing nibs of what the match copied, lit_bits wide
                hist = if pos >= 2 {
                    ((src[pos - 2] as usize) << 8 | src[pos - 1] as usize) & m.lit_mask
                } else {
                    src[pos - 1] as usize & m.lit_mask
                };
                tstate = ((tstate << 1) | 1) & 7;
                if let Some(s) = rc.stats.as_deref_mut() {
                    s.matches += 1;
                    s.match_bytes += len as u64;
                    if is_rep {
                        s.reps += 1;
                    }
                }
            }
        }
    }
    debug_assert_eq!(pos, src.len());
    if let Some(lm) = lm.as_deref() {
        if std::env::var_os("EGG_STATEHASH").is_some() {
            eprintln!("statehash enc {:016x}", lm.state_hash());
        }
    }
    rc.flush()
}

pub fn decode(inp: &[u8], orig_len: usize, lit_bits: u32) -> Result<Vec<u8>, String> {
    let mixed = lit_bits == 0;
    let mut lm = if mixed { Some(Box::new(LitMix::new())) } else { None };
    let mut rd = RDec::new(inp);
    let mut m = Models::new(if mixed { 16 } else { lit_bits });
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    while out.len() < orig_len {
        if rd.bit(&mut m.typ[tstate]) == 0 {
            let am = tstate & 1;
            let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                let mb = out[out.len() - reps[0] as usize];
                ((mb >> 4) as u32, (mb & 15) as u32, true)
            } else {
                (0, 0, false)
            };
            let (hi, lo) = if let Some(lm) = lm.as_deref_mut() {
                let prev4 = tail4(&out, out.len());
                let hi = lm.dec_nib(&mut rd, mhi, &mut still, hist, prev4, 0, 0, am);
                hist = ((hist << 4) | hi as usize) & m.lit_mask;
                let lo = lm.dec_nib(&mut rd, mlo, &mut still, hist, prev4, 1, hi, am);
                hist = ((hist << 4) | lo as usize) & m.lit_mask;
                (hi, lo)
            } else {
                let hi = dec_nib(&mut rd, &mut m.lit[hist * 48..hist * 48 + 48], mhi, &mut still);
                hist = ((hist << 4) | hi as usize) & m.lit_mask;
                let lo = dec_nib(&mut rd, &mut m.lit[hist * 48..hist * 48 + 48], mlo, &mut still);
                hist = ((hist << 4) | lo as usize) & m.lit_mask;
                (hi, lo)
            };
            out.push(((hi << 4) | lo) as u8);
            tstate = (tstate << 1) & 7;
        } else {
            let is_rep = rd.bit(&mut m.is_rep[tstate & 3]) == 1;
            let (len, dist) = if is_rep {
                let k = dec_tree(&mut rd, &mut m.rep_idx, 2) as usize;
                let dist = reps[k];
                if dist == 0 {
                    return Err("rep offset before any offset".into());
                }
                (m.len_r[tstate & 1].decode(&mut rd)? as usize + MIN_MATCH, dist)
            } else {
                let len = m.len_m[tstate & 1].decode(&mut rd)? as usize + MIN_MATCH;
                let cls = (len - MIN_MATCH).min(3);
                let dist = dec_dist(&mut rd, &mut m, cls)?;
                (len, dist)
            };
            rep_update(&mut reps, dist);
            let d = dist as usize;
            if d == 0 || d > out.len() || out.len() + len > orig_len {
                return Err("malformed match in dyadic stream".into());
            }
            for _ in 0..len {
                let b = out[out.len() - d];
                out.push(b);
            }
            hist = if out.len() >= 2 {
                ((out[out.len() - 2] as usize) << 8 | out[out.len() - 1] as usize) & m.lit_mask
            } else {
                out[out.len() - 1] as usize & m.lit_mask
            };
            tstate = ((tstate << 1) | 1) & 7;
        }
    }
    if let Some(lm) = lm.as_deref() {
        if std::env::var_os("EGG_STATEHASH").is_some() {
            eprintln!("statehash dec {:016x}", lm.state_hash());
        }
    }
    Ok(out)
}


// ==================== v9: MODEL_MIX9 (appended; everything above is the ====
// ==================== frozen v8 entrant and is never edited)            ====
// The token walk is v7/v8's exactly; the literal bits go through mix9::Mix9
// (match model + widened mixer + APM). The match model sees EVERY byte that
// lands -- literals and match-copied bytes alike -- on both sides, in the
// same order (the mirror; EGG_STATEHASH proves it per file).

use crate::mix9::{tail4 as tail4_9, tail8 as tail8_9, Mix9};

/// v9 token-side models (try-again stage): the v8 shapes with wider,
/// mirrored contexts -- length models keyed by the last TWO token types,
/// distance slots keyed by (length class x last fresh slot's coarse class),
/// align bits keyed by slot parity. dist-extra was 16-27% of PE output
/// under the v7-era single contexts; these are the same adaptive trees,
/// just told where they are.
struct Tok9 {
    typ: [u16; 8],
    is_rep: [u16; 4],
    rep_idx: [u16; 4],
    len_m: [LenModel; 4],
    len_r: [LenModel; 4],
    dist_slot: [[u16; 32]; 16], // cls(4) x prev fresh-slot class(4)
    dist_lo: [u16; 30],
    align: [[u16; 16]; 4], // slot & 3
    prev_slot_cls: usize,  // maintained identically on both sides
}
impl Tok9 {
    fn new() -> Self {
        Tok9 {
            typ: [PINIT; 8],
            is_rep: [PINIT; 4],
            rep_idx: [PINIT; 4],
            len_m: [LenModel::new(), LenModel::new(), LenModel::new(), LenModel::new()],
            len_r: [LenModel::new(), LenModel::new(), LenModel::new(), LenModel::new()],
            dist_slot: [[PINIT; 32]; 16],
            dist_lo: [PINIT; 30],
            align: [[PINIT; 16]; 4],
            prev_slot_cls: 0,
        }
    }
}
fn enc_dist9(rc: &mut REnc, m: &mut Tok9, dist: u32, cls: usize) {
    let slot = bucket_of(dist) as usize;
    if let Some(s) = rc.stats.as_deref_mut() {
        s.slot_hist[slot] += 1;
    }
    enc_tree(rc, &mut m.dist_slot[cls * 4 + m.prev_slot_cls], 5, slot as u32, CAT_SLOT);
    m.prev_slot_cls = (slot >> 3).min(3);
    let r = dist - (1 << slot);
    if slot == 0 {
    } else if slot <= 4 {
        enc_tree(rc, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32, r, CAT_EXTRA);
    } else {
        rc.direct(r >> 4, slot as u32 - 4, CAT_EXTRA);
        enc_tree(rc, &mut m.align[slot & 3], 4, r & 15, CAT_EXTRA);
    }
}
fn dec_dist9(rd: &mut RDec, m: &mut Tok9, cls: usize) -> Result<u32, String> {
    let slot = dec_tree(rd, &mut m.dist_slot[cls * 4 + m.prev_slot_cls], 5) as usize;
    if slot > 31 {
        return Err("offset slot ran away".into());
    }
    m.prev_slot_cls = (slot >> 3).min(3);
    let r = if slot == 0 {
        0
    } else if slot <= 4 {
        dec_tree(rd, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32)
    } else {
        let hi = rd.direct(slot as u32 - 4);
        let lo = dec_tree(rd, &mut m.align[slot & 3], 4);
        (hi << 4) | lo
    };
    Ok((1u32 << slot) + r)
}

#[allow(clippy::too_many_arguments)]
fn enc_nib9(
    rc: &mut REnc,
    lm: &mut Mix9,
    nib: u32,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let b = (nib >> i) & 1;
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, steer, mm_exp, am);
        rc.bit_p(b9.p, b, CAT_LIT);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
}
#[allow(clippy::too_many_arguments)]
fn dec_nib9(
    rd: &mut RDec,
    lm: &mut Mix9,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) -> u32 {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, steer, mm_exp, am);
        let b = rd.bit_p(b9.p);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
    (node as u32) - 16
}

pub fn encode9(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    let mut rc = REnc::new(false);
    let mut m = Tok9::new(); // v9 token-side models (widened contexts)
    let mut lm = Box::new(Mix9::new());
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    let mut pos = 0usize;
    for t in toks {
        match *t {
            Tok::Lit(b) => {
                rc.bit(&mut m.typ[tstate], 0, CAT_TYP);
                let am = tstate & 1;
                let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                    let mb = src[pos - reps[0] as usize];
                    ((mb >> 4) as u32, (mb & 15) as u32, true)
                } else {
                    (0, 0, false)
                };
                let prev4 = tail4_9(src, pos);
                let t8 = tail8_9(src, pos);
                let mm_pred = lm.mmodel.predicted(&src[..pos]);
                let mut malive = mm_pred.is_some();
                let hi = (b >> 4) as u32;
                let lo = (b & 15) as u32;
                let cx0 = lm.ctx18(prev4, 0, 0);
                let c30 = lm.ctx20(t8, 3, 0, 0);
                let c60 = lm.ctx20(t8, 6, 0, 0);
                let s10 = lm.ctx_sparse(t8, 0, 0, 0);
                let s20 = lm.ctx_sparse(t8, 1, 0, 0);
                enc_nib9(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, mm_pred, &mut malive, 0, am);
                hist = ((hist << 4) | hi as usize) & 0xffff;
                let cx1 = lm.ctx18(prev4, 1, hi);
                let c31 = lm.ctx20(t8, 3, 1, hi);
                let c61 = lm.ctx20(t8, 6, 1, hi);
                let s11 = lm.ctx_sparse(t8, 0, 1, hi);
                let s21 = lm.ctx_sparse(t8, 1, 1, hi);
                enc_nib9(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, mm_pred, &mut malive, 1, am);
                hist = ((hist << 4) | lo as usize) & 0xffff;
                tstate = (tstate << 1) & 7;
                pos += 1;
                lm.mmodel.update(src, pos);
            }
            Tok::Match { len, dist } => {
                rc.bit(&mut m.typ[tstate], 1, CAT_TYP);
                let rep_k = reps.iter().position(|&r| r == dist);
                rc.bit(&mut m.is_rep[tstate & 3], rep_k.is_some() as u32, CAT_REP);
                let v = len - MIN_MATCH as u32;
                if let Some(k) = rep_k {
                    enc_tree(&mut rc, &mut m.rep_idx, 2, k as u32, CAT_REP);
                    m.len_r[tstate & 3].encode(&mut rc, v);
                } else {
                    m.len_m[tstate & 3].encode(&mut rc, v);
                    let cls = (len as usize - MIN_MATCH).min(3);
                    enc_dist9(&mut rc, &mut m, dist, cls);
                }
                rep_update(&mut reps, dist);
                // the match model reads every copied byte, in landing order
                for p in pos..pos + len as usize {
                    lm.mmodel.update(src, p + 1);
                }
                pos += len as usize;
                hist = if pos >= 2 {
                    ((src[pos - 2] as usize) << 8 | src[pos - 1] as usize) & 0xffff
                } else {
                    src[pos - 1] as usize & 0xffff
                };
                tstate = ((tstate << 1) | 1) & 7;
            }
        }
    }
    debug_assert_eq!(pos, src.len());
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
        eprintln!("mm enc agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    rc.flush().0
}

pub fn decode9(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new(inp);
    let mut m = Tok9::new();
    let mut lm = Box::new(Mix9::new());
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    while out.len() < orig_len {
        if rd.bit(&mut m.typ[tstate]) == 0 {
            let am = tstate & 1;
            let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                let mb = out[out.len() - reps[0] as usize];
                ((mb >> 4) as u32, (mb & 15) as u32, true)
            } else {
                (0, 0, false)
            };
            let pos = out.len();
            let prev4 = tail4_9(&out, pos);
            let t8 = tail8_9(&out, pos);
            let mm_pred = lm.mmodel.predicted(&out);
            let mut malive = mm_pred.is_some();
            let cx0 = lm.ctx18(prev4, 0, 0);
            let c30 = lm.ctx20(t8, 3, 0, 0);
            let c60 = lm.ctx20(t8, 6, 0, 0);
            let s10 = lm.ctx_sparse(t8, 0, 0, 0);
            let s20 = lm.ctx_sparse(t8, 1, 0, 0);
            let hi = dec_nib9(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, mm_pred, &mut malive, 0, am);
            hist = ((hist << 4) | hi as usize) & 0xffff;
            let cx1 = lm.ctx18(prev4, 1, hi);
            let c31 = lm.ctx20(t8, 3, 1, hi);
            let c61 = lm.ctx20(t8, 6, 1, hi);
            let s11 = lm.ctx_sparse(t8, 0, 1, hi);
            let s21 = lm.ctx_sparse(t8, 1, 1, hi);
            let lo = dec_nib9(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, mm_pred, &mut malive, 1, am);
            hist = ((hist << 4) | lo as usize) & 0xffff;
            out.push(((hi << 4) | lo) as u8);
            tstate = (tstate << 1) & 7;
            lm.mmodel.update(&out, out.len());
        } else {
            let is_rep = rd.bit(&mut m.is_rep[tstate & 3]) == 1;
            let (len, dist) = if is_rep {
                let k = dec_tree(&mut rd, &mut m.rep_idx, 2) as usize;
                let dist = reps[k];
                if dist == 0 {
                    return Err("rep offset before any offset".into());
                }
                (m.len_r[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH, dist)
            } else {
                let len = m.len_m[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH;
                let cls = (len - MIN_MATCH).min(3);
                let dist = dec_dist9(&mut rd, &mut m, cls)?;
                (len, dist)
            };
            rep_update(&mut reps, dist);
            let d = dist as usize;
            if d == 0 || d > out.len() || out.len() + len > orig_len {
                return Err("malformed match in dyadic stream".into());
            }
            for _ in 0..len {
                let b = out[out.len() - d];
                out.push(b);
                lm.mmodel.update(&out, out.len());
            }
            hist = if out.len() >= 2 {
                ((out[out.len() - 2] as usize) << 8 | out[out.len() - 1] as usize) & 0xffff
            } else {
                out[out.len() - 1] as usize & 0xffff
            };
            tstate = ((tstate << 1) | 1) & 7;
        }
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
        eprintln!("mm dec agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    Ok(out)
}


// ==================== v9-M3: MODEL_CM9 -- the literal-only entrant ====
// No tokenize, no type/length/offset models: every byte walks the Mix9
// literal path and the MATCH MODEL carries the repeats (the remainder
// reading standing alone). The v8 Pavlov steering is unified with it: the
// steering byte is the match model's predicted byte once agreement is >= 4.
// Wins expected on text and long-range-structured files; the trial floor
// makes it free everywhere else.

#[allow(dead_code)] // frozen tier (v9): kept beside its restore path
pub fn encode_cm9(src: &[u8]) -> Vec<u8> {
    let mut rc = REnc::new(false);
    let mut lm = Box::new(Mix9::new());
    let mut hist: usize = 0;
    for pos in 0..src.len() {
        let b = src[pos];
        let prev4 = tail4_9(src, pos);
        let t8 = tail8_9(src, pos);
        let mm_pred = lm.mmodel.predicted(&src[..pos]);
        let mut malive = mm_pred.is_some();
        // steering: the predicted byte, once the agreement is credible
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let hi = (b >> 4) as u32;
        let lo = (b & 15) as u32;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let s10 = lm.ctx_sparse(t8, 0, 0, 0);
        let s20 = lm.ctx_sparse(t8, 1, 0, 0);
        enc_nib9(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let s11 = lm.ctx_sparse(t8, 0, 1, hi);
        let s21 = lm.ctx_sparse(t8, 1, 1, hi);
        enc_nib9(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        lm.mmodel.update(src, pos + 1);
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
    }
    rc.flush().0
}

pub fn decode_cm9(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new(inp);
    let mut lm = Box::new(Mix9::new());
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    while out.len() < orig_len {
        let pos = out.len();
        let prev4 = tail4_9(&out, pos);
        let t8 = tail8_9(&out, pos);
        let mm_pred = lm.mmodel.predicted(&out);
        let mut malive = mm_pred.is_some();
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let s10 = lm.ctx_sparse(t8, 0, 0, 0);
        let s20 = lm.ctx_sparse(t8, 1, 0, 0);
        let hi = dec_nib9(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let s11 = lm.ctx_sparse(t8, 0, 1, hi);
        let s21 = lm.ctx_sparse(t8, 1, 1, hi);
        let lo = dec_nib9(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        out.push(((hi << 4) | lo) as u8);
        lm.mmodel.update(&out, out.len());
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
    }
    Ok(out)
}

// ==================== v10: MODEL_MIX10 (appended; the v9 section above is ====
// ==================== frozen tier now, exactly like the v8 paths)       ====
// The token walk is v7/v8's exactly; the literal bits go through mix9::Mix10
// (match model + widened mixer + APM). The match model sees EVERY byte that
// lands -- literals and match-copied bytes alike -- on both sides, in the
// same order (the mirror; EGG_STATEHASH proves it per file).

use crate::mix10::{tail4 as tail4_10, tail8 as tail8_10, Mix10};

/// v9 token-side models (try-again stage): the v8 shapes with wider,
/// mirrored contexts -- length models keyed by the last TWO token types,
/// distance slots keyed by (length class x last fresh slot's coarse class),
/// align bits keyed by slot parity. dist-extra was 16-27% of PE output
/// under the v7-era single contexts; these are the same adaptive trees,
/// just told where they are.
struct Tok10 {
    typ: [u16; 8],
    is_rep: [u16; 4],
    rep_idx: [u16; 4],
    len_m: [LenModel; 4],
    len_r: [LenModel; 4],
    dist_slot: [[u16; 32]; 16], // cls(4) x prev fresh-slot class(4)
    dist_lo: [u16; 30],
    align: [[u16; 16]; 4], // slot & 3
    prev_slot_cls: usize,  // maintained identically on both sides
}
impl Tok10 {
    fn new() -> Self {
        Tok10 {
            typ: [PINIT; 8],
            is_rep: [PINIT; 4],
            rep_idx: [PINIT; 4],
            len_m: [LenModel::new(), LenModel::new(), LenModel::new(), LenModel::new()],
            len_r: [LenModel::new(), LenModel::new(), LenModel::new(), LenModel::new()],
            dist_slot: [[PINIT; 32]; 16],
            dist_lo: [PINIT; 30],
            align: [[PINIT; 16]; 4],
            prev_slot_cls: 0,
        }
    }
}
fn enc_dist10(rc: &mut REnc, m: &mut Tok10, dist: u32, cls: usize) {
    let slot = bucket_of(dist) as usize;
    if let Some(s) = rc.stats.as_deref_mut() {
        s.slot_hist[slot] += 1;
    }
    enc_tree(rc, &mut m.dist_slot[cls * 4 + m.prev_slot_cls], 5, slot as u32, CAT_SLOT);
    m.prev_slot_cls = (slot >> 3).min(3);
    let r = dist - (1 << slot);
    if slot == 0 {
    } else if slot <= 4 {
        enc_tree(rc, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32, r, CAT_EXTRA);
    } else {
        rc.direct(r >> 4, slot as u32 - 4, CAT_EXTRA);
        enc_tree(rc, &mut m.align[slot & 3], 4, r & 15, CAT_EXTRA);
    }
}
fn dec_dist10(rd: &mut RDec, m: &mut Tok10, cls: usize) -> Result<u32, String> {
    let slot = dec_tree(rd, &mut m.dist_slot[cls * 4 + m.prev_slot_cls], 5) as usize;
    if slot > 31 {
        return Err("offset slot ran away".into());
    }
    m.prev_slot_cls = (slot >> 3).min(3);
    let r = if slot == 0 {
        0
    } else if slot <= 4 {
        dec_tree(rd, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32)
    } else {
        let hi = rd.direct(slot as u32 - 4);
        let lo = dec_tree(rd, &mut m.align[slot & 3], 4);
        (hi << 4) | lo
    };
    Ok((1u32 << slot) + r)
}

#[allow(clippy::too_many_arguments)]
fn enc_nib10(
    rc: &mut REnc,
    lm: &mut Mix10,
    nib: u32,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    ci1: usize,
    ci2: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let b = (nib >> i) & 1;
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, ci1, ci2, steer, mm_exp, am);
        rc.bit_p(b9.p, b, CAT_LIT);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
}
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)] // frozen floor: wired as the widened-fallback contingent at v11-M4
fn dec_nib10(
    rd: &mut RDec,
    lm: &mut Mix10,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    ci1: usize,
    ci2: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) -> u32 {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, ci1, ci2, steer, mm_exp, am);
        let b = rd.bit_p(b9.p);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
    (node as u32) - 16
}

pub fn encode10(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    let mut rc = REnc::new(false);
    let mut m = Tok10::new(); // v9 token-side models (widened contexts)
    let mut lm = Box::new(Mix10::new());
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    let mut pos = 0usize;
    for t in toks {
        match *t {
            Tok::Lit(b) => {
                rc.bit(&mut m.typ[tstate], 0, CAT_TYP);
                let am = tstate & 1;
                let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                    let mb = src[pos - reps[0] as usize];
                    ((mb >> 4) as u32, (mb & 15) as u32, true)
                } else {
                    (0, 0, false)
                };
                let prev4 = tail4_10(src, pos);
                let t8 = tail8_10(src, pos);
                let mm_pred = lm.mmodel.predicted(&src[..pos]);
                let mut malive = mm_pred.is_some();
                let hi = (b >> 4) as u32;
                let lo = (b & 15) as u32;
                let cx0 = lm.ctx18(prev4, 0, 0);
                let c30 = lm.ctx20(t8, 3, 0, 0);
                let c60 = lm.ctx20(t8, 6, 0, 0);
                let s10 = lm.ctx_sparse(t8, 0, 0, 0);
                let s20 = lm.ctx_sparse(t8, 1, 0, 0);
                let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
                let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
                enc_nib10(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
                hist = ((hist << 4) | hi as usize) & 0xffff;
                let cx1 = lm.ctx18(prev4, 1, hi);
                let c31 = lm.ctx20(t8, 3, 1, hi);
                let c61 = lm.ctx20(t8, 6, 1, hi);
                let s11 = lm.ctx_sparse(t8, 0, 1, hi);
                let s21 = lm.ctx_sparse(t8, 1, 1, hi);
                let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
                let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
                enc_nib10(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
                hist = ((hist << 4) | lo as usize) & 0xffff;
                tstate = (tstate << 1) & 7;
                pos += 1;
                lm.byte_update(src, pos);
            }
            Tok::Match { len, dist } => {
                rc.bit(&mut m.typ[tstate], 1, CAT_TYP);
                let rep_k = reps.iter().position(|&r| r == dist);
                rc.bit(&mut m.is_rep[tstate & 3], rep_k.is_some() as u32, CAT_REP);
                let v = len - MIN_MATCH as u32;
                if let Some(k) = rep_k {
                    enc_tree(&mut rc, &mut m.rep_idx, 2, k as u32, CAT_REP);
                    m.len_r[tstate & 3].encode(&mut rc, v);
                } else {
                    m.len_m[tstate & 3].encode(&mut rc, v);
                    let cls = (len as usize - MIN_MATCH).min(3);
                    enc_dist10(&mut rc, &mut m, dist, cls);
                }
                rep_update(&mut reps, dist);
                // the match model reads every copied byte, in landing order
                for p in pos..pos + len as usize {
                    lm.byte_update(src, p + 1);
                }
                pos += len as usize;
                hist = if pos >= 2 {
                    ((src[pos - 2] as usize) << 8 | src[pos - 1] as usize) & 0xffff
                } else {
                    src[pos - 1] as usize & 0xffff
                };
                tstate = ((tstate << 1) | 1) & 7;
            }
        }
    }
    debug_assert_eq!(pos, src.len());
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
        eprintln!("mm enc agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    rc.flush().0
}

pub fn decode10(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new(inp);
    let mut m = Tok10::new();
    let mut lm = Box::new(Mix10::new());
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    while out.len() < orig_len {
        if rd.bit(&mut m.typ[tstate]) == 0 {
            let am = tstate & 1;
            let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                let mb = out[out.len() - reps[0] as usize];
                ((mb >> 4) as u32, (mb & 15) as u32, true)
            } else {
                (0, 0, false)
            };
            let pos = out.len();
            let prev4 = tail4_10(&out, pos);
            let t8 = tail8_10(&out, pos);
            let mm_pred = lm.mmodel.predicted(&out);
            let mut malive = mm_pred.is_some();
            let cx0 = lm.ctx18(prev4, 0, 0);
            let c30 = lm.ctx20(t8, 3, 0, 0);
            let c60 = lm.ctx20(t8, 6, 0, 0);
            let s10 = lm.ctx_sparse(t8, 0, 0, 0);
            let s20 = lm.ctx_sparse(t8, 1, 0, 0);
            let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
            let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
            let hi = dec_nib10(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
            hist = ((hist << 4) | hi as usize) & 0xffff;
            let cx1 = lm.ctx18(prev4, 1, hi);
            let c31 = lm.ctx20(t8, 3, 1, hi);
            let c61 = lm.ctx20(t8, 6, 1, hi);
            let s11 = lm.ctx_sparse(t8, 0, 1, hi);
            let s21 = lm.ctx_sparse(t8, 1, 1, hi);
            let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
            let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
            let lo = dec_nib10(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
            hist = ((hist << 4) | lo as usize) & 0xffff;
            out.push(((hi << 4) | lo) as u8);
            tstate = (tstate << 1) & 7;
            lm.byte_update(&out, out.len());
        } else {
            let is_rep = rd.bit(&mut m.is_rep[tstate & 3]) == 1;
            let (len, dist) = if is_rep {
                let k = dec_tree(&mut rd, &mut m.rep_idx, 2) as usize;
                let dist = reps[k];
                if dist == 0 {
                    return Err("rep offset before any offset".into());
                }
                (m.len_r[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH, dist)
            } else {
                let len = m.len_m[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH;
                let cls = (len - MIN_MATCH).min(3);
                let dist = dec_dist10(&mut rd, &mut m, cls)?;
                (len, dist)
            };
            rep_update(&mut reps, dist);
            let d = dist as usize;
            if d == 0 || d > out.len() || out.len() + len > orig_len {
                return Err("malformed match in dyadic stream".into());
            }
            for _ in 0..len {
                let b = out[out.len() - d];
                out.push(b);
                lm.byte_update(&out, out.len());
            }
            hist = if out.len() >= 2 {
                ((out[out.len() - 2] as usize) << 8 | out[out.len() - 1] as usize) & 0xffff
            } else {
                out[out.len() - 1] as usize & 0xffff
            };
            tstate = ((tstate << 1) | 1) & 7;
        }
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
        eprintln!("mm dec agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    Ok(out)
}


// ==================== v10: MODEL_CM10 (copy of CM9) -- the literal-only entrant ====
// No tokenize, no type/length/offset models: every byte walks the Mix10
// literal path and the MATCH MODEL carries the repeats (the remainder
// reading standing alone). The v8 Pavlov steering is unified with it: the
// steering byte is the match model's predicted byte once agreement is >= 4.
// Wins expected on text and long-range-structured files; the trial floor
// makes it free everywhere else.

pub fn encode_cm10(src: &[u8]) -> Vec<u8> {
    let mut rc = REnc::new(false);
    let mut lm = Box::new(Mix10::new());
    let mut hist: usize = 0;
    for pos in 0..src.len() {
        let b = src[pos];
        let prev4 = tail4_10(src, pos);
        let t8 = tail8_10(src, pos);
        let mm_pred = lm.mmodel.predicted(&src[..pos]);
        let mut malive = mm_pred.is_some();
        // steering: the predicted byte, once the agreement is credible
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let hi = (b >> 4) as u32;
        let lo = (b & 15) as u32;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let s10 = lm.ctx_sparse(t8, 0, 0, 0);
        let s20 = lm.ctx_sparse(t8, 1, 0, 0);
        let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
        let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
        enc_nib10(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let s11 = lm.ctx_sparse(t8, 0, 1, hi);
        let s21 = lm.ctx_sparse(t8, 1, 1, hi);
        let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
        let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
        enc_nib10(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        lm.byte_update(src, pos + 1);
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
    }
    rc.flush().0
}

pub fn decode_cm10(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new(inp);
    let mut lm = Box::new(Mix10::new());
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    while out.len() < orig_len {
        let pos = out.len();
        let prev4 = tail4_10(&out, pos);
        let t8 = tail8_10(&out, pos);
        let mm_pred = lm.mmodel.predicted(&out);
        let mut malive = mm_pred.is_some();
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let s10 = lm.ctx_sparse(t8, 0, 0, 0);
        let s20 = lm.ctx_sparse(t8, 1, 0, 0);
        let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
        let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
        let hi = dec_nib10(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let s11 = lm.ctx_sparse(t8, 0, 1, hi);
        let s21 = lm.ctx_sparse(t8, 1, 1, hi);
        let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
        let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
        let lo = dec_nib10(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        out.push(((hi << 4) | lo) as u8);
        lm.byte_update(&out, out.len());
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
    }
    Ok(out)
}

// ==================== v10: MODEL_MIX10 (appended; the v9 section above is ====
// ==================== frozen tier now, exactly like the v8 paths)       ====
// The token walk is v7/v8's exactly; the literal bits go through mix9::Mix11
// (match model + widened mixer + APM). The match model sees EVERY byte that
// lands -- literals and match-copied bytes alike -- on both sides, in the
// same order (the mirror; EGG_STATEHASH proves it per file).

use crate::mix11::{tail4 as tail4_11, tail8 as tail8_11, Mix11, MIX10_LR};
use crate::mix12::{Book, Mix12, MIX12_LR};

/// v9 token-side models (try-again stage): the v8 shapes with wider,
/// mirrored contexts -- length models keyed by the last TWO token types,
/// distance slots keyed by (length class x last fresh slot's coarse class),
/// align bits keyed by slot parity. dist-extra was 16-27% of PE output
/// under the v7-era single contexts; these are the same adaptive trees,
/// just told where they are.
struct Tok11 {
    typ: [u16; 8],
    is_rep: [u16; 4],
    rep_idx: [u16; 4],
    len_m: [LenModel; 4],
    len_r: [LenModel; 4],
    dist_slot: [[u16; 32]; 16], // cls(4) x prev fresh-slot class(4)
    dist_lo: [u16; 30],
    align: [[u16; 16]; 4], // slot & 3
    prev_slot_cls: usize,  // maintained identically on both sides
}
impl Tok11 {
    fn new() -> Self {
        Tok11 {
            typ: [PINIT; 8],
            is_rep: [PINIT; 4],
            rep_idx: [PINIT; 4],
            len_m: [LenModel::new(), LenModel::new(), LenModel::new(), LenModel::new()],
            len_r: [LenModel::new(), LenModel::new(), LenModel::new(), LenModel::new()],
            dist_slot: [[PINIT; 32]; 16],
            dist_lo: [PINIT; 30],
            align: [[PINIT; 16]; 4],
            prev_slot_cls: 0,
        }
    }
}
fn enc_dist11(rc: &mut REnc, m: &mut Tok11, dist: u32, cls: usize) {
    let slot = bucket_of(dist) as usize;
    if let Some(s) = rc.stats.as_deref_mut() {
        s.slot_hist[slot] += 1;
    }
    enc_tree(rc, &mut m.dist_slot[cls * 4 + m.prev_slot_cls], 5, slot as u32, CAT_SLOT);
    m.prev_slot_cls = (slot >> 3).min(3);
    let r = dist - (1 << slot);
    if slot == 0 {
    } else if slot <= 4 {
        enc_tree(rc, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32, r, CAT_EXTRA);
    } else {
        rc.direct(r >> 4, slot as u32 - 4, CAT_EXTRA);
        enc_tree(rc, &mut m.align[slot & 3], 4, r & 15, CAT_EXTRA);
    }
}
fn dec_dist11(rd: &mut RDec, m: &mut Tok11, cls: usize) -> Result<u32, String> {
    let slot = dec_tree(rd, &mut m.dist_slot[cls * 4 + m.prev_slot_cls], 5) as usize;
    if slot > 31 {
        return Err("offset slot ran away".into());
    }
    m.prev_slot_cls = (slot >> 3).min(3);
    let r = if slot == 0 {
        0
    } else if slot <= 4 {
        dec_tree(rd, &mut m.dist_lo[DIST_LO_OFF[slot]..DIST_LO_OFF[slot] + (1 << slot)], slot as u32)
    } else {
        let hi = rd.direct(slot as u32 - 4);
        let lo = dec_tree(rd, &mut m.align[slot & 3], 4);
        (hi << 4) | lo
    };
    Ok((1u32 << slot) + r)
}

#[allow(clippy::too_many_arguments)]
fn enc_nib11(
    rc: &mut REnc,
    lm: &mut Mix11,
    nib: u32,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    ci1: usize,
    ci2: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let b = (nib >> i) & 1;
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, ci1, ci2, steer, mm_exp, am);
        rc.bit_p(b9.p, b, CAT_LIT);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
}
#[allow(clippy::too_many_arguments)]
fn dec_nib11(
    rd: &mut RDec,
    lm: &mut Mix11,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    ci1: usize,
    ci2: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) -> u32 {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, ci1, ci2, steer, mm_exp, am);
        let b = rd.bit_p(b9.p);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
    (node as u32) - 16
}

pub fn encode11(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    encode11_stats(src, toks, false).0
}
/// v11-M5: the prior-primed twin -- MODEL_MIX11P. The trial decides per file.
pub fn encode11p(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    encode11_inner2(src, toks, false, true, MIX10_LR).0
}
/// v11-M8: the heavy-LR twin -- MODEL_MIX11H. LR 13 feeds text/db/records.
pub fn encode11h(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    encode11_inner2(src, toks, false, false, 13).0
}
/// same stream, optionally with Stats. Built for v11-M2's DP parse; the DP
/// was killed by its own criterion, the stats surface stays for --stats use.
#[allow(dead_code)] // no caller since the DP's kill; the --stats surface may reclaim it
pub fn encode11_stats(src: &[u8], toks: &[Tok], want_stats: bool) -> (Vec<u8>, Option<Box<Stats>>) {
    encode11_inner(src, toks, want_stats, false)
}
fn encode11_inner(src: &[u8], toks: &[Tok], want_stats: bool, prior: bool) -> (Vec<u8>, Option<Box<Stats>>) {
    encode11_inner2(src, toks, want_stats, prior, MIX10_LR)
}
fn encode11_inner2(src: &[u8], toks: &[Tok], want_stats: bool, prior: bool, lr: u32) -> (Vec<u8>, Option<Box<Stats>>) {
    let mut rc = REnc::new(want_stats);
    let mut m = Tok11::new(); // v9 token-side models (widened contexts)
    let mut lm = Box::new(Mix11::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    let mut pos = 0usize;
    for t in toks {
        match *t {
            Tok::Lit(b) => {
                rc.bit(&mut m.typ[tstate], 0, CAT_TYP);
                let am = tstate & 1;
                let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                    let mb = src[pos - reps[0] as usize];
                    ((mb >> 4) as u32, (mb & 15) as u32, true)
                } else {
                    (0, 0, false)
                };
                let prev4 = tail4_11(src, pos);
                let t8 = tail8_11(src, pos);
                let mm_pred = lm.mmodel.predicted(&src[..pos]);
                let mut malive = mm_pred.is_some();
                let hi = (b >> 4) as u32;
                let lo = (b & 15) as u32;
                let cx0 = lm.ctx18(prev4, 0, 0);
                let c30 = lm.ctx20(t8, 3, 0, 0);
                let c60 = lm.ctx20(t8, 6, 0, 0);
                let s10 = lm.ctx_sparse(t8, 0, 0, 0);
                let s20 = lm.ctx_sparse(t8, 1, 0, 0);
                let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
                let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
                enc_nib11(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
                hist = ((hist << 4) | hi as usize) & 0xffff;
                let cx1 = lm.ctx18(prev4, 1, hi);
                let c31 = lm.ctx20(t8, 3, 1, hi);
                let c61 = lm.ctx20(t8, 6, 1, hi);
                let s11 = lm.ctx_sparse(t8, 0, 1, hi);
                let s21 = lm.ctx_sparse(t8, 1, 1, hi);
                let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
                let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
                enc_nib11(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
                hist = ((hist << 4) | lo as usize) & 0xffff;
                tstate = (tstate << 1) & 7;
                pos += 1;
                lm.byte_update(src, pos);
            }
            Tok::Match { len, dist } => {
                rc.bit(&mut m.typ[tstate], 1, CAT_TYP);
                let rep_k = reps.iter().position(|&r| r == dist);
                rc.bit(&mut m.is_rep[tstate & 3], rep_k.is_some() as u32, CAT_REP);
                let v = len - MIN_MATCH as u32;
                if let Some(k) = rep_k {
                    enc_tree(&mut rc, &mut m.rep_idx, 2, k as u32, CAT_REP);
                    m.len_r[tstate & 3].encode(&mut rc, v);
                } else {
                    m.len_m[tstate & 3].encode(&mut rc, v);
                    let cls = (len as usize - MIN_MATCH).min(3);
                    enc_dist11(&mut rc, &mut m, dist, cls);
                }
                rep_update(&mut reps, dist);
                // the match model reads every copied byte, in landing order
                for p in pos..pos + len as usize {
                    lm.byte_update(src, p + 1);
                }
                pos += len as usize;
                hist = if pos >= 2 {
                    ((src[pos - 2] as usize) << 8 | src[pos - 1] as usize) & 0xffff
                } else {
                    src[pos - 1] as usize & 0xffff
                };
                tstate = ((tstate << 1) | 1) & 7;
            }
        }
    }
    debug_assert_eq!(pos, src.len());
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
        eprintln!("mm enc agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    rc.flush()
}

pub fn decode11(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode11_inner(inp, orig_len, false)
}
pub fn decode11p(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode11_inner2(inp, orig_len, true, MIX10_LR)
}
pub fn decode11h(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode11_inner2(inp, orig_len, false, 13)
}
fn decode11_inner(inp: &[u8], orig_len: usize, prior: bool) -> Result<Vec<u8>, String> {
    decode11_inner2(inp, orig_len, prior, MIX10_LR)
}
fn decode11_inner2(inp: &[u8], orig_len: usize, prior: bool, lr: u32) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new(inp);
    let mut m = Tok11::new();
    let mut lm = Box::new(Mix11::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    while out.len() < orig_len {
        if rd.bit(&mut m.typ[tstate]) == 0 {
            let am = tstate & 1;
            let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                let mb = out[out.len() - reps[0] as usize];
                ((mb >> 4) as u32, (mb & 15) as u32, true)
            } else {
                (0, 0, false)
            };
            let pos = out.len();
            let prev4 = tail4_11(&out, pos);
            let t8 = tail8_11(&out, pos);
            let mm_pred = lm.mmodel.predicted(&out);
            let mut malive = mm_pred.is_some();
            let cx0 = lm.ctx18(prev4, 0, 0);
            let c30 = lm.ctx20(t8, 3, 0, 0);
            let c60 = lm.ctx20(t8, 6, 0, 0);
            let s10 = lm.ctx_sparse(t8, 0, 0, 0);
            let s20 = lm.ctx_sparse(t8, 1, 0, 0);
            let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
            let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
            let hi = dec_nib11(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
            hist = ((hist << 4) | hi as usize) & 0xffff;
            let cx1 = lm.ctx18(prev4, 1, hi);
            let c31 = lm.ctx20(t8, 3, 1, hi);
            let c61 = lm.ctx20(t8, 6, 1, hi);
            let s11 = lm.ctx_sparse(t8, 0, 1, hi);
            let s21 = lm.ctx_sparse(t8, 1, 1, hi);
            let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
            let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
            let lo = dec_nib11(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
            hist = ((hist << 4) | lo as usize) & 0xffff;
            out.push(((hi << 4) | lo) as u8);
            tstate = (tstate << 1) & 7;
            lm.byte_update(&out, out.len());
        } else {
            let is_rep = rd.bit(&mut m.is_rep[tstate & 3]) == 1;
            let (len, dist) = if is_rep {
                let k = dec_tree(&mut rd, &mut m.rep_idx, 2) as usize;
                let dist = reps[k];
                if dist == 0 {
                    return Err("rep offset before any offset".into());
                }
                (m.len_r[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH, dist)
            } else {
                let len = m.len_m[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH;
                let cls = (len - MIN_MATCH).min(3);
                let dist = dec_dist11(&mut rd, &mut m, cls)?;
                (len, dist)
            };
            rep_update(&mut reps, dist);
            let d = dist as usize;
            if d == 0 || d > out.len() || out.len() + len > orig_len {
                return Err("malformed match in dyadic stream".into());
            }
            for _ in 0..len {
                let b = out[out.len() - d];
                out.push(b);
                lm.byte_update(&out, out.len());
            }
            hist = if out.len() >= 2 {
                ((out[out.len() - 2] as usize) << 8 | out[out.len() - 1] as usize) & 0xffff
            } else {
                out[out.len() - 1] as usize & 0xffff
            };
            tstate = ((tstate << 1) | 1) & 7;
        }
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
        eprintln!("mm dec agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    Ok(out)
}


// ==================== v10: MODEL_CM10 (copy of CM9) -- the literal-only entrant ====
// No tokenize, no type/length/offset models: every byte walks the Mix11
// literal path and the MATCH MODEL carries the repeats (the remainder
// reading standing alone). The v8 Pavlov steering is unified with it: the
// steering byte is the match model's predicted byte once agreement is >= 4.
// Wins expected on text and long-range-structured files; the trial floor
// makes it free everywhere else.

pub fn encode_cm11(src: &[u8]) -> Vec<u8> {
    cm11_run(src).0
}
pub fn encode_cm11p(src: &[u8]) -> Vec<u8> {
    cm11_run_inner2(src, true, MIX10_LR).0
}
pub fn encode_cm11h(src: &[u8]) -> Vec<u8> {
    cm11_run_inner2(src, false, 13).0
}
/// v11-M5: the same loop teaches the prior -- run the CM shape over the
/// site's own book and hand back the seasoned model for export
pub fn cm11_run(src: &[u8]) -> (Vec<u8>, Box<Mix11>) {
    cm11_run_inner(src, false)
}
fn cm11_run_inner(src: &[u8], prior: bool) -> (Vec<u8>, Box<Mix11>) {
    cm11_run_inner2(src, prior, MIX10_LR)
}
fn cm11_run_inner2(src: &[u8], prior: bool, lr: u32) -> (Vec<u8>, Box<Mix11>) {
    let mut rc = REnc::new(false);
    let mut lm = Box::new(Mix11::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    let mut hist: usize = 0;
    for pos in 0..src.len() {
        let b = src[pos];
        let prev4 = tail4_11(src, pos);
        let t8 = tail8_11(src, pos);
        let mm_pred = lm.mmodel.predicted(&src[..pos]);
        let mut malive = mm_pred.is_some();
        // steering: the predicted byte, once the agreement is credible
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let hi = (b >> 4) as u32;
        let lo = (b & 15) as u32;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let s10 = lm.ctx_sparse(t8, 0, 0, 0);
        let s20 = lm.ctx_sparse(t8, 1, 0, 0);
        let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
        let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
        enc_nib11(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let s11 = lm.ctx_sparse(t8, 0, 1, hi);
        let s21 = lm.ctx_sparse(t8, 1, 1, hi);
        let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
        let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
        enc_nib11(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        lm.byte_update(src, pos + 1);
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
    }
    (rc.flush().0, lm)
}

pub fn decode_cm11(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm11_inner(inp, orig_len, false)
}
pub fn decode_cm11p(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm11_inner2(inp, orig_len, true, MIX10_LR)
}
pub fn decode_cm11h(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm11_inner2(inp, orig_len, false, 13)
}
fn decode_cm11_inner(inp: &[u8], orig_len: usize, prior: bool) -> Result<Vec<u8>, String> {
    decode_cm11_inner2(inp, orig_len, prior, MIX10_LR)
}
fn decode_cm11_inner2(inp: &[u8], orig_len: usize, prior: bool, lr: u32) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new(inp);
    let mut lm = Box::new(Mix11::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    while out.len() < orig_len {
        let pos = out.len();
        let prev4 = tail4_11(&out, pos);
        let t8 = tail8_11(&out, pos);
        let mm_pred = lm.mmodel.predicted(&out);
        let mut malive = mm_pred.is_some();
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let s10 = lm.ctx_sparse(t8, 0, 0, 0);
        let s20 = lm.ctx_sparse(t8, 1, 0, 0);
        let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
        let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
        let hi = dec_nib11(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let s11 = lm.ctx_sparse(t8, 0, 1, hi);
        let s21 = lm.ctx_sparse(t8, 1, 1, hi);
        let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
        let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
        let lo = dec_nib11(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        out.push(((hi << 4) | lo) as u8);
        lm.byte_update(&out, out.len());
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
    }
    Ok(out)
}

// ==================== v12-M2a: the 16-bit arms (MODEL 16..21) ====================
// The same walks as the v11 arms above -- LZ tokens + mixed literals
// (encode12), literal-only (encode_cm12), each with a prior-primed and a
// heavy-LR twin -- driven through Mix12 (mix12.rs: every reading kept to
// sixteen bits) and the wide coder (REnc::new_wide: p in 1..65535, the token
// counters entering as p << 4). The v11 arms stay verbatim as frozen trial
// entrants; a wide arm that loses the armored-total trial costs nothing.

#[allow(clippy::too_many_arguments)]
fn enc_nib12(
    rc: &mut REnc,
    lm: &mut Mix12,
    nib: u32,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    ci1: usize,
    ci2: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let b = (nib >> i) & 1;
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, ci1, ci2, steer, mm_exp, am);
        rc.bit_p16(b9.p, b, CAT_LIT);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
}
#[allow(clippy::too_many_arguments)]
fn dec_nib12(
    rd: &mut RDec,
    lm: &mut Mix12,
    mnib: u32,
    still: &mut bool,
    hist: usize,
    ctx18: usize,
    ctx3: usize,
    ctx6: usize,
    cs13: usize,
    cs24: usize,
    ci1: usize,
    ci2: usize,
    mm_pred: Option<(u8, u32)>,
    malive: &mut bool,
    phase: usize,
    am: usize,
) -> u32 {
    let pnib = mm_pred.map(|(pb, ml)| (if phase == 0 { (pb >> 4) as u32 } else { (pb & 15) as u32 }, ml));
    let mut node = 1usize;
    for i in (0..4).rev() {
        let steer = if *still { Some((mnib >> i) & 1) } else { None };
        let mm_exp = if *malive { pnib.map(|(pn, ml)| ((pn >> i) & 1, ml)) } else { None };
        let b9 = lm.predict(node, phase, hist, ctx18, ctx3, ctx6, cs13, cs24, ci1, ci2, steer, mm_exp, am);
        let b = rd.bit_p16(b9.p);
        lm.learn(&b9, b);
        if let Some((eb, _)) = mm_exp {
            lm.mmodel.offered += 1;
            if b == eb {
                lm.mmodel.agreed += 1;
            } else {
                *malive = false;
            }
        }
        if *still && b != (mnib >> i) & 1 {
            *still = false;
        }
        node = (node << 1) | b as usize;
    }
    (node as u32) - 16
}

/// MODEL_MIX12: LZ tokens + mixed literals at sixteen bits
pub fn encode12(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    encode12_inner(src, toks, false, MIX12_LR)
}
/// MODEL_MIX12P: the site-prior twin
pub fn encode12p(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    encode12_inner(src, toks, true, MIX12_LR)
}
/// MODEL_MIX12H: the heavy-LR twin (13, as v11's)
pub fn encode12h(src: &[u8], toks: &[Tok]) -> Vec<u8> {
    encode12_inner(src, toks, false, 13)
}
fn encode12_inner(src: &[u8], toks: &[Tok], prior: bool, lr: u32) -> Vec<u8> {
    let mut rc = REnc::new_wide();
    let mut m = Tok11::new();
    let mut lm = Box::new(Mix12::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    let mut pos = 0usize;
    for t in toks {
        match *t {
            Tok::Lit(b) => {
                rc.bit(&mut m.typ[tstate], 0, CAT_TYP);
                let am = tstate & 1;
                let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                    let mb = src[pos - reps[0] as usize];
                    ((mb >> 4) as u32, (mb & 15) as u32, true)
                } else {
                    (0, 0, false)
                };
                let prev4 = tail4_11(src, pos);
                let t8 = tail8_11(src, pos);
                let mm_pred = lm.mmodel.predicted(&src[..pos]);
                let mut malive = mm_pred.is_some();
                let hi = (b >> 4) as u32;
                let lo = (b & 15) as u32;
                let cx0 = lm.ctx18(prev4, 0, 0);
                let c30 = lm.ctx20(t8, 3, 0, 0);
                let c60 = lm.ctx20(t8, 6, 0, 0);
                let s10 = lm.ctx_sparse(t8, 0, 0, 0);
                let s20 = lm.ctx_sparse(t8, 1, 0, 0);
                let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
                let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
                enc_nib12(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
                hist = ((hist << 4) | hi as usize) & 0xffff;
                let cx1 = lm.ctx18(prev4, 1, hi);
                let c31 = lm.ctx20(t8, 3, 1, hi);
                let c61 = lm.ctx20(t8, 6, 1, hi);
                let s11 = lm.ctx_sparse(t8, 0, 1, hi);
                let s21 = lm.ctx_sparse(t8, 1, 1, hi);
                let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
                let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
                enc_nib12(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
                hist = ((hist << 4) | lo as usize) & 0xffff;
                tstate = (tstate << 1) & 7;
                pos += 1;
                lm.byte_update(src, pos);
            }
            Tok::Match { len, dist } => {
                rc.bit(&mut m.typ[tstate], 1, CAT_TYP);
                let rep_k = reps.iter().position(|&r| r == dist);
                rc.bit(&mut m.is_rep[tstate & 3], rep_k.is_some() as u32, CAT_REP);
                let v = len - MIN_MATCH as u32;
                if let Some(k) = rep_k {
                    enc_tree(&mut rc, &mut m.rep_idx, 2, k as u32, CAT_REP);
                    m.len_r[tstate & 3].encode(&mut rc, v);
                } else {
                    m.len_m[tstate & 3].encode(&mut rc, v);
                    let cls = (len as usize - MIN_MATCH).min(3);
                    enc_dist11(&mut rc, &mut m, dist, cls);
                }
                rep_update(&mut reps, dist);
                for p in pos..pos + len as usize {
                    lm.byte_update(src, p + 1);
                }
                pos += len as usize;
                hist = if pos >= 2 {
                    ((src[pos - 2] as usize) << 8 | src[pos - 1] as usize) & 0xffff
                } else {
                    src[pos - 1] as usize & 0xffff
                };
                tstate = ((tstate << 1) | 1) & 7;
            }
        }
    }
    debug_assert_eq!(pos, src.len());
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
        eprintln!("mm enc agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    rc.flush().0
}

pub fn decode12(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode12_inner(inp, orig_len, false, MIX12_LR)
}
pub fn decode12p(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode12_inner(inp, orig_len, true, MIX12_LR)
}
pub fn decode12h(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode12_inner(inp, orig_len, false, 13)
}
fn decode12_inner(inp: &[u8], orig_len: usize, prior: bool, lr: u32) -> Result<Vec<u8>, String> {
    let mut rd = RDec::new_wide(inp);
    let mut m = Tok11::new();
    let mut lm = Box::new(Mix12::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    let mut hist: usize = 0;
    let mut tstate: usize = 0;
    let mut reps: [u32; 4] = [0; 4];
    while out.len() < orig_len {
        if rd.bit(&mut m.typ[tstate]) == 0 {
            let am = tstate & 1;
            let (mhi, mlo, mut still) = if tstate & 1 == 1 && reps[0] != 0 {
                let mb = out[out.len() - reps[0] as usize];
                ((mb >> 4) as u32, (mb & 15) as u32, true)
            } else {
                (0, 0, false)
            };
            let pos = out.len();
            let prev4 = tail4_11(&out, pos);
            let t8 = tail8_11(&out, pos);
            let mm_pred = lm.mmodel.predicted(&out);
            let mut malive = mm_pred.is_some();
            let cx0 = lm.ctx18(prev4, 0, 0);
            let c30 = lm.ctx20(t8, 3, 0, 0);
            let c60 = lm.ctx20(t8, 6, 0, 0);
            let s10 = lm.ctx_sparse(t8, 0, 0, 0);
            let s20 = lm.ctx_sparse(t8, 1, 0, 0);
            let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
            let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
            let hi = dec_nib12(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
            hist = ((hist << 4) | hi as usize) & 0xffff;
            let cx1 = lm.ctx18(prev4, 1, hi);
            let c31 = lm.ctx20(t8, 3, 1, hi);
            let c61 = lm.ctx20(t8, 6, 1, hi);
            let s11 = lm.ctx_sparse(t8, 0, 1, hi);
            let s21 = lm.ctx_sparse(t8, 1, 1, hi);
            let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
            let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
            let lo = dec_nib12(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
            hist = ((hist << 4) | lo as usize) & 0xffff;
            out.push(((hi << 4) | lo) as u8);
            tstate = (tstate << 1) & 7;
            lm.byte_update(&out, out.len());
        } else {
            let is_rep = rd.bit(&mut m.is_rep[tstate & 3]) == 1;
            let (len, dist) = if is_rep {
                let k = dec_tree(&mut rd, &mut m.rep_idx, 2) as usize;
                let dist = reps[k];
                if dist == 0 {
                    return Err("rep offset before any offset".into());
                }
                (m.len_r[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH, dist)
            } else {
                let len = m.len_m[tstate & 3].decode(&mut rd)? as usize + MIN_MATCH;
                let cls = (len - MIN_MATCH).min(3);
                let dist = dec_dist11(&mut rd, &mut m, cls)?;
                (len, dist)
            };
            rep_update(&mut reps, dist);
            let d = dist as usize;
            if d == 0 || d > out.len() || out.len() + len > orig_len {
                return Err("malformed match in dyadic stream".into());
            }
            for _ in 0..len {
                let b = out[out.len() - d];
                out.push(b);
                lm.byte_update(&out, out.len());
            }
            hist = if out.len() >= 2 {
                ((out[out.len() - 2] as usize) << 8 | out[out.len() - 1] as usize) & 0xffff
            } else {
                out[out.len() - 1] as usize & 0xffff
            };
            tstate = ((tstate << 1) | 1) & 7;
        }
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
        eprintln!("mm dec agreed/offered {}/{}", lm.mmodel.agreed, lm.mmodel.offered);
    }
    Ok(out)
}

/// MODEL_CM12: literal-only at sixteen bits (the match model carries repeats)
pub fn encode_cm12(src: &[u8]) -> Vec<u8> {
    encode_cm12_inner(src, false, MIX12_LR, None, Lens::Plain)
}
/// WHICH LENS the two sparse inputs of the v12 literal model look through
/// (v13-M3c). `Plain` is the shipped CM12, byte for byte; the other two point
/// the SAME two tables and the SAME two mixer inputs at a different reading of
/// the same already-coded bytes. Nothing else in the model moves.
#[derive(Clone, Copy)]
pub enum Lens {
    Plain,
    /// WS-N: the number field tracker
    Num,
    /// WS-2D: the rectangle, at (stride, pixel)
    Grid(usize, usize),
}

/// v13-M3c (WS-N): the same v12 literal model with its TWO SPARSE INPUTS
/// re-pointed at the number field tracker (`numtext.rs`) -- field, digit
/// position, integer length, column, and the digit at the same position of the
/// previous number. Everything else, mixer and match model and APM included, is
/// the shipped CM12 verbatim, and `Lens::Plain` reproduces it to the byte.
pub fn encode_num(src: &[u8]) -> Vec<u8> {
    encode_cm12_inner(src, false, MIX12_LR, None, Lens::Num)
}
pub fn decode_num(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm12_inner(inp, orig_len, false, MIX12_LR, None, Lens::Num)
}
/// v13-M3c (WS-2D): the same model reading a rectangle. The stride and pixel
/// the encoder MEASURED ride in a five-byte header at the front of the arm's
/// own stream, so the decoder reads them and never guesses -- and it refuses
/// with a number if they are not a rectangle it can read.
pub fn encode_2d(src: &[u8], stride: u32, pixel: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() / 3 + 5);
    out.extend_from_slice(&stride.to_le_bytes());
    out.push(pixel as u8);
    out.extend_from_slice(&encode_cm12_inner(src, false, MIX12_LR, None, Lens::Grid(stride as usize, pixel as usize)));
    out
}
pub fn decode_2d(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    if inp.len() < 5 {
        return Err(format!("2D arm: {} B is shorter than its own header", inp.len()));
    }
    let stride = u32::from_le_bytes([inp[0], inp[1], inp[2], inp[3]]) as usize;
    let pixel = inp[4] as usize;
    if pixel == 0 || pixel >= stride || stride > crate::twod::MAX_STRIDE {
        return Err(format!("2D arm: stride {} pixel {} is not a rectangle", stride, pixel));
    }
    decode_cm12_inner(&inp[5..], orig_len, false, MIX12_LR, None, Lens::Grid(stride, pixel))
}
pub fn encode_cm12p(src: &[u8]) -> Vec<u8> {
    encode_cm12_inner(src, true, MIX12_LR, None, Lens::Plain)
}
pub fn encode_cm12h(src: &[u8]) -> Vec<u8> {
    encode_cm12_inner(src, false, 13, None, Lens::Plain)
}
/// MODEL_CM12_PE / MODEL_CM12_TTF: CM12 primed by a dialect book (M2c(c))
pub fn encode_cm12_book(src: &[u8], book: &Book) -> Vec<u8> {
    encode_cm12_inner(src, false, MIX12_LR, Some(book), Lens::Plain)
}
fn encode_cm12_inner(src: &[u8], prior: bool, lr: u32, book: Option<&Book>, lens: Lens) -> Vec<u8> {
    let mut nf = crate::numtext::Field::new();
    let mut rc = REnc::new_wide();
    let mut lm = Box::new(Mix12::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    if let Some(b) = book {
        lm.apply_book(b);
    }
    // v14-N2c item 2.4: the lens is a `Copy` parameter and never moves inside
    // the loop, so its three dispatches per byte are loop-invariant. `phase2d`
    // is `pos % pixel` carried as a counter -- `pos` steps by one, so the
    // per-byte hardware division goes and the key is the same value.
    let plain = matches!(lens, Lens::Plain);
    let numeric = matches!(lens, Lens::Num);
    let grid = match lens { Lens::Grid(st, px) => Some((st, px)), _ => None };
    let mut phase2d = 0usize;
    let mut hist: usize = 0;
    for pos in 0..src.len() {
        let b = src[pos];
        let (kk0, kk1) = match grid {
            Some((st, px)) => crate::twod::keys_phase(src, pos, st, px, phase2d),
            None if numeric => (nf.key0(), nf.key1()),
            None => (0, 0),
        };
        let prev4 = tail4_11(src, pos);
        let t8 = tail8_11(src, pos);
        let mm_pred = lm.mmodel.predicted(&src[..pos]);
        let mut malive = mm_pred.is_some();
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let hi = (b >> 4) as u32;
        let lo = (b & 15) as u32;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let (s10, s20) = if plain {
            (lm.ctx_sparse(t8, 0, 0, 0), lm.ctx_sparse(t8, 1, 0, 0))
        } else {
            (lm.ctx_key(kk0, 0, 0, 0), lm.ctx_key(kk1, 1, 0, 0))
        };
        let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
        let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
        enc_nib12(&mut rc, &mut lm, hi, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let (s11, s21) = if plain {
            (lm.ctx_sparse(t8, 0, 1, hi), lm.ctx_sparse(t8, 1, 1, hi))
        } else {
            (lm.ctx_key(kk0, 0, 1, hi), lm.ctx_key(kk1, 1, 1, hi))
        };
        let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
        let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
        enc_nib12(&mut rc, &mut lm, lo, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        // v14-N2c item 2.3: `nf`'s state is observable only through key0/
        // key1, which every lens but Num throws away -- so stepping it there
        // is unobservable work on 8 of the 9 CM12 arms.
        if numeric {
            nf.update(b);
        }
        if let Some((_, px)) = grid {
            phase2d += 1;
            if phase2d == px {
                phase2d = 0;
            }
        }
        lm.byte_update(src, pos + 1);
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash enc {:016x}", lm.state_hash());
    }
    rc.flush().0
}

pub fn decode_cm12(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm12_inner(inp, orig_len, false, MIX12_LR, None, Lens::Plain)
}
pub fn decode_cm12p(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm12_inner(inp, orig_len, true, MIX12_LR, None, Lens::Plain)
}
pub fn decode_cm12h(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    decode_cm12_inner(inp, orig_len, false, 13, None, Lens::Plain)
}
pub fn decode_cm12_book(inp: &[u8], orig_len: usize, book: &Book) -> Result<Vec<u8>, String> {
    decode_cm12_inner(inp, orig_len, false, MIX12_LR, Some(book), Lens::Plain)
}
fn decode_cm12_inner(inp: &[u8], orig_len: usize, prior: bool, lr: u32, book: Option<&Book>, lens: Lens) -> Result<Vec<u8>, String> {
    let mut nf = crate::numtext::Field::new();
    let mut rd = RDec::new_wide(inp);
    let mut lm = Box::new(Mix12::new());
    lm.set_lr(lr);
    if prior {
        lm.apply_prior();
    }
    if let Some(b) = book {
        lm.apply_book(b);
    }
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    // v14-N2c item 2.4: the lens is a `Copy` parameter and never moves inside
    // the loop, so its three dispatches per byte are loop-invariant. `phase2d`
    // is `pos % pixel` carried as a counter -- `pos` steps by one, so the
    // per-byte hardware division goes and the key is the same value.
    let plain = matches!(lens, Lens::Plain);
    let numeric = matches!(lens, Lens::Num);
    let grid = match lens { Lens::Grid(st, px) => Some((st, px)), _ => None };
    let mut phase2d = 0usize;
    let mut hist: usize = 0;
    while out.len() < orig_len {
        let pos = out.len();
        let (kk0, kk1) = match grid {
            Some((st, px)) => crate::twod::keys_phase(&out, pos, st, px, phase2d),
            None if numeric => (nf.key0(), nf.key1()),
            None => (0, 0),
        };
        let prev4 = tail4_11(&out, pos);
        let t8 = tail8_11(&out, pos);
        let mm_pred = lm.mmodel.predicted(&out);
        let mut malive = mm_pred.is_some();
        let (mhi, mlo, steer0, am) = match mm_pred {
            Some((pb, ml)) if ml >= 4 => ((pb >> 4) as u32, (pb & 15) as u32, true, 1),
            Some(_) => (0, 0, false, 1),
            None => (0, 0, false, 0),
        };
        let mut still = steer0;
        let cx0 = lm.ctx18(prev4, 0, 0);
        let c30 = lm.ctx20(t8, 3, 0, 0);
        let c60 = lm.ctx20(t8, 6, 0, 0);
        let (s10, s20) = if plain {
            (lm.ctx_sparse(t8, 0, 0, 0), lm.ctx_sparse(t8, 1, 0, 0))
        } else {
            (lm.ctx_key(kk0, 0, 0, 0), lm.ctx_key(kk1, 1, 0, 0))
        };
        let i10c = lm.ctx_ind1((t8 & 0xff) as u8, 0, 0);
        let i20c = lm.ctx_ind2((t8 & 0xffff) as usize, 0, 0);
        let hi = dec_nib12(&mut rd, &mut lm, mhi, &mut still, hist, cx0, c30, c60, s10, s20, i10c, i20c, mm_pred, &mut malive, 0, am);
        hist = ((hist << 4) | hi as usize) & 0xffff;
        let cx1 = lm.ctx18(prev4, 1, hi);
        let c31 = lm.ctx20(t8, 3, 1, hi);
        let c61 = lm.ctx20(t8, 6, 1, hi);
        let (s11, s21) = if plain {
            (lm.ctx_sparse(t8, 0, 1, hi), lm.ctx_sparse(t8, 1, 1, hi))
        } else {
            (lm.ctx_key(kk0, 0, 1, hi), lm.ctx_key(kk1, 1, 1, hi))
        };
        let i11c = lm.ctx_ind1((t8 & 0xff) as u8, 1, hi);
        let i21c = lm.ctx_ind2((t8 & 0xffff) as usize, 1, hi);
        let lo = dec_nib12(&mut rd, &mut lm, mlo, &mut still, hist, cx1, c31, c61, s11, s21, i11c, i21c, mm_pred, &mut malive, 1, am);
        hist = ((hist << 4) | lo as usize) & 0xffff;
        out.push(((hi << 4) | lo) as u8);
        // v14-N2c item 2.3: `nf`'s state is observable only through key0/
        // key1, which every lens but Num throws away -- so stepping it there
        // is unobservable work on 8 of the 9 CM12 arms.
        if numeric {
            nf.update(((hi << 4) | lo) as u8);
        }
        if let Some((_, px)) = grid {
            phase2d += 1;
            if phase2d == px {
                phase2d = 0;
            }
        }
        lm.byte_update(&out, out.len());
    }
    if std::env::var_os("EGG_STATEHASH").is_some() {
        eprintln!("statehash dec {:016x}", lm.state_hash());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the 16-bit coder at its extremes: p = 1 and p = 65535 on both bit
    /// values, mixed with 12-bit token counters (p << 4), round-trips; and the
    /// range never leaves [2^24, 2^32) between decisions
    #[test]
    fn wide_coder_round_trip_extremes() {
        let mut rc = REnc::new_wide();
        let mut tok = [PINIT; 4];
        let mut bits: Vec<(u16, u32)> = Vec::new();
        let mut st = 0x9E37u64;
        for i in 0..20_000u32 {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            let p: u16 = match i % 7 {
                0 => 1,
                1 => 65535,
                2 => 2,
                3 => 65534,
                _ => ((st >> 20) as u16).clamp(1, 65535),
            };
            let b = if i % 5 == 0 { (st & 1) as u32 } else { (p < 32768) as u32 };
            bits.push((p, b));
            rc.bit_p16(p, b, CAT_LIT);
            assert!(rc.range >= TOP);
            rc.bit(&mut tok[(i & 3) as usize], b, CAT_TYP);
            assert!(rc.range >= TOP);
        }
        let (bytes, _) = rc.flush();
        let mut rd = RDec::new_wide(&bytes);
        let mut tok = [PINIT; 4];
        for (i, &(p, b)) in bits.iter().enumerate() {
            assert_eq!(rd.bit_p16(p), b, "wide bit_p16 #{i} p={p}");
            assert_eq!(rd.bit(&mut tok[i & 3]), b, "wide token bit #{i}");
        }
    }

    /// the v12 arms round-trip (mirror by construction, asserted): LZ+mixed,
    /// literal-only, and the prior/heavy twins, on a structured buffer
    #[test]
    fn v12_arms_round_trip() {
        let mut src = Vec::new();
        let words: [&[u8]; 5] = [b"the remainder ", b"names the wound; ", b"clean means zero. ", b"\x00\x01\x02\x03\x7f\xff", b"kept rather than rounded away\n"];
        let mut st = 0x1489u64;
        for _ in 0..3000 {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            src.extend_from_slice(words[(st % 5) as usize]);
            if st.is_multiple_of(11) {
                src.push((st >> 40) as u8);
            }
        }
        let toks = crate::token::tokenize(&src);
        let e = encode12(&src, &toks);
        assert_eq!(decode12(&e, src.len()).unwrap(), src, "MIX12");
        let e = encode12p(&src, &toks);
        assert_eq!(decode12p(&e, src.len()).unwrap(), src, "MIX12P");
        let e = encode12h(&src, &toks);
        assert_eq!(decode12h(&e, src.len()).unwrap(), src, "MIX12H");
        let e = encode_cm12(&src);
        assert_eq!(decode_cm12(&e, src.len()).unwrap(), src, "CM12");
        let e = encode_cm12p(&src);
        assert_eq!(decode_cm12p(&e, src.len()).unwrap(), src, "CM12P");
        let e = encode_cm12h(&src);
        assert_eq!(decode_cm12h(&e, src.len()).unwrap(), src, "CM12H");
        // a damaged stream must not decode to the source (the FNV gate lives
        // above this layer; here we only assert the walk is not trivially lenient)
        let mut bad = encode_cm12(&src);
        let mid = bad.len() / 2;
        bad[mid] ^= 0x55;
        let d = decode_cm12(&bad, src.len());
        assert!(d.is_err() || d.unwrap() != src);
    }

    /// the big-arena regression (2026-09-01): distances at and past 2^26 must
    /// round-trip through every tier's slot codec. All three guards sat at 25
    /// for four generations.
    #[test]
    fn dist_slots_round_trip_past_2_26() {
        let dists: Vec<u32> = vec![
            1, 2, 5, 100, 4096, (1 << 25) + 3, (1 << 26) - 1, 1 << 26,
            (1 << 26) + 12345, (1 << 28) + 7, (1 << 30) + 1, u32::MAX,
        ];
        {
            let mut rc = REnc::new(false);
            let mut m = Models::new(8);
            for &d in &dists {
                enc_dist(&mut rc, &mut m, d, (d as usize) & 3);
            }
            let (bytes, _) = rc.flush();
            let mut rd = RDec::new(&bytes);
            let mut m = Models::new(8);
            for &d in &dists {
                assert_eq!(dec_dist(&mut rd, &mut m, (d as usize) & 3).unwrap(), d, "v8 tier dist {}", d);
            }
        }
        {
            let mut rc = REnc::new(false);
            let mut m = Tok9::new();
            for &d in &dists {
                enc_dist9(&mut rc, &mut m, d, (d as usize) & 3);
            }
            let (bytes, _) = rc.flush();
            let mut rd = RDec::new(&bytes);
            let mut m = Tok9::new();
            for &d in &dists {
                assert_eq!(dec_dist9(&mut rd, &mut m, (d as usize) & 3).unwrap(), d, "v9 tier dist {}", d);
            }
        }
        {
            let mut rc = REnc::new(false);
            let mut m = Tok10::new();
            for &d in &dists {
                enc_dist10(&mut rc, &mut m, d, (d as usize) & 3);
            }
            let (bytes, _) = rc.flush();
            let mut rd = RDec::new(&bytes);
            let mut m = Tok10::new();
            for &d in &dists {
                assert_eq!(dec_dist10(&mut rd, &mut m, (d as usize) & 3).unwrap(), d, "v10 tier dist {}", d);
            }
        }
    }
}
