/* node eggSo-v2/tools/standings.js [file] [--N 32] [--trials 400] [--json]
 *
 * Who the house keeps. Every arm of the fold-native lineage -- eggSo-v0, the
 * three arms of eggSo-v1, the two of eggSo-v2 (plus v2(b) with bit squares
 * declared trit, the configuration the burst prediction was filed for) --
 * against the control codegg-v1, on the same file, the same square choice
 * and the same damage positions, cell for cell. Modelled on
 * codegg-v12/tools/standings.js: fixed injuries, a shared seed, and WRONG
 * stays loud beside its number.
 *
 * Ten channels: v0's eight plus the two this round adds -- the 12-cell
 * UNFLAGGED in-region row burst (v1(b)'s one row) and one sign flip on a
 * canonical square (the trit alphabet's d = +-2). Then cost in TWO
 * conventions, both printed, because v1(b) reads 103% in one and 51% in the
 * other:
 *   per data bit    redundant bits / data bits          (v0's convention)
 *   share           redundant bits / all bits stored    (cells + checks)
 *
 * Arms are plugins: {name, alphabet, squares, checks, repair, isDirect, verify,
 * overhead, share}. The plan's rule for the table, verbatim: every cell that
 * moves from PREDICTIONS.md is a recorded miss. */
const fs = require("fs"), path = require("path"), vm = require("vm");
const root = path.join(__dirname, "..", "..");
const G = require(path.join(root, "codegg-v1", "codegg.js"));
const E = require(path.join(root, "eggSo-v0", "eggso.js"));
const V = require(path.join(root, "eggSo-v1", "eggso1.js"));
const W = require(path.join(root, "eggSo-v2", "eggso2.js"));

const argv = process.argv.slice(2);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const file = argv.find(a => !a.startsWith("--") && !/^\d+$/.test(a)) || path.join(root, "spec.md");
const N = parseInt(flag("N", "32"), 10), L = N * N, T = parseInt(flag("trials", "400"), 10);
const src = fs.readFileSync(file);

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
let g = mul32(20260902);
const pick = n => g() % n;
const same = (a, b) => Buffer.from(a).equals(Buffer.from(b));
const site = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(root, "stalk.js"), "utf8"), site);
const pushLeft = cells => Int8Array.from(vm.runInContext("pushLeft", site)(Array.from(cells)));

const v1 = G.makeCode(N), v0 = E.makeCode(N), A = V.makeCodeA(N), B = V.makeCodeB(N), C = V.makeCodeC(N), c2 = W.makeCode(N);
const bitSquares = G.toCells(src, L), mirrored = V.toCells528(src, B);
const permuted = bitSquares.map(c => V.permuteC(c, C));
const encA = W.encodeA(src, {N, code: c2}), encB = W.encodeB(src, {N, code: c2});
const GEO = v0;
const bytes = src.length, dataBits = bytes * 8;
const share = (redundant, cells) => redundant / (cells + redundant);
const nsq = bitSquares.length;

const COLS = [
  {name: "codegg-v1", alphabet: "bit", squares: bitSquares, tritSquares: encB.squares,
   checks: c => [G.residue(c, v1.p), G.residue(c, v1.q)],
   repair: (h, k, er, trit) => G.repairSquare(h, k, v1, {alphabet: trit ? "trit" : undefined, erased: er}),
   isDirect: r => r.note === "single" || r.note === "erasures", verify: (c, k) => G.verify(c, k, v1),
   overhead: G.sizes({N, L, p: v1.p, q: v1.q, bytes}).overhead, share: share(nsq * 24, nsq * L)},
  {name: "eggSo-v0", alphabet: "bit", squares: bitSquares, checks: c => E.checksFor(c, v0),
   repair: (h, k, er) => E.repairSquare(h, k, v0, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0, verify: (c, k) => E.verify(c, k, v0),
   overhead: E.sizes({N, L, p: v0.p, q: v0.q, confirm: true, bytes}).overhead, share: share(nsq * 48, nsq * L)},
  {name: "v1(a)", alphabet: "bit", squares: bitSquares, checks: c => V.checksForA(c, A),
   repair: (h, k, er) => V.repairA(h, k, A, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0, verify: (c, k) => V.verifyA(c, k, A),
   overhead: V.sizesA({N, L, p: A.p, q: A.q, confirm: true, bytes}).overhead, share: share(nsq * 60, nsq * L)},
  {name: "v1(b)", alphabet: "bit", squares: mirrored, checks: c => E.checksFor(c, B),
   repair: (h, k, er) => V.repairB(h, k, B, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0, verify: (c, k) => E.verify(c, k, B),
   overhead: V.sizesB({N, L, K: B.K, p: B.p, q: B.q, confirm: true, bytes}).overhead, share: V.sizesB({N, L, K: B.K, p: B.p, q: B.q, confirm: true, bytes}).share},
  {name: "v1(c)", alphabet: "bit", squares: permuted, checks: c => E.checksFor(V.permuteC(c, C), C),
   repair: (h, k, er) => V.repairC(h, k, C, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0, verify: (c, k) => E.verify(V.permuteC(c, C), k, C),
   overhead: E.sizes({N, L, p: C.p, q: C.q, confirm: true, bytes}).overhead, share: share(nsq * 48, nsq * L)},
  {name: "v2(a)", alphabet: "trit", squares: encA.squares, checks: (c, s) => ({flag: encA.flags[s], ext: encA.external[s]}),
   repair: (h, k, er) => W.repairA(h, k.flag, k.ext, c2, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0, verify: () => null,
   overhead: W.sizesA(encA.meta).overhead, share: W.sizesA(encA.meta).share},
  {name: "v2(b)", alphabet: "trit", squares: encB.squares, checks: c => E.checksFor(c, c2),
   repair: (h, k, er) => W.repairSquare(h, k, c2, er ? {erased: er} : undefined), isDirect: r => (r.searched || 0) === 0, verify: () => null,
   overhead: W.sizesB(encB.meta).overhead, share: share(nsq * 48, nsq * L)},
  {name: "v2(b)·bat", alphabet: "bit", squares: bitSquares, checks: c => E.checksFor(c, c2),
   repair: (h, k, er) => W.repairSquare(h, k, c2, {canonical: false, cap: 12, erased: er}), isDirect: r => (r.searched || 0) === 0, verify: (c, k) => E.verify(c, k, c2),
   overhead: W.sizesB(encB.meta).overhead, share: share(nsq * 48, nsq * L)},
];
const pad = (s, n) => String(s).padEnd(n), rpad = (s, n) => String(s).padStart(n);
const hit = (h, i, alphabet) => { if(alphabet === "bit") h[i] ^= 1; else h[i] = h[i] === 0 ? (pick(2) ? 1 : -1) : 0; };
const results = {};
const W1 = 38, W2 = 11;

function channel(label, damage, opts){
  const o = opts || {};
  g = mul32(20260902 + label.length);
  const tallies = COLS.map(() => ({corrected: 0, detected: 0, wrong: 0, direct: 0, na: false}));
  for(let t = 0; t < T; t++){
    const at = pick(1 << 30), seedDmg = g();
    COLS.forEach((col, i) => {
      let squares = col.squares, trit = col.alphabet === "trit", alphabet = col.alphabet;
      if(o.canonicalOnly){
        if(col.tritSquares){ squares = col.tritSquares; trit = true; alphabet = "trit"; }
        else if(!trit){ tallies[i].na = true; return; }
      }
      const s = at % squares.length, stored = squares[s], chk = col.checks(stored, s), hurt = stored.slice();
      g = mul32(seedDmg);
      const erased = damage(hurt, GEO, alphabet, stored);
      const r = col.repair(hurt, chk, erased, trit);
      const x = tallies[i];
      if(r.status === "corrected"){ if(same(hurt, stored)){ x.corrected++; if(col.isDirect(r)) x.direct++; } else x.wrong++; }
      else x.detected++;
    });
    g = mul32(seedDmg ^ 0x5bd1e995);
  }
  results[label] = Object.fromEntries(COLS.map((c, i) => [c.name, tallies[i]]));
}
const cellOK = x => x.na ? "--" : `${x.corrected}${x.wrong ? "/" + x.wrong + "W" : ""}`;
const cellDir = x => x.na ? "--" : `${x.direct}`;

const CHANNELS = [
  ["1 cell hit", (h, cd, a) => { hit(h, pick(L), a); }],
  ["2 cells, anywhere", (h, cd, a) => { let x = pick(L), y = pick(L); while(y === x) y = pick(L); hit(h, x, a); hit(h, y, a); }],
  ["2 cells, same region", (h, cd, a) => { const k = pick(3), m = cd.members[k]; let x = m[pick(m.length)], y = m[pick(m.length)]; while(y === x) y = m[pick(m.length)]; hit(h, x, a); hit(h, y, a); }],
  ["2 cells, different regions", (h, cd, a) => { let x = pick(L), y = pick(L); while(cd.region[y] === cd.region[x]) y = pick(L); hit(h, x, a); hit(h, y, a); }],
  ["3 cells, one per region", (h, cd, a) => { for(let k = 0; k < 3; k++){ const m = cd.members[k]; hit(h, m[pick(m.length)], a); } }],
  ["12-cell row burst, flagged", (h, cd, a) => { const r = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ h[r * N + c0 + j] = a === "bit" ? -1 : 0; F.push(r * N + c0 + j); } return F; }],
  ["12-cell row burst, UNFLAGGED, in-region", (h, cd, a) => { for(;;){ const r = pick(N), c0 = pick(N - 12), regs = new Set(); for(let j = 0; j < 12; j++) regs.add(cd.region[r * N + c0 + j]); if(regs.size === 1 && !regs.has(E.FOLD)){ for(let j = 0; j < 12; j++) hit(h, r * N + c0 + j, a); return; } } }],
  ["the Fold filled, 32 unflagged", (h, cd, a) => { for(const i of cd.members[E.FOLD]) hit(h, i, a); }],
  ["1 sign flip, canonical square", (h, cd, a, st) => { const lit = []; for(let i = 0; i < L; i++) if(st[i]) lit.push(i); const i = lit[pick(lit.length)]; h[i] = -h[i]; }, {canonicalOnly: true}],
];
for(const [label, dmg, opts] of CHANNELS) channel(label, dmg, opts);
/* push */
{
  g = mul32(20260902);
  const Tp = Math.min(T, 200), holds = COLS.map(() => 0);
  for(let t = 0; t < Tp; t++){ const at = pick(1 << 30); COLS.forEach((col, i) => { const s = at % col.squares.length, st = col.squares[s], v = col.verify(pushLeft(st), col.checks(st, s)); if(v === null) holds[i] = "vac."; else if(v) holds[i]++; }); }
  results["push holds"] = Object.fromEntries(COLS.map((c, i) => [c.name, typeof holds[i] === "number" ? `${holds[i]}/${Tp}` : holds[i]]));
}

console.log(`THE STANDINGS -- ${path.basename(file)}, ${bytes} bytes, N=${N}, ${T} trials per channel, same squares and damage positions for every column`);
console.log(`corrected, with miscorrections as /nW; "--" = the column's alphabet cannot hold the channel; codegg-v1 takes the sign-flip row in its trit mode\n`);
console.log(pad("channel", W1) + COLS.map(c => rpad(c.name, W2)).join(""));
console.log("-".repeat(W1 + W2 * COLS.length));
for(const [label] of CHANNELS) console.log(pad(label, W1) + COLS.map(c => rpad(cellOK(results[label][c.name]), W2)).join(""));
console.log(pad("push: checks still hold", W1) + COLS.map(c => rpad(results["push holds"][c.name], W2)).join(""));
console.log(pad("cost per data bit", W1) + COLS.map(c => rpad((c.overhead * 100).toFixed(2) + "%", W2)).join(""));
console.log(pad("cost as share of all bits stored", W1) + COLS.map(c => rpad((c.share * 100).toFixed(2) + "%", W2)).join(""));
console.log(`\ndirect -- corrected by a syndrome naming its own cell, no search (the fold-native claims are about this column)\n`);
console.log(pad("channel", W1) + COLS.map(c => rpad(c.name, W2)).join(""));
console.log("-".repeat(W1 + W2 * COLS.length));
for(const [label] of CHANNELS) console.log(pad(label, W1) + COLS.map(c => rpad(cellDir(results[label][c.name]), W2)).join(""));

/* the house's rule, mechanical: an arm that returned WRONG data on any
   channel cannot claim a row; among the rest, a row is "kept" by the arms
   that correct it at the lowest cost per data bit. */
console.log(`\nwho keeps each row -- the cheapest arm(s) correcting >= 99% of it with 0 WRONG on every channel:`);
const clean = COLS.filter(c => CHANNELS.every(([l]) => results[l][c.name].na || results[l][c.name].wrong === 0));
for(const [label] of CHANNELS){
  const able = clean.filter(c => !results[label][c.name].na && results[label][c.name].corrected >= 0.99 * T);
  if(!able.length){ console.log(`  ${pad(label, W1)} nobody`); continue; }
  const cheapest = Math.min(...able.map(c => c.overhead));
  console.log(`  ${pad(label, W1)} ${able.filter(c => c.overhead === cheapest).map(c => c.name).join(", ")} at ${(cheapest * 100).toFixed(2)}%` + (able.length > 1 ? `   (also: ${able.filter(c => c.overhead !== cheapest).map(c => `${c.name} ${(c.overhead * 100).toFixed(2)}%`).join(", ")})` : ""));
}
console.log(`  arms with a WRONG anywhere, disqualified from keeping any row: ${COLS.filter(c => !clean.includes(c)).map(c => c.name).join(", ") || "none"}`);
if(argv.indexOf("--json") >= 0) fs.writeFileSync(path.join(__dirname, "..", "measured-standings.json"), JSON.stringify({file: path.basename(file), bytes, N, T, columns: COLS.map(c => ({name: c.name, alphabet: c.alphabet, overhead: c.overhead, share: c.share})), results}, null, 1));
