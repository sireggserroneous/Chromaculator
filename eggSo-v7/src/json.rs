//! json.rs -- a hand-rolled JSON writer, because `[dependencies]` is empty.
//!
//! The eggSo lineage records every round's numbers in `measured-<slug>.json`
//! written with JavaScript's `JSON.stringify(obj, null, 1)`. v4 is the first
//! Rust round and the record format should not break at the language change,
//! so this reproduces that shape exactly: one space of indent per level,
//! every array and object element on its own line.

use std::fmt::Write as _;

#[derive(Clone, Debug)]
pub enum J {
    U(usize),
    I(i64),
    N(f64),
    S(String),
    B(bool),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

impl J {
    pub fn s(v: &str) -> J {
        J::S(v.to_string())
    }

    /// `JSON.stringify(obj, null, 1)`, to the byte.
    pub fn text(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, 0);
        out
    }

    fn write(&self, out: &mut String, depth: usize) {
        match self {
            J::U(v) => {
                let _ = write!(out, "{v}");
            }
            J::I(v) => {
                let _ = write!(out, "{v}");
            }
            J::N(v) => {
                if v.fract() == 0.0 && v.abs() < 1e15 {
                    let _ = write!(out, "{}", *v as i64);
                } else {
                    let _ = write!(out, "{v}");
                }
            }
            J::B(v) => {
                let _ = write!(out, "{v}");
            }
            J::S(v) => write_str(out, v),
            J::A(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push_str("[\n");
                for (k, it) in items.iter().enumerate() {
                    pad(out, depth + 1);
                    it.write(out, depth + 1);
                    if k + 1 < items.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                pad(out, depth);
                out.push(']');
            }
            J::O(fields) => {
                if fields.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push_str("{\n");
                for (k, (name, val)) in fields.iter().enumerate() {
                    pad(out, depth + 1);
                    write_str(out, name);
                    out.push_str(": ");
                    val.write(out, depth + 1);
                    if k + 1 < fields.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                pad(out, depth);
                out.push('}');
            }
        }
    }
}

fn pad(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push(' ');
    }
}

fn write_str(out: &mut String, v: &str) {
    out.push('"');
    for ch in v.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Build a `J::O` without ceremony: `obj(&[("a", J::U(1))])`.
pub fn obj(fields: &[(&str, J)]) -> J {
    J::O(fields.iter().map(|(k, v)| (k.to_string(), v.clone())).collect())
}

/// Write a round record beside the crate root, the lineage's own convention.
pub fn record(slug: &str, value: &J) -> std::io::Result<()> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::write(dir.join(format!("measured-{slug}.json")), value.text())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_matches_stringify_indent_one() {
        let v = obj(&[
            ("a", J::U(1)),
            ("b", J::A(vec![J::U(1), J::U(2)])),
            ("c", obj(&[("d", J::B(true))])),
        ]);
        let want = "{\n \"a\": 1,\n \"b\": [\n  1,\n  2\n ],\n \"c\": {\n  \"d\": true\n }\n}";
        assert_eq!(v.text(), want);
    }

    #[test]
    fn empty_containers_are_inline() {
        assert_eq!(J::A(vec![]).text(), "[]");
        assert_eq!(J::O(vec![]).text(), "{}");
    }

    #[test]
    fn strings_escape() {
        let v = J::S(String::from("a\"b") + "\\" + "c\nd");
        assert_eq!(v.text(), "\"a\\\"b\\\\c\\nd\"");
    }

    #[test]
    fn whole_floats_print_as_integers_like_js() {
        assert_eq!(J::N(3.0).text(), "3");
        assert_eq!(J::N(0.5).text(), "0.5");
    }
}
