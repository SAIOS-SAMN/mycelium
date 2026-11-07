// MU-RELEASE §3a — the register serves LAW-COORDINATE fixture 11b as the
// clause `FORMS-TURNING` v1 at kernel standing, and the hypha's
// reconstruction of each fixture reaches the value 78.173 §5 states.

use crate::physics::laws::clauses::*;
use algebra::{base_axis, Clifford};
use forms::turning::same;
use forms::Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// A frame unit: `e₊ₖ` for `positive`, else `e₋ₖ`.
fn unit(positive: bool, k: usize) -> usize {
    match positive {
        true => base_axis(2 * k - 1),
        false => base_axis(2 * k),
    }
    .expect("the algebra holds the unit")
}

/// `c` times the product of the units, in the order written, under the
/// signature of planes `1..=3`.
fn word(c: Q, units: &[(bool, usize)]) -> Clifford {
    let sig = (1..=3).fold(0, |s, k| s | unit(false, k));
    units.iter().fold(Clifford::blade(sig, 0, c).expect("a scalar"), |w, (p, k)| {
        w.product(&Clifford::blade(sig, unit(*p, *k), q(1, 1)).expect("a unit")).expect("one signature")
    })
}

fn sum(words: &[Clifford]) -> Clifford {
    words.iter().fold(word(q(0, 1), &[]), |s, w| s.sum(w).expect("one signature"))
}

/// FIXTURE 11b, SERVED: the clause `FORMS-TURNING` v1 stands at kernel
/// standing, and its twelve fixtures reach the motions, products and
/// generators of 11b-i, 11b-ii and 11b-iii: `3/5 + 4/5 e₊₁e₊₂`, `4/5 + 3/5
/// e₊₂e₊₃`, `R₁R₂` and `R₂R₁` with `±12/25 e₊₁e₊₃`, their generators
/// `(16, ±12, 9)/37`, the disjoint planes' volume `12/25 e₊₁e₊₂e₋₁e₋₂` and
/// generator `(4/7, 3/7)`, and one plane's `1` and `5/7`.
#[test]
fn register_reaches_stated_turning_values() {
    let held = register().expect("the register stands");
    let r = held.into_iter().find(|r| r.clause.id.law == forms::turning::LAW_ID).expect("the turning clause");
    assert_eq!(r.clause.standing, Standing::Kernel, "signed or vetoed by each kernel");
    let (p1, p2, p3, n1, n2) = ((true, 1), (true, 2), (true, 3), (false, 1), (false, 2));
    let shared = |s: i64| sum(&[word(q(12, 25), &[]), word(q(16, 25), &[p1, p2]), word(q(s * 12, 25), &[p1, p3]), word(q(9, 25), &[p2, p3])]);
    let back = |s: i64| sum(&[word(q(16, 37), &[p1, p2]), word(q(s * 12, 37), &[p1, p3]), word(q(9, 37), &[p2, p3])]);
    let apart = sum(&[word(q(12, 25), &[]), word(q(16, 25), &[p1, p2]), word(q(9, 25), &[n1, n2]), word(q(12, 25), &[p1, p2, n1, n2])]);
    let stated = [
        sum(&[word(q(3, 5), &[]), word(q(4, 5), &[p1, p2])]),
        sum(&[word(q(4, 5), &[]), word(q(3, 5), &[p2, p3])]),
        shared(1),
        shared(-1),
        back(1),
        back(-1),
        sum(&[word(q(4, 5), &[]), word(q(3, 5), &[n1, n2])]),
        apart.clone(),
        apart,
        sum(&[word(q(4, 7), &[p1, p2]), word(q(3, 7), &[n1, n2])]),
        word(q(1, 1), &[p1, p2]),
        word(q(5, 7), &[p1, n1]),
    ];
    assert_eq!(r.reached.len(), stated.len());
    for (i, (o, s)) in r.reached.iter().zip(&stated).enumerate() {
        match o {
            Object::Multivector(v) => assert!(same(v, s), "fixture {i}: {v:?} against {s:?}"),
            other => panic!("fixture {i} reached {other:?}"),
        }
    }
}
