//! optimum.rs -- which three-class partition actually minimises a burst.
//!
//! eggSo-v4 discovered by accident that the statistic v0's entire verdict
//! rested on -- the chance two random cells land in different classes --
//! moves no error channel at all. Every partition scores 397 to 400 of 400 on
//! the pair channels. What the fold's geometry costs is BURST SPREAD, and
//! that figure of merit has never been optimised. This module optimises it.
//!
//! The objective. For a partition `C` of the `n x n` grid into three classes
//! and a burst length `L`,
//!
//! ```text
//! worst(C, L) = max over bursts B of length L  of  max over classes k of  |B & k|
//! ```
//!
//! over four burst geometries -- along a ROW, along a COLUMN, along an
//! ANTI-DIAGONAL (consecutive cells of one band, which is a tape step of
//! `n-1`), and along the row-major TAPE index, which WRAPS at row boundaries
//! and is what a contiguous storage wound actually looks like.
//!
//! `L` cells over three classes always give some class at least `ceil(L/3)`,
//! so **`ceil(L/3)` is the floor** and the only question is which partitions
//! reach it on all four geometries at once. A partition measured BELOW the
//! floor is an arithmetic error in this file, not a discovery, and `floor_of`
//! is asserted against every measurement for exactly that reason.
//!
//! Three results live here, in increasing order of how much they cost to get:
//!
//!   * `linear_*` -- the nine `(a,b)` of `C(r,c) = (a*r + b*c) mod 3`, their
//!     four shatter conditions, and the theorem that only `n = 2 (mod 3)`
//!     admits a linear partition optimal on all four geometries at once.
//!   * `periodicity_lemma` and `construct_periodic` -- for `3 | L` the tape
//!     constraint has NO SLACK, which forces the tape to be `L`-periodic and
//!     turns the whole question into arithmetic on `Z/L`. This is the part
//!     that says the obstruction is not a linearity artefact.
//!   * `exact` and `anneal` -- the search, for the cases the arithmetic does
//!     not settle. Its seed, schedule and budget are fixed HERE, in the
//!     source, before the first run, per the round's honesty rule.

use crate::code::Mul32;
use crate::fold;

/// The four burst geometries. `Tape` is the only one that crosses a row
/// boundary, and that is precisely why it discriminates.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Geom {
    Row,
    Col,
    Diag,
    Tape,
}

pub const GEOMS: [Geom; 4] = [Geom::Row, Geom::Col, Geom::Diag, Geom::Tape];

impl Geom {
    pub fn name(&self) -> &'static str {
        match self {
            Geom::Row => "row",
            Geom::Col => "col",
            Geom::Diag => "diag",
            Geom::Tape => "tape",
        }
    }
    /// The tape index step between consecutive cells of a burst. `Tape` has
    /// no single step -- it is 1 inside a row and `1 - (n-1)` across the
    /// boundary -- which is the phase slip the whole linear analysis turns on.
    pub fn step(&self, n: usize) -> Option<usize> {
        match self {
            Geom::Row => Some(1),
            Geom::Col => Some(n),
            Geom::Diag => Some(n - 1),
            Geom::Tape => None,
        }
    }
}

/// The floor. `L` cells over three classes: some class gets `ceil(L/3)`.
#[inline]
pub fn floor_of(l: usize) -> usize {
    l.div_ceil(3)
}

/// Every placement of a length-`l` run in one geometry, as cell indices.
/// Empty when the geometry admits no run of that length at this `n` -- which
/// is a real answer and is reported rather than filled in with a zero.
pub fn windows(n: usize, l: usize, g: Geom) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    if l == 0 || l > n * n {
        return out;
    }
    match g {
        Geom::Row => {
            if l > n {
                return out;
            }
            for r in 0..n {
                for c0 in 0..=(n - l) {
                    out.push((0..l).map(|t| r * n + c0 + t).collect());
                }
            }
        }
        Geom::Col => {
            if l > n {
                return out;
            }
            for c in 0..n {
                for r0 in 0..=(n - l) {
                    out.push((0..l).map(|t| (r0 + t) * n + c).collect());
                }
            }
        }
        Geom::Diag => {
            let arcs = fold::arcs(n);
            for (d, &len) in arcs.iter().enumerate() {
                if len < l {
                    continue;
                }
                let rmax = (n - 1).min(d);
                for k0 in 0..=(len - l) {
                    // the site reads a band from the bottom-left corner
                    // UPWARD, so its own order runs `r` down and the tape
                    // index down with it. A burst is a SET, so the same runs
                    // come out either way -- but they are emitted in
                    // increasing tape order here so that `Geom::step` means
                    // the same thing on all four geometries.
                    let r0 = rmax - (k0 + l - 1);
                    out.push((0..l).map(|t| (r0 + t) * n + (d - r0 - t)).collect());
                }
            }
        }
        Geom::Tape => {
            for j0 in 0..=(n * n - l) {
                out.push((j0..j0 + l).collect());
            }
        }
    }
    out
}

/// `worst(C, L)` on one geometry. `None` when the geometry has no run of that
/// length here, which is not the same as zero and is never printed as zero.
pub fn worst_of(class: &[u8], n: usize, l: usize, g: Geom) -> Option<usize> {
    let ws = windows(n, l, g);
    if ws.is_empty() {
        return None;
    }
    let mut worst = 0usize;
    for w in &ws {
        let mut per = [0usize; 3];
        for &i in w {
            per[class[i] as usize] += 1;
        }
        worst = worst.max(*per.iter().max().unwrap());
    }
    Some(worst)
}

/// `worst(C, L)` on all four, in `GEOMS` order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Worst {
    pub per: [Option<usize>; 4],
}

impl Worst {
    /// The worst over the geometries that exist. `None` when none of them do.
    pub fn overall(&self) -> Option<usize> {
        self.per.iter().flatten().copied().max()
    }
    pub fn at_floor(&self, l: usize) -> bool {
        self.overall().map(|w| w == floor_of(l)).unwrap_or(false)
    }
    /// How far above the floor, over the geometries that exist.
    pub fn gap(&self, l: usize) -> Option<usize> {
        self.overall().map(|w| w - floor_of(l))
    }
    pub fn cells(&self) -> Vec<String> {
        self.per
            .iter()
            .map(|v| match v {
                Some(w) => w.to_string(),
                None => "--".to_string(),
            })
            .collect()
    }
}

pub fn worst_all(class: &[u8], n: usize, l: usize) -> Worst {
    let mut per = [None; 4];
    for (k, g) in GEOMS.iter().enumerate() {
        per[k] = worst_of(class, n, l, *g);
        // B1: nothing may come in under the floor. A partition that does is
        // a bug in this file and is not allowed to look like a discovery.
        if let Some(w) = per[k] {
            assert!(
                w >= floor_of(l),
                "worst {w} beats the floor {} on {} at n={n}, L={l}",
                floor_of(l),
                g.name()
            );
        }
    }
    Worst { per }
}

// ---- the linear family ---------------------------------------------------

/// `C(r, c) = (a*r + b*c) mod 3`, the nine-member family that contains every
/// arm eggSo-v4 measured except `fold`, `blocks` and `seam128`.
pub fn linear_class(a: usize, b: usize, n: usize) -> Vec<u8> {
    (0..n * n).map(|j| ((a * (j / n) + b * (j % n)) % 3) as u8).collect()
}

/// The four shatter conditions, as reasoning rather than as measurement, so
/// the suite can confront one with the other:
///
///   * a ROW steps `c` by one, so the class steps by `b`: balanced iff `b`
///     is nonzero mod 3.
///   * a COLUMN steps `r`, so the class steps by `a`.
///   * an ANTI-DIAGONAL steps `r` down and `c` up, so the class steps by
///     `b - a`: balanced iff `a` and `b` differ.
///   * the TAPE steps by `b` inside a row, and crossing `(k, n-1)` to
///     `(k+1, 0)` shifts it by `a - b(n-1)`. Those agree exactly when
///     `a = b*n (mod 3)`, and when they do not the run slips phase at the row
///     boundary and the worst case rises.
pub fn linear_predicted(a: usize, b: usize, n: usize, g: Geom) -> bool {
    let (a, b) = (a % 3, b % 3);
    match g {
        Geom::Row => b != 0,
        Geom::Col => a != 0,
        Geom::Diag => a != b,
        Geom::Tape => b != 0 && a == (b * n) % 3,
    }
}

/// The linear theorem, re-derived rather than quoted: which `(a,b)` satisfy
/// all four conditions at once.
///
/// The system is `{a != 0, b != 0, a != b, a = b*n}` over `(Z/3)^2`. At
/// `n = 0 (mod 3)` it forces `a = 0` and contradicts itself; at `n = 1` it
/// forces `a = b` and contradicts itself; at `n = 2` it has exactly two
/// solutions. So a linear partition is burst-optimal on all four geometries
/// **only** at `n = 2 (mod 3)`.
pub fn linear_solutions(n: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for a in 0..3 {
        for b in 0..3 {
            if GEOMS.iter().all(|&g| linear_predicted(a, b, n, g)) {
                out.push((a, b));
            }
        }
    }
    out
}

/// The lowest-energy linear arm at `(n, L)`, which is what the annealer is
/// seeded from. Ties break toward the lower `(a, b)`, deterministically.
pub fn best_linear(n: usize, l: usize) -> (usize, usize, usize) {
    best_linear_in(&Energy::new(n, l))
}

/// The same, over an `Energy` the caller already built. Enumerating the
/// windows is the expensive half, so the annealer shares one.
pub fn best_linear_in(e: &Energy) -> (usize, usize, usize) {
    let mut best = (0usize, 0usize, usize::MAX);
    for a in 0..3 {
        for b in 0..3 {
            let en = e.energy_of(&linear_class(a, b, e.n));
            if en < best.2 {
                best = (a, b, en);
            }
        }
    }
    best
}

// ---- the periodicity lemma ----------------------------------------------

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// What the periodicity lemma says about `(n, L)`.
///
/// **The lemma.** Suppose `3 | L`. Then `ceil(L/3) = L/3` and a tape window
/// whose three class counts are each at most `L/3` has them all EXACTLY
/// `L/3` -- there is no slack anywhere. Slide the window one cell: it loses
/// `class(j)` and gains `class(j+L)`, and both windows are exactly balanced,
/// so `class(j) = class(j+L)` for every `j`. **The tape is forced to be
/// `L`-periodic**, `class(j) = g(j mod L)` for a balanced `g : Z/L -> Z/3`.
///
/// Then, with no reference to linearity at all:
///
///   * a ROW window is `L` consecutive tape indices, hence all of `Z/L` once,
///     hence exactly balanced -- satisfied for free;
///   * a COLUMN window steps by `n`, walking the coset generated by
///     `d = gcd(n, L)` whose order is `L/d`, covering it exactly `d` times,
///     so balance requires `3 | L/gcd(n, L)`;
///   * an ANTI-DIAGONAL window steps by `n-1`, so balance requires
///     `3 | L/gcd(n-1, L)`.
///
/// A condition whose geometry has no window at this `n` is vacuous, and this
/// function drops it rather than counting it against the case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lemma {
    /// `None` when `3` does not divide `L` -- the lemma is silent there,
    /// because the floor has slack and the periodicity argument never starts.
    pub applies: bool,
    pub col_quotient: Option<usize>,
    pub diag_quotient: Option<usize>,
    pub possible: bool,
}

pub fn periodicity_lemma(n: usize, l: usize) -> Lemma {
    if !l.is_multiple_of(3) || l > n * n {
        return Lemma { applies: false, col_quotient: None, diag_quotient: None, possible: true };
    }
    let col = (l <= n).then(|| l / gcd(n, l));
    let diag = (l <= n).then(|| l / gcd(n - 1, l));
    let ok = |q: Option<usize>| q.map(|q| q.is_multiple_of(3)).unwrap_or(true);
    Lemma {
        applies: true,
        col_quotient: col,
        diag_quotient: diag,
        possible: ok(col) && ok(diag),
    }
}

/// The construction the lemma's sufficiency half asks for: a balanced
/// `g : Z/L -> Z/3` that is also balanced on every residue class mod
/// `gcd(n, L)` and mod `gcd(n-1, L)`, laid onto the grid as
/// `class(j) = g(j mod L)`.
///
/// `g` need not be linear, and where it cannot be linear this is what buys
/// the case -- which is the one place in this round where nonlinearity earns
/// something. `L` is at most a couple of dozen, so a pruned depth-first
/// assignment over `Z/L` settles it instantly.
pub fn construct_g(n: usize, l: usize) -> Option<Vec<u8>> {
    if !l.is_multiple_of(3) || l > n * n {
        return None;
    }
    let lem = periodicity_lemma(n, l);
    if !lem.possible {
        return None;
    }
    // the moduli whose cosets must each come out balanced
    let mut mods: Vec<usize> = vec![1];
    if l <= n {
        mods.push(gcd(n, l));
        mods.push(gcd(n - 1, l));
    }
    mods.sort_unstable();
    mods.dedup();
    // quota per (modulus, coset, class)
    let quota: Vec<usize> = mods.iter().map(|&m| (l / m) / 3).collect();
    if mods.iter().zip(quota.iter()).any(|(&m, &q)| q * 3 * m != l) {
        return None; // a coset that cannot be split three ways at all
    }
    let mut cnt: Vec<Vec<[usize; 3]>> =
        mods.iter().map(|&m| vec![[0usize; 3]; m]).collect();
    let mut g = vec![0u8; l];

    fn go(
        x: usize,
        l: usize,
        mods: &[usize],
        quota: &[usize],
        cnt: &mut [Vec<[usize; 3]>],
        g: &mut [u8],
    ) -> bool {
        if x == l {
            return true;
        }
        // canonical labelling: first occurrences run 0, 1, 2
        let used = g[..x].iter().map(|&v| v as usize + 1).max().unwrap_or(0);
        for k in 0..=used.min(2) {
            if mods
                .iter()
                .enumerate()
                .any(|(mi, &m)| cnt[mi][x % m][k] + 1 > quota[mi])
            {
                continue;
            }
            for (mi, &m) in mods.iter().enumerate() {
                cnt[mi][x % m][k] += 1;
            }
            g[x] = k as u8;
            if go(x + 1, l, mods, quota, cnt, g) {
                return true;
            }
            for (mi, &m) in mods.iter().enumerate() {
                cnt[mi][x % m][k] -= 1;
            }
        }
        false
    }

    if !go(0, l, &mods, &quota, &mut cnt, &mut g) {
        return None;
    }
    Some(g)
}

/// `class(j) = g(j mod L)` over the whole grid, when `construct_g` succeeds.
pub fn construct_periodic(n: usize, l: usize) -> Option<Vec<u8>> {
    let g = construct_g(n, l)?;
    Some((0..n * n).map(|j| g[j % l]).collect())
}

// ---- the energy, shared by both searches --------------------------------

/// The window incidence structure both searches run on. Energy is the TOTAL
/// EXCESS over the floor, summed across every window of all four geometries,
/// so it is zero exactly when `worst(C, L) = ceil(L/3)`.
pub struct Energy {
    pub n: usize,
    pub l: usize,
    pub floor: usize,
    pub win: Vec<Vec<usize>>,
    pub per_geom: [usize; 4],
    /// window ids touching each cell
    pub touch: Vec<Vec<u32>>,
}

impl Energy {
    pub fn new(n: usize, l: usize) -> Energy {
        let mut win: Vec<Vec<usize>> = Vec::new();
        let mut per_geom = [0usize; 4];
        for (k, g) in GEOMS.iter().enumerate() {
            let ws = windows(n, l, *g);
            per_geom[k] = ws.len();
            win.extend(ws);
        }
        let mut touch = vec![Vec::new(); n * n];
        for (w, cells) in win.iter().enumerate() {
            for &i in cells {
                touch[i].push(w as u32);
            }
        }
        Energy { n, l, floor: floor_of(l), win, per_geom, touch }
    }

    #[inline]
    fn excess(&self, cnt: &[usize; 3]) -> usize {
        let m = cnt[0].max(cnt[1]).max(cnt[2]);
        m.saturating_sub(self.floor)
    }

    pub fn counts_of(&self, class: &[u8]) -> Vec<[usize; 3]> {
        self.win
            .iter()
            .map(|w| {
                let mut per = [0usize; 3];
                for &i in w {
                    per[class[i] as usize] += 1;
                }
                per
            })
            .collect()
    }

    pub fn energy_of(&self, class: &[u8]) -> usize {
        self.counts_of(class).iter().map(|c| self.excess(c)).sum()
    }

    pub fn windows_total(&self) -> usize {
        self.win.len()
    }
}

// ---- the search, with its rules fixed here ------------------------------

/// The honesty rules, in the source and not in a flag: ONE seed, ONE
/// schedule, ONE budget. `--full` widens the linear sweep and the pictures
/// and deliberately does NOT widen this, so a search cannot be quietly
/// retried until it wins.
pub const SEARCH_SEED: u32 = 20260903;
pub const SEARCH_BUDGET: usize = 2_000_000;

/// **Schedule A, as filed in PREDICTIONS.md before the first run.** It is
/// kept, and its numbers are kept, because the filed rule says so: "if it is
/// ever re-tuned, this file names the first configuration and the number it
/// produced."
///
/// It is mis-scaled, and the round says how rather than deleting it. A
/// single-cell move touches up to `4L` windows, so `|delta|` runs to a few
/// tens; at `T = 2.0` an uphill move of `+10` is taken with probability
/// `e^-5 = 0.7%`, which is what the measured acceptance rate came out at.
/// The schedule was therefore a greedy descent wearing an annealer's
/// clothes, and it measured the temperature rather than the question.
pub const T_HOT_FILED: f64 = 2.0;
pub const T_COLD_FILED: f64 = 0.005;

/// **Schedule B, the amendment.** One principled change and no more: the
/// temperature is put on the same scale as the energy. `|delta|` is set by
/// how many windows one cell sits in, which is `O(L)`, so `T_HOT = L`. Both
/// schedules are run and both tables are printed.
pub fn t_hot_scaled(l: usize) -> f64 {
    l as f64
}
pub const T_COLD_SCALED: f64 = 0.05;
/// A case that exhausts this many nodes reports INCONCLUSIVE and never
/// "no solution".
pub const EXACT_NODE_CAP: u64 = 200_000_000;

#[derive(Clone, Debug)]
pub struct Found {
    pub n: usize,
    pub l: usize,
    pub seed_arm: (usize, usize),
    pub seed_energy: usize,
    /// the seed's own worst case, carried so a reader can see whether the
    /// search made `worst` WORSE while lowering the energy. It can: energy is
    /// a total and `worst` is a maximum, and the two agree only at zero.
    pub seed_worst: Worst,
    pub best_energy: usize,
    pub best: Vec<u8>,
    pub worst: Worst,
    pub t_hot: f64,
    pub moves: usize,
    pub accepted: usize,
}

impl Found {
    pub fn beat_its_seed(&self) -> bool {
        self.best_energy < self.seed_energy
    }
    pub fn acceptance(&self) -> f64 {
        if self.moves == 0 {
            0.0
        } else {
            self.accepted as f64 / self.moves as f64
        }
    }
    /// Whether lowering the energy raised the worst case. Reported, because
    /// it is the honest limit of the objective the annealer actually runs on.
    pub fn worst_got_worse(&self) -> bool {
        match (self.worst.overall(), self.seed_worst.overall()) {
            (Some(a), Some(b)) => a > b,
            _ => false,
        }
    }
}

/// Annealing on the total excess, seeded from the best linear arm. A run that
/// fails to beat its seed is a result and is returned as one.
pub fn anneal(n: usize, l: usize, seed: u32, budget: usize, t_hot: f64, t_cold: f64) -> Found {
    let e = Energy::new(n, l);
    let (a, b, seed_energy) = best_linear_in(&e);
    let seed_worst = worst_all(&linear_class(a, b, n), n, l);
    let mut class = linear_class(a, b, n);
    let mut cnt = e.counts_of(&class);
    let mut cur: usize = cnt.iter().map(|c| e.excess(c)).sum();
    let mut best = class.clone();
    let mut best_energy = cur;
    let mut g = Mul32::new(seed);
    let mut accepted = 0usize;

    if e.windows_total() == 0 || budget == 0 {
        return Found {
            n,
            l,
            seed_arm: (a, b),
            seed_energy,
            seed_worst,
            best_energy: cur,
            best,
            worst: worst_all(&class, n, l),
            t_hot,
            moves: 0,
            accepted: 0,
        };
    }

    let ratio = t_cold / t_hot;
    for step in 0..budget {
        if best_energy == 0 {
            break;
        }
        let t = t_hot * ratio.powf(step as f64 / budget as f64);
        let j = g.pick(n * n);
        let from = class[j];
        let to = ((from as usize + 1 + g.pick(2)) % 3) as u8;
        // the delta, computed without writing, so a rejection costs nothing
        let mut delta: i64 = 0;
        for &w in &e.touch[j] {
            let c = &cnt[w as usize];
            let before = e.excess(c);
            let mut after = *c;
            after[from as usize] -= 1;
            after[to as usize] += 1;
            delta += e.excess(&after) as i64 - before as i64;
        }
        let take = if delta <= 0 {
            true
        } else {
            let u = g.next() as f64 / 4_294_967_296.0;
            u < (-(delta as f64) / t).exp()
        };
        if take {
            for &w in &e.touch[j] {
                let c = &mut cnt[w as usize];
                c[from as usize] -= 1;
                c[to as usize] += 1;
            }
            class[j] = to;
            cur = (cur as i64 + delta) as usize;
            accepted += 1;
            if cur < best_energy {
                best_energy = cur;
                best.copy_from_slice(&class);
            }
        }
    }
    Found {
        n,
        l,
        seed_arm: (a, b),
        seed_energy,
        seed_worst,
        best_energy,
        worst: worst_all(&best, n, l),
        best,
        t_hot,
        moves: budget,
        accepted,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// a partition at the floor on every geometry that exists
    Reached(Vec<u8>),
    /// the enumeration finished and there is none
    Impossible,
    /// the node cap was hit; this is NOT "no solution"
    Inconclusive,
}

#[derive(Clone, Debug)]
pub struct Exact {
    pub n: usize,
    pub l: usize,
    pub verdict: Verdict,
    pub nodes: u64,
}

/// Exhaustive depth-first enumeration over CANONICAL partitions.
///
/// Cells are assigned in row-major order, which is the order the tape
/// constraint runs in and therefore prunes earliest. Class labels are
/// restricted so that first occurrences run `0, 1, 2`, which quotients the
/// 6-fold relabelling symmetry exactly -- relabelling classes does not change
/// the objective, so one representative per orbit is enough.
///
/// The prune is: after assigning cell `j` to class `k`, no window touching
/// `j` may have more than `floor` cells of class `k`. Window counts only grow
/// as more cells are assigned, so a partial count already over the floor can
/// never come back down. The prune is therefore sound and the enumeration is
/// complete.
pub fn exact(n: usize, l: usize, node_cap: u64) -> Exact {
    let e = Energy::new(n, l);
    let mut cnt = vec![[0usize; 3]; e.win.len()];
    let mut class = vec![0u8; n * n];
    let mut nodes = 0u64;

    struct Ctx<'a> {
        e: &'a Energy,
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
        for k in 0..=used.min(2) {
            *nodes += 1;
            if *nodes > ctx.cap {
                return None;
            }
            let touch = &ctx.e.touch[j];
            if touch.iter().any(|&w| cnt[w as usize][k] + 1 > ctx.e.floor) {
                continue;
            }
            for &w in touch {
                cnt[w as usize][k] += 1;
            }
            class[j] = k as u8;
            let next_used = if k == used { used + 1 } else { used };
            match go(j + 1, next_used, ctx, cnt, class, nodes) {
                Some(true) => return Some(true),
                None => return None,
                Some(false) => {}
            }
            for &w in touch {
                cnt[w as usize][k] -= 1;
            }
        }
        Some(false)
    }

    let ctx = Ctx { e: &e, cap: node_cap };
    let verdict = match go(0, 0, &ctx, &mut cnt, &mut class, &mut nodes) {
        Some(true) => Verdict::Reached(class.clone()),
        Some(false) => Verdict::Impossible,
        None => Verdict::Inconclusive,
    };
    // a claimed solution is re-checked from scratch, by the same function the
    // rest of the round is measured with
    if let Verdict::Reached(ref c) = verdict {
        let w = worst_all(c, n, l);
        assert!(
            w.at_floor(l) || w.overall().is_none(),
            "the enumeration returned a partition at {:?}, not the floor {}",
            w.overall(),
            floor_of(l)
        );
    }
    Exact { n, l, verdict, nodes }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// B1, anchored. Every other test in this round compares a measurement
    /// against `floor_of`, so `floor_of` sits on BOTH sides of every
    /// comparison and a wrong floor would shift the measurements and the
    /// assertions together and stay silent. (Found by mutation: replacing
    /// `l.div_ceil(3)` with `l / 3` passed the whole suite.) So the floor is
    /// pinned twice over, to literals and to the pigeonhole fact itself,
    /// neither of which is derived from the function under test.
    #[test]
    fn the_floor_is_the_pigeonhole_bound_and_not_whatever_the_function_says() {
        // pinned to literals, including the cases where rounding decides it
        for (l, want) in [
            (1usize, 1usize),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 3),
            (8, 3),
            (9, 3),
            (10, 4),
            (11, 4),
            (12, 4),
            (18, 6),
        ] {
            assert_eq!(floor_of(l), want, "the floor at L={l}");
        }

        // and pinned to the pigeonhole fact, computed independently of
        // `div_ceil`: the smallest m with 3m >= L is both unavoidable and
        // achievable when L items go into 3 bins.
        for l in 1..=60usize {
            let independent = (1..=l).find(|m| 3 * m >= l).unwrap();
            assert_eq!(floor_of(l), independent, "the floor at L={l}");

            // unavoidable: no distribution of L into 3 bins has every bin
            // below the floor, or the three would not add up to L
            assert!(3 * (floor_of(l) - 1) < l, "the floor is loose at L={l}");
            // achievable: some distribution reaches it exactly
            let (a, b) = (l / 3, (l + 1) / 3);
            let c = l - a - b;
            assert_eq!(a.max(b).max(c), floor_of(l), "the floor is unreachable at L={l}");
        }
    }

    /// B1. The floor is a floor: `L` cells over three classes cannot do
    /// better than `ceil(L/3)` in the fullest class, for any partition and
    /// any geometry.
    #[test]
    fn nothing_beats_the_floor() {
        for n in [6usize, 8, 12, 15, 16] {
            for l in [3usize, 4, 6, 7, 9, 12] {
                if l > n {
                    continue;
                }
                for a in 0..3 {
                    for b in 0..3 {
                        // worst_all asserts the bound internally
                        let w = worst_all(&linear_class(a, b, n), n, l);
                        for v in w.per.iter().flatten() {
                            assert!(*v >= floor_of(l));
                        }
                    }
                }
            }
        }
    }

    /// B2. The four shatter conditions, reasoning against measurement, over
    /// the whole planning range. This re-derives the filed ground rather than
    /// quoting it.
    #[test]
    fn the_four_shatter_conditions_hold() {
        let mut checked = 0usize;
        for n in [15usize, 16, 17, 30, 31, 32, 33] {
            for l in [6usize, 9, 12, 18] {
                for a in 0..3 {
                    for b in 0..3 {
                        let cl = linear_class(a, b, n);
                        let all = worst_all(&cl, n, l);
                        for (k, g) in GEOMS.iter().enumerate() {
                            let Some(w) = all.per[k] else { continue };
                            assert_eq!(
                                w == floor_of(l),
                                linear_predicted(a, b, n, *g),
                                "({a},{b}) on {} at n={n}, L={l}: measured {w}, floor {}",
                                g.name(),
                                floor_of(l)
                            );
                            checked += 1;
                        }
                    }
                }
            }
        }
        assert!(checked > 900, "only {checked} (a,b,n,L,geometry) cases checked");
    }

    /// The tape's phase slip, named: when `a != b*n`, the worst case rises
    /// to exactly `ceil(L/3) + 1` and not further.
    #[test]
    fn the_tape_phase_slip_costs_exactly_one() {
        for n in [15usize, 16, 31, 32, 33] {
            for l in [6usize, 9, 12] {
                for a in 0..3 {
                    for b in 1..3 {
                        if a == (b * n) % 3 {
                            continue;
                        }
                        let w = worst_of(&linear_class(a, b, n), n, l, Geom::Tape).unwrap();
                        assert_eq!(
                            w,
                            floor_of(l) + 1,
                            "({a},{b}) at n={n}, L={l} slipped to {w}"
                        );
                    }
                }
            }
        }
    }

    /// B3. The theorem: no linear solution at `n = 0` or `1 (mod 3)`, and
    /// exactly `(1,2)` and `(2,1)` at `n = 2`.
    #[test]
    fn only_n_two_mod_three_admits_a_linear_optimum() {
        for n in 2..=64usize {
            let sols = linear_solutions(n);
            match n % 3 {
                0 | 1 => assert!(sols.is_empty(), "n={n} admitted {sols:?}"),
                _ => assert_eq!(sols, vec![(1, 2), (2, 1)], "n={n}"),
            }
        }
    }

    /// And what that says about v4's `idx3`: its clean sweep at `n = 32` was
    /// `(a,b) = (2,1)`, optimal by accident of `32 = 2 (mod 3)`.
    #[test]
    fn idx3_is_the_linear_arm_two_one() {
        for n in 2..=64usize {
            let idx3: Vec<u8> = (0..n * n).map(|j| (j % 3) as u8).collect();
            assert_eq!(idx3, linear_class(n % 3, 1, n), "j mod 3 at n={n}");
        }
        assert!(linear_solutions(32).contains(&(2, 1)));
        assert_eq!(32 % 3, 2);
    }

    /// The periodicity lemma's own arithmetic, and the cases the round calls
    /// impossible before it measures them.
    #[test]
    fn the_periodicity_lemma_arithmetic() {
        assert!(!periodicity_lemma(3, 3).possible);
        assert!(!periodicity_lemma(4, 3).possible);
        assert!(periodicity_lemma(5, 3).possible);
        assert!(!periodicity_lemma(6, 3).possible);
        assert!(!periodicity_lemma(6, 6).possible);
        assert!(!periodicity_lemma(15, 6).possible);
        assert!(!periodicity_lemma(16, 6).possible);
        assert!(!periodicity_lemma(31, 6).possible);
        assert!(!periodicity_lemma(33, 6).possible);
        assert!(periodicity_lemma(32, 6).possible);
        // and it is silent where 3 does not divide L
        assert!(!periodicity_lemma(16, 7).applies);
        assert!(!periodicity_lemma(16, 8).applies);
    }

    /// The lemma's necessity half, against the exact enumeration, at every
    /// small case where the two can both be run. Agreement here is what makes
    /// the lemma a result and not a hope.
    #[test]
    fn the_lemma_and_the_enumeration_agree() {
        for n in 3..=6usize {
            for l in 3..=n {
                if !l.is_multiple_of(3) {
                    continue;
                }
                let lem = periodicity_lemma(n, l);
                let ex = exact(n, l, EXACT_NODE_CAP);
                match ex.verdict {
                    Verdict::Reached(_) => assert!(lem.possible, "n={n}, L={l}"),
                    Verdict::Impossible => assert!(!lem.possible, "n={n}, L={l}"),
                    Verdict::Inconclusive => panic!("n={n}, L={l} exhausted the node cap"),
                }
            }
        }
    }

    /// The construction, where the lemma allows one: it really does reach the
    /// floor on every geometry that exists.
    #[test]
    fn the_periodic_construction_reaches_the_floor() {
        let mut built = 0usize;
        for n in [5usize, 8, 16, 17, 32] {
            for l in [3usize, 6, 9, 12, 18] {
                let lem = periodicity_lemma(n, l);
                match construct_periodic(n, l) {
                    Some(c) => {
                        assert!(lem.possible, "built one where the lemma says no: n={n}, L={l}");
                        let w = worst_all(&c, n, l);
                        assert!(w.at_floor(l), "n={n}, L={l} came in at {:?}", w.overall());
                        built += 1;
                    }
                    None => assert!(
                        !lem.possible || l > n * n,
                        "the lemma allows n={n}, L={l} and no g was found"
                    ),
                }
            }
        }
        assert!(built >= 4, "only {built} constructions were exercised");
    }

    /// The window enumerations are what the whole round is measured with, so
    /// their sizes are asserted rather than assumed.
    #[test]
    fn the_window_enumerations_are_the_right_size() {
        let n = 12usize;
        let l = 4usize;
        assert_eq!(windows(n, l, Geom::Row).len(), n * (n - l + 1));
        assert_eq!(windows(n, l, Geom::Col).len(), n * (n - l + 1));
        assert_eq!(windows(n, l, Geom::Tape).len(), n * n - l + 1);
        let want: usize = fold::arcs(n).iter().map(|&a| (a + 1).saturating_sub(l)).sum();
        assert_eq!(windows(n, l, Geom::Diag).len(), want);
        for g in GEOMS {
            for w in windows(n, l, g) {
                assert_eq!(w.len(), l);
                assert!(w.iter().all(|&i| i < n * n));
                // and the step, where the geometry has one
                if let Some(s) = g.step(n) {
                    for p in w.windows(2) {
                        assert_eq!(p[1] - p[0], s, "{} step at n={n}", g.name());
                    }
                }
            }
        }
        // a geometry with no room says so
        assert!(windows(8, 9, Geom::Row).is_empty());
        assert!(windows(8, 9, Geom::Col).is_empty());
        assert!(windows(8, 9, Geom::Diag).is_empty());
        assert!(!windows(8, 9, Geom::Tape).is_empty());
    }

    /// Energy zero and `worst == floor` are the same statement, which is what
    /// lets the annealer optimise one and the round report the other.
    #[test]
    fn energy_zero_means_at_the_floor() {
        for n in [15usize, 16, 32, 33] {
            for l in [6usize, 8, 12] {
                let e = Energy::new(n, l);
                for a in 0..3 {
                    for b in 0..3 {
                        let cl = linear_class(a, b, n);
                        let w = worst_all(&cl, n, l);
                        assert_eq!(
                            e.energy_of(&cl) == 0,
                            w.at_floor(l),
                            "({a},{b}) at n={n}, L={l}"
                        );
                    }
                }
            }
        }
    }

    /// The annealer is deterministic, which is what makes "one seed, one
    /// schedule, one budget" a checkable rule rather than a promise.
    #[test]
    fn the_annealer_is_deterministic() {
        for t in [T_HOT_FILED, t_hot_scaled(5)] {
            let a = anneal(9, 5, SEARCH_SEED, 20_000, t, T_COLD_SCALED);
            let b = anneal(9, 5, SEARCH_SEED, 20_000, t, T_COLD_SCALED);
            assert_eq!(a.best_energy, b.best_energy);
            assert_eq!(a.best, b.best);
            assert!(a.best_energy <= a.seed_energy, "the search went backwards from its seed");
        }
    }

    /// The filed schedule's acceptance rate is the diagnosis the round
    /// reports rather than deletes: at `T = 2.0` against an energy whose
    /// moves cost tens, almost nothing uphill is ever taken, so schedule A
    /// is a greedy descent. Schedule B puts the temperature on the energy's
    /// own scale and accepts materially more.
    #[test]
    fn the_filed_schedule_barely_accepts_anything() {
        let (n, l) = (30usize, 8usize);
        let a = anneal(n, l, SEARCH_SEED, 100_000, T_HOT_FILED, T_COLD_FILED);
        let b = anneal(n, l, SEARCH_SEED, 100_000, t_hot_scaled(l), T_COLD_SCALED);
        assert!(a.acceptance() < 0.05, "schedule A accepted {:.3}", a.acceptance());
        assert!(
            b.acceptance() > a.acceptance(),
            "schedule B accepted {:.3}, A {:.3}",
            b.acceptance(),
            a.acceptance()
        );
    }
}
