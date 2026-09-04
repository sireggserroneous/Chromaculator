# codegg-v14 PREDICTIONS — LET THE NUMBERS TALK

Plan: `C:\Users\vcepe\.claude\plans\v14-let-the-numbers-talk.md`, which
supersedes `so-give-me-a-iterative-quill.md`.

v13's own record — every measurement this campaign is judged against — is
`codegg-v13/PREDICTIONS.md`, kept whole and not copied here.

## Why this campaign exists, in one table

v13 sealed with **23/23 on the armored bar** and **net −17,963,568 vs sealed
v11**, then met its first two outside specialists on 2026-09-04:

| | v13 | rival | verdict |
|---|---|---|---|
| `corpus-jpeg`, the 40 rows both return byte-exactly | inner 24,432,195 | **packJPG 24,385,599** | **we lose by 0.19%, and 18 rows to 22** |
| `alarm01.wav` | 235,196 | **FLAC 210,087** | **we lose by 10.68%** |
| `ring01.wav` | 130,753 | **FLAC 101,165** | **we lose by 22.63%** |
| progressive (SOF2) JPEGs | **refused** | **packJPG reads them** | 51 of 60 to our 43 |
| the three injuries, every row | **3/3 EXACT** | **both forfeit 0/3** | ours alone |

**Ratio is not our claim any more. Armor is the only currency we hold outright,
and it has a live defect.** That is why N1 is the campaign's most important
milestone rather than its warm-up.

**We do not dismiss the losses. We build, we test, we put up a fight, and the
numbers do the talking** (Vladimir, 2026-09-04).

## House rules that earned themselves at v13

- Predictions FILED **before** the code that could move them. Misses printed
  FIRST. Losers **deleted**, never shelved.
- **A measurement that disagrees with the mechanism is not a finding until a
  control says the instrument works.** A no-op setting that must reproduce the
  baseline byte-for-byte goes in the sweep BEFORE the interesting settings. v13
  burned two numbers this way: an `i32` overflow whose wrapped result I
  published a mechanism for, and a rival run at defaults whose limitation I
  wrote up as a forfeit when the tool had a flag for it.
- **Run every rival at its STRONGEST documented setting, and read its `--help`
  before tabling a forfeit.** A rival we beat by mis-invoking it is worth less
  than no measurement at all.
- **Peels are cheap, arms are expensive.** A peel is magic-gated and costs
  nothing on files that do not match; an arm runs on everything and pays into
  the speed floor, which is now at **0.268 MB/s against 0.25**.
- **Report the RECIPE first** for any peel: raw→coded %, share of the shipped
  inner, a bar, and the RETURN measured against the `EGG_NO_PEEL=1` control.
- **Test the claims that KILLED something.** v13's second session found that the
  whole S1c verdict rested on `alpha` vs `a_alpha` three characters apart in one
  expression — slip it and a fresh 16-bit symbol costs 0 bits, every wide width
  manufactures a win, and the table still looks well-formed. Mutation-check the
  load-bearing line; do not assert it.

---

## N0 FILED (2026-09-04, at the fork, BEFORE the fork ledger ran)

`codegg-v13` → `codegg-v14`, crate `eggv14`, format **EG14 v8**. The armor did
not move, so `.egg13` and `.egg12` are read by the same armor v4 path (one
constant per accepted magic, both sides reading it) and `armor11.rs` still owns
`.egg11` and older.

**FILED: the fork reproduces all 20 sealed rows TO THE BYTE and restores every
ancestor artifact. Any row that moves is a fork bug, not a finding.**

---

## N1 FILED (2026-09-04, BEFORE ONE LINE of drill or armor code)

### The defect, as inherited

`armor.rs`'s `consistent()` (line 1241) reads **`bad.iter().take(4)`** — four of
up to 2,048 codewords — and is **provably vacuous at its caller** (line 1194):
the codewords it checks were just fixed by per-codeword Berlekamp–Massey, so
they pass by construction. The brute-force accept path then rests on **one
16-bit check, ~32 expected false accepts**. FNV-64 still saves conservation — we
never hand back wrong bytes — but **repairable wounds become refusals**.

### The gap that lets it hide, now read from the code rather than remembered

`audit.rs` generates two kinds of wound and never their combination:

- **`kill_squares`** (line 94) noises **whole squares** — fully dead, positions
  known, so they enter the erasure set `E`;
- **section (f)** (line 575) flips **single bits** in `e = ⌊t/2⌋ ` squares —
  partly hit, positions unknown, so they must be LOCATED by the syndromes.

**Nothing in the tree wounds both at once.** Mixed damage is exactly the case
`consistent()` exists to judge: some squares erased, others needing location,
and a partial location taken only if it makes the bad codewords consistent as a
set. The tournament's 4 KB scratch is ADDRESSED (pure erasure) and the drills'
blind 4 KB lands inside 3–5 squares of a synthetic container.

### THE DRILL COMES FIRST. A fix without a failing drill is a guess.

Audit section **(i)**: for every tier and placement, wounds combining **k fully
dead squares** with **m partly hit squares** (a few bytes each, so several
codewords see an error), blind and named, sweeping `k` and `m` across and past
the classical bound `2m + k ≤ t`.

**FILED:**

1. **The drill finds at least one REPAIRABLE wound that this build REFUSES**, at
   some tier, and it is a mixed case — `k ≥ 1` and `m ≥ 1`.
2. **Zero wounds return wrong bytes.** Conservation is not in question here,
   only reach. If any wound returns wrong bytes with a success code, that is a
   far larger finding than the one being chased and it goes first.
3. Widening `consistent()` from `take(4)` to every bad codeword costs **under 2%
   of restore wall clock** and **zero bytes** in any container.
4. **No sealed row's SIZE moves by a single byte** — this is a restore-path
   change and it cannot touch what the encoder writes.

**If the drill finds nothing, the defect is DOWNGRADED and printed as
downgraded, and `consistent()` is left alone.** That is a real possible outcome
and it is not a failure of the milestone — it is the milestone working. The
`take(4)` would then be documented as sufficient-in-practice with the drill as
its evidence, instead of carrying a memory that calls it vacuous.

---

## N0 MEASURED (2026-09-04) -- the baseline is sound, and the gate is taken

Before the fork's own ledger could mean anything, the sealed v13 numbers had to
be re-established against the source that actually exists: a **second session**
was working in `codegg-v13` concurrently and landed a `deflate.rs` guard at
06:54 (`fde5652`) plus `sites.rs` tests at 07:03 (`19419b0`), both AFTER v13's
ledger ran at 21:39-23:1x. It also explains the thirteen ancestor `.gitignore`
files stamped 20:18:48 that the M4 gate had to flag as unaccounted for.

**FILED: zero rows move. MEASURED: zero rows moved.** Six rows chosen so a miss
would be diagnosable -- `aoe4-autosave.sav` mandatory (the only sealed row whose
winning form IS the deflate peel), plus non-peel controls because
`looks_like_deflate` is entered on 9 of the 20 rows -- all six reproduce their
sealed size and restore EXACT, and the 29-file deflate suite reproduces its
verdict exactly.

`codegg-v14` forks from that verified state. Crate `eggv14`, format **EG14 v8**;
`.egg13` and `.egg12` read by the same armor v4 path, `armor11.rs` still owning
`.egg11` and older. `cargo clippy` clean, `cargo test --release` **45 passed**,
`EG14` magic confirmed on the wire, round trip EXACT.

---

## N1 MEASURED (2026-09-04) — the defect is REAL, its consequence was BACKWARDS, and the feared path is UNREACHABLE

Two drills, one control, and the conclusion is the opposite of the one inherited.

### The filed predictions, judged first

| filed | measured | verdict |
|---|---|---|
| the drill finds ≥1 **repairable wound this build refuses** | **0**, in 3,174 mixed wounds | **MISS** |
| **zero wounds return wrong bytes** | **0** wrong data, everywhere | **HIT** |
| widening `consistent()` costs <2% and 0 bytes | it changes **not one outcome** | see below |
| no sealed row's size moves | it is a restore-path change; nothing moved | **HIT** |

### (i) The mixed-damage drill — the case nothing in the tree generated

`kill_squares` noises **whole** squares (erasures, positions known); section (f)
flips **single bits** in ⌊t/2⌋ squares (errors, positions unknown). Nothing
wounded both ways at once, which is the only situation `consistent()` judges.

**3,174 mixed wounds: 2,481 EXACT within `2m+k ≤ t`, 0 REFUSED, 0 wrong data.**
The reach is intact.

### The control that made the result mean something

"No refusals" is not reassurance unless the drill reached the code. It did:

| | |
|---|---|
| `consistent()` reached | **2,311 times** |
| bad codewords handed to it | 263,052 |
| bad codewords it **read** (`take(4)`) | **8,327 — 3.2%** |
| rejections | **2** of 2,311 |
| **widened to all 263,052** | **identical** — same 2,311 calls, same 2 rejections, same tallies, same 3,091,770 checks |

**The `take(4)` reads 3.2% of the evidence and reaches the same verdict every
single time.** So the description in the inherited note is accurate and its
consequence — *"repairable wounds become refusals"* — is **backwards**. A check
that almost always says yes cannot cause refusals; it causes acceptances, and
the exposure would be miscorrection. Measured: none.

### The split by caller, which relocated the whole problem

| caller | reached |
|---|---|
| the partial-location path — the one the note calls "provably vacuous at caller 1188" | **2 times** in 3,091,770 checks |
| `confirm_subset` — the one priced at "~32 expected false accepts" | **2,333 times, accepted 2,333** |

The caller singled out is barely exercised. The real traffic accepts 100% of
what it is offered — and every one of those took the `found.len() == k` early
return, so **the subset-enumeration loop was never entered.**

### THE FINDING: that loop cannot be entered, and the reason is a determinant

The locator tests each position's vector `(1, X, X², …, X^(m−1))` for membership
in a `k`-dimensional span. **Those are Vandermonde rows, and any `m` of them
with distinct nodes are linearly independent.** So `k+1` distinct positions
cannot all lie in a `k`-dimensional space while `k < m` — and `k < m` is exactly
the condition under which the locator runs. **`found.len() > k` is impossible.**

The caller's own comment estimates the false-positive rate at `~n/65536^(m−k)`,
which treats the candidate as a **random** vector. It is a structured one. The
true rate is **zero**.

Measured beside the proof: **2,355 locator runs, 0 over-reports, max excess 0**,
mean `m` 16.2, mean rank 7.4, smallest `m−k` seen **1** — including 24 trials
built for the best case the armor allows, **n = 62,518 squares** (NMAX 65,535)
with `m−k = 1`, where the random-vector estimate predicts 0.95 false positives
per run and would have produced about 23. **It produced none.**

### What shipped

**The subset search is DELETED** (house rule: nothing ships dormant, and dead
code is worse than dormant). Kept: the residual check on `found.len() == k`,
which is real. The impossible case now **refuses** rather than enumerating,
because a locator that over-reports has broken an invariant and guessing a
subset would be the worst available answer. `consistent()` keeps its `take(4)`,
now documented with the drill as its evidence rather than a note calling it
vacuous.

After the deletion: `cargo clippy` clean, `cargo test --release` **45 passed**,
`audit --full` **3,091,771 checks, 0 failing**, and **every tally identical** to
before it — which is what "dead" means.

### Two of my own aims were wrong before the arithmetic was right, and they are the lesson

1. **First construction: 2 wounded squares.** The locator ran **zero** times —
   with `t = 18`, per-codeword Berlekamp–Massey fixes 2 errors and returns long
   before the collaborative path. The locator only runs when per-codeword
   decoding fails.
2. **Second construction: a rank-1 twin pair** (the trick section (h) uses).
   **Counterproductive.** `m = t − |E| = 18`, `k = rank = ` the number of
   distinct damaged positions, and lowering the rank *raises* `m−k`, which
   divides the false-positive rate by another 65,536. Ten wounds give `m−k = 8`
   and `n/65536⁸`.

Only the third aim — `t−1 = 17` **independent** wounds, giving `m−k = 1` — even
reached the interesting regime. **The arithmetic in the caller's comment was the
map the whole time, and I ignored it for two rounds.** Filed as a miss against
myself: the drill's aim is part of the drill.

### The defect, restated for the record

**DOWNGRADED, and more than downgraded.** `take(4)` is a 3.2% sample that has
never once disagreed with reading everything. The "~32 expected false accepts"
priced a code path that a Vandermonde determinant forbids. What was real is that
**the tree had no mixed-damage drill at all** — and now it has one, at 3,174
wounds a run.

---

## N2 MEASURED (2026-09-04) — the gate is correct and buys NO SPEED. The mechanism I assumed was wrong.

### The filed predictions, judged

| filed | measured | verdict |
|---|---|---|
| worst home row returns to **0.283–0.290 MB/s** | `kernel32.dll` 0.278–0.280 → **0.281–0.287** | **touched, not met** |
| the five 2D wins are kept **to the byte** | all five kept, all still model 28 | **HIT** |
| no row's size moves | **all twelve home rows byte-identical** | **HIT** |

### The measurement that kills the premise

The gate skips the 2D arm where a dialect book already matched. Five home rows
stop paying for a CM pass they always lose (`kernel32.dll` 2D=297,069 against
CM12-PE's 284,319; `segoeui.ttf` 2D=408,943 against CM12-TTF's 404,871). On a
**bigger** PE row, where that wasted pass is real work rather than a rounding
error, it should have shown clearly:

```
ntoskrnl.exe (13 MB, PE)   ungated  168,176 ms / 162,096 ms   (0.078 / 0.080 MB/s)
                             gated   162,208 ms / 162,413 ms   (0.080 / 0.080 MB/s)
```

**No gain.** And that makes the `kernel32.dll` movement (0.278 → 0.287, ~3%)
indistinguishable from run-to-run noise rather than a result.

### Why, and it redirects the milestone

**The roster's wall clock is set by its SLOWEST arm, not by the number of arms.**
Fifteen-odd arms run in parallel `std::thread` workers on a machine with cores to
spare, so the row finishes when the longest one finishes. Removing a *loser*
shortens nothing unless the loser was also the *slowest*.

I assumed "one fewer full CM pass" meant "less wall clock". It means less CPU
work, which is not the same thing and is not what the 0.25 MB/s floor measures.
**Filed as a miss against the mechanism, not just the number.**

### What is kept, and on what grounds

The gate **stays** — but the justification is no longer speed. It is that a form
whose magic names a dialect has a specialist arm primed for it, so the rectangle
is the wrong reading by construction, and the house does not do work that cannot
win. It also frees real CPU on a loaded machine, which the SOLO number cannot
see. **No speed claim attaches to it.**

### What N2 actually owes now

**Find the slowest arm and shorten IT.** That is the only thing that can move
the floor, and it has never been measured: `EGG_ARMS=1` prints every arm's
*size* and nothing prints every arm's *time*. The next step is an instrument —
per-arm wall clock on the roster — and then a decision about the arm at the top
of that list.

Until that exists, **the floor stands at 0.278–0.280 MB/s against 0.25 and any
new always-on arm is still blocked**, which was N2's whole reason for existing.
