// D3.2 §C — LIFT: a frame held by its bodies' bands, at any rank.

use super::{sample, sample_n};
use crate::transducer::family::transduce::*;
use crate::transducer::sample::{Place, Sample, SampleRefusal, Symbol};

/// A BAND IS A MAXIMAL RUN CONTINUED OVER CONSECUTIVE SLICES, and the bands
/// partition the frame. A `2 × 2` block of symbol 1 in the corner of a
/// `3 × 3` ground is three bands (the block, the ground beside it, the
/// ground slice after it) and two bodies: the block, and the ground's L,
/// whose two bands meet along the boundary `2/3` of the first phase over the
/// arc `[2/3, 1)` of the second.
#[test]
fn enclosed_holds_bands_and_bodies() {
    let e = Enclosed::of(&sample(&["110", "110", "000"]));
    assert_eq!(e.enclosure, Enclosure::new(vec![3, 3]).expect("periods stand"));
    let (one, zero) = (Symbol::new(&[1]), Symbol::new(&[0]));
    assert_eq!(
        e.bands,
        vec![
            Band::new(vec![(0, 2), (0, 2)], one),
            Band::new(vec![(0, 2), (2, 3)], zero),
            Band::new(vec![(2, 3), (0, 3)], zero),
        ]
    );
    assert_eq!(e.bodies, 2);
    assert_eq!(e.body_of, vec![0, 1, 1]);
}

/// TWO RUNS OF ONE SYMBOL THAT DO NOT TOUCH ARE TWO BODIES, and a run in the
/// next slice of one symbol that overlaps it by an arc of positive length
/// joins it; a diagonal contact is not a contact (a face of grade `n − 1`
/// is). At rank 3 the same reading joins bands across any phase: two
/// bands of one symbol meeting along the second phase, in one slice of the
/// first, are one body.
#[test]
fn enclosed_joins_bands_by_face_contact() {
    let e = Enclosed::of(&sample(&["10", "01"]));
    assert_eq!(e.bands.len(), 4);
    assert_eq!(e.bodies, 4, "diagonal symbols of one value never meet");
    let t = Enclosed::of(&sample(&["111", "010"]));
    let one = Symbol::new(&[1]);
    let ones: Vec<usize> = t.bands.iter().zip(&t.body_of).filter(|(b, _)| b.symbol() == one).map(|(_, &n)| n).collect();
    assert_eq!(ones.len(), 2);
    assert_eq!(ones[0], ones[1], "the T's stem meets its bar");
    // Rank 3, extents 2 × 2 × 2: one slice of the first phase holds an L of
    // symbol 1 read as two bands; the other slice is ground.
    let l = Enclosed::of(&sample_n(&[2, 2, 2], "11100000"));
    let ls: Vec<usize> = l.bands.iter().zip(&l.body_of).filter(|(b, _)| b.symbol() == one).map(|(_, &n)| n).collect();
    assert_eq!(ls.len(), 2, "the L is two bands");
    assert_eq!(ls[0], ls[1], "and one body");
    let diagonal = Enclosed::of(&sample_n(&[2, 2, 2], "10000001"));
    assert_eq!(diagonal.bodies, 3, "two opposite corners and the ground between them");
}

/// AN ENCLOSURE WITH NO PHASE OR AN EMPTY ONE IS NONE, and a raster whose
/// slices disagree in length is refused at intake, as the sample it is not.
#[test]
fn enclosed_refuses_empty_and_ragged() {
    assert_eq!(Enclosure::new(Vec::new()), Err(TransductionRefusal::Empty));
    assert_eq!(Enclosure::new(vec![2, 0]), Err(TransductionRefusal::Empty));
    assert_eq!(
        Sample::new(vec![2, 2], 1, vec![1, 1, 1], Place::new(vec![0])),
        Err(SampleRefusal::Length { expected: 4, found: 3 })
    );
}

/// THE CONTENT IS LOSSLESS (O3): two frames that differ at one site, or in
/// one period with the same sites, write different content; the rank-2 bytes
/// are the periods, the band count, each band's spans and its symbol.
#[test]
fn enclosed_content_separates_frames() {
    let key = |slices: &[&str]| {
        let mut out = Vec::new();
        Enclosed::of(&sample(slices)).put_content(&mut out);
        out
    };
    assert_ne!(key(&["10", "00"]), key(&["01", "00"]));
    assert_ne!(key(&["1"]), key(&["11"]));
    assert_ne!(key(&["1", "1"]), key(&["11"]));
    assert_eq!(key(&["12", "30"]), key(&["12", "30"]));
    assert_eq!(key(&["1"]), vec![1, 1, 1, 0, 1, 0, 1, 1]);
}
