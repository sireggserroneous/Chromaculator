#!/usr/bin/env node
// mkcorpus8.js -- deterministic corpus for the v8 tournament.
// Seeds are FIXED so results stay comparable across regenerations; the only
// non-deterministic member is photo.bin (crypto-random by design: it is the
// pigeonhole witness, and any random bytes witness equally well) and
// program.exe (the then-current eggv8 build; a real PE either way).
// LCG per the plan: s = Math.imul(s, 1103515245) + 12345 | 0; take (s >>> 16).

const fs = require('fs');
const path = require('path');
const { execFileSync } = require('child_process');
const crypto = require('crypto');

const here = path.dirname(__filename);
const root = path.join(here, '..');          // codegg-v8/
const repo = path.join(root, '..');          // Chromaculator/
const out = path.join(root, 'corpus');
fs.mkdirSync(out, { recursive: true });

function lcg(seed) {
  let s = seed | 0;
  return function r(n) {
    s = (Math.imul(s, 1103515245) + 12345) | 0;
    return (s >>> 16) % n;
  };
}
function hex16(r) { // 8-byte hex req id, LCG-derived: deterministic, right class
  let h = '';
  for (let i = 0; i < 16; i++) h += '0123456789abcdef'[r(16)];
  return h;
}

// 1. server-log.json -- JSON lines, seed 12345
{
  const r = lcg(12345);
  const levels = ['INFO', 'WARN', 'ERROR', 'DEBUG'];
  const svcs = ['auth', 'billing', 'cart', 'search', 'gateway'];
  const lines = [];
  let ts = 1725000000000;
  for (let i = 0; i < 40000; i++) {
    const svc = svcs[r(5)];
    lines.push(JSON.stringify({
      ts: new Date(ts).toISOString(),
      level: levels[r(4)],
      svc,
      req: hex16(r),
      user: 'u' + r(9999),
      ms: r(2000),
      msg: 'request completed with status ' + (r(10) < 8 ? 200 : 500),
      path: '/api/v2/' + svc + '/' + r(100),
    }));
    ts += 1731;
  }
  // pad line count upward until ~7.3 MB by repeating the generator
  while (Buffer.byteLength(lines.join('\n')) < 7.2 * 1024 * 1024) {
    const svc = svcs[r(5)];
    lines.push(JSON.stringify({
      ts: new Date(ts).toISOString(),
      level: levels[r(4)],
      svc,
      req: hex16(r),
      user: 'u' + r(9999),
      ms: r(2000),
      msg: 'request completed with status ' + (r(10) < 8 ? 200 : 500),
      path: '/api/v2/' + svc + '/' + r(100),
    }));
    ts += 1731;
  }
  fs.writeFileSync(path.join(out, 'server-log.json'), lines.join('\n') + '\n');
}

// 2. data.csv -- 120,000 rows, seed 12345 (fresh LCG)
{
  const r = lcg(12345);
  const wh = ['EWR', 'JFK', 'LGA', 'ORD', 'DFW'];
  const rows = ['id,sku,warehouse,qty,price,updated'];
  let day = 1725000000000;
  for (let i = 0; i < 120000; i++) {
    const price = ((r(9999) + 1) / 100).toFixed(2);
    const date = new Date(day + r(86400000)).toISOString().slice(0, 10);
    rows.push(`${60000 + i},SKU-${10000 + r(89999)},${wh[r(5)]},${r(500)},${price},${date}`);
  }
  fs.writeFileSync(path.join(out, 'data.csv'), rows.join('\n') + '\n');
}

// 3. big.xml -- 30,000 elements, seed 99
{
  const r = lcg(99);
  const tags = ['order', 'customer', 'item', 'shipment', 'invoice'];
  const status = ['pending', 'shipped', 'delivered', 'returned', 'cancelled'];
  const parts = ['<?xml version="1.0" encoding="UTF-8"?>', '<ledger>'];
  for (let i = 0; i < 30000; i++) {
    const tag = tags[r(5)];
    const total = ((r(99999) + 1) / 100).toFixed(2);
    parts.push(
      `  <${tag} id="${100000 + i}" status="${status[r(5)]}" region="${['us-east', 'us-west', 'eu-central'][r(3)]}" total="${total}">` +
      `<note>ref ${r(9999)} handled by agent ${r(500)}</note>` +
      `<qty>${r(144)}</qty></${tag}>`
    );
  }
  parts.push('</ledger>');
  fs.writeFileSync(path.join(out, 'big.xml'), parts.join('\n') + '\n');
}

// 4. program.exe -- the then-current eggv8 build, a real PE
fs.copyFileSync(path.join(root, 'target', 'release', 'eggv8.exe'), path.join(out, 'program.exe'));

// 5. repo-bundle.bin -- the site's own files, concatenated
{
  const files = ['spec.md', 'README.md', 'stalk.js', 'chroma-ui.js', 'index.html', 'wubbadub.html', 'yinyang.svg'];
  fs.writeFileSync(path.join(out, 'repo-bundle.bin'),
    Buffer.concat(files.map(f => fs.readFileSync(path.join(repo, f)))));
}

// 6. photo.bin -- 4 MiB crypto-random: the pigeonhole witness
fs.writeFileSync(path.join(out, 'photo.bin'), crypto.randomBytes(4 * 1024 * 1024));

// 7. archive.zst -- zstd -19 of server-log.json (pre-compressed member)
{
  const zstd = 'C:\\Users\\vcepe\\AppData\\Local\\Microsoft\\WinGet\\Packages\\Meta.Zstandard_Microsoft.Winget.Source_8wekyb3d8bbwe\\zstd-v1.5.7-win64\\zstd.exe';
  execFileSync(zstd, ['-19', '-f', path.join(out, 'server-log.json'), '-o', path.join(out, 'archive.zst')], { stdio: 'ignore' });
}

// 8-10. real files, already on disk
fs.copyFileSync(path.join(repo, 'codegg-v4', 'real-test.db'), path.join(out, 'real-test.db'));
fs.copyFileSync(path.join(repo, 'codegg-v4', 'real-test.bmp'), path.join(out, 'real-test.bmp'));
fs.copyFileSync(path.join(repo, 'codegg-v2', 'corpus-1489k.bin'), path.join(out, 'corpus-1489k.bin'));

for (const f of fs.readdirSync(out).sort()) {
  const st = fs.statSync(path.join(out, f));
  console.log(`${String(st.size).padStart(10)}  ${f}`);
}
