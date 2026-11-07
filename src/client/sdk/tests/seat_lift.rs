// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// #147 (ruled 2026-11-06) — A SEAT'S SERIES LIFTED INTO THE FIELD as a
// sample of rank one along its series index: exact values and symbols by
// `Lifted::of`, versors as motions the algebra relates; and a seat's call a
// relay whose A20 certificate is held on the mind and rides the claiming
// form's tape.

use crate::anastome::dialogue::seat::*;
use crate::client::sdk::seat_mind::*;
use crate::transducer::cga::translator_at;
use crate::transducer::sentence::tests::fixtures::cert;
use crate::transducer::sentence::{Axis, Form};
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational as Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

fn at(kind: u16) -> Locator {
    Locator { kind, keys: vec![], occurrence: 0, position: 0 }
}

/// A seat mind that perceived `values[i]` on record `kind` at beat `i + 1`.
fn perceived(kind: u16, values: &[Arg]) -> (SeatMind, Perceived) {
    let mut m = SeatMind::new([7; 32], 7);
    let mut last = Perceived::default();
    for (i, v) in values.iter().enumerate() {
        let seq = i as u64 + 1;
        let p = Percept {
            domain: [7; 32],
            sub_origin: None,
            seat: 7,
            mind: [9; 32],
            local_time: Arg::Count(seq),
            seq,
            records: vec![Record::Plain { kind, args: vec![v.clone()] }],
            point: None,
        };
        last = m.perceive(&p);
    }
    (m, last)
}

fn field(m: &SeatMind, kind: u16) -> crate::transducer::field::carrier::Lifted {
    match lift(m.series(&at(kind)).expect("held")) {
        Ok(SeriesLift::Field(l)) => l,
        other => panic!("a value series lifts into the field: {other:?}"),
    }
}

/// A locator's series is a sample of rank one along its index: one site per
/// observation held, lifted onto two frame planes, its one axis turning on
/// `e₊₁e₊₂`, as any source of rank one is. No plane is assigned by record
/// type.
#[test]
fn seat_series_lifts_as_rank_one_sample_along_its_index() {
    let (m, reading) = perceived(30, &[Arg::Count(1), Arg::Count(2), Arg::Count(2), Arg::Count(5)]);
    let sample = series_sample(m.series(&at(30)).expect("held")).expect("a sample");
    assert_eq!(sample.extents(), &[4], "one site per observation, along the series index");
    let lifted = field(&m, 30);
    assert_eq!(lifted.frame.count(), 2);
    assert_eq!(lifted.planes.len(), 1);
    assert_eq!(lifted.planes[0].key().written(), "e_+1+2");
    assert_eq!((reading.lifted, reading.motions, reading.unlifted), (1, 0, 0), "the percept reads the series lifted");
}

/// Q, BigQ and Count are exact values: one value carried three ways lifts
/// as one symbol, one body. A flag is a symbol: two flags, two symbols.
#[test]
fn exact_values_lift_as_one_value_whatever_their_carrier() {
    let three = [Arg::Count(3), Arg::Q { num: 3, den: 1 }, Arg::big_q(BigInt::from(3), BigUint::from(1u8)).expect("reduced")];
    let (m, _) = perceived(31, &three);
    let symbols: Vec<Vec<u8>> = series_sample(m.series(&at(31)).expect("held")).expect("a sample").symbols().map(|s| s.bytes().to_vec()).collect();
    assert!(symbols.windows(2).all(|w| w[0] == w[1]), "one exact value, one symbol");
    assert_eq!(field(&m, 31).bodies.len(), 1);
    let (f, _) = perceived(32, &[Arg::Flag(true), Arg::Flag(false), Arg::Flag(true)]);
    let lifted = field(&f, 32);
    let mut distinct: Vec<Vec<u8>> = Vec::new();
    for b in &lifted.bodies {
        if !distinct.contains(&b.symbol) {
            distinct.push(b.symbol.clone());
        }
    }
    assert_eq!(distinct.len(), 2, "two flags are two symbols");
}

/// A turning read off a lifted series keeps its winding: the band reaching
/// the series' far end stands at 360° with `K = 1`, never read back as 0°.
#[test]
fn lifted_series_turning_keeps_its_winding() {
    let (m, _) = perceived(33, &[Arg::Count(4), Arg::Count(4), Arg::Count(4)]);
    let lifted = field(&m, 33);
    let far: Vec<_> = lifted.bodies.iter().flat_map(|b| &b.boundary).map(|(_, hi)| hi.phases[0].clone()).collect();
    assert!(far.iter().any(|t| t.degrees() == &q(360, 1) && t.winding() == BigInt::from(1)));
}

/// A versor is a motion, never a symbol: a versor series lifts as its exact
/// versors, each step the versor carrying one onto the next, and the steps
/// composed carry the first onto the last exactly. Each step's conjugacy
/// class is read; a value beside the versors is refused, never written as a
/// symbol.
#[test]
fn versor_series_lifted_as_motions_composes_its_steps() {
    let r = translator_at(&[q(1, 2), q(0, 1), q(0, 1)]);
    let s = translator_at(&[q(0, 1), q(1, 3), q(0, 1)]);
    let v0 = translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let v1 = r.geo(&v0);
    let v2 = s.geo(&v1);
    let v3 = r.geo(&v2);
    let (m, reading) = perceived(34, &[Arg::Mv(v0.clone()), Arg::Mv(v1), Arg::Mv(v2), Arg::Mv(v3.clone())]);
    let motions = match lift(m.series(&at(34)).expect("held")) {
        Ok(SeriesLift::Motions(mo)) => mo,
        other => panic!("a versor series lifts as motions: {other:?}"),
    };
    let steps = motions.steps().expect("every step carried");
    assert_eq!(steps.len(), 3);
    assert!(steps[0].sub(&r).is_zero() && steps[1].sub(&s).is_zero() && steps[2].sub(&r).is_zero(), "each step is the motion that made it");
    let composed = motions.composed().expect("composes");
    assert!(composed.geo(&v0).sub(&v3).is_zero(), "the steps composed carry the first versor onto the last");
    let classes = motions.classes().expect("steps");
    assert_eq!(classes[0], classes[2], "one motion, one conjugacy class");
    assert_eq!((reading.lifted, reading.motions, reading.unlifted), (0, 1, 0));
    let (mixed, _) = perceived(35, &[Arg::Mv(v0), Arg::Count(1)]);
    assert_eq!(lift(mixed.series(&at(35)).expect("held")).map(|_| ()).err(), Some(LiftRefusal::Mixed { at: 1 }));
}

/// A seat's call stays a relay; the A20 certificate of the move it names is
/// held beside its reference and, when the move is claimed, rides the
/// claiming form's certificates section under its own key. Claimed once.
#[test]
fn seat_call_certificate_rides_claiming_form_tape() {
    let mut m = SeatMind::new([7; 32], 7);
    let c = cert(Axis::Unit(0), [0, 1, 0, 0]);
    m.called(2, vec![Arg::Count(1)], [1; 32]);
    m.certify([1; 32], c.clone());
    assert_eq!(m.certificate(&[1; 32]), Some(&c));
    let mut form = Form::default();
    assert!(m.claim(&[1; 32], &mut form));
    assert_eq!(form.certificates.get(&c.key()), Some(&c), "the certificate rides the claiming form's tape");
    assert!(!m.claim(&[1; 32], &mut form), "a move is claimed once");
}

/// A refused call moved nothing: its certificate is let go, and no claim
/// can carry it.
#[test]
fn refused_seat_call_lets_its_certificate_go() {
    let mut m = SeatMind::new([7; 32], 7);
    m.called(2, vec![Arg::Count(1)], [1; 32]);
    m.certify([1; 32], cert(Axis::Unit(0), [0, 1, 0, 0]));
    let o = Outcome { domain: [7; 32], sub_origin: None, seat: 7, mind: [9; 32], call: [1; 32], applied: false, reason: REASON_NOT_APPLIED, deed: [0; 32] };
    assert_eq!(m.outcome(&o), Some(2));
    assert_eq!(m.certificate(&[1; 32]), None);
    assert!(!m.claim(&[1; 32], &mut Form::default()));
}
