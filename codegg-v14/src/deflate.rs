//! deflate.rs -- WS-D, THE DEFLATE PEEL (v13-M2). The second client of the peel
//! frame, and the one the charter was written for.
//!
//! A deflate stream and the bytes it codes are the same value in two spellings.
//! v12 shut this door by searching for the encoder: 1,260 zlib configurations,
//! best agreement 3 bytes of 4,096, first difference at byte 0. The key is not
//! to find the encoder; it is to store what the STREAM ITSELF SAYS -- the block
//! boundaries and types, the Huffman code definitions exactly as declared, and
//! the literal/length/distance parse -- and re-emit deflate from that, bit for
//! bit, for any encoder that ever existed. That is preflate's approach (Dirk
//! Steinke) and precomp's (Christian Schneider); RFC 1951 is the codec.
//!
//! What the recipe carries, and what it does NOT:
//!   * NOT the literal VALUES. They are read back out of the inflated output at
//!     re-spell time, which is the whole reason the flags stream exists.
//!   * the block structure, and for dynamic blocks HLIT/HDIST/HCLEN, the 19
//!     code-length code lengths and the RLE-coded lit/dist lengths as symbols;
//!   * one u32 of TOKEN COUNT per block -- an inflater learns where a block ends
//!     from the end-of-block symbol, a re-speller must be told (the gate found
//!     this hole in python before any Rust was written);
//!   * every match's length and distance;
//!   * the FINAL PADDING BITS (the second hole the same gate found).
//!
//! THE LAW OF THE PEEL applies with no exception: main.rs re-spells and compares
//! against the original bytes before anything is written, and one byte of
//! difference discards the peel for that file.
//!
//! Ported from the proven python (scratchpad v13/recipe/tokens.py and
//! respell.py), which round-tripped aoe4-autosave.sav exactly: 1,171 blocks,
//! 38,340,574 tokens, 66,417,533 B re-spelled bit for bit.

// ---------------------------------------------------------------- RFC 1951 tables

const LBASE: [u32; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEXTRA: [u32; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DBASE: [u32; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DEXTRA: [u32; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const CLORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

/// the length symbol of a length
#[inline]
pub fn lsym_of(n: u32) -> usize {
    let mut i = 28;
    while LBASE[i] > n {
        i -= 1;
    }
    i
}
/// the distance symbol of a distance
#[inline]
pub fn dsym_of(d: u32) -> usize {
    let mut i = 29;
    while DBASE[i] > d {
        i -= 1;
    }
    i
}
// ---------------------------------------------------------------- the wrappers

pub const WRAP_RAW: u8 = 0;
pub const WRAP_GZIP: u8 = 1;
pub const WRAP_ZLIB: u8 = 2;
pub const WRAP_PNG: u8 = 3;

/// a bound on what a peel will inflate, so a bomb refuses instead of eating the
/// machine. Far past anything in the corpus and far short of the RAM.
const VALUES_MAX: usize = 2 << 30;

/// what the peel produced. `values` are the inflated bytes (modelled by the
/// ordinary roster); everything else is the recipe.
pub struct Deflate {
    pub wrap: u8,
    /// bytes before the member region (PNG: the signature and every chunk up to
    /// the first IDAT payload)
    pub pre: Vec<u8>,
    /// the wrapper header at the start of the member region, verbatim
    pub head: Vec<u8>,
    /// the wrapper trailer at the end of the member region, verbatim
    pub tail: Vec<u8>,
    /// how the member region splits back into chunks: (payload length, the
    /// bytes that follow it). Empty means one contiguous member.
    pub segs: Vec<(u32, Vec<u8>)>,
    pub meta: Vec<u8>,
    pub flags: Vec<u8>,
    /// match lengths, stored as length - 3: RFC 1951 lengths are 3..258, so
    /// they are ONE byte, not two (the python proof spent a u16 on each)
    pub lens: Vec<u8>,
    /// match distances, 1..32768: they are TWO bytes, not four
    pub dists: Vec<u16>,
    /// THE SPELLING EXCEPTION, sparse. Length 258 has TWO legal spellings:
    /// symbol 285 with no extra bits, and symbol 284 with its 31 (LBASE[27] =
    /// 227, LEXTRA[27] = 5). The recipe stores lengths, not spellings, so the
    /// second one would come back as the first and the re-spell would not be
    /// bit-exact. This is the list of MATCH INDICES that used it -- ascending,
    /// and empty for every encoder that spells 258 the ordinary way, which is
    /// every file in this corpus. wubbadub.html:663-666 is the rule being
    /// followed: record the canonicalisation, do not refuse it.
    ///
    /// Why a fifth stream and not a spare bit: the flags stream carries exactly
    /// one bit per TOKEN with no slack, and widening it to a bit per MATCH
    /// costs +3.2 MB on aoe4-autosave.sav (25,661,244 matches) and destroys the
    /// byte periodicity its roster arm is exploiting. Bit 15 of `dists` is free
    /// in size but breaks the distance-sequence repetition the LZ arm wins on.
    /// `lens` has no spare values: all 256 are in use for 3..=258.
    pub resp: Vec<u32>,
    pub nblocks: u32,
    pub ntok: u32,
    pub pad_bits: u8,
    pub pad_val: u8,
    pub values: Vec<u8>,
}

impl Deflate {
    pub fn describe(&self) -> String {
        format!(
            "deflate {}: {} blocks, {} tokens ({} matches, {} spelled 284), {} B inflated; recipe meta {} + flags {} + lens {} + dists {} + resp {} B",
            match self.wrap {
                WRAP_GZIP => "gzip",
                WRAP_ZLIB => "zlib",
                WRAP_PNG => "PNG IDAT",
                _ => "raw",
            },
            self.nblocks,
            self.ntok,
            self.lens.len(),
            self.resp.len(),
            self.values.len(),
            self.meta.len(),
            self.flags.len(),
            self.lens.len(),
            self.dists.len() * 2,
            self.resp.len() * 4
        )
    }
}

// ---------------------------------------------------------------- the bit reader

struct Bits<'a> {
    d: &'a [u8],
    pos: usize,
    acc: u64,
    n: u32,
}
impl<'a> Bits<'a> {
    fn new(d: &'a [u8]) -> Bits<'a> {
        Bits { d, pos: 0, acc: 0, n: 0 }
    }
    #[inline]
    fn need(&mut self, k: u32) -> Result<(), String> {
        while self.n < k {
            if self.pos >= self.d.len() {
                return Err("the deflate stream ends inside a code".into());
            }
            self.acc |= (self.d[self.pos] as u64) << self.n;
            self.pos += 1;
            self.n += 8;
        }
        Ok(())
    }
    /// fill toward k bits but do not fail at the end of the stream
    #[inline]
    fn fill(&mut self, k: u32) {
        while self.n < k && self.pos < self.d.len() {
            self.acc |= (self.d[self.pos] as u64) << self.n;
            self.pos += 1;
            self.n += 8;
        }
    }
    #[inline]
    fn get(&mut self, k: u32) -> Result<u32, String> {
        if k == 0 {
            return Ok(0);
        }
        self.need(k)?;
        let v = (self.acc & ((1u64 << k) - 1)) as u32;
        self.acc >>= k;
        self.n -= k;
        Ok(v)
    }
    fn align(&mut self) {
        let drop = self.n & 7;
        self.acc >>= drop;
        self.n -= drop;
    }
    /// the byte position of the next unread bit -- whole bytes still sitting in
    /// the accumulator count as UNREAD. (The python this is ported from did not
    /// subtract them; on the save it happened not to matter, and it is exactly
    /// the kind of off-by-a-byte a port should not inherit.)
    fn at(&self) -> usize {
        self.pos - (self.n as usize) / 8
    }
    fn seek(&mut self, at: usize) {
        self.pos = at;
        self.acc = 0;
        self.n = 0;
    }
}

// ---------------------------------------------------------------- canonical Huffman

/// a flat decode table keyed by the next `maxlen` stream bits; each entry is
/// (symbol << 5) | length, and 0 means "no code here"
struct Huff {
    fast: Vec<u32>,
    maxlen: u32,
}
#[inline]
fn rev_bits(mut c: u32, l: u32) -> u32 {
    let mut r = 0;
    for _ in 0..l {
        r = (r << 1) | (c & 1);
        c >>= 1;
    }
    r
}
/// canonical codes from code lengths. None is an all-zero table -- a legal way
/// of saying "this alphabet is unused".
fn build(lengths: &[u8]) -> Result<Option<Huff>, String> {
    let maxlen = lengths.iter().copied().max().unwrap_or(0) as u32;
    if maxlen == 0 {
        return Ok(None);
    }
    if maxlen > 15 {
        return Err(format!("a code length of {} exceeds RFC 1951's 15", maxlen));
    }
    let mut bl = [0u32; 16];
    for &l in lengths {
        if l > 0 {
            bl[l as usize] += 1;
        }
    }
    let mut code = 0u32;
    let mut next = [0u32; 16];
    for b in 1..=maxlen {
        code = (code + bl[(b - 1) as usize]) << 1;
        next[b as usize] = code;
        if code + bl[b as usize] > (1u32 << b) {
            return Err("an over-subscribed Huffman table".into());
        }
    }
    let mut fast = vec![0u32; 1usize << maxlen];
    for (sym, &l) in lengths.iter().enumerate() {
        if l == 0 {
            continue;
        }
        let l = l as u32;
        let c = next[l as usize];
        next[l as usize] += 1;
        let step = 1usize << l;
        let entry = ((sym as u32) << 5) | l;
        let mut i = rev_bits(c, l) as usize;
        while i < fast.len() {
            fast[i] = entry;
            i += step;
        }
    }
    Ok(Some(Huff { fast, maxlen }))
}
#[inline]
fn decode_sym(br: &mut Bits, h: &Huff) -> Result<u32, String> {
    br.fill(h.maxlen);
    let e = h.fast[(br.acc as u32 & ((1u32 << h.maxlen) - 1)) as usize];
    let l = e & 31;
    if e == 0 || l > br.n {
        return Err("an invalid Huffman code in the deflate stream".into());
    }
    br.acc >>= l;
    br.n -= l;
    Ok(e >> 5)
}

fn fixed_lit() -> Vec<u8> {
    let mut v = vec![8u8; 144];
    v.extend(std::iter::repeat_n(9u8, 112));
    v.extend(std::iter::repeat_n(7u8, 24));
    v.extend(std::iter::repeat_n(8u8, 8));
    v
}

// ---------------------------------------------------------------- the walk

/// walk one deflate body, filling the recipe streams and inflating into
/// `values`. Returns the number of BYTES of `body` the stream occupied.
#[allow(clippy::too_many_lines)] // one walk, in RFC 1951's own order; splitting
                                 // it would hide the order the bits are read in
fn walk(body: &[u8], d: &mut Deflate) -> Result<usize, String> {
    let mut br = Bits::new(body);
    let mut flag_acc = 0u8;
    let mut flag_n = 0u32;
    let mut ntok = 0u32;
    let mut nblocks = 0u32;
    let fixed_l = fixed_lit();
    let fixed_d = vec![5u8; 30];
    loop {
        let final_bit = br.get(1)?;
        let btype = br.get(2)?;
        if btype == 3 {
            return Err("block type 3 is reserved and is not deflate".into());
        }
        d.meta.push(((final_bit << 2) | btype) as u8);
        nblocks += 1;
        if btype == 0 {
            br.align();
            let ln = br.get(16)?;
            let nln = br.get(16)?;
            if nln != (!ln & 0xFFFF) {
                return Err(format!("a stored block whose NLEN {} is not the complement of LEN {}", nln, ln));
            }
            d.meta.extend_from_slice(&(ln as u16).to_le_bytes());
            d.meta.extend_from_slice(&(nln as u16).to_le_bytes());
            let at = br.at();
            if at + ln as usize > body.len() {
                return Err("a stored block runs past the end of the stream".into());
            }
            if d.values.len() + ln as usize > VALUES_MAX {
                return Err("the inflated side exceeds the peel's ceiling".into());
            }
            d.values.extend_from_slice(&body[at..at + ln as usize]);
            br.seek(at + ln as usize);
            if final_bit == 1 {
                break;
            }
            continue;
        }
        let (litlen, distlen) = if btype == 1 {
            (fixed_l.clone(), fixed_d.clone())
        } else {
            let hlit = br.get(5)? as usize + 257;
            let hdist = br.get(5)? as usize + 1;
            let hclen = br.get(4)? as usize + 4;
            d.meta.push((hlit - 257) as u8);
            d.meta.push((hdist - 1) as u8);
            d.meta.push((hclen - 4) as u8);
            let mut cl = [0u8; 19];
            for i in 0..hclen {
                cl[CLORDER[i]] = br.get(3)? as u8;
            }
            d.meta.extend_from_slice(&cl);
            let clt = build(&cl)?.ok_or("a dynamic block with no code-length code")?;
            let total = hlit + hdist;
            let mut lens: Vec<u8> = Vec::with_capacity(total);
            while lens.len() < total {
                let s = decode_sym(&mut br, &clt)?;
                match s {
                    0..=15 => {
                        lens.push(s as u8);
                        d.meta.push(s as u8);
                    }
                    16..=18 => {
                        let (nb, base) = match s {
                            16 => (2, 3),
                            17 => (3, 3),
                            _ => (7, 11),
                        };
                        let r = br.get(nb)? + base;
                        d.meta.push(s as u8);
                        d.meta.push(r as u8);
                        let v = if s == 16 { *lens.last().ok_or("a repeat with nothing to repeat")? } else { 0 };
                        lens.resize(lens.len() + r as usize, v);
                    }
                    _ => return Err(format!("code-length symbol {} is not a symbol", s)),
                }
            }
            if lens.len() != total {
                return Err(format!("the code lengths overran their table by {}", lens.len() - total));
            }
            (lens[..hlit].to_vec(), lens[hlit..].to_vec())
        };
        let lt = build(&litlen)?.ok_or("a block with no literal/length code")?;
        let dt = build(&distlen)?;
        let tok0 = ntok;
        let cntpos = d.meta.len();
        d.meta.extend_from_slice(&[0, 0, 0, 0]);
        loop {
            let s = decode_sym(&mut br, &lt)?;
            if s == 256 {
                break;
            }
            ntok += 1;
            if s < 256 {
                if d.values.len() >= VALUES_MAX {
                    return Err("the inflated side exceeds the peel's ceiling".into());
                }
                d.values.push(s as u8);
            } else {
                let i = s as usize - 257;
                if i > 28 {
                    return Err(format!("length symbol {} is not a length symbol", s));
                }
                let ln = LBASE[i] + br.get(LEXTRA[i])?;
                if ln == 258 && i == 27 {
                    // THE SPELLING EXCEPTION. 258 has two legal spellings (285,
                    // and 284 with its 31 extra bits) and the recipe stores
                    // lengths, not spellings -- so this one is RECORDED by match
                    // index in the sparse fifth stream and re-spelled from that
                    // list. Before M3a the whole file refused here.
                    d.resp.push(d.lens.len() as u32);
                }
                let dtab = dt.as_ref().ok_or("a match in a block with no distance code")?;
                let ds = decode_sym(&mut br, dtab)? as usize;
                if ds > 29 {
                    return Err(format!("distance symbol {} is not a distance symbol", ds));
                }
                let dist = DBASE[ds] + br.get(DEXTRA[ds])?;
                if dist as usize > d.values.len() {
                    return Err(format!("a match reaches {} B back into {} B of output", dist, d.values.len()));
                }
                if d.values.len() + ln as usize > VALUES_MAX {
                    return Err("the inflated side exceeds the peel's ceiling".into());
                }
                let start = d.values.len() - dist as usize;
                if dist >= ln {
                    d.values.extend_from_within(start..start + ln as usize);
                } else {
                    for k in 0..ln as usize {
                        let b = d.values[start + k];
                        d.values.push(b);
                    }
                }
                d.lens.push((ln - 3) as u8);
                d.dists.push(dist as u16);
                flag_acc |= 1 << flag_n;
            }
            flag_n += 1;
            if flag_n == 8 {
                d.flags.push(flag_acc);
                flag_acc = 0;
                flag_n = 0;
            }
        }
        d.meta[cntpos..cntpos + 4].copy_from_slice(&(ntok - tok0).to_le_bytes());
        if final_bit == 1 {
            break;
        }
    }
    if flag_n > 0 {
        d.flags.push(flag_acc);
    }
    d.pad_bits = (br.n & 7) as u8;
    d.pad_val = if d.pad_bits > 0 { (br.acc & ((1u64 << d.pad_bits) - 1)) as u8 } else { 0 };
    br.align();
    d.nblocks = nblocks;
    d.ntok = ntok;
    Ok(br.at())
}

// ---------------------------------------------------------------- the re-spell

struct BitW {
    buf: Vec<u8>,
    acc: u64,
    n: u32,
}
impl BitW {
    fn new(cap: usize) -> BitW {
        BitW { buf: Vec::with_capacity(cap), acc: 0, n: 0 }
    }
    #[inline]
    fn put(&mut self, v: u32, k: u32) {
        if k == 0 {
            return;
        }
        self.acc |= ((v as u64) & ((1u64 << k) - 1)) << self.n;
        self.n += k;
        while self.n >= 8 {
            self.buf.push((self.acc & 0xFF) as u8);
            self.acc >>= 8;
            self.n -= 8;
        }
    }
}
/// canonical (reversed code, length) per symbol, ready to be written LSB-first
fn codes(lengths: &[u8]) -> Vec<(u32, u32)> {
    let maxlen = lengths.iter().copied().max().unwrap_or(0) as u32;
    let mut out = vec![(0u32, 0u32); lengths.len()];
    if maxlen == 0 {
        return out;
    }
    let mut bl = [0u32; 16];
    for &l in lengths {
        if l > 0 {
            bl[l as usize] += 1;
        }
    }
    let mut code = 0u32;
    let mut next = [0u32; 16];
    for b in 1..=maxlen {
        code = (code + bl[(b - 1) as usize]) << 1;
        next[b as usize] = code;
    }
    for (s, &l) in lengths.iter().enumerate() {
        if l > 0 {
            let c = next[l as usize];
            next[l as usize] += 1;
            out[s] = (rev_bits(c, l as u32), l as u32);
        }
    }
    out
}

/// the recipe plus the values -> the original bytes, bit for bit
#[allow(clippy::too_many_lines)]
pub fn respell(d: &Deflate) -> Result<Vec<u8>, String> {
    let mut w = BitW::new(d.values.len() / 3 + 1024);
    let mut mi = 0usize;
    let mut tok = 0usize;
    let mut li = 0usize;
    let mut di = 0usize;
    let mut ri = 0usize;
    let mut pos = 0usize;
    let fixed_l = fixed_lit();
    let fixed_d = vec![5u8; 30];
    let m = &d.meta;
    for _ in 0..d.nblocks {
        if mi >= m.len() {
            return Err("the recipe's meta stream ended early".into());
        }
        let hdr = m[mi];
        mi += 1;
        let final_bit = ((hdr >> 2) & 1) as u32;
        let btype = (hdr & 3) as u32;
        w.put(final_bit, 1);
        w.put(btype, 2);
        if btype == 0 {
            if mi + 4 > m.len() {
                return Err("the recipe's meta stream ended inside a stored block".into());
            }
            let ln = u16::from_le_bytes([m[mi], m[mi + 1]]) as u32;
            let nln = u16::from_le_bytes([m[mi + 2], m[mi + 3]]) as u32;
            mi += 4;
            if w.n > 0 {
                w.put(0, 8 - w.n);
            }
            w.put(ln, 16);
            w.put(nln, 16);
            if pos + ln as usize > d.values.len() {
                return Err("a stored block runs past the inflated bytes".into());
            }
            w.buf.extend_from_slice(&d.values[pos..pos + ln as usize]);
            pos += ln as usize;
            if final_bit == 1 {
                break;
            }
            continue;
        }
        let (litlen, distlen) = if btype == 1 {
            (fixed_l.clone(), fixed_d.clone())
        } else {
            if mi + 22 > m.len() {
                return Err("the recipe's meta stream ended inside a code-length table".into());
            }
            let hlit = m[mi] as usize + 257;
            let hdist = m[mi + 1] as usize + 1;
            let hclen = m[mi + 2] as usize + 4;
            mi += 3;
            if hclen > 19 {
                return Err("a recipe HCLEN beyond 19".into());
            }
            let cl: Vec<u8> = m[mi..mi + 19].to_vec();
            mi += 19;
            w.put((hlit - 257) as u32, 5);
            w.put((hdist - 1) as u32, 5);
            w.put((hclen - 4) as u32, 4);
            for i in 0..hclen {
                w.put(cl[CLORDER[i]] as u32, 3);
            }
            let clc = codes(&cl);
            let total = hlit + hdist;
            let mut la: Vec<u8> = Vec::with_capacity(total);
            while la.len() < total {
                if mi >= m.len() {
                    return Err("the recipe's meta stream ended inside a code-length list".into());
                }
                let s = m[mi];
                mi += 1;
                match s {
                    0..=15 => {
                        let (c, l) = clc[s as usize];
                        if l == 0 {
                            return Err(format!("the recipe asks for code-length symbol {} which this table does not define", s));
                        }
                        w.put(c, l);
                        la.push(s);
                    }
                    16..=18 => {
                        if mi >= m.len() {
                            return Err("the recipe's meta stream ended inside a repeat".into());
                        }
                        let r = m[mi] as u32;
                        mi += 1;
                        let (c, l) = clc[s as usize];
                        if l == 0 {
                            return Err(format!("the recipe asks for code-length symbol {} which this table does not define", s));
                        }
                        w.put(c, l);
                        let (base, nb, val): (u32, u32, u8) = match s {
                            16 => (3, 2, *la.last().ok_or("a repeat with nothing to repeat")?),
                            17 => (3, 3, 0),
                            _ => (11, 7, 0),
                        };
                        if r < base || r - base >= (1u32 << nb) {
                            return Err("a repeat count outside its field".into());
                        }
                        w.put(r - base, nb);
                        la.resize(la.len() + r as usize, val);
                    }
                    _ => return Err(format!("code-length symbol {} is not a symbol", s)),
                }
            }
            if la.len() != total {
                return Err("the recipe's code lengths overran their table".into());
            }
            (la[..hlit].to_vec(), la[hlit..].to_vec())
        };
        if mi + 4 > m.len() {
            return Err("the recipe's meta stream ended before a token count".into());
        }
        let ntok_blk = u32::from_le_bytes([m[mi], m[mi + 1], m[mi + 2], m[mi + 3]]) as usize;
        mi += 4;
        let lc = codes(&litlen);
        let dc = codes(&distlen);
        for _ in 0..ntok_blk {
            if tok >> 3 >= d.flags.len() {
                return Err("the recipe's flag stream ended early".into());
            }
            let is_match = (d.flags[tok >> 3] >> (tok & 7)) & 1 == 1;
            if is_match {
                if li >= d.lens.len() || di >= d.dists.len() {
                    return Err("the recipe's length or distance stream ended early".into());
                }
                let ln = d.lens[li] as u32 + 3;
                li += 1;
                let dd = d.dists[di] as u32;
                di += 1;
                if !(3..=258).contains(&ln) || dd == 0 || dd > 32_768 {
                    return Err(format!("the recipe holds a match of length {} at distance {}", ln, dd));
                }
                let mut i = lsym_of(ln);
                if ri < d.resp.len() && d.resp[ri] as usize == li - 1 {
                    // THE SPELLING EXCEPTION, read back: symbol 284 and its 31
                    // extra bits. LBASE[27] = 227 and LEXTRA[27] = 5, so the
                    // two `w.put`s below emit 31 in 5 bits with no other change.
                    if ln != 258 {
                        return Err(format!("the recipe spells a {}-byte match with length symbol 284", ln));
                    }
                    i = 27;
                    ri += 1;
                }
                if 257 + i >= lc.len() {
                    // the bound, not the definition: a block may declare an
                    // HLIT that does not reach this symbol at all, and indexing
                    // first would panic where every other refusal here returns
                    // a reason. The distance side below has always had this
                    // guard; the length side did not, and M3a's spelling list
                    // is a second way to reach a high symbol.
                    return Err(format!(
                        "the recipe asks for length symbol {} beyond this block's table of {}",
                        257 + i,
                        lc.len()
                    ));
                }
                let (c, l) = lc[257 + i];
                if l == 0 {
                    return Err(format!("the recipe asks for length symbol {} which this block does not define", 257 + i));
                }
                w.put(c, l);
                w.put(ln - LBASE[i], LEXTRA[i]);
                let j = dsym_of(dd);
                if j >= dc.len() {
                    return Err("the recipe asks for a distance symbol beyond this block's table".into());
                }
                let (c2, l2) = dc[j];
                if l2 == 0 {
                    return Err(format!("the recipe asks for distance symbol {} which this block does not define", j));
                }
                w.put(c2, l2);
                w.put(dd - DBASE[j], DEXTRA[j]);
                pos += ln as usize;
            } else {
                if pos >= d.values.len() {
                    return Err("the recipe asks for a literal past the end of the inflated bytes".into());
                }
                let (c, l) = lc[d.values[pos] as usize];
                if l == 0 {
                    return Err(format!("the recipe asks for literal {} which this block does not define", d.values[pos]));
                }
                w.put(c, l);
                pos += 1;
            }
            tok += 1;
        }
        let (c, l) = lc[256];
        if l == 0 {
            return Err("a block whose code does not define end-of-block".into());
        }
        w.put(c, l);
        if final_bit == 1 {
            break;
        }
    }
    if w.n > 0 {
        let want = 8 - w.n;
        if d.pad_bits as u32 != want {
            return Err(format!("the recipe's padding is {} bits, the stream needs {}", d.pad_bits, want));
        }
        w.put(d.pad_val as u32, want);
    } else if d.pad_bits != 0 {
        return Err("the recipe carries padding for a stream that ends on a byte".into());
    }
    if pos != d.values.len() {
        return Err(format!("the re-spell consumed {} of {} inflated bytes", pos, d.values.len()));
    }
    if ri != d.resp.len() {
        // an unascending or out-of-range spelling list never matches its match,
        // so this is where it is convicted -- with a number, not a guess
        return Err(format!("the recipe's spelling list names {} matches, the re-spell reached {}", d.resp.len(), ri));
    }
    let body = w.buf;
    // the member region, then the container it came out of
    let mut member = Vec::with_capacity(d.head.len() + body.len() + d.tail.len());
    member.extend_from_slice(&d.head);
    member.extend_from_slice(&body);
    member.extend_from_slice(&d.tail);
    let mut out = d.pre.clone();
    if d.segs.is_empty() {
        out.extend_from_slice(&member);
    } else {
        let mut at = 0usize;
        for (plen, gap) in &d.segs {
            let e = at + *plen as usize;
            if e > member.len() {
                return Err("the recipe's chunk split runs past the member".into());
            }
            out.extend_from_slice(&member[at..e]);
            out.extend_from_slice(gap);
            at = e;
        }
        if at != member.len() {
            return Err(format!("the recipe's chunk split covers {} of {} member bytes", at, member.len()));
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------- sniffing

/// the byte length of a gzip header, or None
fn gzip_header_len(s: &[u8]) -> Option<usize> {
    if s.len() < 18 || s[0] != 0x1F || s[1] != 0x8B || s[2] != 0x08 {
        return None;
    }
    let flg = s[3];
    if flg & 0xE0 != 0 {
        return None; // reserved bits set
    }
    let mut p = 10usize;
    if flg & 4 != 0 {
        if p + 2 > s.len() {
            return None;
        }
        let xlen = s[p] as usize | ((s[p + 1] as usize) << 8);
        p = p.checked_add(2)?.checked_add(xlen)?;
    }
    if flg & 8 != 0 {
        if p >= s.len() {
            return None;
        }
        p += s[p..].iter().position(|&b| b == 0)? + 1;
    }
    if flg & 16 != 0 {
        if p >= s.len() {
            return None;
        }
        p += s[p..].iter().position(|&b| b == 0)? + 1;
    }
    if flg & 2 != 0 {
        p += 2;
    }
    if p + 8 > s.len() {
        return None;
    }
    Some(p)
}
fn is_zlib_header(s: &[u8]) -> bool {
    s.len() >= 8 && s[0] & 0x0F == 8 && (s[0] >> 4) <= 7 && (((s[0] as u32) << 8) | s[1] as u32).is_multiple_of(31) && s[1] & 0x20 == 0
}
fn is_png(src: &[u8]) -> bool {
    src.len() > 8 && src[..8] == [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
}
/// a bounded probe: does the first block header parse as deflate? Cheap enough
/// to run on any file and strong enough that random bytes almost never pass.
fn looks_like_deflate(body: &[u8]) -> bool {
    let mut br = Bits::new(body);
    let Ok(_final_bit) = br.get(1) else { return false };
    let Ok(btype) = br.get(2) else { return false };
    match btype {
        0 => {
            br.align();
            let (Ok(ln), Ok(nln)) = (br.get(16), br.get(16)) else { return false };
            nln == (!ln & 0xFFFF) && br.at() + ln as usize <= body.len()
        }
        1 => true,
        2 => {
            let (Ok(_h1), Ok(_h2), Ok(hclen)) = (br.get(5), br.get(5), br.get(4)) else { return false };
            let mut cl = [0u8; 19];
            for i in 0..hclen as usize + 4 {
                let Ok(v) = br.get(3) else { return false };
                cl[CLORDER[i]] = v as u8;
            }
            matches!(build(&cl), Ok(Some(_)))
        }
        _ => false,
    }
}

/// does this file's own shape nominate the deflate peel? Nomination is never a
/// decision -- the parse may still refuse and the trial may still prefer the
/// raw form.
pub fn nominates(src: &[u8]) -> bool {
    if src.len() < 32 {
        return false;
    }
    if let Some(h) = gzip_header_len(src) {
        return looks_like_deflate(&src[h..src.len() - 8]);
    }
    if is_png(src) {
        return true;
    }
    if is_zlib_header(src) {
        return looks_like_deflate(&src[2..src.len() - 4]);
    }
    // a bare deflate stream: the block-header probe is the whole filter, and a
    // false nomination costs only a refused parse
    looks_like_deflate(src)
}

/// (the bytes before the member, the member itself, the chunk split)
type PngParts = (Vec<u8>, Vec<u8>, Vec<(u32, Vec<u8>)>);
/// PNG: the bytes before the first IDAT payload, the concatenated IDAT
/// payloads, and the chunk split that puts them back
fn png_member(src: &[u8]) -> Result<PngParts, String> {
    let mut p = 8usize;
    let mut pre: Vec<u8> = Vec::new();
    let mut member: Vec<u8> = Vec::new();
    let mut segs: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut gap: Vec<u8> = Vec::new();
    let mut seen = false;
    while p + 8 <= src.len() {
        let len = u32::from_be_bytes([src[p], src[p + 1], src[p + 2], src[p + 3]]) as usize;
        let ty: [u8; 4] = [src[p + 4], src[p + 5], src[p + 6], src[p + 7]];
        let end = p.checked_add(12).and_then(|q| q.checked_add(len)).ok_or("a PNG chunk length overflows")?;
        if end > src.len() {
            return Err("a PNG chunk runs past the end of the file".into());
        }
        if &ty == b"IDAT" {
            if !seen {
                pre = src[..p + 8].to_vec();
                seen = true;
            } else {
                let last = segs.len() - 1;
                gap.extend_from_slice(&src[p..p + 8]);
                segs[last].1 = std::mem::take(&mut gap);
            }
            member.extend_from_slice(&src[p + 8..p + 8 + len]);
            segs.push((len as u32, Vec::new()));
            gap = src[p + 8 + len..p + 12 + len].to_vec(); // the chunk's own CRC
        } else if seen {
            gap.extend_from_slice(&src[p..end]);
        }
        p = end;
    }
    if p != src.len() {
        return Err("trailing bytes after the last PNG chunk".into());
    }
    if segs.is_empty() {
        return Err("a PNG with no IDAT chunk".into());
    }
    let last = segs.len() - 1;
    segs[last].1 = gap;
    Ok((pre, member, segs))
}

// ---------------------------------------------------------------- peel

/// peel `src`, or say why not
pub fn peel(src: &[u8]) -> Result<Deflate, String> {
    let mut d = Deflate {
        wrap: WRAP_RAW,
        pre: Vec::new(),
        head: Vec::new(),
        tail: Vec::new(),
        segs: Vec::new(),
        resp: Vec::new(),
        meta: Vec::new(),
        flags: Vec::new(),
        lens: Vec::new(),
        dists: Vec::new(),
        nblocks: 0,
        ntok: 0,
        pad_bits: 0,
        pad_val: 0,
        values: Vec::new(),
    };
    // the member is BORROWED wherever it already sits in `src` -- only a PNG,
    // whose member is scattered across IDAT chunks, needs a buffer of its own.
    // (The nomination probe is bounded but not certain, so a file that merely
    // LOOKS like a bare deflate stream must not cost a copy of itself.)
    let png: Vec<u8>;
    let member: &[u8];
    let body_range: (usize, usize);
    if let Some(h) = gzip_header_len(src) {
        d.wrap = WRAP_GZIP;
        body_range = (h, src.len() - 8);
        member = src;
    } else if is_png(src) {
        let (pre, mem, segs) = png_member(src)?;
        if !is_zlib_header(&mem) {
            return Err("the PNG IDAT payload is not a zlib stream".into());
        }
        d.wrap = WRAP_PNG;
        d.pre = pre;
        d.segs = segs;
        body_range = (2, mem.len() - 4);
        png = mem;
        member = &png;
    } else if is_zlib_header(src) {
        d.wrap = WRAP_ZLIB;
        body_range = (2, src.len() - 4);
        member = src;
    } else {
        d.wrap = WRAP_RAW;
        body_range = (0, src.len());
        member = src;
    }
    if body_range.0 >= body_range.1 {
        return Err("no deflate body between the wrapper head and tail".into());
    }
    d.head = member[..body_range.0].to_vec();
    d.tail = member[body_range.1..].to_vec();
    let body = &member[body_range.0..body_range.1];
    let used = walk(body, &mut d)?;
    if used != body.len() {
        return Err(format!("the deflate stream used {} of the member's {} body bytes", used, body.len()));
    }
    Ok(d)
}

// ---------------------------------------------------------------- the recipe blob

/// the recipe's serialised header size: version, wrap, the two padding bytes,
/// nblocks, ntok, nmatch, values_len, and the SEVEN section lengths (the
/// seventh, `nresp`, is M3a's sparse spelling list)
pub const HDR: usize = 52;

pub fn blob_len(d: &Deflate) -> usize {
    HDR + d.pre.len()
        + d.head.len()
        + d.tail.len()
        + d.segs.len() * 8
        + d.segs.iter().map(|(_, g)| g.len()).sum::<usize>()
        + d.meta.len()
        + d.flags.len()
        + d.lens.len()
        + d.dists.len() * 2
        + d.resp.len() * 4
}

/// the recipe, serialised. The dedicated model (drecipe.rs) parses these
/// sections and codes each in its own language; the ordinary ladder can also
/// carry the blob verbatim, which is what makes the peel testable on its own.
pub fn blob(d: &Deflate) -> Vec<u8> {
    let mut b = Vec::with_capacity(blob_len(d));
    b.push(1u8); // version
    b.push(d.wrap);
    b.push(d.pad_bits);
    b.push(d.pad_val);
    b.extend_from_slice(&d.nblocks.to_le_bytes());
    b.extend_from_slice(&d.ntok.to_le_bytes());
    b.extend_from_slice(&(d.lens.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.values.len() as u64).to_le_bytes());
    b.extend_from_slice(&(d.pre.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.head.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.tail.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.segs.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.meta.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.flags.len() as u32).to_le_bytes());
    b.extend_from_slice(&(d.resp.len() as u32).to_le_bytes());
    debug_assert_eq!(b.len(), HDR);
    b.extend_from_slice(&d.pre);
    b.extend_from_slice(&d.head);
    b.extend_from_slice(&d.tail);
    for (l, g) in &d.segs {
        b.extend_from_slice(&l.to_le_bytes());
        b.extend_from_slice(&(g.len() as u32).to_le_bytes());
    }
    for (_, g) in &d.segs {
        b.extend_from_slice(g);
    }
    b.extend_from_slice(&d.meta);
    b.extend_from_slice(&d.flags);
    b.extend_from_slice(&d.lens);
    for &x in &d.dists {
        b.extend_from_slice(&x.to_le_bytes());
    }
    for &x in &d.resp {
        b.extend_from_slice(&x.to_le_bytes());
    }
    b
}

/// where each section sits in a blob. The model and the parser share ONE
/// reading of the layout, so they cannot disagree about it (v12-M3's lesson).
pub struct Layout {
    pub wrap: u8,
    pub pad_bits: u8,
    pub pad_val: u8,
    pub nblocks: u32,
    pub ntok: u32,
    pub nmatch: usize,
    pub values_len: u64,
    pub pre: (usize, usize),
    pub head: (usize, usize),
    pub tail: (usize, usize),
    pub segtab: (usize, usize),
    pub gaps: (usize, usize),
    pub meta: (usize, usize),
    pub flags: (usize, usize),
    pub lens: (usize, usize),
    pub dists: (usize, usize),
    pub resp: (usize, usize),
}
pub fn layout(b: &[u8]) -> Result<Layout, String> {
    if b.len() < HDR {
        return Err("a deflate recipe shorter than its header".into());
    }
    if b[0] != 1 {
        return Err(format!("deflate recipe version {} is not 1", b[0]));
    }
    let u32at = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as usize;
    let nblocks = u32at(4) as u32;
    let ntok = u32at(8) as u32;
    let nmatch = u32at(12);
    let values_len = u64::from_le_bytes(b[16..24].try_into().unwrap());
    let (prel, headl, taill) = (u32at(24), u32at(28), u32at(32));
    let (nsegs, metal, flagsl) = (u32at(36), u32at(40), u32at(44));
    let nresp = u32at(48);
    // every section walked forward from HDR, each bounded against the blob's
    // own length: a recipe that does not add up REFUSES here, before a byte of
    // it is believed
    let mut p = HDR;
    let mut sect = |n: usize| -> Result<(usize, usize), String> {
        let e = p.checked_add(n).ok_or("a deflate recipe section length overflows")?;
        if e > b.len() {
            return Err(format!("a deflate recipe section of {} B runs past its {} B", n, b.len()));
        }
        let r = (p, e);
        p = e;
        Ok(r)
    };
    let pre = sect(prel)?;
    let head = sect(headl)?;
    let tail = sect(taill)?;
    let segtab = sect(nsegs.checked_mul(8).ok_or("a deflate recipe chunk table overflows")?)?;
    let mut gaplen = 0usize;
    for i in 0..nsegs {
        let o = segtab.0 + i * 8 + 4;
        gaplen = gaplen
            .checked_add(u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as usize)
            .ok_or("a deflate recipe's gap lengths overflow")?;
    }
    let gaps = sect(gaplen)?;
    let meta = sect(metal)?;
    let flags = sect(flagsl)?;
    let lens = sect(nmatch)?;
    let dists = sect(nmatch.checked_mul(2).ok_or("a deflate recipe distance table overflows")?)?;
    let resp = sect(nresp.checked_mul(4).ok_or("a deflate recipe spelling list overflows")?)?;
    if resp.1 != b.len() {
        return Err(format!("a deflate recipe of {} B whose sections account for {}", b.len(), resp.1));
    }
    Ok(Layout {
        wrap: b[1],
        pad_bits: b[2],
        pad_val: b[3],
        nblocks,
        ntok,
        nmatch,
        values_len,
        pre,
        head,
        tail,
        segtab,
        gaps,
        meta,
        flags,
        lens,
        dists,
        resp,
    })
}

/// the blob back into a recipe (the values are restored separately and set by
/// the caller)
pub fn from_blob(b: &[u8]) -> Result<Deflate, String> {
    let l = layout(b)?;
    let nsegs = (l.segtab.1 - l.segtab.0) / 8;
    if l.lens.1 - l.lens.0 != l.nmatch {
        return Err("a deflate recipe whose length table does not match its match count".into());
    }
    let mut segs = Vec::with_capacity(nsegs);
    let mut go = l.gaps.0;
    for i in 0..nsegs {
        let o = l.segtab.0 + i * 8;
        let plen = u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        let glen = u32::from_le_bytes([b[o + 4], b[o + 5], b[o + 6], b[o + 7]]) as usize;
        segs.push((plen, b[go..go + glen].to_vec()));
        go += glen;
    }
    let lens: Vec<u8> = b[l.lens.0..l.lens.1].to_vec();
    let dists: Vec<u16> = b[l.dists.0..l.dists.1].as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
    let resp: Vec<u32> = b[l.resp.0..l.resp.1].as_chunks::<4>().0.iter().map(|c| u32::from_le_bytes(*c)).collect();
    // the spelling list is a strictly ascending list of match indices, and it
    // is checked HERE rather than trusted: a hostile recipe cannot make the
    // re-speller walk off its own tables
    for (k, w) in resp.windows(2).enumerate() {
        if w[0] >= w[1] {
            return Err(format!("a deflate recipe whose spelling list is not ascending at {}: {} then {}", k, w[0], w[1]));
        }
    }
    if let Some(&last) = resp.last() {
        if last as usize >= l.nmatch {
            return Err(format!("a deflate recipe whose spelling list names match {} of {}", last, l.nmatch));
        }
    }
    Ok(Deflate {
        wrap: l.wrap,
        pre: b[l.pre.0..l.pre.1].to_vec(),
        head: b[l.head.0..l.head.1].to_vec(),
        tail: b[l.tail.0..l.tail.1].to_vec(),
        segs,
        meta: b[l.meta.0..l.meta.1].to_vec(),
        flags: b[l.flags.0..l.flags.1].to_vec(),
        lens,
        dists,
        resp,
        nblocks: l.nblocks,
        ntok: l.ntok,
        pad_bits: l.pad_bits,
        pad_val: l.pad_val,
        values: Vec::new(),
    })
}

/// M3a's FIXTURE, built here so no test needs one on disk: a gzip member with a
/// single fixed-Huffman block whose first and third 258-byte matches use the
/// SECOND legal spelling -- symbol 284 with its 31 extra bits -- and whose
/// second uses the ordinary 285. No encoder in the wild emits this, which is
/// exactly why the corpus cannot test the path; before M3a the whole file
/// refused. `deflate.rs` and `main.rs` both build their gate from it.
#[cfg(test)]
pub fn mk_gzip_284() -> Vec<u8> {
    let lit = codes(&fixed_lit());
    let dst = codes(&[5u8; 30]);
    let mut w = BitW::new(256);
    let mut raw: Vec<u8> = Vec::new();
    w.put(1, 1); // BFINAL
    w.put(1, 2); // BTYPE = 1, the fixed code
    for (b, spell284) in [(b'A', true), (b'B', false), (b'C', true)] {
        let (c, l) = lit[b as usize];
        w.put(c, l);
        raw.push(b);
        // 258 more of the same byte, at distance 1
        let i = if spell284 { 27usize } else { 28usize };
        let (c, l) = lit[257 + i];
        w.put(c, l);
        w.put(258 - LBASE[i], LEXTRA[i]);
        let (c, l) = dst[0]; // distance symbol 0 = distance 1, no extra bits
        w.put(c, l);
        raw.extend(std::iter::repeat_n(b, 258));
    }
    // a tail of plain literals so the member clears `nominates`'s 32-byte
    // floor and looks like an ordinary file rather than a three-token toy
    for b in 0u8..96 {
        let (c, l) = lit[b as usize];
        w.put(c, l);
        raw.push(b);
    }
    let (c, l) = lit[256];
    w.put(c, l);
    if w.n > 0 {
        let k = 8 - w.n;
        w.put(0, k);
    }
    let mut gz = vec![0x1F, 0x8B, 0x08, 0x00, 0, 0, 0, 0, 0x00, 0xFF];
    gz.extend_from_slice(&w.buf);
    let mut crc = 0xFFFF_FFFFu32;
    for &b in &raw {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    gz.extend_from_slice(&(!crc).to_le_bytes());
    gz.extend_from_slice(&(raw.len() as u32).to_le_bytes());
    gz
}

#[cfg(test)]
mod tests {
    use super::*;

    /// M3a, THE GATE: the second legal spelling of a 258-byte match is
    /// RECORDED, not refused, and it comes back bit for bit -- through the
    /// parse, through the serialised recipe, and with every hostile spelling
    /// list refusing with a reason.
    #[test]
    fn the_second_spelling_of_258_round_trips() {
        let gz = mk_gzip_284();
        let d = peel(&gz).expect("a 284-spelled member peels at M3a");
        assert_eq!(d.lens.len(), 3, "three matches");
        assert_eq!(d.resp, vec![0u32, 2], "the first and third took the 284 spelling");
        assert_eq!(d.values.len(), 3 * 259 + 96, "three runs of 1 + 258, then the literal tail");
        assert!(respell(&d).expect("re-spell") == gz, "the 284 spelling did not come back");
        // the serialised recipe carries it
        let b = blob(&d);
        assert_eq!(b.len(), blob_len(&d));
        let l = layout(&b).expect("layout");
        assert_eq!(l.resp.1 - l.resp.0, 8, "two u32 of spelling list");
        let mut back = from_blob(&b).expect("from_blob");
        assert_eq!(back.resp, d.resp);
        back.values = d.values.clone();
        assert!(respell(&back).expect("respell") == gz);
        // hostile spelling lists: unascending, past the match count, and one
        // that names a match of another length. Each refuses with a reason.
        let mut unasc = b.clone();
        unasc[l.resp.0..l.resp.0 + 4].copy_from_slice(&9u32.to_le_bytes());
        assert!(from_blob(&unasc).is_err(), "an unascending spelling list must refuse");
        let mut far = b.clone();
        far[l.resp.0 + 4..l.resp.0 + 8].copy_from_slice(&99u32.to_le_bytes());
        assert!(from_blob(&far).is_err(), "a spelling list past the match count must refuse");
        let mut wrong = from_blob(&b).expect("from_blob");
        wrong.values = d.values.clone();
        wrong.lens[0] = 0; // now a 3-byte match, still named by the spelling list
        assert!(respell(&wrong).is_err(), "284 on a match that is not 258 must refuse");
    }

    /// and the ordinary corpus is untouched by the fifth stream: nothing here
    /// spells 258 the second way, so the list is empty and costs 4 header bytes
    #[test]
    fn the_spelling_list_is_empty_on_an_ordinary_member() {
        let src = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wubbadub.html")).expect("present");
        let gz = mk_gzip(&src);
        let d = peel(&gz).expect("peels");
        assert!(d.resp.is_empty(), "an ordinary member names no re-spelling");
        assert_eq!(blob_len(&d), HDR + d.pre.len() + d.head.len() + d.tail.len() + d.meta.len() + d.flags.len() + d.lens.len() + d.dists.len() * 2);
    }

    /// THE GATE, in Rust: the save's own skin re-spells BIT FOR BIT, as it did
    /// in python (1,171 blocks, 38,340,574 tokens, 66,417,533 B).
    #[test]
    fn the_save_skin_respells_bit_for_bit() {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-big/aoe4-autosave.sav");
        let src = std::fs::read(p).expect("corpus-big/aoe4-autosave.sav present");
        let d = peel(&src).expect("the save peels");
        assert_eq!(d.nblocks, 1_171, "block count");
        assert_eq!(d.ntok, 38_340_574, "token count");
        assert_eq!(d.values.len(), 296_540_843, "inflated length");
        assert_eq!(d.lens.len(), 25_661_244, "match count");
        let back = respell(&d).expect("the save re-spells");
        assert!(back == src, "the re-spell is not the original skin");
    }

    /// the blob is the recipe and nothing else: through the serialisation and
    /// back, the same bytes come out
    #[test]
    fn the_blob_round_trips_the_recipe() {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wubbadub.html");
        let src = std::fs::read(p).expect("corpus-real/wubbadub.html present");
        // a gzip member built by this test, so the test needs no fixture
        let gz = mk_gzip(&src);
        let d = peel(&gz).expect("peels");
        let b = blob(&d);
        assert_eq!(b.len(), blob_len(&d));
        let mut back = from_blob(&b).expect("from_blob");
        back.values = d.values.clone();
        assert!(respell(&back).expect("respell") == gz);
    }

    /// hostiles: every one refuses with a reason, none panics
    #[test]
    fn hostile_streams_refuse_with_a_reason() {
        let src = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wubbadub.html")).expect("present");
        let gz = mk_gzip(&src);
        // truncated member
        assert!(peel(&gz[..gz.len() / 2]).is_err(), "a truncated member must refuse");
        // a corrupt code-length table
        let mut bad = gz.clone();
        let n = bad.len();
        bad[12] ^= 0xFF;
        bad[13] ^= 0xFF;
        let _ = peel(&bad); // must not panic; it may refuse or produce a parse
        assert_eq!(n, bad.len());
        // an empty file and a header with no body
        assert!(peel(&[]).is_err());
        assert!(peel(&gz[..10]).is_err());
        // a blob that does not add up
        assert!(from_blob(&[1u8; HDR]).is_err());
        assert!(layout(&[0u8; 4]).is_err());
    }

    /// a minimal gzip member built with STORED blocks only, so the test brings
    /// its own fixture and depends on no encoder
    fn mk_gzip(src: &[u8]) -> Vec<u8> {
        let mut out = vec![0x1F, 0x8B, 0x08, 0x00, 0, 0, 0, 0, 0x00, 0xFF];
        let mut i = 0usize;
        while i < src.len() {
            let n = (src.len() - i).min(65_535);
            let last = if i + n >= src.len() { 1u8 } else { 0u8 };
            out.push(last);
            out.extend_from_slice(&(n as u16).to_le_bytes());
            out.extend_from_slice(&(!(n as u16)).to_le_bytes());
            out.extend_from_slice(&src[i..i + n]);
            i += n;
        }
        let mut crc = 0xFFFF_FFFFu32;
        for &b in src {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xEDB8_8320 & (0u32.wrapping_sub(crc & 1)));
            }
        }
        out.extend_from_slice(&(!crc).to_le_bytes());
        out.extend_from_slice(&(src.len() as u32).to_le_bytes());
        out
    }
}
