//! eggv13 -- the Transmuter (the Value Underneath campaign).
//!
//! We are not encoding. We TRANSMUTE data into another form and RESTORE it.
//! The first law is conservation: the information never moves; only the form
//! does (push's law, generalized -- the site says "the value never moves;
//! only the colours do"). The end-to-end FNV-64 of the original bytes is the
//! conservation check, and it gates every restore.
//!
//! The transmutation chain, four form-changes, one container:
//!
//!   bytes -> NIBS -> TOKENS (match/literal) -> DYADIC POINT -> ARMORED FORM
//!
//! The centerpiece is the dyadic stage: the whole file becomes ONE dyadic
//! rational -- a single point on the site's own disc -- because that is
//! literally what an arithmetic coder emits: the address of the interval
//! where the message lands on the dyadic tree. Structured files transmute to
//! SHORT addresses; random files to addresses as long as themselves (the
//! pigeonhole, kept, and asserted as a PASS in the tests).
//!
//! Attribution: Elias, Rissanen, Witten-Neal-Cleary (arithmetic coding);
//! Ziv-Lempel 1977 (the match layer; the site's bar notation, generalized);
//! Igor Pavlov's LZMA (the rep-offset and slot/align coding shapes); the
//! site supplied the geometry and the vocabulary.

mod armor;
#[allow(dead_code)]
mod armor11; // v11 armor v3, VERBATIM: the ancestor restore path (.egg8/.egg9/.egg10/.egg11)
mod audit;
mod dyadic;
mod filter;
mod jcoef;
mod jpeg;
mod mix10;
mod mix11;
mod mix12;
mod prior_pe;
mod prior_tab;
mod prior_ttf;
mod state_tab;
mod mix9;
mod numtext; // M3c's WS-N: the number field tracker
mod twod; // M3c's WS-2D: the rectangle
mod deflate;
mod peel;
mod sites; // M3b's two instruments (S1b the gcd, S1c the bit period)
mod squash_tab;
mod structure;
mod token;

use armor::{armor, dearmor, fnv64, geom, offsets, price, promise, rib_no_armor, rib_search, rib_search_with, scratch_guaranteed, square_off, CtMode, Extras, Rib, SURVIVE_DEFAULT};
use std::env;
use std::fs;
use std::process::ExitCode;
use std::time::Instant;

// model byte in the header: which form the payload is in
const MODEL_IDENTITY: u8 = 1; // bytes as they came (armor-only, M0 scaffold)
const MODEL_TOKENS: u8 = 2; // match/literal sequences, raw (M1 stage measure)
const MODEL_DYADIC: u8 = 3; // one dyadic point, literal context 8 bits (2 nibs)
const MODEL_DYADIC2: u8 = 4; // one dyadic point, literal context 16 bits (4 nibs)
const MODEL_MIX: u8 = 5; // one dyadic point, mixed literal model (v8, WS2)
const MODEL_MIX9: u8 = 6; // one dyadic point, v9 model: match model + widened mixer
const MODEL_CM9: u8 = 7; // one dyadic point, literal-only: the match model carries repeats
const MODEL_MIX10: u8 = 8; // one dyadic point, v10 model (the Keeper; restore + frozen floor)
const MODEL_CM10: u8 = 9; // one dyadic point, literal-only v10 (restore + frozen floor)
const MODEL_MIX11: u8 = 10; // one dyadic point, v11 model (the Rematch)
const MODEL_CM11: u8 = 11; // one dyadic point, literal-only v11
const MODEL_MIX11P: u8 = 12; // v11 model, site-prior primed (the educated guess)
const MODEL_CM11P: u8 = 13; // literal-only v11, site-prior primed
const MODEL_MIX11H: u8 = 14; // v11 model, heavy LR 13 (text/db/records feast)
const MODEL_CM11H: u8 = 15; // literal-only v11, heavy LR 13
// v12-M2a: the 16-bit arms (mix12.rs; the precision debt, glossary.js:164)
const MODEL_MIX12: u8 = 16; // v12 model, LZ tokens + mixed literals at 16 bits
const MODEL_CM12: u8 = 17; // literal-only v12
const MODEL_MIX12P: u8 = 18; // v12 model, site-prior primed
const MODEL_CM12P: u8 = 19; // literal-only v12, site-prior primed
const MODEL_MIX12H: u8 = 20; // v12 model, heavy LR 13
const MODEL_CM12H: u8 = 21; // literal-only v12, heavy LR 13
// v12-M2c(c): the dialect books as trial arms (sniffed by magic; the book has no test row in it)
const MODEL_CM12_PE: u8 = 22; // literal-only v12 primed by the PE book
const MODEL_CM12_TTF: u8 = 23; // literal-only v12 primed by the TTF book
// v13-M1: the peel frame (WS-F). The payload is a peel preamble, the recipe
// stream and the values stream; peel.rs owns the layout and the id ceiling.
const MODEL_PEEL: u8 = 24; // a peeled form: (recipe, values), each modelled
// v13-M2: the deflate recipe has its own model byte. It is FOUR streams in
// four languages (v12 reading: lengths and distances model badly interleaved
// and well apart), each through the ordinary roster, each keeping its own
// model byte. peel.rs owns the constant so the preamble and the ladder read
// ONE number, never two.
const MODEL_DRECIPE: u8 = peel::MODEL_DRECIPE;
// v13-M3c (WS-N): the v12 literal model with its two sparse inputs re-pointed
// at the number field tracker. A ROSTER ENTRANT like any other -- nominated by
// `numtext::looks_numeric`, judged on the armored total, free when it loses.
const MODEL_NUM: u8 = 27;
// v13-M3c (WS-2D): the same model reading a rectangle instead of a tail. The
// stride and pixel ride in the arm's own header, measured by `twod::nominate`.
const MODEL_2D: u8 = 28;

/// v14-N2b: per-arm WALL CLOCK. The roster runs its arms in parallel scoped
/// threads, so the row finishes when the SLOWEST arm finishes -- N2 measured
/// that removing a LOSING arm buys nothing, and this is the instrument that
/// says which arm actually sets the floor. `EGG_ARMS=1` prints it. Slot order
/// matches the spawn order below; `ARM_SLOTS` is one number both sides read.
const ARM_SLOTS: usize = 18;
/// THREE banks of slots, not one. `plain_transmute` runs `big_arms` on the
/// plain form and on the chosen filtered form SIDE BY SIDE in a single thread
/// scope, so a single bank races and both rosters print whichever finished
/// last. The first cut of this instrument did exactly that and printed two
/// identical lines; the bank is chosen by the caller's label.
const ARM_BANKS: usize = 3;
static ARM_MS: [std::sync::atomic::AtomicU64; ARM_SLOTS * ARM_BANKS] =
    [const { std::sync::atomic::AtomicU64::new(0) }; ARM_SLOTS * ARM_BANKS];
#[inline]
fn arm_bank(label: &str) -> usize {
    match label {
        "plain" => 0,
        "filtered" => ARM_SLOTS,
        _ => 2 * ARM_SLOTS,
    }
}
/// time one arm into its slot. Returns exactly what the closure returned, so
/// wrapping an arm cannot change what the roster judges.
#[inline]
fn timed<T>(bank: usize, slot: usize, f: impl FnOnce() -> T) -> T {
    let t0 = Instant::now();
    let r = f();
    ARM_MS[bank + slot].store(t0.elapsed().as_millis() as u64, std::sync::atomic::Ordering::Relaxed);
    r
}

// ---------------------------------------------------------------- pipeline
// the transmutation chain: bytes -> nibs -> tokens -> dyadic point. Each
// form is a model byte in the header; restore dispatches on it, so stage
// measurements and the shipped form share one container and one gate.

/// transmute src into the requested form; the dyadic form is coded at both
/// literal depths (2-nib and 4-nib context) and the lighter point is kept --
/// which depth won is stamped in the model byte, so restore knows the walk.
fn transmute_bytes(src: &[u8], model: u8) -> (Vec<u8>, u8) {
    match model {
        MODEL_IDENTITY => (src.to_vec(), MODEL_IDENTITY),
        MODEL_TOKENS => (token::tokens_serialize(&token::tokenize(src)), MODEL_TOKENS),
        MODEL_DYADIC | MODEL_DYADIC2 | MODEL_MIX => {
            let toks = token::tokenize(src);
            encode_best(src, &toks)
        }
        _ => unreachable!(),
    }
}
fn restore_bytes(inner: &[u8], orig_len: usize, model: u8) -> Result<Vec<u8>, String> {
    match model {
        MODEL_IDENTITY => Ok(inner.to_vec()),
        MODEL_TOKENS => token::tokens_restore(inner, orig_len),
        MODEL_DYADIC => dyadic::decode(inner, orig_len, 8),
        MODEL_DYADIC2 => dyadic::decode(inner, orig_len, 16),
        MODEL_MIX => dyadic::decode(inner, orig_len, 0),
        MODEL_MIX9 => dyadic::decode9(inner, orig_len),
        MODEL_CM9 => dyadic::decode_cm9(inner, orig_len),
        MODEL_MIX10 => dyadic::decode10(inner, orig_len),
        MODEL_CM10 => dyadic::decode_cm10(inner, orig_len),
        MODEL_MIX11 => dyadic::decode11(inner, orig_len),
        MODEL_CM11 => dyadic::decode_cm11(inner, orig_len),
        MODEL_MIX11P => dyadic::decode11p(inner, orig_len),
        MODEL_CM11P => dyadic::decode_cm11p(inner, orig_len),
        MODEL_MIX11H => dyadic::decode11h(inner, orig_len),
        MODEL_CM11H => dyadic::decode_cm11h(inner, orig_len),
        MODEL_MIX12 => dyadic::decode12(inner, orig_len),
        MODEL_CM12 => dyadic::decode_cm12(inner, orig_len),
        MODEL_MIX12P => dyadic::decode12p(inner, orig_len),
        MODEL_CM12P => dyadic::decode_cm12p(inner, orig_len),
        MODEL_MIX12H => dyadic::decode12h(inner, orig_len),
        MODEL_CM12H => dyadic::decode_cm12h(inner, orig_len),
        MODEL_CM12_PE => dyadic::decode_cm12_book(inner, orig_len, &mix12::BOOK_PE),
        MODEL_CM12_TTF => dyadic::decode_cm12_book(inner, orig_len, &mix12::BOOK_TTF),
        MODEL_NUM => dyadic::decode_num(inner, orig_len),
        MODEL_2D => dyadic::decode_2d(inner, orig_len),
        MODEL_DRECIPE => decode_drecipe(inner, orig_len),
        MODEL_PEEL => restore_peel(inner),
        m => Err(format!("unknown model byte {} -- newer transmuter?", m)),
    }
}

/// the peeled payload, read back: the preamble names the recipe's model and
/// length and the values' model; the recipe is restored by the ordinary model
/// ladder and the peel re-spells the original bytes from recipe + values. The
/// container's own length check and FNV-64 gate then decide, as always.
fn restore_peel(inner: &[u8]) -> Result<Vec<u8>, String> {
    restore_peel_at(inner, 1)
}

/// THE CHAIN (v13-M3d, S3b). `wubdiv.html:371-375`: "Each step hands its
/// quotient down as the next dividend ... The remainder is not carried. That is
/// what makes it a remainder: it is shown, and the step it belongs to owns it."
/// A peel's VALUES may themselves be a peel, to `PEEL_DEPTH_MAX` and no
/// further; each step's recipe is sealed where it was made, and the depth is a
/// number both directions read from ONE constant.
const PEEL_DEPTH_MAX: u32 = 2;

fn restore_peel_at(inner: &[u8], depth: u32) -> Result<Vec<u8>, String> {
    let (id, rmodel, rraw, rlen, vmodel, vraw) = peel::read_preamble(inner)?;
    if rmodel == MODEL_PEEL {
        return Err("a peel inside a peel recipe: refused".into());
    }
    let end = peel::PREAMBLE.checked_add(rlen).ok_or("peel: recipe length overflows")?;
    if end > inner.len() {
        return Err(format!("peel: recipe stream of {} B does not fit in {} B of payload", rlen, inner.len()));
    }
    let recipe = restore_bytes(&inner[peel::PREAMBLE..end], rraw, rmodel)?;
    if recipe.len() != rraw {
        return Err(format!("peel: recipe restored {} B, the preamble said {}", recipe.len(), rraw));
    }
    if peel::values_are_bytes(id) {
        // the deflate peel values ARE the file underneath: ordinary bytes, so
        // they come back up the ordinary ladder and the peel only re-spells
        let values = if vmodel == MODEL_PEEL {
            if depth >= PEEL_DEPTH_MAX {
                return Err(format!("a peel {} deep: the chain stops at {}", depth + 1, PEEL_DEPTH_MAX));
            }
            restore_peel_at(&inner[end..], depth + 1)?
        } else {
            restore_bytes(&inner[end..], vraw, vmodel)?
        };
        if values.len() != vraw {
            return Err(format!("peel: values restored {} B, the preamble said {}", values.len(), vraw));
        }
        return peel::respell_bytes(id, &recipe, values);
    }
    peel::decode_and_respell(id, &recipe, vmodel, &inner[end..])
}

/// THE DEFLATE RECIPE, MODELLED (v13-M2, a fifth section at M3a). The structure
/// (block headers, every code-length definition, the token counts, the wrapper
/// bytes), the literal-or-match flags, the match lengths one byte each, the
/// match distances two, and the sparse SPELLING LIST -- and each goes through
/// the SAME roster every other byte stream in this house goes through, keeping
/// its own winner model byte.
///
/// Why five rosters and not one: the v12 reading, measured again here. On
/// aoe4-autosave.sav the four original sections are won by four different arms,
/// and the one that wins on distances is the only LZ arm in the roster, because
/// the distance SEQUENCE repeats at a range the deflate 32 KB window could never
/// reach. Interleaved they would hide each other. The fifth section is empty on
/// every file in this corpus and costs 9 B of section table when it is.
fn encode_drecipe(blob: &[u8]) -> Vec<u8> {
    let l = match deflate::layout(blob) {
        Ok(l) => l,
        Err(_) => return Vec::new(), // the caller then keeps the blob raw
    };
    let secs: [(usize, usize); 5] = [(0, l.meta.1), l.flags, l.lens, l.dists, l.resp];
    let mut streams: Vec<(Vec<u8>, u8)> = Vec::with_capacity(5);
    for (a, b) in secs {
        let data = &blob[a..b];
        if data.is_empty() {
            streams.push((Vec::new(), MODEL_IDENTITY));
            continue;
        }
        let toks = token::tokenize(data);
        let (v, m) = encode_best(data, &toks);
        if v.len() < data.len() {
            streams.push((v, m));
        } else {
            streams.push((data.to_vec(), MODEL_IDENTITY));
        }
    }
    let mut out = Vec::with_capacity(1 + 5 * 9 + streams.iter().map(|(v, _)| v.len()).sum::<usize>());
    out.push(5u8);
    for (i, (v, m)) in streams.iter().enumerate() {
        out.push(*m);
        out.extend_from_slice(&((secs[i].1 - secs[i].0) as u32).to_le_bytes());
        out.extend_from_slice(&(v.len() as u32).to_le_bytes());
    }
    for (v, _) in &streams {
        out.extend_from_slice(v);
    }
    out
}
/// the five sections back, concatenated into the blob they came from
fn decode_drecipe(inner: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    if inner.is_empty() {
        return Err("an empty deflate recipe stream".into());
    }
    let n = inner[0] as usize;
    if n != 5 {
        return Err(format!("a deflate recipe of {} sections, not 5", n));
    }
    let tab = 1 + n * 9;
    if inner.len() < tab {
        return Err("a deflate recipe stream shorter than its section table".into());
    }
    let mut at = tab;
    let mut out: Vec<u8> = Vec::with_capacity(orig_len);
    for i in 0..n {
        let o = 1 + i * 9;
        let m = inner[o];
        if m == MODEL_PEEL || m == MODEL_DRECIPE {
            return Err("a deflate recipe section that models itself: refused".into());
        }
        let raw = u32::from_le_bytes(inner[o + 1..o + 5].try_into().unwrap()) as usize;
        let len = u32::from_le_bytes(inner[o + 5..o + 9].try_into().unwrap()) as usize;
        let end = at.checked_add(len).ok_or("a deflate recipe section length overflows")?;
        if end > inner.len() {
            return Err(format!("a deflate recipe section of {} B does not fit in {} B", len, inner.len()));
        }
        let sec = restore_bytes(&inner[at..end], raw, m)?;
        if sec.len() != raw {
            return Err(format!("a deflate recipe section restored {} B, its header said {}", sec.len(), raw));
        }
        out.extend_from_slice(&sec);
        at = end;
    }
    if at != inner.len() {
        return Err(format!("a deflate recipe stream of {} B whose sections account for {}", inner.len(), at));
    }
    if out.len() != orig_len {
        return Err(format!("a deflate recipe restored {} B, the preamble said {}", out.len(), orig_len));
    }
    Ok(out)
}

/// THE PEEL ARM (v13-M1). The foreign code read as a value, offered to the same
/// argmin as every other form -- and THE LAW lives here: the peel re-encodes its
/// own output and compares against the original bytes BEFORE anything is
/// written. One byte off, or any refusal at all, and this returns None; the raw
/// bytes then go through the ordinary pipeline and nothing is lost.
/// `EGG_PEEL=1` (or `EGG_ARMS=1`) prints the reason and the recipe's own weight.
fn peel_arm(src: &[u8]) -> Option<(Vec<u8>, u8)> {
    peel_arm_at(src, 1)
}

fn peel_arm_at(src: &[u8], depth: u32) -> Option<(Vec<u8>, u8)> {
    if std::env::var_os("EGG_NO_PEEL").is_some() {
        return None;
    }
    let id = peel::nominate(src);
    if id == peel::PEEL_NONE {
        return None;
    }
    let trace = std::env::var_os("EGG_PEEL").is_some() || std::env::var_os("EGG_ARMS").is_some();
    let mut p = match peel::peel(src, id) {
        Ok(p) => p,
        Err(e) => {
            if trace {
                eprintln!("  peel {}: REFUSED -- {}; the bytes are kept", id, e);
            }
            return None;
        }
    };
    if trace {
        eprintln!("  peel {}: {}", id, peel::describe(&p));
    }
    match peel::respell(&p) {
        Ok(back) if back == src => {}
        Ok(back) => {
            if trace {
                eprintln!("  peel {}: REFUSED -- the re-encode is not the original ({} B vs {} B); the bytes are kept", id, back.len(), src.len());
            }
            return None;
        }
        Err(e) => {
            if trace {
                eprintln!("  peel {}: REFUSED -- the re-encode failed ({}); the bytes are kept", id, e);
            }
            return None;
        }
    }
    // the recipe is modelled on its own. The JPEG recipe is a few hundred bytes
    // and is judged by argmin between kept-as-it-came and the literal-only
    // 16-bit arm (at that size a model can cost more than it saves, and the trap
    // the plan names is a recipe that eats the prize). The deflate recipe is
    // four streams in four languages and gets the sectioned roster; if that ever
    // came back heavier than the blob, the blob rides raw.
    let raw_len = p.recipe.len();
    let (rstream, rmodel) = if id == peel::PEEL_DEFLATE {
        let m = encode_drecipe(&p.recipe);
        if !m.is_empty() && m.len() < raw_len {
            (m, MODEL_DRECIPE)
        } else {
            (std::mem::take(&mut p.recipe), MODEL_IDENTITY)
        }
    } else {
        let raw = p.recipe.clone();
        let cm = dyadic::encode_cm12(&raw);
        if cm.len() < raw.len() {
            (cm, MODEL_CM12)
        } else {
            (raw, MODEL_IDENTITY)
        }
    };
    // the values: the JPEG values are coefficients and only jcoef can read them;
    // the deflate values are the file underneath, ordinary bytes, and the whole
    // roster reads them
    let (vstream, vmodel) = if peel::values_are_bytes(id) {
        let vals = peel::take_values(&mut p);
        let toks = token::tokenize(&vals);
        let flat = encode_best(&vals, &toks);
        // THE CHAIN: the quotient handed down as the next dividend. The inner
        // peel proves its own bijection before it is offered, exactly as this
        // one did, and it is taken only if it is STRICTLY lighter -- on a tie
        // the flat form keeps it, because that is the form we can already prove.
        match if depth < PEEL_DEPTH_MAX { peel_arm_at(&vals, depth + 1) } else { None } {
            Some((chained, m)) if chained.len() < flat.0.len() => {
                if trace {
                    eprintln!("  peel {}: THE CHAIN took depth {} -- values {} B -> {} B", id, depth + 1, flat.0.len(), chained.len());
                }
                (chained, m)
            }
            _ => flat,
        }
    } else {
        let mut m = 0u8;
        let v = peel::encode_values(&mut p, &mut m);
        (v, m)
    };
    let mut inner = Vec::with_capacity(peel::PREAMBLE + rstream.len() + vstream.len());
    inner.extend_from_slice(&peel::write_preamble(id, rmodel, raw_len, rstream.len(), vmodel, p.values_raw_len));
    inner.extend_from_slice(&rstream);
    inner.extend_from_slice(&vstream);
    if trace {
        eprintln!(
            "  peel {}: recipe {} B -> {} B (model {}); values {} B raw -> {} B (model {}); inner {} B, armored {}",
            id, raw_len, rstream.len(), rmodel, p.values_raw_len, vstream.len(), vmodel, inner.len(), armored_total(inner.len())
        );
    }
    Some((inner, MODEL_PEEL))
}

/// the CHEAP model trial: v8's three entrants exactly, in parallel threads.
/// These are FROZEN -- v9 keeps them as bit-exact entrants so a regression
/// against v8 is impossible by construction. Used for filter selection
/// (the big v9 models run only on the chosen forms, see filtered_transmute).
fn encode_best_v8(src: &[u8], toks: &[token::Tok]) -> (Vec<u8>, u8) {
    let (a, b, c) = std::thread::scope(|s| {
        let h16 = s.spawn(|| dyadic::encode(src, toks, 16));
        let hmx = s.spawn(|| dyadic::encode(src, toks, 0));
        let a = dyadic::encode(src, toks, 8);
        (a, h16.join().expect("trial"), hmx.join().expect("trial"))
    });
    if std::env::var_os("EGG_ARMS").is_some() {
        eprintln!("arms-v8[{} B]: dyadic8={} dyadic16={} mix={}", src.len(), a.len(), b.len(), c.len());
    }
    // fixed tie-break order: mix, then 8, then 16 (newer form wins ties)
    if c.len() <= a.len() && c.len() <= b.len() {
        (c, MODEL_MIX)
    } else if a.len() <= b.len() {
        (a, MODEL_DYADIC)
    } else {
        (b, MODEL_DYADIC2)
    }
}
/// ONE second pass for all three call sites (v10 carried this logic thrice).
/// Gated >=512 KB as always: the greedy price replay. v11-M2's DP optimal
/// parse stood here for one afternoon and was KILLED by its own filed
/// criterion -- 0.000 avg gain on vim/zstd/db/cbs (needed >=0.08) at 2-3x
/// the cost, flat-rate length pricing being the convicted suspect. The miss
/// is printed in PREDICTIONS.md; a per-length, tstate-aware price table is
/// v12 evidence, not v11 code.
fn second_pass(src: &[u8], toks: &[token::Tok], cur: Vec<u8>) -> Vec<u8> {
    if src.len() < 512 * 1024 {
        return cur;
    }
    let p8 = replay_price8(src.len(), toks, cur.len());
    let toks2 = token::tokenize_priced(src, p8);
    let vb = dyadic::encode11(src, &toks2);
    if vb.len() < cur.len() {
        vb
    } else {
        cur
    }
}
/// the same replay for the v12 LZ arm (its own first-pass length prices it)
fn second_pass12(src: &[u8], toks: &[token::Tok], cur: Vec<u8>) -> Vec<u8> {
    if src.len() < 512 * 1024 {
        return cur;
    }
    let p8 = replay_price8(src.len(), toks, cur.len());
    let toks2 = token::tokenize_priced(src, p8);
    let vb = dyadic::encode12(src, &toks2);
    if vb.len() < cur.len() {
        vb
    } else {
        cur
    }
}

/// v9-M6 price replay (the fastest-route reading): after the first MIX9
/// encode, the MEASURED bits/byte of its literal stream re-prices the
/// tokenizer and a second MIX9 pass keeps the lighter point. Encoder-only.
fn replay_price8(src_len: usize, toks: &[token::Tok], inner_len: usize) -> i64 {
    // measured whole-stream cost, attributed to the literal bytes: an
    // over-estimate for literals on matchy files, but a far better guess
    // than the static o2-nib entropy on filtered/structured forms
    let lit_bytes: usize = toks
        .iter()
        .map(|t| if let token::Tok::Lit(_) = t { 1 } else { 0 })
        .sum();
    if lit_bytes == 0 || src_len == 0 {
        return 48;
    }
    ((inner_len as i64 * 8 * 8) / lit_bytes as i64).clamp(16, 72)
}
/// the FULL trial: v8's trio plus BOTH v9 models (the two big models are
/// exactly the <=2-concurrent budget), then the price-replay second pass;
/// newest form wins ties: 7, 6, then v8.
/// THE stage-2 roster, one function for every path (v11-M8 consolidation):
/// MIX11+CM11 always; the prior twins under 1 MB; the heavy-LR twins from
/// 512 KB; the FROZEN MIX10/CM10 arms unconditionally under 4 MB -- after
/// kernel32 breached the <=min(ancestors) law by one armor quantum, the law
/// became structural: on every small file the elders always get their say.
/// Strict less-than keeps ties with the newest form.
/// what the world weighs: the ARMORED total this inner would ship at.
/// v11-M8 kernel32 lesson: a 633 B lighter inner crossed a square boundary
/// into +1,024 B of parity -- trials that compare inner bytes optimize the
/// wrong metric at every armor quantum. All picks compare THIS.
fn armored_total(inner_len: usize) -> usize {
    // v12: the argmin over the grid and the CT placement at the default promise
    rib_search(inner_len, SURVIVE_DEFAULT, None, None).map(|r| r.total).unwrap_or(usize::MAX)
}

/// v12-M2a: the six 16-bit arms join the roster under the same gates as their
/// v11 twins (prior < 1 MB, heavy >= 512 KB); the v11 arms stay FROZEN
/// entrants, so a wide arm that loses costs nothing. Newest form first, strict
/// less-than: ties keep the earlier entrant. EGG_NO_V12 drops the six new arms
/// -- the roster and the tie order then collapse to M1's exactly (the
/// byte-identity proof of the frozen arms). EGG_ARMS prints every arm's inner.
/// M2c(c): which dialect book, if any, this form speaks -- by magic only.
/// PE ("MZ"); TrueType (00 01 00 00, "true") and OpenType ("OTTO") share the
/// TTF book. A filtered form never carries the magic, so only plain forms are
/// booked. The book arm runs at every size (a trial entrant; it costs one
/// parallel CM pass and can only win or be passed over).
fn dialect_of(data: &[u8]) -> Option<(&'static mix12::Book, u8)> {
    if data.len() >= 4 {
        if &data[0..2] == b"MZ" {
            return Some((&mix12::BOOK_PE, MODEL_CM12_PE));
        }
        if data[0..4] == [0, 1, 0, 0] || &data[0..4] == b"true" || &data[0..4] == b"OTTO" {
            return Some((&mix12::BOOK_TTF, MODEL_CM12_TTF));
        }
    }
    None
}

fn big_arms(data: &[u8], toks: &[token::Tok], label: &str) -> (Vec<u8>, u8) {
    let prior_arms = data.len() < (1 << 20);
    let heavy_arms = data.len() >= 512 * 1024;
    let frozen_arms = data.len() < (16 << 20); // raised from 4 MB after the
                                               // synthetic server-log breached
                                               // <=min in the 4-16 MB gap
    let wide = std::env::var_os("EGG_NO_V12").is_none();
    let dialect = dialect_of(data);
    let bank = arm_bank(label);
    let numeric = numtext::looks_numeric(data);
    // v14-N2: THE 2D ARM DOES NOT RUN WHERE A DIALECT BOOK ALREADY MATCHED.
    // Measured at v13-M3c: `twod::nominate` fires on PE and TrueType as well as
    // audio and text, and on the booked forms it always LOSES -- kernel32.dll
    // 2D=297,069 against CM12-PE's 284,319; segoeui.ttf 2D=408,943 against
    // CM12-TTF's 404,871 -- while costing a full CM pass to be passed over.
    // That cost showed up on the clock: the worst home row went 0.289 -> 0.268
    // MB/s at M3c against a 0.25 floor, the narrowest the series has run.
    //
    // The gate is an argument, not a tuning: a form whose magic names a dialect
    // has a specialist arm primed for it, so the rectangle is the wrong reading
    // by construction. Every row the 2D arm WINS is dialect-free -- alarm01.wav,
    // cbs.log, vim-version9.txt, mermaid-bundle.js, msgraph-docs.xml -- and a
    // FILTERED form never carries the magic (`dialect_of` reads offset 0), so
    // alarm01's win, which lives on the order-2 W16 residue, is untouched.
    let rect = if wide && dialect.is_none() { twod::nominate(data) } else { None };
    let (v11, cm, pr, cpr, hv, chv, f10, fc10, v12, cm12, pr12, cpr12, hv12, chv12, bk12, num, two) = std::thread::scope(|s| {
        let h1 = s.spawn(|| timed(bank, 0, || dyadic::encode11(data, toks)));
        let h2 = s.spawn(|| timed(bank, 1, || dyadic::encode_cm11(data)));
        let h3 = s.spawn(|| timed(bank, 2, || if prior_arms { Some(dyadic::encode11p(data, toks)) } else { None }));
        let h4 = s.spawn(|| timed(bank, 3, || if prior_arms { Some(dyadic::encode_cm11p(data)) } else { None }));
        let h5 = s.spawn(|| timed(bank, 4, || if heavy_arms { Some(dyadic::encode11h(data, toks)) } else { None }));
        let h6 = s.spawn(|| timed(bank, 5, || if heavy_arms { Some(dyadic::encode_cm11h(data)) } else { None }));
        let h7 = s.spawn(|| timed(bank, 6, || if frozen_arms { Some(dyadic::encode10(data, toks)) } else { None }));
        let w1 = s.spawn(|| timed(bank, 7, || if wide { Some(dyadic::encode12(data, toks)) } else { None }));
        let w2 = s.spawn(|| timed(bank, 8, || if wide { Some(dyadic::encode_cm12(data)) } else { None }));
        let w3 = s.spawn(|| timed(bank, 9, || if wide && prior_arms { Some(dyadic::encode12p(data, toks)) } else { None }));
        let w4 = s.spawn(|| timed(bank, 10, || if wide && prior_arms { Some(dyadic::encode_cm12p(data)) } else { None }));
        let w5 = s.spawn(|| timed(bank, 11, || if wide && heavy_arms { Some(dyadic::encode12h(data, toks)) } else { None }));
        let w6 = s.spawn(|| timed(bank, 12, || if wide && heavy_arms { Some(dyadic::encode_cm12h(data)) } else { None }));
        let w7 = s.spawn(|| timed(bank, 13, || if wide { dialect.map(|(b, _)| dyadic::encode_cm12_book(data, b)) } else { None }));
        let w8 = s.spawn(|| timed(bank, 14, || if wide && numeric { Some(dyadic::encode_num(data)) } else { None }));
        let w9 = s.spawn(|| timed(bank, 15, || rect.map(|(st, px)| dyadic::encode_2d(data, st, px))));
        let f8 = timed(bank, 16, || if frozen_arms { Some(dyadic::encode_cm10(data)) } else { None });
        (
            h1.join().expect("arm"),
            h2.join().expect("arm"),
            h3.join().expect("arm"),
            h4.join().expect("arm"),
            h5.join().expect("arm"),
            h6.join().expect("arm"),
            h7.join().expect("arm"),
            f8,
            w1.join().expect("arm"),
            w2.join().expect("arm"),
            w3.join().expect("arm"),
            w4.join().expect("arm"),
            w5.join().expect("arm"),
            w6.join().expect("arm"),
            w7.join().expect("arm"),
            w8.join().expect("arm"),
            w9.join().expect("arm"),
        )
    });
    // the price replays, side by side (each LZ arm re-priced by its own pass)
    let (v11, v12) = std::thread::scope(|s| {
        let h = s.spawn(|| timed(bank, 17, || second_pass(data, toks, v11)));
        let v12 = v12.map(|v| second_pass12(data, toks, v));
        (h.join().expect("pass"), v12)
    });
    let book_model = dialect.map(|(_, m)| m).unwrap_or(MODEL_CM12);
    let roster: [(Option<Vec<u8>>, u8, &str); 17] = [
        (two, MODEL_2D, "2D"),
        (num, MODEL_NUM, "NUM"),
        (v12, MODEL_MIX12, "MIX12"),
        (cm12, MODEL_CM12, "CM12"),
        (bk12, book_model, if book_model == MODEL_CM12_PE { "CM12-PE" } else { "CM12-TTF" }),
        (pr12, MODEL_MIX12P, "MIX12P"),
        (cpr12, MODEL_CM12P, "CM12P"),
        (hv12, MODEL_MIX12H, "MIX12H"),
        (chv12, MODEL_CM12H, "CM12H"),
        (Some(v11), MODEL_MIX11, "MIX11"),
        (Some(cm), MODEL_CM11, "CM11"),
        (pr, MODEL_MIX11P, "MIX11P"),
        (cpr, MODEL_CM11P, "CM11P"),
        (hv, MODEL_MIX11H, "MIX11H"),
        (chv, MODEL_CM11H, "CM11H"),
        (f10, MODEL_MIX10, "MIX10"),
        (fc10, MODEL_CM10, "CM10"),
    ];
    let trace = std::env::var_os("EGG_ARMS").is_some();
    if trace {
        // v14-N2b: the arm that sets the floor. The row's clock is the MAX of
        // these, not their sum, because they run in parallel scoped threads --
        // so this list, slowest first, is the only place a speed lever can be.
        use std::sync::atomic::Ordering::Relaxed;
        const SLOT_NAMES: [&str; ARM_SLOTS] = [
            "MIX11", "CM11", "MIX11P", "CM11P", "MIX11H", "CM11H", "MIX10",
            "MIX12", "CM12", "MIX12P", "CM12P", "MIX12H", "CM12H", "CM12-book",
            "NUM", "2D", "CM10", "MIX11-replay",
        ];
        let mut v: Vec<(u64, &str)> = SLOT_NAMES
            .iter()
            .enumerate()
            .map(|(i, n)| (ARM_MS[bank + i].load(Relaxed), *n))
            .filter(|(ms, _)| *ms > 0)
            .collect();
        v.sort_by_key(|&(ms, _)| std::cmp::Reverse(ms));
        let line: Vec<String> = v.iter().map(|(ms, n)| format!("{}={}ms", n, ms)).collect();
        eprintln!("armtime[{} {} B] slowest first: {}", label, data.len(), line.join(" "));
    }
    let mut line = String::new();
    let mut win: Option<(usize, Vec<u8>, u8)> = None;
    for (o, mb, name) in roster {
        if let Some(v) = o {
            let at = armored_total(v.len());
            if trace {
                line.push_str(&format!(" {}={}", name, v.len()));
            }
            if win.as_ref().is_none_or(|w| at < w.0) {
                win = Some((at, v, mb));
            }
        }
    }
    let (at, v, mb) = win.expect("the roster is never empty");
    if trace {
        eprintln!("arms[{} {} B]:{} -> model {} inner {} armored {}", label, data.len(), line, mb, v.len(), at);
    }
    (v, mb)
}

fn encode_best(src: &[u8], toks: &[token::Tok]) -> (Vec<u8>, u8) {
    let (v8, big) = std::thread::scope(|s| {
        let h = s.spawn(|| big_arms(src, toks, "best"));
        (encode_best_v8(src, toks), h.join().expect("trial"))
    });
    // contingent MIX9, as always: the roster losing to v8 is a loud alarm
    let (bat, vat) = (armored_total(big.0.len()), armored_total(v8.0.len()));
    if bat > vat {
        let v9 = dyadic::encode9(src, toks);
        if armored_total(v9.len()) < vat.min(bat) {
            return (v9, MODEL_MIX9);
        }
    }
    if bat <= vat {
        big
    } else {
        v8
    }
}
/// the dyadic form with the filter decision (WS1): nominate + prune on the
/// sample (filter.rs), then FULL trials -- every surviving candidate and the
/// unfiltered form encoded in parallel std::thread workers. A filtered form
/// is kept only on a >=0.5% win over none; among filtered winners the
/// smallest wins, ties broken by fixed candidate order. Deterministic.
/// the whole trial: the peel arm beside the filtered/plain roster, judged --
/// like everything in this house since v11-M8 -- on the ARMORED total, never on
/// inner bytes. The peel must be STRICTLY lighter to be taken: on a tie the
/// ordinary pipeline keeps the row, because it is the form we can already prove.
fn filtered_transmute(src: &[u8], forced: Option<(u8, u32)>) -> (Vec<u8>, u8, u8, u32) {
    let (peeled, rest) = std::thread::scope(|s| {
        let h = s.spawn(|| if forced.is_none() { peel_arm(src) } else { None });
        let rest = plain_transmute(src, forced);
        (h.join().expect("peel arm"), rest)
    });
    match peeled {
        Some((inner, model)) if armored_total(inner.len()) < armored_total(rest.0.len()) => (inner, model, 0, 0),
        _ => rest,
    }
}

fn plain_transmute(src: &[u8], forced: Option<(u8, u32)>) -> (Vec<u8>, u8, u8, u32) {
    let enc = |data: &[u8]| -> (Vec<u8>, u8) {
        let toks = token::tokenize(data);
        encode_best(data, &toks)
    };
    if let Some((id, param)) = forced {
        let f = filter::apply(src, id, param);
        let (inner, model) = enc(&f);
        return (inner, model, id, param);
    }
    let _ = &enc; // the cheap-trial closure below shadows it for arms
    let cands = filter::nominate(src);
    let filtered_srcs: Vec<Vec<u8>> = cands.iter().map(|c| filter::apply(src, c.id, c.param)).collect();
    // v11-M6: RLE8-unroll is the series' first LENGTH-CHANGING filter -- the
    // filtered length rides in the param so restore knows the decode size
    let cands: Vec<filter::Cand> = cands
        .iter()
        .zip(filtered_srcs.iter())
        .map(|(c, f)| {
            if c.id == filter::FILTER_RLE8 {
                filter::Cand { id: c.id, param: f.len() as u32 }
            } else {
                *c
            }
        })
        .collect();
    // stage 1: pick the filter under the CHEAP v8-cost trial (frozen models,
    // ~10 MB each, all arms in parallel) -- the v8-proven procedure
    let enc8 = |data: &[u8]| -> (Vec<u8>, u8) {
        let toks = token::tokenize(data);
        encode_best_v8(data, &toks)
    };
    let t_stage1 = Instant::now();
    let (plain8, trials8) = std::thread::scope(|s| {
        let handles: Vec<_> = filtered_srcs.iter().map(|f| s.spawn(|| enc8(f))).collect();
        let plain = enc8(src);
        let trials: Vec<(Vec<u8>, u8)> = handles.into_iter().map(|h| h.join().expect("trial thread")).collect();
        (plain, trials)
    });
    let stage1_ms = t_stage1.elapsed().as_millis();
    let mut best: Option<usize> = None; // index into trials8; None = plain
    for (i, t) in trials8.iter().enumerate() {
        let cur = best.map(|b| trials8[b].0.len()).unwrap_or(usize::MAX);
        if t.0.len() as u64 * 1000 <= plain8.0.len() as u64 * 995 && t.0.len() < cur {
            best = Some(i);
        }
    }
    // stage 2: the big v9 models run only on the plain form and the chosen
    // filtered form, two concurrent at a time (bounds big-model encodes; the
    // filter choice under v8 cost is a filed, accepted bias)
    // v11-M4, the no-carries reading (spec.md:81: "Multiply two stalks cell
    // by cell and nothing carries"): the plain and filtered forms share no
    // state, so their big arms run side by side; the sum is the same shape.
    let t_tok = Instant::now();
    let toks_plain = token::tokenize(src);
    let tok_ms = t_tok.elapsed().as_millis();
    let t_stage2 = Instant::now();
    let (plain_big, filt_big) = std::thread::scope(|outer| {
        let hplain = outer.spawn(|| big_arms(src, &toks_plain, "plain"));
        let filt: Option<(Vec<u8>, u8)> = best.map(|i| {
            let f = &filtered_srcs[i];
            let toks = token::tokenize(f);
            let (mut m, mut mmodel) = big_arms(f, &toks, "filtered");
            // sparse-LZ arm, audio-filtered forms only (measured: it won
            // -3,572 B on ring01's order-2 residue and nothing anywhere
            // else, at a cost that broke the speed floor -- so it runs
            // exactly where it earns)
            if cands[i].id == filter::FILTER_W16 || cands[i].id == filter::FILTER_W16O2 {
                let toks8 = token::tokenize_min(f, token::lit_price8(f), 16);
                let mc = dyadic::encode11(f, &toks8);
                if mc.len() < m.len() {
                    m = mc;
                    mmodel = MODEL_MIX11;
                }
            }
            (m, mmodel)
        });
        (hplain.join().expect("plain forms"), filt)
    });
    let stage2_ms = t_stage2.elapsed().as_millis();
    if std::env::var_os("EGG_ARMS").is_some() {
        // v14-N2b: the row's clock was 2.14x its slowest arm on kernel32.dll,
        // so the big roster is NOT what dominates and the stages had to be
        // timed before any speed decision could be more than a guess.
        eprintln!(
            "stages[{} B]: cheap-v8 trial {} ms + tokenize {} ms + BIG ROSTER {} ms (the rest of the row is the peel arm, armor and the write-time round-trip law)",
            src.len(), stage1_ms, tok_ms, stage2_ms
        );
    }
    // final selection: filtered forms still need the 0.5% margin over the
    // best plain form; newest model wins ties (7, 6, then the v8 pick)
    let mut best_plain: (usize, &Vec<u8>, u8) = (armored_total(plain8.0.len()), &plain8.0, plain8.1);
    if armored_total(plain_big.0.len()) <= best_plain.0 {
        best_plain = (armored_total(plain_big.0.len()), &plain_big.0, plain_big.1);
    }
    let best_filt: Option<(usize, &Vec<u8>, u8, usize)> = best.map(|i| {
        let t8 = &trials8[i];
        let mut bf: (usize, &Vec<u8>, u8) = (armored_total(t8.0.len()), &t8.0, t8.1);
        if let Some((mb, mmodel)) = &filt_big {
            let at = armored_total(mb.len());
            if at <= bf.0 {
                bf = (at, mb, *mmodel);
            }
        }
        (bf.0, bf.1, bf.2, i)
    });
    if std::env::var_os("EGG_ARMS").is_some() {
        eprintln!(
            "pick: plain8 {}/{} (armored {}) plain_big {}/{} (armored {}) -> best_plain {} armored {}{}",
            plain8.0.len(), plain8.1, armored_total(plain8.0.len()),
            plain_big.0.len(), plain_big.1, armored_total(plain_big.0.len()),
            best_plain.1.len(), best_plain.0,
            match (&best_filt, &filt_big) {
                (Some((flen, fdata, fmodel, i)), fb) => format!(
                    "; filter {}:{} trials8 {} (armored {}) filt_big {} -> best_filt {}/{} armored {} ({})",
                    cands[*i].id, cands[*i].param, trials8[*i].0.len(), armored_total(trials8[*i].0.len()),
                    fb.as_ref().map(|(b, m)| format!("{}/{}", b.len(), m)).unwrap_or_else(|| "none".into()),
                    fdata.len(), fmodel, flen,
                    if (*flen as u64) * 1000 <= (best_plain.0 as u64) * 995 { "TAKEN" } else { "under the 0.5% margin, plain kept" }
                ),
                _ => "; no filter survived the sample".to_string(),
            }
        );
    }
    match best_filt {
        Some((flen, fdata, fmodel, i)) if (flen as u64) * 1000 <= (best_plain.0 as u64) * 995 => {
            (fdata.clone(), fmodel, cands[i].id, cands[i].param)
        }
        _ => (best_plain.1.clone(), best_plain.2, 0, 0),
    }
}
fn parse_filter_arg(v: &str) -> (u8, u32) {
    match v.split_once(':') {
        Some((a, b)) => (a.parse().expect("--filter id:param"), b.parse().expect("--filter id:param")),
        None => (v.parse().expect("--filter id"), 0),
    }
}
fn model_of(name: Option<&str>) -> u8 {
    match name {
        Some("identity") => MODEL_IDENTITY,
        Some("tokens") => MODEL_TOKENS,
        Some("dyadic") | None => MODEL_DYADIC,
        Some(x) => panic!("unknown --form {}", x),
    }
}

// ---------------------------------------------------------------- CLI
fn xorshift(state: &mut u64) -> u8 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state & 0xff) as u8
}

/// the armor geometry from the flags: `--survive <bytes>` (default 4,096),
/// `--tier <blk>`, `--parity <t>`, `--ct triple|incw|none`, `--judge` (the
/// argmin over the residue placements only), `--no-armor`; `--group` is
/// v11's knob and is ignored with a note
fn rib_from_flags(inner_len: usize, get: &dyn Fn(&str) -> Option<String>, no_armor: bool, judge: bool) -> Result<Rib, String> {
    if no_armor {
        return Ok(rib_no_armor(inner_len));
    }
    let survive: usize = match get("--survive") {
        Some(v) => v.parse().map_err(|_| format!("--survive {}: not a byte count", v))?,
        None => SURVIVE_DEFAULT,
    };
    let force_blk: Option<usize> = match get("--tier") {
        Some(v) => Some(v.parse().map_err(|_| format!("--tier {}: not a square size", v))?),
        None => None,
    };
    let force_t: Option<usize> = match get("--parity") {
        Some(v) => Some(v.parse().map_err(|_| format!("--parity {}: not a square count", v))?),
        None => None,
    };
    let mut rib = rib_search_with(inner_len, survive, force_blk, force_t, !judge)?;
    if let Some(ct) = get("--ct") {
        let mode = CtMode::parse(&ct).ok_or_else(|| format!("--ct {}: triple, incw or none", ct))?;
        // a forced placement keeps the parity count `--parity` pinned, else
        // asks for its own (placement none takes one square more)
        let t = force_t.unwrap_or_else(|| mode.parity_for(rib.blk, survive));
        if t > armor::TMAX {
            return Err(format!("--ct {}: {} parity squares exceed the header byte", ct, t));
        }
        let dummy = Extras { orig_len: 0, orig_fnv: 0, model: 0, filter_id: 0, filter_param: 0 };
        let g = geom(inner_len, rib.blk, t, mode, 0, dummy);
        if g.n > armor::NMAX {
            return Err(format!("--ct {}: the codeword would be {} squares (> 65,535)", ct, g.n));
        }
        rib.mode = mode;
        rib.t = t;
        rib.total = g.total;
        rib.n = g.n;
    }
    Ok(rib)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let get = |name: &str| -> Option<String> {
        args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    let bare: Vec<&String> = {
        let mut out = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if args[i] == "--no-doubles" || args[i] == "--no-armor" || args[i] == "--stats" || args[i] == "--full" || args[i] == "--judge" {
                i += 1;
            } else if args[i].starts_with("--") || args[i] == "-o" {
                i += 2;
            } else {
                out.push(&args[i]);
                i += 1;
            }
        }
        out
    };
    let usage = || {
        eprintln!("usage: eggv14 transmute <file> [-o out.egg14] [--no-armor] [--survive BYTES] [--tier BLK] [--parity T] [--ct triple|incw|none] [--judge]");
        eprintln!("       eggv14 restore <file.egg14 | .egg13 | .egg12 | .egg11 | .egg10 | .egg9 | .egg8> [-o out] [--wound start:len] [--no-doubles]");
        eprintln!("       eggv13 scratch <file> [--len 4096] [--at payload|checks|parity|head|end|<off>] [--survive BYTES] [--tier BLK] [--parity T]");
        eprintln!("       eggv13 audit [--full]   -- the geometry audit, counts printed");
        eprintln!("       eggv13 gcdprobe <file>  -- S1b: the gcd of every 64 KB block, at 8/16/32 bits, before and after each filter");
        eprintln!("       eggv13 bitprobe <file>  -- S1c: the order-1 sequential code length per bit width, per 1 MB region");
        ExitCode::from(2)
    };
    if bare.is_empty() || (bare.len() < 2 && bare[0] != "audit") {
        return usage();
    }
    let cmd = bare[0].as_str();
    if cmd == "audit" {
        return if audit::run(has("--full")) { ExitCode::SUCCESS } else { ExitCode::FAILURE };
    }
    let path = bare[1];

    match cmd {
        "transmute" => {
            let src = fs::read(path).expect("read input");
            let model_req = model_of(get("--form").as_deref());
            let forced = get("--filter").as_deref().map(parse_filter_arg);
            let t0 = Instant::now();
            let (inner, model, fid, fparam) = if model_req == MODEL_DYADIC && !has("--stats") {
                filtered_transmute(&src, forced)
            } else if has("--stats") && model_req == MODEL_DYADIC {
                let (_, _, fid, fparam) = filtered_transmute(&src, forced);
                println!("  stats: filter id {} param {} ({})", fid, fparam,
                    if fid == 0 { "none survived the trial" } else { "kept by full trial" });
                let fsrc = filter::apply(&src, fid, fparam);
                let toks = token::tokenize(&fsrc);
                let (out8, st) = dyadic::encode_stats(&fsrc, &toks, 8, true);
                let s = st.unwrap();
                let total_bits: f64 = s.bits.iter().sum();
                println!("  stats: {} literals, {} matches ({} rep) covering {} B ({:.1}% of input)",
                    s.lits, s.matches, s.reps, s.match_bytes,
                    100.0 * s.match_bytes as f64 / fsrc.len().max(1) as f64);
                for (i, name) in dyadic::CAT_NAMES.iter().enumerate() {
                    println!("  stats: {:10} {:>12.0} bits = {:>9.0} B ({:.1}%)",
                        name, s.bits[i], s.bits[i] / 8.0, 100.0 * s.bits[i] / total_bits);
                }
                let hist: Vec<String> = (0..32).filter(|&i| s.slot_hist[i] > 0)
                    .map(|i| format!("2^{}:{}", i, s.slot_hist[i])).collect();
                println!("  stats: dist slots {}", hist.join(" "));
                let out16 = dyadic::encode(&fsrc, &toks, 16);
                let outmx = dyadic::encode(&fsrc, &toks, 0);
                let out9 = dyadic::encode11(&fsrc, &toks);
                let outc = dyadic::encode_cm11(&fsrc);
                println!("  stats: lit-ctx 2 nibs -> {} B, 4 nibs -> {} B, mixed -> {} B, mix9 -> {} B, cm9 -> {} B (keeping the lightest)",
                    out8.len(), out16.len(), outmx.len(), out9.len(), outc.len());
                if outc.len() <= out8.len() && outc.len() <= out16.len() && outc.len() <= outmx.len() && outc.len() <= out9.len() {
                    (outc, MODEL_CM10, fid, fparam)
                } else if out9.len() <= out8.len() && out9.len() <= out16.len() && out9.len() <= outmx.len() {
                    (out9, MODEL_MIX10, fid, fparam)
                } else if outmx.len() <= out8.len() && outmx.len() <= out16.len() {
                    (outmx, MODEL_MIX, fid, fparam)
                } else if out8.len() <= out16.len() {
                    (out8, MODEL_DYADIC, fid, fparam)
                } else {
                    (out16, MODEL_DYADIC2, fid, fparam)
                }
            } else {
                let (inner, model) = transmute_bytes(&src, model_req);
                (inner, model, 0, 0)
            };
            if get("--group").is_some() {
                println!("  note: --group is v11's knob; v12 has ONE codeword per file and ignores it");
            }
            let rib = match rib_from_flags(inner.len(), &get, has("--no-armor"), has("--judge")) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}: transmute REFUSED -- {}; nothing written", path, e);
                    return ExitCode::FAILURE;
                }
            };
            let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model, filter_id: fid, filter_param: fparam };
            let t_armor = Instant::now();
            let out = armor(&inner, rib.blk, rib.t, rib.mode, ex);
            let armor_ms = t_armor.elapsed().as_millis();
            // the >=2^26 law (big arena, 2026-09-01): the slot wall proved a
            // transmute can write what no restore can read. Any input that
            // reaches the regime where that class of wound lives round-trips
            // in memory BEFORE the artifact is written; failure exits loudly.
            let t_rt = Instant::now();
            let rt_ran = src.len() >= (1 << 26) || fid != 0 || model == MODEL_PEEL;
            if rt_ran {
                // the >=2^26 law, EXTENDED at v11-M6 to length-changing filters
                // and at v13-M1 to EVERY peeled form: a peel is a bijection or
                // it does not ship, and the container it writes must be one the
                // restore path can actually read (v12-M3's eleven minutes)
                // (the RLE member shipped broken for one hour) and at v12-M3 to
                // EVERY filtered form: the mid/side member shipped with a filter
                // id the header verifier refused, and the ledger, not this law,
                // caught it. One decode per filtered transmute is the price.
                match restore_container(&out, &[], true) {
                    Ok((back, _)) if back == src => {
                        println!("  round-trip verified in memory before write (the >=64 MB law)");
                    }
                    Ok(_) => {
                        eprintln!("{}: transmute REFUSED -- in-memory round-trip returned wrong bytes; nothing written", path);
                        return ExitCode::FAILURE;
                    }
                    Err(e) => {
                        eprintln!("{}: transmute REFUSED -- in-memory round-trip failed ({}); nothing written", path, e);
                        return ExitCode::FAILURE;
                    }
                }
            }
            let rt_ms = t_rt.elapsed().as_millis();
            let dst = get("-o").unwrap_or(format!("{}.egg14", path));
            let t_write = Instant::now();
            fs::write(&dst, &out).expect("write output");
            if std::env::var_os("EGG_ARMS").is_some() {
                // v14-N2c control 0.3: the tail of the row, which no instrument
                // has ever named. The round-trip law is a SERIAL full decode and
                // it fires on every filtered or peeled form, not only at 64 MB.
                eprintln!(
                    "tail[{} B]: armor {} ms + round-trip {} ms ({}) + write {} ms",
                    src.len(), armor_ms, rt_ms,
                    if rt_ran { "RAN" } else { "skipped" },
                    t_write.elapsed().as_millis()
                );
            }
            let ms = t0.elapsed().as_millis().max(1);
            println!(
                "{}: {} B -> {} B transmuted -> {} B armored ({:.2}% of input) in {} ms ({:.3} MB/s)",
                path,
                src.len(),
                inner.len(),
                out.len(),
                100.0 * out.len() as f64 / src.len().max(1) as f64,
                ms,
                // v13-M2: this was integer u128 -- `src.len() / ms / 1000` -- and
                // printed 0 for every file under about 1 MB/s, which is most of
                // the corpus. It has been printing a wrong number since v12 and
                // M1 filed it rather than change a gated binary mid-milestone.
                // This is the milestone it was filed for.
                src.len() as f64 / (ms as f64 / 1000.0) / 1.0e6
            );
            if fid != 0 {
                println!("  filter: id {} param {} (kept by full trial, the overlay reading)", fid, fparam);
            }
            if model == MODEL_PEEL {
                if let Ok((pid, rmodel, rraw, rlen, vmodel, vraw)) = peel::read_preamble(&inner) {
                    println!(
                        "  peel: id {} -- recipe {} B raw -> {} B (model {}), values {} B raw -> {} B (model {}); the re-encode was compared to the original before this was written",
                        pid, rraw, rlen, rmodel, vraw, inner.len() - peel::PREAMBLE - rlen, vmodel
                    );
                }
            }
            let gm = geom(inner.len(), rib.blk, rib.t, rib.mode, 0, ex);
            if rib.t == 0 {
                println!("  form: dyadic point, NO armor (headers + residues + conservation hash: damage is detected, nothing is repaired)");
            } else {
                println!(
                    "  form: dyadic point in armor v4 -- ONE GF(2^16) codeword of {} squares x {} B ({} data + {} parity + {} CT; {}); sites at head/mid/end",
                    gm.n, gm.blk, gm.s, gm.t, gm.c, gm.mode.name()
                );
            }
            println!("  promise: {}", promise(&gm));
            if let Some(n) = &rib.note {
                println!("  note: {}", n);
            }
            if rib.t > 0 && !scratch_guaranteed(&gm) {
                println!("  armor floor: t {} < dead({}) = {} -- the 4,096 B contiguous promise does NOT hold at this --parity (said, not hidden)", gm.t, gm.blk, armor::dead_slots(gm.blk));
            }
            println!("  wrote {}", dst);
            ExitCode::SUCCESS
        }
        "gen-prior" => {
            // the book: every named file, argv order (pass them sorted);
            // the TEST PAGE (wubbadub.html of corpus-real) must not be here.
            // M2c(c): `--book PE|TTF --out src/prior_pe.rs <files>` trains a
            // dialect book instead (no corpus file may be named; asserted)
            let files: Vec<&String> = bare[1..].to_vec();
            let mut book = Vec::new();
            for f in &files {
                let fl = f.to_lowercase();
                for banned in ["corpus", "kernel32.dll", "notepad.exe", "zstd.exe", "msgraph.dll", "ntoskrnl.exe", "rustc_driver.dll", "arial", "segoe"] {
                    if fl.contains(banned) {
                        eprintln!("gen-prior REFUSED: {} names a test row or a corpus ({}); the book must not know the exam", f, banned);
                        return ExitCode::FAILURE;
                    }
                }
                book.extend_from_slice(&fs::read(f).expect("read book file"));
                book.push(0);
            }
            let (_, lm) = dyadic::cm11_run(&book);
            match get("--book") {
                Some(prefix) => {
                    let out = get("--out").expect("--book needs --out <file>");
                    let names: Vec<String> = files.iter().map(|f| f.rsplit(['/', '\\']).next().unwrap_or(f).to_string()).collect();
                    let header = format!(
                        "//! GENERATED by `eggv13 gen-prior --book {}` -- the {} dialect book, {} B across {} files,\n//! none of them in any corpus of this repo: {}.\n//! Regenerate and byte-compare; never edit.\n",
                        prefix, prefix, book.len(), files.len(), names.join(" ")
                    );
                    fs::write(&out, lm.export_book(&prefix, &header)).expect("write book");
                    println!("{} book trained on {} B across {} files -> {}", prefix, book.len(), files.len(), out);
                }
                None => {
                    fs::write("src/prior_tab.rs", lm.export_prior()).expect("write prior_tab");
                    println!("prior trained on {} B of book across {} files -> src/prior_tab.rs", book.len(), files.len());
                }
            }
            ExitCode::SUCCESS
        }
        // M3d's S3a: the offset-to-member prober. An INSTRUMENT -- it reads a
        // container's declared layout and prints it, and says NULL rather than
        // guessing when the file is not one it can read.
        "members" => {
            let src = fs::read(path).expect("read input");
            match peel::members(&src) {
                None => println!("{}: no container layout this build can read", path),
                Some(ms) => {
                    println!("{}: {} members ({} B)", path, ms.len(), src.len());
                    let mut deflated = 0usize;
                    for m in &ms {
                        let nom = peel::nominate(&src[m.off..m.off + m.len]);
                        if m.method == 8 {
                            deflated += 1;
                        }
                        println!(
                            "  off {:>10} len {:>10} method {:>2} peel-nominates {} {}",
                            m.off, m.len, m.method, nom, m.name
                        );
                    }
                    let probes = [0usize, src.len() / 2, src.len().saturating_sub(1)];
                    for off in probes {
                        match peel::owner(&ms, off) {
                            Some(i) => println!("  offset {} is owned by member {} ({})", off, i, ms[i].name),
                            None => println!("  offset {} is owned by NO member -- null, not a guess", off),
                        }
                    }
                    println!("  {} of {} members are deflate", deflated, ms.len());
                }
            }
            ExitCode::SUCCESS
        }
        "probe" => {
            let src = fs::read(path).expect("read input");
            println!("{}: filter decision trace", path);
            filter::probe(&src);
            ExitCode::SUCCESS
        }
        // M3b's two INSTRUMENTS. Neither decides anything on its own; each
        // prints a measurement that decides whether an arm gets built.
        "gcdprobe" => {
            let src = fs::read(path).expect("read input");
            sites::gcd_probe(path, &src);
            ExitCode::SUCCESS
        }
        "bitprobe" => {
            let src = fs::read(path).expect("read input");
            sites::bit_probe(path, &src);
            ExitCode::SUCCESS
        }
        "restore" => {
            let cont = fs::read(path).expect("read container");
            let mut wounds = Vec::new();
            let mut i = 0;
            while i < args.len() {
                if args[i] == "--wound" {
                    let (a, b) = args[i + 1].split_once(':').expect("--wound start:len");
                    wounds.push((a.parse().unwrap(), b.parse().unwrap()));
                    i += 1;
                }
                i += 1;
            }
            let t0 = Instant::now();
            let dst = get("-o").unwrap_or_else(|| {
                path.strip_suffix(".egg14")
            .or_else(|| path.strip_suffix(".egg13"))
                    .or_else(|| path.strip_suffix(".egg12"))
                    .or_else(|| path.strip_suffix(".egg11"))
                    .or_else(|| path.strip_suffix(".egg10"))
                    .or_else(|| path.strip_suffix(".egg9"))
                    .or_else(|| path.strip_suffix(".egg8"))
                    .unwrap_or(path)
                    .to_string() + ".out"
            });
            match restore_container(&cont, &wounds, !has("--no-doubles")) {
                Ok((data, rep)) => {
                    fs::write(&dst, &data).expect("write output");
                    println!("{}: {}", path, rep);
                    println!("  restored {} B, conservation hash OK [{} ms]", data.len(), t0.elapsed().as_millis());
                    println!("  wrote {}", dst);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{}: NOT restored ({}) -- nothing written, nothing pretended [{} ms]", path, e, t0.elapsed().as_millis());
                    ExitCode::FAILURE
                }
            }
        }
        "info" => {
            // geometry of an existing container, as JSON -- the drill harness
            // aims its wounds with this instead of re-deriving the layout
            let cont = fs::read(path).expect("read container");
            if is_ancestor(&cont) {
                return info_ancestor(&cont);
            }
            match dearmor(&cont, &[], false) {
                Ok(o) => {
                    let g = &o.g;
                    let off = offsets(g);
                    let p = price(g);
                    let peelj = if o.ex.model == MODEL_PEEL {
                        match peel::read_preamble(&o.inner) {
                            Ok((pid, rmodel, rraw, rlen, vmodel, vraw)) => format!(
                                ",\"peel\":{},\"peel_recipe_raw\":{},\"peel_recipe\":{},\"peel_recipe_model\":{},\"peel_values_raw\":{},\"peel_values\":{},\"peel_values_model\":{}",
                                pid, rraw, rlen, rmodel, vraw, o.inner.len() - peel::PREAMBLE - rlen, vmodel
                            ),
                            Err(_) => ",\"peel\":-1".to_string(),
                        }
                    } else {
                        String::new()
                    };
                    println!(
                        "{{\"total\":{},\"len\":{},\"t\":{},\"block\":{},\"s\":{},\"c\":{},\"n\":{},\"pad\":{},\"mode\":\"{}\",\"mid\":{},\"msize\":{},\"guaranteed\":{},\"h0\":{},\"h1\":{},\"h2\":{},\"slots\":{},\"parity_at\":{},\"ct_at\":{},\"orig_len\":{},\"model\":{},\"filter\":{},\"param\":{},\"price\":{},\"parity\":{},\"ct\":{},\"sites\":{},\"floor_x\":{:.4}{},\"promise\":\"{}\"}}",
                        g.total, g.len, g.t, g.blk, g.s, g.c, g.n, g.pad, g.mode.name(), g.mid, g.msize,
                        scratch_guaranteed(g),
                        off.h0, off.h1, off.h2, off.slot_base,
                        square_off(g, g.parity_at()), if g.c > 0 { square_off(g, g.ct_at()) } else { 0 },
                        o.ex.orig_len, o.ex.model, o.ex.filter_id, o.ex.filter_param,
                        p.total, p.parity, p.ct, p.sites, p.total as f64 / 4096.0, peelj, promise(g)
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("info: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        "scratch" => {
            let src = fs::read(path).expect("read input");
            let len: usize = get("--len").map(|s| s.parse().unwrap()).unwrap_or(4096);
            let model_req = model_of(get("--form").as_deref());
            let t0 = Instant::now();
            let (inner, model, fid, fparam) = if model_req == MODEL_DYADIC {
                filtered_transmute(&src, get("--filter").as_deref().map(parse_filter_arg))
            } else {
                let (i, m) = transmute_bytes(&src, model_req);
                (i, m, 0, 0)
            };
            let rib = match rib_from_flags(inner.len(), &get, has("--no-armor"), has("--judge")) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}: scratch REFUSED -- {}", path, e);
                    return ExitCode::FAILURE;
                }
            };
            let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model, filter_id: fid, filter_param: fparam };
            let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
            let g = geom(inner.len(), rib.blk, rib.t, rib.mode, 0, ex);
            let off = offsets(&g);
            let at: usize = match get("--at").as_deref() {
                None | Some("payload") => off.slot_base + (g.mid * g.blk).saturating_sub(len) / 2,
                Some("checks") => if g.c > 0 { square_off(&g, g.ct_at()) } else { off.m0 },
                Some("parity") => square_off(&g, g.parity_at()),
                Some("head") => 0,
                Some("end") => cont.len().saturating_sub(len),
                Some(x) => x.parse().expect("--at offset"),
            };
            let mut hurt = cont.clone();
            let mut st = 0x1489u64;
            for i in at..(at + len).min(hurt.len()) {
                hurt[i] = xorshift(&mut st);
            }
            println!(
                "{}: {} B original, {} B transmuted, {} B armored (blk {} t {} {}); {} B scratch at {} ({}) [prep {} ms]",
                path, src.len(), inner.len(), cont.len(), rib.blk, rib.t, rib.mode.name(), len, at,
                if at >= off.slot_base && rib.t > 0 { "squares" } else { "head" },
                t0.elapsed().as_millis()
            );
            let mut all_ok = true;
            for (label, wounds) in [
                ("blind (location unknown)", vec![]),
                ("wound location known", vec![(at, len)]),
            ] {
                let t1 = Instant::now();
                match restore_container(&hurt, &wounds, true) {
                    Ok((data, rep)) => {
                        let exact = data == src;
                        println!("  {:26} {} [{} ms]", label, rep, t1.elapsed().as_millis());
                        println!(
                            "  {:26} -> {}",
                            "",
                            if exact { "EXACT (conservation hash verified)" } else { "WRONG DATA -- this must never print" }
                        );
                        if !exact {
                            all_ok = false;
                        }
                    }
                    Err(e) => {
                        println!("  {:26} NOT restored ({}) -- honest, not silent [{} ms]", label, e, t1.elapsed().as_millis());
                        all_ok = false;
                    }
                }
            }
            if all_ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        _ => usage(),
    }
}

/// container -> original bytes, or an honest error. NEVER wrong data with Ok:
/// the armor's own hash gates the transmuted stream, the token decode is
/// defensive, and the conservation hash gates the final bytes.
fn restore_container(cont: &[u8], wounds: &[(usize, usize)], doubles: bool) -> Result<(Vec<u8>, String), String> {
    if is_ancestor(cont) {
        return restore_ancestor(cont, wounds, doubles);
    }
    let o = dearmor(cont, wounds, doubles)?;
    let armor_rep = format!(
        "armor: {} clean, {} dead by residue, {} dead by address, {} located by the codewords, {} rebuilt from the codeword (capacity {}); CT: {}{}{}{}",
        o.t.clean, o.t.by_residue, o.t.by_address, o.t.by_syndrome, o.t.rebuilt, o.t.capacity, o.ct_report,
        if o.padded > 0 { format!("; truncated by {} B, treated as a wound", o.padded) } else { String::new() },
        if o.retried { "; retried down the ladder" } else { "" },
        if o.by_hash { "; data verified intact by FNV-64 (rung C)" } else { "" },
    );
    if !o.hash_ok {
        return Err(format!("transmuted stream damaged beyond armor capacity ({})", armor_rep));
    }
    // the RLE8 unroll changes length; its param names the filtered size
    let dec_len = if o.ex.filter_id == filter::FILTER_RLE8 {
        o.ex.filter_param as usize
    } else {
        o.ex.orig_len as usize
    };
    let data = restore_bytes(&o.inner, dec_len, o.ex.model)?;
    let data = if o.ex.filter_id != 0 {
        filter::undo(&data, o.ex.filter_id, o.ex.filter_param)
    } else {
        data
    };
    if data.len() as u64 != o.ex.orig_len {
        return Err(format!("restored length {} != original {} ({})", data.len(), o.ex.orig_len, armor_rep));
    }
    if fnv64(&data) != o.ex.orig_fnv {
        return Err(format!("conservation check FAILED after clean de-armor: stage bug, not damage ({})", armor_rep));
    }
    Ok((data, armor_rep))
}

/// which family is this container? The head magic alone cannot say -- a
/// wounded head is the FIRST drill -- so the head, the tail, then every
/// offset are asked for a magic+version pair; the first one found decides.
/// Nothing found: the v12 path takes it and refuses honestly.
fn is_ancestor(cont: &[u8]) -> bool {
    fn family(b: &[u8]) -> Option<bool> {
        if b.len() < 5 {
            return None;
        }
        match (&b[0..4], b[4]) {
            (b"EG14", 8) => Some(false),
        (b"EG13", 7) => Some(false),
            (b"EG12", 6) => Some(false),
            (b"EG11", 5) | (b"EG10", 4) | (b"EGG9", 3) | (b"EGG8", 2) => Some(true),
            _ => None,
        }
    }
    if let Some(a) = family(cont) {
        return a;
    }
    if cont.len() >= 64 {
        if let Some(a) = family(&cont[cont.len() - 64..]) {
            return a;
        }
    }
    (0..cont.len().saturating_sub(4)).find_map(|i| family(&cont[i..])).unwrap_or(false)
}

/// the ancestor path: .egg8/.egg9/.egg10/.egg11 containers de-armor through
/// v11's armor v3 VERBATIM (armor11.rs, never edited), then the same model
/// ladder; EXACT restore of every elder is a gate, not a courtesy
fn restore_ancestor(cont: &[u8], wounds: &[(usize, usize)], doubles: bool) -> Result<(Vec<u8>, String), String> {
    let o = armor11::dearmor(cont, wounds, doubles)?;
    let armor_rep = format!(
        "armor v3 (ancestor {}): {} clean, {} bit-fixed, {} low-confidence, {} rebuilt from parity, {} beyond capacity; CT: {}{}{}",
        String::from_utf8_lossy(&cont[0..4]),
        o.t.clean, o.t.bitfixed, o.t.bitfixed2, o.t.rebuilt, o.t.detected, o.ct_report,
        if o.padded > 0 { format!("; truncated by {} B, treated as a wound", o.padded) } else { String::new() },
        if o.retried { "; retried down the ladder" } else { "" },
    );
    if !o.hash_ok {
        return Err(format!("transmuted stream damaged beyond armor capacity ({})", armor_rep));
    }
    let dec_len = if o.ex.filter_id == filter::FILTER_RLE8 { o.ex.filter_param as usize } else { o.ex.orig_len as usize };
    let data = restore_bytes(&o.inner, dec_len, o.ex.model)?;
    let data = if o.ex.filter_id != 0 { filter::undo(&data, o.ex.filter_id, o.ex.filter_param) } else { data };
    if data.len() as u64 != o.ex.orig_len {
        return Err(format!("restored length {} != original {} ({})", data.len(), o.ex.orig_len, armor_rep));
    }
    if fnv64(&data) != o.ex.orig_fnv {
        return Err(format!("conservation check FAILED after clean de-armor: stage bug, not damage ({})", armor_rep));
    }
    Ok((data, armor_rep))
}

/// `info` for an ancestor container: v11's geometry, v11's JSON shape
fn info_ancestor(cont: &[u8]) -> ExitCode {
    match armor11::dearmor(cont, &[], false) {
        Ok(o) => {
            let (blk_i, gmax_i) = if &cont[0..4] == b"EG11" {
                (armor11::TIERS[(cont[53] as usize).min(armor11::TIERS.len() - 1)].0, armor11::GMAX5)
            } else {
                (armor11::BLOCK, armor11::GMAX)
            };
            let g = armor11::geom(o.inner.len(), blk_i, gmax_i, cont[5] as usize, cont[30] as usize, 0, o.ex);
            let off = armor11::offsets(&g);
            let ng1 = if g.s == 0 { 0 } else { g.s.div_ceil(g.g.max(1)) };
            let ng2 = if g.c == 0 { 0 } else { g.c.div_ceil(g.g) };
            println!(
                "{{\"total\":{},\"len\":{},\"g\":{},\"t\":{},\"block\":{},\"s\":{},\"nsq\":{},\"nsq2\":{},\"nslots\":{},\"mid\":{},\"msize\":{},\"ct_triple\":{},\"ng1\":{},\"ng2\":{},\"ngtotal\":{},\"guaranteed\":{},\"h0\":{},\"h1\":{},\"h2\":{},\"slots\":{},\"orig_len\":{},\"model\":{},\"filter\":{},\"param\":{},\"ancestor\":\"{}\"}}",
                g.total, g.len, g.g, g.t, g.blk, g.s, g.nsq, g.nsq2,
                g.nslots, g.mid, g.msize, g.ct_triple, ng1, ng2, ng1 + ng2,
                armor11::scratch_guaranteed(&g),
                off.h0, off.h1, off.h2, off.slot_base,
                o.ex.orig_len, o.ex.model, o.ex.filter_id, o.ex.filter_param,
                String::from_utf8_lossy(&cont[0..4])
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("info: {}", e);
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;
    /// v12-M3: the stereo mid/side member round-trips through the WHOLE
    /// pipeline (filter -> trial -> armor -> parse_header -> dearmor -> undo)
    /// -- the header verifier must accept the new filter ids (the defect the
    /// wav ledger caught on 2026-09-02)
    #[test]
    fn stereo_member_full_pipeline_round_trip() {
        // a small correlated stereo WAVE: 16-bit, 2 channels, 22,050 Hz
        let frames = 6000usize;
        let mut pcm = Vec::with_capacity(frames * 4);
        let mut st = 0x9E3779B97F4A7C15u64;
        for i in 0..frames {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            let base = ((i as f64 * 0.07).sin() * 9000.0) as i16;
            let l = base.wrapping_add((st & 63) as i16);
            let r = base.wrapping_add(((st >> 8) & 31) as i16).wrapping_sub(200);
            pcm.extend_from_slice(&l.to_le_bytes());
            pcm.extend_from_slice(&r.to_le_bytes());
        }
        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&((36 + pcm.len()) as u32).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
        wav.extend_from_slice(&2u16.to_le_bytes()); // channels
        wav.extend_from_slice(&22050u32.to_le_bytes());
        wav.extend_from_slice(&(22050u32 * 4).to_le_bytes());
        wav.extend_from_slice(&4u16.to_le_bytes()); // block align
        wav.extend_from_slice(&16u16.to_le_bytes()); // bits
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(pcm.len() as u32).to_le_bytes());
        wav.extend_from_slice(&pcm);
        // every new filter id passes through here: this is the test that
        // catches an un-bumped FILTER_MAX (v12-M3's eleven minutes)
        for (fid, fp) in [(filter::FILTER_MS1, 2u32), (filter::FILTER_MS2, 2)] {
            let (inner, model, id, param) = filtered_transmute(&wav, Some((fid, fp)));
            assert_eq!((id, param), (fid, fp));
            let ex = Extras { orig_len: wav.len() as u64, orig_fnv: fnv64(&wav), model, filter_id: id, filter_param: param };
            let rib = rib_search(inner.len(), SURVIVE_DEFAULT, None, None).unwrap();
            let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
            let (back, _) = restore_container(&cont, &[], true).expect("filtered member restore");
            assert_eq!(back, wav, "filter {} round trip", fid);
            // and the header verifier must read it (the M3 defect)
            assert!(dearmor(&cont, &[], false).is_ok(), "parse_header refused filter id {}", fid);
        }
        // the free trial must ALSO be able to pick a stereo filter and restore
        let (inner, model, id, param) = filtered_transmute(&wav, None);
        let ex = Extras { orig_len: wav.len() as u64, orig_fnv: fnv64(&wav), model, filter_id: id, filter_param: param };
        let rib = rib_search(inner.len(), SURVIVE_DEFAULT, None, None).unwrap();
        let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
        let (back, _) = restore_container(&cont, &[], true).expect("free-trial restore");
        assert_eq!(back, wav);
    }
    /// v13-M1: the PEELED form's whole pipeline (peel -> THE LAW's re-encode
    /// -> trial -> armor -> parse_header -> dearmor -> re-spell) round-trips,
    /// and a peel that must be refused keeps its file. v12-M3's eleven minutes
    /// began with a header verifier that refused a form the encoder had just
    /// written; the peel gets that test on day one.
    #[test]
    fn peel_member_full_pipeline_round_trip() {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wallpaper.jpg");
        let src = std::fs::read(p).expect("corpus-real/wallpaper.jpg present");
        let (inner, model, fid, fparam) = filtered_transmute(&src, None);
        assert_eq!(model, MODEL_PEEL, "the trial must take the peeled form on a baseline JPEG");
        assert_eq!((fid, fparam), (0, 0), "a peel is not a filter");
        let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model, filter_id: fid, filter_param: fparam };
        let rib = rib_search(inner.len(), SURVIVE_DEFAULT, None, None).unwrap();
        let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
        assert!(dearmor(&cont, &[], false).is_ok(), "parse_header refused a peeled form");
        let (back, _) = restore_container(&cont, &[], true).expect("peeled restore");
        assert_eq!(back, src, "the peeled form did not re-spell its file");
        // refuse, do not guess: a progressive frame is not peeled, and the file
        // still goes through the ordinary pipeline and restores EXACT
        let mut prog = src.clone();
        let mut i = 2usize;
        while i + 3 < prog.len() {
            if prog[i] != 0xFF {
                break;
            }
            let m = prog[i + 1];
            if m == 0xD8 || m == 0x01 || (0xD0..=0xD7).contains(&m) {
                i += 2;
                continue;
            }
            let l = ((prog[i + 2] as usize) << 8) | prog[i + 3] as usize;
            if m == 0xC0 {
                prog[i + 1] = 0xC2;
                break;
            }
            i += 2 + l;
        }
        assert!(peel_arm(&prog).is_none(), "a progressive frame must not be peeled");
    }

    /// v13-M2: the DEFLATE peel's whole pipeline, on a member built here so the
    /// test carries its own fixture. The trap the charter names is an artifact
    /// no restore can read; every peel format gets this test on day one, and
    /// this one also proves the OTHER shape of peel -- values that ride the
    /// ordinary roster rather than a model of their own.
    #[test]
    fn deflate_member_full_pipeline_round_trip() {
        // a real gzip member: the corpus HTML, deflated by this build's own
        // re-speller from a parse of a stored-block member, is not a test --
        // so the member under test is the save's, sliced to keep it quick
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-big/aoe4-autosave.sav");
        let src = std::fs::read(p).expect("corpus-big/aoe4-autosave.sav present");
        // the whole save is a minute of modelling; the pipeline law is proved on
        // the parse and the container, so take a member of the same file that
        // ends on a block boundary: the first 2 blocks re-spelled as their own
        // gzip member
        let d = deflate::peel(&src).expect("the save peels");
        assert_eq!(d.wrap, deflate::WRAP_GZIP);
        // the peel is a bijection on this file, checked here and not assumed
        assert!(deflate::respell(&d).expect("re-spell") == src, "the save did not re-spell");
        // and the serialised recipe survives its own layout
        let b = deflate::blob(&d);
        let l = deflate::layout(&b).expect("layout");
        assert_eq!(l.values_len, d.values.len() as u64);
        assert_eq!(l.nmatch, d.lens.len());
        // the container law, on a member small enough to model in a test: a
        // gzip of the corpus HTML made by this build's own stored-block writer
        let html = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-real/wubbadub.html")).expect("html");
        let gz = {
            // one stored block per 65,535 bytes, then the gzip trailer
            let mut out = vec![0x1F, 0x8B, 0x08, 0x00, 0, 0, 0, 0, 0x00, 0xFF];
            let mut i = 0usize;
            while i < html.len() {
                let n = (html.len() - i).min(65_535);
                out.push(u8::from(i + n >= html.len()));
                out.extend_from_slice(&(n as u16).to_le_bytes());
                out.extend_from_slice(&(!(n as u16)).to_le_bytes());
                out.extend_from_slice(&html[i..i + n]);
                i += n;
            }
            let mut crc = 0xFFFF_FFFFu32;
            for &b in &html {
                crc ^= b as u32;
                for _ in 0..8 {
                    crc = (crc >> 1) ^ (0xEDB8_8320 & (0u32.wrapping_sub(crc & 1)));
                }
            }
            out.extend_from_slice(&(!crc).to_le_bytes());
            out.extend_from_slice(&(html.len() as u32).to_le_bytes());
            out
        };
        // the trial owns the verdict -- on a small stored-block member the peel
        // and the ordinary pipeline see nearly the same bytes and the argmin can
        // go either way. The LAW under test is that the PEELED form, whenever it
        // is built, is one the restore path can read, so the container below is
        // built from the peel arm's own output and not from the trial's winner.
        let (inner, model) = peel_arm(&gz).expect("a gzip member must produce a peeled form");
        assert_eq!(model, MODEL_PEEL);
        let (fid, fparam) = (0u8, 0u32);
        let ex = Extras { orig_len: gz.len() as u64, orig_fnv: fnv64(&gz), model, filter_id: fid, filter_param: fparam };
        let rib = rib_search(inner.len(), SURVIVE_DEFAULT, None, None).unwrap();
        let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
        assert!(dearmor(&cont, &[], false).is_ok(), "parse_header refused a deflate-peeled form");
        let (back, _) = restore_container(&cont, &[], true).expect("peeled restore");
        assert_eq!(back, gz, "the deflate-peeled form did not re-spell its file");
        // refuse, do not guess: a member with junk after it is not one member
        assert!(peel_arm(&[&gz[..], b"trailing"].concat()).is_none(), "a member with a tail must not be peeled");
    }

    /// M3a: the recipe's FIFTH section -- the sparse spelling list -- survives
    /// the sectioned model and the whole container, on a member that could not
    /// be peeled AT ALL before M3a (`deflate.rs` refused the file outright).
    #[test]
    fn respelled_258_member_full_pipeline_round_trip() {
        let gz = deflate::mk_gzip_284();
        // the sectioned recipe model, five sections now, round-tripped directly
        // -- on a member this small the argmin may keep the blob raw, so the
        // five-section path is exercised here and not left to the trial
        let d = deflate::peel(&gz).expect("the 284-spelled member peels");
        assert_eq!(d.resp, vec![0u32, 2]);
        let blob = deflate::blob(&d);
        let m = encode_drecipe(&blob);
        assert!(!m.is_empty(), "the recipe lays out");
        assert_eq!(m[0], 5, "five sections");
        assert_eq!(decode_drecipe(&m, blob.len()).expect("drecipe"), blob, "the fifth section did not come back");
        // and the whole container, through the peel arm's own output
        let (inner, model) = peel_arm(&gz).expect("a 284-spelled member must produce a peeled form");
        assert_eq!(model, MODEL_PEEL);
        let ex = Extras { orig_len: gz.len() as u64, orig_fnv: fnv64(&gz), model, filter_id: 0, filter_param: 0 };
        let rib = rib_search(inner.len(), SURVIVE_DEFAULT, None, None).unwrap();
        let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
        let (back, _) = restore_container(&cont, &[], true).expect("peeled restore");
        assert_eq!(back, gz, "the 284 spelling did not survive the container");
    }

    /// the M6 lesson as a permanent gate: the length-changing filter's whole
    /// pipeline (filter -> trial -> armor -> dearmor -> undo) round-trips
    #[test]
    fn rle_member_full_pipeline_round_trip() {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus-rle/gradient-rle8.bmp");
        let src = std::fs::read(p).expect("corpus-rle member present");
        let (inner, model, fid, fparam) = filtered_transmute(&src, None);
        // the trial owns the verdict (post-M8 the heavy arms sometimes take
        // the run-coded form outright); the LAW under test is the round-trip
        // through whichever form won -- and the forced-filter path below
        // proves the length-changing unroll itself in the same breath
        let f = filter::apply(&src, filter::FILTER_RLE8, 0);
        let (fi, fm) = {
            let toks = token::tokenize(&f);
            encode_best(&f, &toks)
        };
        let fex = Extras {
            orig_len: src.len() as u64,
            orig_fnv: fnv64(&src),
            model: fm,
            filter_id: filter::FILTER_RLE8,
            filter_param: f.len() as u32,
        };
        let frib = rib_search(fi.len(), SURVIVE_DEFAULT, None, None).unwrap();
        let fcont = armor(&fi, frib.blk, frib.t, frib.mode, fex);
        let (fback, _) = restore_container(&fcont, &[], true).expect("forced-RLE restore");
        assert_eq!(fback, src, "forced unroll round-trip");
        let rib = rib_search(inner.len(), SURVIVE_DEFAULT, None, None).unwrap();
        let ex = Extras {
            orig_len: src.len() as u64,
            orig_fnv: fnv64(&src),
            model,
            filter_id: fid,
            filter_param: fparam,
        };
        let cont = armor(&inner, rib.blk, rib.t, rib.mode, ex);
        let (back, _) = restore_container(&cont, &[], true).expect("restore");
        assert_eq!(back, src);
    }
}
