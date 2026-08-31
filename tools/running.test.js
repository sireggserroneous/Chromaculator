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
