// minimal DOM so we can RUN a page, not just parse it
function harness(){
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
    getElementById(id){ if(!els.has(id)) els.set(id, mkEl(id)); return els.get(id); },
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
module.exports = {harness};
