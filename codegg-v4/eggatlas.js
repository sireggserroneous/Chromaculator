/* codegg v4 -- the Atlas permutation. Not one byte bigger.
 *
 * v3 was called out twice, correctly: its size grew, and its machinery (CRT,
 * redundant residues) was the site's KIN rather than the site's OWN. This
 * version answers both. The power used here is bitrev -- stalk.js:156, the
 * function the Atlas itself is built on, the van der Corput ordering that
 * inspirations.html credits and that four rounds of measurement kept calling
 * the one dramatic, honest, unclaimed property in the whole system.
 *
 * The encoder is a pure permutation. Byte j of the output is byte sigma(j) of
 * the input, where sigma enumerates positions in bit-reversed order. So:
 *
 *   SIZE IS EXACT. Output length equals input length. No header, no
 *   container, no padding. At power-of-two sizes the permutation is an
 *   involution -- encoding twice is the identity, the same way the fold map
 *   undoes itself and negation flips every colour back.
 *
 *   EVERY PREFIX IS A UNIFORM SAMPLE OF THE WHOLE. The first t bytes of the
 *   encoding are t low-discrepancy positions of the original -- measured in
 *   this conversation at O(log n / n) discrepancy against O(1/sqrt n) for
 *   random. Truncate the encoded file anywhere and every region of the
 *   original is present at that density: loss of the tail is loss of
 *   RESOLUTION, not loss of a region. An ordinary file truncated at 30%
 *   simply does not have its last 70%.
 *
 *   BURSTS SCATTER. Consecutive encoded positions are maximally separated in
 *   the original, so a contiguous wound of B bytes lands as ~B isolated
 *   wounds spread evenly -- which converts the exact failure case codec-v1
 *   and codegg-v1 both admitted (the burst) into the case they are best at
 *   (scattered singles and erasures). armor()/recover() below runs that
 *   pipeline: Atlas permutation for storage, codegg-v1 residues for repair,
 *   and a multi-kilobyte contiguous burn comes back exact. The permutation's
 *   contribution to that costs zero bytes.
 *
 * A permutation cannot compress (it is a bijection -- the counting argument
 * this conversation proved twice) and cannot survive DELETION of bytes
 * (survival needs redundancy; that is a law, not a design choice). What it
 * re-encodes is WHERE damage lands, and that turns out to be worth more than
 * it sounds. */

/* bitrev lives in stalk.js. In the browser it is a global; under node the
   file is evaluated here, the same move every codec in this series makes. */
if(typeof bitrev === "undefined" && typeof require !== "undefined"){
  eval(require("fs").readFileSync(require("path").join(__dirname, "..", "stalk.js"), "utf8"));
}

/* the permutation: positions 0..n-1 in van der Corput order.
   sigma[j] = the j-th value of bitrev(i, bits) that lands under n. */
function atlasOrder(n){
  if(n <= 1) return Uint32Array.from(n === 1 ? [0] : []);
  let bits = 1;
  while((1 << bits) < n) bits++;
  const sigma = new Uint32Array(n);
  let j = 0;
  for(let i = 0; i < (1 << bits); i++){
    const p = bitrev(i, bits);
    if(p < n) sigma[j++] = p;
  }
  return sigma;
}

function encode(bytes){
  const sigma = atlasOrder(bytes.length);
  const out = new Uint8Array(bytes.length);
  for(let j = 0; j < bytes.length; j++) out[j] = bytes[sigma[j]];
  return out;
}
function decode(bytes){
  const sigma = atlasOrder(bytes.length);
  const out = new Uint8Array(bytes.length);
  for(let j = 0; j < bytes.length; j++) out[sigma[j]] = bytes[j];
  return out;
}

/* a truncated encoding, placed back: the bytes we have go to their homes,
   the rest stay zero, and the caller learns exactly which positions arrived */
function placePrefix(prefix, fullLength){
  const sigma = atlasOrder(fullLength);
  const out = new Uint8Array(fullLength);
  const have = new Uint8Array(fullLength);
  for(let j = 0; j < prefix.length && j < fullLength; j++){
    out[sigma[j]] = prefix[j]; have[sigma[j]] = 1;
  }
  return {bytes: out, have};
}

/* ---- the armor pipeline: Atlas ordering + codegg-v1 residues ----
 *
 * Protection is computed in the ORIGINAL domain (codegg-v1 squares over the
 * file as it is), storage happens in the ATLAS domain (bit-level permutation,
 * so even a single damaged byte scatters into eight far-apart bits). A
 * contiguous burn in storage therefore lands on each 128-byte square as a
 * few isolated cells -- inside codegg-v1's erasure capacity -- instead of
 * annihilating 32 consecutive squares.
 *
 * armor(file)  -> {stored, payload}   stored: permuted bits; payload: v1 checks
 * recover(stored, payload, woundStart, woundLen) -> the file, repaired
 */
function bitGet(b, i){ return (b[i >> 3] >> (7 - (i & 7))) & 1; }
function bitSet(b, i, v){
  if(v) b[i >> 3] |= 1 << (7 - (i & 7));
  else b[i >> 3] &= ~(1 << (7 - (i & 7)));
}
function atlasBits(bytes){
  const nb = bytes.length * 8, sigma = atlasOrder(nb);
  const out = new Uint8Array(bytes.length);
  for(let j = 0; j < nb; j++) bitSet(out, j, bitGet(bytes, sigma[j]));
  return out;
}
function unAtlasBits(bytes){
  const nb = bytes.length * 8, sigma = atlasOrder(nb);
  const out = new Uint8Array(bytes.length);
  for(let j = 0; j < nb; j++) bitSet(out, sigma[j], bitGet(bytes, j));
  return out;
}

function armor(bytes, G){                 // G: the codegg-v1 module
  const payload = G.encode(bytes, {N: 32});
  return {stored: atlasBits(bytes), payload};
}
function recover(stored, payload, woundStart, woundLen, G){
  const nb = stored.length * 8, sigma = atlasOrder(nb);
  /* where did the burn land in the original? Known positions -> erasures. */
  const erased = new Map();
  for(let j = woundStart * 8; j < (woundStart + woundLen) * 8 && j < nb; j++){
    const orig = sigma[j];
    const sq = Math.floor(orig / 1024), cell = orig % 1024;
    if(!erased.has(sq)) erased.set(sq, []);
    erased.get(sq).push(cell);
  }
  const file = unAtlasBits(stored);
  /* rebuild the payload's squares from the damaged file, then let the
     residues settle the flagged cells */
  const fresh = G.encode(file, {N: 32, code: payload.code});
  const p = {squares: fresh.squares, checks: payload.checks,
             meta: payload.meta, code: payload.code};
  const out = G.decode(p, {erased});
  let worst = 0; for(const v of erased.values()) worst = Math.max(worst, v.length);
  return {...out, squaresWounded: erased.size, worstPerSquare: worst};
}

if(typeof module !== "undefined" && module.exports)
  module.exports = {atlasOrder, encode, decode, placePrefix,
                    atlasBits, unAtlasBits, armor, recover};
