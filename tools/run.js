const {loadPage} = require(__dirname + "/domharness.js");
const page = process.argv[2];
let ctx;
try{ ({ctx} = loadPage(page)); }
catch(e){ console.log("  THROW:", e.message); process.exit(1); }
const ran = [];
for(const fn of ["draw", "refresh", "select", "drawField"]){
  if(typeof ctx[fn] === "function"){
    try{ ctx[fn](fn === "select" ? 3 : undefined); ran.push(fn + "()"); }
    catch(e){ ran.push(fn + "() THREW: " + e.message); }
  }
}
console.log("  " + (ran.join("  ") || "loaded, no entry point"));
