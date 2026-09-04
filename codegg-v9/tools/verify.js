/* node codegg-v9/tools/verify.js <file>... -- the independent audit.
 *
 * "Should we run a hash on them also? To make sure we are not cheating?"
 * Yes. The tournament's check was a full byte compare (stronger than any
 * hash); what this adds is INDEPENDENCE: every restored file is written to
 * disk and fingerprinted by certutil -- the operating system's own SHA-256,
 * no code of ours in the verdict path. For each file: hash the original,
 * transmute it (armor ON, default ribs), injure the artifact three ways,
 * restore each to disk, hash each restoration, print fingerprints side by
 * side. The conservation law, countersigned by the OS. */
const fs = require("fs"), cp = require("child_process"), path = require("path");
const here = path.dirname(__filename);
const os = require("os");
/* scratch files live OUTSIDE the corpus: a leftover a.xz in the corpus dir
 * once crashed the synthetic tournament by joining the glob */
const SP = process.env.EGG_TMP || fs.mkdtempSync(path.join(os.tmpdir(), "egg9sp-"));
const EGG9 = path.join(here, "..", "target", "release", "eggv9" + (process.platform === "win32" ? ".exe" : ""));

function sha256(p){
  const out = cp.execSync(`certutil -hashfile "${p}" SHA256`).toString();
  return out.split(/\r?\n/)[1].trim().toLowerCase();
}
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

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
  cp.execSync(`"${EGG9}" transmute "${file}" -o "${SP}/w.egg9"`, {stdio: "pipe"});
  const art = fs.readFileSync(`${SP}/w.egg9`);
  const short = file.split(/[\\/]/).pop();
  console.log(`${short}  (egg9, ${(100 * art.length / src.length).toFixed(1)}% of original)`);
  console.log(`  original   ${origHash}`);
  for(const kind of ["flip", "scratch", "trunc"]){
    const [d, at, len] = injure(art, kind);
    fs.writeFileSync(`${SP}/w.hurt`, d);
    const wound = kind === "scratch" ? ` --wound ${at}:${len}` : "";
    const rec = `${SP}/rec-${short}-${kind}`;
    try {
      cp.execSync(`"${EGG9}" restore "${SP}/w.hurt" -o "${rec}"${wound}`, {stdio: "pipe"});
      const h = sha256(rec);
      const ok = h === origHash;
      if(!ok) all = false;
      console.log(`  ${kind.padEnd(8)}   ${h}  ${ok ? "MATCH" : "*** MISMATCH ***"}`);
      fs.rmSync(rec);
    } catch(e){ all = false; console.log(`  ${kind.padEnd(8)}   RESTORE FAILED`); }
  }
  console.log("");
}
console.log(all ? "ALL FINGERPRINTS MATCH -- verified by certutil, not by us."
                : "*** AT LEAST ONE MISMATCH -- the claim does not stand ***");
process.exit(all ? 0 : 1);
