#!/usr/bin/env node
// m0gate.js -- M0 fork-fidelity gate: eggv8 must produce BIT-IDENTICAL
// containers to eggv7 on all 12 real files, differing ONLY in the 5
// magic+version bytes at each of the three header offsets. Any other
// differing byte fails the gate.
const fs = require('fs'), path = require('path'), os = require('os');
const { execFileSync } = require('child_process');
const here = path.dirname(__filename);
const v8root = path.join(here, '..');
const v7exe = path.join(v8root, '..', 'codegg-v7', 'target', 'release', 'eggv7.exe');
const v8exe = path.join(v8root, 'target', 'release', 'eggv8.exe');
const corpus = path.join(v8root, 'corpus-real');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'm0gate-'));
let fail = 0;
for (const f of fs.readdirSync(corpus).sort()) {
  const src = path.join(corpus, f);
  const a7 = path.join(tmp, f + '.egg7'), a8 = path.join(tmp, f + '.egg8');
  execFileSync(v7exe, ['transmute', src, '-o', a7], { stdio: 'pipe' });
  execFileSync(v8exe, ['transmute', src, '-o', a8], { stdio: 'pipe' });
  const b7 = fs.readFileSync(a7), b8 = fs.readFileSync(a8);
  const info = JSON.parse(execFileSync(v8exe, ['info', a8]).toString());
  let ok = b7.length === b8.length;
  if (ok) {
    for (const h of [info.h0, info.h1, info.h2])
      for (let i = 0; i < 5; i++) { b7[h + i] = 0; b8[h + i] = 0; }
    ok = b7.equals(b8);
  }
  if (!ok) fail++;
  console.log(`${ok ? 'PASS' : 'FAIL'}  ${f.padEnd(20)} v7=${b7.length} v8=${b8.length} bytes`);
  fs.rmSync(a7); fs.rmSync(a8);
}
fs.rmSync(tmp, { recursive: true, force: true });
console.log(fail ? `M0 GATE FAILED (${fail})` : 'M0 GATE: all 12 bit-identical (mod magic/version)');
process.exit(fail ? 1 : 0);
