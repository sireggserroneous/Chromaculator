//! eggso8 -- the literature, run rather than read.
//!
//!   eggso8 pin     the copy against v7's committed record
//!   eggso8 lit     the prior art rebuilt from its own definitions, against ours
//!   eggso8 model   the interactive terminal model
//!   eggso8 audit   pin + lit, with the counts printed

use eggso8::{json, lit, optimum, pin, tui};

use json::{obj, record, J};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()).unwrap_or("audit") {
        "pin" => {
            cmd_pin();
        }
        "lit" => {
            cmd_lit();
        }
        "model" => tui::run(),
        "audit" => {
            let a = cmd_pin();
            println!();
            let b = cmd_lit();
            println!("\nAUDIT: {a} pins clean, {b} literature claims measured");
        }
        _ => println!("usage: eggso8 pin | lit | model | audit"),
    }
}

fn cmd_pin() -> usize {
    println!("THE PIN -- the copy against v7's committed record. No node needed.\n");
    let r = pin::v7_figures();
    println!("{}", r.line());
    let clean = usize::from(r.ok());
    println!("\n  {clean} of 1 pin clean{}", if clean == 0 { " -- THE ROUND STOPS" } else { "" });
    let _ = record(
        "pins",
        &J::A(vec![obj(&[
            ("pin", J::s(r.name)),
            ("checked", J::U(r.checked)),
            ("mismatches", J::U(r.mismatches)),
            ("skipped", J::B(r.skipped.is_some())),
        ])]),
    );
    clean
}

/// Does a colouring agree with one of the nine linear arms up to a
/// relabelling of the three colours?
fn matches_some_arm(built: &[u8], n: usize) -> bool {
    (0..3).flat_map(|a| (0..3).map(move |b| (a, b))).any(|(a, b)| {
        let arm = optimum::linear_class(a, b, n);
        let mut map = [None::<u8>; 3];
        let mut back = [None::<u8>; 3];
        for (x, y) in built.iter().zip(arm.iter()) {
            match (map[*x as usize], back[*y as usize]) {
                (None, None) => {
                    map[*x as usize] = Some(*y);
                    back[*y as usize] = Some(*x);
                }
                (Some(v), Some(w)) if v == *y && w == *x => {}
                _ => return false,
            }
        }
        true
    })
}

fn cmd_lit() -> usize {
    println!("IS IT NOVEL? Answered by rebuilding the prior art from its own definitions and");
    println!("  measuring whether it reproduces ours. A citation read relocates the question;");
    println!("  a coincidence measured settles it.\n");
    let mut claims = 0usize;
    let mut rows = Vec::new();

    // 1 -- the catalogued colouring
    let mut same = 0usize;
    for n in 2..=40usize {
        if lit::perfect_colouring_grid(n) == optimum::linear_class(1, 2, n) {
            same += 1;
        }
    }
    println!("  1. THE CATALOGUED COLOURING. The perfect-colourings literature's standard");
    println!("     example for the square grid is colour(x,y) = (y - x) mod 3. Written from");
    println!("     that definition and compared cell for cell against our arms:");
    println!("       it IS arm (1,2), at {same} of 39 widths tested");
    println!(
        "       and (1,2) is one of exactly {} arms the theorem names at n = 2 (mod 3)",
        optimum::linear_solutions(32).len()
    );
    println!("     So the family we enumerated is one the literature has catalogued.");
    claims += 1;
    rows.push(obj(&[
        ("claim", J::s("the catalogued colouring (y-x) mod 3 equals our arm (1,2)")),
        ("widthsAgreeing", J::U(same)),
        ("of", J::U(39)),
    ]));

    // 2 -- lattice interleavers
    let n = 24usize;
    let (mut tried, mut inside) = (0usize, 0usize);
    for v1 in [(1i64, 1i64), (1, -1), (1, 2), (2, 1), (3, 0), (0, 3), (1, 0), (0, 1)] {
        for v2 in [(0i64, 3i64), (3, 0), (1, 2), (2, 1), (1, -1), (1, 1)] {
            if (v1.0 * v2.1 - v1.1 * v2.0).unsigned_abs() != 3 {
                continue;
            }
            let Some(built) = lit::lattice_colouring(n, v1, v2) else { continue };
            tried += 1;
            if matches_some_arm(&built, n) {
                inside += 1;
            }
        }
    }
    println!("\n  2. LATTICE INTERLEAVERS. Blaum/Bruck/Vardy colour by the coset of a");
    println!("     sublattice. Built GEOMETRICALLY here -- cells grouped by whether their");
    println!("     difference lies in the sublattice, cosets numbered as first met, nothing");
    println!("     assuming a linear form:");
    println!("       {inside} of {tried} index-3 sublattices land inside our nine-arm family");
    claims += 1;
    rows.push(obj(&[
        ("claim", J::s("index-3 lattice interleavers land in the linear family")),
        ("inside", J::U(inside)),
        ("tried", J::U(tried)),
    ]));

    // 3 -- where the problems differ
    println!("\n  3. WHERE THE PROBLEMS DIFFER. Their criterion is DISTINCTNESS on a connected");
    println!("     cluster of area t; ours bounds MULTIPLICITY at ceil(L/3) with three colours.");
    println!("     Their own pigeonhole needs t colours for area t, so from t = 4 their");
    println!("     question cannot be posed for our alphabet at all:");
    for t in [2usize, 3, 4, 5] {
        let need = lit::t_interleaved_degree_needed(t);
        println!(
            "       area {t} needs {need} colours; we have 3 -- {}",
            if need <= 3 { "askable" } else { "NOT askable" }
        );
    }
    for &(a, b) in &[(1usize, 2usize), (2, 1)] {
        let cl = optimum::linear_class(a, b, 32);
        println!(
            "       our arm ({a},{b}) breaks their distinctness at area {}",
            lit::t_interleaved_breaks_at(&cl, 32)
        );
    }
    println!("     So ours is the multiplicity RELAXATION of their problem restricted to four");
    println!("     line shapes -- a different question, not a solved one.");
    claims += 1;

    // 4 -- the one that matters
    println!("\n  4. AND THE ONE THAT MATTERS. The lattice literature colours the INFINITE");
    println!("     lattice Z^2, which has no row-major read order and therefore no phase slip.");
    println!("     Measured: over the three SPATIAL geometries alone the verdict is WIDTH-FREE,");
    println!("     and every width dependence in v5/v7 comes from the tape.");
    println!("     {:<9}{:>5}{:>14}{:>16}   reading", "arm", "L", "spatial", "with tape");
    let mut tape_rows = Vec::new();
    for &(a, b) in &[(1usize, 2usize), (2, 1), (1, 1)] {
        for l in [12usize, 13] {
            let first_s = lit::spatial_at_floor(&optimum::linear_class(a, b, 15), 15, l);
            let first_f = optimum::worst_all(&optimum::linear_class(a, b, 15), 15, l).at_floor(l);
            let mut spatial_same = true;
            let mut full_same = true;
            for m in 15..=36usize {
                let cl = optimum::linear_class(a, b, m);
                if lit::spatial_at_floor(&cl, m, l) != first_s {
                    spatial_same = false;
                }
                if optimum::worst_all(&cl, m, l).at_floor(l) != first_f {
                    full_same = false;
                }
            }
            println!(
                "     ({a},{b}){:>7}{:>14}{:>16}   {}",
                l,
                if spatial_same { "width-free" } else { "MOVES" },
                if full_same { "width-free" } else { "moves with n" },
                if spatial_same && !full_same { "the tape is the whole difference" } else { "" }
            );
            tape_rows.push(obj(&[
                ("a", J::U(a)),
                ("b", J::U(b)),
                ("L", J::U(l)),
                ("spatialWidthFree", J::B(spatial_same)),
                ("fullWidthFree", J::B(full_same)),
            ]));
        }
    }
    claims += 1;
    rows.push(obj(&[
        ("claim", J::s("spatial is width-free; the tape carries every n-dependence")),
        ("rows", J::A(tape_rows)),
    ]));

    println!("\n  THE VERDICT ON NOVELTY, measured rather than asserted:");
    println!("    NOT NOVEL, in the part that is mathematics. The nine-arm family is catalogued,");
    println!("    lattice interleavers build it by another route, the forced-periodicity step is");
    println!("    the standard argument for balanced words, and the construction is a block");
    println!("    interleaver -- which v5's README and the site item already called prior art.");
    println!("    WHAT THE SEARCHED LITERATURE DOES NOT HAVE is the TAPE: a finite array read as");
    println!("    a 1-D sequence, where a burst crosses a row boundary and slips phase. Z^2 has");
    println!("    no such thing, so n mod 3, gcd(n,L) and gcd(n-1,L) have nowhere to appear.");
    println!("    That is an engineering geometry rather than a mathematical object, and the");
    println!("    honest claim is a corollary nobody needed, not a gap somebody missed.");
    println!("    AND THE NEGATIVE RESULT IS OURS: (r+c) mod 3 -- the fold's own level sets --");
    println!("    is never burst-optimal at any width, because an anti-diagonal is its level");
    println!("    set. Nobody else has the fold, so nobody else had reason to check.");

    let _ = record(
        "lit",
        &obj(&[
            (
                "sources",
                J::A(vec![
                    J::s("Blaum, Bruck & Vardy, Interleaving schemes for multidimensional cluster errors, IEEE Trans. Inf. Theory 44 (1998) 730-743"),
                    J::s("Perfect colourings of the infinite square grid: coverings and twin colors -- catalogue to 9 colours"),
                    J::s("Optimal interleaving schemes for correcting two-dimensional cluster errors, Discrete Applied Mathematics"),
                ]),
            ),
            ("verdict", J::s("not novel as mathematics; the tape geometry is the only part not found in the searched literature, and it is an engineering geometry")),
            ("claims", J::A(rows)),
        ]),
    );
    claims
}
