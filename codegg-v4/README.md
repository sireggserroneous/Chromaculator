# codegg v4 — the Atlas permutation. Not one byte bigger.

The fifth experiment, and the answer to two corrections at once. v3 was told its +14% was
decoration, and asked *what power from the project did you use?* — a question its honest
audit half-fails: its stance (value over spelling) was the site's, but its machinery (CRT,
redundant residues) was the site's **kin**, credited lineage rather than native function.

v4's power has a file and a line number: **`bitrev` — `stalk.js:156`** — the bit-reversal
the Atlas page is built on, the van der Corput ordering `inspirations.html` credits, and
the property this conversation measured three separate times and never built.

## The construction

A pure permutation. Byte `j` of the encoding is byte `σ(j)` of the file, where `σ` walks
positions in bit-reversed order. That is the entire codec:

- **Size is exact.** Output length = input length. No header, no container, no padding —
  the transform is defined by the file's length alone, like `rev`.
- **At powers of two it undoes itself.** `encode(encode(x)) = x` — the encoder *is* the
  decoder, the way the fold map undoes itself and negation flips every colour back. The
  site's own criterion for an inversion, satisfied literally.

## The powers, measured

### Every prefix is a uniform sample of the whole

Keep the first 25% of the encoding and every 16 KB window of the original holds
**24.9%–25.0%** of its bytes (measured at 10%, 25%, 50% — all within ±0.1%). A raw file
cut at 25% holds 100% before the cut and **0%** after it. Truncation stops being
amputation and becomes resolution: the tail you lose is detail, not territory.

```
node codegg-v4/tools/atlas.js prefix spec.md --keep 25
  every 1024 B window holds 24.9%..25.0% of its bytes
  wrote spec.md.partial -- the whole file at 25% resolution
```

### Bursts become dust

A 4096-byte contiguous wound in the encoding lands on the original as:

```
4096 separate 128 B blocks touched, worst block takes 1 byte, min gap 128 B
stored contiguously, the same wound annihilates 32 whole blocks
```

Not *approximately* spread — **exactly one byte per block**, because consecutive van der
Corput positions are maximally separated. This is low-discrepancy, not luck; a random
permutation would clump.

### The armor pipeline: v4 + codegg-v1, and the burst problem dies

Both earlier codecs admitted the same weakness: a burst overwhelms any local check.
codec-v1 detected bursts and could not repair them; codegg-v1 needed the positions
flagged. The permutation converts the problem into their best case — **at zero bytes of
added cost**:

protect in the original domain (codegg-v1 residues, 2.35%), store in the Atlas domain
(bit-level permutation, so even one damaged byte scatters into eight far-apart bits).
Measured on a 1,489,000-byte file:

```
4096 B contiguous burn on the stored file
  -> scattered over 11,633 squares, worst square 3 cells (cap 16)
  -> repaired EXACT
without the Atlas: 32 squares with 1024 bad cells each -- unrepairable
```

The wound's positions are known (it is a contiguous burn), so its scattered image is a
set of known erasures, ~2.8 per square, far inside v1's 16-cell erasure capacity. One
4 KB hole in storage; the file comes back byte-identical.

## What a permutation cannot do, stated plainly

It cannot compress — it is a bijection, and the counting argument this series proved
twice applies with nothing subtracted. It cannot survive *deletion* — shortening a file
destroys information, and surviving that costs redundancy (codegg-v3's trade, which was
never decoration; it was the price of a different power). And it is not secrecy —
`bitrev` is public and keyless. What it re-encodes is **where things land**, and both of
its powers come from exactly that.

## Running it

```bash
node codegg-v4/tools/eggatlas.test.js              # the six claims

node codegg-v4/tools/atlas.js encode <file>        # -> <file>.atlas, same size
node codegg-v4/tools/atlas.js decode <file>.atlas
node codegg-v4/tools/atlas.js prefix <file> --keep 25   # the whole file at 25% resolution
node codegg-v4/tools/atlas.js burst  <file> --len 4096  # where would a wound land?
```

## Lineage

Van der Corput 1935, the bit-reversal permutation of Cooley–Tukey 1965 — both already on
the site's own inspirations page, for the Atlas. Interleaving against bursts is classical
coding practice (CD players do it with convolutional interleavers). Using a
low-discrepancy sequence as the interleaver — so the same zero-byte transform buys both
progressive prefixes and worst-case-1-per-block scatter — is the fit; as always here, the
parts are old and the fit is the local thing.

## The arc of the five

| | power used | from | size | gains |
| --- | --- | --- | ---: | --- |
| codec-v1 | the square's shape | spec.md | +12.4% | locate an error; never lie |
| codegg-v1 | place value | stalk.js:203 | +2.35% | the error spells its own address; survives push |
| codegg-v2 | push, bar, Q÷R as recipes | the pages | ±input-dependent | honest ledger; confirmed the impossibilities |
| codegg-v3 | value semantics | the site's kin | +14.2% | any 8 of 73 worlds may die |
| **codegg-v4** | **bitrev** | **stalk.js:156** | **+0.00%** | **prefixes are samples; bursts are dust; encode is decode** |

Each round was pushed by the same standard: *use the project's powers, and put the cost
on the label.* v4 is the first with nothing on the label at all.
