/* node codegg-v5/tools/standings.js <file>... -- the tournament.
 *
 * The rule is Vladimir's, stated 2026-08-31 and enforced mechanically here:
 * a tool that returns wrong data or no data after damage CANNOT claim a win --
 * we are only working with lossless tools, and lossless means lossless after
 * the world has touched the file, not only in the demo. Among the tools that
 * hand back the EXACT file under every injury, the smallest artifact wins.
 *
 * Three injuries per artifact, same for everyone:
 *   flip   one byte flipped in the middle
 *   scratch one contiguous 4096-byte overwrite in the middle
 *   trunc  4096 bytes REMOVED from the end
 *
 * Sizing policy printed on the label: artifacts under 1 MB are armored with
 * --group 8 (capacity ~ file/4), larger with the default --group 32 --parity 2
 * (capacity ~ file/16) -- size the ribs to the suitcase. */
const fs = require("fs"), zlib = require("zlib"), cp = require("child_process");
const SP = process.env.EGG_TMP || ".";
const ZSTD = process.env.ZSTD || "zstd";
const EGG = "codegg-v6/target/release/eggv6" + (process.platform === "win32" ? ".exe" : "");

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

/* ribs sized to the wound: capacity = (T/G) x artifact must cover a 4 KB
   scratch, so G <= artifact/2048 at T=2 -- tiny files get dense ribs, big
   files get the slim profile */
const eggFlags = n => n < 65536 ? " --group 8 --parity 2"
                  : n < 1048576 ? " --group 32 --parity 2"
                  : (process.env.EGG_FLAGS ? " "+process.env.EGG_FLAGS : "");

function buildArtifacts(file, src){
  const art = {};
  art.gzip = zlib.gzipSync(src, {level: 9});
  art.brotli = zlib.brotliCompressSync(src,
    {params: {[zlib.constants.BROTLI_PARAM_QUALITY]: src.length > 8e6 ? 9 : 11}});
  cp.execSync(`xz -9 -k -f "${file}"`);
  art.xz = fs.readFileSync(file + ".xz"); fs.unlinkSync(file + ".xz");
  cp.execSync(`"${ZSTD}" -19 -k -f -q "${file}" -o "${SP}/s.zst"`);
  art.zstd = fs.readFileSync(`${SP}/s.zst`);
  cp.execSync(`"${EGG}" encode "${file}" -o "${SP}/s.egg5"${eggFlags(src.length)}`, {stdio: "pipe"});
  art.egg5 = fs.readFileSync(`${SP}/s.egg5`);
  fs.writeFileSync(`${SP}/sh.zst`, art.zstd);
  cp.execSync(`"${EGG}" encode "${SP}/sh.zst" -o "${SP}/sh.egg5"${eggFlags(art.zstd.length)}`, {stdio: "pipe"});
  art["egg5+zstd"] = fs.readFileSync(`${SP}/sh.egg5`);
  return art;
}

function injure(buf, kind){
  const b = Buffer.from(buf);
  const g = mul32(0xACE);
  if(kind === "flip"){ const at = Math.floor(b.length / 2); b[at] ^= 0x40; return [b, at, 1]; }
  if(kind === "trunc") return [b.subarray(0, Math.max(0, b.length - 4096)), b.length - 4096, 4096];
  const at = Math.max(0, Math.floor(b.length / 2) - 2048);
  for(let i = at; i < at + 4096 && i < b.length; i++) b[i] = g() & 0xff;
  return [b, at, 4096];
}

function attempt(name, damaged, at, len, kind, src){
  try {
    let out;
    if(name === "gzip") out = zlib.gunzipSync(damaged);
    else if(name === "brotli") out = zlib.brotliDecompressSync(damaged);
    else if(name === "xz"){
      fs.writeFileSync(`${SP}/a.xz`, damaged);
      cp.execSync(`xz -d -f "${SP}/a.xz"`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/a`); fs.unlinkSync(`${SP}/a`);
    }
    else if(name === "zstd"){
      fs.writeFileSync(`${SP}/a.zst`, damaged);
      cp.execSync(`"${ZSTD}" -d -f -q "${SP}/a.zst" -o "${SP}/a2"`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/a2`);
    }
    else {
      fs.writeFileSync(`${SP}/a.egg5`, damaged);
      /* known location for overwrites (torn sectors have addresses); blind
         for the flip; truncation needs no hint -- the decoder pads-as-wound */
      const wound = kind === "scratch" ? ` --wound ${at}:${len}` : "";
      cp.execSync(`"${EGG}" decode "${SP}/a.egg5" -o "${SP}/a.out"${wound}`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/a.out`);
      if(name === "egg5+zstd"){
        fs.writeFileSync(`${SP}/a2.zst`, out);
        cp.execSync(`"${ZSTD}" -d -f -q "${SP}/a2.zst" -o "${SP}/a3"`, {stdio: "pipe"});
        out = fs.readFileSync(`${SP}/a3`);
      }
    }
    if(out.length !== src.length) return "loss";
    return out.equals(src) ? "EXACT" : "WRONG";   // WRONG = returned bad data = disqualified loudly
  } catch(e){ return "dead"; }
}

const files = process.argv.slice(2);
const NAMES = ["gzip", "zstd", "brotli", "xz", "egg5", "egg5+zstd"];
console.log("THE TOURNAMENT -- rule: wrong-or-no data after any injury forfeits; smallest lossless survivor wins");
console.log("injuries per artifact: 1-byte flip (blind) / 4 KB scratch (addressed) / 4 KB truncation\n");
console.log("file                    orig B      gzip    zstd  brotli      xz    egg5  e5+zstd   WINNER (size)");
console.log("-".repeat(112));
const tally = {};
for(const file of files){
  const src = fs.readFileSync(file);
  const art = buildArtifacts(file, src);
  const verdicts = {};
  for(const n of NAMES){
    let ok = true, lied = false;
    for(const kind of ["flip", "scratch", "trunc"]){
      const [d, at, len] = injure(art[n], kind);
      const r = attempt(n, d, at, len, kind, src);
      if(r !== "EXACT") ok = false;
      if(r === "WRONG") lied = true;
    }
    verdicts[n] = {ok, lied, size: art[n].length};
  }
  const alive = NAMES.filter(n => verdicts[n].ok);
  const winner = alive.sort((a, b) => verdicts[a].size - verdicts[b].size)[0] || "none";
  tally[winner] = (tally[winner] || 0) + 1;
  const cell = n => {
    const v = verdicts[n];
    const pct = (100 * v.size / src.length).toFixed(0) + "%";
    return (v.ok ? pct : v.lied ? "LIED" : "dq").padStart(7);
  };
  const short = file.split(/[\\/]/).pop();
  console.log(short.padEnd(22)
    + src.length.toLocaleString().padStart(11)
    + NAMES.map(cell).join(" ")
    + "   " + winner
    + (winner !== "none" ? ` (${(100 * verdicts[winner].size / src.length).toFixed(1)}%)` : ""));
}
console.log("-".repeat(112));
console.log("podium: " + Object.entries(tally).map(([k, v]) => `${k} x${v}`).join(", "));
console.log("(dq = returned nothing after an injury; LIED = returned wrong data claiming success)");
