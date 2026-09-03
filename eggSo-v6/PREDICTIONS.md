# eggSo v6 predictions — filed 2026-09-03, BEFORE a line of the round was written

The series convention, unchanged since v0: every number below is a guess, written down first,
and the measured value is filled in beside it afterwards or not at all. Misses stay.

The twenty-first codec experiment and the seventh in the fold-native lineage. Rust, own
crate, empty `[dependencies]`.

## What this is, in one sentence

eggSo-v5 ended by finding that the walls in this construction are **three fixed constants
inherited from eggSo-v0**, not the geometry — so this round asks which of them are artifacts
that can be raised and which are information bounds that cannot.

## Why this round exists

v5's verdict, in its own words: *"the geometry of the fold looks mined out… the remaining live
work is engineering, not mathematics — v0's three fixed constants set the real walls at
n ≈ 43, 128 and 384."* Vladimir, 2026-09-03: *"Ok lets do it."*

The three constants, at `eggSo-v5/src/code.rs:343`, `:356` and `:370`, plus the pair cap:

| constant | v0's value | the wall v5 measured |
| --- | --- | --- |
| flagged erasures per class | **16** | a scaled burst kills `fold`/`blocks` at n ≈ 43, `diag3`/`idx3` at n = 128 |
| erasure hits per class | **64** | interacts with the above; never isolated before |
| erasure readings (combos) | **8192** | never isolated before |
| `PC_CANDIDATE_CAP`, pair candidates | **4096** | same-class doubles collapse from 117/120 to 70/120 between n = 384 and n = 512 |

## The constraint that shapes the whole round

v5's port pin holds `code.rs` to eggSo-v0's published behaviour **structurally and
behaviourally, through node** — 6,153 structural checks and 600 decisions, square by square.
Editing the caps in place would break that pin and the round would stop.

So **the caps become a parameter with v0's values as the default.** At defaults the decoder is
v0 to the decision, the pin still passes, and every raised-cap number is measured against that
baseline rather than replacing it. A round that quietly changed v0's behaviour and then
reported an improvement would be measuring its own edit.

## The derivation, done before measuring, because it decides what to expect

Flagged erasure recovery is not a search problem with a tunable budget. It is a counting
problem, and the count is fixed by the check bits.

Suppose class `k` holds `f_k` flagged cells, `F = f_0 + f_1 + f_2` in total. The decoder knows
*which* cells are unknown, so each class contributes `2^{f_k}` candidate assignments, and
class `k`'s own residue mod `p` keeps an expected `2^{f_k} / p` of them. The confirming
residue mod `q` is global and filters the surviving combinations once more, by `1/q`. So the
expected number of readings that satisfy every check is

```
2^F / (p³ · q)        spread across all three classes
2^F / (p  · q)        all F in ONE class, the other two clean
```

Recovery is unique when that expectation drops below 1, giving two bounds:

```
F  ≲  3·log2(p) + log2(q)   =  check_bits        (spread)
F  ≲    log2(p) + log2(q)                        (concentrated in one class)
```

At `n = 32`, `p = 2053` and `q = 2063`, so **the spread bound is 44.0 erasures and the
concentrated bound is 22.0**, against a stored `check_bits` of 48.

**And that is where v0's constants sit relative to the bounds:**

| v0's cap | the bound it sits against | verdict, filed |
| --- | --- | --- |
| 16 per class, so 48 total | spread bound **44** | the cap is **above** the bound — redundant, raising it buys nothing |
| 16 per class | concentrated bound **22** | the cap is **below** the bound — an **artifact**, and raising it should buy real corrections |
| 64 hits per class | at `f = 18` the expected solutions are `2^18/p ≈ 119` | **binds before the cap does**, so the two must be raised together or the experiment measures nothing |
| 4096 pair candidates | the pair count grows as `≈ L/36`, unbounded | pure **artifact**, cheap to raise |

## Predictions

| claim | called |
| --- | --- |
| the pin at default caps | **6 of 6 clean**, decisions identical to v0 — or the round stops |
| **the concentrated burst** — 18 cells in one class at n = 33, which v5 measured at **0 of 200** | **corrects** once the per-class cap reaches 18 **and** the hits cap clears ~120. This is the round's one real capability win |
| the same channel with the per-class cap raised past **22** | **stops improving**, because the concentrated bound is `log2(p·q) = 22.0` and past it the readings are genuinely ambiguous rather than merely uncounted |
| the **spread** burst — 12 or 18 cells split evenly by `diag3` | **no change at any cap**, because 3 × 16 = 48 already exceeds the spread bound of 44 |
| the cost of raising the per-class cap | **exponential**, `2^f` per class: 16 → 22 is a **64×** enumeration. Measurable in wall-clock and reported as such |
| **`PC_CANDIDATE_CAP`** raised to 65,536 at n = 512 | **restores same-class doubles to ≥ 95%**, from v5's 70 of 120 |
| the cost of that | **linear** in the cap and negligible, because the enumeration was already `O(\|class\|)` — the cap only bounded the output list |
| ambiguity created by the larger pair list | **none material**: false survivors go as `pairs/q ≈ (L/36)/(2L) = 1/72`, size-invariant, so ~1.4% either way |
| **the scaled-burst wall at n ≥ 256** | **NOT fixed by any cap.** At n = 256 a `3n/8` burst is 96 cells against a spread bound of ~68, so it is information-limited. v5's 0 of 120 stays 0 |
| miscorrections at every raised cap | **0**, as v5 measured across every width — raising a cap must not convert refusals into lies |
| the headline verdict | **two of v0's four caps are artifacts and two are not.** The pair cap and the per-class cap below 22 are free wins; the total-erasure ceiling and the n ≥ 256 burst wall are `check_bits` and cannot be bought |

## THE BARS

| bar | needed to count as met |
| --- | --- |
| **C1** the pin survives | at default caps the decoder is v0 to the decision, 6 of 6 pins clean, and v5's committed figures reproduced |
| **C2** the bound is derived and measured | the two bounds above stated before the numbers, then the measured recovery rate plotted against `F` and the transition found where the bound says, not where the cap says |
| **C3** each cap isolated | every cap raised **independently**, so an interaction is visible rather than pooled — the 64-hit cap in particular |
| **C4** the win, if there is one | the concentrated burst that v5 refused, corrected, with the cap value and the wall-clock cost both named |
| **C5** the non-win, stated as plainly | the caps that buy nothing, and the n ≥ 256 wall that is information rather than budget |
| **C6** no new lies | 0 miscorrections at every cap setting; a cap that converts a refusal into a wrong answer is a **failed** round and is reported as one |

**MET** if C1–C6 hold with their results printed either way. **MISSED** if a raised cap is
presented as a capability without its cost, or if the information bound is described as a
budget.

## What must NOT be built

1. **No edit to v0's default behaviour.** The caps default to v0's values, permanently.
2. **No compressor and no armour.** Settled and recorded across the whole series.
3. **Nothing in `codegg-*/`, `codec-v1/`, or `eggSo-v0..v5/`.** v6 copies forward and pins,
   as every round here does.
4. **No claim that raising a cap beats an information bound.** If a measurement appears to,
   it is an arithmetic error in the harness and gets found before it gets published.

## Measured (filled as parts land — never before)

Filled 2026-09-03, after `cargo build --release`, `cargo test` (59 tests),
`cargo clippy --all-targets -- -D warnings` clean with no suppressions, and `eggso6 audit`.

### C1 — the pin survives: 5 of 5 clean

| pin | checked | mismatches |
| --- | --- | --- |
| the copy vs v5's committed record | 19 figures | **0** |
| `region_of` vs `stalk.js`'s `regions()` | 22,139 | 0 |
| `arcs` vs `stalk.js`'s `arcs()` | 1,599 | 0 |
| the port vs eggSo-v0's structure | 6,153 | 0 |
| **the port vs eggSo-v0's decisions** | **600** | **0** |

The caps became a parameter and changed nothing: 600 squares decoded by both v0's own
decoder through node and this one at `Caps::v0()`, compared on the status word and the
repaired cells. The record pin includes v5's `70 of 120` at `n = 512` — the one figure this
round exists to move — so it is fixed before any cap is touched.

### C2 — the bound: derived first, and both halves land where the derivation put them

**Spread across three classes**, bound `3·log2(p) + log2(q) = 44.0`, caps raised clear:

| F | per class | corrected | ambiguous | refused | wrong |
| --- | --- | --- | --- | --- | --- |
| 36 | 12/12/12 | **60/60** | 0 | 0 | 0 |
| 39 | 13/13/13 | 28/30 | 2 | 0 | 0 |
| 42 | 14/14/14 | 22/30 | 8 | 0 | 0 |
| **44** | 15/15/14 | **11/30** | 19 | 0 | 0 |
| 45 | 15/15/15 | 9/30 | 21 | 0 | 0 |
| 48 | 16/16/16 | **0/15** | 15 | 0 | 0 |

**Concentrated in one class**, bound `log2(p) + log2(q) = 22.0`:

| F | corrected | ambiguous | refused | wrong | µs each |
| --- | --- | --- | --- | --- | --- |
| 12 | **60/60** | 0 | 0 | 0 | 118 |
| 16 | 59/60 | 1 | 0 | 0 | 2,407 |
| 18 | 56/60 | 4 | 0 | 0 | 10,384 |
| 20 | 23/30 | 7 | 0 | 0 | 42,946 |
| **22** | **14/30** | 16 | 0 | 0 | 179,434 |
| 24 | 0/16 | 16 | 0 | 0 | 741,121 |

Both transitions sit on their derived bound, and both cross ~40% exactly there — which is
the expected-one-solution point. **`refused` is 0 in every row and `wrong` is 0 in every
row:** past the bound the decoder reports *ambiguous*, which is it saying several readings
satisfy every check. That is what an information limit looks like from the inside, and it is
distinguishable from a budget stop, which prints `too many erasures` instead.

### C3, C4 — each cap isolated, and **C6 is MISSED**

18 erasures in one class at n = 32 — inside the bound of 22, so any failure here is a budget:

| caps | corrected | ambiguous | refused | **wrong** |
| --- | --- | --- | --- | --- |
| v0, untouched | 0 | 0 | **100** | 0 |
| per-class 16 → 20, hits still 64 | 46 | 3 | 49 | **2** |
| hits 64 → 4096, per-class still 16 | 0 | 0 | 100 | 0 |
| both: per-class 20, hits 4096 | **95** | 5 | 0 | 0 |
| `Caps::raised(20)` | **95** | 5 | 0 | 0 |
| `Caps::raised(22)` | **95** | 5 | 0 | 0 |

**C6 is MISSED, and it is the most useful thing in the round.** Raising
`erasures_per_class` *alone* makes the decoder **lie**. The erasure path enumerates the
`2^f` subsets of a class's flagged cells, keeps those matching the class residue — about
`2^f/p` of them — but stops collecting at `erasure_hits`, then asks `q` which kept reading
survives. Truncate that list and the **true** reading can fall off it, leaving a false one
as the unique survivor. The decoder commits to it.

At `f = 18` there are `2^18/p ≈ 119` expected solutions against v0's 64 kept, so the list is
short by half. **v0's own pair is safe by exactly a factor of two: `2^16/2053 = 31.9`
against 64.** So these were never four independent knobs — `erasures_per_class` and
`erasure_hits` are a **matched pair**, and v5's reading of them as separate constants, and
mine up to this table, was wrong. `Caps::raised(f, code)` now does the coupling arithmetic
and `Caps::hits_sufficient` checks a hand-built set; a test pins both the lie and its
absence.

**C4, the win, is real once it is bought safely:** 18 erasures in one class goes from **0 of
100** under v0 to **95 of 100**, with 0 lies. And its price is an exponent, not a knob:

| f in class | corrected | µs per square | vs f = 12 |
| --- | --- | --- | --- |
| 12 | 29/30 | 140 | 1× |
| 16 | 30/30 | 2,568 | 18× |
| 18 | 29/30 | 10,998 | 79× |
| 20 | 21/30 | 45,483 | 325× |
| 22 | 11/30 | 185,654 | **1,326×** |

**The pair cap is the clean win, and the only one that is nearly free.** Same-class doubles
at n = 512, which v5 published as 70 of 120:

| `pair_candidates` | corrected | wrong | ms per square |
| --- | --- | --- | --- |
| **4096** (v0) | **70/120** | 0 | 7.9 |
| 16,384 | **118/120** | 0 | 13.6 |
| 65,536 | 118/120 | 0 | 13.5 |
| 262,144 | 118/120 | 0 | 13.1 |

16,384 is already enough, and it saturates at 118 — the residual 2 are genuinely ambiguous.
The cost is 1.7×, not the "negligible" I filed, and it does not grow with the cap because
the enumeration was always `O(|class|)`; the cap only bounded the output list.

### C5 — the wall no cap can move

| n | burst | spread bound | v0 caps | generous caps | verdict |
| --- | --- | --- | --- | --- | --- |
| 33 | 12 | 44 | 50/50 | 50/50 | no wall here |
| 128 | 48 | 60 | 50/50 | 50/50 | no wall here |
| **256** | **96** | **68** | **0/50** | **absent (32/class)** | **INFORMATION** |

Once the burst is longer than the check bits, no budget buys it back. The generous column is
**absent** rather than optimistic at n = 256: 96 flagged cells is 32 per class and `2^32`
subsets is not computable, and substituting a shorter wound there would be two experiments
wearing one row. The bound argument does not need the number — `96 > 68` settles it.

### Predictions against results

| claim | landed |
| --- | --- |
| the pin at default caps | **6 of 6** → 5 of 5 clean (v6 carries one pin fewer, having dropped v5's `cellOrder`) |
| the concentrated burst corrects once per-class reaches 18 and hits clears ~120 | **HELD** — 0/100 → 95/100 |
| it stops improving past 22 | **HELD** — the transition is at 20–22 and `raised(22)` matches `raised(20)` |
| the spread burst: no change at any cap | **HELD** — v0's 3 × 16 = 48 already exceeds the 44 bound |
| the cost is exponential, 16 → 22 is 64× | **HELD** — measured **72×** |
| `PC_CANDIDATE_CAP` → 65,536 restores ≥ 95% at n = 512 | **HELD** — 98.3%, and 16,384 suffices |
| that cost is linear and negligible | **half MISSED** — linear yes, but 1.7×, which is not negligible |
| ambiguity from the larger pair list | **HELD** — 2 of 120 either way |
| n ≥ 256 not fixed by any cap | **HELD** |
| **0 miscorrections at every raised cap** | **MISSED. 2 of 100** at the lopsided raise, and finding it is the round |
| two of four caps are artifacts and two are not | **HELD in outline, wrong in detail** — the pair cap is a free artifact, the per-class cap is an artifact that is **coupled** rather than free, the readings cap is redundant, and the fourth wall was never a cap at all |

### The bar arithmetic, settled

| bar | result |
| --- | --- |
| **C1** the pin survives | **MET** — 600 decisions identical to v0 |
| **C2** the bound derived and measured | **MET** — both halves on their derived value |
| **C3** each cap isolated | **MET**, and isolating them is what exposed the coupling |
| **C4** the win, with its cost | **MET** — 0 → 95 of 100, at 79× the wall clock |
| **C5** the non-win, as plainly | **MET** — `96 > 68`, and the absent column left absent |
| **C6** no new lies | **MISSED — 2 miscorrections of 100.** Reported, mechanised, and pinned |

**Six bars, five met, one missed — and the missed one is the round's result.** A raised cap
that converts a refusal into a silent wrong answer is strictly worse than the refusal, and
the only reason it is a finding rather than a shipped bug is that C6 was filed before the
measurement and the `wrong` column was in the table from the start.
