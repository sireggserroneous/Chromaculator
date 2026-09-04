/* arena.js -- THE SECOND ARENA (v13-M3d), reported APART from the sealed 20 + 3.
 *
 * Vladimir's ruling, 2026-09-03: the 20 + 3 ledger rows stay sealed and frozen
 * for comparison with v11/v12, and the formats we lack get a second, clearly
 * labelled arena with its own totals. This is that arena. It NEVER touches the
 * sealed rows and its numbers are never added to theirs.
 *
 * The gate is the deflate suite's: every member either transmutes and restores
 * BYTE-EXACT, or is cleanly refused and keeps its bytes. LOST must be zero and
 * WRONG must be zero. A member that peels and is then passed over by the argmin
 * is neither lost nor wrong -- that is the trial doing its job.
 *
 *   EGG_EXE=<a copied build>   the build under test (the exe lock: never the
 *                              file cargo is writing)
 *   EGG_BASE=<a copied build>  optional: a second build to print beside it, so
 *                              the arena's movement is a measurement and not a
 *                              memory
 *   node tools/arena.js [dir]  default: corpus-arena
 */
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execFileSync, spawnSync } = require('child_process');

const root = path.join(path.dirname(__filename), '..');
const exe = process.env.EGG_EXE || path.join(root, 'target', 'release', 'eggv13.exe');
const base = process.env.EGG_BASE || '';
const dir = process.argv[2] || path.join(root, 'corpus-arena');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'arena-'));

const prov = {};
try {
  for (const line of fs.readFileSync(path.join(dir, 'suite.txt'), 'utf8').split('\n')) {
    const m = line.match(/^(\S+)\s+\d+\s+(.*)$/);
    if (m) prov[m[1]] = m[2];
  }
} catch (e) { /* provenance is optional */ }

function weigh(bin, src, tag) {
  const cont = path.join(tmp, tag + '.egg13');
  const out = path.join(tmp, tag + '.out');
  const r = { total: 0, verdict: 'LOST', model: -1, note: '', chain: false, proved: false };
  const t = spawnSync(bin, ['transmute', src, '-o', cont], { stdio: 'pipe', env: { ...process.env, EGG_PEEL: '1' } });
  const err = String(t.stderr);
  const ref = err.match(/peel \d+: REFUSED -- ([^;]+)/);
  if (ref) r.note = ref[1].trim();
  if (/THE CHAIN took depth 2/.test(err)) r.chain = true;
  // the OUTER peel proved itself whenever it got as far as naming its recipe.
  // A "REFUSED" line can come from the CHAIN's inner attempt on values that
  // merely look like a deflate stream, and reporting that as the row's story
  // is a lie about which peel refused -- so the recipe line wins.
  if (/peel \d+: recipe /.test(err)) r.proved = true;
  if (t.status === 0 && fs.existsSync(cont)) {
    r.total = fs.statSync(cont).size;
    try { r.model = JSON.parse(execFileSync(bin, ['info', cont]).toString()).model; } catch (e) { r.model = -1; }
    const b = spawnSync(bin, ['restore', cont, '-o', out], { stdio: 'pipe' });
    if (b.status === 0 && fs.existsSync(out)) {
      r.verdict = fs.readFileSync(out).equals(fs.readFileSync(src)) ? 'EXACT' : 'WRONG';
    }
  }
  try { fs.rmSync(cont); } catch (e) {}
  try { fs.rmSync(out); } catch (e) {}
  return r;
}

const files = fs.readdirSync(dir).filter(f => f !== 'suite.txt').sort();
let exact = 0, wrong = 0, lost = 0, peeled = 0, refused = 0, passedOver = 0, chains = 0;
let tot = 0, totBase = 0, totOrig = 0;
console.log(`THE SECOND ARENA -- ${files.length} members, reported APART from the sealed 20 + 3`);
console.log(`build: ${exe}${base ? `\nbaseline: ${base}` : ''}\n`);
for (const f of files) {
  const src = path.join(dir, f);
  const orig = fs.statSync(src).size;
  const r = weigh(exe, src, f);
  const b = base ? weigh(base, src, f + '.base') : null;
  if (r.verdict === 'EXACT') exact++; else if (r.verdict === 'WRONG') wrong++; else lost++;
  if (r.model === 24) peeled++; else if (r.proved) passedOver++; else if (r.note) refused++; else passedOver++;
  if (r.chain) chains++;
  tot += r.total; totOrig += orig; if (b) totBase += b.total;
  const what = r.model === 24 ? (r.chain ? 'PEELED, CHAINED to depth 2' : 'PEELED')
    : r.proved ? 'peel PROVED, passed over by the argmin'
    : (r.note ? 'refused: ' + r.note.slice(0, 58) : 'no peel nominated (the ordinary pipeline)');
  const bc = b ? `  base=${String(b.total).padStart(9)} ${r.total === b.total ? 'unmoved' : (r.total < b.total ? '-' : '+') + Math.abs(r.total - b.total)}` : '';
  console.log(`${f.padEnd(26)} ${String(orig).padStart(10)} -> ${String(r.total).padStart(9)}  ${r.verdict.padEnd(6)} ${what}${bc}`);
  // the prober's reading, printed for every member it can read
  const m = spawnSync(exe, ['members', src], { stdio: 'pipe' });
  const head = String(m.stdout).split('\n')[0];
  if (head && !/no container layout/.test(head)) {
    const tail = String(m.stdout).trim().split('\n').pop();
    console.log(`${''.padEnd(26)} prober: ${head.replace(/^.*?: /, '')}; ${tail.trim()}`);
  }
  if (prov[f]) console.log(`${''.padEnd(26)} provenance: ${prov[f]}`);
}
console.log(`\nARENA TOTAL  ${totOrig} -> ${tot}${base ? `  (baseline ${totBase}, delta ${tot - totBase} = ${(100 * (tot - totBase) / totBase).toFixed(3)}%)` : ''}`);
console.log(`${files.length} members: ${exact} EXACT, ${wrong} WRONG, ${lost} LOST; ${peeled} took the peeled form (${chains} of them CHAINED to depth 2), ${refused} refused the peel with a reason, ${passedOver} passed over by the argmin`);
fs.rmSync(tmp, { recursive: true, force: true });
process.exit(wrong || lost ? 1 : 0);
