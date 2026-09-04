#!/usr/bin/env node
// drills.js -- the M3 battery. Every drill must end EXACT or HONEST;
// one SILENT-WRONG anywhere fails the whole battery. Wounds within the
// armor's printed capacity must restore EXACT; wounds beyond it must be
// refused honestly, and that refusal is asserted as a PASS.
//
// House rules: deterministic PRNG (xorshift -- a float LCG poisoned a
// measurement once), wounds must be able to target EVERY region (v5.0's
// harness couldn't wound its own head and shipped the naked-head defect).

const fs = require('fs');
const path = require('path');
const { execFileSync, spawnSync } = require('child_process');
const os = require('os');

const here = path.dirname(__filename);
const root = path.join(here, '..');
const exe = path.join(root, 'target', 'release', 'eggv7.exe');
const corpus = path.join(root, 'corpus');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'egg7drill-'));

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
  const p = path.join(tmp, 'drill.egg7');
  fs.writeFileSync(p, contBuf);
  const got = restore(p, orig, wounds);
  const ok = got === 'SILENT' ? false : (expect === 'ANY' ? true : got === expect);
  report(name, ok, `expected ${expect}, got ${got}`);
}

const cases = [
  { f: 'repo-bundle.bin', label: 'small (G8 rib)' },
  { f: 'corpus-1489k.bin', label: 'mid (G32 rib)' },
  { f: 'real-test.db', label: 'large (G126 rib)' },
];

for (const { f, label } of cases) {
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.egg7');
  transmute(src, cont);
  const g = info(cont);
  const c = fs.readFileSync(cont);
  const ng = Math.ceil(g.s / g.g);
  // level-2 data squares and group count, derived the same way geom does
  const ct = 4 * g.nsq;
  const c2 = Math.ceil(ct / g.block);
  const ngl2 = Math.ceil(c2 / g.g);
  console.log(`\n== ${f} (${label}): ${orig.length} B -> ${c.length} B armored, G${g.g} T${g.t}, ${g.nsq} slots in ${ng} groups; CT ${g.nsq2} slots in ${ngl2} groups`);

  // pristine
  drill('pristine restore', c, orig, [], 'EXACT');

  // blind single-byte wounds x20, one verdict line
  {
    let bad = '';
    for (let i = 0; i < 20; i++) {
      const at = r(c.length);
      const p = path.join(tmp, 'drill.egg7');
      fs.writeFileSync(p, wound(c, at, 1));
      const got = restore(p, orig, []);
      if (got !== 'EXACT') bad += ` [@${at}: ${got}]`;
    }
    report('blind 1-byte wound x20', bad === '', bad || 'all EXACT');
  }

  // payload 4 KB scratch: within stripe capacity everywhere -> EXACT
  {
    const at = g.slots + Math.floor((g.nsq * g.block - 4096) / 2);
    const hurt = wound(c, at, 4096);
    drill('4 KB scratch @payload, blind', hurt, orig, [], 'EXACT');
    drill('4 KB scratch @payload, addressed', hurt, orig, [[at, 4096]], 'EXACT');
  }
  // checks region: a capacity-sized wound must repair EXACT; a full 4 KB
  // wound may exceed the level-2 group budget (T per group) -- honest then
  {
    const capLen = Math.min(4096, g.t * ngl2 * g.block);
    const atA = g.slots2 + g.block * Math.max(0, Math.floor((g.nsq2 - capLen / g.block) / 2));
    const hurtA = wound(c, atA, capLen);
    drill(`${capLen} B scratch @checks (capacity-sized), blind`, hurtA, orig, [], 'EXACT');
    drill(`${capLen} B scratch @checks (capacity-sized), addressed`, hurtA, orig, [[atA, capLen]], 'EXACT');
    const atB = g.slots2;
    const lenB = Math.min(4096, g.nsq2 * g.block);
    drill(`${lenB} B scratch @checks (may exceed CT budget)`, wound(c, atB, lenB), orig, [], 'ANY');
  }
  // head: killing hdr0+meta0 exactly must vote through; 4 KB into the head
  // also eats check-table squares and may exceed their budget -- honest then
  {
    const m = 4 * g.nsq2;
    const hurt = wound(c, 0, 64 + m);
    drill('head wound (hdr0+meta0), blind', hurt, orig, [], 'EXACT');
    drill('4 KB scratch @head (hdr0+meta0+CT)', wound(c, 0, 4096), orig, [], 'ANY');
  }
  // end: hdr2+meta2+tail slots; tail slots are stripe-consecutive -> EXACT
  {
    const at = c.length - 4096;
    const hurt = wound(c, at, 4096);
    drill('4 KB scratch @end, blind', hurt, orig, [], 'EXACT');
    drill('4 KB scratch @end, addressed', hurt, orig, [[at, 4096]], 'EXACT');
  }

  // twin stripe wound: two dead squares in ONE group (v5.2's caveat; T=2)
  {
    const j = r(ng);
    const a1 = g.slots + j * g.block;
    const a2 = g.slots + (j + ng) * g.block;
    let hurt = wound(c, a1, g.block);
    hurt = wound(hurt, a2, g.block);
    drill('twin stripe wound (same group), blind', hurt, orig, [], 'EXACT');
    drill('twin stripe wound (same group), addressed', hurt, orig, [[a1, g.block], [a2, g.block]], 'EXACT');
    const a3 = g.slots + (j + 2 * ng) * g.block;
    const h3 = wound(hurt, a3, g.block);
    drill('triple stripe wound (beyond T=2), addressed', h3, orig,
      [[a1, g.block], [a2, g.block], [a3, g.block]], 'HONEST');
  }

  // truncation: 4 KB off the end is an ordinary wound
  drill('truncation 4 KB', c.subarray(0, c.length - 4096), orig, [], 'EXACT');
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
  const cont = path.join(tmp, f + '.egg7');
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f}: 3-bit storms x300 (blind)`);
  let exact = 0, honest = 0, silent = 0;
  for (let i = 0; i < 300; i++) {
    const p = path.join(tmp, 'storm.egg7');
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
  const cont = path.join(tmp, f + '.egg7');
  const g = info(cont);
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f}: wide scratches (EXACT or honest, never silent)`);
  for (const kb of [16, 64, 128]) {
    const len = kb * 1024;
    const at = g.slots + Math.floor((g.nsq * g.block - len) / 2);
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
  const cont = path.join(tmp, f + '.noarmor.egg7');
  transmute(src, cont, ['--no-armor']);
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f} --no-armor: hash gate only`);
  drill('no-armor pristine', c, orig, [], 'EXACT');
  drill('no-armor 1-byte wound (must refuse)', wound(c, Math.floor(c.length / 2), 1), orig, [], 'HONEST');
  // cutting exactly hdr2 touches no payload: the vote absorbs it (first run
  // of this drill expected HONEST and was wrong -- the data really is intact)
  drill('no-armor truncation of hdr2 only (vote survives)', c.subarray(0, c.length - 64), orig, [], 'EXACT');
  drill('no-armor truncation into payload (must refuse)', c.subarray(0, c.length - 4096), orig, [], 'HONEST');
}

// the pigeonhole, asserted as a PASS: random MUST transmute larger
{
  const f = 'photo.bin';
  const src = path.join(corpus, f);
  const cont = path.join(tmp, f + '.noarmor.egg7');
  transmute(src, cont, ['--no-armor']);
  const grew = fs.statSync(cont).size > fs.statSync(src).size;
  report('pigeonhole: random transmutes LARGER (required)', grew,
    `${fs.statSync(src).size} -> ${fs.statSync(cont).size} B`);
}

console.log(`\n${pass} passed, ${fail} failed${fail ? ': ' + failures.join('; ') : ''}`);
fs.rmSync(tmp, { recursive: true, force: true });
process.exit(fail ? 1 : 0);
