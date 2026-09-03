/* eggSo v2 -- the green is the code.
 *
 * Not part of the site. The sixteenth codec experiment and the third in the
 * fold-native lineage. v0 used the fold's partition, v1 its symmetry. This
 * round uses the alphabet's own slack: a GREEN is the 0 of the signed-digit
 * alphabet, and under the site's canonical form
 *
 *   "no green is ever followed by a lit cell -- every green left over is
 *    trailing"                                        (stalk.js:416-427, spec.md:55-64)
 *
 * every green is forced, hence information-free. Two readings, both built,
 * both measured, because "we need to test all ways, to see who the house
 * keeps. No free lunch here" (PREDICTIONS.md):
 *
 *   (a) forced greens as check slots. A canonical square whose value has
 *       >= 28 trailing zero bits has 28 forced greens at the end; four
 *       residues fit in 28 trits. Such a square carries its own checks
 *       in-band and needs one flag bit; every other square falls back to
 *       v0's external residues. S1 (tools/greens.js) measures how often
 *       that happens, and the answer is the round's verdict.
 *   (b) greens as three-valued erasures. eggSo-v0's erasure path extended to
 *       base 3, with the cap that budget allows, and -- on canonical squares
 *       -- the two-valued model the Wub already rolls (wub.html:282-292):
 *       a flagged cell before the last lit one is +-1, in the tail it is 0.
 *
 * The stored square is the CANONICAL one: G.toCells lays the bytes row-major
 * as v0 does, pushLeft respells them into a +-1 prefix and a green tail, and
 * the bytes come back from the value V by BigInt -- G.toBytes reads
 * cells[j] === 1 and is wrong for any -1 (codegg.js:125-132).
 *
 * What is borrowed, named where it lives:
 *   - regions, per-region residues and the confirming residue are eggSo-v0's,
 *     required from ../eggSo-v0/eggso.js.
 *   - moduli, weights, the row-major layout, the trit syndrome table with its
 *     2w[j] = w[j-1] aliases (codegg.js:87-102), and the per-candidate confirm
 *     (codegg.js:204-206) are codegg-v1's, required from ../codegg-v1/codegg.js.
 * What is the site's, restated and asserted against the site's own function:
 *   - pushLeft (stalk.js:59-71) and valueOf (stalk.js:74-83).
 */
const G = require(__dirname + "/../codegg-v1/codegg.js");
const E = require(__dirname + "/../eggSo-v0/eggso.js");
const {INNER, FOLD, OUTER} = E;
const mod = (x, m) => ((x % m) + m) % m;
const bits = m => Math.ceil(Math.log2(m));

/* ---- the site's two rules, restated ---------------------------------------
   push toward the coarse end: a lit cell steps left into a green and leaves
   its own sign flipped behind. +1*2^-i == +1*2^-(i-1) - 1*2^-i. stalk.js:59-71 */
function pushLeft(cells){
  const d = Int8Array.from(cells);
  let moved = true;
  while(moved){
    moved = false;
    for(let i = d.length - 1; i > 0; i--)
      if(d[i] !== 0 && d[i - 1] === 0){ d[i - 1] = d[i]; d[i] = -d[i]; moved = true; }
  }
  return d;
}
/* exact value: cell i weighs 2^(L-1-i). stalk.js:74-83, as an integer. */
function valueOf(cells){
  let v = 0n;
  for(let i = 0; i < cells.length; i++){ v <<= 1n; if(cells[i]) v += BigInt(cells[i]); }
  return v;
}
/* canonical: the zeros are a suffix and every other cell is +-1 */
function isCanonical(cells){
  let i = 0;
  while(i < cells.length && (cells[i] === 1 || cells[i] === -1)) i++;
  while(i < cells.length && cells[i] === 0) i++;
  return i === cells.length;
}
/* trailing greens = trailing zero bits of V (the closed form S1 checks) */
function tailOf(cells){ let k = 0; while(k < cells.length && cells[cells.length - 1 - k] === 0) k++; return k; }

/* bytes from values: each square is L bits of V, big-endian */
function toBytesV(squares, L, byteLen){
  const out = new Uint8Array(byteLen), per = L / 8, mask = (1n << BigInt(L)) - 1n;
  for(let s = 0; s < squares.length; s++){
    const n = Math.min(per, byteLen - s * per);
    if(n <= 0) break;
    let v = valueOf(squares[s]) & mask;
    for(let b = per - 1; b >= 0; b--){ const byte = Number(v & 0xFFn); v >>= 8n; if(b < n) out[s * per + b] = byte; }
  }
  return out;
}

/* ---- the code, per file ------------------------------------------------
   v0's regions and moduli; per-region syndrome tables over d in {+-1, +-2}
   after codegg.js:90-102, so a sign flip (d = +-2) names its cell and its
   alias one place up is settled by the alphabet, the canonicity filter and q. */
function makeCode(N, opts){
  const base = E.makeCode(N, {confirm: true});
  const {L, p, w} = base;
  const tables = base.members.map(list => {
    const t = new Map();
    for(const i of list) for(const d of [1, -1, 2, -2]){
      const s = mod(d * w[i], p);
      if(!t.has(s)) t.set(s, []);
      t.get(s).push({i, d});
    }
    return t;
  });
  /* the alias count PREDICTIONS.md files: distinct syndromes over the whole
     square, one prime, four d's. Planning computed 4096 - 2046 = 2050. */
  const all = new Set();
  for(let i = 0; i < L; i++) for(const d of [1, -1, 2, -2]) all.add(mod(d * w[i], p));
  return {...base, arm: "2", tritTables: tables, tritDistinct: all.size, tritEntries: 4 * L,
          cap: (opts && opts.cap) || 10, slots: 28};
}

/* ---- decoding one square -------------------------------------------------
   opts.alphabet   "trit" (default) | "bit"
   opts.canonical  true (default): the square is a pushed one -- reject any
                   repair that leaves a green before a lit cell, and read
                   flagged cells with the two-valued model
   opts.confirm    true (default): q per candidate, codegg.js:204-206
   opts.erased     flagged positions. There is NO sentinel: -1 is a digit.
   opts.cap        base-3 erasure cap per region (default code.cap = 10)
   Returns {status, fixed, direct, searched, note, readings} */
function repairSquare(cells, check, code, opts){
  const o = opts || {};
  const trit = o.alphabet !== "bit", canon = trit && o.canonical !== false, confirm = o.confirm !== false;
  const inAlpha = trit ? (v => v >= -1 && v <= 1) : (v => v === 0 || v === 1);
  const {p, q, w, region, L} = code, Qw = code.Q.w;
  const fitsQ = (base, plan) => { if(!confirm) return true; let r = base; for(const e of plan) r = mod(r - e.d * Qw[e.i], q); return r === check[3]; };
  const canonAfter = plan => { if(!canon) return true; const t = cells.slice(); for(const e of plan) t[e.i] -= e.d; return isCanonical(t); };

  /* ---- erasures, explicit only ---- */
  const F = [...new Set(o.erased || [])].sort((a, b) => a - b);
  if(F.length){
    const flagged = new Set(F), base = cells.slice();
    for(const i of F) base[i] = 0;
    const baseRes = E.regionResidues(base, code), baseQ = G.residue(base, q);
    const need = [0, 1, 2].map(k => mod(check[k] - baseRes[k], p)), needQ = mod(check[3] - baseQ, q);

    /* the two-valued model on a canonical square: unflagged lit cells all sit
       before unflagged greens or the square is not canonical; flagged cells
       up to the last lit one are +-1, from the first unflagged green on they
       are 0, and the boundary falls somewhere among the flagged cells between */
    if(canon){
      let litEnd = -1, zeroStart = L;
      for(let i = 0; i < L; i++) if(!flagged.has(i)){ if(base[i] !== 0) litEnd = i; else if(zeroStart === L) zeroStart = i; }
      if(litEnd < zeroStart){
        const prefix = F.filter(i => i < litEnd), gap = F.filter(i => i > litEnd && i < zeroStart);
        let total = 0; for(let m = 0; m <= gap.length; m++) total += Math.pow(2, prefix.length + m);
        if(total > (1 << 17)) return {status: "detected", fixed: 0, note: "erasures: too many readings"};
        const sols = [];
        for(let m = 0; m <= gap.length && sols.length < 2; m++){
          const lit = [...prefix, ...gap.slice(0, m)], n = lit.length;
          for(let a = 0; a < (1 << n) && sols.length < 2; a++){
            const r = [0, 0, 0]; let rq = 0;
            for(let j = 0; j < n; j++){ const v = (a >> j) & 1 ? 1 : -1, i = lit[j]; r[region[i]] = mod(r[region[i]] + v * w[i], p); rq = mod(rq + v * Qw[i], q); }
            if(r[0] !== need[0] || r[1] !== need[1] || r[2] !== need[2]) continue;
            if(confirm && rq !== needQ) continue;
            sols.push({lit, a});
          }
        }
        if(sols.length !== 1) return {status: sols.length ? "ambiguous" : "detected", fixed: 0, note: "erasures", readings: sols.length};
        for(const i of F) cells[i] = 0;
        sols[0].lit.forEach((i, j) => { cells[i] = (sols[0].a >> j) & 1 ? 1 : -1; });
        return {status: "corrected", fixed: F.length, direct: F.length, searched: 0, note: "erasures: two-valued"};
      }
      /* not canonical after all: fall through to the three-valued model */
    }

    /* base 3 (or base 2 under the bit alphabet), per region, v0's shape */
    /* the cap is a budget: 3^10 = 59,049 <= 2^16 under trits; under bits v0's 16 */
    const cap = o.cap || (trit ? code.cap : 16), vals = trit ? [-1, 0, 1] : [0, 1], B = vals.length;
    const byRegion = [[], [], []];
    for(const i of F) byRegion[region[i]].push(i);
    const hitsPer = [];
    for(let k = 0; k < 3; k++){
      const Fk = byRegion[k];
      if(!Fk.length){ if(need[k] !== 0) return {status: "detected", fixed: 0, note: "erasures+error"}; hitsPer.push([{dig: null, rq: 0}]); continue; }
      if(Fk.length > cap) return {status: "detected", fixed: 0, note: "too many erasures"};
      const n = Fk.length, dig = new Int8Array(n);
      let rp = 0, rq = 0;
      for(let j = 0; j < n; j++){ rp = mod(rp + vals[0] * w[Fk[j]], p); rq = mod(rq + vals[0] * Qw[Fk[j]], q); }
      const hits = [], count = Math.pow(B, n);
      for(let a = 0; ; a++){
        if(rp === need[k]){ hits.push({dig: Int8Array.from(dig), rq}); if(hits.length > 100000) return {status: "detected", fixed: 0, note: "erasures: too many readings"}; }
        if(a + 1 >= count) break;
        let j = 0;                                          // odometer
        while(dig[j] === B - 1){ rp = mod(rp - (vals[B - 1] - vals[0]) * w[Fk[j]], p); rq = mod(rq - (vals[B - 1] - vals[0]) * Qw[Fk[j]], q); dig[j] = 0; j++; }
        rp = mod(rp + (vals[dig[j] + 1] - vals[dig[j]]) * w[Fk[j]], p); rq = mod(rq + (vals[dig[j] + 1] - vals[dig[j]]) * Qw[Fk[j]], q); dig[j]++;
      }
      if(!hits.length) return {status: "detected", fixed: 0, note: "erasures"};
      hitsPer.push(hits);
    }
    const combos = hitsPer[0].length * hitsPer[1].length * hitsPer[2].length;
    if(combos > 1 && !confirm) return {status: "ambiguous", fixed: 0, note: "erasures", readings: combos};
    if(combos > 2000000) return {status: "detected", fixed: 0, note: "erasures: too many readings", readings: combos};
    let survivor = null, count = 0;
    for(const h0 of hitsPer[0]) for(const h1 of hitsPer[1]) for(const h2 of hitsPer[2]){
      if(confirm && mod(h0.rq + h1.rq + h2.rq, q) !== needQ) continue;
      count++; survivor = [h0, h1, h2];
      if(count > 1) break;
    }
    if(count !== 1) return {status: count ? "ambiguous" : "detected", fixed: 0, note: "erasures", readings: combos};
    for(let k = 0; k < 3; k++){ const Fk = byRegion[k], h = survivor[k]; if(h.dig) for(let j = 0; j < Fk.length; j++) cells[Fk[j]] = vals[h.dig[j]]; }
    return {status: "corrected", fixed: F.length, direct: F.length, searched: 0, note: "erasures: base " + B, readings: combos};
  }

  /* ---- errors ---- */
  const cur = E.regionResidues(cells, code);
  const delta = [0, 1, 2].map(k => mod(cur[k] - check[k], p));
  const curQ = G.residue(cells, q);
  const hurt = [0, 1, 2].filter(k => delta[k] !== 0);
  if(!hurt.length){
    if(confirm && curQ !== check[3]) return {status: "detected", fixed: 0, note: "confirm only"};
    return {status: "clean", fixed: 0, direct: 0, searched: 0};
  }
  const tables = trit ? code.tritTables : code.tables;
  const singles = hurt.map(k => (tables[k].get(delta[k]) || []).filter(c => inAlpha(cells[c.i] - c.d)));
  const product = lists => lists.reduce((acc, l) => acc.flatMap(a => l.map(x => [...a, x])), [[]]);
  /* every hurt region has a single: combine, confirm per candidate plan */
  if(singles.every(s => s.length)){
    const sols = product(singles).filter(plan => fitsQ(curQ, plan) && canonAfter(plan));
    if(sols.length === 1){ for(const e of sols[0]) cells[e.i] -= e.d; return {status: "corrected", fixed: sols[0].length, direct: sols[0].length, searched: 0, note: "single"}; }
    if(sols.length > 1) return {status: "ambiguous", fixed: 0, note: "single", readings: sols.length};
  }
  /* one region holds a pair: v0's in-region search with the confirm inside
     the loop (codegg.js:214-232 shape), the others singles */
  if(o.doubles === false) return {status: "detected", fixed: 0, note: "doubles off"};
  const ds = trit ? [1, -1, 2, -2] : [1, -1];
  const sols = [];
  for(let x = 0; x < hurt.length && sols.length < 2; x++){
    const kp = hurt[x], others = singles.filter((_, y) => y !== x);
    if(others.some(s => !s.length)) continue;
    for(const combo of product(others)){
      let rq = curQ; for(const e of combo) rq = mod(rq - e.d * Qw[e.i], q);
      const seen = new Set();
      for(const i1 of code.members[kp]){
        for(const d1 of ds){
          if(!inAlpha(cells[i1] - d1)) continue;
          const rest = mod(delta[kp] - d1 * w[i1], p);
          if(rest === 0) continue;
          for(const c of tables[kp].get(rest) || []){
            if(c.i === i1 || !inAlpha(cells[c.i] - c.d)) continue;
            const key = c.i < i1 ? `${c.i},${c.d},${i1},${d1}` : `${i1},${d1},${c.i},${c.d}`;
            if(seen.has(key)) continue;
            seen.add(key);
            const plan = [...combo, {i: i1, d: d1}, c];
            if(confirm && mod(rq - d1 * Qw[i1] - c.d * Qw[c.i], q) !== check[3]) continue;
            if(!canonAfter(plan)) continue;
            sols.push(plan); if(sols.length > 1) break;
          }
          if(sols.length > 1) break;
        }
        if(sols.length > 1) break;
      }
      if(sols.length > 1) break;
    }
  }
  if(sols.length === 1){ for(const e of sols[0]) cells[e.i] -= e.d; return {status: "corrected", fixed: sols[0].length, direct: sols[0].length - 2, searched: 2, note: "pair"}; }
  return {status: sols.length ? "ambiguous" : "detected", fixed: 0, note: sols.length ? "pair" : "unrepaired", readings: sols.length};
}

/* ---- (a) forced greens as check slots ------------------------------------
   Four residues, 7 balanced-ternary trits each (3^7 = 2187 > q), written into
   the last 28 cells of a canonical square whose value has >= 28 trailing zero
   bits -- cells the canonical form forces green. One flag bit per square says
   whether the square is in-band; every other square carries v0's residues
   outside. */
const OFFSET = 1031;                                     // 0..2062 -> -1031..1031 fits 7 balanced trits (+-1093)
function toTrits(r){ let v = r - OFFSET; const t = new Int8Array(7); for(let k = 6; k >= 0; k--){ let d = mod(v, 3); if(d === 2) d = -1; t[k] = d; v = (v - d) / 3; } return t; }
function fromTrits(t){ let v = 0; for(let k = 0; k < 7; k++) v = v * 3 + t[k]; return v + OFFSET; }
function encodeA(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCode(N, opts);
  const {L, slots} = code;
  const squares = G.toCells(bytes, L).map(pushLeft);
  const flags = new Uint8Array(squares.length), external = new Array(squares.length).fill(null);
  let inBand = 0;
  squares.forEach((sq, s) => {
    const chk = E.checksFor(sq, code);
    if(tailOf(sq) >= slots){
      flags[s] = 1; inBand++;
      chk.forEach((r, k) => { const t = toTrits(r); for(let j = 0; j < 7; j++) sq[L - slots + k * 7 + j] = t[j]; });
    } else external[s] = chk;
  });
  return {squares, flags, external, code, arm: "2a",
          meta: {arm: "2a", N, L, p: code.p, q: code.q, bytes: bytes.length, squares: squares.length, inBand}};
}
/* read the in-band check out of the tail and zero the slots; null if the
   tail does not spell four residues */
function extractChecks(sq, code){
  const {L, slots} = code, chk = [];
  for(let k = 0; k < 4; k++){
    const t = sq.subarray(L - slots + k * 7, L - slots + k * 7 + 7);
    for(const v of t) if(v < -1 || v > 1) return null;
    const r = fromTrits(t);
    if(r < 0 || r >= (k === 3 ? code.q : code.p)) return null;
    chk.push(r);
  }
  for(let j = L - slots; j < L; j++) sq[j] = 0;
  return chk;
}
function writeChecks(sq, chk, code){
  const {L, slots} = code;
  chk.forEach((r, k) => { const t = toTrits(r); for(let j = 0; j < 7; j++) sq[L - slots + k * 7 + j] = t[j]; });
}
/* repair a stored square in place. An in-band square has its checks read out
   of the tail, the tail zeroed for the repair, and the checks written back
   so the stored form -- data and its own check -- is what comes out. */
function repairA(sq, flag, ext, code, opts){
  if(flag){
    const chk = extractChecks(sq, code);
    if(!chk) return {status: "detected", fixed: 0, note: "in-band check unreadable"};
    const r = repairSquare(sq, chk, code, {...(opts || {}), canonical: true});
    writeChecks(sq, chk, code);
    return r;
  }
  if(!ext) return {status: "detected", fixed: 0, note: "no external check for an out-of-band square"};
  return repairSquare(sq, ext, code, {...(opts || {}), canonical: true});
}
/* the value of a stored square: an in-band square's last 28 cells are its
   check, and were green in the canonical square */
function valueOfStored(sq, flag, code){
  if(!flag) return valueOf(sq);
  const c = sq.slice(); for(let j = code.L - code.slots; j < code.L; j++) c[j] = 0;
  return valueOf(c);
}
function decodeA(payload, opts){
  const {squares, flags, external, meta} = payload, code = payload.code || makeCode(meta.N);
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, searched: 0, inBand: 0};
  for(let s = 0; s < squares.length; s++){
    if(flags[s]) tally.inBand++;
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairA(squares[s], flags[s], external[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed; tally.direct += r.direct || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  const plain = squares.map((sq, s) => { if(!flags[s]) return sq; const c = sq.slice(); for(let j = code.L - code.slots; j < code.L; j++) c[j] = 0; return c; });
  return {bytes: toBytesV(plain, meta.L, meta.bytes), ...tally};
}
/* cost: v0's four residues on every out-of-band square, plus one flag bit
   per square. In-band squares pay only the flag. */
function sizesA(meta){
  const nsq = meta.squares || Math.ceil((meta.bytes * 8) / meta.L) || 1, inBand = meta.inBand || 0;
  const per = 3 * bits(meta.p) + bits(meta.q);
  const checkBits = (nsq - inBand) * per + nsq, dataBits = meta.bytes * 8;
  return {squares: nsq, inBand, fallbackRate: 1 - inBand / nsq, dataBytes: meta.bytes, checkBits, checkBytes: Math.ceil(checkBits / 8),
          totalBytes: meta.bytes + Math.ceil(checkBits / 8), overhead: dataBits ? checkBits / dataBits : 0,
          share: checkBits / (nsq * meta.L + checkBits), residuesPerSquare: 4, bitsPerSquare: per + 1};
}

/* ---- (b) greens as erasures: the canonical square with v0's residues outside;
   the arm is the erasure path of repairSquare above -------------------------- */
function encodeB(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCode(N, opts);
  const squares = G.toCells(bytes, code.L).map(pushLeft);
  return {squares, checks: squares.map(c => E.checksFor(c, code)), code, arm: "2b",
          meta: {arm: "2b", N, L: code.L, p: code.p, q: code.q, confirm: true, bytes: bytes.length}};
}
function decodeB(payload, opts){
  const {squares, checks, meta} = payload, code = payload.code || makeCode(meta.N);
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, searched: 0};
  for(let s = 0; s < squares.length; s++){
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairSquare(squares[s], checks[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed; tally.direct += r.direct || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  return {bytes: toBytesV(squares, meta.L, meta.bytes), ...tally};
}
const sizesB = meta => E.sizes({...meta, confirm: true});

if(typeof module !== "undefined" && module.exports)
  module.exports = {pushLeft, valueOf, isCanonical, tailOf, toBytesV, makeCode, repairSquare,
                    toTrits, fromTrits, encodeA, extractChecks, writeChecks, repairA, valueOfStored, decodeA, sizesA,
                    encodeB, decodeB, sizesB, INNER, FOLD, OUTER, NAMES: E.NAMES};
