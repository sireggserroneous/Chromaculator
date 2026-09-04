# codegg-v12 PREDICTIONS -- THE REMAINDER

Filed at M0 (2026-09-02), BEFORE any M1 code was written. The house law: the
predictions go first, the misses are printed beside the wins, and nothing is
silently repriced. This is the series' FIRST EXACT prediction column -- twenty
totals, to the byte, derived from the geometry alone (v1..v11 predicted ranges).

## The charter verse

spec.md:134-156 -- "**Division -- quotient, multiplier, remainder... R is a
stalk too, always inside... Widening the grid grows Q and shrinks R. The
identity never moves.**" (also spec.md:76 "the three sum back to the number
exactly"). The armored form IS that identity in GF(2^16)[x]:

    A * x^t = Q * B + R        the file is A, the generator is B, the armor is R

The transmuted form is A*x^t - R, the nearest multiple of B; nothing is dropped.
A wound leaves a nonzero remainder and the remainder names the wound (codegg-v1's
syndrome, one level up). Clean means remainder zero. v11 divided by a SMALL B
many times (groups <= 248 in GF(256)) and paid R per group -- the price of
survival scaled with the file (0.8-1.2%, up to 106x the pigeonhole floor). v12
divides ONCE by a wide B; R shrinks to a constant. The identity never moves.

## The reframe (Vladimir, 2026-09-02)

"We are transmutating not compression." The v11 bars ("armored egg lighter than
NAKED xz") were a compressor's bars and "impossible by pigeonhole" was a
compressor's word. The transmuter's currencies:

1. **Conservation** -- every injury EXACT; wrong data never; refuse with a number.
2. **The price of each power, printed beside its floor.** Surviving the loss of
   any 4 KB of yourself costs >= 4,096 B for anyone; a price tag, not a wall.
3. **The form's weight, measured form-vs-form** (inner vs the rival's stream),
   and armored-vs-armored against rivals that bought the same power (xz+par2,
   rar -rr). Armored-vs-naked stays PRINTED as an honesty exhibit; never a bar.

## The bars (in the currencies)

1. **Conservation:** 3 injuries x 20 rows (+ the formats card) EXACT,
   certutil-countersigned; scattered-wound drills EXACT; t+1 refuses. Wrong data never.
2. **The price is floored:** survival parity = dead(blk)*blk EXACTLY at every
   size (construction; audit-proved). Total per row = the arithmetic column below
   TO THE BYTE. Every `info` prints price beside 4,096.
3. **Form vs form (M2, not M1):** inner lighter than naked xz on >= 17/18 measured
   rows (cbs flips via the line/column context; rustc coin flip; sav the printed loss).
4. **Armored vs armored** (xz+par2, rar -rr5; tools/challengers.js): 23/23 -- the
   save needs 0.29 pt after the flat price lands (coin flip, filed).
5. **The ratchet:** v12 <= min(v8..v11) armored total on every row.
6. **Exhibit (not a bar):** armored egg12 vs naked xz -- by construction 12/18
   (was 9/18); printed with the price line so the reader sees what the powers
   cost against an artifact that bought none.

## The arithmetic column (M1 gate: TO THE BYTE)

Geometry, fixed here: one systematic RS codeword over GF(2^16) (poly 0x1100B) in
the BCH view, g(x) = prod_(i=0..t-1) (x - alpha^i); the square of blk bytes is
blk/2 symbols, symbol j of every square is codeword j; t = dead(blk) =
ceil(4096/blk)+1 (17/9/5/3 at 256/512/1024/2048); one u16 residue per square =
the square as a big-endian number mod 65,519; the CT is either TRIPLICATED in
the three sites (meta/site = 2(s+t) residues + fnv32; n = s+t) or IN-CODEWORD
(c = ceil(2s/blk) CT squares carry the s data residues and are codeword members;
meta/site = 2c + 2t residues + fnv32; n = s+c+t); the grid is searched over
256/512/1024/2048 and the two CT placements by argmin of the total; the LAST
DATA SQUARE IS KEPT SHORT (glossary.js:164 "Kept rather than rounded away":
zero-padded for the arithmetic, only its real bytes stored, the header's inner
length re-pads). Headers 3 x 64 B. Total = 3*(64 + msize) + n*blk - pad.

The inner (the transmuted stream) is v11's, byte for byte -- M0 proved the
payload bit-identical on 14/14 home rows and the six big rows were transmuted by
eggv12 with v11's armor still in place; `eggv12 info` fixed their exact inners:

| big row | inner (exact) | v11 total (sealed) | v3 geometry |
|---|---|---|---|
| msgraph.dll | 4,558,852 | 4,617,660 | blk 1024, T2 (v12-M0 measured 4,617,660) |
| mermaid-bundle.js | 4,827,729 | 4,891,080 | blk 1024, T2 (v12-M0 measured 4,891,080) |
| ntoskrnl.exe | 4,974,242 | 5,039,572 | blk 1024, T2 (v12-M0 measured 5,038,548: the sealed sheet predates v11's 16 MB frozen-arms law; see M0 note) |
| aoe4-autosave.sav | 17,369,261 | 17,553,840 | blk 2048, G248 T2 (v12-M0 measured 17,553,840) |
| rdr2-shaders.vkcache | 41,881,828 | 42,312,400 | blk 2048, T2 (v12-M0 measured 42,312,400) |
| rustc_driver.dll | 42,286,035 | 42,719,952 | blk 2048, T2 (v12-M0 measured 42,719,952) |

Price = parity + CT + sites, and the components SUM. "x floor" is price / 4,096.

| row | inner | v11 total | **v12 total** | price = parity + CT + sites (grid) | x floor | saves |
|---|---|---|---|---|---|---|
| wubbadub.html | 23,099 | 30,596 | **28,019** | 4,920 = 4,352 + 256 + 312 (256, CT in-cw; n=109) | 1.20x | 2,577 |
| ring01.wav | 137,104 | 146,184 | **143,006** | 5,902 = 4,608 + 1,024 + 270 (512, CT in-cw; n=279) | 1.44x | 3,178 |
| cbs.log | 142,359 | 150,804 | **148,261** | 5,902 = 4,608 + 1,024 + 270 (512, CT in-cw; n=290) | 1.44x | 2,543 |
| notepad.exe | 175,045 | 183,060 | **180,947** | 5,902 = 4,608 + 1,024 + 270 (512, CT in-cw; n=353) | 1.44x | 2,113 |
| real-test.bmp | 259,575 | 268,588 | **265,477** | 5,902 = 4,608 + 1,024 + 270 (512, CT in-cw; n=518) | 1.44x | 3,111 |
| alarm01.wav | 263,764 | 273,196 | **270,148** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=264) | 1.56x | 3,048 |
| kernel32.dll | 291,418 | 300,832 | **297,802** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=291) | 1.56x | 3,030 |
| vim-version9.txt | 308,884 | 319,264 | **315,268** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=308) | 1.56x | 3,996 |
| iconcache48.db | 414,406 | 424,760 | **420,790** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=411) | 1.56x | 3,970 |
| segoeui.ttf | 417,854 | 429,368 | **424,238** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=415) | 1.56x | 5,130 |
| arial.ttf | 456,596 | 468,292 | **462,980** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=452) | 1.56x | 5,312 |
| zstd.exe | 509,765 | 521,540 | **516,149** | 6,384 = 5,120 + 1,024 + 240 (1024, CT in-cw; n=504) | 1.56x | 5,391 |
| real-test.db | 1,223,539 | 1,241,376 | **1,231,959** | 8,420 = 6,144 + 2,048 + 228 (2048, CT in-cw; n=602) | 2.06x | 9,417 |
| wallpaper.jpg | 1,511,982 | 1,533,228 | **1,520,402** | 8,420 = 6,144 + 2,048 + 228 (2048, CT in-cw; n=743) | 2.06x | 12,826 |
| msgraph.dll | 4,558,852 | 4,617,660 | **4,571,380** | 12,528 = 6,144 + 6,144 + 240 (2048, CT in-cw; n=2233) | 3.06x | 46,280 |
| mermaid-bundle.js | 4,827,729 | 4,891,080 | **4,840,257** | 12,528 = 6,144 + 6,144 + 240 (2048, CT in-cw; n=2364) | 3.06x | 50,823 |
| ntoskrnl.exe | 4,974,242 | 5,039,572 | **4,986,770** | 12,528 = 6,144 + 6,144 + 240 (2048, CT in-cw; n=2435) | 3.06x | 52,802 |
| aoe4-autosave.sav | 17,369,261 | 17,553,840 | **17,394,113** | 24,852 = 6,144 + 18,432 + 276 (2048, CT in-cw; n=8494) | 6.07x | 159,727 |
| rdr2-shaders.vkcache | 41,881,828 | 42,312,400 | **41,929,274** | 47,446 = 6,144 + 40,960 + 342 (2048, CT in-cw; n=20474) | 11.58x | 383,126 |
| rustc_driver.dll | 42,286,035 | 42,719,952 | **42,335,535** | 49,500 = 6,144 + 43,008 + 348 (2048, CT in-cw; n=20672) | 12.08x | 384,417 |

Net **-1,142,817 B** across 20 rows before the model moves (v11's whole campaign: -1,766,332). The price is a clean function of the grid: 4,920 (256) / 5,902 (512) / 6,384 (1024) / 8,420 (2048 + one CT square) -- flat per tier; at scale the diagnosis table (2 B/square) is the larger line (rdr2 40,960 vs 6,144) and is printed as its own power.

### M0 note: ntoskrnl.exe's sealed total is stale by one square

v12-M0 (v11's armor verbatim, EG12 magic) produced 5,038,548 for ntoskrnl.exe,
not the sealed 5,039,572 (-1,024 = one 1024-B square). The other 19 rows matched
the sealed sheet to the byte. Cause: codegg-v11/ledger-m8-sealed.txt was written
09:24 on 2026-09-02; v11's main.rs was edited at 09:46 (the frozen-elders law
raised from 4 MB to 16 MB; ntoskrnl, 13 MB, sits in that gap) and the shipped
eggv11.exe rebuilt at 10:07. The v12 column is computed from the inner the
current source produces (4,974,242); the "saves" column for ntoskrnl is judged
against the sealed 5,039,572 as the sheet records it. Both numbers are printed.

### M0 correction of the plan's arithmetic (filed here, before code)

The plan's table (the-remainder-v12.md, "The arithmetic column") put the parity
squares' residues INSIDE the in-codeword CT squares. That is circular: the CT
squares are codeword members, so the parity is a function of the CT's bytes,
which would have to contain a function of the parity. The residues of the t
parity squares therefore sit in the meta of each site (+2t bytes per site, +6t
per row: +102 at 256, +54 at 512, +30 at 1024, +18 at 2048). The six big rows
also move from the plan's +-1-square guesses to exact inners, and real-test.bmp
moves to the 512 grid (the corrected CT needs 2 squares there, not 3, so 512
undercuts 1024 by 482). Plan vs filed:

| row | plan | filed | delta |
|---|---|---|---|
| wubbadub.html | 27,917 | 28,019 | +102 |
| ring01.wav | 142,952 | 143,006 | +54 |
| cbs.log | 148,207 | 148,261 | +54 |
| notepad.exe | 180,893 | 180,947 | +54 |
| real-test.bmp | 265,929 | 265,477 | -452 |
| alarm01.wav | 270,118 | 270,148 | +30 |
| kernel32.dll | 297,772 | 297,802 | +30 |
| vim-version9.txt | 315,238 | 315,268 | +30 |
| iconcache48.db | 420,760 | 420,790 | +30 |
| segoeui.ttf | 424,208 | 424,238 | +30 |
| arial.ttf | 462,950 | 462,980 | +30 |
| zstd.exe | 516,119 | 516,149 | +30 |
| real-test.db | 1,231,941 | 1,231,959 | +18 |
| wallpaper.jpg | 1,520,384 | 1,520,402 | +18 |
| msgraph.dll | 4,572,382 | 4,571,380 | -1,002 |
| mermaid-bundle.js | 4,840,670 | 4,840,257 | -413 |
| ntoskrnl.exe | 4,988,126 | 4,986,770 | -1,356 |
| aoe4-autosave.sav | 17,395,970 | 17,394,113 | -1,857 |
| rdr2-shaders.vkcache | 41,931,076 | 41,929,274 | -1,802 |
| rustc_driver.dll | 42,336,586 | 42,335,535 | -1,051 |

## M1 MEASURED (2026-09-02, the gate) -- 17 of 20 to the byte; 3 misses, all LIGHTER, cause proven

The ledger (tools/ledger12.js with EGG_PRED = the filed column; the monster alone,
the five other big rows in 5 lanes, the 14 home rows in 8 lanes). Injuries are
the tournament's three (flip blind / 4 KB scratch addressed / 4 KB truncation).
MB/s is the transmute under those lanes.

| row | predicted | measured | delta | price | x floor | v11 sealed | vs v11 | injuries | MB/s |
|---|---|---|---|---|---|---|---|---|---|
| wubbadub.html | 28,019 | **28,019** | HIT | 4,920 | 1.20x | 30,596 | -2,577 (-2.789 pt) | E/E/E | 0.25 |
| ring01.wav | 143,006 | **143,006** | HIT | 5,902 | 1.44x | 146,184 | -3,178 (-0.638 pt) | E/E/E | 0.34 |
| cbs.log | 148,261 | **148,090** | MISS -171 | 5,902 | 1.44x | 150,804 | -2,714 (-0.017 pt) | E/E/E | 1.00 |
| notepad.exe | 180,947 | **180,947** | HIT | 5,902 | 1.44x | 183,060 | -2,113 (-0.586 pt) | E/E/E | 0.35 |
| real-test.bmp | 265,477 | **265,318** | MISS -159 | 5,902 | 1.44x | 268,588 | -3,270 (-0.027 pt) | E/E/E | 0.40 |
| alarm01.wav | 270,148 | **270,148** | HIT | 6,384 | 1.56x | 273,196 | -3,048 (-0.620 pt) | E/E/E | 0.25 |
| kernel32.dll | 297,802 | **297,568** | MISS -234 | 6,384 | 1.56x | 300,832 | -3,264 (-0.390 pt) | E/E/E | 0.45 |
| vim-version9.txt | 315,268 | **315,268** | HIT | 6,384 | 1.56x | 319,264 | -3,996 (-0.196 pt) | E/E/E | 0.28 |
| iconcache48.db | 420,790 | **420,790** | HIT | 6,384 | 1.56x | 424,760 | -3,970 (-0.004 pt) | E/E/E | 1.27 |
| segoeui.ttf | 424,238 | **424,238** | HIT | 6,384 | 1.56x | 429,368 | -5,130 (-0.535 pt) | E/E/E | 0.43 |
| arial.ttf | 462,980 | **462,980** | HIT | 6,384 | 1.56x | 468,292 | -5,312 (-0.508 pt) | E/E/E | 0.30 |
| zstd.exe | 516,149 | **516,149** | HIT | 6,384 | 1.56x | 521,540 | -5,391 (-0.337 pt) | E/E/E | 0.32 |
| real-test.db | 1,231,959 | **1,231,959** | HIT | 8,420 | 2.06x | 1,241,376 | -9,417 (-0.099 pt) | E/E/E | 0.26 |
| wallpaper.jpg | 1,520,402 | **1,520,402** | HIT | 8,420 | 2.06x | 1,533,228 | -12,826 (-0.800 pt) | E/E/E | 0.26 |
| msgraph.dll | 4,571,380 | **4,571,380** | HIT | 12,528 | 3.06x | 4,617,660 | -46,280 (-0.107 pt) | E/E/E | 0.45 |
| mermaid-bundle.js | 4,840,257 | **4,840,257** | HIT | 12,528 | 3.06x | 4,891,080 | -50,823 (-0.197 pt) | E/E/E | 0.13 |
| ntoskrnl.exe | 4,986,770 | **4,986,770** | HIT | 12,528 | 3.06x | 5,039,572 | -52,802 (-0.405 pt) | E/E/E | 0.06 |
| aoe4-autosave.sav | 17,394,113 | **17,394,113** | HIT | 24,852 | 6.07x | 17,553,840 | -159,727 (-0.240 pt) | E/E/E | 0.22 |
| rdr2-shaders.vkcache | 41,929,274 | **41,929,274** | HIT | 47,446 | 11.58x | 42,312,400 | -383,126 (-0.784 pt) | E/E/E | 0.10 |
| rustc_driver.dll | 42,335,535 | **42,335,535** | HIT | 49,500 | 12.08x | 42,719,952 | -384,417 (-0.210 pt) | E/E/E | 0.08 |

Net **-1,143,381 B** vs the sealed v11 (predicted -1,142,817; the three misses
each landed lighter). Ratchet: every row <= v11 sealed (and v11 sealed <=
min(v8, v9, v10) on 20/20 per codegg-v11/README.md), so v12 <= min(v8..v11) on
20 of 20. Wrong data: never (0 of 60 injuries, 0 of 143 drills, 0 of 2,087,576
audit checks).

### The three misses: the INNER moved, not the arithmetic

| row | predicted inner | measured inner | inner delta | measured total - inner | predicted price |
|---|---|---|---|---|---|
| cbs.log | 142,359 (model 14, MIX11H) | 142,188 (model 10, MIX11) | -171 | 5,902 | 5,902 |
| kernel32.dll | 291,418 (model 13, CM11P) | 291,184 (model 11, CM11) | -234 | 6,384 | 6,384 |
| real-test.bmp | 259,575 (model 10, MIX11) | 259,416 (model 8, MIX10) | -159 | 5,902 | 5,902 |

The arithmetic column is exact on all 20 rows: total - inner equals the filed
price to the byte everywhere. What the M0 column got wrong is the assumption
"the inner is v11's, byte for byte." The inner is chosen by the model ARMS,
and the arms compare ARMORED totals (the series law since v8, main.rs
`armored_total`; ties keep the first arm). M0 fixed each row's inner under
v11's armor v3, whose price is a GROUP-ROUNDED function of the inner length;
v4's price is byte-exact. Same candidate set, same model code (untouched in
M1), different argmin. Proven with v11's own `armor11::rib_policy` + `geom`
(verbatim in this crate), evaluated on both candidate inners:

    cbs.log       v3 total(142,359) = 150,804 (g93 t3)   v3 total(142,188) = 151,304 (g55 t2)   lighter inner LOSES under v3 by 500 B
    kernel32.dll  v3 total(291,418) = 300,832 (g114 t2)  v3 total(291,184) = 301,344 (g113 t2)  lighter inner LOSES under v3 by 512 B
    real-test.bmp v3 total(259,575) = 268,588 (g169 t3)  v3 total(259,416) = 268,588 (g169 t3)  TIE under v3; the first arm (heavier) was kept

(151,304 and 301,344 are the v10 cbs and the pre-honesty v11 kernel32 totals --
the "kernel32 saga" of the v11 README was this same grid pathology.) Under v4
the lighter inner wins outright and the row lands 171 / 234 / 159 B under the
filed total. A prediction that had modeled the arm choice under the NEW price
would have hit 20/20; this one is filed as 17/20 with the cause, per the law.

### What the plan got wrong (found by the audit, fixed before the gate; totals unaffected)

1. **The short data square must ride at stream position 0.** The plan kept the
   last data square short "at its index" (mid-stream). Audit (a) slid a 4,096-B
   wound across every real container and found the bound `dead(blk) = ceil(4096/blk)+1`
   BREAKS when the short square sits inside the run: a 1-byte square costs the
   wound nothing, so a wound can straddle dead(blk)+1 squares. Fix: the short
   square is stored first (right after the head site, where only the site
   precedes it); data squares 0..s-2 follow at stream positions 1..s-1. Square
   indices, codeword positions and the total are unchanged (`armor::stream_pos`).
2. **Mode B's BLIND promise is qualified.** With the CT in the codeword, a dead
   CT square hides blk/2 residues. Named wounds: any t squares, always. Blind:
   any contiguous run (<= t squares) and any t-1 scattered squares are exact;
   for t scattered squares the decoder's collaborative rung (interleaved RS,
   Krachkovsky-Lee 1997 / Bleichenbacher-Kiayias-Yung 2003) locates k dead
   unjudged data squares when k < m = t - |convicted|, a bounded search settles
   k = m <= 2, and k = m >= 3 is an honest REFUSE with the number. `info` and
   the transmute line print the promise in this form. Mode A (--ct triple) keeps
   the unqualified promise: any t squares, blind or named.
3. **The parity squares' residues live in the meta, not in the CT** (filed at
   M0 above; the +6t per row is in the column).
4. **Berlekamp-Massey alone is not a decoder rung.** A locator of degree d with
   2d >= m (m = remaining capacity) is unverified; the rung accepts a partial
   location only when the located set is consistent across every bad codeword
   and leaves room (e + located < t). Before that guard, one wrong single-error
   location poisoned a k = m = 2 case at blk 2048 (found by the audit, not a
   drill).

### The battery and the audit

- `eggv12 audit`: 187,457 checks, 0 failing (~0.36 s); `--full`: 2,087,576
  checks, 0 failing (2.3 s). ord_65519(2) = 32,759 (the -1 is never reached;
  65,519 prime; +-2^k distinct over 256..2048-B squares; the 4096 tier collides
  at bit 32,759 = byte 4,094 -- printed, the tier is reached only when n > 65,535
  at 2048). alpha has order 65,535; 65,535 inverses; table fnv64 6c9457c15535968b.
- `node tools/drills.js`: 143 passed, 0 failed (8m40s): the three cases, the
  derived mode-B expectations (k < m EXACT, k = m = 2 EXACT, k = m = 16 / 4
  HONEST and named EXACT), --ct triple any-17 blind, --tier 1024, --parity 9
  (9 blind EXACT / 10 named HONEST), --survive 65536 (blk 1024, t 65, 64 KB
  scratch anywhere blind EXACT incl. the head and a 64 KB truncation), 3-bit
  storms 300/300, wide scratches beyond t HONEST, no-armor, ancestors
  .egg8..egg11 pristine + wounded head EXACT, pigeonhole.
- Ancestors' own drills: codegg-v11 75/75 (7m48s), codegg-v10 75/75 (6m59s).
- `cargo test --release`: 15 passed; `cargo clippy --release --all-targets -- -D warnings`: clean.
- The monster (rustc_driver.dll, 183 MB): 39m36s end-to-end alone (transmute +
  info + 3 injuries), <= 45 min (v11: 40m23s).

## Exhibit line (armored egg12 vs NAKED xz -9; an exhibit, not a bar)

Wins: ring01.wav WIN 105,418; notepad.exe WIN 937; alarm01.wav WIN 74,492; kernel32.dll WIN 18,614; vim-version9.txt WIN 55,820; arial.ttf WIN 3,736; zstd.exe WIN 18,423; wallpaper.jpg WIN 51,422; msgraph.dll WIN 177,996; mermaid-bundle.js WIN 111,875; ntoskrnl.exe WIN 514,818; rdr2-shaders.vkcache WIN 40,366.
Heavier: wubbadub.html -1,839; cbs.log -9,257; iconcache48.db -3,822; segoeui.ttf -1,934; aoe4-autosave.sav -654,193; rustc_driver.dll -628,155.
That is 12/18 by construction (v11: 9/18) -- the same twelve the plan named.
The heavier six are the M2/M3 modeling targets, form-vs-form; the exhibit is
printed, not chased.

## Pre-shrunk model ranges (M2/M3; copied from the plan, NOT part of the M1 gate)

- **line/column context** (text arms; sniff >= 2% newlines, no NULs): 1-4% on
  logs (paq8's column contexts lineage). cbs.log needs 2.4% to flip form-vs-form
  -- likely, not certain.
- **checksummed hash slots** (8-bit check of the full hash per slot; paq8/lpaq
  lineage): 0.3-1.5% on dense binaries; rustc needs 1.39% => coin flip with the
  dialect books.
- **dialect books** (PE, TTF via gen-prior over a corpus that EXCLUDES every test
  row): 0.2-1.0%; trial arms with MODEL bytes, never always-on.
- **the save (3.77% behind xz form-vs-form):** probe the float field filter on a
  1 MB sample first; < 0.5% on the sample => the save is printed as the honest
  loss of the card.
- exhibits (no bar depends on them): stereo mid/side + NLMS 2-6% vs FLAC -8;
  JPEG peel 15-25% (stretch); float filter 0-1% speculative; transpose 0-0.3%.

## Calibration note

Every prior campaign predicted RANGES for the model and paid for it in
calibration rows. The column above is different in kind: it is arithmetic on a
geometry that does not depend on the data, so a miss is a BUG in the armor (or
in this arithmetic), never a modeling surprise. The M1 gate is "equal to the
byte, or the miss printed with its cause." Speed floor 0.25 MB/s; the monster
(rustc_driver.dll, 42.7 MB) <= 45 min end-to-end.

## M1 close (2026-09-02 17:30) -- written by the main session, not the agent

The agent that built M1 was terminated by the org's spend limit (HTTP 429) at
16:15 while closing. Every line below names its evidence (scratchpad close/ and
v12/); nothing here is the agent's claim.

- **Speed, SOLO** (nothing else running): wubbadub 92,408 B / 161 ms = 0.57 MB/s;
  alarm01.wav 491,516 / 1,306 ms = 0.38 MB/s (the lane figure 0.25 was
  contention); ntoskrnl.exe 13,047,280 / 134,152 ms = **0.097 MB/s** -- and v11
  solo the same day: 134,247 ms, the same 0.097. Not a v12 regression: a 13 MB
  row under the 16 MB frozen-elders law runs the whole roster. The home floor
  0.25 HOLDS (worst solo home row 0.38). The big-row MB/s in the lane table
  (0.06-0.22) were CONTENDED and are not speed measurements; the monster's solo
  wall time is UNMEASURED in v12 (v11: 40m23s).
- **Home tournament** (tools/standings.js, 14 rows, armor ON; flip / addressed
  scratch / truncation): all injuries EXACT; egg12 podium 13/14 (cbs.log to
  egg6+zstd 136,884 vs 148,090, as every version since v6); <= min(egg8, egg9,
  egg10) 14/14; vs gzip 14/14; vs egg6+zstd 13/14; vs naked xz -9 10/14 (an
  exhibit, not a bar). The first run died at 15:36 on 'xz' not found (a Windows
  spawn; xz lives in Git Bash's /mingw64/bin); rerun from bash 16:03-16:15.
- **Toolchain drift:** rustc/clippy 1.98.0 was installed at 15:40 today,
  mid-campaign. Under it `cargo clippy --all-targets -- -D warnings` on
  codegg-v12 raised 4 lints (armor.rs:108 chunks_exact_to_as_chunks;
  audit.rs:375 unnecessary_min_or_max; dyadic.rs:177/198 needless_late_init --
  those two are v11's coder verbatim, and codegg-v11 itself fails the same two
  under 1.98: frozen, printed, not touched). Fixed in v12, behavior-neutral: the
  rebuilt eggv12 writes byte-identical eggs (wubbadub.html and alarm01.wav
  cmp'd against the pre-fix eggs). After the fix: clippy EXIT 0; cargo test
  15/15; drills 143 passed, 0 failed; codegg-v11 drills 75/75 (ancestors green;
  untouched by mtime since the fork).
- **Not done at M1:** certutil countersign of the big arena (M4); solo monster
  wall time; the final README (the stub stands).

## M2a FILED (2026-09-02 17:45, BEFORE any M2a code) -- the precision debt

The brief is C:\Users\vcepe\.claude\plans\the-remainder-v12-m2.md. Measured on
v11's winning arms by the scratch probe (Mix11::learn counters, EGG_PROBE=1):
the mixer sits pinned at the +-2047 stretch clamp on 10-62% of decisions and
those decisions cost 0.009-0.016 bits each against an empirical miss rate worth
0.0004-0.0008. The bound below = paid-on-pinned minus the empirical entropy of
the pinned population (scratchpad probe/analyze.py). PREDICTED capture: 50-80%
of the bound, per row. The inner it moves is the M1 inner (the arm the M1
armored-total trial chose), so kernel32's bound is recomputed for its M1 arm
(CM11: paid 3,655 B, 287 wrong of 1,114,994 pinned -> ideal 479 -> 3,176; the
brief's 3,327 was CM11P's, the arm M0 assumed).

Design constants (filed here so the measurement cannot re-tune them):
coder PBITS 16, `bound = ((range as u64 * p as u64) >> 16) as u32`, p in
1..65535, range >= 2^24 kept (Mahoney zpaq's 16-bit coder, Knoll cmix); the
12-bit token models enter the SAME coder as p << 4; squash16 = 8,191 entries
round(65536 / (1 + e^(-x/256))) clamped [1, 65535] over x in [-4095, 4095]
(zpaq's 15-bit squash, one bit wider); stretch16[p] = round(256 ln(p/(65536-p)))
clamped +-4095 (exact at the tails, p = 0 guarded to -4095); mixer output
clamp +-4095; the mixer/ISSE learning shifts rise by 4 (err is now at 16-bit
scale: LR 11 -> 15, heavy 13 -> 17, ISSE 9 -> 13) so the v11 dynamics are kept
at the same operating points; StateMap p22 feeds stretch as p22 >> 6; the APM
entries become (p22 << 10 | count) with the StateMap's count-adaptive step
recip[n] = (1<<17)/(2n+3), a prior count of 4 (the identity init is worth four
observations) and a limit of 255 (steady rate ~1/256; v11: fixed 1/128 on
12-bit entries, which caps a bucket at 3,969/4,096 = 0.969 in steady state --
the reading of where the paid-on-pinned bytes went); 33 buckets over the
+-4095 domain (width 256, interpolation >> 8); final blends (3 pa + p) >> 2 at
16 bits; o0/mb/mm counters 16-bit at RATE 5; p is P(bit==0) everywhere. New
MODEL bytes 16 MIX12, 17 CM12, 18 MIX12P, 19 CM12P, 20 MIX12H, 21 CM12H; the
v11 arms (10..15) stay in the trial FROZEN and must write byte-identical eggs.

| row | M1 inner | M1 arm | bound | predicted delta (50-80%) | predicted inner |
|---|---|---|---|---|---|
| wubbadub.html | 23,099 | CM11P (13) | 140 | 70-112 | 22,987-23,029 |
| ring01.wav | 137,104 | MIX11 (10) | 3,027 | 1,514-2,422 | 134,682-135,590 |
| cbs.log | 142,188 | MIX11 (10) | ~8 (10 B paid on 21,803 pinned; not a precision row) | 0 | 142,188 |
| notepad.exe | 175,045 | CM11 (11) | 1,420 | 710-1,136 | 173,909-174,335 |
| real-test.bmp | 259,416 | MIX10 (8, frozen) | 0 | 0 unless a v12 arm takes the row | 259,416 |
| alarm01.wav | 263,764 | CM11 (11) | 2,106 | 1,053-1,685 | 262,079-262,711 |
| kernel32.dll | 291,184 | CM11 (11) | 3,176 (brief: 3,327 on CM11P) | 1,588-2,541 | 288,643-289,596 |
| vim-version9.txt | 308,884 | CM11H (15) | 10,791 | 5,396-8,633 | 300,251-303,488 |
| iconcache48.db | 414,406 | MIX11 (10) | 30 | 15-24 | 414,382-414,391 |
| segoeui.ttf | 417,854 | CM11 (11) | 2,041 | 1,021-1,633 | 416,221-416,833 |
| arial.ttf | 456,596 | CM11 (11) | 2,941 | 1,471-2,353 | 454,243-455,125 |
| zstd.exe | 509,765 | CM11 (11) | 5,069 | 2,535-4,055 | 505,710-507,230 |
| real-test.db | 1,223,539 | CM11H (15) | 92,216 | 46,108-73,773 | 1,149,766-1,177,431 |
| wallpaper.jpg | 1,511,982 | v8 MIX (5, frozen) | 0 | 0 unless a v12 arm takes the row | 1,511,982 |
| msgraph.dll | 4,558,852 | MIX11 (10) | 37,820 | 18,910-30,256 | 4,528,596-4,539,942 |
| mermaid-bundle.js | 4,827,729 | CM11H (15) | 397,131 (8.23%) | 198,566-317,705 | 4,510,024-4,629,163 |
| ntoskrnl.exe | 4,974,242 | CM10 (9, frozen) | 0 | 0 unless a v12 arm takes the row | 4,974,242 |
| aoe4-autosave.sav | 17,369,261 | MIX11 (10) | 13 | 0-13 | 17,369,248-17,369,261 |
| rdr2-shaders.vkcache | 41,881,828 | (arm unrecorded) | UNMEASURED: the probe never ran rdr2 | ~0 by class (near-entropy like the save, whose pinned share was 0.0%); filed as 0-5,000, a guess and marked so | 41,876,828-41,881,828 |
| rustc_driver.dll | 42,286,035 | MIX11 (10) | 38,364 (0.09%; filed 18:10 from the probe's SWEEP-DONE, before its run) | 19,182-30,691 | 42,255,344-42,266,853 |

Calls, in exact xz 5.8.3 bytes (the v11 residuals that M1's armor already
paid; here asked of the FORM alone): arial's M2a delta >= 1,576 needs 54% of
its bound -- CALLED YES; notepad's delta >= 1,176 needs 83% -- a coin flip
leaning MISS. Ratchet: every M2a total <= its M1 total (a v12 arm that loses
to a frozen arm leaves the frozen arm in place; printed, not hidden). Injuries
EXACT. M2a totals = new inner + the M1 (in-codeword) price at the tier the
inner lands on (4,920 / 5,902 / 6,384 / 8,420; a lighter inner can cross a
tier or CT boundary -- ledger12.js prints it).

KILL CRITERION (from the brief): if the measured capture is < 30% of the bound
on BOTH real-test.db (< 27,665 B) and vim (< 3,237 B), print the miss and stop
widening -- the debt is then in the APM shape, not the bits.

A reading, not a re-prediction: the bound counts PINNED decisions only. v11's
12-bit APM (fixed >> 7) cannot hold a bucket above 0.969 in steady state, so
every high-confidence decision -- pinned or not -- was taxed through the 3:1
blend. If that reading is right the capture can exceed 100% of the bound on the
CM rows; the prediction stays the brief's 50-80% and the excess, if any, is
printed as the miss it is.

### M2a amendment (filed 17:58, BEFORE the probe): the LZ arm regressed

First numbers from the built design (frozen arms byte-identical on wubbadub
and alarm01 under EGG_NO_V12; statehash enc == dec; restores EXACT):
wubbadub CM12P 22,906 vs CM11P 23,099 (-193 = 138% of the 140 bound) and
notepad CM12 173,195 vs CM11 175,045 (-1,850 = 130% of the 1,420 bound) --
ABOVE the bound, as the APM reading above said it could be. But the LZ arm
LOST: MIX12 26,395 vs MIX11 26,176 (+219) and 184,209 vs 182,783 (+1,426).
Same Mix12, same coder, the LZ arm's literals are the hard remainder that
lives mid-domain. Hypothesis: the APM lost mid-domain calibration -- its
buckets are 256 logit-units wide (half the paq/zpaq lineage's resolution)
and/or its count-adaptive step (prior 4, limit 255 = 1/256 steady against
v11's fixed 1/128) tracks the LZ arm's sparse literal contexts worse.

Probe, ONE knob each against the built A (33 buckets @256, prior 4, limit 255):
B = 65 buckets @128 over +-4095; C = 33 buckets @128 over +-2047 (the index
clamped, the mixer's +-4095 kept); D = limit 127; E = prior count 32. Rows:
wubbadub, notepad, ring01 (a MIX11 row), kernel32. DECISION RULE, stated
first: the variant with the smallest sum over the four rows of min(MIX12,
CM12) wins if it also leaves every CM12 within 0.1% of A's; otherwise A stays
and the MIX12 loss is printed as the miss it is. Prediction: B or C brings
MIX12 to within +-0.2% of MIX11 while CM12 stays within 0.1% of A; D and E
move less than 0.1%. The knob is a probe-only environment read and is
REMOVED (hard-coded) before the gate: nothing the decoder does may depend on
the environment.

## M2a MEASURED (2026-09-02 18:05) -- home rows; the range was MISSED on every row, upward

The 14-row ledger (tools/ledger12.js, the M2a build snapshotted as eggv12-m2a.exe,
8 lanes, 97 s wall for all fourteen rows; the rustc probe was still running, so
the MB/s column is contended and not a speed measurement).

MISSES FIRST. The filed capture range (50-80% of the bound) was missed on all
twelve measured rows -- every one landed ABOVE 100%: 148% (db) to 350% (zstd),
and cbs.log's inner fell by 53%. The cause is the reading filed beside the
prediction: the bound counted PINNED decisions only, but v11's 12-bit APM (fixed
>> 7) could not hold a bucket above 3,969/4,096 = 0.969, so the 3:1 blend taxed
EVERY high-confidence decision, pinned or not. On cbs.log the CM11 arm paid
~470 KB of that tax (129.5M decisions at ~0.03 bits) and lost to the LZ arm by
4x; CM12 pays ~none and takes the row outright at 67,017 -- lighter than naked
xz -9 (139,004) by 71,987 B. The M2c line/column lever was pre-shrunk to flip
cbs by 2.4%; the precision lever flipped it by 52% first. The lean on notepad
("coin flip leaning MISS") was wrong too: its delta 2,629 clears the 1,176.

| row | M1 inner (arm) | M2a inner (arm) | delta | bound | capture (pred. 50-80%) | M1 total | M2a total (tier/price) | ratchet | injuries |
|---|---|---|---|---|---|---|---|---|---|
| wubbadub.html | 23,099 (CM11P) | 22,886 (CM12P) | -213 | 140 | 152% MISS (above) | 28,019 | 27,806 (256, 4,920) | <= M1 | E/E/E |
| cbs.log | 142,188 (MIX11) | 67,017 (CM12) | -75,171 | ~8 | the CM arm took the row: not a capture, a flip | 148,090 | 72,401 (512, 5,384) | <= M1 | E/E/E |
| ring01.wav | 137,104 (MIX11) | 130,722 (CM12) | -6,382 | 3,027 | 211% MISS (above) | 143,006 | 136,106 (512, 5,384) | <= M1 | E/E/E |
| notepad.exe | 175,045 (CM11) | 172,416 (CM12) | -2,629 | 1,420 | 185% MISS (above) | 180,947 | 178,318 (512, 5,902) | <= M1 | E/E/E |
| real-test.bmp | 259,416 (MIX10 frozen) | 259,269 (MIX12) | -147 | 0 | "0 unless a v12 arm takes the row" -- it did: HIT | 265,318 | 265,171 (512, 5,902) | <= M1 | E/E/E |
| alarm01.wav | 263,764 (CM11) | 259,697 (CM12) | -4,067 | 2,106 | 193% MISS (above) | 270,148 | 265,599 (512, 5,902) | <= M1 | E/E/E |
| vim-version9.txt | 308,884 (CM11H) | 272,256 (CM12H) | -36,628 | 10,791 | 339% MISS (above) | 315,268 | 278,640 (1024, 6,384) | <= M1 | E/E/E |
| kernel32.dll | 291,184 (CM11) | 282,960 (CM12) | -8,224 | 3,176 | 259% MISS (above) | 297,568 | 289,344 (1024, 6,384) | <= M1 | E/E/E |
| segoeui.ttf | 417,854 (CM11) | 410,874 (CM12) | -6,980 | 2,041 | 342% MISS (above) | 424,238 | 417,258 (1024, 6,384) | <= M1 | E/E/E |
| iconcache48.db | 414,406 (MIX11) | 414,160 (MIX12) | -246 | 30 | 820% MISS (above) | 420,790 | 420,544 (1024, 6,384) | <= M1 | E/E/E |
| arial.ttf | 456,596 (CM11) | 448,478 (CM12) | -8,118 | 2,941 | 276% MISS (above) | 462,980 | 454,862 (1024, 6,384) | <= M1 | E/E/E |
| zstd.exe | 509,765 (CM11) | 492,042 (CM12) | -17,723 | 5,069 | 350% MISS (above) | 516,149 | 498,426 (1024, 6,384) | <= M1 | E/E/E |
| real-test.db | 1,223,539 (CM11H) | 1,087,216 (CM12H) | -136,323 | 92,216 | 148% MISS (above) | 1,231,959 | 1,095,636 (2048, 8,420) | <= M1 | E/E/E |
| wallpaper.jpg | 1,511,982 (v8 MIX frozen) | 1,511,982 (v8 MIX frozen) | 0 | 0 | 0, the frozen arm kept the row: HIT | 1,520,402 | 1,520,402 (2048, 8,420) | <= M1 | E/E/E |

Sum of the deltas: -302,851 B against a summed bound of 122,965 B (246%). Two
tier moves (ring01, cbs.log from blk 512/CT 2 squares to 512/CT 1 square: the
price fell 5,902 -> 5,384 because the inner crossed a CT boundary downward).

The calls: arial's delta 8,118 >= 1,576 -- CALLED YES, HIT. notepad's delta
2,629 >= 1,176 -- the coin flip fell to YES (the filed lean was MISS: printed).
KILL CRITERION: db 148% and vim 339% of the bound -- not triggered by a mile.
Ratchet: 14/14 <= M1 (and so <= v11 sealed and <= min(v8..v11)). Injuries
42/42 EXACT. The frozen arms write byte-identical eggs (wubbadub, alarm01 under
EGG_NO_V12 against the M1-close eggs). Mirror: EGG_STATEHASH's decoder hash is
among the roster's encoder hashes on wubbadub / ring01 / notepad (11 / 25 / 22
arms). cargo test 20/20; clippy -D warnings clean in both profiles.

### The APM probe (the amendment above), measured

| variant | wubbadub CM12 / MIX12 | notepad CM12 / MIX12 | ring01 CM12 / MIX12 | kernel32 CM12 / MIX12 | sum of the four winners |
|---|---|---|---|---|---|
| A 33 @256 (built) | 23,347 / 26,395 | 173,195 / 181,026 | 131,924 / 146,103 | 284,259 / 312,326 | 612,284 |
| B 65 @128 | 23,296 / 26,197 | 172,438 / 179,677 | 130,704 / 145,273 | 282,983 / 309,941 | 609,037 |
| **C 33 @128 over +-2047** | 23,288 / 26,195 | 172,416 / 179,676 | 130,722 / 145,270 | 282,960 / 309,937 | **608,984** |
| D limit 127 | 23,370 / 26,389 | 173,108 / 180,913 | 132,080 / 146,186 | 284,257 / 312,181 | 612,372 |
| E prior count 32 | 23,283 / 26,344 | 173,120 / 180,881 | 131,882 / 146,066 | 284,163 / 312,162 | 611,959 |

(MIX11 for reference: 26,176 / 179,651 / 147,016 / 310,251; the wubbadub
winner is CM12P, listed as the row's winner in the sum.) The rule picked C
(B within 53 B): MIX12 is within +0.07% / +0.01% of MIX11 on wubbadub / notepad
and beats it on ring01 / kernel32; every CM12 improved a further 0.2-0.9%. The
probe prediction held for B/C/D; E moved wubbadub by 0.5%, not "< 0.1%" -- a
miss of the probe's own prediction, printed. C is hard-coded; the environment
knob is gone.

Big rows: predicted above (M2a FILED); measured in the M2a big run (the same
snapshot exe, six rows in lanes) and printed in the M2b ledger section below.

## M2b FILED (2026-09-02 18:12, BEFORE any armor code) -- the columns agree

Placement "none": no residue table at all. P = t + 1 parity squares
(t = dead(blk) = ceil(4096/blk) + 1, so P = 18 / 10 / 6 / 4 at 256 / 512 /
1024 / 2048), meta = fnv32 of nothing, sites 3 x (64 + 4) = 204 B. A wounded
square is an error at the SAME position in every one of the blk/2 interleaved
codewords, so their syndromes share one locator (Krachkovsky & Lee 1997;
Bleichenbacher, Kiayias & Yung 2003) -- M1's ladder step (3a) already does
this; the residues were paying 2 B/square to locate what the codewords locate
for free. The price becomes FLAT per tier and independent of the square count:

| tier | P | price = parity + 204 | M1 price | delta |
|---|---|---|---|---|
| 256 | 18 | 4,608 + 204 = **4,812** | 4,920 | -108 |
| 512 | 10 | 5,120 + 204 = **5,324** | 5,902 | -578 |
| 1024 | 6 | 6,144 + 204 = **6,348** | 6,384 | -36 |
| 2048 | 4 | 8,192 + 204 = **8,396** | 8,420 + 2 B/square | rdr2 47,446 -> 6,348 |

Because total = inner + price EXACTLY (the short last square costs nothing
extra) and the price no longer grows with the square count, the tier argmin
takes the smallest square whose codeword fits: every inner <= 16,772,352 B
lands on 256 at 4,812; <= 33,548,800 B on 512 at 5,324; else 1024 at 6,348.
The armored-total trial therefore collapses to the MIN INNER over the roster,
which on all fourteen home rows is the M2a inner (the arms dumps confirm: no
arm was passed over for a square boundary). The powers, as `info` prints them
per placement: NAMED erasures (--wound, truncation) <= P: CERTAIN; BLIND errors
<= floor(P/2): CERTAIN (Berlekamp-Massey); blind up to P-1: located jointly,
succeeds iff the error rows are independent (overwhelming for compressed
squares), failure = REFUSE with the number; blind P: REFUSES. A contiguous
blind wound of 4,096 B straddles <= dead(blk) = P-1 squares: located jointly.
The residue placements stay selectable (`--judge` forces the argmin over them;
`--ct triple|incw|none` forces one).

One rung is added to the ladder, a deviation from the brief filed here: when
the codeword refuses, the DATA SQUARES AS RECEIVED are hashed; if the FNV-64
of the inner matches, the data is intact and the damage was confined to
parity (or CT). Hash-gated (2^-64), never wrong, and it makes "all P parity
squares dead, blind" EXACT under every placement (the v8 lesson: the armor was
righter than the drill).

| row | M2a inner (= the min over the roster) | tier | P | price | x floor | **PREDICTED total** | M1 total | vs M1 |
|---|---|---|---|---|---|---|---|---|
| wubbadub.html | 22,886 | 256 | 18 | 4,812 | 1.17x | **27,698** | 28,019 | -321 |
| cbs.log | 67,017 | 256 | 18 | 4,812 | 1.17x | **71,829** | 148,090 | -76,261 |
| ring01.wav | 130,722 | 256 | 18 | 4,812 | 1.17x | **135,534** | 143,006 | -7,472 |
| notepad.exe | 172,416 | 256 | 18 | 4,812 | 1.17x | **177,228** | 180,947 | -3,719 |
| real-test.bmp | 259,269 | 256 | 18 | 4,812 | 1.17x | **264,081** | 265,318 | -1,237 |
| alarm01.wav | 259,697 | 256 | 18 | 4,812 | 1.17x | **264,509** | 270,148 | -5,639 |
| vim-version9.txt | 272,256 | 256 | 18 | 4,812 | 1.17x | **277,068** | 315,268 | -38,200 |
| kernel32.dll | 282,960 | 256 | 18 | 4,812 | 1.17x | **287,772** | 297,568 | -9,796 |
| segoeui.ttf | 410,874 | 256 | 18 | 4,812 | 1.17x | **415,686** | 424,238 | -8,552 |
| iconcache48.db | 414,160 | 256 | 18 | 4,812 | 1.17x | **418,972** | 420,790 | -1,818 |
| arial.ttf | 448,478 | 256 | 18 | 4,812 | 1.17x | **453,290** | 462,980 | -9,690 |
| zstd.exe | 492,042 | 256 | 18 | 4,812 | 1.17x | **496,854** | 516,149 | -19,295 |
| real-test.db | 1,087,216 | 256 | 18 | 4,812 | 1.17x | **1,092,028** | 1,231,959 | -139,931 |
| wallpaper.jpg | 1,511,982 | 256 | 18 | 4,812 | 1.17x | **1,516,794** | 1,520,402 | -3,608 |

Sum over the 14 home rows: M1 6,224,882 -> M2a 5,920,513 -> M2b **5,899,343** (-325,539 vs M1).

Big rows: the price is exact now -- msgraph.dll, mermaid-bundle.js,
ntoskrnl.exe 4,812 (blk 256, P 18: their inners are 4.5-5.0 MB);
aoe4-autosave.sav 5,324 (512, P 10: 17.4 MB); rdr2-shaders.vkcache and
rustc_driver.dll 6,348 (1024, P 6: 41.9 / 42.3 MB). Their predicted totals are
that price plus the inner the M2a snapshot exe (eggv12-m2a.exe, a build with
no armor change in it) measures in the M2a big run now starting; the six
numbers are appended to this section when that run lands and BEFORE the M2b
ledger runs. (The in-codeword CT bands are flat across every plausible M2a
delta on all six rows, so the M2a arm pick is the min-inner pick there too.)

New drills (tools/drills.js), predictions stated: blind P-1 squares scribbled
at random -> EXACT; blind P (with >= 1 data square) -> HONEST; addressed P ->
EXACT; the RANK TRAP on a crafted identity-form file at --tier 2048 (P = 4):
two data squares with identical content scribbled identically, blind ->
HONEST predicted (the two error rows span one dimension; the joint locator
finds no position, and Berlekamp-Massey's degree-2 locator is refused by the
2 deg >= m guard) -- EXACT would also be lawful, SILENT never; controls: the
same two squares scribbled differently, blind -> EXACT (k = 2 < m = 4); three
different -> EXACT; four blind -> HONEST; the rank trap named -> EXACT. Every
M1 drill runs again under the default placement (none) AND under --judge (the
residue placements), with expectations derived per placement; the audit gains
the joint-locator round-trips per class and the P+1 refusal for none.

### M2b big rows, FILED (2026-09-02 22:20, BEFORE the M2b ledger) -- by the main session

The M2b agent was terminated by the org spend limit at ~19:00 after building
M2b (audit v4 3,091,667 checks passing; drills 255 passed, 2 FAILED -- see the
ledger section) and before this table. The inners are the M2a big run's
(scratchpad v12m2/ledger-big-m2a.txt, the M2a snapshot exe, no armor change);
the price is the flat M2b price for the tier the argmin must pick.

| row | M2a inner | tier | P | price | x floor | **PREDICTED total** |
|---|---|---|---|---|---|---|
| msgraph.dll | 3,325,023 | 256 | 18 | 4,812 | 1.17x | **3,329,835** |
| mermaid-bundle.js | 4,117,782 | 256 | 18 | 4,812 | 1.17x | **4,122,594** |
| ntoskrnl.exe | 4,800,941 | 256 | 18 | 4,812 | 1.17x | **4,805,753** |
| aoe4-autosave.sav | 17,363,293 | 512 | 10 | 5,324 | 1.30x | **17,368,617** |
| rdr2-shaders.vkcache | 41,872,644 | 1024 | 6 | 6,348 | 1.55x | **41,878,992** |
| rustc_driver.dll | 38,814,313 | 1024 | 6 | 6,348 | 1.55x | **38,820,661** |

M2a big-row deltas vs v11 sealed (measured 18:58, before M2b): msgraph
-1,282,163 (-2.965 pt), mermaid -762,824 (-2.952 pt), ntoskrnl -226,103,
sav -165,695, rdr2 -392,310, rustc -3,860,247 (-2.108 pt); net -6,689,342 B;
injuries 18/18 EXACT. Against the M2a FILED bounds: mermaid's 397,131 bound
captured 179%, msgraph's 37,820 bound 3,262% (the LZ arm gave way to CM12 on
a PE), ntoskrnl "0 unless a v12 arm takes the row" -- CM12 took it (-173,301
inner), sav/icon-class rows MIX12 -5,968 on a ~13 B bound. Every big row
missed its range UPWARD, the same cause as the home rows.

## M2b MEASURED (2026-09-02 23:03) -- the columns agree; written by the main session

The 20-row ledger (tools/ledger12.js, EGG_PRED = the filed totals, 8 lanes;
MB/s contended, not a speed measurement). The M2b agent died on the org spend
limit before this run; the two drill FAILs it left were the rank-trap drill
not forcing `--ct none` (at --tier 2048 on a 100-square identity file the argmin
rightly prefers triple, ~6,978 B, over none, 8,396 B) -- fixed in tools/drills.js,
and the battery reruns **257 passed, 0 failed** (four data squares named ->
EXACT; four blind -> HONEST; the RANK TRAP scribbled identically -> HONEST as
predicted, never wrong). clippy -D warnings clean; cargo test 20/20; audit v4
3,091,667 checks passing; ancestors and site manifests unchanged.

MISSES FIRST: 0 of 20 rows off the filed total -- every row to the byte.
Hits to the byte: **20/20**. Injuries: 60/60 EXACT.
Rows heavier than M1: 0. Net vs M1: **-6,056,416 B**. Net vs v11 sealed: **-7,199,797 B** (v11's whole campaign was -1,766,332).
Exhibit (armored egg12 vs NAKED xz -9, not a bar): 17/20 lighter.

| row | predicted | measured | delta | placement / tier / P | price | x floor | M1 total | vs M1 | vs v11 sealed | injuries | MB/s (lanes) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| wubbadub.html | 27,698 | **27,698** | HIT | none / 256 / 18 | 4,812 | 1.17x | 28,019 | -321 | -2,898 | E/E/E | 0.25 |
| cbs.log | 71,829 | **71,829** | HIT | none / 256 / 18 | 4,812 | 1.17x | 148,090 | -76,261 | -78,975 | E/E/E | 0.96 |
| ring01.wav | 135,534 | **135,534** | HIT | none / 256 / 18 | 4,812 | 1.17x | 143,006 | -7,472 | -10,650 | E/E/E | 0.21 |
| notepad.exe | 177,228 | **177,228** | HIT | none / 256 / 18 | 4,812 | 1.17x | 180,947 | -3,719 | -5,832 | E/E/E | 0.26 |
| real-test.bmp | 264,081 | **264,081** | HIT | none / 256 / 18 | 4,812 | 1.17x | 265,318 | -1,237 | -4,507 | E/E/E | 0.35 |
| alarm01.wav | 264,509 | **264,509** | HIT | none / 256 / 18 | 4,812 | 1.17x | 270,148 | -5,639 | -8,687 | E/E/E | 0.16 |
| vim-version9.txt | 277,068 | **277,068** | HIT | none / 256 / 18 | 4,812 | 1.17x | 315,268 | -38,200 | -42,196 | E/E/E | 0.28 |
| kernel32.dll | 287,772 | **287,772** | HIT | none / 256 / 18 | 4,812 | 1.17x | 297,568 | -9,796 | -13,060 | E/E/E | 0.24 |
| segoeui.ttf | 415,686 | **415,686** | HIT | none / 256 / 18 | 4,812 | 1.17x | 424,238 | -8,552 | -13,682 | E/E/E | 0.30 |
| iconcache48.db | 418,972 | **418,972** | HIT | none / 256 / 18 | 4,812 | 1.17x | 420,790 | -1,818 | -5,788 | E/E/E | 0.90 |
| arial.ttf | 453,290 | **453,290** | HIT | none / 256 / 18 | 4,812 | 1.17x | 462,980 | -9,690 | -15,002 | E/E/E | 0.30 |
| zstd.exe | 496,854 | **496,854** | HIT | none / 256 / 18 | 4,812 | 1.17x | 516,149 | -19,295 | -24,686 | E/E/E | 0.31 |
| real-test.db | 1,092,028 | **1,092,028** | HIT | none / 256 / 18 | 4,812 | 1.17x | 1,231,959 | -139,931 | -149,348 | E/E/E | 0.20 |
| wallpaper.jpg | 1,516,794 | **1,516,794** | HIT | none / 256 / 18 | 4,812 | 1.17x | 1,520,402 | -3,608 | -16,434 | E/E/E | 0.21 |
| msgraph.dll | 3,329,835 | **3,329,835** | HIT | none / 256 / 18 | 4,812 | 1.17x | 4,571,380 | -1,241,545 | -1,287,825 | E/E/E | 0.37 |
| mermaid-bundle.js | 4,122,594 | **4,122,594** | HIT | none / 256 / 18 | 4,812 | 1.17x | 4,840,257 | -717,663 | -768,486 | E/E/E | 0.09 |
| ntoskrnl.exe | 4,805,753 | **4,805,753** | HIT | none / 256 / 18 | 4,812 | 1.17x | 4,986,770 | -181,017 | -233,819 | E/E/E | 0.05 |
| aoe4-autosave.sav | 17,368,617 | **17,368,617** | HIT | none / 512 / 10 | 5,324 | 1.30x | 17,394,113 | -25,496 | -185,223 | E/E/E | 0.19 |
| rdr2-shaders.vkcache | 41,878,992 | **41,878,992** | HIT | none / 1024 / 6 | 6,348 | 1.55x | 41,929,274 | -50,282 | -433,408 | E/E/E | 0.10 |
| rustc_driver.dll | 38,820,661 | **38,820,661** | HIT | none / 1024 / 6 | 6,348 | 1.55x | 42,335,535 | -3,514,874 | -3,899,291 | E/E/E | 0.07 |

## M4 MEASURED (2026-09-03) -- the final 20-row ledger, the tournaments, the rivals

Agent 3 died on the org spend limit around midnight, after launching these runs
and drafting the README; they completed unattended and the main session read
them off disk and wrote this section. Every number below names its file in
scratchpad/v12m3/.

MISSES FIRST -- **6 of 20 rows off the filed total** (ledger-final.txt):

| row | called | measured | delta | reading |
|---|---|---|---|---|
| rustc_driver.dll | 38,238,446 | 37,436,978 | **-801,468** | the dense-binary band (-1.5%) was called on 4 MB PEs; at 183 MB the checksummed slots pay far more (-3.6% of the inner). The biggest single miss of the campaign, and it is ours. |
| msgraph.dll | 3,279,960 | 3,211,880 | -68,080 | same cause, -2.1% against a -1.5% call |
| ntoskrnl.exe | 4,733,739 | 4,675,387 | -58,352 | same cause, -2.7% against -1.5% |
| aoe4-autosave.sav | 17,351,254 | 17,333,845 | -17,409 | the deflate bytes DID share tables: -0.20% against a -0.1% call |
| mermaid-bundle.js | 4,097,887 | 4,090,958 | -6,929 | JS text, -0.77% against -0.6% |
| rdr2-shaders.vkcache | 41,753,374 | 41,860,990 | **+107,616 HEAVIER** | the only row that came in above its call. The shader ISA moved -0.04% under the slots, not the -0.3% called: its contexts were never colliding (LZ carries the row, MIX12 holds it). The band was read off dense binaries; a dense stream that the parse already covers does not behave like one. |

Five misses light, one heavy, none silent. The arithmetic never moved: **total -
inner = price on 20 of 20 rows, to the byte** (verified in this script).
Ratchet: **breached on ring01.wav by 31 bytes** (135,534 -> 135,565) and held on the
other 19. The 31 B is lever (b)'s cost on a row whose contexts never collided --
inside (b)'s own filed band for that row (-0.3% .. +0.3%, so the lever HIT its
call) and paid for many times over by the same lever's -57,652 B across the home
rows. The gate as briefed said 'no row heavier than M2b'; this is the one row, and
it is printed rather than repriced. Against the series ratchet that matters --
v12 <= min(v8..v11) -- every row holds (ring01: 135,565 vs v11's 146,184).
Every row lighter than v11 sealed.
Net: **-8,987,397 B vs v11 sealed**, -7,844,016 vs M1, -1,787,600 vs M2b.

| row | **total** | predicted | verdict | inner (arm) | price | x floor | vs M2b | vs v11 sealed | naked xz -9 | exhibit | injuries |
|---|---|---|---|---|---|---|---|---|---|---|---|
| wubbadub.html | **27,621** | 27,621 | HIT | 22,809 (CM12P) | 4,812 | 1.17x | -77 | -2,975 | 26,180 | LOSS +1,441 | E/E/E |
| cbs.log | **71,758** | 71,758 | HIT | 66,946 (CM12) | 4,812 | 1.17x | -71 | -79,046 | 139,004 | win | E/E/E |
| ring01.wav | **135,565** | 135,565 | HIT | 130,753 (CM12) | 4,812 | 1.17x | +31 | -10,619 | 248,424 | win | E/E/E |
| notepad.exe | **176,164** | 176,164 | HIT | 171,352 (CM12B (PE book)) | 4,812 | 1.17x | -1,064 | -6,896 | 181,884 | win | E/E/E |
| alarm01.wav | **252,862** | 252,862 | HIT | 248,050 (CM12) | 4,812 | 1.17x | -11,647 | -20,334 | 344,640 | win | E/E/E |
| real-test.bmp | **261,274** | 261,274 | HIT | 256,462 (MIX12) | 4,812 | 1.17x | -2,807 | -7,314 | 3,612,796 | win | E/E/E |
| vim-version9.txt | **273,982** | 273,982 | HIT | 269,170 (CM12H) | 4,812 | 1.17x | -3,086 | -45,282 | 371,088 | win | E/E/E |
| kernel32.dll | **283,604** | 283,604 | HIT | 278,792 (CM12B (PE book)) | 4,812 | 1.17x | -4,168 | -17,228 | 316,416 | win | E/E/E |
| segoeui.ttf | **409,683** | 409,683 | HIT | 404,871 (CM12B (TTF book)) | 4,812 | 1.17x | -6,003 | -19,685 | 422,304 | win | E/E/E |
| iconcache48.db | **418,323** | 418,323 | HIT | 413,511 (MIX12) | 4,812 | 1.17x | -649 | -6,437 | 416,968 | LOSS +1,355 | E/E/E |
| arial.ttf | **446,354** | 446,354 | HIT | 441,542 (CM12B (TTF book)) | 4,812 | 1.17x | -6,936 | -21,938 | 466,716 | win | E/E/E |
| zstd.exe | **488,915** | 488,915 | HIT | 484,103 (CM12B (PE book)) | 4,812 | 1.17x | -7,939 | -32,625 | 534,572 | win | E/E/E |
| real-test.db | **1,068,149** | 1,068,149 | HIT | 1,063,337 (CM12H) | 4,812 | 1.17x | -23,879 | -173,227 | 1,474,592 | win | E/E/E |
| wallpaper.jpg | **1,513,903** | 1,513,903 | HIT | 1,509,091 (CM12) | 4,812 | 1.17x | -2,891 | -19,325 | 1,571,824 | win | E/E/E |
| msgraph.dll | **3,211,880** | 3,279,960 | MISS -68,080 | 3,207,068 (CM12B (PE book)) | 4,812 | 1.17x | -117,955 | -1,405,780 | 4,749,376 | win | E/E/E |
| mermaid-bundle.js | **4,090,958** | 4,097,887 | MISS -6,929 | 4,086,146 (CM12) | 4,812 | 1.17x | -31,636 | -800,122 | 4,952,132 | win | E/E/E |
| ntoskrnl.exe | **4,675,387** | 4,733,739 | MISS -58,352 | 4,670,575 (CM12B (PE book)) | 4,812 | 1.17x | -130,366 | -364,185 | 5,501,588 | win | E/E/E |
| aoe4-autosave.sav | **17,333,845** | 17,351,254 | MISS -17,409 | 17,328,521 (MIX12) | 5,324 | 1.30x | -34,772 | -219,995 | 16,739,920 | LOSS +593,925 | E/E/E |
| rustc_driver.dll | **37,436,978** | 38,238,446 | MISS -801,468 | 37,430,630 (CM12B (PE book)) | 6,348 | 1.55x | -1,383,683 | -5,282,974 | 41,707,380 | win | E/E/E |
| rdr2-shaders.vkcache | **41,860,990** | 41,753,374 | MISS +107,616 | 41,854,642 (MIX12) | 6,348 | 1.55x | -18,002 | -451,410 | 41,969,640 | win | E/E/E |

### The tournaments, the rivals, the fingerprints

- **Home tournament** (standings-home.txt, 14 rows, armor ON, three injuries each):
  podium **egg12 x14**; <= min(egg8, egg9, egg10) **14/14**; vs gzip -9 14/14;
  vs the egg6+zstd hybrid **14/14** (cbs.log, the one row the hybrid had held
  since v6, fell this milestone: 71,758 vs 136,884); vs naked xz -9 12/14 (an
  exhibit, not a bar). All injuries EXACT.
- **The formats card** (standings-fmt.txt, 3 rows): podium egg12 x3; ratchet 3/3;
  vs naked xz 3/3. msgraph-docs.xml 85.5 MB -> **965,799 B (1.1%)**, embeddings.json
  4,687,145 (29.7%), changelog.md 179,700 (14.9%) -- the CHANGELOG that lost to
  naked xz in v11 by 6,988 B now wins by 35,539.
- **The challengers** (challengers.txt, 23 rows, three corpora, same injuries):
  **rar -rr5 forfeits truncation on 23 of 23** (E/E/x; wubbadub E/x/x) -- the
  same structural death as v11. **xz+par2 survives everything and loses to
  egg12 on 22 of 23 rows.** The one standing row is the game save (-0.20 pt,
  ~17,202,144 vs 17,333,845; v11's margin was -0.53 pt). The bar wanted 23/23:
  **MISSED BY ONE ROW, the same row as v11's**, and the cause is now named (the
  gzip skin, M2c(a)). Percentages are printed to 0.1%; the save's gap (131,701 B)
  and every win's margin exceed that granularity.
- **Countersign** (countersign-big.txt): certutil fingerprints on the 8 big-arena
  rows, **8/8 FINGERPRINT MATCH** after transmute -> restore.
- **Drills** (drills-final.txt): **257 passed, 0 failed**, including the rank trap
  and the P+1 refusals under all three placements. Audit v4: 3,091,667 checks,
  0 failing. clippy -D warnings clean; cargo test 22/22.

### The currencies, measured

- **Form vs form** (our inner vs naked xz -9's stream, xz 5.8.3, exact bytes):
  **19/20**. The one loss is the save: inner 17,328,521 vs 16,739,920 (+588,601,
  3.5%), diagnosed in M2c(a) as the gzip skin and filed as v13's line.
- **The exhibit** (armored total vs naked xz -9; never a bar): **17/20** lighter.
  The three losses: the save +593,925, wubbadub.html +1,441 (a 92 KB page paying
  4,812 B of armor: 1.17x the pigeonhole floor), iconcache48.db +1,355.
  For scale, the armor those rows carry is 4,812 B; xz carries none.
- **Price vs floor**: flat per tier, and the tier is chosen by the codeword's
  reach alone -- 4,812 B (256 B squares, 18 parity: 17 rows), 5,324 (512, 10:
  the save), 6,348 (1024, 6: rdr2 and rustc). 1.17x / 1.30x / 1.55x the 4,096 B
  pigeonhole floor. v11's price on rustc was 433,917 B; v12's is 6,348.

### Speed, SOLO (2026-09-03, machine idle, one transmute at a time)

The lane MB/s printed in every ledger above is CONTENDED (8 lanes) and is not a
speed measurement. These are:

| row | bytes | wall | MB/s |
|---|---|---|---|
| ring01.wav (the worst home row) | 498,420 | 1.80 s | **0.277** |
| kernel32.dll | 836,208 | 2.98 s | 0.281 |
| notepad.exe | 360,448 | 1.19 s | 0.302 |
| alarm01.wav | 491,516 | 1.60 s | 0.307 |
| segoeui.ttf / arial.ttf / wallpaper.jpg | 959,752 / 1,045,720 / 1,602,752 | 2.53 / 2.73 / 4.20 s | 0.380 / 0.382 / 0.382 |
| real-test.db | 9,551,872 | 24.58 s | 0.389 |
| wubbadub.html | 92,408 | 0.23 s | 0.398 |
| zstd.exe | 1,601,409 | 3.48 s | 0.460 |
| vim-version9.txt | 2,035,039 | 4.08 s | 0.499 |
| real-test.bmp | 12,000,054 | 20.68 s | 0.580 |
| iconcache48.db | 97,517,568 | 69.08 s | 1.412 |
| cbs.log | 16,187,036 | 9.99 s | 1.621 |
| **rustc_driver.dll (the monster)** | 183,111,168 | **40m56.9s** | 0.075 |

Home floor 0.25 MB/s: **MET** (worst 0.277). Monster bar 45 min: **MET**
(40m56.9s; v11 40m23s). The solo monster reproduced the lane ledger's total to
the byte (37,436,978), which is also the ledger's own control on the lanes.

## The ancestors: untouched, and the one dated correction (2026-09-03)

Recorded BEFORE the edit below (scratchpad v12m4/ancestor-proof.txt): across
codec-v1 and codegg-v1 .. codegg-v11, **0 files outside `target/` are newer than
codegg-v12's fork**. No site file was modified: `inspirations.html` and the
repo-root `README.md` carry mtimes of 2026-09-02 22:50, before this campaign's
last agent started, and both are content-identical to git HEAD.

The campaign then made exactly ONE edit to an ancestor -- a dated reporting note,
with no value rewritten:

| file | what |
|---|---|
| `codegg-v11/README.md` | a blockquote after "The failures and reverts, first": ntoskrnl.exe's sealed 5,039,572 predates the 09:46 edit raising the frozen-elders law to 16 MB; the shipped eggv11.exe produces 5,038,548 (-1,024, one square); the sealed figure is left as written |
| `codegg-v11/ledger-m8-sealed.txt` | the same note as three `#` comment lines at the head; every recorded byte untouched |

Found by v12-M0 when its fork reproduced 19 of 20 sealed totals to the byte and
missed that one. codegg-v11's drill battery was re-run after the edit and is
green (75/75); the v11 binary and sources are otherwise byte-untouched.

## Attribution

Reed & Solomon 1960 (the code); Berlekamp-Massey (the locator); Forney (the
values); Chien (the search); the BCH-view generator polynomial (the literal
division, spec.md:134-156); the searched grid (spec.md:166); refusing with a
number -> promising with a number (spectrometer.html:396); column by column
(wubbadub.html:698); kept rather than rounded away (glossary.js:164); Winning
Ways ordering (refs/README.md:43). v6..v11 supplied the sites, the residues, the
injuries and the ledger discipline.

## M2c FILED (2026-09-02 23:16, BEFORE any M2c code) -- the model's remaining debts

Agent 3 of the campaign (agents 1 and 2 died on the org spend limit). Each
lever below is filed, then built alone, then judged on a 14-row home ledger
(EGG_PRED) and kept only on a net corpus win; a loser is deleted, not shipped
dormant. Form vs form throughout; the M2b inners are the baseline.

### (a) The save -- diagnosis first (measured 23:14, no code built)

aoe4-autosave.sav (66,417,543 B) is ONE GZIP MEMBER: header 1f 8b 08 00,
mtime 0, xfl 0, OS 0x0a; it inflates (python zlib) to 296,540,843 B of
Relic Chunky ("Relic Chunky\r\n\x1a", DATASDSC) at order-0 4.826 bits/byte,
31.4% zeros; the deflate stream itself is 7.998 bits/byte order-0. xz cuts
the deflate stream to 25% only because the compressed bytes REPEAT at long
range (chunks re-deflated from identical content). The xz knobs, exact bytes
(xz 5.8.3), all on the sav:

| xz setting | bytes | reading |
|---|---|---|
| -6 (dict 8 MiB) | 17,790,636 | our M2b inner 17,363,293 beats this |
| -7 (dict 16 MiB) | 17,528,920 | and this |
| -8 (dict 32 MiB) | 16,873,176 | and loses to this by 490,117 |
| -9 (dict 64 MiB) | 16,739,920 | the rival's stream; the debt is 623,373 (3.59%) |
| -9e | 16,706,932 | |
| preset 9, mf=hc4 depth 1024 | 16,740,280 | a HASH-CHAIN finder ties bt4: finder depth is NOT the gap |
| preset 9, mode=fast | 17,069,712 | the optimal parse is worth 329,792 to xz |
| preset 9, pb=0 / lc=4,pb=0 / lc=0,pb=0 | 16,718,972 / 16,709,316 / 16,749,064 | alignment contexts do not matter here |
| preset 9, nice=273 | 16,706,532 | |

Reading: the dictionary spread 8 -> 64 MiB is 1,050,716 B for xz; our LZ
sees the whole file in principle but (token.rs) hashes 66 M positions into
2^20 buckets (hash_bits_for = bits(n) - 6, clamped 18..23 -> 20 here), so a
repeat at distance D has ~D/2^20 colliders ahead of it in the nearest-first
chain (up to 63 at 66 MB) while the cliff budget (armed >= 16 MB) halves
the walk cap toward its floor of 64 on data this dense, and GOOD_LEN = 128
ends the walk at the first 128-byte match. Far repeats are therefore lost
probabilistically past ~32 MB and long ones are cut short. The match model
(mix11.rs HT_BITS 22 = 4 M slots for 66 M positions) is the secondary
suspect; the parse (greedy + two-step lazy vs LZMA's optimal) is the tertiary
and is NOT in scope (the DP was killed in v11 by its own criterion).

PREDICTION for the probe (a probe build in scratch, knobs read from the
environment there only; nothing in codegg-v12/src changes until a lever is
chosen): raising the hash to 2^24 buckets (4 per bucket), lifting GOOD_LEN and
the budget for the MIX12 arm on the sav ALONE moves the inner by -150,000 to
-400,000 B (0.9-2.3%); the form-vs-form debt (623,373) is NOT closed by reach
alone (the parse's 330 KB is xz's, not ours). Raising HT_BITS to 24 on top
moves < 30,000 B more. Solo time of the MIX12 arm on the sav stays under
6 minutes. Kept only if the whole-roster sav total falls >= 0.5% (>= 86,843 B)
with no home row heavier; shipped, if kept, as a SIZE-CLASS law (inputs
>= 16 MB), like v11's 16 MB frozen-elders law.

The gzip peel (inflate at transmute, re-deflate byte-exactly at restore) is
FILED AS A READING, not built: it needs a byte-exact deflate encoder (the
compressor is unknown; a zlib-level reproduction test runs beside this probe
and its result is printed below), a std-only port of zlib's deflate, and a
296 MB inner through the roster. v13's line, with the numbers measured here.

### (a) MEASURED (23:19) -- the reach hypothesis is REFUTED; the miss printed

Seven probe runs of the MIX12 arm alone on the sav (scratch build, knobs from
the environment there; codegg-v12/src untouched). Tokenize times are contended
(seven runs at once) and are not speed measurements:

| variant | tokens | match cover | MIX12 inner (after 2nd pass) | delta vs M2b | tokenize |
|---|---|---|---|---|---|
| A baseline (hb 20, cap 1024, budget on, GOOD 128) | 1,212,301 matches | 78.34% | 17,363,293 | 0 (reproduces M2b to the byte) | 68.4 s |
| B hb 24 | IDENTICAL token stream to A | 78.34% | 17,363,293 | 0 | 9.2 s |
| F hb 22 | IDENTICAL to A | 78.34% | 17,363,293 | 0 | 21.0 s |
| G GOOD_LEN off | 1,210,734 matches | 78.34% | 17,356,735 | -6,558 (0.04%) | 68.7 s |
| C hb 24 + GOOD off + budget off | = G's tokens | 78.34% | 17,356,735 | -6,558 | 9.4 s |
| D = C + cap 4096 | 1 byte moved | 78.34% | 17,356,734 | -6,559 | 9.3 s |
| E = C + match-model HT 2^24 | = C's tokens | 78.34% | 17,352,724 | -10,569 (0.06%) | 9.3 s |

MISS: predicted -150,000 to -400,000; measured -6,558 (reach) / -10,569 (with
the match model). The tokenizer already finds every far repeat the wider hash
finds -- the slot histogram is identical bucket for bucket at 2^20 and 2^24 --
so the budget and the collider count were never cutting true matches on this
file; GOOD_LEN cut 1,567 long matches short for 6.5 KB. The debt is not
reach. What remains is xz's optimal parse (329,792 by xz's own mode=fast
control) and ~290 KB of token/literal coding on a stream whose literals are
Huffman-coded deflate bytes. NOT SHIPPED (0.04% << 0.5%); the loser is
deleted with the scratch crate. Two readings kept:

1. **hb 24 is a speed lever with identical tokens**: 7.4x faster tokenize on
   66 MB (9.2 s vs 68.4 s, both contended) and byte-identical output. Not
   shipped now (no bytes moved; the monster's critical path is the MIX arms'
   second pass, not the tokenizer); held for M4 if the 45-min bar is
   threatened, then judged by the 20-row ledger like any lever.
2. **The gzip peel is the save's real lever, and it is v13's**: zlib 1.3.1
   reproduces the deflate stream at NO level (levels 1-9, default strategy,
   first difference at byte 0 -- a different deflate implementation wrote it),
   so a byte-exact re-deflate needs preflate-class reconstruction. The peeled
   form (296,540,843 B) under xz -9 is **5,525,752 B** -- 3.03x lighter than
   xz on the deflate stream (16,739,920) and 3.14x lighter than our form. Our
   MIX12 arm on the peeled form is measured beside this line (a reading, no
   bar): see "(a) peeled-form reading" below when it lands.

### (b) FILED (23:22, BEFORE the code) -- checksummed hash slots

Attribution: Mahoney's lpaq/paq8 HashTable (a check byte per bucket; on a
mismatch the bucket is reclaimed) -- here 2-way: the bucket at the hash and its
neighbour (base ^ 16) are both tried; on a double miss the one whose order-1
node has seen fewer bits is reclaimed (paq8's priority = the first state's
count). The check is 8 bits taken from the SAME multiplicative hash below the
index bits (an independent slice of the product); index 0 of each 16-state
bucket is free (the nibble tree uses nodes 1..15) and holds it -- no memory
grows. Tables covered: o3s/o6s (ctx20), o4s (ctx18), sp13s/sp24s (ctx_sparse),
ind1s/ind2s (ctx_ind1/2), lat1s/lat2s (the lattice, in byte_update). Mix11 is
FROZEN and untouched; the v11 arms keep writing byte-identical eggs. Mirror by
construction: the probe runs inside the context functions both sides call in
the same order; EGG_STATEHASH covers the check bytes because it hashes the
whole tables. Where no two contexts ever share a bucket the stream is
byte-identical (a fresh bucket's check is 0 and its reclaim is a no-op).

| row | M2b inner (arm) | predicted inner delta | why |
|---|---|---|---|
| wubbadub.html | 22,886 (CM12P) | -0.2% .. +0.3% (-46 .. +69) | 92 KB: few collisions, reclaims cost |
| cbs.log | 67,017 (CM12) | -0.5% .. -2.0% (-335 .. -1,340) | 16 MB of log through 2^18..2^20 buckets |
| ring01.wav | 130,722 (CM12) | -0.3% .. +0.3% | audio residuals: contexts are noise either way |
| notepad.exe | 172,416 (CM12) | -0.3% .. -1.0% (-517 .. -1,724) | |
| real-test.bmp | 259,269 (MIX12) | -0.2% .. +0.2% | 12 MB gradient: LZ tokens carry it |
| alarm01.wav | 259,697 (CM12) | -0.3% .. +0.3% | |
| vim-version9.txt | 272,256 (CM12H) | -0.3% .. -1.0% (-817 .. -2,723) | |
| kernel32.dll | 282,960 (CM12) | -0.3% .. -1.0% (-849 .. -2,830) | |
| segoeui.ttf | 410,874 (CM12) | -0.3% .. -1.0% (-1,233 .. -4,109) | |
| iconcache48.db | 414,160 (MIX12) | -0.5% .. -2.0% (-2,071 .. -8,283) | 97 MB through the same tables: the most oversubscribed row; the exhibit debt vs naked xz is 2,004 B -- CALLED: flips (coin flip leaning YES) |
| arial.ttf | 448,478 (CM12) | -0.3% .. -1.0% (-1,345 .. -4,485) | |
| zstd.exe | 492,042 (CM12) | -0.3% .. -1.0% (-1,476 .. -4,920) | |
| real-test.db | 1,087,216 (CM12H) | -0.3% .. -1.5% (-3,262 .. -16,308) | |
| wallpaper.jpg | 1,511,982 (v8 MIX, frozen) | 0 | unless a v12 arm takes the row |

Net over the 14 home rows: -12,000 .. -45,000 B. Gate: kept iff the summed
home total falls (net win) with no row heavier than M2b by more than 0.1%;
otherwise deleted. Big rows are judged in the M2c 20-row ledger through
MIX11's inner as the control (the frozen arm moves only with the tokens).

### (c) FILED (23:22, BEFORE the code) -- PE and TTF dialect books as trial arms

Attribution: v11-M5's site-book priors (the free-to-guess reading,
glossary.js:104); Mahoney's zpaq config lineage (models as selectable
methods). Two books trained by `eggv12 gen-prior` (cm11_run, the same export
shape: mixer weights, the h1/h2 followers, the o1 states, the o1 StateMap)
from files that are NOT in any corpus of this repo: the PE book from Windows
System32 DLLs (user32, gdi32full, advapi32, shell32, ole32, combase, ucrtbase,
msvcrt, comctl32, comdlg32, crypt32, wininet, urlmon, shlwapi, setupapi,
rpcrt4, sechost, ws2_32, oleaut32, imm32, uxtheme, dwmapi, dxgi, msvcp140,
vcruntime140, win32u, ntdll, kernelbase -- kernel32.dll, notepad.exe,
zstd.exe, msgraph.dll, ntoskrnl.exe, rustc_driver.dll EXCLUDED); the TTF book
from Windows Fonts (Candara, Calibri, Cambria, Consolas, Constantia, Corbel,
Georgia, Tahoma, Times, Trebuchet, Verdana, Courier New and their weights --
every arial*.ttf and segoe*.ttf EXCLUDED, family and all). Exclusion per test
row: the test row is never in the book because NO corpus file is; the arial
and segoe families are excluded whole so no sibling face leaks a glyph table.
New arms, sniffed by magic and run as trial entrants beside CM12 (strict
less-than keeps ties with the plain arm): MODEL 22 CM12-PE (files starting
"MZ"), MODEL 23 CM12-TTF (files starting 00 01 00 00 / "true" / "OTTO"). No
other row runs them; the LZ twin is not booked (CM12 holds every PE/TTF row).

| row | M2b inner | predicted delta | note |
|---|---|---|---|
| notepad.exe | 172,416 | -0.5% .. -1.5% (-862 .. -2,586) | the smallest PE: the book matters most at the start |
| kernel32.dll | 282,960 | -0.3% .. -1.0% | |
| zstd.exe | 492,042 | -0.2% .. -0.8% | a MinGW/MSVC build, not a Microsoft DLL: the book's dialect is thinner here |
| segoeui.ttf | 410,874 | -0.3% .. -1.0% | |
| arial.ttf | 448,478 | -0.3% .. -1.0% | |
| msgraph.dll / ntoskrnl.exe / rustc_driver.dll | 3,325,023 / 4,800,941 / 38,814,313 | -0.02% .. -0.2% | the model learns the dialect itself within the first MB |

Gate: kept iff the five home PE/TTF rows net lighter and NO row anywhere is
heavier (a trial arm that loses is simply not chosen: the ratchet holds by
construction; what the gate judges is whether the arm ever wins).

### (d) DROPPED -- the line/column context for logs

Reason printed: the lever's target vanished at M2a. cbs.log's form is 67,017
against naked xz -9's 139,004 (the CM12 arm took the row from the LZ arm
once the 12-bit APM tax was gone); the pre-shrunk line/column lever (1-4% on
logs, paq8's column contexts) was sized to flip a 2.4% deficit that no longer
exists. Not built; the reading stays in the README (v13 may want it for
logs that the CM arm does not already take).

### (a) peeled-form reading (23:23; a reading, no bar, nothing shipped)

The MIX12 arm alone (probe build, hb 24) on the inflated save (296,540,843 B):
tokenize 15.1 s, 879,881 matches covering 98.01%, first pass 5,306,038 B,
second pass unchanged, wall 86 s. **Our form on the peeled save: 5,306,038 B
vs xz -9 on the same peeled bytes 5,525,752 -- lighter by 219,714 (3.98%)**;
against our own form on the deflate stream (17,363,293) the peel is worth
12,057,255 B, 3.27x. The save's debt was never the model: it was the gzip
skin. v13's first line: a byte-exact deflate reconstruction (preflate class;
zlib 1.3.1 reproduces nothing here), then this number with armor 5,324 on it.

### (b) MEASURED (23:30) -- checksummed slots: KEPT; the range missed upward on 8 of 14, downward on 2

The 14-row home ledger (tools/ledger12.js, EGG_EXE = the M2c(b) snapshot, EGG_PRED = the
M2b totals so the printed miss IS the lever; 8 lanes, MB/s contended). Injuries 42/42 EXACT.

| row | M2b total | M2c(b) total | delta | M2b inner -> M2c(b) inner (arm) | inner % | predicted | verdict |
|---|---|---|---|---|---|---|---|
| wubbadub.html | 27,698 | **27,621** | -77 | 22,886 -> 22,809 (CM12P) | -0.34% | -0.2% .. +0.3% | MISS above (lighter) |
| cbs.log | 71,829 | **71,758** | -71 | 67,017 -> 66,946 (CM12) | -0.11% | -2.0% .. -0.5% | MISS below |
| ring01.wav | 135,534 | **135,565** | +31 | 130,722 -> 130,753 (CM12) | +0.02% | -0.3% .. +0.3% | HIT |
| notepad.exe | 177,228 | **176,430** | -798 | 172,416 -> 171,618 (CM12) | -0.46% | -1.0% .. -0.3% | HIT |
| real-test.bmp | 264,081 | **261,274** | -2807 | 259,269 -> 256,462 (MIX12) | -1.08% | -0.2% .. +0.2% | MISS above (lighter) |
| alarm01.wav | 264,509 | **264,505** | -4 | 259,697 -> 259,693 (CM12) | -0.00% | -0.3% .. +0.3% | HIT |
| vim-version9.txt | 277,068 | **273,982** | -3086 | 272,256 -> 269,170 (CM12H) | -1.13% | -1.0% .. -0.3% | MISS above (lighter) |
| kernel32.dll | 287,772 | **284,378** | -3394 | 282,960 -> 279,566 (CM12) | -1.20% | -1.0% .. -0.3% | MISS above (lighter) |
| segoeui.ttf | 415,686 | **409,857** | -5829 | 410,874 -> 405,045 (CM12) | -1.42% | -1.0% .. -0.3% | MISS above (lighter) |
| iconcache48.db | 418,972 | **418,323** | -649 | 414,160 -> 413,511 (MIX12) | -0.16% | -2.0% .. -0.5% | MISS below |
| arial.ttf | 453,290 | **446,628** | -6662 | 448,478 -> 441,816 (CM12) | -1.49% | -1.0% .. -0.3% | MISS above (lighter) |
| zstd.exe | 496,854 | **489,318** | -7536 | 492,042 -> 484,506 (CM12) | -1.53% | -1.0% .. -0.3% | MISS above (lighter) |
| real-test.db | 1,092,028 | **1,068,149** | -23879 | 1,087,216 -> 1,063,337 (CM12H) | -2.20% | -1.5% .. -0.3% | MISS above (lighter) |
| wallpaper.jpg | 1,516,794 | **1,513,903** | -2891 | 1,511,982 -> 1,509,091 (CM12) | -0.19% | 0 unless a v12 arm takes the row | HIT (the clause: CM12 took the row from v8's frozen MIX) |

Net over the 14 home rows: **-57,652 B** (predicted -12,000 .. -45,000: missed upward, -0.98% of the M2b home sum 5,899,343). Range verdicts: 4 HIT, 8 MISS above (lighter than the range), 2 MISS below. Rows heavier than M2b: 1 (ring01.wav +31 B = 0.02%, inside the 0.1% gate). GATE: net win, no row past 0.1% heavier -> **KEPT**.

The calls: iconcache48.db was CALLED to flip against naked xz (needed -2,004 on the total, coin flip leaning YES): it moved -649 (418,323 vs xz 416,968, still +1,355) -- **MISSED**. cbs.log, the row predicted most oversubscribed (16 MB of log), moved 0.11%: the log's contexts are few and repetitive, so the tables were never really shared; the reclaim buys nothing where nothing collides. The rows that moved most are the dense binaries and the db (arial -1.49%, zstd -1.53%, segoeui -1.42%, kernel32 -1.20%, db -2.20%) -- the lineage's 0.3-1.5% held its upper edge and then some; wallpaper.jpg's frozen v8 arm finally lost the row to CM12 (-2,891).

Mirror: EGG_STATEHASH's decoder hash is among the roster's encoder hashes on wubbadub.html (1 of 11 arm hashes) and notepad.exe (1 of 22); restores EXACT. cargo test 21/21 (a `claim_semantics` test rides: fresh claim is a no-op, the owner is found at either way, the less-experienced neighbour is reclaimed, check 0 is never live); clippy -D warnings clean.

### (c) MEASURED (23:36) -- the dialect books: KEPT (thin); every range missed BELOW

The 14-row home ledger (EGG_EXE = the M2c(c) snapshot, EGG_PRED = the M2c(b) totals so the printed miss IS the lever). Injuries 42/42 EXACT. The nine rows without a PE/TTF magic are byte-identical to M2c(b) (HIT).

| row | M2c(b) total | M2c(c) total | delta | inner (arm) | inner % | predicted | verdict |
|---|---|---|---|---|---|---|---|
| wubbadub.html | 27,621 | **27,621** | +0 | 22,809 (CM12P) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| cbs.log | 71,758 | **71,758** | +0 | 66,946 (CM12) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| ring01.wav | 135,565 | **135,565** | +0 | 130,753 (CM12) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| notepad.exe | 176,430 | **176,164** | -266 | 171,352 (CM12-PE) | -0.15% | -1.5% .. -0.5% | MISS below (the arm won, thinly) |
| real-test.bmp | 261,274 | **261,274** | +0 | 256,462 (MIX12) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| alarm01.wav | 264,505 | **264,505** | +0 | 259,693 (CM12) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| vim-version9.txt | 273,982 | **273,982** | +0 | 269,170 (CM12H) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| kernel32.dll | 284,378 | **283,604** | -774 | 278,792 (CM12-PE) | -0.28% | -1.0% .. -0.3% | MISS below (the arm won, thinly) |
| segoeui.ttf | 409,857 | **409,683** | -174 | 404,871 (CM12-TTF) | -0.04% | -1.0% .. -0.3% | MISS below (the arm won, thinly) |
| iconcache48.db | 418,323 | **418,323** | +0 | 413,511 (MIX12) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| arial.ttf | 446,628 | **446,354** | -274 | 441,542 (CM12-TTF) | -0.06% | -1.0% .. -0.3% | MISS below (the arm won, thinly) |
| zstd.exe | 489,318 | **488,915** | -403 | 484,103 (CM12-PE) | -0.08% | -0.8% .. -0.2% | MISS below (the arm won, thinly) |
| real-test.db | 1,068,149 | **1,068,149** | +0 | 1,063,337 (CM12H) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |
| wallpaper.jpg | 1,513,903 | **1,513,903** | +0 | 1,509,091 (CM12) | +0.00% | 0 | HIT (byte-identical: no magic, no book arm) |

Net: **-1,891 B** over the five booked rows (predicted -0.2% .. -1.5% per row; measured -0.04% .. -0.27%). Every book arm WON its row -- notepad.exe on the BCJ form (the book still sees "MZ" there), the two fonts on the plain form -- and every gain sits under the filed floor: the v11 finding stands ("the site's book speaks HTML, not PE" was half of it; a book of the dialect's siblings speaks PE, but the model learns a 300 KB file's dialect on its own within its first tens of KB, so the book's head start is worth 0.04-0.27%, not 0.3-1.5%). GATE: the five rows net lighter, no row anywhere heavier -> **KEPT** as trial arms (MODEL 22/23), printed as the thin win it is; the books are 348 KB + 364 KB of generated source, 356 KB in the exe. The M2c(b) mirror proof repeats: EGG_STATEHASH's decoder hash among the encoder's on notepad.exe (1 of 24 arm hashes) and arial.ttf (1 of 36); restores EXACT; cargo test 21/21; clippy -D warnings clean.

### M2c sum, home rows (the M2c 20-row gate ledger runs after M3 with the final exe; its big-row predictions are filed there)

| lever | home net | verdict |
|---|---|---|
| (a) sav reach | 0 (not shipped; -6,558 on the MIX12 arm alone at 0.04%) | MISS printed; two readings (hb-24 speed lever; the gzip peel is v13's: 5,306,038 on the peeled form) |
| (b) checksummed slots | -57,652 | KEPT |
| (c) dialect books | -1,891 | KEPT (thin) |
| (d) line/column | 0 | DROPPED (target vanished at M2a) |

## M3 FILED (23:38, BEFORE any M3 code) -- the exhibits

### (1) Stereo mid/side -- the two-silhouettes reading (spectrometer.html:602)

Both wavs are TRUE stereo, 22,050 Hz, 16-bit (measured before filing): ring01.wav corr(L,R) 0.9491, L == R on 17.1% of frames; alarm01.wav corr 0.8533, L == R on 13.7%. Per-frame RMS: ring L 1,052 / R 1,165 / mid 1,094 / side 371; first differences L 231 / R 256 / mid 241 / side 75. alarm: L 1,863 / R 2,024 / mid 1,871 / side 1,064; first differences L 483 / R 486 / mid 462 / side 290. The naive log2-of-rms arithmetic says mid/side saves ~1.7 bits of ~15.9 per frame on ring (10.7%) and ~0.8 of ~17.8 on alarm (4.5%) BEFORE the model; the CM12 arm's sparse contexts (b[-2], b[-4]) already read part of the cross-channel structure, so the realized gain is smaller.

Design: two length-preserving filters, FILTER_MS1 (id 13: side = L - R, mid = R + (side >> 1), the lifting form, exactly invertible in wrapping i16; then the per-channel order-1 delta of W16) and FILTER_MS2 (id 14: the same lifting, then W16O2's order-2 predictor); nominated by the WAVE sniff only when channels == 2 and bits == 16, pruned by the sample like every filter, decided by the full trial like every filter. Attribution: FLAC's mid/side stereo decorrelation (Coalson), Shorten (Robinson), the S-transform lifting. The NLMS cross-channel predictor (OptimFROG/Monkey's Audio lineage) is NOT built: filed as a reading (below), the adaptive filter belongs in the model, not in a fixed filter. `flac` is not on this machine's PATH: the FLAC -8 column prints "flac not installed" and the exhibit compares against v11's sealed totals and naked xz -9.

| row | M2c inner (arm, filter) | predicted inner delta | predicted total |
|---|---|---|---|
| ring01.wav | 130,753 (CM12) | -4% .. -9% (-5,230 .. -11,768) | 123,797 .. 130,335 |
| alarm01.wav | 259,693 (CM12) | -1.5% .. -4% (-3,895 .. -10,388) | 254,117 .. 260,610 |

Kept only on a net win over the two rows (a filter that loses is simply not chosen by the trial; the gate judges whether it ever wins). No other row can move (the sniff is WAVE-only). Length-preserving, so the write-time round-trip law is not triggered; a cargo test proves apply/undo identity on random stereo frames and on the two wavs' headers.

### (2) The JPEG peel -- a PROBE only (no bar, nothing shipped)

wallpaper.jpg (measured before filing): baseline SOF0, 8-bit, 3840 x 2400, three components all 1x1 (4:4:4), one scan Ss 0 / Se 63, DRI 480 (a restart marker every MCU row: 299 RSTn), APP14 Adobe, entropy-coded bytes 1,602,311 of 1,602,752. The probe (python, scratch): Huffman-decode every MCU (DC prediction reset at each RST), re-encode with the file's own DHT tables and the spec's 1-bit padding before RSTn/EOI, compare byte for byte. PREDICTION: byte-exact round trip on the test file (Adobe pads with 1-bits per the standard; baseline has no EOB-run or refinement ambiguities) -- called YES. The peeled coefficient stream (64 x i16 per block, 55,296,000 B) under xz -9: 1.25-1.45 MB (a 10-22% reading against the JPEG's 1.60 MB; packJPG/Lepton reach 15-25% with a coefficient model, which is not built). No egg row moves.

### (3) Readings, not built: the float-field filter (wubdiv.html:213/217/221) and the transpose (wubx.html:394) -- printed in the README with the reason.

## M3 MEASURED, FAILURE FIRST (23:43)

### The stereo member shipped unreadable, for eleven minutes

The first wav ledger under the mid/side build: ring01.wav 135,565 (unchanged --
the trial kept W16O2, id 8, over both mid/side forms), **alarm01.wav 252,862 B
with filter 14 (MS2) -- and the artifact could not be read: "info: no valid
header at any site", injuries dead/dead/dead.** Cause: armor.rs's header
verifier (`parse_header`) refused any filter id above 12 -- the constant was a
literal in the armor, not the filter table's own maximum -- so all three
sites failed validation and the container was, honestly, refused. Wrong data
never; but a transmute wrote what no restore could read, the v11 big-arena
class (the slot wall) again, and the write-time round-trip law did not cover
it because the filter is length-preserving and the file is under 64 MB. Fixes,
in this order: (1) `filter::FILTER_MAX` is ONE constant read by the header
verifier; (2) the write-time in-memory round-trip law now covers EVERY
filtered form (`fid != 0`), not only length-changing ones and >= 64 MB inputs
-- one decode per filtered transmute is the price, printed; (3) a cargo test
(`stereo_member_full_pipeline_round_trip`) drives a synthetic stereo WAVE
through filter -> trial -> armor -> parse_header -> dearmor -> undo under
both new ids and under the free trial. Found by the ledger, not by a test:
that is the miss to print. The wav ledger reruns below.

### (2) The JPEG peel probe -- round trip HIT, the size call MISSED

wallpaper.jpg: 432,000 blocks Huffman-decoded in 2.3 s; every RST padding is
all-ones and the tail pad is 1 bit of 1; the re-encode with the file's own
tables is **byte-exact on the scan (1,602,311 B) and on the whole file
(1,602,752 B)** -- the call was YES: HIT. The peeled coefficient stream (64 x
i16 per block, 55,296,000 B) under xz -9: **2,192,684 B = 1.368x the JPEG's
entropy-coded bytes -- HEAVIER.** The call (1.25-1.45 MB, a 10-22% gain) is a
MISS: a raw coefficient dump hands a generic coder 64 x 16 bits per block
where the JPEG spends ~30; the peel opens the door, but only a coefficient
model walks through it (packJPG/Lepton's per-band, neighbour-conditioned
contexts). Our MIX12 arm on the same peeled stream is printed below when it
lands. Nothing ships; the reading is v13's, with the round trip proved.

### (1) MEASURED (23:46) -- stereo mid/side: KEPT; alarm above its range, ring at zero

The two-row ledger under the fixed build (EGG_PRED = the M2c(c) totals so the miss IS the lever); injuries 6/6 EXACT; the in-memory round-trip law now runs on every filtered transmute.

| row | M2c total | M3 total | delta | inner (arm, filter) | inner % | predicted | verdict |
|---|---|---|---|---|---|---|---|
| ring01.wav | 135,565 | **135,565** | 0 | 130,753 (CM12, W16O2 id 8 kept) | 0.00% | -4% .. -9% | **MISS** (the trial kept the per-channel order-2 form: the model's sparse contexts already read the pair) |
| alarm01.wav | 264,505 | **252,862** | -11,643 | 248,050 (CM12, MS2 id 14) | -4.48% | -1.5% .. -4% | MISS above (lighter than the range) |

Net -11,643 over the two rows -> **KEPT**. The call had the two rows backwards: the side channel is 3x quieter on ring, yet ring gained nothing and alarm (side 1.7x quieter) gained 4.5%. Reading: the CM12 arm's (b[-2], b[-4]) sparse contexts read a 22 kHz stereo pair at frame distance already -- on ring the per-channel residuals are small enough that the model's cross-channel reading saturates; on alarm the side channel is loud (RMS 1,064) and the lifting removes what the byte-level contexts could not. FLAC -8: **flac not installed** on this machine; the exhibit compares against v11's sealed totals (ring 146,184 -> 135,565, -7.26%; alarm 273,196 -> 252,862, -7.44%) and naked xz -9 (ring 248,424, alarm 344,640: both far behind). The NLMS cross-channel predictor stays a reading.

## M2c/M4 GATE LEDGER FILED (23:47, BEFORE the 20-row run) -- the final exe (M2c(b)+(c) + M3 mid/side + the header fix)

The 14 home rows are predicted EXACTLY (the M2c(c) totals, alarm01.wav at its M3 total: the levers moved nothing else, byte-identical by the lever ledgers). The six big rows are POINT CALLS with ranges, derived from the home bands of lever (b) (dense binaries -1.0..-2.2%, text -0.11..-1.13%, LZ-carried rows -0.16%) and lever (c) (PE arms ~0 at scale); the flat M2b price rides on top. Misses print as misses.

| row | M2b inner | called inner % (range) | called inner | price | **called total** | reason |
|---|---|---|---|---|---|---|
| msgraph.dll | 3,325,023 | -1.5% (-2.0 .. -1.0) | 3,275,148 | 4,812 | **3,279,960** | PE (.NET assembly), CM12 since M2a: the (b) dense-binary band -1.0..-2.2 at home; (c) adds ~0 |
| mermaid-bundle.js | 4,117,782 | -0.6% (-1.2 .. -0.1) | 4,093,075 | 4,812 | **4,097,887** | JS text, CM12: the text rows moved -0.11 (cbs) .. -1.13 (vim) under (b) |
| ntoskrnl.exe | 4,800,941 | -1.5% (-2.0 .. -1.0) | 4,728,927 | 4,812 | **4,733,739** | PE, CM12; the PE book arm runs (13 MB): -0.02..-0.2 more |
| aoe4-autosave.sav | 17,363,293 | -0.1% (-0.3 .. +0.0) | 17,345,930 | 5,324 | **17,351,254** | deflate bytes under MIX12: the literals are near-random, the tables were never really shared (icon moved -0.16) |
| rdr2-shaders.vkcache | 41,872,644 | -0.3% (-0.8 .. -0.1) | 41,747,026 | 6,348 | **41,753,374** | shader ISA under MIX12: dense but LZ-carried; between icon and the binaries |
| rustc_driver.dll | 38,814,313 | -1.5% (-2.2 .. -0.8) | 38,232,098 | 6,348 | **38,238,446** | PE, CM12, 183 MB: the dense-binary band; the PE book arm runs and adds ~0 |

Home rows (exact): alarm01.wav 252,862; arial.ttf 446,354; cbs.log 71,758; iconcache48.db 418,323; kernel32.dll 283,604; notepad.exe 176,164; real-test.bmp 261,274; real-test.db 1,068,149; ring01.wav 135,565; segoeui.ttf 409,683; vim-version9.txt 273,982; wallpaper.jpg 1,513,903; wubbadub.html 27,621; zstd.exe 488,915.

Also filed for this run: injuries 60/60 EXACT; ratchet: every row <= its M2b total (the trial's frozen arms make a heavier row impossible unless a lever regressed; (b) touched the shared Mix12, so a heavier big row would be (b)'s regression and would be printed as such); the frozen MIX11/CM11 arms' inners on the big rows must equal their M2b inners (the tokens did not change: MIX11 is the control that separates (b)/(c) from the parse).
