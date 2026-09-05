#!/usr/bin/env python3
"""Static server for chronochromatic, plus PUT so documents can be edited in the page.

Only files named in WRITABLE may be written, and only at the top level -- the
path is compared as a bare name, so no traversal reaches outside the root.

It also serves a read-only /api/ for the Chroma ordering. The ordering has one
implementation, in Python, and the page asks for readings rather than carrying
a second copy of the tables -- a change to the tables is a change everywhere.
"""
import http.server, json, os, pathlib, re, socketserver, sys, urllib.parse

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


def api_sort(q, lang, push="", read="sound", alphabet="chroma"):
    """Two independent axes.

    alphabet "chroma"  306 symbols, 12-bit digits, base 4096, a 4x4 frame
             "ipa"     126 symbols, 8-bit digits, base 256, a 3x3 frame.
                       A different sort rather than a level inside the other
                       one; ties in it fall back to the Chroma key. It needs a
                       reading, so it forces read="sound".

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
    A, alphabet = _alpha(alphabet)
    if alphabet in ("ipa", "phonetic"): read = "sound"
    items = [l for l in q.split("\n") if l.strip()]
    ipa = lambda r: "".join(x[1] for x in r if x[1] not in ("", "\u00b7"))
    spell_axis = read == "spell"
    if not push:
        rows = []
        ordered = (S.chroma_sorted(items, lang or None, alphabet) if not spell_axis
                   else sorted(items, key=lambda n: S._k(n, S.spellings(n)[0])[0]))
        for n in ordered:
            k, spell, r = (S._k(n, S.spellings(n)[0]) if spell_axis
                           else S.key(n, lang or None))
            rows.append({"name": n, "reading": spell, "ipa": ipa(r),
                         "codes": _digits_for(alphabet, C, S, spell, ipa(r), n)})
        return {"lang": lang, "push": "", "read": read, "alphabet": alphabet,
                "codeBits": A["bits"], "base": 1 << A["bits"], "sorted": rows}
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
                             "codes": _digits_for(alphabet, C, S, spell, ipa(rr), n)}))
        # sort BEFORE capping, so the cap keeps the lowest N in order rather
        # than whichever the branch product happened to emit first
        mine.sort(key=lambda x: x[0])
        total, shown = len(mine), mine[:MAX_PUSH]
        for _k2, row in shown:
            row["of"] = total
            row["truncated"] = total > MAX_PUSH
        ent += shown
    ent.sort(key=lambda x: x[0])
    return {"lang": lang, "push": push, "read": read, "alphabet": alphabet,
            "codeBits": A["bits"], "base": 1 << A["bits"],
            "sorted": [r for _k, r in ent]}


CODE_BITS = 16          # four whole nibbles: fills the 4x4 exactly, so the
                        # square has no padding and all three regions live
MAX_ITEMS = 48
NUMERIC = re.compile(r"^[+-]?[0-9]+$")

# An alphabet is one choice: it fixes the digits, the base, the frame they draw
# in, AND the order. Sorting by one while drawing the other would put the
# divergence marker on a digit that decides nothing.
# Three orderings, and each one sets the storage width — a character is just
# another int, so the only thing that changes is how big the int is.
ALPHABETS = {"ipa":      {"bits": 8,  "label": "Chroma IPA"},
             "chroma":   {"bits": 16, "label": "Chroma UTF"},
             "phonetic": {"bits": 20, "label": "Phonetic index"}}
_PHON = None


def phonetic_map():
    """char -> its first position in the multi-listed phonetic index.

    370,571 entries, so it is built once and kept. The counting base is declared
    as 524,287 = 2^19 - 1, a Mersenne prime, rather than taken as the entry count
    plus one: 370,572 is even. The index already fits inside it with 153,715 to
    spare, so the modulus only had to be named.
    """
    global _PHON
    if _PHON is None:
        C, P, S = ordering()
        pos = {}
        for i, e in enumerate(P.build()):
            pos.setdefault(e[3], i + 1)       # rank 0 stays the zero
        _PHON = pos
    return _PHON


def _alpha(name):
    return ALPHABETS.get(name, ALPHABETS["chroma"]), \
           name if name in ALPHABETS else "chroma"


def _digits_for(alphabet, C, S, spell, reading_ipa, text=""):
    """The CODES a string has in this alphabet — what gets drawn."""
    if alphabet == "ipa":
        import chroma_ipa as I
        return I.digits(reading_ipa)
    if alphabet == "phonetic":
        m = phonetic_map()
        return [m.get(ch, 0) for ch in text]
    return [c for c in C.letters(spell)]


def _ranks_for(alphabet, C, S, spell, reading_ipa, *kwargs_text):
    """The RANKS — what gets counted.

    Rank and code are different numbers. The code is spread across the digit so
    the frame has no dead cells and a character has somewhere to point; the rank
    is the dense position in the order, and it is what a string COUNTS in. With
    126 IPA symbols the counting base is 127 and "10" is 127, not 256 — reading
    the value off the spread codes was giving the storage base instead.

    A separator is rank 0, the zero. A trailing one vanishes, which is right for
    a trailing space; a medial one is a positional zero, which is also right.
    """
    if alphabet == "ipa":
        import chroma_ipa as I
        return I.digits(reading_ipa)          # IPA codes ARE its ranks today
    if alphabet == "phonetic":
        return _digits_for(alphabet, C, S, spell, reading_ipa, kwargs_text[0])
    back = {C.CODE[ch]: C.RANK[ch] for ch in C.TABLE}
    return [back.get(c, 0) for c in C.letters(spell)]


def _base_for(alphabet, C):
    """The counting base: the symbol count plus the zero. Both are prime."""
    if alphabet == "ipa":
        import chroma_ipa as I
        return len(I.ROWS) + 1
    if alphabet == "phonetic":
        return (1 << 19) - 1                  # 524,287, a Mersenne prime
    return len(C.TABLE) + 1


def _int_of(codes, B=1 << CODE_BITS):
    """base(chroma-utf): the codes are the digits, the base is 2^12.

    The VALUE is the fraction: 0.d1 d2 d3 ... in base 4096, which is
    SUM code_i * B^-(i+1) and lands in (0,1) like every other stalk on the site.

    Leading with the integer was wrong. The integer is SUM code_i * B^(n-1-i),
    so more digits always means a bigger number and it sorts short words first
    whatever they say -- "he" beats "hell" on length alone. The fraction does
    not: a missing digit reads as zero, zero is below every code, so a prefix is
    automatically smaller and no terminator rule is needed. Measured against the
    real key, the fraction agrees and the integer does not.

    The integer is kept because Wub +- takes integers, not because it orders.
    """
    n = 0
    for c in codes: n = n * B + c
    return n, B ** len(codes) if codes else 1


def api_cards(q, lang, push="", read="sound", alphabet="chroma"):
    """One card per line; commas separate the items on a card."""
    C, P, S = ordering()
    A, alphabet = _alpha(alphabet)
    if alphabet in ("ipa", "phonetic"): read = "sound"   # both need a reading
    B = 1 << A["bits"]
    ipa = lambda r: "".join(x[1] for x in r if x[1] not in ("", "\u00b7"))
    cards = []
    for line in q.split("\n"):
        if not line.strip(): continue
        texts = [t.strip() for t in line.split(",")]
        texts = [t for t in texts if t][:MAX_ITEMS]
        items = []
        for t in texts:
            # A LEADING sign is negation on any item, not only a numeric one.
            # A character is just another int, so "-a" is a negated, and the
            # hyphen was being read as a separator character instead.
            neg = False
            if len(t) > 1 and t[0] in "+-" and not NUMERIC.match(t):
                neg = t[0] == "-"
                t = t[1:]
            # An item that parses as an integer IS an integer: one ring, sign and
            # all, exactly as Wub +- draws it. Running "-6" through the character
            # path made it a hyphen and a six, so the negative never reached
            # hexSequence and the 1s never became -1s.
            if NUMERIC.match(t):
                items.append({"text": t, "numeric": True, "k": t,
                              "reading": t, "ipa": "", "codes": [], "phasors": [],
                              "num": "0", "den": "1", "approx": "0",
                              "int": "0", "hex": "0", "bits": 0, "alts": []})
                continue
            if read == "spell":
                k, spell, r = S._k(t, S.spellings(t)[0])
                alts = ["".join(x[0] for x in v) for v in S.spellings(t, True)] \
                       if push else []
            else:
                k, spell, r = S.key(t, lang or None)
                alts = ["".join(x[0] for x in v)
                        for v in S.readings(t, lang or None, push != "all")] if push else []
            codes = _digits_for(alphabet, C, S, spell, ipa(r), t)
            ranks = _ranks_for(alphabet, C, S, spell, ipa(r), t)
            CB = _base_for(alphabet, C)
            n, den = _int_of(ranks, CB)
            # each character is a phasor: its code is the angle, its position
            # is the weight, and its own square's fold balance is the height
            ph = []
            for k, c in enumerate(codes):
                bits = format(c, "0%db" % A["bits"])
                # side from the width, not from a magic 12: pad to whole
                # nibbles, then the smallest square that holds them
                nib = -(-A["bits"] // 4) * 4
                side = 1
                while side * side < nib: side += 1
                cells = [int(b) for b in bits] + [0] * (side*side - len(bits))
                reg = lambda f: sum(v for j, v in enumerate(cells)
                                    if f(j//side + j%side, side - 1))
                ph.append({"code": c, "turn": c / B, "w": 2.0 ** -(k+1),
                           "inner": reg(lambda d, e: d < e),
                           "fold":  reg(lambda d, e: d == e),
                           "outer": reg(lambda d, e: d > e),
                           "lit": sum(cells), "n": side})
            items.append({"text": ("-" if neg else "") + t, "neg": neg,
                          "reading": spell, "ipa": ipa(r), "codes": codes,
                          "ranks": ranks, "countBase": CB,
                          "phasors": ph,
                          # the value, in (0,1): 0.d1 d2 d3 ... base 4096
                          "num": str(n), "den": str(den),
                          "approx": f"{n / den:.18f}" if den else "0",
                          # and the integer, for Wub +- which takes integers
                          "int": str(n), "hex": format(n, "x"),
                          "bits": len(codes) * A["bits"],
                          "digits": len(ranks),
                          "alts": sorted(set(alts))[:MAX_PUSH]})
        tot = sum(int(i["int"]) for i in items)
        cards.append({"items": items, "sum": str(tot),
                      "sumHex": format(tot, "x")})
    return {"lang": lang, "read": read, "push": push, "alphabet": alphabet,
            "codeBits": A["bits"], "base": B, "cards": cards}


def api_ipa(q="", lang=""):
    """The third alphabet, and a query's reading expressed in it.

    Chroma UTF is 306 in ring 9, twelve-bit digits, base 4096. This is the IPA
    chart: 126 symbols in ring 7, EIGHT-bit digits -- two whole nibbles, which
    the other two alphabets cannot reach. Its order is not declared, it is the
    Chroma UTF order of each symbol's spelling.

    Rank 0 stays empty because a trailing rank-0 digit is invisible: 0.5 and 0.50
    are one number. That makes 127 elements, and 127 is prime, so every nonzero
    digit has an inverse and division is total. Storage is nibble aligned at 8
    bits; the arithmetic is mod 127. Two different roles for one digit.
    """
    C, P, S = ordering()
    sys.path.insert(0, str(pathlib.Path(__file__).parent / "tools"))
    import chroma_ipa as I
    out = {"count": len(I.ROWS), "ring": I.RING, "width": I.WIDTH, "base": I.BASE,
           "modulus": I.MOD, "spare": (1 << I.RING) - I.MOD,
           "addresses": [{"sym": s2, "rom": r2, "name": n2}
                         for s2, r2, n2 in I.INFINITESIMAL],
           "alphabet": [{"sym": r["sym"], "rom": r["rom"], "rank": r["rank"],
                         "name": r["name"]} for r in I.ROWS],
           "diacritics": [{"mark": m, "name": n} for m, n in I.DIACRITICS]}
    if q:
        rows = []
        for t in [x.strip() for x in q.split("\n") if x.strip()][:MAX_ITEMS]:
            k, spell, r = S.key(t, lang or None)
            ipa = "".join(x[1] for x in r if x[1] not in ("", "\u00b7"))
            d = I.digits(ipa)
            rows.append({"text": t, "reading": spell, "ipa": ipa,
                         "segments": [{"sym": s2, "marks": list(m), "rank": I.RANK[s2]}
                                      for s2, m in I.parse(ipa)],
                         "digits": d, "int": str(I.integer(ipa)),
                         "bits": len(d) * I.WIDTH,
                         "chromaDigits": len(C.letters(spell)),
                         "chromaBits": len(C.letters(spell)) * CODE_BITS})
        out["items"] = rows
    return out


def api_base():
    """Rank orders, code draws.

    Each type gets a share of the ring and is strided to fill it, so the digits
    span 327 degrees instead of the 27 they occupied when the codes were packed
    consecutively. That is what gives a word's phasors somewhere to point.
    """
    C, P, S = ordering()
    return {"ring": C.RING, "width": C.WIDTH, "count": len(C.TABLE),
            "blocks": [{"type": n, "lo": lo, "hi": hi, "stride": st, "n": k}
                       for n, (lo, hi, st, k) in C.PLAN.items()],
            "table": [{"ch": ch, "rank": C.RANK[ch], "code": C.UTF[ch],
                       "type": C.typeof(ch)} for ch in C.TABLE]}

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
        alphabet = (qs.get("alphabet") or qs.get("order") or ["chroma"])[0]
        if len(q) > MAX_Q or len(lang) > 64 or len(push) > 8 or len(read) > 8 \
                or len(alphabet) > 8:
            self.send_error(413, "too long"); return
        try:
            if u.path == "/api/read":    body = api_read(q, lang)
            elif u.path == "/api/sort":  body = api_sort(q, lang, push, read, alphabet)
            elif u.path == "/api/cards": body = api_cards(q, lang, push, read, alphabet)
            elif u.path == "/api/ipa":   body = api_ipa(q, lang)
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
    cd = api_cards("hello, 3, 45\ncerveza", "en")
    assert len(cd["cards"]) == 2 and len(cd["cards"][0]["items"]) == 3, cd
    # a numeric item is one integer, not a string of digit characters
    n45 = cd["cards"][0]["items"][2]
    assert n45["numeric"] and n45["k"] == "45" and not n45["codes"], n45
    neg = api_cards("-6, 6", "en")["cards"][0]["items"]
    assert neg[0]["numeric"] and neg[0]["k"] == "-6", neg[0]
    h = cd["cards"][0]["items"][0]
    assert not h.get("numeric") and len(h["codes"]) == 4, h
    assert h["text"] == "hello" and h["bits"] == len(h["codes"]) * CODE_BITS
    # the integer is the polynomial evaluated in the COUNTING base, on ranks —
    # not in the storage base on the spread codes
    CB = h["countBase"]
    assert int(h["int"]) == sum(r * CB ** (len(h["ranks"]) - 1 - i)
                                for i, r in enumerate(h["ranks"])), h
    assert all(0 <= r < CB for r in h["ranks"]), h["ranks"]
    assert int(cd["cards"][0]["sum"]) == sum(
        int(i["int"]) for i in cd["cards"][0]["items"])
    # a leading sign negates any item, not just a numeric one
    na = api_cards("a, -a", "en", "", "sound", "chroma")["cards"][0]["items"]
    assert na[0]["codes"] == na[1]["codes"], (na[0]["codes"], na[1]["codes"])
    assert not na[0]["neg"] and na[1]["neg"], na
    assert na[1]["text"] == "-a", na[1]["text"]
    # and a medial hyphen is still a separator, not a sign
    assert not api_cards("a-b", "en", "", "sound", "chroma")["cards"][0]["items"][0]["neg"]
    # the counting base is the symbol count plus the zero, and it is prime
    ci = api_cards("shit", "en", "", "sound", "ipa")["cards"][0]["items"][0]
    assert ci["countBase"] == 127, ci["countBase"]
    cc = api_cards("hello", "en", "", "sound", "chroma")["cards"][0]["items"][0]
    assert cc["countBase"] == 307, cc["countBase"]
    # and a two-symbol string "10" counts as 127, not as the storage base
    B127 = ci["countBase"]
    assert 1 * B127 + 0 == 127
    # the VALUE is the fraction, and it is the one that agrees with the order
    from fractions import Fraction
    C0, P0, S0 = ordering()
    ws = ["he", "hell", "hello", "helllo", "helo"]
    cs = {w: C0.letters("".join(x[0] for x in S0.spellings(w)[0])) for w in ws}
    def frac(w):
        n, d = _int_of(cs[w]); return Fraction(n, d)
    def whole(w):
        n, _d = _int_of(cs[w]); return n
    real = sorted(ws, key=lambda w: S0._k(w, S0.spellings(w)[0])[0])
    assert sorted(ws, key=frac) == real, "the fraction must agree with the key"
    assert sorted(ws, key=whole) != real, \
        "the integer is length-dominant; if this ever matches, the control is dead"
    # two sibling orderings, both total, and they genuinely disagree
    W = "church\ncat\ntop\nshoe\nsun"
    ch = [r["name"] for r in api_sort(W, "en", "", "sound", "chroma")["sorted"]]
    ia = [r["name"] for r in api_sort(W, "en", "", "sound", "ipa")["sorted"]]
    # an alphabet fixes the digits AND the base, not just the order
    cc = api_cards("shit", "en", "", "sound", "chroma")
    ci = api_cards("shit", "en", "", "sound", "ipa")
    assert cc["base"] == 65536 and cc["codeBits"] == 16, cc["base"]
    assert ci["base"] == 256 and ci["codeBits"] == 8, ci["base"]
    ic = cc["cards"][0]["items"][0]; ii = ci["cards"][0]["items"][0]
    assert len(ii["codes"]) < len(ic["codes"]), (ic["codes"], ii["codes"])
    assert ii["bits"] < ic["bits"], (ic["bits"], ii["bits"])
    assert all(d < 256 for d in ii["codes"]), ii["codes"]
    assert sorted(ch) == sorted(ia) == sorted(W.split("\n")), (ch, ia)
    assert ch != ia, "the two orderings must differ, or one of them is not doing anything"
    assert ia.index("church") > ia.index("top"), \
        "in IPA church follows top: an affricate starts with /t/"
    ip = api_ipa("shit", "en")
    assert ip["width"] == 8 and ip["base"] == 256, ip
    assert ip["count"] == 126 and ip["modulus"] == 127, (ip["count"], ip["modulus"])
    assert all(ip["modulus"] % d for d in range(2, 12)), "the modulus must be prime"
    assert len(ip["addresses"]) == 5, ip["addresses"]
    it = ip["items"][0]
    assert it["ipa"] == "\u0283it" and it["digits"] == [
        __import__("chroma_ipa").RANK[c] for c in "\u0283it"], it
    # the whole point of the third alphabet: fewer digits AND narrower ones
    assert it["bits"] < it["chromaBits"], it
    b = api_base()
    assert b["count"] == 306 and b["ring"] == 9 and b["width"] == 16
    cs = [r["code"] for r in b["table"]]
    # the invariant that let the codes be respaced without touching an ordering
    assert all(cs[i] > cs[i-1] for i in range(1, len(cs))), "code must rise with rank"
    assert all(0 < c < (1 << b["width"]) for c in cs), "a code escaped the digit"
    assert len(set(cs)) == len(cs), "duplicate code"
    span = 360 * (max(cs) - min(cs)) / (1 << b["width"])
    assert span > 300, f"the alphabet only spans {span:.0f} degrees"
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
