# codegg v1 — the value is the syndrome

Not part of the site. The second codec experiment, sibling to `codec-v1/`, built from the
opposite reading of the same square.

## The reading codec-v1 missed

codec-v1 treats the square as a **bag of interchangeable symbols**: `4N−1` unweighted
sums — rows, columns, anti-diagonals — and position recovered by intersecting the ones
that disagree. It works, at 12.4% overhead.

But the square is a **number**. Cell `i` weighs `2^-(i+1)` (`stalk.js:203`), so every cell
has a distinct magnitude, and damage of size `d` at cell `i` moves the value by exactly

```
d · 2^(L−1−i)
```

— one quantity that names the cell, the direction, and the size simultaneously. Position
was encoded in place value all along. codec-v1 stored 127 sums per square to learn what
the number already knew. codegg stores the number mod two small primes — about **3 bytes
per 128-byte square** — and the syndrome *is* the error's address. Drawn in the site's own
colours it is a single lit cell sitting exactly where the wound is: the error spells its
own address. That is the page.

The second thing a symbol code cannot have: **push conserves the value**, so residues of
the value survive respelling. Canonicalise, renormalise, recode — the checks stand still
while every colour moves. All of codec-v1's sums break under push. The system's redundancy
*is* respelling, and the only checks native to it are arithmetic.

Lineage, in the site's own tradition: this is a residue / arithmetic-error code —
**Avizienis 1971**, fault-tolerant arithmetic, the same Avizienis whose 1961 signed digits
the site already credits. The parts are old. Pointing the square's own place values at its
own wounds is the only local thing here.

## The format

Systematic: **the payload is stored verbatim** — no alphabet expansion (codec-v1's chroma
path paid 2×; here the chroma alphabet is the *error space*, since a damaged cell may read
`−1`, not a storage format). A square is `L = N²` cells of payload bits, and the sidecar
holds `V mod p` and `V mod q` per square, where `p, q` are the smallest primes making
`{±2^k mod m : k < L}` pairwise distinct — verified by enumeration in the tests, never
assumed. At `N = 32`: `p = 2053`, `q = 2063`, 24 check bits per 1024 data bits.

## Results, all measured

### Head to head (`tools/versus.js spec.md`, 300 trials per channel)

```
channel                          codec-v1 (block)         codegg-v1 (arithmetic)
overhead                         12.4%  (1905 B)          2.35% (357 B)
1 bit flipped                    EXACT 300/300            EXACT 300/300
1 byte corrupted                 EXACT 300/300            exact 40, detect 229, MIS 31
1 byte corrupted, doubles off    EXACT 300/300            exact 10, detect 290
2 adjacent bytes, position known exact 1, detect 299      EXACT 300/300
12-cell row burst, known         exact 2, detect 298      EXACT 300/300
push (respell all squares)       checks hold 0/119        checks hold 119/119
```

**Both codecs win rows**, and the table is the honest summary of the whole experiment:

- codegg locates single errors from **5.3× less redundancy**, names the magnitude, repairs
  known-position damage codec-v1 can only report, and survives respelling.
- codec-v1 owns the byte-corruption channel outright — its cell *is* a byte, so one bad
  byte is one error; codegg's bit-cells make the same byte up to eight errors. And
  codec-v1 **never miscorrects**.

### What the suite pins down (`tools/codegg.test.js`)

| claim | result |
| --- | --- |
| moduli injective over `±2^k`, `k < 1024` | enumerated, both primes |
| round-trip | exact, 7 shapes, all clean |
| single error (0→1, 1→0, →−1) | located, signed, repaired **3000/3000** |
| flagged 12-cell row burst | corrected **800/800** (codec-v1 on same shape: 0/200 repaired, 200/200 detected) |
| sentinel `−1` erasures, 1–8/square, unflagged | repaired **800/800** |
| push invariance | residues hold 8/8; codec-v1 sums break 8/8 |
| overhead | **2.34%** vs 12.4% |
| true double errors (search path) | corrected 1747/2000, detected 253, **miscorrected 0** |

### The honest section

Residue checks have failure modes codec-v1 does not, and they are measured, not hidden:

- **Miscorrection.** A multi-error syndrome can imitate a smaller one and pass both
  residues. Measured: 3 scattered errors, singles only — **0.032%**; 5 scattered errors
  with the double-search on — **11.35%**; one corrupted byte with the search on —
  **31/300**. With `doubles: false` the byte channel drops to **0 miscorrections** (all
  detection). The search is a trade: it repairs 87% of true doubles, and it is the only
  path that ever lies. Choose per use.
- **Silent floor.** Random garbage passes both residues with probability
  `~1/(p·q) ≈ 2.4·10⁻⁷`. Observed: 0 in 64,000 storm trials, consistent with the floor.
  codec-v1's matching rule has no floor at all.
- **Unflagged bursts: detect only.** The true syndrome cannot be lifted from the CRT
  range (~2²²) back to `2^L`. Flag the positions and it becomes erasure repair.
- **Erasure capacity is the check size.** `k` erased bits need `≥ k` check bits, and two
  residues carry ~22 — near the information-theoretic line for erasures; the brute-force
  solver caps at 16 cells (2 bytes) per square for time. Each additional modulus adds
  ~11 bits of erasure capacity and divides the miscorrection rate by ~2000, for ~1.1%
  more overhead — the designed extension if the miscorrection rate matters.

## Running it

```bash
node codegg-v1/tools/codegg.test.js           # the eight claims
node tools/run.js codegg-v1/codegg.html       # the page executes under the harness
node codegg-v1/tools/versus.js spec.md        # the table above

node codegg-v1/tools/corrupt.js spec.md --model uniform --rate 0.0005
node codegg-v1/tools/corrupt.js spec.md --model burst --erase   # flagged: EXACT
node codegg-v1/tools/corrupt.js spec.md --model burst           # unflagged: detected
node codegg-v1/tools/corrupt.js spec.md --model sentinel        # -1 cells: EXACT

uv run serve.py    # then http://localhost:1338/codegg-v1/codegg.html
```

`corrupt.js` reports three failure classes, in order of cost: detected-but-unrepaired
(honest, no data lost), **MISCORRECTED** (repaired into wrong data — codegg's own failure
mode), **SILENTLY WRONG** (decoded clean while wrong — the floor).

## Files

| | |
| --- | --- |
| `codegg.js` | the codec: `pickModulus`, `residue`, `syndromeTable`, `encode`, `decode`, `repairSquare`, `verify`, `sizes`. Dependency-free — the check is arithmetic, so the codec is number theory |
| `codegg.html` | the page: corrupt a cell and the syndrome square lights the same cell in the error's colour; press Push and watch every colour move while both residues stand still |
| `tools/codegg.test.js` | the eight claims |
| `tools/corrupt.js` | real files through encode → corrupt → decode |
| `tools/versus.js` | the head-to-head table |

One geometric detail the page teaches: a magnitude-2 error — a cell crossing `−1 → +1` —
spells itself **one place to the left**, because `2·2^w = 2^(w+1)`. The decoder settles
the two readings by which repair lands back inside the alphabet; for stored bit-data the
tie never survives, which is why single-error correction is exact.

## What this is and is not

Same rules as codec-v1's README. Not compression — checks are added, nothing shrinks. Not
novel to coding theory — residue and arithmetic codes are Avizienis 1971 and older, and a
Reed–Solomon code would beat both of these codecs at their own game. What is defensible is
narrower and worth stating plainly: **this system's square is a number, so the code that
fits it natively is arithmetic, not block — its syndromes are its place values, its checks
survive its own respelling operation, and its error, drawn, points at itself.** codec-v1
proved the square's *shape* could locate damage; codegg proves the square's *value* could
have done it alone, at a fifth of the cost, the whole time.
