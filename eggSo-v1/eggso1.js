/* eggSo v1 -- the anti-transpose is the code.
 *
 * Not part of the site. The fifteenth codec experiment and the second in the
 * fold-native lineage. eggSo-v0 used the fold's PARTITION (Inner / Fold /
 * Outer) and got a name for it: an interleaved AN code, legitimate and
 * sub-optimal. This round uses the fold's defining SYMMETRY -- the
 * anti-transpose
 *
 *   sigma(r, c) = (n-1-c, n-1-r)          (index.html:388, glossary.js:58)
 *
 * which fixes the Fold, swaps Inner with Outer and undoes itself. The site
 * draws it inline and has no function for it; partnerOf() below is that line
 * as a function, and tools/eggso1.test.js asserts the two agree for every
 * cell at every width the site draws.
 *
 * Three arms, all built, all measured, because "if we have 3 ways, we need
 * to test all 3 ways" (PREDICTIONS.md):
 *
 *   (a) one extra residue  R_sigma = (V - sigma V) mod p. An error d at j
 *       moves it by d * (w[j] - w[sigma j]) -- zero on the Fold. Two errors
 *       in one hemisphere give two independent syndromes, (Delta_I, Delta_I
 *       - Delta_sigma), and a sorted table names the pair with no search.
 *   (b) a mirror code: Outer := sigma(Inner). 528 data cells, 496 copies.
 *       A mismatch names the damaged pair; the region residue that moved
 *       names the side.
 *   (c) sigma as a self-inverse interleaver: store cells o sigma, check with
 *       v0. Predicted to re-derive v0 exactly; built to falsify fast.
 *
 * What is borrowed, named where it lives:
 *   - regions, per-region residues, the syndrome tables and the confirming
 *     residue are eggSo-v0's, required from ../eggSo-v0/eggso.js.
 *   - moduli, weights, the row-major layout and the whole-square residue are
 *     codegg-v1's, required from ../codegg-v1/codegg.js.
 * Nothing is copied. Bit alphabet only, as v0.
 */
const G = require(__dirname + "/../codegg-v1/codegg.js");
const E = require(__dirname + "/../eggSo-v0/eggso.js");
const {INNER, FOLD, OUTER} = E;

/* ---- the anti-transpose, once -------------------------------------------
   index.html:388   const pr = n - 1 - c, pc = n - 1 - r;   // partner across the fold
   In the row-major index j = r*N + c that the codecs use, the same map is
   sigma(j) = L - 1 - j^T with j^T = (j mod N)*N + floor(j / N).            */
function partnerRC(r, c, N){ return [N - 1 - c, N - 1 - r]; }
function partnerOf(j, N){ return N * N - 1 - ((j % N) * N + Math.floor(j / N)); }
function sigmaTable(N){
  const L = N * N, s = new Int32Array(L);
  for(let j = 0; j < L; j++) s[j] = partnerOf(j, N);
  return s;
}
/* cells o sigma: out[k] = cells[sigma k]. sigma is an involution, so applying
   this twice is the identity -- the test measures that it is. */
function sigmaSquare(cells, sig){
  const out = new Int8Array(cells.length);
  for(let k = 0; k < cells.length; k++) out[k] = cells[sig[k]];
  return out;
}

const mod = (x, m) => ((x % m) + m) % m;
const bits = m => Math.ceil(Math.log2(m));
const inBit = v => v === 0 || v === 1;

/* =========================================================================
   (a)  R_sigma = (V - sigma V) mod p
   ========================================================================= */

/* the pair table over Inner. For an Inner pair {a, b} with signs d1, d2:
     X = d1 w[a]       + d2 w[b]          (= Delta_I)
     Y = d1 w[sigma a] + d2 w[sigma b]    (= Delta_I - Delta_sigma)
   An Outer pair {sigma a, sigma b} casts the same (X, Y) as
   (Delta_O - Delta_sigma, Delta_O), so one table serves both hemispheres.
   491,040 entries at N = 32, packed key*2^20 + value into a Float64Array and
   sorted natively; lookups are two binary searches. The collision count is
   the round's measurement #1 and PREDICTIONS.md files it at 7.25%. */
const SHIFT = 1048576;                      // 2^20 > 4 * 496 * 496
function pairTable(code){
  const {p, w, sig} = code, inner = code.members[INNER], n = inner.length;
  const D = [[1, 1], [1, -1], [-1, 1], [-1, -1]];
  const keys = new Float64Array(n * (n - 1) / 2 * 4);
  let t = 0;
  for(let a = 0; a < n; a++) for(let b = a + 1; b < n; b++){
    const ia = inner[a], ib = inner[b];
    for(let di = 0; di < 4; di++){
      const [d1, d2] = D[di];
      const X = mod(d1 * w[ia] + d2 * w[ib], p);
      const Y = mod(d1 * w[sig[ia]] + d2 * w[sig[ib]], p);
      keys[t++] = (X * p + Y) * SHIFT + ((a * n + b) * 4 + di);
    }
  }
  keys.sort();
  /* the measurement: distinct joint syndromes, and entries sharing one */
  let distinct = 0, colliding = 0;
  for(let i = 0; i < keys.length;){
    const k = Math.floor(keys[i] / SHIFT);
    let j = i + 1;
    while(j < keys.length && Math.floor(keys[j] / SHIFT) === k) j++;
    distinct++; if(j - i > 1) colliding += j - i;
    i = j;
  }
  /* the contrast: the region residue alone, v0's situation */
  const xs = new Set();
  for(let a = 0; a < n; a++) for(let b = a + 1; b < n; b++) for(const [d1, d2] of D)
    xs.add(mod(d1 * w[inner[a]] + d2 * w[inner[b]], p));
  /* and whether any pair's (X, Y) equals a single's (d w[i], d w[sigma i]) */
  let singleAlias = 0;
  for(const i of inner) for(const d of [1, -1]){
    const X = mod(d * w[i], p), Y = mod(d * w[sig[i]], p);
    const [lo, hi] = range(keys, X * p + Y);
    singleAlias += hi - lo;
  }
  return {keys, n, inner, D, total: keys.length, distinct, colliding,
          collisionRate: colliding / keys.length,
          regionOnlyDistinct: xs.size, singleAlias};
}
function lowerBound(keys, v){
  let lo = 0, hi = keys.length;
  while(lo < hi){ const m = (lo + hi) >> 1; if(keys[m] < v) lo = m + 1; else hi = m; }
  return lo;
}
function range(keys, key){ return [lowerBound(keys, key * SHIFT), lowerBound(keys, (key + 1) * SHIFT)]; }
/* every (ia, ib, d1, d2) over Inner whose joint syndrome is (X, Y) */
function lookupPairs(tab, X, Y){
  const [lo, hi] = range(tab.keys, X * tab.p + Y);
  return decodeRange(tab, lo, hi);
}
function decodeRange(tab, lo, hi){
  const out = [];
  for(let k = lo; k < hi; k++){
    const v = tab.keys[k] - Math.floor(tab.keys[k] / SHIFT) * SHIFT;
    const di = v & 3, ab = (v - di) / 4, a = Math.floor(ab / tab.n), b = ab - a * tab.n;
    out.push({ia: tab.inner[a], ib: tab.inner[b], d1: tab.D[di][0], d2: tab.D[di][1]});
  }
  return out;
}

/* the same candidate set by a 992-probe peel: for each (a, d1) over Inner,
   the remainder X - d1 w[a] must be a single (b, d2) in Inner's own table
   and Y must agree. The test asserts peel == lookup on random syndromes. */
function peelPairs(code, X, Y){
  const {p, w, sig} = code, out = [], seen = new Set();
  for(const a of code.members[INNER]) for(const d1 of [1, -1]){
    const rest = mod(X - d1 * w[a], p);
    for(const c of code.tables[INNER].get(rest) || []){
      if(c.i === a) continue;
      if(mod(d1 * w[sig[a]] + c.d * w[sig[c.i]], p) !== Y) continue;
      const ia = Math.min(a, c.i), ib = Math.max(a, c.i);
      const key = `${ia},${ib},${a < c.i ? d1 : c.d},${a < c.i ? c.d : d1}`;
      if(seen.has(key)) continue;
      seen.add(key);
      out.push(a < c.i ? {ia, ib, d1, d2: c.d} : {ia, ib, d1: c.d, d2: d1});
    }
  }
  return out;
}

function makeCodeA(N, opts){
  const base = E.makeCode(N, opts);                 // v0's regions, tables, p, q
  const sig = sigmaTable(N), {L, p, w} = base;
  const ws = new Int32Array(L);                     // (w[j] - w[sigma j]) mod p
  for(let j = 0; j < L; j++) ws[j] = mod(w[j] - w[sig[j]], p);
  const code = {...base, arm: "a", sig, ws};
  code.pair = pairTable(code); code.pair.p = p;
  return code;
}
function sigmaResidue(cells, code){
  let acc = 0;
  for(let j = 0; j < code.L; j++) if(cells[j]) acc = mod(acc + cells[j] * code.ws[j], code.p);
  return acc;
}
/* [I, F, O, R_sigma, (q)] */
function checksForA(cells, code){
  const r = [...E.regionResidues(cells, code), sigmaResidue(cells, code)];
  return code.confirm ? [...r, G.residue(cells, code.q)] : r;
}
function encodeA(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCodeA(N, opts);
  const squares = G.toCells(bytes, code.L);
  return {squares, checks: squares.map(c => checksForA(c, code)), code,
          meta: {arm: "a", N, L: code.L, p: code.p, q: code.q, confirm: code.confirm, bytes: bytes.length}};
}
function sizesA(meta){
  const nsq = Math.ceil((meta.bytes * 8) / meta.L) || 1;
  const per = 4 * bits(meta.p) + (meta.confirm ? bits(meta.q) : 0);
  const checkBits = nsq * per, dataBits = meta.bytes * 8;
  return {squares: nsq, dataBytes: meta.bytes, checkBits, checkBytes: Math.ceil(checkBits / 8),
          totalBytes: meta.bytes + Math.ceil(checkBits / 8),
          overhead: dataBits ? checkBits / dataBits : 0,
          share: checkBits / (nsq * meta.L + checkBits),
          residuesPerSquare: meta.confirm ? 5 : 4, bitsPerSquare: per};
}

const product = lists => lists.reduce((acc, l) => acc.flatMap(a => l.map(x => [...a, x])), [[]]);

/* decoding one square. Returns {status, fixed, direct, lookup, searched, note, regions}
     direct    cells named by a syndrome with no search (singles, table pairs)
     lookup    the subset of direct that came from the pair table
     searched  cells found by the Fold pair search
   Never repairs on a guess: two consistent plans is `ambiguous`. Every plan
   must satisfy R_sigma and, when carried, q, before a cell is touched. */
function repairA(cells, check, code, opts){
  const {p, q, w, ws, confirm} = code, Qw = confirm ? code.Q.w : null;
  const cur = E.regionResidues(cells, code);
  const delta = [0, 1, 2].map(k => mod(cur[k] - check[k], p));
  const dS = mod(sigmaResidue(cells, code) - check[3], p);
  const dQ = confirm ? mod(G.residue(cells, q) - check[4], q) : 0;
  const hurtNames = h => h.map(k => E.NAMES[k]);

  /* flagged erasures: v0's per-region enumeration, then R_sigma and q pick
     among the combined readings. R_sigma is the 1/p filter v0 bare lacked. */
  const flagged = new Set((opts && opts.erased) || []);
  for(let i = 0; i < cells.length; i++) if(cells[i] === -1) flagged.add(i);
  if(flagged.size){
    const byRegion = [[], [], []];
    for(const i of flagged) byRegion[code.region[i]].push(i);
    const base = cells.slice();
    for(const i of flagged) base[i] = 0;
    const baseRes = E.regionResidues(base, code), baseS = sigmaResidue(base, code);
    const baseQ = confirm ? G.residue(base, q) : 0;
    const hitsPer = [];
    for(let k = 0; k < 3; k++){
      const F = byRegion[k];
      if(!F.length){
        if(baseRes[k] !== check[k]) return {status: "detected", fixed: 0, note: "erasures+error"};
        hitsPer.push([{a: 0, s: 0, qq: 0}]); continue;
      }
      if(F.length > 16) return {status: "detected", fixed: 0, note: "too many erasures"};
      const hits = [];
      for(let a = 0; a < (1 << F.length); a++){
        let r = baseRes[k], s = 0, qq = 0;
        for(let j = 0; j < F.length; j++) if(a & (1 << j)){
          r = (r + w[F[j]]) % p; s = (s + ws[F[j]]) % p; if(confirm) qq = (qq + Qw[F[j]]) % q;
        }
        if(r === check[k]){ hits.push({a, s, qq}); if(hits.length > 64) break; }
      }
      if(!hits.length) return {status: "detected", fixed: 0, note: "erasures"};
      hitsPer.push(hits);
    }
    if(hitsPer[0].length * hitsPer[1].length * hitsPer[2].length > 8192)
      return {status: "detected", fixed: 0, note: "erasures: too many readings"};
    let survivor = null, count = 0;
    for(const h0 of hitsPer[0]) for(const h1 of hitsPer[1]) for(const h2 of hitsPer[2]){
      if((baseS + h0.s + h1.s + h2.s) % p !== check[3]) continue;
      if(confirm && (baseQ + h0.qq + h1.qq + h2.qq) % q !== check[4]) continue;
      count++; survivor = [h0.a, h1.a, h2.a];
      if(count > 1) break;
    }
    if(count !== 1) return {status: count ? "ambiguous" : "detected", fixed: 0, note: "erasures"};
    for(let k = 0; k < 3; k++){ const F = byRegion[k]; for(let j = 0; j < F.length; j++) cells[F[j]] = (survivor[k] >> j) & 1; }
    return {status: "corrected", fixed: flagged.size, direct: flagged.size, lookup: 0, searched: 0, note: "erasures"};
  }

  const hurt = [0, 1, 2].filter(k => delta[k] !== 0);
  if(!hurt.length){
    if(dS !== 0 || dQ !== 0) return {status: "detected", fixed: 0, note: "confirm only"};
    return {status: "clean", fixed: 0, direct: 0, lookup: 0, searched: 0};
  }
  const fits = plan => {
    let s = 0, qq = 0;
    for(const e of plan){ s = mod(s + e.d * ws[e.i], p); if(confirm) qq = mod(qq + e.d * Qw[e.i], q); }
    return s === dS && (!confirm || qq === dQ);
  };
  const apply = (plan, direct, lookup, searched, note) => {
    for(const e of plan) cells[e.i] -= e.d;
    return {status: "corrected", fixed: plan.length, direct, lookup, searched, note, regions: hurtNames(hurt)};
  };

  /* stage B: one alphabet-valid single per hurt region; the whole plan must
     also satisfy R_sigma (and q). R_sigma is an 11-bit confirm on every
     non-Fold single -- a same-region pair can never pass it as a single. */
  const singles = hurt.map(k => (code.tables[k].get(delta[k]) || []).filter(c => inBit(cells[c.i] - c.d)));
  if(singles.every(s => s.length)){
    const sols = product(singles).filter(fits);
    if(sols.length === 1) return apply(sols[0], sols[0].length, 0, 0, "single");
    if(sols.length > 1) return {status: "ambiguous", fixed: 0, note: "single", regions: hurtNames(hurt)};
  }

  /* stage C: exactly one region holds a pair, the others singles. Inner and
     Outer pairs come from the table, Fold pairs from the 32-cell search with
     the confirm inside the loop. Two regions needing a pair is detected. */
  const sols = [];
  for(let x = 0; x < hurt.length && sols.length < 2; x++){
    const kp = hurt[x];
    const others = hurt.filter((_, y) => y !== x);
    const otherSingles = others.map((_, y) => singles[hurt.indexOf(others[y])]);
    if(otherSingles.some(s => !s.length)) continue;
    for(const combo of product(otherSingles)){
      let sRest = dS;
      for(const e of combo) sRest = mod(sRest - e.d * ws[e.i], p);
      let cands = [];
      if(kp === FOLD){
        if(sRest !== 0) continue;
        const F = code.members[FOLD];
        for(let a = 0; a < F.length; a++) for(let b = a + 1; b < F.length; b++)
          for(const d1 of [1, -1]) for(const d2 of [1, -1]){
            if(!inBit(cells[F[a]] - d1) || !inBit(cells[F[b]] - d2)) continue;
            if(mod(d1 * w[F[a]] + d2 * w[F[b]], p) === delta[FOLD]) cands.push([{i: F[a], d: d1}, {i: F[b], d: d2}]);
          }
      } else {
        const X = kp === INNER ? delta[INNER] : mod(delta[OUTER] - sRest, p);
        const Y = kp === INNER ? mod(delta[INNER] - sRest, p) : delta[OUTER];
        const [lo, hi] = range(code.pair.keys, X * p + Y);
        for(const e of decodeRange(code.pair, lo, hi)){
          const i1 = kp === INNER ? e.ia : code.sig[e.ia], i2 = kp === INNER ? e.ib : code.sig[e.ib];
          if(inBit(cells[i1] - e.d1) && inBit(cells[i2] - e.d2)) cands.push([{i: i1, d: e.d1}, {i: i2, d: e.d2}]);
        }
      }
      for(const pair of cands){
        const plan = [...combo, ...pair];
        if(fits(plan)){ sols.push({plan, kp, singles: combo.length}); if(sols.length > 1) break; }
      }
      if(sols.length > 1) break;
    }
  }
  if(sols.length === 1){
    const {plan, kp, singles: ns} = sols[0];
    return kp === FOLD ? apply(plan, ns, 0, 2, ns ? "mixed" : "fold pair")
                       : apply(plan, ns + 2, 2, 0, ns ? "mixed" : "pair");
  }
  return {status: sols.length ? "ambiguous" : "detected", fixed: 0,
          note: sols.length ? "pair" : "unrepaired", regions: hurtNames(hurt)};
}
function decodeA(payload, opts){
  const {squares, checks, meta} = payload;
  const code = payload.code || makeCodeA(meta.N, {confirm: meta.confirm});
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, lookup: 0, searched: 0};
  for(let s = 0; s < squares.length; s++){
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairA(squares[s], checks[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed;
    tally.direct += r.direct || 0; tally.lookup += r.lookup || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  return {bytes: G.toBytes(squares, meta.L, meta.bytes), ...tally};
}
function verifyA(cells, check, code){
  const r = E.regionResidues(cells, code);
  return r[0] === check[0] && r[1] === check[1] && r[2] === check[2]
      && sigmaResidue(cells, code) === check[3]
      && (!code.confirm || G.residue(cells, code.q) === check[4]);
}

/* =========================================================================
   (b)  the mirror code: Outer := sigma(Inner)
   ========================================================================= */
function makeCodeB(N, opts){
  const base = E.makeCode(N, opts);
  const sig = sigmaTable(N);
  const dataIdx = [...base.members[INNER], ...base.members[FOLD]].sort((x, y) => x - y);
  return {...base, arm: "b", sig, dataIdx, K: dataIdx.length};
}
/* 528 data cells per square -- Inner then Fold in index order -- and every
   Outer cell a copy of its partner. 66 bytes exactly at N = 32. */
function toCells528(bytes, code){
  const {L, K, dataIdx, sig} = code, squares = [];
  const total = Math.ceil((bytes.length * 8) / K) || 1;
  for(let s = 0; s < total; s++){
    const cells = new Int8Array(L);
    for(let t = 0; t < K; t++){
      const bit = s * K + t, B = bit >> 3;
      cells[dataIdx[t]] = B < bytes.length ? (bytes[B] >> (7 - (bit & 7))) & 1 : 0;
    }
    for(const j of code.members[INNER]) cells[sig[j]] = cells[j];
    squares.push(cells);
  }
  return squares;
}
function toBytes528(squares, code, byteLen){
  const out = new Uint8Array(byteLen), {K, dataIdx} = code;
  for(let bit = 0; bit < byteLen * 8; bit++){
    const cells = squares[Math.floor(bit / K)];
    if(cells[dataIdx[bit % K]] === 1) out[bit >> 3] |= 1 << (7 - (bit & 7));
  }
  return out;
}
function encodeB(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCodeB(N, opts);
  const squares = toCells528(bytes, code);
  return {squares, checks: squares.map(c => E.checksFor(c, code)), code,
          meta: {arm: "b", N, L: code.L, K: code.K, p: code.p, q: code.q, confirm: code.confirm, bytes: bytes.length}};
}
/* two conventions, both always printed (PREDICTIONS.md B6):
     overhead  redundant bits per data bit, v0's convention: (mirror + checks) / data
     share     redundant bits over everything stored: (mirror + checks) / (cells + checks)
     shareCells the mirrored cells alone over the square: 496 / 1024                */
function sizesB(meta){
  const nsq = Math.ceil((meta.bytes * 8) / meta.K) || 1;
  const per = 3 * bits(meta.p) + (meta.confirm ? bits(meta.q) : 0);
  const checkBits = nsq * per, mirrorBits = nsq * (meta.L - meta.K), dataBits = meta.bytes * 8;
  return {squares: nsq, dataBytes: meta.bytes, checkBits, mirrorBits,
          checkBytes: Math.ceil(checkBits / 8),
          totalBytes: Math.ceil((nsq * meta.L + checkBits) / 8),
          overhead: dataBits ? (mirrorBits + checkBits) / dataBits : 0,
          share: (mirrorBits + checkBits) / (nsq * meta.L + checkBits),
          shareCells: (meta.L - meta.K) / meta.L,
          residuesPerSquare: meta.confirm ? 4 : 3, bitsPerSquare: per};
}
/* decoding one square.
     direct    cells whose side was named by the one residue that moved, plus
               Fold singles and partner copies
     searched  cells settled by enumerating side assignments (both residues
               moved) or by the Fold pair search                              */
function repairB(cells, check, code, opts){
  const {p, q, w, sig, confirm} = code, Qw = confirm ? code.Q.w : null;
  const region = code.region;
  let copied = 0;

  /* flagged: copy from the unflagged partner, no enumeration, any count.
     Fold cells and doubly-flagged pairs fall to v0's residue enumeration. */
  const flagged = new Set((opts && opts.erased) || []);
  for(let i = 0; i < cells.length; i++) if(cells[i] === -1) flagged.add(i);
  if(flagged.size){
    const rest = [];
    for(const j of flagged){
      if(region[j] !== FOLD && !flagged.has(sig[j])){ cells[j] = cells[sig[j]]; copied++; }
      else rest.push(j);
    }
    if(rest.length){
      const r = E.repairSquare(cells, check, code, {erased: rest});
      if(r.status !== "corrected") return {...r, note: "erasures: " + (r.note || "")};
      return {status: "corrected", fixed: copied + r.fixed, direct: copied + r.fixed, searched: 0, note: "copied+enumerated"};
    }
    if(E.verify(cells, check, code)) return {status: "corrected", fixed: copied, direct: copied, searched: 0, note: "copied"};
    /* a partner that was copied from was itself wrong: fall through */
  }

  const cur = E.regionResidues(cells, code);
  const dI = mod(cur[INNER] - check[INNER], p), dF = mod(cur[FOLD] - check[FOLD], p), dO = mod(cur[OUTER] - check[OUTER], p);
  const dQ = confirm ? mod(G.residue(cells, q) - check[3], q) : 0;
  /* mismatches name the damaged pairs */
  const M = [];
  for(const j of code.members[INNER]) if(cells[j] !== cells[sig[j]]) M.push(j);
  if(!dI && !dF && !dO){
    if(!dQ) return copied ? {status: "corrected", fixed: copied, direct: copied, searched: 0, note: "copied"}
                          : {status: "clean", fixed: 0, direct: 0, searched: 0};
    /* q moved and no region did: either noise below the residues, or a burst
       whose flips summed to 0 mod p (1/p of bursts -- the suite met one in
       800). With mismatches on the board the enumeration under q can still
       settle it; without, it is detected. */
    if(!M.length) return {status: "detected", fixed: 0, note: "confirm only"};
  }
  const fitsQ = plan => {
    if(!confirm) return true;
    let qq = 0; for(const e of plan) qq = mod(qq + e.d * Qw[e.i], q);
    return qq === dQ;
  };

  /* the Fold has no mirror: v0's path, single by table or pair by search */
  let foldCands = [[]];
  if(dF){
    const s = (code.tables[FOLD].get(dF) || []).filter(c => inBit(cells[c.i] - c.d)).map(c => [c]);
    if(s.length) foldCands = s;
    else {
      foldCands = [];
      const F = code.members[FOLD];
      for(let a = 0; a < F.length; a++) for(let b = a + 1; b < F.length; b++)
        for(const d1 of [1, -1]) for(const d2 of [1, -1]){
          if(!inBit(cells[F[a]] - d1) || !inBit(cells[F[b]] - d2)) continue;
          if(mod(d1 * w[F[a]] + d2 * w[F[b]], p) === dF) foldCands.push([{i: F[a], d: d1}, {i: F[b], d: d2}]);
        }
      if(!foldCands.length) return {status: "detected", fixed: 0, note: "fold"};
    }
  }
  const foldSearched = dF && foldCands.length && foldCands[0].length === 2;

  const innerFix = j => ({i: j, d: cells[j] - cells[sig[j]]});          // cells[j] := cells[sigma j]
  const outerFix = j => ({i: sig[j], d: cells[sig[j]] - cells[j]});     // cells[sigma j] := cells[j]
  const residualI = plan => { let r = dI; for(const e of plan) if(region[e.i] === INNER) r = mod(r - e.d * w[e.i], p); return r; };
  const residualO = plan => { let r = dO; for(const e of plan) if(region[e.i] === OUTER) r = mod(r - e.d * w[e.i], p); return r; };

  const sols = [];
  let mode = "direct";
  if(!M.length){
    if(dI || dO){
      /* no mismatch but the hemispheres moved: both members of one pair took
         the same hit. An Inner single whose partner shows the same d. */
      for(const c of (code.tables[INNER].get(dI) || [])){
        if(!inBit(cells[c.i] - c.d)) continue;
        if(mod(c.d * w[sig[c.i]], p) !== dO) continue;
        for(const f of foldCands){ const plan = [c, {i: sig[c.i], d: c.d}, ...f]; if(fitsQ(plan)) sols.push(plan); }
      }
      mode = "symmetric";
    } else for(const f of foldCands) if(fitsQ(f)) sols.push(f);
  } else if(!(dI && dO)){
    /* one residue moved: every mismatch is that side's. No enumeration. */
    const plan = M.map(dI ? innerFix : outerFix);
    if(residualI(plan) === 0 && residualO(plan) === 0)
      for(const f of foldCands){ const full = [...plan, ...f]; if(fitsQ(full)) sols.push(full); }
    if(!sols.length) mode = "enumerate";
  } else mode = "enumerate";

  if(mode === "enumerate" && M.length){
    if(M.length > 16) return {status: "detected", fixed: 0, note: "too many mismatches"};
    for(let a = 0; a < (1 << M.length) && sols.length < 2; a++){
      const plan = M.map((j, b) => (a >> b) & 1 ? outerFix(j) : innerFix(j));
      if(residualI(plan) !== 0 || residualO(plan) !== 0) continue;
      for(const f of foldCands){ const full = [...plan, ...f]; if(fitsQ(full)){ sols.push(full); if(sols.length > 1) break; } }
    }
  }
  if(sols.length !== 1) return {status: sols.length ? "ambiguous" : "detected", fixed: 0,
                                note: sols.length ? "mismatch" : "unrepaired", mismatches: M.length};
  const plan = sols[0];
  for(const e of plan) cells[e.i] -= e.d;
  const searched = (mode === "enumerate" ? M.length : 0) + (foldSearched ? 2 : 0);
  return {status: "corrected", fixed: plan.length + copied, direct: plan.length + copied - searched, searched,
          note: mode, mismatches: M.length};
}
function decodeB(payload, opts){
  const {squares, checks, meta} = payload;
  const code = payload.code || makeCodeB(meta.N, {confirm: meta.confirm});
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, searched: 0};
  for(let s = 0; s < squares.length; s++){
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairB(squares[s], checks[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed; tally.direct += r.direct || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  return {bytes: toBytes528(squares, code, meta.bytes), ...tally};
}

/* =========================================================================
   (c)  sigma as a self-inverse interleaver: store cells o sigma, check with v0
   ========================================================================= */
function makeCodeC(N, opts){
  const base = E.makeCode(N, opts);
  return {...base, arm: "c", sig: sigmaTable(N)};
}
/* the stored (physical) square is the logical one permuted by sigma; v0's
   four checks are taken on the logical square. encode o encode = id. */
const permuteC = (cells, code) => sigmaSquare(cells, code.sig);
function encodeC(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCodeC(N, opts);
  const logical = G.toCells(bytes, code.L);
  return {squares: logical.map(c => permuteC(c, code)), checks: logical.map(c => E.checksFor(c, code)), code,
          meta: {arm: "c", N, L: code.L, p: code.p, q: code.q, confirm: code.confirm, bytes: bytes.length}};
}
/* repair a physical square in place: un-permute, run v0, re-permute. A
   flagged physical cell j sits at logical index sigma(j). */
function repairC(cells, check, code, opts){
  const logical = permuteC(cells, code);
  const o = opts && opts.erased ? {...opts, erased: opts.erased.map(j => code.sig[j])} : opts;
  const r = E.repairSquare(logical, check, code, o);
  const back = permuteC(logical, code);
  for(let j = 0; j < cells.length; j++) cells[j] = back[j];
  return r;
}
function decodeC(payload, opts){
  const {squares, checks, meta} = payload;
  const code = payload.code || makeCodeC(meta.N, {confirm: meta.confirm});
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, searched: 0};
  for(let s = 0; s < squares.length; s++){
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairC(squares[s], checks[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed; tally.direct += r.direct || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  return {bytes: G.toBytes(squares.map(c => permuteC(c, code)), meta.L, meta.bytes), ...tally};
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {partnerOf, partnerRC, sigmaTable, sigmaSquare,
                    makeCodeA, checksForA, sigmaResidue, encodeA, decodeA, repairA, verifyA, sizesA, pairTable, lookupPairs, peelPairs, decodeRange, range,
                    makeCodeB, toCells528, toBytes528, encodeB, decodeB, repairB, sizesB,
                    makeCodeC, permuteC, encodeC, decodeC, repairC,
                    INNER, FOLD, OUTER, NAMES: E.NAMES};
