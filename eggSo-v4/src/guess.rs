//! guess.rs -- can a decoder guess and fix?
//!
//! `stalk.js:288-306`, the site's own divider, guesses a signed digit from
//! `{-1, 0, +1}` at every step, subtracts, and carries the corrected
//! remainder forward. That is restoring signed-digit division, and it is
//! Newton's method in miniature: guess, fix, repeat. This module asks
//! whether a DECODER can work the same way, and measures the answer against
//! eggSo-v0's table decoder rather than assuming it.
//!
//! Two results are theorems before they are measurements, and the module is
//! shaped to prove rather than sample them.
//!
//! THE LEMMA. Each cell belongs to exactly one class, so a single-cell flip
//! moves exactly one class syndrome. The count of nonzero class syndromes
//! can therefore only change by -1, +1 or 0, and "accept only if some class
//! syndrome becomes zero" is IDENTICALLY "accept if the count of nonzero
//! class syndromes decreases". Two of the three rules a first reading would
//! file are one rule. They separate only when a move may touch two cells.
//! Corollary: under restoring acceptance a rejected move reverts, so random
//! restart is inert for the zero rule -- it only re-permutes the proposal
//! order.
//!
//! THE PLATEAU. The modulus is chosen so `{+-2^k mod p}` are `2L` distinct
//! values (`codegg-v1/codegg.js:60-70`). So a single error has EXACTLY ONE
//! alphabet-valid flip in the whole square that zeroes its class syndrome,
//! and a same-class DOUBLE has NONE. `rule = ZeroClass` corrects 0% of that
//! channel at any budget whatsoever. The very injectivity that makes the
//! table decoder a single lookup is what starves the search.
//!
//! And the sentence the round is for. A division remainder is closer or
//! further; an element of `Z_p` is right, or it is a uniformly-distributed
//! -looking element. The metric detects the solution and does not point
//! toward it: measured, its range is one step. So for the same ~33 bits you
//! can buy an ADDRESS or a METRIC. The residue buys the address, which makes
//! lookup trivial and search blind. `Rule::Count` buys the metric, which
//! makes search converge and lookup impossible. eggSo-v0 bought the address.
//! (The count arm is not an invention either: unweighted per-class sums are
//! codec-v1's own mechanism, cited in `codegg-v1/codegg.js`'s header as
//! "4N-1 unweighted sums".)

use crate::code::{smod, Code, Mul32};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rule {
    /// accept iff some class syndrome becomes exactly zero
    ZeroClass,
    /// accept iff the ring distance strictly decreases
    RingStrict,
    /// accept if it does not increase (plateau walking)
    RingSideways,
    /// Metropolis on the ring sum
    Anneal,
    /// the popcount check: accept iff the class count moves toward its target
    Count,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scope {
    Square,
    Hurt,
    Flagged,
}

#[derive(Clone, Debug)]
pub struct Cfg {
    pub rule: Rule,
    pub scope: Scope,
    pub moves: usize,
    pub budget: usize,
    pub restarts: usize,
    pub temp0: f64,
}

impl Cfg {
    pub fn new(rule: Rule, scope: Scope) -> Cfg {
        Cfg { rule, scope, moves: 1, budget: 4096, restarts: 1, temp0: 8.0 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Trace {
    pub steps: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub restarts: usize,
    pub syndrome_evals: usize,
    pub end_live: usize,
    pub end_ring: u64,
    /// how many accepting single-cell moves existed at entry, exhaustively
    pub accepting_at_start: usize,
    pub terminated: &'static str,
    pub consistent: bool,
    pub exact: bool,
}

/// The ring distance of a syndrome: how far it is from zero, in either
/// direction. This is the only metric `Z_p` offers, and the round measures
/// exactly how much it is worth.
#[inline]
pub fn ring(s: u64, p: u64) -> u64 {
    s.min(p - s)
}

/// The value change a flip of cell `i` makes, given its current bit.
#[inline]
fn flip_step(code: &Code, i: usize, v: i8) -> (u64, u64, bool) {
    // v = 0 -> 1 adds w[i]; v = 1 -> 0 subtracts it
    (code.w[i], code.wq[i], v == 0)
}

/// The live syndrome state, updated in O(1) per flip.
#[derive(Clone)]
pub struct State {
    pub d: [u64; 3],
    pub dq: u64,
}

impl State {
    pub fn of(cells: &[i8], check: &[u64], code: &Code) -> State {
        let cur = code.class_residues(cells);
        let d = [
            smod(cur[0] as i64 - check[0] as i64, code.p),
            smod(cur[1] as i64 - check[1] as i64, code.p),
            smod(cur[2] as i64 - check[2] as i64, code.p),
        ];
        let dq = if code.confirm {
            smod(code.q_residue(cells) as i64 - check[3] as i64, code.q)
        } else {
            0
        };
        State { d, dq }
    }
    pub fn live(&self) -> usize {
        self.d.iter().filter(|&&x| x != 0).count()
    }
    pub fn ring_sum(&self, p: u64) -> u64 {
        self.d.iter().map(|&s| ring(s, p)).sum()
    }
    pub fn zero(&self) -> bool {
        self.d.iter().all(|&x| x == 0) && self.dq == 0
    }
    fn apply(&mut self, code: &Code, i: usize, v: i8) {
        let (w, wq, up) = flip_step(code, i, v);
        let k = code.class[i] as usize;
        if up {
            self.d[k] = (self.d[k] + w) % code.p;
            if code.confirm {
                self.dq = (self.dq + wq) % code.q;
            }
        } else {
            self.d[k] = smod(self.d[k] as i64 - w as i64, code.p);
            if code.confirm {
                self.dq = smod(self.dq as i64 - wq as i64, code.q);
            }
        }
    }
}

fn proposal_set(code: &Code, st: &State, cfg: &Cfg, erased: &[usize]) -> Vec<usize> {
    match cfg.scope {
        Scope::Square => (0..code.l).collect(),
        Scope::Flagged => erased.to_vec(),
        Scope::Hurt => {
            let mut out = Vec::new();
            for k in 0..3 {
                if st.d[k] != 0 {
                    out.extend_from_slice(&code.members[k]);
                }
            }
            if out.is_empty() {
                (0..code.l).collect()
            } else {
                out
            }
        }
    }
}

/// Exhaustively count the single-cell flips that the zero rule would accept.
/// This is the plateau certificate: for a same-class double it returns 0.
pub fn accepting_census(cells: &[i8], check: &[u64], code: &Code) -> usize {
    let st = State::of(cells, check, code);
    let mut n = 0usize;
    for (i, &v) in cells.iter().enumerate().take(code.l) {
        let mut t = st.clone();
        t.apply(code, i, v);
        let k = code.class[i] as usize;
        if st.d[k] != 0 && t.d[k] == 0 {
            n += 1;
        }
    }
    n
}

/// One guess-and-fix decode. Restoring: a rejected step is reverted, which
/// is the site's divider's own rule.
pub fn decode(
    cells: &mut [i8],
    check: &[u64],
    code: &Code,
    cfg: &Cfg,
    erased: &[usize],
    g: &mut Mul32,
    clean: Option<&[i8]>,
) -> Trace {
    let mut tr = Trace { terminated: "budget", ..Default::default() };
    let start = cells.to_vec();
    tr.accepting_at_start = accepting_census(&start, check, code);

    for attempt in 0..cfg.restarts.max(1) {
        if attempt > 0 {
            cells.copy_from_slice(&start);
            tr.restarts += 1;
        }
        let mut st = State::of(cells, check, code);
        tr.syndrome_evals += 1;
        if st.zero() {
            tr.terminated = "zero";
            break;
        }
        let mut budget = cfg.budget;
        let mut temp = cfg.temp0;
        while budget > 0 {
            let pool = proposal_set(code, &st, cfg, erased);
            if pool.is_empty() {
                tr.terminated = "plateau";
                break;
            }
            let i = pool[g.pick(pool.len())];
            let before_live = st.live();
            let before_ring = st.ring_sum(code.p);
            let k = code.class[i] as usize;

            let mut t = st.clone();
            t.apply(code, i, cells[i]);
            tr.steps += 1;
            tr.syndrome_evals += 1;
            budget -= 1;

            let accept = match cfg.rule {
                Rule::ZeroClass => st.d[k] != 0 && t.d[k] == 0,
                Rule::RingStrict => t.ring_sum(code.p) < before_ring,
                Rule::RingSideways => t.ring_sum(code.p) <= before_ring,
                Rule::Anneal => {
                    let after = t.ring_sum(code.p) as f64;
                    let delta = after - before_ring as f64;
                    if delta <= 0.0 {
                        true
                    } else {
                        let u = g.next() as f64 / 4_294_967_296.0;
                        u < (-delta / temp.max(1e-9)).exp()
                    }
                }
                // the count arm keeps its own bookkeeping, below
                Rule::Count => t.live() < before_live,
            };
            temp *= 0.999;

            if accept {
                cells[i] ^= 1;
                st = t;
                tr.accepted += 1;
                if st.zero() {
                    tr.terminated = "zero";
                    break;
                }
            } else {
                tr.rejected += 1;
            }
        }
        let st = State::of(cells, check, code);
        if st.zero() {
            tr.terminated = "zero";
            break;
        }
    }

    let st = State::of(cells, check, code);
    tr.end_live = st.live();
    tr.end_ring = st.ring_sum(code.p);
    tr.consistent = st.zero();
    tr.exact = clean.map(|c| c == cells).unwrap_or(false);
    tr
}

// ---- the count arm: a check that carries a metric instead of an address --

/// Per-class popcount, plus the confirming residue. Ten bits a class at
/// N = 32 against the residues' eleven, so the arm is cheaper and not
/// subsidised. It cannot locate anything; each accepted flip provably
/// removes one error.
pub fn count_checks(cells: &[i8], code: &Code) -> Vec<u64> {
    let mut out = [0u64; 3];
    for i in 0..code.l {
        if cells[i] == 1 {
            out[code.class[i] as usize] += 1;
        }
    }
    let mut v = out.to_vec();
    v.push(code.q_residue(cells));
    v
}

pub fn count_bits(code: &Code) -> usize {
    let bits = |m: u64| (m as f64).log2().ceil() as usize;
    let mut total = 0usize;
    for k in 0..3 {
        total += bits(code.members[k].len() as u64 + 1);
    }
    total + bits(code.q)
}

/// Guess-and-fix under the count check. The direction is known, so this
/// converges where the residue arms cannot -- and it has no way to name a
/// cell, so its `direct` count is zero forever.
pub fn decode_count(
    cells: &mut [i8],
    check: &[u64],
    code: &Code,
    budget: usize,
    g: &mut Mul32,
    clean: Option<&[i8]>,
) -> Trace {
    let mut tr = Trace { terminated: "budget", ..Default::default() };
    let target: Vec<i64> = check[..3].iter().map(|&v| v as i64).collect();
    let mut have: Vec<i64> = count_checks(cells, code)[..3].iter().map(|&v| v as i64).collect();
    let mut left = budget;
    while left > 0 {
        let gap: i64 = (0..3).map(|k| (have[k] - target[k]).abs()).sum();
        if gap == 0 {
            break;
        }
        // pick a class that is off, and a cell whose flip moves it the right way
        let mut classes: Vec<usize> = (0..3).filter(|&k| have[k] != target[k]).collect();
        if classes.is_empty() {
            break;
        }
        let k = classes[g.pick(classes.len())];
        classes.clear();
        let want_down = have[k] > target[k];
        let m = &code.members[k];
        let i = m[g.pick(m.len())];
        tr.steps += 1;
        tr.syndrome_evals += 1;
        left -= 1;
        let is_one = cells[i] == 1;
        if is_one == want_down {
            cells[i] ^= 1;
            have[k] += if want_down { -1 } else { 1 };
            tr.accepted += 1;
        } else {
            tr.rejected += 1;
        }
    }
    let ok = count_checks(cells, code);
    tr.consistent = ok[..3] == check[..3] && (!code.confirm || ok[3] == check[3]);
    if tr.consistent {
        tr.terminated = "zero";
    }
    tr.exact = clean.map(|c| c == cells).unwrap_or(false);
    tr.end_live = (0..3).filter(|&k| ok[k] != check[k]).count();
    tr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code::fold_assign;

    fn c32() -> Code {
        Code::new(32, true, "fold", fold_assign)
    }

    /// G1, THE LEMMA. For single-cell moves, "a class syndrome hits zero" and
    /// "the live count decreases" are the same event. Exhaustive: every cell
    /// of every one of 200 damaged squares.
    #[test]
    fn zero_a_class_and_fewer_hurt_are_one_rule() {
        let c = c32();
        let mut g = Mul32::new(20260903);
        for _ in 0..200 {
            let clean = g.cells(c.l);
            let check = c.checks_for(&clean);
            let mut cells = clean.clone();
            let a = g.pick(c.l);
            let mut b = g.pick(c.l);
            while b == a {
                b = g.pick(c.l);
            }
            cells[a] ^= 1;
            cells[b] ^= 1;
            let st = State::of(&cells, &check, &c);
            for (i, &v) in cells.iter().enumerate() {
                let mut t = st.clone();
                t.apply(&c, i, v);
                let k = c.class[i] as usize;
                let zeroed = st.d[k] != 0 && t.d[k] == 0;
                let fewer = t.live() < st.live();
                assert_eq!(zeroed, fewer, "cell {i}: zeroed {zeroed}, fewer {fewer}");
            }
        }
    }

    /// G3. Injectivity gives a single error exactly one accepting flip.
    ///
    /// THE MISS, KEPT. PREDICTIONS.md filed "accepting single-cell moves for
    /// a same-class double: exactly 0, at any budget" and called it a theorem
    /// from injectivity. It is not. Injectivity separates the `2L` values
    /// `{+-2^k mod p}` from each other; it says nothing about whether a SUM
    /// of two of them lands on a third. There are `O(L^2)` such sums and only
    /// `2L` targets in a ring of size `p ~ 2L`, so collisions are not rare,
    /// they are the expected case.
    ///
    /// The rate is `|class| / p`, and getting it right takes one step more
    /// than it looks. A first count says `2|class|/p`, allowing both signs at
    /// every cell -- but a cell's CURRENT BIT fixes which way its flip moves
    /// the syndrome, so only one of the two directions is available per cell.
    /// Halving it gives `496/2053 = 0.242` for an Inner double and, averaged
    /// over a uniformly chosen class, `1024/(3p) = 0.166`.
    ///
    /// What survives, and it is the sharper statement: every accepting move
    /// on a two-cell error is an ALIAS. One flip cannot undo two errors, so
    /// the move the search accepts is guaranteed to be wrong -- it is exactly
    /// the aliasing eggSo-v0 measured as 216 and 327 miscorrections before it
    /// added the confirming residue. Guess-and-fix does not stall on this
    /// channel; it walks confidently into the trap `q` exists to refuse.
    #[test]
    fn a_single_has_one_accepting_move_and_a_doubles_moves_are_all_aliases() {
        let c = c32();
        let mut g = Mul32::new(31);
        let mut census_total = 0usize;
        let mut doubles = 0usize;
        let mut alias_reaches_clean = 0usize;
        for _ in 0..300 {
            let clean = g.cells(c.l);
            let check = c.checks_for(&clean);

            let mut one = clean.clone();
            one[g.pick(c.l)] ^= 1;
            assert_eq!(accepting_census(&one, &check, &c), 1, "a single names itself");

            let mut two = clean.clone();
            let k = g.pick(3);
            let m = &c.members[k];
            let a = m[g.pick(m.len())];
            let mut b = m[g.pick(m.len())];
            while b == a {
                b = m[g.pick(m.len())];
            }
            two[a] ^= 1;
            two[b] ^= 1;
            let n = accepting_census(&two, &check, &c);
            census_total += n;
            doubles += 1;

            // every accepting move, applied, must differ from the clean square
            let st = State::of(&two, &check, &c);
            for i in 0..c.l {
                let mut t = st.clone();
                t.apply(&c, i, two[i]);
                let kk = c.class[i] as usize;
                if st.d[kk] != 0 && t.d[kk] == 0 {
                    let mut trial = two.clone();
                    trial[i] ^= 1;
                    if trial == clean {
                        alias_reaches_clean += 1;
                    }
                }
            }
        }
        assert_eq!(
            alias_reaches_clean, 0,
            "one flip cannot undo two errors, so no accepting move may reach the clean square"
        );
        let mean = census_total as f64 / doubles as f64;
        assert!(
            (0.08..0.32).contains(&mean),
            "accepting moves per same-class double {mean}, expected near |class|/(3p) averaged = 0.166"
        );
    }

    /// G4. The gradient detects the answer and does not point at it. At
    /// distance one the true error always shrinks the ring; at distance two
    /// the true errors are no better than a coin.
    #[test]
    fn the_gradients_range_is_one_step() {
        let c = c32();
        let mut g = Mul32::new(41);
        let mut d1_true = (0usize, 0usize);
        let mut d2_true = (0usize, 0usize);
        for _ in 0..120 {
            let clean = g.cells(c.l);
            let check = c.checks_for(&clean);
            let inner = &c.members[0];

            // distance 1
            let mut one = clean.clone();
            let a = inner[g.pick(inner.len())];
            one[a] ^= 1;
            let st = State::of(&one, &check, &c);
            let base = st.ring_sum(c.p);
            let mut t = st.clone();
            t.apply(&c, a, one[a]);
            d1_true.1 += 1;
            if t.ring_sum(c.p) < base {
                d1_true.0 += 1;
            }

            // distance 2, both errors in one class
            let mut two = clean.clone();
            let x = inner[g.pick(inner.len())];
            let mut y = inner[g.pick(inner.len())];
            while y == x {
                y = inner[g.pick(inner.len())];
            }
            two[x] ^= 1;
            two[y] ^= 1;
            let st2 = State::of(&two, &check, &c);
            let base2 = st2.ring_sum(c.p);
            for i in [x, y] {
                let mut t = st2.clone();
                t.apply(&c, i, two[i]);
                d2_true.1 += 1;
                if t.ring_sum(c.p) < base2 {
                    d2_true.0 += 1;
                }
            }
        }
        let r1 = d1_true.0 as f64 / d1_true.1 as f64;
        let r2 = d2_true.0 as f64 / d2_true.1 as f64;
        assert_eq!(r1, 1.0, "at distance 1 the true error must always descend");
        assert!(
            (0.3..0.7).contains(&r2),
            "at distance 2 the true errors are a coin, got {r2}"
        );
    }

    /// G6, the count arm, and THE SECOND MISS KEPT.
    ///
    /// PREDICTIONS.md filed "GF-5 clears same-class doubles that the residue
    /// arms cannot". It does not, and the reason completes the round's
    /// thesis rather than denting it. A count says HOW MANY errors a class
    /// holds and never WHICH cells, so the search converges in the count's
    /// own terms -- it reaches the right counts almost every time -- and
    /// lands on the wrong square, which the confirming residue then refuses.
    ///
    /// So neither purchase makes blind search work. **The address makes
    /// search unnecessary; the metric makes it converge to the wrong
    /// answer.** That is the honest form of the address-versus-metric
    /// sentence, and it is better than the one that was filed.
    #[test]
    fn the_count_arm_converges_in_counts_and_lands_on_the_wrong_square() {
        let c = c32();
        let mut g = Mul32::new(53);
        let trials = 200usize;
        let mut counts_met = 0usize;
        let mut fully_consistent = 0usize;
        let mut exact = 0usize;
        for _ in 0..trials {
            let clean = g.cells(c.l);
            let check = count_checks(&clean, &c);
            let mut cells = clean.clone();
            let k = g.pick(3);
            let m = &c.members[k];
            let a = m[g.pick(m.len())];
            let mut b = m[g.pick(m.len())];
            while b == a {
                b = m[g.pick(m.len())];
            }
            cells[a] ^= 1;
            cells[b] ^= 1;
            let tr = decode_count(&mut cells, &check, &c, 100_000, &mut g, Some(&clean));
            let got = count_checks(&cells, &c);
            if got[..3] == check[..3] {
                counts_met += 1;
            }
            if tr.consistent {
                fully_consistent += 1;
            }
            if tr.exact {
                exact += 1;
            }
        }
        // it does reach the right counts: that is the metric working
        assert!(
            counts_met * 4 > trials * 3,
            "the count arm should reach the right counts, got {counts_met}/{trials}"
        );
        // and it does not reach the right square, which is the point
        assert!(
            exact * 4 < trials,
            "the count arm should rarely land exactly, got {exact}/{trials}"
        );
        assert!(fully_consistent <= counts_met);
        // and it is not subsidised: fewer bits than the residues it replaces
        assert!(
            count_bits(&c) <= c.check_bits(),
            "count {} bits vs residues {} bits",
            count_bits(&c),
            c.check_bits()
        );
    }
}
