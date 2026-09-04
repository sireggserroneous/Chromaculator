# codegg v3 — the file as a number, the number as residues

The fourth experiment, and the first true encoder in the series. v2 was a compressor at
heart — it auctioned chunks to the powers and paid literal when they lost, so everything
it promised depended on what the file happened to contain. That was gzip-shaped thinking
in the site's colours, and it was called out as such. A true encoder promises **by
construction**: noise and prose get identical cost and identical powers, because the
guarantee lives in the representation, not in the input's luck.

## The construction

Each 128-byte block — one codegg square — is a 1024-bit value `V`. The encoding is

```
V mod p₁, V mod p₂, … V mod p₇₃
```

for the 73 largest primes under 2¹⁶ (`k = 65` needed, `m = 8` spare, both tunable). The
file becomes **73 shard files**, each ¹⁄₆₄ the size of the original, each one more
world's opinion of the same number. By the Chinese Remainder Theorem, **any 65 opinions
settle it** — the product of any 65 of these primes exceeds 2¹⁰²⁴, so every block has one
address in whatever worlds survive.

This is codegg-v1's move promoted from check to representation: there, two residues
vouched for the square; here, seventy-three residues **are** the square.

## The powers — by construction, not by luck

**Any m shards may die, and it cannot matter which.** There is no header shard, no high
byte, no privileged position. A positional encoding dies by *where* it is wounded; this
one only by *how much*. Tested: 60 random 8-shard kill-sets and every single shard
excluded in turn — all exact.

**One short of k refuses loudly.** 64 shards do not produce a slightly-wrong file; they
produce an error. Never a silent wrong answer.

**Spares convict liars.** With more than 65 alive, the extras vote per block; a corrupted
shard is detected, outvoted by leave-one-out, and **named by its prime**. Tested: one
scribbled shard among 73 → blocks repaired, culprit identified, file exact.

**Arithmetic without decoding.** Residue streams add pointwise, each shard in its own
world — **carries cannot cross primes**. Shard-wise `A+B` and `3·A` reconstruct to the
exact numeric results, verified against BigInt. This is Avizienis's carry-free addition
at file scale, which is where this whole system pointed from the start.

**Structure-independence — the answer to v2.** Measured: 200,000 B of noise and 200,000 B
of prose produce byte-identical 229,512 B encodings with identical guarantees. No
auction. No gzip in the report, because size was never the claim.

## Measured at the requested size

```
random-1489k.bin: 1,489,000 B -> 73 shards, 1,699,732 B total (114.15%)
  killed by lot: shards 25, 33, 38, 47, 51, 59, 65, 67
  rejoined from the 65 survivors: EXACT   (11,633 blocks confirmed)  227 ms
```

And the same on disk, the hard way: `split` wrote 73 files, `rm` deleted 8 of them,
`join` rebuilt from the 65 left — **byte-identical**.

The cost is printed, not hidden: `(k+m)/64` of the input plus 18 B per shard header —

| m | shards | size | survives |
| ---: | ---: | ---: | --- |
| 0 | 65 | 103.3% | nothing (the CRT packing floor) |
| 4 | 69 | 109.7% | any 4 deaths |
| **8** | **73** | **114.2%** | **any 8 deaths** |
| 16 | 81 | 128.8% | any 16 deaths |

Redundancy buys survival at cost `m/64` per death survived — near the erasure-coding
line, and every percent of it is on the label.

## Running it

```bash
node codegg-v3/tools/eggshard.test.js                 # the eight claims

node codegg-v3/tools/shard.js split <file> [--m 8]   # -> <file>.shards/ (k+m files)
node codegg-v3/tools/shard.js join  <dir>  [--out f] # from whatever is there
node codegg-v3/tools/shard.js demo  <file> [--kill 8] # deaths by lot, then byte-compare
```

`join` reports blocks confirmed / repaired / condemned and names any convicted shard.
`demo` is the thesis in one command: the kill-set is drawn by lot, because the point is
that it cannot matter.

## Lineage, in the site's tradition of saying so

The residue number system is ancient — Sun Tzu's remainder problem. Redundant-RNS erasure
codes are **Mandelbaum 1976**; Reed–Solomon is the industrial cousin that does this job
over polynomial fields and does it with more machinery for error (not just erasure)
correction. This is the arithmetic-native one, chosen because the system it serves is
arithmetic-native: the same Avizienis whose 1961 digits the site credits wrote the 1971
fault-tolerant-arithmetic work this descends from. The parts are old; the fit is local.

Not claimed: security. Shards are unreadable individually but this is obfuscation, not
secrecy — the CRT-based *secret-sharing* scheme is Asmuth–Bloom, which adds randomized
masking this deliberately does not have.

## The arc of the four

| | reads the square as | redundancy | promise |
| --- | --- | ---: | --- |
| codec-v1 | a shape | 12.4% | locate one error; never lie |
| codegg-v1 | a number | 2.35% | the error spells its own address; checks survive push |
| codegg-v2 | a recipe book | −39% … +0.8% | recipes where the powers reach — honest, but input-dependent |
| **codegg-v3** | **only a number** | **+14.2%** | **any 8 of 73 worlds may end; arithmetic without decoding; structure irrelevant by design** |

v2 asked *what is this file made of?* — a compressor's question. v3 asks *what is this
file's value in 73 worlds?* — an encoder's question, and every promise it makes is kept
for every file, equally, by construction.
