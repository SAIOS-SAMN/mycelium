// Law 11 — THE SENSOR: the readers permitted the base's coordinates, which
// read the three of them (51.1) exactly or refuse.
//
//! The four functions permitted a site's coordinates: a conformal point
//! read to the site whose lamina holds it, a lamina's center and a whole
//! lamina carried by a versor, and a sibling's assignment pulled back onto
//! a base's sites. A lamina (16.4) is the unit square standing in its
//! frame at a site, with its center carried at `(r + ½, c + ½, k)`.

use num_bigint::BigInt;
use super::*;

/// A point of the base, `(r, c, k)` in ℚ³ (51.1).
pub type Point = (num_rational::BigRational, num_rational::BigRational, num_rational::BigRational);

/// The site whose lamina holds a rational point of the base — exact
/// containment on the sensor's own base (the floor of a rational is an
/// exact integer, not a rounding) on the two plane axes, and the frame the
/// point stands in along `e₃`. `None` where a coordinate does not fit the
/// base (W6): a site it could not convert is a site it did not read.
pub fn site_of(p: &Point) -> Option<(i64, i64, i64)> {
    let f = |x: &num_rational::BigRational| -> Option<i64> {
        i64::try_from(x.floor().to_integer()).ok()
    };
    Some((f(&p.0)?, f(&p.1)?, f(&p.2)?))
}

/// Carry a lamina's center by V and read the base point back (None at
/// infinity). The center stands at `(r + ½, c + ½, k)`: the frame axis's
/// origin lies at the laminae's centers (16.4), so a dilator about a point
/// of the frame fixes the frame.
pub fn carry_center(v: &Versor, r: i64, c: i64, k: i64) -> Option<Point> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x = num_rational::BigRational::from_integer(BigInt::from(r)) + &half;
    let y = num_rational::BigRational::from_integer(BigInt::from(c)) + &half;
    let z = num_rational::BigRational::from_integer(BigInt::from(k));
    crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y, &z)))
}

/// Carry a whole lamina by V — its unit square standing in its frame — and
/// read back the block of laminae its image IS: the sites whose squares
/// tile it exactly. The square is carried by its four corners, and the
/// image is a block of the sensor only where the corners land on the
/// base's vertices, in one frame, as an axis-aligned square of integer
/// side: one site under an isometry, an s×s block under a dilation by an
/// integer s. Everything else is no lamina of the sensor and is refused
/// (law 11): a tilted image (a rotor off the quarter turns), a fractional
/// image (a shrink, a dilation by a ratio), a square standing across the
/// frames (a versor turning `e₃` into the plane, 16.4), a corner at
/// infinity or off the base (W6). Nothing is read off a span of centres:
/// which laminae a carried square covers is its corners' to say, and a
/// square whose corners fall between vertices covers no lamina exactly. A
/// body a versor shrinks is carried as a body, by `blade::carries`, which
/// reads where each lamina's centre lands; that reading is the body's and
/// never the lamina's.
pub fn carry_lamina(v: &Versor, r: i64, c: i64, k: i64) -> Option<Vec<(i64, i64, i64)>> {
    let z = num_rational::BigRational::from_integer(BigInt::from(k));
    // A corner carried onto a vertex of the base, or nothing: a coordinate
    // between two vertices, or one that does not fit the base, is a square
    // no laminae tile.
    let corner = |dr: i64, dc: i64| -> Option<(i64, i64, i64)> {
        let x = num_rational::BigRational::from_integer(BigInt::from(r + dr));
        let y = num_rational::BigRational::from_integer(BigInt::from(c + dc));
        let p = crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y, &z)))?;
        if !(p.0.is_integer() && p.1.is_integer() && p.2.is_integer()) {
            return None;
        }
        Some((
            i64::try_from(p.0.to_integer()).ok()?,
            i64::try_from(p.1.to_integer()).ok()?,
            i64::try_from(p.2.to_integer()).ok()?,
        ))
    };
    let (p00, p10, p01, p11) = (corner(0, 0)?, corner(1, 0)?, corner(0, 1)?, corner(1, 1)?);
    if p10.2 != p00.2 || p01.2 != p00.2 || p11.2 != p00.2 {
        return None;
    }
    // The image's two edges from the carried origin corner. A block's
    // square has each edge along one axis, the two along different axes
    // and of one length, and its fourth corner where the two edges meet
    // again.
    let a = (p10.0 - p00.0, p10.1 - p00.1);
    let b = (p01.0 - p00.0, p01.1 - p00.1);
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
    if p11 != (p00.0 + a.0 + b.0, p00.1 + a.1 + b.1, p00.2) {
        return None;
    }
    // The block: `side` rows and `side` columns from the image's lowest
    // corner, each read off the sign of the edge along that axis.
    let r0 = if a.0 + b.0 < 0 { p00.0 - side } else { p00.0 };
    let c0 = if a.1 + b.1 < 0 { p00.1 - side } else { p00.1 };
    let mut out = Vec::new();
    for i in r0..r0 + side {
        for j in c0..c0 + side {
            out.push((i, j, p00.2));
        }
    }
    Some(out)
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
    let inv = v.versor_inverse()?;
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
