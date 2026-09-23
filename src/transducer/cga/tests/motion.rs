// 78.12, MR1, EQ1, MD1b — a motion's modes read off its own action: rational
// `c`, exact rays that commute and multiply back to the motion, the half turn
// the plane's blade; rays equal by proportion; rational roots at any degree.

use super::common::{budget, q};
use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::*;

fn read(m: &Mv) -> Vec<MotionMode> {
    match motion_modes(m, &mut budget()) {
        Some(Ok(ms)) => ms,
        other => panic!("the motion's modes read: {other:?}"),
    }
}

fn product(ms: &[MotionMode]) -> Mv {
    ms.iter().fold(Mv::scalar(q(1, 1)), |acc, m| acc.geo(&m.ray))
}

/// THE QUARTER-TURN SCREW, which the chart could not reach: its turn reads
/// elliptic, `c = 0`, closing at four; its slip parabolic and open; the two
/// rays commute and multiply back to the screw.
#[test]
fn a_quarter_turn_screw_reads_its_turn_closing_at_four_and_its_slip_open() {
    let screw = rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    assert!(cayley_inverse(&screw).is_none(), "the chart has no point here: tan(π/8) ∉ ℚ");
    let ms = read(&screw);
    assert_eq!(
        ms.iter().map(|m| (m.kind, m.planes, m.order)).collect::<Vec<_>>(),
        vec![(BivectorKind::Parabolic, 1, None), (BivectorKind::Elliptic, 1, Some(4))]
    );
    assert_eq!(ms[1].c, q(0, 1), "a quarter turn: cos θ = 0, rational");
    assert!(ms[0].ray.geo(&ms[1].ray).sub(&ms[1].ray.geo(&ms[0].ray)).is_zero(), "the rays commute");
    assert_eq!(product(&ms), screw, "and multiply back to the screw, as a ray");
}

/// THE HALF TURN IS THE PLANE'S BLADE: `c = −1`, the ray degenerate, the
/// mode's versor `e₁₂` itself, and `−e₁₂` the same rotation — the held `ℤ₂`
/// by construction.
#[test]
fn a_half_turn_reads_as_its_planes_blade() {
    let half = Mv::blade(E1 | E2, q(1, 1));
    let [m] = read(&half).try_into().expect("one mode");
    assert_eq!((m.kind, m.planes, m.c.clone(), m.order), (BivectorKind::Elliptic, 1, q(-1, 1), Some(2)));
    assert_eq!(m.ray, half, "the ray is the plane's blade");
    assert_eq!(m.ray, half.scale(&q(-1, 1)), "and its negative is the same ray: the held ℤ₂");
}

/// THE (3,4,5) TURN READS `c = −7/25` — `cos θ` with `cos(θ/2) = 3/5` — a
/// rosette that closes at no order; a boost reads hyperbolic at `cosh θ`.
#[test]
fn a_pythagorean_turn_is_a_rosette_and_a_boost_reads_its_cosh() {
    let turn = rotator(&q(3, 1), &q(4, 1));
    let [m] = read(&turn).try_into().expect("one mode");
    assert_eq!((m.kind, m.c.clone(), m.order), (BivectorKind::Elliptic, q(-7, 25), None));
    assert_eq!(m.ray, turn);
    let boost = Mv::scalar(q(5, 1)).add(&Mv::blade(E1 | EM, q(4, 1)));
    let [b] = read(&boost).try_into().expect("one mode");
    assert_eq!((b.kind, b.c.clone(), b.order), (BivectorKind::Hyperbolic, q(41, 9), None));
    assert_eq!(b.ray, boost);
}

/// A DISPIRATION: a quarter turn and a boost on orthogonal planes, two
/// modes whose rays multiply back to the motion.
#[test]
fn a_dispiration_reads_a_turn_and_a_boost_that_multiply_back() {
    let turn = rotator(&q(1, 1), &q(1, 1));
    let boost = Mv::scalar(q(5, 1)).add(&Mv::blade(E3 | EM, q(4, 1)));
    let motion = turn.geo(&boost);
    let ms = read(&motion);
    let kinds: Vec<BivectorKind> = ms.iter().map(|m| m.kind).collect();
    assert_eq!(kinds, vec![BivectorKind::Elliptic, BivectorKind::Hyperbolic], "by c: 0 before 41/9");
    assert_eq!(product(&ms), motion);
}

/// AN ISOCLINIC PAIR IS HELD WHOLE: equal quarter turns on `e₁₂` and `e₃₄`
/// read one mode over two planes, its ray carrying the grade-four term the
/// planes' product has, no split among them chosen.
#[test]
fn an_isoclinic_pair_is_one_mode_over_two_planes() {
    let e34 = base_axis(3) | base_axis(4);
    let motion = rotator(&q(1, 1), &q(1, 1)).geo(&rotator_on_blade(&q(1, 1), &q(1, 1), e34));
    let [m] = read(&motion).try_into().expect("one mode");
    assert_eq!((m.kind, m.planes, m.order), (BivectorKind::Elliptic, 2, Some(4)));
    assert!(!m.ray.grade(4).is_zero(), "the ray is the product over the planes, grade four included");
    assert_eq!(m.ray, motion);
}

/// THE IDENTITY HAS NO MODE, AND A NON-VERSOR NO ACTION.
#[test]
fn the_identity_reads_none_and_a_non_versor_is_not_read() {
    assert_eq!(read(&Mv::scalar(q(3, 1))), Vec::new());
    let not_a_motion = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | EM, q(1, 1)));
    assert!(motion_modes(&not_a_motion, &mut budget()).is_none(), "on the light cone: no inverse, no action");
}

/// EQ1 — TWO RAYS ARE EQUAL BY PROPORTION. A non-simple bivector equals
/// itself (its `b̃ b` carries grade four, which the old reading refused);
/// a multiple is the same ray; another ray is not.
#[test]
fn rays_are_equal_by_proportion() {
    let coupled = Mv::blade(E1 | E2, q(1, 1)).add(&Mv::blade(E3 | EM, q(1, 2)));
    assert!(!coupled.geo(&coupled.reverse()).grade(4).is_zero(), "the fixture is non-simple");
    assert_eq!(coupled, coupled.clone(), "a non-simple bivector equals itself");
    let omega = rotator(&q(1, 1), &q(1, 1)).geo(&Mv::scalar(q(5, 1)).add(&Mv::blade(E3 | EM, q(4, 1))));
    assert_eq!(omega.scale(&q(-7, 2)), omega, "−7/2 · Ω is Ω");
    assert_ne!(omega, rotator(&q(1, 1), &q(1, 1)), "a different ray is not");
    assert_eq!(projective_ratio(&omega.scale(&q(-7, 2)), &omega), Some(q(-7, 2)));
    assert_eq!(projective_ratio(&Mv::zero(), &Mv::zero()), None, "zero is no ray");
}

/// MD1b — THREE PLANES AT `n = 4` READ THREE VALUES. `F²`'s square-free
/// polynomial is a cubic with the rational roots `−1, −4, 9`, which the
/// quadratic formula refused as `Irrational(3)`. At `n = 4` the base has
/// five spacelike directions, so two planes turn and the third boosts; at
/// `n = 5` three turn.
#[test]
fn three_planes_read_three_modes_by_the_rational_root_theorem() {
    let plane = |i: usize, j: usize| base_axis(i) | base_axis(j);
    let at_four = Mv::blade(plane(1, 2), q(1, 1))
        .add(&Mv::blade(plane(3, 4), q(2, 1)))
        .add(&Mv::blade(EP | EM, q(3, 1)));
    let kinds = |b: &Mv| match modes(b, &mut budget()) {
        Ok(ms) => ms.iter().map(|m| (m.kind, m.planes)).collect::<Vec<_>>(),
        Err(e) => panic!("undecomposed: {e:?}"),
    };
    assert_eq!(
        kinds(&at_four),
        vec![(BivectorKind::Elliptic, 1), (BivectorKind::Elliptic, 1), (BivectorKind::Hyperbolic, 1)]
    );
    let at_five = Mv::blade(plane(1, 2), q(1, 1))
        .add(&Mv::blade(plane(3, 4), q(2, 1)))
        .add(&Mv::blade(base_axis(5) | EP, q(3, 1)));
    assert_eq!(kinds(&at_five), vec![(BivectorKind::Elliptic, 1); 3]);
}
