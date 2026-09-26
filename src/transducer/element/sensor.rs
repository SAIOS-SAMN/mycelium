// Law 11, 71 — THE SENSOR: the readers permitted the base's coordinates,
// which read a site of ℚⁿ exactly or refuse.
//
//! The functions permitted a site's coordinates: a conformal point read to
//! the site whose compartment holds it, a compartment's center and a whole
//! compartment carried by a versor, and a sibling's assignment pulled back
//! onto a base's sites. A compartment (16.4) is the unit square standing on
//! the grid axes at a site, with its center carried at the site plus ½ on each grid axis and
//! the site's own coordinate on every other axis.
//!
//! THIS IS THE BOUNDARY WHERE THE CORPUS'S COORDINATES BECOME SITES (71.2(1)).
//! The grid reads three of them — row, column, frame — and they enter the
//! base here, as a [`Site`] carrying its `n`; above this file no tuple of
//! coordinates is carried. The readers below take and return sites; the
//! tuple forms are kept for the sensor's own callers that read a grid.

use num_bigint::BigInt;
use super::*;
use crate::transducer::base::{is_grid_axis, Site};

/// THE `n` THE GRID PRESENTS (71.2(1)): row, column and frame. It is read
/// here, at intake, and every site the sensor hands over carries it; a form
/// extracted from grids stands on this base.
pub const GRID_N: u8 = 3;

/// A point of the base, `(r, c, k)` in ℚ³ (51.1): the corpus's three
/// coordinates as the grid presents them.
pub type Point = (num_rational::BigRational, num_rational::BigRational, num_rational::BigRational);

/// THE CORPUS'S THREE COORDINATES AS A SITE: row, column, frame, read at the
/// sensor and carried from here as a site of `n = 3`.
impl From<(i64, i64, i64)> for Site {
    fn from((r, c, k): (i64, i64, i64)) -> Site {
        Site::new(vec![r, c, k])
    }
}

/// A COMPARTMENT AS THE GRID PRESENTS IT: row, column, frame and the hue it
/// carries, read at the sensor.
impl From<(i64, i64, i64, u8)> for crate::transducer::blade::Compartment {
    fn from((r, c, k, hue): (i64, i64, i64, u8)) -> Self {
        crate::transducer::blade::Compartment { site: Site::new(vec![r, c, k]), hue }
    }
}

/// The site whose compartment holds a rational point of the base — exact
/// containment (the floor of a rational is an exact integer, not a
/// rounding) on every axis. `None` where a coordinate does not fit the base
/// (W6): a site it could not convert is a site it did not read.
pub fn site_at(p: &[num_rational::BigRational]) -> Option<Site> {
    p.iter()
        .map(|x| i64::try_from(x.floor().to_integer()).ok())
        .collect::<Option<Vec<i64>>>()
        .map(Site::new)
}

/// The same reading for a point of the corpus's three coordinates.
pub fn site_of(p: &Point) -> Option<(i64, i64, i64)> {
    let s = site_at(&[p.0.clone(), p.1.clone(), p.2.clone()])?;
    match s.axes() {
        [r, c, k] => Some((*r, *c, *k)),
        _ => None,
    }
}

/// Carry a compartment's center by V and read the base point back (None at
/// infinity). The center stands at the site plus ½ on each grid axis: the
/// other axes' origins lie at the compartments' centers (16.4), so a dilator
/// about a point of the frame fixes the frame. At the site's own `n`.
pub fn carry_site_center(v: &Versor, s: &Site) -> Option<Vec<num_rational::BigRational>> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x: Vec<num_rational::BigRational> = s
        .axes()
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let q = num_rational::BigRational::from_integer(BigInt::from(*a));
            match is_grid_axis(i) {
                true => q + &half,
                false => q,
            }
        })
        .collect();
    crate::transducer::cga::base_at(&v.sandwich(&crate::transducer::cga::point_at(&x)), s.n())
}

/// The site a compartment's carried center lands in.
pub fn carry_site(v: &Versor, s: &Site) -> Option<Site> {
    site_at(&carry_site_center(v, s)?)
}

/// Carry a compartment's center at the corpus's three coordinates.
pub fn carry_center(v: &Versor, r: i64, c: i64, k: i64) -> Option<Point> {
    match carry_site_center(v, &Site::new(vec![r, c, k]))?.as_slice() {
        [x, y, z] => Some((x.clone(), y.clone(), z.clone())),
        _ => None,
    }
}

/// Carry a whole compartment by V — its unit square standing on the grid
/// axes — and read back the block of compartments its image IS: the sites whose squares
/// tile it exactly. The square is carried by its four corners, and the
/// image is a block of the sensor only where the corners land on the
/// base's vertices, with every non-grid coordinate shared by all four, as
/// an axis-aligned square of integer side: one site under an isometry, an
/// s×s block under a dilation by an integer s. Everything else is no
/// compartment of the sensor and is refused (law 11): a tilted image (a rotor off the
/// 90° turns), a fractional image (a shrink, a dilation by a ratio), a
/// square standing across the frames (a versor turning a non-grid axis into
/// the plane, 16.4), a corner at infinity or off the base (W6). Nothing is
/// read off a span of centres.
pub fn carry_compartment_site(v: &Versor, s: &Site) -> Option<Vec<Site>> {
    let [ga, gb] = crate::transducer::base::GRID_AXES;
    // A corner carried onto a vertex of the base, or nothing.
    let corner = |da: i64, db: i64| -> Option<Vec<i64>> {
        let x: Vec<num_rational::BigRational> = s
            .axes()
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let d = match i {
                    i if i == ga => da,
                    i if i == gb => db,
                    _ => 0,
                };
                num_rational::BigRational::from_integer(BigInt::from(a + d))
            })
            .collect();
        let p = crate::transducer::cga::base_at(
            &v.sandwich(&crate::transducer::cga::point_at(&x)),
            s.n(),
        )?;
        p.iter()
            .map(|q| match q.is_integer() {
                true => i64::try_from(q.to_integer()).ok(),
                false => None,
            })
            .collect()
    };
    let (p00, p10, p01, p11) = (corner(0, 0)?, corner(1, 0)?, corner(0, 1)?, corner(1, 1)?);
    // Every non-grid coordinate is one for all four corners: the image
    // stands in one frame.
    let off_grid_equal = |p: &[i64]| {
        p.iter().enumerate().all(|(i, a)| is_grid_axis(i) || p00.get(i) == Some(a))
    };
    if !(off_grid_equal(&p10) && off_grid_equal(&p01) && off_grid_equal(&p11)) {
        return None;
    }
    let at = |p: &[i64], i: usize| p.get(i).copied();
    let a = (at(&p10, ga)? - at(&p00, ga)?, at(&p10, gb)? - at(&p00, gb)?);
    let b = (at(&p01, ga)? - at(&p00, ga)?, at(&p01, gb)? - at(&p00, gb)?);
    let along = |d: (i64, i64)| -> Option<(bool, i64)> {
        match (d.0 == 0, d.1 == 0) {
            (false, true) => Some((true, d.0.abs())),
            (true, false) => Some((false, d.1.abs())),
            _ => None,
        }
    };
    let (a_rows, side) = along(a)?;
    let (b_rows, side_b) = along(b)?;
    if a_rows == b_rows || side != side_b {
        return None;
    }
    if at(&p11, ga)? != at(&p00, ga)? + a.0 + b.0 || at(&p11, gb)? != at(&p00, gb)? + a.1 + b.1 {
        return None;
    }
    // The block: `side` rows and `side` columns from the image's lowest
    // corner, each read off the sign of the edge along that axis, at the
    // image's own non-grid coordinates.
    let r0 = if a.0 + b.0 < 0 { at(&p00, ga)? - side } else { at(&p00, ga)? };
    let c0 = if a.1 + b.1 < 0 { at(&p00, gb)? - side } else { at(&p00, gb)? };
    let mut out = Vec::new();
    for i in r0..r0 + side {
        for j in c0..c0 + side {
            let mut axes = p00.clone();
            axes[ga] = i;
            axes[gb] = j;
            out.push(Site::new(axes));
        }
    }
    Some(out)
}

/// A whole compartment carried at the corpus's three coordinates.
pub fn carry_compartment(v: &Versor, r: i64, c: i64, k: i64) -> Option<Vec<(i64, i64, i64)>> {
    carry_compartment_site(v, &Site::new(vec![r, c, k]))?
        .iter()
        .map(|s| match s.axes() {
            [r, c, k] => Some((*r, *c, *k)),
            _ => None,
        })
        .collect()
}

/// THE PULLBACK of a sibling's assignment onto a base support through the
/// versor V carrying the sibling onto the base: each base site's center is
/// carried back by V⁻¹ to the sibling's frame, and the sibling link whose
/// square contains it speaks the transition. Both are sections in their
/// own canonical frame (16.5), so a center carried out of that frame lands
/// on no site. Exact at every scale (a dilated sibling's block, a shrunken
/// sibling's shared site); None when a base site lands outside the sibling.
pub fn pullback(
    base: &[(i64, i64)],
    sibling: &[(i64, i64, u8, u8)],
    v: &Versor,
) -> Option<Vec<(u8, u8)>> {
    // Back by `ṽ`: a centre is read off its point by proportion (78.32(1)).
    crate::transducer::cga::is_versor(v).then_some(())?;
    let inv = v.reverse();
    let mut sorted: Vec<(i64, i64, u8, u8)> = sibling.to_vec();
    sorted.sort_unstable();
    base.iter()
        .map(|&(r, c)| {
            let (sr, sc, sk) = site_of(&carry_center(&inv, r, c, 0)?)?;
            if sk != 0 {
                return None;
            }
            sorted
                .binary_search_by(|x| (x.0, x.1).cmp(&(sr, sc)))
                .ok()
                .map(|i| (sorted[i].2, sorted[i].3))
        })
        .collect()
}
