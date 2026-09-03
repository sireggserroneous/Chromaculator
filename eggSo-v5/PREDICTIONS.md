# eggSo v5 predictions — filed 2026-09-03, BEFORE a line of the round was written

The series convention, unchanged since v0: every number below is a guess, written down
first, and the measured value is filled in beside it afterwards or not at all. Misses
stay. A prediction quietly edited to match its result is worth less than no prediction.

The twentieth codec experiment and the sixth in the fold-native lineage. Rust, own crate,
empty `[dependencies]`, as `eggSo-v4/` established for this line.

Vladimir's rule for the whole round, and the reason Part 3 exists: *"Also lets measure
rather than argue."*

## What this is, in one sentence

Two threads out of v4's accident: what this grid looks like at **degree three**, which is
the question Cayley could state and not see, and which three-class partition actually
**minimises a burst** — the figure of merit v4 discovered was the real one after v0 had
spent its whole verdict on a different statistic.

## Why this round exists

`eggSo-v4` (commit `63598b8`) closed the site README's oldest open question: under
`rho = 2^((r+c)−(n−1))` the three regions are the two Fatou basins and the Julia set of
`z ↦ z²`, and the anti-transpose is the inversion between them. It also established
**Cayley's wall** — 38.3% of a cubic Newton map's basin boundary touches all three basins
at once, so no straight seam can separate three of them, which is why this geometry is a
degree-2 object.

And it discovered by accident what nobody planned for. **The separation statistic eggSo-v0's
entire verdict rested on moves no error channel at all** — every partition scores 397 to
400 of 400 on the pair channels. What the fold's geometry actually costs is **burst
spread**: on a 31-cell flagged burst, of 200 trials, `fold` manages 18, `blocks` manages 0,
and `(r+c) mod 3` manages 200.

So: Part 1 builds the degree-3 geometry, whose bar is **the picture and the name** and not
a channel win. Part 2 optimises the figure of merit that turns out to matter, which has
never been optimised. Part 3 measures three claims in this line that currently rest on my
reasoning.

## Part 1 — the construction, stated before building

`rho` gives a modulus and leaves the angle free — and that freedom is exactly the
`arcs(n)[d]` cells sharing one band. Supply the angle from the site's **own** fill order:
`stalk.js:102-110`'s `cellOrder` reads each anti-diagonal "from the bottom-left corner
upward", i.e. `for(let r = Math.min(n-1, d); r >= 0; r--)`. So a cell's Hankel position on
its band is

```
k(r, c) = min(n−1, r+c) − r
```

and the coordinate is

```
z(r, c) = rho(r, c) · exp(2·pi·i · k(r,c) / arcs(n)[r+c])
```

built entirely from the site's own two orderings — the band for the modulus, the Hankel
walk for the angle. Classify each cell by which root of `z³ − 1` Newton's method reaches
from `z(r,c)`. That is the degree-3 partition, and it is what the fold **would have to be**
if this geometry could carry three basins.

`z` is injective: within a band the moduli agree and the `arcs(n)[d]` angles are distinct;
across bands the moduli are distinct powers of two.

### The bar is the picture and the name, filed up front

So the round cannot be read as chasing a win it will not get:

- It **cannot** beat `(r+c) mod 3`, which already scores 200 of 200 on the burst channel.
- It will probably be **worse**, and for a reason worth stating: **Fatou basins have
  interiors, and interiors concentrate.** v4's own ASCII picture of the cubic shows large
  solid lobes. A burst landing inside a lobe lands in one class. A fractal boundary
  scatters only the cells near the boundary, which are a minority.
- Its class sizes will be **unbalanced**, because the three basins of `z³ − 1` do not have
  equal measure inside any particular annulus, and `rho` only reaches the radii the grid's
  bands provide.

The name: **the basin decomposition of a cubic Newton map** — a Newton fractal. Cayley 1879
for the question, Julia 1918 and Fatou 1919-20 for the theory, and the computer era for the
picture. v4's lineage audit found zero prior mentions of any of it across nineteen
experiments.

## Part 2 — the objective, stated precisely

For a partition `C` of the `n × n` grid into three classes and a burst length `L`,

```
worst(C, L) = max over bursts B of length L  of  max over classes k of  |B ∩ k|
```

over four burst geometries: along a **row**, along a **column**, along an **anti-diagonal**
(consecutive cells of one band, tape step `n−1`), and along the row-major **tape** index,
which wraps at row boundaries and is what a real contiguous storage wound looks like. A
burst of `L` cells over 3 classes always gives some class at least `ceil(L/3)`, so
**`ceil(L/3)` is the floor**, and the question is which partitions reach it on all four
geometries at once.

## Measured during planning — ground, not predictions

Computed 2026-09-03 before this file existed, by exhaustive enumeration of **every
placement** of a length-`L` run in each geometry, over all nine `(a,b)` of the linear
family `C(r,c) = (a·r + b·c) mod 3`, at `n = 15, 16, 17, 30, 31, 32, 33` and
`L = 6, 9, 12, 18`. A number already known cannot honestly be filed as a guess, so it sits
here; **the suite re-derives each one and must match rather than quote it.**

| claim | measured |
| --- | --- |
| a row burst reaches `ceil(L/3)` iff `b` is nonzero mod 3 | **confirmed**, 0 violations |
| a column burst iff `a` is nonzero | **confirmed**, 0 violations |
| an anti-diagonal burst iff `a` differs from `b` | **confirmed**, 0 violations |
| the tape burst iff `b` is nonzero **and** `a = b·n (mod 3)` | **confirmed**, 0 mismatches. The mechanism is a phase slip at the row boundary: crossing `(k, n−1)` to `(k+1, 0)` shifts the class by `a − b(n−1)`, which equals the in-row step `b` exactly when `a = b·n`. When it does not, the worst case rises to `ceil(L/3) + 1` |
| `j mod 3 = ((n mod 3)·r + c) mod 3` | **confirmed** over every cell, `n = 2..64` |
| **the linear theorem** | the system "`a` nonzero, `b` nonzero, `a` differs from `b`, `a = b·n`" over `(Z/3)²` has **no solution at `n = 0 (mod 3)`** (`a = 0` contradicts `a` nonzero) and **none at `n = 1`** (`a = b` contradicts `a` differs from `b`). At `n = 2` it has exactly two: `(a,b) = (1,2)` and `(2,1)`. So **only `n = 2 (mod 3)` admits a linear partition that is burst-optimal on all four geometries**, and there it admits precisely two |
| what that says about v4's `idx3` | at `n = 32`, `(a,b) = (2,1)`, which **is** `j mod 3`. Its clean sweep was optimal by accident of `32 = 2 (mod 3)`, exactly as `seam.rs` refused to generalise. The failures are not marginal: at `n = 31, L = 12` the anti-diagonal lands **12 of 12** in one class; at `n = 33` a column lands **12 of 12** |

So the round's question was never the linear family. It is **the half the linear family
cannot reach**, and that is a search, not an argument.

## An amendment, filed before any code and against the plan's own prediction

The plan this round executes files:

> a nonlinear partition at `n = 0` or `1 (mod 3)` **exists**, and annealing finds one **at**
> the floor, not merely within 1 of it. Reasoning: the linear obstruction is an algebraic
> coincidence over `(Z/3)²`, not an information bound, and the floor `ceil(L/3)` is
> achievable by a periodic tiling that need not be linear in `j`.

**I now expect that to be wrong whenever 3 divides L, and here is the argument, derived
while designing the search and therefore filed as a prediction rather than as ground.** It
is stated in full so that a measurement can kill it.

**The periodicity lemma.** Suppose `3 | L`. Then `ceil(L/3) = L/3`, and a tape window of
`L` cells whose three class counts are each at most `L/3` must have them all *exactly*
`L/3` — there is no slack. Slide the window one cell: it loses `class(j)` and gains
`class(j+L)`, and both windows are exactly balanced, so `class(j) = class(j+L)` for every
`j = 0 .. n²−L−1`. **The tape is forced to be `L`-periodic**, i.e. `class(j) = g(j mod L)`
for a balanced `g : Z/L → Z/3`.

Everything else then follows by arithmetic, with no reference to linearity:

- a **row** window is `L` consecutive tape indices, hence all of `Z/L` once, hence exactly
  balanced — **always satisfied**, for free;
- a **column** window steps the tape index by `n`, so it walks the coset generated by
  `d = gcd(n, L)`, whose order is `L/d`; `L` consecutive steps cover that coset exactly `d`
  times, so the class count is `d · |{x in coset : g(x) = k}|`. Balance therefore **requires
  `3 | L/gcd(n, L)`**, and `g` balanced on every residue class mod `gcd(n, L)`;
- an **anti-diagonal** window steps by `n − 1`, so the same argument **requires
  `3 | L/gcd(n−1, L)`**.

**The prediction, then:** for `3 | L`, **no partition of any kind** — linear, nonlinear,
annealed, or handed down — reaches the floor on all four geometries unless

```
3 | L/gcd(n, L)   and   3 | L/gcd(n−1, L)
```

and I expect those two divisibilities to be **necessary and sufficient**, sufficiency by a
`g` balanced on every coset mod `gcd(n,L)` and mod `gcd(n−1,L)`, which need not be linear —
so nonlinearity buys something real, just not everything.

Worked consequences, called now:

| `(n, L)` | `gcd(n,L)` | `gcd(n−1,L)` | called |
| --- | --- | --- | --- |
| `(3, 3)` | 3 | 1 | `L/gcd(n,L) = 1`, **impossible** |
| `(4, 3)` | 1 | 3 | `L/gcd(n−1,L) = 1`, **impossible** |
| `(5, 3)` | 1 | 1 | both 3, **possible**, and linear already does it |
| `(6, 3)` | 3 | 1 | **impossible** |
| `(6, 6)` | 6 | 1 | **impossible** |
| `(15, 6)` | 3 | 2 | `L/gcd(n,L) = 2`, **impossible** |
| `(16, 6)` | 2 | 3 | `L/gcd(n−1,L) = 2`, **impossible** |
| `(31, 6)` | 1 | 6 | `L/gcd(n−1,L) = 1`, **impossible** |
| `(33, 6)` | 3 | 2 | **impossible** |
| `(32, 6)` | 2 | 1 | 3 and 6, **possible** — and `n = 2 (mod 3)`, so linear does it |

Every `L` the plan named — 6, 9, 12, 18 — is divisible by 3. **So if this lemma holds, the
plan's search has no content at any of them, and the search's real content is at `L` not
divisible by 3, where the floor has slack and the periodicity argument does not start.**
The `L` set is therefore widened to `{6, 8, 9, 11, 12, 18}` — the plan's four, plus 8 and
11 — and that choice is recorded here, before the first run, rather than made after seeing
a flat result.

If the lemma is wrong, the annealer will find a floor-reaching partition at some `3 | L`
case the table above calls impossible, and this section is a filed miss.

## Part 3 — measure rather than argue

| claim, as it was argued | how it gets measured | called |
| --- | --- | --- |
| **`diag3` cannot do v1(b)'s unique row.** An unflagged 12-cell in-region burst is 4 unknowns in one 11-bit class equation, and only the mirror has a second copy | the unflagged in-region burst — 12 flips in one row, all inside Inner or all inside Outer, `eggSo-v1/tools/versus.js:122-128`'s own channel — against every arm | **0 corrected for every arm, and it is a pigeonhole rather than a measurement**: 12 cells over 3 classes puts at least 4 in some class, and this decoder searches to depth 2 per class. So **v1(b)'s row survives**, and the number that actually discriminates is how many arms **miscorrect** on it, which v1(b) does not |
| **The burst bound is `ceil(L/3)`.** | asserted, then every measured `worst(C, L)` checked against it | holds; a partition beating it is an arithmetic error in the harness, not a discovery |
| **`(r+c) mod 3` inherits the fold's blind spot.** Both put a full anti-diagonal in one class | re-measured beside the degree-3 arm | `diag3` puts **32 of 32** in one class; the cubic arm **splits it**, and this is the one channel where it wins |

## THE BARS

| bar | needed to count as met |
| --- | --- |
| **N1** the coordinate | `z(r,c)` well defined for every cell of every `n` tested; modulus exactly `rho`; angle the site's **own** Hankel position, pinned against `stalk.js`'s `cellOrder` through node |
| **N2** the picture | the grid coloured by basin, rendered as ASCII, printed in the README, and legibly *not* a seam |
| **N3** the honest verdict | the degree-3 arm measured on every channel beside `diag3`, its predicted loss stated **before** the numbers, its actual result printed whichever way it falls |
| **N4** the name | the basin decomposition of a cubic Newton map, Cayley / Julia / Fatou cited, the zero-prior-mentions audit noted |
| **B1** the floor | `ceil(L/3)` asserted as a bound; nothing measured beats it |
| **B2** the linear family | all nine `(a,b)` at every `n` in the range, the four shatter conditions each confirmed or refuted, **re-derived by the suite rather than quoted** |
| **B3** the theorem | the impossibility re-derived at both `n = 0` and `n = 1 (mod 3)`, with the two solutions at `n = 2` exhibited |
| **B4** the search | exact for small `n`, annealing for large, best-found reported with its gap to the floor; a search that loses to the linear arm says so |
| **B5** the optimum, if it exists | a partition rule reaching the floor on all four geometries at every `n`, or a statement of exactly which `n` and `L` it is impossible for |
| **M1** v1(b)'s row | the unflagged in-region burst against every arm; if a cheap arm takes it, v1(b)'s justification is withdrawn in v1's README |
| **M2** no silent wrong data | miscorrections reported per arm and channel; an arm that lies is disqualified loudly, as v4 did |
| **P1** the port stays pinned | the modules copied from v4 reproduce v4's **committed** `measured-*.json` figures, so the copy cannot drift |

**MET** if N1–N4 and B1–B4 hold with their results printed either way. **MISSED** if the
degree-3 arm is presented as a capability win, or if a search failure is hidden.

## The search's honesty rules, fixed in the source before the first run

- **one seed**: `20260903`, the filing date, the house convention since v0.
- **one schedule**: geometric cooling `T = 2.0 · (0.005/2.0)^(step/budget)`, single-cell
  reclass moves, Metropolis acceptance, energy = total excess over the floor summed across
  every window of all four geometries. Energy 0 exactly when `worst(C,L) = ceil(L/3)`.
- **one budget**: `2_000_000` proposed moves per `(n, L)`, identical in `audit` and
  `audit --full`. `--full` widens the linear sweep and the picture resolution and **does
  not** widen the search.
- **one seeding**: the lowest-energy arm of the nine linear `(a,b)` at that `(n, L)`.
- A search that fails to beat its linear seed is a result and is printed as one. It is not
  re-tuned until it wins, and if it ever is, this file names the first configuration and
  the number it produced.

The exact search is a depth-first enumeration over canonical partitions — cells in
row-major order, class labels restricted so first occurrences run `0, 1, 2` (which quotients
the 6-fold relabelling symmetry exactly), pruned the instant any window's class count
exceeds the floor. Counts only grow, so the prune is sound and the enumeration is complete.
It carries a fixed node cap of `200_000_000`; a case that exhausts it reports
**INCONCLUSIVE** and never "no solution".

## Per-part predictions

### Part 1 — the degree-3 geometry

| claim | called |
| --- | --- |
| `z` is injective over every cell, and its modulus is `rho` exactly | holds; a miss is a bug |
| the Hankel index `k` vs `stalk.js`'s `cellOrder` | 0 mismatches, every `n` in 2..40 |
| the degree-3 class sizes at `n = 32` | **unbalanced**, no class within 5% of a third (i.e. none in 324..358). Finer guess: class 0 at **370–400**, the other two at **310–340**, because every band's `k = 0` cell sits at angle 0 on the positive real axis, which is deep inside root 1's basin, and there are `2n−1 = 63` bands |
| the degree-3 arm on a 12-cell row burst | **worse than `diag3`**: `worst` at least 6 against `diag3`'s 4, because lobes have interiors |
| the degree-3 arm on the full anti-diagonal | **the one channel it wins**: `worst` about `n/3` (10–14 at `n = 32`) against `diag3`'s 32, because the cubic's boundary crosses the band where a seam cannot |
| corrected counts on the anti-diagonal channel | **0 for every arm including the cubic**, since 32 cells over 3 classes is at least 11 per class and the decoder searches to depth 2. The win is on the spread, not the channel, and will be reported that way |
| unsettled cells (Newton not converged in 200 iterations) | **fewer than 5 of 1024**; each resolved by nearest root and **counted in the record**, because a partition must be total |
| the cubic arm's separation | **below** `diag3`'s 0.6673, because unbalanced classes lower it — 0.63 to 0.66 |

### Part 2 — the burst optimum

| claim | called |
| --- | --- |
| B1, the floor | never beaten, at any `(C, n, L)` measured |
| the four shatter conditions | all four re-derived with **0 violations**, reproducing the planning ground |
| the linear theorem at `n = 0, 1, 2 (mod 3)` | re-derived: 0 solutions, 0 solutions, exactly `(1,2)` and `(2,1)` |
| the exact search at `n = 3..6` where 3 divides L | **matches the periodicity lemma case for case**: infeasible at `(3,3)`, `(4,3)`, `(6,3)`, `(6,6)`; feasible at `(5,3)` |
| the exact search at `n = 3..6` where 3 does not divide L | **reaches the floor at every one of them**, because the floor has slack there and the periodicity argument does not start |
| the annealer where 3 divides L, on a case the lemma calls impossible | **gap of exactly 1**, never 0, at every one of them |
| the annealer where 3 does not divide L | **reaches the floor**, gap 0, at `n = 15, 16, 30, 31, 33` |
| the plan's own filed prediction — a nonlinear partition at the floor for `n = 0, 1 (mod 3)` | **MISSED as filed**, and the amendment above says exactly why. It is right that the linear obstruction is not an information bound; it is wrong that the floor is therefore reachable |
| the name | **a cyclic / block interleaver** — `g(j mod L)` on the tape is precisely a block interleaver's read-out, and the series counts landing on prior art as a legitimate result |

### Part 3 and the pins

| claim | called |
| --- | --- |
| the unflagged in-region burst, every arm | **0 corrected**, every arm, by pigeonhole |
| miscorrections on that channel | **greater than 0 for `fold` and `blocks`** (the whole burst lands in one class, so the pair search has aliases to find) and **0 for `diag3` and `idx3`** (4 per class is past the search depth, so it refuses) |
| v1(b)'s justification | **survives**, and v1's README is not edited |
| P1, the v4 figures | reproduced exactly: 89,439 cells / 0 exceptions, the four pin counts 22,139 / 1,599 / 6,153 / 600 with 0 mismatches, `diag3` at 0.6673 with classes 341/342/341, and `diag3`'s flagged burst sweep `200/200/200/200/200` at lengths 12/15/18/24/31 |
| the five site pins | all clean, or **SKIPPED loudly** if node is absent |

## The bar arithmetic, filed plainly

| bar | needs | call |
| --- | --- | --- |
| N1, N2, N4 | exact, drawn, cited | **YES** |
| N3 | the loss printed before the numbers | **YES**, and it is the point of Part 1 |
| B1, B2, B3 | re-derived, not quoted | **YES** |
| B4 | a search that reports its losses | **YES** |
| B5 | an optimum or a statement of where it cannot exist | **the interesting one** — I expect the statement, not the rule, and the statement is stronger |
| M1 | measured either way | **YES**, and it is a pigeonhole, so v1(b) keeps its row |
| M2 | miscorrections named per arm | **YES** |
| P1 | v4's committed figures reproduced | **YES** or the round stops |

## Measured (filled as parts land — never before)

Filled 2026-09-03, after `cargo build --release`, `cargo test` (54 tests),
`cargo clippy --all-targets -- -D warnings` clean with no suppressions, and `eggso5 audit`.
Every number below is from those runs; `measured-*.json` beside this file is what the
binary wrote.

### P1 and the pins: all six clean

| pin | checked | mismatches |
| --- | --- | --- |
| **the copy vs v4's committed record** | **37 figures** | **0** |
| `region_of` vs `stalk.js`'s `regions()`, n = 2..40 | 22,139 | 0 |
| `arcs` vs `stalk.js`'s `arcs()`, n = 2..40 | 1,599 | 0 |
| **`hankel_k` vs `stalk.js`'s `cellOrder`**, n = 2..40 | **22,139** | **0** |
| the port vs v0's structure | 6,153 | 0 |
| the port vs v0's decisions | 600 | 0 |

P1 held: the copied modules reproduce 89,439 cells / 0 exceptions, v4's four pin counts
with 0 mismatches, `diag3` at 341/342/341 and 0.6673, and `diag3`'s flagged burst sweep
`200/200/200/200/200`. The `cellOrder` pin is what makes Part 1 a construction about the
site rather than about me: the angle is the site's fill order, checked against the site's
own walk.

### Part 1 — the degree-3 geometry: the name and the picture HELD, **two filed numbers MISSED**

| claim | called | landed |
| --- | --- | --- |
| `z` injective, modulus exactly `rho` | holds | **holds**, every cell of n = 2..40 |
| the Hankel index vs `cellOrder` | 0 mismatches | **0 of 22,139** |
| every band's `k = 0` cell in class 0 | the stated mechanism | **holds**, all `2n−1` bands, n = 8/16/32 |
| **the class sizes at n = 32** | **unbalanced, no class within 5% of a third; class 0 at 370–400** | **348 / 338 / 338 — the nearest class is 1.0% off a third. MISSED, and badly.** The direction was right (class 0 is the heavy one) and the magnitude was wrong by a factor of six |
| **the separation** | **0.63–0.66, below `diag3`** | **0.6673 — the same as `diag3` to four decimals. MISSED** |
| the 12-cell row burst | worse than `diag3`, ≥ 6 | **12 against `diag3`'s 4** — the maximum possible. Held, and worse than guessed |
| the full anti-diagonal | the one channel it wins, ≈ n/3 | **11 against `diag3`'s 32.** Held, and 11 is exactly `⌈32/3⌉` |
| corrected on the anti-diagonal channel | 0 for every arm | **0 for both**, as filed |
| unsettled cells at n = 32 | fewer than 5 of 1024 | **0 of 1024** — and **15 of 4096 at n = 64**, so the fallback is real at scale and is counted |

**The separation miss is the better of the two, and it is v4's own finding arriving again
from a new direction.** A Newton fractal and a set of diagonal stripes have the same
separation to four decimal places — 0.6673 — and could not behave more differently on a
burst: 12 against 4 on a row, 11 against 32 on the band. v4 showed this with `blocks`
against `idx3`, two arms that share the statistic and diverge on the channel. That it also
fails to tell a *fractal* from a *seam* is the strongest available statement that the
statistic v0 spent its verdict on is not a figure of merit.

The imbalance prediction failed because the mechanism I named is real but self-cancelling:
the `2n−1` band-initial cells do all land in class 0, but the remaining cells of those same
bands are correspondingly *under*-represented in class 0, and the two nearly cancel. The
residue is +7 cells at n = 32, not +42. Measured across widths: 15.6% off a third at n = 8,
7.4% at n = 16, 1.0% at n = 32, 1.7% at n = 64 — so the construction is close to balanced
and gets closer, which is the opposite of what was filed.

### Part 2 — the burst optimum: **B1–B5 all met, and the plan's own prediction MISSED**

**B2, the linear family, split by `3 | L` because the two halves disagree — which is
itself a result.**

| range | cases compared | violations |
| --- | --- | --- |
| `3 | L` (the ground's own L = 6, 9, 12, 18) | **927** | **0** — the four shatter conditions hold exactly |
| `3 ∤ L` (L = 8, 11, which the ground never covered) | **504** | **28 disagreements** |

Every one of the 28 is the **tape** condition being *sufficient but not necessary*. The
mechanism is slack: with `3 | L` a window at the floor is `L/3` three times over and has no
room, but with `3 ∤ L` it is `L = 3⌈L/3⌉ − s` for `s ∈ {1,2}`, so a window can be
`(f, f, f−s)` and the phase slip's one extra cell has somewhere to go. **So the four
conditions are exact exactly where the ground computed them, and the tape condition
over-predicts failure elsewhere.** The ground is confirmed and its boundary is now known.

**B3, the theorem, re-derived over n = 2..64:** 0 solutions at `n ≡ 0`, 0 at `n ≡ 1`,
exactly `(1,2)` and `(2,1)` at `n ≡ 2`, at all 21 widths of each residue. And `j mod 3` is
`((n mod 3)·r + c) mod 3` over every cell of n = 2..64, so v4's `idx3` is the linear arm
`(2,1)` and its clean sweep was optimal by accident of `32 ≡ 2 (mod 3)`.

**The periodicity lemma — the amendment filed against the plan — HELD, and by a complete
enumeration and not only by argument.**

| check | result |
| --- | --- |
| the lemma vs exhaustive enumeration, n = 3..6 | **5 of 5 agree** |
| the lemma vs exhaustive enumeration, n = 15, 16, 30, 31, 33 | **20 of 20 agree**, in 122 ms total |
| the construction where the lemma permits | **built and verified at the floor in every such case** |
| every worked case in the table above | landed exactly as called, `(3,3)` `(4,3)` `(6,3)` `(6,6)` `(15,6)` `(16,6)` `(31,6)` `(33,6)` impossible, `(5,3)` `(32,6)` possible |

**B4, the search, and the plan's prediction.** The plan filed: *a nonlinear partition at
`n ≡ 0` or `1 (mod 3)` exists, and annealing finds one at the floor.* Both halves were
tested separately and they land on opposite sides.

| claim | landed |
| --- | --- |
| such partitions **exist** at `n ≡ 0 (mod 3)` | **TRUE where `3 ∤ L`** — exhibited by enumeration at (15,8), (15,11), (30,11), and at (4,4) (5,4) (5,5) (6,4) (6,5). The plan was right that the linear obstruction is not an information bound |
| such partitions exist where `3 | L` | **FALSE** — the lemma forbids it and the enumeration confirms 20 of 20. The plan was wrong to conclude the floor is therefore reachable |
| **annealing finds one** | **MISSED, at both schedules.** On **11 cases** the enumeration built a floor-reaching partition that **neither** annealing schedule found |
| my own amendment's "gap of exactly 1, never 0" at `3 | L` | **never 0, as called** — but schedule A produced a gap of **2** at (15,12), so "exactly 1" is a partial miss |
| my own "the annealer at `3 ∤ L` reaches the floor at n = 15, 16, 30, 31, 33" | **MISSED** — gap 0 only at n = 16 and 31, where the *linear* arm already reached it. The annealer added nothing anywhere |

**The schedule was re-tuned, and this file names the first configuration and its numbers,
as the filed rule requires.** Schedule A, as filed — geometric `T = 2.0 → 0.005`, seed
20260903, budget 2,000,000 — produced **6 of 30 at the floor, 3 beat their seed, mean
acceptance 0.30%**, and on one case (15,12) it *lowered the energy while raising the worst
case*. The diagnosis: a single-cell move touches up to `4L` windows, so `|Δ|` runs to a few
tens, and at `T = 2.0` an uphill move of +10 is taken with probability `e^−5 ≈ 0.7%` — which
is the measured rate. Schedule A was a greedy descent wearing an annealer's clothes and it
measured the temperature rather than the question. Schedule B makes one principled change
and no more, `T_hot = L`, putting the temperature on the energy's own scale: **mean
acceptance 8.72%, 0 worst-case regressions — and the same 6 of 30 at the floor.** Both
tables are printed by `eggso5 optimum` and neither is deleted.

So the honest reading of B4 is not that the temperature was wrong. It is that **a local
search is the wrong instrument for this objective and a complete enumeration is the right
one**: the constraint is tight, so solutions are sparse but reachable by propagation, and
the enumeration found in milliseconds what 2,000,000 annealing moves at two temperatures
could not find at all. Three cases — (30,8), (33,8), (33,11) — exhausted the 200,000,000
node cap and are reported **INCONCLUSIVE**, which is neither a construction nor a proof.
27 of 30 settled outright.

**B5, the answer, in two halves because the measurement supports two.** For `3 | L` and a
run that fits across the grid, a partition reaching the floor on all four geometries exists
**exactly when `3 | L/gcd(n, L)` and `3 | L/gcd(n−1, L)`** — necessity by the lemma,
sufficiency by the construction `class(j) = g(j mod L)` with `g` balanced on every coset
mod `gcd(n, L)` and mod `gcd(n−1, L)`. On the tape that is a **block interleaver's**
read-out, so the name is prior art, as filed, and the series counts that as a result. And
`g` need not be linear: when 3 divides `gcd(n, L)` a linear `g` is constant on a coset and
cannot do it, which is the one place in this round where nonlinearity earns something. For
`3 ∤ L` the floor has slack, no obstruction is known, and partitions at the floor exist at
`n ≡ 0 (mod 3)` where no linear one does.

**Linearity is not the obstruction. Arithmetic is, and only when `3 | L`.**

### Part 3 — measure rather than argue

**M1 held, and v1's README is not edited.** The unflagged 12-cell in-region burst:
**0 corrected, every one of the nine arms.** As filed, it is a pigeonhole and not a
measurement — 12 cells over 3 classes puts at least 4 in some class, and this decoder
searches to depth 2 per class — so no three-class partition can take this channel at any
width. v1(b)'s 103% overhead keeps its only justification, and v1's own suite re-run here
confirms its side of it: `B4 MET 100.00% unflagged in-region burst`.

| claim | called | landed |
| --- | --- | --- |
| 0 corrected, every arm | 0 | **0 for all nine** |
| miscorrections on that channel: `fold` and `blocks` > 0 | > 0 | **`fold` 10, `blocks` 4** — held |
| miscorrections: `diag3` and `idx3` = 0 | 0 | **`idx3` 0, `diag3` 1. Partial MISS** — `diag3` lied once in 400 |

**M2, every arm that lied, is printed per channel** by `eggso5 arms` — 24 arm/channel pairs
carry a miscorrection, none of them on any flagged channel. The one worth naming: on the
full anti-diagonal, `diag3` miscorrects 6 of 400 and `idx3` 3 of 400, while the **cubic arm
miscorrects 0** — so on the single channel where the degree-3 partition wins the spread, it
also stops lying. That is a narrow win and is reported as a narrow win.

**B1 held:** `worst(C, L)` is asserted against `⌈L/3⌉` inside `optimum::worst_all`, on every
measurement the round takes, and nothing came in under it.

**And the reason none of it generalises across n**, measured at n = 33 with 18-cell flagged
bursts, corrected of 200 with (mean/worst cells in one class):

| channel | `diag3` | `idx3` | `blocks` | `cubic` |
| --- | --- | --- | --- | --- |
| row | 200 (6.0/6) | 200 (6.0/6) | 0 (18.0/18) | 177 (11.8/18) |
| column | 200 (6.0/6) | **0 (18.0/18)** | 200 (10.5/11) | 176 (11.9/18) |
| anti-diagonal | **0 (18.0/18)** | 200 (6.0/6) | 200 (10.4/11) | 200 (7.6/10) |
| tape | 200 (6.4/7) | 200 (6.0/6) | 7 (17.8/18) | 184 (11.2/18) |

The gap between mean and worst is the whole mechanism. The erasure decoder refuses above 16
flagged in one class, so an arm whose *mean* is already 18 — `idx3` on a column at n = 33,
where it degenerates to `cols3`; `diag3` on an anti-diagonal, which is its level set — is
detected on every trial and corrects nothing. The cubic arm touches 18 at its worst and
sits near the floor on average, so it corrects most trials and loses the rest: a fractal has
no periodic bad case, only a rare one. That is exactly the difference `worst(C, L)`
optimises, and it is why the objective is stated as a maximum and not an average.

### The bar arithmetic, settled

| bar | result |
| --- | --- |
| **N1** the coordinate | **MET** — pinned to `cellOrder` over 22,139 cells |
| **N2** the picture | **MET** — the grid by basin, printed, and legibly not a seam |
| **N3** the honest verdict | **MET** — the loss filed first, printed either way, and it landed |
| **N4** the name | **MET** — the basin decomposition of a cubic Newton map |
| **B1** the floor | **MET** — asserted on every measurement, never beaten |
| **B2** the linear family | **MET** — 927 cases at `3 | L`, 0 violations, re-derived not quoted |
| **B3** the theorem | **MET** — both impossibilities and the two solutions, n = 2..64 |
| **B4** the search | **MET** — and it lost, at both schedules, and says so |
| **B5** the optimum | **MET** — an exact characterisation for `3 | L`, a construction, and three cases left open |
| **M1** v1(b)'s row | **MET** — it survives, as a pigeonhole |
| **M2** no silent wrong data | **MET** — 24 arm/channel pairs named |
| **P1** the port stays pinned | **MET** — 37 figures against v4's committed record |

**Twelve bars, twelve met. Five filed numbers missed: the cubic arm's class sizes, its
separation, `diag3`'s miscorrection count on the in-region burst, the annealer's gap at
`(15,12)`, and the annealer reaching the floor at all.** The plan's own prediction missed on
one half and held on the other. The separation miss is the most useful thing in the round.

## Addendum, 2026-09-03 — everything above ran on a coin

Asked afterwards whether the round was tested **on real data**. It was not. Every codec
figure in this file came from `Mul32` — uniform random bits, the maximum-entropy case, which
is not what a file looks like. The only file v5 read was v4's JSON record, for the pin.

**Two thirds of the round cannot care, and that is a statement about the constructions and
not an excuse.** Part 1's partition, picture, class sizes and every `worst(C, L)` figure, and
the whole of Part 2 — the floor, the linear family, the theorem, the periodicity lemma, the
construction and both searches — are counts over **cells**, not over **values**. No payload
can move them. The correction and miscorrection rates are the part that depends on the data,
and they now have real bytes under them: `eggso5 real`, over six repo files spanning 4.86 to
7.79 bits of byte entropy (markup, code, prose, CSS, an SVG, and a compressed PNG).

**One filed mechanism was wrong and the test that went looking for it said so.** I expected a
biased payload to *shrink* the decoder's candidate space, since every candidate passes
`in_bit(cells[i] − d)` and an all-zero square admits no `+1`. It does not shrink it: for a
binary cell exactly one of the two directions is representable **either way**, so the
representable count is `L` for every payload. The payload changes *which* cells are flippable
in which direction — hence which aliases a syndrome has — not how many.

| what real bytes did | result |
| --- | --- |
| the real-bytes round trip, `to_cells` → `to_bytes` | **6 of 6 exact**, including `og.png`. Until now this was only ever checked on random bytes |
| singles, and every **flagged** burst channel | **400/400 on all six corpora**, unmoved |
| every **blind** burst channel, corrected | **0 for every arm on every corpus** — so M1's pigeonhole holds on real bytes, as it must, being a counting argument |
| **the miscorrection counts** | **moved, in both directions, on 35 arm/channel/corpus cells** |

So the round's **M2 disclosure is a coin-specific number, not a property of the arms.** Some
lies vanish on real data — `idx3` on the full anti-diagonal drops from 3 of 400 to 0 on all
six corpora, `tape12` on the in-region burst from 2 to 0 on five of six — and some appear
where the coin reported none, including `cubic` on the anti-diagonal, the one channel where
Part 1 claimed the cubic arm "stops lying". **That claim is coin-specific and is withdrawn:**
on `index.html` and `og.png` the cubic arm miscorrects 1 of 400 on that channel.

**The worst case found, and its caveat, stated together.** `diag3` on the full anti-diagonal:
**6 of 400 on the coin against 45 of 400 on `favicon.svg`**, and 13 on `spec.md`. But that
channel's damage is *deterministic* — always the same 32 cells — so on a pool the effective
sample is the distinct square count, and `favicon.svg` holds **9** squares. Read it as **1 of
9 squares against the coin's 1.5%**, and `spec.md`'s 13 of 400 as roughly **4 of 119**. The
direction is consistent across corpora and the sample is small. The honest claim is that
**low-entropy payloads make this arm lie more often, and six files do not settle the size of
the effect.** `eggso5 real` prints the distinct-square count beside every row so no reader
takes 45/400 for a rate.

**Bar status is unchanged** — M2 asked that miscorrections be reported per arm and they were,
and B1–B5 and N1–N4 are payload-independent — but the round is more honest with this addendum
than without it, and the cubic arm's "stops lying" line is now a withdrawn claim.
