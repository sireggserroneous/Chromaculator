/* node codegg-v3/tools/shard.js <cmd> ... -- the file as residues, on disk.
 *
 * split <file> [--m 8] [--out dir]     write k+m shard files (.egg3-000 ...)
 * join  <dir|shards...> [--out path]   reconstruct from whatever is there
 * demo  <file> [--kill 8] [--m 8]      split, delete a random kill-set,
 *                                      join from the survivors, byte-compare
 *
 * join takes ANY k of the shards -- there is no privileged one. It reports
 * confirmed / repaired / condemned blocks and names any shard the spares
 * convicted. demo is the whole thesis in one command: the deaths are chosen
 * by lot, and the file comes back exact anyway. */
const fs = require("fs"), path = require("path");
const S = require(path.join(__dirname, "..", "eggshard.js"));

const argv = process.argv.slice(2);
const VALUED = new Set(["m", "out", "kill"]);
const flag = (n, d) => { const i = argv.indexOf("--" + n);
  return i >= 0 && argv[i + 1] !== undefined && !argv[i + 1].startsWith("--") ? argv[i + 1] : d; };
const bare = [];
for(let i = 0; i < argv.length; i++){
  const a = argv[i];
  if(a.startsWith("--")){ if(VALUED.has(a.slice(2))) i++; continue; }
  bare.push(a);
}
const cmd = bare[0];

function gatherShards(args){
  const files = [];
  for(const a of args){
    const st = fs.statSync(a);
    if(st.isDirectory())
      for(const f of fs.readdirSync(a)) if(/\.egg3-\d+$/.test(f)) files.push(path.join(a, f));
    else files.push(a);
  }
  return files;
}

if(cmd === "split"){
  const file = bare[1];
  const src = fs.readFileSync(file);
  const m = +flag("m", 8);
  const t0 = Date.now();
  const e = S.encode(src, {m});
  const dir = flag("out", file + ".shards");
  fs.mkdirSync(dir, {recursive: true});
  let total = 0;
  e.shards.forEach((s, i) => {
    fs.writeFileSync(path.join(dir, path.basename(file) + ".egg3-" + String(i).padStart(3, "0")), s);
    total += s.length;
  });
  console.log(`${file}: ${src.length} B -> ${e.shards.length} shards of ${e.shards[0].length} B`
    + ` = ${total} B (${(100 * total / src.length).toFixed(2)}%) in ${Date.now() - t0} ms`);
  console.log(`  any ${S.K} of the ${e.shards.length} reconstruct the file exactly;`
    + ` any ${m} may die. wrote ${dir}/`);
}
else if(cmd === "join"){
  const files = gatherShards(bare.slice(1));
  const t0 = Date.now();
  const d = S.decode(files.map(f => fs.readFileSync(f)));
  const out = flag("out", "joined.out");
  fs.writeFileSync(out, d.bytes);
  console.log(`${files.length} shards (${d.alive} distinct, ${d.k} needed) -> ${d.bytes.length} B`
    + ` in ${Date.now() - t0} ms`);
  console.log(`  blocks: ${d.confirmed} confirmed, ${d.repaired} repaired, ${d.condemned} condemned`);
  if(d.suspects.length) console.log(`  convicted shards (by prime): ${d.suspects.join(", ")}`);
  console.log(`  wrote ${out}`);
  process.exit(d.condemned ? 1 : 0);
}
else if(cmd === "demo"){
  const file = bare[1];
  const src = fs.readFileSync(file);
  const m = +flag("m", 8), kill = +flag("kill", m);
  const t0 = Date.now();
  const e = S.encode(src, {m});
  const n = e.shards.length;
  /* the kill-set is drawn by lot -- the point is that it cannot matter */
  const idx = [...Array(n).keys()];
  for(let i = n - 1; i > 0; i--){ const r = Math.floor(Math.random() * (i + 1)); [idx[i], idx[r]] = [idx[r], idx[i]]; }
  const dead = new Set(idx.slice(0, kill));
  const survivors = e.shards.filter((_, i) => !dead.has(i));
  const d = S.decode(survivors);
  const exact = Buffer.from(d.bytes).equals(Buffer.from(src));
  const total = e.shards.reduce((a, s) => a + s.length, 0);
  console.log(`${file}: ${src.length} B -> ${n} shards (${total} B, ${(100 * total / src.length).toFixed(2)}%)`);
  console.log(`  killed by lot: shards ${[...dead].sort((a, b) => a - b).join(", ")}`);
  console.log(`  rejoined from the ${survivors.length} survivors: ${exact ? "EXACT" : "WRONG"}`
    + `  (${d.confirmed} blocks confirmed) in ${Date.now() - t0} ms`);
  process.exit(exact ? 0 : 1);
}
else {
  console.error("usage: shard.js split <file> [--m 8] | join <dir|shards...> | demo <file> [--kill n]");
  process.exit(1);
}
