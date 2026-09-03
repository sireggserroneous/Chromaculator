/* node eggSo-v3/tools/eggso3.test.js -- outside the square.
 *
 * Every claim below was predicted in PREDICTIONS.md before this file was
 * first run. Where a prediction MISSED, the test measures and records rather
 * than asserting the wish. Asserts are kept for what would be a bug (the
 * moduli, round-trip, the involution) and for the bars.
 *
 * #1 re-derives the cached primes by search and re-verifies them by
 * enumeration, because a cached constant that the search does not reproduce
 * is a failure and not a shortcut. #4 is the round: one corrupted BYTE, the
 * injury the lineage never ran, across every radix. #6 is the file-scale
 * fold, and #7 is the capacity it buys for nothing. */
const X = require(__dirname + "/../eggso3.js");
const E = require(__dirname + "/../../eggSo-v0/eggso.js");
const fs = require("fs"), path = require("path");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const record = (name, obj) => fs.writeFileSync(path.join(__dirname, "..", `measured-${name}.json`), JSON.stringify(obj, null, 1));
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
let g = mul32(20260902);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const pick = n => g() % n;
const eq = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
const same = (a, b) => Buffer.from(a).equals(Buffer.from(b));
const pct = (x, n) => (100 * x / n).toFixed(2) + "%";
const root = path.join(__dirname, "..", "..");

const CFGS = [["bit", 2, 32], ["nibble", 16, 16], ["nibble", 16, 32], ["byte", 256, 16], ["byte", 256, 32]];
const codes = CFGS.map(([, A, N]) => X.makeCode(N, A));

/* 1. A1. The moduli: re-derived by search, re-verified by enumeration. */
{
  const out = [];
  CFGS.forEach(([name, A, N], k) => {
    const L = N * N, code = codes[k];
    const t0 = Date.now();
    const p = X.pickModulus(L, A), q = X.pickModulus(L, A, [p]);
    ok(p === code.p && q === code.q, `${name} A=${A} N=${N}: search found (${p}, ${q}), cached (${code.p}, ${code.q})`);
    ok(X.injectiveByEnumeration(p, L, A) && X.injectiveByEnumeration(q, L, A), `${name} A=${A} N=${N}: enumeration says (${p}, ${q}) is not injective`);
    const per = 3 * Math.ceil(Math.log2(p)) + Math.ceil(Math.log2(q));
    out.push({name, A, N, L, blockBytes: code.blockBytes, p, q, bitsP: Math.ceil(Math.log2(p)), per, overhead: per / (code.blockBytes * 8), searchMs: Date.now() - t0});
    console.log(`  A=${String(A).padStart(3)} N=${N} L=${String(L).padStart(4)} block ${String(code.blockBytes).padStart(4)} B  p=${String(p).padStart(7)} q=${String(q).padStart(7)}  ${per} check bits  ${pct(per, code.blockBytes * 8)}  (search ${Date.now() - t0} ms, injectivity re-verified by enumeration)`);
  });
  const naive = 2 * 255 * 256;
  console.log(`  A1 MET: five moduli re-derived and re-verified. The naive bound 2(A-1)L for A=256 N=16 is ${naive}; the prime is ${codes[3].p}, ${(codes[3].p / naive).toFixed(1)}x larger -- the ratios d2/d1 cost more than the count does.`);
  record("moduli", out);
}

/* 2. A2. Round-trip through the artifact, every radix, seven shapes, sigma
      both ways. The last block is partial in most of them. */
{
  let n = 0;
  for(let k = 0; k < CFGS.length; k++) for(const sigma of [false, true])
    for(const len of [0, 1, 7, codes[k].blockBytes, codes[k].blockBytes + 1, 1000, 4097]){
      const src = randBytes(len), enc = X.encode(src, {N: CFGS[k][2], A: CFGS[k][1], code: codes[k], sigma});
      const out = X.decode(enc.artifact, {code: codes[k]});
      ok(out.ok && same(out.bytes, src) && !out.corrected && !out.detected, `${CFGS[k][0]} A=${CFGS[k][1]} sigma=${sigma}: round-trip broke at ${len} bytes`);
      n++;
    }
  console.log(`  A2 MET: round-trip exact through the artifact, ${n} shapes (5 radices x sigma on/off x 7 lengths)`);
}

/* 3. A3. One corrupted digit: the syndrome names the cell, the sign and the
      size, at every radix. Injectivity makes the candidate unique. */
{
  const out = {};
  CFGS.forEach(([name, A, N], k) => {
    const code = codes[k];
    let corrected = 0, wrong = 0, detected = 0;
    for(let t = 0; t < 3000; t++){
      const src = randBytes(code.blockBytes), enc = X.encode(src, {N, A, code});
      const sq = enc.squares[0], h = sq.slice(), i = pick(code.L);
      let v = pick(A); while(v === sq[i]) v = pick(A);
      h[i] = v;
      const r = X.repairSquare(h, enc.checks[0], code);
      if(r.status === "corrected"){ if(eq(h, sq)) corrected++; else wrong++; } else detected++;
    }
    out[`${name}/${code.blockBytes}B`] = {corrected, detected, wrong, of: 3000};
    ok(corrected === 3000 && wrong === 0, `A3: ${name} A=${A} N=${N} single digit ${corrected}/3000, ${wrong} wrong`);
  });
  console.log(`  A3 MET: one corrupted digit 3000/3000 at every radix, all direct, 0 miscorrected`);
  record("digits", out);
}

/* 4. A4, THE ROUND. One corrupted BYTE -- the injury real storage delivers,
      and the one no round before this ever ran. In a bit square it is eight
      cells in one row; in a nibble square two adjacent cells; in a byte
      square one cell. */
{
  const out = {};
  CFGS.forEach(([name, A, N], k) => {
    const code = codes[k];
    let corrected = 0, wrong = 0, detected = 0;
    for(let t = 0; t < 3000; t++){
      const src = randBytes(code.blockBytes), enc = X.encode(src, {N, A, code});
      const sq = enc.squares[0], h = sq.slice();
      const b = pick(code.blockBytes), {cells} = X.cellsOfByte(b, code);
      const nv = pick(256);
      if(code.digitBits === 8) h[cells[0]] = nv;
      else if(code.digitBits === 4){ h[cells[0]] = nv >> 4; h[cells[1]] = nv & 15; }
      else for(let j = 0; j < 8; j++) h[cells[j]] = (nv >> (7 - j)) & 1;
      if(eq(h, sq)){ t--; continue; }
      const r = X.repairSquare(h, enc.checks[0], code);
      if(r.status === "corrected"){ if(eq(h, sq)) corrected++; else wrong++; } else detected++;
    }
    out[`${name}/${code.blockBytes}B`] = {corrected, detected, wrong, of: 3000};
    console.log(`  one corrupted BYTE, ${(name + "/" + code.blockBytes + "B").padEnd(12)} ${String(corrected).padStart(4)} corrected, ${String(detected).padStart(4)} detected, ${String(wrong).padStart(3)} MISCORRECTED`);
  });
  const bit = out["bit/128B"], byte32 = out["byte/1024B"];
  ok(byte32.corrected === 3000 && byte32.wrong === 0, `A4 MISSED: byte square ${byte32.corrected}/3000, ${byte32.wrong} wrong`);
  ok(out["nibble/512B"].corrected >= 2850 && out["nibble/512B"].wrong === 0, `A4: nibble square ${out["nibble/512B"].corrected}/3000`);
  console.log(`  A4 MET for the byte square: 3000/3000, every one direct. And the miss that matters: the BIT square -- eggSo-v0's, v1's and v2's square --`);
  console.log(`     corrects ${bit.corrected}/3000 and MISCORRECTS ${bit.wrong}. PREDICTIONS called "~0 corrected, 0 wrong"; the 0 wrong was wrong.`);
  record("bytes", out);
}

/* 5. A5. An erasure at a known address is one unknown in one equation, so it
      is SOLVED. At A = 256 there is no enumeration to fall back on. */
{
  const out = {};
  CFGS.forEach(([name, A, N], k) => {
    const code = codes[k];
    let one = 0, two = 0, twoWrong = 0;
    for(let t = 0; t < 1000; t++){
      const src = randBytes(code.blockBytes), enc = X.encode(src, {N, A, code});
      const sq = enc.squares[0], h = sq.slice();
      const F = [0, 1, 2].map(r => code.members[r][pick(code.members[r].length)]);
      for(const i of F) h[i] = 0;
      const r = X.repairSquare(h, enc.checks[0], code, {erased: F});
      if(r.status === "corrected" && eq(h, sq)) one++;
      const h2 = sq.slice(), m = code.members[X.INNER];
      let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)];
      h2[a] = 0; h2[b] = 0;
      const r2 = X.repairSquare(h2, enc.checks[0], code, {erased: [a, b]});
      if(r2.status === "corrected"){ if(eq(h2, sq)) two++; else twoWrong++; }
    }
    out[`${name}/${code.blockBytes}B`] = {onePerRegion: one, twoInOneRegion: two, twoWrong, of: 1000};
    ok(one === 1000, `A5: ${name} one erasure per region solved ${one}/1000`);
    ok(twoWrong === 0, `A5: ${name} two erasures in one region miscorrected ${twoWrong}`);
    console.log(`  ${(name + "/" + code.blockBytes + "B").padEnd(12)} one flagged cell per region: ${one}/1000 SOLVED (no enumeration) · two in one region: ${two}/1000 recovered, ${twoWrong} wrong`);
  });
  console.log(`  A5 MET: the solve is exact wherever there is one unknown per equation, at every radix.`);
  record("erasures", out);
}

/* 6. B1, B2. The file-scale fold: an involution that stores nothing, and a
      scatter whose density is the ground table. */
{
  let inv = 0, T = 0;
  for(const M of [1, 7, 128, 1000, 4097, 15190, 92408]){
    const sig = X.fileSigma(M);
    let good = true;
    for(let j = 0; j < M; j++) if(sig[sig[j]] !== j) good = false;
    const src = randBytes(M);
    if(good && same(X.scatter(X.scatter(src, sig), sig), src)) inv++;
    T++;
  }
  ok(inv === T, `B1 MISSED: sigma is not an involution on ${T - inv} of ${T} lengths`);
  /* PREDICTIONS.md's ground table, and beside it what the codec's own sigma
     does. They differ on the two smallest files and the difference is a
     defect in the ground, not in the codec: the planning script DROPPED
     partners that fell past the end of the file, which is not a permutation.
     fileSigma keeps them as fixed points, so nothing is lost and a few more
     bytes stay where they were. Both numbers are kept. */
  const GROUND = {"spec.md": 37, "stalk.js": 30, "og.png": 23, "wubbadub.html": 14};
  const density = {};
  let drift = 0;
  for(const [name, worst] of Object.entries(GROUND)){
    const file = path.join(root, name);
    if(!fs.existsSync(file)) continue;
    const M = fs.statSync(file).size, sig = X.fileSigma(M), at = Math.floor(M / 2) - 2048, per = new Map();
    for(let j = Math.max(0, at); j < at + 4096 && j < M; j++){ const b = Math.floor(sig[j] / 128); per.set(b, (per.get(b) || 0) + 1); }
    let mx = 0; for(const v of per.values()) mx = Math.max(mx, v);
    density[name] = {worstBlock128: mx, ground: worst, blocksTouched: per.size};
    if(mx !== worst) drift++;
    ok(mx <= worst + 4, `B2 MISSED badly: ${name} worst block ${mx}, ground says ${worst}`);
  }
  console.log(`  B1 MET: sigma o sigma = id on ${inv}/${T} file lengths, and it stores nothing -- the artifact is the source's size plus checks`);
  console.log(`  B2 met, with the ground corrected: worst 128-B block  ` + Object.entries(density).map(([n, d]) => `${n} ${d.worstBlock128}${d.worstBlock128 !== d.ground ? ` (ground said ${d.ground})` : ""}`).join(", "));
  if(drift) console.log(`     ${drift} file(s) differ because the planning script dropped out-of-range partners instead of fixing them in place; only the codec's sigma is a permutation.`);
  record("sigma", {involution: {held: inv, of: T}, density});
}

/* 7. B3, B4. What sigma is worth, in bytes. The largest contiguous scratch
      survived exactly, with sigma and without, and the same for a truncation
      -- which with the length carried is the same injury. */
{
  const survives = (src, cfg, sigma, len, kind) => {
    const [, A, N] = cfg, code = codes[CFGS.indexOf(cfg)];
    const enc = X.encode(src, {N, A, code, sigma});
    const payStart = 10 + enc.meta.squares * enc.meta.cb;
    const b = Buffer.from(enc.artifact);
    if(kind === "trunc"){
      const art = new Uint8Array(b.subarray(0, Math.max(payStart, b.length - len)));
      try { const o = X.decode(art, {code}); return o.ok && same(o.bytes, src); } catch(e){ return false; }
    }
    const gg = mul32(0xACE), at = Math.max(payStart, payStart + Math.floor((b.length - payStart) / 2) - (len >> 1));
    const n = Math.min(len, b.length - at);
    for(let i = at; i < at + n; i++) b[i] = gg() & 0xff;
    try { const o = X.decode(new Uint8Array(b), {code, wound: {at, len: n}}); return o.ok && same(o.bytes, src); } catch(e){ return false; }
  };
  const largest = (src, cfg, sigma, kind) => {
    if(!survives(src, cfg, sigma, 1, kind)) return 0;
    let lo = 1, hi = 2;
    while(hi < src.length && survives(src, cfg, sigma, hi, kind)){ lo = hi; hi *= 2; }
    while(lo + 1 < hi){ const mid = (lo + hi) >> 1; if(survives(src, cfg, sigma, mid, kind)) lo = mid; else hi = mid; }
    return lo;
  };
  const out = {};
  for(const name of ["spec.md", "wubbadub.html"]){
    const file = path.join(root, name);
    if(!fs.existsSync(file)) continue;
    const src = fs.readFileSync(file);
    out[name] = {};
    for(const cfg of [CFGS[0], CFGS[4]]){
      const key = `${cfg[0]}/${codes[CFGS.indexOf(cfg)].blockBytes}B`;
      const bare = largest(src, cfg, false, "scratch"), withS = largest(src, cfg, true, "scratch"), tr = largest(src, cfg, true, "trunc");
      out[name][key] = {scratchBare: bare, scratchSigma: withS, truncSigma: tr, gain: bare ? withS / bare : null};
      console.log(`  ${name.padEnd(14)} ${key.padEnd(12)} largest scratch survived: ${String(bare).padStart(5)} B bare -> ${String(withS).padStart(5)} B with sigma (x${bare ? (withS / bare).toFixed(0) : "-"}); truncation with sigma ${tr} B`);
      ok(withS >= bare, `B3: sigma made ${key} on ${name} worse (${bare} -> ${withS})`);
    }
  }
  const s = out["wubbadub.html"];
  if(s) ok(s["bit/128B"].truncSigma > 0 && s["byte/1024B"].truncSigma > 0, `B4 MISSED: truncation not survivable at any size with sigma`);
  console.log(`  B3/B4: sigma costs no bytes and multiplies the survivable wound; truncation and scratch become the same injury once the length is carried.`);
  record("capacity", out);
}

/* 8. A6. Cost, from the format, beside the lineage's own number. */
{
  const out = {};
  CFGS.forEach(([name, A, N], k) => {
    const code = codes[k], cb = Math.ceil((3 * Math.ceil(Math.log2(code.p)) + Math.ceil(Math.log2(code.q))) / 8);
    const s = X.sizes({squares: 1, bytes: code.blockBytes, cb, headerBytes: 10}, code);
    out[`${name}/${code.blockBytes}B`] = {blockBytes: code.blockBytes, bitsPerSquare: s.bitsPerSquare, idealOverhead: s.overheadIdeal};
  });
  console.log("  A6: " + Object.entries(out).map(([n, v]) => `${n} ${(100 * v.idealOverhead).toFixed(2)}%`).join(" · ") + "   (bit/128B is eggSo-v0 exactly)");
  record("sizes", out);
}

console.log("eggso3 ok");
