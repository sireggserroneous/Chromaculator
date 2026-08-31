/* node tools/ui.test.js — the interaction layer's arithmetic.
 *
 * chroma-ui.js binds pointer and input listeners, and the harness stubs
 * addEventListener, so nothing reached through a listener runs here. The parts
 * worth testing are pure and are tested directly: the cell hit-test against
 * stalk.js's own cellOrder, and the scrubber's step arithmetic at its ends. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../index.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* 1. the hit-test inverts cellOrder exactly, at every width the fold uses */
{
  let cells = 0;
  for(let n = 1; n <= 12; n++){
    const order = run(`cellOrder(${n})`);
    ok(order.length === n * n, `cellOrder(${n}) gave ${order.length}, want ${n * n}`);
    order.forEach(([r, c], i) => {
      const got = run(`UI.indexOfCell(${r}, ${c}, ${n})`);
      ok(got === i, `indexOfCell(${r},${c},${n}) = ${got}, want ${i}`);
      cells++;
    });
  }
  console.log(`  indexOfCell inverts cellOrder over ${cells} cells, n = 1..12`);
}

/* 2. a point in a cell maps to that cell, and regions agree with regions() */
{
  let checked = 0, bad = 0;
  for(const n of [1, 2, 3, 4, 5, 8]){
    const size = 240, step = size / n;
    const want = run(`regions(Array(${n * n}).fill(1), ${n})`);
    const region = new Map();
    for(const key of ["inner", "fold", "outer"])
      for(const s of want[key]) region.set(s.r * n + s.c, key);
    for(let r = 0; r < n; r++) for(let c = 0; c < n; c++){
      /* aim at the middle of the cell, where no rounding rule is in doubt */
      const x = (c + 0.5) * step, y = (r + 0.5) * step;
      const hit = run(`UI.cellAt(${x}, ${y}, ${size}, ${n})`);
      ok(hit, `cellAt missed the middle of (${r},${c}) at n=${n}`);
      if(hit.r !== r || hit.c !== c) bad++;
      ok(hit.region === region.get(r * n + c),
         `region at (${r},${c}) n=${n}: got ${hit.region}, regions() says ${region.get(r * n + c)}`);
      checked++;
    }
  }
  ok(bad === 0, `${bad} cells hit the wrong square`);
  console.log(`  cellAt lands in the right cell and names the right region, ${checked} cells`);
}

/* 3. outside the square is null, not the nearest edge. A drag that leaves the
      grid must stop reporting, or it smears along the last row it touched. */
{
  const out = [[-1, 10], [10, -1], [240, 10], [10, 240], [1e6, 1e6], [-1e6, 3]];
  for(const [x, y] of out)
    ok(run(`UI.cellAt(${x}, ${y}, 240, 4)`) === null, `cellAt(${x},${y}) should be null`);
  /* and a degenerate box reports nothing rather than dividing by zero */
  ok(run(`UI.cellAt(1, 1, 0, 4)`) === null, "zero-size square should be null");
  ok(run(`UI.cellAt(1, 1, 240, 0)`) === null, "zero-width fold should be null");
  console.log(`  cellAt returns null outside the square (${out.length} points) and when degenerate`);
}

/* 4. every css pixel of the square belongs to exactly one cell — no seam
      between cells that reports null, which is what a reader feels as a drag
      that flickers. */
{
  const n = 5, size = 251;                    // deliberately not divisible by n
  let holes = 0;
  for(let x = 0; x < size; x++) for(let y = 0; y < size; y++)
    if(run(`UI.cellAt(${x}, ${y}, ${size}, ${n})`) === null) holes++;
  ok(holes === 0, `${holes} px inside the square hit no cell`);
  console.log(`  no seams: all ${size * size} px of a ${size}px square map to a cell`);
}

/* 5. phase() at its ends. t=1 must report every step done, not one past. */
{
  const p = t => run(`UI.phase(${t}, 8)`);
  ok(p(0).done === 0 && p(0).frac === 0, "t=0 should be step 0");
  ok(p(1).done === 8 && p(1).frac === 1, `t=1 gave ${JSON.stringify(p(1))}, want 8 done`);
  ok(p(1.5).done === 8, "t past 1 should clamp");
  ok(p(-3).done === 0, "t below 0 should clamp");
  ok(p(0.5).done === 4, `t=0.5 of 8 gave ${p(0.5).done}, want 4`);
  /* done never exceeds the total, at any t — the property that keeps a caller
     from indexing one past the end of its own array */
  let worst = 0;
  for(let t = -0.2; t <= 1.2; t += 0.001) worst = Math.max(worst, p(t).done);
  ok(worst === 8, `phase().done reached ${worst}, want at most 8`);
  console.log(`  phase() clamps at both ends and never runs past the last step`);
}

/* 6. a binder whose element the page does not have returns an inert handle
      rather than throwing. Nine pages wire different subsets of these. */
{
  for(const call of ["UI.slider('nope')", "UI.select('nope')", "UI.toggle('nope')"]){
    const h = run(`(() => { const h = ${call}; return {ok: h.ok, v: h.get()}; })()`);
    ok(h.ok === false, `${call} should report ok:false`);
    run(`${call}.set(1)`);                    // must not throw
    run(`${call}.on(() => {})`);
  }
  ok(run(`UI.drag('nope').ok`) === false, "drag on a missing canvas should be inert");
  ok(run(`UI.scrub('nope').get()`) === 0, "scrub on a missing range should read 0");
  console.log(`  missing elements give inert handles: slider, select, toggle, drag, scrub`);
}

/* 7. fit() must not reallocate a canvas it does not need to.
      Assigning width or height frees and retakes the backing store. These
      draw paths run inside a rAF loop, so doing it unconditionally was sixty
      buffer reallocations a second and a renderer that eventually died. */
{
  const seen = run(`(() => {
    const cv = document.getElementById("foldcv");
    let writes = 0;
    let w = 0, h = 0;
    Object.defineProperty(cv, "width",  {get: () => w, set: v => { writes++; w = v; }});
    Object.defineProperty(cv, "height", {get: () => h, set: v => { writes++; h = v; }});
    UI.fit(cv, 330);                       // first call: must size it
    const first = writes;
    for(let i = 0; i < 50; i++) UI.fit(cv, 330);   // fifty more at the same size
    const after = writes;
    return {first, after, w, h};
  })()`);
  ok(seen.first > 0, "the first fit() did not size the canvas at all");
  ok(seen.after === seen.first,
     `fit() reallocated on ${seen.after - seen.first} of 50 unchanged calls`);
  ok(seen.w > 0 && seen.h > 0, `the canvas came out ${seen.w}x${seen.h}`);
  console.log(`  fit() sizes the canvas once and leaves it alone for 50 more calls`);
}

/* 8. and it does resize when the box really changes, or the canvas would keep
      drawing at the old size after a window resize */
{
  const grew = run(`(() => {
    const cv = document.getElementById("pourcv");
    UI.fit(cv, 300);
    const before = cv.height;
    UI.fit(cv, 600);                       // a taller box
    return {before, after: cv.height};
  })()`);
  ok(grew.after !== grew.before,
     `fit() ignored a real size change (${grew.before} -> ${grew.after})`);
  console.log(`  ...but it does follow a real change of size`);
}

console.log("ui ok");
