# eggSo v1 predictions — filed 2026-09-02, BEFORE a line of the codec was written

The series convention: every number below is a guess, written down first, and the
measured value is filled in beside it afterwards or not at all. Misses stay. A
prediction that is quietly edited to match its result is worth less than no prediction.

This round has **three arms**, each built and measured — *"if we have 3 ways, we need to
test all 3 ways"* — with its own bars, its own stages and its own verdict, and one
shared closing audit. Bare `v1` below means eggSo-v1; the control is always written out
as `codegg-v1`.

## What this is, in one sentence

The fold's **defining symmetry** — the anti-transpose `σ(r,c) = (n−1−c, n−1−r)`, which
fixes the Fold, swaps Inner with Outer and undoes itself — used as a coding mechanism for
the first time, three different ways.

## Why this round exists

eggSo-v0 made the fold's *partition* the code and got a name for it: an interleaved AN
code, legitimate and sub-optimal. Its sharpest failure is the hook here. **Same-region
pairs**: two errors in one region hit one residue with two unknowns; with one prime the
in-region search aliased and miscorrected 22%, with the confirming residue it refused
instead — **2 of 921 corrected, 919 detected**. v0 cannot correct a same-region pair.
codegg-v1 corrects 87% of them by search; on `2 cells anywhere` (spec.md, 400 trials)
codegg-v1 lands 344, v0 191.

The lineage audit found the words **anti-transpose** and **involution** at zero mentions
across thirteen `codec-*`/`codegg-*` versions. The partition of v0 is exactly what σ
leaves invariant; v0 used the invariant and never the map. This round uses the map,
three ways, and asks of each: what does it buy, what does it cost, what is it called.
A clean name for any arm is as good a result as a capability.

## The construction, stated before building

Row-major layout as v0 and codegg-v1 (`G.toCells`: cell `j = r·N + c`, weight
`w[j] = 2^(L−1−j)`), N = 32, L = 1024, bit alphabet. In that index

```
σ(j) = L − 1 − jᵀ,      jᵀ = (j mod N)·N + ⌊j/N⌋
```

is the anti-transpose. The site draws it inline (`index.html:388`, `const pr = n - 1 - c,
pc = n - 1 - r;  // partner across the fold`) and has no function for it; v1 defines
`partnerOf` once, in `eggso1.js`, and asserts it against that line.

Everything `require`s `../eggSo-v0/eggso.js` (regions, residues, tables, the confirming
residue) and `../codegg-v1/codegg.js` (moduli, weights, layout). Nothing is copied.

### v1(a) — one extra residue, `R_σ = (V − σV) mod p`

`σV = Σ cells[k]·w[σk]`. An error `d` at `j` moves `R_σ` by `d·(w[j] − w[σj])` — zero on
the Fold, since σ fixes it. For an Inner pair `{a,b}` with signs `d₁,d₂`:

```
X = Δ_I        = d₁·w[a]  + d₂·w[b]
Y = Δ_I − Δ_σ  = d₁·w[σa] + d₂·w[σb]
```

An Outer pair casts the same `(X, Y)` as `(Δ_O − Δ_σ, Δ_O)`, so **one pair table over
Inner serves both hemispheres**: 491,040 entries `(X·p + Y)`, sorted, binary-searched.

Decoder: (1) `Δ_I, Δ_F, Δ_O, Δ_σ mod p`, `Δ_q mod q`. (2) Per hurt region, v0's
alphabet-valid singles; the whole plan must also satisfy `Σ d·(w[i] − w[σi]) ≡ Δ_σ` and
`Δ_q` — `R_σ` is an 11-bit confirm on every non-Fold single. (3) A region with no
consistent single: look up `(X, Y)` in the pair table, filter by alphabet, then `Δ_q`;
unique → apply. (4) Fold pairs: no table (`Y ≡ X`); v0's search over 32 cells with
confirm inside the loop. Two regions each needing a pair is detected, not attempted.

Configurations: **`v1a`** (5 residues: I, F, O, σ, q) and **`v1a-replaces-q`** (`R_σ`
in place of `q`, 4 residues) as a *negative control* — the Fold is in `R_σ`'s kernel, so
Fold-filled must miscorrect again at the bare rate.

### v1(b) — mirror code, `Outer := σ(Inner)`

Data = Inner + Fold, **528 cells = exactly 66 bytes** per square. `cells[σj] = cells[j]`
for every Inner `j`. v0's four residues kept unchanged — the Fold has no mirror and needs
`F mod p`, which v0 already has. Decoder: mismatches `M = {j ∈ Inner : cells[j] ≠
cells[σj]}` name the damaged pairs; **the region residue that moved names the side**
(only `Δ_I` moved → every mismatch is Inner's fault). When both moved, enumerate the
`2^|M|` side assignments (cap 16) against `Δ_I` and `Δ_O`, then `Δ_q`. Flagged cells are
**copied from the unflagged partner** — no enumeration, any count; doubly-flagged pairs
and flagged Fold cells fall to v0's residue enumeration. A Fold error is v0's Fold path.
The plan lists "both errors in one pair" as a failure of the mismatch reading; the
decoder as designed here catches that case by the residue path (Inner single at `j` whose
partner shows the same `d`), and predicts it corrected — recorded here so the bar cannot
be met by an unfiled addition.

Geometry to state: row `r` meets its mirror (column `31−r`) only at the Fold cell
`(r, 31−r)`, so a row or column burst never hits both members of a pair.

### v1(c) — σ as a self-inverse interleaver

Store `cells∘σ`; apply v0's four checks to the *logical* square. By the automorphism
fact this is v0 with weight table `w∘σ` and the Inner/Outer labels swapped. `encode∘encode
= id` by construction. ~30 lines, built second, to falsify fast.

## Measured during planning — ground, not predictions

These were computed before any arm was built. A number already known cannot honestly be
filed as a guess, so they sit here, and what remains to predict is stated under each arm.
`makeCode` recomputes each and the suite asserts the match.

| arm | quantity | computed |
| --- | --- | --- |
| σ | `σ(j) = L − 1 − jᵀ` vs `index.html:388`'s `(n−1−c, n−1−r)`, N = 32 | **0 mismatches** over 1024 cells; `σ∘σ = id`, 0 failures; exactly **32 fixed points**, the Fold; every Inner cell maps into Outer and back, 0 violations |
| σ | `w[j] / w[σj] = 2^(σj − j)` over Inner | ranges **2^33 to 2^1023** — a cell and its partner never have comparable weight |
| σ | `w[j] − w[σj]` on the Fold | **0 for all 32 cells** — a Fold error is invisible to any check built on `V − σV` |
| **v1(a)** | same-region pairs in Inner, `d = ±1`: distinct joint syndromes `(S₁, S₁−S₂)` | **455,428 of 491,040 → 7.25% collide**; 92.7% resolve to a unique pair by lookup alone |
| v1(a) | contrast — region residue alone (v0's situation) | 2,052 distinct of 491,040: **99.6% collide** — why v0 could not search |
| v1(a) | pair joint syndromes equal to any *single*-error joint syndrome | **0** — the lookup can never repair a pair as a single |
| **v1(c)** | 12-cell row bursts (all 672): regions touched before → after σ | **420 / 42 / 210 → 420 / 42 / 210, identical.** σ does not spread a row burst across regions |

Terminology trap, on the record: the *site's* prose calls Inner "the low place values" in
Hankel order; in the row-major index used here Inner is the *high-weight* half. Only
`w[j]` enters the arithmetic.

## THE BARS

### v1(a)

| bar | needed to count as met |
| --- | --- |
| **A1** singles | 3000/3000 anywhere, 3000/3000 on the Fold, all direct, 0 wrong, both configurations |
| **A2** the table | pair-table collision rate recomputed at `makeCode` **= 7.25%** at N = 32 (455,428 distinct of 491,040); the 992-probe peel returns the same candidate set as the table |
| **A3** the capability | same-region pairs, random squares, 2000 trials: **≥ 95% corrected direct, 0 miscorrected**, remainder detected as ambiguous |
| **A4** the kernel | `v1a-replaces-q` miscorrects the Fold filled at the bare rate, **15–25%**. If it lands at 0 the kernel analysis is wrong and this file says so |
| **A5** cost | 5 residues, **60 bits = 5.86%** |
| **A6** the name | a two-row linear check over `Z_p` per hemisphere, `H = [w ; w − w∘σ]` — a two-syndrome (double-error-correcting) arithmetic code in which the mirror weights play the role BCH gives to the second locator power |

**MET** if A3 holds with 0 miscorrected *and* A4 shows the Fold hole. **MISSED** if the
collision rate lands > 25% or any miscorrection with confirm.

### v1(b)

| bar | needed to count as met |
| --- | --- |
| **B1** layout | `toBytes528` round-trips 66 bytes per square exactly, 7 shapes |
| **B2** singles | 3000/3000, 0 wrong |
| **B3** doubles anywhere | ≥ 99% corrected, 0 wrong |
| **B4** the one capability no other arm has | 12-cell **unflagged** in-region row burst **≥ 99%, 0 wrong** (v0, codegg-v1: detected only) |
| **B5** erasures by copy | flagged 100% by partner copy; the *whole Inner region* erased and recovered, 100/100 |
| **B6** cost, both conventions | **103%** check-and-mirror bits per data bit (v0's convention); **48%** redundant share of the square — both printed, always |
| **B7** the name | a `[2,1]` repetition code concatenated with the AN check, decoded by error-to-erasure conversion (mismatch → two-candidate erasure → residue decides); the placement is an interleaver that keeps bursts off both copies |

**MET** if B4 ≥ 95% with 0 wrong everywhere; **MISSED** below 90%.

### v1(c)

| bar | needed to count as met |
| --- | --- |
| **C1** involution | `encode∘encode = id`, exact, every square |
| **C2** re-derivation | every channel within 2σ of v0's tally; singles, cross-region pairs and 3-per-region **exactly** identical; only inner-fires / outer-fires swap |
| **C3** the name | an intra-block permutation that is an automorphism of the check partition — a no-op interleaver |

**MET** if identical to v0 within 2σ on every channel; **MISSED** if any channel differs
significantly — that would falsify the automorphism claim, and would be worth knowing.

## Calibration, stated before the numbers

I expect **v1(a)** to be the arm the house keeps: twelve extra bits should turn v0's
~209 detected-only pairs into corrections with no search and no miscorrection, and the
kernel control should show the hole exactly where the algebra puts it. **v1(b)** should
meet every bar and be too expensive to keep — one row of capability at 18× v1(a)'s cost.
**v1(c)** should re-derive v0 to the cell; if it does not, the plan's automorphism claim is
wrong, which is the more interesting outcome. I am least sure of A4's *range*: the
mechanism is certain, the rate is a guess from v0's bare 63/300.

## Per-stage predictions

### S1 — `partnerOf` and the shared ground

| claim | predicted |
| --- | --- |
| `partnerOf` equals `index.html:388`'s inline formula for every cell, N = 2..40 | 0 mismatches — a miss is a bug |
| `σ∘σ = id`; fixed set = `regionOf == FOLD`; `regionOf(σj) = 2 − regionOf(j)` | all hold, every N |
| pair-table collision rate at `makeCode`, N = 32 | **exactly 7.25%** (455,428 distinct) |
| pair joint syndromes that equal a single's | **0** |
| 992-probe peel vs table lookup, 200 random pair syndromes | identical candidate sets |

### S2 — v1(a) singles and cost

| channel | predicted |
| --- | --- |
| single anywhere, 3000, both cfgs | 3000/3000 direct, 0 wrong |
| single on the Fold, 3000, both cfgs | 3000/3000 — carried by `F mod p` alone, `R_σ` blind to it |
| round-trip, 7 shapes, both cfgs | exact |
| overhead | `v1a` **60 bits, 5.86%**; `replaces-q` 48 bits, 4.69% |

### S3 — v1(a) pairs, the question

| channel | predicted |
| --- | --- |
| same-region pairs, random, 2000, `v1a` | **≥ 95% corrected direct, called 99%**, 0 wrong, rest ambiguous. Arithmetic at filing: 7.25% collide, ~¼ survive the alphabet, 1/q survive confirm — the 99% call is deliberately loose |
| same-region pairs, `replaces-q` | ~98% corrected, ~2% ambiguous, 0 wrong (collisions filtered by alphabet only) |
| Fold–Fold aimed, 500, `v1a` | **60–95%** as the plan calls it, credited to `q` not σ. Arithmetic at filing says the top of that range or above |
| Fold–Fold aimed, `replaces-q` | ~75%, rest ambiguous, 0 wrong |
| cross-region pairs | 100% direct, as v0 |
| pairs "repaired" as a single, either cfg | **0** |
| `2 cells anywhere` on spec.md, 400 | **≥ 396/400** vs v0's 191 and codegg-v1's 344 |

### S4 — v1(a) the Fold, the kernel, push

| channel | predicted |
| --- | --- |
| the Fold filled, 32 unflagged, `v1a` | 300 detected, **0 wrong** |
| the Fold filled, `replaces-q` | **15–25% MISCORRECTED** — the kernel hole, on purpose |
| 12-cell flagged row burst, `v1a` | 800/800 (σ and q both filter the ~2 readings per region) |
| 12-cell flagged row burst, `replaces-q` | ~800/800 — σ alone is the 1/p filter v0 bare lacked |
| push: three parts hold | **0/200**, as v0 |
| push: `R_σ` holds | **no** — push moves colour between a cell and a non-partner; `V − σV` is not conserved. Predicted ~0/200 |

### S5 — v1(c), falsify fast

| channel | predicted |
| --- | --- |
| `encode∘encode = id` | exact, 500/500 |
| singles / cross-region / 3-per-region vs v0, same seeds | **exactly identical** |
| same-region pairs, flagged burst, Fold filled vs v0 | within 2σ (binomial) |
| inner-fires / outer-fires on a row burst | **swapped**, Fold count unchanged |
| overhead | 4.69% |

### S6 — v1(b)

| channel | predicted |
| --- | --- |
| `toBytes528` round-trip, 7 shapes | exact, 66 bytes per square |
| singles, 3000 | 3000/3000, 0 wrong; all direct (one residue moved) |
| doubles anywhere, 2000 | **≥ 99%**, 0 wrong; the residual is Fold–Fold at 0.095% |
| 12-cell **unflagged** in-region row burst, 800 | **≥ 99%, 0 wrong** — only one residue moves, so every mismatch is that side's; direct |
| 12-cell flagged row burst, 800 | 800/800 by partner copy |
| whole Inner erased, 100 | **100/100** by copy |
| the Fold filled, 300 | detected, 0 wrong |
| push | 0/200 |
| overhead | **103%** (v0's convention) · **48%** (redundant share) — the 48% is 496 mirrored cells over 1024; if the share is taken over the whole artifact including the 48 check bits it is 50.7%, and this file says so now |

## The bar arithmetic, filed plainly

| bar | needs | call |
| --- | --- | --- |
| A1 | 3000/3000 | **YES** |
| A2 | 7.25% exactly | **YES** — the same code recomputes it |
| A3 | ≥ 95%, 0 wrong | **YES, ~99%** |
| A4 | 15–25% wrong in the control | **YES**, the rate is the uncertain part |
| A5 | 5.86% | **YES** by arithmetic |
| A6 | a name | **YES**: two-syndrome arithmetic code |
| B1–B3 | exact, 3000, ≥ 99% | **YES** |
| B4 | ≥ 99% unflagged in-region burst | **YES** — the geometry guarantees it |
| B5 | 100/100 | **YES** |
| B6 | both conventions printed | **YES**; 103% / 48% |
| B7 | a name | **YES**: repetition ⊕ AN, error-to-erasure |
| C1 | exact | **YES** |
| C2 | within 2σ everywhere | **YES** — and the miss would be the interesting result |
| C3 | a name | **YES**: no-op interleaver |

## Predicted standings (spec.md, 400 trials) — filed before building

| channel | codegg-v1 | eggSo-v0 | v1(a) | v1(b) | v1(c) |
| --- | --- | --- | --- | --- | --- |
| 1 flip | 400 | 400 | 400 | 400 | 400 |
| 2 anywhere | 344 | 191 | **≥ 396** | ≥ 398 | 191 ± n |
| 2 same-region | 353 | 130 | **≥ 396** | ≥ 398 | ~130 |
| 2 cross-region | 349 | 400 direct | 400 direct | 400 | 400 direct |
| 3 one-per-region | 0 (44 wrong) | 400 direct | 400 direct | 400 | 400 direct |
| 12 flagged burst | 400 | 400 | 400 | 400 | 400 |
| **12 unflagged in-region** | 0 | 0 | 0 | **≥ 396** | 0 |
| Fold filled | 340 det / 60 wrong | 400 det | 400 det | 400 det | 400 det |
| push | 200/200 | 0 | 0 | 0 | 0 |
| overhead | 2.34% | 4.69% | 5.86% | 103% (48% share) | 4.69% |

**Who the house is predicted to keep:** v1(a). v1(b) buys one row at 18× the cost.
v1(c) adds nothing over v0.

## Measured (filled as stages land — never before)

Filled 2026-09-02, after `tools/eggso1.test.js`, `tools/versus.js ../spec.md --trials 400
--json` and `tools/corrupt.js` on `stalk.js` and `wubbadub.html`. Every number here is from
those runs; the JSON beside this file (`measured-*.json`) is what the suite wrote.

### S1 — `partnerOf` and the shared ground: HELD, with one mis-derivation in the ground

| claim | called | landed |
| --- | --- | --- |
| `partnerOf` = `index.html`'s inline `pr = n - 1 - c, pc = n - 1 - r` | 0 mismatches | **0 over 22,139 cells, n = 2..40**. The line is at `index.html:398`, not 388 as the plan cited — inside its 388–400 range, cited exactly from here on |
| involution; fixed set = Fold; `regionOf(σj) = 2 − regionOf(j)` | hold | **hold at every n**; 819 fixed cells over n = 2..40, all Fold |
| weight ratio over Inner; `w − w∘σ` on the Fold | 2^33..2^1023; 0 on 32 cells | **2^33..2^1023; 0 on all 32** |
| distinct joint syndromes | 455,428 exactly | **455,428 exactly**; excess rate 35,612 / 491,040 = **7.25%** |
| region residue alone | 2,052 distinct | **2,052** |
| pair syndromes equal to a single's | 0 | **0** |
| peel vs table | identical | **identical on 400 random pair syndromes**, 47 of which had twins |

**The mis-derivation, kept:** the plan read "7.25% collide" as "92.7% resolve to a unique
pair by lookup alone." 7.25% is the *excess* rate, `1 − distinct/total`. The share of
entries that *have* a twin — the number the bare lookup actually faces — is **14.04%**
(68,936 of 491,040), so lookup alone resolves **86.0%**, not 92.7%. With the alphabet
filter and `q` on top the difference vanished (S3); without `q` it did not (S3, replaces-q).

### S2 — v1(a) singles and cost: HELD

| channel | called | landed |
| --- | --- | --- |
| single anywhere, 3000, both cfgs | 3000/3000 direct | **3000/3000, all direct, 0 wrong, both** |
| single on the Fold, 3000, both cfgs | 3000/3000 | **3000/3000, both** — `F mod p` alone carries it |
| round-trip | exact | **exact, 7 shapes, both cfgs** |
| overhead | 60 bits, 5.86% / 48 bits, 4.69% | **60 bits = 5.86%** / **4.69%** (`p = 2053`, `q = 2063`) |

### S3 — v1(a) pairs: HELD on the live arm, MISSED twice on the control

| channel | called | landed |
| --- | --- | --- |
| same-region pairs, random, 2000, `v1a` | ≥ 95%, called 99%, 0 wrong | **2000/2000 corrected, 0 wrong** — 1366 by table, 634 Fold pairs by search |
| same-region pairs, `replaces-q` | ~98%, 0 wrong | **1817/2000 = 90.9%**, 183 ambiguous, **0 wrong** — MISSED. Twins are 14%, not 7%, and they *share a cell* with the true pair, so the alphabet filter kills fewer than the ¼ assumed |
| Fold–Fold aimed, 500, `v1a` | 60–95% (plan) | **500/500** — the plan's range MISSED on the high side; the arithmetic note at filing ("top of the range or above") held |
| Fold–Fold aimed, `replaces-q` | ~75% | **407/500 = 81.4%**, 93 ambiguous, 0 wrong |
| cross-region pairs, 1000 | all direct | **1000/1000, 2000 cells direct**, both cfgs |
| pairs repaired as a single | 0 | **0**, either cfg, any channel |
| 2 anywhere, random, 2000 | — | `v1a` **2000/2000**; replaces-q 1965/2000, 0 wrong |
| 3 one-per-region, 1000 | direct | **1000/1000**, both cfgs |
| `2 cells anywhere` on spec.md, 400 | ≥ 396 | **400/400, 399 direct** (one Fold pair by search). codegg-v1 346, v0 212 on the same squares |

### S4 — v1(a) the Fold, the kernel, push: HELD

| channel | called | landed |
| --- | --- | --- |
| the Fold filled, `v1a` | 300 detected, 0 wrong | **0 ok, 300 detected, 0 wrong** |
| the Fold filled, `replaces-q` | 15–25% MISCORRECTED | **54/300 = 18.0% MISCORRECTED** — the kernel hole, where the algebra put it |
| flagged 12-cell burst, `v1a` | 800/800 | **800/800** |
| flagged 12-cell burst, `replaces-q` | ~800/800 | **800/800** — σ alone is the second filter v0 bare lacked |
| push: three parts | 0/200 | **0/200** |
| push: `R_σ` | ~0/200 | **0/200** |

### S5 — v1(c): HELD, and more exactly than called

| channel | called | landed |
| --- | --- | --- |
| `encode∘encode = id` | 500/500 | **500/500** |
| singles / cross-region / 3-per-region vs v0 | identical | **identical** (3000, 1000, 1000) |
| same-region 2000 · 2 anywhere 2000 · flagged burst 800 · Fold filled 300 · row burst 400 | within 2σ | **identical to the trial, z = 0.00 on every channel** (527/527, 1074/1074, 800/800, 0/0, 0/0) |
| inner / outer fires on a row burst | swap, Fold unchanged | **v0 264 / 149 / 254 → c 254 / 149 / 264** |

Why identical and not merely close: with `confirm` on, v0's only *chance* outcome is the
Fold pair search (Inner and Outer pairs always alias in one 11-bit residue and are always
refused). σ fixes the Fold, so a physical Fold pair is the same logical Fold pair on the
same square, and every coin lands the same way. The automorphism claim is not just within
noise; on this decoder it is exact.

### S6 — v1(b): HELD, with one 1/p case met and answered

| channel | called | landed |
| --- | --- | --- |
| `toBytes528` round-trip | exact, 66 B/square | **exact, 7 shapes** |
| singles, 3000 | 3000/3000 direct | **3000/3000, all direct** |
| doubles anywhere, 2000 | ≥ 99%, Fold–Fold the residual | **2000/2000, 0 wrong** — Fold–Fold 500/500 by search with `q`; the plan's 0.095% residual did not appear |
| both members of one pair, 500 | corrected (filed here, not in the plan) | **500/500** by the residue path |
| 12-cell **unflagged** in-region row burst, 800 | ≥ 99%, direct | first run **799/800**: one burst's twelve flips summed to 0 mod p, no region residue moved, and the decoder returned "confirm only" before looking at the mismatches. Amended to enumerate under `q` when residues are silent but mismatches are not: **800/800**, 12 cells of it by that enumeration. Both numbers kept |
| flagged burst 800 · whole Inner erased 100 | 800 · 100 by copy | **800/800 · 100/100**, all by copy |
| the Fold filled, 300 | detected, 0 wrong | **300 detected, 0 wrong** |
| cross-region pairs, 1000 | — | 1000/1000, but **180 direct / 1820 enumerated**: both hemispheres move, so the side is settled by the 4-way enumeration, not named |
| push | 0/200 | **0/200** |
| overhead | 103% / 48% | **103.03%** per data bit · **50.75%** redundant share of the artifact · **48.44%** mirrored cells over the square. The plan's 48% is the last of these |

### Standings (spec.md, 400 trials, `tools/versus.js`) — measured, replacing the filed table

| channel | codegg-v1 | eggSo-v0 | v1(a) | v1(b) | v1(c) |
| --- | --- | --- | --- | --- | --- |
| 1 flip | 400 | 400 | 400 | 400 | 400 |
| 2 anywhere | 346 | 212 | **400** (399 direct) | 400 (205 direct) | 212 |
| 2 same-region | 338 | 120 | **400** (254 direct) | 400 (254 direct) | 120 |
| 2 cross-region | 343 | 400 direct | 400 direct | 400 (33 direct) | 400 direct |
| 3 one-per-region | 0 · **36 wrong** | 400 direct | 400 direct | 400 (2 direct) | 400 direct |
| 12 flagged burst | 400 | 400 | 400 | 400 | 400 |
| **12 unflagged in-region** | 0 · **37 wrong** | 0 | 0 | **400 direct** | 0 |
| Fold filled | 352 det · **48 wrong** | 400 det | 400 det | 400 det | 400 det |
| push holds | 200/200 | 0 | 0 | 0 | 0 |
| overhead | 2.35% | 4.70% | 5.88% | 103.4% (50.7% share) | 4.70% |

**What was not filed and should have been:** codegg-v1 *miscorrects* **37 of 400** unflagged
in-region bursts — its double-error search finds a spurious pair and applies it. The plan's
standings table filed that cell as "0" and meant detected. v0 and every v1 arm refuse all
400. Also: v0's cells moved from its own versus run (191 → 212, 130 → 120) because the
five-column harness draws its damage from a different point in the same PRNG stream; the
rates are the same channel within noise.

## THE CLOSING AUDIT — every bar, called vs landed

| bar | called | landed | verdict |
| --- | --- | --- | --- |
| A1 singles, both cfgs | 3000/3000 | 3000/3000 anywhere and on the Fold, 0 wrong | **MET** |
| A2 the table | 7.25% exactly; peel = table; no single alias | 455,428 distinct exactly; peel = table on 400; 0 aliases. Twin share 14.04% — the ground's "92.7%" was 86.0% | **MET**, ground corrected |
| A3 same-region pairs | ≥ 95%, called 99%, 0 wrong | **100%** (2000/2000), 0 wrong | **MET** |
| A4 the kernel | 15–25% wrong in replaces-q | **18.0%** | **MET** |
| A5 cost | 60 bits, 5.86% | 60 bits, 5.86% | **MET** |
| A6 a name | two-syndrome arithmetic code | holds: `H = [w ; w − w∘σ]` over `Z_p` per hemisphere, the mirror weights as the second locator | **MET** |
| B1 layout | exact | exact, 66 B/square | **MET** |
| B2 singles | 3000/3000 | 3000/3000 | **MET** |
| B3 doubles anywhere | ≥ 99% | 100% | **MET** |
| B4 the one row | ≥ 99% unflagged in-region burst, 0 wrong | 799/800 first run, 800/800 after the 1/p case was answered | **MET** (with the amendment on the record) |
| B5 erasures by copy | 100/100 | 800/800, 100/100 | **MET** |
| B6 both conventions | 103% / 48% | 103.03% / 50.75% artifact share / 48.44% cells | **MET**, the 48% named for what it is |
| B7 a name | repetition ⊕ AN, error-to-erasure | holds | **MET** |
| C1 involution | exact | 500/500 | **MET** |
| C2 re-derivation | within 2σ | identical, z = 0.00 on every channel, fires swapped | **MET** exactly |
| C3 a name | no-op interleaver | holds — an automorphism of the check partition | **MET** |

Sixteen bars, sixteen met; two predictions on the *control* missed (replaces-q pairs 98% →
90.9%; Fold–Fold range 60–95% → 100%), one mis-derivation in the ground corrected (92.7% →
86.0%), one 1/p case met and answered in the mirror decoder, and one codegg-v1 failure not
filed (37 miscorrected bursts). **The house keeps v1(a):** twelve bits turn every
same-region pair v0 could only detect into a correction by lookup, with no search and no
miscorrection on any channel. v1(b) buys one row — the unflagged in-region burst — at
18× the cost, and is the only column that has it. v1(c) is v0 to the trial.
