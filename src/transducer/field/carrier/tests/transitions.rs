// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A family's pairs lifted as transitions (ROADMAP track 26 S7 stage 4;
//! 78.171 §8, the lift's convention agreed 2026-11-06 with netstratum
//! docs/117 §11.6): every signed permutation of the phases, and its
//! re-period, lifts whole as a motion of the field.

use super::super::*;
use super::fixtures::{degrees, frame, plane, sample_of, turning};
use crate::core::Q;
use crate::transducer::cga::Clifford;
use crate::transducer::family::transduce::{factorisations, factorisations_need, Enclosure, Motion as Transduced, Sense, SignedPermutation};
use forms::coordinate::Square;
use num_bigint::BigInt;

/// The point of the lift at boundary `sites` of a frame of `periods`, on
/// the lift's planes of rank `periods.len()`.
fn point_at(planes: &[TurningPlane], sites: &[usize], periods: &[usize]) -> Point {
    let count = |x: usize| i64::try_from(x).expect("a small count");
    let phases = planes.iter().zip(sites).zip(periods).map(|((p, &s), &period)| turning(p, 360 * count(s), count(period))).collect();
    Point { phases }
}

/// EVERY SIGNED PERMUTATION LIFTS WHOLE AS A MOTION OF THE FIELD. At rank 3
/// each of the 48 signed permutations, re-periodded onto an output whose
/// first period is doubled, is lifted (`motion_of`). The lifted motion
/// carries every boundary point of the lift exactly where the transduction
/// carries its turns: output phase `j` reads source `σ(j)` at boundary `s`,
/// or `p − s` where reversed, on the output's period. No permutation is
/// refused and none is lifted in part.
#[test]
fn every_signed_permutation_lifts_whole_as_a_motion_of_the_field() {
    let periods = [2usize, 3, 4];
    let f = frame(4);
    let planes: Vec<TurningPlane> = (1..=3).map(|k| plane(&f, k, 4)).collect();
    let from = Enclosure::new(periods.to_vec()).expect("an enclosure");
    let mut lifted = 0;
    for permutation in SignedPermutation::all(3) {
        let moved = permutation.enclosure(&from).expect("moved");
        let mut onto = moved.periods().to_vec();
        onto[0] *= 2;
        let motion = Transduced { permutation: permutation.clone(), onto: Enclosure::new(onto.clone()).expect("an enclosure") };
        let field = motion_of(&f, &from, &motion).expect("every signed permutation lifts");
        for s0 in 0..=periods[0] {
            for s1 in [0, 1, periods[1]] {
                for s2 in [1, 3] {
                    let sites = [s0, s1, s2];
                    let carried = field.carry_point(&point_at(&planes, &sites, &periods)).expect("carried exactly");
                    let expected: Vec<usize> = permutation
                        .images()
                        .iter()
                        .map(|&(k, sense)| match sense {
                            Sense::Kept => sites[k],
                            Sense::Reversed => periods[k] - sites[k],
                        })
                        .collect();
                    assert_eq!(carried, point_at(&planes, &expected, &onto), "{permutation:?} at {sites:?}");
                }
            }
        }
        lifted += 1;
    }
    assert_eq!(lifted, 48, "the whole hyperoctahedral group of rank 3");
}

/// A REVERSAL IS THE REFLECTION IN ITS OWN UNIT ALONE (the coordinators'
/// fixture, n = 2, the shared unit `e₊₃`): `x = 3/5·e₊₃ + 4/5·e₊₁` maps to
/// `3/5·e₊₃ − 4/5·e₊₁` under `x ↦ −e₊₁·x·e₊₁`, and with `1/2·e₊₂` added that
/// part is kept, so axis 2's pair `(3/5, 1/2)` is unchanged. On the lift's
/// degrees the flip of `e₊₁` reads axis 1's phase `θ` as `−θ` and keeps axis
/// 2's.
#[test]
fn reversal_is_the_reflection_in_its_own_unit_alone() {
    let f = frame(3);
    let unit = |k: u32| f.unit_value(Square::Positive, k).expect("a unit");
    let scalar = |n: i64, d: i64| Clifford::blade(f.negatives(), 0, Q::new(BigInt::from(n), BigInt::from(d))).expect("a scalar");
    let term = |c: (i64, i64), k: u32| scalar(c.0, c.1).product(&unit(k)).expect("one signature");
    let reflect = |x: &Clifford| scalar(-1, 1).product(&unit(1)).and_then(|m| m.product(x)).and_then(|m| m.product(&unit(1))).expect("one signature");
    let x = term((3, 5), 3).sum(&term((4, 5), 1)).expect("a vector");
    let y = term((3, 5), 3).difference(&term((4, 5), 1)).expect("a vector");
    let same = |a: &Clifford, b: &Clifford| a.difference(b).expect("one signature").is_zero();
    assert!(same(&reflect(&x), &y), "the reflection in e₊₁ reverses axis 1");
    let half = term((1, 2), 2);
    let (x2, y2) = (x.sum(&half).expect("a vector"), y.sum(&half).expect("a vector"));
    assert!(same(&reflect(&x2), &y2), "axis 2's part, and its pair (3/5, 1/2), is kept");

    let planes = [plane(&f, 1, 3), plane(&f, 2, 3)];
    let at = Point { phases: vec![turning(&planes[0], 120, 1), turning(&planes[1], 90, 1)] };
    let flipped = FieldMotion::of(Generator::Flip(Unit { square: Square::Positive, plane: 1 })).carry_point(&at).expect("carried");
    assert_eq!(flipped.phases[0].degrees(), &degrees(-120, 1), "θ ↦ −θ on axis 1");
    assert_eq!(flipped.phases[1], at.phases[1], "axis 2 fixed");
}

/// A PAIR IS LIFTED AS TRANSITIONS BETWEEN ITS BODIES' CHARTS. A rank-2
/// sample and its transpose: under the factorisation by the transposition,
/// every body is carried whole, each transition runs from its input body's
/// chart to its output body's chart by the lifted motion, and that motion
/// carries each one-band body's lifted corners onto the output body's.
#[test]
fn pair_lifted_as_transitions_between_its_bodies_charts() {
    let x = sample_of(&[2, 3], "120300");
    let y = sample_of(&[3, 2], "132000");
    let need = factorisations_need(2, 1).expect("a need");
    let held = factorisations(&x, &y, &mut crate::physics::Physics::default().stroke_n(need)).expect("read");
    let swap = SignedPermutation::new(vec![(1, Sense::Kept), (0, Sense::Kept)]).expect("the transposition");
    let f = held.held().iter().find(|f| f.motion.permutation == swap).expect("the transposition factors the pair");
    let transitions = transitions_of(&x, &y, f).expect("lifted");
    assert!(!transitions.is_empty());
    let (source, output) = (Lifted::of(&x).expect("lifts"), Lifted::of(&y).expect("lifts"));
    let motion = motion_of(&source.frame, &Enclosure::of(&x), &f.motion).expect("lifted");
    for t in &transitions {
        let t = t.as_ref().expect("every body carried whole");
        assert_eq!(t.motion, motion);
        assert!(source.bodies.iter().any(|b| b.chart == t.from), "from an input body's chart");
        assert!(output.bodies.iter().any(|b| b.chart == t.to), "to an output body's chart");
    }
    for symbol in [1u8, 2, 3] {
        let body = |l: &Lifted| l.bodies.iter().find(|b| b.symbol == [symbol]).cloned().expect("the body");
        let (a, b) = (body(&source), body(&output));
        let carried: Vec<(Point, Point)> =
            a.boundary.iter().map(|(lo, hi)| (motion.carry_point(lo).expect("carried"), motion.carry_point(hi).expect("carried"))).collect();
        assert_eq!(carried, b.boundary, "body {symbol}'s corners carried onto the output's");
    }
}
