/* node tools/atlas.test.js — the atlas's own arithmetic, and its labels.
 *
 * The panel prints several numbers about one integer and a footnote saying how
 * they relate. That footnote was wrong: it said the angle was the Value times
 * 180 degrees, when the Value is the hex reading and the angle comes from the
 * arc position -- 13/16 and 11/16 for k=13, near enough to look like the same
 * number and far enough apart to be a different one. This pins both down. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../atlas.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const fs = require("fs");

/* 1. the arc positions are exactly the dyadic rationals, once each.
      This is the page's headline claim -- "every dyadic, once" -- and it is
      the reason the angle is what it is. */
{
  const R = 9;
  const seen = new Set();
  for(let r = 0; r < R; r++)
    for(let s = 0; s < (1 << r); s++){
      const key = (2 * s + 1) + "/" + (1 << (r + 1));
      ok(!seen.has(key), `${key} appeared twice`);
      seen.add(key);
    }
  const want = new Set();
  for(let d = 1; d <= R; d++)
    for(let n = 1; n < (1 << d); n += 2) want.add(n + "/" + (1 << d));
  ok(seen.size === want.size, `${seen.size} arcs vs ${want.size} dyadics`);
  for(const w of want) ok(seen.has(w), `the atlas never places ${w}`);
  console.log(`  rings 0..${R - 1} place all ${want.size} dyadics n/2^d, odd n, exactly once`);
}

/* 2. the angle the panel prints is (arc - 1/2) x 180 from due east, and half a
      turn further for a negative. This is what the footnote now says, so if
      either the drawing or the wording drifts, this fails. */
{
  let checked = 0, worst = 0;
  for(let r = 0; r <= 7; r++)
    for(let s = 0; s < (1 << r); s++){
      for(const sgn of [1, -1]){
        const k = sgn * run(`kAtRing(${s}, ${r})`);
        const shown = run(`(((angleOf(${k})*180/Math.PI) % 360 + 360) % 360)`);
        const arc = (2 * s + 1) / (1 << (r + 1));
        const want = (((arc - 0.5) * 180 + (k < 0 ? 180 : 0)) % 360 + 360) % 360;
        worst = Math.max(worst, Math.abs(shown - want));
        checked++;
      }
    }
  ok(worst < 1e-9, `the angle drifted from the stated rule by ${worst} degrees`);
  console.log(`  angle = (arc - 1/2) x 180 from due east, exact over ${checked} tiles`);
}

/* 3. the arc and the value are genuinely different numbers, so the panel has
      to show both. If they were the same this test is the thing that says so. */
{
  const diff = [];
  for(let r = 0; r <= 5; r++)
    for(let s = 0; s < (1 << r); s++){
      const k = run(`kAtRing(${s}, ${r})`);
      const arc = (2 * s + 1) / (1 << (r + 1));
      const val = run(`dec(stalkFor(${k}).value)`);
      if(Math.abs(arc - val) > 1e-12) diff.push(k);
    }
  ok(diff.length > 0,
     "arc and value came out identical everywhere -- the panel need not show both");
  ok(diff.indexOf(13) >= 0, "k=13 should be one of the integers where they differ");
  console.log(`  arc and value differ for ${diff.length} integers, 13 among them`);
}

/* 4. a ring counts plainly upward, which is what the masthead claims */
{
  for(let r = 1; r <= 6; r++){
    let prev = -Infinity;
    for(let s = 0; s < (1 << r); s++){
      const k = run(`kAtRing(${s}, ${r})`);
      ok(k > prev, `ring ${r} did not count upward at slot ${s}: ${k} after ${prev}`);
      prev = k;
    }
  }
  console.log(`  every ring counts plainly upward around the arc, as the masthead says`);
}

/* 5. the radius crosses the number's own ancestry, and nothing else */
{
  for(const k of [13, 22, 200, 511]){
    const chain = run(`chainOf(${k})`);
    ok(chain[0] === k, `the chain should start at k, got ${chain[0]}`);
    for(let i = 1; i < chain.length; i++)
      ok(chain[i] === (chain[i - 1] >> 1),
         `chain step ${i} was ${chain[i]}, not ${chain[i - 1]} >> 1`);
    ok(chain[chain.length - 1] === 1, `the chain should end at 1, got ${chain[chain.length - 1]}`);
  }
  console.log(`  the radius crosses exactly k >> 1, k >> 2, ... down to 1`);
}

/* 6. the footnote in the file says what the code does. A label that has drifted
      from its own arithmetic is the bug this whole file exists for. */
{
  const html = fs.readFileSync(__dirname + "/../atlas.html", "utf8");
  /* the file spells some symbols as \uXXXX escapes and some as the characters
     themselves, so this checks what the footnote SAYS, not how it is spelled */
  const note = html.split("\n").find(L => L.indexOf("from due east") >= 0) || "";
  ok(note, "the angle footnote is gone entirely");
  ok(note.indexOf("arc") >= 0,
     `the footnote does not name the arc, which is what the angle is made of: ${note.trim()}`);
  ok(!/=\s*value/.test(note),
     `the footnote still calls the angle the Value times 180: ${note.trim()}`);
  ok(html.indexOf('["Arc"') >= 0, "the Arc card is gone, so the angle cannot be checked on screen");
  console.log(`  the printed footnote states the rule the code actually follows`);
}

console.log("atlas ok");
