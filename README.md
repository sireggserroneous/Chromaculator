# Chromaculator

An instrument for looking at integers. Lives at **chronochromatic.org**.

Write a number in hex, so its bits arrive padded to a whole nibble. Lay them most
significant first into the smallest square that holds them, and fold that square
along its main anti-diagonal into three regions — **Inner**, **Fold**, **Outer** —
which sum back to the number exactly. Blue is +1, red is −1, green is 0. Negating a
number flips every colour.

Cell *i* weighs 2<sup>−(i+1)</sup>, so the value is *k* / 2<sup>4·nibbles</sup> —
always a dyadic rational strictly inside (−1, 1).

## The pages

| | |
|---|---|
| **Spectrometer** | one integer in full: its stalk, its square, its three regions, its value as a light wave, and its point on the unit sphere |
| **Atlas** | every integer as a ring of dyadics, ordered by value. Ring *r* is the 2<sup>r+1</sup> roots of unity; selecting one draws a radius and lights the path down to it |
| **Wub** | integers as phasors summed tip to tail, each riding an ellipsoid, tracing one closed curve. Torus knots, crossing counts, and a curve you can export |
| **Wub ×** | multiplication as a rectangle; an ordered sequence of operands |
| **Wub ÷** | division kept exact: quotient, a multiplier on the boundary, and the remainder drawn |
| **Wubba Dub** | every operation on one page: each card is plain, pushed, or an operand — + − × ÷ |
| **Spec** | the convention, editable in the page |
| **Inspirations** | who found each piece of this first |

Every page is interactive and every page explains itself. `stalk.js` holds the
arithmetic and the fold's geometry; `glossary.js` holds the vocabulary -- one entry
per term, each saying what it is and what it is for -- and `chroma-ui.js` holds the
controls — sliders, dropdowns, scrubbers with a play button,
and pointer drag with a hit-test that inverts `cellOrder` rather than re-deriving it.
The pages that teach the convention (Home, Spec) carry live figures inline; the
instruments carry a phase bar over their own clock, so a moving picture can be stopped
on a chosen instant and read.

The four Wub pages can be **heard** as well as seen: each phasor becomes two sine
voices, one per rate, at an amplitude set by its value. It is the same additive sum
the drawing is, so what you watch traced is what you hear. Nothing plays until you
press the button.

Every figure is **linkable** -- the hash carries the state, so an arrangement can be
sent to someone. And the landing page runs the instrument backwards: paint a square
by hand and read the number back out of it.

Nothing on the site expects you to have read the spec first. Hovering or tabbing to any
dotted term or control opens a card saying what it means and why you would touch it; a
caption under each figure says in plain words what is on screen right now; and a row of
**try** buttons on each figure jumps to a state worth seeing.

## Running it

```
uv run serve.py         # http://localhost:1338
```

`uv` builds the environment on first run, from `pyproject.toml` and `uv.lock`, at the
interpreter `.python-version` names. There is nothing to install: `serve.py` is stdlib
only, and the site itself has no Python in it at all. `uv sync` makes the environment
without starting the server; plain `python serve.py` still works if you would rather
not have one.

Every path in the pages is relative, so the site also opens straight off the disk --
double-click `index.html` and every page, figure and control works. The one thing that
needs a server is the Spec page's Save, which `fetch`es `spec.md` back to the `PUT`
below. Any static server will do for everything else; only `serve.py` answers that PUT.

Static files, one `PUT` endpoint so `spec.md` can be edited from the Spec page, and a
`404.html` for anything else. Only that one file is writable. The server is threaded:
a browser opens several connections at once and a single-threaded one deadlocks
behind the first slow client. On a static host the site still works; the Spec page
becomes read-only.

```
uv run serve.py --check    # the whitelist is the only thing between a PUT and the disk
```

## Checking a change

```
node tools/run.js wub.html      # does the page's JavaScript actually run?
node tools/product.test.js      # the product grid, against the arithmetic
node tools/wubx.test.js         # Wub x, including the parts run.js cannot reach
node tools/gizmo.test.js        # the corner gizmo points where it says
node tools/divide.test.js       # A = 2^e x Q x B + R, at every width
node tools/wubdiv.test.js       # Wub div, including the parts run.js cannot reach
node tools/load.test.js         # all three Wub pages under a full rack
node tools/running.test.js      # the 2^E-times-a-stalk pair all four operations share
node tools/wubbadub.test.js     # the paged cards, and that they reproduce the other three
node tools/ui.test.js           # the hit-test against cellOrder, and the scrubber's ends
node tools/index.test.js        # the landing page's figures: regions sum back to k
node tools/spec.test.js         # the spec panel: regions sum to the value, exactly
node tools/inspirations.test.js # the attribution filter, as a pure predicate
node tools/spectrometer.test.js # sweep, speed, phase clock, and the sphere's orbit
node tools/phase.test.js        # the phase bar on all four Wub pages
node tools/atlas.test.js        # the arc really is every dyadic, and the angle label matches it
node tools/audio.test.js        # the chord the Wub pages make, and the permalink codec
node tools/tips.test.js         # tooltip placement, and that every data-tip resolves
node tools/bulk.test.js         # 10,000 integers through both grids (~20s)
```

The suites above run against a stand-in DOM, which is enough for arithmetic and
not enough for a pointer. Dispatching `pointerleave` proves the handler works,
not that the browser ever sends it -- and a tooltip that stuck in a real Chrome
passed every synthetic test there was. So the pointer is tested with a real one:

```
npm i && npx playwright install chromium
node tools/tips.pw.js            # needs the server running
node tools/tips.pw.js --headed   # watch it
```

That is what caught 28 tooltips across the four Wub pages that could never
open: the entries all existed and all resolved, so the content check passed,
and nothing asked whether anything was listening.

`tools/sequences.js` is not a test. It prints the integer sequences this construction
makes -- distinct grids, the integers carried entirely by the Fold, the side of the
square -- computed from the site's own functions, in the form the OEIS takes. A
construction that is new leaves sequences nobody has written down; one that is a
rediscovery leaves sequences already in the literature under another name. Either
answer is worth having. Run it with `--bfiles` to write them out.

`run.js` runs a page's JavaScript against a stand-in DOM. A 200 from the server says
nothing about whether the page's scripts executed; this does. See `tools/`.

## Licence

Code under MIT, writing and figures under CC BY 4.0. See `LICENSE`. The site spends a
page naming everyone it borrowed from; it would be strange to be careful about that
and silent about its own terms.

## What is original here

Very little of the mathematics. Signed digits are Booth and Avizienis, the stalks are
Conway's, the bit reversal is van der Corput and Cooley–Tukey, the ring geometry is a
rotary encoder disc, the phasor sum is Fourier epicycles. See **Inspirations** for the
full accounting, with links.

What I could not place for a long time was the fold itself: laying a digit string along
the anti-diagonals of a square and cutting it into three regions that sum back to the
number. It now has a name, and the name is a **basin boundary**. Give a cell the
coordinate `rho = 2^((r+c) - (n-1))` and Inner becomes the inside of the unit
circle, Outer the outside, and the Fold the circle itself. That circle is the Julia set
of `z -> z^2`, whose two Fatou basins are exactly those two hemispheres, and the
anti-transpose is the inversion `rho -> 1/rho` that exchanges them. Checked over
89,439 cells, n = 2 to 64, with no exceptions.

That places the geometry rather than a new object: `z -> z^2` is the oldest example
in complex dynamics. And the page had already named what that map does *on* the circle,
the doubling map, credited under Shifting, without ever naming the circle as the map’s own
invariant set. Held loosely still, for one honest reason: the coordinate is an exact place
value on the product rectangle and a magnitude *ordering* on a single folded stalk, which
is what the three-region key says in words rather than in an equation.

There is now some evidence for that, of an odd kind. A separate series of experiments
read the site's pages as coding theory and built whatever each reading implied. Thirteen
rounds, and **every one landed on something already in the literature** — the square on
product codes, place value on residue arithmetic codes, the Atlas ordering on
interleavers, the Wub on Reed–Solomon, the file-scale work on the PAQ/lpaq family. All
of it is attributed in that series' own source, and none of it is claimed here.

The negative result is the interesting half. Across all thirteen, the fold's own
partition was **never the mechanism** — Inner and Outer appear nowhere in that work as
regions, and neither does the anti-transpose that swaps them. Every derivation walked
past the one construction the project cannot find a name for.

That is not proof the fold is new. It is a record that thirteen honest attempts to reach
prior art from inside this geometry all reached it by some other road.

A fourteenth, `eggSo-v0/`, then made the partition the mechanism for the first time — one
residue check per region, Inner, Fold and Outer, with the fold's own identity
`I + F + O = V` doing the accounting. It works, it met the bars it filed for itself, and
it has a name: an **interleaved AN code**, with the fold as the interleaving pattern. The
sharper finding is that as a pattern the fold is legitimate and sub-optimal — its
496 / 32 / 496 split separates two random errors 53% of the time where a plain three-way
split manages 67%. The thin seam that makes it the fold is what makes it worse.

So: what one *does* with the fold was placed first, and what the fold *is* came after.

Two more rounds, `eggSo-v1/` and `eggSo-v2/`, then tried the fold's *symmetry* and its
alphabet's *slack*, every way each could be built, with predictions filed before code and
the misses kept. v1 used the anti-transpose three ways. As one extra residue, `V − σV`, it
gives each hemisphere a second equation and names any same-region pair by lookup, with no
search and no miscorrection — a **two-syndrome arithmetic code**, the residue cousin of a
two-locator BCH code, and blind to the Fold it turns on. As a mirror, `Outer := σ(Inner)`,
it is a repetition code that reads an unflagged twelve-cell burst off the partner, the one
row no other arm holds, at twice the size. As an interleaver it is a no-op: identical to
v0 to the trial. v2 used the green, the alphabet's `0`. Under the canonical form every
green is trailing, so the slack is the 2-adic valuation of the value — a geometric law
with mean one, measured at 1.011 — and 0 of 10,000 random squares carry their own checks;
only zero-padded binaries do. Greens as three-valued erasures cannot be decoded on a run
at all, because `2^k − 2^(k−1) = 2^(k−1)` is an integer identity no residue sees — the
identity push is built on. The Wub's two-valued coin is not an approximation of the
erasure model; it is the only one that decodes. And one correction to the record above:
v0's refusal of same-region pairs came from where it applied its confirming residue, not
from the partition; applied per candidate, its own search takes 97.8% of them.

Three constructions, three names, one theorem about the greens — and then, in a fifth
round, a name for the fold itself. Arthur Cayley asked in 1879 where Newton's method lands
from a given guess. Two roots give a straight line down the middle; three give a boundary
he could describe and not solve, and it took Julia, Fatou and then computers a century
later to see it. The site's three regions are the two-root picture exactly, and Cayley's wall
is why this geometry never reaches three: 38% of a cubic's basin boundary touches all three
basins at once, and no straight anti-diagonal can do that.

That round corrected its own lineage twice, which is the more useful half. eggSo-v0 judged
the fold against a fair three-way split when it is a two-basin object, and against a fair
two-way split the fold wins at every width. And the statistic it judged on turns out to
move no error channel at all: what the fold's geometry actually costs is burst spread, where
its own level sets taken mod 3 hold 200 of 200 at a 31-cell burst and the fold itself
manages 18.

A fourth round, `eggSo-v3/`, then asked what those three had never varied: all of them put
**one bit in a cell** and stopped at the square's edge. Both were inherited, not chosen. A
cell holding a byte costs 1.12% where a bit costs 4.69%, and it turns the injury real
storage actually delivers — one corrupted byte — into a single error named by its own
syndrome: 3000 of 3000, against 604 of 3000 for the bit square, which also hands back
**wrong** data 43 times in 3000 and did so once on a real file. And the fold's own
anti-transpose, applied to the whole file instead of the square, scatters a contiguous
wound into thin erasures across every block: it costs **nothing at all** and multiplies the
survivable damage by 31× to 974×. What it does not buy is equations — surviving a 4 KB
wound needs one per damaged byte per region, which is Reed–Solomon, and nothing in that
folder survives 4 KB.
