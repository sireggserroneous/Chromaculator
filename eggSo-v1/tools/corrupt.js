/* node eggSo-v1/tools/corrupt.js <file> [flags] -- a real file through one arm of eggSo v1.
 *
 *   --arm a|b|c                arm (a) the sigma residue, (b) the mirror, (c) the interleaver
 *   --N 32                     square side
 *   --model uniform|pair|fold|burst
 *                              uniform: per-cell flip probability (--rate)
 *                              pair:    two flips per hit square, random cells
 *                              fold:    every hit square has its Fold filled
 *                              burst:   12 UNFLAGGED flips in one row, inside one region
 *   --rate 0.001               uniform only
 *   --hits 50                  pair/fold/burst: how many squares to hit
 *   --bare                     arm a only: sigma in place of q (the negative control)
 *   --quiet                    one line
 *
 * Three outcomes, and they are different failures:
 *   SILENTLY WRONG   decoded "clean" while differing from the source   exit 3
 *   MISCORRECTED     "repaired" into the wrong data                    exit 3
 *   detected         refused to guess -- honest, and costs no data     exit 2
 *   EXACT                                                              exit 0 */
const fs = require("fs"), path = require("path");
const V = require(path.join(__dirname, "..", "eggso1.js"));
const E = require(path.join(__dirname, "..", "..", "eggSo-v0", "eggso.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["N", "model", "rate", "hits", "arm"]);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const has = n => argv.indexOf("--" + n) >= 0;
let file = null;
for(let i = 0; i < argv.length; i++){ const a = argv[i]; if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; } file = a; break; }
if(!file){ console.error("usage: corrupt.js <file> [--arm a|b|c] [--N 32] [--model uniform|pair|fold|burst] [--rate 0.001] [--hits 50] [--bare]"); process.exit(1); }

const N = parseInt(flag("N", "32"), 10), model = flag("model", "uniform"), arm = flag("arm", "a");
const rate = parseFloat(flag("rate", "0.001")), hits = parseInt(flag("hits", "50"), 10);
const src = fs.readFileSync(file);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(20260902), pick = n => g() % n, unit = () => g() / 4294967296;

const ARMS = {
  a: {encode: b => V.encodeA(b, {N, confirm: !has("bare")}), decode: V.decodeA, sizes: V.sizesA, label: "v1(a) R_sigma" + (has("bare") ? " in place of q" : " + q")},
  b: {encode: b => V.encodeB(b, {N}), decode: V.decodeB, sizes: V.sizesB, label: "v1(b) mirror"},
  c: {encode: b => V.encodeC(b, {N}), decode: V.decodeC, sizes: m => E.sizes(m), label: "v1(c) interleaver"},
};
const X = ARMS[arm]; if(!X){ console.error("unknown arm " + arm); process.exit(1); }

const t0 = Date.now();
const enc = X.encode(src);
const tEnc = Date.now() - t0;
const code = enc.code, L = code.L;

/* damage, per model, on the stored squares */
let flips = 0, hitSq = 0;
if(model === "uniform"){
  for(const sq of enc.squares) for(let i = 0; i < L; i++) if(unit() < rate){ sq[i] ^= 1; flips++; }
} else {
  const chosen = new Set(); while(chosen.size < Math.min(hits, enc.squares.length)) chosen.add(pick(enc.squares.length));
  for(const s of chosen){
    const sq = enc.squares[s]; hitSq++;
    if(model === "pair"){ let a = pick(L), b = pick(L); while(b === a) b = pick(L); sq[a] ^= 1; sq[b] ^= 1; flips += 2; }
    else if(model === "fold"){ for(const i of code.members[E.FOLD]) sq[i] ^= 1; flips += code.members[E.FOLD].length; }
    else if(model === "burst"){
      for(;;){
        const r = pick(N), c0 = pick(N - 12), regs = new Set();
        for(let j = 0; j < 12; j++) regs.add(code.region[r * N + c0 + j]);
        if(regs.size === 1 && !regs.has(E.FOLD)){ for(let j = 0; j < 12; j++) sq[r * N + c0 + j] ^= 1; flips += 12; break; }
      }
    }
    else { console.error("unknown model " + model); process.exit(1); }
  }
}

const t1 = Date.now();
const out = X.decode(enc);
const tDec = Date.now() - t1;
const exact = Buffer.from(out.bytes).equals(src);
const silentlyWrong = !exact && out.detected === 0;
/* a square that still differs from the source was either left marked
   (detected) or "repaired" wrong. Detected squares keep their damage, so the
   difference between the two counts is the miscorrections. */
const truth = X.encode(src).squares;
let differing = 0;
for(let s = 0; s < truth.length; s++) if(!Buffer.from(enc.squares[s]).equals(Buffer.from(truth[s]))) differing++;
const miscorrected = Math.max(0, differing - out.detected);
const sizes = X.sizes(enc.meta);

if(has("quiet")){
  console.log(`${path.basename(file)} ${src.length}B ${X.label} N=${N} ${model} flips=${flips} -> ${exact ? "EXACT" : silentlyWrong ? "SILENTLY WRONG" : miscorrected ? `MISCORRECTED ${miscorrected} squares` : "detected, not exact"} corrected=${out.corrected} detected=${out.detected} direct=${out.direct} searched=${out.searched} overhead=${(sizes.overhead*100).toFixed(2)}%`);
} else {
  console.log(`${path.basename(file)}: ${src.length} bytes -> ${sizes.squares} squares of ${N}x${N}, ${X.label}, ${sizes.residuesPerSquare} residues each`);
  console.log(`  overhead ${(sizes.overhead * 100).toFixed(2)}%${sizes.share !== undefined && arm === "b" ? ` per data bit (${(sizes.share * 100).toFixed(1)}% redundant share of the artifact)` : ""}  (${sizes.checkBytes} check bytes on ${sizes.dataBytes})`);
  console.log(`  damage: model=${model}${model === "uniform" ? ` rate=${rate}` : ` hits=${hitSq}`} -> ${flips} cells flipped`);
  console.log(`  decode: clean ${out.clean}, corrected ${out.corrected} (${out.direct} cells direct, ${out.searched} by search), detected ${out.detected}`);
  console.log(`  result: ${exact ? "EXACT round-trip" : silentlyWrong ? "SILENTLY WRONG -- decoded clean but differs" : miscorrected ? `MISCORRECTED -- ${miscorrected} squares "repaired" into the wrong data` : "not exact; damage was detected and left marked"}`);
  console.log(`  time: encode ${tEnc} ms, decode ${tDec} ms`);
}
process.exit(exact ? 0 : (silentlyWrong || miscorrected) ? 3 : 2);
