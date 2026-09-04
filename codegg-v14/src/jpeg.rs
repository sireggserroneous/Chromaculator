//! jpeg.rs -- the JPEG peel (v13-M1, WS-J): the foreign code read as a value.
//!
//! spec.html:359-360, the site's tooltip for `form`: "The value underneath does
//! not change -- compare the two and only the colours differ." A JPEG's Huffman
//! bits and its quantised DCT coefficients are one value in two spellings. This
//! module peels the spelling off (`peel`) and puts it back on byte for byte
//! (`respell`); `jcoef.rs` models what is underneath.
//!
//! THE LAW OF THE PEEL: a peel is a bijection or it is not used. `peel` returns
//! the parse; the caller re-encodes it with `respell` and compares against the
//! original bytes BEFORE anything is written. One byte off and the raw bytes go
//! through the ordinary pipeline instead. Nothing here guesses: everything the
//! re-encode needs is what the stream itself declared, kept verbatim in the
//! recipe (the whole marker skeleton) and re-parsed on the way back.
//!
//! Refusals, printed with their reason, never guessed at: progressive (SOF2),
//! arithmetic-coded (SOF9/SOF10), lossless and every other SOFn, 12-bit
//! samples, more than one scan, a scan that names some but not all components,
//! a missing or truncated entropy segment, a Huffman code the tables do not
//! define, a DNL height.
//!
//! Attribution: ITU T.81 (the codec, the canonical decode, the 1-bit padding
//! rule of F.1.2.3); packJPG (Matthias Stirner) and Lepton (Dropbox) for the
//! idea that the coefficients, not the bits, are the thing to model.

#[derive(Clone, Debug)]
pub struct Comp {
    pub id: u8,
    pub h: usize,
    pub v: usize,
    pub tq: usize,
    /// blocks across / down of this component's own plane (MCU-padded when
    /// the scan is interleaved)
    pub bw: usize,
    pub bh: usize,
    /// the DC and AC Huffman table selectors this scan gave the component
    pub td: usize,
    pub ta: usize,
}

/// one canonical Huffman table, both directions (T.81 Annex C)
#[derive(Clone)]
pub struct Huff {
    /// (length, code) -> symbol, as a flat per-length map
    pub min_code: [i32; 17],
    pub max_code: [i32; 17],
    pub val_ptr: [usize; 17],
    pub vals: Vec<u8>,
    /// symbol -> (length, code)
    pub enc: [(u8, u16); 256],
    pub present: bool,
}
impl Default for Huff {
    fn default() -> Huff {
        Huff { min_code: [0; 17], max_code: [-1; 17], val_ptr: [0; 17], vals: Vec::new(), enc: [(0, 0); 256], present: false }
    }
}
impl Huff {
    fn build(counts: &[u8; 16], vals: Vec<u8>) -> Result<Huff, String> {
        let mut h = Huff { vals, present: true, ..Default::default() };
        let mut code: i32 = 0;
        let mut k = 0usize;
        for l in 1..=16usize {
            h.val_ptr[l] = k;
            h.min_code[l] = code;
            let n = counts[l - 1] as usize;
            for _ in 0..n {
                if k >= h.vals.len() {
                    return Err("DHT: symbol list shorter than its counts".into());
                }
                if code > 0xFFFF {
                    return Err("DHT: code overflows 16 bits".into());
                }
                h.enc[h.vals[k] as usize] = (l as u8, code as u16);
                code += 1;
                k += 1;
            }
            h.max_code[l] = if n == 0 { -1 } else { code - 1 };
            code <<= 1;
        }
        Ok(h)
    }
}

#[derive(Clone)]
pub struct Jpeg {
    /// bytes [0, scan_start): SOI and every marker segment through the SOS header
    pub prefix: Vec<u8>,
    /// bytes [scan_end, len): EOI and anything the file carries after it
    pub suffix: Vec<u8>,
    /// how this encoder padded the last partial byte before every RSTn and the
    /// end of the scan: true = 1-bits (T.81 F.1.2.3), false = 0-bits
    pub pad_ones: bool,
    pub width: usize,
    pub height: usize,
    pub comps: Vec<Comp>,
    pub mcux: usize,
    pub mcuy: usize,
    pub dri: usize,
    pub interleaved: bool,
    /// the quantisation tables, verbatim (the model reads them as a context)
    pub qt: [[u16; 64]; 4],
    /// the coefficient planes, zigzag order, one Vec of bw*bh*64 per component
    pub coef: Vec<Vec<i16>>,
    dc: [Huff; 4],
    ac: [Huff; 4],
}

impl Jpeg {
    /// total coded blocks over every component
    pub fn nblocks(&self) -> usize {
        self.comps.iter().map(|c| c.bw * c.bh).sum()
    }
    /// what this frame is, in one line: the peel trace and every refusal report
    /// print it, so a reading is never a guess about which file was meant
    pub fn describe(&self) -> String {
        format!(
            "{}x{}, {} comp ({}), {} blocks, DRI {}, pad {}",
            self.width,
            self.height,
            self.comps.len(),
            self.comps.iter().map(|c| format!("{}:{}x{}", c.id, c.h, c.v)).collect::<Vec<_>>().join(" "),
            self.nblocks(),
            self.dri,
            if self.pad_ones { "ones" } else { "zeros" }
        )
    }
}

// ---------------------------------------------------------------- the parse

/// (width, height, [(component id, h, v, quant table)]) as the SOF declared it
type Frame = (usize, usize, Vec<(u8, usize, usize, usize)>);

struct Parsed {
    prefix_end: usize,
    scan_end: usize,
    j: Jpeg,
}

/// parse the marker skeleton of `src` and set up the (empty) coefficient planes.
/// Refuses -- with the reason -- anything this peel cannot re-spell exactly.
fn parse_skeleton(src: &[u8], want_scan_end: bool) -> Result<Parsed, String> {
    if src.len() < 4 || src[0] != 0xFF || src[1] != 0xD8 {
        return Err("not a JPEG (no SOI)".into());
    }
    let mut qt = [[0u16; 64]; 4];
    let mut dc: [Huff; 4] = Default::default();
    let mut ac: [Huff; 4] = Default::default();
    let mut frame: Option<Frame> = None;
    let mut dri = 0usize;
    let mut i = 2usize;
    let prefix_end;
    let mut scan_sel: Vec<(u8, usize, usize)> = Vec::new();
    loop {
        if i + 1 >= src.len() {
            return Err("truncated before the scan".into());
        }
        if src[i] != 0xFF {
            return Err(format!("marker desync at offset {}", i));
        }
        let mut m = src[i + 1];
        // fill bytes: 0xFF may repeat before a marker code
        let mut adv = 0usize;
        while m == 0xFF && i + 2 + adv < src.len() {
            adv += 1;
            m = src[i + 1 + adv];
        }
        if adv > 0 {
            // a fill run in the skeleton is legal but we do not respell it
            return Err("fill bytes between markers (not respelled)".into());
        }
        match m {
            0xD8 => {
                i += 2;
                continue;
            }
            0xD9 => return Err("EOI before any scan".into()),
            0x01 | 0xD0..=0xD7 => {
                i += 2;
                continue;
            }
            _ => {}
        }
        if i + 4 > src.len() {
            return Err("truncated marker length".into());
        }
        let l = ((src[i + 2] as usize) << 8) | src[i + 3] as usize;
        if l < 2 || i + 2 + l > src.len() {
            return Err("marker segment runs past the end of the file".into());
        }
        let seg = &src[i + 4..i + 2 + l];
        match m {
            0xC0 => {
                // baseline sequential, Huffman -- the only frame we peel
                if seg.len() < 6 {
                    return Err("SOF0 too short".into());
                }
                if seg[0] != 8 {
                    return Err(format!("{}-bit samples (only 8-bit is peeled)", seg[0]));
                }
                let hh = ((seg[1] as usize) << 8) | seg[2] as usize;
                let ww = ((seg[3] as usize) << 8) | seg[4] as usize;
                if hh == 0 {
                    return Err("height 0 (a DNL frame)".into());
                }
                if ww == 0 {
                    return Err("width 0".into());
                }
                let nc = seg[5] as usize;
                if nc == 0 || nc > 4 || seg.len() < 6 + 3 * nc {
                    return Err(format!("{} frame components (1..4 supported)", nc));
                }
                let mut cs = Vec::with_capacity(nc);
                for k in 0..nc {
                    let id = seg[6 + 3 * k];
                    let h = (seg[7 + 3 * k] >> 4) as usize;
                    let v = (seg[7 + 3 * k] & 15) as usize;
                    let tq = seg[8 + 3 * k] as usize;
                    if !(1..=4).contains(&h) || !(1..=4).contains(&v) || tq > 3 {
                        return Err(format!("component {}: sampling {}x{} table {}", id, h, v, tq));
                    }
                    cs.push((id, h, v, tq));
                }
                if frame.is_some() {
                    return Err("two frames in one file".into());
                }
                frame = Some((ww, hh, cs));
            }
            0xC1 => return Err("SOF1 extended sequential (not peeled)".into()),
            0xC2 => return Err("progressive JPEG (SOF2): the bytes are kept".into()),
            0xC3 => return Err("lossless JPEG (SOF3): the bytes are kept".into()),
            0xC5..=0xC7 => return Err("differential SOF (not peeled)".into()),
            0xC9..=0xCB => return Err("arithmetic-coded JPEG (SOF9/10/11): the bytes are kept".into()),
            0xCD..=0xCF => return Err("differential arithmetic SOF (not peeled)".into()),
            0xCC => return Err("DAC (arithmetic conditioning): the bytes are kept".into()),
            0xDC => return Err("DNL (height defined after the scan): the bytes are kept".into()),
            0xC4 => {
                let mut p = 0usize;
                while p < seg.len() {
                    if p + 17 > seg.len() {
                        return Err("DHT segment truncated".into());
                    }
                    let tc = (seg[p] >> 4) as usize;
                    let th = (seg[p] & 15) as usize;
                    if tc > 1 || th > 3 {
                        return Err(format!("DHT class {} id {}", tc, th));
                    }
                    p += 1;
                    let mut counts = [0u8; 16];
                    counts.copy_from_slice(&seg[p..p + 16]);
                    p += 16;
                    let n: usize = counts.iter().map(|&c| c as usize).sum();
                    if p + n > seg.len() {
                        return Err("DHT symbol list truncated".into());
                    }
                    let vals = seg[p..p + n].to_vec();
                    p += n;
                    let h = Huff::build(&counts, vals)?;
                    if tc == 0 {
                        dc[th] = h;
                    } else {
                        ac[th] = h;
                    }
                }
            }
            0xDB => {
                let mut p = 0usize;
                while p < seg.len() {
                    let pq = (seg[p] >> 4) as usize;
                    let tq = (seg[p] & 15) as usize;
                    if tq > 3 || pq > 1 {
                        return Err(format!("DQT precision {} id {}", pq, tq));
                    }
                    p += 1;
                    let need = if pq == 1 { 128 } else { 64 };
                    if p + need > seg.len() {
                        return Err("DQT table truncated".into());
                    }
                    for k in 0..64 {
                        qt[tq][k] = if pq == 1 {
                            ((seg[p + 2 * k] as u16) << 8) | seg[p + 2 * k + 1] as u16
                        } else {
                            seg[p + k] as u16
                        };
                    }
                    p += need;
                }
            }
            0xDD => {
                if seg.len() < 2 {
                    return Err("DRI too short".into());
                }
                dri = ((seg[0] as usize) << 8) | seg[1] as usize;
            }
            0xDA => {
                if seg.is_empty() {
                    return Err("SOS too short".into());
                }
                let ns = seg[0] as usize;
                if ns == 0 || seg.len() < 1 + 2 * ns + 3 {
                    return Err("SOS header malformed".into());
                }
                for k in 0..ns {
                    scan_sel.push((seg[1 + 2 * k], (seg[2 + 2 * k] >> 4) as usize, (seg[2 + 2 * k] & 15) as usize));
                }
                let ss = seg[1 + 2 * ns];
                let se = seg[2 + 2 * ns];
                let a = seg[3 + 2 * ns];
                if ss != 0 || se != 63 || a != 0 {
                    return Err(format!("scan Ss {} Se {} Ah/Al {:02x} (not a baseline whole-block scan)", ss, se, a));
                }
                prefix_end = i + 2 + l;
                break;
            }
            _ => {}
        }
        i += 2 + l;
    }
    let (width, height, fcs) = frame.ok_or("no SOF0 frame before the scan")?;
    if scan_sel.len() != fcs.len() && fcs.len() != 1 {
        return Err(format!("scan names {} of {} components (only whole-frame scans are peeled)", scan_sel.len(), fcs.len()));
    }
    if scan_sel.len() != fcs.len() {
        return Err("single-component scan of a multi-component frame".into());
    }
    let hmax = fcs.iter().map(|c| c.1).max().unwrap();
    let vmax = fcs.iter().map(|c| c.2).max().unwrap();
    let interleaved = fcs.len() > 1;
    let mcux = width.div_ceil(8 * hmax);
    let mcuy = height.div_ceil(8 * vmax);
    let mut comps = Vec::with_capacity(fcs.len());
    for (id, h, v, tq) in fcs.iter().copied() {
        let (td, ta) = scan_sel
            .iter()
            .find(|s| s.0 == id)
            .map(|s| (s.1, s.2))
            .ok_or_else(|| format!("component {} is not in the scan", id))?;
        if td > 3 || ta > 3 {
            return Err("scan names a table id above 3".into());
        }
        if !dc[td].present {
            return Err(format!("scan names DC table {} that no DHT defined", td));
        }
        if !ac[ta].present {
            return Err(format!("scan names AC table {} that no DHT defined", ta));
        }
        let (bw, bh) = if interleaved {
            (mcux * h, mcuy * v)
        } else {
            ((width * h).div_ceil(hmax).div_ceil(8), (height * v).div_ceil(vmax).div_ceil(8))
        };
        comps.push(Comp { id, h, v, tq, bw, bh, td, ta });
    }
    // the entropy-coded segment: from prefix_end to the first marker that is
    // neither a stuffed 0x00 nor an RSTn
    let mut scan_end = prefix_end;
    if want_scan_end {
        let mut p = prefix_end;
        loop {
            if p + 1 >= src.len() {
                return Err("entropy-coded segment runs to the end of the file (truncated)".into());
            }
            if src[p] == 0xFF {
                let n = src[p + 1];
                if n == 0x00 || (0xD0..=0xD7).contains(&n) {
                    p += 2;
                    continue;
                }
                break;
            }
            p += 1;
        }
        scan_end = p;
        if scan_end == prefix_end {
            return Err("empty entropy-coded segment".into());
        }
        // one scan only: a second SOS in the tail is not this peel's business
        let tail = &src[scan_end..];
        let mut q = 0usize;
        while q + 1 < tail.len() {
            if tail[q] == 0xFF && tail[q + 1] == 0xDA {
                return Err("more than one scan (not peeled)".into());
            }
            q += 1;
        }
    }
    let coef = comps.iter().map(|c| vec![0i16; c.bw * c.bh * 64]).collect();
    Ok(Parsed {
        prefix_end,
        scan_end,
        j: Jpeg {
            prefix: src[..prefix_end].to_vec(),
            suffix: if want_scan_end { src[scan_end..].to_vec() } else { Vec::new() },
            pad_ones: true,
            width,
            height,
            comps,
            mcux,
            mcuy,
            dri,
            interleaved,
            qt,
            coef,
            dc,
            ac,
        },
    })
}

// ---------------------------------------------------------------- bit reader

struct Reader<'a> {
    d: &'a [u8],
    pos: usize,
    acc: u32,
    n: u32,
}
impl<'a> Reader<'a> {
    #[inline]
    fn bit(&mut self) -> Result<u32, String> {
        if self.n == 0 {
            if self.pos >= self.d.len() {
                return Err("entropy data ran out mid-block".into());
            }
            let b = self.d[self.pos];
            self.pos += 1;
            if b == 0xFF {
                if self.pos >= self.d.len() || self.d[self.pos] != 0 {
                    return Err("a marker appeared inside the entropy data".into());
                }
                self.pos += 1;
            }
            self.acc = b as u32;
            self.n = 8;
        }
        self.n -= 1;
        Ok((self.acc >> self.n) & 1)
    }
    #[inline]
    fn bits(&mut self, k: u32) -> Result<u32, String> {
        let mut v = 0u32;
        for _ in 0..k {
            v = (v << 1) | self.bit()?;
        }
        Ok(v)
    }
    fn huff(&mut self, t: &Huff) -> Result<u8, String> {
        let mut code: i32 = self.bit()? as i32;
        for l in 1..=16usize {
            if t.max_code[l] >= 0 && code <= t.max_code[l] {
                let idx = t.val_ptr[l] + (code - t.min_code[l]) as usize;
                return t.vals.get(idx).copied().ok_or_else(|| "Huffman symbol out of range".to_string());
            }
            if l == 16 {
                break;
            }
            code = (code << 1) | self.bit()? as i32;
        }
        Err("a Huffman code the tables do not define".into())
    }
    /// drop the padding bits at a restart, reporting whether they were all 1s
    fn restart(&mut self) -> Result<bool, String> {
        let ones = if self.n == 0 { true } else { (self.acc & ((1 << self.n) - 1)) == (1 << self.n) - 1 };
        let zeros = if self.n == 0 { true } else { (self.acc & ((1 << self.n) - 1)) == 0 };
        self.n = 0;
        if self.pos + 1 >= self.d.len() || self.d[self.pos] != 0xFF || !(0xD0..=0xD7).contains(&self.d[self.pos + 1]) {
            return Err("expected a restart marker and did not find one".into());
        }
        self.pos += 2;
        if ones {
            Ok(true)
        } else if zeros {
            Ok(false)
        } else {
            Err("restart padding is neither all ones nor all zeros".into())
        }
    }
}

#[inline]
fn extend(v: u32, t: u32) -> i32 {
    if t == 0 {
        0
    } else if v < (1 << (t - 1)) {
        v as i32 - (1 << t) + 1
    } else {
        v as i32
    }
}

/// the block index (into `coef[ci]`) of data unit (bx, by) of component ci
#[inline]
pub fn block_at(c: &Comp, bx: usize, by: usize) -> usize {
    (by * c.bw + bx) * 64
}

/// walk the coded data units in scan order, calling `f(ci, bx, by)`
fn for_each_unit<F: FnMut(usize, usize, usize, usize)>(j: &Jpeg, mut f: F) {
    let mut unit = 0usize;
    if j.interleaved {
        for my in 0..j.mcuy {
            for mx in 0..j.mcux {
                for (ci, c) in j.comps.iter().enumerate() {
                    for vv in 0..c.v {
                        for hh in 0..c.h {
                            f(unit, ci, mx * c.h + hh, my * c.v + vv);
                        }
                    }
                }
                unit += 1;
            }
        }
    } else {
        let c = &j.comps[0];
        for by in 0..c.bh {
            for bx in 0..c.bw {
                f(unit, 0, bx, by);
                unit += 1;
            }
        }
    }
}

/// the number of restart-delimited units in the scan (MCUs, or data units when
/// the scan is not interleaved)
fn unit_count(j: &Jpeg) -> usize {
    if j.interleaved {
        j.mcux * j.mcuy
    } else {
        j.comps[0].bw * j.comps[0].bh
    }
}

// ---------------------------------------------------------------- peel

/// peel `src`: parse the skeleton, Huffman-decode every data unit into the
/// coefficient planes. Errors carry the reason, and the caller keeps the bytes.
pub fn peel(src: &[u8]) -> Result<Jpeg, String> {
    let p = parse_skeleton(src, true)?;
    let mut j = p.j;
    let data = &src[p.prefix_end..p.scan_end];
    let mut rd = Reader { d: data, pos: 0, acc: 0, n: 0 };
    let ncomp = j.comps.len();
    let mut pred = vec![0i32; ncomp];
    let mut pad_ones: Option<bool> = None;
    let units = unit_count(&j);
    let dri = j.dri;

    // the walk, inlined so the coefficient planes can be written through
    let mut order: Vec<(usize, usize, usize)> = Vec::with_capacity(j.nblocks());
    for_each_unit(&j, |_, ci, bx, by| order.push((ci, bx, by)));
    let per_unit = if j.interleaved { j.comps.iter().map(|c| c.h * c.v).sum::<usize>() } else { 1 };

    let mut oi = 0usize;
    for unit in 0..units {
        if dri != 0 && unit != 0 && unit % dri == 0 {
            let ones = rd.restart()?;
            match pad_ones {
                None => pad_ones = Some(ones),
                Some(p0) if p0 != ones => return Err("the encoder padded some restarts with ones and some with zeros".into()),
                _ => {}
            }
            pred.iter_mut().for_each(|p| *p = 0);
        }
        for _ in 0..per_unit {
            let (ci, bx, by) = order[oi];
            oi += 1;
            let c = &j.comps[ci];
            let (td, ta) = (c.td, c.ta);
            let base = block_at(c, bx, by);
            // DC
            let t = rd.huff(&j.dc[td])? as u32;
            if t > 15 {
                return Err("DC category above 15".into());
            }
            let diff = extend(rd.bits(t)?, t);
            pred[ci] += diff;
            let dcv = pred[ci];
            if !(-32768..=32767).contains(&dcv) {
                return Err("DC prediction leaves the 16-bit coefficient range".into());
            }
            j.coef[ci][base] = dcv as i16;
            // AC
            let mut k = 1usize;
            while k < 64 {
                let rs = rd.huff(&j.ac[ta])?;
                let (r, s) = ((rs >> 4) as usize, (rs & 15) as u32);
                if s == 0 {
                    if r == 15 {
                        k += 16;
                        continue;
                    }
                    break; // EOB
                }
                k += r;
                if k > 63 {
                    return Err("AC run walks past coefficient 63".into());
                }
                j.coef[ci][base + k] = extend(rd.bits(s)?, s) as i16;
                k += 1;
            }
        }
    }
    // the tail padding before EOI
    let tail_ones = if rd.n == 0 { true } else { (rd.acc & ((1 << rd.n) - 1)) == (1 << rd.n) - 1 };
    let tail_zeros = if rd.n == 0 { true } else { (rd.acc & ((1 << rd.n) - 1)) == 0 };
    if rd.n != 0 {
        let ones = if tail_ones {
            true
        } else if tail_zeros {
            false
        } else {
            return Err("the tail padding is neither all ones nor all zeros".into());
        };
        match pad_ones {
            None => pad_ones = Some(ones),
            Some(p0) if p0 != ones => return Err("the tail padding disagrees with the restart padding".into()),
            _ => {}
        }
    }
    if rd.pos != data.len() {
        return Err(format!("the scan has {} bytes the decode did not consume", data.len() - rd.pos));
    }
    j.pad_ones = pad_ones.unwrap_or(true);
    Ok(j)
}

// ---------------------------------------------------------------- respell

struct Writer {
    out: Vec<u8>,
    acc: u32,
    n: u32,
}
impl Writer {
    #[inline]
    fn put(&mut self, len: u32, code: u32) {
        for k in (0..len).rev() {
            self.acc = (self.acc << 1) | ((code >> k) & 1);
            self.n += 1;
            if self.n == 8 {
                let b = self.acc as u8;
                self.out.push(b);
                if b == 0xFF {
                    self.out.push(0);
                }
                self.acc = 0;
                self.n = 0;
            }
        }
    }
    #[inline]
    fn pad(&mut self, ones: bool) {
        if self.n != 0 {
            let k = 8 - self.n;
            self.put(k, if ones { (1u32 << k) - 1 } else { 0 });
        }
    }
}
#[inline]
fn size_of(v: i32) -> u32 {
    let mut a = v.unsigned_abs();
    let mut s = 0u32;
    while a != 0 {
        s += 1;
        a >>= 1;
    }
    s
}
#[inline]
fn bits_of(v: i32, s: u32) -> u32 {
    if v >= 0 {
        v as u32
    } else {
        (v + (1 << s) - 1) as u32
    }
}

/// re-spell the peel: prefix + the re-encoded entropy segment + suffix. THE LAW
/// asks the caller to compare this against the original bytes before use.
pub fn respell(j: &Jpeg) -> Result<Vec<u8>, String> {
    let mut wr = Writer { out: Vec::with_capacity(j.prefix.len() + j.suffix.len() + 64), acc: 0, n: 0 };
    let ncomp = j.comps.len();
    let mut pred = vec![0i32; ncomp];
    let units = unit_count(j);
    let dri = j.dri;
    let mut order: Vec<(usize, usize, usize)> = Vec::with_capacity(j.nblocks());
    for_each_unit(j, |_, ci, bx, by| order.push((ci, bx, by)));
    let per_unit = if j.interleaved { j.comps.iter().map(|c| c.h * c.v).sum::<usize>() } else { 1 };
    let mut oi = 0usize;
    let mut rst = 0u8;
    for unit in 0..units {
        if dri != 0 && unit != 0 && unit % dri == 0 {
            wr.pad(j.pad_ones);
            wr.out.push(0xFF);
            wr.out.push(0xD0 + (rst & 7));
            rst = rst.wrapping_add(1);
            pred.iter_mut().for_each(|p| *p = 0);
        }
        for _ in 0..per_unit {
            let (ci, bx, by) = order[oi];
            oi += 1;
            let c = &j.comps[ci];
            let base = block_at(c, bx, by);
            let blk = &j.coef[ci][base..base + 64];
            let diff = blk[0] as i32 - pred[ci];
            pred[ci] = blk[0] as i32;
            let t = size_of(diff);
            let (l, code) = j.dc[c.td].enc[t as usize];
            if l == 0 {
                return Err(format!("the DC table cannot spell category {}", t));
            }
            wr.put(l as u32, code as u32);
            if t != 0 {
                wr.put(t, bits_of(diff, t));
            }
            let mut last = 0usize;
            for k in (1..64).rev() {
                if blk[k] != 0 {
                    last = k;
                    break;
                }
            }
            let mut k = 1usize;
            let mut run = 0usize;
            while k <= last {
                if blk[k] == 0 {
                    run += 1;
                    k += 1;
                    continue;
                }
                while run > 15 {
                    let (l, code) = j.ac[c.ta].enc[0xF0];
                    if l == 0 {
                        return Err("the AC table cannot spell ZRL".into());
                    }
                    wr.put(l as u32, code as u32);
                    run -= 16;
                }
                let s = size_of(blk[k] as i32);
                let sym = (run << 4) | s as usize;
                let (l, code) = j.ac[c.ta].enc[sym];
                if l == 0 {
                    return Err(format!("the AC table cannot spell run/size {:02x}", sym));
                }
                wr.put(l as u32, code as u32);
                wr.put(s, bits_of(blk[k] as i32, s));
                run = 0;
                k += 1;
            }
            if last < 63 {
                let (l, code) = j.ac[c.ta].enc[0x00];
                if l == 0 {
                    return Err("the AC table cannot spell EOB".into());
                }
                wr.put(l as u32, code as u32);
            }
        }
    }
    wr.pad(j.pad_ones);
    let mut out = Vec::with_capacity(j.prefix.len() + wr.out.len() + j.suffix.len());
    out.extend_from_slice(&j.prefix);
    out.extend_from_slice(&wr.out);
    out.extend_from_slice(&j.suffix);
    Ok(out)
}

// ---------------------------------------------------------------- the recipe

/// the recipe: everything needed to re-spell that is NOT a coefficient. The
/// marker skeleton rides verbatim -- what the STREAM said, never what a library
/// would have said (the determinism trap of the charter plan).
pub fn recipe_bytes(j: &Jpeg) -> Vec<u8> {
    let mut r = Vec::with_capacity(9 + j.prefix.len() + j.suffix.len());
    r.push(1u8); // peel format version
    r.push(j.pad_ones as u8);
    r.extend_from_slice(&(j.prefix.len() as u32).to_le_bytes());
    r.extend_from_slice(&(j.suffix.len() as u32).to_le_bytes());
    r.extend_from_slice(&j.prefix);
    r.extend_from_slice(&j.suffix);
    r
}

/// the recipe read back: the same skeleton, re-parsed by the same code, with
/// empty coefficient planes for the model to fill.
pub fn from_recipe(r: &[u8]) -> Result<Jpeg, String> {
    if r.len() < 10 || r[0] != 1 {
        return Err("peel recipe: bad header".into());
    }
    let pad_ones = r[1] != 0;
    let pl = u32::from_le_bytes(r[2..6].try_into().unwrap()) as usize;
    let sl = u32::from_le_bytes(r[6..10].try_into().unwrap()) as usize;
    if 10 + pl + sl != r.len() {
        return Err("peel recipe: length mismatch".into());
    }
    let prefix = &r[10..10 + pl];
    let suffix = &r[10 + pl..];
    let p = parse_skeleton(prefix, false)?;
    let mut j = p.j;
    j.suffix = suffix.to_vec();
    j.pad_ones = pad_ones;
    Ok(j)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the peel is a bijection on this repo's own JPEG or it does not ship
    #[test]
    fn corpus_jpeg_round_trips_byte_exact() {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wallpaper.jpg");
        let src = std::fs::read(p).expect("corpus-real/wallpaper.jpg present");
        let j = peel(&src).expect("wallpaper.jpg peels");
        let back = respell(&j).expect("respell");
        assert_eq!(back.len(), src.len(), "respelled length");
        assert!(back == src, "respelled bytes differ from the original");
        // and the recipe alone rebuilds the same skeleton
        let r = recipe_bytes(&j);
        let mut j2 = from_recipe(&r).expect("recipe re-parses");
        assert_eq!(j2.nblocks(), j.nblocks());
        assert_eq!(j2.mcux, j.mcux);
        assert_eq!(j2.dri, j.dri);
        j2.coef = j.coef.clone();
        assert!(respell(&j2).expect("respell from recipe") == src);
    }

    /// refuse, do not guess: a progressive frame keeps its bytes and says why
    #[test]
    fn progressive_is_refused_with_a_reason() {
        let mut b = vec![0xFF, 0xD8];
        b.extend_from_slice(&[0xFF, 0xC2, 0x00, 0x11, 8, 0, 16, 0, 16, 3, 1, 0x11, 0, 2, 0x11, 1, 3, 0x11, 1]);
        b.extend_from_slice(&[0xFF, 0xD9]);
        let e = match peel(&b) {
            Ok(_) => panic!("a progressive frame must be refused, not peeled"),
            Err(e) => e,
        };
        assert!(e.contains("progressive"), "{}", e);
    }
}
