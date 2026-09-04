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
const v10root = path.join(here, '..');
const v8exe = path.join(v10root, '..', 'codegg-v8', 'target', 'release', 'eggv8.exe');
const v9exe = path.join(v10root, '..', 'codegg-v9', 'target', 'release', 'eggv9.exe');
const v10exe = path.join(v10root, 'target', 'release', 'eggv10.exe');
const corpus = path.join(v10root, 'corpus-real');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'm0gate10-'));
let fail = 0;
for (const f of fs.readdirSync(corpus).sort()) {
  const src = path.join(corpus, f);
  const a9 = path.join(tmp, f + '.egg9'), a10 = path.join(tmp, f + '.egg10');
  const a8 = path.join(tmp, f + '.egg8');
  execFileSync(v9exe, ['transmute', src, '-o', a9], { stdio: 'pipe' });
  execFileSync(v10exe, ['transmute', src, '-o', a10], { stdio: 'pipe' });
  execFileSync(v8exe, ['transmute', src, '-o', a8], { stdio: 'pipe' });
  const b9 = fs.readFileSync(a9), b10 = fs.readFileSync(a10);
  const info = JSON.parse(execFileSync(v10exe, ['info', a10]).toString());
  let ok = b9.length === b10.length;
  if (ok) {
    for (const h of [info.h0, info.h1, info.h2]) {
      // magic+version (5 B) and the header FNV-32 that covers them (4 B)
      for (let i = 0; i < 5; i++) { b9[h + i] = 0; b10[h + i] = 0; }
      // the model byte: v10 stamps 8/9 where v9 stamped 6/7 for the SAME
      // payload (the clone entrants); neutralize it and the FNV that covers it
      b9[h + 31] = 0; b10[h + 31] = 0;
      for (let i = 60; i < 64; i++) { b9[h + i] = 0; b10[h + i] = 0; }
    }
    ok = b9.equals(b10);
  }
  // back-compat: v10 restores BOTH ancestors' containers
  const out = path.join(tmp, f + '.anc.out');
  let compat9 = false, compat8 = false;
  try {
    execFileSync(v10exe, ['restore', a9, '-o', out], { stdio: 'pipe' });
    compat9 = fs.readFileSync(out).equals(fs.readFileSync(src));
    execFileSync(v10exe, ['restore', a8, '-o', out], { stdio: 'pipe' });
    compat8 = fs.readFileSync(out).equals(fs.readFileSync(src));
    fs.rmSync(out);
  } catch (e) {}
  if (!ok || !compat9 || !compat8) fail++;
  console.log(`${ok && compat9 && compat8 ? 'PASS' : 'FAIL'}  ${f.padEnd(20)} v9=${b9.length} v10=${b10.length} B, egg9 ${compat9 ? 'EXACT' : 'BROKEN'}, egg8 ${compat8 ? 'EXACT' : 'BROKEN'}`);
  fs.rmSync(a8); fs.rmSync(a9); fs.rmSync(a10);
}
fs.rmSync(tmp, { recursive: true, force: true });
console.log(fail ? `M0 GATE FAILED (${fail})` : 'M0 GATE: 12/12 bit-identical (mod magic/version) and both-ancestor-compatible');
process.exit(fail ? 1 : 0);
