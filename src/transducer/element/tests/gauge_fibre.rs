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
    // base: transitions 1→2, 2→3, 3→1 over three sites.
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = from_canonical(&germinate(&a, &b).unwrap());
    // One witnessed sibling: σ·base where σ = {1↦2, 2↦3, 3↦1}.
    assert!(t.puncture(vec![(2, 3), (3, 1), (1, 2)]));
    assert_eq!(t.torse(), 1, "the section condenses into the cycle");
    assert_eq!(t.torsion().len(), 1, "one torsion cycle (σ, 3)");
    assert_eq!(t.torsion()[0].period, 3, "the period is the torsion_order");
    assert!(t.sections().is_empty(), "the free list drained into the cycle");
    // THE UNWITNESSED MEMBER: σ²·base = transitions 3→1, 1→2, 2→3 — never
    // punctured, never seen — yet the cycle speaks it.
    let unwitnessed = vec![(0, 0, 3, 1), (0, 1, 1, 2), (0, 2, 2, 3)];
    assert!(t.matches_chain(&unwitnessed), "capacity beyond the evidence, exactly");
    assert!(!t.matches_chain(&[(0, 0, 5, 6), (0, 1, 1, 2), (0, 2, 2, 3)]), "no hallucination");
    assert_eq!(t.fiber_span(), Some(3), "the volume counts the full orbit");
}

/// A SWAP is the smallest torsion (ℤ/2): one witnessed section closes at
/// period 2 and condenses. An orbit that neither closes nor spans two
/// sections extracts NOTHING — no false structure from thin evidence.
#[test]
fn a_swap_torses_and_thin_evidence_does_not() {
    let mut swap = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 9)], &[(0, 0, 2u8), (0, 1, 9)]).unwrap());
    assert!(swap.puncture(vec![(2, 1)]));
    assert_eq!(swap.torse(), 1, "sigma 1↦2, 2↦1 closes at period 2");
    assert_eq!(swap.torsion()[0].period, 2);
    // Thin evidence: sections whose hues never recur — no orbit spans two,
    // nothing closes — the free part stays free.
    let mut open = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 9)], &[(0, 0, 2u8), (0, 1, 9)]).unwrap());
    assert!(open.puncture(vec![(3, 4)]));
    assert!(open.puncture(vec![(5, 6)]));
    assert_eq!(open.torse(), 0, "no torsion invented from unrelated sections");
    assert_eq!(open.sections().len(), 2, "the free part is untouched");
}

// PD1, 78.43(2) — THE PERIOD IS READ, THE MEMBERSHIP IS DECIDED ON THE
// CYCLES. Under the sensor's PALETTE = 10 the largest order is Landau's
// g(10) = 30, below the retired cap of 32, so no reading of the suite moves
// and the guard is the MECHANISM: every cycle type the palette admits, each
// read against the walk the traversal made. The walk lives only here, as the
// reference the reading must agree with.

/// The cycle types of the palette: every partition of ten, largest part first.
fn partitions(n: usize, max: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for part in (1..=max.min(n)).rev() {
        for mut rest in partitions(n - part, part) {
            rest.insert(0, part);
            out.push(rest);
        }
    }
    out
}

/// One permutation of the palette's hues with the given cycle type: each
/// part a run of consecutive hues turning one step, every hue listed.
fn permutation_of(parts: &[usize]) -> Vec<(u8, u8)> {
    let mut sigma = Vec::new();
    let mut start = 0u8;
    for &len in parts {
        let len = u8::try_from(len).expect("a part of ten");
        for i in 0..len {
            sigma.push((start + i, start + (i + 1) % len));
        }
        start += len;
    }
    sigma
}

/// The base that carries every hue of the palette.
fn palette_base() -> Vec<(u8, u8)> {
    vec![(0, 1), (2, 3), (4, 5), (6, 7), (8, 9)]
}

/// THE REFERENCE: the orbit of the base walked until it returns, and its
/// members σ¹ … σ^(ℓ−1). A permutation of ten hues returns within 10! steps.
fn walked(sigma: &[(u8, u8)], base: &[(u8, u8)]) -> (u64, Vec<Vec<(u8, u8)>>) {
    let mut members = Vec::new();
    let mut cur = base.to_vec();
    for step in 1..=3_628_800u64 {
        cur = apply_hue_map(sigma, &cur).expect("a permutation of the palette maps every hue");
        if cur == base {
            return (step, members);
        }
        members.push(cur.clone());
    }
    panic!("a permutation of ten hues returned within 10! steps");
}

/// (1) THE PERIOD READ OFF THE CYCLES is the walk's, on every one of the 42
/// cycle types of the palette — and at most Landau's g(10) = 30.
#[test]
fn the_period_read_off_the_cycles_is_the_walks_on_every_cycle_type() {
    let types = partitions(10, 10);
    assert_eq!(types.len(), 42, "the partitions of ten");
    let base = palette_base();
    for parts in &types {
        let sigma = permutation_of(parts);
        let (walk, _) = walked(&sigma, &base);
        assert_eq!(orbit_period(&sigma, &base), Ok(walk), "cycle type {parts:?}");
        assert!(walk <= 30, "Landau's g(10) = 30 bounds the palette: {parts:?} at {walk}");
    }
}

/// (2) MEMBERSHIP PER SECTION, decided on the cycles, is the walk's: every
/// member of every orbit is on it, the base is not, and every other cycle
/// type's image of the base is on it exactly when the walk met it.
#[test]
fn membership_decided_on_the_cycles_is_the_walks() {
    let base = palette_base();
    let types = partitions(10, 10);
    let images: Vec<Vec<(u8, u8)>> = types
        .iter()
        .map(|p| apply_hue_map(&permutation_of(p), &base).expect("a permutation maps every hue"))
        .collect();
    for parts in &types {
        let sigma = permutation_of(parts);
        let (_, members) = walked(&sigma, &base);
        for m in &members {
            assert!(on_orbit(&sigma, &base, m), "{parts:?}: a walked member is on the orbit");
        }
        assert!(!on_orbit(&sigma, &base, &base), "{parts:?}: the base is not one of its members");
        for image in &images {
            assert_eq!(
                on_orbit(&sigma, &base, image),
                members.contains(image),
                "{parts:?}: the reading and the walk agree on {image:?}"
            );
        }
    }
}

/// (3) A HUE PAST THE PALETTE IS REFUSED BY NAME, never walked: the sensor
/// delivers none, and a map naming one has no period on any base.
#[test]
fn a_hue_past_the_palette_is_refused_by_name() {
    let sigma = vec![(1, 10), (10, 1)];
    assert_eq!(orbit_period(&sigma, &[(1, 10)]), Err(PeriodRefused::NotAPermutation));
    assert!(!on_orbit(&sigma, &[(1, 10)], &[(10, 1)]), "and nothing is on its orbit");
    // A base hue the map does not list ends the orbit at its first step.
    assert_eq!(orbit_period(&[(1, 2), (2, 1)], &[(1, 3)]), Err(PeriodRefused::OffDomain));
}

/// (4) THE PROFILE TABLE ANSWERS ASSIGNMENTS BY THEIR CHARGE, with no
/// position in the element (78.43(1)): every assignment the word speaks is
/// held under its charge, and `holds_assignment` reads it there.
#[test]
fn the_profile_table_answers_every_assignment_by_its_charge() {
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = from_canonical(&germinate(&a, &b).unwrap());
    assert!(t.puncture(vec![(2, 3), (3, 1), (1, 2)]));
    assert_eq!(t.torse(), 1, "the section condenses into the cycle");
    let spoken = t.assignments();
    assert_eq!(spoken.len(), 3, "the base and the cycle's two members");
    for asg in &spoken {
        let charged = t.assignments_by_profile(&gauge_key(asg));
        assert!(charged.contains(asg), "{asg:?} is held under its own charge");
        assert!(crate::transducer::sentence::holds_assignment(&t, asg), "and held, read by charge");
    }
    assert!(
        !crate::transducer::sentence::holds_assignment(&t, &[(5, 6), (1, 2), (2, 3)]),
        "an assignment the word does not speak is not held"
    );
}
