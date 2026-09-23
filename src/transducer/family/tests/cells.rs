// NB1, 72.1b, 78.2 — the contact complex's cells are read off the base's
// elements, one per standing set, carrying the meet of their bodies' nuclei;
// no arity is written in.

use crate::transducer::base::Site;
use crate::transducer::family::pairing::named_incidence;

fn member(i: u16) -> Vec<u8> {
    crate::transducer::family::member_key(i, crate::transducer::stalk::Invariant::default().key().as_bytes())
}

/// A traversed circuit by its steps and composite, through no held coset and
/// no static contact.
fn traversed(steps: Vec<(u16, bool)>, composite: Option<crate::transducer::cga::Mv>) -> crate::transducer::sentence::TraversedCircuit {
    crate::transducer::sentence::TraversedCircuit { steps, composite, coset: None, gauge_static: false }
}

/// One body per member, each a single cube at `site`.
fn bodies(sites: &[Vec<i64>]) -> crate::network::nerve::Keyed<Vec<Site>> {
    let mut k = crate::network::nerve::Keyed::new();
    for (i, s) in sites.iter().enumerate() {
        k.insert(member(i as u16 + 1), vec![Site::new(s.clone())]);
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
    let (_, cells, _) = named_incidence(None, &three);
    let c = one(by_grade(&cells, 2), "one 2-cell");
    assert_eq!(c.witnesses, vec![Site::new(vec![2, 2, 2])], "the corner all three stand in");
    let meet = c.meet().expect("each nucleus reads");
    assert!(!meet.grade(3).is_zero(), "the meet of three spheres is a grade-3 blade");
    assert_eq!(by_grade(&cells, 1).len(), 3, "and its three faces stand as the pairs' own cells");

    let four = bodies(&[vec![0, 0, 0], vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
    let (_, cells, _) = named_incidence(None, &four);
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
    k.insert(member(1), vec![Site::new(vec![0, 0, 0]), Site::new(vec![1, 1, 0])]);
    k.insert(member(2), vec![Site::new(vec![1, 0, 1])]);
    let (_, cells, _) = named_incidence(None, &k);
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
    form.face_bodies.insert(0u32.to_be_bytes(), (p.clone(), r.clone()));
    form.face_bodies.insert(2u32.to_be_bytes(), (p, s));
    let turn = cga::rotator(&q(1), &q(1));
    let circuits = vec![
        traversed(vec![(1u16, true), (0, false)], Some(turn.clone())),
        traversed(vec![(1u16, true), (3, true)], Some(turn.clone())),
        traversed(vec![(2u16, true)], Some(turn)),
    ];
    let cells = crate::transducer::family::comprehend::cell_algebras(&form, &circuits,&mut crate::physics::Physics::default().stroke_n(1 << 16));
    let [c] = cells.as_slice() else { panic!("one cell") };
    assert_eq!(c.circuits, 1, "only the circuit over face 0, between the cell's two bodies, crosses it");
    // CX1 — crossing face 0 against its orientation, the circuit leaves the
    // body at the face's `j` end first: its order of the cell's bodies.
    let [p] = c.products.as_slice() else { panic!("one product: {:?}", c.products) };
    assert_eq!(p.order, vec![1, 0], "the circuit's own order, read off the step's way");
}

/// PG1, RC2 — THE PERCEPT'S TYPED READINGS AGREE WITH ONE ANOTHER AND ARE
/// HELD UNTIL TAKEN: the progress's cells by grade are the typed cells'
/// own, and the census's tally is read off the same cells; a second
/// percept's cells replace the first's while its spend and progress sum
/// onto them, reader by reader and grade by grade (78.16).
#[test]
fn the_percepts_readings_agree_and_are_held_until_taken() {
    use crate::transducer::cga::{self, Mv};
    use crate::transducer::family::comprehend;
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let (p, r) = (vec![1u8], vec![2u8]);
    let mut form = crate::transducer::sentence::Form::default();
    let nuclei = vec![Mv::blade(cga::E1, q(1)), Mv::blade(cga::E2, q(1))];
    let cell = crate::transducer::sentence::FormCell { bodies: vec![p.clone(), r.clone()], witnesses: Vec::new(), nuclei: Some(nuclei) };
    form.cells.insert([0u8], cell);
    form.face_bodies.insert(0u32.to_be_bytes(), (p, r));
    let circuits = vec![traversed(vec![(0u16, true)], Some(cga::rotator(&q(1), &q(1)))), traversed(vec![(0u16, false)], None)];
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    let comprehension = comprehend::comprehend(&form, &circuits, &mut stroke);
    let cells = comprehend::cells_reading(&comprehension);
    let progress = comprehend::progress(&form, &circuits, &cells);
    let [g] = progress.grades.as_slice() else { panic!("one grade: {:?}", progress.grades) };
    assert_eq!((g.grade, g.cells), (1, 1), "the progress's cells are the typed cells'");
    let kept = cells.cells.iter().filter(|c| c.round == crate::observe::CellRound::Kept).count() as u64;
    assert_eq!(g.round_kept, kept, "the round kept is the typed cells' own arm");
    assert_eq!(progress.contact_cycles, 0, "a circuit over no contact face is no contact cycle");
    assert!(super::tally::CellTally::of(&cells).line().starts_with("cells: grade 1: "), "the census reads the same cells");

    let spend = |reads| crate::observe::Spend { percepts: 1, reads, unbounded: [0; 5] };
    let _ = crate::transducer::family::take_cells();
    let _ = crate::transducer::family::take_spend();
    let _ = crate::transducer::family::take_progress();
    let _ = crate::transducer::family::take_viscosity();
    comprehend::hold(crate::observe::Cells::default(), Default::default(), spend([1, 2, 3, 4, 5]), progress.clone());
    comprehend::hold(cells.clone(), Default::default(), spend([10, 0, 0, 0, 1]), progress);
    assert_eq!(crate::transducer::family::take_cells(), Some(cells), "the last percept's cells stand");
    let held = crate::transducer::family::take_spend().expect("a spend held");
    assert_eq!((held.percepts, held.reads), (2, [11, 2, 3, 4, 6]), "spend sums reader by reader");
    let held = crate::transducer::family::take_progress().expect("a progress held");
    assert_eq!((held.percepts, held.grades[0].cells), (2, 2), "progress sums grade by grade");
    assert_eq!(crate::transducer::family::take_spend(), None, "a taken reading is no longer held");
}

/// EN1 — EACH CELL'S ENCLOSURE AGAINST THE FORM'S OTHER BODIES: the cell of
/// three bodies at `(0,0,0), (2,0,0), (0,2,0)` joins the round about
/// `(1,1,0)`, `ρ² = 2`; of the form's other bodies the small one at
/// `(1,½,0)` is inside it and the rest apart. The pair cell of that small
/// body and one at `(10,0,0)` joins the round on their diameter, about
/// `(11/2,¼,0)`: `(2,0,0)` is inside it. The two rounds cut, so neither is
/// parsed within the other; three collinear centres span no round.
#[test]
fn each_cell_reads_its_enclosure_against_the_forms_other_bodies() {
    use crate::observe::CellEnclosure;
    use crate::transducer::cga::sphere_at;
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let body = |x: i64, y: i64, r2| sphere_at(&[q(x, 1), q(y, 1), q(0, 1)], &r2);
    let cell = |pieces: &[u8], nuclei: Vec<crate::transducer::cga::Mv>| crate::transducer::sentence::FormCell {
        bodies: pieces.iter().map(|&p| vec![p]).collect(),
        witnesses: Vec::new(),
        nuclei: Some(nuclei),
    };
    let quarter = q(1, 4);
    let mut form = crate::transducer::sentence::Form::default();
    form.cells.insert([0u8], cell(&[1, 2, 3], vec![body(0, 0, quarter.clone()), body(2, 0, quarter.clone()), body(0, 2, quarter.clone())]));
    let small = sphere_at(&[q(1, 1), q(1, 2), q(0, 1)], &q(1, 100));
    form.cells.insert([1u8], cell(&[4, 5], vec![small, body(10, 0, quarter.clone())]));
    form.cells.insert([2u8], cell(&[6, 7, 8], vec![body(0, 5, quarter.clone()), body(1, 5, quarter.clone()), body(2, 5, quarter)]));
    let read = crate::transducer::family::comprehend::read_enclosures(&form);
    assert_eq!(
        read,
        vec![
            Some(CellEnclosure::Round { bodies: [1, 0, 4, 0, 0], within: None }),
            Some(CellEnclosure::Round { bodies: [1, 0, 5, 0, 0], within: None }),
            Some(CellEnclosure::Flat),
        ]
    );
}

/// THE PAIRWISE READING OF THE SAME THREE CARRIES NO GRADE-3 CONTENT
/// (72.1b): each body taken once per tether and revisited around the loop,
/// `V_ca V_bc V_ab = s_a s_c s_c s_b s_b s_a` is a scalar — the square the
/// cell's meet does not take.
#[test]
fn a_pairwise_loop_of_the_same_three_reads_no_grade_three_content() {
    let nucleus = |s: Vec<i64>| {
        crate::transducer::blade::shape_of(&[Site::new(s)]).blade.nucleus().expect("a nucleus")
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
    let (_, cells, _) = named_incidence(None, &four);
    let c = one(by_grade(&cells, 3), "one 3-cell at n = 4");
    assert_eq!(c.witnesses, vec![Site::new(vec![2, 2, 2, 2])], "the corner of ℤ⁴ the four share");
    assert!(!c.meet().expect("each nucleus reads").grade(4).is_zero(), "their meet is a grade-4 blade");
}
