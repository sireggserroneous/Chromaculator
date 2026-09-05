#!/usr/bin/env python3
"""Chroma Certified Ordering over the whole character map.

Every assigned codepoint gets a READING, and sorts as if spelled in Chroma UTF.
The reading comes from Unicode's own data, not a hand table:

  1  Latin / digit      the character itself, base | case | accent from NFD
  2  Hangul syllable    the Unicode name IS the romanisation (HANGUL SYLLABLE GA)
  3  Han ideograph      Unihan kMandarin, then Cantonese, on'yomi, Korean
  4  other script       the Unicode name's letter core (GREEK SMALL LETTER ALPHA)
  5  symbol / emoji     the whole Unicode name (TOP HAT)
  6  no name            sorts last, by codepoint

The sort key, most significant first:

     letters… | 0 | tone | strokes | codepoint

Letter codes are >= 1 and the run is 0-terminated, so a reading that is a prefix
of another always sorts first. Tone, strokes and codepoint are the tie breaks,
each one a sub-level of consecutive codes — the same shape as e < é < è < ê < ë.
"""
import unicodedata as u, json, re, sys, os, collections

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")

# ---------- LAYER 1: the Chroma UTF base table ----------
def ducet():
    """{codepoint: (primary, secondary, tertiary)} from the real DUCET."""
    W = {}
    for line in open(os.path.join(DATA, "allkeys.txt"), encoding="utf8"):
        line = line.split("#")[0].split("%")[0].strip()
        if not line or line.startswith("@"): continue
        cps, keys = line.split(";", 1)
        cps = cps.split()
        if len(cps) != 1: continue                      # single characters only
        m = re.findall(r"\[[.*]([0-9A-F]{4})\.([0-9A-F]{4})\.([0-9A-F]{4})\]", keys)
        if not m: continue
        W[int(cps[0], 16)] = tuple(int(x, 16) for x in m[0])
    return W

def build_table():
    """Latin letters and digits ordered base | case | accent.

    DUCET already weighs every character as (primary, secondary, tertiary) =
    (base, accent, case). Chroma UTF is DUCET with the last two levels swapped:
    sort by (primary, tertiary, secondary), so case outranks accent and every
    lowercase form of a letter precedes every uppercase form of it.
    """
    W = ducet()
    chars = [chr(c) for c in range(0x30, 0x3A)]
    for c in range(0x41, 0x250):
        s = chr(c)
        if not u.category(s).startswith("L"): continue
        d = u.normalize("NFD", s)
        if not ("a" <= d[0] <= "z" or "A" <= d[0] <= "Z"): continue
        chars.append(s)

    def facet(ch):
        d = u.normalize("NFD", ch)
        base = W.get(ord(d[0]), (0xFFFF, 0, 0))
        marks = [W.get(ord(m), (0, 0xFFFF, 0)) for m in d[1:]]
        return (base[0],                                 # base letter
                base[2],                                 # case  (DUCET tertiary)
                tuple(m[1] for m in marks),              # accent (DUCET secondary)
                ord(ch))
    chars.sort(key=facet)
    return chars

TABLE = build_table()
RING  = max(1, (len(TABLE) - 1).bit_length())
RANK  = {ch: i + 1 for i, ch in enumerate(TABLE)}     # the ORDER, and the arithmetic

# ---------- the code layer: rank orders, code draws ----------
#
# Rank and code stop being the same number. Rank is the position in the order.
# Code is where a symbol sits in the digit space, and it decides two things rank
# cannot: how much of every square is dead, and where the character points when
# it is drawn as a phasor on the ring.
#
# Packed, all 306 codes sat in 512..817 -- a 27 degree wedge out of 360. Every
# character in the alphabet pointed the same way, so a word's phasors landed on
# top of each other and a rack was a smear rather than a shape.
#
# So each TYPE gets a share of the ring and is strided to fill its share:
#
#   the type comes from Unicode    Nd is a digit, everything else a letter
#   the block bounds are DECLARED  the same split as "DUCET orders, we tailor"
#
# Blocks land on nibble boundaries, so the leading nibble is the type -- and the
# leading nibble is the top row of the square. A body's top row says what kind
# of character it is. Blocks ALONE are barely better than packed, because inside
# a block every member shares its leading bits; the stride within the block is
# what kills them.
# 16 bits, not 12. Twelve bits pad into a 4x4 with four cells left over, and
# every one of those four lands in the OUTER region — so outer was always zero
# and fold, which is the radius a phasor rides on, could only use three cells.
# Every character came out a spike with no width. Sixteen bits is four whole
# nibbles and fills the 4x4 exactly: no padding, and all three regions live.
WIDTH  = 16                                           # four nibbles, a full 4x4
BLOCKS = [("address", 0x0000, 0x0FFF),                # separators and markers
          ("digit",   0x1000, 0x2FFF),
          ("letter",  0x4000, 0xFFFF)]


def typeof(ch):
    return "digit" if u.category(ch) == "Nd" else "letter"


def build_codes(table):
    out, plan = {}, {}
    for name, lo, hi in BLOCKS:
        members = [ch for ch in table if typeof(ch) == name]
        if not members: continue
        step = max(1, (hi - lo) // len(members))
        plan[name] = (lo, hi, step, len(members))
        for i, ch in enumerate(members): out[ch] = lo + step * i
    return out, plan


CODE, PLAN = build_codes(TABLE)
UTF = CODE                                            # what letters() writes
SPACE, HYPHEN, TERM = 1, 2, 0                            # below the table, above the terminator

# ---------- LAYER 2: the bridge ----------
TONE = {"̄": 1, "́": 2, "̌": 3, "̀": 4}   # macron acute caron grave

def load_unihan():
    p = os.path.join(DATA, "unihan.json")
    if os.path.exists(p):
        raw = json.load(open(p, encoding="utf8"))
        return {k: {int(a): b for a, b in v.items()} for k, v in raw.items()}
    return {}
UNIHAN = load_unihan()

def pinyin_split(r):
    """zhōng -> ('zhong', 1). Tone lives in the diacritic."""
    d = u.normalize("NFD", r); tone = 0; out = []
    for ch in d:
        if ch in TONE: tone = TONE[ch]
        elif u.combining(ch): pass
        else: out.append(ch)
    return "".join(out).lower(), (tone or 5)

_CORE = re.compile(r"\b(?:LETTER|SYLLABLE|SYLLABICS|VOWEL SIGN|DIGIT|SIGN|CHARACTER)\s+(.*)$")

def reading(ch):
    """-> (reading, tone, strokes, layer, source-language rank)"""
    cp = ord(ch)
    if ch in UTF:                                        # 1
        return ch, 0, 0, "table", 0
    name = u.name(ch, None)
    if name is None:                                     # 6
        return None, 0, 0, "unnamed", 0
    if name.startswith("HANGUL SYLLABLE "):              # 2
        return name[16:].lower(), 0, 0, "hangul", 0
    st = UNIHAN.get("strokes", {}).get(cp)
    if st is not None:                                   # 3
        for src, (field, conv) in enumerate((("mandarin", pinyin_split), ("cantonese", None),
                                             ("on", None), ("kun", None), ("korean", None))):
            v = UNIHAN.get(field, {}).get(cp)
            if not v: continue
            first = v.split()[0]
            r, tone = conv(first) if conv else (re.sub(r"[^a-z]", "", first.lower()), 0)
            if r: return r, tone, int(st.split()[0]), "han", src
        return name.lower(), 0, int(st.split()[0]), "han-unread", 5
    m = _CORE.search(name)                               # 4
    if m and u.category(ch).startswith("L"):
        core = m.group(1).lower()
        if "CAPITAL" in name: core = core.upper()        # Chroma UTF has both codes
        return core, 0, 0, "script", 0
    return name.lower(), 0, 0, "symbol", 0               # 5

# ---------- LAYER 3: the key ----------
def letters(r):
    out = []
    for c in r:
        if c == " ": out.append(SPACE)
        elif c == "-": out.append(HYPHEN)
        elif c in UTF: out.append(UTF[c])
        elif c.lower() in UTF: out.append(UTF[c.lower()])
        else: out.append((1 << (RING + 1)) + ord(c))     # unrepresentable: above the table
    return out

UNREAD = 1 << (RING + 2)

def key(ch):
    r, tone, st, layer, src = reading(ch)
    ls = [UNREAD] if r is None else letters(r)
    return tuple(ls) + (TERM, src, tone, st, ord(ch))

def cells(ch):
    """the key as bits — the picture. Letter codes are RING+1 wide, tails fixed."""
    r, tone, st, _, src = reading(ch)
    ls = [UNREAD] if r is None else letters(r)
    W = RING + 3
    out = []
    for L in ls: out += [int(b) for b in format(L, "0%db" % W)]
    out += [0] * W                                        # terminator
    for x, w in ((src, 4), (tone, 8), (st, 8), (ord(ch), 22)):
        out += [int(b) for b in format(x, "0%db" % w)]
    return out

def value(ch):
    """cells folded to an exact rational in (0,1): cell i weighs 2^-(i+1)."""
    c = cells(ch)
    return (int("".join(map(str, c)) or "0", 2), 1 << len(c))

# ---------- the whole charmap ----------
def all_assigned():
    return [cp for cp in range(0x110000)
            if not (0xD800 <= cp <= 0xDFFF) and u.category(chr(cp)) != "Cn"]

def build(cps=None):
    cps = all_assigned() if cps is None else cps
    rows = [(key(chr(cp)), cp) for cp in cps]
    rows.sort()
    return rows

if __name__ == "__main__":
    import random, time
    t0 = time.time()
    cps = all_assigned()
    layers = collections.Counter(); lens = []
    for cp in cps:
        r, _, _, L, _ = reading(chr(cp)); layers[L] += 1
        if r: lens.append(len(r))
    rows = build(cps)
    order = [cp for _, cp in rows]
    keys  = [k for k, _ in rows]
    print(f"CHROMA CERTIFIED ORDERING — Unicode {u.unidata_version}\n")
    print(f"  base alphabet      {len(TABLE)} characters, ring {RING}, "
          f"{WIDTH}-bit digits")
    for nm, (lo, hi, st, k) in PLAN.items():
        arc = 360 * (CODE[[c for c in TABLE if typeof(c) == nm][-1]]
                     - CODE[[c for c in TABLE if typeof(c) == nm][0]]) / (1 << WIDTH)
        print(f"     {nm:<8} {k:>3} symbols  {lo:#05x}..{hi:#05x}  stride {st}"
              f"  {arc:5.1f} degrees")
    print(f"  ordered            {len(order):,} assigned codepoints "
          f"({time.time()-t0:.1f}s)")
    print(f"  longest reading    {max(lens)} characters -> "
          f"key at most {max(lens)*(RING+3)+42} bits\n")
    print("  READING SOURCE")
    NAMES = {"table":"1  in Chroma UTF (Latin, digits)","hangul":"2  Hangul — name is the romanisation",
             "han":"3  Han — Unihan reading + strokes","han-unread":"3* Han with no reading on file",
             "script":"4  other script — letter core of the name",
             "symbol":"5  symbol / emoji — the whole name","unnamed":"6  unnamed (private use, control)"}
    for k in ["table","hangul","han","han-unread","script","symbol","unnamed"]:
        if layers[k]: print(f"     {layers[k]:>8,}  {NAMES[k]}")
    print()
    fails = []
    # [1] total
    print(f"  [1] total          {len(set(keys)):,}/{len(keys):,} distinct keys, no ties")
    if len(set(keys)) != len(keys): fails.append(1)
    # [2] monotone in the folded value, sampled (cells are expensive at 292k)
    random.seed(7); s = sorted(random.sample(range(len(order)-1), 4000))
    inv = 0
    for i in s:
        (an, ad), (bn, bd) = value(chr(order[i])), value(chr(order[i+1]))
        if an*bd >= bn*ad: inv += 1
    print(f"  [2] monotone       {inv} inversions in 4,000 sampled adjacent pairs "
          f"(exact rationals)")
    if inv: fails.append(2)
    # [3] stable
    sh = cps[:]; random.shuffle(sh)
    print(f"  [3] stable         reshuffled input sorts identically: "
          f"{'yes' if [c for _,c in build(sh)] == order else 'NO'}")
    if [c for _, c in build(sh)] != order: fails.append(3)
    # [4] radix-sortable: bit order == key order
    sub = [order[i] for i in sorted(random.sample(range(len(order)), 3000))]
    W = max(len(cells(chr(c))) for c in sub)
    rk = lambda c: int("1" + "".join(map(str, cells(chr(c)))).ljust(W, "0"), 2)
    print(f"  [4] radix-sortable integer key order == comparison order: "
          f"{'yes' if sorted(sub, key=rk) == sub else 'NO'}   (3,000 sampled, {W} bits)")
    if sorted(sub, key=rk) != sub: fails.append(4)
    # [5] the tie-break ladder actually decides
    lad = collections.Counter()
    for i in range(len(keys)-1):
        a, b = keys[i], keys[i+1]
        n = min(len(a), len(b))
        for j in range(n):
            if a[j] != b[j]:
                lad["reading" if j < n-5 else
                    ["terminator","language","tone","strokes","codepoint"][j-(n-5)]] += 1
                break
        else: lad["length"] += 1
    print(f"  [5] ladder used    " + ", ".join(f"{k} {v:,}" for k, v in lad.most_common()))
    # [6] declared order survives at scale
    lat = [c for c in order if chr(c) in UTF]
    print(f"  [6] base intact    {'yes' if [chr(c) for c in lat] == TABLE else 'NO'}"
          f"   — the 306 stay in declared order inside the full sort")
    if [chr(c) for c in lat] != TABLE: fails.append(6)
    print(f"\n  {'ALL PROPERTIES HOLD' if not fails else 'FAILED: ' + str(fails)}")
    W = ducet()
    ranks = sorted({W[ord(m)][1] for ch in TABLE for m in u.normalize("NFD", ch)[1:]
                    if ord(m) in W})
    rank = {w: i + 1 for i, w in enumerate(ranks)}
    with open(os.path.join(DATA, "chroma-base.tsv"), "w", encoding="utf8") as f:
        f.write("# char\tbase\tcase\taccent\trank\tcode\ttype   — Chroma UTF base "
                f"table, DUCET {u.unidata_version} with case above accent. "
                f"rank orders, code draws.\n")
        for ch in TABLE:
            d = u.normalize("NFD", ch)
            b = W.get(ord(d[0]), (0xFFFF, 0, 0))
            # a character can carry more than one mark (ǟ = a + diaeresis + macron).
            # The accent level is a SEQUENCE, compared left to right like a reading;
            # flattening it to one number reorders the double accents.
            a = [rank[W[ord(m)][1]] for m in d[1:] if ord(m) in W]
            f.write(f"{ch}\t{b[0]}\t{b[2]}\t{'.'.join(map(str, a)) or '0'}"
                    f"\t{RANK[ch]}\t{CODE[ch]}\t{typeof(ch)}\n")
    with open(os.path.join(DATA, "chroma-order.tsv"), "w", encoding="utf8") as f:
        for pos, cp in enumerate(order):
            r, tone, st, L, _ = reading(chr(cp))
            f.write(f"{pos}\t{cp:04X}\t{chr(cp) if L!='unnamed' else ''}\t{r or ''}\t{tone}\t{st}\t{L}\n")
    print(f"  wrote data/chroma-order.tsv  ({len(order):,} rows)")
