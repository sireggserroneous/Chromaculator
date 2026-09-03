/* node eggSo-v0/tools/versus.js <file> [--N 32] [--trials 300] [--bare]
 *
 *   --bare   drop the whole-square confirming residue: the construction as
 *            PREDICTIONS.md first filed it, three residues, 3.52%. Kept so
 *            the alias floor that forced confirm on can be reproduced.
 *
 *   --per-candidate   ask the confirming residue INSIDE the in-region search
 *            rather than after the plan (codegg-v1's rule, codegg.js:204-206).
 *            THE AMENDMENT of 2026-09-02: the same-region pair column below
 *            is the one this round misread as the partition's cost. See the
 *            note in eggso.js and the Amendment section of README.md.
 *
 * Head to head: codegg-v1 (one residue of the whole square) against eggSo v0
 * (one residue per fold region). Same file, same squares, same damage, cell
 * for cell -- both codecs lay bytes into the square with v1's own toCells, so
 * there is nothing to normalise.
 *
 * Four numbers per channel, and they are different outcomes:
 *   corrected      restored exactly
 *   detected       refused to guess; the data is marked, not lost
 *   MISCORRECTED   "repaired" into the wrong bytes -- the failure that matters
 *   direct         corrected with NO search: the region's own syndrome named
 *                  the cell. This column is the round's claim. v1 can only
 *                  ever show it for single errors.
 */
const fs = require("fs"), path = require("path");
const G = require(path.join(__dirname, "..", "..", "codegg-v1", "codegg.js"));
const E = require(path.join(__dirname, "..", "eggso.js"));

const argv = process.argv.slice(2);
const flag = (n, d) => { const i = argv.indexOf("--" + n); return i >= 0 && argv[i + 1] && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const has = n => argv.indexOf("--" + n) >= 0;
const file = argv.find(a => !a.startsWith("--") && !/^\d+$/.test(a));
if(!file){ console.error("usage: versus.js <file> [--N 32] [--trials 300] [--bare]"); process.exit(1); }

const N = parseInt(flag("N", "32"), 10), L = N * N, T = parseInt(flag("trials", "300"), 10);
const src = fs.readFileSync(file);
const v1 = G.makeCode(N), so = E.makeCode(N, {confirm: !has("bare")});

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(20260902);
const pick = n => g() % n;
const same = (a, b) => Buffer.from(a).equals(Buffer.from(b));

/* one square of the real file per trial, damaged the same way for both */
const squares = G.toCells(src, L);
const pad = (s, n) => String(s).padEnd(n);

function channel(label, damage, note){
  const tally = () => ({corrected: 0, detected: 0, wrong: 0, direct: 0});
  const a = tally(), b = tally();
  for(let t = 0; t < T; t++){
    const cells = squares[pick(squares.length)];
    const dmg = damage(cells.slice(), so);           // {hurt, erased?}
    /* v1 */
    { const h = dmg.hurt.slice(), chk = [G.residue(cells, v1.p), G.residue(cells, v1.q)];
      const r = G.repairSquare(h, chk, v1, dmg.erased ? {erased: dmg.erased} : undefined);
      if(r.status === "corrected"){ if(same(h, cells)){ a.corrected++; if(r.note === "single" || r.note === "erasures") a.direct++; } else a.wrong++; }
      else a.detected++; }
    /* eggSo */
    { const h = dmg.hurt.slice(), chk = E.checksFor(cells, so);
      const o = {};
      if(dmg.erased) o.erased = dmg.erased;
      if(has("per-candidate")) o.perCandidate = true;          // the amendment; see eggso.js
      const r = E.repairSquare(h, chk, so, Object.keys(o).length ? o : undefined);
      if(r.status === "corrected"){ if(same(h, cells)){ b.corrected++; if((r.searched || 0) === 0) b.direct++; } else b.wrong++; }
      else b.detected++; }
  }
  const fmt = x => `${pad(x.corrected, 4)} ok ${pad(x.detected, 4)} det ${pad(x.wrong, 3)} WRONG ${pad(x.direct, 4)} direct`;
  console.log(`  ${pad(label, 34)} ${fmt(a)}   |   ${fmt(b)}${note ? "\n  " + " ".repeat(34) + note : ""}`);
}

console.log(`${path.basename(file)}, ${src.length} bytes, N=${N}, ${T} trials per channel${so.confirm ? "" : ", eggSo BARE (no confirming residue)"}`);
console.log(`  eggSo ${(E.sizes({N, L, p: so.p, q: so.q, confirm: so.confirm, bytes: src.length}).overhead * 100).toFixed(2)}% overhead vs codegg-v1 ${(G.sizes({N, L, p: v1.p, q: v1.q, bytes: src.length}).overhead * 100).toFixed(2)}%\n`);
console.log(`  ${pad("channel", 34)} ${pad("codegg-v1  (whole-square residue)", 40)}   |   eggSo v0  (one residue per region)`);
console.log("  " + "-".repeat(118));

channel("1 cell flipped", h => { h[pick(L)] ^= 1; return {hurt: h}; });

channel("2 cells flipped, anywhere", h => { let a = pick(L), b = pick(L); while(b === a) b = pick(L); h[a] ^= 1; h[b] ^= 1; return {hurt: h}; },
  "the claim: eggSo's `direct` here is what v1 structurally cannot do");

channel("2 cells, DIFFERENT regions", (h, cd) => {
  let a = pick(L), b = pick(L); while(cd.region[b] === cd.region[a]) b = pick(L);
  h[a] ^= 1; h[b] ^= 1; return {hurt: h}; });

channel("2 cells, SAME region", (h, cd) => {
  const k = pick(3), m = cd.members[k]; let a = m[pick(m.length)], b = m[pick(m.length)]; while(b === a) b = m[pick(m.length)];
  h[a] ^= 1; h[b] ^= 1; return {hurt: h}; },
  "both fall to search; eggSo's is inside one region, and pays the alias risk of one prime");

channel("3 cells, one per region", (h, cd) => {
  for(let k = 0; k < 3; k++){ const m = cd.members[k]; h[m[pick(m.length)]] ^= 1; } return {hurt: h}; },
  "three single errors to eggSo; three unknowns in one syndrome to v1");

channel("12-cell row burst, flagged", h => {
  const row = pick(N), c0 = pick(N - 12), F = []; for(let j = 0; j < 12; j++){ const i = row * N + c0 + j; h[i] = -1; F.push(i); }
  return {hurt: h, erased: F}; });

channel("the Fold filled: 32 cells, unflagged", (h, cd) => { for(const i of cd.members[E.FOLD]) h[i] ^= 1; return {hurt: h}; },
  "the predicted weak spot -- a burst that exactly fills the 3% region");

console.log(`\n  direct = corrected by a syndrome naming its own cell, no search. For v1 that is singles and erasures only.`);
