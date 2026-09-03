# eggSo v3 predictions — filed 2026-09-02, BEFORE a line of the codec was written

The series convention: every number below is a guess, written down first, and the
measured value is filled in beside it afterwards or not at all. Misses stay. A
prediction that is quietly edited to match its result is worth less than no prediction —
and, since v0's amendment, so is a measurement.

Two arms and a ring, each built and measured. Bare `v3` means eggSo-v3.

## What this is, in one sentence

Everything the fold-native lineage has built lives **inside one square, one bit to a
cell**. This round changes both: the cell's alphabet (bit, nibble, byte) and the scale
the fold acts on (the whole file, not the square) — and then puts every arm of the
lineage in a **ring** against real files and real injuries.

## Why this round exists

Three things the first three rounds never did.

1. **Every cell has held one bit.** v0, v1 and v2 all compared overheads at N = 32 bits
   and called them the codec's cost. They are the *block's* cost. A corrupted byte — the
   unit real storage actually loses — has never been a channel; in a bit square it is
   eight cells in one row and no arm in the lineage corrects it.
2. **Every construction has stopped at the square's edge.** v1(c)'s verdict says it
   plainly: "σ never leaves its square." Real damage does. A 4 KB scratch is 32 whole
   blocks; a truncation removes blocks outright. The fold has never been applied to the
   file.
3. **Nothing has been in a ring.** The codegg line has a tournament with a rule —
   wrong-or-no data after any injury forfeits, smallest survivor wins. The eggSo line has
   synthetic channels on random squares. The two shipped candidates (v0 with the
   amendment at 4.70%, v1(a) at 5.88%) have never been asked to hand back a real file.

## The construction, stated before building

**Arm (a), the radix arm.** v0's codec with the cell's alphabet as a parameter. A cell
holds a digit in `0..A−1`; the square's value is `V = Σ cell_i · A^(L−1−i)`; an error `d`
at cell `i` moves it by `d·A^(L−1−i)`, and one prime injective over
`{d·A^k : d ∈ ±1..A−1, k < L}` names the cell, the sign and the size. Three region
residues plus the confirming `q`, exactly as v0, with the amendment's per-candidate
confirm on by default. Erasures at known positions are **solved, not enumerated**: a
region with one flagged cell reads `v = Δ/w[i] mod p` directly, which is the only path
open at `A = 256` where enumeration is `256^k`.

**Arm (b), the file-scale fold.** The checks stay on the source file's blocks; the
*artifact* stores those bytes in anti-transpose order over the whole file:

```
artifact[k] = source[σ(k)],   σ(j) = M − 1 − ((j mod n)·n + ⌊j/n⌋),  n = ⌈√M⌉
```

σ is an involution, so the encoder is the decoder and there is no table. A contiguous
wound in the artifact is therefore a σ-scattered set of source positions: the damage a
block sees is thin instead of total, and its positions are known, so it arrives as
erasures. Truncation becomes the same thing as a scratch, given the original length.

## Measured during planning — ground, not predictions

Computed before any arm was built; `makeCode` recomputes each and the suite asserts it.

**The moduli and what a block costs.** `p`, `q` are the first two primes injective over
`{d·A^k : d ∈ ±1..A−1, k < L}`, found by search and verified by enumeration:

| A | N | L | block | p | q | bits(p) | 4 residues | overhead |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2 (bit) | 32 | 1024 | 128 B | 2,053 | 2,063 | 12 | 48 b | **4.688%** (v0's) |
| 16 (nibble) | 16 | 256 | 128 B | 17,627 | 19,429 | 15 | 60 b | **5.859%** |
| 16 | 32 | 1024 | 512 B | 61,381 | 65,831 | 16 | 65 b | **1.587%** |
| 256 (byte) | 16 | 256 | 256 B | 2,265,761 | 2,288,267 | 22 | 88 b | **4.297%** |
| 256 | 32 | 1024 | 1024 B | 8,030,879 | 8,035,021 | 23 | 92 b | **1.123%** |

Two facts fall straight out and neither was known to the lineage before today:

- **At equal block size a bigger alphabet is worse.** 128 B costs 4.69% in bits and 5.86%
  in nibbles. The prime has `2(A−1)L` values to keep apart, and it must also dodge every
  ratio `d₂/d₁`, so it runs far past the naive bound: at `A = 256, L = 256` the estimate
  is 130,560 and the prime is **2,265,761**, seventeen times larger.
- **Overhead is the block's property, not the codec's.** It is `4·⌈log₂p⌉ / (L·log₂A)`,
  and `log₂p` grows logarithmically while the block grows linearly. Every overhead number
  the first three rounds compared — 2.34%, 4.69%, 5.86%, 103% — was measured at one
  design point, 128 B, and the design point was never the variable.

**Where a corrupted byte lands.** In a 32×32 bit square a byte is 8 consecutive cells in
one row; of the 128 bytes in a block, **32 straddle a region boundary and 32 touch the
Fold**. In a 16×16 nibble square a byte is 2 adjacent cells in one row; **16 straddle, 16
touch the Fold**. In a byte square a byte is **one cell**.

**What σ does to a 4 KB wound at file scale**, per 128-byte block:

| file | bytes | blocks | blocks touched | worst block |
| --- | --- | --- | --- | --- |
| `spec.md` | 15,190 | 119 | 119 | **37 bytes** |
| `stalk.js` | 19,567 | 153 | 153 | 30 |
| `og.png` | 33,742 | 264 | 206 | 23 |
| `wubbadub.html` | 92,408 | 722 | 304 | 14 |
| `notepad.exe` | 360,448 | 2,816 | 627 | **7** |
| `archive.zst` | 946,623 | 7,396 | 997 | **5** |

Unscattered, the same wound destroys 32 blocks completely. Scratch and truncation give the
same table to within one byte. The density is `4096 · blockBytes / fileBytes`, so **the
threshold is file size**, and it is arithmetic, not a guess.

## THE BARS

### Arm (a) — radix

| bar | needed to count as met |
| --- | --- |
| **A1** the moduli | `makeCode` recomputes the five (p, q) pairs above exactly, and the injectivity is re-verified by enumeration, not taken on faith |
| **A2** round-trip | exact at every radix, 7 shapes, including a partial last block |
| **A3** singles | one corrupted **digit**, 3000 trials, every radix: 3000/3000, all direct, 0 wrong |
| **A4** the realistic injury | one corrupted **byte**: **byte square 3000/3000 direct**; nibble square ≥ 95%; bit square measured and expected near 0, all three with **0 miscorrected** |
| **A5** erasure solving | one flagged cell per region solved exactly, no enumeration, at every radix; two per region refused |
| **A6** cost | the table above, reproduced from the format |
| **A7** the name | an AN / residue arithmetic code in radix A, interleaved by the fold's partition — v0's name with the radix named |

**MET** if A4's byte square lands 3000/3000 with 0 wrong. **MISSED** if any radix
miscorrects a single, or if the byte square cannot name a corrupted byte.

### Arm (b) — the file-scale fold

| bar | needed to count as met |
| --- | --- |
| **B1** involution | `encode∘encode = id` exactly, every file, every length; no table |
| **B2** the scatter | the measured worst-block density matches the ground table to the byte |
| **B3** the threshold | a 4 KB scratch is corrected exactly when the scatter leaves each region ≤ its solvable count, and the file-size threshold is stated in bytes and then measured |
| **B4** truncation | with the original length carried, a 4 KB truncation is corrected on the same files a scratch is — the two become one injury |
| **B5** cost | σ costs **no bits at all** beyond the length header; the artifact is the same size as the source |
| **B6** the name | a whole-file interleaver concatenated with a per-block AN code; the involution makes encoder and decoder the same map |

**MET** if B1 and B4 hold and B3's threshold is measured. **MISSED** if σ-scattered damage
is not correctable on *any* corpus file — that would mean interleaving buys nothing here.

### The ring

| bar | needed to count as met |
| --- | --- |
| **R1** the rule | codegg-v12's, unchanged: wrong-or-no data after any injury forfeits; a forfeit is loud and keeps its size |
| **R2** the injuries | 1-byte flip (blind), 4 KB scratch (addressed), 4 KB truncation — plus **1 flipped bit**, which is the injury this lineage was built for and the codegg ring never had |
| **R3** honesty | every arm's forfeits reported, including both shipped candidates; no arm is excused for being ours |

## Calibration, stated before the numbers

I expect the **byte square to win the round**: a corrupted byte is the injury that
happens, and only a byte square names it. I expect the bit square — every arm of v0, v1
and v2 — to **forfeit the byte flip**, which would mean the lineage's three rounds have
been correcting an injury (a single flipped bit) that storage rarely delivers alone, and
missing the one it does. I expect σ at file scale to hold B1 trivially and to be
**decided by arithmetic already in the ground**: correctable on `notepad.exe` and
`archive.zst`, hopeless on `spec.md` and `stalk.js`, with `og.png` and `wubbadub.html`
the interesting middle. I expect the ring to end with **no eggSo arm surviving all four
injuries** unless σ carries the two big files, and I expect codegg-v1 to forfeit the byte
flip too. The bar I am least sure of is A4's nibble square: a byte there is a same-region
adjacent pair, which is exactly what v0's amended search is good at, but adjacency is
also what killed v2(b) — `d·A^k` aliases across neighbouring cells the way `2·2^k` did.

## Per-stage predictions

### S1 — the codec, every radix

| claim | predicted |
| --- | --- |
| the five (p, q) pairs, recomputed | exactly the ground table — a miss is a bug |
| injectivity re-verified by enumeration | holds at every radix |
| round-trip, 7 shapes, 5 configurations | exact |
| table build time, A = 256 N = 32 (≈ 760k entries) | under 5 s |
| `I + F + O ≡ V (mod p)` at every radix | holds |

### S2 — singles and the corrupted byte (A3, A4)

| channel | bit N=32 | nibble N=16 | byte N=32 |
| --- | --- | --- | --- |
| one corrupted digit, 3000 | 3000 direct | 3000 direct | 3000 direct |
| **one corrupted byte**, 3000 | **~0 corrected, 0 wrong** (8 cells in one row, one region) | **≥ 95%**, called 97%, 0 wrong (an adjacent same-region pair, v0's amended search) | **3000/3000 direct, 0 wrong** |
| three corrupted bytes, one per region, 3000 | ~0 | ~90% | **3000/3000 direct** |
| one flagged byte-erasure per region, 3000 | solved | solved | solved |
| two flagged in one region | refused, 0 wrong | refused or enumerated | refused, 0 wrong |

### S3 — the file-scale fold (B1–B4)

| claim | predicted |
| --- | --- |
| `encode∘encode = id`, 7 file lengths | exact, 0 bytes of overhead |
| worst-block density, 6 corpora | the ground table, to the byte |
| 4 KB scratch, byte square N=32 (1 KB blocks), correctable | needs ≤ ~1 damaged byte per region per block. Density is `4096·1024/F`, so the threshold is about **F ≥ 1.4 MB** for the byte square and **F ≥ 175 KB** for a bit square with 3 solvable per block. Predicted: **`archive.zst` corrected, `notepad.exe` borderline, everything smaller forfeits** |
| 4 KB truncation with the length carried | the same result as the scratch, file for file |
| the same injuries without σ | every arm forfeits every file — 32 blocks destroyed whole |

### S4 — the ring

400-trial channels are gone; this is one file, one injury, one verdict, codegg-v12's rule.

| contender | 1 bit flipped | 1 byte flipped | 4 KB scratch | 4 KB truncation |
| --- | --- | --- | --- | --- |
| codegg-v1 | EXACT | **forfeit** | forfeit | forfeit |
| eggSo-v0 + amendment (4.70%) | EXACT | **forfeit** | forfeit | forfeit |
| eggSo-v1(a) (5.88%) | EXACT | **forfeit** | forfeit | forfeit |
| v3(a) byte square, N=32 (1.12%) | EXACT | **EXACT** | forfeit | forfeit |
| v3(a) byte + v3(b) σ | EXACT | EXACT | **EXACT on ≥ 1.4 MB** | **EXACT on ≥ 1.4 MB** |

**Who the house is predicted to keep:** the byte square, on cost *and* on the injury that
happens — and the round's sentence is predicted to be that **the lineage spent three
rounds tuning the decoder and the free variable was the cell.**

## The bar arithmetic, filed plainly

| bar | needs | call |
| --- | --- | --- |
| A1, A2, A6 | exact | **YES** |
| A3 | 3000/3000 every radix | **YES** |
| A4 | byte square names a byte | **YES**; nibble ~97% is the uncertain one |
| A5 | solve, not enumerate | **YES** |
| A7 | a name | **YES**: AN code in radix A, fold-interleaved |
| B1, B5 | exact, free | **YES** — σ is an involution and stores nothing |
| B2 | matches ground | **YES** |
| B3 | threshold measured | **YES**, and predicted to exclude four of six corpora |
| B4 | truncation = scratch | **YES**, given the length |
| B6 | a name | **YES**: interleaver ⊗ AN, self-inverse |
| R1–R3 | the rule, the injuries, the forfeits | **YES** — and most of the lineage is predicted to forfeit |

## Measured (filled as stages land — never before)

Filled 2026-09-02, after `tools/eggso3.test.js` and `tools/ring.js --json`. Every number
here is from those runs; `measured-*.json` beside this file is what they wrote.

### S1 — the codec at every radix: HELD, with one defect found in the ground

| claim | called | landed |
| --- | --- | --- |
| the five (p, q) pairs, re-derived by search | the ground table | **exactly**, all five |
| injectivity re-verified by enumeration | holds | **holds**, all five |
| round-trip | exact | **exact, 70 shapes** (5 radices × σ on/off × 7 lengths) |
| table build, A = 256 N = 32 | under 5 s | the search is 19 s and the **primes are cached in the source**; the suite re-derives them by search every run, so the cache cannot drift. Build from the cached prime: **under 100 ms** |
| the naive bound vs the prime | — | at A = 256, N = 16 the bound `2(A−1)L` is 130,560 and the prime is **2,265,761 — 17.4×** |

### S2 — singles and the corrupted byte: A4 MET, and the miss is the round

| channel | called | landed |
| --- | --- | --- |
| one corrupted digit, 3000, every radix | 3000/3000 direct | **3000/3000, 0 wrong, all five** |
| one corrupted **byte**, byte/1024B | 3000/3000 direct | **3000/3000, 0 wrong** |
| one corrupted **byte**, byte/256B | — | **3000/3000, 0 wrong** |
| one corrupted **byte**, nibble/128B and /512B | ≥ 95%, called 97% | **2997/3000 and 2973/3000**, 0 wrong — better than called |
| one corrupted **byte**, bit/128B | "~0 corrected, **0 wrong**" | **604 corrected, 2353 detected, 43 MISCORRECTED** — MISSED on both halves |

**The miss that matters.** `bit/128B` is not a straw man: it is eggSo-v0's code exactly —
same regions, same primes 2053 and 2063, same three residues and confirm — and it is the
square v0, v1 and v2 all ran on. On the injury real storage delivers it corrects one in
five and **hands back wrong data 43 times in 3000, 1.4%**. Three rounds of this lineage
measured miscorrection at 0 on every channel they ran, and none of them ran this one.

### S3 — erasures: A5 MET, and better than called

| channel | called | landed |
| --- | --- | --- |
| one flagged cell per region, every radix | solved, no enumeration | **1000/1000 solved**, all five |
| two flagged in one region | refused, 0 wrong | **1000/1000 recovered**, 0 wrong — MISSED on the good side. `A²` is 65,536 at A = 256, which is cheaper to enumerate than to refuse, and `q` settles it |

### S4 — the file-scale fold: B1 MET, B2 corrected the ground, B3 MISSED

| claim | called | landed |
| --- | --- | --- |
| `σ∘σ = id`, cost | exact, 0 bytes | **7/7 lengths, 0 bytes** — the artifact is the source plus checks plus a 10-byte header |
| worst-block density | the ground table, to the byte | **39 / 30 / 23 / 14** where the ground said 37 / 30 / 23 / 14 |
| a 4 KB scratch, correctable on `archive.zst` with σ | **YES** | **NO — every row forfeits 4096 bytes on every file.** MISSED |
| a 4 KB truncation | same as scratch | **same as scratch: forfeited by everyone** |

**The defect in the ground, kept:** the planning script's σ *dropped* partners that fell
past the end of the file. That is not a permutation. `fileSigma` fixes them in place, so
the map is an involution on `0..M−1` and a few more bytes stay where they were —
`spec.md`'s worst block is 39, not 37. Only the codec's σ is a permutation; the ground
table is wrong by two bytes on the smallest file and right elsewhere.

**Why B3 missed, in one line:** the threshold arithmetic filed above counted damaged bytes
per *block* and forgot that one equation per region solves one unknown **per region**. A
4 KB wound scattered into `archive.zst`'s 1 KB blocks leaves ~4 bytes per block, which is
one or two per region — and the blocks that catch three refuse. The fix is not a bigger
file; it is **more equations per region**, which is Reed–Solomon, and is the next round's
opening, not this one's result.

### S5 — the capacity curve, added after the 4 KB row came back all-forfeit

"Nobody survives 4096 bytes" is true and useless. The number that says what σ is worth is
the **largest contiguous scratch survived exactly**, by bisection. The expectation was
filed in `tools/ring.js` before it was run: about 3 bytes bare, independent of file size;
hundreds to low thousands with σ, rising with the file.

| row | artifact | spec.md | stalk.js | og.png | wubbadub | program.exe | notepad.exe | archive.zst |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bit/128B | 104.69% | 2 | 1 | 1 | 1 | 2 | 2 | 1 |
| **bit/128B + σ** | 104.69% | **61** | **141** | **185** | **306** | **515** | **602** | **974** |
| nib/512B | 101.76% | 2 | 2 | 2 | 4 | 4 | 4 | 3 |
| nib/512B + σ | 101.76% | 3 | 2 | 4 | 306 | 517 | 601 | 975 |
| byte/256B | 104.30% | 2 | 2 | 2 | 2 | 4 | 4 | 3 |
| byte/1KB | 101.17% | 2 | 2 | 2 | 5 | 4 | 4 | 3 |
| **byte/1KB + σ** | **101.17%** | 2 | 2 | 5 | 306 | 520 | 604 | **1946** |

Bare capacity is 1–5 bytes at every block size and every file size, exactly as called. σ
multiplies it by **31× to 974×**, costs zero bytes, and its gain scales with the number of
blocks — which is why `bit/128B + σ` beats `byte/1KB + σ` on the small files (119 blocks
against 15) and loses to it on the big ones.

### S6 — the ring (`tools/ring.js`)

Seven rows, seven files, four injuries, codegg-v12's rule. Sizes are the whole artifact.

| row | artifact | 1 bit | 1 byte | 4 KB scratch | 4 KB trunc |
| --- | --- | --- | --- | --- | --- |
| bit/128B — *eggSo-v0's code exactly* | 104.69% | EXACT | detected ×5, **WRONG on notepad.exe**, EXACT ×1 | detected | detected |
| bit/128B + σ | 104.69% | EXACT | EXACT ×6, detected ×1 | detected | detected |
| nib/512B (+σ) | 101.76% | EXACT | **EXACT everywhere** | detected | detected |
| byte/256B | 104.30% | EXACT | **EXACT everywhere** | detected | detected |
| byte/1KB (+σ) | **101.17%** | EXACT | **EXACT everywhere** | detected | detected |

Called and landed, row by row: the bit square forfeiting the byte flip — **called, and it
did worse than called, by lying once**. Every radix ≥ 16 taking it — called. Nobody taking
the 4 KB injuries — **not** called; σ was predicted to carry the two big files and did not.
codegg-v1 was predicted to forfeit the byte flip too; it is not in the ring, and the
README says why rather than counting a row that was never run.

### S7 — the implementation audit, 2026-09-03, after the round was committed

Not a bar. The round was reviewed against its own artifact rather than against
synthetic squares, and three defects came out. All three are fixed, and the numbers
above that moved are restated here rather than edited in place.

| what was audited | found | fixed |
| --- | --- | --- |
| **damage to the header** | the ten header bytes were naked. Flipping the low bit of the declared **length** returned a file one byte short that passed every residue check **silently**; flipping a high bit of the length threw `Invalid array length` out of the decoder instead of refusing | header is now 12 bytes with a Fletcher-16 over the other ten, and every field is bounds-checked before it sizes an array. **All 96 single-bit flips of the header now refuse cleanly: 0 silent, 0 throws** |
| **damage to the check area** | already safe, and worth recording: flipping **every one of the 180 check bytes**, one at a time, leaves the data exact 180 times, 0 silently wrong. A corrupted check cannot manufacture a repair that also satisfies `q` | nothing to fix |
| **a wound straddling checks and payload** | detected, not silently wrong | nothing to fix |
| **erasure ambiguity** | a region with two readings returned `ambiguous` immediately, which threw away the whole point of carrying `q`. Safe, but it cost capacity | candidate lists per region are now combined and `q` picks among them, which is eggSo-v0's own shape (`eggso.js:158-188`) |

**What the erasure fix moved.** The capacity curve roughly doubled, and the ordering
changed: the bit square, which has the most blocks to scatter into, now ties the byte
square on the largest file instead of losing to it.

| row | spec.md | stalk.js | og.png | wubbadub | program.exe | notepad.exe | archive.zst |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bit/128B | 3 | 4 | 2 | 2 | 4 | 4 | 3 |
| **bit/128B + σ** | **248** | **280** | **300** | **610** | **1030** | **1203** | **1946** |
| nib/512B + σ | 3 | 2 | 185 | 608 | 1030 | 1203 | 1946 |
| byte/1KB | 2 | 2 | 2 | 5 | 4 | 4 | 3 |
| byte/1KB + σ | 2 | 2 | 5 | 306 | 520 | 604 | **1946** |

Everything else re-ran unchanged: the moduli, the 70 round-trip shapes, singles
3000/3000 at every radix, the corrupted byte 3000/3000 for the byte square and 604 with
43 miscorrections for the bit square, and the ring's verdicts including `bit/128B`
returning WRONG data on `notepad.exe`. The header is two bytes larger, so every artifact
in the ring is two bytes larger; the overhead column moves by 0.01 point at most.

### S8 — original against recipe, and what it means for the transmuter line

Asked after the round: **how big is the recipe next to the file?** Every eggSo artifact is
*larger* than its source, because eggSo has no compression stage at all. It is armor.

| file | original | recipe, byte/1KB | added | recipe, bit/128B — v0's square |
| --- | --- | --- | --- | --- |
| spec.md | 15,190 | 15,382 | **+192 B, 1.26%** | 15,916, +4.78% |
| stalk.js | 19,567 | 19,819 | +252 B, 1.29% | 20,497, +4.75% |
| og.png | 33,742 | 34,150 | +408 B, 1.21% | 35,338, +4.73% |
| wubbadub.html | 92,408 | 93,512 | +1,104 B, 1.19% | 96,752, +4.70% |
| program.exe | 265,216 | 268,336 | +3,120 B, 1.18% | 277,660, +4.69% |
| notepad.exe | 360,448 | 364,684 | +4,236 B, 1.18% | 377,356, +4.69% |
| archive.zst | 946,623 | 957,735 | **+11,112 B, 1.17%** | 991,011, +4.69% |

So eggSo can never win the codegg tournament, whose rule is *smallest lossless survivor*
and whose contenders compress. What it can do is be the **armor layer** in the hybrid
posture, and there the comparison is against codegg-v6's ribs on the same file:

| file | eggSo-v3 byte/1KB | codegg-v6 ribs | eggSo saves |
| --- | --- | --- | --- |
| spec.md | +192 B, 1.26% | +5,958 B, **39.22%** | 5,766 B |
| notepad.exe | +4,236 B, 1.18% | +26,840 B, 7.45% | 22,604 B |
| archive.zst | +11,112 B, 1.17% | +69,009 B, 7.29% | **57,897 B** |

**Six to thirty-one times cheaper, and correspondingly weaker.** codegg-v6's ribs restore
a 4 KB scratch and a 4 KB truncation; eggSo-v3 restores a corrupted byte anywhere, and
with σ a wound of up to 1,946 bytes on a megabyte file. It is the right armor for bit rot
and the wrong armor for a lost sector, and the two are not interchangeable at any price.

## THE CLOSING AUDIT — every bar, called vs landed

| bar | called | landed | verdict |
| --- | --- | --- | --- |
| A1 the moduli | recomputed exactly, verified by enumeration | five for five, both ways | **MET** |
| A2 round-trip | exact | 70 shapes | **MET** |
| A3 singles | 3000/3000 every radix | 3000/3000 × 5, 0 wrong | **MET** |
| A4 the realistic injury | byte 3000/3000, nibble ≥ 95%, bit ~0 and **0 wrong** | byte **3000/3000**, nibble 2997 and 2973, bit 604 with **43 WRONG** | **MET for the byte square; the bit square's "0 wrong" MISSED** |
| A5 erasure solving | solved; two per region refused | solved 1000/1000; two per region **recovered** 1000/1000 | **MET**, better than called |
| A6 cost | the ground table | 4.69 / 5.86 / 1.59 / 4.30 / 1.12 % | **MET** |
| A7 the name | AN code in radix A, fold-interleaved | holds | **MET** |
| B1 involution, free | exact, 0 bytes | 7/7, 0 bytes | **MET** |
| B2 the scatter | matches ground to the byte | matches on 3 of 4; the **ground was wrong** on the fourth | **MET, ground corrected** |
| B3 the threshold | 4 KB correctable on ≥ 1.4 MB | **nothing survives 4 KB**; capacity is 1–5 B bare, 61–1946 B with σ | **MISSED**, and the curve replaces it |
| B4 truncation = scratch | yes, given the length | **yes** — same capacity, file for file | **MET** |
| B5 σ costs nothing | 0 bytes | 0 bytes | **MET** |
| B6 the name | interleaver ⊗ AN, self-inverse | holds | **MET** |
| R1–R3 the ring | the rule, four injuries, honest forfeits | run; every row forfeits something; one row **lied** | **MET** |

Fourteen bars, twelve met, one met only after the ground it rested on was corrected, one
missed outright — and the missed one produced the round's most useful number, the capacity
curve. **The house keeps `byte/1KB + σ`:** the smallest artifact in the ring at 101.17%,
the only kind of square that names a corrupted byte, and the largest wound survived on
every big file. It does not survive 4 KB, and neither does anything else here.

The sentence the round was for: **the lineage spent three rounds tuning the decoder, and
the free variables were the cell and the scale.** One bit to a cell was inherited from
codegg-v1's first line and never questioned; it costs 4.69% where a byte costs 1.12%, and
it turns the commonest injury into a 1.4% chance of silent corruption. The fold's own
involution, applied to the file instead of the square, multiplies the survivable wound by
up to 974× and costs nothing at all.
