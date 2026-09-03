//! tui.rs -- the model, as a terminal you can drive.
//!
//! `[dependencies]` is empty and stays empty, so this is a terminal interface
//! and not a window. A windowed GUI needs crates -- `egui`, `minifb`,
//! `winit` -- and the lineage's law outranks the convenience. What it costs is
//! a title bar; what it buys is that this round builds and runs anywhere the
//! other seven do, with no network and no supply chain.
//!
//! Rendering is pure: `render` takes a `View` and returns a `String`, so the
//! whole display is testable with no terminal attached. Only `run` touches
//! stdin and stdout.
//!
//! Colour is ANSI, and the three classes take the three backgrounds so the
//! grid reads as a picture rather than as digits.

use std::io::{BufRead, Write};

use crate::lit;
use crate::optimum::{floor_of, linear_class, worst_all, GEOMS};
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
            Arm::Literature => "the catalogued perfect colouring, (y - x) mod 3".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct View {
    pub n: usize,
    pub l: usize,
    pub arm: Arm,
    /// how many grid cells to draw down each side
    pub draw: usize,
}

impl Default for View {
    fn default() -> View {
        View { n: 16, l: 8, arm: Arm::Linear(1, 2), draw: 16 }
    }
}

const RESET: &str = "\x1b[0m";
/// three backgrounds, each with a foreground that reads on it
const PAINT: [&str; 3] = [
    "\x1b[48;5;30m\x1b[38;5;231m",
    "\x1b[48;5;136m\x1b[38;5;16m",
    "\x1b[48;5;125m\x1b[38;5;231m",
];
const MARK: [char; 3] = ['.', 'o', '#'];

/// The whole display, as a string. Pure, so the tests can read it.
pub fn render(v: &View, colour: bool) -> String {
    let mut out = String::new();
    let class = v.arm.classes(v.n);
    let w = worst_all(&class, v.n, v.l);
    let floor = floor_of(v.l);

    out.push_str(&format!(
        "  eggSo v8 -- the burst floor, n = {}, L = {}, floor = {}\n  arm {}\n\n",
        v.n,
        v.l,
        floor,
        v.arm.label()
    ));

    let side = v.draw.min(v.n);
    for r in 0..side {
        out.push_str("  ");
        for c in 0..side {
            let k = class[r * v.n + c] as usize;
            if colour {
                out.push_str(PAINT[k]);
                out.push(MARK[k]);
                out.push_str(RESET);
            } else {
                out.push(MARK[k]);
            }
        }
        if r == 0 && side < v.n {
            out.push_str(&format!("   (top-left {side} of {})", v.n));
        }
        out.push('\n');
    }

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
        out.push_str(&format!("  {:<12}{:>7}{:>8}   {}\n", g.name(), cell, floor, ok));
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
            "this IS arm (1,2): {}   -- the literature colours Z^2, where there is no tape",
            if at { "at the floor" } else { "above it" }
        ),
    };
    out.push_str(&format!("\n  {says}\n"));

    let spatial = lit::spatial_at_floor(&class, v.n, v.l);
    out.push_str(&format!(
        "  spatial only (row/col/diag, no tape): {}   -- width-free, and where the prior art lives\n",
        if spatial { "at the floor" } else { "above it" }
    ));
    if spatial && !at {
        out.push_str(
            "  so THIS width fails on the tape alone: the phase slip at the row boundary.\n",
        );
    }

    out.push_str("\n  commands:  n <N>   l <L>   arm <a> <b>   arm lit   plain   colour   q\n");
    out
}

/// Drive it. Line-based, because raw-mode key handling needs OS calls this
/// crate will not take a dependency for. Every command is one line.
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
            [] => {}
            ["q"] | ["quit"] | ["exit"] => break,
            ["plain"] => colour = false,
            ["colour"] | ["color"] => colour = true,
            ["n", x] => {
                if let Ok(x) = x.parse::<usize>() {
                    if (2..=256).contains(&x) {
                        v.n = x;
                    }
                }
            }
            ["l", x] => {
                if let Ok(x) = x.parse::<usize>() {
                    if (1..=64).contains(&x) {
                        v.l = x;
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

    /// The display is pure, so it can be read without a terminal -- and it
    /// says the right thing about the case the whole round turns on.
    #[test]
    fn the_view_reports_the_tape_failure_at_n_thirty() {
        // (1,2) at n = 30, L = 12: spatial fine, tape fails. That is the
        // sentence separating us from the lattice literature.
        let v = View { n: 30, l: 12, arm: Arm::Linear(1, 2), draw: 8 };
        let s = render(&v, false);
        assert!(s.contains("fails on the tape alone"), "{s}");
        assert!(s.contains("spatial only"));
        // and at n = 32 the same arm is fine everywhere
        let v = View { n: 32, l: 12, arm: Arm::Linear(1, 2), draw: 8 };
        let s = render(&v, false);
        assert!(!s.contains("fails on the tape alone"), "{s}");
        assert!(s.contains("at the floor"));
    }

    /// The literature arm renders as what it is.
    #[test]
    fn the_literature_arm_is_labelled_as_arm_one_two() {
        let v = View { n: 16, l: 8, arm: Arm::Literature, draw: 6 };
        let s = render(&v, false);
        assert!(s.contains("(y - x) mod 3"));
        assert!(s.contains("this IS arm (1,2)"));
        assert_eq!(Arm::Literature.classes(16), Arm::Linear(1, 2).classes(16));
    }

    /// Plain mode emits no escape codes, which keeps the tests readable and
    /// the output pipe-able.
    #[test]
    fn plain_mode_has_no_escapes() {
        let v = View::default();
        assert!(!render(&v, false).contains('\x1b'));
        assert!(render(&v, true).contains('\x1b'));
    }

    /// The grid is drawn at the size asked for and never past the square.
    #[test]
    fn the_grid_is_clipped_to_the_square() {
        let v = View { n: 4, l: 3, arm: Arm::Linear(1, 2), draw: 16 };
        let s = render(&v, false);
        let rows: Vec<&str> = s
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && t.chars().all(|ch| MARK.contains(&ch))
            })
            .collect();
        assert_eq!(rows.len(), 4, "{s}");
        assert!(rows.iter().all(|r| r.trim().len() == 4));
    }

    /// Every geometry gets a row, and an absent one says so rather than
    /// printing a zero -- the trap this lineage has now fixed twice.
    #[test]
    fn an_absent_geometry_prints_as_absent() {
        let v = View { n: 6, l: 12, arm: Arm::Linear(1, 2), draw: 6 };
        let s = render(&v, false);
        assert!(s.contains("--"), "{s}");
        assert!(s.contains("n/a"), "{s}");
    }
}
