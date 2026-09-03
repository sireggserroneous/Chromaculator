//! eggso4 -- the fold is a basin boundary.
//!
//! The nineteenth codec experiment and the fifth in the fold-native lineage,
//! and the first written in Rust. Three parts, in the order they were filed:
//! the placement, then a guess-and-fix decoder, then a chosen-seam
//! interleaver, so each is measured rather than assumed.
//!
//!   eggso4 pin      the port and the coordinate against the site's own code
//!   eggso4 basins   Cayley's two-root line and three-root tangle
//!   eggso4 guess    can a decoder guess and fix?
//!   eggso4 seam     what the fold's forced 1/n seam costs
//!   eggso4 audit    all of it, with the counts printed

use eggso4::{code, dynamics, fold, guess, json, pin, seam};

use code::{repair, Code, Mul32, Opts, Status};
use dynamics as dy;
use json::{obj, record, J};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("audit");
    let full = args.iter().any(|a| a == "--full");
    match cmd {
        "pin" => {
            cmd_pin();
        }
        "basins" => {
            cmd_basins(full);
        }
        "guess" => {
            cmd_guess();
        }
        "seam" => {
            cmd_seam(full);
        }
        "audit" => {
            let a = cmd_pin();
            println!();
            let b = cmd_basins(full);
            println!();
            let c = cmd_guess();
            println!();
            let d = cmd_seam(full);
            println!("\nAUDIT: {} pins clean, {b} basin claims, {c} guess claims, {d} seam arms", a);
        }
        _ => println!("usage: eggso4 pin | basins | guess | seam | audit [--full]"),
    }
}

fn pct(x: usize, n: usize) -> String {
    format!("{:.2}%", 100.0 * x as f64 / n as f64)
}

// ---- the pins ------------------------------------------------------------

fn cmd_pin() -> usize {
    println!("THE PINS -- the port and the coordinate against the site's own code");
    println!("every fold-native round checks its restatement against the site's OWN function;");
    println!("Rust cannot eval stalk.js, so the audit shells out to node rather than lose that.\n");
    if !pin::node_available() {
        println!("  node is not on PATH: every pin SKIPPED, loudly. Nothing here passed.");
        return 0;
    }
    let c = Code::new(32, true, "fold", code::fold_assign);
    let mut results = vec![pin::site_regions(40), pin::site_arcs(40), pin::v0_structure(&c)];

    let mut g = Mul32::new(20260903);
    let mut cases = Vec::new();
    for channel in 0..5 {
        for _ in 0..60 {
            let clean = g.cells(c.l);
            let check = c.checks_for(&clean);
            let mut cells = clean.clone();
            let mut erased = Vec::new();
            match channel {
                0 => cells[g.pick(c.l)] ^= 1,
                1 => {
                    let a = g.pick(c.l);
                    let mut b = g.pick(c.l);
                    while b == a {
                        b = g.pick(c.l);
                    }
                    cells[a] ^= 1;
                    cells[b] ^= 1;
                }
                2 => {
                    let k = g.pick(3);
                    let m = &c.members[k];
                    let a = m[g.pick(m.len())];
                    let mut b = m[g.pick(m.len())];
                    while b == a {
                        b = m[g.pick(m.len())];
                    }
                    cells[a] ^= 1;
                    cells[b] ^= 1;
                }
                3 => {
                    let row = g.pick(c.n);
                    let c0 = g.pick(c.n - 12);
                    for j in 0..12 {
                        let i = row * c.n + c0 + j;
                        cells[i] = -1;
                        erased.push(i);
                    }
                }
                _ => {
                    for &i in &c.members[fold::FOLD as usize] {
                        cells[i] ^= 1;
                    }
                }
            }
            for pc in [false, true] {
                cases.push(pin::Case {
                    cells: cells.clone(),
                    check: check.clone(),
                    erased: erased.clone(),
                    per_candidate: pc,
                });
            }
        }
    }
    results.push(pin::v0_decisions(&c, &cases));

    for r in &results {
        println!("{}", r.line());
    }
    let clean = results.iter().filter(|r| r.ok()).count();
    println!(
        "\n  {clean} of {} pins clean{}",
        results.len(),
        if clean < results.len() { " -- THE ROUND STOPS UNTIL THEY ARE" } else { "" }
    );
    let rec = J::A(
        results
            .iter()
            .map(|r| {
                obj(&[
                    ("pin", J::s(r.name)),
                    ("checked", J::U(r.checked)),
                    ("mismatches", J::U(r.mismatches)),
                    ("skipped", J::B(r.skipped.is_some())),
                ])
            })
            .collect(),
    );
    let _ = record("pins", &rec);
    clean
}

// ---- Part 1: the placement ----------------------------------------------

fn cmd_basins(full: bool) -> usize {
    println!("PART 1 -- the placement. Cayley 1879: two roots give a line, three do not.");
    let mut claims = 0usize;

    // the coordinate, re-derived here and not quoted from the plan
    let mut cells = 0usize;
    let mut bad = 0usize;
    for n in 2..=64usize {
        for r in 0..n {
            for c in 0..n {
                cells += 1;
                let rho = fold::rho_of(r, c, n);
                let by = if rho < 1.0 {
                    fold::INNER
                } else if rho == 1.0 {
                    fold::FOLD
                } else {
                    fold::OUTER
                };
                let (pr, pc) = fold::sigma_rc(r, c, n);
                if by != fold::region_of(r, c, n) || fold::rho_of(pr, pc, n) != 1.0 / rho {
                    bad += 1;
                }
            }
        }
    }
    println!("  the coordinate rho = 2^(d-(n-1)), n = 2..64: {cells} cells, {bad} exceptions");
    println!("    Inner is |rho|<1, the Fold is |rho|=1, Outer is |rho|>1, and sigma sends rho -> 1/rho.");
    claims += 1;

    // the quadratic: a straight line
    let g = if full { 801 } else { 401 };
    let mut tested = 0usize;
    let mut wrong = 0usize;
    let rs2 = dy::roots2();
    for i in 0..g {
        let x = -2.0 + 4.0 * i as f64 / (g - 1) as f64;
        if x.abs() < 0.02 {
            continue;
        }
        for j in 0..g {
            let y = -2.0 + 4.0 * j as f64 / (g - 1) as f64;
            let want = if x > 0.0 { 0 } else { 1 };
            if dy::basin(dy::C::new(x, y), dy::newton2, &rs2, 200) != Some(want) {
                wrong += 1;
            }
            tested += 1;
        }
    }
    println!("  Newton on z^2-1: {tested} guesses, {wrong} landed anywhere but where sign(Re z) says");
    claims += 1;

    // the cubic: no line will do
    let mut stability = Vec::new();
    for res in if full { vec![201usize, 301, 501, 801] } else { vec![201usize, 301, 501] } {
        let grid = dy::basin_grid(res, 2.0, dy::newton3, &dy::roots3());
        let t = dy::tangle(&grid);
        let share = t.all_three as f64 / t.boundary as f64;
        println!(
            "  Newton on z^3-1 at {res}x{res}: {} of {} cells on a boundary, {} touch ALL THREE ({})",
            t.boundary,
            t.samples,
            t.all_three,
            pct(t.all_three, t.boundary)
        );
        stability.push(obj(&[
            ("grid", J::U(res)),
            ("samples", J::U(t.samples)),
            ("boundary", J::U(t.boundary)),
            ("allThree", J::U(t.all_three)),
            ("share", J::N(share)),
        ]));
    }
    println!("    two regions can meet along a line; three meet only at points. No straight seam");
    println!("    separates three basins, which is the wall Cayley published and could not pass.");
    claims += 1;

    // the pictures
    let q = dy::basin_grid(301, 2.0, dy::newton2, &rs2);
    let cu = dy::basin_grid(301, 2.0, dy::newton3, &dy::roots3());
    let pq = dy::ascii(&q, 25);
    let pc = dy::ascii(&cu, 25);
    println!("\n  two roots                        three roots");
    for k in 0..pq.len() {
        println!("  {}      {}", pq[k], pc[k]);
    }

    // the correction to v0's verdict
    println!("\n  the correction to eggSo-v0's verdict, measured against the right family:");
    let mut rows = Vec::new();
    for n in [4usize, 8, 16, 32, 64, 128] {
        let l = n * n;
        let f = fold::separation(&[n * (n - 1) / 2, n, n * (n - 1) / 2]);
        let two = fold::separation(&[l - l / 2, l / 2, 0]);
        let three = fold::separation(&[l / 3, l - 2 * (l / 3), l / 3]);
        println!(
            "    n={n:<4} fold {f:.4}   fair two-way {two:.4} (fold +{:.2} pts)   fair three-way {three:.4}",
            100.0 * (f - two)
        );
        rows.push(obj(&[
            ("n", J::U(n)),
            ("fold", J::N(f)),
            ("fairTwoWay", J::N(two)),
            ("fairThreeWay", J::N(three)),
            ("foldMarginOverTwoWay", J::N(f - two)),
            ("foldShareOfSquare", J::N(1.0 / n as f64)),
        ]));
    }
    println!("    the Fold's share is n/n^2 = 1/n and vanishes as the grid grows. A partition class");
    println!("    keeps its share; a basin boundary has measure zero. That is the fingerprint.");
    claims += 1;

    let _ = record(
        "basins",
        &obj(&[
            ("coordinate", obj(&[("cells", J::U(cells)), ("exceptions", J::U(bad))])),
            ("quadratic", obj(&[("tested", J::U(tested)), ("wrong", J::U(wrong))])),
            ("cubicStability", J::A(stability)),
            ("separation", J::A(rows)),
            ("pictureTwoRoots", J::A(pq.iter().map(|s| J::s(s)).collect())),
            ("pictureThreeRoots", J::A(pc.iter().map(|s| J::s(s)).collect())),
        ]),
    );
    claims
}

// ---- Part 2: the guess-and-fix decoder ----------------------------------

fn cmd_guess() -> usize {
    use guess::{Cfg, Rule, Scope};
    println!("PART 2 -- can a decoder guess and fix? The site's divider does; a decoder cannot.");
    let c = Code::new(32, true, "fold", code::fold_assign);
    let trials = 400usize;

    // the table baseline: v0 with the amendment
    let mut g = Mul32::new(20260903);
    let mut table_ok = 0usize;
    for _ in 0..trials {
        let clean = g.cells(c.l);
        let check = c.checks_for(&clean);
        let mut h = clean.clone();
        h[g.pick(c.l)] ^= 1;
        let r = repair(&mut h, &check, &c, &Opts::new());
        if r.status == Status::Corrected && h == clean {
            table_ok += 1;
        }
    }
    println!("\n  GF-0, the table (v0 + the amendment), singles: {table_ok}/{trials} in ONE lookup");

    // GF-1 on singles, by budget: the geometric law
    let mut budget_rows = Vec::new();
    println!("  GF-1, blind restoring guess-and-fix, singles, by probe budget:");
    for b in [32usize, 496, 1024, 4096, 16384] {
        let mut g = Mul32::new(4242);
        let mut ok = 0usize;
        let mut evals = 0usize;
        for _ in 0..trials {
            let clean = g.cells(c.l);
            let check = c.checks_for(&clean);
            let mut h = clean.clone();
            h[g.pick(c.l)] ^= 1;
            let mut cfg = Cfg::new(Rule::ZeroClass, Scope::Square);
            cfg.budget = b;
            let tr = guess::decode(&mut h, &check, &c, &cfg, &[], &mut g, Some(&clean));
            evals += tr.syndrome_evals;
            if tr.exact {
                ok += 1;
            }
        }
        let law = 1.0 - (1.0 - 1.0 / c.l as f64).powi(b as i32);
        println!(
            "    budget {b:<6} {ok}/{trials} = {:<7} the law 1-(1-1/L)^B says {:.3}   mean {} evals",
            pct(ok, trials),
            law,
            evals / trials
        );
        budget_rows.push(obj(&[
            ("budget", J::U(b)),
            ("corrected", J::U(ok)),
            ("of", J::U(trials)),
            ("geometricLaw", J::N(law)),
            ("meanEvals", J::U(evals / trials)),
        ]));
    }

    // the rest of the ladder, on the channel that decides it: a same-class
    // double. GF-2 narrows the proposals to the hurt class; GF-3a/b/c try to
    // use the only metric Z_p offers.
    println!("\n  the ladder on a same-class double, 4096 probes each:");
    let ladder: Vec<(&str, Rule, Scope, usize)> = vec![
        ("GF-1  zero / square", Rule::ZeroClass, Scope::Square, 1),
        ("GF-2  zero / hurt", Rule::ZeroClass, Scope::Hurt, 1),
        ("GF-3a ring strict", Rule::RingStrict, Scope::Hurt, 1),
        ("GF-3b ring sideways", Rule::RingSideways, Scope::Hurt, 8),
        ("GF-3c anneal", Rule::Anneal, Scope::Hurt, 8),
    ];
    let mut ladder_rows = Vec::new();
    for (name, rule, scope, restarts) in &ladder {
        let mut g = Mul32::new(808);
        let (mut ok, mut consistent_wrong) = (0usize, 0usize);
        let mut evals = 0usize;
        for _ in 0..trials {
            let clean = g.cells(c.l);
            let check = c.checks_for(&clean);
            let mut h = clean.clone();
            let k = g.pick(3);
            let m = &c.members[k];
            let a = m[g.pick(m.len())];
            let mut b = m[g.pick(m.len())];
            while b == a {
                b = m[g.pick(m.len())];
            }
            h[a] ^= 1;
            h[b] ^= 1;
            let mut cfg = Cfg::new(*rule, *scope);
            cfg.restarts = *restarts;
            let tr = guess::decode(&mut h, &check, &c, &cfg, &[], &mut g, Some(&clean));
            evals += tr.syndrome_evals;
            if tr.exact {
                ok += 1;
            } else if tr.consistent {
                consistent_wrong += 1;
            }
        }
        println!(
            "    {name:<22} exact {:>6}   consistent-but-WRONG {:>3}   mean {} evals",
            pct(ok, trials),
            consistent_wrong,
            evals / trials
        );
        ladder_rows.push(obj(&[
            ("arm", J::s(name)),
            ("exact", J::U(ok)),
            ("consistentButWrong", J::U(consistent_wrong)),
            ("of", J::U(trials)),
            ("meanEvals", J::U(evals / trials)),
        ]));
    }
    println!("    consistent-but-WRONG is the failure that matters: guess-and-fix cannot express");
    println!("    ambiguity, so it halts at the first square that satisfies the checks and calls it");
    println!("    done. v0 refuses these. That is the honest detection this decoder cannot buy.");

    // the census: the plateau claim, corrected
    let mut g = Mul32::new(31);
    let mut single_census = 0usize;
    let mut double_census = 0usize;
    let mut reaches_clean = 0usize;
    for _ in 0..trials {
        let clean = g.cells(c.l);
        let check = c.checks_for(&clean);
        let mut one = clean.clone();
        one[g.pick(c.l)] ^= 1;
        single_census += guess::accepting_census(&one, &check, &c);
        let mut two = clean.clone();
        let k = g.pick(3);
        let m = &c.members[k];
        let a = m[g.pick(m.len())];
        let mut b = m[g.pick(m.len())];
        while b == a {
            b = m[g.pick(m.len())];
        }
        two[a] ^= 1;
        two[b] ^= 1;
        double_census += guess::accepting_census(&two, &check, &c);
        for i in 0..c.l {
            let mut t = two.clone();
            t[i] ^= 1;
            if t == clean {
                reaches_clean += 1;
            }
        }
    }
    println!("\n  the accepting-move census, exhaustive over all 1024 cells:");
    println!(
        "    a single error:        {:.3} accepting moves -- injectivity leaves exactly one, and it is the answer",
        single_census as f64 / trials as f64
    );
    println!(
        "    a same-class double:   {:.3} accepting moves -- PREDICTIONS said 0 and called it a theorem. MISSED.",
        double_census as f64 / trials as f64
    );
    println!("      Injectivity separates the 2L values from each other and says nothing about a SUM");
    println!("      of two landing on a third. The rate is |class|/p, one direction per cell because");
    println!("      a cell's current bit fixes which way its flip moves the syndrome.");
    println!(
        "    accepting moves that reach the clean square: {reaches_clean} -- one flip cannot undo two errors,"
    );
    println!("      so every accepting move on a double is the ALIAS that v0 added q to refuse.");

    // the gradient
    let mut g = Mul32::new(41);
    let (mut d1t, mut d1n) = (0usize, 0usize);
    let (mut d1w, mut d1wn) = (0usize, 0usize);
    let (mut d2t, mut d2n) = (0usize, 0usize);
    let (mut d2w, mut d2wn) = (0usize, 0usize);
    let inner = c.members[0].clone();
    for _ in 0..trials {
        let clean = g.cells(c.l);
        let check = c.checks_for(&clean);
        let a = inner[g.pick(inner.len())];
        let mut one = clean.clone();
        one[a] ^= 1;
        let st = guess::State::of(&one, &check, &c);
        let base = st.ring_sum(c.p);
        // wrong cells are sampled from the HURT class only. Sampling the whole
        // square flatters the metric badly: a flip in an untouched class moves
        // that class's syndrome off zero and always raises the ring sum, so it
        // never descends and the comparison becomes meaningless.
        for &i in &inner {
            let mut t = one.clone();
            t[i] ^= 1;
            let after = guess::State::of(&t, &check, &c).ring_sum(c.p);
            if i == a {
                d1n += 1;
                if after < base {
                    d1t += 1;
                }
            } else if g.pick(40) == 0 {
                d1wn += 1;
                if after < base {
                    d1w += 1;
                }
            }
        }
        let x = inner[g.pick(inner.len())];
        let mut y = inner[g.pick(inner.len())];
        while y == x {
            y = inner[g.pick(inner.len())];
        }
        let mut two = clean.clone();
        two[x] ^= 1;
        two[y] ^= 1;
        let st2 = guess::State::of(&two, &check, &c);
        let base2 = st2.ring_sum(c.p);
        for &i in &inner {
            let mut t = two.clone();
            t[i] ^= 1;
            let after = guess::State::of(&t, &check, &c).ring_sum(c.p);
            if i == x || i == y {
                d2n += 1;
                if after < base2 {
                    d2t += 1;
                }
            } else if g.pick(40) == 0 {
                d2wn += 1;
                if after < base2 {
                    d2w += 1;
                }
            }
        }
    }
    let r = |a: usize, b: usize| if b == 0 { 0.0 } else { a as f64 / b as f64 };
    println!("\n  the gradient -- does 'closer in Z_p' point anywhere?");
    println!(
        "    one error:  the true cell descends {:.3}, a wrong cell {:.3}",
        r(d1t, d1n),
        r(d1w, d1wn)
    );
    println!(
        "    two errors: the true cells descend {:.3}, a wrong cell {:.3}",
        r(d2t, d2n),
        r(d2w, d2wn)
    );
    println!("    The metric detects the answer and does not point toward it. Its range is one step.");
    println!("    A division remainder shrinks at every step; this is a coin everywhere but the answer.");

    // the count arm
    let mut g = Mul32::new(53);
    let (mut counts_met, mut exact, mut consistent) = (0usize, 0usize, 0usize);
    for _ in 0..trials {
        let clean = g.cells(c.l);
        let check = guess::count_checks(&clean, &c);
        let mut h = clean.clone();
        let k = g.pick(3);
        let m = &c.members[k];
        let a = m[g.pick(m.len())];
        let mut b = m[g.pick(m.len())];
        while b == a {
            b = m[g.pick(m.len())];
        }
        h[a] ^= 1;
        h[b] ^= 1;
        let tr = guess::decode_count(&mut h, &check, &c, 100_000, &mut g, Some(&clean));
        if guess::count_checks(&h, &c)[..3] == check[..3] {
            counts_met += 1;
        }
        if tr.consistent {
            consistent += 1;
        }
        if tr.exact {
            exact += 1;
        }
    }
    println!("\n  GF-5, the count arm: per-class popcounts instead of residues,");
    println!(
        "    {} bits against the residues' {} -- cheaper, not subsidised.",
        guess::count_bits(&c),
        c.check_bits()
    );
    println!(
        "    same-class doubles: counts reached {}, fully consistent {}, EXACT {}",
        pct(counts_met, trials),
        pct(consistent, trials),
        pct(exact, trials)
    );
    println!("    PREDICTIONS said the count arm would CLEAR this channel. MISSED, and the reason");
    println!("    finishes the thesis: a count says how many and never which, so the search converges");
    println!("    in the count's own terms and lands on the wrong square, which q then refuses.");
    println!("    For the same ~33 bits you buy an ADDRESS or a METRIC. The address makes search");
    println!("    unnecessary; the metric makes it converge to the wrong answer. Neither buys a decoder.");

    // where a blind guess does win
    let mut g = Mul32::new(67);
    let mut flagged_ok = 0usize;
    for _ in 0..trials {
        let clean = g.cells(c.l);
        let check = c.checks_for(&clean);
        let mut h = clean.clone();
        let f: Vec<usize> = (0..3).map(|k| c.members[k][g.pick(c.members[k].len())]).collect();
        for &i in &f {
            h[i] = 0;
        }
        let mut cfg = Cfg::new(Rule::ZeroClass, Scope::Flagged);
        cfg.budget = 64;
        let tr = guess::decode(&mut h, &check, &c, &cfg, &f, &mut g, Some(&clean));
        if tr.exact {
            flagged_ok += 1;
        }
    }
    println!("\n  GF-6, one flagged erasure per class, 64 probes: {}", pct(flagged_ok, trials));
    println!("    the one place a blind guess ties the table -- and it does it with NO table at all,");
    println!("    because one equation and one unknown is a solve wearing a guess's clothes.");

    let _ = record(
        "guess",
        &obj(&[
            ("tableSingles", obj(&[("corrected", J::U(table_ok)), ("of", J::U(trials))])),
            ("gf1ByBudget", J::A(budget_rows)),
            (
                "census",
                obj(&[
                    ("singleMean", J::N(single_census as f64 / trials as f64)),
                    ("doubleMean", J::N(double_census as f64 / trials as f64)),
                    ("acceptingMovesReachingClean", J::U(reaches_clean)),
                    ("predicted", J::s("0, called a theorem -- MISSED, the rate is |class|/p")),
                ]),
            ),
            (
                "gradient",
                obj(&[
                    ("oneErrorTrue", J::N(r(d1t, d1n))),
                    ("oneErrorWrong", J::N(r(d1w, d1wn))),
                    ("twoErrorsTrue", J::N(r(d2t, d2n))),
                    ("twoErrorsWrong", J::N(r(d2w, d2wn))),
                ]),
            ),
            (
                "countArm",
                obj(&[
                    ("bits", J::U(guess::count_bits(&c))),
                    ("residueBits", J::U(c.check_bits())),
                    ("countsReached", J::U(counts_met)),
                    ("fullyConsistent", J::U(consistent)),
                    ("exact", J::U(exact)),
                    ("of", J::U(trials)),
                ]),
            ),
            ("ladder", J::A(ladder_rows)),
            ("flaggedErasure", obj(&[("exact", J::U(flagged_ok)), ("of", J::U(trials))])),
        ]),
    );
    6
}

// ---- Part 3: the chosen seam --------------------------------------------

fn cmd_seam(full: bool) -> usize {
    use seam::Channel::*;
    println!("PART 3 -- what the fold's forced 1/n seam costs against a chosen one.");
    let arms: Vec<(String, Code, &'static str)> = seam::seams()
        .into_iter()
        .map(|s| (s.name.to_string(), Code::new(32, true, s.name, s.assign), s.note))
        .collect();
    let trials = if full { 800 } else { 400 };

    println!("\n  every arm pays the same {} check bits, so every difference below is pure geometry.", arms[0].1.check_bits());
    println!("  {:<9} {:>18} {:>10}   note", "arm", "classes", "separation");
    for (name, c, note) in &arms {
        let s = c.sizes();
        println!(
            "  {name:<9} {:>18} {:>10.4}   {note}",
            format!("{}/{}/{}", s[0], s[1], s[2]),
            fold::separation(&s)
        );
    }

    let channels = vec![
        One,
        TwoAnywhere,
        TwoSameClass,
        TwoDifferentClasses,
        ThreeOnePerClass,
        RowBurstFlagged(12),
        RowBurstBlind(12),
        AntiDiagonal,
        ThinnestClass,
    ];
    println!("\n  corrected of {trials}, with miscorrections as /nW, and `direct` in brackets");
    print!("  {:<28}", "channel");
    for (name, _, _) in &arms {
        print!("{:>12}", name);
    }
    println!();
    let mut rows = Vec::new();
    for ch in &channels {
        print!("  {:<28}", ch.label());
        let mut cells = Vec::new();
        for (name, c, _) in &arms {
            let t = seam::run_channel(c, *ch, trials, 900 + ch.label().len() as u32);
            let cell = if t.wrong > 0 {
                format!("{}/{}W", t.corrected, t.wrong)
            } else {
                format!("{}", t.corrected)
            };
            print!("{:>12}", cell);
            cells.push(obj(&[
                ("arm", J::s(name)),
                ("corrected", J::U(t.corrected)),
                ("detected", J::U(t.detected)),
                ("wrong", J::U(t.wrong)),
                ("direct", J::U(t.direct)),
                ("classMaxMean", J::N(t.class_max_mean())),
            ]));
        }
        println!();
        rows.push(obj(&[("channel", J::s(&ch.label())), ("arms", J::A(cells))]));
    }

    // the burst sweep: at 12 cells every arm wins and the channel cannot discriminate
    println!("\n  the flagged row burst, swept -- at 12 cells every arm wins, so a single length");
    println!("  measures nothing. The breaking point and its reason are what price the geometry.");
    let lengths = vec![12usize, 15, 18, 24, 31];
    print!("  {:<28}", "burst length");
    for b in &lengths {
        print!("{:>8}", b);
    }
    println!();
    let mut sweep = Vec::new();
    for (name, c, _) in &arms {
        print!("  {name:<28}");
        let mut per = Vec::new();
        for (b, t) in seam::burst_breaking_point(c, &lengths, trials / 2, 1700) {
            print!("{:>8}", t.corrected);
            per.push(obj(&[
                ("burst", J::U(b)),
                ("corrected", J::U(t.corrected)),
                ("of", J::U(t.trials)),
                ("wrong", J::U(t.wrong)),
                ("classMaxMean", J::N(t.class_max_mean())),
                (
                    "notes",
                    J::A(t.notes.iter().map(|(n, k)| obj(&[("note", J::s(n)), ("count", J::U(*k))])).collect()),
                ),
            ]));
        }
        println!();
        sweep.push(obj(&[("arm", J::s(name)), ("lengths", J::A(per))]));
    }

    println!("\n  idx3's clean sweep belongs to one residue class and is reported as such:");
    for n in [31usize, 32, 33] {
        println!("    n = {n} ({} mod 3): idx3 {}", n % 3, seam::idx3_identity(n));
    }

    let _ = record(
        "seam",
        &obj(&[
            (
                "arms",
                J::A(
                    arms.iter()
                        .map(|(name, c, note)| {
                            let s = c.sizes();
                            obj(&[
                                ("arm", J::s(name)),
                                ("note", J::s(note)),
                                ("classes", J::A(s.iter().map(|&v| J::U(v)).collect())),
                                ("separation", J::N(fold::separation(&s))),
                                ("checkBits", J::U(c.check_bits())),
                            ])
                        })
                        .collect(),
                ),
            ),
            ("channels", J::A(rows)),
            ("burstSweep", J::A(sweep)),
        ]),
    );
    arms.len()
}
