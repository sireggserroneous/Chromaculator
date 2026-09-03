# eggSo v3 — outside the square

Not part of the site. The seventeenth codec experiment and the fourth in the fold-native
lineage — [`eggSo-v0/`](../eggSo-v0/) used the fold's partition, [`eggSo-v1/`](../eggSo-v1/)
its symmetry, [`eggSo-v2/`](../eggSo-v2/) its alphabet's slack, and all three lived inside
one square with one bit to a cell. This round changes the cell and the scale, and then
puts the result in a ring against real files and real injuries.

Built 2026-09-02 against [PREDICTIONS.md](PREDICTIONS.md), filed before a line of the
codec was written.

## The verdict, first

**Fourteen bars, twelve met, one met only after the ground it rested on was corrected, one
missed outright — and the missed one produced the round's most useful number.**

The lineage spent three rounds tuning the decoder. **The free variables were the cell and
the scale**, and neither was ever a choice anybody made: one bit to a cell came from
codegg-v1's first line and was inherited three times.

| | bit/128 B — v0, v1, v2's square | **byte/1 KB — this round's** |
| --- | --- | --- |
| check cost | 4.69% | **1.12%** |
| one corrupted **digit** | 3000/3000 | 3000/3000 |
| one corrupted **byte** | 604/3000, **43 MISCORRECTED** | **3000/3000, 0 wrong** |
| artifact in the ring | 104.69% | **101.17%** |
| recipe on a 946 KB file | +44,388 B | **+11,112 B** |
| largest scratch survived, with σ | 1946 B | 1946 B |

And the second finding, free: **the fold's own involution applied to the file instead of
the square multiplies the survivable wound by 62× to 649× and costs zero bytes.**

The third thing to know before the tables: **every recipe here is bigger than the file it
protects.** eggSo has no compression stage. It is armor, and the cheapest armor in the
family by a factor of six to thirty-one, which is the only reason it matters to the
transmuter line at all. See *Original against recipe* below.

## What failed, first

- **The bit square hands back wrong data on a corrupted byte.** `bit/128B` is eggSo-v0's
  code exactly — same regions, same primes 2053 and 2063, same three residues and confirm.
  On one corrupted byte it corrects 604 of 3000 and **miscorrects 43, 1.4%**. In the ring
  it returned WRONG data on `notepad.exe` and forfeited. PREDICTIONS called this channel
  "~0 corrected, **0 wrong**"; the second half was wrong. Three rounds of this lineage
  reported 0 miscorrections on every channel they ran, and not one of them ran the injury
  that actually happens.
- **Nothing survives a 4 KB wound, σ or no σ.** B3 predicted `archive.zst` corrected and
  `notepad.exe` borderline. Every row forfeits every 4 KB injury on every file. The
  arithmetic that predicted otherwise counted damaged bytes per *block* and forgot that
  one equation per region solves one unknown **per region**. The fix is more equations per
  region, which is Reed–Solomon, and it is the next round's opening, not this one's result.
- **The planning ground had a defect.** The script that measured σ's scatter *dropped*
  partners falling past the end of the file, which is not a permutation. `fileSigma` fixes
  them in place. `spec.md`'s worst block is **39**, not the 37 filed. Right on the other
  files, wrong on the smallest, and kept.
- **Two erasures in one region were called "refused" and were recovered 1000/1000.** At
  A = 256, `A²` is 65,536 — cheaper to enumerate than to refuse, and `q` settles it.
- **The audit of 2026-09-03 found three defects in this round's own artifact**, one of them
  a silent data loss. They are fixed and written up under *The implementation audit* below,
  with the numbers they moved.

## Why this round exists

Three things the first three rounds never did.

1. **Every cell held one bit.** Overheads of 2.34%, 4.69%, 5.86% and 103% were compared as
   the codec's cost. They are the *block's* cost: overhead is `4·⌈log₂p⌉ / (L·log₂A)`, and
   `log₂p` grows logarithmically while the block grows linearly. Every one of those numbers
   was measured at one design point that was never the variable.
2. **Every construction stopped at the square's edge.** v1(c)'s own verdict says it: "σ
   never leaves its square." Real damage does.
3. **Nothing had been in a ring.** The codegg line has a tournament with a rule. The eggSo
   line had synthetic channels on random squares, and its two shipped candidates had never
   been asked to hand back a real file.

## The construction

**Arm (a), the radix arm.** v0's codec with the cell's alphabet as a parameter. A cell
holds a digit in `0..A−1`, the square is that number in base `A`, an error `d` at cell `i`
moves it by `d·A^(L−1−i)`, and one prime injective over `{d·A^k : d ∈ ±1..A−1, k < L}`
names the cell, the sign and the size. Three region residues and the confirming `q`, as
v0, with the amendment's per-candidate confirm. Erasures at known addresses are **solved,
not enumerated** — one unknown in one equation — which is the only path open at A = 256.

The moduli, found by search and re-verified by enumeration every run:

| A | N | block | p | q | check bits | overhead |
| --- | --- | --- | --- | --- | --- | --- |
| 2 (bit) | 32 | 128 B | 2,053 | 2,063 | 48 | 4.688% |
| 16 (nibble) | 16 | 128 B | 17,627 | 19,429 | 60 | 5.859% |
| 16 | 32 | 512 B | 61,381 | 65,831 | 65 | 1.587% |
| 256 (byte) | 16 | 256 B | 2,265,761 | 2,288,267 | 88 | 4.297% |
| 256 | 32 | **1024 B** | 8,030,879 | 8,035,021 | 92 | **1.123%** |

At *equal* block size a bigger alphabet is worse: 128 B costs 4.69% in bits and 5.86% in
nibbles, because the prime must keep `2(A−1)L` values apart *and* dodge every ratio
`d₂/d₁`. At A = 256, N = 16 the naive bound is 130,560 and the prime is **2,265,761,
17.4× larger**. The alphabet's win is not cheapness; it is that **one corrupted byte is
one cell**.

**Arm (b), the file-scale fold.** The checks stay on the source's blocks; the artifact
stores those bytes in anti-transpose order over the whole file:

```
artifact[k] = source[σ(k)],   σ(j) = M−1 − ((j mod n)·n + ⌊j/n⌋),   n = ⌈√M⌉
```

σ is an involution, so the encoder is the decoder, there is no table, and it stores
nothing. A contiguous wound in the artifact becomes a σ-scattered set of source addresses
— thin erasures in many blocks instead of the total loss of a few — and with the original
length carried in the header, **a truncation is the same injury as a scratch**.

| | |
| --- | --- |
| **borrowed** | the region rule is eggSo-v0's `regionOf`, `require`d, which asserts itself against `stalk.js:118`. The residue, the modulus search and the injectivity-by-enumeration discipline are codegg-v1's (`codegg.js:55-74`), generalised from radix 2 to radix A. The per-candidate confirm is `codegg.js:204-206`, which v0 now carries as its amendment |
| **the site's** | the anti-transpose, via eggSo-v1's `partnerOf`, which is `index.html:398` |
| **the names** | arm (a) is an AN / residue arithmetic code **in radix A**, interleaved by the fold's partition — v0's name with the radix finally named. Arm (b) is a whole-file interleaver concatenated with a per-block AN code, and the involution makes encoder and decoder one map |

## Results, all measured

### The ring (`tools/ring.js`)

codegg-v12's rule, unchanged: wrong-or-no data after any injury forfeits; among survivors
the smallest artifact wins. Seven rows, seven files, four injuries. `bitflip` is the
injury this lineage was built for and the codegg ring never had; `byteflip` is the one real
storage delivers and no round before this ever ran.

| row | artifact | 1 bit | 1 byte | 4 KB scratch | 4 KB trunc |
| --- | --- | --- | --- | --- | --- |
| bit/128B — *eggSo-v0's code exactly* | 104.69% | EXACT | detected ×5, **WRONG ×1**, EXACT ×1 | detected | detected |
| bit/128B + σ | 104.69% | EXACT | EXACT ×6, detected ×1 | detected | detected |
| nib/512B, ±σ | 101.76% | EXACT | **EXACT everywhere** | detected | detected |
| byte/256B | 104.30% | EXACT | **EXACT everywhere** | detected | detected |
| **byte/1KB, ±σ** | **101.17%** | EXACT | **EXACT everywhere** | detected | detected |

No row survives all four, so the ring's own rule crowns nobody — and that is the honest
result. What separates the rows is the byte flip, where one row lies, and the capacity
below, where σ is worth up to three orders of magnitude.

### The capacity curve — the largest contiguous scratch survived exactly

Added after the 4 KB row came back all-forfeit, with the expectation filed in `ring.js`
before it was run. Bisection, per row, per file.

| row | spec.md | stalk.js | og.png | wubbadub | program.exe | notepad.exe | archive.zst |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bit/128B | 3 | 4 | 2 | 2 | 4 | 4 | 3 |
| **bit/128B + σ** | **248** | **280** | **300** | **610** | **1030** | **1203** | **1946** |
| nib/512B | 2 | 2 | 2 | 4 | 4 | 4 | 3 |
| nib/512B + σ | 3 | 2 | 185 | 608 | 1030 | 1203 | 1946 |
| byte/256B | 2 | 2 | 2 | 2 | 4 | 4 | 3 |
| byte/1KB | 2 | 2 | 2 | 5 | 4 | 4 | 3 |
| **byte/1KB + σ** | 2 | 2 | 5 | 306 | 520 | 604 | **1946** |

Bare capacity is 2–5 bytes at every block size and every file size, exactly as called: a
contiguous wound lives inside one or two blocks and one equation per region solves one byte
per region. σ makes the same three-per-block available in every block the scatter reaches,
so its gain scales with the *number* of blocks — which is why `bit/128B + σ`, the most
expensive square and the one with the most blocks, beats `byte/1KB + σ` everywhere except
a tie on the largest file. A truncation gives the same number, file for file.

### What the suite pins down (`tools/eggso3.test.js`)

| claim | result |
| --- | --- |
| five moduli re-derived by search, re-verified by enumeration | five for five, both ways, every run |
| round-trip through the artifact | exact, 70 shapes (5 radices × σ on/off × 7 lengths) |
| one corrupted **digit**, 3000, every radix | 3000/3000, all direct, 0 wrong |
| one corrupted **byte**: byte/1KB · byte/256B | **3000/3000 · 3000/3000**, 0 wrong |
| one corrupted **byte**: nibble/128B · nibble/512B | 2997 · 2973, 0 wrong |
| one corrupted **byte**: bit/128B | **604 corrected, 2353 detected, 43 MISCORRECTED** |
| one flagged cell per region, every radix | **1000/1000 solved**, no enumeration |
| two flagged in one region, every radix | 1000/1000 recovered, 0 wrong |
| `σ∘σ = id`; σ's cost | 7/7 file lengths; **0 bytes** |
| σ's worst 128-B block: spec / stalk / og / wubbadub | 39 / 30 / 23 / 14 (ground said 37 for the first) |
| cost per block | 4.69 / 5.86 / 1.59 / 4.30 / **1.12** % |

### The implementation audit — 2026-09-03

The round was reviewed against its own artifact instead of against synthetic squares.
Three defects, one of them the kind that matters.

| audited | found | fixed |
| --- | --- | --- |
| **the header** | its ten bytes were naked. Flipping the low bit of the declared **length** returned a file one byte short that passed every residue check **silently**. Flipping a high bit threw `Invalid array length` out of the decoder instead of refusing | 12-byte header with a Fletcher-16 over the other ten, and every field bounds-checked before it sizes an array. **All 96 single-bit flips of the header now refuse cleanly: 0 silent, 0 throws** |
| **the check area** | already safe, and worth recording: flipping **every one of the 180 check bytes** one at a time leaves the data exact 180 times, 0 silently wrong. A corrupted check cannot manufacture a repair that also satisfies `q` | nothing to fix |
| **a wound straddling checks and payload** | detected, not silently wrong | nothing to fix |
| **erasure ambiguity** | a region with two readings returned `ambiguous` at once, throwing away the point of carrying `q`. Safe, but it cost capacity | candidate lists per region are combined and `q` picks among them, which is eggSo-v0's own shape (`eggso.js:158-188`) |

The erasure fix roughly **doubled the capacity curve** and changed its ordering: the bit
square, which has the most blocks to scatter into, now ties the byte square on the largest
file instead of losing to it. The capacity table above is the post-fix run. Everything else
re-ran unchanged. The header is two bytes larger, so every artifact is two bytes larger.

### Original against recipe — what this is worth to the transmuter line

**Every eggSo artifact is larger than its source.** There is no compression stage anywhere
in this lineage; it is armor, and armor costs. So eggSo cannot enter the codegg tournament,
whose rule is *smallest lossless survivor* and whose contenders compress.

| file | original | recipe, byte/1KB | added | recipe, bit/128B — v0's square |
| --- | --- | --- | --- | --- |
| spec.md | 15,190 | 15,382 | **+192 B, 1.26%** | 15,916, +4.78% |
| stalk.js | 19,567 | 19,819 | +252 B, 1.29% | 20,497, +4.75% |
| og.png | 33,742 | 34,150 | +408 B, 1.21% | 35,338, +4.73% |
| wubbadub.html | 92,408 | 93,512 | +1,104 B, 1.19% | 96,752, +4.70% |
| program.exe | 265,216 | 268,336 | +3,120 B, 1.18% | 277,660, +4.69% |
| notepad.exe | 360,448 | 364,684 | +4,236 B, 1.18% | 377,356, +4.69% |
| archive.zst | 946,623 | 957,735 | **+11,112 B, 1.17%** | 991,011, +4.69% |

Where it *can* matter is as the armor layer in the hybrid posture — compress, then armor.
Against codegg-v6's ribs on the same files:

| file | eggSo-v3 byte/1KB | codegg-v6 ribs | eggSo saves |
| --- | --- | --- | --- |
| spec.md | +192 B, 1.26% | +5,958 B, **39.22%** | 5,766 B |
| notepad.exe | +4,236 B, 1.18% | +26,840 B, 7.45% | 22,604 B |
| archive.zst | +11,112 B, 1.17% | +69,009 B, 7.29% | **57,897 B** |

**Six to thirty-one times cheaper, and correspondingly weaker.** codegg-v6's ribs restore a
4 KB scratch and a 4 KB truncation. eggSo-v3 restores a corrupted byte anywhere, and with σ
a wound up to 1,946 bytes on a megabyte file. It is the right armor for bit rot and the
wrong armor for a lost sector, and the two are not interchangeable at any price. Choosing
between them is a threat-model decision, not a size decision.

### The honest section

- **The ring is eggSo against eggSo.** `bit/128B` is v0's code exactly, so v0 is really in
  it; eggSo-v1(a)'s extra residue and v1(b)'s mirror are not. They change what a *square*
  carries, and these injuries are decided by the block size and the radix — a second
  syndrome names a pair that a byte square never has to name. Saying so is cheaper than a
  row that was never run. codegg-v1 is absent for the same reason and would forfeit the
  byte flip on the same arithmetic as `bit/128B`.
- **Nothing here survives 4 KB.** One equation per region solves one unknown per region.
  Surviving a wound of `k` bytes needs `k` equations spread over the blocks it touches,
  and that is Reed–Solomon — a name this lineage already reached through the Wub. σ buys
  the spreading for free; it does not buy the equations.
- **σ's gain is the block count, not the block size.** The smallest, most expensive square
  scatters best. That is a genuine tension between the two findings of this round, and it
  is not resolved here.
- **Two erasures per region are enumerated, not solved.** `A²` = 65,536 at A = 256 is
  affordable; `A³` is not, and the decoder refuses rather than guessing.
- **The recipe is never smaller than the file.** Worth saying twice, because every other
  series in this repo is measured on getting smaller. This one is measured on getting the
  file back.
- **The cached primes.** `PRIMES` in the source holds the five pairs so the ring does not
  spend nineteen seconds a file rediscovering them. The suite re-derives every one **by
  search** and re-verifies it **by enumeration** on every run, so a cache that drifts is a
  failed test, not a silent wrong answer.

## Running it

```
node eggSo-v3/tools/eggso3.test.js              # eight blocks, five radices, ~40 s
node eggSo-v3/tools/ring.js --json              # the ring and the capacity curve, ~55 s
node eggSo-v3/tools/ring.js spec.md stalk.js    # a shorter ring on two files
```

## Files

| | |
| --- | --- |
| `eggso3.js` | `makeCode(N, A)`, `pickModulus`/`injectiveFor`/`injectiveByEnumeration`, `repairSquare` (solve-not-enumerate erasures, per-candidate confirm, in-region pair), `fileSigma`/`scatter`, `encode`/`decode` over a real artifact with a header, `cellsOfByte`, `sizes`. Requires `../eggSo-v0/eggso.js`, `../eggSo-v1/eggso1.js`, `../codegg-v1/codegg.js` |
| `PREDICTIONS.md` | filed before building; measured after; misses kept |
| `tools/eggso3.test.js` | the eight claims |
| `tools/ring.js` | the ring, the rule, and the capacity curve |
| `measured-*.json` | the tools' own record of the numbers above |

## What this is and is not

It is the round that found the variable nobody in this lineage had changed, and a
capability the fold's own involution supplies for nothing. It is also the round that had
to report that the code three previous rounds shipped will silently corrupt a file that
loses a single byte. It is not a claim that the fold is anything other than what the
site's README says it is: held loosely — with its partition, its map, its
alphabet, its radix and now its scale each on the record, and each with a name.
Placed in [eggSo-v4](../eggSo-v4/) as a basin boundary: the Julia set of a degree-2 map, with Inner and Outer as its Fatou basins.
