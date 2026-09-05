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
      n: b.good.length,
      deg: b.good.map(i => Math.round(i.p.phase * 180 / Math.PI))})))`);
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
  const r = JSON.parse(p.run(`CARDS = [{src: "(1/3)"}]; recompute();
    JSON.stringify({exact: BODIES[0].good[0].exact,
      rem: BODIES[0].good[0].rem.num + "/" + BODIES[0].good[0].rem.den,
      cells: BODIES[0].good[0].raw ? BODIES[0].good[0].raw.join("") : ""})`));
  ok(!r.exact && r.rem !== "0/1", "1/3 must carry a remainder");
  const e = JSON.parse(p.run(`CARDS = [{src: "oops, 1/0, 3"}]; recompute();
    JSON.stringify(BODIES[0].items.map(it => it.ok ? "ok" : "bad"))`));
  ok(String(e) === "bad,bad,ok", `bad input should be reported per item: ${e}`);
  ok(p.run("BODIES[0].good.length") === 1, "the good item should still draw");
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
  ok(p.run("BODIES.filter(b => !b.isDef).length") === 2, "two bodies");
  /* a definition does not take one of the n slots, so two bodies are 180 apart */
  const ph = p.run(`JSON.stringify(BODIES.filter(b => !b.isDef)
    .map(b => Math.round(b.bp.phase * 180 / Math.PI)))`);
  ok(ph === "[0,180]", `two drawn bodies should be 180 apart, got ${ph}`);
  /* the slider drives everything downstream */
  const items = () => p.run(`JSON.stringify(BODIES.filter(b => !b.isDef)
    .map(b => b.good.map(i => String(i.f.num))))`);
  const before = items();
  p.run("CARDS[0].slider = 7; recompute();");
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
  ok(p.run(`JSON.stringify(BODIES[2].good.map(i => String(i.f.num)))`)
     === '["12","13"]', "the third card should read 12, 13");
  console.log("  [9] variables      a = 3 is a knob not a body; sliding it to 7 moves");
  console.log("                     every card that uses it, and b = a*4 follows");
  console.log("                     an unknown name errors, and so does a forward reference");
}

/* ---- 10. collapsing is the squash ----
 * Not a rendering tweak. Collapsing adds every int on every card together —
 * alignByWeight walks them all once by weight, each on its own ring, and
 * reconciles what they come to into ONE grid. Same walk + and - use, and the
 * same move as summing a product's anti-diagonals: a 2D spread becomes one
 * 1D stalk, and the scene becomes a single black body at {:}.
 */
{
  const set = srcs => p.run(`CARDS = ${JSON.stringify(srcs.map(s => ({src: s})))};
    recompute();`);
  set(["3", "5", "7"]);
  ok(p.run("ALL.num + '/' + ALL.den") === "15/1", "3 + 5 + 7 should collapse to 15");
  ok(p.run("ALL.parts") === 3, "three ints went in");
  ok(p.run("ALL.stalk.d.join('')") === "1111" && p.run("ALL.stalk.E") === 4,
     "15 is [1111] on ring 4");
  /* it does not matter how the ints are spread across cards */
  const one = p.run("ALL.num + '/' + ALL.den");
  set(["3, 5, 7"]);
  ok(p.run("ALL.num + '/' + ALL.den") === one,
     "the same ints on one card must collapse to the same total");
  set(["3, 5", "7"]);
  ok(p.run("ALL.num + '/' + ALL.den") === one, "and split any other way");
  /* every dyadic part sums exactly */
  for(const srcs of [["3", "5", "7"], ["1/2, 1/4", "1/8"], ["47*127", "-3"],
                     ["(13*3*127/2^4)", "2^10"]]){
    set(srcs);
    const want = p.run(`(() => { let s = {num: 0n, den: 1n};
      for(const b of BODIES) for(const it of b.good) s = fAdd(s, it.f);
      return s.num + "/" + s.den; })()`);
    ok(p.run("ALL.num + '/' + ALL.den") === want,
       `${srcs} should collapse to ${want}, got ${p.run("ALL.num + '/' + ALL.den")}`);
  }
  /* a definition is a knob, so it contributes nothing to the collapse */
  set(["a = 3", "5", "7"]);
  ok(p.run("ALL.parts") === 2 && p.run("ALL.num + '/' + ALL.den") === "12/1",
     "a definition should not be added in: " + p.run("ALL.num + '/' + ALL.den"));
  /* and a non-dyadic contributes its CUT, not a pretend exact value */
  set(["3", "1/3"]);
  const got = p.run("ALL.num + '/' + ALL.den");
  ok(got !== "10/3", "1/3 cannot land exactly, so the total must not claim it does");
  ok(got === "1004885/327680" || /\/\d+$/.test(got),
     "the total should be the cut sum, exactly: " + got);
  set([]);
  ok(p.run("ALL") === null, "nothing to collapse should be nothing, not a crash");
  console.log("  [10] collapse       3 + 5 + 7 = 15, cells [1111] on ring 4, and the");
  console.log("                      same however the ints are split across cards");
  console.log("                      a definition adds nothing; a cut contributes its cut");
}

console.log("\n  certified.");
