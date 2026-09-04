/* node codegg-v12/tools/standings.js <file>... -- the tournament, v12 row added (ancestor rows kept).
 *
 * The rule is Vladimir's, stated 2026-08-31 and enforced mechanically here:
 * a tool that returns wrong data or no data after damage CANNOT claim a win --
 * lossless means lossless after the world has touched the file, not only in
 * the demo. Among the tools that hand back the EXACT file under every injury,
 * the smallest artifact wins.
 *
 * Three injuries per artifact, same for everyone:
 *   flip    one byte flipped in the middle (blind)
 *   scratch one contiguous 4096-byte overwrite in the middle (addressed)
 *   trunc   4096 bytes REMOVED from the end
 *
 * Rows: the four compressors; eggv6 (armor only); the eggv6+zstd hybrid (the
 * old recommended posture); eggv7 (the prior transmuter); eggv8 (the Squeeze).
 *
 * v8's victory ledger holds THREE bars at once (the stretch, Vladimir's
 * choice 2026-09-01), each against the STRONGER implementation where two
 * exist (the zlib-vs-CLI-gzip lesson of v7 -- gz* = the smaller of the two):
 *   (i)   lighter than strongest gzip -9 on >=10 of 12 real files
 *   (ii)  lighter than the egg6+zstd hybrid artifact on >=6 of 12
 *   (iii) lighter than xz -9 on >=3 of 12
 * all with armor ON and every injury restored EXACT. */
const fs = require("fs"), zlib = require("zlib"), cp = require("child_process"), path = require("path");
const here = path.dirname(__filename);
const os = require("os");
/* scratch files live OUTSIDE the corpus: a leftover a.xz in the corpus dir
 * once crashed the synthetic tournament by joining the glob */
const SP = process.env.EGG_TMP || fs.mkdtempSync(path.join(os.tmpdir(), "egg10sp-"));
const ZSTD = process.env.ZSTD ||
  "C:\\Users\\vcepe\\AppData\\Local\\Microsoft\\WinGet\\Packages\\Meta.Zstandard_Microsoft.Winget.Source_8wekyb3d8bbwe\\zstd-v1.5.7-win64\\zstd.exe";
const EGG6 = path.join(here, "..", "..", "codegg-v6", "target", "release", "eggv6" + (process.platform === "win32" ? ".exe" : ""));
const EGG7 = path.join(here, "..", "..", "codegg-v7", "target", "release", "eggv7" + (process.platform === "win32" ? ".exe" : ""));
const EGG8 = path.join(here, "..", "..", "codegg-v8", "target", "release", "eggv8" + (process.platform === "win32" ? ".exe" : ""));
const EGG9 = path.join(here, "..", "..", "codegg-v9", "target", "release", "eggv9" + (process.platform === "win32" ? ".exe" : ""));
const EGG10 = path.join(here, "..", "..", "codegg-v10", "target", "release", "eggv10" + (process.platform === "win32" ? ".exe" : ""));
const EGG11 = path.join(here, "..", "..", "codegg-v11", "target", "release", "eggv11" + (process.platform === "win32" ? ".exe" : ""));
const EGG12 = path.join(here, "..", "..", "codegg-v12", "target", "release", "eggv12" + (process.platform === "win32" ? ".exe" : ""));
const EGG14 = path.join(here, "..", "target", "release", "eggv14" + (process.platform === "win32" ? ".exe" : ""));

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

/* v6 rib policy, unchanged from its own tournament */
const egg6Flags = n => n < 65536 ? " --group 8 --parity 2"
                   : n < 1048576 ? " --group 32 --parity 2"
                   : " --group 126 --parity 2";

function buildArtifacts(file, src){
  const art = {};
  art.gzip = zlib.gzipSync(src, {level: 9});
  /* gz* = the STRONGER gzip: CLI gzip -9 sometimes beats zlib and vice versa;
   * the bar is held against whichever is smaller (v7's xml lesson). The
   * tournament row keeps zlib (deterministic injuries need one artifact);
   * gzStar is the WEIGHT the ledger judges against. */
  let gzCli = null;
  try {
    cp.execSync(`gzip -9 -c "${file}" > "${SP}/s.gz"`, {stdio: "pipe", shell: "bash.exe"});
    gzCli = fs.readFileSync(`${SP}/s.gz`);
  } catch(e){ /* CLI gzip unavailable: zlib alone judges */ }
  art._gzStar = Math.min(art.gzip.length, gzCli ? gzCli.length : Infinity);
  art.brotli = zlib.brotliCompressSync(src,
    {params: {[zlib.constants.BROTLI_PARAM_QUALITY]: src.length > 8e6 ? 9 : 11}});
  cp.execSync(`xz -9 -k -f "${file}"`);
  art.xz = fs.readFileSync(file + ".xz"); fs.unlinkSync(file + ".xz");
  cp.execSync(`"${ZSTD}" -19 -k -f -q "${file}" -o "${SP}/s.zst"`);
  art.zstd = fs.readFileSync(`${SP}/s.zst`);
  cp.execSync(`"${EGG6}" encode "${file}" -o "${SP}/s.egg5"${egg6Flags(src.length)}`, {stdio: "pipe"});
  art.egg6 = fs.readFileSync(`${SP}/s.egg5`);
  fs.writeFileSync(`${SP}/sh.zst`, art.zstd);
  cp.execSync(`"${EGG6}" encode "${SP}/sh.zst" -o "${SP}/sh.egg5"${egg6Flags(art.zstd.length)}`, {stdio: "pipe"});
  art["egg6+zstd"] = fs.readFileSync(`${SP}/sh.egg5`);
  cp.execSync(`"${EGG7}" transmute "${file}" -o "${SP}/s.egg7"`, {stdio: "pipe"});
  art.egg7 = fs.readFileSync(`${SP}/s.egg7`);
  cp.execSync(`"${EGG8}" transmute "${file}" -o "${SP}/s.egg8"`, {stdio: "pipe"});
  art.egg8 = fs.readFileSync(`${SP}/s.egg8`);
  cp.execSync(`"${EGG9}" transmute "${file}" -o "${SP}/s.egg9"`, {stdio: "pipe"});
  art.egg9 = fs.readFileSync(`${SP}/s.egg9`);
  cp.execSync(`"${EGG10}" transmute "${file}" -o "${SP}/s.egg10"`, {stdio: "pipe"});
  art.egg10 = fs.readFileSync(`${SP}/s.egg10`);
  cp.execSync(`"${EGG11}" transmute "${file}" -o "${SP}/s.egg11"`, {stdio: "pipe"});
  art.egg11 = fs.readFileSync(`${SP}/s.egg11`);
  cp.execSync(`"${EGG14}" transmute "${file}" -o "${SP}/s.egg14"`, {stdio: "pipe"});
  art.egg14 = fs.readFileSync(`${SP}/s.egg14`);
  cp.execSync(`"${EGG12}" transmute "${file}" -o "${SP}/s.egg12"`, {stdio: "pipe"});
  art.egg12 = fs.readFileSync(`${SP}/s.egg12`);
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
    else if(name === "egg7" || name === "egg8" || name === "egg9" || name === "egg10" || name === "egg11" || name === "egg12" || name === "egg13"){
      const exe = name === "egg13" ? EGG14 : name === "egg12" ? EGG12 : name === "egg11" ? EGG11 : name === "egg10" ? EGG10 : name === "egg9" ? EGG9 : name === "egg8" ? EGG8 : EGG7;
      fs.writeFileSync(`${SP}/a.${name}`, damaged);
      const wound = kind === "scratch" ? ` --wound ${at}:${len}` : "";
      cp.execSync(`"${exe}" restore "${SP}/a.${name}" -o "${SP}/a.out.${name}"${wound}`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/a.out.${name}`);
    }
    else {
      fs.writeFileSync(`${SP}/a.egg5`, damaged);
      const wound = kind === "scratch" ? ` --wound ${at}:${len}` : "";
      cp.execSync(`"${EGG6}" decode "${SP}/a.egg5" -o "${SP}/a.out"${wound}`, {stdio: "pipe"});
      out = fs.readFileSync(`${SP}/a.out`);
      if(name === "egg6+zstd"){
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
const NAMES = ["gzip", "zstd", "brotli", "xz", "egg6", "egg6+zstd", "egg7", "egg8", "egg9", "egg10", "egg11", "egg12", "egg13"];
console.log("THE TOURNAMENT -- rule: wrong-or-no data after any injury forfeits; smallest lossless survivor wins");
console.log("injuries per artifact: 1-byte flip (blind) / 4 KB scratch (addressed) / 4 KB truncation\n");
console.log("file                    orig B" + NAMES.map(n => n.padStart(11)).join("") + "   WINNER (size)");
console.log("-".repeat(180));
const tally = {};
let barGz = 0, barHybrid = 0, barXz = 0, barMin = 0, total = 0;
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
  /* the weight is kept for posterity even when the tool forfeits: how small
   * it got and whether it survived are two different facts (Vladimir,
   * 2026-09-01). LIED stays loud beside its number. */
  const cell = n => {
    const v = verdicts[n];
    const pct = (100 * v.size / src.length).toFixed(1) + "%";
    return (v.ok ? pct : v.lied ? pct + "LIED" : pct + "(dq)").padStart(11);
  };
  const short = file.split(/[\\/]/).pop();
  console.log(short.padEnd(22) + String(src.length).padStart(9)
    + NAMES.map(cell).join("") + "   " + winner + " (" + (verdicts[winner] ? verdicts[winner].size : "-") + ")");
  /* the v9 victory ledger: THE bar is xz >=8/12 (the stretch, 2026-09-01);
   * the v8 bars are re-verified for the record */
  total++;
  if(verdicts.egg14.ok){
    if(verdicts.egg14.size < art._gzStar) barGz++;
    if(verdicts.egg14.size < verdicts["egg6+zstd"].size) barHybrid++;
    if(verdicts.egg14.size < verdicts.xz.size) barXz++;
    if(verdicts.egg14.size <= Math.min(verdicts.egg8.size, verdicts.egg9.size, verdicts.egg10.size, verdicts.egg11.size, verdicts.egg12.size)) barMin++;
  }
}
console.log("-".repeat(180));
console.log("podium: " + Object.entries(tally).sort((a, b) => b[1] - a[1])
  .map(([n, c]) => `${n} x${c}`).join(", "));
console.log(`victory ledger (egg13, armor ON, all injuries EXACT, of ${total} files):`);
console.log(`  vs naked xz -9               : ${barXz}/${total}   (v12: an EXHIBIT, not a bar -- armored vs naked)`);
console.log(`  vs strongest gzip -9         : ${barGz}/${total}`);
console.log(`  vs egg6+zstd hybrid          : ${barHybrid}/${total}`);
console.log(`  <= min(egg8,egg9,egg10)      : ${barMin}/${total}  (v12 ratchet: ALL rows)`);
