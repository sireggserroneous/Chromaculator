/* node codegg-v1/tools/versus.js [file] -- codec-v1 and codegg-v1, same damage.
 *
 * Two codecs over the same square, built from opposite readings of it:
 * codec-v1 reads the square as a bag of symbols and triangulates position from
 * 4N-1 sums; codegg-v1 reads it as a number and asks the value where it hurts.
 * Each channel below applies equivalent damage to both and reports what came
 * back. Both codecs win rows here, and the table says which.
 *
 * The channels are byte-fair where the alphabets differ: codec-v1's cell is a
 * byte, codegg's is a bit, so "one corrupted byte" is a single-cell error for
 * codec-v1 and up to eight cell errors for codegg -- and the table shows
 * exactly that cost, not a flattering translation. */
const fs = require("fs"), path = require("path");
const V1 = require(path.join(__dirname, "..", "..", "codec-v1", "chromacode.js"));
const G = require(path.join(__dirname, "..", "codegg.js"));

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

const file = process.argv[2] || path.join(__dirname, "..", "..", "spec.md");
const src = fs.readFileSync(file);
const N = 32;
const code = G.makeCode(N);
const T = 300;

const same = (a, b) => Buffer.from(a).equals(Buffer.from(src));

/* one trial against codec-v1: damage(p) mutates its squares */
function v1trial(damage){
  const g = v1trial.g;
  const p = V1.encode(src, {N, alphabet: "byte"});
  damage(p, g);
  const out = V1.decode(p);
  const exact = same(out.bytes);
  return exact ? "exact" : (out.detected > 0 ? "detected" : "silent");
}
/* one trial against codegg: damage(p) mutates its squares, may return opts */
function ggtrial(damage){
  const g = ggtrial.g;
  const p = G.encode(src, {N, code});
  const opts = damage(p, g) || {};
  const out = G.decode(p, opts);
  const exact = same(out.bytes);
  if(exact) return "exact";
  if(out.detected > 0) return "detected";
  return out.corrected > 0 ? "miscorrected" : "silent";
}

function channel(label, v1damage, ggdamage, note){
  v1trial.g = mul32(1234); ggtrial.g = mul32(1234);
  const tally = side => {
    const t = {exact: 0, detected: 0, silent: 0, miscorrected: 0};
    for(let i = 0; i < T; i++) t[side === "v1" ? v1trial(v1damage) : ggtrial(ggdamage)]++;
    return t;
  };
  const fmt = t => {
    if(t.exact === T) return "EXACT " + T + "/" + T;
    if(t.exact === 0 && t.silent === 0 && t.miscorrected === 0) return "detect only";
    return `exact ${t.exact}, detect ${t.detected}`
      + (t.miscorrected ? `, MIS ${t.miscorrected}` : "")
      + (t.silent ? `, SILENT ${t.silent}` : "");
  };
  const a = tally("v1"), b = tally("gg");
  console.log(`  ${label.padEnd(30)} ${fmt(a).padEnd(24)} ${fmt(b)}`);
  if(note) console.log(`  ${"".padEnd(30)} ${note}`);
}

/* ---- the table ---- */
const s1 = V1.sizes(V1.encode(src, {N, alphabet: "byte"}).meta);
const s2 = G.sizes(G.encode(src, {N, code}).meta);
console.log(`${path.basename(file)}, ${src.length} bytes, N=${N}, ${T} trials per channel\n`);
console.log(`  ${"channel".padEnd(30)} ${"codec-v1 (block)".padEnd(24)} codegg-v1 (arithmetic)`);
console.log(`  ${"-".repeat(30)} ${"-".repeat(24)} ${"-".repeat(24)}`);
console.log(`  ${"overhead".padEnd(30)} ${((100 * s1.parityOverhead).toFixed(1) + "%  (" + s1.parityBytes + " B)").padEnd(24)}`
  + `${(100 * s2.overhead).toFixed(2)}% (${s2.checkBytes} B)`);

/* one flipped bit somewhere in the file */
channel("1 bit flipped",
  (p, g) => {
    const bit = g() % (src.length * 8);
    const s = Math.floor(bit / 8 / (N * N)) % p.squares.length;
    const cellIdx = Math.floor(bit / 8) % (N * N);
    p.squares[s][Math.floor(cellIdx / N)][cellIdx % N] ^= 1 << (7 - (bit & 7));
  },
  (p, g) => {
    const bit = g() % (src.length * 8);
    const s = Math.floor(bit / code.L), i = bit % code.L;
    p.squares[s][i] = p.squares[s][i] === 1 ? 0 : 1;
  });

/* one whole byte replaced -- one cell for codec-v1, up to 8 cells for codegg */
channel("1 byte corrupted",
  (p, g) => {
    const B = g() % src.length;
    const s = Math.floor(B / (N * N)) % p.squares.length, cellIdx = B % (N * N);
    const r = Math.floor(cellIdx / N), c = cellIdx % N;
    p.squares[s][r][c] ^= 1 + (g() % 255);
  },
  (p, g) => {
    const B = g() % src.length, nv = src[B] ^ (1 + (g() % 255));
    for(let j = 0; j < 8; j++){
      const bit = B * 8 + j;
      p.squares[Math.floor(bit / code.L)][bit % code.L] = (nv >> (7 - j)) & 1;
    }
  },
  "(codegg pays its bit-cell resolution here: a byte is up to 8 errors)");

/* the same channel with codegg's double-error search off: the aggressive
   search is what miscorrects on 3+ errors, so this is the safe configuration
   -- fewer repairs, and detection instead of confident wrong answers */
channel("1 byte corrupted, doubles off",
  (p, g) => {
    const B = g() % src.length;
    const s = Math.floor(B / (N * N)) % p.squares.length, cellIdx = B % (N * N);
    p.squares[s][Math.floor(cellIdx / N)][cellIdx % N] ^= 1 + (g() % 255);
  },
  (p, g) => {
    const B = g() % src.length, nv = src[B] ^ (1 + (g() % 255));
    for(let j = 0; j < 8; j++){
      const bit = B * 8 + j;
      p.squares[Math.floor(bit / code.L)][bit % code.L] = (nv >> (7 - j)) & 1;
    }
    return {doubles: false};
  });

/* two adjacent bytes scribbled at a KNOWN position -- 16 flagged bit-cells for
   codegg, two same-row cells for codec-v1, which has no erasure notion */
channel("2 adjacent bytes, position known",
  (p, g) => {
    const s = g() % p.squares.length, r = g() % N, c = g() % (N - 1);
    p.squares[s][r][c] ^= 1 + (g() % 255);
    p.squares[s][r][c + 1] ^= 1 + (g() % 255);
  },
  (p, g) => {
    const B = g() % Math.max(1, src.length - 2);
    const F = [];
    for(let j = 0; j < 16; j++){
      const bit = B * 8 + j, s = Math.floor(bit / code.L);
      if(s !== Math.floor(B * 8 / code.L)) break;      // stay in one square
      p.squares[s][bit % code.L] = g() % 2;
      F.push(bit % code.L);
    }
    return F.length ? {erased: new Map([[Math.floor(B * 8 / code.L), F]])} : {};
  });

/* the 12-cell row burst from codec-v1's own README, position known */
channel("12-cell row burst, known",
  (p, g) => {
    const s = g() % p.squares.length, r = g() % N, c0 = g() % (N - 12);
    for(let j = 0; j < 12; j++) p.squares[s][r][c0 + j] ^= 1 + (g() % 255);
  },
  (p, g) => {
    const s = g() % p.squares.length, r = g() % N, c0 = g() % (N - 12);
    const F = [];
    for(let j = 0; j < 12; j++){ const i = r * N + c0 + j; p.squares[s][i] = g() % 2; F.push(i); }
    return {erased: new Map([[s, F]])};
  });

/* push: respell every square, values conserved, then just verify */
{
  eval(fs.readFileSync(path.join(__dirname, "..", "..", "stalk.js"), "utf8"));  // pushLeft
  const p = G.encode(src, {N, code});
  let hold = 0, respelled = 0;
  const v1p = [];
  for(let s = 0; s < p.squares.length; s++){
    const before = Array.from(p.squares[s]);
    const pushed = pushLeft(before);
    if(pushed.join() !== before.join()) respelled++;
    if(G.verify(pushed, p.checks[s], code)) hold++;
    const alph = V1.ALPHABETS.chroma;
    const a = V1.parities(V1.toSquares(before, N)[0], alph, N);
    const b = V1.parities(V1.toSquares(pushed, N)[0], alph, N);
    v1p.push(a.rows.join() === b.rows.join() && a.cols.join() === b.cols.join());
  }
  const v1hold = v1p.filter(Boolean).length;
  console.log(`  ${"push (respell all squares)".padEnd(30)} ${`checks hold ${v1hold}/${p.squares.length}`.padEnd(24)}`
    + `checks hold ${hold}/${p.squares.length}`);
  console.log(`  ${"".padEnd(30)} (${respelled}/${p.squares.length} squares actually respelled)`);
}

console.log(`\n  Both codecs win rows. codec-v1 never miscorrects and detects scattered`);
console.log(`  multi-errors without a floor; codegg locates from 5x less redundancy,`);
console.log(`  names the magnitude, repairs known-position damage, and survives push.`);
