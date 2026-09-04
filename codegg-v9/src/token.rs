//! token.rs -- the bar, generalized.
//!
//! The site's bar notation says "this run repeats"; at arbitrary offsets that
//! is honestly LZ77 (Ziv-Lempel 1977) and we say so. A hash-chain match
//! finder walks the whole file (no 32 KB porthole -- gzip's window is its
//! deepest handicap on the corpus's file-scale repetition) and emits
//! literals and (length, offset) matches. codegg-v2's GREEN insight -- zero
//! runs are the commonest structure -- falls out as a natural match case
//! (offset 1..4 self-overlap). v2's NAF/DIV recipes are dropped: measured at
//! file scale they ~never fired; the ledger is in codegg-v2/README.md.

pub const MIN_MATCH: usize = 4;
const HASH_BITS: u32 = 18;
const MAX_CHAIN: usize = 1024; // v9-M6: doubled (measured; revert if <0.05pt)
const GOOD_LEN: usize = 128; // a match this long ends the chain walk early
const MAX_LAZY: usize = 64; // matches at least this long are taken greedily

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tok {
    Lit(u8),
    Match { len: u32, dist: u32 },
}

/// the four most recent offsets, move-to-front (LZMA's rep machinery,
/// attributed to Igor Pavlov): structured data reuses the same few strides
/// over and over, and naming "the stride again" costs ~3 bits instead of ~25.
/// The tokenizer, the coder and the decoder all walk this identically.
pub fn rep_update(reps: &mut [u32; 4], dist: u32) {
    if let Some(k) = reps.iter().position(|&r| r == dist) {
        for i in (1..=k).rev() {
            reps[i] = reps[i - 1];
        }
    } else {
        reps[3] = reps[2];
        reps[2] = reps[1];
        reps[1] = reps[0];
    }
    reps[0] = dist;
}

#[inline]
fn slot_of(dist: u32) -> u32 {
    31 - dist.leading_zeros()
}

/// what a literal byte of THIS file actually costs the dyadic stage:
/// the order-2-nib conditional entropy, measured in one pass -- the same
/// statistic the coder's literal model converges to. Scaled x8 for integer
/// math, clamped to [2, 9] bits. A flat guess of 6 bits/literal made the
/// finder keep matches seeded inside random hex fields whose nearest
/// recurrence is megabytes away -- matches that cost MORE to name than the
/// bytes cost to spell (convicted by --stats on server-log/data.csv).
pub fn lit_price8(src: &[u8]) -> i64 {
    let mut h = vec![0u64; 4096]; // 256 nib-pair contexts x 16 next-nib counts
    let mut ctx = 0usize;
    for &b in src {
        for nib in [(b >> 4) as usize, (b & 15) as usize] {
            h[ctx * 16 + nib] += 1;
            ctx = ((ctx << 4) | nib) & 0xff;
        }
    }
    let mut bits = 0f64;
    for c in 0..256 {
        let tot: u64 = h[c * 16..c * 16 + 16].iter().sum();
        if tot == 0 {
            continue;
        }
        let lt = (tot as f64).log2();
        for v in 0..16 {
            let n = h[c * 16 + v];
            if n > 0 {
                bits += (n as f64) * (lt - (n as f64).log2());
            }
        }
    }
    let per_byte = bits / src.len().max(1) as f64;
    ((8.0 * per_byte).round() as i64).clamp(16, 72)
}

/// price of a match in bits x8, against literals at price8 bits x8 per byte.
/// A rep offset costs ~a rep flag + index; a fresh offset costs its slot.
#[inline]
fn gain_of(len: usize, dist: u32, reps: &[u32; 4], price8: i64) -> i64 {
    let name_cost = if let Some(k) = reps.iter().position(|&r| r == dist) {
        9 + k as i64
    } else {
        10 + slot_of(dist) as i64
    };
    price8 * len as i64 - 8 * name_cost
}

#[inline]
fn hash4(src: &[u8], pos: usize) -> usize {
    let v = u32::from_le_bytes([src[pos], src[pos + 1], src[pos + 2], src[pos + 3]]);
    (v.wrapping_mul(2654435761) >> (32 - HASH_BITS)) as usize
}

#[inline]
fn extend(src: &[u8], a: usize, b: usize) -> usize {
    // match length of src[a..] against src[b..], a < b; overlap is legal
    let n = src.len();
    let mut l = 0;
    let max = n - b;
    while l < max && src[a + l] == src[b + l] {
        l += 1;
    }
    l
}

/// best (len, dist, gain) at pos, PRICE-AWARE: candidates are scored by what
/// they save against spelling the bytes, not by raw length. The first cut of
/// this function kept the longest match; the stats convicted it -- offsets
/// were 57% of server-log.json's output and 76% of data.csv's, because a
/// one-byte-longer match four megabytes away beat the same line one line up.
/// The chain is nearest-first, so a skipped not-longer candidate is always
/// also farther, i.e. strictly worse -- the quick reject stays sound.
fn best_at(
    src: &[u8],
    head: &[u32],
    prev: &[u32],
    pos: usize,
    reps: &[u32; 4],
    price8: i64,
) -> (usize, u32, i64) {
    let n = src.len();
    if pos + MIN_MATCH > n {
        return (0, 0, 0);
    }
    let mut best: (usize, u32, i64) = (0, 0, 0); // len, dist, gain > 0
    for &rd in reps.iter() {
        if rd != 0 && (rd as usize) <= pos {
            let l = extend(src, pos - rd as usize, pos);
            if l >= MIN_MATCH {
                let g = gain_of(l, rd, reps, price8);
                if g > best.2 {
                    best = (l, rd, g);
                }
            }
        }
    }
    let mut c = head[hash4(src, pos)];
    let mut chain = 0;
    while c != u32::MAX && chain < MAX_CHAIN {
        let cp = c as usize;
        // quick reject: the byte just past the current best must match
        if pos + best.0 < n && src[cp + best.0] == src[pos + best.0] {
            let l = extend(src, cp, pos);
            if l >= MIN_MATCH {
                let g = gain_of(l, (pos - cp) as u32, reps, price8);
                if g > best.2 {
                    best = (l, (pos - cp) as u32, g);
                    if pos + best.0 >= n || best.0 >= GOOD_LEN {
                        break;
                    }
                }
            }
        }
        c = prev[cp];
        chain += 1;
    }
    best
}

/// re-check a cached probe at its position: the only candidates a probe can
/// have missed are the positions inserted AFTER it ran -- on the literal
/// path those are exactly the last one or two bytes, i.e. distances 1 and 2.
/// Probing them unconditionally is sound (any candidate may only raise the
/// gain) and costs two extends.
#[inline]
fn refresh(src: &[u8], pos: usize, reps: &[u32; 4], price8: i64, mut best: (usize, u32, i64)) -> (usize, u32, i64) {
    for d in [1u32, 2] {
        if (d as usize) <= pos {
            let l = extend(src, pos - d as usize, pos);
            if l >= MIN_MATCH {
                let g = gain_of(l, d, reps, price8);
                if g > best.2 {
                    best = (l, d, g);
                }
            }
        }
    }
    best
}

pub fn tokenize(src: &[u8]) -> Vec<Tok> {
    tokenize_priced(src, lit_price8(src))
}

/// try-again stage: a SPARSE-LZ pass -- only matches of at least `min_len`
/// are emitted (the format's MIN_MATCH stays 4; emitting longer is legal),
/// so the mixing model keeps the short repeats it reads better than the
/// token layer can name them. Encoder-only, trial-gated per file.
pub fn tokenize_min(src: &[u8], price8: i64, min_len: usize) -> Vec<Tok> {
    let toks = tokenize_priced(src, price8);
    // re-tokenize is overkill: demoting short matches to literals is the
    // same decision the tokenizer would make with the higher floor, and
    // costs one pass
    let mut out = Vec::with_capacity(toks.len());
    let mut pos = 0usize;
    for t in toks {
        match t {
            Tok::Lit(b) => {
                out.push(Tok::Lit(b));
                pos += 1;
            }
            Tok::Match { len, dist } => {
                if (len as usize) < min_len {
                    for i in 0..len as usize {
                        out.push(Tok::Lit(src[pos + i]));
                    }
                } else {
                    out.push(Tok::Match { len, dist });
                }
                pos += len as usize;
            }
        }
    }
    out
}

/// v9-M6, the fastest-route reading (spec.md:58-59: "Push is the address;
/// NAF is the fastest route to it"): a second tokenize pass fed the MEASURED
/// literal price from a first encode, instead of the static o2-nib guess.
/// Encoder-only -- tokens are the interface; the decoder never knows.
pub fn tokenize_priced(src: &[u8], price8: i64) -> Vec<Tok> {
    let n = src.len();
    let mut toks = Vec::new();
    if n < MIN_MATCH + 1 {
        for &b in src {
            toks.push(Tok::Lit(b));
        }
        return toks;
    }
    let mut head = vec![u32::MAX; 1 << HASH_BITS];
    let mut prev = vec![u32::MAX; n];
    let mut reps: [u32; 4] = [0; 4];
    let mut pos = 0usize;
    let insert_limit = n - MIN_MATCH + 1;
    // the lazy-probe cache (WS5): the lazy checks below compute best_at for
    // pos+1 (and pos+2); when the literal path advances one byte those
    // probes ARE the next best_at -- v7 threw them away and recomputed.
    // Sound to reuse: no rep_update happens on the literal path, and the
    // bytes inserted since are re-checked by refresh().
    let mut c1: Option<(usize, u32, i64)> = None; // best_at(pos), precomputed
    let mut c2: Option<(usize, u32, i64)> = None; // best_at(pos+1), precomputed
    macro_rules! insert {
        ($p:expr) => {
            if $p < insert_limit {
                let h = hash4(src, $p);
                prev[$p] = head[h];
                head[h] = $p as u32;
            }
        };
    }
    while pos < n {
        let (len, dist, gain) = match c1.take() {
            Some(b) => refresh(src, pos, &reps, price8, b),
            None => best_at(src, &head, &prev, pos, &reps, price8),
        };
        if len >= MIN_MATCH && gain > 0 {
            if len < MAX_LAZY && pos + 1 < n {
                // one-step lazy (gzip's move): a better match one byte later
                let b1 = match c2.take() {
                    Some(b) => refresh(src, pos + 1, &reps, price8, b),
                    None => best_at(src, &head, &prev, pos + 1, &reps, price8),
                };
                if b1.2 > gain {
                    toks.push(Tok::Lit(src[pos]));
                    insert!(pos);
                    pos += 1;
                    c1 = Some(b1);
                    continue;
                }
                // two-step lazy (WS5): a match two bytes later wins only if
                // it also pays for the extra literal (price8 = one byte)
                if pos + 2 < n {
                    let b2 = best_at(src, &head, &prev, pos + 2, &reps, price8);
                    if b2.2 > gain + price8 {
                        toks.push(Tok::Lit(src[pos]));
                        insert!(pos);
                        pos += 1;
                        c1 = Some(b1);
                        c2 = Some(b2);
                        continue;
                    }
                }
            }
            c1 = None;
            c2 = None;
            toks.push(Tok::Match { len: len as u32, dist });
            rep_update(&mut reps, dist);
            for p in pos..pos + len {
                insert!(p);
            }
            pos += len;
        } else {
            c1 = c2.take().map(|b| b); // best_at(pos+1) if a probe left one
            toks.push(Tok::Lit(src[pos]));
            insert!(pos);
            pos += 1;
        }
    }
    toks
}

// ---------- M1 raw form: LZ4-style sequences, literals as raw nibs ----------
// [varint lit_count][literal bytes][varint match_len][varint dist] repeated;
// the stream ends when the restored length reaches the original length, so a
// trailing literal run needs no terminator. This form exists to measure the
// token layer ALONE; the dyadic stage replaces it as the shipped form.

fn put_varint(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            break;
        }
        out.push(b | 0x80);
    }
}
fn get_varint(inp: &[u8], pos: &mut usize) -> Result<u64, String> {
    let mut v = 0u64;
    let mut shift = 0u32;
    loop {
        if *pos >= inp.len() {
            return Err("varint ran off the stream".into());
        }
        let b = inp[*pos];
        *pos += 1;
        v |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            return Ok(v);
        }
        shift += 7;
        if shift > 63 {
            return Err("varint too long".into());
        }
    }
}

pub fn tokens_serialize(toks: &[Tok]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut lits: Vec<u8> = Vec::new();
    let flush = |out: &mut Vec<u8>, lits: &mut Vec<u8>| {
        put_varint(out, lits.len() as u64);
        out.extend_from_slice(lits);
        lits.clear();
    };
    for t in toks {
        match *t {
            Tok::Lit(b) => lits.push(b),
            Tok::Match { len, dist } => {
                flush(&mut out, &mut lits);
                put_varint(&mut out, len as u64);
                put_varint(&mut out, dist as u64);
            }
        }
    }
    if !lits.is_empty() {
        flush(&mut out, &mut lits);
    }
    out
}

pub fn tokens_restore(inp: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(orig_len);
    let mut pos = 0usize;
    while out.len() < orig_len {
        let nlit = get_varint(inp, &mut pos)? as usize;
        if pos + nlit > inp.len() || out.len() + nlit > orig_len {
            return Err("literal run overruns".into());
        }
        out.extend_from_slice(&inp[pos..pos + nlit]);
        pos += nlit;
        if out.len() >= orig_len {
            break;
        }
        let len = get_varint(inp, &mut pos)? as usize;
        let dist = get_varint(inp, &mut pos)? as usize;
        if len < MIN_MATCH || dist == 0 || dist > out.len() || out.len() + len > orig_len {
            return Err("malformed match".into());
        }
        for _ in 0..len {
            let b = out[out.len() - dist];
            out.push(b);
        }
    }
    Ok(out)
}
