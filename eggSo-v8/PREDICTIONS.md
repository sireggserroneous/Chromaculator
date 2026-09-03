# eggSo v8 predictions — filed 2026-09-03, BEFORE the literature was searched

The series convention: the guess is written down first. Misses stay.

The twenty-third codec experiment. Rust, own crate, empty `[dependencies]`.

## What this is

v7 closed the mathematics and I gave an opinion on novelty rather than a check. Vladimir:
*"I want it answered properly please. Run the literature in rust ofcouse. And model it in
rust also with a small gui."*

So: **search the literature, then RUN it** — reimplement the prior art's constructions in
Rust and measure whether they reproduce v5/v7's characterisation. A coincidence measured is
worth more than a citation read, and it is the only way this project settles anything.

## The claim under test

v5 + v7's characterisation. For an `n x n` grid 3-coloured by `C`, `worst(C,L)` the largest
single-colour count over every length-`L` burst along a row, a column, an anti-diagonal, or
the row-major tape:

```
the LINEAR family (a*r + b*c) mod 3 reaches ceil(L/3) on all four iff
    L = 0 (mod 3):  n = 2 (mod 3)
    L = 1 (mod 3):  every n
    L = 2 (mod 3):  n != 0 (mod 3)

for 3|L and L <= n, over ALL colourings:
    reachable iff 3 | L/gcd(n,L) and 3 | L/gcd(n-1,L)
```

## Predictions, filed before searching

| claim | called |
| --- | --- |
| **is it novel?** | **NO.** I expect it to be folklore or a special case of known work, and I expect to be able to name the family it belongs to |
| where it lives | **2-D / multidimensional interleaving against cluster and burst errors.** `(a*r+b*c) mod 3` is a **lattice colouring** of `Z^2`, and lattice colourings are that field's standard tool. Blaum / Bruck / Vardy on multidimensional cluster errors, Etzion / Vardy on 2-D interleaving |
| the periodicity step | **standard.** "No slack in a window forces period `L`" is the classical argument for perfectly balanced words; bounded-discrepancy sequences are their own field |
| the construction | **a block interleaver**, already named as prior art in v5's README and on the site |
| the `check_bits` erasure ceiling | **a Singleton-type counting bound**, certainly not new |
| what may survive as ours | the **four-geometry** constraint set, specifically the wrapping row-major TAPE order, which is an artefact of storage layout rather than a natural mathematical object; and the `gcd(n-1,L)` term the anti-diagonal contributes. I expect these to be a corollary someone has stated, not a gap |
| **the negative result** | **`(r+c) mod 3` is never burst-optimal at any width** -- this is about THIS object and I expect no prior art at all, because nobody else has the fold |
| what running the literature will show | a known scheme **reproducing our characterisation exactly** on at least one of the three residues. If a known scheme disagrees with us, one of us is wrong and finding out which is the round |

**MET** if the novelty question is answered by measurement and the answer is printed whichever
way it falls. **MISSED** if novelty is claimed on the strength of not having found a paper.

## Measured (filled as parts land — never before)

Filled 2026-09-03, after `cargo test` (79 tests), `cargo clippy --all-targets -- -D warnings`
clean with no suppressions, and `eggso8 audit`. The literature was searched first, then
rebuilt and run.

### The sources found

- **Blaum, Bruck & Vardy**, "Interleaving schemes for multidimensional cluster errors",
  *IEEE Trans. Inf. Theory* **44** (1998) 730-743. `t`-interleaved arrays and lattice
  interleavers.
- **Perfect colourings of the infinite square grid**, catalogued to nine colours; the
  standard example given is `colour(x, y) = (y - x) mod 3`.
- **Optimal interleaving schemes for correcting two-dimensional cluster errors**,
  *Discrete Applied Mathematics*.

### The prediction: NOT NOVEL. Landed: **NOT NOVEL**, and measured rather than asserted

| what was run | result |
| --- | --- |
| the catalogued `(y - x) mod 3`, written from its own definition and compared cell for cell | **it IS our arm `(1,2)`, at 39 of 39 widths** -- and `(1,2)` is one of exactly the two arms the theorem names at `n = 2 (mod 3)` |
| index-3 **lattice interleavers**, built GEOMETRICALLY (cells grouped by whether their difference lies in the sublattice, cosets numbered as first met, nothing assuming a linear form) | **20 of 20 land inside our nine-arm family** |
| **BBV's criterion** -- distinctness on a connected cluster of area `t` | needs `t` colours for area `t`, so from `t = 4` it **cannot be posed** with three colours. Our arms break their distinctness at area **3** |
| the three **spatial** geometries alone, swept over `n = 15..36` | **width-free for every arm**, and equal to exactly the three conditions `b != 0`, `a != 0`, `a != b`, with no reference to `n` |
| the **full** verdict including the tape | **moves with `n`** -- `(1,2)` at `L = 12` works at `n = 32` and fails at `n = 30` on the tape alone |

So every filed prediction about where it lives held. The family is catalogued, the lattice
recipe builds it by another route, the forced-periodicity step is the standard argument for
balanced words, and the construction is a block interleaver -- which v5's README and the site
item already called prior art.

### What the searched literature does not have

The **tape**. The lattice work colours the *infinite* lattice `Z^2`, which has no row-major
read order and therefore no phase slip at a row boundary. Every width-dependent quantity in
v5/v7 -- `n mod 3`, `gcd(n, L)`, `gcd(n-1, L)` -- comes from the tape alone, and none of them
has anywhere to appear in `Z^2`. Measured above: strip the tape and the verdict stops
depending on `n` at all.

**And that is an engineering geometry rather than a mathematical object.** The honest claim
is a corollary nobody needed, not a gap somebody missed. I am not going to dress it up as the
latter.

### What is ours

The **negative result**: `(r+c) mod 3` -- the fold's own level sets -- is never burst-optimal
at any width, because an anti-diagonal is its level set. Nobody else has the fold, so nobody
else had reason to check. That is the one on the site, and it is the one that stands.

### The model

A zero-dependency terminal explorer, `eggso8 model`. Rendering is pure -- `render(&View) ->
String` -- so the display is tested with no terminal attached; only `run` touches stdin. It
shows, on one screen, the grid coloured by class, the four geometries against the floor, the
closed form's verdict, and the spatial-only verdict beside the full one. At `n = 30, L = 12,
arm (1,2)` it prints row 4, col 4, diag 4, **tape 5** -- the entire finding in four rows.

**It is a terminal interface and not a window, and that was a choice.** A windowed GUI needs
crates; `[dependencies]` has been empty for every Rust round in this repo and the law outranks
the convenience.

### The bar

**MET.** The novelty question was answered by measurement and the answer is printed the way it
fell: not novel in the mathematics, with the one part the search did not find named as an
engineering detail rather than promoted to a discovery.
