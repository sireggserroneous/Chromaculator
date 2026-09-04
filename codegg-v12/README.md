# codegg-v12 — the Remainder

eggv12 is still the Transmuter. We transmute data into another form and
restore it; the first law is conservation, and the FNV-64 of the original
bytes gates every restore. The charter verse is the spec's own division
(spec.md:134–156): "**Division — quotient, multiplier, remainder… R is a stalk
too, always inside… Widening the grid grows Q and shrinks R. The identity
never moves.**" The armored form IS that identity in GF(2^16)[x]:

    A · x^t = Q · B + R        the file is A, the generator is B, the armor is R

The transmuted form is A·x^t − R, the nearest multiple of B; nothing is
dropped. A wound leaves a nonzero remainder and the remainder names the wound.
v11 divided by a SMALL B many times (groups ≤ 248 over GF(256)) and paid R per
group; v12 divides ONCE by a wide B and R is a constant.

**The reframe (Vladimir, 2026-09-02): "We are transmutating not compression."**
The v11 bars were a compressor's bars. The transmuter's currencies are (1)
conservation — every injury EXACT, wrong data never, refuse with a number;
(2) the price of each power printed beside its floor — surviving the loss of
any 4 KB of yourself costs ≥ 4,096 B for anyone, a price tag, not a wall;
(3) the form's weight measured form-vs-form (our inner against the rival's
stream) and armored-vs-armored against rivals that bought the same power.
Armored-vs-naked is printed as an honesty exhibit; it is never a bar.
Predictions were filed first, in `PREDICTIONS.md`, before every piece of code
that could move them; the misses are printed there beside the wins.

## THE BARS — in the currencies, verdicts first

| bar | needed | landed | verdict |
|---|---|---|---|
| 1. Conservation | 3 injuries × 20 rows + the formats card EXACT, certutil-countersigned; scattered-wound drills EXACT; P+1 refuses | **60/60** on the ledger and **9/9** on the formats card, every one EXACT; certutil **8/8 FINGERPRINT MATCH**; drills **257/257**; audit v4 **3,091,667 checks, 0 failing**; wrong data 0 in every run of the campaign | **MET** |
| 2. The price is floored | total = inner + price TO THE BYTE on every row; price printed beside 4,096 | **20/20** at M2b and **20/20** on the final ledger (the prediction of the TOTAL missed on 6 rows — the model moved, the arithmetic never did) | **MET**; price flat per tier: 4,812 / 5,324 / 6,348 (1.17× / 1.30× / 1.55× the floor) |
| 3. Form vs form (inner vs naked xz -9's stream) | ≥ 17 of 18 measured rows | **19 of 20**. The save is the one printed loss: 17,328,521 vs 16,739,920 (+588,601, 3.5%) — the debt is the gzip skin, diagnosed and filed for v13 | **MET** |
| 4. Armored vs armored (xz+par2, rar -rr5) | 23 of 23 | rar -rr5 **forfeits truncation on 23 of 23** (structural, as in v11); xz+par2 survives everything and loses to egg12 on **22 of 23** | **MISSED BY ONE ROW** — the game save, −0.20 pt (v11's margin on the same row: −0.53) |
| 5. The ratchet | v12 ≤ min(v8..v11) on every row | **20/20** at M1, M2a, M2b and on the final ledger (14/14 printed by the home tournament). The internal M2c gate — no row heavier than M2b — broke on ring01.wav by **31 bytes**, inside that lever's own filed band and printed | **MET** (the 31 B noted) |
| 6. Exhibit, not a bar: armored egg12 vs NAKED xz -9 | — | **17 of 20** lighter (losses: the save +593,925, wubbadub.html +1,441, iconcache48.db +1,355 — each carrying 4,812 B of armor xz does not carry); 12/14 on the home card, 3/3 on the formats card | printed, not chased |

Net against the sealed v11 across 20 rows: **-8,987,397 B** — five times v11's whole
campaign (−1,766,332), with zero regressions and every injury EXACT.

## The failures and reverts, first — the house ledger

- **Two agents died mid-milestone on the org's API spend limit** (HTTP 429):
  the M1 agent at 16:15 while closing (the main session read the gate from
  disk and wrote the M1 close), the M2b agent at ~19:00 after building M2b
  and before its ledger (the main session filed the big-row predictions and
  closed M2b at 23:03). Cost: two closes written from disk evidence rather
  than by the builder, and the third agent's brief begins with "checkpoint
  every result to disk". Everything in `PREDICTIONS.md` names its evidence.
- **The toolchain drifted mid-campaign**: rustc/clippy 1.98.0 installed at
  15:40 on 2026-09-02. Four new lints in codegg-v12 fixed behaviour-neutral
  (eggs byte-identical before and after); codegg-v11 itself fails two of them
  under 1.98 and is frozen, printed, not touched.
- **The M2a bound undercounted by 1.5–3.5×, on every row.** The probe measured
  PINNED decisions only (the mixer at the ±2047 clamp) and the filed capture
  range was 50–80% of that bound; every row landed at 148–350% of it, because
  v11's fixed-rate 12-bit APM could not hold a bucket above 3,969/4,096 and
  taxed EVERY confident decision, pinned or not. cbs.log's CM arm had been
  paying ~470 KB of that tax and losing the row to the LZ arm by 4×. A bound
  measured on one mechanism undercounts when the tax is systemic. Printed as
  the miss it is — a miss that saved 6.7 MB.
- **Two drill FAILs at M2b** (255 of 257): the rank-trap drill did not force
  `--ct none`; at `--tier 2048` on a 100-square identity file the argmin
  rightly prefers the triplicated table. The drill was wrong, the armor was
  right; fixed in tools/drills.js, 257/257.
- **The M0 arithmetic had a circular term** (the parity squares' residues
  inside the in-codeword table that the parity protects) and the plan kept
  the short square mid-stream where the audit found the wound bound breaks.
  Both fixed before M1 code; the totals moved +18..+102 B and the audit grew
  the case. **Three M1 misses**, all lighter: the byte-exact price let a
  different arm win the armored-total trial on cbs, kernel32, bmp.
- **M2c(a), the save: the reach hypothesis was REFUTED.** Called −150..−400 KB
  from a wider LZ hash and a lifted chain budget; measured −6,558 B (0.04%).
  A 2^24 hash finds the SAME tokens as 2^20, bucket for bucket, 7.4× faster.
  The debt is not reach: the save is one gzip member (see the exhibits).
- **M2c(c), the dialect books: every range missed BELOW.** Called −0.2..−1.5%
  on the five PE/TTF rows; measured −0.04..−0.27%. The arms win their rows and
  are kept, thinly: a book of the dialect's siblings speaks PE, but a model
  learns a 300 KB file's dialect on its own within its first tens of KB.
- **M2c(d), the line/column context, DROPPED**: its target vanished when the
  16-bit pipeline flipped cbs.log by 52% (67,017 against xz's 139,004).
- **M3(1), the stereo member shipped UNREADABLE for eleven minutes.** The
  header verifier refused filter ids above 12 (a literal in armor.rs, not the
  filter table's maximum); alarm01.wav's mid/side artifact failed validation
  at all three sites — refused honestly, never wrong, but a transmute wrote
  what no restore could read, the big-arena class again. Found by the ledger,
  not by a test. Fixed: ONE constant (`filter::FILTER_MAX`) read by both; the
  write-time in-memory round-trip law now covers EVERY filtered form (it
  covered only length-changing filters and ≥ 64 MB inputs); a pipeline test
  drives a synthetic stereo WAVE through both new ids and the free trial.
  And the call had the two wavs backwards: ring (side 3× quieter) gained
  nothing, alarm gained 4.48%.
- **M3(2), the JPEG peel: the round trip HIT, the size call MISSED.** The
  scan re-encodes byte-exactly (the door is open); the peeled coefficient
  stream under xz -9 is 1.368× the JPEG's own entropy coding and under our
  MIX12 arm 1.379× — a raw coefficient dump is not a form. Only a coefficient
  model (packJPG/Lepton) walks through that door; v13's, with the round trip
  proved.
- The cwd trap bit the formats-card launch (a backgrounded `&&` chain took
  the `cd` with it); the exe-lock trap was dodged by copying every build
  before running it. Filed. Again.

## What v12 is — the readings made mechanism

1. **The remainder** (spec.md:134–156): armor v4 is ONE systematic
   Reed–Solomon codeword over GF(2^16) (poly 0x1100B, BCH view, g(x) =
   ∏(x − αⁱ)); the square of `blk` bytes is blk/2 symbols and symbol j of
   every square is codeword j. `t = ceil(4096/blk) + 1` parity squares
   (`--survive` dials it); the price of surviving any 4 KB is FLAT per tier
   and independent of the file: 4,812 (256-B squares), 5,324 (512), 6,348
   (1024) — 1.17× / 1.30× / 1.55× the pigeonhole floor, where v11 paid
   1.8×–106×. The searched grid (spec.md:166) picks the tier by argmin of
   the printed total. Berlekamp–Massey locates, Forney values, Chien
   searches; every step refuses with a number past capacity.
2. **The columns agree** (wubbadub.html:698 — "the sum is computed ON the
   array, column by column"): a wounded square is an error at the SAME
   position in every one of the blk/2 interleaved codewords, so their
   syndromes share one locator (Krachkovsky–Lee 1997; Bleichenbacher–
   Kiayias–Yung 2003). The 2 B/square residue table that v11 (and M1) paid
   to LOCATE wounds became redundant: placement "none" carries t+1 parity
   squares and no table (rdr2's price 47,446 → 6,348). Blind wounds ≤ P−1
   squares are located jointly, named wounds ≤ P are certain, P blind
   refuses; the hash-gated rung makes "all parity dead" EXACT. Residue
   placements stay selectable (`--ct triple|incw`, `--judge`).
3. **Kept rather than rounded away** (glossary.js:164) — the 16-bit pipeline
   (M2a, mix12.rs): v11's whole model path was twelve bits (coder p in
   1..4095, mixer clamp ±2047, APM entries at a fixed 1/128 step that could
   not hold a bucket above 0.969). Everything is kept to sixteen: a u64
   coder (Mahoney's zpaq, Knoll's cmix), stretch ±4095 with exact logits,
   count-adaptive 16-bit APM entries, the learning shifts widened by four so
   the dynamics stay where v11's sweep put them. It moved 6.7 MB across the
   20 rows and took wallpaper.jpg from v8's frozen arm and cbs.log from the
   LZ arm. New MODEL bytes 16–21; the v11 arms (10–15) stay frozen entrants
   and write byte-identical eggs under `EGG_NO_V12`.
4. **Two voices on one pitch** (chroma-ui.js:569) — checksummed slots (M2c(b),
   Mahoney's lpaq/paq8 HashTable): every hashed context bucket carries a
   check byte at its free index 0 (the nibble tree uses nodes 1..15), the
   probe is 2-way, a double miss reclaims the less-experienced neighbour.
   −57,652 B over the 14 home rows; the dense binaries moved 1.2–2.2%.
5. **Free to guess, in another dialect** (glossary.js:104) — the PE and TTF
   books (M2c(c)): `gen-prior --book` trains the site-prior's shape on
   28 System32 DLLs / 51 Windows fonts that sit in NO corpus of this repo
   (the arial and segoe families excluded whole; gen-prior refuses a book
   that names a test row). Trial arms MODEL 22/23, sniffed by magic; they
   win their five rows by 0.04–0.27%.
6. **Two silhouettes** (spectrometer.html:602) — stereo mid/side (M3): the
   lifting form side = L − R, mid = R + (side ≫ 1), exactly invertible in
   wrapping i16, then the per-channel order-1 or order-2 delta (filters 13
   and 14, WAVE-sniffed at channels == 2). alarm01.wav −4.48%.
7. **The promise with a number** (spectrometer.html:396 — refusing with a
   number): `info` and every transmute print "survives any P squares named
   / P−1 blind (bytes), contiguous ≥ 4,096; price P B (floor 4,096; ×)".

Format EG12 v6 (`.egg12`); eggv12 restores `.egg11`, `.egg10`, `.egg9` and
`.egg8` EXACT through `src/armor11.rs` (v11's armor v3, verbatim, never edited).

## The price vs the floor

| tier (square) | P parity squares | price = parity + 3 sites | × 4,096 | rows |
|---|---|---|---|---|
| 256 B | 18 | 4,608 + 204 = **4,812** | 1.17× | every inner ≤ 16,772,352 B (17 of 20) |
| 512 B | 10 | 5,120 + 204 = **5,324** | 1.30× | aoe4-autosave.sav |
| 1024 B | 6 | 6,144 + 204 = **6,348** | 1.55× | rdr2-shaders.vkcache, rustc_driver.dll |
| 2048 B | 4 | 8,192 + 204 = 8,396 | 2.05× | none (reached only past 65,535 squares at 1024) |

v11 paid 7,497 (wubbadub, 1.83×) to 432,848 (rustc, 106×). The price no
longer grows with the file; at 183 MB it is 6,348 B.

## The ledger (20 rows, the final exe)

| row | orig | **egg12 total** | inner (arm) | price | × floor | vs v11 sealed | naked xz -9 | injuries |
|---|---|---|---|---|---|---|---|---|
| wubbadub.html | 92,408 | **27,621** | 22,809 (CM12P) | 4,812 | 1.17× | -2,975 | 26,180 → **+1,441** | E/E/E |
| cbs.log | 16,187,036 | **71,758** | 66,946 (CM12) | 4,812 | 1.17× | -79,046 | 139,004 → win | E/E/E |
| ring01.wav | 498,420 | **135,565** | 130,753 (CM12) | 4,812 | 1.17× | -10,619 | 248,424 → win | E/E/E |
| notepad.exe | 360,448 | **176,164** | 171,352 (CM12·PE) | 4,812 | 1.17× | -6,896 | 181,884 → win | E/E/E |
| alarm01.wav | 491,516 | **252,862** | 248,050 (CM12) | 4,812 | 1.17× | -20,334 | 344,640 → win | E/E/E |
| real-test.bmp | 12,000,054 | **261,274** | 256,462 (MIX12) | 4,812 | 1.17× | -7,314 | 3,612,796 → win | E/E/E |
| vim-version9.txt | 2,035,039 | **273,982** | 269,170 (CM12H) | 4,812 | 1.17× | -45,282 | 371,088 → win | E/E/E |
| kernel32.dll | 836,208 | **283,604** | 278,792 (CM12·PE) | 4,812 | 1.17× | -17,228 | 316,416 → win | E/E/E |
| segoeui.ttf | 959,752 | **409,683** | 404,871 (CM12·TTF) | 4,812 | 1.17× | -19,685 | 422,304 → win | E/E/E |
| iconcache48.db | 97,517,568 | **418,323** | 413,511 (MIX12) | 4,812 | 1.17× | -6,437 | 416,968 → **+1,355** | E/E/E |
| arial.ttf | 1,045,720 | **446,354** | 441,542 (CM12·TTF) | 4,812 | 1.17× | -21,938 | 466,716 → win | E/E/E |
| zstd.exe | 1,601,409 | **488,915** | 484,103 (CM12·PE) | 4,812 | 1.17× | -32,625 | 534,572 → win | E/E/E |
| real-test.db | 9,551,872 | **1,068,149** | 1,063,337 (CM12H) | 4,812 | 1.17× | -173,227 | 1,474,592 → win | E/E/E |
| wallpaper.jpg | 1,602,752 | **1,513,903** | 1,509,091 (CM12) | 4,812 | 1.17× | -19,325 | 1,571,824 → win | E/E/E |
| msgraph.dll | 43,249,696 | **3,211,880** | 3,207,068 (CM12·PE) | 4,812 | 1.17× | -1,405,780 | 4,749,376 → win | E/E/E |
| mermaid-bundle.js | 25,842,004 | **4,090,958** | 4,086,146 (CM12) | 4,812 | 1.17× | -800,122 | 4,952,132 → win | E/E/E |
| ntoskrnl.exe | 13,047,280 | **4,675,387** | 4,670,575 (CM12·PE) | 4,812 | 1.17× | -364,185 | 5,501,588 → win | E/E/E |
| aoe4-autosave.sav | 66,417,543 | **17,333,845** | 17,328,521 (MIX12) | 5,324 | 1.30× | -219,995 | 16,739,920 → **+593,925** | E/E/E |
| rustc_driver.dll | 183,111,168 | **37,436,978** | 37,430,630 (CM12·PE) | 6,348 | 1.55× | -5,282,974 | 41,707,380 → win | E/E/E |
| rdr2-shaders.vkcache | 48,872,878 | **41,860,990** | 41,854,642 (MIX12) | 6,348 | 1.55× | -451,410 | 41,969,640 → win | E/E/E |
| **20 rows** | | | | | | **-8,987,397** | 17 of 20 lighter | **60/60 EXACT** |

Every total is `inner + price` to the byte. Six of the twenty missed their
FILED total (five lighter, rdr2-shaders +107,616 heavier) — the calls are in
`PREDICTIONS.md` with their causes. Podium: egg12 takes **14 of 14** home rows
and **3 of 3** formats rows; the egg6+zstd hybrid loses cbs.log, the last row it
held since v6.

## The exhibits

- **The save is one gzip member.** aoe4-autosave.sav (66,417,543 B) opens
  `1f 8b 08` and inflates to 296,540,843 B of Relic Chunky at 4.83 bits/byte.
  Its deflate stream is 7.998 bits/byte, and xz cuts it to 25% only because
  the compressed bytes repeat over long ranges. Our form on the deflate stream
  is 17,363,293 (xz -9: 16,739,920 — the card's one form-vs-form loss). Our
  MIX12 arm on the PEELED bytes: **5,306,038 B**, lighter than xz -9 on the
  same peeled bytes (5,525,752) by 3.98% and 3.27× lighter than our own form
  on the skin. zlib 1.3.1 reproduces the stream at no level (first
  difference at byte 0), so a byte-exact re-deflate needs preflate-class
  reconstruction. Not built; v13's first line, with the numbers.
- **Stereo vs FLAC**: `flac` is not installed on this machine, so the FLAC -8
  column is not printed; against v11's sealed totals ring01.wav 146,184 →
  135,565 (−7.26%) and alarm01.wav 273,196 → 252,862 (−7.44%); against naked
  xz -9 (248,424 / 344,640) both are far ahead. The NLMS cross-channel
  predictor is a reading, not a filter: an adaptive predictor belongs in the
  model.
- **The JPEG peel** round-trips byte-exactly on wallpaper.jpg (432,000 blocks,
  a restart marker per MCU row, Adobe's 1-bit padding); the peeled
  coefficients are heavier under every generic coder we have (see the
  failures). The v9-states precedent still stands FOR a coefficient model.
- **The float-field filter** (wubdiv.html:213/217/221, fpzip/zfp lineage) and
  **the transpose** (wubx.html:394, Blosc's shuffle) are FILED AS READINGS,
  not built: the save turned out to be a gzip skin over its floats (no field
  filter reaches through deflate), and the db/shader rows moved under the
  16-bit pipeline and the checksummed slots instead — the two probes had no
  row left to be measured on honestly this campaign.
- **Armored egg12 vs NAKED xz -9** — the honesty exhibit, never a bar:
  **17 of 20** rows lighter than xz's undamaged stream while carrying 4,812 B
  of armor. The three losses: the save +593,925 (the gzip skin), wubbadub.html
  +1,441 (a 92 KB page, 1.17× the pigeonhole floor in armor alone) and
  iconcache48.db +1,355. Strip the armor and 19 of 20 are lighter.

## Speed (SOLO measurements only)

Every number here is a SOLO run: one transmute at a time, nothing else on the
machine. (The MB/s printed by the lane ledgers is contended and is not a speed
measurement; it is left in `PREDICTIONS.md` labelled as such.)

| what | bar | measured | verdict |
|---|---|---|---|
| worst home row | ≥ 0.25 MB/s | **ring01.wav 0.277 MB/s** (1.80 s for 498,420 B) | **MET** |
| the monster, end to end | ≤ 45 min | **rustc_driver.dll 40m56.9s** (183,111,168 B → 37,436,978 B) | **MET** (v11: 40m23s) |

The fourteen home rows, solo: wubbadub 0.398, ring01 **0.277**, notepad 0.302,
alarm01 0.307, kernel32 0.281, segoeui 0.380, arial 0.382, zstd 0.460, vim
0.499, real-test.db 0.389, real-test.bmp 0.580, wallpaper.jpg 0.382, cbs.log
1.621, iconcache48.db 1.412 MB/s.

Thirty-four seconds slower than v11 on the monster, carrying a roster that
gained six v12 arms and two dialect books, and handing back 5,282,974 B less.
The tokenizer lever from M2c(a) — a 2^24 hash that finds the same tokens 7.4×
faster — was never needed and stays on the shelf, measured, for v13.

## Run it

```bash
cd codegg-v12 && cargo build --release          # std only, offline
cargo test --release                            # 23 tests incl. the two pipeline laws
cargo clippy --all-targets -- -D warnings       # clean under rustc 1.98
target/release/eggv12 transmute <file> [--survive BYTES] [--tier BLK] [--parity T] [--ct triple|incw|none] [--judge]
target/release/eggv12 restore <f>.egg12|.egg11|.egg10|.egg9|.egg8 [--wound start:len]
target/release/eggv12 info <f>.egg12            # the promise with its number, price beside 4,096
target/release/eggv12 audit [--full]            # GF(2^16) self-test, ord_65519(2) walk, decode classes
target/release/eggv12 gen-prior <site files>    # the site book -> src/prior_tab.rs
target/release/eggv12 gen-prior --book PE --out src/prior_pe.rs <DLLs not in any corpus>
node tools/drills.js                            # the 257-drill battery
node tools/ledger12.js                          # the ledger; EGG_PRED=<json> judges every row to the byte
node tools/standings.js corpus-real/*           # the home tournament
node tools/challengers.js <files>               # the armored rivals (xz+par2, rar -rr5)
node tools/countersign-big.js                   # certutil fingerprints, big arena
```

## Attribution

Reed & Solomon 1960 (the code); Berlekamp–Massey (the locator); Chien (the
search); Forney (the values); Krachkovsky & Lee 1997 and Bleichenbacher,
Kiayias & Yung 2003 (collaborative decoding of interleaved RS — the columns
agree); Matt Mahoney's zpaq / lpaq / paq8 lineage (the 16-bit coder and
squash, StateMap, APM, the checksummed HashTable, the config lineage of
selectable models); Byron Knoll's cmix (the 16-bit coder); Avizienis 1971
and Mandelbaum 1976 (the residue-number lineage the u16 residues descend
from); Fowler–Noll–Vo (FNV, the conservation hash); Josh Coalson's FLAC and
Tony Robinson's Shorten (mid/side, the fixed predictors); ITU T.81 and
packJPG / Lepton (the peel probe); Igor Pavlov's LZMA (the token shapes);
Yutaka Sawada's MultiPar and Alexander Roshal's RAR (the challengers); XZ
Utils 5.8.3 (the rival's stream, exact bytes). The site supplied every
reading; the tournament the discipline; the misses supplied v13.
