# chronochromatic · v2 specification

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

A number is a stalk of cells, read left to right.

- **Cell 0 is A1.** Reserved. Always green, weight `2^0`. It is never a digit.
- **Cell `i` weighs `2^-i`.** Each further cell halves the step, so reading rightward is reading precision — the stalk is a decimal expansion, not a place-value column.
- **The digits are the integer's bits, smallest first.**

The value is

```
value(k) = Σ bit_j(k) · 2^-(j+1)
```

which is `k`'s bits reversed over `2^L`. Every integer lands on its own dyadic in `(0, 1)`; no two collide.

| k | binary | stalk | pushed | value |
| --- | --- | --- | --- | --- |
| 1 | 1 | `0,1` | `1,-` | 1/2 |
| 2 | 10 | `0,01` | `1,--` | 1/4 |
| 3 | 11 | `0,11` | `1,-1` | 3/4 |
| 4 | 100 | `0,00,1` | `1,--,-` | 1/8 |
| 5 | 101 | `0,10,1` | `1,-1,-` | 5/8 |
| 6 | 110 | `0,01,1` | `1,--,1` | 3/8 |
| 7 | 111 | `0,11,1` | `1,-1,1` | 7/8 |

## Commas

Commas fall on the anti-diagonals of the square the stalk folds into. For an `n × n` square the group lengths run

```
1, 2, 3, … n, … 3, 2, 1
```

The stalk **stops when the bits run out**. The 1D reading never shows padding — only the square is padded.

## Push

A lit cell may step toward the coarse end, into a green on its left, leaving its own sign flipped behind:

```
+1 · 2^-i   ==   +1 · 2^-(i-1)  −  1 · 2^-i
```

Run to a fixpoint. The value never moves; only the colours do. Push runs *leftward* here because the place values run rightward — the mirror of the v1 rule.

## The square

`n` is the smallest square that holds the stalk: `n = ⌈√L⌉`. Cells fill in **Hankel order** — anti-diagonal by anti-diagonal, each read from the bottom-left corner upward, which is the `A1 / B1 B2 / C1 C2 C3` naming. Leftover cells are padded green.

## The fold

The main anti-diagonal is `r + c = n − 1`.

| region | cells | meaning |
| --- | --- | --- |
| Inner | `r + c < n − 1` | the coarse end — interior of the circle, one hemisphere |
| Fold | `r + c = n − 1` | the circle of inversion, the equator |
| Outer | `r + c > n − 1` | the fine end — exterior, the other hemisphere |

The map `(r,c) ↦ (n−1−c, n−1−r)` is the anti-transpose `J Aᵀ J`. It fixes the Fold, swaps Inner with Outer, and undoes itself — the three defining properties of inversion in a circle.

## Open questions

- Whether the square should pad after the last bit or before the first. Padding after makes trailing zeros invisible; padding before keeps every integer distinct but moves the leading bit.
- Whether the Fold's reading should be its place value or its stalk value.
