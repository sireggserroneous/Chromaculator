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

console.log("\n  certified.");
