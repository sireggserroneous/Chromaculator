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

/* spreadRight — the mirror of push, and the only thing that reaches the greens
   push leaves behind. Exact only in the limit, hence the bar. */
{
  const digs3 = k => { const {v, neg} = parse(String(k)); return hexSequence(v, neg).raw; };
  const G2 = (a,b) => { a = a<0n?-a:a; while(b){ [a,b] = [b, a % b]; } return a || 1n; };
  const red2 = (n,d) => { const h = G2(n,d); return [n/h, d/h]; };
  let bad = 0, n = 0, barred = 0, trailing = 0, after = 0;
  for(let k = -3000; k <= 3000; k++){
    const p = pushLeft(digs3(k)), r = spreadRight(p);
    n++; if(r.bar) barred++;
    /* the finite reading plus the stated shortfall must be the true value */
    const v = hexValue(p), f = hexValue(r.d);
    const [sn, sd] = red2(f.num * r.short.den + r.short.num * f.den, f.den * r.short.den);
    if(sn !== v.num || sd !== v.den) bad++;
    /* push leaves a RUN of greens on the right; spread leaves at most one */
    let t = 0; for(let i = p.length - 1; i >= 0 && p[i] === 0; i--) t++;
    trailing += t;
    let a = 0; for(let i = r.d.length - 1; i >= 0 && r.d[i] === 0; i--) a++;
    after += a;
    if(r.bar && a !== 0) bad++;                       // a barred tail is never green
  }
  ok(bad === 0, `spreadRight wrong on ${bad} of ${n}`);
  ok(after < trailing, `spread should reduce trailing greens: ${trailing} -> ${after}`);
  console.log(`  spreadRight: ${n - bad}/${n} exact in the limit, ${barred} needed a bar,`
    + ` trailing greens ${trailing} -> ${after}`);

  /* and a second push really is a no-op, in either scan direction */
  let same = 0, canon = 0;
  const ltr = c => { const d = c.slice(); let m = true;
    while(m){ m = false; for(let i = 1; i < d.length; i++)
      if(d[i] !== 0 && d[i-1] === 0){ d[i-1] = d[i]; d[i] = -d[i]; m = true; } } return d; };
  for(let k = -3000; k <= 3000; k++){
    const p = pushLeft(digs3(k));
    if(pushLeft(p).join(",") === p.join(",")) same++;
    if(ltr(digs3(k)).join(",") === p.join(",")) canon++;
  }
  ok(same === 6001 && canon === 6001, "push must be an idempotent, canonical fixpoint");
  console.log(`  push: idempotent ${same}/6001, same fixpoint scanned either way ${canon}/6001`);
}

/* the cascade: our squash is a convolution, and arcs() is a row of Pascal's
   triangle. Recorded because the Inspirations page and spec.md now claim it. */
{
  const ones = n => Array(n).fill(1);
  const conv = (a, b) => { const r = new Array(a.length + b.length - 1).fill(0);
    for(let i = 0; i < a.length; i++) for(let j = 0; j < b.length; j++) r[i+j] += a[i] * b[j];
    return r; };
  const C = (n, k) => { let r = 1; for(let i = 0; i < k; i++) r = r * (n - i) / (i + 1);
    return Math.round(r); };

  let bad = 0, n = 0;
  for(let a = 1; a <= 9; a++) for(let b = 1; b <= 9; b++){
    const A = ones(a), B = ones(b);
    n++;
    if(squashDiagonals(hexProduct(A, B)).join(",") !== conv(B, A).join(",")) bad++;
  }
  ok(bad === 0, `the squash is not a convolution on ${bad} of ${n} shapes`);

  let bad2 = 0, n2 = 0;
  for(let m = 1; m <= 12; m++){
    n2++;
    if(arcs(m).join(",") !== squashDiagonals(hexProduct(ones(m), ones(m))).join(",")) bad2++;
  }
  ok(bad2 === 0, `arcs(n) is not the all-ones squash on ${bad2} of ${n2}`);

  /* and k-fold convolution of all-ones is a diagonal of Pascal */
  let bad3 = 0;
  for(let k = 2; k <= 6; k++){
    let v = ones(40);
    for(let i = 1; i < k; i++) v = conv(v, ones(40));
    for(let i = 0; i < 7; i++) if(v[i] !== C(i + k - 1, k - 1)) bad3++;
  }
  ok(bad3 === 0, `${bad3} binomials wrong in the all-ones cascade`);
  console.log(`  cascade: squash == convolution ${n}/${n}, arcs(n) == the all-ones squash ${n2}/${n2},`
    + ` k-fold all-ones == C(i+k-1,k-1) for k = 2..6`);

  /* the register facts the Atlas rings rest on */
  let bad4 = 0, n4 = 0;
  for(const p of [3, 4, 5, 7, 8, 11]){
    const D = (1 << p) - 1;
    for(let x = 1; x < D; x++) for(let m = 1; m <= p; m++){
      const rot = ((x << m) | (x >> (p - m))) & D;
      const mul = (x * Math.pow(2, m)) % D;
      n4++;
      if(rot !== (mul === 0 ? D : mul)) bad4++;
    }
  }
  ok(bad4 === 0, `rotation != multiplication by 2^m mod 2^p-1 on ${bad4} of ${n4}`);
  let bad5 = 0, n5 = 0;
  for(const p of [3, 5, 8, 11]){
    const D = (1 << p) - 1;
    for(let x = 0; x <= D; x++){ n5++; if((D - x) !== ((~x) & D)) bad5++; }
  }
  ok(bad5 === 0, `D-x != ~x on ${bad5} of ${n5}`);
  console.log(`  the ring: x·2^m mod 2^p−1 is a rotation ${n4}/${n4},`
    + ` and D−x is the complement ${n5}/${n5}`);
}
