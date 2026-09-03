/* node eggSo-v2/tools/greens.js -- S1: the trailing-green histogram, before any codec.
 *
 * pushLeft (stalk.js, run in a box) over 10,000 random squares and every
 * square of the corpora PREDICTIONS.md names. Counts how many greens trail the
 * last lit cell after push, and how many squares have >= 28 -- the number of
 * trits four residues need. This file files v2(a)'s verdict: the free lunch
 * is whatever fraction of squares carry their own checks in the padding. */
const fs = require("fs"), vm = require("vm"), path = require("path");
const G = require(path.join(__dirname, "..", "..", "codegg-v1", "codegg.js"));
const root = path.join(__dirname, "..", "..");
const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(root, "stalk.js"), "utf8"), site);
const pushLeft = vm.runInContext("pushLeft", site);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(20260902);
const L = 1024, NEED = 28;

const tail = cells => { const p = pushLeft(Array.from(cells)); let k = 0; while(k < p.length && p[p.length - 1 - k] === 0) k++; return k; };
const v2 = cells => { let k = 0; while(k < cells.length && cells[cells.length - 1 - k] === 0) k++; return k; };   // trailing zero bits of V

function histogram(name, squares){
  const hist = new Map(); let sum = 0, inBand = 0, mismatch = 0;
  const tails = [];
  for(const sq of squares){
    const k = tail(sq); tails.push(k); sum += k;
    if(k !== v2(sq)) mismatch++;                       // the closed form: tail = 2-adic valuation of V
    if(k >= NEED) inBand++;
    hist.set(k, (hist.get(k) || 0) + 1);
  }
  tails.sort((a, b) => a - b);
  const q = f => tails[Math.min(tails.length - 1, Math.floor(f * tails.length))];
  const out = {name, squares: squares.length, mean: sum / squares.length, median: q(0.5), p90: q(0.9), max: tails[tails.length - 1],
               inBand, inBandRate: inBand / squares.length, fallbackRate: 1 - inBand / squares.length, closedFormMismatch: mismatch,
               hist: Object.fromEntries([...hist.entries()].sort((a, b) => a[0] - b[0]))};
  console.log(`  ${name.padEnd(22)} ${String(squares.length).padStart(6)} squares  mean ${out.mean.toFixed(3)}  median ${out.median}  p90 ${out.p90}  max ${String(out.max).padStart(4)}   >= ${NEED} greens: ${String(inBand).padStart(4)} (${(100 * out.inBandRate).toFixed(2)}%)  fall back ${(100 * out.fallbackRate).toFixed(2)}%${mismatch ? `  CLOSED-FORM MISMATCH ${mismatch}` : ""}`);
  return out;
}

const results = {};
console.log(`S1 -- trailing greens after pushLeft, per 1024-cell square (in-band needs >= ${NEED})`);
{
  const squares = []; for(let t = 0; t < 10000; t++) squares.push(Int8Array.from({length: L}, () => g() & 1));
  results.random = histogram("random bits", squares);
  /* the geometric law: P(k) = 2^-(k+1) */
  const fit = [];
  for(let k = 0; k <= 6; k++){ const obs = results.random.hist[k] || 0, exp = 10000 * Math.pow(2, -(k + 1)); fit.push({k, observed: obs, expected: exp, z: (obs - exp) / Math.sqrt(exp * (1 - Math.pow(2, -(k + 1))))}); }
  results.random.geometricFit = fit;
  console.log(`    geometric law 2^-(k+1): ` + fit.map(f => `k=${f.k} ${f.observed}/${f.expected.toFixed(0)} (z ${f.z.toFixed(1)})`).join("  "));
}
const CORPORA = [
  ["spec.md", "spec.md"], ["stalk.js", "stalk.js"], ["og.png", "og.png"],
  ["program.exe", "codegg-v10/corpus/program.exe"], ["notepad.exe", "codegg-v10/corpus-real/notepad.exe"], ["archive.zst", "codegg-v10/corpus/archive.zst"],
];
for(const [name, rel] of CORPORA){
  const file = path.join(root, rel);
  if(!fs.existsSync(file)){ console.log(`  ${name.padEnd(22)} not present, skipped`); continue; }
  results[name] = histogram(name, G.toCells(fs.readFileSync(file), L));
}
fs.writeFileSync(path.join(__dirname, "..", "measured-greens.json"), JSON.stringify(results, null, 1));
console.log(`\n  written measured-greens.json`);
