/* node tools/chroma_poly.test.js — is a word a polynomial, and are its readings pushes?
 *
 * Pedro's claim, in two halves:
 *
 *   (a) a word can be treated as a sum of polynomials
 *   (b) the phonetic variants are PUSHED versions of one thing, the way a lit
 *       cell can be rewritten as (+1,-1) or (-1,+1)
 *
 * (a) is exact and already built. (b) is structural but not numeric, and the
 * measurement below says exactly where it stops.
 */
const fs = require("fs");
const {execFileSync} = require("child_process");
const path = require("path");
const ROOT = path.join(__dirname, "..");
eval(fs.readFileSync(path.join(ROOT, "stalk.js"), "utf8"));

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const W = 12;                       // ring 9 + 3: the field a code is padded into
const X = 1n << BigInt(W);          // so x = 1/X

/* readings, branch counts and codes come from the one Python implementation */
const py = (code, ...args) => JSON.parse(execFileSync("python3", ["-c",
  "import sys, os, json; sys.path.insert(0, 'tools');\n"
  + "import chroma_sort as S, chroma_phonetic as P, chroma_utf as C\n" + code,
  ...args], {cwd: ROOT, encoding: "utf8"}));

const facts = (words, lang) => py(
  `w = sys.argv[1].split("\\n"); lang = sys.argv[2] or None\n`
  + `out = []\n`
  + `for s in w:\n`
  + `    segs = S.segment(s)\n`
  + `    accept = P.accept_for(P.candidates(lang)[0])\n`
  + `    per = []\n`
  + `    for i, g in enumerate(segs):\n`
  + `        if g == S.SEP: per.append([S.SEP]); continue\n`
  + `        b = S.branches(g, i, segs, s)\n`
  + `        if accept:\n`
  + `            k = [x for x in b if P.exact_match(x[2], accept)]\n`
  + `            b = k or b\n`
  + `        per.append(list(dict.fromkeys(x[0] for x in b)))\n`
  + `    rs = ["".join(x[0] for x in r) for r in S.readings(s, lang)]\n`
  + `    out.append({"word": s, "primary": S.key(s, lang)[1],\n`
  + `                "codes": C.letters(S.key(s, lang)[1]),\n`
  + `                "branchCounts": [len(p) for p in per],\n`
  + `                "readings": rs,\n`
  + `                "readingCodes": [C.letters(r) for r in rs[:64]]})\n`
  + `print(json.dumps(out, ensure_ascii=False))`,
  words.join("\n"), lang || "");

/* the polynomial: value = sum of code_i * x^(i+1), x = 2^-W, as an exact rational */
function poly(codes){
  let num = 0n, den = 1n;
  for(const c of codes){ num = num * X + BigInt(c); den = den * X; }
  return {num, den};
}
const cmpFrac = (a, b) => {
  const l = a.num * b.den, r = b.num * a.den;
  return l < r ? -1 : l > r ? 1 : 0;
};
const cmpSeq = (a, b) => {
  for(let i = 0; i < Math.max(a.length, b.length); i++){
    const p = a[i] === undefined ? -1 : a[i], q = b[i] === undefined ? -1 : q_(b, i);
    if(p !== q) return p < q ? -1 : 1;
  }
  return 0;
};
const q_ = (b, i) => b[i] === undefined ? -1 : b[i];

const WORDS = ["cerveza", "cerezas", "zapato", "canvas", "clear", "dox", "kicks",
  "knicks", "this", "thin", "dis", "box", "isit", "shit", "apple", "zebra",
  "photo", "knight", "night", "enough", "psalm", "who", "rose", "city", "gem"];

/* ---- 1. a word IS a polynomial, and lexicographic == numeric ---- */
{
  const f = facts(WORDS, "en");
  const rows = f.map(x => ({w: x.word, codes: x.codes, v: poly(x.codes)}));
  ok(rows.every(r => r.codes.every(c => c < (1 << W))),
     "a code overflows its field, so the fields would carry into each other");
  let bad = 0, pairs = 0;
  for(const a of rows) for(const b of rows){
    if(a === b) continue;
    pairs++;
    if(cmpFrac(a.v, b.v) !== cmpSeq(a.codes, b.codes)) bad++;
  }
  ok(bad === 0, `${bad}/${pairs} pairs where the value disagrees with the key order`);
  const mx = Math.max(...rows.flatMap(r => r.codes));
  console.log(`  [1] a word is a polynomial   value = SUM code_i * x^(i+1), x = 2^-${W}`);
  console.log(`                               ${pairs} pairs: value order == key order, `
    + `no exceptions`);
  console.log(`                               largest coefficient ${mx} < ${1 << W}, `
    + `so the fields never carry — the same reason the product rectangle is carry free`);
  console.log(`      cerveza = ` + rows.find(r => r.w === "cerveza").codes
    .map((c, i) => `${c}x^${i + 1}`).join(" + "));
}

/* ---- 2. the branch product IS a polynomial expansion ---- */
{
  const f = facts(WORDS, null);
  let bad = [];
  for(const x of f){
    const want = x.branchCounts.reduce((a, b) => a * b, 1);
    if(want <= 4096 && x.readings.length > want) bad.push([x.word, x.readings.length, want]);
  }
  ok(!bad.length, "expansion exceeded the product: " + JSON.stringify(bad.slice(0, 3)));
  const eg = f.find(x => x.word === "cerveza");
  console.log(`  [2] readings are the expansion  a word is a PRODUCT of per-grapheme sums;`);
  console.log(`                               expanding it enumerates the readings`);
  console.log(`      cerveza  branches ${eg.branchCounts.join(" x ")} = `
    + `${eg.branchCounts.reduce((a, b) => a * b, 1)} terms, `
    + `${eg.readings.length} distinct after dedupe`);
}

/* ---- 3. the language filter IS specialisation: terms go to zero ---- */
{
  const all = facts(["cerveza", "zapato", "cerezas"], null);
  for(const lang of ["en", "es-ES", "es-419"]){
    const some = facts(["cerveza", "zapato", "cerezas"], lang);
    for(let i = 0; i < all.length; i++){
      const sup = new Set(all[i].readings);
      ok(some[i].readings.every(r => sup.has(r)),
         `${lang} produced a reading the unfiltered set does not contain, on ${all[i].word}`);
      ok(some[i].readings.length <= all[i].readings.length, "filtering grew the set");
    }
  }
  const a = all.find(x => x.word === "cerveza");
  const es = facts(["cerveza"], "es-ES")[0], en = facts(["cerveza"], "en")[0];
  console.log(`  [3] the filter specialises   declaring a language sets terms to zero;`);
  console.log(`                               the survivors are always a subset`);
  console.log(`      cerveza  unfiltered ${a.readings.length} terms  ->  `
    + `en ${en.readings.length} (${en.primary})  ->  es-ES ${es.readings.length} (${es.primary})`);
}

/* ---- 4. but the readings do NOT share a value, so they are not pushes ---- */
{
  const f = facts(["cerveza"], null)[0];
  const vals = f.readingCodes.map(poly);
  /* exact, not truncated. Hashing these as num*2^64/den said 32 readings held
     only 10 values — cerveza's key is 84 bits, so the truncation collided. The
     same trap the character certificate keeps a control for. */
  const uniq = new Set(vals.map(v => v.num.toString() + "/" + v.den.toString()));
  ok(uniq.size > 1, "the readings all shared a value, which would make them pushes");
  ok(uniq.size === new Set(f.readingCodes.map(c => c.join(","))).size,
     `${vals.length - uniq.size} readings share an exact value; the polynomial `
     + "should be injective on code sequences");
  /* push conserves value exactly. Two readings with different values cannot be
     push variants of each other, however alike they look. */
  const lo = vals.reduce((a, b) => cmpFrac(a, b) < 0 ? a : b);
  const hi = vals.reduce((a, b) => cmpFrac(a, b) > 0 ? a : b);
  const spread = Number(hi.num * 10000n / hi.den) / 10000
               - Number(lo.num * 10000n / lo.den) / 10000;
  console.log(`  [4] readings are NOT pushes  push conserves value exactly; `
    + `these ${vals.length} readings`);
  console.log(`                               hold ${uniq.size} different values, `
    + `spread ${spread.toFixed(4)} of the unit`);
}

/* ---- 5. push on a word leaves the alphabet ---- */
{
  const f = facts(["cerveza"], "en")[0];
  const bits = [];
  for(const c of f.codes) for(const b of c.toString(2).padStart(W, "0")) bits.push(+b);
  const before = hexValue(bits);
  const p = pushLeft(bits.slice());
  const after = hexValue(p);
  ok(before.num * after.den === after.num * before.den,
     "push did not conserve the word's value");
  const neg = p.filter(c => c < 0).length;
  ok(neg > 0, "push produced no negative cells, so there is nothing to say");
  const decodable = p.every(c => c === 0 || c === 1);
  ok(!decodable, "the pushed form still looks like plain bits");
  console.log(`  [5] push exits the alphabet  pushing cerveza's ${bits.length} bits `
    + `conserves the value exactly,`);
  console.log(`                               but introduces ${neg} negative cells — `
    + `and no code is negative,`);
  console.log(`                               so a pushed word is a value, not a spelling.`);
}

console.log(`
  WHERE THE ANALOGY LANDS
    The polynomial half is exact, and it was already the implementation: a word
    is a product of per-grapheme sums, expanding it enumerates the readings, and
    declaring a language specialises it by sending terms to zero.
    The push half is structural, not numeric. Push and the reading set share the
    shape "one object, many representations, one canonical choice" — and picking
    the primary branch really is the analogue of pushing to the fixpoint. But
    push conserves value and phonetic variation does not, so a reading is not
    reachable from another by pushing.`);
console.log("\n  certified.");
