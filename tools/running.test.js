/* node tools/running.test.js — the running value all four operations share. */
const fs = require("fs");
eval(fs.readFileSync(__dirname + "/../stalk.js", "utf8"));
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const digs = k => { const {v, neg} = parse(String(k)); return hexSequence(v, neg).raw; };
const ABS = x => x < 0n ? -x : x;
const G = (a,b) => { a = ABS(a); while(b){ [a,b] = [b, a % b]; } return a || 1n; };
const red = (n,d) => { if(d < 0n){ n = -n; d = -d; } const h = G(n,d); return [n/h, d/h]; };
const isPow2 = d => (d & (d - 1n)) === 0n;

/* 1. stalkFrac and fracToStalk are inverses, and the mantissa is normalised */
{
  let bad = 0, unlit = 0, out = 0, n = 0;
  for(let k = -400; k <= 400; k++) for(const E of [-9, -3, -1, 0, 1, 4, 11]){
    const d = digs(k), f = stalkFrac(d, E);
    const s = fracToStalk(f.num, f.den);
    const back = stalkFrac(s.d, s.E);
    if(back.num !== f.num || back.den !== f.den) bad++;
    if(f.num !== 0n){
      if(s.d[0] === 0) unlit++;                       // must lead with a lit cell
      const mv = hexValue(s.d);
      if(ABS(mv.num) >= mv.den) out++;                // mantissa must stay inside (-1,1)
    }
    n++;
  }
  ok(bad === 0, `${bad} round trips lost the value`);
  ok(unlit === 0, `${unlit} mantissas wasted their leading cell`);
  ok(out === 0, `${out} mantissas escaped (-1,1)`);
  console.log(`  round trip 2^E·stalk -> fraction -> 2^E·stalk   ${n - bad}/${n}`);
  console.log(`  mantissa inside (−1,1) and leading cell lit     ${n}/${n}`);
}

/* 2. +, − and × close on the dyadics, so they round-trip exactly through the
      running pair. ÷ does not — a quotient of dyadics is usually not dyadic —
      which is the whole reason it carries a remainder instead. */
{
  const ops = {
    "+": (a, b) => red(a.num * b.den + b.num * a.den, a.den * b.den),
    "-": (a, b) => red(a.num * b.den - b.num * a.den, a.den * b.den),
    "*": (a, b) => red(a.num * b.num, a.den * b.den),
  };
  const tally = {};
  for(const k of Object.keys(ops)) tally[k] = {ok: 0, n: 0, rings: new Set()};
  for(let a = -60; a <= 60; a++) for(let b = -60; b <= 60; b += 7){
    const A = digs(a), B = digs(b);
    for(const [sym, f] of Object.entries(ops)) for(const E of [-2, 0, 3]){
      const want = f(stalkFrac(A, E), stalkFrac(B, 0));
      const t = tally[sym]; t.n++;
      const s = fracToStalk(want[0], want[1]);
      const got = stalkFrac(s.d, s.E);
      if(got.num === want[0] && got.den === want[1]) t.ok++;
      t.rings.add(s.E);
    }
  }
  for(const [sym, t] of Object.entries(tally)){
    ok(t.ok === t.n, `${sym} lost ${t.n - t.ok} of ${t.n}`);
    const r = [...t.rings].sort((x, y) => x - y);
    console.log(`  ${sym}  ${t.ok}/${t.n} exact through the running pair   rings ${r[0]} .. ${r[r.length - 1]}`);
  }
}

/* 2b. division: how often the quotient is even representable, and that the
       remainder makes it exact anyway */
{
  let dyadic = 0, n = 0, bad = 0;
  for(let a = -60; a <= 60; a++) for(let b = -60; b <= 60; b += 7){
    const A = digs(a), B = digs(b);
    if(hexValue(B).num === 0n) continue;
    const av = stalkFrac(A, 0), bv = stalkFrac(B, 0);
    const [qn, qd] = red(av.num * bv.den, av.den * bv.num);
    n++;
    if(isPow2(qd < 0n ? -qd : qd)) dyadic++;
    /* the identity holds regardless */
    const d = divide(A, B, 16), qv = hexValue(d.Q);
    let [ln, ld] = red(qv.num * bv.num, qv.den * bv.den);
    if(d.e >= 0) ln <<= BigInt(d.e); else ld <<= BigInt(-d.e);
    [ln, ld] = red(ln, ld);
    const [sn, sd] = red(ln * d.R.den + d.R.num * ld, ld * d.R.den);
    if(sn !== av.num || sd !== av.den) bad++;
  }
  ok(bad === 0, `${bad} divisions broke the identity`);
  console.log(`  ÷  only ${dyadic}/${n} quotients are dyadic at all — the rest cannot be a`);
  console.log(`     stalk on any ring, which is why ÷ carries R. Identity held ${n}/${n}.`);
}

/* 3. addition really does leave the disc — which is why it needs a ring too */
{
  const a = digs(255), b = digs(255);                  // 255/256 each
  const f = red(stalkFrac(a, 0).num * 1n + stalkFrac(b, 0).num, stalkFrac(a, 0).den);
  const s = fracToStalk(f[0], f[1]);
  ok(s.E > 0, `255/256 + 255/256 should need a ring, got 2^${s.E}`);
  console.log(`  255/256 + 255/256 = ${f[0]}/${f[1]} = 2^${s.E} × `
    + `${hexValue(s.d).num}/${hexValue(s.d).den} — outside the disc, so the ring carries it`);
}
console.log("\nall good.");

/* alignByWeight — the one walk that addition, subtraction and the composite of
   a whole rack all share. Checked directly against the arithmetic. */
{
  const digs2 = k => { const {v, neg} = parse(String(k)); return hexSequence(v, neg).raw; };
  let bad = 0, n = 0, owedTot = 0;
  const NUMS = [3, -3, 7, -7, 200, -200, 255, -255, 1, -1, 4095];
  const RINGS = [-3, 0, 2];
  for(const a of NUMS) for(const b of NUMS) for(const E of RINGS) for(const sign of [1, -1]){
    const A = {d: digs2(a), E, sign: 1}, B = {d: digs2(b), E: 0, sign};
    const r = alignByWeight([A, B]);
    const fa = stalkFrac(A.d, E), fb = stalkFrac(B.d, 0);
    const wn = fa.num * fb.den + BigInt(sign) * fb.num * fa.den, wd = fa.den * fb.den;
    const g = (x, y) => { x = x < 0n ? -x : x; while(y){ [x, y] = [y, x % y]; } return x || 1n; };
    const h = g(wn, wd);
    n++; owedTot += r.owed;
    const back = stalkFrac(r.stalk.d, r.stalk.E);
    if(r.num !== wn / h || r.den !== wd / h) bad++;
    else if(back.num !== r.num || back.den !== r.den) bad++;
    else if(!r.ws.every((w, i) => i === 0 || w === r.ws[i-1] + 1)) bad++;
    else if(!r.sums.every((v, i) => v === r.rows[0][i] + r.rows[1][i])) bad++;
    else if(!r.rows.every(row => row.every(v => v >= -1 && v <= 1))) bad++;
  }
  ok(bad === 0, `alignByWeight wrong on ${bad} of ${n}`);
  console.log(`  alignByWeight: ${n - bad}/${n} exact across signs and rings —`
    + ` columns contiguous, sums = the rows, inputs stay digits, ${owedTot} carries owed`);

  /* and with more than two sources, which is what a whole rack is */
  let bad2 = 0, n2 = 0;
  for(let t = 0; t < 200; t++){
    const src = [0,1,2,3].map(j => ({d: digs2(NUMS[(t*3+j*5) % NUMS.length]),
                                     E: RINGS[(t+j) % RINGS.length]}));
    const r = alignByWeight(src);
    let wn = 0n, wd = 1n;
    for(const s of src){ const f = stalkFrac(s.d, s.E); wn = wn * f.den + f.num * wd; wd = wd * f.den; }
    const g = (x, y) => { x = x < 0n ? -x : x; while(y){ [x, y] = [y, x % y]; } return x || 1n; };
    const h = g(wn, wd);
    n2++;
    if(r.num !== wn / h || r.den !== wd / h) bad2++;
    else if(r.rows.length !== src.length) bad2++;
  }
  ok(bad2 === 0, `alignByWeight wrong on ${bad2} of ${n2} four-source racks`);
  console.log(`  alignByWeight: ${n2 - bad2}/${n2} four-source racks, one row kept per source`);
}
