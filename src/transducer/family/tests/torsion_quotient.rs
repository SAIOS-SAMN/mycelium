// 78.74 (as amended) — THE FIBRES OF A FRAME AND THE QUOTIENT BY TORSION
// (`family/torsion_quotient.rs`): pieces as pivots, each piece's coordinates checked against
// its own versor, relative motions read on fibres, classes merged only
// where every fibre agreed — never blindly transitive — and the certainty of
// the moving algorithm held across percepts, exact.

use crate::transducer::base::Site;
use crate::transducer::cga::{self, Mv, E1, E2};
use crate::transducer::family::torsion_quotient::{aligned, piece_of, read_pieces, take_last, vertex_point, Piece, Said};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn at(x: i64, y: i64) -> Mv {
    cga::point_at(&[q(x, 1), q(y, 1)])
}

/// The mirror across the line through the origin along (3, 4): an odd
/// versor, the grid's flip.
fn mirror() -> Mv {
    Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)))
}

fn slide() -> Mv {
    cga::translator_at(&[q(1, 1), q(0, 1)])
}

fn one() -> Mv {
    Mv::scalar(q(1, 1))
}

fn piece(frame: u64, class: &[u8], point: Mv, carriage: Mv) -> Piece {
    Piece { frame, class: class.to_vec(), point, out: None, mass: 2, out_mass: 2, carriage, n: 2 }
}

fn wide() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn said(q: &crate::transducer::family::torsion_quotient::Quotient, a: &[u8], b: &[u8]) -> Option<Said> {
    q.said.iter().find(|((x, y), _)| x.as_slice() == a && y.as_slice() == b).map(|(_, s)| s.clone())
}

/// A PIECE'S PIVOT IS ITS SITES' CENTROID IN VERTEX COORDINATES: each axis's
/// mean, one half on each grid axis, the frame axis as it stands — the same
/// point the lifted body reads (`Blade::vertex_point`). Sites of two bases,
/// or none, give no pivot.
#[test]
fn vertex_point_equals_sites_centroid() {
    let sites = [Site::of([0, 0]), Site::of([0, 1])];
    let (point, n) = vertex_point(&sites).expect("two sites of one base");
    assert_eq!(n, 2);
    assert!(point.projective_eq(&cga::point_at(&[q(1, 2), q(1, 1)])), "(0, ½) + (½, ½)");
    let framed = [Site::of([1, 2, 5])];
    let (point, _) = vertex_point(&framed).expect("one site");
    assert!(point.projective_eq(&cga::point_at(&[q(3, 2), q(5, 2), q(5, 1)])), "the frame axis takes no half");
    assert_eq!(vertex_point(&[Site::of([0, 0]), Site::of([0, 0, 1])]).map(|(_, n)| n), None, "two bases");
    assert_eq!(vertex_point(&[]).map(|(_, n)| n), None, "no sites");
    // The lifted body reads the same pivot.
    let comps: Vec<crate::transducer::blade::Compartment> =
        [(0, 0), (0, 1), (1, 1)].iter().map(|&(r, c)| crate::transducer::blade::Compartment { site: Site::of([r, c]), hue: 3 }).collect();
    let body = crate::transducer::blade::lift(&comps);
    let sites: Vec<Site> = comps.iter().map(|c| c.site.clone()).collect();
    let (from_sites, _) = vertex_point(&sites).expect("sites");
    assert!(body.vertex_point().expect("the body's pivot").projective_eq(&from_sites), "one convention, two readers");
}

/// THE SELF-CHECK: a piece's own versor carries its input pivot onto its
/// output pivot where the coordinates agree, and says so where they do not.
#[test]
fn aligned_checks_versor_carries_pivot_onto_output() {
    let domain = [Site::of([0, 0]), Site::of([0, 1])];
    let landed = [Site::of([2, 3]), Site::of([2, 4])];
    let elsewhere = [Site::of([5, 5]), Site::of([5, 6])];
    let carriage = cga::translator_at(&[q(2, 1), q(3, 1)]);
    let p = piece_of(1, b"c".to_vec(), &carriage, &domain, &landed).expect("a pivot");
    assert_eq!((p.mass, aligned(&p)), (2, Some(true)), "carried where it landed");
    let wrong = piece_of(1, b"c".to_vec(), &carriage, &domain, &elsewhere).expect("a pivot");
    assert_eq!(aligned(&wrong), Some(false), "a landing the versor does not reach is named");
    let nowhere = piece_of(1, b"c".to_vec(), &carriage, &domain, &[]).expect("a pivot");
    assert_eq!(aligned(&nowhere), None, "no landing, no reading");
}

/// THE QUOTIENT, GRADED AND NEVER BLINDLY TRANSITIVE. Four classes over two
/// frames:
/// - `a` stands; `b` is `a` flipped across the line through them — torsion on
///   both frames: certainty 2/2, merged;
/// - `c` slides — lift on both: 0/2, its own algorithm;
/// - `d` is flipped like `b` but its pivot leaves the line on the second
///   frame: against `a`, 1/2 — partial, reported and not eliminated;
/// - `b` and `d` move alike — rigid across two classes agrees: 2/2 certain.
///
/// Two certain pairs conflict through the 1/2 of `a`–`d`; THE STRONGER HOLDS
/// (78.74). `b`–`d` is bound at 4/25 + 4/20 = 9/25 (masses 2·2 over d² 25,
/// then 20), `a`–`b` at 4/25 + 4/25 = 8/25: `b`–`d` merges, and `a`–`b` is
/// the merge refused. Three algorithms from four classes; `d`, beyond `b`,
/// turns free.
#[test]
fn read_pieces_merges_only_where_every_fibre_agreed() {
    let pieces = vec![
        piece(1, b"a", at(0, 0), one()),
        piece(1, b"b", at(3, 4), mirror()),
        piece(1, b"c", at(0, 5), slide()),
        piece(1, b"d", at(6, 8), mirror()),
        piece(2, b"a", at(0, 0), one()),
        piece(2, b"b", at(3, 4), mirror()),
        piece(2, b"c", at(0, 5), slide()),
        piece(2, b"d", at(1, 0), mirror()),
    ];
    let quo = read_pieces(&pieces, &mut wide());
    let s = |a: &[u8], b: &[u8]| said(&quo, a, b).map(|s| (s.agree, s.observed));
    assert_eq!(s(b"a", b"b"), Some((2, 2)), "the flip across the pieces' line: torsion, twice");
    assert_eq!(s(b"a", b"c"), Some((0, 2)), "the slide: lift, twice");
    assert_eq!(s(b"a", b"d"), Some((1, 2)), "torsion, then lift: partial");
    assert_eq!(s(b"b", b"d"), Some((2, 2)), "moving alike: rigid agrees");
    assert_eq!(said(&quo, b"a", b"d").and_then(|s| s.certainty()), Some(BigRational::new(1.into(), 2.into())), "certainty exact");
    let bind = |a: &[u8], b: &[u8]| said(&quo, a, b).map(|s| s.bind_agree);
    assert_eq!((bind(b"a", b"b"), bind(b"b", b"d")), (Some(q(8, 25)), Some(q(9, 25))), "binding, exact");
    // a–d agrees on its far fibre (d² 100: binding 1/25) and disagrees on
    // its near one (d² 1: binding 4): counted 1/2, bound 1/101 — the
    // disagreement holds a hundred times harder, and the measure says so.
    assert_eq!(said(&quo, b"a", b"d").and_then(|s| s.binding_certainty()), Some(q(1, 101)), "the partial pair's binding certainty");
    let groups: Vec<Vec<&[u8]>> = quo.groups.iter().map(|g| g.iter().map(Vec::as_slice).collect()).collect();
    assert_eq!(groups, vec![vec![&b"a"[..]], vec![&b"b"[..], &b"d"[..]], vec![&b"c"[..]]], "the stronger b–d holds; a is not merged across a–d's disagreement");
    assert_eq!(quo.free, vec![b"d".to_vec()], "d turns free beyond b; b carries the charge");
}


/// A CLASS'S OWN SECTIONS ARE NOT TWO ALGORITHMS: two pieces of one class
/// carried by two versors make no class pair; and a stroke that ends reads
/// no cross pair, so nothing is merged and nothing is free.
#[test]
fn read_pieces_merges_nothing_for_sections_or_ended_stroke() {
    let sections = vec![piece(1, b"s", at(0, 0), one()), piece(1, b"s", at(3, 4), mirror())];
    let q = read_pieces(&sections, &mut wide());
    assert!(q.said.is_empty() && q.free.is_empty(), "one class: no pair");
    let pieces = vec![piece(1, b"a", at(0, 0), one()), piece(1, b"b", at(3, 4), mirror())];
    let mut spent = crate::physics::Physics::default().stroke_n(0);
    let q = read_pieces(&pieces, &mut spent);
    assert!(q.said.is_empty() && q.free.is_empty(), "nothing read, nothing eliminated");
    assert_eq!(q.groups.len(), 2, "two classes stand apart");
}

/// ON A REAL PERCEPT, THE QUOTIENT READS THE ALGORITHMS: one class where a
/// body stands and another slides; two, joined by lifted fibres, where a
/// body turns in place.
#[test]
fn extract_rule_quotient_reads_percept_algorithms() {
    use super::common::grid;
    let slides = vec![
        (grid(&["1100000", "0000000", "0002000", "0002200"]), grid(&["1100000", "0000000", "0000020", "0000022"])),
        (grid(&["0000110", "0000000", "0200000", "0220000"]), grid(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let flips = vec![
        (grid(&["0000", "0120", "0000"]), grid(&["0000", "0210", "0000"])),
        (grid(&["00000", "00340", "00000"]), grid(&["00000", "00430", "00000"])),
    ];
    let turns = vec![
        (grid(&["11000", "00000", "00200", "00220"]), grid(&["11000", "00000", "00220", "00200"])),
        (grid(&["00011", "00000", "20000", "22000"]), grid(&["00011", "00000", "22000", "20000"])),
    ];
    for (name, family) in [("slides", slides), ("flips", flips), ("turns", turns)] {
        let _ = take_last();
        let _ = crate::transducer::family::extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20));
        let Some(q) = take_last() else { panic!("{name}: the percept read its fibres") };
        match name {
            // The standing bar and the sliding body are ONE transport class
            // — the class quotients the translation — so the fibre between
            // them is the class's own sections, never a class pair.
            "slides" => assert!(q.said.is_empty() && q.groups.len() == 1, "slides: one algorithm: {q:?}"),
            // The standing bar and the body turned in place are two
            // classes, joined by a fibre the turn lifts — work: two
            // algorithms, nothing eliminated.
            "turns" => {
                let joined: Vec<&Said> = q.said.iter().map(|(_, s)| s).collect();
                assert!(!joined.is_empty(), "turns: the two classes are joined by fibres: {q:?}");
                assert!(joined.iter().all(|s| s.agree == 0 && s.observed >= 1), "turns: every fibre lifted: {joined:?}");
                assert!(q.free.is_empty() && q.groups.len() >= 2, "turns: nothing eliminated: {:?}", q.groups);
            }
            _ => {}
        }
    }
}

/// A section from `from` to `to` carried by `carriage`, two compartments
/// each end.
fn section(from: Mv, to: Mv, carriage: Mv) -> Piece {
    Piece { frame: 1, class: b"k".to_vec(), point: from, out: Some(to), mass: 2, out_mass: 2, carriage, n: 2 }
}

/// THE TWO SECTIONS OF A MULTIPLICITY, RELATED:
/// - congruent — two sections carried the same weighed length with the same
///   masses overlay (a copy), whichever way they point;
/// - coherent — a body standing and one flipped across the line joining
///   them are one motion up to torsion;
/// - standing — sections of different lengths whose relative motion lifts a
///   source; and a pair on one source reads no fibre, so it stands.
#[test]
fn relate_sections_tells_copy_motion_or_second_algorithm() {
    use crate::transducer::family::torsion_quotient::{relate_sections, Sections};
    let t = |x: i64, y: i64| cga::translator_at(&[q(x, 1), q(y, 1)]);
    let first = section(at(0, 0), at(3, 4), t(3, 4));
    let b = &mut wide();
    assert_eq!(relate_sections(&first, &section(at(10, 1), at(14, 4), t(4, 3)), b), Sections::Congruent, "a copy, pointing elsewhere");
    let standing = section(at(0, 0), at(0, 0), one());
    let flipped = section(at(3, 4), at(3, 4), mirror());
    assert_eq!(relate_sections(&standing, &flipped, b), Sections::Coherent, "the flip across the pieces' line: the 180° turn it unfolds to twists the fibre");
    assert_eq!(relate_sections(&first, &section(at(0, 5), at(0, 7), t(0, 2)), b), Sections::Standing, "a second algorithm");
    assert_eq!(relate_sections(&standing, &section(at(0, 0), at(0, 0), slide()), b), Sections::Standing, "one source: no fibre to read");
}

