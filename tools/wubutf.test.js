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

/* ---- 6. the alphabet changes the digits, the base and the frame ---- */
{
  const c = api("api_cards", "shit", "en", "", "sound", "chroma");
  const i = api("api_cards", "shit", "en", "", "sound", "ipa");
  ok(c.base === 4096 && c.codeBits === 12, "chroma should be 12-bit, base 4096");
  ok(i.base === 256 && i.codeBits === 8, "ipa should be 8-bit, base 256");
  const ci = c.cards[0].items[0], ii = i.cards[0].items[0];
  ok(ii.codes.length < ci.codes.length && ii.bits < ci.bits,
     "IPA should need fewer, narrower digits");
  ok(ii.phasors.every(p => p.n === 3) && ci.phasors.every(p => p.n === 4),
     "IPA draws 3x3 and Chroma 4x4");
  console.log(`  [6] alphabets      chroma ${ci.codes.length} digits/${ci.bits} bits `
    + `in 4x4; ipa ${ii.codes.length}/${ii.bits} in 3x3`);
}

/* ---- 7. the whole stack over a live socket ---- */
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
    console.log("  [7] live stack     page 200, 2 cards with phasors, 404 and 413 hold");
  } finally { srv.kill("SIGTERM"); }
}

console.log("\n  certified.");
})().catch(e => { console.error(e.message); process.exit(1); });
