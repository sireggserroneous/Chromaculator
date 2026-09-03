# eggSo v2 — the green is the code

Not part of the site. The sixteenth codec experiment and the third in the fold-native
lineage — [`eggSo-v0/`](../eggSo-v0/) used the fold's partition, [`eggSo-v1/`](../eggSo-v1/)
its symmetry, and this round its alphabet's own slack, the **green**, the `0` of the
signed-digit alphabet. Kept in its own folder so it does not entangle with
`chronochromatic.org`, which claims none of this.

Built 2026-09-02 against [PREDICTIONS.md](PREDICTIONS.md), filed before a line of the
codec was written. Two arms, each built and measured — *"we need to test all ways, to see
who the house keeps. No free lunch here"* — and a standings table across every arm of v0,
v1 and v2 so the house can keep or discard each on the record.

## The verdict, first

**Twelve bars, eleven met, one missed for a reason that is the round's best result.
There is no free lunch in the greens, and the house discards both arms.**

| arm | what it is | the bar that mattered | landed |
| --- | --- | --- | --- |
| v2(a) — forced greens as check slots | a canonical square whose value has ≥ 28 trailing zero bits carries its four residues in its own tail, plus one flag bit | 0 of 10,000 random squares carry the check, i.e. the lunch is falsified | **0 / 10,000**; spec.md 1/119, stalk.js 1/153, archive.zst 1/7396; only PEs 8–15% |
| v2(b) — greens as 3-valued erasures | v0's erasure path in base 3 | 12-cell flagged burst on bit squares declared trit, 88–94% | **1 / 400** — MISSED, structurally |

The two sentences the round was for:

**The slack is the 2-adic valuation of the value.** After `pushLeft` the lit cells are a
`±1` prefix and every green is trailing, so the number of greens is the number of trailing
zero bits of `V`: `P(k) = 2^−(k+1)`, mean 1. Measured on 10,000 random squares: mean 1.011,
the law holding at every `k`. A square carries its checks in-band iff its 128-byte block
ends in 3.5 zero bytes, which random data never does, text does once per file (the padded
last block), and a Windows PE does 8–15% of the time. A flag bit per square costs more
than the slack saves anywhere but inside a PE.

**Greens cannot be decoded as three-valued erasures on a run.** `1·2^k − 1·2^(k−1)` and
`0·2^k + 1·2^(k−1)` are the same integer, so for any two adjacent flagged cells the
spellings `(1,−1)` and `(0,1)` are indistinguishable to *every* value-based check — the
same identity `pushLeft` is built on (`stalk.js:59`). The plan's `3^12 / p ≈ 259 readings`
counted spellings that no residue can separate. On scattered, non-adjacent cells the
arithmetic was right: **361/400**. On a contiguous run: **1/400**, 399 ambiguous, 0 wrong.
The Wub's two-valued roll (`wub.html:282-292`) is therefore not an approximation of the
erasure model; it is the only one that decodes, because the canonical form is a spelling
rule and three-valued erasures have none. On canonical squares the two-valued model takes
the burst **400/400**.

## What failed, first

- **B2 missed at 0.25%.** Above. The miss is the finding.
- **The bare single rate was called ~25% ambiguous and landed 45%.** The alias `d = ±1 at
  j−1` for a sign flip at `j` survives the alphabet whenever `cells[j−1]` has the sign
  that permits it, which on a `±1` prefix is half the time.
- **codegg-v1 in trit mode was called 3000/3000 on singles and landed 1515/3000.** The
  alias `2·2^(L−1−j) = 2^(L−j)` is an *integer* identity: it passes `p`, passes `q`,
  passes any residue. Only a spelling rule can see it. The canonicity filter is that rule,
  and with it v2 takes 3000/3000 on both `d = ±1` and `d = ±2`. The plan's "two primes per
  region is unnecessary" was true for a stronger reason than it gave: two primes would
  not help at all.
- **`og.png` was called 0 or 1 in-band and landed 2.** One interior run of zero bytes.
- **The standings' pair rows were filed at v0's rate (~130/400) and landed 400/400** for
  both v2 arms. Not greens: the plan's own instruction to confirm *per candidate*
  (`codegg.js:204-206`) inside v0's in-region search, plus the canonicity filter.
- **Not filed and should have been, and it changes the lineage's record.** v0's "2 of
  921 same-region pairs" was a consequence of *where* v0 applied its confirming residue,
  not of the partition. codegg-v1's rule — `q` inside the loop — takes **978/1000** of
  them on v0's own bit squares at v0's own 4.69% (filed at ~97% before running). v1(a)'s
  twelve extra bits still buy what that does not: the pair *named* by lookup (254 direct
  of 400), not searched, and 100% rather than 97.8%. But the gap the v1 plan was written
  against was 2 vs 400, and the honest gap is 391 vs 400.

## Why this round exists

The lineage audit found **puncture** at zero mentions across thirteen versions, beside
anti-transpose and involution. v1 used the map. This round used the alphabet's slack, and
existed mainly to file a verdict on it before anyone built a codec on the promise: the
canonical form forces every green trailing (`stalk.js:416-427`, `spec.md:55-64`), the Wub
rolls a green as a coin (`wub.html:282-292`), and codec-v1 had already measured that
canonicity *as detection* catches 0 of 50,000 sign flips (`codec-v1/README.md:15-20`). Two
readings remained — greens as storage, greens as erasures — and both are now measured.

## The construction

The stored square is the **canonical one**: `G.toCells` lays the bytes row-major as v0
does, `pushLeft` (restated from `stalk.js:59-71` and asserted against the site's own
function on 500 squares) respells them into a `±1` prefix and a green tail, and the bytes
come back from `V` by BigInt — `G.toBytes` reads `cells[j] === 1` and is wrong on 64 of 64
pushed squares. Trit alphabet `d ∈ {±1, ±2}`: per-region tables after `codegg.js:90-102`,
v0's `p` per region and `q` over the square, `q` applied **per candidate** as codegg-v1
does, and a **canonicity filter** — a repair that leaves a green before a lit cell is
refused. The `−1` sentinel erasure is disabled; `−1` is a digit here.

**v2(a).** Four residues × 7 balanced-ternary trits = 28 cells. In-band iff `v₂(V) ≥ 28`;
the check sits in fixed slots `L−28..L−1`, which the canonical form forces green, and the
decoder zeroes them after reading. Whether a square is in-band cannot be derived from a
damaged square, so one **flag bit** per square rides out of band; out-of-band squares carry
v0's four residues externally. Cost `4.69%·(1 − f) + 1/1024`.

**v2(b).** `eggso.js:142-191` in base 3, cap 10 per region (`3^10 ≤ 2^16`), cap 12 where
said. On canonical squares the **two-valued model**: a flagged cell before the last lit
cell is `±1`, one in the tail is `0`, and the boundary is one of a handful.

| | |
| --- | --- |
| **borrowed** | regions, per-region residues, the confirming residue — eggSo-v0's, `require`d. Moduli, weights, the row-major layout, the trit syndrome table with its `2w[j] = w[j−1]` aliases, the per-candidate confirm — codegg-v1's, `require`d. Nothing copied |
| **the site's, restated and pinned** | `pushLeft` (`stalk.js:59-71`) and `valueOf` (`stalk.js:74-83`), each asserted against the site's own function |
| **the names** | the pushed form is the all-nonzero signed-binary recoding (Joye–Tymen, Okeya–Takagi); v2(a) is v0 over a ternary channel with a padding-resident checksum; v2(b) is v0's erasure path in base 3, and the Wub's coin is its only decodable case |

## Results, all measured

### S1 — the histogram (`tools/greens.js`)

| corpus | squares | mean greens | ≥ 28 (in-band) | fall back |
| --- | --- | --- | --- | --- |
| random bits | 10,000 | **1.011** | **0** | 100% |
| `spec.md` | 119 | 4.72 | 1 (the padded last block) | 99.16% |
| `stalk.js` | 153 | 2.25 | 1 (the padded last block) | 99.35% |
| `og.png` | 264 | 3.56 | 2 | 99.24% |
| `program.exe` | 2,072 | 13.2 | 169 (8.16%) | 91.84% |
| `notepad.exe` | 2,816 | 94.7 | 416 (14.77%) | 85.23% |
| `archive.zst` | 7,396 | 1.09 | 1 | 99.99% |

The geometric law on the random run: 4955/5000 · 2513/2500 · 1264/1250 · 624/625 ·
340/313 · 147/156 · 78/78 for `k = 0..6`, worst |z| 1.58. `tail = v₂(V)` on all 20,820
squares.

### The standings (`tools/standings.js ../spec.md --trials 400`)

Every arm of the lineage against codegg-v1 on the same file, same squares, same damage
positions. Corrected, with miscorrections as `/nW`; `—` = the alphabet cannot hold the
channel; codegg-v1 takes the sign-flip row in its trit mode.

| channel | codegg-v1 | eggSo-v0 | v1(a) | v1(b) | v1(c) | v2(a) | v2(b) | v2(b)·bits-as-trits |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 cell hit | 400 | 400 | 400 | 400 | 400 | 400 | 400 | 298 |
| 2 cells, anywhere | 339 | 198 | 400 | 400 | 198 | 400 | 400 | 231 |
| 2 cells, same region | 338 | 120 | **400** | 400 | 120 | 400 | 400 | 277 |
| 2 cells, different regions | 343 | 400 | 400 | 400 | 400 | 400 | 400 | 251 |
| 3 cells, one per region | 0 · **36 W** | 400 | 400 | 400 | 400 | 400 | 400 | 228 |
| 12-cell row burst, flagged | 400 | 400 | 400 | 400 | 400 | 400 | 400 | **6** |
| 12-cell row burst, **unflagged**, in-region | 0 · **37 W** | 0 | 0 | **400** | 0 | 0 | 0 | 0 · **15 W** |
| the Fold filled, 32 unflagged | 0 · **62 W** | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **1 sign flip, canonical square** | 199 | — | — | — | — | **400** | **400** | — |
| push: checks still hold | 200/200 | 0 | 0 | 0 | 0 | vacuous | vacuous | 0 |
| cost per data bit | 2.35% | 4.70% | 5.88% | 103.41% | 4.70% | 4.76% | 4.70% | 4.70% |
| cost, share of all bits stored | 2.29% | 4.48% | 5.54% | 50.75% | 4.48% | 4.53% | 4.48% | 4.48% |

**Direct** — corrected by a syndrome naming its own cell, no search — is the column the
fold-native claims are about: on `2 same-region`, v1(a) and v1(b) name 254 of 400 and
every other arm names 0; on the unflagged burst v1(b) names all 400; on `2 anywhere` v0,
v1(c), v2(a) and v2(b) name ~197 (the cross-region half) and v1(a) 399.

**Who the house keeps, by the mechanical rule** — the cheapest arm correcting ≥ 99% of a
row with 0 wrong on every channel:

| row | kept by |
| --- | --- |
| 1 cell · 2 cross-region · 3 one-per-region · 12 flagged burst | eggSo-v0, v1(c), v2(b) at 4.70% |
| 2 anywhere · 2 same-region | **v2(b) at 4.70%** — by search, with the per-candidate confirm; v1(a) at 5.88% is the only arm that *names* them |
| 12 unflagged in-region burst | **v1(b) at 103%**, alone |
| 1 sign flip, canonical | v2(b) at 4.70% |
| the Fold filled | nobody |
| disqualified for returning wrong data | codegg-v1, v2(b)·bits-as-trits |

Read it plainly. Nothing v2 adds over v0 comes from greens: v2(a)'s slots are used by 1
square in 119 and its decoder is v0's; v2(b)'s pair rows come from `q` inside the loop,
which v0 could have had. The one row only a fold-native arm holds is v1(b)'s burst, at
eighteen times v1(a)'s price. The one thing only v1(a) does is *name* a same-region pair.

### What the suite pins down (`tools/eggso2.test.js`)

| claim | result |
| --- | --- |
| `pushLeft` = `stalk.js pushLeft` · `V` conserved · fixpoint is `±1` prefix + tail · tail = `v₂(V)` | 500/500 each |
| trit syndromes, one prime | 2050 distinct of 4096: 2046 collide, all `2w[j] = w[j−1]` |
| round-trip by BigInt, 7 shapes, both arms · `G.toBytes` on pushed squares | exact · wrong 64/64 |
| trit singles, filter + `q`: `d = ±1` / sign flip | **3000/3000 / 3000/3000**, 0 wrong |
| the same, bare / filter alone / `q` alone / codegg-v1 trit | 45% ambiguous / 3000 / 1661 / 1515 |
| pairs on canonical squares: anywhere / same-region / cross / Fold filled | 1000 / **1000** / 1000 / 300 detected, 0 wrong |
| **v0's search on bit squares, `q` per candidate**, same-region pairs | **978/1000**, 22 ambiguous, 0 wrong |
| in-band round-trip 200 · data error / sign flip on in-band | 200 · 200 / 200 |
| check-slot damage · flag 1→0 · flag 0→1 | detected 200/200 each, 0 silently wrong |
| overhead: random / spec.md / notepad.exe | 4.79% / 4.76% / **4.09%** (v0 4.69%) |
| cap 10 enforced · canonical burst · bit-alphabet control | yes · **400/400** · 400/400 |
| bits-as-trits 12-cell burst, cap 12 / cap 10 | **1/400** (399 ambiguous) / 1/400 (253 too many, 146 ambiguous) |
| 12 scattered non-adjacent, bits-as-trits cap 12 / canonical | **361/400**, 0 wrong / 400/400 |
| push | 200/200 fixpoints — vacuous |

### The honest section

- **The slots do no work on real data.** On `spec.md` one square in 119 is in-band. v2(a)'s
  400s in the standings are the trit decoder's, which is v0's arithmetic with the
  canonicity filter; the slots contribute one square. Where they do contribute — a PE with
  zero pages — the saving is 0.6 points and the flag bit is already paid for.
- **The trit alphabet's alias is invisible to arithmetic.** `2·w[j] ≡ w[j−1]` is an equality
  of integers. No prime, no number of primes, separates a sign flip at `j` from a `±1` at
  `j−1`; codegg-v1's two primes leave 50% ambiguous. The canonical form separates them by
  spelling, and that is the only reason v2's singles are 3000/3000.
- **Three-valued erasures are undecodable on runs**, for the same reason, and the plan's
  arithmetic did not see it. Scattered cells decode at 90%; a run decodes at 0.25%.
- **codegg-v1's erasure path is bit-only even in trit mode**: it "corrects" a flagged burst
  on a canonical square wrongly 162 of 400 times (`versus.js`). The control has its own
  hole in the trit alphabet.
- **v0 refused what it could have taken.** 978/1000 same-region pairs with `q` per
  candidate. This is on v0's record now, in its successor's file.
- **Push is vacuous, not survived.** The stored square is push's fixpoint. A respelling of
  the stored square would break every region residue exactly as it broke v0's.

## Running it

```
node eggSo-v2/tools/greens.js                                  # S1, before anything else
node eggSo-v2/tools/eggso2.test.js                             # nine claims, ~20 s (3^12 twice)
node eggSo-v2/tools/versus.js spec.md --trials 400 --json      # both controls, trit mode, both arms
node eggSo-v2/tools/standings.js spec.md --trials 400 --json   # every arm of v0, v1, v2
node eggSo-v2/tools/corrupt.js stalk.js --arm b --model sign --hits 40
node eggSo-v2/tools/corrupt.js codegg-v10/corpus-real/notepad.exe --arm a --model pair --hits 200
```

`corrupt.js` exits 0 (exact), 2 (detected, not exact) or 3 (silently wrong or
miscorrected). Across `stalk.js`, `wubbadub.html` and `notepad.exe`, both arms and every
model exit 0 or 2.

## Files

| | |
| --- | --- |
| `eggso2.js` | `pushLeft`, `valueOf`, `isCanonical`, `tailOf`, `toBytesV`; `makeCode` (trit tables), `repairSquare` (errors, base-3 erasures, the two-valued model); arm a (`encodeA`, `extractChecks`, `writeChecks`, `repairA`, `decodeA`, `sizesA`); arm b (`encodeB`, `decodeB`). Requires `../eggSo-v0/eggso.js` and `../codegg-v1/codegg.js` |
| `PREDICTIONS.md` | filed before building; measured after; misses kept |
| `tools/greens.js` | S1 — the histogram, run first |
| `tools/eggso2.test.js` | the nine claims |
| `tools/versus.js` | both controls (bit and trit mode) against both arms |
| `tools/standings.js` | every arm of the lineage, ten channels, cost in two conventions, and who keeps each row |
| `tools/corrupt.js` | real files through one arm, five damage models |
| `measured-*.json` | the tools' own record of the numbers above |

## What this is and is not

It is a measured *no*: the representation's slack is a suffix of mean length one, and its
alphabet's redundancy is exactly what makes a green undecodable as a free variable. It is
also two small corrections to the lineage's own record — v0's pair rate, and the reason
one prime is enough under trits. It is not a claim that the fold is anything other than
what the site's README says it is: held loosely — with its partition, its map and
its alphabet now each on the record, and each with a name.
Placed in [eggSo-v4](../eggSo-v4/) as a basin boundary: the Julia set of a degree-2 map, with Inner and Outer as its Fatou basins.
