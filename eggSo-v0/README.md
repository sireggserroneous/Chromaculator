# eggSo v0 — the partition is the code

Not part of the site. The fourteenth codec experiment, sibling to `codec-v1/` and the
`codegg-v*/` series, kept in its own folder so it does not entangle with
`chronochromatic.org`, which claims none of this.

Built 2026-09-02 against [PREDICTIONS.md](PREDICTIONS.md), filed before a line of the
codec was written. Guesses and misses are kept side by side there.

## Amendment — 2026-09-02, after eggSo-v2

**This round misread its own worst number.** Everything below is the round as it was
built and measured, and it stays that way. But the headline failure — *"eggSo cannot
correct a same-region pair: 2 of 921"* — is not a property of the partition. It is a
property of **where this code puts its confirming residue.**

v0 assembles a whole repair plan and only then asks `q`. Its in-region search refuses at
the *second* candidate, before `q` is ever consulted, and a pair whose syndrome also reads
as a valid single is spent on that single and refused by the final confirm. codegg-v1 asks
`q` **inside** the search (`codegg.js:204-206`, `223-231`), so a second candidate that `q`
rejects costs it nothing. Same partition, same four residues, same 4.69%:

| same-region pairs | corrected | detected | miscorrected |
| --- | --- | --- | --- |
| `q` after the plan — v0 as shipped, 1000 random squares | **281** | 719 | 0 |
| `q` per candidate — codegg-v1's rule, same squares | **972** | 28 | 0 |
| on `spec.md`, 400 trials: shipped / amended | **130 / 391** | 270 / 9 | 0 / 0 |

It costs nothing on any channel this round won: singles, cross-region pairs and
three-one-per-region stay 1000/1000 with 0 miscorrected.

The amendment is an **option, default off**, so every number in this file and in
`PREDICTIONS.md` stays reproducible from the code that produced it:

```
node eggSo-v0/tools/eggso.test.js                            # block 10 measures it
node eggSo-v0/tools/versus.js spec.md --per-candidate        # the column, corrected
```

What this does to the lineage's record: eggSo-v1 was planned against a gap of **2 vs 400**
and the honest gap is **391 vs 400**. v1(a) still buys something v0 cannot have — the pair
is *named* by a table lookup, 254 direct of 400, with no search at all — but it buys nine
corrections, not 398. The 130/400 in the head-to-head table below is the shipped form's
number and is left standing; read it with this section.

## The verdict, first

**Five bars, five met — two of them only after a miss was measured and answered.**

| bar | needed | landed |
| --- | --- | --- |
| B1 no regression on singles | 3000/3000 | **3000/3000**, all direct, 0 wrong |
| B2 the capability: two-cell pairs corrected with **no search** | ≥ 50% of random pairs | **53.9%** |
| B3 cost | ≤ 4.7% | **4.69%** |
| B4 miscorrection ≤ codegg-v1's | 0 | **0** with the confirming residue; **327/2000 without** |
| B5 a literature name | one | **interleaved AN code** |

And the sentence the round was for: **used as a coding mechanism, the fold is a
legitimate interleaver and a sub-optimal one.** Its split is 496 / 32 / 496. A uniform
three-way split puts two random cells in different regions 66.7% of the time; the fold's
does it 53%. The thin seam that makes the partition *the fold* is exactly what makes it a
worse interleaver than the trivial `i mod 3`.

The fold itself remains unplaced. This round placed what one *does* with it.

## What failed, first

Three predictions missed, and one of them cost a design change.

- **One prime per region cannot support anything past a single error.** Filed as
  "3 residues, ~3.5%". `p ≈ 2L` gives each region's syndrome about 11 bits. The in-region
  two-error search saturates that space: on random squares **216 of 952** same-region
  pairs aliased to a single and were "repaired" wrong. The flagged 12-cell burst that
  codegg-v1 corrects 800/800 landed **292/800** for the same reason — 2^12 assignments
  against 11 bits leaves two standing. A fourth residue over the whole square, in v1's
  second prime, refuses every alias: **0 miscorrected**, **800/800**. It costs 1.17 points
  and is on by default. The 3-residue form is kept behind `--bare` so the floor can be
  reproduced.
- **The overall two-error rate was called at ~94% and landed at 54%.** The 94% assumed
  codegg-v1's search would work inside a region. It does not, and with confirm the search
  refuses what it cannot confirm. On `2 cells anywhere`, **codegg-v1 corrects 344/400 and
  eggSo 191/400**. eggSo's win is the *direct* column (191 vs 0); its loss is the total.
  The gain was called and the loss was not.
- **"The Fold filled" was called *detected only*.** Bare, it miscorrects 63/300. So does
  codegg-v1 (60/400) — thirty-two flips make a syndrome that lands on a valid single about
  a fifth of the time. With confirm: 300 detected, 0 wrong. Held for the shipped form,
  missed for the filed one.

## Why this round exists

The lineage audit of the thirteen versions before this one found that every one reached
prior art by a road *other* than the fold — product codes, residue arithmetic,
interleavers, Reed–Solomon, PAQ — and that across all thirteen the words **Outer** and
**anti-transpose** appear zero times as regions or maps. The one construction the site
cannot place was never the mechanism. This round made it the mechanism, alone, and asked
what it buys and what it is called.

## The construction

codegg-v1 stores one residue of the whole square's value, `V mod p`. A single-cell error
of size `d` at cell `i` moves it by `d·2^(L−1−i) mod p`, which names the cell. Two errors
give one syndrome with two unknowns and v1 falls back to search.

eggSo stores the residue of **each region**:

```
I mod p,   F mod p,   O mod p          where  I + F + O = V     (stalk.js:118, spec.md)
```

and runs v1's mechanism inside each. Two errors in different regions are two single
errors, each named by its own residue, no search. A fourth residue, `V mod q`, confirms
any proposed repair before a cell is touched. Nothing else is new.

| | |
| --- | --- |
| **borrowed** | the residue, the weight table, the modulus choice and its injectivity proof — codegg-v1's, `require`d from `../codegg-v1/codegg.js`, not copied. v1 is itself an AN / residue arithmetic code. The row-major layout is v1's `toCells`, so both codecs see identical squares |
| **the site's** | which cells belong to which region. `regionOf()` is the one comparison `stalk.js:118` makes, and the suite asserts it against `stalk.js`'s own `regions()` over 22,139 cells so the two cannot drift |
| **the alphabet** | bits only. A stored bit is damaged by `d = ±1`, so one prime injective over `±2^k` separates every (cell, direction). Pushed spellings would need v1's second prime per region — out of v0's scope |

## Results, all measured

### Head to head (`tools/versus.js ../spec.md --trials 400`)

Same file, same squares, same damage, cell for cell. `direct` = corrected by a syndrome
naming its own cell, with no search — the column the round is about.

| channel | codegg-v1 | eggSo v0 |
| --- | --- | --- |
| 1 cell flipped | 400 ok · **400 direct** | 400 ok · **400 direct** |
| 2 cells, anywhere | **344 ok** · 56 det · 0 direct | 191 ok · 209 det · **191 direct** |
| 2 cells, different regions | 349 ok · 0 direct | **400 ok · 400 direct** |
| 2 cells, same region | 353 ok | 130 ok · 270 det · 0 wrong |
| 3 cells, one per region | **0 ok** · 356 det · 44 wrong | **400 ok · 400 direct** |
| 12-cell row burst, flagged | 400 ok | 400 ok |
| the Fold filled, 32 unflagged | 340 det · **60 wrong** | 400 det · **0 wrong** |
| overhead | 2.35% | 4.70% |

Read the table as a trade, because it is one. v1 corrects more two-cell errors in total.
eggSo corrects more *without searching*, corrects every three-cell error v1 cannot touch,
and never miscorrected on any channel here where v1 did on two.

### What the suite pins down (`tools/eggso.test.js`)

| claim | result |
| --- | --- |
| `regionOf` is `stalk.js regions()` | 22,139 cells, n = 2..40, cell for cell |
| `I + F + O ≡ V (mod 2053)` | 500/500 |
| round-trip | exact, 7 shapes, both configs |
| single error, anywhere / on the Fold | 3000/3000 · 3000/3000, all direct, 0 wrong |
| cross-region pairs | 1079/1079 corrected, 1079 direct |
| same-region pairs, bare / confirm | 216 wrong / **0 wrong** |
| flagged 12-cell burst, bare / confirm | 292/800 / **800/800** |
| burst straddling the fold line | 400/400 |
| the Fold filled, bare / confirm | 63 wrong / **0 wrong**, 300 detected |
| push invariance | v1's `V` holds 200/200; eggSo's three parts hold **0/200** — the predicted loss |
| overhead, bare / confirm / v1 | 3.52% / **4.69%** / 2.34% |

### The honest section

- **Push breaks it.** `pushLeft` conserves `V` and moves colour across region boundaries,
  so `I`, `F`, `O` each change. v1's whole-square residue survives respelling; eggSo's
  three do not. Predicted, measured 0/200, and a real cost: the site's canonicalising
  move is one this code cannot check through.
- **The Fold is 3% of the square.** It is a region with its own residue and only 32
  cells, so its check is almost never the one that fires — and when a burst fills it,
  the residue has 32 unknowns and one equation. Confirm makes that a refusal rather than
  a miscorrection; it does not make it a correction.
- **Real text beats random.** On `spec.md` the same-region search corrects 130/400; on
  random squares, 2/921. Biased bits leave fewer alphabet-valid candidates, so the search
  is sharper. The suite's random-square numbers are the floor; the file's are what a user
  sees.
- **The name.** Interleaved AN code; with confirm, a two-level residue code — component
  checks and an overall check, the product-code shape the series met in codec-v1. The
  interleaving pattern is the fold. The code is not.

## Running it

```
node eggSo-v0/tools/eggso.test.js                        # the nine claims, both configs
node eggSo-v0/tools/versus.js spec.md --trials 400       # head to head with codegg-v1
node eggSo-v0/tools/versus.js spec.md --bare             # the 3-residue form, and its floor
node eggSo-v0/tools/corrupt.js stalk.js --model pair --hits 60
node eggSo-v0/tools/corrupt.js stalk.js --model fold --hits 20
```

`measured-*.json` beside this file is what the suite wrote on the run the tables quote.

## Files

| | |
| --- | --- |
| `eggso.js` | the codec: `regionOf`, `makeCode`, `regionResidues`, `encode`, `decode`, `repairSquare`, `verify`, `sizes`. Requires `../codegg-v1/codegg.js` for the arithmetic it borrows |
| `PREDICTIONS.md` | filed before building; measured after; misses kept |
| `tools/eggso.test.js` | the nine claims, each run bare and with confirm |
| `tools/versus.js` | the head-to-head table |
| `tools/corrupt.js` | real files through encode → damage → decode, three damage models |
| `measured-*.json` | the suite's own record of the numbers above |

## What this is and is not

It is the first time in fourteen experiments that the fold's own partition did the work,
and the answer is that the work it does has a name and a better-known shape. It is not a
claim that the fold is anything other than what the site's README says it is: unplaced,
held loosely — now with one more attempt on the record, and this one from the inside.
