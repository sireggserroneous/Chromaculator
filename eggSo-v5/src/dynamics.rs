//! dynamics.rs -- guess and fix, and where it lands.
//!
//! (The module is `dynamics` and not `dyn`, because `dyn` is a keyword.)
//!
//! Arthur Cayley, 1879, asked where Newton's method lands from a given
//! guess. For a quadratic the answer is a straight line down the middle:
//! guess on one side and you get one root, the other side the other. For a
//! cubic he could state the problem and not solve it, published it as a
//! failure, and it stayed open until Julia and Fatou, and then until
//! computers made it visible a century later.
//!
//! Newton's method IS guess-and-fix, and the site's own divider already runs
//! it in miniature: `stalk.js:288-306` guesses a signed digit from
//! `{-1, 0, +1}` at every step and carries the corrected remainder forward.
//!
//! The two facts this module measures:
//!
//!   * Newton on `z^2 - 1` is conjugate to `z -> z^2` by the Moebius map
//!     `w = (z-1)/(z+1)`. Under that coordinate its two basins are the
//!     inside and the outside of the unit circle, and the circle between
//!     them is the Julia set. That is Inner / Fold / Outer -- see `fold.rs`
//!     for the grid half of the identification.
//!   * Newton on `z^3 - 1` has no straight separator at all. Three regions
//!     meet along its boundary everywhere, and two regions can only meet
//!     along a line. That is Cayley's wall, and it is why a grid split by a
//!     straight anti-diagonal is a degree-2 object.
//!
//! Every routine here reproduces the arithmetic ORDER of the JavaScript that
//! produced this round's filed ground numbers, so `f64` gives the same
//! answers rather than merely close ones.
//!
//! CARRIED FROM eggSo-v4 unchanged except for the last section, which is
//! v5's: the cell -> complex coordinate. v4 placed the fold by MODULUS alone,
//! because `rho` is a modulus and `z -> z^2` needs nothing more. Degree three
//! needs the angle too -- and the angle is not free, because the site already
//! fixed it, in `cellOrder`. See `z_of`.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct C {
    pub re: f64,
    pub im: f64,
}

impl C {
    #[inline]
    pub fn new(re: f64, im: f64) -> C {
        C { re, im }
    }
    #[inline]
    pub fn norm2(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    #[inline]
    pub fn abs(&self) -> f64 {
        self.norm2().sqrt()
    }
    #[inline]
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }
    #[inline]
    pub fn finite(&self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
    #[inline]
    pub fn dist2(&self, o: C) -> f64 {
        let dx = self.re - o.re;
        let dy = self.im - o.im;
        dx * dx + dy * dy
    }
}

/// The blown-up value the JS used when a denominator vanished. Kept so the
/// two implementations agree on the degenerate cells too.
const BLOWN: C = C { re: 1e9, im: 1e9 };

/// `z -> z^2`. The whole of this round's dynamics, in one line: its Julia
/// set is the unit circle and its two Fatou basins are 0 and infinity.
#[inline]
pub fn zsq(z: C) -> C {
    C::new(z.re * z.re - z.im * z.im, 2.0 * z.re * z.im)
}

/// Newton on `z^2 - 1`, i.e. `(z^2 + 1) / (2z)`, computed as the JS did:
/// numerator times the conjugate, over `2|z|^2`.
#[inline]
pub fn newton2(z: C) -> C {
    let (x, y) = (z.re, z.im);
    let d = 2.0 * (x * x + y * y);
    if d == 0.0 {
        return BLOWN;
    }
    let nx = x * x - y * y + 1.0;
    let ny = 2.0 * x * y;
    C::new((nx * x + ny * y) / d, (ny * x - nx * y) / d)
}

/// Newton on `z^3 - 1`, i.e. `(2z^3 + 1) / (3z^2)`. Cayley's failure.
#[inline]
pub fn newton3(z: C) -> C {
    let (x, y) = (z.re, z.im);
    let x2 = x * x - y * y;
    let y2 = 2.0 * x * y;
    let x3 = x2 * x - y2 * y;
    let y3 = x2 * y + y2 * x;
    let nx = 2.0 * x3 + 1.0;
    let ny = 2.0 * y3;
    let dx = 3.0 * x2;
    let dy = 3.0 * y2;
    let d = dx * dx + dy * dy;
    if d == 0.0 {
        return BLOWN;
    }
    C::new((nx * dx + ny * dy) / d, (ny * dx - nx * dy) / d)
}

/// `w = (z - 1) / (z + 1)`, the conjugacy that turns Newton's straight
/// boundary into the unit circle.
///
/// `None` at `z = -1`, which is the map's pole and has no image in the
/// plane. That single cell is why the grid sweep below reports 159,597
/// points and not 159,598: the pole is not a counterexample, it is not a
/// point of the domain.
#[inline]
pub fn mobius(z: C) -> Option<C> {
    let (x, y) = (z.re, z.im);
    let (dx, dy) = (x - 1.0, y);
    let (ex, ey) = (x + 1.0, y);
    let den = ex * ex + ey * ey;
    if den == 0.0 {
        return None;
    }
    Some(C::new((dx * ex + dy * ey) / den, (dy * ex - dx * ey) / den))
}

pub fn roots2() -> Vec<C> {
    vec![C::new(1.0, 0.0), C::new(-1.0, 0.0)]
}

pub fn roots3() -> Vec<C> {
    let s = (3.0f64).sqrt() / 2.0;
    vec![C::new(1.0, 0.0), C::new(-0.5, s), C::new(-0.5, -s)]
}

/// Iterate and report which root the guess fell into, or `None` if it never
/// settled. The tolerance and the iteration cap are the JS ones.
pub fn basin(mut z: C, f: impl Fn(C) -> C, roots: &[C], iters: usize) -> Option<usize> {
    for _ in 0..iters {
        z = f(z);
        if !z.finite() {
            return None;
        }
        for (k, r) in roots.iter().enumerate() {
            if z.dist2(*r) < 1e-12 {
                return Some(k);
            }
        }
    }
    None
}

/// The fate of a point under `z -> z^2`: inside falls to zero, outside runs
/// away, and the unit circle does neither.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fate {
    Zero,
    Infinity,
    Neither,
}

pub fn fate_zsq(mut z: C, iters: usize) -> Fate {
    for _ in 0..iters {
        z = zsq(z);
        let m = z.abs();
        if m < 1e-12 {
            return Fate::Zero;
        }
        if m > 1e12 || !m.is_finite() {
            return Fate::Infinity;
        }
    }
    Fate::Neither
}

/// A grid of basin labels over `[-span, span]^2`, laid out row by row in the
/// same order the JS walked it.
pub struct Grid {
    pub g: usize,
    pub span: f64,
    pub cell: Vec<i8>,
}

impl Grid {
    pub fn at(&self, i: usize, j: usize) -> i8 {
        self.cell[i * self.g + j]
    }
    pub fn coord(&self, i: usize) -> f64 {
        -self.span + 2.0 * self.span * i as f64 / (self.g - 1) as f64
    }
}

pub fn basin_grid(g: usize, span: f64, f: impl Fn(C) -> C + Copy, roots: &[C]) -> Grid {
    let mut cell = vec![-1i8; g * g];
    for i in 0..g {
        let x = -span + 2.0 * span * i as f64 / (g - 1) as f64;
        for j in 0..g {
            let y = -span + 2.0 * span * j as f64 / (g - 1) as f64;
            cell[i * g + j] = match basin(C::new(x, y), f, roots, 200) {
                Some(k) => k as i8,
                None => -1,
            };
        }
    }
    Grid { g, span, cell }
}

/// How tangled a basin boundary is. A cell is on the boundary when its
/// four-neighbour cross touches more than one basin; the number that matters
/// is how much of that boundary touches ALL THREE at once, because two
/// regions can meet along a line and three cannot.
pub struct Tangle {
    pub samples: usize,
    pub boundary: usize,
    pub all_three: usize,
}

pub fn tangle(grid: &Grid) -> Tangle {
    let mut t = Tangle { samples: 0, boundary: 0, all_three: 0 };
    for i in 1..grid.g - 1 {
        for j in 1..grid.g - 1 {
            let mut seen = [false; 3];
            for v in [
                grid.at(i, j),
                grid.at(i - 1, j),
                grid.at(i + 1, j),
                grid.at(i, j - 1),
                grid.at(i, j + 1),
            ] {
                if v >= 0 {
                    seen[v as usize] = true;
                }
            }
            let n = seen.iter().filter(|&&b| b).count();
            t.samples += 1;
            if n > 1 {
                t.boundary += 1;
            }
            if n == 3 {
                t.all_three += 1;
            }
        }
    }
    t
}

/// A small picture, for the README. Two basins read as a clean split down
/// the middle; three do not read as anything you could draw with a ruler.
///
/// TRAP, carried forward from v4 with a name on it: this divides by `w - 1`,
/// so `w == 1` panicked on a division by zero with no message at all. It is
/// an assert now, because a picture one column wide is a caller's mistake and
/// ought to say which caller.
pub fn ascii(grid: &Grid, w: usize) -> Vec<String> {
    assert!(w >= 2, "ascii(): a picture needs at least 2 columns, got {w}");
    let marks = *b".o#";
    let mut rows = Vec::with_capacity(w);
    for jj in 0..w {
        let j = jj * (grid.g - 1) / (w - 1);
        let mut line = Vec::with_capacity(w);
        for ii in 0..w {
            let i = ii * (grid.g - 1) / (w - 1);
            let v = grid.at(i, grid.g - 1 - j);
            line.push(if v < 0 { b' ' } else { marks[v as usize] });
        }
        rows.push(String::from_utf8(line).unwrap());
    }
    rows
}

// ---- v5: the cell -> complex coordinate ---------------------------------

/// The site's own Hankel position of a cell on its band.
///
/// `stalk.js:102-110` lays the stalk into the square "anti-diagonal by
/// anti-diagonal, each read from the bottom-left corner upward":
///
/// ```text
/// for(let d = 0; d <= 2*(n-1); d++)
///   for(let r = Math.min(n-1, d); r >= 0; r--){ const c = d - r; ... }
/// ```
///
/// The walk starts at the largest valid `r` and decreases, so the `k`-th cell
/// of band `d` has `r = min(n-1, d) - k`, and inverting that is the whole
/// function. Pinned against `cellOrder` ITSELF by `pin::site_cell_order`.
#[inline]
pub fn hankel_k(r: usize, c: usize, n: usize) -> usize {
    debug_assert!(r < n && c < n);
    (n - 1).min(r + c) - r
}

/// The coordinate this round is built on:
///
/// ```text
/// z(r, c) = rho(r, c) * exp(2*pi*i * k / arcs(n)[r+c])
/// ```
///
/// `rho` is v4's and fixes the modulus. The angle is the free variable `rho`
/// left behind -- and that freedom is exactly the `arcs(n)[d]` cells sharing
/// one band, so the site's own fill order supplies it. Nothing here is
/// invented: the band gives the radius, the Hankel walk gives the angle.
///
/// `z` is injective over the grid. Within a band the moduli agree and the
/// `arcs(n)[d]` angles are distinct; across bands the moduli are distinct
/// powers of two.
pub fn z_of(r: usize, c: usize, n: usize) -> C {
    let rho = crate::fold::rho_of(r, c, n);
    let k = hankel_k(r, c, n);
    let m = crate::fold::arcs(n)[r + c];
    let th = 2.0 * std::f64::consts::PI * k as f64 / m as f64;
    C::new(rho * th.cos(), rho * th.sin())
}

/// Which root of `z^3 - 1` Newton reaches from a cell, and whether it needed
/// the fallback.
///
/// A partition must be TOTAL, so a point that has not settled inside the
/// iteration cap is resolved by nearest root -- and the count of those is
/// carried out of here in the `bool` and reported, rather than swallowed into
/// a class that then looks as principled as the rest.
pub fn cubic_class_of(r: usize, c: usize, n: usize, roots: &[C], iters: usize) -> (u8, bool) {
    let z = z_of(r, c, n);
    match basin(z, newton3, roots, iters) {
        Some(k) => (k as u8, false),
        None => {
            let mut w = z;
            for _ in 0..iters {
                let nx = newton3(w);
                if !nx.finite() {
                    break;
                }
                w = nx;
            }
            let mut best = 0usize;
            let mut bd = f64::INFINITY;
            for (k, rt) in roots.iter().enumerate() {
                let d = w.dist2(*rt);
                if d < bd {
                    bd = d;
                    best = k;
                }
            }
            (best as u8, true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// N1, the coordinate: the modulus is `rho` exactly, and `z` is injective
    /// over the whole grid.
    #[test]
    fn the_coordinate_has_rho_for_its_modulus_and_is_injective() {
        for n in 2..=40usize {
            let mut seen: Vec<(u64, u64)> = Vec::with_capacity(n * n);
            for r in 0..n {
                for c in 0..n {
                    let z = z_of(r, c, n);
                    let rho = crate::fold::rho_of(r, c, n);
                    assert!(z.finite(), "z is not finite at ({r},{c},{n})");
                    assert!(
                        (z.abs() - rho).abs() <= 1e-12 * rho.max(1.0),
                        "|z| {} is not rho {rho} at ({r},{c},{n})",
                        z.abs()
                    );
                    // the AS-COMPUTED bits, not a quantisation of them: two
                    // cells could in principle round to the same f64 pair even
                    // though the exact values differ, and that would be a real
                    // collision for every consumer of this coordinate.
                    seen.push((z.re.to_bits(), z.im.to_bits()));
                }
            }
            let before = seen.len();
            seen.sort_unstable();
            seen.dedup();
            assert_eq!(seen.len(), before, "z collides at n={n}");
        }
    }

    /// The Hankel position against the site's own walk, restated in Rust. The
    /// pin against `cellOrder` itself lives in `pin.rs`; this is the cheap
    /// always-on half.
    #[test]
    fn hankel_k_inverts_the_sites_fill_order() {
        for n in 2..=40usize {
            let arcs = crate::fold::arcs(n);
            let mut i = 0usize;
            for (d, &band) in arcs.iter().enumerate() {
                let mut r = (n - 1).min(d) as i64;
                while r >= 0 {
                    let c = d as i64 - r;
                    if (0..n as i64).contains(&c) {
                        let (ru, cu) = (r as usize, c as usize);
                        assert_eq!(hankel_k(ru, cu, n), (n - 1).min(d) - ru, "k at ({ru},{cu},{n})");
                        assert!(hankel_k(ru, cu, n) < band, "k is off its band at ({ru},{cu},{n})");
                        i += 1;
                    }
                    r -= 1;
                }
            }
            assert_eq!(i, n * n, "the walk missed cells at n={n}");
        }
    }

    /// Every band's `k = 0` cell sits at angle 0, on the positive real axis,
    /// which is deep inside root 1's basin. That is the mechanism behind the
    /// filed prediction that the cubic arm's class 0 comes out heavy: there
    /// are `2n-1` bands and every one of them donates its first cell.
    #[test]
    fn the_first_cell_of_every_band_is_on_the_positive_real_axis() {
        let rs = roots3();
        for n in [8usize, 16, 32] {
            for d in 0..=2 * (n - 1) {
                let r = (n - 1).min(d);
                let c = d - r;
                assert_eq!(hankel_k(r, c, n), 0);
                let z = z_of(r, c, n);
                assert_eq!(z.im, 0.0, "band {d} at n={n} is not on the real axis");
                assert!(z.re > 0.0);
                assert_eq!(cubic_class_of(r, c, n, &rs, 200), (0, false));
            }
        }
    }

    /// P2. Cayley's two-root case: the basin is decided by the sign of the
    /// real part, with no exceptions. Ground: 159,598 of 159,598.
    #[test]
    fn the_quadratic_boundary_is_a_straight_line() {
        let g = 401usize;
        let span = 2.0f64;
        let rs = roots2();
        let mut tested = 0usize;
        for i in 0..g {
            let x = -span + 2.0 * span * i as f64 / (g - 1) as f64;
            if x.abs() < 0.02 {
                continue;
            }
            for j in 0..g {
                let y = -span + 2.0 * span * j as f64 / (g - 1) as f64;
                let want = if x > 0.0 { 0 } else { 1 };
                assert_eq!(
                    basin(C::new(x, y), newton2, &rs, 200),
                    Some(want),
                    "at ({x},{y})"
                );
                tested += 1;
            }
        }
        assert_eq!(tested, 159_598);
    }

    /// P2, the other half: under the Moebius map the two basins become the
    /// inside and the outside of the unit circle. Ground: 159,597 of 159,597.
    #[test]
    fn the_moebius_map_turns_the_line_into_the_unit_circle() {
        let g = 401usize;
        let span = 2.0f64;
        let rs = roots2();
        let mut tested = 0usize;
        for i in 0..g {
            let x = -span + 2.0 * span * i as f64 / (g - 1) as f64;
            if x.abs() < 0.02 {
                continue;
            }
            for j in 0..g {
                let y = -span + 2.0 * span * j as f64 / (g - 1) as f64;
                let Some(w) = mobius(C::new(x, y)) else {
                    continue; // the pole at z = -1 has no image
                };
                let inside = w.norm2() < 1.0;
                let b = basin(C::new(x, y), newton2, &rs, 200);
                assert_eq!(b == Some(0), inside, "at ({x},{y})");
                tested += 1;
            }
        }
        assert_eq!(tested, 159_597);
    }

    /// P2, the map itself: off the circle, the modulus decides the fate.
    /// Ground: 158,265 of 158,265.
    #[test]
    fn zsq_fates_follow_the_modulus() {
        let g = 401usize;
        let span = 2.0f64;
        let mut tested = 0usize;
        for i in 0..g {
            let x = -span + 2.0 * span * i as f64 / (g - 1) as f64;
            for j in 0..g {
                let y = -span + 2.0 * span * j as f64 / (g - 1) as f64;
                let m = (x * x + y * y).sqrt();
                if m < 0.98 || m > 1.02 {
                    let want = if m < 1.0 { Fate::Zero } else { Fate::Infinity };
                    assert_eq!(fate_zsq(C::new(x, y), 200), want, "at ({x},{y})");
                    tested += 1;
                }
            }
        }
        assert_eq!(tested, 158_265);
    }

    /// P4. On the circle `z -> z^2` is the doubling map, which the site
    /// already cites for "dropping a digit; orbits and periods"
    /// (`inspirations.html:311-315`). So the site named what the map does ON
    /// the Fold without naming the Fold as the map's invariant set.
    #[test]
    fn on_the_circle_squaring_is_the_doubling_map() {
        let n = 4096;
        for k in 0..n {
            let th = -std::f64::consts::PI + 2.0 * std::f64::consts::PI * k as f64 / n as f64;
            let z = C::new(th.cos(), th.sin());
            let w = zsq(z);
            assert!((w.abs() - 1.0).abs() < 1e-12, "the circle is invariant");
            let want = 2.0 * th;
            let got = w.arg();
            let two_pi = 2.0 * std::f64::consts::PI;
            let mut diff = (got - want) % two_pi;
            if diff > std::f64::consts::PI {
                diff -= two_pi;
            }
            if diff < -std::f64::consts::PI {
                diff += two_pi;
            }
            assert!(diff.abs() < 1e-9, "arg doubling at theta={th}: {got} vs {want}");
        }
    }

    /// P3. Cayley's wall. A large share of the cubic's boundary touches all
    /// three basins, which no straight seam can do.
    #[test]
    fn the_cubic_boundary_touches_all_three_basins() {
        let grid = basin_grid(301, 2.0, newton3, &roots3());
        let t = tangle(&grid);
        assert_eq!(t.samples, 89_401);
        let share = t.all_three as f64 / t.boundary as f64;
        assert!(
            (0.30..=0.45).contains(&share),
            "all-three share {share} of {} boundary cells",
            t.boundary
        );
    }
}
