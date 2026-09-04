/* node codegg-v4/tools/atlas.js <cmd> <file> -- the Atlas permutation, on disk.
 *
 * encode <file> [--out p]      write the permuted file. Same size, always.
 * decode <file> [--out p]      undo it (at powers of two, encode IS decode).
 * prefix <file> --keep 25      demo: cut the ENCODED file at 25%, place what
 *                              survives, report per-window coverage, write
 *                              the partial reconstruction.
 * burst  <file> [--len 4096]   demo: where would a contiguous wound land?
 *                              Reports scatter, writes nothing.
 *
 * There is no container and no header: the transform is defined entirely by
 * the file's length, like rev or a rotation. It cannot compress (bijection),
 * cannot survive deletion (that costs redundancy -- see codegg-v3), and does
 * not care what the bytes mean. What it re-encodes is WHERE things land:
 * prefixes become uniform samples, bursts become dust. */
const fs = require("fs"), path = require("path");
const A = require(path.join(__dirname, "..", "eggatlas.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["out", "keep", "len"]);
const flag = (n, d) => { const i = argv.indexOf("--" + n);
  return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const bare = [];
for(let i = 0; i < argv.length; i++){
  const a = argv[i];
  if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; }
  bare.push(a);
}
const [cmd, file] = bare;
if(!cmd || !file){ console.error("usage: atlas.js encode|decode|prefix|burst <file> [--out p] [--keep 25] [--len 4096]"); process.exit(1); }
const src = fs.readFileSync(file);

if(cmd === "encode" || cmd === "decode"){
  const t0 = Date.now();
  const out = (cmd === "encode" ? A.encode : A.decode)(src);
  const dst = flag("out", file + (cmd === "encode" ? ".atlas" : ".plain"));
  fs.writeFileSync(dst, out);
  console.log(`${file}: ${src.length} B -> ${out.length} B (identical size, always) in ${Date.now() - t0} ms`);
  console.log(`  wrote ${dst}`);
}
else if(cmd === "prefix"){
  const keep = +flag("keep", 25);
  const enc = A.encode(src);
  const t = Math.floor(src.length * keep / 100);
  const {bytes, have} = A.placePrefix(enc.subarray(0, t), src.length);
  const W = Math.max(1024, Math.floor(src.length / 64));
  let lo = 1, hi = 0;
  for(let w = 0; w + W <= src.length; w += W){
    let c = 0; for(let i = w; i < w + W; i++) c += have[i];
    lo = Math.min(lo, c / W); hi = Math.max(hi, c / W);
  }
  const dst = flag("out", file + ".partial");
  fs.writeFileSync(dst, bytes);
  console.log(`OK -- kept ${keep}% of the encoding`);
  console.log(`  this file:  every part survives at ~${keep}%`
    + `  (measured ${(100 * lo).toFixed(1)}%..${(100 * hi).toFixed(1)}% per window)`);
  console.log(`  plain file: everything after the ${keep}% mark would be gone`);
  console.log(`  wrote ${dst}`);
}
else if(cmd === "burst"){
  const B = +flag("len", 4096);
  const sigma = A.atlasOrder(src.length);
  const start = Math.floor(src.length / 2);
  const perBlock = new Map();
  for(let j = start; j < Math.min(start + B, src.length); j++){
    const b = Math.floor(sigma[j] / 128);
    perBlock.set(b, (perBlock.get(b) || 0) + 1);
  }
  const worst = Math.max(...perBlock.values());
  console.log(`OK -- simulated a ${B}-byte scratch on the encoded file`);
  console.log(`  this file:  ${perBlock.size} tiny nicks, at most ${worst}`
    + ` byte${worst > 1 ? "s" : ""} each, spread evenly (repairable)`);
  console.log(`  plain file: one ${B}-byte hole, ${Math.ceil(B / 128)} blocks destroyed outright`);
}
else { console.error("unknown command " + cmd); process.exit(1); }
