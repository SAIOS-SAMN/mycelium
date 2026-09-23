// 75, 78.11, MD1 — a generator's modes at every exponent: commuting 2-blades,
// `m` read off the bivector, each with its plane and class; undecomposed by
// name; and the inverse Cayley chart.

use super::common::{budget, q};
use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::*;

fn kinds(b: &Mv) -> Vec<(BivectorKind, usize)> {
    match modes(b, &mut budget()) {
        Ok(ms) => ms.iter().map(|m| (m.kind, m.planes)).collect(),
        Err(e) => panic!("undecomposed: {e:?}"),
    }
}

/// A SCREW READS TWO MODES: the turn in `e₁₂` elliptic, the slip along `e₃`
/// parabolic — the pair the two-part closed form refused, since its open
/// half is null.
#[test]
fn a_screw_reads_an_elliptic_and_a_parabolic_mode() {
    let screw = rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    let ms = modes(&screw, &mut budget()).expect("the screw splits over ℚ");
    assert_eq!(
        ms.iter().map(|m| (m.kind, m.planes)).collect::<Vec<_>>(),
        vec![(BivectorKind::Parabolic, 1), (BivectorKind::Elliptic, 1)]
    );
    let sum = ms.iter().fold(Mv::zero(), |acc, m| acc.add(&m.blade));
    assert!(sum.sub(&screw.grade(2)).is_zero(), "the modes re-sum to the generator");
    assert!(ms[0].blade.geo(&ms[1].blade).sub(&ms[1].blade.geo(&ms[0].blade)).is_zero(), "and commute");
}

/// EACH MODE CLOSES ON ITS OWN PLANE: a half-turn screw's turn closes at two
/// — read on the motion's action on `e₁₂` — while its slip along `e₃` is
/// open by name; the joint motion never closes, and that is the parabolic
/// planet's reading, not the turn's.
///
/// AND THE CHART'S REACH, MEASURED: a QUARTER-turn screw has no rational
/// point in the Cayley chart. Its rotor `1 + e₁₂` has weight 2, and the
/// generator would need `tan(π/8) = √2 − 1`; the inverse chart refuses it by
/// name (MD1's finding, for 78.11 link 2).
#[test]
fn a_screws_turn_closes_at_its_own_order_and_its_slip_is_open() {
    let half = Mv::blade(E1 | E2, q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    let generator = cayley_inverse(&half).expect("a half-turn screw has a rational chart point");
    let ms = modes(&generator, &mut budget()).expect("its modes split");
    let orders: Vec<(BivectorKind, Option<u32>)> = ms.iter().map(|m| (m.kind, mode_order(&half, m))).collect();
    assert_eq!(orders, vec![(BivectorKind::Parabolic, None), (BivectorKind::Elliptic, Some(2))]);
    assert_eq!(finite_order(&half), None, "and the whole motion's orbit never closes");

    let quarter = rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    assert!(cayley_inverse(&quarter).is_none(), "a quarter turn has no rational chart point: tan(π/8) ∉ ℚ");
}

/// A DISPIRATION READS TWO: a turn and a boost on orthogonal planes.
#[test]
fn a_dispiration_reads_an_elliptic_and_a_hyperbolic_mode() {
    let b = Mv::blade(E1 | E2, q(1, 1)).add(&Mv::blade(E3 | EM, q(1, 2)));
    assert_eq!(kinds(&b), vec![(BivectorKind::Elliptic, 1), (BivectorKind::Hyperbolic, 1)]);
}

/// AT FOUR AXES, NO ARITY WRITTEN IN: two unequal turns on `e₁₂` and `e₃₄`
/// read two elliptic modes; equal turns read ONE mode over two planes — an
/// isoclinic pair, held whole, no split among its planes chosen.
#[test]
fn at_four_axes_two_turns_read_two_modes_and_an_isoclinic_pair_reads_one_over_two_planes() {
    let e34 = base_axis(3) | base_axis(4);
    let unequal = Mv::blade(E1 | E2, q(1, 1)).add(&Mv::blade(e34, q(2, 1)));
    assert_eq!(kinds(&unequal), vec![(BivectorKind::Elliptic, 1), (BivectorKind::Elliptic, 1)]);
    let isoclinic = Mv::blade(E1 | E2, q(1, 1)).add(&Mv::blade(e34, q(1, 1)));
    assert_eq!(kinds(&isoclinic), vec![(BivectorKind::Elliptic, 2)], "one value, two planes, held whole");
}

/// UNDECOMPOSED BY NAME: `e₁₂ + e₂₃ + e₃₄` has mode values `(−3 ± √5)/2`,
/// which no rational split reaches — refused with the factor's degree, never
/// rounded onto a nearby split.
#[test]
fn a_generator_whose_modes_lie_in_an_extension_is_undecomposed_by_name() {
    let b = Mv::blade(E1 | E2, q(1, 1))
        .add(&Mv::blade(E2 | E3, q(1, 1)))
        .add(&Mv::blade(base_axis(3) | base_axis(4), q(1, 1)));
    assert_eq!(modes(&b, &mut budget()), Err(Undecomposed::Irrational(2)));
}

/// THE INVERSE CHART (78.11 link 2): a generator taken to its motion by
/// Cayley comes back as a generator of the same ray, and exactly where the
/// ray's canonical writing is the one Cayley produced.
#[test]
fn the_inverse_cayley_chart_returns_the_generator_of_the_ray() {
    let b = Mv::blade(E1 | E2, q(1, 3)).add(&Mv::blade(E3 | EM, q(1, 5)));
    let omega = cayley(&b).expect("off the pole");
    let back = cayley_inverse(&omega).expect("the ray has a rational chart point");
    assert!(back.sub(&b).is_zero(), "the same generator, exactly: {back:?}");
    let again = cayley(&back).expect("and it evolves");
    assert!(again.projective_eq(&omega), "to the same motion");
    // A scalar multiple of the motion is the same ray and the same chart —
    // compared exactly, by the difference, since `Mv`'s projective `==`
    // reads `b̃b` and does not hold for a non-simple bivector.
    let scaled = omega.scale(&q(-7, 2));
    let chart = cayley_inverse(&scaled).expect("the scaled ray has the same chart point");
    assert!(chart.sub(&b).is_zero(), "the chart is the ray's, not the writing's");
    // A multivector that is not a versor has no chart point.
    assert!(cayley_inverse(&Mv::blade(E1, q(1, 1))).is_none(), "a vector is no motion of this chart");
}
