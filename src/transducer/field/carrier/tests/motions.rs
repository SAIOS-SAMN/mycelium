// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ROADMAP track 26 S8 (stage 5) — the field's motions as one exact group:
//! every isometry carries the field's curvature onto its conjugate, a
//! re-period is no versor, and the inversion in the membrane writes no
//! infinity.

use super::super::*;
use super::fixtures::{degrees, frame, plane, revolution_turning, three_charts, turning};
use forms::coordinate::Square;
use forms::passage::{Frame, Place, Pole};
use num_bigint::BigInt;

/// A frame state `H·n_Z + R·n_Q` on `membrane`'s plane.
fn state(membrane: &Membrane, hold: (i64, i64), reach: (i64, i64)) -> crate::transducer::cga::Clifford {
    let f = Frame { hold: degrees(hold.0, hold.1), reach: degrees(reach.0, reach.1) };
    membrane.poles().state(&f).expect("a frame state")
}

/// THE FIELD STAYS SMOOTH UNDER EVERY ISOMETRY: the orbit of three 30°
/// steps on `e₊₁e₊₂` (holonomy 90°, order 4), carried by each generator —
/// a turning on its own plane, a quarter turn on `e₊₂e₊₃` that shares a
/// unit, the flip of `e₊₁`, the pole swap on plane 1, a boost on plane 3 —
/// reads as its holonomy the holonomy carried, with its order and winding
/// kept, and the inverse carries it back. A motion the carrier cannot hold
/// is refused by name: a boost on plane 1 and a 30° turning on `e₊₂e₊₃`.
#[test]
fn field_stays_smooth_under_every_isometry() {
    let f = frame(3);
    let (_, charts) = three_charts();
    let (p12, p23, p13) = (plane(&f, 1, 2), plane(&f, 2, 3), plane(&f, 1, 3));
    let orbit = revolution_turning(&charts, &p12, 30, 1);
    let holonomy = orbit.holonomy().expect("composes");
    let turn = |p: &TurningPlane, n: i64| Generator::Turn(Motion::of(vec![turning(p, n, 1)]).expect("a motion"));
    let flip = Generator::Flip(Unit { square: Square::Positive, plane: 1 });
    let boost = |k: u32| Generator::Boost { plane: k, scale: degrees(3, 1) };
    let isometries = [turn(&p12, 45), turn(&p23, 90), flip, Generator::swap(1), boost(3)];
    let expected = [(&p12, 90), (&p13, 90), (&p12, -90), (&p12, 90), (&p12, 90)];
    for (g, (on, angle)) in isometries.into_iter().zip(expected) {
        let g = FieldMotion::of(g);
        let carried = g.carry_orbit(&f, &orbit).expect("an isometry carries the orbit");
        let read = carried.holonomy().expect("composes");
        let whole = Motion::of(holonomy.turnings().to_vec()).expect("one plane");
        assert_eq!(read.turnings(), g.carry_motion(&f, &whole).expect("carried").turnings(), "{g:?}: curvature carried");
        let a = read.angle().expect("one plane");
        assert_eq!((a.plane(), a.degrees(), a.winding(), a.order()), (on, &degrees(angle, 1), BigInt::from(0), Some(4)), "{g:?}");
        let back = g.inverse().expect("undone").carry_orbit(&f, &carried).expect("carried back");
        assert_eq!(back, orbit, "{g:?}: the inverse carries it back");
    }
    let refused = |g: Generator| FieldMotion::of(g).carry_orbit(&f, &orbit);
    let unnamed = Err(FieldRefusal::NotExact { residue: Residue::Unnamed { plane: p12.key().clone() } });
    assert_eq!(refused(boost(1)), unnamed, "a boost on plane 1 carries e₊₁ off every named plane");
    assert_eq!(refused(turn(&p23, 30)), unnamed, "30° on a plane sharing e₊₂ turns onto no named plane");
}

/// STRETCH IS A RE-PERIOD, NOT A VERSOR: the sites 1 and 2 of a phase of
/// period 3 on `e₊₁e₊₂` stand at 120° and 240°; re-periodised to 4 they
/// stand at 90° and 180°, 90° apart, where every versor of the group keeps
/// them 120° apart (a turning, a flip of either unit, a quarter turn on
/// `e₋₁e₋₂`, which shares none of their units). The re-period's inverse returns
/// them, and a re-period of another axis moves nothing.
#[test]
fn stretch_is_a_re_period_not_a_versor() {
    let f = frame(3);
    let p = plane(&f, 1, 2);
    let at = |n: i64| Point { phases: vec![turning(&p, n, 1)] };
    let (one, two) = (at(120), at(240));
    let apart = |g: &FieldMotion| {
        let (a, b) = (g.carry_point(&one).expect("carried"), g.carry_point(&two).expect("carried"));
        let gap = b.phases[0].degrees() - a.phases[0].degrees();
        num_traits::Signed::abs(&gap)
    };
    let stretch = FieldMotion::of(Generator::RePeriod { axis: 0, from: 3, to: 4 });
    assert_eq!(stretch.carry_point(&one).expect("re-periodised").phases[0].degrees(), &degrees(90, 1));
    assert_eq!(stretch.carry_point(&two).expect("re-periodised").phases[0].degrees(), &degrees(180, 1));
    assert_eq!(apart(&stretch), degrees(90, 1), "the gap is scaled: no versor does that");
    let versors = [
        Generator::Turn(Motion::of(vec![turning(&p, 30, 1)]).expect("a motion")),
        Generator::Flip(Unit { square: Square::Positive, plane: 1 }),
        Generator::Flip(Unit { square: Square::Positive, plane: 2 }),
        Generator::Turn(Motion::of(vec![turning(&TurningPlane::of(&f, Square::Negative, 1, 2).expect("e₋₁e₋₂"), 90, 1)]).expect("a motion")),
    ];
    for g in versors {
        let g = FieldMotion::of(g);
        assert_eq!(apart(&g), degrees(120, 1), "{g:?} keeps the gap");
    }
    let back = stretch.then(&stretch.inverse().expect("undone"));
    assert_eq!(back, FieldMotion::identity(), "a re-period and its inverse compose to the identity");
    let other = FieldMotion::of(Generator::RePeriod { axis: 1, from: 3, to: 4 });
    assert_eq!(other.carry_point(&one).expect("kept"), one, "another axis's re-period moves nothing here");
}

/// THE INVERSION IN THE MEMBRANE NEEDS NO INFINITY: on the lift's own plane
/// the state `4/5·n_∞` stands lawful only inside the membrane; inverted in
/// the membrane, read from the throat alone, it is `4/5·n_Z`, the origin,
/// which stands anywhere. The inversion is the pole swap and the group's
/// swap generator, twice the identity. A boost on the plane by 2 carries
/// the throat `(1, 1)` to `(2, 1/2)`.
#[test]
fn inversion_in_the_membrane_needs_no_infinity() {
    let (lifted, _) = three_charts();
    let m = &lifted.membrane;
    let k = m.plane();
    let infinity = state(m, (0, 1), (4, 5));
    assert_eq!(stand(&infinity, m.poles(), Labels::Standard, None), Err(FieldRefusal::Unenclosed { pole: Pole::Open }));
    assert_eq!(stand(&infinity, m.poles(), Labels::Standard, Some(m)), Ok(Place::Pole(Pole::Open)), "lawful inside");
    let read = |x: Result<crate::transducer::cga::Clifford, FieldRefusal>| m.poles().read(&x.expect("carried")).expect("on the plane");
    let frame_of = |hold: (i64, i64), reach: (i64, i64)| Frame { hold: degrees(hold.0, hold.1), reach: degrees(reach.0, reach.1) };
    let inverted = m.invert(&infinity).expect("on the plane");
    assert_eq!(read(Ok(inverted.clone())), frame_of((4, 5), (0, 1)), "the origin pole, 4/5·n_Z");
    assert_eq!(stand(&inverted, m.poles(), Labels::Standard, None), Ok(Place::Pole(Pole::Closed)), "standing anywhere");
    assert_eq!(read(m.poles().swap(&infinity).map_err(FieldRefusal::Passage)), frame_of((4, 5), (0, 1)), "the inversion is the pole swap");
    let swap = FieldMotion::of(Generator::swap(k));
    assert_eq!(read(swap.carry_state(k, m.poles(), &infinity)), frame_of((4, 5), (0, 1)), "and the group's swap");
    assert_eq!(read(m.invert(&inverted)), frame_of((0, 1), (4, 5)), "twice, the identity");
    assert_eq!(swap.then(&swap), FieldMotion::identity());
    let boost = FieldMotion::of(Generator::Boost { plane: k, scale: degrees(2, 1) });
    assert_eq!(read(boost.carry_state(k, m.poles(), &state(m, (1, 1), (1, 1)))), frame_of((2, 1), (1, 2)));
}

/// A MOTION AND ITS INVERSE COMPOSE TO THE IDENTITY: a word of a turning, a
/// flip, a boost and a re-period, then its inverse, reduces to the empty
/// word; boosts on one plane multiply and a flip twice cancels.
#[test]
fn field_motion_and_its_inverse_compose_to_the_identity() {
    let f = frame(3);
    let g = FieldMotion::of(Generator::Turn(Motion::of(vec![turning(&plane(&f, 1, 2), 30, 1)]).expect("a motion")))
        .then(&FieldMotion::of(Generator::Flip(Unit { square: Square::Negative, plane: 3 })))
        .then(&FieldMotion::of(Generator::Boost { plane: 2, scale: degrees(5, 3) }))
        .then(&FieldMotion::of(Generator::RePeriod { axis: 0, from: 9, to: 3 }));
    assert_eq!(g.word().len(), 4, "four generators, none joined");
    assert_eq!(g.then(&g.inverse().expect("undone")), FieldMotion::identity());
    let b = |n: i64| FieldMotion::of(Generator::Boost { plane: 1, scale: degrees(n, 1) });
    assert_eq!(b(2).then(&b(3)), b(6), "boosts on one plane multiply");
}
