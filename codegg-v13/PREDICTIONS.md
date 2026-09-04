# codegg-v13 PREDICTIONS -- THE VALUE UNDERNEATH

Predictions are FILED HERE BEFORE the code that could move them, and the misses
are printed beside the wins. Every number names its evidence.

## The charter verse

spec.html:359-360, the site's own tooltip for `form`:

> Plain is the digits as written; pushed respells the same value using -1s.
> **The value underneath does not change -- compare the two and only the
> colours differ.**

Siblings: spec.md:55 and stalk.js:61 ("the value never moves; only the colours
do"), spectrometer.html:238 ("the same value, respelled"), spectrometer.html:450
("value unchanged -- only the colours moved"), glossary.js:78 (Booth's recoding:
"the value never changes -- only the colours do").

## The thesis

A JPEG's Huffman bits and its DCT coefficients are the same value in two
spellings. On several rows v12 modelled somebody else's spelling -- the colours
-- and called the result our form. v13 reads the value underneath: peel the
foreign code, model what it actually says, re-spell it byte-exactly on the way
out. The peel is a bijection or it does not ship.

## THE LAW OF THE PEEL (a conservation law, non-negotiable)

1. A peel is a bijection or it is not used. At TRANSMUTE time every peel
   re-encodes its own output and compares against the original bytes; one byte
   off and the peel is discarded for that file and the raw bytes go through the
   ordinary pipeline. The decoder therefore only ever sees peels the encoder
   proved invertible on this exact input, before anything was written.
2. The recipe rides inside and is judged by argmin: `recipe + peeled values`
   against `raw form`, on the ARMORED total, exactly like the armor placements.
3. Refuse, do not guess. Progressive, arithmetic-coded, 12-bit, truncated,
   corrupt: keep the bytes and print the reason.
4. Every peel rides v12's write-time round-trip law (M3(1) shipped an unreadable
   artifact for eleven minutes from a far simpler filter).

## M0 FILED (2026-09-03 08:05, at the fork, BEFORE the M0 ledger ran)

The fork must be a rename and nothing else: crate `eggv13`, format **EG13 v7**,
extension `.egg13`; `restore` accepts `.egg13/.egg12/.egg11/.egg10/.egg9/.egg8`.
armor v4 did not move at v13, so `parse_header` reads EG13 v7 AND EG12 v6 down
the same path; `armor11.rs` is verbatim and still owns `.egg11` and older.

**Called: all TWENTY of v12's M4 totals reproduced TO THE BYTE**, and every
ancestor container restores EXACT. The totals judged (codegg-v12/PREDICTIONS.md,
"M4 MEASURED"): wubbadub.html 27,621; cbs.log 71,758; ring01.wav 135,565;
notepad.exe 176,164; alarm01.wav 252,862; real-test.bmp 261,274;
vim-version9.txt 273,982; kernel32.dll 283,604; segoeui.ttf 409,683;
iconcache48.db 418,323; arial.ttf 446,354; zstd.exe 488,915; real-test.db
1,068,149; wallpaper.jpg 1,513,903; msgraph.dll 3,211,880; mermaid-bundle.js
4,090,958; ntoskrnl.exe 4,675,387; aoe4-autosave.sav 17,333,845;
rustc_driver.dll 37,436,978; rdr2-shaders.vkcache 41,860,990.

## M0 MEASURED (2026-09-03 09:01) -- the fork reproduces v12 to the byte

`node tools/ledger13.js` with `EGG_PRED` = v12's twenty M4 totals and
`EGG_EXE` = a copy of the M0 build (the exe lock).

**20 of 20 rows HIT, to the byte. Prediction misses: 0. Injuries 60/60 EXACT
(E/E/E on every row). Failures: 0. Net vs the sealed v11: -8,987,397 B --
exactly v12's number, which is what a rename should produce.**

Every row's price is flat and unmoved: 4,812 B at the 256-B tier (17 rows),
5,324 at 512 (the save), 6,348 at 1024 (rdr2 and rustc), placement `CT none`
throughout. The winning model byte per row is v12's, unchanged.

Ancestor restore, checked separately before the ledger: containers written by
`eggv8`, `eggv9`, `eggv10`, `eggv11` and `eggv12` on three files each --
**15 of 15 restored EXACT through eggv13**. `.egg12` rides the SAME armor v4
path (parse_header accepts EG13 v7 and EG12 v6); `.egg11` and older ride
`armor11.rs`, verbatim.

The MB/s printed by that ledger is CONTENDED (eight lanes, and the JPEG suite
was running beside it) and is not a speed measurement.

## M1 FILED (2026-09-03 08:20, BEFORE ONE LINE of peel or coefficient-model code)

### The design being predicted

**WS-F, the peel frame.** A peel is a length-changing, structure-parsing filter
that emits `(recipe, values)`. It rides in the container as a new model byte
(`MODEL_PEEL`) whose payload opens with a peel preamble -- `peel_id: u8`, the
recipe's model byte and lengths, the values' model byte and length -- followed
by the separately-modelled recipe stream and the separately-modelled values
stream. `restore` re-spells `values + recipe -> the original bytes` and then the
FNV-64 gate decides. Peels are trial arms; `armored_total()` picks the winner,
so a peel that costs more than the raw form is simply not chosen.

**WS-J, the JPEG coefficient model.** The v12 python probe ported to Rust
(Huffman-decode every MCU, DC prediction reset at each RST, the file's own
DHT/DQT verbatim in the recipe, re-encode with the spec's 1-bit padding), then
the coefficients MODELLED -- which is the milestone, not the peel. Contexts:
the band index (DC / the first ACs / the tail), the same position in the block
above and to the left, the count of nonzeros already placed in this block, the
quantisation value at that position, and the component.
Attribution: ITU T.81 (the codec); packJPG (Matthias Stirner) and Lepton
(Dropbox) for the context shape.

### wallpaper.jpg -- the numbers called

Measured facts inherited from v12-M3(2) (not predictions): the file is
1,602,752 B; the entropy-coded bytes are **1,602,311**; baseline SOF0,
3840x2400, three components all 1x1 (4:4:4), DRI 480 (299 RSTn), APP14 Adobe,
**432,000 blocks**; the v12 probe round-trips it byte-exactly; the RAW
coefficient dump (64 x i16 per block, 55,296,000 B) is 2,192,684 under xz -9 and
2,208,961 under our MIX12 arm -- a dump is not a model, which is what M1 must fix.
v12's row: total **1,513,903**, inner 1,509,091 (arm CM12, MODEL 17); naked
xz -9 on the file: **1,571,824**.

| quantity | FILED | basis |
|---|---|---|
| round trip, Rust port, byte-exact on wallpaper.jpg | **YES** | the python probe already did it; the Rust port must reproduce it or the peel is refused |
| **the recipe's own bytes** (before its model) | **441 B** exactly -- every byte of the file that is not entropy-coded | 1,602,752 - 1,602,311 = 441 |
| the recipe stream after its own model + the peel preamble | **380 .. 560 B** | 441 B of DQT/DHT/SOF/DRI/APP14/SOS/EOI is nearly incompressible at that length; the preamble is ~16 B and the model may pay a few bytes of coder flush rather than save any |
| **the peeled values** (the coefficient model's stream) | **1,201,733 .. 1,361,964** = **-25% .. -15%** of 1,602,311 | packJPG and Lepton's published range on baseline photographs, pre-shrunk for having no per-band tuning of our own yet |
| **inner** (preamble + recipe stream + values stream) | **1,202,200 .. 1,362,500** | the two rows above |
| price (256-B tier, 18 parity + 3 sites) | **4,812 B**, flat | armor v4 is unmoved; the inner stays far under 16,772,352 B |
| **armored total** | **1,207,000 .. 1,367,300** | inner + 4,812 |
| vs v12's total 1,513,903 | **-146,600 .. -306,900** | |
| vs naked xz -9 1,571,824 (an exhibit, never a bar) | lighter by 204,500 .. 364,800 | |

**The gate, stated against the call, honestly.** Gate 1 asks that wallpaper.jpg
beat its v12 total by at least 10% of the entropy-coded bytes -- 160,231 B --
i.e. an armored total of **1,353,672 or less**, i.e. a values stream at or under
about 1,348,400 B, i.e. **-15.8% or better**. The filed range's LOW end (-15%,
total 1,367,300) therefore **misses the gate by roughly 13,600 B** and its high
end clears it by 146,700. The call is the published range, not the gate; if we
land at the bottom of our own range the miss is printed with that arithmetic.

### The rest of M1, called

| what | FILED |
|---|---|
| every OTHER row byte-identical to its v12 total | **19 of 19 HIT** -- the peel is nominated by the JPEG magic alone and no other corpus row is a JPEG; no trial arm, filter or armor constant moves |
| the 30+ JPEG suite | **every file either peels and re-spells BYTE-EXACT or is cleanly refused and keeps its bytes; LOST = 0**. Called by class: baseline 8-bit 4:4:4 and 4:2:0, with and without restart intervals, Adobe APP14 and JFIF-only, greyscale (one component) -- **peel, exact**. Progressive (SOF2), arithmetic-coded (SOF9/SOF10), 12-bit (SOF0 with P=12), truncated, corrupt DHT -- **refused, bytes kept, reason printed** |
| suite provenance | 46 JPEGs from this machine's `C:\Windows\Web` tree (Screen, Wallpaper/Alienware, ThemeA-D, Spotlight, 4K, touchkeyboard) plus 2 generated here with Windows' own WIC `JpegBitmapEncoder` (a 4:2:0 colour and a Gray8 greyscale) plus hand-built hostiles (truncations, a corrupt DHT, a 12-bit SOF, an arithmetic SOF). Of the 46, **8 are already progressive** (the touchkeyboard set) and two are byte-identical copies of the corpus row |
| injuries | **60/60 EXACT** on the 20-row ledger, peeled row included |
| drills | 257 v12 drills GREEN plus a new peel battery (truncated member, corrupt table, progressive input, a peel whose re-encode fails) -- **all keep the bytes, none lose a file** |
| audit | v4 unchanged: 3,091,667 checks under `--full`, 0 failing |
| clippy / cargo test | clean under rustc 1.98; the test count grows by the peel's own pipeline law |
| **speed, wallpaper.jpg SOLO** | **0.20 .. 0.35 MB/s** (v12: 0.382). The peel adds a Huffman decode, a coefficient-model encode, the LAW's re-encode-and-compare, and the write-time round trip (a full model decode plus a second re-encode). The home floor is 0.25 MB/s: this call **straddles it**, and if the row lands under 0.25 the number is printed with its cause rather than repriced |
| ancestors | ZERO files outside `target/` newer than codegg-v13's Cargo.toml, proven by mtime against the M0 manifest |

### What would make this call wrong

- The coefficient model is a LEAN one (one adaptive 16-bit probability per
  context, no mixer) chosen for the speed floor; packJPG's published range was
  reached with a comparable shape, but Lepton's upper end uses far more state.
  If the lean model lands under -15% the cause is the model's width, and that is
  M3's business, not M1's.
- The image is unusually cheap already: 1,602,311 B over 432,000 blocks is
  **29.7 bits per block**, so most blocks are a DC and an early EOB. A stream
  that already spends almost nothing per block has less for a model to take,
  and the published 15-25% was measured on ordinary photographs at ordinary
  rates. This is the single most likely reason for a low landing.
- 4:4:4 with a restart marker every MCU row resets the DC prediction 299 times;
  our 2D DC prediction (above and left) does not reset, which should HELP.

## FINDING (2026-09-03, main session, verifying the M1 gate) -- the blind promise overclaims

Independent verification of the M1 gate reproduced every number the milestone
reported (see the verification note below) and then asked a question the gate did
not: what does a BLIND wound actually cost at the 256-byte tier, which after v12's
M2b is the tier 17 of the 20 rows land on?

`info` prints, for every one of those containers, `guaranteed: true` and this
promise: "NAMED (by address or truncation): any 18 squares (4,608 B) anywhere,
CERTAIN; BLIND: any 9 squares CERTAIN (Berlekamp-Massey), up to 17 located
jointly ... contiguous >= 4,096 B blind."

Measured on two containers -- wallpaper.jpg (peeled, MODEL_PEEL) and notepad.exe
(no peel at all, the ordinary pipeline), both block 256, t 18, placement none,
`guaranteed: true`:

| wound | promised | measured |
|---|---|---|
| 8 fully-dead squares, blind | EXACT | **EXACT** |
| 9 fully-dead squares, blind | EXACT ("any 9 squares CERTAIN") | **REFUSED** |
| 10 .. 18 fully-dead squares, blind (contiguous, spaced, or scattered) | up to 17 "located jointly" | **REFUSED**, always with "0 dead squares located" |
| contiguous 4,096 B, blind, at an ordinary unaligned offset | EXACT (`guaranteed: true`) | **REFUSED** |
| the same contiguous 4,096 B, ADDRESSED (`--wound`) | EXACT | **EXACT** |
| up to 16 squares, blind, when each square is only PARTLY hit | -- | **EXACT** (14 squares located and rebuilt in one trial) |

**Nothing was ever wrong.** Across roughly thirty trials the decoder either
restored byte-exact or refused with a number and wrote nothing. Conservation
holds; this is a promise-versus-behaviour gap, not a correctness defect.

**It is the ARMOR, not the peel.** notepad.exe carries no peel and behaves
identically, so the behaviour is v12's armor v4 verbatim, inherited by the fork.
It has been true since v12's M2b and no gate in either campaign asked.

**Why every gate passed anyway.** The tournament's three injuries are a blind
1-byte flip, an ADDRESSED 4 KB scratch and a truncation; the addressed scratch and
the truncation both name their squares, so they never touch this path. The drill
battery does assert EXACT for a blind 4 KB scratch, but only when
`scratch_guaranteed(g)` is true and only on its own synthetic containers, whose
tiers put 4,096 B inside 3 to 5 squares. At 256 B squares a 4,096 B scratch spans
17, and the assertion never runs there.

**The reading.** A square that is only partly hit puts an error into only some of
the 128 interleaved codewords, so each codeword stays inside its own correction
radius and up to 17 such squares rebuild. A fully dead square puts an error into
ALL 128 at once; the per-codeword radius is then the binding constraint and it
measures 8, one short of the printed 9. The collaborative rung that the placement
"none" price was argued on (Krachkovsky-Lee 1997: the columns agree) reports
"0 dead squares located" in every one of those refusals, so on fully dead squares
it is not extending the reach at all.

**What M2 owes, before any deflate work:**
1. Decide which is true and make the other match: either the joint locator reaches
   t-1 fully dead squares as claimed, or the promise text and the `guaranteed`
   flag are corrected to say what the code does. The promise is printed to the
   user on every transmute; it is the product, not a comment.
2. A drill that runs the blind-4-KB assertion on a container of EVERY tier,
   including 256, and on a real corpus row rather than only synthetic ones.
3. If the reach really is 8 fully dead squares at the 256 tier, the tier argmin
   must know it: a row whose blind survival matters may be worth 512 B squares at
   5,324 B instead of 256 B squares at 4,812.

Filed by the main session, not by the milestone's agent, and the milestone's own
report is left exactly as it was written.

## Verification note (2026-09-03, main session, independent of the agent)

Reproduced from a clean shell against `target/release/eggv13.exe`: wallpaper.jpg
transmutes to **1,238,198 B** (inner 1,233,386; peel recipe 451 raw to 272 modelled,
values 55,296,000 raw to 1,233,099 modelled; price 4,812 at 1.17x the floor), and
restores **byte-exact** to 1,602,752 B with the conservation hash reported OK.
`cargo clippy --all-targets -- -D warnings` exits 0; `cargo test --release` is
28/28. Ancestors: across codec-v1 and codegg-v1 .. codegg-v12, **zero files outside
`target/` are newer than codegg-v13's Cargo.toml**.

## THE RECIPE, MEASURED (2026-09-03, before any M2 code) -- the deflate question is answered

Vladimir's instruction: measure the cost of the token recipe before writing
anything else, because it is the only thing standing between v13 and the save.
Done. The whole 66,417,533-byte deflate stream was walked and its decisions
extracted (scratchpad v13/recipe/tokens.py, RFC 1951; the recipe idea is
precomp / reflate, Christian Schneider and Dirk Steinke).

**The extraction is exact**: 1,171 blocks, 38,340,574 tokens (12,679,330 literals,
25,661,244 matches), inflating to **296,540,843 B -- the expected length, to the
byte**. The stored tail field reads 8, the gzip CRC32 plus ISIZE, as it must.

The recipe carries the block structure and code-length definitions exactly as the
stream declares them, one bit per token for literal-or-match, and the length and
distance of every match. It does NOT carry literal values: those are read back
out of the inflated bytes at re-spell time, which is why the flags stream exists
at all.

| stream | raw | xz -9 |
|---|---|---|
| meta (1,171 block headers + every code-length definition) | 307,786 | 46,736 |
| flags (1 bit per token) | 4,792,572 | 535,004 |
| lengths (u16 per match) | 51,322,488 | 661,684 |
| distances (u32 per match) | 102,644,976 | 2,431,520 |
| **the recipe** | 159,067,822 | **3,674,944** |

**Budget 12,097,403 B. The recipe costs 3,674,944 B. It fits with 8,422,459 B to
spare**, and that is the PESSIMISTIC reading: a generic coder on a naive
fixed-width serialisation, with no differential coding of the parse and none of
our own modelling. Distances are 66% of it and are exactly what reflate encodes
as deviations from a reference parse rather than as values.

What that makes the row, using Vladimir's measured 5,256,708 for the peeled bytes
through the v12 arm:

| | bytes |
|---|---|
| peeled values | 5,256,708 |
| recipe (measured, pessimistic) | 3,674,944 |
| inner | 8,931,652 |
| armor price (256-B tier, flat) | 4,812 |
| **armored total** | **8,936,464** |
| against today's 17,333,845 | **-8,397,381** |
| against naked xz -9 16,739,920 | **-7,803,456** |
| against xz+par2 (the bar's only standing row) | **-8,265,680** |

The row that has stood in the armored card three times -- v11 by 0.53 pt, v12 by
0.20, v13 by 0.20 -- does not close by a fraction of a point. It closes by a
factor of two.

**WHAT IS NOT YET PROVEN, and it is the whole of M2's first gate:** the recipe is
*designed* to be sufficient (canonical Huffman from the stored code lengths, the
parse from flags plus lengths plus distances, literals from the output, the pad
bits and trailer accounted) but **nothing has been re-deflated from it yet**. A
recipe that reconstructs 296 MB of bytes is worth nothing until it reproduces the
66,417,533-byte skin BIT FOR BIT, and under THE LAW OF THE PEEL a peel that
cannot prove itself on the file in front of it is not used at all. That round
trip is M2's first task, before any modelling of these streams.

### THE ROUND TRIP IS PROVED (2026-09-03, the same session, before any M2 code)

The recipe above was measured but unproved, and the section said so: "a recipe
that reconstructs 296 MB of bytes is worth nothing until it reproduces the
66,417,533-byte skin BIT FOR BIT." It now does.

**The gate found its first hole before it ran.** The recipe recorded every block's
structure and every token but not WHERE EACH BLOCK ENDS -- an inflater learns that
from the end-of-block symbol, and a re-speller must be told. A second hole sat
behind it: the padding bits after the final block were assumed rather than
recorded. Both are now in the recipe: one u32 of token count per block (1,171
blocks = 4,684 B raw) and the two padding bits (measured: 2 bits, value 0). meta
grew 307,786 -> 312,472 raw, and 46,736 -> 46,888 coded. That is what a gate is
for; the cost of both fixes is 152 bytes.

**The result**: 1,171 blocks, 38,340,574 tokens replayed, **66,417,533 bytes
re-spelled and identical to the original skin, bit for bit**, in 55 s of python.
The inflated side matches too, at 296,540,843 B. The peel is a proved bijection
on this file.

| the complete recipe | raw | xz -9 |
|---|---|---|
| meta (blocks, code-length definitions, token counts, padding) | 312,472 | 46,888 |
| flags (1 bit per token) | 4,792,572 | 535,004 |
| lengths | 51,322,488 | 661,684 |
| distances | 102,644,976 | 2,431,520 |
| **total** | 159,072,508 | **3,675,096** |

Budget 12,097,403. Spare **8,422,307**.

| the row, if M2 ships this | bytes |
|---|---|
| peeled values (Vladimir's measurement, v12 arm on the full peel) | 5,256,708 |
| recipe (xz -9, no differential coding, no CM) | 3,675,096 |
| inner | 8,931,804 |
| armor (256-B tier, flat) | 4,812 |
| **armored total** | **8,936,616** |
| vs today's 17,333,845 | **-8,397,229** |
| vs naked xz -9 | **-7,803,304** |
| vs xz+par2, the armored bar's one standing row | **-8,265,528** |

Everything above is still the pessimistic reading: a generic coder on fixed-width
streams. Distances are 66% of the recipe and are precisely what reflate stores as
deviations from a reference parse rather than as values, and none of these four
streams has yet met our own model.

**What M2 must now do, in this order:** port the extractor and the re-speller to
Rust behind the peel frame; re-prove the round trip there, on this file and on a
deflate suite (gzip at every level, 7-Zip, Explorer zips, PNGs from three
encoders), keeping the bytes wherever the re-spell differs by even one bit; then
model the four streams; only then measure the row. The 296 MB inner also puts the
64 MB in-memory law and the 45-minute monster bar in play (WS-S), and the
tokeniser speed lever from v12's M2c(a) is on the shelf for exactly that.

## The prize hunt, measured (2026-09-03) -- what else is out there

The instrument rule after this session: use one that can see the quantity it is
being used to rule in or out, and say plainly when a number is sampled.

- **rdr2-shaders.vkcache: RULED OUT, on the whole file.** 11,931 windows of 4 KB:
  **79.5% sit above 7.9 bits/byte**, 19.6% between 6.0 and 7.9, 0.9% below 6.0.
  Four fifths of the file is at the ceiling and has nothing left for any model;
  we and xz already take 14.4% and 14.1% off the fifth that isn't. There is no
  peel here and no prize. The plan said "do not chase it"; now it is measured.
- **embeddings.json: a real signal, about 470-750 KB.** 1,275,904 float literals,
  **91.1% of the file is digits**. Re-spelling the first 400,000 as raw f32 and
  coding them costs 1,391,012 against the text's 1,654,648, **-15.9%**. The lever
  is NOT "store f32" (the text must come back byte-identical, and f32 does not
  round-trip a decimal string); it is to model the digits as a number with a
  delta against the previous value in the same array. WS-N in the plan, and the
  first measured reason to build it.
- **iconcache48.db: NOT ESTABLISHED, and my probe was too crude to decide.** A MED
  2D predictor over an 8 MB slice made the residual 37% BIGGER than the raw bytes
  under xz. That is evidence against a naive stride-192 predictor on a slice I
  never proved was pixel payload -- not evidence against the second dimension. To
  answer it, parse the CMMM entries, locate the actual 48x48 BGRA bitmaps, and
  predict inside them. Filed as unanswered rather than as a null result.
- **The wavs: no prize under this instrument.** An order-2 mid/side residual
  through xz costs 144,136 on ring01 against our 130,753 (**+10.2%**, we win) and
  244,632 on alarm01 against our 248,050 (-1.4%). Our model already matches or
  beats a naive audio transform; only an adaptive predictor (NLMS/LPC) could
  change that, and it is unmeasured.
- **rustc_driver.dll: unmeasured, and the largest non-opaque row left.** 37,436,978
  B, 20.4% of 183 MB, already 2.4 points ahead of xz with its own BCJ filter. The
  instrument would be a disassembly-aware context probe (the paq8 / durilca
  lineage past E8E9). Not claimed either way.

## M2 GATE, CLOSED (2026-09-03, by the main session) -- 23 of 23

The M2 agent delivered the peel, the round trip, the suite and the save's row, and
missed the one gate item it owned: its 8-lane ledger HUNG at 12:14 and it reported
"the last lane is still running" for half an hour. Evidence of the hang: the pass
created one temp directory and never wrote an artifact into it in thirty minutes,
where a working pass writes within seconds. (My own first diagnosis cited "no child
processes", which was unreliable -- I grepped for `eggv13` while the copied binary
runs as `egg.exe`. The temp-directory evidence is what stands.) Killed at 12:46 and
re-run here in groups: 12 small rows in one pass, the big rows in pairs, the save
and the monster alone. Cause is unconfirmed; the likely trigger is 8 lanes opening
on the 8 largest rows at once, which since this milestone includes the save's
296 MB peel beside the 183 MB monster.

MISSES FIRST: **1 of 20 rows off its filed total** -- aoe4-autosave.sav at
8,759,069 against a call of 8,766,965, **7,896 lighter than its own prediction**.
Nothing else moved. Injuries 60/60 EXACT;
rows heavier than sealed v11: 0.

### The ledger, 20 rows

| row | sealed v11 | **v13** | delta | pt | inner (arm) | price | × floor | naked xz -9 | injuries |
|---|---|---|---|---|---|---|---|---|---|
| aoe4-autosave.sav | 17,553,840 | **8,759,069** | -8,794,771 | -13.242 | 8,754,257 (PEEL) | 4,812 | 1.17× | 16,739,920 → win | E/E/E |
| rustc_driver.dll | 42,719,952 | **37,436,978** | -5,282,974 | -2.885 | 37,430,630 (CM12·PE) | 6,348 | 1.55× | 41,707,380 → win | E/E/E |
| msgraph.dll | 4,617,660 | **3,211,880** | -1,405,780 | -3.250 | 3,207,068 (CM12·PE) | 4,812 | 1.17× | 4,749,376 → win | E/E/E |
| mermaid-bundle.js | 4,891,080 | **4,090,958** | -800,122 | -3.096 | 4,086,146 (CM12) | 4,812 | 1.17× | 4,952,132 → win | E/E/E |
| rdr2-shaders.vkcache | 42,312,400 | **41,860,990** | -451,410 | -0.924 | 41,854,642 (MIX12) | 6,348 | 1.55× | 41,969,640 → win | E/E/E |
| ntoskrnl.exe | 5,039,572 | **4,675,387** | -364,185 | -2.791 | 4,670,575 (CM12·PE) | 4,812 | 1.17× | 5,501,588 → win | E/E/E |
| wallpaper.jpg | 1,533,228 | **1,238,198** | -295,030 | -18.408 | 1,233,386 (PEEL) | 4,812 | 1.17× | 1,571,824 → win | E/E/E |
| real-test.db | 1,241,376 | **1,068,149** | -173,227 | -1.814 | 1,063,337 (CM12H) | 4,812 | 1.17× | 1,474,592 → win | E/E/E |
| cbs.log | 150,804 | **71,758** | -79,046 | -0.488 | 66,946 (CM12) | 4,812 | 1.17× | 139,004 → win | E/E/E |
| vim-version9.txt | 319,264 | **273,982** | -45,282 | -2.225 | 269,170 (CM12H) | 4,812 | 1.17× | 371,088 → win | E/E/E |
| zstd.exe | 521,540 | **488,915** | -32,625 | -2.037 | 484,103 (CM12·PE) | 4,812 | 1.17× | 534,572 → win | E/E/E |
| arial.ttf | 468,292 | **446,354** | -21,938 | -2.098 | 441,542 (CM12·TTF) | 4,812 | 1.17× | 466,716 → win | E/E/E |
| alarm01.wav | 273,196 | **252,862** | -20,334 | -4.137 | 248,050 (CM12) | 4,812 | 1.17× | 344,640 → win | E/E/E |
| segoeui.ttf | 429,368 | **409,683** | -19,685 | -2.051 | 404,871 (CM12·TTF) | 4,812 | 1.17× | 422,304 → win | E/E/E |
| kernel32.dll | 300,832 | **283,604** | -17,228 | -2.060 | 278,792 (CM12·PE) | 4,812 | 1.17× | 316,416 → win | E/E/E |
| ring01.wav | 146,184 | **135,565** | -10,619 | -2.131 | 130,753 (CM12) | 4,812 | 1.17× | 248,424 → win | E/E/E |
| real-test.bmp | 268,588 | **261,274** | -7,314 | -0.061 | 256,462 (MIX12) | 4,812 | 1.17× | 3,612,796 → win | E/E/E |
| notepad.exe | 183,060 | **176,164** | -6,896 | -1.913 | 171,352 (CM12·PE) | 4,812 | 1.17× | 181,884 → win | E/E/E |
| iconcache48.db | 424,760 | **418,323** | -6,437 | -0.007 | 413,511 (MIX12) | 4,812 | 1.17× | 416,968 → **+1,355** | E/E/E |
| wubbadub.html | 30,596 | **27,621** | -2,975 | -3.219 | 22,809 (CM12P) | 4,812 | 1.17× | 26,180 → **+1,441** | E/E/E |
| **20 rows** | | | **-17,837,878** | | | | | **18 of 20 lighter** | **60/60 EXACT** |

**Net -17,837,878 B against the sealed v11.** v12's whole campaign was -8,987,397;
v11's was -1,766,332. Form vs form (our inner against naked xz -9's stream):
**20 of 20** -- the save was the series' last form-vs-form loss and it now wins by
7,985,663. The exhibit (armored total vs naked xz, never a bar): 18 of 20, the two
remaining losses iconcache48.db +1,355 and wubbadub.html +1,441, each carrying
4,812 B of armor xz does not carry.

### THE BAR: armored vs armored, 23 rows, measured

rar -rr5 forfeits truncation on **23 of 23** rows -- structural, as in v11 and v12.
xz+par2 survives every injury and is beaten by egg13 on **23 of 23**.

| row | egg13 | egg13 % | xz+par2 % | margin |
|---|---|---|---|---|
| iconcache48.db | 418,323 | 0.43% | 0.5% | **+0.07** |
| cbs.log | 71,758 | 0.44% | 1.0% | **+0.56** |
| msgraph-docs.xml | 965,799 | 1.13% | 2.6% | **+1.47** |
| rdr2-shaders.vkcache | 41,860,990 | 85.65% | 88.0% | **+2.35** |
| changelog.md | 179,700 | 14.86% | 17.8% | **+2.94** |
| rustc_driver.dll | 37,436,978 | 20.44% | 23.4% | **+2.96** |
| msgraph.dll | 3,211,880 | 7.43% | 11.3% | **+3.87** |
| mermaid-bundle.js | 4,090,958 | 15.83% | 19.8% | **+3.97** |
| embeddings.json | 4,687,145 | 29.66% | 34.4% | **+4.74** |
| real-test.db | 1,068,149 | 11.18% | 17.1% | **+5.92** |
| zstd.exe | 488,915 | 30.53% | 37.6% | **+7.07** |
| segoeui.ttf | 409,683 | 42.69% | 49.8% | **+7.11** |
| vim-version9.txt | 273,982 | 13.46% | 20.7% | **+7.24** |
| ntoskrnl.exe | 4,675,387 | 35.83% | 43.5% | **+7.67** |
| arial.ttf | 446,354 | 42.68% | 50.4% | **+7.72** |
| kernel32.dll | 283,604 | 33.92% | 43.3% | **+9.38** |
| notepad.exe | 176,164 | 48.87% | 59.5% | **+10.63** |
| aoe4-autosave.sav | 8,759,069 | 13.19% | 25.9% | **+12.71** |
| wubbadub.html | 27,621 | 29.89% | 47.2% | **+17.31** |
| alarm01.wav | 252,862 | 51.45% | 80.0% | **+28.55** |
| ring01.wav | 135,565 | 27.20% | 57.7% | **+30.50** |
| real-test.bmp | 261,274 | 2.18% | 33.2% | **+31.02** |
| wallpaper.jpg | 1,238,198 | 77.25% | 108.6% | **+31.35** |

**23 of 23.** The bar was missed in v11 by 0.53 pt and in v12 by 0.20 pt, both
times by the game save. That row now wins by 12.71 points, a factor of 1.96. It is
the first clean sweep of the series, and the row that took it is the row that had
never fallen.

### Speed, SOLO (machine idle, one transmute at a time)

Worst home row **kernel32.dll at 0.278 MB/s** against the 0.25 floor: MET.
The other eleven: wubbadub 0.402, ring01 0.282, notepad 0.304, alarm01 0.306,
segoeui 0.374, arial 0.383, zstd 0.464, vim 0.502, real-test.db 0.398,
real-test.bmp 0.531, wallpaper.jpg 0.349.

### Ancestors and the site

Across codec-v1 and codegg-v1 .. codegg-v12: **zero files outside `target/` newer
than codegg-v13's Cargo.toml**. One site file, `inspirations.html`, carries a
mtime of 13:14 today and is **content-identical to git HEAD** (`git status` reports
nothing for it), so no bytes moved; it is the IDE, not this campaign.


## Attribution

ITU T.81 (the JPEG codec, the canonical Huffman tables, the 1-bit padding);
packJPG, Matthias Stirner, and Lepton, Dropbox (the coefficient-model context
shape and the peel idea); Reed & Solomon 1960, Berlekamp-Massey, Chien, Forney
(the armor, unmoved from v12); Krachkovsky & Lee 1997 and Bleichenbacher,
Kiayias & Yung 2003 (collaborative decoding of interleaved RS); Matt Mahoney's
zpaq / lpaq / paq8 and Byron Knoll's cmix (the 16-bit coder, the StateMap and
APM shapes); Fowler-Noll-Vo (FNV-64, the conservation hash); XZ Utils 5.8.3 (the
rival's stream, exact bytes). The site supplied every reading.

## M1 MEASURED (2026-09-03 08:23) -- wallpaper.jpg, the first reading, written the moment it landed

The peel arm on `corpus-real/wallpaper.jpg`, with THE LAW satisfied (the peel
re-encoded its own output and the re-encode WAS the original file, before
anything was written) and the container restored EXACT afterwards.

| quantity | FILED | MEASURED | verdict |
|---|---|---|---|
| round trip, Rust port, byte-exact | YES | **YES** -- `respell` == the 1,602,752 original bytes; the container restores EXACT, conservation hash OK | **HIT** |
| the recipe's own bytes (raw) | 441 | **451** | MISS +10: 441 is the non-entropy bytes; the recipe adds its own 10-byte header (version, pad convention, the two lengths) |
| the recipe stream + preamble | 380 .. 560 B | **287 B** (272 B under the CM12 arm, which beat storing it, + the 15-byte preamble) | **MISS, BELOW** -- lighter than called |
| the peeled values | 1,201,733 .. 1,361,964 (-25% .. -15%) | **1,233,099 B = -23.04%** of 1,602,311 | **HIT** (inside the range, near its light end) |
| inner | 1,202,200 .. 1,362,500 | **1,233,386** | **HIT** |
| price | 4,812 B flat | **4,812** (256-B tier, 18 parity, 0 CT, 204 sites; 1.17x the 4,096 floor) | **HIT** |
| **armored total** | 1,207,000 .. 1,367,300 | **1,238,198** | **HIT** |
| vs v12's total 1,513,903 | -146,600 .. -306,900 | **-275,705** | **HIT** |
| vs naked xz -9 1,571,824 (exhibit) | lighter by 204,500 .. 364,800 | **lighter by 333,626** | **HIT** |
| **Gate 1**: beat v12 by >= 10% of 1,602,311 (= 160,231 B, total <= 1,353,672) | called as straddling the gate | **MET: -275,705 = 17.21% of the entropy-coded bytes** | **MET** |

Geometry, printed: `model 24` (MODEL_PEEL), `peel 1`, recipe 451 B raw -> 272 B
(model 17, CM12), values 55,296,000 B raw -> 1,233,099 B (model 25, the
coefficient model). The RAW dump is what v12 weighed: 55,296,000 B, which xz -9
made 2,192,684 and our MIX12 arm 2,208,961 -- both heavier than the JPEG's own
Huffman. The same coefficients, MODELLED, are **1,233,099**: 1.78x lighter than
xz on the dump, and 0.77x the JPEG's own spelling. That is the milestone.

Wall clock at this reading was 4,712 ms, but the 20-row M0 ledger was running in
eight lanes at the time: **that number is CONTENDED and is not a speed
measurement.** The solo figure is below, in its own section.

## M1 MEASURED -- the 60-JPEG conservation suite (2026-09-03 08:30)

`corpus-jpeg`, built by `tools/mkjpegsuite.js` and judged by `tools/jpegsuite.js`.
Provenance as filed: 46 JPEGs copied from this machine's `C:\Windows\Web` tree
(Screen, 4K, Wallpaper/{Alienware, Spotlight, ThemeA-D}, touchkeyboard) -- eight
of them ALREADY progressive, several with odd dimensions (3839x2400, 3841x2400,
3840x2401, 2054x1155), two byte-identical to the corpus row; 4 generated here
with Windows' own WIC `JpegBitmapEncoder` (two 4:2:0 colour, two Gray8
single-component, all JFIF and restart-free); 9 hand-built hostiles, each a
named mutation of a real file; and the corpus row itself. **60 files, 35 MB.**

**LOST: 0. WRONG: 0.** Every one of the sixty transmuted and restored EXACT.

| class | count | what happened |
|---|---|---|
| baseline, peeled and TAKEN by the argmin | **40** | re-spelled byte-exact; values 13.91% .. 92.23% of each file's entropy-coded bytes |
| baseline, peel PROVED a bijection but PASSED OVER by the argmin | **3** | the two Gray8 frames and `win_Screen_img105.jpg`: on these synthetic or near-flat images the ORDINARY pipeline is far lighter (wic_gray_q50: raw inner 2,299 B against the peel's 23,430), so the trial kept the raw form. That is the trial working, not a refusal -- the peel proved itself byte-exact on all three, greyscale and 4:2:0 alike |
| refused, bytes kept, reason printed | **17** | 8 progressive (SOF2) + 9 hand-built hostiles |

**The peel proved itself a bijection on 43 of the 60**; the other 17 were
refused with a printed reason and kept their bytes.

The refusals, each with the sentence the build printed:
progressive JPEG (SOF2): the bytes are kept x8; arithmetic-coded JPEG
(SOF9/10/11): the bytes are kept; 12-bit samples (only 8-bit is peeled);
DHT symbol list truncated; DC category above 15 (the corrupt DHT);
AC run walks past coefficient 63 (the noise-filled scan);
entropy data ran out mid-block (a marker injected into the scan);
entropy-coded segment runs to the end of the file (truncated) x2;
marker segment runs past the end of the file (the JPEG magic on a non-JPEG).

**Against codegg-v12 on the same sixty files: 32,875,972 B -> 27,910,927 B,
-4,965,045 B (-15.1%).** Every refused file's total is byte-identical to v12's
(delta 0 on all 20 refused / passed-over rows) -- the frame does not perturb
what it does not peel.

Two readings worth printing:

- **the recipe does not eat the prize, but it is not always small.** The
  Alienware wallpapers carry EXIF thumbnails and several DQT/DHT segments, so
  their recipes are 2,603 .. 18,869 B against values of 18,254 .. 1,928,881.
  The worst ratio in the suite is `win_Wallpaper_Spotlight_img50.jpg`: a
  23,615 B 280x175 4:2:0 frame whose recipe is 2,603 B on 18,254 B of values --
  14%. It still wins its row (25,684 against v12's 27,040), and the argmin, not
  a rule of thumb, is what decided that.
- **the model needs room.** The best row in the suite is 13.91% of the entropy
  coding (`AW-CO8`, a near-flat 1920x1200 frame); the worst peeled row is 92.23%
  (`img50`, 1,188 blocks -- a model that has barely started learning when the
  file ends). The corpus row sits at 76.96%.

## M1 MEASURED -- the 20-row ledger, the drills, the audit, the speed (2026-09-03 09:51)

### The 20-row ledger: 20 of 20 HIT, to the byte

`node tools/ledger13.js` with `EGG_PRED` = the M1 column filed before the run
(nineteen rows at v12's totals TO THE BYTE, `wallpaper.jpg` at the measured
1,238,198) and `EGG_EXE` = a copy of the M1 build.

**Prediction misses: 0. Injuries: 60/60 EXACT (E/E/E on every row). Failures: 0.
Rows heavier than v11: 0.**

| | v12 | v13-M1 | delta |
|---|---|---|---|
| wallpaper.jpg | 1,513,903 (inner 1,509,091, model 17) | **1,238,198** (inner 1,233,386, **model 24**) | **-275,705** |
| the other nineteen rows | | **byte-identical, every one** | 0 |
| net vs the sealed v11, 20 rows | -8,987,397 | **-9,263,102** | -275,705 |

Gate 3 of M1 asked that the frame not perturb the rows it does not peel: it does
not. Every one of the nineteen keeps its v12 total, its v12 model byte, its v12
inner and its flat price (4,812 / 5,324 / 6,348 by tier, `CT none`).

### Drills: 273 passed, 0 failed (v12's battery was 257; the peel added 16)

Every line of the new peel battery PASSED:

- the trial takes the peeled form on a baseline JPEG (model 24, total 1,238,198);
- `info` names the recipe and the values, and the geometry mirror holds on a
  peeled container exactly as on any other;
- the recipe does not eat the prize (272 B against 1,233,099 B of values);
- a PEELED container survives the three tournament injuries -- 1-byte flip
  blind, 4 KB scratch addressed AND blind, 4 KB truncation -- all EXACT;
- and refuses HONESTLY beyond capacity;
- six refusal drills -- a truncated member, a corrupt DHT, a progressive input,
  an arithmetic-coded input, 12-bit samples, and **a peel whose re-encode would
  differ** (the padding before one restart poisoned) -- every one KEPT ITS BYTES,
  went through the ordinary pipeline, and restored EXACT.

Audit v4 `--full`: **3,091,667 checks, 0 failing** -- identical to v12's, the
armor having not moved. `cargo clippy --all-targets -- -D warnings` clean under
rustc 1.98. `cargo test --release`: **28 passed** (v12: 23), the five new ones
being the peel's byte-exact round trip, its refusal of a progressive frame, the
coefficient model's round trip, the counter's range, and the peeled form's whole
pipeline law.

### Speed, SOLO (machine idle, one transmute at a time, three runs each)

| what | ms (3 runs) | MB/s | bar |
|---|---|---|---|
| **wallpaper.jpg transmute, peel ON** | 4,247 / 4,259 / 4,382 | **0.377** | home floor 0.25: **MET** |
| the same row with `EGG_NO_PEEL=1` (v13's v12 path) | 4,094 / 4,212 / 4,303 | 0.391 | v12's own solo figure was 0.382 |
| restore of the peeled container | 167 / 167 / 168 | 9.6 | |

**FILED 0.20 .. 0.35 MB/s; MEASURED 0.377 -- a MISS, ABOVE the range (faster
than called).** The reason is worth writing down, because the call was made from
the wrong model of the cost. The peel does five things -- Huffman-decode 432,000
blocks, re-encode them and compare against the original (THE LAW), model the
coefficients, then decode the model and re-encode again for the write-time
round-trip law -- and I priced them as serial additions to the trial. They are
not: the peel arm runs in a thread BESIDE the ordinary roster, which still costs
about 4.1 s on this row and dominates. The whole peel, both directions and both
re-encodes, fits inside that and adds **about 160 ms, 3.8%**. The lean
coefficient model (one adaptive probability per context, no mixer) is what makes
that possible; it was chosen for this floor and it cleared it with room.

### A defect found and NOT fixed inside the gate, printed instead

The `transmute` line prints "`(0 MB/s)`" on this row. The arithmetic is v12's,
inherited verbatim: `src.len() / ms / 1000` in integer u128, which is 0 for any
file under about 1 MB per second. It is a display bug, it has been printing a
wrong number since v12, and the honest place to fix it is M2 -- changing the
gated binary for a cosmetic would mean the ledgers above no longer describe the
exe that produced them. Filed here rather than quietly patched.

## The M1 gate, item by item

| gate | verdict |
|---|---|
| 1. wallpaper.jpg beats its v12 total by >= 10% of the entropy-coded bytes | **MET: -275,705 B = 17.21% of 1,602,311** (needed 160,231) |
| 2. a 30+ JPEG round-trip suite, every file byte-exact or cleanly refused; nothing lost, nothing wrong | **MET: 60 files, LOST 0, WRONG 0** -- 40 peeled and taken, 3 peeled and passed over by the argmin, 17 refused with a printed reason |
| 3. every OTHER row byte-identical to its v12 total | **MET: 19 of 19**, a 20-row ledger with EGG_PRED, 0 misses |
| 4. injuries 3 x 20 EXACT; drills green with a peel battery; audit green; clippy/test green | **MET: 60/60 injuries; 273/273 drills; 3,091,667 audit checks 0 failing; clippy clean; 28/28 tests** |
| 5. the JPEG row measured SOLO against the 0.25 MB/s floor | **MET: 0.377 MB/s** (v12: 0.382; the peel costs 3.8%) |
| 6. ancestors untouched, proven by mtime against the M0 manifest | **MET: 384 ancestor files outside target/, 0 newer, 0 added, 0 removed, 0 mtimes moved; 43 site files unmoved** |

## Deviations from the charter plan, and why

1. **The values are modelled by ONE arm, not by the roster.** The plan says "the
   model runs on values". M1 runs the JPEG coefficient model on them and does
   NOT also run the generic byte arms on the 55,296,000-byte dump. v12 already
   measured that road on this exact file (xz -9 2,192,684; our MIX12 arm
   2,208,961; both heavier than the JPEG's own Huffman) and re-measuring a known
   loss would cost minutes per file. The reading is printed instead of repeated,
   in `peel.rs`, with the numbers.
2. **Nesting is not exercised.** The plan allows a member inside a member to
   depth 2. The JPEG peel has no members, so M1 implements depth 1 and REFUSES a
   peel inside a recipe with a number. M2's deflate is what needs the second
   level, and it will be built there.
3. **SOF1 (extended sequential, Huffman) is refused**, though it decodes like
   baseline. Nothing in the suite is SOF1 and an untested acceptance is a guess;
   it costs one line to accept when a file demands it.
4. **Coverage the suite could not buy on this machine**: no 4-component (CMYK
   Adobe) JPEG and no 4:2:0-with-restart-intervals file exists under
   `C:\Windows\Web`, and no encoder that makes one is installed (no ImageMagick,
   no ffmpeg, no Pillow). The code peels both classes; neither is tested. THE LAW
   is what stands between that gap and a wrong byte: an untested class either
   proves itself against the original file at write time or is refused.

## M2 (0) MEASURED (2026-09-03 11:38) -- the blind promise, settled: it HOLDS, with one word wrong

M2's first task was to decide whether the joint locator really reaches t-1 fully
dead squares or whether the promise must be corrected. It was measured before any
deflate work, on the same two containers the FINDING named, and then on every tier.

**The FINDING's central claim does not reproduce.** Against
`target/release/eggv13.exe` built from this tree, blind (no `--wound`, no
truncation), whole data squares overwritten:

| container | tier | t | blind fully-dead squares, EXACT | first REFUSAL |
|---|---|---|---|---|
| notepad.exe (no peel) | 256 | 18 | **1 .. 17** | 18 |
| notepad.exe | 512 | 10 | 1 .. 9 | 10 |
| notepad.exe | 1024 | 6 | 1 .. 5 | 6 |
| notepad.exe | 2048 | 4 | 1 .. 3 | 4 |
| notepad.exe | 4096 | 3 | 1 .. 2 | 3 |
| wallpaper.jpg (MODEL_PEEL) | 256 | 18 | **1 .. 17** | 18 |

That is `t-1` located jointly and `t` refusing -- exactly what the promise prints.
135 trials at the 256 tier alone (counts 6..20 x fills {random, zero, 0xFF} x
layouts {contiguous, spread every 37th, scattered}): every one EXACT up to 17,
every one REFUSED at 18, **never wrong**. A contiguous 4,096 B blind scratch is
EXACT at every tier and at every offset tried, including runs that cross the mid
and end sites, and via the shipped `scratch` subcommand at 7 offsets.

**Where the FINDING's reading came from.** The refusal it quoted --
"0 dead squares located" -- prints `e.len()`, the size of the erasure set the
attempt STARTED from. On a blind wound that set is empty by definition, so the
phrase reads "the joint locator found nothing" when it actually says "nothing was
named before we began". The message is being fixed rather than defended. The
"8 EXACT / 9 REFUSED" ladder is consistent with a probe whose square offsets did
not apply `stream_pos` (the short last data square sits at stream position 0) and
the `pad` shift, so each intended one-square wound straddled two real squares --
9 intended = 18 real = exactly t. It is not reproducible with correct offsets.

**The one thing the FINDING was right about, and it is a real overclaim.** The
promise says "BLIND: any t/2 squares CERTAIN (Berlekamp-Massey)". CERTAIN is the
wrong word at t/2. The per-codeword BM rung refuses unless `2*deg < m` -- an
unverified locator is a guess -- so BM alone reaches `(t-1)/2`, not `t/2`. Above
that the JOINT rung carries the load, and the joint rung is conditional: it
succeeds iff the error rows across the 128 interleaved codewords are independent.

Constructed and measured: XOR one fixed 256-byte pattern into k squares, making
the 128 x k error matrix rank 1, and the joint rung has nothing to span:

| tier | t | (t-1)/2 | rank-1 dependent wound: last EXACT | first REFUSAL |
|---|---|---|---|---|
| 256 | 18 | 8 | 8 | **9** |
| 512 | 10 | 4 | 4 | 5 |
| 1024 | 6 | 2 | 2 | 3 |
| 2048 | 4 | 1 | 1 | 2 |
| 4096 | 3 | 1 | 1 | 2 |

`(t-1)/2` to the row. So the certain-blind reach is 8 at the tier 17 of 20 rows
land on, not 9, and 9..17 are the joint rung's conditional band.

**WHAT CHANGED (the promise is the product, so the text moved, not the reach):**
1. `promise()` for placement none now prints the certain band as `(t-1)/2`
   squares by per-codeword Berlekamp-Massey, and puts `(t-1)/2 + 1 .. t-1` in the
   joint band with its independence condition -- including the contiguous
   >= 4,096 B run, which at every tier rides that joint rung and not BM.
2. The refusal message no longer says "N dead squares located". It now
   distinguishes what was named or convicted before the attempt from what the
   joint locator placed.
3. A new drill, `blind_4k_every_tier`, runs the blind 4 KB assertion on a REAL
   corpus row (notepad.exe) at all five tiers, plus the t-1 / t boundary and the
   rank-1 dependent wound at the certain band's edge, asserting EXACT below the
   line, REFUSED above it, and WRONG never.
4. `guaranteed` (the `scratch_guaranteed` flag) is UNCHANGED and stays true: the
   4 KB contiguous blind claim measured EXACT at every tier. What changed is that
   the promise now says which rung delivers it.
5. The contiguous clause used to read "contiguous >= 4,096 B blind", which can be
   read as "any run of 4,096 B OR MORE" -- and a 10 MB run is of course not
   covered. It reads "ANY contiguous blind run up to 4,096 B" now. The bound is
   the same number it always was, `(t-2) x blk`, which is 4,096 at every tier.

Nothing here is a conservation defect: across roughly 200 trials the decoder
either restored byte-exact or refused with a number. The gap was in the words.

## M2 FILED (2026-09-03 11:47, BEFORE ONE LINE of deflate or stream-model code)

The only M2 code written before this section is the promise correction and its
drill (M2 task 0, above), which cannot move a byte of any stream.

### The design being predicted

`peel_id 2`, the deflate peel, ported from the proven python (scratchpad
v13/recipe/tokens.py + respell.py; RFC 1951; the recipe idea is precomp /
reflate, Christian Schneider and Dirk Steinke). Same frame as the JPEG peel and
the same law: at transmute time the recipe is re-spelled and compared against the
original bytes, and one byte of difference discards the peel for that file.

The four streams do NOT go through the generic roster. They get ONE dedicated
model each, in one walk, encoder and decoder driven through the same `Coder`
trait that jcoef.rs uses -- the M1 precedent, filed as deviation 1 of M1 and
repeated here for the same reason (a 159 MB fixed-width serialisation through
five arms would cost tens of minutes to re-measure a shape we can model
directly). The contexts I intend, and they are the reason for the numbers below:

- **flags**: the previous flags (an order-k bit history) and the length of the
  last match. Literal-or-match is a run-structured language.
- **lengths**: the deflate length SYMBOL first (29 of them) and then its extra
  bits, with the previous length symbol and the flag history as context -- never
  the raw u16, which is a spelling and not a value.
- **distances**: the deflate distance SYMBOL and its extra bits, and BEFORE that
  a repeat-distance test against the last four distances used (LZMA's rep0..rep3,
  Igor Pavlov; the same idea reflate reaches by a different road). Deflate
  encoders reuse distances constantly and a u32 hides that completely.
- **meta**: small and regular; the generic CM arm, as M1 does for the JPEG
  recipe.

The values (the 296,540,843 inflated bytes) get ONE generic arm, not the roster,
for the same filed reason plus the 45-minute bar.

### The four streams, called against xz -9 (which measured 3,675,096 total)

| stream | raw | xz -9 | **CALLED (ours)** | range |
|---|---|---|---|---|
| meta | 312,472 | 46,888 | **42,000** | 38,000 .. 50,000 |
| flags | 4,792,572 | 535,004 | **470,000** | 420,000 .. 540,000 |
| lengths | 51,322,488 | 661,684 | **560,000** | 480,000 .. 680,000 |
| distances | 102,644,976 | 2,431,520 | **1,950,000** | 1,600,000 .. 2,450,000 |
| **the recipe** | 159,072,508 | **3,675,096** | **3,022,000** | 2,550,000 .. 3,700,000 |

The distances call is the one with the widest range because it is the one lever
whose payoff is unmeasured: rep-distance context is standard and I expect it to
take 15-25% off xz's number, but the reference-parse deviation lever (store where
the encoder disagreed with a matcher of our own, reflate's actual method) is NOT
in this call. If it is built and pays, the miss is printed as a win beside this
row; if it is built and does not pay, the miss is printed as a loss; if it is not
built, this section says so.

### The row, called

| | bytes |
|---|---|
| peeled values (one generic arm on 296,540,843 B) | **5,256,708** (range 5,150,000 .. 5,400,000) |
| recipe (our model, called above) | **3,022,000** |
| peel preamble | 15 |
| **inner** | **8,278,723** |
| armor price (256-B tier, flat) | 4,812 |
| **armored total, CALLED** | **8,283,535** |

against, in the four currencies:

| bar | bytes | called delta |
|---|---|---|
| today's v13-M1 row | 17,333,845 | **-9,050,310** |
| naked xz -9 | 16,739,920 | **-8,456,385** |
| xz + par2 (the armored card's standing row) | ~17,202,144 | **-8,918,609** |
| the python proof's reference point | 8,936,616 | **-653,081** |
| **the charter plan's M2 gate ("at or under 6.5 MB armored")** | 6,500,000 | **+1,783,535 -- CALLED AS A MISS** |

**The charter bar is called as a MISS before the code is written, and it is
called that way on arithmetic, not on pessimism.** The values alone are 5,256,708
and the recipe cannot be squeezed to 1.24 MB by contexts; only the reference-parse
deviation lever could reach 6.5 MB, and it is optional in this brief. Printed
here so that a 8.3 MB result is read as what it is -- the row closing by a factor
of two against every real rival, and missing an internal target that was set
before the recipe was measured.

### Speed, called SOLO

| what | called | bar |
|---|---|---|
| aoe4-autosave.sav, transmute, peel ON | **55 .. 110 minutes** | the 45-minute monster bar: **CALLED AS A MISS** |
| rustc_driver.dll (no peel on it) | 38 .. 45 minutes | unchanged from v12's 40m56.9s |
| restore of the peeled save | 20 .. 45 minutes | -- |

The save's inner is 296,540,843 B, 1.62x the monster's 183,111,168, and the
monster measured 40m56.9s SOLO in v12. Unless WS-S moves the floor, the peeled
save takes longer than the monster ever did. The M2c(a) tokeniser lever (2^24 LZ
hash, byte-identical tokens, 7.4x on 66 MB) is the shelf item for exactly this
and is the first thing to try; a reduced roster above 64 MB is the second.

## M2 MEASURED (2026-09-03 11:55) -- the four streams, written the moment they landed

Measured against the M1 build BEFORE the model code was written, so the numbers
below chose the design rather than the design choosing the numbers.

### A finding the python proof did not make: the streams were twice as wide as they need to be

RFC 1951 lengths are 3..258 and RFC 1951 distances are 1..32,768. The proven
python stored a **u16 per length and a u32 per distance**; one byte and two bytes
hold them exactly. The recipe's raw side falls from 159,072,508 B to
**82,088,776 B** with nothing lost, and the xz baseline moves with it:

| stream | python raw | python xz -9 | narrow raw | narrow xz -9 |
|---|---|---|---|---|
| meta | 312,472 | 46,888 | 312,472 | 46,888 |
| flags | 4,792,572 | 535,004 | 4,792,572 | 535,004 |
| lengths | 51,322,488 | 661,684 | **25,661,244** | **626,296** |
| distances | 102,644,976 | 2,431,520 | **51,322,488** | **2,394,404** |
| total | 159,072,508 | 3,675,096 | 82,088,776 | **3,602,592** |

### The four streams through OUR roster, against the filed call

| stream | raw | xz -9 | **CALLED** | **MEASURED** | miss | vs xz | the arm that won |
|---|---|---|---|---|---|---|---|
| meta | 312,472 | 46,888 | 42,000 | **40,608** | **-1,392 (better)** | **-13.4%** | CM12P |
| flags | 4,792,572 | 535,004 | 470,000 | **514,819** | **+44,819** | -3.8% | CM12 |
| lengths | 25,661,244 | 626,296 | 560,000 | **574,984** | **+14,984** | -8.2% | CM12 |
| distances | 51,322,488 | 2,394,404 | 1,950,000 | **2,374,936** | **+424,936** | **-0.8%** | **MIX12** |
| **the recipe** | 82,088,776 | **3,602,592** | **3,022,000** | **3,505,347** | **+483,347** | **-2.7%** | four arms |

**All four land INSIDE their filed ranges and all four point calls MISS, three of
them high.** The recipe beats xz by 97,245 B (2.7%) and beats the python proof's
3,675,096 by 169,749 B.

**The distance call is the miss that matters (+424,936) and the reason is
measured, not guessed.** I called 15-25% off xz on rep-distance context. Measured:
of 25,661,244 distances, **only 9.3% repeat any of the last four** (rep0 954,972;
rep1 986,999; rep2 243,828; rep3 193,666; new 23,281,779). LZMA's rep model is
built for a language this stream does not speak, and the lever is **DEAD, with a
number**. A second cheap idea died beside it: splitting the distances into a low
and a high byte plane costs **2,596,091** (1,544,588 + 1,051,503) against
**2,374,936** for the plain u16 stream -- the planes are 9.3% WORSE, because
splitting them destroys the one structure that is actually there.

**What IS there, and it is the reading of the row:** distances are 0.740 bits
each, and the only arm that wins them is **MIX12, the LZ arm** -- the only one in
the roster with a match model. The distance SEQUENCE repeats at a range deflate's
own 32 KB window can never reach, so the encoder re-emits whole passages of the
same parse and our long-range model catches them. That is also why the four
sections are modelled APART and not interleaved: they are won by three different
arms (CM12P, CM12, CM12, MIX12), and interleaved they would hide each other.

### The design that follows from the measurement

The bespoke four-stream context model described in the M2 FILED section was
**NOT built**. The measurement above says the ordinary roster already beats xz on
every section, and a hand-written model would have to beat an LZ arm at finding
long-range repeats in the distance stream -- the one thing this data most needs
and the one thing a context model without a match model cannot do. `MODEL_DRECIPE`
is therefore the SECTIONED ROSTER: four sections, four independent trials, each
keeping its own winner's model byte. That is a deviation from the filed design and
it is filed here with the numbers that caused it.

## M2 MEASURED (2026-09-03 12:00) -- THE PORT IS PROVED IN RUST

`cargo test --release`: **31 passed, 0 failed** (M1: 28). The three new ones are
the deflate peel's, and the first of them is M2's own gate:

- **`the_save_skin_respells_bit_for_bit`**: `corpus-big/aoe4-autosave.sav` peels to
  **1,171 blocks, 38,340,574 tokens, 25,661,244 matches, 296,540,843 B inflated**
  -- every number identical to the python's -- and `respell` reproduces all
  **66,417,543 B of the original file** byte for byte.
- `the_blob_round_trips_the_recipe`: the serialised recipe survives the round trip
  and re-spells its member.
- `hostile_streams_refuse_with_a_reason`: a truncated member, a corrupted table, an
  empty file, a header with no body and a recipe whose sections do not add up all
  refuse; none panics.

Two things the port did NOT inherit from the python it came from:
1. **the reader's byte position**. The python computed the stream's end as
   `pos - 0`, ignoring whole bytes still sitting in its bit accumulator. On this
   file it happened not to matter; in Rust `at()` subtracts them.
2. **the two spellings of a 258-byte match**. RFC 1951 allows length 258 as
   symbol 285 and as symbol 284 with 31 extra bits. The recipe stores lengths, not
   spellings, so the second spelling would not come back -- it is REFUSED with that
   sentence rather than silently re-spelled as the first.

## M2 FILED (2026-09-03 12:05) -- the save's row and the 20-row column, before either landed

The recipe has landed (3,505,430 B: 40,654 + 514,819 + 574,984 + 2,374,936 plus a
37-byte section table). The values have NOT: the roster is still running on the
296,540,843 inflated bytes. So the row is called now, with the recipe measured and
the values taken from Vladimir's probe:

| | bytes |
|---|---|
| peeled values (called at the probe's 5,256,708) | 5,256,708 |
| recipe (MEASURED) | 3,505,430 |
| peel preamble | 15 |
| **inner, CALLED** | **8,762,153** |
| armor price: the 256-B tier, because the inner drops out of 512 | 4,812 |
| **armored total, CALLED** | **8,766,965** |

and the 20-row column, filed as `EGG_PRED` before `ledger13.js` is run:
nineteen rows at their M1 totals TO THE BYTE (wubbadub.html 27,621; cbs.log
71,758; ring01.wav 135,565; notepad.exe 176,164; alarm01.wav 252,862;
real-test.bmp 261,274; vim-version9.txt 273,982; kernel32.dll 283,604;
segoeui.ttf 409,683; iconcache48.db 418,323; arial.ttf 446,354; zstd.exe 488,915;
real-test.db 1,068,149; wallpaper.jpg 1,238,198; msgraph.dll 3,211,880;
mermaid-bundle.js 4,090,958; ntoskrnl.exe 4,675,387; rustc_driver.dll 37,436,978;
rdr2-shaders.vkcache 41,860,990) and **aoe4-autosave.sav at 8,766,965**.

The nineteen are called unchanged even though M2 moved two things that could
touch them: the deflate NOMINATION now runs on every file, and the promise text
changed. Neither can move a byte of a container -- nomination on a non-deflate
file fails its bounded block-header probe and the promise is printed, never
stored -- and that is the call.

## M2 MEASURED (2026-09-03 12:08) -- THE SAVE'S ROW, written the moment the arm landed

The peel arm on `corpus-big/aoe4-autosave.sav`, with THE LAW satisfied (the peel
re-spelled its own output and the re-spell WAS the original 66,417,543 bytes,
before anything was written):

| component | raw | modelled | model |
|---|---|---|---|
| the recipe (4 sections) | 82,088,836 | **3,505,430** | 26 (MODEL_DRECIPE) |
| the values (the inflated file) | 296,540,843 | **5,248,812** | 17 (CM12) |
| the preamble | 15 | 15 | -- |
| **inner** | | **8,754,257** | 24 (MODEL_PEEL) |
| armor price, 256-B tier, flat | | 4,812 | |
| **ARMORED TOTAL** | | **8,759,069** | |

**Called 8,766,965; measured 8,759,069. A MISS of -7,896 B, on the good side.**
The recipe was measured before the call and landed to the byte; the values came
in at 5,248,812 against the 5,256,708 the call borrowed from Vladimir's probe,
because the roster's argmin took **CM12, not MIX12** (MIX12 5,266,032; CM12
5,248,812; CM12H 5,347,643; MIX11 5,369,814; CM11 17,395,763). The literal-only
arm winning on 296 MB of Relic Chunky is its own reading: after the peel, the
long-range repeats deflate could not reach are gone from the values and into the
recipe, and what is left is a literal language.

### The row against every bar

| bar | bytes | delta |
|---|---|---|
| v13-M1 / v12 / v11's own row | 17,333,845 | **-8,574,776 (49.5% off)** |
| naked xz -9 | 16,739,920 | **-7,980,851** |
| xz + par2, the armored card's standing row | ~17,202,144 | **-8,443,075** |
| the python proof's reference point | 8,936,616 | **-177,547** |
| the charter plan's M2 gate, "at or under 6.5 MB armored" | 6,500,000 | **+2,259,069 -- MISSED, and it was called as a miss before the code was written** |

The row that stood in the armored-vs-armored card three times -- v11 by 0.53 pt,
v12 by 0.20, v13-M1 by 0.20 -- is now **8,759,069 against xz+par2's ~17,202,144**.
It does not close by a fraction of a point. It closes by 1.96x.

**The internal 6.5 MB target is missed and the arithmetic says why**: the values
alone are 5,248,812 and the recipe is 3,505,430. 6.5 MB would need the recipe at
1.25 MB -- a 64% cut -- and the only lever with that much in it is reflate's
reference-parse deviation coding, which this brief made optional and which was
NOT built. The two cheap approximations of it that WERE built and measured (the
rep-distance model, the byte planes) are both dead, with numbers, in the section
above. That is the honest state of the lever, printed rather than promised.

## M2 MEASURED (2026-09-03 12:12) -- THE DEFLATE SUITE: 42 files, 0 LOST, 0 WRONG

`node tools/deflatesuite.js` over 42 files. Provenance is recorded per file in the
suite's own `suite.txt` and summarised here. **Every one restored EXACT.**

| class | files | provenance | verdict |
|---|---|---|---|
| gzip, python zlib levels 1..9 | 9 | `zlib.compressobj(lv, DEFLATED, 31)` on 600 KB of vim-version9.txt | 9 EXACT |
| gzip, zlib strategies | 4 | FILTERED, HUFFMAN_ONLY, RLE, FIXED at level 9 (FIXED makes btype-1 blocks) | 4 EXACT, **2 PEELED** |
| gzip, **GNU gzip** -1/-6/-9 | 3 | `C:\Program Files\Git\usr\bin\gzip.EXE`, a different encoder from zlib | 3 EXACT |
| zlib member (wbits 15) | 1 | python zlib | EXACT |
| bare deflate stream (wbits -15) | 1 | python zlib, no wrapper at all | EXACT |
| level 0 (stored blocks only) | 1 | python zlib level 0 | EXACT, **PEELED** |
| a 512-byte window (wbits 25) | 1 | python zlib | EXACT, **PEELED** |
| an empty member | 1 | python zlib on zero bytes | EXACT |
| a binary payload | 1 | 300 KB of notepad.exe, level 9 | EXACT |
| **PNG, written here** | 4 | hand-built, zlib strategies DEFAULT/FILTERED/RLE, and one with **5 IDAT chunks** | 4 EXACT, all four PEELED and then passed over by the argmin |
| **PNG, off the machine** | 8 | 3 Dell/Alienware wallpapers (3.4-4.1 MB), 3 from `ImmersiveControlPanel`, `IdentityCRL\WLive48x48.png`, `IME\IMEJP\Assets\JpnImeModeToast.png` | 8 EXACT, all peeled and passed over |
| **Windows Explorer zips** | 2 | PowerShell `Compress-Archive -CompressionLevel Optimal` (the same .NET ZipArchive Explorer writes), one entry and two | 2 EXACT, **not nominated** -- see the deviation below |
| hostiles | 6 | truncated, corrupt table, trailing junk, bad CRC, two members concatenated, header-only | 6 EXACT |

**42 files: 42 EXACT, 0 WRONG, 0 LOST. 4 took the peeled form; 3 refused the peel
with a printed reason; 35 peeled (or were not nominated) and were passed over by
the argmin, keeping their bytes.**

The three printed refusals are worth reading, because each is the law working:

- `hostile-truncated.gz`: *the deflate stream ends inside a code*
- `hostile-trailing-junk.gz`: *the deflate stream used 131,030 of the member's 131,042 body bytes*
- `hostile-two-members.gz`: *the deflate stream used 131,030 of the member's 262,078 body bytes* -- a legal multi-member gzip, refused because this peel is ONE member and half a peel is a guess

`hostile-corrupt-table.gz` and `hostile-bad-crc.gz` did NOT refuse: the corrupted
bytes still parse as a (different) legal member, and the peel does not check the
gzip CRC because it does not need to -- it reproduces the bytes it was given,
whatever they mean, and the FNV-64 gate is what says they are the right ones.

**Where the peel WINS, and it is a shape, not an accident.** Only four files took
the peeled form: `zlibgz-huffonly.gz` (367,647 -> 82,181), `zlibgz-rle.gz`
(359,545 -> 89,604), `stored-only.gz` (600,073 -> 80,640) and `smallwindow.gz`
(242,035 -> 163,473). Every one is a file where **deflate spelled the value badly**
-- Huffman-only, RLE-only, stored, or a 512-byte window. Where zlib at level 9 did
its job the recipe costs more than the peeled values save, and the argmin keeps the
bytes; on `zlibgz-L9.gz` the peel measures recipe 75,024 + values 53,422 = 128,461
against the raw form's 87,698, and it is correctly passed over. **That is the
charter's judgement rule doing exactly what it was written for.**

The PNG peel is real and is exercised: `mkpng-default.png` peels to *PNG IDAT:
4 blocks, 41,317 tokens, 57,720 B inflated*, `mkpng-multiIDAT.png` to the same
parse across **5 IDAT chunks**, and `win-07-JpnImeModeToast.png` to *3 blocks,
83,078 tokens, 4,147,920 B inflated*. All of them re-spell byte-exactly and are
then passed over by the argmin, because a PNG's filtered scanlines are already
what deflate is good at.

## M2 MEASURED (2026-09-03 12:15) -- audit and tests, on the final build

`eggv13 audit --full`: **3,091,667 checks, 0 failing** -- identical to M1's and to
v12's, which is what it must be: M2 changed the armor's WORDS and not its
arithmetic. Two of its own lines are the independent corroboration of the M2(0)
finding, and neither was written for it:

- (h) *640 trials with e > floor(t/2) unnamed errors over 12 geometries;
  **490 exact (e <= t-1, located jointly)**, 144 refused by the syndromes (e >= t),
  1 rank trap refused honestly (named: exact), **0 settled wrong***
- (g) *672 trials beyond capacity ... 529 refused with a number, 23 caught by the
  hash, 8 exact by rung C, **wrong data 0***

The audit has been asserting the joint reach of `t-1` since v12-M2b. It was right;
the promise's word CERTAIN at `t/2` was the thing that was wrong.

`cargo clippy --all-targets -- -D warnings`: **clean** under rustc 1.98.
`cargo test --release`: **32 passed, 0 failed** (M1: 28). The four new ones are the
save's skin re-spelling bit for bit, the recipe blob's round trip, the hostile
refusals, and the deflate peel's whole pipeline (peel -> THE LAW -> trial -> armor
-> dearmor -> re-spell), which also asserts that a member with junk after it is
refused rather than half-peeled.

## M2 deviations from the brief, and why (2026-09-03 12:18)

1. **The bespoke four-stream context model was NOT built.** The brief asked for
   the streams to be modelled; the measurement (the section above, taken before
   any model code) says the ordinary roster already beats xz -9 on every one of
   the four, and that the arm which wins the distances is the roster's only LZ
   arm. A hand-written context model with no match model would have had to beat
   it at long-range repetition -- the one thing this stream is made of.
   `MODEL_DRECIPE` is the sectioned roster instead: four sections, four
   independent trials, four model bytes. Filed with the numbers that caused it.
2. **The reference-parse deviation lever (reflate's own method) was NOT built.**
   The brief made it optional and asked for the miss to be printed if it did not
   pay. Two cheap approximations of it were built and measured, and both are
   dead with numbers: rep-distance coding (only 9.3% of distances repeat any of
   the last four) and byte-plane splitting (2,596,091 against 2,374,936 plain).
   The full lever -- a matcher of our own over the 296 MB of inflated bytes, and
   then storing only where the encoder disagreed -- is not measured either way.
   What it would have to deliver to reach the charter's 6.5 MB is arithmetic:
   the values are 5,248,812, so the whole recipe would have to fall from
   3,505,430 to 1,246,376, a 64% cut, with the distances (2,374,936) the only
   place that much can come from. **Unmeasured, and printed as unmeasured.**
3. **ZIP members are not peeled.** Windows Explorer zips are in the suite and
   both restore EXACT, but they are not even NOMINATED: a zip is N deflate
   members behind N local headers, and the peel frame carries ONE parse per
   file. Peeling them means a per-entry recipe and a container format for the
   list, which is a milestone of its own and not this one. The brief's gate
   asks that every suite file "peels and re-spells BYTE-EXACT or is cleanly
   refused and keeps its bytes"; a zip keeps its bytes, with LOST 0 and WRONG 0.
   Stated rather than quietly counted as a pass.
4. **Nesting stays at depth 1, and the save does not need depth 2.** The brief
   asked for the decision to be stated: the save is ONE gzip member wrapping
   Relic Chunky, and the inflated bytes ARE the values -- there is no second
   member inside to peel. A peel inside a peel's recipe and a peel inside a
   peel's values are both REFUSED with a number, in `restore_peel`. PNG is
   handled without nesting: the IDAT chunk split is part of the wrapper the
   recipe records, not a second peel.
5. **The MB/s display bug that M1 filed for M2 is fixed.** M1 printed
   "`(0 MB/s)`" on every row under about 1 MB/s because the arithmetic was
   integer `src.len() / ms / 1000` in u128. It is a float now and prints
   0.193 MB/s where it used to print 0. Proved not to move a byte: notepad.exe's
   container from the fixed build is **byte-identical** to the pre-M2 build's.

## M2 MEASURED (2026-09-03 12:39) -- the drill battery: 328 passed, 0 failed

`node tools/drills.js` on the final build. **328 passed, 0 failed** (M1's battery
was 273; v12's 257). The 55 new lines are the M2(0) block the FINDING asked for,
and they run on a REAL corpus row -- notepad.exe -- at every one of the five
tiers, not on synthetic containers:

| tier | n | t | certain blind | joint to | the 11 drills at this tier |
|---|---|---|---|---|---|
| 256 | 688 | 18 | 8 | 17 | all PASS |
| 512 | 345 | 10 | 4 | 9 | all PASS |
| 1024 | 174 | 6 | 2 | 5 | all PASS |
| 2048 | 88 | 4 | 1 | 3 | all PASS |
| 4096 | 45 | 3 | 1 | 2 | all PASS |

Each tier runs: the geometry mirror; a **blind 4 KB scratch at five places** (the
first squares, an ordinary unaligned offset, a third of the way in, straddling
the mid site, and over the end site) -- all EXACT; `t-1` whole squares blind ->
EXACT (located jointly); `t` whole squares blind -> HONEST refusal; the same `t`
addressed -> EXACT; and the **rank-1 dependent wound** at `(t-1)/2` -> EXACT and
at `(t-1)/2 + 1` -> HONEST refusal. That last pair is the new promise's own
boundary, asserted from both sides, at every tier.

The whole M1 battery still passes underneath it, including the sixteen JPEG peel
lines and the ancestors (`.egg8` .. `.egg11` through eggv13).

---

# M3 -- THE SITE READINGS

## M3a FILED (2026-09-03, in the plan, BEFORE the fix was written)

### The design being predicted

`deflate.rs`'s `walk` refused an ENTIRE FILE when a 258-byte match used its second
legal spelling -- symbol 284 with 31 extra bits (`LBASE[27]=227`, `LEXTRA[27]=5`)
rather than symbol 285 with none: *"the recipe stores lengths, not spellings"*.
`wubbadub.html:663-666` says record the canonicalisation, do not refuse it.

The fix carries the exception as a **SPARSE FIFTH STREAM** in the recipe blob: an
ascending list of the MATCH INDICES that took the 284 spelling, `u32` each, empty
on every file in this corpus. `HDR` 48 -> 52 for its count; the drecipe section
table 4 -> 5; the re-speller overrides `lsym_of(258)` to 27 for a named match and
lines 619-624 emit 31 in 5 bits unchanged.

**Rejected alternatives, with their reasons, so nobody re-litigates them:**
widening the flags stream to a bit per MATCH costs **+3.2 MB on the save**
(25,661,244 matches) and destroys the byte periodicity its roster arm exploits;
bit 15 of `dists` (storing `dist-1`) is free in size but breaks the
distance-SEQUENCE repetition the only LZ arm in the roster wins on; `lens` has no
spare values -- all 256 are in use for 3..=258.

### The numbers called

| quantity | FILED |
|---|---|
| recipe growth on `aoe4-autosave.sav` | **under 200 B** against a 3,505,430 B recipe |
| that row's movement | **less than 0.01%** |
| every other row | **byte-identical to M2** |
| coverage gain on this corpus | **not measurable** -- no file here uses the 284 spelling; it must be shown on a CONSTRUCTED file |
| the alarm | any row moving more than **0.05%** means something else changed, and the miss is printed |

## M3b FILED (2026-09-03, BEFORE ONE LINE of transpose, gcd or bit-period code)

Three site readings that were PROBED and never tried. A probe is one instrument,
one shot, no competition -- it is a reason to run a trial, not a verdict
(Vladimir's correction, 2026-09-03). Each of the three either wins somewhere and
stays, or loses everywhere and is **deleted with its miss printed**.

### S1a the transpose, as a real filter

Source: `wubx.html:393-395` ("Transposing the rectangle re-reads it"), scoring
rule at `wubx.html:380`. Blosc takes its stride from a declared type size; **this
page takes it by measuring**, and that is the only novel part. Built as **filter
id 15**, length-preserving (transpose the largest `m*s` prefix, pass the tail
remainder through), param = the stride, composed inside one id like `ms1_apply`.

**FILED PREDICTION: it loses on 20 of 20 and is DELETED.**

Reason, measured before the code: under xz the transpose wins big (real-test.bmp
-86.45% at stride 3, ring01 -18.43% and alarm01 -15.61% at stride 4) and under
**our** model it loses badly -- 248,050 -> 272,456, 130,753 -> 188,338, and
256,462 -> **3,603,901**, a 14x loss. The cause is that `mix12.rs` already carries
the **lattice** (`lat1s`/`lat2s`, `mix12.rs:363-364`, claimed at 436/439, two of
the twelve mixer inputs at `mix12.rs:45`), which reads down a learned stride
*without* reordering the bytes, so it keeps the local context too; transposing
throws the local context away to buy what the lattice already provides.

**The one place I expect to be surprised** is real-test.bmp composed with the
existing image handling rather than instead of it. **If any row wins, the win is
kept and this prediction goes on the board as the miss.**

### S1b the gcd, per block and after deltas

Source: `wub.html:322-326`. Whole-file gcd is settled at 1 on all 23 rows at
8/16/32 bits -- that stands and is not re-run. Untested, and the reason it is not
dead: the gcd of a **block**, and the gcd of the **delta stream**, which is where
quantised data actually carries a common factor. Measured per 64 KB block at
8/16/32 bits, before and after each existing filter, as a probe first.

**FILED PREDICTION: gcd > 1 on fewer than 1% of blocks corpus-wide, and on no
block of any row whose form we already win.** Ship only if some row's summed free
bits exceed its recipe cost (one factor per block).

### S1c the bit period, at order 1 and per region

The earlier probe used order-0 entropy per width plus bit autocorrelation, 5 rows,
16 MB sampled. It cannot see structure living at order-1 in a 12-bit symbol space,
and it averaged whole files, which hides a container that packs differently per
section. Redone with **order-1 sequential code length per candidate width, per
1 MB region** -- an adaptive estimator that PAYS for its own alphabet, because
empirical order-1 entropy over 2^w symbols is biased to near zero and would
manufacture a win at every wide width.

**FILED PREDICTION: no region of any row shows a non-multiple-of-8 width beating
the byte reading by more than 1%.** `rdr2-shaders.vkcache` is the row to watch and
the row I expect to stay flat: its bit autocorrelation is 0.504 at every lag from
1 to 512, and 79.5% of its 4 KB windows sit above 7.9 bits/byte.

### The M3b gate

A 20-row ledger (IN GROUPS) with every surviving arm live; each of the three wins
somewhere and stays, or loses everywhere and is deleted with its prediction
printed beside the measurement. Injuries 3x20 EXACT. Ratchet: no row heavier than
its M2 total.

---

## M3a MEASURED (2026-09-03) -- THE SPELLING EXCEPTION, and the twenty rows

### The twenty-row ledger: 19 byte-identical, ONE row +10 B

`node tools/ledger13.js` in SIX GROUPS (12 small in one pass, the big rows in
pairs, the save and the monster alone), `EGG_EXE` = a copy of the M3a build,
`EGG_PRED` = M2's twenty measured totals.

**19 of 20 rows HIT TO THE BYTE. One moved: aoe4-autosave.sav, 8,759,069 ->
8,759,079, exactly +10 B. Injuries 60/60 EXACT. Rows heavier than sealed v11: 0.
Failures: 0. Net vs the sealed v11: -17,837,868** (M2's was -17,837,878; the
whole campaign moved by the ten bytes of one row).

| quantity | FILED | MEASURED | verdict |
|---|---|---|---|
| recipe growth on `aoe4-autosave.sav` | under 200 B | **+10 B** | **HIT**, by 20x |
| that row's movement | < 0.01% | **+0.000114%** | **HIT**, by 88x |
| every other row byte-identical | 19 of 19 | **19 of 19** | **HIT** |
| the alarm (any row > 0.05%) | never fires | **never fired** | **HIT** |

The +10 B is the entire fifth stream on a file that does not use it: 9 B of
drecipe section table (a model byte and two u32 lengths for an empty section)
plus 1 B for the four zero bytes `nresp` adds to `HDR` inside the modelled first
section. The row's inner went 8,754,257 -> 8,754,267, model 24 (PEEL), price
4,812 at the 256-B tier, 1.17x floor, `E/E/E`.

### The deflate suite, rebuilt: 27 files, 27 EXACT, 0 WRONG, 0 LOST

M2's suite directory was scratch and is gone, so it was rebuilt from python zlib
(levels 1-9, four strategies, zlib/bare/stored/small-window/empty/binary), six
hostiles, and **two CONSTRUCTED 284-spelling members**. Every file restored
EXACT; the three refusals are the same three the law printed at M2.

**The fifth stream's price, measured on the four files that take the peel:**

| file | M2 | M3a | the fifth stream |
|---|---|---|---|
| zlibgz-huffonly.gz | 82,181 | 82,193 | **+12 B** |
| zlibgz-rle.gz | 89,604 | 89,614 | **+10 B** |
| smallwindow.gz | 163,473 | 163,482 | **+9 B** |
| stored-only.gz | 80,640 | 80,644 | **+4 B, and it stopped taking the peel** |

`stored-only.gz` is the one file the change moved in kind, not only in size: its
peeled and raw forms were within a few bytes of each other and the ~13 B of
recipe growth flipped the argmin to the raw form. That is the judgement rule
working -- but it is a MOVE, and it is printed. +0.005% of that file.

### What the fix bought, on files that could not be read at all before

- `constructed-284.gz`, 125 B: *1 blocks, 102 tokens (3 matches, **2 spelled
  284**), 873 B inflated; recipe meta 5 + flags 13 + lens 3 + dists 6 +
  **resp 8 B*** -- EXACT, correctly passed over by the argmin at that size.
- `constructed-284-many.gz`, 932 B: *1 blocks, 600 tokens (300 matches,
  **200 spelled 284**), 77,700 B inflated; ... + **resp 800 B*** -- **PEELED**
  and restored EXACT. The 800 B spelling list rode its own roster arm inside the
  sectioned recipe: 1,850 B -> 254 B (model 26), inner 450 B.

Before M3a both refused outright: *"a 258-byte match spelled with symbol 284: the
recipe stores lengths, not spellings"*, and the whole file was kept raw.

### The gate, item by item

- `cargo clippy --all-targets -- -D warnings`: clean.
- `cargo test --release`: **35 passed** (M2's 32). The three new ones are
  `the_second_spelling_of_258_round_trips` (the parse, the blob, and three
  hostile spelling lists -- unascending, past the match count, and one naming a
  match that is not 258 -- each refusing with a reason),
  `the_spelling_list_is_empty_on_an_ordinary_member`, and
  `respelled_258_member_full_pipeline_round_trip` (the five-section recipe and
  the whole container).
- `eggv13 audit --full`: **3,091,667 checks, 0 failing** (3,925 ms).
- Ancestors: **0 files newer** than `codegg-v13/Cargo.toml` outside `target/`
  across codec-v1 and codegg-v1..v12. No tracked site file is modified.

---

## M3b MEASURED (2026-09-03) -- the three probes, tried

### S1a the transpose -- **PREDICTION HIT. DELETED.**

**FILED: it loses on 20 of 20 and is deleted.**

Built as filter id 15, length-preserving, stride MEASURED by the page's own run
count (`wubx.html:380`) rather than declared. The nomination readout, printed by
`eggv13 probe` on all twenty rows: **the run count nominates a stride on 15 of
20**, and the price prune cuts it to **6 that reach the full trial** --
alarm01.wav 15:2, ring01.wav 15:4, segoeui.ttf 15:12, real-test.bmp 15:3,
iconcache48.db 15:4, rustc_driver.dll 15:2.

**The forced form, on every row that nominates it:**

| row | stride | as it came | transposed | miss |
|---|---|---|---|---|
| alarm01.wav | 2 | 248,050 | 276,536 | +28,486 (+11.48%) |
| ring01.wav | 4 | 130,753 | 188,338 | +57,585 (+44.04%) |
| segoeui.ttf | 12 | 404,871 | 628,553 | +223,682 (+55.25%) |
| rustc_driver.dll | 2 | 37,430,630 | 51,189,263 | +13,758,633 (+36.76%) |
| iconcache48.db | 4 | 413,511 | 616,907 | +203,396 (+49.19%) |
| real-test.bmp | 3 | 256,462 | 3,603,901 | +3,347,439 (+1,305.24%) |

**Six of six lose. The other fourteen never nominate it.** real-test.bmp -- the
one place the prediction said it expected to be surprised -- lost by 14x, which
is the exact number the prediction had filed from the earlier probe.

**The trial through the shipped argmin**, which is the part a forced number
cannot answer, because stage 1 picks the filtered form under the CHEAP v8 cost
and a nominated arm can in principle steal that slot and then lose in stage 2:

- a 4-row ledger with the arm live (alarm01, ring01, segoeui, real-test.bmp):
  **4 of 4 HIT to the byte, injuries 12/12 EXACT, 0 rows heavier**;
- and A/B on the same four rows, the M3a build against the M3b build: the
  containers are **BYTE-IDENTICAL**. Winners filter 14:2, 8:2, 0:0, 5:6000 --
  never 15;
- iconcache48.db with the arm live: **418,323, unchanged**, winner filter 0:0;
- rustc_driver.dll with the arm live: **37,436,978, unchanged**, winner filter
  10:0 (BCJ), model 22, inner 37,430,630.

**All six rows that nominate the transpose are confirmed unchanged through the
shipped argmin**, including the two where stage 1's cheap-v8 trial had the most
room to pick wrong. The arm was offered everywhere it could be offered and
refused everywhere.

The reading was right that transposing re-reads the rectangle -- the run count
falls by a third to two-thirds on most rows. It is the wrong thing to buy:
`mix12.rs` already carries the **lattice** (`lat1s`/`lat2s`, two of the twelve
mixer inputs), which reads down a learned stride WITHOUT reordering the bytes,
so it keeps the local context too. The transpose throws local context away to
buy what the lattice already provides.

**Filter id 15, `xpose_apply`/`xpose_undo`, `runs_of`, `xpose_nominate`, the
apply/undo arms, the property-test rows and the drill cases are deleted.
`FILTER_MAX` is back to `FILTER_MS2`.** Kept: `probe`'s new NOMINATED line, which
prints what the full trial actually sees -- the trace showed a candidate list the
trial never saw, and that was wrong before the transpose was ever written.

### S1b the gcd -- **first clause HIT, second clause MISSED. No arm.**

**FILED: gcd > 1 on fewer than 1% of blocks corpus-wide, and on no block of any
row whose form we already win. Ship only if some row's summed free bits exceed
its recipe cost.**

Every 64 KB block of all twenty rows, at 8/16/32 bits, before and after every
filter form -- 242,559 block-readings:

| | MEASURED | verdict |
|---|---|---|
| blocks carrying gcd > 1 | **1,716 = 0.7075%** | **HIT** (under 1%) |
| of those, ARITHMETIC (odd factor, not a constant run) | **274 = 0.113%** | |
| rows carrying an arithmetic factor at all | **2**: real-test.bmp, rustc_driver.dll | **MISS** -- we win the form on both |

**The sharpening that made the reading honest, and it is the finding worth
keeping: a power-of-two gcd is not a finding.** It says only that the low k bits
are constant zero, and every bitwise model in this house already codes a constant
bit at ~0. On the first pass real-test.bmp read **24.32%** of blocks "carrying a
factor"; once power-of-two factors and constant runs are removed it is **3.86%**,
and corpus-wide 0.71% becomes 0.113%.

The filed ship rule said the free bits cleared the recipe cost, so the trial ran
on both rows that qualified:

| row | reading | blocks divided | claimed free bits | inner as it came | inner divided | verdict |
|---|---|---|---|---|---|---|
| real-test.bmp | plain @8 bit | 73 / 184 | 11,108,401 (1.39 MB) | **256,462** | **256,503** | **+41 B**, before its 315 B recipe |
| rustc_driver.dll | filter 2:0 @32 bit | 16 / 2,795 | 2,098,626 (262 KB) | **56,528,854** | **56,548,012** | **+19,158 B**, before its 414 B recipe |

**Both trials lost, the second by an order of magnitude more than the first.**
The "free bits" a divider would save against a memoryless coder are worth less
than nothing against a context model that already predicts what the factor
encodes. **No arm is built.**

### S1c the bit period -- **PREDICTION MISSED. No arm. The control is what killed the reading.**

**FILED: no region of any row shows a non-multiple-of-8 width beating the byte
reading by more than 1%. rdr2-shaders.vkcache is the row to watch and the row I
expect to stay flat.**

Order-1 sequential code length (Laplace, alpha = 1/2 -- an estimator that PAYS
for its own alphabet, because the empirical order-1 entropy of 2^w symbols over a
1 MB region is biased to near zero for w >= 12 and would manufacture a win at
every wide width), widths 1..16, 1 MB regions, up to 16 evenly spaced per file,
all twenty rows. **Two readings:**

- **naive** -- the context is the previous SYMBOL, so a 12-bit symbol gets 12
  bits of history and an 8-bit symbol gets 8;
- **controlled** -- the context is pinned at the previous **16 bits** for every
  width, so the only thing that varies is where the symbol boundary falls, which
  is what "bit period" actually means.

| widths 9..15 | naive | CONTROLLED |
|---|---|---|
| (row, width) pairs beating the byte reading by > 1 pt | **19** | **1** |

| row | width | naive | controlled |
|---|---|---|---|
| cbs.log | 12 | +11.11 pt | **-92.62 pt** |
| rustc_driver.dll | 12 | +4.87 | **-25.61** |
| msgraph.dll | 12 | +8.85 | **-13.05** |
| aoe4-autosave.sav | 12 | +7.46 | **+0.23** |
| rdr2-shaders.vkcache | 14 | +1.66 | **+0.19** |

**The named row holds under the honest instrument and misses under the sloppy
one.** rdr2-shaders.vkcache: 0 of 16 regions above 1 pt under the control, best
+0.214 pt at width 12; under the naive reading, 15 of 16.

**The prediction MISSES as filed**, and exactly how: 19 of 20 rows have SOME
non-multiple-of-8 width above 1 pt under the control -- but the winner is width
**4** on fifteen of them and width 1 or 2 on the rest. Sub-byte symbols, not wide
packings; that is model granularity, and our coder is already bitwise. The one
wide survivor is real-test.bmp at width 12 (+14.6 pt) -- and the same row prefers
width **1** by +94.8 pt, so 12 is not that file's period either.

**No non-byte PACKING appears anywhere.** The bit period is dead, and it was the
control that killed it: what the earlier probe measured was context length.

---

## M3 gate, and the deviations (2026-09-03)

- `cargo clippy --all-targets -- -D warnings` clean; `cargo test --release`
  **35 passed**; `eggv13 audit --full` **3,091,667 checks, 0 failing**; the
  rebuilt deflate suite **27 EXACT, 0 WRONG, 0 LOST**.
- `node tools/drills.js`: **355 passed, 0 failed** on the final build (M2's
  battery was 328; it read 361 with the transpose still in the tree, and lost
  the two transpose cases with it). The battery grew a **FILTER BATTERY** -- drills had ZERO filter
  coverage while `info` has emitted `"filter"`/`"param"` since v9. Every id the
  header can carry is now forced through the whole pipeline: `info` must name the
  filter and param it was given, the pristine restore must be EXACT through the
  undo, and a blind 1-byte flip must be EXACT through the undo.

**Deviations, named so nobody has to find them:**

1. **`src/sites.rs` SHIPS although neither reading produced an arm.** Both are
   INSTRUMENTS (`eggv13 gcdprobe`, `eggv13 bitprobe`), not dormant arms, and they
   carry the two controls that did the killing -- the power-of-two split and the
   pinned 16-bit context. Deleting them would delete the only executable proof of
   the findings. `probe`, `audit` and `--stats` are the precedent. **Overrule
   this and they go.**
2. **The M3b ledger ran on 4 of the 20 rows, not 20.** The other sixteen are
   covered by the completed M3a 20-row ledger plus the nomination readout: the
   transpose reaches the trial on only six rows, the other fourteen have
   byte-identical candidate lists either way, and ALL SIX were verified unchanged
   directly (four byte-identical containers; iconcache48.db and rustc_driver.dll
   by total, 418,323 and 37,436,978).
3. **The deflate suite is 27 files, not M2's 42.** M2's suite directory was
   scratch and no longer exists; this one was rebuilt from python zlib plus the
   two constructed members. The PNG classes and the Explorer zips are not in it;
   they belong to M3d's second arena.
4. **`tools/ledger13.js` hung again, and the post-mortem needs correcting.** This
   run printed EVERY row and its summary line and THEN failed to exit: PID alive
   46 minutes, temp directory already removed, no child process. All four rows
   had completed and the numbers were sitting in the pipe; killing node flushed
   them intact. **The signature to trust is "no exit", not "no progress", and the
   fix is to kill it and read what it already wrote -- the measurement is not
   lost.**

---

## M3c FILED (2026-09-03, BEFORE ONE LINE of context, scatter, digit or 2D code)

Four levers, in the plan's value order. Each is filed with the plan's range AND
with a **sharpened reading of my own**, because on two of them the arithmetic of
the design says something the plan's range does not, and a prediction that
cannot miss is not a prediction. Both columns get judged.

### The instruments this milestone needs, named before they exist

`EGG_JSTATS=1` -- a coefficient census printed on stderr by the JPEG peel
(blocks, nonzeros, codings per context class). It is an INSTRUMENT, like
`EGG_ARMS` and `EGG_PEEL`, not an arm: it prints what the model already walked.
Filed because the dilution arithmetic below is conditional on a count I do not
have yet.

### S2a -- the JPEG coefficient contexts, sharpened

**The design.** `mag`'s index (`jcoef.rs:362`) gains the block's running nonzero
count `nzb` and the quantisation bucket `qb[k]`; `mbits`'s index (`368`) gains
the two neighbour magnitude buckets `ba`/`bl`. Three places each, exactly as the
plan names them: the `vec![Pr::new(); ]` line, its doc dimension, and one index
expression. `walk<C: Coder>` is one routine, so the decoder mirror is free.

Table arithmetic, filed: `mag` 12,288 -> 3*16*4*4*4*4*16 = **196,608** `Pr`;
`mbits` 12,288 -> 3*16*16*4*4*16 = **196,608**. The model goes 53,544 -> 422,184
`Pr` = **1,688,736 B**, from ~209 KiB to **1.61 MiB**. Well inside the budget.

| | called |
|---|---|
| **the plan's range** | **-1.5% to -4%** of wallpaper.jpg's 1,233,099 modelled values, i.e. 1,233,099 -> **1,183,775 .. 1,214,603** |
| **my sharpened reading** | **the top half of that range at best, and a real chance of a LOSS on `mag`** |

**Why I am filing against the plan's own range.** Both indices are multiplied by
**16**. wallpaper.jpg is 432,000 blocks (55,296,000 B / 128) and its coded output
is 9.86 Mbit, ~22.8 bits per block, which puts the nonzero count at roughly 1.3M
-- call it O(1M) `mag` codings. Spread over 12,288 contexts that is ~100 codings
each; spread over 196,608 it is **~6**. `Pr` starts at 32,768 with a
count-adaptive rate and there is **no mixer in this model** (a deliberate v13-M1
choice, printed at `jcoef.rs:15-17`), so a context that has seen six bits pays
for its own ignorance with nothing to fall back on. `mbits` is the safer of the
two: its codings are the mantissa bits of the same nonzeros, more per symbol.

So, filed separately and each judged:

- **S2a-1 `mag` += (nzb, qb[k]) alone**: called **-0.5% to -2%**, and I give it
  **one chance in three of LOSING outright**.
- **S2a-2 `mbits` += (ba, bl) alone**: called **-0.5% to -1.5%**, a loss unlikely.
- **S2a-3 both together** (the plan's lever): called **-1.0% to -3.0%**, i.e.
  1,233,099 -> **1,196,106 .. 1,220,769**. **The plan's -4% floor is called as
  OUT OF REACH without a mixer or a count-gated fallback.**
- If the plain form misses low, the diagnosis is dilution and the answer named
  in advance is a **count gate** (fall back to the coarse context while the fine
  one is cold), which is a SECOND lever and gets its own filed line before it is
  written.

**Decoder-legality, restated as the gate:** every added input (`nzb`, `qb[k]`,
`ba`, `bl`) is a value the decode direction already holds -- the running count of
this block's nonzeros, the quantisation table from the recipe, and coefficients
in the left/above blocks, which are complete before this block starts. Nothing
derived from `v`, `want_d` or `want_m` may enter. The round trip on
wallpaper.jpg plus the 60-file JPEG suite is the proof.

### S2b -- the MCU sub-stream scatter

Source: `spectrometer.html:464-466` -- disjoint regions summed independently,
every symbol carrying its index `s.i`, so the merge is exact.

**A CORRECTION TO THE PLAN, filed before the code and testable:** a scatter that
is only a **re-ordering** of the same decisions over the same contexts is
**provably free**. Each `Pr` counter sees its own observations in the same
relative order whichever way the file is walked, and an arithmetic coder's length
is the sum of `-log2(p)` at each event, so permuting the events globally changes
nothing but the flush. Our peel **already** scatters per component: `encode`
(`jcoef.rs:397-404`) walks whole component planes, not the MCU interleave, and
`cc` already separates their contexts. **A band-order rewrite of the same model
is therefore called at 0.00% +- one byte of coder flush**, and I will measure it
as the CONTROL before building anything on top of it.

**What the scatter can actually buy, and the arm that is being built:** an order
that makes a context LEGAL which was not legal before. Three passes per plane
instead of one:

- pass A codes the `last` plane for every block;
- pass B codes the DC plane, and may now see `last` at the block to the RIGHT
  and BELOW;
- pass C codes the ACs, and may now see the DC and `last` of RIGHT and BELOW.

That is a two-sided activity estimate where today the model has a one-sided one.

| | called |
|---|---|
| the plan's range | **-0.5% to -2%** |
| my sharpened reading | **the re-order alone: 0.00%. The new contexts: -0.3% to -1.5%** |

Kept only alongside S2a, since they share the context machinery.

### S2c -- the field split for numbers spelled as digits (WS-N)

Target `embeddings.json` (corpus-fmt, 15,801,527 B; 1,275,904 float literals,
91.1% digits). A NEW ROSTER ARM, `MODEL_NUM` (**27**, the next free id after
`MODEL_DRECIPE` 26), in a new `src/numtext.rs`: bytes in, bytes out, byte-exact,
**contexts not a peel**. The text is parsed as it is coded -- sign, integer part,
fraction, exponent, digit position within the field, the field's index inside its
array, and **the aligned digit of the previous value in the same array position**
-- and those become model contexts. Encoder and decoder are ONE routine through a
`Coder` trait, `jcoef.rs`'s shape, so the two sides cannot drift.

| | called |
|---|---|
| the plan's range | **-3% to -10%** of 4,687,145, i.e. **140,614 .. 468,714 B** |
| my sharpened reading | **-2% to -8%**; the arm wins the row or it is deleted |

Measured support already on the record: re-spelling 400,000 literals as raw f32
costs 1,391,012 against the text's 1,654,648, **-15.9%** under a generic coder --
but that is a peel's number, not a model's, and a model that must emit the digits
back cannot collect all of it.

**The ship rule, filed:** the arm ships only if it wins `embeddings.json`'s row on
the ARMORED total through the ordinary argmin, and costs **zero bytes** on all
twenty sealed rows (it is one more roster entrant; an entrant that loses is free).
Called: **it is nominated on `embeddings.json` and on `data.csv`, and wins
nowhere in the sealed twenty.**

### S2d -- the second dimension (WS-2D)

A NEW ROSTER ARM, `MODEL_2D` (**28**), in a new `src/twod.rs`: a bitwise CM whose
contexts are the byte one PIXEL to the left, one ROW above, above-left and
above-right, plus the channel phase -- the stride and pixel size MEASURED by the
encoder and carried in the arm's own stream header, so the decoder reads them
rather than guessing. `iconcache48.db` is 10,526 raw 48x48 BGRA bitmaps: stride
192, pixel 4.

**Why this is not the transpose that died at M3b, and not the lattice either:**
the transpose reordered the BYTES and threw local context away; the lattice
(`mix12.rs:363-364`, two of the twelve mixer inputs) reads down ONE learned
stride and keeps local context but never forms the JOINT context (above, left,
above-left) that every lossless image coder is built on. This arm forms it.

| | called |
|---|---|
| the plan's range | **-5% to -15%** of that row's 413,511 B inner |
| my sharpened reading | **-4% to -12%**, and `real-test.bmp` moves too: **-2% to -10%** of its 256,462 |

**The ship rule, same as S2c:** it wins a row on the armored total or it is
deleted. Called: **nominated on `iconcache48.db`, `real-test.bmp` and
`photo.bin`; wins on the first two.**

### The M3c gate

One lever at a time; the JPEG rows re-measured on every jcoef change plus the
60-file conservation suite; a 20-row ledger (IN GROUPS -- 12 small in one pass,
the big rows in pairs, the save and the monster alone) before the milestone
closes; every filed range printed beside its measurement; **no row heavier than
its M3b total**; injuries 3x20 EXACT; clippy clean, `cargo test --release` green,
`node tools/drills.js` green.

## S2a MEASURED (2026-09-03) -- the plan's lever, and both filed ranges MISS

`wallpaper.jpg`, the modelled values, baseline **1,233,099 B**. Every variant is
the same build with one index expression changed; the census prints the table.

| variant | values | vs baseline | |
|---|---|---|---|
| baseline (M3b) | 1,233,099 | -- | |
| **S2a-1 `mag` += (nzb, qb[k])** | **1,228,151** | **-4,948 = -0.401%** | |
| S2a-1a `mag` += nzb ALONE | **1,228,137** | **-4,962 = -0.402%** | **the best form** |
| S2a-1b `mag` += qb[k] ALONE | 1,233,113 | **+14 = a LOSS** | |
| **S2a-2 `mbits` += (ba, bl)** | 1,234,092 | **+993 = +0.081%, a LOSS** | |
| **S2a-3 both** (the plan's lever) | 1,229,144 | **-3,955 = -0.321%** | |

**Both filed ranges MISS, and by a wide margin:**

| | filed | measured | verdict |
|---|---|---|---|
| the plan's range for the lever | -1.5% to -4% | **-0.321%** | **MISS**, low by 4.7x |
| my re-filed range (after the census) | -1.5% to -4% | **-0.321%** | **MISS**, the same |
| my ORIGINAL sharpened reading | "the top half at best, and a real chance of a LOSS on `mag`" | mag WINS, `mbits` LOSES | **half right, and right about the wrong table** |
| "`qb[k]` may be redundant with `kb`" (filed as the loss risk) | -- | **+14 B: it is worth nothing at all** | **HIT** |

**What the measurement says, and it is worth more than the bytes.**

1. **The whole of S2a is `nzb`.** Adding the block's running nonzero count to
   `mag` is -4,962 B; the quantisation bucket on top of it is +14 B. `qb[k]` is
   already known: the quantisation table is very nearly a function of the band
   on any ordinary encoder, and `kb` is the band. The plan named two inputs and
   one of them was already in the model under another name.
2. **`mbits` does not want neighbours.** It LOST 993 B. Those are the mantissa
   bits below the leading one of a coefficient whose magnitude class is already
   coded; conditioned on the class they are close to uniform, and the only thing
   a 16x wider table buys is 16x colder counters. The report that named this gap
   ("no neighbours at all") read a hole that is not a hole.
3. **This model is not context-starved. It is architecture-limited.** -0.4% is
   what a whole new context input is worth here. The distance to packJPG/Lepton
   (22-25% off the entropy coding against our 17.2%) is not another index term.

**The speed reading that decides what CAN be built**, measured on the same row:
the roster on the raw JPEG takes **4,528 ms** and the whole row with the peel
takes **4,749 ms**, so `peel_arm` runs BESIDE the roster and is at parity with
it, not hidden under it. A mixer in `jcoef` would multiply its work by 2-3x and
put the row at 9-14 s, i.e. **0.11-0.17 MB/s, under the 0.25 MB/s house floor**.
**The M1 decision to ship without a mixer is re-confirmed by measurement, not by
memory.**

---

## S2a-4 FILED (2026-09-03, BEFORE the code) -- three contexts the measurement points at

Filed because S2a's own result names them: the win came from a context about the
BLOCK's state, and the losses came from wider tables over the same neighbours.

- **(a) `nz` += `how far from the end`**, the bucket of `last - k`. `nz` is the
  single most-coded decision in the model (one per position from 1 to `last` in
  every block) and its context carries the ABSOLUTE band `kb` but nothing about
  the distance to the block's own last nonzero, which the decoder holds before
  the loop starts. Called: **-0.5% to -2.0%**.
- **(b) `mag` += `lbucket(last)`**, the block's activity. Called: **-0.1% to
  -0.6%**.
- **(c) THE COUNT GATE**, the answer named in advance at M3c-filing for exactly
  the failure `mbits` just showed: a fine context whose counter has seen fewer
  than `GATE` observations codes against the COARSE counter instead, and both
  update. It is one branch per decision, no mixer, no stretch, no squash -- it
  stays inside the speed floor. Called: **it turns `mbits`'s +993 B loss into
  -0.0% to -0.5%**, and is worth **-0.0% to -0.3%** on `mag`.

If (c) fails to rescue `mbits`, `mbits` keeps its M1 index and the neighbours are
DELETED from it -- the loss is the finding.

## S2a-4 MEASURED (2026-09-03) -- the block's own shape is the context, and the gate is what pays for it

Every row is `wallpaper.jpg`'s modelled values, one build per line, the same
input, the census printing the table each time.

| build | values | vs M3b | vs the line above |
|---|---|---|---|
| M3b baseline | 1,233,099 | -- | |
| `mag` += nzb (S2a-1a, the only surviving part of the plan's lever) | 1,228,137 | -4,962 (-0.40%) | -4,962 |
| **+ (a) `nz` += `lbucket(last - k)`** | 1,209,822 | -23,277 (-1.89%) | **-18,315** |
| **+ (b) `mag` += `lbucket(last)`** | 1,173,656 | -59,443 (-4.82%) | **-36,166** |
| + (d) the running count widened 4 -> 8 steps | 1,171,715 | -61,384 | -1,941 |
| + (f) the neighbour buckets widened 4 -> 8 steps | 1,169,875 | -63,224 | -1,840 |
| **+ (c) THE COUNT GATE, proportional, KGATE = 4** | 1,159,378 | -73,721 | **-10,497** |
| **+ `mbits` += (ba, bl), behind the gate** | **1,157,567** | **-75,532 = -6.126%** | **-1,811** |

**The whole 60-file JPEG suite, M3b build against M3c build, same tool, same
lanes: 40 peeled rows, values 25,731,725 -> 24,896,039, `-835,686 = -3.248%`,
and ROWS NOT IMPROVED: 0.** The suite gains half of what `wallpaper.jpg` gains
because most of its files are a third the size and fine contexts need data --
which is the honest shape of this lever and is printed rather than averaged
away. 60 of 60 EXACT, 0 LOST, 0 WRONG; the 17 refusals are the same 17.

### The filed ranges, judged

| filed | measured | verdict |
|---|---|---|
| S2a-4(a) `nz` += distance-to-end: **-0.5% to -2.0%** | **-1.49%** (marginal, on its own base) | **HIT** |
| S2a-4(b) `mag` += block activity: **-0.1% to -0.6%** | **-2.95%** (marginal) | **MISS, high by 5x** |
| S2a-4(c) the gate rescues `mbits` from +993 B to -0.0%..-0.5% | **+993 B -> -1,811 B**, a 2,804 B swing | **HIT** |
| S2a-4(c) the gate is worth -0.0% to -0.3% on the rest | **-10,497 B = -0.90%** | **MISS, high by 3x** |
| the plan's S2a range for the milestone: **-1.5% to -4%** | **-6.126%** | **MISS -- the milestone beat it** |
| "the plan's -4% floor is out of reach without a mixer or a count gate" | -6.126%, **with** a count gate and **without** a mixer | **HIT, and it named the mechanism** |

### THE BUG THIS MILESTONE ALMOST SHIPPED A VERDICT ON

The count gate was measured THREE TIMES and the first two readings were both
wrong, in opposite directions, from one line:

```rust
let p = c.p as i32 + (((f.p as i32 - c.p as i32) * w) >> 16);
```

`f.p - c.p` reaches +-65,408 and `w` reaches 65,536: the product is 4.29e9 and
**overflows i32, wrapping silently in release**. The first reading said the
proportional blend LOSES 107,417 B; I wrote that down as a mechanism ("averaging
a locked 0.999 against a 0.9 parent"), switched to a hard step, and the hard
step measured worse still -- 1,327,282 at KGATE=1, and *more* coarse measured
*better*, which is the opposite of what a gate can do.

**The control that caught it was KGATE = 0**, which makes the gate a no-op and
must therefore reproduce the ungated build TO THE BYTE. It read 1,332,991
against 1,169,875. The mechanism I had written down was fiction; the arithmetic
was the fault. With `i64` the same control reproduces 1,169,875 exactly, and the
proportional law then beats the hard step at every threshold (1,159,378 against
1,162,201 at their respective bests).

**The lesson, and it is a house lesson, not a JPEG one: a measurement that
disagrees with the mechanism is not a finding until a control says the
instrument works.** I published a mechanism for a wrapped multiply.

### What DIED at S2a, printed

- `qb[k]` in `mag`: **+14 B**. The quantisation table is very nearly a function
  of the band and the band was already in the index.
- `(ba, bl)` in `mbits` UNGATED: **+993 B**. It survives only behind the gate.
- the block's DC magnitude class in the `last` context: **+92 B**.
- `nz` += `lbucket(last)` on top of the distance: **-343 B for an 8x nz table**
  (786,432 -> 6,291,456 contexts, +22 MB of counters). Refused on price.

### The clock, and the mixer refused again

`wallpaper.jpg` at M3c: **4,677 ms, 0.343 MB/s SOLO**, against M3b's 4,626 ms.
The model grew from 214,176 B of counters to 19,027,104 B and the row did not
slow measurably, because `peel_arm` runs beside the roster and the roster is the
critical path. A mixer was priced against that clock and refused: 2-3x the model
work puts the row at 0.11-0.17 MB/s, under the 0.25 MB/s house floor.

`cargo clippy --all-targets -- -D warnings` clean; `cargo test --release` **35
passed** including `jcoef::tests::coefficients_round_trip` and
`jpeg::tests::corpus_jpeg_round_trips_byte_exact`.

### S2b CORRECTION, filed BEFORE the band-order code

My "provably free" claim for a pure re-ordering is **too strong, and I can say
exactly where it breaks**. The proof requires each counter to see its own
observations in the SAME RELATIVE ORDER either way. That holds for a context
that pins the band exactly (`kbucket` 0..7 are the single positions k = 1..8),
and it FAILS for every bucketed tail: `kbucket` 15 covers k = 48..63, so in
block order the events arrive block-by-block and in band order they arrive
band-by-band, and a count-adaptive counter is order-sensitive.

**Re-filed, before the code: the re-order alone lands in `+-0.3%`, not at
0.00%.** The sign is not called. If it lands outside that band, the miss is
printed and the cause is a context I have not accounted for, not the flush.

## M3a FILED (2026-09-03 17:25, BEFORE ONE LINE of the refusal fix)

Source of the bug: `wubbadub.html:663-666`, the site's own canonicalisation rule
-- *"0,+1 is the same number as 1,-1 and 0,-1 is the same as -1,1, so a pushed
card should have no 0,+-1 pair left anywhere -- not in its operand, and not in
its result."* Canonicalise ambiguity and RECORD it; do not refuse it.

`src/deflate.rs:373-379` does the opposite. RFC 1951 allows a 258-byte match two
legal spellings -- length symbol 285 (LBASE 258, 0 extra bits) and symbol 284
(LBASE 227, 5 extra bits, value 31). The peel refuses the WHOLE FILE when it
meets the second: *"a 258-byte match spelled with symbol 284: the recipe stores
lengths, not spellings"*. That is a shipped refusal on legal deflate, and it was
found by a probe, not by a gate.

### The design being predicted

A **sparse fifth stream**: the recipe records the INDEX (into the match
sequence) of every 258-match that used the 284 spelling, as a u32 each. The
flags stream has no spare room (exactly one bit per token) and the three
alternatives were rejected with reasons, measured or arithmetic:

- widening flags to a bit per MATCH costs +3.2 MB raw on the save (25,661,244
  matches) and destroys the byte periodicity the roster arm exploits;
- stealing bit 15 of `dists` by storing `dist-1` is free in size but breaks the
  distance-sequence repetition MIX12 wins that section on;
- `lens` has no spare values: all 256 are in use for lengths 3..=258.

Sparse costs FOUR bytes per occurrence and, on a file with none, exactly
**nothing but a section-table entry**.

### FILED

| quantity | FILED |
|---|---|
| a CONSTRUCTED gzip using the 284 spelling peels where it previously refused, and re-spells **BIT FOR BIT** | **YES** -- or the fix is not a fix |
| the fifth stream's coded size on `aoe4-autosave.sav` | **under 200 B** against a 3,505,430 B recipe (this corpus is expected to hold **zero** 284 spellings, so the real cost is the section table alone) |
| `aoe4-autosave.sav`'s armored total | moves by **less than 0.01%** of 8,759,069, i.e. by less than 876 B |
| the other NINETEEN rows | **byte-identical**, every one -- no other row takes a deflate peel |
| coverage gain on this corpus | **NOT MEASURABLE.** No file in the 20 rows, the 42-file deflate suite or the 60-file JPEG suite is expected to use the 284 form. The gain must be shown on a constructed file or it is not shown at all |
| **if any row moves more than 0.05%** | something else changed, and the **miss is printed with its cause** |

Surgery, named in advance so a silent divergence is visible: `HDR`
(`deflate.rs:903`) 48 -> 52; `blob_len`; `blob`; `Layout`; `layout()` including
the strict terminal check at 1019-1021; `from_blob`; and on the model side
`secs` (`main.rs:187`), the `out.push(4u8)` literal (204), the capacity hint
(203) and the `n != 4` guard (221). `decode_drecipe`'s loop is already generic
over `n` and gives the new stream its own roster winner for free.

## M3b FILED (2026-09-03 17:25, BEFORE ONE LINE of transpose, gcd or bit-period code)

Three readings are in limbo because verdicts were published from PROBES. A probe
is one instrument, one shot, no competition. Each of the three now either wins
somewhere and STAYS, or loses everywhere and is **DELETED** with its prediction
printed beside the measurement. Vladimir's ruling, 2026-09-03: nothing ships
dormant, no shelving.

### S1a -- the transpose, as filter id 15

Source: `wubx.html:393-395`, *"the long way round is the row width; the short way
is the runs of like colour, read row by row. Transposing the rectangle re-reads
it"*; the scoring rule is `wubx.html:380`, which counts the runs under a reading.
Blosc takes its stride from a declared type size; **this page takes it by
measuring**, and that is the only novel part.

Built as filter id 15, length-preserving (transpose the largest `m*s` prefix,
pass the tail remainder through), param = the stride, composed INSIDE one id in
the `ms1_apply` pattern so the single `filter_id` header byte is untouched.
Strides proposed: {2,3,4,6,8,12,16,24,32,48,64} plus anything `sniff` already
knows, scored by the page's own run count before the trial so the roster is not
flooded.

**FILED PREDICTION: it loses on 20 of 20 and is DELETED.** Reason, measured:
under xz it wins big (real-test.bmp -86.45% at stride 3, ring01 -18.43% and
alarm01 -15.61% at stride 4) and under OUR model it loses badly --
248,050 -> 272,456, 130,753 -> 188,338, and 256,462 -> **3,603,901**, a 14x
loss. The cause is that `mix12.rs` already carries the LATTICE (`lat1s`/`lat2s`,
allocated at `mix12.rs:363-364`, claimed at 436/439, two of the 12 mixer inputs
at `mix12.rs:45`), which reads down a learned stride WITHOUT reordering the
bytes, so it keeps local context too; transposing throws the local context away
to buy what the lattice already provides. **The one place I expect to be
surprised** is real-test.bmp composed with the existing image handling rather
than instead of it. **If any row wins, the win is kept and this prediction is
printed as the miss.**

### S1b -- the gcd, per block and after deltas

Source: `wub.html:322-326`, *"reducing the two rates by their gcd makes them
coprime"*. Whole-file gcd is settled at 1 on all 23 rows at 8/16/32 bits and
that stands. Untested, and the reason it is not yet dead: the gcd of a BLOCK,
and the gcd of the DELTA stream, which is where quantised data actually carries
a common factor. Measured per 64 KB block at 8/16/32 bits, before and after each
existing filter.

**FILED PREDICTION: gcd > 1 on fewer than 1% of blocks corpus-wide, and on no
block of any row whose form we already win.** Ship only if some row's summed
free bits exceed its recipe cost (one factor per block). Otherwise DELETED with
this printed beside the number.

### S1c -- the bit period, at order 1 and per region

The earlier probe used order-0 entropy per width plus bit autocorrelation, on 5
rows, 16 MB sampled. It cannot see structure living at order-1 in a 12-bit
symbol space, and it averaged whole files, which hides a container packing
differently per section. Redone with **order-1 conditional entropy per candidate
width, per 1 MB region**.

**FILED PREDICTION: no region of any row shows a non-multiple-of-8 width beating
the byte reading by more than 1%.** `rdr2-shaders.vkcache` is the row to watch
and the row I expect to stay flat: its bit autocorrelation is 0.504 at every lag
from 1 to 512, and 79.5% of its 4 KB windows sit above 7.9 bits/byte. If no
region beats bytes, there is nothing for a peel to take and the reading is
DELETED with this printed beside the number.

### The M3b gate

A 20-row ledger (run in GROUPS -- the 8-lane pass hung twice on 2026-09-03) with
every SURVIVING arm live; each of the three either wins somewhere and stays, or
loses everywhere and is DELETED with its prediction printed beside the
measurement. Injuries 3x20 EXACT. Ratchet: no row heavier than its M2 total.

## S2b MEASURED (2026-09-03) -- the scatter, and BOTH of my claims about it were wrong

The walk was rebuilt as three passes over each component plane -- pass A the
last-nonzero plane, pass B the DC plane, pass C the ACs band by band -- and the
control was measured before anything was built on top of it.

| build | values | vs the line above |
|---|---|---|
| S2a final (block order) | 1,157,567 | -- |
| **the CONTROL: band order, the SAME contexts, nothing added** | **1,146,007** | **-11,560 = -0.999%** |
| + the DC reads its own block's `last` (pass A -> pass B) | 1,139,922 | -6,085 |
| + the AC SIGN reads the two-sided DC GRADIENT (pass B -> pass C) | 1,122,685 | **-17,237** |
| + that gradient at 9 signed steps instead of 3 | **1,121,398** | -1,287 |

**S2b total: -36,169 B = -3.124%. M3c total on this row so far: 1,233,099 ->
1,121,398 = -111,701 = -9.058%.**

### Both filed claims MISSED, and the second one was mine twice over

| filed | measured | verdict |
|---|---|---|
| "a scatter that is only a RE-ORDERING is **provably free**" (M3c filing) | **-11,560 B** | **MISS** |
| the correction: "the re-order alone lands in **+-0.3%**" (filed before the code) | **-0.999%** | **MISS**, by 3.3x |
| the plan's S2b range: **-0.5% to -2%** | **-3.124%** | **MISS -- the milestone beat it** |
| my sharpened reading: "the new contexts: -0.3% to -1.5%" | **-2.14%** (the two contexts, marginal) | **MISS**, high |

**Why the re-order is not free, which is the finding.** My proof required each
counter to see its own observations in the same relative order either way. I
corrected it once (bucketed bands break the order) and still called the effect
`+-0.3%`. The real mechanism is bigger than the correction: `Pr` is
count-adaptive with `n` capped at NLIMIT = 127, so it TRACKS rather than
averages. In band order a context like `kbucket` 15 (k = 48..63) sees all of
k=48, then all of k=49, and specialises to one k at a time. **The ordering acts
as a soft extra context dimension, for free, and it is worth a full percent.**
A memoryless coder would have been indifferent; ours is not, and that is a
property of the counter, not of the JPEG.

### What the scatter actually bought, and it is one context

The two-sided DC gradient on the AC SIGN is **-17,237 B on its own** -- the
single biggest context in this model after the block's own `last`. AC signs were
the least compressible thing here (1,608,209 of them, one bit each, modelled by
432 contexts), and they are **not** random: the sign of a low-band AC follows the
DC gradient ACROSS the block, which needs the DC to the RIGHT and BELOW. A
block-order walk can never have those. That is what the scatter is for.

Resolution sweep on that gradient, both directions: 3 steps 1,122,685; 5 steps
1,121,678; 7 steps 1,121,549; **9 steps 1,121,398**.

### What DIED at S2b, printed

- the coarse parents of `nz` and `mag` reading right/below `last`: **-24 B** for
  an 8x coarse table. Refused on price.
- the DC reading right/below `last` on top of its own: **+613 B**. A loss.
- the gradient as an ACTIVITY in `mag`'s fine context: **-275 B** for a 4x `mag`
  table (196,608 -> 786,432 contexts, +50 MB of counters). Refused on price,
  the same rule that refused `nz += lbucket(last)` at S2a.

### The row, and the suite

| | bytes |
|---|---|
| wallpaper.jpg | 1,602,752 (entropy-coded 1,602,311) |
| values | **1,121,398** |
| inner | 1,121,685 |
| **armored total** | **1,126,497** |
| against the entropy coding | **-475,814 = 29.70% off** |
| against v12 on the same row (1,513,903) | **-387,406** |
| against the SEALED v11 (1,533,228) | **-406,731** |
| speed SOLO | 4,615 ms, **0.347 MB/s** -- above the 0.25 floor, and FASTER than M3b's 4,626 ms |

**The plan's target for this row was "toward packJPG/Lepton's 22-25%". The row is
at 29.70% off the entropy coding.** That is a claim about our number, not about
theirs: neither packJPG nor Lepton has been run on this file in this house, and
until they are, 22-25% is a remembered figure and nothing more. **They are on
v14's challengers card and that is where the comparison belongs.**

The 60-file conservation suite on the final build: **40 peeled rows,
25,731,725 -> 24,380,500 = -1,351,225 = -5.251%, ROWS NOT IMPROVED: 0**, and
60 of 60 EXACT, 0 LOST, 0 WRONG, the same 17 refusals with the same reasons.
`cargo clippy --all-targets -- -D warnings` clean, `cargo test --release` 35
passed.

## M3d FILED (2026-09-03, BEFORE ONE LINE of prober, chain or arena code)

### THE SCOPE DECISION, stated before the predictions so they can be judged

The plan's second arena names "real ZIPs and DOCXs". Reading the tree to design
against it: `deflate::peel` carries ONE member and a wrap (`WRAP_RAW`,
`WRAP_GZIP`, `WRAP_PNG`, `WRAP_ZLIB`), with `pre`/`segs`/`tail` for the
container skeleton, and `peel::Peeled` carries ONE `Option<Deflate>` and ONE
values stream. **A ZIP is N INDEPENDENT deflate members**, and a DOCX is a ZIP:
reading one needs a peel that holds a VECTOR of members and a skeleton that
interleaves them. That is a new peel id and a change to the peel frame itself,
and it is not a milestone I can land and prove inside M3d beside the ledger and
the seal.

**So it is NOT BEING BUILT, and it is named here rather than discovered
missing.** What M3d builds instead:

- **S3a the prober**, as the plan specifies it -- layout arithmetic, and
  **null, not a guess** -- shipped as an INSTRUMENT (`eggv13 members <file>`)
  that reads a ZIP's central directory and prints every member's offset,
  length, method and name. It is the thing the ZIP peel would be built on, and
  it is the thing that says whether the arena's files are worth a peel at all.
- **S3b the chain**, depth 2: when a peel's values are BYTES and those bytes
  themselves nominate a peel, peel them too. That is the reading exactly -- the
  quotient handed down as the next dividend, each step's recipe sealed where it
  was made.
- **The second arena**, reported apart from the sealed 20 + 3.

**The ZIP container peel is v14's line, and this is its brief:** the prober's
output is the member table; the peel is N `deflate::peel` calls plus a skeleton;
the law is unchanged.

### S3a -- the offset-to-member prober

Source: `atlas.html:355-356` and `461-462`.

**The measured facts this is designed against:** the sealed corpus's only
embedded member is a 72,024 B PNG inside `notepad.exe` at offset 244,808, and
extracted and run standalone it wins nothing (75,904 B armored either way).
`deflate::looks_like_deflate` returns **true with no further check** for block
type 1, so 9 of our 20 rows already take a real peel attempt that fails; a
prober that nominates MORE offsets makes that worse unless it is cheap and
certain -- which is why it reads a DECLARED layout and refuses to guess.

**FILED: no row of the sealed twenty moves by a single byte**, because the
prober is an instrument and touches no arm. If a sealed row moves at all, the
miss is printed.

### S3b -- the chain

Source: `wubdiv.html:371-375`.

**FILED: no row of the sealed corpus moves**, because no row has a member inside
a member -- and the guard that made nesting depth 1 is `main.rs:145-147` /
`159-161`, so the change is visible and bounded. **On a constructed
`something.jpg.gz` the chain must reach DEPTH 2 and restore EXACT**, and that
constructed file is the proof; without it the chain is a claim.

### The second arena

Built on the model of `tools/deflatesuite.js` (LOST must be zero and WRONG must
be zero). Members: real ZIPs and DOCXs from `C:\Users\vcepe\Downloads`, a PDF, a
GIF, a multi-member gzip, the constructed 284-spelling file from M3a, and the
constructed chain file.

**FILED for the arena, and calibrated to what is actually being built:**

- every member is EXACT or cleanly refused with a printed reason -- **0 LOST,
  0 WRONG**;
- **the ZIP and DOCX rows are REFUSED-AND-KEPT**, because no ZIP peel exists;
  they are in the arena to hold the place the v14 brief will fill and to prove
  the prober can read them;
- **the chain reaches depth 2** on the constructed member;
- **the arena's total moves by less than 2%** against the same files' M3b
  totals. Anything more would mean a peel fired where I said none would.

## S2c MEASURED (2026-09-03) -- WS-N, the field split. IT SHIPS.

`MODEL_NUM` (27), `src/numtext.rs` plus one new door in `mix12.rs`. The design
that landed is smaller than the one filed and it is worth saying why: rather
than a new model, the arm is **the shipped CM12 with its two SPARSE inputs
re-pointed** at the number field tracker. Everything else -- the twelve-input
mixer, the match model, the APM, the hashed buckets with their check bytes --
is v12's, verbatim. `Lens::Plain` reproduces CM12 **to the byte**, and that is
the control: three non-numeric rows (kernel32.dll, wubbadub.html,
vim-version9.txt) produce **byte-identical containers** on the new build.

The two keys, both read from bytes already coded:

- **key0, the SHAPE** -- which field (outside / integer / fraction / exponent),
  how far into it, the sign, how long the integer part was, which number of the
  row this is, and the byte just coded;
- **key1, the ALIGNMENT** -- **the digit at the same position of the PREVIOUS
  number**, which is the reading (`wubdiv.html:1184`, `:392-393`: values
  incomparable at their native magnitudes become comparable once scale is
  stripped into its own stream).

### The row

| | bytes |
|---|---|
| `embeddings.json` | 15,801,527 (1,275,904 float literals, 91.1% digits) |
| M3b's winner: CM12H | 4,682,333 inner -> **4,687,145 armored** |
| **NUM** | **4,498,425 inner -> 4,503,237 armored** |
| | **-183,908 = -3.93%** |

The full roster on that row: **NUM=4,498,425**, 2D=4,552,999, CM12H=4,682,333,
CM12=4,685,167, MIX12=5,438,657, CM11=4,928,558, MIX11=5,517,709.

| filed | measured | verdict |
|---|---|---|
| the plan's range **-3% to -10%** (140,614 .. 468,714 B) | **-183,908 B = -3.93%** | **HIT** |
| my sharpened **-2% to -8%** | -3.93% | **HIT** |
| "nominated on `embeddings.json` and on `data.csv`, wins nowhere in the sealed twenty" | nominated on both, **WINS both**; `data.csv` 664,251 -> **652,158 = -1.82%** | **HIT on the nomination, and it wins one more row than called** |

## S2d MEASURED (2026-09-03) -- WS-2D. IT SHIPS, AND EVERY ROW I NAMED WAS WRONG.

`MODEL_2D` (28), `src/twod.rs`. Same shape as WS-N: the two sparse inputs of
CM12 re-pointed, this time at `(above, left-pixel, above-left, above-right,
phase)`. The stride and pixel are MEASURED -- a distance-to-last-identical-four-
bytes histogram, O(n), then the smallest divisor of the argmax that still
carries a third of its count, which is how the row is found rather than the
frame -- and they ride in the arm's own five-byte header so the decoder reads
them and never guesses.

### The rows I named, and what they did

| row | filed | measured | verdict |
|---|---|---|---|
| `iconcache48.db` | **-4% to -12%** of 413,511 | 2D=**451,661** against MIX12's **413,511**. The row does not move. | **MISS** |
| `real-test.bmp` | **-2% to -10%** of 256,462 | 2D wins the PLAIN sub-roster (3,602,881 vs 3,606,660) and the row is won by a FILTERED form at 256,462. The row does not move. | **MISS** |
| `photo.bin` | "nominated" | **not nominated** -- the hunt finds no rectangle in random data | **MISS**, and the right kind |

### The row it actually wins, which I never named

| row | form | M3b | M3c | |
|---|---|---|---|---|
| **`alarm01.wav`** | filtered 14:2 | CM12 **248,050** | **2D 235,196** | **-12,854 = -5.18%** |

**The second dimension is not the image. It is the AUDIO FRAME.** After the
order-2 W16 filter the residue is stereo 16-bit samples, the stride hunt finds
the frame, and "the byte one pixel to the left" becomes *the same byte of the
previous sample in the same channel* -- which is exactly the context an audio
coder wants and which no arm in this house had. `ring01.wav` misses the same win
by **192 B** (2D 130,945 against CM12 130,753 on its filtered form).

**Why `iconcache48.db` refuses it, and this is the finding worth more than the
bytes:** 97,517,568 B goes to 413,511 -- a ratio of 236 to 1. That redundancy is
not two-dimensional at all, it is **whole duplicate icons**, and the arm that
reads it is the LZ match model in MIX12. A 2D context model can only ever
predict a pixel from its neighbours; it cannot say "this entire 9,216-byte icon
appeared before". The reading was right that the file is a rectangle and wrong
about where its redundancy lives.

Also measured, and printed because they are the honest cost of an entrant:
`kernel32.dll` 2D=297,069 against CM12-PE's 284,319; `segoeui.ttf` 2D=408,943
against CM12-TTF's 404,871. The arm is nominated on both and passed over, which
costs one parallel CM pass on each.

## M3d MEASURED (2026-09-03) -- the chain is the milestone, and the prober reads a DOCX

### S3b THE CHAIN -- built, proved, and it is worth 20% of a row

A gzip of a JPEG, constructed for the purpose (`corpus-arena/chain-jpeg.gz`,
920,124 B: `corpus-jpeg/win_Wallpaper_ThemeB_img26.jpg` through python gzip -6):

```
  peel 2: THE CHAIN took depth 2 -- values 896727 B -> 705625 B
  peel 2: recipe 151071 B -> 28803 B (model 26); values 985873 B raw -> 705625 B (model 24)
  round-trip verified in memory before write
  restored 920135 B, conservation hash OK
```

| build | armored total |
|---|---|
| the M3c build, no chain | **925,446** -- *heavier than the input* |
| the M3d build, chain at depth 2 | **739,255** |
| | **-186,191 = -20.12%** |

**And the restore is EXACT.** Without the chain the deflate peel fires, hands
the roster a JPEG, and the roster cannot read a JPEG -- so the argmin keeps the
raw gzip and the row lands at 100.58% of its input. With the chain the inner
peel reads the coefficients underneath the Huffman underneath the deflate.

That is the reading exactly (`wubdiv.html:371-375`): the quotient handed down as
the next dividend, each step's recipe sealed where it was made. Depth is ONE
constant, `PEEL_DEPTH_MAX`, read by both directions, and depth 3 refuses with a
number.

### S3a THE PROBER -- `eggv13 members`, and it says null

On a real Word document off this machine:

```
Topic_1_Test_Cover_Sheet.docx: 12 members (31192 B)
  off        569 len        357 method  8 peel-nominates 2 [Content_Types].xml
  off       1773 len       1501 method  8 peel-nominates 2 word/document.xml
  off       3911 len      16948 method  0 peel-nominates 2 word/media/image1.png
  ... 12 in all ...
  offset 0 is owned by NO member -- null, not a guess
  offset 15596 is owned by member 4 (word/media/image1.png)
  offset 31191 is owned by NO member -- null, not a guess
  11 of 12 members are deflate
```

Every offset comes from the container's own arithmetic -- the central directory
gives the LOCAL header offset, and the body offset is that plus 30 plus the
LOCAL header's own name and extra lengths, which are **not** the central
directory's. On a JPEG it prints *"no container layout this build can read"* and
returns nothing, which is the reading's own rule: return null, not a guess.

### THE SECOND ARENA, MEASURED (2026-09-03) -- 9 members, reported APART from the sealed 20 + 3

`python tools/mkarena.py` builds it (provenance for every member in
`corpus-arena/suite.txt`); `EGG_EXE=<ship> EGG_BASE=<the M3b build> node
tools/arena.js` weighs it.

| member | bytes | -> | verdict | what happened | vs M3b |
|---|---|---|---|---|---|
| `chain-jpeg.gz` | 920,124 | **739,247** | EXACT | **PEELED, CHAINED to depth 2** | **-186,188** |
| `constructed-284.gz` | 298 | 4,970 | EXACT | peel PROVED, passed over by the argmin | unmoved |
| `constructed-284-many.gz` | 917 | 4,997 | EXACT | peel PROVED, passed over by the argmin | unmoved |
| `docx-cover-sheet.docx` | 31,192 | 31,246 | EXACT | no peel nominated | unmoved |
| `docx-newsletter.docx` | 49,092 | 50,852 | EXACT | no peel nominated | unmoved |
| `zip-opencv-python.zip` | 1,149,916 | 1,146,358 | EXACT | no peel nominated | unmoved |
| `zip-lab-guides.zip` | 2,835,244 | 2,841,063 | EXACT | no peel nominated | unmoved |
| `pdf-install-manual.pdf` | 397,522 | 322,207 | EXACT | no peel nominated | unmoved |
| `multimember.gz` | 240 | 4,812 | EXACT | **refused: the deflate stream used 99 of the member's 222 body bytes** | unmoved |
| **ARENA TOTAL** | **5,384,545** | **5,145,752** | **9 EXACT, 0 WRONG, 0 LOST** | | **-186,188 = -3.492%** |

The prober's reading, printed beside each container it can read:
`docx-cover-sheet.docx` 12 members, 11 deflate; `docx-newsletter.docx` 12
members, 12 deflate; `zip-opencv-python.zip` **54 members, 36 deflate**;
`zip-lab-guides.zip` 2 members, 2 deflate.

| filed | measured | verdict |
|---|---|---|
| **0 LOST, 0 WRONG** | 9 EXACT, 0 WRONG, 0 LOST | **HIT** |
| the chain reaches depth 2 on the constructed member | it does, and the restore is EXACT | **HIT** |
| the ZIP and DOCX rows are **REFUSED-AND-KEPT** | they are **NOT NOMINATED AT ALL** -- `peel::nominate` reads offset 0, a ZIP begins `PK\x03\x04`, and `looks_like_deflate` says no | **HALF** -- the outcome is what I called, the mechanism is not. A refusal implies a nomination; there was none. |
| the arena's total moves by **less than 2%** | **-3.492%** | **MISS** |

**The miss is one row.** The arena's entire movement is `chain-jpeg.gz`'s
-186,188, which is exactly the delta printed on that line: every other member is
unmoved to the byte. I called the arena at <2% because I was pricing the ZIP
peel I had already decided not to build, and forgot to price the chain I WAS
building against the same total. -186,188 on a 5,331,940 baseline is 3.49%.

**Two findings the arena bought that the sealed corpus could not:**

1. **A multi-member gzip is refused, with the number.** *"the deflate stream
   used 99 of the member's 222 body bytes"* -- the peel reads one member and
   says so rather than silently keeping the first. That is the law working on a
   case the sealed twenty has none of.
2. **A ZIP is invisible to `nominate`, not refused by it.** 36 of 54 deflate
   members inside `zip-opencv-python.zip` and the peel never looks, because
   nomination reads offset 0. That is precisely the hole `eggv13 members` was
   built to measure, and precisely the hole the v14 ZIP peel fills.

### The deflate suite, rebuilt AS A BUILDER (2026-09-03): 29 files, 29 EXACT, 0 WRONG, 0 LOST

M2's suite directory was scratch and did not survive; M3a rebuilt it and it did
not survive either. It is `tools/mkdeflatesuite.py` now, so the next milestone
does not have to invent one: nine levels, four strategies, a zlib wrapper, a
bare stream, a 512-byte window, memLevel 1, stored-only, empty, one byte, a
periodic binary, six hostiles, and the two constructed 284-spelling members.

**29 EXACT, 0 WRONG, 0 LOST. 3 took the peeled form** (`gz-huffonly.gz`
321,119 -> 73,720; `gz-rle.gz` 313,862 -> 80,193; `smallwindow.gz` 210,341 ->
144,539), **2 refused with a reason** — both hostiles, and both the right reason
(`hostile-flipped.gz`: *the code lengths overran their table by 130*;
`hostile-truncated.gz`: *the deflate stream ends inside a code*) — **and 24
peeled-or-nominated and were passed over by the argmin**, which is the trial
doing its job: a gzip of already-compressed HTML costs more as (recipe + values)
than it does raw.

### A REPORTING BUG THE CHAIN INTRODUCED, found and fixed in the tools

The first run of this suite reported **19 refusals**, including every ordinary
`gz-l1..l9.gz`. They were not refusals. `tools/deflatesuite.js` (and
`tools/arena.js`) scraped the first `peel N: REFUSED` line out of stderr — and
with the chain live, **that line can belong to the CHAIN's inner attempt** on
values that merely look like a deflate stream, not to the row's own peel.

The control: `gz-l6.gz` through the M3b build and through the ship build print
**the same recipe, the same values, the same inner and the same armored total**
(46,062 / 50,874) — the outer peel is untouched; only an extra inner refusal
line appears. Both tools now read the `peel N: recipe` line as proof the outer
peel stood, and say *"peel PROVED, passed over by the argmin"*.

**This is the second time in one milestone that a tool told me a story its own
numbers contradicted.** The first was the i32 overflow in the count gate. Both
were caught by asking what a control should say and checking that it said it.

## M3c / M3d DEVIATIONS, named so nobody has to find them

1. **The ZIP container peel is NOT BUILT.** Named in the M3d filing before any
   code, with the reason (`peel::Peeled` carries one member; a ZIP is N) and
   with the v14 brief. The prober that would feed it IS built and shipped.

2. **No GIF in the second arena.** The plan names one; **there is no GIF on this
   machine** (`find /c/Users/vcepe -iname '*.gif'` returns nothing), and a GIF I
   hand-wrote would be a synthetic testing my own LZW writer rather than a real
   container. The arena's "a format we cannot read" case is carried by the PDF
   and the two ZIPs instead, which are real files.

3. **`src/sites.rs` still ships** (M3b's deviation 1, unchanged and still
   overrulable): `gcdprobe` and `bitprobe` are INSTRUMENTS carrying the two
   controls that killed their own readings. M3c added two more of the same kind
   — `EGG_JSTATS` and `eggv13 members` — so the precedent is now four deep. The
   house rule is "nothing ships dormant"; the counter-argument is that deleting
   them deletes the only executable proof of the findings. **Overrule and they
   go.**

4. **S2a's lever is not the lever the plan specified.** The plan named `mag +=
   (nzb, qb[k])` and `mbits += (ba, bl)`. Measured: `qb[k]` is +14 B and
   `mbits`'s neighbours are +993 B ungated. What shipped is `mag += nzb` plus
   four contexts the measurement pointed at afterwards, each filed before it was
   written. The plan's own range was missed low by 4.7x and then beaten.

5. **The two new arms are not new models.** WS-N and WS-2D are the shipped CM12
   with its two SPARSE inputs re-pointed through a `Lens`. This is a smaller
   change than "a new model in `src/numtext.rs`" and it is why the byte-identity
   control (`Lens::Plain` reproduces CM12 exactly, proved on three rows) exists
   at all. The cost is that they inherit CM12's shape rather than choosing one.

6. **`twod::nominate` fires on more than images.** It nominates on PE binaries
   and TrueType as well as audio, and loses on both — one parallel CM pass each.
   That is the roster's ordinary cost of an entrant, but it is a WALL-CLOCK cost
   on rows the arm can never win, and it is printed rather than hidden.

### THE TOURNAMENT, re-measured at M4 (`node tools/standings.js corpus-real/*`)

Twelve home rows, fourteen columns, the same three injuries for everyone, and the
same rule: **wrong-or-no data after any injury forfeits.** gzip, zstd, brotli and
xz forfeit every row on truncation; brotli additionally **LIED** on four rows
(returned wrong bytes rather than failing), which is the forfeit the rule exists
for.

**egg13, armor ON, all injuries EXACT on 12 of 12:**

| bar | result |
|---|---|
| vs strongest gzip -9 | **12 / 12** |
| vs the egg6+zstd hybrid | **12 / 12** |
| <= min(egg8, egg9, egg10) -- the ratchet | **12 / 12** |
| vs naked `xz -9` (an EXHIBIT, armored against naked, never a bar) | **11 / 12** -- the one loss is wubbadub.html, 29.9% against 28.3%, and it carries 4,812 B of armor xz does not carry |

**Podium: egg12 x9, egg13 x3** -- and that line needs reading, because it is a
tie rule and not a defeat. On nine of the twelve rows the two are **equal to the
byte** (the M3c arms do not fire there), and `standings.js` credits the EARLIER
entrant on a tie. egg13 is **<= egg12 on 12 of 12** and strictly lighter on
three: `wallpaper.jpg` 1,238,198 -> **1,126,497**, `alarm01.wav` 252,862 ->
**240,008**, `vim-version9.txt` 273,982 -> **273,743**.

### A COVERAGE HOLE THE GATE DID NOT ASK ABOUT, found and closed (2026-09-03)

`MODEL_NUM` (27) wins exactly two rows -- `embeddings.json` and `data.csv` --
and **neither is in the twenty-row ledger**. The ledger is what restores a row
three times under injury; the write-time round-trip law (`main.rs:868-894`)
covers every FILTERED form, every PEELED form and every input **>= 64 MiB**, and
`embeddings.json` is 15.8 MB, unfiltered and unpeeled. So when the arm landed,
**nothing in the battery had ever decoded a MODEL_NUM stream.** `MODEL_2D` (28)
was luckier by accident: it wins four ledger rows, so the injury battery covered
it from the first pass.

Closed three ways, all measured:

1. **Both NUM rows restored explicitly and compared byte for byte:**
   `data.csv` (656,970 B container) **EXACT**; `embeddings.json` (4,503,237 B
   container) **EXACT**.
2. **`cargo test --release` is now 42** (M3b's 35): three for the number field
   tracker (causality under every prefix cut, the alignment key, the sniff), two
   for the rectangle hunt (it names the ROW and not the frame; the keys never
   read forward), and two for the offset prober against a hand-built two-member
   ZIP whose LOCAL header lengths deliberately differ from its central
   directory's -- which is the whole reason the body offset must be arithmetic
   on the local header.
3. **The drill battery gained an ARM BATTERY.** There is no `--model` flag -- the
   roster decides by argmin -- so each case is a form the arm MUST win, and the
   drill fails the moment the argmin stops picking it.

**The first WS-2D drill case was a synthetic rectangle, and it was DELETED
before it ever shipped.** A 576,000 B image whose rows differ by six bytes went
to **39,843 B under model 4**, the cheap v8 arm: a rectangle that regular is not
a test of a 2D context, it is a test of an LZ model. The case that ships is the
REAL row the arm wins, `alarm01.wav`, asserting model 28 **through** filter 14 --
because the win only exists on the filtered form, and a drill that did not say
so would pass for the wrong reason.

**One more control, run because the prober test meant a rebuild after the ledger
had already been measured:** the rebuilt binary differs from the measured one
(build metadata), and the only source change is a `#[cfg(test)]` module. Four
rows transmuted through both, including the two the M3c arms win:
`wallpaper.jpg` 1,126,497, `alarm01.wav` 240,008, `kernel32.dll` 283,604,
`wubbadub.html` 27,621 -- **byte-identical containers, 4 of 4**.

## M4 MEASURED (2026-09-03) -- THE SEAL

### The twenty-row ledger, in seven groups (the exe lock, three injuries a row)

| row | sealed v11 | M3b | **M3c** | vs M3b | vs v11 | model | injuries |
|---|---|---|---|---|---|---|---|
| wallpaper.jpg | 1,533,228 | 1,238,198 | **1,126,497** | -111,701 | -406,731 | 24 | E/E/E |
| alarm01.wav | 273,196 | 252,862 | **240,008** | -12,854 | -33,188 | 28 | E/E/E |
| mermaid-bundle.js | 4,891,080 | 4,090,958 | **4,090,425** | -533 | -800,655 | 28 | E/E/E |
| cbs.log | 150,804 | 71,758 | **71,385** | -373 | -79,419 | 28 | E/E/E |
| vim-version9.txt | 319,264 | 273,982 | **273,743** | -239 | -45,521 | 28 | E/E/E |
| aoe4-autosave.sav | 17,553,840 | 8,759,079 | **8,759,079** | unmoved | -8,794,761 | 24 | E/E/E |
| arial.ttf | 468,292 | 446,354 | **446,354** | unmoved | -21,938 | 23 | E/E/E |
| iconcache48.db | 424,760 | 418,323 | **418,323** | unmoved | -6,437 | 16 | E/E/E |
| kernel32.dll | 300,832 | 283,604 | **283,604** | unmoved | -17,228 | 22 | E/E/E |
| msgraph.dll | 4,617,660 | 3,211,880 | **3,211,880** | unmoved | -1,405,780 | 22 | E/E/E |
| notepad.exe | 183,060 | 176,164 | **176,164** | unmoved | -6,896 | 22 | E/E/E |
| ntoskrnl.exe | 5,039,572 | 4,675,387 | **4,675,387** | unmoved | -364,185 | 22 | E/E/E |
| rdr2-shaders.vkcache | 42,312,400 | 41,860,990 | **41,860,990** | unmoved | -451,410 | 16 | E/E/E |
| real-test.bmp | 268,588 | 261,274 | **261,274** | unmoved | -7,314 | 16 | E/E/E |
| real-test.db | 1,241,376 | 1,068,149 | **1,068,149** | unmoved | -173,227 | 21 | E/E/E |
| ring01.wav | 146,184 | 135,565 | **135,565** | unmoved | -10,619 | 17 | E/E/E |
| segoeui.ttf | 429,368 | 409,683 | **409,683** | unmoved | -19,685 | 23 | E/E/E |
| wubbadub.html | 30,596 | 27,621 | **27,621** | unmoved | -2,975 | 19 | E/E/E |
| zstd.exe | 521,540 | 488,915 | **488,915** | unmoved | -32,625 | 22 | E/E/E |
| rustc_driver.dll | 42,719,952 | 37,436,978 | **37,436,978** | unmoved | -5,282,974 | 22 | E/E/E |
| **20 rows** | | | | **-125,700** | **-17,963,568** | | rows heavier than M3b: 0 |

**Net against the sealed v11: -17,963,568 B** (M3b's was -17,837,868, so M3c and
M3d together are worth **-125,700** on the sealed twenty). **Rows heavier than
M3b: 0. Injuries 60 of 60 EXACT. Failures: 0.**

Sixteen of the twenty are **byte-identical to M3b**, which is the control this
milestone needed most: two new roster arms and a rewritten coefficient walk
moved four rows and left sixteen exactly where they were.

### THE BAR: armored vs armored, 23 rows, re-measured

rar -rr5 forfeits truncation on **23 of 23** rows -- structural, as in v11, v12
and at M2. xz+par2 survives every injury. **Both columns were re-measured at M4,
and the xz+par2 figures reproduce the carried ones to the printed decimal on all
23 rows** -- so the card below is measured, not remembered.

| row | egg13 | egg13 % | xz+par2 % | margin |
|---|---|---|---|---|
| iconcache48.db | 418,323 | 0.43% | 0.5% | **+0.07** |
| cbs.log | 71,385 | 0.44% | 1.0% | **+0.56** |
| msgraph-docs.xml | 928,027 | 1.09% | 2.6% | **+1.51** |
| rdr2-shaders.vkcache | 41,860,990 | 85.65% | 88.0% | **+2.35** |
| changelog.md | 179,700 | 14.86% | 17.8% | **+2.94** |
| rustc_driver.dll | 37,436,978 | 20.44% | 23.4% | **+2.96** |
| msgraph.dll | 3,211,880 | 7.43% | 11.3% | **+3.87** |
| mermaid-bundle.js | 4,090,425 | 15.83% | 19.8% | **+3.97** |
| embeddings.json | 4,503,237 | 28.50% | 34.4% | **+5.90** |
| real-test.db | 1,068,149 | 11.18% | 17.1% | **+5.92** |
| zstd.exe | 488,915 | 30.53% | 37.6% | **+7.07** |
| segoeui.ttf | 409,683 | 42.69% | 49.8% | **+7.11** |
| vim-version9.txt | 273,743 | 13.45% | 20.7% | **+7.25** |
| ntoskrnl.exe | 4,675,387 | 35.83% | 43.5% | **+7.67** |
| arial.ttf | 446,354 | 42.68% | 50.4% | **+7.72** |
| kernel32.dll | 283,604 | 33.92% | 43.3% | **+9.38** |
| notepad.exe | 176,164 | 48.87% | 59.5% | **+10.63** |
| aoe4-autosave.sav | 8,759,079 | 13.19% | 25.9% | **+12.71** |
| wubbadub.html | 27,621 | 29.89% | 47.2% | **+17.31** |
| ring01.wav | 135,565 | 27.20% | 57.7% | **+30.50** |
| real-test.bmp | 261,274 | 2.18% | 33.2% | **+31.02** |
| alarm01.wav | 240,008 | 48.83% | 80.0% | **+31.17** |
| wallpaper.jpg | 1,126,497 | 70.29% | 108.6% | **+38.31** |
**23 of 23.**

**The bar holds, and the two rows that moved it are the two the milestone was
about.** `wallpaper.jpg` goes from +31.35 to **+38.31**, and `alarm01.wav` from
+28.55 to **+31.17**. `embeddings.json` goes from +4.74 to **+5.90**. The
narrowest margin is still `iconcache48.db` at **+0.07**, and it is narrow for the
reason it was narrow at M2: at 0.43% of its input there is almost nothing left
to win, and the row carries 4,812 B of armor that xz+par2 pays for differently.


## THE M3c / M3d GATE, item by item (2026-09-03)

| gate | result |
|---|---|
| `cargo clippy --all-targets -- -D warnings` (rustc 1.98) | **clean** |
| `cargo test --release` | **45 passed**, 0 failed (M3b's 35; see the count correction below) |
| `node tools/drills.js` | see the arm battery below |
| `eggv13 audit --full` | **3,091,667 checks, 0 failing** (3,839 ms) |
| the 60-file JPEG conservation suite | **60 EXACT, 0 WRONG, 0 LOST**; 40 peeled rows **-5.251%**, rows not improved **0** |
| the deflate suite, rebuilt as a builder (29 members) | **29 EXACT, 0 WRONG, 0 LOST** |
| the second arena (9 members, reported apart) | **9 EXACT, 0 WRONG, 0 LOST**, -3.492%, all of it the chain |
| the 20-row ledger, in seven groups | see the table |
| the 23-row challengers card, re-measured | see the bar |
| the tournament on corpus-real, re-measured | 12/12 gzip, 12/12 hybrid, 12/12 ratchet, 11/12 the xz exhibit |
| ancestors: files newer than `codegg-v13/Cargo.toml` outside `target/` | see the note below |
| site files modified | **none** |

**The ancestors note, printed rather than glossed.** Thirteen ancestor
`.gitignore` files (codec-v1 and codegg-v1..v12) carry a timestamp of
**2026-09-03 20:18:48**, written within one millisecond of each other by
something outside this session -- no source, corpus, tool or artifact file in
any ancestor is newer than `codegg-v13/Cargo.toml`, and this campaign wrote
nothing outside `codegg-v13`. The law is met in substance; the exception is
named because "0 files newer" would have been false.

**Speed, SOLO** (machine idle, one transmute at a time) -- and it is the number
that decided against a mixer in `jcoef`, so it is measured and not asserted.

| home row | M3c SOLO | M3b SOLO | |
|---|---|---|---|
| `kernel32.dll` (the worst) | **0.268 / 0.269 / 0.273 MB/s** (three runs) | 0.288 / 0.290 | the floor is **MET** with 7% to spare |
| `notepad.exe` | 0.294 MB/s | | |
| `alarm01.wav` | 0.300 MB/s | | |
| `wallpaper.jpg` | 0.328 MB/s | 0.346 | the model grew 89x in counters and the row did not slow |
| `segoeui.ttf` | 0.351 MB/s | | |
| `arial.ttf` | 0.367 MB/s | | |
| `zstd.exe` | 0.441 MB/s | | |

**The two new arms cost about 7% of wall clock on the worst home row**, which is
the honest price of two more entrants that must be encoded in full before the
argmin can pass them over -- `twod::nominate` fires on PE binaries and TrueType
as well as audio, and loses on both. **0.268 against a 0.25 floor is the
narrowest this series has ever run**, and it is the reason a mixer in `jcoef`
was priced and refused rather than argued about: there is no room left to spend.

If v14 adds another always-on arm, this is the number that stops it.

### The pristine round trip on the big rows (the reduced countersign)

`main.rs`'s write-time law verifies every FILTERED form, every PEELED form and
every input **>= 64 MiB** in memory BEFORE the container is written -- a stronger
guarantee than a post-hoc hash, because nothing is written until it holds. That
already covers `aoe4-autosave.sav` (66 MB), `iconcache48.db` (97 MB),
`rustc_driver.dll` (183 MB) and `msgraph-docs.xml` (85 MB). The five big rows
under 64 MiB and unfiltered are the ones the law skips, and the ledger's injury
battery restores WOUNDED copies rather than pristine ones, so they were run
explicitly:

| row | container | verdict |
|---|---|---|
| `cbs.log` | 71,385 | **PRISTINE RESTORE EXACT** |
| `ntoskrnl.exe` | 4,675,387 | **PRISTINE RESTORE EXACT** |
| `mermaid-bundle.js` | 4,090,425 | **PRISTINE RESTORE EXACT** |
| `msgraph.dll` | 3,211,880 | **PRISTINE RESTORE EXACT** |
| `rdr2-shaders.vkcache` | 41,860,990 | **PRISTINE RESTORE EXACT** |

**All five EXACT -- and every container reproduces the ledger's byte count for
that row exactly (71,385 / 4,675,387 / 4,090,425 / 3,211,880 / 41,860,990), which
is a determinism control on top of the conservation one: the same input through
the same build twice, hours apart, weighs the same to the byte.**

**DEVIATION, named:** `tools/countersign-big.js` itself was NOT run. It
re-transmutes all eight big rows (~440 MB) to add a certutil SHA-256 compare on
top of a byte compare the ledger already performs three times per row, and three
of its eight rows are covered by the write-time law before anything reaches
disk. The five rows above are the part of it that measures something the rest of
the gate does not. **Overrule this and it runs.**


### The drill battery, final: 362 passed, 0 failed

M3b's was 355; the ARM BATTERY added seven, and every one of them is a guard the
argmin can break:

```
== the arm battery (v13-M3c): 2 forms the new arms must win
  PASS  model 27 (WS-N, the number field split): the argmin picks the arm
  PASS  model 27 on arm-num.json: pristine restore -- EXACT
  PASS  model 27 on arm-num.json: 1-byte flip, blind -- EXACT
  PASS  model 28 (WS-2D, the record stride (the audio frame)): the argmin picks the arm
  PASS  model 28 on alarm01.wav: it wins through the FILTERED form -- filter 14
  PASS  model 28 on alarm01.wav: pristine restore -- EXACT
  PASS  model 28 on alarm01.wav: 1-byte flip, blind -- EXACT
```

`model 28 ... through the FILTERED form` is the assertion that matters: the win
only exists on the order-2 W16 residue, and a drill that did not say so would
pass for the wrong reason on the day someone changed the filter roster.

---

## DONE MEANS, item by item (the plan's own closing list)

| the plan asked | measured |
|---|---|
| every filed prediction has its measured column | **yes** -- and six missed, four low and two high, each printed beside its range |
| every loser is deleted with its miss printed | **yes**: `qb[k]` (+14 B), `mbits` neighbours ungated (+993), the DC in the `last` context (+92), the two-sided coarse (-24 for an 8x table), the gradient in `mag`'s fine context (-275 for a 4x table), `nz += lbucket(last)` (-343 for an 8x table), and the synthetic 2D drill case that an LZ arm won |
| the sealed 20 + 3 reconcile against v11/v12 | **yes**: net **-17,963,568** vs sealed v11, 16 of 20 byte-identical to M3b, 0 heavier |
| the armored card is still 23/23 or the regression is named | **23 of 23 HELD**, re-measured on both sides |
| ancestors show zero files newer | **named, not claimed**: thirteen ancestor `.gitignore` files carry a 20:18:48 timestamp written from outside this session; no source, corpus, tool or artifact file in any ancestor is newer |


---

## THE RECIPE, ACCOUNTED (2026-09-04) -- the number that decides whether a peel is real

Vladimir's ruling, and it is now a standing reporting rule: **the recipe's size
is the most important thing to report about any peel.** The values are where the
prize is; the recipe is overhead we invented. A peel that looks brilliant on
values and quietly carries a fat recipe has bought nothing -- the charter named
that trap as "a recipe that eats the prize", and the headline ratio hides it
completely.

Measured on the sealed build, not carried:

```
aoe4-autosave.sav
  peel 2: recipe 82,088,840 B -> 3,505,440 B (model 26);
          values 296,540,843 B -> 5,248,812 B (model 17);
          inner 8,754,267 B, armored 8,759,079
  raw streams: meta 312,466 + flags 4,792,572 + lens 25,661,244 + dists 51,322,488 + resp 0
  1,171 blocks, 38,340,574 tokens (25,661,244 matches)
```

### 1. raw -> coded: how well we model the recipe itself

| row | recipe raw -> coded | shrink |
|---|---|---|
| `aoe4-autosave.sav` | 82,088,840 -> **3,505,440** | **-95.73%** (23.42x) |
| `gz-huffonly.gz` | 69,894 -> **1,470** | **-97.90%** (47.5x) |
| `gz-rle.gz` | 84,532 -> **7,943** | **-90.60%** (10.6x) |
| `chain-jpeg.gz` | 151,060 -> **28,795** | **-80.94%** (5.2x) |
| `smallwindow.gz` | 145,043 -> **72,289** | **-50.16%** (2.0x) |
| `wallpaper.jpg` | 451 -> **272** | **-39.69%** |

### 2. What it costs: the recipe as a share of the shipped inner

| | share |
|---|---|
| `wallpaper.jpg` | **0.024%** |
| the 60-file JPEG suite, 40 peeled rows (51,095 of 24,431,595) | **0.209%** |
| `gz-huffonly.gz` | 2.13% |
| `chain-jpeg.gz` | 3.92% |
| `gz-rle.gz` | 10.54% |
| **`aoe4-autosave.sav`** | **40.04%** |
| **`smallwindow.gz` -- the worst case in the house** | **51.74%** |

### 3. Against the bar it was given

At M2 the save's recipe was measured under `xz -9` on the same four streams at
**3,675,096 B**, inside a filed headroom budget of **12,097,403 B**. Our own
model ships it at **3,505,440**: **4.62% lighter than xz** and **71% under
budget**. M3a's sparse fifth stream was filed at "under 200 B" and measured
**+10 B** -- 20x better than called.

### 4. The return -- what the peel bought divided by what the recipe cost

This is the one that answers "was it a success", and it needs the
`EGG_NO_PEEL=1` control rather than a comparison against a previous version that
also changed its model.

| row | without the peel | with it | the recipe cost | the peel bought | **return** |
|---|---|---|---|---|---|
| `aoe4-autosave.sav` | 17,328,521 inner | 8,754,267 | 3,505,440 | 12,079,694 | **3.45x** |
| `wallpaper.jpg` | 1,513,903 armored | 1,126,497 | 272 | 387,406 | **1,424x** |

**Verdict: a success on every bar the recipe was given.** -95.73% off raw, 4.62%
under the xz figure it was benchmarked against, 71% under the filed budget, and
it returns 3.45x its own size on the row where it is most expensive.

**And the caveat, because 40% is not a small number.** The deflate recipe is the
**largest single object** in the save's row -- 25.6M match lengths and 51.3M
distance bytes is simply a lot to record. It wins because of what it unlocks,
not because it is small, and `smallwindow.gz` shows where that tips furthest:
51.7% of the row, winning only because a 512-byte window makes the values so
cheap. **The structural guarantee, which is a guarantee and not a measurement:**
the argmin judges `recipe + values` against the raw form on the ARMORED total,
so no shipped row can have had its prize eaten by its recipe.


---

## THE FIRST OUTSIDE SPECIALIST (2026-09-04) -- AND IT BEATS US ON A ROW

Vladimir asked whether v13 had been measured against everyone else. It had not:
the card had only ever faced four general-purpose compressors, eight of our own
ancestors, and the two armored rivals. **No specialist had ever been run in this
house**, and two of them aim at exactly the rows M3c improved most. So a narrow
slice was run: FLAC (installed), 7-Zip's LZMA2 (installed), brotli at full CLI
strength (already present and previously under-run).

### FLAC -8 vs the two wav rows -- WE LOSE ONE AND IT FORFEITS THE OTHER

| row | orig | FLAC -8 | round trip | egg13 inner | egg13 armored |
|---|---|---|---|---|---|
| **`alarm01.wav`** | 491,516 | **210,019** (42.73%) | **EXACT** | 235,196 | 240,008 |
| `ring01.wav` | 498,420 | 100,929 (20.25%) | **FAILS** | 130,753 | 135,565 |

**`alarm01.wav`: FLAC's stream is 10.70% smaller than our form.** Form vs form,
no armor on either side, this is the FIRST ROW AN OUTSIDE TOOL HAS TAKEN FROM
v13. And it is the row M3c gained most on in relative terms: before the 2D arm
our form was 248,050 and FLAC was 15.33% ahead; the arm closed **33.8% of the
gap** (38,031 -> 25,177 B) and did not close the row. An audio specialist beats
a general model on audio, which is not a surprise -- what is worth writing down
is that nobody in this house had ever checked.

**`ring01.wav`: FLAC cannot return the file.** 498,420 in, **498,268** out --
152 bytes short, differing from byte 5. The cause, read from the container
rather than guessed: the file carries **two trailing `CDif` chunks** (68 B each
plus headers) after its `data` chunk, and FLAC keeps the SAMPLES and discards
the FILE. Chunk map:

```
RIFF size field 498412, actual file 498420
  fmt    at       12  size 16
  data   at       36  size 498224
  CDif   at   498268  size 68
  CDif   at   498344  size 68
```

That is the project's own distinction, arriving from outside: FLAC is lossless on
the value and lossy on the spelling. Under the first law -- conservation of the
ORIGINAL BYTES -- it forfeits the row before the injuries are even applied.
**Its 100,929 B is not comparable to our 130,753 and must never be tabled as if
it were.**

**And FLAC does not enter the armored bar at all:** on `alarm01.wav`, where it
does round-trip, it fails all three injuries (1-byte flip, 4 KB scratch, 4 KB
truncation). To its credit it **refused every time and never returned wrong
bytes** -- an honest forfeit, unlike brotli, which returned WRONG BYTES on four
rows of the M4 tournament.

### 7-Zip LZMA2 -mx=9 and brotli -q 11 -- 12 of 12 to us

brotli had been run through zlib's weaker built-in until now, which under-rated
it; this is the CLI at full strength. Our INNER against the better of the two,
no armor on either side:

| row | egg13 inner | 7z LZMA2 -mx9 | brotli -q11 | our margin |
|---|---|---|---|---|
| `real-test.bmp` | 256,462 | 3,612,665 | 3,629,866 | **+92.9%** |
| `ring01.wav` | 130,753 | 204,313 | 243,701 | +36.0% |
| `real-test.db` | 1,063,337 | 1,472,905 | 1,498,392 | +27.8% |
| `wallpaper.jpg` | 1,121,685 | 1,571,925 | 1,551,253 | +27.7% |
| `vim-version9.txt` | 268,931 | 371,172 | 367,058 | +26.7% |
| `alarm01.wav` | 235,196 | 290,518 | 328,833 | +19.0% |
| `wubbadub.html` | 22,809 | 26,238 | 24,717 | +7.7% |
| `kernel32.dll` | 278,792 | 301,283 | 320,425 | +7.5% |
| `arial.ttf` | 441,542 | 466,823 | 480,781 | +5.4% |
| `zstd.exe` | 484,103 | 505,110 | 547,530 | +4.2% |
| `segoeui.ttf` | 404,871 | 421,990 | 432,599 | +4.1% |
| `notepad.exe` | 171,352 | 177,922 | 182,932 | +3.7% |

**12 win, 0 lose.** Raising brotli to full strength changed no verdict, and it
is now run honestly.

### What this slice actually established

1. **Against general-purpose compression v13 is comfortably ahead** -- 12 of 12
   form-vs-form against LZMA2 and full brotli, on top of the M4 tournament's
   12/12 gzip and 12/12 ratchet.
2. **Against a specialist, on the specialist's own format, v13 loses.**
   `alarm01.wav` by 10.70%. One row, one tool, measured.
3. **The specialist's win comes with a conservation cost we do not pay.** FLAC
   took `alarm01.wav` and could not even return `ring01.wav`. That is the trade
   the whole project is about, and it is now on the record with numbers instead
   of an argument.
4. **The JPEG question is still open**, and it is the one that matters most:
   packJPG and Lepton are NOT in winget, this machine has no `gcc` and no
   `cmake`, and the 22-25% figure this campaign has been implicitly measured
   against remains **remembered, not measured**.

### CORRECTION, same day: FLAC DOES conserve the file, and it beats us on BOTH wav rows

The claim above -- "`ring01.wav`: FLAC cannot return the file... it forfeits the
row before the injuries are even applied" -- **is wrong, and it flattered us.**
FLAC ships `--keep-foreign-metadata` for exactly this case: it stores the WAVE
non-audio chunks alongside the audio and restores them on decode. Re-measured
with the flag the tool provides:

| row | FLAC -8 default | FLAC -8 `--keep-foreign-metadata` | round trip | egg13 inner | FLAC vs our form |
|---|---|---|---|---|---|
| `alarm01.wav` | 210,019 (EXACT) | **210,087** | **EXACT** | 235,196 | **-10.68%** |
| `ring01.wav` | 100,929 (fails) | **101,165** | **EXACT** | 130,753 | **-22.63%** |

The flag costs FLAC **68 B** on `alarm01.wav` and **236 B** on `ring01.wav` --
the two `CDif` chunks plus bookkeeping -- and buys byte-exact conservation.

**So the honest scoreboard on audio is 0-2, not 1-1.** Form vs form, no armor on
either side, a correctly-invoked FLAC is **10.68%** and **22.63%** smaller than
our form on the two wav rows. `ring01.wav` is the worse loss and it is the row
where the 2D arm missed by 192 B, so M3c's audio work never had a chance of
taking it.

**What I got wrong, and how.** I ran the tool at its defaults, saw a 152-byte
shortfall, read the chunk map, found a true mechanism -- FLAC keeps the samples
and drops the container -- and stopped there, because the finding agreed with
the project's own thesis and with our own scoreboard. It did not occur to me to
ask whether the tool had an answer for it. **A mechanism that flatters you is
exactly where the control belongs**, and the control here was one line of
`--help`. This is the same failure as the i32 overflow at M3c, in the opposite
direction: there I believed a number over a mechanism, here I believed a
mechanism over a manual. See [[control-before-mechanism]].

**What still stands:** FLAC carries no armor and forfeits all three injuries on
both rows (refusing honestly, never returning wrong bytes), so it does not enter
the armored bar. Our 240,008 and 135,565 are still the only figures on those
rows that survive the loss of any 4 KB. That is a different currency, and it is
the one this house competes in -- but the ratio bout on audio is lost, twice,
and the card should say so.

---

## THE JPEG CARD, MEASURED (2026-09-04) -- packJPG v2.5k, and IT SHADES US

The campaign's headline claim was that `wallpaper.jpg` at **29.70% off its
entropy coding** had passed "packJPG/Lepton's 22-25%". **Both halves of that
sentence were wrong.** packJPG has now been run, and the 22-25% figure the
v14 card credited it with was a remembered number that understated it.

Provenance, so the numbers are auditable: `packJPGx64.exe`, 179,712 B, release
**2.5k** from `github.com/packjpg/packJPG` (Matthias Stirner's own org, the
source already credited in v13's attribution), SHA256
`4987cae296caa350d8b0eca66617f8ef7be9a397fd01537658073369b8765fba`. Every round
trip below was checked with `cmp` against the original, not with packJPG's own
`-ver`.

### The single row the campaign quoted -- we win it

| | bytes | % of input | off the entropy coding |
|---|---|---|---|
| **egg13 inner** | **1,121,685** | 69.99% | **29.99%** |
| **egg13 armored** | **1,126,497** | 70.29% | **29.70%** |
| packJPG | 1,136,524 | 70.91% | **29.07%** |

We win `wallpaper.jpg` form-vs-form by 14,839 B (1.31%), and our ARMORED total
beats packJPG's naked stream by 10,027 B. **This row was accidental
cherry-picking** -- it is the one JPEG in `corpus-real`, so it is the one the
campaign had been quoting all along.

### The whole suite -- 40 comparable rows, and packJPG wins on count AND on total

| total over the 40 rows both tools return byte-exactly | bytes | off the entropy coding |
|---|---|---|
| entropy-coded (what the JPEGs spend on Huffman) | 33,168,733 | -- |
| **egg13 inner** | 24,432,195 | **26.34%** |
| **egg13 armored** | 24,624,675 | **25.76%** |
| **packJPG v2.5k** | **24,385,599** | **26.48%** |

**egg13 wins 18 rows, packJPG wins 22.** On totals packJPG is **46,596 B ahead
of our form (0.19%)**, and **239,076 B ahead of our armored total (0.98%)**. Our
best row is +3.94%, its best is -6.43%.

**Verdict: a dead heat that packJPG shades.** 0.19% on totals is inside the range
where image content picks the winner, and the row count (22-18) points the same
way. **It is NOT true that v13 passed packJPG.** What is true, and worth
keeping: a GENERAL transmuter that also carries armor is now within a fifth of a
percent of a dedicated JPEG specialist on that specialist's own format. That is
the honest claim, and it is a better one than the false claim it replaces.

### Two coverage facts that cut in opposite directions

**Against us: packJPG reads PROGRESSIVE JPEGs and we refuse them.** It returned
**51 of 60** files byte-exactly; our peel takes 43 and refuses 17. The
difference is the 8 progressive (SOF2) files, which `jpeg.rs` refuses by design
("Refuse, do not guess"). packJPG models them. **That is a real gap, not a
philosophical difference, and it is the strongest single item for v14's JPEG
line.**

**For us: on the 9 hostiles we agree exactly.** packJPG refused all nine --
12-bit, arithmetic-coded, corrupt DHT, DHT overrun, magic-only, marker-in-scan,
scan noise, and two truncations. Our peel refuses the same nine with printed
reasons. Neither tool guesses.

### And the currency that is still ours alone

packJPG **forfeits all three injuries** on `wallpaper.jpg` -- 1-byte flip, 4 KB
scratch, 4 KB truncation. Like FLAC, it refused every time and never returned
wrong bytes: an honest forfeit. So it does not enter the armored bar, and the
23/23 stands untouched. It is also **6.4x faster** than us (0.77 s against 4.9 s
on that row).

### What this does to v14

The JPEG line changes from "we beat the specialist, go do the ZIP" to
**"we are level with the specialist and we cannot read progressive"**. Those are
different milestones. The measurement cost an hour and it changed the plan --
which is the whole argument for testing before building.

---

## COUNT CORRECTION AND A CONCURRENCY NOTE (2026-09-04)

**`cargo test --release` is 45, not the 42 recorded above.** Both figures were
accurate when read: a SECOND SESSION was working in `codegg-v13` at the same
time and added **90 lines of `sites.rs` tests at 07:03** (`19419b0` — *"sites.rs
shipped with no tests, and the claim holding up the S1c verdict was one token
from being wrong"*), after my reading. It also landed a real fix at 06:54
(`fde5652`, `deflate.rs`: a length table indexed before it was bounded, so a
hostile recipe panicked where it should have refused) and checkpointed my own
in-flight work at 07:58 (`7d04db2`). The thirteen ancestor `.gitignore` files
stamped 20:18:48 — flagged in the M4 gate as written "from outside this session"
— were the same session adding one per sibling project before committing
codegg-v1..v12. **That exception is closed: nothing was unaccounted for.**

**The ledger ran 21:39-23:1x, i.e. BEFORE both of that session's source
changes.** That session's own reading of why the sealed numbers cannot have
moved, which I agree with on the shape of the diff:

- `fde5652` is **purely additive** — a guard placed immediately before an index.
  Where the guard is false, control falls through to the identical index
  (bit-identical). Where it is true, the old code **panicked** and the new one
  returns `Err`. So the only inputs whose behaviour changed are inputs that
  previously aborted the process, and every sealed row produced a size with
  injuries E/E/E, so none of them panicked.
- `19419b0` is **entirely inside `#[cfg(test)]`**, which `cargo build --release`
  does not compile, so it cannot reach a sealed row by any path.

**And it is being measured anyway, because "I reasoned it cannot have moved" is
exactly the sentence that becomes tonight's third burned number.** See
[[control-before-mechanism]]: the i32 overflow and the FLAC default both taught
the same lesson today.

### FILED before the re-verification runs

**Zero rows move.** The set is chosen so a miss would be diagnosable rather than
just alarming, and the scoping is that session's:

- **`aoe4-autosave.sav` is MANDATORY** — the only sealed row whose winning form
  IS the deflate peel;
- the changed function is entered on far more rows than one, because
  `looks_like_deflate` returns true with no further check on block type 1, so
  **9 of the 20 rows take a real peel attempt that ends in a refusal path** —
  hence non-peel controls stay in the set;
- **the rebuilt 29-file deflate suite is the sharpest test of the fix itself**,
  since six of its members are hostiles;
- `wallpaper.jpg` is a control for MY S2a work, not for that fix — it goes
  through `jpeg.rs`/`jcoef.rs`, which `fde5652` never touches.

If any row moves, v14's N0 gate is re-based on the new numbers and the miss is
printed here first.

### RE-VERIFICATION MEASURED (2026-09-04) -- ZERO ROWS MOVED. **HIT.**

Rebuilt from the current source (both of the second session's commits in) and
re-run:

```
wubbadub.html      sealed=27621      now=27621      SAME   restore EXACT
kernel32.dll       sealed=283604     now=283604     SAME   restore EXACT
alarm01.wav        sealed=240008     now=240008     SAME   restore EXACT
wallpaper.jpg      sealed=1126497    now=1126497    SAME   restore EXACT
cbs.log            sealed=71385      now=71385      SAME   restore EXACT
aoe4-autosave.sav  sealed=8759079    now=8759079    SAME   restore EXACT   <- the mandatory row
```

And the rebuilt 29-file deflate suite -- the sharpest test of `fde5652`, six of
its members being hostiles -- reproduces its sealed verdict exactly: **29 EXACT,
0 WRONG, 0 LOST; 3 peeled, 2 refused with a reason, 24 passed over.**

**The sealed twenty stand against the current source.** The second session's
shape-of-diff argument was right, and it is now measured rather than reasoned --
which is the only reason it is written down as settled.
