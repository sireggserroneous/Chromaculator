> Superseded by ../codegg-v6 -- the Wub reading (Reed-Solomon square scale).
> This folder keeps the v5.0 -> v5.2 story and its audits, per series convention.

# codegg v5.2 — protection with shape

Third build of the series' final codec, and the one the 256 MiB drills demanded. The
lineage: v5.0 combined the series' winners; its own audit convicted it; v5.1 fixed the
convictions; the sizeable-file drills then convicted v5.1 in turn. v5.2 is the answer to
*those* lessons, and its design instruction came from the site itself: the landing page
says **numbers have a shape**, and the Spectrometer shows one number as nested readings —
stalk, square, regions — one thing at every scale at once. v5.1 protected at exactly one
scale, the bit inside a square. That was the whole diagnosis.

## v5.1's convictions at 256 MiB, and what fixed each

| conviction (measured) | v5.2's answer |
| --- | --- |
| 16 MiB wound hopeless: 64 erasures/square vs cap 20 — all protection lived at the bit scale | **a second scale**: every 16 squares fold into one parity square (cell-wise XOR — the sum of the rack, one level up); any one dead square per group is rebuilt from its siblings |
| 1 MiB truncation refused while raw kept 99.6% | truncation is not special: pad back to geometry, call the missing tail a wound, let the parity rebuild it |
| ~5 MB/s encode — 2 billion random single-bit accesses, cache death | the interleave moves from bit to **square granularity**: 128-byte moves. **169 MB/s measured — 33×** |
| (found during v5.2 itself) vdC slot order collided 810 pairs into shared groups on tail truncation | **group striping**: stored slot `j` serves group `j mod nGroups` — any contiguous run ≤ nGroups slots hits each group at most once, a pigeonhole guarantee, not a low-discrepancy promise |
| (found during v5.2 itself) random wound-fill fakes 1–2-bit repairs (~2.4·10⁻⁴/square) and poisons rebuilds | a three-rung retry ladder: full → no doubles → **parity-only** (trust no bit fix; residues judge, parity rebuilds, hash certifies) |

## The shape, spelled out

```
BIT SCALE     per-square residues (V mod 2053 / 2063, codegg-v1's move):
              repair 1-2 flipped bits locally; SELF-DIAGNOSE bad squares
              (miss ~2.4e-7) so nothing above needs a map
SQUARE SCALE  1 parity square per 16 (the fold, one level up): rebuild any
              one dead square per group, blind
FILE SCALE    3 headers at far-apart offsets, byte-voted; FNV-64 hash that
              catches lying repairs and drives the retry ladder
```

The check table is itself a small file, so it is protected by **the same two-scale scheme,
recursively** — its pieces get their own residues and their own parity, and its checks
(the meta) ride in triplicate beside the headers. The machinery pointed at itself, one
level down. Layout: `[hdr0 meta0][CT shielded][hdr1 meta1][data shielded][meta2 hdr2]`.

## Measured, 256 MiB crypto-random file

| drill | v5.1 | **v5.2** |
| --- | --- | --- |
| encode | 52 s (5 MB/s) | **1.6 s (169 MB/s)** |
| 16 MiB scratch, location known | hopeless | **EXACT** — 131,072 squares rebuilt |
| 16 MiB scratch, **blind** | hopeless | **EXACT** — no location information at all |
| 1 MiB truncation | refused | **byte-identical**, 6,892 rebuilt, 1.7 s |

And the 9.5 MB SQLite regressions all hold: payload scratches 4 KB–128 KB, check-table
scratches, head scratches — **2/2 modes EXACT each**; edge shapes exact; 3-bit wounds
went from v5.1's honest refusal to **300/300 repaired** (the parity rebuilds the square).

## Costs and limits, on the label

**~9%** total (2.35% residues + 6.25% parity at `--group 16` + recursive CT + headers).
**Capacity: one contiguous wound up to ~file/16, blind.** Measured at the edge on 9.5 MB:
512 KB EXACT, 2 MB honest fail. `--group` trades cost against capacity linearly. Multiple
wounds spaced near a multiple of the stripe length can share groups — detected honestly.
Not compression, not novelty (RAID striping + parity, Avizienis's residues, PAR2's
posture), not tamper-proofing (FNV-64 is not a MAC).

## The scoreboard (9.5 MB SQLite, this machine)

```
format      size          of orig   enc-ms   1 byte flipped     4096 B scratch
raw           9,551,872   100.0%       0     1 byte wrong       4,077 bytes wrong
gzip          1,860,951    19.5%      92     DEAD               DEAD
zstd          1,650,973    17.3%    3899     DEAD               DEAD
brotli        1,694,362    17.7%     326     130 bytes wrong*   DEAD
xz            1,474,592    15.4%    1586     DEAD               DEAD
egg5         10,419,696   109.1%      83     EXACT              EXACT
egg5+zstd     1,801,479    18.9%    3937     EXACT              EXACT
```

*brotli decodes a flipped stream without error and returns 130 silently wrong bytes.

**The recommendation is still the hybrid**: `egg5(zstd(file))` — 18.9% of the original,
smaller than gzip, both injuries repaired, and now with v5.2's capacities: the armored
1.65 MB artifact tolerates a ~100 KB contiguous wound blind. Armor the suitcase, not the
elephant.

## Running it

```bash
cd codegg-v5 && cargo build --release          # std only, no deps, ~2 s

eggv5 encode <file> [--group 16]               # -> <file>.egg5  (~109%)
eggv5 decode <f>.egg5 [--wound s:l] [--no-doubles]
eggv5 scratch <file> [--len N] [--at payload|checks|head|end|<off>]
node codegg-v5/tools/versus5.js <file>         # scoreboard, hybrid row included
```

## The last word

Five codecs, three rebuilds of this one, and every improvement in v5.2 was purchased by a
measured failure: the audit found the naked head, the 256 MiB drills found the single
scale, the truncation test found the vdC collision, the blind drill found the fake
repairs. Nothing here is new to coding theory — striping and parity are RAID, the
residues are Avizienis 1971, the posture is PAR2's — but the method is the site's own:
draw the thing at every scale, wound it where it lives, print the failures beside the
wins, and keep the audit at the top of the README because it is the part worth copying.
The shape was the lesson. Numbers have one; so does protection.
