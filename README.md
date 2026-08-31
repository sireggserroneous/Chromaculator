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

What I have not been able to place is the fold itself: laying a digit string along the
anti-diagonals of a square and cutting it into three regions that sum back to the
number. Held loosely.
