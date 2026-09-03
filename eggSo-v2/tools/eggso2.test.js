/* node eggSo-v2/tools/eggso2.test.js -- the green is the code.
 *
 * Every claim below was predicted in PREDICTIONS.md before this file was first
 * run, with the numbers filed. Where a prediction MISSED, the test measures
 * and records rather than asserting the wish -- the series keeps its misses.
 * Asserts are kept for what would be a bug (pushLeft vs the site, the closed
 * form, round-trips, singles with the filter) and for the bars themselves.
 *
 * #1 is the site's own rule restated and pinned. #3 is S1, the histogram that
 * files v2(a)'s verdict. #4 is the trit alphabet's alias and what settles it.
 * #6 is v2(a) end to end on the squares that qualify. #7 is v2(b), where the
 * bits-as-trits burst MISSED at 0% for a reason worth the round. */
const W = require(__dirname + "/../eggso2.js");
const E = require(__dirname + "/../../eggSo-v0/eggso.js");
const G = require(__dirname + "/../../codegg-v1/codegg.js");
const fs = require("fs"), vm = require("vm"), path = require("path");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const record = (name, obj) => fs.writeFileSync(path.join(__dirname, "..", `measured-${name}.json`), JSON.stringify(obj, null, 1));
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
let g = mul32(20260902);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const randCells = L => Int8Array.from({length: L}, () => g() & 1);
const pick = n => g() % n;
const same = (a, b) => a.length === b.length && Buffer.from(a).equals(Buffer.from(b));
const pct = (x, n) => (100 * x / n).toFixed(2) + "%";
const root = path.join(__dirname, "..", "..");

const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(root, "stalk.js"), "utf8"), site);
const sitePush = vm.runInContext("pushLeft", site);

const N = 32, L = N * N;
const code = W.makeCode(N);
const canonSq = () => W.pushLeft(G.toCells(randBytes(L / 8), L)[0]);
const litCells = c => { const o = []; for(let i = 0; i < L; i++) if(c[i]) o.push(i); return o; };
const hit1 = (h, i) => { h[i] = h[i] === 0 ? (pick(2) ? 1 : -1) : 0; };

/* 1. the site's rules, restated and pinned: pushLeft equals stalk.js's own
      on 500 squares; the fixpoint is a +-1 prefix and a green tail; the tail
      is the 2-adic valuation of V; the trit alias count is the planning one */
{
  let agree = 0, canonical = 0, closed = 0, T = 500;
  for(let t = 0; t < T; t++){
    const c = randCells(L), mine = W.pushLeft(c), theirs = Int8Array.from(sitePush(Array.from(c)));
    if(same(mine, theirs)) agree++;
    if(W.isCanonical(mine)) canonical++;
    let v2 = 0; while(v2 < L && c[L - 1 - v2] === 0) v2++;
    if(W.tailOf(mine) === v2) closed++;
    ok(W.valueOf(mine) === W.valueOf(c), `push moved the value on trial ${t}`);
  }
  ok(agree === T, `restated pushLeft disagrees with stalk.js on ${T - agree}/${T}`);
  ok(canonical === T && closed === T, `canonical ${canonical}/${T}, closed form ${closed}/${T}`);
  ok(code.tritDistinct === 2050, `trit syndromes: ${code.tritDistinct} distinct of ${code.tritEntries}, planning said 2050 (2046 collide)`);
  console.log(`  pushLeft = stalk.js pushLeft on ${T}/${T} squares; V conserved; fixpoint is a +-1 prefix + green tail ${canonical}/${T}; tail = v2(V) ${closed}/${T}`);
  console.log(`  trit alphabet, one prime: ${code.tritDistinct} distinct syndromes of ${code.tritEntries} -> ${code.tritEntries - code.tritDistinct} collide (planning: 2,046), all of them 2w[j] = w[j-1]`);
}

/* 2. round-trip by BigInt, seven shapes, both arms -- and G.toBytes wrong on
      any pushed square with a -1, which is why toBytesV exists */
{
  for(const n of [0, 1, 7, 128, 129, 1000, 4097]){
    const src = randBytes(n);
    const a = W.decodeA(W.encodeA(src, {N})), b = W.decodeB(W.encodeB(src, {N}));
    ok(same(a.bytes, src) && !a.corrected && !a.detected, `arm a round-trip broke at ${n} bytes`);
    ok(same(b.bytes, src) && !b.corrected && !b.detected, `arm b round-trip broke at ${n} bytes`);
  }
  let wrongSquares = 0;
  for(let t = 0; t < 64; t++){ const src = randBytes(128), sq = G.toCells(src, L).map(W.pushLeft); if(!same(G.toBytes(sq, L, 128), src)) wrongSquares++; }
  console.log(`  round-trip exact via BigInt, 7 shapes, both arms; G.toBytes on pushed squares wrong ${wrongSquares}/64 (it reads cells[j] === 1)`);
}

/* 3. S1 -- the histogram, the geometric law, the fallback rate. The random
      run is repeated here and asserted; the corpora are read from what
      tools/greens.js wrote so the two files cannot disagree. */
{
  g = mul32(20260902);
  const hist = new Map(); let sum = 0, inBand = 0, T = 10000;
  for(let t = 0; t < T; t++){ const k = W.tailOf(W.pushLeft(randCells(L))); sum += k; if(k >= 28) inBand++; hist.set(k, (hist.get(k) || 0) + 1); }
  const mean = sum / T;
  let worstZ = 0;
  for(let k = 0; k <= 6; k++){ const p = Math.pow(2, -(k + 1)), exp = T * p, z = ((hist.get(k) || 0) - exp) / Math.sqrt(exp * (1 - p)); worstZ = Math.max(worstZ, Math.abs(z)); }
  ok(Math.abs(mean - 1) <= 0.02, `S1 MISSED: random mean ${mean.toFixed(3)}`);
  ok(worstZ <= 3.5, `S1: geometric law off, worst |z| ${worstZ.toFixed(2)}`);
  ok(inBand === 0, `S1 MISSED: ${inBand} random squares in-band`);
  const greensFile = path.join(__dirname, "..", "measured-greens.json");
  const greens = fs.existsSync(greensFile) ? JSON.parse(fs.readFileSync(greensFile, "utf8")) : null;
  console.log(`  S1 MET: random mean ${mean.toFixed(3)} (bar 1.00 +- 0.02), geometric law worst |z| ${worstZ.toFixed(2)} over k = 0..6, ${inBand}/${T} in-band`);
  if(greens) for(const [name, r] of Object.entries(greens)) if(name !== "random") console.log(`     ${name.padEnd(14)} ${String(r.squares).padStart(5)} squares  mean ${r.mean.toFixed(2)}  in-band ${r.inBand} (${pct(r.inBand, r.squares)})  fall back ${pct(r.squares - r.inBand, r.squares)}`);
  record("s1", {random: {T, mean, worstZ, inBand, hist: Object.fromEntries([...hist.entries()].sort((a, b) => a[0] - b[0]))}, corpora: greens});
}

/* 4. T -- trit singles on canonical squares. d = +-1 (a lit cell greened or a
      green lit) and d = +-2 (a sign flip). Each has the 2w[j] = w[j-1] alias;
      the canonicity filter and q settle it. The bare rate is measured. */
{
  const run = (T, dmg, opts) => {
    const out = {T, corrected: 0, ambiguous: 0, detected: 0, wrong: 0};
    for(let t = 0; t < T; t++){
      const c = canonSq(), chk = E.checksFor(c, code), h = c.slice();
      dmg(h, c);
      const r = W.repairSquare(h, chk, code, opts);
      if(r.status === "corrected"){ if(same(h, c)) out.corrected++; else out.wrong++; } else out[r.status]++;
    }
    return out;
  };
  const one = (h) => hit1(h, pick(L));
  const sign = (h, c) => { const l = litCells(c); const i = l[pick(l.length)]; h[i] = -h[i]; };
  g = mul32(101);
  const d1 = run(3000, one), d2 = run(3000, sign);
  const d1bare = run(3000, one, {canonical: false, confirm: false}), d2bare = run(3000, sign, {canonical: false, confirm: false});
  const d1filter = run(3000, one, {confirm: false}), d2q = run(3000, sign, {canonical: false});
  /* codegg-v1 in trit mode on the same kind of squares */
  const v1 = G.makeCode(N); let v1ok = 0, v1wrong = 0;
  for(let t = 0; t < 3000; t++){ const c = canonSq(), chk = [G.residue(c, v1.p), G.residue(c, v1.q)], h = c.slice(); if(t & 1) sign(h, c); else one(h); const r = G.repairSquare(h, chk, v1, {alphabet: "trit"}); if(r.status === "corrected"){ if(same(h, c)) v1ok++; else v1wrong++; } }
  ok(d1.corrected === 3000 && d1.wrong === 0 && d2.corrected === 3000 && d2.wrong === 0, `T MISSED: d=+-1 ${d1.corrected}/3000 (${d1.wrong} wrong), d=+-2 ${d2.corrected}/3000 (${d2.wrong} wrong)`);
  console.log(`  T MET: trit singles on canonical squares, filter + q: d=+-1 ${d1.corrected}/3000, d=+-2 (sign flip) ${d2.corrected}/3000, 0 wrong`);
  console.log(`     bare (no filter, no q): d=+-1 ${d1bare.corrected} ok ${d1bare.ambiguous} ambiguous; d=+-2 ${d2bare.corrected} ok ${d2bare.ambiguous} ambiguous; 0 wrong  (called ~25% ambiguous)`);
  console.log(`     filter alone: d=+-1 ${d1filter.corrected}/3000 · q alone: d=+-2 ${d2q.corrected}/3000 · codegg-v1 trit mode ${v1ok}/3000, ${v1wrong} wrong`);
  record("singles", {filterAndQ: {d1, d2}, bare: {d1: d1bare, d2: d2bare}, filterOnly: d1filter, qOnly: d2q, codeggV1Trit: {corrected: v1ok, wrong: v1wrong, of: 3000}});
}

/* 5. pairs on canonical squares, filter + q per candidate. PREDICTIONS filed
      the standings at v0's ~130/400 same-region; the per-candidate confirm
      the plan asked for changes that, and this is where it is measured. */
{
  g = mul32(111);
  const run = (T, dmg) => { const out = {T, corrected: 0, ambiguous: 0, detected: 0, wrong: 0, direct: 0, searched: 0}; for(let t = 0; t < T; t++){ const c = canonSq(), chk = E.checksFor(c, code), h = c.slice(); dmg(h); const r = W.repairSquare(h, chk, code); if(r.status === "corrected"){ if(same(h, c)){ out.corrected++; out.direct += r.direct || 0; out.searched += r.searched || 0; } else out.wrong++; } else out[r.status]++; } return out; };
  const any = run(1000, h => { let a = pick(L), b = pick(L); while(b === a) b = pick(L); hit1(h, a); hit1(h, b); });
  const sameR = run(1000, h => { const k = pick(3), m = code.members[k]; let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)]; hit1(h, a); hit1(h, b); });
  const cross = run(1000, h => { let a = pick(L), b = pick(L); while(code.region[b] === code.region[a]) b = pick(L); hit1(h, a); hit1(h, b); });
  const fold = run(300, h => { for(const i of code.members[E.FOLD]) hit1(h, i); });
  ok(any.wrong === 0 && sameR.wrong === 0 && cross.wrong === 0 && fold.wrong === 0, `pairs miscorrected: ${any.wrong} ${sameR.wrong} ${cross.wrong} ${fold.wrong}`);
  console.log(`  pairs on canonical squares: 2 anywhere ${any.corrected}/1000 (${any.searched / 2} by search) · same-region ${sameR.corrected}/1000 (${sameR.ambiguous} ambiguous) · cross-region ${cross.corrected}/1000 all direct · Fold filled ${fold.detected + fold.ambiguous}/300 detected, ${fold.wrong} wrong`);
  /* the control that was not filed and should have been: v0's own search on
     plain BIT squares with q applied per candidate (codegg.js:204-206) instead
     of after the whole plan (eggso.js:237-241). Filed before running, from the
     arithmetic: ~60 alphabet-valid pairs satisfy the region residue, 1/q of
     them pass q -> ~97% unique, ~3% ambiguous, 0 wrong. */
  const bitSq = () => G.toCells(randBytes(128), L)[0];
  const v0q = {T: 1000, corrected: 0, ambiguous: 0, detected: 0, wrong: 0};
  for(let t = 0; t < 1000; t++){
    const c = bitSq(), chk = E.checksFor(c, code), h = c.slice();
    const k = pick(3), m = code.members[k]; let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)]; h[a] ^= 1; h[b] ^= 1;
    const r = W.repairSquare(h, chk, code, {alphabet: "bit", canonical: false});
    if(r.status === "corrected"){ if(same(h, c)) v0q.corrected++; else v0q.wrong++; } else v0q[r.status]++;
  }
  ok(v0q.wrong === 0, `v0's search with per-candidate q miscorrected ${v0q.wrong}`);
  console.log(`  v0's search on BIT squares with q per candidate, same-region pairs: ${v0q.corrected}/1000 corrected, ${v0q.ambiguous} ambiguous, ${v0q.wrong} wrong (v0 as shipped: 2/921; filed here at ~97% before running)`);
  record("pairs", {anywhere: any, sameRegion: sameR, crossRegion: cross, foldFilled: fold, v0SearchPerCandidateQ: v0q});
}

/* 6. v2(a) -- in-band squares end to end. Crafted blocks ending in >= 4 zero
      bytes qualify; the check goes into the last 28 cells and comes back out;
      damage to data is corrected, damage to the slots and to the flag is
      detected, never silently wrong. Then the fallback rate per corpus. */
{
  g = mul32(121);
  const craft = () => { const b = randBytes(128); for(let i = 124; i < 128; i++) b[i] = 0; return b; };
  let rt = 0, dataFix = 0, signFix = 0, slotDet = 0, slotWrong = 0, flag10det = 0, flag10wrong = 0, flag01det = 0, flag01wrong = 0, T = 200;
  for(let t = 0; t < T; t++){
    const src = craft();
    let e = W.encodeA(src, {N}); ok(e.flags[0] === 1, "a block ending in 4 zero bytes should be in-band");
    if(same(W.decodeA(e).bytes, src)) rt++;
    e = W.encodeA(src, {N}); hit1(e.squares[0], pick(L - 28)); { const o = W.decodeA(e); if(o.corrected === 1 && same(o.bytes, src)) dataFix++; }
    e = W.encodeA(src, {N}); { const l = litCells(e.squares[0]).filter(i => i < L - 28); const i = l[pick(l.length)]; e.squares[0][i] = -e.squares[0][i]; const o = W.decodeA(e); if(o.corrected === 1 && same(o.bytes, src)) signFix++; }
    e = W.encodeA(src, {N}); { const j = L - 28 + pick(28); e.squares[0][j] = e.squares[0][j] === 0 ? (pick(2) ? 1 : -1) : 0; const o = W.decodeA(e); if(o.detected) slotDet++; else if(!same(o.bytes, src)) slotWrong++; }
    e = W.encodeA(src, {N}); e.flags[0] = 0; { const o = W.decodeA(e); if(o.detected) flag10det++; else if(!same(o.bytes, src)) flag10wrong++; }
    const src2 = randBytes(128); src2[127] |= 1;                       // odd: never in-band
    e = W.encodeA(src2, {N}); ok(e.flags[0] === 0, "an odd block cannot be in-band"); e.flags[0] = 1; { const o = W.decodeA(e); if(o.detected) flag01det++; else if(!same(o.bytes, src2)) flag01wrong++; }
  }
  ok(rt === T && dataFix === T && signFix === T, `A1: in-band round-trip ${rt}/${T}, data ${dataFix}/${T}, sign ${signFix}/${T}`);
  ok(slotWrong === 0 && flag10wrong === 0 && flag01wrong === 0, `A3 MISSED: silently wrong -- slots ${slotWrong}, flag 1->0 ${flag10wrong}, flag 0->1 ${flag01wrong}`);
  console.log(`  A1 MET: in-band squares round-trip ${rt}/${T} with the checks in the tail; a d=+-1 error ${dataFix}/${T} and a sign flip ${signFix}/${T} corrected`);
  console.log(`  A3 MET: check-slot damage detected ${slotDet}/${T} (${slotWrong} silently wrong); flag 1->0 detected ${flag10det}/${T}; flag 0->1 detected ${flag01det}/${T}`);
  /* the fallback rate, per corpus, from the encoder itself */
  const corp = {};
  for(const [name, rel] of [["spec.md", "spec.md"], ["stalk.js", "stalk.js"], ["og.png", "og.png"], ["program.exe", "codegg-v10/corpus/program.exe"], ["notepad.exe", "codegg-v10/corpus-real/notepad.exe"], ["archive.zst", "codegg-v10/corpus/archive.zst"]]){
    const f = path.join(root, rel); if(!fs.existsSync(f)) continue;
    const src = fs.readFileSync(f), e = W.encodeA(src, {N}), s = W.sizesA(e.meta);
    const out = W.decodeA(e);
    ok(same(out.bytes, src) && out.corrected === 0 && out.detected === 0, `${name}: clean in-band file did not round-trip`);
    corp[name] = {squares: s.squares, inBand: s.inBand, fallbackRate: s.fallbackRate, overhead: s.overhead, v0Overhead: E.sizes({N, L, p: code.p, q: code.q, confirm: true, bytes: src.length}).overhead};
    console.log(`     ${name.padEnd(12)} ${String(s.squares).padStart(5)} squares, ${String(s.inBand).padStart(4)} in-band -> fall back ${pct(s.squares - s.inBand, s.squares)}; overhead ${pct(s.overhead, 1)} vs v0 ${pct(corp[name].v0Overhead, 1)}`);
  }
  const randomOverhead = W.sizesA({N, L, p: code.p, q: code.q, bytes: 12800, squares: 100, inBand: 0}).overhead;
  console.log(`  A2 ${Object.values(corp).filter(c => c.fallbackRate >= 0.99).length}/${Object.keys(corp).length} corpora fall back >= 99% (the PEs do not) · A4: overhead random ${pct(randomOverhead, 1)}, spec.md ${pct(corp["spec.md"].overhead, 1)} (v0 4.69%)`);
  record("a", {inBand: {T, roundTrip: rt, dataFix, signFix, slotDetected: slotDet, slotWrong, flag10detected: flag10det, flag10wrong, flag01detected: flag01det, flag01wrong}, corpora: corp, randomOverhead});
}

/* 7. v2(b) -- greens as erasures. The cap; the canonical two-valued burst; the
      bits-as-trits burst at cap 12 (called 92%) and at the default cap 10;
      and -- added after the first run showed 0% -- scattered flagged cells,
      where adjacent-cell aliases cannot occur (filed at ~88% before running). */
{
  g = mul32(131);
  const run = (T, mk, dmg, opts) => {
    const out = {T, corrected: 0, ambiguous: 0, detected: 0, wrong: 0, notes: {}};
    for(let t = 0; t < T; t++){
      const c = mk(), chk = E.checksFor(c, code), h = c.slice();
      const F = dmg(h);
      const r = W.repairSquare(h, chk, code, {...opts, erased: F});
      if(r.status === "corrected"){ if(same(h, c)) out.corrected++; else out.wrong++; } else { out[r.status]++; out.notes[r.note] = (out.notes[r.note] || 0) + 1; }
    }
    return out;
  };
  const bitSq = () => G.toCells(randBytes(128), L)[0];
  const rowBurst = h => { const r = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ h[r * N + c0 + j] = 0; F.push(r * N + c0 + j); } return F; };
  const scattered = h => { const k = pick(2) ? 0 : 2, m = code.members[k], S = new Set(); while(S.size < 12){ const i = m[pick(m.length)]; if(![...S].some(j => Math.abs(j - i) === 1)) S.add(i); } const F = [...S]; for(const i of F) h[i] = 0; return F; };
  /* cap enforcement */
  { const c = bitSq(), chk = E.checksFor(c, code), h = c.slice(); const F = []; for(let j = 0; j < 11; j++){ h[j] = 0; F.push(j); }
    const r = W.repairSquare(h, chk, code, {canonical: false, erased: F}); ok(r.note === "too many erasures", `B1: 11 flagged in one region at cap 10 gave ${r.status} ${r.note}`); }
  const canon = run(400, canonSq, rowBurst, {});
  const bitsCtl = run(400, bitSq, rowBurst, {alphabet: "bit", canonical: false});
  const t0 = Date.now();
  const bat12 = run(400, bitSq, rowBurst, {canonical: false, cap: 12});
  const ms = Date.now() - t0;
  const bat10 = run(400, bitSq, rowBurst, {canonical: false});
  const scat12 = run(400, bitSq, scattered, {canonical: false, cap: 12});
  const scatCanon = run(400, canonSq, scattered, {});
  ok(canon.corrected === 400 && canon.wrong === 0, `B3 MISSED: canonical burst ${canon.corrected}/400, ${canon.wrong} wrong`);
  ok(bitsCtl.corrected === 400, `bit-alphabet control ${bitsCtl.corrected}/400`);
  ok(bat12.wrong === 0 && bat10.wrong === 0 && scat12.wrong === 0, `erasures miscorrected: ${bat12.wrong} ${bat10.wrong} ${scat12.wrong}`);
  console.log(`  B1 MET: cap 10 per region enforced. B3 MET: canonical 12-cell burst, two-valued model, ${canon.corrected}/400. Bit-alphabet control ${bitsCtl.corrected}/400.`);
  console.log(`  B2 ${bat12.corrected / 400 >= 0.85 && bat12.corrected / 400 <= 0.96 ? "MET" : "MISSED"}: bits-as-trits 12-cell row burst, cap 12: ${bat12.corrected} ok, ${bat12.ambiguous} ambiguous, ${bat12.detected} detected, ${bat12.wrong} wrong (called 368/400; ${ms} ms)`);
  console.log(`     at the default cap 10: ${bat10.corrected} ok, ${bat10.ambiguous} ambiguous, ${bat10.detected} detected ${JSON.stringify(bat10.notes)}`);
  console.log(`     12 SCATTERED non-adjacent flagged cells in one hemisphere, bits-as-trits cap 12: ${scat12.corrected} ok, ${scat12.ambiguous} ambiguous, ${scat12.wrong} wrong (filed ~88% after the burst's miss, before running) · canonical: ${scatCanon.corrected}/400`);
  record("b", {canonicalBurst: canon, bitControl: bitsCtl, bitsAsTritsCap12: bat12, bitsAsTritsCap10: bat10, scatteredCap12: scat12, scatteredCanonical: scatCanon, cap12ms: ms});
}

/* 8. push is vacuous here: the stored square is push's fixpoint */
{
  let fixed = 0, T = 200; g = mul32(141);
  for(let t = 0; t < T; t++){ const c = canonSq(); if(same(W.pushLeft(c), c)) fixed++; }
  ok(fixed === T, `stored squares are not push fixpoints: ${fixed}/${T}`);
  console.log(`  push: the stored square is its own pushLeft ${fixed}/${T} -- invariance is vacuous, as filed`);
  record("push", {T, fixed});
}

/* 9. cost */
{
  const a = W.sizesA({N, L, p: code.p, q: code.q, bytes: 12800, squares: 100, inBand: 0});
  const b = W.sizesB({N, L, p: code.p, q: code.q, bytes: 128});
  console.log(`  cost per square -- v2(a) ${a.bitsPerSquare} bits when out of band = ${pct(a.overhead, 1)} on random data; v2(b) ${pct(b.overhead, 1)}; v0 4.69%; codegg-v1 2.34%`);
  record("sizes", {a, b});
}

console.log("eggso2 ok");
