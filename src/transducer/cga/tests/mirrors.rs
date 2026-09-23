// 78.14, MG1 — the base's mirrors as constructors, and a nucleus read against
// the mirrors that bracket its centre.

use super::common::{budget, q};
use crate::transducer::cech::{read_coxeter, Closure, ComponentClass, Refusal};
use crate::transducer::cga::*;

/// A NUCLEUS SITS IN THE GRID BY ITS δ² AGAINST ITS BRACKETING MIRRORS: the
/// sphere of centre `(1/2, 0, 3/2)`, `ρ² = 1/4`, touches both mirrors of its
/// first and third axes (δ² = 1, tangent) and stands centred on `x₂ = 0`
/// (δ² = 0, the plane through its centre). With `ρ² = 1` the same centre's
/// mirrors cut it at `δ² = 1/4`.
#[test]
fn a_nucleus_reads_its_delta_squared_against_the_mirrors_it_stands_between() {
    let centre = [q(1, 2), q(0, 1), q(3, 2)];
    let read = |r2| {
        mirrors_between(&sphere_at(&centre, &r2), 3)
            .into_iter()
            .map(|m| (m.axis, m.k, m.delta2, m.regime))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        read(q(1, 4)),
        vec![
            (1, q(0, 1), q(1, 1), PairRegime::Parabolic),
            (1, q(1, 1), q(1, 1), PairRegime::Parabolic),
            (2, q(0, 1), q(0, 1), PairRegime::Finite(2)),
            (3, q(1, 1), q(1, 1), PairRegime::Parabolic),
            (3, q(2, 1), q(1, 1), PairRegime::Parabolic),
        ]
    );
    let wide = read(q(1, 1));
    assert!(
        wide.iter().filter(|m| m.0 != 2).all(|m| m.2 == q(1, 4) && m.3 == PairRegime::Finite(3)),
        "the mirrors cut the wider sphere at δ² = 1/4: {wide:?}"
    );
}

/// THE MIRRORS THAT BRACKET IT ARE THE GRID'S OWN GROUP: two parallel
/// mirrors on an axis read `Ã₁`, affine; a lone mirror reads finite.
#[test]
fn the_bracketing_mirrors_read_the_grids_affine_group() {
    let planes: Vec<Mv> =
        [(1, 0), (1, 1), (2, 0), (3, 1), (3, 2)].iter().map(|&(a, k)| base_mirror(a, &q(k, 1))).collect();
    let r = read_coxeter(&planes, &[], &mut budget());
    assert_eq!(
        r.components,
        vec![
            (vec![0, 1], ComponentClass::Affine),
            (vec![2], ComponentClass::Finite),
            (vec![3, 4], ComponentClass::Affine)
        ]
    );
    assert!(matches!(r.closure, Closure::Infinite(Refusal::Affine(_))));
}
