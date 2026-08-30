/* node tools/wubdiv.test.js — Wub ÷ draws what the arithmetic says. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../wubdiv.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

const setRack = (ks, w) =>
  run(`WIDTH = ${w}; KS = ${JSON.stringify(ks.map(k => ({k, push: false, op: .55})))}; refresh();`);

/* the identity, checked in exact rationals against the page's own phasors:
   dividend  ==  2^e * Q * B + R,  step by step down the rack */
function identity(){
  return run(`(() => {
    const abs = x => x < 0n ? -x : x;
    const gg = (a,b) => { a = abs(a); while(b){ [a,b] = [b, a % b]; } return a || 1n; };
    const red = (n,d) => { if(d < 0n){ n = -n; d = -d; } const h = gg(n,d); return [n/h, d/h]; };
    let acc = operandDigits(KS[0]), bad = 0, seen = 0;
    for(const p of M.P){
      const av = hexValue(acc), bv = hexValue(p.B), qv = hexValue(p.Q);
      let [ln, ld] = red(qv.num * bv.num, qv.den * bv.den);
      if(p.e >= 0) ln <<= BigInt(p.e); else ld <<= BigInt(-p.e);
      [ln, ld] = red(ln, ld);
      const [sn, sd] = red(ln * p.rem.den + p.rem.num * ld, ld * p.rem.den);
      const [wn, wd] = red(av.num, av.den);
      if(sn !== wn || sd !== wd) bad++;
      seen++;
      acc = p.Q;
    }
    return {bad, seen};
  })()`);
}

/* 1. the chain builds, and every step's identity holds */
{
  setRack([3, 10, 200], 8);
  ok(run("M.N") === 2, "3 operands -> 2 divisions, got " + run("M.N"));
  const r = identity();
  ok(r.bad === 0 && r.seen === 2, `identity broke on ${r.bad} of ${r.seen}`);
  console.log(`  3 ÷ 10 ÷ 200 at 8 cells: ${run("M.N")} steps, `
    + run(`M.P.map(p => p.rows + "x" + p.cols + " ring 2^" + p.E).join("  ")`));
  console.log(`    A = 2^e·Q·B + R held on ${r.seen}/${r.seen} steps`);
}

/* 2. exponents add — the running ring is the sum of the step shifts */
{
  const es = run("M.P.map(p => p.e)"), Es = run("M.P.map(p => p.E)");
  let acc = 0;
  es.forEach((e, i) => { acc += e; ok(Es[i] === acc, `ring ${Es[i]} != running ${acc}`); });
  console.log(`  steps ${es.join(", ")} accumulate to rings ${Es.join(", ")}`);
}

/* 3. widening trades remainder for quotient and never breaks anything */
{
  let prev = null, bad = 0, grew = 0;
  const row = [];
  for(const W of [1, 2, 4, 8, 16, 24]){
    setRack([3, 10, 200], W);
    const r = identity(); if(r.bad) bad++;
    const worst = Math.max(...run(`M.P.map(p => Math.abs(Number(p.rem.num)) / Number(p.rem.den))`));
    if(prev !== null && worst > prev + 1e-18) grew++;
    row.push(`W=${W}:${worst.toExponential(1)}`);
    prev = worst;
  }
  ok(bad === 0 && grew === 0, `${bad} widths broke the identity, ${grew} grew the remainder`);
  console.log(`  widening: ${row.join("  ")}`);
  console.log(`    identity held at all 6 widths, remainder never grew`);
}

/* 4. the remainder dot walks back to its arm's base */
{
  const at = W => { setRack([3, 10, 200], W); return run("M.P.map(p => p.rratio)"); };
  const a = at(2), b = at(24);
  ok(a.every((v, i) => b[i] <= v), "rratio should not grow with width");
  ok(Math.max(...b) < 1e-4, `at 24 cells the dots should be home, worst ${Math.max(...b)}`);
  console.log(`  remainder dot: ${a.map(v => (v*100).toFixed(1) + "%").join(", ")} of its arm at 2 cells`
    + ` -> ${b.map(v => v.toExponential(1)).join(", ")} at 24`);
}

/* 5. rings scale the geometry; turning them off leaves the mantissa shape */
{
  setRack([3, 10, 200], 8);
  const on = run(`M.P.map(p => [p.inner, p.fold, p.outer, p.ring])`);
  run(`RINGS = false; refresh();`);
  const off = run(`M.P.map(p => [p.inner, p.fold, p.outer, p.ring])`);
  let bad = 0;
  on.forEach((o, i) => {
    ok(off[i][3] === 1, "ring should be 1 when off");
    for(const k of [0, 1, 2]) if(Math.abs(o[k] - off[i][k] * o[3]) > 1e-15 * Math.max(1, Math.abs(o[k]))) bad++;
  });
  ok(bad === 0, `${bad} components did not scale by 2^E`);
  run(`RINGS = true; refresh();`);
  console.log(`  rings: Inner/Fold/Outer scale by 2^E exactly, ${on.length * 3}/${on.length * 3} components`);
}

/* 6. paintRow — the harness cannot reach it, so call it with a stub */
{
  const mk = () => {
    const body = {innerHTML: ""}, dot = {style: {}};
    return {row: {querySelector: s => s === ".body" ? body : dot}, body, dot};
  };
  setRack([3, 10, 200], 8);
  const a = mk(); run("paintRow")(0, a.row);
  ok(/the dividend/.test(a.body.innerHTML), "row 0 should name itself the dividend");
  const b = mk(); run("paintRow")(1, b.row);
  const p = run("M.P[0]");
  ok(/gbits/.test(b.body.innerHTML), "no tableau drawn");
  ok(/>Q</.test(b.body.innerHTML) && /># R|>R</.test(b.body.innerHTML), "Q and R should both be shown");
  ok(/ring 2/.test(b.body.innerHTML), "the ring should be named");
  const folds = (b.body.innerHTML.match(/ f"/g) || []).length;
  ok(folds === Math.min(p.rows, p.cols), `${folds} fold cells, wanted ${Math.min(p.rows, p.cols)}`);
  ok(b.dot.style.background, "row should take its colour");
  console.log(`  paintRow: ${p.rows}×${p.cols} tableau, ${folds} on the fold, Q, ring and R all shown`);
}

/* 7. a zero divisor is skipped, not crashed into */
{
  setRack([3, 0, 200], 8);
  ok(run("M.N") === 1, `zero divisor should drop one step, got ${run("M.N")}`);
  const c = {innerHTML: ""};
  run("paintRow")(1, {querySelector: s => s === ".body" ? c : {style: {}}});
  ok(/nothing divides by green/.test(c.innerHTML), "row 1 should say why it is missing");
  console.log(`  a zero divisor drops its step and says so, rather than throwing`);
}

/* 8. the curve still closes */
{
  setRack([3, 10, 200], 8);
  const f = run("CURVE[0]"), l = run("CURVE[CURVE.length-1]");
  const d = Math.hypot(f.x - l.x, f.y - l.y, f.z - l.z);
  ok(d < 1e-9, `curve does not close, gap ${d}`);
  ok(isFinite(run("EXTENT")) && run("EXTENT") > 0, "EXTENT bad");
  console.log(`  curve closes (gap ${d.toExponential(1)}), extent ${run("EXTENT").toExponential(2)}`);
}
console.log("\nall good.");
