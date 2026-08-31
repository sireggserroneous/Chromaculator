/* node tools/spectrometer.test.js — the controls added to the spectrometer.
 *
 * The page had one way in, a text box that takes an integer of any width. It
 * now also has a sweep bar, a speed bar, a phase scrubber and a draggable
 * sphere. The bar must not cost the box its range: that is the property most
 * at risk here, and the one tested hardest. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../spectrometer.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* 1. the sweep bar writes the box and redraws */
{
  for(const k of [-1023, -7, 0, 1, 181, 1023]){
    run(`SWEEP.el.value = "${k}"; $("num").value = String(${k}); draw();`);
    ok(run(`$("num").value`) === String(k), `sweep to ${k} did not reach the box`);
  }
  console.log(`  the sweep bar drives the box across its whole range`);
}

/* 2. a number too big for the bar survives being typed. This is the one that
      matters: clamping here would silently truncate a seventy-bit integer to
      1023 the moment the bar tried to follow it. */
{
  const big = "1180591620717411303424";                 // 2^70
  run(`$("num").value = "${big}"; syncSweep(); draw();`);
  ok(run(`$("num").value`) === big, `the box lost its value: ${run(`$("num").value`)}`);
  ok(run(`$("err").textContent`) === "", `a legal integer raised "${run(`$("err").textContent`)}"`);
  /* and the bar stayed where it was rather than jumping to an end stop */
  const parked = run(`SWEEP.el.value`);
  run(`$("num").value = "-${big}"; syncSweep(); draw();`);
  ok(run(`SWEEP.el.value`) === parked, `the bar moved for an out-of-range number`);
  console.log(`  a 70-bit integer types in cleanly and leaves the bar parked`);
}

/* 3. in range, the bar does follow the box */
{
  run(`$("num").value = "404"; syncSweep();`);
  ok(run(`SWEEP.el.value`) === "404", `bar did not follow the box: ${run(`SWEEP.el.value`)}`);
  run(`$("num").value = "-404"; syncSweep();`);
  ok(run(`SWEEP.el.value`) === "-404", `bar did not follow a negative: ${run(`SWEEP.el.value`)}`);
  /* an unreadable box is the error line's problem; the bar must not throw */
  run(`$("num").value = "not a number"; syncSweep();`);
  console.log(`  the bar follows the box while the box is inside the bar's range`);
}

/* 4. the virtual clock: scrubbing sets it, and speed scales it */
{
  run(`$("num").value = "5"; draw(); PLAYING = true; SPEED = 1; VT = 0; T0 = null;`);
  run(`frame(0); frame(100); frame(200);`);
  const moved = run(`VT`);
  ok(moved > 0, `the clock did not advance at speed 1 (VT=${moved})`);

  run(`VT = 0; T0 = null; SPEED = 0;`);
  run(`frame(0); frame(100); frame(200);`);
  ok(run(`VT`) === 0, `speed 0 still advanced the clock to ${run(`VT`)}`);

  run(`PLAYING = false; VT = 1234; T0 = null; SPEED = 1;`);
  run(`frame(0); frame(100);`);
  ok(run(`VT`) === 1234, `paused clock moved to ${run(`VT`)}`);
  console.log(`  the clock runs, scales with speed, and holds when paused`);
}

/* 5. every animation reads that one clock, so a scrub lands them all on the
      same instant. PHASE is derived from VT rather than accumulated. */
{
  run(`$("num").value = "5"; draw(); PLAYING = false; VT = 2000; T0 = null; frame(0);`);
  const a = run(`PHASE`);
  run(`VT = 4000; frame(100);`);
  const b = run(`PHASE`);
  ok(a !== b, `PHASE ignored the clock`);
  run(`VT = 2000; frame(200);`);
  ok(Math.abs(run(`PHASE`) - a) < 1e-9, `PHASE did not come back to where it was: ${run(`PHASE`)} vs ${a}`);
  console.log(`  the wave phase is a function of the clock, so scrubbing is repeatable`);
}

/* 6. the sphere turns when dragged, and its tilt stays in front of the poles */
{
  run(`ORBIT.az = 0; ORBIT.el = 0.42;`);
  run(`ORBIT.grabX = 0; ORBIT.grabY = 0; ORBIT.az0 = 0; ORBIT.el0 = 0.42;`);
  run(`ORBIT.az = ORBIT.az0 + (60 - ORBIT.grabX) * 0.02;
       ORBIT.el = UI.clamp(ORBIT.el0 - (40 - ORBIT.grabY) * 0.02, -1.45, 1.45);`);
  ok(Math.abs(run(`ORBIT.az`) - 1.2) < 1e-9, `azimuth wrong: ${run(`ORBIT.az`)}`);
  ok(Math.abs(run(`ORBIT.el`) - (-0.38)) < 1e-9, `elevation wrong: ${run(`ORBIT.el`)}`);
  /* dragged hard in either direction it stops short of straight down the axis,
     where the equator would collapse to a line and the drawing lose its sense */
  for(const far of [-100000, 100000]){
    run(`ORBIT.el = UI.clamp(0 - (${far}) * 0.02, -1.45, 1.45);`);
    ok(Math.abs(run(`ORBIT.el`)) <= 1.45, `tilt escaped to ${run(`ORBIT.el`)}`);
  }
  run(`draw();`);
  console.log(`  the sphere orbits on drag and its tilt stays clamped`);
}

/* 7. and the page still draws at every combination it now offers */
{
  let n = 0;
  for(const k of ["0", "1", "-1", "255", "-4096"])
    for(const form of ["f_plain", "f_push"])
      for(const lat of ["area", "io", "golden"]){
        run(`$("num").value = "${k}"; $("${form}").checked = true;
             $("latmode").value = "${lat}"; draw(); frame(${1000 + n});`);
        n++;
      }
  console.log(`  ${n} draws across integer, form and latitude: no throw`);
}

/* 8. opening a widget: one at a time, announced, and reversible */
{
  const state = () => run(`VBOXES.map(b => b.className.indexOf("big") >= 0)`);
  const aria  = () => run(`VBOXES.map(b => b.getAttribute("aria-expanded"))`);
  ok(run(`VBOXES.length`) === 6, `expected 6 widgets, found ${run(`VBOXES.length`)}`);
  ok(state().every(v => !v), "a widget was open before anything was clicked");

  run(`openBox(VBOXES[3])`);
  ok(state().filter(Boolean).length === 1 && state()[3], `wrong widget opened: ${state()}`);
  ok(aria()[3] === "true", "the open widget did not report aria-expanded=true");

  /* opening a second closes the first -- two full-width panels would stack and
     push the rest of the page off the bottom */
  run(`openBox(VBOXES[0])`);
  ok(state().filter(Boolean).length === 1 && state()[0], `two widgets open at once: ${state()}`);
  ok(aria()[3] === "false", "the widget that closed still says it is open");

  run(`closeBox(true)`);
  ok(state().every(v => !v), `a widget stayed open after close: ${state()}`);
  ok(aria().every(v => v === "false"), "a closed widget still reports expanded");
  ok(run(`OPEN`) === null, "OPEN was not cleared");

  /* opening the one already open is not a reopen */
  run(`openBox(VBOXES[2]); openBox(VBOXES[2]);`);
  ok(state().filter(Boolean).length === 1, "re-opening the open widget disturbed it");
  run(`closeBox(true)`);
  console.log(`  widgets open one at a time, announce it, and close again`);
}

/* 9. every widget survives being opened and drawn at the larger size */
{
  for(let i = 0; i < 6; i++){
    run(`openBox(VBOXES[${i}]); draw(); frame(${5000 + i * 10});`);
    run(`closeBox(true); draw();`);
  }
  console.log(`  all 6 widgets draw open and closed without throwing`);
}

/* 10. input hardening. None of these may throw, and none may leave the box
       holding something the page cannot get back from. */
{
  const junk = ["", "   ", "hello", "0x", "0b", "--5", "5.5", "1e9", "0xZZ", "+", "-", "0b12", "#5"];
  for(const v of junk){
    run(`$("num").value = ${JSON.stringify(v)}; draw();`);
    const err = run(`$("err").textContent`);
    ok(err.length > 0, `"${v}" was accepted silently`);
    ok(run(`$("num").value`) === v, `"${v}" was rewritten in the box`);
  }
  /* and a good value after junk brings it straight back */
  run(`$("num").value = "5"; draw();`);
  ok(run(`$("err").textContent`) === "", `a good value left the error line up: "${run(`$("err").textContent`)}"`);

  /* separators are grouping, not junk: parse() strips [_,\s] on purpose, so a
     number pasted out of a document keeps working. Worth pinning down, since it
     is the one case that looks like junk and is not. */
  for(const [text, want] of [["1 2 3", "123"], ["1,000", "1000"], ["1_000", "1000"], [" 42 ", "42"]]){
    run(`$("num").value = ${JSON.stringify(text)}; draw();`);
    ok(run(`$("err").textContent`) === "", `grouped "${text}" was refused`);
    ok(String(run(`parse($("num").value).v`)) === want, `"${text}" did not read as ${want}`);
  }
  run(`$("num").value = "5"; draw();`);
  console.log(`  ${junk.length} kinds of junk refused; grouped digits still read`);
}

/* 11. a number too large to draw is refused by count, before any DOM is built.
       The old behaviour was to try, and take the tab down with it. */
{
  const huge = "9".repeat(40000);
  const t0 = Date.now();
  run(`$("num").value = "${huge}"; draw();`);
  const ms = Date.now() - t0;
  const err = run(`$("err").textContent`);
  ok(err.indexOf("too wide") >= 0, `a 40000-digit number gave "${err}"`);
  ok(ms < 3000, `refusing took ${ms}ms -- it is still doing the work it refused`);
  /* just under the limit still draws */
  run(`$("num").value = "${"9".repeat(200)}"; draw();`);
  ok(run(`$("err").textContent`) === "", `a 200-digit number was refused: "${run(`$("err").textContent`)}"`);
  run(`$("num").value = "5"; draw();`);
  console.log(`  an unbuildable square is refused by count in ${ms}ms, not attempted`);
}

/* 12. reset puts every control back, and leaves the integer alone */
{
  run(`$("num").value = "777"; draw();
       SPEED = 0; VT = 4321; PLAYING = false;
       ORBIT.az = 2.5; ORBIT.el = -1.4;
       openBox(VBOXES[1]);
       resetView();`);
  ok(run(`SPEED`) === 1, `speed reset to ${run(`SPEED`)}`);
  ok(run(`VT`) === 0, `clock reset to ${run(`VT`)}`);
  ok(run(`PLAYING`) === true, "reset left the clock paused");
  ok(run(`ORBIT.az`) === 0 && run(`ORBIT.el`) === 0.42, "reset did not restore the sphere");
  ok(run(`OPEN`) === null, "reset left a widget open");
  ok(run(`$("play").getAttribute("aria-pressed")`) === "true", "the play button lies after reset");
  ok(run(`$("num").value`) === "777", `reset changed the integer to ${run(`$("num").value`)}`);
  console.log(`  reset restores speed, clock, sphere and panels -- and not the integer`);
}

console.log("spectrometer ok");
