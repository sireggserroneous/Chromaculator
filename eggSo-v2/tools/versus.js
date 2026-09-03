/* node eggSo-v2/tools/versus.js <file> [--N 32] [--trials 400] [--json]
 *
 * Head to head: both controls -- codegg-v1 and eggSo-v0 on bit squares, and
 * codegg-v1 in its own trit mode on the canonical squares -- against the two
 * arms of eggSo-v2, plus v2(b) with bit squares declared trit (cap 12), the
 * configuration PREDICTIONS.md filed the burst prediction for. Same file,
 * same square, same damage positions for every column; a "flip" is XOR on a
 * bit square and lit<->green on a canonical one, so each alphabet takes the
 * error it can actually suffer.
 *
 * Each cell reads   ok / det / WRONG / direct   (see eggSo-v1/tools/versus.js)
 * "--" means the column's alphabet cannot hold the channel's square.
 */
const fs = require("fs"), path = require("path"), vm = require("vm");
const G = require(path.join(__dirname, "..", "..", "codegg-v1", "codegg.js"));
const E = require(path.join(__dirname, "..", "..", "eggSo-v0", "eggso.js"));
const W = require(path.join(__dirname, "..", "eggso2.js"));

const argv = process.argv.slice(2);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const file = argv.find(a => !a.startsWith("--") && !/^\d+$/.test(a));
if(!file){ console.error("usage: versus.js <file> [--N 32] [--trials 400] [--json]"); process.exit(1); }
const N = parseInt(flag("N", "32"), 10), L = N * N, T = parseInt(flag("trials", "400"), 10);
const src = fs.readFileSync(file);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
let g = mul32(20260902);
const pick = n => g() % n;
const same = (a, b) => Buffer.from(a).equals(Buffer.from(b));
const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname, "..", "..", "stalk.js"), "utf8"), site);
const pushLeft = cells => Int8Array.from(vm.runInContext("pushLeft", site)(Array.from(cells)));

const v1 = G.makeCode(N), v0 = E.makeCode(N), c2 = W.makeCode(N);
const bitSquares = G.toCells(src, L);
const encA = W.encodeA(src, {N, code: c2}), encB = W.encodeB(src, {N, code: c2});
const GEO = v0;

const COLS = [
  {name: "codegg-v1", alphabet: "bit", squares: bitSquares, checks: c => [G.residue(c, v1.p), G.residue(c, v1.q)],
   repair: (h, k, er) => G.repairSquare(h, k, v1, er ? {erased: er} : undefined), isDirect: r => r.note === "single" || r.note === "erasures",
   verify: (c, k) => G.verify(c, k, v1), overhead: G.sizes({N, L, p: v1.p, q: v1.q, bytes: src.length}).overhead},
  {name: "codegg-v1·trit", alphabet: "trit", squares: encB.squares, checks: c => [G.residue(c, v1.p), G.residue(c, v1.q)],
   repair: (h, k, er) => G.repairSquare(h, k, v1, {alphabet: "trit", erased: er}), isDirect: r => r.note === "single" || r.note === "erasures",
   verify: (c, k) => G.verify(c, k, v1), overhead: G.sizes({N, L, p: v1.p, q: v1.q, bytes: src.length}).overhead},
  {name: "eggSo-v0", alphabet: "bit", squares: bitSquares, checks: c => E.checksFor(c, v0),
   repair: (h, k, er) => E.repairSquare(h, k, v0, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0,
   verify: (c, k) => E.verify(c, k, v0), overhead: E.sizes({N, L, p: v0.p, q: v0.q, confirm: true, bytes: src.length}).overhead},
  {name: "v2(a)", alphabet: "trit", squares: encA.squares, checks: (c, s) => ({flag: encA.flags[s], ext: encA.external[s]}),
   repair: (h, k, er) => W.repairA(h, k.flag, k.ext, c2, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0,
   verify: () => null, overhead: W.sizesA(encA.meta).overhead, inBand: encA.meta.inBand},
  {name: "v2(b)", alphabet: "trit", squares: encB.squares, checks: c => E.checksFor(c, c2),
   repair: (h, k, er) => W.repairSquare(h, k, c2, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0,
   verify: () => null, overhead: W.sizesB(encB.meta).overhead},
  {name: "v2(b)·bits-as-trits", alphabet: "bit", squares: bitSquares, checks: c => E.checksFor(c, c2), bat: true,
   repair: (h, k, er) => W.repairSquare(h, k, c2, {canonical: false, cap: 12, erased: er}), isDirect: r => (r.searched || 0) === 0,
   verify: (c, k) => E.verify(c, k, c2), overhead: W.sizesB(encB.meta).overhead},
];
const pad = (s, n) => String(s).padEnd(n), rpad = (s, n) => String(s).padStart(n);
const results = {};
const hit = (h, i, alphabet) => { if(alphabet === "bit") h[i] ^= 1; else h[i] = h[i] === 0 ? (pick(2) ? 1 : -1) : 0; };

function channel(label, damage, note, needs){
  g = mul32(20260902 + label.length);
  const tallies = COLS.map(() => ({corrected: 0, detected: 0, wrong: 0, direct: 0, na: false}));
  for(let t = 0; t < T; t++){
    const at = pick(1 << 30), seedDmg = g();
    COLS.forEach((col, i) => {
      if(needs && needs !== col.alphabet){ tallies[i].na = true; return; }
      const s = at % col.squares.length, stored = col.squares[s], chk = col.checks(stored, s), hurt = stored.slice();
      g = mul32(seedDmg);
      const erased = damage(hurt, GEO, col.alphabet, stored);
      const r = col.repair(hurt, chk, erased);
      const x = tallies[i];
      if(r.status === "corrected"){ if(same(hurt, stored)){ x.corrected++; if(col.isDirect(r)) x.direct++; } else x.wrong++; }
      else x.detected++;
    });
    g = mul32(seedDmg ^ 0x5bd1e995);
  }
  results[label] = Object.fromEntries(COLS.map((c, i) => [c.name, tallies[i]]));
  const fmt = x => x.na ? pad("--", 15) : `${rpad(x.corrected, 3)}/${rpad(x.detected, 3)}/${rpad(x.wrong, 2)}/${rpad(x.direct, 3)}`;
  console.log(`  ${pad(label, 36)} ${tallies.map(fmt).join("   ")}${note ? "\n  " + " ".repeat(36) + note : ""}`);
}

console.log(`${path.basename(file)}, ${src.length} bytes, N=${N}, ${T} trials per channel; v2(a) has ${encA.meta.inBand} of ${encA.squares.length} squares in-band`);
console.log(`  overhead: ${COLS.map(c => `${c.name} ${(c.overhead * 100).toFixed(2)}%`).join(" · ")}\n`);
console.log(`  ${pad("channel", 36)} ${COLS.map(c => pad(c.name, 15)).join("   ")}`);
console.log(`  ${pad("", 36)} ${COLS.map(() => "ok /det/ W/dir ").join("   ")}`);
console.log("  " + "-".repeat(36 + 18 * COLS.length));

channel("1 cell hit (d = +-1)", (h, cd, a) => { hit(h, pick(L), a); });
channel("1 sign flip (d = +-2), canonical", (h, cd, a, st) => { const lit = []; for(let i = 0; i < L; i++) if(st[i]) lit.push(i); const i = lit[pick(lit.length)]; h[i] = -h[i]; },
  "the round's new channel: only trit columns can hold it", "trit");
channel("2 cells hit, anywhere", (h, cd, a) => { let x = pick(L), y = pick(L); while(y === x) y = pick(L); hit(h, x, a); hit(h, y, a); });
channel("2 cells, DIFFERENT regions", (h, cd, a) => { let x = pick(L), y = pick(L); while(cd.region[y] === cd.region[x]) y = pick(L); hit(h, x, a); hit(h, y, a); });
channel("2 cells, SAME region", (h, cd, a) => { const k = pick(3), m = cd.members[k]; let x = m[pick(m.length)], y = m[pick(m.length)]; while(y === x) y = m[pick(m.length)]; hit(h, x, a); hit(h, y, a); },
  "v2 confirms per candidate (codegg.js:204-206), so its in-region search does not refuse like v0's");
channel("3 cells, one per region", (h, cd, a) => { for(let k = 0; k < 3; k++){ const m = cd.members[k]; hit(h, m[pick(m.length)], a); } });
channel("12-cell row burst, flagged", (h, cd, a) => { const r = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ h[r * N + c0 + j] = a === "bit" ? -1 : 0; F.push(r * N + c0 + j); } return F; },
  "bit columns read a -1 sentinel; trit columns take the explicit list. bits-as-trits: the alias 2^k - 2^(k-1) = 2^(k-1)");
channel("12-cell row burst, UNFLAGGED, in-region", (h, cd, a) => { for(;;){ const r = pick(N), c0 = pick(N - 12), regs = new Set(); for(let j = 0; j < 12; j++) regs.add(cd.region[r * N + c0 + j]); if(regs.size === 1 && !regs.has(E.FOLD)){ for(let j = 0; j < 12; j++) hit(h, r * N + c0 + j, a); return; } } });
channel("the Fold filled: 32 cells, unflagged", (h, cd, a) => { for(const i of cd.members[E.FOLD]) hit(h, i, a); });
{
  g = mul32(20260902);
  const Tp = Math.min(T, 200), holds = COLS.map(() => 0);
  for(let t = 0; t < Tp; t++){ const at = pick(1 << 30); COLS.forEach((col, i) => { const s = at % col.squares.length, st = col.squares[s], v = col.verify(pushLeft(st), col.checks(st, s)); if(v === null) holds[i] = "vacuous"; else if(v) holds[i]++; }); }
  results["push holds"] = Object.fromEntries(COLS.map((c, i) => [c.name, holds[i]]));
  console.log(`  ${pad(`push: checks still hold, of ${Tp}`, 36)} ${holds.map(h => pad(typeof h === "number" ? `${h}/${Tp}` : h, 15)).join("   ")}`);
  console.log(`  ${" ".repeat(36)} vacuous = the stored square is already push's fixpoint`);
}
console.log(`\n  direct = corrected by a syndrome naming its own cell, no search.`);
if(argv.indexOf("--json") >= 0) fs.writeFileSync(path.join(__dirname, "..", "measured-versus.json"), JSON.stringify({file: path.basename(file), bytes: src.length, N, T, inBand: encA.meta.inBand, results}, null, 1));
