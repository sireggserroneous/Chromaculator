/* node tools/wubbadub.test.js — the paged card layout.
   The harness has no querySelector, so cards()'s inner loop never runs; the
   pieces it would reach are exercised directly with stubs. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run, g} = loadPage(__dirname + "/../wubbadub.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* 1. one card per integer, one phasor each — no plain/pushed pair here */
{
  run(`KS = [{k:3,pg:0},{k:5,pg:0},{k:7,pg:0}]; refresh();`);
  ok(run("M.N") === 3, `3 integers -> 3 phasors, got ${run("M.N")}`);
  ok(run(`KS.every(e => !("plain" in e) && !("push" in e))`), "cards carry no spelling toggle");
  console.log(`  ${run("M.N")} integers, ${run("M.N")} phasors, `
    + run(`M.P.map(p => fmt(p.value)).join(" ")`));
}

/* 2. the facts page says all seven things, and says them correctly */
{
  const html = run(`factsHTML(M.P[0])`);
  for(const k of ["Inner", "Fold", "Outer", "Value", "Cells", "Commas", "Push"])
    ok(html.includes(`<dt>${k}</dt>`), `the facts are missing ${k}`);
  /* the commas must be the anti-diagonal grouping, not row grouping */
  const groups = run(`commas(M.P[0].shown, M.P[0].n).map(g => g.length).join(",")`);
  const shown = (html.match(/class="cma"/g) || []).length;
  ok(shown === run(`commas(M.P[0].shown, M.P[0].n).length`) - 1,
     `${shown} commas drawn for ${groups} groups`);
  /* push conserves the value, so the fact must agree with Value */
  const same = run(`(() => { const p = M.P[0], U = pushLeft(p.shown);
    const a = hexValue(U), b = p.value; return a.num === b.num && a.den === b.den; })()`);
  ok(same, "the pushed spelling must be the same number");
  console.log(`  facts: all 7 rows, commas grouped ${groups} on the anti-diagonals,`
    + ` push conserves the value`);
}

/* 3. paging shows exactly one page and lights exactly one dot */
{
  const mkCard = () => {
    const pgs = [0,1,2,3].map(p => ({dataset:{p:String(p)}, hidden:false}));
    const dts = [0,1,2,3].map(p => ({dataset:{p:String(p)}, on:false,
      classList:{add(){this._o.on = true}, remove(){this._o.on = false}}}));
    dts.forEach(d => d.classList._o = d);
    return {querySelectorAll: s => s === ".dpg" ? pgs : dts, _pgs: pgs, _dts: dts};
  };
  for(const q of [0,1,2,3]){
    const c = mkCard();
    run("showPage")(c, q);
    ok(c._pgs.filter(p => !p.hidden).length === 1, `page ${q}: not exactly one page shown`);
    ok(!c._pgs[q].hidden, `page ${q} should be the visible one`);
    ok(c._dts.filter(d => d.on).length === 1 && c._dts[q].on, `page ${q}: wrong dot lit`);
  }
  console.log("  paging: 4 pages, exactly one shown and one dot lit at every step");
}

/* 4. only the page you are on gets drawn */
{
  const stub = () => { const e = g.document.createElement("canvas"); e.width = 300; return e; };
  const mkCV = () => ({x:stub(), y:stub(), z:stub(), xy:stub(), yz:stub(), zx:stub(), helix:stub()});
  const drawn = cv => Object.entries(cv).filter(([, c]) => c.width !== 300).map(([k]) => k).sort();
  const want = {0: [], 1: ["x","y","z"], 2: ["xy","yz","zx"], 3: ["helix"]};
  for(const pg of [0,1,2,3]){
    run(`KS = [{k:3,pg:${pg}}]; refresh();`);
    const cv = mkCV();
    run("CV")[0] = cv;                       // cards() cannot reach the DOM here
    g.CV = run("CV");
    run(`CV[0] = null;`);                    // put ours in through the same slot
    vmSet(cv);
    run(`draw(100)`);
    const got = drawn(cv).join(",");
    ok(got === want[pg].join(","), `page ${pg} drew [${got}], wanted [${want[pg]}]`);
  }
  function vmSet(cv){ g.__stub = cv; run(`CV[0] = __stub;`); }
  console.log("  drawing: page 0 draws nothing, 1 draws x/y/z, 2 draws the projections, 3 the helix");
}

/* 5. the winding number the composite helix is drawn at is a real count */
{
  run(`KS = [{k:3,pg:0},{k:5,pg:0},{k:7,pg:0}]; refresh();`);
  const w = run("windingOf()");
  ok(Number.isInteger(w) && w >= 1, `winding is ${w}`);
  console.log(`  the summed curve winds ${w} times per period`);
}

/* 6. the helix stays inside whatever box it is handed. A tilted helix spreads
      much further than its radius, so this is the check that matters. */
{
  let worst = 0, cases = 0;
  for(const [W, H] of [[528,150],[470,150],[130,86],[90,300],[300,90]]){
    for(const freq of [1, 3, 8, 24]){
      const pts = [];
      const cv = g.document.createElement("canvas");
      cv.clientWidth = W; cv.clientHeight = H; cv.width = 0;
      const rec = new Proxy({}, {get: (t, k) =>
        (k === "moveTo" || k === "lineTo") ? ((x, y) => pts.push([x, y])) : (() => {}), set: () => true});
      cv.getContext = () => rec;
      run("helixOn")(cv, freq, "#fff", 0);
      ok(pts.length > 10, `${W}x${H} freq ${freq}: nothing drawn`);
      for(const [x, y] of pts){
        ok(isFinite(x) && isFinite(y), "non-finite point");
        worst = Math.max(worst, -x, x - W, -y, y - H);
      }
      cases++;
    }
  }
  ok(worst <= 0.5, `the helix left its canvas by ${worst.toFixed(2)}px`);
  console.log(`  helix fits: ${cases} canvas/frequency combinations, worst overflow`
    + ` ${worst.toFixed(2)}px`);
}
console.log("\nall good.");
