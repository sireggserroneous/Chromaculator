# eggSo v7 predictions — filed 2026-09-03, BEFORE a line of the round was written

The series convention, unchanged since v0: every number below is a guess, written down first,
and the measured value is filled in beside it afterwards or not at all. Misses stay.

The twenty-second codec experiment and the eighth in the fold-native lineage. Rust, own
crate, empty `[dependencies]`.

## What this is, in one sentence

The two things v6 left that were worth doing, and nothing else: **finish the characterisation
of the burst floor for `L` not divisible by 3**, settle the **three cases v6 left
inconclusive**, and **ship the one safety fix** v6's failed bar earned.

## Why this round exists, and why it is deliberately small

v6's verdict was that the construction's ceiling is `check_bits` and that every road onward
leads to Reed–Solomon, which this repo has already reached from two other doors. So there is
no engineering round left. Vladimir picked the two items that survive that verdict:

> *"Small and real: characterise `3∤L`, and settle the three inconclusive cases `(30,8)`,
> `(33,8)`, `(33,11)`. Bounded, genuine mathematics."*
>
> *"Shippable now: the safety invariant… the general lesson being that truncating a candidate
> list and then filtering by a second check converts detection into miscorrection."*

Two parts. No third.

## Measured before this file existed — ground, not predictions

Computed 2026-09-03 while scoping this round, so it cannot honestly be filed as a guess. The
suite re-derives each one rather than quoting it.

**The derivation first.** A tape run of `L` cells crossing a row boundary is two arithmetic
progressions with a phase slip: `m` cells before the boundary, `L − m` after. For a linear
partition with `b ≢ 0`, an AP of length `t` over `Z/3` puts at most `⌈t/3⌉` cells in one
class, so the run's worst class is at most `⌈m/3⌉ + ⌈(L−m)/3⌉`. Then:

| `L mod 3` | the arithmetic | consequence |
| --- | --- | --- |
| `L = 3t` | `m ≢ 0` gives `t + 1` | the slip costs 1, and v5's periodicity lemma applies |
| `L = 3t+1` | every split gives at most `t+1` = the floor | **the slip is always absorbed** |
| `L = 3t+2` | `m = 1` gives `t+2` > the floor | **conditional**, and this is where the open cases live |

| claim | measured |
| --- | --- |
| `L ≡ 1 (mod 3)`: the arms `(1,2)` and `(2,1)` reach the floor on **all four** geometries at every `n` | **confirmed**, 0 failures over `n = 15..40` and `L = 7, 10, 13, 16`. The tape condition is **vacuous** at this residue |
| `L ≡ 2 (mod 3)`: which `n` admit a linear arm | **exactly the `n` with `n ≢ 0 (mod 3)`** — measured identical for `L = 8, 11, 14, 17` over `n = 15..36`, with every multiple of 3 absent and every other `n` present |
| what that completes | v5 proved the `L ≡ 0` case: only `n ≡ 2 (mod 3)`. With the two rows above, **the linear family is now characterised for every `L`** |

**So the complete linear characterisation, which the suite must re-derive:**

```
a linear partition reaches ceil(L/3) on all four geometries iff
    L = 0 (mod 3):  n = 2 (mod 3)
    L = 1 (mod 3):  every n
    L = 2 (mod 3):  n != 0 (mod 3)
```

And that places the three open cases exactly: `(30,8)`, `(33,8)`, `(33,11)` all have
`n ≡ 0 (mod 3)` with `L ≡ 2 (mod 3)` — the one cell of the table where **no linear arm
exists**. So the open question is precisely whether a **nonlinear** partition reaches the
floor there.

## Part A — the three cases

`(15,8)`, `(15,11)` and `(30,11)` sit in that same cell and v6's enumeration **reached** all
three with nonlinear partitions. `(30,8)`, `(33,8)` and `(33,11)` exhausted the 200,000,000
node cap.

**The method change, and why it is still rigorous.** Settling a case *positively* needs one
partition, not a completed enumeration — a construction is its own proof. So this round adds
a **randomised-restart** search: the same pruned depth-first walk, but with the value order
shuffled per restart and a node budget per restart. It is **not** a complete method and
cannot return "impossible"; it can only return **REACHED with an exhibited partition**, which
is then re-verified from scratch by `worst_all`. Every case it fails to settle stays
**INCONCLUSIVE** and is printed as such.

Two cheap exact reductions go in first:

- **row windows are redundant.** When `L ≤ n`, a row window of `L` consecutive cells *is* `L`
  consecutive tape indices, so every row constraint is already a tape constraint. Dropping
  them is exact and shrinks the constraint set by about a quarter.
- **canonical labelling** stays as v5 built it: first occurrences run `0,1,2`.

## Part B — the safety fix

v6's C6 failed: raising `erasures_per_class` without `erasure_hits` produced **2 silent wrong
answers in 100** where v0 refused all 100. v6's answer was `Caps::raised`, which computes the
coupled budget. That is a *calibration*, and calibration is the weaker fix — it needs `p`,
`f` and a margin to be right.

**The strong fix is to make truncation unforgeable.** If the reading list was truncated, the
decoder cannot know whether the true reading was among the discarded ones, so a unique
survivor is **not** evidence of uniqueness. So: thread a `truncated` flag out of the
enumeration and refuse to report `Corrected` when it is set. That is safe at **any** cap
setting, needs no arithmetic, and is the generic form of the lesson.

**It must not change v0.** So the guard is a `Caps` field, `refuse_on_truncation`, which is
**`false` in `Caps::v0()`** — preserving the 600-decision pin exactly — and **`true`
everywhere else**, including `Caps::raised`.

## Predictions

| claim | called |
| --- | --- |
| the pin at default caps | **5 of 5 clean**, 600 decisions still identical to v0, or the round stops |
| the complete linear characterisation, re-derived over a wide sweep | **0 violations**, all three residues |
| **the three open cases** | **all three are SOLVABLE**, and the randomised restart finds at least two of them. Reasoning: `(15,8)`, `(15,11)` and `(30,11)` sit in the same cell of the table and all three were reached, and nothing about `n = 30` or `33` introduces an obstruction that `n = 15` lacks — the failures were search difficulty, not structure |
| the partitions it finds | **nonlinear and not tape-periodic**, since a tape-periodic one would have to be `g(j mod L)` and the whole point of this cell is that the linear/periodic family is empty here |
| **the truncation guard on the lopsided raise** (per-class 20, hits 64), which v6 measured at 46 corrected / 2 wrong | **0 wrong**, and the 46 corrections become refusals or ambiguities. The channel gets *worse* and that is the correct outcome: those 46 included 2 lies and the decoder could not tell which |
| the guard on a **coupled** raise, `Caps::raised(20)` | **95 of 100 unchanged, 0 wrong** — the guard costs nothing when the list is never truncated, which is the whole point of coupling |
| the guard at **`Caps::v0()`** | **bit-identical to v0**, because the field is `false` there. The decisions pin is the proof |
| how often v0 itself truncates | **rarely but not never**: `2^16/2053 = 31.9` expected against 64 kept, so truncation needs a 2× fluctuation. **Predicted under 1% of squares at `f = 16`** — which is the honest reason v0 was never caught by this |
| the general lesson, stated as a rule | truncate-then-filter converts **detection into miscorrection**, and the fix is to make truncation visible to the caller rather than to raise the budget |

## THE BARS

| bar | needed to count as met |
| --- | --- |
| **T1** the pin survives | 5 of 5 clean, 600 decisions identical to v0, v6's committed figures reproduced |
| **T2** the characterisation | all three `L mod 3` residues re-derived by the suite over a wide `(n, L)` sweep, with 0 violations, and the `L ≡ 1` vacuity shown rather than asserted |
| **T3** the three cases | each settled with an **exhibited and re-verified** partition, or printed **INCONCLUSIVE**. A randomised search may never return "impossible" |
| **T4** the guard is safe | 0 miscorrections at **every** cap setting tested, including the lopsided one that produced 2 in v6 |
| **T5** the guard is free where it should be | `Caps::v0()` bit-identical to v0; a coupled raise unchanged in rate |
| **T6** the guard's cost stated | the corrections it *removes* counted, not hidden — a safety fix that quietly lowers a rate is reported, because that is the trade being made |

**MET** if T1–T6 hold with results printed either way. **MISSED** if a randomised search
reports an impossibility, or if the guard's cost is presented as free where it is not.

## What must NOT be built

1. **No third part.** v6's verdict was that the engineering is done; this round is the two
   items that survive it and nothing more.
2. **No edit to v0's default behaviour**, permanently. `refuse_on_truncation` is `false` in
   `Caps::v0()` and the decisions pin proves it.
3. **No "impossible" from an incomplete search.** A randomised restart settles YES only.
4. **Nothing in `codegg-*/`, `codec-v1/`, or `eggSo-v0..v6/`.** v7 copies forward and pins.

## Measured (filled as parts land — never before)

Filled 2026-09-03, after `cargo build --release`, `cargo test` (69 tests),
`cargo clippy --all-targets -- -D warnings` clean with no suppressions, and `eggso7 audit`.

### T1 — the pin survives: 5 of 5 clean

| pin | checked | mismatches |
| --- | --- | --- |
| the copy vs v6's committed record | 13 figures | **0** |
| `region_of` vs `stalk.js`'s `regions()` | 22,139 | 0 |
| `arcs` vs `stalk.js`'s `arcs()` | 1,599 | 0 |
| the port vs eggSo-v0's structure | 6,153 | 0 |
| **the port vs eggSo-v0's decisions** | **600** | **0** |

The guard changes what the decoder does, so the first thing proved is that it does not change
what **v0** does. `refuse_on_truncation` is `false` in `Caps::v0()` and only there, and the
600-decision pin is what says so. The record pin includes v6's **2 miscorrections** at the
lopsided raise — the figure this round exists to drive to zero — fixed before the guard
touches it.

### T2 — the characterisation, finished: **409 cases, 0 disagreements**

```
a linear partition reaches ceil(L/3) on all four geometries iff
    L = 0 (mod 3):  n = 2 (mod 3)          <- eggSo-v5
    L = 1 (mod 3):  every n                <- this round
    L = 2 (mod 3):  n != 0 (mod 3)         <- this round
```

Re-derived by measurement over `n = 8..36`, `L = 3..18`: **409 cases, 0 disagreements.** And
the `L ≡ 1` vacuity is shown rather than asserted — the tape reaches the floor for every arm
with `b ≢ 0` in **every** case at that residue, and is **not** vacuous at the other two
(`L = 8` and `L = 12` at `n = 30` both fail it), so the claim is not empty.

That completes the linear family for **every** `L`, where v5 had only `L ≡ 0`.

### T3 — the three open cases: **2 of 3 settled**

| `(n, L)` | verdict | structure of the exhibited partition |
| --- | --- | --- |
| (15, 8) | REACHED | nonlinear, tape-periodic period 9 |
| (15, 11) | REACHED | nonlinear, tape-periodic period 11 |
| (30, 11) | REACHED | nonlinear, tape-periodic period 11 |
| **(30, 8)** | **REACHED** | nonlinear, tape-periodic period 9 |
| **(33, 8)** | **REACHED** | nonlinear, tape-periodic period 9 |
| **(33, 11)** | **INCONCLUSIVE** | — |

**What settled them was following the data, not more budget.** The first grid walk found
`(30,8)` on its third restart after 354 million nodes, and the *exhibited* partitions at
`(15,11)` and `(30,11)` both came back **tape-periodic with period 11**. That named a family:
`class(j) = g(j mod P)` is `3^P` choices of `g` against `3^(n²)` for the grid. Searching it
exhaustively settles `(30,8)` and `(33,8)` in **milliseconds** and finds a period-9 solution
at `(15,8)` that the grid walk had not.

Two structural facts came out of it, and both are pinned:

- **row windows are redundant.** When `L ≤ n`, a row window of `L` consecutive cells *is* `L`
  consecutive tape indices. Dropping them is exact and free.
- **a period `P` dividing `n` is hopeless.** Every row then starts at the same phase, so every
  row carries an identical pattern and every **column is constant** — all `L` cells of a
  column burst in one class. At `n = 33` that rules out `P = 1, 3, 11, 33`, **including the
  period 11 that settles `(15,11)` and `(30,11)`.** That is exactly why `(33,11)` is the hard
  one, and `33 = 3 × 11` while `30` is not a multiple of 11 is the whole difference.

`(33,11)` survived every period to 15 (excluding the divisors of 33) and 960 million grid
nodes across 8 restarts. It is reported **INCONCLUSIVE** and never as an impossibility: a
randomised search and a bounded periodic family can only ever say YES.

One efficiency note worth recording because it changed nothing but the clock: the first
periodic search rebuilt the whole `n²` grid for every candidate and took **7m 9s**. Filtering
on the tape constraint first — necessary, and its phases genuinely cover all `P` residues —
brought the same answers in **11.7s**.

### T4, T5, T6 — the guard

18 erasures in one class at n = 32, corrected of 100:

| caps | corrected | ambiguous | refused | **wrong** |
| --- | --- | --- | --- | --- |
| v0, untouched (guard off) | 0 | 0 | 100 | 0 |
| v0 + guard | 0 | 0 | 100 | 0 |
| **LOPSIDED per-class 20, hits 64 — v6's row** | 46 | 3 | 49 | **2** |
| **the same, + guard** | **0** | **51** | 49 | **0** |
| `Caps::raised(20)` — coupled, guard on | **95** | 5 | 0 | **0** |

- **T4 MET.** The worst `wrong` across every **guarded** row is **0**. The 2 is v6's own
  number, kept as the baseline being fixed.
- **T5 MET.** `Caps::v0()` is bit-identical to v0 — the decisions pin proves it — and the
  coupled raise is unchanged at 95 of 100, because it never truncates.
- **T6 MET, and it is a cost.** The guard **takes away** all 46 corrections the lopsided raise
  was making. That is the correct outcome: those 46 included the 2 lies and the decoder could
  not tell which. A rate that falls because the answers it was giving were not trustworthy is
  not a regression — but it is a real reduction and it is printed, not hidden.

**How often v0 itself truncates**, which is the honest reason this was never caught:

| f in class | guard off | guard on | truncation rate |
| --- | --- | --- | --- |
| 8 | 100/100 | 100/100 | **0.0%** |
| 12 | 100/100 | 100/100 | **0.0%** |
| 14 | 100/100 | 100/100 | **0.0%** |
| 16 | 100/100 | 100/100 | **0.0%** |

I filed "under 1%"; measured **0.0%** at every `f` up to v0's own cap. So v0's margin of two
is doing exactly its job, and the bug only ever existed for someone who raised the cap.

### The rule — the shippable half

> **Truncating a candidate list and then filtering by a second check converts detection into
> miscorrection.** The fix is to make the truncation visible to the caller, not to raise the
> budget: raising the budget only moves the threshold, and the failure is silent on either
> side of it.

### Predictions against results

| claim | landed |
| --- | --- |
| the pin at default caps | **HELD** — 5 of 5, 600 decisions identical to v0 |
| the characterisation, 0 violations | **HELD** — 409 cases, 0 disagreements |
| **all three open cases solvable, at least two found** | **HELD on "at least two" — 2 of 3.** `(33,11)` is still open, and the reason turned out to be structural rather than search difficulty: `33 = 3 × 11` kills the period that settles its siblings |
| the partitions are nonlinear and **not tape-periodic** | **MISSED, and usefully.** All five are nonlinear as called, but every one is **tape-periodic** — and following that is what settled two of the three. I had reasoned that a periodic solution would have to be `g(j mod L)` and that family is empty here; the error was fixing the period at `L`. The winning periods are 9 and 11, and 9 ≠ 8 |
| the guard on the lopsided raise | **HELD** — 2 wrong → 0, and the 46 corrections become 51 ambiguous + 49 refused |
| the guard on a coupled raise | **HELD** — 95 of 100 unchanged, 0 wrong |
| the guard at `Caps::v0()` | **HELD** — bit-identical, by the decisions pin |
| v0's own truncation rate under 1% | **HELD**, and stronger: 0.0% |

### The bar arithmetic, settled

| bar | result |
| --- | --- |
| **T1** the pin survives | **MET** |
| **T2** the characterisation | **MET** — all three residues, 409 cases, 0 disagreements |
| **T3** the three cases | **MET** — 2 settled with exhibited and re-verified partitions, 1 printed INCONCLUSIVE |
| **T4** the guard is safe | **MET** — 0 wrong across every guarded setting |
| **T5** the guard is free where it should be | **MET** |
| **T6** the guard's cost stated | **MET** — all 46 corrections removed, and said so |

**Six bars, six met. One filed prediction missed, and the miss is what settled two of the
three open cases:** I called the solutions non-periodic, they were periodic, and the family I
had dismissed was the one worth searching.
