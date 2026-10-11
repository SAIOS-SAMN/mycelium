// NB1, 72.1b, 78.2 — the contact complex's cells are read off the base's
// elements, one per standing set, carrying the meet of their bodies' nuclei;
// no arity is written in.

use crate::transducer::base::Site;
use crate::transducer::family::pairing::named_incidence;

/// The identity's class: every fixture member is a member of it, told apart
/// by its chart (AT1: a vertex by its coordinate, a member by its chart).
fn identity_at() -> crate::network::nerve::coord::VertexAt {
    crate::transducer::sentence::Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(crate::transducer::alphabet::Word::identity()),
        crate::transducer::stalk::Invariant::default(),
        Box::new(crate::transducer::element::identity_versor()),
        Vec::new(),
        [0u64],
        None,
    )
    .at()
    .expect("the identity has a class")
}

fn member(i: u16) -> Vec<u8> {
    let chart = crate::transducer::stalk::Invariant::counts(&[u64::from(i)]);
    crate::transducer::family::member_key(&identity_at(), chart.key().as_bytes())
}

/// Face `n` of a fixture: the Frame's chart to the identity's member on
/// chart `[n]` at frame 0 — an overlap by its coordinates (AT1).
fn face(n: u64) -> crate::network::nerve::coord::Overlap {
    use crate::network::nerve::coord;
    coord::Overlap {
        from: coord::ChartAt::Frame,
        to: coord::ChartAt::at(identity_at(), crate::transducer::stalk::Invariant::counts(&[n]), 0),
    }
}

/// A traversed circuit by its steps and composite, through no held coset and
/// no static contact; a composite that did not read is unread as not rigid.
/// Its steps carry no hue maps, so a read composite's hue is the composed
/// empty map — measured, the fibre's holonomy the identity.
fn traversed(steps: Vec<(u64, bool)>, composite: Option<crate::transducer::cga::Mv>) -> crate::transducer::sentence::TraversedCircuit {
    let unread = composite.is_none().then_some(crate::transducer::cech::CircuitUnread::NotRigid);
    let hue = composite.is_some().then(Vec::new);
    let steps = steps.into_iter().map(|(n, forward)| (face(n), forward)).collect();
    crate::transducer::sentence::TraversedCircuit { steps, composite, hue, unread }
}

/// One body per member, each a single cube at `site`.
fn bodies(sites: &[Vec<i64>]) -> crate::network::nerve::Keyed<Vec<Site>> {
    let mut k = crate::network::nerve::Keyed::new();
    for (i, s) in sites.iter().enumerate() {
        k.insert(member(u16::try_from(i).expect("a fixture of a handful of bodies") + 1),vec![Site::new(s.clone()).expect("a fixture site within the algebra")]);
    }
    k
}

fn by_grade(cells: &crate::network::nerve::Keyed<crate::transducer::sentence::FormCell>, g: usize) -> Vec<crate::transducer::sentence::FormCell> {
    cells.scoped().filter(|(_, c)| c.grade() == g).map(|(_, c)| c.clone()).collect()
}

/// The one cell of a grade, read by a slice pattern.
fn one(cells: Vec<crate::transducer::sentence::FormCell>, what: &str) -> crate::transducer::sentence::FormCell {
    match cells.as_slice() {
        [c] => c.clone(),
        _ => panic!("{what}: {cells:?}"),
    }
}

/// THREE BODIES MEETING ONLY AT ONE CORNER MINT ONE 2-CELL THERE, carrying
/// `s_a ∧ s_b ∧ s_c`; a fourth at the same corner makes it one 3-cell. The
/// three cubes share pairwise only an edge each, so no pairwise read sees the
/// corner they share, and the base read does.
#[test]
fn three_bodies_meeting_at_a_corner_mint_one_two_cell_carrying_their_meet() {
    let three = bodies(&[vec![0, 0, 0], vec![1, 1, 0], vec![1, 0, 1]]);
    let (_, cells) = named_incidence(None, &three);
    let c = one(by_grade(&cells, 2), "one 2-cell");
    assert_eq!(c.witnesses, vec![Site::of([2, 2, 2])], "the corner all three stand in");
    let meet = c.meet().expect("each nucleus reads");
    assert!(!meet.grade(3).is_zero(), "the meet of three spheres is a grade-3 blade");
    assert_eq!(by_grade(&cells, 1).len(), 3, "and its three faces stand as the pairs' own cells");

    let four = bodies(&[vec![0, 0, 0], vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
    let (_, cells) = named_incidence(None, &four);
    let c = one(by_grade(&cells, 3), "one 3-cell");
    assert_eq!(c.bodies.len(), 4, "four bodies at one corner are ONE cell, not four triples");
    assert!(by_grade(&cells, 2).is_empty(), "its 2-faces are read off it, never minted as subsets");
}

/// ONE MEMBER, TWO BODIES, ONE CELL: a member whose cubes touch only along
/// an edge stands as two bodies, and with a third member's cube at their
/// shared corner the three bodies mint one 2-cell. Keyed by member keys the
/// member's two bodies collided (measured on the corpus); keyed by their
/// pieces they stand apart (BD2).
#[test]
fn two_bodies_of_one_member_stand_in_one_cell_apart() {
    let mut k = crate::network::nerve::Keyed::new();
    k.insert(member(1), vec![Site::of([0, 0, 0]), Site::of([1, 1, 0])]);
    k.insert(member(2), vec![Site::of([1, 0, 1])]);
    let (_, cells) = named_incidence(None, &k);
    let c = one(by_grade(&cells, 2), "one 2-cell over three bodies");
    assert_eq!(c.bodies.len(), 3, "three bodies, two of them one member's");
    let mut pieces = c.bodies.clone();
    pieces.dedup();
    assert_eq!(pieces.len(), 3, "three distinct pieces: the member's two bodies stand apart");
    assert!(c.meet().is_some_and(|m| !m.is_zero()), "and the cell carries its meet");
}

/// BD2 — A STEP'S BODY IS READ OFF THE FACE IT CROSSES: a circuit crossing
/// the contact face between the cell's two bodies is the cell's; one
/// crossing a face with no body pair on the carrier, or a pair outside the
/// cell, is not — whatever members it visits.
#[test]
fn a_circuit_crosses_a_cell_by_the_body_pair_of_the_face_it_steps_over() {
    use crate::transducer::cga::{self, Mv};
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let (p, r, s) = (vec![1u8], vec![2u8], vec![3u8]);
    let mut form = crate::transducer::sentence::Form::default();
    let nuclei = vec![Mv::blade(cga::E1, q(1)), Mv::blade(cga::E2, q(1))];
    let cell = crate::transducer::sentence::FormCell { bodies: vec![p.clone(), r.clone()], witnesses: Vec::new(), nuclei: Some(nuclei) };
    form.cells.insert([0u8], cell);
    form.face_bodies.put(&face(0), (p.clone(), r.clone()));
    form.face_bodies.put(&face(2), (p, s));
    let turn = cga::rotator(&q(1), &q(1));
    let circuits = vec![
        traversed(vec![(1u64, true), (0, false)], Some(turn.clone())),
        traversed(vec![(1u64, true), (3, true)], Some(turn.clone())),
        traversed(vec![(2u64, true)], Some(turn)),
    ];
    let cells = crate::transducer::family::comprehend::cell_algebras(&form, &circuits,&mut crate::physics::Physics::default().stroke_n(1 << 16));
    let [c] = cells.as_slice() else { panic!("one cell") };
    assert_eq!(c.circuits, 1, "only the circuit over face 0, between the cell's two bodies, crosses it");
    // CX1 — crossing face 0 against its orientation, the circuit leaves the
    // body at the face's `j` end first: its order of the cell's bodies.
    let [p] = c.products.as_slice() else { panic!("one product: {:?}", c.products) };
    assert_eq!(p.order, vec![1, 0], "the circuit's own order, read off the step's way");
}

/// THE PAIRWISE READING OF THE SAME THREE CARRIES NO GRADE-3 CONTENT
/// (72.1b): each body taken once per tether and revisited around the loop,
/// `V_ca V_bc V_ab = s_a s_c s_c s_b s_b s_a` is a scalar — the square the
/// cell's meet does not take.
#[test]
fn a_pairwise_loop_of_the_same_three_reads_no_grade_three_content() {
    let nucleus = |s: Vec<i64>| {
        crate::transducer::blade::shape_of(&[Site::new(s).expect("a fixture site within the algebra")]).blade.nucleus().expect("a nucleus")
    };
    let (a, b, c) = (nucleus(vec![0, 0, 0]), nucleus(vec![1, 1, 0]), nucleus(vec![1, 0, 1]));
    // `V_xy = s_y s_x`, the tether of two nuclei, around the loop a → b → c → a.
    let around = a.geo(&c).geo(&c.geo(&b)).geo(&b.geo(&a));
    for g in 1..=5u32 {
        assert!(around.grade(g).is_zero(), "the pairwise loop telescopes: grade {g} is empty");
    }
}

/// NO ARITY WRITTEN IN: at `n = 4`, four bodies standing at one element
/// mint a 3-cell with the same code.
#[test]
fn at_four_axes_four_bodies_at_one_element_mint_a_three_cell() {
    let four = bodies(&[vec![0, 0, 0, 0], vec![1, 1, 0, 0], vec![1, 0, 1, 0], vec![1, 0, 0, 1]]);
    let (_, cells) = named_incidence(None, &four);
    let c = one(by_grade(&cells, 3), "one 3-cell at n = 4");
    assert_eq!(c.witnesses, vec![Site::of([2, 2, 2, 2])], "the corner of ℤ⁴ the four share");
    assert!(!c.meet().expect("each nucleus reads").grade(4).is_zero(), "their meet is a grade-4 blade");
}

