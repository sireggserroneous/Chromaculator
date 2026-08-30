/* node tools/bulk.test.js — 10,000 integers through both grids.
   Every invariant the site claims for × and ÷, checked in exact rationals.
   Nothing here uses floats except where noted. */
const fs = require("fs");
eval(fs.readFileSync(__dirname + "/../stalk.js", "utf8"));

const glyph = v => v > 0 ? "1" : v < 0 ? "−" : "0";
const ABS = x => x < 0n ? -x : x;
const G = (a, b) => { a = ABS(a); while(b){ [a, b] = [b, a % b]; } return a || 1n; };
const red = (n, d) => { if(d < 0n){ n = -n; d = -d; } const h = G(n, d); return [n / h, d / h]; };
const digs = k => { const {v, neg} = parse(String(k)); return hexSequence(v, neg).raw; };
const isPow2 = d => (d & (d - 1n)) === 0n;
const eqf = (a, b) => a.num === b.num && a.den === b.den;

/* ---- the population: 10,000 integers, deliberately not all the same size ---- */
const N = 10000;
const POP = [];
for(let i = 1; i <= N; i++){
  /* a spread of magnitudes: most small, a tail out past 2^40, and a third of
     them negative, so sign and width are both exercised */
  const band = i % 7;
  let k = band === 0 ? i
        : band === 1 ? i * 7 + 1
        : band === 2 ? i * 1021
        : band === 3 ? i * 65537
        : band === 4 ? (1 << (i % 30)) + (i % 17)
        : band === 5 ? Number(BigInt(i) * 1048583n)
        : (i % 255) + 1;
  if(i % 3 === 0) k = -k;
  POP.push(k);
}
const partner = i => POP[(i * 7919 + 13) % N];   // a coprime stride, so pairs vary

const tally = {};
const check = (name, cond) => {
  const t = tally[name] || (tally[name] = {ok: 0, n: 0, first: null});
  t.n++;
  if(cond) t.ok++; else if(t.first === null) t.first = t.n;
};

/* ================= multiplication ================= */
let t0 = Date.now();
for(let i = 0; i < N; i++){
  const a = POP[i], b = partner(i);
  const A = digs(a), B = digs(b), P = hexProduct(A, B);
  const av = hexValue(A), bv = hexValue(B);

  // 1. the rectangle sums to A*B, exactly
  const want = (() => { const [n, d] = red(av.num * bv.num, av.den * bv.den); return {num: n, den: d}; })();
  check("× rectangle sums to A·B", eqf(productValue(P), want));

  // 2. shape: A along the columns, B down the rows
  check("× shape is |B| rows by |A| cols", P.rows === B.length && P.cols === A.length);

  // 3. the fold is the anti-diagonal through the shorter operand's last cell
  const L = Math.min(P.rows, P.cols);
  const R = productRegions(P);
  check("× fold index is min(rows,cols)−1", P.foldAt === L - 1);
  check("× fold reaches both edges",
    R.fold.some(s => s.r === 0 && s.c === L - 1) && R.fold.some(s => s.r === L - 1 && s.c === 0));
  check("× fold cell count is min(rows,cols)", R.fold.length === L);

  // 4. every cell is a signed digit, and lands on the weight it claims
  check("× cells are digits in {−1,0,1}", P.cells.every(v => v === -1 || v === 0 || v === 1));
  check("× cell (r,c) equals A[c]·B[r]",
    P.cells.every((v, j) => v === A[j % P.cols] * B[Math.floor(j / P.cols)]));

  // 5. Inner + Fold + Outer reconstructs the value, in exact rationals
  const regionSum = g => {
    let n = 0n; const D = P.rows + P.cols;
    for(const s of g) n += BigInt(s.v) * (1n << BigInt(D - s.w));
    return n;
  };
  const D = P.rows + P.cols;
  const total = regionSum(R.inner) + regionSum(R.fold) + regionSum(R.outer);
  const [tn, td] = red(total, 1n << BigInt(D));
  check("× Inner+Fold+Outer == value", eqf({num: tn, den: td}, productValue(P)));

  // 6. squash: the anti-diagonals are the place values
  const S = squashDiagonals(P);
  check("× squash has rows+cols−1 entries", S.length === P.rows + P.cols - 1);
  let sn = 0n;
  S.forEach((v, d) => sn += BigInt(v) * (1n << BigInt(D - (d + 2))));
  const [qn, qd] = red(sn, 1n << BigInt(D));
  check("× squash reconstructs the product", eqf({num: qn, den: qd}, productValue(P)));

  // 7. reconcile == productDigits, and it is the same number
  const V = productDigits(P);
  check("× squash+reconcile == productDigits", eqf(hexValue(V), productValue(P)));
  check("× vector length is rows+cols", V.length === D);

  // 8. push conserves the squashed vector
  const U = pushLeft(V);
  check("× push conserves the vector", eqf(hexValue(U), hexValue(V)));
  check("× pushed is still signed digits", U.every(v => v === -1 || v === 0 || v === 1));

  // 9. a swap transposes the rectangle and leaves the regions alone
  const Pt = hexProduct(B, A), Rt = productRegions(Pt);
  check("× swap transposes the shape", Pt.rows === P.cols && Pt.cols === P.rows);
  check("× swap leaves Inner/Fold/Outer identical",
    regionSum(R.inner) === (() => { let n = 0n; for(const s of Rt.inner) n += BigInt(s.v) * (1n << BigInt(D - s.w)); return n; })()
    && regionSum(R.outer) === (() => { let n = 0n; for(const s of Rt.outer) n += BigInt(s.v) * (1n << BigInt(D - s.w)); return n; })());
}
const tMul = Date.now() - t0;

/* ================= division ================= */
t0 = Date.now();
let widened = 0, exactRan = 0;
for(let i = 0; i < N; i++){
  const a = POP[i], b = partner(i);
  const A = digs(a), B = digs(b);
  if(hexValue(B).num === 0n) continue;
  const W = 4 + (i % 21);                          // 4..24 cells
  const d = divide(A, B, W);
  const av = hexValue(A), bv = hexValue(B), qv = hexValue(d.Q);

  // 1. the identity, in exact rationals
  let [ln, ld] = red(qv.num * bv.num, qv.den * bv.den);
  if(d.e >= 0) ln <<= BigInt(d.e); else ld <<= BigInt(-d.e);
  [ln, ld] = red(ln, ld);
  const [sn, sd] = red(ln * d.R.den + d.R.num * ld, ld * d.R.den);
  const [wn, wd] = red(av.num, av.den);
  check("÷ A = 2^e·Q·B + R", sn === wn && sd === wd);

  // 2. the parts are drawable
  check("÷ Q has exactly W cells", d.Q.length === W);
  check("÷ Q is inside (−1,1)", ABS(qv.num) < qv.den || qv.num === 0n);
  check("÷ Q's leading cell is lit", av.num === 0n || d.Q[0] !== 0);
  check("÷ R is dyadic", isPow2(d.R.den));
  check("÷ R is inside (−1,1)", ABS(d.R.num) < d.R.den || d.R.num === 0n);
  check("÷ R redraws as a stalk", eqf(hexValue(fracDigits(d.R)), d.R));

  // 3. the tableau is the multiplication rectangle of B and Q
  const T = hexProduct(B, d.Q);
  const [pn, pd] = red(qv.num * bv.num, qv.den * bv.den);
  check("÷ tableau == B × Q", eqf(productValue(T), {num: pn, den: pd}));
  check("÷ tableau is |Q| rows by |B| cols", T.rows === W && T.cols === B.length);

  // 4. push conserves the quotient
  check("÷ push conserves Q", eqf(hexValue(pushLeft(d.Q)), qv));

  // 5. widening trades R for Q and never breaks the identity
  const d2 = divide(A, B, W + 8);
  let [l2n, l2d] = red(hexValue(d2.Q).num * bv.num, hexValue(d2.Q).den * bv.den);
  if(d2.e >= 0) l2n <<= BigInt(d2.e); else l2d <<= BigInt(-d2.e);
  [l2n, l2d] = red(l2n, l2d);
  const [s2n, s2d] = red(l2n * d2.R.den + d2.R.num * l2d, l2d * d2.R.den);
  check("÷ identity holds at W+8 too", s2n === wn && s2d === wd);
  check("÷ the multiplier does not move with width", d2.e === d.e);
  check("÷ R never grows when widened", ABS(d2.R.num) * d.R.den <= ABS(d.R.num) * d2.R.den);
  if(ABS(d2.R.num) * d.R.den < ABS(d.R.num) * d2.R.den) widened++;
  if(d.exact) exactRan++;
}
const tDiv = Date.now() - t0;

/* ================= the edges the population misses =================
   0, ±1, exact powers of two, all-ones, and the extremes. These are where a
   grid is most likely to degenerate: an empty stalk, a single lit cell, a fold
   with nowhere to sit. */
{
  const EDGE = [0, 1, -1, 2, -2, 8, 15, 16, 255, 256, 65535, 65536,
                4294967295, 4294967296, 1099511627775];
  let n = 0;
  for(const a of EDGE) for(const b of EDGE){
    const A = digs(a), B = digs(b), P = hexProduct(A, B);
    const av = hexValue(A), bv = hexValue(B);
    const [wn, wd] = red(av.num * bv.num, av.den * bv.den);
    check("edge × rectangle sums to A·B", eqf(productValue(P), {num: wn, den: wd}));
    check("edge × fold sits inside the rectangle",
      P.foldAt >= 0 && P.foldAt < P.rows && P.foldAt < P.cols);
    check("edge × squash+reconcile == value", eqf(hexValue(productDigits(P)), productValue(P)));
    check("edge × push conserves the vector",
      eqf(hexValue(pushLeft(productDigits(P))), productValue(P)));

    if(bv.num === 0n){
      check("edge ÷ by zero returns null", divide(A, B, 8) === null);
      n++; continue;
    }
    const d = divide(A, B, 12), qv = hexValue(d.Q);
    let [ln, ld] = red(qv.num * bv.num, qv.den * bv.den);
    if(d.e >= 0) ln <<= BigInt(d.e); else ld <<= BigInt(-d.e);
    [ln, ld] = red(ln, ld);
    const [sn2, sd2] = red(ln * d.R.den + d.R.num * ld, ld * d.R.den);
    const [an, ad] = red(av.num, av.den);
    check("edge ÷ A = 2^e·Q·B + R", sn2 === an && sd2 === ad);
    check("edge ÷ R is dyadic and in range", isPow2(d.R.den) && ABS(d.R.num) <= d.R.den);
    check("edge ÷ Q leads with a lit cell unless A is zero", av.num === 0n || d.Q[0] !== 0);
    n++;
  }
  console.log(`  edges: ${EDGE.length}×${EDGE.length} = ${n} pairs including 0, ±1,`
    + ` 2^k, 2^k−1 and 2^40−1\n`);
}

/* ================= chains, since order only bites past two =================
   1,000 chains of 5 operands: the product must not care about the order, the
   division chain must add its exponents, and both must stay exact. */
{
  let chains = 0;
  for(let i = 0; i < 1000; i++){
    const ks = [0, 1, 2, 3, 4].map(j => POP[(i * 31 + j * 977) % N]);

    // multiplication: value is order-independent, grids are not
    const runMul = order => {
      let acc = digs(order[0]), shapes = [];
      for(let j = 1; j < order.length; j++){
        const g = hexProduct(acc, digs(order[j]));
        shapes.push(g.rows + "x" + g.cols);
        acc = productDigits(g);
      }
      return {v: hexValue(acc), shapes: shapes.join(",")};
    };
    const fwd = runMul(ks), rev = runMul([ks[0], ...ks.slice(1).reverse()]);
    const want = (() => { let n = 1n, d = 1n;
      for(const k of ks){ const f = hexValue(digs(k)); n *= f.num; d *= f.den; }
      const [a, b] = red(n, d); return {num: a, den: b}; })();
    check("chain × product is exact", eqf(fwd.v, want));
    check("chain × product ignores order", eqf(fwd.v, rev.v));

    // division: mantissas carry, exponents add, every step exact
    let acc = digs(ks[0]), E = 0, ok = true;
    for(let j = 1; j < ks.length; j++){
      const B = digs(ks[j]);
      if(hexValue(B).num === 0n){ ok = false; break; }
      const d = divide(acc, B, 12);
      const av = hexValue(acc), bv = hexValue(B), qv = hexValue(d.Q);
      let [ln, ld] = red(qv.num * bv.num, qv.den * bv.den);
      if(d.e >= 0) ln <<= BigInt(d.e); else ld <<= BigInt(-d.e);
      [ln, ld] = red(ln, ld);
      const [sn3, sd3] = red(ln * d.R.den + d.R.num * ld, ld * d.R.den);
      const [an, ad] = red(av.num, av.den);
      check("chain ÷ every step is exact", sn3 === an && sd3 === ad);
      E += d.e;
      acc = d.Q;
    }
    if(ok){ check("chain ÷ ring stays finite", Number.isFinite(E) && Math.abs(E) < 4096); chains++; }
  }
  console.log(`  chains: ${chains} five-deep ÷ chains and 1,000 five-deep × chains\n`);
}

/* ================= report ================= */
const rows = Object.entries(tally);
const wide = Math.max(...rows.map(([k]) => k.length));
let failed = 0;
console.log(`  ${"invariant".padEnd(wide)}   passed / checked`);
console.log(`  ${"-".repeat(wide)}   ----------------`);
for(const [k, t] of rows){
  const bad = t.n - t.ok;
  if(bad) failed++;
  console.log(`  ${k.padEnd(wide)}   ${String(t.ok).padStart(6)} / ${String(t.n).padStart(6)}`
    + (bad ? `   FAILED ${bad}, first at case ${t.first}` : ""));
}
const checks = rows.reduce((s, [, t]) => s + t.n, 0);
console.log(`\n  ${N.toLocaleString()} integers, ${checks.toLocaleString()} checks`
  + `  ·  × ${tMul}ms, ÷ ${tDiv}ms`);
console.log(`  magnitudes ${Math.min(...POP.map(Math.abs))} … ${Math.max(...POP.map(Math.abs)).toLocaleString()}`
  + `, ${POP.filter(k => k < 0).length.toLocaleString()} negative`
  + `  ·  widest grid ${Math.max(...POP.map(k => digs(k).length))} cells`);
console.log(`  ÷ ran out exactly on ${exactRan.toLocaleString()} of ${N.toLocaleString()};`
  + ` widening shrank R on ${widened.toLocaleString()}`);
if(failed) { console.log(`\n${failed} invariant(s) FAILED.`); process.exit(1); }
console.log("\nall good.");
