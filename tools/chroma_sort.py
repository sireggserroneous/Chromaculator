#!/usr/bin/env python3
"""chroma_sort — sort strings (filenames, words) phonetically.

A word is not a bag of characters. "shit" is /ʃ/ /ɪ/ /t/, not /s/ /h/ /ɪ/ /t/:
sh is one sound written with two letters. So a string is segmented into
graphemes first, longest match wins, and each grapheme carries the same
branches a character does.

A string with branching graphemes has more than one reading, so it occupies
more than one position — the same multi-listing as a character. To SORT you
need one position per string, so the key is the minimum over the readings the
declared language admits. Declare a different language, get a different (still
certified) order.

  python3 tools/chroma_sort.py shit 飼 this isit
  python3 tools/chroma_sort.py --lang cmn shit 飼 this isit
  ls ~/Downloads | python3 tools/chroma_sort.py -
"""
import sys, os, itertools, unicodedata as u
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import chroma_utf as C, chroma_phonetic as P

# ---------- digraphs: one sound, two letters ----------
# (ipa, chroma-utf spelling, languages, note)
# (ipa, chroma-utf spelling, languages, condition, note) — same shape and same
# condition grammar as the single-letter table in chroma_phonetic.
DIGRAPH = {
 "sh": [("ʃ","sh","en","",""),        ("sx","skh","nl","","")],
 "ch": [("tʃ","ch","en es","",""),    ("k","k","it gr","",""),
        ("ʃ","sh","fr pt","",""),     ("x","kh","de","","ach-laut")],
 "th": [("ð","dh","en","@the|this|that|these|those|them|then|than|they|their|there|thence|thither|thou|thee|thy|thine|though|thus|other|another|mother|father|brother|either|neither|whether|weather|rather|gather|together|bother|leather|feather|heather|breathe|clothe|smooth|with|within|without|northern|southern|worthy|farther|further|hither|whither|lathe|scythe","lexical, not positional: a closed list of "
                                  "function words and a few stems take /ð/"),
        ("θ","th","en","",""),        ("t","t","fr de it","","")],
 "ph": [("f","f","en fr de","","")],
 "wh": [("w","w","en","^",""),        ("h","h","en","^,>o","who, whom")],
 "ng": [("ŋ","ng","en de","",""),     ("nɡ","ng","it","","")],
 "qu": [("kw","kw","en it","",""),    ("k","k","fr es","","")],
 "ck": [("k","k","en de","","")],
 "gh": [("ɡ","g","en it","^","ghost"),
        ("f","f","en","$","enough, laugh"),
        ("","","en","","night — silent in the middle")],
 "kn": [("n","n","en","^","knee, knicks"), ("kn","kn","de","","")],
 "wr": [("ɹ","r","en","^","write")],
 "zh": [("ʒ","zh","en","","")],
 "dh": [("ð","dh","en ga","","")],
 "ll": [("j","y","es","",""),         ("l","l","en fr it","","")],
 "rr": [("r","r","es it","","")],
 "bh": [("v","v","ga","","abhan reads Avon")],
 "ts": [("ts","ts","de ja","","")],
 "ss": [("s","s","en de fr","","")],
 "ps": [("s","s","en","^","psalm")],
}
MAXG = max(len(g) for g in DIGRAPH)
MID = "\u00b7"
for _v in DIGRAPH.values():
    for _ipa, _rom, _l, _c, _n in _v: P.TAGS |= set(_l.split())
SEP = "\x01"                       # sorts below every letter, spells nothing

def branches(seg, i=0, segs=None, word=""):
    """-> [(chroma-utf spelling, ipa, [langs])] for one grapheme IN CONTEXT.

    A character alone has no context, which is why the index lists every branch.
    A word supplies it, so branches whose positional condition fails are gone:
    c before e is not /k/, and kn is only /n/ word-initially.
    """
    segs = segs if segs is not None else [seg]
    l = seg.lower()
    src = None
    if l in DIGRAPH:
        up = seg[0] != l[0]
        src = [((rom.upper() if up and rom else rom), ipa, langs.split(), cond)
               for ipa, rom, langs, cond, _ in DIGRAPH[l]]
    else:
        e = P.latin_entries(seg)
        if e: src = [(r, ipa, langs, cond) for r, ipa, langs, cond, _ in e]
    if src is not None:
        keep = [(r, ipa, langs) for r, ipa, langs, cond in src
                if P.cond_holds(cond, i, segs, word)]
        return keep or [(r, ipa, langs) for r, ipa, langs, _ in src]
    r, tone, st, layer, _ = C.reading(seg)
    if r is None: return [(SEP, "", ["und"])]
    if layer in ("han", "han-unread"):                 # every Unihan reading
        out = [(pr, ipa, [lang])
               for pr, lang, ipa, ch, tone, st in P.entries([ord(seg)])]
        return out or [(r, "", ["und"])]
    return [(r, "", ["und"])]

def segment(s):
    """Greedy longest-match. Separators become one low-sorting break."""
    out, i = [], 0
    while i < len(s):
        ch = s[i]
        cat = u.category(ch)
        if cat[0] in "PZC" or ch in "._-/ ":
            if not out or out[-1] != SEP: out.append(SEP)
            i += 1; continue
        for n in range(min(MAXG, len(s) - i), 1, -1):
            if s[i:i+n].lower() in DIGRAPH:
                out.append(s[i:i+n]); i += n; break
        else:
            out.append(ch); i += 1
    return out

def _readings_one(s, segs, accept):
    """Every reading ONE coherent language admits."""
    per = []
    for i, seg in enumerate(segs):
        if seg == SEP: per.append([(SEP, MID)]); continue
        b = branches(seg, i, segs, s)
        if accept:
            keep = [x for x in b if P.exact_match(x[2], accept)]
            b = keep or b                              # never erase a segment
        seen, uniq = set(), []
        for rom, ipa, _ in b:
            if rom not in seen: seen.add(rom); uniq.append((rom, ipa))
        per.append(uniq or [(SEP, MID)])
    n = 1
    for q in per: n *= len(q)
    if n > 4096:                                       # ponytail: cap the product
        return [tuple(q[0] for q in per)]              # greedy, first branch each
    return [tuple(c) for c in itertools.product(*per)]


def readings(s, lang=None):
    """Every reading the declared language admits, as (spelling, ipa).

    ONE coherent language at a time. The first candidate's first reading is the
    word's primary, so a reading is never assembled from two regions at once --
    picking the primary branch per grapheme gave cerveza as "serbetha", seseo c
    with distincion z, which is the reading the language coupling exists to
    rule out.
    """
    segs = segment(s)
    out, seen = [], set()
    for tag in P.candidates(lang):
        for r in _readings_one(s, segs, P.accept_for(tag)):
            k = tuple(x[0] for x in r)
            if k not in seen: seen.add(k); out.append(r)
    return out


def _k(s, r):
    spelling = "".join(x[0] for x in r)
    return (tuple(C.letters(spelling)) + (C.TERM,)
            + tuple(C.letters(u.normalize("NFD", s))),   # tie break: the spelling
            spelling, r)


# tag -> set of words. Optional: load it and a declared language set becomes a
# choice per word instead of a single rule for the whole list.
LEXICON = {}

def load_lexicon(tag, path, limit=None):
    """Register a word list for `tag`. Lowercased, so detection is caseless."""
    n = 0
    ws = LEXICON.setdefault(tag, set())
    with open(path, encoding="utf8", errors="replace") as f:
        for line in f:
            w = line.strip()
            if not w or " " in w: continue
            ws.add(w.lower()); n += 1
            if limit and n >= limit: break
    return len(ws)


def detect(s, cands):
    """Which declared language does this word belong to?

    Pedro's rule: the branch set is the detector. Look the word up in each
    declared language and the hits name the language. First declared wins a
    tie, which is what makes declaration order a priority order.

    Returns None when no lexicon claims it -- then the first candidate reads it,
    which is the old behaviour and the right fallback: a filter should not fail
    on a foreign word, it should read it with the rules it has.
    """
    if not LEXICON: return None
    low = s.lower()
    for tag in cands:
        if tag is None: continue
        for t in (tag, tag.split("-")[0]):
            if low in LEXICON.get(t, ()): return tag
    return None


def primary(s, lang=None):
    """The one reading a string sorts at: first candidate, first branch each.

    This does NOT build the branch product. key() only ever wanted the product's
    first element, and at a hundred thousand words the difference is the whole
    run -- a word with eight branchy graphemes has thousands of readings and
    every one of them was being built to throw away.
    """
    segs = segment(s)
    cands = P.candidates(lang)
    accept = P.accept_for(detect(s, cands) or cands[0])
    out = []
    for i, seg in enumerate(segs):
        if seg == SEP: out.append((SEP, MID)); continue
        b = branches(seg, i, segs, s)
        if accept:
            keep = [x for x in b if P.exact_match(x[2], accept)]
            b = keep or b                              # never erase a segment
        out.append((b[0][0], b[0][1]) if b else (SEP, MID))
    return tuple(out)


def key(s, lang=None):
    """One key per string: the PRIMARY reading the language admits.

    Not the minimum. Taking the minimum over branches read "isit" as "ishit",
    because s has a /sh/ branch and sh sorts before si -- a real position for
    the string, a nonsense canonical for it.
    """
    return _k(s, primary(s, lang))


def positions(s, lang=None):
    """Every position the string occupies — the multi-listing, sorted."""
    return sorted((_k(s, r) for r in readings(s, lang)), key=lambda x: x[0])

def chroma_sorted(items, lang=None):
    return sorted(items, key=lambda s: key(s, lang)[0])

if __name__ == "__main__":
    a = sys.argv[1:]
    lang = None; showall = False
    while a and a[0].startswith("--"):
        if a[0] == "--lang": lang = a[1]; a = a[2:]
        elif a[0] == "--all": showall = True; a = a[1:]
        else: break
    items = [l.rstrip("\n") for l in sys.stdin] if a == ["-"] else a
    if not items: print(__doc__); sys.exit(0)
    w = min(46, max(len(i) for i in items))
    print(f"  lang: {lang or 'none declared — primary branch of each grapheme'}\n")
    print(f"  {'name'.ljust(w)}   reads as{'':<23}sounds")
    for s in chroma_sorted(items, lang):
        k, spell, r = key(s, lang)
        ipa = "".join(x[1] for x in r if x[1] and x[1] != "·")
        print(f"  {s[:w].ljust(w)}   {spell.replace(SEP,'/')[:30]:<30} /{ipa[:34]}/")
    if showall:
        print(f"\n  every position each name occupies")
        for s in chroma_sorted(items, lang):
            ps = positions(s, lang)
            print(f"  {s[:w].ljust(w)}   {len(ps):>3} " + ", ".join(
                p[1].replace(SEP, "/") for p in ps[:8])
                + (" …" if len(ps) > 8 else ""))
