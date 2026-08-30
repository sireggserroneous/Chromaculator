/* chronochromatic.org — the fold, shared by every page.
   SIGNED-DIGIT convention: red = -1, green = 0, blue = +1, base 2.
   Ascending anti-diagonals, 2^0 at A1 and the top power at nA1, so the
   units digit sits in the same cell whatever size the grid is.        */

function parseNum(s){
  s = s.trim().replace(/[_,\s]/g, "");
  if(!s) throw new Error("Enter an integer.");
  let neg = false;
  if(s[0] === "-"){ neg = true; s = s.slice(1); }
  else if(s[0] === "+") s = s.slice(1);
  let v;
  try{ v = BigInt(s); }catch(e){ throw new Error(`Not an integer I can read: ${s}`); }
  if(v > (1n << 4096n)) throw new Error("Too large to lay out — keep it under 4096 bits.");
  return {v, neg};
}

/* digits LSB-first: 0/1 for a positive, 0/-1 for a negative. */
function digitsOf(v, neg){
  const d = [];
  let x = v;
  while(x > 0n){ d.push(Number(x & 1n) * (neg ? -1 : 1)); x >>= 1n; }
  return d;
}

/* the walk: anti-diagonals in ASCENDING order, each bottom-left to top-right.
   index 0 is A1 (place value 2^0); the last index is nA1.               */
function diagOrder(n){
  const o = [];
  for(let d = 0; d <= 2*(n-1); d++) o.push(d);
  return o;
}
const arcSize = (n,d) => Math.min(d,n-1) - Math.max(0,d-(n-1)) + 1;
const arcs = n => diagOrder(n).map(d => arcSize(n,d));
function cells(n){
  const out = [];
  for(const d of diagOrder(n))
    for(let c = Math.max(0,d-(n-1)); c <= Math.min(d,n-1); c++) out.push([d-c, c]);
  return out;
}

/* size the grid to the digits, then pad with green zeros toward the high pole */
function sequence(v, neg){
  const raw = digitsOf(v, neg);
  const n = Math.ceil(Math.sqrt(raw.length));
  const seq = raw.slice();
  while(seq.length < n*n) seq.push(0);
  return {n, raw, seq};
}

/* place value of a region = sum of digit * 2^index over its cells */
function buildFold(seq, n){
  const g = Array.from({length:n}, () => new Array(n).fill(0));
  const pv = Array.from({length:n}, () => new Array(n).fill(0));
  const C = cells(n);
  C.forEach(([r,c], i) => { g[r][c] = seq[i] ?? 0; pv[r][c] = i; });

  const LET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  const label = (r,c) => {
    const d = r + c;
    if(d < n-1)  return LET[d] + (c+1);
    if(d === n-1) return "Fold" + (c+1);
    return "n" + LET[2*(n-1)-d] + (n-r);
  };
  const A = [], F = [], NA = [];          // Inner, Fold, Outer — as {v, pow}
  C.forEach(([r,c], i) => {
    const item = {v: g[r][c], pow: i};
    const d = r + c;
    (d < n-1 ? A : d === n-1 ? F : NA).push(item);
  });
  let match = 0, comparable = 0;
  for(let r=0;r<n;r++) for(let c=0;c<n;c++){
    if(r + c >= n-1) continue;
    comparable++; if(g[r][c] === g[n-1-c][n-1-r]) match++;
  }
  return {n, g, pv, label, A, F, NA, match, comparable};
}

/* the value a set of cells contributes: sum of digit * 2^power */
function regionValue(cellsList){
  let t = 0n;
  for(const {v, pow} of cellsList) if(v) t += BigInt(v) * (1n << BigInt(pow));
  return t;
}
const numOf = b => Number(b);
const fmtBig = b => b.toString();

/* the whole number is exactly Inner + Fold + Outer */
const wholeValue = f => regionValue(f.A) + regionValue(f.F) + regionValue(f.NA);

/* sign: the highest non-zero digit decides it */
function signOf(f){
  const all = [...f.A, ...f.F, ...f.NA].sort((a,b) => b.pow - a.pow);
  for(const {v} of all) if(v) return Math.sign(v);
  return 0;
}

function sphereData(f, n){
  const pts = [];
  for(const d of diagOrder(n)){
    const lo = Math.max(0, d-(n-1)), hi = Math.min(d, n-1), L = hi - lo + 1;
    for(let c = lo, j = 0; c <= hi; c++, j++){
      const th = Math.PI * (d + .5) / (2*n - 1);
      const ph = 2 * Math.PI * (j + .5) / L;
      pts.push({ x: Math.sin(th)*Math.cos(ph), y: Math.cos(th), z: Math.sin(th)*Math.sin(ph),
                 v: f.g[d-c][c], fold: d === n-1 });
    }
  }
  return pts;
}

/* ---- helpers the pages use ---- */
const cls   = v => v > 0 ? "b" : v < 0 ? "r" : "g";
const glyph = v => v > 0 ? "1" : v < 0 ? "−" : "0";
const digitsStr = region => region.map(d => glyph(d.v)).join("");
/* a region's contribution, as a readable string */
function fmtVal(region){
  const t = regionValue(region);
  return t.toString();
}
/* the highest power a region actually uses, and its lowest -- for readouts */
function powRange(region){
  const on = region.filter(d => d.v);
  if(!on.length) return null;
  return [Math.min(...on.map(d => d.pow)), Math.max(...on.map(d => d.pow))];
}
function chunk(arr, lens){
  const out = []; let k = 0;
  for(const L of lens){ if(k >= arr.length) break; out.push(arr.slice(k, k+L)); k += L; }
  if(k < arr.length) out.push(arr.slice(k));
  return out;
}

/* push every non-zero digit as high as the padding allows:
     1 * 2^i  =  -1 * 2^i  +  1 * 2^(i+1)
   the value never changes; the colours do. run to a fixpoint. */
function pushDigits(seq){
  const d = seq.slice();
  let moved = true;
  while(moved){
    moved = false;
    for(let i = 0; i < d.length - 1; i++)
      if(d[i] !== 0 && d[i+1] === 0){ d[i+1] = d[i]; d[i] = -d[i]; moved = true; }
  }
  return d;
}

/* ---- the surreal reading, alongside the place-value one ----
   read a region's digits as a Blue-Red Hackenbush stalk: +1 blue, -1 red,
   and 0 GREEN meaning undecided. `sub` says what every green becomes, so
   sub=-1 gives the floor of the range and sub=+1 the ceiling.            */
function stalkValue(digs, sub){
  const b = digs.map(x => x === 0 ? sub : x);
  const m = b.length;
  if(!m) return {num: 0n, den: 1n};
  const sign = b[0] > 0 ? 1n : -1n;
  let k = 1; while(k < m && b[k] === b[0]) k++;
  const den = 1n << BigInt(m - k);
  let num = sign * BigInt(k) * den;
  for(let i = k; i < m; i++) num += BigInt(b[i]) * (1n << BigInt(m - i - 1));
  const g = (a,c) => { a = a < 0n ? -a : a; while(c){ [a,c] = [c, a % c]; } return a || 1n; };
  const d = g(num, den);
  return {num: num/d, den: den/d};
}
const bare = f => f.den === 1n ? f.num.toString() : `${f.num}/${f.den}`;
const rawDigits = region => region.map(d => (d && d.v !== undefined) ? d.v : d);
const hasGreen  = region => rawDigits(region).some(x => x === 0);
function stalkRange(region){
  const d = rawDigits(region);
  return {lo: stalkValue(d, -1), hi: stalkValue(d, 1)};
}
function fmtStalk(region){
  if(!region.length) return "—";
  const {lo, hi} = stalkRange(region);
  return hasGreen(region) ? `${bare(lo)} → ${bare(hi)}` : bare(lo);
}

/* ---- the inverted reading: negate every exponent ----
   digit i normally weighs 2^i; here it weighs 2^-i, so 2^0 stays put and
   2^n falls to 2^-n. that is exactly bit reversal over 2^(cells-1), which
   makes it a dyadic always, and injective -- no leading run to collapse
   information the way a Hackenbush tally does.                          */
function invValue(digs){
  const m = digs.length;
  if(!m) return {num: 0n, den: 1n};
  const den = 1n << BigInt(m - 1);
  let num = 0n;
  for(let i = 0; i < m; i++) num += BigInt(digs[i]) * (1n << BigInt(m - 1 - i));
  const g = (a,c) => { a = a < 0n ? -a : a; while(c){ [a,c] = [c, a % c]; } return a || 1n; };
  const d = g(num, den);
  return {num: num/d, den: den/d};
}
const fmtInv = region => bare(invValue(rawDigits(region)));
/* reverse the low `bits` bits of i -- the order segments fall into when a
   ring is sorted by its inverted value. */
function bitrev(i, bits){
  let o = 0;
  for(let b = 0; b < bits; b++) if(i >> b & 1) o |= 1 << (bits - 1 - b);
  return o;
}

/* ---- the bisection reading ----
   2^0 is pinned to a fixed 0, so slot i weighs 2^-(i+1) and the walk can
   never spend digits on an integer tally. digits are the binary of |k|
   MSB-first with 0 -> -1: every slot is a left/right step, blue right and
   red left, so the string IS the path to the value. lands k=1..2^n-1 on
   exactly the dyadics j/2^n in (0,1), ascending inside each ring.       */
function bisectSequence(v, neg){
  const raw = v === 0n ? []
    : v.toString(2).split("").map(c => (c === "1" ? 1 : -1) * (neg ? -1 : 1));
  const n = Math.ceil(Math.sqrt(raw.length));
  const seq = raw.slice();
  while(seq.length < n*n) seq.push(0);
  return {n, raw, seq};
}
/* padding is free: trailing zeros scale numerator and denominator alike. */
function bisectValue(digs){
  const L = digs.length;
  if(!L) return {num: 0n, den: 1n};
  let num = 0n;
  for(let i = 0; i < L; i++) num += BigInt(digs[i]) * (1n << BigInt(L - 1 - i));
  const den = 1n << BigInt(L);
  const g = (a,c) => { a = a < 0n ? -a : a; while(c){ [a,c] = [c, a % c]; } return a || 1n; };
  const d = g(num, den);
  return {num: num/d, den: den/d};
}
const fmtBisect = region => bare(bisectValue(rawDigits(region)));

/* ---- the surreal address ----
   the sign expansion: walk down the surreal tree from 0, appending + when
   the target is above where you stand and - when it is below, stepping to
   the simplest number in the interval you have narrowed to. this is not
   the bisection path -- the tree spends its first signs reaching the
   integer part, so 3/4 addresses as +-+ while its path is RR.          */
function floorDiv(a, b){ const q = a / b; return (a % b !== 0n && (a < 0n) !== (b < 0n)) ? q - 1n : q; }
function simplestBetween(lo, hi){          // null bound = infinite on that side
  const cmp = (a, b) => { const l = a.n * b.d, r = b.n * a.d; return l < r ? -1 : l > r ? 1 : 0; };
  const below0 = lo === null || lo.n < 0n, above0 = hi === null || hi.n > 0n;
  if(below0 && above0) return {n: 0n, d: 1n};
  if(lo === null) return {n: -floorDiv(-hi.n, hi.d) - 1n, d: 1n};   // (-inf, hi)
  if(hi === null) return {n: floorDiv(lo.n, lo.d) + 1n, d: 1n};     // (lo, +inf)
  for(let j = 0n; j < 256n; j++){
    const D = 1n << j;
    const cand = {n: floorDiv(lo.n * D, lo.d) + 1n, d: D};
    if(cmp(cand, hi) < 0) return cand;
  }
  return {n: 0n, d: 1n};
}
function signExpansion(num, den){
  if(num === 0n) return "";
  const cmp = (a, b) => { const l = a.n * b.d, r = b.n * a.d; return l < r ? -1 : l > r ? 1 : 0; };
  const X = {n: num, d: den};
  let lo = null, hi = null, cur = {n: 0n, d: 1n}, out = "";
  while(cmp(cur, X) !== 0 && out.length < 4096){
    if(cmp(X, cur) > 0){ out += "+"; lo = cur; } else { out += "−"; hi = cur; }
    cur = simplestBetween(lo, hi);
  }
  return out;
}
const addressOf = f => signExpansion(f.num, f.den);

/* ---- non-adjacent form ----
   the third canonical spelling in the same digit set. greedy: at every odd
   step take the sign that leaves the next value divisible by four, which
   collapses each run +++...+ into -0...0+. no two nonzeros ever end up
   adjacent, and no signed-binary spelling of k has fewer lit cells.     */
function nafDigits(v, neg){
  const d = [], s = neg ? -1 : 1;
  let x = v;
  while(x > 0n){
    if(x & 1n){ const z = (x % 4n) === 1n ? 1n : -1n; d.push(Number(z) * s); x -= z; }
    else d.push(0);
    x >>= 1n;
  }
  return d;
}
function nafSequence(v, neg){
  const raw = nafDigits(v, neg);
  const n = Math.ceil(Math.sqrt(raw.length));
  const seq = raw.slice();
  while(seq.length < n*n) seq.push(0);
  return {n, raw, seq};
}
/* one place to ask for a place-value spelling: "plain" | "push" | "naf" */
function formSequence(v, neg, form){
  if(form === "naf") return nafSequence(v, neg);
  const s = sequence(v, neg);
  return form === "push" ? {n: s.n, raw: s.raw, seq: pushDigits(s.seq)} : s;
}
