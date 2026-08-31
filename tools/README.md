# tools

Not part of the site. A stand-in DOM so a page can be *run* in node rather than
just parsed — enough of `document`, canvas and the window globals that the real
page scripts execute unmodified.

```
node tools/run.js wub.html
```

Loads whatever cores the page's own `<script src>` tags name, then its inline
scripts, then calls whichever entry points exist (`draw`, `refresh`, `select`,
`drawField`) and reports what threw.

It exists because "the page returns 200" says nothing about whether its
JavaScript ran. Several bugs this project has had — a stale `ctx`, a duplicated
`const`, an unbalanced `</div>`, a `<span id="spacing">` deleted out of a
rewritten paragraph while the code still wrote to it — were invisible to curl
and obvious here.

`loadPage(path)` is the shared loader: it builds a harness that knows the page's
ids, evaluates the page's scripts in a fresh context, and hands back
`{run, ctx, g, els}`. `run.js` and every test use it, so the boilerplate lives in
one place.

## What it deliberately fakes

Canvas is a proxy that swallows every drawing call and returns gradients from
`createRadialGradient`. Nothing is rasterised, so this catches *errors*, never
appearance. Sizes come back as fixed numbers, so layout must be verified in a
browser.

`querySelector` is absent and `querySelectorAll` returns empty, so code reached
through them does not run. Test those functions directly with a stub element —
`paintRow` is only ever exercised that way, and both Wub × and Wub ÷ do so.

## What it used to fake, and stopped

`getElementById` invented an element for any id you asked for. That turned a
real "this element does not exist" into a silent pass, and shipped a page whose
render loop was dead on arrival: the masthead had been rewritten without its
`<span id="spacing">`, `rack()` threw on its last line, and
`requestAnimationFrame` was never reached. The canvas stayed 300×150 and the
whole right half of the page was blank.

`harness(html)` now reads the ids the page actually declares and returns `null`
for anything else, which reproduces that failure immediately. Calling
`harness()` with no html keeps the old permissive behaviour; do not.

## Seeing it

The harness cannot tell you what a page *looks* like. For that:

```
google-chrome --headless=new --disable-gpu --window-size=1440,900 \
  --virtual-time-budget=5000 --screenshot=out.png http://127.0.0.1:1338/wub.html
```

To read an error the screenshot cannot show, put a `window.onerror` trap in the
`<head>` — not at the end of `<body>`, where it is registered too late to catch
the failure it is looking for — write the result into a hidden element, and
`--dump-dom`.

## The interaction layer

`chroma-ui.js` is loaded by every page, after `stalk.js`. Its binders are thin on
purpose, because the harness stubs `addEventListener` and nothing behind a listener
runs here. What is worth testing was written pure and is tested directly:

```
node tools/ui.test.js
```

`cellAt` and `indexOfCell` invert `cellOrder` from `stalk.js` rather than re-deriving
the anti-diagonal walk, so the hit-test cannot drift from the fold it is testing
against. `ui.test.js` checks that over every cell at n = 1..12, and sweeps all 63001
pixels of a square whose size is not divisible by its width, looking for a seam that
reports no cell — the flicker a reader feels but a spot check misses.

Two harness notes, both learned by breaking something:

`setAttribute` and friends now exist on the stub element. They did not, and a page
that set `aria-pressed` on its own play button — which every real element accepts —
came back as a THROW that said nothing about the page. They record, so a test can
assert on what a control published to assistive tech.

`querySelectorAll` still returns empty, so a page that reads its own markup gets
nothing here. That is why the attributions filter on `inspirations.html` keeps its
decision in `keeps(item, sec, q, before)`, a predicate over a plain object with no
DOM in it at all. `inspirations.test.js` calls it with synthetic items. Anything
that must be tested has to be reachable without the markup.

## Two more things the harness now does

`classList` is real, backed by `className`. It used to be four no-ops, which
meant a page that shows and hides by class passed whatever it did — including
doing nothing. `contains()` returning a flat `false` was the worst of them: any
guard written around it never took its own branch. The expanding widget panels
on the spectrometer are toggled entirely by class, and none of that was
testable until this changed.

The stub elements record attributes, so `getAttribute` reads back what
`setAttribute` wrote. They do **not** parse attributes out of the page's markup:
an element whose `aria-expanded="false"` is only in the HTML reads as `null`
here. That is a limit worth knowing rather than working around — a page that
publishes its own initial state from script, instead of inheriting it from the
markup, is both testable and immune to the two drifting apart.

## Tooltips

`glossary.js` is the site's vocabulary and `UI.tips` is the machine that shows it.
The split is deliberate: placement is arithmetic and is tested exhaustively, while
the writing is checked for the one thing code cannot catch.

```
node tools/tips.test.js
```

`UI.place()` is pure — anchor rect, card size, viewport in; x, y and a side out. The
test drives 360 combinations, including cards larger than the viewport and anchors
off every edge, and asserts the card never starts off the left or top edge. That is
the only failure a reader cannot work around: the text they asked for is off-screen.

The content check is the useful half. `data-tip="shel"` is not an error — it shows
nothing at all, silently. So the test reads every `data-tip` out of all nine pages
and asserts each resolves to a real entry, and that each entry has a title, a what
and a why that are not the same sentence. A tooltip whose second line restates its
own label is worse than no tooltip, because the reader spent a hover on it.

Entry text lands in `innerHTML`, so it is escaped first; the test proves it, since
one stray angle bracket in authored prose would eat the rest of the card.
