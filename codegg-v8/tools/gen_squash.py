#!/usr/bin/env python
# regenerates src/squash_tab.rs byte-identically; float math lives HERE only
import math
vals = []
for i in range(4096):
    x = i - 2047
    v = round(4096.0 / (1.0 + math.exp(-x / 256.0)))
    vals.append(max(1, min(4095, v)))
assert all(vals[i] <= vals[i+1] for i in range(4095))
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
open('src/squash_tab.rs','w',newline='\n').write("\n".join(lines) + "\n")
