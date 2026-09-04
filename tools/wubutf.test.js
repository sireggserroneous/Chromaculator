/* node tools/wubutf.test.js — Wub UTF, page and API together.
 *
 * The ordering has one implementation, in Python, and the page asks the server
 * for readings rather than carrying a second copy of the tables. So the thing
 * worth testing is the seam: the page's own logic against real API output, and
 * then the whole stack over a live socket.
 */
const {loadPage} = require("./domharness.js");
const {execFileSync, spawn} = require("child_process");
const path = require("path");
const ROOT = path.join(__dirname, "..");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const tick = () => new Promise(r => setImmediate(r));
const drain = async (n = 10) => { for(let i = 0; i < n; i++) await tick(); };

/* serve.py starts nothing on import — the server lives under __main__ — so the
   API is callable directly and the test drives the same code the socket does. */
const api = (ep, ...args) => JSON.parse(execFileSync("python3", ["-c",
  "import sys, json, serve; print(json.dumps("
  + `serve.${ep}(*sys.argv[1:]), ensure_ascii=False))`,
  ...args.map(String)], {cwd: ROOT, encoding: "utf8"}));

const PAGE = path.join(ROOT, "wubutf.html");
const Q = "hello, 3, 45\ncervezas, c3rv3zas\ndis, this, thin";

/* a fetch that answers all three endpoints from the real API */
function seeded(q, lang, push, read){
  const base = api("api_base");
  const cards = api("api_cards", q, lang, push, read);
  const sorted = api("api_sort", q, lang, push, read);
  return {base, cards, sorted, fetch: u => Promise.resolve({
    ok: true, status: 200, statusText: "OK",
    json: () => Promise.resolve(u.startsWith("/api/base") ? base
      : u.startsWith("/api/cards") ? cards : sorted)})};
}

(async () => {

/* ---- 1. the page survives having no server, and says so ---- */
{
  const p = loadPage(PAGE);
  await drain();
  const cards = p.g.document.getElementById("cards");
  ok(/the ordering lives in the server/.test(cards.innerHTML),
     "no-server path did not render its message: " + cards.innerHTML.slice(0, 90));
  console.log("  [1] no server      the page explains itself instead of blanking");
}

/* ---- 2. cards: one per line, its items, and every page renders ---- */
{
  const s = seeded(Q, "", "", "spell");
  const p = loadPage(PAGE, {fetch: s.fetch});
  await drain();
  ok(p.run("CARDS.length") === 3, "expected 3 cards, got " + p.run("CARDS.length"));
  ok(p.run("CARDS[0].items.length") === 3, "card 1 should hold hello, 3, 45");
  ok(p.run("CARDS[0].items.map(i => i.text).join('|')") === "hello|3|45",
     "card 1 items wrong: " + p.run("CARDS[0].items.map(i => i.text).join('|')"));
  ok(p.g.document.getElementById("cards").children.length === 3,
     "three cards should be painted");
  /* every page must build for every card, or a tab is a dead end */
  const pages = p.run("PAGES");
  for(const pg of pages) for(let i = 0; i < 3; i++)
    ok(p.run(`(() => { try { RENDER["${pg}"](CARDS[${i}]); return true; }
                       catch(e) { return e.message; } })()`) === true,
       `page ${pg} threw on card ${i + 1}`);
  console.log(`  [2] cards          3 cards from 3 lines, card 1 holds `
    + `${p.run("CARDS[0].items.map(i => i.text).join(', ')")}; `
    + `all ${pages.length} pages build on all 3`);
}

/* ---- 3. the integer IS the polynomial, base 4096 ---- */
{
  const s = seeded("hello, 3, 45", "en", "", "sound");
  const B = BigInt(s.cards.base);
  ok(s.cards.codeBits === 12, "a digit must be 12 bits: three whole nibbles");
  for(const it of s.cards.cards[0].items){
    let v = 0n;
    for(const c of it.codes) v = v * B + BigInt(c);
    ok(v === BigInt(it.int), `${it.text}: polynomial ${v} != integer ${it.int}`);
    ok(it.bits === it.codes.length * s.cards.codeBits, `${it.text}: bit count wrong`);
    ok(it.codes.every(c => c < s.cards.base), `${it.text}: a digit overflows the base`);
  }
  const sum = s.cards.cards[0].items.reduce((a, i) => a + BigInt(i.int), 0n);
  ok(sum === BigInt(s.cards.cards[0].sum), "rack sum is not the sum of the items");
  /* the VALUE is the fraction 0.d1 d2 d3..., and it is the one that carries the
     order. The integer is length dominant — more digits is always a bigger
     number — so it sorts short words first whatever they say. Kept as a live
     control: if it ever starts agreeing, this check has stopped meaning anything. */
  const W2 = ["he", "hell", "hello", "helllo", "helo"];
  const c2 = api("api_cards", W2.join("\n"), "", "", "spell").cards.map(c => c.items[0]);
  const frac = it => [BigInt(it.num), BigInt(it.den)];
  const cmpF = (a, b) => { const [an, ad] = frac(a), [bn, bd] = frac(b);
    const l = an * bd, r = bn * ad; return l < r ? -1 : l > r ? 1 : 0; };
  const byFrac = [...c2].sort(cmpF).map(x => x.text);
  const byInt = [...c2].sort((a, b) =>
    BigInt(a.int) < BigInt(b.int) ? -1 : BigInt(a.int) > BigInt(b.int) ? 1 : 0)
    .map(x => x.text);
  const real = api("api_sort", W2.join("\n"), "", "", "spell").sorted.map(x => x.name);
  ok(String(byFrac) === String(real),
     `the fraction must agree with the key: ${byFrac} vs ${real}`);
  ok(String(byInt) !== String(real),
     "the integer agreed with the key — the length-dominance control is dead");
  console.log(`  [3] value order    0.word ${byFrac.join(" ")}  == the sort`);
  console.log(`                     integer ${byInt.join(" ")}  != the sort `
    + "(length dominant, kept as a control)");
  const h = s.cards.cards[0].items[0];
  console.log(`  [4] base(chroma-utf)  ${h.text} reads ${h.reading}, digits `
    + `[${h.codes}] -> ${h.int} in base ${s.cards.base}, ${h.bits} bits`);
  console.log(`                     rack sum ${s.cards.cards[0].sum} = the integers a `
    + "Wub +- rack would hold");
}

/* ---- 4. divergence names the digit and the cell that decide the order ---- */
{
  const p = loadPage(PAGE, {fetch: seeded(Q, "en", "", "sound").fetch});
  await drain();
  const codesOf = w => api("api_read", w, "en").codes;
  const dis = codesOf("dis"), thi = codesOf("this"), thin = codesOf("thin");
  p.run(`globalThis._a = ${JSON.stringify(dis)};`
      + `globalThis._b = ${JSON.stringify(thi)};`
      + `globalThis._c = ${JSON.stringify(thin)};`);
  const dt = p.run("divergence(_a, _b)"), dn = p.run("divergence(_b, _c)");
  ok(dt && dt.code === 1 && dt.cell != null,
     "dis vs this should be decided inside the second digit: " + JSON.stringify(dt));
  ok(dn && dn.code === 0,
     "this vs thin should be decided in the first digit: " + JSON.stringify(dn));
  console.log(`  [5] divergence     dis|this at digit ${dt.code + 1} cell ${dt.cell}; `
    + `this|thin at digit ${dn.code + 1} cell ${dn.cell} — dis is the closer of the two`);
}

/* ---- 5. every digit is the same frame, which is why a row reads as a row ---- */
{
  const p = loadPage(PAGE, {fetch: seeded(Q, "", "", "spell").fetch});
  await drain();
  const base = api("api_base");
  const shapes = new Set(), deads = new Set();
  for(const row of base.table){
    p.run(`globalThis._s = squareOf(${row.code})`);
    shapes.add(p.run("_s.n") + "x" + p.run("_s.cells.length"));
    deads.add(p.run("_s.dead"));
  }
  ok(shapes.size === 1 && deads.size === 1,
     `frames differ: ${[...shapes].join(",")} dead ${[...deads].join(",")}`);
  console.log(`  [6] one frame      all ${base.count} digits draw ${[...shapes][0]} `
    + `with ${[...deads][0]} padding cells`);
}

/* ---- 6. the four combinations, and the two push levels ---- */
{
  const cell = (push, read) => api("api_sort", "cervezas",
    read === "sound" ? "en" : "", push, read).sorted;
  const plainSpell = cell("", "spell"), pushSpell = cell("ctx", "spell");
  const plainSound = cell("", "sound");
  ok(plainSpell.length === 1 && plainSpell[0].reading === "cervezas",
     "plain + as written should be the string itself");
  ok(pushSpell.some(r => r.reading === "c3rv3zas"),
     "pushed + as written should give the shape variants");
  ok(plainSound.length === 1 && plainSound[0].reading === "servezas",
     "plain + phonetic should be the primary reading");
  const ctxRows = api("api_sort", "cervezas", "", "ctx", "sound").sorted;
  const allRows = api("api_sort", "cervezas", "", "all", "sound").sorted;
  ok(!ctxRows.some(r => r.reading === "kervezas"),
     "in context, c before e is never /k/");
  ok(allRows.some(r => r.reading === "kervezas"), "rules off, kervezas must appear");
  console.log(`  [7] four combos    as written: plain "${plainSpell[0].reading}", `
    + `pushed ${pushSpell.length} shape variants`);
  console.log(`                     phonetic:   plain "${plainSound[0].reading}", `
    + `pushed ${ctxRows[0].of} in context (no kervezas), `
    + `${allRows[0].of} rules off (kervezas present)`);
}

/* ---- 7. the shape axis is orthogonal to the sound axis ---- */
{
  const leet = api("api_read", "c3rv3zas", "en leet").reading;
  const plain = api("api_read", "c3rv3zas", "en").reading;
  const target = api("api_read", "cervezas", "en").reading;
  ok(leet === target && plain !== target,
     `shape axis: ${plain} plain, ${leet} with leet, cervezas is ${target}`);
  console.log(`  [8] shape axis     c3rv3zas -> ${plain} plain, ${leet} with leet `
    + `— the same key as cervezas`);
}

/* ---- 8. the whole stack over a live socket ---- */
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
    ok(page.status === 200 && /base\(chroma/.test(page.body), "page not served");
    const cd = JSON.parse((await get("/api/cards?read=spell&q="
      + encodeURIComponent(Q))).body);
    ok(cd.cards.length === 3, "live cards wrong: " + cd.cards.length);
    const s = JSON.parse((await get("/api/sort?lang=en&read=sound&q="
      + encodeURIComponent("canvas\nclear\ncervezas\ndox\nknicks\nkicks"))).body);
    const names = s.sorted.map(x => x.name);
    ok(String(names) === String(["dox", "canvas", "kicks", "clear", "knicks", "cervezas"]),
       "live sort wrong: " + names);
    const r = JSON.parse((await get("/api/read?lang=ja-on&q="
      + encodeURIComponent("飼"))).body);
    ok(r.reading === "shi", "live read wrong: " + r.reading);
    ok((await get("/api/nope")).status === 404, "unknown endpoint should 404");
    ok((await get("/api/read?q=" + "a".repeat(5000))).status === 413,
       "oversized query should 413");
    console.log(`  [9] live stack     page 200, 3 cards, sort ${names.join(" ")}, `
      + "飼 as ja-on -> shi, /api/nope 404, 5000 chars 413");
  } finally { srv.kill("SIGTERM"); }
}

console.log("\n  certified.");
})().catch(e => { console.error(e.message); process.exit(1); });
