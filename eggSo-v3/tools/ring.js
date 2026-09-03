/* node eggSo-v3/tools/ring.js [file...] [--json] -- the ring.
 *
 * The codegg line has a tournament with a rule. The eggSo line has had
 * synthetic channels on random squares, and its two shipped candidates have
 * never been asked to hand back a real file. This is the ring, and the rule
 * is codegg-v12/tools/standings.js's, unchanged and enforced mechanically:
 *
 *   a codec that returns WRONG data or NO data after an injury FORFEITS.
 *   Among the survivors, the smallest artifact wins.
 *
 * Four injuries per artifact, the same for everyone:
 *   bitflip   one bit flipped in the middle of the payload   (blind)
 *   byteflip  one byte replaced in the middle of the payload (blind)
 *   scratch   4096 contiguous bytes overwritten, address given to the decoder
 *   trunc     4096 bytes REMOVED from the end
 *
 * `bitflip` is the injury this lineage was built for and the codegg ring
 * never had. `byteflip` is the one real storage delivers and no round before
 * this one ever ran. Both are here.
 *
 * The rows are eggSo-v3 configurations, and one of them is the lineage
 * itself: A=2, N=32 is eggSo-v0's code exactly -- same regions, same primes
 * 2053/2063, same three residues and confirm -- decoded by v0's amended
 * decoder. It is labelled so. eggSo-v1(a)'s extra residue and eggSo-v1(b)'s
 * mirror are not in the ring: they change what a square carries, not what a
 * file survives, and the injuries here are answered by the block size and the
 * radix. That is said plainly in the README rather than hidden by an absence.
 */
const fs = require("fs"), path = require("path");
const X = require(path.join(__dirname, "..", "eggso3.js"));
const root = path.join(__dirname, "..", "..");

const argv = process.argv.slice(2);
const files = argv.filter(a => !a.startsWith("--"));
const FILES = files.length ? files : [
  "spec.md", "stalk.js", "og.png", "wubbadub.html",
  "codegg-v10/corpus/program.exe", "codegg-v10/corpus-real/notepad.exe", "codegg-v10/corpus/archive.zst",
].map(f => path.join(root, f)).filter(fs.existsSync);

const ROWS = [
  {name: "bit/128B",      A: 2,   N: 32, sigma: false, note: "eggSo-v0's code exactly (p=2053, q=2063)"},
  {name: "bit/128B+o",    A: 2,   N: 32, sigma: true},
  {name: "nib/512B",      A: 16,  N: 32, sigma: false},
  {name: "nib/512B+o",    A: 16,  N: 32, sigma: true},
  {name: "byte/256B",     A: 256, N: 16, sigma: false},
  {name: "byte/1KB",      A: 256, N: 32, sigma: false},
  {name: "byte/1KB+o",    A: 256, N: 32, sigma: true},
];
const INJURIES = ["bitflip", "byteflip", "scratch", "trunc"];
const codes = new Map();
const codeFor = r => { const k = `${r.A}:${r.N}`; if(!codes.has(k)) codes.set(k, X.makeCode(r.N, r.A)); return codes.get(k); };

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }

/* injure the artifact. Payload starts after the header and the checks, so the
   middle of the payload is where every blind injury lands -- the same place
   codegg-v12 puts its flip and its scratch. */
function injure(art, kind, payStart){
  const b = Buffer.from(art), g = mul32(0xACE);
  const mid = payStart + Math.floor((b.length - payStart) / 2);
  if(kind === "bitflip"){ b[mid] ^= 0x40; return {art: new Uint8Array(b)}; }
  if(kind === "byteflip"){ b[mid] = (b[mid] + 1 + (g() & 0x7f)) & 0xff; return {art: new Uint8Array(b)}; }
  if(kind === "trunc") return {art: new Uint8Array(b.subarray(0, Math.max(payStart, b.length - 4096)))};
  const at = Math.max(payStart, mid - 2048), len = Math.min(4096, b.length - at);
  for(let i = at; i < at + len; i++) b[i] = g() & 0xff;
  return {art: new Uint8Array(b), wound: {at, len}};
}

const pad = (s, n) => String(s).padEnd(n), rp = (s, n) => String(s).padStart(n);
const results = {};
console.log("THE RING -- rule: wrong-or-no data after any injury forfeits; among survivors the smallest artifact wins");
console.log("injuries: 1 bit flipped (blind) / 1 byte replaced (blind) / 4 KB scratch (addressed) / 4 KB truncated\n");

let podium = {};
for(const file of FILES){
  const src = fs.readFileSync(file), short = path.basename(file);
  console.log(`${short}  ${src.length} B`);
  console.log("  " + pad("row", 14) + pad("size", 12) + INJURIES.map(i => rp(i, 12)).join("") + "   verdict");
  console.log("  " + "-".repeat(14 + 12 + 12 * INJURIES.length + 12));
  const table = {};
  for(const r of ROWS){
    const code = codeFor(r);
    const enc = X.encode(src, {N: r.N, A: r.A, code, sigma: r.sigma});
    const payStart = X.HEAD + enc.meta.squares * enc.meta.cb;
    const cells = [];
    let survived = true, lied = false;
    for(const kind of INJURIES){
      const {art, wound} = injure(enc.artifact, kind, payStart);
      let verdict;
      try {
        const out = X.decode(art, {code, wound});
        if(!out.ok) verdict = "dead";
        else if(Buffer.from(out.bytes).equals(src)) verdict = "EXACT";
        else verdict = out.detected ? "detected" : "WRONG";
      } catch(e){ verdict = "dead"; }
      if(verdict !== "EXACT") survived = false;
      if(verdict === "WRONG") lied = true;
      cells.push(verdict);
    }
    const size = enc.artifact.length;
    table[r.name] = {size, ratio: size / src.length, cells: Object.fromEntries(INJURIES.map((k, i) => [k, cells[i]])), survived, lied};
    console.log("  " + pad(r.name, 14) + pad(`${size} (${(100 * size / src.length).toFixed(2)}%)`, 12)
      + cells.map(c => rp(c, 12)).join("") + "   " + (survived ? "SURVIVES" : lied ? "FORFEIT (LIED)" : "forfeit"));
  }
  const alive = ROWS.filter(r => table[r.name].survived).sort((a, b) => table[a.name].size - table[b.name].size);
  const winner = alive.length ? alive[0].name : "none";
  podium[winner] = (podium[winner] || 0) + 1;
  console.log(`  winner: ${winner}${alive.length ? ` (${table[winner].size} B, ${(100 * table[winner].ratio).toFixed(2)}%)` : " -- every row forfeited at least one injury"}\n`);
  results[short] = {bytes: src.length, table, winner};
}
console.log("podium: " + Object.entries(podium).sort((a, b) => b[1] - a[1]).map(([n, c]) => `${n} x${c}`).join(", "));

/* THE CAPACITY CURVE, added after the 4 KB row came back all-forfeit.
   "Nobody survives 4096 bytes" is a true sentence and a useless one. The
   number that says what sigma is worth is the LARGEST contiguous scratch a
   row survives exactly, found by bisection. Filed before running: without
   sigma a wound lives inside one or two blocks and one equation per region
   solves one byte per region, so the capacity should be about 3 bytes and
   independent of the file; with sigma the same 3-per-block is available in
   every block the scatter reaches, so the capacity should rise with the file
   and land in the hundreds or low thousands. */
function survives(src, r, code, len){
  const enc = X.encode(src, {N: r.N, A: r.A, code, sigma: r.sigma});
  const payStart = X.HEAD + enc.meta.squares * enc.meta.cb;
  const b = Buffer.from(enc.artifact), g = mul32(0xACE);
  const at = Math.max(payStart, payStart + Math.floor((b.length - payStart) / 2) - Math.floor(len / 2));
  const n = Math.min(len, b.length - at);
  for(let i = at; i < at + n; i++) b[i] = g() & 0xff;
  try {
    const out = X.decode(new Uint8Array(b), {code, wound: {at, len: n}});
    return out.ok && Buffer.from(out.bytes).equals(src);
  } catch(e){ return false; }
}
console.log("\nCAPACITY -- the largest contiguous scratch each row survives EXACTLY (bytes), by bisection");
console.log("  " + pad("row", 14) + FILES.map(f => rp(path.basename(f).slice(0, 11), 13)).join(""));
console.log("  " + "-".repeat(14 + 13 * FILES.length));
const capacity = {};
for(const r of ROWS){
  const code = codeFor(r), row = [];
  for(const file of FILES){
    const src = fs.readFileSync(file);
    let lo = 0, hi = 8192;
    if(!survives(src, r, code, 1)){ row.push(0); capacity[`${r.name}|${path.basename(file)}`] = 0; continue; }
    while(survives(src, r, code, hi) && hi < src.length){ lo = hi; hi *= 2; }
    while(lo + 1 < hi){ const mid = (lo + hi) >> 1; if(survives(src, r, code, mid)) lo = mid; else hi = mid; }
    row.push(lo); capacity[`${r.name}|${path.basename(file)}`] = lo;
  }
  console.log("  " + pad(r.name, 14) + row.map(v => rp(v, 13)).join(""));
}
/* per injury, who is still standing anywhere */
console.log("\nper injury, rows that were EXACT on every file:");
for(const kind of INJURIES){
  const ok = ROWS.filter(r => Object.values(results).every(f => f.table[r.name].cells[kind] === "EXACT"));
  console.log(`  ${pad(kind, 10)} ${ok.length ? ok.map(r => r.name).join(", ") : "nobody"}`);
}
console.log("\nrows that ever returned WRONG data (the failure that matters):");
for(const r of ROWS){
  const bad = Object.entries(results).filter(([, f]) => f.table[r.name].lied).map(([n]) => n);
  if(bad.length) console.log(`  ${pad(r.name, 14)} ${bad.join(", ")}`);
}
if(argv.indexOf("--json") >= 0) fs.writeFileSync(path.join(__dirname, "..", "measured-ring.json"), JSON.stringify({rows: ROWS, injuries: INJURIES, results, podium, capacity}, null, 1));
