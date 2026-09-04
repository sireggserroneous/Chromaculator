//! filter.rs -- the overlay reading, made executable.
//!
//! wubbadub.html:1304: "Lay the two stalks over each other at matching place
//! values... nothing has to line up with anything but its own weight."
//! spectrometer.html:238: "the same value, respelled." The site's layout is
//! order-preserving (spec.md:30), so near values have near spellings and a
//! difference is small. A signal respelled as its differences AT ITS OWN
//! STRIDE is the site's subtraction-on-the-grid -- that is all a filter is.
//!
//! Honest note, kept from the plan: the site's audio page is spectral
//! (rate->pitch, additive sines, no samples); it does NOT ground PCM
//! transforms -- the overlay/subtraction geometry does. Prior art attributed:
//! PNG filters, shorten/FLAC sample deltas.
//!
//! Every filter is length-preserving, exactly invertible, and defined for
//! ALL lengths including zero (the tails -- len < stride, W16 remainders --
//! are the 100%-fail class; the property test walks every length 0..1024).
//!
//! ids: 0 none | 1..4 byte delta at k=id | 5 byte delta at param |
//!      6 W16(ch): true 16-bit LE per-channel sample delta | 7 reserved
//!      (AVG2D, gated M4).

use crate::{dyadic, token};

pub const FILTER_NONE: u8 = 0;
pub const FILTER_STRIDE: u8 = 5;
pub const FILTER_W16: u8 = 6;

// ---------------- apply / undo ----------------
fn delta_apply(src: &[u8], k: usize) -> Vec<u8> {
    if k == 0 || k > src.len() {
        return src.to_vec();
    }
    let mut out = Vec::with_capacity(src.len());
    out.extend_from_slice(&src[..k]);
    for i in k..src.len() {
        out.push(src[i].wrapping_sub(src[i - k]));
    }
    out
}
fn delta_undo(buf: &[u8], k: usize) -> Vec<u8> {
    if k == 0 || k > buf.len() {
        return buf.to_vec();
    }
    let mut out = Vec::with_capacity(buf.len());
    out.extend_from_slice(&buf[..k]);
    for i in k..buf.len() {
        let b = buf[i].wrapping_add(out[i - k]);
        out.push(b);
    }
    out
}
/// true 16-bit little-endian per-channel sample delta: each sample minus the
/// previous sample OF ITS CHANNEL, wrapping i16 arithmetic, over the
/// floor(len/(2 ch)) * (2 ch) prefix; the first frame and the byte remainder
/// stay verbatim. This is the real thing the borrow-bit math asked for:
/// byte-delta-at-4 loses the carry between lo and hi bytes.
fn w16_apply(src: &[u8], ch: usize) -> Vec<u8> {
    let frame = 2 * ch;
    if ch == 0 || src.len() < 2 * frame {
        return src.to_vec();
    }
    let body = (src.len() / frame) * frame;
    let mut out = Vec::with_capacity(src.len());
    out.extend_from_slice(&src[..frame]);
    for o in (frame..body).step_by(2) {
        let s = i16::from_le_bytes([src[o], src[o + 1]]);
        let p = i16::from_le_bytes([src[o - frame], src[o - frame + 1]]);
        out.extend_from_slice(&s.wrapping_sub(p).to_le_bytes());
    }
    out.extend_from_slice(&src[body..]);
    out
}
fn w16_undo(buf: &[u8], ch: usize) -> Vec<u8> {
    let frame = 2 * ch;
    if ch == 0 || buf.len() < 2 * frame {
        return buf.to_vec();
    }
    let body = (buf.len() / frame) * frame;
    let mut out = Vec::with_capacity(buf.len());
    out.extend_from_slice(&buf[..frame]);
    for o in (frame..body).step_by(2) {
        let d = i16::from_le_bytes([buf[o], buf[o + 1]]);
        let p = i16::from_le_bytes([out[o - frame], out[o - frame + 1]]);
        out.extend_from_slice(&d.wrapping_add(p).to_le_bytes());
    }
    out.extend_from_slice(&buf[body..]);
    out
}

pub fn apply(src: &[u8], id: u8, param: u32) -> Vec<u8> {
    match id {
        0 => src.to_vec(),
        1..=4 => delta_apply(src, id as usize),
        FILTER_STRIDE => delta_apply(src, param as usize),
        FILTER_W16 => w16_apply(src, param as usize),
        _ => src.to_vec(), // unknown ids pass through; the gate convicts misuse
    }
}
pub fn undo(buf: &[u8], id: u8, param: u32) -> Vec<u8> {
    match id {
        0 => buf.to_vec(),
        1..=4 => delta_undo(buf, id as usize),
        FILTER_STRIDE => delta_undo(buf, param as usize),
        FILTER_W16 => w16_undo(buf, param as usize),
        _ => buf.to_vec(),
    }
}

// ---------------- the decision, steps 1-4 (step 5 = full trial in main) ----------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cand {
    pub id: u8,
    pub param: u32,
}

/// three slices, up to 64 KB total, offsets aligned to 8 so W16 frames and
/// small strides stay phase-true across slice joins
fn sample_of(src: &[u8]) -> Vec<u8> {
    const TOTAL: usize = 64 * 1024;
    if src.len() <= TOTAL {
        return src.to_vec();
    }
    let sl = TOTAL / 3;
    let mid = (src.len() / 2 - sl / 2) & !7;
    let end = (src.len() - sl) & !7;
    let mut out = Vec::with_capacity(3 * sl);
    out.extend_from_slice(&src[..sl]);
    out.extend_from_slice(&src[mid..mid + sl]);
    out.extend_from_slice(&src[end..end + sl]);
    out
}

/// header sniffing NOMINATES, never decides (step 1)
fn sniff(src: &[u8]) -> Vec<Cand> {
    let mut out = Vec::new();
    if src.len() >= 54 && &src[0..2] == b"BM" {
        let bpp = u16::from_le_bytes([src[28], src[29]]) as usize;
        let compression = u32::from_le_bytes(src[30..34].try_into().unwrap());
        let width = i32::from_le_bytes(src[18..22].try_into().unwrap());
        if compression == 0 && bpp >= 8 && bpp % 8 == 0 && width > 0 {
            let pix = bpp / 8;
            let row = ((width as usize * bpp + 31) / 32) * 4;
            if pix <= 4 {
                out.push(Cand { id: pix as u8, param: 0 });
            } else {
                out.push(Cand { id: FILTER_STRIDE, param: pix as u32 });
            }
            out.push(Cand { id: FILTER_STRIDE, param: row as u32 });
        }
    }
    if src.len() >= 44 && &src[0..4] == b"RIFF" && &src[8..12] == b"WAVE" {
        // walk chunks to the fmt block (usually at 12)
        let mut o = 12usize;
        while o + 8 <= src.len() {
            let id = &src[o..o + 4];
            let sz = u32::from_le_bytes(src[o + 4..o + 8].try_into().unwrap()) as usize;
            if id == b"fmt " && o + 8 + 16 <= src.len() {
                let channels = u16::from_le_bytes([src[o + 10], src[o + 11]]) as usize;
                let block_align = u16::from_le_bytes([src[o + 20], src[o + 21]]) as usize;
                let bits = u16::from_le_bytes([src[o + 22], src[o + 23]]) as usize;
                if bits == 16 && channels >= 1 && channels <= 8 {
                    out.push(Cand { id: FILTER_W16, param: channels as u32 });
                } else if block_align >= 1 && block_align <= 4096 {
                    if block_align <= 4 {
                        out.push(Cand { id: block_align as u8, param: 0 });
                    } else {
                        out.push(Cand { id: FILTER_STRIDE, param: block_align as u32 });
                    }
                }
                break;
            }
            o += 8 + sz + (sz & 1);
        }
    }
    out
}

/// autocorrelation nominates a period (step 2): the lag whose folded byte
/// distance min(d, 256-d) is smallest over the sample
fn autocorr(sample: &[u8]) -> Option<Cand> {
    if sample.len() < 512 {
        return None;
    }
    let max_lag = 4096.min(sample.len() / 4);
    let mut best = (0usize, f64::MAX);
    for lag in 1..=max_lag {
        let step = 1 + lag / 64; // budget: long lags sample sparser
        let mut acc = 0u64;
        let mut n = 0u32;
        let mut i = lag;
        while i < sample.len() {
            let d = sample[i].wrapping_sub(sample[i - lag]) as u32;
            acc += d.min(256 - d) as u64;
            n += 1;
            i += step;
        }
        let mean = acc as f64 / n.max(1) as f64;
        if mean < best.1 {
            best = (lag, mean);
        }
    }
    // random bytes fold to a mean of 64; only a clear period is worth a trial
    if best.1 < 44.0 {
        let lag = best.0;
        if lag <= 4 {
            return Some(Cand { id: lag as u8, param: 0 });
        }
        return Some(Cand { id: FILTER_STRIDE, param: lag as u32 });
    }
    None
}

/// nominate (sniff + autocorrelation), prune by the dyadic stage's own
/// literal price on the filtered sample. Returns EVERY candidate whose
/// sample gain clears 0.5 bit/byte, in fixed order (sniff first, then
/// autocorrelation) -- the FULL trial in main decides among them.
///
/// M2's probe convicted anything stronger than pruning: the 3-slice sample
/// ranked byte-delta-2 over W16 on alarm01 (the full file says W16 by 7
/// points) and vetoed stride-6000 on the BMP (the full file says 31% -> 2.4%
/// -- each slice starts with a full verbatim stride, so long strides are
/// systematically understated). Samples may NOMINATE and PRUNE; only whole
/// files decide.
pub fn nominate(src: &[u8]) -> Vec<Cand> {
    let sample = sample_of(src);
    let mut cands = sniff(src);
    if let Some(c) = autocorr(&sample) {
        if !cands.contains(&c) {
            cands.push(c);
        }
    }
    let base = token::lit_price8(&sample);
    cands.retain(|c| {
        let f = apply(&sample, c.id, c.param);
        base - token::lit_price8(&f) >= 4 // 0.5 bit/byte, x8 scale
    });
    cands
}

/// the decision trace, printed -- instrument before tuning (the --stats
/// lesson of v7: exact prices found every gap)
pub fn probe(src: &[u8]) {
    let sample = sample_of(src);
    let sniffed = sniff(src);
    println!("  probe: {} sniffed candidate(s): {:?}", sniffed.len(), sniffed);
    if sample.len() >= 512 {
        let max_lag = 4096.min(sample.len() / 4);
        let mut best = (0usize, f64::MAX);
        for lag in 1..=max_lag {
            let step = 1 + lag / 64;
            let mut acc = 0u64;
            let mut n = 0u32;
            let mut i = lag;
            while i < sample.len() {
                let d = sample[i].wrapping_sub(sample[i - lag]) as u32;
                acc += d.min(256 - d) as u64;
                n += 1;
                i += step;
            }
            let mean = acc as f64 / n.max(1) as f64;
            if mean < best.1 {
                best = (lag, mean);
            }
        }
        println!("  probe: autocorr best lag {} mean folded {:.1} (threshold 44, random ~64)", best.0, best.1);
    }
    let base = token::lit_price8(&sample);
    println!("  probe: base sample price8 = {} ({:.2} bits/byte)", base, base as f64 / 8.0);
    let mut cands = sniffed.clone();
    if let Some(c) = autocorr(&sample) {
        if !cands.contains(&c) {
            cands.push(c);
        }
    }
    for c in &cands {
        let f = apply(&sample, c.id, c.param);
        let p = token::lit_price8(&f);
        println!(
            "  probe: candidate id {} param {} -> price8 {} ({:.2} bits/byte, gain {:.2}, gate needs >=0.50)",
            c.id, c.param, p, p as f64 / 8.0, (base - p) as f64 / 8.0
        );
    }
    let toks = token::tokenize(&sample);
    let mut match_bytes = 0usize;
    for t in &toks {
        if let token::Tok::Match { len, .. } = t {
            match_bytes += *len as usize;
        }
    }
    println!(
        "  probe: sample match cover {:.1}% (informational; the guard is the sample encode)",
        match_bytes as f64 * 100.0 / sample.len().max(1) as f64
    );
    let plain = dyadic::encode(&sample, &toks, 8).len();
    for c in &cands {
        let f = apply(&sample, c.id, c.param);
        let filt = dyadic::encode(&f, &token::tokenize(&f), 8).len();
        println!(
            "  probe: sample encode id {} param {}: {} B vs plain {} B ({})",
            c.id, c.param, filt, plain,
            if (filt as u64) * 1000 <= (plain as u64) * 995 { "confirmed" } else { "guard says no" }
        );
    }
    println!("  probe: nominate() -> {:?} (all go to full trial)", nominate(src));
}

// ---------------- the property test: every id, every length ----------------
#[cfg(test)]
mod tests {
    use super::*;

    fn xs(state: &mut u64) -> u8 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        (*state & 0xff) as u8
    }

    #[test]
    fn apply_undo_every_length() {
        // ids x params x lengths 0..=1024, random and structured content:
        // the tails (len 0, len < stride, W16 remainder) are the fail class
        let mut st = 0x1489u64;
        let cases: Vec<(u8, u32)> = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0), // param 0 must be a safe no-op
            (5, 1),
            (5, 3),
            (5, 7),
            (5, 384),
            (5, 600),
            (5, 6000),
            (6, 0), // ch 0 must be a safe no-op
            (6, 1),
            (6, 2),
            (6, 3),
        ];
        for len in 0..=1024usize {
            let mut rnd = vec![0u8; len];
            for b in rnd.iter_mut() {
                *b = xs(&mut st);
            }
            let ramp: Vec<u8> = (0..len).map(|i| (i * 7 + i / 5) as u8).collect();
            for data in [&rnd, &ramp] {
                for &(id, param) in &cases {
                    let f = apply(data, id, param);
                    assert_eq!(f.len(), data.len(), "length must be preserved id={} p={} len={}", id, param, len);
                    let u = undo(&f, id, param);
                    assert_eq!(&u, data, "undo(apply) != id at id={} p={} len={}", id, param, len);
                }
            }
        }
    }

    #[test]
    fn w16_is_not_byte_delta_4() {
        // the borrow-bit point: a rising 16-bit ramp respells to constant
        // small deltas under W16, but byte-delta-at-4 leaves hi-byte noise
        let mut src = Vec::new();
        for i in 0..2000i16 {
            src.extend_from_slice(&(i.wrapping_mul(191)).to_le_bytes());
            src.extend_from_slice(&(i.wrapping_mul(-173)).to_le_bytes());
        }
        let w = w16_apply(&src, 2);
        let d4 = delta_apply(&src, 4);
        let ones = |v: &[u8]| v.iter().filter(|&&b| b != 0).count();
        assert!(ones(&w) < ones(&d4), "W16 must beat byte-delta-4 on 16-bit ramps");
    }
}
