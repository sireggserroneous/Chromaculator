/* node tools/challengers.js <file>... -- the challengers card (2026-09-01).
 *
 * Two shield-carrying outsiders take the SAME three injuries as the main
 * tournament (flip / 4 KB scratch / 4 KB trunc, same PRNG, same placement):
 *
 *   rar      WinRAR 7 CLI: -m5 -s -rr5% -- solid, max compression, 5%
 *            recovery record (the common archival posture). Measured facts
 *            from the smoke round: survives flip; its record is parity-thin
 *            for a 4 KB scratch on small archives, and it lives at the
 *            archive TAIL, so truncation eats medicine with the wound.
 *   xz+par2  xz -9 payload + a Parchive 2.0 recovery set (MultiPar par2j64),
 *            sized to the injury contract: slices small enough that a 4 KB
 *            scratch spans few, recovery blocks enough to rebuild them.
 *            The artifact is the concatenation; weights are the sum.
 *
 * Runs files CONCURRENTLY (Vladimir's highway, 2026-09-01: 24 lanes, not a
 * bus lane) -- results are byte-identical to a serial run; only the clock
 * changes. Every weight prints beside its verdict, wins and forfeits alike. */
const fs = require("fs"), cp = require("child_process"), path = require("path"), os = require("os");

const RAR = "C:\\Program Files\\WinRAR\\Rar.exe";
const PAR2 = process.env.USERPROFILE + "\\AppData\\Local\\MultiPar\\par2j64.exe";
const LANES = Math.max(2, Math.min(12, os.cpus().length - 4));

function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }

function injure(buf, kind){
  const b = Buffer.from(buf);
  const g = mul32(0xACE);
  if(kind === "flip"){ b[Math.floor(b.length / 2)] ^= 0x40; return b; }
  if(kind === "trunc") return b.subarray(0, Math.max(0, b.length - 4096));
  const at = Math.max(0, Math.floor(b.length / 2) - 2048);
  for(let i = at; i < at + 4096 && i < b.length; i++) b[i] = g() & 0xff;
  return b;
}

function sh(cmd, opts){ return cp.execSync(cmd, {stdio: "pipe", ...opts}); }

/* ---- rar: build once, then repair+extract per injury ---- */
function rarBuild(file, dir){
  const rar = path.join(dir, "a.rar");
  sh(`"${RAR}" a -ep -m5 -s -rr5% -idq "${rar}" "${file}"`);
  return fs.readFileSync(rar);
}
function rarAttempt(art, kind, src, base, dir){
  const wd = path.join(dir, "rar-" + kind);
  fs.mkdirSync(wd, {recursive: true});
  const dmg = path.join(wd, "d.rar");
  fs.writeFileSync(dmg, injure(art, kind));
  try {
    try { sh(`"${RAR}" r -idq "${dmg}"`, {cwd: wd}); } catch(e){ /* repair may "fail" yet emit a fixed archive */ }
    const fixed = ["fixed.d.rar", "rebuilt.d.rar", "d.rar"]
      .map(n => path.join(wd, n)).find(p => fs.existsSync(p));
    sh(`"${RAR}" x -idq -y "${fixed}" "${wd}${path.sep}"`);
    const out = fs.readFileSync(path.join(wd, base));
    return out.equals(src) ? "EXACT" : "WRONG";
  } catch(e){ return "dead"; }
}

/* ---- xz+par2: concat artifact [xz | par2 index | par2 vols] ---- */
function par2Params(xzLen){
  // a 4 KB contiguous scratch must be rebuildable: slice size s spans
  // ceil(4096/s)+1 slices; recovery blocks cover that with margin x2
  if(xzLen < 4 * 1024 * 1024) return {ss: 1024, rn: 12};
  return {ss: 4096, rn: 8};
}
function xpBuild(file, dir){
  const base = path.basename(file);
  const xzp = path.join(dir, base + ".xz");
  sh(`xz -9 -k -c "${file}" > "${xzp}"`, {shell: "bash.exe"});
  const xz = fs.readFileSync(xzp);
  const {ss, rn} = par2Params(xz.length);
  sh(`"${PAR2}" c /ss${ss} /rn${rn} /rf1 "${path.join(dir, "p.par2")}" "${base}.xz"`, {cwd: dir});
  const idx = fs.readFileSync(path.join(dir, "p.par2"));
  const volName = fs.readdirSync(dir).find(n => /^p\.vol.*\.par2$/.test(n));
  const vol = fs.readFileSync(path.join(dir, volName));
  return {art: Buffer.concat([xz, idx, vol]), lens: [xz.length, idx.length, vol.length], volName};
}
function xpAttempt(built, kind, src, base, dir){
  const wd = path.join(dir, "xp-" + kind);
  fs.mkdirSync(wd, {recursive: true});
  const d = injure(built.art, kind);
  const [n1, n2] = built.lens;
  try {
    fs.writeFileSync(path.join(wd, base + ".xz"), d.subarray(0, Math.min(n1, d.length)));
    fs.writeFileSync(path.join(wd, "p.par2"), d.subarray(Math.min(n1, d.length), Math.min(n1 + n2, d.length)));
    fs.writeFileSync(path.join(wd, built.volName), d.subarray(Math.min(n1 + n2, d.length)));
    try { sh(`"${PAR2}" r "p.par2"`, {cwd: wd}); } catch(e){ /* verify-only exit codes vary */ }
    sh(`xz -d -k -f "${path.join(wd, base + ".xz")}"`, {shell: "bash.exe"});
    const out = fs.readFileSync(path.join(wd, base));
    return out.equals(src) ? "EXACT" : "WRONG";
  } catch(e){ return "dead"; }
}

async function runFile(file){
  const src = fs.readFileSync(file);
  const base = path.basename(file);
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "chal-"));
  const row = {file: base, orig: src.length};
  try {
    const rart = rarBuild(file, dir);
    row.rar = {size: rart.length, verdicts: {}};
    for(const k of ["flip", "scratch", "trunc"]) row.rar.verdicts[k] = rarAttempt(rart, k, src, base, dir);
  } catch(e){ row.rar = {size: 0, verdicts: {flip: "dead", scratch: "dead", trunc: "dead"}}; }
  try {
    const built = xpBuild(file, dir);
    row.xp = {size: built.art.length, verdicts: {}};
    for(const k of ["flip", "scratch", "trunc"]) row.xp.verdicts[k] = xpAttempt(built, k, src, base, dir);
  } catch(e){ row.xp = {size: 0, verdicts: {flip: "dead", scratch: "dead", trunc: "dead"}}; }
  fs.rmSync(dir, {recursive: true, force: true});
  return row;
}

async function pool(items, n, fn){
  const out = new Array(items.length); let i = 0;
  await Promise.all(Array.from({length: Math.min(n, items.length)}, async () => {
    while(i < items.length){ const k = i++; out[k] = await fn(items[k]); }
  }));
  return out;
}

(async () => {
  const files = process.argv.slice(2);
  console.log(`THE CHALLENGERS CARD -- same injuries, same rule: every wound EXACT or forfeit. ${LANES} lanes.`);
  console.log("file                    orig B          rar   f/s/t        xz+par2   f/s/t");
  console.log("-".repeat(90));
  const rows = await pool(files, LANES, runFile);
  const mark = v => v === "EXACT" ? "E" : v === "WRONG" ? "W" : "x";
  for(const r of rows){
    const cell = t => {
      const pct = (100 * t.size / r.orig).toFixed(1) + "%";
      const ok = Object.values(t.verdicts).every(v => v === "EXACT");
      const ms = ["flip", "scratch", "trunc"].map(k => mark(t.verdicts[k])).join("/");
      return (ok ? pct : pct + "(dq)").padStart(13) + "   " + ms.padEnd(8);
    };
    console.log(r.file.padEnd(22) + String(r.orig).padStart(9) + cell(r.rar) + cell(r.xp));
  }
  console.log("-".repeat(90));
  console.log("E=EXACT restore, x=failed, W=wrong bytes; (dq) = forfeited at least one injury");
})();
