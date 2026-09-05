/* node tools/wubutf.test.js — Wub UTF: the code layer, the phasors, the page.
 *
 * The ordering has one implementation, in Python. The page asks for readings and
 * for the codes, so what is worth testing is the seam and the model built on it.
 */
const {loadPage} = require("./domharness.js");
const {execFileSync, spawn} = require("child_process");
const path = require("path");
const ROOT = path.join(__dirname, "..");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const tick = () => new Promise(r => setImmediate(r));
const drain = async (n = 12) => { for(let i = 0; i < n; i++) await tick(); };
const api = (ep, ...a) => JSON.parse(execFileSync("python3", ["-c",
  "import sys, json, serve; print(json.dumps("
  + `serve.${ep}(*sys.argv[1:]), ensure_ascii=False))`,
  ...a.map(String)], {cwd: ROOT, encoding: "utf8"}));
const PAGE = path.join(ROOT, "wubutf.html");
const Q = "hello, 3, 45\ndis, this, thin";
function seeded(q, lang, push, read, alphabet){
  const base = api("api_base");
  const cards = api("api_cards", q, lang, push, read, alphabet);
  return {base, cards, fetch: u => Promise.resolve({ok: true, status: 200,
    json: () => Promise.resolve(u.startsWith("/api/base") ? base : cards)})};
}

(async () => {

/* ---- 1. the page survives having no server ---- */
{
  const p = loadPage(PAGE);
  await drain();
  ok(/the ordering lives in the server/.test(
       p.g.document.getElementById("cards").innerHTML),
     "no-server path did not render its message");
  console.log("  [1] no server      the page explains itself instead of blanking");
}

/* ---- 2. rank orders, code draws ----
 * Comparing tuples of codes is the same as comparing tuples of ranks ONLY while
 * the code rises with the rank. That is the invariant that let every code be
 * respaced without moving a single ordering.
 */
{
  const b = api("api_base");
  const cs = b.table.map(r => r.code), rk = b.table.map(r => r.rank);
  let bad = 0;
  for(let i = 1; i < cs.length; i++) if(!(cs[i] > cs[i-1] && rk[i] > rk[i-1])) bad++;
  ok(bad === 0, `${bad} places where code does not rise with rank`);
  ok(cs.every(c => c > 0 && c < 2 ** b.width), "a code escaped the digit");
  ok(new Set(cs).size === cs.length, "duplicate code");
  const span = 360 * (Math.max(...cs) - Math.min(...cs)) / 2 ** b.width;
  ok(span > 300, `the alphabet only spans ${span.toFixed(0)} degrees`);
  console.log(`  [2] rank vs code   ${b.count} symbols, ${b.width}-bit digits, `
    + `codes ${Math.min(...cs)}..${Math.max(...cs)}, spanning ${span.toFixed(0)}° of 360`);
  console.log("                     " + b.blocks.map(x =>
    `${x.type} ${x.n} at stride ${x.stride}`).join(", "));
}

/* ---- 3. one phasor per character, and its angle is its code ---- */
{
  const s = seeded(Q, "en", "", "spell", "chroma");
  const it = s.cards.cards[0].items[0];
  ok(it.phasors.length === it.codes.length,
     `${it.phasors.length} phasors for ${it.codes.length} characters`);
  for(let i = 0; i < it.codes.length; i++){
    ok(Math.abs(it.phasors[i].turn - it.codes[i] / s.cards.base) < 1e-12,
       "a phasor's turn is not its code's share of the ring");
    ok(Math.abs(it.phasors[i].w - 2 ** -(i + 1)) < 1e-12,
       "a phasor's weight is not its positional weight");
    const p = it.phasors[i];
    ok(p.inner + p.fold + p.outer === p.lit,
       "inner + fold + outer must account for every lit cell");
  }
  console.log(`  [3] phasors        ${it.text} -> ${it.phasors.length}, `
    + `angles ${it.phasors.map(p => Math.round(360*p.turn) + "°").join(" ")}`);
}

/* ---- 4. the curve is built, finite, and the depth stays a cosine ----
 * proj returns d as a depth that consumers read as a cosine. It once came back
 * as a raw dot product, which put coordinates at 1e14, drove a point size
 * negative and threw IndexSizeError out of the draw loop.
 */
{
  const p = loadPage(PAGE, {fetch: seeded(Q, "en", "", "spell", "chroma").fetch});
  await drain();
  ok(p.run("CARDS.length") === 2, "expected 2 cards");
  ok(p.run("SEL && SEL.text") === "hello", "the first item should be selected");
  ok(p.run("PH.length") === p.run("SEL.codes.length"), "phasor count");
  ok(p.run("CURVE.length") > 100, "the curve was not traced");
  ok(p.run("CURVE.every(v => v.every(Number.isFinite))"), "the curve has a hole in it");
  ok(p.run("EXTENT") > 0, "the curve has no extent");
  const worst = p.run(`(() => { let m = 0;
    for(const v of CURVE) m = Math.max(m, Math.abs(proj(v, 0, 0, 1).d));
    return m; })()`);
  ok(worst <= 1.0000001, `depth is not a cosine, worst |d| = ${worst}`);
  console.log(`  [4] the curve      ${p.run("CURVE.length")} points, extent `
    + `${p.run("EXTENT").toFixed(3)}, depth stays a cosine (worst ${worst.toFixed(4)})`);
}

/* ---- 5. selecting a different string rebuilds the model ---- */
{
  const p = loadPage(PAGE, {fetch: seeded(Q, "en", "", "spell", "chroma").fetch});
  await drain();
  const before = p.run("CURVE.map(v => v.join()).join('|')").length;
  p.run("select(CARDS[1].items[2])");
  ok(p.run("SEL.text") === "thin", "selection did not move");
  ok(p.run("PH.length") === 4, "thin has four characters");
  const after = p.run("CURVE.map(v => v.join()).join('|')").length;
  ok(before !== after, "the curve did not change when the selection did");
  console.log(`  [5] selection      clicking thin rebuilds to `
    + `${p.run("PH.length")} phasors and a new curve`);
}

/* ---- 6. weight, and the card as a rack ----
 * 2^-(k+1) is the positional weight: correct for the value, ruinous for the
 * picture. Over eight characters the first is 96x the last, so phasor 0 drew a
 * circle and everything else was a wobble on it.
 */
{
  const p = loadPage(PAGE, {fetch: seeded("cervezas\nhello, 3, 45", "en", "",
    "spell", "chroma").fetch});
  await drain();
  const share = w => p.run(`(() => { WEIGHT = ${JSON.stringify(w)}; recompute();
    const m = PH.map(ph => { let x = 0;
      for(let i = 0; i <= 180; i++){ const c = cmp3(ph, Math.PI*2*i/180);
        x = Math.max(x, Math.hypot(c[0], c[1], c[2])); }
      return x; });
    const t = m.reduce((a, b) => a + b, 0);
    return m.map(v => v / t); })()`);
  const even = share("even"), val = share("value");
  const ratio = a => Math.max(...a) / Math.min(...a);
  ok(ratio(val) > 20, "the positional weight should be lopsided; that is the point");
  ok(ratio(even) < 6, `even weights are still lopsided: ${ratio(even).toFixed(1)}x`);
  /* a character fills a RING: it comes from the shared phasor(), so its radius,
     height and both rates are read off its whole cell arrangement */
  ok(p.run("PH.every(x => x.rateA > 1 && x.rateB > 0 && x.n === 4)"),
     "a character is not filling a ring");
  /* plain and pushed are not a toggle — Wub +- shows both at once, and push is
     a real second ring: it conserves the value and rewrites the cells, so the
     radius, height and run count all move and the fold can flip sign. */
  const rings = (pl, pu) => p.run(`PLAIN = ${pl}; PUSH = ${pu}; recompute();
    JSON.stringify([PH.length, PH.map(x => x.fold)])`);
  const [n1, f1] = JSON.parse(rings(true, false));
  const [n2, f2] = JSON.parse(rings(false, true));
  const [n3] = JSON.parse(rings(true, true));
  ok(n3 === n1 + n2, `both on should give ${n1 + n2} rings, got ${n3}`);
  ok(f1.some((v, i) => v !== f2[i]), "pushing did not change a single ring");
  ok(f1.some((v, i) => Math.sign(v) !== Math.sign(f2[i])),
     "pushing should flip a fold's sign somewhere, which flips the handedness");
  p.run("PLAIN = true; PUSH = false; recompute()");
  p.run('WEIGHT = "even"; recompute()');
  /* and a card is a rack: every item's phasors, laid end to end */
  p.run("selectCard(CARDS[1], 1)");
  const want = p.run("CARDS[1].items.reduce((a, i) => a + i.phasors.length, 0)");
  ok(p.run("PH.length") === want, "the card rack lost phasors");
  ok(p.run("CURVE.every(v => v.every(Number.isFinite))"), "the rack curve has a hole");
  console.log(`  [6] weight & rack  positional ${ratio(val).toFixed(0)}x lopsided, `
    + `even ${ratio(even).toFixed(1)}x; card 2 is a ${want}-phasor rack`);
}

/* ---- 7. an item that parses as an integer IS an integer ----
 * "-6" was being segmented into a hyphen and a six, so it never reached
 * hexSequence and the 1s never became -1s. And "45" was two digit characters
 * when it should be forty-five.
 */
{
  const p = loadPage(PAGE, {fetch: seeded("6, -6, 45\nhello", "en", "", "spell",
    "chroma").fetch});
  await drain();
  const six = p.run("CARDS[0].items[0]"), neg = p.run("CARDS[0].items[1]");
  ok(six.numeric && neg.numeric && !p.run("CARDS[1].items[0].numeric"),
     "numbers should be numeric and words should not");
  p.run("select(CARDS[0].items[2])");
  ok(p.run("PH.length") === 1, "45 is one integer, not two digit characters");
  p.run("select(CARDS[1].items[0])");
  ok(p.run("PH.length") === 5, "hello is still five characters");
  /* the cells flip sign and the handedness with them */
  const cells = k => p.run(`(() => { const {v, neg} = parse(${JSON.stringify(k)});
    return hexSequence(v, neg).cells.join(","); })()`);
  ok(cells("6") === "0,1,1,0" && cells("-6") === "0,-1,-1,0",
     `negation must flip the cells: 6 ${cells("6")}, -6 ${cells("-6")}`);
  p.run("select(CARDS[0].items[0])");
  const dirA = p.run("PH[0].dir"), foldA = p.run("PH[0].fold");
  p.run("select(CARDS[0].items[1])");
  ok(p.run("PH[0].dir") === -dirA && Math.abs(p.run("PH[0].fold") + foldA) < 1e-12,
     "negation must flip the handedness and the fold");
  /* and it is an exact reflection: the same curve run backwards */
  const worst = p.run(`(() => {
    const a = phasor("6", false), b = phasor("-6", false);
    a.phase = 0; b.phase = 0;
    let w = 0;
    for(let i = 0; i <= 240; i++){ const t = Math.PI*2*i/240;
      const A = comp(a, t), B = comp(b, -t);
      for(let k = 0; k < 3; k++) w = Math.max(w, Math.abs(A[k] - B[k])); }
    return w; })()`);
  ok(worst < 1e-12, `negation should be an exact reflection, worst ${worst}`);
  console.log(`  [7] integers       6 -> 0,1,1,0 and -6 -> 0,-1,-1,0; 45 is one `
    + `ring, hello is five`);
  console.log(`                     comp(-6, -t) == comp(6, t) to ${worst.toExponential(0)}`
    + " — negation is the same curve run backwards");
}

/* ---- 8. rank counts, code draws ----
 * Two different numbers again. The code is spread across the digit so the frame
 * has no dead cells; the rank is the dense position in the order and it is what
 * a string COUNTS in. With 126 IPA symbols the counting base is 127, so "10" is
 * 127 — reading the value off the spread codes gave the storage base instead.
 */
{
  const i = api("api_cards", "shit", "en", "", "sound", "ipa").cards[0].items[0];
  const c = api("api_cards", "hello", "en", "", "sound", "chroma").cards[0].items[0];
  ok(i.countBase === 127 && c.countBase === 307,
     `counting bases wrong: ipa ${i.countBase}, chroma ${c.countBase}`);
  const isPrime = n => { for(let d = 2; d * d <= n; d++) if(n % d === 0) return false;
    return n > 1; };
  ok(isPrime(i.countBase) && isPrime(c.countBase),
     "the counting base is the symbol count plus the zero, and should be prime");
  ok(i.ranks.every(r => r >= 0 && r < i.countBase), "a rank escaped the counting base");
  /* "10" is 127, not 256 */
  ok(1 * i.countBase + 0 === 127, "10 must count as 127 in the IPA alphabet");
  /* and the value really is the ranks in that base */
  let v = 0n;
  for(const r of c.ranks) v = v * BigInt(c.countBase) + BigInt(r);
  ok(v === BigInt(c.int), `the value is not the ranks in base ${c.countBase}`);
  ok(c.codes.some((x, k) => x !== c.ranks[k]),
     "code and rank should be different numbers");
  console.log(`  [8] rank vs count  ipa base ${i.countBase}, chroma base ${c.countBase}, `
    + `both prime; "10" counts 127`);
  console.log(`                     hello ranks [${c.ranks}] but draws codes `
    + `[${c.codes.slice(0, 2)}...]`);
}

/* ---- 9. three orderings, and each sets the storage width ----
 * A character is just another int; the only thing an ordering changes is how
 * big the int is, and the grid it is drawn in follows from that.
 */
{
  const p = loadPage(PAGE, {fetch: seeded("hi", "en", "", "sound", "chroma").fetch});
  await drain();
  const want = [["ipa", 8, 3, 127], ["chroma", 16, 4, 307], ["phonetic", 20, 5, 524287]];
  const seen = [];
  for(const [name, bits, side, base] of want){
    const j = api("api_cards", "hi", "en", "", "sound", name);
    const it = j.cards[0].items[0];
    ok(j.codeBits === bits, `${name} should store ${bits} bits, got ${j.codeBits}`);
    ok(it.countBase === base, `${name} counting base ${it.countBase}, wanted ${base}`);
    ok(it.codes.every(c => c < 2 ** bits), `${name}: a code overflows its width`);
    const g = p.run(`(() => { const x = gridFor(${it.codes[0]}, ${bits}, false);
      return x.n + "," + x.cells.length; })()`).split(",");
    ok(+g[0] === side, `${name} should draw ${side}x${side}, got ${g[0]}`);
    seen.push(`${name} ${bits}b ${side}x${side} base ${base}`);
  }
  /* and pushed is the same value written differently, so it draws its own grid */
  const same = p.run(`(() => {
    const a = gridFor(31988, 16, false), b = gridFor(31988, 16, true);
    return [hexValue(a.cells).num + "/" + hexValue(a.cells).den,
            hexValue(b.cells).num + "/" + hexValue(b.cells).den,
            a.cells.join("") === b.cells.join("")]; })()`);
  ok(same[0] === same[1], "pushing must conserve the value");
  ok(same[2] === false, "pushing must change the cells");
  console.log(`  [9] three widths   ${seen.join("; ")}`);
  console.log("                     pushed keeps the value and changes the cells, "
    + "so it draws beside the plain grid");
}

/* ---- 10. the whole stack over a live socket ---- */
{
  const PORT = 18338;
  const srv = spawn("python3", ["-u", "serve.py", "--port", String(PORT)],
    {cwd: ROOT, stdio: ["ignore", "pipe", "pipe"]});
  try{
    await new Promise((res, rej) => {
      srv.stdout.on("data", d => String(d).includes(String(PORT)) && res());
      srv.on("error", rej);
      setTimeout(() => rej(new Error("server did not start")), 15000);
    });
    const get = async u => {
      const r = await fetch(`http://127.0.0.1:${PORT}${u}`);
      return {status: r.status, body: await r.text()};
    };
    const page = await get("/wubutf.html");
    ok(page.status === 200 && /A letter is an integer/.test(page.body), "page not served");
    const cd = JSON.parse((await get("/api/cards?read=spell&q="
      + encodeURIComponent(Q))).body);
    ok(cd.cards.length === 2 && cd.cards[0].items[0].phasors.length === 5,
       "live cards wrong");
    ok((await get("/api/nope")).status === 404, "unknown endpoint should 404");
    ok((await get("/api/read?q=" + "a".repeat(5000))).status === 413, "should 413");
    console.log("  [10] live stack     page 200, 2 cards with phasors, 404 and 413 hold");
  } finally { srv.kill("SIGTERM"); }
}

console.log("\n  certified.");
})().catch(e => { console.error(e.message); process.exit(1); });
