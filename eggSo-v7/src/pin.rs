//! pin.rs -- the discipline the language change must not cost.
//!
//! Every fold-native round asserts its restatement against the site's OWN
//! function rather than against a second restatement. eggSo-v0 checked
//! `regionOf` against `stalk.js`'s `regions()` cell for cell over 22,139
//! cells; eggSo-v1 read the partner formula out of `index.html` at runtime
//! and checked against that. Losing it at the switch to Rust would be a real
//! regression in rigour, so the audit shells out to node.
//!
//! Five pins. v6 drops v5's `cellOrder` pin, because v6 does not carry
//! `dynamics.rs` -- the degree-3 coordinate was v5's Part 1 and this round is
//! about the decoder's caps -- and repoints the record pin at v5.
//!
//! Four pins:
//!
//!   * `site_regions` / `site_arcs` -- `region_of` and `arcs` against
//!     `stalk.js`'s own `regions()` and `arcs()`.
//!   * `v0_structure` -- the port's class array, member lists and syndrome
//!     tables against eggSo-v0's, element for element, in the same order.
//!     Outcome equality can hide two compensating bugs; this cannot.
//!   * `v0_decisions` -- squares and damage generated HERE, decoded by BOTH
//!     v0's decoder and the port, compared square by square. Stronger than
//!     matching v0's published aggregates, and it does not depend on
//!     replaying v0's test file's exact PRNG stream.
//!   * `v6_figures` -- **C1**, and it is the gate this round rests on. v6
//!     copies v5's modules rather than depending on them, so a copy can
//!     drift silently. This recomputes v5's headline figures with v6's copies
//!     and compares them against v5's COMMITTED `measured-*.json` -- the
//!     figure that matters most being the one v6 exists to move, v5's
//!     same-class collapse to 70 of 120 at `n = 512`. It needs no node.
//!
//! Node is already required for the site's own suite and this is a dev-time
//! audit rather than a build step. If node is absent every node pin reports
//! SKIPPED, loudly, and never passes quietly.

use std::io::Write;
use std::process::{Command, Stdio};

use crate::code::{Code, Opts, Status};

/// Where the repo root sits relative to this crate.
pub fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

pub fn node_available() -> bool {
    Command::new("node")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn run_node(script: &str, stdin_text: Option<&str>) -> Result<String, String> {
    let mut cmd = Command::new("node");
    cmd.arg("-e")
        .arg(script)
        .current_dir(repo_root())
        .stdin(if stdin_text.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn node: {e}"))?;
    if let Some(text) = stdin_text {
        child
            .stdin
            .as_mut()
            .ok_or("no stdin")?
            .write_all(text.as_bytes())
            .map_err(|e| format!("write stdin: {e}"))?;
    }
    let out = child.wait_with_output().map_err(|e| format!("wait node: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Pull every integer out of a flat JSON-ish payload, in order. The pins all
/// emit deliberately flat integer streams so no JSON parser is needed on this
/// side -- one fewer thing to get wrong, and `[dependencies]` stays empty.
fn ints(text: &str) -> Vec<i64> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        let neg = b[i] == b'-' && i + 1 < b.len() && b[i + 1].is_ascii_digit();
        if neg || b[i].is_ascii_digit() {
            let start = i;
            if neg {
                i += 1;
            }
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            if let Ok(v) = text[start..i].parse::<i64>() {
                out.push(v);
            }
        } else {
            i += 1;
        }
    }
    out
}

pub struct PinResult {
    pub name: &'static str,
    pub checked: usize,
    pub mismatches: usize,
    pub skipped: Option<String>,
}

impl PinResult {
    pub fn skipped(name: &'static str, why: String) -> PinResult {
        PinResult { name, checked: 0, mismatches: 0, skipped: Some(why) }
    }
    pub fn ok(&self) -> bool {
        self.skipped.is_none() && self.mismatches == 0
    }
    pub fn line(&self) -> String {
        match &self.skipped {
            Some(why) => format!("  {:<28} SKIPPED -- {}", self.name, why),
            None if self.mismatches == 0 => {
                format!("  {:<28} {} checked, 0 mismatches", self.name, self.checked)
            }
            None => format!(
                "  {:<28} {} checked, {} MISMATCHES",
                self.name, self.checked, self.mismatches
            ),
        }
    }
}

/// P5, first half. `region_of` against `stalk.js`'s own `regions()`, over
/// every width the site draws. The site's tests load `stalk.js` by `eval`
/// because it has no exports (`tools/bulk.test.js:4-5`); so does this.
pub fn site_regions(max_n: usize) -> PinResult {
    let script = format!(
        "const fs=require('fs');eval(fs.readFileSync('stalk.js','utf8'));\
         const out=[];for(let n=2;n<={max_n};n++){{const reg=regions(new Array(n*n).fill(1),n);\
         const cls=new Array(n*n).fill(-1);\
         const code={{inner:0,fold:1,outer:2}};\
         for(const k of ['inner','fold','outer'])for(const s of reg[k])cls[s.r*n+s.c]=code[k];\
         out.push(cls.join(','));}}\
         process.stdout.write(out.join(';'));"
    );
    let text = match run_node(&script, None) {
        Ok(t) => t,
        Err(e) => return PinResult::skipped("region_of vs stalk.js", e),
    };
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    for (row, n) in text.split(';').zip(2..=max_n) {
        let theirs = ints(row);
        if theirs.len() != n * n {
            mismatches += 1;
            continue;
        }
        for (j, &t) in theirs.iter().enumerate().take(n * n) {
            let mine = crate::fold::region_of(j / n, j % n, n) as i64;
            if mine != t {
                mismatches += 1;
            }
            checked += 1;
        }
    }
    PinResult { name: "region_of vs stalk.js", checked, mismatches, skipped: None }
}

/// P5, second half. `arcs` against `stalk.js`'s own, which is where the
/// site's "single-cell poles, widest ring at the Fold, equal hemispheres"
/// claim (`index.html:313-314`) actually comes from.
pub fn site_arcs(max_n: usize) -> PinResult {
    let script = format!(
        "const fs=require('fs');eval(fs.readFileSync('stalk.js','utf8'));\
         const out=[];for(let n=2;n<={max_n};n++)out.push(arcs(n).join(','));\
         process.stdout.write(out.join(';'));"
    );
    let text = match run_node(&script, None) {
        Ok(t) => t,
        Err(e) => return PinResult::skipped("arcs vs stalk.js", e),
    };
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    for (row, n) in text.split(';').zip(2..=max_n) {
        let theirs = ints(row);
        let mine = crate::fold::arcs(n);
        if theirs.len() != mine.len() {
            mismatches += 1;
            continue;
        }
        for k in 0..mine.len() {
            if mine[k] as i64 != theirs[k] {
                mismatches += 1;
            }
            checked += 1;
        }
    }
    PinResult { name: "arcs vs stalk.js", checked, mismatches, skipped: None }
}

/// S1, the structural pin. The port's `p`, `q`, class array, member lists and
/// syndrome tables against eggSo-v0's own, in order.
///
/// v4 baked `makeCode(32)` into this script and ignored `code.n`, so asking
/// it about any other width would have compared two different grids and
/// reported a mismatch that meant nothing. It takes `code.n` now.
pub fn v0_structure(code: &Code) -> PinResult {
    let script = format!(
        "const E=require('./eggSo-v0/eggso.js');const c=E.makeCode({n});\
         const parts=[String(c.p),String(c.q),Array.from(c.region).join(','),\
         c.members.map(m=>m.join(',')).join('|')];\
         const tab=c.tables.map(t=>[...t.entries()].sort((a,b)=>a[0]-b[0])\
         .map(([s,v])=>s+':'+v.map(x=>x.i+'/'+x.d).join('&')).join(','));\
         parts.push(tab.join('|'));process.stdout.write(parts.join(';'));",
        n = code.n
    );
    let text = match run_node(&script, None) {
        Ok(t) => t,
        Err(e) => return PinResult::skipped("port vs v0 structure", e),
    };
    let parts: Vec<&str> = text.split(';').collect();
    if parts.len() != 5 {
        return PinResult {
            name: "port vs v0 structure",
            checked: 0,
            mismatches: 1,
            skipped: None,
        };
    }
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    let mut check = |cond: bool| {
        checked += 1;
        if !cond {
            mismatches += 1;
        }
    };

    check(parts[0].trim().parse::<u64>() == Ok(code.p));
    check(parts[1].trim().parse::<u64>() == Ok(code.q));

    let region = ints(parts[2]);
    check(region.len() == code.l);
    for (j, &r) in region.iter().enumerate().take(code.l) {
        check(r == code.class[j] as i64);
    }

    for (k, chunk) in parts[3].split('|').enumerate() {
        if k >= 3 {
            break;
        }
        let theirs = ints(chunk);
        check(theirs.len() == code.members[k].len());
        for (a, b) in theirs.iter().zip(code.members[k].iter()) {
            check(*a == *b as i64);
        }
    }

    for (k, chunk) in parts[4].split('|').enumerate() {
        if k >= 3 {
            break;
        }
        let mut keys: Vec<(u64, Vec<(usize, i8)>)> = Vec::new();
        for entry in chunk.split(',') {
            let Some((s, rest)) = entry.split_once(':') else { continue };
            let Ok(s) = s.trim().parse::<u64>() else { continue };
            let mut cands = Vec::new();
            for c in rest.split('&') {
                if let Some((i, d)) = c.split_once('/') {
                    if let (Ok(i), Ok(d)) = (i.trim().parse::<usize>(), d.trim().parse::<i8>()) {
                        cands.push((i, d));
                    }
                }
            }
            keys.push((s, cands));
        }
        check(keys.len() == code.tables[k].len());
        for (s, cands) in keys {
            match code.tables[k].get(&s) {
                None => check(false),
                Some(mine) => {
                    check(mine.len() == cands.len());
                    for (a, b) in mine.iter().zip(cands.iter()) {
                        check(a == b);
                    }
                }
            }
        }
    }
    PinResult { name: "port vs v0 structure", checked, mismatches, skipped: None }
}

/// One decoding problem: a damaged square, its checks, and any flagged cells.
pub struct Case {
    pub cells: Vec<i8>,
    pub check: Vec<u64>,
    pub erased: Vec<usize>,
    pub per_candidate: bool,
}

fn encode_cells(cells: &[i8]) -> String {
    cells
        .iter()
        .map(|&v| match v {
            0 => '0',
            1 => '1',
            _ => 'x',
        })
        .collect()
}

/// S1, the behavioural pin. Both decoders on the same problems, compared
/// square by square: the status word AND the repaired cells.
pub fn v0_decisions(code: &Code, cases: &[Case]) -> PinResult {
    let mut payload = String::new();
    for c in cases {
        payload.push_str(&encode_cells(&c.cells));
        payload.push(' ');
        payload.push_str(
            &c.check.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","),
        );
        payload.push(' ');
        payload.push_str(
            &c.erased.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","),
        );
        payload.push(' ');
        payload.push(if c.per_candidate { '1' } else { '0' });
        payload.push('\n');
    }
    let script = "const E=require('./eggSo-v0/eggso.js');const fs=require('fs');\
        const code=E.makeCode(32);\
        const lines=fs.readFileSync(0,'utf8').split('\\n').filter(s=>s.length>2);\
        const out=lines.map(line=>{\
          const [cs,ck,er,pc]=line.split(' ');\
          const cells=Int8Array.from([...cs].map(ch=>ch==='x'?-1:(ch==='1'?1:0)));\
          const check=ck.split(',').map(Number);\
          const opts={};if(er.length)opts.erased=er.split(',').map(Number);\
          if(pc==='1')opts.perCandidate=true;\
          const r=E.repairSquare(cells,check,code,Object.keys(opts).length?opts:undefined);\
          return r.status[0]+[...cells].map(v=>v<0?'x':String(v)).join('');\
        });process.stdout.write(out.join('\\n'));";
    let text = match run_node(script, Some(&payload)) {
        Ok(t) => t,
        Err(e) => return PinResult::skipped("port vs v0 decisions", e),
    };
    let rows: Vec<&str> = text.split('\n').filter(|s| s.len() > 2).collect();
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    if rows.len() != cases.len() {
        return PinResult {
            name: "port vs v0 decisions",
            checked: rows.len(),
            mismatches: cases.len().abs_diff(rows.len()).max(1),
            skipped: None,
        };
    }
    for (row, case) in rows.iter().zip(cases.iter()) {
        let theirs_status = row.as_bytes()[0];
        let theirs_cells = &row[1..];
        let mut mine = case.cells.clone();
        let opts = Opts {
            erased: case.erased.clone(),
            doubles: true,
            confirm_mode: Some(if case.per_candidate {
                crate::code::Confirm::PerCandidate
            } else {
                crate::code::Confirm::AfterPlan
            }),
            // v0's own caps, explicitly: this pin's whole job is to prove the
            // DEFAULT decoder is still v0 to the decision now that the caps
            // are a parameter.
            caps: crate::code::Caps::v0(),
        };
        let r = crate::code::repair(&mut mine, &case.check, code, &opts);
        let mine_status = match r.status {
            Status::Clean => b'c',
            Status::Corrected => b'c',
            Status::Detected => b'd',
            Status::Ambiguous => b'a',
        };
        // v0's words: clean | corrected | detected | ambiguous. `clean` and
        // `corrected` share a first letter, so compare the cells to separate
        // them, which is the thing that actually matters anyway.
        checked += 1;
        if mine_status != theirs_status || encode_cells(&mine) != theirs_cells {
            mismatches += 1;
        }
    }
    PinResult { name: "port vs v0 decisions", checked, mismatches, skipped: None }
}

// ---- P1: the copy against v4's committed record -------------------------

/// A cursor over one of the lineage's `measured-*.json` files.
///
/// `json.rs` writes `JSON.stringify(obj, null, 1)` exactly, so the records
/// are a predictable shape and a key-then-number scan reads them without a
/// parser -- which keeps `[dependencies]` empty and is one fewer thing to get
/// wrong than a parser would be. The files it reads are committed and frozen,
/// so the shape cannot move under it.
struct Scan<'a> {
    t: &'a str,
    at: usize,
}

impl<'a> Scan<'a> {
    fn new(t: &'a str) -> Scan<'a> {
        Scan { t, at: 0 }
    }
    /// Move the cursor just past the next occurrence of `needle`.
    fn seek(&mut self, needle: &str) -> bool {
        match self.t[self.at..].find(needle) {
            Some(k) => {
                self.at += k + needle.len();
                true
            }
            None => false,
        }
    }
    /// Read the next JSON number, advancing past it. A number is
    /// `-? digits (. digits)? ([eE] [+-]? digits)?`, and nothing else is
    /// treated as the start of one -- so a digit inside a key or a string
    /// value (`"seam128"`, `"port vs v0 structure"`) is never mistaken for a
    /// figure. Every caller seeks its key first for the same reason.
    fn num(&mut self) -> Option<f64> {
        let b = self.t.as_bytes();
        let digit_at = |i: usize| i < b.len() && b[i].is_ascii_digit();
        let mut i = self.at;
        while i < b.len() && !(digit_at(i) || (b[i] == b'-' && digit_at(i + 1))) {
            i += 1;
        }
        if i >= b.len() {
            return None;
        }
        let start = i;
        if b[i] == b'-' {
            i += 1;
        }
        while digit_at(i) {
            i += 1;
        }
        if i < b.len() && b[i] == b'.' && digit_at(i + 1) {
            i += 1;
            while digit_at(i) {
                i += 1;
            }
        }
        if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
            let mut k = i + 1;
            if k < b.len() && (b[k] == b'+' || b[k] == b'-') {
                k += 1;
            }
            if digit_at(k) {
                i = k;
                while digit_at(i) {
                    i += 1;
                }
            }
        }
        let tok = &self.t[start..i];
        self.at = i;
        tok.parse::<f64>().ok()
    }
    /// `seek` then `num`, which is how every figure below is addressed.
    fn after(&mut self, needle: &str) -> Option<f64> {
        if !self.seek(needle) {
            return None;
        }
        self.num()
    }
}

fn read_record(slug: &str) -> Result<String, String> {
    let p = repo_root().join("eggSo-v6").join(format!("measured-{slug}.json"));
    std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))
}

/// T1. v7 copies v6's modules rather than depending on them, so a copy can
/// drift silently. This recomputes v6's headline figures with v7's copies and
/// compares them against v6's committed `measured-*.json`:
///
///   * the 600-decision port pin, 0 mismatches;
///   * the two derived bounds, 44.0 spread and 22.0 concentrated at n = 32;
///   * **v6's 2 miscorrections at the lopsided raise** -- the figure this
///     round exists to drive to zero, fixed here before the guard touches it.
///
/// No node. If a record file is missing the pin reports SKIPPED loudly.
pub fn v6_figures() -> PinResult {
    const NAME: &str = "the copy vs v6's record";
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    let mut check = |cond: bool| {
        checked += 1;
        if !cond {
            mismatches += 1;
        }
    };

    // -- v6's own pins ------------------------------------------------------
    let pins = match read_record("pins") {
        Ok(t) => t,
        Err(e) => return PinResult::skipped(NAME, e),
    };
    let mut s = Scan::new(&pins);
    let mut seen_decisions = false;
    for _ in 0..8 {
        if !s.seek("\"checked\"") {
            break;
        }
        let c = s.num();
        let m = s.after("\"mismatches\"");
        check(m == Some(0.0));
        if c == Some(600.0) {
            seen_decisions = true;
        }
    }
    check(seen_decisions);

    // -- the bounds, recomputed with v7's copies ---------------------------
    let bound = match read_record("bound") {
        Ok(t) => t,
        Err(e) => return PinResult::skipped(NAME, e),
    };
    let d3 = crate::seam::seams().into_iter().find(|x| x.name == "diag3").unwrap();
    let code = d3.code(32, true);
    let mut s = Scan::new(&bound);
    let theirs_spread = s.after("\"spreadBoundAt32\"").unwrap_or(-1.0);
    let theirs_conc = s.after("\"concentratedBoundAt32\"").unwrap_or(-1.0);
    check((theirs_spread - crate::code::Caps::spread_bound(&code)).abs() < 1e-6);
    check((theirs_conc - crate::code::Caps::concentrated_bound(&code)).abs() < 1e-6);
    check((theirs_spread - 44.0).abs() < 0.1);
    check((theirs_conc - 22.0).abs() < 0.1);
    check(code.check_bits() == 48);
    check(code.p == 2053 && code.q == 2063);

    // -- THE figure this round exists to move ------------------------------
    let caps = match read_record("caps") {
        Ok(t) => t,
        Err(e) => return PinResult::skipped(NAME, e),
    };
    let mut s = Scan::new(&caps);
    let mut lopsided_wrong: Option<f64> = None;
    for _ in 0..12 {
        if !s.seek("\"caps\"") {
            break;
        }
        // the lopsided row is the one v6 labelled UNSAFE
        let head = &s.t[s.at..(s.at + 60).min(s.t.len())];
        let is_lopsided = head.contains("UNSAFE");
        let w = s.after("\"wrong\"");
        if is_lopsided {
            lopsided_wrong = w;
            break;
        }
    }
    check(lopsided_wrong == Some(2.0));

    PinResult { name: NAME, checked, mismatches, skipped: None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{obj, J};

    /// `Scan::num` reads a JSON number and nothing else. The hazard this
    /// guards is specific and real: the records contain digits INSIDE strings
    /// -- `"seam128"`, `"port vs v0 structure"`, `"arm": "diag3"` -- and a
    /// reader that treated those as figures would compare the wrong numbers
    /// and still report 0 mismatches.
    #[test]
    fn the_reader_reads_numbers_and_not_digits_in_strings() {
        let mut s = Scan::new("{\"a\": 12, \"b\": -3, \"c\": 0.5, \"d\": 1e3, \"e\": 2.5e-2}");
        assert_eq!(s.after("\"a\""), Some(12.0));
        assert_eq!(s.after("\"b\""), Some(-3.0));
        assert_eq!(s.after("\"c\""), Some(0.5));
        assert_eq!(s.after("\"d\""), Some(1000.0));
        assert_eq!(s.after("\"e\""), Some(0.025));

        // a digit inside a string must not be read as the next figure
        let mut s = Scan::new("{\"arm\": \"seam128\", \"corrected\": 7}");
        assert_eq!(s.after("\"corrected\""), Some(7.0));
        // and scanning from the top, the first NUMBER is 128 only because it
        // really is a digit run; the guard is that callers seek their key
        // first, which the line above does and every caller in v4_figures does
        let mut s = Scan::new("{\"pin\": \"port vs v0 structure\", \"checked\": 6153}");
        assert_eq!(s.after("\"checked\""), Some(6153.0));

        // a missing key is None, never a stale or defaulted number
        let mut s = Scan::new("{\"a\": 1}");
        assert_eq!(s.after("\"nope\""), None);
        assert!(!s.seek("\"nope\""));
    }

    /// The reader against the lineage's OWN writer, so the two cannot drift:
    /// build a record with `json::J`, print it the way every round prints
    /// one, and read the figures back out.
    #[test]
    fn the_reader_round_trips_the_writers_own_shape() {
        let rec = J::A(vec![
            obj(&[("pin", J::s("region_of vs stalk.js")), ("checked", J::U(22139)), ("mismatches", J::U(0))]),
            obj(&[("pin", J::s("arcs vs stalk.js")), ("checked", J::U(1599)), ("mismatches", J::U(0))]),
        ]);
        let text = rec.text();
        let mut s = Scan::new(&text);
        for want in [22139.0, 1599.0] {
            assert_eq!(s.after("\"checked\""), Some(want));
            assert_eq!(s.after("\"mismatches\""), Some(0.0));
        }
    }

    /// THE NEGATIVE CONTROL, and the reason this module needed tests at all.
    /// A pin that cannot fail is not a pin. This proves the reader is really
    /// addressing the field `v6_figures` believes it is addressing: perturb
    /// v6's committed record in memory and the figure that comes back
    /// changes.
    ///
    /// The figure chosen is the one this round exists to drive to zero --
    /// v6's 2 miscorrections at the lopsided raise.
    #[test]
    fn a_perturbed_record_reads_back_perturbed() {
        let Ok(real) = read_record("caps") else {
            return; // v6's record is not on disk; v6_figures reports SKIPPED
        };
        let find_lopsided_wrong = |text: &str| -> Option<f64> {
            let mut s = Scan::new(text);
            for _ in 0..12 {
                if !s.seek("\"caps\"") {
                    return None;
                }
                let head = &s.t[s.at..(s.at + 60).min(s.t.len())];
                let is_lopsided = head.contains("UNSAFE");
                let w = s.after("\"wrong\"");
                if is_lopsided {
                    return w;
                }
            }
            None
        };
        assert_eq!(
            find_lopsided_wrong(&real),
            Some(2.0),
            "v6 recorded 2 miscorrections at the lopsided raise"
        );

        // the same lookup over a record whose figure has been changed must
        // return the changed value, so the comparison in v6_figures would
        // have to fail
        let faked = real.replacen("\"wrong\": 2", "\"wrong\": 9", 1);
        assert_ne!(faked, real, "the perturbation did not apply");
        assert_eq!(
            find_lopsided_wrong(&faked),
            Some(9.0),
            "the reader ignored a change it should have seen"
        );
    }

    /// P1 itself, as a test and not only as a line of `eggso5 pin` output --
    /// and with the vacuous pass ruled out. `PinResult::ok()` is true when
    /// nothing mismatched, so a pin that checked NOTHING would read as clean;
    /// this asserts it checked something.
    #[test]
    fn the_v6_figures_pin_checks_something_and_agrees() {
        let r = v6_figures();
        if r.skipped.is_some() {
            return; // v5's records are not on disk
        }
        assert!(r.checked > 0, "the pin passed without checking anything");
        assert_eq!(r.mismatches, 0, "{}", r.line());
        assert!(r.ok());
    }

    /// The reporting contract the round's verdict is read through: a skipped
    /// pin is NOT a clean pin, and a mismatching pin is not either.
    #[test]
    fn a_skipped_pin_never_counts_as_clean() {
        let s = PinResult::skipped("x", "node is missing".to_string());
        assert!(!s.ok());
        assert!(s.line().contains("SKIPPED"));

        let bad = PinResult { name: "x", checked: 10, mismatches: 1, skipped: None };
        assert!(!bad.ok());
        assert!(bad.line().contains("MISMATCH"));

        let good = PinResult { name: "x", checked: 10, mismatches: 0, skipped: None };
        assert!(good.ok());
    }
}
