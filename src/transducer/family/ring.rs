// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 37.7, L11 (3/3) — THE RING BRANCH: a derivation through a generator
//! the alphabet does not spell.
//!
//! A12 reduces over LETTER exponents, so it cannot see a prime outside
//! the seventeen however many of them a mind holds. The ring can: the
//! sensor block of a versor is a Gaussian integer, an adjoined
//! generator's block is one too, and division in ℤ[i] is exact.
//!
//! Split from `perceive.rs` at the §1.1 600-line reading bound; the
//! items are moved, not rewritten (§1.6).

use crate::transducer::alphabet::{block_of_gauss, gauss_of_block, rotor_of_block};
use crate::transducer::element::{sensor_plane_block, Versor};

// `gauss_of_block`, `block_of_gauss`, `rotor_of_block` and `gauss_sqrt`
// were here. They are arithmetic on a rotor's block, not logic of the ring
// branch, and being private to this file put them out of reach of the two
// other readers that need the same arithmetic: the versor readers in
// `element`, which classify a meet's transition by its rotor part, and the
// court's complex, which re-derives that classification. They are moved to
// `alphabet::block`, unchanged (§1.6).

/// 37.7, L11 (3/3) — A DERIVATION THROUGH AN ADJOINED GENERATOR.
///
/// The target's block is cleared of its common denominator exactly and
/// read as `s + t·i`. Each rotor-only generator the mind holds on this
/// chart is divided out to its FULL multiplicity, in the order the index
/// returns them — an order by content, never a preference (rule 4). What
/// is left is the cofactor: peeled by A22 for the residue in letters, or,
/// where the peel cannot spell it either, no derivation at all and the
/// demand the peel typed stands.
///
/// `None` means no held generator divides the target, or the composition
/// did not replay.
pub(crate) fn ring_derivation(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    demand: &crate::transducer::sentence::Demand,
) -> Option<crate::transducer::alphabet::Derivation> {
    use crate::transducer::alphabet::{Derivation, Rotor, PLANE_COL, PLANE_ROW};
    let plane = (PLANE_ROW, PLANE_COL);
    // The projection is asked ABOUT THE ROTATION — which Gaussian integer
    // the rotation is, so a prime may divide it — which is what the
    // sensor-plane block answers exactly (A3).
    let block = sensor_plane_block(v)?;
    let target = gauss_of_block(&block)?;
    // The chart the presentation is read at is the target's own, as it is
    // for the letter branch (42.6).
    let target_rotor = rotor_of_block(&block, plane)?;
    let chart = crate::transducer::sentence::versor_chart_of(&target_rotor)?;
    let held = crystal.generators_under(&chart, budget);

    let mut rest = target;
    let mut cited: Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        i64,
        Rotor,
    )> = Vec::new();
    for (form, vertex, g) in &held {
        // Letter-bearing generators are A12's, not the ring's.
        if !g.letters().is_empty() {
            continue;
        }
        let rotor = g.rotor();
        let Some(gb) = crate::transducer::alphabet::rotor_block_on(&rotor, plane) else {
            continue;
        };
        let Some(divisor) = gauss_of_block(&gb) else {
            continue;
        };
        if divisor.is_unit() || divisor.is_zero() {
            continue;
        }
        // FULL MULTIPLICITY, bounded by the norm's own valuation: each
        // division divides the norm by `N(divisor) ≥ 2`, so the count
        // cannot exceed the norm's bit length and the loop's bound is the
        // target's own size (rule 10).
        let mut e = 0i64;
        let bound = rest.norm().bits() as i64;
        while e < bound {
            let Some(r) = rest.rem(&divisor) else { break };
            if !r.is_zero() {
                break;
            }
            let Some(quotient) = rest.div_round(&divisor) else { break };
            rest = quotient;
            e += 1;
        }
        if e > 0 {
            cited.push((form.clone(), vertex.clone(), e, rotor));
        }
    }
    if cited.is_empty() {
        return None;
    }

    // The cofactor in letters. A cofactor that is a unit is the identity:
    // the generators reached the whole target.
    let residue = match rest.is_unit() {
        true => crate::transducer::alphabet::Word::identity(),
        false => {
            let cb = block_of_gauss(&rest);
            match crate::transducer::alphabet::spell_peel_on(plane, &cb) {
                Some((crate::transducer::peel::PeelNest::Closed { cores }, _)) => {
                    let mut w = crate::transducer::alphabet::Word::identity();
                    for syllable in cores.iter().rev() {
                        for &(id, inv) in syllable {
                            w.push(id, inv);
                        }
                    }
                    w
                }
                // The peel cannot spell what the ring left either: the
                // demand the peel typed stands, and it names the block it
                // stood on.
                _ => return None,
            }
        }
    };

    let mut composed = Rotor::identity();
    for (_, _, e, rotor) in &cited {
        composed = composed.geo(&crate::transducer::alphabet::rotor_power(rotor, *e)?);
    }
    composed = composed.geo(&residue.rotor());
    // REPLAY BEFORE RETURNING. The ring's arithmetic is exact, but the
    // map from a block to a rotor is a half-angle and the composition
    // must be checked against the target it claims to reach; a mismatch
    // is not a guess to ship (rule 11).
    if !composed.projective_eq(&target_rotor) {
        eprintln!(
            "  derivation refused: the ring's citations do not compose to the target ({})",
            demand.label()
        );
        return None;
    }
    let generators = cited.iter().map(|(f, x, e, _)| (f.clone(), x.clone(), *e)).collect();
    Some(Derivation::factored(generators, residue, composed))
}
