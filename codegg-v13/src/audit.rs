//! audit.rs -- the geometry audit, v4 (the Remainder). The site ships a
//! checked table ("298,545 checks, all passing"); this is that discipline
//! pointed at armor v4. Nothing here restates the shipping code's OPINIONS:
//! the wounds slide over the REAL square offsets, the claims come from the
//! REAL rib_search, the arithmetic column is recomputed INDEPENDENTLY, and
//! the field is checked against its own axioms.
//!
//! Sections: (a) dead(blk) <= t for every tier and every --survive value, by
//! sliding a contiguous wound over real containers; (b) the residue modulus:
//! ord_65519(2) walked, -1 never reached, +-2^k pairwise distinct over the
//! square (the 4096 tier's 4,094-byte caveat printed); (c) the field: alpha
//! of order 65,535, inverses, the generator's roots; (d) the systematic
//! property + the column arithmetic to the byte; (e) erasure round-trips per
//! pattern class, blind and named; (f) errors located by the syndromes alone
//! (residues distrusted) for e <= floor(t/2) and the collaborative rung;
//! (g) t+1 dead squares REFUSE, and beyond-capacity noise never yields wrong
//! data; (h) the miscorrection bound. Counts printed; `--full` widens (d)-(g).

use crate::armor::{
    armor, dead_slots, dearmor, dearmor_with, fnv64, generator, geom, gf, offsets, residue, rib_policy, rib_search, rib_search_with,
    square_len, square_off, t_for, CtMode, Extras, Geom, Rs, FIELD_ORDER, HDR, MODULUS, NMAX, SURVIVE_DEFAULT, TIERS,
};
/// the three placements; placement none asks one parity square more
const MODES: [CtMode; 3] = [CtMode::Triple, CtMode::InCodeword, CtMode::Absent];

fn dummy_ex() -> Extras {
    Extras { orig_len: 0, orig_fnv: 0, model: 0, filter_id: 0, filter_param: 0 }
}
fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
fn xbytes(n: usize, seed: u64) -> Vec<u8> {
    let mut st = seed | 1;
    (0..n).map(|_| xorshift(&mut st) as u8).collect()
}
/// the column arithmetic, recomputed here without the armor's Geom
fn column_total(len: usize, blk: usize, t: usize, mode: CtMode) -> (usize, usize) {
    let s = len.div_ceil(blk);
    let pad = s * blk - len;
    let (c, m) = match mode {
        CtMode::Triple => (0, 2 * (s + t)),
        CtMode::InCodeword => {
            let c = (2 * s).div_ceil(blk);
            (c, 2 * (t + c))
        }
        CtMode::Absent => (0, 0),
    };
    let n = s + t + c;
    (3 * (HDR + m + 4) + n * blk - pad, n)
}
struct Counter {
    checks: u64,
    fails: u64,
    shown: u64,
}
impl Counter {
    fn check(&mut self, ok: bool, what: impl Fn() -> String) {
        self.checks += 1;
        if !ok {
            self.fails += 1;
            self.shown += 1;
            if self.shown <= 25 {
                println!("  FAIL {}", what());
            }
        }
    }
    /// a new section: up to 25 failures printed per section
    fn section(&mut self) {
        self.shown = 0;
    }
}
/// squares (and sites) a contiguous wound [a, a+w) touches
fn squares_hit(g: &Geom, a: usize, w: usize) -> usize {
    let (lo, hi) = (a, a + w);
    (0..g.n)
        .filter(|&j| {
            let o = square_off(g, j);
            o < hi && o + square_len(g, j) > lo
        })
        .count()
}
fn sites_alive(g: &Geom, a: usize, w: usize) -> (usize, usize) {
    let off = offsets(g);
    let (lo, hi) = (a, a + w);
    let alive = |o: usize, l: usize| !(o < hi && o + l > lo);
    let h = [off.h0, off.h1, off.h2].iter().filter(|&&o| alive(o, HDR)).count();
    let m = [off.m0, off.m1, off.m2].iter().filter(|&&o| alive(o, g.msize)).count();
    (h, m)
}
/// wound a copy of the container: the listed squares (whole) get noise
fn kill_squares(cont: &[u8], g: &Geom, squares: &[usize], seed: u64) -> (Vec<u8>, Vec<(usize, usize)>) {
    let mut hurt = cont.to_vec();
    let mut st = seed | 1;
    let mut wounds = Vec::new();
    for &j in squares {
        let o = square_off(g, j);
        let l = square_len(g, j);
        for b in &mut hurt[o..o + l] {
            *b = xorshift(&mut st) as u8;
        }
        // a killed square must CHANGE: the short data square can be a single
        // byte, and noise equal to the original (1/256) would leave it alive
        // and the tally one short of the pattern (found by --full at len 4097)
        if hurt[o..o + l] == cont[o..o + l] {
            hurt[o] ^= 0x5A;
        }
        wounds.push((o, l));
    }
    (hurt, wounds)
}
fn pick_distinct(n: usize, k: usize, st: &mut u64) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    while v.len() < k.min(n) {
        let x = (xorshift(st) % n as u64) as usize;
        if !v.contains(&x) {
            v.push(x);
        }
    }
    v
}

pub fn run(full: bool) -> bool {
    let mut c = Counter { checks: 0, fails: 0, shown: 0 };
    let t0 = std::time::Instant::now();
    println!("audit v4 -- the Remainder{}", if full { " (--full)" } else { "" });

    // ---------- (a) dead(blk) <= t: every tier, every --survive ----------
    let survives: &[usize] = &[4096, 8192, 16384, 65536];
    for &blk in &TIERS {
        for &sv in survives {
            let t = t_for(blk, sv);
            let dead = sv.div_ceil(blk) + 1;
            c.check(t >= dead && t == dead, || format!("(a) t_for({blk},{sv}) = {t}, dead = {dead}"));
        }
        c.check(dead_slots(blk) == t_for(blk, SURVIVE_DEFAULT), || format!("(a) dead_slots({blk}) != t_for default"));
    }
    // slide a contiguous `survive`-byte wound over REAL containers: squares
    // hit <= t, and at least one header + one meta copy stay alive
    let mut windows = 0u64;
    let lens: Vec<usize> = if full {
        (1..=400).map(|k| k * 97 + 3).collect()
    } else {
        vec![1, 100, 4_000, 4_096, 4_097, 20_000, 65_536, 100_003, 262_144]
    };
    for &len in &lens {
        for &sv in &[4096usize, 16384] {
            for &blk in &TIERS[..4] {
                for mode in MODES {
                    let t = mode.parity_for(blk, sv);
                    let g = geom(len, blk, t, mode, 0, dummy_ex());
                    if g.n > NMAX {
                        continue;
                    }
                    // every distinct wound start matters only at square boundaries;
                    // step by blk/4 plus the exact boundaries around the sites
                    let step = (blk / 4).max(1);
                    let off = offsets(&g);
                    let mut starts: Vec<usize> = (0..g.total.saturating_sub(sv)).step_by(step).collect();
                    for &edge in &[off.h1, off.m2, off.h2, off.h1 + 1, off.h1.saturating_sub(sv - 1), off.m2.saturating_sub(sv - 1)] {
                        if edge + sv <= g.total {
                            starts.push(edge);
                        }
                    }
                    for a in starts {
                        windows += 1;
                        let hit = squares_hit(&g, a, sv);
                        c.check(hit <= t, || format!("(a) len {len} blk {blk} {} survive {sv}: wound at {a} kills {hit} squares > t {t}", mode.name()));
                        let (h, m) = sites_alive(&g, a, sv);
                        c.check(h >= 1 && m >= 1, || format!("(a) len {len} blk {blk} {}: wound at {a} leaves {h} headers, {m} metas", mode.name()));
                    }
                }
            }
        }
    }
    println!("  (a) dead(blk) <= t: {} tiers x {} survive values; {} contiguous wounds slid over real containers", TIERS.len(), survives.len(), windows);

    c.section();
    // ---------- (b) the residue modulus ----------
    let p = MODULUS as u64;
    let mut x = 1u64;
    let mut ord = 0usize;
    let mut minus_one = false;
    loop {
        x = (x * 2) % p;
        ord += 1;
        if x == p - 1 {
            minus_one = true;
        }
        if x == 1 {
            break;
        }
    }
    c.check(ord == 32_759, || format!("(b) ord_65519(2) = {ord}, expected 32,759"));
    c.check(!minus_one, || "(b) -1 reached in the walk: +2^a = -2^b would collide".to_string());
    // +-2^k pairwise distinct over the square's bit span (2-bit certainty)
    let mut is_prime = true;
    let mut d = 2u64;
    while d * d <= p {
        if p.is_multiple_of(d) {
            is_prime = false;
        }
        d += 1;
    }
    c.check(is_prime, || "(b) 65,519 is not prime".to_string());
    for &blk in &TIERS {
        let bits = 8 * blk;
        let mut seen = vec![false; p as usize];
        let mut first_collision: Option<usize> = None;
        let mut v = 1u64;
        for k in 0..bits {
            for val in [v, (p - v) % p] {
                if seen[val as usize] {
                    first_collision.get_or_insert(k);
                }
                seen[val as usize] = true;
            }
            v = (v * 2) % p;
        }
        if blk <= 2048 {
            c.check(first_collision.is_none(), || format!("(b) +-2^k collide within a {blk} B square at bit {}", first_collision.unwrap()));
        } else {
            // the 4096 tier: distinct only within 32,759 bits = 4,094 B (said, not hidden)
            let fc = first_collision.unwrap_or(bits);
            c.check(fc == 32_759, || format!("(b) 4096 tier: first collision at bit {fc}, expected 32,759"));
            println!("  (b) 4096 tier: +-2^k distinct within {} bits = {} B of a square (the printed caveat)", fc, fc / 8);
        }
    }
    // the residue function agrees with a slow big-number reduction
    let mut st = 0x5EEDu64;
    for _ in 0..200 {
        let blk = TIERS[(xorshift(&mut st) % 5) as usize];
        let sq = xbytes(blk, xorshift(&mut st));
        let slow = sq.iter().fold(0u64, |acc, &b| (acc * 256 + b as u64) % p) as u16;
        c.check(residue(&sq) == slow, || "(b) residue() disagrees with the slow reduction".to_string());
    }
    println!("  (b) modulus 65,519 prime; ord(2) = {ord}; -1 unreached; +-2^k distinct over 256..2048 B squares");

    c.section();
    // ---------- (c) the field ----------
    let f = gf();
    let mut a = 1u16;
    let mut order = 0usize;
    loop {
        a = f.mul(a, 2);
        order += 1;
        if a == 1 {
            break;
        }
    }
    c.check(order == FIELD_ORDER, || format!("(c) alpha has order {order}, expected 65,535"));
    for v in 1..=0xFFFFu32 {
        let v = v as u16;
        c.check(f.mul(v, f.inv(v)) == 1, || format!("(c) {v} * inv({v}) != 1"));
    }
    for _ in 0..2000 {
        let (u, v, w) = (xorshift(&mut st) as u16, xorshift(&mut st) as u16, xorshift(&mut st) as u16);
        c.check(f.mul(u, v ^ w) == f.mul(u, v) ^ f.mul(u, w), || "(c) distributivity".to_string());
        c.check(f.mul(u, v) == f.mul(v, u), || "(c) commutativity".to_string());
        c.check(f.mul(f.mul(u, v), w) == f.mul(u, f.mul(v, w)), || "(c) associativity".to_string());
    }
    for t in [1usize, 2, 3, 5, 9, 17, 33, 255] {
        let g = generator(t);
        c.check(g.len() == t + 1 && *g.last().unwrap() == 1, || format!("(c) generator({t}) not monic of degree t"));
        for i in 0..t {
            c.check(f.poly_eval(&g, f.alpha(i)) == 0, || format!("(c) g_{t}(alpha^{i}) != 0"));
        }
        c.check(f.poly_eval(&g, f.alpha(t)) != 0, || format!("(c) g_{t}(alpha^{t}) == 0: a root too many"));
    }
    println!("  (c) GF(2^16) poly 0x1100B: alpha order {order}; 65,535 inverses; generator roots for 8 values of t; table fnv64 {:016x}", f.table_fnv());

    c.section();
    // ---------- (d) the systematic property + the column arithmetic ----------
    let d_lens: Vec<usize> = if full {
        let mut v: Vec<usize> = (1..=60).map(|k| k * 1_777 + 11).collect();
        v.extend([1, 2, 255, 256, 257, 511, 512, 513, 4_095, 4_096, 4_097, 65_535, 65_536, 65_537, 300_001, 1_048_576]);
        v
    } else {
        vec![1, 2, 255, 256, 257, 4_095, 4_096, 4_097, 12_345, 65_536, 65_537, 200_003]
    };
    let mut geoms = 0usize;
    for &len in &d_lens {
        let inner = xbytes(len, len as u64 * 7 + 1);
        for &blk in &TIERS {
            for mode in MODES {
                let t = mode.parity_for(blk, SURVIVE_DEFAULT);
                let (col_total, col_n) = column_total(len, blk, t, mode);
                if col_n > NMAX {
                    continue;
                }
                let g = geom(len, blk, t, mode, fnv64(&inner), dummy_ex());
                c.check(g.total == col_total && g.n == col_n, || format!("(d) geom({len},{blk},{t},{}) total {} n {} vs column {} / {}", mode.name(), g.total, g.n, col_total, col_n));
                if blk >= 2048 && !full && len > 65_536 {
                    continue; // the big squares cost real time; --full does them
                }
                let cont = armor(&inner, blk, t, mode, dummy_ex());
                geoms += 1;
                c.check(cont.len() == col_total, || format!("(d) armor({len},{blk},{t},{}) wrote {} B, column says {}", mode.name(), cont.len(), col_total));
                // data verbatim at the square offsets
                let mut verbatim = true;
                for j in 0..g.s {
                    let o = square_off(&g, j);
                    let l = square_len(&g, j);
                    if cont[o..o + l] != inner[j * blk..j * blk + l] {
                        verbatim = false;
                    }
                }
                c.check(verbatim, || format!("(d) data not verbatim at len {len} blk {blk} {}", mode.name()));
                // syndromes all zero over the received squares
                let mut sq = vec![0u8; g.n * blk];
                for j in 0..g.n {
                    let o = square_off(&g, j);
                    let l = square_len(&g, j);
                    sq[j * blk..j * blk + l].copy_from_slice(&cont[o..o + l]);
                }
                let rs = Rs { g: &g, sq: &sq };
                c.check(rs.all_clean(), || format!("(d) syndromes nonzero on a clean container len {len} blk {blk} {}", mode.name()));
                // the header round-trips the geometry; the clean restore is exact
                match dearmor(&cont, &[], true) {
                    Ok(o) => {
                        c.check(o.hash_ok && o.inner == inner, || format!("(d) clean restore not exact at len {len} blk {blk} {}", mode.name()));
                        c.check(o.g.total == g.total && o.g.n == g.n && o.g.t == t && o.g.mode == mode && o.g.blk == blk, || "(d) header geometry mismatch".to_string());
                        c.check(o.t.clean == g.n && o.t.rebuilt == 0, || format!("(d) clean container tallied {} clean of {}", o.t.clean, g.n));
                    }
                    Err(e) => c.check(false, || format!("(d) clean restore failed: {e}")),
                }
            }
        }
        // the policy's pick is the argmin the column would compute
        let rib = rib_policy(len);
        c.check(rib_search(len, SURVIVE_DEFAULT, None, None).map(|r| r.total) == Ok(rib.total), || "(d) rib_policy != rib_search".to_string());
        let mut best = usize::MAX;
        let mut best_judge = usize::MAX;
        for &blk in &TIERS[..4] {
            for mode in MODES {
                let (tot, n) = column_total(len, blk, mode.parity_for(blk, SURVIVE_DEFAULT), mode);
                if n <= NMAX && tot < best {
                    best = tot;
                }
                if n <= NMAX && mode != CtMode::Absent && tot < best_judge {
                    best_judge = tot;
                }
            }
        }
        c.check(rib.total == best, || format!("(d) rib_search({len}) = {} but the column's argmin is {best}", rib.total));
        let judged = rib_search_with(len, SURVIVE_DEFAULT, None, None, false).map(|r| r.total);
        c.check(judged == Ok(best_judge), || format!("(d) --judge rib_search({len}) = {judged:?} but the residue placements' argmin is {best_judge}"));
        // placement none: the price is flat per tier and total = inner + price exactly;
        // it is the argmin from ~10 KB up to 16,772,352 B (n <= 65,535 at blk 256).
        // Below that a residue placement's meta (6n B over three sites) undercuts the
        // one extra parity square (256 B) and the argmin keeps it -- printed, not hidden
        if (16_384..=16_772_352).contains(&len) {
            c.check(best == len + 4_812, || format!("(d) placement none at {len}: total {best}, expected inner + 4,812"));
        } else if len < 16_384 {
            c.check(best <= len + 4_812, || format!("(d) tiny inner {len}: the argmin {best} exceeds placement none's {}", len + 4_812));
        }
    }
    println!("  (d) systematic + column arithmetic: {} lengths x {} tiers x 3 CT placements; {} containers built, restored clean; the argmin and the --judge argmin match the column", d_lens.len(), TIERS.len(), geoms);

    c.section();
    // ---------- (e) erasure round-trips per pattern class ----------
    struct Case {
        len: usize,
        blk: usize,
        t: usize,
        mode: CtMode,
    }
    let mut cases: Vec<Case> = Vec::new();
    let e_lens: Vec<usize> = if full { vec![1, 700, 4_097, 33_333, 70_001, 262_145] } else { vec![1, 700, 4_097, 33_333] };
    for &len in &e_lens {
        for &blk in &TIERS[..4] {
            for mode in MODES {
                cases.push(Case { len, blk, t: mode.parity_for(blk, SURVIVE_DEFAULT), mode });
            }
        }
    }
    cases.push(Case { len: 5_000, blk: 4096, t: 2, mode: CtMode::Triple });
    cases.push(Case { len: 100_000, blk: 4096, t: 2, mode: CtMode::InCodeword });
    cases.push(Case { len: 20_000, blk: 256, t: 40, mode: CtMode::InCodeword }); // --survive 9,984
    cases.push(Case { len: 20_000, blk: 512, t: 1, mode: CtMode::Triple }); // --parity 1
    cases.push(Case { len: 20_000, blk: 512, t: 3, mode: CtMode::Absent }); // --parity 3 under none: blind 1 certain, 2 jointly, 3 refuses
    cases.push(Case { len: 100_000, blk: 4096, t: 3, mode: CtMode::Absent });
    let mut e_trials = 0u64;
    let mut e_refused = 0u64;
    let mut e_beyond_exact = 0u64;
    let mut e_rung_c = 0u64;
    for cs in &cases {
        let inner = xbytes(cs.len, cs.len as u64 * 13 + 5);
        let g = geom(cs.len, cs.blk, cs.t, cs.mode, fnv64(&inner), dummy_ex());
        if g.n > NMAX {
            continue;
        }
        let cont = armor(&inner, cs.blk, cs.t, cs.mode, dummy_ex());
        let (n, s, t) = (g.n, g.s, g.t);
        let mut patterns: Vec<(&str, Vec<usize>)> = vec![
            ("head t squares", (0..t.min(n)).collect()),
            ("mid t squares", ((n / 2).saturating_sub(t / 2)..(n / 2).saturating_sub(t / 2) + t).filter(|&j| j < n).collect()),
            ("end t squares", (n - t..n).collect()),
            ("all parity", (s..s + t).filter(|&j| g.is_parity(j)).collect()),
            ("last data + first parity", (s.saturating_sub(t / 2)..s.saturating_sub(t / 2) + t).filter(|&j| j < n).collect()),
        ];
        if g.c > 0 {
            let ct: Vec<usize> = (g.ct_at()..n).take(t).collect();
            patterns.push(("CT squares", ct.clone()));
            // CT dead AND some of the data it judged dead: the blind corner
            let mut mix = ct[..1.min(ct.len())].to_vec();
            let (q0, _) = g.ct_slot(0);
            let mut i = 0;
            while mix.len() < t && i < s {
                if g.ct_slot(i).0 == q0 && !mix.contains(&i) {
                    mix.push(i);
                }
                i += 1;
            }
            patterns.push(("CT + its data", mix));
        }
        let mut st = 0xA11CE_u64 ^ cs.len as u64;
        for r in 0..(if full { 6 } else { 3 }) {
            patterns.push(("scattered t squares", pick_distinct(n, t, &mut st)));
            if r == 0 {
                patterns.push(("scattered t-1 squares", pick_distinct(n, t.saturating_sub(1), &mut st)));
            }
        }
        for (name, squares) in &patterns {
            if squares.is_empty() {
                continue;
            }
            let (hurt, wounds) = kill_squares(&cont, &g, squares, xorshift(&mut st));
            // blind, in-codeword: k dead data squares whose CT square also died
            // are unjudged; the codewords locate them when k < m (collaborative,
            // m = t - |convicted|) or k == m <= 2 (the residue-checked search);
            // k == m >= 3 is the printed corner: an honest REFUSAL is the law
            // placement none: the codewords locate up to t-1 blind squares jointly, t refuses;
            // any placement: dead squares that are all parity/CT leave the data intact (rung C)
            let all_nondata = squares.iter().all(|&j| !g.is_data(j));
            let blind_exact = all_nondata
                || match cs.mode {
                    CtMode::Absent => squares.len() < t,
                    _ => {
                        let k = squares.iter().filter(|&&j| g.is_data(j) && squares.contains(&g.ct_slot(j).0)).count();
                        let m = t - (squares.len() - k);
                        k < m || (k == m && k <= 2)
                    }
                };
            for named in [false, true] {
                e_trials += 1;
                let w: &[(usize, usize)] = if named { &wounds } else { &[] };
                let expect_exact = named || blind_exact;
                match dearmor(&hurt, w, true) {
                    Ok(o) => {
                        let exact = o.hash_ok && o.inner == inner;
                        c.check(exact, || format!("(e) {} blk {} {} len {} [{}]: restored WRONG bytes with hash_ok={}", name, cs.blk, cs.mode.name(), cs.len, if named { "named" } else { "blind" }, o.hash_ok));
                        if o.by_hash {
                            e_rung_c += 1;
                            c.check(all_nondata, || format!("(e) {} blk {} {} len {}: rung C fired with a dead DATA square", name, cs.blk, cs.mode.name(), cs.len));
                        } else {
                            c.check(o.t.rebuilt == squares.len(), || format!("(e) {} blk {} {} len {}: rebuilt {} of {} dead", name, cs.blk, cs.mode.name(), cs.len, o.t.rebuilt, squares.len()));
                        }
                        if !expect_exact {
                            e_beyond_exact += 1;
                        }
                    }
                    Err(e) => {
                        if expect_exact {
                            c.check(false, || format!("(e) {} blk {} {} len {} [{}]: REFUSED ({e})", name, cs.blk, cs.mode.name(), cs.len, if named { "named" } else { "blind" }));
                        } else {
                            e_refused += 1;
                            c.check(e.contains("capacity"), || format!("(e) corner refusal without the number: {e}"));
                        }
                    }
                }
            }
        }
        // a site wound: the whole head site (hdr0 + meta0) plus the first squares
        {
            let off = offsets(&g);
            let w = (off.slot_base + (t.saturating_sub(1)) * cs.blk).min(cont.len());
            let mut hurt = cont.clone();
            for b in &mut hurt[..w] {
                *b = xorshift(&mut st) as u8;
            }
            // the wound straddles the short square, so it can touch t squares
            // (not t-1): within every promise NAMED; blind, placement none
            // locates up to t-1 and must REFUSE at t (never wrong)
            let hit = squares_hit(&g, 0, w);
            e_trials += 2;
            for wounds in [vec![], vec![(0usize, w)]] {
                let blind = wounds.is_empty();
                let expect_exact = !blind || cs.mode != CtMode::Absent || hit < t;
                match dearmor(&hurt, &wounds, true) {
                    Ok(o) => c.check(o.hash_ok && o.inner == inner, || format!("(e) head site wound blk {} {} len {}: wrong bytes", cs.blk, cs.mode.name(), cs.len)),
                    Err(e) => {
                        if expect_exact {
                            c.check(false, || format!("(e) head site wound blk {} {} len {} ({hit} squares hit): REFUSED ({e})", cs.blk, cs.mode.name(), cs.len));
                        } else {
                            e_refused += 1;
                            c.check(e.contains("capacity"), || format!("(e) head site wound, none, e = t: refusal without the number: {e}"));
                        }
                    }
                }
            }
            // truncation: the tail site and the last t-1 squares gone
            let cut = cont.len() - (HDR + g.msize) - (t.saturating_sub(1)) * cs.blk / 2;
            e_trials += 1;
            match dearmor(&cont[..cut], &[], true) {
                Ok(o) => c.check(o.hash_ok && o.inner == inner && o.padded == cont.len() - cut, || format!("(e) truncation blk {} {} len {}: wrong", cs.blk, cs.mode.name(), cs.len)),
                Err(e) => c.check(false, || format!("(e) truncation blk {} {} len {}: REFUSED ({e})", cs.blk, cs.mode.name(), cs.len)),
            }
            // the mid site dead (hdr1 + meta1) together with the squares around it
            // (t = 1 kills only the square after the site; t >= 2 one on each side)
            let a = if t >= 2 { off.h1.saturating_sub(cs.blk) } else { off.h1 };
            let l = (HDR + g.msize + if t >= 2 { 2 * cs.blk } else { cs.blk }).min(cont.len() - a);
            let mut hurt = cont.clone();
            for b in &mut hurt[a..a + l] {
                *b = xorshift(&mut st) as u8;
            }
            e_trials += 1;
            match dearmor(&hurt, &[], true) {
                Ok(o) => c.check(o.hash_ok && o.inner == inner, || format!("(e) mid site wound blk {} {} len {}: wrong bytes", cs.blk, cs.mode.name(), cs.len)),
                Err(e) => c.check(false, || format!("(e) mid site wound blk {} {} len {}: REFUSED ({e})", cs.blk, cs.mode.name(), cs.len)),
            }
        }
    }
    println!("  (e) erasure round-trips: {} geometries over 3 placements, {} trials (blind and named, {} pattern classes + sites + truncation); {} honest refusals in the blind corners (in-codeword k = m >= 3; none e = t), {} of the corner still exact, {} exact by rung C (all dead squares parity/CT)", cases.len(), e_trials, 9, e_refused, e_beyond_exact, e_rung_c);

    c.section();
    // ---------- (f) errors located by the syndromes alone ----------
    // residues distrusted: e <= floor(t/2) noisy squares, nothing names them
    let mut f_trials = 0u64;
    let mut f_collab = 0u64;
    let f_cases: Vec<(usize, usize, usize, CtMode)> = if full {
        vec![(3_000, 256, 17, CtMode::Triple), (3_000, 256, 17, CtMode::InCodeword), (20_000, 512, 9, CtMode::Triple), (20_000, 512, 9, CtMode::InCodeword), (50_000, 1024, 5, CtMode::Triple), (90_000, 2048, 3, CtMode::InCodeword), (9_000, 256, 40, CtMode::Triple),
             (3_000, 256, 18, CtMode::Absent), (20_000, 512, 10, CtMode::Absent), (50_000, 1024, 6, CtMode::Absent), (90_000, 2048, 4, CtMode::Absent), (9_000, 256, 41, CtMode::Absent)]
    } else {
        vec![(3_000, 256, 17, CtMode::Triple), (3_000, 256, 17, CtMode::InCodeword), (20_000, 512, 9, CtMode::InCodeword), (50_000, 1024, 5, CtMode::Triple), (90_000, 2048, 3, CtMode::InCodeword),
             (3_000, 256, 18, CtMode::Absent), (20_000, 512, 10, CtMode::Absent), (90_000, 2048, 4, CtMode::Absent)]
    };
    for &(len, blk, t, mode) in &f_cases {
        let inner = xbytes(len, len as u64 + 99);
        let g = geom(len, blk, t, mode, fnv64(&inner), dummy_ex());
        let cont = armor(&inner, blk, t, mode, dummy_ex());
        let mut st = 0xF00D_u64 ^ len as u64;
        for e in 1..=t / 2 {
            for _ in 0..(if full { 4 } else { 2 }) {
                let squares = pick_distinct(g.n, e, &mut st);
                let (hurt, _) = kill_squares(&cont, &g, &squares, xorshift(&mut st));
                f_trials += 1;
                match dearmor_with(&hurt, &[], false) {
                    Ok(o) => {
                        c.check(o.hash_ok && o.inner == inner, || format!("(f) e={e} blk {blk} {}: wrong bytes", mode.name()));
                        c.check(o.t.by_syndrome == e && o.t.by_residue == 0, || format!("(f) e={e} blk {blk} {}: located {} by syndrome, {} by residue", mode.name(), o.t.by_syndrome, o.t.by_residue));
                        f_collab += 1;
                    }
                    Err(err) => c.check(false, || format!("(f) e={e} blk {blk} {}: REFUSED ({err})", mode.name())),
                }
                // e errors + (t - 2e) named erasures: 2e + |E| == t exactly
                let extra = t - 2 * e;
                if extra > 0 {
                    let mut all = squares.clone();
                    let mut more = pick_distinct(g.n, e + extra, &mut st).into_iter().filter(|j| !squares.contains(j)).take(extra).collect::<Vec<_>>();
                    all.append(&mut more);
                    let (hurt, wounds) = kill_squares(&cont, &g, &all, xorshift(&mut st));
                    let named: Vec<(usize, usize)> = wounds[e..].to_vec();
                    f_trials += 1;
                    match dearmor_with(&hurt, &named, false) {
                        Ok(o) => c.check(o.hash_ok && o.inner == inner, || format!("(f) e={e} + {extra} erasures blk {blk} {}: wrong bytes", mode.name())),
                        Err(err) => c.check(false, || format!("(f) e={e} + {extra} erasures blk {blk} {}: REFUSED ({err})", mode.name())),
                    }
                }
            }
        }
        // single-symbol noise (one byte) in e squares: the smallest error weight
        for _ in 0..2 {
            let e = (t / 2).max(1);
            let squares = pick_distinct(g.n, e, &mut st);
            let mut hurt = cont.clone();
            for &j in &squares {
                let o = square_off(&g, j) + (xorshift(&mut st) as usize % square_len(&g, j));
                hurt[o] ^= 1 << (xorshift(&mut st) % 8);
            }
            f_trials += 1;
            match dearmor_with(&hurt, &[], false) {
                Ok(o) => c.check(o.hash_ok && o.inner == inner, || format!("(f) one-bit x {e} blk {blk} {}: wrong bytes", mode.name())),
                Err(err) => c.check(false, || format!("(f) one-bit x {e} blk {blk} {}: REFUSED ({err})", mode.name())),
            }
        }
    }
    println!("  (f) errors located by the syndromes alone (residues distrusted; placement none has none): {} trials over {} geometries, e <= floor(t/2) and 2e + |E| = t; {} exact with every error tallied by syndrome", f_trials, f_cases.len(), f_collab);

    c.section();
    // ---------- (g) t+1 REFUSES; beyond capacity never yields wrong data ----------
    let mut g_trials = 0u64;
    let mut g_refused = 0u64;
    let mut g_hash_caught = 0u64;
    let mut g_exact = 0u64;
    let mut g_rung_c = 0u64;
    for cs in cases.iter().filter(|cs| cs.t > 0) {
        let inner = xbytes(cs.len, cs.len as u64 * 13 + 5);
        let g = geom(cs.len, cs.blk, cs.t, cs.mode, fnv64(&inner), dummy_ex());
        if g.n > NMAX || g.n <= cs.t + 1 {
            continue;
        }
        let cont = armor(&inner, cs.blk, cs.t, cs.mode, dummy_ex());
        let mut st = 0xDEAD_u64 ^ cs.len as u64;
        for _ in 0..(if full { 4 } else { 2 }) {
            let squares = pick_distinct(g.n, cs.t + 1, &mut st);
            let all_nondata = squares.iter().all(|&j| !g.is_data(j));
            let (hurt, wounds) = kill_squares(&cont, &g, &squares, xorshift(&mut st));
            for named in [false, true] {
                g_trials += 1;
                let w: &[(usize, usize)] = if named { &wounds } else { &[] };
                match dearmor(&hurt, w, true) {
                    Ok(o) => {
                        // Ok is tolerable as an honest hash failure (restore refuses on it) or as
                        // rung C when every dead square was parity/CT (the data was never touched)
                        if o.hash_ok {
                            c.check(o.inner == inner && all_nondata && o.by_hash, || format!("(g) t+1 dead blk {} {} len {}: SUCCESS CODE WITH WRONG DATA (or exact from a dead data square)", cs.blk, cs.mode.name(), cs.len));
                            g_rung_c += 1;
                        } else {
                            g_hash_caught += 1;
                        }
                    }
                    Err(e) => {
                        g_refused += 1;
                        c.check(e.contains("capacity") || e.contains("dead squares") || e.contains("refus") || e.contains("inconsistent"), || format!("(g) refusal without a number: {e}"));
                    }
                }
            }
        }
    }
    // random byte noise at many densities, blind: EXACT or a refusal, never wrong
    for &(len, blk, t, mode) in &f_cases {
        let inner = xbytes(len, len as u64 + 99);
        let cont = armor(&inner, blk, t, mode, dummy_ex());
        let mut st = 0xBEEF_u64 ^ len as u64;
        for &density in &[1usize, 8, 64, 512] {
            for _ in 0..(if full { 3 } else { 1 }) {
                let mut hurt = cont.clone();
                let hits = (cont.len() / 4096 + 1) * density / 8 + 1;
                for _ in 0..hits {
                    let o = xorshift(&mut st) as usize % hurt.len();
                    hurt[o] = xorshift(&mut st) as u8;
                }
                g_trials += 1;
                match dearmor(&hurt, &[], true) {
                    Ok(o) => {
                        if o.hash_ok {
                            c.check(o.inner == inner, || format!("(g) noise {hits} hits blk {blk}: hash_ok with WRONG bytes"));
                            g_exact += 1;
                        } else {
                            g_hash_caught += 1;
                        }
                    }
                    Err(_) => g_refused += 1,
                }
            }
        }
    }
    println!("  (g) beyond capacity: {} trials (t+1 scattered squares blind and named, all 3 placements; random noise at 4 densities); {} refused with a number, {} caught by the hash, {} exact by rung C (t+1 dead squares all parity/CT), {} light-noise trials exact, wrong data 0", g_trials, g_refused, g_hash_caught, g_rung_c, g_exact);

    c.section();
    // ---------- (h) the miscorrection bound ----------
    // e > t/2 errors with residues distrusted: BM may find a wrong locator
    // of degree <= floor(t/2); the residual syndromes, the systematic check
    // and the FNV-64 must stop every one. Count them.
    let mut h_trials = 0u64;
    let mut h_wrong_decode = 0u64;
    let mut h_refused = 0u64;
    let mut h_exact = 0u64;
    let mut h_rung_c = 0u64;
    let mut h_rank_trap = 0u64;
    for &(len, blk, t, mode) in f_cases.iter() {
        let inner = xbytes(len, len as u64 + 99);
        let g = geom(len, blk, t, mode, fnv64(&inner), dummy_ex());
        let cont = armor(&inner, blk, t, mode, dummy_ex());
        let mut st = 0x7777_u64 ^ len as u64;
        // e in (t/2, t-1]: the interleaved codewords locate them jointly (k < m);
        // e in {t, t+1}: k >= m, nothing can place them -- REFUSE or hash-caught
        // (or rung C when every dead square was parity/CT)
        let es: Vec<usize> = ((t / 2 + 1)..=(t - 1).max(t / 2 + 1)).chain([t, t + 1]).collect();
        for e in es {
            for _ in 0..(if full { 6 } else { 3 }) {
                let squares = pick_distinct(g.n, e, &mut st);
                let all_nondata = squares.iter().all(|&j| !g.is_data(j));
                let (hurt, _) = kill_squares(&cont, &g, &squares, xorshift(&mut st));
                h_trials += 1;
                match dearmor_with(&hurt, &[], false) {
                    Ok(o) => {
                        if o.hash_ok {
                            c.check(o.inner == inner, || format!("(h) e={e} > t/2 blk {blk} {}: hash_ok with WRONG bytes", mode.name()));
                            c.check(e < t || (all_nondata && o.by_hash), || format!("(h) e={e} >= t={t} blk {blk} {}: exact from nothing -- impossible, investigate", mode.name()));
                            if o.by_hash {
                                h_rung_c += 1;
                            } else {
                                h_exact += 1;
                            }
                        } else {
                            h_wrong_decode += 1;
                        }
                    }
                    Err(_) => {
                        c.check(e >= t, || format!("(h) e={e} < t={t} blk {blk} {}: the codewords should have located them jointly", mode.name()));
                        h_refused += 1;
                    }
                }
            }
        }
        // the RANK TRAP (placement none): two data squares with identical content
        // scribbled identically have identical error rows -- the joint locator sees
        // one dimension and finds no position; the verdict must be HONEST or EXACT,
        // never wrong. Built on a container whose squares 1 and 3 are made equal.
        if mode == CtMode::Absent && g.s >= 5 && t >= 2 {
            let mut twin = inner.clone();
            let (a, b) = (1usize, 3usize);
            let src = twin[a * blk..(a + 1) * blk].to_vec();
            twin[b * blk..(b + 1) * blk].copy_from_slice(&src);
            let cont2 = armor(&twin, blk, t, mode, dummy_ex());
            let g2 = geom(twin.len(), blk, t, mode, fnv64(&twin), dummy_ex());
            let noise = xbytes(blk, xorshift(&mut st));
            let mut hurt = cont2.clone();
            for &j in &[a, b] {
                let o = square_off(&g2, j);
                hurt[o..o + blk].copy_from_slice(&noise);
            }
            h_trials += 1;
            match dearmor(&hurt, &[], true) {
                Ok(o) => {
                    c.check(!o.hash_ok || o.inner == twin, || format!("(h) RANK TRAP blk {blk}: hash_ok with WRONG bytes"));
                    if o.hash_ok {
                        h_exact += 1;
                    } else {
                        h_wrong_decode += 1;
                    }
                }
                Err(_) => {
                    h_rank_trap += 1;
                }
            }
            // the same two squares named: erasures, EXACT
            let w = vec![(square_off(&g2, a), blk), (square_off(&g2, b), blk)];
            h_trials += 1;
            match dearmor(&hurt, &w, true) {
                Ok(o) => c.check(o.hash_ok && o.inner == twin, || format!("(h) RANK TRAP named blk {blk}: not exact")),
                Err(e) => c.check(false, || format!("(h) RANK TRAP named blk {blk}: REFUSED ({e})")),
            }
        }
    }
    c.check(h_wrong_decode == 0, || format!("(h) {h_wrong_decode} decodes settled the syndromes on a wrong codeword (the FNV caught them; the bound wants zero)"));
    println!("  (h) miscorrection bound: {} trials with e > floor(t/2) unnamed errors over {} geometries; {} exact (e <= t-1, located jointly), {} exact by rung C, {} refused by the syndromes (e >= t), {} rank traps refused honestly (named: exact), {} settled wrong and caught only by FNV-64", h_trials, f_cases.len(), h_exact, h_rung_c, h_refused, h_rank_trap, h_wrong_decode);

    let verdict = c.fails == 0;
    println!(
        "audit v4: {} checks, {} failing -- {} [{} ms]",
        c.checks,
        c.fails,
        if verdict { "ALL PASSING" } else { "FAILING" },
        t0.elapsed().as_millis()
    );
    verdict
}
