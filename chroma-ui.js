/* chroma-ui.js — the interaction layer: sliders, dropdowns, scrubbers, drag.
 *
 * Loaded after stalk.js, which it leans on for geometry: the cell hit-test
 * inverts stalk.js's own cellOrder() rather than re-deriving the anti-diagonal
 * walk, so the two can never disagree about where a cell is. Everything here
 * hangs off one UI object, because stalk.js and every page's inline script
 * share a single global scope -- a bare `const slider` here would collide and
 * throw before a line of the page ran.
 *
 * Two rules the DOM harness in tools/ imposes, and which shape this file:
 *
 *   getElementById returns null for an id the page does not declare. Nine
 *   pages want different subsets of these controls, so every binder returns an
 *   inert handle of the same shape when its element is missing. Callers wire
 *   controls unconditionally and never branch on existence.
 *
 *   querySelector is absent and addEventListener is a stub, so nothing reached
 *   only through a listener is exercised by a test. The arithmetic therefore
 *   lives in pure functions -- cellAt, indexOfCell, regionOf, phase -- which
 *   tools/ui.test.js calls directly. The binders are the thin part on purpose.
 */
const UI = (function(){
  "use strict";

  /* ---- geometry, all pure ------------------------------------------------ */

  /* cellOrder(n) is the source of truth for where the i-th digit of a stalk
     lands in the square. Inverting it per n and caching costs one small walk
     and keeps this file honest: if the fold's order ever changes, the hit-test
     changes with it and nothing here needs editing. */
  const orderCache = new Map();
  function orderOf(n){
    if(!orderCache.has(n)){
      if(typeof cellOrder !== "function")
        throw new Error("chroma-ui.js needs stalk.js loaded first");
      const o = cellOrder(n), idx = new Map();
      o.forEach(([r, c], i) => idx.set(r * n + c, i));
      orderCache.set(n, {order: o, idx});
    }
    return orderCache.get(n);
  }

  /* which stalk position sits at (r,c), or -1 if that cell is off the square */
  function indexOfCell(r, c, n){
    if(!(r >= 0 && r < n && c >= 0 && c < n)) return -1;
    const hit = orderOf(n).idx.get(r * n + c);
    return hit === undefined ? -1 : hit;
  }

  /* the fold's three regions, by side of the main anti-diagonal. Matches
     regions() in stalk.js; kept here so a hit-test can name a region without
     building every slot. */
  function regionOf(r, c, n){
    const s = r + c;
    return s < n - 1 ? "inner" : s === n - 1 ? "fold" : "outer";
  }

  /* Point -> cell, for a square of `size` css px whose top-left is the origin.
     Returns null outside the square rather than clamping: a drag that leaves
     the grid should stop reporting cells, not smear along the last edge. */
  function cellAt(px, py, size, n){
    if(!(size > 0) || !(n > 0)) return null;
    if(px < 0 || py < 0 || px >= size || py >= size) return null;
    const step = size / n;
    const r = Math.floor(py / step), c = Math.floor(px / step);
    if(r >= n || c >= n) return null;                 // guards fp at the edge
    const i = indexOfCell(r, c, n);
    return i < 0 ? null : {r, c, i, region: regionOf(r, c, n)};
  }

  /* t in [0,1] -> how many of `total` steps have landed, and how far into the
     next one. Scrubbers everywhere need exactly this and get it subtly wrong
     at t=1, where floor() would run one past the end. */
  function phase(t, total){
    const u = t <= 0 ? 0 : t >= 1 ? 1 : t;
    const x = u * total;
    const done = u >= 1 ? total : Math.floor(x);
    return {done, frac: u >= 1 ? 1 : x - done};
  }

  const clamp = (v, lo, hi) => v < lo ? lo : v > hi ? hi : v;

  /* ---- binders ----------------------------------------------------------- */

  const el = id => (typeof document === "undefined" ? null : document.getElementById(id));

  /* every binder returns this shape, so a page can wire a control it does not
     have and simply get a handle whose get() is a constant. */
  const inert = value => ({el: null, ok: false, get: () => value, set(){}, on(){}, sync(){}});

  function listen(node, ev, fn){
    if(node && typeof node.addEventListener === "function") node.addEventListener(ev, fn);
  }

  /* A range input plus an optional readout element. `fmt` renders the readout;
     `onInput` fires on every move. Reading `.value` off a range gives a string,
     so get() is always a number and callers never parseFloat by hand. */
  function slider(id, opts){
    opts = opts || {};
    const node = el(id);
    if(!node) return inert(opts.value === undefined ? 0 : opts.value);
    const out = opts.out ? el(opts.out) : null;
    const fmt = opts.fmt || (v => String(v));
    const get = () => {
      const v = parseFloat(node.value);
      return isNaN(v) ? (opts.value || 0) : v;
    };
    const paint = () => { if(out) out.textContent = fmt(get()); };
    const handle = {
      el: node, ok: true, get,
      set(v){ node.value = String(v); paint(); },
      on(fn){ listen(node, "input", () => { paint(); fn(get()); }); },
      sync: paint,
    };
    if(opts.value !== undefined) handle.set(opts.value);
    if(opts.onInput) handle.on(opts.onInput);
    else paint();
    return handle;
  }

  /* A <select>. Same shape; get() returns the string value. */
  function select(id, opts){
    opts = opts || {};
    const node = el(id);
    if(!node) return inert(opts.value === undefined ? "" : opts.value);
    const get = () => node.value;
    const handle = {
      el: node, ok: true, get,
      set(v){ node.value = v; },
      on(fn){ listen(node, "change", () => fn(get())); },
      sync(){},
    };
    if(opts.value !== undefined) handle.set(opts.value);
    if(opts.onChange) handle.on(opts.onChange);
    return handle;
  }

  /* A checkbox. get() returns a boolean. */
  function toggle(id, opts){
    opts = opts || {};
    const node = el(id);
    if(!node) return inert(!!opts.value);
    const get = () => !!node.checked;
    const handle = {
      el: node, ok: true, get,
      set(v){ node.checked = !!v; },
      on(fn){ listen(node, "change", () => fn(get())); },
      sync(){},
    };
    if(opts.value !== undefined) handle.set(opts.value);
    if(opts.onChange) handle.on(opts.onChange);
    return handle;
  }

  /* A scrubber: a range over t in [0,1], a play/pause button, and a rAF loop
     that advances t and loops. onTick(t) fires for both a drag and a frame,
     so a caller draws one way and does not care which moved it.

     Autoplay defers to prefers-reduced-motion -- the loop still exists and the
     button still starts it, but a page does not begin moving on its own for a
     reader who asked it not to. */
  function scrub(id, opts){
    opts = opts || {};
    const node = el(id);
    const btn = opts.playId ? el(opts.playId) : null;
    const period = opts.period || 6000;               // ms for a full pass
    const tick = opts.onTick || function(){};
    let t = opts.value === undefined ? 0 : opts.value;
    let playing = false, last = 0, raf = 0;

    const calm = () => {
      try { return typeof matchMedia === "function"
        && matchMedia("(prefers-reduced-motion: reduce)").matches; }
      catch(e){ return false; }
    };
    const paint = () => {
      if(node) node.value = String(t);
      if(btn) btn.textContent = playing ? "‖ pause" : "▸ play";
      if(btn) btn.setAttribute("aria-pressed", playing ? "true" : "false");
    };
    const frame = now => {
      if(!playing) return;
      if(last){
        t += (now - last) / period;
        while(t > 1) t -= 1;                          // loop, never stall at 1
      }
      last = now;
      paint(); tick(t);
      raf = requestAnimationFrame(frame);
    };
    const handle = {
      el: node, ok: !!node,
      get: () => t,
      set(v){ t = clamp(v, 0, 1); paint(); tick(t); },
      on(fn){ opts.onTick = fn; },
      sync: paint,
      get playing(){ return playing; },
      play(){
        if(playing) return;
        playing = true; last = 0; paint();
        if(typeof requestAnimationFrame === "function") raf = requestAnimationFrame(frame);
      },
      stop(){
        playing = false; last = 0; paint();
        if(raf && typeof cancelAnimationFrame === "function") cancelAnimationFrame(raf);
        raf = 0;
      },
      toggle(){ playing ? handle.stop() : handle.play(); },
    };
    listen(node, "input", () => {
      handle.stop();                                  // grabbing the bar takes over
      const v = parseFloat(node.value);
      t = isNaN(v) ? 0 : clamp(v, 0, 1);
      tick(t);
    });
    listen(btn, "click", () => handle.toggle());
    paint();
    if(opts.autoplay && !calm()) handle.play();
    return handle;
  }

  /* Pointer drag over a canvas. onMove gets css-pixel coords relative to the
     element's top-left, plus normalised -1..1, plus which button-phase it is.

     Coordinates come from getBoundingClientRect, not from the canvas's width
     attribute: those differ whenever the backing store is scaled for dpr, and
     using the wrong one puts every hit at half the cursor's distance from the
     origin on a retina screen. */
  function drag(id, opts){
    opts = opts || {};
    const node = el(id);
    if(!node) return {el: null, ok: false, stop(){}};
    const at = ev => {
      const r = node.getBoundingClientRect();
      const x = (ev.clientX || 0) - r.left, y = (ev.clientY || 0) - r.top;
      const w = r.width || 1, h = r.height || 1;
      return {x, y, w, h, nx: (x / w) * 2 - 1, ny: (y / h) * 2 - 1,
              /* the modifiers travel with the point: a caller that wants
                 shift-to-reverse should not have to keep its own key state */
              shift: !!ev.shiftKey, alt: !!ev.altKey, meta: !!(ev.metaKey || ev.ctrlKey),
              inside: x >= 0 && y >= 0 && x < w && y < h};
    };
    let down = false;
    const move = opts.onMove || function(){};
    const start = ev => {
      down = true;
      if(node.setPointerCapture && ev.pointerId !== undefined){
        try { node.setPointerCapture(ev.pointerId); } catch(e){}
      }
      if(ev.preventDefault) ev.preventDefault();
      const p = at(ev); p.phase = "down";
      (opts.onDown || move)(p);
    };
    const go = ev => {
      const p = at(ev);
      p.phase = down ? "drag" : "hover";
      if(down && ev.preventDefault) ev.preventDefault();
      if(down || opts.hover) move(p);
    };
    const end = ev => {
      if(!down) return;
      down = false;
      const p = at(ev); p.phase = "up";
      (opts.onUp || move)(p);
    };
    listen(node, "pointerdown", start);
    listen(node, "pointermove", go);
    listen(node, "pointerup", end);
    listen(node, "pointercancel", end);
    listen(node, "pointerleave", ev => { if(!down && opts.hover) move({...at(ev), phase: "leave", inside: false}); });
    return {el: node, ok: true, at, get dragging(){ return down; },
            stop(){ down = false; }};
  }

  /* Size a canvas's backing store to its css box times dpr, and hand back a
     context already scaled so every later coordinate is a css pixel. Returns
     the css width and height too, since that is what the drawing code wants
     and reading .width back off the element would give the scaled figure.

     The harness reports a fixed 300x300 box and a dpr of 2, so this runs in a
     test; it is the drawing that cannot be checked there, not the sizing. */
  function fit(canvas, h){
    if(!canvas || typeof canvas.getContext !== "function") return null;
    const dpr = (typeof devicePixelRatio === "number" && devicePixelRatio) || 1;
    const box = canvas.getBoundingClientRect ? canvas.getBoundingClientRect() : null;
    const w = Math.max(1, Math.round((box && box.width) || canvas.clientWidth || 300));
    const hh = Math.max(1, Math.round(h || (box && box.height) || canvas.clientHeight || 300));
    const W = Math.round(w * dpr), H = Math.round(hh * dpr);
    /* Only when it actually changed. Assigning width or height reallocates the
       backing store -- on a GPU-composited canvas that is a real buffer, freed
       and taken again. These draw functions are driven by a rAF loop, so an
       unconditional assignment here was sixty reallocations a second for as
       long as the page was open, which is how you kill a renderer.

       Every hand-written canvas on this site already guarded this. This one
       did not, which is the argument for the guard rather than against it. */
    if(canvas.width !== W || canvas.height !== H){
      canvas.width = W;
      canvas.height = H;
      canvas.style.height = hh + "px";
    }
    const ctx = canvas.getContext("2d");
    /* reapplied every call: cheap, and the transform is reset by a resize */
    if(ctx && ctx.setTransform) ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    return {ctx, w, h: hh, dpr};
  }

  /* The page's own palette, read off the stylesheet rather than repeated here.
     base.css already defines light and dark; duplicating those hexes in JS is
     how a figure ends up the only thing on the page that ignores the theme. */
  function ink(name, fallback){
    try {
      const v = getComputedStyle(document.documentElement).getPropertyValue(name);
      return (v && v.trim()) || fallback;
    } catch(e){ return fallback; }
  }
  const palette = () => ({
    blue: ink("--blue", "#2558C6"), red: ink("--red", "#C13030"),
    green: ink("--green", "#2C8A61"), brass: ink("--brass", "#9A6E24"),
    ink: ink("--ink", "#131923"), ink2: ink("--ink2", "#4C5A6E"),
    ink3: ink("--ink3", "#7B889B"), rule: ink("--rule", "#CFD8E4"),
    panel: ink("--panel", "#F7F9FC"), sunk: ink("--sunk", "#DEE4EC"),
  });
  /* a signed digit's colour, by the site's one convention: +1 blue, -1 red, 0 green */
  const hue = (v, p) => v > 0 ? p.blue : v < 0 ? p.red : p.green;

  /* ---- tooltips -----------------------------------------------------------
     One dictionary, one floating card, reused. A control says which entry it
     wants and nothing else; the writing lives in one place per page and the
     glossary lives in glossary.js, shared by all nine.

     Every entry is {title, what, why}: the term, what the control does, and
     what it is for. The third line is the one that earns the tooltip -- a
     reader who can see a slider called SHELL can already guess it changes the
     shell; what they cannot guess is why they would touch it. */
  const TIPS = {};
  let tipEl = null, tipFor = null, tipPinned = false, tipTimer = null;

  /* How long a card may stay up without the pointer being over the thing it
     belongs to. pointerleave is the normal way one closes, but it is not
     guaranteed to arrive: opening a native <select> puts an OS popup over the
     page and the label underneath never gets its leave event, so the card sat
     there until something else happened to move it. The floor under all of it
     is this cap: however a card was opened and whatever events do or do not
     arrive afterwards, it is gone within TIP_LIFE. */
  const TIP_LIFE = 4000;

  /* is (x,y) within `pad` of this rect? Pure, so the dismissal rule can be
     tested without a pointer to move. */
  function overRect(r, x, y, pad){
    if(!r) return false;
    pad = pad === undefined ? 24 : pad;
    /* right and bottom are derived when absent rather than assumed present:
       comparing against an undefined edge is a silent NaN, and the answer is
       then always "not over it" -- a card that closes the instant it opens. */
    const right  = r.right  === undefined ? r.left + (r.width  || 0) : r.right;
    const bottom = r.bottom === undefined ? r.top  + (r.height || 0) : r.bottom;
    return x >= r.left - pad && x <= right + pad
        && y >= r.top - pad  && y <= bottom + pad;
  }

  /* Where the card goes, given the thing it points at and the room available.
     Pure, so tools/tips.test.js can check the one property that matters: it
     never leaves the viewport, at any anchor position or card size. */
  function place(anchor, card, vw, vh, gap){
    gap = gap === undefined ? 10 : gap;
    const w = card.w, h = card.h;
    /* below by default; above when there is not room below but there is above */
    const roomBelow = vh - anchor.bottom, roomAbove = anchor.top;
    const side = (roomBelow >= h + gap || roomBelow >= roomAbove) ? "below" : "above";
    let y = side === "below" ? anchor.bottom + gap : anchor.top - h - gap;
    /* centre on the anchor, then pull back inside the edges */
    let x = anchor.left + anchor.width / 2 - w / 2;
    x = clamp(x, gap, Math.max(gap, vw - w - gap));
    y = clamp(y, gap, Math.max(gap, vh - h - gap));
    return {x, y, side};
  }

  function tipNode(){
    if(tipEl || typeof document === "undefined") return tipEl;
    tipEl = document.createElement("div");
    tipEl.className = "cc-tip";
    tipEl.id = "cc-tip";
    tipEl.setAttribute("role", "tooltip");
    tipEl.hidden = true;
    if(document.body && document.body.appendChild) document.body.appendChild(tipEl);
    return tipEl;
  }

  const esc = s => String(s == null ? "" : s)
    .replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

  /* the card's markup, from an entry. Kept separate from showing it so a test
     can read what a term would say without a DOM to say it into. */
  function tipHTML(key){
    const t = TIPS[key];
    if(!t) return "";
    return `<b>${esc(t.title || key)}</b>`
      + (t.what ? `<span>${esc(t.what)}</span>` : "")
      + (t.why ? `<em>${esc(t.why)}</em>` : "");
  }

  function showTip(key, anchorEl){
    const node = tipNode();
    if(!node || !TIPS[key] || !anchorEl) return false;
    node.innerHTML = tipHTML(key);
    node.hidden = false;
    tipFor = anchorEl;
    const r = anchorEl.getBoundingClientRect ? anchorEl.getBoundingClientRect()
            : {left: 0, top: 0, bottom: 0, width: 0};
    const cw = node.offsetWidth || 260, ch = node.offsetHeight || 90;
    const vw = (typeof innerWidth === "number" && innerWidth) || 1200;
    const vh = (typeof innerHeight === "number" && innerHeight) || 800;
    const p = place({left: r.left, top: r.top, bottom: r.bottom, width: r.width || 0},
                    {w: cw, h: ch}, vw, vh);
    node.style.left = Math.round(p.x) + "px";
    node.style.top = Math.round(p.y) + "px";
    node.className = "cc-tip " + p.side;
    if(anchorEl.setAttribute) anchorEl.setAttribute("aria-describedby", "cc-tip");
    /* the backstop: however it was opened and whatever happens to the pointer
       afterwards, the card is gone within TIP_LIFE */
    if(tipTimer && typeof clearTimeout === "function") clearTimeout(tipTimer);
    if(typeof setTimeout === "function")
      tipTimer = setTimeout(() => hideTip(true), TIP_LIFE);
    return true;
  }
  function hideTip(force){
    if(tipPinned && !force) return;
    tipPinned = false;
    if(tipTimer && typeof clearTimeout === "function") clearTimeout(tipTimer);
    tipTimer = null;
    if(tipEl){ tipEl.hidden = true; }
    if(tipFor && tipFor.removeAttribute) tipFor.removeAttribute("aria-describedby");
    tipFor = null;
  }

  /* Is the card outstaying its welcome? True when one is up and the pointer is
     nowhere near the thing it explains. Kept separate from the listener so the
     rule can be tested with coordinates rather than with a mouse. */
  function tipStale(x, y){
    if(!tipFor || tipPinned) return false;
    const r = tipFor.getBoundingClientRect ? tipFor.getBoundingClientRect() : null;
    return !overRect(r, x, y);
  }

  /* Bind one element. Hover and focus both show it -- a keyboard reader gets
     the same help as a mouse one -- and a tap pins it, since a touch device
     has no hover to end. */
  function attachTip(el, key){
    if(!el || !TIPS[key]) return false;
    listen(el, "pointerenter", () => { if(!tipPinned) showTip(key, el); });
    listen(el, "pointerleave", () => hideTip());
    listen(el, "focus", () => showTip(key, el));
    listen(el, "blur", () => hideTip(true));
    listen(el, "click", e => {
      /* on touch there is no pointerleave, so a tap toggles it open and shut */
      if(e && e.pointerType === "touch"){
        tipPinned ? hideTip(true) : (showTip(key, el), tipPinned = true);
      }
    });
    listen(el, "keydown", e => { if(e.key === "Escape") hideTip(true); });
    return true;
  }

  const tips = {
    /* merge a dictionary in. Pages call this with their own controls; the
       shared vocabulary arrives the same way from glossary.js. */
    add(dict){ for(const k in dict) if(dict.hasOwnProperty(k)) TIPS[k] = dict[k]; return tips; },
    get(key){ return TIPS[key] || null; },
    has(key){ return !!TIPS[key]; },
    keys(){ return Object.keys(TIPS); },
    html: tipHTML,
    place,
    /* by id -- explicit, and the form the harness can follow */
    attach(id, key){ return attachTip(el(id), key === undefined ? id : key); },
    attachEl: attachTip,
    show: showTip,
    hide: hideTip,
    /* everything on the page carrying data-tip. Convenience for the browser;
       querySelectorAll is empty under the harness, so this is never the only
       way a control gets its tooltip. */
    scan(root){
      const scope = root || (typeof document !== "undefined" ? document : null);
      if(!scope || typeof scope.querySelectorAll !== "function") return 0;
      let n = 0;
      for(const node of Array.from(scope.querySelectorAll("[data-tip]")))
        if(attachTip(node, node.getAttribute("data-tip"))) n++;
      return n;
    },
    /* missing entries are a content bug, not a crash: report them so a test
       can fail on a control that points at writing nobody did */
    missing(keys){ return keys.filter(k => !TIPS[k]); },
    stale: tipStale, overRect, life: TIP_LIFE,
  };
  /* Closing a tooltip must not depend on any one event arriving.
     pointerleave is the obvious one and it is not reliable: a native <select>
     popup swallows it, an implicit pointer capture during a slider drag
     retargets it, and a pointer that leaves the window fires nothing at all.
     Each of those left a card up with nothing watching it.

     So every one of these closes it, and any single one is enough. */
  if(typeof document !== "undefined" && document.addEventListener){
    const away = () => { if(tipFor) hideTip(); };

    document.addEventListener("keydown", e => { if(e.key === "Escape") hideTip(true); });

    /* the pointer is over some other element now. pointerover bubbles and
       fires on entering anything, so it catches the cases a missed
       pointerleave does -- this is the one that does most of the work. */
    document.addEventListener("pointerover", e => {
      if(!tipFor) return;
      const t = e.target;
      if(t === tipFor || (tipFor.contains && t && tipFor.contains(t))) return;
      hideTip();
    }, {passive: true});

    /* and the pointer moved somewhere not near the anchor */
    document.addEventListener("pointermove", e => {
      if(tipFor && tipStale(e.clientX, e.clientY)) hideTip();
    }, {passive: true});

    /* the pointer left the document entirely: no move, no leave, nothing */
    document.addEventListener("mouseleave", away);
    document.addEventListener("pointerleave", away);

    /* any click, anywhere, is the reader doing something else */
    document.addEventListener("pointerdown", e => {
      if(!tipFor) return;
      const t = e.target;
      if(t === tipFor || (tipFor.contains && t && tipFor.contains(t))) return;
      hideTip(true);
    }, {passive: true});

    /* scrolling moves the anchor out from under a card fixed to the viewport */
    document.addEventListener("scroll", () => hideTip(true), {passive: true, capture: true});
    document.addEventListener("visibilitychange", () => hideTip(true));
  }
  if(typeof addEventListener === "function"){
    addEventListener("blur", () => hideTip(true));
    addEventListener("resize", () => hideTip(true));
  }

  /* ---- first-visit orientation -------------------------------------------
     Shown once per page, then remembered. Storage can throw outright in a
     private window, so every touch of it is guarded and a failure just means
     the panel shows again -- which is the harmless direction to fail in. */
  function seen(key, mark){
    try {
      if(typeof localStorage === "undefined" || !localStorage) return false;
      const k = "cc-seen-" + key;
      if(mark){ localStorage.setItem(k, "1"); return true; }
      return localStorage.getItem(k) === "1";
    } catch(e){ return false; }
  }

  /* ---- audio ---------------------------------------------------------------
     The Wub pages are a bank of oscillators that has never been plugged in.
     Each phasor already carries an amplitude, two integer rates and a phase;
     that is additive synthesis with the speaker left off. The rates are small
     integers, so the result is a harmonic stack rather than noise.

     Nothing here starts on its own. A browser would refuse anyway, but the
     rule is worth keeping for its own sake: a page that makes a sound you did
     not ask for is a page you close. */

  /* One phasor -> two voices, because the drawing uses two rates: the phasor
     rides an ellipsoid and angleA and angleB turn at rateA and rateB. Taking
     only rateA would be a different instrument from the one on screen, and
     would collapse two phasors that happen to share it onto one pitch.
     base is the frequency that rate 1 maps to. Pure. */
  function voicesOf(p, base){
    if(!p) return [];
    const amp = Math.abs(p.amp || 0), ph = p.phase || 0;
    const out = [];
    for(const rate of [p.rateA, p.rateB]){
      const r = Math.abs(rate || 0);
      if(r > 0) out.push({freq: base * r, amp: amp / 2, phase: ph});
    }
    /* a phasor with no rate at all still has a value; give it the fundamental
       rather than dropping it silently out of the chord */
    return out.length ? out : [{freq: base, amp, phase: ph}];
  }

  /* Scale a set of voices so their sum cannot clip, and drop the inaudible.
     Additive synthesis sums amplitudes, so eight phasors at 0.5 is a 4.0 peak
     and the output is square-wave mush. Pure, so tools/audio.test.js can hold
     it to the one property that matters: the total never exceeds 1. */
  function mix(voices, ceiling){
    const cap = ceiling === undefined ? 1 : ceiling;
    const live = (voices || []).filter(v => v && isFinite(v.freq) && v.freq > 0
                                          && isFinite(v.amp) && v.amp > 0);
    const total = live.reduce((s, v) => s + v.amp, 0);
    if(total <= 0) return [];
    const k = total > cap ? cap / total : 1;
    return live.map(v => ({freq: v.freq, amp: v.amp * k, phase: v.phase || 0}));
  }

  let AC = null, master = null, bank = [], playing = false;

  const audio = {
    available(){ return typeof AudioContext !== "undefined" || typeof webkitAudioContext !== "undefined"; },
    get on(){ return playing; },
    voicesOf, mix,

    /* Called from a click, never otherwise. Returns false if the browser has
       no audio at all, so a caller can hide the control rather than offer a
       button that does nothing. */
    start(){
      if(playing) return true;
      if(!audio.available()) return false;
      try {
        const Ctor = typeof AudioContext !== "undefined" ? AudioContext : webkitAudioContext;
        if(!AC) AC = new Ctor();
        if(AC.state === "suspended" && AC.resume) AC.resume();
        if(!master){
          master = AC.createGain();
          master.gain.value = 0.0001;
          master.connect(AC.destination);
        }
        /* ramp up rather than switch on: a gain that jumps from 0 is a click */
        master.gain.setTargetAtTime(0.22, AC.currentTime, 0.05);
        playing = true;
        return true;
      } catch(e){ return false; }
    },
    stop(){
      playing = false;
      if(!AC || !master) return;
      try { master.gain.setTargetAtTime(0.0001, AC.currentTime, 0.05); } catch(e){}
      for(const v of bank){ try { v.osc.stop(AC.currentTime + 0.3); } catch(e){} }
      bank = [];
    },
    toggle(){ return playing ? (audio.stop(), false) : audio.start(); },
    level(v){
      if(master && AC) try { master.gain.setTargetAtTime(clamp(v, 0, 1) * 0.4, AC.currentTime, 0.05); } catch(e){}
    },

    /* Hand it the current voices; it grows or shrinks the bank to match and
       glides the survivors to their new pitch. Rebuilding every oscillator on
       every change would click on every slider move. */
    set(voices){
      if(!playing || !AC || !master) return 0;
      const want = mix(voices);
      try {
        while(bank.length > want.length){
          const v = bank.pop();
          v.gain.gain.setTargetAtTime(0.0001, AC.currentTime, 0.03);
          v.osc.stop(AC.currentTime + 0.2);
        }
        while(bank.length < want.length){
          const osc = AC.createOscillator(), gain = AC.createGain();
          osc.type = "sine";
          gain.gain.value = 0.0001;
          osc.connect(gain); gain.connect(master); osc.start();
          bank.push({osc, gain});
        }
        want.forEach((w, i) => {
          const v = bank[i];
          v.osc.frequency.setTargetAtTime(w.freq, AC.currentTime, 0.03);
          v.gain.gain.setTargetAtTime(w.amp, AC.currentTime, 0.05);
        });
      } catch(e){ return 0; }
      return want.length;
    },
  };

  /* ---- permalinks ----------------------------------------------------------
     Every figure on this site is a state nobody can send anyone. The hash is
     the cheapest fix: readable, no server, and it survives a reload.

     encode/decode are pure and each other's inverse, which is the whole of
     what a test needs to check. */
  function encodeState(obj){
    const parts = [];
    for(const k in obj) if(obj.hasOwnProperty(k)){
      const v = obj[k];
      if(v === undefined || v === null || v === "") continue;
      parts.push(encodeURIComponent(k) + "=" + encodeURIComponent(String(v)));
    }
    return parts.join("&");
  }
  function decodeState(str){
    const out = {};
    for(const pair of String(str || "").replace(/^#/, "").split("&")){
      if(!pair) continue;
      const i = pair.indexOf("=");
      const k = i < 0 ? pair : pair.slice(0, i);
      const v = i < 0 ? "" : pair.slice(i + 1);
      try { out[decodeURIComponent(k)] = decodeURIComponent(v); }
      catch(e){ out[k] = v; }          // a hand-mangled hash is not an error
    }
    return out;
  }

  const hash = {
    encode: encodeState, decode: decodeState,
    read(){
      try { return decodeState(typeof location !== "undefined" ? location.hash : ""); }
      catch(e){ return {}; }
    },
    /* replaceState so the back button still means "the previous page" rather
       than "the previous slider position" */
    write(obj){
      const s = encodeState(obj);
      try {
        if(typeof history !== "undefined" && history.replaceState)
          history.replaceState(null, "", s ? "#" + s : location.pathname + location.search);
        else if(typeof location !== "undefined") location.hash = s;
      } catch(e){}
      return s;
    },
    /* read a number out of the hash, with a fallback and a range */
    num(state, key, dflt, lo, hi){
      const v = parseFloat(state && state[key]);
      if(!isFinite(v)) return dflt;
      return (lo === undefined) ? v : clamp(v, lo, hi);
    },
  };

  return {cellAt, indexOfCell, regionOf, phase, clamp,
          slider, select, toggle, scrub, drag, fit, palette, hue,
          tips, place, seen, audio, hash};
})();
