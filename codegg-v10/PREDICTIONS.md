# eggv10 predictions — filed 2026-09-01, at v10-M0, BEFORE any workstream was built

House rule: predictions first, measurements after, unflattering rows kept.
THE BAR (unchanged from v9, unfinished, not repriced): lighter than xz -9's
undamaged weight on **≥8 of 12** real files, armor ON, all injuries EXACT.
v9 landed 6/12 twice. Decisions this round (Vladimir, 2026-09-01): speed floor
relaxed to 0.25 MB/s (all arms always run, no pruning); font contexts in as a
gated stretch; v9 entrants retired from the trial (measured vs-v9 gate + a
contingent MIX9 fallback replace the by-construction guarantee).

The charter (glossary.js:164): "Kept rather than rounded away, so the identity
holds as written and nothing is approximate."

## Baselines (egg9 = the measured floor v10 is gated against; % of input)

| file | orig B | xz -9 | egg9 | gap to xz |
|---|---|---|---|---|
| vim-version9.txt | 2,035,039 | 18.2 | 18.03 | WIN, protect |
| wubbadub.html | 92,408 | 28.3 | 36.51 | armor floor, out of scope |
| real-test.db | 9,551,872 | 15.4 | 14.78 | WIN, protect |
| real-test.bmp | 12,000,054 | 30.1 | 2.27 | WIN, protect |
| arial.ttf | 1,045,720 | 44.6 | 46.79 | −2.19 |
| segoeui.ttf | 959,752 | 44.0 | 46.44 | −2.44 |
| zstd.exe | 1,601,409 | 33.4 | 35.25 | −1.85 |
| kernel32.dll | 836,208 | 37.8 | 39.04 | −1.24 |
| notepad.exe | 360,448 | 50.5 | 52.49 | −1.99 |
| alarm01.wav | 491,516 | 70.1 | 56.41 | WIN, protect |
| ring01.wav | 498,420 | 49.8 | 30.36 | WIN, protect |
| wallpaper.jpg | 1,602,752 | 98.1 | 96.76 | WIN, protect |

## M0 — fork fidelity (measured at filing time, the one stage already run)

MIX10/CM10 inner streams bit-identical to MIX9/CM9 on 12/12 (containers differ
only in magic/version/model-byte/header-FNV — 24 bytes across 3 sites); every
.egg9 AND .egg8 container restores EXACT through eggv10; drills and audit
green. Everything below is a real prediction.

## Calibration (stated before the numbers)

v9's levers that LANDED in range were new information channels (match model,
sparse skip-grams 0.4–1.2, dual-rate twins 0.3–0.5, CM9); its 10× misses were
deeper-same-kind (M2 orders) and transforms (BCJ/TTF). States and ICM are new
channels (recency shape; indirection); ISSE is a new combination structure;
DP/token work is deeper-same and pre-shrunk accordingly.

## Per-stage predictions (points of armored %, per file class)

- **M1 (state infra + o1/o2 conversion, twins o1f/o2f dropped):** net
  −0.05..+0.20 everywhere (small: o1/o2 are the least sparse tables; this
  milestone is the infrastructure proof, not the earner). Mirror 12/12.
- **M2 (o3/o4/o6/sp13/sp24 → states; o4f dropped; twins A/B):** binaries
  0.10–0.45; text/db 0.05–0.25; audio residue 0–0.10. Twins-removal A/B:
  predicted net-neutral ±0.05 (removal kept).
- **M3 (ICMs ind1+ind2):** PEs 0.10–0.50 (the operand-alternation channel);
  TTFs 0.10–0.45; db 0.05–0.25; text 0.05–0.30; audio/bmp/jpg ~0. Gate: each
  PE row ≥0.10 or the miss is filed with keep/revert on net bytes.
- **M4 (ISSE chain, gated):** binaries 0.15–0.60; TTFs 0.10–0.50; text/db
  0.05–0.30. Risk stated: chain replaces flat inputs — if the A/B says the
  flat mixer was already extracting the same signal, this lands ~0 and is
  deleted (predicted probability of deletion: ~30%).
- **M5 (run model, gated):** db/PE/TTF 0.03–0.15; ~0 elsewhere.
- **M6 (sweeps + Tok10 package item-wise + DP replacing replay):** DP: PEs
  0.10–0.30, TTFs 0.05–0.20, db 0.05–0.15 — kill criterion: <0.08 avg across
  gated losers → revert to replay, print the miss. Tok10 package: PEs
  0.05–0.15, sign-flip risk on small files, item-wise keeps.
- **M7 (font contexts, gated stretch):** TTFs 0.2–0.8; zero effect elsewhere
  by construction (non-TTF paths bit-identical).

## The bar arithmetic, filed plainly

| file | needed | combined mid projection | flip call |
|---|---|---|---|
| kernel32.dll | 1.24 | ~1.3 | YES at mid — the likeliest flip |
| zstd.exe | 1.85 | ~1.3 | needs mid-to-optimistic — coin flip |
| notepad.exe | 1.99 | ~1.2 | unlikely without optimistic tail |
| arial.ttf | 2.19 | ~1.6 (with fonts) | coin flip |
| segoeui.ttf | 2.44 | ~1.6 (with fonts) | unlikely, upside |

**Ledger prediction: 7/12 likely (kernel32 flips); THE BAR (8/12) is a genuine
coin flip — zstd.exe or arial must land mid-to-optimistic.** Predicted final
verdicts: 7 wins called (v9's six + kernel32), zstd.exe called NARROW MISS,
arial called narrow miss, notepad/segoeui called misses, wubbadub the armor
floor. If the bar misses a third time, the miss is printed beside what moved,
and the residual gaps name the next levers (checksummed ICM slots, glyf-parsed
contexts, true SSE-chain depth).

Speed predictions (floor 0.25): worst small files 0.40–0.70 MB/s (u8 cache +
3-concurrency at M6 vs ~+15% model cost), db 0.30–0.50. Restore on v10-model
files 0.5–8 MB/s. Pigeonhole: photo.bin MUST transmute >100%, asserted PASS.
Zero silent anywhere. v7/v8/v9 untouched and green at M8.

## Measured (filled at M8, beside the guesses — not before)

Armored % per stage. M1 = o1/o2 states; M2 = all tables states; M3 = ICMs;
M4 = ISSE chain; M5 = run model (REVERTED, 0 net); M6 = LR/LIMIT sweep
(11/1023); M7 = font contexts (REVERTED, ±512 B noise).

| file | egg9 | M1 | M2 | M3 | M4 | M6 | final | vs xz |
|---|---|---|---|---|---|---|---|---|
| vim-version9.txt | 18.03 | 17.65 | 16.95 | 16.92 | 16.97 | **16.04** | 16.04 | **WIN** (by 2.16) |
| wubbadub.html | 36.51 | 34.24 | 33.68 | 33.68 | **32.54** | 33.68 | 33.68 | loss (armor floor; the sweep cost it 1.1 — corpus ruled) |
| real-test.db | 14.78 | 14.73 | 14.66 | 14.62 | 14.62 | **13.34** | 13.34 | **WIN** (by 2.06) |
| real-test.bmp | 2.27 | 2.27 | 2.23 | 2.23 | 2.23 | 2.25 | 2.25 | **WIN** |
| arial.ttf | 46.79 | 46.69 | 46.10 | 45.86 | 45.71 | **45.22** | 45.22 | loss by 0.62 (called coin-flip YES — MISSED) |
| segoeui.ttf | 46.44 | 46.28 | 45.91 | 45.75 | 45.70 | **45.27** | 45.27 | loss by 1.27 (called miss ✓) |
| zstd.exe | 35.25 | 35.13 | 34.01 | 33.56 | 33.40 | **32.73** | 32.73 | **WIN** (called coin flip — HIT) |
| kernel32.dll | 39.04 | 38.67 | 36.89 | 36.59 | 36.53 | **35.98** | 35.98 | **WIN** (called YES ✓) |
| notepad.exe | 52.49 | 52.63 | 51.64 | 51.35 | 51.35 | **51.21** | 51.21 | loss by 0.71 (called unlikely ✓) |
| alarm01.wav | 56.41 | 56.31 | 55.89 | 55.89 | 56.00 | **55.79** | 55.79 | **WIN** |
| ring01.wav | 30.36 | 30.36 | 30.36 | 30.25 | 30.25 | **29.84** | 29.84 | **WIN** |
| wallpaper.jpg | 96.76 | — | — | — | — | — | 96.76 | **WIN** (v8's model still holds the file) |

**THE BAR: 8 of 12 vs xz -9 — MET** (pending the tournament's live-byte
countersign). Predicted 7 likely / 8 coin-flip; landed 8.

Stage audit, honestly:
- **M1 landed ABOVE its (deliberately timid) range** — wubbadub alone took
  2.27 points from o1/o2 states. One regression filed (notepad +0.14, the
  retired twins), recovered at M2 as predicted.
- **M2 was the earthquake: 2–4× ABOVE range.** kernel32 −1.78 in one stage
  (predicted 0.10–0.45). After v9 taught us to shrink transform predictions
  10×, v10 teaches the mirror lesson: NEW information channels through
  state machines were UNDER-predicted by the same caution. Calibration cuts
  both ways; both misses are now on the record.
- **M3 ICMs: squarely in range** (PEs 0.28–0.45 vs 0.10–0.50 predicted).
- **M4 ISSE: kept at the low end** (net −5,144 B; predicted 30% chance of
  deletion — survived).
- **M5 run model: exactly 0 B net — REVERTED.** LZ eats runs before
  literals see them; mm predicts the rest. Predicted 0.03–0.15; measured 0.
- **M6 sweep: the second earthquake, unpredicted.** LR 9→11 and LIMIT
  255→1023 were worth 0.3–1.3 per row (db alone −1.28; CM10 overtook the
  LZ model on a DATABASE). The deeper model wanted slower learning; no one
  filed that guess. Costs printed: wubbadub +1.14 vs its M4 best, bmp
  +0.02 — the corpus ruled.
- **M7 font contexts: ±512 B (armor-quantum noise) — REVERTED.** The third
  time explicit structure knowledge lost to the model that learns it from
  the bytes (BCJ outgrown at v10-M2, TTF transform outgrown at v9,
  now even the region LABEL redundant). Controls were exactly 0 —
  the gate worked.
- **Tok10 package and the DP: NOT BUILT**, reasons filed: their calibrated
  ceilings (0.15 / 0.30) sit below every remaining gap (arial 0.62,
  notepad 0.71, segoeui 1.27); the replay arm they would replace still
  earns on db/kernel32. The v9-states precedent says a skipped lever can
  surprise when finally built — that argument is on the record FOR a
  future v11, not against this skip.

Speed (floor 0.25): worst transmute 0.31 MB/s (alarm01), corpus 0.31–0.71;
restore 0.6–29.6 (v10-model rows 0.6–1.7). v10 beat v9 on 11 of 12 rows
(jpg unchanged — v8's entrant still wins it, the trial floor working).
