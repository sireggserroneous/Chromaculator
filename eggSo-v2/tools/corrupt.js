/* node eggSo-v2/tools/corrupt.js <file> [flags] -- a real file through one arm of eggSo v2.
 *
 *   --arm a|b                  (a) forced greens as check slots, (b) greens as erasures
 *   --N 32                     square side
 *   --model uniform|pair|fold|sign|burst
 *                              uniform: per-cell d=+-1 error with probability (--rate)
 *                              pair:    two d=+-1 errors per hit square
 *                              fold:    every hit square has its Fold cells hit (d=+-1)
 *                              sign:    one sign flip (d=+-2) on a lit cell per hit square
 *                              burst:   12-cell FLAGGED row burst per hit square (the erasure arm)
 *   --rate 0.001               uniform only
 *   --hits 50                  pair/fold/sign/burst: how many squares to hit
 *   --quiet                    one line
 *
 * Squares are canonical (pushed); a d=+-1 error greens a lit cell or lights a
 * green, a d=+-2 error flips a sign. Three outcomes, three exit codes:
 *   SILENTLY WRONG / MISCORRECTED   exit 3     detected   exit 2     EXACT   exit 0 */
const fs = require("fs"), path = require("path");
const W = require(path.join(__dirname, "..", "eggso2.js"));
const E = require(path.join(__dirname, "..", "..", "eggSo-v0", "eggso.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["N", "model", "rate", "hits", "arm"]);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const has = n => argv.indexOf("--" + n) >= 0;
let file = null;
for(let i = 0; i < argv.length; i++){ const a = argv[i]; if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; } file = a; break; }
if(!file){ console.error("usage: corrupt.js <file> [--arm a|b] [--N 32] [--model uniform|pair|fold|sign|burst] [--rate 0.001] [--hits 50]"); process.exit(1); }

const N = parseInt(flag("N", "32"), 10), model = flag("model", "uniform"), arm = flag("arm", "b");
const rate = parseFloat(flag("rate", "0.001")), hits = parseInt(flag("hits", "50"), 10);
const src = fs.readFileSync(file);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(20260902), pick = n => g() % n, unit = () => g() / 4294967296;

const ARMS = {
  a: {encode: b => W.encodeA(b, {N}), decode: (e, o) => W.decodeA(e, o), sizes: W.sizesA, label: "v2(a) greens as check slots"},
  b: {encode: b => W.encodeB(b, {N}), decode: (e, o) => W.decodeB(e, o), sizes: W.sizesB, label: "v2(b) greens as erasures"},
};
const X = ARMS[arm]; if(!X){ console.error("unknown arm " + arm); process.exit(1); }

const t0 = Date.now();
const enc = X.encode(src);
const tEnc = Date.now() - t0;
const code = enc.code, L = code.L;
const hit1 = (sq, i) => { sq[i] = sq[i] === 0 ? (pick(2) ? 1 : -1) : 0; };

let flips = 0, hitSq = 0;
const erased = new Map();
if(model === "uniform"){
  for(const sq of enc.squares) for(let i = 0; i < L; i++) if(unit() < rate){ hit1(sq, i); flips++; }
} else {
  const chosen = new Set(); while(chosen.size < Math.min(hits, enc.squares.length)) chosen.add(pick(enc.squares.length));
  for(const s of chosen){
    const sq = enc.squares[s]; hitSq++;
    if(model === "pair"){ let a = pick(L), b = pick(L); while(b === a) b = pick(L); hit1(sq, a); hit1(sq, b); flips += 2; }
    else if(model === "fold"){ for(const i of code.members[E.FOLD]) hit1(sq, i); flips += 32; }
    else if(model === "sign"){ const lit = []; for(let i = 0; i < L; i++) if(sq[i]) lit.push(i); if(lit.length){ const i = lit[pick(lit.length)]; sq[i] = -sq[i]; flips++; } }
    else if(model === "burst"){ const r = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ sq[r * N + c0 + j] = 0; F.push(r * N + c0 + j); } erased.set(s, F); flips += 12; }
    else { console.error("unknown model " + model); process.exit(1); }
  }
}

const t1 = Date.now();
const out = X.decode(enc, erased.size ? {erased} : undefined);
const tDec = Date.now() - t1;
const exact = Buffer.from(out.bytes).equals(src);
const silentlyWrong = !exact && out.detected === 0;
/* truth is the VALUE of each source square; an in-band square's tail holds
   its check, so the comparison zeroes those slots first */
const G = require(path.join(__dirname, "..", "..", "codegg-v1", "codegg.js"));
const truthV = G.toCells(src, L).map(W.valueOf);
let differing = 0;
for(let s = 0; s < truthV.length; s++) if(W.valueOfStored(enc.squares[s], arm === "a" ? enc.flags[s] : 0, code) !== truthV[s]) differing++;
const miscorrected = Math.max(0, differing - out.detected);
const sizes = X.sizes(enc.meta);

if(has("quiet")){
  console.log(`${path.basename(file)} ${src.length}B ${X.label} N=${N} ${model} hits=${flips} -> ${exact ? "EXACT" : silentlyWrong ? "SILENTLY WRONG" : miscorrected ? `MISCORRECTED ${miscorrected} squares` : "detected, not exact"} corrected=${out.corrected} detected=${out.detected} direct=${out.direct} searched=${out.searched}${arm === "a" ? ` inBand=${out.inBand}` : ""} overhead=${(sizes.overhead * 100).toFixed(2)}%`);
} else {
  console.log(`${path.basename(file)}: ${src.length} bytes -> ${sizes.squares} canonical squares of ${N}x${N}, ${X.label}${arm === "a" ? `, ${enc.meta.inBand} in-band (${(100 * (1 - sizes.fallbackRate)).toFixed(2)}%)` : ""}`);
  console.log(`  overhead ${(sizes.overhead * 100).toFixed(2)}%  (${sizes.checkBytes} check bytes on ${sizes.dataBytes})`);
  console.log(`  damage: model=${model}${model === "uniform" ? ` rate=${rate}` : ` hits=${hitSq}`} -> ${flips} cells hit`);
  console.log(`  decode: clean ${out.clean}, corrected ${out.corrected} (${out.direct} cells direct, ${out.searched} by search), detected ${out.detected}`);
  console.log(`  result: ${exact ? "EXACT round-trip" : silentlyWrong ? "SILENTLY WRONG -- decoded clean but differs" : miscorrected ? `MISCORRECTED -- ${miscorrected} squares "repaired" into the wrong data` : "not exact; damage was detected and left marked"}`);
  console.log(`  time: encode ${tEnc} ms, decode ${tDec} ms`);
}
process.exit(exact ? 0 : (silentlyWrong || miscorrected) ? 3 : 2);
