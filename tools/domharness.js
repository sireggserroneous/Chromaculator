// minimal DOM so we can RUN a page, not just parse it
//
// harness(html) reads the ids the page actually declares, and returns null for
// any other getElementById. That matters: a stub for every id you ask for turns
// a genuine "this element does not exist" bug into a silent pass, which is
// exactly how a missing <span id="spacing"> got a page shipped with a dead
// render loop. Call it with no html only if you do not care.
function harness(html){
  const known = html == null ? null
    : new Set([...String(html).matchAll(/\bid="([^"]+)"/g)].map(m => m[1]));
  const grad = () => ({addColorStop(){}});
  const ctx = new Proxy({measureText:()=>({width:10}), canvas:{width:300,height:300},
      createRadialGradient:grad, createLinearGradient:grad, createPattern:()=>null,
      getImageData:()=>({data:new Uint8ClampedArray(4)})},
    {get:(t,k)=> k in t ? t[k] : ()=>{} , set:()=>true});
  const els = new Map();
  const mkEl = id => {
    const el = {
      id, innerHTML:"", textContent:"", value:"3", className:"", checked:false, disabled:false,
      style:{_v:{}, setProperty(k,v){this._v[k]=v}, getPropertyValue(k){return this._v[k]||""},
             removeProperty(k){delete this._v[k]}},
      width:300, height:300, clientWidth:300, clientHeight:300,
      classList:{add(){},remove(){},toggle(){},contains(){return false}},
      children:[], dataset:{}, files:[],
      getContext:()=>ctx, addEventListener(){}, removeEventListener(){},
      setPointerCapture(){}, focus(){}, appendChild(c){this.children.push(c)},
      showModal(){this._open=true}, close(){this._open=false}, ownerDocument:null,
      getBoundingClientRect:()=>({left:0,top:0,width:300,height:300}),
      closest(){return null}, querySelectorAll(){return []},
      get parentElement(){ return mkEl(id+":parent"); },
    };
    return el;
  };
  const doc = {
    getElementById(id){
      if(known && !known.has(id)) return null;      // the page does not have it
      if(!els.has(id)) els.set(id, mkEl(id));
      return els.get(id);
    },
    createElement:()=>mkEl("new"), body:mkEl("body"), documentElement:mkEl("html"),
    addEventListener(){}, styleSheets:[],
  };
  let frames = 0;
  const g = {
    document: doc, devicePixelRatio: 2,
    getComputedStyle:()=>({getPropertyValue:()=>"#2558C6"}),
    matchMedia:()=>({matches:false, addEventListener(){}, addListener(){}}),
    requestAnimationFrame:(fn)=>{ if(frames++ < 4) fn(0); return frames; },
    addEventListener(){}, removeEventListener(){}, innerHeight:900, innerWidth:1400,
    console, Math, JSON, BigInt, Number, String, Array, Object, Set, Map, parseInt, parseFloat,
    isNaN, Int8Array, Uint8ClampedArray, Uint16Array, Float32Array, Proxy, Infinity, NaN, undefined,
    Date, RegExp, Error, Promise, Symbol,
    setTimeout:(f)=>{ return 1; }, clearTimeout(){}, FileReader: function(){},
    Blob: function(){}, URL:{createObjectURL:()=>"blob:x", revokeObjectURL(){}},
    navigator:{clipboard:{writeText:()=>Promise.resolve()}},
    localStorage:{_v:{}, getItem(k){return this._v[k]||null}, setItem(k,v){this._v[k]=v}},
  };
  g.window = g; g.globalThis = g; g.self = g;
  return {g, els, frames:()=>frames};
}
/* load a page's scripts into a fresh context and hand back a runner. every
   caller wants exactly this, so it lives here rather than in four copies. */
function loadPage(pagePath){
  const fs = require("fs"), vm = require("vm"), path = require("path");
  const html = fs.readFileSync(pagePath, "utf8");
  const h = harness(html);
  const ctx = vm.createContext(h.g);
  let src = "";
  for(const m of html.matchAll(/<script[^>]*\bsrc="([^"]+)"/g)){
    const f = path.join(path.dirname(pagePath), m[1].replace(/^\//, ""));
    if(fs.existsSync(f)) src += fs.readFileSync(f, "utf8") + "\n";
  }
  for(const m of html.matchAll(/<script(?![^>]*\bsrc=)[^>]*>([\s\S]*?)<\/script>/g)) src += m[1] + "\n";
  vm.runInContext(src, ctx, {filename: path.basename(pagePath)});
  return {...h, ctx, html, run: code => vm.runInContext(code, ctx)};
}
module.exports = {harness, loadPage};
