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


MAX_PUSH = 64                                # per name, so a page stays a page


def api_sort(q, lang, push="", read="sound"):
    """Two independent axes.

    read  "spell"  the string as written, in Chroma UTF codes
          "sound"  its phonetic reading

    push  ""       one row per name
          "ctx"    one row per representation the positional rules allow
          "all"    the same with the rules off

    So all four combinations exist. Pushed without phonetic is the SHAPE axis --
    cervezas and c3rv3zas are one thing written two ways. Pushed with phonetic
    is the sound axis, and there the rules matter: in context cervezas has no
    /k/ reading, out of context it has kervezas.
    """
    C, P, S = ordering()
    items = [l for l in q.split("\n") if l.strip()]
    ipa = lambda r: "".join(x[1] for x in r if x[1] not in ("", "\u00b7"))
    spell_axis = read == "spell"
    if not push:
        rows = []
        keyed = [(S._k(n, S.spellings(n)[0]) if spell_axis else S.key(n, lang or None), n)
                 for n in items]
        keyed.sort(key=lambda x: x[0][0])
        for (k, spell, r), n in keyed:
            rows.append({"name": n, "reading": spell, "ipa": ipa(r),
                         "codes": [c for c in C.letters(spell)]})
        return {"lang": lang, "push": "", "read": read, "sorted": rows}
    ent = []
    for n in items:
        mine, seen = [], set()
        variants = (S.spellings(n, True) if spell_axis
                    else S.readings(n, lang or None, push == "ctx"))
        for r in variants:
            k, spell, rr = S._k(n, r)
            if spell in seen: continue
            seen.add(spell)
            mine.append((k, {"name": n, "reading": spell, "ipa": ipa(rr),
                             "codes": [c for c in C.letters(spell)]}))
        # sort BEFORE capping, so the cap keeps the lowest N in order rather
        # than whichever the branch product happened to emit first
        mine.sort(key=lambda x: x[0])
        total, shown = len(mine), mine[:MAX_PUSH]
        for _k2, row in shown:
            row["of"] = total
            row["truncated"] = total > MAX_PUSH
        ent += shown
    ent.sort(key=lambda x: x[0])
    return {"lang": lang, "push": push, "read": read,
            "sorted": [r for _k, r in ent]}


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
        push = (qs.get("push") or [""])[0]
        read = (qs.get("read") or ["sound"])[0]
        if len(q) > MAX_Q or len(lang) > 64 or len(push) > 8 or len(read) > 8:
            self.send_error(413, "too long"); return
        try:
            if u.path == "/api/read":    body = api_read(q, lang)
            elif u.path == "/api/sort":  body = api_sort(q, lang, push, read)
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
    # the two push levels: in context cervezas has no /k/ reading, out of it it does
    ctx = {r["reading"] for r in api_sort("cervezas", "", "ctx")["sorted"]}
    alle = {r["reading"] for r in api_sort("cervezas", "", "all")["sorted"]}
    assert "kervezas" not in ctx and "servezas" in ctx, sorted(ctx)[:4]
    assert "kervezas" in alle, sorted(alle)[:4]
    # the subset property belongs to the readings, not to a capped display
    _C, _P, _S = ordering()
    rd = lambda c: {"".join(x[0] for x in r) for r in _S.readings("cervezas", None, c)}
    assert rd(True) < rd(False), "rules-on must be a strict subset of rules-off"
    # the two axes are independent, so all four combinations exist
    sp = api_sort("cervezas", "", "", "spell")["sorted"]
    assert len(sp) == 1 and sp[0]["reading"] == "cervezas", sp
    spp = {r["reading"] for r in api_sort("cervezas", "", "ctx", "spell")["sorted"]}
    assert "c3rv3zas" in spp and "cervezas" in spp, sorted(spp)[:4]
    sn = api_sort("cervezas", "en", "", "sound")["sorted"]
    assert sn[0]["reading"] == "servezas", sn
    # the shape axis is orthogonal to the sound axis, not a rival candidate
    assert api_read("c3rv3zas", "en leet")["reading"] == "servezas"
    assert api_read("c3rv3zas", "en")["reading"] == "k3rv3zas"
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
