// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE Δ MOUNT — a blade ensemble becomes the antisymmetric relational field.
//!
//! This is where the Cl(3,1) transducer meets the topological core. The core's
//! Δ is an n×n ANTISYMMETRIC matrix of exact-ℚ relations (`Δ_ij = −Δ_ji`); a
//! scene's bodies are its states, and the relation between two bodies is their
//! exact geometric offset. The base fiber (m = 2) carries the spatial
//! displacement `c_j − c_i` — antisymmetric by construction. Richer relations
//! (relative rotation class, scale ratio, gauge residual) are ADDED fibers via
//! `Delta::expand_fiber` as dimensional resolution demands — the manifold folds
//! to exactly the dimension the data needs, never a flat bucket.

use crate::core::engine::{Delta, Q, QVec};
use crate::transducer::blade::Blade;
use num_traits::Zero;

/// The base-fiber relation carrying body `a` to body `b`: the exact ℚ² centroid
/// displacement `c_b − c_a`. Antisymmetric — swapping the pair negates it — so it
/// is a lawful entry of the antisymmetric relational field. `None` if either body
/// has no readable centroid (a degenerate/null sphere, which the lift forbids).
pub fn relation(a: &Blade, b: &Blade) -> Option<QVec> {
    let (ax, ay) = a.centroid()?;
    let (bx, by) = b.centroid()?;
    Some(vec![bx - ax, by - ay])
}

/// Build the antisymmetric relational field Δ over a blade ensemble: `dim = n`
/// bodies, base fiber `m = 2` (spatial displacement). `Δ_ij = c_j − c_i`,
/// `Δ_ji = −Δ_ij` (enforced by `set_antisym`), `Δ_ii = 0`. This is the scene's
/// intrinsic relational geometry — position-free: a global translation of the
/// whole scene leaves every pairwise displacement, hence Δ, unchanged.
pub fn field(blades: &[Blade]) -> Delta {
    let n = blades.len();
    let mut d = Delta::zero(n, 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let v = relation(&blades[i], &blades[j]).unwrap_or_else(|| vec![Q::zero(); 2]);
            d.set_antisym(i, j, v);
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::blade::transduce;
    use num_bigint::BigInt;
    use num_rational::BigRational;

    fn q(v: i64) -> Q {
        BigRational::from_integer(BigInt::from(v))
    }

    // Two separated unit bodies at (0,0) and (0,3): one horizontal pair.
    fn scene(shift_r: i64, shift_c: i64) -> Vec<Blade> {
        transduce(&[(shift_r, shift_c, 1), (shift_r, shift_c + 3, 2)], 0)
    }

    /// The field is antisymmetric with a zero diagonal — a lawful Δ.
    #[test]
    fn the_field_is_antisymmetric() {
        let d = field(&scene(0, 0));
        assert_eq!(d.dim, 2);
        assert_eq!(d.m, 2);
        // Δ_01 = c_1 − c_0 = (0, 3); Δ_10 = −(0,3); Δ_ii = 0.
        assert_eq!(d.get(0, 1), &vec![q(0), q(3)]);
        assert_eq!(d.get(1, 0), &vec![q(0), q(-3)]);
        assert_eq!(d.get(0, 0), &vec![q(0), q(0)]);
    }

    /// TOPOLOGICAL SOUNDNESS: the relational field is position-free — translating
    /// the whole scene leaves Δ identical. Recognition can therefore be a
    /// relation on Δ, never a lookup keyed on absolute coordinates.
    #[test]
    fn the_field_is_translation_invariant() {
        let here = field(&scene(0, 0));
        let there = field(&scene(5, 9));
        assert_eq!(here.entries, there.entries, "Δ is intrinsic, not positional");
    }
}
