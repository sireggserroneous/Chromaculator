//! sites.rs -- M3b's two INSTRUMENTS, S1b and S1c. Neither is an arm; both are
//! measurements that decide whether an arm gets built at all. If the readings
//! say no, this file is DELETED with its numbers printed (the house rule:
//! losers are deleted, never shelved).
//!
//! S1b, the gcd (`wub.html:322-326`, `const gcd = (a, b) => b ? gcd(b, a % b) : a`
//! and the reduction that makes two rates coprime). Whole-file gcd is already
//! settled at 1 on all 23 rows at 8/16/32 bits. What was never measured is the
//! gcd of a BLOCK, and the gcd of a DELTA stream -- which is where quantised
//! data actually carries a common factor.
//!
//! S1c, the bit period. The earlier probe used order-0 entropy per width plus
//! bit autocorrelation over whole files. It cannot see structure living at
//! order 1 in a 12-bit symbol space, and averaging a whole file hides a
//! container that packs differently per section. This one measures an ORDER-1
//! SEQUENTIAL CODE LENGTH per candidate width, per 1 MB region.
//!
//! Why sequential and not empirical: the empirical order-1 entropy of 2^w
//! symbols over a 1 MB region is biased to near zero for w >= 12 (there are
//! more (context, symbol) cells than samples), so it would manufacture a win at
//! every wide width. The sequential estimator PAYS for its own alphabet -- the
//! first symbol in a fresh context costs log2(A) bits -- and is an honest code
//! length, not a fit.

use crate::filter;

// ---------------------------------------------------------------- S1b, the gcd

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// the gcd of one block read at `w` bytes per value, little-endian, and how
/// many DISTINCT values it holds (capped at 16 -- the question is only whether
/// the block is a constant run). None for a degenerate block: all zeros, whose
/// gcd of 0 names no factor.
fn block_gcd(blk: &[u8], w: usize) -> Option<(u64, usize)> {
    let mut g = 0u64;
    let mut i = 0usize;
    let mut seen: Vec<u64> = Vec::new();
    while i + w <= blk.len() {
        let mut v = 0u64;
        for k in 0..w {
            v |= (blk[i + k] as u64) << (8 * k);
        }
        g = gcd(g, v);
        if g == 1 {
            return Some((1, 0));
        }
        if seen.len() < 16 && !seen.contains(&v) {
            seen.push(v);
        }
        i += w;
    }
    if g == 0 {
        None
    } else {
        Some((g, seen.len()))
    }
}

const BLK: usize = 64 * 1024;

/// one (filter, width) reading of a file: how many 64 KB blocks carry a factor,
/// and what that factor would be worth against what it would cost.
struct GcdRow {
    label: String,
    width: usize,
    blocks: usize,
    with_factor: usize,
    /// of those, the ones whose factor is NOT a power of two. A power-of-two
    /// gcd says only that the low k bits are constant zero, and every bitwise
    /// model in this house already codes a constant bit at ~0 -- so those
    /// blocks are not a finding, they are the model working. An odd factor
    /// (10, 257, 8191) is arithmetic, and that is the only kind a divider
    /// could ever sell.
    odd_factor: usize,
    /// and of THOSE, the ones that are not simply a constant run: a block of
    /// one repeated value has a gcd equal to that value and is already free
    real: usize,
    all_zero: usize,
    free_bits: f64,
    real_free_bits: f64,
    biggest: u64,
    biggest_real: u64,
}

fn gcd_rows(label: &str, data: &[u8]) -> Vec<GcdRow> {
    let mut out = Vec::new();
    for width in [1usize, 2, 4] {
        let mut r = GcdRow {
            label: label.to_string(),
            width,
            blocks: 0,
            with_factor: 0,
            odd_factor: 0,
            real: 0,
            all_zero: 0,
            free_bits: 0.0,
            real_free_bits: 0.0,
            biggest: 1,
            biggest_real: 1,
        };
        let mut at = 0usize;
        while at < data.len() {
            let e = (at + BLK).min(data.len());
            let blk = &data[at..e];
            at = e;
            if blk.len() < width {
                continue;
            }
            r.blocks += 1;
            match block_gcd(blk, width) {
                None => r.all_zero += 1,
                Some((1, _)) => {}
                Some((g, distinct)) => {
                    let n = (blk.len() / width) as f64;
                    r.with_factor += 1;
                    r.biggest = r.biggest.max(g);
                    r.free_bits += n * (g as f64).log2();
                    if !g.is_power_of_two() {
                        r.odd_factor += 1;
                        if distinct > 2 {
                            // the ARITHMETIC half of the factor: the part a bit
                            // mask could not already have taken
                            let odd = g >> g.trailing_zeros();
                            r.real += 1;
                            r.biggest_real = r.biggest_real.max(g);
                            r.real_free_bits += n * (odd as f64).log2();
                        }
                    }
                }
            }
        }
        out.push(r);
    }
    out
}

/// S1b: every 64 KB block of the file, at 8/16/32 bits, before and after each
/// filter the tree already carries. Prints a line per reading that finds
/// anything and one summary line always.
pub fn gcd_probe(name: &str, src: &[u8]) {
    let mut forms: Vec<(String, Vec<u8>)> = vec![("plain".to_string(), src.to_vec())];
    let mut ids: Vec<(u8, u32)> = vec![(1, 0), (2, 0), (3, 0), (4, 0), (filter::FILTER_W16, 1), (filter::FILTER_W16, 2), (filter::FILTER_W16O2, 2), (filter::FILTER_W16BE, 1)];
    for c in filter::nominate(src) {
        if !ids.contains(&(c.id, c.param)) {
            ids.push((c.id, c.param));
        }
    }
    for (id, param) in ids {
        let f = filter::apply(src, id, param);
        if f.len() == src.len() {
            forms.push((format!("filter {}:{}", id, param), f));
        }
    }
    let mut tot_blocks = 0usize;
    let mut tot_factor = 0usize;
    let mut tot_real = 0usize;
    let mut best: Option<(String, usize, f64, usize)> = None;
    for (label, data) in &forms {
        for r in gcd_rows(label, data) {
            tot_blocks += r.blocks;
            tot_factor += r.with_factor;
            tot_real += r.real;
            if r.with_factor > 0 {
                // the recipe cost, called honestly: one u32 factor per block
                // that carries one, plus a bit per block to say which
                let cost_bits = r.real as f64 * 32.0 + r.blocks as f64;
                println!(
                    "  {:>14} @{:>2} bit: {:>5}/{:<5} carry a factor (biggest {}, {:.0} raw free bits); {} ODD, {} of those not a constant run (biggest {}) -> {:.0} ARITHMETIC free bits vs {:.0} of recipe -> {}",
                    r.label,
                    r.width * 8,
                    r.with_factor,
                    r.blocks,
                    r.biggest,
                    r.free_bits,
                    r.odd_factor,
                    r.real,
                    r.biggest_real,
                    r.real_free_bits,
                    cost_bits,
                    if r.real_free_bits > cost_bits { "WORTH A TRIAL" } else { "not worth its recipe" }
                );
                if best.as_ref().is_none_or(|(_, _, fb, _)| r.real_free_bits > *fb) {
                    best = Some((r.label.clone(), r.width * 8, r.real_free_bits, r.real));
                }
            }
            if r.all_zero > 0 && r.width == 1 {
                println!("  {:>14} @ 8 bit: {} of {} blocks are all zero (no factor: gcd 0 names nothing)", r.label, r.all_zero, r.blocks);
            }
        }
    }
    let pct = 100.0 * tot_factor as f64 / tot_blocks.max(1) as f64;
    let rpct = 100.0 * tot_real as f64 / tot_blocks.max(1) as f64;
    println!(
        "{}: {} readings, {} block-readings, {} carry gcd > 1 ({:.4}%), {} of them ARITHMETIC ({:.4}%){}",
        name,
        forms.len() * 3,
        tot_blocks,
        tot_factor,
        pct,
        tot_real,
        rpct,
        match best {
            Some((l, w, fb, n)) => format!("; best {} @{} bit, {} blocks, {:.0} arithmetic free bits", l, w, n, fb),
            None => "; NO factor anywhere".to_string(),
        }
    );
}

// ------------------------------------------------------- S1c, the bit period

/// a small open-addressing count table. The default hasher costs more than the
/// counting does at this volume, and the keys are already well spread.
struct Counts {
    /// the key + 1, so 0 is the empty marker and no real key can be mistaken
    /// for one (a u32 key of u32::MAX is a real (context, symbol) pair at
    /// width 16, and an in-band sentinel would silently lose its count)
    keys: Vec<u64>,
    vals: Vec<u32>,
    mask: usize,
}
impl Counts {
    fn with_capacity(n: usize) -> Counts {
        let mut cap = 16usize;
        while cap < n * 2 {
            cap <<= 1;
        }
        Counts { keys: vec![0u64; cap], vals: vec![0u32; cap], mask: cap - 1 }
    }
    #[inline]
    fn bump(&mut self, key: u32) -> u32 {
        let k = key as u64 + 1;
        let mut i = (key.wrapping_mul(0x9E37_79B9) >> 8) as usize & self.mask;
        loop {
            if self.keys[i] == k {
                let was = self.vals[i];
                self.vals[i] = was + 1;
                return was;
            }
            if self.keys[i] == 0 {
                self.keys[i] = k;
                self.vals[i] = 1;
                return 0;
            }
            i = (i + 1) & self.mask;
        }
    }
}

/// the symbols of a region read `w` bits at a time, most significant bit first
fn symbols(region: &[u8], w: u32) -> Vec<u32> {
    let total = region.len() as u64 * 8;
    let n = (total / w as u64) as usize;
    let mut out = Vec::with_capacity(n);
    let mut acc = 0u64;
    let mut have = 0u32;
    let mut at = 0usize;
    for _ in 0..n {
        while have < w {
            acc = (acc << 8) | region[at] as u64;
            at += 1;
            have += 8;
        }
        out.push(((acc >> (have - w)) & ((1u64 << w) - 1)) as u32);
        have -= w;
    }
    out
}

/// the ORDER-1 SEQUENTIAL code length of a region read at `w` bits, in bits.
/// Laplace-style with alpha = 1/2 over an alphabet of 2^w: the estimator pays
/// for its own alphabet, so a wide width cannot win by overfitting.
fn order1_bits(region: &[u8], w: u32) -> f64 {
    let syms = symbols(region, w);
    if syms.is_empty() {
        return 0.0;
    }
    let a = 1u64 << w;
    let alpha = 0.5f64;
    let a_alpha = a as f64 * alpha;
    let mut ctx_n = vec![0u32; a as usize];
    let mut pairs = Counts::with_capacity(syms.len().min((a * a) as usize));
    let mut bits = 0f64;
    let mut ctx = 0u32; // the start context, like any other
    for &s in &syms {
        let key = ctx.wrapping_mul(a as u32).wrapping_add(s);
        let n_pair = pairs.bump(key) as f64;
        let n_ctx = ctx_n[ctx as usize] as f64;
        bits += ((n_ctx + a_alpha) / (n_pair + alpha)).log2();
        ctx_n[ctx as usize] += 1;
        ctx = s;
    }
    bits
}

/// THE CONTROL. The naive order-1 reading gives a 12-bit symbol a 12-bit
/// context and an 8-bit symbol an 8-bit one, so a wide width can win purely on
/// having seen more history -- nothing to do with where the packing boundary
/// is. This reading pins the context at the previous 16 bits for EVERY width,
/// so the only thing that varies is the symbol boundary. At width 8 that makes
/// the baseline an order-2 byte model, which is the fair opponent.
fn order1_bits_ctx16(region: &[u8], w: u32) -> f64 {
    let syms = symbols(region, w);
    if syms.is_empty() {
        return 0.0;
    }
    let a = 1u64 << w;
    let alpha = 0.5f64;
    let a_alpha = a as f64 * alpha;
    let mut ctx_n = vec![0u32; 1 << 16];
    let mut pairs = Counts::with_capacity(syms.len());
    let mut bits = 0f64;
    let mut hist = 0u32; // the previous 16 bits, most recent in the low bits
    for &sym in &syms {
        let ctx = hist & 0xFFFF;
        let key = ctx.wrapping_mul(a as u32).wrapping_add(sym);
        let n_pair = pairs.bump(key) as f64;
        let n_ctx = ctx_n[ctx as usize] as f64;
        bits += ((n_ctx + a_alpha) / (n_pair + alpha)).log2();
        ctx_n[ctx as usize] += 1;
        hist = ((hist << w) | sym) & 0xFFFF;
    }
    bits
}

const REGION: usize = 1 << 20;
/// at most this many regions per file, evenly spaced: a 183 MB row does not
/// need 183 readings to answer "does any region prefer a width that is not 8"
const MAX_REGIONS: usize = 16;

/// S1c: order-1 sequential code length per candidate width, per 1 MB region.
/// Prints the widths that beat the byte reading, and one summary line.
pub fn bit_probe(name: &str, src: &[u8]) {
    let nreg_total = src.len().div_ceil(REGION).max(1);
    let step = nreg_total.div_ceil(MAX_REGIONS);
    let mut worst: Option<(usize, u32, f64)> = None; // region, width, gain pt
    let mut regions = 0usize;
    let mut over1 = 0usize;
    let mut r = 0usize;
    while r < nreg_total {
        let at = r * REGION;
        let e = (at + REGION).min(src.len());
        if e <= at + 4096 {
            r += step;
            continue;
        }
        let region = &src[at..e];
        regions += 1;
        let naive = |w: u32| order1_bits(region, w) / region.len() as f64;
        let ctl = |w: u32| order1_bits_ctx16(region, w) / region.len() as f64;
        let base_n = naive(8);
        let base_c = ctl(8);
        let mut line = String::new();
        let mut best_here: Option<(u32, f64, f64)> = None;
        for w in 1..=16u32 {
            if w == 8 {
                continue;
            }
            let gn = 100.0 * (base_n - naive(w)) / base_n;
            let gc = 100.0 * (base_c - ctl(w)) / base_c;
            if gn > 0.0 || gc > 0.0 {
                line.push_str(&format!(" {}:{:+.2}/{:+.2}pt", w, gn, gc));
            }
            if !w.is_multiple_of(8) && best_here.is_none_or(|(_, _, bg)| gc > bg) {
                best_here = Some((w, gn, gc));
            }
        }
        if let Some((w, _, gc)) = best_here {
            if gc > 1.0 {
                over1 += 1;
            }
            if worst.is_none_or(|(_, _, wg)| gc > wg) {
                worst = Some((r, w, gc));
            }
        }
        if !line.is_empty() {
            println!(
                "  region {:>4} ({} B, byte reading {:.4} naive / {:.4} controlled bits/byte): widths that beat it (naive/controlled):{}",
                r,
                region.len(),
                base_n,
                base_c,
                line
            );
        }
        r += step;
    }
    match worst {
        Some((reg, w, g)) => println!(
            "{}: {} of {} regions read, {} with a non-multiple-of-8 width beating the byte reading by more than 1 pt UNDER THE CONTROL; best is width {} in region {} at {:+.3} pt",
            name, regions, nreg_total, over1, w, reg, g
        ),
        None => println!("{}: {} of {} regions read, nothing to report", name, regions, nreg_total),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xs(state: &mut u64) -> u8 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        (*state & 0xff) as u8
    }

    /// S1c's LOAD-BEARING CLAIM, pinned. The whole reason a wide width cannot
    /// win by overfitting is that a symbol in a fresh context costs log2(A) = w
    /// bits. That payment is one expression -- `(n_ctx + a_alpha) / (n_pair +
    /// alpha)` -- and a one-token slip between `alpha` and `a_alpha` makes a
    /// fresh context cost log2(1) = 0 instead, which would hand every wide
    /// width a manufactured win and invert a verdict already written into
    /// PREDICTIONS.md and a commit message.
    #[test]
    fn the_estimator_pays_for_its_own_alphabet() {
        // exact and arithmetic-free: two bytes read as ONE 16-bit symbol, in a
        // fresh context, must cost exactly log2(65536) = 16 bits
        let one = order1_bits(&[0xAB, 0xCD], 16);
        assert!((one - 16.0).abs() < 1e-9, "one novel 16-bit symbol must cost 16 bits, got {}", one);
        let four = order1_bits(&[0x5A], 4);
        // two novel 4-bit symbols: the first in the start context, the second
        // in the context the first created -- 4 bits each
        assert!((four - 8.0).abs() < 1e-9, "two novel 4-bit symbols must cost 8 bits, got {}", four);

        // and the consequence: incompressible bytes must not compress at ANY
        // width. Without the payment the wide widths collapse toward zero.
        let mut st = 0x1489u64;
        let mut r = vec![0u8; 8192];
        for b in r.iter_mut() {
            *b = xs(&mut st);
        }
        for w in [1u32, 4, 8, 12, 16] {
            let per_bit = order1_bits(&r, w) / (r.len() as f64 * 8.0);
            assert!(per_bit > 0.95, "random must not compress at width {}: {:.4} bits per bit", w, per_bit);
        }
    }

    /// THE CONTROL, pinned. S1c's verdict rests on the context being the
    /// previous SIXTEEN BITS for every width -- that is what removed the
    /// context-length confound and took wide-width wins from 19 pairs to 1.
    /// On data where one byte of history is ambiguous and two bytes are
    /// decisive, the control must beat the naive reading; if it silently saw
    /// only its own symbol, the two would agree.
    #[test]
    fn the_control_really_sees_sixteen_bits() {
        let mut d = Vec::new();
        for _ in 0..8192 {
            d.extend_from_slice(b"ABAC"); // after 'A' the next is B or C; after "CA" it is B
        }
        let naive = order1_bits(&d, 8) / d.len() as f64;
        let control = order1_bits_ctx16(&d, 8) / d.len() as f64;
        assert!(
            control < naive * 0.5,
            "the 16-bit control must beat the 8-bit reading on order-2 data: naive {:.4}, control {:.4}",
            naive,
            control
        );
    }

    /// S1b's finding was that a POWER-OF-TWO gcd is not a finding (it says the
    /// low bits are constant zero, which every bitwise model already codes at
    /// ~0) and neither is a CONSTANT RUN (its gcd is just its value). Both
    /// discriminators live in `block_gcd`'s second return value, and both are
    /// pinned here -- they are what took real-test.bmp from 24.32% of blocks to
    /// 3.86%, and the corpus from 0.71% to 0.113%.
    #[test]
    fn the_gcd_split_tells_a_factor_from_a_constant_run() {
        // a genuine arithmetic factor: every value a multiple of 5
        let odd: Vec<u8> = (0..4096u32).map(|i| ((i % 51) * 5) as u8).collect();
        let (g, distinct) = block_gcd(&odd, 1).expect("multiples of 5 have a gcd");
        assert_eq!(g, 5, "the arithmetic factor itself");
        assert!(distinct > 2, "and it is not a constant run: {} distinct", distinct);

        // a constant run: the gcd is the value, and it is NOT a finding
        let (g2, d2) = block_gcd(&vec![200u8; 4096], 1).expect("a constant run has a gcd");
        assert_eq!(g2, 200);
        assert_eq!(d2, 1, "a constant run has ONE distinct value");

        // the degenerate cases refuse rather than invent a factor
        assert!(block_gcd(&[0u8; 4096], 1).is_none(), "all zeros names no factor");
        assert!(block_gcd(&[7u8, 3], 4).is_none(), "a block shorter than its width names no factor");
        assert_eq!(block_gcd(&[3u8, 5, 7, 11], 1), Some((1, 0)), "coprime values name no factor");
    }
}
