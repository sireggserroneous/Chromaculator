/* chromacode v1 -- the square as a product code.
 *
 * spec.md builds a square for a purely dimensional reason: n = ceil(sqrt(L)) is
 * the smallest one that holds the stalk. That shape turns out to be the
 * geometry of a product code, which gives the square an operational
 * justification it did not have.
 *
 * Measured before writing any of this:
 *
 *   - pushLeft is a bijection onto values (4096 values -> 4096 pushed forms),
 *     so the representation cannot compress. This code does not try.
 *   - canonicity detects 0 of 50,000 sign flips: "all greens trailing" survives
 *     any +1 <-> -1 change. That is the single most likely error in a
 *     colour-coded medium and the canonical form is blind to it.
 *
 * Row parity is not blind to it. A sign flip moves its row sum by 2, so the
 * parity rim catches and locates exactly the error the canonical form misses.
 * That is the whole reason this file exists.
 *
 * WHAT THE PARITY IS
 *
 * N rows + N columns + (2N-1) anti-diagonals = 4N-1 sums for N^2 cells, so the
 * overhead is (4N-1)/N^2 -- 64% at N=6 but 12.4% at N=32, which is why N is
 * large by default. Anti-diagonals are not decoration: cell weight in this
 * representation depends only on r+c, so the anti-diagonals ARE the place
 * values, and they buy real coding power.
 *
 *   1 error   row r, col c, diagonal d, with d == r+c as a consistency check.
 *   2 errors  row/col alone gives 2x2 = 4 candidates and can correct none of
 *             them. The diagonals pick the true pair out of the four.
 *   4 corners (r1,c1) (r1,c2) (r2,c1) (r2,c2) leaves every row and column sum
 *             unchanged -- the classic product-code blind spot. The diagonals
 *             see it unless r1-r2 == c1-c2 makes the middle two coincide.
 *
 * TWO ALPHABETS, ONE GEOMETRY
 *
 * The parity layer is written once against a small symbol interface so that the
 * cost of routing data through chroma cells is measured rather than assumed.
 * `byte` is the honest baseline: one byte per cell, parity by XOR. `chroma` is
 * the on-theme path: one byte becomes 8 signed-digit cells, 2 bits each, which
 * is a 2x expansion before parity is even added.
 *
 * A NOTE ON WHAT NOT TO REUSE
 *
 * hexSequence() is wrong for this. It pads to whole nibbles but discards the
 * leading-zero WIDTH, which is the non-injectivity already measured:
 * value(16) == value(256) == 1/16, and width survived a round trip in only
 * 2001 of 4001 cases. A codec cannot afford that, so bytes are expanded to a
 * fixed 8 bits here and pushLeft is applied to that fixed-width string.
 */

/* pushLeft lives in stalk.js. In the browser both files are plain scripts and
   it is simply a global by the time this runs. Under node's module scope it is
   not, so stalk.js is evaluated here -- the same eval the tools/ tests use,
   which is why stalk.js keeps its top-level declarations as `function`. */
if(typeof pushLeft === "undefined" && typeof require !== "undefined"){
  /* eslint-disable no-eval */
  eval(require("fs").readFileSync(require("path").join(__dirname, "..", "stalk.js"), "utf8"));
}

/* ---- the symbol interface ----
   Everything the parity layer needs to know about an alphabet. `add` and `sub`
   must form a group so that a located error can be repaired by subtracting the
   observed delta. */
const ALPHABETS = {
  /* one byte per cell, parity by XOR. The baseline the chroma path is measured
     against. XOR is its own inverse, so add and sub are the same function. */
  byte: {
    name: "byte",
    zero: 0,
    add: (a, b) => a ^ b,
    sub: (a, b) => a ^ b,
    symbolsPerByte: 1,
    bitsPerSymbol: 8,
    bitsPerSum: 8,
    /* one byte in, one symbol out */
    fromBytes: bytes => Array.from(bytes),
    toBytes: syms => Uint8Array.from(syms.map(s => s & 0xff)),
    /* every distinct wrong value a corrupted symbol could take */
    others: s => { const o = []; for(let v = 0; v < 256; v++) if(v !== s) o.push(v); return o; },
  },

  /* one byte -> 8 fixed-width bits -> pushLeft -> 8 signed-digit cells.
     Push is a value-preserving bijection at fixed length, so this round-trips.
     Parity is the integer sum over {-1,0,+1}, which lands in -N..N. */
  chroma: {
    name: "chroma",
    zero: 0,
    add: (a, b) => a + b,
    sub: (a, b) => a - b,
    symbolsPerByte: 8,
    bitsPerSymbol: 2,
    bitsPerSum: 0,                       // filled in per-N by sumBits()
    fromBytes(bytes){
      const out = [];
      for(const b of bytes){
        /* fixed 8 bits, MSB first -- no nibble padding, no width loss */
        const bits = [];
        for(let i = 7; i >= 0; i--) bits.push((b >> i) & 1);
        out.push(...pushLeft(bits));
      }
      return out;
    },
    toBytes(syms){
      const out = new Uint8Array(Math.ceil(syms.length / 8));
      for(let i = 0; i + 8 <= syms.length; i += 8){
        /* undo push by reading the value back at fixed width 8 */
        let v = 0;
        for(let j = 0; j < 8; j++) v += syms[i + j] * (1 << (7 - j));
        out[i / 8] = v & 0xff;
      }
      return out;
    },
    others: s => [1, 0, -1].filter(v => v !== s),
  },
};

/* bits needed for one parity sum, which is alphabet- and N-dependent */
function sumBits(alph, N){
  if(alph.name === "byte") return 8;
  return Math.ceil(Math.log2(2 * N + 1));      // chroma sums land in -N..N
}

/* the smallest square holding L cells. squareFor in stalk.js is a const arrow
   and does not survive the tests' eval, so it is redefined here -- the same
   thing every test in tools/ does. */
function squareSide(L){ return Math.max(1, Math.ceil(Math.sqrt(L))); }

/* ---- the three parity vectors ----
   One function, used by both the encoder and the decoder, so there is no way
   for the two sides to disagree about what a sum means. */
function parities(grid, alph, N){
  const rows = new Array(N).fill(alph.zero);
  const cols = new Array(N).fill(alph.zero);
  const diags = new Array(2 * N - 1).fill(alph.zero);
  for(let r = 0; r < N; r++)
    for(let c = 0; c < N; c++){
      const v = grid[r][c];
      rows[r] = alph.add(rows[r], v);
      cols[c] = alph.add(cols[c], v);
      diags[r + c] = alph.add(diags[r + c], v);
    }
  return {rows, cols, diags};
}

/* which entries of two parity vectors disagree */
function mismatches(a, b){
  const out = [];
  for(let i = 0; i < a.length; i++) if(a[i] !== b[i]) out.push(i);
  return out;
}

/* ---- laying symbols into squares ---- */
function toSquares(syms, N){
  const per = N * N, out = [];
  for(let i = 0; i < syms.length; i += per){
    const chunk = syms.slice(i, i + per);
    const g = Array.from({length: N}, () => new Array(N).fill(0));
    chunk.forEach((v, j) => { g[Math.floor(j / N)][j % N] = v; });
    out.push(g);
  }
  return out;
}
function fromSquares(squares, N){
  const out = [];
  for(const g of squares) for(let r = 0; r < N; r++) for(let c = 0; c < N; c++) out.push(g[r][c]);
  return out;
}

/* ---- encode ----
   Returns the squares alongside the parity each was computed from, plus enough
   metadata that decode() needs nothing else. `count` is the true symbol length
   so the final partial square's padding can be discarded exactly. */
function encode(bytes, opts){
  const o = opts || {};
  const N = o.N || 32;
  const alph = typeof o.alphabet === "string" ? ALPHABETS[o.alphabet] : (o.alphabet || ALPHABETS.byte);
  if(!alph) throw new Error("unknown alphabet");
  const syms = alph.fromBytes(bytes);
  const squares = toSquares(syms, N);
  const parity = squares.map(g => parities(g, alph, N));
  return {
    squares, parity,
    meta: {N, alphabet: alph.name, count: syms.length, bytes: bytes.length,
           sumBits: sumBits(alph, N)},
  };
}

/* ---- sizes, so overhead is reported from the format rather than guessed ---- */
function sizes(meta){
  const {N, count, bytes} = meta;
  const alph = ALPHABETS[meta.alphabet];
  const squares = Math.ceil(count / (N * N));
  const dataBits = count * alph.bitsPerSymbol;
  const parityBits = squares * (4 * N - 1) * meta.sumBits;
  return {
    squares, dataBits, parityBits,
    dataBytes: Math.ceil(dataBits / 8),
    parityBytes: Math.ceil(parityBits / 8),
    totalBytes: Math.ceil((dataBits + parityBits) / 8),
    sourceBytes: bytes,
    ratio: bytes ? Math.ceil((dataBits + parityBits) / 8) / bytes : 0,
    parityOverhead: (4 * N - 1) / (N * N),
  };
}

/* ---- locate and repair one square ----
   `useDiags` exists so the test can measure what the anti-diagonals actually
   buy, by running the identical decoder without them.

   Returns {fixed, status}: how many cells were repaired, and one of
     "clean"          no parity disagreed
     "corrected"      every disagreement was resolved and parity now matches
     "detected"       something is wrong but it could not be placed
   "detected" is the honest outcome and much better than a silent wrong answer,
   which is what a code without enough parity would give. */
function repairSquare(grid, want, alph, N, useDiags){
  const got = parities(grid, alph, N);
  const badR = mismatches(want.rows, got.rows);
  const badC = mismatches(want.cols, got.cols);
  const badD = useDiags ? mismatches(want.diags, got.diags) : [];

  if(!badR.length && !badC.length && !badD.length) return {fixed: 0, status: "clean"};

  /* a disagreement only in the diagonals means the rows and columns cancelled:
     the 4-corner rectangle. Detected, not placeable from these sums alone. */
  if(!badR.length || !badC.length) return {fixed: 0, status: "detected"};

  /* candidates are the bad rows crossed with the bad columns. With diagonals
     available, keep only those whose r+c is itself a disagreeing diagonal --
     that is what turns an ambiguous 2x2 into a single answer. */
  let cand = [];
  for(const r of badR) for(const c of badC) cand.push([r, c]);
  if(useDiags && badD.length){
    const dset = new Set(badD);
    const kept = cand.filter(([r, c]) => dset.has(r + c));
    if(kept.length) cand = kept;
  }

  /* repair only when the candidate set is exactly one cell per bad row and per
     bad column -- a perfect matching. Anything else is ambiguous and gets
     reported rather than guessed at. */
  const rSeen = new Map(), cSeen = new Map();
  for(const [r, c] of cand){
    rSeen.set(r, (rSeen.get(r) || 0) + 1);
    cSeen.set(c, (cSeen.get(c) || 0) + 1);
  }
  const unique = cand.length === badR.length && cand.length === badC.length
    && [...rSeen.values()].every(n => n === 1) && [...cSeen.values()].every(n => n === 1);
  if(!unique) return {fixed: 0, status: "detected"};

  /* each located cell is off by exactly the delta its row sum reports */
  for(const [r, c] of cand){
    const rowNow = grid[r].reduce((a, b) => alph.add(a, b), alph.zero);
    const delta = alph.sub(want.rows[r], rowNow);
    grid[r][c] = alph.add(grid[r][c], delta);
  }

  const after = parities(grid, alph, N);
  const ok = !mismatches(want.rows, after.rows).length
    && !mismatches(want.cols, after.cols).length
    && (!useDiags || !mismatches(want.diags, after.diags).length);
  return {fixed: cand.length, status: ok ? "corrected" : "detected"};
}

/* ---- decode ----
   Repairs in place across every square and reports what happened, so a caller
   can distinguish a clean recovery from a detected-but-unrepaired one. */
function decode(payload, opts){
  const o = opts || {};
  const useDiags = o.useDiags !== false;
  const {meta, squares, parity} = payload;
  const alph = ALPHABETS[meta.alphabet];
  const N = meta.N;
  const tally = {clean: 0, corrected: 0, detected: 0, fixed: 0};
  for(let i = 0; i < squares.length; i++){
    const r = repairSquare(squares[i], parity[i], alph, N, useDiags);
    tally[r.status]++; tally.fixed += r.fixed;
  }
  const syms = fromSquares(squares, N).slice(0, meta.count);
  return {bytes: alph.toBytes(syms).slice(0, meta.bytes), ...tally};
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {ALPHABETS, encode, decode, parities, mismatches, sizes,
                    toSquares, fromSquares, repairSquare, squareSide, sumBits};
