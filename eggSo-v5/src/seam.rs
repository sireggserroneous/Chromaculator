//! seam.rs -- what the fold's forced seam costs against a chosen one.
//!
//! eggSo-v0's published verdict is that the fold is "legitimate and
//! sub-optimal" as an interleaver: two random cells land in different classes
//! 0.5303 of the time against a fair three-way split's 0.6673. This module
//! builds v0's codec over an arbitrary partition and measures what that gap
//! is actually worth on real channels, because separation is a statistic
//! about random PAIRS and the thing storage delivers is a BURST.
//!
//! The seam is not a design choice for the fold. The anti-diagonal is `n`
//! cells of `n^2`, so its share is `1/n` and shrinks as the grid grows --
//! which is the measure-zero signature of a boundary rather than a class,
//! and the reason `fold.rs` reads the three regions as two basins and the
//! Julia set between them.
//!
//! CARRIED FROM eggSo-v4 with four changes, each of which v5 needs and v4
//! did not have:
//!
//!   1. **Three burst geometries did not exist and are written here.** v4's
//!      row burst is strictly intra-row by construction, its `AntiDiagonal`
//!      channel is the whole main anti-diagonal deterministically rather than
//!      a length-`L` run, and there was **no** column burst and **no** tape
//!      burst at all. Part 2 needs a length-`L` run in all four geometries.
//!   2. **`Tally` recorded a mean and not a worst case.** It carried
//!      `class_max_sum` over trials; Part 2 is a worst-case question, so
//!      `class_max_worst` is added beside it.
//!   3. **The seven assignment functions, `Tally::note` and `class_spread`
//!      were private.** They are `pub` here rather than re-declared in a
//!      second place where the two copies could drift.
//!   4. **A channel that cannot exist now says so.** v4's
//!      `burst_breaking_point` silently dropped any length `>= code.n`, which
//!      would have quietly hidden a Part 2 row rather than failing it. Every
//!      entry point returns an `Option` and the callers print the absence.
//!
//! Two arms cannot be a bare `fn` pointer -- the cubic partition and the
//! annealed one are tables, not formulae -- so `Arm` carries either, and
//! `Seam` owns its strings. That is the only shape change to the type.

use crate::code::{repair, Code, Mul32, Opts, Status};
use crate::fold;

/// A cell-to-class assignment, as either a rule or a precomputed table.
///
/// The linear and geometric arms are closed forms. The degree-3 arm is the
/// outcome of running Newton's method from every cell, and the searched arms
/// are the outcome of a search, so neither can be a `fn` pointer. A table is
/// bound to one `n` and `at` says so if it is asked about another.
pub enum Arm {
    Rule(fn(usize, usize, usize, usize) -> u8),
    Table { n: usize, class: Vec<u8> },
}

impl Arm {
    #[inline]
    pub fn at(&self, r: usize, c: usize, j: usize, n: usize) -> u8 {
        match self {
            Arm::Rule(f) => f(r, c, j, n),
            Arm::Table { n: tn, class } => {
                assert_eq!(*tn, n, "this arm is a table built for n={tn}, asked about n={n}");
                class[j]
            }
        }
    }
}

/// A named cell-to-class assignment.
pub struct Seam {
    pub name: String,
    pub note: String,
    pub arm: Arm,
}

impl Seam {
    pub fn rule(name: &str, note: &str, f: fn(usize, usize, usize, usize) -> u8) -> Seam {
        Seam { name: name.to_string(), note: note.to_string(), arm: Arm::Rule(f) }
    }
    pub fn table(name: &str, note: &str, n: usize, class: Vec<u8>) -> Seam {
        assert_eq!(class.len(), n * n, "a table arm needs exactly n*n entries");
        Seam { name: name.to_string(), note: note.to_string(), arm: Arm::Table { n, class } }
    }
    /// The codec over this arm. Every arm pays the same check bits, which is
    /// the fairness statement every comparison in this round rests on.
    pub fn code(&self, n: usize, confirm: bool) -> Code {
        Code::new(n, confirm, &self.name, |r, c, j, nn| self.arm.at(r, c, j, nn))
    }
    pub fn sizes(&self, n: usize) -> [usize; 3] {
        fold::class_sizes(n, |r, c, j, nn| self.arm.at(r, c, j, nn))
    }
    /// The whole grid's class array, which is what the burst geometries want.
    pub fn classes(&self, n: usize) -> Vec<u8> {
        (0..n * n).map(|j| self.arm.at(j / n, j % n, j, n)).collect()
    }
}

pub fn a_fold(r: usize, c: usize, _j: usize, n: usize) -> u8 {
    fold::region_of(r, c, n)
}
pub fn a_diag3(r: usize, c: usize, _j: usize, _n: usize) -> u8 {
    ((r + c) % 3) as u8
}
pub fn a_idx3(_r: usize, _c: usize, j: usize, _n: usize) -> u8 {
    (j % 3) as u8
}
pub fn a_rows3(r: usize, _c: usize, _j: usize, _n: usize) -> u8 {
    (r % 3) as u8
}
pub fn a_cols3(_r: usize, c: usize, _j: usize, _n: usize) -> u8 {
    (c % 3) as u8
}
pub fn a_blocks(_r: usize, _c: usize, j: usize, n: usize) -> u8 {
    let l = n * n;
    let third = l / 3;
    if j < third {
        0
    } else if j < 2 * third {
        1
    } else {
        2
    }
}
/// The fold's own shape with a widened seam: a band of anti-diagonals.
pub fn a_seam128(r: usize, c: usize, _j: usize, n: usize) -> u8 {
    let b = fold::band_of(r, c, n);
    // 128 cells at n = 32 is the four anti-diagonals nearest the fold
    if b < -2 {
        0
    } else if b <= 1 {
        1
    } else {
        2
    }
}

pub fn seams() -> Vec<Seam> {
    vec![
        Seam::rule("fold", "the anti-diagonal -- eggSo-v0 exactly", a_fold),
        Seam::rule(
            "diag3",
            "(r+c) mod 3 -- the fold's own level sets, at the optimal split",
            a_diag3,
        ),
        Seam::rule("idx3", "j mod 3 -- shatters rows, columns and diagonals at n=32", a_idx3),
        Seam::rule("rows3", "r mod 3 -- shatters columns, concentrates rows", a_rows3),
        Seam::rule("cols3", "c mod 3 -- the mirror of rows3", a_cols3),
        Seam::rule(
            "blocks",
            "contiguous thirds -- the control: optimal split, worst bursts",
            a_blocks,
        ),
        Seam::rule("seam128", "the fold's shape with a four-diagonal seam", a_seam128),
    ]
}

/// `idx3 = ((n mod 3) * r + c) mod 3`, so what it shatters depends on
/// `n mod 3`. At `n = 32` it is lines of slope -2 and everything shatters; at
/// `n = 0 (mod 3)` it degenerates to `cols3`; at `n = 1 (mod 3)` it IS
/// `diag3` and concentrates the anti-diagonal completely. The round reports
/// this rather than generalising a win that belongs to one residue class.
pub fn idx3_identity(n: usize) -> &'static str {
    match n % 3 {
        0 => "degenerates to cols3",
        1 => "identical to diag3",
        _ => "lines of slope -2: shatters rows, columns and diagonals",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Channel {
    One,
    TwoAnywhere,
    TwoSameClass,
    TwoDifferentClasses,
    ThreeOnePerClass,
    /// a contiguous run of `b` cells in one row, flagged as erasures
    RowBurstFlagged(usize),
    /// the same run, unflagged
    RowBurstBlind(usize),
    /// v1(b)'s own channel: `b` unflagged flips in one row, every one of them
    /// inside Inner or every one inside Outer. The damage is defined by the
    /// FOLD's regions for every arm, so each arm decodes the same wound.
    RowBurstInRegion(usize),
    /// a contiguous run of `b` cells down one column, flagged
    ColBurstFlagged(usize),
    ColBurstBlind(usize),
    /// a run of `b` consecutive cells along one anti-diagonal band, flagged
    DiagBurstFlagged(usize),
    DiagBurstBlind(usize),
    /// a run of `b` consecutive row-major indices, which WRAPS at row
    /// boundaries -- what a contiguous storage wound actually looks like
    TapeBurstFlagged(usize),
    TapeBurstBlind(usize),
    /// every cell of one full anti-diagonal, held fixed in grid coordinates
    AntiDiagonal,
    /// every cell of the smallest class
    ThinnestClass,
}

impl Channel {
    pub fn label(&self) -> String {
        match self {
            Channel::One => "1 cell".into(),
            Channel::TwoAnywhere => "2 anywhere".into(),
            Channel::TwoSameClass => "2 same class".into(),
            Channel::TwoDifferentClasses => "2 different classes".into(),
            Channel::ThreeOnePerClass => "3 one per class".into(),
            Channel::RowBurstFlagged(b) => format!("{b} row burst, flagged"),
            Channel::RowBurstBlind(b) => format!("{b} row burst, blind"),
            Channel::RowBurstInRegion(b) => format!("{b} in-region burst, blind"),
            Channel::ColBurstFlagged(b) => format!("{b} col burst, flagged"),
            Channel::ColBurstBlind(b) => format!("{b} col burst, blind"),
            Channel::DiagBurstFlagged(b) => format!("{b} diag burst, flagged"),
            Channel::DiagBurstBlind(b) => format!("{b} diag burst, blind"),
            Channel::TapeBurstFlagged(b) => format!("{b} tape burst, flagged"),
            Channel::TapeBurstBlind(b) => format!("{b} tape burst, blind"),
            Channel::AntiDiagonal => "one full anti-diagonal".into(),
            Channel::ThinnestClass => "the thinnest class filled".into(),
        }
    }

    /// Whether this channel exists at all on an `n x n` grid. v4's sweep
    /// silently dropped the cases where it does not; this round asks first
    /// and prints the absence.
    pub fn exists_at(&self, n: usize) -> bool {
        match self {
            Channel::RowBurstFlagged(b)
            | Channel::RowBurstBlind(b)
            | Channel::ColBurstFlagged(b)
            | Channel::ColBurstBlind(b)
            | Channel::DiagBurstFlagged(b)
            | Channel::DiagBurstBlind(b) => *b >= 1 && *b <= n,
            // an in-region run needs to fit in a row AND inside one region,
            // and the widest region row-run available is n - 1 cells
            Channel::RowBurstInRegion(b) => *b >= 1 && *b < n,
            Channel::TapeBurstFlagged(b) | Channel::TapeBurstBlind(b) => *b >= 1 && *b <= n * n,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Tally {
    pub trials: usize,
    pub corrected: usize,
    pub detected: usize,
    pub wrong: usize,
    pub direct: usize,
    /// mean damaged cells landing in one class -- the mechanism number that
    /// explains every row above it
    pub class_max_sum: usize,
    /// v5's addition: the WORST over trials, not the mean. Part 2's whole
    /// objective is a worst case, and a mean cannot be checked against a
    /// bound.
    pub class_max_worst: usize,
    pub notes: Vec<(&'static str, usize)>,
}

impl Tally {
    pub fn note(&mut self, n: &'static str) {
        for e in self.notes.iter_mut() {
            if e.0 == n {
                e.1 += 1;
                return;
            }
        }
        self.notes.push((n, 1));
    }
    pub fn class_max_mean(&self) -> f64 {
        if self.trials == 0 {
            0.0
        } else {
            self.class_max_sum as f64 / self.trials as f64
        }
    }
}

/// The cells of a length-`b` run along one anti-diagonal band, or `None` if
/// no band of this grid is that long.
///
/// The site reads a band from the bottom-left corner upward, which runs the
/// tape index DOWN. A burst is a set, so the same runs come out either way --
/// but they are emitted in increasing tape order, matching
/// `optimum::windows`, so the two enumerations cannot disagree about what a
/// diagonal burst is.
fn diag_run(n: usize, b: usize, g: &mut Mul32) -> Option<Vec<usize>> {
    let arcs = fold::arcs(n);
    let bands: Vec<usize> = (0..arcs.len()).filter(|&d| arcs[d] >= b).collect();
    if bands.is_empty() {
        return None;
    }
    let d = bands[g.pick(bands.len())];
    let k0 = g.pick(arcs[d] - b + 1);
    let r0 = (n - 1).min(d) - (k0 + b - 1);
    Some((0..b).map(|t| (r0 + t) * n + (d - r0 - t)).collect())
}

/// Damage a clean square, returning the flagged list when the channel flags.
/// `None` when the channel does not exist on this grid.
fn damage(cells: &mut [i8], code: &Code, ch: Channel, g: &mut Mul32) -> Option<Vec<usize>> {
    let n = code.n;
    if !ch.exists_at(n) {
        return None;
    }
    let flip = |cells: &mut [i8], run: &[usize]| {
        for &i in run {
            cells[i] ^= 1;
        }
    };
    let erase = |cells: &mut [i8], run: &[usize]| -> Vec<usize> {
        for &i in run {
            cells[i] = -1;
        }
        run.to_vec()
    };
    let row_run = |g: &mut Mul32, b: usize| -> Vec<usize> {
        let row = g.pick(n);
        let c0 = g.pick(n - b + 1);
        (0..b).map(|t| row * n + c0 + t).collect()
    };
    let col_run = |g: &mut Mul32, b: usize| -> Vec<usize> {
        let col = g.pick(n);
        let r0 = g.pick(n - b + 1);
        (0..b).map(|t| (r0 + t) * n + col).collect()
    };
    let tape_run = |g: &mut Mul32, b: usize| -> Vec<usize> {
        let j0 = g.pick(n * n - b + 1);
        (j0..j0 + b).collect()
    };
    Some(match ch {
        Channel::One => {
            let i = g.pick(code.l);
            cells[i] ^= 1;
            vec![]
        }
        Channel::TwoAnywhere => {
            let a = g.pick(code.l);
            let mut b = g.pick(code.l);
            while b == a {
                b = g.pick(code.l);
            }
            cells[a] ^= 1;
            cells[b] ^= 1;
            vec![]
        }
        Channel::TwoSameClass => {
            let mut k = g.pick(3);
            while code.members[k].len() < 2 {
                k = g.pick(3);
            }
            let m = &code.members[k];
            let a = m[g.pick(m.len())];
            let mut b = m[g.pick(m.len())];
            while b == a {
                b = m[g.pick(m.len())];
            }
            cells[a] ^= 1;
            cells[b] ^= 1;
            vec![]
        }
        Channel::TwoDifferentClasses => {
            let a = g.pick(code.l);
            let mut b = g.pick(code.l);
            while code.class[b] == code.class[a] {
                b = g.pick(code.l);
            }
            cells[a] ^= 1;
            cells[b] ^= 1;
            vec![]
        }
        Channel::ThreeOnePerClass => {
            for k in 0..3 {
                let m = &code.members[k];
                if !m.is_empty() {
                    cells[m[g.pick(m.len())]] ^= 1;
                }
            }
            vec![]
        }
        Channel::RowBurstFlagged(b) => {
            let run = row_run(g, b);
            erase(cells, &run)
        }
        Channel::RowBurstBlind(b) => {
            let run = row_run(g, b);
            flip(cells, &run);
            vec![]
        }
        Channel::RowBurstInRegion(b) => {
            // eggSo-v1/tools/versus.js:122-128, rejection sampling and all.
            // The region is the FOLD's, for every arm, so each arm is asked
            // about the same wound rather than about its own geometry.
            loop {
                let run = row_run(g, b);
                let reg = fold::region_of(run[0] / n, run[0] % n, n);
                if reg != fold::FOLD
                    && run.iter().all(|&i| fold::region_of(i / n, i % n, n) == reg)
                {
                    flip(cells, &run);
                    break vec![];
                }
            }
        }
        Channel::ColBurstFlagged(b) => {
            let run = col_run(g, b);
            erase(cells, &run)
        }
        Channel::ColBurstBlind(b) => {
            let run = col_run(g, b);
            flip(cells, &run);
            vec![]
        }
        Channel::DiagBurstFlagged(b) => {
            let run = diag_run(n, b, g)?;
            erase(cells, &run)
        }
        Channel::DiagBurstBlind(b) => {
            let run = diag_run(n, b, g)?;
            flip(cells, &run);
            vec![]
        }
        Channel::TapeBurstFlagged(b) => {
            let run = tape_run(g, b);
            erase(cells, &run)
        }
        Channel::TapeBurstBlind(b) => {
            let run = tape_run(g, b);
            flip(cells, &run);
            vec![]
        }
        Channel::AntiDiagonal => {
            for r in 0..n {
                cells[r * n + (n - 1 - r)] ^= 1;
            }
            vec![]
        }
        Channel::ThinnestClass => {
            let k = (0..3).min_by_key(|&k| code.members[k].len()).unwrap();
            for &i in &code.members[k] {
                cells[i] ^= 1;
            }
            vec![]
        }
    })
}

/// How the damage of one trial fell across the classes.
pub fn class_spread(clean: &[i8], hurt: &[i8], code: &Code) -> usize {
    let mut per = [0usize; 3];
    for i in 0..code.l {
        if clean[i] != hurt[i] {
            per[code.class[i] as usize] += 1;
        }
    }
    *per.iter().max().unwrap()
}

/// `None` when the channel does not exist on this grid -- never a silent zero.
pub fn run_channel(code: &Code, ch: Channel, trials: usize, seed: u32) -> Option<Tally> {
    if !ch.exists_at(code.n) {
        return None;
    }
    let mut g = Mul32::new(seed);
    let mut t = Tally { trials, ..Default::default() };
    for _ in 0..trials {
        let clean = g.cells(code.l);
        let check = code.checks_for(&clean);
        let mut hurt = clean.clone();
        let erased = damage(&mut hurt, code, ch, &mut g)?;
        let spread = class_spread(&clean, &hurt, code);
        t.class_max_sum += spread;
        t.class_max_worst = t.class_max_worst.max(spread);
        let opts = if erased.is_empty() { Opts::new() } else { Opts::erased(&erased) };
        let r = repair(&mut hurt, &check, code, &opts);
        match r.status {
            Status::Corrected | Status::Clean => {
                if hurt == clean {
                    t.corrected += 1;
                    if r.searched == 0 {
                        t.direct += 1;
                    }
                } else {
                    t.wrong += 1;
                }
            }
            _ => {
                t.detected += 1;
                t.note(r.note);
            }
        }
    }
    Some(t)
}

/// The largest flagged row burst an assignment survives, by sweeping rather
/// than by a single length -- at 12 cells every arm wins and the channel
/// cannot discriminate.
///
/// v4 filtered `b >= code.n` out of the returned list, so a length that could
/// not be run looked identical to one that was never asked for. It is an
/// explicit `None` here.
pub fn burst_breaking_point(
    code: &Code,
    lengths: &[usize],
    trials: usize,
    seed: u32,
) -> Vec<(usize, Option<Tally>)> {
    lengths
        .iter()
        .map(|&b| {
            (b, run_channel(code, Channel::RowBurstFlagged(b), trials, seed + b as u32))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// S2, the fairness assert, carried from v4. `sizes` depends only on p, q
    /// and confirm, so every arm pays exactly the same 4.69% and every
    /// measured difference between them is pure geometry.
    #[test]
    fn every_seam_costs_the_same() {
        let base = Code::new(32, true, "fold", a_fold);
        for s in seams() {
            let c = s.code(32, true);
            assert_eq!(c.check_bits(), base.check_bits(), "{}", s.name);
            assert_eq!(c.p, base.p);
            assert_eq!(c.q, base.q);
            assert_eq!(c.sizes().iter().sum::<usize>(), 1024, "{}", s.name);
        }
    }

    /// A table arm costs the same as a rule arm, which is what lets the cubic
    /// and the searched partitions sit in the same table as `diag3`.
    #[test]
    fn a_table_arm_costs_what_a_rule_arm_costs() {
        let d = seams().into_iter().find(|s| s.name == "diag3").unwrap();
        let copy = Seam::table("diag3-as-a-table", "the same partition, tabulated", 32, d.classes(32));
        let a = d.code(32, true);
        let b = copy.code(32, true);
        assert_eq!(a.check_bits(), b.check_bits());
        assert_eq!(a.sizes(), b.sizes());
        assert_eq!(a.class, b.class);
    }

    /// S4. `diag3` -- the fold's own level sets -- hits the optimal split.
    #[test]
    fn diag3_is_optimal_and_the_fold_is_not() {
        let f = Code::new(32, true, "fold", a_fold);
        let d = Code::new(32, true, "diag3", a_diag3);
        let sf = fold::separation(&f.sizes());
        let sd = fold::separation(&d.sizes());
        assert!((sf - 0.5303).abs() < 5e-5, "fold {sf}");
        assert!((sd - 0.6673).abs() < 5e-5, "diag3 {sd}");
        assert!(sd > sf);
    }

    /// S3. `blocks` and `idx3` share the separation figure to the digit and
    /// could not behave more differently on a burst. Separation was never
    /// the figure of merit.
    #[test]
    fn separation_alone_does_not_decide_anything() {
        let b = Code::new(32, true, "blocks", a_blocks);
        let i = Code::new(32, true, "idx3", a_idx3);
        let sb = fold::separation(&b.sizes());
        let si = fold::separation(&i.sizes());
        assert!((sb - si).abs() < 1e-6, "blocks {sb} vs idx3 {si}");

        // and now the burst: same statistic, opposite geometry
        let spread = |code: &Code| {
            let mut worst = 0usize;
            for row in 0..32 {
                for c0 in 0..(32 - 12) {
                    let mut per = [0usize; 3];
                    for j in 0..12 {
                        per[code.class[row * 32 + c0 + j] as usize] += 1;
                    }
                    worst = worst.max(*per.iter().max().unwrap());
                }
            }
            worst
        };
        assert_eq!(spread(&i), 4, "idx3 shatters a 12-cell row burst");
        assert_eq!(spread(&b), 12, "blocks concentrates it entirely");
    }

    /// S6. `idx3`'s clean sweep belongs to `n = 2 (mod 3)` and is reported as
    /// such rather than generalised.
    #[test]
    fn idx3_is_an_accident_of_n_mod_three() {
        assert_eq!(idx3_identity(32), "lines of slope -2: shatters rows, columns and diagonals");
        assert_eq!(idx3_identity(33), "degenerates to cols3");
        assert_eq!(idx3_identity(31), "identical to diag3");

        // at n = 31 the anti-diagonal really does collapse into one class
        let n = 31usize;
        let mut per = [0usize; 3];
        for r in 0..n {
            let j = r * n + (n - 1 - r);
            per[j % 3] += 1;
        }
        assert_eq!(*per.iter().max().unwrap(), n, "at n=31 idx3 concentrates the anti-diagonal");
    }

    /// The fold has a thin class to fill and a balanced arm does not.
    #[test]
    fn only_the_fold_shaped_arms_have_a_thin_class() {
        let f = Code::new(32, true, "fold", a_fold);
        assert_eq!(*f.sizes().iter().min().unwrap(), 32);
        for name in ["diag3", "idx3", "blocks"] {
            let s = seams().into_iter().find(|s| s.name == name).unwrap();
            let c = s.code(32, true);
            assert!(*c.sizes().iter().min().unwrap() > 300, "{name} has no thin class");
        }
    }

    /// v5's own: a channel that cannot exist reports `None` rather than a
    /// zero that reads like a loss. This is the trap v4 carried.
    #[test]
    fn a_channel_that_cannot_exist_says_so() {
        let c = Code::new(8, true, "diag3", a_diag3);
        assert!(Channel::RowBurstFlagged(8).exists_at(8));
        assert!(!Channel::RowBurstFlagged(9).exists_at(8));
        assert!(run_channel(&c, Channel::RowBurstFlagged(9), 4, 1).is_none());
        assert!(run_channel(&c, Channel::RowBurstFlagged(8), 4, 1).is_some());
        let swept = burst_breaking_point(&c, &[4, 8, 12], 4, 1);
        assert_eq!(swept.len(), 3, "every asked-for length comes back, present or absent");
        assert!(swept[2].1.is_none(), "12 does not fit in an 8-wide row and says so");
    }

    /// The in-region burst is v1(b)'s channel: every flipped cell shares one
    /// of the FOLD's regions, and the region is never the Fold itself.
    #[test]
    fn the_in_region_burst_stays_inside_one_region() {
        let c = Code::new(32, true, "diag3", a_diag3);
        let mut g = Mul32::new(20260903);
        for _ in 0..200 {
            let mut cells = vec![0i8; c.l];
            let e = damage(&mut cells, &c, Channel::RowBurstInRegion(12), &mut g).unwrap();
            assert!(e.is_empty(), "the in-region burst is unflagged");
            let hit: Vec<usize> = (0..c.l).filter(|&i| cells[i] == 1).collect();
            assert_eq!(hit.len(), 12);
            let reg = fold::region_of(hit[0] / 32, hit[0] % 32, 32);
            assert_ne!(reg, fold::FOLD);
            for &i in &hit {
                assert_eq!(fold::region_of(i / 32, i % 32, 32), reg);
            }
            // and it really is one contiguous row run
            assert_eq!(hit[11] - hit[0], 11);
            assert_eq!(hit[0] / 32, hit[11] / 32);
        }
    }

    /// A diagonal run walks one band, the site's way -- `r` down and `c` up.
    #[test]
    fn a_diagonal_run_stays_on_one_band() {
        let mut g = Mul32::new(7);
        for n in [8usize, 16, 32] {
            for b in [2usize, 5, n] {
                for _ in 0..50 {
                    let run = diag_run(n, b, &mut g).unwrap();
                    assert_eq!(run.len(), b);
                    let d = run[0] / n + run[0] % n;
                    for &i in &run {
                        assert_eq!(i / n + i % n, d, "the run left band {d} at n={n}");
                    }
                    // consecutive cells of a band are n-1 apart on the tape
                    for w in run.windows(2) {
                        assert_eq!(w[1] - w[0], n - 1);
                    }
                }
            }
            assert!(diag_run(n, n + 1, &mut g).is_none(), "no band is that long at n={n}");
        }
    }

    /// The tape burst is the only geometry that WRAPS, which is the whole
    /// reason it discriminates: it crosses row boundaries, and that is where
    /// a linear partition slips phase.
    #[test]
    fn the_tape_burst_wraps_and_the_row_burst_does_not() {
        let c = Code::new(8, true, "diag3", a_diag3);
        let mut g = Mul32::new(3);
        let mut wrapped = 0usize;
        for _ in 0..400 {
            let mut cells = vec![0i8; c.l];
            damage(&mut cells, &c, Channel::TapeBurstBlind(6), &mut g).unwrap();
            let hit: Vec<usize> = (0..c.l).filter(|&i| cells[i] == 1).collect();
            assert_eq!(hit.len(), 6);
            if hit[0] / 8 != hit[5] / 8 {
                wrapped += 1;
            }
        }
        assert!(wrapped > 0, "a tape burst that never wraps is a row burst");
        for _ in 0..200 {
            let mut cells = vec![0i8; c.l];
            damage(&mut cells, &c, Channel::RowBurstBlind(6), &mut g).unwrap();
            let hit: Vec<usize> = (0..c.l).filter(|&i| cells[i] == 1).collect();
            assert_eq!(hit[0] / 8, hit[5] / 8, "a row burst must not leave its row");
        }
    }
}
