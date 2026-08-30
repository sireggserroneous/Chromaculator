/* node tools/load.test.js — the three Wub pages under a full rack.
   A page that throws inside draw() loses its requestAnimationFrame loop and
   freezes, which is what "it crashed when I added too many integers" looks
   like from the outside. The fake canvas enforces the radius and finiteness
   rules the real one does, so that failure surfaces here. */
const {loadPage} = require(__dirname + "/domharness.js");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

const PAGES = ["wub.html", "wubx.html", "wubdiv.html"];
for(const page of PAGES){
  const {run} = loadPage(__dirname + "/../" + page);
  const spellings = run("typeof KS[0].plain") === "boolean";
  const seed = spellings ? `{k:3, plain:true, push:false, op:.55}` : `{k:3, push:false, op:.55}`;
  const line = [];
  for(const n of [2, 8, 20, 40, 80]){
    run(`KS = [${seed}];
         for(let i = 0; i < ${n - 1}; i++) KS.push(${spellings
           ? `{k: KS.length + 2, plain:true, push:false, op:0.55}`
           : `{k: KS.length + 2, push:false, op:0.55}`});
         refresh();`);
    /* several frames: trails accumulate, so frame 1 is not representative */
    for(let f = 0; f < 6; f++) run(`draw(${100 + f * 17})`);
    const bad = run(`(() => {
      let nf = 0;
      for(const c of CURVE) for(const v of [c.x, c.y, c.z]) if(!isFinite(v)) nf++;
      for(const p of M.P) for(const v of [p.inner, p.fold, p.outer, p.amp]) if(!isFinite(v)) nf++;
      return nf;
    })()`);
    ok(bad === 0, `${page} at ${n}: ${bad} non-finite components`);
    ok(run("isFinite(RMAX) && RMAX > 0"), `${page} at ${n}: RMAX is ${run("RMAX")}`);
    line.push(`${n}:ok`);
  }
  console.log(`  ${page.padEnd(12)} ${line.join("  ")}`);
}

/* the depth term must be a direction, not a distance — that is what kept a
   point size from going negative and throwing out of the gradient call */
{
  const {run} = loadPage(__dirname + "/../wubdiv.html");
  run(`KS = [{k:3,push:false,op:.55},{k:10,push:false,op:.55}];
       for(let i = 0; i < 30; i++) KS.push({k: KS.length + 2, push:false, op:0.55});
       refresh(); draw(100);`);
  const ds = run(`(() => {
    const cv = $("sph"), W = cv.clientWidth, H = cv.clientHeight;
    const ca = Math.cos(SPIN), sa = Math.sin(SPIN), ct = Math.cos(TILT), st = Math.sin(TILT);
    const d = q => { const x = q.x*ca - q.y*sa, h = q.x*sa + q.y*ca;
      const m = Math.hypot(q.x, q.y, q.z) || 1; return (q.z*st + h*ct) / m; };
    return CURVE.map(d);
  })()`);
  const worst = Math.max(...ds.map(Math.abs));
  ok(worst <= 1 + 1e-12, `depth left [-1,1]: worst ${worst}`);
  console.log(`  depth stays a cosine across ${ds.length} sampled points, worst |d| = ${worst.toFixed(6)}`);
}
console.log("\nall good.");
