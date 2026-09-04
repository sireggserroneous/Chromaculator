/* the M9 big-arena countersign, in lanes: eggv12 transmute + restore +
 * certutil SHA-256 fingerprint compare on every big member. The ancestor and
 * raw-compressor rows of the big table are SPLICED from the 2026-09-01
 * official run (deterministic; those binaries are frozen) -- only the egg12
 * column is new, and this signs it. */
const fs = require('fs'), path = require('path'), os = require('os');
const { execFile, execFileSync } = require('child_process');
const root = path.join(path.dirname(__filename), '..');
const EXE = path.join(root, 'target', 'release', 'eggv12.exe');
const LANES = Math.max(2, Math.min(6, os.cpus().length - 4));
function run(exe, args){ return new Promise((res, rej) => execFile(exe, args, {maxBuffer: 1 << 28}, e => e ? rej(e) : res())); }
function sha(p){
  const out = execFileSync('certutil', ['-hashfile', p, 'SHA256']).toString();
  return out.split(/\r?\n/)[1].trim();
}
async function doFile(f){
  const src = path.join(root, 'corpus-big', f);
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'cs12-'));
  const art = path.join(tmp, f + '.egg12'), out = path.join(tmp, f + '.out');
  try {
    await run(EXE, ['transmute', src, '-o', art]);
    await run(EXE, ['restore', art, '-o', out]);
    const ok = sha(src) === sha(out);
    const n = fs.statSync(art).size;
    return `${f.padEnd(22)} ${String(n).padStart(9)} B  ${ok ? 'FINGERPRINT MATCH' : 'MISMATCH'}`;
  } catch(e){ return `${f}: FAILED ${String(e).slice(0, 60)}`; }
  finally { fs.rmSync(tmp, {recursive: true, force: true}); }
}
(async () => {
  const files = fs.readdirSync(path.join(root, 'corpus-big')).sort();
  files.sort((a, b) => fs.statSync(path.join(root, 'corpus-big', b)).size - fs.statSync(path.join(root, 'corpus-big', a)).size);
  const out = new Array(files.length); let i = 0;
  await Promise.all(Array.from({length: Math.min(LANES, files.length)}, async () => {
    while(i < files.length){ const k = i++; out[k] = await doFile(files[k]); }
  }));
  for(const line of out.sort()) console.log(line);
})();
