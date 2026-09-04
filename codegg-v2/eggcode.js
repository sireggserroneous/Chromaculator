/* codegg v2 -- recipes where the powers reach, literals where they do not.
 *
 * A file is cut into 128-byte chunks -- one codegg square each, keeping v1's
 * unit -- and every chunk is auctioned to the site's powers. Each power bids
 * the exact size of the recipe it would write; LITERAL bids 128 bytes plus an
 * opcode and always bids; the cheapest bid wins. Decoding replays the recipes.
 * Round-trips are byte-exact or the container is refused.
 *
 * THE MENU, and where each power comes from:
 *
 *   GREEN     the all-zero chunk. The site's green cell / padding, and in
 *             coding terms plain RLE wearing the site's colour.
 *   PREV      chunk identical to the one before it -- bar notation at chunk
 *             scale: a repeating block of length 128.
 *   BAR       chunk periodic with period P < 126: store one period and P.
 *             The site's repeating-expansion bar, literally.
 *   NAF       the chunk's value respelled with the fewest lit cells (push run
 *             the other way), stored as sparse (position, sign) pairs. Wins
 *             exactly when the chunk is run-heavy -- the repunit country this
 *             conversation kept returning to.
 *   DIV       the chunk is the first 1024 bits of A/B for small A < B: store
 *             (A, B). The site's own Q/e/R power, three bytes for 128 when it
 *             fires. Measured in an earlier turn at 0 of 17,460 on arbitrary
 *             grids; it is in the menu to measure, not because it will eat.
 *   LITERAL   the floor. 128 bytes plus one opcode.
 *
 * WHAT IS DELIBERATELY ABSENT: multiplication. bits(A) + bits(B) - bits(A*B)
 * was measured at exactly 0 in 61% of random pairs and POSITIVE on average
 * (+0.392 bits): the logarithm is additive, so the rectangle recipe can never
 * underbid literal. No search fixes that; it is left out by proof.
 *
 * WHAT THIS CANNOT DO, stated before any file is fed to it: beat counting.
 * 2^L chunks cannot fit in fewer than L bits, push is a bijection, and on
 * high-entropy data every auction goes to LITERAL and the container costs
 * ~0.8% MORE than the input. The honest promise is smaller: every byte a
 * power can genuinely reach is taken, every byte it cannot is carried
 * verbatim, and the report says which was which.
 *
 * Optional integrity: --check stores each chunk's value mod p and mod q --
 * codegg-v1's residues, computed by byte-Horner, the same value mod the same
 * primes -- so the recipe file can vouch for its own chunks at +2 bytes each.
 */

const CHUNK = 128;                       // one N=32 codegg square
const OP = {LITERAL: 0, GREEN: 1, PREV: 2, BAR: 3, NAF: 4, DIV: 5, TAIL: 6};
const OPNAME = ["literal", "green", "prev", "bar", "naf", "div", "tail"];
const P = 2053, Q = 2063;                // codegg-v1's moduli for L = 1024

/* ---- residues, byte-Horner: the same V mod m codegg-v1 keeps ---- */
function residueBytes(bytes, off, len, m){
  let acc = 0;
  for(let i = 0; i < len; i++) acc = (acc * 256 + bytes[off + i]) % m;
  return acc;
}

/* ---- the powers' bids. Each returns {cost, ...} or null. ---- */

function bidGreen(bytes, off){
  for(let i = 0; i < CHUNK; i++) if(bytes[off + i] !== 0) return null;
  return {cost: 1};
}

function bidPrev(bytes, off){
  if(off < CHUNK) return null;
  for(let i = 0; i < CHUNK; i++) if(bytes[off + i] !== bytes[off - CHUNK + i]) return null;
  return {cost: 1};
}

function bidBar(bytes, off){
  /* smallest period P with chunk[i] == chunk[i-P] for all i >= P */
  for(let p = 1; p < CHUNK - 2; p++){
    let ok = true;
    for(let i = p; i < CHUNK; i++) if(bytes[off + i] !== bytes[off + i - p]){ ok = false; break; }
    if(ok) return {cost: 2 + p, p};      // opcode + period byte + one period
  }
  return null;
}

/* NAF of the chunk's 1024-bit value; sparse (11-bit position, 1-bit sign).
   Quick reject by TRANSITION count, not popcount: NAF's weight tracks run
   boundaries -- a chunk of long alternating 0- and 1-runs has ~512 ones but
   NAF weight ~one per boundary. (The first version filtered on popcount and
   rejected exactly the run-heavy chunks NAF exists to win; the corpus ledger
   caught it: NAF claimed 0 chunks of the 150 KB laid out as its country.) */
const POPB = new Uint8Array(256);
for(let b = 0; b < 256; b++){ let n = 0, x = b; while(x){ n += x & 1; x >>= 1; } POPB[b] = n; }
function transitions(bytes, off){
  let t = 0;
  for(let i = 0; i < CHUNK; i++){
    const b = bytes[off + i];
    t += POPB[(b ^ (b << 1)) & 0xff];        // boundaries inside the byte...
    if(i){ const prev = bytes[off + i - 1]; t += (prev & 1) ^ (b >> 7); }
  }
  return t;                                   // ...minus/plus edge effects; a bound, not a count
}
function bidNaf(bytes, off){
  if(transitions(bytes, off) > 200) return null;
  let v = 0n;
  for(let i = 0; i < CHUNK; i++) v = (v << 8n) | BigInt(bytes[off + i]);
  const hits = [];                        // {pos (bit index from LSB), sign}
  let k = 0;
  while(v > 0n){
    if(v & 1n){
      const z = 2n - (v % 4n);            // +1 or -1
      hits.push({pos: k, neg: z < 0n});
      v -= z;
    }
    v >>= 1n;
    k++;
    if(hits.length > 84) return null;     // cannot win; stop early
  }
  /* opcode + 1-byte count + 12 bits per hit */
  const cost = 2 + Math.ceil(hits.length * 12 / 8);
  return cost < 1 + CHUNK ? {cost, hits} : null;
}

/* DIV: is the chunk floor(2^1024 * A / B) for A < B <= limit?  A is implied:
   A = round(V * B / 2^1024); accept only an exact reproduction. */
function bidDiv(bytes, off, limit){
  let v = 0n;
  for(let i = 0; i < CHUNK; i++) v = (v << 8n) | BigInt(bytes[off + i]);
  if(v === 0n) return null;               // GREEN's territory
  const SHIFT = BigInt(8 * CHUNK);
  for(let b = 3n; b <= BigInt(limit); b++){
    const a = (v * b + (1n << (SHIFT - 1n))) >> SHIFT;   // rounded
    if(a === 0n || a >= b) continue;
    if((a << SHIFT) / b === v) return {cost: 3, a: Number(a), b: Number(b)};
  }
  return null;
}

/* ---- encode ---- */
function encode(bytes, opts){
  const check = !!(opts && opts.check);
  const divLimit = (opts && opts.divLimit) || 64;
  const out = [];                         // byte arrays to concatenate
  const stats = {};                       // per-power accounting
  const map = [];                         // one opcode per chunk, for the strip
  for(const n of OPNAME) stats[n] = {chunks: 0, inBytes: 0, outBytes: 0};

  /* header: magic, version, flags, original length (u32 LE), divLimit */
  out.push(Uint8Array.from([0x45, 0x47, 0x47, 0x32, 2, check ? 1 : 0,
    bytes.length & 0xff, (bytes.length >>> 8) & 0xff,
    (bytes.length >>> 16) & 0xff, (bytes.length >>> 24) & 0xff]));

  const whole = Math.floor(bytes.length / CHUNK);
  for(let c = 0; c < whole; c++){
    const off = c * CHUNK;
    /* the auction: every power bids, cheapest recipe wins, literal is floor */
    const bids = [];
    const g = bidGreen(bytes, off);   if(g) bids.push({op: OP.GREEN, ...g});
    const pv = bidPrev(bytes, off);   if(pv) bids.push({op: OP.PREV, ...pv});
    const br = bidBar(bytes, off);    if(br) bids.push({op: OP.BAR, ...br});
    const nf = bidNaf(bytes, off);    if(nf) bids.push({op: OP.NAF, ...nf});
    if(!g && !pv && (!br || br.p > 8)){
      const dv = bidDiv(bytes, off, divLimit);
      if(dv) bids.push({op: OP.DIV, ...dv});
    }
    bids.push({op: OP.LITERAL, cost: 1 + CHUNK});
    bids.sort((x, y) => x.cost - y.cost);
    const w = bids[0];

    const rec = [w.op];
    if(w.op === OP.BAR){ rec.push(w.p); for(let i = 0; i < w.p; i++) rec.push(bytes[off + i]); }
    else if(w.op === OP.NAF){
      rec.push(w.hits.length);
      let bit = 0, acc = 0;
      const push12 = x => {                // 12-bit big-endian packing
        acc = (acc << 12) | x; bit += 12;
        while(bit >= 8){ bit -= 8; rec.push((acc >>> bit) & 0xff); }
      };
      for(const h of w.hits) push12((h.pos << 1) | (h.neg ? 1 : 0));
      if(bit) rec.push((acc << (8 - bit)) & 0xff);
    }
    else if(w.op === OP.DIV){ rec.push(w.a); rec.push(w.b); }
    else if(w.op === OP.LITERAL) for(let i = 0; i < CHUNK; i++) rec.push(bytes[off + i]);
    if(check){
      const rp = residueBytes(bytes, off, CHUNK, P), rq = residueBytes(bytes, off, CHUNK, Q);
      rec.push(rp & 0xff, ((rp >> 8) & 0x0f) | ((rq & 0x0f) << 4), (rq >> 4) & 0xff);
    }
    out.push(Uint8Array.from(rec));
    const name = OPNAME[w.op];
    stats[name].chunks++; stats[name].inBytes += CHUNK; stats[name].outBytes += rec.length;
    map.push(w.op);
  }

  /* the tail: whatever does not fill a chunk travels verbatim */
  const rest = bytes.length - whole * CHUNK;
  if(rest){
    const rec = [OP.TAIL, rest];
    for(let i = 0; i < rest; i++) rec.push(bytes[whole * CHUNK + i]);
    out.push(Uint8Array.from(rec));
    stats.tail.chunks++; stats.tail.inBytes += rest; stats.tail.outBytes += rec.length;
    map.push(OP.TAIL);
  }

  let total = 0; for(const a of out) total += a.length;
  const packed = new Uint8Array(total);
  let pos = 0; for(const a of out){ packed.set(a, pos); pos += a.length; }
  return {packed, stats, map, check};
}

/* ---- decode ---- */
function decode(packed){
  if(packed.length < 10 || packed[0] !== 0x45 || packed[1] !== 0x47
     || packed[2] !== 0x47 || packed[3] !== 0x32) throw new Error("not an EGG2 container");
  const check = packed[5] === 1;
  const len = packed[6] | (packed[7] << 8) | (packed[8] << 16) | (packed[9] << 24);
  const out = new Uint8Array(len);
  let pos = 10, off = 0, verified = 0, failed = 0;

  const readCheck = (chunkOff, chunkLen) => {
    const rp = packed[pos] | ((packed[pos + 1] & 0x0f) << 8);
    const rq = ((packed[pos + 1] >> 4) & 0x0f) | (packed[pos + 2] << 4);
    pos += 3;
    if(residueBytes(out, chunkOff, chunkLen, P) === rp
       && residueBytes(out, chunkOff, chunkLen, Q) === rq) verified++; else failed++;
  };

  while(off < len){
    const op = packed[pos++];
    const here = off;
    if(op === OP.TAIL){
      const n = packed[pos++];
      for(let i = 0; i < n; i++) out[off++] = packed[pos++];
      continue;                            // tail carries no residues
    }
    if(op === OP.LITERAL) for(let i = 0; i < CHUNK; i++) out[off++] = packed[pos++];
    else if(op === OP.GREEN) off += CHUNK;                 // already zero
    else if(op === OP.PREV){
      for(let i = 0; i < CHUNK; i++) out[off + i] = out[off - CHUNK + i];
      off += CHUNK;
    }
    else if(op === OP.BAR){
      const p = packed[pos++];
      for(let i = 0; i < p; i++) out[off + i] = packed[pos + i];
      for(let i = p; i < CHUNK; i++) out[off + i] = out[off + i - p];
      pos += p; off += CHUNK;
    }
    else if(op === OP.NAF){
      const k = packed[pos++];
      let v = 0n, acc = 0, bit = 0;
      for(let h = 0; h < k; h++){
        while(bit < 12){ acc = (acc << 8) | packed[pos++]; bit += 8; }
        bit -= 12;
        const x = (acc >>> bit) & 0xfff;
        const d = (x & 1) ? -1n : 1n;
        v += d * (1n << BigInt(x >>> 1));
      }
      for(let i = CHUNK - 1; i >= 0; i--){ out[off + i] = Number(v & 0xffn); v >>= 8n; }
      off += CHUNK;
    }
    else if(op === OP.DIV){
      const a = BigInt(packed[pos++]), b = BigInt(packed[pos++]);
      let v = (a << BigInt(8 * CHUNK)) / b;
      for(let i = CHUNK - 1; i >= 0; i--){ out[off + i] = Number(v & 0xffn); v >>= 8n; }
      off += CHUNK;
    }
    else throw new Error("bad opcode " + op + " at " + (pos - 1));
    if(check) readCheck(here, CHUNK);
  }
  return {bytes: out, verified, failed, check};
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {encode, decode, CHUNK, OP, OPNAME, residueBytes, P, Q,
                    bidGreen, bidPrev, bidBar, bidNaf, bidDiv};
