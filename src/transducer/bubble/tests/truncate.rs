// 78.37(2), TC1's fix — THE TRUNCATION: a filament with no flux leaves, down
// to the unique 2-core, and the nerve's Ȟ¹ is what it was.

use super::super::*;

/// A nerve of `n` thoughts, each held with `escrow`, and `edges` between
/// them at identity: no 2-simplex, so each independent ring is one Ȟ¹.
fn nerve(n: u8, edges: &[(usize, usize)], escrow: [u64; 4]) -> (Contemplation, Vec<Vec<u8>>) {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    use crate::transducer::element::identity_versor;
    let mut c = Contemplation::new();
    let vs: Vec<Vec<u8>> = (1..=n).map(|i| vec![i; 4]).collect();
    for x in &vs {
        c.complex.set_section(x, Section::at(x.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
        c.bubbles.insert(x, Bubble { escrow, ..Bubble::default() });
    }
    for &(a, b) in edges {
        c.complex.set_edge(&vs[a], &vs[b], Rotor::identity(), identity_versor(), [1, 0, 0, 0]);
    }
    (c, vs)
}

const RING: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 3), (3, 0)];

/// A ring of four with a leaf on one corner: the leaf leaves, its escrow
/// returns to the room, its remainder goes to the cavity; the ring stands
/// and Ȟ¹ is 1 before and after.
#[test]
fn truncate_removes_leaf_and_keeps_ring() {
    let (mut c, vs) = nerve(5, &[RING.as_slice(), &[(0, 4)]].concat(), [1, 2, 3, 4]);
    assert_eq!(c.core_reading().h1, 1);
    c.truncate(std::slice::from_ref(&vs[4]));
    let r = c.core_reading();
    assert_eq!((r.free_vertices, r.core, r.h1), (0, 4, 1), "{r:?}");
    let t = c.meet_graph().truncation;
    assert_eq!((t.truncated, t.rank_moved, t.malformed), (1, 0, 0), "{t:?}");
    assert!(!c.bubbles.contains(&vs[4]), "the leaf left");
    assert_eq!(c.take_returned_escrow(), [1, 2, 3, 4], "its escrow returns");
    assert_eq!(c.take_returned_escrow(), [0; 4], "once");
    assert_eq!(c.cavity_len(), 1, "its remainder at the cavity");
}

/// A chain of two leaves hangs off a ring: the tip leaves first, then the
/// thought it hung from, each round's degrees recounted.
#[test]
fn truncate_removes_leaf_chain_round_by_round() {
    let (mut c, vs) = nerve(6, &[RING.as_slice(), &[(0, 4), (4, 5)]].concat(), [0; 4]);
    c.truncate(std::slice::from_ref(&vs[0]));
    let r = c.core_reading();
    assert_eq!((r.free_vertices, r.core, r.h1), (0, 4, 1), "{r:?}");
    assert_eq!(c.meet_graph().truncation.truncated, 2);
    assert_eq!(c.len(), 4, "the ring's four thoughts stand");
}

/// Two rings joined by a path: the path is in the 2-core and stays, a leaf on
/// it leaves; Ȟ¹ is 2 before and after.
#[test]
fn truncate_keeps_path_joining_two_rings() {
    let rings: Vec<(usize, usize)> = RING.iter().chain(&RING.map(|(a, b)| (a + 4, b + 4))).copied().collect();
    let (mut c, vs) = nerve(10, &[rings.as_slice(), &[(0, 8), (8, 4), (8, 9)]].concat(), [0; 4]);
    assert_eq!(c.core_reading().h1, 2);
    c.truncate(std::slice::from_ref(&vs[8]));
    let r = c.core_reading();
    assert_eq!((r.free_vertices, r.core, r.h1), (0, 9, 2), "{r:?}");
    assert_eq!(c.meet_graph().truncation.truncated, 1, "the leaf on the path, only");
}

/// A class on the pendant edge is 13.12's held coordinate: that leaf stands,
/// counted; a leaf something glued stands, counted; an isolated thought is
/// untouched.
#[test]
fn truncate_keeps_class_edge_and_glued_leaves() {
    use crate::transducer::alphabet::LetterId;
    let (mut c, vs) = nerve(7, &[RING.as_slice(), &[(0, 4), (1, 5)]].concat(), [0; 4]);
    let p = crate::transducer::alphabet::Word::from_letters(&[(LetterId::pythagorean_5((0, 1)).expect("a plane"), false)]).rotor();
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(
        &p,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a class reads");
    let ek = c.complex.link(&vs[4], 1)[0].clone();
    c.complex.degrees[1].get_mut(&ek).expect("the pendant edge").fill = Fill::Class(Box::new(kappa), Box::new(p));
    c.bubbles.get_mut(&vs[5]).expect("held").glued_by.insert(&vs[0], ());
    c.truncate(&[vs[0].clone(), vs[6].clone()]);
    let t = c.meet_graph().truncation;
    assert_eq!((t.truncated, t.held_by_class, t.glued), (0, 1, 1), "{t:?}");
    assert_eq!(c.len(), 7, "nothing left");
    assert_eq!(c.core_reading().isolated, 1, "the isolated thought untouched");
}

/// Truncation is scoped to the changed components, and that is the whole
/// nerve's 2-core: a leaf in another component stays until its own
/// component changes, and per-component and whole-nerve readings agree.
#[test]
fn truncate_agrees_per_component_with_whole_nerve() {
    let other: Vec<(usize, usize)> = RING.iter().map(|&(a, b)| (a + 5, b + 5)).chain([(5, 9)]).collect();
    let (mut c, vs) = nerve(10, &[RING.as_slice(), &[(0, 4)], other.as_slice()].concat(), [0; 4]);
    let whole = c.two_core();
    let mut component: Keyed<()> = Keyed::new();
    for v in &vs[..5] {
        component.insert(v, ());
    }
    let within = c.two_core_within(component);
    for v in &vs[..5] {
        assert_eq!(within.contains(v), whole.contains(v), "one component's 2-core is the whole nerve's on it");
    }
    c.truncate(std::slice::from_ref(&vs[0]));
    assert!(!c.bubbles.contains(&vs[4]) && c.bubbles.contains(&vs[9]), "only the changed component truncated");
    assert_eq!(c.core_reading().free_vertices, 1);
    c.truncate(std::slice::from_ref(&vs[5]));
    let r = c.core_reading();
    assert_eq!((r.free_vertices, r.core, r.h1), (0, 8, 2), "{r:?}");
    let core = c.two_core();
    assert!(vs.iter().filter(|v| c.bubbles.contains(v)).all(|v| core.contains(v)), "what stands is the whole 2-core");
}
