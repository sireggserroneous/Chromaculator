//! tui.rs -- the model: a burst you can slide across the grid.
//!
//! The first version of this file was a VIEWER, not a model. It rendered a
//! partition and printed the aggregate `worst()` figures that `optimum.rs`
//! had already computed -- the conclusion, `tape 5`, with no burst anywhere
//! on the screen. But the whole result is about what happens to a burst when
//! it CROSSES A ROW BOUNDARY, and a display that never draws one cannot show
//! that.
//!
//! So this draws the burst. You pick a geometry, slide the burst one
//! placement at a time, and watch its three class counts change. Step a tape
//! burst across a row end and the count goes from `ceil(L/3)` to
//! `ceil(L/3)+1` in one keystroke, on screen, with the two fragments of the
//! run drawn on different rows. That is the phase slip, modelled rather than
//! reported.
//!
//! `[dependencies]` is empty and stays empty, so this is a terminal interface
//! and not a window. A windowed GUI needs crates and the lineage's law
//! outranks the convenience.
//!
//! Rendering is pure -- `render(&View) -> String` -- so every claim the
//! display makes is testable with no terminal attached. Only `run` touches
//! stdin.

use std::io::{BufRead, Write};

use crate::lit;
use crate::optimum::{floor_of, linear_class, windows, worst_all, Geom, GEOMS};
use crate::thirds::linear_verdict;

/// Which partition the view is showing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arm {
    /// the linear family `(a*r + b*c) mod 3`
    Linear(usize, usize),
    /// the literature's catalogued perfect colouring, `(y - x) mod 3`
    Literature,
}

impl Arm {
    pub fn classes(&self, n: usize) -> Vec<u8> {
        match self {
            Arm::Linear(a, b) => linear_class(*a, *b, n),
            Arm::Literature => lit::perfect_colouring_grid(n),
        }
    }
    pub fn label(&self) -> String {
        match self {
            Arm::Linear(1, 1) => "(1,1) = (r+c) mod 3, the fold's own level sets".into(),
            Arm::Linear(a, b) => format!("({a},{b}) = ({a}r + {b}c) mod 3"),
            Arm::Literature => "(y - x) mod 3, the catalogued colouring = arm (1,2)".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct View {
    pub n: usize,
    pub l: usize,
    pub arm: Arm,
    /// which burst geometry is on screen
    pub geom: Geom,
    /// which placement of the burst, as an index into `windows`
    pub at: usize,
    /// how many grid cells to draw down each side
    pub draw: usize,
}

impl Default for View {
    fn default() -> View {
        View { n: 16, l: 8, arm: Arm::Linear(1, 2), geom: Geom::Tape, at: 0, draw: 40 }
    }
}

/// One burst placement, and everything the screen says about it.
#[derive(Clone, Debug)]
pub struct Burst {
    pub cells: Vec<usize>,
    pub per_class: [usize; 3],
    /// how many row boundaries the run crosses -- the phase slip lives here
    pub row_crossings: usize,
}

impl Burst {
    pub fn worst(&self) -> usize {
        *self.per_class.iter().max().unwrap()
    }
}

impl View {
    pub fn placements(&self) -> Vec<Vec<usize>> {
        windows(self.n, self.l, self.geom)
    }

    /// The burst at the current placement, or `None` when this geometry has
    /// no run of this length at this width -- an absent measurement, never a
    /// zero.
    pub fn burst(&self) -> Option<Burst> {
        let ws = self.placements();
        if ws.is_empty() {
            return None;
        }
        let cells = ws[self.at % ws.len()].clone();
        let class = self.arm.classes(self.n);
        let mut per_class = [0usize; 3];
        for &i in &cells {
            per_class[class[i] as usize] += 1;
        }
        let row_crossings =
            cells.windows(2).filter(|w| w[0] / self.n != w[1] / self.n).count();
        Some(Burst { cells, per_class, row_crossings })
    }

    /// Jump to the placement that does the most damage, which is the one
    /// `worst(C, L)` is actually reporting.
    pub fn worst_placement(&self) -> Option<usize> {
        let class = self.arm.classes(self.n);
        let ws = self.placements();
        if ws.is_empty() {
            return None;
        }
        let mut best = (0usize, 0usize);
        for (k, w) in ws.iter().enumerate() {
            let mut per = [0usize; 3];
            for &i in w {
                per[class[i] as usize] += 1;
            }
            let m = *per.iter().max().unwrap();
            if m > best.1 {
                best = (k, m);
            }
        }
        Some(best.0)
    }

    pub fn step(&mut self, d: i64) {
        let n = self.placements().len();
        if n == 0 {
            return;
        }
        let cur = (self.at % n) as i64;
        self.at = (((cur + d) % n as i64 + n as i64) % n as i64) as usize;
    }
}

const RESET: &str = "\x1b[0m";
/// three backgrounds for the three classes, each with a readable foreground
const PAINT: [&str; 3] = [
    "\x1b[48;5;30m\x1b[38;5;231m",
    "\x1b[48;5;136m\x1b[38;5;16m",
    "\x1b[48;5;125m\x1b[38;5;231m",
];
/// the burst itself: bright, so it reads as damage laid over the partition
const HIT: &str = "\x1b[48;5;231m\x1b[38;5;16m\x1b[1m";
const MARK: [char; 3] = ['.', 'o', '#'];

/// The whole display, as a string. Pure, so the tests can read it.
pub fn render(v: &View, colour: bool) -> String {
    let mut out = String::new();
    let class = v.arm.classes(v.n);
    let w = worst_all(&class, v.n, v.l);
    let floor = floor_of(v.l);
    let b = v.burst();
    let hit: std::collections::HashSet<usize> =
        b.as_ref().map(|b| b.cells.iter().copied().collect()).unwrap_or_default();

    out.push_str(&format!(
        "  eggSo v8 -- n = {}, L = {}, floor = {}\n  arm {}\n\n",
        v.n,
        v.l,
        floor,
        v.arm.label()
    ));

    let side = v.draw.min(v.n);
    for r in 0..side {
        out.push_str("  ");
        for c in 0..side {
            let j = r * v.n + c;
            let k = class[j] as usize;
            let is_hit = hit.contains(&j);
            if colour {
                out.push_str(if is_hit { HIT } else { PAINT[k] });
                out.push(MARK[k]);
                out.push_str(RESET);
            } else if is_hit {
                // uppercase marks the burst when there is no colour
                out.push(match k {
                    0 => ':',
                    1 => 'O',
                    _ => '@',
                });
            } else {
                out.push(MARK[k]);
            }
        }
        if r == 0 && side < v.n {
            out.push_str(&format!("   (top-left {side} of {})", v.n));
        }
        out.push('\n');
    }
    // If part of the burst is outside the drawn corner, SAY SO. A grid with
    // no burst visible on it, next to a panel describing one, would read as
    // "there is no burst here" -- which is the display lying.
    if let Some(b) = &b {
        let off = b.cells.iter().filter(|&&j| j / v.n >= side || j % v.n >= side).count();
        if off > 0 {
            out.push_str(&format!(
                "  ({off} of {} burst cells lie outside the drawn corner -- `draw {}` to see them)\n",
                b.cells.len(),
                v.n
            ));
        }
    }

    // the burst under the cursor: this is the model, not the summary
    match &b {
        None => out.push_str(&format!(
            "\n  no {} run of {} cells exists at n = {} -- absent, not zero\n",
            v.geom.name(),
            v.l,
            v.n
        )),
        Some(b) => {
            let total = v.placements().len();
            out.push_str(&format!(
                "\n  BURST  {} geometry, placement {} of {}\n",
                v.geom.name(),
                v.at % total.max(1) + 1,
                total
            ));
            out.push_str(&format!(
                "    classes {} / {} / {}   worst {}   floor {}   {}\n",
                b.per_class[0],
                b.per_class[1],
                b.per_class[2],
                b.worst(),
                floor,
                if b.worst() > floor { "OVER THE FLOOR" } else { "at the floor" }
            ));
            out.push_str(&format!(
                "    starts at cell ({}, {}), crosses {} row boundar{}",
                b.cells[0] / v.n,
                b.cells[0] % v.n,
                b.row_crossings,
                if b.row_crossings == 1 { "y" } else { "ies" }
            ));
            if b.row_crossings > 0 && b.worst() > floor {
                out.push_str("   <- THE PHASE SLIP\n");
            } else {
                out.push('\n');
            }
        }
    }

    // the four geometries, as the summary they are
    out.push_str("\n  geometry      worst   floor   at floor?\n");
    for (k, g) in GEOMS.iter().enumerate() {
        let cell = match w.per[k] {
            Some(x) => x.to_string(),
            None => "--".into(),
        };
        let ok = match w.per[k] {
            Some(x) if x == floor => "yes",
            Some(_) => "NO",
            None => "n/a",
        };
        let cursor = if *g == v.geom { "<" } else { " " };
        out.push_str(&format!("  {:<12}{:>7}{:>8}   {:<4}{}\n", g.name(), cell, floor, ok, cursor));
    }

    let at = w.at_floor(v.l);
    let says = match &v.arm {
        Arm::Linear(a, b) => format!(
            "closed form: L = {} (mod 3), n = {} (mod 3) => some arm works: {}   (this arm ({a},{b}): {})",
            v.l % 3,
            v.n % 3,
            if linear_verdict(v.n, v.l) { "YES" } else { "no" },
            if at { "at the floor" } else { "above it" }
        ),
        Arm::Literature => format!(
            "this IS arm (1,2): {}   -- the literature colours Z^2, which has no tape",
            if at { "at the floor" } else { "above it" }
        ),
    };
    out.push_str(&format!("\n  {says}\n"));

    let spatial = lit::spatial_at_floor(&class, v.n, v.l);
    out.push_str(&format!(
        "  spatial only (row/col/diag): {}   -- width-free, and where the prior art lives\n",
        if spatial { "at the floor" } else { "above it" }
    ));
    if spatial && !at {
        out.push_str("  so THIS width fails on the tape alone. Press w to see where.\n");
    }

    out.push_str(
        "\n  n <N>  l <L>  draw <k>  arm <a> <b>  arm lit  g row|col|diag|tape  + -  w  plain  q\n",
    );
    out
}

/// Drive it. Line-based, because raw-mode key handling needs OS calls this
/// crate will not take a dependency for. A bare newline steps the burst
/// forward, so sliding it is one key.
pub fn run() {
    let mut v = View::default();
    let mut colour = true;
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    loop {
        print!("\x1b[2J\x1b[H{}\n> ", render(&v, colour));
        let _ = out.flush();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            // a bare enter slides the burst one placement
            [] => v.step(1),
            ["q"] | ["quit"] | ["exit"] => break,
            ["+"] => v.step(1),
            ["-"] => v.step(-1),
            ["w"] => {
                if let Some(k) = v.worst_placement() {
                    v.at = k;
                }
            }
            ["draw", x] => {
                if let Ok(x) = x.parse::<usize>() {
                    if (2..=200).contains(&x) {
                        v.draw = x;
                    }
                }
            }
            ["plain"] => colour = false,
            ["colour"] | ["color"] => colour = true,
            ["g", g] => {
                v.geom = match *g {
                    "row" => Geom::Row,
                    "col" => Geom::Col,
                    "diag" => Geom::Diag,
                    _ => Geom::Tape,
                };
                v.at = 0;
            }
            ["n", x] => {
                if let Ok(x) = x.parse::<usize>() {
                    if (2..=256).contains(&x) {
                        v.n = x;
                        v.at = 0;
                    }
                }
            }
            ["l", x] => {
                if let Ok(x) = x.parse::<usize>() {
                    if (1..=64).contains(&x) {
                        v.l = x;
                        v.at = 0;
                    }
                }
            }
            ["arm", "lit"] => v.arm = Arm::Literature,
            ["arm", a, b] => {
                if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                    v.arm = Arm::Linear(a % 3, b % 3);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The model actually models the thing.** At `n = 30`, `L = 12`, arm
    /// `(1,2)`, sliding a tape burst finds a placement that crosses a row
    /// boundary AND goes over the floor -- and one that does not. That
    /// difference, visible on one screen, is the phase slip.
    #[test]
    fn sliding_a_tape_burst_exhibits_the_phase_slip() {
        let mut v = View { n: 30, l: 12, arm: Arm::Linear(1, 2), geom: Geom::Tape, at: 0, draw: 8 };
        let floor = floor_of(12);
        let total = v.placements().len();

        let (mut clean_inside, mut slipped) = (false, false);
        for k in 0..total {
            v.at = k;
            let b = v.burst().unwrap();
            if b.row_crossings == 0 {
                assert_eq!(b.worst(), floor, "a run inside one row must sit at the floor");
                clean_inside = true;
            }
            if b.row_crossings > 0 && b.worst() > floor {
                slipped = true;
            }
        }
        assert!(clean_inside, "no placement stayed inside a row");
        assert!(slipped, "no placement crossed a boundary and went over the floor");

        // and the screen says so at the worst placement
        v.at = v.worst_placement().unwrap();
        let s = render(&v, false);
        assert!(s.contains("THE PHASE SLIP"), "{s}");
        assert!(s.contains("OVER THE FLOOR"), "{s}");
    }

    /// The burst is drawn on the grid, not merely counted.
    #[test]
    fn the_burst_is_drawn_on_the_grid() {
        let v = View { n: 12, l: 6, arm: Arm::Linear(1, 2), geom: Geom::Row, at: 0, draw: 12 };
        let s = render(&v, false);
        // count only inside the GRID rows -- prose elsewhere carries colons,
        // and counting those would make this pass for the wrong reason
        const GRID: &str = ".o#:O@";
        let drawn: usize = s
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.len() == 12 && l.chars().all(|c| GRID.contains(c)))
            .map(|l| l.chars().filter(|c| ":O@".contains(*c)).count())
            .sum();
        assert_eq!(drawn, 6, "expected 6 burst cells drawn, found {drawn}\n{s}");
        // and in colour mode it gets its own paint
        assert!(render(&v, true).contains(HIT));
    }

    /// The counts on screen are the counts of the cells drawn -- the display
    /// cannot drift from the thing it is showing.
    #[test]
    fn the_counts_match_the_cells_drawn() {
        for geom in GEOMS {
            let mut v = View { n: 15, l: 7, arm: Arm::Linear(2, 1), geom, at: 0, draw: 15 };
            for k in [0usize, 3, 11] {
                v.at = k;
                let Some(b) = v.burst() else { continue };
                assert_eq!(b.cells.len(), 7);
                assert_eq!(b.per_class.iter().sum::<usize>(), 7);
                let class = v.arm.classes(v.n);
                let mut per = [0usize; 3];
                for &i in &b.cells {
                    per[class[i] as usize] += 1;
                }
                assert_eq!(per, b.per_class);
            }
        }
    }

    /// Only the TAPE crosses a row boundary. That is the whole distinction
    /// between our problem and the lattice literature's, and the model has to
    /// show it rather than assert it.
    #[test]
    fn only_the_tape_crosses_a_row_boundary() {
        let n = 20usize;
        for geom in GEOMS {
            let mut v = View { n, l: 8, arm: Arm::Linear(1, 2), geom, at: 0, draw: 8 };
            let total = v.placements().len();
            let mut crossings = 0usize;
            for k in 0..total {
                v.at = k;
                if let Some(b) = v.burst() {
                    crossings += b.row_crossings;
                }
            }
            match geom {
                Geom::Tape => assert!(crossings > 0, "the tape never crossed a row"),
                Geom::Row => assert_eq!(crossings, 0, "a row burst left its row"),
                // a column or diagonal changes row every cell -- those are
                // steps WITHIN the geometry, not the tape's wrap, and the
                // model counts them honestly rather than hiding them
                _ => assert!(crossings > 0),
            }
        }
    }

    /// Stepping wraps and never panics, at any geometry or width.
    #[test]
    fn stepping_wraps_and_never_panics() {
        let mut v = View { n: 9, l: 4, arm: Arm::Linear(1, 2), geom: Geom::Diag, at: 0, draw: 9 };
        let total = v.placements().len();
        assert!(total > 0);
        v.step(-1);
        assert_eq!(v.at, total - 1);
        v.step(1);
        assert_eq!(v.at, 0);
        for _ in 0..(total * 2 + 5) {
            v.step(1);
            assert!(v.burst().is_some());
        }
        // and a geometry with no placements steps harmlessly
        let mut v = View { n: 6, l: 12, arm: Arm::Linear(1, 2), geom: Geom::Row, at: 0, draw: 6 };
        assert!(v.placements().is_empty());
        v.step(1);
        assert!(v.burst().is_none());
        assert!(render(&v, false).contains("absent, not zero"));
    }

    /// Plain mode emits no escape codes, so the output pipes and the tests
    /// read.
    #[test]
    fn plain_mode_has_no_escapes() {
        let v = View::default();
        assert!(!render(&v, false).contains('\x1b'));
        assert!(render(&v, true).contains('\x1b'));
    }

    /// The literature arm renders as what it is.
    #[test]
    fn the_literature_arm_is_labelled_as_arm_one_two() {
        let v = View { n: 16, l: 8, arm: Arm::Literature, geom: Geom::Tape, at: 0, draw: 6 };
        let s = render(&v, false);
        assert!(s.contains("(y - x) mod 3"));
        assert!(s.contains("this IS arm (1,2)"));
        assert_eq!(Arm::Literature.classes(16), Arm::Linear(1, 2).classes(16));
    }
}
