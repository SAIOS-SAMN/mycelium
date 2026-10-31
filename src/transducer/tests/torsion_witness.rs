use crate::transducer::torsion_witness::*;
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;

/// A torse-extracted ℤ/3 cycle carries a CLOSED orbit witness of period 3 and
/// valency 1/3 — ∮ dX = 0 exactly; an unrelated open family witnesses Open.
#[test]
fn witness_thought_closes_on_cycle_and_names_open() {
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = from_canonical(&germinate(&a, &b).unwrap());
    t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
    assert_eq!(t.torse(), 1);
    let ws = witness_thought(&t);
    assert_eq!(
        ws,
        vec![HolonomicOrbitWitness::Closed { period: 3, valency: HolonomicValency::new(1, 3) }],
        "the cycle's monodromy is proven, not assumed"
    );
    assert_eq!(total_winding(&ws), HolonomicValency::new(1, 3).0, "Σ windings, exact ℚ");
    // An open orbit (σ leaves its domain) witnesses Open with the residual span.
    let open = witness_cycle(&t, &[(1, 9)], 8);
    assert!(matches!(open, HolonomicOrbitWitness::Open { .. }), "no closure is never faked");
}
