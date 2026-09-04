//! twod.rs -- THE SECOND DIMENSION (v13-M3c, WS-2D).
//!
//! `iconcache48.db` is 10,526 raw 48x48 BGRA bitmaps modelled, until now, as a
//! byte stream; its bit autocorrelation peaks hard at 32/64/96/128 bits, which
//! is the 4-byte pixel. The v11 RLE post-mortem said 2D context is the lever and
//! nothing in the series had ever built it.
//!
//! **This is not the transpose that died at M3b**, and it is not the lattice
//! either. The transpose REORDERED the bytes and threw local context away. The
//! lattice (`mix12.rs`, two of the twelve mixer inputs) reads down ONE learned
//! stride and keeps local context, but it never forms the JOINT context
//! `(above, left, above-left, above-right)` that every lossless image coder is
//! built on. This forms it, and it moves no bytes at all: the model sees a
//! rectangle, the stream stays a stream.
//!
//! The stride and the pixel are MEASURED by the encoder and ride in the arm's
//! own five-byte header, so the decoder reads them rather than guessing. Both
//! directions read only bytes that have already been coded -- `above-right` is
//! at `pos - stride + pixel`, which is behind `pos` exactly while
//! `pixel < stride`, and `nominate` guarantees that.

/// how much of a file the stride hunt reads
const SAMPLE: usize = 4 << 20;
/// the largest stride the hunt will name
pub const MAX_STRIDE: usize = 1 << 16;
/// below this a file has no rectangle worth finding
const MIN_LEN: usize = 1 << 16;

#[inline]
fn h4(b: &[u8]) -> usize {
    let k = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as u64;
    (k.wrapping_mul(0x9E3779B97F4A7C15) >> 44) as usize
}

/// which rectangle, if any, this form is written on. A NOMINATION, never a
/// decision: the roster still judges the arm on the armored total.
///
/// The hunt is the standard record-length histogram -- for every position, the
/// distance back to the last place the same four bytes appeared -- which is
/// O(n) and finds a row stride without ever scanning a candidate list. The
/// argmax alone would name the ICON on `iconcache48.db` (9,216 B) rather than
/// its ROW (192 B), so the smallest divisor of the argmax that still carries a
/// third of its count wins: that is the row, and the row is the context.
pub fn nominate(src: &[u8]) -> Option<(u32, u32)> {
    if src.len() < MIN_LEN {
        return None;
    }
    let n = src.len().min(SAMPLE);
    let mut cnt = vec![0u32; MAX_STRIDE + 1];
    let mut last = vec![0u32; 1 << 20];
    for i in 0..n - 4 {
        let h = h4(&src[i..i + 4]);
        let l = last[h] as usize;
        if l != 0 {
            let d = i + 1 - l;
            if d <= MAX_STRIDE {
                cnt[d] += 1;
            }
        }
        last[h] = (i + 1) as u32;
    }
    let total: u64 = cnt.iter().map(|&c| c as u64).sum();
    let mut best = 0usize;
    for d in 8..=MAX_STRIDE {
        if cnt[d] > cnt[best] {
            best = d;
        }
    }
    if best == 0 {
        return None;
    }
    // the rectangle has to actually stand out of the noise floor
    let floor = (total / MAX_STRIDE as u64).max(1) as u32;
    if cnt[best] < floor.saturating_mul(8) || (cnt[best] as u64) * 200 < total {
        return None;
    }
    let mut stride = best;
    for d in 8..best {
        if best.is_multiple_of(d) && (cnt[d] as u64) * 3 >= cnt[best] as u64 {
            stride = d;
            break;
        }
    }
    // the pixel: the shortest repeat that beats the byte reading clearly
    let m = n.min(1 << 20);
    let mut pixel = 1usize;
    let mut bestp = 0usize;
    for p in 1..=4usize {
        if p >= stride {
            break;
        }
        let c = (p..m).filter(|&i| src[i] == src[i - p]).count();
        if c * 20 > bestp * 23 {
            bestp = c;
            pixel = p;
        }
    }
    Some((stride as u32, pixel as u32))
}

/// the two 2D context keys at `pos`, read from bytes already coded. `buf` is
/// the source when encoding and the output so far when decoding, and they are
/// the same bytes, which is the whole reason this is legal.
#[inline]
pub fn keys(buf: &[u8], pos: usize, stride: usize, pixel: usize) -> (u64, u64) {
    let at = |d: usize| -> u64 {
        if d <= pos {
            buf[pos - d] as u64
        } else {
            0x100
        }
    };
    let w = at(pixel);
    let n = at(stride);
    let nw = at(stride + pixel);
    let ne = if stride >= pixel { at(stride - pixel) } else { 0x100 };
    let phase = (pos % pixel) as u64;
    let k0 = n | (w << 9) | (phase << 18) | (0xB1u64 << 56);
    let k1 = nw | (ne << 9) | (n << 18) | (phase << 27) | (0xB2u64 << 56);
    (k0, k1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a synthetic 192-byte-stride image is named at 192, not at its frame
    #[test]
    fn the_hunt_finds_the_row_not_the_frame() {
        // an image, not noise: each row is the row above with a few pixels
        // changed, which is what a 48x48 icon actually looks like
        let mut v = Vec::new();
        let mut s = 0x2545F491u32;
        let mut rnd = || {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            s
        };
        let mut row: Vec<u8> = (0..192).map(|_| (rnd() >> 24) as u8).collect();
        for _frame in 0..64 {
            for _r in 0..48 {
                for _ in 0..8 {
                    let at = (rnd() as usize) % 192;
                    row[at] = (rnd() >> 24) as u8;
                }
                v.extend_from_slice(&row);
            }
        }
        let (stride, pixel) = nominate(&v).expect("a rectangle");
        assert!(stride == 192 || 192 % stride == 0, "stride {} is not the row", stride);
        assert!(pixel < stride);
    }

    /// prose has no rectangle, and the keys never read forward
    #[test]
    fn the_keys_are_causal() {
        let buf: Vec<u8> = (0..1000u32).map(|i| (i * 7) as u8).collect();
        for pos in 0..buf.len() {
            let (k0, k1) = keys(&buf, pos, 192, 4);
            let (j0, j1) = keys(&buf[..pos + 1], pos, 192, 4);
            assert_eq!(k0, j0, "pos {}", pos);
            assert_eq!(k1, j1, "pos {}", pos);
        }
    }
}
