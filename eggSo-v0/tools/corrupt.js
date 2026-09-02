/* node eggSo-v0/tools/corrupt.js <file> [flags] -- a real file through eggSo.
 *
 *   --N 32                     square side
 *   --model uniform|pair|fold  uniform: per-cell flip probability (--rate)
 *                              pair:    two flips per hit square, random cells
 *                              fold:    every hit square has its Fold filled
 *   --rate 0.001               uniform only
 *   --hits 50                  pair/fold: how many squares to hit
 *   --bare                     drop the whole-square confirming residue (the
 *                              construction as first filed; see PREDICTIONS)
 *   --quiet                    one line
 *
 * Three outcomes, and they are different failures:
 *   SILENTLY WRONG   decoded "clean" while differing from the source
 *   MISCORRECTED     "repaired" into the wrong data
 *   detected         refused to guess -- honest, and costs no data
 * As with the whole series, correction is a property of the checks, not the
 * bytes, so every file reports the same rates; files are here for overhead
 * and for plumbing against awkward real sizes. */
const fs = require("fs"), path = require("path");
const E = require(path.join(__dirname, "..", "eggso.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["N", "model", "rate", "hits"]);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const has = n => argv.indexOf("--" + n) >= 0;
let file = null;
for(let i = 0; i < argv.length; i++){ const a = argv[i]; if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; } file = a; break; }
if(!file){ console.error("usage: corrupt.js <file> [--N 32] [--model uniform|pair|fold] [--rate 0.001] [--hits 50] [--bare]"); process.exit(1); }

const N = parseInt(flag("N", "32"), 10), model = flag("model", "uniform");
const rate = parseFloat(flag("rate", "0.001")), hits = parseInt(flag("hits", "50"), 10);
const src = fs.readFileSync(file);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(20260902), pick = n => g() % n, unit = () => g() / 4294967296;

const t0 = Date.now();
const enc = E.encode(src, {N, confirm: !has("bare")});
const tEnc = Date.now() - t0;
const code = enc.code, L = code.L;

/* damage, per model */
let flips = 0, hitSq = 0;
if(model === "uniform"){
  for(const sq of enc.squares) for(let i = 0; i < L; i++) if(unit() < rate){ sq[i] ^= 1; flips++; }
} else {
  const chosen = new Set(); while(chosen.size < Math.min(hits, enc.squares.length)) chosen.add(pick(enc.squares.length));
  for(const s of chosen){
    const sq = enc.squares[s]; hitSq++;
    if(model === "pair"){ let a = pick(L), b = pick(L); while(b === a) b = pick(L); sq[a] ^= 1; sq[b] ^= 1; flips += 2; }
    else if(model === "fold"){ for(const i of code.members[E.FOLD]) sq[i] ^= 1; flips += code.members[E.FOLD].length; }
    else { console.error("unknown model " + model); process.exit(1); }
  }
}

const t1 = Date.now();
const out = E.decode(enc);
const tDec = Date.now() - t1;
const exact = Buffer.from(out.bytes).equals(src);
const silentlyWrong = !exact && out.detected === 0;
const sizes = E.sizes(enc.meta);

if(has("quiet")){
  console.log(`${path.basename(file)} ${src.length}B N=${N} ${model} flips=${flips} -> ${exact ? "EXACT" : silentlyWrong ? "SILENTLY WRONG" : "detected, not exact"} corrected=${out.corrected} detected=${out.detected} direct=${out.direct} searched=${out.searched} overhead=${(sizes.overhead*100).toFixed(2)}%`);
} else {
  console.log(`${path.basename(file)}: ${src.length} bytes -> ${sizes.squares} squares of ${N}x${N}, ${sizes.residuesPerSquare} residues each`);
  console.log(`  overhead ${(sizes.overhead * 100).toFixed(2)}%  (${sizes.checkBytes} check bytes on ${sizes.dataBytes})`);
  console.log(`  damage: model=${model}${model === "uniform" ? ` rate=${rate}` : ` hits=${hitSq}`} -> ${flips} cells flipped`);
  console.log(`  decode: clean ${out.clean}, corrected ${out.corrected} (${out.direct} cells direct, ${out.searched} by search), detected ${out.detected}`);
  console.log(`  result: ${exact ? "EXACT round-trip" : silentlyWrong ? "SILENTLY WRONG -- decoded clean but differs" : "not exact; damage was detected and left marked"}`);
  console.log(`  time: encode ${tEnc} ms, decode ${tDec} ms`);
}
process.exit(exact ? 0 : silentlyWrong ? 3 : 2);
