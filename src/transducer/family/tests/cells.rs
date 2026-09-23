// NB1, 72.1b, 78.2 — the contact complex's cells are read off the base's
// elements, one per standing set, carrying the meet of their bodies' nuclei;
// no arity is written in.

use crate::transducer::base::Site;
use crate::transducer::family::pairing::named_incidence;

fn member(i: u16) -> Vec<u8> {
    crate::transducer::family::member_key(i, crate::transducer::stalk::Invariant::default().key().as_bytes())
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
/// three cubes share pairwise only an edge each, so no pair walk sees the
/// corner they share, and the base read does.
#[test]
fn three_bodies_meeting_at_a_corner_mint_one_two_cell_carrying_their_meet() {
    let three = bodies(&[vec![0, 0, 0], vec![1, 1, 0], vec![1, 0, 1]]);
    let (_, cells) = named_incidence(None, &three);
    let c = one(by_grade(&cells, 2), "one 2-cell");
    assert_eq!(c.witnesses, vec![Site::new(vec![2, 2, 2])], "the corner all three stand in");
    let meet = c.meet.expect("each nucleus reads");
    assert!(!meet.grade(3).is_zero(), "the meet of three spheres is a grade-3 blade");
    assert_eq!(by_grade(&cells, 1).len(), 3, "and its three faces stand as the pairs' own cells");

    let four = bodies(&[vec![0, 0, 0], vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
    let (_, cells) = named_incidence(None, &four);
    let c = one(by_grade(&cells, 3), "one 3-cell");
    assert_eq!(c.bodies.len(), 4, "four bodies at one corner are ONE cell, not four triples");
    assert!(by_grade(&cells, 2).is_empty(), "its 2-faces are read off it, never minted as subsets");
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
    let (_, cells) = named_incidence(None, &four);
    let c = one(by_grade(&cells, 3), "one 3-cell at n = 4");
    assert_eq!(c.witnesses, vec![Site::new(vec![2, 2, 2, 2])], "the corner of ℤ⁴ the four share");
    assert!(!c.meet.expect("each nucleus reads").grade(4).is_zero(), "their meet is a grade-4 blade");
}
