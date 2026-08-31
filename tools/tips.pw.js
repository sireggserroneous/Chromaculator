/* node tools/tips.pw.js — the tooltip, driven by a real mouse.
 *
 * The synthetic-event tests in tips.test.js pass and the tooltip still stuck
 * in a real browser, which tells you what those tests are worth here:
 * dispatching pointerleave proves the handler works, not that the browser
 * ever sends it. Playwright moves an actual pointer, so the events are the
 * ones Chrome really generates -- including the ones it declines to.
 *
 *   node tools/tips.pw.js            # needs the server on :1338
 *   node tools/tips.pw.js --headed   # watch it
 */
const {chromium} = require(__dirname + "/../node_modules/playwright");

const BASE = process.env.CC_BASE || "http://127.0.0.1:1338";
const HEADED = process.argv.includes("--headed");
const results = [];
const ok = (name, pass, note) => {
  results.push({name, pass, note});
  console.log(`  ${pass ? "ok  " : "FAIL"} ${name}${note ? "  — " + note : ""}`);
};

const visible = page => page.evaluate(() => {
  const t = document.getElementById("cc-tip");
  return !!t && !t.hidden;
});
/* wait for the card to go, up to `ms`, and report how long it took */
async function goneWithin(page, ms){
  const t0 = Date.now();
  while(Date.now() - t0 < ms){
    if(!(await visible(page))) return Date.now() - t0;
    await page.waitForTimeout(60);
  }
  return null;
}

(async () => {
  const browser = await chromium.launch({headless: !HEADED});
  const page = await browser.newPage({viewport: {width: 1400, height: 900}});
  page.on("pageerror", e => ok("page threw", false, e.message));

  /* ---------------------------------------------------------------- atlas */
  await page.goto(BASE + "/atlas.html", {waitUntil: "load"});
  await page.waitForTimeout(400);

  const ring = page.locator('label[data-tip="ring"]');
  await ring.waitFor();

  /* 1. a real hover opens it */
  await ring.hover();
  await page.waitForTimeout(250);
  ok("a real hover opens the card", await visible(page));

  /* 2. moving the real pointer away closes it */
  await page.mouse.move(1100, 700, {steps: 12});
  const t2 = await goneWithin(page, 2000);
  ok("moving the pointer away closes it", t2 !== null, t2 !== null ? t2 + "ms" : "still up after 2s");

  /* 3. THE REPORTED CASE. Hover the label, then drag the slider it belongs to.
        A range input takes an implicit pointer capture for the drag, which is
        where the leave event goes missing. */
  await ring.hover();
  await page.waitForTimeout(200);
  const shownBeforeDrag = await visible(page);
  const slider = page.locator("#rings");
  const box = await slider.boundingBox();
  await page.mouse.move(box.x + box.width * 0.5, box.y + box.height / 2);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width * 0.95, box.y + box.height / 2, {steps: 15});
  await page.mouse.up();
  await page.mouse.move(1100, 750, {steps: 12});
  const t3 = await goneWithin(page, 6000);
  ok("after dragging the slider it belongs to, it closes",
     t3 !== null, `shown before drag: ${shownBeforeDrag}; ` + (t3 !== null ? t3 + "ms" : "STILL UP after 6s"));

  /* 4. hover, then take the pointer off the window entirely */
  await ring.hover();
  await page.waitForTimeout(200);
  await page.mouse.move(-50, -50);
  const t4 = await goneWithin(page, 6000);
  ok("with the pointer off the window, the cap closes it",
     t4 !== null, t4 !== null ? t4 + "ms" : "STILL UP after 6s");

  /* 5. hover, then do nothing whatsoever -- no move, no key, no click */
  await ring.hover();
  await page.waitForTimeout(200);
  const t5 = await goneWithin(page, 7000);
  ok("with no input at all, the cap closes it",
     t5 !== null, t5 !== null ? t5 + "ms" : "STILL UP after 7s");

  /* 6. the <select> case: hover its label, then open the dropdown */
  const around = page.locator('label[data-tip="t_around"]');
  if(await around.count()){
    await around.hover();
    await page.waitForTimeout(200);
    await page.locator("#tourring").click();
    await page.keyboard.press("Escape");
    await page.mouse.move(1100, 780, {steps: 10});
    const t6 = await goneWithin(page, 6000);
    ok("after opening the dropdown it belongs to, it closes",
       t6 !== null, t6 !== null ? t6 + "ms" : "STILL UP after 6s");
  }

  /* 7. hovering a second control swaps the card rather than stacking */
  await ring.hover(); await page.waitForTimeout(200);
  await around.hover(); await page.waitForTimeout(250);
  const one = await page.evaluate(() => document.querySelectorAll("#cc-tip").length);
  ok("only ever one card in the document", one === 1, `found ${one}`);

  await page.mouse.move(1200, 800); await page.waitForTimeout(600);

  /* ------------------------------------------------------- the other pages */
  for(const [name, sel] of [["index.html", '[data-tip]'],
                            ["spectrometer.html", '[data-tip]'],
                            ["spec.html", '[data-tip]'],
                            ["inspirations.html", '[data-tip]'],
                            ["wub.html", '[data-tip]'],
                            ["wubx.html", '[data-tip]'],
                            ["wubdiv.html", '[data-tip]'],
                            ["wubbadub.html", '[data-tip]']]){
    await page.goto(BASE + "/" + name, {waitUntil: "load"});
    await page.waitForTimeout(400);
    const first = page.locator(sel).first();
    if(!(await first.count())) continue;
    await first.hover();
    await page.waitForTimeout(250);
    const shown = await visible(page);
    await page.mouse.move(20, 860, {steps: 10});
    const t = await goneWithin(page, 6000);
    ok(`${name}: opens on hover and closes after`, shown && t !== null,
       shown ? (t !== null ? t + "ms" : "STILL UP") : "never opened");
  }

  await browser.close();
  const bad = results.filter(r => !r.pass).length;
  console.log(bad ? `\ntips.pw FAILED (${bad} of ${results.length})` : `\ntips.pw ok (${results.length} checks)`);
  process.exit(bad ? 1 : 0);
})().catch(e => { console.error("harness error:", e.message); process.exit(2); });
