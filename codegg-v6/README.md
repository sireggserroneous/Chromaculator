# codegg v6 — the Wub reading

The last unmined page of the site was **Wub**: phasors summed tip to tail, one closed
curve carried by many rotating parts — and enough phasors recover the whole curve. Read
as coding theory, that is **evaluation**: a group of squares is a polynomial, redundancy
is extra evaluations of it, and any enough-sized subset re-interpolates the rest. Which
is **Reed–Solomon** — the industrial tool this series benchmarked against all along. The
Inspirations page's sentence closes the arc: *the rooms were furnished when we got
there*, and the final room was RS.

v6 replaces v5.2's single XOR parity square with **T Reed–Solomon parity squares per
group over GF(256)** (`--parity`, default 2; `--group`, default 32). Cost stays ~9%;
contiguous capacity stays ~file/16; and the group now survives **any T dead squares**,
which kills v5.2's one documented caveat. Measured on the 9.5 MB database:

```
twin 4 KB wounds spaced exactly one stripe apart (same groups hit twice):
  T=1 (v5.2 scheme):  NOT recovered  -- the documented caveat, reproduced
  T=2 (v6):           EXACT, known-location AND blind
```

All v5.2 regressions hold at T=2: edge shapes, round trips, payload scratches 4 KB–512 KB,
check-table and head scratches — 2/2 modes EXACT each, 138 MB/s encode. Everything below
this line is the v5.2 story, kept because each rung was bought with a measured failure.

(The v5.0 -> v5.2 lineage and its audits live in ../codegg-v5/README.md.)
