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
  const raw = (b ? b.padStart(nib, "0") : "").split("")
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
    const slot = {v: cells[i] || 0, r, c, i};
    (r + c < n - 1 ? inner : r + c === n - 1 ? fold : outer).push(slot);
  }
  return {inner, fold, outer};
}
