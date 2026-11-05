// A21 — a clause's content written out reads back as the same exact objects,
// and a value of `Cl(p, q)` has one writing: any other is refused.

use crate::physics::laws::clauses::writing::*;
use crate::physics::laws::clauses::*;
use crate::transducer::address::put_key_count;
use crate::transducer::alphabet::put_q;
use algebra::Clifford;
use forms::Q;

fn held() -> Vec<Registered> {
    register().expect("the register stands")
}

/// Every registered clause, as served, reads back as itself, with no byte
/// left over.
#[test]
fn put_clause_round_trips() {
    for r in held() {
        let mut out = Vec::new();
        put_clause(&mut out, &r.clause);
        let mut p = 0;
        assert_eq!(take_clause(&out, &mut p), Some(r.clause.clone()), "{}", r.clause.id.law);
        assert_eq!(p, out.len());
    }
}

/// Every object the hypha reaches, and every clause's content address,
/// reads back as itself.
#[test]
fn put_object_round_trips() {
    for r in held() {
        for o in &r.reached {
            let mut out = Vec::new();
            put_object(&mut out, o);
            let mut p = 0;
            assert_eq!(take_object(&out, &mut p).as_ref(), Some(o));
            assert_eq!(p, out.len());
        }
        let mut out = Vec::new();
        put_content(&mut out, &r.clause.id, &r.clause.fixtures);
        let mut p = 0;
        assert_eq!(take_content(&out, &mut p), Some((r.clause.id.clone(), r.clause.fixtures.clone())));
    }
}

/// A value written with its masks out of order, or with a zero
/// coefficient, is no writing of a value: the one writing is the only one
/// read.
#[test]
fn take_clifford_refuses_second_writing() {
    let write = |terms: &[(usize, i64)]| {
        let mut out = Vec::new();
        put_key_count(&mut out, algebra::EM);
        put_key_count(&mut out, terms.len());
        for (mask, c) in terms {
            put_key_count(&mut out, *mask);
            put_q(&mut out, &Q::from_integer((*c).into()));
        }
        out
    };
    let mut p = 0;
    assert!(take_clifford(&write(&[(0, 3), (algebra::E1, 2)]), &mut p).is_some(), "ascending, non-zero");
    for terms in [&[(algebra::E1, 2), (0, 3)][..], &[(0, 3), (0, 1)], &[(0, 0)]] {
        let mut p = 0;
        assert!(take_clifford(&write(terms), &mut p).is_none(), "{terms:?}");
    }
}

/// A fixture or object tag no form names reads as none: past the last tag,
/// LAW-PASSAGE v2's retired tags 0–3 and `FORMS-COORDINATE` v2's retired
/// tags 6 and 7 (and its retired object tag 3), whatever follows them.
#[test]
fn take_fixture_refuses_unknown_tag() {
    let mut product = Vec::new();
    put_clifford(&mut product, &Clifford::blade(algebra::EM, algebra::E1, Q::from_integer(1.into())).expect("a blade"));
    for tag in [0u8, 1, 2, 3, 6, 7, 12] {
        let mut bytes = vec![tag];
        bytes.extend_from_slice(&product);
        bytes.extend_from_slice(&product);
        let mut p = 0;
        assert!(take_fixture(&bytes, &mut p).is_none(), "tag {tag}");
    }
    for tag in [3u8, 9] {
        let mut p = 0;
        assert!(take_object(&[tag, 1, 1], &mut p).is_none(), "object tag {tag}");
    }
}

/// A coordinate whose lanes or keys stand out of canonical order is a
/// second writing and reads as none; the canonical writing reads back.
#[test]
fn take_coordinate_refuses_second_writing() {
    let held = held();
    let fixtures = held.iter().find(|r| r.clause.id.law == forms::coordinate::LAW_ID).map(|r| r.clause.fixtures.as_slice());
    let Some([Fixture::Ordered { bodies }, ..]) = fixtures else { panic!("the coordinate's order fixture") };
    let [some, ..] = bodies.as_slice() else { panic!("a body") };
    // A body at an origin of no enclosure: genesis (32), epoch (4), the
    // enclosure count (one byte), `h` (8), the lane count (one byte), then
    // S's name byte, 0.
    let body = forms::coordinate::Coordinate { origin: forms::coordinate::Origin::default(), ..some.clone() };
    let mut out = Vec::new();
    put_coordinate(&mut out, &body);
    let mut p = 0;
    assert_eq!(take_coordinate(&out, &mut p), Some(body));
    let lane_at = 32 + 4 + 1 + 8 + 1;
    assert_eq!(out.get(lane_at), Some(&0), "S's byte");
    // Naming S as H puts H twice: no lane set.
    if let Some(b) = out.get_mut(lane_at) {
        *b = 3;
    }
    let mut p = 0;
    assert!(take_coordinate(&out, &mut p).is_none());
}
