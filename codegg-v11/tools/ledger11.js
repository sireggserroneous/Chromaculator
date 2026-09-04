/* node tools/ledger11.js -- the per-milestone ledger, in lanes.
 * eggv11 transmutes every corpus file, takes the tournament's three injuries,
 * and prints bytes beside the recorded v10 baseline (m0gate.txt's v10 column,
 * byte-exact). The milestone law: no row worse than v10 by >0.05 pt without a
 * filed cause; injuries all EXACT or the row is a FAILURE.
 * The monster rejoined at M2 (the cliff retired: 40m23s end-to-end). */
const fs = require('fs'), path = require('path'), os = require('os');
const { execFile } = require('child_process');
const here = path.dirname(__filename);
const root = path.join(here, '..');
const EXE = path.join(root, 'target', 'release', 'eggv11.exe');
const V10 = { // bytes, from m0gate.txt (bit-identity run, 2026-09-02)
  'alarm01.wav': 274208, 'aoe4-autosave.sav': 17789372, 'arial.ttf': 472900,
  'cbs.log': 151304, 'iconcache48.db': 427320, 'kernel32.dll': 300832,
  'mermaid-bundle.js': 4959348, 'msgraph.dll': 4665400, 'notepad.exe': 184584,
  'ntoskrnl.exe': 5096588, 'rdr2-shaders.vkcache': 42895828,
  'real-test.bmp': 269600, 'real-test.db': 1273812, 'ring01.wav': 148744,
  'segoeui.ttf': 434488, 'vim-version9.txt': 326432, 'wallpaper.jpg': 1550852,
  'wubbadub.html': 31120, 'zstd.exe': 524100, 'rustc_driver.dll': 43415092,
};
const SKIP = new Set([]); // M2 retired the cliff; the monster sits at the table
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
function run(args){ return new Promise((res, rej) => execFile(EXE, args, {maxBuffer: 1 << 28}, e => e ? rej(e) : res())); }

async function doFile(dir, f){
  const src = fs.readFileSync(path.join(dir, f));
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'ldg11-'));
  const art = path.join(tmp, f + '.egg11');
  const row = { f, orig: src.length };
  try {
    await run(['transmute', path.join(dir, f), '-o', art]);
    const a = fs.readFileSync(art);
    row.n11 = a.length;
    row.verd = [];
    for(const kind of ['flip', 'scratch', 'trunc']){
      const [d, at, len] = injure(a, kind);
      const dp = path.join(tmp, 'd.egg11'), op = path.join(tmp, 'd.out');
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
    for(const f of fs.readdirSync(dir).sort()) if(!SKIP.has(f)) jobs.push([dir, f]);
  jobs.sort((a, b) => fs.statSync(path.join(b[0], b[1])).size - fs.statSync(path.join(a[0], a[1])).size);
  console.log(`M-LEDGER (eggv11 vs recorded v10 bytes) -- ${jobs.length} files, ${LANES} lanes; rustc_driver deferred to M2 (the cliff)`);
  const rows = await pool(jobs, LANES, doFile);
  let worse = 0, fail = 0, savedTotal = 0;
  for(const r of rows.sort((a, b) => a.f.localeCompare(b.f))){
    if(r.err){ fail++; console.log(`FAIL ${r.f}: ${r.err}`); continue; }
    const v10 = V10[r.f];
    const d = r.n11 - v10;
    savedTotal -= d;
    const dpct = (100 * d / r.orig).toFixed(3);
    const inj = r.verd.join('/');
    const ok = r.verd.every(v => v === 'E');
    if(!ok) fail++;
    if(d > 0 && 100 * d / r.orig > 0.05) worse++;
    console.log(`${r.f.padEnd(22)} v10=${String(v10).padStart(9)} v11=${String(r.n11).padStart(9)} delta=${String(d).padStart(8)} (${dpct} pt) inj=${inj}${ok ? '' : '  INJURY FAIL'}`);
  }
  console.log(`net bytes saved vs v10: ${savedTotal}; rows worse >0.05pt: ${worse}; failures: ${fail}`);
  process.exit(fail ? 1 : 0);
})();
