// 21 step 1 — the gauge fiber: sections that close condense into a torsion
// cycle, and the cycle speaks orbit members no witness ever supplied.

use crate::transducer::element::*;
use crate::transducer::germinate::germinate;

/// THE TORSE CAPACITY THEOREM: a 3-cycle hue rotation derived from ONE
/// witnessed section closes (σ³ = id), so the cycle speaks the THIRD orbit
/// member that was never individually witnessed — capacity earned by exact
/// structure, in O(1) storage where the list held O(n).
#[test]
fn a_closed_cycle_speaks_unwitnessed_members() {
    // base: transitions 1→2, 2→3, 3→1 over three cells.
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = from_residual(&germinate(&a, &b).unwrap());
    // One witnessed sibling: σ·base where σ = {1↦2, 2↦3, 3↦1}.
    assert!(t.puncture(vec![(2, 3), (3, 1), (1, 2)]));
    assert_eq!(t.torse(), 1, "the section condenses into the cycle");
    assert_eq!(t.torsion().len(), 1, "one torsion cycle (σ, 3)");
    assert_eq!(t.torsion()[0].1, 3, "the period is the torsion_order");
    assert!(t.sections().is_empty(), "the free list drained into the cycle");
    // THE UNWITNESSED MEMBER: σ²·base = transitions 3→1, 1→2, 2→3 — never
    // punctured, never seen — yet the cycle speaks it.
    let unwitnessed = vec![(0, 0, 3, 1), (0, 1, 1, 2), (0, 2, 2, 3)];
    assert!(t.matches_chain(&unwitnessed), "capacity beyond the evidence, exactly");
    assert!(!t.matches_chain(&vec![(0, 0, 5, 6), (0, 1, 1, 2), (0, 2, 2, 3)]), "no hallucination");
    assert_eq!(t.fiber_span(), 3, "the volume counts the full orbit");
}

/// A SWAP is the smallest torsion (ℤ/2): one witnessed section closes at
/// period 2 and condenses. An orbit that neither closes nor spans two
/// sections extracts NOTHING — no false structure from thin evidence.
#[test]
fn a_swap_torses_and_thin_evidence_does_not() {
    let mut swap = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 9)], &[(0, 0, 2u8), (0, 1, 9)]).unwrap());
    assert!(swap.puncture(vec![(2, 1)]));
    assert_eq!(swap.torse(), 1, "sigma 1↦2, 2↦1 closes at period 2");
    assert_eq!(swap.torsion()[0].1, 2);
    // Thin evidence: sections whose hues never recur — no orbit spans two,
    // nothing closes — the free part stays free.
    let mut open = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 9)], &[(0, 0, 2u8), (0, 1, 9)]).unwrap());
    assert!(open.puncture(vec![(3, 4)]));
    assert!(open.puncture(vec![(5, 6)]));
    assert_eq!(open.torse(), 0, "no torsion invented from unrelated sections");
    assert_eq!(open.sections().len(), 2, "the free part is untouched");
}
