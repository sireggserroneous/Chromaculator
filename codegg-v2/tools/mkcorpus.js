/* node codegg-v2/tools/mkcorpus.js [out] -- a 1,489,000-byte file of known
 * composition, so the encoder's report can be checked against ground truth
 * instead of admired. Deterministic: same bytes every run.
 *
 * regions, in order:
 *   text      300,000 B   the repo's own prose (md files, tiled)
 *   code      200,000 B   the repo's own JavaScript (tiled)
 *   zeros     300,000 B   the sparse-disk-image case         -> GREEN country
 *   periodic  150,000 B   a 16-byte pattern repeated         -> PREV/BAR country
 *   runs      150,000 B   long 0-runs and 1-runs interleaved -> NAF country
 *   random    389,000 B   seeded noise                       -> nobody's country
 *
 * The honest predictions, filed before the encoder ever sees it: zeros,
 * periodic and runs are captured (and plain RLE would capture them just as
 * well); text, code and random go literal, each paying the opcode byte. */
const fs = require("fs"), path = require("path");
const ROOT = path.join(__dirname, "..", "..");

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

function tiled(files, n){
  const src = Buffer.concat(files.map(f => fs.readFileSync(path.join(ROOT, f))));
  const out = Buffer.alloc(n);
  for(let i = 0; i < n; i += src.length) src.copy(out, i, 0, Math.min(src.length, n - i));
  return out;
}

const parts = [];
parts.push(["text", tiled(["spec.md", "README.md", "tools/README.md"], 300000)]);
parts.push(["code", tiled(["stalk.js", "chroma-ui.js", "codec-v1/chromacode.js"], 200000)]);
parts.push(["zeros", Buffer.alloc(300000)]);
{
  const pat = Buffer.from("chronochroma16B!");            // 16 bytes
  const b = Buffer.alloc(150000);
  for(let i = 0; i < b.length; i++) b[i] = pat[i % 16];
  parts.push(["periodic", b]);
}
{
  /* run-heavy: alternating runs of 0x00 and 0xFF, lengths 16..47 bytes --
     exactly the repunit country NAF was measured to win in */
  const g = mul32(777);
  const b = Buffer.alloc(150000);
  let i = 0, v = 0;
  while(i < b.length){
    const len = 16 + g() % 32;
    for(let j = 0; j < len && i < b.length; j++) b[i++] = v;
    v ^= 0xff;
  }
  parts.push(["runs", b]);
}
{
  const g = mul32(424242);
  const b = Buffer.alloc(389000);
  for(let i = 0; i < b.length; i++) b[i] = g() & 0xff;
  parts.push(["random", b]);
}

const total = Buffer.concat(parts.map(p => p[1]));
if(total.length !== 1489000) throw new Error("corpus is " + total.length + ", wanted 1489000");
const out = process.argv[2] || path.join(__dirname, "..", "corpus-1489k.bin");
fs.writeFileSync(out, total);
let off = 0;
console.log(`${out}: ${total.length} bytes (~1.489 MB)`);
for(const [name, b] of parts){
  console.log(`  ${name.padEnd(9)} ${String(off).padStart(8)} .. ${String(off + b.length - 1).padStart(8)}  (${b.length} B)`);
  off += b.length;
}
