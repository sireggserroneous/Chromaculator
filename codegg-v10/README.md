# codegg-v10 — the Keeper

eggv10 is still the Transmuter. The first law is conservation; the FNV-64 of
the original bytes gates every restore. The charter is the glossary's own
definition (glossary.js:164): "**Kept rather than rounded away, so the
identity holds as written and nothing is approximate.**" The thesis, bought
with v9's measurements: stop widening, start deepening — v9's counters knew
where they stood but not how they got there. v10 gave every context a memory
of its path, and the bar that had been missed twice fell.

## THE BAR: beat xz -9 on ≥8 of 12 real files. **MET: 8 of 12.**

v9 landed 6/12 twice. v10 flipped kernel32.dll (37.8 needed, landed
**35.98**) and zstd.exe (33.4 needed, landed **32.73**) and widened every
older win — armor ON, all 36 injuries EXACT, certutil-countersigned.

## The failures and reverts, first

- **arial.ttf missed by 0.62** (45.22 vs 44.6) — and it was called a
  coin-flip YES in the predictions. That call was wrong. notepad (−0.71)
  and segoeui (−1.27) were called misses and missed as called.
- **The font-context stretch (M7) measured ±512 B — armor-quantum noise —
  and was REVERTED.** The third structure lesson in two versions: the BCJ
  transform was outgrown by the state model at M2 (zstd.exe dropped it in
  trial), the TTF transform was outgrown at v9, and now even the parsed
  region LABEL is redundant to contexts that learn the regions from the
  bytes. Explicit structure knowledge keeps losing to models that read it
  implicitly. (The gate's controls were exactly zero — the machinery
  worked; the idea didn't.)
- **The run model (M5) measured exactly 0 B net and was REVERTED.** LZ eats
  byte-runs before literals see them; the match model predicts the rest.
- **The Tok10 token package and the optimal-parse DP were NOT BUILT**,
  reasons filed: calibrated ceilings (≤0.15, ≤0.30) below every remaining
  gap. The v9-states precedent — a skipped lever that later erupted — is on
  the record FOR a future v11, not against this skip.
- **The M6 sweep cost two files while winning ten**: wubbadub +1.14 vs its
  M4 best (33.68; LR 11 punishes the smallest file), bmp +0.02. The corpus
  ruled; both prints stand.
- **Predictions missed in BOTH directions this time.** After v9's 10×
  transform over-predictions, the same caution UNDER-predicted the state
  machines by 2–4× (M2 alone: kernel32 −1.78 vs 0.10–0.45 predicted) and
  nobody filed a guess for what the LR/LIMIT re-sweep would be worth
  (0.3–1.3 per row). Calibration cuts both ways; PREDICTIONS.md carries
  the full stage audit.
- **The speed floor is 0.25 MB/s by decision, and v8's old 0.5 is gone**:
  worst transmute 0.31 (alarm01), corpus 0.31–0.71; restore on v10-model
  rows 0.6–1.7 MB/s. Weight first, priced honestly.
- wubbadub.html remains the armor-floor loss vs gzip/xz — ~5 KB of parity
  physics on a ~26 KB payload, stated every version because it is still true.

## What the Keeper is

Three readings became the engine:

1. **The cycle reading → bit-history states** (index.html:220 — "Click a
   cell to cycle it green → blue → red"; index.html:730 — the landing
   page's own `(state, input) → next state` table). Every big context table
   (o1, o2, o3ʰ, o4ʰ, o6ʰ, both sparse skip-grams, both indirect models)
   holds a u8 STATE from a generated 255-state machine
   (tools/gen_states.py → src/state_tab.rs, byte-identical regeneration,
   construction documented in the generator; attribution: Mahoney's zpaq
   states, re-derived for this codebase's P(bit==0) — **bit 0 increments
   n0**). Two contexts at the same probability with different pasts are now
   different states — the ancestry reading (atlas.html:429: the site prints
   a value's ancestry BESIDE its value).
2. **The keeps-R reading → StateMaps** (spec.md:204-205 — "floating point
   rounds inside the band and forgets, and this keeps R"): per-(node ×
   state) maps with count-adaptive steps — early observations move them a
   lot, seasoned ones barely (honest note: the site ties step size to
   depth, not count; the formula is Mahoney's StateMap, flipped for P(0)).
   Swept: LIMIT 1023, mixer LR 11 — the deeper model wanted slower
   learning, worth 0.3–1.3 points per row by itself (db's CM10 overtook
   the LZ model on a *database*: 14.62 → 13.34).
3. **The dial reading → indirect contexts** (spectrometer.html:874 — the
   latitude dial: "Which rule decides…"): h1/h2 remember what last followed
   each byte/pair; the context selects the history, the history selects the
   probability. And **the settle-chain reading → ISSE** (spec.md:124
   squash-settle-push): the dense orders chain o1→o2→o3→o4→o6, each stage a
   2-weight learned mixer over the previous stage's prediction, selected by
   its own state (zpaq's ISSE, net −5,144 B, kept). One
   `Mix10::byte_update` is the only place per-byte state moves — the
   mirror is correct by construction, and EGG_STATEHASH proved it 12/12 at
   every milestone.

Format EG10 v4 (.egg10); eggv10 restores .egg9 and .egg8 containers EXACT.
The trial: v8's trio (frozen forever) + MIX10 + CM10 (+ the audio sparse-LZ
arm + the ≥512 KB replay arm) + a contingent MIX9 that runs only if both
v10 arms lose to the v8 pick (it never fired). v10 beat v9 on 11 of 12
rows; wallpaper.jpg still belongs to v8's entrant — the trial floor doing
its job. Armor v2 untouched: the audit re-proves it, 62.5M checks.

## The table (armored % of input, all injuries EXACT)

| file | orig B | xz -9 | egg8 | egg9 | **egg10** | vs xz |
|---|---|---|---|---|---|---|
| vim-version9.txt | 2,035,039 | 18.2 | 19.64 | 18.03 | **16.04** | **WIN** by 2.16 |
| wubbadub.html | 92,408 | 28.3 | 37.08 | 36.51 | **33.68** | loss (armor floor) |
| real-test.db | 9,551,872 | 15.4 | 15.65 | 14.78 | **13.34** | **WIN** by 2.06 |
| real-test.bmp | 12,000,054 | 30.1 | 2.29 | 2.27 | **2.25** | **WIN** |
| arial.ttf | 1,045,720 | 44.6 | 48.60 | 46.79 | **45.22** | loss by 0.62 |
| segoeui.ttf | 959,752 | 44.0 | 48.31 | 46.44 | **45.27** | loss by 1.27 |
| zstd.exe | 1,601,409 | 33.4 | 36.50 | 35.25 | **32.73** | **WIN** — the eighth flip |
| kernel32.dll | 836,208 | 37.8 | 41.06 | 39.04 | **35.98** | **WIN** — the seventh flip |
| notepad.exe | 360,448 | 50.5 | 54.20 | 52.49 | **51.21** | loss by 0.71 |
| alarm01.wav | 491,516 | 70.1 | 61.73 | 56.41 | **55.79** | **WIN** |
| ring01.wav | 498,420 | 49.8 | 41.86 | 30.36 | **29.84** | **WIN** |
| wallpaper.jpg | 1,602,752 | 98.1 | 96.76 | 96.76 | **96.76** | **WIN** |

**The tournament** (blind 1-byte flip, addressed 4 KB scratch, 4 KB
truncation — same injuries for everyone): podium **egg10 × 11, egg8 × 1**
(wallpaper.jpg). Every raw compressor forfeited every file; brotli returned
wrong data with a success code on 6 of 12. All 36 egg10 injuries restored
EXACT, countersigned by certutil SHA-256: ALL FINGERPRINTS MATCH.

**The victory ledger, live bytes:**

| bar | needed | got | |
|---|---|---|---|
| **THE BAR: vs xz -9** | **≥8 of 12** | **8 of 12** | **MET** |
| held: vs strongest gzip -9 | ≥10 (v8's bar) | 11 of 12 | MET |
| held: vs egg6+zstd hybrid | ≥6 (v8's bar) | 12 of 12 | MET |
| unasked but measured: vs zstd -19's undamaged weight | — | 11 of 12 | — |

Synthetic regression: podium egg10 × 8 (egg9 keeps one row, egg6 keeps
pure-random photo.bin); hybrid bar 9/10 held; vs gzip 8/10; both
pre-compressed members transmute >100% — the pigeonhole, kept, as always.

## Attribution

Everything v8/v9 attributed, plus: Matt Mahoney's zpaq — the bit-history
state construction (re-derived, documented, and polarity-flipped in
tools/gen_states.py), StateMaps, and ISSE; the paq lineage's indirect
models. The site supplied the geometry: the cycle, the ancestry, the
keeps-R, the dial, the settle chain — and the charter about keeping.

## The big arena (same day, after the victory — BIG-ARENA.md)

The 12-file bar was met at a median file size of ~1 MB. The same day,
Vladimir sent the Keeper to scale: 8 real files, 13 MB to 183 MB
(corpus-big/). The full campaign — predictions, the bug, the fix, both
tournaments, the armored rivals — lives in **BIG-ARENA.md**. The short form,
failures first, as always:

- **THE SLOT WALL:** the LZ distance-slot decoders were guarded at slot 25
  (dist < 2^26). The unbounded finder emitted farther matches on >64 MB
  files; the artifact was WRITTEN HEALTHY AND UNRESTORABLE. Carried by
  v7-v10; found by the arena's own injuries; fixed the same day (guards to
  the 5-bit tree's true ceiling of 31; bit-identical below 2^26 — the
  12-file table re-countersigned byte-for-byte in
  standings-real-postfix.txt). New law: any transmute >= 2^26 B round-trips
  IN MEMORY before the artifact is written. A regression test drives
  distances to u32::MAX through all three tiers. v7/v8/v9 keep the wall,
  frozen; the fixed eggv10 restores even THEIR wounded big artifacts.
- **vs naked xz -9 at scale: 2/8** — the 6/8 call missed badly and the audit
  is in BIG-ARENA.md. Three losses by a combined <0.1 pt (the armor tax,
  not the model); two honest modeling defeats (game-save records, shader
  ISA); the 183 MB monster by 0.9.
- **vs ARMORED rivals: 18-2.** rar's tail recovery record died to
  truncation 20/20; xz+par2 (the archivist's posture, MultiPar) survives
  everything and still loses to egg10 on 18 of 20 rows across both arenas —
  every naked photo-finish loss flips once xz pays for a shield.
- Podium at scale: egg10 x5, egg9 x2 (the retired entrant's two
  tenth-point wins — the fallback crack, named for v11), hybrid x1.
- The serial tournament cost ~12 h; the parallel challengers harness did its
  card in ~6 min on 24 cores. The chain cliff (superlinear tokenize on big
  dense files) is named and measured.

The rematch is chartered with full mandate:
`~/.claude/plans/rematch-of-the-heavyweights-v11.md`.

## Run it

```bash
cd codegg-v10 && cargo build --release      # std only, offline
cargo test --release                        # 9 tests: polarity, state-table invariants, filters, structure fuzz
target/release/eggv10 transmute <file>      # .egg10; filters+models auto
target/release/eggv10 restore <f>.egg10     # conservation-gated (.egg9/.egg8 too)
target/release/eggv10 audit [--full]        # the geometry audit, counts printed
node tools/drills.js                        # black-box battery (75 drills)
node tools/standings.js corpus-real/*       # the tournament + the ≥8/12 ledger
node tools/verify.js corpus-real/*          # certutil countersign
```
