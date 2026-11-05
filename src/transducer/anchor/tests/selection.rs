// A9 — the selection term: what it selects, what it refuses to select, and
// that its key is stable under a change of representation.

use crate::transducer::anchor::*;
use crate::transducer::sentence::tests::fixtures::{painted, sites};
use crate::transducer::spelled::{PaintedSite, SectionSite};

fn block() -> Vec<PaintedSite> {
    (0..3).flat_map(|r| (0..3).map(move |c| PaintedSite::on_plane([r, c], 5))).collect()
}

/// Interior and coboundary are DERIVED terms (K ∖ δK and δK), never tags:
/// the centre of a block derives to Minus(Hue, Delta(Hue)) and re-evaluates
/// exactly on a larger scene of the same species.
#[test]
fn derive_transfers_interior_to_larger_scene() {
    let scene = block();
    let t = derive(&sites(&[(1, 1)]), &scene).expect("derivable");
    assert_eq!(t.eval(&scene), sites(&[(1, 1)]), "exact on its own evidence");
    assert!(matches!(t, Selection::Minus(_, _)), "the interior DERIVES as K ∖ δK: {t:?}");
    // Transfer: a 5×5 block of the same hue — the SAME TERM selects the
    // new interior with no re-derivation (selection is language).
    let big: Vec<PaintedSite> = (0..5).flat_map(|r| (0..5).map(move |c| PaintedSite::on_plane([r, c], 5))).collect();
    let inner = t.eval(&big);
    assert_eq!(inner.len(), 9, "the 3×3 interior of the 5×5 block");
    assert!(inner.contains(&SectionSite::on_plane([2, 2])));
}

/// The boundary ring derives as δ of the hue support.
#[test]
fn derive_reads_coboundary_as_delta() {
    let scene = block();
    let ring: Vec<SectionSite> = scene.iter().map(|p| p.site).filter(|s| s.coordinates() != [1, 1]).collect();
    let t = derive(&ring, &scene).expect("derivable");
    assert_eq!(t.eval(&scene).len(), 8);
    assert!(matches!(t, Selection::Delta(_)), "δK, derived: {t:?}");
}

/// The hue-interface TERM K_h ∩ star(K_o) — the relative frontier composed
/// from primitives — transfers across rearranged scenes (algebra law).
/// (Which term `derive` picks from ONE pair is honestly ambiguous — a
/// single exact evidence underdetermines; consensus across train pairs
/// disambiguates in the pipeline.)
#[test]
fn eval_transfers_hue_interface() {
    let t = Selection::Inter(Box::new(Selection::Hue(1)), Box::new(Selection::Star(Box::new(Selection::Hue(2)))));
    let scene = painted(&[(0, 0, 1), (0, 1, 1), (0, 2, 2), (0, 3, 2)]);
    assert_eq!(t.eval(&scene), sites(&[(0, 1)]), "the hue-1 site touching hue 2");
    let moved = painted(&[(5, 5, 1), (6, 5, 1), (7, 5, 2)]);
    assert_eq!(t.eval(&moved), sites(&[(6, 5)]), "the same term finds the new frontier");
    // And derive() does produce SOME exact term from the one pair.
    let d = derive(&sites(&[(0, 1)]), &scene).expect("derivable");
    assert_eq!(d.eval(&scene), sites(&[(0, 1)]), "exact on the evidence");
}

/// Serde round-trips every operator exactly; hostile bytes refuse cleanly.
#[test]
fn from_bytes_round_trips_and_refuses_hostile_bytes() {
    let t = Selection::Minus(
        Box::new(Selection::ArgExt(Box::new(Selection::Hue(3)), 0)),
        Box::new(Selection::Flood(
            Box::new(Selection::Inter(Box::new(Selection::Hue(0)), Box::new(Selection::Delta(Box::new(Selection::Hue(255)))))),
            Box::new(Selection::Transport(Box::new(Selection::Hue(0)), Box::new(Selection::Star(Box::new(Selection::Ext(Box::new(Selection::Comp(Box::new(Selection::Hue(1)), 2)), 3)))))),
        )),
    );
    let mut buf = Vec::new();
    t.to_bytes(&mut buf);
    let mut pos = 0;
    assert_eq!(Selection::from_bytes(&buf, &mut pos), Some(t), "every operator survives the wire");
    assert_eq!(pos, buf.len(), "the parse consumes exactly the term");
    // Hostile: unknown opcode, truncation, and a depth bomb all refuse.
    assert_eq!(Selection::from_bytes(&[200], &mut 0), None);
    assert_eq!(Selection::from_bytes(&buf[..buf.len() - 1], &mut 0), None);
    let bomb = vec![2u8; 64]; // Delta(Delta(Delta(... unterminated
    assert_eq!(Selection::from_bytes(&bomb, &mut 0), None, "depth cap holds");
}

/// The LARGEST component selects by order invariant — uniquely, with ties
/// selecting nothing — and the same term transfers to a rearranged scene.
#[test]
fn eval_arg_ext_selects_unique_extremum() {
    let scene = painted(&[(0, 0, 3), (0, 1, 3), (0, 2, 3), (5, 5, 3)]);
    let t = Selection::ArgExt(Box::new(Selection::Hue(3)), 0);
    assert_eq!(t.eval(&scene), sites(&[(0, 0), (0, 1), (0, 2)]), "the three-site body wins");
    let small = Selection::ArgExt(Box::new(Selection::Hue(3)), 1);
    assert_eq!(small.eval(&scene), sites(&[(5, 5)]), "the isolated site is the argmin");
    let moved = painted(&[(9, 9, 3), (2, 0, 3), (2, 1, 3)]);
    assert_eq!(t.eval(&moved), sites(&[(2, 0), (2, 1)]), "the same term, the new largest");
    // A TIE selects nothing: two equal bodies have no unique extremum.
    let tie = painted(&[(0, 0, 3), (0, 1, 3), (5, 5, 3), (5, 6, 3)]);
    assert!(t.eval(&tie).is_empty(), "no coin flips");
}

/// ENCLOSURE composes from primitives: on a full-complex scene (background
/// included), the ring's hole is Hue(0) ∖ Flood(border∩Hue(0), Hue(0)) —
/// and the derived term transfers to a different ring.
#[test]
fn derive_transfers_enclosure_to_wider_ring() {
    // A 3×3 ring of hue 5 with a hue-0 hole, on a 5×5 full sample.
    let mut scene: Vec<PaintedSite> = Vec::new();
    for r in 0..5 {
        for c in 0..5 {
            let on_ring = (1..=3).contains(&r) && (1..=3).contains(&c) && !(r == 2 && c == 2);
            scene.push(PaintedSite::on_plane([r, c], if on_ring { 5 } else { 0 }));
        }
    }
    let t = derive(&sites(&[(2, 2)]), &scene).expect("the hole is derivable");
    assert_eq!(t.eval(&scene), sites(&[(2, 2)]), "exactly the enclosed site");
    // Transfer: a wider ring elsewhere on a 7×7 sample — same term, new hole.
    let mut big: Vec<PaintedSite> = Vec::new();
    for r in 0..7 {
        for c in 0..7 {
            let on_ring = (2..=5).contains(&r) && (2..=5).contains(&c)
                && !((3..=4).contains(&r) && (3..=4).contains(&c));
            big.push(PaintedSite::on_plane([r, c], if on_ring { 5 } else { 0 }));
        }
    }
    let hole = t.eval(&big);
    assert_eq!(hole.len(), 4, "the 2×2 hole of the big ring: {hole:?}");
    assert!(hole.contains(&SectionSite::on_plane([3, 3])));
}

/// Extremal projection: the least line along the plane's first axis of a
/// component derives via Ext.
#[test]
fn derive_reads_top_row_via_ext() {
    let scene = block();
    let top = sites(&[(0, 0), (0, 1), (0, 2)]);
    let t = derive(&top, &scene).expect("derivable");
    assert_eq!(t.eval(&scene), top);
}
