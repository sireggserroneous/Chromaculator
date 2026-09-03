# eggSo v1 — the anti-transpose is the code

Not part of the site. The fifteenth codec experiment and the second in the fold-native
lineage begun by [`eggSo-v0/`](../eggSo-v0/), kept in its own folder so it does not entangle
with `chronochromatic.org`, which claims none of this.

Built 2026-09-02 against [PREDICTIONS.md](PREDICTIONS.md), filed before a line of the
codec was written. Three arms, each built and measured — *"if we have 3 ways, we need to
test all 3 ways"* — and a standings table at the end so the house can keep or discard
each on the record. Guesses and misses are kept side by side in the predictions file.

## The verdict, first

**Sixteen bars across three arms, sixteen met. The house keeps v1(a).**

| arm | what it is | the bar that mattered | landed | cost |
| --- | --- | --- | --- | --- |
| **v1(a)** — one extra residue, `R_σ = (V − σV) mod p` | two syndromes per hemisphere; a table names any same-region pair | same-region pairs ≥ 95% corrected with no search, 0 wrong | **2000/2000, 0 wrong** (v0: 2/921) | 60 bits = **5.86%** |
| v1(b) — mirror code, `Outer := σ(Inner)` | 528 data cells, 496 copies; a mismatch names the pair, the residue that moved names the side | 12-cell **unflagged** in-region burst ≥ 99%, 0 wrong | **800/800** — the one row no other column has | **103%** per data bit, 50.7% of the artifact |
| v1(c) — σ as a self-inverse interleaver | store `cells∘σ`, check with v0 | identical to v0 within 2σ | **identical to the trial**, z = 0.00 everywhere | 4.69% |

And the sentence the round was for: **σ buys the fold's partition its second syndrome.**
v0 stored one residue per region and could name one error in each. Adding `V − σV` —
which is zero on the Fold, since σ fixes it, and pairs every Inner cell with an Outer cell
of incomparable weight — gives each hemisphere a second, independent 11-bit equation.
Two equations, two unknowns: the pair is named by lookup, no search, and the confirming
residue leaves nothing to alias. That is a two-syndrome arithmetic code, the residue
analogue of a double-error-correcting BCH code with the mirror weights playing the second
locator power. It has a name; the fold supplied the second row of `H`.

The fold itself remains unplaced. This round placed what its symmetry *does*. (Placed in [eggSo-v4](../eggSo-v4/): the Fold is the Julia set of a degree-2 map and Inner and Outer are its Fatou basins.)

## What failed, first

Nothing on the live arm. On the record anyway:

- **A number in the plan's ground was mis-derived.** "7.25% collide" was read as "92.7%
  of pairs resolve by lookup alone." 7.25% is the *excess* rate, `1 − distinct/total`.
  The share of table entries that *have* a twin is **14.04%**, so bare lookup resolves
  **86.0%**. With the alphabet filter and `q` the difference vanished (2000/2000); without
  `q` it did not — the negative control landed **90.9%** where ~98% was called. The twins
  share a cell with the true pair, so the alphabet filter kills fewer than the ¼ assumed.
- **The plan's Fold–Fold range (60–95%) missed on the high side.** With `q` the 32-cell
  search is 500/500. The prediction file said at filing that the arithmetic pointed above
  the range; it did.
- **v1(b) met a 1/p case.** On the first run one of 800 unflagged bursts was refused: its
  twelve flips summed to zero mod p, no region residue moved, and the decoder returned
  "confirm only" before looking at the mismatches. It now enumerates under `q` when the
  residues are silent but the mismatches are not: 800/800. Both numbers are in the
  predictions file.
- **Not filed and should have been:** codegg-v1 *miscorrects* 37 of 400 unflagged
  in-region bursts on `spec.md` — its double-error search finds a spurious pair and
  applies it. The plan's table had that cell as "0" and meant "detected." v0 and every v1
  arm refuse all 400.

## Why this round exists

v0's sharpest failure was the hook. Two errors in one region hit one residue with two
unknowns; with one prime the in-region search aliased and miscorrected 22%, with the
confirming residue it refused instead — **2 of 921 corrected, 919 detected**. codegg-v1
corrects 87% of them by search. On `2 cells anywhere` v0 lost to its own control.

The lineage audit had found **anti-transpose** and **involution** at zero mentions across
thirteen versions. v0's partition is exactly what σ leaves invariant; v0 used the
invariant and never the map. This round uses the map, three ways, and asks of each what
it buys, what it costs, and what it is called.

## The construction

Row-major layout as v0 and codegg-v1, N = 32, L = 1024, bit alphabet. The anti-transpose
`σ(r,c) = (n−1−c, n−1−r)` is drawn inline at `index.html:398` and has no function on the
site; `partnerOf` in [`eggso1.js`](eggso1.js) is that line as a function, and the suite
asserts it against the line itself over 22,139 cells, n = 2..40, plus `σ∘σ = id`, fixed
set = Fold, and `regionOf(σj) = 2 − regionOf(j)`.

```
σ(j) = L − 1 − jᵀ,     jᵀ = (j mod N)·N + ⌊j/N⌋           (row-major index)
```

**v1(a).** An error `d` at `j` moves `R_σ` by `d·(w[j] − w[σj])`, zero on the Fold. For an
Inner pair `{a,b}`: `X = Δ_I = d₁w[a] + d₂w[b]`, `Y = Δ_I − Δ_σ = d₁w[σa] + d₂w[σb]`. An
Outer pair casts the same `(X,Y)` as `(Δ_O − Δ_σ, Δ_O)`, so one table of 491,040 entries
over Inner serves both hemispheres. Decoder: v0's singles per hurt region, with `R_σ` and
`q` as confirms on the whole plan; a region with no consistent single looks up `(X,Y)`;
Fold pairs (which `R_σ` cannot see) fall to v0's 32-cell search with confirm inside the
loop. Five residues: `I, F, O, R_σ, q`. The **`replaces-q`** configuration drops `q` and
is a negative control: the Fold is in `R_σ`'s kernel, so the Fold filled must miscorrect
again — and does, **18.0%**, where 15–25% was called.

**v1(b).** Data = Inner + Fold = 528 cells = 66 bytes per square; every Outer cell is its
partner's copy; v0's four residues unchanged. Mismatches `cells[j] ≠ cells[σj]` name the
damaged pairs. If only `Δ_I` moved, every mismatch is Inner's fault — no enumeration, any
count, which is how a 12-cell burst is read straight off the partner. If both moved, the
`2^|M|` side assignments (cap 16) are tried against both residues and `q`. Flagged cells
are copied from the unflagged partner. A row meets its mirror column only at the Fold
cell, so a row burst never hits both members of a pair.

**v1(c).** Store `cells∘σ`; apply v0's checks to the logical square. By the automorphism
fact this is v0 with weight table `w∘σ` and the Inner/Outer labels swapped.

| | |
| --- | --- |
| **borrowed** | regions, per-region residues, syndrome tables, the confirming residue — eggSo-v0's, `require`d from `../eggSo-v0/eggso.js`. Moduli, weights, the row-major layout and the whole-square residue — codegg-v1's, `require`d from `../codegg-v1/codegg.js`. Nothing copied |
| **the site's** | the anti-transpose. `partnerOf` is `index.html:398`'s `pr = n - 1 - c, pc = n - 1 - r`, and the suite reads that line out of `index.html` and asserts against it |
| **the alphabet** | bits only, as v0. Pushed spellings are eggSo-v2's question |

## Results, all measured

### Head to head (`tools/versus.js ../spec.md --trials 400`)

Both controls and all three arms on the same file. Each cell is
`ok / detected / MISCORRECTED / direct`; `direct` = corrected by a syndrome naming its own
cell with no search (for v1(a) that includes the pair table; for v1(b) the side named by
the one residue that moved).

| channel | codegg-v1 | eggSo-v0 | **v1(a)** | v1(b) | v1(c) |
| --- | --- | --- | --- | --- | --- |
| 1 cell flipped | 400 · 400 direct | 400 · 400 direct | 400 · 400 direct | 400 · 400 direct | 400 · 400 direct |
| 2 cells, anywhere | 346 · 54 det · 0 direct | 212 · 188 det · 211 direct | **400 · 399 direct** | 400 · 205 direct | 212 · 211 direct |
| 2 cells, different regions | 343 · 57 det | 400 · 400 direct | 400 · 400 direct | 400 · 33 direct | 400 · 400 direct |
| 2 cells, same region | 338 · 62 det | 120 · 280 det | **400 · 254 direct** | 400 · 254 direct | 120 · 280 det |
| 3 cells, one per region | 0 · 364 det · **36 WRONG** | 400 · 400 direct | 400 · 400 direct | 400 · 2 direct | 400 · 400 direct |
| 12-cell row burst, flagged | 400 | 400 | 400 | 400 | 400 |
| 12-cell row burst, **unflagged**, in-region | 0 · 363 det · **37 WRONG** | 0 · 400 det | 0 · 400 det | **400 · 400 direct** | 0 · 400 det |
| the Fold filled, 32 unflagged | 352 det · **48 WRONG** | 400 det | 400 det | 400 det | 400 det |
| push: checks still hold | **200/200** | 0/200 | 0/200 | 0/200 | 0/200 |
| overhead | 2.35% | 4.70% | 5.88% | 103.4% (50.7% share) | 4.70% |

Read down the v1(a) column: every channel v0 had is kept, the one it lost is won, and
nothing is miscorrected. Read across the unflagged-burst row: one column corrects it, at
eighteen times the price. Read v1(c) beside v0: the same numbers to the trial.

### What the suite pins down (`tools/eggso1.test.js`)

| claim | result |
| --- | --- |
| `partnerOf` = `index.html:398`, involution, fixed set = Fold, `regionOf(σj) = 2 − regionOf(j)` | 22,139 cells, n = 2..40, 0 mismatches |
| pair table: distinct joint syndromes | **455,428 of 491,040** exactly as planning computed; excess 7.25%; twin share 14.04%; region residue alone 2,052 distinct |
| pair syndromes equal to a single's · peel = table | **0** · identical on 400 random syndromes |
| v1(a) singles, anywhere / on the Fold, both cfgs | 3000/3000 · 3000/3000, all direct, 0 wrong |
| v1(a) same-region pairs, 2000 | **2000/2000, 0 wrong** (1366 by table, 634 Fold pairs by search) · replaces-q 1817, 183 ambiguous, 0 wrong |
| v1(a) Fold–Fold aimed 500 · cross-region 1000 · 3 one-per-region 1000 | 500/500 · 1000/1000 all direct · 1000/1000 |
| v1(a) the Fold filled, 300: with `q` / replaces-q | 300 detected, 0 wrong / **54 MISCORRECTED = 18.0%** — the kernel, on purpose |
| v1(a) flagged 12-cell burst, both cfgs | 800/800 |
| push: `V` / three parts / `R_σ` / arm b / arm c | 200/200 / 0/200 / 0/200 / 0/200 / 0/200 |
| v1(c) `encode∘encode = id` · every channel vs v0 | 500/500 · identical, z = 0.00; inner/outer fires 264/254 ↔ 254/264, Fold 149 both |
| v1(b) singles 3000 · doubles 2000 · both members of one pair 500 · Fold–Fold 500 | 3000 · 2000 · 500 · 500, 0 wrong |
| v1(b) **unflagged in-region burst** 800 · flagged 800 · whole Inner erased 100 | **800/800** (799 on the first run, see above) · 800/800 · 100/100 by copy |
| v1(b) the Fold filled | 300 detected, 0 wrong |
| cost per square | v1(a) 60 bits 5.86% · v1(b) 103.03% per data bit, 50.75% of the artifact, 48.44% of the square · v1(c) 4.69% |

### The honest section

- **`R_σ` cannot see the Fold.** `w[j] − w[σj] = 0` on all 32 Fold cells, so a Fold error
  rides on `F mod p` alone and Fold pairs are still a search. With `q` that search is
  500/500; without `q` the Fold filled miscorrects 18% — which is exactly why the control
  exists, and exactly why v1(a) is five residues, not four.
- **Push breaks every arm.** `pushLeft` conserves `V`, and `R_σ = V − σV` is not `V`:
  0/200. The mirror property breaks too. codegg-v1's whole-square residue remains the only
  check here that survives the site's canonicalising move.
- **v1(b)'s "direct" is honest and therefore small on cross-region pairs.** When both
  hemispheres move, the side is settled by a 4-way enumeration, not named: 33 direct of
  400. Its capability is the burst, and the burst is 400 direct.
- **v1(c) is not an interleaver in any useful sense.** codegg-v4's `bitrev` spreads a 4 KB
  wound across 11,633 squares; σ never leaves its square, and by measurement (all 672 row
  bursts) it does not even move a burst across regions — it swaps the labels. The
  identical tallies are structural: v0's only chance channel is the Fold pair search, and
  σ fixes the Fold.
- **Real text still beats random for v0**, and it no longer matters for v1(a): 400/400 on
  `spec.md` and 2000/2000 on random squares.
- **The names.** v1(a): a two-syndrome arithmetic (AN) code, `H = [w ; w − w∘σ]` over `Z_p`
  per hemisphere — double-error-correcting by table, the residue analogue of a two-locator
  BCH code. v1(b): a `[2,1]` repetition code concatenated with the AN check, decoded by
  error-to-erasure conversion, with a placement that keeps row bursts off both copies.
  v1(c): an intra-block permutation that is an automorphism of the check partition — a
  no-op interleaver.

## Running it

```
node eggSo-v1/tools/eggso1.test.js                          # ten claims, three arms, both v1(a) configs
node eggSo-v1/tools/versus.js spec.md --trials 400 --json    # five columns, nine channels
node eggSo-v1/tools/corrupt.js stalk.js --arm a --model pair --hits 40
node eggSo-v1/tools/corrupt.js stalk.js --arm b --model burst --hits 40
node eggSo-v1/tools/corrupt.js stalk.js --arm a --bare --model fold --hits 40   # the kernel hole: exits 3
```

`corrupt.js` exits 0 (exact), 2 (detected, not exact) or 3 (silently wrong *or*
miscorrected). Across `stalk.js` and `wubbadub.html`, every arm and model exits 0 or 2;
only the `--bare` negative control exits 3, as it must.

## Files

| | |
| --- | --- |
| `eggso1.js` | `partnerOf`, `partnerRC`, `sigmaTable`; arm a (`makeCodeA`, `checksForA`, `repairA`, `pairTable`, `peelPairs`); arm b (`makeCodeB`, `toCells528`, `toBytes528`, `repairB`, `sizesB`); arm c (`makeCodeC`, `permuteC`, `repairC`). Requires `../eggSo-v0/eggso.js` and `../codegg-v1/codegg.js` |
| `PREDICTIONS.md` | filed before building; measured after; misses kept |
| `tools/eggso1.test.js` | the ten claims |
| `tools/versus.js` | both controls against all three arms |
| `tools/corrupt.js` | real files through one arm, four damage models |
| `measured-*.json` | the suite's own record of the numbers above; `measured-versus.json` is the head-to-head |

## What this is and is not

It is the first time the fold's own symmetry did the work, and the answer is that it
supplies exactly one more equation per hemisphere — enough to name a pair, blind to the
seam it turns on. It is not a claim that the fold is anything other than what the site's
README says it is: unplaced, held loosely — now with the partition *and* the map on the
record, both with names.
