/* node tools/card.js -- the challengers card, side by side.
 * Every rival number was taken with the rival RESTORING the original byte for
 * byte first; a tool that cannot restore does not get a column. `ours` is the
 * INNER (sealed minus the 4,812 B shield) because no rival carries one -- the
 * ratio ring. Re-run after any new measurement lands in card.json. */
const fs = require('fs'), path = require('path');
const root = path.join(path.dirname(__filename), '..');
const C = JSON.parse(fs.readFileSync(path.join(root, 'tools', 'card.json'), 'utf8'));
const dirs = ['corpus-real', 'corpus-big'].map(d => path.join(root, d));
const inputOf = f => { for (const d of dirs) { const p = path.join(d, f); if (fs.existsSync(p)) return fs.statSync(p).size; } return null; };
const pct = (r, o) => r == null ? '' : (100 * (r - o) / o).toFixed(2).replace(/^(?!-)/, '+') + '%';
const n = v => v == null ? '—' : v.toLocaleString('en-US');

const rows = Object.keys(C.sealed_v13).map(f => {
  const ours = C.sealed_v13[f] - C.armor_price;
  const z = C.zpaq_m5[f] ?? null, p = C.paq8px_9LAET[f] ?? null, pc = C.precomp_best[f] ?? null;
  const rivals = [z, p, pc].filter(v => v != null);
  const best = rivals.length ? Math.min(...rivals) : null;
  return { f, input: inputOf(f), ours, z, p, pc, best, gap: best == null ? null : (best - ours) / ours };
}).sort((a, b) => (b.gap ?? -9) - (a.gap ?? -9));

console.log('| row | input | ours (inner) | zpaq -m5 | paq8px | precomp+ | best rival | vs ours |');
console.log('|---|---:|---:|---:|---:|---:|---:|---:|');
for (const r of rows)
  console.log(`| ${r.f} | ${n(r.input)} | **${n(r.ours)}** | ${n(r.z)} ${pct(r.z, r.ours)} | ${n(r.p)} ${pct(r.p, r.ours)} | ${n(r.pc)} ${pct(r.pc, r.ours)} | ${n(r.best)} | ${r.gap == null ? '—' : (r.gap > 0 ? '**we win** ' : '**we lose** ') + pct(r.best, r.ours)} |`);

const done = rows.filter(r => r.best != null);
const win = done.filter(r => r.gap > 0), lose = done.filter(r => r.gap <= 0);
const sumOurs = done.reduce((s, r) => s + r.ours, 0), sumBest = done.reduce((s, r) => s + r.best, 0);
const allOurs = rows.reduce((s, r) => s + r.ours, 0);
console.log(`\nmeasured ${done.length}/${rows.length} rows -- we win ${win.length}, we lose ${lose.length}`);
console.log(`bytes on measured rows: ours ${n(sumOurs)}  best-rival ${n(sumBest)}  -> ${pct(sumBest, sumOurs)}`);
console.log(`measured rows are ${(100 * sumOurs / allOurs).toFixed(1)}% of the ${n(allOurs)} B sealed corpus (inner)`);
const unmeasured = rows.filter(r => r.best == null);
if (unmeasured.length) console.log(`NOT YET CONTESTED: ${unmeasured.map(r => `${r.f} (${n(r.ours)})`).join(', ')}`);
