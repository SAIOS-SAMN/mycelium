// Law 11 — THE SENSOR: the one place in the element where a coordinate
// pair is read or written.
//
//! Everything above the sensor is versors and words. Here, and only here,
//! a conformal point is rounded to a cell, a versor is applied to a cell,
//! and a sibling's assignment is pulled back onto a base's positions. The
//! squaring gate names this file in its allow-list for exactly that
//! reason.

use num_bigint::BigInt;
use super::*;

/// The cell whose unit square contains a rational point — exact
/// containment on the sensor's own lattice (the floor of a rational is an
/// exact integer, not a rounding).
pub fn cell_of(p: &(num_rational::BigRational, num_rational::BigRational)) -> (i64, i64) {
    let f = |x: &num_rational::BigRational| -> i64 { x.floor().to_integer().try_into().unwrap_or(i64::MAX) };
    (f(&p.0), f(&p.1))
}

/// Carry a cell's center by V and read the base point back (None at
/// infinity).
pub fn carry_center(v: &Versor, r: i64, c: i64) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x = num_rational::BigRational::from_integer(BigInt::from(r)) + &half;
    let y = num_rational::BigRational::from_integer(BigInt::from(c)) + &half;
    crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y)))
}

/// Carry a whole cell by V as a blade — its unit square — and read back the
/// cells whose centers the image square holds: one cell under an
/// isometry, an s×s block under a dilation by s, the one shared cell (or
/// none) under a shrink. The support is carried forward; nothing outside
/// it is visited. None at infinity.
pub fn carry_cell(v: &Versor, r: i64, c: i64) -> Option<Vec<(i64, i64)>> {
    let corner = |dr: i64, dc: i64| -> Option<(num_rational::BigRational, num_rational::BigRational)> {
        let x = num_rational::BigRational::from_integer(BigInt::from(r + dr));
        let y = num_rational::BigRational::from_integer(BigInt::from(c + dc));
        crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y)))
    };
    let (p0, p1) = (corner(0, 0)?, corner(1, 1)?);
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    // The cells whose center (i + ½) lies in the half-open image range
    // [lo, hi): i ≥ ⌈lo − ½⌉ and i + ½ < hi.
    let span = |a: &num_rational::BigRational, b: &num_rational::BigRational| -> (i64, i64) {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let first: i64 = (lo - &half).ceil().to_integer().try_into().unwrap_or(i64::MAX);
        let last: i64 = (hi - &half).ceil().to_integer().try_into().unwrap_or(i64::MIN); // exclusive
        (first, last)
    };
    let (r0, r1) = span(&p0.0, &p1.0);
    let (c0, c1) = span(&p0.1, &p1.1);
    let mut out = Vec::new();
    for i in r0..r1 {
        for j in c0..c1 {
            out.push((i, j));
        }
    }
    Some(out)
}

/// THE PULLBACK of a sibling's assignment onto a base support through the
/// versor V carrying the sibling onto the base: each base cell's center is
/// carried back by V⁻¹ to the sibling's frame, and the sibling cell whose
/// square contains it speaks the transition. Exact at every scale (a
/// dilated sibling's block, a shrunken sibling's shared cell); None when a
/// base cell lands outside the sibling.
pub fn pullback(
    base: &[(i64, i64)],
    sibling: &[(i64, i64, u8, u8)],
    v: &Versor,
) -> Option<Vec<(u8, u8)>> {
    let inv = v.versor_inverse()?;
    let mut sorted: Vec<(i64, i64, u8, u8)> = sibling.to_vec();
    sorted.sort_unstable();
    base.iter()
        .map(|&(r, c)| {
            let (sr, sc) = cell_of(&carry_center(&inv, r, c)?);
            sorted
                .binary_search_by(|x| (x.0, x.1).cmp(&(sr, sc)))
                .ok()
                .map(|i| (sorted[i].2, sorted[i].3))
        })
        .collect()
}
