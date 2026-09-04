# eggv7 predictions — filed 2026-08-31, BEFORE the token and dyadic stages were built

House rule: predictions first, measurements after, unflattering rows kept.
The bar is **gzip -9 with armor included**, on ≥5 of the 8 structured files,
with all three tournament injuries restored EXACT.

## Baselines measured tonight on the regenerated corpus (percent of input)

| file | orig B | gzip -9 | zstd -19 | xz -9 | brotli |
|---|---|---|---|---|---|
| server-log.json | 7,549,839 | **15.30%** | 12.54% | 12.67% | 12.22% |
| data.csv | 4,960,761 | **27.85%** | 23.84% | 20.42% | 22.19% |
| big.xml | 4,156,948 | **12.21%** | 10.17% | 9.74% | 9.89% |
| program.exe | 225,280 | **49.06%** | 44.98% | 41.98% | 42.63% |
| repo-bundle.bin | 215,055 | **31.98%** | 28.35% | 27.66% | 26.74% |
| real-test.db | 9,551,872 | **19.60%** | 17.28% | 15.44% | 15.69% |
| real-test.bmp | 12,000,054 | **30.54%** | 30.21% | 30.11% | 30.25% |
| corpus-1489k.bin | 1,489,000 | **32.26%** | 28.54% | 28.53% | 28.37% |
| archive.zst | 946,623 | 100.02% | 100.00% | 100.01% | 100.00% |
| photo.bin | 4,194,304 | 100.02% | 100.00% | 100.01% | 100.00% |

Notes: program.exe is the M0 scaffold build (225 KB); the final exe will be
larger and the row is re-measured at M4 with the final build. bold = the bar.

## M1 prediction — token layer alone (matches + literals as raw nibs, no entropy)

LZ4-style sequences, min match 4, whole-file window, hash chains, 1-step lazy.

| file | predicted M1 size |
|---|---|
| server-log.json | 30–38% |
| data.csv | 45–55% |
| big.xml | 28–36% |
| program.exe | 75–90% |
| repo-bundle.bin | 45–55% |
| real-test.db | 35–45% |
| real-test.bmp | 65–90% |
| corpus-1489k.bin | 45–60% |
| archive.zst | 100–105% |
| photo.bin | 100–105% |

Gate: round-trip byte-exact on all 10. Both tails (zst/photo) MUST exceed 100%.

## M2 prediction — dyadic stage (order-2 nib literals, bucketed len/dist), --no-armor

| file | predicted M2 size | vs gzip bar | call |
|---|---|---|---|
| server-log.json | 10–12% | 15.30% | WIN |
| data.csv | 20–25% | 27.85% | WIN |
| big.xml | 8–11% | 12.21% | WIN |
| program.exe | 44–49% | 49.06% | coin flip |
| repo-bundle.bin | 26–30% | 31.98% | WIN (pre-armor) |
| real-test.db | 14–17% | 19.60% | WIN |
| real-test.bmp | 27.5–30.5% | 30.54% | narrow WIN |
| corpus-1489k.bin | 27–31% | 32.26% | WIN |
| archive.zst | ~100.05% | — | MUST be >100 (pigeonhole) |
| photo.bin | ~100.05% | — | MUST be >100 (pigeonhole) |

Gate: ≤ gzip -9 on ≥5/8 before armor. Speed ≥5 MB/s transmute (predict 8–25 MB/s;
the order-2 binary coder is the bottleneck).

## M3 prediction — armor ON (rib policy: artifact <64K→G8, <1M→G32, else G126)

Armor multiplier by artifact size: G126 ≈ ×1.027, G32 ≈ ×1.074, G8 ≈ ×1.27.

| file | predicted final | bar | call |
|---|---|---|---|
| server-log.json | 11–13% (artifact ~0.8 MB → G32) | 15.30% | WIN |
| data.csv | 21–26% (artifact ~1.1 MB → G126) | 27.85% | WIN |
| big.xml | 9–12% (artifact ~0.4 MB → G32) | 12.21% | WIN |
| program.exe | 47–52% (artifact ~105 KB → G32) | 49.06% | predicted LOSS (marginal) |
| repo-bundle.bin | 32–36% (artifact ~60 KB → G8 boundary) | 31.98% | predicted LOSS — structural: at the 64 KB rib boundary even a 29.8% dyadic form lands ≥31.96% armored |
| real-test.db | 15–18% (artifact ~1.5 MB → G126) | 19.60% | WIN |
| real-test.bmp | 28.5–31.5% (artifact ~3.5 MB → G126) | 30.54% | coin flip |
| corpus-1489k.bin | 29–33% (artifact ~430 KB → G32) | 32.26% | narrow WIN |
| archive.zst | ~107.5% (G32) | — | >100, honest |
| photo.bin | ~102.7% (G126) | — | >100 asserted as PASS |

Predicted score: **5–6 wins of 8** — the bar (≥5) is met with no slack.
Predicted failures, filed in advance: program.exe and repo-bundle.bin lose to
the armor's own price at small artifact sizes; that is the cost of a form that
also survives what kills gzip, and the README will print it, not excuse it.

Injuries (all three, per file): blind 1-byte flip → bit-scale repair, EXACT;
addressed 4 KB scratch → stripe puts ≤1 dead square per group, RS rebuilds,
EXACT (artifact/G capacity clears 4 KB on every corpus member under its rib);
4 KB truncation → pad-as-wound, EXACT. Zero silent wrong answers anywhere on
the ladder.

---

# Measured (filled in after each gate; predictions above left untouched)

## M1 measured — token layer alone (gate: round-trip EXACT on all 10 — PASSED)

| file | predicted | measured | verdict on the prediction |
|---|---|---|---|
| server-log.json | 30–38% | 22.96% | MISS — finder stronger than guessed |
| data.csv | 45–55% | 44.53% | near miss (better) |
| big.xml | 28–36% | 18.19% | MISS — whole-file window underrated |
| program.exe | 75–90% | 65.12% | MISS (better) |
| repo-bundle.bin | 45–55% | 47.01% | HIT |
| real-test.db | 35–45% | 23.64% | MISS (better) |
| real-test.bmp | 65–90% | 30.66% | BIG MISS — the bmp's 30% floor is mostly match structure, not entropy |
| corpus-1489k.bin | 45–60% | 30.26% | MISS (better) |
| archive.zst | 100–105% | 100.02% | HIT, >100 as required |
| photo.bin | 100–105% | 100.01% | HIT, >100 as required |

## M2 measured — dyadic stage, --no-armor (gate: ≤ gzip -9 on ≥5/8 — PASSED 8/8 after tuning)

First build vs final (the tuning ledger, each step measured):

| file | first build | + reps/pricing | + len trees | + chains 512 + match-byte + ctx | final |
|---|---|---|---|---|---|
| server-log.json | 15.33% (LOST to gzip!) | 14.71% | 14.09% | 13.94% | **13.94%** |
| data.csv | 27.44% | 25.35% | 24.01% | 23.71% | **23.71%** |
| big.xml | 11.99% | 11.67% | 11.41% | 11.07% | **11.07%** |
| program.exe | 47.12% | 46.75% | 46.77% | 45.49% | **45.49%** |
| repo-bundle.bin | 29.79% | 29.72% | 29.66% | 29.56% | **29.56%** |
| real-test.db | 17.46% | 17.04% | 16.05% | 15.97% | **15.97%** |
| real-test.bmp | 30.63% | 30.63% | 30.45% | 30.42% | **30.42%** |
| corpus-1489k.bin | 29.05% | 29.08% | 28.98% | 28.95% | **28.95%** |
| photo.bin | 101.29% | 101.28% | 100.86% | 100.86% | **100.86%** (>100 required, held) |

Prediction misses worth keeping: server-log was predicted 10–12% and the first
build LOST to gzip at 15.33% — the flat 6-bit literal price kept matches seeded
inside random hex fields (57% of the output was offsets; --stats convicted it).
The fix was measuring each file's own order-2-nib entropy as the literal price.
bmp was predicted a narrow win and stayed a narrow loss: its literals are
~8.09-bit pixel noise no general context reaches.

## M3 measured — armor ON (gate: full battery EXACT-or-honest, zero silent — PASSED 64/64)

The bar was held against the STRONGER gzip per file (zlib -9 beats CLI gzip -9
on big.xml by 1.4%; the first tournament run scored 4/8 against it and forced
the last three tuning steps).

| file | predicted final | measured final | bar (best gzip) | call (predicted → measured) |
|---|---|---|---|---|
| server-log.json | 11–13% | 14.31% | 14.81% | WIN → WIN (thinner than hoped) |
| data.csv | 21–26% | 24.33% | 27.56% | WIN → WIN |
| big.xml | 9–12% | 11.92% | 12.04% | WIN → WIN (by 0.12 pt) |
| program.exe | 47–52% | 49.70% | ~48.2% | LOSS → LOSS (as filed) |
| repo-bundle.bin | 32–36% | 38.20% | 31.98% | LOSS → LOSS (as filed) |
| real-test.db | 15–18% | 16.38% | 19.49% | WIN → WIN |
| real-test.bmp | 28.5–31.5% | 31.17% | 30.54% | coin flip → LOSS |
| corpus-1489k.bin | 29–33% | 31.14% | 32.15% | WIN → WIN |
| archive.zst | ~107.5% | 107.68% | — | >100, honest |
| photo.bin | ~102.7% | 103.33% | — | >100 asserted as PASS |

**Score: 5 of 8 — the bar (≥5) is met, exactly with the predicted 5–6 range's
floor.** Speed prediction MISSED: transmute averages ~2.4 MB/s (csv 0.8, bmp 5.2)
against the ≥5 MB/s target — the dual-depth literal coding and 512-deep chains
were spent on weight, not speed. Restore runs 15–100 MB/s.
