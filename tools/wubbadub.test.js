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
/* 7. the point of the page: it reproduces the other three exactly.
      Wubba Dub also draws its seed, which Wub × and Wub ÷ do not, so those two
      are compared from the first operand onward. */
{
  const {loadPage: lp} = require(__dirname + "/domharness.js");
  /* Wub ± phasors have no rows/cols -- they were never rectangles -- so the
     square comparison drops those two fields. */
  const shape = (r, skip, box) => r(`JSON.stringify(M.P.slice(${skip}).map(p =>`
    + ` [p.inner, p.fold, p.outer, p.rateA, p.rateB, fmt(p.value)`
    + (box ? `, p.rows, p.cols` : ``) + `]))`);
  const ks = [3, 10, 200];

  const A = lp(__dirname + "/../wub.html");
  A.run(`KS = ${JSON.stringify(ks.map(k => ({k, plain: true, push: false, op: .55})))}; refresh();`);
  run(`KS = ${JSON.stringify(ks.map(k => ({k, pg: 0, mode: "plain", op: "+"})))}; refresh();`);
  ok(shape(A.run, 0, false) === shape(run, 0, false), "all-plain must reproduce Wub ±");

  for(const [page, sym] of [["wubx.html", "*"], ["wubdiv.html", "/"]]){
    const O = lp(__dirname + "/../" + page);
    O.run(`WIDTH = 8; KS = ${JSON.stringify(ks.map(k => ({k, push: false, op: .55})))}; refresh();`);
    run(`WIDTH = 8; KS = ${JSON.stringify(ks.map((k, i) =>
      ({k, pg: 0, mode: i ? "op" : "plain", op: sym})))}; refresh();`);
    ok(shape(O.run, 0, true) === shape(run, 1, true), `seed + all-${sym} must reproduce ${page}`);
  }
  console.log("  reproduces Wub ± exactly, and Wub × and Wub ÷ step for step");
}

/* 8. + and − are exact, and carry a ring when they leave the disc */
{
  const exact = run(`(() => {
    const abs = x => x < 0n ? -x : x;
    const gg = (a,b) => { a = abs(a); while(b){ [a,b] = [b, a % b]; } return a || 1n; };
    const red = (n,d) => { const h = gg(n,d); return [n/h, d/h]; };
    let bad = 0, rings = 0, n = 0;
    for(const sym of ["+", "-"]) for(const a of [3, 200, 255, 4095]) for(const b of [5, 60, 255, 1000]){
      KS = [{k:a,pg:0,mode:"plain",op:"+"}, {k:b,pg:0,mode:"op",op:sym}];
      refresh();
      const p = M.P[1];
      const A = stalkFrac(ownDigits({k:a,mode:"plain"}), 0);
      const B = stalkFrac(ownDigits({k:b,mode:"plain"}), 0);
      const want = red(sym === "+" ? A.num*B.den + B.num*A.den : A.num*B.den - B.num*A.den,
                       A.den * B.den);
      const got = stalkFrac(p.shown.slice(0, p.shown.length), p.E);
      const g2 = stalkFrac(p.cells, p.E);
      if(g2.num !== want[0] || g2.den !== want[1]) bad++;
      if(p.E !== 0) rings++;
      n++;
    }
    return {bad, rings, n};
  })()`);
  ok(exact.bad === 0, `${exact.bad} of ${exact.n} sums were wrong`);
  console.log(`  + and −: ${exact.n - exact.bad}/${exact.n} exact,`
    + ` ${exact.rings} needed a ring to stay inside the disc`);
  run(`KS = [{k:3,pg:0,mode:"plain",op:"+"},{k:5,pg:0,mode:"plain",op:"+"},{k:7,pg:0,mode:"plain",op:"+"}]; refresh();`);
}

/* 9. a card cannot be an operand with nothing above it */
{
  run(`KS = [{k:3,pg:0,mode:"op",op:"*"},{k:5,pg:0,mode:"plain",op:"+"}]; refresh();`);
  ok(run("M.N") === 2, "an operand in row 0 should still yield a phasor");
  ok(run(`M.P[0].kind`) === "square" && run(`M.P[0].op === undefined`),
     "row 0 must fall back to being a plain number");
  console.log("  an operand in row 0 falls back to plain — there is nothing above it");
}
/* 10. the facts describe the number, not the rectangle. A grid cell weighs
       2^-(r+c+2), so reading a rectangle left to right is a different value --
       Cells, Commas and Push all have to use the squashed vector. */
{
  run(`WIDTH = 8; KS = [{k:3,pg:0,mode:"plain",op:"+"},{k:10,pg:0,mode:"op",op:"*"},
       {k:200,pg:0,mode:"op",op:"+"},{k:6,pg:0,mode:"op",op:"/"}]; refresh();`);
  const rows = run(`M.P.map(p => { const v = hexValue(p.vec), u = hexValue(pushLeft(p.vec));
    return [p.kind, fmt(p.value), fmt(v), fmt(u)]; })`);
  rows.forEach((r, i) => {
    ok(r[1] === r[2], `card ${i+1} (${r[0]}): vector reads ${r[2]}, value is ${r[1]}`);
    ok(r[2] === r[3], `card ${i+1}: push changed the value to ${r[3]}`);
  });
  /* and a rectangle's vector must be shorter than its cells -- it is a squash */
  const g = run(`M.P.filter(p => p.kind === "grid").map(p => [p.vec.length, p.cells.length])`);
  ok(g.length > 0 && g.every(([v, c]) => v < c), "a grid's vector should be its squash");
  console.log(`  facts: value == vector == pushed on all ${rows.length} cards;`
    + ` grids squash ${g.map(([v,c]) => c + "\u2192" + v).join(", ")}`);
  run(`KS = [{k:3,pg:0,mode:"plain",op:"+"},{k:5,pg:0,mode:"plain",op:"+"},{k:7,pg:0,mode:"plain",op:"+"}]; refresh();`);
}
/* 11. the mode and the operator are one control, and it is sized for a finger.
       cards() writes the whole list as markup, so the bar can be read back. */
{
  run(`KS = [{k:3,pg:0,mode:"plain",op:"+"},{k:10,pg:0,mode:"op",op:"*"}]; refresh();`);
  const html = run(`$("cards").innerHTML`);
  const bars = html.split(`class="seg"`).length - 1;
  ok(bars === 2, `${bars} choice bars for 2 cards`);
  const per = (html.match(/class="sg[^"]*"/g) || []).length / 2;
  ok(per === 6, `${per} choices per card, wanted plain + pushed + 4 operators`);
  /* row 0 has nothing above it, so its operators must be unavailable */
  const first = html.slice(0, html.indexOf(`data-i="1"`));
  ok((first.match(/disabled/g) || []).length === 4, "row 0's four operators must be disabled");
  ok(!/disabled/.test(html.slice(html.indexOf(`data-i="1"`))), "row 1's operators must be live");
  /* exactly one choice is lit per card, and on card 2 it is the operator */
  const on = (html.match(/class="sg[^"]*\bon\b/g) || []).length;
  ok(on === 2, `${on} lit choices across 2 cards`);
  ok(/class="sg op on"[^>]*data-o="\*"/.test(html), "card 2 should light × itself");
  /* picking an operator has to carry the mode with it */
  ok(/data-m="op" data-o="\+"/.test(html), "each operator must also set the mode");
  console.log("  choice bar: 6 options per card, row 0's operators disabled,"
    + " one lit, operator carries the mode");
}

/* 12. touch targets: nothing on a card smaller than 44px */
{
  const css = run(`document`) && require("fs").readFileSync(__dirname + "/../wubbadub.html", "utf8");
  const need = [
    [".sg", /\.sg\{[^}]*min-height:44px/],
    ["input.kin", /input\.kin\{[^}]*min-height:44px/],
    [".rm", /\.rm\{[^}]*min-height:44px/],
    [".dt", /\.dt\{[^}]*width:34px;\s*height:34px/],
    ["add buttons", /\.addrow button\{min-height:44px\}/],
  ];
  for(const [what, re] of need) ok(re.test(css), `${what} is not sized for touch`);
  console.log("  touch: choices, input, remove and the add row at 44px; dots hit 34px");
}
/* 13. the composite array. Every card's vector sits on its own ring, so cell i
       of a card with ring E weighs 2^-(i+1-E). Lining them up by that absolute
       weight and adding must give the sum of the cards, and reconciling the
       redundant column sums must give that same number back. */
{
  const RACKS = {
    "3,5,7 plain": `[{k:3,pg:0,mode:"plain",op:"+"},{k:5,pg:0,mode:"plain",op:"+"},{k:7,pg:0,mode:"plain",op:"+"}]`,
    "chained ×": `[{k:3,pg:0,mode:"plain",op:"+"},{k:10,pg:0,mode:"op",op:"*"},{k:200,pg:0,mode:"op",op:"*"}]`,
    "across rings": `[{k:3,pg:0,mode:"plain",op:"+"},{k:8,pg:0,mode:"plain",op:"+"},{k:-7,pg:0,mode:"op",op:"+"}]`,
    "all four ops": `[{k:3,pg:0,mode:"plain",op:"+"},{k:10,pg:0,mode:"op",op:"*"},`
      + `{k:200,pg:0,mode:"op",op:"+"},{k:6,pg:0,mode:"op",op:"/"},{k:5,pg:0,mode:"pushed",op:"+"}]`,
  };
  const line = [];
  for(const [name, KS] of Object.entries(RACKS)){
    run(`WIDTH = 8; KS = ${KS}; refresh();`);
    const r = run(`(() => {
      const gg = (a,b) => { a = a<0n?-a:a; while(b){ [a,b] = [b, a % b]; } return a || 1n; };
      const c = compositeArray();
      let n = 0n, d = 1n;
      for(const p of M.P){ const f = stalkFrac(p.vec || p.shown, p.E);
        n = n * f.den + f.num * d; d = d * f.den; }
      const h = gg(n, d);
      const back = stalkFrac(c.stalk.d, c.stalk.E);
      return {
        sum: [String(c.num), String(c.den)],
        want: [String(n/h), String(d/h)],
        back: [String(back.num), String(back.den)],
        places: c.ws.length, owed: c.owed,
        realOwed: c.sums.filter(v => Math.abs(v) > 1).length,
        contiguous: c.ws.every((w, i) => i === 0 || w === c.ws[i-1] + 1),
      };
    })()`);
    ok(r.sum.join("/") === r.want.join("/"),
       `${name}: composite ${r.sum.join("/")} != sum of cards ${r.want.join("/")}`);
    ok(r.back.join("/") === r.sum.join("/"),
       `${name}: reconciled stalk reads ${r.back.join("/")}, not ${r.sum.join("/")}`);
    ok(r.owed === r.realOwed, `${name}: ${r.owed} carries reported, ${r.realOwed} actual`);
    ok(r.contiguous, `${name}: place values must run without gaps`);
    line.push(`${name} ${r.places}pv/${r.owed}c`);
  }
  console.log(`  composite array: sums the cards and reconciles back — ${line.join(", ")}`);
}

/* 14. the dominoes: one edge per cell, comma-grouped on the anti-diagonals */
{
  run(`KS = [{k:3,pg:0,mode:"plain",op:"+"}]; refresh();`);
  for(const k of [1, 3, 200, 65535]){
    run(`KS = [{k:${k},pg:0,mode:"plain",op:"+"}]; refresh();`);
    const d = run(`M.P[0].vec || M.P[0].shown`);
    const html = run(`dominoesHTML(M.P[0].vec || M.P[0].shown)`);
    const bars = (html.match(/class="dbar /g) || []).length;
    ok(bars === d.length, `${k}: ${bars} edges for ${d.length} cells`);
    const cmas = (html.match(/class="dcma"/g) || []).length;
    const want = run(`commas(M.P[0].vec || M.P[0].shown,
      Math.max(1, Math.ceil(Math.sqrt((M.P[0].vec || M.P[0].shown).length)))).length`) - 1;
    ok(cmas === want, `${k}: ${cmas} commas, wanted ${want}`);
    ok(/dground/.test(html), `${k}: the stalk needs its ground line`);
  }
  /* and they are on the card, which is where they were asked for. This page
     paints through page1HTML, not paintRow. */
  run(`KS = [{k:3,pg:0,mode:"plain",op:"+"}]; refresh();`);
  const card = run(`page1HTML(M.P[0], KS[0])`);
  ok(/dbar/.test(card), "the int card must carry its dominoes");
  ok(/gsc /.test(card), "...and still its square");
  ok(/<dt>Push<\/dt>/.test(card), "...and its facts");
  console.log("  dominoes: one edge per cell, commas on the anti-diagonals, on the card");
  run(`KS = [{k:3,pg:0,mode:"plain",op:"+"},{k:5,pg:0,mode:"plain",op:"+"},{k:7,pg:0,mode:"plain",op:"+"}]; refresh();`);
}
console.log("\nall good.");
