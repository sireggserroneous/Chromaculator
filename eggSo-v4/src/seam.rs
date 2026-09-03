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
//! Julia set between them. What a FREE choice would pick is a third of the
//! square, and the optimum of the two-basins-plus-seam family is therefore
//! the fair three-way split again. So this module prices the geometry, and
//! `diag3` is the arm that decides how the verdict should be worded: it is
//! the fold's OWN level sets, `(r+c) mod 3`, and it hits the optimal split
//! exactly. If it wins, the fold's direction was right and only its
//! threshold was wrong.

use crate::code::{repair, Code, Mul32, Opts, Status};
use crate::fold;

/// A named cell-to-class assignment.
pub struct Seam {
    pub name: &'static str,
    pub note: &'static str,
    pub assign: fn(usize, usize, usize, usize) -> u8,
}

fn a_fold(r: usize, c: usize, _j: usize, n: usize) -> u8 {
    fold::region_of(r, c, n)
}
fn a_diag3(r: usize, c: usize, _j: usize, _n: usize) -> u8 {
    ((r + c) % 3) as u8
}
fn a_idx3(_r: usize, _c: usize, j: usize, _n: usize) -> u8 {
    (j % 3) as u8
}
fn a_rows3(r: usize, _c: usize, _j: usize, _n: usize) -> u8 {
    (r % 3) as u8
}
fn a_cols3(_r: usize, c: usize, _j: usize, _n: usize) -> u8 {
    (c % 3) as u8
}
fn a_blocks(_r: usize, _c: usize, j: usize, n: usize) -> u8 {
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
fn a_seam128(r: usize, c: usize, _j: usize, n: usize) -> u8 {
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
        Seam { name: "fold", note: "the anti-diagonal -- eggSo-v0 exactly", assign: a_fold },
        Seam {
            name: "diag3",
            note: "(r+c) mod 3 -- the fold's own level sets, at the optimal split",
            assign: a_diag3,
        },
        Seam { name: "idx3", note: "j mod 3 -- shatters rows, columns and diagonals at n=32", assign: a_idx3 },
        Seam { name: "rows3", note: "r mod 3 -- shatters columns, concentrates rows", assign: a_rows3 },
        Seam { name: "cols3", note: "c mod 3 -- the mirror of rows3", assign: a_cols3 },
        Seam { name: "blocks", note: "contiguous thirds -- the control: optimal split, worst bursts", assign: a_blocks },
        Seam { name: "seam128", note: "the fold's shape with a four-diagonal seam", assign: a_seam128 },
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
            Channel::RowBurstFlagged(b) => format!("{b}-cell row burst, flagged"),
            Channel::RowBurstBlind(b) => format!("{b}-cell row burst, blind"),
            Channel::AntiDiagonal => "one full anti-diagonal".into(),
            Channel::ThinnestClass => "the thinnest class filled".into(),
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
    /// mean and max damaged cells landing in one class -- the mechanism
    /// number that explains every row above it
    pub class_max_sum: usize,
    pub notes: Vec<(&'static str, usize)>,
}

impl Tally {
    fn note(&mut self, n: &'static str) {
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

/// Damage a clean square, returning the flagged list when the channel flags.
fn damage(
    cells: &mut [i8],
    code: &Code,
    ch: Channel,
    g: &mut Mul32,
) -> Vec<usize> {
    let n = code.n;
    match ch {
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
            let row = g.pick(n);
            let c0 = g.pick(n - b);
            let mut f = Vec::with_capacity(b);
            for j in 0..b {
                let i = row * n + c0 + j;
                cells[i] = -1;
                f.push(i);
            }
            f
        }
        Channel::RowBurstBlind(b) => {
            let row = g.pick(n);
            let c0 = g.pick(n - b);
            for j in 0..b {
                cells[row * n + c0 + j] ^= 1;
            }
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
    }
}

/// How the damage of one trial fell across the classes.
fn class_spread(clean: &[i8], hurt: &[i8], code: &Code) -> usize {
    let mut per = [0usize; 3];
    for i in 0..code.l {
        if clean[i] != hurt[i] {
            per[code.class[i] as usize] += 1;
        }
    }
    *per.iter().max().unwrap()
}

pub fn run_channel(code: &Code, ch: Channel, trials: usize, seed: u32) -> Tally {
    let mut g = Mul32::new(seed);
    let mut t = Tally { trials, ..Default::default() };
    for _ in 0..trials {
        let clean = g.cells(code.l);
        let check = code.checks_for(&clean);
        let mut hurt = clean.clone();
        let erased = damage(&mut hurt, code, ch, &mut g);
        t.class_max_sum += class_spread(&clean, &hurt, code);
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
    t
}

/// The largest flagged row burst an assignment survives, by sweeping rather
/// than by a single length -- at 12 cells every arm wins and the channel
/// cannot discriminate.
pub fn burst_breaking_point(code: &Code, lengths: &[usize], trials: usize, seed: u32) -> Vec<(usize, Tally)> {
    lengths
        .iter()
        .filter(|&&b| b < code.n)
        .map(|&b| (b, run_channel(code, Channel::RowBurstFlagged(b), trials, seed + b as u32)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// S2, the fairness assert. `sizes` depends only on p, q and confirm, so
    /// every arm pays exactly the same 4.69% and every measured difference
    /// between them is pure geometry.
    #[test]
    fn every_seam_costs_the_same() {
        let base = Code::new(32, true, "fold", a_fold);
        for s in seams() {
            let c = Code::new(32, true, s.name, s.assign);
            assert_eq!(c.check_bits(), base.check_bits(), "{}", s.name);
            assert_eq!(c.p, base.p);
            assert_eq!(c.q, base.q);
            assert_eq!(c.sizes().iter().sum::<usize>(), 1024, "{}", s.name);
        }
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

    /// The fold has a thin class to fill and a balanced arm does not, which
    /// is why v0's "Fold filled" channel has no analogue here and is replaced
    /// rather than reinterpreted.
    #[test]
    fn only_the_fold_shaped_arms_have_a_thin_class() {
        let f = Code::new(32, true, "fold", a_fold);
        assert_eq!(*f.sizes().iter().min().unwrap(), 32);
        for name in ["diag3", "idx3", "blocks"] {
            let s = seams().into_iter().find(|s| s.name == name).unwrap();
            let c = Code::new(32, true, s.name, s.assign);
            assert!(*c.sizes().iter().min().unwrap() > 300, "{name} has no thin class");
        }
    }
}
