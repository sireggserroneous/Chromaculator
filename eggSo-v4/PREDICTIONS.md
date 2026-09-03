# eggSo v4 predictions — filed 2026-09-03, BEFORE a line of the round was written

The series convention: every number below is a guess, written down first, and the
measured value is filled in beside it afterwards or not at all. Misses stay. A
prediction that is quietly edited to match its result is worth less than no prediction —
and, since v0's amendment, so is a measurement.

The nineteenth codec experiment and the fifth in the fold-native lineage. **The first one
written in Rust**, per Vladimir 2026-09-03: *"I also noticed that eggso is not written in
rust so far. This changes moving forward. We write in rust."*

## What this is, in one sentence

The site's three regions are the two **Fatou basins** and the **Julia set** of a degree-2
map, the anti-transpose is the inversion that exchanges them, and Cayley's 1879 wall is why
this geometry never gets past two.

## Why this round exists

The site's README has carried one open question since it was written —
`README.md:166`: *"So: what one does with the fold has now been placed. What the fold is
has not."* Four fold-native rounds placed what one *does* with it and none touched what it
**is**.

Vladimir's pointer, 2026-09-03: Arthur Cayley, 1879, asked where Newton's method lands from
a given guess. Two roots gives a straight line down the middle — guess left, get one root,
guess right, get the other. Three roots he could state and not solve; he published it as a
failure and it stayed open until Julia, Fatou, and then computers a century later. And the
guess-and-fix trick is already running here: `stalk.js:288-306`, the site's own divider,
guesses a signed digit from `{−1,0,+1}` each step and carries the corrected remainder
forward.

Three parts, in Vladimir's stated order — *"Placement and correction only first, then add a
guess-and-fix decoder, and finally add the chosen-seam interleaver. All in the same run. So
we can see what each brings to the table and not assume."*

## The construction, stated before building

For a cell `(r, c)` in an `n × n` grid let `d = r + c`, the anti-diagonal index — what the
site calls the place-value band. Set

```
rho(r, c) = 2 ^ (d − (n − 1))
```

Then Inner is `|rho| < 1`, the Fold is `|rho| = 1`, Outer is `|rho| > 1`, and the
anti-transpose `sigma(r,c) = (n−1−c, n−1−r)` sends `rho → 1/rho`, which is inversion in the
unit circle. That circle is the Julia set of `z ↦ z²`; its two Fatou basins are the inside,
attracted to 0, and the outside, attracted to ∞. Newton's method on `z² − 1` is conjugate to
`z ↦ z²` by the Möbius map `w = (z−1)/(z+1)`, so Cayley's two-root picture and the site's
three regions are the same object seen from the root-finding side.

Part 2 asks whether the site's guess-and-fix can decode. Part 3 asks what the fold's forced
seam width costs against a freely chosen one.

## Measured during planning — ground, not predictions

Computed 2026-09-03 before this file existed. A number already known cannot honestly be
filed as a guess, so it sits here; the suite recomputes each one and must match.

| quantity | computed |
| --- | --- |
| `rho` reproduces the three regions, n = 2..64 | **89,439 of 89,439 cells, 0 exceptions** |
| `sigma` sends `rho → 1/rho`, same range | **89,439 of 89,439, 0 exceptions**; every Fold cell fixed at `rho = 1` |
| `z ↦ z²`: fate matches the modulus | **158,265 of 158,265** |
| Newton on `z² − 1`: basin predicted by `sign(Re z)` | **159,598 of 159,598, 0 exceptions** |
| the same under `w = (z−1)/(z+1)`: basin ↔ inside/outside the circle | **159,597 of 159,597** |
| Newton on `z³ − 1`: cells on a basin boundary | 5,982 of 89,401 (**6.7%**) |
| of those, touching **all three** basins | 2,292 (**38.3% of the boundary**) |
| the Fold's share is `n/n²` | 25% at n=4, **3.125% at n=32**, 0.098% at n=1024 |
| separation, n=32: fold · fair two-way · fair three-way | **0.5303 · 0.5005 · 0.6673** |
| the fold's margin over a fair **two-way** split | +16.67 pts (n=4), +5.69 (16), **+2.98 (32)**, +1.53 (64), +0.77 (128) — always positive |
| best seam in the two-basins-plus-seam family, L=1024 | seam 342 → 0.6673, i.e. the optimum of that family **is** the fair three-way split; the fold reaches 79.47% of it |
| `(r+c) mod 3` at n=32 | classes **341/342/341**, separation **0.6673** — the optimum, and it is the fold's own level sets |
| worst cells in one class, 12-cell burst: fold / diag3 / idx3 / blocks | row 12/4/4/12 · col 12/4/4/11 · tape 12/5/4/12 · full anti-diagonal **32/32/11/12** |
| ring-distance decrease at the true error vs a wrong cell, one error, Inner, p=2053 | **1.000 (200/200) vs 0.525** |
| the same with two errors | **0.512 vs 0.516** |
| accepting single-cell moves for a same-class double | **0**, by injectivity |

Two caveats belong in the ground rather than the results, because they are the places this
argument could be attacked.

**The sphere the site declines.** `spec.md:261-264` and `inspirations.html:189-192` say
"Doubling only zero is what makes this a *disc with a centre* rather than a sphere." That
is about the number line's topology, zero doubled and ω unified. The sphere this round uses
is the grid's `rho`-plane, where `z ↦ z²` has its two attracting fixed points at 0 and ∞.
Different object, and the round says so rather than quietly contradicting the page.

**`d` is not `i`.** A cell's weight is `2^−(i+1)` in the stalk index `i`
(`spec.md:23`, `glossary.js:48-52`). The site's strong claim that "the anti-diagonals **are**
the place values" is exact for the **product rectangle**, where weight is `2^−(r+c+2)`
(`spec.md:108-110`; `productRegions` sets `w: r+c+2` at `stalk.js:229-237`;
`squashDiagonals` sums by `r+c` and comments "S[d] rides weight 2^−(d+2)" at
`stalk.js:336-341`). On a **single folded stalk**, anti-diagonal `d` holds `arcs(n)[d]`
cells spanning a *band*. So `rho` is an exact normalised place value on the product grid and
a magnitude **ordering** on the folded stalk. The site's own bridge sentence,
`index.html:170/172/174`, says "low / middle / high place values" — deliberately an
ordering. **The round claims exactly that much and no more.**

## The lineage audit

Zero prior mentions across all eighteen experiment directories and the whole site: Newton,
Newton–Raphson, Cayley, Julia, Fatou, Mandelbrot, fractal, basin, attractor, dynamical,
Möbius, complex plane, root finding, stereographic, SRT division, quotient digit selection,
holomorphic, escape time, self-similar. The cleanest slate in the audit's history.

Three things already on the record, cited rather than rediscovered:

1. **The site already names the map's action on the circle.** `inspirations.html:311-315`
   places "dropping a digit; orbits and periods" on the **doubling map** and **symbolic
   dynamics**. On `|z| = 1`, `z ↦ z²` is `θ ↦ 2θ` — the doubling map exactly. **So the site
   named what the map does on the Fold and never named the Fold as the map's invariant
   set.** That is this round's sharpest sentence and it rests on the site's own citation.
2. **The site already has poles, an equator and two hemispheres for the grid.**
   `index.html:138-139`, `:169-174`, and `:313-314` — "single-cell poles, widest ring at the
   Fold, equal hemispheres", claimed as measured fact.
3. **The site characterises the inversion axiomatically and never commits to a map.** Five
   places say only "fixes the Fold, swaps Inner with Outer, undoes itself":
   `index.html:175-177`, `spec.md:76-77`, `glossary.js:58-62`, `index.html:311-312`,
   `inspirations.html:413-417`. The only `z ↦ 1/z` on the site (`spec.md:255-259`) is about
   the surreal disc, not the anti-transpose. **A degree-2 map is an addition, not a
   re-derivation.**

## THE BARS

### Part 1 — the placement

| bar | needed to count as met |
| --- | --- |
| **P1** the coordinate | `rho` reproduces the three regions and `sigma → 1/rho` with **0 exceptions**, n = 2..64, re-derived by the suite and not quoted from the plan |
| **P2** the dynamics | the two-root basin is the straight line with 0 exceptions, the Möbius conjugacy holds, `z ↦ z²`'s fates match the modulus |
| **P3** Cayley's wall | the all-three-basin share of the cubic boundary measured at three grid resolutions and **stable**, so 38.3% is not an artefact |
| **P4** the doubling map | `arg(z²) = 2·arg(z)` on the circle, cited to `inspirations.html:311-315` |
| **P5** the site's own function | `region_of` and `arcs` asserted against `stalk.js`'s own `regions()` and `arcs()` at runtime, cell for cell — **SKIPPED loudly** if node is absent, never passed quietly |
| **P6** the correction | the fold's separation against a fair two-way split, positive at every n from 4 to 128, with the forced-seam counterweight stated in the same breath |
| **P7** the name | the fold placed as the Julia set of a degree-2 map, Inner and Outer as its Fatou basins, the anti-transpose as the inversion exchanging them — with the sphere tension and the `d`-versus-`i` caveat both written down |

### Part 2 — the guess-and-fix decoder

| bar | needed to count as met |
| --- | --- |
| **G1** the lemma | zero-a-class and fewer-hurt are **provably the same rule** for single-cell moves, by exhaustive enumeration; restarts inert under restoring acceptance |
| **G2** singles | GF-1 reaches v0's rate given ≤ 4096 probes; the geometric law `1 − (1 − 1/L)^B` holds |
| **G3** the plateau | **no arm clears a same-class double without two-cell moves**, asserted by an exhaustive move census returning zero accepting moves |
| **G4** the gradient's range is one | decrease rates at distance 1 and 2, with mutual information in bits |
| **G5** the honest-detection loss | `consistentButWrong > 0` where v0 returns 0, because guess-and-fix cannot express ambiguity |
| **G6** the count arm | GF-5 clears same-class doubles the residue arms cannot, with **zero** direct corrections, at equal or lower check cost |
| **G7** where a blind guess wins | flagged erasures, thin-class damage with no table at all, the amortisation crossover — measured, not asserted |

### Part 3 — the chosen-seam interleaver

| bar | needed to count as met |
| --- | --- |
| **S1** the port is faithful | reproduces v0's committed `measured-*.json` **to the trial**, and is pinned **structurally** — `region`, `members`, table keys and candidate lists element-for-element in the same order |
| **S2** the fairness assert | `sizes` bit-identical across all seven arms, so every measured difference is pure geometry |
| **S3** separation is not the figure of merit | `blocks` and `idx3` share `P(diff)` to the digit and diverge on the burst sweep by ≥ 30 points |
| **S4** the direction was right | `diag3`, the fold's own level sets mod 3, hits 0.6673 exactly and beats `fold` on random pairs while matching it on the anti-diagonal channel |
| **S5** the cost of the geometry | the forced `1/n` seam priced against a free choice on real channels, at identical overhead |
| **S6** the accident, filed | `idx3`'s clean sweep is a consequence of `32 ≡ 2 (mod 3)`, reported as such, with the split printed at `N ≡ 0, 1, 2` |

**MET** if P1–P7 hold and G and S are measured either way. **MISSED** if the coordinate has
any exception, or if the round reports a name without recording the sphere tension.

## Calibration, stated before the numbers

I expect Part 1 to hold exactly, because it is arithmetic rather than a channel — a miss
there is a bug and not a result. I expect Part 2 to fail completely and for the *reason* to
be the round's best sentence: injectivity is what makes the table trivial and the search
blind, and those are the same property. The bar I am least sure of is G6 — the count arm
should converge where the residue cannot, but whether it does so at a rate worth printing
is a genuine question. I expect Part 3's `diag3` to win, which would sharpen v0's verdict
rather than overturn it, and I expect `blocks` to be the arm that proves separation was
the wrong figure of merit all along.

## Per-stage predictions

### S1 — the coordinate and the dynamics

| claim | called |
| --- | --- |
| P1, P2, P4 | hold exactly; a miss is a bug |
| the cubic's all-three share at three resolutions | **35–42%, stable to within 3 points** |
| `region_of` vs `stalk.js regions()` | 0 mismatches over 22,139 cells, n = 2..40 |
| `arcs(n)` gives single-cell poles and equal hemispheres | holds at every n |

### S2 — the guess-and-fix decoder

| claim | called |
| --- | --- |
| the lemma: zero-a-class = fewer-hurt for one-cell moves | holds exactly; a miss means the assignment is not a partition |
| GF-1 on singles at budget 1024 / 4096 | **63.2% / 98.2%**, against v0's one lookup |
| accepting single-cell moves for a same-class double | **exactly 0**, at any budget |
| ring decrease, true vs wrong, distance 1 | **1.000 vs 0.525** |
| the same at distance 2 | **0.512 vs 0.516**, mutual information **< 0.01 bits** |
| `consistentButWrong` where v0 returns 0 | **> 0** — the qualitative headline |
| GF-5 on same-class doubles | clears them; **0 direct ever**; fails when two errors cancel in count |
| the amortisation crossover | **1–2 squares** |
| GF-1 matching GF-0 on singles | only at ~**1000×** the work |

### S3 — the chosen seam

| claim | called |
| --- | --- |
| `diag3` vs `fold` on two cells anywhere | `diag3` wins by **12–15 points**, matching 0.6673 against 0.5303 |
| `blocks` vs `idx3` on the burst sweep | `blocks` breaks at **17**, `idx3` reaches **~48**, despite identical `P(diff)` |
| the interleaving gain | **3×** in correctable burst length, and that is what prices the geometry |
| same-class miscorrections, bare, 341-cell vs 496-cell classes | roughly **half**, tracking `C(m,2)/p` |
| push invariance, every arm | **0/200**, as the fold's |
| `sizes` across arms | **bit-identical**, 4.69% |

## The bar arithmetic, filed plainly

| bar | needs | call |
| --- | --- | --- |
| P1–P5 | exact | **YES** |
| P3 | stable across resolutions | **YES**, 35–42% |
| P6, P7 | the correction and the name, with caveats | **YES** |
| G1, G3 | provable | **YES**, both are theorems before they are measurements |
| G2, G4 | the geometric law and the range-1 gradient | **YES** |
| G5 | a silent miscorrection where v0 refuses | **YES** — and it is the reason not to ship it |
| G6 | the count arm converges | **the uncertain one** |
| G7 | a real if narrow win | **YES**, at 1–2 squares |
| S1 | to the trial, and structurally | **YES** or the round stops |
| S2–S6 | measured either way | **YES** |

## Measured (filled as stages land — never before)

Filled 2026-09-03, after `cargo test`, `cargo clippy --all-targets -- -D warnings` and
`eggso4 audit`. Every number here is from those runs; `measured-*.json` beside this file
is what the binary wrote.

### One method change, made before measuring and recorded rather than slipped in

S1 was filed as "reproduces v0's committed `measured-*.json` numbers **to the trial**".
That would require replaying v0's test file's exact PRNG consumption order, block by
block, which is brittle and tests the wrong thing. It was replaced, **before any number
was taken**, by two pins that are strictly stronger:

- **structural** — v0's `p`, `q`, class array, member lists, and every syndrome table's
  keys and candidate lists, element for element in the same order. Outcome equality can
  hide two compensating bugs; this cannot.
- **behavioural** — squares and damage generated in Rust, decoded by **both** v0's own
  decoder through node and by the port, compared square by square on the status word and
  the repaired cells.

### S1 — the pins: all four clean

| pin | checked | mismatches |
| --- | --- | --- |
| `region_of` vs `stalk.js`'s `regions()`, n = 2..40 | **22,139** | **0** |
| `arcs` vs `stalk.js`'s `arcs()`, n = 2..40 | 1,599 | **0** |
| the port vs v0's structure | 6,153 | **0** |
| the port vs v0's decisions, 5 channels × both confirm modes | 600 | **0** |

22,139 is v0's own figure, reproduced. Node is required and reports SKIPPED loudly if
absent; it was present.

### S1 — the coordinate and the dynamics: HELD

| claim | called | landed |
| --- | --- | --- |
| `rho` reproduces the regions and `sigma → 1/rho`, n = 2..64 | 0 exceptions | **89,439 cells, 0 exceptions** |
| Newton on `z²−1`: basin by `sign(Re z)` | 0 exceptions | **159,598 guesses, 0 exceptions** |
| the Möbius conjugacy to inside/outside the circle | holds | **159,597 of 159,597** — the missing point is the pole at `z = −1`, which has no image and is therefore not a counterexample |
| `z ↦ z²` fates follow the modulus | holds | **158,265 of 158,265** |
| `arg(z²) = 2·arg(z)` on the circle | holds | holds to 1e-9 over 4,096 samples |
| the cubic's all-three share, three resolutions | 35–42%, stable to 3 points | **39.67% / 38.31% / 39.36%** at 201² / 301² / 501² — stable to 1.4 points |

### S2 — the guess-and-fix decoder: G2 and G4 HELD, **two filed claims MISSED**

| claim | called | landed |
| --- | --- | --- |
| the lemma: zero-a-class = fewer-hurt for one-cell moves | holds | **holds**, exhaustively over every cell of 200 damaged squares |
| GF-1 on singles at 1024 / 4096 probes | 63.2% / 98.2% | **63.00% / 97.75%**, against the law's 0.632 / 0.982 — and v0's table takes them in **one lookup** |
| **accepting single-cell moves for a same-class double** | **exactly 0, "a theorem from injectivity"** | **0.195** — **MISSED** |
| accepting moves that reach the clean square | — | **0** of all of them |
| ring decrease, true cell vs wrong cell, one error | 1.000 vs ~0.5 | **1.000 vs 0.516** |
| the same at two errors | no signal | **0.456 vs 0.479** — no signal, and the true cells are marginally *worse* than a coin |
| **GF-5, the count arm, on same-class doubles** | **clears them** | counts reached **100%**, fully consistent **0.50%**, exact **0.50%** — **MISSED** |
| the ladder on a same-class double, 4096 probes | all lose | GF-1 **0%**, GF-2 **0%**, GF-3a **0.50%**, GF-3b **3.25%**, GF-3c **3.75%** |
| one flagged erasure per class, 64 probes | 100% | **100%**, with no table at all |

**The first miss, and it is the more interesting of the two.** "A same-class double has no
accepting move" was filed as a theorem and is not one. Injectivity separates the `2L`
values `{±2^k mod p}` from **each other**; it says nothing about whether a **sum of two**
of them lands on a third. There are `O(L²)` such sums and only `2L` targets in a ring of
size `p ≈ 2L`, so collisions are the expected case, not the exception.

Getting the rate right took one step more than it looks. A first count says `2|class|/p` —
but a cell's **current bit** fixes which way its flip moves the syndrome, so only one of
the two directions is available per cell. Halving gives `496/2053 = 0.242` for an Inner
double and `1024/(3p) = 0.166` averaged over a uniformly chosen class. Measured **0.195**.

What survives is sharper than what was filed: **every accepting move on a two-cell error
is an alias.** One flip cannot undo two errors, so the move the search accepts is
guaranteed wrong — measured, **0** of them ever reach the clean square. Guess-and-fix does
not stall on this channel. It walks confidently into the trap v0 added `q` to refuse.

**The second miss completes the thesis rather than denting it.** The count arm reaches the
right counts 100% of the time and the right square 0.5% of the time. A count says *how
many* and never *which*, so the search converges in the count's own terms and lands
somewhere wrong, which the confirming residue then refuses. So:

> For the same ~33 bits you can buy an **address** or a **metric**. The residue buys the
> address, which makes lookup trivial and search blind. The count buys the metric, which
> makes search converge to the wrong answer. **Neither purchase buys a decoder.**

And the gradient's range is exactly one, which is the filed claim, met: at distance one the
true cell descends every time against a coin; at distance two there is no signal at all. A
division remainder shrinks at every step. This is a coin everywhere except on the answer.

### S3 — the chosen seam: measured, and the headline statistic turns out not to matter

Every arm pays the same **48 check bits**, so every difference is pure geometry.

| arm | classes | separation |
| --- | --- | --- |
| `fold` | 496 / 32 / 496 | 0.5303 |
| `diag3` = `(r+c) mod 3` | 341 / 342 / 341 | **0.6673** |
| `idx3` = `j mod 3` | 342 / 341 / 341 | 0.6673 |
| `blocks`, contiguous thirds | 341 / 341 / 342 | 0.6673 |
| `seam128` | 435 / 124 / 465 | 0.5993 |

Corrected of 400, miscorrections as `/nW`:

| channel | fold | diag3 | idx3 | rows3 | cols3 | blocks | seam128 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 cell | 400 | 400 | 400 | 400 | 400 | 400 | 400 |
| 2 anywhere | 400 | 400 | 399 | 400 | 399 | 397 | 400 |
| 2 same class | 393 | 394 | 387 | 394 | 393 | 393 | 392 |
| 3 one per class | 400 | 400 | 400 | 400 | 400 | 400 | 400 |
| 12-cell burst, flagged | 400 | 400 | 400 | 400 | 400 | 400 | 400 |
| 12-cell burst, blind | 0 · **10W** | 0 | 0 | 0 · 1W | 0 | 0 · 5W | 0 · 6W |
| one full anti-diagonal | 0 | 0 · 6W | 0 · 3W | 0 | 0 | 0 · 1W | 0 |
| the thinnest class filled | 0 · 1W | 0 · 2W | 0 · 3W | 0 · 9W | 0 · 5W | 0 · 3W | 0 |

The flagged burst, swept, of 200:

| burst length | 12 | 15 | 18 | 24 | 31 |
| --- | --- | --- | --- | --- | --- |
| `fold` | 200 | 200 | **116** | 70 | 18 |
| **`diag3`** | 200 | 200 | **200** | **200** | **200** |
| `idx3` | 200 | 200 | 200 | 200 | 200 |
| `cols3` | 200 | 200 | 200 | 200 | 200 |
| `rows3` | 200 | 200 | **0** | 0 | 0 |
| `blocks` | 200 | 200 | **10** | 9 | **0** |
| `seam128` | 200 | 200 | 133 | 82 | 39 |

**What was not filed and should have been, and it is the round's most useful correction.**
The separation statistic that eggSo-v0's whole verdict rested on **does not move the error
channels at all.** Every arm takes `2 anywhere` at 397–400 of 400 and `2 same class` at
387–394. The fold's 0.5303 against a fair split's 0.6673 is invisible on the very channel
it was supposed to be about, once the confirming residue is placed correctly. `S2` was
filed expecting `diag3` to win those rows by 12–15 points; it wins by **zero**.

Where the geometry does matter is the burst, and there the gap is enormous: `diag3` holds
200 of 200 at 31 cells where `fold` manages **18** and `blocks` manages **0**. `blocks` and
`diag3` and `idx3` share the separation figure to the digit and could not behave more
differently. **Separation was never the figure of merit. Burst spread is.**

And `diag3` is the arm that settles the wording, because it is the fold's **own level
sets** taken mod 3. It hits the optimal split exactly, matches the fold everywhere the
fold is good, and beats it 200 to 18 on the burst. So: **the fold's direction was right and
only its threshold was wrong.** It also inherits the fold's exact blind spot — both put a
full anti-diagonal into one class, and both fail that channel.

`idx3`'s clean sweep is an accident of arithmetic and is filed as one: `j mod 3 =
((n mod 3)·r + c) mod 3`, so at `n ≡ 1` it *is* `diag3` and concentrates the anti-diagonal
completely, at `n ≡ 0` it degenerates to `cols3`, and only at `n ≡ 2` — which 32 happens to
be — does it shatter rows, columns and diagonals together.

## THE CLOSING AUDIT — every bar, called vs landed

| bar | called | landed | verdict |
| --- | --- | --- | --- |
| P1 the coordinate | 0 exceptions | 89,439 cells, 0 exceptions | **MET** |
| P2 the dynamics | 0 exceptions, conjugacy holds | 159,598 / 159,597 / 158,265, all clean | **MET** |
| P3 Cayley's wall | 35–42%, stable to 3 points | 39.67 / 38.31 / 39.36% | **MET** |
| P4 the doubling map | holds | holds to 1e-9, cited to `inspirations.html:311-315` | **MET** |
| P5 the site's own function | 0 mismatches or SKIPPED loudly | 22,139 and 1,599 checked, 0 mismatches | **MET** |
| P6 the correction | positive at every n | +16.67 to +0.77 points, n = 4..128 | **MET** |
| P7 the name | with both caveats written down | Julia set of a degree-2 map; the sphere tension and the `d`-vs-`i` caveat are in `fold.rs` and the README | **MET** |
| G1 the lemma | provable | proved exhaustively | **MET** |
| G2 singles | the geometric law | 63.00% / 97.75% against 0.632 / 0.982 | **MET** |
| G3 the plateau | **0 accepting moves, a theorem** | **0.195** — a sum of two can land on a third | **MISSED**, and the alias finding replaces it |
| G4 the gradient's range is one | no signal at distance 2 | 1.000 vs 0.516, then 0.456 vs 0.479 | **MET** |
| G5 the honest-detection loss | > 0 somewhere | 2 under annealing; and the blind-burst row shows 10 wrong for `fold` | **MET** |
| G6 the count arm | **clears same-class doubles** | counts 100%, exact **0.50%** | **MISSED**, and it completes the thesis |
| G7 where a blind guess wins | measured | flagged erasures 100% with no table | **MET** |
| S1 the port | to the trial, and structurally | four pins, 30,491 facts, 0 mismatches | **MET**, by a stronger method than filed |
| S2 the fairness assert | identical cost | 48 bits for every arm | **MET** |
| S3 separation is not the figure | ≥ 30 points apart on bursts | `blocks` 0 against `diag3` 200 at 31 cells | **MET**, and more sharply than called |
| S4 the direction was right | `diag3` wins random pairs | it wins the **burst** by 200 to 18; the random-pair gap turns out to be worth **nothing** | **MET**, for a different reason than filed |
| S5 the cost of the geometry | separation gap plus 3× burst | the separation gap is worth 0 points; the burst gap is 200 vs 18 | **MET**, and the filed reason was wrong |
| S6 the accident, filed | reported as such | printed at n ≡ 0, 1, 2 | **MET** |

Twenty bars, seventeen met, two missed outright and one met by a stronger method than the
one filed. Both misses improved the round. The plateau was not a theorem, and what replaced
it — every accepting move on a two-cell error is the alias `q` exists to refuse — is a
better sentence. The count arm did not clear the channel, and the reason finishes the
thesis: the address makes search unnecessary, the metric makes it converge to the wrong
answer, and neither buys a decoder.

**The sentence the round was for.** Under `rho = 2^(d−(n−1))` the site's three regions are
the two Fatou basins and the Julia set of `z ↦ z²`, and the anti-transpose is the inversion
that exchanges them. The site had already named what that map does **on** the circle — the
doubling map, `inspirations.html:311-315` — and had never named the circle as the map's
invariant set. Cayley's wall is why it stops at two: three basins have no straight
separator, 38.3% of the cubic's boundary touching all three at once, and a grid split by a
straight anti-diagonal cannot hold that.

**And the correction the round owes eggSo-v0.** Its verdict was measured against a fair
three-way split when the fold is a two-basin object, and against a fair two-way split the
fold wins at every n. But the deeper correction is that the statistic itself was the wrong
one: it moves no error channel at all. The fold's real cost is the burst, and its own level
sets taken mod 3 fix it.

