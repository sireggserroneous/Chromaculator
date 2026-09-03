# eggSo v4 — the fold is a basin boundary

Not part of the site. The nineteenth codec experiment and the fifth in the fold-native
lineage — [`eggSo-v0/`](../eggSo-v0/) used the fold's partition, [`eggSo-v1/`](../eggSo-v1/)
its symmetry, [`eggSo-v2/`](../eggSo-v2/) its alphabet's slack, [`eggSo-v3/`](../eggSo-v3/)
its radix and its scale. None of them touched what the fold **is**. Kept in its own folder
so it does not entangle with `chronochromatic.org`, which claims none of this.

**The first round in this lineage written in Rust.** Its own crate, no workspace, and an
empty `[dependencies]` — the policy `codegg-v13/Cargo.toml` states and every Rust round
here holds. JSON is hand-rolled in `src/json.rs`.

Built 2026-09-03 against [PREDICTIONS.md](PREDICTIONS.md), filed before a line of the
round was written.

## The verdict, first

**Twenty bars, seventeen met, two missed outright, one met by a stronger method than the
one filed. Both misses improved the round.**

Set `rho(r, c) = 2^((r+c) − (n−1))`, the anti-diagonal band as a modulus. Then

| the site says | this round says |
| --- | --- |
| Inner is "the low place values — the interior of the circle, one hemisphere" | `|rho| < 1`: the Fatou basin of `0` |
| Fold is "the middle place values — the circle of inversion, the equator" | `|rho| = 1`: the **Julia set** |
| Outer is "the high place values — the exterior, the other hemisphere" | `|rho| > 1`: the Fatou basin of `∞` |
| the anti-transpose "fixes the Fold, swaps Inner with Outer, and undoes itself" | `rho → 1/rho`, the inversion that exchanges the two basins |

Measured, and re-derived by the suite rather than quoted from the plan: **89,439 cells,
n = 2..64, zero exceptions**, for both halves of that claim.

The map is `z ↦ z²`, and the site **had already named what it does on the circle**.
`inspirations.html:311-315` places "dropping a digit; orbits and periods" on the
**doubling map** — and on `|z| = 1`, `z ↦ z²` is `θ ↦ 2θ`, the doubling map exactly. So the
site named the map's action on the Fold and never named the Fold as the map's invariant set.

And Cayley's 1879 wall is why the geometry stops at two. Newton's method on a quadratic is
conjugate to `z ↦ z²` by `w = (z−1)/(z+1)` — verified over **159,597 of 159,597** points —
so its two basins are the inside and outside of a circle. On a cubic, **38.3% of the basin
boundary touches all three basins at once**. Two regions can meet along a line; three meet
only at isolated points. A grid separated by a straight anti-diagonal is a degree-2 object,
and no amount of work on it will make it a degree-3 one.

## What failed, first

Two filed claims missed, and both left the round better than they found it.

- **"A same-class double has no accepting move" was filed as a theorem. It is not one.**
  Injectivity separates the `2L` values `{±2^k mod p}` from *each other*; it says nothing
  about whether a **sum of two** of them lands on a third. There are `O(L²)` such sums and
  `2L` targets in a ring of size `p ≈ 2L`, so collisions are the expected case. The rate is
  `|class|/p` — not `2|class|/p`, because a cell's current bit fixes which way its flip
  moves the syndrome and only one direction is available. Measured **0.195** against the
  0.166 that arithmetic predicts.

  What replaced it is sharper. **Every accepting move on a two-cell error is an alias**: one
  flip cannot undo two errors, so the move the search accepts is guaranteed wrong, and
  measured, **0** of them ever reach the clean square. Guess-and-fix does not stall on that
  channel — it walks confidently into the trap eggSo-v0 added `q` to refuse.

- **The count arm was predicted to clear the channel the residue cannot. It does not.** It
  reaches the right counts **100%** of the time and the right square **0.50%** of the time.
  A count says *how many* and never *which*. That completes the thesis instead of denting
  it: for the same ~33 bits you can buy an **address** or a **metric**; the address makes
  search unnecessary, the metric makes it converge to the wrong answer, and **neither
  purchase buys a decoder**.

- **Not filed and should have been, and it is the round's most useful correction.** The
  separation statistic eggSo-v0's whole verdict rested on **does not move the error channels
  at all**. Every seam arm takes `2 anywhere` at 397–400 of 400. The fold's 0.5303 against a
  fair split's 0.6673 is invisible on the very channel it was supposed to be about. The
  round filed `diag3` to win those rows by 12–15 points; it wins by **zero**.

## Why this round exists

The site's README has carried one open question since it was written —
`README.md:166`: *"So: what one does with the fold has now been placed. What the fold is
has not."*

Vladimir's pointer, 2026-09-03: Cayley asked where Newton's method lands from a given
guess, got a straight line for two roots, could not do three, and published the failure. It
stayed open until Julia and Fatou and then computers a century later. And the guess-and-fix
trick is already running here — `stalk.js:288-306`, the site's own divider, guesses a signed
digit from `{−1,0,+1}` at every step and carries the corrected remainder forward.

Three parts, in the order Vladimir set: *"Placement and correction only first, then add a
guess-and-fix decoder, and finally add the chosen-seam interleaver. All in the same run. So
we can see what each brings to the table and not assume."*

## The pins — the discipline the language change nearly cost

Every fold-native round asserts its restatement against the site's **own** function, not
against a second restatement. v0 checked its region rule against `stalk.js`'s `regions()`
cell for cell; v1 read the partner formula out of `index.html` at runtime. Rust cannot
`eval` `stalk.js` the way `tools/bulk.test.js` does, so the audit shells out to node. If
node is absent every pin reports **SKIPPED**, loudly, and never passes quietly.

| pin | checked | mismatches |
| --- | --- | --- |
| `region_of` vs `stalk.js`'s `regions()`, n = 2..40 | **22,139** | **0** |
| `arcs` vs `stalk.js`'s `arcs()` | 1,599 | **0** |
| the port vs eggSo-v0's **structure** | 6,153 | **0** |
| the port vs eggSo-v0's **decisions** | 600 | **0** |

22,139 is v0's own figure, reproduced. And because the round is Rust, v0's codec had to be
**ported rather than patched** — which is the better arrangement: `eggSo-v0/eggso.js` is not
touched, the three sibling JS rounds that require it keep working, and the port is held to a
stricter gate than an option flag would have been. The filed bar said "reproduce v0's
published aggregates"; that was replaced **before any number was taken** by the two pins
above, because aggregate equality can hide two compensating bugs and per-square agreement
cannot.

## Results, all measured

### Part 1 — the placement

```
the coordinate rho = 2^(d-(n-1)), n = 2..64: 89439 cells, 0 exceptions
Newton on z^2-1:                             159598 guesses, 0 exceptions
Newton on z^3-1 at 201x201:  1336 of 3368 boundary cells touch ALL THREE (39.67%)
Newton on z^3-1 at 301x301:  2292 of 5982 boundary cells touch ALL THREE (38.31%)
Newton on z^3-1 at 501x501:  4984 of 12662 boundary cells touch ALL THREE (39.36%)
```

Stable to 1.4 points across resolutions, so the tangle is the geometry and not the grid.
`eggso4 basins` also prints both pictures; two roots split down the middle and three do not:

```
  two roots                     three roots
  oooooooooooo ............     ooooooooooooooooo..#.....
  oooooooooooo ............     oooooooooooooooo.##o.....
  oooooooooooo ............     ooooooooooooooo##o.......
  oooooooooooo ............     #..o##oo###o.#...........
  oooooooooooo ............     ############.oo..........
  oooooooooooo ............     #############oo#.........
  oooooooooooo ............     #################oo......
```

**The correction to eggSo-v0's verdict.** v0 judged the fold against a fair *three-way*
split. If the Fold is a boundary rather than a class, the family is two-basins-plus-seam
and the baseline is a fair *two-way* split:

| n | fold | fair two-way | fold's margin | Fold's share of the square |
| --- | --- | --- | --- | --- |
| 4 | 0.7000 | 0.5333 | **+16.67 pts** | 25% |
| 16 | 0.5588 | 0.5020 | +5.69 pts | 6.25% |
| 32 | 0.5303 | 0.5005 | +2.98 pts | 3.125% |
| 128 | 0.5078 | 0.5000 | +0.77 pts | 0.78% |

Positive at every n. And the fingerprint: the Fold's share is `n/n² = 1/n` and **vanishes**.
A partition class keeps its share; a basin boundary has measure zero.

### Part 2 — can a decoder guess and fix? No, and the reason is the round

| | singles | same-class doubles |
| --- | --- | --- |
| GF-0, v0's table with the amendment | **400/400 in one lookup** | — |
| GF-1, blind, 1024 probes | 63.00% (the law says 0.632) | 0% |
| GF-1, blind, 4096 probes | 97.75% (the law says 0.982) | 0% |
| GF-3a, ring descent | — | 0.50% |
| GF-3c, annealing, 8 restarts | — | 3.75%, and 2 silent miscorrections |
| GF-6, one flagged erasure per class | — | **100%, with no table at all** |

**The gradient's range is exactly one step.** With one error the true cell descends 1.000 of
the time against a wrong cell's 0.516. With two errors it is **0.456 against 0.479** — no
signal, the true cells marginally *worse* than a coin. A division remainder shrinks at every
step; this is a coin everywhere except on the answer itself. **The very injectivity that
makes the table a single lookup is what starves the search.**

Two structural facts, both proved rather than sampled. Each cell belongs to exactly one
class, so a single-cell flip moves exactly one class syndrome — which makes "some class
syndrome hits zero" and "the live count decreases" **the same rule**, and makes random
restart **inert** under restoring acceptance. Two of the three decoders a first reading
would file are one decoder.

And the failure that matters is qualitative: **guess-and-fix cannot express ambiguity.** It
halts at the first square that satisfies the checks and calls it done, where v0 refuses. The
honest detection is the thing this decoder cannot buy at any budget.

### Part 3 — what the fold's forced seam costs

Every arm pays the same **48 check bits**, so every difference below is pure geometry.

| arm | classes | separation |
| --- | --- | --- |
| `fold` | 496 / 32 / 496 | 0.5303 |
| **`diag3` = `(r+c) mod 3`** | 341 / 342 / 341 | **0.6673** |
| `idx3` = `j mod 3` | 342 / 341 / 341 | 0.6673 |
| `blocks`, contiguous thirds | 341 / 341 / 342 | 0.6673 |

The flagged burst, swept, of 200 — because at 12 cells every arm wins and a single length
measures nothing:

| burst | 12 | 15 | 18 | 24 | 31 |
| --- | --- | --- | --- | --- | --- |
| `fold` | 200 | 200 | **116** | 70 | **18** |
| **`diag3`** | 200 | 200 | **200** | **200** | **200** |
| `idx3` | 200 | 200 | 200 | 200 | 200 |
| `rows3` | 200 | 200 | **0** | 0 | 0 |
| `blocks` | 200 | 200 | **10** | 9 | **0** |

`blocks`, `diag3` and `idx3` share the separation figure **to the digit** and could not
behave more differently. **Separation was never the figure of merit. Burst spread is.**

And `diag3` settles how the verdict should be worded, because it is the fold's **own level
sets** taken mod 3. It hits the optimal split exactly, matches the fold on every channel the
fold is good at, and beats it 200 to 18 on the burst. **The fold's direction was right and
only its threshold was wrong.** It also inherits the fold's exact blind spot: both put a full
anti-diagonal into a single class, and both fail that channel.

`idx3`'s clean sweep is an accident and is filed as one. `j mod 3 = ((n mod 3)·r + c) mod 3`,
so at `n ≡ 1` it *is* `diag3` and concentrates the anti-diagonal completely, at `n ≡ 0` it
degenerates to `cols3`, and only at `n ≡ 2` — which 32 happens to be — does it shatter rows,
columns and diagonals together.

### The honest section

- **The site declines the sphere, deliberately.** `spec.md:261-264` and
  `inspirations.html:189-192`: "Doubling only zero is what makes this a *disc with a centre*
  rather than a sphere." That sentence is about the number line's topology, zero doubled and
  ω unified. The sphere this round uses is the grid's `rho`-plane, where `z ↦ z²` has its two
  attracting fixed points at 0 and ∞. Different object — and the tension is printed here
  rather than smoothed over.
- **`d` is not `i`, and this is where the argument is thinnest.** A cell's weight is
  `2^−(i+1)` in the *stalk* index. The site's strong claim that "the anti-diagonals **are**
  the place values" is exact for the **product rectangle**, where weight is `2^−(r+c+2)`
  (`spec.md:108-110`, `stalk.js:229-237`, `stalk.js:336-341`). On a single folded stalk,
  anti-diagonal `d` holds `arcs(n)[d]` cells spanning a *band*. So `rho` is an exact
  normalised place value on the product grid and a magnitude **ordering** on the folded
  stalk. The site's own bridge sentence says "low / middle / high place values" —
  deliberately an ordering. This round claims that much and no more.
- **The placement is of the geometry, not of a novel object.** `z ↦ z²` is the oldest example
  in complex dynamics and its Julia set is the unit circle. The site's own footer already
  concedes at `index.html:324-327` that "inversive geometry is not [original] — the interest
  is in the coordinate system that puts them in one frame." What is new here is the
  identification, not the mathematics.
- **The lineage audit was clean and that is worth stating plainly.** Newton, Cayley, Julia,
  Fatou, Mandelbrot, fractal, basin, attractor, dynamical, Möbius, complex plane, root
  finding, stereographic — **zero mentions** across all eighteen prior experiment
  directories and the whole site.
- **The new Inspirations entry is the first `open` item with a date.** A Cayley 1879 citation
  makes it filterable by the year bar where every other debt in that section is undated.
  That is correct behaviour, not a bug.
- **Part 2 built a decoder nobody should ship.** It is slower by three orders of magnitude,
  it cannot express ambiguity, and it silently miscorrects where v0 refuses. It was built
  because the alternative was assuming, and the round says so.

## Running it

```
cargo test                                  # 32 identities and theorems, inline
cargo clippy --all-targets -- -D warnings   # clean, no suppressions
cargo build --release                       # no network: [dependencies] is empty

./target/release/eggso4 pin       # the port and the coordinate against the site's own code
./target/release/eggso4 basins    # Cayley's two-root line and three-root tangle
./target/release/eggso4 guess     # can a decoder guess and fix?
./target/release/eggso4 seam      # what the fold's forced 1/n seam costs
./target/release/eggso4 audit     # all of it, with the counts printed; --full widens
```

## Files

| | |
| --- | --- |
| `src/fold.rs` | the coordinate `rho`, the anti-transpose in it, `arcs`, `separation`. Carries the `d`-versus-`i` caveat in its header |
| `src/dynamics.rs` | `z ↦ z²`, Newton on the quadratic and the cubic, the Möbius conjugacy, basin grids and the ASCII pictures |
| `src/code.rs` | eggSo-v0's codec ported, class assignment as a parameter, both confirm placements |
| `src/guess.rs` | the decoder ladder, the accepting-move census, the count arm |
| `src/seam.rs` | the seven assignments, the channels, the burst sweep |
| `src/pin.rs` | the three pins, via node, SKIPPED loudly if it is missing |
| `src/json.rs` | ~150 lines, hand-rolled, because `[dependencies]` is empty |
| `PREDICTIONS.md` | filed before building; measured after; both misses kept |
| `measured-*.json` | the binary's own record of the numbers above |

## What this is and is not

It is the round that answered the question the site's README has carried from the start, and
the answer is that the fold is a **basin boundary** — the Julia set of the simplest degree-2
map there is, with Inner and Outer as its two Fatou basins and the anti-transpose as the
inversion between them. Cayley's wall explains why nothing here ever reached degree three.

It is not a claim to have found a new object. It is a claim to have found which old one this
is. And it owes eggSo-v0 a correction that goes deeper than the one it set out to make: not
only was the fold measured against the wrong family, the statistic itself was the wrong one.
