#!/usr/bin/env python
# regenerates src/squash_tab.rs byte-identically; float math lives HERE only
#   SQUASH    (v8..v11, 12-bit): 4096 entries, round(4096 / (1 + e^(-(i-2047)/256))) in [1, 4095]
#   SQUASH16  (v12-M2a):        8191 entries over x in [-4095, 4095],
#                               round(65536 / (1 + e^(-x/256))) in [1, 65535]
#                               (Mahoney zpaq's 15-bit squash, one bit wider; e^-16 < 2^-16 so the
#                               domain saturates by construction)
#   STRETCH16 (v12-M2a):        65536 entries, round(256 ln(p / (65536 - p))) clamped to [-4095, 4095];
#                               exact at the tails (p = 1 -> -2839, p = 65535 -> +2839), p = 0 guarded
import math
vals = []
for i in range(4096):
    x = i - 2047
    v = round(4096.0 / (1.0 + math.exp(-x / 256.0)))
    vals.append(max(1, min(4095, v)))
assert all(vals[i] <= vals[i+1] for i in range(4095))
sq16 = []
for i in range(8191):
    x = i - 4095
    v = round(65536.0 / (1.0 + math.exp(-x / 256.0)))
    sq16.append(max(1, min(65535, v)))
assert all(sq16[i] <= sq16[i+1] for i in range(8190))
assert sq16[4095] == 32768
st16 = [-4095]
for p in range(1, 65536):
    v = round(256.0 * math.log(p / (65536.0 - p)))
    st16.append(max(-4095, min(4095, v)))
assert st16[32768] == 0 and st16[1] == -2839 and st16[65535] == 2839
assert all(st16[i] <= st16[i+1] for i in range(65535))
lines = ["//! squash_tab.rs -- the logistic squash table, GENERATED ONCE and checked",
         "//! in (tools/gen_squash.py regenerates it byte-identically). The coding path",
         "//! never touches a float: the decoder mirror is sacred across machines, and",
         "//! Rust's libm is not bit-identical across platforms. SQUASH[i] =",
         "//! round(4096 / (1 + e^(-(i-2047)/256))), clamped to [1, 4095]; STRETCH is",
         "//! derived from this table by integer binary search at model init.",
         "", "pub const SQUASH: [u16; 4096] = ["]
for i in range(0, 4096, 16):
    lines.append("    " + ", ".join(str(v) for v in vals[i:i+16]) + ",")
lines.append("];")
lines += ["",
          "/// v12-M2a, the 16-bit pipeline (glossary.js:164 \"kept rather than rounded",
          "/// away\"; attribution: Mahoney's zpaq 15-bit squash / 16-bit coder, Knoll's",
          "/// cmix 16-bit coder): SQUASH16[x + 4095] = round(65536 / (1 + e^(-x/256)))",
          "/// clamped to [1, 65535] for x in [-4095, 4095]. Same logit scale as SQUASH",
          "/// (x/256), twice the domain, sixteen times the resolution.",
          "pub static SQUASH16: [u16; 8191] = ["]
for i in range(0, 8191, 16):
    lines.append("    " + ", ".join(str(v) for v in sq16[i:i+16]) + ",")
lines.append("];")
lines += ["",
          "/// STRETCH16[p] = round(256 ln(p / (65536 - p))) clamped to [-4095, 4095]: the",
          "/// exact inverse (a scan of SQUASH16 would flatten the tails); p = 0 is a guard",
          "/// (-4095) that no clamped probability ever indexes.",
          "pub static STRETCH16: [i16; 65536] = ["]
for i in range(0, 65536, 16):
    lines.append("    " + ", ".join(str(v) for v in st16[i:i+16]) + ",")
lines.append("];")
open('src/squash_tab.rs','w',newline='\n').write("\n".join(lines) + "\n")
print("squash_tab.rs written: SQUASH 4096, SQUASH16 8191, STRETCH16 65536")
