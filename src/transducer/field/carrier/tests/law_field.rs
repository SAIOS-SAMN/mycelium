// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §6 (proposal 78.171): the six joint-read fixtures, each an
//! input and its exact decoded output, on the plane `e₊₁e₊₂`.

use super::super::*;
use super::fixtures::{degrees, frame, plane, revolution_turning, three_charts, turning};
use crate::transducer::family::transduce::Factoring;
use forms::passage::{Place, Pole};
use num_bigint::BigInt;

/// FIXTURE 1. One revolution of an orbit through three charts whose
/// transitions are turnings by 30° each on `e₊₁e₊₂`: the holonomy, the
/// precession per revolution, is the turning by 90° on `e₊₁e₊₂`, `K = 0`,
/// of order 4, and its motion is the product of the three motions, `ζ₁₂³`,
/// exact over `ℚ(ζ₁₂)`.
#[test]
fn precession_of_three_thirty_degree_steps_equals_ninety_degrees() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let Ok(Precession::Advances(h)) = revolution_turning(&charts, &p, 30, 1).precession() else {
        panic!("the orbit precesses")
    };
    let angle = h.angle().expect("one plane");
    assert_eq!(angle.plane().key().written(), "e_+1+2", "the plane is named");
    assert_eq!(angle.degrees(), &degrees(90, 1));
    assert_eq!(angle.winding(), BigInt::from(0));
    assert_eq!(angle.order(), Some(4));
    let step = turning(&p, 30, 1);
    let product = step.motion().product(step.motion()).and_then(|m| m.product(step.motion())).expect("in ℚ(ζ₁₂)");
    assert_eq!(product.conductor(), 12, "the product stands in ℚ(ζ₁₂)");
    assert_eq!(angle.motion(), &product);
    assert_eq!(angle.motion(), &Cyclotomic::root(12, 3).expect("ζ₁₂³"));
}

/// FIXTURE 2. A flat region binds no orbit: a path through it, no
/// attractor held, is the identity at every step, and it is refused
/// `Unbound` with its charts, never read as an orbit.
#[test]
fn flat_region_binds_no_orbit_refused_unbound() {
    let (lifted, charts) = three_charts();
    assert_eq!(lifted.atlas().revolution(&charts), Err(FieldRefusal::Unbound { charts }));
}

/// A CLOSED GEODESIC WITH ZERO PRECESSION IN A CURVED FIELD IS RESONANT,
/// never refused: three steps of 30°, 30° and −60° on `e₊₁e₊₂` compose to
/// 0° on every plane, and the orbit reads `Resonant`, its holonomy the
/// identity.
#[test]
fn closed_geodesic_with_zero_precession_is_resonant() {
    let (lifted, charts) = three_charts();
    let p = plane(&lifted.frame, 1, 2);
    let step = |i: usize, n: i64| Transition {
        from: charts[i].clone(),
        to: charts[(i + 1) % 3].clone(),
        motion: Motion::of(vec![turning(&p, n, 1)]).expect("a motion"),
    };
    let orbit = Orbit::of(vec![step(0, 30), step(1, 30), step(2, -60)]).expect("curved, so bound");
    assert_eq!(orbit.precession(), Ok(Precession::Resonant));
    assert!(orbit.holonomy().expect("composes").turnings().is_empty(), "0° on every plane, K = 0");
}

/// FIXTURE 3. A turning by 72° on `e₊₁e₊₂` composed five times: the motion
/// is the identity, exact over `ℚ(ζ₅)`, and the angle composed is 360°,
/// held with its winding `K = 1`, never read back as 0°.
#[test]
fn five_seventy_two_degree_turnings_keep_winding_one() {
    let f = frame(2);
    let p = plane(&f, 1, 2);
    let fifth = turning(&p, 72, 1);
    assert_eq!(fifth.motion().conductor(), 5, "72° stands in ℚ(ζ₅)");
    let m = Motion::of(vec![fifth.clone(); 5]).expect("one plane composes");
    let [whole] = m.turnings() else { panic!("one plane, held at 360°, not dropped as 0°") };
    assert_eq!(whole.degrees(), &degrees(360, 1));
    assert_eq!(whole.winding(), BigInt::from(1));
    assert_eq!(whole.motion(), &Cyclotomic::one(), "the motion is the identity");
    let fifth_power = (0..4).try_fold(fifth.motion().clone(), |acc, _| acc.product(fifth.motion())).expect("in ℚ(ζ₅)");
    assert_eq!(fifth_power, Cyclotomic::one(), "ζ₅⁵ = 1 by reduction modulo Φ₅");
}

/// FIXTURE 4. A phase of period 9 taken apart as `3 · 3`, coarsest first:
/// site 7 maps to `(2/3, 1/3)`, 240° at the coarse level and 120° at the
/// fine level on `e₊₁e₊₂`, each `K = 0`; recomposed, `240° + 120°/3 = 280°
/// = (7/9)·360°`.
#[test]
fn nested_phase_of_site_seven_recomposes_to_two_hundred_eighty_degrees() {
    let p = plane(&frame(2), 1, 2);
    let n = Nested::of(&p, Factoring::new(9, 3, 3).expect("9 = 3·3"), 7).expect("7 is a site");
    assert_eq!((n.coarse.degrees(), n.coarse.winding()), (&degrees(240, 1), BigInt::from(0)));
    assert_eq!((n.fine.degrees(), n.fine.winding()), (&degrees(120, 1), BigInt::from(0)));
    let whole = n.recomposed().expect("recomposes");
    assert_eq!(whole.degrees(), &degrees(280, 1));
    assert_eq!(whole.degrees(), &(degrees(7, 9) * degrees(360, 1)));
}

/// FIXTURE 5. Two candidates over one demand with monodromy 0° and 120° on
/// `e₊₁e₊₂`: one flat (order 1), one torsion of period 3.
#[test]
fn monodromy_reads_flat_and_torsion_of_order_three() {
    let p = plane(&frame(2), 1, 2);
    let flat = Holonomy::of(Motion::of(vec![turning(&p, 0, 1)]).expect("composes"));
    let torsion = Holonomy::of(Motion::of(vec![turning(&p, 120, 1)]).expect("composes"));
    assert_eq!(flat.monodromy(), Monodromy::Flat);
    assert_eq!(torsion.monodromy(), Monodromy::Torsion { order: 3 });
}

/// FIXTURE 6. A state that needs a plane's pole as infinity (`n_∞ = n_Q`
/// under the standard labels) where no membrane encloses it is refused
/// `Unenclosed`; the same pole inside the plane's membrane, `4/5·n_∞`, is a
/// lawful frame state at the open pole. Under the swapped labels the closed
/// pole is infinity and is refused alike.
#[test]
fn stand_refuses_infinity_outside_every_enclosure() {
    let f = frame(1);
    let membrane = Membrane::of(&f, 1).expect("the plane's throat");
    let poles = membrane.poles();
    let infinity = at_pole(poles, Labels::Standard.infinity(), degrees(4, 5)).expect("a pole state");
    assert_eq!(stand(&infinity, poles, Labels::Standard, None), Err(FieldRefusal::Unenclosed { pole: Pole::Open }));
    assert_eq!(stand(&infinity, poles, Labels::Standard, Some(&membrane)), Ok(Place::Pole(Pole::Open)));
    let origin = at_pole(poles, Pole::Closed, degrees(4, 5)).expect("a pole state");
    assert_eq!(stand(&origin, poles, Labels::Standard, None), Ok(Place::Pole(Pole::Closed)));
    assert_eq!(stand(&origin, poles, Labels::Swapped, None), Err(FieldRefusal::Unenclosed { pole: Pole::Closed }));
}
