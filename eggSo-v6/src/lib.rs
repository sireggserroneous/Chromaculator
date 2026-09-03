//! eggSo v6 -- the caps.
//!
//! The twenty-first codec experiment and the seventh in the fold-native
//! lineage. eggSo-v5 ended by finding that the walls in this construction are
//! four fixed constants inherited from eggSo-v0, not the geometry, so this
//! round asks which of them are ARTIFACTS that can be raised and which are
//! INFORMATION BOUNDS that cannot.
//!
//! * `caps` -- the harness that isolates one cap at a time, by flagging an
//!   exact count in an exact distribution across the three classes.
//! * `code` -- eggSo-v0's codec, with the four caps now a PARAMETER whose
//!   default is v0's own values, so the port pin still proves the default is
//!   v0 to the decision.
//! * `seam` -- the arms and the four burst geometries, carried from v5.
//! * `optimum` -- the burst floor and its theorem, carried from v5, because
//!   the arms that reach the floor are the ones whose caps bind latest.
//! * `fold` -- the coordinate, carried unchanged since v4.
//! * `pin` -- the round against the site's OWN code, and the COPY against
//!   v5's committed record.
//! * `json` -- a hand-rolled writer, because `[dependencies]` is empty.
//!
//! v5's `cubic` and `dynamics` are deliberately NOT carried: the degree-3
//! coordinate was v5's Part 1 and nothing here needs it. Copying a module
//! forward has a price -- a copy can drift silently -- so the rule in this
//! repo is to copy only what the round uses and pin every copy.

pub mod caps;
pub mod code;
pub mod fold;
pub mod json;
pub mod optimum;
pub mod pin;
pub mod seam;
