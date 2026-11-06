// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// RULED 2026-11-06 ON #32, THE ENVELOPE — THE SAMPLE AS A MANIFOLD: a phase
// with a period is a circle of that circumference, one without is an
// interval with boundary; the sample is their product, or a fibration where
// an axis is taken apart; a fully periodic sample is a torus.

use super::{sample, sample_by};
use crate::transducer::family::transduce::{AxisManifold, Factoring, PhaseManifold, SampleManifold};

/// A sample whose symbols repeat along every axis with a period dividing
/// its extent reads as a torus, its circles' circumferences the periods: a
/// 4 × 6 checker of period 2 and 3, and a rank-3 stripe at every rank.
#[test]
fn periodic_sample_reads_as_a_torus() {
    let s = sample_by(&[4, 6], |a| u8::try_from((a[0] % 2) * 3 + a[1] % 3).expect("a digit"));
    let m = SampleManifold::of(&s);
    assert_eq!(m.axes, vec![AxisManifold::One(PhaseManifold::Circle { period: 2 }), AxisManifold::One(PhaseManifold::Circle { period: 3 })]);
    assert_eq!(m.torus(), Some(vec![2, 3]), "a fully periodic sample is the torus T² of its periods");
    let deep = sample_by(&[2, 4, 3], |a| u8::try_from(a[1] % 2).expect("a digit"));
    assert_eq!(SampleManifold::of(&deep).torus(), Some(vec![1, 2, 1]), "a constant phase is a circle of one site: T³");
    assert_eq!(SampleManifold::of(&deep).dimension(), 3);
}

/// A phase whose symbols do not repeat, or repeat with a period that does
/// not divide the extent (its far seam does not glue), is an interval with
/// boundary, and the sample is then no torus: a product with boundary.
#[test]
fn aperiodic_phase_reads_as_an_interval() {
    let s = sample(&["0120", "0120"]);
    let m = SampleManifold::of(&s);
    assert_eq!(m.axes, vec![AxisManifold::One(PhaseManifold::Circle { period: 1 }), AxisManifold::One(PhaseManifold::Interval { extent: 4 })]);
    assert_eq!(m.torus(), None, "a phase with boundary: no torus");
    let off = sample(&["01010"]);
    assert_eq!(SampleManifold::of(&off).axes[1], AxisManifold::One(PhaseManifold::Interval { extent: 5 }), "period 2 does not divide 5: the seam does not glue");
}

/// An axis taken apart is a fibration: blocks of `0011` repeated three
/// times read a base circle of one block over a fibre interval of four;
/// within each block the fine phase does not repeat, so the fibre has
/// boundary, while the base closes. An axis whose factoring is not of its
/// extent stays one phase.
#[test]
fn nested_axis_reads_as_a_fibration() {
    let s = sample(&["001100110011"]);
    let f = Factoring::new(12, 3, 4).expect("12 = 3 · 4");
    let m = SampleManifold::nested(&s, &[None, Some(f)]);
    assert_eq!(m.axes[1], AxisManifold::Fibred { base: PhaseManifold::Circle { period: 1 }, fibre: PhaseManifold::Interval { extent: 4 } });
    assert_eq!(m.dimension(), 3, "the fibred axis is two phases");
    assert_eq!(m.torus(), None);
    let alien = Factoring::new(6, 2, 3).expect("6 = 2 · 3");
    assert_eq!(SampleManifold::nested(&s, &[None, Some(alien)]), SampleManifold::of(&s), "no factoring of the axis's extent");
}
