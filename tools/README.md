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
