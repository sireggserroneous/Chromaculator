/* node eggSo-v0/tools/eggso.test.js -- the partition is the code.
 *
 * Every claim below was predicted in PREDICTIONS.md before this file was first
 * run, with the numbers filed. Where a prediction MISSED, the test measures
 * and records rather than asserting the wish -- the series keeps its misses.
 * Asserts are kept for what would be a bug (the identity, round-trip, singles)
 * and for the bars themselves (B1, B2, B3, B4).
 *
 * The one that carries the round is #5: two errors in different regions are
 * two single errors, each named by its own residue, no search -- a capability
 * codegg-v1's whole-square residue does not have. #7 is the predicted loss
 * (push), #9 is the honest floor (aliasing), and #6 holds the burst that
 * exposed the one-prime cap. All of it is measured with confirm both off and
 * on, because the off numbers are what forced confirm on by default. */
const E = require(__dirname + "/../eggso.js");
const G = require(__dirname + "/../../codegg-v1/codegg.js");
const fs = require("fs"), vm = require("vm"), path = require("path");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const record = (name, obj) => fs.writeFileSync(path.join(__dirname, "..", `measured-${name}.json`), JSON.stringify(obj, null, 1));

/* the precision-safe PRNG the series settled on after the LCG incident */
function mul32(a){
  return function(){
    a |= 0; a = a + 0x6D2B79F5 | 0;
    let t = Math.imul(a ^ a >>> 15, 1 | a);
    t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
    return (t ^ t >>> 14) >>> 0;
  };
}
const g = mul32(20260902);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const randCells = L => Int8Array.from({length: L}, () => g() & 1);
const pick = n => g() % n;
const same = (a, b) => a.length === b.length && Buffer.from(a).equals(Buffer.from(b));

/* stalk.js is a browser script with no exports; run it in a box to reach the
   real regions() -- the source of truth regionOf() must match */
const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname, "..", "..", "stalk.js"), "utf8"), site);
const siteRegions = (cells, n) => vm.runInContext(`regions(${JSON.stringify(Array.from(cells))}, ${n})`, site);

const N = 32, L = N * N;
const code  = E.makeCode(N);                       // confirm on: the default
const bare  = E.makeCode(N, {confirm: false});     // the filed construction
const CFGS = [["one prime", bare], ["+ confirm", code]];

/* 1. regionOf is stalk.js's own rule, cell for cell, at every width the site
      draws -- and the three regions at N=32 are the sizes PREDICTIONS filed */
{
  let cells = 0;
  for(let n = 2; n <= 40; n++){
    const reg = siteRegions(new Array(n * n).fill(1), n);
    const want = new Map();
    for(const k of ["inner", "fold", "outer"]) for(const s of reg[k]) want.set(s.r * n + s.c, k);
    for(let r = 0; r < n; r++) for(let c = 0; c < n; c++){
      ok(E.NAMES[E.regionOf(r, c, n)] === want.get(r * n + c), `regionOf(${r},${c},${n}) disagrees with stalk.js regions()`);
      cells++;
    }
  }
  const sz = code.members.map(m => m.length);
  ok(sz[0] === 496 && sz[1] === 32 && sz[2] === 496, `regions at N=32 are ${sz}, predicted 496/32/496`);
  console.log(`  regionOf matches stalk.js regions() over ${cells} cells, n = 2..40; N=32 splits 496/32/496`);
}

/* 2. the identity: I + F + O = V, mod p. By construction -- a miss is a bug. */
{
  for(let t = 0; t < 500; t++){
    const cells = randCells(L);
    const [I, F, O] = E.regionResidues(cells, code);
    ok((I + F + O) % code.p === G.residue(cells, code.p), `I+F+O != V mod p on trial ${t}`);
  }
  console.log(`  I + F + O = V (mod ${code.p}) on 500 random squares`);
}

/* 3. round-trip, exact, the same seven shapes v1 used, both configs */
{
  for(const [, cd] of CFGS) for(const n of [0, 1, 7, 128, 129, 1000, 4097]){
    const src = randBytes(n);
    const out = E.decode(E.encode(src, {N, confirm: cd.confirm}));
    ok(same(out.bytes, src), `round-trip broke at ${n} bytes`);
    ok(out.corrected === 0 && out.detected === 0, `clean data reported damage at ${n} bytes`);
  }
  console.log(`  round-trip exact, 7 shapes, both configs, all clean`);
}

/* 4. B1. single-cell errors: located, signed, repaired by the region's own
      residue, no search. 3000 anywhere, 3000 aimed at the 3% Fold. */
{
  const run = (cd, trials, where) => {
    let corrected = 0, direct = 0, wrong = 0;
    for(let t = 0; t < trials; t++){
      const cells = randCells(L), chk = E.checksFor(cells, cd), hurt = cells.slice();
      hurt[where ? where[pick(where.length)] : pick(L)] ^= 1;
      const r = E.repairSquare(hurt, chk, cd);
      if(r.status === "corrected"){ corrected++; direct += r.direct || 0; if(!same(hurt, cells)) wrong++; }
    }
    return {corrected, direct, wrong};
  };
  for(const [name, cd] of CFGS){
    const any = run(cd, 3000), fold = run(cd, 3000, cd.members[E.FOLD]);
    ok(any.corrected === 3000 && any.wrong === 0 && any.direct === 3000, `${name}: single anywhere ${any.corrected}/3000, ${any.wrong} wrong, ${any.direct} direct`);
    ok(fold.corrected === 3000 && fold.wrong === 0, `${name}: single on the Fold ${fold.corrected}/3000, ${fold.wrong} wrong`);
  }
  console.log(`  B1 MET: single error 3000/3000 anywhere, 3000/3000 on the Fold, all direct, 0 miscorrected, both configs`);
}

/* 5. B2, THE CLAIM. Two errors in different regions: two single errors, each
      named by its own residue, no search. Predicted 53% of random pairs; the
      direct share of all pairs must be >= 0.50. Same-region pairs fall to the
      in-region search, and with one prime that search aliases -- measured. */
{
  const out = {};
  for(const [name, cd] of CFGS){
    let cross = 0, sameR = 0, crossFixed = 0, crossDirect = 0, sameFixed = 0, wrong = 0, detected = 0;
    const T = 2000;
    for(let t = 0; t < T; t++){
      const cells = randCells(L), chk = E.checksFor(cells, cd), hurt = cells.slice();
      let a = pick(L), b = pick(L); while(b === a) b = pick(L);
      hurt[a] ^= 1; hurt[b] ^= 1;
      const isCross = cd.region[a] !== cd.region[b];
      if(isCross) cross++; else sameR++;
      const r = E.repairSquare(hurt, chk, cd);
      if(r.status === "corrected"){
        if(!same(hurt, cells)) wrong++;
        else if(isCross){ crossFixed++; if(r.searched === 0) crossDirect++; }
        else sameFixed++;
      } else detected++;
    }
    out[name] = {T, cross, crossShare: cross / T, crossFixed, crossDirect, sameR, sameFixed, detected, wrong, directShare: crossDirect / T};
    console.log(`  [${name}] two errors, ${T}: ${cross} cross-region (${(100*cross/T).toFixed(1)}%), ${crossFixed} corrected, ${crossDirect} without search`);
    console.log(`     same-region ${sameR}: ${sameFixed} corrected by search, ${detected} detected, ${wrong} MISCORRECTED`);
  }
  const c = out["+ confirm"], b = out["one prime"];
  ok(c.crossFixed === c.cross && c.crossDirect === c.cross, `cross-region pairs: ${c.crossFixed}/${c.cross} corrected, ${c.crossDirect} direct -- all should be`);
  ok(c.directShare >= 0.50, `B2 MISSED: direct share ${c.directShare.toFixed(3)} < 0.50`);
  console.log(`  B2 MET: direct-syndrome share of all random pairs = ${c.directShare.toFixed(3)} (predicted 0.53, bar 0.50)`);
  console.log(`  B4 (doubles): one prime miscorrected ${b.wrong}; with confirm ${c.wrong}. v1 on this channel: 0.`);
  record("doubles", out);
}

/* 6. bursts. The flagged 12-cell row burst is where the one-prime cap showed:
      predicted 800/800 as v1, measured 315/800 before confirm reached the
      erasure decoder. The Fold filled with 32 flips was predicted "detected
      only" and miscorrects instead -- for v1 too. Both measured, both kept. */
{
  const out = {};
  for(const [name, cd] of CFGS){
    let flagged = 0, flaggedAmb = 0, straddle = 0, straddleTried = 0;
    const fb = {corrected: 0, detected: 0, wrong: 0};
    for(let t = 0; t < 800; t++){
      const cells = randCells(L), chk = E.checksFor(cells, cd), hurt = cells.slice();
      const row = pick(N), c0 = pick(N - 12), F = [];
      for(let j = 0; j < 12; j++){ const i = row * N + c0 + j; hurt[i] = -1; F.push(i); }
      const r = E.repairSquare(hurt, chk, cd, {erased: F});
      if(r.status === "corrected" && same(hurt, cells)) flagged++; else if(r.status === "ambiguous") flaggedAmb++;
    }
    for(let t = 0; t < 400; t++){
      const cells = randCells(L), chk = E.checksFor(cells, cd), hurt = cells.slice();
      const row = 5 + pick(N - 10), cFold = N - 1 - row, c0 = Math.max(0, Math.min(N - 12, cFold - 6)), F = [];
      for(let j = 0; j < 12; j++){ const i = row * N + c0 + j; hurt[i] = -1; F.push(i); }
      if(new Set(F.map(i => cd.region[i])).size < 2) continue;
      straddleTried++;
      const r = E.repairSquare(hurt, chk, cd, {erased: F});
      if(r.status === "corrected" && same(hurt, cells)) straddle++;
    }
    for(let t = 0; t < 300; t++){
      const cells = randCells(L), chk = E.checksFor(cells, cd), hurt = cells.slice();
      for(const i of cd.members[E.FOLD]) hurt[i] ^= 1;
      const r = E.repairSquare(hurt, chk, cd);
      if(r.status === "corrected"){ if(same(hurt, cells)) fb.corrected++; else fb.wrong++; } else fb.detected++;
    }
    out[name] = {flaggedRow: {corrected: flagged, ambiguous: flaggedAmb, of: 800}, straddle: {corrected: straddle, of: straddleTried}, foldFilled: fb};
    console.log(`  [${name}] flagged 12-cell row burst ${flagged}/800 (${flaggedAmb} ambiguous); straddling the fold line ${straddle}/${straddleTried}`);
    console.log(`     the Fold filled, 32 unflagged: ${fb.corrected} ok, ${fb.detected} detected, ${fb.wrong} MISCORRECTED  (predicted: detected only -- MISSED)`);
  }
  ok(out["+ confirm"].flaggedRow.corrected === 800, `with confirm the flagged burst should be v1's 800/800, got ${out["+ confirm"].flaggedRow.corrected}`);
  record("bursts", out);
}

/* 7. push, the predicted loss: V is conserved, its three parts are not */
{
  let vHolds = 0, partsHold = 0, T = 200;
  for(let t = 0; t < T; t++){
    const cells = randCells(L);
    const pushed = Int8Array.from(vm.runInContext(`pushLeft(${JSON.stringify(Array.from(cells))})`, site));
    if(G.residue(cells, code.p) === G.residue(pushed, code.p)) vHolds++;
    const a = E.regionResidues(cells, code), b = E.regionResidues(pushed, code);
    if(a[0] === b[0] && a[1] === b[1] && a[2] === b[2]) partsHold++;
  }
  ok(vHolds === T, `v1's whole residue should survive push, held ${vHolds}/${T}`);
  console.log(`  push: v1's V residue holds ${vHolds}/${T}; eggSo's three parts hold ${partsHold}/${T}  (predicted a loss; it is)`);
  record("push", {T, vHolds, partsHold});
}

/* 8. B3, cost, from the format */
{
  const s  = E.sizes({N, L, p: bare.p, q: 0, confirm: false, bytes: 128});
  const sc = E.sizes({N, L, p: code.p, q: code.q, confirm: true, bytes: 128});
  const v1 = G.sizes({N, L, p: code.p, q: G.pickModulus(L, [code.p]), bytes: 128});
  console.log(`  B3: overhead per square -- one prime ${(s.overhead*100).toFixed(2)}%, + confirm ${(sc.overhead*100).toFixed(2)}%, codegg-v1 ${(v1.overhead*100).toFixed(2)}% (bar 4.70%)`);
  ok(sc.overhead <= 0.047, `B3 MISSED with confirm: ${(sc.overhead*100).toFixed(2)}%`);
  record("sizes", {onePrime: s, confirm: sc, v1});
}

/* 9. B4, the honest floor: same-region doubles, one prime vs confirm */
{
  const trial = (cd, T) => {
    let wrong = 0, corrected = 0, detected = 0;
    for(let t = 0; t < T; t++){
      const cells = randCells(L), chk = E.checksFor(cells, cd), hurt = cells.slice();
      const k = pick(3), m = cd.members[k];
      let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)];
      hurt[a] ^= 1; hurt[b] ^= 1;
      const r = E.repairSquare(hurt, chk, cd);
      if(r.status === "corrected"){ if(same(hurt, cells)) corrected++; else wrong++; } else detected++;
    }
    return {corrected, detected, wrong};
  };
  const one = trial(bare, 2000), two = trial(code, 2000);
  console.log(`  same-region doubles, 2000 each: one prime -> ${one.corrected} ok, ${one.detected} detected, ${one.wrong} MISCORRECTED`);
  console.log(`                                  + confirm  -> ${two.corrected} ok, ${two.detected} detected, ${two.wrong} MISCORRECTED`);
  ok(two.wrong <= 2, `B4 MISSED even with confirm: ${two.wrong} miscorrected`);
  console.log(`  B4 ${two.wrong === 0 ? "MET" : "met within alias floor"} with confirm; MISSED without (${one.wrong})`);
  record("floor", {onePrime: one, confirm: two});
}

console.log("eggso ok");
