/* eggSo v3 -- outside the square.
 *
 * Not part of the site. The seventeenth codec experiment and the fourth in the
 * fold-native lineage. v0 used the fold's partition, v1 its symmetry, v2 its
 * alphabet's slack -- and all three lived INSIDE one square with ONE BIT to a
 * cell. This round changes both of those, because neither was ever a choice
 * anybody made; they were inherited from codegg-v1's first line.
 *
 *   (a) THE RADIX ARM. A cell holds a digit in 0..A-1 and the square is that
 *       number in base A: V = sum cell_i * A^(L-1-i). An error d at cell i
 *       moves V by d * A^(L-1-i), and one prime injective over
 *       {d * A^k : d in +-1..A-1, k < L} still names the cell, the sign and
 *       the size. Everything else is v0: three region residues, the
 *       confirming q, and the amendment's per-candidate confirm.
 *       The point is not elegance. It is that ONE CORRUPTED BYTE -- the unit
 *       real storage actually loses -- is eight cells in one row of a bit
 *       square and no arm in this lineage corrects it, and is ONE CELL of a
 *       byte square, named by its own syndrome.
 *
 *   (b) THE FILE-SCALE FOLD. v1(c)'s verdict was "sigma never leaves its
 *       square." Real damage does. Here sigma acts on the whole file:
 *          artifact[k] = source[sigma(k)],  sigma(j) = M-1 - ((j mod n)*n + floor(j/n))
 *       with n = ceil(sqrt(M)) over the file's own length. It is an
 *       involution, so the encoder IS the decoder and there is no table, and
 *       it stores nothing: the artifact is the size of the source. A
 *       contiguous wound in the artifact becomes a scattered set of source
 *       positions whose addresses are known -- thin erasures in many blocks
 *       instead of the total loss of a few. Truncation becomes the same
 *       injury as a scratch, given the original length.
 *
 * What is borrowed, named where it lives:
 *   - the region rule is eggSo-v0's regionOf, required from
 *     ../eggSo-v0/eggso.js, which asserts it against stalk.js:118 itself.
 *   - the residue, the injectivity-by-enumeration discipline and the
 *     modulus search are codegg-v1's (codegg.js:55-74), generalised here from
 *     radix 2 to radix A. The per-candidate confirm is codegg.js:204-206,
 *     which eggSo-v0 now also carries as its amendment.
 *   - the anti-transpose is eggSo-v1's partnerOf, which is index.html:398.
 * Nothing is copied that could be required.
 */
const G = require(__dirname + "/../codegg-v1/codegg.js");
const E = require(__dirname + "/../eggSo-v0/eggso.js");
const V1 = require(__dirname + "/../eggSo-v1/eggso1.js");
const {INNER, FOLD, OUTER} = E;

const mod = (x, m) => ((x % m) + m) % m;
const bitsOf = m => Math.ceil(Math.log2(m));

/* ---- moduli, radix A -----------------------------------------------------
   codegg-v1's property, widened: the 2(A-1)L values {d * A^k mod m} must be
   pairwise distinct, so a syndrome names one place, one sign and one size.
   Two tests, and the suite runs both: the ratio test below (fast -- A^k must
   never land within +-(A-1) of 0 after scaling by any d) and the honest
   enumeration, which is what codegg-v1 does and what is never taken on faith. */
function injectiveFor(m, L, A){
  let pow = 1 % m;
  for(let k = 0; k < L; k++){
    if(k) for(let d = 1; d < A; d++){
      const v = (d * pow) % m;
      if(v <= A - 1 || v >= m - (A - 1)) return false;
    }
    pow = (pow * A) % m;
  }
  return true;
}
function injectiveByEnumeration(m, L, A){
  const seen = new Set();
  let pow = 1 % m;
  for(let k = 0; k < L; k++){
    for(let d = 1; d < A; d++){
      const v = (d * pow) % m, neg = (m - v) % m;
      if(v === 0 || seen.has(v) || seen.has(neg) || v === neg) return false;
      seen.add(v); seen.add(neg);
    }
    pow = (pow * A) % m;
  }
  return true;
}
const isPrime = n => { if(n < 2) return false; for(let d = 2; d * d <= n; d++) if(n % d === 0) return false; return true; };
function pickModulus(L, A, avoid){
  for(let m = 2 * (A - 1) * L + 1; ; m += 2)
    if(isPrime(m) && (!avoid || avoid.indexOf(m) < 0) && injectiveFor(m, L, A)) return m;
}
/* modular inverse, for solving an erasure rather than enumerating it */
function inv(a, m){
  let [old_r, r] = [mod(a, m), m], [old_s, s] = [1, 0];
  while(r){ const qq = Math.floor(old_r / r); [old_r, r] = [r, old_r - qq * r]; [old_s, s] = [s, old_s - qq * s]; }
  return mod(old_s, m);
}

/* ---- the code ------------------------------------------------------------
   One syndrome table for the whole square, not one per region: injectivity is
   global, so a syndrome names at most one (cell, delta) and the region is a
   property of the cell. At A = 256, N = 32 that is an 8,030,879-entry Int32
   array -- 32 MB, one allocation, O(1) lookup -- where a Map of 760,000
   entries per region would be three allocations and a hash on every probe. */
function encodeCand(i, d, A){ const slot = d > 0 ? d - 1 : (A - 1) + (-d) - 1; return i * (2 * (A - 1)) + slot + 1; }
function decodeCand(c, A){ const w = 2 * (A - 1), slot = (c - 1) % w, i = ((c - 1) - slot) / w;
  return {i, d: slot < A - 1 ? slot + 1 : -(slot - (A - 1) + 1)}; }

/* The five (p, q) pairs this round uses, found by pickModulus during planning
   and recorded here so the ring does not spend nineteen seconds a file
   rediscovering them. tools/eggso3.test.js re-derives every one by search AND
   re-verifies it by enumeration; a cached constant that the search does not
   reproduce is a failure, not a shortcut. */
const PRIMES = {
  "2:1024":    [2053, 2063],
  "16:256":    [17627, 19429],
  "16:1024":   [61381, 65831],
  "256:256":   [2265761, 2288267],
  "256:1024":  [8030879, 8035021],
};
function makeCode(N, A, opts){
  const L = N * N, cached = PRIMES[`${A}:${L}`];
  const p = (opts && opts.p) || (cached ? cached[0] : pickModulus(L, A));
  const q = (opts && opts.q) || (cached ? cached[1] : pickModulus(L, A, [p]));
  const w = new Float64Array(L), wq = new Float64Array(L);
  let pw = 1 % p, pq = 1 % q;
  for(let i = L - 1; i >= 0; i--){ w[i] = pw; pw = (pw * A) % p; wq[i] = pq; pq = (pq * A) % q; }
  const region = new Int8Array(L), members = [[], [], []];
  for(let j = 0; j < L; j++){ const k = E.regionOf(Math.floor(j / N), j % N, N); region[j] = k; members[k].push(j); }
  const table = new Int32Array(p);
  for(let i = 0; i < L; i++) for(let d = 1; d < A; d++){
    const s = (d * w[i]) % p;
    table[s] = encodeCand(i, d, A);
    table[p - s] = encodeCand(i, -d, A);
  }
  const winv = new Float64Array(L), wqinv = new Float64Array(L);
  for(let i = 0; i < L; i++){ winv[i] = inv(w[i], p); wqinv[i] = inv(wq[i], q); }
  return {N, L, A, p, q, w, wq, winv, wqinv, region, members, table,
          digitBits: Math.log2(A), blockBytes: L * Math.log2(A) / 8};
}
const lookup = (code, s) => { const c = code.table[s]; return c ? decodeCand(c, code.A) : null; };

function residue(cells, code, m, w){
  let acc = 0;
  for(let i = 0; i < code.L; i++) if(cells[i]) acc = (acc + cells[i] * w[i]) % m;
  return acc;
}
function regionResidues(cells, code){
  const out = [0, 0, 0], {region, w, p, L} = code;
  for(let i = 0; i < L; i++) if(cells[i]) out[region[i]] = (out[region[i]] + cells[i] * w[i]) % p;
  return out;
}
const checksFor = (cells, code) => [...regionResidues(cells, code), residue(cells, code, code.q, code.wq)];

/* ---- bytes <-> digits ----------------------------------------------------
   A is a power of two, so a cell is a fixed field of the byte stream: 1 bit,
   4 bits or 8. The last block is padded with zeros, which is what the site
   does too (spec.md:64, "leftover cells are padded green"). */
function toCells(bytes, code){
  const {L, digitBits} = code, perSquare = L * digitBits / 8, squares = [];
  const total = Math.ceil(bytes.length / perSquare) || 1;
  for(let s = 0; s < total; s++){
    const cells = digitBits === 8 ? new Int16Array(L) : new Int8Array(L);
    for(let j = 0; j < L; j++){
      if(digitBits === 8){ const b = s * perSquare + j; cells[j] = b < bytes.length ? bytes[b] : 0; }
      else if(digitBits === 4){ const b = s * perSquare + (j >> 1); const v = b < bytes.length ? bytes[b] : 0; cells[j] = (j & 1) ? v & 15 : v >> 4; }
      else { const bit = s * L + j, b = bit >> 3; cells[j] = b < bytes.length ? (bytes[b] >> (7 - (bit & 7))) & 1 : 0; }
    }
    squares.push(cells);
  }
  return squares;
}
function toBytes(squares, code, byteLen){
  const {L, digitBits} = code, perSquare = L * digitBits / 8, out = new Uint8Array(byteLen);
  for(let s = 0; s < squares.length; s++){
    const cells = squares[s];
    for(let j = 0; j < L; j++){
      const v = cells[j] < 0 ? 0 : cells[j];
      if(digitBits === 8){ const b = s * perSquare + j; if(b < byteLen) out[b] = v; }
      else if(digitBits === 4){ const b = s * perSquare + (j >> 1); if(b < byteLen) out[b] |= (j & 1) ? (v & 15) : (v & 15) << 4; }
      else { const bit = s * L + j, b = bit >> 3; if(b < byteLen && v) out[b] |= 1 << (7 - (bit & 7)); }
    }
  }
  return out;
}
/* which cells of which square hold source byte b */
function cellsOfByte(b, code){
  const {L, digitBits} = code, perSquare = L * digitBits / 8;
  const s = Math.floor(b / perSquare), off = b - s * perSquare;
  if(digitBits === 8) return {square: s, cells: [off]};
  if(digitBits === 4) return {square: s, cells: [off * 2, off * 2 + 1]};
  return {square: s, cells: [0, 1, 2, 3, 4, 5, 6, 7].map(k => off * 8 + k)};
}

/* ---- decoding one square -------------------------------------------------
   Order: erasures first (positions known -> SOLVED, not enumerated), then
   alphabet-valid singles per hurt region combined and put to q per candidate
   (codegg.js:204-206, eggSo-v0's amendment), then -- when the radix is small
   enough for the search to be worth running -- one in-region pair.
   Three outcomes, never conflated: corrected / detected / ambiguous. */
function repairSquare(cells, check, code, opts){
  const o = opts || {}, {p, q, w, wq, winv, region, A, L} = code;
  const inAlpha = v => v >= 0 && v < A;

  const F = [...new Set(o.erased || [])];
  if(F.length){
    /* an erasure at a known address is one unknown in its region's equation.
       Solve it: v = (check - base) / w[i] mod p. Enumerate only when the
       radix is small enough to make enumeration cheaper than refusing --
       at A = 256 there is no enumeration to be had (256^k). */
    const base = cells.slice();
    for(const i of F) base[i] = 0;
    const baseR = regionResidues(base, code), baseQ = residue(base, code, q, wq);
    const need = [0, 1, 2].map(k => mod(check[k] - baseR[k], p));
    const byRegion = [[], [], []];
    for(const i of F) byRegion[region[i]].push(i);
    const plan = [];
    for(let k = 0; k < 3; k++){
      const Fk = byRegion[k];
      if(!Fk.length){ if(need[k] !== 0) return {status: "detected", fixed: 0, note: "erasures+error"}; continue; }
      if(Fk.length === 1){
        const v = mod(need[k] * winv[Fk[0]], p);
        if(!inAlpha(v)) return {status: "detected", fixed: 0, note: "erasure out of alphabet"};
        plan.push({i: Fk[0], v}); continue;
      }
      const space = Math.pow(A, Fk.length);
      if(space > 262144) return {status: "detected", fixed: 0, note: "too many erasures"};
      const dig = new Int32Array(Fk.length), hits = [];
      for(let n = 0; n < space && hits.length < 2; n++){
        let r = 0;
        for(let j = 0; j < Fk.length; j++) if(dig[j]) r = (r + dig[j] * w[Fk[j]]) % p;
        if(r === need[k]) hits.push(Int32Array.from(dig));
        for(let j = 0; j < Fk.length; j++){ if(++dig[j] < A) break; dig[j] = 0; }
      }
      if(!hits.length) return {status: "detected", fixed: 0, note: "erasures"};
      if(hits.length > 1) return {status: "ambiguous", fixed: 0, note: "erasures"};
      for(let j = 0; j < Fk.length; j++) plan.push({i: Fk[j], v: hits[0][j]});
    }
    let rq = baseQ;
    for(const e of plan) rq = (rq + e.v * wq[e.i]) % q;
    if(rq !== check[3]) return {status: "detected", fixed: 0, note: "erasures failed confirm"};
    for(const i of F) cells[i] = 0;
    for(const e of plan) cells[e.i] = e.v;
    return {status: "corrected", fixed: F.length, direct: F.length, searched: 0, note: "erasures solved"};
  }

  const cur = regionResidues(cells, code);
  const delta = [0, 1, 2].map(k => mod(cur[k] - check[k], p));
  const curQ = residue(cells, code, q, wq), dQ = mod(curQ - check[3], q);
  const hurt = [0, 1, 2].filter(k => delta[k] !== 0);
  if(!hurt.length) return dQ ? {status: "detected", fixed: 0, note: "confirm only"}
                             : {status: "clean", fixed: 0, direct: 0, searched: 0};

  /* one candidate per hurt region -- injectivity makes it at most one -- and
     the whole plan must satisfy q before a cell is touched */
  const singles = hurt.map(k => {
    const c = lookup(code, delta[k]);
    return c && region[c.i] === k && inAlpha(cells[c.i] - c.d) ? c : null;
  });
  if(singles.every(Boolean)){
    let rq = 0;
    for(const c of singles) rq = mod(rq + c.d * wq[c.i], q);
    if(rq === dQ){
      for(const c of singles) cells[c.i] -= c.d;
      return {status: "corrected", fixed: singles.length, direct: singles.length, searched: 0, note: "single"};
    }
  }
  /* one region holds two errors: peel every first error and ask whether the
     remainder is a valid second, with q inside the loop. At A = 256 the peel
     is 496 x 510 probes per region, which is why it is opt-out. */
  if(o.doubles !== false){
    const sols = [];
    for(let x = 0; x < hurt.length && sols.length < 2; x++){
      const kp = hurt[x], rest = hurt.filter((_, y) => y !== x);
      if(rest.some((k, y) => !singles[hurt.indexOf(k)])) continue;
      const fixed = rest.map(k => singles[hurt.indexOf(k)]);
      let rq0 = dQ;
      for(const c of fixed) rq0 = mod(rq0 - c.d * wq[c.i], q);
      const seen = new Set();
      for(const i1 of code.members[kp]){
        for(let d1 = -(A - 1); d1 <= A - 1 && sols.length < 2; d1++){
          if(!d1 || !inAlpha(cells[i1] - d1)) continue;
          const r = mod(delta[kp] - d1 * w[i1], p);
          if(!r) continue;
          const c = lookup(code, r);
          if(!c || c.i === i1 || region[c.i] !== kp || !inAlpha(cells[c.i] - c.d)) continue;
          if(mod(d1 * wq[i1] + c.d * wq[c.i], q) !== rq0) continue;
          const key = c.i < i1 ? `${c.i},${c.d},${i1},${d1}` : `${i1},${d1},${c.i},${c.d}`;
          if(seen.has(key)) continue;
          seen.add(key); sols.push([...fixed, {i: i1, d: d1}, c]);
        }
        if(sols.length > 1) break;
      }
    }
    if(sols.length === 1){
      for(const e of sols[0]) cells[e.i] -= e.d;
      return {status: "corrected", fixed: sols[0].length, direct: sols[0].length - 2, searched: 2, note: "pair"};
    }
    if(sols.length > 1) return {status: "ambiguous", fixed: 0, note: "pair"};
  }
  return {status: "detected", fixed: 0, note: "unrepaired"};
}

/* ---- (b) the file-scale fold --------------------------------------------
   sigma over the file's own length. n = ceil(sqrt(M)); indices past M-1 have
   no partner inside the file and are left where they are, which keeps the map
   a permutation of 0..M-1 and keeps it an involution. eggSo-v1's partnerOf is
   the same arithmetic at square scale (index.html:398). */
function fileSigma(M){
  const n = Math.ceil(Math.sqrt(M)), s = new Int32Array(M);
  for(let j = 0; j < M; j++){ const t = V1.partnerOf(j, n); s[j] = t < M ? t : j; }
  return s;
}
function scatter(bytes, sig){
  const out = new Uint8Array(bytes.length);
  for(let k = 0; k < bytes.length; k++) out[k] = bytes[sig[k]];
  return out;
}

/* ---- the artifact --------------------------------------------------------
   header (10 B) | checks | payload.  Checks come FIRST so a truncation takes
   payload and leaves the checks that repair it -- the same reasoning the
   codegg line puts its ribs at the front for. */
const MAGIC = 0xE993;
function encode(bytes, opts){
  const N = (opts && opts.N) || 32, A = (opts && opts.A) || 2;
  const code = (opts && opts.code) || makeCode(N, A, opts);
  const useSigma = !!(opts && opts.sigma);
  const squares = toCells(bytes, code);
  const checks = squares.map(c => checksFor(c, code));
  const cb = Math.ceil((3 * bitsOf(code.p) + bitsOf(code.q)) / 8);
  const head = new Uint8Array(10);
  head[0] = MAGIC >> 8; head[1] = MAGIC & 0xff; head[2] = N; head[3] = Math.log2(A);
  head[4] = useSigma ? 1 : 0;
  head[5] = (bytes.length >>> 24) & 0xff; head[6] = (bytes.length >>> 16) & 0xff;
  head[7] = (bytes.length >>> 8) & 0xff; head[8] = bytes.length & 0xff; head[9] = cb;
  const checkBytes = new Uint8Array(checks.length * cb);
  checks.forEach((c, s) => {
    let acc = 0n, bits = 0n;
    const widths = [bitsOf(code.p), bitsOf(code.p), bitsOf(code.p), bitsOf(code.q)];
    c.forEach((r, k) => { acc = (acc << BigInt(widths[k])) | BigInt(r); bits += BigInt(widths[k]); });
    acc <<= BigInt(cb * 8) - bits;
    for(let i = cb - 1; i >= 0; i--){ checkBytes[s * cb + i] = Number(acc & 0xFFn); acc >>= 8n; }
  });
  const payload = useSigma ? scatter(bytes, fileSigma(bytes.length)) : bytes;
  const art = new Uint8Array(head.length + checkBytes.length + payload.length);
  art.set(head, 0); art.set(checkBytes, head.length); art.set(payload, head.length + checkBytes.length);
  return {artifact: art, squares, checks, code,
          meta: {N, A, sigma: useSigma, bytes: bytes.length, squares: squares.length, cb, headerBytes: head.length}};
}
function readChecks(checkBytes, s, cb, code){
  const widths = [bitsOf(code.p), bitsOf(code.p), bitsOf(code.p), bitsOf(code.q)];
  let acc = 0n;
  for(let i = 0; i < cb; i++) acc = (acc << 8n) | BigInt(checkBytes[s * cb + i]);
  acc >>= BigInt(cb * 8) - BigInt(widths.reduce((a, b) => a + b, 0));
  const out = [];
  for(let k = 3; k >= 0; k--){ out[k] = Number(acc & ((1n << BigInt(widths[k])) - 1n)); acc >>= BigInt(widths[k]); }
  return out;
}
/* decode an artifact. `wound` is {at, len} in ARTIFACT coordinates for an
   addressed injury; a truncated artifact is detected from the header's length
   and treated as a wound running to the end. Both become erasures at known
   source addresses -- which is the whole of arm (b). */
function decode(art, opts){
  const o = opts || {};
  if(art.length < 10 || art[0] !== (MAGIC >> 8) || art[1] !== (MAGIC & 0xff)) return {ok: false, note: "no header"};
  const N = art[2], A = Math.pow(2, art[3]), useSigma = !!art[4];
  const byteLen = (art[5] << 24 >>> 0) + (art[6] << 16) + (art[7] << 8) + art[8], cb = art[9];
  const code = o.code || makeCode(N, A, o);
  const nsq = Math.ceil(byteLen / code.blockBytes) || 1;
  const checkStart = 10, payStart = checkStart + nsq * cb;
  const checkBytes = art.subarray(checkStart, payStart);
  const payload = new Uint8Array(byteLen);
  const have = Math.max(0, art.length - payStart);
  payload.set(art.subarray(payStart, payStart + Math.min(have, byteLen)));
  /* every artifact byte we cannot trust, as a source address */
  const lost = [];
  for(let k = have; k < byteLen; k++) lost.push(k);                       // truncation
  if(o.wound) for(let k = Math.max(0, o.wound.at - payStart); k < o.wound.at - payStart + o.wound.len && k < byteLen; k++) lost.push(k);
  const sig = useSigma ? fileSigma(byteLen) : null;
  const srcLost = sig ? lost.map(k => sig[k]) : lost;
  const src = useSigma ? scatter(payload, sig) : payload;                 // sigma is its own inverse
  const squares = toCells(src, code);
  const erasedBy = new Map();
  let worstBlock = 0; const perBlock = new Map();
  for(const b of srcLost){
    const {square, cells} = cellsOfByte(b, code);
    if(!erasedBy.has(square)) erasedBy.set(square, []);
    erasedBy.get(square).push(...cells);
    perBlock.set(square, (perBlock.get(square) || 0) + 1);
  }
  for(const v of perBlock.values()) worstBlock = Math.max(worstBlock, v);
  const tally = {clean: 0, corrected: 0, detected: 0, ambiguous: 0, fixed: 0, direct: 0, searched: 0,
                 blocksTouched: perBlock.size, worstBlock};
  for(let s = 0; s < squares.length && s < nsq; s++){
    const chk = readChecks(checkBytes, s, cb, code);
    const r = repairSquare(squares[s], chk, code, {erased: erasedBy.get(s), doubles: o.doubles});
    tally[r.status]++; tally.fixed += r.fixed; tally.direct += r.direct || 0; tally.searched += r.searched || 0;
  }
  tally.detected += tally.ambiguous;
  return {ok: true, bytes: toBytes(squares, code, byteLen), meta: {N, A, sigma: useSigma, bytes: byteLen, squares: nsq}, ...tally};
}
function sizes(meta, code){
  const per = 3 * bitsOf(code.p) + bitsOf(code.q);
  const checkBits = meta.squares * meta.cb * 8, dataBits = meta.bytes * 8;
  return {squares: meta.squares, blockBytes: code.blockBytes, bitsPerSquare: per, checkBits,
          checkBytes: meta.squares * meta.cb, headerBytes: meta.headerBytes || 10,
          totalBytes: meta.bytes + meta.squares * meta.cb + (meta.headerBytes || 10),
          overheadIdeal: per / (code.blockBytes * 8),
          overhead: dataBits ? (checkBits + 8 * (meta.headerBytes || 10)) / dataBits : 0};
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {injectiveFor, injectiveByEnumeration, pickModulus, inv, makeCode, lookup, PRIMES,
                    residue, regionResidues, checksFor, toCells, toBytes, cellsOfByte,
                    repairSquare, fileSigma, scatter, encode, decode, readChecks, sizes,
                    INNER, FOLD, OUTER, NAMES: E.NAMES};
