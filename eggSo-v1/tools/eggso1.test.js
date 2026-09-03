/* node eggSo-v1/tools/eggso1.test.js -- the anti-transpose is the code.
 *
 * Every claim below was predicted in PREDICTIONS.md before this file was first
 * run, with the numbers filed. Where a prediction MISSED, the test measures
 * and records rather than asserting the wish -- the series keeps its misses.
 * Asserts are kept for what would be a bug (partnerOf vs the site, the
 * involution, round-trips, singles) and for the bars themselves.
 *
 * Three arms, each under every configuration it has (CFGS). #2 is the
 * round's measurement #1 -- the pair table's collision count. #5 carries
 * v1(a): same-region pairs corrected with no search. #6 is the kernel
 * control that must FAIL on purpose. #9 is v1(c) against v0 on the same
 * seeds, built to falsify. #10 is v1(b) and its one row nobody else has. */
const V = require(__dirname + "/../eggso1.js");
const E = require(__dirname + "/../../eggSo-v0/eggso.js");
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
const SEED = 20260902;
let g = mul32(SEED);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const randCells = L => Int8Array.from({length: L}, () => g() & 1);
const pick = n => g() % n;
const same = (a, b) => a.length === b.length && Buffer.from(a).equals(Buffer.from(b));
const pct = (x, n) => (100 * x / n).toFixed(2) + "%";

/* stalk.js is a browser script with no exports; run it in a box for pushLeft */
const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname, "..", "..", "stalk.js"), "utf8"), site);
const pushLeft = cells => Int8Array.from(vm.runInContext(`pushLeft(${JSON.stringify(Array.from(cells))})`, site));

const N = 32, L = N * N;
const A  = V.makeCodeA(N);                        // 5 residues: I F O sigma q
const A0 = V.makeCodeA(N, {confirm: false});      // sigma in place of q: the negative control
const CFGS = [["v1a", A], ["v1a-replaces-q", A0]];
const B  = V.makeCodeB(N);
const C  = V.makeCodeC(N);
const V0 = E.makeCode(N);

/* 1. partnerOf is index.html:388's inline arithmetic, cell for cell, at every
      width the site draws. The site has no function for the anti-transpose,
      so the assertion is against the line itself: it must still be there,
      and the function built from it must agree. */
{
  const html = fs.readFileSync(path.join(__dirname, "..", "..", "index.html"), "utf8").split("\n");
  const at = html.findIndex(l => /const pr = n - 1 - c, pc = n - 1 - r;/.test(l));
  ok(at >= 0, "index.html no longer contains `const pr = n - 1 - c, pc = n - 1 - r;`");
  const expr = html[at].match(/const (pr = .*?, pc = .*?);/)[1];
  const sitePartner = new Function("r", "c", "n", `const ${expr}; return [pr, pc];`);
  let cells = 0, fixedAll = 0;
  for(let n = 2; n <= 40; n++){
    const Ln = n * n;
    for(let j = 0; j < Ln; j++){
      const r = Math.floor(j / n), c = j % n;
      const [pr, pc] = sitePartner(r, c, n);
      const s = V.partnerOf(j, n);
      ok(s === pr * n + pc, `partnerOf(${j},${n}) = ${s}, index.html says (${pr},${pc})`);
      const [qr, qc] = V.partnerRC(r, c, n);
      ok(qr === pr && qc === pc, `partnerRC(${r},${c},${n}) disagrees with index.html`);
      ok(V.partnerOf(s, n) === j, `sigma not an involution at ${j}, n=${n}`);
      const fixed = s === j, fold = E.regionOf(r, c, n) === E.FOLD;
      ok(fixed === fold, `fixed set != Fold at ${j}, n=${n}`);
      ok(E.regionOf(Math.floor(s / n), s % n, n) === 2 - E.regionOf(r, c, n), `regionOf(sigma j) != 2 - regionOf(j) at ${j}, n=${n}`);
      cells++; if(fixed) fixedAll++;
    }
  }
  /* the planning ground at N = 32: partner weights never comparable, the
     Fold in the kernel of w - w o sigma */
  let lo = Infinity, hi = -Infinity, foldZero = 0;
  for(const j of A.members[E.INNER]){ const e = A.sig[j] - j; lo = Math.min(lo, e); hi = Math.max(hi, e); }
  for(const j of A.members[E.FOLD]) if(A.ws[j] === 0) foldZero++;
  ok(lo === 33 && hi === 1023, `w[j]/w[sigma j] exponent range ${lo}..${hi}, planning said 33..1023`);
  ok(foldZero === 32, `Fold cells with w - w o sigma = 0: ${foldZero}/32`);
  console.log(`  partnerOf matches index.html:${at + 1} (\`${expr}\`) over ${cells} cells, n = 2..40; involution; fixed set = Fold (${fixedAll} cells); regionOf(sigma j) = 2 - regionOf(j)`);
  console.log(`  N=32: partner weight ratio 2^33..2^1023 over Inner; w - w o sigma = 0 on all 32 Fold cells`);
  record("sigma", {cells, fixedAll, indexHtmlLine: at + 1, expr, exponentRange: [lo, hi], foldKernel: foldZero});
}

/* 2. A2, measurement #1: the pair table. Planning computed 455,428 distinct
      joint syndromes of 491,040 -- "7.25% collide". That figure is the
      EXCESS rate, 1 - distinct/total. The share of entries that have a twin
      is a different number and is recorded beside it. The peel must return
      the table's candidate set, and no pair may alias a single. */
{
  const t = A.pair;
  const excess = 1 - t.distinct / t.total;
  ok(t.total === 491040, `pair table has ${t.total} entries, planning said 491,040`);
  ok(t.distinct === 455428, `distinct joint syndromes ${t.distinct}, planning computed 455,428`);
  ok(t.singleAlias === 0, `${t.singleAlias} pair syndromes equal a single's`);
  ok(t.regionOnlyDistinct === 2052, `region residue alone: ${t.regionOnlyDistinct} distinct, planning said 2,052`);
  let checked = 0, withTwins = 0;
  for(let k = 0; k < 400; k++){
    const inner = A.members[E.INNER];
    let a = inner[pick(inner.length)], b = inner[pick(inner.length)]; while(b === a) b = inner[pick(inner.length)];
    const d1 = pick(2) ? 1 : -1, d2 = pick(2) ? 1 : -1;
    const X = (((d1 * A.w[a] + d2 * A.w[b]) % A.p) + A.p) % A.p;
    const Y = (((d1 * A.w[A.sig[a]] + d2 * A.w[A.sig[b]]) % A.p) + A.p) % A.p;
    const key = e => `${e.ia},${e.ib},${e.d1},${e.d2}`;
    const byTable = V.lookupPairs(t, X, Y).map(key).sort(), byPeel = V.peelPairs(A, X, Y).map(key).sort();
    ok(byTable.join("|") === byPeel.join("|"), `peel and table disagree on (${X},${Y}): ${byTable} vs ${byPeel}`);
    ok(byTable.includes(`${Math.min(a, b)},${Math.max(a, b)},${a < b ? d1 : d2},${a < b ? d2 : d1}`), "the true pair is missing from its own lookup");
    checked++; if(byTable.length > 1) withTwins++;
  }
  console.log(`  A2: pair table ${t.total} entries, ${t.distinct} distinct -> excess ${pct(t.total - t.distinct, t.total)} (planning: 7.25%); entries with a twin ${pct(t.colliding, t.total)}`);
  console.log(`      region residue alone ${t.regionOnlyDistinct} distinct (99.6% collide); pair syndromes equal to a single's: ${t.singleAlias}`);
  console.log(`      992-probe peel == table lookup on ${checked} random pair syndromes (${withTwins} of them had twins)`);
  record("table", {total: t.total, distinct: t.distinct, excessRate: excess, entriesWithTwin: t.colliding, twinShare: t.collisionRate,
                   regionOnlyDistinct: t.regionOnlyDistinct, singleAlias: t.singleAlias, peelChecked: checked, peelWithTwins: withTwins, p: A.p, q: A.q});
}

/* 3. round-trip, exact, seven shapes, every arm and configuration */
{
  for(const [, cd] of CFGS) for(const n of [0, 1, 7, 128, 129, 1000, 4097]){
    const src = randBytes(n), out = V.decodeA(V.encodeA(src, {N, confirm: cd.confirm}));
    ok(same(out.bytes, src) && !out.corrected && !out.detected, `arm a round-trip broke at ${n} bytes`);
  }
  for(const n of [0, 1, 7, 66, 67, 1000, 4097]){
    const src = randBytes(n), enc = V.encodeB(src, {N});
    ok(enc.squares.length === (Math.ceil(n * 8 / 528) || 1), `arm b squares at ${n} bytes`);
    const out = V.decodeB(enc);
    ok(same(out.bytes, src) && !out.corrected && !out.detected, `arm b round-trip broke at ${n} bytes`);
  }
  for(const n of [0, 1, 7, 128, 129, 1000, 4097]){
    const src = randBytes(n), out = V.decodeC(V.encodeC(src, {N}));
    ok(same(out.bytes, src) && !out.corrected && !out.detected, `arm c round-trip broke at ${n} bytes`);
  }
  console.log(`  round-trip exact: arm a (both cfgs), arm b (528 cells = 66 bytes per square), arm c; 7 shapes each`);
}

/* shared channel runner. `arm` = {code, checks(cells), repair(hurt, chk, erased), phys(cells)}
   where phys maps a logical square to what the arm stores (identity for
   all but arm c). Damage is applied to the stored square at the positions
   the channel names, so region membership means the same to every arm. */
const armA  = cd => ({code: cd, checks: c => V.checksForA(c, cd), repair: (h, k, er) => V.repairA(h, k, cd, er ? {erased: er} : undefined), phys: c => c});
const armB  = {code: B, checks: c => E.checksFor(c, B), repair: (h, k, er) => V.repairB(h, k, B, er ? {erased: er} : undefined), phys: c => c};
const armC  = {code: C, checks: c => E.checksFor(V.permuteC(c, C), C), repair: (h, k, er) => V.repairC(h, k, C, er ? {erased: er} : undefined), phys: c => V.permuteC(c, C)};
const armV0 = {code: V0, checks: c => E.checksFor(c, V0), repair: (h, k, er) => E.repairSquare(h, k, V0, er ? {erased: er} : undefined), phys: c => c};
function channel(arm, T, damage, seed, mk){
  g = mul32(seed);
  const out = {T, corrected: 0, detected: 0, wrong: 0, direct: 0, lookup: 0, searched: 0, fires: {inner: 0, fold: 0, outer: 0}};
  for(let t = 0; t < T; t++){
    const logical = mk ? mk() : randCells(L), stored = arm.phys(logical), chk = arm.checks(stored), hurt = stored.slice();
    const erased = damage(hurt, arm.code);
    const r = arm.repair(hurt, chk, erased);
    if(r.status === "corrected"){
      if(same(hurt, stored)){ out.corrected++; out.direct += r.direct || 0; out.lookup += r.lookup || 0; out.searched += r.searched || 0; }
      else out.wrong++;
    } else out.detected++;
    if(r.regions) for(const n of r.regions) out.fires[n]++;
  }
  return out;
}
const flip1 = (h) => { h[pick(L)] ^= 1; };
const flip2 = (h) => { let a = pick(L), b = pick(L); while(b === a) b = pick(L); h[a] ^= 1; h[b] ^= 1; };
const flip2same = (h, cd) => { const k = pick(3), m = cd.members[k]; let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)]; h[a] ^= 1; h[b] ^= 1; };
const flip2cross = (h, cd) => { let a = pick(L), b = pick(L); while(cd.region[b] === cd.region[a]) b = pick(L); h[a] ^= 1; h[b] ^= 1; };
const flip3 = (h, cd) => { for(let k = 0; k < 3; k++){ const m = cd.members[k]; h[m[pick(m.length)]] ^= 1; } };
const foldPair = (h, cd) => { const m = cd.members[E.FOLD]; let a = m[pick(32)], b = m[pick(32)]; while(b === a) b = m[pick(32)]; h[a] ^= 1; h[b] ^= 1; };
const foldFill = (h, cd) => { for(const i of cd.members[E.FOLD]) h[i] ^= 1; };
const burstFlagged = (h) => { const r = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ h[r * N + c0 + j] = -1; F.push(r * N + c0 + j); } return F; };
const burstInRegion = (h, cd) => {                 // 12 flips in one row, all in Inner or all in Outer
  for(;;){
    const r = pick(N), c0 = pick(N - 12), regs = new Set();
    for(let j = 0; j < 12; j++) regs.add(cd.region[r * N + c0 + j]);
    if(regs.size === 1 && !regs.has(E.FOLD)){ for(let j = 0; j < 12; j++) h[r * N + c0 + j] ^= 1; return; }
  }
};
const burstRow = (h) => { const r = pick(N), c0 = pick(N - 12); for(let j = 0; j < 12; j++) h[r * N + c0 + j] ^= 1; };

/* 4. A1. single-cell errors, both configurations: 3000 anywhere, 3000 on
      the Fold, all direct, none wrong. The Fold rides on F mod p alone --
      R_sigma is blind to it -- and must still land. */
{
  for(const [name, cd] of CFGS){
    const any = channel(armA(cd), 3000, flip1, 11);
    const fold = channel(armA(cd), 3000, (h) => { const m = cd.members[E.FOLD]; h[m[pick(32)]] ^= 1; }, 12);
    ok(any.corrected === 3000 && any.wrong === 0 && any.direct === 3000, `${name}: single anywhere ${any.corrected}/3000, ${any.wrong} wrong, ${any.direct} direct`);
    ok(fold.corrected === 3000 && fold.wrong === 0, `${name}: single on the Fold ${fold.corrected}/3000, ${fold.wrong} wrong`);
  }
  console.log(`  A1 MET: single error 3000/3000 anywhere, 3000/3000 on the Fold, all direct, 0 wrong, both configurations`);
}

/* 5. A3, THE CLAIM. Same-region pairs corrected with no search: the pair
      table names them. Predicted >= 95% direct, called 99%, 0 wrong. Also
      Fold-Fold aimed (search, credited to q), cross-region, 2 anywhere. */
{
  const out = {};
  for(const [name, cd] of CFGS){
    const arm = armA(cd);
    const sameR = channel(arm, 2000, flip2same, 21);
    const foldFold = channel(arm, 500, foldPair, 22);
    const cross = channel(arm, 1000, flip2cross, 23);
    const any = channel(arm, 2000, flip2, 24);
    const three = channel(arm, 1000, flip3, 25);
    out[name] = {sameRegion: sameR, foldFold, crossRegion: cross, anywhere: any, threeOnePerRegion: three};
    console.log(`  [${name}] same-region pairs 2000: ${sameR.corrected} corrected (${sameR.lookup / 2} by table, ${sameR.searched / 2} Fold pairs by search), ${sameR.detected} detected, ${sameR.wrong} MISCORRECTED`);
    console.log(`     Fold-Fold aimed 500: ${foldFold.corrected} ok, ${foldFold.detected} det, ${foldFold.wrong} wrong | cross-region 1000: ${cross.corrected} ok, ${cross.direct} direct | 2 anywhere 2000: ${any.corrected} ok, ${any.detected} det, ${any.wrong} wrong | 3 one-per-region 1000: ${three.corrected} ok, ${three.wrong} wrong`);
  }
  const live = out["v1a"];
  ok(live.sameRegion.wrong === 0, `A3 MISSED: ${live.sameRegion.wrong} same-region pairs miscorrected with confirm`);
  ok(live.sameRegion.corrected / 2000 >= 0.95, `A3 MISSED: same-region pairs ${live.sameRegion.corrected}/2000 < 95%`);
  ok(live.crossRegion.corrected === 1000 && live.crossRegion.direct === 2000, `cross-region pairs ${live.crossRegion.corrected}/1000, ${live.crossRegion.direct} direct cells`);
  ok(live.threeOnePerRegion.corrected === 1000 && live.threeOnePerRegion.wrong === 0, `3 one-per-region ${live.threeOnePerRegion.corrected}/1000`);
  console.log(`  A3 MET: same-region pairs ${pct(live.sameRegion.corrected, 2000)} corrected, 0 miscorrected (called 99%, bar 95%). v0 on this channel: 2/921.`);
  record("a-pairs", out);
}

/* 6. A4, the kernel, and the flagged burst. The Fold is in R_sigma's kernel:
      with sigma in place of q, the Fold filled must miscorrect at the bare
      rate again (predicted 15-25%). With q it must be refused every time.
      The flagged 12-cell burst: sigma is the 1/p filter v0 bare lacked. */
{
  const out = {};
  for(const [name, cd] of CFGS){
    const arm = armA(cd);
    const ff = channel(arm, 300, foldFill, 31);
    const fb = channel(arm, 800, burstFlagged, 32);
    out[name] = {foldFilled: ff, flaggedBurst: fb};
    console.log(`  [${name}] the Fold filled, 32 unflagged: ${ff.corrected} ok, ${ff.detected} detected, ${ff.wrong} MISCORRECTED | flagged 12-cell row burst ${fb.corrected}/800, ${fb.detected} det`);
  }
  ok(out["v1a"].foldFilled.wrong === 0, `with q the Fold filled must be refused: ${out["v1a"].foldFilled.wrong} wrong`);
  ok(out["v1a"].flaggedBurst.corrected === 800, `flagged burst with q ${out["v1a"].flaggedBurst.corrected}/800`);
  const hole = out["v1a-replaces-q"].foldFilled.wrong / 300;
  console.log(`  A4 ${hole > 0 ? (hole >= 0.15 && hole <= 0.25 ? "MET" : "met, outside the called range") : "MISSED -- the kernel claim is wrong"}: replaces-q miscorrects the Fold filled ${pct(out["v1a-replaces-q"].foldFilled.wrong, 300)} (called 15-25%)`);
  record("a-fold", out);
}

/* 7. push. V is conserved; the three parts are not (v0's loss), and neither
      is R_sigma -- push moves colour between a cell and a non-partner. */
{
  let vHolds = 0, partsHold = 0, sigmaHolds = 0, bHolds = 0, cHolds = 0, T = 200;
  g = mul32(41);
  for(let t = 0; t < T; t++){
    const cells = randCells(L), pushed = pushLeft(cells);
    if(G.residue(cells, A.p) === G.residue(pushed, A.p)) vHolds++;
    const a = E.regionResidues(cells, A), b = E.regionResidues(pushed, A);
    if(a[0] === b[0] && a[1] === b[1] && a[2] === b[2]) partsHold++;
    if(V.sigmaResidue(cells, A) === V.sigmaResidue(pushed, A)) sigmaHolds++;
    const sqB = V.toCells528(randBytes(66), B)[0], chkB = E.checksFor(sqB, B);
    if(E.verify(pushLeft(sqB), chkB, B)) bHolds++;
    const phys = V.permuteC(cells, C), chkC = E.checksFor(cells, C);
    if(E.verify(V.permuteC(pushLeft(phys), C), chkC, C)) cHolds++;
  }
  ok(vHolds === T, `codegg-v1's whole residue should survive push, held ${vHolds}/${T}`);
  console.log(`  push: V holds ${vHolds}/${T}; three parts ${partsHold}/${T}; R_sigma ${sigmaHolds}/${T}; arm b ${bHolds}/${T}; arm c ${cHolds}/${T}  (all predicted losses)`);
  record("push", {T, vHolds, partsHold, sigmaHolds, bHolds, cHolds});
}

/* 8. cost, from the format. Two conventions for arm b, both printed always. */
{
  const a  = V.sizesA({N, L, p: A.p, q: A.q, confirm: true, bytes: 128});
  const a0 = V.sizesA({N, L, p: A0.p, q: 0, confirm: false, bytes: 128});
  const b  = V.sizesB({N, L, K: B.K, p: B.p, q: B.q, confirm: true, bytes: 66});
  const c  = E.sizes({N, L, p: C.p, q: C.q, confirm: true, bytes: 128});
  const v0 = E.sizes({N, L, p: V0.p, q: V0.q, confirm: true, bytes: 128});
  const v1 = G.sizes({N, L, p: V0.p, q: G.pickModulus(L, [V0.p]), bytes: 128});
  ok(a.bitsPerSquare === 60, `arm a carries ${a.bitsPerSquare} bits per square, predicted 60`);
  console.log(`  cost per square -- v1(a) ${a.bitsPerSquare} bits = ${pct(a.overhead, 1)} (replaces-q ${pct(a0.overhead, 1)}); v1(b) ${pct(b.overhead, 1)} per data bit / ${pct(b.share, 1)} redundant share of the artifact / ${pct(b.shareCells, 1)} mirrored cells over the square; v1(c) ${pct(c.overhead, 1)}; v0 ${pct(v0.overhead, 1)}; codegg-v1 ${pct(v1.overhead, 1)}`);
  record("sizes", {a, aReplacesQ: a0, b, c, v0, codeggV1: v1});
}

/* 9. v1(c) against v0 on the same seeds. encode o encode = id exactly, and
      every channel within 2 sigma; the deterministic ones identical. The
      one thing that should move: which region fires on a row burst. */
{
  g = mul32(51);
  let inv = 0;
  for(let t = 0; t < 500; t++){ const c = randCells(L); if(same(V.permuteC(V.permuteC(c, C), C), c)) inv++; }
  ok(inv === 500, `encode o encode = id on ${inv}/500`);
  const chans = [["1 flip", 3000, flip1, true], ["2 cross-region", 1000, flip2cross, true], ["3 one-per-region", 1000, flip3, true],
                 ["2 same-region", 2000, flip2same, false], ["2 anywhere", 2000, flip2, false], ["12 flagged burst", 800, burstFlagged, false],
                 ["Fold filled", 300, foldFill, false], ["12-cell unflagged row burst", 400, burstRow, false]];
  const out = {involution: inv};
  let worst = 0;
  chans.forEach(([name, T, dmg, exact], k) => {
    const v0 = channel(armV0, T, dmg, 60 + k), c = channel(armC, T, dmg, 60 + k);
    const pHat = (v0.corrected + c.corrected) / (2 * T), sd = Math.sqrt(2 * pHat * (1 - pHat) / T) * T || 1;
    const z = Math.abs(v0.corrected - c.corrected) / sd;
    worst = Math.max(worst, z);
    out[name] = {v0, c, z: exact ? 0 : z};
    if(exact) ok(v0.corrected === c.corrected && v0.wrong === c.wrong && v0.detected === c.detected, `${name}: v0 ${v0.corrected}/${T} vs c ${c.corrected}/${T} -- should be identical`);
    else ok(z <= 2 || v0.corrected === c.corrected, `C2 MISSED on ${name}: v0 ${v0.corrected} vs c ${c.corrected}, z = ${z.toFixed(2)}`);
    console.log(`  [c vs v0] ${name.padEnd(28)} v0 ${String(v0.corrected).padStart(4)} ok ${String(v0.detected).padStart(4)} det ${v0.wrong} wrong | c ${String(c.corrected).padStart(4)} ok ${String(c.detected).padStart(4)} det ${c.wrong} wrong ${exact ? "(identical, as required)" : `z = ${z.toFixed(2)}`}`
      + (name.startsWith("12-cell unflagged") ? `\n     fires: v0 inner ${v0.fires.inner} fold ${v0.fires.fold} outer ${v0.fires.outer} | c inner ${c.fires.inner} fold ${c.fires.fold} outer ${c.fires.outer}` : ""));
  });
  const rb = out["12-cell unflagged row burst"];
  ok(rb.v0.fires.inner === rb.c.fires.outer && rb.v0.fires.outer === rb.c.fires.inner && rb.v0.fires.fold === rb.c.fires.fold,
     `inner/outer fires should swap under sigma: v0 ${JSON.stringify(rb.v0.fires)} c ${JSON.stringify(rb.c.fires)}`);
  console.log(`  C1 MET: encode o encode = id 500/500. C2 MET: every channel within 2 sigma of v0 (worst z = ${worst.toFixed(2)}); inner/outer fires swap, Fold unchanged.`);
  record("c", out);
}

/* 10. v1(b). The mirror decoder: a mismatch names the pair, the residue
       that moved names the side. Its one row: the unflagged in-region burst. */
{
  const out = {};
  const mk = () => V.toCells528(randBytes(66), B)[0];
  const run = (name, T, dmg, seed) => { out[name] = channel(armB, T, dmg, seed, mk); return out[name]; };
  const s  = run("1 flip", 3000, flip1, 71);
  const d  = run("2 anywhere", 2000, flip2, 72);
  const sr = run("2 same-region", 1000, flip2same, 73);
  const cr = run("2 cross-region", 1000, flip2cross, 74);
  const pp = run("both members of one pair", 500, (h, cd) => { const m = cd.members[E.INNER]; const a = m[pick(m.length)]; h[a] ^= 1; h[cd.sig[a]] ^= 1; }, 75);
  const ffo = run("Fold-Fold", 500, foldPair, 76);
  const th = run("3 one-per-region", 1000, flip3, 77);
  const ub = run("12 unflagged in-region row burst", 800, burstInRegion, 78);
  const fb = run("12 flagged row burst", 800, burstFlagged, 79);
  const wi = run("whole Inner erased", 100, (h, cd) => { const F = [...cd.members[E.INNER]]; for(const j of F) h[j] = -1; return F; }, 80);
  const ff = run("Fold filled", 300, foldFill, 81);
  for(const [name, r] of Object.entries(out))
    console.log(`  [b] ${name.padEnd(34)} ${String(r.corrected).padStart(4)} ok ${String(r.detected).padStart(4)} det ${String(r.wrong).padStart(2)} WRONG  direct ${r.direct} searched ${r.searched}`);
  ok(s.corrected === 3000 && s.wrong === 0, `B2: singles ${s.corrected}/3000, ${s.wrong} wrong`);
  ok(d.wrong === 0 && d.corrected / 2000 >= 0.99, `B3: doubles anywhere ${d.corrected}/2000, ${d.wrong} wrong`);
  ok(ub.wrong === 0 && ub.corrected / 800 >= 0.95, `B4 MISSED: unflagged in-region burst ${ub.corrected}/800, ${ub.wrong} wrong`);
  ok(fb.corrected === 800 && wi.corrected === 100, `B5: flagged burst ${fb.corrected}/800, whole Inner ${wi.corrected}/100`);
  ok(ff.wrong === 0, `Fold filled miscorrected ${ff.wrong}`);
  const anyWrong = Object.values(out).reduce((n, r) => n + r.wrong, 0);
  console.log(`  B2 MET ${s.corrected}/3000 · B3 ${d.corrected / 2000 >= 0.99 ? "MET" : "MISSED"} ${pct(d.corrected, 2000)} · B4 ${ub.corrected / 800 >= 0.99 ? "MET" : ub.corrected / 800 >= 0.95 ? "met at the lower bar" : "MISSED"} ${pct(ub.corrected, 800)} unflagged in-region burst · B5 MET · ${anyWrong} miscorrections on any channel`);
  record("b", out);
}

console.log("eggso1 ok");
