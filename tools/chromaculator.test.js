/* node tools/chromaculator.test.js — a card is a black body.
 *
 * No server: everything here is client side, because the arithmetic is exact
 * rationals in the page rather than a lookup.
 */
const {loadPage} = require("./domharness.js");
const path = require("path");
const ROOT = path.join(__dirname, "..");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const p = loadPage(path.join(ROOT, "chromaculator.html"));

/* ---- 1. what a card holds becomes phasors spaced evenly ---- */
{
  const set = src => p.run(`CARDS = ${JSON.stringify(src.map(s => ({src: s})))};
    recompute(); JSON.stringify(BODIES.map(b => ({
      n: b.live.length,
      deg: b.live.map(i => Math.round(i.phase * 180 / Math.PI))})))`);
  const one = JSON.parse(set(["3"]));
  const two = JSON.parse(set(["3, 5"]));
  const three = JSON.parse(set(["3, 5, 7"]));
  ok(one[0].n === 1 && String(one[0].deg) === "0", "one item sits at 0");
  ok(two[0].n === 2 && String(two[0].deg) === "0,180", "two items are 180 apart");
  ok(three[0].n === 3 && String(three[0].deg) === "0,120,240",
     "three items are 120 apart");
  const six = JSON.parse(set(["1,2,3,4,5,6"]));
  ok(String(six[0].deg) === "0,60,120,180,240,300", "six items are 60 apart");
  console.log("  [1] evenly spaced  1 at 0°, 2 at 180°, 3 at 120°, 6 at 60°");
}

/* ---- 2. the expressions, exact ---- */
{
  const read = src => JSON.parse(p.run(
    `CARDS = [{src: ${JSON.stringify(src)}}]; recompute();
     JSON.stringify(BODIES[0].items.map(it => it.ok
       ? {v: it.f.num + "/" + it.f.den, exact: it.exact}
       : {bad: it.why}))`));
  const CASES = [["47*127", "5969/1", true], ["(13*3*127/2^4)", "4953/16", true],
                 ["(1/3)", "1/3", false], ["(2/3)", "2/3", false],
                 ["(3/93)", "1/31", false], ["-3", "-3/1", true]];
  const bad = CASES.filter(([s, v, ex]) => {
    const r = read(s)[0];
    return r.v !== v || r.exact !== ex;
  });
  ok(!bad.length, "wrong: " + JSON.stringify(bad.map(c => [c[0], read(c[0])[0]])));
  console.log(`  [2] exact          ${CASES.length} expressions; 47*127 = 5969, `
    + "(13*3*127/2^4) = 4953/16");
  console.log("                     1/3, 2/3 and 3/93 are not dyadic, so they are cut");
}

/* ---- 3. a cut says what it dropped, and bad input is reported not crashed ---- */
{
  /* the ITEM says whether its result landed; its OPERANDS are exact integers */
  const r = JSON.parse(p.run(`CARDS = [{src: "(1/3)"}]; recompute();
    JSON.stringify({exact: BODIES[0].live[0].exact,
      rem: BODIES[0].live[0].rem.num + "/" + BODIES[0].live[0].rem.den,
      ops: BODIES[0].live[0].ops.length})`));
  ok(!r.exact && r.rem !== "0/1", "1/3 must carry a remainder");
  const e = JSON.parse(p.run(`CARDS = [{src: "oops, 1/0, 3"}]; recompute();
    JSON.stringify(BODIES[0].items.map(it => it.ok ? "ok" : "bad"))`));
  ok(String(e) === "bad,bad,ok", `bad input should be reported per item: ${e}`);
  ok(p.run("BODIES[0].live.length") === 1, "the good item should still draw");
  console.log(`  [3] honest         1/3 carries remainder ${r.rem}; a bad item is `
    + "reported and the good ones still draw");
}

/* ---- 4. bodies are packed apart, and never on top of each other ---- */
{
  for(const n of [1, 2, 3, 8, 30]){
    const s = JSON.parse(p.run(`JSON.stringify(sites(${n}))`));
    ok(s.length === n, `${n} sites expected, got ${s.length}`);
    if(n === 1) continue;
    let worst = Infinity;
    for(let i = 0; i < n; i++) for(let j = i + 1; j < n; j++)
      worst = Math.min(worst, Math.hypot(s[i][0]-s[j][0], s[i][1]-s[j][1],
                                         s[i][2]-s[j][2]));
    ok(worst > 0.2, `${n} bodies come within ${worst.toFixed(3)} of each other`);
    ok(s.every(v => Math.abs(Math.hypot(...v) - 1) < 1e-9),
       `${n}: a site left the shell`);
  }
  console.log("  [4] packed apart   1, 2, 3, 8 and 30 bodies all land on the shell "
    + "with no pair closer than 0.2");
}

/* ---- 5. the curve is finite and the depth stays a cosine ---- */
{
  p.run(`CARDS = [{src: "3, 5, 7"}, {src: "(13*3*127/2^4)"}, {src: "47*127, 1/3"}];
    recompute()`);
  ok(p.run("CURVES.every(c => c.every(v => v.every(Number.isFinite)))"),
     "a curve has a hole in it");
  ok(p.run("SPAN") > 0, "the field has no extent");
  const worst = p.run(`(() => { let m = 0;
    for(const c of CURVES) for(const v of c)
      m = Math.max(m, Math.abs(proj(v, 0, 0, 1).d));
    return m; })()`);
  ok(worst <= 1.0000001, `depth is not a cosine, worst |d| = ${worst}`);
  ok(!p.run("(() => { try { draw(); return false; } catch(e){ return e.message; } })()"),
     "draw() threw");
  console.log(`  [5] draws          3 bodies, curves finite, depth a cosine `
    + `(worst ${worst.toFixed(4)}), draw() runs clean`);
}

/* ---- 6. an integer wubs the way it does in Wub +-, and the waves are not flat ----
 * x is fold*cos(B)*cos(A), so a phasor with an empty fold has no radius and only
 * z moves. Cutting an integer to a fixed 16 cells pushed every lit bit into the
 * top-left, left the fold empty, and the sine waves came out dead flat. A whole
 * number takes phasor() — the same path Wub +- uses.
 */
{
  const same = p.run(`(() => {
    CARDS = [{src: "3"}]; SEL = 0; recompute();
    const mine = BODIES[0].good[0].p, theirs = phasor("3", false);
    return JSON.stringify({whole: BODIES[0].good[0].whole,
      cells: mine.shown.join(",") === theirs.shown.join(","),
      fold: mine.fold === theirs.fold, inner: mine.inner === theirs.inner,
      rates: mine.rateA === theirs.rateA && mine.rateB === theirs.rateB}); })()`);
  const m = JSON.parse(same);
  ok(m.whole && m.cells && m.fold && m.inner && m.rates,
     "an integer must be exactly the phasor Wub +- builds: " + same);
  /* and every axis must actually move */
  for(const src of ["3", "3, 5", "3, 5, 7", "(13*3*127/2^4)", "(1/3)"]){
    const pk = JSON.parse(p.run(`CARDS = [{src: ${JSON.stringify(src)}}]; SEL = 0;
      recompute(); JSON.stringify([0,1,2].map(k =>
        Math.max(...SUM[k].map(Math.abs))))`));
    ok(pk.every(v => v > 1e-9),
       `${src}: an axis is flat, peaks ${pk.map(v => v.toFixed(4))}`);
  }
  p.run(`CARDS = [{src: "3"}]; SEL = 0; recompute();`);
  const one = JSON.parse(p.run(`JSON.stringify([0,1,2].map(k =>
    Math.max(...SUM[k].map(Math.abs))))`));
  ok(Math.abs(one[0] - 0.125) < 1e-12,
     `3 should peak in X at its fold, 0.125, got ${one[0]}`);
  console.log("  [6] integers wub   3 is the same phasor Wub +- builds, cells, "
    + "regions and rates");
  console.log(`                     and no axis is flat: 3 peaks X ${one[0]}, `
    + `which is its fold`);
}

/* ---- 7. the bodies swing about {:} ----
 * A card is itself a phasor one level up. Two cards are 180 apart about {:},
 * three are 120 — the same n-gon step the items use inside a card, which is
 * 360/n, the EXTERIOR angle. The whole thing is tip to tail twice over.
 */
{
  const seats = n => JSON.parse(p.run(`
    CARDS = ${JSON.stringify("x".repeat(1))}.split("x").slice(0,0)
      .concat(${JSON.stringify([...Array(10).keys()].map(i => ({src: String(i + 2)})))}
        .slice(0, ${n}));
    recompute();
    JSON.stringify(BODIES.map(b => Math.round(b.bp.phase * 180 / Math.PI)))`));
  ok(String(seats(2)) === "0,180", "two bodies are 180 apart about {:}");
  ok(String(seats(3)) === "0,120,240", "three bodies are 120 apart");
  ok(String(seats(4)) === "0,90,180,270", "four bodies are 90 apart");
  /* a body's own phasor is what its card SUMS to */
  const tot = p.run(`CARDS = [{src: "3, 5, 7"}]; recompute();
    BODIES[0].total.num + "/" + BODIES[0].total.den`);
  ok(tot === "15/1", `a body should be its card's total, got ${tot}`);
  /* and they actually move */
  p.run(`CARDS = [{src: "3"}, {src: "3, 5"}, {src: "3, 5, 7"}]; recompute();`);
  const moves = p.run(`(() => { let m = 0;
    for(const b of BODIES){ const a = bodyAt(b, 0), c = bodyAt(b, 1.1);
      m = Math.max(m, Math.hypot(a[0]-c[0], a[1]-c[1], a[2]-c[2])); }
    return m; })()`);
  ok(moves > 1e-6, "the bodies are not swinging");
  ok(p.run(`BODIES.every(b => bodyAt(b, 0.7).every(Number.isFinite))`),
     "a body left the field");
  /* the swing is normalised, so no card's total drowns the others */
  const spread = JSON.parse(p.run(`CARDS = [{src: "3"}, {src: "47*127"}];
    recompute(); JSON.stringify(BODIES.map(b => {
      let m = 0; for(let i = 0; i <= 60; i++){ const v = bodyAt(b, Math.PI*2*i/60);
        m = Math.max(m, Math.hypot(v[0], v[1], v[2])); } return m; }))`));
  ok(Math.max(...spread) / Math.min(...spread) < 1.01,
     `3 and 47*127 should swing the same distance: ${spread}`);
  console.log("  [7] {:} is shared   2 bodies at 180°, 3 at 120°, 4 at 90°; a body");
  console.log("                      is its card's total (3,5,7 -> 15) and 3 swings");
  console.log("                      as far as 47*127, so neither drowns the other");
}

/* ---- 8. collapsing renders point groups, and both modes draw ---- */
{
  p.run(`CARDS = [{src: "3"}, {src: "3, 5"}, {src: "3, 5, 7"}]; recompute();`);
  for(const c of [false, true]){
    const r = p.run(`(() => { COLLAPSE = ${c};
      try { draw(); return true; } catch(e){ return e.message; } })()`);
    ok(r === true, `draw() threw with COLLAPSE=${c}: ${r}`);
  }
  ok(p.run("typeof COLLAPSE") === "boolean", "the toggle should be a flag");
  console.log("  [8] collapse        draws clean open and collapsed; collapsed drops");
  console.log("                      the sphere and leaves the point group on {:}");
}

/* ---- 9. variables: a card that reads name = expr is a knob ----
 * Desmos's move. The definition takes a slider instead of a body, and every
 * card after it can use the name. An unknown name is an ERROR, not a zero — a
 * typo should say so rather than quietly draw the wrong thing.
 */
{
  const set = srcs => p.run(`CARDS = ${JSON.stringify(srcs.map(s => ({src: s})))};
    recompute();`);
  set(["a = 3", "a, a*5", "a+1, a+2, a+3"]);
  ok(p.run("String(ENV.a.num)") === "3", "a should be 3");
  ok(p.run("BODIES.filter(b => b.isDef).length") === 1, "one definition");
  /* a definition is a knob AND a body: it has a value, so it draws. Typing
     a = 3 and getting an empty field was the page refusing to show the one
     thing on it. */
  ok(p.run("BODIES.length") === 3, "every card draws, definitions included");
  ok(p.run("BODIES[0].live.length") === 1, "a = 3 should draw its value");
  const ph = p.run(`JSON.stringify(BODIES.map(b =>
    Math.round(b.bp.phase * 180 / Math.PI)))`);
  ok(ph === "[0,120,240]", `three bodies should be 120 apart, got ${ph}`);
  /* the slider drives everything downstream */
  const items = () => p.run(`JSON.stringify(BODIES.slice(1)
    .map(b => b.live.map(i => String(i.f.num))))`);
  const before = items();
  p.run("CARDS[0].tick = tickOf({num:7n,den:1n}, CARDS[0]); CARDS[0].val = valOf(CARDS[0]); recompute();");
  ok(p.run("String(ENV.a.num)") === "7", "the slider should own the value");
  const after = items();
  ok(before !== after, "sliding a did not move the cards that use it");
  ok(after === '[["7","35"],["8","9","10"]]', "a=7 should give 7,35 and 8,9,10: " + after);
  /* an unknown name is reported, not guessed */
  set(["b*2"]);
  ok(!p.run("BODIES[0].items[0].ok")
     && /not defined/.test(p.run("BODIES[0].items[0].why")),
     "an undefined name must be an error");
  /* definitions read in order, so a forward reference is one too */
  set(["x = y", "y = 2"]);
  ok(p.run("CARDS[0].err") && /not defined/.test(p.run("CARDS[0].err")),
     "a forward reference should be reported");
  /* one definition can build on an earlier one */
  set(["a = 3", "b = a*4", "b, b+1"]);
  ok(p.run("String(ENV.b.num)") === "12", "b should be a*4 = 12");
  ok(p.run("String(BODIES[1].total.num)") === "12", "b = a*4 should be worth 12");
  ok(p.run(`JSON.stringify(BODIES[2].live.map(i => String(i.f.num)))`)
     === '["12","13"]', "the third card should read 12, 13");
  console.log("  [9] variables      a = 3 is a knob not a body; sliding it to 7 moves");
  console.log("                     every card that uses it, and b = a*4 follows");
  console.log("                     an unknown name errors, and so does a forward reference");
}

/* ---- 10. collapsing moves where points are measured from, nothing else ----
 * Every point, at time t, measured from {:} instead of from its own body. The
 * bodies stop swinging apart and their point groups sit on the one origin —
 * each card keeps its own phase step, so three items are still 120 apart.
 *
 * I built this as a SUM first: alignByWeight adds 3, 5, 7 into a single 15.
 * That is true and it is a different question. Collapsing keeps all three
 * waves.
 */
{
  const set = srcs => p.run(`CARDS = ${JSON.stringify(srcs.map(s => ({src: s})))};
    recompute();`);
  set(["3, 5, 7"]);
  const phases = () => p.run(`JSON.stringify(BODIES[0].live
    .map(i => Math.round(i.phase * 180 / Math.PI)))`);
  const at = t2 => p.run(`JSON.stringify(BODIES[0].good
    .map(i => comp(i.p, ${t2}).map(v => +v.toFixed(9))))`);
  const ph0 = phases(), v0 = at(0.4);
  ok(ph0 === "[0,120,240]", "three items are 120 apart: " + ph0);
  p.run("COLLAPSE = true;");
  ok(phases() === ph0, "collapsing must not move a phase");
  ok(at(0.4) === v0, "collapsing must not change what a point is worth at t");
  ok(p.run("BODIES[0].live.length") === 3,
     "collapsing must keep all three waves, not fold them into one");
  /* what it DOES change: every body is drawn from the same origin */
  set(["3", "5", "7"]);
  p.run("COLLAPSE = true;");
  const same = p.run(`(() => {
    const o = BODIES.map(() => proj([0, 0, 0], 100, 100, 50));
    return o.every(q => q.px === o[0].px && q.py === o[0].py); })()`);
  ok(same, "collapsed, every body should share the origin");
  /* open, they are apart */
  p.run("COLLAPSE = false;");
  const apart = p.run(`(() => {
    const o = BODIES.map(b => bodyAt(b, 0.4));
    let m = 0;
    for(let i = 0; i < o.length; i++) for(let j = i + 1; j < o.length; j++)
      m = Math.max(m, Math.hypot(o[i][0]-o[j][0], o[i][1]-o[j][1], o[i][2]-o[j][2]));
    return m; })()`);
  ok(apart > 1e-6, "open, the bodies should be somewhere else than each other");
  for(const c of [false, true]){
    const r = p.run(`(() => { COLLAPSE = ${c};
      try { draw(); return true; } catch(e){ return e.message; } })()`);
    ok(r === true, `draw() threw with COLLAPSE=${c}: ${r}`);
  }
  console.log("  [10] collapse       every point measured from {:}, and nothing else");
  console.log("                      changes: 3, 5, 7 stay three waves at 120°, worth");
  console.log("                      the same at every t. It is not a sum.");
}

/* ---- 11. a lone definition still draws, and the curves are sized to fit ----
 * Typing a = 3 gave an empty field: the definition was a knob and nothing else,
 * so the one thing on the page refused to render.
 */
{
  p.run(`CARDS = [{src: "a = 3"}]; recompute();`);
  ok(p.run("BODIES.length") === 1 && p.run("BODIES[0].good.length") === 1,
     "a lone definition must still draw");
  ok(p.run("String(BODIES[0].total.num)") === "3", "and be worth what it says");
  p.run("CARDS[0].tick = tickOf({num:5n,den:1n}, CARDS[0]); CARDS[0].val = valOf(CARDS[0]); recompute();");
  ok(p.run("String(BODIES[0].total.num)") === "5", "and follow its slider");
  /* the trail fills and stays finite */
  p.run(`CARDS = [{src: "3, 5"}]; recompute();
    for(let i = 0; i < 50; i++){ T += 0.05;
      BODIES.forEach((b, k) => { if(!b.good.length) return;
        let x = 0, y = 0, z = 0;
        for(const it of b.good){ const v = comp(it.p, T);
          x += v[0]; y += v[1]; z += v[2]; }
        if(!TRAIL[k]) TRAIL[k] = [];
        TRAIL[k].push([x, y, z]);
        while(TRAIL[k].length > TRAIL_MAX) TRAIL[k].shift(); }); }`);
  ok(p.run("TRAIL[0].length") === 50, "the trail should fill");
  ok(p.run("TRAIL[0].every(v => v.every(Number.isFinite))"), "the trail has a hole");
  /* and every count of bodies draws clean */
  for(const n of [1, 2, 3, 5, 9]){
    p.run(`CARDS = ${JSON.stringify([...Array(9).keys()]
      .map(i => ({src: String(i + 3)})))}.slice(0, ${n}); recompute();`);
    const r = p.run("(() => { try { draw(); return true; } catch(e){ return e.message; } })()");
    ok(r === true, `${n} bodies threw: ${r}`);
  }
  console.log("  [11] every card draws  a lone definition renders and follows its");
  console.log("                      slider; trails fill; 1, 2, 3, 5 and 9 bodies "
    + "all draw clean");
}

/* ---- 12. a card is a Wubba Dub card ----
 * Same renderer, not a lookalike: dominoesHTML, boxes and factsHTML moved out
 * of wubbadub.html into stalk.js, so the card here IS that card. nmOf had five
 * copies across the pages and now has one.
 */
{
  p.run(`CARDS = [{src: "3"}, {src: "3, 5, 7"}]; recompute();`);
  const h = p.run(`BODIES[0].good.map(it => page1HTML(it.p, {k: it.src})).join("")`);
  const WANT = ["Inner", "Fold", "Outer", "Value", "Cells", "Commas", "Push", "Spread"];
  const missing = WANT.filter(k => !h.includes(k));
  ok(!missing.length, "the card is missing " + missing.join(", "));
  /* and the numbers are the ones Wubba Dub prints for 3 */
  ok(h.includes("0.125000") && h.includes("0.062500") && h.includes("3/16"),
     "fold 0.125, outer 0.0625 and value 3/16 should all be there");
  ok(h.includes("446 nm"), "the wavelength should be there");
  ok(/dbar/.test(h) && /gsc/.test(h), "dominoes and the boxed grid should be there");
  /* one block per item on the card */
  const h2 = p.run(`BODIES[1].good.map(it => page1HTML(it.p, {k: it.src})).join("")`);
  ok((h2.match(/class="p1"/g) || []).length === 3,
     "a card with three items should show three blocks");
  /* every page builds and draws */
  ok(p.run(`(() => { for(const pg of [0, 1, 2]){ CARDS[0].page = pg;
      try { gallery(); drawGallery(); } catch(e){ return "page " + pg + ": " + e.message; } }
    return true; })()`) === true, "a gallery page threw");
  ok(p.run("PAGES.length") === 3, "three pages");
  console.log("  [12] the card       Inner, Fold, Outer, Value, Cells, Commas, Push,");
  console.log("                      Spread, dominoes and the boxed grid — 3 gives");
  console.log("                      fold 0.125, outer 0.0625, 3/16 at 446 nm");
}

/* ---- 13. a variable can hold int elements ----
 * a = 3, 5, 7 is a variable in the form of int elements, and naming it on its
 * own splices them in. Lists compose. In an EXPRESSION a list is an error
 * rather than a guess: a*2 over three values has no one obviously right answer.
 */
{
  const items = srcs => JSON.parse(p.run(
    `CARDS = ${JSON.stringify([].concat(srcs).map(s => ({src: s})))}; recompute();
     JSON.stringify(BODIES.map(b => b.items.map(i => i.ok
       ? (i.f.den === 1n ? String(i.f.num) : i.f.num + "/" + i.f.den)
       : "ERR")))`));
  ok(String(items(["a = 3, 5, 7", "a"])[1]) === "3,5,7", "a should splice its elements");
  ok(String(items(["a = 3, 5, 7", "1, a, 100"])[1]) === "1,3,5,7,100",
     "a list splices in place, keeping what is around it");
  ok(String(items(["a = 3, 5", "b = a, 9", "b"])[2]) === "3,5,9", "lists compose");
  ok(String(items(["a = 3, 5", "b = a, a", "b"])[2]) === "3,5,3,5",
     "a list can be used twice");
  /* a list has no one value, so it cannot stand in an expression */
  const err = p.run(`CARDS = [{src: "a = 3, 5, 7"}, {src: "a*2"}]; recompute();
    BODIES[1].items[0].why`);
  ok(/is a list/.test(err), "a list in an expression should say so: " + err);
  /* a scalar is still a knob, and its slider still drives everything */
  p.run(`CARDS = [{src: "n = 4"}, {src: "n, n*2"}]; recompute();
    CARDS[0].tick = tickOf({num: 7n, den: 1n}, CARDS[0]);
    CARDS[0].val = valOf(CARDS[0]); recompute();`);
  ok(p.run(`JSON.stringify(BODIES[1].items.map(i => String(i.f.num)))`)
     === '["7","14"]', "a scalar knob still drives its readers");
  ok(p.run("CARDS[0].list") === null, "a scalar is not a list");
  /* a definition that failed must not draw */
  const fwd = items(["z = q, 1", "q = 2"]);
  ok(String(fwd[0]) === "ERR",
     "a forward reference must not render: " + JSON.stringify(fwd[0]));
  /* and every spliced element gets its own phase */
  p.run(`CARDS = [{src: "a = 3, 5, 7"}, {src: "a"}]; recompute();`);
  ok(p.run(`JSON.stringify(BODIES[1].live.map(i =>
    Math.round(i.phase * 180 / Math.PI)))`) === "[0,120,240]",
     "spliced elements should space like any others");
  console.log("  [13] list variables  a = 3, 5, 7 splices where it is named; lists");
  console.log("                      compose; a list in an expression says so; and a");
  console.log("                      failed definition draws nothing");
}

/* ---- 14. a variable can actually be TYPED ----
 * The keystroke that turns a card into a definition is exactly the one that
 * repainted the list, and repainting rebuilds every row — so the input the
 * caret was in was thrown away and "b = 5" died on the "=". And a half-typed
 * "b = " reported a raw "Cannot read properties of undefined" into the row,
 * which made adding a variable look broken at the moment you were doing it.
 */
{
  for(const typed of ["b = 5", "c = 3, 5, 7", "x=1/3", "a = 3", "z = 47*127"]){
    for(let k = 1; k <= typed.length; k++){
      const r = p.run(`(() => { try {
        CARDS = [{src: ${JSON.stringify(typed.slice(0, k))}}]; CARDS[0].tick = null;
        recompute(); paint(); gallery();
        return "ok";
      } catch(e){ return e.message; } })()`);
      ok(r === "ok", `typing ${JSON.stringify(typed)} threw at "`
        + typed.slice(0, k) + `": ${r}`);
    }
    /* and it lands as the thing it says */
    const end = p.run(`CARDS = [{src: ${JSON.stringify(typed)}}]; CARDS[0].tick = null;
      recompute(); CARDS[0].partial ? "partial"
        : CARDS[0].def ? (CARDS[0].err ? "err" : (CARDS[0].list ? "list" : "knob"))
        : "plain"`);
    ok(end === "knob" || end === "list",
       `${JSON.stringify(typed)} should finish as a variable, got ${end}`);
  }
  /* half-typed is not wrong: it is quiet, not an error */
  p.run(`CARDS = [{src: "b = "}]; CARDS[0].tick = null; recompute();`);
  ok(p.run("CARDS[0].partial") === true && p.run("CARDS[0].err") === null,
     "a half-typed definition should be partial, not an error");
  ok(p.run("BODIES[0].live.length") === 0, "and draw nothing yet");
  /* the caret survives a repaint, which is what let the "=" through */
  ok(p.run("paint.toString().includes('setSelectionRange')"),
     "paint must put the caret back where it was");
  /* the boxed grid has its styles, or the cells stack into one column */
  ok(/\.gsq\{/.test(p.html) && /\.gsc\{/.test(p.html),
     "the boxed grid needs .gsq and .gsc, or boxes() lays out into one column");
  console.log("  [14] typing works   every keystroke of five variable spellings is");
  console.log("                      clean; half-typed is quiet, not an error; the");
  console.log("                      caret survives the repaint the '=' triggers");
}

/* ---- 15. the knob: a symmetric range, an integer stepper, exact ticks ----
 * The range used to be min(-10, v-10) .. max(10, v+10) — ten either side of
 * what you typed, floored at -10..10, so 3 gave the lopsided -10..13 and 1000
 * gave -10..1010. It is now -B..B with B = max(10, |v|), and min, max and step
 * are yours. The slider moves in whole TICKS of step, so its value stays an
 * exact rational: a float slider would put 0.30000000000000004 into a system
 * whose whole claim is that the value is exact.
 */
{
  const knob = src => p.run(`CARDS = [{src: ${JSON.stringify(src)}}]; recompute();
    JSON.stringify({lo: CARDS[0].lo, hi: CARDS[0].hi, ticks: ticks(CARDS[0]),
      v: String(CARDS[0].val.num)})`);
  for(const [src, lo, hi] of [["a = 3", -10, 10], ["a = 0", -10, 10],
                              ["a = 47", -47, 47], ["a = -4", -10, 10]]){
    const k = JSON.parse(knob(src));
    ok(k.lo === lo && k.hi === hi,
       `${src} should give ${lo}..${hi}, got ${k.lo}..${k.hi}`);
    ok(k.lo === -k.hi, `${src}: the range should be symmetric`);
  }
  /* the stepper lands on whole numbers */
  p.run(`CARDS = [{src: "a = 3"}]; recompute();
    CARDS[0].tick += 1; CARDS[0].val = valOf(CARDS[0]); recompute();`);
  ok(p.run("String(CARDS[0].val.num)") === "4" && p.run("CARDS[0].val.den") === 1n,
     "one step up from 3 is 4");
  /* and a fractional step stays exact */
  p.run(`CARDS = [{src: "a = 3"}]; recompute();
    CARDS[0].step = {num: 1n, den: 2n}; CARDS[0].tick = null; recompute();
    CARDS[0].tick += 1; CARDS[0].val = valOf(CARDS[0]); recompute();`);
  ok(p.run("CARDS[0].val.num + '/' + CARDS[0].val.den") === "7/2",
     "a half step from 3 is 7/2 exactly, not 3.5");
  console.log("  [15] the knob       -B..B with B = max(10, |v|), so 3 gives -10..10");
  console.log("                      not -10..13; the stepper lands on whole numbers");
  console.log("                      and a 1/2 step gives 7/2 exactly, never 3.5");
}

/* ---- 16. plain and pushed, per card ---- */
{
  const spell = (pl, pu) => JSON.parse(p.run(
    `CARDS = [{src: "3, 5", plain: ${pl}, push: ${pu}}]; recompute();
     JSON.stringify(BODIES[0].good.map(i => ({v: String(i.f.num), push: i.push,
       fold: +i.p.fold.toFixed(4), cells: i.p.shown.join("")})))`));
  const a = spell(true, false), b = spell(false, true), c = spell(true, true);
  ok(a.length === 2 && b.length === 2 && c.length === 4,
     "both on should give two rings per item");
  ok(a[0].cells !== b[0].cells, "pushing must rewrite the cells");
  ok(a[0].fold !== b[0].fold, "pushing must move the fold");
  /* push conserves the value, which is the whole point of it */
  ok(p.run(`(() => { const x = hexValue(BODIES[0].good[0].p.shown),
      y = hexValue(BODIES[0].good[1].p.shown);
    return x.num * y.den === y.num * x.den; })()`),
     "a pushed ring must be worth what the plain one is");
  /* plain and pushed are the SAME operand spelled twice, so they SHARE its
     phase. Spreading them as if they were two operands put a pushed ring
     exactly where the next operand's plain one already was. */
  ok(p.run(`JSON.stringify(BODIES[0].good.map(i =>
    Math.round(i.p.phase * 180 / Math.PI)))`) === "[0,0,180,180]",
     "a pushed ring sits on its own operand's phase, not beside it");
  console.log("  [16] plain / pushed  both can be on, giving two rings an operand;");
  console.log("                      push rewrites the cells and moves the fold while");
  console.log("                      conserving the value, and shares the operand's phase");
}

/* ---- 17. the card refreshes with the number ----
 * The sphere followed the value and the card did not: the input handler updated
 * the info line and never the gallery, so the dominoes, the grid and the facts
 * kept showing the number before the edit. 22339 and -22339 drew identically.
 */
{
  const facts = src => p.run(`CARDS = [{src: ${JSON.stringify(src)}}]; recompute();
    BODIES[0].good.map(it => page1HTML(it.p, {k: it.src})).join("")`);
  const a = facts("22339"), b = facts("-22339");
  ok(a !== b, "22339 and -22339 must not render the same card");
  ok(/class="gsc b/.test(a) && /class="gsc r/.test(b),
     "a positive draws blue cells and a negative red");
  ok(a.includes("22339/65536") && b.includes("-22339/65536"),
     "and each says its own value");
  /* the handler that edits a card must repaint the gallery, not only the info */
  const src = p.run("paint.toString()");
  ok(/gallery\(\)/.test(src),
     "editing a card must refresh the gallery, or the dominoes go stale");
  console.log("  [17] card refreshes  22339 and -22339 draw different cards — blue");
  console.log("                      cells against red — and editing repaints the");
  console.log("                      gallery, not just the info line");
}

/* ---- 18. randomise, reset, and collapsing a card to just the int ---- */
{
  const rnd = () => p.run(`(() => { const R = () => Math.floor(Math.random()*127) - 63 || 1;
    for(const c of CARDS){
      c.src = c.src.replace(/(\\^\\s*)?\\d+/g, (m, up) => up ? m : String(R()))
        .replace(/-{2,}/g, m => m.length % 2 ? "-" : "");
      c.tick = null; c.lo = null; c.hi = null; }
    recompute(); return JSON.stringify(CARDS.map(c => c.src)); })()`);
  p.run(`CARDS = [{src: "a = 3"}, {src: "a, a*5"},
                 {src: "47*127, (13*3*127/2^4)"}, {src: "2^10, 1/3"}]; recompute();`);
  for(let r = 0; r < 6; r++){
    const after = JSON.parse(rnd());
    /* the shape survives: names, operators and exponents all stand */
    ok(after[0].startsWith("a = "), "a definition keeps its name: " + after[0]);
    ok(after[1].startsWith("a, a*"), "an expression keeps its shape: " + after[1]);
    ok(/\^4/.test(after[2]) && /\^10/.test(after[3]),
       "an exponent is structure, not a value: " + after[2] + " | " + after[3]);
    ok(after.every(s => !/--/.test(s)),
       "signs must not pile up across passes: " + after.join(" | "));
    ok(p.run("BODIES.every(b => b.items.every(i => i.ok))"),
       "everything randomised must still parse: " + after.join(" | "));
    ok(p.run("Math.max(...BODIES.flatMap(b => b.good.map(i => i.p.shown.length)))") < 400,
       "a randomised stalk should stay a reasonable size");
  }
  /* reset puts the defaults back */
  p.run(`CARDS = DEFAULT(); SEL = 0; recompute();`);
  ok(p.run(`JSON.stringify(CARDS.map(c => c.src))`)
     === '["a = 3","a, a*5","a+1, a+2, a+3"]', "reset should restore the defaults");
  /* a card can be collapsed to just its int, and still draws */
  p.run(`CARDS = [{src: "3, 5"}, {src: "7"}]; recompute();
    CARDS[0].shut = true; paint();`);
  ok(p.run("CARDS[0].shut") === true, "the card should be shut");
  ok(p.run("BODIES[0].live.length") === 2,
     "a shut card is hidden, not removed — it still holds its items");
  ok(p.run("(() => { try { draw(); gallery(); return true; } catch(e){ return e.message; } })()")
     === true, "a shut card must not break the draw");
  console.log("  [18] randomise      six passes keep every name, operator and");
  console.log("                      exponent, never pile up signs, and always parse;");
  console.log("                      reset restores; a shut card still holds its phasors");
}

/* ---- 19. an expression is its OPERANDS ----
 * "3 + 2, 4 * 9" holds FOUR operands expressed as two. Drawing only the results
 * lost a1 and a2 entirely. The items take the n-gon step — two are 180 apart —
 * and an item's operands take their own step from there, so a1 and a2 straddle A.
 */
{
  const rings = (src, pl, pu) => JSON.parse(p.run(
    `CARDS = [{src: ${JSON.stringify(src)}, plain: ${pl}, push: ${pu}}]; recompute();
     JSON.stringify(BODIES[0].good.map(o => o.src + (o.push ? "+" : "")
       + "@" + Math.round(o.p.phase * 180 / Math.PI)))`));
  ok(String(rings("3 + 2, 4 * 9", true, false)) === "3@0,2@180,4@180,9@360",
     "3 + 2, 4 * 9 should draw four operands, not two results");
  ok(String(rings("3, 5", true, false)) === "3@0,5@180",
     "a bare int is one operand");
  /* the items still carry their results, which is what the card totals */
  p.run(`CARDS = [{src: "3 + 2, 4 * 9"}]; recompute();`);
  ok(p.run(`JSON.stringify(BODIES[0].live.map(i => String(i.f.num)))`) === '["5","36"]',
     "the items keep their results");
  ok(p.run("String(BODIES[0].total.num)") === "41",
     "the card totals the results, not the operands");
  ok(p.run(`JSON.stringify(BODIES[0].live.map(i =>
    Math.round(i.phase * 180 / Math.PI)))`) === "[0,180]",
     "two items are 180 apart, whatever they are made of");
  /* the operands are exact integers even when the result is not */
  p.run(`CARDS = [{src: "1/3"}]; recompute();`);
  ok(p.run("BODIES[0].good.length") === 2, "1/3 is made of two operands");
  ok(p.run("BODIES[0].good.every(o => o.f.den === 1n)"),
     "both operands are whole");
  ok(p.run("BODIES[0].live[0].exact") === false,
     "but their result is not dyadic, and the item says so");
  console.log("  [19] operands       3 + 2, 4 * 9 draws four rings, not two: a1 and");
  console.log("                      a2 straddle A while A and B stay 180 apart, and");
  console.log("                      1/3 is two exact operands with an inexact result");
}

/* ---- 20. the sweep: 1/8x to 8x in exact doublings ----
 * Powers of two, so the multiplier stays dyadic like everything else here, and
 * the ends clamp rather than running off.
 */
{
  const WANT = [[-3, 0.125, "1/8"], [-2, 0.25, "1/4"], [-1, 0.5, "1/2"],
                [0, 1, "1"], [1, 2, "2"], [2, 4, "4"], [3, 8, "8"]];
  for(const [e, mult, lab] of WANT){
    p.run(`setSweep(${e})`);
    ok(p.run("SWEEP()") === mult, `sweep ${e} should be ${mult}x, got ${p.run("SWEEP()")}`);
    ok(p.run("SWEEPLABEL()").startsWith(lab),
       `sweep ${e} should read ${lab}x, got ${p.run("SWEEPLABEL()")}`);
  }
  p.run("setSweep(99)");
  ok(p.run("SWEEPE") === 3, "the sweep must clamp at 8x");
  p.run("setSweep(-99)");
  ok(p.run("SWEEPE") === -3, "and at 1/8x");
  /* the clock actually follows it */
  p.run(`CARDS = [{src: "3"}]; recompute(); RUN = true;`);
  const advance = e => p.run(`(() => { setSweep(${e}); T = 0;
    const dt = 0.1; T += dt * 0.9 * SWEEP(); return T; })()`);
  const slow = advance(-3), fast = advance(3);
  ok(Math.abs(fast / slow - 64) < 1e-9,
     `8x should advance 64 times as far as 1/8x, got ${fast / slow}`);
  p.run("setSweep(0)");
  /* the sweep gets its own row: squeezed in beside the buttons it had no room
     to be swept, and the multiplier was pushed off the edge */
  ok((p.html.match(/class="bar/g) || []).length === 2, "the toolbar should be two rows");
  ok(/bar2[\s\S]{0,400}id="sweep"/.test(p.html),
     "the sweep belongs on the second row, with room to move");
  for(const id of ["add", "rand", "resetall", "reset", "run", "sweep",
                   "slower", "faster", "sweepv", "collapse"])
    ok(p.g.document.getElementById(id), `the toolbar lost #${id}`);
  console.log("  [20] the sweep      1/8x to 8x in seven exact doublings, clamped at");
  console.log("                      both ends; 8x advances the clock 64 times 1/8x,");
  console.log("                      on its own row with room to be swept");
}

console.log("\n  certified.");
