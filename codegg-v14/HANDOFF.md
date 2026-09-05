# codegg-v14 HANDOFF — start here in a fresh context

**Plan:** `C:\Users\vcepe\.claude\plans\v14-let-the-numbers-talk.md` (supersedes
`so-give-me-a-iterative-quill.md`). **Filed predictions and every measurement:**
`codegg-v14/PREDICTIONS.md`. **v13's whole record:** `codegg-v13/PREDICTIONS.md`
— kept, not copied.

Read the plan in full first. It is self-contained. This file says only where the
work stopped and what to pick up.

---

## State: N0 done, N1 done, N2 half done

### N0 — the fork. DONE, gate taken.

`codegg-v14`, crate `eggv14`, format **EG14 v8**; reads `.egg13`/`.egg12` on the
same armor v4 path, `armor11.rs` still owns `.egg11` and older.

A **second session** was working in `codegg-v13` concurrently and landed a
`deflate.rs` guard (`fde5652`) and `sites.rs` tests (`19419b0`) AFTER v13's
ledger ran, so the sealed numbers were re-verified against the current source
before the fork could mean anything: **six rows including the mandatory
`aoe4-autosave.sav` all reproduce their sealed size and restore EXACT**, and the
29-file deflate suite reproduces its verdict. That session is now **read-only**;
this session owns `codegg-v13` and `codegg-v14`.

`cargo clippy` clean · `cargo test --release` **45** · `audit --full`
**3,091,771 checks, 0 failing**.

### N1 — the armor gate. DONE, and the conclusion inverted the inherited claim.

Built two audit sections. **(i) the mixed-damage drill** — `k` fully dead
squares alongside `m` partly hit ones, the combination nothing in the tree
generated: **3,174 wounds, 2,481 EXACT within `2m+k <= t`, 0 REFUSED, 0 wrong
data.** **(j) the false-accept hunt.**

The finding: `consistent()`'s `take(4)` reads **8,327 of 263,052** bad codewords
(3.2%) and **widening it to 100% changes not one outcome**. The subset-search
branch — the code the old memory priced at "~32 expected false accepts" — is
**unreachable, forbidden by a Vandermonde determinant** (`k+1` distinct locator
vectors cannot lie in a `k`-dim span while `k < m`, which is the branch's own
precondition). Measured: **0 over-reports in 2,355 locator runs**, including 24
trials at n = 62,518 with `m-k = 1` where the random-vector estimate predicted
~23. **The dead branch is deleted**; every audit tally is identical after
removing it.

Instruments left in place, printed under `EGG_CONSIST=1 eggv14 audit --full`:
call counts per caller, codewords read vs handed, and the locator's measured
`m`, `k`, `m-k` and `n`. **Keep them** — they are what makes the next armor
claim checkable.

### N2 — reclaim the speed headroom. **DONE (N2/N2b/N2c).**

**Done:** the 2D arm no longer runs where a dialect book already matched
(`src/main.rs`, `let rect = if wide && dialect.is_none() { ... }`). The argument,
not a tuning: a form whose magic names a dialect has a specialist arm primed for
it, and a FILTERED form never carries the magic (`dialect_of` reads offset 0), so
`alarm01.wav`'s win on the order-2 W16 residue was never at risk.

**Correctness: clean.** All twelve home rows byte-identical; all five 2D wins
kept — `alarm01.wav` and `vim-version9.txt` (home), `cbs.log`,
`mermaid-bundle.js`, `msgraph-docs.xml` — every one still model 28.

**Speed: the filed range is not clearly met.** `kernel32.dll` SOLO went
**0.278–0.280 → 0.281–0.287 MB/s**, the bottom edge of the filed 0.283–0.290.
The reason is structural: on an 836 KB file the 2D arm is one thread beside ~15
others and the machine has spare cores, so dropping a thread only helps where the
roster saturates.

**Also flagged rather than banked:** the fork measures **0.278–0.280** on
`kernel32.dll` where sealed v13 measured **0.268–0.273**, same source but for
N1's never-executed deletion. Most likely machine state. **Treat 0.278 as N2's
baseline and do not claim the difference.**

**MEASURED AFTER THE ABOVE WAS WRITTEN, and it kills the premise:** on
`ntoskrnl.exe` (13 MB, PE) the gate buys **nothing** — ungated 0.078/0.080 MB/s,
gated 0.080/0.080. So the `kernel32.dll` movement is noise, not a result.

**Why: the roster's wall clock is set by its SLOWEST arm, not by the number of
arms.** ~15 arms run in parallel threads on a machine with spare cores, so the
row ends when the longest one ends; removing a *loser* shortens nothing unless it
was also the *slowest*. "One fewer CM pass" means less CPU work, which is not
what the 0.25 MB/s floor measures. The gate **stays** (a dialect-booked form has
a specialist arm; the house does not do work that cannot win, and it frees CPU on
a loaded machine) but **no speed claim attaches to it.**

**What N2 still owes — REDIRECTED:**

0. **Instrument per-arm WALL CLOCK on the roster.** `EGG_ARMS=1` prints every
   arm's size and nothing prints every arm's time, so the slowest arm has never
   been identified. That is the only measurement that can move the floor.
   Everything below is secondary to it.

**And the original list, now lower priority:**

1. **DO NOT raise the rectangle hunt's bar.** The plan's other half is now
   pointless for speed: removing 2D nominations cannot move the clock, and the
   losers (`iconcache48.db`, `real-test.bmp`, both dialect-free) are not
   separable from the winners by the stride statistic anyway. It would risk a
   win to buy nothing.
2. A 20-row ledger group pass (`tools/ledger14.js`, IN GROUPS) to confirm no row
   moved, then N2's measured column in `PREDICTIONS.md`.

---

### N2c — the CM inner loop. DONE.

**Result: `kernel32.dll` 0.297 -> 0.311 MB/s, `vim-version9.txt` 0.498 ->
0.519 MB/s, every byte identical.** The floor row now clears 0.25 by 24%.
Three items paid (the lattice scan made branchless and const-length; the
number-field tracker not stepped where its keys are discarded; `pos % pixel`
rolled instead of divided). Three measured ZERO and were reverted -- the
compiler had already done them.

**Four things a fresh context must not re-learn the hard way:**

- **This machine drifts ~4% between measurement sessions**, which is the size
  of every effect here. Two flags were published as -3.3% and -4.7% losses
  before the baseline was re-read and both turned out to be ZERO. **Measure
  interleaved A/B in one shell, medians of 4-5 alternating runs.** The harness
  is `scratchpad/ab.sh`; the noise floor is +-0.2% on the round-trip.
- **The instrument is the round-trip, not `armtime`.** Per-arm times swing
  +-20% run to run because ~30 arm threads share 24 cores; N2b's published
  plateau ORDER was one sample and does not reproduce. The plateau's
  MEMBERSHIP (8 CM arms above 6 MIX arms) is 100% stable and is arithmetic.
- **`tools/m0gate.js` cannot pass and has not since v12-M1.** It asserts v14 ==
  v11 byte for byte (the v12-M0 fork condition). Its ancestor-compat half is
  live and does pass 14/14. Do not read its FAIL as a regression.
- **The round-trip law is 28% of the floor row** and fires on `fid != 0`, not
  only at 64 MB -- so it hits every filtered or peeled row. It is a serial
  single-threaded decode of the winning arm, which means every future cut to
  the shared CM loop is paid twice.

**Nothing owed on verification.** All twenty sealed rows have been re-run after
N2c and N3b. The twelve home rows plus `mermaid-bundle.js`, `msgraph.dll`,
`rdr2-shaders.vkcache`, `iconcache48.db` and `rustc_driver.dll` are all **SAME**
with injuries E/E/E; `aoe4-autosave.sav` moved by exactly the intended -39.6%
and restores EXACT read back from disk. `ROWS MOVED vs SEALED v13: 0` now means
all twenty rather than fifteen.

**Line for a future speed milestone, priced:** the lattice's remaining 4.3% and
the ISSE/APM stages all MOVE BYTES. On text rows `tokenize` + the cheap-v8
trial are 36% of the clock and have never been looked at.

### N3 the card · N3b the recipe · N3c the PE question. ALL DONE.

**N3b banked 3,464,635 B.** `aoe4-autosave.sav` 8,759,079 -> 5,294,444 (-39.6%),
90 s faster, every other row unmoved. The deflate recipe is PREDICTED now:
`src/zmatch.rs` infers the zlib matcher and 38,340,574 tokens ride as two
parameter bytes and **20 corrections**. Blob v2; v1 still reads.

Corpus-wide against the best pipeline anyone can actually run, a 1.33% LOSS
became a **2.02% WIN**. Against zpaq -m5 alone we are **14.85% ahead**.

#### Every milestone is now PRICED. Run `node tools/card.js`.

| next | worth | state |
|---|---|---|
| ~~N4 ZIP peel~~ | **SHIPPED** -- python312.zip -46.0%, ipf -48.3%, all restore EXACT | now the corpus's 21st sealed row |
| PE / binary model | **2,150,378 B** over 6 rows | a model-architecture project, not a fix |
| JPEG baseline peel | 265,778 B | vs paq8px on wallpaper.jpg |
| font / TTF model | 222,450 B | both TTF rows ~26% under paq8px |
| JS / text model | 188,813 B | |
| N6 audio | 168,871 B to the ceiling; **FLAC reaches only 58,050** | LPC+Rice is a third of it |
| N5 progressive JPEG | **UNPRICEABLE HERE** | needs packJPG installed; paq8px does not model progressive either |

### N4 the ZIP peel · N4b the guard and the parallel peel. DONE.
### **PICK UP AT N7, THE SEAL. It is the only thing left.**

`src/zip.rs`, `PEEL_ZIP = 3`. A file is alternating spans -- gap, member, gap,
..., gap -- with gaps carried VERBATIM and a member taken only if
`deflate::peel` reads it AND re-spells it exactly.

| row | result |
|---|---|
| **python312.zip** | 3,753,980 -> **1,963,843** (-47.7%), the corpus's **21st sealed row** |
| ipf-alienware.zip | 5,653,821 -> 2,925,145 (-48.3%), suite only |
| icu4j.jar | 5,826 STORED members, zero deflate -- **not nominated**, nothing attempted |

N4b then removed `deflate.rs`'s `values.len() < 4096` guard (331/599 -> **599/599**
members predicted, -63,663 B) and parallelised the per-member peel with a
work-stealing cursor (**33,679 -> 3,797 ms, 8.9x**, byte-identical). Removing the
guard exposed an N3b bug no corpus file could have reached: `from_blob` checked
the 284-spelling list against `nmatch`, which a PREDICTED recipe writes as 0.

#### WHERE v14 STANDS, for the seal to state

```
ours (21 rows)                    103,860,180
zpaq -m5 alone                    120,771,244   +16.28%  we WIN
precomp wherever it helps + zpaq  105,811,547    +1.88%  we WIN  <- the honest column
```

On ONE basis, 21 rows and precomp everywhere: **-3.03% -> +1.88%**, a 4.9-point
swing across N3b and N4. Do not quote the older +3.64% -- that column hands the
ZIP row the zpaq-alone figure a real competitor would never concede.

#### THE ONE OPEN BAR QUESTION THE SEAL MUST STATE

**`python312.zip` runs at 0.088 MB/s against the 0.25 home floor**, and ~0.20
even counting the inflated bytes the roster actually models. It fails on every
available basis. The scope taken, on a purpose test: the floor exists to stop a
new always-on ARM from making the codec unusable, and a peel is nominated per
format, so the bar does not govern it. A peel row is **not exempt but
accountable to a different bar -- the peel must pay for its time in bytes**:
this row paid 47.7%, `aoe4-autosave.sav` 39.6%. The seal should state this
plainly rather than let a 5x breach sit unremarked.

#### What is left after the seal, priced

| next | worth |
|---|---|
| PE / binary model | **2,150,378 B** over 6 rows -- a model-architecture project |
| JPEG baseline peel | 265,778 B vs paq8px |
| JS / text model | 243,131 B |
| font / TTF model | 222,450 B |
| N6 audio | 168,871 B to the ceiling; FLAC reaches only 58,050 of it |
| N5 progressive JPEG | **unpriceable here** -- needs packJPG installed |

#### Things a fresh context must not re-learn

- **The card is a TOOL.** `tools/card.js` over `tools/card.json`. Add a
  measurement to the json and re-run; never hand-edit a table.
- **zmatch's two measured holes:** `deflate_fast` (levels 1..=3) is wrong
  (gz-l1.gz predicts at 64.6% where the file IS level 1) and `windowBits` is
  unmodelled (smallwindow.gz 64.0%). They cost nothing today -- the format
  falls back per file -- but they cap N4.
- **The PE gap is the MODEL.** `FILTER_BCJ` is already taken on ntoskrnl.exe
  (worth -5.9% there) and notepad.exe. Do not go filter-hunting.
- **The 0.5% filter margin is a GOOD trade**, measured: it leaves 3,444 B on
  the table across corpus-real and buys not firing the round-trip law, which
  is 28% of a row. Closed; do not 'fix' it.
- **This machine drifts ~4% between measurement sessions.** Interleave A/B in
  one shell (`scratchpad/ab.sh`); the round-trip is the low-noise instrument.
- **`tools/m0gate.js` cannot pass** and has not since v12-M1: it asserts
  v14 == v11 byte for byte. Its ancestor-compat half is live and passes 14/14.
- The card binaries live in the scratchpad, never on PATH: precomp v0.4.7,
  paq8px v216, zpaqfranz v64.8, plus FLAC 1.5.0 and 7-Zip already installed.

## Then, in order: N3 · N4 · N5 · N6 · N7

All four are specified with filed predictions in the plan. Priority argument, in
one line each:

- **N3 finish the card** (measure, do not build). `precomp` on
  `aoe4-autosave.sav` first — our largest win, taken by *peeling*, which is
  precomp's own idea. Then Lepton (needs a build; **no gcc/cmake on this
  machine**), zpaq -m5, paq8px on two rows, PAR3.
- **N4 the ZIP container peel** — the largest new file class. `peel::members`
  already reads the layout, `deflate::peel` handles a member, THE CHAIN reaches
  depth 2. Only `peel::Peeled` needs to hold a **vector** of members.
- **N5 progressive JPEG** — the hole packJPG exposed: 51 of 60 byte-exact
  against our 43, the difference being exactly the 8 SOF2 files.
- **N6 audio** — the largest measured deficit. FLAC beats us **10.68%** on
  `alarm01.wav` and **22.63%** on `ring01.wav` with LPC + Rice, which we do not
  have. Build it as a **WAV-gated peel**, not an arm, so the speed floor is
  untouched.
- **N7 the seal.**

---

## Things that will bite a fresh context

- **The Bash tool resets cwd between calls.** Use absolute paths for corpus
  files or the transmute panics with `read input: NotFound`.
- **The exe lock is real.** Copy the build before running lanes
  (`EGG_EXE=<copy>`); `cargo` writing `eggv14.exe` while a lane runs it fails
  confusingly.
- **`tools/ledger14.js` opens 8 lanes on the 8 largest rows.** Run it in GROUPS
  (13 small in one pass, big rows in pairs, the save and the monster alone). The
  hang signature is **"no exit", not "no progress"** — kill it and read what it
  already wrote.
- **`packJPG` waits on `< press ENTER >`.** Close stdin and give every call a
  timeout, or one file takes the whole run down.
- **winget has FLAC (`Xiph.FLAC`) and 7-Zip (`7zip.7zip`) and nothing else on
  the card.** `CosmoX.Lepton` in that search is an UNRELATED product — not the
  JPEG codec. FLAC lands under the WinGet Packages dir, **not on PATH**.
- **Run every rival at its strongest documented setting and read `--help`
  first.** v13 published a FLAC "forfeit" that was a default, not a limitation:
  with `--keep-foreign-metadata` it round-trips byte-exactly and beats us on
  both wav rows.
- **A measurement that disagrees with the mechanism is not a finding until a
  control says the instrument works.** Three instances in two days: an `i32`
  overflow whose wrapped result got a published mechanism, the FLAC default, and
  my own drill aimed twice at a branch its own arithmetic said it could not
  reach.

## One loose end in codegg-v13, small and worth doing

`sites.rs`'s two subcommands (`gcdprobe`, `bitprobe`) are exercised by no tool in
the tree — the second session named it as still owed, and it is also v13's M3c
**deviation 3** (the overrulable "nothing ships dormant"). Wiring them into a
tool closes both at once.

## Uncommitted work

`codegg-v14/` is entirely untracked. `codegg-v13` has modified `PREDICTIONS.md`
and `README.md` (the specialist results, the recipe accounting, the test-count
correction). The second session offered `git reset --soft HEAD~1` on `7d04db2`
so this session's v13 work can be re-committed in topical, findings-first
splits — **Vladimir has not been asked yet; do not rewrite history without
asking.**
