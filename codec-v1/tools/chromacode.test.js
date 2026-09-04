/* node codec-v1/tools/chromacode.test.js -- the square as a product code.
 *
 * Each claim was predicted before it was run, so a pass carries information.
 * The one that matters most is #3: canonicity detects 0 of 50,000 sign flips,
 * and parity should detect all of them. That is the hole this code exists to
 * close.
 *
 * #7 is the honest control. Correction power is a property of the parity
 * geometry, not of the data, so random bytes and English text must give
 * identical rates. If they differ, something is wrong with the harness rather
 * than interesting about the data. */
const C = require(__dirname + "/../chromacode.js");
const fs = require("fs");

const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

/* a precision-safe PRNG. The obvious LCG loses its low bits above 2^53 and
   quietly stops being random -- that mistake already cost one bad measurement
   in this project, so it is not repeated here. */
function mul32(a){
  return function(){
    a |= 0; a = a + 0x6D2B79F5 | 0;
    let t = Math.imul(a ^ a >>> 15, 1 | a);
    t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
    return (t ^ t >>> 14) >>> 0;
  };
}
const g = mul32(20260831);
const randBytes = n => Uint8Array.from({length: n}, () => g() & 0xff);
const clone = p => ({meta: p.meta, parity: p.parity,
                     squares: p.squares.map(s => s.map(r => r.slice()))});
const same = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);

/* 1. round-trip is exact, both alphabets, including the awkward shapes */
{
  const cases = [
    ["empty", new Uint8Array(0)],
    ["one byte", Uint8Array.from([0xA7])],
    ["all zero", new Uint8Array(300)],
    ["all 0xFF", Uint8Array.from({length: 300}, () => 0xff)],
    ["partial final square", randBytes(1000)],
    ["exactly one square", randBytes(1024)],
    ["many squares", randBytes(5000)],
  ];
  for(const alphabet of ["byte", "chroma"])
    for(const [label, src] of cases){
      const p = C.encode(src, {N: 32, alphabet});
      const out = C.decode(clone(p));
      ok(same(Array.from(out.bytes), Array.from(src)),
        `${alphabet} round-trip broke on ${label}: ${out.bytes.length} vs ${src.length}`);
    }
  console.log(`  round-trip exact: ${cases.length} shapes x 2 alphabets`);
}

/* 2. one error is always found and always placed */
{
  for(const alphabet of ["byte", "chroma"]){
    const alph = C.ALPHABETS[alphabet];
    const src = randBytes(4096);
    let n = 0, fixed = 0;
    for(let t = 0; t < 2000; t++){
      const p = C.encode(src, {N: 32, alphabet});
      const s = g() % p.squares.length, r = g() % 32, c = g() % 32;
      const cur = p.squares[s][r][c];
      const alts = alph.others(cur);
      p.squares[s][r][c] = alts[g() % alts.length];
      const out = C.decode(p);
      n++;
      if(out.corrected === 1 && same(Array.from(out.bytes), Array.from(src))) fixed++;
    }
    ok(fixed === n, `${alphabet}: single error corrected ${fixed}/${n}`);
    console.log(`  ${alphabet}: single error located and repaired ${fixed}/${n}`);
  }
}

/* 3. sign flips -- the error canonicity cannot see at all.
      Measured on the canonical form: 0 of 50,000 detected. Here it must be
      all of them, because a flip moves its row sum by 2. */
{
  const src = randBytes(4096);
  let n = 0, caught = 0, repaired = 0, blindToCanon = 0;
  const atFix = d => pushLeftLocal(d).join() === d.join();
  /* stalk.js is not in scope here; the canonical test only needs the rule
     "no green may be followed by a lit cell", which is what push guarantees */
  function pushLeftLocal(cells){
    const d = cells.slice(); let moved = true;
    while(moved){ moved = false;
      for(let i = d.length - 1; i > 0; i--)
        if(d[i] !== 0 && d[i - 1] === 0){ d[i - 1] = d[i]; d[i] = -d[i]; moved = true; } }
    return d;
  }
  for(let t = 0; t < 5000; t++){
    const p = C.encode(src, {N: 32, alphabet: "chroma"});
    const s = g() % p.squares.length;
    /* pick a lit cell so there is a sign to flip */
    let r = -1, c = -1;
    for(let tries = 0; tries < 64; tries++){
      const rr = g() % 32, cc = g() % 32;
      if(p.squares[s][rr][cc] !== 0){ r = rr; c = cc; break; }
    }
    if(r < 0) continue;
    const row = p.squares[s][r].slice();
    p.squares[s][r][c] = -p.squares[s][r][c];
    n++;
    /* would the canonical form have noticed? */
    if(atFix(row) && atFix(p.squares[s][r])) blindToCanon++;
    const out = C.decode(p);
    if(out.corrected + out.detected > 0) caught++;
    if(same(Array.from(out.bytes), Array.from(src))) repaired++;
  }
  ok(caught === n, `sign flips caught ${caught}/${n}`);
  ok(repaired === n, `sign flips repaired ${repaired}/${n}`);
  console.log(`  sign flips: parity caught ${caught}/${n}, repaired ${repaired}/${n}`);
  console.log(`    (of those, ${blindToCanon} were invisible to canonicity -- the hole this closes)`);
}

/* 4. two errors: what the anti-diagonals actually buy.
      Row/column alone gives a 2x2 candidate set and can place none of it.
      The diagonals should resolve most pairs. */
{
  const src = randBytes(8192);
  const run = useDiags => {
    let n = 0, corrected = 0, detected = 0, silent = 0;
    for(let t = 0; t < 3000; t++){
      const p = C.encode(src, {N: 32, alphabet: "byte"});
      const alph = C.ALPHABETS.byte;
      const s = g() % p.squares.length;
      const r1 = g() % 32, r2 = g() % 32, c1 = g() % 32, c2 = g() % 32;
      if(r1 === r2 || c1 === c2) continue;
      for(const [r, c] of [[r1, c1], [r2, c2]]){
        const alts = alph.others(p.squares[s][r][c]);
        p.squares[s][r][c] = alts[g() % alts.length];
      }
      const out = C.decode(p, {useDiags});
      n++;
      const exact = same(Array.from(out.bytes), Array.from(src));
      if(exact) corrected++;
      else if(out.detected > 0) detected++;
      else silent++;
    }
    return {n, corrected, detected, silent};
  };
  const withD = run(true), without = run(false);
  console.log(`  two errors, row+col only : corrected ${without.corrected}/${without.n}`
    + `  detected ${without.detected}  silent ${without.silent}`);
  console.log(`  two errors, +anti-diagonal: corrected ${withD.corrected}/${withD.n}`
    + `  detected ${withD.detected}  silent ${withD.silent}`);
  ok(withD.corrected > without.corrected,
    `diagonals did not improve 2-error correction: ${withD.corrected} vs ${without.corrected}`);
  ok(withD.silent === 0, `${withD.silent} silently wrong answers with diagonals on`);
  console.log(`    diagonals bought ${withD.corrected - without.corrected} more repairs, 0 silent`);
}

/* 5. the 4-corner rectangle -- plain row/column parity's total blind spot.
      Predicted: 0% detected without diagonals; detected with them unless
      r1-r2 == c1-c2 makes the middle two diagonals coincide and cancel. */
{
  const src = randBytes(8192);
  const run = useDiags => {
    let n = 0, seen = 0, silent = 0, degenerate = 0;
    for(let t = 0; t < 3000; t++){
      const p = C.encode(src, {N: 32, alphabet: "byte"});
      const s = g() % p.squares.length;
      const r1 = g() % 32, r2 = g() % 32, c1 = g() % 32, c2 = g() % 32;
      if(r1 === r2 || c1 === c2) continue;
      /* XOR the same delta into all four corners: every row and column sum is
         left unchanged by construction */
      const d = 1 + (g() % 255);
      for(const [r, c] of [[r1, c1], [r1, c2], [r2, c1], [r2, c2]])
        p.squares[s][r][c] ^= d;
      n++;
      if(r1 - r2 === c1 - c2 || r1 - r2 === c2 - c1) degenerate++;
      const out = C.decode(p, {useDiags});
      if(out.detected + out.corrected > 0) seen++;
      else if(!same(Array.from(out.bytes), Array.from(src))) silent++;
    }
    return {n, seen, silent, degenerate};
  };
  const withD = run(true), without = run(false);
  ok(without.seen === 0,
    `row/col should be totally blind to rectangles, saw ${without.seen}/${without.n}`);
  console.log(`  4-corner rectangle, row+col only : detected ${without.seen}/${without.n}`
    + ` (blind by construction), silently wrong ${without.silent}`);
  console.log(`  4-corner rectangle, +anti-diagonal: detected ${withD.seen}/${withD.n}`
    + ` (${(100 * withD.seen / withD.n).toFixed(1)}%), silently wrong ${withD.silent}`);
  console.log(`    the surviving family r1-r2 = +-(c1-c2) was ${withD.degenerate}/${withD.n}`
    + ` of the sample`);
}

/* 6. overhead is (4N-1)/N^2, reported from the format rather than asserted */
{
  console.log(`  overhead by N, from sizes():`);
  for(const N of [6, 12, 16, 32, 64]){
    const p = C.encode(randBytes(N * N * 4), {N, alphabet: "byte"});
    const s = C.sizes(p.meta);
    const want = (4 * N - 1) / (N * N);
    ok(Math.abs(s.parityOverhead - want) < 1e-12, `N=${N} overhead ${s.parityOverhead} != ${want}`);
    console.log(`    N=${String(N).padStart(2)}  ${(100 * want).toFixed(1).padStart(5)}%`
      + `   ${4 * N - 1} sums for ${N * N} cells`);
  }
}

/* 7. head-to-head, and the control: correction is data-independent */
{
  const N = 32;
  console.log(`  chroma vs byte at N=${N}, same 4096-byte source:`);
  for(const alphabet of ["byte", "chroma"]){
    const p = C.encode(randBytes(4096), {N, alphabet});
    const s = C.sizes(p.meta);
    console.log(`    ${alphabet.padEnd(7)} data ${String(s.dataBytes).padStart(5)}B`
      + `  parity ${String(s.parityBytes).padStart(4)}B`
      + `  total ${String(s.totalBytes).padStart(5)}B`
      + `  ${s.ratio.toFixed(2)}x source`);
  }

  /* the control. Same error model over two very different sources. */
  const text = fs.readFileSync(__dirname + "/../../spec.md");
  const corpora = {"random bytes": randBytes(text.length), "spec.md text": text};
  for(const [label, src] of Object.entries(corpora)){
    let n = 0, fixed = 0;
    for(let t = 0; t < 800; t++){
      const p = C.encode(src, {N, alphabet: "byte"});
      const s = g() % p.squares.length, r = g() % N, c = g() % N;
      p.squares[s][r][c] ^= 1 + (g() % 255);
      const out = C.decode(p);
      n++; if(same(Array.from(out.bytes), Array.from(src))) fixed++;
    }
    console.log(`    ${label.padEnd(13)} single-error repair ${fixed}/${n}`);
    ok(fixed === n, `${label} repaired only ${fixed}/${n}`);
  }
  console.log(`    identical rates confirm correction is geometric, not data-dependent`);
}

/* 8. the page's own classify(), driven directly.
      tools/run.js only proves the page's scripts did not throw. This proves the
      function the page makes decisions with agrees with the codec, including
      the four-corner case the page exists to demonstrate. */
{
  const {loadPage} = require(__dirname + "/../../tools/domharness.js");
  const {run} = loadPage(__dirname + "/../chromacode.html");

  /* prepare runs BEFORE the expected parity is taken, mutate runs after -- so
     the grid can be put in a known state without that itself counting as
     damage. build() is random, and a rectangle needs room to move both ways. */
  const status = (n, prepare, mutate, useDiags) => run(`(() => {
    N = ${n}; build();
    const g = grid;
    (${prepare})(g, ${n});
    const want0 = parities(g, ${n});
    (${mutate})(g, ${n});
    return classify(g, ${n}, want0, ${useDiags}).status;
  })()`);

  const asIs = "(g,n)=>{}";
  const allGreen = "(g,n)=>{ for(let r=0;r<n;r++) for(let c=0;c<n;c++) g[r][c]=0; }";

  ok(status(8, asIs, asIs, true) === "clean", "untouched grid was not clean");
  console.log(`  page classify: untouched grid reads clean`);

  const one = "(g,n)=>{ g[3][5] = [1,0,-1].filter(v=>v!==g[3][5])[0]; }";
  ok(status(8, asIs, one, true) === "located", "page failed to locate a single error");
  console.log(`  page classify: single error reads located`);

  /* four corners with deltas +1 -1 -1 +1, which is what actually leaves every
     row and column sum unchanged under an integer sum. Applying the SAME step
     to all four would cancel under XOR but not here -- the page had that bug
     until this test caught it. */
  const rect = "(g,n)=>{ g[1][2]+=1; g[4][6]+=1; g[1][6]-=1; g[4][2]-=1; }";
  ok(status(8, allGreen, rect, false) === "clean",
    "row/col parity should be blind to a rectangle in the page too");
  ok(status(8, allGreen, rect, true) === "hidden",
    "page should report a rectangle as hidden-but-detected once diagonals are on");
  console.log(`  page classify: rectangle invisible without diagonals, detected with them`);
}

console.log("chromacode ok");
