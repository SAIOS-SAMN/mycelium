// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ANONYMOUS ANASTOME ELEMENT — the exact transform carrying one body to
//! another, derived from the geometry in one solve (no per-operation branch, no
//! English "kind"). It is an element of the extended conformal group ⊕ the gauge
//! fiber: the spatial part on σ_𝒮, the fiber permutation on σ_𝒢. Cognition
//! treats it OPAQUELY — addressed by its canonical form, never branched on. Words
//! like "rotation"/"recolour" exist ONLY in the `ccl` display seam, downstream,
//! one-way (English reads ANASTOME; it never defines it).

use crate::transducer::blade::lift;
use crate::transducer::motor::{solve_conformal_motor, ConformalMotor};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}

/// A body transform: the spatial similarity plus the gauge relabel (sorted
/// non-identity `(from_hue, to_hue)` pairs; empty ⇒ no recolour).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform {
    pub motor: ConformalMotor,
    pub relabel: Vec<(u8, u8)>,
}

/// Solve the full transform for one body `a → b` (raw cells). `None` when the
/// shape is not a conformal image (a pivot) or the recolour is inconsistent (a
/// hue mapping to two targets — not a function, so not a relabel).
pub fn solve_transform(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> Option<Transform> {
    let motor = solve_conformal_motor(&lift(a, 0), &lift(b, 1))?;
    let relabel = solve_relabel(a, b, &motor)?;
    Some(Transform { motor, relabel })
}

/// Read the gauge relabel off the corresponded cells: map each A-cell through the
/// motor to its B-position, take B's hue there, and require a consistent hue
/// function `h → h'`. `None` if a position has no B-cell (should not happen once
/// the motor closed the cells) or the map is inconsistent.
fn solve_relabel(
    a: &[(i64, i64, u8)],
    b: &[(i64, i64, u8)],
    m: &ConformalMotor,
) -> Option<Vec<(u8, u8)>> {
    let mut pairs: Vec<(u8, u8)> = Vec::new();
    for (r, c, h) in a {
        let (pr, pc) = m.apply_point(&q(*r), &q(*c));
        if !pr.is_integer() || !pc.is_integer() {
            return None;
        }
        let (tr, tc) = (pr.to_integer().to_i64()?, pc.to_integer().to_i64()?);
        let bh = b.iter().find(|(br, bc, _)| *br == tr && *bc == tc).map(|(_, _, x)| *x)?;
        match pairs.iter().find(|(x, _)| *x == *h) {
            Some((_, y)) if *y == bh => {}
            Some(_) => return None, // one hue → two targets: not a relabel
            None => pairs.push((*h, bh)),
        }
    }
    pairs.retain(|(x, y)| x != y); // drop identity mappings
    pairs.sort_unstable();
    Some(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pure recolour: shape unchanged, hues 1→2. Identity motor, relabel {(1,2)}.
    #[test]
    fn a_pure_recolour_is_a_gauge_relabel() {
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)];
        let b = [(0, 0, 2u8), (0, 1, 2), (1, 0, 2)];
        let t = solve_transform(&a, &b).expect("recolour solves");
        assert!(t.motor.is_identity() && t.relabel == vec![(1, 2)]);
    }

    /// Motion ∘ recolour: translate +0,+3 AND recolour 1→5, one transform.
    #[test]
    fn motion_and_recolour_solve_together() {
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)];
        let b = [(0, 3, 5u8), (0, 4, 5), (1, 3, 5)];
        let t = solve_transform(&a, &b).expect("motion∘recolour solves");
        assert_eq!(t.motor.translation, (q(0), q(3)));
        assert_eq!(t.relabel, vec![(1, 5)]);
    }

    /// Pure motion: no recolour, empty relabel.
    #[test]
    fn a_pure_motion_has_no_relabel() {
        let a = [(0, 0, 7u8), (0, 1, 7), (1, 0, 7)];
        let b = [(0, 3, 7u8), (0, 4, 7), (1, 3, 7)];
        let t = solve_transform(&a, &b).expect("motion solves");
        assert!(t.relabel.is_empty());
    }

    /// A shape change is still a pivot (no motor).
    #[test]
    fn a_shape_change_is_a_pivot() {
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)];
        let b = [(0, 0, 1u8), (0, 1, 1), (0, 2, 1)];
        assert!(solve_transform(&a, &b).is_none());
    }
}
