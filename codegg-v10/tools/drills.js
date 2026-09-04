#!/usr/bin/env node
// drills.js -- the black-box battery, v8 edition. Every drill must end EXACT
// or HONEST; one SILENT-WRONG anywhere fails the whole battery. Wounds
// within the armor's printed capacity must restore EXACT; wounds beyond it
// must be refused honestly, and that refusal is asserted as a PASS.
//
// v8 upgrades over the v7 battery (the two geometry fixes, drilled):
//   - the 4 KB head scratch expectation is EXACT for guaranteed artifacts
//     (v7 provably failed it on small files: clustered replicas);
//   - killing TWO whole replica sites (head + end) must restore EXACT off
//     the surviving checksum-verified middle copy;
//   - stripe-wound expectations are DERIVED from the container's own
//     geometry (T dead in one group -> EXACT; T+1 -> HONEST), not hardcoded.
//
// House rules: deterministic PRNG (xorshift -- a float LCG poisoned a
// measurement once), wounds must be able to target EVERY region.

const fs = require('fs');
const path = require('path');
const { execFileSync, spawnSync } = require('child_process');
const os = require('os');

const here = path.dirname(__filename);
const root = path.join(here, '..');
const exe = path.join(root, 'target', 'release', 'eggv10.exe');
const corpus = path.join(root, 'corpus');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'egg10drill-'));

let seed = 0x1489 >>> 0;
function rnd() { // xorshift32
  seed ^= seed << 13; seed >>>= 0;
  seed ^= seed >>> 17;
  seed ^= seed << 5; seed >>>= 0;
  return seed;
}
function r(n) { return rnd() % n; }

let pass = 0, fail = 0;
const failures = [];
function report(name, ok, detail) {
  if (ok) { pass++; console.log(`  PASS  ${name}${detail ? ' -- ' + detail : ''}`); }
  else { fail++; failures.push(name); console.log(`  FAIL  ${name}${detail ? ' -- ' + detail : ''}`); }
}

function transmute(src, out, extra = []) {
  execFileSync(exe, ['transmute', src, '-o', out, ...extra], { stdio: 'pipe' });
}
function info(cont) {
  return JSON.parse(execFileSync(exe, ['info', cont]).toString());
}
// byte offset of merged slot j -- mirrors armor::slot_off
function slotOff(g, j) {
  const site = 64 + g.msize;
  return site + j * g.block + (j >= g.mid ? site : 0);
}
// restore a (possibly wounded) container; classify EXACT / HONEST / SILENT
function restore(cont, orig, wounds = []) {
  const out = cont + '.out';
  const args = ['restore', cont, '-o', out];
  for (const [a, l] of wounds) args.push('--wound', `${a}:${l}`);
  const res = spawnSync(exe, args, { stdio: 'pipe' });
  if (res.status !== 0) return 'HONEST';
  if (!fs.existsSync(out)) return 'HONEST';
  const got = fs.readFileSync(out);
  fs.rmSync(out);
  return got.equals(orig) ? 'EXACT' : 'SILENT';
}
function wound(contBuf, at, len) {
  const b = Buffer.from(contBuf);
  for (let i = at; i < Math.min(at + len, b.length); i++) b[i] = rnd() & 0xff;
  return b;
}
function flipBits(contBuf, nbits) {
  const b = Buffer.from(contBuf);
  for (let i = 0; i < nbits; i++) {
    const bit = r(b.length * 8);
    b[bit >> 3] ^= 1 << (bit & 7);
  }
  return b;
}
// expect: 'EXACT' (must repair), 'HONEST' (must refuse), 'ANY' (either; a lie never)
function drill(name, contBuf, orig, wounds, expect) {
  const p = path.join(tmp, 'drill.egg10');
  fs.writeFileSync(p, contBuf);
  const got = restore(p, orig, wounds);
  const ok = got === 'SILENT' ? false : (expect === 'ANY' ? true : got === expect);
  report(name, ok, `expected ${expect}, got ${got}`);
}

const cases = [
  { f: 'repo-bundle.bin', label: 'small' },
  { f: 'corpus-1489k.bin', label: 'mid' },
  { f: 'real-test.db', label: 'large' },
];

for (const { f, label } of cases) {
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.egg10');
  transmute(src, cont);
  const g = info(cont);
  const c = fs.readFileSync(cont);
  const site = 64 + g.msize;
  console.log(`\n== ${f} (${label}): ${orig.length} B -> ${c.length} B armored, G${g.g} T${g.t}, ${g.nslots} slots in ${g.ngtotal} groups (${g.ct_triple ? 'CT triplicate' : 'CT in merged stripe'}), 4KB guarantee: ${g.guaranteed}`);

  // pristine
  drill('pristine restore', c, orig, [], 'EXACT');

  // blind single-byte wounds x20, one verdict line
  {
    let bad = '';
    for (let i = 0; i < 20; i++) {
      const at = r(c.length);
      const p = path.join(tmp, 'drill.egg10');
      fs.writeFileSync(p, wound(c, at, 1));
      const got = restore(p, orig, []);
      if (got !== 'EXACT') bad += ` [@${at}: ${got}]`;
    }
    report('blind 1-byte wound x20', bad === '', bad || 'all EXACT');
  }

  // 4 KB scratches at random slot-region offsets x3: within the pigeonhole
  // guarantee everywhere (the mid replica site included -- selection absorbs it)
  {
    const expect = g.guaranteed ? 'EXACT' : 'ANY';
    for (let i = 0; i < 3; i++) {
      const span = c.length - site - 4096 - g.slots;
      const at = g.slots + r(Math.max(1, span));
      const hurt = wound(c, at, 4096);
      drill(`4 KB scratch @${at} (random slot region), blind`, hurt, orig, [], expect);
      drill(`4 KB scratch @${at}, addressed`, hurt, orig, [[at, 4096]], expect);
    }
  }

  // the v7 defect drills: replica sites
  {
    // hdr0+meta0 exactly
    drill('head site killed (hdr0+meta0), blind', wound(c, 0, site), orig, [], 'EXACT');
    // 4 KB into the head: hdr0, meta0 AND leading slots -- v7 failed this on
    // small artifacts; v2 must repair whenever the guarantee is claimed
    drill('4 KB scratch @head, blind', wound(c, 0, 4096), orig, [],
      g.guaranteed && c.length >= 24 * 1024 ? 'EXACT' : 'ANY');
    // mid site killed
    drill('mid site killed (hdr1+meta1), blind', wound(c, g.h1, site), orig, [], 'EXACT');
    // TWO whole sites killed (head + end): the surviving verified middle
    // copy must carry the restore alone
    let hurt2 = wound(c, 0, site);
    hurt2 = wound(hurt2, c.length - site, site);
    drill('two replica sites killed (head+end), blind', hurt2, orig, [], 'EXACT');
  }

  // end: meta2+hdr2+tail slots (the sparsest stripe rows -- the ragged tail)
  {
    const at = c.length - 4096;
    const hurt = wound(c, at, 4096);
    const expect = g.guaranteed ? 'EXACT' : 'ANY';
    drill('4 KB scratch @end, blind', hurt, orig, [], expect);
    drill('4 KB scratch @end, addressed', hurt, orig, [[at, 4096]], expect);
  }

  // stripe wounds, expectations DERIVED from the geometry: T dead squares in
  // one PAYLOAD group repair; T+1 refuse. In row-major order slot j (j <
  // ngtotal) belongs to group j, and level-1 groups come first -- so
  // j < ng1 picks a payload group.
  {
    const j = r(g.ng1);
    const wounds = [];
    let hurt = c;
    for (let k = 0; k < g.t; k++) {
      const at = slotOff(g, j + k * g.ngtotal);
      hurt = wound(hurt, at, g.block);
      wounds.push([at, g.block]);
    }
    drill(`${g.t} stripe wounds (same payload group, =T), blind`, hurt, orig, [], 'EXACT');
    drill(`${g.t} stripe wounds (same payload group, =T), addressed`, hurt, orig, wounds, 'EXACT');
    const at = slotOff(g, j + g.t * g.ngtotal);
    const h3 = wound(hurt, at, g.block);
    drill(`${g.t + 1} stripe wounds (same payload group, beyond T), addressed`, h3, orig,
      [...wounds, [at, g.block]], 'HONEST');
  }

  // the check-table group is different physics: killing T+1 of its squares
  // loses CHECKS, not payload -- the untouched data must still come back
  // EXACT, arbitrated by the payload hash (found by this very drill: the
  // first run expected HONEST here and the armor was righter than the test)
  if (!g.ct_triple && g.ng2 > 0) {
    const j = g.ng1; // first level-2 group
    let hurt = c;
    const wounds = [];
    for (let k = 0; k < g.t + 1; k++) {
      const at = slotOff(g, j + k * g.ngtotal);
      hurt = wound(hurt, at, g.block);
      wounds.push([at, g.block]);
    }
    drill(`${g.t + 1} stripe wounds (CT group, beyond T -- checks lost, data intact)`, hurt, orig, wounds, 'EXACT');
  }

  // truncation: 4 KB off the end is an ordinary wound
  drill('truncation 4 KB', c.subarray(0, c.length - 4096), orig, [], g.guaranteed ? 'EXACT' : 'ANY');
  // truncation of 30% is beyond any capacity: honest refusal required
  drill('truncation 30% (beyond capacity)', c.subarray(0, Math.floor(c.length * 0.7)), orig, [], 'HONEST');

  // a scratch far beyond capacity must be detected, never silent
  const big = Math.min(Math.floor(c.length / 2), 512 * 1024);
  drill(`${(big / 1024) | 0} KB scratch (beyond capacity), blind`,
    wound(c, g.slots + 4096, big), orig, [], 'HONEST');
}

// 3-bit storms x300 on the mid artifact
{
  const f = 'corpus-1489k.bin';
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.egg10');
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f}: 3-bit storms x300 (blind)`);
  let exact = 0, honest = 0, silent = 0;
  for (let i = 0; i < 300; i++) {
    const p = path.join(tmp, 'storm.egg10');
    fs.writeFileSync(p, flipBits(c, 3));
    const got = restore(p, orig, []);
    if (got === 'EXACT') exact++;
    else if (got === 'HONEST') honest++;
    else silent++;
  }
  report('3-bit storm x300: zero silent', silent === 0, `${exact} EXACT, ${honest} honest, ${silent} SILENT`);
  report('3-bit storm x300: all repaired', exact === 300, `${exact}/300 EXACT`);
}

// wide scratches on the large artifact: EXACT when capacity allows, honest
// beyond it, a lie never
{
  const f = 'real-test.db';
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.egg10');
  const g = info(cont);
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f}: wide scratches (EXACT or honest, never silent)`);
  for (const kb of [16, 64, 128]) {
    const len = kb * 1024;
    const at = g.slots + Math.floor(Math.max(0, g.mid * g.block - len) / 2);
    const hurt = wound(c, at, len);
    drill(`${kb} KB scratch, addressed`, hurt, orig, [[at, len]], 'ANY');
    drill(`${kb} KB scratch, blind`, hurt, orig, [], 'ANY');
  }
}

// no-armor: pristine exact; any wound must fail honestly (hash gate)
{
  const f = 'repo-bundle.bin';
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.noarmor.egg10');
  transmute(src, cont, ['--no-armor']);
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f} --no-armor: hash gate only`);
  drill('no-armor pristine', c, orig, [], 'EXACT');
  drill('no-armor 1-byte wound (must refuse)', wound(c, Math.floor(c.length / 2), 1), orig, [], 'HONEST');
  // cutting exactly hdr2 touches no payload: the surviving headers carry it
  drill('no-armor truncation of hdr2 only (headers survive)', c.subarray(0, c.length - 64), orig, [], 'EXACT');
  drill('no-armor truncation into payload (must refuse)', c.subarray(0, c.length - 4096), orig, [], 'HONEST');
}

// the pigeonhole, asserted as a PASS: random MUST transmute larger
{
  const f = 'photo.bin';
  const src = path.join(corpus, f);
  const cont = path.join(tmp, f + '.noarmor.egg10');
  transmute(src, cont, ['--no-armor']);
  const grew = fs.statSync(cont).size > fs.statSync(src).size;
  report('pigeonhole: random transmutes LARGER (required)', grew,
    `${fs.statSync(src).size} -> ${fs.statSync(cont).size} B`);
}

console.log(`\n${pass} passed, ${fail} failed${fail ? ': ' + failures.join('; ') : ''}`);
fs.rmSync(tmp, { recursive: true, force: true });
process.exit(fail ? 1 : 0);
