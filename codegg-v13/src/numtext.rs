//! numtext.rs -- THE FIELD SPLIT FOR NUMBERS SPELLED AS DIGITS (v13-M3c, WS-N).
//!
//! The reading is the dive's, `wubdiv.html`: `:217` the `2^e` normalisation
//! ("the shift that brings the quotient back inside, normalised so the leading
//! cell is never wasted"), `:221` ("Mantissas carry down the rack and exponents
//! add"), `:416` ("colour is the mantissa -- the shape. size is the ring"),
//! `:592-593` (the sign moved to a carrier that can hold it, "an exact
//! reflection"), and `:1184` / `:392-393` for the REASON: values incomparable
//! at their native magnitudes become comparable once scale is stripped into its
//! own stream.
//!
//! **This is CONTEXTS, not a peel.** The text goes back out byte for byte; what
//! changes is what the model is allowed to condition on. A generic byte model
//! sees `0.055887558` as eleven bytes with an order-4 history. This one also
//! sees: which FIELD the byte is in (sign / integer / fraction / exponent), how
//! far into that field it is, how long the integer part was, which number of
//! the row this is, and -- the one that matters -- **the digit at the SAME
//! POSITION of the previous number in the same array**.
//!
//! Nothing here reads a byte that has not already been coded, so the tracker
//! runs identically in both directions: `Field::update` is called with the byte
//! that just went through, encoder and decoder alike. That is the same law
//! `jcoef.rs` runs under, stated for a different alphabet.

/// how much of a file the sniff reads before it decides
const SNIFF: usize = 1 << 20;
/// digits per thousand bytes below which this is not numeric text
const SNIFF_PER_MILLE: usize = 350;

/// does this form spell numbers as digits densely enough to be worth the arm?
/// A NOMINATION, never a decision -- the roster still judges the arm on the
/// armored total, and an arm that loses costs nothing but one parallel pass.
pub fn looks_numeric(src: &[u8]) -> bool {
    let n = src.len().min(SNIFF);
    if n < 4096 {
        return false;
    }
    let mut digits = 0usize;
    let mut seps = 0usize;
    for &b in &src[..n] {
        if b.is_ascii_digit() {
            digits += 1;
        } else if b == b'.' || b == b',' || b == b'-' || b == b'e' || b == b'E' {
            seps += 1;
        }
    }
    digits * 1000 / n >= SNIFF_PER_MILLE && seps * 1000 / n >= 20
}

/// which field of a number a byte sits in
const OUT: u8 = 0;
const INT: u8 = 1;
const FRAC: u8 = 2;
const EXP: u8 = 3;

/// the longest run of digits one number is tracked across; past it the
/// alignment context degrades to "absent", which is a legal answer
const DMAX: usize = 24;

/// the field tracker. It holds nothing but what the decoder also holds.
pub struct Field {
    state: u8,
    /// digits seen so far inside the CURRENT field
    dpos: u8,
    /// digits seen so far inside the whole current number -- the alignment index
    dall: u8,
    neg: bool,
    intlen: u8,
    /// which number of the current row/array this is
    col: u16,
    cur: [u8; DMAX],
    prev: [u8; DMAX],
    prevn: u8,
    last: u8,
}

impl Default for Field {
    fn default() -> Field {
        Field::new()
    }
}

impl Field {
    pub fn new() -> Field {
        Field {
            state: OUT,
            dpos: 0,
            dall: 0,
            neg: false,
            intlen: 0,
            col: 0,
            cur: [0; DMAX],
            prev: [0xff; DMAX],
            prevn: 0,
            last: 0,
        }
    }

    /// the digit the PREVIOUS number carried at this alignment position, or
    /// 0xff for "the previous number was not that long"
    #[inline]
    fn aligned(&self) -> u8 {
        let i = self.dall as usize;
        if i < DMAX && i < self.prevn as usize {
            self.prev[i]
        } else {
            0xff
        }
    }

    /// key 0 -- the SHAPE: which field, how far in, the sign, how long the
    /// integer part was, which number of the row, and the byte just coded
    #[inline]
    pub fn key0(&self) -> u64 {
        (self.state as u64)
            | ((self.dpos.min(15) as u64) << 2)
            | ((self.neg as u64) << 6)
            | ((self.intlen.min(7) as u64) << 7)
            | ((self.col.min(255) as u64) << 10)
            | ((self.last as u64) << 18)
            | (0xA5u64 << 56)
    }

    /// key 1 -- the ALIGNMENT: the digit at the same position of the previous
    /// number, which is the whole point of the reading
    #[inline]
    pub fn key1(&self) -> u64 {
        (self.state as u64)
            | ((self.dall.min(23) as u64) << 2)
            | ((self.aligned() as u64) << 7)
            | ((self.last as u64) << 15)
            | (0xA6u64 << 56)
    }

    /// one byte has been coded; move the tracker. Both directions call this
    /// with the byte that actually went through.
    #[inline]
    pub fn update(&mut self, b: u8) {
        if b.is_ascii_digit() {
            if self.state == OUT {
                self.state = INT;
                self.dpos = 0;
                self.dall = 0;
                self.intlen = 0;
            }
            if (self.dall as usize) < DMAX {
                self.cur[self.dall as usize] = b - b'0';
            }
            self.dpos = self.dpos.saturating_add(1);
            self.dall = self.dall.saturating_add(1);
            if self.state == INT {
                self.intlen = self.intlen.saturating_add(1);
            }
        } else if b == b'.' && self.state == INT {
            self.state = FRAC;
            self.dpos = 0;
        } else if (b == b'e' || b == b'E') && (self.state == INT || self.state == FRAC) {
            self.state = EXP;
            self.dpos = 0;
        } else if (b == b'-' || b == b'+') && self.state == OUT {
            self.neg = b == b'-';
        } else if !(self.state == EXP && (b == b'-' || b == b'+') && self.dpos == 0) {
            // the number ended here
            if self.state != OUT {
                let n = (self.dall as usize).min(DMAX);
                self.prev[..n].copy_from_slice(&self.cur[..n]);
                for s in self.prev.iter_mut().skip(n) {
                    *s = 0xff;
                }
                self.prevn = n as u8;
                self.col = self.col.saturating_add(1);
            }
            self.state = OUT;
            self.dpos = 0;
            self.dall = 0;
            self.neg = false;
            self.intlen = 0;
            if b == b'\n' || b == b'[' || b == b'{' {
                self.col = 0;
            }
        }
        self.last = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the tracker never reads ahead: feeding it a prefix and then the rest
    /// leaves it in the same state as feeding it the whole thing
    #[test]
    fn the_tracker_is_causal() {
        let s = b"[-0.0063418,0.0558875,4.5e-3]\n[1,22,333]";
        for cut in 0..s.len() {
            let mut a = Field::new();
            let mut b = Field::new();
            for &c in &s[..cut] {
                a.update(c);
                b.update(c);
            }
            assert_eq!(a.key0(), b.key0());
            for &c in &s[cut..] {
                a.update(c);
            }
            for &c in &s[cut..] {
                b.update(c);
            }
            assert_eq!(a.key0(), b.key0(), "cut {}", cut);
            assert_eq!(a.key1(), b.key1(), "cut {}", cut);
        }
    }

    /// the alignment key is what carries the reading: two runs identical
    /// except for the PREVIOUS number must give different key1 at the same
    /// position, and identical key0, because key0 knows nothing about it
    #[test]
    fn the_alignment_finds_the_previous_number() {
        let mut a = Field::new();
        let mut b = Field::new();
        for &c in b"0.1234,0.5" {
            a.update(c);
        }
        for &c in b"0.9876,0.5" {
            b.update(c);
        }
        assert_eq!(a.key0(), b.key0(), "the shape is the same");
        assert_ne!(a.key1(), b.key1(), "the alignment is not");
    }

    #[test]
    fn the_sniff_refuses_prose_and_takes_digits() {
        let prose = vec![b'a'; 8192];
        assert!(!looks_numeric(&prose));
        let mut nums = Vec::new();
        while nums.len() < 8192 {
            nums.extend_from_slice(b"-0.006341881,");
        }
        assert!(looks_numeric(&nums));
    }
}
