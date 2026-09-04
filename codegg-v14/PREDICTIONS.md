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

---

## N2b FILED (2026-09-04, BEFORE the instrument) — which arm sets the clock?

N2 measured that removing a losing arm does not move the wall clock, and named
the reason: **the roster finishes when its SLOWEST arm finishes, not when the sum
of its arms finishes.** That makes exactly one question worth asking, and this
project has never asked it — `EGG_ARMS=1` prints every arm's SIZE and nothing
anywhere prints an arm's TIME.

### The instrument

A per-arm elapsed-millis counter, printed under `EGG_ARMS=1` beside the sizes,
sorted slowest first, with the row's own wall clock beside it. An INSTRUMENT, in
the house sense: it prints what the roster already does and decides nothing.

### FILED, and the reasoning is from the code rather than a hunch

The roster is 15–17 arms. The LZ arms (`MIX*`) code TOKENS — fewer decisions
than one per byte — while the literal-only arms (`CM*`) push every byte through
the twelve-input mixer twice, once per nibble. That would make the CM arms
slower per byte. But `big_arms` then runs a **price replay**: `second_pass` on
the v11 LZ arm and `second_pass12` on the v12 one, which encodes those arms a
SECOND time.

1. **The slowest arm is an LZ arm carrying its price replay — `MIX12`, or
   `MIX12H` on rows ≥ 512 KB.** Called at **≥ 1.6× the median arm**.
2. **The row's wall clock lands within 1.0–1.3× of the slowest arm**, which is
   what "max, not sum" predicts and what would make N2's negative result
   inevitable rather than surprising.
3. **The spread between slowest and fastest arm is > 2×.**
4. `NUM` and `2D` cost about the same as `CM12`, since they ARE `CM12` with two
   sparse inputs re-pointed — **within ±15% of it**, and therefore not the arms
   that set the clock. If they are, N2's whole diagnosis is wrong and the gate
   should have worked.

**What would make this wrong:** if the times come back roughly flat, then no
single arm sets the clock, the wall clock is a genuine sum, and the parallelism
is not doing what the code assumes. That would be a bigger finding than the one
being chased, and it goes first.

## N2b MEASURED (2026-09-04) — the clock is set by a PLATEAU, not by an arm

The first per-arm timing ever taken in this project. It says my prediction was
wrong on the arm, wrong on the row, and it explains N2's negative result
completely.

### The filed predictions, judged

| filed | measured | verdict |
|---|---|---|
| the slowest arm is an **LZ arm** (`MIX12`/`MIX12H`) carrying its price replay | the **CM arms** are slowest by ~2×: `CM12P` 1,433 ms against `MIX12` 718 ms | **MISS** |
| the row's clock lands within **1.0–1.3×** of the slowest arm | the ROW is **2.07×** it (2,973 ms vs 1,433 ms); the big-roster STAGE alone is 1.34× | **MISS on the row, near on the stage** |
| the spread slowest-to-fastest is **> 2×** | 1,433 ms against `MIX10`'s ~450 ms — **> 3×** | **HIT** |
| `NUM`/`2D` cost about the same as `CM12`, **within ±15%** | on `vim-version9.txt`, **2D 1,699 ms against CM12 1,654 ms = +2.7%** | **HIT** |

**Where my reasoning went wrong, and it is instructive:** I wrote that the CM
arms push every byte through the twelve-input mixer twice while the LZ arms code
tokens, then talked myself out of my own first half by over-weighting the price
replay. The replay is real but it is **its own slot** (`MIX11-replay` 899 ms)
and the LZ arms are still half the cost of the CM arms. The first sentence of my
own reasoning was the answer.

### `kernel32.dll` (836,208 B), row 2,973 ms

```
armtime[plain] CM12P=1433 CM12=1411 CM12H=1357 CM12-book=1337 CM11H=1278
               CM11P=1228 CM11=1195 CM10=1099 | MIX11P=782 MIX12P=719
               MIX12H=711 MIX11H=705 MIX12=670 ... MIX10=453
stages         cheap-v8 trial 137 ms + tokenize 78 ms + BIG ROSTER 1917 ms
               (the remaining 841 ms is the peel arm, armor and the
                write-time round-trip law)
```

### `vim-version9.txt` (2,035,039 B), row 4,187 ms — the row the 2D arm WINS

```
armtime[plain] 2D=1699 CM12H=1670 CM12=1654 CM11=1526 CM11H=1516 CM10=1413
               | MIX11-replay=899 MIX12H=356 MIX12=350 MIX11H=323 MIX11=306
               MIX10=249
stages         cheap-v8 trial 783 ms + tokenize 763 ms + BIG ROSTER 2623 ms
```

### THE FINDING, and it closes N2

**There is no slowest arm. There is a PLATEAU.** Eight CM-class arms sit within
1,099–1,433 ms of each other on `kernel32.dll` and six within 1,413–1,699 ms on
`vim-version9.txt`. They run in parallel, so the stage ends when the last of
them ends — and **removing any ONE of them moves nothing, because the next one
is 2% behind it.**

That is exactly why N2's dialect gate bought nothing, and it is a stronger
statement than "the max, not the sum": *no single-arm change can ever move this
clock.* The gate was never going to work, and neither would gating any other
single arm.

**And the arm that sets the clock on `vim-version9.txt` is `2D` itself, at 1,699
ms — the arm that WINS that row.** So the one arm at the top of the list is the
one that cannot be dropped.

### What can actually move the floor, in order of measured size

1. **The shared CM inner loop.** Eight to nine arms are all `CM`-class and all
   within a few percent, so a change to the common path moves every one of them
   at once. It is the only lever that touches the plateau rather than a member
   of it.
2. **A whole CLASS, not a member.** `CM11`, `CM11H`, `CM10` cost 1,099–1,526 ms
   each and are FROZEN ancestor entrants, kept so "on every small file the
   elders always get their say" after `kernel32.dll` once breached the
   `<= min(ancestors)` law by one armor quantum. Dropping the class would move
   the plateau — and would break that law. **That is a trade to price, not a
   tuning, and it needs its own filed prediction.**
3. **`tokenize` and the cheap-v8 trial: 37% of the row on a text file**
   (763 + 783 of 4,187 ms), against 7% on `kernel32.dll`. Nobody has ever looked
   at either for speed, and on text they are the second-largest cost after the
   plateau.

### The instrument, and one defect it had first

`EGG_ARMS=1` now prints `armtime[...]` slowest-first and a `stages[...]` line.
Sizes are unchanged on every row checked — `kernel32.dll` 283,604,
`vim-version9.txt` 273,743, `alarm01.wav` 240,008, `wallpaper.jpg` 1,126,497 —
`cargo test` 45, `audit` 0 failing.

**The first cut used ONE bank of slots and raced.** `plain_transmute` runs
`big_arms` on the plain form and on the chosen filtered form **side by side in a
single thread scope**, so both rosters wrote the same 18 slots and both printed
whichever finished last — two identical lines, which is how it was caught. There
are three banks now, chosen by the caller's label. **The same lesson a third
time: the instrument gets a control before its output becomes a finding.**

---

## N2c FILED (2026-09-04, BEFORE the controls ran) — three questions N2b never asked

N2b concluded from one run that the clock is set by a PLATEAU of eight CM arms
and that **the shared CM inner loop is the only lever**. Before writing a line
of that, three things were checkable and unchecked.

1. **Is the plateau arithmetic, or is it the scheduler?** The machine is an
   Intel Core Ultra 9 285K — **24 cores, 8 P + 16 E, no SMT, 36 MB L3** — and no
   measurement in this project has ever accounted for the P/E split. FILED: the
   plateau's *membership and order* is stable run to run. If the membership
   shuffles, the plateau is the scheduler and N2c stops.
2. **How many arm threads are actually live?** `main.rs:775-788` runs `big_arms`
   on the plain form and on the chosen filtered form in the SAME thread scope.
   FILED: if a filter survives on `kernel32.dll`, the live count is ~30 on 24
   cores and N2's "removing an arm buys nothing" is true only at 18.
3. **What is the 841 ms nobody has named?** 2,973 − (137 + 78 + 1,917). FILED:
   the largest single component is **`Vec` teardown at process exit, >= 200 ms**
   (up to 30 models x ~71 MiB of tables). If under 50 ms, say so and move on.

## N2c MEASURED (2026-09-04) — two misses, and the tail was something else entirely

### The filed predictions, judged

| filed | measured | verdict |
|---|---|---|
| the plateau's membership **and order** is stable run to run | membership **100% stable** over 5 runs and 4 affinity masks — the 8 CM arms are ALWAYS above the 6 MIX arms, no crossing. **Order inside the class is noise: +-20%**, and the top arm changes every run (CM12H, CM12P, CM12H, CM12P, CM12H) | **HIT on membership, MISS on order** |
| the tail's largest component is **`Vec` teardown, >= 200 ms** | internal row 2,884 ms vs external wall 2,894 ms. **Process start + teardown is 10 ms.** The hypothesis is dead | **MISS** |
| a filter survives on `kernel32.dll`, so ~30 arm threads run | `armtime[filtered]` prints; **32 slots across two banks**, and the filtered form **WINS the row** (`filter 10:0`, 278,792 -> armored 283,604) | **HIT** |

### N2b's headline needs correcting, and so does one of my own numbers

**"The roster's wall clock is set by its SLOWEST arm, not by the number of
arms" is not what the data says.** Pinning the process to 8, 16 and 24 cores:

```
cores    row      BIG ROSTER   slowest arm   stage/max
   8    4,707 ms    3,601 ms     3,084 ms      1.17
  16    3,172 ms    2,133 ms     1,725 ms      1.24
  24    2,894 ms    1,838 ms     1,411 ms      1.30
```

The stage **scales with core count** — 8 -> 16 buys 1.69x, 16 -> 24 buys a
further 1.16x. So the roster is still CPU-limited at 24 cores with ~30 arms, and
arm CPU is not free. It is not "max" and it is not "sum": it is a queue with a
tail, and `stage/max` grows as cores are added because the queue drains.

**And a number I computed and had to throw away.** Summing the 32 `armtime`
values against the stage gave a beautifully stable **15.5x, five runs, +-0.1**,
which looked like a throughput law. It is an artifact: `timed()` measures
ELAPSED time, not CPU time, so an arm's number inflates when its thread is
descheduled. The 8-core run proves it — the same arithmetic gives **17.5x on 8
cores**, which is impossible. *Control before mechanism, a fourth time: the
check that killed it was running the same formula at a different core count.*

### THE TAIL, NAMED — and it is 29% of the floor row

```
tail[836208 B]: armor 2 ms + round-trip 822 ms (RAN) + write 0 ms
```

The 841 ms is **the in-memory round-trip law** (`main.rs:1011`), and it fires on
`kernel32.dll` **not because of size but because `fid != 0`** — the filtered
form wins the row, and v12-M3 extended the law to every filtered form. It is a
**serial, single-threaded full decode**: one thread, no roster, no parallelism
to hide behind. `armor()` is 2 ms and the write is 0.

`vim-version9.txt` is the control: the plain form wins there, `fid == 0`, the
law is **skipped**, and its tail is ~18 ms of 4,116.

### What this does to the lever

The round-trip decode runs the SAME `predict`/`learn`/`byte_update` as the arms.
So on the floor row the CM inner loop owns:

- the big roster, **1,823 ms** (throughput-limited across ~30 threads), plus
- the round-trip decode, **822 ms** (one thread, at full effect),

= **2,645 of a 2,875 ms row — 92%.** N2c's plan put the ceiling at "roster is
64% of the row"; with the tail named it is 92%, and a cut in the shared loop is
paid **twice**, once amortised and once whole. The lever is bigger than the
plan claimed, and it is the same lever.

**Not chased here, recorded for N3:** at ~30 threads on 24 cores the roster is
CPU-limited, so dropping arm CPU *does* move the clock — N2's null result holds
only where the roster is underloaded (`ntoskrnl.exe`, 11 slots). The arm-count
question is a scheduling question, not a byte question, and it is not this
milestone's.

## N2c PHASE 1 — the two ablations, judged

Throwaway builds, never shipped, each reverted after its reading.

| filed | measured | verdict |
|---|---|---|
| shrinking `o3s`/`o6s`/`ind2s` from `1<<20` to `1<<17` buckets is worth **>= 15%**; **under 5% and the memory hypothesis is dead** | **-5.9%** round-trip, -5.2% row | **MISS** — 48 MiB of tables per arm cut to 6 MiB, across ~30 concurrent arms, buys 5.9%. Just past its own kill line |
| the lattice scan is worth **4-10%**, not the top item | **-8.4%** round-trip with `lat_state = 2` (the detector removed outright) | **HIT** |

**Consequence, taken as filed:** the loop is **not** memory-bound in any way worth
chasing. Phase 2's item 2.7 (warming the seven bucket lines before `claim`)
was **dropped unbuilt** — its ceiling is a fraction of a 5.9% that costs 48 MiB
of context resolution to buy.

## N2c — A DEFECT IN THE MEASUREMENT PROTOCOL, found the only way it could be

`codegen-units = 1` measured **+3.3%** (a loss) and `target-cpu=x86-64-v3`
measured **+4.7%** (a worse loss). Two consecutive losses of the same size from
two unrelated flags is not a mechanism, so the baseline was re-read — and **the
baseline itself had moved from 2,821 ms to 2,933 ms.** Interleaved A/B in one
shell, base and candidate alternating, `target-cpu=v3` is **worth zero**.

**The machine drifts ~4% between measurement sessions, which is the same size as
every effect this milestone is chasing.** Every reading below is therefore
INTERLEAVED A/B, medians of 4-5 alternating runs. An accidental A/A run (a patch
that silently failed to apply) puts the noise floor at **+-0.2% on the
round-trip, +-1.2% on the roster stage** — so the round-trip, being one
single-threaded CM12 decode, is the instrument, and the per-arm `armtime`
numbers (+-20% run to run) are not.

## N2c PHASE 2 — the bit-exact wins, each measured alone

| # | change | round-trip | kept? |
|---|---|---|---|
| 2.1a | `codegen-units = 1` | **-1.2%** | no — inside the drift band, and it costs 60% more build time |
| 2.1b | `-C target-cpu=x86-64-v3` | **0.0%** | no — zero, and it would specialise the binary to this machine |
| **2.2** | the lattice scan: branchless, unit-stride, with a **const-length** steady-state path so the trip count is known at compile time | **-4.1%** | **YES** |
| **2.3** | `nf.update(b)` skipped where the lens is not `Num` — `NumField`'s state is observable only through `key0`/`key1`, which 8 of the 9 CM12 arms discard | **-2.3%** | **YES** |
| **2.4** | `pos % pixel` (a hardware `div` per byte on the 2D arm) carried as a rolling counter; the three per-byte `Lens` dispatches hoisted | *(with 2.3)* | **YES** |
| 2.5 | the 12-wide mixer accumulator split into 4 partial sums | **+0.1%** | no — LLVM was already breaking that chain. **Reverted** |
| 2.6 | `len_bucket(mlen)` computed once per bit instead of twice | **0.0%** | no — LLVM had already CSE'd it. **Reverted** |

**Two of the three items I expected to pay, paid nothing**, and both for the same
reason: the compiler had already done them. What paid was the work the compiler
could not remove — a branch it could not predict (2.2), a call whose result it
could not prove unused across a `&mut self` boundary (2.3), and a division it
could not strength-reduce (2.4).

### The result

```
kernel32.dll (the floor row)   2,818 -> 2,687 ms   0.297 -> 0.311 MB/s   -4.6%
  of which: BIG ROSTER         1,783 -> 1,696 ms                         -4.9%
            round-trip           814 ->   762 ms                         -6.4%
vim-version9.txt               4,090 -> 3,922 ms   0.498 -> 0.519 MB/s   -4.1%
  of which: BIG ROSTER         2,624 -> 2,450 ms                         -6.6%
```

The floor row clears 0.25 by **24%** where it cleared by 11%.

### Byte identity — the gate, and one gate that turned out not to be one

**14 of the 20 sealed rows re-measured, 0 moved, 0 failures, 42/42 injuries
EXACT**: the 12 `corpus-real` rows plus `cbs.log` and `ntoskrnl.exe`. Every lens
is covered — Plain (models 16/17/19/21/22/23), **Grid** (28: `alarm01.wav`,
`vim-version9.txt`, `cbs.log`), **Num** (27: `corpus/data.csv`, byte-identical
against the pre-change binary and restores EXACT), and the **peel** path (24:
`wallpaper.jpg`). **NOT re-run: `mermaid-bundle.js`, `msgraph.dll`,
`rdr2-shaders.vkcache`, `aoe4-autosave.sav`, `iconcache48.db`,
`rustc_driver.dll`** — six rows, hours each, and no lens they exercise is
untested above. That is a gap, and it is named rather than papered over.

**`tools/m0gate.js` is not the bit-exactness gate the N2c plan took it for.** It
asserts that eggv14's containers are IDENTICAL to eggv11's and match v11's
sealed sizes — the **v12-M0 fork** condition, which v12-M1 broke on purpose and
v13 broke by 17.9 MB. It reports FAIL on all 14 rows and has done since v12-M1;
running it on an unmodified v14 would fail exactly the same way. **Its live half
did pass, and it is the half a `mix11.rs` edit needs:** `.egg11`, `.egg10`,
`.egg9` and `.egg8` containers all restore **EXACT** through the modified binary,
14/14 — and the `.egg11`/`.egg10` restore paths run `mix11`/`mix10` decode, so
that is a real gate on the change, not a formality.

`cargo clippy` clean - `cargo test --release` **45** - `audit --full`
**3,091,771 checks, 0 failing**.

### What is left on the table, measured rather than guessed

- The **round-trip law is 28% of the floor row** and is a single serial decode.
  It cannot be removed (it is a safety law) and it cannot start earlier (it needs
  the winner), but every future cut to the shared CM loop is paid **twice** on
  any filtered or peeled row.
- The lattice's remaining **4.3%** (8.4% ablated - 4.1% taken) is its two mixer
  inputs and its lock work, which are **not** free: removing them moves bytes.
- On `vim-version9.txt`, `tokenize` + the cheap-v8 trial are still **36%** of the
  row (1,480 of 4,090 ms) and nobody has ever looked at either for speed.
