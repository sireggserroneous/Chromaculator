/* deflatesuite.js -- the M2 conservation suite for the deflate peel (WS-D).
 *
 * The gate: every file either PEELS and re-spells BYTE-EXACT, or is cleanly
 * REFUSED and keeps its bytes. LOST must be zero and WRONG must be zero. A file
 * that peels and is then passed over by the argmin is neither lost nor wrong --
 * it is the trial doing its job, and it is counted in its own column.
 *
 * Usage: EGG_EXE=<a copied build> node tools/deflatesuite.js <dir>
 * The directory carries a suite.txt naming every file's provenance.
 */
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execFileSync, spawnSync } = require('child_process');

const root = path.join(path.dirname(__filename), '..');
const exe = process.env.EGG_EXE || path.join(root, 'target', 'release', 'eggv13.exe');
const dir = process.argv[2];
if (!dir) { console.error('usage: node tools/deflatesuite.js <dir>'); process.exit(2); }
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'dsuite-'));

const prov = {};
try {
  for (const line of fs.readFileSync(path.join(dir, 'suite.txt'), 'utf8').split('\n')) {
    const m = line.match(/^(\S+)\s+\d+\s+(.*)$/);
    if (m) prov[m[1]] = m[2];
  }
} catch (e) { /* provenance is optional */ }

const files = fs.readdirSync(dir).filter(f => f !== 'suite.txt').sort();
let exact = 0, wrong = 0, lost = 0, peeled = 0, refused = 0, passedOver = 0;
const rows = [];
for (const f of files) {
  const src = path.join(dir, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.egg13');
  const out = path.join(tmp, f + '.out');
  let verdict = 'LOST', model = -1, note = '';
  const t = spawnSync(exe, ['transmute', src, '-o', cont], { stdio: 'pipe', env: { ...process.env, EGG_PEEL: '1' } });
  if (t.status === 0 && fs.existsSync(cont)) {
    const err = String(t.stderr);
    const m = err.match(/peel 2: REFUSED -- ([^;]+)/);
    if (m) { note = m[1].trim(); }
    try { model = JSON.parse(execFileSync(exe, ['info', cont]).toString()).model; } catch (e) { model = -1; }
    const r = spawnSync(exe, ['restore', cont, '-o', out], { stdio: 'pipe' });
    if (r.status === 0 && fs.existsSync(out)) {
      verdict = fs.readFileSync(out).equals(orig) ? 'EXACT' : 'WRONG';
    }
  }
  if (verdict === 'EXACT') exact++; else if (verdict === 'WRONG') wrong++; else lost++;
  if (model === 24) peeled++;
  else if (note) refused++;
  else passedOver++;
  rows.push({ f, n: orig.length, total: fs.existsSync(cont) ? fs.statSync(cont).size : 0, verdict, model, note });
  try { fs.rmSync(cont); } catch (e) {}
  try { fs.rmSync(out); } catch (e) {}
}
for (const r of rows) {
  const what = r.model === 24 ? 'PEELED' : (r.note ? 'refused: ' + r.note.slice(0, 62) : 'raw pipeline (the argmin kept the bytes)');
  console.log(`${r.f.padEnd(34)} ${String(r.n).padStart(9)} -> ${String(r.total).padStart(9)}  ${r.verdict.padEnd(6)} ${what}`);
  if (prov[r.f]) console.log(`${''.padEnd(34)} provenance: ${prov[r.f]}`);
}
console.log(`\n${files.length} files: ${exact} EXACT, ${wrong} WRONG, ${lost} LOST; ${peeled} took the peeled form, ${refused} refused the peel with a reason, ${passedOver} peeled-or-nominated and passed over by the argmin`);
fs.rmSync(tmp, { recursive: true, force: true });
process.exit(wrong || lost ? 1 : 0);
