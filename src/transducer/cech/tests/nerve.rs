// 13, 54.7, NB1 — THE NERVE IS READ OFF THE BASE, never off the meet graph:
// one certified cell per standing set, at the grade its members give.

use crate::network::nerve::keyed::coord;
use crate::transducer::cech::{read_nerve, support_of_cover};

fn cell(row: i64, col: i64, frame: i64) -> coord::Lamina {
    coord::Lamina { site: crate::transducer::base::Site::from((row, col, frame)) }
}

/// THE PORE CASE — the whole reason the read is over the base.
///
/// Three charts arranged in a ring: each meets the next, and no element
/// stands in all three. A clique search would certify the triangle and fill
/// it; the triple intersection is EMPTY, the ring bounds a pore, and filling
/// it would kill a real `H₁` class by fiat. The base read certifies the
/// three pairwise cells and no cell of grade two.
#[test]
fn three_charts_ringing_a_pore_certify_no_two_cell() {
    let charts = vec![
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(1, 0, 0)],
        vec![cell(0, 1, 0), cell(0, 2, 0), cell(1, 1, 0)],
        vec![cell(1, 1, 0), cell(2, 0, 0), cell(1, 0, 0)],
    ];
    let support = support_of_cover(&charts);
    let (cells, refused) = support.decoded();
    assert!(refused.is_empty(), "every lamina key decodes: {refused:?}");
    assert_eq!(cells.iter().filter(|(_, s)| s.len() == 2).count(), 3, "three pairwise overlaps stand");
    let read = read_nerve(&charts);
    assert!(read.refused.is_empty(), "no refusals: {:?}", read.refused);
    let grades: Vec<usize> = read.certified.iter().map(|c| c.grade()).collect();
    assert_eq!(grades, vec![1, 1, 1], "three 1-cells and no 2-cell: a ring around a pore is a HOLE");
}

/// AND THE INHABITED CASE — one element standing in three charts certifies
/// ONE 2-cell, carrying the element that proves it and the meet of the three
/// charts' nuclei.
#[test]
fn an_element_standing_in_three_charts_certifies_one_two_cell() {
    let shared = cell(4, 2, 3);
    let charts = vec![
        vec![cell(0, 0, 0), shared.clone()],
        vec![cell(9, 9, 1), shared.clone()],
        vec![shared.clone(), cell(7, 7, 2)],
    ];
    let read = read_nerve(&charts);
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
fn many_witnesses_certify_one_cell() {
    let charts = vec![
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(0, 2, 0)],
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(0, 2, 0)],
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(0, 2, 0)],
    ];
    let read = read_nerve(&charts);
    let [c] = read.certified.as_slice() else { panic!("one cell: {:?}", read.certified) };
    assert_eq!(c.witnesses.len(), 3, "three elements witness it");
}

/// THE COORDINATE IS INVERTIBLE — `x ≡ decode(encode(x))`, including on
/// negative axes, which a cover translated past its own origin produces.
#[test]
fn a_lamina_cell_survives_its_own_encoding() {
    use crate::network::nerve::keyed::Coordinate;
    for p in [cell(0, 0, 0), cell(1, 2, 3), cell(-5, 7, -9), cell(i64::MIN, i64::MAX, 0)] {
        let written = coord::Lamina::encode(&p);
        let round = coord::Lamina::decode(written.as_bytes()).expect("a written lamina decodes");
        assert_eq!(round, p, "a cell is what its coordinate says it is");
    }
}
