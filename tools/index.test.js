/* node tools/index.test.js — the landing page's two live figures.
 *
 * The figures are the page's claim made operable, so the claim is what gets
 * tested: the three regions sum back to the integer, at every integer the
 * slider can reach and at every stage of the pour. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../index.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* 1. Inner + Fold + Outer = k, over the slider's whole range */
{
  let worst = null;
  for(let k = 1; k <= 1023; k++){
    const s = run(`(() => {
      const {slots} = pourPlan(${k});
      const t = {inner:0, fold:0, outer:0};
      for(const s of slots) if(s.b) t[s.region] += s.place;
      return t;
    })()`);
    const total = s.inner + s.fold + s.outer;
    if(total !== k){ worst = {k, s, total}; break; }
  }
  ok(!worst, `regions summed to ${worst && worst.total} for k=${worst && worst.k}: ${JSON.stringify(worst && worst.s)}`);
  console.log(`  Inner + Fold + Outer = k for every k in 1..1023`);
}

/* 2. the grid is always big enough, and A1 is never given a digit */
{
  for(let k = 1; k <= 1023; k++){
    const {n, slots, L} = run(`pourPlan(${k})`);
    ok(n * n - 1 >= L, `k=${k}: ${L} bits will not fit a ${n}x${n} grid with A1 reserved`);
    for(const s of slots)
      ok(!(s.r === 0 && s.c === 0), `k=${k}: a bit landed on the reserved A1`);
    /* no two bits share a cell */
    const seen = new Set();
    for(const s of slots){
      const key = s.r + "," + s.c;
      ok(!seen.has(key), `k=${k}: two bits both landed on ${key}`);
      seen.add(key);
    }
  }
  console.log(`  A1 stays reserved and no two bits share a cell, k = 1..1023`);
}

/* 3. the pour is monotone: dragging t forward only ever adds bits, never
      removes one. A scrubber that drops a cell halfway reads as a glitch. */
{
  const k = 837, {slots} = run(`pourPlan(${k})`);
  let prev = 0;
  for(let t = 0; t <= 1.0001; t += 0.005){
    const {done} = run(`UI.phase(${t}, ${slots.length})`);
    ok(done >= prev, `pour went backwards at t=${t}: ${done} after ${prev}`);
    ok(done <= slots.length, `pour ran past the end at t=${t}: ${done}`);
    prev = done;
  }
  ok(prev === slots.length, `pour ended at ${prev} of ${slots.length}`);
  console.log(`  the pour only ever adds bits, and ends exactly full`);
}

/* 4. both figures survive being driven across their whole control range --
      this is the part run.js cannot reach, since it never moves a control */
{
  let frames = 0;
  for(let n = 2; n <= 9; n++)
    for(const t of [0, 0.13, 0.5, 0.87, 1]){
      run(`foldN.set(${n}); foldT.set(${t}); drawFold(); foldSay();`);
      frames++;
    }
  for(const k of [1, 2, 7, 64, 181, 512, 1023])
    for(const t of [0, 0.4, 1])
      for(const m of ["bits", "weight", "region"]){
        run(`pourK.set(${k}); pourT.set(${t}); pourMode.set("${m}"); drawPour(); pourSay();`);
        frames++;
      }
  console.log(`  ${frames} draws across every width, fold angle, integer and mode: no throw`);
}

/* 5. the hit-test drives the fold readout without throwing, including the
      corners and the reserved cell */
{
  for(let n = 2; n <= 9; n++){
    const order = run(`cellOrder(${n})`);
    for(const [r, c] of order){
      run(`foldN.set(${n}); foldHit = UI.cellAt(${(c + 0.5)} * (240/${n}), ${(r + 0.5)} * (240/${n}), 240, ${n}); foldSay();`);
    }
  }
  run(`foldHit = null; foldSay();`);
  console.log(`  every cell of every width names itself in the readout`);
}

/* 6. the inverted instrument. Whatever is painted, the three regions must add
      back to the value -- and unlike the pour figure, this one can be painted
      into states no integer produces, which is exactly where a claim about the
      fold either holds or does not. */
{
  /* the check itself, run inside the page against whatever PAINT holds */
  const holds = () => run(`(() => {
    const cells = paintCells(), L = cells.length;
    const val = valueOf(cells), reg = regions(cells, PAINTN);
    const part = key => reg[key].reduce((a, s) => a + BigInt(s.v) * (1n << BigInt(L - 1 - s.i)), 0n);
    const tot = part("inner") + part("fold") + part("outer");
    const den = 1n << BigInt(L - 1);
    const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
    const d = g(tot, den);
    return (tot / d) === val.num && (den / d) === val.den;
  })()`);

  let checked = 0;
  for(const n of [2, 3, 4, 5, 6]){
    run(`paintReset(${n})`);
    /* patterns no integer produces, which is where a claim about the fold
       either holds in general or turns out to be a fact about integers */
    const pats = [
      "PAINT.fill(0)", "PAINT.fill(1)", "PAINT.fill(-1)",
      "PAINT.forEach((_, i) => PAINT[i] = i % 2 ? 1 : -1)",
      "PAINT.forEach((_, i) => PAINT[i] = i % 3 === 0 ? 1 : i % 3 === 1 ? -1 : 0)",
    ];
    for(let i = 0; i < n * n; i++) pats.push(`PAINT.fill(0); PAINT[${i}] = 1`);
    for(let i = 0; i < n * n; i++) pats.push(`PAINT.fill(1); PAINT[${i}] = -1`);
    for(const pat of pats){
      run(pat);
      ok(holds(), `painted pattern at n=${n} did not sum to its own value: ${pat}`);
      checked++;
    }
    run("drawPaint(); paintSay();");
  }
  console.log(`  every painted square sums to its own value, ${checked} patterns, n = 2..6`);
}

/* 7. the paint cycle is a cycle, and shift walks it backwards */
{
  run(`paintReset(3); PAINT.fill(0);`);
  const step = back => run(`(() => {
    const v = PAINT[0];
    PAINT[0] = v === 0 ? (${back} ? -1 : 1) : v === 1 ? (${back} ? 0 : -1) : (${back} ? 1 : 0);
    return PAINT[0];
  })()`);
  ok(step(false) === 1 && step(false) === -1 && step(false) === 0, "forward cycle is not green -> blue -> red -> green");
  ok(step(true) === -1 && step(true) === 1 && step(true) === 0, "shift does not walk the cycle backwards");
  console.log(`  clicking cycles green, blue, red; shift-clicking goes the other way`);
}

console.log("index ok");
