// 13, 54.7 — THE NERVE IS DERIVED FROM THE BASE, never from the meet graph.

use crate::network::nerve::keyed::coord;
use crate::transducer::cech::{certified_triples, support_of_cover};

fn cell(row: i64, col: i64, hue: i64) -> coord::Lamina {
    coord::Lamina { row, col, hue }
}

/// THE PORE CASE — the whole reason the walk is over the base.
///
/// Three charts arranged in a ring: each meets the next, and no cell
/// stands in all three. A 3-clique of the meet graph exists — `U₀ ∩ U₁`,
/// `U₁ ∩ U₂`, `U₂ ∩ U₀` are all inhabited — so a clique search would
/// certify the triangle and fill it. The triple intersection is EMPTY, the
/// ring bounds a pore, and filling it would kill a real `H₁` class by
/// fiat. The base walk must emit nothing.
#[test]
fn three_charts_ringing_a_pore_certify_no_simplex() {
    // U₀ and U₁ share (0,1); U₁ and U₂ share (1,1); U₂ and U₀ share (1,0).
    // Nothing is shared by all three.
    let charts = vec![
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(1, 0, 0)],
        vec![cell(0, 1, 0), cell(0, 2, 0), cell(1, 1, 0)],
        vec![cell(1, 1, 0), cell(2, 0, 0), cell(1, 0, 0)],
    ];

    // Every pairwise meet is genuinely inhabited: the clique is real.
    let support = support_of_cover(&charts);
    let (cells, refused) = support.decoded();
    assert!(refused.is_empty(), "every lamina key decodes: {refused:?}");
    let pairs: Vec<Vec<usize>> = cells
        .iter()
        .filter(|(_, standing)| standing.len() == 2)
        .map(|(_, standing)| (*standing).clone())
        .collect();
    assert_eq!(pairs.len(), 3, "three pairwise overlaps stand, got {pairs:?}");

    // And yet no cell stands in three charts, so no simplex is certified.
    let (triples, refused) = certified_triples(&charts);
    assert!(refused.is_empty(), "no refusals: {refused:?}");
    assert!(
        triples.is_empty(),
        "a ring of pairwise meets around an uncovered pore is a HOLE, not a \
         2-cell; filling it would destroy an H₁ class. got {triples:?}"
    );
}

/// AND THE INHABITED CASE — one cell standing in three charts certifies
/// the simplex, and carries the witness that proves it.
#[test]
fn a_cell_standing_in_three_charts_certifies_one_simplex() {
    let shared = cell(4, 2, 3);
    let charts = vec![
        vec![cell(0, 0, 0), shared],
        vec![cell(9, 9, 1), shared],
        vec![shared, cell(7, 7, 2)],
    ];

    let (triples, refused) = certified_triples(&charts);
    assert!(refused.is_empty(), "no refusals: {refused:?}");
    assert_eq!(triples.len(), 1, "one inhabited triple, got {triples:?}");
    assert_eq!(triples[0].charts, [0, 1, 2], "the three charts that meet");
    assert_eq!(
        triples[0].witness, shared,
        "the simplex carries the cell that proves the overlap is inhabited"
    );
}

/// A SIMPLEX IS EMITTED ONCE however many cells certify it — the witness
/// is a proof, not a multiplicity.
#[test]
fn many_witnesses_certify_one_simplex() {
    let charts = vec![
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(0, 2, 0)],
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(0, 2, 0)],
        vec![cell(0, 0, 0), cell(0, 1, 0), cell(0, 2, 0)],
    ];
    let (triples, _) = certified_triples(&charts);
    assert_eq!(triples.len(), 1, "three cells certify ONE simplex, got {triples:?}");
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
