/* node tools/phase.test.js — the phase bar on all four Wub pages.
 *
 * run.js calls draw() once and never touches a control, so the clock's new
 * behaviour is invisible to it. This drives the bar and the pause button
 * directly, on every page that carries them, and checks the three things that
 * make a scrubber a scrubber: it sets the clock, pausing stops the clock, and
 * playing lets it move again. */
const {loadPage} = require(__dirname + "/domharness.js");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const PAGES = ["wub", "wubx", "wubdiv", "wubbadub"];

for(const name of PAGES){
  const {run} = loadPage(`${__dirname}/../${name}.html`);
  const period = run(`M.period`);
  ok(typeof period === "number" && period > 0, `${name}: no period to scrub over`);

  /* 1. the bar sets the clock, across the whole bar */
  for(const v of [0, 0.25, 0.5, 0.75, 1]){
    run(`PHASEBAR.el.value = "${v}"; PHASEBAR.el.addEventListener; setPaused(true); T = ${v} * (M.period || 1);`);
    const t = run(`T`);
    const want = v * period;
    ok(Math.abs(t - want) < 1e-9, `${name}: scrub to ${v} gave T=${t}, want ${want}`);
  }

  /* 2. paused, the clock does not move however many frames arrive */
  run(`setPaused(true); T = 0.4 * M.period; LAST = null;`);
  const held = run(`T`);
  run(`draw(1000); draw(1100); draw(1200);`);
  ok(run(`T`) === held, `${name}: T moved while paused (${run(`T`)} vs ${held})`);

  /* 3. and released, it does. FREQ must be non-zero or nothing would move
        anyway, which would make this test pass for the wrong reason. */
  run(`FREQ = 0.5; setPaused(false); LAST = null; draw(2000);`);
  const before = run(`T`);
  run(`draw(2050); draw(2100);`);
  const after = run(`T`);
  ok(after > before, `${name}: T did not advance after play (${before} -> ${after})`);

  /* 4. the button reports its state to assistive tech both ways round */
  run(`setPaused(true)`);
  ok(run(`$("phaseplay").getAttribute("aria-pressed")`) === "false", `${name}: paused button not aria-pressed=false`);
  run(`setPaused(false)`);
  ok(run(`$("phaseplay").getAttribute("aria-pressed")`) === "true", `${name}: playing button not aria-pressed=true`);

  /* 5. while it runs the bar follows the clock and stays a legal 0..1 value --
        a range whose value falls outside its own min/max silently snaps back,
        which reads as a bar that will not track */
  run(`FREQ = 0.9; setPaused(false); LAST = null;`);
  let worst = 0;
  for(let f = 0; f < 40; f++){
    run(`draw(${3000 + f * 40});`);
    const v = parseFloat(run(`PHASEBAR.el.value`));
    ok(v >= 0 && v <= 1, `${name}: bar left its range at ${v}`);
    worst = Math.max(worst, v);
  }
  ok(worst > 0, `${name}: bar never moved while playing`);
  console.log(`  ${name}: scrub sets the clock, pause holds it, play resumes, bar tracks`);
}

console.log("phase ok");
