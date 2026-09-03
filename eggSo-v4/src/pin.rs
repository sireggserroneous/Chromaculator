//! pin.rs -- the discipline the language change must not cost.
//!
//! Every fold-native round asserts its restatement against the site's OWN
//! function rather than against a second restatement. eggSo-v0 checked
//! `regionOf` against `stalk.js`'s `regions()` cell for cell over 22,139
//! cells; eggSo-v1 read the partner formula out of `index.html` at runtime
//! and checked against that. Losing it at the switch to Rust would be a real
//! regression in rigour, so the audit shells out to node.
//!
//! Three pins:
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
//!
//! Node is already required for the site's own suite and this is a dev-time
//! audit rather than a build step. If node is absent every pin reports
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
pub fn v0_structure(code: &Code) -> PinResult {
    let script = "const E=require('./eggSo-v0/eggso.js');const c=E.makeCode(32);\
        const parts=[String(c.p),String(c.q),Array.from(c.region).join(','),\
        c.members.map(m=>m.join(',')).join('|')];\
        const tab=c.tables.map(t=>[...t.entries()].sort((a,b)=>a[0]-b[0])\
        .map(([s,v])=>s+':'+v.map(x=>x.i+'/'+x.d).join('&')).join(','));\
        parts.push(tab.join('|'));process.stdout.write(parts.join(';'));";
    let text = match run_node(script, None) {
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
