// A9 — the selection term: what it selects, what it refuses to select, and
// that its key is stable under a change of representation.

use crate::transducer::anchor::*;


fn block() -> Vec<(i64, i64, u8)> {
    (0..3).flat_map(|r| (0..3).map(move |c| (r, c, 5u8))).collect()
}

/// Interior and coboundary are DERIVED terms (K ∖ δK and δK), never tags:
/// the center of a block derives to Minus(Hue, Delta(Hue)) and re-evaluates
/// exactly on a larger scene of the same species.
#[test]
fn derive_interior_transfers_to_larger_scene() {
    let scene = block();
    let t = derive(&[(1, 1)], &scene).expect("derivable");
    assert_eq!(t.eval(&scene), vec![(1, 1)], "exact on its own evidence");
    assert!(matches!(t, Sel::Minus(_, _)), "the interior DERIVES as K ∖ δK: {t:?}");
    // Transfer: a 5×5 block of the same hue — the SAME TERM selects the
    // new interior with no re-derivation (selection is language).
    let big: Vec<(i64, i64, u8)> =
        (0..5).flat_map(|r| (0..5).map(move |c| (r, c, 5u8))).collect();
    let inner = t.eval(&big);
    assert_eq!(inner.len(), 9, "the 3×3 interior of the 5×5 block");
    assert!(inner.contains(&(2, 2)));
}

/// The boundary ring derives as δ of the hue support.
#[test]
fn derive_coboundary_as_delta() {
    let scene = block();
    let ring: Vec<(i64, i64)> =
        scene.iter().map(|(r, c, _)| (*r, *c)).filter(|&(r, c)| !(r == 1 && c == 1)).collect();
    let t = derive(&ring, &scene).expect("derivable");
    assert_eq!(t.eval(&scene).len(), 8);
    assert!(matches!(t, Sel::Delta(_)), "δK, derived: {t:?}");
}

/// The hue-interface TERM K_h ∩ star(K_o) — the relative frontier composed
/// from primitives — transfers across rearranged scenes (algebra law).
/// (Which term `derive` picks from ONE pair is honestly ambiguous — a
/// single exact evidence underdetermines; consensus across train pairs
/// disambiguates in the pipeline.)
#[test]
fn eval_hue_interface_transfers() {
    let t = Sel::Inter(Box::new(Sel::Hue(1)), Box::new(Sel::Star(Box::new(Sel::Hue(2)))));
    let scene = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 2), (0, 3, 2)];
    assert_eq!(t.eval(&scene), vec![(0, 1)], "the hue-1 site touching hue 2");
    let moved = vec![(5, 5, 1u8), (6, 5, 1), (7, 5, 2)];
    assert_eq!(t.eval(&moved), vec![(6, 5)], "the same term finds the new frontier");
    // And derive() does produce SOME exact term from the one pair.
    let d = derive(&[(0, 1)], &scene).expect("derivable");
    assert_eq!(d.eval(&scene), vec![(0, 1)], "exact on the evidence");
}

/// Serde round-trips every operator exactly; hostile bytes refuse cleanly.
#[test]
fn from_bytes_round_trips_and_refuses_hostile_bytes() {
    let t = Sel::Minus(
        Box::new(Sel::ArgExt(Box::new(Sel::Hue(3)), 0)),
        Box::new(Sel::Flood(
            Box::new(Sel::Inter(Box::new(Sel::Hue(0)), Box::new(Sel::Delta(Box::new(Sel::Hue(255)))))),
            Box::new(Sel::Transport(Box::new(Sel::Hue(0)), Box::new(Sel::Star(Box::new(Sel::Ext(Box::new(Sel::Comp(Box::new(Sel::Hue(1)), 2)), 3)))))),
        )),
    );
    let mut buf = Vec::new();
    t.to_bytes(&mut buf);
    let mut pos = 0;
    assert_eq!(Sel::from_bytes(&buf, &mut pos), Some(t), "every operator survives the wire");
    assert_eq!(pos, buf.len(), "the parse consumes exactly the term");
    // Hostile: unknown opcode, truncation, and a depth bomb all refuse.
    assert_eq!(Sel::from_bytes(&[200], &mut 0), None);
    assert_eq!(Sel::from_bytes(&buf[..buf.len() - 1], &mut 0), None);
    let bomb = vec![2u8; 64]; // Delta(Delta(Delta(... unterminated
    assert_eq!(Sel::from_bytes(&bomb, &mut 0), None, "depth cap holds");
}

/// The LARGEST component selects by order invariant — uniquely, with ties
/// selecting nothing — and the same term transfers to a rearranged scene.
#[test]
fn eval_arg_ext_selects_unique_extremum() {
    let scene = vec![(0, 0, 3u8), (0, 1, 3), (0, 2, 3), (5, 5, 3)];
    let t = Sel::ArgExt(Box::new(Sel::Hue(3)), 0);
    assert_eq!(t.eval(&scene), vec![(0, 0), (0, 1), (0, 2)], "the three-site body wins");
    let small = Sel::ArgExt(Box::new(Sel::Hue(3)), 1);
    assert_eq!(small.eval(&scene), vec![(5, 5)], "the isolated site is the argmin");
    let moved = vec![(9, 9, 3u8), (2, 0, 3), (2, 1, 3)];
    assert_eq!(t.eval(&moved), vec![(2, 0), (2, 1)], "the same term, the new largest");
    // A TIE selects nothing: two equal bodies have no unique extremum.
    let tie = vec![(0, 0, 3u8), (0, 1, 3), (5, 5, 3), (5, 6, 3)];
    assert!(t.eval(&tie).is_empty(), "no coin flips");
}

/// ENCLOSURE composes from primitives: on a full-complex scene (background
/// included), the ring's hole is Hue(0) ∖ Flood(border∩Hue(0), Hue(0)) —
/// and the derived term transfers to a different ring.
#[test]
fn derive_enclosure_transfers_to_wider_ring() {
    // A 3×3 ring of hue 5 with a hue-0 hole, on a 5×5 full grid.
    let mut scene: Vec<(i64, i64, u8)> = Vec::new();
    for r in 0..5 {
        for c in 0..5 {
            let on_ring = (1..=3).contains(&r) && (1..=3).contains(&c) && !(r == 2 && c == 2);
            scene.push((r, c, if on_ring { 5 } else { 0 }));
        }
    }
    let t = derive(&[(2, 2)], &scene).expect("the hole is derivable");
    assert_eq!(t.eval(&scene), vec![(2, 2)], "exactly the enclosed site");
    // Transfer: a wider ring elsewhere on a 7×7 grid — same term, new hole.
    let mut big: Vec<(i64, i64, u8)> = Vec::new();
    for r in 0..7 {
        for c in 0..7 {
            let on_ring = (2..=5).contains(&r) && (2..=5).contains(&c)
                && !((3..=4).contains(&r) && (3..=4).contains(&c));
            big.push((r, c, if on_ring { 5 } else { 0 }));
        }
    }
    let hole = t.eval(&big);
    assert_eq!(hole.len(), 4, "the 2×2 hole of the big ring: {hole:?}");
    assert!(hole.contains(&(3, 3)));
}

/// Extremal projection: the top row of a component derives via Ext.
#[test]
fn derive_top_row_via_ext() {
    let scene = block();
    let top: Vec<(i64, i64)> = vec![(0, 0), (0, 1), (0, 2)];
    let t = derive(&top, &scene).expect("derivable");
    assert_eq!(t.eval(&scene), top);
}
