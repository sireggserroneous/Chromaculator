//! structure.rs -- the structure tier (v9-M5): the file's own table of
//! contents names its grids, and the geometry is applied AT those grids.
//! The honest-grouping reading (wubbadub.html:1416-1417): "commas fall on
//! the anti-diagonals of a square. A rectangle has no such arcs, so its
//! rows are the honest grouping" -- the grouping must fit the object's own
//! shape. Format facts here (x86 rel32 opcodes, the TTF table directory)
//! are borrowed from the formats' public specs and attributed.
//!
//! THE INVERTIBILITY LAW: undo re-derives its segment map from the
//! TRANSFORMED bytes, so every byte a segmenter READS (opcodes, directory
//! entries, the head table) is left VERBATIM by the transform, and apply
//! and undo make byte-identical validity decisions from byte-identical
//! evidence. Anything malformed passes through untouched. The fuzz tests
//! below are the proof, not this comment.

// ---------------- id 10: BCJ x86 (E8/E9 rel32 -> absolute) ----------------
// A checked port of Bra86.c from Igor Pavlov's LZMA SDK (public domain) --
// ported, not invented. Call/jmp displacements become absolute addresses, so
// repeated call targets respell as repeated bytes. ip = 0 on both sides;
// opcodes are untouched, so apply and undo scan identical positions.

#[inline]
fn test_msb(b: u8) -> bool {
    b == 0 || b == 0xFF
}
fn bra86(data: &mut [u8], ip: u32, encoding: bool) {
    let full = data.len();
    if full < 5 {
        return;
    }
    let size = full - 4;
    let ip = ip.wrapping_add(5);
    let mut pos: usize = 0;
    let mut mask: u32 = 0;
    loop {
        let mut p = pos;
        while p < size && (data[p] & 0xFE) != 0xE8 {
            p += 1;
        }
        let d = p - pos;
        pos = p;
        if p >= size {
            return;
        }
        if d > 2 {
            mask = 0;
        } else {
            mask >>= d;
            if mask != 0
                && (mask > 4 || mask == 3 || test_msb(data[p + (mask as usize >> 1) + 1]))
            {
                mask = (mask >> 1) | 4;
                pos += 1;
                continue;
            }
        }
        if test_msb(data[p + 4]) {
            let mut v = u32::from_le_bytes([data[p + 1], data[p + 2], data[p + 3], data[p + 4]]);
            let cur = ip.wrapping_add(pos as u32);
            pos += 5;
            if encoding {
                v = v.wrapping_add(cur);
            } else {
                v = v.wrapping_sub(cur);
            }
            if mask != 0 {
                let sh = (mask & 6) << 2;
                if test_msb((v >> sh) as u8) {
                    v ^= (0x100u32 << sh).wrapping_sub(1);
                    if encoding {
                        v = v.wrapping_add(cur);
                    } else {
                        v = v.wrapping_sub(cur);
                    }
                }
                mask = 0;
            }
            data[p + 1] = v as u8;
            data[p + 2] = (v >> 8) as u8;
            data[p + 3] = (v >> 16) as u8;
            data[p + 4] = 0u8.wrapping_sub(((v >> 24) & 1) as u8);
        } else {
            mask = (mask >> 1) | 4;
            pos += 1;
        }
    }
}
pub fn bcj_apply(src: &[u8]) -> Vec<u8> {
    let mut d = src.to_vec();
    bra86(&mut d, 0, true);
    d
}
pub fn bcj_undo(buf: &[u8]) -> Vec<u8> {
    let mut d = buf.to_vec();
    bra86(&mut d, 0, false);
    d
}

// ---------------- id 11: TTF segmented (loca delta + metrics W16BE) ----------------
// The table directory and the head table stay VERBATIM (they are what both
// sides read); only `loca` (monotone big-endian offsets -> consecutive
// deltas) and `hmtx`/`vmtx` (pairs of big-endian u16 -> per-channel deltas)
// are respelled in place, length-preserving. Any malformed or overlapping
// directory means identity -- decided identically on both sides.

struct TtfPlan {
    loca: Option<(usize, usize, bool)>, // (offset, length, long_format)
    metrics: Vec<(usize, usize)>,       // hmtx / vmtx regions
}
fn ttf_plan(buf: &[u8]) -> Option<TtfPlan> {
    if buf.len() < 12 {
        return None;
    }
    let magic_ok =
        buf[0..4] == [0x00, 0x01, 0x00, 0x00] || &buf[0..4] == b"OTTO" || &buf[0..4] == b"true";
    if !magic_ok {
        return None;
    }
    let n = u16::from_be_bytes([buf[4], buf[5]]) as usize;
    if n == 0 || n > 512 {
        return None;
    }
    let dir_end = 12 + 16 * n;
    if dir_end > buf.len() {
        return None;
    }
    let mut loca: Option<(usize, usize)> = None;
    let mut head: Option<(usize, usize)> = None;
    let mut metrics: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        let e = 12 + 16 * i;
        let tag = &buf[e..e + 4];
        let off = u32::from_be_bytes(buf[e + 8..e + 12].try_into().unwrap()) as usize;
        let len = u32::from_be_bytes(buf[e + 12..e + 16].try_into().unwrap()) as usize;
        if off.checked_add(len).is_none_or(|end| end > buf.len()) || off < dir_end {
            // any out-of-bounds entry poisons trust in the whole directory
            return None;
        }
        match tag {
            b"loca" => loca = Some((off, len)),
            b"head" => head = Some((off, len)),
            b"hmtx" | b"vmtx" => metrics.push((off, len)),
            _ => {}
        }
    }
    // loca's entry width comes from the (verbatim) head table
    let loca_full = match (loca, head) {
        (Some((lo, ll)), Some((ho, hl))) if hl >= 54 => {
            let fmt = i16::from_be_bytes([buf[ho + 50], buf[ho + 51]]);
            match fmt {
                0 => Some((lo, ll, false)),
                1 => Some((lo, ll, true)),
                _ => None,
            }
        }
        _ => None,
    };
    // every region we rewrite must be disjoint from the directory, the head
    // table, and each other -- otherwise a rewrite would move the evidence
    // the other side reads
    let mut zones: Vec<(usize, usize)> = vec![(0, dir_end)];
    if let Some((ho, hl)) = head {
        zones.push((ho, ho + hl));
    }
    let mut targets: Vec<(usize, usize)> = Vec::new();
    if let Some((lo, ll, _)) = loca_full {
        targets.push((lo, lo + ll));
    }
    for &(o, l) in &metrics {
        targets.push((o, o + l));
    }
    let mut all = zones.clone();
    all.extend_from_slice(&targets);
    all.sort_unstable();
    for w in all.windows(2) {
        if w[0].1 > w[1].0 {
            return None; // overlap: identity, decided from verbatim bytes
        }
    }
    if targets.is_empty() {
        return None;
    }
    Some(TtfPlan { loca: loca_full, metrics })
}
fn delta_be_in_place(region: &mut [u8], width: usize, encoding: bool) {
    // consecutive big-endian values -> deltas (first verbatim), wrapping
    let n = region.len() / width;
    if n < 2 {
        return;
    }
    if encoding {
        // walk backward so each delta reads the ORIGINAL predecessor
        for i in (1..n).rev() {
            let o = i * width;
            let po = o - width;
            if width == 2 {
                let v = u16::from_be_bytes([region[o], region[o + 1]]);
                let p = u16::from_be_bytes([region[po], region[po + 1]]);
                region[o..o + 2].copy_from_slice(&v.wrapping_sub(p).to_be_bytes());
            } else {
                let v = u32::from_be_bytes(region[o..o + 4].try_into().unwrap());
                let p = u32::from_be_bytes(region[po..po + 4].try_into().unwrap());
                region[o..o + 4].copy_from_slice(&v.wrapping_sub(p).to_be_bytes());
            }
        }
    } else {
        for i in 1..n {
            let o = i * width;
            let po = o - width;
            if width == 2 {
                let d = u16::from_be_bytes([region[o], region[o + 1]]);
                let p = u16::from_be_bytes([region[po], region[po + 1]]);
                region[o..o + 2].copy_from_slice(&d.wrapping_add(p).to_be_bytes());
            } else {
                let d = u32::from_be_bytes(region[o..o + 4].try_into().unwrap());
                let p = u32::from_be_bytes(region[po..po + 4].try_into().unwrap());
                region[o..o + 4].copy_from_slice(&d.wrapping_add(p).to_be_bytes());
            }
        }
    }
}
fn metrics_w16be_in_place(region: &mut [u8], encoding: bool) {
    // pairs of big-endian u16 (advance, bearing): per-channel delta at
    // stride 2 values = 4 bytes; first frame verbatim; remainder verbatim
    let frame = 4;
    let body = (region.len() / frame) * frame;
    if body < 2 * frame {
        return;
    }
    if encoding {
        for o in ((frame..body).step_by(2)).rev() {
            let v = u16::from_be_bytes([region[o], region[o + 1]]);
            let p = u16::from_be_bytes([region[o - frame], region[o - frame + 1]]);
            region[o..o + 2].copy_from_slice(&v.wrapping_sub(p).to_be_bytes());
        }
    } else {
        for o in (frame..body).step_by(2) {
            let d = u16::from_be_bytes([region[o], region[o + 1]]);
            let p = u16::from_be_bytes([region[o - frame], region[o - frame + 1]]);
            region[o..o + 2].copy_from_slice(&d.wrapping_add(p).to_be_bytes());
        }
    }
}
fn ttf_convert(buf: &[u8], encoding: bool) -> Vec<u8> {
    let plan = match ttf_plan(buf) {
        Some(p) => p,
        None => return buf.to_vec(), // malformed -> identity, both sides alike
    };
    let mut out = buf.to_vec();
    if let Some((off, len, long)) = plan.loca {
        let width = if long { 4 } else { 2 };
        let usable = (len / width) * width;
        delta_be_in_place(&mut out[off..off + usable], width, encoding);
    }
    for &(off, len) in &plan.metrics {
        metrics_w16be_in_place(&mut out[off..off + len], encoding);
    }
    out
}
pub fn ttf_apply(src: &[u8]) -> Vec<u8> {
    ttf_convert(src, true)
}
pub fn ttf_undo(buf: &[u8]) -> Vec<u8> {
    ttf_convert(buf, false)
}

// ---------------- the fuzz gate: invertibility proven, not argued ----------------
#[cfg(test)]
mod tests {
    use super::*;

    fn xs(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }
    fn roundtrip(name: &str, data: &[u8]) {
        let a = bcj_apply(data);
        assert_eq!(a.len(), data.len(), "bcj length {}", name);
        assert_eq!(bcj_undo(&a), data, "bcj roundtrip {}", name);
        let t = ttf_apply(data);
        assert_eq!(t.len(), data.len(), "ttf length {}", name);
        assert_eq!(ttf_undo(&t), data, "ttf roundtrip {}", name);
    }

    #[test]
    fn structure_lengths_0_to_1024() {
        let mut st = 0x1489u64;
        for len in 0..=1024usize {
            let data: Vec<u8> = (0..len).map(|_| (xs(&mut st) & 0xff) as u8).collect();
            roundtrip(&format!("random len {}", len), &data);
        }
    }

    #[test]
    fn structure_fuzz_real_files() {
        // every real corpus file + 2000 mutants each (first 64 KB): the
        // segmenter must never panic and never lose a byte
        let dir = std::path::Path::new("corpus-real");
        if !dir.exists() {
            panic!("corpus-real missing: the fuzz gate needs the real files");
        }
        let mut st = 0xACEu64;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let data = std::fs::read(&path).unwrap();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            roundtrip(&name, &data);
            let head: Vec<u8> = data.iter().take(64 * 1024).copied().collect();
            for m in 0..2000 {
                let mut mutant = head.clone();
                match xs(&mut st) % 3 {
                    0 => {
                        // truncate
                        let cut = (xs(&mut st) as usize) % mutant.len().max(1);
                        mutant.truncate(cut);
                    }
                    1 => {
                        // scratch
                        if !mutant.is_empty() {
                            let at = (xs(&mut st) as usize) % mutant.len();
                            let len = 1 + (xs(&mut st) as usize) % 256;
                            for i in at..(at + len).min(mutant.len()) {
                                mutant[i] = (xs(&mut st) & 0xff) as u8;
                            }
                        }
                    }
                    _ => {
                        // bit flips
                        for _ in 0..1 + (xs(&mut st) % 16) {
                            if !mutant.is_empty() {
                                let bit = (xs(&mut st) as usize) % (mutant.len() * 8);
                                mutant[bit >> 3] ^= 1 << (bit & 7);
                            }
                        }
                    }
                }
                roundtrip(&format!("{} mutant {}", name, m), &mutant);
            }
        }
    }
}

// ---------------- id 12: BMP RLE8 unrolled (v11-M6) ----------------
// The nested-readings law applied to the simplest image codec: the pixels
// are the reading underneath the run codes. NOT canonical in the wild, so
// the filter fires ONLY when our canonical re-encode reproduces the original
// stream byte-for-byte (the TTF invertibility law: verified at apply, undo
// re-derives the runs from the pixels). Anything else passes through.

/// canonical RLE8: pure runs [k,v] (k>=1), row end [0,0], image end [0,1]
fn rle8_encode_canon(pix: &[u8], w: usize, h: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for row in 0..h {
        let r = &pix[row * w..(row + 1) * w];
        let mut i = 0;
        while i < w {
            let v = r[i];
            let mut k = 1usize;
            while i + k < w && r[i + k] == v && k < 255 {
                k += 1;
            }
            out.push(k as u8);
            out.push(v);
            i += k;
        }
        if row + 1 < h {
            out.push(0);
            out.push(0);
        }
    }
    out.push(0);
    out.push(1);
    out
}
/// decode any spec-legal RLE8 stream (runs, EOL, EOD, delta, absolute)
fn rle8_decode(rle: &[u8], w: usize, h: usize) -> Option<(Vec<u8>, usize)> {
    let mut pix = vec![0u8; w * h];
    let (mut x, mut y, mut i) = (0usize, 0usize, 0usize);
    loop {
        if i + 2 > rle.len() || y >= h {
            return None;
        }
        let (a, b) = (rle[i], rle[i + 1]);
        i += 2;
        if a > 0 {
            let k = a as usize;
            if x + k > w {
                return None;
            }
            for _ in 0..k {
                pix[y * w + x] = b;
                x += 1;
            }
        } else {
            match b {
                0 => {
                    x = 0;
                    y += 1;
                }
                1 => return Some((pix, i)),
                2 => {
                    if i + 2 > rle.len() {
                        return None;
                    }
                    x += rle[i] as usize;
                    y += rle[i + 1] as usize;
                    i += 2;
                    if x > w || y > h {
                        return None;
                    }
                }
                n => {
                    let k = n as usize;
                    if x + k > w || i + k > rle.len() {
                        return None;
                    }
                    for j in 0..k {
                        pix[y * w + x] = rle[i + j];
                        x += 1;
                    }
                    i += k + (k & 1); // absolute runs pad to word
                }
            }
        }
    }
}
struct RleShape {
    off: usize,
    w: usize,
    h: usize,
    rle_len: usize,
}
fn rle8_shape(src: &[u8]) -> Option<RleShape> {
    if src.len() < 54 || &src[0..2] != b"BM" {
        return None;
    }
    let off = u32::from_le_bytes(src[10..14].try_into().ok()?) as usize;
    let w = i32::from_le_bytes(src[18..22].try_into().ok()?);
    let h = i32::from_le_bytes(src[22..26].try_into().ok()?);
    let bpp = u16::from_le_bytes(src[28..30].try_into().ok()?);
    let comp = u32::from_le_bytes(src[30..34].try_into().ok()?);
    if comp != 1 || bpp != 8 || w <= 0 || h <= 0 || off >= src.len() || off < 54 {
        return None;
    }
    let (w, h) = (w as usize, h as usize);
    if w * h > 1 << 28 {
        return None;
    }
    let (pix, used) = rle8_decode(&src[off..], w, h)?;
    // the invertibility law: only a stream our canon reproduces exactly
    if rle8_encode_canon(&pix, w, h) != src[off..off + used] {
        return None;
    }
    Some(RleShape { off, w, h, rle_len: used })
}
pub fn rle8_sniff(src: &[u8]) -> bool {
    rle8_shape(src).is_some()
}
/// [off u32][rle_len u32][w u32][h u32][head verbatim][tail verbatim][pixels]
pub fn rle8_apply(src: &[u8]) -> Vec<u8> {
    let sh = match rle8_shape(src) {
        Some(s) => s,
        None => return src.to_vec(), // never fires: candidates() sniffed
    };
    let (pix, _) = rle8_decode(&src[sh.off..], sh.w, sh.h).expect("sniffed");
    let mut out = Vec::with_capacity(src.len() + pix.len());
    out.extend_from_slice(&(sh.off as u32).to_le_bytes());
    out.extend_from_slice(&(sh.rle_len as u32).to_le_bytes());
    out.extend_from_slice(&(sh.w as u32).to_le_bytes());
    out.extend_from_slice(&(sh.h as u32).to_le_bytes());
    out.extend_from_slice(&src[..sh.off]);
    out.extend_from_slice(&src[sh.off + sh.rle_len..]);
    out.extend_from_slice(&pix);
    out
}
pub fn rle8_undo(buf: &[u8]) -> Vec<u8> {
    let off = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
    let rle_len = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as usize;
    let w = u32::from_le_bytes(buf[8..12].try_into().unwrap()) as usize;
    let h = u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize;
    let head = &buf[16..16 + off];
    let pix_at = buf.len() - w * h;
    let tail = &buf[16 + off..pix_at];
    let pix = &buf[pix_at..];
    let rle = rle8_encode_canon(pix, w, h);
    debug_assert_eq!(rle.len(), rle_len);
    let mut out = Vec::with_capacity(off + rle.len() + tail.len());
    out.extend_from_slice(head);
    out.extend_from_slice(&rle);
    out.extend_from_slice(tail);
    out
}

#[cfg(test)]
mod rle_tests {
    use super::*;
    fn mk_bmp(w: usize, h: usize, seed: u64) -> Vec<u8> {
        // 8bpp RLE8 bmp with 54B header + 1024B palette, canon-encoded
        let mut st = seed;
        let mut pix = vec![0u8; w * h];
        for p in pix.iter_mut() {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            *p = if st & 7 == 0 { (st >> 8) as u8 } else { *p }; // runs dominate
        }
        let rle = rle8_encode_canon(&pix, w, h);
        let off = 54 + 1024;
        let mut f = vec![0u8; off];
        f[0] = b'B';
        f[1] = b'M';
        f[10..14].copy_from_slice(&(off as u32).to_le_bytes());
        f[14..18].copy_from_slice(&40u32.to_le_bytes());
        f[18..22].copy_from_slice(&(w as u32).to_le_bytes());
        f[22..26].copy_from_slice(&(h as u32).to_le_bytes());
        f[26..28].copy_from_slice(&1u16.to_le_bytes());
        f[28..30].copy_from_slice(&8u16.to_le_bytes());
        f[30..34].copy_from_slice(&1u32.to_le_bytes());
        f.extend_from_slice(&rle);
        f
    }
    #[test]
    fn rle8_round_trip_and_mutation_fuzz() {
        let base = mk_bmp(64, 48, 0x1489);
        assert!(rle8_sniff(&base));
        let t = rle8_apply(&base);
        assert_eq!(rle8_undo(&t), base, "clean round-trip");
        // mutation fuzz: every accepted mutant must round-trip EXACT;
        // rejected mutants pass through untouched by candidates()
        let mut st = 0xACEu64;
        for _ in 0..500 {
            let mut m = base.clone();
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            let at = (st as usize) % m.len();
            m[at] ^= (st >> 11) as u8 | 1;
            if rle8_sniff(&m) {
                assert_eq!(rle8_undo(&rle8_apply(&m)), m, "mutant round-trip");
            }
        }
    }
}
