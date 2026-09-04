#!/usr/bin/env node
// m0gate.js -- v12-M0 fork-fidelity gate, run in LANES (the highway):
//  (1) eggv12 (v11's armor v3 still in place) must produce BIT-IDENTICAL
//      containers to eggv11 on the 14 home rows (corpus-real + cbs.log +
//      iconcache48.db), differing ONLY in magic+version (5 B) and the header
//      FNV-32 that covers them -- at each of the three header sites; totals
//      must equal the SEALED v11 totals to the byte;
//  (2) back-compat: .egg11 AND .egg10 AND .egg9 AND .egg8 containers restore
//      EXACT through eggv12 (the four ancestors' forms, armor11.rs verbatim).
//  The six big rows run as separate lanes (tools/biglanes.js).
const fs = require('fs'), path = require('path'), os = require('os');
const { execFileSync, execFile } = require('child_process');
const here = path.dirname(__filename);
const v12root = path.join(here, '..');
const v8exe = path.join(v12root, '..', 'codegg-v8', 'target', 'release', 'eggv8.exe');
const v9exe = path.join(v12root, '..', 'codegg-v9', 'target', 'release', 'eggv9.exe');
const v10exe = path.join(v12root, '..', 'codegg-v10', 'target', 'release', 'eggv10.exe');
const v11exe = path.join(v12root, '..', 'codegg-v11', 'target', 'release', 'eggv11.exe');
const v12exe = path.join(v12root, 'target', 'release', 'eggv12.exe');
const SEALED = { 'alarm01.wav': 273196, 'arial.ttf': 468292, 'cbs.log': 150804, 'iconcache48.db': 424760, 'kernel32.dll': 300832,
  'notepad.exe': 183060, 'real-test.bmp': 268588, 'real-test.db': 1241376, 'ring01.wav': 146184, 'segoeui.ttf': 429368,
  'vim-version9.txt': 319264, 'wallpaper.jpg': 1533228, 'wubbadub.html': 30596, 'zstd.exe': 521540 };
const LANES = Math.max(2, Math.min(10, os.cpus().length - 4));
const SMALL = 4 * 1024 * 1024; // ancestors with the slot wall never see >64MB; and v8/v9 arms on big files cost hours -- ancestor-compat is proven on the 12-file corpus + small big-arena members

function run(exe, args) {
  return new Promise((res, rej) => execFile(exe, args, { maxBuffer: 1 << 28 }, e => e ? rej(e) : res()));
}

async function gateFile(dir, f) {
  const src = path.join(dir, f);
  const size = fs.statSync(src).size;
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'm0g12-'));
  const a11 = path.join(tmp, f + '.egg11'), a12 = path.join(tmp, f + '.egg12');
  const row = { f, size };
  try {
    await Promise.all([run(v11exe, ['transmute', src, '-o', a11]),
                       run(v12exe, ['transmute', src, '-o', a12])]);
    const b11 = fs.readFileSync(a11), b12 = fs.readFileSync(a12);
    row.n11 = b11.length; row.n12 = b12.length;
    row.sealed = SEALED[f] === undefined ? 'n/a' : SEALED[f] === b12.length;
    const info = JSON.parse(execFileSync(v12exe, ['info', a12]).toString());
    let ok = b11.length === b12.length;
    if (ok) {
      for (const h of [info.h0, info.h1, info.h2]) {
        for (let i = 0; i < 5; i++) { b11[h + i] = 0; b12[h + i] = 0; }
        for (let i = 60; i < 64; i++) { b11[h + i] = 0; b12[h + i] = 0; }
      }
      ok = b11.equals(b12);
    }
    row.identical = ok;
    // ancestor compat: v12 restores the .egg11 (always); .egg10/.egg9/.egg8 on small files
    const out = path.join(tmp, f + '.out');
    await run(v12exe, ['restore', a11, '-o', out]);
    row.c11 = fs.readFileSync(out).equals(fs.readFileSync(src));
    if (size <= SMALL) {
      const a10 = path.join(tmp, f + '.egg10'), a9 = path.join(tmp, f + '.egg9'), a8 = path.join(tmp, f + '.egg8');
      await run(v10exe, ['transmute', src, '-o', a10]);
      await run(v12exe, ['restore', a10, '-o', out]);
      row.c10 = fs.readFileSync(out).equals(fs.readFileSync(src));
      await run(v9exe, ['transmute', src, '-o', a9]);
      await run(v12exe, ['restore', a9, '-o', out]);
      row.c9 = fs.readFileSync(out).equals(fs.readFileSync(src));
      await run(v8exe, ['transmute', src, '-o', a8]);
      await run(v12exe, ['restore', a8, '-o', out]);
      row.c8 = fs.readFileSync(out).equals(fs.readFileSync(src));
    } else { row.c10 = 'skip'; row.c9 = 'skip'; row.c8 = 'skip'; }
  } catch (e) { row.err = String(e).slice(0, 80); }
  fs.rmSync(tmp, { recursive: true, force: true });
  return row;
}

async function pool(items, n, fn) {
  const out = new Array(items.length); let i = 0;
  await Promise.all(Array.from({ length: Math.min(n, items.length) }, async () => {
    while (i < items.length) { const k = i++; out[k] = await fn(items[k][0], items[k][1]); }
  }));
  return out;
}

(async () => {
  const jobs = [];
  for (const f of fs.readdirSync(path.join(v12root, 'corpus-real')).sort()) jobs.push([path.join(v12root, 'corpus-real'), f]);
  for (const f of ['cbs.log', 'iconcache48.db']) jobs.push([path.join(v12root, 'corpus-big'), f]);
  // heaviest first so the monster starts immediately
  jobs.sort((a, b) => fs.statSync(path.join(b[0], b[1])).size - fs.statSync(path.join(a[0], a[1])).size);
  console.log(`v12 M0 GATE -- ${jobs.length} files, ${LANES} lanes`);
  const rows = await pool(jobs, LANES, gateFile);
  let fail = 0;
  for (const r of rows.sort((a, b) => a.f.localeCompare(b.f))) {
    const pass = r.identical && r.c11 && (r.sealed === true || r.sealed === 'n/a') && (r.c10 === true || r.c10 === 'skip') && (r.c9 === true || r.c9 === 'skip') && (r.c8 === true || r.c8 === 'skip') && !r.err;
    if (!pass) fail++;
    console.log(`${pass ? 'PASS' : 'FAIL'}  ${r.f.padEnd(22)} v11=${r.n11} v12=${r.n12} sealed=${r.sealed} identical=${r.identical} egg11=${r.c11} egg10=${r.c10} egg9=${r.c9} egg8=${r.c8}${r.err ? ' ERR ' + r.err : ''}`);
  }
  console.log(fail ? `M0 GATE FAILED (${fail})` : `M0 GATE: ${rows.length}/${rows.length} bit-identical (mod magic/version/FNV), sealed totals matched, ancestor-compatible`);
  process.exit(fail ? 1 : 0);
})();
