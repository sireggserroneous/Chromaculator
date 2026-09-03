# eggSo v8 — the literature, run rather than read

Not part of the site. The twenty-third codec experiment. [`v7`](../eggSo-v7/) closed the
mathematics, and then I answered "is it novel?" with an **opinion**. This round answers it the
way the project answers everything else: by rebuilding the prior art from its own definitions
and measuring whether it reproduces ours.

Rust, its own crate, empty `[dependencies]`. Built 2026-09-03 against
[PREDICTIONS.md](PREDICTIONS.md), filed **before the literature was searched**.

## The verdict, first

**NOT NOVEL** in the part that is mathematics — and now measured, not estimated.

| what was run | result |
| --- | --- |
| the catalogued perfect colouring `(y − x) mod 3`, written from its own definition and compared cell for cell | **it IS our arm `(1,2)`, at 39 of 39 widths** |
| index-3 **lattice interleavers**, built geometrically with nothing assuming a linear form | **20 of 20 land inside our nine-arm family** |
| **Blaum–Bruck–Vardy's** criterion — distinctness on a connected cluster of area `t` | needs `t` colours for area `t`, so from `t = 4` it **cannot be posed** with three colours |
| the three **spatial** geometries alone, over `n = 15..36` | **width-free for every arm** |
| the **full** verdict, tape included | **moves with `n`** |

The colouring family we enumerated is catalogued. The lattice recipe builds it by another
route. The forced-periodicity step is the standard argument for balanced words. The
construction is a block interleaver — which v5's README and the site item already called prior
art. Every filed prediction about where the result lives held.

## The sources

- **Blaum, Bruck & Vardy**, "Interleaving schemes for multidimensional cluster errors",
  *IEEE Trans. Inf. Theory* **44** (1998) 730–743 — `t`-interleaved arrays, lattice
  interleavers.
- **Perfect colourings of the infinite square grid**, catalogued to nine colours; the standard
  example is `colour(x, y) = (y − x) mod 3`.
- **Optimal interleaving schemes for correcting two-dimensional cluster errors**, *Discrete
  Applied Mathematics*.

## Where ours is genuinely a different question

Three differences, and only the third is interesting.

**1. Distinctness against multiplicity.** BBV require *all-distinct* colours on a cluster. We
fix three colours and bound the *multiplicity* at `⌈L/3⌉`. Their own pigeonhole needs `t`
colours for area `t`, so with three colours their question is unaskable from `t = 4`. Ours is
its **relaxation**, not an instance of it.

**2. Clusters against lines.** Their bursts are connected regions measured by area; ours are
four specified line geometries.

**3. `Z²` against a finite array read as a tape — and this is the real one.** The lattice
literature colours the **infinite** lattice, which has no row-major read order and therefore
no phase slip at a row boundary. Every width-dependent quantity in v5/v7 — `n mod 3`,
`gcd(n, L)`, `gcd(n−1, L)` — comes from the tape alone. Measured: strip the tape and the
verdict stops depending on `n` at all.

**But that is an engineering geometry, not a mathematical object.** The honest claim is a
corollary nobody needed, not a gap somebody missed, and I am not going to dress it up as the
latter.

## What is actually ours

The **negative result**: `(r+c) mod 3` — the fold's own level sets — is **never**
burst-optimal at any width, because an anti-diagonal is its level set. Nobody else has the
fold, so nobody else had reason to check. That is the claim on the site, and it stands.

## The model

`eggso8 model` — a zero-dependency terminal explorer. Rendering is **pure**,
`render(&View) -> String`, so the whole display is tested with no terminal attached; only
`run` touches stdin. On one screen: the grid coloured by class, the four geometries against
the floor, the closed form's verdict, and the spatial-only verdict beside the full one.

At `n = 30`, `L = 12`, arm `(1,2)` it prints the entire finding in four rows:

```
  geometry      worst   floor   at floor?
  row               4       4   yes
  col               4       4   yes
  diag              4       4   yes
  tape              5       4   NO

  spatial only (row/col/diag, no tape): at the floor
  so THIS width fails on the tape alone: the phase slip at the row boundary.
```

Commands: `n <N>`, `l <L>`, `arm <a> <b>`, `arm lit`, `plain`, `colour`, `q`.

**It is a terminal interface and not a window, and that was a choice.** A windowed GUI needs
crates — `egui`, `minifb`, `winit` — and `[dependencies]` has been empty for every Rust round
in this repo. The law outranks the convenience. What it costs is a title bar; what it buys is
that this round builds and runs anywhere the other seven do, with no network and no supply
chain. If a real window is wanted, it belongs in a separate crate outside the frozen-record
lineage.

## Running it

```
cargo build --release
cargo test                                    # 79 tests
cargo clippy --all-targets -- -D warnings     # clean, no suppressions

cargo run --release -- pin      # the copy against v7's committed record; no node needed
cargo run --release -- lit      # the prior art rebuilt and run against ours
cargo run --release -- model    # the interactive terminal model
cargo run --release -- audit    # pin + lit
```

## Files

```
Cargo.toml        name = "eggso8", edition 2021, NO dependencies
PREDICTIONS.md    filed before the literature was searched
src/lib.rs        pub mod declarations
src/main.rs       pin | lit | model | audit
src/lit.rs        NEW: the prior art, from its own definitions
src/tui.rs        NEW: the model, pure render + a line-driven loop
src/thirds.rs     carried from v7 -- the characterisation under test
src/optimum.rs    carried -- worst(C,L), the floor, the linear family
src/code.rs       carried
src/caps.rs       carried
src/fold.rs       carried unchanged since v4
src/seam.rs       carried
src/pin.rs        carried, repointed at v7's record
src/json.rs       carried unchanged
measured-*.json   what the binary wrote
```

Everything is carried because v8 checks v7's **whole** characterisation against the
literature, so it needs the machinery that produced it — and the pin holds the copy to v7's
committed 409 cases / 0 disagreements.

## What this is and is not

- **It is a novelty audit, and it came back negative.** That was the filed prediction and it
  is the useful outcome: the result is correct, verified, and already known.
- **The one part the search did not find is labelled an engineering detail**, because that is
  what it is.
- **A literature search is not a proof of absence.** Three sources were rebuilt; the field is
  larger. If the tape corollary is written down somewhere, this round would not have found it,
  and that limitation is the round's own.
