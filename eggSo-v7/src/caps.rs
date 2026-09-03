//! caps.rs -- the harness that isolates one cap at a time.
//!
//! eggSo-v5 ended by finding that the walls in this construction are four
//! fixed constants inherited from eggSo-v0, not the geometry. This round asks
//! which of them are ARTIFACTS that can be raised and which are INFORMATION
//! BOUNDS that cannot, and the difference has to be measured rather than
//! argued, because they look identical from the outside: both come back as
//! `Detected`.
//!
//! Going through `seam.rs`'s burst channels would confound the two, because a
//! burst's length and its distribution across classes move together. So the
//! harness here flags an EXACT number of cells in an EXACT distribution --
//! `[f0, f1, f2]` -- which is the only way to put the derived bounds
//!
//! ```text
//! F <~ 3*log2(p) + log2(q)   = check_bits    (spread)
//! F <~   log2(p) + log2(q)                   (all F in one class)
//! ```
//!
//! against a measurement. See `code::Caps` for the derivation and
//! `PREDICTIONS.md` for it stated before any of this ran.
//!
//! The four outcomes are kept apart, and that separation is the whole point:
//!
//!   * **corrected** -- the square came back exactly right;
//!   * **ambiguous** -- more than one reading satisfied every check, which is
//!     what an INFORMATION limit looks like from inside the decoder;
//!   * **refused** -- a cap stopped the enumeration, which is what a BUDGET
//!     limit looks like;
//!   * **wrong** -- a single reading satisfied every check and was not the
//!     original. This must stay 0 at every cap setting or the round has
//!     failed, because raising a budget must never convert a refusal into a
//!     lie.

use std::time::Instant;

use crate::code::{repair, Caps, Code, Mul32, Opts, Status};

#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub trials: usize,
    pub corrected: usize,
    /// the decoder found several readings and said so
    pub ambiguous: usize,
    /// a cap stopped it, or no reading satisfied the checks
    pub refused: usize,
    /// it committed to a single wrong reading -- must be 0
    pub wrong: usize,
    pub micros: u128,
    /// which refusal notes fired, so a budget stop is distinguishable from a
    /// genuine dead end
    pub notes: Vec<(&'static str, usize)>,
}

impl Outcome {
    pub fn rate(&self) -> f64 {
        if self.trials == 0 {
            0.0
        } else {
            self.corrected as f64 / self.trials as f64
        }
    }
    pub fn micros_each(&self) -> u128 {
        if self.trials == 0 {
            0
        } else {
            self.micros / self.trials as u128
        }
    }
    fn note(&mut self, n: &'static str) {
        for e in self.notes.iter_mut() {
            if e.0 == n {
                e.1 += 1;
                return;
            }
        }
        self.notes.push((n, 1));
    }
    pub fn note_list(&self) -> String {
        if self.notes.is_empty() {
            return "-".into();
        }
        let mut v = self.notes.clone();
        v.sort_by_key(|e| std::cmp::Reverse(e.1));
        v.iter().map(|(n, k)| format!("{n} x{k}")).collect::<Vec<_>>().join(", ")
    }
}

/// Flag exactly `per_class[k]` cells of class `k`, then decode under `caps`.
///
/// `None` when the code has no room for the requested distribution, which is
/// an absent measurement and never a zero.
pub fn flagged_trial(
    code: &Code,
    caps: Caps,
    per_class: [usize; 3],
    trials: usize,
    seed: u32,
) -> Option<Outcome> {
    for (k, &want) in per_class.iter().enumerate() {
        if want > code.members[k].len() {
            return None;
        }
    }
    let mut g = Mul32::new(seed);
    let mut out = Outcome { trials, ..Default::default() };
    for _ in 0..trials {
        let clean = g.cells(code.l);
        let check = code.checks_for(&clean);
        let mut hurt = clean.clone();
        let mut flagged = Vec::new();
        for (k, &want) in per_class.iter().enumerate() {
            let m = &code.members[k];
            let mut picked: Vec<usize> = Vec::with_capacity(want);
            while picked.len() < want {
                let i = m[g.pick(m.len())];
                if !picked.contains(&i) {
                    picked.push(i);
                }
            }
            for &i in &picked {
                hurt[i] = -1;
                flagged.push(i);
            }
        }
        let opts = Opts::erased(&flagged).with_caps(caps);
        let t = Instant::now();
        let r = repair(&mut hurt, &check, code, &opts);
        out.micros += t.elapsed().as_micros();
        match r.status {
            Status::Corrected | Status::Clean => {
                if hurt == clean {
                    out.corrected += 1;
                } else {
                    out.wrong += 1;
                }
            }
            Status::Ambiguous => {
                out.ambiguous += 1;
                out.note(r.note);
            }
            Status::Detected => {
                out.refused += 1;
                out.note(r.note);
            }
        }
    }
    Some(out)
}

/// `F` erasures split as evenly as the three classes allow.
pub fn spread(f: usize) -> [usize; 3] {
    let base = f / 3;
    let rem = f % 3;
    [base + usize::from(rem > 0), base + usize::from(rem > 1), base]
}

/// All `F` erasures in one class, the other two clean.
pub fn concentrated(f: usize) -> [usize; 3] {
    [f, 0, 0]
}

/// Caps raised far enough that only the arithmetic can stop the decoder.
///
/// `erasures_per_class` is left to the caller because it is the one being
/// swept; everything else is lifted clear so that a refusal means the
/// information ran out rather than a secondary budget.
pub fn generous(erasures_per_class: usize) -> Caps {
    Caps {
        erasures_per_class,
        erasure_hits: 1 << 22,
        erasure_readings: 1 << 22,
        pair_candidates: 1 << 20,
        pc_combos: 1 << 22,
        // v7's guard on, because these caps are raised and anything raised
        // must be safe. `Caps::v0()` is the only place it is off.
        refuse_on_truncation: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seam;

    fn diag3(n: usize) -> Code {
        seam::seams().into_iter().find(|s| s.name == "diag3").unwrap().code(n, true)
    }

    /// The distributions are what they claim to be, or every number in the
    /// round is about the wrong experiment.
    #[test]
    fn the_distributions_hold_their_counts() {
        for f in 1..=30usize {
            assert_eq!(spread(f).iter().sum::<usize>(), f, "spread({f})");
            assert_eq!(concentrated(f).iter().sum::<usize>(), f, "concentrated({f})");
            // spread really is even to within one
            let s = spread(f);
            assert!(s.iter().max().unwrap() - s.iter().min().unwrap() <= 1, "spread({f}) = {s:?}");
            assert_eq!(concentrated(f)[1], 0);
            assert_eq!(concentrated(f)[2], 0);
        }
    }

    /// The harness flags exactly the cells it says it does, in the classes it
    /// says, and flags them rather than flipping them.
    #[test]
    fn the_harness_flags_the_right_cells() {
        let c = diag3(32);
        let mut g = Mul32::new(1);
        for per in [[3usize, 0, 0], [2, 2, 2], [5, 1, 0]] {
            let mut hurt = g.cells(c.l);
            let mut flagged = Vec::new();
            for (k, &want) in per.iter().enumerate() {
                let m = &c.members[k];
                let mut picked: Vec<usize> = Vec::new();
                while picked.len() < want {
                    let i = m[g.pick(m.len())];
                    if !picked.contains(&i) {
                        picked.push(i);
                    }
                }
                for &i in &picked {
                    hurt[i] = -1;
                    flagged.push(i);
                }
            }
            assert_eq!(flagged.len(), per.iter().sum::<usize>());
            let mut seen = [0usize; 3];
            for &i in &flagged {
                assert_eq!(hurt[i], -1, "cell {i} was not flagged");
                seen[c.class[i] as usize] += 1;
            }
            assert_eq!(seen, per, "the distribution came out wrong");
        }
    }

    /// C1's half of the story: at v0's caps the harness reproduces what v0
    /// does -- one flagged cell per class is recovered every time.
    #[test]
    fn one_per_class_is_recovered_at_v0_caps() {
        let c = diag3(32);
        let o = flagged_trial(&c, Caps::v0(), [1, 1, 1], 200, 13).unwrap();
        assert_eq!(o.corrected, 200, "{o:?}");
        assert_eq!(o.wrong, 0);
    }

    /// C6, and it is the bar that would sink the round: raising a cap must
    /// never convert a refusal into a LIE. Checked across the sweep, at both
    /// distributions, well past the point where the decoder starts failing.
    #[test]
    fn raising_a_cap_never_creates_a_wrong_answer() {
        let c = diag3(32);
        for f in [6usize, 12, 18, 24, 30] {
            for per in [spread(f), concentrated(f)] {
                if per[0] > 20 {
                    continue; // 2^21 enumeration per trial is not a unit test
                }
                let o = flagged_trial(&c, generous(20), per, 30, 20260903).unwrap();
                assert_eq!(o.wrong, 0, "F={f} as {per:?} produced {} lies", o.wrong);
                assert_eq!(
                    o.corrected + o.ambiguous + o.refused,
                    o.trials,
                    "outcomes do not add up at F={f}"
                );
            }
        }
    }

    /// An impossible request is absent, not zero.
    #[test]
    fn a_distribution_that_does_not_fit_says_so() {
        let c = diag3(8);
        let biggest = c.members[0].len();
        assert!(flagged_trial(&c, Caps::v0(), [biggest, 0, 0], 2, 1).is_some());
        assert!(flagged_trial(&c, Caps::v0(), [biggest + 1, 0, 0], 2, 1).is_none());
    }

    /// The default really is v0's, which is what keeps `pin::v0_decisions`
    /// honest while the caps are a parameter.
    #[test]
    fn the_default_caps_are_v0s() {
        assert_eq!(Caps::default(), Caps::v0());
        assert_eq!(Opts::new().caps, Caps::v0());
        assert_eq!(Opts::erased(&[1, 2]).caps, Caps::v0());
        assert_eq!(Opts::after_plan().caps, Caps::v0());
        let v0 = Caps::v0();
        assert_eq!(v0.erasures_per_class, 16);
        assert_eq!(v0.erasure_hits, 64);
        assert_eq!(v0.erasure_readings, 8192);
        assert_eq!(v0.pair_candidates, 4096);
    }

    /// The two bounds, computed from the code's own moduli, land where
    /// PREDICTIONS.md said before any measurement: 44.0 and 22.0 at n = 32.
    #[test]
    fn the_bounds_are_where_the_derivation_puts_them() {
        let c = diag3(32);
        assert_eq!(c.p, 2053);
        assert_eq!(c.q, 2063);
        let spread_b = Caps::spread_bound(&c);
        let conc_b = Caps::concentrated_bound(&c);
        assert!((spread_b - 44.0).abs() < 0.1, "spread bound {spread_b}");
        assert!((conc_b - 22.0).abs() < 0.1, "concentrated bound {conc_b}");
        // and the stored check bits round up past the real information
        assert_eq!(c.check_bits(), 48);
        assert!(spread_b < c.check_bits() as f64);
        // v0's cap sits ABOVE the spread bound once tripled, and BELOW the
        // concentrated one -- redundant in one regime, an artifact in the other
        assert!(3.0 * Caps::v0().erasures_per_class as f64 > spread_b);
        assert!((Caps::v0().erasures_per_class as f64) < conc_b);
    }
}
