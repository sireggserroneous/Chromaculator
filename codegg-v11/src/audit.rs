//! audit.rs -- the geometry audit (the bulk reading: the site ships a
//! checked table, "298,545 checks, all passing"; this is that discipline
//! pointed at the armor). Sections (a) and (b) run at every milestone gate;
//! the full battery (repair-boundary maps, fuzz, adversarial multi-wounds)
//! rides behind `audit --full` and lands at M5.
//!
//! Nothing here is hardcoded to match the shipping code's OPINIONS -- the
//! windows slide over the REAL slot_order, the claims come from the REAL
//! rib_policy/guaranteed_st, and the naive continuous policy G = ceil(s/5)
//! is kept in as a NEGATIVE CONTROL: the audit must catch it failing, or
//! the audit itself is broken.

use crate::armor::{
    armor, dearmor, dead_slots, fnv64, geom, guaranteed_st, offsets, rib_policy, slot_off,
    slot_order, syn_distinct, syndrome_pair, Extras, Geom, BLOCK, GMAX5, HDR, TIERS,
};
use std::collections::HashMap;

fn dummy_ex() -> Extras {
    Extras { orig_len: 0, orig_fnv: 0, model: 0, filter_id: 0, filter_param: 0 }
}

/// slide every dead_slots(blk)-slot window over the real merged order;
/// return the most members any single group loses to one window, and the
/// windows examined.
fn worst_window(g: &Geom) -> (usize, u64) {
    let order = slot_order(g);
    let n = order.len();
    if n == 0 {
        return (0, 0);
    }
    let ngroups = order.iter().map(|s| s.group).max().unwrap() as usize + 1;
    let mut count = vec![0u32; ngroups];
    let mut worst = 0usize;
    let mut windows = 0u64;
    let w = dead_slots(g.blk).min(n);
    for j in 0..w {
        let c = &mut count[order[j].group as usize];
        *c += 1;
        worst = worst.max(*c as usize);
    }
    windows += 1;
    for j in w..n {
        count[order[j - w].group as usize] -= 1;
        let c = &mut count[order[j].group as usize];
        *c += 1;
        worst = worst.max(*c as usize);
        windows += 1;
    }
    (worst, windows)
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// sections (a) + (b); returns (all_ok, checks_counted)
pub fn run_gate() -> (bool, u64) {
    let mut ok = true;
    let mut checks: u64 = 0;

    // ---------- (a) stripe pigeonhole ----------
    // policy choice, exhaustive s = 1..=2000
    let mut claimed_cnt = 0u64;
    let mut floor_cnt = 0u64;
    for s in 1..=2000usize {
        let len = s * BLOCK;
        let rib = rib_policy(len);
        let g = geom(len, rib.blk, GMAX5, rib.g, rib.t, 0, dummy_ex());
        let (worst, w) = worst_window(&g);
        checks += w;
        if rib.guaranteed {
            claimed_cnt += 1;
            if worst > g.t {
                ok = false;
                println!(
                    "  FAIL (a) policy s={}: G{} T{} claimed, window kills {} of one group",
                    s, rib.g, rib.t, worst
                );
            }
        } else {
            floor_cnt += 1;
        }
    }
    println!(
        "  (a) policy, exhaustive s=1..=2000: {} guaranteed geometries hold under every {}-slot window; {} tiny (s<=11) honestly unguaranteed",
        claimed_cnt, dead_slots(BLOCK), floor_cnt
    );

    // adversarial grids: the PREDICATE itself vs reality, many (G,T) per s.
    // guaranteed_st may under-claim (conservative) but must NEVER over-claim.
    // the predicate is proved PER TIER (v11-M1: the wide grid): every square
    // size gets its own window width and its own adversarial sweep
    let grids: [(usize, usize); 13] = [
        (4, 2), (4, 3), (5, 2), (9, 2), (11, 2), (13, 3), (31, 2), (63, 2),
        (126, 2), (126, 3), (200, 2), (248, 2), (248, 3),
    ];
    let mut grid_claims = 0u64;
    for &(blk, _, _) in TIERS.iter() {
        for s in 1..=2000usize {
            for &(gg, tt) in &grids {
                let g = geom(s * blk, blk, GMAX5, gg, tt, 0, dummy_ex());
                let (worst, w) = worst_window(&g);
                checks += w;
                if guaranteed_st(s, gg, tt, blk) {
                    grid_claims += 1;
                    if worst > tt {
                        ok = false;
                        println!(
                            "  FAIL (a) predicate blk={} s={} G{} T{}: claimed but window kills {}",
                            blk, s, gg, tt, worst
                        );
                    }
                }
            }
        }
    }
    println!(
        "  (a) predicate, adversarial grids (13 rib shapes x 2000 sizes x 3 tiers): {} claims, none over-claimed",
        grid_claims
    );

    // negative control: the naive continuous policy G = ceil(s/5), T = 2
    // MUST fail at a size where the argmin policy HOLDS -- that isolates the
    // ragged-tail defect (the formula's sin, not the physics'). If the naive
    // formula never fails there, the audit is not looking.
    let mut naive_broken: Option<(usize, usize, usize)> = None;
    let mut naive_fail_count = 0u64;
    for s in 12..=2000usize {
        let gg = s.div_ceil(5).clamp(4, 126);
        let g = geom(s * BLOCK, BLOCK, GMAX5, gg, 2, 0, dummy_ex());
        let (worst, w) = worst_window(&g);
        checks += w;
        if worst > 2 && rib_policy(s * BLOCK).guaranteed {
            naive_fail_count += 1;
            if naive_broken.is_none() {
                naive_broken = Some((s, gg, worst));
            }
        }
    }
    match naive_broken {
        Some((s, gg, worst)) => println!(
            "  (a) negative control: naive G=ceil(s/5) breaks at {} sizes the argmin holds; first s={} (G{}: one group loses {} > T=2) -- the audit sees",
            naive_fail_count, s, gg, worst
        ),
        None => {
            ok = false;
            println!("  FAIL (a) negative control: naive formula never failed -- the audit is blind");
        }
    }

    // log-sampled to 10^6 slots, policy choice
    let mut st = 0x1489u64;
    let mut sampled = 0u64;
    for i in 0..200 {
        // log-spaced base + jitter, 2001..~10^6
        let mag = 2001.0 * (1e6f64 / 2001.0).powf(i as f64 / 199.0);
        let s = (mag as usize + (xorshift(&mut st) % 97) as usize).min(1_000_000);
        let rib = rib_policy(s * BLOCK);
        let g = geom(s * BLOCK, rib.blk, GMAX5, rib.g, rib.t, 0, dummy_ex());
        let (worst, w) = worst_window(&g);
        checks += w;
        sampled += 1;
        if rib.guaranteed && worst > g.t {
            ok = false;
            println!("  FAIL (a) sampled s={}: G{} T{} claimed, window kills {}", s, rib.g, rib.t, worst);
        }
    }
    println!("  (a) log-sampled: {} sizes up to 10^6 slots, policy holds", sampled);

    // ---------- (b) residue injectivity, per tier ----------
    for &(blk, p, q) in TIERS.iter() {
        let nbits = 8 * blk;
        for m in [p, q] {
            let d = syn_distinct(m, nbits);
            checks += 2 * nbits as u64;
            if d != 2 * nbits {
                ok = false;
            }
            println!(
                "  (b) tier {} B, modulus {}: {}/{} signed syndromes +-2^k (k<{}) distinct{}",
                blk,
                m,
                d,
                2 * nbits,
                nbits,
                if d == 2 * nbits { "" } else { " -- FAIL" }
            );
        }
    }
    // sampled double-error ambiguity: two different bit-pairs colliding in
    // BOTH moduli at once -- the physics behind the doubles retry rung
    for (ti, &(blk, p, q)) in TIERS.iter().enumerate() {
        let nbits = (8 * blk) as u64;
        let mut seen: HashMap<(u32, u32), (u16, u16)> = HashMap::new();
        let mut ambig = 0u64;
        let nsamp: u64 = if ti == 0 { 200_000 } else { 50_000 };
        let mut st2 = 0xACEu64 + ti as u64;
        for _ in 0..nsamp {
            let i1 = (xorshift(&mut st2) % nbits) as u16;
            let mut i2 = (xorshift(&mut st2) % nbits) as u16;
            if i2 == i1 {
                i2 = ((i2 as u64 + 1) % nbits) as u16;
            }
            let d1: i8 = if xorshift(&mut st2) & 1 == 1 { 1 } else { -1 };
            let d2: i8 = if xorshift(&mut st2) & 1 == 1 { 1 } else { -1 };
            let (p1, q1) = syndrome_pair(blk, i1, d1);
            let (p2, q2) = syndrome_pair(blk, i2, d2);
            let key = (((p1 + p2) % p), ((q1 + q2) % q));
            let pair = (i1.min(i2), i1.max(i2));
            if let Some(&prev) = seen.get(&key) {
                if prev != pair {
                    ambig += 1;
                }
            } else {
                seen.insert(key, pair);
            }
        }
        checks += nsamp;
        println!(
            "  (b) tier {} B double-error ambiguity, {} sampled pairs: {} dual-modulus collisions ({:.2e}/pair) -- why the ladder's last rung trusts parity only",
            blk,
            nsamp,
            ambig,
            ambig as f64 / nsamp as f64
        );
    }

    (ok, checks)
}

// ---------------- (c)(d)(e): the armor under real injuries, in process ----------------

fn xbytes(n: usize, seed: u64) -> Vec<u8> {
    let mut st = seed;
    (0..n).map(|_| (xorshift(&mut st) & 0xff) as u8).collect()
}
fn make_artifact(inner_len: usize, seed: u64) -> (Vec<u8>, Vec<u8>, Geom) {
    let inner = xbytes(inner_len, seed);
    let rib = rib_policy(inner.len());
    let ex = Extras { orig_len: inner.len() as u64, orig_fnv: fnv64(&inner), model: 1, filter_id: 0, filter_param: 0 };
    let cont = armor(&inner, rib.blk, rib.g, rib.t, ex);
    let g = geom(inner.len(), rib.blk, GMAX5, rib.g, rib.t, 0, ex);
    (inner, cont, g)
}
#[derive(PartialEq, Clone, Copy, Debug)]
enum Verdict {
    Exact,
    Honest,
    Silent,
}
/// injure -> dearmor -> classify. EXACT means the armor returned the true
/// inner with its payload hash verified; HONEST means it refused or reported
/// failure; SILENT (hash ok but wrong bytes) fails the audit outright.
fn attempt(inner: &[u8], hurt: &[u8], wounds: &[(usize, usize)]) -> Verdict {
    match dearmor(hurt, wounds, true) {
        Ok(o) => {
            if !o.hash_ok {
                Verdict::Honest
            } else if o.inner == inner {
                Verdict::Exact
            } else {
                Verdict::Silent
            }
        }
        Err(_) => Verdict::Honest,
    }
}
fn scratch(cont: &[u8], at: usize, len: usize, seed: u64) -> Vec<u8> {
    let mut b = cont.to_vec();
    let mut st = seed;
    for i in at..(at + len).min(b.len()) {
        b[i] = (xorshift(&mut st) & 0xff) as u8;
    }
    b
}

/// (c) repair-boundary maps: binary-search the largest EXACT-repairable
/// contiguous scratch per region, blind and addressed, probe +-512 around
/// the boundary, print measured vs the geometry's own theory.
fn boundary_maps() -> (bool, u64) {
    let mut ok = true;
    let mut checks = 0u64;
    println!("  (c) repair-boundary maps (largest EXACT contiguous scratch, measured vs theory):");
    // 4 MB rides tier 1, 9 MB tier 2 -- the wide grid's boundaries measured
    for &inner_len in &[8 * 1024usize, 30 * 1024, 100 * 1024, 500 * 1024, 4 * 1024 * 1024, 9 * 1024 * 1024] {
        let (inner, cont, g) = make_artifact(inner_len, 0x1489 + inner_len as u64);
        let off = offsets(&g);
        // theory: the largest k such that EVERY k-slot window leaves every
        // group <= T dead -- computed from the real order, not the formula
        let order = slot_order(&g);
        let ngroups = order.iter().map(|s| s.group).max().map(|x| x as usize + 1).unwrap_or(0);
        let window_ok = |k: usize| -> bool {
            if k == 0 {
                return true;
            }
            let mut cnt = vec![0u32; ngroups];
            let mut worst = 0;
            for j in 0..order.len() {
                cnt[order[j].group as usize] += 1;
                if j >= k {
                    cnt[order[j - k].group as usize] -= 1;
                }
                worst = worst.max(*cnt.iter().max().unwrap());
            }
            (worst as usize) <= g.t
        };
        let mut klo = 0usize;
        let mut khi = order.len();
        while klo < khi {
            let mid = (klo + khi).div_ceil(2);
            if window_ok(mid) {
                klo = mid;
            } else {
                khi = mid - 1;
            }
        }
        let theory = klo.saturating_sub(1) * g.blk + 1; // k dead slots span >= (k-1)*B+1 bytes
        let anchors: Vec<(&str, usize, bool)> = {
            let first_l2 = order.iter().position(|s| s.level == 1).map(|j| slot_off(&g, j));
            let mut v = vec![
                ("head", 0usize, true),
                ("payload", off.slot_base + (g.mid / 2) * g.blk, true),
                ("mid-replica", off.h1, true),
            ];
            if let Some(o2) = first_l2 {
                v.push(("CT slot", o2, true));
            }
            v.push(("end", cont.len(), false)); // scratch ENDS at the end
            v
        };
        for (name, anchor, forward) in anchors {
            for &blind in &[true, false] {
                let try_len = |len: usize| -> Verdict {
                    if len == 0 {
                        return Verdict::Exact;
                    }
                    let at = if forward { anchor } else { cont.len().saturating_sub(len) };
                    let hurt = scratch(&cont, at, len, 0xACE ^ len as u64);
                    let wounds: Vec<(usize, usize)> = if blind { vec![] } else { vec![(at, len)] };
                    attempt(&inner, &hurt, &wounds)
                };
                // binary search on EXACT (probe the boundary after)
                let (mut lo, mut hi) = (0usize, cont.len());
                while lo < hi {
                    let mid = (lo + hi).div_ceil(2);
                    checks += 1;
                    match try_len(mid) {
                        Verdict::Exact => lo = mid,
                        Verdict::Honest => hi = mid - 1,
                        Verdict::Silent => {
                            ok = false;
                            println!("      SILENT at {} len {}", name, mid);
                            hi = mid - 1;
                        }
                    }
                }
                // probe +-512 around the boundary; non-monotonicity is
                // reported, silence is a failure
                let mut above_exact = 0;
                for d in 1..=4usize {
                    let len = lo + d * 128;
                    if len <= cont.len() {
                        checks += 1;
                        match try_len(len) {
                            Verdict::Exact => above_exact += 1,
                            Verdict::Honest => {}
                            Verdict::Silent => {
                                ok = false;
                                println!("      SILENT at {} len {}", name, len);
                            }
                        }
                    }
                }
                println!(
                    "    {:>7} B artifact, {:<11} {}: {:>7} B exact ({} theory{}{})",
                    cont.len(),
                    name,
                    if blind { "blind    " } else { "addressed" },
                    lo,
                    theory,
                    if anchor == 0 || !forward { " + replica/site absorption" } else { "" },
                    if above_exact > 0 { format!("; {} exact spots past the boundary", above_exact) } else { String::new() }
                );
            }
        }
    }
    (ok, checks)
}

/// (d) fuzz: 10,000 deterministic injuries, EXACT-or-honest, ZERO silent
fn fuzz() -> (bool, u64) {
    let mut ok = true;
    let arts: Vec<(Vec<u8>, Vec<u8>, Geom)> = [8 * 1024usize, 30 * 1024, 100 * 1024, 500 * 1024, 2 * 1024 * 1024, 9 * 1024 * 1024]
        .iter()
        .map(|&n| make_artifact(n, 0xE99 + n as u64))
        .collect();
    let mut st = 0x1489u64;
    let (mut exact, mut honest) = (0u64, 0u64);
    let mut silent = 0u64;
    let n_iter = 10_000u64;
    for it in 0..n_iter {
        let (inner, cont, g) = &arts[(xorshift(&mut st) % arts.len() as u64) as usize];
        let off = offsets(g);
        let kind = xorshift(&mut st) % 5;
        let v = match kind {
            0 => {
                // scratch 1 B .. 50%
                let len = 1 + (xorshift(&mut st) as usize) % (cont.len() / 2);
                let at = (xorshift(&mut st) as usize) % (cont.len() - len.min(cont.len() - 1));
                let hurt = scratch(cont, at, len, it);
                let blind = xorshift(&mut st) & 1 == 0;
                let w: Vec<(usize, usize)> = if blind { vec![] } else { vec![(at, len)] };
                attempt(inner, &hurt, &w)
            }
            1 => {
                // truncation 1 B .. 90%
                let cut = 1 + (xorshift(&mut st) as usize) % (cont.len() * 9 / 10);
                attempt(inner, &cont[..cont.len() - cut], &[])
            }
            2 => {
                // random container (must refuse or fail honestly)
                let n = 64 + (xorshift(&mut st) as usize) % 65536;
                let junk = xbytes(n, it ^ 0xDEAD);
                match dearmor(&junk, &[], true) {
                    Ok(o) if o.hash_ok => Verdict::Silent, // random bytes must never verify
                    _ => Verdict::Honest,
                }
            }
            3 => {
                // header/meta-targeted scratch
                let sites = [off.h0, off.m0, off.h1, off.m1, off.m2, off.h2];
                let at = sites[(xorshift(&mut st) % 6) as usize];
                let len = 1 + (xorshift(&mut st) as usize) % (HDR + g.msize);
                let hurt = scratch(cont, at.min(cont.len() - 1), len, it);
                attempt(inner, &hurt, &[])
            }
            _ => {
                // bit storms, 1..64 flips
                let bits = 1 + (xorshift(&mut st) as usize) % 64;
                let mut hurt = cont.to_vec();
                for _ in 0..bits {
                    let bit = (xorshift(&mut st) as usize) % (hurt.len() * 8);
                    hurt[bit >> 3] ^= 1 << (bit & 7);
                }
                attempt(inner, &hurt, &[])
            }
        };
        match v {
            Verdict::Exact => exact += 1,
            Verdict::Honest => honest += 1,
            Verdict::Silent => {
                silent += 1;
                ok = false;
                println!("      SILENT at fuzz iteration {} (kind {})", it, kind);
            }
        }
    }
    println!(
        "  (d) fuzz x{}: {} EXACT, {} honest, {} SILENT (zero required; accept-wrong physics ~2^-64/attempt, FNV-64 -- not cryptographic, adversaries out of scope)",
        n_iter, exact, honest, silent
    );
    (ok, n_iter)
}

/// (e) adversarial multi-wounds; expected verdicts DERIVED from the geometry
/// functions, never hardcoded. The derivation follows the drill lesson: a
/// payload group past T is data loss (HONEST); a check-table group past T
/// loses only checks and the payload hash arbitrates (EXACT).
fn adversarial() -> (bool, u64) {
    let mut ok = true;
    let mut checks = 0u64;
    println!("  (e) adversarial multi-wounds (verdicts derived from geometry):");
    for &inner_len in &[30 * 1024usize, 500 * 1024, 3 * 1024 * 1024, 9 * 1024 * 1024] {
        let (inner, cont, g) = make_artifact(inner_len, 0xC0FFEE + inner_len as u64);
        let off = offsets(&g);
        let order = slot_order(&g);
        let ngroups = order.iter().map(|s| s.group).max().unwrap() as usize + 1;
        let ng1 = {
            let s = g.s;
            if s == 0 { 0 } else { s.div_ceil(g.g) }
        };
        // derive the verdict for a set of byte wounds: dead slots per group
        let derive = |wounds: &[(usize, usize)]| -> Verdict {
            let mut dead = vec![0usize; ngroups];
            for (j, sl) in order.iter().enumerate() {
                let a = slot_off(&g, j);
                if wounds.iter().any(|&(w, l)| w < a + g.blk && w + l > a) {
                    dead[sl.group as usize] += 1;
                }
            }
            let payload_over = dead.iter().take(ng1).any(|&d| d > g.t);
            if payload_over { Verdict::Honest } else { Verdict::Exact }
        };
        let run_case = |name: &str, wounds: &[(usize, usize)], ok: &mut bool, checks: &mut u64| {
            let expected = derive(wounds);
            let mut hurt = cont.clone();
            for (i, &(at, len)) in wounds.iter().enumerate() {
                hurt = scratch(&hurt, at, len, 0xE0 + i as u64);
            }
            for blind in [true, false] {
                let w: Vec<(usize, usize)> = if blind { vec![] } else { wounds.to_vec() };
                let got = attempt(&inner, &hurt, &w);
                *checks += 1;
                let pass = got == expected;
                if got == Verdict::Silent || !pass {
                    *ok = false;
                    println!(
                        "      FAIL {} ({}): expected {:?}, got {:?}",
                        name,
                        if blind { "blind" } else { "addressed" },
                        expected,
                        got
                    );
                } else {
                    println!(
                        "    PASS {:>7} B artifact, {} ({}): {:?} as derived",
                        cont.len(),
                        name,
                        if blind { "blind" } else { "addressed" },
                        got
                    );
                }
            }
        };
        // two disjoint scratches at different stripe phases, within capacity
        let ngt = ngroups;
        let a1 = slot_off(&g, 0);
        let a2 = slot_off(&g, ngt + 1); // different phase, different group rows
        run_case("two disjoint 1-slot scratches", &[(a1, g.blk), (a2, g.blk)], &mut ok, &mut checks);
        // T+1 members of payload group 0 via slot arithmetic (must refuse)
        let mut w = Vec::new();
        for k in 0..=g.t {
            w.push((slot_off(&g, k * ngt), g.blk));
        }
        run_case("T+1 same payload group", &w, &mut ok, &mut checks);
        // T members of payload group 0 AND T of a CT group (both repair)
        if !g.ct_triple {
            let mut w = Vec::new();
            for k in 0..g.t {
                w.push((slot_off(&g, k * ngt), g.blk));
                w.push((slot_off(&g, ng1 + k * ngt), g.blk));
            }
            run_case("T payload + T check-table (twin+CT)", &w, &mut ok, &mut checks);
        }
        // two whole replica sites + a payload slot (selection + parity)
        let site = HDR + g.msize;
        run_case(
            "two replica sites + one slot",
            &[(off.h0, site), (cont.len() - site, site), (slot_off(&g, 3), g.blk)],
            &mut ok,
            &mut checks,
        );
        // region straddle: scratch across the mid site boundary
        run_case("mid-site straddle", &[(off.h1.saturating_sub(g.blk / 2), site + g.blk)], &mut ok, &mut checks);
        // scratch + 8-bit storm on top: never silent (verdict may be either)
        {
            let at = slot_off(&g, 1);
            let mut hurt = scratch(&cont, at, g.blk, 0xF00D);
            let mut st = 0xF00Du64;
            for _ in 0..8 {
                let bit = (xorshift(&mut st) as usize) % (hurt.len() * 8);
                hurt[bit >> 3] ^= 1 << (bit & 7);
            }
            let got = attempt(&inner, &hurt, &[]);
            checks += 1;
            if got == Verdict::Silent {
                ok = false;
                println!("      FAIL scratch+storm: SILENT");
            } else {
                println!("    PASS {:>7} B artifact, scratch+storm (blind): {:?}, never silent", cont.len(), got);
            }
        }
    }
    (ok, checks)
}

pub fn run(full: bool) -> bool {
    println!("the geometry audit -- every claim checked against the real order, counts printed:");
    let t0 = std::time::Instant::now();
    let (mut ok, mut checks) = run_gate();
    if full {
        let (o2, c2) = boundary_maps();
        ok &= o2;
        checks += c2;
        let (o3, c3) = fuzz();
        ok &= o3;
        checks += c3;
        let (o4, c4) = adversarial();
        ok &= o4;
        checks += c4;
    }
    println!(
        "{} checks, {} [{} ms]",
        checks,
        if ok { "all passing" } else { "FAILURES above" },
        t0.elapsed().as_millis()
    );
    ok
}
