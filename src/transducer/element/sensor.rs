// Law 11 — THE SENSOR: the one place in the element where a coordinate
// pair is read or written.
//
//! The four functions permitted a coordinate pair: a conformal point rounded
//! to a cell, a cell's center and a whole cell carried by a versor, and a
//! sibling's assignment pulled back onto a base's positions.

use num_bigint::BigInt;
use num_traits::Zero;
use super::*;

/// A point of the base, `(r, c, k)` in ℚ³ (51.1).
pub type Point = (num_rational::BigRational, num_rational::BigRational, num_rational::BigRational);

/// The cell whose unit cube contains a rational point of the base — exact
/// containment on the sensor's own lattice (the floor of a rational is an
/// exact integer, not a rounding). The cells this sensor reads stand in
/// frame `0` until a cell carries `k` (51.3): a point carried into another
/// frame lands on no cell of it, which is the reading and not a rounding
/// back (rule 11). `None` likewise where a coordinate does not fit the
/// lattice (W6): a cell it could not convert is a cell it did not read.
pub fn cell_of(p: &Point) -> Option<(i64, i64)> {
    let f = |x: &num_rational::BigRational| -> Option<i64> {
        i64::try_from(x.floor().to_integer()).ok()
    };
    if f(&p.2)? != 0 {
        return None;
    }
    Some((f(&p.0)?, f(&p.1)?))
}

/// The cell whose unit cube contains a rational point in (r, c, k).
pub fn cell_of_3d(p: &Point) -> Option<(i64, i64, i64)> {
    let f = |x: &num_rational::BigRational| -> Option<i64> {
        i64::try_from(x.floor().to_integer()).ok()
    };
    Some((f(&p.0)?, f(&p.1)?, f(&p.2)?))
}

/// Carry a cell's center by V and read the base point back (None at
/// infinity). The cell stands in frame `0`, and a cell of frame `k` stands
/// AT `k` — the frame axis's origin lies at the cells' centers (16.4), so
/// a dilator about the origin fixes the frame.
pub fn carry_center(v: &Versor, r: i64, c: i64) -> Option<Point> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x = num_rational::BigRational::from_integer(BigInt::from(r)) + &half;
    let y = num_rational::BigRational::from_integer(BigInt::from(c)) + &half;
    crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y, &num_rational::BigRational::zero())))
}

/// Carry a cell's center at (r, c, k) by V in 3D and read the base point back.
pub fn carry_center_3d(v: &Versor, r: i64, c: i64, k: i64) -> Option<Point> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x = num_rational::BigRational::from_integer(BigInt::from(r)) + &half;
    let y = num_rational::BigRational::from_integer(BigInt::from(c)) + &half;
    let z = num_rational::BigRational::from_integer(BigInt::from(k));
    crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y, &z)))
}

/// Carry a whole cell by V as a blade — its unit square in its frame — and
/// read back the cells whose centers the image holds: one cell under an
/// isometry, an s×s block under a dilation by s, the one shared cell (or
/// none) under a shrink. The support is carried forward; nothing outside
/// it is visited. None at infinity, and none where the cell's center
/// leaves frame `0`.
pub fn carry_cell(v: &Versor, r: i64, c: i64) -> Option<Vec<(i64, i64)>> {
    cell_of(&carry_center(v, r, c)?)?;
    let corner = |dr: i64, dc: i64| -> Option<Point> {
        let x = num_rational::BigRational::from_integer(BigInt::from(r + dr));
        let y = num_rational::BigRational::from_integer(BigInt::from(c + dc));
        crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y, &num_rational::BigRational::zero())))
    };
    let (p0, p1) = (corner(0, 0)?, corner(1, 1)?);
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    // The cells whose center (i + ½) lies in the half-open image range
    // [lo, hi): i ≥ ⌈lo − ½⌉ and i + ½ < hi.
    // A span whose ends do not fit the lattice is a span this reader did
    // not measure (W6), and no cells are carried for it.
    let span = |a: &num_rational::BigRational,
                b: &num_rational::BigRational|
     -> Option<(i64, i64)> {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let first = i64::try_from((lo - &half).ceil().to_integer()).ok()?;
        let last = i64::try_from((hi - &half).ceil().to_integer()).ok()?; // exclusive
        Some((first, last))
    };
    let (r0, r1) = span(&p0.0, &p1.0)?;
    let (c0, c1) = span(&p0.1, &p1.1)?;
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
            let (sr, sc) = cell_of(&carry_center(&inv, r, c)?)?;
            sorted
                .binary_search_by(|x| (x.0, x.1).cmp(&(sr, sc)))
                .ok()
                .map(|i| (sorted[i].2, sorted[i].3))
        })
        .collect()
}
