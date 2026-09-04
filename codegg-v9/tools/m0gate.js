#!/usr/bin/env node
// m0gate.js -- v9-M0 fork-fidelity gate, two parts:
//  (1) eggv9 must produce BIT-IDENTICAL containers to eggv8 on all 12 real
//      files, differing ONLY in the 5 magic+version bytes at each of the
//      three header sites (any other differing byte fails);
//  (2) back-compat: an .egg8 container written by eggv8 must restore EXACT
//      through eggv9 (eggv9 reads its ancestor's form).
const fs = require('fs'), path = require('path'), os = require('os');
const { execFileSync } = require('child_process');
const here = path.dirname(__filename);
const v9root = path.join(here, '..');
const v8exe = path.join(v9root, '..', 'codegg-v8', 'target', 'release', 'eggv8.exe');
const v9exe = path.join(v9root, 'target', 'release', 'eggv9.exe');
const corpus = path.join(v9root, 'corpus-real');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'm0gate9-'));
let fail = 0;
for (const f of fs.readdirSync(corpus).sort()) {
  const src = path.join(corpus, f);
  const a8 = path.join(tmp, f + '.egg8'), a9 = path.join(tmp, f + '.egg9');
  execFileSync(v8exe, ['transmute', src, '-o', a8], { stdio: 'pipe' });
  execFileSync(v9exe, ['transmute', src, '-o', a9], { stdio: 'pipe' });
  const b8 = fs.readFileSync(a8), b9 = fs.readFileSync(a9);
  const info = JSON.parse(execFileSync(v9exe, ['info', a9]).toString());
  let ok = b8.length === b9.length;
  if (ok) {
    for (const h of [info.h0, info.h1, info.h2]) {
      // magic+version (5 B) and the header FNV-32 that covers them (4 B)
      for (let i = 0; i < 5; i++) { b8[h + i] = 0; b9[h + i] = 0; }
      for (let i = 60; i < 64; i++) { b8[h + i] = 0; b9[h + i] = 0; }
    }
    ok = b8.equals(b9);
  }
  // back-compat: v9 restores the v8 container
  const out = path.join(tmp, f + '.v8.out');
  let compat = false;
  try {
    execFileSync(v9exe, ['restore', a8, '-o', out], { stdio: 'pipe' });
    compat = fs.readFileSync(out).equals(fs.readFileSync(src));
    fs.rmSync(out);
  } catch (e) { compat = false; }
  if (!ok || !compat) fail++;
  console.log(`${ok && compat ? 'PASS' : 'FAIL'}  ${f.padEnd(20)} v8=${b8.length} v9=${b9.length} B, egg8-restore ${compat ? 'EXACT' : 'BROKEN'}`);
  fs.rmSync(a8); fs.rmSync(a9);
}
fs.rmSync(tmp, { recursive: true, force: true });
console.log(fail ? `M0 GATE FAILED (${fail})` : 'M0 GATE: 12/12 bit-identical (mod magic/version) and .egg8-compatible');
process.exit(fail ? 1 : 0);
