/* node tools/frac.test.js — exact arithmetic on what a card holds.
 *
 * Cards take expressions: 47*127, (13*3*127/2^4), (1/3). Everything is exact
 * rationals in BigInt. A library like math.js would evaluate these to doubles,
 * and this whole system's claim is that the value is exact — so the parser is
 * forty lines here rather than a dependency that hands back 0.333.
 */
const fs = require("fs");
eval(fs.readFileSync(__dirname + "/../stalk.js", "utf8"));
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };
const F = (n, d) => n + "/" + d;

/* ---- 1. it evaluates, exactly ---- */
{
  const CASES = [["3", 3, 1], ["47*127", 5969, 1], ["(13*3*127/2^4)", 4953, 16],
                 ["(3/93)", 1, 31], ["(1/3)", 1, 3], ["(2/3)", 2, 3],
                 ["1/2+1/4", 3, 4], ["2^10", 1024, 1], ["-3", -3, 1],
                 ["(1+1/2)*4", 6, 1], ["2^-3", 1, 8], ["7-9", -2, 1]];
  const bad = CASES.filter(([s, n, d]) => {
    const f = evalFrac(s);
    return f.num !== BigInt(n) || f.den !== BigInt(d);
  });
  ok(!bad.length, "wrong: " + bad.map(c => c[0] + " -> "
    + F(evalFrac(c[0]).num, evalFrac(c[0]).den)).join(", "));
  console.log(`  [1] exact          ${CASES.length} expressions, all in lowest terms`);
  console.log(`      (13*3*127/2^4) = ${F(evalFrac("(13*3*127/2^4)").num,
    evalFrac("(13*3*127/2^4)").den)}, 47*127 = ${evalFrac("47*127").num}`);
}

/* ---- 2. never floats ---- */
{
  /* 1/3 + 1/3 + 1/3 is 1 exactly. In doubles it is not. */
  const t = evalFrac("1/3+1/3+1/3");
  ok(t.num === 1n && t.den === 1n, `1/3 three times should be 1, got ${F(t.num, t.den)}`);
  ok(0.1 + 0.2 !== 0.3, "the float control is dead");
  const p = evalFrac("1/10+2/10");
  ok(p.num === 3n && p.den === 10n, "a tenth plus two tenths should be exactly 3/10");
  console.log("  [2] not floats     1/3+1/3+1/3 = 1/1 exactly, and 1/10+2/10 = 3/10");
  console.log("                     while 0.1 + 0.2 !== 0.3 in the doubles beside it");
}

/* ---- 3. bad input is refused, not guessed at ---- */
{
  const BAD = ["", "(3", "3)", "3//2", "1/0", "2^(1/2)", "3 4 five", "*3"];
  const slipped = BAD.filter(s => { try { evalFrac(s); return true; }
    catch(e){ return false; } });
  ok(!slipped.length, "accepted nonsense: " + slipped.join(", "));
  console.log(`  [3] refuses        ${BAD.length} malformed inputs all throw, `
    + "including 1/0 and a non-whole exponent");
}

/* ---- 4. dyadic lands exactly; nothing else can ----
 * A stalk's cells weigh 2^-(i+1), so only a power-of-two denominator can end.
 * 1/3 is 0.010101... for ever, which is the same fact Wub / reports when only
 * 430 of its 2178 quotients came out exact.
 */
{
  const dy = s => isDyadic(evalFrac(s));
  ok(dy("3") && dy("1/2+1/4") && dy("(13*3*127/2^4)"), "these are dyadic");
  ok(!dy("(1/3)") && !dy("(2/3)") && !dy("(3/93)"), "these are not");
  for(const s of ["3", "1/2+1/4", "(13*3*127/2^4)", "2^10", "-3"]){
    const f = evalFrac(s), st = fracStalk(f, 24), back = stalkFrac(st.d, st.E);
    ok(st.exact, `${s} is dyadic and should land exactly`);
    ok(back.num * f.den === f.num * back.den,
       `${s} read back as ${F(back.num, back.den)}, wanted ${F(f.num, f.den)}`);
  }
  console.log("  [4] dyadic exact   every dyadic reads back as itself through the stalk");
}

/* ---- 5. a non-dyadic is cut, and says how much it dropped ---- */
{
  const third = fracStalk(evalFrac("1/3"), 16);
  ok(!third.exact, "1/3 cannot be exact");
  ok(third.d.join("") === "0101010101010101", "1/3 should be 0101... : " + third.d.join(""));
  ok(fracStalk(evalFrac("2/3"), 16).d.join("") === "1010101010101010", "2/3 should be 1010...");
  /* the cut plus what it dropped is the original, exactly */
  for(const s of ["1/3", "2/3", "3/93", "1/31"]){
    const f = evalFrac(s), W = 20, st = fracStalk(f, W);
    const back = stalkFrac(st.d, st.E);
    const dropped = fSub(f, back);
    ok(dropped.num >= 0n, `${s}: the cut should never overshoot`);
    ok(dropped.num !== 0n, `${s} should have dropped something`);
    /* and widening the cut can only shrink what was dropped */
    const wider = stalkFrac(fracStalk(f, W + 8).d, fracStalk(f, W + 8).E);
    const d2 = fSub(f, wider);
    ok(d2.num * dropped.den <= dropped.num * d2.den,
       `${s}: a wider cut dropped more, which cannot be`);
  }
  console.log("  [5] non-dyadic     1/3 is 0101... and 2/3 is 1010...; the cut plus");
  console.log("                     what it dropped is the original, and widening only shrinks it");
}

/* ---- 6. a card is a comma-separated list, and each item stands alone ---- */
{
  const items = "3, 5, 7".split(",").map(s => evalFrac(s));
  ok(items.length === 3 && items.every((f, i) => f.num === BigInt(3 + 2 * i)),
     "3, 5, 7 should be three items");
  const mixed = "(1/3), 47*127, -3".split(",").map(s => evalFrac(s));
  ok(mixed.length === 3 && mixed[1].num === 5969n && mixed[2].num === -3n,
     "a card should take expressions and signs side by side");
  console.log("  [6] a card is a list  3, 5, 7 and (1/3), 47*127, -3 both parse item by item");
}

console.log("\n  certified.");
