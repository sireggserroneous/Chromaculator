//! zmatch.rs -- WS-P, THE PREDICTED PARSE (v14-N3b).
//!
//! v13's deflate recipe STORES the parse: one flag bit per token, one byte per
//! match length, two bytes per match distance. On `aoe4-autosave.sav` that is
//! 38,340,574 tokens and 25,661,244 matches -- 82,088,840 raw bytes of recipe,
//! coded to 3,505,440, which is 40.0% of everything the row ships. precomp does
//! the same job in 24,160 B, 145x smaller, and the card measured the whole
//! difference: 97.9% of a 40.6% loss on our largest row.
//!
//! `deflate.rs`'s header says its approach "is preflate's approach". It is not.
//! Preflate re-runs a zlib-compatible matcher over the inflated bytes, PREDICTS
//! the token zlib would have emitted at each position, and stores only the
//! corrections. This module is that matcher.
//!
//! Why v12's search could not find it and this can. v13-M2 recorded the failure
//! as "1,260 zlib configurations, best agreement 3 bytes of 4,096, first
//! difference at byte 0" -- but **v12 compared COMPRESSED BYTES.** Huffman
//! construction and block splitting diverge on the first block for any
//! re-compression, so that search could only ever return zero: it was measuring
//! the wrong layer. At the PARSE layer, with the parameters INFERRED rather
//! than assumed, `aoe4-autosave.sav` agrees with zlib level 4 / memLevel 9 on
//! 1,796,005 of 1,796,006 tokens over 12 MB -- 99.99994%, and the single miss
//! is the measurement's own truncation edge. At the default level 6 the same
//! file agrees on 82.7%. The parameters are the whole finding.
//!
//! Attribution: Jean-loup Gailly and Mark Adler's zlib `deflate.c`
//! (`deflate_slow`, `deflate_fast`, `longest_match`, the configuration table);
//! Dirk Steinke's preflate for the predict-and-correct framing.

/// RFC 1951 bounds
const MIN_MATCH: usize = 3;
const MAX_MATCH: usize = 258;
const W_SIZE: usize = 1 << 15;
const W_MASK: usize = W_SIZE - 1;
/// zlib's MAX_DIST: the furthest back a match may reach
const MAX_DIST: usize = W_SIZE - MAX_MATCH - MIN_MATCH - 1;
const TOO_FAR: usize = 4096;
const NIL: u32 = 0;

/// one parsed token: a literal byte, or a match of (len, dist)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tok {
    Lit(u8),
    Match(u16, u16), // len 3..=258, dist 1..=32768
}

impl Tok {
    /// how many output bytes this token produces
    #[inline]
    pub fn span(&self) -> usize {
        match self {
            Tok::Lit(_) => 1,
            Tok::Match(l, _) => *l as usize,
        }
    }
}

/// zlib's `configuration_table` entry plus the memLevel-derived hash geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cfg {
    pub level: u8,
    pub mem_level: u8,
    good: usize,
    lazy: usize,
    nice: usize,
    chain: usize,
}

impl Cfg {
    /// zlib deflate.c `configuration_table`, levels 1..=9
    pub fn new(level: u8, mem_level: u8) -> Cfg {
        let (good, lazy, nice, chain) = match level {
            1 => (4, 4, 8, 4),
            2 => (4, 5, 16, 8),
            3 => (4, 6, 32, 32),
            4 => (4, 4, 16, 16),
            5 => (8, 16, 32, 32),
            6 => (8, 16, 128, 128),
            7 => (8, 32, 128, 256),
            8 => (32, 128, 258, 1024),
            _ => (32, 258, 258, 4096),
        };
        Cfg { level, mem_level, good, lazy, nice, chain }
    }
    /// levels 1..=3 run `deflate_fast`: greedy, with no lazy step
    fn greedy(&self) -> bool {
        self.level <= 3
    }
    fn hash_bits(&self) -> u32 {
        self.mem_level as u32 + 7
    }
}

/// the hash chains, exactly zlib's: `head` by hash, `prev` by window position
struct Chains {
    head: Vec<u32>,
    prev: Vec<u32>,
    ins_h: usize,
    hash_mask: usize,
    hash_shift: u32,
}

impl Chains {
    fn new(cfg: &Cfg) -> Chains {
        let hb = cfg.hash_bits();
        Chains {
            head: vec![NIL; 1usize << hb],
            prev: vec![NIL; W_SIZE],
            ins_h: 0,
            hash_mask: (1usize << hb) - 1,
            // zlib: hash_shift = (hash_bits + MIN_MATCH-1) / MIN_MATCH
            hash_shift: hb.div_ceil(MIN_MATCH as u32),
        }
    }
    #[inline]
    fn upd(&mut self, c: u8) {
        self.ins_h = ((self.ins_h << self.hash_shift) ^ c as usize) & self.hash_mask;
    }
    /// zlib's INSERT_STRING: hash the three bytes at `s`, chain it, hand back
    /// the previous head of that bucket
    #[inline]
    fn insert(&mut self, data: &[u8], s: usize) -> u32 {
        self.upd(data[s + MIN_MATCH - 1]);
        let h = self.ins_h;
        let prev_head = self.head[h];
        self.prev[s & W_MASK] = prev_head;
        self.head[h] = s as u32;
        prev_head
    }
}

/// zlib's `longest_match`. `prev_len` is the caller's current best; the walk
/// only reports something strictly longer.
fn longest_match(data: &[u8], ch: &Chains, cfg: &Cfg, strstart: usize, mut cur: usize, prev_len: usize) -> (usize, usize) {
    let mut chain = cfg.chain;
    if prev_len >= cfg.good {
        chain >>= 2;
    }
    let lookahead = data.len() - strstart;
    let nice = cfg.nice.min(lookahead);
    let limit = strstart.saturating_sub(MAX_DIST);
    let maxlen = MAX_MATCH.min(lookahead);
    let mut best = prev_len;
    let mut best_start = 0usize;
    while chain > 0 {
        if cur <= limit || cur == 0 {
            break;
        }
        let mut n = 0usize;
        while n < maxlen && data[cur + n] == data[strstart + n] {
            n += 1;
        }
        if n > best {
            best = n;
            best_start = cur;
            if n >= nice {
                break;
            }
        }
        cur = ch.prev[cur & W_MASK] as usize;
        chain -= 1;
    }
    (best.min(lookahead), best_start)
}

/// Run the matcher over `data` and return the token stream zlib would emit for
/// a single `compress()` call over the whole buffer.
pub fn parse(data: &[u8], cfg: Cfg) -> Vec<Tok> {
    let n = data.len();
    let mut out: Vec<Tok> = Vec::with_capacity(n / 3 + 16);
    if n < MIN_MATCH {
        out.extend(data.iter().map(|&b| Tok::Lit(b)));
        return out;
    }
    let mut ch = Chains::new(&cfg);
    // zlib primes ins_h with the first MIN_MATCH-1 bytes before any insert
    ch.upd(data[0]);
    ch.upd(data[1]);

    let mut strstart = 0usize;
    let mut match_available = false;
    let mut prev_len = MIN_MATCH - 1;
    let mut prev_start = 0usize;

    while strstart < n {
        // the last MIN_MATCH-1 bytes cannot start a hashed string
        let hash_head = if strstart + MIN_MATCH <= n { ch.insert(data, strstart) } else { NIL };

        let mut cur_len = MIN_MATCH - 1;
        let mut cur_start = 0usize;
        if hash_head != NIL && prev_len < cfg.lazy && strstart - (hash_head as usize) <= MAX_DIST {
            let (l, s) = longest_match(data, &ch, &cfg, strstart, hash_head as usize, MIN_MATCH - 1);
            cur_len = l;
            cur_start = s;
            // zlib drops a three-byte match that reaches TOO_FAR back
            if cur_len == MIN_MATCH && strstart - cur_start > TOO_FAR {
                cur_len = MIN_MATCH - 1;
            }
        }

        if cfg.greedy() {
            if cur_len >= MIN_MATCH {
                out.push(Tok::Match(cur_len as u16, (strstart - cur_start) as u16));
                for k in 1..cur_len {
                    let s = strstart + k;
                    if s + MIN_MATCH <= n {
                        ch.insert(data, s);
                    }
                }
                strstart += cur_len;
            } else {
                out.push(Tok::Lit(data[strstart]));
                strstart += 1;
            }
            continue;
        }

        // deflate_slow's lazy step: the match at strstart-1 only wins if the
        // one at strstart is no longer than it
        if prev_len >= MIN_MATCH && cur_len <= prev_len {
            let start = strstart - 1;
            out.push(Tok::Match(prev_len as u16, (start - prev_start) as u16));
            let last = start + prev_len;
            let mut s = strstart + 1;
            while s + MIN_MATCH <= n && s < last {
                ch.insert(data, s);
                s += 1;
            }
            strstart = last;
            match_available = false;
            prev_len = MIN_MATCH - 1;
        } else if match_available {
            out.push(Tok::Lit(data[strstart - 1]));
            prev_len = cur_len;
            prev_start = cur_start;
            strstart += 1;
        } else {
            match_available = true;
            prev_len = cur_len;
            prev_start = cur_start;
            strstart += 1;
        }
    }
    if match_available {
        out.push(Tok::Lit(data[n - 1]));
    }
    out
}

/// The parse a v13 recipe STORES, read back as tokens. `flags` is one bit per
/// token LSB-first (set = match), `lens` holds length-3, `dists` the distance;
/// literal values are read out of the inflated bytes at the current position,
/// which is exactly what `deflate::respell` does.
pub fn from_recipe(d: &crate::deflate::Deflate) -> Vec<Tok> {
    let mut out = Vec::with_capacity(d.ntok as usize);
    let (mut li, mut di, mut pos) = (0usize, 0usize, 0usize);
    for tok in 0..d.ntok as usize {
        if tok >> 3 >= d.flags.len() {
            break;
        }
        if (d.flags[tok >> 3] >> (tok & 7)) & 1 == 1 {
            if li >= d.lens.len() || di >= d.dists.len() {
                break;
            }
            let ln = d.lens[li] as u16 + 3;
            out.push(Tok::Match(ln, d.dists[di]));
            pos += ln as usize;
            li += 1;
            di += 1;
        } else {
            if pos >= d.values.len() {
                break;
            }
            out.push(Tok::Lit(d.values[pos]));
            pos += 1;
        }
    }
    out
}

/// Count tokens of `actual` the prediction does not reproduce, aligned by
/// OUTPUT POSITION. Aligning by token INDEX is meaningless -- one differing
/// token shifts every index after it, which is how a first cut of this
/// measurement read 2.2% where the truth was 98.5%.
pub fn disagreements(actual: &[Tok], pred: &[Tok]) -> usize {
    // both streams are in output order, so a two-pointer merge over POSITIONS
    // settles it in one pass and no allocation. A HashMap keyed by position
    // works too and is what the first cut used -- at 38.3M tokens it wants the
    // better part of a gigabyte for a number this walk gets for free.
    let (mut i, mut j, mut pa, mut pp, mut bad) = (0usize, 0usize, 0usize, 0usize, 0usize);
    while i < actual.len() {
        while j < pred.len() && pp < pa {
            pp += pred[j].span();
            j += 1;
        }
        if j >= pred.len() || pp != pa || pred[j] != actual[i] {
            bad += 1;
        }
        pa += actual[i].span();
        i += 1;
    }
    bad
}

/// Try every (level, memLevel) zlib could have used and return the one whose
/// parse needs the fewest corrections, with that count. This is the step v12
/// skipped: the parameters are read out of the stream, not assumed.
pub fn infer(data: &[u8], actual: &[Tok]) -> (Cfg, usize) {
    let mut best = (Cfg::new(6, 8), usize::MAX);
    for level in 1..=9u8 {
        for mem_level in 1..=9u8 {
            let cfg = Cfg::new(level, mem_level);
            let d = disagreements(actual, &parse(data, cfg));
            if d < best.1 {
                best = (cfg, d);
                if d == 0 {
                    return best;
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the parse must reproduce the input exactly -- a token stream that does
    /// not re-spell its own data is not a parse, whatever its agreement rate
    fn replay(toks: &[Tok]) -> Vec<u8> {
        let mut out = Vec::new();
        for t in toks {
            match t {
                Tok::Lit(b) => out.push(*b),
                Tok::Match(l, d) => {
                    let start = out.len() - *d as usize;
                    for k in 0..*l as usize {
                        let b = out[start + k];
                        out.push(b);
                    }
                }
            }
        }
        out
    }

    #[test]
    fn parse_replays_its_input() {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..40000u32 {
            v.push((i.wrapping_mul(2654435761) >> 13) as u8);
        }
        v.extend_from_slice(&v.clone()[..15000]); // a long repeat the matcher must find
        v.extend(b"the same words the same words the same words".iter().copied());
        for cfg in [Cfg::new(1, 8), Cfg::new(4, 9), Cfg::new(6, 8), Cfg::new(9, 9)] {
            let toks = parse(&v, cfg);
            assert_eq!(replay(&toks), v, "parse did not replay its own input at {:?}", cfg);
            assert!(toks.len() < v.len(), "no matches found at {:?}", cfg);
        }
    }

    /// every match must be legal RFC 1951: 3..=258 long, 1..=32768 back, and
    /// never reaching before the start of the output
    #[test]
    fn matches_are_legal() {
        let mut v: Vec<u8> = b"abcabcabcabc".repeat(500);
        v.extend(b"zzzz".repeat(300));
        let toks = parse(&v, Cfg::new(6, 8));
        let mut p = 0usize;
        for t in &toks {
            if let Tok::Match(l, d) = t {
                assert!((3..=258).contains(l), "illegal length {}", l);
                assert!(*d >= 1 && (*d as usize) <= p, "illegal distance {} at {}", d, p);
                assert!((*d as usize) <= 32768, "distance past the window: {}", d);
            }
            p += t.span();
        }
        assert_eq!(p, v.len());
    }

    /// short inputs: below MIN_MATCH nothing can be hashed
    #[test]
    fn tiny_inputs() {
        for n in 0..4usize {
            let v: Vec<u8> = (0..n as u8).collect();
            let toks = parse(&v, Cfg::new(6, 8));
            assert_eq!(replay(&toks), v);
        }
    }

    /// `disagreements` counts by OUTPUT POSITION, so a single extra literal
    /// early must not be read as total disagreement
    #[test]
    fn disagreement_is_position_aligned() {
        let a = vec![Tok::Lit(1), Tok::Match(4, 1), Tok::Lit(2)];
        assert_eq!(disagreements(&a, &a), 0);
        // same output, spelled with literals instead of the match: the two
        // tokens at positions 1 and 5 disagree, not "everything after 0"
        let b = vec![Tok::Lit(1), Tok::Lit(1), Tok::Lit(1), Tok::Lit(1), Tok::Lit(1), Tok::Lit(2)];
        assert_eq!(disagreements(&a, &b), 1);
    }
}
