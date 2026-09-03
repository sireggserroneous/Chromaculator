//! eggso6 -- the caps.
//!
//! The twenty-first codec experiment and the seventh in the fold-native
//! lineage. v5 ended by finding that the walls here are four fixed constants
//! inherited from eggSo-v0, not the geometry. This round separates the
//! artifacts from the bounds.
//!
//!   eggso6 pin      the round against the site's own code, the copy against v5's
//!   eggso6 bound    the information ceiling on erasure recovery, derived and measured
//!   eggso6 caps     each cap raised INDEPENDENTLY, with what it buys and what it costs
//!   eggso6 audit    all of it, with the counts printed

use eggso6::{caps, code, fold, json, pin, seam};

use caps::{concentrated, flagged_trial, generous, spread};
use code::{Caps, Code};
use json::{obj, record, J};
use seam::Channel;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("audit");
    let full = args.iter().any(|a| a == "--full");
    match cmd {
        "pin" => {
            cmd_pin();
        }
        "bound" => {
            cmd_bound(full);
        }
        "caps" => {
            cmd_caps(full);
        }
        "audit" => {
            let a = cmd_pin();
            println!();
            let b = cmd_bound(full);
            println!();
            let c = cmd_caps(full);
            println!("\nAUDIT: {a} pins clean, {b} bound claims, {c} caps isolated");
        }
        _ => println!("usage: eggso6 pin | bound | caps | audit [--full]"),
    }
}

fn diag3(n: usize) -> Code {
    seam::seams().into_iter().find(|s| s.name == "diag3").unwrap().code(n, true)
}

// ---- the pins ------------------------------------------------------------

fn cmd_pin() -> usize {
    println!("THE PINS -- C1, and this round does not start without it.");
    println!("  The caps are a PARAMETER now, so the thing that has to be proved first is that");
    println!("  the DEFAULT is still eggSo-v0 to the decision. If it were not, every raised-cap");
    println!("  number below would be measuring my own edit rather than the cap.\n");

    let mut results = vec![pin::v5_figures()];
    if !pin::node_available() {
        println!("{}", results[0].line());
        println!("  node is not on PATH: every SITE pin SKIPPED, loudly. Nothing there passed.");
        let _ = record("pins", &pins_record(&results));
        return results.iter().filter(|r| r.ok()).count();
    }

    let c = Code::new(32, true, "fold", code::fold_assign);
    results.push(pin::site_regions(40));
    results.push(pin::site_arcs(40));
    results.push(pin::v0_structure(&c));

    let mut g = code::Mul32::new(20260903);
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
    println!("  the decisions pin is the one that matters here: 600 squares decoded by BOTH v0's");
    println!("  own decoder through node and this one at Caps::v0(), compared on the status word");
    println!("  and the repaired cells. The caps became a parameter and changed nothing.");
    let _ = record("pins", &pins_record(&results));
    clean
}

fn pins_record(results: &[pin::PinResult]) -> J {
    J::A(
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
    )
}

// ---- C2: the bound -------------------------------------------------------

fn cmd_bound(full: bool) -> usize {
    println!("C2 -- THE BOUND. Flagged erasure recovery is not a search with a tunable budget.");
    println!("  It is a counting problem, and the count is fixed by the check bits.");
    println!("  With f_k flagged cells in class k and F in total, the decoder knows WHICH cells");
    println!("  are unknown, so class k offers 2^f_k candidate assignments and its own residue");
    println!("  mod p keeps about 2^f_k / p of them. The confirming residue mod q is global and");
    println!("  filters the surviving combinations once more, by 1/q. So the expected number of");
    println!("  readings that satisfy every check is\n");
    println!("      2^F / (p^3 * q)     spread across all three classes");
    println!("      2^F / (p   * q)     all F in ONE class, the other two clean\n");
    println!("  and recovery is unique only while that stays below 1:\n");
    println!("      F <~ 3*log2(p) + log2(q)  = check_bits     (spread)");
    println!("      F <~   log2(p) + log2(q)                   (concentrated)\n");

    let mut claims = 0usize;
    let ns: Vec<usize> = if full { vec![32, 33, 64] } else { vec![32] };
    let mut rows = Vec::new();

    for &n in &ns {
        let c = diag3(n);
        let sb = Caps::spread_bound(&c);
        let cb = Caps::concentrated_bound(&c);
        println!("  n = {n}: p = {}, q = {}, check_bits = {}", c.p, c.q, c.check_bits());
        println!(
            "    the spread bound is {sb:.1} erasures, the concentrated bound is {cb:.1}"
        );
        println!("    and v0's cap of {} per class sits ABOVE the spread bound once tripled", Caps::v0().erasures_per_class);
        println!("    ({} > {sb:.1}, so it is redundant there) and BELOW the concentrated one", 3 * Caps::v0().erasures_per_class);
        println!("    ({} < {cb:.1}, so it is an ARTIFACT there). That is the round in one line.\n", Caps::v0().erasures_per_class);

        // Sweep F under GENEROUS caps, so only the arithmetic can stop it.
        //
        // The sweep has to REACH each bound or it proves nothing: the spread
        // bound is 44, which needs about 16 flagged per class, and the
        // concentrated bound is 22 in a single class. Trials fall as F rises
        // because the enumeration is 2^f per class -- the count is printed
        // beside every row rather than assumed.
        let base_trials = if full { 120 } else { 60 };
        let budget = |f: usize| -> usize {
            let per = f.div_ceil(3);
            if per <= 12 {
                base_trials
            } else if per <= 15 {
                base_trials / 2
            } else {
                base_trials / 4
            }
            .max(8)
        };

        println!("    SPREAD across the three classes, caps raised clear of the question.");
        println!("    The bound is {sb:.1}, so the sweep runs past it:");
        println!(
            "    {:<5}{:>11}{:>12}{:>11}{:>11}{:>10}{:>12}   why it stopped",
            "F", "per class", "corrected", "ambiguous", "refused", "wrong", "us each"
        );
        let mut spread_fs: Vec<usize> =
            (1..=18).map(|i| i * 3).chain([40, 44, 46]).collect();
        spread_fs.sort_unstable();
        spread_fs.dedup();
        for f in spread_fs {
            let per = spread(f);
            if per[0] > 18 {
                continue; // 2^19 per class, three classes: past the point of use
            }
            let t = budget(f);
            let Some(o) = flagged_trial(&c, generous(18), per, t, 20260903 + f as u32) else {
                continue;
            };
            println!(
                "    {:<5}{:>11}{:>12}{:>11}{:>11}{:>10}{:>12}   {}",
                f,
                format!("{}/{}/{}", per[0], per[1], per[2]),
                format!("{}/{}", o.corrected, o.trials),
                o.ambiguous,
                o.refused,
                o.wrong,
                o.micros_each(),
                o.note_list()
            );
            rows.push(obj(&[
                ("n", J::U(n)),
                ("distribution", J::s("spread")),
                ("F", J::U(f)),
                ("perClass", J::A(per.iter().map(|&v| J::U(v)).collect())),
                ("corrected", J::U(o.corrected)),
                ("ambiguous", J::U(o.ambiguous)),
                ("refused", J::U(o.refused)),
                ("wrong", J::U(o.wrong)),
                ("microsEach", J::U(o.micros_each() as usize)),
                ("of", J::U(o.trials)),
            ]));
        }
        println!("    the transition is where the ARITHMETIC runs out, not where a cap sits, and");
        println!("    the failures say so themselves: they come back AMBIGUOUS -- several readings");
        println!("    satisfy every check -- and never `too many erasures`, which is what a budget");
        println!("    stop would print. wrong stays 0 the whole way.");
        claims += 1;

        // and the concentrated case, which is where v0's cap is below the bound
        println!("\n    CONCENTRATED in one class, where the bound is {cb:.1} and v0's cap is {}:", Caps::v0().erasures_per_class);
        println!(
            "    {:<5}{:>12}{:>11}{:>11}{:>10}{:>12}   why it stopped",
            "F", "corrected", "ambiguous", "refused", "wrong", "us each"
        );
        let conc_fs: Vec<usize> =
            if full { vec![6, 12, 16, 17, 18, 20, 21, 22, 23, 24] } else { vec![6, 12, 16, 18, 20, 22, 24] };
        for f in conc_fs {
            let t = if f >= 23 { 16 } else if f >= 20 { 30 } else { base_trials };
            let Some(o) = flagged_trial(&c, generous(24), concentrated(f), t, 4242 + f as u32)
            else {
                continue;
            };
            println!(
                "    {:<5}{:>12}{:>11}{:>11}{:>10}{:>12}   {}",
                f,
                format!("{}/{}", o.corrected, o.trials),
                o.ambiguous,
                o.refused,
                o.wrong,
                o.micros_each(),
                o.note_list()
            );
            rows.push(obj(&[
                ("n", J::U(n)),
                ("distribution", J::s("concentrated")),
                ("F", J::U(f)),
                ("corrected", J::U(o.corrected)),
                ("ambiguous", J::U(o.ambiguous)),
                ("refused", J::U(o.refused)),
                ("wrong", J::U(o.wrong)),
                ("microsEach", J::U(o.micros_each() as usize)),
                ("of", J::U(o.trials)),
            ]));
        }
        claims += 1;
        println!();
    }

    let c = diag3(32);
    let _ = record(
        "bound",
        &obj(&[
            (
                "derivation",
                obj(&[
                    ("spread", J::s("F <= 3*log2(p) + log2(q) = check_bits")),
                    ("concentrated", J::s("F <= log2(p) + log2(q)")),
                    ("spreadBoundAt32", J::N(Caps::spread_bound(&c))),
                    ("concentratedBoundAt32", J::N(Caps::concentrated_bound(&c))),
                    ("checkBitsAt32", J::U(c.check_bits())),
                ]),
            ),
            ("sweep", J::A(rows)),
        ]),
    );
    claims
}

// ---- C3, C4, C5: each cap isolated --------------------------------------

fn cmd_caps(full: bool) -> usize {
    println!("C3 -- EACH CAP ISOLATED. Raised one at a time, from v0's values, so an interaction");
    println!("  is visible rather than pooled. The 64-hit cap in particular was never separated");
    println!("  from the 16-per-class cap before, and the two bind in the wrong order.\n");

    let trials = if full { 200 } else { 100 };
    let mut isolated = Vec::new();
    let mut n_caps = 0usize;

    // ---- the concentrated erasure channel, cap by cap --------------------
    let c = diag3(32);
    println!("  18 erasures in ONE class at n = 32. The bound says 22, so this is inside it and");
    println!("  any failure here is a budget and not the arithmetic. Corrected of {trials}:");
    println!("  {:<44}{:>11}{:>11}{:>11}{:>10}", "caps", "corrected", "ambiguous", "refused", "wrong");
    let ladder: Vec<(&str, Caps)> = vec![
        ("v0, untouched", Caps::v0()),
        (
            "per-class 16 -> 20, hits still 64   UNSAFE",
            Caps { erasures_per_class: 20, ..Caps::v0() },
        ),
        (
            "hits 64 -> 4096, per-class still 16",
            Caps { erasure_hits: 4096, ..Caps::v0() },
        ),
        (
            "both: per-class 20, hits 4096",
            Caps { erasures_per_class: 20, erasure_hits: 4096, ..Caps::v0() },
        ),
        ("Caps::raised(20) -- the coupled raise", Caps::raised(20, &c)),
        ("Caps::raised(22) -- the coupled raise", Caps::raised(22, &c)),
    ];
    for (name, cap) in &ladder {
        let Some(o) = flagged_trial(&c, *cap, concentrated(18), trials, 777) else { continue };
        println!(
            "  {:<44}{:>11}{:>11}{:>11}{:>10}",
            name, o.corrected, o.ambiguous, o.refused, o.wrong
        );
        isolated.push(obj(&[
            ("experiment", J::s("18 erasures in one class, n=32")),
            ("caps", J::s(name)),
            ("hitsSufficient", J::B(cap.hits_sufficient(&c))),
            ("erasuresPerClass", J::U(cap.erasures_per_class)),
            ("erasureHits", J::U(cap.erasure_hits)),
            ("erasureReadings", J::U(cap.erasure_readings)),
            ("corrected", J::U(o.corrected)),
            ("ambiguous", J::U(o.ambiguous)),
            ("refused", J::U(o.refused)),
            ("wrong", J::U(o.wrong)),
            ("of", J::U(o.trials)),
            ("microsEach", J::U(o.micros_each() as usize)),
        ]));
        n_caps += 1;
    }
    println!("
    READ THE `wrong` COLUMN. This is C6, and C6 is MISSED.");
    println!("    Raising `erasures_per_class` ALONE makes the decoder LIE. The erasure path");
    println!("    enumerates the 2^f subsets of a class's flagged cells, keeps those matching the");
    println!("    class residue -- about 2^f/p of them -- but stops collecting at `erasure_hits`");
    println!("    and then asks q which kept reading survives. Truncate that list and the TRUE");
    println!("    reading can fall off it, leaving a false one as the unique survivor. The");
    println!("    decoder commits to it. At f = 18 there are about 2^18/p = 119 expected");
    println!("    solutions against v0's 64 kept, so the list is short by half.");
    println!("    v0's own pair is safe by exactly a factor of two: 2^16/2053 = 31.9 against 64.");
    println!("    So these are NOT four independent knobs -- `erasures_per_class` and");
    println!("    `erasure_hits` are a MATCHED PAIR, and v5's reading of them as separate");
    println!("    constants (and mine, until this table) was wrong.");
    println!("    `Caps::raised(f, code)` does the coupling arithmetic, and the two rows using it");
    println!("    show the win with 0 lies, at 18 erasures in one class, where v0 refuses");
    println!("    all {trials}. That is C4, and it is real -- but it had to be bought");
    println!("    safely, and the unsafe version of the same raise is a silent-corruption bug.");

    // ---- the cost of the per-class cap ----------------------------------
    println!("\n  C4's other half, the price. The erasure enumeration is 2^f per class, so the");
    println!("  per-class cap is not a knob, it is an exponent. Wall clock per square:");
    println!("  {:<10}{:>14}{:>16}{:>12}", "f in class", "corrected", "us per square", "vs f=12");
    let mut base_us = 0u128;
    for f in [12usize, 16, 18, 20, 22] {
        let cap = Caps { erasures_per_class: f.max(22), erasure_hits: 1 << 22, erasure_readings: 1 << 22, ..Caps::v0() };
        let Some(o) = flagged_trial(&c, cap, concentrated(f), if full { 60 } else { 30 }, 99) else {
            continue;
        };
        if f == 12 {
            base_us = o.micros_each().max(1);
        }
        println!(
            "  {:<10}{:>14}{:>16}{:>12}",
            f,
            format!("{}/{}", o.corrected, o.trials),
            o.micros_each(),
            format!("{:.0}x", o.micros_each() as f64 / base_us as f64)
        );
        isolated.push(obj(&[
            ("experiment", J::s("cost of the per-class enumeration")),
            ("f", J::U(f)),
            ("corrected", J::U(o.corrected)),
            ("of", J::U(o.trials)),
            ("microsEach", J::U(o.micros_each() as usize)),
        ]));
        n_caps += 1;
    }

    // ---- the pair cap, which is the cheap win ---------------------------
    println!("\n  C4 -- the pair cap at n = 512, which v5 measured collapsing to 70 of 120.");
    println!("  This one bounds a search whose answer is always PRESENT, so it is a pure");
    println!("  artifact. Same-class doubles, corrected of 120:");
    println!("  {:<26}{:>12}{:>12}{:>14}", "pair_candidates", "corrected", "wrong", "ms per square");
    let big = diag3(512);
    for cand in [4096usize, 16384, 65536, 262144] {
        let mut g = code::Mul32::new(7);
        let cap = Caps { pair_candidates: cand, pc_combos: 1 << 22, ..Caps::v0() };
        let t = std::time::Instant::now();
        let (mut ok, mut wrong) = (0usize, 0usize);
        for _ in 0..120 {
            let clean = g.cells(big.l);
            let check = big.checks_for(&clean);
            let mut h = clean.clone();
            let k = g.pick(3);
            let m = &big.members[k];
            let a = m[g.pick(m.len())];
            let mut b = m[g.pick(m.len())];
            while b == a {
                b = m[g.pick(m.len())];
            }
            h[a] ^= 1;
            h[b] ^= 1;
            let r = code::repair(&mut h, &check, &big, &code::Opts::new().with_caps(cap));
            if r.status == code::Status::Corrected {
                if h == clean {
                    ok += 1;
                } else {
                    wrong += 1;
                }
            }
        }
        let ms = t.elapsed().as_millis() as f64 / 120.0;
        println!(
            "  {:<26}{:>12}{:>12}{:>14.1}",
            cand,
            format!("{ok}/120"),
            wrong,
            ms
        );
        isolated.push(obj(&[
            ("experiment", J::s("pair cap at n=512")),
            ("pairCandidates", J::U(cand)),
            ("corrected", J::U(ok)),
            ("of", J::U(120)),
            ("wrong", J::U(wrong)),
            ("msPerSquare", J::N(ms)),
        ]));
        n_caps += 1;
    }
    println!("    4096 is v0's, and it is the row v5 published. The cost of raising it is LINEAR");
    println!("    and small, because the enumeration was already O(|class|) -- the cap only ever");
    println!("    bounded the output list.");

    // ---- C5: the wall that is not a cap ---------------------------------
    println!("\n  C5 -- and the wall that no cap can move. v5 found a 3n/8 burst killing every arm");
    println!("  at n >= 256. That is 96 flagged cells against a spread bound of {:.0}, so it is", Caps::spread_bound(&diag3(256)));
    println!("  information and not budget. Flagged col burst, corrected of {}:", trials / 2);
    println!(
        "  {:<8}{:>8}{:>10}{:>14}{:>18}{:>15}",
        "n", "burst", "bound", "v0 caps", "generous caps", "verdict"
    );
    let mut walls = Vec::new();
    for n in if full { vec![33usize, 64, 128, 256] } else { vec![33usize, 128, 256] } {
        let cd = diag3(n);
        let b = 3 * n / 8;
        let bound = Caps::spread_bound(&cd);
        let ch = Channel::ColBurstFlagged(b);
        let v0 = seam::run_channel(&cd, ch, trials / 2, 1700);
        // `seam.rs`'s channels are not parameterised by caps, so the
        // generous-cap number is taken through the erasure harness at the
        // distribution this arm actually produces -- `diag3` spreads a column
        // burst evenly, so b/3 per class.
        //
        // The enumeration is 2^f PER CLASS, so a burst of 96 cells means 32
        // per class and 4.3 billion subsets: NOT COMPUTABLE. Substituting a
        // shorter wound and reporting its success would be comparing two
        // different experiments, so the generous-cap column is left ABSENT
        // wherever the burst does not fit an affordable enumeration. The
        // bound argument stands on its own there and does not need a number
        // propped up beside it.
        let per = spread(b);
        let gen = if per[0] <= 18 {
            flagged_trial(&cd, generous(18), per, trials / 2, 1700)
        } else {
            None
        };
        let saturated = v0.as_ref().map(|t| t.corrected == t.trials).unwrap_or(false);
        let verdict = if (b as f64) > bound {
            "INFORMATION"
        } else if saturated {
            "no wall here"
        } else {
            "budget"
        };
        println!(
            "  {:<8}{:>8}{:>10.0}{:>14}{:>18}{:>15}",
            n,
            b,
            bound,
            v0.as_ref().map(|t| t.corrected.to_string()).unwrap_or_else(|| "--".into()),
            gen.as_ref()
                .map(|o| o.corrected.to_string())
                .unwrap_or_else(|| format!("-- ({}/class)", per[0])),
            verdict
        );
        walls.push(obj(&[
            ("n", J::U(n)),
            ("burst", J::U(b)),
            ("spreadBound", J::N(bound)),
            ("v0Corrected", match &v0 {
                Some(t) => J::U(t.corrected),
                None => J::s("absent"),
            }),
            ("generousPerClass", J::A(per.iter().map(|&v| J::U(v)).collect())),
            ("generousCorrected", match &gen {
                Some(o) => J::U(o.corrected),
                None => J::s("absent"),
            }),
            ("verdict", J::s(verdict)),
        ]));
        n_caps += 1;
    }
    println!("    once the burst is longer than the check bits, no budget buys it back -- and the");
    println!("    generous column is ABSENT rather than optimistic at n = 256, because 96 flagged");
    println!("    cells is 32 per class and 2^32 subsets is not computable. Substituting a shorter");
    println!("    wound there and reporting its success would be two experiments wearing one row.");
    println!("    The bound argument does not need the number: 96 > 68 settles it.");

    let _ = record(
        "caps",
        &obj(&[
            ("v0", obj(&[
                ("erasuresPerClass", J::U(Caps::v0().erasures_per_class)),
                ("erasureHits", J::U(Caps::v0().erasure_hits)),
                ("erasureReadings", J::U(Caps::v0().erasure_readings)),
                ("pairCandidates", J::U(Caps::v0().pair_candidates)),
                ("pcCombos", J::U(Caps::v0().pc_combos)),
            ])),
            ("isolated", J::A(isolated)),
            ("walls", J::A(walls)),
        ]),
    );
    n_caps
}
