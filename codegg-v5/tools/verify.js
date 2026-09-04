/* node codegg-v5/tools/verify.js <file>... -- the independent audit.
 *
 * "Should we run a hash on them also? To make sure we are not cheating?"
 * Yes. The tournament's check was a full byte compare (stronger than any
 * hash); what this adds is INDEPENDENCE: every recovered file is written to
 * disk and fingerprinted by certutil -- the operating system's own SHA-256,
 * no code of ours in the verdict path. For each file: hash the original,
 * then injure the winner's artifact three ways, recover each to disk, hash
 * each recovery, and print the fingerprints side by side. */
const fs = require("fs"), zlib = require("zlib"), cp = require("child_process");
const SP = process.env.EGG_TMP || ".";
const ZSTD = process.env.ZSTD || "zstd";
const EGG = "codegg-v5/target/release/eggv5" + (process.platform === "win32" ? ".exe" : "");

function sha256(path){
  const out = cp.execSync(`certutil -hashfile "${path}" SHA256`).toString();
  return out.split(/\r?\n/)[1].trim().toLowerCase();
}
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
const eggFlags = n => n < 1048576 ? " --group 8 --parity 2" : "";

function injure(buf, kind){
  const b = Buffer.from(buf);
  const g = mul32(0xACE);
  if(kind === "flip"){ const at = Math.floor(b.length / 2); b[at] ^= 0x40; return [b, at, 1]; }
  if(kind === "trunc") return [b.subarray(0, Math.max(0, b.length - 4096)), b.length - 4096, 4096];
  const at = Math.max(0, Math.floor(b.length / 2) - 2048);
  for(let i = at; i < at + 4096 && i < b.length; i++) b[i] = g() & 0xff;
  return [b, at, 4096];
}

let all = true;
for(const file of process.argv.slice(2)){
  const src = fs.readFileSync(file);
  const origHash = sha256(file);
  /* the winner's pipeline: hybrid when zstd helps, plain egg5 when it cannot */
  cp.execSync(`"${ZSTD}" -19 -k -f -q "${file}" -o "${SP}/w.zst"`);
  const z = fs.readFileSync(`${SP}/w.zst`);
  const hybrid = z.length < src.length * 0.95;
  const inner = hybrid ? `${SP}/w.zst` : file;
  const innerLen = hybrid ? z.length : src.length;
  cp.execSync(`"${EGG}" encode "${inner}" -o "${SP}/w.egg5"${eggFlags(innerLen)}`, {stdio: "pipe"});
  const art = fs.readFileSync(`${SP}/w.egg5`);
  const short = file.split(/[\\/]/).pop();
  console.log(`${short}  (${hybrid ? "egg5+zstd" : "egg5"})`);
  console.log(`  original   ${origHash}`);
  for(const kind of ["flip", "scratch", "trunc"]){
    const [d, at, len] = injure(art, kind);
    fs.writeFileSync(`${SP}/w.hurt`, d);
    const wound = kind === "scratch" ? ` --wound ${at}:${len}` : "";
    let rec = `${SP}/rec-${short}-${kind}`;
    try {
      cp.execSync(`"${EGG}" decode "${SP}/w.hurt" -o "${SP}/w.dec"${wound}`, {stdio: "pipe"});
      if(hybrid){
        cp.execSync(`"${ZSTD}" -d -f -q "${SP}/w.dec" -o "${rec}"`, {stdio: "pipe"});
      } else fs.copyFileSync(`${SP}/w.dec`, rec);
      const h = sha256(rec);
      const ok = h === origHash;
      if(!ok) all = false;
      console.log(`  ${kind.padEnd(8)}   ${h}  ${ok ? "MATCH" : "*** MISMATCH ***"}`);
    } catch(e){ all = false; console.log(`  ${kind.padEnd(8)}   RECOVERY FAILED`); }
  }
  console.log("");
}
console.log(all ? "ALL FINGERPRINTS MATCH -- verified by certutil, not by us."
                : "*** AT LEAST ONE MISMATCH -- the claim does not stand ***");
process.exit(all ? 0 : 1);
