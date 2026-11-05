// CONCORDANCE §0, MU-RELEASE §3a — the register names each clause by its law
// form's id and version, and the hypha's reconstruction of each fixture
// reaches the object the proposals state (78.172 §5, 78.173 §5).

use crate::physics::laws::clauses::*;
use algebra::{base_axis, Clifford, EM};
use forms::coordinate::Side;
use forms::Q;
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

fn held() -> Vec<Registered> {
    register().expect("the register stands")
}

/// The registered clause of the law `law`.
fn clause(law: &str) -> Registered {
    held().into_iter().find(|r| r.clause.id.law == law).expect("a registered clause")
}

/// A WORDING CHANGE CHANGES NO CLAUSE: every clause is named by its law
/// form's own id and version and holds the form's fixtures, nothing read
/// from the Concordance's text; each is a network clause, and the hypha
/// reaches an object for every fixture.
#[test]
fn register_equals_forms_law_versions() {
    let held = held();
    let ids: Vec<(String, u16)> = held.iter().map(|r| (r.clause.id.law.clone(), r.clause.id.version)).collect();
    let named: Vec<(String, u16)> = CLAUSES.iter().map(|(law, v)| (law.to_string(), *v)).collect();
    assert_eq!(ids, named);
    assert_eq!(named, [("FORMS-PASSAGE".to_string(), forms::passage::LAW_VERSION), ("FORMS-COORDINATE".to_string(), forms::coordinate::LAW_VERSION)]);
    for r in &held {
        assert_eq!(r.clause.standing, Standing::Network, "{}", r.clause.id.law);
        assert!(!r.clause.fixtures.is_empty(), "{}", r.clause.id.law);
        assert_eq!(r.reached.len(), r.clause.fixtures.len(), "{}", r.clause.id.law);
    }
    assert_eq!(register(), Ok(held), "the register is one object, built again");
}

/// The degrees a value holds terms on, ascending, each once.
fn degrees(x: &Clifford) -> Vec<u32> {
    let mut d: Vec<u32> = x.terms().map(|(m, _)| m.count_ones()).collect();
    d.sort_unstable();
    d.dedup();
    d
}

/// LAW-PASSAGE's FOUR FIXTURES REACHED: `v·v·q = 0` at the horizon, `v·q`
/// on degrees 1 and 3 only, `[e_Z1Q1, e_Z1Q2] = −2·e_Q1Q2`, and
/// `(e_Z1Z2 + e_Q1Z2)² = 0`.
#[test]
fn register_reaches_stated_passage_objects() {
    let reached = clause(forms::passage::LAW_ID).reached;
    let value = |o: &Object| match o {
        Object::Multivector(x) => x.clone(),
        other => panic!("a multivector, not {other:?}"),
    };
    let [one, two, three, four] = reached.as_slice() else { panic!("four fixtures") };
    assert!(value(one).is_zero(), "v·v·q = 0");
    assert_eq!(degrees(&value(two)), [1, 3]);
    let (q1, q2) = (base_axis(2).expect("e_Q1"), base_axis(4).expect("e_Q2"));
    let turning = Clifford::blade(EM | q1 | q2, q1 | q2, q(-2, 1)).expect("a blade the algebra holds");
    assert!(value(three).difference(&turning).expect("one signature").is_zero(), "−2·e_Q1Q2");
    assert!(value(four).is_zero(), "the null pair squares to zero");
}

/// LAW-COORDINATE's FIXTURES REACHED: the bodies in canonical order, the
/// eight sector indices, the six sides and the refusal of two origins,
/// and the two ranges' bodies.
#[test]
fn register_reaches_stated_coordinate_objects() {
    let held = clause(forms::coordinate::LAW_ID);
    let pairs: Vec<(&Fixture, &Object)> = held.clause.fixtures.iter().zip(&held.reached).collect();
    let Some((Fixture::Ordered { bodies }, Object::Bodies(ordered))) = pairs.iter().copied().find(|(f, _)| matches!(f, Fixture::Ordered { .. })) else {
        panic!("the order")
    };
    let mut written = bodies.clone();
    written.reverse();
    assert_eq!(ordered, &written, "the seven bodies of 1a, in canonical order");
    let indices: Vec<BigInt> = pairs.iter().filter_map(|(_, o)| match o {
        Object::Sector(s) => Some(s.index.clone()),
        _ => None,
    }).collect();
    let stated: Vec<BigInt> = [2i64, 140, 8_400, 30_857, 185_142, -20, -9, 15_551_999_999].into_iter().map(BigInt::from).collect();
    assert_eq!(indices, stated);
    let sides: Vec<Side> = pairs.iter().filter_map(|(_, o)| match o {
        Object::Side(s) => Some(*s),
        _ => None,
    }).collect();
    assert_eq!(sides, [Side::Within, Side::Above, Side::Below, Side::Within, Side::Within, Side::Above]);
    assert!(pairs.iter().any(|(_, o)| matches!(o, Object::Refused(_))), "two origins refused");
    let ranges: Vec<Vec<Q>> = pairs.iter().filter_map(|(f, o)| match (f, o) {
        (Fixture::Range { lane, .. }, Object::Bodies(b)) => Some(b.iter().map(|c| c.lane(*lane).clone()).collect()),
        _ => None,
    }).collect();
    assert_eq!(ranges, [vec![q(1, 2), q(1, 2), q(1, 2), q(1, 2), q(3, 5), q(7, 10), q(4, 5)], vec![q(0, 1), q(1, 4)]]);
}
