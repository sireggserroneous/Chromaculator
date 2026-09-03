//! thirds.rs -- the burst floor, characterised for every `L`.
//!
//! eggSo-v5 settled `L` divisible by 3: the tape window at the floor has no
//! slack, which forces the tape `L`-periodic, and then a coset count decides
//! it -- a partition reaching the floor on all four geometries exists exactly
//! when `3 | L/gcd(n,L)` and `3 | L/gcd(n-1,L)`. It left `L` not divisible by
//! 3 open, with three cases INCONCLUSIVE.
//!
//! **The other two residues come out of one piece of arithmetic.** A tape run
//! of `L` cells crossing a row boundary is two arithmetic progressions with a
//! phase slip: `m` cells before the boundary and `L - m` after. For a linear
//! partition with `b` nonzero, an AP of length `t` over `Z/3` puts at most
//! `ceil(t/3)` cells in one class, so the run's worst class is bounded by
//! `ceil(m/3) + ceil((L-m)/3)`. Then:
//!
//! ```text
//! L = 3t     m not divisible by 3 gives t+1     the slip costs 1 -- v5's lemma
//! L = 3t+1   every split gives at most t+1      THE SLIP IS ALWAYS ABSORBED
//! L = 3t+2   m = 1 gives t+2                    conditional
//! ```
//!
//! So at `L = 1 (mod 3)` the tape condition is **vacuous**: the phase slip
//! cannot cost anything, the four conditions collapse to three, and
//! `{a != 0, b != 0, a != b}` is satisfiable at every `n`. At `L = 2 (mod 3)`
//! it is conditional, and that is where the open cases live.
//!
//! `linear_verdict` states the whole characterisation and `characterise`
//! re-derives it by measurement rather than quoting it.
//!
//! The three remaining cases -- `(30,8)`, `(33,8)`, `(33,11)` -- all sit at
//! `n = 0 (mod 3)` with `L = 2 (mod 3)`, the one cell of the table where NO
//! linear arm exists. `search` goes after them, and it can only ever say YES:
//! see its own note.

use crate::code::Mul32;
use crate::optimum::{floor_of, worst_all, Geom, GEOMS};

/// Which `n` admit a linear partition at the floor, for each `L mod 3`.
///
/// The `L = 0` row is v5's theorem; the other two are this round's. Stated as
/// a closed form so the suite can confront it with a measurement.
pub fn linear_verdict(n: usize, l: usize) -> bool {
    match l % 3 {
        0 => n % 3 == 2,
        1 => true,
        _ => !n.is_multiple_of(3),
    }
}

/// One row of the confrontation: what the closed form says against what every
/// one of the nine `(a,b)` actually measures.
#[derive(Clone, Debug)]
pub struct Row {
    pub n: usize,
    pub l: usize,
    pub predicted: bool,
    pub measured: bool,
    /// the arms that reached the floor on all four geometries
    pub arms: Vec<(usize, usize)>,
}

impl Row {
    pub fn agrees(&self) -> bool {
        self.predicted == self.measured
    }
}

pub fn characterise(ns: &[usize], ls: &[usize]) -> Vec<Row> {
    let mut out = Vec::new();
    for &n in ns {
        for &l in ls {
            if l > n {
                continue; // a run that does not fit leaves geometries vacuous
            }
            let mut arms = Vec::new();
            for a in 0..3 {
                for b in 0..3 {
                    let cl = crate::optimum::linear_class(a, b, n);
                    if worst_all(&cl, n, l).at_floor(l) {
                        arms.push((a, b));
                    }
                }
            }
            out.push(Row {
                n,
                l,
                predicted: linear_verdict(n, l),
                measured: !arms.is_empty(),
                arms,
            });
        }
    }
    out
}

/// The vacuity at `L = 1 (mod 3)`, shown rather than asserted: the tape
/// reaches the floor for EVERY arm with `b` nonzero, so the tape condition
/// adds nothing at that residue.
pub fn tape_is_vacuous(n: usize, l: usize) -> bool {
    (0..3)
        .flat_map(|a| (0..3).map(move |b| (a, b)))
        .filter(|&(_, b)| b % 3 != 0)
        .all(|(a, b)| {
            let cl = crate::optimum::linear_class(a, b, n);
            crate::optimum::worst_of(&cl, n, l, Geom::Tape) == Some(floor_of(l))
        })
}

// ---- the search ---------------------------------------------------------

/// **Row windows are redundant, and dropping them is exact.**
///
/// When `L <= n`, a row window of `L` consecutive cells IS `L` consecutive
/// row-major indices, so every row constraint is already a tape constraint.
/// The search therefore carries tape, column and anti-diagonal only, which is
/// about a quarter fewer windows for free. Proved by
/// `row_windows_are_tape_windows`.
pub fn windows_without_rows(n: usize, l: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for g in GEOMS {
        if g == Geom::Row && l <= n {
            continue;
        }
        out.extend(crate::optimum::windows(n, l, g));
    }
    out
}

#[derive(Clone, Debug)]
pub enum Found {
    /// an exhibited partition, re-verified from scratch by `worst_all`
    Reached { class: Vec<u8>, restarts: usize, nodes: u64 },
    /// the budget ran out. NOT a proof of impossibility, and never printed
    /// as one
    Inconclusive { restarts: usize, nodes: u64 },
}

/// A randomised-restart depth-first search for a partition at the floor.
///
/// **This method can only ever say YES.** Settling a case positively needs one
/// partition and a construction is its own proof, so shuffling the value
/// order per restart costs nothing in rigour. It is NOT complete and cannot
/// return "impossible" -- a case it fails to settle comes back
/// `Inconclusive`, which is neither a construction nor a proof. v5's `exact`
/// remains the complete method for the cases small enough to finish.
///
/// The walk is v5's: cells in row-major order, class labels restricted so
/// first occurrences run `0,1,2`, pruned the instant any window's class count
/// exceeds the floor. Counts only grow, so the prune is sound. What is new is
/// the per-restart value order and the redundant-row reduction.
pub fn search(n: usize, l: usize, restarts: usize, nodes_per: u64, seed: u32) -> Found {
    let win = windows_without_rows(n, l);
    let floor = floor_of(l);
    let mut touch = vec![Vec::new(); n * n];
    for (w, cells) in win.iter().enumerate() {
        for &i in cells {
            touch[i].push(w as u32);
        }
    }

    let mut g = Mul32::new(seed);
    let mut total = 0u64;

    for r in 0..restarts {
        // the value order this restart tries first, shuffled per restart
        let mut order = [0u8, 1, 2];
        for i in (1..3).rev() {
            let j = g.pick(i + 1);
            order.swap(i, j);
        }
        let mut cnt = vec![[0usize; 3]; win.len()];
        let mut class = vec![0u8; n * n];
        let mut nodes = 0u64;

        struct Ctx<'a> {
            touch: &'a [Vec<u32>],
            floor: usize,
            order: [u8; 3],
            cap: u64,
        }

        fn go(
            j: usize,
            used: usize,
            ctx: &Ctx,
            cnt: &mut [[usize; 3]],
            class: &mut [u8],
            nodes: &mut u64,
        ) -> Option<bool> {
            if j == class.len() {
                return Some(true);
            }
            for idx in 0..3 {
                // canonical labelling still applies: a class may only be used
                // if a lower-numbered one already has been
                let k = ctx.order[idx] as usize;
                if k > used.min(2) {
                    continue;
                }
                *nodes += 1;
                if *nodes > ctx.cap {
                    return None;
                }
                let t = &ctx.touch[j];
                if t.iter().any(|&w| cnt[w as usize][k] + 1 > ctx.floor) {
                    continue;
                }
                for &w in t {
                    cnt[w as usize][k] += 1;
                }
                class[j] = k as u8;
                let next_used = if k == used { used + 1 } else { used };
                match go(j + 1, next_used, ctx, cnt, class, nodes) {
                    Some(true) => return Some(true),
                    None => return None,
                    Some(false) => {}
                }
                for &w in t {
                    cnt[w as usize][k] -= 1;
                }
            }
            Some(false)
        }

        let ctx = Ctx { touch: &touch, floor, order, cap: nodes_per };
        let got = go(0, 0, &ctx, &mut cnt, &mut class, &mut nodes);
        total += nodes;
        if got == Some(true) {
            // re-verified from scratch, with the ROW windows back in, by the
            // same function every other measurement in the lineage uses
            let w = worst_all(&class, n, l);
            assert!(
                w.at_floor(l),
                "the search returned a partition at {:?}, not the floor {}",
                w.overall(),
                floor
            );
            return Found::Reached { class, restarts: r + 1, nodes: total };
        }
    }
    Found::Inconclusive { restarts, nodes: total }
}

/// **Exhaustive over the tape-periodic family**, which the exhibited
/// partitions pointed at: `(15,11)` and `(30,11)` both came back periodic
/// with period 11, so the family is worth searching directly.
///
/// A partition of the form `class(j) = g(j mod P)` is determined by `3^P`
/// choices of `g`, which is nothing for `P` up to about 14 -- against `3^(n^2)`
/// for the grid. Canonical labelling cuts it by six. This is **complete
/// within the family**, so a negative result here is real but partial: it
/// says "no tape-periodic solution with period at most `max_p`", never
/// "no solution".
///
/// v5's periodicity lemma FORCES this form when `3 | L`. At the other two
/// residues it is not forced, which is exactly why finding solutions in it
/// is a finding rather than a tautology.
///
/// **One structural exclusion, which explains why `(33,11)` is the hard
/// one.** If `P` divides `n`, every row starts at the same phase, so every
/// row carries an identical class pattern and every COLUMN is constant --
/// putting all `L` cells of a column burst in one class. So a period `P` with
/// `P | n` can never work, and at `n = 33` that rules out `P = 1, 3, 11, 33`
/// -- including the period 11 that settles `(15,11)` and `(30,11)`.
/// `period_divides_n_is_hopeless` pins it.
pub fn search_periodic(n: usize, l: usize, max_p: usize) -> Option<(usize, Vec<u8>)> {
    let floor = floor_of(l);
    for p in 1..=max_p {
        if p > 15 {
            break; // 3^16 and up is no longer free
        }
        if n.is_multiple_of(p) {
            continue; // every row identical, so every column constant
        }
        let total = 3usize.pow(p as u32);
        let mut g = vec![0u8; p];
        for code in 0..total {
            let mut v = code;
            let mut used = 0usize;
            let mut canonical = true;
            for slot in g.iter_mut() {
                let k = (v % 3) as u8;
                v /= 3;
                // first occurrences must run 0,1,2
                if k as usize > used {
                    canonical = false;
                    break;
                }
                if k as usize == used {
                    used += 1;
                }
                *slot = k;
            }
            if !canonical {
                continue;
            }
            // A cheap NECESSARY pre-filter before building any grid: the
            // tape windows start at every index `0..n*n-L`, so their phases
            // cover all `P` residues, and a period-`P` tape window starting
            // at phase `t` is just `g[t..t+L]` read cyclically. If any phase
            // busts the floor the candidate is dead, and this costs `P*L`
            // instead of a whole `n^2` grid plus every window in it.
            let tape_ok = (0..p).all(|t| {
                let mut per = [0usize; 3];
                for x in 0..l {
                    per[g[(t + x) % p] as usize] += 1;
                }
                *per.iter().max().unwrap() <= floor
            });
            if !tape_ok {
                continue;
            }
            let class: Vec<u8> = (0..n * n).map(|j| g[j % p]).collect();
            if worst_all(&class, n, l).at_floor(l) {
                return Some((p, class));
            }
        }
    }
    None
}

/// Whether an exhibited partition is tape-periodic with any period up to
/// `max_p`. The filed prediction is that the `n = 0 (mod 3)`, `L = 2 (mod 3)`
/// solutions are NOT, since a periodic one would be `g(j mod P)` and that
/// family is exactly the one the cell has none of.
pub fn tape_period(class: &[u8], max_p: usize) -> Option<usize> {
    (1..=max_p).find(|&p| class.iter().skip(p).zip(class.iter()).all(|(a, b)| a == b))
}

/// Whether an exhibited partition is one of the nine linear arms.
pub fn is_linear(class: &[u8], n: usize) -> Option<(usize, usize)> {
    (0..3)
        .flat_map(|a| (0..3).map(move |b| (a, b)))
        .find(|&(a, b)| crate::optimum::linear_class(a, b, n) == class)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact reduction the search rests on: when `L <= n`, every row
    /// window is a tape window, so carrying rows separately is redundant.
    #[test]
    fn row_windows_are_tape_windows() {
        for n in [12usize, 15, 16, 30, 33] {
            for l in [4usize, 8, 11, 12] {
                if l > n {
                    continue;
                }
                let tape: std::collections::HashSet<Vec<usize>> =
                    crate::optimum::windows(n, l, Geom::Tape).into_iter().collect();
                for w in crate::optimum::windows(n, l, Geom::Row) {
                    assert!(tape.contains(&w), "row window {w:?} is not a tape window at n={n}, L={l}");
                }
                // and the reduction really does drop them
                let full = GEOMS.iter().map(|&g| crate::optimum::windows(n, l, g).len()).sum::<usize>();
                let cut = windows_without_rows(n, l).len();
                assert!(cut < full, "the reduction dropped nothing at n={n}, L={l}");
                assert_eq!(cut, full - crate::optimum::windows(n, l, Geom::Row).len());
            }
        }
    }

    /// T2. The closed form against the measurement, over a wide sweep and all
    /// three residues. This is the round's characterisation and it is
    /// re-derived here rather than quoted.
    #[test]
    fn the_characterisation_holds_at_every_residue() {
        let ns: Vec<usize> = (8..=36).collect();
        let ls: Vec<usize> = (3..=18).collect();
        let rows = characterise(&ns, &ls);
        assert!(rows.len() > 300, "only {} cases", rows.len());
        let bad: Vec<&Row> = rows.iter().filter(|r| !r.agrees()).collect();
        assert!(
            bad.is_empty(),
            "{} disagreements, first: n={} L={} predicted {} measured {}",
            bad.len(),
            bad[0].n,
            bad[0].l,
            bad[0].predicted,
            bad[0].measured
        );
        // and all three residues are actually exercised
        for res in 0..3 {
            assert!(rows.iter().any(|r| r.l % 3 == res), "no L = {res} (mod 3) case");
        }
    }

    /// The mechanism behind the `L = 1 (mod 3)` row, shown rather than
    /// asserted: at that residue the phase slip cannot cost anything, so the
    /// tape condition is vacuous for every arm with `b` nonzero.
    #[test]
    fn the_tape_condition_is_vacuous_at_l_one_mod_three() {
        for l in [4usize, 7, 10, 13, 16] {
            for n in 15..=30usize {
                if l > n {
                    continue;
                }
                assert!(tape_is_vacuous(n, l), "the tape bit at n={n}, L={l}");
            }
        }
        // and it is NOT vacuous at the other two residues, or the claim would
        // be empty
        assert!(!tape_is_vacuous(30, 12), "L=12 should not be vacuous");
        assert!(!tape_is_vacuous(30, 8), "L=8 should not be vacuous");
    }

    /// The three open cases sit in exactly one cell of the table, and the
    /// closed form says that cell has no linear arm. That is what makes them
    /// a nonlinear question.
    #[test]
    fn the_open_cases_are_the_cell_with_no_linear_arm() {
        for (n, l) in [(30usize, 8usize), (33, 8), (33, 11), (15, 8), (15, 11), (30, 11)] {
            assert_eq!(n % 3, 0, "n={n}");
            assert_eq!(l % 3, 2, "L={l}");
            assert!(!linear_verdict(n, l), "n={n}, L={l} should admit no linear arm");
            let rows = characterise(&[n], &[l]);
            assert!(rows[0].arms.is_empty(), "n={n}, L={l} measured arms {:?}", rows[0].arms);
        }
    }

    /// The search settles a case it should, and its answer is a construction
    /// that survives independent re-verification.
    #[test]
    fn the_search_finds_what_the_complete_method_found() {
        // (15,8) is the case v6's exact enumeration reached
        match search(15, 8, 4, 5_000_000, 20260903) {
            Found::Reached { class, .. } => {
                assert!(worst_all(&class, 15, 8).at_floor(8));
                assert_eq!(class.len(), 225);
            }
            Found::Inconclusive { .. } => panic!("(15,8) is known reachable"),
        }
    }

    /// The periodic family is complete within itself, and it reproduces the
    /// periodic solutions the grid search stumbled on.
    #[test]
    fn the_periodic_family_finds_the_periodic_solutions() {
        // (15,11) and (30,11) came back period-11 from the grid search
        for (n, l) in [(15usize, 11usize), (30, 11)] {
            let got = search_periodic(n, l, 12);
            assert!(got.is_some(), "no periodic solution at n={n}, L={l}");
            let (p, class) = got.unwrap();
            assert!(worst_all(&class, n, l).at_floor(l), "n={n}, L={l} period {p}");
        }
        // and where v5's lemma forbids it outright, the family is empty
        assert!(search_periodic(30, 12, 12).is_none(), "(30,12) is impossible by the lemma");
        assert!(search_periodic(15, 6, 12).is_none(), "(15,6) is impossible by the lemma");
    }

    /// A period that divides `n` is hopeless, and this is why `(33,11)` is
    /// the hard case: it rules out the period 11 that settles `(15,11)` and
    /// `(30,11)`.
    #[test]
    fn period_divides_n_is_hopeless() {
        for (n, l, p) in [(33usize, 11usize, 11usize), (30, 8, 10), (15, 8, 5)] {
            assert!(n.is_multiple_of(p));
            let class: Vec<u8> = (0..n * n).map(|j| ((j % p) % 3) as u8).collect();
            // every row identical => every column constant => a column burst
            // lands entirely in one class
            let w = crate::optimum::worst_of(&class, n, l, Geom::Col);
            assert_eq!(w, Some(l), "n={n}, L={l}, P={p}: column worst {w:?}");
            assert!(!worst_all(&class, n, l).at_floor(l));
        }
        // and 33 is a multiple of 11, which 30 is not -- the whole difference
        assert!(33usize.is_multiple_of(11));
        assert!(!30usize.is_multiple_of(11));
    }

    /// A randomised search must never be read as a proof of impossibility,
    /// so the type does not offer one.
    #[test]
    fn the_search_cannot_say_impossible() {
        // a budget of one node cannot settle anything, and comes back
        // Inconclusive rather than claiming a negative
        match search(30, 8, 1, 1, 7) {
            Found::Inconclusive { .. } => {}
            Found::Reached { .. } => panic!("one node should not settle (30,8)"),
        }
    }

    /// The structure helpers do what the README claims of the exhibited
    /// partitions.
    #[test]
    fn the_structure_helpers_detect_what_they_claim() {
        let n = 16usize;
        let lin = crate::optimum::linear_class(1, 2, n);
        assert_eq!(is_linear(&lin, n), Some((1, 2)));
        // j mod 3 is tape-periodic with period 3 by construction
        let idx3: Vec<u8> = (0..n * n).map(|j| (j % 3) as u8).collect();
        assert_eq!(tape_period(&idx3, 12), Some(3));
        // and something with no short period reports none
        let mut odd = idx3.clone();
        odd[100] = (odd[100] + 1) % 3;
        assert_eq!(tape_period(&odd, 12), None);
    }

    /// The energy view and the window view agree about the reduced set, which
    /// is what lets the search prune on one and report on the other.
    #[test]
    fn the_reduction_does_not_change_any_verdict() {
        for (n, l) in [(15usize, 8usize), (16, 8), (30, 11), (12, 4)] {
            for a in 0..3 {
                for b in 0..3 {
                    let cl = crate::optimum::linear_class(a, b, n);
                    let full = worst_all(&cl, n, l).at_floor(l);
                    // the reduced set, evaluated directly
                    let reduced = windows_without_rows(n, l).iter().all(|w| {
                        let mut per = [0usize; 3];
                        for &i in w {
                            per[cl[i] as usize] += 1;
                        }
                        *per.iter().max().unwrap() <= floor_of(l)
                    });
                    assert_eq!(full, reduced, "({a},{b}) at n={n}, L={l}");
                }
            }
        }
        // and the v5 Energy object still agrees at zero with `at_floor`
        let e = crate::optimum::Energy::new(15, 8);
        let cl = crate::optimum::linear_class(1, 1, 15);
        assert_eq!(e.energy_of(&cl) == 0, worst_all(&cl, 15, 8).at_floor(8));
    }
}
