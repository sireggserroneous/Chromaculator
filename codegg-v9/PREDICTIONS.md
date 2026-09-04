# eggv9 predictions — filed 2026-09-01, at v9-M0, BEFORE any workstream was built

House rule: predictions first, measurements after, unflattering rows kept.
THE BAR (Vladimir, 2026-09-01): lighter than xz -9's undamaged weight on
**≥8 of 12** real files, armor ON, all injuries EXACT. All v8 bars are kept
by construction (v8's encoders remain frozen trial entrants). Speed floor
0.5 MB/s worst-file transmute; weight first.

The charter, two voices: "the leading cell is never wasted" (spec.md:152)
and Vladimir's colleague: "Every nib counts."

## Baselines (measured; % of input; egg8 = the frozen floor v9 cannot fall below)

| file | orig B | xz -9 | zstd -19 | gz* | egg8 |
|---|---|---|---|---|---|
| vim-version9.txt | 2,035,039 | 18.2 | 18.7 | 24.2 | 19.64 |
| wubbadub.html | 92,408 | 28.3 | 29.0 | 30.8 | 37.08 |
| real-test.db | 9,551,872 | 15.4 | 17.3 | 19.5 | 15.65 |
| real-test.bmp | 12,000,054 | 30.1 | 30.2 | 30.5 | 2.29 |
| arial.ttf | 1,045,720 | 44.6 | 48.2 | 53.6 | 48.60 |
| segoeui.ttf | 959,752 | 44.0 | 48.1 | 56.5 | 48.31 |
| zstd.exe | 1,601,409 | 33.4 | 36.2 | 41.7 | 36.50 |
| kernel32.dll | 836,208 | 37.8 | 40.3 | 45.9 | 41.06 |
| notepad.exe | 360,448 | 50.5 | 52.6 | 55.5 | 54.20 |
| alarm01.wav | 491,516 | 70.1 | 77.3 | 78.2 | 61.73 |
| ring01.wav | 498,420 | 49.8 | 56.5 | 58.9 | 41.86 |
| wallpaper.jpg | 1,602,752 | 98.1 | 98.0 | 98.9 | 96.76 |

## M0 — fork fidelity (measured at filing time, the one stage already run)

All 12 real containers bit-identical to eggv8's modulo the magic/version and
the header FNV-32 that covers them; every .egg8 container restores EXACT
through eggv9. PASSED before this file was written; everything below is a
real prediction.

## M1 — match model + widened mixer selection (MODEL_MIX9 live)

The remainder reading: predict literal bits as the continuation of the
longest previous occurrence (ht 2^22, order-6 key, 2-byte verify), plus
weight vectors keyed by (phase×node)×match-state×after-match×o1-top3.

| file class | predicted M1 gain (points of % of input, armored) |
|---|---|
| text (vim, wubbadub) | 0.4–0.8 |
| db | 0.2–0.6 |
| TTF | 0.4–0.9 |
| PE ×3 | 0.3–0.7 |
| wav (filtered) | 0.3–0.7 |
| jpg | 0.1–0.3 |
| bmp | ~0 (nothing recurs in a gradient's residue) |

Gate predictions: state-hash mirror equal on 12/12; match-model hit-rate
highest on vim/db, near zero on bmp/jpg; worst transmute ≥0.5 MB/s.

## M2 — orders o3/o6 + APM stage 2 + LR sweep {8,9,10}

| file class | predicted M2 gain (on top of M1) |
|---|---|
| text | 0.5–1.0 |
| TTF | 0.7–1.4 |
| PE | 0.6–1.1 |
| db | 0.3–0.6 |
| wav | 0.2–0.5 |
| jpg | 0.2–0.5 |

Prediction: LR stays 9 or drops to 8 (bigger model, more inputs → smaller
steps win); memory ≤2 concurrent big models, measured <256 MB.

## M3 — CM9 literal-only entrant (LZ dropped, match model carries repeats)

Prediction: CM9 wins outright on vim (0–1.5 further points) and possibly
the wavs (0–0.8); loses to MIX9 on wubbadub (<100 KB warm-up) and on
LZ-heavy db; costs nothing anywhere (trial floor).

## M4 — WS-B shape filters (W16O2 id 8, W16BE id 9, autocorr 16384+harmonics)

| file | prediction |
|---|---|
| alarm01.wav | W16O2 beats W16 by trial; lands 54.5–59.0 |
| ring01.wav | order-1 vs order-2 is a real trial; lands 35.6–39.8 |
| TTFs | W16BE survives pruning but likely loses the full trial to none or to id 11 later; 0–1.0 |
| db | autocorr harmonics ~0 on this corpus (priced honestly) |

## M5 — WS-S structure tier (BCJ id 10, TTF-segmented id 11, DB-page id 12)

| file | prediction |
|---|---|
| zstd.exe | BCJ + M1/M2 model → 30.5–33.0 (xz 33.4: WIN) |
| kernel32.dll | 34.5–37.5 (xz 37.8: WIN, narrow) |
| notepad.exe | 48.5–51.5 (xz 50.5: coin flip) |
| arial.ttf | id 11 (loca delta + hmtx W16BE) + model → 43.5–46.5 (xz 44.6: coin flip, likely WIN) |
| segoeui.ttf | 43.2–46.2 (xz 44.0: coin flip) |
| real-test.db | id 12 page-stride: ~0–0.3 (probably picks none; priced honestly) |

Fuzz gate: 2,000 mutants per real file per structure id, zero panics, zero
apply∘undo mismatches. The invertibility law: undo re-derives its segments
from bytes apply left verbatim.

## M6 — gated extras (one at a time, net wins only)

- Bit-history states: corpus-net 0.15–0.6 on literal-heavy files; genuine
  risk of a net miss → then reverted and filed.
- Price replay + MAX_CHAIN 1024: wubbadub 0.3–0.8, db/PE 0.2–0.5, vim
  0.1–0.3; DP only if stats still show headroom after replay.

## M7 — the bar, predicted verdicts (egg9 final, armored, all injuries EXACT)

| file | predicted egg9 final | xz -9 | beats xz? |
|---|---|---|---|
| vim-version9.txt | 17.4–18.6 | 18.2 | YES (likely) |
| wubbadub.html | 34.3–36.1 | 28.3 | NO — honest loss (armor floor ~5 KB on ~26 KB inner) |
| real-test.db | 13.9–14.9 | 15.4 | YES |
| real-test.bmp | 2.2–2.3 | 30.1 | YES |
| arial.ttf | 43.5–46.5 | 44.6 | coin flip, called YES |
| segoeui.ttf | 43.2–46.2 | 44.0 | coin flip, called NO (segoeui is the tighter font) |
| zstd.exe | 30.5–33.0 | 33.4 | YES |
| kernel32.dll | 34.5–37.5 | 37.8 | YES (narrow) |
| notepad.exe | 48.5–51.5 | 50.5 | coin flip, called YES (narrow) |
| alarm01.wav | 54.5–59.0 | 70.1 | YES |
| ring01.wav | 35.6–39.8 | 49.8 | YES |
| wallpaper.jpg | 95.6–96.5 | 98.1 | YES |

**Ledger prediction: 9 of 12 vs xz (8 called wins + notepad; segoeui and
wubbadub the called losses, arial the shakiest yes) — THE BAR (≥8) MET.**
v8 bars re-verified: vs gz* 11–12/12, vs hybrid 12/12. Pigeonhole: photo.bin
MUST transmute >100%, asserted as PASS. Zero silent in all fuzz.

Speed predictions: transmute worst file 0.6–0.95 MB/s (db through two big
models), corpus average 1.5–3 MB/s; restore 1–2.5 MB/s on files landing on
models 6/7, unchanged (3.3–109) on files that keep v8 models.

## Measured (filled at M7, beside the guesses — not before)

Armored % of input per stage (— = row unchanged at that stage):

| file | egg8 | M1 | M2 | M3 | M4 | M5 | M6 | xz -9 | vs xz |
|---|---|---|---|---|---|---|---|---|---|
| vim-version9.txt | 19.64 | 19.46 | 19.39 | **18.13** (CM9) | — | — | 18.13 | 18.2 | **WIN** |
| wubbadub.html | 37.08 | — | — | — | — | — | 37.08 | 28.3 | loss (called) |
| real-test.db | 15.65 | 15.34 | 15.26 | — | — | — | **15.22** | 15.4 | **WIN** |
| real-test.bmp | 2.29 | — | — | — | — | — | 2.29 | 30.1 | **WIN** |
| arial.ttf | 48.60 | 48.31 | 48.21 | — | — | 47.92 (id 11) | 47.92 | 44.6 | loss (called WIN) |
| segoeui.ttf | 48.31 | 47.99 | 47.88 | — | — | 47.46 (id 11) | 47.46 | 44.0 | loss (called) |
| zstd.exe | 36.50 | 36.34 | 36.25 | — | — | 35.93 (BCJ) | 35.93 | 33.4 | loss (called WIN) |
| kernel32.dll | 41.06 | 40.94 | 40.88 | — | — | 40.02 (BCJ) | **39.96** | 37.8 | loss (called WIN) |
| notepad.exe | 54.20 | — | 54.05 | — | — | 53.48 (BCJ) | 53.48 | 50.5 | loss (called WIN) |
| alarm01.wav | 61.73 | — | 61.62 | 61.52 (CM9) | **59.12** (W16O2) | — | 59.12 | 70.1 | **WIN** |
| ring01.wav | 41.86 | — | 41.56 | 41.15 (CM9) | **32.21** (W16O2) | — | 32.21 | 49.8 | **WIN** |
| wallpaper.jpg | 96.76 | — | — | — | — | — | 96.76 | 98.1 | **WIN** |

**THE BAR: 6 of 12 vs xz -9 — MISSED (needed 8; predicted 9).** The miss is
printed, not repriced. Where the prediction went wrong, stage by stage:

- **M1 (match model): under range on binaries** (kernel32 0.12 vs 0.3–0.7
  predicted; zstd.exe 0.16 vs 0.3–0.7); in range on db/TTFs; the wavs took
  0 because the trial kept v8's model on the filtered forms until M2.
- **M2 (o3/o6 + APM2): far under range everywhere** (PE 0.06–0.15 vs
  0.6–1.1 predicted; text 0.07 vs 0.5–1.0). The M1 match model had already
  captured most of what deeper orders could see. This is the stage whose
  optimism carried the 8/12 projection.
- **M3 (CM9): in range** — vim 1.26 pts (0–1.5), the third xz flip.
- **M4 (W16O2): in range and better** — ring01 32.21 vs 35.6–39.8
  predicted; alarm01 59.12 vs the "lands ≤59" gate: missed by 0.12, filed.
- **M5 (structure tier): the big miss.** BCJ measured 0.32–0.86 pts vs 2–4
  predicted; TTF segmented 0.29–0.43 vs 1–3 predicted. Both transforms won
  their trials on every relevant file and both survive fuzz (24,000
  mutants) — the transforms are real, the projected magnitude was not. The
  residual gap to xz on PEs/TTFs (2.2–3.5 pts) is literal-modeling depth,
  not transform absence.
- **M6: price replay kept** (db −3,596 B, kernel32 −512 B, nothing worse —
  but ~0.04 pts vs 0.2–0.8 predicted). **Alignment context (pb bits): net
  LOSS +14,336 B over 7 files — REVERTED**, the miss filed. **Bit-history
  states: not built** — predicted ≤0.6 pts and no losing row within 2.1 pts
  of flipping; the risk budget had gone to the structure tier. **jcc BCJ:
  not buildable as specified** — the plan's "E8/E9/0F8x" conflated xz's
  plain x86 filter (E8/E9 only, which id 10 is) with BCJ2, whose
  four-stream layout cannot ride a length-preserving in-place filter.

What v9 DID deliver against its other standing bars: never lost to v8 on
any file (construction); beat v8 outright on 9 of 12; the two biggest
levers were CM9 on text (−1.5 pts vim) and W16O2 on audio (−9.7 pts ring01,
−2.6 alarm01). Speed and the tournament ledger land in the README.

## THE TRY-AGAIN ROUND (Vladimir: "Try again", 2026-09-01 — unplanned
stages, predictions made per-lever in-flight, measured immediately)

| lever | prediction | measured | verdict |
|---|---|---|---|
| sparse skip-grams (b−1,b−3)+(b−2,b−4) | 0.3–1.0 on binaries | −0.4 to −1.2 on ALL 8 measured rows | KEPT — the round's workhorse |
| token-side Tok9 (wider len/dist/align ctx) | 0.1–0.4 | net −500 B (sign flips row to row) | KEPT, marginal |
| sparse-LZ arm (min-match 16) | 0.3–1.5 on PEs | 0 on PEs/TTFs, −3.5 KB on ring01, broke the floor | NARROWED to audio arms |
| byte-alignment ctx (pb bits) | 0.3–1.0 on binaries | net +14,336 B (LOSS) | REVERTED, filed |
| dual-rate counter twins (o1/o2/o4 at RATE 3) | 0.2–0.6 | −0.3 to −0.5 broadly | KEPT |
| LR re-sweep at 14 inputs | maybe 8 or 10 | 9 held (3 of 4 files) | unchanged |

Final after the round: vim 18.03, wubbadub 36.51 (first movement since
v8), db 14.78, bmp 2.27, arial 46.79, segoeui 46.44 (both TTFs DROPPED
their id 11 filter — the model outgrew the transform), zstd.exe 35.25,
kernel32 39.04, notepad 52.49, alarm 56.41, ring 30.36, jpg 96.76.
**Still 6 of 12 vs xz** — the round cut every losing gap by 1–2.4 points
and flipped none; the misses that remain (1.2–2.4 pts) are modeling depth.
Cost, printed: the 0.5 MB/s floor broke on four small files (0.34–0.45).
v9 now beats v8 on 12 of 12 and beats zstd -19's undamaged weight on 11
of 12.
