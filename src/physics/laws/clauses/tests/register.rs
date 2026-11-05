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

/// A WORDING CHANGE CHANGES NO CLAUSE: every Concordance clause is named by
/// its law form's own id and version and holds the form's fixtures, nothing
/// read from the Concordance's text; the passage is a network clause and
/// the coordinate, its fixtures not yet joint-read agreed, a kernel clause;
/// and the hypha reaches an object for every fixture.
#[test]
fn register_equals_forms_law_versions() {
    let held = held();
    let ids: Vec<(String, u16)> = held.iter().map(|r| (r.clause.id.law.clone(), r.clause.id.version)).collect();
    let named: Vec<(String, u16)> = CLAUSES.iter().map(|(law, v)| (law.to_string(), *v)).collect();
    assert_eq!(ids, named);
    assert_eq!(named, [("FORMS-PASSAGE".to_string(), forms::passage::LAW_VERSION), ("FORMS-COORDINATE".to_string(), forms::coordinate::LAW_VERSION)]);
    let standings: Vec<Standing> = held.iter().map(|r| r.clause.standing).collect();
    assert_eq!(standings, [Standing::Network, Standing::Kernel]);
    for r in &held {
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

/// The same value of `Cl(p, q)`: one signature, the same terms.
fn same(a: &Clifford, b: &Clifford) -> bool {
    a.negatives() == b.negatives() && a.terms().eq(b.terms())
}

/// The frame state on plane 1 with `c₊` on `e₊₁` and `c₋` on `e₋₁`.
fn on_plane(plus: Q, minus: Q) -> Clifford {
    let (p, n) = (base_axis(1).expect("e₊₁"), base_axis(2).expect("e₋₁"));
    let sig = EM | n;
    Clifford::blade(sig, p, plus).and_then(|a| a.sum(&Clifford::blade(sig, n, minus)?)).expect("a blade the algebra holds")
}

/// LAW-PASSAGE v3's FIVE FIXTURES REACHED, each pass its own entry:
/// (1) span `3/5` across gives `S = −3/5`, `T = −6/5`, and back `S = 3/5`,
/// `T = 0`; (2) the throat changes nothing; (3) the pole `4/5·n_Z` swaps
/// to `4/5·n_Q`; (4) the even state's pass is odd, and the second pass
/// returns it; (5) a general frame state's two passes are the identity.
#[test]
fn register_reaches_stated_passage_objects() {
    let held = clause(forms::passage::LAW_ID);
    let crossing = |o: &Object| match o {
        Object::Crossing { frame, span, flux } => (frame.clone(), span.clone(), flux.clone()),
        other => panic!("a crossing, not {other:?}"),
    };
    let value = |o: &Object| match o {
        Object::Multivector(x) => x.clone(),
        other => panic!("a multivector, not {other:?}"),
    };
    let input = |f: &Fixture| match f {
        Fixture::Crossed { frame, .. } => frame.clone(),
        Fixture::Ridden { state, .. } => state.clone(),
        other => panic!("a passage fixture, not {other:?}"),
    };
    let [one, back, throat, pole, ride, ride_back, general, general_back] = held.reached.as_slice() else { panic!("eight passes") };
    let (frame, span, flux) = crossing(one);
    assert!(same(&frame, &on_plane(q(1, 1), q(-3, 5))), "1/5·n_Z + 4/5·n_Q");
    assert_eq!((span, flux), (q(-3, 5), q(-6, 5)));
    let (frame, span, flux) = crossing(back);
    assert!(same(&frame, &input(&held.clause.fixtures[0])), "the return");
    assert_eq!((span, flux), (q(3, 5), q(0, 1)));
    let (frame, span, flux) = crossing(throat);
    assert!(same(&frame, &input(&held.clause.fixtures[2])), "the throat changes nothing");
    assert_eq!((span, flux), (q(0, 1), q(0, 1)));
    let (frame, span, flux) = crossing(pole);
    assert!(same(&frame, &on_plane(q(4, 5), q(-4, 5))), "4/5·n_Q");
    assert_eq!((span, flux), (q(-4, 5), q(-8, 5)));
    assert_eq!(degrees(&input(&held.clause.fixtures[4])), [0, 2]);
    assert_eq!(degrees(&value(ride)), [1, 3]);
    assert!(same(&value(ride), &input(&held.clause.fixtures[5])), "e₊·q as fixture 4 states it");
    assert!(same(&value(ride_back), &input(&held.clause.fixtures[4])), "e₊·e₊·q = q");
    let (frame, span, flux) = crossing(general);
    assert!(same(&frame, &input(&held.clause.fixtures[7])));
    assert_eq!((span, flux), (q(-7, 3), q(-11, 3)));
    let (frame, span, flux) = crossing(general_back);
    assert!(same(&frame, &input(&held.clause.fixtures[6])), "the sandwich twice is the identity");
    assert_eq!((span, flux), (q(7, 3), q(1, 1)));
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
