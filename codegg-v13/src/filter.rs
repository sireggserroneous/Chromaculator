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

use crate::{dyadic, structure, token};

#[allow(dead_code)] // id-space legend: 0 = no filter, named for the reader
pub
const FILTER_NONE: u8 = 0;
pub const FILTER_STRIDE: u8 = 5;
pub const FILTER_W16: u8 = 6;
// id 7 stays reserved (AVG2D, never shipped)
pub const FILTER_W16O2: u8 = 8; // v9: order-2 per-channel sample predictor
pub const FILTER_W16BE: u8 = 9; // v9: W16, big-endian (the MSB reading)
pub const FILTER_BCJ: u8 = 10; // v9-M5: x86 rel32 -> absolute (LZMA Bra86, ported)
pub const FILTER_TTF: u8 = 11; // v9-M5: TTF loca/metrics segmented respelling
pub const FILTER_RLE8: u8 = 12; // v11-M6: BMP RLE8 unrolled to pixels (canon-verified)
pub const FILTER_MS1: u8 = 13; // v12-M3: stereo mid/side (lifting), then the order-1 per-channel delta
pub const FILTER_MS2: u8 = 14; // v12-M3: stereo mid/side (lifting), then the order-2 per-channel predictor
/// the highest filter id a header may carry; armor::parse_header refuses
/// anything above it. v12-M3 lesson: ids 13/14 were added here and the
/// header check still said 12 -- alarm01.wav shipped as an artifact no site
/// could validate ("no valid header at any site"; caught by the ledger, not
/// by a test). ONE constant now, read by both.
pub const FILTER_MAX: u8 = FILTER_MS2;

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

/// v9 order-2 per-channel predictor: d = s - (2*s[-f] - s[-2f]), wrapping
/// i16; the first TWO frames and the byte remainder stay verbatim; anything
/// shorter than three frames passes through untouched. HONEST GROUNDING
/// NOTE: the site never names second differences -- the nearest reading is
/// spec.md:190-193 (the two ring systems distinguished by how the step
/// itself changes). Attribution, plainly: FLAC fixed predictor 2 / shorten.
fn w16o2_apply(src: &[u8], ch: usize) -> Vec<u8> {
    let frame = 2 * ch;
    if ch == 0 || src.len() < 3 * frame {
        return src.to_vec();
    }
    let body = (src.len() / frame) * frame;
    let mut out = Vec::with_capacity(src.len());
    out.extend_from_slice(&src[..2 * frame]);
    for o in (2 * frame..body).step_by(2) {
        let s = i16::from_le_bytes([src[o], src[o + 1]]);
        let s1 = i16::from_le_bytes([src[o - frame], src[o - frame + 1]]);
        let s2 = i16::from_le_bytes([src[o - 2 * frame], src[o - 2 * frame + 1]]);
        let pred = s1.wrapping_mul(2).wrapping_sub(s2);
        out.extend_from_slice(&s.wrapping_sub(pred).to_le_bytes());
    }
    out.extend_from_slice(&src[body..]);
    out
}
fn w16o2_undo(buf: &[u8], ch: usize) -> Vec<u8> {
    let frame = 2 * ch;
    if ch == 0 || buf.len() < 3 * frame {
        return buf.to_vec();
    }
    let body = (buf.len() / frame) * frame;
    let mut out = Vec::with_capacity(buf.len());
    out.extend_from_slice(&buf[..2 * frame]);
    for o in (2 * frame..body).step_by(2) {
        let d = i16::from_le_bytes([buf[o], buf[o + 1]]);
        let s1 = i16::from_le_bytes([out[o - frame], out[o - frame + 1]]);
        let s2 = i16::from_le_bytes([out[o - 2 * frame], out[o - 2 * frame + 1]]);
        let pred = s1.wrapping_mul(2).wrapping_sub(s2);
        out.extend_from_slice(&d.wrapping_add(pred).to_le_bytes());
    }
    out.extend_from_slice(&buf[body..]);
    out
}
/// v12-M3, the two-silhouettes reading (spectrometer.html:602: the stereo
/// pair are two shadows of one helix). Mid/side by LIFTING, exactly
/// invertible in wrapping i16 with no 17th bit: side = L - R, mid = R +
/// (side >> 1) (arithmetic shift), so R = mid - (side >> 1) and L = R + side.
/// Stereo 16-bit LE frames [L R] become [mid side]; the byte remainder past
/// the last whole frame stays verbatim. Attribution: FLAC's mid/side
/// decorrelation (Coalson), Shorten (Robinson), the S-transform lifting.
fn ms_lift(src: &[u8]) -> Vec<u8> {
    let body = (src.len() / 4) * 4;
    let mut out = Vec::with_capacity(src.len());
    for o in (0..body).step_by(4) {
        let l = i16::from_le_bytes([src[o], src[o + 1]]);
        let r = i16::from_le_bytes([src[o + 2], src[o + 3]]);
        let side = l.wrapping_sub(r);
        let mid = r.wrapping_add(side >> 1);
        out.extend_from_slice(&mid.to_le_bytes());
        out.extend_from_slice(&side.to_le_bytes());
    }
    out.extend_from_slice(&src[body..]);
    out
}
fn ms_unlift(buf: &[u8]) -> Vec<u8> {
    let body = (buf.len() / 4) * 4;
    let mut out = Vec::with_capacity(buf.len());
    for o in (0..body).step_by(4) {
        let mid = i16::from_le_bytes([buf[o], buf[o + 1]]);
        let side = i16::from_le_bytes([buf[o + 2], buf[o + 3]]);
        let r = mid.wrapping_sub(side >> 1);
        let l = r.wrapping_add(side);
        out.extend_from_slice(&l.to_le_bytes());
        out.extend_from_slice(&r.to_le_bytes());
    }
    out.extend_from_slice(&buf[body..]);
    out
}
/// mid/side, then the per-channel deltas the wavs already ride (order 1 = W16,
/// order 2 = W16O2), each over the two lifted channels
fn ms1_apply(src: &[u8]) -> Vec<u8> {
    w16_apply(&ms_lift(src), 2)
}
fn ms1_undo(buf: &[u8]) -> Vec<u8> {
    ms_unlift(&w16_undo(buf, 2))
}
fn ms2_apply(src: &[u8]) -> Vec<u8> {
    w16o2_apply(&ms_lift(src), 2)
}
fn ms2_undo(buf: &[u8]) -> Vec<u8> {
    ms_unlift(&w16o2_undo(buf, 2))
}

/// v9 W16 big-endian: the MSB reading (index.html:149 -- "most significant
/// first, no reordering"); TTF tables and other BE arrays respell small.
fn w16be_apply(src: &[u8], ch: usize) -> Vec<u8> {
    let frame = 2 * ch;
    if ch == 0 || src.len() < 2 * frame {
        return src.to_vec();
    }
    let body = (src.len() / frame) * frame;
    let mut out = Vec::with_capacity(src.len());
    out.extend_from_slice(&src[..frame]);
    for o in (frame..body).step_by(2) {
        let s = i16::from_be_bytes([src[o], src[o + 1]]);
        let p = i16::from_be_bytes([src[o - frame], src[o - frame + 1]]);
        out.extend_from_slice(&s.wrapping_sub(p).to_be_bytes());
    }
    out.extend_from_slice(&src[body..]);
    out
}
fn w16be_undo(buf: &[u8], ch: usize) -> Vec<u8> {
    let frame = 2 * ch;
    if ch == 0 || buf.len() < 2 * frame {
        return buf.to_vec();
    }
    let body = (buf.len() / frame) * frame;
    let mut out = Vec::with_capacity(buf.len());
    out.extend_from_slice(&buf[..frame]);
    for o in (frame..body).step_by(2) {
        let d = i16::from_be_bytes([buf[o], buf[o + 1]]);
        let p = i16::from_be_bytes([out[o - frame], out[o - frame + 1]]);
        out.extend_from_slice(&d.wrapping_add(p).to_be_bytes());
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
        FILTER_W16O2 => w16o2_apply(src, param as usize),
        FILTER_W16BE => w16be_apply(src, param as usize),
        FILTER_BCJ => structure::bcj_apply(src),
        FILTER_TTF => structure::ttf_apply(src),
        FILTER_RLE8 => structure::rle8_apply(src),
        FILTER_MS1 => ms1_apply(src),
        FILTER_MS2 => ms2_apply(src),
        _ => src.to_vec(), // unknown ids pass through; the gate convicts misuse
    }
}
pub fn undo(buf: &[u8], id: u8, param: u32) -> Vec<u8> {
    match id {
        0 => buf.to_vec(),
        1..=4 => delta_undo(buf, id as usize),
        FILTER_STRIDE => delta_undo(buf, param as usize),
        FILTER_W16 => w16_undo(buf, param as usize),
        FILTER_W16O2 => w16o2_undo(buf, param as usize),
        FILTER_W16BE => w16be_undo(buf, param as usize),
        FILTER_BCJ => structure::bcj_undo(buf),
        FILTER_TTF => structure::ttf_undo(buf),
        FILTER_RLE8 => structure::rle8_undo(buf),
        FILTER_MS1 => ms1_undo(buf),
        FILTER_MS2 => ms2_undo(buf),
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
        if compression == 0 && bpp >= 8 && bpp.is_multiple_of(8) && width > 0 {
            let pix = bpp / 8;
            let row = (width as usize * bpp).div_ceil(32) * 4;
            if pix <= 4 {
                out.push(Cand { id: pix as u8, param: 0 });
            } else {
                out.push(Cand { id: FILTER_STRIDE, param: pix as u32 });
            }
            out.push(Cand { id: FILTER_STRIDE, param: row as u32 });
        }
    }
    if src.len() >= 12
        && (src[0..4] == [0x00, 0x01, 0x00, 0x00] || &src[0..4] == b"OTTO" || &src[0..4] == b"true")
    {
        // a font's tables are big-endian arrays: the MSB reading; and the
        // table directory names the grids for the segmented respelling
        out.push(Cand { id: FILTER_TTF, param: 0 });
        out.push(Cand { id: FILTER_W16BE, param: 1 });
    }
    if src.len() >= 2 && &src[0..2] == b"MZ" {
        // a PE: call/jmp targets respell as absolute addresses (structure tier)
        out.push(Cand { id: FILTER_BCJ, param: 0 });
    }
    if src.len() >= 1100 && src.starts_with(b"BM") && structure::rle8_sniff(src) {
        out.push(Cand { id: FILTER_RLE8, param: 0 });
    }
    if src.len() >= 100 && src.starts_with(b"SQLite format 3 ") {
        // the db header names its own page grid (BE u16 at 16; 1 = 64 KB)
        let ps = u16::from_be_bytes([src[16], src[17]]) as u32;
        let page = if ps == 1 { 65536 } else { ps };
        if page >= 512 {
            out.push(Cand { id: FILTER_STRIDE, param: page });
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
                if bits == 16 && (1..=8).contains(&channels) {
                    out.push(Cand { id: FILTER_W16, param: channels as u32 });
                    out.push(Cand { id: FILTER_W16O2, param: channels as u32 });
                    if channels == 2 {
                        // v12-M3: the stereo pair as mid and side (two silhouettes)
                        out.push(Cand { id: FILTER_MS1, param: 2 });
                        out.push(Cand { id: FILTER_MS2, param: 2 });
                    }
                } else if (1..=4096).contains(&block_align) {
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
    let max_lag = 16384.min(sample.len() / 4);
    let folded_mean = |lag: usize| -> f64 {
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
        acc as f64 / n.max(1) as f64
    };
    let mut best = (0usize, f64::MAX);
    for lag in 1..=max_lag {
        let mean = folded_mean(lag);
        if mean < best.1 {
            best = (lag, mean);
        }
    }
    // random bytes fold to a mean of 64; only a clear period is worth a trial
    if best.1 < 44.0 {
        // a strong period often echoes at half or double: probe the harmonics
        for h in [best.0 / 2, best.0 * 2] {
            if h >= 1 && h <= max_lag {
                let m = folded_mean(h);
                if m < best.1 {
                    best = (h, m);
                }
            }
        }
        let lag = best.0;
        if lag <= 4 {
            return Some(Cand { id: lag as u8, param: 0 });
        }
        return Some(Cand { id: FILTER_STRIDE, param: lag as u32 });
    }
    None
}

/// is this a structure-tier id? Those transforms read the WHOLE file's own
/// table of contents; a 3-slice sample can neither apply them nor price
/// them (each slice lacks the directory / has wrong instruction offsets),
/// so they bypass the sample prune and go straight to the full trial --
/// they are already sniff-gated to at most a couple per file.
fn is_structural(id: u8) -> bool {
    id == FILTER_BCJ || id == FILTER_TTF || id == FILTER_RLE8
}

/// nominate (sniff + autocorrelation), prune by the dyadic stage's own
/// literal price on the filtered sample. Returns EVERY candidate whose
/// sample gain clears 0.5 bit/byte, in fixed order (sniff first, then
/// autocorrelation) -- the FULL trial in main decides among them.
/// Structure-tier candidates bypass the prune (see is_structural).
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
        if is_structural(c.id) {
            return true;
        }
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
    println!("  probe: NOMINATED (what the full trial sees, after the prune): {:?}", nominate(src));
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
            (8, 0), // v9 order-2: ch 0 must be a safe no-op
            (8, 1),
            (8, 2),
            (8, 3),
            (9, 0), // v9 big-endian
            (9, 1),
            (9, 2),
            (9, 3),
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
    fn ms_lifting_round_trips() {
        // every (L, R) pair, including the wrap corners
        let mut st = 0x2545F4914F6CDD1Du64;
        let mut src = Vec::new();
        for _ in 0..20000 {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            src.extend_from_slice(&(st as u32).to_le_bytes());
        }
        for corner in [[0x00, 0x80, 0xff, 0x7f], [0xff, 0x7f, 0x00, 0x80], [0x01, 0x00, 0xff, 0xff], [0x00, 0x80, 0x00, 0x80]] {
            src.extend_from_slice(&corner);
        }
        src.extend_from_slice(&[1, 2, 3]); // an odd tail stays verbatim
        assert_eq!(ms_unlift(&ms_lift(&src)), src);
        assert_eq!(ms1_undo(&ms1_apply(&src)), src);
        assert_eq!(ms2_undo(&ms2_apply(&src)), src);
        assert_eq!(undo(&apply(&src, FILTER_MS1, 2), FILTER_MS1, 2), src);
        assert_eq!(undo(&apply(&src, FILTER_MS2, 2), FILTER_MS2, 2), src);
        // a correlated pair: the side channel is small
        let mut corr = Vec::new();
        for i in 0..4096i32 {
            let l = ((i * 37) % 2000 - 1000) as i16;
            let r = l.wrapping_add(((i % 7) - 3) as i16);
            corr.extend_from_slice(&l.to_le_bytes());
            corr.extend_from_slice(&r.to_le_bytes());
        }
        let ms = ms_lift(&corr);
        let side_hi_zero = (0..ms.len() / 4).filter(|&k| ms[4 * k + 3] == 0 || ms[4 * k + 3] == 0xff).count();
        assert_eq!(side_hi_zero, ms.len() / 4, "side must be small on a correlated pair");
        assert_eq!(ms_unlift(&ms), corr);
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
