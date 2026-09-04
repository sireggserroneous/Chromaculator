//! eggv11 -- the Transmuter.
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
mod audit;
mod dyadic;
mod filter;
mod mix10;
mod mix11;
mod prior_tab;
mod state_tab;
mod mix9;
mod squash_tab;
mod structure;
mod token;

use armor::{armor, dearmor, fnv64, geom, offsets, rib_policy, scratch_guaranteed, slot_off, slot_order, Extras, BLOCK, GMAX, GMAX5, TIERS};
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
        m => Err(format!("unknown model byte {} -- newer transmuter?", m)),
    }
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
    let rib = rib_policy(inner_len);
    let dummy = Extras { orig_len: 0, orig_fnv: 0, model: 0, filter_id: 0, filter_param: 0 };
    geom(inner_len, rib.blk, GMAX5, rib.g, rib.t, 0, dummy).total
}

fn big_arms(data: &[u8], toks: &[token::Tok]) -> (Vec<u8>, u8) {
    let prior_arms = data.len() < (1 << 20);
    let heavy_arms = data.len() >= 512 * 1024;
    let frozen_arms = data.len() < (16 << 20); // raised from 4 MB after the
                                               // synthetic server-log breached
                                               // <=min in the 4-16 MB gap
    let (v11, cm, pr, cpr, hv, chv, f10, fc10) = std::thread::scope(|s| {
        let h1 = s.spawn(|| dyadic::encode11(data, toks));
        let h2 = s.spawn(|| dyadic::encode_cm11(data));
        let h3 = s.spawn(|| if prior_arms { Some(dyadic::encode11p(data, toks)) } else { None });
        let h4 = s.spawn(|| if prior_arms { Some(dyadic::encode_cm11p(data)) } else { None });
        let h5 = s.spawn(|| if heavy_arms { Some(dyadic::encode11h(data, toks)) } else { None });
        let h6 = s.spawn(|| if heavy_arms { Some(dyadic::encode_cm11h(data)) } else { None });
        let h7 = s.spawn(|| if frozen_arms { Some(dyadic::encode10(data, toks)) } else { None });
        let f8 = if frozen_arms { Some(dyadic::encode_cm10(data)) } else { None };
        (
            h1.join().expect("arm"),
            h2.join().expect("arm"),
            h3.join().expect("arm"),
            h4.join().expect("arm"),
            h5.join().expect("arm"),
            h6.join().expect("arm"),
            h7.join().expect("arm"),
            f8,
        )
    });
    let v11 = second_pass(data, toks, v11);
    let mut win = (armored_total(v11.len()), v11, MODEL_MIX11);
    let others: [(Option<Vec<u8>>, u8); 7] = [
        (Some(cm), MODEL_CM11),
        (pr, MODEL_MIX11P),
        (cpr, MODEL_CM11P),
        (hv, MODEL_MIX11H),
        (chv, MODEL_CM11H),
        (f10, MODEL_MIX10),
        (fc10, MODEL_CM10),
    ];
    for (o, mb) in others {
        if let Some(v) = o {
            let at = armored_total(v.len());
            if at < win.0 {
                win = (at, v, mb);
            }
        }
    }
    (win.1, win.2)
}

fn encode_best(src: &[u8], toks: &[token::Tok]) -> (Vec<u8>, u8) {
    let (v8, big) = std::thread::scope(|s| {
        let h = s.spawn(|| big_arms(src, toks));
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
fn filtered_transmute(src: &[u8], forced: Option<(u8, u32)>) -> (Vec<u8>, u8, u8, u32) {
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
    let (plain8, trials8) = std::thread::scope(|s| {
        let handles: Vec<_> = filtered_srcs.iter().map(|f| s.spawn(|| enc8(f))).collect();
        let plain = enc8(src);
        let trials: Vec<(Vec<u8>, u8)> = handles.into_iter().map(|h| h.join().expect("trial thread")).collect();
        (plain, trials)
    });
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
    let toks_plain = token::tokenize(src);
    let (plain_big, filt_big) = std::thread::scope(|outer| {
        let hplain = outer.spawn(|| big_arms(src, &toks_plain));
        let filt: Option<(Vec<u8>, u8)> = best.map(|i| {
            let f = &filtered_srcs[i];
            let toks = token::tokenize(f);
            let (mut m, mut mmodel) = big_arms(f, &toks);
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
            if args[i] == "--no-doubles" || args[i] == "--no-armor" || args[i] == "--stats" || args[i] == "--full" {
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
        eprintln!("usage: eggv11 transmute <file> [-o out.egg10] [--no-armor] [--group N] [--parity T]");
        eprintln!("       eggv11 restore <file.egg10 | .egg9 | .egg8> [-o out] [--wound start:len] [--no-doubles]");
        eprintln!("       eggv11 scratch <file> [--len 4096] [--at payload|checks|head|end|<off>]");
        eprintln!("       eggv11 audit [--full]   -- the geometry audit, counts printed");
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
            let rib = rib_policy(inner.len());
            let (grp, tpar) = if has("--no-armor") {
                (0, 0)
            } else {
                (
                    get("--group").map(|s| s.parse().unwrap()).unwrap_or(rib.g),
                    get("--parity").map(|s| s.parse().unwrap()).unwrap_or(rib.t),
                )
            };
            let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model, filter_id: fid, filter_param: fparam };
            let out = armor(&inner, rib.blk, grp, tpar, ex);
            // the >=2^26 law (big arena, 2026-09-01): the slot wall proved a
            // transmute can write what no restore can read. Any input that
            // reaches the regime where that class of wound lives round-trips
            // in memory BEFORE the artifact is written; failure exits loudly.
            if src.len() >= (1 << 26) || fid == filter::FILTER_RLE8 {
                // the >=2^26 law, EXTENDED at v11-M6: any artifact behind a
                // length-changing filter round-trips in memory before write
                // (the RLE member shipped broken for one hour; never again)
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
            let dst = get("-o").unwrap_or(format!("{}.egg11", path));
            fs::write(&dst, &out).expect("write output");
            let ms = t0.elapsed().as_millis().max(1);
            println!(
                "{}: {} B -> {} B transmuted -> {} B armored ({:.2}% of input) in {} ms ({} MB/s)",
                path,
                src.len(),
                inner.len(),
                out.len(),
                100.0 * out.len() as f64 / src.len().max(1) as f64,
                ms,
                src.len() as u128 / ms / 1000
            );
            if fid != 0 {
                println!("  filter: id {} param {} (kept by full trial, the overlay reading)", fid, fparam);
            }
            if grp == 0 {
                println!("  form: dyadic point, NO armor (headers + conservation hash only)");
            } else {
                let gm = geom(inner.len(), rib.blk, GMAX5, grp, tpar, 0, ex);
                println!(
                    "  form: dyadic point in armor v3 ({} B squares -- the wide grid; {} RS parities per {} squares; replicas at head/mid/end)",
                    gm.blk, tpar, grp
                );
                if !scratch_guaranteed(&gm) {
                    println!("  armor floor: payload too small for the 4 KB-scratch guarantee (parity for 9 dead slots is ~4.6-6.1 KB of physics; said, not hidden)");
                }
            }
            println!("  wrote {}", dst);
            ExitCode::SUCCESS
        }
        "gen-prior" => {
            // the book: every named file, argv order (pass them sorted);
            // the TEST PAGE (wubbadub.html of corpus-real) must not be here
            let mut book = Vec::new();
            for f in args.iter().filter(|a| *a != "gen-prior") {
                book.extend_from_slice(&fs::read(f).expect("read book file"));
                book.push(0);
            }
            let (_, lm) = dyadic::cm11_run(&book);
            fs::write("src/prior_tab.rs", lm.export_prior()).expect("write prior_tab");
            println!("prior trained on {} B of book across {} files -> src/prior_tab.rs", book.len(), args.len());
            ExitCode::SUCCESS
        }
        "probe" => {
            let src = fs::read(path).expect("read input");
            println!("{}: filter decision trace", path);
            filter::probe(&src);
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
                path.strip_suffix(".egg11")
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
            match dearmor(&cont, &[], false) {
                Ok(o) => {
                    let (blk_i, gmax_i) = if &cont[0..4] == b"EG11" {
                        (TIERS[(cont[53] as usize).min(TIERS.len() - 1)].0, GMAX5)
                    } else {
                        (BLOCK, GMAX)
                    };
                    let g = geom(o.inner.len(), blk_i, gmax_i, {
                        // recover g,t from the header bytes directly
                        cont[5] as usize
                    }, cont[30] as usize, 0, o.ex);
                    let off = offsets(&g);
                    let ng1 = if g.s == 0 { 0 } else { g.s.div_ceil(g.g.max(1)) };
                    let ng2 = if g.c == 0 { 0 } else { g.c.div_ceil(g.g) };
                    println!(
                        "{{\"total\":{},\"len\":{},\"g\":{},\"t\":{},\"block\":{},\"s\":{},\"nsq\":{},\"nsq2\":{},\"nslots\":{},\"mid\":{},\"msize\":{},\"ct_triple\":{},\"ng1\":{},\"ng2\":{},\"ngtotal\":{},\"guaranteed\":{},\"h0\":{},\"h1\":{},\"h2\":{},\"slots\":{},\"orig_len\":{},\"model\":{},\"filter\":{},\"param\":{}}}",
                        g.total, g.len, g.g, g.t, g.blk, g.s, g.nsq, g.nsq2,
                        g.nslots, g.mid, g.msize, g.ct_triple, ng1, ng2, ng1 + ng2,
                        scratch_guaranteed(&g),
                        off.h0, off.h1, off.h2, off.slot_base,
                        o.ex.orig_len, o.ex.model, o.ex.filter_id, o.ex.filter_param
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
            let rib = rib_policy(inner.len());
            let grp: usize = get("--group").map(|s| s.parse().unwrap()).unwrap_or(rib.g);
            let tpar: usize = get("--parity").map(|s| s.parse().unwrap()).unwrap_or(rib.t);
            let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model, filter_id: fid, filter_param: fparam };
            let cont = armor(&inner, rib.blk, grp, tpar, ex);
            let g = geom(inner.len(), rib.blk, GMAX5, grp, tpar, 0, ex);
            let off = offsets(&g);
            let at: usize = match get("--at").as_deref() {
                None | Some("payload") => off.slot_base + (g.mid * g.blk).saturating_sub(len) / 2,
                Some("checks") => slot_order(&g)
                    .iter()
                    .position(|sl| sl.level == 1)
                    .map(|j| slot_off(&g, j))
                    .unwrap_or(off.m0),
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
                "{}: {} B original, {} B transmuted, {} B armored (G{} T{}); {} B scratch at {} ({}) [prep {} ms]",
                path, src.len(), inner.len(), cont.len(), grp, tpar, len, at,
                if at >= off.slot_base && grp > 0 { "slots" } else { "head" },
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
    let o = dearmor(cont, wounds, doubles)?;
    let armor_rep = format!(
        "armor: {} clean, {} bit-fixed, {} low-confidence, {} rebuilt from parity, {} beyond capacity; CT: {}{}{}",
        o.t.clean, o.t.bitfixed, o.t.bitfixed2, o.t.rebuilt, o.t.detected, o.ct_report,
        if o.padded > 0 { format!("; truncated by {} B, treated as a wound", o.padded) } else { String::new() },
        if o.retried { "; retried down the ladder" } else { "" },
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

#[cfg(test)]
mod pipeline_tests {
    use super::*;
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
        let frib = rib_policy(fi.len());
        let fcont = armor(&fi, frib.blk, frib.g, frib.t, fex);
        let (fback, _) = restore_container(&fcont, &[], true).expect("forced-RLE restore");
        assert_eq!(fback, src, "forced unroll round-trip");
        let rib = rib_policy(inner.len());
        let ex = Extras {
            orig_len: src.len() as u64,
            orig_fnv: fnv64(&src),
            model,
            filter_id: fid,
            filter_param: fparam,
        };
        let cont = armor(&inner, rib.blk, rib.g, rib.t, ex);
        let (back, _) = restore_container(&cont, &[], true).expect("restore");
        assert_eq!(back, src);
    }
}
