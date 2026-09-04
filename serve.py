#!/usr/bin/env python3
"""Static server for chronochromatic, plus PUT so documents can be edited in the page.

Only files named in WRITABLE may be written, and only at the top level -- the
path is compared as a bare name, so no traversal reaches outside the root.

It also serves a read-only /api/ for the Chroma ordering. The ordering has one
implementation, in Python, and the page asks for readings rather than carrying
a second copy of the tables -- a change to the tables is a change everywhere.
"""
import http.server, json, os, pathlib, socketserver, sys, urllib.parse

MAX_Q = 4000                                 # the API reads text, and only text
_mods = None


def ordering():
    """Import the ordering on first use, so the static server starts instantly."""
    global _mods
    if _mods is None:
        sys.path.insert(0, str(pathlib.Path(__file__).parent / "tools"))
        import chroma_utf, chroma_phonetic, chroma_sort
        _mods = (chroma_utf, chroma_phonetic, chroma_sort)
    return _mods


def api_read(q, lang):
    """Every grapheme of q: its branches, its reading, its Chroma UTF codes."""
    C, P, S = ordering()
    segs = S.segment(q)
    out = []
    for i, seg in enumerate(segs):
        if seg == S.SEP:
            out.append({"seg": "", "sep": True, "branches": []}); continue
        br = S.branches(seg, i, segs, q)
        allbr = S.branches(seg, i, segs, q) if not lang else None
        keep = br
        if lang:
            want = set()
            for x in lang.split():
                want.add(x)
                if "-" in x: want.add(x.split("-")[0])
            f = [b for b in br if set(b[2]) & want or "und" in b[2]]
            keep = f or br
        prim = keep[0]
        out.append({"seg": seg, "sep": False,
                    "reading": prim[0], "ipa": prim[1],
                    "codes": [c for c in C.letters(prim[0])],
                    "branches": [{"reading": b[0], "ipa": b[1], "langs": b[2]}
                                 for b in keep]})
    k, spell, r = S.key(q, lang or None)
    return {"input": q, "lang": lang, "segments": out, "reading": spell,
            "ipa": "".join(x[1] for x in r if x[1] not in ("", "\u00b7")),
            "codes": [c for c in C.letters(spell)],
            "ring": C.RING, "width": C.RING + 1,
            "positions": [x[1] for x in S.positions(q, lang or None)][:24]}


def api_sort(q, lang):
    C, P, S = ordering()
    items = [l for l in q.split("\n") if l.strip()]
    rows = []
    for n in S.chroma_sorted(items, lang or None):
        k, spell, r = S.key(n, lang or None)
        rows.append({"name": n, "reading": spell,
                     "ipa": "".join(x[1] for x in r if x[1] not in ("", "\u00b7")),
                     "codes": [c for c in C.letters(spell)]})
    return {"lang": lang, "sorted": rows}


def api_base():
    C, P, S = ordering()
    return {"ring": C.RING, "floor": 1 << C.RING, "count": len(C.TABLE),
            "table": [{"ch": ch, "code": C.UTF[ch]} for ch in C.TABLE]}

ROOT = pathlib.Path(__file__).parent.resolve()
WRITABLE = {"spec.md"}
MAX_BYTES = 1_000_000


class Handler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if not self.path.startswith("/api/"):
            return super().do_GET()
        u = urllib.parse.urlparse(self.path)
        qs = urllib.parse.parse_qs(u.query)
        q = (qs.get("q") or [""])[0]
        lang = (qs.get("lang") or [""])[0]
        if len(q) > MAX_Q or len(lang) > 64:
            self.send_error(413, "too long"); return
        try:
            if u.path == "/api/read":    body = api_read(q, lang)
            elif u.path == "/api/sort":  body = api_sort(q, lang)
            elif u.path == "/api/base":  body = api_base()
            else:
                self.send_error(404, "no such endpoint"); return
        except Exception as e:
            self.send_error(500, f"{type(e).__name__}"); return
        raw = json.dumps(body, ensure_ascii=False).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

    def do_PUT(self):
        name = self.path.lstrip("/")
        if name not in WRITABLE:
            self.send_error(403, "not writable")
            return
        try:
            n = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            self.send_error(400, "bad length")
            return
        if n < 0 or n > MAX_BYTES:
            self.send_error(413, "too large")
            return
        body = self.rfile.read(n)
        target = ROOT / name
        tmp = target.with_suffix(target.suffix + ".tmp")
        tmp.write_bytes(body)
        tmp.replace(target)                      # atomic, never a half-written spec
        self.send_response(204)
        self.end_headers()

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


def demo():
    """Self-check: the whitelist is the only thing standing between a PUT and the disk."""
    assert "spec.md" in WRITABLE
    for bad in ("../fold.js", "/etc/passwd", "atlas.html", "spec.md/../fold.js", ""):
        assert bad.lstrip("/") not in WRITABLE, bad
    # the API is read only, and takes text. Nothing it receives reaches the disk.
    r = api_read("shit", "en")
    assert r["reading"] == "shit" and r["ipa"] == "\u0283it", r
    assert api_read("\u98fc", "")["reading"] == "si"
    assert api_read("\u98fc", "ja-on")["reading"] == "shi"
    assert api_read("cervezas", "en")["reading"] == "servezas"
    s = api_sort("canvas\nclear\ncervezas\ndox\nknicks\nkicks", "en")
    assert [x["name"] for x in s["sorted"]] == \
        ["dox", "canvas", "kicks", "clear", "knicks", "cervezas"], s
    b = api_base()
    assert b["count"] == 306 and b["ring"] == 9 and b["table"][0]["code"] == 512
    print("serve.py self-check ok")


if __name__ == "__main__":
    import sys
    if "--check" in sys.argv:
        demo()
    else:
        port = 1338
        if "--port" in sys.argv:
            port = int(sys.argv[sys.argv.index("--port") + 1])
        os.chdir(ROOT)
        socketserver.TCPServer.allow_reuse_address = True
        with socketserver.TCPServer(("0.0.0.0", port), Handler) as s:
            print(f"chronochromatic on http://localhost:{port}")
            s.serve_forever()
