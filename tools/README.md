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
`const`, an unbalanced `</div>` — were invisible to curl and obvious here.

## What it deliberately fakes

Canvas is a proxy that swallows every drawing call and returns gradients from
`createRadialGradient`. Nothing is rasterised, so this catches *errors*, never
appearance. Sizes come back as fixed numbers, so layout must be verified in a
browser.

`querySelector` is absent and `querySelectorAll` returns empty, so code reached
through them does not run. Test those functions directly with a stub element.
