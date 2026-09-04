# codegg v2 — recipes where the powers reach, literals where they do not

The third experiment, and the one the theory turns spent four rounds predicting. A file is
cut into 128-byte chunks — one codegg square each — and every chunk is **auctioned to the
site's powers**. Each power bids the exact size of the recipe it would write; LITERAL
always bids 129; the cheapest wins. Decoding replays the recipes. The container round-trips
byte-exactly in memory before a single byte reaches disk, or it is refused.

This is, named honestly, the LZ-family architecture — recipes-or-literals — with the
site's operations as the recipe language and no entropy stage. Which is why gzip is
printed in every report as the reference, and why gzip wins.

## The menu

| power | the site's word for it | what it captures | recipe |
| --- | --- | --- | --- |
| GREEN | the green cell | an all-zero chunk | 1 B |
| PREV | bar at chunk scale | chunk = previous chunk | 1 B |
| BAR | bar notation | chunk periodic, period < 126 | 2 B + one period |
| NAF | push, run the other way | run-heavy chunks, as sparse (position, sign) pairs | 2 B + 1.5 B/run-edge |
| DIV | Q/e/R | chunk = first 1024 bits of `A/B`, small `A < B` | 3 B |
| LITERAL | — | everything else | 129 B |

**Deliberately absent: multiplication.** `bits(A) + bits(B) − bits(A·B)` was measured at
exactly 0 in 61% of pairs and *positive* on average — the logarithm is additive, so the
rectangle recipe can never underbid literal. Left out by proof, not preference.

## Results

### The 1,489,000-byte corpus of known composition (`tools/mkcorpus.js`)

Six labelled regions — repo prose, repo code, zeros, a 16-byte pattern repeated,
alternating 0/1 runs, seeded noise — so the ledger is checked against ground truth
instead of admired:

```
corpus-1489k.bin: 1,489,000 B -> 910,857 B  (61.17%)   round-trip verified
gzip -9 same bytes: 478,623 B  (32.14%)  -- the reference

power     chunks   covered      spent       net
literal     6916    885248     892164     +6916   (text, code, noise: the floor)
green       2343    299904       2343   -297561   (the zeros region, exactly)
prev        1170    149760       1170   -148590   (the periodic region)
bar           32      4096       3893      -203   (accidental small periods)
naf         1171    149888      11171   -138717   (the runs region: 13.4x)
```

Every region went where the theory said it would. The chunk map (`--map`) draws it as a
strip: a mostly-literal prose country with occasional accidental bars, then unbroken
`0`s, `=`s, `s`s, and a final desert of `·`s.

### The counting argument, made flesh at the requested size

```
1,489,000 B of seeded noise -> 1,500,644 B  (100.78%)   0 recipe chunks
```

**Larger.** Every auction went to LITERAL and the container paid the opcode tax. This is
not a failure of the implementation; it is the pigeonhole principle collecting what it is
owed, at exactly the size asked for. Any encoder that shrank this file would be broken,
and the test suite asserts the growth.

### DIV, the site's own power

Given a genuine truncated rational — the first 1024 bits of `5/37` — DIV reproduces
**128 bytes from 3**. On the corpus's real prose, code, and noise it claimed **0 chunks**,
which is the earlier measurement (`0 of 17,460 grids`) reincarnated at file scale: the
win is unbounded and the data that triggers it does not occur in nature.

### Integrity, inherited from v1

`--check` rides codegg-v1's residues (`V mod 2053`, `V mod 2063`, byte-Horner — the same
value mod the same primes) on every chunk at +3 B each. The full corpus packs to 945,753 B
with checks, unpacks byte-identical from disk, 11,632/11,632 chunks verified; one flipped
container byte is flagged.

## What the ledger honestly says

- **Every byte the powers saved, RLE would have saved.** GREEN is zero-RLE; PREV and BAR
  are repetition; NAF is run-coding in signed-digit costume — its 13.4× on the runs region
  is real and is exactly what a run-length coder earns there.
- **Every byte the powers could not reach, the theory said they couldn't.** Prose and code
  sit at ~0.4–0.5 bit-density with ~450 bit-transitions per chunk; NAF needs ≤ ~84
  nonzeros to underbid literal. Noise is noise.
- **gzip wins because it has what this menu lacks**: cross-chunk matching at arbitrary
  offsets and an entropy stage. Those are Lempel–Ziv and Huffman, not chronochromatic
  powers, and borrowing them would make this a worse gzip rather than a truer instrument.

## A bug worth keeping

The first NAF bid filtered candidates by **popcount** — and run-heavy chunks have ~512
ones, so the filter rejected exactly the chunks NAF exists to win. The corpus ledger
caught it in one glance: `naf: 0 chunks` of the 150 KB laid out as its country. The filter
now counts **bit transitions** (NAF weight tracks run boundaries, not ones), and test #7
is the tombstone: a 480-ones run chunk must draw a NAF bid under 30 bytes, forever.
Ground-truth corpora exist for this reason.

## Running it

```bash
node codegg-v2/tools/eggcode.test.js               # the eight claims
node codegg-v2/tools/mkcorpus.js                   # build the labelled 1.489 MB corpus

node codegg-v2/tools/eggpack.js <file>             # pack -> <file>.egg2, ledger + gzip ref
node codegg-v2/tools/eggpack.js <file> --check --map
node codegg-v2/tools/eggpack.js <file>.egg2 --unpack
```

Feed it anything. A file with real structure — sparse images, padded records, bitmap
masks — will lose weight where the powers reach and the ledger will name each power's
take. A file that is already compressed (zip, jpeg, mp4) will come out ~0.8% larger, and
the first line of the report will say LARGER, because that is the truth.

## Files

| | |
| --- | --- |
| `eggcode.js` | encoder/decoder: the auction, the six recipes, the residues |
| `tools/eggpack.js` | CLI: pack / unpack / ledger / chunk map / gzip reference |
| `tools/mkcorpus.js` | the 1,489,000-byte labelled corpus |
| `tools/eggcode.test.js` | eight claims, including the mandatory failure on noise |

## The lineage of the three codecs

- **codec-v1** read the square as a *shape*: block parity, 12.4%, never lies.
- **codegg-v1** read the square as a *number*: arithmetic residues, 2.35%, the error
  spells its own address, checks survive push.
- **codegg-v2** reads the *file* through the square's powers: recipes where they reach,
  literals where they do not, and a ledger instead of a claim. It is the conversation's
  four rounds of impossibility theory converted into one falsifiable machine — and the
  machine agrees with the theory, byte for byte.
