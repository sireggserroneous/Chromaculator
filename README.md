# Chromaculator

An instrument for looking at integers. Lives at **chronochromatic.org**.

Write a number in hex, so its bits arrive padded to a whole nibble. Lay them most
significant first into the smallest square that holds them, and fold that square
along its main anti-diagonal into three regions — **Inner**, **Fold**, **Outer** —
which sum back to the number exactly. Blue is +1, red is −1, green is 0. Negating a
number flips every colour.

Cell *i* weighs 2<sup>−(i+1)</sup>, so the value is *k* / 2<sup>4·nibbles</sup> —
always a dyadic rational strictly inside (−1, 1).

## The pages

| | |
|---|---|
| **Spectrometer** | one integer in full: its stalk, its square, its three regions, its value as a light wave, and its point on the unit sphere |
| **Atlas** | every integer as a ring of dyadics, ordered by value. Ring *r* is the 2<sup>r+1</sup> roots of unity; selecting one draws a radius and lights the path down to it |
| **Wub** | integers as phasors summed tip to tail, each riding an ellipsoid, tracing one closed curve. Torus knots, crossing counts, and a curve you can export |
| **Wub ×** | multiplication as a rectangle; an ordered sequence of operands |
| **Wub ÷** | division kept exact: quotient, a multiplier on the boundary, and the remainder drawn |
| **Wubba Dub** | every operation on one page: each card is plain, pushed, or an operand — + − × ÷ |
| **Spec** | the convention, editable in the page |
| **Inspirations** | who found each piece of this first |

## Running it

```
python3 serve.py        # http://localhost:1338
```

Static files plus one `PUT` endpoint so `spec.md` can be edited from the Spec page.
Only that one file is writable. On a static host the site still works; the Spec page
becomes read-only.

## Checking a change

```
node tools/run.js wub.html      # does the page's JavaScript actually run?
node tools/product.test.js      # the product grid, against the arithmetic
node tools/wubx.test.js         # Wub x, including the parts run.js cannot reach
node tools/gizmo.test.js        # the corner gizmo points where it says
node tools/divide.test.js       # A = 2^e x Q x B + R, at every width
node tools/wubdiv.test.js       # Wub div, including the parts run.js cannot reach
node tools/load.test.js         # all three Wub pages under a full rack
node tools/running.test.js      # the 2^E-times-a-stalk pair all four operations share
node tools/wubbadub.test.js     # the paged cards, and that they reproduce the other three
node tools/bulk.test.js         # 10,000 integers through both grids (~20s)
node tools/chroma-order.test.js # the Chroma Certified Ordering certificate
```

`run.js` runs a page's JavaScript against a stand-in DOM. A 200 from the server says
nothing about whether the page's scripts executed; this does. See `tools/`.

## What is original here

Very little of the mathematics. Signed digits are Booth and Avizienis, the stalks are
Conway's, the bit reversal is van der Corput and Cooley–Tukey, the ring geometry is a
rotary encoder disc, the phasor sum is Fourier epicycles. See **Inspirations** for the
full accounting, with links.

What I have not been able to place is the fold itself: laying a digit string along the
anti-diagonals of a square and cutting it into three regions that sum back to the
number. Held loosely.

## The ordering

`data/chroma-base.tsv` is the Chroma UTF base table: 306 Latin letters and
digits ordered `base | case | accent`, generated from the real DUCET
(`data/allkeys.txt`) with case ranked above accent. It is the one source — the
certificate reads it rather than re-deriving it, because deriving it a second
time with `Intl.Collator` gave a different accent order. Locale collation is
not DUCET.

    tools/fetch-ucd.sh            # allkeys.txt + Unihan, into data/ (not committed)
    python3 tools/chroma_utf.py   # orders every assigned codepoint, certifies, writes data/
    node tools/chroma-order.test.js   # the 8-property certificate on the base table

`tools/chroma_utf.py` extends the ordering over the whole character map. Every
assigned codepoint gets a reading and sorts as if spelled in Chroma UTF; the
reading comes from Unicode's own data rather than a hand table.

| layer | source | characters |
|---|---|---|
| 1 | in Chroma UTF — Latin, digits | 306 |
| 2 | Hangul — the Unicode name *is* the romanisation | 11,172 |
| 3 | Han — Unihan `kMandarin`, then Cantonese, on'yomi, Korean, plus `kTotalStrokes` | 47,063 |
| 3\* | Han with no reading on file — falls back to its name | 51,619 |
| 4 | other script — the letter core of the Unicode name | 15,033 |
| 5 | symbol / emoji — the whole name (`TOP HAT`) | 23,660 |
| 6 | unnamed — private use, controls; sort last by codepoint | 143,678 |

Key, most significant first: `letters… | 0 | language | tone | strokes | codepoint`.
The letter run is 0-terminated, so a reading that is a prefix of another sorts
first. The tail levels are the tie breaks, each a sub-level of consecutive
codes — the same shape as `e < é < è < ê < ë`.

## The phonetic index — multi-listing

We read C as /k/ or /s/, so C belongs in the k path **and** the s path. Every
character is listed in every place its sound can put it, which makes the
phonetic ordering an **index** rather than a permutation: one entry per
(character, reading, language).

    python3 tools/chroma_phonetic.py

    entries            370,571
    multi-listed       27,732 characters sit in more than one place
    widest             阸×16, 啐×14, 噦×13, 噲×13, 僤×13

飼 sits in seven places — `si` (Mandarin), `zi` (Cantonese), `shi` and `ji`
(on'yomi), `kau` and `yashinau` (kun'yomi), `sa` (Korean). 行 sits in ten.

**Language is a filter, not a level.** Declare it and the impossible branches
are pruned; what remains is a *subsequence* of the full order, never a
reordering of it — that is property [4] of the certificate, checked by
comparing sort-then-filter against filter-then-sort. Region narrows a language
rather than siding with another region, so a request for `es-ES` accepts
entries tagged `es` but not `es-419`.

    c in Spanish (Spain)          /θ/  /k/
    c in Spanish (Latin America)  /s/  /k/
    c in Italian                  /tʃ/ /k/
    c in English                  /k/  /s/

Which is what rules out the impossible readings: *Cerveza* in Latin America has
`z` = /s/, so seseo `c` can no longer pair with distinción `z`. Two branches for
the word, not 108.

**The final tie break is the spelling, and the spelling order is the spine.**
Sound discards the accent — á, ä and ǎ all read `a` — so entries that tie on
sound fall back to Chroma UTF's own declared order rather than to the
codepoint. The `a` run reads `a á à ă â ǎ å ǻ ä ǟ ã ȧ ǡ ą ā ȁ ȃ` inside the
phonetic index too.

## Two lists

Same construction twice: position in the sorted list becomes the code, in the
smallest ring that holds the list. Ring *r* gives *r*+1 bit codes.

| | entries | ring | slots | code width | codes | spare |
|---|---|---|---|---|---|---|
| **short** — Chroma UTF base | 306 | 9 | 512 | 10 bits | 512..817 | 206 (40%) |
| **long** — phonetic index | 370,571 | 19 | 524,288 | 20 bits | 524,288..894,858 | 153,717 (29%) |

Ring 18 holds 262,144 — too small — so 19 is the smallest ring that takes the
long list.

The short list is the **spine**: 306 characters in spelling order,
`base | case | accent`. The long list is the **index**: one entry per
(character, reading, language), in sound order.

They are two *orders*, not one nested inside the other. The short list is not a
subsequence of the long one — 3 of its 305 steps run backwards, and all three
are multi-listing doing its job:

    Ư reads 'U' at 204,050   ->   v reads 'b' /b/  at   5,311   (betacism)
    Ŵ reads 'V' at 204,778   ->   x reads 'kh' /x/ at 128,094
    X reads 'KH' at 136,258  ->   y reads 'i' /i/  at 112,255

But the spine is the long list's **final tie break**, which is how the declared
accent run survives inside the sound order:
`a á à ă â ǎ å ǻ ä ǟ ã ȧ ǡ ą ā ȁ ȃ` then every other script's `a`.

## Sorting filenames

    python3 tools/chroma_sort.py shit 飼 this isit
    python3 tools/chroma_sort.py --lang ja-on shit 飼 this isit
    ls ~/Downloads | python3 tools/chroma_sort.py --lang en -

A word is not a bag of characters: `shit` is /ʃ/ /ɪ/ /t/, not /s/ /h/ /ɪ/ /t/.
So a string is segmented into graphemes first, longest match wins, and each
grapheme carries the same branches a character does.

    no language declared        Japanese (ja-on)
      isit                        isit
      shit                        飼    shi
      飼    si                     shit
      this                        this

飼 moves because /ʃi/ is a prefix of /ʃit/. Separators (`. _ - /` and spaces)
sort below every letter and spell nothing.

A string with branching graphemes occupies more than one position, exactly as a
character does — `this` has 9, 飼 has 7. To *sort* you need one position per
string, so the key is the **primary** branch of each grapheme, not the minimum:
taking the minimum read `isit` as `ishit`, because `s` has a /ʃ/ branch in
German and `sh` sorts before `si`. A real position for the string, a nonsense
canonical for it.

On a real directory, 5 of 16 positions differ from alphabetical, and every
mover is a `c` that sounds like `k` — `canvas.png` sorts under K.

### Context rules

A branch carries a positional condition, so a word prunes what a character
alone cannot:

    ^        word-initial          $        word-final
    >set     next grapheme starts with one of set     !>set   does not
    <set     previous grapheme ends with one of set
    @a|b|c   the whole word is one of these — the exception hook

    cat -> kat        city -> siti        gem -> jem       game -> game
    knight -> nit     night -> nit        enough -> enouf  ghost -> gost
    xylophone -> zilofone   box -> boks   psalm -> salm    who -> wo
    rose -> roze      canvas -> kanvas    cervezas -> servezas

The **character index still lists every branch** — a character on its own has
no context, so `c` belongs in the k path and the s path both. The word sorter
prunes by condition. Same table, two readers.

The `@` hook exists because some distinctions are not positional at all.
English θ vs ð is **lexical**: a closed list of function words and stems takes
/ð/ and no rule about position separates `this` from `thin`. Rules plus a short
exception list, which is how every real g2p is built. All four minimal pairs
separate:

    this -> dhis /ðis/        thin -> thin /θin/
    mother -> modher /moðeɹ/  month -> month /monθ/
    with -> widh /wið/        width -> width /widθ/
    breathe -> breadhe        breath -> breath

`tools/chroma_sort_test.py` carries the requirement a filename sort has that a
character index does not: **it must be a permutation.** Drop or duplicate a
file and the sort is worse than useless.

## Wub UTF

    python3 serve.py            # then http://localhost:1338/wubutf.html

The ordering, drawn. Every character is a Chroma UTF code in ring 9, so every
code is ten bits in the same four by four frame — ten bits pad to three whole
nibbles, twelve bits lay into a four by four square, and the four leftover
cells are empty in every frame. That is what makes a row read as a row.

A word is a strip of those squares, and **the first square where two names
differ holds the cell that decided the order.** The page marks it.

    dis  | this     decided at square 2, cell 9
    this | thin     decided at square 1, cell 4

So `dis` is *closer* to `this` than `thin` is, which is invisible in an
alphabetical list where d and t are far apart.

The page carries no copy of the tables. The ordering has one implementation, in
Python, and `serve.py` exposes it read-only:

    /api/base            the 306 base characters and their codes, and the ring
    /api/read?q=&lang=   every grapheme of q: its branches, reading and codes
    /api/sort?q=&lang=   q's lines ordered by sound

A change to the tables is a change everywhere. `tools/wubutf.test.js` tests the
seam — the page's own logic against real API output, then the whole stack over
a live socket, including a 404 on an unknown endpoint and a 413 on an oversized
query.

## A word is a polynomial

    node tools/chroma_poly.test.js

Exactly, not by analogy. A word's key is

    value = SUM code_i * x^(i+1),   x = 2^-12

    cerveza = 734x + 574x^2 + 722x^3 + 792x^4 + 574x^5 + 810x^6 + 522x^7

and lexicographic key order equals numeric value order for the same reason the
product rectangle is carry free: the largest coefficient is 810 and the field
is 4096, so no field ever carries into its neighbour.

The multi-listing is the rest of it. A word is a **product of per-grapheme
sums**, and expanding that product enumerates the readings:

    cerveza   branches 4 x 1 x 1 x 2 x 1 x 4 x 1 = 32 terms

Declaring a language is **specialisation** — it sends terms to zero, and the
survivors are always a subset:

    cerveza  32 terms  ->  en 1 (serveza)  ->  es-ES 1 (therbetha)

### Where the push analogy stops

Push and the reading set share a shape — one object, many representations, one
canonical choice — and picking the primary branch really is the analogue of
pushing to the fixpoint. But **push conserves value and phonetic variation does
not.** Cerveza's 32 readings hold 32 different values, so no reading is
reachable from another by pushing. And pushing a word's own bits conserves its
value exactly while introducing 47 negative cells; no Chroma UTF code is
negative, so a pushed word is a value, not a spelling.

## 100,000 words, three filters

    python3 tools/chroma_scale.py en.txt es.txt ja.txt

40,000 English, 40,000 Spanish, 20,000 Japanese, mixed. Each filter holds:
permutation kept, keys nondecreasing, and 100,000/100,000 distinct keys — no
two of a hundred thousand words collide.

**A declared language set is a choice per word, not one rule for the list.** The
detector is the branch set, as Pedro put it: look the word up in each declared
language and the hits name the language. First declared wins a tie.

    detection   claimed by one lexicon 98,142, by several 1,858, by none 0
                overlaps: en+es 1,858

    'en' -> 'en es'          99,889 of 100,000 positions changed, 17,701 readings
                             median displacement 522, max 83,642
    'en' -> 'en ja-on es'    99,895 of 100,000 positions changed, 31,136 readings
                             median displacement 1,392, max 96,224

    亜美      96,511 ->    287    yamei  -> abi
    本文      10,385 -> 38,720    benwen -> honbun

## Two axes, four combinations

Plain or pushed is the **representation** axis. As written or phonetic is the
**reading** axis. They are independent, so:

| | as written | phonetic |
|---|---|---|
| **plain** | `cervezas` | `servezas` |
| **pushed** | 32 shape variants — `c3rv3zas`, `c3rv32as`, … | every reading — `servezas`, `thervezas`, `chervezas`, … |

Pushed stops collapsing a name to one representation and lists every one it
has, each at its own position. On the spelling axis those are the **shape**
variants, so `cervezas` and `c3rv3zas` are one thing written two ways and both
produce the same 32-variant set. On the sound axis they are the readings.

### The shape axis is orthogonal to the sound axis

Not a rival. Declaring `leet` alongside `en` means English sounds over leet
shapes — `c3rv3zas` reads `servezas`, the same key as `cervezas`. Treating
`leet` as another candidate *language* made it compete with `en` for the single
primary slot, `en` won by being declared first, and the shape branches never
got a look in.

Shape is a **pre-pass**, because conditions test their neighbours: `c` before a
literal `3` took the `!>eiy` branch and `c3rv3zas` came out `kervezas` when the
`3` plainly stands for an `e`. Shape resolves first; sound reads what it leaves.

### Two push levels on the sound axis

    in context    32 readings   servezas, thervezas, chervezas, tservezas …
    rules off    120 readings   the same plus kervezas, kerbethas, …

`kervezas` is **not** a reading of `cervezas` in context — the positional rule
says `c` before `e` is never `/k/`, so it is pruned before the branch set is
built. Out of context it is, because a character on its own has no position to
be judged by, which is exactly how the character index already reads one.

## base(chroma-utf)

    python3 serve.py            # then http://localhost:1338/wubutf.html

A string is a number written with Chroma UTF digits. **One card per line,
commas between the items on a card**, the way Wub ± holds a rack:

    card 1 : (hello, 3, 45)

      hello  digits [616, 574, 658, 658, 680]
             616x + 574x² + 658x³ + 658x⁴ + 680x⁵      x = 1/4096
             = 0.616 574 658 658 680 in base 4096

### The value is the fraction, not the integer

`0.hello`, in (0,1) like every other stalk on the site. Leading with the
integer was wrong: `SUM code_i · B^(n-1-i)` grows with the digit count, so **it
sorts short words first whatever they say** — `he` beats `hell` on length alone.
Measured against the real key:

    0.word    he hell helllo hello helo   == the sort
    integer   he hell helo hello helllo   != the sort

The fraction also gets prefix ordering **for free**: a missing digit reads as
zero, zero is below every code, so `hell` is smaller than `hello` with no
terminator rule at all. The integer is kept because Wub ± takes integers, and
`tools/wubutf.test.js` keeps it as a live control — if it ever starts agreeing
with the key, the check has stopped meaning anything.

Each card pages through **Bodies** (one 4×4 square per digit), **Polynomial**,
**Facts** and **Rack**.

### Why 12 bits a digit and not 10

A code only needs ten bits, but **12 is three whole nibbles**. So a character
never straddles a nibble, and laid four cells wide every character is exactly
three rows. At ten bits nothing lines up — five characters is fifty bits, which
is twelve and a half nibbles.

### One integer construction, both axes

The digits change with the reading axis; the construction never does. That is
what keeps a spelling and its phonetic reading comparable pictures, and it is
the polynomial `tools/chroma_poly.test.js` already certifies — the integer and
the picture are the same object.

## Three alphabets

    python3 tools/chroma_ipa.py

| | entries | ring | digit | base | nibbles |
|---|---|---|---|---|---|
| **Chroma UTF** — the base table | 306 | 9 | 12 bits | 4,096 | 3 |
| **the phonetic index** — multi-listed | 370,571 | 19 | 20 bits | 1,048,576 | 5 |
| **Chroma IPA** — the chart | 123 | 7 | **8 bits** | **256** | **2** |

123 digits: 108 chart letters plus 15 prosody and tone marks. **Eight bits is
two whole nibbles**, which neither of the other two can reach — 306 needs nine
bits and 154 needs nine once the ring code's leading bit is carried, and both
round up to twelve. Chroma IPA uses bare ranks, so eight is exact.

### The order is derived, not declared

Not place|manner|voice. Every phoneme already carries a Chroma UTF spelling —
that is what the sound axis has always sorted on — so **the IPA order is those
spellings in the order the base table already declares.** Nothing new is judged,
and the adjacency falls out rather than being arranged:

    /s/ s and /ʃ/ sh   +1   immediate
    /d/ d and /ð/ dh   +2   retroflex /ɖ/ spells dd and sorts between them
    /t/ t and /θ/ th   +2   the dental click /ǀ/ spells tc

All ten stop/fricative pairs share a first digit; five are immediate. The ones
that are not have a sibling phoneme sitting between, which is a fact about the
chart rather than noise in the order.

Prosody marks are spelled with digits, so they sort ahead of every letter for
free — Chroma UTF already puts `0..9` before `a`. **Diacritics are not digits.**
They modify a segment the way an accent modifies a letter, so they are a
sub-level, the same shape as `base | case | accent`.

### What it buys

    word      chroma utf   digits  bits   ipa       digits  bits   saving
    shit      shit              4    48   ʃit            3    24    50%
    this      dhis              4    48   ðis            3    24    50%
    think     think             5    60   θink           4    32    46%
    cervezas  servezas          8    96   seɹvezas       8    64    33%

Two savings compound: one sound is one digit where `sh`, `th`, `dh` cost two,
and the digit is eight bits rather than twelve. `/api/ipa` serves the alphabet
and a query's reading in it.
