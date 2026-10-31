// 13, 54.7, NB1 — THE NERVE IS READ OFF THE BASE, never off the meet graph:
// one certified cell per standing set, at the grade its members give.

use crate::network::nerve::keyed::coord;
use crate::transducer::cech::{cells_over, read_nerve, support_of_cover, Chart};

fn compartment(row: i64, col: i64, frame: i64) -> coord::Compartment {
    coord::Compartment { site: crate::transducer::base::Site::from((row, col, frame)) }
}

/// Each chart with its own section coordinates: here, one carriage each.
fn sections(footprints: &[Vec<coord::Compartment>]) -> Vec<Chart> {
    footprints.iter().enumerate().map(|(i, f)| (vec![b's', u8::try_from(i).expect("a fixture's chart ordinal, a few charts")], f.clone())).collect()
}

/// THE PORE CASE — the whole reason the read is over the base.
///
/// Three charts arranged in a ring: each meets the next, and no element
/// stands in all three. A clique search would certify the triangle and fill
/// it; the triple intersection is EMPTY, the ring bounds a pore, and filling
/// it would kill a real `H₁` class by fiat. The base read certifies the
/// three pairwise cells and no cell of grade two.
#[test]
fn read_nerve_pore_ring_certifies_no_two_cell() {
    let charts = vec![
        vec![compartment(0, 0, 0), compartment(0, 1, 0), compartment(1, 0, 0)],
        vec![compartment(0, 1, 0), compartment(0, 2, 0), compartment(1, 1, 0)],
        vec![compartment(1, 1, 0), compartment(2, 0, 0), compartment(1, 0, 0)],
    ];
    let support = support_of_cover(&charts);
    let (cells, refused) = support.decoded();
    assert!(refused.is_empty(), "every compartment key decodes: {refused:?}");
    assert_eq!(cells.iter().filter(|(_, s)| s.len() == 2).count(), 3, "three pairwise overlaps stand");
    let read = read_nerve(&sections(&charts));
    assert!(read.refused.is_empty(), "no refusals: {:?}", read.refused);
    let grades: Vec<usize> = read.certified.iter().map(|c| c.grade()).collect();
    assert_eq!(grades, vec![1, 1, 1], "three 1-cells and no 2-cell: a ring around a pore is a HOLE");
}

/// AND THE INHABITED CASE — one element standing in three charts certifies
/// ONE 2-cell, carrying the element that proves it and the meet of the three
/// charts' nuclei.
#[test]
fn read_nerve_shared_element_certifies_one_two_cell() {
    let shared = compartment(4, 2, 3);
    let charts = vec![
        vec![compartment(0, 0, 0), shared.clone()],
        vec![compartment(9, 9, 1), shared.clone()],
        vec![shared.clone(), compartment(7, 7, 2)],
    ];
    let read = read_nerve(&sections(&charts));
    assert!(read.refused.is_empty(), "no refusals: {:?}", read.refused);
    let [c] = read.certified.as_slice() else { panic!("one standing set, one cell: {:?}", read.certified) };
    assert_eq!(c.members, vec![0, 1, 2], "the three charts that meet");
    assert_eq!(c.grade(), 2);
    assert_eq!(c.witnesses, vec![shared], "the cell carries the element that proves the overlap");
    let meet = c.meet.as_ref().expect("each chart's nucleus reads");
    assert!(!meet.grade(3).is_zero(), "the meet of three independent spheres is a grade-3 blade");
}

/// A CELL IS CERTIFIED ONCE however many elements stand under it — the
/// witnesses are its fibre, not a multiplicity.
#[test]
fn read_nerve_many_witnesses_certify_one_cell() {
    let charts = vec![
        vec![compartment(0, 0, 0), compartment(0, 1, 0), compartment(0, 2, 0)],
        vec![compartment(0, 0, 0), compartment(0, 1, 0), compartment(0, 2, 0)],
        vec![compartment(0, 0, 0), compartment(0, 1, 0), compartment(0, 2, 0)],
    ];
    let read = read_nerve(&sections(&charts));
    let [c] = read.certified.as_slice() else { panic!("one cell: {:?}", read.certified) };
    assert_eq!(c.witnesses.len(), 3, "three elements witness it");
}

/// KB1 — A CELL IS KEYED BY ITS MEMBERS' COORDINATES, NEVER THE MEMBERS' ORDINALS.
/// Two sections over one region with two carriages are two members; one
/// section offered at two ordinals is one member, so its overlap with itself
/// is no cell; a member whose coordinates do not read is counted by name and
/// its standing set mints nothing.
#[test]
fn read_nerve_keys_cells_by_member_coordinates() {
    let region = vec![compartment(0, 0, 0), compartment(0, 1, 0)];
    let two_carriages = vec![(vec![1u8], region.clone()), (vec![2u8], region.clone())];
    let read = read_nerve(&two_carriages);
    let [c] = read.certified.as_slice() else { panic!("two sections, one cell: {:?}", read.certified) };
    assert_eq!(c.coords, vec![vec![1u8], vec![2u8]], "keyed by the sections, in the sections' own order");
    let one_section_twice = vec![(vec![1u8], region.clone()), (vec![1u8], region.clone())];
    let once = read_nerve(&one_section_twice);
    assert!(once.certified.is_empty(), "one section is one member, not a cell");
    assert_eq!(once.singular, 2, "each witness whose members are one, counted (R1)");
    let at = compartment(0, 0, 0);
    let standing = [(at, vec![0usize, 1])];
    let over = cells_over(standing.iter().map(|(l, m)| (l.clone(), m.as_slice())), |m| (m == 0).then(|| vec![0u8]), |_| None);
    assert_eq!((over.cells.len(), over.unkeyed), (0, 1), "unkeyed by name, no cell minted");
}

/// THE COORDINATE IS INVERTIBLE — `x ≡ decode(encode(x))`, including on
/// negative axes, which a cover translated past its own origin produces.
#[test]
fn compartment_decode_inverts_encode() {
    use crate::network::nerve::keyed::Coordinate;
    for p in [compartment(0, 0, 0), compartment(1, 2, 3), compartment(-5, 7, -9), compartment(i64::MIN, i64::MAX, 0)] {
        let written = coord::Compartment::encode(&p);
        let round = coord::Compartment::decode(written.as_bytes()).expect("a written compartment decodes");
        assert_eq!(round, p, "a compartment is what its coordinate says it is");
    }
}
