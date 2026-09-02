# eggSo v0 predictions — filed 2026-09-02, BEFORE a line of the codec was written

The series convention: every number below is a guess, written down first, and the
measured value is filled in beside it afterwards or not at all. Misses stay. A
prediction that is quietly edited to match its result is worth less than no prediction.

## What this is, in one sentence

The fold's own partition — **Inner, Fold, Outer**, the three regions that sum back to
the number — used as the coding mechanism for the first time in fourteen experiments.

## Why this round exists

The lineage audit of codec-v1 through codegg-v12 found that every version reached prior
art by some road *other* than the fold: product codes, residue arithmetic, interleavers,
Reed–Solomon, PAQ. Across all thirteen, the words **Outer** and **anti-transpose** appear
zero times as regions or maps. The one construction the site cannot place was never the
mechanism. This round makes it the mechanism, alone, and asks two questions:

1. Does splitting the check along the fold buy a capability the flat check does not have?
2. When it is named against the literature, what is it called?

A clean *no* to (1) or a quick name for (2) is as good a result as the reverse. The
point is the verdict, not the win.

## The construction, stated before building

codegg-v1 stores one residue of the whole square's value: `V mod p`. A single-cell error
of size `d` at cell `i` moves it by `d · 2^(L−1−i) mod p`, which names the cell. Two
errors give one syndrome with two unknowns, and v1 falls back to search.

eggSo stores **three residues, one per region**: `I mod p`, `F mod p`, `O mod p`, where
`I + F + O = V` is the fold's own identity (`stalk.js regions()`, `spec.md`). The same
syndrome mechanism runs *inside each region*. Nothing else is new. The weight table, the
modulus choice and the injectivity proof are codegg-v1's, required rather than copied,
and credited.

At `N = 32` (`L = 1024`, the series' standard square):

| region | cells | share |
| --- | --- | --- |
| Inner (`r + c < 31`) | 496 | 48.4% |
| Fold (`r + c = 31`) | 32 | 3.1% |
| Outer (`r + c > 31`) | 496 | 48.4% |

## THE BARS

| bar | needed to count as met |
| --- | --- |
| **B1** — no regression | single-cell errors corrected at the same rate as codegg-v1 (v1 landed 3000/3000) |
| **B2** — the capability question | two-cell errors in **different** regions corrected without search, at ≥ 50% of all random two-cell errors |
| **B3** — cost | overhead ≤ 2× codegg-v1's 2.34%, i.e. ≤ 4.7% |
| **B4** — the honest floor | miscorrection rate ≤ codegg-v1's, measured on the same channels |
| **B5** — the name | a literature name for the construction, or a stated reason none fits |

## Calibration, stated before the numbers

I expect to hit B1, B3 and B5 and I expect B5's answer to be **"it has a name."** B2 is
the only bar with a real question in it, and I expect to hit it narrowly. B4 I am least
sure of: three residues give an error three chances to alias instead of one.

## Per-stage predictions

### S1 — the codec round-trips and the identity holds

| claim | predicted |
| --- | --- |
| `I + F + O ≡ V (mod p)` on every square, every prime tried | holds, by construction — a miss here is a bug, not a result |
| round-trip exact, 7 shapes as v1 | exact |
| moduli: one prime injective over `±2^k, k < 1024` | reuse v1's `pickModulus`; same `p = 2053`-class prime |

### S2 — single-cell errors (B1)

| channel | predicted corrected | predicted miscorrected |
| --- | --- | --- |
| uniform single flip, 3000 trials | **3000 / 3000** — the region residue names the cell exactly as v1's whole residue did | 0 |
| single flip in the Fold specifically | 3000 / 3000 — the Fold is 32 cells and the search space is smaller, not larger | 0 |

### S3 — two-cell errors (B2, the question)

Two random distinct cells land in different regions with probability

```
1 − [C(496,2) + C(32,2) + C(496,2)] / C(1024,2)  =  1 − 246,016 / 523,776  =  0.530
```

| channel | predicted |
| --- | --- |
| two cells, different regions | each region's residue names its own cell: **corrected, no search** |
| two cells, same region | one residue, two unknowns: falls to v1's search path; **~87%** corrected (v1 landed 1747/2000 = 87.4% on this) |
| all random two-cell, 2000 trials | **≈ 53% by direct syndrome + 47% × 87% by search ≈ 94%** overall; direct-syndrome share alone **0.50–0.56** → B2 met at mid |
| codegg-v1 on the same 2000 | 87% by search, 0% direct — the control |

The interesting number is not the 94%. It is the **53% that needs no search at all**,
which v1 cannot do at any rate. If that lands under 50%, B2 is missed and the partition
bought nothing a coin did not.

### S4 — bursts, and where the fold hurts

| channel | predicted |
| --- | --- |
| 12-cell row burst, flagged (v1: 800/800) | 800 / 800 — erasure decoding is unchanged |
| 12-cell burst **across the fold line**, i.e. spanning two regions | corrected — two regions each see part of it |
| 12-cell burst inside Inner, unflagged | detected, **not** corrected — 12 unknowns in one residue, same as v1 |
| **the Fold as a weakness**: a 32-cell burst exactly along the anti-diagonal | detected only. This is the one shape where the partition is worse than a random split: the Fold is a 3% region and a burst can fill it. Stated so it is looked for. |

### S5 — push invariance

| claim | predicted |
| --- | --- |
| `I`, `F`, `O` each invariant under `pushLeft` | **NO** — push moves colour across cells and therefore across region boundaries. `V` is conserved; its three parts are not. This is a real cost the flat residue does not pay, and it is predicted as a loss. |

### S6 — cost (B3)

| item | predicted |
| --- | --- |
| checks per square | 3 residues × ~11 bits → **4–6 bytes**, vs v1's 3 |
| overhead at 128 B / square | **3.1–4.7%**, vs v1's 2.34%. B3 met at the top of the range. |

### S7 — the name (B5)

Filed now, before any of it runs: I predict the construction is an **interleaved AN
code** — codegg-v1's residue check applied independently to three fixed, disjoint
subsets of the codeword, which is the standard way of buying multi-error correction from
a single-error code, and is what interleaving *is*. The subsets happen to be the fold's
regions rather than `i mod 3`. The interleaving pattern is the fold; the code is not.

If that prediction holds, the round's verdict is: **the fold supplied a partition, and
partitions are known. The fold itself remains unplaced.** Which is the same place the
README already stands, with one more attempt on the record.

## The bar arithmetic, filed plainly

| bar | needs | call |
| --- | --- | --- |
| B1 | 3000/3000 | **YES** — the mechanism is v1's, restricted |
| B2 | direct-syndrome ≥ 50% of two-cell | **YES at mid** (0.53), a narrow one |
| B3 | ≤ 4.7% | **YES at the top of the range** |
| B4 | miscorrect ≤ v1 | **coin flip** — three residues, three chances to alias |
| B5 | a name | **YES: interleaved AN code** |

## Measured (filled as stages land — never before)

Filled 2026-09-02, after `tools/eggso.test.js`, `tools/versus.js ../spec.md --trials 400`
and `tools/corrupt.js`. Every number here is from those runs; the JSON beside this file
(`measured-*.json`) is what the suite wrote.

### S1 — identity and round-trip: HELD

| claim | called | landed |
| --- | --- | --- |
| `I + F + O ≡ V (mod p)` | holds by construction | **500/500**, `p = 2053` |
| `regionOf` is `stalk.js regions()` | — | **22,139 cells, n = 2..40, cell for cell** |
| round-trip, 7 shapes | exact | **exact, both configs** |

### S2 — singles (B1): HELD

| channel | called | landed |
| --- | --- | --- |
| single, anywhere | 3000/3000, 0 wrong | **3000/3000, all direct, 0 wrong** |
| single, on the Fold | 3000/3000 | **3000/3000** |

### S3 — doubles (B2): HELD, and the shape of the trade came out clearer than called

| channel | called | landed |
| --- | --- | --- |
| cross-region share of random pairs | 0.530 | **0.539** (2000 trials); 0.524 on the bare run |
| cross-region pairs corrected, no search | all | **1079/1079, all direct** |
| same-region pairs, one prime | ~87% by search | **0 by search, 736 detected, 216 MISCORRECTED** — the call was wrong. One prime is ~11 bits per region; the in-region search saturates and aliases |
| same-region pairs, + confirm | — | 2 by search, 919 detected, **0 miscorrected** |
| overall two-cell corrected | ≈ 94% | **54%** with confirm. The 94% assumed v1's search rate inside a region; one prime cannot deliver it, and confirm refuses what it cannot confirm |
| direct-syndrome share (the bar) | 0.50–0.56 | **0.539 → B2 MET** |

**What was not filed and should have been:** on `2 cells anywhere`, codegg-v1 corrects
**344/400** and eggSo **191/400** (spec.md, 400 trials). eggSo wins on *direct* (191 vs 0)
and loses on *total*. The partition trades total two-error capacity for search-free
correction of the pairs that straddle it. I called the gain and did not call the loss.

### S4 — bursts: one HELD, one MISSED and fixed, one MISSED for the bare form

| channel | called | landed |
| --- | --- | --- |
| 12-cell row burst, flagged | 800/800 | bare: **292/800, 508 ambiguous** — MISSED. 2^12 assignments against an 11-bit residue leaves ~2 standing. With confirm reaching the erasure decoder: **800/800** |
| burst straddling the fold line | corrected | **400/400**, both configs |
| the Fold filled, 32 unflagged | detected only | bare: 237 detected, **63 MISCORRECTED** — MISSED. + confirm: **300 detected, 0 wrong** — held. codegg-v1 on the same channel: 340 detected, **60 wrong** |

### S5 — push: the predicted loss, exactly

| claim | called | landed |
| --- | --- | --- |
| v1's `V` residue survives push | holds | **200/200** |
| eggSo's three parts survive push | NO | **0/200** |

### S6 — cost (B3): HELD, by 0.01

| item | called | landed |
| --- | --- | --- |
| residues per square | 3 | **4** — confirm is not optional in practice (see S3, S4) |
| overhead | 3.1–4.7% | bare **3.52%**; with confirm **4.69%** vs bar 4.70%. codegg-v1: 2.34% |

### B4 — the honest floor: MISSED bare, HELD with confirm

| channel | one prime | + confirm | codegg-v1 |
| --- | --- | --- | --- |
| same-region doubles, 2000 | 515 ok, 1158 det, **327 wrong** | 553 ok, 1447 det, **0 wrong** | 0 wrong |

The coin flip landed on the bad side and then confirm picked it up. Three residues give an
alias three chances; a fourth over the whole square takes them all back.

### S7 — the name (B5): HELD, with a sharper sentence than filed

An **interleaved AN code**: codegg-v1's residue check applied independently to three fixed
disjoint subsets, with the whole-square confirming residue making it a two-level residue
code (component checks plus an overall check — the product-code shape again). The subsets
are the fold's regions.

The sharper part, from the numbers: **as an interleaving pattern the fold is legitimate
and sub-optimal.** Its split is 496/32/496. A uniform three-way split (`i mod 3`) puts two
random cells in different regions **66.7%** of the time; the fold's does it **53%**. The
thin seam that makes the partition *the fold* is exactly what makes it a worse interleaver
than the trivial one.

## THE CLOSING AUDIT — every bar, called vs landed

| bar | called | landed | verdict |
| --- | --- | --- | --- |
| B1 no regression on singles | YES | 3000/3000, 0 wrong | **MET** |
| B2 direct share ≥ 0.50 | YES at mid (0.53) | 0.539 | **MET** |
| B3 overhead ≤ 4.7% | YES at top of range | 4.69% (4 residues) | **MET by 0.01** |
| B4 miscorrect ≤ v1 | coin flip | 0 with confirm; 327 without | **MET only with confirm** |
| B5 a name | interleaved AN code | interleaved AN, two-level with confirm | **MET** |

Five bars, five met, two of them only after a miss was measured and answered. The fold
was the mechanism for the first time, and it turned out to be a known kind of thing, done
in a slightly worse shape than the obvious way. The fold itself remains unplaced — this
round placed what one *does* with it, not what it *is*.
