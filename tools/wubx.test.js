/* node tools/wubx.test.js — runs wubx.html for real, then pokes the parts the
   harness cannot reach on its own (paintRow lives behind querySelector). */
const fs = require("fs"), vm = require("vm"), path = require("path");
const {harness} = require(__dirname + "/domharness.js");

const page = __dirname + "/../wubx.html";
const html = fs.readFileSync(page, "utf8");
const {g} = harness();
const ctx = vm.createContext(g);
let src = "";
for(const m of html.matchAll(/<script[^>]*\bsrc="([^"]+)"/g)){
  const f = path.join(path.dirname(page), m[1].replace(/^\//, ""));
  if(fs.existsSync(f)) src += fs.readFileSync(f, "utf8") + "\n";
}
for(const m of html.matchAll(/<script(?![^>]*\bsrc=)[^>]*>([\s\S]*?)<\/script>/g)) src += m[1] + "\n";
vm.runInContext(src, ctx, {filename: "wubx.html"});
const run = code => vm.runInContext(code, ctx);
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* 1. the default rack builds one grid per step */
run(`KS = [{k:3,push:false,op:.55},{k:10,push:false,op:.55},{k:200,push:false,op:.55}]; refresh();`);
ok(run("M.N") === 2, "3 operands -> 2 grids, got " + run("M.N"));
console.log(`  3 operands -> ${run("M.N")} grids, shapes `
  + run(`M.P.map(p => p.rows + "x" + p.cols).join(" then ")`));

/* 2. the phasor's value is the real product of the three */
{
  const got = run(`fmt(M.P[M.N-1].value)`);
  const want = run(`(() => {
    const f = ks => { const {v,neg} = parse(String(ks)); return hexValue(hexSequence(v,neg).raw); };
    let n = 1n, d = 1n;
    for(const k of [3,10,200]){ const x = f(k); n *= x.num; d *= x.den; }
    const gg = (a,b)=>{a=a<0n?-a:a;while(b){[a,b]=[b,a%b];}return a||1n;};
    const h = gg(n,d); return fmt({num:n/h, den:d/h});
  })()`);
  ok(got === want, `product ${got} != ${want}`);
  console.log(`  3 × 10 × 200 (as stalks) = ${got}, matching the operands multiplied out`);
}

/* 3. paintRow — the bit the harness never reaches. stub a row and call it. */
{
  const mk = () => {
    const body = {innerHTML: ""}, dot = {style: {}};
    return {row: {querySelector: s => s === ".body" ? body : dot}, body, dot};
  };
  const a = mk(); run("paintRow")(0, a.row);
  ok(/the seed/.test(a.body.innerHTML), "row 0 should say it is the seed");
  const b = mk(); run("paintRow")(1, b.row);
  const cells = (b.body.innerHTML.match(/<i class="[brg]/g) || []).length;
  const p = run("M.P[0]");
  const want = p.rows * p.cols + run("operandDigits(KS[1]).length");
  ok(cells === want, `grid drew ${cells} cells, wanted ${want}`);
  ok(/gbits/.test(b.body.innerHTML), "no grid element");
  const folds = (b.body.innerHTML.match(/ f"/g) || []).length;
  ok(folds === Math.min(p.rows, p.cols), `${folds} fold cells, wanted ${Math.min(p.rows, p.cols)}`);
  ok(b.dot.style.background, "row 1 should take the grid's colour");
  console.log(`  paintRow: ${p.rows}×${p.cols} grid drawn, ${folds} cells on the fold, dot coloured`);
}

/* 4. reordering. the product must not move; the grids must. */
{
  run(`KS = [{k:3,push:false,op:.55},{k:10,push:false,op:.55},{k:70000,push:false,op:.55}]; refresh();`);
  const v0 = run("fmt(M.P[M.N-1].value)"), s0 = run(`M.P.map(p=>p.rows+"x"+p.cols).join(",")`);
  const g0 = run("M.P.map(p => [p.inner,p.fold,p.outer]).flat().join(',')");
  run(`{ const t = KS[1]; KS[1] = KS[2]; KS[2] = t; } refresh();`);
  const v1 = run("fmt(M.P[M.N-1].value)"), s1 = run(`M.P.map(p=>p.rows+"x"+p.cols).join(",")`);
  const g1 = run("M.P.map(p => [p.inner,p.fold,p.outer]).flat().join(',')");
  ok(v0 === v1, `product moved on reorder: ${v0} -> ${v1}`);
  ok(s0 !== s1, "reorder should reshape the grids");
  ok(g0 !== g1, "reorder should move Inner/Fold/Outer");
  console.log(`  reorder 3,10,70000 -> 3,70000,10: product held at ${v0}`);
  console.log(`    shapes ${s0} -> ${s1}, and the regions moved with them`);
}

/* 5. a two-operand swap: the rectangle turns, the regions do not. this is the
      honest limit of what order does to a single pair. */
{
  run(`KS = [{k:5,push:false,op:.55},{k:1000,push:false,op:.55}]; refresh();`);
  const A = run(`JSON.stringify([M.P[0].rows, M.P[0].cols, M.P[0].inner, M.P[0].fold, M.P[0].outer, M.P[0].rateA, M.P[0].rateB])`);
  run(`{ const t = KS[0]; KS[0] = KS[1]; KS[1] = t; } refresh();`);
  const B = run(`JSON.stringify([M.P[0].rows, M.P[0].cols, M.P[0].inner, M.P[0].fold, M.P[0].outer, M.P[0].rateA, M.P[0].rateB])`);
  const a = JSON.parse(A), b = JSON.parse(B);
  ok(a[0] === b[1] && a[1] === b[0], "a swap should transpose the rectangle");
  ok(a[2] === b[2] && a[3] === b[3] && a[4] === b[4], "a swap should leave I/F/O alone");
  ok(a[5] !== b[5] || a[6] !== b[6], "but the rates should re-read");
  console.log(`  swapping one pair: ${a[0]}×${a[1]} -> ${b[0]}×${b[1]},`
    + ` I/F/O identical, rates ${a[5]},${a[6]} -> ${b[5]},${b[6]}`);
}

/* 6. the curve still closes and the sphere still has something to scale to */
{
  run(`KS = [{k:3,push:false,op:.55},{k:10,push:false,op:.55},{k:200,push:false,op:.55}]; refresh();`);
  const first = run("CURVE[0]"), last = run("CURVE[CURVE.length-1]");
  const d = Math.hypot(first.x - last.x, first.y - last.y, first.z - last.z);
  ok(d < 1e-9, `curve does not close, gap ${d}`);
  ok(run("EXTENT") > 0 && isFinite(run("EXTENT")), "EXTENT bad: " + run("EXTENT"));
  console.log(`  curve closes (gap ${d.toExponential(1)}), extent ${run("EXTENT").toExponential(2)}`);
}
console.log("\nall good.");
