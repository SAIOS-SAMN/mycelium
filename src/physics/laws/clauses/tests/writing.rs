// A21 — a clause's content written out reads back as the same exact objects,
// and a value of `Cl(p, q)` has one writing: any other is refused.

use crate::physics::laws::clauses::writing::*;
use crate::physics::laws::clauses::*;
use crate::transducer::address::put_key_count;
use crate::transducer::alphabet::put_q;
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

/// A fixture or object tag no form names reads as none.
#[test]
fn take_fixture_refuses_unknown_tag() {
    let mut p = 0;
    assert!(take_fixture(&[8], &mut p).is_none());
    let mut p = 0;
    assert!(take_object(&[5], &mut p).is_none());
}
