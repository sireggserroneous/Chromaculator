/* stalk.js — the v2 core. written fresh; shares nothing with fold.js.
 *
 * A number is a stalk of cells. Cell 0 is A1, reserved: always green, weight
 * 2^0. Cell i after it weighs 2^-i, so reading left to right is reading
 * decimal precision — each further cell halves the step. The digits are the
 * integer's bits, smallest first.
 *
 * Commas fall on the anti-diagonals of the square the stalk folds into, so
 * the groups run 1, 2, 3, ... — the same cut the grid makes.
 */

const DIG = {P: 1, Z: 0, N: -1};
const cls   = v => v > 0 ? "b" : v < 0 ? "r" : "g";
const glyph = v => v > 0 ? "1" : v < 0 ? "−" : "0";

/* parse decimal, 0x…, or 0b…; a leading - negates. */
function parse(text){
  const t = String(text).trim().replace(/[_,\s]/g, "");
  if(!t) throw new Error("empty");
  const neg = t[0] === "-";
  const body = neg ? t.slice(1) : t;
  let v;
  if(/^0[xX][0-9a-fA-F]+$/.test(body))      v = BigInt(body);
  else if(/^0[bB][01]+$/.test(body))        v = BigInt(body);
  else if(/^\d+$/.test(body))               v = BigInt(body);
  else throw new Error(`cannot read "${text}"`);
  return {v, neg};
}

/* cell 0 is the reserved A1; then the bits, smallest first. a negative
   number is the same stalk with every colour flipped. */
function digitsOf(v, neg){
  if(v === 0n) return [0];
  const bits = v.toString(2).split("").reverse().map(c => (c === "1" ? 1 : 0));
  return [0, ...bits.map(b => b * (neg ? -1 : 1))];
}

/* arc lengths 1, 2, 3, … n, … 3, 2, 1 for an n x n square */
function arcs(n){
  const a = [];
  for(let i = 1; i <= n; i++) a.push(i);
  for(let i = n - 1; i >= 1; i--) a.push(i);
  return a;
}
/* the smallest square that holds this many cells */
const squareFor = len => Math.max(1, Math.ceil(Math.sqrt(len)));

/* `raw` is the stalk as written -- it simply stops when the bits run out.
   `cells` is the same stalk padded green out to a full square, which is only
   what the 2D grid needs. the 1D reading never shows padding. */
function stalkOf(v, neg){
  const raw = digitsOf(v, neg);
  const n = squareFor(raw.length);
  const cells = raw.slice();
  while(cells.length < n * n) cells.push(0);
  return {n, raw, cells};
}

/* push toward the coarse end: a lit cell steps left into a green and leaves
   its own sign flipped behind.  +1·2^-i  ==  +1·2^-(i-1)  −  1·2^-i
   the value never moves; only the colours do. */
function pushLeft(cells){
  const d = cells.slice();
  let moved = true;
  while(moved){
    moved = false;
    for(let i = d.length - 1; i > 0; i--)
      if(d[i] !== 0 && d[i - 1] === 0){ d[i - 1] = d[i]; d[i] = -d[i]; moved = true; }
  }
  return d;
}

/* exact value: cell i weighs 2^-i, so the whole stalk is over 2^(len-1) */
function valueOf(cells){
  const L = cells.length;
  if(!L) return {num: 0n, den: 1n};
  const den = 1n << BigInt(L - 1);
  let num = 0n;
  for(let i = 0; i < L; i++) num += BigInt(cells[i]) * (1n << BigInt(L - 1 - i));
  const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const d = g(num, den);
  return {num: num / d, den: den / d};
}
const fmt = f => f.den === 1n ? `${f.num}` : `${f.num}/${f.den}`;
const dec = f => Number(f.num) / Number(f.den);

/* split a stalk on the anti-diagonals */
function commas(cells, n){
  const out = [], lens = arcs(n);
  let k = 0;
  for(const L of lens){
    if(k >= cells.length) break;
    out.push(cells.slice(k, k + L));
    k += L;
  }
  if(k < cells.length) out.push(cells.slice(k));
  return out;
}

/* lay the stalk into the square, anti-diagonal by anti-diagonal, each read
   from the bottom-left corner upward — Hankel order. */
function cellOrder(n){
  const o = [];
  for(let d = 0; d <= 2 * (n - 1); d++)
    for(let r = Math.min(n - 1, d); r >= 0; r--){
      const c = d - r;
      if(c >= 0 && c < n) o.push([r, c]);
    }
  return o;
}
function square(cells, n){
  const g = Array.from({length: n}, () => Array(n).fill(0));
  cellOrder(n).forEach(([r, c], i) => { if(i < cells.length) g[r][c] = cells[i]; });
  return g;
}

/* the three regions, by where a cell sits relative to the main anti-diagonal */
function regions(cells, n){
  const inner = [], fold = [], outer = [];
  cellOrder(n).forEach(([r, c], i) => {
    const v = i < cells.length ? cells[i] : 0;
    const slot = {v, r, c, i, w: i};
    (r + c < n - 1 ? inner : r + c === n - 1 ? fold : outer).push(slot);
  });
  return {inner, fold, outer};
}

/* scientific form of an exact fraction, so a value with hundreds of digits
   still occupies one line. truncates rather than rounds — the exact figure
   is a click away, and a rounded mantissa beside an exact box would be two
   different numbers on screen. */
function sci(num, den, sig = 6){
  if(num === 0n) return {m: "0", e: 0, neg: false};
  const neg = num < 0n;
  const n = neg ? -num : num;
  const nl = n.toString().length, dl = den.toString().length;
  const k = sig + 2 + Math.max(0, dl - nl);
  const q = (n * 10n ** BigInt(k)) / den;
  const qs = q.toString();
  return {m: qs[0] + (sig > 1 ? "." + qs.slice(1, sig) : ""), e: qs.length - 1 - k, neg};
}
const SUPD = "⁰¹²³⁴⁵⁶⁷⁸⁹";
const sup = n => (n < 0 ? "⁻" : "") + String(Math.abs(n)).replace(/\d/g, d => SUPD[d]);
const sciText = f => {
  const s = sci(f.num, f.den);
  return s.m === "0" ? "0" : `${s.neg ? "−" : ""}${s.m}×10${sup(s.e)}`;
};
/* the denominator is always a power of two, so name it by its exponent */
const log2den = den => den.toString(2).length - 1;

/* ---- ring slots ----
   ring r holds |k| in [2^r, 2^(r+1)), and those stalk values are exactly the
   odd multiples of 2^-(r+1) — evenly spaced. so laying a ring out in value
   order is the same as spacing it evenly, and the integer at slot i is found
   by reversing the bits of 2i+1. */
function bitrev(x, bits){
  let o = 0;
  for(let b = 0; b < bits; b++) if(x >> b & 1) o |= 1 << (bits - 1 - b);
  return o;
}
const kAtSlot = (i, r) => bitrev(2*i + 1, r + 1);
const slotOfK = (k, r) => (bitrev(Math.abs(k), r + 1) - 1) / 2;

/* ---- the hex arrangement ----
   write k in hex, so the bits arrive already padded to a whole nibble, and lay
   them most-significant first, row by row, into the smallest square that holds
   them. the leading zeros of the hex form are the padding — nothing has to be
   reserved. cell i weighs 2^-(i+1), so the value is k / 2^(4*nibbles): still
   inside (0,1), but order-preserving, since the bits are no longer reversed. */
function hexSequence(v, neg){
  const b = v === 0n ? "" : v.toString(2);
  const nib = Math.ceil(b.length / 4) * 4;
  /* zero is a number and its stalk is one green cell, not none. `cells` always
     said so -- it pads to a 1x1 square -- while `raw` came back empty, and the
     two disagreeing gave hexProduct a 0x0 rectangle with foldAt = -1. */
  const raw = v === 0n ? [0] : b.padStart(nib, "0").split("")
    .map(c => (c === "1" ? 1 : 0) * (neg ? -1 : 1));
  const n = Math.max(1, Math.ceil(Math.sqrt(raw.length)));
  const cells = raw.slice();
  while(cells.length < n * n) cells.push(0);
  return {n, raw, cells};
}
function hexValue(digs){
  const L = digs.length;
  if(!L) return {num: 0n, den: 1n};
  const den = 1n << BigInt(L);
  let num = 0n;
  for(let i = 0; i < L; i++) num += BigInt(digs[i]) * (1n << BigInt(L - 1 - i));
  const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const d = g(num, den);
  return {num: num / d, den: den / d};
}
/* row major, unlike the Hankel fill — this arrangement reads left to right */
function rowMajor(cells, n){
  const g = Array.from({length: n}, () => Array(n).fill(0));
  for(let i = 0; i < cells.length && i < n*n; i++) g[Math.floor(i/n)][i % n] = cells[i];
  return g;
}
function hexRegions(cells, n){
  const inner = [], fold = [], outer = [];
  for(let i = 0; i < n*n; i++){
    const r = Math.floor(i/n), c = i % n;
    const slot = {v: cells[i] || 0, r, c, i, w: i + 1};   // the cell weighs 2^-w
    (r + c < n - 1 ? inner : r + c === n - 1 ? fold : outer).push(slot);
  }
  return {inner, fold, outer};
}

/* ---- the product grid ----
   Multiply two stalks cell by cell and the answer is a rectangle. A runs along
   the columns, B down the rows, so row r column c holds a_c * b_r and weighs
   2^-(c+1) * 2^-(r+1) = 2^-(r+c+2). Summing the whole rectangle is the product,
   exactly -- no carries, because nothing is ever added into the same place.

   A with m cells times B with n cells is therefore n rows by m columns: the
   operands keep their sides, so A*B and B*A are transposes of each other.

   Weight depends only on r+c, so the anti-diagonals are the place values, just
   as they are for a single stalk. The fold is the one through the last cell of
   the shorter operand -- r+c = L-1, L = min(m, n) -- which is the last
   anti-diagonal that still reaches both edges of the rectangle. */
function hexProduct(A, B){
  const cols = A.length, rows = B.length;
  const cells = [];
  for(let r = 0; r < rows; r++)
    for(let c = 0; c < cols; c++) cells.push(A[c] * B[r]);
  return {rows, cols, cells, foldAt: Math.min(rows, cols) - 1};
}
function productRegions(P){
  const inner = [], fold = [], outer = [];
  for(let i = 0; i < P.cells.length; i++){
    const r = Math.floor(i / P.cols), c = i % P.cols;
    const slot = {v: P.cells[i], r, c, i, w: r + c + 2};   // the cell weighs 2^-w
    (r + c < P.foldAt ? inner : r + c === P.foldAt ? fold : outer).push(slot);
  }
  return {inner, fold, outer};
}
/* exact, over 2^(rows+cols) — the largest weight any cell can carry */
function productValue(P){
  const D = P.rows + P.cols;
  let num = 0n;
  for(let i = 0; i < P.cells.length; i++){
    const v = P.cells[i];
    if(!v) continue;
    const r = Math.floor(i / P.cols), c = i % P.cols;
    num += BigInt(v) * (1n << BigInt(D - (r + c + 2)));
  }
  const den = 1n << BigInt(D);
  const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const d = g(num, den);
  return {num: num / d, den: den / d};
}
/* the grid read back out as a plain stalk, so a product can be an operand
   again. rows+cols cells, because that is what the value is over. */
function productDigits(P){
  const D = P.rows + P.cols;
  let num = 0n;
  for(let i = 0; i < P.cells.length; i++){
    const v = P.cells[i];
    if(!v) continue;
    const r = Math.floor(i / P.cols), c = i % P.cols;
    num += BigInt(v) * (1n << BigInt(D - (r + c + 2)));
  }
  const sgn = num < 0n ? -1 : 1, mag = num < 0n ? -num : num;
  return mag.toString(2).padStart(D, "0").split("").map(ch => ch === "1" ? sgn : 0);
}

/* ---- division: quotient, multiplier, remainder ----
   A/B will not fit a grid the way A*B does, and usually it will not fit at all
   -- most quotients repeat forever. Both problems go away by refusing to round.
   Run the division only as far as the grid allows and keep what is left over:

       A  =  2^e * Q * B  +  R

   holds exactly at every width, with nothing repeating and nothing dropped.

   Q is an ordinary stalk inside (-1,1), so it draws like every other number.
   The size lives in e, the multiplier on the boundary -- the shift that brings
   the quotient back inside, normalised so |2^-e * A/B| is in [1/2, 1) and the
   leading cell is never wasted. R is a stalk too, always dyadic and always
   inside (-1,1): it is exactly what the repeating digits would have been.

   Widening the grid grows Q and shrinks R; the identity never moves.

   The tableau -- the rows of B that got subtracted -- is hexProduct(B, Q): the
   same rectangle multiplication draws, only solved for rather than read off. */
function divide(A, B, W){
  const abs = x => x < 0n ? -x : x;
  const g = (a, b) => { a = abs(a); while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const red = (n, d) => { if(d < 0n){ n = -n; d = -d; } const h = g(n, d); return [n/h, d/h]; };
  const fa = hexValue(A), fb = hexValue(B);
  if(fb.num === 0n) return null;                       // nothing divides by green
  const [N, D] = red(fa.num * fb.den, fa.den * fb.num);
  /* the multiplier: the smallest e with |A/B| < 2^e */
  let e = 0;
  if(N !== 0n){
    e = abs(N).toString(2).length - D.toString(2).length + 1;
    const under = k => k >= 0 ? abs(N) < (D << BigInt(k)) : (abs(N) << BigInt(-k)) < D;
    while(!under(e)) e++;
    while(under(e - 1)) e--;
  }
  /* long division on what is left, which is now inside (-1,1) */
  let rn = e >= 0 ? N : N << BigInt(-e);
  const DD = e >= 0 ? (D << BigInt(e)) : D;
  const Q = [];
  for(let i = 0; i < W; i++){
    rn *= 2n;
    if(rn >= DD){ Q.push(1); rn -= DD; }
    else if(-rn >= DD){ Q.push(-1); rn += DD; }
    else Q.push(0);
  }
  /* R = A - 2^e*Q*B = B * 2^(e-W) * rn/DD */
  let [rnum, rden] = red(fb.num * rn, fb.den * DD);
  const sh = e - W;
  if(sh >= 0) rnum <<= BigInt(sh); else rden <<= BigInt(-sh);
  [rnum, rden] = red(rnum, rden);
  return {Q, e, R: {num: rnum, den: rden}, exact: rn === 0n, value: hexValue(Q)};
}
/* a dyadic inside (-1,1) written back out as a stalk, so R can be drawn */
function fracDigits(f){
  const k = f.den.toString(2).length - 1;              // den is a power of two
  if(!k) return [0];
  const sgn = f.num < 0n ? -1 : 1, mag = f.num < 0n ? -f.num : f.num;
  return mag.toString(2).padStart(k, "0").split("").map(ch => ch === "1" ? sgn : 0);
}

/* ---- squashing a grid back to a stalk ----
   A cell's weight depends only on r+c, so the anti-diagonals ARE the place
   values and collapsing the rectangle can only mean summing each one. What
   comes back is not a stalk yet: the sums are integers, and anything outside
   {-1,0,+1} still owes a carry. That is the bill the rectangle deferred --
   about two grids in three owe something -- and productDigits() settles it.

   Summing a lattice along its diagonals and carrying is gelosia, the method
   Napier's bones mechanise. The rectangle is the lattice; this is its sum. */
function squashDiagonals(P){
  const S = new Array(P.rows + P.cols - 1).fill(0);
  for(let i = 0; i < P.cells.length; i++)
    S[Math.floor(i / P.cols) + (i % P.cols)] += P.cells[i];
  return S;                                   // S[d] rides weight 2^-(d+2)
}

/* ---- a running value: 2^E times a stalk ----
   Multiplication and division already produce a mantissa and a ring. Addition
   does too the moment it crosses 1 -- 15/16 + 15/16 is 15/8, outside the disc
   every stalk lives in. So all four operations carry the same pair: a stalk
   inside (-1,1), and the exponent that says which ring it sits on. */
function stalkFrac(d, E){
  const v = hexValue(d);                       // den is a power of two
  let num = v.num, den = v.den;
  if(E >= 0) num <<= BigInt(E); else den <<= BigInt(-E);
  const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const h = g(num, den);
  return {num: num / h, den: den / h};
}
/* the inverse: an exact dyadic back to a stalk and a ring, normalised so the
   leading cell is lit and nothing is truncated. */
function fracToStalk(num, den){
  if(num === 0n) return {d: [0], E: 0};
  const abs = x => x < 0n ? -x : x;
  const k = den.toString(2).length - 1;        // den = 2^k
  /* e is the smallest exponent with |num/den| < 2^e */
  let e = abs(num).toString(2).length - k;
  const under = x => x >= 0 ? abs(num) < (den << BigInt(x)) : (abs(num) << BigInt(-x)) < den;
  while(!under(e)) e++;
  while(under(e - 1)) e--;
  const L = k + e;                             // cells in the mantissa
  const sgn = num < 0n ? -1 : 1, mag = abs(num);
  return {d: mag.toString(2).padStart(L, "0").split("").map(c => c === "1" ? sgn : 0), E: e};
}

/* ---- aligning stalks by place value ----
   A stalk on ring E has its cell i at absolute weight 2^-(i+1-E). Adding
   stalks means lining them up on that weight and summing the columns -- which
   is what the arrays were shaped for, and the only thing addition, subtraction
   and the composite of a whole rack have in common.

   `sources` is a list of {d, E, sign}. Returns the columns from the coarsest
   place value to the finest, contiguous, each holding the summed digit; the
   per-source rows, so a caller can draw what went in; the exact total; and the
   stalk it reconciles to. Columns can land anywhere outside {-1,0,+1} -- that
   is the carry the array deferred, exactly as a squashed product does. */
function alignByWeight(sources){
  const maps = sources.map(s => {
    const m = new Map();
    const sign = s.sign === undefined ? 1 : s.sign;
    for(let i = 0; i < s.d.length; i++){
      if(!s.d[i]) continue;
      const w = i + 1 - (s.E || 0);
      m.set(w, (m.get(w) || 0) + s.d[i] * sign);
    }
    return m;
  });
  const keys = [];
  for(const m of maps) for(const w of m.keys()) keys.push(w);
  if(!keys.length)
    return {ws: [], rows: sources.map(() => []), sums: [], owed: 0,
            num: 0n, den: 1n, stalk: {d: [0], E: 0}};
  const lo = Math.min(...keys), hi = Math.max(...keys);
  const ws = [], rows = maps.map(() => []), sums = [];
  for(let w = lo; w <= hi; w++){
    ws.push(w);
    let t = 0;
    maps.forEach((m, j) => { const v = m.get(w) || 0; rows[j].push(v); t += v; });
    sums.push(t);
  }
  let num = 0n;
  ws.forEach((w, i) => { if(sums[i]) num += BigInt(sums[i]) * (1n << BigInt(hi - w)); });
  const den = 1n << BigInt(hi);
  const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const h = g(num, den);
  return {ws, rows, sums, owed: sums.filter(v => Math.abs(v) > 1).length,
          num: num / h, den: den / h, stalk: fracToStalk(num / h, den / h)};
}

/* ---- spread: the mirror of push, and where the bar comes in ----
   Push moves a lit cell LEFT into a green, because 2^-i == 2^-(i-1) - 2^-i.
   It runs to a canonical fixpoint after which no green is ever followed by a
   lit cell -- every green left over is trailing, sitting to the RIGHT of the
   last lit cell, and push cannot reach it: going that way would need
   2^-i == 2^-(i+1) + 2^-(i+1), a digit of 2.

   The exact move rightward is the geometric one, 2^-i == 2^-(i+1) + 2^-(i+2)
   + ... , which only closes in the limit. So the last lit cell empties and
   every trailing green takes its sign, with the final cell barred: it repeats
   forever. Read as a finite string it falls short by exactly one place value
   -- sign * 2^-L -- which is what the bar is for.

   Returns {d, bar, at, short}: the digits, whether a bar is needed, the index
   the repeat starts at, and the exact shortfall of the finite reading. */
function spreadRight(cells){
  const d = cells.slice();
  const L = d.length;
  let last = -1;
  for(let i = L - 1; i >= 0; i--) if(d[i] !== 0){ last = i; break; }
  if(last < 0 || last === L - 1) return {d, bar: false, at: -1, short: {num: 0n, den: 1n}};
  for(let i = last + 1; i < L; i++) if(d[i] !== 0)
    return {d, bar: false, at: -1, short: {num: 0n, den: 1n}};   // not a clean tail
  const s = d[last];
  d[last] = 0;
  for(let i = last + 1; i < L; i++) d[i] = s;
  return {d, bar: true, at: last + 1,
          short: {num: BigInt(s), den: 1n << BigInt(L)}};
}

/* ---- an integer as a phasor on the ring ----
 * Lifted out of wub.html so Wub UTF can use it unchanged. A character is just
 * an integer, so a character fills a ring exactly the way an integer does:
 * its whole cell arrangement decides the radius, the height and both rates.
 * Building a second, different phasor for characters was the mistake — it gave
 * every letter one flat spoke instead of a ring.
 */
/* c.w is the general form. hexRegions sets w = i + 1, so a hex cell weighs
   2^-(i+1) as it always did; productRegions sets w = r + c + 2, and a rectangle
   cell has no single i to fall back on. Merging these two bodies to the hex
   special case silently broke Wub x — they were never the same function. */
const wOf = c => c.v * Math.pow(2, -c.w);
const settled = cells => cells.reduce((s, c) => s + wOf(c), 0);
const spreadOf = cells => cells.reduce((s, c) => s + (c.v === 0 ? Math.pow(2, -c.w) : 0), 0);

/* A phasor from CELLS, so anything that is a stalk can be one: an integer, or
   a fraction cut to a width. phasor() is this with the cells read off an
   integer, which is all it ever was. */
function phasorOf(full, n, extra){
  const R = hexRegions(full, n);
  const runs = full.reduce((c, d, i) => c + (i && d !== full[i-1] ? 1 : 0),
                           full.length ? 1 : 0);
  const inner = settled(R.inner), outer = settled(R.outer), fold = settled(R.fold);
  return Object.assign({
    n, R, shown: full, inner, outer, fold,
    greens: full.filter(d => d === 0).length,
    spread: spreadOf(R.outer),
    amp: Math.abs(fold) + Math.max(Math.abs(inner), Math.abs(outer)),
    dir: Math.sign(fold) || 1,
    rateA: Math.max(1, full.length),
    rateB: Math.max(1, runs),                // one per run of like colour
    value: hexValue(full),
  }, extra || {});
}

function phasor(k, push){
  const {v, neg} = parse(String(k));
  const {n, raw, cells} = hexSequence(v, neg);
  const shown = push ? pushLeft(raw) : raw;
  const full = shown.concat(cells.slice(raw.length));
  const p = phasorOf(full, n, {k, push});
  /* handedness must reverse with the sign, always. the Fold usually says which
     way, but it can be exactly zero -- then the integer's own sign does. */
  p.dir = Math.sign(p.fold) || (neg ? -1 : 1);
  /* the hex string is nibble-quantised, so its length alone would put every
     integer under 16 on the same rate. the bit length is what actually varies,
     and it is what the stalk length stood for before. */
  p.rateA = Math.max(1, v === 0n ? 1 : v.toString(2).length);
  return p;
}

/* a fraction as a phasor: cut it to W cells, lay them into a square, read the
   regions off exactly as an integer's are read. A non-dyadic carries what it
   dropped, so a ring can say it is an approximation rather than pretend. */
function fracPhasor(f, W){
  const st = fracStalk(f, W || 16);
  const n = Math.max(1, Math.ceil(Math.sqrt(st.d.length)));
  const full = st.d.slice();
  while(full.length < n * n) full.push(0);
  const p = phasorOf(full, n, {frac: f, E: st.E, exact: st.exact, rem: st.rem,
                               raw: st.d});
  p.dir = Math.sign(p.fold) || (f.num < 0n ? -1 : 1);
  return p;
}

const angleA = (p, t) => p.phase + p.dir * p.rateA * t;
/* r is a width and cannot go negative, so a negative integer carries its sign
   in B's phase instead — that keeps negation an exact reflection. */
const angleB = (p, t) => p.phase + (p.dir < 0 ? Math.PI : 0) + p.dir * p.rateB * t;
/* the regions keep the meaning the fold gives them: Fold is the equator, Inner
   reaches north, Outer reaches south. A is longitude, B latitude, so a phasor
   rides an ellipsoid whose two halves have different heights. */
const comp = (p, t) => {
  const A = angleA(p, t), B = angleB(p, t);
  const sB = Math.sin(B), rr = p.fold * Math.cos(B);
  const m = (p.inner + p.outer) / 2, d = (p.inner - p.outer) / 2;
  return [rr * Math.cos(A), rr * Math.sin(A), m * sB + d * Math.abs(sB)];
};


/* ---- the boxed grid ----
 * Lifted out of wubbadub.html so every page draws a stalk the same way: one
 * boxed cell per digit with the digit written in it, the fold outlined.
 */
function boxes(cells, cols, rows, foldAt){
  const sz = Math.max(6, Math.min(22, Math.floor(104 / cols), Math.floor(96 / rows)));
  const gl = sz >= 9;
  const body = cells.map((v, i) => {
    const r = Math.floor(i / cols), c = i % cols;
    return `<div class="gsc ${cls(v)}${r + c === foldAt ? " fold" : ""}">${gl ? glyph(v) : ""}</div>`;
  }).join("");
  return `<div class="gsq" style="--sc:${sz}px;`
    + ` grid-template-columns:repeat(${cols},var(--sc))">${body}</div>`;
}
function stalkSquare(digits){
  const n = Math.max(1, Math.ceil(Math.sqrt(digits.length)));
  const cells = digits.slice();
  while(cells.length < n * n) cells.push(0);
  return boxes(cells, n, n, n - 1);
}

/* ---- exact arithmetic on what a card holds ----
 * Cards take expressions: 47*127, (13*3*127/2^4), (1/3). Everything here is
 * exact rationals in BigInt, never floats — math.js and friends evaluate to
 * doubles, and this whole system's claim is that the value is exact. A parser
 * that hands back num/den is forty lines; a dependency that hands back 0.333 is
 * a different project.
 */
function gcdBig(a, b){
  a = a < 0n ? -a : a; b = b < 0n ? -b : b;
  while(b){ [a, b] = [b, a % b]; }
  return a || 1n;
}
/* `rat`, not `frac`: three pages already have a local frac() for the animation
   phase, and a top-level frac here shadowed them all. */
function rat(num, den){
  if(den === 0n) throw new Error("divide by zero");
  if(den < 0n){ num = -num; den = -den; }
  const g = gcdBig(num, den);
  return {num: num / g, den: den / g};
}
/* declarations, not consts: a const does not escape an eval scope, and every
   test in this repo loads stalk.js by eval. Three separate debugging detours
   started with exactly that. */
function fAdd(a, b){ return rat(a.num * b.den + b.num * a.den, a.den * b.den); }
function fSub(a, b){ return rat(a.num * b.den - b.num * a.den, a.den * b.den); }
function fMul(a, b){ return rat(a.num * b.num, a.den * b.den); }
function fDiv(a, b){ return rat(a.num * b.den, a.den * b.num); }
function fPow(a, n){
  if(n < 0n) return fDiv({num: 1n, den: 1n}, fPow(a, -n));
  let r = {num: 1n, den: 1n};
  for(let i = 0n; i < n; i++) r = fMul(r, a);
  return r;
}

/* evalFrac(src, env) — env maps a name to a rational, so a card can say a = 3
   and every other card can use a. An unknown name is an error rather than a
   zero: a typo should say so, not quietly draw the wrong thing. */
function evalFrac(src, env){
  const s = String(src).replace(/[\s_]/g, "");
  let i = 0;
  const peek = () => s[i];
  const eat = c => { if(s[i] === c){ i++; return true; } return false; };
  function atom(){
    if(eat("(")){
      const v = expr();
      if(!eat(")")) throw new Error("missing )");
      return v;
    }
    const id = /^[A-Za-z][A-Za-z0-9]*/.exec(s.slice(i));
    if(id){
      i += id[0].length;
      if(!env || !(id[0] in env)) throw new Error(id[0] + " is not defined");
      return env[id[0]];
    }
    const m = /^[0-9]+/.exec(s.slice(i));
    if(!m) throw new Error("expected a number at " + (i + 1));
    i += m[0].length;
    return {num: BigInt(m[0]), den: 1n};
  }
  function unary(){
    if(eat("-")) return fSub({num: 0n, den: 1n}, unary());
    if(eat("+")) return unary();
    return atom();
  }
  function power(){
    const base = unary();
    if(eat("^")){
      const e = power();
      if(e.den !== 1n) throw new Error("the exponent must be a whole number");
      return fPow(base, e.num);
    }
    return base;
  }
  function term(){
    let v = power();
    for(;;){
      if(eat("*")) v = fMul(v, power());
      else if(eat("/")) v = fDiv(v, power());
      else return v;
    }
  }
  function expr(){
    let v = term();
    for(;;){
      if(peek() === "+"){ i++; v = fAdd(v, term()); }
      else if(peek() === "-"){ i++; v = fSub(v, term()); }
      else return v;
    }
  }
  const v = expr();
  if(i !== s.length) throw new Error("unexpected '" + s[i] + "'");
  return v;
}
/* Is it dyadic? Only then does it have a finite stalk — 1/3 never terminates,
   which is the same fact Wub / reports as a remainder.
   A declaration, not a const: a const does not escape an eval scope, and every
   test in this repo loads stalk.js by eval. */
function isDyadic(f){ return (f.den & (f.den - 1n)) === 0n; }

/* A fraction as a stalk: cells in (-1,1) riding a ring exponent E.
 *
 * A dyadic lands exactly. A non-dyadic never terminates -- 1/3 is 0.010101...
 * for ever -- so it is cut at W cells and the part that did not fit comes back
 * as a remainder, which is the same answer Wub / gives and the same reason only
 * 430 of 2178 quotients there were exact.
 */
function fracStalk(f, W){
  W = W || 24;
  if(f.num === 0n) return {d: [0], E: 0, exact: true, rem: {num: 0n, den: 1n}};
  const sgn = f.num < 0n ? -1 : 1;
  const num = f.num < 0n ? -f.num : f.num, den = f.den;
  /* smallest E with |f| < 2^E */
  let E = 0;
  while ((num << BigInt(Math.max(0, -E))) >= (den << BigInt(Math.max(0, E)))) E++;
  while (E > 0 && (num << BigInt(Math.max(0, 1 - E))) < (den << BigInt(Math.max(0, E - 1)))) E--;
  /* mantissa = floor(|f| * 2^(W-E)), so cell i weighs 2^-(i+1) on ring E */
  const shift = BigInt(W) - BigInt(E);
  const scaledNum = shift >= 0n ? num << shift : num;
  const scaledDen = shift >= 0n ? den : den << -shift;
  const m = scaledNum / scaledDen;
  const rest = scaledNum - m * scaledDen;
  const d = m.toString(2).padStart(W, "0").split("").map(c => (c === "1" ? sgn : 0));
  return {d, E, exact: rest === 0n,
          rem: rest === 0n ? {num: 0n, den: 1n} : rat(rest * BigInt(sgn), scaledDen)};
}

/* a card that reads  name = expr  defines a variable */
function defOf(src){
  const m = /^\s*([A-Za-z][A-Za-z0-9]*)\s*=\s*(.+)$/.exec(String(src));
  return m ? {name: m[1], body: m[2]} : null;
}
