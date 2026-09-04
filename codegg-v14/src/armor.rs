//! armor.rs -- eggv13 armor v4: THE REMAINDER (v12's armor, unmoved).
//!
//! The charter verse (spec.md:134-156): "Division -- quotient, multiplier,
//! remainder ... R is a stalk too, always inside ... Widening the grid grows
//! Q and shrinks R. The identity never moves." The armored form IS that
//! identity in GF(2^16)[x]:
//!
//!     A * x^t = Q * B + R        the file is A, the generator is B, the armor is R
//!
//! v11 divided by a SMALL B many times (RS groups <= 248 over GF(256)) and
//! paid R per group: the price of survival scaled with the file. v12 divides
//! ONCE by a wide B: one systematic Reed-Solomon codeword over GF(2^16) in the
//! BCH view, g(x) = prod_(i=0..t-1) (x - alpha^i), the square of blk bytes is
//! blk/2 symbols and symbol j of every square is codeword j (blk/2 interleaved
//! codewords of length n = s + c + t <= 65,535). R is exactly t squares:
//! t = dead(blk) = ceil(4096/blk)+1 by default, `--survive <bytes>` dials it.
//!
//! Three scales remain, each with one job:
//!   * the u16 residue per square (square as a big-endian number mod 65,519;
//!     ord(2) = 32,759 and -1 is never reached, so +-2^k are distinct over
//!     4,094-byte spans) CONVICTS a square -- it never repairs (bit_repair is
//!     retired: no mimic-confirmation problem, R does every repair);
//!   * the codeword REBUILDS: any <= t dead squares anywhere, located by
//!     residue or by address, are erasures; Forney fills them exactly;
//!     squares the residues could not judge are located by the syndromes
//!     (collaboratively across the interleaved codewords, then per codeword by
//!     Berlekamp-Massey + Chien), at the classical cost of two parities each;
//!   * the three sites (head / mid / end) carry the header and the meta
//!     (residues + fnv32); any checksum-verified copy wins, two verified
//!     copies that disagree REFUSE, byte-vote is the last resort.
//!
//! The check table (CT) is either TRIPLICATED (the meta IS the residue table;
//! n = s + t) or IN-CODEWORD (c = ceil(2s/blk) CT squares carry the s data
//! residues and are codeword members; the meta carries the CT and parity
//! squares' residues -- the parity squares' residues cannot ride inside the
//! CT squares because the parity is a function of them) or ABSENT (v12-M2b,
//! placement "none": P = t + 1 parity squares and no residues at all -- the
//! interleaved codewords locate blind wounds jointly, so the 2 B/square table
//! was paying for what the syndromes give for free; the price is then flat
//! per tier, 4,812 / 5,324 / 6,348 / 8,396). The grid 256/512/1024/2048
//! (+4096 only when n would exceed 65,535 at 2048) and the three CT
//! placements are searched by argmin of the total (`--judge` restricts the
//! argmin to the residue placements; `--ct` forces one). The square order is
//! data | parity | CT so that no contiguous <= t-square wound can hit a data
//! square and the CT square that judges it. The last data square is kept
//! short (glossary.js:164 "Kept rather than rounded away").
//!
//! The decoding ladder: (1) residues + `--wound` + truncation => erasure set
//! E, |E| <= t => erasure decode; (2) syndromes of the result all zero AND
//! FNV-64 => EXACT; (3) syndromes nonzero => the interleaved codewords locate
//! the unjudged squares jointly, then Berlekamp-Massey per codeword within
//! 2e + |E| <= t; (4) beyond => the data squares as received are hashed (rung
//! C: damage confined to parity/CT leaves the inner intact), else REFUSE with
//! the number. Wrong data NEVER.
//!
//! Attribution: Reed & Solomon 1960 (the code); Berlekamp-Massey (the
//! locator); Forney (the values); Chien (the search); collaborative decoding
//! of interleaved RS codes (Krachkovsky & Lee 1997; Bleichenbacher, Kiayias &
//! Yung 2003); residues Avizienis 1971 / Mandelbaum 1976; FNV by
//! Fowler-Noll-Vo. The site supplied the verse and the nesting.

use std::collections::BTreeSet;
use std::sync::OnceLock;

pub const HDR: usize = 64;
/// the container magic and format version this build WRITES
/// v14-N1's instrument: how often the consistency check is reached, how many
/// bad codewords it was handed, how many it actually READ (the `take(4)`), and
/// how often it rejected. Printed by `audit` under EGG_CONSIST.
pub static CONSIST_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CONSIST_BAD: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CONSIST_READ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CONSIST_REJECT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// which CALLER reached it: the partial-location path (the one the inherited
/// note calls vacuous) or `confirm_subset`'s brute search over subsets (the one
/// the note prices at ~32 expected false accepts)
pub static CONSIST_VIA_PARTIAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CONSIST_VIA_BRUTE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CONFIRM_SUBSET_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CONFIRM_SUBSET_HITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// the joint locator's false-positive traffic: how often it returned MORE
/// candidate positions than the rank it solved for (which is the only way the
/// subset search is ever entered), the largest excess seen, and how many
/// subsets the search actually had to try
pub static FOUND_GT_K: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static FOUND_EQ_K: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static FOUND_EXCESS_MAX: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static SUBSET_TRIES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// the locator's actual arithmetic, measured instead of assumed: m, the rank it
/// solved for, and how many positions it scanned. The false-positive rate is
/// ~n/65536^(m-k), so m-k is the whole story and it is worth printing.
pub static LOC_M_SUM: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LOC_K_SUM: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LOC_N_SUM: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LOC_RUNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LOC_MK_MIN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

pub const MAGIC: &[u8; 4] = b"EG14";
pub const FORMAT_VERSION: u8 = 8;
/// v13's magic and version, READ by this same armor v4 path -- the armor did
/// not move at v14, only the name did
pub const MAGIC_V13: &[u8; 4] = b"EG13";
/// the v12 magic this build still READS (armor v4 did not move at v13)
pub const MAGIC_V12: &[u8; 4] = b"EG12";
/// the searched grid; index = header byte 53
pub const TIERS: [usize; 5] = [256, 512, 1024, 2048, 4096];
pub const MODULUS: u32 = 65_519;
pub const MODULUS_ID: u8 = 1;
/// GF(2^16) primitive polynomial x^16 + x^12 + x^3 + x + 1
pub const POLY: u32 = 0x1100B;
pub const FIELD_ORDER: usize = 65_535;
/// codeword length bound: n <= 2^16 - 1
pub const NMAX: usize = 65_535;
pub const SURVIVE_DEFAULT: usize = 4096;
/// header byte 30 is a u8
pub const TMAX: usize = 255;
/// candidate cap for the k == m brute force (pairs are quadratic)
const BRUTE_MAX: usize = 2048;

/// most squares a contiguous 4096-byte scratch can straddle at this square size
pub fn dead_slots(blk: usize) -> usize {
    4096usize.div_ceil(blk) + 1
}
/// squares a contiguous wound of `survive` bytes can straddle
pub fn t_for(blk: usize, survive: usize) -> usize {
    survive.div_ceil(blk) + 1
}
pub fn tier_index(blk: usize) -> usize {
    TIERS.iter().position(|&b| b == blk).expect("unknown block tier")
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

// ---------- the residue: square as a big-endian number mod 65,519 ----------
pub fn residue(sq: &[u8]) -> u16 {
    let m = MODULUS as u64;
    let mut r: u64 = 0;
    let (chunks, rem) = sq.as_chunks::<4>();
    for c in chunks {
        let w = u32::from_be_bytes(*c) as u64;
        r = ((r << 32) | w) % m;
    }
    for &b in rem {
        r = ((r << 8) | b as u64) % m;
    }
    r as u16
}

// ---------- GF(2^16) ----------
pub struct Gf16 {
    exp: Vec<u16>, // 2 * 65535 entries: alpha^i, doubled so a+b never wraps
    log: Vec<u16>, // 65536 entries; log[0] is a GUARD (never read by mul/inv)
}
static GF: OnceLock<Gf16> = OnceLock::new();
pub fn gf() -> &'static Gf16 {
    GF.get_or_init(Gf16::build)
}
impl Gf16 {
    fn build() -> Gf16 {
        let mut exp = vec![0u16; 2 * FIELD_ORDER];
        let mut log = vec![0u16; 1 << 16];
        let mut seen = vec![false; 1 << 16];
        let mut v: u32 = 1;
        for (i, e) in exp.iter_mut().enumerate().take(FIELD_ORDER) {
            assert!(!seen[v as usize], "alpha is not primitive: alpha^{} repeats", i);
            seen[v as usize] = true;
            *e = v as u16;
            log[v as usize] = i as u16;
            v <<= 1;
            if v & 0x10000 != 0 {
                v ^= POLY;
            }
        }
        assert_eq!(v, 1, "alpha's order must be exactly 65,535");
        for i in FIELD_ORDER..2 * FIELD_ORDER {
            exp[i] = exp[i - FIELD_ORDER];
        }
        Gf16 { exp, log }
    }
    #[inline]
    pub fn mul(&self, a: u16, b: u16) -> u16 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
    #[inline]
    pub fn inv(&self, a: u16) -> u16 {
        assert!(a != 0, "inverse of zero");
        self.exp[FIELD_ORDER - self.log[a as usize] as usize]
    }
    #[inline]
    pub fn div(&self, a: u16, b: u16) -> u16 {
        self.mul(a, self.inv(b))
    }
    /// alpha^e for any e (reduced mod 65,535)
    #[inline]
    pub fn alpha(&self, e: usize) -> u16 {
        self.exp[e % FIELD_ORDER]
    }
    /// alpha^(-e)
    #[inline]
    pub fn alpha_inv(&self, e: usize) -> u16 {
        self.exp[(FIELD_ORDER - e % FIELD_ORDER) % FIELD_ORDER]
    }
    #[inline]
    pub fn log_of(&self, a: u16) -> usize {
        debug_assert!(a != 0);
        self.log[a as usize] as usize
    }
    #[inline]
    pub fn exp_at(&self, i: usize) -> u16 {
        self.exp[i]
    }
    /// FNV-64 of the two tables, printed by the audit
    pub fn table_fnv(&self) -> u64 {
        let mut bytes = Vec::with_capacity(2 * (self.exp.len() + self.log.len()));
        for &e in self.exp.iter().chain(self.log.iter()) {
            bytes.extend_from_slice(&e.to_le_bytes());
        }
        fnv64(&bytes)
    }
    // polynomials: little-endian coefficient vectors (p[i] is the x^i term)
    pub fn poly_mul(&self, a: &[u16], b: &[u16]) -> Vec<u16> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut out = vec![0u16; a.len() + b.len() - 1];
        for (i, &ai) in a.iter().enumerate() {
            if ai == 0 {
                continue;
            }
            for (j, &bj) in b.iter().enumerate() {
                out[i + j] ^= self.mul(ai, bj);
            }
        }
        out
    }
    /// a * b mod x^k
    pub fn poly_mul_mod(&self, a: &[u16], b: &[u16], k: usize) -> Vec<u16> {
        let mut out = vec![0u16; k];
        for (i, &ai) in a.iter().enumerate().take(k) {
            if ai == 0 {
                continue;
            }
            for (j, &bj) in b.iter().enumerate().take(k - i) {
                out[i + j] ^= self.mul(ai, bj);
            }
        }
        out
    }
    pub fn poly_eval(&self, p: &[u16], x: u16) -> u16 {
        let mut acc = 0u16;
        for &c in p.iter().rev() {
            acc = self.mul(acc, x) ^ c;
        }
        acc
    }
    /// formal derivative in characteristic 2: only the odd terms survive
    pub fn poly_deriv(&self, p: &[u16]) -> Vec<u16> {
        let mut d = vec![0u16; p.len().saturating_sub(1).max(1)];
        for (i, &c) in p.iter().enumerate().skip(1) {
            if i % 2 == 1 {
                d[i - 1] = c;
            }
        }
        d
    }
}
/// g(x) = prod_(i=0..t-1) (x - alpha^i), coefficients g_0..g_t (g_t = 1)
pub fn generator(t: usize) -> Vec<u16> {
    let f = gf();
    let mut g = vec![1u16];
    for i in 0..t {
        g = f.poly_mul(&g, &[f.alpha(i), 1]);
    }
    g
}

// ---------- geometry ----------
// armored layout v4 (three spread sites, one square region, last data square short):
//   [hdr0 meta0][squares 0..mid)[hdr1 meta1][squares mid..n)[meta2 hdr2]
//   squares (index order): data (s, the last one short) | parity (t) | CT (c, in-codeword only)
//   stream order: the SHORT data square rides first, right after the head site,
//   then data 0.., parity, CT -- audit (a) found that a short square inside the
//   run lets a 4,096 B wound reach dead(blk)+1 squares (a 1-byte square costs
//   the wound almost nothing); at the head nothing precedes it but the site
//   meta copy = [residues][fnv32]; residues of squares [meta_from..n)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CtMode {
    Triple = 0,     // the residue table IS the meta, stored at the three sites
    InCodeword = 1, // c CT squares carry the data residues; they are codeword members
    /// v12-M2b, placement "none" (the columns agree): NO residue table. A
    /// wounded square is an error at the same position in every interleaved
    /// codeword, so their syndromes share one locator and the codewords locate
    /// blind wounds jointly (Krachkovsky & Lee 1997; Bleichenbacher, Kiayias &
    /// Yung 2003). P = t + 1 parity squares; the meta is an fnv32 of nothing.
    Absent = 2,
}
impl CtMode {
    pub fn from_byte(b: u8) -> Option<CtMode> {
        match b {
            0 => Some(CtMode::Triple),
            1 => Some(CtMode::InCodeword),
            2 => Some(CtMode::Absent),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            CtMode::Triple => "CT x3",
            CtMode::InCodeword => "CT in-codeword",
            CtMode::Absent => "CT none",
        }
    }
    /// parses `--ct triple|incw|none` (and the header byte's digits)
    pub fn parse(s: &str) -> Option<CtMode> {
        match s {
            "triple" | "0" => Some(CtMode::Triple),
            "incw" | "1" => Some(CtMode::InCodeword),
            "none" | "2" => Some(CtMode::Absent),
            _ => None,
        }
    }
    /// the parity count this placement asks for at a promise: the residue
    /// placements need dead(blk) squares (residues locate, the code rebuilds);
    /// placement none needs one more, so a blind contiguous wound of dead(blk)
    /// squares stays within the joint locator's reach (e <= P - 1)
    pub fn parity_for(self, blk: usize, survive: usize) -> usize {
        t_for(blk, survive) + (self == CtMode::Absent) as usize
    }
}
/// the conservation check rides in the voted header: what the ORIGINAL bytes
/// were (length + FNV-64), which model transmuted them, and which filter
/// respelled them first.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Extras {
    pub orig_len: u64,
    pub orig_fnv: u64,
    pub model: u8,
    pub filter_id: u8,
    pub filter_param: u32,
}
#[derive(Clone, Debug)]
pub struct Geom {
    pub len: usize,
    pub blk: usize,
    pub t: usize, // parity squares (0 = no armor)
    pub mode: CtMode,
    pub s: usize,   // data squares
    pub c: usize,   // CT squares (in-codeword) or 0
    pub n: usize,   // s + t + c: the codeword length
    pub pad: usize, // s*blk - len: the short last square's missing bytes
    pub m: usize,   // meta body bytes = 2 * (n - meta_from)
    pub msize: usize, // m + 4 (fnv32)
    pub mid: usize, // square index the [hdr1 meta1] site sits before
    pub hash: u64,  // FNV-64 of the inner (stage-local truth)
    pub ex: Extras,
    pub total: usize,
}
impl Geom {
    /// first square whose residue lives in the meta
    pub fn meta_from(&self) -> usize {
        match self.mode {
            CtMode::Triple => 0,
            CtMode::InCodeword => self.s,
            CtMode::Absent => self.n, // no residues anywhere
        }
    }
    /// square index of the first parity square
    pub fn parity_at(&self) -> usize {
        self.s
    }
    /// square index of the first CT square (in-codeword only)
    pub fn ct_at(&self) -> usize {
        self.s + self.t
    }
    pub fn is_data(&self, j: usize) -> bool {
        j < self.s
    }
    pub fn is_parity(&self, j: usize) -> bool {
        j >= self.s && j < self.s + self.t
    }
    pub fn is_ct(&self, j: usize) -> bool {
        j >= self.s + self.t
    }
    /// which CT square (absolute index) and which symbol hold data square i's residue
    pub fn ct_slot(&self, i: usize) -> (usize, usize) {
        let l = self.blk / 2;
        (self.ct_at() + i / l, i % l)
    }
}
pub fn geom(len: usize, blk: usize, t: usize, mode: CtMode, hash: u64, ex: Extras) -> Geom {
    let s = len.div_ceil(blk);
    let pad = s * blk - len;
    let c = match mode {
        CtMode::Triple | CtMode::Absent => 0,
        CtMode::InCodeword => (2 * s).div_ceil(blk),
    };
    let n = s + t + c;
    let m = match mode {
        CtMode::Triple => 2 * n,
        CtMode::InCodeword => 2 * (t + c),
        CtMode::Absent => 0,
    };
    let msize = m + 4;
    let total = 3 * (HDR + msize) + n * blk - pad;
    let mid = if n == 0 { 0 } else { (n / 2).max(1) };
    Geom { len, blk, t, mode, s, c, n, pad, m, msize, mid, hash, ex, total }
}
/// the 4 KB-scratch claim: t parity squares cover dead(blk) straddled squares;
/// under placement none the blind claim needs one more (the joint locator
/// reaches P - 1)
pub fn scratch_guaranteed(g: &Geom) -> bool {
    match g.mode {
        CtMode::Absent => g.t > dead_slots(g.blk),
        _ => g.t > 0 && g.t >= dead_slots(g.blk),
    }
}
pub struct Off {
    pub h0: usize,
    pub m0: usize,
    pub h1: usize,
    pub m1: usize,
    pub m2: usize,
    pub h2: usize,
    pub slot_base: usize, // byte offset of square 0
}
pub fn offsets(g: &Geom) -> Off {
    let site = HDR + g.msize;
    let h1 = site + g.mid * g.blk - if g.mid >= 1 { g.pad } else { 0 };
    let m2 = 2 * site + g.n * g.blk - g.pad;
    Off { h0: 0, m0: HDR, h1, m1: h1 + HDR, m2, h2: m2 + g.msize, slot_base: site }
}
/// stream position of square j: the short data square (s-1) is stream
/// position 0; data 0..s-1 follow at 1..s; parity and CT keep their index
pub fn stream_pos(g: &Geom, j: usize) -> usize {
    if g.s == 0 || j >= g.s {
        j
    } else if j == g.s - 1 {
        0
    } else {
        j + 1
    }
}
/// byte offset of square j
pub fn square_off(g: &Geom, j: usize) -> usize {
    let site = HDR + g.msize;
    let p = stream_pos(g, j);
    site + p * g.blk - if p >= 1 { g.pad } else { 0 } + if p >= g.mid { site } else { 0 }
}
/// stored bytes of square j (the last data square is short)
pub fn square_len(g: &Geom, j: usize) -> usize {
    if g.s > 0 && j == g.s - 1 { g.blk - g.pad } else { g.blk }
}

// ---------- the price and the promise ----------
pub struct Price {
    pub parity: usize,
    pub ct: usize,
    pub sites: usize,
    pub total: usize,
}
pub fn price(g: &Geom) -> Price {
    let parity = g.t * g.blk;
    let ct = g.c * g.blk;
    let sites = 3 * (HDR + g.msize);
    Price { parity, ct, sites, total: parity + ct + sites }
}
fn commas(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}
/// the promise, with its numbers (spectrometer.html:396 -- refusing with a
/// number becomes promising with a number)
pub fn promise(g: &Geom) -> String {
    let p = price(g);
    if g.t == 0 {
        return format!(
            "no armor: detects damage (residues + hashes), repairs nothing; price {} B = 0 parity + 0 CT + {} sites",
            commas(p.total),
            commas(p.sites)
        );
    }
    if g.mode == CtMode::Absent {
        // v13-M2(0): the certain blind reach is the PER-CODEWORD rung's, and that
        // rung refuses unless 2*deg < t (an unverified locator is a guess), so it
        // is (t-1)/2 and not t/2. Everything above it is the JOINT rung, which is
        // conditional on the error rows being independent -- measured: a rank-1
        // dependent wound of (t-1)/2 + 1 squares REFUSES at every tier, and an
        // ordinary one of t-1 squares is EXACT at every tier. The promise says
        // which rung carries which band, because the promise is the product.
        let bm = g.t.saturating_sub(1) / 2;
        let joint = if bm < g.t.saturating_sub(1) {
            format!(
                "{} .. {} located jointly by the {} codewords, which succeeds iff the error rows are independent -- overwhelming for compressed squares, and a dependent wound REFUSES with the number rather than guessing; ANY contiguous blind run up to {} B rides that same joint rung",
                bm + 1,
                g.t - 1,
                g.blk / 2,
                commas(g.t.saturating_sub(2) * g.blk)
            )
        } else {
            "there is no joint band at this parity count".to_string()
        };
        return format!(
            "placement none: no residue table, the {} interleaved codewords locate blind wounds jointly (Krachkovsky-Lee 1997). NAMED (by address or truncation): any {} squares ({} B) anywhere, CERTAIN; BLIND: any {} squares CERTAIN (Berlekamp-Massey per codeword, the rung that assumes nothing); {}; {} blind REFUSES; price {} B = {} parity + 0 CT + {} sites (floor 4,096; {:.2}x)",
            g.blk / 2,
            g.t,
            commas(g.t * g.blk),
            bm,
            joint,
            g.t,
            commas(p.total),
            commas(p.parity),
            commas(p.sites),
            p.total as f64 / 4096.0
        );
    }
    let contiguous = (g.t - 1) * g.blk;
    let blind = match g.mode {
        CtMode::Triple => "blind or named (the residues ride in the three sites)".to_string(),
        CtMode::InCodeword => format!(
            "named by address or residue; blind: any contiguous run, any {} scattered (a dead CT square hides {} residues; the codewords locate what it hid within their reach, then REFUSE with the number)",
            g.t - 1,
            g.blk / 2
        ),
        CtMode::Absent => unreachable!(),
    };
    format!(
        "survives any {} squares ({} B) anywhere, {}; contiguous >= {} B; price {} B = {} parity + {} CT + {} sites (floor 4,096; {:.2}x)",
        g.t,
        commas(g.t * g.blk),
        blind,
        commas(contiguous),
        commas(p.total),
        commas(p.parity),
        commas(p.ct),
        commas(p.sites),
        p.total as f64 / 4096.0
    )
}

// ---------- the rib policy: argmin over the grid and the CT placement ----------
#[derive(Clone, Debug)]
pub struct Rib {
    pub blk: usize,
    pub t: usize,
    pub mode: CtMode,
    pub total: usize,
    pub n: usize,
    pub note: Option<String>,
}
fn dummy_ex() -> Extras {
    Extras { orig_len: 0, orig_fnv: 0, model: 0, filter_id: 0, filter_param: 0 }
}
/// the searched grid: every tier x the three CT placements, strict less-than
/// (smallest square and CT x3 win ties), n <= 65,535; the 4096 tier only
/// when 2048 cannot hold the codeword, with the reason in `note`.
/// `force_blk` / `force_t` pin one axis (drills, `--tier`, `--parity`).
pub fn rib_search(len: usize, survive: usize, force_blk: Option<usize>, force_t: Option<usize>) -> Result<Rib, String> {
    rib_search_with(len, survive, force_blk, force_t, true)
}
/// `allow_none = false` is `--judge`: the argmin over the residue placements only
pub fn rib_search_with(len: usize, survive: usize, force_blk: Option<usize>, force_t: Option<usize>, allow_none: bool) -> Result<Rib, String> {
    let grid: Vec<usize> = match force_blk {
        Some(b) => {
            if !TIERS.contains(&b) {
                return Err(format!("--tier {}: not on the grid {:?}", b, TIERS));
            }
            vec![b]
        }
        None => TIERS[..4].to_vec(),
    };
    let pick = |grid: &[usize]| -> Option<Rib> {
        let mut best: Option<Rib> = None;
        for &blk in grid {
            for mode in [CtMode::Triple, CtMode::InCodeword, CtMode::Absent] {
                if mode == CtMode::Absent && !allow_none {
                    continue;
                }
                // --parity pins the parity count for every placement; the
                // default gives placement none its one extra square
                let t = force_t.unwrap_or_else(|| mode.parity_for(blk, survive));
                if t > TMAX {
                    continue;
                }
                let g = geom(len, blk, t, mode, 0, dummy_ex());
                if g.n > NMAX {
                    continue;
                }
                if best.as_ref().is_none_or(|b| g.total < b.total) {
                    best = Some(Rib { blk, t, mode, total: g.total, n: g.n, note: None });
                }
            }
        }
        best
    };
    if let Some(r) = pick(&grid) {
        return Ok(r);
    }
    if force_blk.is_none() {
        if let Some(mut r) = pick(&[4096]) {
            r.note = Some(
                "the codeword would exceed 65,535 squares at 2048; the 4096 tier is taken (its residue's 2-bit certainty holds within 4,094-byte spans of a square, not the whole square)"
                    .into(),
            );
            return Ok(r);
        }
    }
    Err(format!(
        "no geometry on the grid holds {} B with --survive {} (t <= 255 squares and n <= 65,535 squares)",
        len, survive
    ))
}
/// the default policy: survive 4,096 contiguous bytes
pub fn rib_policy(len: usize) -> Rib {
    rib_search(len, SURVIVE_DEFAULT, None, None).expect("the grid always holds a default geometry")
}
/// `--no-armor`: t = 0, residues triplicated (they convict, nothing repairs)
pub fn rib_no_armor(len: usize) -> Rib {
    let mut best: Option<Rib> = None;
    for &blk in &TIERS {
        let g = geom(len, blk, 0, CtMode::Triple, 0, dummy_ex());
        if g.n > NMAX {
            continue;
        }
        if best.as_ref().is_none_or(|b| g.total < b.total) {
            best = Some(Rib { blk, t: 0, mode: CtMode::Triple, total: g.total, n: g.n, note: None });
        }
    }
    best.expect("some tier holds the naked form")
}

// ---------- headers ----------
fn write_header(g: &Geom, out: &mut [u8]) {
    out[0..4].copy_from_slice(MAGIC);
    out[4] = FORMAT_VERSION;
    out[5] = 0; // v3's group size: unused, one codeword now
    out[6..8].copy_from_slice(&(MODULUS as u16).to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..18].copy_from_slice(&(g.len as u64).to_le_bytes());
    out[18..22].copy_from_slice(&(g.s as u32).to_le_bytes());
    out[22..30].copy_from_slice(&g.hash.to_le_bytes());
    out[30] = g.t as u8;
    out[31] = g.ex.model;
    out[32..40].copy_from_slice(&g.ex.orig_len.to_le_bytes());
    out[40..48].copy_from_slice(&g.ex.orig_fnv.to_le_bytes());
    out[48] = g.ex.filter_id;
    out[49..53].copy_from_slice(&g.ex.filter_param.to_le_bytes());
    out[53] = tier_index(g.blk) as u8;
    out[54] = MODULUS_ID;
    out[55] = g.mode as u8;
    for b in out[56..60].iter_mut() {
        *b = 0;
    }
    let sum = fnv32(&out[0..60]);
    out[60..64].copy_from_slice(&sum.to_le_bytes());
}
/// parse validity and checksum are separate verdicts: a header can be
/// internally consistent yet unverified (its fnv32 wounded) -- usable only
/// down the ladder, with the payload hashes still gating everything.
fn parse_header(b: &[u8]) -> Option<(Geom, bool)> {
    // v14 writes EG14 v8; EG13 v7 and EG12 v6 are READ by this same armor v4
    // path (the armor did not move -- only the name did), so every .egg13 and
    // .egg12 container restores here byte for byte, and armor11.rs still owns
    // .egg11 and older.
    if b.len() < HDR {
        return None;
    }
    if !((&b[0..4] == MAGIC && b[4] == FORMAT_VERSION)
        || (&b[0..4] == MAGIC_V13 && b[4] == 7)
        || (&b[0..4] == MAGIC_V12 && b[4] == 6))
    {
        return None;
    }
    if b[5] != 0 || b[54] != MODULUS_ID || u16::from_le_bytes([b[6], b[7]]) as u32 != MODULUS {
        return None;
    }
    let ti = b[53] as usize;
    if ti >= TIERS.len() {
        return None;
    }
    let mode = CtMode::from_byte(b[55])?;
    if b[48] > crate::filter::FILTER_MAX {
        return None; // filter ids 0..=FILTER_MAX (one constant, filter.rs)
    }
    let blk = TIERS[ti];
    let t = b[30] as usize;
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
    if t == 0 && mode != CtMode::Triple {
        return None;
    }
    let g = geom(len, blk, t, mode, hash, ex);
    if g.s != s || g.n > NMAX {
        return None; // internal consistency (the v5.0 audit's lesson)
    }
    let verified = fnv32(&b[0..60]) == u32::from_le_bytes(b[60..64].try_into().unwrap());
    Some((g, verified))
}

// ---------- encode ----------
/// the n squares as one buffer (n * blk bytes): data zero-padded, then the
/// parity of the systematic codeword, then (in-codeword) the CT squares.
/// Message order is data then CT; parity squares hold x^(t-1)..x^0.
pub fn build_squares(inner: &[u8], g: &Geom) -> Vec<u8> {
    let blk = g.blk;
    let mut sq = vec![0u8; g.n * blk];
    sq[..inner.len()].copy_from_slice(inner);
    if g.mode == CtMode::InCodeword {
        for i in 0..g.s {
            let r = residue(&sq[i * blk..(i + 1) * blk]).to_be_bytes();
            let (q, j) = g.ct_slot(i);
            let o = q * blk + 2 * j;
            sq[o..o + 2].copy_from_slice(&r);
        }
    }
    if g.t > 0 {
        let par = parity_of(&sq, g);
        let p0 = g.parity_at() * blk;
        sq[p0..p0 + g.t * blk].copy_from_slice(&par);
    }
    sq
}
/// message squares in codeword order (data then CT)
fn message_order(g: &Geom) -> Vec<usize> {
    (0..g.s).chain(g.ct_at()..g.n).collect()
}
/// codeword position of square j: data 0..s, then CT, then parity last
/// (position <-> degree: position p holds x^(n-1-p))
fn cw_position(g: &Geom, j: usize) -> usize {
    if g.is_data(j) {
        j
    } else if g.is_ct(j) {
        j - g.t
    } else {
        g.s + g.c + (j - g.s)
    }
}
fn square_of_position(g: &Geom, p: usize) -> usize {
    if p < g.s {
        p
    } else if p < g.s + g.c {
        p + g.t
    } else {
        g.s + (p - g.s - g.c)
    }
}
/// parity by LFSR division: (m(x) x^t) mod g(x), highest degree first, for
/// all blk/2 interleaved codewords at once; returns t squares
fn parity_of(sq: &[u8], g: &Geom) -> Vec<u8> {
    let f = gf();
    let (blk, t, l) = (g.blk, g.t, g.blk / 2);
    let gen = generator(t);
    // log of each coefficient g_0..g_(t-1); NONE marks a zero coefficient
    const NONE: u32 = u32::MAX;
    let glog: Vec<u32> = gen[..t].iter().map(|&c| if c == 0 { NONE } else { f.log_of(c) as u32 }).collect();
    let mut reg = vec![0u16; l * t];
    for i in message_order(g) {
        let s = &sq[i * blk..(i + 1) * blk];
        for j in 0..l {
            let sym = u16::from_be_bytes([s[2 * j], s[2 * j + 1]]);
            let r = &mut reg[j * t..(j + 1) * t];
            let fb = sym ^ r[0];
            if fb == 0 {
                r.copy_within(1..t, 0);
                r[t - 1] = 0;
            } else {
                let lf = f.log_of(fb);
                for q in 0..t - 1 {
                    let gl = glog[t - 1 - q];
                    r[q] = r[q + 1] ^ if gl == NONE { 0 } else { f.exp_at(lf + gl as usize) };
                }
                let gl = glog[0];
                r[t - 1] = if gl == NONE { 0 } else { f.exp_at(lf + gl as usize) };
            }
        }
    }
    let mut out = vec![0u8; t * blk];
    for q in 0..t {
        for j in 0..l {
            let v = reg[j * t + q].to_be_bytes();
            out[q * blk + 2 * j..q * blk + 2 * j + 2].copy_from_slice(&v);
        }
    }
    out
}

pub fn armor(inner: &[u8], blk: usize, t: usize, mode: CtMode, ex: Extras) -> Vec<u8> {
    let g = geom(inner.len(), blk, t, mode, fnv64(inner), ex);
    assert!(g.n <= NMAX, "codeword too long for GF(2^16): n={}", g.n);
    assert!(t <= TMAX, "t exceeds the header byte");
    let sq = build_squares(inner, &g);
    let off = offsets(&g);
    let mut out = vec![0u8; g.total];
    let mut hdr = [0u8; HDR];
    write_header(&g, &mut hdr);
    for h in [off.h0, off.h1, off.h2] {
        out[h..h + HDR].copy_from_slice(&hdr);
    }
    let mut meta = Vec::with_capacity(g.msize);
    for j in g.meta_from()..g.n {
        meta.extend_from_slice(&residue(&sq[j * g.blk..(j + 1) * g.blk]).to_be_bytes());
    }
    debug_assert_eq!(meta.len(), g.m);
    meta.extend_from_slice(&fnv32(&meta).to_le_bytes());
    for m in [off.m0, off.m1, off.m2] {
        out[m..m + g.msize].copy_from_slice(&meta);
    }
    for j in 0..g.n {
        let o = square_off(&g, j);
        let n = square_len(&g, j);
        out[o..o + n].copy_from_slice(&sq[j * g.blk..j * g.blk + n]);
    }
    out
}

// ---------- the codeword machinery ----------
/// the n squares of a container as received, with the codeword view over them
pub struct Rs<'a> {
    pub g: &'a Geom,
    pub sq: &'a [u8], // n * blk
}
impl<'a> Rs<'a> {
    fn l(&self) -> usize {
        self.g.blk / 2
    }
    /// symbol j of square `sq_index`
    #[inline]
    fn sym(&self, sq_index: usize, j: usize) -> u16 {
        let o = sq_index * self.g.blk + 2 * j;
        u16::from_be_bytes([self.sq[o], self.sq[o + 1]])
    }
    /// the locator of codeword position p: X_p = alpha^(n-1-p)
    #[inline]
    fn x_of(&self, p: usize) -> u16 {
        gf().alpha(self.g.n - 1 - p)
    }
    /// syndromes S_l = r(alpha^l), l = 0..t-1, for every codeword j (index
    /// j*t + l), with the squares in `erased` read as zero
    pub fn syndromes(&self, erased: &[bool]) -> Vec<u16> {
        let f = gf();
        let (t, l, n) = (self.g.t, self.l(), self.g.n);
        let mut acc = vec![0u16; l * t];
        for p in 0..n {
            let sqi = square_of_position(self.g, p);
            let dead = erased[sqi];
            for j in 0..l {
                let sym = if dead { 0 } else { self.sym(sqi, j) };
                let a = &mut acc[j * t..(j + 1) * t];
                a[0] ^= sym;
                for (e, v) in a.iter_mut().enumerate().skip(1) {
                    *v = if *v == 0 { sym } else { f.exp_at(f.log_of(*v) + e) ^ sym };
                }
            }
        }
        acc
    }
    /// all syndromes zero? (the systematic property)
    pub fn all_clean(&self) -> bool {
        let erased = vec![false; self.g.n];
        self.syndromes(&erased).iter().all(|&s| s == 0)
    }
}
/// erasure context shared by every codeword: Gamma over the erased positions
struct ErCtx {
    pos: Vec<usize>,   // codeword positions, ascending
    gamma: Vec<u16>,   // prod (1 + X_p x)
    xinv: Vec<u16>,    // 1 / X_p
    forney: Vec<u16>,  // X_p / Gamma'(1/X_p)
    xpow: Vec<Vec<u16>>, // X_p^l for l < t
}
fn er_ctx(rs: &Rs, e: &BTreeSet<usize>) -> ErCtx {
    let f = gf();
    let t = rs.g.t;
    let pos: Vec<usize> = e.iter().map(|&j| cw_position(rs.g, j)).collect();
    let mut gamma = vec![1u16];
    let xs: Vec<u16> = pos.iter().map(|&p| rs.x_of(p)).collect();
    for &x in &xs {
        gamma = f.poly_mul(&gamma, &[1, x]);
    }
    let dgamma = f.poly_deriv(&gamma);
    let xinv: Vec<u16> = xs.iter().map(|&x| f.inv(x)).collect();
    let forney: Vec<u16> = xs.iter().zip(&xinv).map(|(&x, &xi)| f.div(x, f.poly_eval(&dgamma, xi))).collect();
    let xpow: Vec<Vec<u16>> = xs
        .iter()
        .map(|&x| {
            let mut v = vec![1u16; t.max(1)];
            for l in 1..t {
                v[l] = f.mul(v[l - 1], x);
            }
            v
        })
        .collect();
    ErCtx { pos, gamma, xinv, forney, xpow }
}
/// Forney's erasure fill for one codeword: values at the erased positions,
/// Omega = Gamma S mod x^t, and the residual syndromes after the fill
fn erasure_fill(f: &Gf16, ctx: &ErCtx, s: &[u16]) -> (Vec<u16>, Vec<u16>, bool) {
    let t = s.len();
    let omega = f.poly_mul_mod(&ctx.gamma, s, t);
    let vals: Vec<u16> = (0..ctx.pos.len()).map(|k| f.mul(ctx.forney[k], f.poly_eval(&omega, ctx.xinv[k]))).collect();
    let mut clean = true;
    for (l, &sl) in s.iter().enumerate() {
        let mut r = sl;
        for (k, &v) in vals.iter().enumerate() {
            r ^= f.mul(v, ctx.xpow[k][l]);
        }
        if r != 0 {
            clean = false;
            break;
        }
    }
    (vals, omega, clean)
}
/// Berlekamp-Massey over GF(2^16): the shortest LFSR generating `s`;
/// returns (connection polynomial C, length L)
pub fn berlekamp_massey(f: &Gf16, s: &[u16]) -> (Vec<u16>, usize) {
    let mut c = vec![1u16];
    let mut b = vec![1u16];
    let (mut l, mut m, mut bb) = (0usize, 1usize, 1u16);
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l {
            if i < c.len() {
                d ^= f.mul(c[i], s[n - i]);
            }
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = f.div(d, bb);
        let prev = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for (i, &bi) in b.iter().enumerate() {
            c[i + m] ^= f.mul(coef, bi);
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = prev;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    while c.len() > 1 && *c.last().unwrap() == 0 {
        c.pop();
    }
    (c, l)
}
/// Gaussian elimination over GF(2^16): a reduced basis of the row space
struct Basis {
    rows: Vec<Vec<u16>>, // each normalized: leading 1 at `piv`
    piv: Vec<usize>,
}
impl Basis {
    fn new() -> Basis {
        Basis { rows: Vec::new(), piv: Vec::new() }
    }
    /// reduce v against the basis; returns the remainder (zero iff v in span)
    fn reduce(&self, f: &Gf16, v: &[u16]) -> Vec<u16> {
        let mut w = v.to_vec();
        for (r, &p) in self.rows.iter().zip(&self.piv) {
            let c = w[p];
            if c != 0 {
                for (wi, &ri) in w.iter_mut().zip(r) {
                    *wi ^= f.mul(c, ri);
                }
            }
        }
        w
    }
    fn insert(&mut self, f: &Gf16, v: &[u16]) -> bool {
        let w = self.reduce(f, v);
        match w.iter().position(|&x| x != 0) {
            None => false,
            Some(p) => {
                let inv = f.inv(w[p]);
                let row: Vec<u16> = w.iter().map(|&x| f.mul(x, inv)).collect();
                self.rows.push(row);
                self.piv.push(p);
                true
            }
        }
    }
    fn rank(&self) -> usize {
        self.rows.len()
    }
}

// ---------- dearmor (decode) ----------
/// (located squares, (square, corrected value) fixes)
type Located = (Vec<usize>, Vec<(usize, u16)>);
pub struct Tally {
    pub clean: usize,       // squares neither dead nor corrected
    pub by_residue: usize,  // dead squares convicted by their residue
    pub by_address: usize,  // dead squares named by --wound or truncation
    pub by_syndrome: usize, // squares the codewords located themselves
    pub rebuilt: usize,     // squares rebuilt from the codeword (all of the above)
    pub capacity: usize,    // t
}
pub struct DearmorOut {
    pub inner: Vec<u8>,
    pub ex: Extras,
    pub g: Geom,
    pub t: Tally,
    pub ct_report: String,
    pub hash_ok: bool, // FNV of the inner matched (stage-local truth)
    pub retried: bool,
    pub padded: usize,
    /// rung C: the codeword refused (or settled wrong) but the data squares as
    /// received hash true -- the damage was confined to parity/CT squares
    pub by_hash: bool,
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
    let verified: Vec<&(Geom, bool, [u8; HDR])> = cand.iter().flatten().filter(|(_, v, _)| *v).collect();
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
    if let Some(c) = cand.iter().flatten().next() {
        return Ok(c.0.clone());
    }
    Err("no valid header at any site".into())
}

/// the decode of one geometry from one erasure hypothesis; `candidates` are
/// the data squares whose residues died with their CT square (the brute
/// force's search space when the syndromes alone cannot place the last errors)
struct Decoder<'a> {
    rs: Rs<'a>,
    candidates: Vec<usize>,
}
impl Decoder<'_> {
    fn n(&self) -> usize {
        self.rs.g.n
    }
    fn t(&self) -> usize {
        self.rs.g.t
    }
    /// corrected squares (n * blk) or the refusal, from erasure set `e`
    fn decode(&self, e: &BTreeSet<usize>, tally: &mut Tally, depth: usize) -> Result<Vec<u8>, String> {
        let f = gf();
        let (n, t, l, blk) = (self.n(), self.t(), self.rs.l(), self.rs.g.blk);
        if e.len() > t {
            return Err(format!("{} dead squares found, capacity {} (t)", e.len(), t));
        }
        if depth > t + 2 {
            return Err("the conviction loop did not settle".into());
        }
        let mut out = self.rs.sq.to_vec();
        if t == 0 {
            return Ok(out);
        }
        let mut erased = vec![false; n];
        for &j in e {
            erased[j] = true;
            out[j * blk..(j + 1) * blk].fill(0);
        }
        let synd = self.rs.syndromes(&erased);
        let ctx = er_ctx(&self.rs, e);
        let m = t - e.len();
        let mut bad: Vec<usize> = Vec::new();
        let mut rows: Vec<Vec<u16>> = Vec::new();
        for j in 0..l {
            let s = &synd[j * t..(j + 1) * t];
            let (vals, omega, clean) = erasure_fill(f, &ctx, s);
            for (k, &p) in ctx.pos.iter().enumerate() {
                let sqi = square_of_position(self.rs.g, p);
                let o = sqi * blk + 2 * j;
                out[o..o + 2].copy_from_slice(&vals[k].to_be_bytes());
            }
            if !clean {
                bad.push(j);
                rows.push(omega[e.len()..t].to_vec());
            }
        }
        if bad.is_empty() {
            tally.rebuilt = e.len();
            tally.clean = n - e.len();
            return Ok(out);
        }
        // ---- (3a) collaborative location across the interleaved codewords ----
        // a damaged square is an error at the SAME position in every codeword
        // it touches; the modified syndromes of all codewords lie in the span
        // of the locator vectors (1, X, .., X^(m-1)) of the damaged positions
        // (Krachkovsky & Lee 1997; Bleichenbacher, Kiayias & Yung 2003)
        if m > 0 {
            let mut basis = Basis::new();
            for r in &rows {
                basis.insert(f, r);
                if basis.rank() == m {
                    break;
                }
            }
            let k = basis.rank();
            if k < m {
                let mut found: Vec<usize> = Vec::new();
                for p in 0..n {
                    let sqi = square_of_position(self.rs.g, p);
                    if erased[sqi] {
                        continue;
                    }
                    let x = self.rs.x_of(p);
                    let mut v = vec![1u16; m];
                    for i in 1..m {
                        v[i] = f.mul(v[i - 1], x);
                    }
                    if basis.reduce(f, &v).iter().all(|&z| z == 0) {
                        found.push(sqi);
                    }
                }
                // false positives (v_p in the span by chance, ~n/65536^(m-k))
                // are pruned by the residual check on the bad codewords
                {
                    use std::sync::atomic::Ordering::Relaxed;
                    LOC_RUNS.fetch_add(1, Relaxed);
                    LOC_M_SUM.fetch_add(m as u64, Relaxed);
                    LOC_K_SUM.fetch_add(k as u64, Relaxed);
                    LOC_N_SUM.fetch_add(n as u64, Relaxed);
                    LOC_MK_MIN.fetch_min((m - k) as u64, Relaxed);
                    if found.len() > k {
                        FOUND_GT_K.fetch_add(1, Relaxed);
                        let ex = (found.len() - k) as u64;
                        FOUND_EXCESS_MAX.fetch_max(ex, Relaxed);
                    } else {
                        FOUND_EQ_K.fetch_add(1, Relaxed);
                    }
                }
                if let Some(set) = self.confirm_subset(e, &found, k, &synd, &bad) {
                    if e.len() + set.len() <= t {
                        let mut e1 = e.clone();
                        e1.extend(set.iter().copied());
                        tally.by_syndrome += set.len();
                        return self.decode(&e1, tally, depth + 1);
                    }
                }
            }
        }
        // ---- (3b) per codeword: Berlekamp-Massey + Chien + Forney ----
        let mut located: BTreeSet<usize> = BTreeSet::new();
        let mut all_fixed = true;
        for &j in &bad {
            let s = &synd[j * t..(j + 1) * t];
            match self.bm_correct(&ctx, s, &erased, j) {
                Some((errs, fixes)) => {
                    for (sqi, v) in fixes {
                        let o = sqi * blk + 2 * j;
                        out[o..o + 2].copy_from_slice(&v.to_be_bytes());
                    }
                    located.extend(errs);
                }
                None => all_fixed = false,
            }
        }
        if all_fixed {
            let check = Rs { g: self.rs.g, sq: &out };
            if check.all_clean() {
                tally.by_syndrome += located.len();
                tally.rebuilt = e.len() + located.len();
                tally.clean = n - tally.rebuilt;
                return Ok(out);
            }
        }
        // a partial location is taken only if it makes the bad codewords
        // consistent as a set (a lone codeword's locator with no residual
        // syndrome to answer to is a guess, and a wrong guess poisons E)
        if !located.is_empty() && e.len() + located.len() < t {
            let set: Vec<usize> = located.iter().copied().collect();
            CONSIST_VIA_PARTIAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if self.consistent(e, &set, &synd, &bad) {
                let mut e1 = e.clone();
                e1.extend(set.iter().copied());
                tally.by_syndrome += set.len();
                return self.decode(&e1, tally, depth + 1);
            }
        }
        // ---- (3c) the last errors sit where their residues died: search ----
        if m > 0 && m <= 2 && !self.candidates.is_empty() && self.candidates.len() <= BRUTE_MAX {
            if let Some(set) = self.brute(e, &synd, m) {
                let mut e1 = e.clone();
                e1.extend(set.iter().copied());
                tally.by_syndrome += set.len();
                return self.decode(&e1, tally, depth + 1);
            }
        }
        // v13-M2(0): this used to read "{} dead squares located", printing the
        // size of the erasure set the attempt STARTED from -- which on a blind
        // wound is 0 by definition, and read as "the joint locator found
        // nothing". It says what it means now.
        Err(format!(
            "beyond capacity: {} squares were named or convicted before this attempt and the joint locator could place none of the rest; {} codewords still inconsistent (>= {} more damaged squares the syndromes cannot place); capacity {} (t)",
            e.len(),
            bad.len(),
            m.max(1),
            t
        ))
    }
    /// the syndromes of codeword j were taken with `e` zeroed; return them
    /// with the squares in `extra` zeroed as well
    fn zero_extra(&self, synd: &[u16], j: usize, extra: &[usize]) -> Vec<u16> {
        let f = gf();
        let t = self.t();
        let mut s = synd[j * t..(j + 1) * t].to_vec();
        for &sqi in extra {
            let x = self.rs.x_of(cw_position(self.rs.g, sqi));
            let r = self.rs.sym(sqi, j);
            let mut xp = 1u16;
            for sl in s.iter_mut() {
                *sl ^= f.mul(r, xp);
                xp = f.mul(xp, x);
            }
        }
        s
    }
    /// does erasing `e` + `extra` make the bad codewords consistent? (the
    /// residual check exists because |e| + |extra| < t leaves >= 1 syndrome)
    fn consistent(&self, e: &BTreeSet<usize>, extra: &[usize], synd: &[u16], bad: &[usize]) -> bool {
        // EGG_CONSIST: the control for v14-N1. A drill that never reaches this
        // function proves nothing about the `take(4)`, so the calls, the
        // codewords actually examined, and the rejections are counted and
        // printed rather than assumed.
        CONSIST_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        CONSIST_BAD.fetch_add(bad.len() as u64, std::sync::atomic::Ordering::Relaxed);
        CONSIST_READ.fetch_add(bad.len().min(4) as u64, std::sync::atomic::Ordering::Relaxed);
        let f = gf();
        let mut e1 = e.clone();
        e1.extend(extra.iter().copied());
        let ctx = er_ctx(&self.rs, &e1);
        for &j in bad.iter().take(4) {
            let s = self.zero_extra(synd, j, extra);
            let (_, _, clean) = erasure_fill(f, &ctx, &s);
            if !clean {
                CONSIST_REJECT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return false;
            }
        }
        true
    }
    /// choose the k true positions among `found`.
    ///
    /// **`found.len() > k` is IMPOSSIBLE, and the enumeration this function used
    /// to do was dead code** (v14-N1, proved and then measured). The locator
    /// tests each position's vector `(1, X, X^2, .., X^(m-1))` for membership in
    /// a `k`-dimensional span, and those are VANDERMONDE rows: any `m` of them
    /// with distinct nodes are linearly independent. So `k + 1` distinct
    /// positions cannot all lie in a `k`-dimensional space while `k < m` -- and
    /// `k < m` is exactly the condition under which the locator runs at all.
    /// The estimate in the caller's comment, `~n/65536^(m-k)`, treats the
    /// candidate as a random vector. It is not one, and the true rate is zero.
    ///
    /// Measured before this was written: **2,355 locator runs, 0 over-reports,
    /// max excess 0** -- including 24 trials built for the best possible case,
    /// n = 62,518 squares (NMAX is 65,535) with `m - k = 1`, where the
    /// random-vector estimate predicts 0.95 false positives per run and would
    /// have produced about 23. It produced none.
    ///
    /// Kept: the residual check on the `found.len() == k` case, which is real
    /// and runs (2,333 calls in one --full audit, all accepted). Deleted: the
    /// subset search, and with it the "one 16-bit check, ~32 expected false
    /// accepts" exposure -- which priced a path that cannot be entered. The
    /// impossible case now REFUSES rather than enumerating, because a locator
    /// that over-reports has broken an invariant and guessing a subset would be
    /// the worst available answer.
    fn confirm_subset(&self, e: &BTreeSet<usize>, found: &[usize], k: usize, synd: &[u16], bad: &[usize]) -> Option<Vec<usize>> {
        if found.len() < k || k == 0 {
            return None;
        }
        CONFIRM_SUBSET_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if found.len() > k {
            FOUND_GT_K.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return None;
        }
        CONSIST_VIA_BRUTE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if self.consistent(e, found, synd, bad) {
            CONFIRM_SUBSET_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(found.to_vec())
        } else {
            None
        }
    }
    /// errors-and-erasures for codeword j: the located squares and the
    /// corrected values (square, value) for every erased or errored square
    fn bm_correct(&self, ctx: &ErCtx, s: &[u16], erased: &[bool], j: usize) -> Option<Located> {
        let f = gf();
        let (n, t) = (self.n(), self.t());
        let ne = ctx.pos.len();
        let m = t - ne;
        if m < 2 {
            return None;
        }
        let xi = f.poly_mul_mod(&ctx.gamma, s, t);
        let (lambda, deg) = berlekamp_massey(f, &xi[ne..t]);
        // strictly inside the bound: 2*deg == m leaves no residual syndrome
        // to verify the locator, and an unverified locator is a guess
        if deg == 0 || 2 * deg >= m || lambda.len() != deg + 1 {
            return None;
        }
        // Chien: roots of Lambda among the inverse locators of the live positions
        let mut roots: Vec<usize> = Vec::new();
        for p in 0..n {
            if erased[square_of_position(self.rs.g, p)] {
                continue;
            }
            if f.poly_eval(&lambda, f.alpha_inv(n - 1 - p)) == 0 {
                roots.push(p);
                if roots.len() > deg {
                    return None;
                }
            }
        }
        if roots.len() != deg {
            return None;
        }
        let psi = f.poly_mul(&lambda, &ctx.gamma);
        let omega = f.poly_mul_mod(&psi, s, t);
        let dpsi = f.poly_deriv(&psi);
        let mut fixes: Vec<(usize, u16)> = Vec::new();
        let mut resid = s.to_vec();
        for &p in ctx.pos.iter().chain(roots.iter()) {
            let x = self.rs.x_of(p);
            let xinv = f.inv(x);
            let den = f.poly_eval(&dpsi, xinv);
            if den == 0 {
                return None;
            }
            let ev = f.mul(x, f.div(f.poly_eval(&omega, xinv), den));
            let sqi = square_of_position(self.rs.g, p);
            let received = if erased[sqi] { 0 } else { self.rs.sym(sqi, j) };
            fixes.push((sqi, received ^ ev));
            let mut xp = 1u16;
            for r in resid.iter_mut() {
                *r ^= f.mul(ev, xp);
                xp = f.mul(xp, x);
            }
        }
        if resid.iter().any(|&r| r != 0) {
            return None;
        }
        Some((roots.iter().map(|&p| square_of_position(self.rs.g, p)).collect(), fixes))
    }
    /// k == m: the syndromes carry no location; the residues that would have
    /// convicted the last m squares died with their CT square. Every m-subset
    /// of the candidates is a hypothesis; a hypothesis is tested on codewords
    /// whose rebuilt CT symbol is the residue of a candidate it calls clean.
    fn brute(&self, e: &BTreeSet<usize>, synd: &[u16], m: usize) -> Option<Vec<usize>> {
        let f = gf();
        let g = self.rs.g;
        let cands = &self.candidates;
        let res: Vec<u16> = cands.iter().map(|&i| residue(&self.rs.sq[i * g.blk..(i + 1) * g.blk])).collect();
        let test = |sub: &[usize]| -> bool {
            let mut e1 = e.clone();
            e1.extend(sub.iter().copied());
            let ctx = er_ctx(&self.rs, &e1);
            let mut checks = 0;
            for (ci, &cand) in cands.iter().enumerate() {
                if e1.contains(&cand) {
                    continue; // hypothesised, or located by an earlier rung: its residue is noise
                }
                let (q, j) = g.ct_slot(cand);
                if !e1.contains(&q) {
                    continue; // its CT square is alive: judged already
                }
                let s = self.zero_extra(synd, j, sub);
                let (vals, _, _) = erasure_fill(f, &ctx, &s);
                let k = ctx.pos.iter().position(|&p| square_of_position(g, p) == q).unwrap();
                if vals[k] != res[ci] {
                    return false;
                }
                checks += 1;
                if checks >= 3 {
                    break;
                }
            }
            if checks > 0 {
                return true;
            }
            // no other candidate to judge the hypothesis by: rebuild every
            // hypothesised square in full and ask whether its own residue is
            // what the rebuilt CT square says it is (an independent relation
            // between data and CT, 2^-16 per square for a wrong hypothesis)
            let l = self.rs.l();
            for &cand in sub {
                let (q, j) = g.ct_slot(cand);
                if !e1.contains(&q) {
                    continue;
                }
                let kc = ctx.pos.iter().position(|&p| square_of_position(g, p) == cand).unwrap();
                let kq = ctx.pos.iter().position(|&p| square_of_position(g, p) == q).unwrap();
                let mut square = vec![0u8; g.blk];
                let mut ct_sym = 0u16;
                for jj in 0..l {
                    let s = self.zero_extra(synd, jj, sub);
                    let (vals, _, _) = erasure_fill(f, &ctx, &s);
                    square[2 * jj..2 * jj + 2].copy_from_slice(&vals[kc].to_be_bytes());
                    if jj == j {
                        ct_sym = vals[kq];
                    }
                }
                if residue(&square) != ct_sym {
                    return false;
                }
                checks += 1;
            }
            checks > 0
        };
        let live: Vec<usize> = cands.iter().copied().filter(|c| !e.contains(c)).collect();
        match m {
            1 => live.iter().find(|&&c| test(&[c])).map(|&c| vec![c]),
            2 => {
                for (a, &ca) in live.iter().enumerate() {
                    for &cb in &live[a + 1..] {
                        if test(&[ca, cb]) {
                            return Some(vec![ca, cb]);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}

pub fn dearmor(cont_in: &[u8], wounds_in: &[(usize, usize)], _doubles: bool) -> Result<DearmorOut, String> {
    dearmor_with(cont_in, wounds_in, true)
}
/// `trust_residues = false` is the audit's hook: no square is convicted by
/// its residue, so the syndromes alone must locate every unnamed error
/// (the classical errors-and-erasures path, exercised on purpose)
pub fn dearmor_with(cont_in: &[u8], wounds_in: &[(usize, usize)], trust_residues: bool) -> Result<DearmorOut, String> {
    // bootstrap: a header at the head or at the raw end names the geometry
    // (and so the third site's true offsets after truncation padding); with
    // BOTH end sites dead, scan for the magic -- the surviving mid copy sits
    // at a geometry-dependent offset only it can name, so a hit only counts
    // if its own geometry puts a site exactly where it was found
    let seed = select_header(cont_in, &[0, cont_in.len().saturating_sub(HDR)]).or_else(|e| {
        let mut fallback: Option<Geom> = None;
        let mut i = 0usize;
        while i + HDR <= cont_in.len() {
            if &cont_in[i..i + 4] == MAGIC || &cont_in[i..i + 4] == MAGIC_V13 || &cont_in[i..i + 4] == MAGIC_V12 {
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
    let g = select_header(&cont, &[off.h0, off.h1, off.h2])?;
    if g.total != seed.total {
        return Err("surviving headers name inconsistent geometry".into());
    }
    let off = offsets(&g);
    let blk = g.blk;
    let in_wound = |lo: usize, hi: usize| wounds.iter().any(|&(a, l)| a < hi && a + l > lo);

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
    let (meta, meta_verified) = if let Some((b, _)) = ver.first() {
        (b.to_vec(), true)
    } else {
        let voted: Vec<u8> = (0..g.m)
            .map(|i| {
                let (x, y, z) = (cont[off.m0 + i], cont[off.m1 + i], cont[off.m2 + i]);
                if x == y || x == z { x } else if y == z { y } else { x }
            })
            .collect();
        (voted, false)
    };

    // ---- the squares as received; wounded ones named by address ----
    let mut sq = vec![0u8; g.n * blk];
    let mut wounded = vec![false; g.n];
    for j in 0..g.n {
        let a = square_off(&g, j);
        let len = square_len(&g, j);
        sq[j * blk..j * blk + len].copy_from_slice(&cont[a..a + len]);
        wounded[j] = in_wound(a, a + len);
    }
    let meta_res = |j: usize| -> u16 {
        let o = 2 * (j - g.meta_from());
        u16::from_be_bytes([meta[o], meta[o + 1]])
    };
    // squares the meta judges: all (CT x3) or CT + parity (in-codeword)
    let mut convicted: Vec<bool> = vec![false; g.n];
    for j in g.meta_from()..g.n {
        if !wounded[j] && residue(&sq[j * blk..(j + 1) * blk]) != meta_res(j) {
            convicted[j] = true;
        }
    }
    // data squares in the in-codeword layout: judged by a LIVE CT square only
    let mut unjudged: Vec<usize> = Vec::new();
    let mut judged = 0usize;
    match g.mode {
        CtMode::InCodeword => {
            for i in 0..g.s {
                let (q, jj) = g.ct_slot(i);
                if wounded[q] || convicted[q] {
                    if !wounded[i] {
                        unjudged.push(i);
                    }
                    continue;
                }
                judged += 1;
                let o = q * blk + 2 * jj;
                let stored = u16::from_be_bytes([sq[o], sq[o + 1]]);
                if !wounded[i] && residue(&sq[i * blk..(i + 1) * blk]) != stored {
                    convicted[i] = true;
                }
            }
        }
        CtMode::Triple => judged = g.s,
        CtMode::Absent => {} // no residues: the codewords judge every square
    }
    let by_address = wounded.iter().filter(|&&w| w).count();
    let by_residue = convicted.iter().filter(|&&c| c).count();
    let ct_report = match g.mode {
        CtMode::Absent => format!(
            "CT none (no residue table; {} data + {} parity squares judged by the {} interleaved codewords alone); {} dead by address",
            g.s,
            g.t,
            blk / 2,
            by_address
        ),
        _ => format!(
            "{} ({}); {} data squares judged, {} unjudged (their CT square is dead); {} convicted by residue, {} dead by address",
            g.mode.name(),
            if meta_verified { "meta verified" } else { "meta voted, unverified" },
            judged,
            unjudged.len(),
            by_residue,
            by_address
        ),
    };
    let rs = Rs { g: &g, sq: &sq };
    let dec = Decoder { rs, candidates: unjudged };
    let finish = |squares: &[u8], tally: Tally, retried: bool| -> DearmorOut {
        let inner = squares[..g.len].to_vec();
        let hash_ok = fnv64(&inner) == g.hash;
        DearmorOut { inner, ex: g.ex, g: g.clone(), t: tally, ct_report: ct_report.clone(), hash_ok, retried, padded, by_hash: false }
    };
    // rung A: residues trusted; rung B: address only (the meta itself may lie)
    let mut e_a: BTreeSet<usize> = BTreeSet::new();
    let mut e_b: BTreeSet<usize> = BTreeSet::new();
    for j in 0..g.n {
        if wounded[j] {
            e_a.insert(j);
            e_b.insert(j);
        } else if convicted[j] {
            e_a.insert(j);
        }
    }
    let mut first_err: Option<String> = None;
    let mut best: Option<DearmorOut> = None;
    for (rung, e0) in [e_a, e_b].iter().enumerate() {
        if rung == 0 && !trust_residues {
            continue; // the audit's hook: address only
        }
        if rung == 1 && by_residue == 0 && trust_residues {
            break; // rung B would repeat rung A
        }
        let mut tally = Tally {
            clean: 0,
            by_residue: e0.iter().filter(|&&j| !wounded[j]).count(),
            by_address,
            by_syndrome: 0,
            rebuilt: 0,
            capacity: g.t,
        };
        match dec.decode(e0, &mut tally, 0) {
            Ok(squares) => {
                let out = finish(&squares, tally, rung > 0);
                if out.hash_ok {
                    return Ok(out);
                }
                if best.is_none() {
                    best = Some(out);
                }
            }
            Err(e) => {
                if first_err.is_none() {
                    first_err = Some(e);
                }
            }
        }
    }
    // rung C (v12-M2b): the codeword refused, or settled on a codeword the FNV
    // rejects. The damage may have been confined to parity (or CT) squares:
    // the DATA SQUARES AS RECEIVED are hashed, and an intact inner hashes true
    // (a damaged one passes with 2^-64). Never wrong; it is the v8 lesson made
    // a rung (t+1 dead check-table squares lost only checks, the payload was
    // intact, and the armor was righter than the drill).
    {
        let inner = sq[..g.len].to_vec();
        if fnv64(&inner) == g.hash {
            let tally = Tally { clean: g.s, by_residue: 0, by_address, by_syndrome: 0, rebuilt: 0, capacity: g.t };
            return Ok(DearmorOut {
                inner,
                ex: g.ex,
                g: g.clone(),
                t: tally,
                ct_report: format!("{}; rung C: the data squares as received hash true -- the damage was confined to parity/CT squares", ct_report),
                hash_ok: true,
                retried: true,
                padded,
                by_hash: true,
            });
        }
    }
    match best {
        Some(out) => Ok(out), // syndromes settled but the FNV did not: reported honestly, never written
        None => Err(first_err.unwrap_or_else(|| "undecodable".into())),
    }
}
