# The reference shelf

Published books, kept here so they can be read at `http://localhost:1338/refs/…`
while working. **They are not in the repository** — `.gitignore` excludes
`refs/*.pdf`, so they never enter git history and never reach GitHub. They are
hardlinks to the copies in `~/Downloads`, which costs no extra disk and leaves
the originals where they were.

Anyone else cloning this repo gets this file and an empty folder. What travels
is the citations below, not the books.

| file | what it is |
| --- | --- |
| `winning-ways-v1.pdf` | Berlekamp–Conway–Guy, *Winning Ways for Your Mathematical Plays*, 2nd ed., vol. 1 (chapters 1–8) |
| `winning-ways-v4.pdf` | the same, vol. 4 (chapters 23–25, peg solitaire — nothing here bears on this project) |
| `conway-atlas-of-finite-groups.pdf` | Conway et al., *Atlas of Finite Groups*. Conway, but about finite simple groups — unrelated to the number work |

Only vol. 1 has anything load-bearing. Note that vols. 1 and 4 are image scans
with no text layer (`pdftotext` yields about one character per page); to read
them, render the page and look at it:

```
pdftoppm -r 150 -png -f <page> -l <page> refs/winning-ways-v1.pdf out
```

The Atlas of Finite Groups does have a text layer, if that is ever useful.

## Page offset

Volume 1's printed page *n* is PDF page *n + 20*. The index runs from printed
page 267 (PDF 288).

## What has been read, and where

All from vol. 1, ch. 5, **Numbers, Nimbers and Numberless Wonders**, printed
p. 126 (PDF page 146) unless noted.

| claim | source |
| --- | --- |
| tiny-`x` is written `+ₓ`, miny-`x` is written `−ₓ` | p. 126. The book's own notation for the two halves is plus and minus |
| miny is exactly the negative of tiny: `−ₓ = {{x \| 0} \| 0}` | p. 126, *"The negative of `+ₓ` is, of course…"* |
| **up is tiny-zero**: `+₀ = {0 \| {0 \| 0}} = {0 \| ∗} = ↑` | p. 126. Up is not a separate species — it is the tiny with parameter 0 |
| a tiny shrinks as its parameter grows, faster than any multiple can catch | p. 126: *"if `x` and `y` are numbers with `x > y ≥ 0`, then `+ₓ` is so much smaller than `+ᵧ` that no matter how many terms `+ₓ` we add to each other, the sum will be less than `+ᵧ`"* |
| `n · tiny < ↑` for every `n` | p. 126: *"So any multiple of `+¼` will be less than ↑"* — which, with the line above, is the same statement |
| tiny values arise in Toads-and-Frogs | p. 127, *Tiny Toads-and-Frogs* |
| index entries for tiny | printed p. 275: tiny 126, 127, 169, 170; tiny-`x` 126; tiny-a-quarter 132; tiny-two 126 |

## What is still not here

**over, under and on.** These are loopy games, and they are in **chapter 11,
*Games Infinite and Indefinite* — volume 2**, which is not on the shelf. Conway,
*On Numbers and Games*, ch. 11 covers the same ground.

That leaves one claim in `spec.md` uncited: whether `over > n·↑` for every `n`.
Volume 2 or ONAG would settle it.
