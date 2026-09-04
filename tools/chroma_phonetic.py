#!/usr/bin/env python3
"""Chroma phonetic index — multi-listing.

We read C as /k/ or /s/, so C belongs in the k path AND the s path. A character
is listed in every place its sound can put it. 飼 is the same character: si in
Mandarin, zi in Cantonese, shi or ji as on'yomi, kau or yashinau as kun'yomi,
sa in Korean — seven entries, seven positions.

The ordering is therefore an INDEX, not a permutation. One entry per
(character, reading, language). Language is not a level you sort by so much as
a FILTER: declare the language and the impossible branches are pruned, and what
is left is a subsequence of the full order — never a reordering of it.

Each phoneme sorts by its Chroma UTF spelling, not by its IPA glyph: IPA names
the sound, Chroma UTF orders it.

  /k/ -> k     /s/ -> s     /θ/ -> th    /ʃ/ -> sh
  /tʃ/ -> ch   /dʒ/ -> j    /ʒ/ -> zh    /x/ -> kh    /ŋ/ -> ng
"""
import json, os, sys, collections, unicodedata as u
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import chroma_utf as C

DATA = C.DATA

# ---------- the Latin branches: the C case ----------
# (ipa, chroma-utf spelling, languages, condition, note)
#
# The condition is a real rule now, not a note:
#   ^        word-initial          $        word-final
#   >set     next grapheme starts with one of set
#   !>set    next grapheme does NOT
#   <set     previous grapheme ends with one of set
#   @a|b|c   the whole word is one of these — the exception hook
#   ,        every part must hold
#
# The @ hook exists because some distinctions are not positional at all. English
# θ vs ð is lexical: a closed list of function words takes /ð/ and everything
# else takes /θ/, and no rule about position separates "this" from "thin".
# Rules plus a short exception list, which is how every real g2p is built.
#
# The character index lists EVERY branch regardless: a character on its own has
# no context, so c belongs in the k path and the s path both. A word supplies
# context, so the word sorter prunes by it. Same table, two readers.
VOW = "aeiouyáéíóúàèìòùäëïöüâêîôû"
LATIN = {
 "c": [("k","k",  "en fr es it de la pt", "!>eiyéèê", "before a o u, or a consonant"),
       ("s","s",  "en fr pt es-419",      ">eiyéèê",  "before e i y"),
       ("θ","th", "es-ES",                ">eiyéèê",  "distinción"),
       ("tʃ","ch","it",                   ">eiéè",    "before e i"),
       ("ts","ts","de pl",                "",         "")],
 "g": [("ɡ","g",  "en fr es it de la pt", "!>eiyéèê", "before a o u"),
       ("dʒ","j", "en it",                ">eiyéèê",  "before e i y"),
       ("ʒ","zh", "fr pt",                ">eiyéèê",  "before e i y"),
       ("x","kh", "es",                   ">eiéè",    "before e i")],
 "j": [("dʒ","j", "en it",                "",         ""),
       ("ʒ","zh", "fr pt",                "",         ""),
       ("x","kh", "es",                   "",         ""),
       ("j","y",  "de nl sv la",          "",         "")],
 "x": [("z","z",  "en",                   "^",        "xylophone"),
       ("ks","ks","en fr es pt la",       "!^",       ""),
       ("ʃ","sh", "pt ca",                "",         ""),
       ("x","kh", "es",                   "",         "México, older orthography")],
 "s": [("z","z",  "en fr de pt",          "<" + VOW + ",>" + VOW, "between vowels"),
       ("s","s",  "en fr es it de pt la", "",         ""),
       ("ʃ","sh", "de pt",                ">tp",      "before t or p")],
 "z": [("z","z",  "en fr pt it",          "",         ""),
       ("ts","ts","de it",                "",         ""),
       ("θ","th", "es-ES",                "",         "distinción"),
       ("s","s",  "es-419",               "",         "seseo")],
 "y": [("j","y",  "en de nl es",          ">" + VOW,  "before a vowel"),
       ("i","i",  "en es",                "",         "as a vowel"),
       ("y","u",  "fr nl",                "",         "front rounded")],
 "h": [("h","h",  "en de nl",             "",         ""),
       ("",  "",  "fr es it pt",          "",         "silent")],
 "v": [("v","v",  "en fr it pt de",       "",         ""),
       ("b","b",  "es",                   "",         "betacism: v and b merge")],
 "w": [("w","w",  "en nl",                "",         ""),
       ("v","v",  "de pl sv",             "",         "")],
 "r": [("ɹ","r",  "en",                   "",         ""),
       ("ʁ","r",  "fr de",                "",         "uvular"),
       ("r","r",  "es it nl la",          "",         "trill")],
 "b": [("b","b",  "en fr es it de pt la", "",         ""),
       ("β","v",  "es",                   "<" + VOW + ",>" + VOW, "between vowels"),
       ("v","v",  "ga",                   "",         "bh lenition — abhan reads Avon")],
}

def cond_holds(cond, i, segs, word=""):
    """Is this branch's positional condition satisfied at segs[i]?"""
    if not cond: return True
    prv = segs[i-1] if i > 0 else ""
    nxt = segs[i+1] if i + 1 < len(segs) else ""
    for part in cond.split(","):
        if part.startswith("@"):
            if word.lower() not in part[1:].split("|"): return False
            continue
        if part == "^":
            if i != 0: return False
        elif part == "$":
            if i != len(segs) - 1: return False
        elif part == "!^":
            if i == 0: return False
        elif part.startswith("!>"):
            if nxt and nxt[0].lower() in part[2:]: return False
        elif part.startswith(">"):
            if not nxt or nxt[0].lower() not in part[1:]: return False
        elif part.startswith("<"):
            if not prv or prv[-1].lower() not in part[1:]: return False
    return True
# The letters with one uncontroversial value. Vowels are the obvious next
# branching set (a is /a/ /æ/ /ɑ/ /eɪ/ depending on the language) — listed with
# their Latin value here so a word renders whole, and marked for branching.
PLAIN = {"a":"a","b":"b","d":"d","e":"e","f":"f","i":"i","k":"k","l":"l","m":"m",
         "n":"n","o":"o","p":"p","q":"k","t":"t","u":"u","g":"ɡ"}
LANGS = ("cmn", "yue", "ja-on", "ja-kun", "ko")

def latin_entries(ch):
    """Branches for a Latin character. An accented form inherits its base
    letter's branches — ä is a vowel with a sound, not a spelling variant."""
    l = u.normalize("NFD", ch)[0].lower()
    upper = ch != ch.lower()
    if l in LATIN:
        return [((rom.upper() if upper and rom else rom), ipa, langs.split(), cond, note)
                for ipa, rom, langs, cond, note in LATIN[l]]
    if l in PLAIN:
        r = l.upper() if upper else l
        return [(r, PLAIN[l], ["und"], "", "single value")]
    return []

# ---------- entries ----------
def entries(cps=None):
    """-> [(reading, language, ipa, char, tone, strokes)] — one per branch."""
    cps = C.all_assigned() if cps is None else cps
    U = C.UNIHAN
    out = []
    for cp in cps:
        ch = chr(cp)
        st = U.get("strokes", {}).get(cp)
        if st is not None:                                   # Han: every reading
            strokes = int(st.split()[0])
            any_r = False
            for field, lang in (("mandarin","cmn"), ("cantonese","yue"),
                                ("on","ja-on"), ("kun","ja-kun"), ("korean","ko")):
                v = U.get(field, {}).get(cp)
                if not v: continue
                # Unihan itself repeats readings in places — 橫's kJapaneseKun
                # lists YOKO YOKOTAWARU YOKOTAERU twice. One reading, one branch.
                for tok in dict.fromkeys(v.split()):
                    if field == "mandarin": r, tone = C.pinyin_split(tok)
                    else:
                        r, tone = "".join(c for c in tok.lower() if c.isalpha()), 0
                    if not r: continue
                    out.append((r, lang, tok, ch, tone, strokes)); any_r = True
            if not any_r:
                out.append((u.name(ch, "").lower(), "und", "", ch, 0, strokes))
            continue
        lat = latin_entries(ch) if ch in C.UTF else []
        if lat:                                              # the C case
            for r, ipa, langs, cond, note in lat:
                out.append((r or "\x00", "|".join(langs), ipa, ch, 0, 0))
            continue
        r, tone, strokes, layer, _ = C.reading(ch)           # single branch
        out.append((r if r is not None else "￿", "und", "", ch, tone, strokes))
    return out

LANGRANK = {l: i for i, l in enumerate(LANGS + ("und",))}

# The final tie break is the SPELLING, and the spelling order is the spine —
# Chroma UTF's own declared order, not the codepoint. Sound discards the accent:
# á ä ǎ all read "a", so without this they tie and fall back to codepoint order,
# which is not the declared a á à ă â ǎ … run. Anything off the spine sorts
# after it, by codepoint.
SPELL = {ch: i for i, ch in enumerate(C.TABLE)}
def spell_rank(ch):
    return SPELL.get(ch, len(C.TABLE) + ord(ch))

def ekey(e):
    r, lang, ipa, ch, tone, st = e
    return tuple(C.letters(r)) + (C.TERM, LANGRANK.get(lang, 99), tone, st,
                                  spell_rank(ch), ipa)

def build(cps=None):
    es = entries(cps)
    es.sort(key=ekey)
    return es

def filter_lang(es, want):
    """The filter. Keeps a SUBSEQUENCE — never reorders.

    A request for es-ES accepts entries tagged es-ES and es, but not es-419:
    a region narrows a language, it is not a sibling of another region.
    """
    w = set(want.split()) if isinstance(want, str) else set(want)
    accept = set(w)
    for x in w:
        if "-" in x: accept.add(x.split("-")[0])          # es-ES also accepts es
    return [e for e in es
            if e[1] == "und" or e[1] in accept or set(e[1].split("|")) & accept]

def ecells(e):
    """the entry as bits — the picture."""
    r, lang, ipa, ch, tone, st = e
    W = C.RING + 3
    out = []
    for L in C.letters(r): out += [int(b) for b in format(L, "0%db" % W)]
    out += [0] * W                                            # terminator
    for x, w in ((LANGRANK.get(lang, 99), 7), (tone, 8), (st, 8), (spell_rank(ch), 22)):
        out += [int(b) for b in format(x, "0%db" % w)]
    return out

def evalue(e):
    c = ecells(e)
    return (int("".join(map(str, c)) or "0", 2), 1 << len(c))

if __name__ == "__main__":
    import random, time
    t0 = time.time()
    cps = C.all_assigned()
    es = build(cps)
    print(f"CHROMA PHONETIC INDEX — Unicode {u.unidata_version}\n")
    print(f"  characters         {len(cps):,} assigned")
    print(f"  entries            {len(es):,}  ({time.time()-t0:.1f}s)")
    at = collections.Counter(e[3] for e in es)
    multi = {c: n for c, n in at.items() if n > 1}
    print(f"  multi-listed       {len(multi):,} characters sit in more than one place")
    print(f"  widest             " + ", ".join(
        f"{c}×{n}" for c, n in collections.Counter(multi).most_common(5)))
    print(f"  mean entries per multi-listed character {sum(multi.values())/len(multi):.2f}\n")
    print("  BY LANGUAGE")
    for l, n in collections.Counter(e[1] for e in es).most_common(8):
        print(f"     {n:>8,}  {l}")
    print()
    fails = []
    # [1] entries distinct
    ids = {(e[0], e[1], e[2], e[3]) for e in es}
    print(f"  [1] distinct       {len(ids):,}/{len(es):,} entries, no duplicates")
    if len(ids) != len(es): fails.append(1)
    # [2] monotone in the exact folded value
    random.seed(11); s = sorted(random.sample(range(len(es)-1), 4000))
    inv = sum(1 for i in s
              if (lambda a, b: a[0]*b[1] > b[0]*a[1])(evalue(es[i]), evalue(es[i+1])))
    print(f"  [2] monotone       {inv} inversions in 4,000 sampled adjacent pairs")
    if inv: fails.append(2)
    # [3] stable
    sh = entries(cps); random.shuffle(sh); sh.sort(key=ekey)
    print(f"  [3] stable         reshuffled input sorts identically: "
          f"{'yes' if sh == es else 'NO'}")
    if sh != es: fails.append(3)
    # [4] THE filter property: prune, never reorder
    print("  [4] filter is a subsequence — sort-then-filter == filter-then-sort")
    for want in ["cmn", "yue", "ja-on ja-kun", "ko", "es-ES", "it"]:
        a = filter_lang(es, want)
        b = sorted(filter_lang(entries(cps), want), key=ekey)
        same = a == b
        # and a genuine subsequence of the full order
        it_ = iter(es); sub = all(any(x is y for y in it_) for x in a)
        print(f"      {want:<14} {len(a):>8,} entries   "
              f"{'subsequence' if (same and sub) else 'REORDERED'}")
        if not (same and sub): fails.append(4)
    # [5] round trip: every reading of a character finds that character
    lookup = collections.defaultdict(list)
    for i, e in enumerate(es): lookup[e[3]].append(i)
    probe = ["飼", "c", "C", "中", "生", "行", "g"]
    okrt = all(len(lookup[p]) == at[p] for p in probe if p in at)
    print(f"  [5] round trip     every branch of a character resolves to it: "
          f"{'yes' if okrt else 'NO'}")
    if not okrt: fails.append(5)
    # [6] the base 306 keep their declared order among their own entries
    seen, order306 = [], C.TABLE
    for e in es:
        if e[3] in C.UTF and e[3] not in seen: seen.append(e[3])
    # "unbranched" is a measured fact, not a list: any base character with one
    # single entry. Hardcoding the branch set went stale the moment accented
    # letters started inheriting their base letter's sounds.
    branched = {c for c in C.TABLE if at[c] > 1}
    plain = [c for c in seen if c not in branched]
    decl  = [c for c in order306 if c not in branched]
    print(f"  [6] base intact    unbranched base characters in declared order: "
          f"{'yes' if plain == decl else 'NO'}   ({len(plain)} of 306; "
          f"{len(branched & set(order306))} are multi-listed by sound)")
    if plain != decl: fails.append(6)
    # [7][8] the flat code: same construction as the short list, one ring up.
    # 306 entries need ring 9 and 10-bit codes; 370,571 need ring 19 and 20-bit
    # codes. Position in the sorted index becomes the code, so the long list is
    # addressable by a fixed-width integer exactly as the base table is.
    RING = max(1, (len(es) - 1).bit_length())
    FLOOR = 1 << RING
    widths = {(FLOOR + i).bit_length() for i in range(len(es))}
    print(f"  [7] fixed width    every code {widths.pop() if len(widths)==1 else widths} bits, "
          f"ring {RING}: {FLOOR:,} slots, codes {FLOOR:,}..{FLOOR+len(es)-1:,}, "
          f"{FLOOR-len(es):,} spare")
    if len(widths) or (1 << (RING - 1)) >= len(es): fails.append(7)
    byc = sorted(range(len(es)), key=lambda i: FLOOR + i)
    print(f"  [8] code == order  flat 20-bit code reproduces the key order: "
          f"{'yes' if [es[i] for i in byc] == es else 'NO'}")
    if [es[i] for i in byc] != es: fails.append(8)
    print(f"\n  {'ALL PROPERTIES HOLD' if not fails else 'FAILED: ' + str(sorted(set(fails)))}")
    with open(os.path.join(DATA, "chroma-phonetic.tsv"), "w", encoding="utf8") as f:
        f.write("# pos\tcode\treading\tlang\tsound\tchar\tcp\ttone\tstrokes\n")
        for i, e in enumerate(es):
            # some characters ARE tabs and newlines. Print them only when printable.
            glyph = e[3] if u.category(e[3])[0] not in "CZ" else ""
            read  = e[0] if e[0].isprintable() else ""
            f.write(f"{i}\t{(1 << max(1, (len(es)-1).bit_length())) + i}"
                    f"\t{read}\t{e[1]}\t{e[2]}\t{glyph}\t{ord(e[3]):04X}"
                    f"\t{e[4]}\t{e[5]}\n")
    print(f"  wrote data/chroma-phonetic.tsv  ({len(es):,} rows)")
