/* codegg v1 -- the value is the syndrome.
 *
 * codec-v1 next door treats the square as a bag of interchangeable symbols:
 * 4N-1 unweighted sums, position recovered by intersecting the ones that
 * disagree. It works, and it ignores the one thing this system actually is.
 *
 * The square is a NUMBER. Cell i weighs 2^-(i+1) (stalk.js:203), so every cell
 * has a distinct magnitude, and an error of delta d at cell i moves the value
 * by exactly d * 2^(L-1-i) -- one quantity that names the cell, the direction
 * and the size of the damage simultaneously. Position is already encoded in
 * place value. codec-v1 stored 127 sums per square to learn what the number
 * already knew. This codec stores the number mod two small primes: ~3 bytes
 * per 128-byte square, and the syndrome IS the error's address.
 *
 * The second thing codec-v1 could not have: push conserves the value while
 * changing every symbol, so a residue of the value survives respelling --
 * canonicalise, renormalise, recode however you like and the check still
 * holds. Every one of codec-v1's sums breaks under push. The system's
 * redundancy is respelling, and the only checks native to it are arithmetic.
 *
 * Lineage, in the site's own tradition of saying so: this is a residue /
 * arithmetic-error code -- Avizienis 1971, fault-tolerant arithmetic, the
 * same Avizienis whose 1961 signed digits the site already credits. The parts
 * are old; pointing the square's own place values at its own wounds is the
 * only thing local here.
 *
 * LAYOUT. Systematic: the payload is stored verbatim. A square is L = N*N
 * cells holding the payload's bits {0,+1}, row-major, most significant first,
 * the final square padded green. The chroma alphabet is the ERROR SPACE -- a
 * damaged cell may read -1 -- not a storage format, which is why this codec
 * pays none of the 2x the chroma path cost in codec-v1.
 *
 * CHECKS. V = sum of v_i * 2^(L-1-i), an integer far too large to hold, and
 * there is no need to hold it: store V mod p and V mod q for two primes chosen
 * so that {+-2^k mod m : k < L} are 2L distinct values. Then a single-cell
 * error's syndrome d * 2^(L-1-i) mod p is looked up in a table and hands back
 * (i, d): the cell, the sign, the magnitude. The q residue confirms it, and
 * the repaired cell must land back in the alphabet, or the repair is refused
 * and the square is reported detected rather than guessed at.
 *
 * WHAT THIS BUYS AND WHAT IT COSTS. ~2.3% overhead against codec-v1's 12.4%,
 * magnitude for free, erasure bursts corrected instead of detected, and
 * push-invariant checks. The honest cost: residues have a silent floor --
 * random multi-error garbage passes both with probability ~1/(p*q) -- and a
 * multi-error syndrome can imitate a single error. codec-v1's matching rule
 * had neither failure mode. Both are measured in tools/codegg.test.js and
 * reported in the README, not hidden.
 */

/* ---- moduli ----
   The whole scheme rests on one property: the 2L values {+-2^k mod m} must be
   pairwise distinct, so that a syndrome names one place and one sign. Search
   upward from 2L+1 and verify by enumeration -- the test suite re-verifies the
   same way, so the property is never taken on faith. */
function isPrime(n){
  if(n < 2) return false;
  for(let d = 2; d * d <= n; d++) if(n % d === 0) return false;
  return true;
}
function injectiveFor(m, L){
  const seen = new Set();
  let pow = 1 % m;
  for(let k = 0; k < L; k++){
    const neg = (m - pow) % m;
    if(pow === 0 || seen.has(pow) || seen.has(neg) || pow === neg) return false;
    seen.add(pow); seen.add(neg);
    pow = (pow * 2) % m;
  }
  return true;
}
function pickModulus(L, avoid){
  for(let m = 2 * L + 1; ; m += 2)
    if(isPrime(m) && (!avoid || avoid.indexOf(m) < 0) && injectiveFor(m, L)) return m;
}

/* ---- residues ----
   Horner, left to right, exactly the reading order of hexValue -- the residue
   is the square's value mod m, nothing more. Cells may be -1 (error space,
   pushed spellings), so the step keeps itself non-negative. */
function residue(cells, m){
  let acc = 0;
  for(let i = 0; i < cells.length; i++) acc = (acc * 2 + cells[i] + m) % m;
  return acc;
}

/* the syndrome table: s = d * 2^(L-1-i) mod m  ->  every (i, d) that spells
   it. d = +-2 at cell i spells the same number as d = +-1 at cell i-1 --
   2 * 2^w = 2^(w+1) -- so a syndrome can have two readings, and the decoder
   settles them by which repair lands back inside the alphabet. */
function syndromeTable(m, L){
  const t = new Map();
  const w = new Array(L);                      // 2^(L-1-i) mod m
  let pow = 1 % m;
  for(let i = L - 1; i >= 0; i--){ w[i] = pow; pow = (pow * 2) % m; }
  for(let i = 0; i < L; i++)
    for(const d of [1, -1, 2, -2]){
      const s = ((d * w[i]) % m + m) % m;
      if(!t.has(s)) t.set(s, []);
      t.get(s).push({i, d});
    }
  return {t, w};
}

/* ---- the code, per file ---- */
function makeCode(N){
  const L = N * N;
  const p = pickModulus(L);
  const q = pickModulus(L, [p]);
  return {N, L, p, q, P: syndromeTable(p, L), Q: syndromeTable(q, L)};
}

function toCells(bytes, L){
  const squares = [];
  const total = Math.ceil((bytes.length * 8) / L) || 1;
  for(let s = 0; s < total; s++){
    const cells = new Int8Array(L);
    for(let j = 0; j < L; j++){
      const bit = s * L + j, B = bit >> 3;
      cells[j] = B < bytes.length ? (bytes[B] >> (7 - (bit & 7))) & 1 : 0;
    }
    squares.push(cells);
  }
  return squares;
}
function toBytes(squares, L, byteLen){
  const out = new Uint8Array(byteLen);
  for(let bit = 0; bit < byteLen * 8; bit++){
    const cells = squares[Math.floor(bit / L)];
    if(cells[bit % L] === 1) out[bit >> 3] |= 1 << (7 - (bit & 7));
  }
  return out;
}

function encode(bytes, opts){
  const N = (opts && opts.N) || 32;
  const code = (opts && opts.code) || makeCode(N);
  const squares = toCells(bytes, code.L);
  const checks = squares.map(c => [residue(c, code.p), residue(c, code.q)]);
  return {squares, checks, meta: {N, L: code.L, p: code.p, q: code.q, bytes: bytes.length}, code};
}

/* ---- sizes: overhead reported from the format, not asserted ---- */
function sizes(meta){
  const bits = m => Math.ceil(Math.log2(m));
  const nsq = Math.ceil((meta.bytes * 8) / meta.L) || 1;
  const checkBits = nsq * (bits(meta.p) + bits(meta.q));
  return {
    squares: nsq,
    dataBytes: meta.bytes,                     // systematic: payload verbatim
    checkBits,
    checkBytes: Math.ceil(checkBits / 8),
    totalBytes: meta.bytes + Math.ceil(checkBits / 8),
    ratio: meta.bytes ? (meta.bytes + Math.ceil(checkBits / 8)) / meta.bytes : 1,
    overhead: meta.bytes ? checkBits / (meta.bytes * 8) : 0,
  };
}

/* ---- decoding one square ----
   `alphabet` is the set a repaired (or assigned) cell must land in: {0,1}
   for stored bit-data, {-1,0,1} when the square may be a pushed respelling.
   Returns {status, fixed, note} with status one of
     clean | corrected | detected | ambiguous
   and never repairs on a guess: two consistent readings is `ambiguous`,
   which counts as detected -- honest, and cheaper than being wrong. */
function repairSquare(cells, check, code, opts){
  const bitsOnly = !opts || opts.alphabet !== "trit";
  const inAlpha = v => bitsOnly ? (v === 0 || v === 1) : (v >= -1 && v <= 1);
  const sp = (residue(cells, code.p) - check[0] + code.p) % code.p;
  const sq = (residue(cells, code.q) - check[1] + code.q) % code.q;

  /* sentinel erasures: under the bit alphabet a -1 cell is self-evident
     damage. Caller-flagged positions (a suspect burst) join the same list. */
  const flagged = new Set((opts && opts.erased) || []);
  if(bitsOnly) for(let i = 0; i < cells.length; i++) if(cells[i] === -1) flagged.add(i);

  if(flagged.size){
    const F = [...flagged];
    if(F.length > 16) return {status: "detected", fixed: 0, note: "too many erasures"};
    /* base: flagged cells zeroed, then try every {0,1} assignment against
       both residues. One survivor is a correction; several are ambiguity. */
    const base = cells.slice();
    for(const i of F) base[i] = 0;
    const rp0 = residue(base, code.p), rq0 = residue(base, code.q);
    const wp = F.map(i => code.P.w[i]), wq = F.map(i => code.Q.w[i]);
    const hits = [];
    for(let a = 0; a < (1 << F.length); a++){
      let rp = rp0, rq = rq0;
      for(let j = 0; j < F.length; j++) if(a & (1 << j)){ rp = (rp + wp[j]) % code.p; rq = (rq + wq[j]) % code.q; }
      if(rp === check[0] && rq === check[1]){ hits.push(a); if(hits.length > 1) break; }
    }
    if(hits.length === 1){
      for(let j = 0; j < F.length; j++) cells[F[j]] = (hits[0] >> j) & 1;
      return {status: "corrected", fixed: F.length, note: "erasures"};
    }
    return {status: hits.length ? "ambiguous" : "detected", fixed: 0, note: "erasures"};
  }

  if(sp === 0 && sq === 0) return {status: "clean", fixed: 0};

  /* single error: the syndrome names its own address. Up to two readings
     (d and 2d one place apart); each must satisfy the q residue and land the
     repaired cell back in the alphabet. */
  const single = [];
  for(const c of code.P.t.get(sp) || [])
    if(((c.d * code.Q.w[c.i]) % code.q + code.q) % code.q === sq
       && inAlpha(cells[c.i] - c.d)) single.push(c);
  if(single.length === 1){
    cells[single[0].i] -= single[0].d;
    return {status: "corrected", fixed: 1, note: "single"};
  }
  if(single.length > 1) return {status: "ambiguous", fixed: 0, note: "single"};

  /* double error, by search: peel every possible first error and ask whether
     what remains is a valid single. Accept only a unique solution. */
  if(!opts || opts.doubles !== false){
    const seen = new Set(); const sols = [];
    for(let i1 = 0; i1 < cells.length && sols.length < 2; i1++)
      for(const d1 of [1, -1, 2, -2]){
        if(!inAlpha(cells[i1] - d1)) continue;
        const rp = (sp - d1 * code.P.w[i1] % code.p + 2 * code.p) % code.p;
        const rq = (sq - d1 * code.Q.w[i1] % code.q + 2 * code.q) % code.q;
        if(rp === 0) continue;                  // that would be a single
        for(const c of code.P.t.get(rp) || []){
          if(c.i === i1) continue;
          if(((c.d * code.Q.w[c.i]) % code.q + code.q) % code.q !== rq) continue;
          if(!inAlpha(cells[c.i] - c.d)) continue;
          const key = c.i < i1 ? `${c.i},${c.d},${i1},${d1}` : `${i1},${d1},${c.i},${c.d}`;
          if(seen.has(key)) continue;
          seen.add(key); sols.push([{i: i1, d: d1}, c]);
          if(sols.length > 1) break;
        }
      }
    if(sols.length === 1){
      for(const e of sols[0]) cells[e.i] -= e.d;
      return {status: "corrected", fixed: 2, note: "double"};
    }
    if(sols.length > 1) return {status: "ambiguous", fixed: 0, note: "double"};
  }

  return {status: "detected", fixed: 0};
}

function decode(payload, opts){
  const {squares, checks, meta} = payload;
  const code = payload.code || makeCode(meta.N);
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0};
  for(let s = 0; s < squares.length; s++){
    const o = opts && opts.erased ? {...opts, erased: opts.erased.get ? opts.erased.get(s) : undefined} : opts;
    const r = repairSquare(squares[s], checks[s], code, o);
    tally[r.status]++; tally.fixed += r.fixed;
  }
  tally.detected += tally.ambiguous;            // ambiguity is honest detection
  return {bytes: toBytes(squares, meta.L, meta.bytes), ...tally};
}

/* verify without repairing: do the stored residues still describe the cells?
   This is the check that survives push -- the value does not move, so neither
   do its residues. */
function verify(cells, check, code){
  return residue(cells, code.p) === check[0] && residue(cells, code.q) === check[1];
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {pickModulus, injectiveFor, residue, syndromeTable, makeCode,
                    encode, decode, repairSquare, verify, sizes, toCells, toBytes};
