// 78.14, MG1 — the base's mirrors as constructors, and a nucleus read against
// the mirrors that bracket its centre.

use super::fixtures::{budget, q};
use crate::transducer::cech::{read_coxeter, GroupClosure, ComponentClass, Refusal};
use crate::transducer::cga::*;

/// A NUCLEUS SITS IN THE GRID BY ITS OFFSET FROM ITS BRACKETING MIRRORS, at
/// unit weight and first degree (78.59 — never a squared ratio): the
/// membrane about `(1/2, 0, 3/2)` at the gauge `ρ = 1/2` stands `±1/2` from
/// both mirrors of its first and third axes and touches them (parabolic),
/// and stands on `x₂ = 0` (offset 0, the plane through its centre:
/// orthogonal, order two). At `ρ = 1` the same mirrors cut it at 60°, a
/// turn of order three.
#[test]
fn mirrors_between_reads_nucleus_offset_and_regime() {
    let centre = [q(1, 2), q(0, 1), q(3, 2)];
    let read = |rho| {
        mirrors_between(&membrane_at(&centre, &rho).expect("a positive gauge"), 3)
            .into_iter()
            .map(|m| (m.axis, m.k, m.offset, m.regime))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        read(q(1, 2)),
        vec![
            (1, q(0, 1), q(1, 2), PairRegime::Parabolic),
            (1, q(1, 1), q(-1, 2), PairRegime::Parabolic),
            (2, q(0, 1), q(0, 1), PairRegime::Finite(2)),
            (3, q(1, 1), q(1, 2), PairRegime::Parabolic),
            (3, q(2, 1), q(-1, 2), PairRegime::Parabolic),
        ]
    );
    let wide = read(q(1, 1));
    assert!(
        wide.iter().filter(|m| m.0 != 2).all(|m| m.3 == PairRegime::Finite(3)),
        "the mirrors cut the wider sphere at 60°: {wide:?}"
    );
}

/// THE MIRRORS THAT BRACKET IT ARE THE GRID'S OWN GROUP: two parallel
/// mirrors on an axis read `Ã₁`, affine; a lone mirror reads finite.
#[test]
fn base_mirror_brackets_reads_affine_group() {
    let planes: Vec<Mv> =
        [(1, 0), (1, 1), (2, 0), (3, 1), (3, 2)].iter().map(|&(a, k)| base_mirror(a, &q(k, 1)).expect("an axis of the base")).collect();
    let r = read_coxeter(&planes, &[], &mut budget());
    assert_eq!(
        r.components,
        vec![
            (vec![0, 1], ComponentClass::Affine),
            (vec![2], ComponentClass::Finite),
            (vec![3, 4], ComponentClass::Affine)
        ]
    );
    assert!(matches!(r.closure, GroupClosure::Infinite(Refusal::Affine(_))));
}
