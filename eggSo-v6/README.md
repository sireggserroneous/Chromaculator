# eggSo v6 — the caps

Not part of the site. The twenty-first codec experiment and the seventh in the fold-native
lineage — [`eggSo-v0/`](../eggSo-v0/) used the fold's partition, [`v1`](../eggSo-v1/) its
symmetry, [`v2`](../eggSo-v2/) its alphabet's slack, [`v3`](../eggSo-v3/) its radix and its
scale, [`v4`](../eggSo-v4/) named what the fold **is**, [`v5`](../eggSo-v5/) found the burst
optimum and then found that the walls are not the geometry at all. This round goes after the
walls. Kept in its own folder so it does not entangle with `chronochromatic.org`, which
claims none of this.

Rust, its own crate, empty `[dependencies]`. Built 2026-09-03 against
[PREDICTIONS.md](PREDICTIONS.md), filed before a line of the round was written.

## The verdict, first

**Six bars, five met, one missed — and the missed one is the result.**

v5 ended by naming four fixed constants inherited from eggSo-v0 as the real limits of this
construction. This round asked which of them are **artifacts** that can be raised and which
are **information bounds** that cannot. The answer is both, and one raise turned out to be
actively dangerous:

> **Raising `erasures_per_class` on its own makes the decoder lie.** It went from refusing
> 100 of 100 to answering 46 correctly, 49 refusals — and **2 silent wrong answers.**
> `erasures_per_class` and `erasure_hits` were never independent knobs. They are a matched
> pair, and v0's values are safe by exactly a factor of two.

That is C6, it was filed as a bar before any measurement, and it is **MISSED**. A raised cap
that converts a refusal into a silent wrong answer is strictly worse than the refusal. The
only reason it is a finding here rather than a shipped bug is that the `wrong` column was in
the table from the start.

## What the round bought, and what it cannot

| | |
| --- | --- |
| **the free win** | the pair cap is a pure artifact. `4096 → 16,384` takes same-class doubles at n = 512 from v5's **70 of 120 to 118 of 120**, for 1.7× the wall clock |
| **the expensive win** | 18 erasures in one class goes from **0 of 100 to 95 of 100** — but only if the caps are raised *together*, and at **79×** the wall clock |
| **the thing no cap buys** | a 96-cell burst at n = 256 is past the information bound of 68. `0 of 50`, and no budget moves it |
| **the correction to v5** | v5 called these "three fixed constants" setting "the real walls". Two of them are one constant with two halves, and the fourth wall was never a cap |

## The bound, derived before measuring

Flagged erasure recovery is not a search with a tunable budget. It is a counting problem, and
the count is fixed by the check bits.

With `f_k` flagged cells in class `k` and `F` in total: the decoder knows *which* cells are
unknown, so class `k` offers `2^{f_k}` candidate assignments and its own residue mod `p`
keeps about `2^{f_k}/p` of them. The confirming residue mod `q` is global and filters the
surviving combinations once more, by `1/q`. So the expected number of readings that satisfy
every check is `2^F/(p³q)` spread, or `2^F/(pq)` concentrated in one class — and recovery is
unique only while that stays below 1:

```
F  ≲  3·log2(p) + log2(q)   =  check_bits        (spread)
F  ≲    log2(p) + log2(q)                        (concentrated in one class)
```

At n = 32, `p = 2053` and `q = 2063`: **44.0 spread, 22.0 concentrated.**

### Both halves land on their derived value

**Spread**, bound 44.0:

| F | per class | corrected | ambiguous | refused | wrong |
| --- | --- | --- | --- | --- | --- |
| 36 | 12/12/12 | **60/60** | 0 | 0 | 0 |
| 39 | 13/13/13 | 28/30 | 2 | 0 | 0 |
| **44** | 15/15/14 | **11/30** | 19 | 0 | 0 |
| 48 | 16/16/16 | **0/15** | 15 | 0 | 0 |

**Concentrated**, bound 22.0:

| F | corrected | ambiguous | refused | wrong | µs each |
| --- | --- | --- | --- | --- | --- |
| 12 | **60/60** | 0 | 0 | 0 | 118 |
| 18 | 56/60 | 4 | 0 | 0 | 10,384 |
| **22** | **14/30** | 16 | 0 | 0 | 179,434 |
| 24 | 0/16 | 16 | 0 | 0 | 741,121 |

Both cross ~40% exactly at their bound, which is the expected-one-solution point. And the
column that makes it a *bound* rather than a budget: **`refused` is 0 in every row.** Past
the bound the decoder reports **ambiguous** — several readings satisfy every check — which is
what an information limit looks like from the inside. A budget stop prints `too many
erasures` instead, and the harness keeps the two apart precisely so this is legible.

## The miss, in full

18 erasures in one class at n = 32. That is inside the bound of 22, so every failure here is
a budget and not the arithmetic:

| caps | corrected | ambiguous | refused | **wrong** |
| --- | --- | --- | --- | --- |
| v0, untouched | 0 | 0 | **100** | 0 |
| per-class 16 → 20, hits still 64 | 46 | 3 | 49 | **2** |
| hits 64 → 4096, per-class still 16 | 0 | 0 | 100 | 0 |
| both: per-class 20, hits 4096 | **95** | 5 | 0 | 0 |
| `Caps::raised(20)` | **95** | 5 | 0 | 0 |

**The mechanism.** The erasure path enumerates the `2^f` subsets of a class's flagged cells
and keeps those matching the class residue — about `2^f/p` of them — but it stops collecting
at `erasure_hits`, and *then* asks `q` which kept reading survives. Truncate that list and
the **true** reading can fall off it, leaving a false one as the unique survivor. The decoder
commits to it.

At `f = 18` there are `2^18/p ≈ 119` expected solutions against v0's 64 kept — the list is
short by half. **v0's own pair is safe by exactly a factor of two: `2^16/2053 = 31.9` against
64.** So v0's constants are not arbitrary and not independent; they are a matched pair whose
ratio *is* the safety margin.

`Caps::raised(f, code)` now does the coupling arithmetic, `Caps::hits_sufficient` checks a
hand-built set, and a test pins both the lie and its absence so no later round can raise one
without the other by accident.

## The price of each win

The erasure enumeration is `2^f` per class, so that cap is not a knob — it is an exponent:

| f in class | corrected | µs per square | vs f = 12 |
| --- | --- | --- | --- |
| 12 | 29/30 | 140 | 1× |
| 16 | 30/30 | 2,568 | 18× |
| 18 | 29/30 | 10,998 | 79× |
| 22 | 11/30 | 185,654 | **1,326×** |

The pair cap is the opposite — nearly free, because the enumeration was always `O(|class|)`
and the cap only ever bounded the output list:

| `pair_candidates` | corrected of 120 | wrong | ms per square |
| --- | --- | --- | --- |
| **4096** (v0) | **70** | 0 | 7.9 |
| 16,384 | **118** | 0 | 13.6 |
| 262,144 | 118 | 0 | 13.1 |

16,384 is already enough and it saturates at 118 — the residual 2 are genuinely ambiguous. I
filed that cost as "negligible"; 1.7× is not negligible, and that half of the prediction
missed.

## The wall no cap can move

| n | burst | spread bound | v0 caps | generous caps | verdict |
| --- | --- | --- | --- | --- | --- |
| 33 | 12 | 44 | 50/50 | 50/50 | no wall here |
| 128 | 48 | 60 | 50/50 | 50/50 | no wall here |
| **256** | **96** | **68** | **0/50** | **absent (32/class)** | **INFORMATION** |

The generous column is left **absent** at n = 256 rather than optimistic: 96 flagged cells is
32 per class and `2^32` subsets is not computable. Substituting a shorter wound there and
reporting its success would be two experiments wearing one row. The bound argument does not
need the number — `96 > 68` settles it.

## The pins — C1, and the round does not start without it

| pin | checked | mismatches |
| --- | --- | --- |
| the copy vs v5's committed record | 19 figures | **0** |
| `region_of` vs `stalk.js`'s `regions()` | 22,139 | 0 |
| `arcs` vs `stalk.js`'s `arcs()` | 1,599 | 0 |
| the port vs eggSo-v0's structure | 6,153 | 0 |
| **the port vs eggSo-v0's decisions** | **600** | **0** |

The caps are a parameter now, so the first thing that has to be proved is that the **default
is still v0 to the decision** — otherwise every raised-cap number would be measuring my own
edit. 600 squares are decoded by both v0's own decoder through node and this one at
`Caps::v0()`, compared on the status word and the repaired cells. The record pin includes
v5's `70 of 120` at n = 512, the one figure this round exists to move, fixed before any cap
is touched.

## Running it

```
cargo build --release
cargo test                                    # 59 tests
cargo clippy --all-targets -- -D warnings     # clean, no suppressions

cargo run --release -- pin      # C1; SKIPPED loudly if node is absent
cargo run --release -- bound    # C2: the information ceiling, derived and measured
cargo run --release -- caps     # C3-C5: each cap isolated, with its cost
cargo run --release -- audit    # all of it, ~45s, writes every measured-*.json
```

## Files

```
Cargo.toml        name = "eggso6", edition 2021, NO dependencies
PREDICTIONS.md    filed first, with the measured column filled in afterwards
src/lib.rs        pub mod declarations
src/main.rs       pin | bound | caps | audit
src/caps.rs       NEW: the harness that flags an exact count in an exact distribution
src/code.rs       carried from v5; the four caps are a PARAMETER defaulting to v0's,
                  plus Caps::raised and the safety invariant the round found
src/fold.rs       carried unchanged since v4
src/seam.rs       carried from v5
src/optimum.rs    carried from v5
src/pin.rs        carried from v5, repointed at v5's record; the cellOrder pin dropped
src/json.rs       carried unchanged
measured-*.json   what the binary wrote
```

v5's `cubic.rs` and `dynamics.rs` are deliberately **not** carried: the degree-3 coordinate
was v5's Part 1 and nothing here needs it. Copying a module forward has a price — a copy can
drift silently — so the rule is to copy only what the round uses, and pin every copy.

## What this is and is not

- **It is not a compressor and not armour.** Settled across the whole series.
- **v0's default behaviour is unchanged, permanently.** The caps default to v0's values and
  the decisions pin proves it. Nothing here is a patch to a shipped round.
- **The expensive win is probably not worth shipping.** 0 → 95 of 100 at 79× the wall clock
  buys one channel; whether that trade is ever right is a question for whoever needs it, not
  a recommendation from here.
- **The safety finding is worth shipping.** If any future round raises `erasures_per_class`,
  it must go through `Caps::raised`, and the test says so in a way that fails loudly.
