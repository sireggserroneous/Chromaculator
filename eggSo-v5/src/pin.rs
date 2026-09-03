//! pin.rs -- the discipline the language change must not cost.
//!
//! Every fold-native round asserts its restatement against the site's OWN
//! function rather than against a second restatement. eggSo-v0 checked
//! `regionOf` against `stalk.js`'s `regions()` cell for cell over 22,139
//! cells; eggSo-v1 read the partner formula out of `index.html` at runtime
//! and checked against that. Losing it at the switch to Rust would be a real
//! regression in rigour, so the audit shells out to node.
//!
//! Five pins:
//!
//!   * `site_regions` / `site_arcs` -- `region_of` and `arcs` against
//!     `stalk.js`'s own `regions()` and `arcs()`.
//!   * `site_cell_order` -- **v5's**, and N1 rests on it. The degree-3
//!     coordinate takes its ANGLE from the site's fill order, so
//!     `dynamics::hankel_k` is checked against `stalk.js`'s own `cellOrder`
//!     cell for cell. If the angle were mine rather than the site's, Part 1
//!     would be a construction about nothing.
//!   * `v0_structure` -- the port's class array, member lists and syndrome
//!     tables against eggSo-v0's, element for element, in the same order.
//!     Outcome equality can hide two compensating bugs; this cannot.
//!   * `v0_decisions` -- squares and damage generated HERE, decoded by BOTH
//!     v0's decoder and the port, compared square by square. Stronger than
//!     matching v0's published aggregates, and it does not depend on
//!     replaying v0's test file's exact PRNG stream.
//!   * `v4_figures` -- **v5's**, and P1 is exactly it. Each round in this
//!     repo is a frozen record and its own crate, so v5 COPIES v4's modules
//!     rather than depending on them -- and a copy can drift silently. This
//!     recomputes v4's headline figures with v5's copies and compares them
//!     against v4's COMMITTED `measured-*.json`. It needs no node.
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

/// N1's other half. `dynamics::hankel_k` against `stalk.js`'s own
/// `cellOrder`, which is where the degree-3 coordinate's ANGLE comes from.
///
/// The site's walk is `for(let r = Math.min(n-1,d); r >= 0; r--)`, so the
/// `k`-th cell of band `d` is at `r = min(n-1,d) - k`. That inversion is the
/// whole of `hankel_k`, and this checks it against the walk itself rather
/// than against a restatement of the walk.
pub fn site_cell_order(max_n: usize) -> PinResult {
    let script = format!(
        "const fs=require('fs');eval(fs.readFileSync('stalk.js','utf8'));\
         const out=[];for(let n=2;n<={max_n};n++){{const k=new Array(n*n).fill(-1);\
         const seen={{}};cellOrder(n).forEach(([r,c])=>{{const d=r+c;\
         seen[d]=(seen[d]===undefined?0:seen[d]+1);k[r*n+c]=seen[d];}});\
         out.push(k.join(','));}}\
         process.stdout.write(out.join(';'));"
    );
    let text = match run_node(&script, None) {
        Ok(t) => t,
        Err(e) => return PinResult::skipped("hankel_k vs cellOrder", e),
    };
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    for (row, n) in text.split(';').zip(2..=max_n) {
        let theirs = ints(row);
        if theirs.len() != n * n {
            mismatches += 1;
            continue;
        }
        for (j, &t) in theirs.iter().enumerate() {
            let mine = crate::dynamics::hankel_k(j / n, j % n, n) as i64;
            if mine != t {
                mismatches += 1;
            }
            checked += 1;
        }
    }
    PinResult { name: "hankel_k vs cellOrder", checked, mismatches, skipped: None }
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
    let p = repo_root().join("eggSo-v4").join(format!("measured-{slug}.json"));
    std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))
}

/// P1. v5 copies v4's `fold`, `code`, `dynamics`, `seam`, `json` and `pin`
/// rather than depending on v4, because each round here is a frozen record
/// and a path dependency would let v5's numbers drift when v4 changes. The
/// price of copying is that a copy can drift SILENTLY, so this pin recomputes
/// v4's headline figures with v5's copies and compares them against v4's
/// committed `measured-*.json`, figure by figure:
///
///   * the coordinate: 89,439 cells over `n = 2..64`, 0 exceptions;
///   * the four pins v4 recorded, by their checked counts and 0 mismatches;
///   * `diag3` at `341/342/341` and separation `0.6673`;
///   * `diag3`'s flagged burst sweep, `200/200/200/200/200`.
///
/// No node. If a record file is missing the pin reports SKIPPED loudly.
pub fn v4_figures() -> PinResult {
    const NAME: &str = "the copy vs v4's record";
    let mut checked = 0usize;
    let mut mismatches = 0usize;
    let mut check = |cond: bool| {
        checked += 1;
        if !cond {
            mismatches += 1;
        }
    };

    // -- the coordinate, recomputed with v5's fold.rs -----------------------
    let basins = match read_record("basins") {
        Ok(t) => t,
        Err(e) => return PinResult::skipped(NAME, e),
    };
    let mut cells = 0usize;
    let mut bad = 0usize;
    for n in 2..=64usize {
        for r in 0..n {
            for c in 0..n {
                cells += 1;
                let rho = crate::fold::rho_of(r, c, n);
                let by = if rho < 1.0 {
                    crate::fold::INNER
                } else if rho == 1.0 {
                    crate::fold::FOLD
                } else {
                    crate::fold::OUTER
                };
                let (pr, pc) = crate::fold::sigma_rc(r, c, n);
                if by != crate::fold::region_of(r, c, n)
                    || crate::fold::rho_of(pr, pc, n) != 1.0 / rho
                {
                    bad += 1;
                }
            }
        }
    }
    let mut s = Scan::new(&basins);
    check(s.after("\"cells\"") == Some(cells as f64));
    check(s.after("\"exceptions\"") == Some(bad as f64));
    check(cells == 89_439 && bad == 0);

    // -- the four pin counts v4 recorded ------------------------------------
    let pins = match read_record("pins") {
        Ok(t) => t,
        Err(e) => return PinResult::skipped(NAME, e),
    };
    let mut s = Scan::new(&pins);
    let mut counts = Vec::new();
    for _ in 0..4 {
        let c = s.after("\"checked\"");
        let m = s.after("\"mismatches\"");
        check(m == Some(0.0));
        counts.push(c);
    }
    for (got, want) in counts.iter().zip([22_139.0, 1_599.0, 6_153.0, 600.0]) {
        check(*got == Some(want));
    }

    // -- diag3, recomputed with v5's seam.rs and code.rs -------------------
    let seamrec = match read_record("seam") {
        Ok(t) => t,
        Err(e) => return PinResult::skipped(NAME, e),
    };
    let d3 = crate::seam::seams().into_iter().find(|x| x.name == "diag3").unwrap();
    let code = d3.code(32, true);
    let sizes = code.sizes();
    let sep = crate::fold::separation(&sizes);
    let mut s = Scan::new(&seamrec);
    check(s.seek("\"arm\": \"diag3\""));
    check(s.seek("\"classes\""));
    for m in &sizes {
        check(s.num() == Some(*m as f64));
    }
    let theirs_sep = s.after("\"separation\"").unwrap_or(-1.0);
    check((theirs_sep - sep).abs() < 5e-7);
    check(sizes == [341, 342, 341]);
    check((sep - 0.6673).abs() < 5e-5);
    check(code.check_bits() == 48);

    // -- diag3's flagged burst sweep, re-run with v5's channels ------------
    let mut s = Scan::new(&seamrec);
    check(s.seek("\"burstSweep\""));
    check(s.seek("\"arm\": \"diag3\""));
    let lengths = [12usize, 15, 18, 24, 31];
    for (b, t) in crate::seam::burst_breaking_point(&code, &lengths, 200, 1700) {
        let theirs_b = s.after("\"burst\"");
        let theirs_c = s.after("\"corrected\"");
        check(theirs_b == Some(b as f64));
        match t {
            Some(t) => {
                check(theirs_c == Some(t.corrected as f64));
                check(t.corrected == 200 && t.wrong == 0);
            }
            None => check(false),
        }
    }

    PinResult { name: NAME, checked, mismatches, skipped: None }
}
