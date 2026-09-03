/* node eggSo-v1/tools/versus.js <file> [--N 32] [--trials 400]
 *
 * Head to head: both controls -- codegg-v1 (one residue of the whole square)
 * and eggSo-v0 (one residue per fold region) -- against the three arms of
 * eggSo-v1. Same file, same squares, same damage, cell for cell: every codec
 * but v1(b) lays bytes into the square with codegg-v1's own toCells; v1(b)
 * holds 66 bytes per square (Inner + Fold) with Outer a mirror, so its
 * squares carry the same file in more squares. Damage is applied by
 * position, and region membership is positional, so "same region" means the
 * same thing to every column. v1(c) stores the permuted square and is damaged
 * where it is stored.
 *
 * Each cell reads   ok / det / WRONG / direct
 *   corrected      restored exactly
 *   detected       refused to guess; the data is marked, not lost
 *   MISCORRECTED   "repaired" into the wrong bytes -- the failure that matters
 *   direct         corrected with NO search: a syndrome named its own cell
 *                  (for v1(a) that includes the pair table; for v1(b) the
 *                  side named by the one residue that moved)
 */
const fs = require("fs"), path = require("path"), vm = require("vm");
const G = require(path.join(__dirname, "..", "..", "codegg-v1", "codegg.js"));
const E = require(path.join(__dirname, "..", "..", "eggSo-v0", "eggso.js"));
const V = require(path.join(__dirname, "..", "eggso1.js"));

const argv = process.argv.slice(2);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const file = argv.find(a => !a.startsWith("--") && !/^\d+$/.test(a));
if(!file){ console.error("usage: versus.js <file> [--N 32] [--trials 400]"); process.exit(1); }

const N = parseInt(flag("N", "32"), 10), L = N * N, T = parseInt(flag("trials", "400"), 10);
const src = fs.readFileSync(file);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
let g = mul32(20260902);
const pick = n => g() % n;
const same = (a, b) => Buffer.from(a).equals(Buffer.from(b));
const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname, "..", "..", "stalk.js"), "utf8"), site);
const pushLeft = cells => Int8Array.from(vm.runInContext(`pushLeft(${JSON.stringify(Array.from(cells))})`, site));

const v1 = G.makeCode(N), v0 = E.makeCode(N), A = V.makeCodeA(N), B = V.makeCodeB(N), C = V.makeCodeC(N);
const rowMajor = G.toCells(src, L), mirrored = V.toCells528(src, B);
const GEO = v0;                                          // region / members tables, identical for every arm

/* every column: where its squares come from, how it checks, how it repairs,
   how it verifies (for push), and what it costs */
const COLS = [
  {name: "codegg-v1", code: v1, squares: rowMajor, phys: c => c,
   checks: c => [G.residue(c, v1.p), G.residue(c, v1.q)],
   repair: (h, k, er) => G.repairSquare(h, k, v1, er ? {erased: er} : undefined),
   isDirect: r => r.note === "single" || r.note === "erasures",
   verify: (c, k) => G.verify(c, k, v1),
   overhead: G.sizes({N, L, p: v1.p, q: v1.q, bytes: src.length}).overhead},
  {name: "eggSo-v0", code: v0, squares: rowMajor, phys: c => c,
   checks: c => E.checksFor(c, v0),
   repair: (h, k, er) => E.repairSquare(h, k, v0, er ? {erased: er} : undefined),
   isDirect: r => (r.searched || 0) === 0,
   verify: (c, k) => E.verify(c, k, v0),
   overhead: E.sizes({N, L, p: v0.p, q: v0.q, confirm: true, bytes: src.length}).overhead},
  {name: "v1(a)", code: A, squares: rowMajor, phys: c => c,
   checks: c => V.checksForA(c, A),
   repair: (h, k, er) => V.repairA(h, k, A, er ? {erased: er} : undefined),
   isDirect: r => (r.searched || 0) === 0,
   verify: (c, k) => V.verifyA(c, k, A),
   overhead: V.sizesA({N, L, p: A.p, q: A.q, confirm: true, bytes: src.length}).overhead},
  {name: "v1(b)", code: B, squares: mirrored, phys: c => c,
   checks: c => E.checksFor(c, B),
   repair: (h, k, er) => V.repairB(h, k, B, er ? {erased: er} : undefined),
   isDirect: r => (r.searched || 0) === 0,
   verify: (c, k) => E.verify(c, k, B),
   overhead: V.sizesB({N, L, K: B.K, p: B.p, q: B.q, confirm: true, bytes: src.length}).overhead,
   share: V.sizesB({N, L, K: B.K, p: B.p, q: B.q, confirm: true, bytes: src.length}).share},
  {name: "v1(c)", code: C, squares: rowMajor, phys: c => V.permuteC(c, C),
   checks: c => E.checksFor(V.permuteC(c, C), C),          // v0's checks on the logical square
   repair: (h, k, er) => V.repairC(h, k, C, er ? {erased: er} : undefined),
   isDirect: r => (r.searched || 0) === 0,
   verify: (c, k) => E.verify(V.permuteC(c, C), k, C),
   overhead: E.sizes({N, L, p: C.p, q: C.q, confirm: true, bytes: src.length}).overhead},
];

const pad = (s, n) => String(s).padEnd(n), rpad = (s, n) => String(s).padStart(n);
const results = {};

function channel(label, damage, note){
  g = mul32(20260902 + label.length);
  const tallies = COLS.map(() => ({corrected: 0, detected: 0, wrong: 0, direct: 0}));
  for(let t = 0; t < T; t++){
    const at = pick(1 << 30);                          // same square choice, scaled per column
    const seedDmg = g();
    COLS.forEach((col, i) => {
      const stored = col.phys(col.squares[at % col.squares.length]);
      const chk = col.checks(stored), hurt = stored.slice();
      g = mul32(seedDmg);                              // identical damage positions for every column
      const erased = damage(hurt, GEO);                // regions are positional: one geometry for every column
      const r = col.repair(hurt, chk, erased);
      const x = tallies[i];
      if(r.status === "corrected"){ if(same(hurt, stored)){ x.corrected++; if(col.isDirect(r)) x.direct++; } else x.wrong++; }
      else x.detected++;
    });
    g = mul32(seedDmg ^ 0x5bd1e995);                   // move on
  }
  results[label] = Object.fromEntries(COLS.map((c, i) => [c.name, tallies[i]]));
  const fmt = x => `${rpad(x.corrected, 3)}/${rpad(x.detected, 3)}/${rpad(x.wrong, 2)}/${rpad(x.direct, 3)}`;
  console.log(`  ${pad(label, 34)} ${tallies.map(fmt).join("   ")}${note ? "\n  " + " ".repeat(34) + note : ""}`);
}

console.log(`${path.basename(file)}, ${src.length} bytes, N=${N}, ${T} trials per channel`);
console.log(`  overhead: ${COLS.map(c => `${c.name} ${(c.overhead * 100).toFixed(2)}%${c.share !== undefined ? ` (${(c.share * 100).toFixed(1)}% share)` : ""}`).join(" · ")}\n`);
console.log(`  ${pad("channel", 34)} ${COLS.map(c => pad(c.name, 15)).join("   ")}`);
console.log(`  ${pad("", 34)} ${COLS.map(() => "ok /det/ W/dir ").join("   ")}`);
console.log("  " + "-".repeat(34 + 18 * COLS.length));

channel("1 cell flipped", h => { h[pick(L)] ^= 1; });
channel("2 cells flipped, anywhere", h => { let a = pick(L), b = pick(L); while(b === a) b = pick(L); h[a] ^= 1; h[b] ^= 1; },
  "v1(a)'s claim: the pairs v0 could only detect, corrected by table with no search");
channel("2 cells, DIFFERENT regions", (h, cd) => { let a = pick(L), b = pick(L); while(cd.region[b] === cd.region[a]) b = pick(L); h[a] ^= 1; h[b] ^= 1; });
channel("2 cells, SAME region", (h, cd) => { const k = pick(3), m = cd.members[k]; let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)]; h[a] ^= 1; h[b] ^= 1; },
  "v0: one residue, two unknowns. v1(a): two syndromes, one lookup.");
channel("3 cells, one per region", (h, cd) => { for(let k = 0; k < 3; k++){ const m = cd.members[k]; h[m[pick(m.length)]] ^= 1; } });
channel("12-cell row burst, flagged", h => { const r = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ h[r * N + c0 + j] = -1; F.push(r * N + c0 + j); } return F; });
channel("12-cell row burst, UNFLAGGED, in-region", (h, cd) => {
  for(;;){
    const r = pick(N), c0 = pick(N - 12), regs = new Set();
    for(let j = 0; j < 12; j++) regs.add(cd.region[r * N + c0 + j]);
    if(regs.size === 1 && !regs.has(E.FOLD)){ for(let j = 0; j < 12; j++) h[r * N + c0 + j] ^= 1; return; }
  }
}, "twelve unknowns in one residue for everyone but the mirror, which reads them off the partner");
channel("the Fold filled: 32 cells, unflagged", (h, cd) => { for(const i of cd.members[E.FOLD]) h[i] ^= 1; },
  "the predicted weak spot -- a burst that exactly fills the 3% region");

/* push: does each column's check survive the site's canonicalising move? */
{
  g = mul32(20260902);
  const holds = COLS.map(() => 0), Tp = Math.min(T, 200);
  for(let t = 0; t < Tp; t++){
    const at = pick(1 << 30);
    COLS.forEach((col, i) => {
      const stored = col.phys(col.squares[at % col.squares.length]), chk = col.checks(stored);
      if(col.verify(pushLeft(stored), chk)) holds[i]++;
    });
  }
  results["push holds"] = Object.fromEntries(COLS.map((c, i) => [c.name, {holds: holds[i], of: Tp}]));
  console.log(`  ${pad(`push: checks still hold, of ${Tp}`, 34)} ${holds.map(h => pad(`${h}/${Tp}`, 15)).join("   ")}`);
}
console.log(`\n  direct = corrected by a syndrome naming its own cell, no search. For codegg-v1 that is singles and erasures only.`);
if(argv.indexOf("--json") >= 0) fs.writeFileSync(path.join(__dirname, "..", "measured-versus.json"), JSON.stringify({file: path.basename(file), bytes: src.length, N, T, results}, null, 1));
