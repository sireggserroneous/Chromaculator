//! eggv7 -- the Transmuter.
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
mod dyadic;
mod token;

use armor::{armor, dearmor, fnv64, geom, offsets, Extras, BLOCK};
use std::env;
use std::fs;
use std::process::ExitCode;
use std::time::Instant;

// model byte in the header: which form the payload is in
const MODEL_IDENTITY: u8 = 1; // bytes as they came (armor-only, M0 scaffold)
const MODEL_TOKENS: u8 = 2; // match/literal sequences, raw (M1 stage measure)
const MODEL_DYADIC: u8 = 3; // one dyadic point, literal context 8 bits (2 nibs)
const MODEL_DYADIC2: u8 = 4; // one dyadic point, literal context 16 bits (4 nibs)

/// rib policy by artifact (transmuted stream) size, from the plan
fn rib_policy(inner_len: usize) -> (usize, usize) {
    if inner_len < 64 * 1024 {
        (8, 2)
    } else if inner_len < 1024 * 1024 {
        (32, 2)
    } else {
        (126, 2)
    }
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
        MODEL_DYADIC | MODEL_DYADIC2 => {
            let toks = token::tokenize(src);
            let a = dyadic::encode(src, &toks, 8);
            let b = dyadic::encode(src, &toks, 16);
            if a.len() <= b.len() { (a, MODEL_DYADIC) } else { (b, MODEL_DYADIC2) }
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
        m => Err(format!("unknown model byte {} -- newer transmuter?", m)),
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
            if args[i] == "--no-doubles" || args[i] == "--no-armor" || args[i] == "--stats" {
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
        eprintln!("usage: eggv7 transmute <file> [-o out.egg7] [--no-armor] [--group N] [--parity T]");
        eprintln!("       eggv7 restore <file.egg7> [-o out] [--wound start:len] [--no-doubles]");
        eprintln!("       eggv7 scratch <file> [--len 4096] [--at payload|checks|head|end|<off>]");
        ExitCode::from(2)
    };
    if bare.len() < 2 {
        return usage();
    }
    let cmd = bare[0].as_str();
    let path = bare[1];

    match cmd {
        "transmute" => {
            let src = fs::read(path).expect("read input");
            let model_req = model_of(get("--form").as_deref());
            let t0 = Instant::now();
            let (inner, model) = if has("--stats") && model_req == MODEL_DYADIC {
                let toks = token::tokenize(&src);
                let (out8, st) = dyadic::encode_stats(&src, &toks, 8, true);
                let s = st.unwrap();
                let total_bits: f64 = s.bits.iter().sum();
                println!("  stats: {} literals, {} matches ({} rep) covering {} B ({:.1}% of input)",
                    s.lits, s.matches, s.reps, s.match_bytes,
                    100.0 * s.match_bytes as f64 / src.len().max(1) as f64);
                for (i, name) in dyadic::CAT_NAMES.iter().enumerate() {
                    println!("  stats: {:10} {:>12.0} bits = {:>9.0} B ({:.1}%)",
                        name, s.bits[i], s.bits[i] / 8.0, 100.0 * s.bits[i] / total_bits);
                }
                let hist: Vec<String> = (0..26).filter(|&i| s.slot_hist[i] > 0)
                    .map(|i| format!("2^{}:{}", i, s.slot_hist[i])).collect();
                println!("  stats: dist slots {}", hist.join(" "));
                let out16 = dyadic::encode(&src, &toks, 16);
                println!("  stats: lit-ctx 2 nibs -> {} B, 4 nibs -> {} B (keeping the lighter)",
                    out8.len(), out16.len());
                if out8.len() <= out16.len() { (out8, MODEL_DYADIC) } else { (out16, MODEL_DYADIC2) }
            } else {
                transmute_bytes(&src, model_req)
            };
            let (gd, td) = rib_policy(inner.len());
            let (grp, tpar) = if has("--no-armor") {
                (0, 0)
            } else {
                (
                    get("--group").map(|s| s.parse().unwrap()).unwrap_or(gd),
                    get("--parity").map(|s| s.parse().unwrap()).unwrap_or(td),
                )
            };
            let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model };
            let out = armor(&inner, grp, tpar, ex);
            let dst = get("-o").unwrap_or(format!("{}.egg7", path));
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
            if grp == 0 {
                println!("  form: dyadic point, NO armor (headers + conservation hash only)");
            } else {
                println!(
                    "  form: dyadic point in eggv6 armor (residues; {} RS parities per {} squares; 3 voted headers)",
                    tpar, grp
                );
            }
            println!("  wrote {}", dst);
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
                path.strip_suffix(".egg7").unwrap_or(path).to_string() + ".out"
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
                    let g = geom(o.inner.len(), {
                        // recover g,t from the header bytes directly
                        cont[5] as usize
                    }, cont[30] as usize, 0, o.ex);
                    let off = offsets(&g);
                    println!(
                        "{{\"total\":{},\"len\":{},\"g\":{},\"t\":{},\"block\":{},\"s\":{},\"nsq\":{},\"nsq2\":{},\"h0\":{},\"slots2\":{},\"h1\":{},\"slots\":{},\"h2\":{},\"orig_len\":{},\"model\":{}}}",
                        g.total, g.len, g.g, g.t, BLOCK, g.s, g.nsq, g.nsq2,
                        off.h0, off.slots2, off.h1, off.slots, off.h2,
                        o.ex.orig_len, o.ex.model
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
            let (inner, model) = transmute_bytes(&src, model_req);
            let (gd, td) = rib_policy(inner.len());
            let grp: usize = get("--group").map(|s| s.parse().unwrap()).unwrap_or(gd);
            let tpar: usize = get("--parity").map(|s| s.parse().unwrap()).unwrap_or(td);
            let ex = Extras { orig_len: src.len() as u64, orig_fnv: fnv64(&src), model };
            let cont = armor(&inner, grp, tpar, ex);
            let g = geom(inner.len(), grp, tpar, 0, ex);
            let off = offsets(&g);
            let at: usize = match get("--at").as_deref() {
                None | Some("payload") => off.slots + (g.nsq * BLOCK).saturating_sub(len) / 2,
                Some("checks") => off.slots2 + (g.nsq2 * BLOCK).saturating_sub(len) / 2,
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
                if at >= off.slots { "slots" } else if at >= off.slots2 && grp > 0 { "check table" } else { "head" },
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
    let data = restore_bytes(&o.inner, o.ex.orig_len as usize, o.ex.model)?;
    if data.len() as u64 != o.ex.orig_len {
        return Err(format!("restored length {} != original {} ({})", data.len(), o.ex.orig_len, armor_rep));
    }
    if fnv64(&data) != o.ex.orig_fnv {
        return Err(format!("conservation check FAILED after clean de-armor: stage bug, not damage ({})", armor_rep));
    }
    Ok((data, armor_rep))
}
