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

### N2 — reclaim the speed headroom. **HALF DONE. PICK UP HERE.**

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
  (12 small in one pass, big rows in pairs, the save and the monster alone). The
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
