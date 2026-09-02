/* eggSo v0 -- the partition is the code.
 *
 * Not part of the site. The fourteenth codec experiment, and the first to use
 * the one construction the site cannot place: the fold's own three regions.
 *
 *   Inner + Fold + Outer = V            (stalk.js:118 regions(); spec.md)
 *
 * codegg-v1 stores one residue of the whole value, V mod p, and a single-cell
 * error names its own address through it. Two errors give one syndrome with
 * two unknowns and v1 falls back to search. eggSo stores the residue of EACH
 * REGION -- I mod p, F mod p, O mod p -- and runs v1's mechanism inside each.
 * Two errors in different regions are then two single errors, each named by
 * its own residue, with no search at all. That is the whole claim, and
 * PREDICTIONS.md says what it is expected to be worth before it is measured.
 *
 * What is borrowed, named where it lives:
 *   - the residue, the weight table, the modulus choice and its injectivity
 *     proof are codegg-v1's, required from ../codegg-v1/codegg.js rather than
 *     copied. Attribution: codegg-v1, itself an AN / residue arithmetic code.
 *   - the layout of bytes into a square is v1's toCells, row-major, so the two
 *     codecs can be run on identical squares and compared cell for cell.
 * What is the site's:
 *   - which cells belong to which region. regionOf() below is the one
 *     comparison stalk.js:118 makes, cited, and tools/eggso.test.js asserts it
 *     against stalk.js's own regions() so the two can never drift.
 *
 * The bit alphabet only. A stored bit can only be damaged by d = +-1, so one
 * prime injective over +-2^k separates every (cell, direction) and the
 * alphabet check settles the rest. Pushed spellings (d up to +-2) would need
 * v1's second prime per region; that is out of v0's scope and said so.
 */
const G = require(__dirname + "/../codegg-v1/codegg.js");

/* the fold: which side of the main anti-diagonal a cell sits on.
   stalk.js:118  (r + c < n - 1 ? inner : r + c === n - 1 ? fold : outer)  */
const INNER = 0, FOLD = 1, OUTER = 2;
const NAMES = ["inner", "fold", "outer"];
function regionOf(r, c, N){
  const s = r + c;
  return s < N - 1 ? INNER : s === N - 1 ? FOLD : OUTER;
}

/* ---- the code, per file ----------------------------------------------------
   One prime, v1's choice for this L. Per region: the member cells, and a
   syndrome table over those members only -- d * 2^(L-1-i) mod p for d = +-1.
   `confirm` adds one whole-square residue in a second prime, v1-style, as a
   check against a two-error in one region aliasing to a single. It costs a
   fourth residue; PREDICTIONS.md files the trade and versus.js measures it. */
function makeCode(N, opts){
  const L = N * N;
  const p = G.pickModulus(L);
  const {w} = G.syndromeTable(p, L);                 // w[i] = 2^(L-1-i) mod p
  const region = new Int8Array(L);
  const members = [[], [], []];
  for(let j = 0; j < L; j++){
    const k = regionOf(Math.floor(j / N), j % N, N);
    region[j] = k; members[k].push(j);
  }
  const tables = members.map(list => {
    const t = new Map();
    for(const i of list) for(const d of [1, -1]){
      const s = ((d * w[i]) % p + p) % p;
      if(!t.has(s)) t.set(s, []);
      t.get(s).push({i, d});
    }
    return t;
  });
  /* confirm defaults ON. Measured first with it off, as PREDICTIONS filed:
     one prime per region resolves a single error and nothing more. p is about
     2L, so a region's syndrome carries ~11 bits; the in-region double search
     saturates that space and 22% of same-region pairs aliased to a single and
     were "repaired" wrong (tools/eggso.test.js #9, measured-floor.json). The
     whole-square q residue refuses every one of those. It costs a fourth
     residue -- 4.69% against the 4.7% bar -- and it is not optional in
     practice, so it is not optional by default. */
  const confirm = !(opts && opts.confirm === false);
  const q = confirm ? G.pickModulus(L, [p]) : 0;
  return {N, L, p, q, confirm, w, region, members, tables,
          Q: confirm ? G.syndromeTable(q, L) : null};
}

/* ---- residues, one per region ---------------------------------------------
   sum over the region's cells of cell * 2^(L-1-i), mod p. The three add to
   v1's residue() of the whole square, which is the identity the test pins. */
function regionResidues(cells, code){
  const out = [0, 0, 0];
  for(let i = 0; i < code.L; i++){
    const v = cells[i];
    if(v) out[code.region[i]] = (out[code.region[i]] + v * code.w[i] + code.p * 2) % code.p;
  }
  return out;
}
function checksFor(cells, code){
  const r = regionResidues(cells, code);
  return code.confirm ? [...r, G.residue(cells, code.q)] : r;
}

function encode(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCode(N, opts);
  const squares = G.toCells(bytes, code.L);
  const checks = squares.map(c => checksFor(c, code));
  return {squares, checks, code,
          meta: {N, L: code.L, p: code.p, q: code.q, confirm: code.confirm, bytes: bytes.length}};
}

function sizes(meta){
  const bits = m => Math.ceil(Math.log2(m));
  const nsq = Math.ceil((meta.bytes * 8) / meta.L) || 1;
  const per = 3 * bits(meta.p) + (meta.confirm ? bits(meta.q) : 0);
  const checkBits = nsq * per;
  return {
    squares: nsq, dataBytes: meta.bytes, checkBits,
    checkBytes: Math.ceil(checkBits / 8),
    totalBytes: meta.bytes + Math.ceil(checkBits / 8),
    ratio: meta.bytes ? (meta.bytes + Math.ceil(checkBits / 8)) / meta.bytes : 1,
    overhead: meta.bytes ? checkBits / (meta.bytes * 8) : 0,
    residuesPerSquare: meta.confirm ? 4 : 3,
  };
}

/* ---- decoding one square ------------------------------------------------
   Returns {status, fixed, note, direct, searched, regions} with status one of
     clean | corrected | detected | ambiguous
   `direct` counts cells repaired by a region's own syndrome, no search;
   `searched` counts those that needed the two-error search inside one region.
   That split is the number PREDICTIONS.md bar B2 is about.

   Never repairs on a guess: two consistent readings is `ambiguous`, which is
   honest detection. With `confirm`, every proposed repair must also satisfy
   the whole-square q residue or it is refused as an alias. */
function repairSquare(cells, check, code, opts){
  const inAlpha = v => v === 0 || v === 1;
  const p = code.p;
  const cur = regionResidues(cells, code);
  const delta = [0, 1, 2].map(k => (cur[k] - check[k] + p) % p);
  const sqDelta = code.confirm
    ? (G.residue(cells, code.q) - check[3] + code.q) % code.q : 0;

  /* flagged erasures, per region: v1's brute force, but each region only has
     to enumerate its own flagged cells -- three small searches, not one big */
  const flagged = new Set((opts && opts.erased) || []);
  for(let i = 0; i < cells.length; i++) if(cells[i] === -1) flagged.add(i);
  if(flagged.size){
    const byRegion = [[], [], []];
    for(const i of flagged) byRegion[code.region[i]].push(i);
    const base = cells.slice();
    for(const i of flagged) base[i] = 0;
    const baseRes = regionResidues(base, code);

    /* Every assignment each region's own residue accepts. A region's residue
       is ~11 bits, so twelve flagged cells in one region leave about two
       readings standing -- which is why the first version of this, taking
       only a unique per-region reading, corrected 315 of 800 row bursts where
       v1 corrected 800. The readings are combined across regions and the
       whole-square q residue picks among them. Without confirm, only a set of
       readings that was already unique can be applied. */
    const hitsPer = [];
    for(let k = 0; k < 3; k++){
      const F = byRegion[k];
      if(!F.length){
        if(baseRes[k] !== check[k]) return {status: "detected", fixed: 0, note: "erasures+error"};
        hitsPer.push([0]); continue;
      }
      if(F.length > 16) return {status: "detected", fixed: 0, note: "too many erasures"};
      const hits = [];
      for(let a = 0; a < (1 << F.length); a++){
        let r = baseRes[k];
        for(let j = 0; j < F.length; j++) if(a & (1 << j)) r = (r + code.w[F[j]]) % p;
        if(r === check[k]){ hits.push(a); if(hits.length > 64) break; }
      }
      if(!hits.length) return {status: "detected", fixed: 0, note: "erasures"};
      hitsPer.push(hits);
    }
    const combos = hitsPer[0].length * hitsPer[1].length * hitsPer[2].length;
    if(combos > 1 && !code.confirm) return {status: "ambiguous", fixed: 0, note: "erasures"};
    if(combos > 8192) return {status: "detected", fixed: 0, note: "erasures: too many readings"};

    const apply = (dst, k, a) => { const F = byRegion[k]; for(let j = 0; j < F.length; j++) dst[F[j]] = (a >> j) & 1; };
    let survivor = null, count = 0;
    for(const a0 of hitsPer[0]) for(const a1 of hitsPer[1]) for(const a2 of hitsPer[2]){
      const trial = base.slice();
      apply(trial, 0, a0); apply(trial, 1, a1); apply(trial, 2, a2);
      if(code.confirm && G.residue(trial, code.q) !== check[3]) continue;
      count++; survivor = [a0, a1, a2];
      if(count > 1) break;
    }
    if(count !== 1) return {status: count ? "ambiguous" : "detected", fixed: 0, note: "erasures"};
    apply(cells, 0, survivor[0]); apply(cells, 1, survivor[1]); apply(cells, 2, survivor[2]);
    return {status: "corrected", fixed: flagged.size, note: "erasures", direct: flagged.size, searched: 0};
  }

  const hurt = [0, 1, 2].filter(k => delta[k] !== 0);
  if(!hurt.length){
    if(code.confirm && sqDelta !== 0) return {status: "detected", fixed: 0, note: "confirm only"};
    return {status: "clean", fixed: 0, direct: 0, searched: 0};
  }

  /* each damaged region is repaired on its own. The regions are disjoint, so
     the order does not matter and no region's repair can disturb another's. */
  const plan = [];                     // [{i, d}] to apply
  let direct = 0, searched = 0;
  for(const k of hurt){
    const s = delta[k];
    const single = (code.tables[k].get(s) || []).filter(c => inAlpha(cells[c.i] - c.d));
    if(single.length === 1){ plan.push(single[0]); direct++; continue; }
    if(single.length > 1) return {status: "ambiguous", fixed: 0, note: "single", regions: hurt.map(h => NAMES[h])};

    /* two errors inside one region: peel every first error among the region's
       members and ask whether the remainder is a valid single. v1's search,
       confined to one region's cells. */
    if(opts && opts.doubles === false) return {status: "detected", fixed: 0, note: "doubles off", regions: hurt.map(h => NAMES[h])};
    const seen = new Set(), sols = [];
    for(const i1 of code.members[k]){
      for(const d1 of [1, -1]){
        if(!inAlpha(cells[i1] - d1)) continue;
        const rest = ((s - d1 * code.w[i1]) % p + 2 * p) % p;
        if(rest === 0) continue;
        for(const c of code.tables[k].get(rest) || []){
          if(c.i === i1 || !inAlpha(cells[c.i] - c.d)) continue;
          const key = c.i < i1 ? `${c.i},${c.d},${i1},${d1}` : `${i1},${d1},${c.i},${c.d}`;
          if(seen.has(key)) continue;
          seen.add(key); sols.push([{i: i1, d: d1}, c]);
          if(sols.length > 1) break;
        }
        if(sols.length > 1) break;
      }
      if(sols.length > 1) break;
    }
    if(sols.length === 1){ plan.push(...sols[0]); searched += 2; continue; }
    return {status: sols.length ? "ambiguous" : "detected", fixed: 0,
            note: sols.length ? "double" : "unrepaired", regions: hurt.map(h => NAMES[h])};
  }

  /* the confirming residue, if carried, must agree with the whole plan before
     a single cell is touched -- an alias is refused, not applied */
  if(code.confirm){
    let rq = G.residue(cells, code.q);
    for(const e of plan) rq = ((rq - e.d * code.Q.w[e.i]) % code.q + 2 * code.q) % code.q;
    if(rq !== check[3]) return {status: "detected", fixed: 0, note: "failed confirm", regions: hurt.map(h => NAMES[h])};
  }
  for(const e of plan) cells[e.i] -= e.d;
  return {status: "corrected", fixed: plan.length, direct, searched,
          note: searched ? (direct ? "mixed" : "double") : "single", regions: hurt.map(h => NAMES[h])};
}

function decode(payload, opts){
  const {squares, checks, meta} = payload;
  const code = payload.code || makeCode(meta.N, {confirm: meta.confirm});
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, searched: 0};
  for(let s = 0; s < squares.length; s++){
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairSquare(squares[s], checks[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed;
    tally.direct += r.direct || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  return {bytes: G.toBytes(squares, meta.L, meta.bytes), ...tally};
}

function verify(cells, check, code){
  const r = regionResidues(cells, code);
  return r[0] === check[0] && r[1] === check[1] && r[2] === check[2]
      && (!code.confirm || G.residue(cells, code.q) === check[3]);
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {regionOf, makeCode, regionResidues, checksFor, encode, decode,
                    repairSquare, verify, sizes, INNER, FOLD, OUTER, NAMES};
