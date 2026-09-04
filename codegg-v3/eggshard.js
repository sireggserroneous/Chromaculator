/* codegg v3 -- the file as a number, the number as residues.
 *
 * v2 was a compressor at heart: it auctioned chunks to the powers and paid
 * literal when they lost, so everything it promised depended on what the file
 * happened to contain. A true encoder promises by CONSTRUCTION. This one
 * takes the system's deepest reading -- value semantics over spelling
 * semantics -- and makes it the representation itself.
 *
 * Each 128-byte block (one codegg square) is a 1024-bit value V. The encoding
 * is V mod p_1 .. V mod p_(k+m), for k+m distinct 16-bit primes: the file
 * becomes k+m SHARD FILES, each carrying one residue stream, each 1/64th the
 * size of the original. By the Chinese Remainder Theorem, ANY k shards
 * reconstruct every block exactly -- the product of any k of these primes
 * exceeds 2^1024, so V has one address in the surviving worlds.
 *
 * What the encoding has that the bytes do not:
 *
 *   ERASURE, egalitarian. Any m shards may die. There is no header shard, no
 *   high byte, no privileged position -- every shard is one more world's
 *   opinion of the same number, and k opinions settle it. (Positional
 *   encodings die by WHERE they are wounded; this one only by HOW MUCH.)
 *
 *   CONVICTION. With more than k shards alive, the spares vote: a corrupted
 *   shard is detected per block and located by leave-one-out -- codegg-v1's
 *   move, promoted from check to representation.
 *
 *   ARITHMETIC WITHOUT DECODING. Residue streams add shard-wise: carries
 *   CANNOT cross primes. Two encoded files can be summed, or scaled, without
 *   ever reconstructing either -- Avizienis's carry-free addition at file
 *   scale, which is where this whole system pointed from the start.
 *
 *   STRUCTURE-INDEPENDENCE. Noise and prose encode at identical cost with
 *   identical guarantees: (k+m)/k of the input, stated up front. No auction,
 *   no luck, no gzip envy. This is not a compressor and never claims to be.
 *
 * Lineage, in the site's tradition of saying so: the residue number system is
 * ancient (Sun Tzu's problem), redundant-RNS erasure codes are Mandelbaum
 * 1976, and Reed-Solomon is the industrial cousin that does this job over
 * polynomial fields. This is the arithmetic-native one, chosen because the
 * system it serves is arithmetic-native. The parts are old; the fit is the
 * only local thing.
 */

const BLOCK = 128;                        // one codegg square, 1024 bits
const K = 65;                             // any 65 shards reconstruct
const MAGIC = [0x45, 0x47, 0x47, 0x33];   // EGG3

/* ---- primes: the largest below 2^16, so residues pack in two bytes ----
   The smallest 65 of however many we take must multiply past 2^1024; that is
   checked at init by actual multiplication, not assumed. */
function primesBelow65536(count){
  const out = [];
  for(let n = 65535; out.length < count && n > 2; n -= 2){
    let prime = true;
    for(let d = 3; d * d <= n; d += 2) if(n % d === 0){ prime = false; break; }
    if(prime) out.push(n);
  }
  return out;                             // descending
}
function makeSystem(m){
  if(m < 0 || m > 35) throw new Error("m must be 0..35");
  const primes = primesBelow65536(K + m);
  /* the worst case: the SMALLEST K primes of the set must cover a block */
  let prod = 1n;
  const sorted = primes.slice().sort((a, b) => a - b);
  for(let i = 0; i < K; i++) prod *= BigInt(sorted[i]);
  if(prod <= (1n << BigInt(8 * BLOCK))) throw new Error("prime set cannot cover a block");
  return {k: K, m, primes};
}

/* ---- encode: one file -> k+m shard buffers ---- */
function blockValue(bytes, off){
  let v = 0n;
  const end = Math.min(off + BLOCK, bytes.length);
  for(let i = off; i < end; i++) v = (v << 8n) | BigInt(bytes[i]);
  /* a short tail is padded on the right so every block is 1024 bits */
  if(end - off < BLOCK) v <<= BigInt(8 * (BLOCK - (end - off)));
  return v;
}

function encode(bytes, opts){
  const m = (opts && opts.m !== undefined) ? opts.m : 8;
  const sys = makeSystem(m);
  const blocks = Math.max(1, Math.ceil(bytes.length / BLOCK));
  /* header: magic, version, k, m, shard index, prime u16, blocks u32, len u32 */
  const HEAD = 18;
  const shards = sys.primes.map((p, idx) => {
    const b = new Uint8Array(HEAD + 2 * blocks);
    b.set(MAGIC, 0); b[4] = 3; b[5] = sys.k; b[6] = sys.m; b[7] = idx;
    b[8] = p & 0xff; b[9] = p >> 8;
    b[10] = blocks & 0xff; b[11] = (blocks >>> 8) & 0xff;
    b[12] = (blocks >>> 16) & 0xff; b[13] = (blocks >>> 24) & 0xff;
    b[14] = bytes.length & 0xff; b[15] = (bytes.length >>> 8) & 0xff;
    b[16] = (bytes.length >>> 16) & 0xff; b[17] = (bytes.length >>> 24) & 0xff;
    return b;
  });
  for(let c = 0; c < blocks; c++){
    const v = blockValue(bytes, c * BLOCK);
    for(let s = 0; s < shards.length; s++){
      const r = Number(v % BigInt(sys.primes[s]));
      shards[s][HEAD + 2 * c] = r & 0xff;
      shards[s][HEAD + 2 * c + 1] = r >> 8;
    }
  }
  return {shards, sys, blocks, length: bytes.length, HEAD};
}

/* ---- shard parsing ---- */
function parseShard(buf){
  if(buf.length < 18 || buf[0] !== MAGIC[0] || buf[1] !== MAGIC[1]
     || buf[2] !== MAGIC[2] || buf[3] !== MAGIC[3]) throw new Error("not an EGG3 shard");
  const k = buf[5], m = buf[6], idx = buf[7];
  const prime = buf[8] | (buf[9] << 8);
  const blocks = buf[10] | (buf[11] << 8) | (buf[12] << 16) | (buf[13] << 24);
  const length = buf[14] | (buf[15] << 8) | (buf[16] << 16) | (buf[17] << 24);
  if(buf.length < 18 + 2 * blocks) throw new Error("shard truncated");
  const res = c => buf[18 + 2 * c] | (buf[18 + 2 * c + 1] << 8);
  return {k, m, idx, prime, blocks, length, res};
}

/* ---- CRT over a fixed prime subset, precomputed once, reused per block ----
   Incremental Garner: x starts at r_0; at step i, t = (r_i - x) * inv(M mod
   p_i) mod p_i, then x += M * t, M *= p_i. The inverses depend only on the
   subset, so they are computed once. All small arithmetic stays under 2^32,
   which Number holds exactly. */
function modinv(a, p){
  let [old_r, r] = [a % p, p], [old_s, s] = [1, 0];
  while(r){ const q = Math.floor(old_r / r);
    [old_r, r] = [r, old_r - q * r]; [old_s, s] = [s, old_s - q * s]; }
  return ((old_s % p) + p) % p;
}
function crtPlan(primes){
  const plan = [];
  let M = 1n;
  for(const p of primes){
    plan.push({p, P: BigInt(p), M, inv: modinv(Number(M % BigInt(p)), p)});
    M *= BigInt(p);
  }
  return {plan, M};
}
function crtSolve(planObj, residues){
  const {plan} = planObj;
  let x = 0n;
  for(let i = 0; i < plan.length; i++){
    const {p, P, M, inv} = plan[i];
    const xi = Number(x % P);
    let t = ((residues[i] - xi) % p + p) % p;
    t = (t * inv) % p;                     // < 2^32, exact in Number
    x += M * BigInt(t);
  }
  return x;                                // < product of the subset's primes
}

/* ---- decode: any >= k shards -> the file, spares voting ----
   Per block: solve from the first k shards, then ask every spare to confirm.
   On dissent, leave-one-out over the alive shards finds the single shard
   whose exclusion restores unanimity -- conviction, not guesswork. Returns
   the bytes plus a tally of blocks confirmed / repaired / condemned. */
function decode(shardBufs){
  const parsed = shardBufs.map(parseShard);
  const {k, m, blocks, length} = parsed[0];
  for(const s of parsed)
    if(s.k !== k || s.m !== m || s.blocks !== blocks || s.length !== length)
      throw new Error("shards disagree about the file");
  const seen = new Set();
  const alive = parsed.filter(s => !seen.has(s.prime) && seen.add(s.prime));
  if(alive.length < k) throw new Error(`only ${alive.length} distinct shards; ${k} needed`);

  const LIMIT = 1n << BigInt(8 * BLOCK);
  const base = alive.slice(0, k);
  const spares = alive.slice(k);
  const plan = crtPlan(base.map(s => s.prime));
  /* leave-one-out plans are built lazily, once per excluded shard */
  const looPlans = new Map();

  const out = new Uint8Array(length);
  const tally = {confirmed: 0, repaired: 0, condemned: 0, suspects: new Set()};

  for(let c = 0; c < blocks; c++){
    let v = crtSolve(plan, base.map(s => s.res(c)));
    let good = v < LIMIT && spares.every(s => Number(v % BigInt(s.prime)) === s.res(c));
    let repairedHere = false;
    if(!good && spares.length){
      /* one of the alive shards lies about this block; find the exclusion
         that restores unanimity among all the others */
      for(let x = 0; x < alive.length; x++){
        const rest = alive.filter((_, i) => i !== x);
        if(rest.length < k) break;
        if(!looPlans.has(x)) looPlans.set(x, crtPlan(rest.slice(0, k).map(s => s.prime)));
        const w = crtSolve(looPlans.get(x), rest.slice(0, k).map(s => s.res(c)));
        if(w < LIMIT && rest.slice(k).every(s => Number(w % BigInt(s.prime)) === s.res(c))){
          v = w; tally.suspects.add(alive[x].prime);
          good = true; repairedHere = true;
          break;
        }
      }
    }
    if(!good){ tally.condemned++; if(v >= LIMIT) v = 0n; }
    else if(repairedHere) tally.repaired++;
    else tally.confirmed++;
    /* write the block */
    const start = c * BLOCK, take = Math.min(BLOCK, length - start);
    for(let i = BLOCK - 1; i >= 0; i--){
      const byte = Number(v & 0xffn); v >>= 8n;
      if(i < take) out[start + i] = byte;
    }
  }
  tally.suspects = [...tally.suspects];
  return {bytes: out, ...tally, alive: alive.length, k, m};
}

/* ---- arithmetic in the encoded domain ----
   Residue streams add pointwise mod their own prime: carries cannot cross
   shards, because each shard is its own world. The reconstruction of the sum
   is the exact numeric sum of the block values -- verified in the tests
   against plain BigInt. Scale works the same way for small factors. */
function shardAdd(bufA, bufB){
  const a = parseShard(bufA), b = parseShard(bufB);
  if(a.prime !== b.prime || a.blocks !== b.blocks) throw new Error("shard mismatch");
  const out = Uint8Array.from(bufA);
  for(let c = 0; c < a.blocks; c++){
    const r = (a.res(c) + b.res(c)) % a.prime;
    out[18 + 2 * c] = r & 0xff; out[18 + 2 * c + 1] = r >> 8;
  }
  return out;
}
function shardScale(buf, factor){
  const a = parseShard(buf);
  const out = Uint8Array.from(buf);
  for(let c = 0; c < a.blocks; c++){
    const r = (a.res(c) * (factor % a.prime)) % a.prime;
    out[18 + 2 * c] = r & 0xff; out[18 + 2 * c + 1] = r >> 8;
  }
  return out;
}

/* decode a shard set to BLOCK VALUES rather than bytes -- the honest output
   for arithmetic results, whose sums may exceed a block's 1024 bits */
function decodeValues(shardBufs){
  const parsed = shardBufs.map(parseShard);
  const {k, blocks} = parsed[0];
  const seen = new Set();
  const alive = parsed.filter(s => !seen.has(s.prime) && seen.add(s.prime));
  if(alive.length < k) throw new Error(`only ${alive.length} distinct shards; ${k} needed`);
  const base = alive.slice(0, k);
  const plan = crtPlan(base.map(s => s.prime));
  const vals = [];
  for(let c = 0; c < blocks; c++) vals.push(crtSolve(plan, base.map(s => s.res(c))));
  return vals;
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {BLOCK, K, encode, decode, decodeValues, parseShard,
                    shardAdd, shardScale, makeSystem, blockValue, crtPlan, crtSolve};
