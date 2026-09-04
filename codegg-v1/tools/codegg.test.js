/* node codegg-v1/tools/codegg.test.js -- the value is the syndrome.
 *
 * Every claim was predicted in the plan before it was run. The two that carry
 * the thesis are #4 and #5: the flagged row burst codec-v1 could only detect
 * is corrected here, and the residues survive push while codec-v1's sums
 * break -- the difference between checking a spelling and checking a number.
 *
 * #7 is the unflattering one, kept on purpose: residues have a silent floor
 * and can miscorrect, failure modes codec-v1 does not have. They are measured
 * here and reported in the README either way. */
const G = require(__dirname + "/../codegg.js");
const V1 = require(__dirname + "/../../codec-v1/chromacode.js");
const fs = require("fs");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* the precision-safe PRNG this project settled on after the LCG incident */
function mul32(a){
  return function(){
    a |= 0; a = a + 0x6D2B79F5 | 0;
    let t = Math.imul(a ^ a >>> 15, 1 | a);
    t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
    return (t ^ t >>> 14) >>> 0;
  };
}
const g = mul32(20260831);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const same = (a, b) => a.length === b.length && Buffer.from(a).equals(Buffer.from(b));
const clone = p => ({meta: p.meta, code: p.code, checks: p.checks,
                     squares: p.squares.map(s => Int8Array.from(s))});

const N = 32, L = N * N;
const code = G.makeCode(N);

/* 1. the moduli really are injective -- enumerated here, independently of the
      codec's own check, because the whole scheme rests on it */
{
  for(const m of [code.p, code.q]){
    const seen = new Set();
    let pow = 1 % m, good = true;
    for(let k = 0; k < L; k++){
      const neg = m - pow;
      if(seen.has(pow) || seen.has(neg) || pow === neg){ good = false; break; }
      seen.add(pow); seen.add(neg);
      pow = (pow * 2) % m;
    }
    ok(good, `modulus ${m} is not injective over +-2^k, k<${L}`);
  }
  console.log(`  moduli p=${code.p}, q=${code.q}: {+-2^k mod m} enumerated distinct, k<${L}`);
}

/* 2. round-trip is exact on the awkward shapes */
{
  const cases = [
    ["empty", new Uint8Array(0)],
    ["one byte", Uint8Array.from([0xA7])],
    ["all zero", new Uint8Array(300)],
    ["all 0xFF", Uint8Array.from({length: 300}, () => 0xff)],
    ["partial final square", randBytes(1000)],
    ["exactly one square", randBytes(128)],
    ["many squares", randBytes(5000)],
  ];
  for(const [label, src] of cases){
    const out = G.decode(clone(G.encode(src, {N, code})));
    ok(same(out.bytes, src), `round-trip broke on ${label}`);
    ok(out.detected === 0 && out.corrected === 0, `${label} was not clean`);
  }
  console.log(`  round-trip exact: ${cases.length} shapes, all clean`);
}

/* 3. a single error is located, signed and repaired -- from ~3 bytes of check.
      All three corruptions a bit-cell can suffer: 0->1, 1->0, and ->-1 (which
      the bit alphabet reads as a sentinel erasure). */
{
  const src = randBytes(4096);
  let n = 0, fixedExact = 0;
  for(let t = 0; t < 3000; t++){
    const p = G.encode(src, {N, code});
    const s = g() % p.squares.length, i = g() % L;
    const cur = p.squares[s][i];
    const alts = [0, 1, -1].filter(v => v !== cur);
    p.squares[s][i] = alts[g() % alts.length];
    const out = G.decode(p);
    n++;
    if(out.corrected === 1 && same(out.bytes, src)) fixedExact++;
  }
  ok(fixedExact === n, `single error repaired ${fixedExact}/${n}`);
  console.log(`  single error: located, signed and repaired ${fixedExact}/${n}`);
}

/* 4. erasures -- the case codec-v1 admitted it could not repair.
      A 12-cell burst in one row, positions flagged, values randomised.
      codegg corrects it; codec-v1 on the same-shaped damage detects only. */
{
  const src = randBytes(4096);
  let n = 0, fixed = 0;
  for(let t = 0; t < 800; t++){
    const p = G.encode(src, {N, code});
    const s = g() % p.squares.length;
    const row = g() % N, c0 = g() % (N - 12);
    const flagged = [];
    for(let j = 0; j < 12; j++){
      const i = row * N + c0 + j;
      p.squares[s][i] = g() % 2;               // burst scribbles bits
      flagged.push(i);
    }
    const out = G.decode(p, {erased: new Map([[s, flagged]])});
    n++;
    if(same(out.bytes, src)) fixed++;
  }
  ok(fixed === n, `flagged row burst repaired ${fixed}/${n}`);
  console.log(`  flagged 12-cell row burst: corrected ${fixed}/${n}`);

  /* the same shape against codec-v1: one bad row, twelve bad columns, no
     perfect matching -- detected, never repaired. Measured, not asserted. */
  let v1n = 0, v1fixed = 0, v1det = 0;
  for(let t = 0; t < 200; t++){
    const p = V1.encode(src, {N, alphabet: "byte"});
    const s = g() % p.squares.length, row = g() % N, c0 = g() % (N - 12);
    for(let j = 0; j < 12; j++) p.squares[s][row][c0 + j] ^= 1 + (g() % 255);
    const out = V1.decode(p);
    v1n++;
    if(same(Array.from(out.bytes), Array.from(src))) v1fixed++;
    else if(out.detected > 0) v1det++;
  }
  ok(v1fixed === 0 && v1det === v1n, `codec-v1 burst behaviour changed: fixed ${v1fixed}, detected ${v1det}/${v1n}`);
  console.log(`  same burst on codec-v1: repaired ${v1fixed}/${v1n}, detected ${v1det}/${v1n} -- as its README admits`);

  /* sentinel erasures need no flags at all: a -1 cell announces itself */
  let sn = 0, sfixed = 0;
  for(let t = 0; t < 800; t++){
    const p = G.encode(src, {N, code});
    const s = g() % p.squares.length;
    const k = 1 + g() % 8;
    for(let j = 0; j < k; j++) p.squares[s][g() % L] = -1;
    const out = G.decode(p);
    sn++;
    if(same(out.bytes, src)) sfixed++;
  }
  ok(sfixed === sn, `sentinel erasures repaired ${sfixed}/${sn}`);
  console.log(`  sentinel -1 erasures, 1..8 per square, unflagged: repaired ${sfixed}/${sn}`);
}

/* 5. push invariance -- the thesis. Push conserves the value, so the residues
      do not move; codec-v1's sums are sums of symbols, so they do. */
{
  eval(fs.readFileSync(__dirname + "/../../stalk.js", "utf8"));   // pushLeft
  const src = randBytes(1024);
  const p = G.encode(src, {N, code});
  let invariant = 0, respelled = 0, v1broken = 0;
  for(let s = 0; s < p.squares.length; s++){
    const before = Array.from(p.squares[s]);
    const pushed = pushLeft(before);
    if(pushed.join() !== before.join()) respelled++;
    if(G.verify(pushed, p.checks[s], code)) invariant++;
    /* codec-v1's parity over the same respelling */
    const alph = V1.ALPHABETS.chroma;
    const a = V1.parities(V1.toSquares(before, N)[0], alph, N);
    const b = V1.parities(V1.toSquares(pushed, N)[0], alph, N);
    if(a.rows.join() !== b.rows.join() || a.cols.join() !== b.cols.join()
       || a.diags.join() !== b.diags.join()) v1broken++;
  }
  ok(invariant === p.squares.length, `residues moved under push: ${invariant}/${p.squares.length}`);
  ok(respelled === p.squares.length, "push respelled nothing -- test is vacuous");
  ok(v1broken === p.squares.length, `codec-v1 sums survived push on ${p.squares.length - v1broken} squares`);
  console.log(`  push respelled ${respelled}/${p.squares.length} squares:`
    + ` codegg residues invariant ${invariant}/${p.squares.length},`
    + ` codec-v1 sums broken ${v1broken}/${p.squares.length}`);
}

/* 6. overhead, from the format, head-to-head */
{
  const src = randBytes(4096);
  const eg = G.sizes(G.encode(src, {N, code}).meta);
  const v1 = V1.sizes(V1.encode(src, {N: 32, alphabet: "byte"}).meta);
  console.log(`  overhead on 4096 B: codegg ${(100 * eg.overhead).toFixed(2)}%`
    + ` (${eg.checkBytes} B of checks), codec-v1 byte ${(100 * v1.parityOverhead).toFixed(1)}%`
    + ` (${v1.parityBytes} B of parity) -- ${(v1.parityOverhead / eg.overhead).toFixed(1)}x less`);
  ok(eg.overhead < v1.parityOverhead / 4, "codegg overhead not under a quarter of codec-v1");
}

/* 7. the honest section: doubles, the silent floor, and miscorrection.
      codec-v1 never miscorrects; codegg can. Measured, and in the README. */
{
  const src = randBytes(1024);

  /* true double errors through the search path */
  let dn = 0, dfix = 0, ddet = 0, dmis = 0;
  for(let t = 0; t < 2000; t++){
    const p = G.encode(src, {N, code});
    const s = g() % p.squares.length;
    const i1 = g() % L; let i2 = g() % L;
    while(i2 === i1) i2 = g() % L;
    for(const i of [i1, i2]){
      const alts = [0, 1].filter(v => v !== p.squares[s][i]);
      p.squares[s][i] = alts[0];
    }
    const out = G.decode(p);
    dn++;
    if(same(out.bytes, src)) dfix++;
    else if(out.detected > 0) ddet++;
    else dmis++;
  }
  console.log(`  true double errors: corrected ${dfix}/${dn}, detected ${ddet}, miscorrected ${dmis}`);
  ok(dmis === 0, `double-error path miscorrected ${dmis} times`);

  /* 3..6 scattered errors: how often does the residue pair miss or lie?
     Single path only (doubles off) at volume, then the search path at less. */
  const storm = (errs, trials, doubles) => {
    let n = 0, silent = 0, mis = 0, det = 0;
    for(let t = 0; t < trials; t++){
      const p = G.encode(src, {N, code});
      const s = g() % p.squares.length;
      for(let e = 0; e < errs; e++){
        const i = g() % L;
        p.squares[s][i] = p.squares[s][i] === 1 ? 0 : 1;
      }
      const out = G.decode(p, {doubles});
      n++;
      const exact = same(out.bytes, src);
      if(exact) continue;                       // repeated index cancelled out
      if(out.clean === p.squares.length) silent++;
      else if(out.corrected > 0) mis++;
      else det++;
    }
    return {n, silent, mis, det};
  };
  const a = storm(3, 60000, false);
  console.log(`  3 errors x ${a.n} squares, singles only: detected ${a.det},`
    + ` miscorrected ${a.mis} (${(100 * a.mis / a.n).toFixed(3)}%), silent ${a.silent}`);
  const b = storm(5, 4000, true);
  console.log(`  5 errors x ${b.n} squares, search on:    detected ${b.det},`
    + ` miscorrected ${b.mis} (${(100 * b.mis / b.n).toFixed(3)}%), silent ${b.silent}`);
  console.log(`    (analytic silent floor ~1/(p*q) = ${(1 / (code.p * code.q)).toExponential(1)};`
    + ` codec-v1 has neither failure mode)`);
}

/* 8. the page's decision functions, driven directly through the harness */
{
  const {loadPage} = require(__dirname + "/../../tools/domharness.js");
  const {run} = loadPage(__dirname + "/../codegg.html");

  const r = run(`(() => {
    const out = {};
    build();                                    // fresh page square
    out.clean = diagnose().status;
    const i = 3 * PN + 5;
    cells[i] = cells[i] === 1 ? 0 : 1;          // one flipped bit
    const d = diagnose();
    out.oneStatus = d.status;
    out.hit = d.hits && d.hits.length === 1 ? d.hits[0] : null;
    cells[i] = cells[i] === 1 ? 0 : 1;          // undo
    cells[7] = -1;                              // a sentinel erasure
    out.erasure = diagnose().status;
    return out;
  })()`);
  ok(r.clean === "clean", `page: untouched square read ${r.clean}`);
  ok(r.oneStatus === "located", `page: single error read ${r.oneStatus}`);
  ok(r.hit && r.hit.i === 3 * r.hit.n + 5 || r.hit.i === 3 * 8 + 5,
    `page: located the wrong cell: ${JSON.stringify(r.hit)}`);
  ok(r.erasure === "erasure", `page: -1 cell read ${r.erasure}`);
  console.log(`  page diagnose(): clean / located(cell ${r.hit.i}, d ${r.hit.d}) / erasure -- all correct`);
}

console.log("codegg ok");
