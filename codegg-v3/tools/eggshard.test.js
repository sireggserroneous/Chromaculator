/* node codegg-v3/tools/eggshard.test.js -- the file as a number, as residues.
 *
 * The contract under test is a true encoder's: properties BY CONSTRUCTION,
 * independent of the input. #6 is the one that answers v2's failure directly:
 * noise and prose must encode at identical cost with identical guarantees.
 * #3 is the loud refusal: k-1 shards must not decode wrongly, they must not
 * decode at all. #5 is the system's oldest promise kept at file scale:
 * arithmetic on the encoding, carries never crossing shards. */
const S = require(__dirname + "/../eggshard.js");
const fs = require("fs"), path = require("path");

const ok = (c, msg) => { if(!c) throw new Error("FAIL " + msg); };
function mul32(a){ return function(){ a |= 0; a = a + 0x6D2B79F5 | 0;
  let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
  return (t ^ t >>> 14) >>> 0; }; }
const g = mul32(3);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const eq = (a, b) => Buffer.from(a).equals(Buffer.from(b));

/* pick a random j-subset of 0..n-1 */
const subset = (n, j) => {
  const idx = [...Array(n).keys()];
  for(let i = n - 1; i > 0; i--){ const r = g() % (i + 1); [idx[i], idx[r]] = [idx[r], idx[i]]; }
  return new Set(idx.slice(0, j));
};

/* 1. round-trip on the awkward shapes, all shards present */
{
  for(const [label, src] of [
    ["empty", new Uint8Array(0)],
    ["one byte", Uint8Array.from([0xA7])],
    ["127 B", randBytes(127)],
    ["128 B exactly", randBytes(128)],
    ["129 B", randBytes(129)],
    ["all zero", new Uint8Array(500)],
    ["all 0xFF (top of range)", Uint8Array.from({length: 500}, () => 0xff)],
    ["10 KB random", randBytes(10240)],
  ]){
    const e = S.encode(src, {m: 8});
    ok(eq(S.decode(e.shards).bytes, src), `round-trip broke on ${label}`);
  }
  console.log("  round-trip exact: 8 shapes, all 73 shards");
}

/* 2. ANY m shards may die: random m-subsets, and every single shard in turn,
      so no shard can secretly be special */
{
  const src = randBytes(4096);
  const e = S.encode(src, {m: 8});
  const n = e.shards.length;                 // 73
  for(let t = 0; t < 60; t++){
    const dead = subset(n, 8);
    const keep = e.shards.filter((_, i) => !dead.has(i));
    ok(eq(S.decode(keep).bytes, src), `died on kill-set ${[...dead].join(",")}`);
  }
  for(let x = 0; x < n; x++){
    const keep = e.shards.filter((_, i) => i !== x);
    ok(eq(S.decode(keep).bytes, src), `shard ${x} turned out to be special`);
  }
  console.log(`  60 random 8-shard kill sets + each of ${n} shards alone: all EXACT`);
}

/* 3. k-1 shards refuse loudly -- never a wrong file, always an error */
{
  const src = randBytes(1024);
  const e = S.encode(src, {m: 8});
  let threw = false;
  try { S.decode(e.shards.slice(0, S.K - 1)); } catch(err){ threw = true; }
  ok(threw, "64 shards produced output instead of refusing");
  console.log(`  ${S.K - 1} shards: refused, as it must -- never a silent wrong file`);
}

/* 4. conviction: corrupt one shard's bytes; the spares vote it out per block,
      the culprit is NAMED, and the file comes back exact */
{
  const src = randBytes(4096);
  const e = S.encode(src, {m: 8});
  const victim = 7;
  const bad = Uint8Array.from(e.shards[victim]);
  for(let i = 0; i < 6; i++) bad[18 + 2 * (g() % 32)] ^= 0xff;   // scribble residues
  const shards = e.shards.map((s, i) => i === victim ? bad : s);
  const d = S.decode(shards);
  ok(eq(d.bytes, src), "corrupted shard was not overruled");
  ok(d.repaired > 0, "no block reported repaired");
  const culpritPrime = S.parseShard(e.shards[victim]).prime;
  ok(d.suspects.length === 1 && d.suspects[0] === culpritPrime,
    `suspects ${JSON.stringify(d.suspects)}, culprit prime ${culpritPrime}`);
  console.log(`  1 scribbled shard among 73: ${d.repaired} blocks repaired,`
    + ` culprit prime ${d.suspects[0]} named, file EXACT`);
}

/* 5. arithmetic in the encoded domain: add two files shard-wise, never
      decoding either; the reconstruction equals the blockwise numeric sums,
      verified against BigInt. Scaling likewise. Carries cannot cross shards
      because each shard is its own world. */
{
  const a = randBytes(1024), b = randBytes(1024);
  const ea = S.encode(a, {m: 8}), eb = S.encode(b, {m: 8});
  const sum = ea.shards.map((s, i) => S.shardAdd(s, eb.shards[i]));
  const vals = S.decodeValues(sum);
  for(let c = 0; c < vals.length; c++){
    const want = S.blockValue(a, c * S.BLOCK) + S.blockValue(b, c * S.BLOCK);
    ok(vals[c] === want, `block ${c}: encoded-domain sum disagrees with BigInt`);
  }
  const tripled = ea.shards.map(s => S.shardScale(s, 3));
  const v3 = S.decodeValues(tripled);
  for(let c = 0; c < v3.length; c++)
    ok(v3[c] === 3n * S.blockValue(a, c * S.BLOCK), `block ${c}: scale by 3 disagrees`);
  console.log(`  shard-wise A+B and 3*A reconstruct to the exact numeric results:`
    + ` arithmetic without decoding, carries never crossing shards`);
}

/* 6. structure-independence -- the answer to v2. Noise and prose, same size,
      byte-identical encoding cost, same guarantees. No auction, no luck. */
{
  const cpath = path.join(__dirname, "..", "..", "codegg-v2", "corpus-1489k.bin");
  const noise = randBytes(200000);
  const prose = fs.existsSync(cpath) ? fs.readFileSync(cpath).subarray(0, 200000)
                                     : randBytes(200000);
  const en = S.encode(noise, {m: 8}), ep = S.encode(prose, {m: 8});
  const size = e => e.shards.reduce((a, s) => a + s.length, 0);
  ok(size(en) === size(ep), `noise ${size(en)} B vs prose ${size(ep)} B -- structure leaked in`);
  const dead = subset(73, 8);
  ok(eq(S.decode(en.shards.filter((_, i) => !dead.has(i))).bytes, noise), "noise kill failed");
  ok(eq(S.decode(ep.shards.filter((_, i) => !dead.has(i))).bytes, prose), "prose kill failed");
  const ratio = size(en) / noise.length;
  console.log(`  noise and prose, 200,000 B each: identical ${size(en)} B encodings`
    + ` (${(100 * ratio).toFixed(2)}%), identical guarantees -- structure is irrelevant by design`);
}

/* 7. the overhead is (k+m)/k plus headers, exactly -- reported, not admired */
{
  for(const m of [0, 4, 8, 16]){
    const src = randBytes(65536);
    const e = S.encode(src, {m});
    const total = e.shards.reduce((a, s) => a + s.length, 0);
    /* 2 B per block per shard on 128 B blocks, plus an 18 B header per shard */
    const want = (S.K + m) / 64 + (S.K + m) * 18 / src.length;
    const got = total / src.length;
    ok(Math.abs(got - want) < 0.001, `m=${m}: ratio ${got.toFixed(4)} vs ${want.toFixed(4)}`);
    console.log(`  m=${String(m).padStart(2)}: ${e.shards.length} shards, `
      + `${(100 * got).toFixed(2)}% of input -- survives any ${m} deaths`);
  }
}

/* 8. garbage is refused */
{
  let threw = false;
  try { S.decode([randBytes(100)]); } catch(e){ threw = true; }
  ok(threw, "garbage accepted as a shard");
  console.log("  garbage shard refused");
}

console.log("eggshard ok");
