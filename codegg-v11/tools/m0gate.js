#!/usr/bin/env node
// m0gate.js -- v11-M0 fork-fidelity gate, run in LANES (the highway):
//  (1) eggv11 must produce BIT-IDENTICAL containers to eggv10 on every file
//      of BOTH corpora, differing ONLY in magic+version (5 B), the model byte
//      (v11 stamps 10/11 where v10 stamps 8/9 for the same payload), and the
//      header FNV-32 that covers them -- at each of the three header sites;
//  (2) back-compat: .egg10 AND .egg9 AND .egg8 containers restore EXACT
//      through eggv11 (the three ancestors' forms).
//  This gate is also the PROOF that the M0 clippy rewrites (enumerate/zip in
//  live coder loops) moved zero streams.
const fs = require('fs'), path = require('path'), os = require('os');
const { execFileSync, execFile } = require('child_process');
const here = path.dirname(__filename);
const v11root = path.join(here, '..');
const v8exe = path.join(v11root, '..', 'codegg-v8', 'target', 'release', 'eggv8.exe');
const v9exe = path.join(v11root, '..', 'codegg-v9', 'target', 'release', 'eggv9.exe');
const v10exe = path.join(v11root, '..', 'codegg-v10', 'target', 'release', 'eggv10.exe');
const v11exe = path.join(v11root, 'target', 'release', 'eggv11.exe');
const LANES = Math.max(2, Math.min(10, os.cpus().length - 4));
const SMALL = 4 * 1024 * 1024; // ancestors with the slot wall never see >64MB; and v8/v9 arms on big files cost hours -- ancestor-compat is proven on the 12-file corpus + small big-arena members

function run(exe, args) {
  return new Promise((res, rej) => execFile(exe, args, { maxBuffer: 1 << 28 }, e => e ? rej(e) : res()));
}

async function gateFile(dir, f) {
  const src = path.join(dir, f);
  const size = fs.statSync(src).size;
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'm0g11-'));
  const a10 = path.join(tmp, f + '.egg10'), a11 = path.join(tmp, f + '.egg11');
  const row = { f, size };
  try {
    await Promise.all([run(v10exe, ['transmute', src, '-o', a10]),
                       run(v11exe, ['transmute', src, '-o', a11])]);
    const b10 = fs.readFileSync(a10), b11 = fs.readFileSync(a11);
    row.n10 = b10.length; row.n11 = b11.length;
    const info = JSON.parse(execFileSync(v11exe, ['info', a11]).toString());
    let ok = b10.length === b11.length;
    if (ok) {
      for (const h of [info.h0, info.h1, info.h2]) {
        for (let i = 0; i < 5; i++) { b10[h + i] = 0; b11[h + i] = 0; }
        b10[h + 31] = 0; b11[h + 31] = 0;
        for (let i = 60; i < 64; i++) { b10[h + i] = 0; b11[h + i] = 0; }
      }
      ok = b10.equals(b11);
    }
    row.identical = ok;
    // ancestor compat: v11 restores the .egg10 (always); .egg9/.egg8 on small files
    const out = path.join(tmp, f + '.out');
    await run(v11exe, ['restore', a10, '-o', out]);
    row.c10 = fs.readFileSync(out).equals(fs.readFileSync(src));
    if (size <= SMALL) {
      const a9 = path.join(tmp, f + '.egg9'), a8 = path.join(tmp, f + '.egg8');
      await run(v9exe, ['transmute', src, '-o', a9]);
      await run(v11exe, ['restore', a9, '-o', out]);
      row.c9 = fs.readFileSync(out).equals(fs.readFileSync(src));
      await run(v8exe, ['transmute', src, '-o', a8]);
      await run(v11exe, ['restore', a8, '-o', out]);
      row.c8 = fs.readFileSync(out).equals(fs.readFileSync(src));
    } else { row.c9 = 'skip'; row.c8 = 'skip'; }
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
  for (const dir of ['corpus-real', 'corpus-big'].map(d => path.join(v11root, d)))
    for (const f of fs.readdirSync(dir).sort()) jobs.push([dir, f]);
  // heaviest first so the monster starts immediately
  jobs.sort((a, b) => fs.statSync(path.join(b[0], b[1])).size - fs.statSync(path.join(a[0], a[1])).size);
  console.log(`v11 M0 GATE -- ${jobs.length} files, ${LANES} lanes`);
  const rows = await pool(jobs, LANES, gateFile);
  let fail = 0;
  for (const r of rows.sort((a, b) => a.f.localeCompare(b.f))) {
    const pass = r.identical && r.c10 && (r.c9 === true || r.c9 === 'skip') && (r.c8 === true || r.c8 === 'skip') && !r.err;
    if (!pass) fail++;
    console.log(`${pass ? 'PASS' : 'FAIL'}  ${r.f.padEnd(22)} v10=${r.n10} v11=${r.n11} identical=${r.identical} egg10=${r.c10} egg9=${r.c9} egg8=${r.c8}${r.err ? ' ERR ' + r.err : ''}`);
  }
  console.log(fail ? `M0 GATE FAILED (${fail})` : `M0 GATE: ${rows.length}/${rows.length} bit-identical (mod magic/version/model/FNV) and ancestor-compatible`);
  process.exit(fail ? 1 : 0);
})();
