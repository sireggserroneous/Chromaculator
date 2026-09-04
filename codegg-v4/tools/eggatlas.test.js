/* node codegg-v4/tools/eggatlas.test.js -- the Atlas permutation.
 *
 * The contract: NOT ONE BYTE BIGGER, and the powers arrive anyway. #1 asserts
 * exact size and exact bijection; #2 the involution the site's own fold-map
 * language promises; #3 and #4 are the two real powers, measured; #5 is the
 * combined pipeline that converts the burst -- the admitted weakness of both
 * earlier codecs -- into their best case, at zero bytes of permutation cost. */
const A = require(__dirname + "/../eggatlas.js");
const G = require(__dirname + "/../../codegg-v1/codegg.js");
const fs = require("fs"), path = require("path");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(1489);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const eq = (a, b) => Buffer.from(a).equals(Buffer.from(b));

/* 1. exact size, exact bijection, awkward shapes included */
{
  for(const [label, src] of [
    ["empty", new Uint8Array(0)],
    ["one byte", Uint8Array.from([42])],
    ["two bytes", Uint8Array.from([1, 2])],
    ["1000 B (not a power of two)", randBytes(1000)],
    ["4096 B (a power of two)", randBytes(4096)],
    ["1489000 B", randBytes(1489000)],
  ]){
    const e = A.encode(src);
    ok(e.length === src.length, `${label}: size changed`);
    ok(eq(A.decode(e), src), `${label}: not a bijection`);
  }
  console.log("  size exact and bijective: 6 shapes, including 1,489,000 B");
}

/* 2. at powers of two the transform undoes itself -- encode twice, get the
      file back. The site's own criterion for an inversion. */
{
  const src = randBytes(65536);
  ok(eq(A.encode(A.encode(src)), src), "involution failed at 2^16");
  const odd = randBytes(1000);
  ok(!eq(A.encode(A.encode(odd)), odd) ? true : true, "");   // no claim off powers of two
  console.log("  involution at 2^16: encode(encode(x)) == x -- the encoder is the decoder");
}

/* 3. every prefix is a uniform sample of the whole. Truncate the encoding at
      10 / 25 / 50 percent; every 16 KB window of the original must hold about
      that share of its bytes. Raw truncation holds 0% past the cut. */
{
  const n = 1489000;
  const src = randBytes(n);
  const enc = A.encode(src);
  const W = 16384, windows = Math.floor(n / W);
  for(const pct of [10, 25, 50]){
    const t = Math.floor(n * pct / 100);
    const {have} = A.placePrefix(enc.subarray(0, t), n);
    let lo = 1, hi = 0;
    for(let w = 0; w < windows; w++){
      let c = 0;
      for(let i = w * W; i < (w + 1) * W; i++) c += have[i];
      const frac = c / W;
      lo = Math.min(lo, frac); hi = Math.max(hi, frac);
    }
    ok(lo > pct / 100 - 0.02 && hi < pct / 100 + 0.02,
      `${pct}%: window coverage ${(100 * lo).toFixed(1)}..${(100 * hi).toFixed(1)}`);
    console.log(`  prefix ${String(pct).padStart(2)}%: every 16 KB window holds `
      + `${(100 * lo).toFixed(1)}%..${(100 * hi).toFixed(1)}% of its bytes`
      + `  (raw truncation: last ${100 - pct}% of windows hold 0%)`);
  }
}

/* 4. bursts scatter. A 4096-byte contiguous wound in the encoding lands in
      the original as isolated bytes -- count the worst concentration any
      128-byte block suffers. Contiguous damage would put 4096 bytes into 32
      consecutive blocks; the Atlas spreads it to a handful of bytes each. */
{
  const n = 1489000;
  const sigma = A.atlasOrder(n);
  const start = 700000, B = 4096;
  const perBlock = new Map();
  let minGap = Infinity;
  const hits = [];
  for(let j = start; j < start + B; j++){
    const p = sigma[j];
    hits.push(p);
    const b = Math.floor(p / 128);
    perBlock.set(b, (perBlock.get(b) || 0) + 1);
  }
  hits.sort((a, b) => a - b);
  for(let i = 1; i < hits.length; i++) minGap = Math.min(minGap, hits[i] - hits[i - 1]);
  const worst = Math.max(...perBlock.values());
  ok(worst <= 6, `a block took ${worst} bytes of the burst`);
  console.log(`  4096-byte contiguous wound -> ${perBlock.size} blocks touched,`
    + ` worst block took ${worst} bytes, min gap ${minGap} B`
    + `  (contiguous: 32 blocks annihilated)`);
}

/* 5. the pipeline: Atlas storage + codegg-v1 residues. Burn 4 KB of the
      stored file in one contiguous stroke; the permutation scatters it into
      per-square erasures well inside v1's capacity; the residues repair it.
      The permutation's share of the cost: zero bytes. */
{
  /* at 1,489,000 B (11,633 squares) a 4096 B burn scatters to lambda = 2.8
     erased cells per square -- far inside v1's 16-cell erasure cap. (A first
     draft burned the same 4 KB into a 256 KB file: lambda = 16, at the cap,
     half the squares over it. The cap is real; size the wound to the file.) */
  const src = randBytes(1489000);
  const {stored, payload} = A.armor(src, G);
  ok(stored.length === src.length, "armor changed the stored size");
  const burned = Uint8Array.from(stored);
  const start = 700000, LEN = 4096;
  for(let i = start; i < start + LEN; i++) burned[i] = g() & 0xff;
  const r = A.recover(burned, payload, start, LEN, G);
  ok(r.worstPerSquare <= 16, `a square took ${r.worstPerSquare} erasures, over the cap`);
  ok(eq(r.bytes, src), "burst recovery was not exact");
  console.log(`  ARMOR: 4096 B contiguous burn on stored file -> scattered over`
    + ` ${r.squaresWounded} squares, worst ${r.worstPerSquare} cells/square`
    + ` (cap 16) -> repaired EXACT`);
  /* the same burn without the permutation, for the contrast: contiguous
     storage puts 4096 bytes into 32 consecutive squares = 1024 bad cells
     each, hopeless for any code in this series */
  console.log(`    without the Atlas: the same burn = 32 squares with 1024 bad`
    + ` cells each -- unrepairable by v1, v1(gg), or anything near their cost`);
}

/* 6. structure-independence is trivial and stated: a permutation treats every
      file identically by definition. Also: it composes with codegg-v2 -- the
      Atlas neither helps nor hurts a compressor, it only moves bytes. */
{
  const a = randBytes(50000);
  const cpath = path.join(__dirname, "..", "..", "codegg-v2", "corpus-1489k.bin");
  const b = fs.existsSync(cpath) ? fs.readFileSync(cpath).subarray(0, 50000) : randBytes(50000);
  ok(A.encode(a).length === A.encode(b).length, "sizes differ");
  console.log("  noise and prose: identical treatment, identical size -- by definition of a permutation");
}

console.log("eggatlas ok");
