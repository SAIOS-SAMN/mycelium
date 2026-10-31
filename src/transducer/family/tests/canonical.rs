// 17.2, A4 — AN ASSIGNMENT'S KEY IS ITS CONTENT: the carriage as a ray and
// the gauge as a map. One assignment derived with either sign, and its
// gauge listed in either order, is one key (law 2).

use crate::transducer::element;
use crate::transducer::family::extract::assignment_key;
use crate::transducer::sentence::Acting;
use crate::transducer::stalk;

/// An assignment key is one under every scaling of the carriage.
#[test]
fn assignment_key_is_invariant_under_carriage_scaling() {
    let c = element::translator_versor(1, 2);
    let neg = c.scale(&num_rational::BigRational::from_integer(num_bigint::BigInt::from(-1)));
    let chart = stalk::Invariant::counts(&[3, 5]);
    let a = Acting {
        chart: chart.clone(),
        carriage: Box::new(c.clone()),
        gauge: vec![(2, 1), (1, 2)],
        frames: crate::network::nerve::Keyed::new(),
    };
    let b = Acting {
        chart: chart.clone(),
        carriage: Box::new(neg),
        gauge: vec![(1, 2), (2, 1)],
        frames: crate::network::nerve::Keyed::new(),
    };
    assert_eq!(assignment_key(&a), assignment_key(&b), "one assignment: A4 carriage, gauge as a map");

    let other = Acting {
        chart,
        carriage: Box::new(element::translator_versor(1, 3)),
        gauge: vec![(1, 2), (2, 1)],
        frames: crate::network::nerve::Keyed::new(),
    };
    assert_ne!(assignment_key(&a), assignment_key(&other), "a different carriage is a different assignment");
}
