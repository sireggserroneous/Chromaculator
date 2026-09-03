#!/usr/bin/env node
// drills.js -- the black-box battery, v12 edition (armor v4: ONE GF(2^16)
// codeword per file). Every drill must end EXACT or HONEST; one SILENT-WRONG
// anywhere fails the whole battery. Wounds within the armor's printed promise
// must restore EXACT; wounds beyond it must be refused honestly, and that
// refusal is asserted as a PASS.
//
// v12 upgrades over the v11 battery (the geometry changed under it):
//   - there are no groups: ANY t whole squares anywhere are one erasure
//     pattern of the single codeword, so the scattered-square drills pick
//     squares uniformly over data+parity (+CT in mode B);
//   - mode B (CT in-codeword) has a qualified BLIND promise: a dead CT
//     square hides blk/2 residues. The expectation is DERIVED here from the
//     wound set exactly as the promise states it -- k dead unjudged data
//     squares vs m = t - |convicted|: EXACT when k < m or k = m <= 2,
//     otherwise an honest REFUSE (and the same wound NAMED must be EXACT);
//   - mode A (--ct triple) must restore any t squares blind, CT included;
//   - M2b, placement NONE (the default argmin since M2b): no residue table;
//     P = t+1 parity squares; the interleaved codewords locate blind wounds
//     jointly (Krachkovsky-Lee 1997). Blind e <= P-1 squares -> EXACT, blind
//     P -> HONEST, named <= P -> EXACT. Every M1 drill runs under the default
//     (none) AND under --judge (the residue placements), expectations derived
//     per placement. Rung C (any placement): dead squares that are all
//     parity/CT leave the data intact and the FNV-64 of the inner says so ->
//     EXACT (the v8 lesson: the armor is righter than a drill that expects
//     a refusal there);
//   - --survive 65536 must survive a 64 KB contiguous scratch, blind;
//   - the short data square rides at stream position 0 (audit (a) found the
//     wound bound broke with it mid-stream); squareOff mirrors armor::square_off;
//   - every ancestor container (.egg8..egg11) must restore EXACT through
//     eggv13, wounded head included.
//
// House rules: deterministic PRNG (xorshift -- a float LCG poisoned a
// measurement once), wounds must be able to target EVERY region, scratch
// files live outside the corpus dirs.

const fs = require('fs');
const path = require('path');
const { execFileSync, spawnSync } = require('child_process');
const os = require('os');

const here = path.dirname(__filename);
const root = path.join(here, '..');
const exe = process.env.EGG_EXE || path.join(root, 'target', 'release', 'eggv13.exe');
const corpus = path.join(root, 'corpus');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'egg13drill-'));

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
// ---- geometry mirror (armor::stream_pos / square_off / square_len) ----
// square index order: data 0..s-1 | parity s..s+t-1 | CT s+t..n-1
// stream order: the short data square (index s-1) rides first, then data
// 0..s-2, then parity and CT at their indices; the mid site precedes stream
// position `mid`.
function streamPos(g, j) {
  if (g.s === 0 || j >= g.s) return j;
  return j === g.s - 1 ? 0 : j + 1;
}
function squareOff(g, j) {
  const site = 64 + g.msize;
  const p = streamPos(g, j);
  return site + p * g.block - (p >= 1 ? g.pad : 0) + (p >= g.mid ? site : 0);
}
function squareLen(g, j) {
  return (g.s > 0 && streamPos(g, j) === 0) ? g.block - g.pad : g.block;
}
const isData = (g, j) => j < g.s;
const isCt = (g, j) => j >= g.s + g.t;
const inCodeword = (g) => g.mode === 'CT in-codeword';
const isNone = (g) => g.mode === 'CT none';
// which CT square (absolute index) holds data square i's residue (mode B)
function ctOf(g, i) { return g.s + g.t + Math.floor(i / (g.block / 2)); }
// the promise, derived for a set of dead squares, BLIND:
//   any placement: all dead squares parity/CT -> the data is intact, rung C -> EXACT;
//   beyond t -> HONEST;
//   none: e <= t-1 located jointly -> EXACT, e = t -> HONEST;
//   triple: EXACT;
//   in-codeword: convicted = dead squares whose residue survives (parity/CT via
//   meta, data whose CT square is alive); k = dead data squares whose CT died;
//   m = t - |convicted|; EXACT iff k < m or k = m <= 2, else HONEST.
function blindExpect(g, dead) {
  if (dead.every(j => !isData(g, j))) return 'EXACT';
  if (dead.length > g.t) return 'HONEST';
  if (isNone(g)) return dead.length < g.t ? 'EXACT' : 'HONEST';
  if (!inCodeword(g)) return 'EXACT';
  const set = new Set(dead);
  let convicted = 0, k = 0;
  for (const j of dead) {
    if (!isData(g, j)) convicted++;
    else if (set.has(ctOf(g, j))) k++;
    else convicted++;
  }
  const m = g.t - convicted;
  return (k < m || (k === m && k <= 2)) ? 'EXACT' : 'HONEST';
}
// NAMED: erasures <= t -> EXACT; beyond -> HONEST unless every dead square is parity/CT (rung C)
function namedExpect(g, dead) {
  if (dead.length <= g.t) return 'EXACT';
  return dead.every(j => !isData(g, j)) ? 'EXACT' : 'HONEST';
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
// kill whole squares (by index); returns [buffer, wounds]
function killSquares(g, c, idx) {
  let hurt = c;
  const wounds = [];
  for (const j of idx) {
    const at = squareOff(g, j), len = squareLen(g, j);
    hurt = wound(hurt, at, len);
    wounds.push([at, len]);
  }
  return [hurt, wounds];
}
function pickDistinct(count, n, avoid = new Set()) {
  const out = new Set();
  let guard = 0;
  while (out.size < count && guard++ < 100000) {
    const j = r(n);
    if (!avoid.has(j)) out.add(j);
  }
  return [...out];
}
// expect: 'EXACT' (must repair), 'HONEST' (must refuse), 'ANY' (either; a lie never)
function drill(name, contBuf, orig, wounds, expect) {
  const p = path.join(tmp, 'drill.egg13');
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

// ---- the geometry mirror, asserted against info before anything else ----
function checkMirror(g, c, name) {
  const site = 64 + g.msize;
  const okSlots = g.slots === site;
  // parity_at/ct_at in info are byte offsets of the first parity / CT square
  const okP = squareOff(g, g.s) === g.parity_at;
  const okC = g.c === 0 || squareOff(g, g.s + g.t) === g.ct_at;
  // the end site follows the last square
  const last = g.n - 1;
  const okEnd = squareOff(g, last) + squareLen(g, last) === g.h2 - g.msize;
  const okTotal = g.h2 + 64 === c.length && g.total === c.length;
  report(`${name}: geometry mirror (slots, parity_at, ct_at, end site, total)`,
    okSlots && okP && okC && okEnd && okTotal,
    `slots ${g.slots} parity@${g.parity_at} ct@${g.ct_at} h2 ${g.h2} total ${g.total}`);
}

// the M1 battery on the three cases, under one placement policy
// (extra = [] is the default argmin, placement none since M2b; ['--judge'] the residue placements)
const kept = {}; // f -> {cont, g, c, orig} of the DEFAULT placement (reused below)
function battery(tag, extra) {
  for (const { f, label } of cases) {
    const src = path.join(corpus, f);
    const orig = fs.readFileSync(src);
    const cont = path.join(tmp, f + tag + '.egg13');
    transmute(src, cont, extra);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    const site = 64 + g.msize;
    console.log(`\n== ${f} (${label})${tag}: ${orig.length} B -> ${c.length} B armored, ONE codeword n=${g.n} (${g.s} data + ${g.t} parity + ${g.c} CT) x ${g.block} B, ${g.mode}, price ${g.price} (${Number(g.floor_x).toFixed(2)}x floor), 4KB guarantee: ${g.guaranteed}`);
    if (extra.length === 0) {
      kept[f] = { cont, g, c, orig };
      report('default placement is none (the argmin)', isNone(g), g.mode);
    } else {
      report(`${tag}: a residue placement was chosen`, !isNone(g), g.mode);
    }
    checkMirror(g, c, 'pristine');

    // pristine
    drill('pristine restore', c, orig, [], 'EXACT');

    // blind single-byte wounds x20, one verdict line
    {
      let bad = '';
      for (let i = 0; i < 20; i++) {
        const at = r(c.length);
        const p = path.join(tmp, 'drill.egg13');
        fs.writeFileSync(p, wound(c, at, 1));
        const got = restore(p, orig, []);
        if (got !== 'EXACT') bad += ` [@${at}: ${got}]`;
      }
      report('blind 1-byte wound x20', bad === '', bad || 'all EXACT');
    }

    // 4 KB scratches at random square-region offsets x3: within the guarantee
    // everywhere (the mid site included -- a site costs the wound nothing)
    {
      const expect = g.guaranteed ? 'EXACT' : 'ANY';
      for (let i = 0; i < 3; i++) {
        const span = c.length - site - 4096 - g.slots;
        const at = g.slots + r(Math.max(1, span));
        const hurt = wound(c, at, 4096);
        drill(`4 KB scratch @${at} (random square region), blind`, hurt, orig, [], expect);
        drill(`4 KB scratch @${at}, addressed`, hurt, orig, [[at, 4096]], expect);
      }
    }

    // sites
    {
      drill('head site killed (hdr0+meta0), blind', wound(c, 0, site), orig, [], 'EXACT');
      // 4 KB into the head: hdr0, meta0, the short square AND leading squares
      drill('4 KB scratch @head, blind', wound(c, 0, 4096), orig, [], g.guaranteed ? 'EXACT' : 'ANY');
      drill('mid site killed (hdr1+meta1), blind', wound(c, g.h1, site), orig, [], 'EXACT');
      // 4 KB straddling the mid site (site + squares either side)
      drill('4 KB scratch straddling mid site, blind', wound(c, g.h1 - 1024, 4096), orig, [], g.guaranteed ? 'EXACT' : 'ANY');
      let hurt2 = wound(c, 0, site);
      hurt2 = wound(hurt2, c.length - site, site);
      drill('two sites killed (head+end), blind', hurt2, orig, [], 'EXACT');
    }

    // end: meta2+hdr2+tail squares (CT and the last parity squares)
    {
      const at = c.length - 4096;
      const hurt = wound(c, at, 4096);
      const expect = g.guaranteed ? 'EXACT' : 'ANY';
      drill('4 KB scratch @end, blind', hurt, orig, [], expect);
      drill('4 KB scratch @end, addressed', hurt, orig, [[at, 4096]], expect);
    }

    // scattered whole squares: t distinct squares anywhere in the codeword
    // (data, parity, CT alike). NAMED must be EXACT; BLIND per the promise.
    {
      const idx = pickDistinct(g.t, g.n);
      const [hurt, wounds] = killSquares(g, c, idx);
      const exp = blindExpect(g, idx);
      drill(`${g.t} scattered squares (=t, uniform over n), named`, hurt, orig, wounds, 'EXACT');
      drill(`${g.t} scattered squares (=t), blind (promise says ${exp})`, hurt, orig, [], exp);
      // t-1 scattered: blind EXACT in every placement (in-codeword: "any t-1
      // scattered"; none: located jointly)
      const idx1 = idx.slice(0, g.t - 1);
      const [hurt1] = killSquares(g, c, idx1);
      drill(`${g.t - 1} scattered squares (=t-1), blind`, hurt1, orig, [], blindExpect(g, idx1));
      // t+1: honest refusal, named or blind (unless every dead square is parity/CT: rung C)
      const extra1 = pickDistinct(1, g.n, new Set(idx))[0];
      const all = [...idx, extra1];
      const [hurt3, wounds3] = killSquares(g, hurt, [extra1]);
      drill(`${g.t + 1} scattered squares (beyond t), named`, hurt3, orig, [...wounds, ...wounds3], namedExpect(g, all));
      drill(`${g.t + 1} scattered squares (beyond t), blind`, hurt3, orig, [], blindExpect(g, all));
    }
    // the short data square (index s-1, stream position 0) plus t-1 others
    {
      const others = pickDistinct(g.t - 1, g.n, new Set([g.s - 1]));
      const idx = [g.s - 1, ...others];
      const [hurt, wounds] = killSquares(g, c, idx);
      const exp = blindExpect(g, idx);
      drill(`short square + ${g.t - 1} others (=t), named`, hurt, orig, wounds, 'EXACT');
      drill(`short square + ${g.t - 1} others (=t), blind (promise says ${exp})`, hurt, orig, [], exp);
    }
    // all t parity squares dead (the data must come back untouched, verified)
    {
      const idx = [];
      for (let k = 0; k < g.t; k++) idx.push(g.s + k);
      const [hurt, wounds] = killSquares(g, c, idx);
      drill('all parity squares dead, blind', hurt, orig, [], 'EXACT');
      drill('all parity squares dead, named', hurt, orig, wounds, 'EXACT');
    }
    // mode B physics: a CT square and the data squares it covers
    if (inCodeword(g)) {
      const ct = g.s + g.t; // first CT square
      const covered = [];
      for (let i = 0; i < g.s && covered.length < g.t; i++) if (ctOf(g, i) === ct) covered.push(i);
      // CT alone: its residues are gone, the data is intact -> EXACT
      {
        const [hurt] = killSquares(g, c, [ct]);
        drill('CT square dead alone (residues lost, data intact), blind', hurt, orig, [], 'EXACT');
      }
      // CT + t-2 of its data squares: k = t-2 < m = t-1 -> collaborative, EXACT
      if (g.t >= 2) {
        const idx = [ct, ...covered.slice(0, g.t - 2)];
        const [hurt, wounds] = killSquares(g, c, idx);
        const exp = blindExpect(g, idx);
        drill(`CT + ${g.t - 2} covered data (k=${g.t - 2} < m=${g.t - 1}), blind (promise says ${exp})`, hurt, orig, [], exp);
        drill(`CT + ${g.t - 2} covered data, named`, hurt, orig, wounds, 'EXACT');
      }
      // CT + t-1 of its data squares: k = m = t-1 -> EXACT iff t-1 <= 2, else an
      // honest REFUSE (and named EXACT)
      {
        const idx = [ct, ...covered.slice(0, g.t - 1)];
        const [hurt, wounds] = killSquares(g, c, idx);
        const exp = blindExpect(g, idx);
        drill(`CT + ${g.t - 1} covered data (k=m=${g.t - 1}), blind (promise says ${exp})`, hurt, orig, [], exp);
        drill(`CT + ${g.t - 1} covered data, named`, hurt, orig, wounds, 'EXACT');
      }
    }
    // placement none physics: the columns agree
    if (isNone(g)) {
      // blind P-1 squares scribbled at random, at least one of them data -> EXACT (located jointly)
      const idx = pickDistinct(g.t - 1, g.n);
      if (!idx.some(j => isData(g, j))) idx[0] = r(g.s);
      const [hurt] = killSquares(g, c, idx);
      drill(`none: ${g.t - 1} random squares (P-1) scribbled, blind -> located jointly`, hurt, orig, [], 'EXACT');
      // blind P squares, at least one data -> HONEST (the syndromes carry no location)
      const idxP = pickDistinct(g.t, g.n);
      if (!idxP.some(j => isData(g, j))) idxP[0] = r(g.s);
      const [hurtP, woundsP] = killSquares(g, c, idxP);
      drill(`none: ${g.t} random squares (P) scribbled, blind -> refuses`, hurtP, orig, [], 'HONEST');
      drill(`none: the same ${g.t} squares (P) addressed -> erasures`, hurtP, orig, woundsP, 'EXACT');
      // floor(P/2) squares: the classical certainty (Berlekamp-Massey alone)
      const idxH = pickDistinct(Math.floor(g.t / 2), g.n);
      const [hurtH] = killSquares(g, c, idxH);
      drill(`none: ${idxH.length} squares (floor(P/2)) scribbled, blind -> certain`, hurtH, orig, [], 'EXACT');
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
}

battery('', []);
battery('.judge', ['--judge']);

// ---- the RANK TRAP (placement none): two data squares with identical content
// scribbled identically have identical error rows; the joint locator sees one
// dimension and finds no position, Berlekamp-Massey's degree-2 locator is
// refused by the 2 deg >= m guard -> HONEST predicted; EXACT lawful; SILENT never.
// Built on an identity-form file (the inner IS the bytes) at --tier 2048 (P = 4).
{
  console.log(`\n== the rank trap (identity form, --tier 2048, placement none: P = 4)`);
  const buf = Buffer.alloc(200 * 1024);
  for (let i = 0; i < buf.length; i++) buf[i] = rnd() & 0xff;
  buf.copy(buf, 20 * 2048, 10 * 2048, 11 * 2048); // square 20 := square 10
  const src = path.join(tmp, 'twins.bin');
  fs.writeFileSync(src, buf);
  const cont = path.join(tmp, 'twins.egg13');
  transmute(src, cont, ['--form', 'identity', '--tier', '2048', '--ct', 'none']); // forced: at 2048 on 100 squares the argmin rightly prefers triple (t=3, ~6,978 B) over none (P=4, 8,396 B)
  const g = info(cont);
  const c = fs.readFileSync(cont);
  report('rank trap geometry: none, blk 2048, P 4, identity form', isNone(g) && g.block === 2048 && g.t === 4 && g.model === 1, `${g.mode} blk ${g.block} t ${g.t} model ${g.model} total ${g.total}`);
  report('rank trap: squares 10 and 20 identical in the container', c.subarray(squareOff(g, 10), squareOff(g, 10) + 2048).equals(c.subarray(squareOff(g, 20), squareOff(g, 20) + 2048)));
  const noise = Buffer.alloc(2048);
  for (let i = 0; i < noise.length; i++) noise[i] = rnd() & 0xff;
  const same = Buffer.from(c);
  noise.copy(same, squareOff(g, 10)); noise.copy(same, squareOff(g, 20));
  const p = path.join(tmp, 'drill.egg13');
  fs.writeFileSync(p, same);
  const got = restore(p, buf, []);
  report('RANK TRAP: two identical squares scribbled identically, blind -> HONEST or EXACT, never wrong', got !== 'SILENT', `got ${got} (predicted HONEST)`);
  drill('rank trap named -> erasures, EXACT', same, buf, [[squareOff(g, 10), 2048], [squareOff(g, 20), 2048]], 'EXACT');
  // controls
  const diff = wound(wound(c, squareOff(g, 10), 2048), squareOff(g, 20), 2048);
  drill('control: the same two squares scribbled differently, blind (k = 2 < m = 4)', diff, buf, [], 'EXACT');
  const [three] = killSquares(g, c, [10, 20, 30]);
  drill('control: three different squares, blind (k = 3 < m = 4)', three, buf, [], 'EXACT');
  const [four, w4] = killSquares(g, c, [10, 20, 30, 40]);
  drill('control: four data squares, blind (e = P) -> refuses', four, buf, [], 'HONEST');
  drill('control: four data squares, named -> EXACT', four, buf, w4, 'EXACT');
}

// ---- flag variants on the small artifact ----
{
  const f = 'repo-bundle.bin';
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  console.log(`\n== ${f}: flag variants (--ct triple, --ct none, --tier, --parity, --survive 65536)`);

  // mode A: CT triplicated -- any t squares blind, CT-covering patterns included
  {
    const cont = path.join(tmp, f + '.triple.egg13');
    transmute(src, cont, ['--ct', 'triple']);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    report('--ct triple: mode A chosen', g.mode === 'CT x3' && g.c === 0, `${g.mode}, c=${g.c}, n=${g.n}, total ${g.total}`);
    checkMirror(g, c, '--ct triple');
    drill('--ct triple pristine', c, orig, [], 'EXACT');
    const idx = pickDistinct(g.t, g.n);
    const [hurt, wounds] = killSquares(g, c, idx);
    drill(`--ct triple: ${g.t} scattered squares (=t), blind`, hurt, orig, [], 'EXACT');
    drill(`--ct triple: ${g.t} scattered squares (=t), named`, hurt, orig, wounds, 'EXACT');
    const extra = pickDistinct(1, g.n, new Set(idx))[0];
    const [hurt3] = killSquares(g, hurt, [extra]);
    drill(`--ct triple: ${g.t + 1} scattered squares (beyond t), blind`, hurt3, orig, [], blindExpect(g, [...idx, extra]));
    drill('--ct triple: head site killed, blind', wound(c, 0, 64 + g.msize), orig, [], 'EXACT');
    // two sites dead (head+mid): the residues survive in the end copy alone
    let h2 = wound(c, 0, 64 + g.msize);
    h2 = wound(h2, g.h1, 64 + g.msize);
    drill('--ct triple: head+mid sites killed, blind', h2, orig, [], 'EXACT');
  }
  // --ct none forced: the same as the default here, asserted
  {
    const cont = path.join(tmp, f + '.none.egg13');
    transmute(src, cont, ['--ct', 'none']);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    report('--ct none: placement none, P = dead(blk)+1, msize 4, price parity + 204', isNone(g) && g.c === 0 && g.msize === 4 && g.t === Math.ceil(4096 / g.block) + 2 && g.price === g.t * g.block + 204, `${g.mode} blk ${g.block} P ${g.t} price ${g.price} (${Number(g.floor_x).toFixed(2)}x)`);
    report('--ct none equals the default argmin byte for byte', c.equals(kept[f].c), `${c.length} vs ${kept[f].c.length}`);
  }
  // --tier 1024: the tier is honoured, P = dead(1024)+1 = 6 under none
  {
    const cont = path.join(tmp, f + '.t1024.egg13');
    transmute(src, cont, ['--tier', '1024']);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    const want = isNone(g) ? 6 : 5;
    report(`--tier 1024 honoured (parity = ${want} under ${g.mode})`, g.block === 1024 && g.t === want, `blk ${g.block} t ${g.t} total ${g.total}`);
    drill('--tier 1024 pristine', c, orig, [], 'EXACT');
    const at = g.slots + r(Math.max(1, c.length - g.slots - 4096 - (64 + g.msize)));
    drill(`--tier 1024: 4 KB scratch @${at}, blind`, wound(c, at, 4096), orig, [], 'EXACT');
  }
  // --parity 9: the parity count is honoured in every placement; under none 9
  // blind squares are P = 9 -> honest refusal, 8 blind exact; 10 named refuse
  {
    const cont = path.join(tmp, f + '.p9.egg13');
    transmute(src, cont, ['--parity', '9']);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    report('--parity 9 honoured', g.t === 9, `blk ${g.block} t ${g.t} ${g.mode} guaranteed ${g.guaranteed}`);
    drill('--parity 9 pristine', c, orig, [], 'EXACT');
    const idx = pickDistinct(9, g.s); // data squares only
    const [hurt, wounds] = killSquares(g, c, idx);
    const exp = blindExpect(g, idx);
    drill(`--parity 9: 9 data squares dead, blind (promise says ${exp})`, hurt, orig, [], exp);
    drill('--parity 9: 9 data squares dead, named', hurt, orig, wounds, 'EXACT');
    const [hurt8] = killSquares(g, c, idx.slice(0, 8));
    drill('--parity 9: 8 data squares dead, blind', hurt8, orig, [], 'EXACT');
    const extra = pickDistinct(1, g.s, new Set(idx))[0];
    const [hurt3, w3] = killSquares(g, hurt, [extra]);
    drill('--parity 9: 10 data squares dead, named (beyond t)', hurt3, orig, [...wounds, ...w3], 'HONEST');
  }
  // --survive 65536: a 64 KB contiguous scratch anywhere, blind
  {
    const cont = path.join(tmp, f + '.s64k.egg13');
    transmute(src, cont, ['--survive', '65536']);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    const need = Math.ceil(65536 / g.block) + 1 + (isNone(g) ? 1 : 0);
    report(`--survive 65536: parity >= ${need} under ${g.mode}`, g.t >= need && g.t <= 255,
      `blk ${g.block} t ${g.t} price ${g.price} B (${g.floor_x}x floor) total ${g.total}`);
    drill('--survive 65536 pristine', c, orig, [], 'EXACT');
    for (let i = 0; i < 2; i++) {
      const at = g.slots + r(Math.max(1, c.length - g.slots - 65536 - (64 + g.msize)));
      const hurt = wound(c, at, 65536);
      drill(`--survive 65536: 64 KB scratch @${at}, blind`, hurt, orig, [], 'EXACT');
      drill(`--survive 65536: 64 KB scratch @${at}, addressed`, hurt, orig, [[at, 65536]], 'EXACT');
    }
    drill('--survive 65536: 64 KB scratch @head (sites + squares), blind', wound(c, 0, 65536), orig, [], 'EXACT');
    drill('--survive 65536: 64 KB off the end (truncation), blind', c.subarray(0, c.length - 65536), orig, [], 'EXACT');
  }
}

// 3-bit storms x300 on the mid artifact (default placement)
{
  const f = 'corpus-1489k.bin';
  const { c, orig } = kept[f];
  console.log(`\n== ${f}: 3-bit storms x300 (blind)`);
  let exact = 0, honest = 0, silent = 0;
  for (let i = 0; i < 300; i++) {
    const p = path.join(tmp, 'storm.egg13');
    fs.writeFileSync(p, flipBits(c, 3));
    const got = restore(p, orig, []);
    if (got === 'EXACT') exact++;
    else if (got === 'HONEST') honest++;
    else silent++;
  }
  report('3-bit storm x300: zero silent', silent === 0, `${exact} EXACT, ${honest} honest, ${silent} SILENT`);
  report('3-bit storm x300: all repaired', exact === 300, `${exact}/300 EXACT`);
}

// wide scratches on the large artifact: beyond t squares they MUST be
// refused (blind or addressed), a lie never
{
  const f = 'real-test.db';
  const { g, c, orig } = kept[f];
  console.log(`\n== ${f}: wide scratches (capacity ${g.t} x ${g.block} B; beyond it honest, never silent)`);
  for (const kb of [16, 64, 128]) {
    const len = kb * 1024;
    const at = g.slots + Math.floor(Math.max(0, g.mid * g.block - len) / 2);
    const hurt = wound(c, at, len);
    const exp = len > g.t * g.block ? 'HONEST' : 'ANY';
    drill(`${kb} KB scratch, addressed`, hurt, orig, [[at, len]], exp);
    drill(`${kb} KB scratch, blind`, hurt, orig, [], exp);
  }
}

// no-armor: pristine exact; any wound must fail honestly (residues convict,
// nothing repairs)
{
  const f = 'repo-bundle.bin';
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  const cont = path.join(tmp, f + '.noarmor.egg13');
  transmute(src, cont, ['--no-armor']);
  const c = fs.readFileSync(cont);
  console.log(`\n== ${f} --no-armor: residues convict, nothing repairs`);
  drill('no-armor pristine', c, orig, [], 'EXACT');
  drill('no-armor 1-byte wound (must refuse)', wound(c, Math.floor(c.length / 2), 1), orig, [], 'HONEST');
  // cutting exactly hdr2 touches no payload: the surviving headers carry it
  drill('no-armor truncation of hdr2 only (headers survive)', c.subarray(0, c.length - 64), orig, [], 'EXACT');
  drill('no-armor truncation into payload (must refuse)', c.subarray(0, c.length - 4096), orig, [], 'HONEST');
}

// ancestors: every prior container restores EXACT through eggv13, wounded
// head included (their armor v3/v2 paths ride verbatim as armor11.rs)
{
  const f = 'repo-bundle.bin';
  const src = path.join(corpus, f);
  const orig = fs.readFileSync(src);
  console.log(`\n== ancestors (.egg8 .. .egg11) through eggv13`);
  for (const v of [8, 9, 10, 11]) {
    const anc = path.join(root, '..', `codegg-v${v}`, 'target', 'release', `eggv${v}.exe`);
    if (!fs.existsSync(anc)) { report(`ancestor v${v} present`, false, `${anc} missing`); continue; }
    const cont = path.join(tmp, `${f}.egg${v}`);
    try {
      execFileSync(anc, ['transmute', src, '-o', cont], { stdio: 'pipe' });
    } catch (e) { report(`ancestor v${v} transmute`, false, String(e.message).slice(0, 120)); continue; }
    const c = fs.readFileSync(cont);
    const okP = restore(cont, orig, []);
    report(`ancestor .egg${v} pristine restore`, okP === 'EXACT', `got ${okP} (${c.length} B)`);
    const p = path.join(tmp, `wounded.egg${v}`);
    fs.writeFileSync(p, wound(c, 0, 4096));
    const okW = restore(p, orig, []);
    report(`ancestor .egg${v} 4 KB head wound, blind`, okW === 'EXACT', `got ${okW}`);
  }
}

// ---- the PEEL battery (v13-M1) -------------------------------------------
// THE LAW OF THE PEEL: a peel is a bijection or it is not used, and a peel
// that cannot prove itself keeps the bytes. Every drill here must end EXACT:
// a refusal of the PEEL is not a refusal of the FILE, and losing a file is the
// one thing that must never happen. A wounded peeled container must repair
// inside its promise and refuse honestly beyond it, exactly like any other.
{
  const jsrc = path.join(root, 'corpus-real', 'wallpaper.jpg');
  if (!fs.existsSync(jsrc)) {
    report('peel battery: corpus-real/wallpaper.jpg present', false, 'missing');
  } else {
    const orig = fs.readFileSync(jsrc);
    console.log(`\n== the peel battery (v13-M1): ${orig.length} B of baseline JPEG`);

    // 1. the peel is taken and the container restores EXACT
    const cont = path.join(tmp, 'peel.egg13');
    transmute(jsrc, cont);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    report('the trial takes the peeled form on a baseline JPEG', g.model === 24, `model ${g.model}, total ${g.total}`);
    report('the peel names its recipe and its values in info',
      g.peel === 1 && g.peel_recipe > 0 && g.peel_values > 0,
      `peel ${g.peel} recipe ${g.peel_recipe} B (raw ${g.peel_recipe_raw}) values ${g.peel_values} B (raw ${g.peel_values_raw})`);
    report('the recipe does not eat the prize (recipe < 1% of the values)',
      g.peel_recipe * 100 < g.peel_values, `${g.peel_recipe} vs ${g.peel_values}`);
    drill('peeled container: pristine restore', c, orig, [], 'EXACT');
    checkMirror(g, c, 'peeled');

    // 2. the three tournament injuries on a peeled container
    drill('peeled container: 1-byte flip, blind', wound(c, Math.floor(c.length / 2), 1), orig, [], 'EXACT');
    {
      const at = Math.max(0, Math.floor(c.length / 2) - 2048);
      drill('peeled container: 4 KB scratch, addressed', wound(c, at, 4096), orig, [[at, 4096]], 'EXACT');
      drill('peeled container: 4 KB scratch, blind', wound(c, at, 4096), orig, [], 'EXACT');
    }
    drill('peeled container: 4 KB truncation', c.subarray(0, c.length - 4096), orig, [], 'EXACT');
    // beyond the promise: honest, never wrong
    {
      const P = g.t;
      const at = 64 + g.msize + (P + 2) * g.block; // well inside the data squares
      drill('peeled container: beyond capacity, blind (must refuse)',
        wound(c, at, (P + 2) * g.block), orig, [], 'HONEST');
    }

    // 3. the refusals: each of these must KEEP ITS BYTES and restore EXACT
    const hostiles = [];
    function seg(b, marker) {
      let i = 2;
      while (i + 3 < b.length) {
        if (b[i] !== 0xFF) return null;
        const m = b[i + 1];
        if (m === 0xD8 || m === 0x01 || (m >= 0xD0 && m <= 0xD7)) { i += 2; continue; }
        if (m === 0xD9 || m === 0xDA) return null;
        const L = (b[i + 2] << 8) | b[i + 3];
        if (m === marker) return [i, L];
        i += 2 + L;
      }
      return null;
    }
    // a truncated member
    hostiles.push(['truncated member', orig.subarray(0, Math.floor(orig.length * 0.5))]);
    // a corrupt Huffman table
    {
      const b = Buffer.from(orig); const d = seg(b, 0xC4);
      if (d) { b[d[0] + 4 + 25] ^= 0xFF; hostiles.push(['corrupt DHT', b]); }
    }
    // a progressive frame (SOF2 in place of SOF0)
    {
      const b = Buffer.from(orig); const d = seg(b, 0xC0);
      if (d) { b[d[0] + 1] = 0xC2; hostiles.push(['progressive input', b]); }
    }
    // an arithmetic-coded frame
    {
      const b = Buffer.from(orig); const d = seg(b, 0xC0);
      if (d) { b[d[0] + 1] = 0xC9; hostiles.push(['arithmetic-coded input', b]); }
    }
    // 12-bit samples
    {
      const b = Buffer.from(orig); const d = seg(b, 0xC0);
      if (d) { b[d[0] + 4] = 12; hostiles.push(['12-bit samples', b]); }
    }
    // a peel whose RE-ENCODE cannot reproduce the file: the padding before one
    // restart is changed from all-ones to a mixed value, so what the decoder
    // reads back is not what the encoder would write
    {
      const b = Buffer.from(orig);
      let i = 2;
      while (i + 3 < b.length) {
        if (b[i] === 0xFF && b[i + 1] === 0xDA) { i += 2 + ((b[i + 2] << 8) | b[i + 3]); break; }
        i += 2 + ((b[i + 2] << 8) | b[i + 3]);
      }
      // find the first RST marker and poison the byte before it
      for (let k = i; k + 1 < b.length; k++) {
        if (b[k] === 0xFF && b[k + 1] >= 0xD0 && b[k + 1] <= 0xD7) { b[k - 1] = (b[k - 1] & 0xF0) | 0x05; break; }
      }
      hostiles.push(['a peel whose re-encode would differ', b]);
    }
    for (const [name, buf] of hostiles) {
      const hp = path.join(tmp, 'hostile.jpg');
      fs.writeFileSync(hp, buf);
      const hc = path.join(tmp, 'hostile.egg13');
      let ok = false, detail = '';
      try {
        transmute(hp, hc);
        const hg = info(hc);
        const back = restore(hc, buf, []);
        ok = back === 'EXACT';
        detail = `model ${hg.model} (${hg.model === 24 ? 'PEELED' : 'raw pipeline'}), ${hg.total} B, restore ${back}`;
      } catch (e) { detail = String(e.message).slice(0, 120); }
      report(`peel refusal keeps the bytes: ${name}`, ok, detail);
    }
  }
}

// ---- v13-M2(0): the BLIND promise on a REAL row, at EVERY tier -------------
// The M1 gate asserted the blind 4 KB scratch only on synthetic containers,
// whose tiers put 4,096 B inside 3 to 5 squares; at the 256 tier -- where 17 of
// the 20 corpus rows land -- the same wound spans 17 squares and the assertion
// never ran there. It runs here, on notepad.exe, at all five tiers, together
// with the two edges the promise now names: the JOINT rung's t-1 (EXACT) vs t
// (refuses), and the CERTAIN rung's (t-1)/2, proved by a wound the joint rung
// cannot help with -- one fixed pattern XORed into every dead square, so the
// 128 error rows are rank 1 and there is no span to place a position in.
{
  const f = 'notepad.exe';
  const src = path.join(root, 'corpus-real', f);
  const orig = fs.readFileSync(src);
  for (const tier of [256, 512, 1024, 2048, 4096]) {
    const cont = path.join(tmp, `${f}.t${tier}.egg13`);
    transmute(src, cont, ['--tier', String(tier), '--ct', 'none']);
    const g = info(cont);
    const c = fs.readFileSync(cont);
    const bm = (g.t - 1) >> 1;
    console.log(`\n== ${f} @ tier ${tier} (placement none): n=${g.n} (${g.s} data + ${g.t} parity) x ${g.block} B, guaranteed ${g.guaranteed}, certain blind ${bm}, joint to ${g.t - 1}`);
    checkMirror(g, c, `${f} t${tier}`);
    // the 4 KB blind scratch the promise sells, at five places including the
    // mid site and the end site
    const spots = [
      ['first squares', g.slots + 1],
      ['an ordinary unaligned offset', g.slots + g.block + 7],
      ['a third of the way in', Math.floor(c.length / 3)],
      ['straddling the mid site', Math.max(0, g.h1 - 1024)],
      ['the end site', c.length - 4096],
    ];
    for (const [where, at] of spots) {
      drill(`tier ${tier}: 4 KB blind scratch @${at} (${where})`, wound(c, at, 4096), orig, [], 'EXACT');
    }
    // the joint rung's two edges
    {
      const idx = pickDistinct(g.t - 1, g.n);
      if (!idx.some(j => isData(g, j))) idx[0] = r(g.s);
      drill(`tier ${tier}: ${g.t - 1} whole squares (=t-1) blind -> located jointly`,
        killSquares(g, c, idx)[0], orig, [], 'EXACT');
      const idxT = pickDistinct(g.t, g.n);
      if (!idxT.some(j => isData(g, j))) idxT[0] = r(g.s);
      const [hurtT, woundsT] = killSquares(g, c, idxT);
      drill(`tier ${tier}: ${g.t} whole squares (=t) blind -> refuses with the number`, hurtT, orig, [], 'HONEST');
      drill(`tier ${tier}: the same ${g.t} squares addressed -> erasures`, hurtT, orig, woundsT, 'EXACT');
    }
    // the CERTAIN rung: a rank-1 dependent wound. bm squares must still come
    // back (Berlekamp-Massey per codeword assumes nothing); bm+1 is beyond that
    // rung and the joint rung has nothing to span, so it must REFUSE -- never
    // a silent wrong answer.
    {
      const pat = Buffer.alloc(g.block);
      for (let i = 0; i < g.block; i++) pat[i] = (i * 37 + 11) & 0xff;
      const dep = (k) => {
        const b = Buffer.from(c);
        for (let i = 0; i < k; i++) {
          const j = 3 + i * 5;
          const o = squareOff(g, j), L = squareLen(g, j);
          for (let q = 0; q < L; q++) b[o + q] ^= pat[q];
        }
        return b;
      };
      if (bm >= 1) {
        drill(`tier ${tier}: rank-1 dependent wound of ${bm} squares ((t-1)/2, the certain band) -> EXACT`,
          dep(bm), orig, [], 'EXACT');
      }
      drill(`tier ${tier}: rank-1 dependent wound of ${bm + 1} squares (past the certain band) -> refuses, never wrong`,
        dep(bm + 1), orig, [], 'HONEST');
    }
  }
}

// ---- the filter battery (v13-M3b) ----
// Drills had ZERO filter coverage before this, while `info` has emitted
// "filter":<id>,"param":<n> since v9 -- the hook existed and nothing used it.
// Every id the header can carry is forced through the WHOLE pipeline here:
// the container must name the filter it used, the undo must be exact, and a
// wound inside the promise must still restore exactly through the undo.
{
  const cases = [
    ['wubbadub.html', 1, 0, 'byte delta 1'],
    ['wubbadub.html', 5, 7, 'byte delta at param'],
    ['ring01.wav', 6, 2, 'W16 per-channel'],
    ['ring01.wav', 8, 2, 'W16 order 2'],
    ['ring01.wav', 9, 2, 'W16 big-endian'],
    ['ring01.wav', 13, 2, 'mid/side order 1'],
    ['ring01.wav', 14, 2, 'mid/side order 2'],
    ['segoeui.ttf', 11, 0, 'the TTF respelling'],
    ['notepad.exe', 10, 0, 'BCJ'],
  ];
  console.log(`\n== the filter battery (v13-M3b): ${cases.length} forced ids through the whole pipeline`);
  for (const [f, id, param, label] of cases) {
    const src = path.join(root, 'corpus-real', f);
    if (!fs.existsSync(src)) { report(`filter ${id}: ${f} present`, false, 'missing'); continue; }
    const orig = fs.readFileSync(src);
    const cont = path.join(tmp, `filt${id}_${param}.egg13`);
    let g = null;
    try {
      transmute(src, cont, ['--filter', `${id}:${param}`]);
      g = info(cont);
    } catch (e) {
      report(`filter ${id} (${label}) on ${f}: transmutes`, false, String(e).slice(0, 90));
      continue;
    }
    // RLE8 is the one length-changing id and rides its filtered length in the
    // param; every other id must report the param it was given
    report(`filter ${id} (${label}) on ${f}: info names the filter`,
      g.filter === id && (id === 12 || g.param === param),
      `info says filter ${g.filter} param ${g.param}, asked for ${id}:${param}`);
    const c = fs.readFileSync(cont);
    drill(`filter ${id} on ${f}: pristine restore through the undo`, c, orig, [], 'EXACT');
    drill(`filter ${id} on ${f}: 1-byte flip, blind, through the undo`,
      wound(c, Math.floor(c.length / 2), 1), orig, [], 'EXACT');
  }
}

// the pigeonhole, asserted as a PASS: random MUST transmute larger
{
  const f = 'photo.bin';
  const src = path.join(corpus, f);
  const cont = path.join(tmp, f + '.noarmor.egg13');
  transmute(src, cont, ['--no-armor']);
  const grew = fs.statSync(cont).size > fs.statSync(src).size;
  report('pigeonhole: random transmutes LARGER (required)', grew,
    `${fs.statSync(src).size} -> ${fs.statSync(cont).size} B`);
}

console.log(`\n${pass} passed, ${fail} failed${fail ? ': ' + failures.join('; ') : ''}`);
fs.rmSync(tmp, { recursive: true, force: true });
process.exit(fail ? 1 : 0);
