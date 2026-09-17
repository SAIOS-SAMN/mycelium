// A6, A7 — the chart: read from the rotor and never from a matrix, a prefix
// of one chart is its coarser stratum, and refinement splits a stratum by
// the coordinate that arrived.

use crate::transducer::element::{from_canonical, from_connection};
use crate::transducer::germinate::germinate;
use crate::transducer::element::{self, identity_versor, Element};
use crate::transducer::stalk::*;


fn chain(cells: &[(i64, i64)]) -> element::CanonicalChain {
    let raw: Vec<Cell> = cells.iter().map(|&(r, c)| (r, c, 0u8, 1u8)).collect();
    element::canonical_chain(&raw).expect("one entry per position is A5's chain")
}

#[test]
fn chart_axes_read_from_the_rotor_not_a_matrix() {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let rot = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
    let w = from_connection(&rot, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0])
        .expect("connection");
    let inv = invariant_of(&w);
    assert!(!inv.0.is_empty(), "a connection has a chart");
    // A SOURCE-TEXT GATE STOOD HERE and it is deleted. It `include_str!`d
    // four production files and asserted their BYTES did not contain
    // "word.fiber(" or "GERM_DIM" — two spellings that exist nowhere in the
    // tree, so the assertion could only ever pass, and what it checked was
    // the text of the engine rather than the behaviour of it. That is §4.6,
    // and it is the gate disease reappearing inside the test suite after
    // eighteen gate files were deleted for being exactly this.
    //
    // The property it claimed is checked below on the object: the chart is
    // read off the rotor, so it is invariant under the motions a rotor
    // spells and it refines by the coordinate that arrived. A test that
    // greps for a forbidden identifier proves nothing about either, and it
    // goes green on a file that has been rewritten to do the wrong thing
    // under a different name.
    let q2 = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let twice = crate::transducer::alphabet::Word::from_letters(&[(q2, false), (q2, false)]);
    let half = from_connection(&twice, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0])
        .expect("connection");
    assert_ne!(
        inv.0,
        invariant_of(&half).0,
        "a quarter turn and a half turn are different charts, read from the rotor itself"
    );
}

#[test]
fn chart_prefix_is_its_coarser_stratum() {
    let coarse = Invariant(vec![1]).key();
    let fine = Invariant(vec![1, 2]).key();
    let sib = Invariant(vec![1, 3]).key();
    assert!(
        fine.as_bytes().starts_with(coarse.as_bytes()),
        "a finer chart's key starts with the coarser chart"
    );
    assert!(sib.as_bytes().starts_with(coarse.as_bytes()));
    assert!(!fine.as_bytes().starts_with(sib.as_bytes()));
    let mut k: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    k.insert(coarse.as_bytes(), 0);
    k.insert(fine.as_bytes(), 1);
    k.insert(sib.as_bytes(), 2);
    let at = crate::network::nerve::Prefix::new(coarse.as_bytes()).expect("a coarse key is a coordinate");
    let fiber: Vec<_> = k.fiber(at).map(|(key, v)| (key, *v)).collect();
    assert_eq!(fiber.len(), 3, "the coarse key's fiber is the coarser stratum");
    assert!(fiber.iter().all(|(key, _)| key.starts_with(coarse.as_bytes())));
}

/// The chart is invariant under translation, rotation, reflection, and
/// exact dilation, with no group enumeration.
#[test]
fn the_chart_is_invariant_under_the_conformal_group() {
    let l = chain(&[(0, 0), (1, 0), (2, 0), (2, 1)]);
    let base = invariant(&l);
    let moved = chain(&[(7, -3), (7, -2), (7, -1), (6, -1)]); // rotated + translated
    assert_eq!(invariant(&moved), base, "an isometry image shares the chart");
    let mirrored = chain(&[(0, 1), (1, 1), (2, 1), (2, 0)]);
    assert_eq!(invariant(&mirrored), base, "a reflection shares the chart");
    let big: Vec<Cell> = l
        .iter()
        .flat_map(|&(r, c, b, a)| {
            [(2 * r, 2 * c), (2 * r, 2 * c + 1), (2 * r + 1, 2 * c), (2 * r + 1, 2 * c + 1)]
                .into_iter()
                .map(move |(rr, cc)| (rr, cc, b, a))
        })
        .collect();
    assert_eq!(invariant(&big), base, "a dilation shares the chart");
}

/// Distinct orbits separate on structure: a square, a bar, and a ring.
#[test]
fn structure_separates_equal_volumes() {
    let square = chain(&[(0, 0), (0, 1), (1, 0), (1, 1)]);
    let bar = chain(&[(0, 0), (0, 1), (0, 2), (0, 3)]);
    let ring = chain(&[(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
    assert_ne!(invariant(&square), invariant(&bar));
    assert_ne!(invariant(&square), invariant(&ring));
    assert_eq!(&invariant(&square).0[..4], &[1, 0, 0, 4], "one body, no hole, no cavity, four turns");
    assert_eq!(&invariant(&ring).0[..4], &[1, 1, 0, 8], "one body, one hole, no cavity, eight turns");
}

/// The lookup finds a rotated sibling's base point through its chart and
/// returns the versor; an empty stratum is novelty; the atlas holds
/// addresses, so a removal walks one path and nothing shifts.
#[test]
fn the_chart_finds_the_base_point_and_the_versor() {
    let held: Vec<Element> = vec![
        from_canonical(&chain(&[(0, 0), (0, 1), (0, 2)])),         // a bar
        from_canonical(&chain(&[(0, 0), (0, 1), (1, 0)])),         // an L
        from_canonical(&chain(&[(0, 0), (0, 1), (1, 0), (1, 1)])), // a square
    ];
    let mut stalks = Stalks::over_elements(held.iter());
    assert_eq!(stalks.len(), 3);
    let at = |a: &[u8]| held.iter().find(|w| w.address().as_bytes() == a);
    let bar_rotated = from_canonical(&chain(&[(5, 5), (6, 5), (7, 5)]));
    let (a, v) = congruent(&stalks, at, &bar_rotated).expect("the bar's orbit");
    assert_eq!(a, held[0].address());
    assert!(!element::same_versor(&v, &identity_versor()), "a rotation carries the vertical bar onto the base");
    let l_reflected = from_canonical(&chain(&[(3, 3), (3, 2), (4, 3)]));
    assert_eq!(congruent(&stalks, at, &l_reflected).map(|(a, _)| a), Some(held[1].address()));
    let zigzag = from_canonical(&chain(&[(0, 0), (0, 1), (1, 1), (1, 2)]));
    assert!(stalks.fiber(&invariant_of(&zigzag)).is_empty(), "an empty stratum is novelty");
    assert_eq!(congruent(&stalks, at, &zigzag), None);
    stalks.remove(&held[1].address());
    assert_eq!(stalks.len(), 2);
    let sq = from_canonical(&chain(&[(9, 9), (9, 10), (10, 9), (10, 10)]));
    assert_eq!(congruent(&stalks, at, &sq).map(|(a, _)| a), Some(held[2].address()));
    assert_eq!(congruent(&stalks, at, &l_reflected), None);
}

/// A connection is charted on its fibres: the linear class and the
/// gauge's cycle type, never a translation and never the empty chart;
/// residual charts and connection charts are disjoint families.
#[test]
fn a_connection_is_charted_on_its_fibres_not_on_an_empty_chain() {
    let a = from_connection(&crate::transducer::alphabet::Word::identity(), &element::translator_versor(0, 2), &[(1, 2), (2, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
    let b = from_connection(&crate::transducer::alphabet::Word::identity(), &element::translator_versor(5, -3), &[(1, 2), (2, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
    let c = from_connection(&crate::transducer::alphabet::Word::identity(), &element::translator_versor(0, 2), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
    assert!(!invariant_of(&a).0.is_empty(), "a connection never lands on the empty chart");
    assert_eq!(invariant_of(&a), invariant_of(&b), "two identical motions at different origins share the chart");
    assert_ne!(invariant_of(&a), invariant_of(&c), "a different cycle type is a different chart");
    assert_eq!(invariant_of(&a).0[0], 0, "the connection family's leading axis");
    let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    assert!(invariant_of(&word).0[0] >= 1, "a residual chart leads with its component count");
    // Congruence on connections: the fibre's representative and the transition.
    let stalks = Stalks::over_elements([&a, &c].into_iter());
    let at = |x: &[u8]| [&a, &c].into_iter().find(|w| w.address().as_bytes() == x);
    assert_eq!(congruent(&stalks, at, &b).map(|(x, _)| x), Some(a.address()), "b is a's word, placed elsewhere");
    assert_eq!(congruent(&stalks, at, &c), None, "c has no other word on its chart");
}

/// Connecting the bundle: a rotated, differently-hued sibling attaches
/// to the base as a section pulled back through V, the connection records
/// (assignment, V), a second observation adds nothing, and the whole
/// bundle round-trips its bytes.
#[test]
fn the_sibling_attaches_as_a_transported_section() {
    let mut base = from_canonical(
        &germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap(),
    );
    let sibling = from_canonical(
        &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
    );
    let v = element::transition(sibling.sub(), base.sub()).expect("congruent");
    assert_eq!(base.attach(&sibling, &v), 1);
    assert_eq!(base.attach(&sibling, &v), 0, "a second observation adds nothing");
    let bytes = base.to_bytes();
    let mut pos = 0usize;
    let back = Element::from_bytes(&bytes, &mut pos).expect("round trip");
    assert_eq!(back.address(), base.address(), "the bundle round-trips");
}

/// A class refines one axis; members re-home under the new prefix.
#[test]
fn refine_splits_the_stratum_by_the_new_coordinate() {
    let a = from_canonical(&chain(&[(0, 0), (0, 1), (0, 2)]));
    let b = from_canonical(
        &element::canonical_chain(&[(0, 0, 2, 3), (0, 1, 2, 3), (0, 2, 2, 3)])
            .expect("one entry per position is A5's chain"),
    );
    let mut stalks = Stalks::over_elements([&a, &b].into_iter());
    let inv = invariant_of(&a);
    assert_eq!(stalks.fiber(&inv).len(), 2);
    let class_coord = 7u64;
    stalks.split_stratum(&inv, |addr| {
        if a.address().as_bytes() == addr {
            vec![0]
        } else {
            vec![class_coord]
        }
    });
    assert!(stalks.fiber(&inv).is_empty(), "the coarse chart is empty after the split");
    assert_eq!(stalks.fiber(&Stalks::refine_axes(&inv, &[0])), vec![a.address().into_vec()]);
    assert_eq!(stalks.fiber(&Stalks::refine_axes(&inv, &[class_coord])), vec![b.address().into_vec()]);
    let peeled = Stalks::peel(&Stalks::refine_axes(&inv, &[class_coord]));
    assert_eq!(peeled, inv, "peel drops the class axis");
    // The prefixes run coarse-last, so the tail is the root stratum.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — canonical key order: `prefixes` writes the strata from finest to coarsest, so its tail is the root"
    )]
    let coarsest = inv.prefixes().last().map(|p| p.0.is_empty());
    assert_eq!(coarsest, Some(true), "the coarsest prefix is the root stratum");
}
