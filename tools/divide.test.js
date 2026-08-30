/* node tools/divide.test.js — A = 2^e * Q * B + R, exactly, at every width. */
const fs = require("fs");
eval(fs.readFileSync(__dirname + "/../stalk.js", "utf8"));
const glyph = v => v > 0 ? "1" : v < 0 ? "−" : "0";
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const digs = k => { const {v, neg} = parse(String(k)); return hexSequence(v, neg).raw; };
const ABS = x => x < 0n ? -x : x;
const G = (a,b) => { a = ABS(a); while(b){ [a,b] = [b, a % b]; } return a || 1n; };
const red = (n,d) => { if(d < 0n){ n = -n; d = -d; } const h = G(n,d); return [n/h, d/h]; };
const isPow2 = d => (d & (d - 1n)) === 0n;

/* the identity, in exact rationals — no floats anywhere near this */
function holds(A, B, W){
  const r = divide(A, B, W); if(!r) return null;
  const bv = hexValue(B), av = hexValue(A);
  let [ln, ld] = red(r.value.num * bv.num, r.value.den * bv.den);   // Q*B
  if(r.e >= 0) ln <<= BigInt(r.e); else ld <<= BigInt(-r.e);        // times 2^e
  [ln, ld] = red(ln, ld);
  const [sn, sd] = red(ln * r.R.den + r.R.num * ld, ld * r.R.den);  // plus R
  const [wn, wd] = red(av.num, av.den);
  return {r, holds: sn === wn && sd === wd};
}

/* 1. it holds, everywhere, at the natural width */
{
  let bad = 0, notDy = 0, out = 0, exact = 0, n = 0, lo = 99, hi = -99;
  for(let a = -80; a <= 80; a++) for(let b = -120; b <= 120; b++){
    const A = digs(a), B = digs(b);
    if(hexValue(B).num === 0n) continue;
    const h = holds(A, B, A.length + B.length); n++;
    if(!h.holds) bad++;
    if(!isPow2(h.r.R.den)) notDy++;
    if(ABS(h.r.R.num) >= h.r.R.den) out++;
    if(h.r.exact) exact++;
    lo = Math.min(lo, h.r.e); hi = Math.max(hi, h.r.e);
  }
  ok(bad === 0, `identity broke ${bad} times`);
  ok(notDy === 0, `${notDy} remainders were not dyadic`);
  ok(out === 0, `${out} remainders escaped (-1,1)`);
  console.log(`  A = 2^e x Q x B + R exact            ${n - bad}/${n}`);
  console.log(`  R is a dyadic, drawable as a stalk   ${n - notDy}/${n}`);
  console.log(`  R stays inside (-1,1)                ${n - out}/${n}`);
  console.log(`  ran out exactly (R = 0)              ${exact}/${n}, the rest keep a leftover`);
  console.log(`  multiplier e ranged                  ${lo} .. ${hi}`);
}

/* 2. Q is a stalk like any other: inside (-1,1), and normalised so the first
      cell is always lit — no width wasted on leading greens */
{
  let out = 0, dead = 0, n = 0;
  for(let a = -60; a <= 60; a++) for(let b = 1; b <= 90; b++){
    const r = divide(digs(a), digs(b), 12); n++;
    if(ABS(r.value.num) >= r.value.den) out++;
    if(hexValue(digs(a)).num !== 0n && r.Q[0] === 0) dead++;
  }
  ok(out === 0 && dead === 0, `${out} outside, ${dead} with a wasted leading cell`);
  console.log(`  Q inside (-1,1) and leading cell lit ${n - out - dead}/${n}`);
}

/* 3. widening only trades R for Q — the answer itself never moves */
{
  let bad = 0, shrunk = 0, n = 0;
  for(let a = 1; a <= 40; a++) for(let b = 1; b <= 60; b++){
    let prev = null;
    for(const W of [2, 4, 8, 16, 24]){
      const h = holds(digs(a), digs(b), W); n++;
      if(!h.holds) bad++;
      const mag = Number(ABS(h.r.R.num)) / Number(h.r.R.den);
      if(prev !== null && mag > prev + 1e-18) shrunk++;
      prev = mag;
    }
  }
  ok(bad === 0 && shrunk === 0, `${bad} broke, ${shrunk} grew their remainder`);
  console.log(`  holds at 5 widths, R never grows      ${n - bad - shrunk}/${n}`);
}

/* 4. the tableau is the multiplication rectangle, solved for instead of read */
{
  let bad = 0, n = 0;
  for(let a = 1; a <= 40; a++) for(let b = 1; b <= 60; b++){
    const A = digs(a), B = digs(b), r = divide(A, B, 8);
    const P = hexProduct(B, r.Q);            // rows = quotient cells, cols = B
    const f = productValue(P);
    const [qn, qd] = red(r.value.num * hexValue(B).num, r.value.den * hexValue(B).den);
    if(f.num !== qn || f.den !== qd) bad++;
    n++;
  }
  ok(bad === 0, `${bad} tableaus did not equal Q*B`);
  console.log(`  tableau == hexProduct(B, Q) == Q x B ${n - bad}/${n}`);
}

/* 5. the worked example */
{
  const A = digs(3), B = digs(10), g = f => `${f.num}/${f.den}`;
  console.log(`\n  3 -> ${A.map(glyph).join("")} = ${g(hexValue(A))}   `
    + `10 -> ${B.map(glyph).join("")} = ${g(hexValue(B))}   true quotient 3/10, repeats forever`);
  for(const W of [2, 4, 8, 16]){
    const r = divide(A, B, W);
    console.log(`    W=${String(W).padStart(2)}  Q=${r.Q.map(glyph).join("").padEnd(17)}`
      + `2^${String(r.e).padStart(2)}   R=${g(r.R).padEnd(12)}`
      + `R as a stalk ${fracDigits(r.R).map(glyph).join("")}`);
  }
}
console.log("\nall good.");
