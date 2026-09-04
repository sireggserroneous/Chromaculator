# codegg-v13 — the Value Underneath

**SEALED at M4.** M0 the fork, M1 the peel frame and the JPEG coefficient model,
M2 the deflate peel, M3a the spelling exception, M3b the three site readings,
M3c the model side, M3d the chain and the second arena. Every number below was
measured on this machine; every filed prediction has its measured column in
`PREDICTIONS.md`, and the misses are printed first.

eggv13 is still the Transmuter. Transmutation, not compression. The first law is
conservation and the FNV-64 of the original bytes gates every restore. The
charter verse is the site's own tooltip for `form` (spec.html:359-360):

> Plain is the digits as written; pushed respells the same value using −1s.
> **The value underneath does not change — compare the two and only the colours
> differ.**

## FAILURES FIRST — what this campaign got wrong

**1. A wrapped multiply published a mechanism.** The count gate in `jcoef.rs`
was measured three times and the first two readings were both wrong, in opposite
directions, from `(f.p as i32 - c.p as i32) * w` overflowing i32 in release. The
first reading said the proportional blend LOSES 107,417 B; I wrote down a
mechanism for it ("averaging a locked 0.999 against a 0.9 parent") and switched
to a hard step, which measured worse still — and *more* coarse measured
*better*, which is not something a gate can do. **The control that caught it was
`KGATE = 0`**, which must reproduce the ungated build to the byte and did not.
With `i64` it does, and the proportional law then wins. The house lesson: a
measurement that disagrees with the mechanism is not a finding until a control
says the instrument works.

**2. Three predictions missed low and three missed high.** The plan's S2a lever,
built exactly as specified, was worth **−0.32%** against a filed −1.5% to −4%;
two of the three inputs it named were measured and DELETED (`qb[k]` at +14 B,
`mbits`'s neighbours at +993 B ungated). The contexts the measurement then
pointed at were worth **−6.1%**, and the biggest of them — the block's own
last-nonzero index — was filed at −0.1% to −0.6% and measured at −2.95%.

**3. "Provably free" was not.** I filed a proof that a pure re-ordering of the
coefficient walk costs nothing, corrected it once before the code, and it was
still wrong: band order alone is **−1.0%**, because the counters are
count-adaptive and the ordering acts as a soft extra context.

**4. What died, printed:** the transpose filter (M3b, 6 of 6 rows lose), the gcd
reading (M3b, both trials lose), the bit period (M3b, killed by its own
control), `qb[k]` in `mag`, the neighbour buckets in `mbits` ungated, the DC
magnitude in the `last` context, the two-sided coarse reading (−24 B for an 8×
table), the gradient in `mag`'s fine context (−275 B for a 4× table).

**5. Not built, and named rather than discovered missing:** the ZIP container
peel. A ZIP is N independent deflate members and `peel::Peeled` carries one;
that is a change to the peel frame and it is v14's line. The prober that would
feed it IS built and shipped as `eggv13 members`.

## The thesis

A JPEG's Huffman bits and its quantised DCT coefficients are the same value in
two spellings. On several rows v12 modelled somebody else's spelling — the
colours — and called the result our form. v13 reads the value underneath: peel
the foreign code, model what it actually says, re-spell it byte-exactly on the
way out.

## THE LAW OF THE PEEL

1. **A peel is a bijection or it is not used.** At transmute time the peel
   re-encodes its own output and compares against the original bytes. One byte
   off and the peel is discarded for that file and the raw bytes go through the
   ordinary pipeline.
2. **The recipe rides inside** and is judged with the values, by argmin on the
   ARMORED total — never on inner bytes.
3. **Refuse, do not guess.** Progressive, arithmetic-coded, 12-bit, truncated,
   corrupt: keep the bytes and print the reason.
4. Every peel rides v12's write-time round-trip law.
5. **The chain (M3d):** a peel's values may themselves be a peel, to depth 2 and
   no further, and both directions read that depth from ONE constant.

## The twenty-row ledger, measured

Run in seven groups on a copied build (the exe lock), three injuries per row:

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

## THE BAR: armored vs armored, 23 rows

rar -rr5 forfeits truncation on 23 of 23 rows -- structural, as in v11, v12
and at M2. xz+par2 survives every injury. BOTH columns were re-measured at M4
and the xz+par2 figures reproduce the carried ones to the printed decimal on
all 23 rows -- so the card below is measured, not remembered.

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


## THE TOURNAMENT, re-measured at M4 (`node tools/standings.js corpus-real/*`)

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

## MEASURED AGAINST THE SPECIALISTS (2026-09-04) — and they shade us

Until this point v13 had only ever faced four general-purpose compressors, eight
of its own ancestors, and the two armored rivals. **No specialist had ever been
run in this house.** Two now have, and both results go against us on ratio.

**packJPG v2.5k** (release 2.5k from `github.com/packjpg/packJPG`, SHA256
`4987cae2…`; every round trip checked with `cmp`, not with its own `-ver`), over
the **40 rows of `corpus-jpeg` both tools return byte-exactly**:

| | bytes | off the entropy coding |
|---|---|---|
| entropy-coded | 33,168,733 | — |
| egg13 inner | 24,432,195 | 26.34% |
| egg13 armored | 24,624,675 | 25.76% |
| **packJPG** | **24,385,599** | **26.48%** |

**egg13 wins 18 rows, packJPG wins 22**, and packJPG is **0.19% ahead on our
form** and **0.98% ahead of our armored total**. We win `wallpaper.jpg` (the row
this README quotes) by 1.31% — that row is the *only* JPEG in `corpus-real`, so
quoting it alone was accidental cherry-picking. **It is not true that v13 passed
packJPG.** The honest claim: a general transmuter that also carries armor is
within a fifth of a percent of a dedicated specialist on its own format.

**packJPG also reads PROGRESSIVE JPEGs, which we refuse** — 51 of 60 files
byte-exact against our 43. The 8 progressive (SOF2) files are a real coverage
gap. On the 9 hostiles the two tools agree exactly: both refuse all nine.

**FLAC 1.5.0** with `--keep-foreign-metadata`, byte-exact both rows:
`alarm01.wav` 210,087 against our 235,196 (**−10.68%**), `ring01.wav` 101,165
against our 130,753 (**−22.63%**). Audio is lost 0–2.

**What is still ours alone.** Both specialists **forfeit all three injuries** —
they refuse honestly and never return wrong bytes, but neither survives losing
4 KB. Our figures on those rows are the only ones that do, and the armored bar
stays 23/23. packJPG is also **6.4× faster** than us.

## THE RECIPE — the number that decides whether a peel is real

A peel splits a file into a RECIPE (everything needed to re-spell it exactly)
and VALUES (what it actually says). The values are where the prize is; the
recipe is overhead we invented, and a peel that looks brilliant on values while
carrying a fat recipe has bought nothing. So the recipe is reported first.

| row | recipe raw → coded | share of the shipped inner | return on the recipe |
|---|---|---|---|
| `aoe4-autosave.sav` | 82,088,840 → **3,505,440** (**−95.73%**) | **40.04%** | **3.45×** — costs 3,505,440, buys 12,079,694 |
| `wallpaper.jpg` | 451 → **272** (−39.69%) | **0.024%** | **1,424×** — costs 272, buys 387,406 |
| the 60-file JPEG suite, 40 peeled rows | 51,095 total | **0.209%** | |
| `gz-huffonly.gz` | 69,894 → **1,470** (−97.90%) | 2.13% | |
| `chain-jpeg.gz` | 151,060 → **28,795** (−80.94%) | 3.92% | |
| `smallwindow.gz` — the worst case | 145,043 → **72,289** (−50.16%) | **51.74%** | still wins |

Against the bar it was given: the save's recipe measured **3,675,096 B** under
`xz -9` on the same four streams at M2, inside a filed budget of 12,097,403 B.
It ships at **3,505,440** — **4.62% lighter than xz, 71% under budget**. M3a's
sparse fifth stream was filed at "under 200 B" and measured **+10 B**.

The returns are measured against the `EGG_NO_PEEL=1` control, not against a
previous version that also changed its model.

**The caveat, because 40% is not small.** The deflate recipe is the largest
single object in the save's row — 25.6M match lengths and 51.3M distance bytes
is a lot to record. It wins because of what it unlocks, not because it is small.
**The structural guarantee** (a guarantee, not a measurement): the argmin judges
`recipe + values` against the raw form on the ARMORED total, so no shipped row
can have had its prize eaten by its recipe.

## The JPEG row, which is what M3c was for

| | bytes |
|---|---|
| wallpaper.jpg | 1,602,752 (entropy-coded 1,602,311) |
| recipe | 451 raw → 272 (the CM12 arm beat storing it) + 15 B of preamble |
| values | 55,296,000 raw → **1,121,398** |
| inner | 1,121,685 |
| price (armor v4, unmoved) | 4,812 — 256-B squares, 18 parity, 0 CT, 204 sites; 1.17× the pigeonhole floor |
| **armored total** | **1,126,497** |
| against the entropy coding | **−475,814 = 29.70% off** |
| against v12 on the same row (1,513,903) | **−387,406** |
| naked `xz -9` (an exhibit, never a bar) | 1,571,824 |

The 60-file JPEG conservation suite on the shipping build: **40 peeled rows,
25,731,725 → 24,380,500 = −5.251%, rows not improved: 0**; 60 of 60 EXACT, 0
LOST, 0 WRONG, the same 17 refusals with the same printed reasons.

**A dump is not a model.** The same coefficients dumped raw are 2,192,684 B
under `xz -9`; modelled they are 1,121,398.

## The second arena, reported apart from the sealed 20 + 3

`python tools/mkarena.py` builds nine members — real ZIPs, DOCXs and a PDF off
this machine, plus a multi-member gzip, two constructed 284-spelling gzips and a
gzipped JPEG — and `node tools/arena.js` weighs them:

**9 EXACT, 0 WRONG, 0 LOST. Total 5,384,545 → 5,145,752 (−186,188 = −3.492%
against the M3b build), and every byte of that movement is one row:** the
gzipped JPEG, which the CHAIN takes to depth 2. A multi-member gzip is refused
with the number. The ZIPs and DOCXs are not nominated at all — `peel::nominate`
reads offset 0 — while `eggv13 members` reads their layouts and counts 36
deflate members inside one 1.1 MB ZIP. That gap is v14's brief.

The deflate peel's own conservation suite, rebuilt as a BUILDER
(`tools/mkdeflatesuite.py`, 29 members): **29 EXACT, 0 WRONG, 0 LOST**; 3 took
the peeled form, 2 hostiles refused with the right reason, 24 peeled-or-nominated
and were passed over by the argmin.


## What each milestone built

**M1 — the peel frame and the JPEG coefficient model.** `src/peel.rs` is the
frame: `MODEL_PEEL` (24) in the header's model byte, a 15-byte preamble, then
the recipe stream and the values stream. ONE constant, `PEEL_MAX`, bounds the id
space and both sides read it. `src/jpeg.rs` is the JPEG peel — every MCU
Huffman-decoded, DC prediction reset at each restart, the file's own DQT/DHT and
the whole marker skeleton kept **verbatim**, re-encoded with the spec's padding.
`src/jcoef.rs` is the coefficient model.

**M2 — the deflate peel.** gzip, zlib, PNG IDAT and bare streams parsed to
(recipe, inflated bytes); the recipe is four streams in four languages under
`MODEL_DRECIPE` (26), each through the ordinary roster.

**M3a — the spelling exception.** A 258-byte match has two legal spellings and
the parser refused the second. It is now recorded in a **sparse fifth stream**
(the alternatives cost +3.2 MB on the save, or broke the periodicity the roster
arm exploits). Cost on the one corpus row that has a deflate member: **+10 B**.

**M3b — the three site readings, all three tried and all three deleted.** The
transpose (filter id 15) lost on 6 of 6 rows that nominated it — the lattice
already reads a learned stride without throwing local context away. The gcd
survives on 0.113% of blocks once power-of-two factors are removed (a
power-of-two gcd is not a finding) and both qualifying trials lost. The bit
period died to its own control: what the first probe measured was context
length, not period.

**M3c — the model side.** `jcoef` gained the block's own last-nonzero index, the
distance to it, a widened nonzero count and neighbour bucket, a **count gate**
that makes fine contexts affordable, and — after the walk was rebuilt in three
passes — the AC sign against the **two-sided DC gradient**, which only a walk
that decides every DC before any AC can read. Two new roster arms joined:
`MODEL_NUM` (27), the number field split, and `MODEL_2D` (28), the rectangle.
Both are the shipped CM12 with its two sparse inputs re-pointed; `Lens::Plain`
reproduces CM12 to the byte and three rows prove it.

**M3d — the chain and the prober.** A peel's values may themselves be a peel, to
depth 2, read from ONE constant by both directions. `eggv13 members` reads a
container's declared layout and returns null rather than guessing.

## Run it

```bash
cd codegg-v13 && cargo build --release          # std only, offline
cargo test --release                            # 42
cargo clippy --all-targets -- -D warnings       # clean under rustc 1.98
target/release/eggv13 transmute <file> [--survive BYTES] [--tier BLK] [--parity T] [--ct triple|incw|none] [--judge]
target/release/eggv13 restore <f>.egg13|.egg12|.egg11|.egg10|.egg9|.egg8 [--wound start:len]
target/release/eggv13 info <f>.egg13            # the promise with its number
target/release/eggv13 members <file>            # M3d: a container's declared layout, or null
target/release/eggv13 probe <file>              # the filter nomination trace
target/release/eggv13 gcdprobe|bitprobe <file>  # M3b's two instruments
EGG_JSTATS=1  target/release/eggv13 transmute x.jpg   # M3c: the coefficient census
EGG_PEEL=1    target/release/eggv13 transmute x.jpg   # the peel's reading, or its refusal
EGG_NO_PEEL=1 target/release/eggv13 transmute x.jpg   # the ordinary pipeline, for a control
EGG_ARMS=1    target/release/eggv13 transmute x       # every arm's inner
node tools/mkjpegsuite.js && node tools/jpegsuite.js  # the 60-file conservation suite
python tools/mkarena.py && node tools/arena.js        # M3d: the second arena
node tools/drills.js                                  # the drill battery (362)
node tools/ledger13.js                                # the 20-row ledger -- RUN IT IN GROUPS
```

**`tools/ledger13.js` opens 8 lanes on the 8 largest rows at once.** It hung
twice on 2026-09-03 and the signature to trust is **"no exit", not "no
progress"**: it printed every row and its summary and then failed to exit with
the temp directory already gone. Run it in groups (`EGG_ONLY=`), and if it hangs
at the end, kill it and read what it already wrote — the measurement is not
lost.

Format **EG13 v7** (`.egg13`); eggv13 restores `.egg12` through the same armor
v4 and `.egg11` / `.egg10` / `.egg9` / `.egg8` through `src/armor11.rs`, v11's
armor v3, verbatim.

## Attribution

ITU T.81 (the JPEG codec, the canonical Huffman decode, the 1-bit padding rule);
packJPG (Matthias Stirner) and Lepton (Dropbox) — the peel idea and the
coefficient-model context shape; LOCO-I / JPEG-LS (the MED predictor on the DC
plane); RFC 1951 (deflate, and its two spellings of 258); PKWARE's APPNOTE (the
ZIP central directory the prober reads); Reed & Solomon 1960,
Berlekamp–Massey, Chien, Forney (the armor, unmoved from v12); Krachkovsky &
Lee 1997 and Bleichenbacher, Kiayias & Yung 2003 (collaborative decoding of
interleaved RS); Matt Mahoney's zpaq / lpaq / paq8 and Byron Knoll's cmix (the
16-bit coder, StateMap, APM, and the idea of backing a sparse context with a
dense one); Fowler–Noll–Vo (FNV-64); Igor Pavlov's LZMA (the token shapes);
XZ Utils 5.8.3 (the rival's stream, exact bytes). The site supplied every
reading.
