# codec v1 — the square is a product code

Not part of the site. An experiment, kept in its own folder so it does not entangle with
`chronochromatic.org`, which claims none of this.

## Why

Six rounds of measurement established what this project's representation cannot do as an
encoding, and those results are settled:

| claim | result |
| --- | --- |
| compression | **impossible** — `pushLeft` is a bijection onto values (4096 → 4096 distinct pushed forms) |
| generative recipes | **no saving** — `bits(A) + bits(B) − bits(A·B)` is 0 in 61% of pairs and *positive* on average (+0.392 bits) |
| canonicity as error detection | **48.0%** of single-symbol errors, and **0 of 50,000 sign flips** |

That last row is the reason this folder exists. The canonical form's rule is "no green may
be followed by a lit cell," so every green is trailing — and flipping a lit cell's sign
preserves that exactly. A `+1 ↔ −1` confusion, which is the *most likely* error in a
colour-coded medium, is structurally invisible to the representation.

Row parity is not blind to it. A sign flip moves its row sum by 2.

And the square is already there. `spec.md` builds it for a purely dimensional reason —
`n = ⌈√L⌉` is the smallest one that holds the stalk — but that shape is the geometry of a
**product code**. This gives the square an operational justification it did not have.

## What it does

Sums every row, every column, and every anti-diagonal. Anti-diagonals are not decoration:
cell weight in this representation depends only on `r + c`, so the anti-diagonals *are* the
place values, and they carry real coding power.

`N` rows + `N` cols + `2N−1` diagonals = `4N−1` sums for `N²` cells:

| N | sums | overhead |
| --- | --- | --- |
| 6 | 23 | 63.9% |
| 12 | 47 | 32.6% |
| 16 | 63 | 24.6% |
| **32** | **127** | **12.4%** |
| 64 | 255 | 6.2% |

Default `N = 32`.

## Results

All measured, all reproducible by the commands below.

### What works

| case | row + column | + anti-diagonal |
| --- | --- | --- |
| single error | 100% located, 4000/4000 both alphabets | same |
| **sign flips** | — | **5000/5000 caught and repaired** |
| two errors | 0/2814 corrected (all detected) | **2801/2801 corrected**, 0 silent |
| 4-corner rectangle | **0/2829 detected — 2829 silently wrong** | **2817/2817 detected**, 0 silent |

Of the 5000 sign flips, **726 were invisible to canonicity** — that is the hole this
closes, measured rather than argued.

### Rectangle detection is total, not partial

The plan predicted `≈1 − O(1/N)` detection, expecting the family `r1−r2 = c1−c2` to
survive by collapsing two diagonals. That was too pessimistic. The four corners land on
diagonals `r1+c1`, `r1+c2`, `r2+c1`, `r2+c2`; the first can only collide with the fourth
(needs `r1−r2 = c2−c1`) and the second only with the third (needs `r1−r2 = c1−c2`), and
both at once forces `c1 = c2`, which is excluded. So at least two diagonals are always hit
exactly once.

Checked exhaustively over every rectangle for `N = 2..40`:

```
20,458,672 rectangles
  both diagonal pairs collide (would be invisible): 0
  exactly one pair collides (still 2 unbalanced)  : 852,800
  no collision (all 4 unbalanced)                 : 19,605,872
```

**Anti-diagonal parity detects every 4-corner rectangle.**

### What does not work: bursts

A burst along a row puts many errors in one row, so the row/column matching that locates a
single error has nothing to match against. Measured on real files, an 8-cell burst is
reliably *detected* and reliably *not repaired*. The classical fix is interleaving — map
consecutive symbols to different rows so a burst becomes scattered errors. **Not
implemented here.** It is the obvious next step and would not change the parity layer at
all, only the symbol→cell mapping.

### The chroma layer costs 2.6× and buys nothing

Predicted before building, then confirmed at `N=32` on the same 4096-byte source:

| alphabet | data | parity | total | vs source |
| --- | --- | --- | --- | --- |
| byte | 4096 B | 508 B | 4604 B | **1.12×** |
| chroma | 8192 B | 3556 B | 11748 B | **2.87×** |

Identical geometry, identical correction power, identical error behaviour. One byte becomes
8 signed-digit cells at 2 bits each, which is a 2× expansion before parity is even added.

**The honest finding: routing data through chroma cells costs ~2.6× for no gain in
protection.** The `chroma` alphabet is worth keeping because it makes the sign-flip result
visible in the site's own language, and because it is the thing being measured — not
because anyone should ship it.

### The control

Correction is a property of the parity geometry, not of the data. Single-error repair on
two maximally different sources of the same length:

```
random bytes  800/800
spec.md text  800/800
```

Identical, as they must be. This is why "test it on real files" cannot establish correction
power — real files are measured here for **overhead and plumbing**, not for power.

### Real files

Every file below round-trips **exactly** under a uniform error rate of 0.0005, and **not one
silently-wrong result appeared anywhere with diagonals on**:

```
spec.md       uniform inj  11  fixed  11  EXACT
stalk.js      uniform inj  12  fixed  12  EXACT
uv.lock       uniform inj   2  fixed   2  EXACT
yinyang.svg   uniform inj   4  fixed   4  EXACT
rand-4k.bin   uniform inj   3  fixed   3  EXACT
rand-64k.bin  uniform inj  38  fixed  38  EXACT
```

The clearest demonstration of what the anti-diagonals are for, on a real file:

```
spec.md, chroma, rect, diagonals on
  squares  114 clean, 0 corrected, 5 detected-unrepaired
  result   18 bytes differ
  SILENTLY WRONG: no

spec.md, chroma, rect, diagonals OFF
  squares  119 clean, 0 corrected, 0 detected-unrepaired      <- all "clean"
  result   18 bytes differ
  SILENTLY WRONG: YES -- data lost without warning
```

Same damage, same file. Without the third parity vector every square reports clean while
18 bytes are wrong.

## Running it

```bash
node codec-v1/tools/chromacode.test.js        # eight claims, each predicted first
node tools/run.js codec-v1/chromacode.html    # does the page's JavaScript execute?

node codec-v1/tools/corrupt.js spec.md --alphabet byte   --model uniform --rate 0.0005
node codec-v1/tools/corrupt.js spec.md --alphabet chroma --model flip    --hits 8
node codec-v1/tools/corrupt.js spec.md --alphabet chroma --model rect    --hits 5
node codec-v1/tools/corrupt.js spec.md --alphabet chroma --model rect    --hits 5 --no-diags

uv run serve.py    # then http://localhost:1338/codec-v1/chromacode.html
```

`corrupt.js` flags: `--alphabet byte|chroma`, `--N 32`, `--model uniform|burst|rect|flip`,
`--rate`, `--burst`, `--hits`, `--no-diags`, `--quiet`.

The number that matters in its output is **SILENTLY WRONG**. A detected-but-unrepaired file
is an honest outcome for a code that has run out of parity; a file that decodes clean while
differing from the source is the failure that costs data.

## Files

| | |
| --- | --- |
| `chromacode.js` | the codec. `ALPHABETS`, `encode`, `decode`, `parities`, `repairSquare`, `sizes` |
| `chromacode.html` | the page: click a cell, watch the sums break, press Repair |
| `tools/chromacode.test.js` | the eight claims |
| `tools/corrupt.js` | real files through encode → corrupt → decode |

`chromacode.js` reuses `pushLeft` from `../stalk.js`. In the browser both are plain scripts
and it is simply a global; under node it evaluates `stalk.js` into its own module scope,
which is why `stalk.js` keeps its top-level declarations as `function`.

Two traps worth recording, both of which bit during construction:

- **`squareFor` in `stalk.js` is a `const` arrow and does not survive `eval`.** Every test
  in `tools/` redefines it locally. So does this codec.
- **`hexSequence` is wrong for a codec.** It pads to whole nibbles but discards leading-zero
  *width* — the measured non-injectivity where `value(16) == value(256) == 1/16`, and width
  survived a round-trip in only 2001 of 4001 cases. Bytes are expanded to a fixed 8 bits
  here instead.

And one bug the tests caught: a 4-corner rectangle cancels row and column sums only if the
four deltas alternate `+d −d −d +d`. Applying the *same* step to all four works under XOR,
because XOR is its own inverse, and does not work under an integer sum. The first version of
both the page and `corrupt.js` had it wrong, which made the blind-spot demonstration
silently test something else.

## What this is not

Not compression — it deliberately makes files larger. Not novel: product codes are
classical (RAID, tape, 1950s), and Hamming beats 2-D parity on rate at `~2 log N` checks
against `4N−1`. What is defensible is narrower and worth stating plainly: **the square this
project already draws, for reasons that had nothing to do with coding, turns out to be the
right shape for locating a corrupted cell — and its anti-diagonals, which are its place
values, detect every rectangle that defeats the classical version.**
