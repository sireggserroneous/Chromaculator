# eggSo v5 — Cayley's unfinished business, and the burst optimum

Not part of the site. The twentieth codec experiment and the sixth in the fold-native
lineage — [`eggSo-v0/`](../eggSo-v0/) used the fold's partition, [`eggSo-v1/`](../eggSo-v1/)
its symmetry, [`eggSo-v2/`](../eggSo-v2/) its alphabet's slack, [`eggSo-v3/`](../eggSo-v3/)
its radix and its scale, [`eggSo-v4/`](../eggSo-v4/) named what the fold **is**. This one
takes the two threads v4 left hanging. Kept in its own folder so it does not entangle with
`chronochromatic.org`, which claims none of this.

Rust, its own crate, empty `[dependencies]` — the policy every Rust round here holds. JSON
is hand-rolled in `src/json.rs`.

Built 2026-09-03 against [PREDICTIONS.md](PREDICTIONS.md), filed before a line of the round
was written.

## The verdict, first

**Twelve bars, twelve met. Five filed numbers missed, and the most useful thing in the
round is one of the misses.**

v4 discovered by accident that the statistic eggSo-v0 spent its entire verdict on — the
chance two random cells land in different classes — moves no error channel at all. This
round took that seriously in two directions, and both landed somewhere better than planned.

**Part 1.** Give every cell a complex coordinate and colour the grid by which root of
`z³ − 1` Newton's method reaches. Its bar was **the picture and the name**, filed that way
up front so the round could not be read as chasing a capability win it would not get. It
did not get one. What it got instead:

> A Newton fractal and a set of diagonal stripes have **the same separation to four decimal
> places — 0.6673** — and could not behave more differently on a burst: **12 cells in one
> class against 4** on a row, **11 against 32** on the anti-diagonal.

v4 showed that separation cannot tell `blocks` from `idx3`. That it cannot tell a **fractal**
from a **seam** is the strongest available statement that the number was never a figure of
merit. I filed the cubic arm's separation at 0.63–0.66, *below* `diag3`. It came in equal to
four decimals. That miss is the result.

**Part 2.** Optimise the figure of merit that actually matters, which had never been
optimised. It has an exact answer, and the answer is arithmetic:

> For a burst length `L` divisible by 3 and a run that fits across the grid, a three-class
> partition reaching the pigeonhole floor `⌈L/3⌉` on **all four** burst geometries exists
> **exactly when `3 | L/gcd(n, L)` and `3 | L/gcd(n−1, L)`**. Where it exists it is
> `class(j) = g(j mod L)` for a balanced `g` — a **block interleaver**, named and not by us.
>
> **Linearity is not the obstruction. Arithmetic is.**

The plan for this round predicted the opposite — that the linear family's failure was "an
algebraic coincidence over `(Z/3)²`, not an information bound", so a nonlinear partition
would exist and annealing would find one. Half of that is right and half is wrong, and the
round separates the halves rather than scoring itself on the average.

**Part 3.** Three claims in this lineage that rested on my reasoning became measurements.
One of them turned out to be a pigeonhole rather than a measurement, which is a better
answer than the one that was asked for.

## What failed, first

- **The cubic arm's class sizes.** Filed: unbalanced, no class within 5% of a third, class 0
  at 370–400 of 1024. Measured: **348 / 338 / 338**, nearest class **1.0%** off a third. The
  mechanism I named is real and self-cancelling — every band's first cell does sit at angle 0
  inside root 1's basin, but the rest of those same bands are correspondingly
  *under*-represented there. Residue +7, not +42. And it gets *more* balanced with width:
  15.6% off at n = 8, 7.4% at 16, 1.0% at 32.
- **The cubic arm's separation.** Filed below `diag3`. Measured **equal to four decimals**.
  See the verdict above; this is the miss worth having.
- **`diag3` on the in-region burst.** Filed 0 miscorrections. It lied **once in 400**.
- **The annealer.** Filed to reach the floor at every `n` where `3 ∤ L`. It reached the floor
  in **6 of 30** cases, and every one of those six was a case where the *linear* seed was
  already there. On **11 cases** a complete enumeration built a floor-reaching partition that
  **neither** annealing schedule found. The annealer contributed nothing to this round except
  the knowledge that it was the wrong instrument.
- **And the schedule was re-tuned.** PREDICTIONS.md filed one seed, one schedule, one budget,
  and said that if it were ever re-tuned this round would name the first configuration and
  the number it produced. It is named: schedule A, `T = 2.0 → 0.005`, **6 of 30 at the floor,
  mean acceptance 0.30%**, and on `(15,12)` it *lowered the energy while raising the worst
  case*. The diagnosis is arithmetic — a single-cell move touches up to `4L` windows so `|Δ|`
  runs to tens, and `e^−10/2 ≈ 0.7%` is exactly the measured acceptance — so schedule A was a
  greedy descent wearing an annealer's clothes. Schedule B makes one change, `T_hot = L`:
  **acceptance 8.72%, zero worst-case regressions, and the same 6 of 30**. Both tables are
  printed by `eggso5 optimum`. Neither is deleted.

## The pins — six, and one of them decides whether Part 1 is about anything

| pin | checked | mismatches |
| --- | --- | --- |
| **the copy vs v4's committed record** | **37 figures** | **0** |
| `region_of` vs `stalk.js`'s `regions()` | 22,139 | 0 |
| `arcs` vs `stalk.js`'s `arcs()` | 1,599 | 0 |
| **`hankel_k` vs `stalk.js`'s `cellOrder`** | **22,139** | **0** |
| the port vs eggSo-v0's structure | 6,153 | 0 |
| the port vs eggSo-v0's decisions | 600 | 0 |

Part 1's coordinate takes its **angle** from the site's own fill order. If that angle were
mine rather than the site's, the whole construction would be about nothing — so
`hankel_k` is checked against `cellOrder` itself, through node, cell for cell.

And **P1** is the price of this repo's copy-forward rule. Each round is a frozen record and
its own crate, so v5 copies v4's `fold`, `code`, `dynamics`, `seam`, `json` and `pin` rather
than depending on them; a path dependency would let v5's recorded numbers drift when v4
changes. The cost of copying is that a copy can drift *silently*, so `pin::v4_figures`
recomputes v4's headline figures with v5's copies and compares them against v4's committed
`measured-*.json`: 89,439 cells / 0 exceptions, v4's four pin counts, `diag3` at 341/342/341
and 0.6673, and `diag3`'s flagged burst sweep `200/200/200/200/200`. It needs no node.

## Part 1 — the degree-3 geometry

v4 placed the fold as the Julia set of `z ↦ z²` using `rho = 2^((r+c)−(n−1))` alone, because
`rho` is a modulus and a degree-2 map needs no more than one. Degree three needs the angle —
and the angle is **not free**, because the site already fixed it. `rho` hands one modulus to
the `arcs(n)[d]` cells of a band, and `stalk.js:102-110` already distinguishes them: it reads
each anti-diagonal "from the bottom-left corner upward". So

```
k(r, c) = min(n−1, r+c) − r                          the site's own Hankel position
z(r, c) = rho(r, c) · exp(2·pi·i · k / arcs(n)[r+c])
```

Band for the radius, Hankel walk for the angle, nothing invented. `z` is injective — within a
band the moduli agree and the angles are distinct, across bands the moduli are distinct powers
of two — and `|z| = rho` exactly.

Classify each cell by which root of `z³ − 1` Newton reaches. That is what the fold **would
have to be** if this geometry could carry three basins.

### The picture, which is half the bar

The grid coloured by basin, at n = 32, with `(r+c) mod 3` beside it. The left is legibly
**not a seam**; the right is what a seam looks like. That is Cayley's wall arriving on the
grid rather than in the plane.

```
      degree 3: z^3 - 1                     diag3: (r+c) mod 3
      ..#.ooooooo#....................      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      .o.###.ooooooooooooo#o#.o.......      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ..o.#####.ooooooooooooooooooo...      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      .#oo..######.ooooooooooooooo....      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      .#oo...o#######.oooooooooooo....      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      .#.oo....#########.ooooooo#o....      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      .##oo#....o##########.#o.#oo....      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      .##ooo.....o################....      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      .##.ooo......o#############.#...      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      .###ooo#......o############.....      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      .o##oooo#......##############...      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ..##.oooo........o##########o...      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ..###oooo#........##########o...      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ..###ooooo#.......###########...      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ..###.oooooo.....#############..      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ..####oooooo.....o###########...      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ..####oooooo#..o#.###########o..      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ..####.ooooooooooo.o#########o..      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ..#####ooooooooooo#.##########..      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ..o####ooooooooooooo.##########.      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ..#####.ooooooooooooo.########o.      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ..o####ooooooooooooooo.#######o.      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ...#####ooooooooooooooo.######..      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ..#####.oooooooooooooooo.######.      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ...####oooooooooooooooooo.#####.      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ...###o#oooooooooooooooooo.####o      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ...#####o..oooooooooooooooo.###o      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ...#.....o.o##oooooooooooooo.###      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ...............o.##oooooooooo.##      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
      ....................o##.oooooo.#      #.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.
      ..........................##ooo.      .o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o
      ................................      o#.o#.o#.o#.o#.o#.o#.o#.o#.o#.o#
```

### The honest verdict, with the loss stated before the numbers

Filed before building: it **cannot** beat `(r+c) mod 3`; it will be **worse**, because
**Fatou basins have interiors and interiors concentrate** — a burst landing inside a lobe
lands in one class, while a fractal boundary scatters only the minority of cells near it.

Worst cells in one class, over **every** placement of a 12-cell run (floor 4):

| arm | row | column | anti-diagonal | tape | full anti-diagonal |
| --- | --- | --- | --- | --- | --- |
| `diag3` | **4** | **4** | 12 | 5 | **32** |
| `cubic` | 12 | 12 | **10** | 12 | **11** |

The row burst is the filed loss and it landed at the maximum possible. The full
anti-diagonal is the one channel it **wins**, and by the mechanism that was filed: `diag3`'s
level set *is* that band, so a seam puts all 32 cells in one class and a fractal boundary
cannot. 11 is exactly `⌈32/3⌉`.

On the codec itself, corrected of 400, the anti-diagonal channel is **0 for both** — 32
cells over 3 classes is at least 11 per class and this decoder searches to depth 2. So the
cubic arm's win is on the **spread** and not on the channel, and the round says so rather
than banking a correction it did not make. The one thing it does buy: on that channel
`diag3` miscorrects 6 of 400 and the cubic arm **0**. Where it wins the spread, it also
stops lying.

### The name

**The basin decomposition of a cubic Newton map** — a [Newton
fractal](https://en.wikipedia.org/wiki/Newton_fractal). Cayley 1879 for the question, Julia
1918 and Fatou 1919–20 for the theory, the computer era for the picture. v4's lineage audit
found zero prior mentions of any of it across nineteen experiments.

**Part 1 adds no site claim**, deliberately. A Newton fractal is not the site's geometry; it
is what the site's geometry would have to *become* at degree three, and that distinction is
the point of the exercise.

## Part 2 — the burst optimum

For a partition `C` and a burst length `L`,

```
worst(C, L) = max over bursts B of length L  of  max over classes k of  |B ∩ k|
```

over four geometries: a **row**, a **column**, an **anti-diagonal** (tape step `n−1`), and
the row-major **tape**, which wraps at row boundaries and is what a contiguous storage wound
actually looks like. `L` cells over three classes always give some class `⌈L/3⌉`, so that is
the floor, and it is asserted against every measurement the round takes.

### The linear family, and where the ground's own conditions stop being true

The nine `(a,b)` of `C(r,c) = (a·r + b·c) mod 3` were re-derived by the suite rather than
quoted. The four shatter conditions — row iff `b ≠ 0`, column iff `a ≠ 0`, anti-diagonal iff
`a ≠ b`, tape iff `b ≠ 0` **and** `a ≡ b·n` — were computed during planning at
`L = 6, 9, 12, 18`, all divisible by 3. This round also ran `L = 8` and `11`:

| range | cases | violations |
| --- | --- | --- |
| `3 \| L` | 927 | **0** — exact |
| `3 ∤ L` | 504 | **28 disagreements** |

Every one of the 28 is the **tape** condition being *sufficient but not necessary*, and the
mechanism is slack: with `3 | L` a window at the floor is `L/3` three times over with no
room, but with `3 ∤ L` it is `L = 3⌈L/3⌉ − s` for `s ∈ {1,2}`, so a window can be
`(f, f, f−s)` and the phase slip's extra cell has somewhere to go. The ground is confirmed
where it was computed, and its boundary is now known.

The exhibit, at n = 32, L = 12 (floor 4):

| `(a,b)` | row | col | diag | tape | at the floor? |
| --- | --- | --- | --- | --- | --- |
| (0,0) | 12 | 12 | 12 | 12 | no |
| (0,1) | 4 | 12 | 4 | 5 | no |
| (1,1) = `diag3` | 4 | 4 | **12** | 5 | no |
| (1,2) | 4 | 4 | 4 | 4 | **YES** |
| (2,1) = `idx3` | 4 | 4 | 4 | 4 | **YES** |
| (2,2) | 4 | 4 | 12 | 5 | no |

**The theorem**, re-derived over n = 2..64: the system `{a ≠ 0, b ≠ 0, a ≠ b, a ≡ b·n}` over
`(Z/3)²` has **no solution at `n ≡ 0 (mod 3)`** (it forces `a ≡ 0`), **none at `n ≡ 1`** (it
forces `a ≡ b`), and **exactly two at `n ≡ 2`**: `(1,2)` and `(2,1)`. Since
`j mod 3 ≡ ((n mod 3)·r + c) mod 3` over every cell, v4's `idx3` **is** the arm `(2,1)`, and
its clean sweep at n = 32 was optimal by accident of `32 ≡ 2 (mod 3)` — exactly as v4's
`seam.rs` refused to generalise.

### The periodicity lemma, which is the round's real answer

Filed in PREDICTIONS.md as an amendment **against the plan's own prediction**, before any of
this ran.

> Suppose `3 | L`. Then `⌈L/3⌉ = L/3`, and a tape window whose three class counts are each at
> most `L/3` has them all **exactly** `L/3` — there is no slack anywhere. Slide the window one
> cell: it loses `class(j)` and gains `class(j+L)`, and both windows are exactly balanced, so
> `class(j) = class(j+L)`. **The tape is forced to be `L`-periodic**, `class(j) = g(j mod L)`.
>
> Then, with no reference to linearity at all: a **row** window is all of `Z/L` once and is
> satisfied for free; a **column** window steps by `n`, walking the coset generated by
> `gcd(n, L)` and covering it exactly `gcd(n,L)` times, so balance requires
> `3 | L/gcd(n, L)`; an **anti-diagonal** steps by `n−1`, so it requires `3 | L/gcd(n−1, L)`.

| check | result |
| --- | --- |
| the lemma vs exhaustive enumeration, n = 3..6 | **5 of 5 agree** |
| the lemma vs exhaustive enumeration, n = 15, 16, 30, 31, 33 | **20 of 20 agree**, in 122 ms |
| the construction where the lemma permits | **built and verified at the floor every time** |

So the obstruction is **not** a linearity artefact, which is what the plan supposed. It is
arithmetic in the grid's width, and it binds every partition of every shape.

### The search, and what it was actually good for

Exact search: depth-first over **canonical** partitions — cells in row-major order, class
labels restricted so first occurrences run `0, 1, 2`, which quotients the 6-fold relabelling
symmetry exactly — pruned the instant any window's count exceeds the floor. Counts only grow,
so the prune is sound and the enumeration is complete. Node cap 200,000,000; a case that
exhausts it reports **INCONCLUSIVE** and never "no solution".

| | result |
| --- | --- |
| cases settled outright, of 30 | **27** |
| cases where the enumeration built a floor-reaching partition **neither** annealing schedule found | **11** |
| INCONCLUSIVE at the node cap | 3 — `(30,8)`, `(33,8)`, `(33,11)` |

The plan predicted existence *and* that annealing would find it. Existence holds where
`3 ∤ L`, including at `n ≡ 0 (mod 3)` where no linear partition reaches the floor —
exhibited at `(15,8)`, `(15,11)`, `(30,11)`, and at `(4,4) (5,4) (5,5) (6,4) (6,5)`. Annealing
found **none** of it, at either temperature. The honest reading is not that the temperature
was wrong; it is that **a local search is the wrong instrument for this objective and a
complete enumeration is the right one** — the constraint is tight, so solutions are sparse
but reachable by propagation, and the enumeration found in milliseconds what 2,000,000
annealing moves at two schedules could not find at all.

One more limit worth naming: the annealer's energy is the **total** excess over the floor and
the objective is a **maximum**. The two agree only at zero, so a run can lower the energy and
raise the worst case — and schedule A did, at `(15,12)`. Every row prints the seed's worst
case beside the best found so this is visible rather than inferred.

### The answer

**For `3 | L` and a run that fits across the grid:** a partition reaching the floor on all
four geometries exists **exactly when `3 | L/gcd(n, L)` and `3 | L/gcd(n−1, L)`**. Necessity
is the lemma; sufficiency is the construction `class(j) = g(j mod L)` with `g` balanced on
every coset mod `gcd(n, L)` and mod `gcd(n−1, L)`. On the tape that is a [block
interleaver](https://en.wikipedia.org/wiki/Burst_error-correcting_code#Interleaved_codes)'s
read-out, so the name is prior art — as predicted — and this series counts landing on prior
art as a legitimate result.

And `g` **need not be linear**: when 3 divides `gcd(n, L)` a linear `g` is constant on a coset
and cannot do it. That is the one place in this round where nonlinearity earns something.

**For `3 ∤ L`:** the floor has slack, no obstruction is known, and partitions at the floor
exist at `n ≡ 0 (mod 3)` where no linear one does. Three cases are open.

**Linearity is not the obstruction. Arithmetic is, and only when `3 | L`.**

## Part 3 — measure rather than argue

Vladimir's rule for the round: *"Also lets measure rather than argue."* Three claims in this
lineage rested on my reasoning. Each became a measurement.

### M1 — v1(b)'s unique row survives, and it is a pigeonhole

[`eggSo-v1/`](../eggSo-v1/) pays **103% per data bit** for its mirror arm, justified by one
channel no other column could take: a 12-cell **unflagged in-region** burst. That claim was
mine, argued rather than measured. Measured now, against all nine arms:
**0 corrected, every one of them.**

And it is not a measurement at all, which is a better answer than the one asked for: 12 cells
over 3 classes puts at least 4 in some class, and this decoder searches to depth 2 per class.
**No three-class partition can take this channel, at any width, ever.** v1(b) keeps its
justification and v1's README is not edited. v1's own suite, re-run here, confirms the other
side: `B4 MET 100.00% unflagged in-region burst`.

What the channel *does* discriminate is which arms **lie** on it. Filed: `fold` and `blocks`
miscorrect, `diag3` and `idx3` do not. Landed: `fold` 10 of 400, `blocks` 4, `idx3` **0**, and
`diag3` **1** — a partial miss.

### M2 — every arm that lied, per channel

`eggso5 arms` prints all of it: **24 arm/channel pairs** carry a miscorrection, none of them
on any flagged channel. The nine arms at n = 32, all paying the same 48 check bits so every
difference is pure geometry:

| arm | classes | separation |
| --- | --- | --- |
| `fold` | 496 / 32 / 496 | 0.5303 |
| `diag3` = `(r+c) mod 3` | 341 / 342 / 341 | 0.6673 |
| `idx3` = `j mod 3` | 342 / 341 / 341 | 0.6673 |
| `blocks` | 341 / 341 / 342 | 0.6673 |
| `seam128` | 435 / 124 / 465 | 0.5993 |
| **`cubic`**, the basin decomposition | **348 / 338 / 338** | **0.6673** |
| **`tape12`** = `g(j mod 12)`, Part 2's interleaver | 344 / 340 / 340 | 0.6673 |

Five of those seven share the separation figure to four decimals: two different stripe
patterns, a contiguous block split, a Newton fractal, and a block interleaver. It is the
same statistic for all five and it distinguishes none of them.

### And why none of it generalises across `n`

`idx3`'s clean sweep at n = 32 is the arm `(2,1)`; at n = 33 it degenerates to `cols3`.
18-cell **flagged** bursts on each of the four geometries at n = 33, corrected of 200, with
(mean / worst cells in one class):

| channel | `diag3` | `idx3` | `blocks` | `cubic` |
| --- | --- | --- | --- | --- |
| row | 200 (6.0/6) | 200 (6.0/6) | 0 (18.0/18) | 177 (11.8/18) |
| column | 200 (6.0/6) | **0 (18.0/18)** | 200 (10.5/11) | 176 (11.9/18) |
| anti-diagonal | **0 (18.0/18)** | 200 (6.0/6) | 200 (10.4/11) | 200 (7.6/10) |
| tape | 200 (6.4/7) | 200 (6.0/6) | 7 (17.8/18) | 184 (11.2/18) |

The gap between **mean** and **worst** is the whole mechanism. The erasure decoder refuses
above 16 flagged in one class, so an arm whose *mean* is already 18 — `idx3` on a column here,
`diag3` on an anti-diagonal, which is its level set — is detected on every trial and corrects
nothing. The cubic arm touches 18 at its worst and sits near the floor on average, so it
corrects most trials and loses the rest: **a fractal has no periodic bad case, only a rare
one.** That difference is exactly what `worst(C, L)` optimises, and it is why the objective is
stated as a maximum and not an average.

## Running it

```
cargo build --release
cargo test                                    # 54 tests
cargo clippy --all-targets -- -D warnings     # clean, no suppressions

cargo run --release -- pin        # the six pins; SKIPPED loudly if node is absent
cargo run --release -- cubic      # Part 1: the picture, the classes, the channels
cargo run --release -- optimum    # Part 2: the floor, the theorem, the lemma, both searches
cargo run --release -- arms       # Part 3: every arm on every channel
cargo run --release -- audit      # all of it, ~24s, writes every measured-*.json
```

`--full` widens the linear sweep and the pictures. It deliberately does **not** widen the
search: the seed, the schedule and the budget are fixed in `src/optimum.rs` so that a search
cannot be quietly retried until it wins.

## Files

```
Cargo.toml        name = "eggso5", edition 2021, NO dependencies
PREDICTIONS.md    filed first, with the measured column filled in afterwards
src/lib.rs        pub mod declarations
src/main.rs       pin | cubic | optimum | arms | audit
src/fold.rs       carried from v4 unchanged
src/code.rs       carried from v4; `in_bit` made pub
src/json.rs       carried from v4 unchanged
src/dynamics.rs   carried from v4, plus the cell -> complex coordinate
src/seam.rs       carried from v4, plus three burst geometries, a worst-case field,
                  an Arm that can be a table, and the silent-drop trap fixed
src/pin.rs        carried from v4, plus the cellOrder pin and the v4-figures pin
src/cubic.rs      NEW: the degree-3 partition and its picture
src/optimum.rs    NEW: the bound, the linear family, the lemma, the construction, the search
measured-*.json   what the binary wrote
```

**Copy forward rather than depend**, and the two traps v4 left in the copied code are named
in the source rather than quietly fixed: `dynamics::ascii` divided by `w − 1` and panicked
with no message at `w == 1`, and `seam::burst_breaking_point` silently dropped any length
`≥ n`, which in this round would have hidden a Part 2 row rather than failing it. Both now
say which caller was wrong.

## What this is and is not

- **It is not a compressor and not armour.** Settled and recorded elsewhere in this repo: the
  site's constructions are bijections, and codegg's armour is at the 1.01× floor.
- **The degree-3 arm is not a capability.** Its bar was the picture and the name, filed that
  way before building, and it lost every channel it was predicted to lose.
- **The site gains one line, not two.** [`inspirations.html`](../inspirations.html)'s fold
  section gains a single item, for Part 2's interleaver name and the arithmetic obstruction.
  Part 1 adds nothing to the site, because a Newton fractal is not the site's geometry.
- **Three cases are open**, and are printed as INCONCLUSIVE rather than as impossibility.
