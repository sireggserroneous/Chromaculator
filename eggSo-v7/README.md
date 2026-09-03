# eggSo v7 — the last two things worth doing

Not part of the site. The twenty-second codec experiment and the eighth in the fold-native
lineage. [`v6`](../eggSo-v6/) ended with a verdict: the construction's ceiling is
`check_bits`, and every road onward leads to Reed–Solomon, which this repo has already
reached from two other doors. So there is no engineering round left. This one is deliberately
the two items that survive that verdict, and nothing else.

Rust, its own crate, empty `[dependencies]`. Built 2026-09-03 against
[PREDICTIONS.md](PREDICTIONS.md), filed before a line of the round was written.

## The verdict, first

**Six bars, six met. One filed prediction missed — and the miss is what settled two of the
three open cases.**

### 1. The characterisation is finished

v5 settled `L` divisible by 3. The other two residues come out of one piece of arithmetic: a
tape run of `L` crossing a row boundary is two arithmetic progressions with a phase slip of
`m` and `L−m` cells, so its worst class is bounded by `⌈m/3⌉ + ⌈(L−m)/3⌉`. At `L = 3t+1`
**every** split gives at most `t+1` — the slip is always absorbed, the tape condition is
vacuous, and the four conditions collapse to three.

```
a linear partition reaches ceil(L/3) on all four geometries iff
    L = 0 (mod 3):  n = 2 (mod 3)          <- eggSo-v5
    L = 1 (mod 3):  every n                <- this round
    L = 2 (mod 3):  n != 0 (mod 3)         <- this round
```

**409 cases, 0 disagreements**, over `n = 8..36` and `L = 3..18`, re-derived by measurement
rather than quoted. The linear family is now characterised for **every** `L`.

### 2. Two of three open cases settled, by following the data

| `(n, L)` | verdict | structure |
| --- | --- | --- |
| **(30, 8)** | **REACHED** | nonlinear, tape-periodic period 9 |
| **(33, 8)** | **REACHED** | nonlinear, tape-periodic period 9 |
| **(33, 11)** | **INCONCLUSIVE** | — |

I predicted the solutions would be **non-periodic**, and every one of them is **periodic**.
That miss is what cracked it: the exhibited partitions at `(15,11)` and `(30,11)` came back
tape-periodic with period 11, which named a family — `class(j) = g(j mod P)` is `3^P` choices
of `g` against `3^(n²)` for the grid. Searching it exhaustively settles two open cases in
**milliseconds**, where 354 million grid nodes had settled one.

My error was fixing the period at `L`. The winning periods are **9 and 11**, and 9 ≠ 8.

**And `(33,11)` is hard for a structural reason, not a search one.** A period `P` that divides
`n` makes every row start at the same phase, so every row carries an identical pattern and
every **column is constant** — all `L` cells of a column burst in one class. At `n = 33` that
rules out `P = 1, 3, 11, 33`, **including the period 11 that settles its two siblings.**
`33 = 3 × 11` while `30` is not a multiple of 11: that is the whole difference. It is reported
INCONCLUSIVE and never as an impossibility — a randomised search and a bounded periodic family
can only ever say YES.

### 3. The safety fix, shipped

v6's C6 failed: raising `erasures_per_class` alone produced 2 silent wrong answers in 100.
v6's answer was `Caps::raised`, which **calibrates** the coupled budget — the weaker fix,
because it needs `p`, `f` and a margin to be right.

**v7's fix is to make truncation unforgeable.** If the reading list was truncated, the decoder
cannot know whether the true reading was among the ones it discarded, so a unique survivor is
**not** evidence of uniqueness. A `truncated` flag is threaded out of the enumeration and
`Corrected` is refused when it is set. Safe at **any** cap setting, and it needs no
arithmetic.

| caps | corrected | ambiguous | refused | **wrong** |
| --- | --- | --- | --- | --- |
| v0, untouched | 0 | 0 | 100 | 0 |
| **LOPSIDED per-class 20, hits 64 — v6's row** | 46 | 3 | 49 | **2** |
| **the same, + guard** | **0** | **51** | 49 | **0** |
| `Caps::raised(20)` — coupled, guard on | **95** | 5 | 0 | **0** |

> **The rule, which is the shippable half of this round:** truncating a candidate list and
> then filtering by a second check converts **detection** into **miscorrection**. The fix is
> to make the truncation visible to the caller, not to raise the budget — raising the budget
> only moves the threshold, and the failure is silent on either side of it.

## What it cost, said plainly

The guard **removes all 46 corrections** the lopsided raise was making. That is the correct
outcome — those 46 included the 2 lies and the decoder could not tell which — but it is a real
reduction, and T6 exists so that it gets printed rather than buried.

And **v0 itself never truncates** in the measured range: 0.0% at every `f` up to its own cap
of 16, where `2^16/2053 = 31.9` expected solutions sit against 64 kept. v0's margin of two is
doing exactly its job. The bug only ever existed for someone who raised the cap — which is
precisely what v6 did.

## The pins — T1, and the round does not start without it

| pin | checked | mismatches |
| --- | --- | --- |
| the copy vs v6's committed record | 13 figures | **0** |
| `region_of` vs `stalk.js`'s `regions()` | 22,139 | 0 |
| `arcs` vs `stalk.js`'s `arcs()` | 1,599 | 0 |
| the port vs eggSo-v0's structure | 6,153 | 0 |
| **the port vs eggSo-v0's decisions** | **600** | **0** |

The guard changes what the decoder does, so what has to be proved first is that it does not
change what **v0** does. `refuse_on_truncation` is `false` in `Caps::v0()` and only there. The
record pin fixes v6's **2 miscorrections** before the guard touches them.

## Two exact reductions, both pinned

- **Row windows are redundant.** When `L ≤ n`, a row window of `L` consecutive cells *is* `L`
  consecutive row-major indices, so every row constraint is already a tape constraint.
  Dropping them is exact and free — and every REACHED partition is re-verified with the row
  windows **put back in**, so the reduction cannot buy a false positive.
- **A period dividing `n` is hopeless**, by the constant-column argument above.

One efficiency note, recorded because it changed nothing but the clock: the first periodic
search rebuilt the whole `n²` grid per candidate and took **7m 9s**. Pre-filtering on the tape
constraint — necessary, and its phases genuinely cover all `P` residues — gave the same
answers in **11.7s**.

## Running it

```
cargo build --release
cargo test                                    # 69 tests
cargo clippy --all-targets -- -D warnings     # clean, no suppressions

cargo run --release -- pin      # T1; SKIPPED loudly if node is absent
cargo run --release -- thirds   # T2: the characterisation, all three residues
cargo run --release -- open     # T3: the three cases v6 left inconclusive
cargo run --release -- guard    # T4-T6: the safety fix against v6's own numbers
cargo run --release -- audit    # all of it, ~15s, writes every measured-*.json
```

## Files

```
Cargo.toml        name = "eggso7", edition 2021, NO dependencies
PREDICTIONS.md    filed first, measured column filled in afterwards
src/lib.rs        pub mod declarations
src/main.rs       pin | thirds | open | guard | audit
src/thirds.rs     NEW: the characterisation, the reductions, both searches
src/code.rs       carried from v6, plus `refuse_on_truncation` -- the guard
src/caps.rs       carried from v6
src/fold.rs       carried unchanged since v4
src/seam.rs       carried from v6
src/optimum.rs    carried from v6
src/pin.rs        carried from v6, repointed at v6's record
src/json.rs       carried unchanged
measured-*.json   what the binary wrote
```

## What this is and is not

- **It is the end of the line, and that was the point.** v6's verdict said the engineering was
  done; this round is the two items that survive it. There is no third part and no v8 implied.
- **v0's default behaviour is unchanged, permanently**, and the 600-decision pin proves it.
- **`(33,11)` is open.** Not impossible — open. The methods used here cannot say otherwise and
  the round does not pretend they can.
- **The rule is the part to take away.** The characterisation belongs to this project; the
  truncate-then-filter lesson belongs to anything that enumerates candidates under a budget
  and then confirms with a second check.
