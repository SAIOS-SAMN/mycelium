// Law 11, 71 — THE SENSOR: the readers permitted the base's coordinates,
// which read a site of ℚⁿ exactly or refuse.
//
//! The functions permitted a site's coordinates: a conformal point read to
//! the site whose compartment holds it, a compartment's centre and a whole
//! compartment carried by a versor, and a sibling's assignment pulled back
//! onto a base's sites. A compartment (16.4) is the unit square standing on
//! the raster axes at a site, with its centre carried at the site plus ½ on each raster axis and
//! the site's own coordinate on every other axis.
//!
//! THIS IS THE BOUNDARY WHERE THE CORPUS'S COORDINATES BECOME SITES (71.2(1)).
//! The raster reads three of them — row, column, frame — and they enter the
//! base here, as a [`Site`] carrying its `n`; above this file no tuple of
//! coordinates is carried. The readers below take and return sites; the
//! tuple forms are kept for the sensor's own callers that read a raster.

use num_bigint::BigInt;
use super::*;
use crate::transducer::base::Site;
use crate::transducer::perspective::{family_role, Role};

/// THE `n` OF THE BASE A FORM IS WRITTEN ON BY DEFAULT (71.2(1)): the
/// alphabet's base, a sample of the rank the letters spell and its place
/// in its family (`alphabet::ALPHABET_BASE_N`, derived from the letters'
/// rank). A form read from a sample of another rank carries that rank's
/// `n` on its address.
#[expect(
    clippy::cast_possible_truncation,
    reason = "STD1 C1 — the algebra's range: ALPHABET_BASE_N = LETTER_RANK − 1 = 3"
)]
pub const BASE_N: u8 = crate::transducer::alphabet::ALPHABET_BASE_N as u8;

/// A point of the base, `(r, c, k)` in ℚ³ (51.1): the corpus's three
/// coordinates as the raster presents them.
pub type Point = (num_rational::BigRational, num_rational::BigRational, num_rational::BigRational);

/// THE CORPUS'S THREE COORDINATES AS A SITE: row, column, frame, read at the
/// sensor and carried from here as a site of `n = 3`.
impl From<(i64, i64, i64)> for Site {
    fn from((r, c, k): (i64, i64, i64)) -> Site {
        Site::of([r, c, k])
    }
}

/// A COMPARTMENT AS THE RASTER PRESENTS IT: row, column, frame and the hue it
/// carries, read at the sensor.
impl From<(i64, i64, i64, u8)> for crate::transducer::blade::Compartment {
    fn from((r, c, k, hue): (i64, i64, i64, u8)) -> Self {
        crate::transducer::blade::Compartment { site: Site::of([r, c, k]), hue }
    }
}

/// A PAINTED SITE OF THE SECTION AS A COMPARTMENT of the family's base, in
/// the frame its scene stands in (the place `0`), read at the sensor.
impl From<crate::transducer::spelled::PaintedSite> for crate::transducer::blade::Compartment {
    fn from(p: crate::transducer::spelled::PaintedSite) -> Self {
        let [r, c] = p.site.coordinates();
        crate::transducer::blade::Compartment { site: Site::of([r, c, 0]), hue: p.symbol }
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
        .and_then(Site::new)
}

/// The same reading for a point of the corpus's three coordinates.
pub fn site_of(p: &Point) -> Option<(i64, i64, i64)> {
    let s = site_at(&[p.0.clone(), p.1.clone(), p.2.clone()])?;
    match s.axes() {
        [r, c, k] => Some((*r, *c, *k)),
        _ => None,
    }
}

/// Carry a compartment's centre by V and read the base point back (None at
/// infinity). The centre stands at the site plus ½ on each axis the default
/// perspective reads as space (the compartment's cube spans them): the
/// place's origin lies at the compartments' centres (16.4), so a dilator
/// about a point of the frame fixes the frame. At the site's own `n`.
pub fn carry_site_centre(v: &Versor, s: &Site) -> Option<Vec<num_rational::BigRational>> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x: Vec<num_rational::BigRational> = s
        .axes()
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let q = num_rational::BigRational::from_integer(BigInt::from(*a));
            match family_role(s.n(), i) {
                Some(Role::Space) => q + &half,
                Some(Role::Sequence | Role::Value) | None => q,
            }
        })
        .collect();
    crate::transducer::cga::base_at(&v.sandwich(&crate::transducer::cga::point_at(&x)), s.n())
}

/// The site a compartment's carried centre lands in.
pub fn carry_site(v: &Versor, s: &Site) -> Option<Site> {
    site_at(&carry_site_centre(v, s)?)
}

/// Carry a compartment's centre at the corpus's three coordinates.
pub fn carry_centre(v: &Versor, r: i64, c: i64, k: i64) -> Option<Point> {
    match carry_site_centre(v, &Site::of([r, c, k]))?.as_slice() {
        [x, y, z] => Some((x.clone(), y.clone(), z.clone())),
        _ => None,
    }
}

/// Carry a whole compartment by V — its unit cube standing on the axes the
/// default perspective reads as space (`perspective::Perspective`) — and
/// read back the block of compartments its image IS: the sites whose cubes
/// tile it exactly. The cube is carried by its least corner, the corner one
/// step along each space axis, and its far corner (at two space axes, its
/// four corners), and the image is a block of the sensor only where the
/// corners land on the base's vertices, with every coordinate off the space
/// axes shared by all, as an axis-aligned cube of integer side: one site
/// under an isometry, an `sᵐ` block under a dilation by an integer `s`.
/// Everything else is no compartment of the sensor and is refused (law
/// 11): a tilted image (a rotor off the 90° turns), a fractional image (a
/// shrink, a dilation by a ratio), a cube standing across the frames (a
/// versor turning the place into a space axis, 16.4), a corner at infinity
/// or off the base (W6). Nothing is read off a span of centres. At any `n`:
/// no axis is named.
pub fn carry_compartment_site(v: &Versor, s: &Site) -> Option<Vec<Site>> {
    let space = crate::transducer::perspective::Perspective::of_family_site(s.n()).space_axes();
    // A corner carried onto a vertex of the base, or nothing: the site one
    // step along each space axis in `ones`.
    let corner = |ones: &[usize]| -> Option<Vec<i64>> {
        let x: Vec<num_rational::BigRational> = s
            .axes()
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let d = i64::from(ones.contains(&i));
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
    let least = corner(&[])?;
    let far = corner(&space)?;
    // Every coordinate off the space axes is one for every corner: the
    // image stands in one frame.
    let off_space_equal = |p: &[i64]| p.iter().enumerate().all(|(i, a)| space.contains(&i) || least.get(i) == Some(a));
    // THE EDGES: each space axis's step carried, the image an edge along
    // one space axis, every edge of one side and no two on one axis — a
    // signed permutation of the space axes times the side.
    let mut side: Option<i64> = None;
    let mut taken: Vec<usize> = Vec::with_capacity(space.len());
    let mut lift = vec![0i64; least.len()];
    for &axis in &space {
        let p = corner(&[axis])?;
        if !off_space_equal(&p) {
            return None;
        }
        let d: Vec<i64> = p.iter().zip(&least).map(|(a, b)| a - b).collect();
        let moved: Vec<usize> = (0..d.len()).filter(|&i| d.get(i).is_some_and(|x| *x != 0)).collect();
        let &[along] = moved.as_slice() else { return None };
        let step = *d.get(along)?;
        if !space.contains(&along) || taken.contains(&along) || side.is_some_and(|s| s != step.abs()) {
            return None;
        }
        side = Some(step.abs());
        taken.push(along);
        *lift.get_mut(along)? += step;
    }
    if !off_space_equal(&far) || far.iter().zip(&least).zip(&lift).any(|((f, l), d)| *f != l + d) {
        return None;
    }
    // The block: `side` sites along each space axis from the image's least
    // corner, each read off the sign of the edge along that axis, at the
    // image's own coordinates off the space axes; in canonical order.
    let side = match side {
        Some(s) => s,
        None => 1,
    };
    let mut out: Vec<Vec<i64>> = vec![least.clone()];
    for &axis in &space {
        let lo = least.get(axis)? + (*lift.get(axis)?).min(0);
        out = out
            .into_iter()
            .flat_map(|p| {
                (lo..lo + side).map(move |x| {
                    let mut q = p.clone();
                    if let Some(slot) = q.get_mut(axis) {
                        *slot = x;
                    }
                    q
                })
            })
            .collect();
    }
    out.into_iter().map(Site::new).collect()
}

/// A whole compartment carried at the corpus's three coordinates.
pub fn carry_compartment(v: &Versor, r: i64, c: i64, k: i64) -> Option<Vec<(i64, i64, i64)>> {
    carry_compartment_site(v, &Site::of([r, c, k]))?
        .iter()
        .map(|s| match s.axes() {
            [r, c, k] => Some((*r, *c, *k)),
            _ => None,
        })
        .collect()
}

/// THE PULLBACK of a sibling's assignment onto a base support through the
/// versor V carrying the sibling onto the base: each base site's centre is
/// carried back by V⁻¹ to the sibling's frame, and the sibling link whose
/// square contains it speaks the transition. Both are sections in each one's
/// own canonical frame (16.5), so a centre carried out of that frame lands
/// on no site. Exact at every scale (a dilated sibling's block, a shrunken
/// sibling's shared site); None when a base site lands outside the sibling.
pub fn pullback(
    base: &[crate::transducer::spelled::SectionSite],
    sibling: &[crate::transducer::sentence::SectionLink],
    v: &Versor,
) -> Option<Vec<(u8, u8)>> {
    // Back by `ṽ`: a centre is read off its point by proportion (78.32(1)).
    crate::transducer::cga::is_versor(v).then_some(())?;
    let inv = v.reverse();
    let mut sorted: Vec<crate::transducer::sentence::SectionLink> = sibling.to_vec();
    sorted.sort_unstable();
    base.iter()
        .map(|s| {
            let [r, c] = s.coordinates();
            let (sr, sc, sk) = site_of(&carry_centre(&inv, r, c, 0)?)?;
            if sk != 0 {
                return None;
            }
            let landed = crate::transducer::spelled::SectionSite::on_plane([sr, sc]);
            sorted
                .binary_search_by(|x| x.site.cmp(&landed))
                .ok()
                .and_then(|i| sorted.get(i))
                .map(crate::transducer::sentence::SectionLink::transition)
        })
        .collect()
}
