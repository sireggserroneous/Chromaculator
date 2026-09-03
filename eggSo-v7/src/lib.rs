//! eggSo v7 -- the last two things worth doing.
//!
//! The twenty-second codec experiment and the eighth in the fold-native
//! lineage. v6's verdict was that the construction's ceiling is `check_bits`
//! and that every road onward leads to Reed-Solomon, which this repo has
//! already reached from two other doors. So there is no engineering round
//! left, and this one is deliberately the two items that survive that
//! verdict and nothing else:
//!
//! * **the mathematics.** Finish the characterisation of the burst floor for
//!   `L` not divisible by 3, and settle the three cases v6 left
//!   INCONCLUSIVE. See `thirds`.
//! * **the safety fix.** v6's C6 failed: raising one cap alone produced 2
//!   silent wrong answers in 100. v6's answer was to calibrate the coupled
//!   budget; v7's is to make truncation unforgeable, which is safe at any
//!   cap setting and needs no arithmetic. See `code::Caps`.
//!
//! Everything else is carried and pinned: `fold` since v4, `code` and `seam`
//! and `optimum` and `pin` and `json` from v6, `caps` from v6.

pub mod caps;
pub mod code;
pub mod fold;
pub mod json;
pub mod optimum;
pub mod pin;
pub mod seam;
pub mod thirds;
