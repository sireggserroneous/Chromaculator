/* node codec-v1/tools/corrupt.js <file> [flags] -- real files through the codec.
 *
 * usage:
 *   --alphabet chroma|byte   default byte
 *   --N 32                   square side
 *   --model uniform|burst|rect|flip   error model, default uniform
 *   --rate 0.001             uniform: probability per symbol
 *   --burst 8                burst: run length
 *   --hits 20                burst/rect/flip: how many to inject
 *   --no-diags               run the decoder without anti-diagonal parity
 *   --quiet                  one line of output
 *
 * The number that matters is SILENTLY WRONG. A detected-but-unrepaired file is
 * a normal, honest outcome for a code that has run out of parity; a file that
 * decodes clean while differing from the source is the failure that costs data.
 *
 * Note on what this can and cannot show: single-error correction is a property
 * of the parity geometry, not of the bytes, so every file here will report the
 * same rate. Real files are measured for OVERHEAD and for plumbing against
 * awkward real sizes -- not to establish correction power. */
const fs = require("fs"), path = require("path");
const C = require(path.join(__dirname, "..", "chromacode.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["alphabet", "N", "model", "rate", "burst", "hits"]);
const flag = (name, dflt) => {
  const i = argv.indexOf("--" + name);
  return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : dflt;
};
const has = name => argv.indexOf("--" + name) >= 0;
/* the file is the first bare argument that is not the value of a valued flag */
let file = null;
for(let i = 0; i < argv.length; i++){
  const a = argv[i];
  if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; }
  file = a; break;
}

if(!file){ console.error("usage: corrupt.js <file> [--alphabet byte|chroma] [--N 32] [--model uniform|burst|rect|flip]"); process.exit(1); }

const alphabet = flag("alphabet", "byte");
const N = +flag("N", 32);
const model = flag("model", "uniform");
const rate = +flag("rate", 0.001);
const burstLen = +flag("burst", 8);
const hits = +flag("hits", 20);
const useDiags = !has("no-diags");

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(0xC0DEC1);

const src = fs.readFileSync(file);
const p = C.encode(src, {N, alphabet});
const alph = C.ALPHABETS[alphabet];
const s = C.sizes(p.meta);
if(!p.squares.length){ console.log(`${file}: empty, nothing to do`); process.exit(0); }

/* ---- inject ---- */
let injected = 0;
const corruptCell = (sq, r, c) => {
  const alts = alph.others(p.squares[sq][r][c]);
  p.squares[sq][r][c] = alts[g() % alts.length];
  injected++;
};
if(model === "uniform"){
  const thresh = rate * 0x100000000;
  for(let sq = 0; sq < p.squares.length; sq++)
    for(let r = 0; r < N; r++) for(let c = 0; c < N; c++)
      if(g() < thresh) corruptCell(sq, r, c);
} else if(model === "burst"){
  for(let h = 0; h < hits; h++){
    const sq = g() % p.squares.length, r = g() % N; let c = g() % N;
    for(let t = 0; t < burstLen && c < N; t++, c++) corruptCell(sq, r, c);
  }
} else if(model === "rect"){
  for(let h = 0; h < hits; h++){
    const sq = g() % p.squares.length;
    const r1 = g() % N, c1 = g() % N;
    let r2 = g() % N, c2 = g() % N;
    if(r2 === r1) r2 = (r2 + 1) % N;
    if(c2 === c1) c2 = (c2 + 1) % N;
    /* A true rectangle leaves every row and column sum exactly where it was.
       Under XOR the same delta on all four corners does that, because XOR is
       its own inverse. Under an integer sum it does NOT -- the deltas have to
       alternate +d -d -d +d. Getting this wrong makes the "blind spot" test
       silently test something else, which is what happened first time. */
    if(alphabet === "byte"){
      const d = 1 + (g() % 255);
      for(const [r, c] of [[r1,c1],[r1,c2],[r2,c1],[r2,c2]]){ p.squares[sq][r][c] ^= d; injected++; }
    } else {
      /* chroma cells are {-1,0,+1}, so a +1/-1 rectangle needs room to move
         both ways: two corners <= 0 and two >= 0. Search for one that fits. */
      let found = null;
      for(let tries = 0; tries < 200 && !found; tries++){
        const a = g() % N, b = g() % N, x = g() % N, y = g() % N;
        if(a === b || x === y) continue;
        const G = p.squares[sq];
        if(G[a][x] <= 0 && G[b][y] <= 0 && G[a][y] >= 0 && G[b][x] >= 0) found = [a, b, x, y];
      }
      if(found){
        const [a, b, x, y] = found, G = p.squares[sq];
        G[a][x] += 1; G[b][y] += 1; G[a][y] -= 1; G[b][x] -= 1;
        injected += 4;
      }
    }
  }
} else if(model === "flip"){
  if(alphabet !== "chroma"){ console.error("--model flip needs --alphabet chroma"); process.exit(1); }
  for(let h = 0; h < hits; h++){
    const sq = g() % p.squares.length;
    for(let tries = 0; tries < 64; tries++){
      const r = g() % N, c = g() % N;
      if(p.squares[sq][r][c] !== 0){ p.squares[sq][r][c] *= -1; injected++; break; }
    }
  }
} else { console.error("unknown model " + model); process.exit(1); }

/* ---- decode and compare ---- */
const out = C.decode(p, {useDiags});
const got = Buffer.from(out.bytes);
const exact = got.length === src.length && got.equals(src);
let firstDiff = -1, diffs = 0;
for(let i = 0; i < Math.max(got.length, src.length); i++)
  if(got[i] !== src[i]){ diffs++; if(firstDiff < 0) firstDiff = i; }
const silent = !exact && out.detected === 0;

if(has("quiet")){
  console.log(`${path.basename(file).padEnd(16)} ${alphabet.padEnd(6)} ${model.padEnd(7)}`
    + ` inj ${String(injected).padStart(5)}  fixed ${String(out.fixed).padStart(5)}`
    + `  ${exact ? "EXACT" : (silent ? "SILENTLY WRONG" : "detected, unrepaired")}`);
  process.exit(exact ? 0 : 1);
}

console.log(`${file}  ${src.length} bytes  ->  ${alphabet}, N=${N}, diagonals ${useDiags ? "on" : "OFF"}`);
console.log(`  format     ${s.squares} squares, data ${s.dataBytes}B + parity ${s.parityBytes}B`
  + ` = ${s.totalBytes}B  (${s.ratio.toFixed(2)}x source, parity overhead ${(100 * s.parityOverhead).toFixed(1)}%)`);
console.log(`  model      ${model}${model === "uniform" ? ` rate=${rate}` : ` hits=${hits}`
  + (model === "burst" ? ` len=${burstLen}` : "")}`);
console.log(`  injected   ${injected} cell errors across ${p.squares.length} squares`);
console.log(`  squares    ${out.clean} clean, ${out.corrected} corrected, ${out.detected} detected-unrepaired`);
console.log(`  cells      ${out.fixed} repaired`);
console.log(`  result     ${exact ? "EXACT round-trip" : `${diffs} bytes differ, first at ${firstDiff}`}`);
console.log(`  SILENTLY WRONG: ${silent ? "YES -- data lost without warning" : "no"}`);
process.exit(exact || !silent ? 0 : 1);
