//! code.rs -- eggSo-v0's codec, ported, with the class assignment as a
//! parameter from the start.
//!
//! CARRIED FROM eggSo-v5, which carried it from v4. v6's ONE change is that
//! the decoder's four caps are a PARAMETER instead of four literals, with
//! eggSo-v0's values as the default -- so at defaults this decoder is v0 to
//! the decision and `pin::v0_decisions` still proves it, while a raised cap
//! is measured against that baseline rather than replacing it. A round that
//! quietly changed v0's behaviour and then reported an improvement would be
//! measuring its own edit. See `Caps`.
//!
//! `in_bit` is `pub` here. Each round in this repo is a frozen record and its
//! own crate -- a path dependency would let v6's recorded numbers drift when
//! v5 changes -- so the shared modules are COPIED, and `pin::v5_figures` pins
//! the copy to v5's committed `measured-*.json` so a silent divergence is a
//! failed gate rather than a quiet one.
//!
//! v0 through v3 are JavaScript. This round is Rust, so v0's module cannot
//! be extended in place and must be reimplemented -- which is the better
//! arrangement anyway: `eggSo-v0/eggso.js` is not touched, the three sibling
//! JS rounds that require it keep working, and the port is held to a gate
//! stricter than an option flag would have been. See `audit.rs`: the port is
//! pinned to v0 structurally (class array, member lists, table keys and
//! their candidate lists, element for element in the same order) and
//! behaviourally (square by square, decision by decision, against v0's own
//! decoder run through node).
//!
//! What is borrowed, named where it lives:
//!   * the residue, the modulus search and the injectivity-by-enumeration
//!     discipline are codegg-v1's (`codegg-v1/codegg.js:55-102`), widened
//!     from one hardcoded partition to an arbitrary one.
//!   * the three region residues, the confirming residue and the erasure
//!     enumeration are eggSo-v0's (`eggSo-v0/eggso.js:49-245`).
//!   * the per-candidate confirm is codegg-v1's rule
//!     (`codegg.js:204-206, 223-231`), which v0 carries as its amendment of
//!     2026-09-02 (`eggSo-v0/eggso.js`, `regionSingles` / `regionPairs`).
//!     It is the DEFAULT here, because v0's amendment records that reading
//!     the confirm-after-the-plan numbers as the partition's cost was wrong.

use std::collections::HashMap;

use crate::fold;

/// The three outcomes the series never conflates, plus `Clean`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Clean,
    Corrected,
    Detected,
    Ambiguous,
}

#[derive(Clone, Debug)]
pub struct Repair {
    pub status: Status,
    pub fixed: usize,
    /// cells named by a syndrome with no search
    pub direct: usize,
    /// cells found by the in-class pair search
    pub searched: usize,
    pub note: &'static str,
}

impl Repair {
    fn of(status: Status, note: &'static str) -> Repair {
        Repair { status, fixed: 0, direct: 0, searched: 0, note }
    }
}

// ---- moduli: codegg-v1's property, unchanged -----------------------------

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

/// The whole scheme rests on this: the `2L` values `{+-2^k mod m}` must be
/// pairwise distinct, so a syndrome names one place and one sign. Verified by
/// enumeration, never taken on faith -- `codegg-v1/codegg.js:60-70`.
pub fn injective_for(m: u64, l: usize) -> bool {
    let mut seen = vec![false; m as usize];
    let mut pow = 1u64 % m;
    for _ in 0..l {
        let neg = (m - pow) % m;
        if pow == 0 || seen[pow as usize] || seen[neg as usize] || pow == neg {
            return false;
        }
        seen[pow as usize] = true;
        seen[neg as usize] = true;
        pow = (pow * 2) % m;
    }
    true
}

pub fn pick_modulus(l: usize, avoid: &[u64]) -> u64 {
    let mut m = 2 * l as u64 + 1;
    loop {
        if is_prime(m) && !avoid.contains(&m) && injective_for(m, l) {
            return m;
        }
        m += 2;
    }
}

/// `w[i] = 2^(L-1-i) mod m`, walked backwards as `syndromeTable` does.
pub fn weights(m: u64, l: usize) -> Vec<u64> {
    let mut w = vec![0u64; l];
    let mut pow = 1u64 % m;
    for i in (0..l).rev() {
        w[i] = pow;
        pow = (pow * 2) % m;
    }
    w
}

// ---- the code ------------------------------------------------------------

pub struct Code {
    pub n: usize,
    pub l: usize,
    pub p: u64,
    pub q: u64,
    pub confirm: bool,
    pub w: Vec<u64>,
    pub wq: Vec<u64>,
    /// cell -> class. The fold is one assignment among many.
    pub class: Vec<u8>,
    pub members: [Vec<usize>; 3],
    pub tables: [HashMap<u64, Vec<(usize, i8)>>; 3],
    pub seam: String,
}

/// The default assignment: the fold's own three regions, `stalk.js:118-126`.
pub fn fold_assign(r: usize, c: usize, _j: usize, n: usize) -> u8 {
    fold::region_of(r, c, n)
}

impl Code {
    /// Build a code over an arbitrary cell-to-class assignment. With
    /// `fold_assign` this is eggSo-v0's `makeCode(N, {confirm})` exactly.
    pub fn new(
        n: usize,
        confirm: bool,
        seam: &str,
        assign: impl Fn(usize, usize, usize, usize) -> u8,
    ) -> Code {
        let l = n * n;
        let p = pick_modulus(l, &[]);
        let q = if confirm { pick_modulus(l, &[p]) } else { 0 };
        let w = weights(p, l);
        let wq = if confirm { weights(q, l) } else { vec![0; l] };
        let mut class = vec![0u8; l];
        let mut members: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (j, slot) in class.iter_mut().enumerate() {
            let k = assign(j / n, j % n, j, n);
            *slot = k;
            members[k as usize].push(j);
        }
        let mut tables: [HashMap<u64, Vec<(usize, i8)>>; 3] =
            [HashMap::new(), HashMap::new(), HashMap::new()];
        for k in 0..3 {
            for &i in &members[k] {
                for d in [1i8, -1] {
                    let s = smod(d as i64 * w[i] as i64, p);
                    tables[k].entry(s).or_default().push((i, d));
                }
            }
        }
        Code { n, l, p, q, confirm, w, wq, class, members, tables, seam: seam.to_string() }
    }

    pub fn sizes(&self) -> [usize; 3] {
        [self.members[0].len(), self.members[1].len(), self.members[2].len()]
    }

    /// Bits of check per square. Depends only on `p`, `q` and `confirm`, so
    /// it is identical across every assignment -- which is the fairness
    /// statement Part 3 rests on.
    pub fn check_bits(&self) -> usize {
        let bits = |m: u64| (m as f64).log2().ceil() as usize;
        3 * bits(self.p) + if self.confirm { bits(self.q) } else { 0 }
    }

    pub fn overhead(&self) -> f64 {
        self.check_bits() as f64 / self.l as f64
    }

    /// Sum over each class of `cell * w[i]`, mod p. The three add back to the
    /// whole square's residue, which is the identity v0's suite pins.
    pub fn class_residues(&self, cells: &[i8]) -> [u64; 3] {
        let mut out = [0u64; 3];
        for (i, &v) in cells.iter().enumerate().take(self.l) {
            if v != 0 {
                let k = self.class[i] as usize;
                out[k] = smod(out[k] as i64 + v as i64 * self.w[i] as i64, self.p);
            }
        }
        out
    }

    pub fn q_residue(&self, cells: &[i8]) -> u64 {
        if !self.confirm {
            return 0;
        }
        let mut acc = 0i64;
        for (i, &v) in cells.iter().enumerate().take(self.l) {
            if v != 0 {
                acc = smod(acc + v as i64 * self.wq[i] as i64, self.q) as i64;
            }
        }
        acc as u64
    }

    /// `[I, F, O]` plus the confirming residue when it is carried.
    pub fn checks_for(&self, cells: &[i8]) -> Vec<u64> {
        let mut v = self.class_residues(cells).to_vec();
        if self.confirm {
            v.push(self.q_residue(cells));
        }
        v
    }

    pub fn verify(&self, cells: &[i8], check: &[u64]) -> bool {
        let r = self.class_residues(cells);
        r[0] == check[0]
            && r[1] == check[1]
            && r[2] == check[2]
            && (!self.confirm || self.q_residue(cells) == check[3])
    }
}

#[inline]
pub fn smod(x: i64, m: u64) -> u64 {
    let m = m as i64;
    (((x % m) + m) % m) as u64
}

/// The alphabet gate: a repaired cell has to land back on `{0, 1}`.
///
/// `pub` in this copy, per v5's carry-forward audit, so a caller that needs
/// the same gate uses THIS one rather than re-declaring it in a second place
/// where the two can drift apart.
#[inline]
pub fn in_bit(v: i8) -> bool {
    v == 0 || v == 1
}

/// How the confirming residue is applied. v0 as shipped assembled a whole
/// plan and only then asked `q`; codegg-v1 asks inside the search. v0's
/// amendment measured the difference at 281 against 972 on the same-class
/// channel, so `PerCandidate` is the default and `AfterPlan` exists to
/// reproduce v0's published figures.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Confirm {
    AfterPlan,
    PerCandidate,
}

#[derive(Clone, Debug, Default)]
pub struct Opts {
    pub erased: Vec<usize>,
    pub doubles: bool,
    pub confirm_mode: Option<Confirm>,
    /// v6's addition. `Caps::default()` is `Caps::v0()`, so every existing
    /// call site keeps v0's behaviour without saying so.
    pub caps: Caps,
}

impl Opts {
    pub fn new() -> Opts {
        Opts { erased: Vec::new(), doubles: true, confirm_mode: None, caps: Caps::v0() }
    }
    pub fn erased(list: &[usize]) -> Opts {
        Opts { erased: list.to_vec(), doubles: true, confirm_mode: None, caps: Caps::v0() }
    }
    pub fn after_plan() -> Opts {
        Opts {
            erased: Vec::new(),
            doubles: true,
            confirm_mode: Some(Confirm::AfterPlan),
            caps: Caps::v0(),
        }
    }
    /// The same options with the caps replaced, which is how every v6
    /// measurement is taken.
    pub fn with_caps(mut self, caps: Caps) -> Opts {
        self.caps = caps;
        self
    }
}

/// The decoder's four caps, and the whole subject of eggSo-v6.
///
/// **These are eggSo-v0's literals, and none of them scales with `L`.** v5
/// found that they, and not the geometry, set the construction's real walls.
/// They are a parameter here so each can be raised INDEPENDENTLY and priced,
/// while `Caps::v0()` -- the default everywhere -- keeps the decoder
/// bit-identical to v0.
///
/// What each one sits against, derived in `PREDICTIONS.md` before measuring.
/// With `f_k` flagged cells in class `k` and `F` in total, the expected
/// number of readings satisfying every check is `2^F / (p^3 q)` spread and
/// `2^F / (p q)` concentrated in one class, so recovery is unique only up to
///
/// ```text
/// F <~ 3*log2(p) + log2(q)   = check_bits    (spread)
/// F <~   log2(p) + log2(q)                   (one class)
/// ```
///
/// At `n = 32` that is **44.0 spread and 22.0 concentrated**. So v0's 16 per
/// class is ABOVE the spread bound once tripled (48 > 44) and BELOW the
/// concentrated one (16 < 22): redundant in one regime and an artifact in the
/// other. `erasure_hits` binds before either, because at `f = 18` there are
/// already about `2^18/p = 119` solutions to enumerate.
///
/// `pair_candidates` is the odd one out -- it bounds a search whose answer is
/// always present, so it is a pure artifact. The in-class pairs hitting one
/// syndrome grow as about `L/36`, crossing 4096 at `n` about 384, which is
/// exactly where v5 measured same-class doubles collapse from 117/120 to
/// 70/120.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Caps {
    /// flagged erasures the decoder will enumerate in one class
    pub erasures_per_class: usize,
    /// per-class residue solutions kept before giving up
    pub erasure_hits: usize,
    /// combinations across the three classes put to `q`
    pub erasure_readings: usize,
    /// in-class error PAIRS enumerated for one syndrome
    pub pair_candidates: usize,
    /// per-candidate combinations swept against `q`
    pub pc_combos: usize,
    /// **v7's fix, and the shippable half of the round.**
    ///
    /// When `erasure_hits` truncates a class's reading list, the decoder
    /// cannot know whether the true reading was among the ones it threw
    /// away -- so a unique survivor is NOT evidence of uniqueness. With this
    /// set, a truncated enumeration can never report `Corrected`; it reports
    /// `Ambiguous` instead.
    ///
    /// It is `false` in `Caps::v0()` and only there, so v0's published
    /// behaviour is untouched and `pin::v0_decisions` still proves it. It is
    /// `true` in `Caps::raised` and in anything else a caller builds through
    /// the constructors, because the calibration fix needs `p`, `f` and a
    /// margin to be right and this one needs nothing.
    pub refuse_on_truncation: bool,
}

impl Caps {
    /// eggSo-v0's own values, `eggSo-v0/eggso.js` and v4's port of it. The
    /// default, permanently: v6 does not change what v0 does.
    pub const fn v0() -> Caps {
        Caps {
            erasures_per_class: 16,
            erasure_hits: 64,
            erasure_readings: 8192,
            pair_candidates: 4096,
            pc_combos: 20000,
            // v0 did not have this and v0's behaviour is frozen. This is the
            // ONLY place it is false.
            refuse_on_truncation: false,
        }
    }

    /// v0's caps with the truncation guard on: what v0 would be if the fix
    /// had existed, and the honest baseline for what the guard COSTS.
    pub const fn v0_guarded() -> Caps {
        Caps { refuse_on_truncation: true, ..Caps::v0() }
    }
    /// The information ceiling on total erasures, `3*log2(p) + log2(q)`.
    pub fn spread_bound(code: &Code) -> f64 {
        3.0 * (code.p as f64).log2() + if code.confirm { (code.q as f64).log2() } else { 0.0 }
    }
    /// The same for erasures concentrated in a single class.
    pub fn concentrated_bound(code: &Code) -> f64 {
        (code.p as f64).log2() + if code.confirm { (code.q as f64).log2() } else { 0.0 }
    }
}

/// **The safety invariant, and it is the thing this round found.**
///
/// `erasures_per_class` and `erasure_hits` are NOT independent knobs. The
/// erasure path enumerates the `2^f` subsets of a class's flagged cells and
/// keeps those whose weights match the class residue -- about `2^f / p` of
/// them -- but it stops collecting at `erasure_hits` and then asks `q` which
/// of the kept readings survives. If the list was TRUNCATED, the true reading
/// may not be in it, and a false one that satisfies `q` is then the unique
/// survivor. The decoder commits to it. **It lies.**
///
/// v0's pair is safe by exactly a factor of two: at `f = 16` and `p = 2053`
/// there are `2^16/2053 = 31.9` expected solutions against 64 kept. Raising
/// `erasures_per_class` to 20 without touching `erasure_hits` puts 511
/// expected solutions against the same 64, and it was measured producing
/// **2 wrong answers in 100** where v0 refused all 100. So v6's own bar C6
/// is MISSED, and this is where it is recorded.
///
/// Every raise must therefore preserve
///
/// ```text
/// erasure_hits  >=  2^erasures_per_class / p  * SAFETY_MARGIN
/// ```
///
/// `raised` does that arithmetic; `hits_sufficient` checks a hand-built set.
impl Caps {
    /// How many readings a class of `f` flagged cells is expected to admit.
    pub fn expected_solutions(f: usize, p: u64) -> f64 {
        (f as f64).exp2() / p as f64
    }

    /// v0's own ratio of kept readings to expected ones, and the floor any
    /// raise has to clear.
    pub const SAFETY_MARGIN: f64 = 2.0;

    /// Whether this set can truncate the reading list at its own erasure cap.
    /// `false` means the decoder may commit to a wrong reading, which is
    /// strictly worse than refusing.
    pub fn hits_sufficient(&self, code: &Code) -> bool {
        let expected = Caps::expected_solutions(self.erasures_per_class, code.p);
        self.erasure_hits as f64 >= expected * Caps::SAFETY_MARGIN
    }

    /// Raise the erasure cap to `f` and carry every coupled budget with it.
    /// This is the only safe way to raise it, and the round says so.
    pub fn raised(f: usize, code: &Code) -> Caps {
        let need = (Caps::expected_solutions(f, code.p) * Caps::SAFETY_MARGIN).ceil();
        let hits = (need as usize).max(Caps::v0().erasure_hits);
        Caps {
            erasures_per_class: f,
            erasure_hits: hits,
            // three classes each offering `hits` readings
            erasure_readings: hits.saturating_mul(8).max(Caps::v0().erasure_readings),
            // belt and braces: the calibration above should mean the list is
            // never truncated, and the guard makes it safe if the estimate is
            // ever wrong. Calibration is the weaker fix; this is the strong one.
            refuse_on_truncation: true,
            ..Caps::v0()
        }
    }
}

impl Default for Caps {
    fn default() -> Caps {
        Caps::v0()
    }
}

/// Kept as free constants so v5's own tests and docs still refer to something
/// real; they are `Caps::v0()`'s fields by definition.
pub const PC_CANDIDATE_CAP: usize = Caps::v0().pair_candidates;
pub const PC_COMBO_CAP: usize = Caps::v0().pc_combos;

/// The width at which the pair enumeration starts truncating, from the
/// estimate `L/36` against `PC_CANDIDATE_CAP`. Approximate by construction --
/// it is a mean pair count, not a bound.
pub fn pair_cap_crossover_width() -> usize {
    let l = 36 * PC_CANDIDATE_CAP;
    (l as f64).sqrt() as usize
}

/// Repair one square in place. eggSo-v0's `repairSquare`, ported, including
/// the erasure caps at their v0 values (16 flagged per class, 64 hits, 8192
/// combinations) so a refusal is never mistaken for geometry.
pub fn repair(cells: &mut [i8], check: &[u64], code: &Code, opts: &Opts) -> Repair {
    let mode = opts.confirm_mode.unwrap_or(Confirm::PerCandidate);
    let p = code.p;

    // ---- flagged erasures, per class ------------------------------------
    let mut flagged: Vec<usize> = opts.erased.clone();
    for (i, &v) in cells.iter().enumerate() {
        if v == -1 && !flagged.contains(&i) {
            flagged.push(i);
        }
    }
    if !flagged.is_empty() {
        flagged.sort_unstable();
        flagged.dedup();
        let mut by_class: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for &i in &flagged {
            by_class[code.class[i] as usize].push(i);
        }
        let mut base = cells.to_vec();
        for &i in &flagged {
            base[i] = 0;
        }
        let base_res = code.class_residues(&base);
        let base_q = code.q_residue(&base);

        let mut truncated = false;
        let mut hits_per: Vec<Vec<u32>> = Vec::with_capacity(3);
        for k in 0..3 {
            let f = &by_class[k];
            if f.is_empty() {
                if base_res[k] != check[k] {
                    return Repair::of(Status::Detected, "erasures+error");
                }
                hits_per.push(vec![0]);
                continue;
            }
            if f.len() > opts.caps.erasures_per_class {
                return Repair::of(Status::Detected, "too many erasures");
            }
            let mut hits: Vec<u32> = Vec::new();
            for a in 0u32..(1u32 << f.len()) {
                let mut r = base_res[k];
                for (j, &i) in f.iter().enumerate() {
                    if a & (1 << j) != 0 {
                        r = (r + code.w[i]) % p;
                    }
                }
                if r == check[k] {
                    hits.push(a);
                    if hits.len() > opts.caps.erasure_hits {
                        // THE TRUNCATION. Everything after this point is
                        // reasoning over an incomplete list, and v7's guard
                        // exists because v6 measured what that costs.
                        truncated = true;
                        break;
                    }
                }
            }
            if hits.is_empty() {
                return Repair::of(Status::Detected, "erasures");
            }
            hits_per.push(hits);
        }
        let combos = hits_per[0].len() * hits_per[1].len() * hits_per[2].len();
        if combos > 1 && !code.confirm {
            return Repair::of(Status::Ambiguous, "erasures");
        }
        if combos > opts.caps.erasure_readings {
            return Repair::of(Status::Detected, "erasures: too many readings");
        }
        let mut survivor: Option<[u32; 3]> = None;
        let mut count = 0usize;
        'outer: for &a0 in &hits_per[0] {
            for &a1 in &hits_per[1] {
                for &a2 in &hits_per[2] {
                    if code.confirm {
                        let mut rq = base_q;
                        for (k, a) in [a0, a1, a2].iter().enumerate() {
                            for (j, &i) in by_class[k].iter().enumerate() {
                                if a & (1 << j) != 0 {
                                    rq = (rq + code.wq[i]) % code.q;
                                }
                            }
                        }
                        if rq != check[3] {
                            continue;
                        }
                    }
                    count += 1;
                    survivor = Some([a0, a1, a2]);
                    if count > 1 {
                        break 'outer;
                    }
                }
            }
        }
        if count != 1 {
            let st = if count > 0 { Status::Ambiguous } else { Status::Detected };
            return Repair::of(st, "erasures");
        }
        // v7's guard. One survivor of a TRUNCATED list is not a unique
        // reading -- the true one may be among those never enumerated -- so
        // committing to it is exactly the silent miscorrection v6 measured.
        // Refusing here is strictly safer than answering.
        if truncated && opts.caps.refuse_on_truncation {
            return Repair::of(Status::Ambiguous, "truncated list");
        }
        let s = survivor.unwrap();
        for k in 0..3 {
            for (j, &i) in by_class[k].iter().enumerate() {
                cells[i] = ((s[k] >> j) & 1) as i8;
            }
        }
        let n = flagged.len();
        return Repair {
            status: Status::Corrected,
            fixed: n,
            direct: n,
            searched: 0,
            note: "erasures",
        };
    }

    // ---- errors ---------------------------------------------------------
    let cur = code.class_residues(cells);
    let delta: Vec<u64> = (0..3).map(|k| smod(cur[k] as i64 - check[k] as i64, p)).collect();
    let dq = if code.confirm {
        smod(code.q_residue(cells) as i64 - check[3] as i64, code.q)
    } else {
        0
    };
    let hurt: Vec<usize> = (0..3).filter(|&k| delta[k] != 0).collect();
    if hurt.is_empty() {
        if code.confirm && dq != 0 {
            return Repair::of(Status::Detected, "confirm only");
        }
        return Repair::of(Status::Clean, "clean");
    }

    match mode {
        Confirm::PerCandidate if code.confirm => {
            per_candidate(cells, check, code, opts, &delta, &hurt)
        }
        _ => after_plan(cells, check, code, opts, &delta, &hurt, dq),
    }
}

/// v0 as shipped: one candidate per hurt class, then the whole plan is put to
/// `q`. Kept so v0's published figures can be reproduced.
fn after_plan(
    cells: &mut [i8],
    _check: &[u64],
    code: &Code,
    opts: &Opts,
    delta: &[u64],
    hurt: &[usize],
    dq: u64,
) -> Repair {
    let p = code.p;
    let mut plan: Vec<(usize, i8)> = Vec::new();
    let mut direct = 0usize;
    let mut searched = 0usize;
    for &k in hurt {
        let s = delta[k];
        let singles = class_singles(cells, code, k, s);
        if singles.len() == 1 {
            plan.push(singles[0]);
            direct += 1;
            continue;
        }
        if singles.len() > 1 {
            return Repair::of(Status::Ambiguous, "single");
        }
        if !opts.doubles {
            return Repair::of(Status::Detected, "doubles off");
        }
        let pairs = class_pairs(cells, code, k, s, 2);
        if pairs.len() == 1 {
            plan.extend_from_slice(&pairs[0]);
            searched += 2;
            continue;
        }
        let st = if pairs.is_empty() { Status::Detected } else { Status::Ambiguous };
        return Repair::of(st, if pairs.is_empty() { "unrepaired" } else { "double" });
    }
    if code.confirm {
        let mut rq = dq;
        for &(i, d) in &plan {
            rq = smod(rq as i64 - d as i64 * code.wq[i] as i64, code.q);
        }
        if rq != 0 {
            return Repair::of(Status::Detected, "failed confirm");
        }
    }
    let _ = p;
    for &(i, d) in &plan {
        cells[i] -= d;
    }
    Repair {
        status: Status::Corrected,
        fixed: plan.len(),
        direct,
        searched,
        note: if searched > 0 { "double" } else { "single" },
    }
}

/// The amendment: every candidate each hurt class admits, combined, with `q`
/// asked of each combination. Two stages, in codegg-v1's order -- singles
/// first, and only if no all-singles reading satisfies `q`, one class holds a
/// pair. The stage-2 fall-through is the part v0 never had.
fn per_candidate(
    cells: &mut [i8],
    check: &[u64],
    code: &Code,
    opts: &Opts,
    delta: &[u64],
    hurt: &[usize],
) -> Repair {
    let rq0 = code.q_residue(cells);
    let mut survivors: Vec<Vec<Vec<(usize, i8)>>> = Vec::new();
    let mut room = true;

    let sweep = |lists: &[Vec<Vec<(usize, i8)>>], survivors: &mut Vec<Vec<Vec<(usize, i8)>>>| -> bool {
        if lists.iter().any(|l| l.is_empty()) {
            return true;
        }
        let combos: usize = lists.iter().map(|l| l.len()).product();
        if combos > opts.caps.pc_combos {
            return false;
        }
        let mut idx = vec![0usize; lists.len()];
        for _ in 0..combos {
            if survivors.len() >= 2 {
                break;
            }
            let mut rq = rq0;
            for (j, l) in lists.iter().enumerate() {
                for &(i, d) in &l[idx[j]] {
                    rq = smod(rq as i64 - d as i64 * code.wq[i] as i64, code.q);
                }
            }
            if rq == check[3] {
                survivors.push(lists.iter().enumerate().map(|(j, l)| l[idx[j]].clone()).collect());
            }
            for j in 0..lists.len() {
                idx[j] += 1;
                if idx[j] < lists[j].len() {
                    break;
                }
                idx[j] = 0;
            }
        }
        true
    };

    let singles: Vec<Vec<Vec<(usize, i8)>>> = hurt
        .iter()
        .map(|&k| class_singles(cells, code, k, delta[k]).into_iter().map(|c| vec![c]).collect())
        .collect();
    room &= sweep(&singles, &mut survivors);

    if survivors.is_empty() && room && opts.doubles {
        for x in 0..hurt.len() {
            if survivors.len() >= 2 {
                break;
            }
            let mut lists = singles.clone();
            lists[x] = class_pairs(cells, code, hurt[x], delta[hurt[x]], opts.caps.pair_candidates);
            if !sweep(&lists, &mut survivors) {
                room = false;
                break;
            }
        }
    }
    if !room {
        return Repair::of(Status::Detected, "too many readings");
    }
    if survivors.len() != 1 {
        let st = if survivors.is_empty() { Status::Detected } else { Status::Ambiguous };
        return Repair::of(st, if survivors.is_empty() { "unrepaired" } else { "per-candidate" });
    }
    let chosen = &survivors[0];
    let mut direct = 0usize;
    let mut searched = 0usize;
    for part in chosen {
        if part.len() == 1 {
            direct += 1;
        } else {
            searched += part.len();
        }
    }
    let mut fixed = 0usize;
    for part in chosen {
        for &(i, d) in part {
            cells[i] -= d;
            fixed += 1;
        }
    }
    Repair {
        status: Status::Corrected,
        fixed,
        direct,
        searched,
        note: if searched > 0 { "double" } else { "single" },
    }
}

/// Alphabet-valid single-cell readings of a class syndrome.
pub fn class_singles(cells: &[i8], code: &Code, k: usize, s: u64) -> Vec<(usize, i8)> {
    match code.tables[k].get(&s) {
        None => Vec::new(),
        Some(v) => v.iter().copied().filter(|&(i, d)| in_bit(cells[i] - d)).collect(),
    }
}

/// Peel every first error among the class's cells and ask whether the
/// remainder is a valid second. v0's search, confined to one class.
pub fn class_pairs(
    cells: &[i8],
    code: &Code,
    k: usize,
    s: u64,
    cap: usize,
) -> Vec<Vec<(usize, i8)>> {
    let p = code.p;
    let mut out: Vec<Vec<(usize, i8)>> = Vec::new();
    let mut seen: std::collections::HashSet<(usize, i8, usize, i8)> =
        std::collections::HashSet::new();
    for &i1 in &code.members[k] {
        for d1 in [1i8, -1] {
            if !in_bit(cells[i1] - d1) {
                continue;
            }
            let rest = smod(s as i64 - d1 as i64 * code.w[i1] as i64, p);
            if rest == 0 {
                continue;
            }
            if let Some(cands) = code.tables[k].get(&rest) {
                for &(i2, d2) in cands {
                    if i2 == i1 || !in_bit(cells[i2] - d2) {
                        continue;
                    }
                    let key = if i2 < i1 { (i2, d2, i1, d1) } else { (i1, d1, i2, d2) };
                    if !seen.insert(key) {
                        continue;
                    }
                    out.push(vec![(i1, d1), (i2, d2)]);
                    if out.len() >= cap {
                        return out;
                    }
                }
            }
        }
    }
    out
}

// ---- bytes into squares, codegg-v1's row-major layout --------------------

pub fn to_cells(bytes: &[u8], l: usize) -> Vec<Vec<i8>> {
    let total = ((bytes.len() * 8) as f64 / l as f64).ceil() as usize;
    let total = total.max(1);
    let mut out = Vec::with_capacity(total);
    for s in 0..total {
        let mut cells = vec![0i8; l];
        for (j, slot) in cells.iter_mut().enumerate() {
            let bit = s * l + j;
            let b = bit >> 3;
            *slot = if b < bytes.len() {
                ((bytes[b] >> (7 - (bit & 7))) & 1) as i8
            } else {
                0
            };
        }
        out.push(cells);
    }
    out
}

pub fn to_bytes(squares: &[Vec<i8>], l: usize, byte_len: usize) -> Vec<u8> {
    let mut out = vec![0u8; byte_len];
    for bit in 0..byte_len * 8 {
        let cells = &squares[bit / l];
        if cells[bit % l] == 1 {
            out[bit >> 3] |= 1 << (7 - (bit & 7));
        }
    }
    out
}

// ---- the series' PRNG, ported exactly -----------------------------------

/// `mul32`, the precision-safe generator the lineage settled on after the LCG
/// incident. `Math.imul` is a wrapping 32-bit multiply, so this walks the
/// identical stream to the JavaScript rounds when seeded identically.
pub struct Mul32(u32);

impl Mul32 {
    pub fn new(seed: u32) -> Mul32 {
        Mul32(seed)
    }
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(0x6D2B_79F5);
        let a = self.0;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        t ^ (t >> 14)
    }
    pub fn pick(&mut self, n: usize) -> usize {
        (self.next() as usize) % n
    }
    pub fn cells(&mut self, l: usize) -> Vec<i8> {
        (0..l).map(|_| (self.next() & 1) as i8).collect()
    }
    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| (self.next() & 0xff) as u8).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fold_code(confirm: bool) -> Code {
        Code::new(32, confirm, "fold", fold_assign)
    }

    /// The moduli v0 recorded: p = 2053, q = 2063 at L = 1024, and the
    /// injectivity re-verified by enumeration rather than assumed.
    #[test]
    fn the_moduli_are_v0s() {
        let c = fold_code(true);
        assert_eq!(c.p, 2053);
        assert_eq!(c.q, 2063);
        assert!(injective_for(c.p, c.l));
        assert!(injective_for(c.q, c.l));
    }

    /// The fold's classes at N = 32, and the overhead v0 published.
    #[test]
    fn the_fold_splits_496_32_496_at_4_69_percent() {
        let c = fold_code(true);
        assert_eq!(c.sizes(), [496, 32, 496]);
        assert_eq!(c.check_bits(), 48);
        assert!((c.overhead() - 0.046875).abs() < 1e-12);
    }

    /// `sizes` depends only on p, q and confirm, so it is identical across
    /// every assignment. This is the fairness statement Part 3 rests on.
    #[test]
    fn overhead_is_identical_across_assignments() {
        let a = fold_code(true);
        let b = Code::new(32, true, "diag3", |r, c, _j, _n| ((r + c) % 3) as u8);
        let d = Code::new(32, true, "idx3", |_r, _c, j, _n| (j % 3) as u8);
        assert_eq!(a.check_bits(), b.check_bits());
        assert_eq!(a.check_bits(), d.check_bits());
        assert_eq!(a.p, b.p);
        assert_eq!(a.q, d.q);
    }

    /// `I + F + O = V (mod p)`, the fold's own identity, which holds for any
    /// partition because a partition is what makes the sum a sum.
    #[test]
    fn the_classes_sum_back_to_the_value() {
        let c = fold_code(true);
        let mut g = Mul32::new(20260903);
        for _ in 0..200 {
            let cells = g.cells(c.l);
            let r = c.class_residues(&cells);
            let mut whole = 0i64;
            for (i, &v) in cells.iter().enumerate() {
                whole = smod(whole + v as i64 * c.w[i] as i64, c.p) as i64;
            }
            assert_eq!(smod(r[0] as i64 + r[1] as i64 + r[2] as i64, c.p), whole as u64);
        }
    }

    #[test]
    fn round_trip_is_exact() {
        let c = fold_code(true);
        let mut g = Mul32::new(7);
        for n in [0usize, 1, 7, 128, 129, 1000, 4097] {
            let src = g.bytes(n);
            let squares = to_cells(&src, c.l);
            let back = to_bytes(&squares, c.l, src.len());
            assert_eq!(back, src, "round trip at {n} bytes");
            for sq in &squares {
                let chk = c.checks_for(sq);
                assert!(c.verify(sq, &chk));
            }
        }
    }

    /// Singles are named by their own class syndrome, with no search.
    #[test]
    fn singles_are_direct() {
        let c = fold_code(true);
        let mut g = Mul32::new(11);
        for _ in 0..2000 {
            let cells = g.cells(c.l);
            let chk = c.checks_for(&cells);
            let mut hurt = cells.clone();
            let i = g.pick(c.l);
            hurt[i] ^= 1;
            let r = repair(&mut hurt, &chk, &c, &Opts::new());
            assert_eq!(r.status, Status::Corrected);
            assert_eq!(r.direct, 1);
            assert_eq!(r.searched, 0);
            assert_eq!(hurt, cells);
        }
    }

    /// The amendment, ported: the same-class pair channel is decided by where
    /// the confirm sits. v0 recorded 281 against 972 on 1000 trials.
    #[test]
    fn the_confirm_placement_is_the_whole_difference() {
        let c = fold_code(true);
        let run = |mode: Option<Confirm>| {
            let mut g = Mul32::new(20260902);
            let mut ok = 0usize;
            let mut wrong = 0usize;
            for _ in 0..400 {
                let cells = g.cells(c.l);
                let chk = c.checks_for(&cells);
                let mut hurt = cells.clone();
                let k = g.pick(3);
                let m = &c.members[k];
                let a = m[g.pick(m.len())];
                let mut b = m[g.pick(m.len())];
                while b == a {
                    b = m[g.pick(m.len())];
                }
                hurt[a] ^= 1;
                hurt[b] ^= 1;
                let opts =
                    Opts { erased: Vec::new(), doubles: true, confirm_mode: mode, caps: Caps::v0() };
                let r = repair(&mut hurt, &chk, &c, &opts);
                if r.status == Status::Corrected {
                    if hurt == cells {
                        ok += 1;
                    } else {
                        wrong += 1;
                    }
                }
            }
            (ok, wrong)
        };
        let (shipped, w1) = run(Some(Confirm::AfterPlan));
        let (amended, w2) = run(Some(Confirm::PerCandidate));
        assert_eq!(w1, 0, "after-plan miscorrected {w1}");
        assert_eq!(w2, 0, "per-candidate miscorrected {w2}");
        assert!(
            amended > shipped * 2,
            "per-candidate {amended} should dwarf after-plan {shipped}"
        );
    }

    /// The pair cap is a SIZE LIMIT, and the round now says so rather than
    /// discovering it at some future width.
    ///
    /// `PC_CANDIDATE_CAP` is a fixed 4096 while the number of in-class pairs
    /// hitting a syndrome grows as about `L/36`, so past `n` about 384 the
    /// enumeration truncates before reaching the true pair. The thing that
    /// makes this tolerable rather than dangerous is the second assert: the
    /// lost corrections become REFUSALS, never miscorrections.
    #[test]
    fn the_pair_cap_is_a_size_limit_and_it_fails_safe() {
        assert!(
            (350..=400).contains(&pair_cap_crossover_width()),
            "the crossover estimate moved to {}",
            pair_cap_crossover_width()
        );

        let run = |n: usize, trials: usize| {
            let c = Code::new(n, true, "diag3", |r, c, _j, _n| ((r + c) % 3) as u8);
            let mut g = Mul32::new(7);
            let (mut ok, mut wrong) = (0usize, 0usize);
            for _ in 0..trials {
                let clean = g.cells(c.l);
                let check = c.checks_for(&clean);
                let mut h = clean.clone();
                let k = g.pick(3);
                let m = &c.members[k];
                let a = m[g.pick(m.len())];
                let mut b = m[g.pick(m.len())];
                while b == a {
                    b = m[g.pick(m.len())];
                }
                h[a] ^= 1;
                h[b] ^= 1;
                let r = repair(&mut h, &check, &c, &Opts::new());
                if r.status == Status::Corrected {
                    if h == clean {
                        ok += 1;
                    } else {
                        wrong += 1;
                    }
                }
            }
            (ok, wrong)
        };

        // under the cap: the channel works, as it does at n = 32
        let (under, w_under) = run(128, 40);
        assert_eq!(under, 40, "n=128 should still clear same-class doubles");
        assert_eq!(w_under, 0);

        // over the cap: corrections are lost
        let (over, w_over) = run(512, 40);
        assert!(over < 36, "n=512 corrected {over}/40 -- the cap is not biting");

        // AND THE POINT: what is lost is refused, not miscorrected
        assert_eq!(w_over, 0, "n=512 miscorrected {w_over} -- the cap does NOT fail safe");
    }

    /// **C6, and it is a filed MISS reported as one.** Raising
    /// `erasures_per_class` without `erasure_hits` makes the decoder LIE:
    /// the reading list is truncated, the true reading falls off it, and a
    /// false one satisfies `q` alone and is committed. Raising the pair
    /// together does not.
    ///
    /// This is the round's most useful result and it is pinned here so no
    /// later round can raise one cap and not the other by accident.
    #[test]
    fn raising_the_erasure_cap_alone_makes_the_decoder_lie() {
        let c = Code::new(32, true, "diag3", |r, c, _j, _n| ((r + c) % 3) as u8);

        // v0's own pair is safe, by a factor of two
        assert!(Caps::v0().hits_sufficient(&c));
        let ev = Caps::expected_solutions(16, c.p);
        assert!((ev - 31.9).abs() < 0.5, "expected solutions at f=16: {ev}");

        // raising the erasure cap alone is NOT safe, and the arithmetic says
        // so before the measurement does
        let lopsided = Caps { erasures_per_class: 20, ..Caps::v0() };
        assert!(!lopsided.hits_sufficient(&c));
        assert!(Caps::expected_solutions(20, c.p) > Caps::v0().erasure_hits as f64);

        // and the coupled raise is safe
        let safe = Caps::raised(20, &c);
        assert!(safe.hits_sufficient(&c));
        assert_eq!(safe.erasures_per_class, 20);
        assert!(safe.erasure_hits >= 1022, "hits {} is too few", safe.erasure_hits);

        // now the measurement, which is what makes it a finding rather than
        // an argument: 18 erasures in one class, inside the bound of 22.
        let run = |caps: Caps, trials: usize| {
            let mut g = Mul32::new(777);
            let (mut ok, mut wrong) = (0usize, 0usize);
            for _ in 0..trials {
                let clean = g.cells(c.l);
                let check = c.checks_for(&clean);
                let mut h = clean.clone();
                let m = &c.members[0];
                let mut picked: Vec<usize> = Vec::new();
                while picked.len() < 18 {
                    let i = m[g.pick(m.len())];
                    if !picked.contains(&i) {
                        picked.push(i);
                    }
                }
                for &i in &picked {
                    h[i] = -1;
                }
                let r = repair(&mut h, &check, &c, &Opts::erased(&picked).with_caps(caps));
                if r.status == Status::Corrected {
                    if h == clean {
                        ok += 1;
                    } else {
                        wrong += 1;
                    }
                }
            }
            (ok, wrong)
        };

        let (v0_ok, v0_wrong) = run(Caps::v0(), 60);
        assert_eq!(v0_ok, 0, "v0 should refuse all of these");
        assert_eq!(v0_wrong, 0, "and refuse them safely");

        let (bad_ok, bad_wrong) = run(lopsided, 60);
        assert!(bad_wrong > 0, "the lopsided raise was expected to lie, got {bad_wrong}");
        assert!(bad_ok > 0, "and to correct some too, got {bad_ok}");

        let (good_ok, good_wrong) = run(Caps::raised(20, &c), 60);
        assert_eq!(good_wrong, 0, "the coupled raise lied {good_wrong} times");
        assert!(good_ok > 50, "the coupled raise only managed {good_ok}/60");
    }

    /// A flagged erasure per class is recovered, which is the channel a blind
    /// guess can also win because one equation determines one unknown.
    #[test]
    fn one_erasure_per_class_is_recovered() {
        let c = fold_code(true);
        let mut g = Mul32::new(13);
        for _ in 0..300 {
            let cells = g.cells(c.l);
            let chk = c.checks_for(&cells);
            let mut hurt = cells.clone();
            let f: Vec<usize> =
                (0..3).map(|k| c.members[k][g.pick(c.members[k].len())]).collect();
            for &i in &f {
                hurt[i] = 0;
            }
            let r = repair(&mut hurt, &chk, &c, &Opts::erased(&f));
            assert_eq!(r.status, Status::Corrected);
            assert_eq!(hurt, cells);
        }
    }
}
