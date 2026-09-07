// A5, 16.11 — THE RESIDUAL: the centered coboundary chain and the words
// built from one.
//
//! A5 centers a chain to its minimum corner and sorts it, so the same
//! deformation at any position is one substance. This file holds that
//! canonicalization and every constructor that reads a chain: a residual
//! germ, a framed residual, a connection, a translation, and the listing
//! key that names a chain within a frame.

use crate::transducer::germinate::{germ_bytes as residual_word, ResidualGerm};
use crate::core::Q;
use num_bigint::BigInt;
use num_traits::One;
use super::*;

/// The positions skeleton of a centered chain (its base geometry).
pub fn positions(chain: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64)> {
    chain.iter().map(|(r, c, _, _)| (*r, *c)).collect()
}

/// The DESCRIPTION of a perceived residual as a germ — post-excision, the ONLY way
/// cognition mints anything: every non-transposition demand IS a coboundary the
/// eye measured, and every "rotation"/"recolour" the mesh will ever speak is a
/// germinated pattern, never a named solver.
///
/// The chain is CENTERED — the changed-cell set framed with its min-corner at the
/// origin (the substrate's transposition projection, hypha-side per the ruling) —
/// so the same deformation at any position is ONE word, while geometry (the
/// relative offsets between changed cells) is preserved exactly: equal-tally,
/// different-geometry residuals stay distinct. The word/identity IS the centered
/// chain; the antisymmetric hue tally is kept only as the δ-algebra lens.
/// A5: translate min-corner to origin, sort, dedup. None when two
/// distinct transitions occupy one site.
pub fn canonical_chain(cells: &[(i64, i64, u8, u8)]) -> Option<Vec<(i64, i64, u8, u8)>> {
    if cells.is_empty() {
        return Some(Vec::new());
    }
    let min_r = cells.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = cells.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    let mut out: Vec<(i64, i64, u8, u8)> = cells
        .iter()
        .map(|(r, c, b, a)| (r - min_r, c - min_c, *b, *a))
        .collect();
    out.sort_unstable();
    out.dedup();
    for w in out.windows(2) {
        if w[0].0 == w[1].0 && w[0].1 == w[1].1 && (w[0].2, w[0].3) != (w[1].2, w[1].3) {
            return None;
        }
    }
    Some(out)
}

pub fn from_residual(g: &ResidualGerm) -> Element {
    let centered: ResidualGerm = canonical_chain(g).unwrap_or_else(|| {
        let min_r = g.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
        let min_c = g.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
        let mut c: ResidualGerm = g.iter().map(|(r, col, b, a)| (r - min_r, col - min_c, *b, *a)).collect();
        c.sort_unstable();
        c
    });

    let mut e = Element::identity();
    e.word = residual_word(&centered);
    e.address.take();
    e.form.take();
    e.mag = measure_lanes(&centered);
    e.sub = centered;
    e
}

/// A residual germ with its FRAME FIBER: the exact frame map
/// ρ = (h_out/h_in, w_out/w_in) ∈ ℚ² enters the spatial generator's
/// diagonal, so a word at ρ ≠ (1,1) is a different word from its planar
/// shadow (identity and substance both carry the fiber; only translation is
/// quotiented). `frame` is (h_in, w_in, h_out, w_out).
/// The unique listing key of a demand: the address of the framed residual.
/// Intake, the book, quote, attest, probe, settle, task, and the nerve
/// route all use this, never an unframed shadow.
pub fn listing_key(chain: &[(i64, i64, u8, u8)], frame: &(usize, usize, usize, usize)) -> crate::transducer::a21::Address {
    from_residual_framed(&chain.to_vec(), frame).address()
}

/// Identity frame: no scale. Unframed speech uses this so its key matches
/// a planar intake.
pub const IDENTITY_FRAME: (usize, usize, usize, usize) = (1, 1, 1, 1);

pub fn from_residual_framed(g: &ResidualGerm, frame: &(usize, usize, usize, usize)) -> Element {
    let mut e = from_residual(g);
    let (hi, wi, ho, wo) = *frame;
    if hi > 0 && wi > 0 && (hi, wi) != (ho, wo) {
        let rh = Q::new(BigInt::from(ho as u64), BigInt::from(hi as u64));
        let rw = Q::new(BigInt::from(wo as u64), BigInt::from(wi as u64));
        if rh == rw && !rh.is_one() {
            e.action.spatial = Some(crate::transducer::cga::dilator(&rh));
            e.address.take();
        }
    }
    e
}

/// MINT THE CONNECTION: the linear word, the A4 carriage, the gauge map,
/// and the remainder chain. `V = carriage · linear_versor(word)`. A word
/// that does not keep the plane is not a connection.
pub fn from_connection(
    linear: &crate::transducer::alphabet::Word,
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidualGerm,
    carried: [u64; 4],
) -> Option<Element> {
    let v = conformal_of(linear, carriage)?;
    let spelling = crate::transducer::alphabet::Spelling { word: linear.clone() };
    let mut e = from_residual(remainder);
    e.word = connection_word(&spelling, carriage, sigma, &e.sub);
    e.action.spatial = Some(v);
    e.action.gauge = sigma.to_vec();
    e.address.take();
    for l in 0..4 {
        e.mag[l] = e.mag[l].saturating_add(carried[l]);
    }
    Some(e)
}

/// The description of a connection: its spelling in the alphabet at its
/// dilation, the placement, the gauge map, and the centered remainder.
pub(crate) fn connection_word(
    spelling: &crate::transducer::alphabet::Spelling,
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidualGerm,
) -> Vec<u8> {
    let mut out = vec![CONNECTION_TAG];
    spelling.to_bytes(&mut out);
    carriage.to_bytes(&mut out);
    out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
    for &(f, t) in sigma {
        out.push(f);
        out.push(t);
    }
    let germ = residual_word(remainder);
    out.extend_from_slice(&(germ.len() as u32).to_le_bytes());
    out.extend_from_slice(&germ);
    out
}

/// A gauge between two sections, spelled as a connection with identity
/// gauge map and tagged so the court replays it as a translation.
pub fn from_translation(
    rotor: &crate::transducer::alphabet::Rotor,
    a: impl AsRef<[u8]>,
    b: impl AsRef<[u8]>,
) -> Option<Element> {
    let word = rotor.spelled_word()?;
    let mut e = from_connection(&word, &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])?;
    let mut d = e.description().to_vec();
    if d.first() == Some(&CONNECTION_TAG) {
        d[0] = TRANSLATION_TAG;
    }
    crate::transducer::a21::put_addr(&mut d, a)?;
    crate::transducer::a21::put_addr(&mut d, b)?;
    e.set_description(d);
    Some(e)
}
