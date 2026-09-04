/* node codegg-v5/tools/versus5.js [file] -- the encoder scoreboard.
 *
 * The size race was run in v2 and lost on purpose; this is the other race.
 * Every format gets the same file and the same two injuries -- one flipped
 * byte, and one contiguous 4096-byte scratch -- and the question is the
 * encoder's question: DO YOU GET THE FILE BACK?
 *
 * Compressors answer with their CRCs: they can tell you the file died. egg5
 * answers with repairs. Raw answers with silence. All three answers are
 * printed; none is the wrong question's answer dressed up. */
const fs = require("fs"), zlib = require("zlib"), cp = require("child_process");
const SP = process.env.EGG_TMP || ".";
const ZSTD = process.env.ZSTD || "zstd";
const file = process.argv[2] || "codegg-v4/real-test.db";
const src = fs.readFileSync(file);
const EGG = "codegg-v6/target/release/eggv6" + (process.platform === "win32" ? ".exe" : "");

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

/* build every artifact */
const T = {};
const time = (k, fn) => { const t0 = Date.now(); const r = fn(); T[k] = Date.now() - t0; return r; };
const art = {};
art.raw    = Buffer.from(src); T.raw = 0;
art.gzip   = time("gzip",   () => zlib.gzipSync(src, {level: 9}));
art.brotli = time("brotli", () => zlib.brotliCompressSync(src,
  {params: {[zlib.constants.BROTLI_PARAM_QUALITY]: src.length > 8e6 ? 9 : 11}}));
time("xz",   () => { cp.execSync(`xz -9 -k -f "${file}"`); });
art.xz = fs.readFileSync(file + ".xz"); fs.unlinkSync(file + ".xz");
time("zstd", () => { cp.execSync(`"${ZSTD}" -19 -k -f -q "${file}" -o "${SP}/v.zst"`); });
art.zstd = fs.readFileSync(`${SP}/v.zst`); fs.unlinkSync(`${SP}/v.zst`);
time("egg5", () => { cp.execSync(`"${EGG}" encode "${file}" -o "${SP}/v.egg5"`, {stdio: "pipe"}); });
art.egg5 = fs.readFileSync(`${SP}/v.egg5`);
/* the hybrid the first scoreboard omitted: armor the compressed artifact */
fs.writeFileSync(`${SP}/vh.zst`, art.zstd);
time("egg5+zstd", () => { cp.execSync(`"${EGG}" encode "${SP}/vh.zst" -o "${SP}/vh.egg5"`, {stdio: "pipe"}); });
T["egg5+zstd"] += T.zstd;
art["egg5+zstd"] = fs.readFileSync(`${SP}/vh.egg5`);

/* injure a copy: returns damaged buffer and the wound range */
function injure(buf, kind){
  const b = Buffer.from(buf);
  const g = mul32(0xACE);
  if(kind === "flip"){ const at = Math.floor(b.length / 2); b[at] ^= 0x40; return [b, at, 1]; }
  const at = Math.floor(b.length / 2) - 2048;
  for(let i = at; i < at + 4096 && i < b.length; i++) b[i] = g() & 0xff;
  return [b, at, 4096];
}

/* can each format give the file back? */
function attempt(name, damaged, at, len){
  try {
    let out;
    if(name === "raw") out = damaged;
    else if(name === "gzip") out = zlib.gunzipSync(damaged);
    else if(name === "brotli") out = zlib.brotliDecompressSync(damaged);
    else if(name === "xz"){
      fs.writeFileSync(`${SP}/v.xz`, damaged);
      cp.execSync(`xz -d -f "${SP}/v.xz"`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/v`); fs.unlinkSync(`${SP}/v`);
    }
    else if(name === "zstd"){
      fs.writeFileSync(`${SP}/v2.zst`, damaged);
      cp.execSync(`"${ZSTD}" -d -f -q "${SP}/v2.zst" -o "${SP}/v2"`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/v2`); fs.unlinkSync(`${SP}/v2`);
    }
    else if(name === "egg5+zstd"){
      fs.writeFileSync(`${SP}/vhd.egg5`, damaged);
      const wound = len > 1 ? ` --wound ${at}:${len}` : "";
      cp.execSync(`"${EGG}" decode "${SP}/vhd.egg5" -o "${SP}/vhd.zst"${wound}`, {stdio: "pipe"});
      cp.execSync(`"${ZSTD}" -d -f -q "${SP}/vhd.zst" -o "${SP}/vhd"`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/vhd`);
    }
    else if(name === "egg5"){
      fs.writeFileSync(`${SP}/vd.egg5`, damaged);
      /* blind for the flip; location known for the scratch, which is the
         honest real-world model: torn sectors come with addresses */
      const wound = len > 1 ? ` --wound ${at}:${len}` : "";
      cp.execSync(`"${EGG}" decode "${SP}/vd.egg5" -o "${SP}/vd.out"${wound}`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/vd.out`);
    }
    if(out.length !== src.length) return `WRONG SIZE (${out.length})`;
    let diff = 0; for(let i = 0; i < out.length; i++) if(out[i] !== src[i]) diff++;
    return diff === 0 ? "EXACT" : `${diff.toLocaleString()} bytes wrong`;
  } catch(e){ return "DEAD (refuses/throws)"; }
}

console.log(`${file}  ${src.length.toLocaleString()} B -- the encoder scoreboard\n`);
console.log("format   size            of orig   enc-ms   1 byte flipped        4096 B scratch");
console.log("-".repeat(96));
for(const name of ["raw", "gzip", "zstd", "brotli", "xz", "egg5", "egg5+zstd"]){
  const a = art[name];
  const [d1, a1, l1] = injure(a, "flip");
  const [d2, a2, l2] = injure(a, "scratch");
  console.log(name.padEnd(9)
    + a.length.toLocaleString().padStart(11)
    + ((100 * a.length / src.length).toFixed(1) + "%").padStart(9)
    + String(T[name] ?? 0).padStart(8)
    + "   " + attempt(name, d1, a1, l1).padEnd(22)
    + attempt(name, d2, a2, l2));
}
console.log("\n(egg5 scratch repair uses the wound's address -- torn sectors come with");
console.log(" addresses; its 1-byte repair is blind. All numbers from this run.)");
