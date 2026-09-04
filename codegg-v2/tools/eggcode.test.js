/* node codegg-v2/tools/eggcode.test.js -- recipes where the powers reach.
 *
 * The claim under test is not "it compresses" -- counting forbids that in
 * general and #4 demonstrates the forbidding on 1.489 MB of noise. The claim
 * is narrower and checkable: every chunk a power can genuinely reach is
 * taken, every chunk it cannot is carried verbatim, the ledger matches the
 * corpus's ground truth, and the container always reproduces its input. */
const E = require(__dirname + "/../eggcode.js");
const fs = require("fs"), path = require("path"), zlib = require("zlib"),
      cp = require("child_process");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(1489);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const roundtrip = (src, opts) => {
  const r = E.encode(src, opts);
  const d = E.decode(r.packed);
  ok(Buffer.from(d.bytes).equals(Buffer.from(src)), "round-trip mismatch");
  return {r, d};
};

/* 1. round-trip on the awkward shapes */
{
  for(const [label, src] of [
    ["empty", new Uint8Array(0)],
    ["one byte", Uint8Array.from([7])],
    ["127 B (all tail)", randBytes(127)],
    ["128 B exactly", randBytes(128)],
    ["129 B", randBytes(129)],
    ["all zero 1000 B", new Uint8Array(1000)],
    ["random 10 KB", randBytes(10240)],
  ]) roundtrip(src);
  console.log("  round-trip exact: 7 shapes");
}

/* 2. each power takes its designed chunk, and only then */
{
  const one = (mk, want) => {
    const src = new Uint8Array(E.CHUNK);
    mk(src);
    const {r} = roundtrip(src);
    const got = E.OPNAME[r.map[0]];
    ok(got === want, `wanted ${want}, auction gave ${got}`);
  };
  one(() => {}, "green");
  one(s => { const pat = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]; // period 11
             for(let i = 0; i < s.length; i++) s[i] = pat[i % 11]; }, "bar");
  one(s => { let i = 0, v = 0;                              // long 0/1 runs
             while(i < s.length){ for(let j = 0; j < 24 && i < s.length; j++) s[i++] = v; v ^= 0xff; } }, "naf");
  one(s => { for(let i = 0; i < s.length; i++) s[i] = (i * 37 + 11) & 0xff; }, "literal");
  /* prev needs two chunks */
  {
    const src = new Uint8Array(2 * E.CHUNK);
    for(let i = 0; i < E.CHUNK; i++) src[i] = src[i + E.CHUNK] = (i * 91 + 5) & 0xff;
    const {r} = roundtrip(src);
    ok(E.OPNAME[r.map[1]] === "prev", `second chunk went ${E.OPNAME[r.map[1]]}`);
  }
  console.log("  green / bar / naf / literal / prev each claim their designed chunk");
}

/* 3. DIV fires on a genuine truncated rational and reproduces it exactly.
      5/37 has bit-period 36, byte-period 9 -- long enough that BAR cannot
      undercut DIV's three bytes. */
{
  const SHIFT = BigInt(8 * E.CHUNK);
  let v = (5n << SHIFT) / 37n;
  const src = new Uint8Array(E.CHUNK);
  for(let i = E.CHUNK - 1; i >= 0; i--){ src[i] = Number(v & 0xffn); v >>= 8n; }
  const {r} = roundtrip(src);
  ok(E.OPNAME[r.map[0]] === "div", `5/37 chunk went ${E.OPNAME[r.map[0]]}`);
  const spent = r.stats.div.outBytes;
  console.log(`  div claims the 5/37 expansion: ${E.CHUNK} B reproduced from ${spent} B`);
}

/* 4. the counting argument, made flesh at the requested size:
      1,489,000 bytes of noise come out LARGER, every auction to literal. */
{
  const src = randBytes(1489000);
  const {r} = roundtrip(src);
  const ratio = r.packed.length / src.length;
  const recipes = r.map.filter(op => op !== E.OP.LITERAL && op !== E.OP.TAIL).length;
  ok(r.packed.length > src.length, "noise came out smaller -- counting is broken");
  ok(recipes === 0, `${recipes} recipe chunks on pure noise`);
  console.log(`  1,489,000 B of noise -> ${r.packed.length} B (${(100 * ratio).toFixed(2)}%),`
    + ` 0 recipes: LARGER, as counting requires`);
}

/* 5. the labelled corpus: the ledger must match ground truth, and gzip must
      win overall -- asserting the reference's victory keeps the report honest */
{
  const cpath = path.join(__dirname, "..", "corpus-1489k.bin");
  if(!fs.existsSync(cpath))
    cp.execSync(`node "${path.join(__dirname, "mkcorpus.js")}" "${cpath}"`);
  const src = fs.readFileSync(cpath);
  ok(src.length === 1489000, "corpus is the wrong size");
  const {r} = roundtrip(src);
  const s = r.stats;
  ok(s.green.chunks === 2343, `green took ${s.green.chunks}, zeros region is 2343 chunks`);
  ok(s.prev.chunks >= 1100, `prev took only ${s.prev.chunks} of the periodic region`);
  ok(s.naf.chunks >= 1100, `naf took only ${s.naf.chunks} of the runs region`);
  ok(s.div.chunks === 0, `div claimed ${s.div.chunks} chunks of real data`);
  const ratio = r.packed.length / src.length;
  const gz = zlib.gzipSync(src, {level: 9}).length / src.length;
  ok(ratio < 0.65, `corpus ratio ${ratio.toFixed(3)} not under 0.65`);
  ok(gz < ratio, "gzip lost to the powers -- something is miscounted");
  console.log(`  corpus 1,489,000 B -> ${(100 * ratio).toFixed(2)}%; ledger matches ground truth;`
    + ` gzip ${(100 * gz).toFixed(2)}% still wins`);
  console.log(`    naf's own country: ${s.naf.inBytes} B reproduced from ${s.naf.outBytes} B`
    + ` (${(s.naf.inBytes / s.naf.outBytes).toFixed(1)}x)`);
}

/* 6. --check: codegg-v1's residues ride along and catch a corrupted container */
{
  const src = randBytes(4000);
  const {r} = roundtrip(src, {check: true});
  const clean = E.decode(r.packed);
  ok(clean.check && clean.verified === Math.floor(src.length / E.CHUNK) && clean.failed === 0,
    `clean container verified ${clean.verified}, failed ${clean.failed}`);
  /* flip one byte inside the first literal chunk's payload */
  const bad = Uint8Array.from(r.packed);
  bad[10 + 1 + 5] ^= 0x40;                  // header 10, opcode 1, then payload
  const d = E.decode(bad);
  ok(d.failed >= 1, "corrupted container passed its own residues");
  console.log(`  --check: ${clean.verified} chunks verified clean; 1 flipped byte -> ${d.failed} chunk flagged`);
}

/* 7. tombstone for the popcount filter: an alternating-runs chunk has ~512
      ones and NAF weight ~10; the first filter rejected it by popcount and
      the corpus ledger showed naf taking 0 chunks of its own country. */
{
  const s = new Uint8Array(E.CHUNK);
  let i = 0, v = 0;
  while(i < s.length){ for(let j = 0; j < 20 && i < s.length; j++) s[i++] = v; v ^= 0xff; }
  let ones = 0; for(const b of s){ let x = b; while(x){ ones += x & 1; x >>= 1; } }
  ok(ones > 400 && ones < 624, `test chunk popcount ${ones} not mid-range`);
  const bid = E.bidNaf(s, 0);
  ok(bid && bid.cost < 30, `naf did not bid on its own country (popcount ${ones})`);
  console.log(`  naf bids ${bid.cost} B on a ${ones}-ones run chunk -- the popcount filter stays dead`);
}

/* 8. a container that is not one is refused */
{
  let threw = false;
  try { E.decode(randBytes(64)); } catch(e){ threw = true; }
  ok(threw, "garbage was accepted as a container");
  console.log("  garbage container refused");
}

console.log("eggcode ok");
