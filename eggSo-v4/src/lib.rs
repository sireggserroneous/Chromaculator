//! eggSo v4 -- the fold is a basin boundary.
//!
//! The nineteenth codec experiment and the fifth in the fold-native lineage,
//! and the first written in Rust. The modules are the round's three parts:
//!
//!   * `fold`     the coordinate `rho = 2^(d-(n-1))` and the anti-transpose in it
//!   * `dynamics` Cayley's guess-and-fix, and where it lands for two roots and three
//!   * `code`     eggSo-v0's codec, ported, with the class assignment as a parameter
//!   * `guess`    can a decoder guess and fix? (no, and the reason is the round)
//!   * `seam`     what the fold's forced 1/n seam costs against a chosen one
//!   * `pin`      the port and the coordinate against the site's OWN code, via node
//!   * `json`     a hand-rolled writer, because `[dependencies]` is empty
//!
//! The crate is a library with a thin binary over it so that `pub` items are
//! API rather than dead code: most of this surface is exercised by
//! `cargo test` and by the `audit` subcommand, and splitting it this way lets
//! `cargo clippy --all-targets -- -D warnings` stay clean without a single
//! suppression.

pub mod code;
pub mod dynamics;
pub mod fold;
pub mod guess;
pub mod json;
pub mod pin;
pub mod seam;
