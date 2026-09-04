#!/bin/sh
# Build inputs for the Chroma Certified Ordering. Public Unicode data, not committed.
set -e
cd "$(dirname "$0")/.."
mkdir -p data
[ -f data/allkeys.txt ] || curl -sS -o data/allkeys.txt \
  https://www.unicode.org/Public/UCA/latest/allkeys.txt
[ -f data/unihan.json ] || {
  T=$(mktemp -d); curl -sS -o "$T/Unihan.zip" \
    https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip
  unzip -oq "$T/Unihan.zip" -d "$T"
  python3 - "$T" <<'PY'
import sys, collections, json, os
T = sys.argv[1]; F = collections.defaultdict(dict)
for fn in ("Unihan_Readings.txt", "Unihan_IRGSources.txt"):
    for line in open(os.path.join(T, fn), encoding="utf8"):
        if line.startswith("#") or not line.strip(): continue
        p = line.rstrip("\n").split("\t", 2)
        if len(p) == 3: F[p[1]][int(p[0][2:], 16)] = p[2]
want = {"mandarin":"kMandarin", "cantonese":"kCantonese", "on":"kJapaneseOn",
        "kun":"kJapaneseKun", "korean":"kKorean", "strokes":"kTotalStrokes"}
json.dump({k: {str(a): b for a, b in F[v].items()} for k, v in want.items()},
          open("data/unihan.json", "w"), ensure_ascii=False)
PY
  rm -rf "$T"; }
ls -la data/allkeys.txt data/unihan.json | awk '{print "  "$5"\t"$9}'
