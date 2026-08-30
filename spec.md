# chronochromatic · v3 specification

*Editable in place. Save writes this file back to disk.*

## Digits

Three colours, one signed digit each.

| colour | digit | glyph |
| --- | --- | --- |
| blue | +1 | `1` |
| green | 0 | `0` |
| red | −1 | `−` |

Negating a number flips every colour. Green is its own opposite, so it never moves.

## The stalk

A number is written in **hex**, so its bits arrive already padded to a whole nibble, and they lay
in **most significant first**. There is no reserved cell: the leading zeros of the hex form are
the padding, and nothing has to be set aside for them.

**Cell `i` weighs `2^-(i+1)`.** So a stalk of `L` cells is a fraction over `2^L`, and for an
integer `k` written in `N` nibbles the value is

```
value(k) = k / 2^(4N)
```

which is strictly inside `(−1, 1)` and, because the bits are not reversed, **order-preserving**:
if `j < k` and both take the same number of nibbles, `value(j) < value(k)`.

| k | binary | stalk | pushed | square | value |
| --- | --- | --- | --- | --- | --- |
| 1 | `1` | `0001` | `1−−−` | 2×2 | 1/16 |
| 2 | `10` | `0010` | `1−−0` | 2×2 | 1/8 |
| 3 | `11` | `0011` | `1−−1` | 2×2 | 3/16 |
| 7 | `111` | `0111` | `1−11` | 2×2 | 7/16 |
| 8 | `1000` | `1000` | `1000` | 2×2 | 1/2 |
| 15 | `1111` | `1111` | `1111` | 2×2 | 15/16 |
| 16 | `10000` | `00010000` | `1−−−0000` | 3×3 | 1/16 |
| 255 | `11111111` | `11111111` | `11111111` | 3×3 | 255/256 |

Nibble padding is why `16` and `256` both read `1/16`: they are the same stalk on a wider field.

## Push

A lit cell may step toward the coarse end, into a green on its left, leaving its own sign flipped
behind:

```
+1 · 2^-i   ==   +1 · 2^-(i-1)  −  1 · 2^-i
```

Run to a fixpoint. The value never moves; only the colours do. A stalk with no greens — `8`, `15`,
`255` above — is already at its fixpoint and push does nothing.

**Push** spells a number with the most lit cells; **NAF** spells it with the fewest. Push is the
address; NAF is the fastest route to it.

## The square

`n` is the smallest square that holds the stalk: `n = ⌈√L⌉`. Cells fill **row-major**, left to
right and top to bottom, the way the stalk reads. Leftover cells are padded green.

## The fold

The main anti-diagonal is `r + c = n − 1`.

| region | cells | meaning |
| --- | --- | --- |
| Inner | `r + c < n − 1` | the coarse end — one hemisphere, reaching north |
| Fold | `r + c = n − 1` | the equator |
| Outer | `r + c > n − 1` | the fine end — the other hemisphere, reaching south |

The three sum back to the number exactly. The map `(r,c) ↦ (n−1−c, n−1−r)` fixes the Fold, swaps
Inner with Outer, and undoes itself — the three defining properties of inversion in a circle.

## Multiplication — the rectangle

Multiply two stalks cell by cell and nothing carries. Every cell of **A** meets every cell of
**B**, and the pair weighs

```
2^-(c+1) · 2^-(r+1)  =  2^-(r+c+2)
```

**A** runs along the columns and **B** down the rows, so `m` cells times `n` cells is `n` rows by
`m` columns. The operands keep their sides: `A×B` and `B×A` are transposes. Summing the whole
rectangle is the product, exactly.

Weight still depends only on `r+c`, so the anti-diagonals are still the place values, and the fold
is still one of them: the anti-diagonal through the **last cell of the shorter operand**,
`r + c = L − 1` where `L = min(rows, cols)` — the last one that still reaches both edges.

`3 × 10`, both four cells:

```
    0011      A = 0011  (3/16)
    0000      B = 1010  (10/16)
    0011
    0000      sum = 15/128,  fold on r+c = 3
```

A sequence is multiplied **left to right**, so `n` operands give `n−1` rectangles. Each grid is
read back out as a plain stalk to be the next step's left operand.

**What order does and does not change.** Swapping a single pair only transposes the rectangle.
Weight depends on `r+c`, and transposing preserves `r+c`, so Inner, Fold and Outer come out
identical — measured, 625 of 625 pairs. Order only bites in a chain of three or more, where the
accumulator's length changes and drags the fold with it: 268 of 343 three-operand reorderings moved
the regions. The product itself never moves.

## Division — quotient, multiplier, remainder

`A/B` will not fit a grid the way `A×B` does. Multiplication distributes and reciprocal does not:
`1/(x+y) ≠ 1/x + 1/y`. Laying a grid of pairwise ratios at weight `2^(i−j)` is exact only when `B`
has a single lit cell — which is a shift, the one division that is linear. Measured: 540 of 540
exact with one lit cell, 0 of 17,460 with more.

Worse, most quotients repeat forever, and half of them leave `(−1, 1)` altogether.

Both problems go away by refusing to round. Run the division only as far as the grid allows and
keep what is left over:

```
A  =  2^e · Q · B  +  R
```

**Q** is an ordinary stalk inside `(−1, 1)`, so it draws like every other number.
**e** is the **multiplier on the boundary** — the shift that brings the quotient back inside,
normalised so `|2^-e · A/B|` lands in `[1/2, 1)` and the leading cell is never wasted.
**R** is a stalk too, always dyadic and always inside `(−1, 1)`. It is exactly what the repeating
digits would have been.

Widening the grid grows `Q` and shrinks `R`. The identity never moves.

```
3 / 10, whose true quotient repeats forever:

  W=4   Q = 1001        e = −1   R = 3/256    R as a stalk  00000011
  W=8   Q = 10011001    e = −1   R = 3/4096   R as a stalk  000000000011
```

The tableau — the rows of `B` that get subtracted — is the multiplication rectangle of `B` and `Q`.
Multiplication's grid is **read**; division's grid is **searched**. Each row is one yes-or-no
question: does this shifted copy of `B` fit in what is left?

## The horizon and its rings

The multiplier bands the numbers into rings. Ring `e` holds every stalk with its leading cell lit,
scaled by `2^e`, and reaches

```
2^e − 2^(e−W)
```

— `2^e` minus exactly one step. Never two steps short, never zero. The rings **tile**: the top of
ring `e` plus one step is the bottom of ring `e+1`, with no gap and no overlap.

That deficit is load-bearing. If a ring closed, its top would equal the next ring's bottom and the
value would have two addresses. **The tiny is what makes the address unique.**

The surreals are short the same way. Each generation holds exactly `2^n`, and everything up to and
including it holds `2^(n+1) − 1` — always one short of a power of two, forever. The missing one is
the horizon itself.

**The two ring systems are different geometries, and the boundary is where they change.**

| | population | step | geometry |
| --- | --- | --- | --- |
| Atlas ring `r`, inward | `2^r`, doubling | `2^-(r+1)`, halving | hyperbolic |
| Horizon ring `e`, outward | `2^(W−1)`, fixed | `2^(e−W)`, doubling | logarithmic |

The Atlas draws ring `r` at a **linear** radius holding `2^r` tiles. Circumference growing like
`e^(r ln 2)` against a linear radius is the signature of hyperbolic space — Euclidean would be
linear. One ring per unit puts the curvature at `K = −(ln 2)² ≈ −0.48`, and the picture is a
Poincaré disc of the dyadic tree: **big in the middle, small at the edge**, with the rim an ideal
boundary at infinite distance. That is why it is a horizon and not a wall. You do not approach it
and stop. You never arrive, at any width.

Outside the boundary the population per band is fixed, which is uniform in `log(value)` — a
cylinder, not a disc. This layout is floating point: mantissa normalised into a band, exponent
naming the band. The difference is that floating point rounds inside the band and forgets, and this
keeps `R`.

## The anatomy of 0

**0 is not a point.** Between `−0` and `+0` there is an entire structure, and it is where the site's
numbers come from. Everything above is what happens *after* you are out of it.

**It is not linearly ordered, and that is not a detail.** The game `∗ = {0 | 0}` is not less than
`0`, not greater than `0`, and not equal to `0` — it is *confused* with it. Trichotomy fails, so
there is no side to be on. **Left** and **right** are total-order words and cannot be used in here.
**Red** and **Blue** can, because they name *whose advantage*, not which direction — which is also
the Winning Ways convention: bLue is Left, Red is Right. The vocabulary did not happen to fit. It
is the only vocabulary that can.

The order, Red to Blue:

```
−0 | tiny-on | tiny | up | over | +ε | ±ω | −ε | under | down | miny | miny-on | +0
```

Every step ascends. `under < down` because `under = −over` and `over > up`; `down < miny` by the
same mirror. `miny = −tiny` exactly, so the two halves are one negation apart.

```
∗   star    { 0 | 0 }          confused with 0 — no position at all
↑   up      { 0 | ∗ }          positive, below every positive NUMBER
over        { 0 | over }       a loopy infinitesimal, above the short all-small games
⧾   tiny    { 0 || 0 | −G }    positive, below n·↑ for every n
on          { on | }           pronounced omega. Above everything
```

**The rim is made of the centre.** `tiny-on` is not tiny stacked on tiny — `on` is a game, and
`tiny-on` is tiny *parameterised by* it: the smallest thing you can build, made out of the biggest
thing there is. So the disc closes on itself. Start at `tiny-on`, end at `on`, and `on` is what
`tiny-on` was made of. Every positive infinitesimal is greater than `1/on`, so the floor of the
whole zoo is the reciprocal of what sits at the centre.

**The pinhole is a real map.** Stretch the hole at 0 open and look down, and `on` is what is at the
bottom — because it was the material all along. That is inversion in a circle, `z ↦ 1/z`, the
conformal map that swaps the centre of a Poincaré disc with its ideal boundary. The two rings drawn
outside the disc are `+0` and `−0`, and the reciprocal sends them straight to the centre. Floating
point agrees in its blunt way: `1/+0 = +∞` and `1/−0 = −∞`.

**0 is doubled and ω is unified**, and that asymmetry is a choice, not a given. Floating point
doubles both. The Riemann sphere unifies both. Doubling only zero is what makes this a *disc with a
centre* rather than a sphere, and it is what turns the list above into a closed loop rather than a
line.

**The bits cannot start until you are out.** A sign expansion is a *number's* address, and `∗`, `↑`
and `⧾` are not numbers — they have no position for a bit string to point at. So the digits at the
top of this document begin exactly where the game zoo ends. That is forced by the theory, not
chosen.

And everything ordinarily called a number — the counting numbers, the reals, the irrationals, all
of analysis — is the single unlabelled band between `ε` and `ω`. One step of thirteen. All the
structure is at the two ends; the middle, where mathematics normally lives, is the featureless part
of the picture.

## What is checked

Every claim below was measured, not assumed. Where a guess failed it is not listed.

| claim | result |
| --- | --- |
| hex value is order-preserving | 253/253 |
| Inner + Fold + Outer sums to the value | exact, worst gap 0 |
| the rectangle sums to `A×B` | 3321/3321 pairs |
| a grid read back as a stalk keeps its value | 3721/3721 |
| a pair swap leaves Inner/Fold/Outer identical | 625/625 |
| `A = 2^e · Q · B + R` | 38640/38640 |
| `R` is dyadic and inside `(−1, 1)` | 38640/38640 each |
| `R` never grows when the grid widens | 12000/12000 |
| the division tableau equals `Q × B` | 2400/2400 |
| ring `r` is `2^(r+1)` roots of unity | spacing exactly `360/2^(r+1)` |
| the grid's turn-walk matches the Atlas angle | 20000/20000, both spellings |

## Open questions

- Whether `over > n·↑` for all `n`, which the ordering above assumes. It matches every source consulted, but has not been checked against Siegel's tables.
- Whether the horizon rings belong on the Atlas as a continuation, given that the geometry changes from hyperbolic to logarithmic at the boundary.
- Whether the remainder should be reported in `A`'s units, as here, or as `R/B` in the quotient's.
