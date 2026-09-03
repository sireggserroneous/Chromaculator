//! eggSo v8 -- the literature, run rather than read.
//!
//! The twenty-third codec experiment. v7 closed the mathematics and the
//! novelty question got an opinion instead of a check. Vladimir: *"I want it
//! answered properly please. Run the literature in rust ofcouse. And model it
//! in rust also with a small gui."*
//!
//! * `lit` -- the prior art's constructions, implemented from THEIR
//!   definitions, so that agreement with ours is a measurement rather than an
//!   artefact of how we chose to restate them.
//! * `tui` -- the model: a zero-dependency terminal explorer for the grid,
//!   its four burst geometries and the verdict at any `(n, L)`.
//! * everything else is carried from v7 and pinned, because v8 checks v7's
//!   whole characterisation against the literature and therefore needs the
//!   machinery that produced it.
//!
//! `[dependencies]` is empty, as it has been for every Rust round here. That
//! is why the model is a terminal interface and not a window: a windowed GUI
//! needs crates, and the lineage's law outranks the convenience.

pub mod caps;
pub mod code;
pub mod fold;
pub mod json;
pub mod lit;
pub mod optimum;
pub mod pin;
pub mod seam;
pub mod thirds;
pub mod tui;
