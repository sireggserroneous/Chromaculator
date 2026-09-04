/* node codegg-v2/tools/eggpack.js <file> [flags] -- feed a file to the powers.
 *
 * usage:
 *   --out <path>      where to write the container (default <file>.egg2)
 *   --check           carry codegg-v1 residues per chunk (+3 B each)
 *   --div <n>         DIV search limit for B (default 64; 0 disables)
 *   --unpack          treat <file> as a container: decode, verify, report
 *   --no-write        measure only, write nothing
 *   --map             print the chunk strip (who claimed what, in order)
 *
 * The output is a ledger, not a boast. Every power's take is listed against
 * what LITERAL would have cost, gzip runs on the same bytes as the reference,
 * and when the container comes out LARGER than the input the first line of
 * the report says so. Encoding always round-trips in memory before a single
 * byte is written; a container that cannot reproduce its input is refused. */
const fs = require("fs"), path = require("path"), zlib = require("zlib");
const E = require(path.join(__dirname, "..", "eggcode.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["out", "div"]);
const flag = (n, d) => { const i = argv.indexOf("--" + n);
  return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const has = n => argv.indexOf("--" + n) >= 0;
let file = null;
for(let i = 0; i < argv.length; i++){
  const a = argv[i];
  if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; }
  file = a; break;
}
if(!file){ console.error("usage: eggpack.js <file> [--check] [--div n] [--map] [--unpack]"); process.exit(1); }

const src = fs.readFileSync(file);

/* ---- unpack mode ---- */
if(has("unpack")){
  const t0 = Date.now();
  const d = E.decode(src);
  console.log(`${file}: ${src.length} B container -> ${d.bytes.length} B in ${Date.now() - t0} ms`);
  if(d.check) console.log(`  residues: ${d.verified} chunks verified, ${d.failed} FAILED`);
  const out = flag("out", file.replace(/\.egg2$/, "") + ".out");
  if(!has("no-write")){ fs.writeFileSync(out, d.bytes); console.log(`  wrote ${out}`); }
  process.exit(d.failed ? 1 : 0);
}

/* ---- pack ---- */
const t0 = Date.now();
const r = E.encode(src, {check: has("check"), divLimit: +flag("div", 64)});
const tEnc = Date.now() - t0;

/* round-trip before anything touches disk */
const back = E.decode(r.packed);
if(!Buffer.from(back.bytes).equals(src)){
  console.error("REFUSED: container does not reproduce its input"); process.exit(2);
}

const gz = zlib.gzipSync(src, {level: 9});
const ratio = r.packed.length / src.length;

console.log(`${file}: ${src.length} B -> ${r.packed.length} B  (${(100 * ratio).toFixed(2)}% of input, `
  + `${ratio > 1 ? "LARGER by " + (r.packed.length - src.length) + " B" : "saved " + (src.length - r.packed.length) + " B"})`
  + `  in ${tEnc} ms, round-trip verified`);
console.log(`  gzip -9 same bytes: ${gz.length} B (${(100 * gz.length / src.length).toFixed(2)}%) -- the reference\n`);

console.log(`  power     chunks   covered      spent       net`);
console.log(`  ${"-".repeat(52)}`);
let covered = 0, spent = 0;
for(const name of E.OPNAME){
  const s = r.stats[name];
  if(!s || !s.chunks) continue;
  covered += s.inBytes; spent += s.outBytes;
  const net = s.inBytes - s.outBytes;
  console.log(`  ${name.padEnd(9)} ${String(s.chunks).padStart(6)} ${String(s.inBytes).padStart(9)}`
    + ` ${String(s.outBytes).padStart(10)} ${String(net >= 0 ? "-" + net : "+" + (-net)).padStart(9)}`
    + `${name === "literal" || name === "tail" ? "   (the floor)" : ""}`);
}
console.log(`  ${"-".repeat(52)}`);
console.log(`  ${"total".padEnd(9)} ${String(r.map.length).padStart(6)} ${String(covered).padStart(9)}`
  + ` ${String(spent).padStart(10)} ${String(spent + 10 <= covered ? "-" + (covered - spent - 10) : "+" + (spent + 10 - covered)).padStart(9)}   (incl. 10 B header)`);

if(has("map")){
  const CH = {0: "·", 1: "0", 2: "=", 3: "~", 4: "s", 5: "÷", 6: "t"};
  console.log(`\n  chunk map  (· literal, 0 green, = prev, ~ bar, s naf, ÷ div, t tail)`);
  let line = "";
  for(let i = 0; i < r.map.length; i++){
    line += CH[r.map[i]] || "?";
    if(line.length === 96 || i === r.map.length - 1){ console.log("  " + line); line = ""; }
  }
}

const out = flag("out", file + ".egg2");
if(!has("no-write")){ fs.writeFileSync(out, r.packed); console.log(`\n  wrote ${out}`); }
