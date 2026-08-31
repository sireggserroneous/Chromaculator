/* glossary.js — the site's vocabulary, in one place.
 *
 * Every page uses these words and no page defines them. A reader meeting
 * "push the stalk" or "the greens are frozen" has no way in, and the prose
 * around it assumes they already had one.
 *
 * Each entry is {title, what, why}. `what` is the definition in a sentence a
 * newcomer can hold. `why` is the part that actually earns its place: what it
 * is for, or what to look at once you know. Neither may assume another term
 * the reader has not met -- if an entry needs one, it is a term with its own
 * entry and they can hover that too.
 *
 * Loaded after chroma-ui.js; registers itself and is otherwise inert.
 */
const GLOSSARY = {

  /* --- the convention itself --- */
  fold: {
    title: "the Fold",
    what: "The main anti-diagonal of the square — every cell where row + column = n − 1.",
    why: "It is the axis the square reflects across. Cells above it are Inner, below it Outer, and the Fold itself is fixed: it maps to itself.",
  },
  inner: {
    title: "Inner",
    what: "The cells above the anti-diagonal — the small place values, nearest the start of the number.",
    why: "Inner, Fold and Outer add back to the number exactly. Splitting it three ways loses nothing.",
  },
  outer: {
    title: "Outer",
    what: "The cells below the anti-diagonal — the large place values, at the far end of the number.",
    why: "It is Inner's mirror image. The map (r,c) → (n−1−c, n−1−r) swaps the two and leaves the Fold alone.",
  },
  stalk: {
    title: "stalk",
    what: "The number written out as one row of signed digits, smallest place value first, before it is folded into a square.",
    why: "The name is from Hackenbush, where a stalk of blue and red edges stands for a number. Everything on this site is that row, drawn differently.",
  },
  digit: {
    title: "signed digits",
    what: "Each cell is −1, 0 or +1 — red, green, blue — rather than the usual 0 or 1.",
    why: "Allowing −1 means a number has many spellings, which is what lets addition run with no carries at all.",
  },
  a1: {
    title: "A1, reserved",
    what: "The top-left cell. It is always green and never holds a digit.",
    why: "It is kept aside so the digits begin at B1 and the anti-diagonal walk starts clean. The dashed outline marks it.",
  },
  dyadic: {
    title: "dyadic rational",
    what: "A fraction whose denominator is a power of two — 3/8, 173/256, and so on.",
    why: "Every value on this site is one, and always strictly between −1 and +1. Cell i weighs 2^−(i+1), so nothing else can come out.",
  },
  hankel: {
    title: "Hankel order",
    what: "Filling the square one anti-diagonal at a time, each read from the bottom-left corner upward.",
    why: "It is the order a Hankel matrix is constant along, which is exactly the direction the fold cares about.",
  },
  antitranspose: {
    title: "anti-transpose",
    what: "The map (r,c) → (n−1−c, n−1−r): reflect the square across its anti-diagonal.",
    why: "It fixes the Fold, swaps Inner with Outer, and undoes itself — the three properties that define inversion in a circle.",
  },
  nibble: {
    title: "nibble",
    what: "Four bits. Numbers are written in hex here, so they arrive padded to a whole number of nibbles.",
    why: "The padding is why the leading zeros are real cells rather than nothing: they take up room in the square and change its size.",
  },

  /* --- the two forms --- */
  plain: {
    title: "plain",
    what: "The number's digits as written — just 0s and 1s, no negatives.",
    why: "The starting point. Compare it with pushed to see the same value spelled a different way.",
  },
  push: {
    title: "push",
    what: "Move colour toward the coarse end: a lit cell steps left into a green and leaves its own sign flipped behind.",
    why: "+1·2^−i is the same as +1·2^−(i−1) − 1·2^−i, so the value never changes — only the colours do. This is Booth's recoding.",
  },
  spread: {
    title: "spread",
    what: "The opposite move, rightward — one lit cell becomes an endless tail of the same sign.",
    why: "It only closes in the limit, so the finite reading falls short by exactly one place value. That shortfall is what the bar over the last digit means.",
  },

  /* --- the wub pages --- */
  wub: {
    title: "wub",
    what: "Several numbers drawn as phasors — little rotating arms — summed tip to tail, tracing one closed curve.",
    why: "The same idea as Fourier epicycles. Each number sets one arm's rate, and the sum is the shape they draw together.",
  },
  phasor: {
    title: "phasor",
    what: "A rotating arm of fixed length. Its length is the number's value, its rate is set by the number itself.",
    why: "One phasor draws a circle. Several, added end to end, draw everything else.",
  },
  torusknot: {
    title: "torus knot",
    what: "A closed curve that winds p times one way round a doughnut and q times the other.",
    why: "Each phasor alone rides a torus, so it has a (p,q) type. The summed trace does not lie on one and has no type.",
  },
  greens: {
    title: "greens",
    what: "The zero cells. They have no sign of their own, so the page is free to guess one.",
    why: "Frozen, they stay put. Turned up, each is re-rolled red or blue once per slot, and the shape breathes rather than sitting still.",
  },
  bias: {
    title: "bias",
    what: "Which way the greens lean when they are re-rolled — all red at one end, all blue at the other.",
    why: "50/50 is a fair coin. Push it to an extreme to see how much of the shape the undecided cells were holding up.",
  },
  shell: {
    title: "shell",
    what: "How solid the sphere the phasors ride is drawn, from bare wireframe to opaque.",
    why: "Turn it up when you cannot tell whether a point is in front of the equator or behind it.",
  },
  sweep: {
    title: "sweep",
    what: "How fast the whole figure turns, in cycles per second — and, with the sound on, the pitch.",
    why: "It was always a frequency; it is just far below hearing. The tone is that same number transposed up, so 0.12 Hz sits on A1 and doubling the sweep doubles the pitch. Frozen is silent, because a figure that is not turning has no tone.",
  },
  phase: {
    title: "phase",
    what: "Where you are within one full turn of the curve, from start to start.",
    why: "Dragging it takes the clock off automatic, so you can stop on one instant and look. Press play to hand it back.",
  },
  t_sound: {
    title: "sound",
    what: "Plays the curve. Each phasor becomes two sine voices, one per rate, at an amplitude set by its value.",
    why: "This is the same additive sum the drawing is — what you are watching traced is what you are hearing. The rates are whole numbers, so they land on a harmonic stack.",
  },
  pstyle: {
    title: "points",
    what: "How the moving points are drawn — as lights, birds or flames.",
    why: "Purely how it looks. Nothing about the number changes.",
  },

  /* --- the atlas --- */
  ring: {
    title: "ring",
    what: "One circle of the atlas. Ring r holds the 2^(r+1) roots of unity — every number of that width, once.",
    why: "Reading outward is reading one more bit of precision. The innermost ring is the coarsest number.",
  },
  radius: {
    title: "the radius",
    what: "The line drawn from the centre through the number you selected.",
    why: "What it crosses on the way is that number with its lowest bit dropped, again and again — its whole ancestry.",
  },
  allbits: {
    title: "all bits",
    what: "Off, a tile shows only the one digit its own ring owns. On, it opens out into the number's whole nibble string.",
    why: "Off is the ring's own bit, so the disc reads as one place value per ring. On, every red, blue and green of the number is visible in its tile.",
  },

  /* --- multiplication and division --- */
  rectangle: {
    title: "the rectangle",
    what: "Every cell of A meets every cell of B, and the pair weighs 2^−(i+j+2).",
    why: "That is the whole of multiplication — m cells by n cells is an m×n rectangle, and summing it is the exact product.",
  },
  remainder: {
    title: "remainder",
    what: "What division leaves over: A = 2^e × Q × B + R, exactly, at every width.",
    why: "Kept rather than rounded away, so the identity holds as written and nothing is approximate.",
  },
  operand: {
    title: "operand",
    what: "A card marked as one side of an operation rather than a number in its own right.",
    why: "The rack is read left to right: each row acts on everything above it.",
  },
};

if(typeof UI !== "undefined" && UI.tips) UI.tips.add(GLOSSARY);
