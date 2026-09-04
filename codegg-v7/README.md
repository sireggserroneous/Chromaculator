# codegg v7 — the Transmuter

We are not encoding. **eggv7 TRANSMUTES data into another form and RESTORES
it.** The first law is conservation: the information never moves; only the
form does — push's law from the site ("the value never moves; only the
colours do"), generalized to whole files. The FNV-64 of the original bytes
rides in the voted header as the conservation check, and no restore returns
without it. The transmuted form of structured data happens to WEIGH LESS —
a property of the form, not a goal we chase — and the same form survives
damage that kills every compressor in the tournament.

Built 2026-08-31 against an approved plan with predictions filed before each
stage ([PREDICTIONS.md](PREDICTIONS.md) — guesses and misses kept side by side).

## What failed, first

- **3 of 8 structured files still weigh more than gzip -9** after armor:
  program.exe (49.7% vs ~48.2%), repo-bundle.bin (38.2% vs 32.0%),
  real-test.bmp (31.2% vs 30.5%). The first two were predicted losses —
  small files pay the armor's densest rib prices (G8 ≈ +27%, G32 ≈ +7.5%) —
  and bmp was a filed coin flip that landed a loss: its literals are
  ~8.1-bit pixel noise that no general-purpose context reaches.
- **The first dyadic build LOST to gzip on server-log.json** (15.33% vs
  15.30%) against a 10–12% prediction. The stats instrumentation convicted
  the match finder: 57% of the output was offsets, because a flat literal
  price kept matches seeded inside random hex fields whose nearest
  recurrence is megabytes away. The fix — price literals at each file's own
  measured order-2-nib entropy — is now part of the tokenizer.
- **The victory bar nearly fell to a gzip technicality**: node's zlib
  `gzipSync -9` emits a smaller stream than CLI `gzip -9` on big.xml
  (500,477 vs 507,694 B). The first tournament run scored 4/8 against it.
  The bar was held against the STRONGER gzip per file, and three more model
  steps (wider length trees, matched-byte literals, deeper chains) earned
  the win honestly.
- **The speed target (≥5 MB/s transmute) was missed**: ~2.4 MB/s average,
  worst 0.8 MB/s on data.csv — dual-depth literal coding plus 512-deep hash
  chains were spent on weight, not speed. Restore runs 15–100 MB/s.
- v2's NAF/DIV recipes are gone: measured at file scale they ~never fired
  (the ledger is in codegg-v2/README.md). Only the bar survived, generalized.

## The transmutation chain

```
bytes -> NIBS -> TOKENS (match/literal) -> DYADIC POINT -> ARMORED FORM -> .egg7
         |            |                        |               |
   the site's    "the bar,               the Atlas as     eggv6's shield,
   atom (hex     generalized":           codebook:        lifted whole:
   cells, 4-bit) LZ77-family match      adaptive binary   residues + RS
                 finder, whole-file      arithmetic        parity + stripe +
                 window, rep offsets     coding on nibs    voted headers
```

The centerpiece is the dyadic stage, hidden in the Atlas all along: the
Atlas page draws the Poincaré disc of the **dyadic tree**, and an arithmetic
coder is a machine that walks down that tree and emits the address of the
interval where the message lands. The site's numbers ARE arithmetic-coder
outputs. So the whole token stream becomes **one dyadic rational — a single
point on the site's own disc**. Structured files land in wide intervals
(short addresses); random files land in intervals as narrow as themselves.
That is the pigeonhole, kept and asserted as a test PASS: random MUST come
out >100%, and does (photo.bin 103.3% armored, 100.86% bare).

Restore walks the chain backward: de-armor → the point back to tokens →
detokenize → bytes → conservation check. Damage beyond capacity is refused
with the armor's report; nothing is ever returned wrong with a success code.

## The tournament (10 files, 3 injuries each, rule enforced mechanically)

Rule (Vladimir's): wrong-or-no data after any injury forfeits; smallest
lossless survivor wins. Injuries: blind 1-byte flip, addressed 4 KB scratch,
4 KB truncation.

| file | orig B | gzip | zstd | brotli | xz | egg6 | e6+zstd | egg7 | winner |
|---|---|---|---|---|---|---|---|---|---|
| server-log.json | 7,549,839 | dq | dq | dq | dq | 102.4% | 13.5% | **14.3%** | egg6+zstd |
| data.csv | 4,960,761 | dq | dq | dq | dq | 102.4% | 24.5% | **24.3%** | **egg7** |
| big.xml | 4,156,948 | dq | dq | LIED | dq | 102.5% | 10.9% | **11.9%** | egg6+zstd |
| program.exe | 265,216 | dq | dq | dq | dq | 108.0% | 48.1% | 49.7% | egg6+zstd |
| repo-bundle.bin | 215,055 | dq | dq | LIED | dq | 108.4% | 36.7% | 38.2% | egg6+zstd |
| real-test.db | 9,551,872 | dq | dq | LIED | dq | 102.4% | 17.7% | **16.4%** | **egg7** |
| real-test.bmp | 12,000,054 | dq | dq | LIED | dq | 102.4% | 31.0% | 31.2% | egg6+zstd |
| corpus-1489k.bin | 1,489,000 | dq | dq | LIED | dq | 102.6% | 30.7% | **31.1%** | egg6+zstd |
| archive.zst | 946,623 | dq | dq | LIED | dq | 107.3% | 107.3% | 107.7% | egg6 |
| photo.bin | 4,194,304 | dq | dq | LIED | dq | 102.5% | 102.5% | 103.3% | egg6 |

- **Victory (the plan's falsifiable bar): egg7, armor ON, restored all three
  injuries EXACT on every file and weighs less than gzip -9 on 5 of the 8
  structured files** (bold in the egg7 column). The bar was ≥5. Met.
- egg7 never forfeited and never lied — 30 of 30 injuries restored EXACT,
  independently fingerprinted by certutil SHA-256 (`tools/verify.js`:
  ALL FINGERPRINTS MATCH). The compressors forfeited every file; brotli
  returned wrong bytes with a success code on 7 of 10 (RFC 7932 has no
  mandatory checksum — the v6 finding, reproduced).
- The podium stays in-house: egg6+zstd ×6, egg7 ×2, egg6 ×2. The old hybrid
  is still the lightest survivor on most files — but it is a borrowed
  compressor inside our armor. **egg7 is the house's own form end to end**,
  and it beat the hybrid outright on data.csv and real-test.db.

## The real-file tournament (12 files off this machine, nothing generated)

Run 2026-09-01 at Vladimir's ask: at least 10 REAL files. Same rule, same
three injuries. egg7 column is armor ON; "best gzip -9" is the stronger of
CLI gzip and zlib per file. egg7 survived all 36 injuries EXACT; gzip, zstd
and xz forfeited every file; brotli LIED (wrong bytes, success code) on 6 of
12. Podium: egg6+zstd x10, egg7 x2 (real-test.db and the JPEG, outright).

| file | class | orig B | best gzip -9 | egg7 armored | verdict |
|---|---|---|---|---|---|
| vim-version9.txt | English text | 2,035,039 | 24.2% | **21.4%** | WIN |
| real-test.db | SQLite | 9,551,872 | 19.5% | **16.4%** | WIN (won the file) |
| segoeui.ttf | TrueType font | 959,752 | 56.5% | **53.0%** | WIN |
| arial.ttf | TrueType font | 1,045,720 | 53.6% | **53.3%** | WIN |
| zstd.exe | PE executable | 1,601,409 | 41.7% | **40.1%** | WIN |
| kernel32.dll | system DLL | 836,208 | 45.9% | **45.2%** | WIN |
| wallpaper.jpg | JPEG photo | 1,602,752 | 98.9% | **97.7%** | WIN (won the file) |
| real-test.bmp | BMP image | 12,000,054 | 30.5% | 31.2% | LOSS |
| notepad.exe | PE executable | 360,448 | 55.5% | 58.0% | LOSS (G32 rib price) |
| wubbadub.html | HTML | 92,408 | 30.8% | 40.1% | LOSS (G8 rib price) |
| alarm01.wav | PCM audio | 491,516 | 78.2% | 84.5% | LOSS |
| ring01.wav | PCM audio | 498,420 | 58.9% | 62.0% | LOSS |

**7 of 12 real files lighter than the strongest gzip -9, with armor on and
all injuries restored EXACT — the bar (>=5) holds on real data.** The losses
say what they always say: small artifacts pay the dense-rib price (notepad,
wubbadub), the bmp stays 0.7 pt short, and PCM audio wants sample-domain
modeling (delta/LPC) that a byte-matcher honestly does not have -- xz and
brotli beat gzip there too, the same way. The JPEG row is the surprise worth
keeping: a "pre-compressed" real photo carried ~2% of recoverable slack
(metadata, thumbnail, low-entropy regions), and the transmuted point found
it while staying repairable.

## The drill battery (tools/drills.js — 64/64, zero silent)

Per rib class (G8 / G32 / G126): pristine restore; 20 blind single-byte
wounds; 4 KB scratches at payload, checks, head and end, blind AND
addressed; capacity-sized check-table wounds; twin stripe wounds (two dead
squares in one group — v5.2's caveat, killed by T=2); triple stripe wounds
refused honestly; 4 KB truncation repaired, 30% truncation refused honestly;
half-container scratches refused honestly. Plus 300× blind 3-bit storms on
the mid artifact: **300/300 EXACT, 0 silent**. Plus the no-armor hash gate:
any wound refused; truncating exactly hdr2 survives (three voted headers).

Costs on the label: armor overhead by artifact size is ~2.5% (≥1 MB, G126),
~7.5% (<1 MB, G32), ~27% (<64 KB, G8) — plan's rib policy. Contiguous blind
capacity ≈ artifact/G; a check-table wound beyond T×(CT groups) squares is
detected, never repaired, never silent. Not tamper-proof: residues, RS and
FNV stop accidents, not adversaries.

## Model, briefly (all adaptive 12-bit counters, one range coder)

Literal nibs down 15-node trees, context = previous 2 or 4 nibs — both
depths are coded and the lighter point is kept (the model byte remembers);
the first literal after a match is steered by the byte the match source
would have continued with. Token type by last three types; four recent
offsets in a move-to-front rack (~3 bits to name a stride again);
log2-bucketed lengths and offsets with adaptive trees on the narrow buckets,
raw bits + align tree on the wide ones. The tokenizer prices every candidate
match against the file's own measured literal entropy and refuses matches
that cost more to name than to spell.

## Run it

```bash
cd codegg-v7 && cargo build --release        # std only, no deps, ~2 s
node tools/mkcorpus7.js                      # deterministic corpus (seeds fixed)
target/release/eggv7 transmute <file>        # -> <file>.egg7 (armor ON; --no-armor for bare weight)
target/release/eggv7 restore <file>.egg7     # conservation-gated, honest or exact
target/release/eggv7 scratch <file> --len 65536 --at payload|checks|head|end
target/release/eggv7 info <file>.egg7        # geometry as JSON
node tools/drills.js                         # the 64-drill battery
node tools/standings.js corpus/*             # the tournament
node tools/verify.js corpus/*                # certutil SHA-256, no code of ours
```

## Lineage (the parts are old; the fit is the local thing)

Ziv–Lempel 1977 (the match layer; the site's bar notation at arbitrary
offsets). Elias, Rissanen, Witten–Neal–Cleary (arithmetic coding). Igor
Pavlov's LZMA (the range-coder shape, rep offsets, slot/align offset coding,
matched-byte literals). Avizienis 1971 / Mandelbaum 1976 (residue checks).
Reed–Solomon 1960 via Lagrange evaluation (the Wub reading). The site
supplied the geometry, the vocabulary, and the dyadic tree the whole file
now lives on as a single point.
