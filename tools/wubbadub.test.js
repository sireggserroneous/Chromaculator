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
  ok(run(`KS.every(e => !("plain" in e) && !("push" in e) && !("mode" in e))`), "cards carry no spelling toggle");
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
  run(`KS = ${JSON.stringify(ks.map(k => ({k, pg: 0, spell: "plain", role: "num", op: "+"})))}; refresh();`);
  ok(shape(A.run, 0, false) === shape(run, 0, false), "all-plain must reproduce Wub ±");

  for(const [page, sym] of [["wubx.html", "*"], ["wubdiv.html", "/"]]){
    const O = lp(__dirname + "/../" + page);
    O.run(`WIDTH = 8; KS = ${JSON.stringify(ks.map(k => ({k, push: false, op: .55})))}; refresh();`);
    run(`WIDTH = 8; KS = ${JSON.stringify(ks.map((k, i) =>
      ({k, pg: 0, role: i ? "op" : "num", op: sym})))}; refresh();`);
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
      KS = [{k:a,pg:0,spell:"plain",role:"num",op:"+"}, {k:b,pg:0,role:"op",op:sym}];
      refresh();
      const p = M.P[1];
      const A = stalkFrac(ownDigits({k:a,spell:"plain",role:"num"}), 0);
      const B = stalkFrac(ownDigits({k:b,spell:"plain",role:"num"}), 0);
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
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},{k:7,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
}

/* 9. a card cannot be an operand with nothing above it */
{
  run(`KS = [{k:3,pg:0,role:"op",op:"*"},{k:5,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
  ok(run("M.N") === 2, "an operand in row 0 should still yield a phasor");
  ok(run(`M.P[0].kind`) === "square" && run(`M.P[0].op === undefined`),
     "row 0 must fall back to being a plain number");
  console.log("  an operand in row 0 falls back to plain — there is nothing above it");
}
/* 10. the facts describe the number, not the rectangle. A grid cell weighs
       2^-(r+c+2), so reading a rectangle left to right is a different value --
       Cells, Commas and Push all have to use the squashed vector. */
{
  run(`WIDTH = 8; KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:10,pg:0,role:"op",op:"*"},
       {k:200,pg:0,role:"op",op:"+"},{k:6,pg:0,role:"op",op:"/"}]; refresh();`);
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
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},{k:7,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
}
/* 11. spelling and role are two choices, not one. A card can be pushed AND an
       operand -- push conserves the value, so it moves the working and never
       the answer, which is the whole reason to allow it. */
{
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},`
    + `{k:10,pg:0,spell:"pushed",role:"op",op:"*"}]; refresh();`);
  const html = run(`$("cards").innerHTML`);
  ok((html.split(`class="segs"`).length - 1) === 2, "each card needs its two bars");
  const spells = (html.match(/class="sg sp/g) || []).length / 2;
  const roles = (html.match(/class="sg rl/g) || []).length / 2;
  ok(spells === 2, `${spells} spellings per card, wanted plain + pushed`);
  ok(roles === 5, `${roles} roles per card, wanted num + four operators`);
  /* row 0 has nothing above it, so only its operators are unavailable */
  const first = html.slice(0, html.indexOf(`data-i="1"`));
  ok((first.match(/disabled/g) || []).length === 4, "row 0's four operators must be disabled");
  ok(/data-s="pushed"[^>]*>/.test(first) && !/data-s="pushed"[^>]*disabled/.test(first),
     "row 0 must still be free to be pushed");
  /* card 2 is pushed AND an operand: one lit in each bar */
  const second = html.slice(html.indexOf(`data-i="1"`));
  ok(/class="sg sp on" data-s="pushed"/.test(second), "card 2 should light pushed");
  ok(/class="sg rl op on"[^>]*data-o="\*"/.test(second), "card 2 should light ×");
  ok(!/class="sg rl on"/.test(second), "...and not also light num");
  console.log("  two bars: 2 spellings + 5 roles, row 0 keeps its spelling,"
    + " a card can be pushed and an operand at once");
}

/* 11b. and pushing an operand moves the working without moving the answer */
{
  let same = 0, moved = 0, n = 0;
  for(const a of [3, -3, 200, 255, 4095]) for(const b of [5, -5, 10, 200, -255])
    for(const sym of ["+", "-", "*", "/"]){
    const grab = sp => run(`WIDTH = 12; KS = [{k:${a},pg:0,spell:"plain",role:"num",op:"+"},`
      + `{k:${b},pg:0,spell:"${sp}",role:"op",op:"${sym}"}]; refresh();`
      + `(()=>{const p = M.P[1], f = stalkFrac(p.vec || p.shown, p.E);`
      + ` return {v: String(f.num) + "/" + String(f.den),`
      + ` work: p.overlay ? p.overlay.rows.map(r => r.join(",")).join("|") : p.cells.join(",")};})()`);
    const P = grab("plain"), U = grab("pushed");
    n++;
    if(P.v === U.v) same++;
    if(P.work !== U.work) moved++;
  }
  ok(same === n, `${n - same} of ${n}: pushing an operand changed the answer`);
  ok(moved > n * 0.6, `only ${moved} of ${n} workings moved — push should be visible`);
  console.log(`  pushed operand: ${same}/${n} answers unchanged, ${moved}/${n} workings moved`);
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},`
    + `{k:7,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
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
    "3,5,7 plain": `[{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},{k:7,pg:0,spell:"plain",role:"num",op:"+"}]`,
    "chained ×": `[{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:10,pg:0,role:"op",op:"*"},{k:200,pg:0,role:"op",op:"*"}]`,
    "across rings": `[{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:8,pg:0,spell:"plain",role:"num",op:"+"},{k:-7,pg:0,role:"op",op:"+"}]`,
    "all four ops": `[{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:10,pg:0,role:"op",op:"*"},`
      + `{k:200,pg:0,role:"op",op:"+"},{k:6,pg:0,role:"op",op:"/"},{k:5,pg:0,spell:"pushed",role:"num",op:"+"}]`,
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
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
  for(const k of [1, 3, 200, 65535]){
    run(`KS = [{k:${k},pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
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
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
  const card = run(`page1HTML(M.P[0], KS[0])`);
  ok(/dbar/.test(card), "the int card must carry its dominoes");
  ok(/gsc /.test(card), "...and still its square");
  ok(/<dt>Push<\/dt>/.test(card), "...and its facts");
  console.log("  dominoes: one edge per cell, commas on the anti-diagonals, on the card");
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},{k:7,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
}
/* 15. the chevron tucks the gallery away. The trap here is that a flex
       container ignores the hidden attribute unless the CSS says otherwise --
       this project has shipped two blank panels to exactly that. */
{
  const src = require("fs").readFileSync(__dirname + "/../wubbadub.html", "utf8");
  ok(/\.gal\[hidden\]\{display:none\}/.test(src),
     "display:flex would beat [hidden] without an explicit rule");
  ok(/id="galtoggle"[\s\S]{0,200}aria-controls="gal"/.test(src),
     "the chevron must point at what it collapses");
  ok(/\.chev\{[^}]*min-height:44px/.test(src), "the chevron is a touch target");
  /* one .addrow rule, not two fighting over flex-wrap */
  const rules = (src.match(/^\.addrow\{/gm) || []).length;
  ok(rules === 1, `${rules} .addrow rules — a second one silently keeps the first's flex-wrap`);
  /* and a hidden gallery must not be drawn */
  ok(/if\(GALOPEN && TOPPAGE === 1\)/.test(src),
     "the gallery should not be drawn while it is tucked away");
  console.log("  chevron: [hidden] beats display:flex, one .addrow rule,"
    + " 44px target, hidden gallery is not drawn");
}
/* 16. + and − are done ON the grid, not on the values.
       × has always built a rectangle and ÷ a tableau; addition used to be
       computed on the values and only shown as a result, which is why negating
       an operand reshuffled the whole array instead of flipping a row. */
{
  const ABS = x => x < 0n ? -x : x;
  const gg = (a,b) => { a = ABS(a); while(b){ [a,b] = [b, a % b]; } return a || 1n; };
  const red = (n,d) => { if(d < 0n){ n = -n; d = -d; } const h = gg(n,d); return [n/h, d/h]; };
  const val = k => { const v = run(`(()=>{const f=hexValue(ownDigits({k:${k},spell:"plain",role:"num"}));`
    + `return [String(f.num),String(f.den)];})()`); return [BigInt(v[0]), BigInt(v[1])]; };

  let bad = 0, n = 0, owed = 0;
  for(const a of [3, -3, 7, -7, 200, -200, 255, -255, 4095, -4095])
    for(const b of [3, -3, 10, -10, 255, -255])
      for(const sym of ["+", "-"]){
    run(`KS=[{k:${a},pg:0,spell:"plain",role:"num",op:"+"},{k:${b},pg:0,role:"op",op:"${sym}"}]; refresh();`);
    const r = run(`(()=>{const p=M.P[1], o=p.overlay; const f=stalkFrac(p.vec||p.shown,p.E);
      return {sum:[String(o.num),String(o.den)], val:[String(f.num),String(f.den)], owed:o.owed,
              contiguous:o.ws.every((w,i)=>i===0||w===o.ws[i-1]+1),
              addsUp:o.sums.every((v,i)=>v===o.a[i]+o.b[i]),
              inRange:o.a.concat(o.b).every(v=>v>=-1&&v<=1)};})()`);
    const A = val(a), B = val(b);
    const [wn, wd] = red(sym === "+" ? A[0]*B[1] + B[0]*A[1] : A[0]*B[1] - B[0]*A[1], A[1]*B[1]);
    n++; owed += r.owed;
    const good = BigInt(r.sum[0]) === wn && BigInt(r.sum[1]) === wd
              && BigInt(r.val[0]) === wn && BigInt(r.val[1]) === wd
              && r.contiguous && r.addsUp && r.inRange;
    if(!good) bad++;
  }
  ok(bad === 0, `${bad} of ${n} overlays wrong`);
  console.log(`  overlay: ${n}/${n} exact — columns contiguous, every column = a+b,`
    + ` the two rows stay signed digits, ${owed} carries owed`);

  /* negating the operand must flip exactly one row */
  let bad2 = 0, n2 = 0;
  for(const a of [3, 200, 255, 4095]) for(const b of [3, 5, 10, 200, 255]){
    const grab = k => run(`KS=[{k:${a},pg:0,spell:"plain",role:"num",op:"+"},{k:${k},pg:0,role:"op",op:"+"}];`
      + ` refresh(); (()=>{const o=M.P[1].overlay;`
      + ` return {a:o.a.join(","), b:o.b.join(","), ws:o.ws.join(",")};})()`);
    const P = grab(b), N = grab(-b);
    n2++;
    if(P.a !== N.a || P.ws !== N.ws) bad2++;
    else if(P.b.split(",").map(Number).map(x => -x).join(",") !== N.b) bad2++;
  }
  ok(bad2 === 0, `${bad2} of ${n2} negations disturbed more than the operand row`);

  /* and a − b must draw the same row as a + (−b) */
  let bad3 = 0, n3 = 0;
  for(const a of [3, -200, 255]) for(const b of [5, -10, 4095]){
    const row = (k, sym) => run(`KS=[{k:${a},pg:0,spell:"plain",role:"num",op:"+"},`
      + `{k:${k},pg:0,role:"op",op:"${sym}"}]; refresh(); M.P[1].overlay.b.join(",")`);
    n3++;
    if(row(b, "-") !== row(-b, "+")) bad3++;
  }
  ok(bad3 === 0, `${bad3} of ${n3}: a−b should draw the same row as a+(−b)`);
  console.log(`  negation flips one row and nothing else (${n2}/${n2}),`
    + ` and a−b draws the same row as a+(−b) (${n3}/${n3})`);

  /* the card actually shows it */
  run(`KS=[{k:200,pg:0,spell:"plain",role:"num",op:"+"},{k:-7,pg:0,role:"op",op:"+"}]; refresh();`);
  const html = run(`page1HTML(M.P[1], KS[1])`);
  ok(/overlay/.test(html) && (html.match(/class="crow"/g) || []).length === 3,
     "the card should show both rows and their sum");
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},{k:7,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
}
/* 17. the spelling reaches what a card PRODUCES, not just what it consumes.
       0,+1 is the same number as 1,-1 and 0,-1 the same as -1,1, so a pushed
       card must leave no 0,+-1 pair anywhere -- operand or result. That means
       push runs a second time, on the way out. */
{
  const pairs = a => { let c = 0; for(let i = 1; i < a.length; i++)
    if(a[i-1] === 0 && a[i] !== 0) c++; return c; };
  let left = 0, n = 0, valBad = 0, plainHad = 0;
  const NUMS = [3, -3, 7, -7, 200, -200, 255, 1000, -1000, 4095];
  for(const a of NUMS) for(const b of NUMS) for(const sym of ["+", "-", "*", "/"]){
    const grab = sp => run(`WIDTH = 12; KS = [{k:${a},pg:0,spell:"${sp}",role:"num",op:"+"},`
      + `{k:${b},pg:0,spell:"${sp}",role:"op",op:"${sym}"}]; refresh();`
      + ` M.P.map(p => ({vec: p.vec || p.shown,`
      + ` v: (()=>{const f = stalkFrac(p.vec || p.shown, p.E);`
      + ` return String(f.num) + "/" + String(f.den);})()}))`);
    const P = grab("pushed"), Q = grab("plain");
    P.forEach((p, i) => { n++; left += pairs(p.vec); if(p.v !== Q[i].v) valBad++; });
    Q.forEach(q => { plainHad += pairs(q.vec); });
  }
  ok(left === 0, `${left} of ${n} pushed cards still hold a 0,±1 pair`);
  ok(valBad === 0, `${valBad} values moved when the spelling changed`);
  ok(plainHad > 0, "the plain spelling should still have them — otherwise this proves nothing");
  console.log(`  pushed cards: 0 of ${n} keep a 0,±1 pair (plain keeps ${plainHad}),`
    + ` and no value moved`);

  /* and it reaches through a chain, not just one step */
  run(`WIDTH = 12; KS = [{k:200,pg:0,spell:"pushed",role:"num",op:"+"},`
    + `{k:7,pg:0,spell:"pushed",role:"op",op:"*"},`
    + `{k:5,pg:0,spell:"pushed",role:"op",op:"+"},`
    + `{k:3,pg:0,spell:"pushed",role:"op",op:"/"}]; refresh();`);
  const chain = run(`M.P.map(p => p.vec || p.shown)`);
  const tot = chain.reduce((s, v) => s + pairs(v), 0);
  ok(tot === 0, `a fully pushed chain still holds ${tot} 0,±1 pairs`);
  console.log(`  and through a four-card chain of all four operations: 0 pairs left`);
  run(`KS = [{k:3,pg:0,spell:"plain",role:"num",op:"+"},{k:5,pg:0,spell:"plain",role:"num",op:"+"},`
    + `{k:7,pg:0,spell:"plain",role:"num",op:"+"}]; refresh();`);
}
console.log("\nall good.");
