#!/usr/bin/env python3
"""python3 tools/chroma_sort_test.py — the word-level sort certificate.

A filename sort has one hard requirement the character index does not: it must
be a PERMUTATION. Drop or duplicate a file and the sort is worse than useless.
"""
import sys, os, random, unicodedata as u
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import chroma_sort as S, chroma_utf as C

fails = []
def check(n, cond, msg):
    print(f"  [{n}] {msg}")
    if not cond: fails.append(n)

WORDS = ["shit", "飼", "this", "isit", "canvas", "code", "kode", "cerveza",
         "Cerveza", "abhan", "Avon", "école", "Äpfel", "apple", "zebra",
         "中国", "한국", "日本", "🎩", "photo", "foto", "knight", "night",
         "quick", "kwik", "thing", "sing", "gnome", "nome", "", "a"]

# [1] permutation
out = S.chroma_sorted(WORDS, "en")
check(1, sorted(out) == sorted(WORDS) and len(out) == len(WORDS),
      f"permutation      {len(out)}/{len(WORDS)} in, none dropped or duplicated")

# [2] deterministic under reshuffle
sh = WORDS[:]; random.seed(3); random.shuffle(sh)
check(2, S.chroma_sorted(sh, "en") == out,
      "stable           reshuffled input sorts identically")

# [3] total: distinct strings get distinct keys, so ties never depend on input order
keys = [S.key(w, "en")[0] for w in WORDS]
check(3, len(set(keys)) == len(WORDS),
      f"total            {len(set(keys))}/{len(WORDS)} distinct keys "
      f"(homophones separate on spelling: "
      f"{'code/kode ' + ('differ' if S.key('code','en')[0] != S.key('kode','en')[0] else 'TIE')})")

# [4] digraphs are one sound, not two letters
sh_read = S.key("shit", "en")[1]
th_read = S.key("this", "en")[1]
check(4, sh_read == "shit" and S.segment("shit") == ["sh", "i", "t"]
      and S.segment("this") == ["th", "i", "s"],
      f"digraphs         shit -> {S.segment('shit')}, this -> {S.segment('this')}")

# [5] the spelling lies: c-words move to k
moved = [(w, S.key(w, "en")[1]) for w in ["canvas", "code", "cerveza", "photo", "knight"]]
check(5, all(r and r[0] != w[0] for w, r in moved),
      "spelling lies    " + ", ".join(f"{w}->{r}" for w, r in moved))

# [6] declaring a language moves the multi-branch items, and only those
base = S.chroma_sorted(["shit", "飼", "this", "isit"])
ja   = S.chroma_sorted(["shit", "飼", "this", "isit"], "ja-on")
same = [w for w in base if base.index(w) == ja.index(w)]
check(6, base != ja and set(base) == set(ja)
      and S.key("飼")[1] == "si" and S.key("飼", "ja-on")[1] == "shi",
      f"language moves   飼 reads si undeclared, shi as ja-on; "
      f"{len(base)-len(same)} of 4 positions change")

# [7] the primary branch, not the minimum. Taking the minimum read isit as ishit.
check(7, S.key("isit", "en")[1] == "isit"
      and any(r[1] == "ishit" for r in S.positions("isit")),
      "primary not min  isit reads isit; ishit is still one of its "
      f"{len(S.positions('isit'))} positions")

# [8] real filenames, if any are around
d = os.path.expanduser("~/Downloads")
if os.path.isdir(d):
    names = os.listdir(d)
    got = S.chroma_sorted(names, "en")
    ks = [S.key(n, "en")[0] for n in got]
    check(8, sorted(got) == sorted(names) and all(ks[i] <= ks[i+1] for i in range(len(ks)-1)),
          f"real filenames   {len(names)} names, permutation held, keys nondecreasing")

print("\n  " + ("certified." if not fails else f"FAILED: {fails}"))
sys.exit(1 if fails else 0)
