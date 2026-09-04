/* node tools/jpegsuite.js -- the peel's CONSERVATION suite.
 *
 * The bar, from the charter plan's M1 gate: every file in corpus-jpeg either
 * peels and re-spells BYTE-EXACT, or is cleanly refused and keeps its bytes.
 * Nothing lost, nothing wrong. A refusal is a PASS as long as the file still
 * transmutes and restores exactly through the ordinary pipeline -- and the
 * reason is printed, never swallowed.
 *
 * EGG_EXE=<a copied build>   (the exe lock: never run the file cargo is writing)
 * EGG_V12=<codegg-v12 exe>   optional: print v12's total beside v13's
 * EGG_LANES=n                default min(8, cpus-4)
 */
const fs = require('fs'), path = require('path'), os = require('os');
const { execFile, execFileSync } = require('child_process');
const here = path.dirname(__filename);
const root = path.join(here, '..');
const EXE = process.env.EGG_EXE || path.join(root, 'target', 'release', 'eggv14.exe');
const V12 = process.env.EGG_V12 || '';
const LANES = Number(process.env.EGG_LANES) || Math.max(2, Math.min(8, os.cpus().length - 4));
const dir = path.join(root, 'corpus-jpeg');

function run(args, env) {
  return new Promise((res) => execFile(EXE, args, { maxBuffer: 1 << 28, env: Object.assign({}, process.env, env || {}) },
    (e, so, se) => res({ e, so: String(so), se: String(se) })));
}

/* the entropy-coded byte count of a baseline JPEG, so a peeled row can be
 * priced against the spelling it replaced. Returns 0 when the file is not a
 * single-scan JPEG this reader understands. */
function entropyBytes(b) {
  try {
    let i = 2;
    while (i + 3 < b.length) {
      if (b[i] !== 0xFF) return 0;
      const m = b[i + 1];
      if (m === 0xD8 || m === 0x01 || (m >= 0xD0 && m <= 0xD7)) { i += 2; continue; }
      if (m === 0xD9) return 0;
      const L = (b[i + 2] << 8) | b[i + 3];
      if (m === 0xDA) {
        let j = i + 2 + L;
        const start = j;
        while (j + 1 < b.length) {
          if (b[j] === 0xFF && b[j + 1] !== 0 && !(b[j + 1] >= 0xD0 && b[j + 1] <= 0xD7)) break;
          j++;
        }
        return j - start;
      }
      i += 2 + L;
    }
  } catch (e) { /* a hostile: it has no honest count */ }
  return 0;
}

async function doFile(f) {
  const src = fs.readFileSync(path.join(dir, f));
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'jsuite-'));
  const art = path.join(tmp, f + '.egg14'), out = path.join(tmp, f + '.out');
  const row = { f, orig: src.length, ecs: entropyBytes(src) };
  const t0 = Date.now();
  const r = await run(['transmute', path.join(dir, f), '-o', art], { EGG_PEEL: '1' });
  if (r.e) { row.verdict = 'TRANSMUTE FAILED'; row.why = String(r.e).slice(0, 120); fs.rmSync(tmp, { recursive: true, force: true }); return row; }
  row.ms = Date.now() - t0;
  row.total = fs.statSync(art).size;
  try { row.info = JSON.parse(execFileSync(EXE, ['info', art], { maxBuffer: 1 << 24 }).toString()); } catch (e) { row.info = null; }
  const m = (r.se.match(/peel \d+: REFUSED -- ([^\n;]+)/) || [])[1];
  row.why = m ? m.trim() : '';
  /* a peel that PROVED itself -- THE LAW passed, its re-encode WAS the
   * original file -- but that the trial then passed over because the
   * ordinary pipeline was lighter. That is the trial working, not a
   * refusal, and the two must not be reported as one thing. */
  row.proved = /peel \d+: recipe /.test(r.se);
  const rr = await run(['restore', art, '-o', out]);
  if (rr.e) row.verdict = 'LOST (restore refused)';
  else {
    const back = fs.readFileSync(out);
    row.verdict = back.equals(src) ? 'EXACT' : 'WRONG';
  }
  row.peeled = !!(row.info && row.info.model === 24);
  if (row.peeled && row.info) { row.recipe = row.info.peel_recipe; row.values = row.info.peel_values; }
  if (V12) {
    const a12 = path.join(tmp, f + '.egg12');
    try { execFileSync(V12, ['transmute', path.join(dir, f), '-o', a12], { stdio: 'pipe', maxBuffer: 1 << 26 }); row.v12 = fs.statSync(a12).size; } catch (e) { row.v12 = 0; }
  }
  fs.rmSync(tmp, { recursive: true, force: true });
  return row;
}

(async () => {
  const files = fs.readdirSync(dir).sort();
  console.log(`THE PEEL SUITE -- ${files.length} JPEGs, ${LANES} lanes; the law: peel byte-exact OR refuse and keep the bytes`);
  const rows = new Array(files.length);
  let i = 0;
  await Promise.all(Array.from({ length: Math.min(LANES, files.length) }, async () => {
    while (i < files.length) { const k = i++; rows[k] = await doFile(files[k]); }
  }));
  let peeled = 0, refused = 0, lost = 0, wrong = 0, gain = 0, v12sum = 0, v13sum = 0, passedOver = 0;
  console.log('\nfile                                          orig      entropy      total   %ecs   model  verdict   note');
  console.log('-'.repeat(150));
  for (const r of rows) {
    const pct = (r.peeled && r.ecs) ? ((100 * r.values / r.ecs).toFixed(2) + '%') : '';
    const model = r.info ? String(r.info.model) : '?';
    if (r.verdict === 'EXACT' && r.peeled) peeled++;
    else if (r.verdict === 'EXACT' && r.proved) { passedOver++; refused++; }
    else if (r.verdict === 'EXACT') refused++;
    else if (r.verdict === 'WRONG') wrong++;
    else lost++;
    if (r.v12) { v12sum += r.v12; v13sum += r.total; gain += r.v12 - r.total; }
    console.log(
      `${r.f.slice(0, 44).padEnd(44)} ${String(r.orig).padStart(9)} ${String(r.ecs || '-').padStart(10)} ${String(r.total || '-').padStart(10)} ${pct.padStart(7)}  ${model.padStart(4)}   ${r.verdict.padEnd(8)}  ${r.peeled ? `recipe ${r.recipe} values ${r.values}` : (r.proved ? 'peel PROVED byte-exact, the raw form won the argmin' : r.why)}${r.v12 ? `  [v12 ${r.v12}, ${r.v12 - r.total >= 0 ? '-' : '+'}${Math.abs(r.v12 - r.total)}]` : ''}`
    );
  }
  console.log('-'.repeat(150));
  console.log(`peeled-and-taken EXACT: ${peeled}   peel proved but passed over by the argmin: ${passedOver}   refused-and-kept EXACT: ${refused - passedOver}   LOST: ${lost}   WRONG: ${wrong}   (of ${rows.length})`);
  console.log(`the peel PROVED itself a bijection on ${peeled + passedOver} of ${rows.length}; every other file was refused with a printed reason and kept its bytes`);
  if (v12sum) console.log(`against codegg-v12 over the ${rows.filter(r => r.v12).length} rows that ran both: v12 ${v12sum} B, v13 ${v13sum} B, ${gain >= 0 ? '-' : '+'}${Math.abs(gain)} B`);
  process.exit(lost + wrong ? 1 : 0);
})();
