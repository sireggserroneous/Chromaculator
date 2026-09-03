//! eggSo v5 -- Cayley's unfinished business, and the burst optimum.
//!
//! The twentieth codec experiment and the sixth in the fold-native lineage.
//! Two threads out of v4's accident, plus a rule for the whole round.
//!
//! v4 closed the site README's oldest question -- the fold is the Julia set
//! of `z -> z^2` and the anti-transpose is the inversion between its two
//! Fatou basins -- and discovered by accident that **the separation statistic
//! eggSo-v0's entire verdict rested on moves no error channel at all**. What
//! the geometry actually costs is BURST SPREAD.
//!
//! So:
//!
//! * `cubic` -- what this grid looks like at DEGREE THREE, which is the
//!   question Cayley could state and not see. Its bar is the picture and the
//!   name, not a channel win, and that is filed in PREDICTIONS.md first.
//! * `optimum` -- which three-class partition actually minimises a burst: the
//!   figure of merit that turns out to matter and has never been optimised.
//!   The bound, the linear family's theorem, the periodicity lemma, the search.
//! * `seam` -- v4's arms, plus the three burst geometries v4 did not have.
//! * `dynamics` -- v4's, plus the cell to complex coordinate Part 1 needs.
//! * `fold` -- v4's coordinate `rho = 2^(d-(n-1))`, carried unchanged.
//! * `code` -- eggSo-v0's codec, with the class assignment a parameter.
//! * `pin` -- the round against the site's OWN code, and the COPY against
//!   v4's committed record.
//! * `json` -- a hand-rolled writer, because `[dependencies]` is empty.
//!
//! **Copy forward rather than depend.** Each round in this repo is a frozen
//! record and its own crate; `codegg-v13` carries `armor11.rs` and
//! `mix9..12.rs` for exactly this reason. A path dependency on v4 would let
//! v5's recorded numbers drift when v4 changes. So the shared modules are
//! copied -- and `pin::v4_figures` pins the copy to v4's committed
//! `measured-*.json`, so a silent divergence is a failed gate.
//!
//! The crate is a library with a thin binary over it so that `pub` items are
//! API rather than dead code, which lets
//! `cargo clippy --all-targets -- -D warnings` stay clean with no
//! suppressions, as v4 holds.

pub mod code;
pub mod cubic;
pub mod dynamics;
pub mod fold;
pub mod json;
pub mod optimum;
pub mod pin;
pub mod seam;
