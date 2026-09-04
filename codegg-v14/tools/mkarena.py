"""mkarena.py -- build THE SECOND ARENA (v13-M3d).

The sealed 20 + 3 do not move: this is a SEPARATE, clearly labelled suite with
its own totals, for the formats the sealed corpus has none of. Real files off
this machine where real files exist, constructed ones where they do not -- and
every member's provenance is written into suite.txt beside it, so nobody has to
guess later where a byte came from.

  python tools/mkarena.py [outdir]
"""

import io
import os
import shutil
import struct
import sys
import zlib

OUT = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.path.dirname(__file__), "..", "corpus-arena")
DL = os.path.join(os.environ.get("USERPROFILE", r"C:\Users\vcepe"), "Downloads")
HERE = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

os.makedirs(OUT, exist_ok=True)
prov = []


def take(name, src, note):
    if not os.path.exists(src):
        print("MISSING, not faked:", src)
        return
    dst = os.path.join(OUT, name)
    shutil.copyfile(src, dst)
    prov.append((name, os.path.getsize(dst), note))
    print("copied ", name, os.path.getsize(dst))


def put(name, data, note):
    with open(os.path.join(OUT, name), "wb") as f:
        f.write(data)
    prov.append((name, len(data), note))
    print("built  ", name, len(data))


# ------------------------------------------------------------------ real files
take("docx-cover-sheet.docx", os.path.join(DL, "Topic_1_Test_Cover_Sheet.docx"),
     "a real Word document off this machine (Downloads); a ZIP of 12 members, 11 deflate")
take("docx-newsletter.docx", os.path.join(DL, "BRACE_Frsh_Soph_Newsletter_#1.docx"),
     "a real Word document off this machine (Downloads)")
take("zip-opencv-python.zip", os.path.join(DL, "opencv-python-88.zip"),
     "a real ZIP off this machine (Downloads)")
take("zip-lab-guides.zip", os.path.join(DL, "Win11_25H2_Lab_Guides.zip"),
     "a real ZIP off this machine (Downloads)")
take("pdf-install-manual.pdf", os.path.join(DL, "AW3423DW_Firmware", "DellAW3423DWSoftwareInstallionManual.pdf"),
     "a real vendor PDF off this machine (Downloads); FlateDecode streams inside an object graph")

# ------------------------------------------------------------------ constructed
JPG = os.path.join(HERE, "corpus-jpeg", "win_Wallpaper_ThemeB_img26.jpg")
if os.path.exists(JPG):
    src = open(JPG, "rb").read()
    buf = io.BytesIO()
    import gzip
    g = gzip.GzipFile(fileobj=buf, mode="wb", compresslevel=6, mtime=0)
    g.write(src)
    g.close()
    put("chain-jpeg.gz", buf.getvalue(),
        "CONSTRUCTED: corpus-jpeg/win_Wallpaper_ThemeB_img26.jpg through python gzip -6. "
        "The member inside the member -- the proof the CHAIN reaches depth 2")

# a multi-member gzip: two gzip streams concatenated, which is legal and which
# every gzip decoder joins
import gzip
parts = []
for i, text in enumerate([b"the first member, and it repeats: " * 500, b"the second member, different bytes: " * 500]):
    b = io.BytesIO()
    g = gzip.GzipFile(fileobj=b, mode="wb", compresslevel=9, mtime=0)
    g.write(text)
    g.close()
    parts.append(b.getvalue())
put("multimember.gz", b"".join(parts),
    "CONSTRUCTED: two gzip members concatenated (legal, and every gzip decoder joins them)")


# ------------------------------------------------------------------ the 284 spelling
class BitW:
    def __init__(self):
        self.by = bytearray()
        self.acc = 0
        self.n = 0

    def put(self, v, nbits):
        """deflate order: LSB first"""
        for i in range(nbits):
            self.acc |= ((v >> i) & 1) << self.n
            self.n += 1
            if self.n == 8:
                self.by.append(self.acc)
                self.acc = 0
                self.n = 0

    def putrev(self, v, nbits):
        """Huffman codes are packed MSB-first within the code"""
        for i in range(nbits - 1, -1, -1):
            self.acc |= ((v >> i) & 1) << self.n
            self.n += 1
            if self.n == 8:
                self.by.append(self.acc)
                self.acc = 0
                self.n = 0

    def flush(self):
        if self.n:
            self.by.append(self.acc)
            self.acc = 0
            self.n = 0
        return bytes(self.by)


def fixed_lit(b):
    """the fixed literal/length code of RFC 1951, 3.2.6"""
    if b <= 143:
        return (0x30 + b, 8)
    if b <= 255:
        return (0x190 + b - 144, 9)
    if b <= 279:
        return (b - 256, 7)
    return (0xC0 + b - 280, 8)


def build284(nmatches):
    """a fixed-Huffman deflate block whose 258-byte matches are spelled with
    symbol 284 + 31 extra bits, which is the SECOND legal spelling and the one
    the site's canonicalisation rule says to record rather than refuse."""
    lit = bytes((i * 37 + 11) & 0xFF for i in range(258))
    raw = bytearray(lit)
    w = BitW()
    w.put(1, 1)  # final
    w.put(1, 2)  # fixed Huffman
    for b in lit:
        c, n = fixed_lit(b)
        w.putrev(c, n)
    for _ in range(nmatches):
        c, n = fixed_lit(284)
        w.putrev(c, n)
        w.put(31, 5)  # LBASE[27] = 227, 227 + 31 = 258
        w.putrev(16, 5)  # distance symbol 16: base 257, 7 extra bits
        w.put(1, 7)  # 257 + 1 = 258
        raw.extend(raw[-258:])
    c, n = fixed_lit(256)
    w.putrev(c, n)
    body = w.flush()
    head = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\xff"
    tail = struct.pack("<II", zlib.crc32(bytes(raw)) & 0xFFFFFFFF, len(raw) & 0xFFFFFFFF)
    gz = head + body + tail
    # the gate: python's own inflater must agree, or this file is a lie
    assert zlib.decompress(gz, 16 + zlib.MAX_WBITS) == bytes(raw), "the constructed member does not inflate"
    return gz


put("constructed-284.gz", build284(2),
    "CONSTRUCTED: a fixed-Huffman gzip whose two 258-byte matches are spelled with symbol 284 "
    "+ 31 extra bits (the second legal spelling). Verified against python zlib before it was written. "
    "Before M3a this file was REFUSED outright")
put("constructed-284-many.gz", build284(200),
    "CONSTRUCTED: the same, with 200 such matches -- the size at which the spelling list rides its own arm")

with open(os.path.join(OUT, "suite.txt"), "w", encoding="utf-8") as f:
    f.write("# the second arena (v13-M3d). Reported apart from the sealed 20 + 3.\n")
    for n, sz, note in prov:
        f.write("%s %d %s\n" % (n, sz, note))
print("\n%d members in %s" % (len(prov), os.path.abspath(OUT)))
