// MU-SENSING §3.6 (ROADMAP track 26 S4; ruled 2026-11-06 on #32) — which
// factorings a family is invariant under: read per pair, separators at the
// fine phase of period b + 1, repeating blocks as relations, motions per
// level, and the collapse of the vortex by a reading that holds for one
// candidate alone.

use super::sample_n;
use crate::transducer::family::transduce::*;
use crate::transducer::sample::Sample;

/// The nesting of `train`'s axis `axis`.
fn axis(train: &[(Sample, Sample)], axis: usize) -> AxisNesting {
    nesting_of(train).into_iter().find(|n| n.axis == axis).expect("the axis is read")
}

/// The kept candidate `c` of an axis, if kept.
fn kept<'a>(n: &'a AxisNesting, c: &Candidate) -> Option<&'a Held> {
    n.kept.iter().find(|h| h.candidate == *c)
}

fn abutting(coarse: (usize, usize)) -> Candidate {
    Candidate { axis: 0, joint: Joint::Abutting, coarse }
}

/// Point 1: a factoring is read on each pair, input and output at their own
/// extents (`a · b → a′ · b′`); the family keeps it only where every pair
/// admits it, and a pair that refuses it holds it as that pair's residue.
#[test]
fn factoring_read_per_pair_and_kept_only_where_every_pair_admits() {
    let admitted = (sample_n(&[4], "1212"), sample_n(&[6], "121212"));
    let one = vec![admitted.clone()];
    let n = axis(&one, 0);
    let held = kept(&n, &abutting((2, 3))).expect("the one pair admits 2·2 → 3·2");
    let p = &held.pairs[0];
    assert_eq!((p.input.coarse(), p.input.fine(), p.output.coarse(), p.output.fine()), (2, 2, 3, 2), "the shape stated per pair");
    let refusing = (sample_n(&[4], "1234"), sample_n(&[6], "123456"));
    let two = vec![admitted, refusing];
    let n = axis(&two, 0);
    assert!(kept(&n, &abutting((2, 3))).is_none(), "a pair refuses it, so the family does not keep it");
    let unkept = n.residue.iter().find(|u| u.candidate == abutting((2, 3))).expect("held as residue");
    assert_eq!(unkept.refused, vec![(1, NestRefusal::Unread)], "the refusing pair, named");
}

/// Point 2: a separator is part of the fine period: blocks of `b` sites and
/// one separator after each but the last have the fine period `b + 1` and
/// the extent `a·(b + 1) − 1`, the far seam closing the last block. The
/// nested map is exact on the one carrier, and the separator's symbol is
/// read from the samples, never declared.
#[test]
fn separator_is_the_fine_phase_of_period_b_plus_one() {
    let f = Factoring::separated(8, 3, 3).expect("3 blocks of 2 sites, 2 separators");
    assert_eq!((f.joint(), f.block(), f.extent()), (Joint::Separated, 2, 8));
    for s in 0..8 {
        let (c, j) = f.nested(s).expect("a site");
        assert_eq!((c, j), (Turn::of(s / 3, 3), Turn::of(s % 3, 3)));
        assert_eq!(f.joined(c, j), Some(Turn::of(s, 8)), "read back exactly");
    }
    assert_eq!(f.joined(Turn::of(2, 3), Turn::of(2, 3)), None, "the far seam closes the last block: no separator after it");
    assert!(Factorings::of_axis(8).candidates().contains(&f), "the separated factoring is a candidate of the extent");
    assert!(Factoring::separated(9, 3, 3).is_err(), "9 is not 3·3 − 1");
    let train = vec![(sample_n(&[8], "12934956"), sample_n(&[8], "21943965"))];
    let n = axis(&train, 0);
    let c = Candidate { axis: 0, joint: Joint::Separated, coarse: (3, 3) };
    let held = kept(&n, &c).expect("kept");
    assert_eq!(held.pairs[0].separators, Some((vec![9], vec![9])), "the separator symbol is deduced on each member");
    assert!(held.readings().contains(&Reading::Separator));
    let at = n.kept.iter().position(|h| h.candidate == c).expect("kept");
    assert_eq!(n.taken, Taken::Apart { candidate: at, by: Reading::Separator }, "the separator holds for it alone");
}

/// Point 3: repeating blocks are two or more coarse blocks related by a
/// motion of the fine phase, the identity included; which blocks, by which
/// sense, is kept as content.
#[test]
fn repeating_blocks_kept_as_relation_by_fine_motion() {
    let train = vec![(sample_n(&[9], "123321123"), sample_n(&[9], "123321123"))];
    let n = axis(&train, 0);
    let held = kept(&n, &abutting((3, 3))).expect("kept");
    let relations = vec![
        BlockRelation { from: 0, to: 1, sense: Sense::Reversed },
        BlockRelation { from: 0, to: 2, sense: Sense::Kept },
        BlockRelation { from: 1, to: 2, sense: Sense::Reversed },
    ];
    assert_eq!(held.pairs[0].repeats, Some((relations.clone(), relations)), "the relations found, on each member");
}

/// Point 4: a motion per level is the product of a coarse-phase motion and a
/// fine-phase motion, each re-periodded onto its own level. Every block
/// mirrored in place is carried by (kept, reversed), which no signed
/// permutation of the whole phase carries without residue; the fine level
/// stretched onto twice its sites is carried the same way.
#[test]
fn motion_per_level_is_coarse_times_fine() {
    let (x, y) = (sample_n(&[4], "1234"), sample_n(&[4], "2143"));
    let n = axis(&[(x.clone(), y.clone())], 0);
    let held = kept(&n, &abutting((2, 2))).expect("kept");
    assert_eq!(held.pairs[0].level, vec![LevelMotion { coarse: Sense::Kept, fine: Sense::Reversed }]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 10);
    let whole = factorisations(&x, &y, &mut budget).expect("the pair factors");
    assert!(whole.held().iter().all(|f| !f.residue.is_empty()), "no whole-phase motion carries it");
    let stretched = vec![(sample_n(&[4], "1234"), sample_n(&[8], "22114433"))];
    let n = axis(&stretched, 0);
    let held = kept(&n, &abutting((2, 2))).expect("2·2 → 2·4 kept");
    assert_eq!(held.pairs[0].level, vec![LevelMotion { coarse: Sense::Kept, fine: Sense::Reversed }], "each level onto its own sites");
}

/// Point 5: candidates whose readings hold alike stay superposed and the
/// axis is not taken apart; a reading that holds for one kept candidate and
/// no other collapses the vortex to it.
#[test]
fn several_factorings_holding_alike_stay_superposed() {
    let flat = vec![(sample_n(&[6], "111111"), sample_n(&[6], "111111"))];
    let n = axis(&flat, 0);
    assert!(n.kept.len() >= 2, "several candidates kept");
    assert_eq!(n.taken, Taken::Held, "alike: none chosen, the axis one phase");
    let tiled = vec![(sample_n(&[6], "121212"), sample_n(&[6], "121212"))];
    let n = axis(&tiled, 0);
    let at = n.kept.iter().position(|h| h.candidate == abutting((3, 3))).expect("three blocks of two kept");
    assert!(kept(&n, &abutting((2, 2))).is_some_and(|h| !h.readings().contains(&Reading::Repeat)), "two blocks of three do not repeat");
    assert_eq!(n.taken, Taken::Apart { candidate: at, by: Reading::Repeat }, "the repeat holds for one alone");
}

/// THE AXIS IS TAKEN APART ONLY WHERE THE FAMILY IS INVARIANT: on a rank-2
/// family, the axis no factoring holds stays one phase, every candidate it
/// names held as residue; the axis whose blocks repeat is taken apart; and
/// the percept records the nesting as its coordinate.
#[test]
fn axis_taken_apart_only_where_family_invariant() {
    let x = sample_n(&[3, 4], "121234345656");
    let train = vec![(x.clone(), x.clone())];
    let rows = axis(&train, 0);
    assert_eq!(rows.taken, Taken::Whole, "no factoring holds: one phase");
    assert!(rows.kept.is_empty() && !rows.residue.is_empty(), "the candidates refused, held as residue");
    let cols = axis(&train, 1);
    let at = cols.kept.iter().position(|h| h.candidate == Candidate { axis: 1, joint: Joint::Abutting, coarse: (2, 2) }).expect("kept");
    assert_eq!(cols.taken, Taken::Apart { candidate: at, by: Reading::Repeat });
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let percept = TaskPercept::of(&train, &[], &mut budget).expect("perceived");
    assert_eq!(percept.nesting, nesting_of(&train), "the nesting is a coordinate of the percept");
}
