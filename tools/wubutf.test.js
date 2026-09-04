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
/* serve.py starts nothing on import — the server lives under __main__ — so the
   API is callable directly and the test drives the same code the socket does. */
const api = (ep, q, lang = "") => JSON.parse(execFileSync("python3", ["-c",
  `import sys, json, serve; print(json.dumps(`
  + `serve.${ep}(*sys.argv[1:${ep === "api_base" ? 1 : 3}]), ensure_ascii=False))`,
  ...(ep === "api_base" ? [] : [q, lang])], {cwd: ROOT, encoding: "utf8"}));

(async () => {

/* ---- 1. the page survives having no server, and says so ---- */
{
  const p = loadPage(path.join(ROOT, "wubutf.html"));
  await tick(); await tick();
  const rows = p.els.get("rows");
  ok(rows, "the page never asked for #rows");
  ok(/the ordering lives in the server/.test(rows.innerHTML),
     "no-server path did not render its message: " + rows.innerHTML.slice(0, 80));
  console.log("  [1] no server      the page explains itself instead of blanking");
}

/* ---- 2. the page renders real API output ---- */
{
  const NAMES = "dis\nthis\nthin\ncanvas\nclear\ncervezas\ndox\nknicks\nkicks";
  const sorted = api("api_sort", NAMES, "en");
  const base = api("api_base", "", "");
  const p = loadPage(path.join(ROOT, "wubutf.html"), {
    fetch: (u) => Promise.resolve({ok: true, status: 200, statusText: "OK",
      json: () => Promise.resolve(u.startsWith("/api/base") ? base : sorted)})});
  for(let i = 0; i < 8; i++) await tick();
  const rows = p.g.document.getElementById("rows");
  /* one element per name, plus one divergence marker between each pair */
  const want = sorted.sorted.length * 2 - 1;
  ok(rows.children.length === want,
     `built ${rows.children.length} elements, expected ${want}`);
  ok(p.run("RING") === base.ring && p.run("WIDTH") === base.ring + 1,
     "the page did not take the ring from the server");
  console.log(`  [2] renders        ${sorted.sorted.length} names -> `
    + `${rows.children.length} elements, ring ${p.run("RING")} from /api/base`);
}

/* ---- 3. divergence names the square and the cell that decide the order ----
 * This is the whole point of the page: dis and this are one cell apart in the
 * sound order and nowhere near each other alphabetically.
 */
{
  const p = loadPage(path.join(ROOT, "wubutf.html"));
  await tick();
  const codesOf = s => api("api_read", s, "en").codes;
  const dis = codesOf("dis"), thi = codesOf("this"), thin = codesOf("thin");
  p.run(`globalThis._a = ${JSON.stringify(dis)};`
      + `globalThis._b = ${JSON.stringify(thi)};`
      + `globalThis._c = ${JSON.stringify(thin)};`);
  const dt = p.run("divergence(_a, _b)");
  const dn = p.run("divergence(_b, _c)");
  ok(dt && dt.code === 1 && dt.cell != null,
     "dis vs this should be decided inside the second square: " + JSON.stringify(dt));
  ok(dn && dn.code === 0,
     "this vs thin should be decided in the first square: " + JSON.stringify(dn));
  console.log(`  [3] divergence     dis|this decided at square ${dt.code + 1} cell ${dt.cell}`
    + `; this|thin at square ${dn.code + 1} cell ${dn.cell}`);
  console.log(`                     dis reads ${api("api_read","dis","en").reading}, `
    + `this reads ${api("api_read","this","en").reading} — one square apart by sound, `
    + `d and t apart alphabetically`);
}

/* ---- 4. every square is the same frame, which is why a row reads as a row ---- */
{
  const p = loadPage(path.join(ROOT, "wubutf.html"));
  await tick();
  const base = api("api_base", "", "");
  const shapes = new Set(), deads = new Set();
  for(const {code} of base.table){
    p.run(`globalThis._s = squareOf(${code})`);
    shapes.add(p.run("_s.n") + "x" + p.run("_s.cells.length"));
    deads.add(p.run("_s.dead"));
  }
  ok(shapes.size === 1 && deads.size === 1,
     `frames differ: ${[...shapes].join(",")} dead ${[...deads].join(",")}`);
  console.log(`  [4] one frame      all ${base.count} codes draw `
    + `${[...shapes][0]} with ${[...deads][0]} padding cells, every square comparable`);
}

/* ---- 5. the whole stack over a live socket ---- */
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
    const get = async p => {
      const r = await fetch(`http://127.0.0.1:${PORT}${p}`);
      return {status: r.status, body: await r.text()};
    };
    const page = await get("/wubutf.html");
    ok(page.status === 200 && /Wub UTF/.test(page.body), "page not served");
    const s = await get("/api/sort?lang=en&q=" + encodeURIComponent(
      "canvas\nclear\ncervezas\ndox\nknicks\nkicks"));
    const names = JSON.parse(s.body).sorted.map(x => x.name);
    ok(String(names) === String(["dox", "canvas", "kicks", "clear", "knicks", "cervezas"]),
       "live sort wrong: " + names);
    const r = await get("/api/read?q=" + encodeURIComponent("飼") + "&lang=ja-on");
    ok(JSON.parse(r.body).reading === "shi", "live read wrong: " + r.body);
    /* the two push levels, and the shape axis as its own switch */
    const ps2 = async (q, lang, push, read) => JSON.parse((await get(
      `/api/sort?lang=${encodeURIComponent(lang)}&push=${push}&read=${read || "sound"}`
      + `&q=${encodeURIComponent(q)}`)).body).sorted;
    const ps = (q, lang, push) => ps2(q, lang, push, "sound");
    const plain = await ps("cervezas", "", "");
    const ctx = await ps("cervezas", "", "ctx");
    const all = await ps("cervezas", "", "all");
    ok(plain.length === 1, "plain should give one row per name");
    ok(ctx.length > 1 && !ctx.some(r => r.reading === "kervezas"),
       "in context, c before e is never /k/ — kervezas must not appear");
    ok(all.some(r => r.reading === "kervezas"),
       "rules off, kervezas must appear: a character alone has no position");
    ok(new Set(ctx.map(r => r.reading)).size === ctx.length, "pushed rows repeat a reading");
    ok(ctx.every((r, i) => i === 0 || ctx[i - 1].reading < r.reading),
       "pushed rows are not in order");
    ok(ctx.every(r => r.name === "cervezas" && r.of === ctx[0].of),
       "every pushed row should name its parent and its count");
    /* the 2x2: representation and reading are independent */
    const grid = {};
    for(const rep of ["", "ctx"]) for(const rd of ["spell", "sound"])
      grid[rep + "/" + rd] = (await ps2("cervezas", rep === "" ? "" : "en", rep, rd));
    ok(grid["/spell"].length === 1 && grid["/spell"][0].reading === "cervezas",
       "plain + as written should be the string itself");
    ok(grid["ctx/spell"].some(r => r.reading === "c3rv3zas"),
       "pushed + as written should give the shape variants");
    ok(grid["/sound"].length === 1 && grid["/sound"][0].reading === "servezas",
       "plain + phonetic should be the primary reading");
    ok(grid["ctx/sound"].length >= 1, "pushed + phonetic should give the readings");
    console.log(`  [8] four combinations  as-written plain `
      + `${grid["/spell"][0].reading}, pushed ${grid["ctx/spell"].length} shape variants; `
      + `phonetic plain ${grid["/sound"][0].reading}, pushed ${grid["ctx/sound"][0].of}`);
    const leet = JSON.parse((await get("/api/read?lang=" + encodeURIComponent("en leet")
      + "&q=" + encodeURIComponent("c3rv3zas"))).body).reading;
    const noleet = JSON.parse((await get("/api/read?lang=en&q="
      + encodeURIComponent("c3rv3zas"))).body).reading;
    ok(leet === "servezas" && noleet === "k3rv3zas",
       `shape axis: got ${leet} with leet and ${noleet} without`);
    console.log(`  [6] push levels    plain 1 row, in-context ${ctx.length} `
      + `(no kervezas), rules off ${all[0].of} (kervezas present)`);
    console.log(`  [7] shape axis     c3rv3zas -> ${noleet} plain, ${leet} with leet `
      + `— same key as cervezas`);
    const bad = await get("/api/nope");
    ok(bad.status === 404, "unknown endpoint should 404, got " + bad.status);
    const big = await get("/api/read?q=" + "a".repeat(5000));
    ok(big.status === 413, "oversized query should 413, got " + big.status);
    console.log(`  [9] live stack     page 200, sort ${names.join(" ")}, `
      + `飼 as ja-on -> shi, /api/nope 404, 5000 chars 413`);
  } finally {
    srv.kill("SIGTERM");
  }
}

console.log("\n  certified.");
})().catch(e => { console.error(e.message); process.exit(1); });
