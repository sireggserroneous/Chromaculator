/* node tools/ledger12.js -- the per-milestone ledger, in lanes.
 * eggv12 transmutes every corpus file, takes the tournament's three injuries,
 * and prints bytes beside the SEALED v11 baseline (codegg-v11/ledger-m8-sealed.txt,
 * byte-exact). The milestone law: no row worse than v10 by >0.05 pt without a
 * filed cause; injuries all EXACT or the row is a FAILURE.
 * The monster rejoined at M2 (the cliff retired: 40m23s end-to-end). */
const fs = require('fs'), path = require('path'), os = require('os');
const { execFile, execFileSync } = require('child_process');
const here = path.dirname(__filename);
const root = path.join(here, '..');
const EXE = process.env.EGG_EXE || path.join(root, 'target', 'release', 'eggv12.exe'); // EGG_EXE: a copied build (the exe lock)
const V11 = { // bytes, codegg-v11/ledger-m8-sealed.txt (the sealed Rematch, 2026-09-02)
  'alarm01.wav': 273196, 'aoe4-autosave.sav': 17553840, 'arial.ttf': 468292,
  'cbs.log': 150804, 'iconcache48.db': 424760, 'kernel32.dll': 300832,
  'mermaid-bundle.js': 4891080, 'msgraph.dll': 4617660, 'notepad.exe': 183060,
  'ntoskrnl.exe': 5039572, 'rdr2-shaders.vkcache': 42312400,
  'real-test.bmp': 268588, 'real-test.db': 1241376, 'ring01.wav': 146184,
  'segoeui.ttf': 429368, 'vim-version9.txt': 319264, 'wallpaper.jpg': 1533228,
  'wubbadub.html': 30596, 'zstd.exe': 521540, 'rustc_driver.dll': 42719952,
};
// the M0 column, filed in PREDICTIONS.md BEFORE M1 code: every row is judged to the byte
const PRED = process.env.EGG_PRED ? JSON.parse(fs.readFileSync(process.env.EGG_PRED, 'utf8')) : null;
const SKIP = new Set((process.env.EGG_SKIP || '').split(',').filter(Boolean)); // EGG_SKIP=a,b to leave rows to the lanes
const ONLY = new Set((process.env.EGG_ONLY || '').split(',').filter(Boolean));
const LANES = Math.max(2, Math.min(8, os.cpus().length - 4));

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
function injure(buf, kind){
  const b = Buffer.from(buf);
  const g = mul32(0xACE);
  if(kind === 'flip'){ const at = Math.floor(b.length / 2); b[at] ^= 0x40; return [b, at, 1]; }
  if(kind === 'trunc') return [b.subarray(0, Math.max(0, b.length - 4096)), b.length - 4096, 4096];
  const at = Math.max(0, Math.floor(b.length / 2) - 2048);
  for(let i = at; i < at + 4096 && i < b.length; i++) b[i] = g() & 0xff;
  return [b, at, 4096];
}
// EGG_ARMS_DIR: with EGG_ARMS=1 in the environment eggv12 prints every arm's inner on
// stderr; the transmute's stderr is kept per row there (the M2b prediction needs the
// min inner over the roster, not only the winner)
function run(args){ return new Promise((res, rej) => execFile(EXE, args, {maxBuffer: 1 << 28}, (e, so, se) => {
  if(process.env.EGG_ARMS_DIR && args[0] === 'transmute'){
    try { fs.writeFileSync(path.join(process.env.EGG_ARMS_DIR, path.basename(args[1]) + '.arms.txt'), String(so) + '\n' + String(se)); } catch(err){}
  }
  e ? rej(e) : res();
})); }

async function doFile(dir, f){
  const src = fs.readFileSync(path.join(dir, f));
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'ldg12-'));
  const art = path.join(tmp, f + '.egg12');
  const row = { f, orig: src.length, t0: Date.now() };
  try {
    await run(['transmute', path.join(dir, f), '-o', art]);
    const a = fs.readFileSync(art);
    row.n12 = a.length; row.ms = Date.now() - row.t0;
    try { row.info = JSON.parse(execFileSync(EXE, ['info', art], {maxBuffer: 1 << 24}).toString()); } catch(e){ row.info = null; }
    row.verd = [];
    for(const kind of ['flip', 'scratch', 'trunc']){
      const [d, at, len] = injure(a, kind);
      const dp = path.join(tmp, 'd.egg12'), op = path.join(tmp, 'd.out');
      fs.writeFileSync(dp, d);
      const args = ['restore', dp, '-o', op];
      if(kind === 'scratch'){ args.push('--wound', `${at}:${len}`); }
      let v = 'dead';
      try { await run(args); v = fs.readFileSync(op).equals(src) ? 'E' : 'WRONG'; } catch(e){}
      row.verd.push(v);
    }
  } catch(e){ row.err = String(e).slice(0, 100); }
  fs.rmSync(tmp, {recursive: true, force: true});
  return row;
}
async function pool(items, n, fn){
  const out = new Array(items.length); let i = 0;
  await Promise.all(Array.from({length: Math.min(n, items.length)}, async () => {
    while(i < items.length){ const k = i++; out[k] = await fn(items[k][0], items[k][1]); }
  }));
  return out;
}
(async () => {
  const jobs = [];
  for(const dir of ['corpus-real', 'corpus-big'].map(d => path.join(root, d)))
    for(const f of fs.readdirSync(dir).sort()) if(!SKIP.has(f) && (ONLY.size === 0 || ONLY.has(f))) jobs.push([dir, f]);
  jobs.sort((a, b) => fs.statSync(path.join(b[0], b[1])).size - fs.statSync(path.join(a[0], a[1])).size);
  console.log(`M-LEDGER (eggv12 vs SEALED v11 bytes${PRED ? ' + the filed prediction' : ''}) -- ${jobs.length} files, ${LANES} lanes`);
  const rows = await pool(jobs, LANES, doFile);
  let worse = 0, fail = 0, savedTotal = 0, misses = 0;
  for(const r of rows.sort((a, b) => a.f.localeCompare(b.f))){
    if(r.err){ fail++; console.log(`FAIL ${r.f}: ${r.err}`); continue; }
    const v11 = V11[r.f];
    const d = r.n12 - v11;
    savedTotal -= d;
    const dpct = (100 * d / r.orig).toFixed(3);
    const inj = r.verd.join('/');
    const ok = r.verd.every(v => v === 'E');
    if(!ok) fail++;
    if(d > 0) worse++;
    const mbs = (r.orig / 1e6 / Math.max(0.001, r.ms / 1000)).toFixed(2);
    let predcol = '';
    if(PRED && PRED[r.f] !== undefined){
      const miss = r.n12 - PRED[r.f];
      predcol = ` pred=${String(PRED[r.f]).padStart(9)} ${miss === 0 ? 'HIT' : 'MISS ' + (miss > 0 ? '+' : '') + miss}`;
      if(miss !== 0) misses++;
    }
    const geo = r.info ? ` inner=${r.info.len} model=${r.info.model} blk=${r.info.block} t=${r.info.t} ${r.info.mode} price=${r.info.price} (${Number(r.info.floor_x).toFixed(2)}x floor)` : '';
    console.log(`${r.f.padEnd(22)} v11=${String(v11).padStart(9)} v12=${String(r.n12).padStart(9)} delta=${String(d).padStart(8)} (${dpct} pt)${predcol} inj=${inj} ${mbs} MB/s${geo}${ok ? '' : '  INJURY FAIL'}`);
  }
  console.log(`net bytes saved vs v11: ${savedTotal}; rows heavier than v11: ${worse}; prediction misses: ${misses}; failures: ${fail}`);
  process.exit(fail ? 1 : 0);
})();
