/* node tools/gizmo.test.js — the corner gizmo actually points where it says.
   Runs against wubx.html; wub.html carries the same block. */
const {loadPage} = require(__dirname + "/domharness.js");
const page = process.argv[2] || (__dirname + "/../wubx.html");
const {run} = loadPage(page);
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* the camera sits where proj's depth term points: (sin S cos T, cos S cos T, sin T) */
const camera = () => run(`[Math.sin(SPIN)*Math.cos(TILT), Math.cos(SPIN)*Math.cos(TILT), Math.sin(TILT)]`);
const settle = () => { run(`SNAP.at -= 9999;`); run(`drawSphere({x:.1,y:.1,z:.1}, [])`); };

/* 1. clicking a ball puts the camera on that axis, from any starting angle */
{
  const AX = [[1,0,0],[-1,0,0],[0,1,0],[0,-1,0],[0,0,1],[0,0,-1]];
  let worst = 0;
  for(const a of AX) for(const [s, t] of [[0.7,-0.2],[-2.9,1.1],[3.0,0.0]]){
    run(`SPIN = ${s}; TILT = ${t}; SNAP = null;`);
    run(`snapTo(${JSON.stringify(a)})`); settle();
    const v = camera();
    worst = Math.max(worst, Math.max(...a.map((c, i) => Math.abs(v[i] - c))));
  }
  ok(worst < 1e-12, `camera off axis by ${worst}`);
  console.log(`  6 axes x 3 starting angles: camera lands on the axis, worst error ${worst.toExponential(1)}`);
}

/* 2. it takes the short way round — never more than half a turn of spin */
{
  let worst = 0;
  for(let s = -3.1; s <= 3.1; s += 0.37){
    run(`SPIN = ${s}; TILT = 0; SNAP = null;`);
    run(`snapTo([1,0,0])`);
    worst = Math.max(worst, Math.abs(run("SNAP.s1 - SNAP.s0")));
    settle();
  }
  ok(worst <= Math.PI + 1e-9, `travelled ${worst} rad, more than half a turn`);
  console.log(`  longest snap is ${worst.toFixed(3)} rad — never the long way round`);
}

/* 3. home resets, and stops the drift */
{
  run(`SPIN = 2.2; TILT = -1.0; AUTOSPIN = true; SNAP = null; snapTo(null)`); settle();
  ok(Math.abs(run("SPIN")) < 1e-12 && Math.abs(run("TILT") - 0.38) < 1e-12,
     `home went to ${run("SPIN")}, ${run("TILT")}`);
  ok(run("AUTOSPIN") === false, "taking the wheel should stop the auto-spin");
  console.log(`  home -> spin 0, tilt 0.38, auto-spin off`);
}

/* 4. every ball you can see is clickable where it is drawn */
{
  run(`SPIN = 0.6; TILT = 0.35; SNAP = null;`);
  const balls = run(`gizBalls(300, 300)`);
  const c = run(`gizCentre(300, 300)`);
  let hit = 0, skipped = 0;
  for(const b of balls){
    if(Math.hypot(b.px - c.x, b.py - c.y) < 9){ skipped++; continue; }  // sits on home
    const got = run(`gizHit({clientX: ${b.px}, clientY: ${b.py}})`);
    ok(got && got !== "home", `no ball at ${b.n}${b.pos ? "+" : "-"}`);
    ok(Math.hypot(got.px - b.px, got.py - b.py) < 1, "hit the wrong ball");
    hit++;
  }
  ok(run(`gizHit({clientX: ${c.x}, clientY: ${c.y}})`) === "home", "centre is not home");
  ok(run(`gizHit({clientX: 20, clientY: 260})`) === null, "far away should be a plain drag");
  console.log(`  ${hit}/6 balls hit at their drawn position (${skipped} behind home), centre is home,`
    + ` elsewhere falls through to drag`);
}

/* 5. the gizmo never leaves the canvas */
{
  for(const [W, H] of [[300,300],[1200,700],[420,900]]){
    const c = run(`gizCentre(${W}, ${H})`);
    const r = run("GIZ.r") + 13;
    ok(c.x + r <= W && c.x - r >= 0 && c.y + r <= H && c.y - r >= 0,
       `gizmo off-canvas at ${W}x${H}`);
  }
  console.log("  stays inside the canvas at 300x300, 1200x700 and 420x900");
}
console.log("\nall good.");
