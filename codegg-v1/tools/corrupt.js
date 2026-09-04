/* node codegg-v1/tools/corrupt.js <file> [flags] -- real files through codegg.
 *
 * usage:
 *   --N 32                    square side
 *   --model uniform|burst|sentinel   error model, default uniform
 *   --rate 0.001              uniform: probability per cell (bit flips)
 *   --burst 12                burst: run length, one row
 *   --hits 20                 burst/sentinel: how many to inject
 *   --erase                   burst only: hand the decoder the damaged range,
 *                             turning the burst into flagged erasures
 *   --no-doubles              disable the double-error search
 *   --quiet                   one line
 *
 * Two numbers matter in the output, and they are different failures:
 *   SILENTLY WRONG -- decoded clean while differing from the source.
 *   MISCORRECTED   -- "repaired" into the wrong data. codec-v1 cannot do
 *                     this; codegg can, at the measured rate in the README.
 * Detected-but-unrepaired is the honest third outcome and costs no data.
 *
 * As with codec-v1: single-error correction is a property of the checks, not
 * the bytes, so every file reports the same rates. Files are here for
 * overhead and plumbing against awkward real sizes. */
const fs = require("fs"), path = require("path");
const G = require(path.join(__dirname, "..", "codegg.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["N", "model", "rate", "burst", "hits"]);
const flag = (name, dflt) => {
  const i = argv.indexOf("--" + name);
  return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : dflt;
};
const has = name => argv.indexOf("--" + name) >= 0;
let file = null;
for(let i = 0; i < argv.length; i++){
  const a = argv[i];
  if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; }
  file = a; break;
}
if(!file){ console.error("usage: corrupt.js <file> [--N 32] [--model uniform|burst|sentinel] [--erase]"); process.exit(1); }

const N = +flag("N", 32);
const model = flag("model", "uniform");
const rate = +flag("rate", 0.001);
const burstLen = +flag("burst", 12);
const hits = +flag("hits", 20);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(0xE66E66);

const src = fs.readFileSync(file);
const code = G.makeCode(N);
const p = G.encode(src, {N, code});
const s = G.sizes(p.meta);
if(!p.squares.length){ console.log(`${file}: empty, nothing to do`); process.exit(0); }

/* ---- inject ---- */
let injected = 0;
const erased = new Map();
if(model === "uniform"){
  const thresh = rate * 0x100000000;
  for(let sq = 0; sq < p.squares.length; sq++)
    for(let i = 0; i < code.L; i++)
      if(g() < thresh){ p.squares[sq][i] = p.squares[sq][i] === 1 ? 0 : 1; injected++; }
} else if(model === "burst"){
  for(let h = 0; h < hits; h++){
    const sq = g() % p.squares.length, row = g() % N;
    const c0 = g() % Math.max(1, N - burstLen);
    const F = [];
    for(let j = 0; j < burstLen && c0 + j < N; j++){
      const i = row * N + c0 + j;
      p.squares[sq][i] = g() % 2; injected++;
      F.push(i);
    }
    if(has("erase")){
      if(!erased.has(sq)) erased.set(sq, []);
      erased.get(sq).push(...F);
    }
  }
} else if(model === "sentinel"){
  for(let h = 0; h < hits; h++){
    const sq = g() % p.squares.length;
    p.squares[sq][g() % code.L] = -1; injected++;
  }
} else { console.error("unknown model " + model); process.exit(1); }

/* ---- decode and compare ---- */
const opts = {doubles: !has("no-doubles")};
if(erased.size) opts.erased = erased;
const out = G.decode(p, opts);
const got = Buffer.from(out.bytes);
const exact = got.length === src.length && got.equals(src);
let diffs = 0, firstDiff = -1;
for(let i = 0; i < Math.max(got.length, src.length); i++)
  if(got[i] !== src[i]){ diffs++; if(firstDiff < 0) firstDiff = i; }
const silent = !exact && out.detected === 0 && out.corrected === 0;
const miscorrected = !exact && out.detected === 0 && out.corrected > 0;

if(has("quiet")){
  console.log(`${path.basename(file).padEnd(16)} ${model.padEnd(9)}`
    + ` inj ${String(injected).padStart(5)}  fixed ${String(out.fixed).padStart(5)}  `
    + (exact ? "EXACT" : miscorrected ? "MISCORRECTED" : silent ? "SILENTLY WRONG" : "detected, unrepaired"));
  process.exit(exact ? 0 : 1);
}

console.log(`${file}  ${src.length} bytes  ->  codegg, N=${N}, p=${code.p}, q=${code.q}`);
console.log(`  format     ${s.squares} squares, data ${s.dataBytes}B verbatim + checks ${s.checkBytes}B`
  + ` = ${s.totalBytes}B  (${s.ratio.toFixed(3)}x source, check overhead ${(100 * s.overhead).toFixed(2)}%)`);
console.log(`  model      ${model}${model === "uniform" ? ` rate=${rate}` : ` hits=${hits}`
  + (model === "burst" ? ` len=${burstLen}${has("erase") ? " (flagged as erasures)" : " (unflagged)"}` : "")}`);
console.log(`  injected   ${injected} cell errors across ${p.squares.length} squares`);
console.log(`  squares    ${out.clean} clean, ${out.corrected} corrected, ${out.detected} detected`);
console.log(`  cells      ${out.fixed} repaired`);
console.log(`  result     ${exact ? "EXACT round-trip" : `${diffs} bytes differ, first at ${firstDiff}`}`);
console.log(`  MISCORRECTED:   ${miscorrected ? "YES -- repaired into wrong data" : "no"}`);
console.log(`  SILENTLY WRONG: ${silent ? "YES -- data lost without warning" : "no"}`);
process.exit(exact || (!silent && !miscorrected) ? 0 : 1);
