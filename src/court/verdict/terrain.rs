// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 54.5, A20 — THE TERRAIN AND THE CERTIFICATES: the speaker's claims about
//! the complex, read against the geometry the court traversed.
//!
//! Split from `replay.rs` at the §1.1 1200-line reading bound when 78.153
//! metered them; the items are moved, and each gained its stroke.

use super::replay::chord_orbits;
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::Form;

/// 54.5 — THE LABOUR, CHECKED AGAINST THE TERRAIN.
///
/// A receipt is a SUB-SELECTION: it names which cycles this mind's stroke
/// discharged, and the complex says what each closes at. So the court does
/// not take `ℓ` from the claimant — it takes the class from the claimant
/// and the period from the geometry.
///
/// ```text
///     yield(F, r) = Σ_{κ ∈ r ∩ dom(orbit_classes F)} (ℓ_derived(κ) − 1)
/// ```
///
/// A class the form does not carry projects to ZERO and is not a refusal.
/// An observer may track cycles outside this form's horizon — that is
/// pruning, not fraud, and refusing it would make a mind's own memory a
/// liability.
///
/// What IS refused is a receipt that carries a period the complex does not
/// derive. `Σ(ℓ − 1)` prices reserve regeneration (E2), so an unchecked
/// period mints capacity from nothing; and unlike the class, the period is
/// a claim ABOUT THE TERRAIN, which the terrain can contradict. That is
/// the one lie in the envelope the court can catch, and catching it is
/// what makes the attestation worth carrying.
#[cfg(test)]
pub(crate) fn receipts_price_the_terrain(
    f: &Form,
    filed: &crate::network::nerve::Keyed<
        crate::transducer::sentence::FiledOrbit,
        crate::network::nerve::coord::Holonomy,
    >,
) -> Option<&'static str> {
    let mut rows = Vec::new();
    match terrain(f, filed, &mut Meter::Free, 0, &mut rows) {
        Err(Cut::Refused(why)) => Some(why),
        Err(Cut::Spent(_)) | Ok(()) => None,
    }
}

/// 78.153 — [`receipts_price_the_terrain`] on the stroke: one read per orbit
/// whose classes are derived (the rows kept on the cursor, in orbit order,
/// so a resumed reading derives each orbit once) and one per claim read.
/// `from` counts the orbits first, then the claims.
pub(crate) fn receipts_price_the_terrain_from(
    f: &Form,
    spoken: &Element,
    meter: &mut Meter<'_>,
    from: usize,
    rows: &mut Vec<(crate::network::nerve::coord::Holonomy, crate::transducer::sentence::ToralPeriod)>,
) -> Step {
    terrain(f, spoken.filing(), meter, from, rows).map_err(|cut| match cut {
        Cut::Refused(why) => Halt::Refused(Refusal::of_tape(why, spoken)),
        Cut::Spent(at) => Halt::Spent(at),
    })
}

fn terrain(
    f: &Form,
    filed: &crate::network::nerve::Keyed<
        crate::transducer::sentence::FiledOrbit,
        crate::network::nerve::coord::Holonomy,
    >,
    meter: &mut Meter<'_>,
    from: usize,
    rows: &mut Vec<(crate::network::nerve::coord::Holonomy, crate::transducer::sentence::ToralPeriod)>,
) -> Result<(), Cut> {
    if filed.is_empty() {
        return Ok(());
    }
    // THE PERIODS THE COMPLEX DERIVES, by class, over every orbit. One
    // reader (`orbit_classes`), the same one `file_orbit` writes through and
    // `edge_coheres` asks.
    let n = f.orbits.len();
    for (i, lp) in f.orbits.iter().enumerate().skip(from) {
        if !meter.spend() {
            return Err(Cut::Spent(i));
        }
        for row in f.orbit_classes(lp) {
            rows.push((row.class, row.periods));
        }
    }
    // The rows filed in orbit order: a class two orbits derive holds the
    // later one's periods, as the keyed write over the orbits did.
    let mut derived: Keyed<
        crate::transducer::sentence::ToralPeriod,
        crate::network::nerve::coord::Holonomy,
    > = Keyed::new();
    for (class, periods) in rows.iter() {
        derived.put(class, *periods);
    }
    let claims_from = match from.checked_sub(n) {
        Some(c) => c,
        None => 0,
    };
    for (j, (class, claim)) in filed.entries().enumerate().skip(claims_from) {
        if !meter.spend() {
            return Err(Cut::Spent(n + j));
        }
        match derived.at(&class) {
            // The period is a claim about the terrain and the terrain
            // disagrees.
            // BOTH PLANES. A claim that agrees on the `lcm` and differs on
            // the pair is a different orbit, and the terrain is what the
            // complex derives — so the comparison is the spectrum (54.7).
            Some(&p) if claim.periods != p => return Err(Cut::Refused(REFUSE_INFLATED_YIELD)),
            // Claimed, and the complex agrees.
            Some(_) => {}
            // A class this form does not carry. It earns nothing here and
            // says nothing against the speaker (rule 11).
            None => {}
        }
    }
    Ok(())
}

/// A20: each certificate is the un-pivot of one chord face, re-read.
/// The orbit's class key and rank before and after must match the tape.
/// The orbit is reached by its chord's coordinates on each side (AT1), never
/// by its position in either form's list.
pub(crate) fn replay_certificates(
    f: &Form,
    certificates: &[crate::transducer::sentence::PivotCertificate],
    by_chord: &Keyed<crate::transducer::sentence::Orbit, crate::network::nerve::coord::Overlap>,
) -> Option<&'static str> {
    match replay_certificates_from(f, certificates, by_chord, &mut Meter::Free, 0) {
        Err(Cut::Refused(why)) => Some(why),
        Err(Cut::Spent(_)) | Ok(()) => None,
    }
}

/// 78.153 — [`replay_certificates`] on a meter: one read per certificate
/// (each one a whole-form clone, a forest, two class vectors and two
/// ranks), from the `from`-th of the envelope's order.
pub(crate) fn replay_certificates_from(
    f: &Form,
    certificates: &[crate::transducer::sentence::PivotCertificate],
    by_chord: &Keyed<crate::transducer::sentence::Orbit, crate::network::nerve::coord::Overlap>,
    meter: &mut Meter<'_>,
    from: usize,
) -> Result<(), Cut> {
    // THE CERTIFICATES ARE THE SPEAKER'S (54.5 Road (d), B3). They hung off
    // the form, and a certificate is not geometry: it is what ONE mind's
    // stroke did to this complex against ONE budget. Two minds holding the
    // same form pivot differently, so the map does not factor through the
    // form and `from_address` cannot rebuild it — the same argument that
    // moved the filing at `df1eeaa`, one carrier later.
    //
    // Every certificate the envelope carries is re-read or the word is
    // refused; there is no subset of them that would do.
    for (i, cert) in certificates.iter().enumerate().skip(from) {
        if !meter.spend() {
            return Err(Cut::Spent(i));
        }
        if let Some(why) = certificate_replays(f, cert, by_chord) {
            return Err(Cut::Refused(why));
        }
    }
    Ok(())
}

/// One certificate of [`replay_certificates_from`].
fn certificate_replays(
    f: &Form,
    cert: &crate::transducer::sentence::PivotCertificate,
    by_chord: &Keyed<crate::transducer::sentence::Orbit, crate::network::nerve::coord::Overlap>,
) -> Option<&'static str> {
    let face = cert.by.face();
    let Some(after) = by_chord.at(face) else {
        return Some(REFUSE_PIVOT_INEXACT);
    };
    let mut pre = f.clone();
    // A move with no inverse cannot be un-pivoted: the certificate is
    // not re-read, and it is refused rather than read against a form the
    // skip left unrestored (VS1's ruling, item 6).
    if !pre.unapply_move(&cert.by) {
        return Some(REFUSE_PIVOT_INEXACT);
    }
    pre.orbits = Form::orbits_of(&pre.faces, &pre.vertices);
    let before = chord_orbits(&pre);
    let Some(lp_before) = before.at(face) else {
        return Some(REFUSE_PIVOT_INEXACT);
    };
    // An opened axis (I6) is seeded by the orbit's own holonomy versor
    // (16.20): the court re-derives the seed from the orbit it re-read
    // and refuses a certificate whose seed is anything else (law 12).
    if let crate::transducer::sentence::PivotBy::OpenAxis { seed, .. } = &cert.by {
        let rederived = pre.axis_seed_of(lp_before);
        if !rederived.is_some_and(|v| element::same_versor(&v, seed)) {
            return Some(REFUSE_PIVOT_INEXACT);
        }
    }
    let Some(kappa_before) = pre.class_vector_of(lp_before) else {
        return Some(REFUSE_PIVOT_INEXACT);
    };
    if kappa_before.class_key() != cert.before {
        return Some(REFUSE_PIVOT_INEXACT);
    }
    let Some(kappa_after) = f.class_vector_of(after) else {
        return Some(REFUSE_PIVOT_INEXACT);
    };
    if kappa_after.class_key() != cert.after || kappa_after.occupies(cert.axis) {
        return Some(REFUSE_PIVOT_INEXACT);
    }
    let Some(rb) = pre.rank() else {
        return Some(REFUSE_PIVOT_INEXACT);
    };
    let Some(ra) = f.rank() else {
        return Some(REFUSE_PIVOT_INEXACT);
    };
    // S9.1 — THE COURT RE-DERIVES THE MOVE. The certificate claims what
    // the pivot did to the rank; the court traverses the form before and
    // after for itself and computes the same move. The two absolute
    // audits it used to compare against are the court's own here, so
    // carrying them on the tape only invited a second comparison.
    if crate::transducer::sentence::Rank::moved(rb, ra) != cert.rank_move {
        return Some(REFUSE_PIVOT_INEXACT);
    }
    None
}
