/* node tools/product.test.js — the product grid says what it claims. */
const fs = require("fs");
eval(fs.readFileSync(__dirname + "/../stalk.js", "utf8"));

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const digs = k => { const {v, neg} = parse(String(k)); return hexSequence(v, neg).raw; };
const val  = d => { const f = hexValue(d); return Number(f.num) / Number(f.den); };
const R = (n, d) => { const g = (a,b)=>b?g(b,a%b):a; const h = g(Math.abs(n), d); return [n/h, d/h]; };

/* 1. the rectangle sums to the product, for every pair in range */
let bad = 0;
for(let a = 0; a <= 40; a++) for(let b = -40; b <= 40; b++){
  const A = digs(a), B = digs(b), P = hexProduct(A, B);
  const f = productValue(P);
  const want = R(Number(BigInt(hexValue(A).num) * BigInt(hexValue(B).num)),
                 Number(BigInt(hexValue(A).den) * BigInt(hexValue(B).den)));
  if(Number(f.num) !== want[0] || Number(f.den) !== want[1]) bad++;
}
ok(bad === 0, `sum == product (${bad} mismatched)`);
console.log("  rectangle sums to A*B exactly           3321 pairs, 0 wrong");

/* 2. shape: |A| columns, |B| rows */
{
  const A = digs(5), B = digs(1000);            // 4 cells, 12 cells
  const P = hexProduct(A, B);
  ok(P.cols === 4 && P.rows === 12, `shape got ${P.rows}x${P.cols}`);
  const Q = hexProduct(B, A);
  ok(Q.cols === 12 && Q.rows === 4, `swapped shape got ${Q.rows}x${Q.cols}`);
  console.log(`  4-cell x 12-cell -> ${P.rows} rows x ${P.cols} cols; swapped -> ${Q.rows} x ${Q.cols}`);
}

/* 3. the fold reaches the last cell of the shorter operand, both ways round */
{
  let n = 0;
  for(const [a, b] of [[5, 1000], [1000, 5], [3, 3], [7, 300], [65535, 2]]){
    const P = hexProduct(digs(a), digs(b));
    const L = Math.min(P.rows, P.cols);
    ok(P.foldAt === L - 1, "fold index");
    ok(P.foldAt < P.cols && P.foldAt < P.rows, "fold touches both edges");
    const F = productRegions(P).fold;
    ok(F.some(s => s.c === L - 1 && s.r === 0), "fold hits last column of a short A");
    ok(F.some(s => s.r === L - 1 && s.c === 0), "fold hits last row of a short B");
    n++;
  }
  console.log(`  fold lands on the shorter operand's last cell   ${n}/${n}`);
}

/* 4. the three regions still add back up to the whole */
{
  let worst = 0;
  for(let a = 1; a <= 30; a++) for(let b = 1; b <= 30; b++){
    const P = hexProduct(digs(a), digs(b)), G = productRegions(P);
    const s = c => c.reduce((t, x) => t + x.v * Math.pow(2, -x.w), 0);
    const f = productValue(P);
    worst = Math.max(worst, Math.abs(s(G.inner) + s(G.fold) + s(G.outer)
                                     - Number(f.num) / Number(f.den)));
  }
  ok(worst < 1e-12, `regions sum, off by ${worst}`);
  console.log(`  Inner + Fold + Outer == value            worst gap ${worst.toExponential(1)}`);
}

/* 5. reading the grid back out as a stalk keeps the value — so a product
      can be the next operand */
{
  let bad = 0;
  for(let a = -30; a <= 30; a++) for(let b = -30; b <= 30; b++){
    const P = hexProduct(digs(a), digs(b));
    const f = productValue(P), d = productDigits(P), g = hexValue(d);
    if(g.num !== f.num || g.den !== f.den) bad++;
  }
  ok(bad === 0, `read-back (${bad} wrong)`);
  console.log("  grid read back as a stalk keeps its value   3721 pairs, 0 wrong");
}

/* 6. what a swap actually changes, and what it does not. every quantity the
      sphere uses depends only on r+c, and transposing preserves r+c. */
{
  let sameGeom = 0, sameCells = 0, n = 0;
  const s = c => c.reduce((t, x) => t + x.v * Math.pow(2, -x.w), 0);
  for(let a = 1; a <= 25; a++) for(let b = 1; b <= 25; b++){
    const P = hexProduct(digs(a), digs(b)), Q = hexProduct(digs(b), digs(a));
    const gp = productRegions(P), gq = productRegions(Q);
    const eq = ["inner", "fold", "outer"].every(r => Math.abs(s(gp[r]) - s(gq[r])) < 1e-15);
    if(eq) sameGeom++;
    if(P.rows === Q.rows && P.cols === Q.cols) sameCells++;
    n++;
  }
  ok(sameGeom === n, "swap should leave I/F/O alone");
  console.log(`  swapping a pair leaves I/F/O identical   ${sameGeom}/${n}`
    + ` (only the rectangle turns; ${n - sameCells} of ${n} change shape)`);
}

/* 7. ...so order has to show up in the chain. (a*b)*c vs (a*c)*b: same value,
      different last grid. */
{
  const chain = ks => {
    let acc = digs(ks[0]), last = null;
    for(let i = 1; i < ks.length; i++){ last = hexProduct(acc, digs(ks[i])); acc = productDigits(last); }
    return last;
  };
  let sameVal = 0, diffShape = 0, n = 0;
  const POOL = [3, 10, 200, 5000, 70000, 9, 4095];   // 4, 8, 12, 16 and 20 cells
  for(const a of POOL) for(const b of POOL) for(const c of POOL){
    const P = chain([a, b, c]), Q = chain([a, c, b]);
    const fp = productValue(P), fq = productValue(Q);
    if(fp.num === fq.num && fp.den === fq.den) sameVal++;
    if(P.rows !== Q.rows || P.cols !== Q.cols) diffShape++;
    n++;
  }
  ok(sameVal === n, "the value must not depend on order");
  console.log(`  chain: value order-independent ${sameVal}/${n},`
    + ` final grid reshaped by reorder ${diffShape}/${n}`);
  /* and when the shape moves, the fold moves with it, so the sphere moves */
  let diffGeom = 0;
  const ss = c => c.reduce((t, x) => t + x.v * Math.pow(2, -x.w), 0);
  for(const a of POOL) for(const b of POOL) for(const c of POOL){
    const P = chain([a, b, c]), Q = chain([a, c, b]);
    const gp = productRegions(P), gq = productRegions(Q);
    if(["inner","fold","outer"].some(r => Math.abs(ss(gp[r]) - ss(gq[r])) > 1e-15)) diffGeom++;
  }
  console.log(`  chain: reorder moves Inner/Fold/Outer   ${diffGeom}/${n}`);
}
console.log("\nall good.");
