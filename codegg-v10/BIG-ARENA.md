# The Big Arena — eggv10 at scale. Predictions filed 2026-09-01, BEFORE any run.

Vladimir, 2026-09-01: "We need to test this against bigger files. So we can
see where the players in the arena land. So far we fought over little files."
True: the 12-file arena spans 92 KB - 12 MB, median under 1.3 MB. Small files
are where armor overhead and cold models hurt US. Big files are where xz's
64 MB dictionary and zstd's long-range matcher stretch THEIR legs. Nothing
about the 8/12 bar transfers automatically; this arena finds out what does.

House rules unchanged: same three injuries for everyone, wrong-or-no data
forfeits, predictions first, misses printed beside wins.

## The eight (all real files from this machine, copied as found)

| file | bytes | class |
|---|---|---|
| cbs.log | 16,187,036 | Windows servicing log (repetitive text) |
| mermaid-bundle.js | 25,842,004 | bundled JS (code text) |
| ntoskrnl.exe | 13,047,280 | native kernel PE |
| msgraph.dll | 43,249,696 | .NET IL PE (Microsoft.Graph) |
| rustc_driver.dll | 183,111,168 | the monster: native PE, 2.8x xz's dictionary |
| iconcache48.db | 97,517,568 | Explorer icon cache DB (bitmaps in a container) |
| rdr2-shaders.vkcache | 48,872,878 | Vulkan pipeline cache (compiled shader blob) |
| aoe4-autosave.sav | 66,417,543 | game save (expected: internally compressed) |

Honest absences: this machine holds no big real WAV/BMP/JPG that wasn't
already synthetic or covered by the small arena; the media classes sit out.

## The structural theses (stated before the numbers)

1. **The window thesis**: our LZ chain (`prev` per position, dist u32) spans
   the WHOLE file; xz -9 stops at 64 MB. On rustc_driver.dll (183 MB) that
   difference is structural, not statistical.
2. **The warm-model thesis**: v10's state machines and StateMaps pay a cold
   start that a 92 KB file never amortizes and a 90 MB file fully does. Our
   armored % should IMPROVE with size on every modelable class.
3. **The saturation risk, filed against ourselves**: the match-model ht is
   2^22 slots keyed order-6. At 183 MB that is ~44 positions per slot —
   collision pressure the small arena never applied. If the monster row
   disappoints, THIS is the named suspect (and the v11 lever: bigger or
   checksummed ht).
4. **The armor tax at scale**: parities+replicas+CT cost roughly a constant
   fraction; on incompressible payloads (the .sav, likely the .vkcache) the
   raw compressors sit at ~100% and we sit above it by the tax. The
   pigeonhole is a law, not an embarrassment — those rows are called LOSSES
   vs xz's weight now, in writing, while every raw row still forfeits the
   injuries.

## Predictions vs xz -9 (armored % of input; egg10 wins iff lighter)

| file | call | predicted margin (pts of %) | confidence |
|---|---|---|---|
| cbs.log | **WIN** | 0.5 - 2.5 | high (vim won by 2.16 at 2 MB; logs repeat harder) |
| mermaid-bundle.js | **WIN** | 1.0 - 3.0 | high (code text is our best class) |
| ntoskrnl.exe | **WIN** | 1.0 - 3.0 | high (kernel32 won by 1.8 at 836 KB, warmer here) |
| msgraph.dll | **WIN** | 1.5 - 4.0 | high (IL+metadata is dense structure; CM feasts) |
| rustc_driver.dll | **WIN** | 0.5 - 2.5 | moderate (window thesis vs saturation risk) |
| iconcache48.db | **WIN** | 1.0 - 4.0 | moderate (db won by 2.06; bitmap payload may let filters fire) |
| rdr2-shaders.vkcache | coin flip | -2.0 - +1.5 | none claimed (unknown internal compression) |
| aoe4-autosave.sav | **LOSS** | armor tax 1.5 - 3.5 above xz | high (if the save is NOT internally compressed, this flips to a big win — both branches filed) |

**Ledger call: 6 of 8 vs xz -9 (range 5-7).**
Podium call: egg10 x7, egg6 x1 (the .sav goes to the lightest armor-only row,
the photo.bin precedent). vs strongest gzip: 7 of 8 (32 KB window collapses at
scale; only the .sav resists). vs hybrid: 7-8 of 8. Brotli predicted to return
wrong data with a success code again on at least one injured row (it did on 6
of 12 small files); the disqualification prints itself.

## Speed and physics predictions

- Floor 0.25 MB/s HOLDS on all 8; predicted transmute speeds 0.35-0.90 MB/s
  (big files amortize what small files cannot).
- The monster: ~5-10 min transmute, restores 0.8-2 MB/s.
- Full tournament wall time: 2-4 hours (each row = 4 compressors + 5 egg
  generations + 30 injury-restores).
- Memory: peak < 3 GB per arm (183 MB src + ~1.5 GB chains + 81 MB model).
- Pigeonhole: any internally-compressed member transmutes >100% or near it;
  asserted loudly, never silently.
- All 24 injuries (8 files x 3) restore EXACT or the row forfeits, ours
  included. No wrong data with a success code, ever.

## What the first run found (the diagnostic run, 2026-09-01)

The first tournament never finished as a standings table — it finished as a
BUG REPORT, and that is worth more.

**THE SLOT WALL.** On iconcache48.db (97.5 MB) every armor-v2 generation —
egg7, egg8, egg9, egg10 — was disqualified at once, while egg6 (armor only,
no LZ) survived. The undamaged artifact would not restore either: the decoder
refused with "offset slot ran away", wrote nothing, pretended nothing. Anatomy:
the LZ distance is coded as a log2 slot through a 5-bit tree; the three
decoders (v8/v9/v10 tiers) guarded `slot > 25`, i.e. distances < 2^26 = 64 MB.
The ENCODERS had no such guard — the finder's window is deliberately unbounded,
so on a file past 64 MB it names matches from deeper than the wall, the tree
encodes slot 26 happily, and the artifact is written healthy and unrestorable.
Four generations carried this; no corpus under 64 MB could ever touch it.
aoe4-autosave.sav (66,417,543 B) passed sitting 691 KB BELOW the wall;
iconcache48.db failed far above it. Filed prediction #3 named the match-model
ht as the monster-row suspect — the right organ class (match machinery), the
wrong organ. Printed as a prediction miss.

**THE FIX (same day):** the three decoder guards raised to 31 — the 5-bit
tree's own ceiling (the encoders needed nothing); slot_hist widened 26→32.
Below 2^26 the format is untouched BIT-FOR-BIT. And a new law in transmute:
any input >= 2^26 B round-trips IN MEMORY through the full restore gate
before the artifact may be written — a transmute can never again claim
success on bytes no restore can read. Regression test drives distances
1..u32::MAX through all three tiers' codecs.

**The proof gates, all green before the official run:**
- cargo tests 10/10 (incl. the new dist_slots_round_trip_past_2_26)
- drills 75/75; audit 62,545,574 checks passing; pigeonhole PASS
- iconcache48.db under the fixed binary: in-memory round-trip verified at
  write; artifact 427,320 B (0.44%) — byte-identical to the pre-fix artifact
  (the encoder was never the broken half); flip/scratch/trunc all EXACT
- inertness below the wall: kernel32.dll 300,832 / vim 326,432 /
  wubbadub 31,120 / zstd.exe 524,100 — the books, to the byte
- the 12-file arena re-countersigned on the fixed binary: podium egg10 x11 +
  egg8 x1, THE BAR vs xz 8/12 MET, gz* 11/12, hybrid 12/12 — identical to
  the victory ledger (standings-real-postfix.txt)

**Ancestral note, filed honestly:** eggv7/eggv8/eggv9 keep the slot wall —
frozen by law. They cannot fight above 64 MB and their rows will show dq
there forever, weights kept beside the verdict. The fixed eggv10 restores
THEIR wounded artifacts through its repaired decoders, though they cannot.

**Diagnostic-run verdicts already convicted (raw weights recomputed with the
tournament's exact settings; egg10 sizes from the run/repro):**

| file | xz -9 | egg10 | verdict vs xz | the call |
|---|---|---|---|---|
| ntoskrnl.exe | 42.2% | 39.1% | **WIN +3.1** | called WIN 1.0-3.0 — top of range |
| msgraph.dll | 10.98% | 10.79% | **WIN +0.19** | called WIN 1.5-4.0 — direction right, margin 8x under |
| mermaid-bundle.js | 19.16% | 19.19% | LOSS by 7,216 B | called WIN — MISS (payload beat xz; armor tax flipped it) |
| iconcache48.db | 0.43% | 0.44% | LOSS by ~10 KB | called WIN — MISS (armor tax is the whole margin) |
| cbs.log | 0.86% | ~0.9% | LOSS by a hair | called WIN — MISS (photo finish) |
| aoe4-autosave.sav | 25.2% | 26.8% | LOSS −1.6 | called LOSS — right verdict, WRONG mechanism (the save is NOT internally compressed; xz simply out-fights us on game-state structs). egg9 beat egg10 by 0.1 here — the retired-entrant cost, filed for v11 (the MIX9 fallback trigger is too narrow) |
| rdr2-shaders.vkcache | (ledger) | 87.8% | near-entropy scrap fight | no call filed — egg9 edged egg10 by 0.1 again |
| rustc_driver.dll | — | — | VOID in diagnostic (the wall); real answer in the official run | — |

**The scale lesson, stated early:** on executable code the model beats xz
outright at any size (ntoskrnl +3.1 is the widest PE margin in series
history). Everywhere else the model fights xz to a photo finish and the
ARMOR TAX decides the verdict — three losses totaling under 0.1 points
combined. At big-file scale the fight is not model vs model; it is our
redundancy vs xz's nakedness. Halve the armor's constant drag and this
table reads 5-1. That reframes the 12/12 campaign: the armor tax is now a
first-class lever beside the modeling levers. Also: gzip is annihilated at
scale (25-85% vs our 0.4-27%) — the 32 KB window collapsing as filed; and
brotli returned wrong bytes with a success code on FIVE diagnostic rows.

## The official table (fixed binary, posterity format, standings-big.txt)

| file | orig B | gz* | zstd | brotli | xz -9 | e6+zstd | egg7 | egg8 | egg9 | **egg10** | PODIUM |
|---|---|---|---|---|---|---|---|---|---|---|---|
| aoe4-autosave.sav | 66,417,543 | 84.6(dq) | 26.5(dq) | 27.3 LIED | 25.2(dq) | 27.1 | 27.1 | 26.8 | **26.7** | 26.8 | egg9 |
| cbs.log | 16,187,036 | 2.5(dq) | 0.8(dq) | 1.0 LIED | 0.9(dq) | **0.8** | 1.0 | 0.9 | 0.9 | 0.9 | egg6+zstd |
| iconcache48.db | 97,517,568 | 26.6(dq) | 0.7(dq) | 0.7 LIED | 0.4(dq) | 0.8 | 0.5(dq) | 0.5(dq) | 0.4(dq) | **0.44** | **egg10** |
| mermaid-bundle.js | 25,842,004 | 24.4(dq) | 20.0(dq) | 21.6(dq) | 19.2(dq) | 20.5 | 21.2 | 20.6 | 20.4 | **19.2** | **egg10** |
| msgraph.dll | 43,249,696 | 22.0(dq) | 14.5(dq) | 15.3(dq) | 11.0(dq) | 14.8 | 15.8 | 14.1 | 12.4 | **10.8** | **egg10** |
| ntoskrnl.exe | 13,047,280 | 54.2(dq) | 44.7(dq) | 47.9 LIED | 42.2(dq) | 45.8 | 47.2 | 45.5 | 41.6 | **39.1** | **egg10** |
| rdr2-shaders.vkcache | 48,872,878 | 96.8(dq) | 90.1(dq) | 92.9 LIED | 85.9(dq) | 92.3 | 88.7 | 87.8 | **87.7** | 87.8 | egg9 |
| rustc_driver.dll | 183,111,168 | 35.3(dq) | 25.6(dq) | 27.8(dq) | 22.8(dq) | 26.3 | 26.9(dq) | 25.6(dq) | 24.1(dq) | **23.7** | **egg10** |

Podium: **egg10 x5, egg9 x2, egg6+zstd x1**. Ledger (egg10, armor ON, all
injuries EXACT): **vs naked xz -9: 2/8** (ntoskrnl +3.1, msgraph +0.19);
vs strongest gzip: 8/8; vs the hybrid: 7/8. Countersign: the tournament
byte-compared every restore EXACT; certutil SHA-256 spot-countersign on
cbs.log, ntoskrnl.exe, iconcache48.db (one file above the 2^26 law):
ALL FINGERPRINTS MATCH.

The monster's verdict, plainly: **egg10 held 183 MB at 23.7% with every
injury EXACT — the largest artifact in series history, ancestral wall printed
beside it (egg7/8/9 all dq: they compress it and cannot restore it)** — and
naked xz still won the weight by 0.9 (22.8), so the WIN call on this row
MISSED (called 0.5-2.5 WIN). The filed suspect stands: match-model ht
saturation (2^22 slots, ~44 positions each at this size) plus the chain
cliff; the v11 plan carries both by name. The armored rival xz+par2 also
edges the monster (23.4 vs 23.7) — with the save, the SECOND armored defeat
(18-2 across both arenas).

## The prediction audit, unflinching

Filed call: 6/8 vs naked xz (range 5-7). **Landed: 2/8.** The biggest
prediction miss in series history, and the anatomy is the finding:
- Three calls (cbs, iconcache, mermaid) lost by a COMBINED <0.1 points —
  the payloads beat or tied xz; the ARMOR TAX flipped every verdict. The
  model calls were right; the tax was unpriced. Calibration lesson filed:
  predictions must price the tax per row, not per arena.
- The save was called LOSS for the wrong mechanism (called "internally
  compressed"; actually compressible interleaved records that xz's parse
  reads better). Right verdict, wrong reason — filed as a miss of kind.
- The monster WIN call missed by 0.9 (above).
- ntoskrnl landed at the TOP of its called range (+3.1, called 1.0-3.0);
  msgraph landed in-range on direction, 8x under on margin.
- The speed floor held (worst transmute >= 0.31 MB/s) but the WALL-TIME was
  never predicted and became the story: the serial tournament took ~12h and
  the monster row alone ~7h (the chain cliff, found here, named in v11).

## What the big arena proved (the campaign's yield)

1. **THE SLOT WALL** — a >64 MB conservation bug in four shipped generations,
   found, fixed, regression-locked, and re-countersigned the same day. The
   >=2^26 in-memory round-trip law now makes the class impossible.
2. **THE ARMORED SWEEP** — vs xz+par2 (the only outside posture that survives
   the injuries): **egg10 18-2 across both arenas**; every naked photo-finish
   flips armored; rar's tail-medicine died 20/20 to truncation.
3. **THE TAX IS THE FIGHT** — on executable code egg10 beats naked xz at any
   scale; everywhere else the armor tax decides. v11's first workstream.
4. **THE CHAIN CLIFF** — tokenize time superlinear on big dense files; named,
   measured, v11's second workstream.
5. **THE NEAR-ENTROPY GAP** — game-save records and shader ISA: the one class
   that beats us armored AND naked; v11's lattice workstream.
6. **THE HIGHWAY** — 24 cores; the parallel challengers harness did in ~6 min
   what the serial tournament did in ~12h; v11 tournaments run in lanes.

Brotli returned wrong bytes with a success code on 6 of 12 small-arena rows
and lied on 5 big-arena rows in the diagnostic (6 in the official). The rule
that catches it — wrong-or-no data forfeits — remains the series' spine.

**The campaign closes with the bar NOT met on foreign soil (2/8), the home
title intact (8/12, re-countersigned post-fix), the armored crown won 18-2,
and four named levers handed to v11: the tax, the parser, the lattice, the
crack. The rematch is chartered: `.claude/plans/rematch-of-the-heavyweights-v11.md`.**
