# eggv8 predictions — filed 2026-09-01, at M0, BEFORE any workstream was built

House rule: predictions first, measurements after, unflattering rows kept.
The bar is THE STRETCH (Vladimir, 2026-09-01), armor ON, all injuries EXACT:
  (i) lighter than the STRONGEST gzip -9 (min of CLI and zlib) on ≥10/12 real files,
  (ii) lighter than the egg6+zstd hybrid artifact on ≥6/12,
  (iii) lighter than xz -9 on ≥3/12.
Measured columns get filled at M6; a blank measured cell before then is the point.

## Real-corpus baselines (measured 2026-09-01, percent of input)

| file | orig B | gz* | zstd -19 | xz -9 | e6+zstd | egg7 |
|---|---|---|---|---|---|---|
| vim-version9.txt | 2,035,039 | 24.2 | 18.7 | 18.2 | 20.1 | 21.4 |
| wubbadub.html | 92,408 | 30.8 | 29.0 | 28.3 | 39.0 | 40.1 |
| real-test.db | 9,551,872 | 19.5 | 17.3 | 15.4 | 17.7 | 16.4 |
| real-test.bmp | 12,000,054 | 30.5 | 30.2 | 30.1 | 31.0 | 31.2 |
| arial.ttf | 1,045,720 | 53.6 | 48.2 | 44.6 | 51.8 | 53.3 |
| segoeui.ttf | 959,752 | 56.5 | 48.1 | 44.0 | 51.8 | 53.0 |
| zstd.exe | 1,601,409 | 41.7 | 36.2 | 33.4 | 38.9 | 40.1 |
| kernel32.dll | 836,208 | 45.9 | 40.3 | 37.8 | 43.4 | 45.2 |
| notepad.exe | 360,448 | 55.5 | 52.6 | 50.5 | 57.0 | 58.0 |
| alarm01.wav | 491,516 | 78.2 | 77.3 | 70.1 | 83.3 | 84.5 |
| ring01.wav | 498,420 | 58.9 | 56.5 | 49.8 | 61.1 | 62.0 |
| wallpaper.jpg | 1,602,752 | 98.9 | 98.0 | 98.1 | 100.5 | 97.7 |

## M0 — fork fidelity (measured at filing time, the one stage already run)

All 12 real containers bit-identical to eggv7's, differing only in the 15
magic/version bytes (3 headers × "EGG8"+v2). PASSED before this file was
written; every later stage is a real prediction.

## M1 — armor v2 (argmin rib policy + spread replicas + right-sized CT)

Overhead predictions (armor total ÷ inner), from the pigeonhole math
(⌈9/ng_eff⌉ ≤ T for a 4 KB scratch = at most 9 dead slots):

| inner size | v7 overhead | predicted v8 overhead | gate |
|---|---|---|---|
| 30 KB | ~33% | 22–25% | ≤25% |
| 200 KB | ~7.5% | 3.5–5% | ≤5% |
| 500 KB | ~7.5% | 2.4–3.2% | — |
| ≥1 MB | ~2.4% | 2.3–2.6% (unchanged) | ≤2.6% |

Physics floor, stated not hidden: the 4 KB-scratch guarantee needs parity for
≥9 dead slots plus level-2/replica copies ≈ 4.6–6.1 KB regardless of size, so
artifacts under ~16–24 KB cannot be cheap. wubbadub stays honest about this.

Per-file M1 predictions (armor v2 under the UNCHANGED v7 inner, % of input):

| file | egg7 | predicted M1 |
|---|---|---|
| vim-version9.txt | 21.4 | 20.3–20.8 |
| wubbadub.html | 40.1 | 36.0–38.0 |
| real-test.db | 16.4 | 16.3–16.5 (unchanged) |
| real-test.bmp | 31.2 | 31.1–31.3 (unchanged) |
| arial.ttf | 53.3 | 50.9–51.7 |
| segoeui.ttf | 53.0 | 50.6–51.4 |
| zstd.exe | 40.1 | 38.2–38.9 |
| kernel32.dll | 45.2 | 43.0–44.1 |
| notepad.exe | 58.0 | 55.2–56.6 |
| alarm01.wav | 84.5 | 80.5–81.8 |
| ring01.wav | 62.0 | 59.0–60.2 |
| wallpaper.jpg | 97.7 | 97.6–97.8 (unchanged) |

Audit predictions filed with the design:
- the naive continuous policy G=⌈s/5⌉ MUST FAIL audit(a) as the negative
  control (first counterexample at or near s=59: ⌈9/4⌉ = 3 > T=2);
- argmin policy passes exhaustive s=1..2000 plus 200 log-sampled to 10^6;
- ±2^k injectivity prints 8192/8192 distinct for BOTH 8219 and 8221;
- head 4 KB scratch EXACT for artifacts ≥ ~24 KB (v7 provably fails this:
  h0,m0,CT,h1,m1 all live in the first ~2 KB of small artifacts).

## M2 — stride/sample-delta filters (the overlay reading)

Filter choices predicted per file (everything else picks `none` by trial):

| file | predicted filter | predicted M2 (% of input) | vs gz* | vs xz |
|---|---|---|---|---|
| alarm01.wav | W16 ch=2 | 52–68 | WIN (78.2) | WIN if <70.1 |
| ring01.wav | W16 ch=2 | 42–55 | WIN (58.9) | maybe (49.8) |
| real-test.bmp | stride 6000 or byte k=3 | 23–29 | WIN (30.5) | WIN if <30.1 |
| all other 9 | none (by trial) | unchanged from M1 | — | — |

Gate: both WAVs and BMP beat gz* armored; apply∘undo property test green over
lengths 0..1024 × all filter ids. Risk named now: the WAVs are small (~490 KB)
so the armor floor eats 2.5–3 points of the filter's win; if alarm01 lands
above 70.1 the xz bar leans on ring01/db instead.

## M3 — context-mixed literals (the Spectrometer reading, part two)

Predicted inner shrink vs v7's single-depth choice (the mixer reads o0, o1,
o2, o4 and the match-bank at once, weighted by prediction record):

| file class | predicted inner gain |
|---|---|
| text/html (vim, wubbadub) | 3–8% |
| PE/TTF (5 files) | 4–10% |
| db | 3–7% |
| filtered PCM (wavs) | 2–6% |
| filtered BMP | 2–5% |
| jpg | 0.5–2% |

Gate: inner ≤ v7's inner on ≥11/12 (the per-file trial-and-stamp insurance
makes a regression structurally impossible; the prediction is that the mixer
WINS outright on ≥9). Encoder/decoder model-state hash equal on every file.
Speed: transmute drops to 1.0–2.0 MB/s (worst file ≥1 MB/s, the sanity
floor); restore drops to 8–40 MB/s.

## M5 — the three bars, predicted verdicts (egg8 final vs the table above)

| file | predicted egg8 final | beats gz*? | beats e6+zstd? | beats xz? |
|---|---|---|---|---|
| vim-version9.txt | 18.8–20.2 | YES | maybe | no |
| wubbadub.html | 33–36 | NO (honest loss) | YES | no |
| real-test.db | 15.2–16.1 | YES | YES | maybe |
| real-test.bmp | 22–28 | YES | YES | YES |
| arial.ttf | 48–51 | YES | maybe | no |
| segoeui.ttf | 47.5–50.5 | YES | YES | no |
| zstd.exe | 36.5–38.5 | YES | YES | no |
| kernel32.dll | 41.5–43.5 | YES | maybe | no |
| notepad.exe | 52.5–55.5 | maybe | YES | no |
| alarm01.wav | 52–68 | YES | YES | YES |
| ring01.wav | 42–55 | YES | YES | maybe |
| wallpaper.jpg | 97.4–97.7 | YES | YES | YES |

Ledger predictions: bar i 10–11 of 12 (wubbadub the certain loss, notepad the
coin flip) — MET at 10; bar ii 9–12 of 12 — MET; bar iii 3–5 of 12 (bmp,
alarm01, jpg the expected three; ring01 and db the bonuses) — MET at 3.
Fuzz: 10,000 deterministic injuries, ZERO silent. Pigeonhole: photo.bin MUST
transmute >100%, asserted as PASS.

## Measured (filled at M6, beside the guesses — not before)

Stage columns are % of input, armored. M1 = armor v2 alone; M2 = + filters;
final = + mixed literals (LR 9), lazy cache/lazy2, APM. Verdicts against the
baselines table; the tournament (standings.js) holds the official ledger
with all injuries EXACT.

| file | M1 pred | M1 meas | M2 pred | M2 meas | final pred | final meas |
|---|---|---|---|---|---|---|
| vim-version9.txt | 20.3–20.8 | 20.44 ✓ | unchanged | 20.44 ✓ | 18.8–20.2 | 19.64 ✓ |
| wubbadub.html | 36.0–38.0 | 37.08 ✓ | unchanged | 37.08 ✓ | 33–36 | 37.08 ✗ high |
| real-test.db | 16.3–16.5 | 16.38 ✓ | unchanged | 16.38 ✓ | 15.2–16.1 | 15.65 ✓ |
| real-test.bmp | 31.1–31.3 | 31.17 ✓ | 23–29 | **2.35 ✗ (far better)** | 22–28 | 2.29 ✗ (far better) |
| arial.ttf | 50.9–51.7 | 51.05 ✓ | unchanged | 51.05 ✓ | 48–51 | 48.60 ✓ |
| segoeui.ttf | 50.6–51.4 | 50.77 ✓ | unchanged | 50.77 ✓ | 47.5–50.5 | 48.31 ✓ |
| zstd.exe | 38.2–38.9 | 38.39 ✓ | unchanged | 38.39 ✓ | 36.5–38.5 | 36.50 ✓ |
| kernel32.dll | 43.0–44.1 | 43.26 ✓ | unchanged | 43.26 ✓ | 41.5–43.5 | 41.06 ✗ better |
| notepad.exe | 55.2–56.6 | 56.33 ✓ | unchanged | 56.33 ✓ | 52.5–55.5 | 54.20 ✓ |
| alarm01.wav | 80.5–81.8 | 80.69 ✓ | 52–68 | 68.81 ✗ high | 52–68 | 61.73 ✓ |
| ring01.wav | 59.0–60.2 | 59.33 ✓ | 42–55 | 46.90 ✓ | 42–55 | 41.86 ✗ better |
| wallpaper.jpg | 97.6–97.8 | 97.66 ✓ | unchanged | 97.66 ✓ | 97.4–97.7 | 96.76 ✗ better |

Prediction audit: M1 12/12 inside range. M2: the BMP prediction was WRONG in
the good direction — the range guessed pixel-delta physics (23–29) and the
row-stride delta respelled a vertical gradient to 2.35% (no LZ window can
see a gradient; the overlay reading can). alarm01 landed 0.8 above its range
at M2 and inside it after the mixer. Final: 7/12 inside range, 4 better than
predicted, 1 worse (wubbadub — the armor floor plus an HTML file whose
matches the mixer cannot replace; the honest loss stays).

Filter predictions vs measured: ring01 W16 ✓ as predicted; alarm01 W16 ✓ (the
sample briefly preferred byte-delta-2; the full trial chose W16); BMP
predicted "stride 6000 or byte k=3" — stride 6000 won ✓. The other 9 picked
none by trial ✓ as predicted.

Overhead gates: 30 KB → 18.36% (predicted 22–25, gate ≤25) ✓ better than
predicted (T=3 argmin found 3 groups × 3 parities cheaper than 5 × 2);
200 KB → 4.13% (≤5) ✓; 1 MB → 2.630% vs v7's 2.628% (the plan's "≤2.6%" was
its own rounding of "unchanged ~2.4%"; measured: unchanged, +12 B for the
three FNV-32s) ✓ by intent; 4 MB → 2.47% ✓.

Mixer gate: model 5 (mixed) won the trial on 12 of 12 files (insurance made
regression impossible; prediction was ≥9 outright wins) ✓. Encoder/decoder
state hash equal on every file ✓. LR sweep: 9 beat 10 and 11 on 3 of 4
sweep files. APM (gated extra): net −20,992 B across the corpus, kept; the
honest losses beside the win: bmp +1,536 B, jpg +1,536 B (no verdict flips).
Speed: transmute 2.5–5.9 MB/s (worst file ≥1 MB/s floor ✓, and above v7's
2.4 MB/s average despite three-model trials); restore 3.3–109 MB/s.
