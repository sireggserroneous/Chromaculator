const fs=require("fs"), vm=require("vm"), path=require("path");
const {harness}=require(__dirname+"/domharness.js");
const page=process.argv[2];
const html=fs.readFileSync(page,"utf8");
const {g,els}=harness();
const ctx=vm.createContext(g);
let src="";
for(const m of html.matchAll(/<script[^>]*\bsrc="([^"]+)"/g)){
  const f=path.join(path.dirname(page), m[1].replace(/^\//,""));
  if(fs.existsSync(f)) src+=fs.readFileSync(f,"utf8")+"\n";
}
for(const m of html.matchAll(/<script(?![^>]*\bsrc=)[^>]*>([\s\S]*?)<\/script>/g)) src+=m[1]+"\n";
try{ vm.runInContext(src,ctx,{filename:page}); }
catch(e){ console.log("  THROW:",e.message); process.exit(1); }
const ran=[];
for(const fn of ["draw","refresh","select","drawField"]){
  if(typeof ctx[fn]==="function"){
    try{ ctx[fn](fn==="select"?3:undefined); ran.push(fn+"()"); }
    catch(e){ ran.push(fn+"() THREW: "+e.message); }
  }
}
console.log("  " + (ran.join("  ") || "loaded, no entry point"));
