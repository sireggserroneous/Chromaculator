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
//! all but 22 of 38,340,574 tokens. At the default level 6 the same file agrees
//! on 82.7%. The parameter search is the finding, not the matcher.
//!
//! THE LOCKSTEP, and why the simpler design does not work: a free-running
//! matcher that disagrees 22 times produces **38,340,647 tokens against the
//! actual 38,340,574**, because a disagreement splits a match into literals and
//! every index after it shifts. Corrections therefore cannot be (index, token)
//! patches over an independently produced stream -- that was measured, not
//! assumed. `drive` walks ONE loop in which the caller sees each predicted
//! token and may replace it; the EMITTED token, never the prediction, advances
//! the state. The encoder forces the actual token and records where; the
//! decoder forces the recorded ones. Both run this identical function, so they
//! agree by construction rather than by luck.
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
    /// every position below this has been INSERT_STRING'd, in order
    upto: usize,
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
            upto: 0,
        }
    }
    #[inline]
    fn upd(&mut self, c: u8) {
        self.ins_h = ((self.ins_h << self.hash_shift) ^ c as usize) & self.hash_mask;
    }
    /// INSERT_STRING every position from `upto` through `target`, in order --
    /// the rolling hash demands the order. Returns the previous head of the
    /// LAST bucket touched, which is where `longest_match` starts its walk.
    /// zlib never hashes a position without MIN_MATCH bytes ahead of it.
    #[inline]
    fn insert_upto(&mut self, data: &[u8], target: usize) -> u32 {
        let n = data.len();
        let mut last = NIL;
        while self.upto <= target {
            let s = self.upto;
            if s + MIN_MATCH > n {
                self.upto = target + 1;
                return NIL;
            }
            self.upd(data[s + MIN_MATCH - 1]);
            let h = self.ins_h;
            last = self.head[h];
            self.prev[s & W_MASK] = last;
            self.head[h] = s as u32;
            self.upto += 1;
        }
        last
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

/// THE ONE LOOP. `decide(tok_index, out_pos, predicted) -> emitted` is called
/// once per token; returning `predicted` unchanged reproduces zlib exactly, and
/// returning anything else forces that token and resyncs the matcher to a clean
/// state just past it. The EMITTED token drives the state, never the
/// prediction, which is what keeps encoder and decoder identical.
///
/// The resync after a forced token is deliberately the state zlib is in after
/// emitting a match -- no pending lazy candidate. It is not what zlib would do
/// after a literal, and that does not matter: both sides run this function, so
/// both agree. Fidelity to zlib is what makes predictions ACCURATE; determinism
/// is what makes them CORRECT, and only the second is load-bearing.
fn drive<F>(data: &[u8], cfg: Cfg, mut decide: F) -> Vec<Tok>
where
    F: FnMut(usize, usize, Tok) -> Tok,
{
    let n = data.len();
    let mut out: Vec<Tok> = Vec::with_capacity(n / 3 + 16);
    let mut idx = 0usize;
    if n < MIN_MATCH {
        for (p, &b) in data.iter().enumerate() {
            out.push(decide(p, p, Tok::Lit(b)));
        }
        return out;
    }
    let mut ch = Chains::new(&cfg);
    // zlib primes ins_h with the first MIN_MATCH-1 bytes before any insert
    ch.upd(data[0]);
    ch.upd(data[1]);

    let mut strstart = 0usize;
    let mut pos = 0usize; // output bytes emitted so far == the next token's start
    let mut match_available = false;
    let mut prev_len = MIN_MATCH - 1;
    let mut prev_start = 0usize;

    while strstart < n {
        let hash_head = ch.insert_upto(data, strstart);

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

        // what this iteration would emit, if anything
        let proposal: Option<Tok> = if cfg.greedy() {
            Some(if cur_len >= MIN_MATCH {
                Tok::Match(cur_len as u16, (strstart - cur_start) as u16)
            } else {
                Tok::Lit(data[strstart])
            })
        } else if prev_len >= MIN_MATCH && cur_len <= prev_len {
            Some(Tok::Match(prev_len as u16, (pos - prev_start) as u16))
        } else if match_available {
            Some(Tok::Lit(data[pos]))
        } else {
            None // the lazy step: nothing is emitted yet, just look one ahead
        };

        let Some(pred) = proposal else {
            match_available = true;
            prev_len = cur_len;
            prev_start = cur_start;
            strstart += 1;
            continue;
        };

        let tok = decide(idx, pos, pred);
        idx += 1;
        out.push(tok);
        let span = tok.span();

        if tok == pred && !cfg.greedy() && matches!(tok, Tok::Lit(_)) {
            // zlib's own literal step keeps the lazy candidate alive
            pos += 1;
            prev_len = cur_len;
            prev_start = cur_start;
            strstart += 1;
        } else {
            // a match, a greedy emission, or a FORCED token: insert every
            // position the token covers, then restart clean just past it
            if span > 1 {
                ch.insert_upto(data, pos + span - 1);
            }
            pos += span;
            strstart = pos;
            match_available = false;
            prev_len = MIN_MATCH - 1;
        }
    }
    if match_available && pos < n {
        let pred = Tok::Lit(data[pos]);
        out.push(decide(idx, pos, pred));
    }
    out
}

/// Walk in lockstep with `actual`, forcing it wherever the prediction differs.
/// Returns the stream produced and the corrections as (token index, forced
/// token). The stream IS `actual` whenever this succeeds; every caller verifies
/// that rather than assuming it.
pub fn lockstep(data: &[u8], cfg: Cfg, actual: &[Tok]) -> (Vec<Tok>, Vec<(u32, Tok)>) {
    let mut corr: Vec<(u32, Tok)> = Vec::new();
    let got = drive(data, cfg, |i, _, pred| match actual.get(i) {
        Some(&a) => {
            if a != pred {
                corr.push((i as u32, a));
            }
            a
        }
        None => pred,
    });
    (got, corr)
}

/// Rebuild the parse from the inferred parameters and the corrections -- the
/// decoder's half, driven by exactly the same loop.
pub fn replay(data: &[u8], cfg: Cfg, corr: &[(u32, Tok)]) -> Vec<Tok> {
    let mut k = 0usize;
    drive(data, cfg, |i, _, pred| {
        if k < corr.len() && corr[k].0 as usize == i {
            let t = corr[k].1;
            k += 1;
            t
        } else {
            pred
        }
    })
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

/// Try every (level, memLevel) zlib could have used and return the one needing
/// the fewest corrections IN LOCKSTEP, with that count. This is the step v12
/// skipped: the parameters are read out of the stream, not assumed.
pub fn infer(data: &[u8], actual: &[Tok]) -> (Cfg, usize) {
    let mut best = (Cfg::new(6, 8), usize::MAX);
    for level in 1..=9u8 {
        for mem_level in 1..=9u8 {
            let cfg = Cfg::new(level, mem_level);
            let (got, corr) = lockstep(data, cfg, actual);
            // a config that cannot reproduce the stream is no fit at all. NOTE
            // this demands `data` and `actual` cover the same bytes exactly: a
            // sample cut mid-token fails here for EVERY config and silently
            // returns the default. The caller aligns the sample; this asserts it.
            debug_assert!(
                actual.iter().map(|t| t.span()).sum::<usize>() == data.len(),
                "infer was handed a token slice that does not cover its data"
            );
            let d = if got == actual { corr.len() } else { usize::MAX };
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

    /// a token stream that does not re-spell its own data is not a parse,
    /// whatever its agreement rate
    fn replay_bytes(toks: &[Tok]) -> Vec<u8> {
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

    fn sample() -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..40000u32 {
            v.push((i.wrapping_mul(2654435761) >> 13) as u8);
        }
        let head = v[..15000].to_vec();
        v.extend_from_slice(&head); // a long repeat the matcher must find
        v.extend(b"the same words the same words the same words".iter().copied());
        v
    }

    #[test]
    fn parse_replays_its_input() {
        let v = sample();
        for cfg in [Cfg::new(1, 8), Cfg::new(4, 9), Cfg::new(6, 8), Cfg::new(9, 9)] {
            let toks = replay(&v, cfg, &[]);
            assert_eq!(replay_bytes(&toks), v, "parse did not replay its own input at {:?}", cfg);
            assert!(toks.len() < v.len(), "no matches found at {:?}", cfg);
        }
    }

    #[test]
    fn matches_are_legal() {
        let mut v: Vec<u8> = b"abcabcabcabc".repeat(500);
        v.extend(b"zzzz".repeat(300));
        let toks = replay(&v, Cfg::new(6, 8), &[]);
        let mut p = 0usize;
        for t in &toks {
            if let Tok::Match(l, d) = t {
                assert!((3..=258).contains(l), "illegal length {}", l);
                assert!(*d >= 1 && (*d as usize) <= p, "illegal distance {} at {}", d, p);
            }
            p += t.span();
        }
        assert_eq!(p, v.len());
    }

    #[test]
    fn tiny_inputs() {
        for n in 0..4usize {
            let v: Vec<u8> = (0..n as u8).collect();
            assert_eq!(replay_bytes(&replay(&v, Cfg::new(6, 8), &[])), v);
        }
    }

    /// THE LOCKSTEP LAW: whatever the target parse is, lockstep reproduces it
    /// exactly and `replay` rebuilds it from the corrections alone. The target
    /// here is a DIFFERENT config's parse, so corrections are many and the
    /// forcing path is exercised hard -- including forcing a literal where the
    /// matcher wanted a match, which is the resync that a free-running design
    /// cannot express.
    #[test]
    fn lockstep_reproduces_a_foreign_parse() {
        let v = sample();
        for (a, b) in [((9u8, 9u8), (4u8, 9u8)), ((1, 8), (6, 8)), ((6, 8), (1, 8))] {
            let target = replay(&v, Cfg::new(a.0, a.1), &[]);
            let cfg = Cfg::new(b.0, b.1);
            let (got, corr) = lockstep(&v, cfg, &target);
            assert_eq!(got, target, "lockstep did not reproduce the {:?} parse under {:?}", a, b);
            assert!(!corr.is_empty(), "a foreign config should need corrections ({:?} vs {:?})", a, b);
            assert_eq!(replay(&v, cfg, &corr), target, "replay did not rebuild the parse from its corrections");
            assert_eq!(replay_bytes(&got), v, "the reproduced parse does not re-spell the data");
        }
    }

    /// with the matching config the corrections are empty and replay is a no-op
    #[test]
    fn lockstep_on_its_own_parse_needs_no_corrections() {
        let v = sample();
        for cfg in [Cfg::new(1, 8), Cfg::new(4, 9), Cfg::new(6, 8), Cfg::new(9, 9)] {
            let own = replay(&v, cfg, &[]);
            let (got, corr) = lockstep(&v, cfg, &own);
            assert_eq!(got, own, "lockstep diverged from its own parse at {:?}", cfg);
            assert!(corr.is_empty(), "{} corrections against its own parse at {:?}", corr.len(), cfg);
            assert_eq!(replay(&v, cfg, &corr), own);
        }
    }

    /// infer must FIND the config a stream was made with, not merely score it
    #[test]
    fn infer_recovers_the_config() {
        let v = sample();
        for (lvl, ml) in [(4u8, 9u8), (6, 8), (9, 9)] {
            let target = replay(&v, Cfg::new(lvl, ml), &[]);
            let (cfg, d) = infer(&v, &target);
            assert_eq!(d, 0, "infer settled for {} corrections on a {}/{} stream", d, lvl, ml);
            assert_eq!(replay(&v, cfg, &[]), target, "the inferred config does not reproduce the stream");
        }
    }
}
