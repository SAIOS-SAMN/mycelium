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

use super::perceive::sensor_plane_block;
use crate::transducer::element::Versor;

/// A plane block `[[s, −t], [t, s]]` as the Gaussian integer `s + t·i`,
/// with the common denominator cleared EXACTLY. A rotation block over ℚ
/// is `π/ρ` for a Gaussian `π`; clearing the denominator recovers `π` up
/// to a rational scale, and scale is what the ring divides out anyway —
/// identity is projective (1.5). `None` when the block is not a rotation
/// block, which is not an arc at all.
fn gauss_of_block(block: &[[crate::core::Q; 2]; 2]) -> Option<crate::core::gaussian::Gauss> {
    use num_integer::Integer;
    let (s, t) = (&block[0][0], &block[1][0]);
    if &block[1][1] != s || block[0][1] != -t.clone() {
        return None;
    }
    let d = s.denom().lcm(t.denom());
    let a = (s * num_rational::BigRational::from_integer(d.clone())).to_integer();
    let b = (t * num_rational::BigRational::from_integer(d)).to_integer();
    let g = crate::core::gaussian::Gauss::new(a, b);
    match g.is_zero() {
        true => None,
        false => Some(g),
    }
}

/// The inverse reading: `s + t·i` written back as its plane block.
fn block_of_gauss(g: &crate::core::gaussian::Gauss) -> [[crate::core::Q; 2]; 2] {
    crate::transducer::alphabet::arc_block(&g.a, &g.b)
}

/// The rotor a plane block is the action of: the block's arc is the
/// rotor's SQUARE (a rotor is a half-angle), so the rotor is the arc's
/// square root in ℤ[i]. Found by the ring, never searched for. `None`
/// when the arc has no square root there, which is a block no rotor over
/// ℚ induces.
///
/// THE SIGN, measured rather than assumed: `rotor_block_on` writes the
/// rotor `x + y·e_ij` as `[[x²−y², 2xy], [−2xy, x²−y²]]`, and this file
/// reads an arc as `s + t·i` with `t = block[1][0]`. So the arc of that
/// rotor is `(x²−y²) − 2xy·i = (x − y·i)²`: the square root's imaginary
/// part is `−y`, and the blade coefficient is its negation.
fn rotor_of_block(
    block: &[[crate::core::Q; 2]; 2],
    plane: (u8, u8),
) -> Option<crate::transducer::alphabet::Rotor> {
    use crate::transducer::alphabet::Rotor;
    let arc = gauss_of_block(block)?;
    let root = gauss_sqrt(&arc)?;
    let mask = (1usize << plane.0) | (1usize << plane.1);
    let q = |n: &num_bigint::BigInt| num_rational::BigRational::from_integer(n.clone());
    Some(Rotor::scalar(q(&root.a)).add(&Rotor::blade(mask, -q(&root.b))))
}

/// The square root of a Gaussian integer when it has one in ℤ[i], by the
/// exact identity `√(a+bi) = ±(x + y·i)` with `x² = (a + |π|)/2`. Every
/// step is an exact integer square root or a refusal; nothing is rounded.
fn gauss_sqrt(g: &crate::core::gaussian::Gauss) -> Option<crate::core::gaussian::Gauss> {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::Zero;
    let n = g.norm();
    let m = n.sqrt();
    if &m * &m != n {
        return None;
    }
    let two = BigInt::from(2);
    let (xx, r1) = (&g.a + &m).div_rem(&two);
    if !r1.is_zero() {
        return None;
    }
    let x = xx.sqrt();
    if &x * &x != xx {
        return None;
    }
    if x.is_zero() {
        return None;
    }
    let (y, r2) = g.b.div_rem(&(&two * &x));
    if !r2.is_zero() {
        return None;
    }
    Some(crate::core::gaussian::Gauss::new(x, y))
}

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
