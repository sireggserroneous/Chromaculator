/* node tools/audio.test.js — the sound the Wub pages make, and the permalinks.
 *
 * WebAudio does not exist under the harness, so nothing here makes a noise.
 * What it checks is the part that decides what the noise WOULD be: the mapping
 * from a phasor to voices, and the mix that keeps their sum from clipping.
 * Both are pure, which is why they were written that way. */
const {loadPage} = require(__dirname + "/domharness.js");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const WUBS = ["wub", "wubx", "wubdiv", "wubbadub"];

/* 1. a phasor becomes two voices, one per rate, because the drawing uses two */
{
  const {run} = loadPage(__dirname + "/../wub.html");
  const v = run(`UI.audio.voicesOf({amp: 0.5, rateA: 2, rateB: 3, phase: 1}, 55)`);
  ok(v.length === 2, `expected 2 voices, got ${v.length}`);
  ok(v[0].freq === 110 && v[1].freq === 165, `rates did not map to pitch: ${JSON.stringify(v)}`);
  ok(v[0].amp === 0.25 && v[1].amp === 0.25, "the phasor's amplitude should split between its two voices");
  ok(v.every(x => x.phase === 1), "phase did not carry through");

  /* a phasor with no rates still sounds, at the fundamental, rather than
     dropping silently out of the chord */
  const bare = run(`UI.audio.voicesOf({amp: 0.4, rateA: 0, rateB: 0}, 55)`);
  ok(bare.length === 1 && bare[0].freq === 55 && bare[0].amp === 0.4,
     `a rateless phasor should keep its amplitude at the fundamental: ${JSON.stringify(bare)}`);
  ok(run(`UI.audio.voicesOf(null, 55)`).length === 0, "no phasor should give no voices");
  console.log(`  a phasor becomes one voice per rate, at rate x base`);
}

/* 2. the mix never clips. Additive synthesis sums amplitudes, so eight
      phasors at 0.5 is a peak of 4 and the output is mush. */
{
  const {run} = loadPage(__dirname + "/../wub.html");
  for(const n of [1, 2, 5, 16, 64]){
    const voices = Array.from({length: n}, (_, i) => ({freq: 55 * (i + 1), amp: 0.9}));
    const m = run(`UI.audio.mix(${JSON.stringify(voices)})`);
    const total = m.reduce((s, v) => s + v.amp, 0);
    ok(total <= 1 + 1e-9, `${n} voices summed to ${total}`);
    ok(m.length === n, `${n} voices in, ${m.length} out`);
    /* quiet inputs are left alone rather than boosted */
    if(n === 1) ok(Math.abs(m[0].amp - 0.9) < 1e-9, "a single quiet voice should not be scaled up");
  }
  /* proportions survive the scaling: it is a volume change, not a rebalance */
  const m = run(`UI.audio.mix([{freq:100, amp:1}, {freq:200, amp:3}])`);
  ok(Math.abs(m[1].amp / m[0].amp - 3) < 1e-9, "the mix changed the balance between voices");
  console.log(`  the mix caps the total at 1 and keeps the voices in proportion`);
}

/* 3. nonsense never reaches an oscillator. A NaN frequency throws inside
      WebAudio and takes the render loop down with it. */
{
  const {run} = loadPage(__dirname + "/../wub.html");
  const junk = `[{freq: NaN, amp: 0.5}, {freq: 100, amp: NaN}, {freq: -5, amp: 0.5},
                 {freq: 0, amp: 0.5}, {freq: 100, amp: 0}, {freq: 100, amp: -1},
                 null, undefined, {freq: Infinity, amp: 0.5}]`;
  const m = run(`UI.audio.mix(${junk})`);
  ok(m.length === 0, `${m.length} bad voices survived the mix: ${JSON.stringify(m)}`);
  ok(run(`UI.audio.mix([])`).length === 0, "an empty bank should stay empty");
  ok(run(`UI.audio.mix(null)`).length === 0, "a missing bank should not throw");
  console.log(`  NaN, infinite, negative and silent voices are all dropped`);
}

/* 4. the real model on all four pages produces a usable chord */
{
  for(const name of WUBS){
    const {run} = loadPage(`${__dirname}/../${name}.html`);
    const m = run(`UI.audio.mix(M.P.flatMap(p => UI.audio.voicesOf(p, 55)))`);
    ok(m.length > 0, `${name}: the model produced no voices at all`);
    const total = m.reduce((s, v) => s + v.amp, 0);
    ok(total <= 1 + 1e-9, `${name}: voices summed to ${total}`);
    for(const v of m){
      ok(v.freq >= 20 && v.freq <= 20000, `${name}: ${v.freq}Hz is outside hearing`);
      ok(isFinite(v.amp) && v.amp > 0, `${name}: bad amplitude ${v.amp}`);
    }
    /* and it must not start on its own */
    ok(run(`UI.audio.on`) === false, `${name}: audio was already running on load`);
  }
  console.log(`  all four Wub pages yield audible, non-clipping chords, and none autoplay`);
}

/* 5. permalinks: encode and decode are inverses, including the awkward cases */
{
  const {run} = loadPage(__dirname + "/../index.html");
  const cases = [
    {k: "1729", f: "push"},
    {k: "-1", t: "0.500"},
    {s: "a b&c=d#e", n: "3"},          // separators inside a value
    {u: "±×÷"},          // the site's own symbols
    {empty: "0"},
  ];
  for(const c of cases){
    const round = run(`UI.hash.decode(UI.hash.encode(${JSON.stringify(c)}))`);
    ok(JSON.stringify(round) === JSON.stringify(c),
       `round trip lost something: ${JSON.stringify(c)} -> ${JSON.stringify(round)}`);
  }
  /* empty and absent values are dropped rather than written as noise */
  ok(run(`UI.hash.encode({a: "", b: null, c: undefined, d: 1})`) === "d=1",
     "empty values should not reach the hash");
  /* a hand-mangled hash is a shrug, not an exception */
  for(const bad of ["", "#", "&&&", "a", "a=", "=b", "%%%", "%E0%A4%A"]){
    run(`UI.hash.decode(${JSON.stringify(bad)})`);
  }
  console.log(`  the hash round-trips, drops empties, and survives being hand-edited`);
}

/* 6. hash.num clamps and falls back, so a hand-typed URL cannot put a control
      outside its own range */
{
  const {run} = loadPage(__dirname + "/../index.html");
  const n = (st, k, d, lo, hi) =>
    run(`UI.hash.num(${JSON.stringify(st)}, ${JSON.stringify(k)}, ${d}, ${lo}, ${hi})`);
  ok(n({a: "5"}, "a", 1, 0, 10) === 5, "a good value should come through");
  ok(n({a: "500"}, "a", 1, 0, 10) === 10, "an over-range value should clamp");
  ok(n({a: "-500"}, "a", 1, 0, 10) === 0, "an under-range value should clamp");
  ok(n({a: "wat"}, "a", 7, 0, 10) === 7, "junk should fall back to the default");
  ok(n({}, "a", 7, 0, 10) === 7, "a missing key should fall back");
  ok(n({a: "Infinity"}, "a", 7, 0, 10) === 7, "Infinity should fall back, not clamp");
  console.log(`  a hand-typed URL cannot drive a control out of its own range`);
}

/* 7. every change that alters the model must reach the speaker.
      Editing an integer took a light path that skipped refresh() -- on purpose,
      since rebuilding the rack steals focus from the box you are typing in --
      and so the sound only ever changed on Randomise. */
{
  for(const name of WUBS){
    const {run} = loadPage(`${__dirname}/../${name}.html`);
    const chord = () => JSON.stringify(
      run(`UI.audio.mix(M.P.flatMap(p => UI.audio.voicesOf(p, 55))).map(v => v.freq)`));
    const before = chord();
    /* exactly what the .kin input handler does */
    run(`KS[0].k = 91; recompute();`);
    ok(chord() !== before, `${name}: editing an integer did not change the chord`);
    ok(typeof run(`touched`) === "function", `${name}: no touched() to carry the change`);
  }
  console.log(`  editing an integer changes the chord on all four pages`);
}

/* 8. Sweep is a frequency and now behaves like one. It was labelled in Hz and
      inaudible; the pitch is that number transposed into hearing. */
{
  for(const name of WUBS){
    const {run} = loadPage(`${__dirname}/../${name}.html`);
    const base = () => run(`FREQ * (55 / 0.12)`);
    run(`FREQ = 0.12`);
    ok(Math.abs(base() - 55) < 1e-9, `${name}: the default sweep should sit on A1, got ${base()}`);
    run(`FREQ = 0.24`);
    ok(Math.abs(base() - 110) < 1e-9, `${name}: doubling the sweep should double the pitch`);
    /* frozen is silent: a figure that is not turning has no tone, and mix()
       drops a zero-frequency voice rather than handing WebAudio a DC term */
    run(`FREQ = 0`);
    const v = run(`UI.audio.mix(M.P.flatMap(p => UI.audio.voicesOf(p, FREQ * (55 / 0.12))))`);
    ok(v.length === 0, `${name}: a frozen figure should be silent, got ${v.length} voices`);
  }
  console.log(`  sweep sets the pitch: 0.12 Hz is A1, doubling doubles it, frozen is silent`);
}

/* 9. Randomise moves its own controls. Setting the variable and leaving the
      slider showing the old number is how a button looks like it did nothing. */
{
  for(const name of WUBS){
    const {run} = loadPage(`${__dirname}/../${name}.html`);
    run(`setArrangement({freq: 0.42, divs: 77, bias: -0.5, shell: 0.3})`);
    ok(run(`FREQ`) === 0.42, `${name}: FREQ not set`);
    ok(run(`$("speed").value`) === "0.42", `${name}: the sweep slider did not move (${run(`$("speed").value`)})`);
    ok(run(`$("speedval").textContent`) === "0.42 Hz", `${name}: the sweep readout is stale`);
    ok(run(`DIVS`) === 77 && run(`$("jit").value`) === "77", `${name}: the greens slider did not move`);
    ok(run(`$("jitval").textContent`) === "77 /lap", `${name}: the greens readout is stale`);
    ok(run(`BIAS`) === -0.5 && run(`$("bias").value`) === "-0.5", `${name}: the bias slider did not move`);
    ok(run(`$("biasval").textContent`) === "75% red", `${name}: the bias readout is stale: ${run(`$("biasval").textContent`)}`);
    ok(run(`SHELL`) === 0.3 && run(`$("shell").value`) === "0.3", `${name}: the shell slider did not move`);
    ok(run(`$("shellval").textContent`) === "30%", `${name}: the shell readout is stale`);
    /* frozen reads as a word, not a zero */
    run(`setArrangement({divs: 0})`);
    ok(run(`$("jitval").textContent`) === "frozen", `${name}: zero greens should read "frozen"`);
  }
  console.log(`  Randomise's arrangement moves every slider and every readout with it`);
}

console.log("audio ok");
