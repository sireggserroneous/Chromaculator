# eggSo v2 predictions — filed 2026-09-02, BEFORE a line of the codec was written

The series convention: every number below is a guess, written down first, and the
measured value is filled in beside it afterwards or not at all. Misses stay. A
prediction that is quietly edited to match its result is worth less than no prediction.

This round has **two arms**, each built and measured — *"we need to test all ways, to see
who the house keeps. No free lunch here"* — with its own bars, stages and verdict, and one
shared closing audit that also seats the arms of eggSo-v0 and eggSo-v1 at the same table
(`tools/standings.js`). Bare `v2` means eggSo-v2; the control is always `codegg-v1`.

## What this is, in one sentence

A **green** — the `0` of the signed-digit alphabet — read two ways: as the
representation's own forced slack (canonical form makes every green trailing, hence
information-free, hence a slot), and as a three-valued erasure (the Wub rolls a green
red-or-blue as a coin).

## Why this round exists

The lineage audit found **puncture** at zero mentions across thirteen versions, alongside
anti-transpose and involution. eggSo-v1 used the map. This round uses the alphabet's own
slack, and it exists mainly to **file a verdict**: is there a free lunch in the forced
greens? The plan's arithmetic says no, and says so before building; the round's value is
the measured rate at which the lunch fails to appear, and the name of what remains.

The ground it stands on, cited:

- `stalk.js:416-427` — after `pushLeft` "no green is ever followed by a lit cell — every
  green left over is trailing." `spec.md:55-59` runs push to a fixpoint; `spec.md:64`:
  "Leftover cells are padded green."
- `codec-v1/README.md:15-20` — canonicity *as detection* caught 48% of singles and **0 of
  50,000 sign flips**: a `±1` flip keeps the form canonical. v2(a) therefore uses the
  forced slots as *storage*, not canonicity as a check.
- `wub.html:282-292` — "a green is undecided, so rolling it red-or-blue gives one concrete
  draw"; `BIAS` leans the coin. An erasure that is sampled: ground for v2(b).
- `codegg.js:166-174` — the trit path already exists: `opts.alphabet === "trit"` widens
  the alphabet to `[−1, 1]`; under bits a `−1` cell is a sentinel erasure. `codegg.js:87-89`
  — `d = ±2` at `j` spells the same number as `d = ±1` at `j − 1`.
- `codegg.js:125-132` — `G.toBytes` reads `cells[j] === 1` and is wrong for any `−1`; v2
  recovers `V` by BigInt (`stalk.js:74-83 valueOf`) and serialises from that.

## The construction, stated before building

v2 stores the **canonical (pushed) square**: `G.toCells` lays the bytes row-major as
before, then `pushLeft` (restated from `stalk.js:59-71`, asserted against the site's own
function) respells it into a `±1` prefix and a green tail. Push conserves `V`, so the bytes
come back from `V` by BigInt. The trit alphabet `d ∈ {±1, ±2}` builds the per-region
tables after `codegg.js:90-102`; `inAlpha = |v| ≤ 1`; the confirming residue `q` is applied
**per candidate** as `codegg.js:204-206` does, not after the whole plan as `eggso.js:237-241`
does; the `−1` sentinel-erasure at `eggso.js:143` is **disabled** — under trits `−1` is a
digit — and erasures come only from an explicit `erased` list. A **canonicity filter**
rejects any repair that leaves a green followed by a lit cell.

### S1 — the trailing-green histogram, before any codec code

`pushLeft` over 10,000 random squares, `spec.md`, `stalk.js`, `og.png`, and — read-only,
not ours — `codegg-v10/corpus/program.exe`, `codegg-v10/corpus-real/notepad.exe` and
`codegg-v10/corpus/archive.zst`. **This stage files v2(a)'s verdict.**

### v2(a) — forced greens as check slots

Capacity needed: 4 residues × 7 trits (`3^7 = 2187 > q = 2063`) = **28 trits**, balanced
ternary of `r − 1031`. A square is **in-band** iff `v₂(V) ≥ 28`, i.e. its 128-byte block
ends in ≥ 3.5 zero bytes. The check occupies **fixed slots `L−28..L−1`**; in-band
eligibility is a property of `V`, so the decoder knows those cells were green and zeroes
them after extraction. What it cannot derive is *whether* the square is in-band — damage
can fake either state — so **one flag bit per square, out of band** (a bitmap). Out-of-band
squares carry v0's four residues externally. Damage inside the check slots yields a
spurious syndrome refused by the other residues → detected. Two primes per region (8.2%)
is predicted **unnecessary**: `p` per region + `q` + the canonicity filter settle the
`2w[j] = w[j−1]` aliases.

### v2(b) — greens as 3-valued erasures

Extend `eggso.js:142-191` to base 3. Budget: `3^10 = 59,049 ≤ 2^16` → **erasure cap 10 per
region** by default; the 12-cell burst is measured at **cap 12** (`3^12 = 531,441`) and
says so; 16 is infeasible. On **canonical** squares the canonicity filter changes the
count: a flagged cell before the last lit cell is two-valued (`±1`), a flagged cell in the
tail is forced (`0`), and the boundary is one of a handful — capacity returns to ≥ v0's.
**That is exactly the Wub's coin** (`wub.html:287-292`): the two-valued roll is the
canonical form's erasure model; three values are for non-canonical spellings.

## Measured during planning — ground, not predictions

| arm | quantity | computed |
| --- | --- | --- |
| **v2(a)** | trailing greens after `pushLeft`, per 1024-cell square (n ≈ 150 each) | random bits: median **1**, p90 3, max 6 · `spec.md`: median **1**, p90 5, max 337 (one padding square) · `stalk.js`: median **0**, p90 5 · `og.png`: median **0**, p90 4, max 34 |
| v2(a) | squares with ≥ 48 trailing green **cells** (a bit-alphabet threshold; the four residues are 44 bits — in trits, 28 cells) | **0 / 150** random · **1 / 119** text · **0 / 150** source · **0 / 150** binary |
| v2(a) | closed form (from `pushLeft`, stalk.js:62-71): at the fixpoint the lit cells are a `±1` prefix, so the green tail length is the **2-adic valuation of `V`** | for random bits `P(k greens) = 2^−(k+1)`, mean **1.0**. A square carries the checks in-band iff its block ends in ≥ 3.5 zero bytes; `spec.md`'s 337-green square is the file's zero-padded last block |
| **v2** | trit alphabet `d ∈ {±1,±2}`, one prime, single errors: collisions | **2,046 of 4,096** — exactly `d=±2 at j ≡ d=±1 at j−1`. One prime cannot separate trit singles; the alphabet check, the canonicity filter or a second prime must |
| v2(b) | 12-cell row-burst placements at N = 32 | 672 total; **420 (62.5%) inside one hemisphere**, 252 (37.5%) straddle the fold line and contain exactly one Fold cell |

## THE BARS

### Shared

| bar | needed to count as met |
| --- | --- |
| **S1** the histogram | random mean within **1.00 ± 0.02**, the histogram fits `2^−(k+1)`; counts of `≥ 28` per corpus recorded |
| **R** round-trip | bytes → `toCells` → `pushLeft` (stalk.js via vm) → `valueOf` → bytes, exact, 7 shapes; the restated `pushLeft` equals the site's on 500 squares |
| **T** trit singles | on canonical squares, `d = ±1` (a lit cell greened or a green lit) and `d = ±2` (a sign flip), 3000 each: **3000/3000, 0 wrong** with the canonicity filter and `q`; the *bare* rate (no filter, no `q`) recorded, predicted ~25% ambiguous |

### v2(a)

| bar | needed to count as met |
| --- | --- |
| **A1** in-band round-trip | every in-band square round-trips with the checks extracted from its tail; the checks are zeroed and `V` is exact |
| **A2** the fallback rate | exact per-corpus fraction of squares that fall back to external residues: predicted **≥ 99%** everywhere but a zero-padded binary |
| **A3** the flag | a flag-bit flip in either direction is **detected**, never silently wrong |
| **A4** cost | `4.69%·(1 − f) + 1/1024` = **4.79%** random, **4.75%** text — *worse than v0 on text* |
| **A5** the name | the pushed form is the all-nonzero signed-binary recoding (Joye–Tymen / Okeya–Takagi); the arm is v0 over a ternary channel with a padding-resident checksum |

**MET** if S1 matches the geometric law and **0 / 10,000** random squares carry the check —
i.e. the free lunch is falsified and the README says so plainly. **MISSED** if the random
mean > 2 (which would mean `pushLeft` was misread).

### v2(b)

| bar | needed to count as met |
| --- | --- |
| **B1** the cap | erasure cap enforced at 10 per region by default; the 12-cell channel runs at cap 12 and says so |
| **B2** bits-as-trits burst | 12-cell flagged row burst on bit squares declared trit, 400 trials, cap 12: **88–94%**, called **92% (368/400)**, rest ambiguous, **0 wrong** |
| **B3** canonical burst | the same burst on canonical squares with the canonicity filter: **100%** |
| **B4** the name | the Wub's two-valued roll is the canonical form's erasure model; the arm re-measures v0's erasure path in base 3 |

**MET** if 85–96% on bits-as-trits and 100% canonical; **MISSED** if ≥ 99% or < 80%.

## Calibration, stated before the numbers

I expect S1 to land on the geometric law almost exactly — it is a theorem about `pushLeft`,
not a property of data — and I expect **v2(a) to be discarded by the house on its own
measurement**: the slack is a suffix of median length ≤ 1, the flag bit costs more than
the slack saves, and only zero-padded binaries carry the check at any rate worth a
number. I expect the PE to be the one corpus where the rate is not ~0 (call 10%), and the
`.zst` to be the cleanest of all (< 0.5%). v2(b) I expect to land inside its range with
the arithmetic of `3^12 / p / q`; a landing at ≥ 99% would mean the readings are far fewer
than counted and the base-3 budget was overstated. The bar I am least sure of is B2's
exact rate; the mechanism is not in doubt.

## Per-stage predictions

### S1 — the histogram (files v2(a)'s verdict)

| corpus | mean greens | ≥ 28 greens (in-band) |
| --- | --- | --- |
| 10,000 random squares | **1.00 ± 0.02**, `P(k) = 2^−(k+1)` to within binomial noise | **0 / 10,000** |
| `spec.md` (119 squares) | ~1 | **1 / 119** — the padded last block |
| `stalk.js` (153 squares) | ~1 | **1 / 153** — the padded last block |
| `og.png` (264 squares) | ~1 | 0 or 1 |
| a PE (`program.exe`, `notepad.exe`) | > 1 — zero-padded sections | **5–20%**, call **10%** |
| `archive.zst` | ~1 | **< 0.5%** |

### S2 — round-trip and the restated `pushLeft`

| claim | predicted |
| --- | --- |
| restated `pushLeft` = `stalk.js pushLeft`, 500 random squares | identical — a miss is a bug |
| canonical form is a `±1` prefix + green tail; tail = `v₂(V)` | holds on every square tried |
| bytes → cells → push → `valueOf` → bytes, 7 shapes | exact |
| `G.toBytes` on a pushed square | **wrong** whenever the square has a `−1` — recorded as the reason `valueOf` exists |

### S3 — trit singles on canonical squares

| channel | predicted |
| --- | --- |
| `d = ±1` (lit ↔ green), 3000, filter + `q` | 3000/3000, 0 wrong |
| `d = ±2` (sign flip), 3000, filter + `q` | 3000/3000, 0 wrong |
| the same, bare (no canonicity filter, no `q`) | ~25% ambiguous, 0 wrong — the `2w[j] = w[j−1]` alias survives the alphabet about half the time and is never wrong, only undecided |
| codegg-v1 in trit mode on the same squares | 3000/3000 (two primes, confirm per candidate) |

### S4 — v2(a)

| claim | predicted |
| --- | --- |
| in-band squares round-trip with checks extracted | exact, every one found by S1 |
| fallback rate: random / `spec.md` / `stalk.js` / `og.png` / PE / `.zst` | 100% / 99.2% / 99.3% / ~100% / ~90% / > 99.5% |
| flag flipped 0 → 1 on an out-of-band square | detected (garbage trits fail the residues) |
| flag flipped 1 → 0 on an in-band square | detected (no external check to read) |
| damage inside the check slots of an in-band square | detected |
| overhead: random / `spec.md` | **4.79% / 4.75%** — worse than v0's 4.69% |
| push invariance | vacuous: the stored square *is* push's fixpoint |

### S5 — v2(b)

| channel | predicted |
| --- | --- |
| 12-cell flagged row burst, bits-as-trits, cap 12, 400 | **88–94%, called 92% (368/400)**, rest ambiguous, 0 wrong — in-region placements (62.5%) leave `3^12 / p ≈ 259` readings and a spurious one passes `q` w.p. ≈ 12.5%; straddling ones (37.5%) ≈ 100% |
| the same at the default cap 10 | in-region placements refused as "too many erasures": **~37%** corrected, 0 wrong |
| 12-cell flagged row burst, **canonical** squares, filter on | **100%** |
| 12-cell flagged burst, bit squares, bit alphabet (v0's path) | 400/400 — the control inside the arm |
| erasure cap | 10 per region enforced; a region with 11 flagged cells is refused unless `cap` is raised |

## Predicted standings (spec.md, 400 trials) — filed before building

Rows are the eight of v0's `versus.js` plus the two new channels; every arm of v0 and v1
sits at the table. The two rows in bold are what v2 adds. `—` = the arm's alphabet cannot
hold the channel's square.

| channel | codegg-v1 | v0 | v1(a) | v1(b) | v1(c) | v2(a) | v2(b) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 flip | 400 | 400 | 400 | 400 | 400 | 400 | 400 |
| 2 anywhere | 344 | 191 | ≥ 396 | ≥ 398 | 191 ± n | ~191 | ~191 |
| 2 same-region | 353 | 130 | ≥ 396 | ≥ 398 | ~130 | ~130 | ~130 |
| 2 cross-region | 349 | 400 direct | 400 direct | 400 | 400 direct | 400 direct | 400 direct |
| 3 one-per-region | 0 (44 wrong) | 400 direct | 400 direct | 400 | 400 direct | 400 direct | 400 direct |
| 12 flagged burst | 400 | 400 | 400 | 400 | 400 | 400 | **~368** (bits-as-trits, cap 12) |
| 12 unflagged in-region | 0 | 0 | 0 | ≥ 396 | 0 | 0 | 0 |
| Fold filled | 340 det / 60 wrong | 400 det | 400 det | 400 det | 400 det | 400 det | 400 det |
| **1 sign flip, canonical square** | 400 (trit mode) | — | — | — | — | **400** | **400** |
| push | 200/200 | 0 | 0 | 0 | 0 | vacuous | vacuous |
| overhead | 2.34% | 4.69% | 5.86% | 103% (48% share) | 4.69% | 4.75% | 4.69% |

**Who the house is predicted to keep:** v1(a), as before. v1(b) buys one row at 18× the
cost. v1(c), v2(a), v2(b) add nothing over v0, and v2(a) costs a flag bit more. The
measured table replaces this one in the README; every cell that moves is a recorded miss.

## The bar arithmetic, filed plainly

| bar | needs | call |
| --- | --- | --- |
| S1 | mean 1.00 ± 0.02, geometric | **YES** — a theorem, measured |
| R | exact | **YES** |
| T | 3000/3000 ×2, 0 wrong | **YES** with filter + `q` |
| A1 | in-band round-trip | **YES** on the handful that exist |
| A2 | ≥ 99% fall back | **YES** everywhere but the PE |
| A3 | flag flips detected | **YES** |
| A4 | 4.79% / 4.75% | **YES** by arithmetic |
| A5 | a name | **YES**: all-nonzero signed-binary recoding + padding-resident checksum |
| B1 | cap 10 | **YES** |
| B2 | 88–94% | **YES, ~92%** — the uncertain one |
| B3 | 100% canonical | **YES** |
| B4 | a name | **YES**: the Wub's coin is the canonical erasure model |

## Measured (filled as stages land — never before)

Filled 2026-09-02, after `tools/greens.js`, `tools/eggso2.test.js`, `tools/versus.js
../spec.md --trials 400 --json`, `tools/standings.js ../spec.md --trials 400 --json` and
`tools/corrupt.js` on `stalk.js`, `wubbadub.html` and `notepad.exe`. Every number here is
from those runs; the JSON beside this file (`measured-*.json`) is what they wrote.

### S1 — the histogram: HELD, and it files v2(a)'s verdict

| corpus | called | landed |
| --- | --- | --- |
| 10,000 random squares | mean 1.00 ± 0.02, `2^−(k+1)` | mean **1.011**; law holds at every k = 0..6 (worst \|z\| 1.58: 4955/5000, 2513/2500, 1264/1250, 624/625, 340/313, 147/156, 78/78); max tail 13; **0 / 10,000 in-band** |
| `spec.md` (119) | 1/119 | **1/119** (0.84%), the padded last block, tail 337 |
| `stalk.js` (153) | 1/153 | **1/153** (0.65%), the padded last block, tail 137 |
| `og.png` (264) | 0 or 1 | **2/264** (0.76%) — MISSED by one: the padded last block (tail 401) and one interior run of zero bytes |
| `program.exe` (2072) | 5–20%, call 10% | **169/2072 = 8.16%**; mean tail 13.2 |
| `notepad.exe` (2816) | 5–20% | **416/2816 = 14.77%**; mean tail 94.7 — whole zero pages, max 1024 |
| `archive.zst` (7396) | < 0.5% | **1/7396 = 0.01%** |
| closed form `tail = v₂(V)` | holds | **0 mismatches** on 20,820 squares |

### S2 — round-trip and the restated `pushLeft`: HELD

| claim | called | landed |
| --- | --- | --- |
| restated `pushLeft` = `stalk.js`'s | identical | **500/500**; `V` conserved 500/500 |
| fixpoint is a `±1` prefix + green tail; tail = `v₂(V)` | holds | **500/500 · 500/500** |
| round-trip by BigInt, 7 shapes, both arms | exact | **exact** |
| `G.toBytes` on pushed squares | wrong | **wrong on 64/64** |
| trit alphabet, one prime: distinct syndromes | 2050 (2046 collide) | **2050 of 4096** |

### S3 — trit singles: HELD with the filter, MISSED on the bare rate

| channel | called | landed |
| --- | --- | --- |
| `d = ±1`, filter + `q`, 3000 | 3000/3000 | **3000/3000, 0 wrong** |
| `d = ±2` sign flip, filter + `q`, 3000 | 3000/3000 | **3000/3000, 0 wrong** |
| bare, no filter no `q` | ~25% ambiguous | **45.3% / 45.7% ambiguous** (1358, 1370 of 3000), 0 wrong — MISSED. The alias `(j−1, ±1)` survives the alphabet whenever `cells[j−1]` has the sign that lets it, which on a `±1` prefix is half the time, not a quarter |
| filter alone / `q` alone | — | filter alone **3000/3000**; `q` alone **1661/3000** |
| codegg-v1 in trit mode | 3000/3000 | **1515/3000**, 0 wrong — MISSED, and for a reason worth the round: `2·2^(L−1−j) = 2^(L−j)` is an *integer* identity, so the alias passes `q` (and any residue) exactly; only a spelling rule can see it. The canonicity filter is that rule |

### S4 — pairs on canonical squares, and the control that was not filed

| channel | called | landed |
| --- | --- | --- |
| same-region pairs, 1000 | ~130/400 in the standings (v0's rate) | **1000/1000, 0 ambiguous, 0 wrong** — MISSED on the good side. Cause: the plan's own instruction to confirm *per candidate* (`codegg.js:204-206`) inside the search, plus the canonicity filter |
| 2 anywhere · cross-region · Fold filled | — | **1000/1000** (464 by search) · **1000/1000** direct · **300/300 detected, 0 wrong** |
| **v0's search on plain BIT squares with `q` per candidate** — added after S4, filed at ~97% before running | ~97%, ~3% ambiguous, 0 wrong | **978/1000 = 97.8%**, 22 ambiguous, **0 wrong** |

**What was not filed and should have been, and changes the lineage's record:** eggSo-v0's
"2 of 921 same-region pairs" was a consequence of *where* v0 applied its confirming residue
— after collecting the plan, refusing at the second in-region candidate — not of the
partition. codegg-v1's rule, `q` inside the loop, takes 97.8% of them on v0's own bit
squares at v0's own 4.69%. v1(a)'s twelve bits still buy something v0-with-per-candidate-q
does not have: the pair is *named* by lookup (254 direct of 400 on spec.md), not searched,
and 100% rather than 97.8%. But the gap the v1 plan was written against was 2 vs 400, and
the honest gap is 391 vs 400.

### S5 — v2(a): HELD on every bar, and discarded by the house on its own numbers

| claim | called | landed |
| --- | --- | --- |
| in-band round-trip, checks in the tail, 200 | exact | **200/200**; a `d = ±1` error **200/200**, a sign flip **200/200** corrected |
| check-slot damage · flag 1→0 · flag 0→1 | detected, never silently wrong | **200/200 · 200/200 · 200/200 detected, 0 silently wrong** |
| fallback: random / spec / stalk / og / PE / zst | 100 / 99.2 / 99.3 / ~100 / ~90 / > 99.5 % | **100 / 99.16 / 99.35 / 99.24 / 91.84 & 85.23 / 99.99 %** — 4 of 6 corpora ≥ 99%; the PEs do not, as called |
| overhead random / `spec.md` | 4.79% / 4.75% | **4.79% / 4.76%** (the last square is partial, so per data bit it rounds up); `notepad.exe` **4.09%** vs v0's 4.69% — the one place the slack is worth anything |
| versus/standings, spec.md | ~v0 | **400/400 on every channel, 0 wrong**, sign flip 400/400 — carried by the trit decoder, not by the slots |

### S6 — v2(b): B2 MISSED at 1/400, and the miss is the result

| channel | called | landed |
| --- | --- | --- |
| cap 10 enforced | yes | **yes** — 11 flagged in one region → "too many erasures" |
| canonical 12-cell flagged burst, two-valued | 100% | **400/400** |
| bit-alphabet control | 400/400 | **400/400** |
| **bits-as-trits 12-cell row burst, cap 12** | 88–94%, called 368/400 | **1/400 corrected, 399 ambiguous, 0 wrong** — MISSED. `1·2^k − 1·2^(k−1) = 0·2^k + 1·2^(k−1)`: for any two *adjacent* flagged cells the spellings `(1,−1)` and `(0,1)` are the same integer, so `3^12 / p` counted readings that no residue can tell apart. Every "01" in the window is an alias; 12 random bits have one with probability 1 − 13/4096 |
| the same at cap 10 | ~37% | **1/400**: 253 refused as too many, 146 ambiguous |
| 12 **scattered** non-adjacent flagged cells, bits-as-trits, cap 12 — added after the miss, filed at ~88% | ~88% | **361/400 = 90.25%**, 39 ambiguous, **0 wrong** — the plan's `3^12 / p / q` arithmetic was right for cells that do not touch |
| scattered, canonical squares | — | **400/400** |
| push | vacuous | **200/200 fixpoints** — vacuous, as filed |

### Standings (spec.md, 400 trials, `tools/standings.js`) — measured, replacing the filed table

| channel | codegg-v1 | eggSo-v0 | v1(a) | v1(b) | v1(c) | v2(a) | v2(b) | v2(b)·bits-as-trits |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 cell hit | 400 | 400 | 400 | 400 | 400 | 400 | 400 | 298 |
| 2 anywhere | 339 | 198 | 400 | 400 | 198 | 400 | 400 | 231 |
| 2 same-region | 338 | 120 | **400** (254 direct) | 400 (254 direct) | 120 | 400 (0 direct) | 400 (0 direct) | 277 |
| 2 cross-region | 343 | 400 | 400 | 400 | 400 | 400 | 400 | 251 |
| 3 one-per-region | 0 · **36 W** | 400 | 400 | 400 | 400 | 400 | 400 | 228 |
| 12 flagged burst | 400 | 400 | 400 | 400 | 400 | 400 | 400 | **6** |
| 12 unflagged in-region | 0 · **37 W** | 0 | 0 | **400** | 0 | 0 | 0 | 0 · **15 W** |
| Fold filled | 0 · **62 W** | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **1 sign flip, canonical** | 199 (trit mode) | — | — | — | — | **400** | **400** | — |
| push holds | 200/200 | 0 | 0 | 0 | 0 | vacuous | vacuous | 0 |
| cost per data bit | 2.35% | 4.70% | 5.88% | 103.41% | 4.70% | 4.76% | 4.70% | 4.70% |
| cost, share of all bits stored | 2.29% | 4.48% | 5.54% | 50.75% | 4.48% | 4.53% | 4.48% | 4.48% |

Cells that moved from the filed table, each a recorded miss: v2(a) and v2(b) on `2
anywhere` and `2 same-region` (~191 / ~130 → 400, the per-candidate confirm); v2(b) on the
flagged burst (~368 → 400 on canonical squares; the bits-as-trits configuration the 368 was
filed for lands at **6**); codegg-v1 on the sign flip (400 → 199, the integer alias);
codegg-v1 on the unflagged burst and the Fold filled (0 → 37 and 62 *wrong*, not detected).

**Who the house keeps, by the mechanical rule** (cheapest arm correcting ≥ 99% of a row
with 0 wrong on every channel): the pair rows go to **v2(b) at 4.70%** — and v2(b) earns
them not with greens but with the per-candidate confirm and the canonical form as a
spelling filter; v1(a) at 5.88% is the only arm that *names* those pairs (254 direct).
The unflagged burst goes to **v1(b) at 103%**, alone. The sign flip goes to the trit
arms. The Fold filled goes to nobody. codegg-v1 and the bits-as-trits configuration are
disqualified from every row for returning wrong data.

## THE CLOSING AUDIT — every bar, called vs landed

| bar | called | landed | verdict |
| --- | --- | --- | --- |
| S1 the histogram | mean 1.00 ± 0.02, geometric | 1.011, law holds, 0/10,000 in-band; og.png 2 not ≤ 1 | **MET**, one corpus off by one |
| R round-trip | exact | exact; `pushLeft` = site's 500/500 | **MET** |
| T trit singles | 3000/3000 ×2; bare ~25% | 3000/3000 ×2, 0 wrong; bare **45%** | **MET**, bare rate MISSED |
| A1 in-band round-trip | exact | 200/200, damage corrected 200/200 | **MET** |
| A2 fallback ≥ 99% | everywhere but the PE | 4 of 6 corpora; PEs 91.8% and 85.2% | **MET** as called |
| A3 the flag | detected both ways | 200/200 both ways, 0 silently wrong | **MET** |
| A4 cost | 4.79% / 4.75% | 4.79% / 4.76% | **MET** |
| A5 a name | all-nonzero signed-binary recoding + padding-resident checksum | holds; and the arm's decoder is what carried it, not the slots | **MET** |
| **v2(a) overall** | MET iff 0/10,000 random squares carry the check | **0/10,000** — the free lunch is falsified | **MET, and the house discards the arm**: 1 flag bit for a saving that appears only on zero-padded binaries |
| B1 the cap | 10 | enforced | **MET** |
| B2 bits-as-trits burst | 88–94% | **0.25%** — adjacent spellings are equal integers | **MISSED**, the reason is the finding |
| B3 canonical burst | 100% | 400/400 | **MET** |
| B4 a name | the Wub's coin is the canonical erasure model | holds, and sharper: it is the *only* erasure model that decodes a run, because the canonical form is a spelling rule and three-valued erasures have none | **MET** |

Twelve bars, eleven met, one missed for a reason that is the round's best result. The
free lunch in the greens does not exist on data that is not already zero: the slack is
`v₂(V)`, mean 1, and a flag bit costs more than it saves anywhere but inside a PE. Greens
as three-valued erasures cannot be decoded on a run at all — the signed-digit alphabet's
redundancy, the very identity `pushLeft` is built on, makes adjacent spellings equal as
integers. The Wub's two-valued roll is not an approximation of the erasure model; it is
the erasure model. And a correction to the lineage's own record: v0 could always have
taken 97.8% of its same-region pairs by confirming per candidate.
