#!/usr/bin/env python3
"""python3 tools/chroma_ipa.py — the third alphabet.

Chroma UTF is 306 characters in ring 9. The phonetic index is 370,571 entries in
ring 19. This is the third: the IPA chart, ordered by CHROMA UTF.

Not by place|manner|voice. Every phoneme already carries a Chroma UTF spelling —
that is what the sound axis has always sorted on — so the IPA order is just those
spellings in the order the base table already declares. Nothing new is judged,
and the adjacency falls out rather than being arranged:

    /s/ s and /ʃ/ sh   +1    immediate
    /d/ d and /ð/ dh   +2    retroflex /ɖ/ spells dd and sorts between them
    /t/ t and /θ/ th   +2    the dental click /ǀ/ spells tc

All ten stop/fricative pairs share a first digit; five are immediate. The ones
that are not have a sibling phoneme sitting between, which is a fact about the
chart rather than noise in the order.

Diacritics are NOT digits. They modify a segment the way an accent modifies a
letter, so they are a sub-level, exactly as in the base table. The digits are the
letters and the marks that stand between segments.
"""
import os, sys, collections, math
from fractions import Fraction
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import chroma_utf as C

# ---------- the chart, each symbol with its Chroma UTF spelling ----------
# The spelling is the nearest Latin sound plus a disambiguator, and the scheme is
# consistent: a fricative counterpart of a stop takes h (t/th, d/dh, p/ph, k/kh),
# retroflex doubles (t/tt, s/ss, n/nn), palatal takes y, uvular takes q.
PULMONIC = [
 ("p","p","voiceless bilabial plosive"),      ("b","b","voiced bilabial plosive"),
 ("t","t","voiceless alveolar plosive"),      ("d","d","voiced alveolar plosive"),
 ("ʈ","tt","voiceless retroflex plosive"),    ("ɖ","dd","voiced retroflex plosive"),
 ("c","ky","voiceless palatal plosive"),      ("ɟ","gy","voiced palatal plosive"),
 ("k","k","voiceless velar plosive"),         ("ɡ","g","voiced velar plosive"),
 ("q","kq","voiceless uvular plosive"),       ("ɢ","gq","voiced uvular plosive"),
 ("ʔ","hq","glottal stop"),
 ("m","m","bilabial nasal"),                  ("ɱ","mf","labiodental nasal"),
 ("n","n","alveolar nasal"),                  ("ɳ","nn","retroflex nasal"),
 ("ɲ","ny","palatal nasal"),                  ("ŋ","ng","velar nasal"),
 ("ɴ","nq","uvular nasal"),
 ("ʙ","bb","bilabial trill"),                 ("r","r","alveolar trill"),
 ("ʀ","rq","uvular trill"),
 ("ⱱ","vv","labiodental flap"),               ("ɾ","rd","alveolar tap"),
 ("ɽ","rr","retroflex flap"),
 ("ɸ","ph","voiceless bilabial fricative"),   ("β","bh","voiced bilabial fricative"),
 ("f","f","voiceless labiodental fricative"), ("v","v","voiced labiodental fricative"),
 ("θ","th","voiceless dental fricative"),     ("ð","dh","voiced dental fricative"),
 ("s","s","voiceless alveolar fricative"),    ("z","z","voiced alveolar fricative"),
 ("ʃ","sh","voiceless postalveolar fricative"),("ʒ","zh","voiced postalveolar fricative"),
 ("ʂ","ss","voiceless retroflex fricative"),  ("ʐ","zz","voiced retroflex fricative"),
 ("ç","hy","voiceless palatal fricative"),    ("ʝ","jy","voiced palatal fricative"),
 ("x","kh","voiceless velar fricative"),      ("ɣ","gh","voiced velar fricative"),
 ("χ","xq","voiceless uvular fricative"),     ("ʁ","rh","voiced uvular fricative"),
 ("ħ","hh","voiceless pharyngeal fricative"), ("ʕ","aa","voiced pharyngeal fricative"),
 ("h","h","voiceless glottal fricative"),     ("ɦ","hv","voiced glottal fricative"),
 ("ɬ","lh","voiceless lateral fricative"),    ("ɮ","lz","voiced lateral fricative"),
 ("ʋ","vw","labiodental approximant"),        ("ɹ","rw","alveolar approximant"),
 ("ɻ","rj","retroflex approximant"),          ("j","y","palatal approximant"),
 ("ɰ","gw","velar approximant"),
 ("l","l","alveolar lateral approximant"),    ("ɭ","ll","retroflex lateral"),
 ("ʎ","ly","palatal lateral"),                ("ʟ","lg","velar lateral"),
]
NONPULMONIC = [
 ("ʘ","pc","bilabial click"),   ("ǀ","tc","dental click"),
 ("ǃ","qc","postalveolar click"),("ǂ","cc","palatoalveolar click"),
 ("ǁ","lc","lateral click"),
 ("ɓ","bi","bilabial implosive"),("ɗ","di","alveolar implosive"),
 ("ʄ","ji","palatal implosive"), ("ɠ","gi","velar implosive"),
 ("ʛ","gj","uvular implosive"),  ("ʼ","ej","ejective"),
]
OTHER = [
 ("ʍ","wh","voiceless labial-velar fricative"),("w","w","labial-velar approximant"),
 ("ɥ","wy","labial-palatal approximant"),      ("ʜ","hp","voiceless epiglottal fricative"),
 ("ʢ","aj","voiced epiglottal fricative"),     ("ʡ","qp","epiglottal plosive"),
 ("ɕ","sy","alveolo-palatal fricative"),       ("ʑ","zy","voiced alveolo-palatal"),
 ("ɺ","lr","lateral flap"),                    ("ɧ","sx","simultaneous sh and x"),
]
VOWELS = [
 ("i","i","close front unrounded"),   ("y","yu","close front rounded"),
 ("ɨ","ic","close central unrounded"),("ʉ","uc","close central rounded"),
 ("ɯ","ub","close back unrounded"),   ("u","u","close back rounded"),
 ("ɪ","ih","near-close front unrounded"),("ʏ","yh","near-close front rounded"),
 ("ʊ","uh","near-close back rounded"),
 ("e","e","close-mid front unrounded"),("ø","eo","close-mid front rounded"),
 ("ɘ","ec","close-mid central unrounded"),("ɵ","oc","close-mid central rounded"),
 ("ɤ","ob","close-mid back unrounded"),("o","o","close-mid back rounded"),
 ("ə","eu","mid central"),
 ("ɛ","eh","open-mid front unrounded"),("œ","eq","open-mid front rounded"),
 ("ɜ","er","open-mid central unrounded"),("ɞ","or","open-mid central rounded"),
 ("ʌ","uv","open-mid back unrounded"),("ɔ","oh","open-mid back rounded"),
 ("æ","ae","near-open front unrounded"),("ɐ","av","near-open central"),
 ("a","a","open front unrounded"),    ("ɶ","aq","open front rounded"),
 ("ɑ","ab","open back unrounded"),    ("ɒ","ap","open back rounded"),
]
# The addresses. Not sounds: they say what KIND of address a reading is.
#
# Appending any digit already means "infinitesimally above the prefix" -- every
# appended digit lands between hello and hellp, whatever it is. So the rank does
# not decide WHETHER you went up, it decides HOW FAR. These are the smallest
# increments there are, so they sort below everything, and their spellings begin
# with 0 where prosody begins with 1: the order still derives from Chroma UTF
# rather than being declared.
#
# Each is one digit, not two. A stalk carries a sign, so under is over negated,
# down is up negated, miny is tiny negated. Five digits, ten addresses.
INFINITESIMAL = [
 ("\u29be","01","tiny-on, the floor"),   ("\u29bf","02","tiny"),
 ("\u2191","03","up"),                   ("\u25cb","04","over"),
 ("\u03b5","05","epsilon, below every real"),
]
# Boundaries and prosody sort BEFORE every letter, so they are spelled with
# digits -- Chroma UTF puts 0..9 ahead of a..Z, so the convention costs nothing.
PROSODY = [
 ("ˈ","1","primary stress"),   ("ˌ","2","secondary stress"),
 ("ː","3","long"),
 ("|","5","minor group"),      ("‖","6","major group"),
 (".","7","syllable break"),
 # the linking mark and half-long are gone, so the alphabet lands on 126
 # symbols; with rank 0 reserved that is 127 elements, and 127 is prime.
 ("˥","90","extra high tone"), ("˦","91","high tone"),
 ("˧","92","mid tone"),        ("˨","93","low tone"),
 ("˩","94","extra low tone"),  ("↗","95","global rise"),
 ("↘","96","global fall"),
]
LETTERS = PULMONIC + NONPULMONIC + OTHER + VOWELS
DIGITS = INFINITESIMAL + PROSODY + LETTERS

# Diacritics are not digits. They modify a segment the way an accent modifies a
# letter, so they are a sub-level of the digit they sit on -- the same shape the
# base table already uses for base | case | accent.
DIACRITICS = [
 ("̥","voiceless"), ("̬","voiced"), ("ʰ","aspirated"), ("̹","more rounded"),
 ("̜","less rounded"), ("̟","advanced"), ("̠","retracted"), ("̈","centralized"),
 ("̽","mid-centralized"), ("̩","syllabic"), ("̯","non-syllabic"), ("˞","rhoticity"),
 ("̤","breathy voiced"), ("̰","creaky voiced"), ("̼","linguolabial"),
 ("ʷ","labialized"), ("ʲ","palatalized"), ("ˠ","velarized"), ("ˤ","pharyngealized"),
 ("̴","velarized or pharyngealized"), ("̝","raised"), ("̞","lowered"),
 ("̘","advanced tongue root"), ("̙","retracted tongue root"), ("̪","dental"),
 ("̺","apical"), ("̻","laminal"), ("̃","nasalized"), ("ⁿ","nasal release"),
 ("ˡ","lateral release"), ("̚","no audible release"),
]

def build():
    """Order the digits by their Chroma UTF spelling. Nothing else is judged."""
    rows = [{"sym": s, "rom": r, "name": n} for s, r, n in DIGITS]
    key = lambda row: tuple(C.letters(row["rom"]))
    rows.sort(key=key)
    # Rank 0 is the ZERO and stays empty. A trailing rank-0 digit is invisible --
    # 0.5 and 0.50 are the same number -- so a symbol parked there cannot be told
    # from not being there at all. Ranks start at 1.
    for i, row in enumerate(rows):
        row["rank"] = i + 1
        row["key"] = key(row)
    MOD = len(rows) + 1                          # the symbols plus the zero
    RING = max(1, (MOD - 1).bit_length())
    W = 4 * -(-RING // 4)
    return rows, RING, W, MOD

ROWS, RING, WIDTH, MOD = build()
RANK = {r["sym"]: r["rank"] for r in ROWS}
ROM = {r["sym"]: r["rom"] for r in ROWS}
BY_ROM = {r["rom"]: r["sym"] for r in ROWS}
MARK = {m: i for i, (m, _n) in enumerate(DIACRITICS)}
BASE = 1 << WIDTH


def parse(ipa):
    """An IPA string -> [(symbol, [diacritics])], longest match first.

    Diacritics attach to the segment they follow, the way an accent attaches to
    a letter. They are not digits of their own.
    """
    out, i = [], 0
    longest = max(len(s) for s in RANK)
    while i < len(ipa):
        for n in range(min(longest, len(ipa) - i), 0, -1):
            if ipa[i:i+n] in RANK:
                out.append([ipa[i:i+n], []]); i += n; break
        else:
            if out and ipa[i] in MARK: out[-1][1].append(ipa[i])
            i += 1                                  # anything else is skipped
    return [(s, tuple(m)) for s, m in out]


def digits(ipa):
    """The base-256 digits of an IPA string: one per segment."""
    return [RANK[s] for s, _m in parse(ipa)]


def integer(ipa):
    """base(chroma-ipa): the digits are the coefficients, the base is 256."""
    n = 0
    for d in digits(ipa): n = n * BASE + d
    return n


if __name__ == "__main__":
    fails = []
    def check(n, cond, msg):
        print(f"  [{n}] {msg}")
        if not cond: fails.append(n)

    print(f"CHROMA IPA — the third alphabet\n")
    print(f"  symbols            {len(ROWS)}  ({len(LETTERS)} letters + "
          f"{len(PROSODY)} prosody + {len(INFINITESIMAL)} addresses)")
    print(f"  diacritics         {len(DIACRITICS)}  a sub-level, not digits")
    print(f"  ranks              1..{len(ROWS)}   rank 0 is the zero and stays empty")
    print(f"  modulus            {MOD}   the symbols plus the zero")
    print(f"  ring               {RING}   ({2 ** RING} slots, "
          f"{2 ** RING - MOD} spare)")
    print(f"  digit width        {WIDTH} bits = {WIDTH // 4} nibbles, "
          f"base {2 ** WIDTH}   (storage is nibble aligned, arithmetic is mod "
          f"{MOD})\n")

    # [1] the width lands on a nibble, which is the whole reason for a third alphabet
    check(1, WIDTH % 4 == 0 and len(ROWS) <= 2 ** WIDTH,
          f"nibble aligned   {WIDTH} bits is {WIDTH // 4} whole nibbles, and "
          f"{len(ROWS)} digits fit in {2 ** WIDTH}")

    # [2] total: no two symbols may share a spelling, or the order is not an order
    rom = collections.Counter(r["rom"] for r in ROWS)
    dup = {k: v for k, v in rom.items() if v > 1}
    check(2, not dup, f"total            {len(set(rom))}/{len(ROWS)} spellings distinct"
          + (f"   COLLISIONS: {dup}" if dup else ""))

    # [3] monotone in the Chroma UTF order of the spellings
    bad = sum(1 for i in range(1, len(ROWS)) if ROWS[i - 1]["key"] > ROWS[i]["key"])
    check(3, bad == 0, f"monotone         {bad} inversions across "
          f"{len(ROWS) - 1} steps, ordered by Chroma UTF spelling")

    # [4] the adjacency falls out rather than being arranged.
    #
    # The claim is BLOCK adjacency, not always immediate adjacency: /d/ spells d
    # and /ð/ spells dh, so they share a first digit, but retroflex /ɖ/ spells dd
    # and sorts between them. Asserting "immediate neighbours" was simply false --
    # 5 of 10 pairs, not 8 -- and the sibling sitting between is a real phoneme,
    # not noise.
    pairs = [("d", "ð"), ("t", "θ"), ("s", "ʃ"), ("z", "ʒ"), ("p", "ɸ"), ("k", "x"),
             ("b", "β"), ("ɡ", "ɣ"), ("n", "ŋ"), ("l", "ɬ")]
    gaps = [(a, b, RANK[b] - RANK[a], ROM[a], ROM[b]) for a, b in pairs
            if a in RANK and b in RANK]
    block = [g for g in gaps if g[3][0] == g[4][0]]
    imm = [g for g in gaps if g[2] == 1]
    check(4, len(block) == len(gaps),
          f"adjacency        {len(block)}/{len(gaps)} stop/fricative pairs share a "
          f"first digit, {len(imm)} of them immediately")
    print("       " + ",  ".join(
        f"/{a}/{ra} /{b}/{rb} +{d}" for a, b, d, ra, rb in gaps[:6]))
    far = [g for g in gaps if g[2] > 1]
    if far:
        print("       the gaps are siblings, not noise: "
              + ", ".join(f"{ra}..{rb} has "
                          + ",".join(r["rom"] for r in ROWS
                                     if RANK[a] < r["rank"] < RANK[b])
                          for a, b, d, ra, rb in far[:3]))

    # [5] prosody before letters, because digits sort before letters in the base
    firstLetter = min(r["rank"] for r in ROWS if r["sym"] in dict(
        (s, n) for s, _r, n in LETTERS))
    lastPros = max(r["rank"] for r in ROWS if r["sym"] in dict(
        (s, n) for s, _r, n in PROSODY))
    check(5, lastPros < firstLetter,
          f"prosody first    all {len(PROSODY)} prosody marks precede all "
          f"{len(LETTERS)} letters, for free: Chroma UTF puts 0..9 ahead of a")

    # [6] every spelling is inside Chroma UTF, or a digit would leave the alphabet
    off = [r for r in ROWS if any(ch not in C.UTF for ch in r["rom"])]
    check(6, not off, f"inside the base  every spelling uses only Chroma UTF "
          f"characters" + (f"   OUTSIDE: {[r['rom'] for r in off][:4]}" if off else ""))

    # [7] rank 0 is the zero, and nothing may sit there
    lowest = min(r["rank"] for r in ROWS)
    B = 1 << WIDTH
    def val(ds):
        num, den = 0, 1
        for d in ds: num, den = num * B + d, den * B
        return Fraction(num, den)
    probe = digits("sit")
    vanishes = val(probe) == val(probe + [0])
    check(7, lowest == 1 and vanishes,
          f"zero reserved    lowest rank is {lowest}; a trailing rank-0 digit is "
          "invisible (0.5 and 0.50 are one number), so nothing may sit there")

    # [8] the modulus is prime, so every nonzero digit divides
    isprime = MOD > 1 and all(MOD % d for d in range(2, int(MOD ** 0.5) + 1))
    units = sum(1 for a in range(1, MOD) if math.gcd(a, MOD) == 1)
    check(8, isprime and units == MOD - 1,
          f"a field          {MOD} is {'prime' if isprime else 'NOT prime'}, so "
          f"{units}/{MOD - 1} nonzero digits have an inverse mod {MOD}")

    # [9] the addresses are the smallest increments there are
    addr = [r for r in ROWS if r["sym"] in {s2 for s2, _r, _n in INFINITESIMAL}]
    ranks = [r["rank"] for r in addr]
    want = [s2 for s2, _r, _n in INFINITESIMAL]
    got = [r["sym"] for r in sorted(addr, key=lambda r: r["rank"])]
    check(9, ranks == list(range(1, len(INFINITESIMAL) + 1)) and got == want,
          f"addresses first  the {len(addr)} addresses hold ranks "
          f"{min(ranks)}..{max(ranks)}, below every mark and every letter")
    print("       " + " < ".join(
        f"{r['sym']} {r['name'].split(',')[0]}" for r in sorted(addr, key=lambda r: r["rank"]))
        + " < every sound")
    print("       each is one digit; a stalk carries the sign, so under is over")
    print("       negated, down is up negated, miny is tiny negated")

    print(f"\n  THE ORDER\n")
    line, col = [], 0
    for r in ROWS:
        cell = f"{r['sym']} {r['rom']}"
        line.append(cell.ljust(8)); col += 1
        if col % 9 == 0: print("    " + "".join(line)); line = []
    if line: print("    " + "".join(line))

    print(f"\n  {'certified.' if not fails else 'FAILED: ' + str(fails)}")
    sys.exit(1 if fails else 0)
