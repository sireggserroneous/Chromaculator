/* node tools/tips.test.js — the tooltip system and the writing behind it.
 *
 * Two separable things. The placement is arithmetic and is tested hard: a card
 * that leaves the viewport is the one failure a reader cannot work around,
 * because the text they wanted is off the screen. The content is checked for
 * the thing no amount of code can catch — a control pointing at an entry
 * nobody wrote. */
const {loadPage} = require(__dirname + "/domharness.js");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const fs = require("fs");
const PAGES = ["index", "spectrometer", "atlas", "spec", "inspirations",
               "wub", "wubx", "wubdiv", "wubbadub"];

/* 1. the card never leaves the viewport, wherever the anchor is */
{
  const {run} = loadPage(__dirname + "/../index.html");
  const VW = 1200, VH = 800, GAP = 10;
  let checked = 0, worst = "";
  for(const w of [120, 300, 900, 1400])          // including a card wider than the page
    for(const h of [60, 200, 900])
      for(const ax of [-200, 0, 1, 600, 1199, 1400])
        for(const ay of [-50, 0, 400, 799, 1000]){
          const p = run(`UI.place({left:${ax}, top:${ay}, bottom:${ay + 20}, width:40},
                                  {w:${w}, h:${h}}, ${VW}, ${VH}, ${GAP})`);
          /* it may be clipped when the card is bigger than the viewport, but it
             must never start off the left or top edge -- that hides the title */
          ok(p.x >= 0, `x=${p.x} left the screen (card ${w}x${h} at ${ax},${ay})`);
          ok(p.y >= 0, `y=${p.y} left the screen (card ${w}x${h} at ${ax},${ay})`);
          if(w + 2 * GAP <= VW) ok(p.x + w <= VW, `card ran off the right: ${p.x}+${w} > ${VW}`);
          if(h + 2 * GAP <= VH) ok(p.y + h <= VH, `card ran off the bottom: ${p.y}+${h} > ${VH}`);
          ok(p.side === "above" || p.side === "below", `odd side ${p.side}`);
          checked++;
        }
  console.log(`  the card stays on screen across ${checked} anchor and size combinations`);
}

/* 2. it flips above when there is no room below, and not before */
{
  const {run} = loadPage(__dirname + "/../index.html");
  const low  = run(`UI.place({left:100, top:760, bottom:790, width:40}, {w:200, h:150}, 1200, 800, 10)`);
  ok(low.side === "above", `an anchor at the bottom should flip above, got ${low.side}`);
  const high = run(`UI.place({left:100, top:20, bottom:50, width:40}, {w:200, h:150}, 1200, 800, 10)`);
  ok(high.side === "below", `an anchor at the top should stay below, got ${high.side}`);
  ok(high.y >= 50 + 10 - 1, `the card overlapped the thing it points at: ${high.y}`);
  console.log(`  it opens below by default and flips above only when it must`);
}

/* 3. the glossary is well formed: every entry has all three parts, and the
      third one actually says something rather than restating the label */
{
  const {run} = loadPage(__dirname + "/../index.html");
  const keys = run(`UI.tips.keys()`);
  ok(keys.length >= 25, `only ${keys.length} entries registered`);
  for(const k of keys){
    const t = run(`UI.tips.get(${JSON.stringify(k)})`);
    ok(t.title && t.title.length > 1, `${k}: no title`);
    ok(t.what && t.what.length > 20, `${k}: "what" is too thin to help`);
    ok(t.why && t.why.length > 20, `${k}: "why" is too thin to help`);
    ok(t.what !== t.why, `${k}: what and why are the same sentence`);
    /* a definition that opens with the term itself is a dictionary that says
       "shell: the shell" -- worth catching while the writing is cheap to fix */
    ok(t.what.toLowerCase().indexOf(String(t.title).toLowerCase() + " is") !== 0,
       `${k}: "what" just restates the title`);
  }
  console.log(`  all ${keys.length} entries carry a title, a what and a why`);
}

/* 4. no control points at writing nobody did. This is the failure the system
      cannot catch by itself: data-tip="shel" simply shows nothing at all. */
{
  let total = 0;
  for(const name of PAGES){
    const html = fs.readFileSync(`${__dirname}/../${name}.html`, "utf8");
    const keys = [...html.matchAll(/data-tip="([^"]+)"/g)].map(m => m[1]);
    if(!keys.length) continue;
    const {run} = loadPage(`${__dirname}/../${name}.html`);
    const missing = run(`UI.tips.missing(${JSON.stringify(keys)})`);
    ok(missing.length === 0, `${name}.html points at entries that do not exist: ${missing.join(", ")}`);
    total += keys.length;
  }
  console.log(`  all ${total} data-tip anchors across ${PAGES.length} pages resolve to an entry`);
}

/* 5. the markup escapes. An entry is authored text, but it lands in innerHTML,
      and one stray angle bracket would silently eat the rest of the card. */
{
  const {run} = loadPage(__dirname + "/../index.html");
  run(`UI.tips.add({__x: {title: "a<b>", what: "x & y <script>", why: "1 < 2 & 3 > 2"}})`);
  const html = run(`UI.tips.html("__x")`);
  ok(html.indexOf("<script>") < 0, `a script tag survived into the card: ${html}`);
  ok(html.indexOf("&lt;") >= 0 && html.indexOf("&amp;") >= 0, `nothing was escaped: ${html}`);
  ok(run(`UI.tips.html("nope")`) === "", "an unknown key should render nothing");
  console.log(`  entry text is escaped before it reaches the card`);
}

/* 6. the first-visit memory: it answers, it remembers, and it survives storage
      being unavailable rather than taking the page down with it */
{
  const {run} = loadPage(__dirname + "/../index.html");
  ok(run(`UI.seen("zzz")`) === false, "an unseen key should be false");
  run(`UI.seen("zzz", true)`);
  ok(run(`UI.seen("zzz")`) === true, "marking it seen did not stick");
  const blind = run(`(() => { const old = localStorage;
    try { localStorage = null; return UI.seen("qqq"); } finally { localStorage = old; } })()`);
  ok(blind === false, "with no storage it should report unseen, not throw");
  console.log(`  first-visit memory remembers, and degrades quietly with no storage`);
}

/* 7. the card does not outstay its welcome.
      pointerleave closes it in the normal case, but it is not guaranteed to
      arrive: opening a native <select> puts an OS popup over the page and the
      label underneath never gets one. Two things catch that -- a pointer that
      has moved away, and a hard cap on how long any card may live. */
{
  const {run} = loadPage(__dirname + "/../index.html");

  /* the pointer rule, as coordinates */
  const R = {left: 100, top: 100, right: 200, bottom: 130};
  const over = (x, y, pad) => run(`UI.tips.overRect(${JSON.stringify(R)}, ${x}, ${y}, ${pad === undefined ? "undefined" : pad})`);
  ok(over(150, 115), "a point inside the anchor should count as over it");
  ok(over(205, 132), "a point just outside should still count -- the card is reachable");
  ok(!over(400, 115), "a point far to the side should not count");
  ok(!over(150, 400), "a point far below should not count");
  ok(!over(150, 115, 0) === false, "zero padding should still contain an inside point");
  ok(run(`UI.tips.overRect(null, 1, 1)`) === false, "no anchor means nothing to be over");

  /* with no card up, nothing is stale -- the listener must not fire on an
     empty page for every mouse move */
  ok(run(`UI.tips.stale(0, 0)`) === false, "stale should be false when no card is up");

  /* with one up, a pointer far away is stale and a pointer on it is not */
  const st = run(`(() => {
    const el = document.getElementById("foldn");
    UI.tips.show("t_width", el);
    const far = UI.tips.stale(9999, 9999);
    const near = UI.tips.stale(150, 150);   // the harness rect is 0,0..300,300
    UI.tips.hide(true);
    return {far, near, afterHide: UI.tips.stale(9999, 9999)};
  })()`);
  ok(st.far === true, "a pointer far from the anchor should be stale");
  ok(st.near === false, "a pointer over the anchor should not be stale");
  ok(st.afterHide === false, "nothing is stale once the card is down");

  /* and the hard cap is a real number, in the range a reader would accept */
  const life = run(`UI.tips.life`);
  ok(life >= 2000 && life <= 6000, `the life cap is ${life}ms, outside a sane range`);
  console.log(`  a card closes when the pointer leaves, and dies after ${life / 1000}s regardless`);
}

/* 8. a page that carries data-tip must also bind it.
      Four pages had 28 tooltips between them that never opened: the entries
      existed, so the content check above passed, and nothing anywhere asked
      whether anything was listening. That is the gap a resolver test leaves. */
{
  const fs2 = require("fs");
  const dead = [];
  for(const name of PAGES){
    const html = fs2.readFileSync(`${__dirname}/../${name}.html`, "utf8");
    const n = (html.match(/data-tip="/g) || []).length;
    if(!n) continue;
    const bound = html.indexOf("UI.tips.scan()") >= 0 || html.indexOf("UI.tips.attach") >= 0;
    if(!bound) dead.push(`${name}.html (${n} attributes, nothing binding them)`);
  }
  ok(dead.length === 0, `tooltips that can never open: ${dead.join("; ")}`);
  console.log(`  every page with data-tip attributes also binds them`);
}

console.log("tips ok");
