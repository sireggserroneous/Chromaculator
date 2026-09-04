"""mkdeflatesuite.py -- rebuild the deflate peel's conservation suite.

M2 built this directory in scratch and it did not survive; M3a rebuilt it and it
did not survive either. It is a BUILDER now, so the next milestone does not have
to invent one: every member is generated here, from python's own zlib plus a
handful of hostiles, and its provenance is written into suite.txt beside it.

  python tools/mkdeflatesuite.py [outdir]     default: corpus-deflate
"""

import io
import os
import struct
import sys
import zlib

OUT = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.path.dirname(__file__), "..", "corpus-deflate")
HERE = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
os.makedirs(OUT, exist_ok=True)
prov = []


def put(name, data, note):
    with open(os.path.join(OUT, name), "wb") as f:
        f.write(data)
    prov.append((name, len(data), note))


# the payload: real bytes, not noise -- a deflate stream over noise has no matches
SRC = open(os.path.join(HERE, "corpus-real", "wubbadub.html"), "rb").read()
TXT = open(os.path.join(HERE, "corpus-real", "vim-version9.txt"), "rb").read()[: 1 << 19]


def gz(data, level, strategy=zlib.Z_DEFAULT_STRATEGY, wbits=31, memlevel=8):
    co = zlib.compressobj(level, zlib.DEFLATED, wbits, memlevel, strategy)
    return co.compress(data) + co.flush()


STRAT = {
    "default": zlib.Z_DEFAULT_STRATEGY,
    "filtered": zlib.Z_FILTERED,
    "huffonly": zlib.Z_HUFFMAN_ONLY,
    "rle": zlib.Z_RLE,
}
for lvl in range(1, 10):
    put("gz-l%d.gz" % lvl, gz(SRC, lvl), "python zlib gzip, level %d, default strategy, over corpus-real/wubbadub.html" % lvl)
for name, st in STRAT.items():
    put("gz-%s.gz" % name, gz(TXT, 9, st), "python zlib gzip, level 9, strategy %s, over 512 KB of corpus-real/vim-version9.txt" % name)
put("zlib-l6.zz", gz(SRC, 6, wbits=15), "python zlib, zlib wrapper (not gzip), level 6")
put("bare-l6.deflate", gz(SRC, 6, wbits=-15), "python zlib, BARE deflate stream, no wrapper, level 6")
put("smallwindow.gz", gz(TXT, 9, wbits=9 + 16), "python zlib gzip, level 9, 512-byte window -- many short distances")
put("memlevel1.gz", gz(TXT, 9, memlevel=1), "python zlib gzip, level 9, memLevel 1 -- forces many small blocks")
put("stored-only.gz", gz(bytes(range(256)) * 64, 0), "python zlib gzip, level 0 -- STORED blocks only")
put("empty.gz", gz(b"", 6), "python zlib gzip of zero bytes")
put("onebyte.gz", gz(b"x", 6), "python zlib gzip of one byte")
put("binary.gz", gz(bytes((i * 131 + 7) & 0xFF for i in range(1 << 18)), 9), "python zlib gzip of a periodic binary sequence")

# ---- the hostiles: each must be REFUSED with a reason and keep its bytes
put("hostile-truncated.gz", gz(SRC, 6)[:-40], "HOSTILE: a gzip with its last 40 bytes cut off")
put("hostile-badcrc.gz", gz(SRC, 6)[:-8] + b"\x00\x00\x00\x00" + struct.pack("<I", len(SRC)),
    "HOSTILE: a gzip whose CRC32 is zeroed")
body = bytearray(gz(SRC, 6))
body[40] ^= 0xFF
put("hostile-flipped.gz", bytes(body), "HOSTILE: a gzip with one byte of its deflate body flipped")
put("hostile-notgzip.gz", b"\x1f\x8b\x08\x00" + b"\x00" * 6 + b"garbage that is not deflate at all",
    "HOSTILE: a gzip header over bytes that are not a deflate stream")
put("hostile-empty.gz", b"", "HOSTILE: zero bytes")
put("hostile-headeronly.gz", b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\xff", "HOSTILE: a gzip header and nothing after it")

# ---- and the two constructed 284-spelling members, from the arena's builder
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
ARENA = os.path.join(HERE, "corpus-arena")
for n in ("constructed-284.gz", "constructed-284-many.gz"):
    p = os.path.join(ARENA, n)
    if os.path.exists(p):
        put(n, open(p, "rb").read(), "CONSTRUCTED by tools/mkarena.py: 258-byte matches spelled with symbol 284 + 31 extra bits")

with open(os.path.join(OUT, "suite.txt"), "w", encoding="utf-8") as f:
    f.write("# the deflate peel's conservation suite, rebuilt by tools/mkdeflatesuite.py\n")
    for n, sz, note in prov:
        f.write("%s %d %s\n" % (n, sz, note))
print("%d members in %s" % (len(prov), os.path.abspath(OUT)))
