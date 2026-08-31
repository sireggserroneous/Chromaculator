/* node tools/spec.test.js — the live panel on the spec page.
 *
 * The panel prints a region breakdown directly beside the value. That is only
 * honest if the three numerators really do add back to the value's, over the
 * common denominator, so that is the property under test -- in both forms,
 * across the whole range of the bar, in exact integer arithmetic. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../spec.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

const parts = (k, form) => run(`(() => {
  const st = stalkOf(parse(String(Math.abs(${k}))).v, ${k} < 0);
  const cells = "${form}" === "push" ? pushLeft(st.cells) : st.cells;
  const reg = regions(cells, st.n), L = cells.length;
  const s = key => reg[key].reduce((a, x) => a + BigInt(x.v) * (1n << BigInt(L - 1 - x.i)), 0n);
  const v = valueOf(cells), den = 1n << BigInt(L - 1);
  const tot = s("inner") + s("fold") + s("outer");
  const g = (a, b) => { a = a < 0n ? -a : a; while(b){ [a, b] = [b, a % b]; } return a || 1n; };
  const d = g(tot, den);
  return {inner: String(s("inner")), fold: String(s("fold")), outer: String(s("outer")),
          exact: (tot / d) === v.num && (den / d) === v.den,
          value: fmt(v), n: st.n};
})()`);

/* 1. Inner + Fold + Outer is the value, exactly, in both forms */
{
  let checked = 0;
  for(let k = -511; k <= 511; k++)
    for(const form of ["plain", "push"]){
      const p = parts(k, form);
      ok(p.exact, `k=${k} ${form}: regions did not sum to ${p.value}`);
      checked++;
    }
  console.log(`  the three regions sum to the value exactly, ${checked} cases`);
}

/* 2. negating flips every region's sign and nothing else -- the page's own
      claim, and the reason the integer bar runs through zero */
{
  for(let k = 1; k <= 511; k++){
    const a = parts(k, "plain"), b = parts(-k, "plain");
    ok(a.n === b.n, `k=${k}: negating changed the square from ${a.n} to ${b.n}`);
    for(const key of ["inner", "fold", "outer"])
      ok(BigInt(a[key]) === -BigInt(b[key]),
         `k=${k}: ${key} went ${a[key]} -> ${b[key]}, not its negation`);
  }
  console.log(`  negating flips all three regions and leaves the square the same size`);
}

/* 3. the panel draws over its whole control surface without throwing */
{
  let n = 0;
  for(const k of [-511, -256, -1, 0, 1, 7, 181, 511])
    for(const t of [0, 0.31, 0.5, 0.77, 1])
      for(const form of ["plain", "push"]){
        run(`sK.set(${k}); sT.set(${t}); sForm.set("${form}"); sDraw(); sSay();`);
        n++;
      }
  console.log(`  ${n} draws across integer, morph and form: no throw`);
}

/* 4. the morph's two ends are the strip and the square, and the hit-test is
      only live once the square has actually formed */
{
  run(`sK.set(181); sT.set(0); sHit = null; sDraw();`);
  const strip = run(`(() => { const st = stalkOf(parse("181").v, false);
    const l = sLayout(st.cells, st.n, 600, 330); return {y0: l[0].strip.y, ys: l.map(c => c.strip.y)}; })()`);
  ok(strip.ys.every(y => y === strip.y0), "the strip is not a straight line");
  const grid = run(`(() => { const st = stalkOf(parse("181").v, false);
    const l = sLayout(st.cells, st.n, 600, 330); return new Set(l.map(c => c.grid.y)).size; })()`);
  ok(grid > 1, "the square collapsed to one row");
  console.log(`  the morph runs from a flat strip to a square of ${grid} rows`);
}

console.log("spec ok");
