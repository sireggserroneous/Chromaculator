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
DIGRAPH = {
 "sh": [("ʃ","sh","en",""),          ("sx","skh","nl","")],
 "ch": [("tʃ","ch","en es",""),      ("k","k","it de-el gr",""),
        ("ʃ","sh","fr pt",""),       ("x","kh","de","ach-laut")],
 "th": [("θ","th","en",""),          ("ð","dh","en","this, the"),
        ("t","t","fr de it","")],
 "ph": [("f","f","en fr de","")],
 "wh": [("w","w","en",""),           ("h","h","en","who")],
 "ng": [("ŋ","ng","en de",""),       ("nɡ","ng","it","")],
 "qu": [("kw","kw","en it",""),      ("k","k","fr es","")],
 "ck": [("k","k","en de","")],
 "gh": [("ɡ","g","en it",""),        ("f","f","en","enough"),
        ("","","en","night — silent")],
 "kn": [("n","n","en","knee"),       ("kn","kn","de","")],
 "wr": [("ɹ","r","en","write")],
 "zh": [("ʒ","zh","en","")],
 "dh": [("ð","dh","en ga","")],
 "ll": [("j","y","es",""),           ("l","l","en fr it","")],
 "rr": [("r","r","es it","")],
 "bh": [("v","v","ga","abhan reads Avon")],
 "ts": [("ts","ts","de ja","")],
 "ss": [("s","s","en de fr","")],
}
MAXG = max(len(g) for g in DIGRAPH)
SEP = "\x01"                       # sorts below every letter, spells nothing

def branches(seg):
    """-> [(chroma-utf spelling, ipa, [langs])] for one grapheme."""
    l = seg.lower()
    if l in DIGRAPH:
        up = seg[0] != l[0]
        return [((rom.upper() if up and rom else rom), ipa, langs.split())
                for ipa, rom, langs, _ in DIGRAPH[l]]
    e = P.latin_entries(seg)
    if e: return [(r, ipa, langs) for r, ipa, langs, _ in e]
    r, tone, st, layer, _ = C.reading(seg)
    if r is None: return [(SEP, "", ["und"])]
    if layer in ("han", "han-unread"):                 # every Unihan reading
        out = []
        for pr, lang, ipa, ch, tone, st in P.entries([ord(seg)]):
            out.append((pr, ipa, [lang]))
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

def readings(s, lang=None):
    """Every reading the declared language admits, as (spelling, ipa)."""
    want = set()
    if lang:
        for x in lang.split():
            want.add(x)
            if "-" in x: want.add(x.split("-")[0])
    per = []
    for seg in segment(s):
        if seg == SEP: per.append([(SEP, "·")]); continue
        b = branches(seg)
        if want:
            keep = [x for x in b if set(x[2]) & want or "und" in x[2]]
            b = keep or b                              # never erase a segment
        seen, uniq = set(), []
        for rom, ipa, _ in b:
            if rom not in seen: seen.add(rom); uniq.append((rom, ipa))
        per.append(uniq or [(SEP, "·")])
    n = 1
    for p in per: n *= len(p)
    if n > 4096:                                       # ponytail: cap the product
        return [tuple(p[0] for p in per)]              # greedy, first branch each
    return [tuple(c) for c in itertools.product(*per)]

def _k(s, r):
    spelling = "".join(x[0] for x in r)
    return (tuple(C.letters(spelling)) + (C.TERM,)
            + tuple(C.letters(u.normalize("NFD", s))),   # tie break: the spelling
            spelling, r)

def key(s, lang=None):
    """One key per string: the PRIMARY reading the language admits.

    Not the minimum. Taking the minimum over branches read "isit" as "ishit",
    because s has a /ʃ/ branch in German and sh sorts before si — a real
    position for the string, but a nonsense canonical for it. The primary
    branch is the first one listed, which is the most common reading.
    """
    per = readings(s, lang)
    return _k(s, per[0])

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
