// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.179 §1; MU-SENSING §3.1 — the geometric manifold an observation lifts
// into, one factor per phase, replacing legacy "envelopes".

use crate::transducer::manifold::{Axis, Manifold, Phase};
use crate::transducer::sample::{Place, Sample};

fn sample_1d(symbols: &[u8]) -> Sample {
    Sample::new(vec![symbols.len()], 1, symbols.to_vec(), Place::new(vec![]))
        .expect("valid 1d sample")
}

fn sample_2d(rows: usize, cols: usize, symbols: &[u8]) -> Sample {
    Sample::new(vec![rows, cols], 1, symbols.to_vec(), Place::new(vec![]))
        .expect("valid 2d sample")
}

#[test]
fn periodic_content_becomes_circle() {
    let s = sample_1d(&[1, 2, 1, 2]);
    let m = Manifold::lift_from(&s);
    assert_eq!(m.rank(), 1);
    assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 2 }));
    assert!(m.is_torus(), "a 1D circle is a 1-torus T¹");
}

#[test]
fn constant_content_becomes_circle_period_one() {
    let s = sample_1d(&[5, 5, 5]);
    let m = Manifold::lift_from(&s);
    assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 1 }));
}

#[test]
fn non_repeating_content_becomes_interval() {
    let s = sample_1d(&[1, 2, 3, 4]);
    let m = Manifold::lift_from(&s);
    assert_eq!(m.axes()[0], Axis::One(Phase::Interval { extent: 4 }));
    assert!(!m.is_torus());
}

#[test]
fn fully_periodic_2d_sample_is_torus_t2() {
    let s = sample_2d(2, 4, &[7, 8, 7, 8, 7, 8, 7, 8]);
    let m = Manifold::lift_from(&s);
    assert_eq!(m.rank(), 2);
    assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 1 }));
    assert_eq!(m.axes()[1], Axis::One(Phase::Circle { period: 2 }));
    assert!(m.is_torus(), "both phases circles => torus T²");
}

#[test]
fn torus_constructor_creates_all_circle_phases() {
    let t = Manifold::torus(&[3, 5, 7]);
    assert_eq!(t.rank(), 3);
    assert!(t.is_torus(), "T³");
    assert_eq!(t.measure(), Some(105));
}

#[test]
fn sample_lift_produces_manifold() {
    let s = sample_1d(&[1, 2, 1, 2]);
    let m = s.lift();
    assert_eq!(m.rank(), 1);
    assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 2 }));
}

/// CONFIRMED 2026-11-06 ON #32 — A PARTIAL PERIOD IS NO CIRCLE: content that
/// repeats with a period not dividing the extent reads as an interval,
/// since taken round the extent the far seam does not glue to the near one;
/// the extent alone never makes a circle. `01010` repeats with period 2 on
/// 5 sites, and `0120120` with period 3 on 7: both intervals. The same
/// content on 4 and 6 sites closes.
#[test]
fn partial_period_reads_as_an_interval() {
    let axis = |symbols: &[u8]| Manifold::lift_from(&sample_1d(symbols)).axes()[0].clone();
    assert_eq!(axis(&[0, 1, 0, 1, 0]), Axis::One(Phase::Interval { extent: 5 }), "period 2 does not divide 5");
    assert_eq!(axis(&[0, 1, 2, 0, 1, 2, 0]), Axis::One(Phase::Interval { extent: 7 }), "period 3 does not divide 7");
    assert_eq!(axis(&[0, 1, 0, 1]), Axis::One(Phase::Circle { period: 2 }), "period 2 divides 4: the seam glues");
    assert_eq!(axis(&[0, 1, 2, 0, 1, 2]), Axis::One(Phase::Circle { period: 3 }), "period 3 divides 6");
}
