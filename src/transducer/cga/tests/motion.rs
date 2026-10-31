// 78.12, MR1, EQ1, MD1b — a motion's modes read off its own action: rational
// `c`, exact rays that commute and multiply back to the motion, the 180° turn
// the plane's blade; rays equal by proportion; rational roots at any degree.

use super::common::{ax, budget, q};
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

/// THE 90°-TURN SCREW, which the chart could not reach: its turn reads
/// elliptic, `c = 0`, closing at four; its slip parabolic and open; the two
/// rays commute and multiply back to the screw.
#[test]
fn motion_modes_read_turn_90_screw_closing_at_four() {
    let screw = rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    assert!(cayley_inverse(&screw).is_none(), "the chart has no point here: tan(π/8) ∉ ℚ");
    let ms = read(&screw);
    assert_eq!(
        ms.iter().map(|m| (m.kind, m.planes, m.order)).collect::<Vec<_>>(),
        vec![(BivectorKind::Parabolic, 1, None), (BivectorKind::Elliptic, 1, Some(4))]
    );
    assert_eq!(ms[1].c, q(0, 1), "a 90° turn: cos θ = 0, rational");
    assert!(ms[0].ray.geo(&ms[1].ray).sub(&ms[1].ray.geo(&ms[0].ray)).is_zero(), "the rays commute");
    assert_eq!(product(&ms), screw, "and multiply back to the screw, as a ray");
}

/// THE 180° TURN IS THE PLANE'S BLADE: `c = −1`, the ray degenerate, the
/// mode's versor `e₁₂` itself, and `−e₁₂` the same rotation — the held `ℤ₂`
/// by construction.
#[test]
fn motion_modes_read_turn_180_as_plane_blade() {
    let turn_180 = Mv::blade(E1 | E2, q(1, 1));
    let [m] = read(&turn_180).try_into().expect("one mode");
    assert_eq!((m.kind, m.planes, m.c.clone(), m.order), (BivectorKind::Elliptic, 1, q(-1, 1), Some(2)));
    assert_eq!(m.ray, turn_180, "the ray is the plane's blade");
    assert_eq!(m.ray, turn_180.scale(&q(-1, 1)), "and its negative is the same ray: the held ℤ₂");
}

/// THE (3,4,5) TURN READS `c = −7/25` — `cos θ` with `cos(θ/2) = 3/5` — a
/// rosette that closes at no order; a boost reads hyperbolic at `cosh θ`.
#[test]
fn motion_modes_read_pythagorean_rosette_and_boost_cosh() {
    let turn = rotator(&q(3, 1), &q(4, 1));
    let [m] = read(&turn).try_into().expect("one mode");
    assert_eq!((m.kind, m.c.clone(), m.order), (BivectorKind::Elliptic, q(-7, 25), None));
    assert_eq!(m.ray, turn);
    let boost = Mv::scalar(q(5, 1)).add(&Mv::blade(E1 | EM, q(4, 1)));
    let [b] = read(&boost).try_into().expect("one mode");
    assert_eq!((b.kind, b.c.clone(), b.order), (BivectorKind::Hyperbolic, q(41, 9), None));
    assert_eq!(b.ray, boost);
}

/// A DISPIRATION: a 90° turn and a boost on orthogonal planes, two
/// modes whose rays multiply back to the motion.
#[test]
fn motion_modes_read_dispiration_multiplying_back() {
    let turn = rotator(&q(1, 1), &q(1, 1));
    let boost = Mv::scalar(q(5, 1)).add(&Mv::blade(E3 | EM, q(4, 1)));
    let motion = turn.geo(&boost);
    let ms = read(&motion);
    let kinds: Vec<BivectorKind> = ms.iter().map(|m| m.kind).collect();
    assert_eq!(kinds, vec![BivectorKind::Elliptic, BivectorKind::Hyperbolic], "by c: 0 before 41/9");
    assert_eq!(product(&ms), motion);
}

/// AN ISOCLINIC PAIR IS HELD WHOLE: equal 90° turns on `e₁₂` and `e₃₄`
/// read one mode over two planes, its ray carrying the grade-four term the
/// planes' product has, no split among them chosen.
#[test]
fn motion_modes_hold_isoclinic_pair_as_one_mode() {
    let e34 = ax(3) | ax(4);
    let motion = rotator(&q(1, 1), &q(1, 1)).geo(&rotator_on_blade(&q(1, 1), &q(1, 1), e34));
    let [m] = read(&motion).try_into().expect("one mode");
    assert_eq!((m.kind, m.planes, m.order), (BivectorKind::Elliptic, 2, Some(4)));
    assert!(!m.ray.grade(4).is_zero(), "the ray is the product over the planes, grade four included");
    assert_eq!(m.ray, motion);
}

/// THE IDENTITY HAS NO MODE, AND A NON-VERSOR NO ACTION.
#[test]
fn motion_modes_read_none_for_identity_refuse_non_versor() {
    assert_eq!(read(&Mv::scalar(q(3, 1))), Vec::new());
    let not_a_motion = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | EM, q(1, 1)));
    assert!(motion_modes(&not_a_motion, &mut budget()).is_none(), "on the light cone: no inverse, no action");
}

/// EQ1 — TWO RAYS ARE EQUAL BY PROPORTION. A non-simple bivector equals
/// itself (its `b̃ b` carries grade four, which the old reading refused);
/// a multiple is the same ray; another ray is not.
#[test]
fn mv_eq_holds_by_proportion() {
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
fn modes_read_three_planes_by_rational_roots() {
    let plane = |i: usize, j: usize| ax(i) | ax(j);
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
        .add(&Mv::blade(ax(5) | EP, q(3, 1)));
    assert_eq!(kinds(&at_five), vec![(BivectorKind::Elliptic, 1); 3]);
}

/// 78.62 — THE CHARGE IS READ OFF THE ACTION, NEVER A SQUARED SCALAR: a turn
/// and a dilation carry it, the identity and a translation do not — the
/// translation's `(R + R⁻¹)/2` is not the identity, its values all `1` — a
/// turn composed with a slide carries it, and an element with no action is
/// unread.
#[test]
fn charged_holds_where_mode_turns_or_boosts() {
    let turn = rotator(&q(2, 1), &q(1, 1));
    let slide = translator_at(&[q(3, 1), q(0, 1), q(0, 1)]);
    assert_eq!(charged(&turn), Some(true), "a turn");
    assert_eq!(charged(&dilator(&q(2, 1))), Some(true), "a dilation");
    assert_eq!(charged(&Mv::scalar(q(1, 1))), Some(false), "the identity");
    assert_eq!(charged(&slide), Some(false), "a translation: every value 1, its S not the identity");
    assert_eq!(charged(&turn.geo(&slide)), Some(true), "a turn and a slide composed");
    assert_eq!(charged(&Mv::scalar(q(1, 1)).add(&e_inf())), None, "no versor: no action, unread");
}

/// 78.62 — A MODE'S ORDER IS READ OFF ITS RATIONAL POINT, never by powers:
/// for every letter of the seventeen, its square and its cube — each one
/// turning plane, so the plane's order is the motion's — the order MR1 reads
/// off `c` is the order the motion's own powers close at.
#[test]
fn motion_mode_order_read_off_rational_point() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut checked = 0;
    for l in LetterId::all() {
        for k in 1..=3usize {
            let m = Word::from_letters(&vec![(l, false); k]).rotor().lift();
            let Some(Ok(modes)) = motion_modes(&m, &mut budget()) else { continue };
            let turning: Vec<_> = modes.iter().filter(|x| x.kind == BivectorKind::Elliptic).collect();
            let [mode] = turning.as_slice() else { continue };
            assert_eq!(mode.order, finite_order(&m).map(|o| o.get()), "letter {l:?}^{k}: c = {}", mode.c);
            checked += 1;
        }
    }
    assert!(checked >= 32, "the turning letters and the letters' powers were read: {checked}");
}
