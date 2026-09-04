#!/usr/bin/env python3
"""python3 tools/chroma_scale.py [en.txt es.txt ja.txt] — 100,000 words, three filters.

Sorts the same corpus three ways and reports what moved:

    en              English only
    en es           English and Spanish
    en ja-on es     English, Japanese on'yomi, and Spanish

The corpus is deliberately mixed, so every filter has words it cannot read
natively. That is the interesting case: a filter does not fail on a word from
another language, it reads it with the rules it has.

Word lists are not committed. English is /usr/share/dict/words. For the others:

  curl -sSo es.txt https://raw.githubusercontent.com/lorenbrichter/Words/master/Words/es.txt
  curl -sSo ja.txt https://raw.githubusercontent.com/hingston/japanese/master/44492-japanese-words-latin-lines-removed.txt
"""
import sys, os, time, random, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import chroma_sort as S, chroma_phonetic as P, chroma_utf as C

TARGET = 100_000
MIX = [("en", 40_000), ("es", 40_000), ("ja", 20_000)]
CONFIGS = [("en", "English only"),
           ("en es", "English and Spanish"),
           ("en ja-on es", "English, Japanese on'yomi, Spanish")]


LEX = {"en": "en", "es": "es", "ja": "ja-on"}


def load(paths):
    """Sample the corpus, and register the FULL lists as detection lexicons.

    The lexicons are the full lists, not the sample: detection asks "does this
    word exist in that language", and a word left out of the sample still does.
    """
    out, seen = {}, set()
    for (tag, n), path in zip(MIX, paths):
        if not os.path.exists(path):
            sys.exit(f"missing word list for {tag}: {path}\n{__doc__}")
        words = [w.strip() for w in open(path, encoding="utf8", errors="replace")]
        words = [w for w in words if w and " " not in w]
        random.Random(hash(tag) & 0xFFFF).shuffle(words)
        picked = []
        for w in words:
            if w in seen: continue                    # "different words" means different
            seen.add(w); picked.append(w)
            if len(picked) == n: break
        out[tag] = picked
        n = S.load_lexicon(LEX[tag], path)
        print(f"  {tag}  {len(picked):>7,} of {len(words):,} available, "
              f"lexicon {n:,}   e.g. " + " ".join(picked[:4]))
    return out


def run(words, lang):
    t = time.time()
    keys = {}
    for w in words: keys[w] = S.key(w, lang)[0]
    order = sorted(words, key=lambda w: keys[w])
    return order, keys, time.time() - t


def main(paths):
    print(__doc__.splitlines()[0] + "\n")
    lists = load(paths)
    words = [w for t, _ in MIX for w in lists[t]]
    random.Random(9).shuffle(words)
    print(f"\n  corpus  {len(words):,} distinct words, three scripts mixed")
    # how well can the corpus even be told apart?
    claims = collections.Counter()
    for w in words:
        hits = tuple(t for t in ("en", "es", "ja-on")
                     if w.lower() in S.LEXICON.get(t, ()))
        claims[len(hits)] += 1
        if len(hits) > 1: claims[hits] += 1
    amb = {k: v for k, v in claims.items() if isinstance(k, tuple)}
    print(f"  detection  claimed by one lexicon {claims[1]:,}, "
          f"by several {sum(v for k, v in claims.items() if isinstance(k, int) and k > 1):,}, "
          f"by none {claims[0]:,}")
    if amb:
        print("             overlaps: " + ", ".join(
            f"{'+'.join(k)} {v:,}" for k, v in sorted(amb.items(), key=lambda x: -x[1])[:4]))
    print()

    results = []
    for lang, label in CONFIGS:
        order, keys, dt = run(words, lang)
        pos = {w: i for i, w in enumerate(order)}
        ks = [keys[w] for w in order]

        perm = sorted(order) == sorted(words)
        mono = all(ks[i] <= ks[i + 1] for i in range(len(ks) - 1))
        distinct = len(set(ks))
        clash = collections.Counter()
        byk = collections.defaultdict(list)
        for w in order: byk[keys[w]].append(w)
        groups = [g for g in byk.values() if len(g) > 1]

        print(f"  {label}   (--lang {lang!r})")
        print(f"     sorted in            {dt:.1f}s")
        print(f"     permutation          {'held' if perm else 'BROKEN'}")
        print(f"     keys nondecreasing   {'yes' if mono else 'NO'}")
        print(f"     distinct keys        {distinct:,}/{len(words):,}"
              f"   {len(words)-distinct:,} words share a key with another")
        if groups:
            groups.sort(key=len, reverse=True)
            print(f"     homophone groups     {len(groups):,}, largest {len(groups[0])}: "
                  + " ".join(groups[0][:6]))
        print(f"     first ten            " + " ".join(order[:10]))
        print()
        results.append({"lang": lang, "label": label, "order": order, "pos": pos,
                        "keys": keys, "groups": groups})

    base = results[0]
    print("  WHAT MOVES WHEN A LANGUAGE IS ADDED\n")
    for r in results[1:]:
        moved = [w for w in words if r["pos"][w] != base["pos"][w]]
        shifts = [abs(r["pos"][w] - base["pos"][w]) for w in moved]
        reread = [w for w in words
                  if S.key(w, r["lang"])[1] != S.key(w, base["lang"])[1]]
        print(f"  {base['lang']!r} -> {r['lang']!r}")
        print(f"     positions changed    {len(moved):,} of {len(words):,}"
              f"   ({100*len(moved)/len(words):.1f}%)")
        if shifts:
            shifts.sort()
            print(f"     displacement         median {shifts[len(shifts)//2]:,}, "
                  f"mean {sum(shifts)//len(shifts):,}, max {shifts[-1]:,}")
        print(f"     readings changed      {len(reread):,}")
        if reread:
            print(f"     e.g. " + ",  ".join(
                f"{w} {S.key(w, base['lang'])[1]} -> {S.key(w, r['lang'])[1]}"
                for w in reread[:4]))
        big = sorted(moved, key=lambda w: -abs(r["pos"][w] - base["pos"][w]))[:5]
        for w in big:
            print(f"        {w:<16} {base['pos'][w]:>7,} -> {r['pos'][w]:>7,}"
                  f"   {S.key(w, base['lang'])[1]} -> {S.key(w, r['lang'])[1]}")
        print()

    print("  THE SAME WORD DOWN EACH FILTER\n")
    probe = [w for w in ["cerveza", "zapato", "canvas", "this", "knight"] if w in base["pos"]]
    probe += [w for w in words if any(ord(c) > 0x3000 for c in w)][:3]
    w0 = max((len(w) for w in probe), default=8)
    print(f"     {'word'.ljust(w0)}  " + "  ".join(f"{c[0]:<22}" for c in CONFIGS))
    for w in probe:
        cells = []
        for r in results:
            cells.append(f"{S.key(w, r['lang'])[1][:13]:<13} {r['pos'][w]:>7,}")
        print(f"     {w.ljust(w0)}  " + "  ".join(f"{c:<22}" for c in cells))

    bad = [r["label"] for r in results
           if sorted(r["order"]) != sorted(words)
           or not all(r["keys"][r["order"][i]] <= r["keys"][r["order"][i+1]]
                      for i in range(len(words) - 1))]
    print("\n  " + ("every filter held: permutation and nondecreasing keys"
                    if not bad else "FAILED: " + ", ".join(bad)))
    return 1 if bad else 0


if __name__ == "__main__":
    a = sys.argv[1:]
    if a and a[0] in ("-h", "--help"): print(__doc__); sys.exit(0)
    d = os.environ.get("WORDS", "/tmp")
    default = ["/usr/share/dict/words", f"{d}/es.txt", f"{d}/ja.txt"]
    sys.exit(main(a if len(a) == 3 else default))
