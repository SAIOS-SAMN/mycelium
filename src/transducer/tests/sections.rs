// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// RULED 2026-11-06 ON #32 (4) — LETTERS PAST n = 3 ARE NEVER WRITTEN BY HAND:
// the seventeen stand on one chart; past the alphabet's section a sample is
// read through its section samples; a new letter is adjoined only by the
// presentation's growth.

use crate::transducer::alphabet::{alphabet_admits, letter, plane_of, slot_of_axis, LetterId, Rotor, CAYLEY_ALPHABET_LEN, LETTER_RANK, ALPHABET_SAMPLE_RANK};
use crate::transducer::sample::{Place, Sample};
use crate::transducer::sections::sections_of;

/// A rank-3 sample of extents 2 × 3 × 2 whose symbol at `(i, j, k)` is
/// `6i + 2j + k`, every site its own symbol.
fn deep() -> Sample {
    Sample::new(vec![2, 3, 2], 1, (0u8..12).collect(), Place::new(vec![0, 1, 0])).expect("a sample")
}

/// THE LETTERS STAND ON ONE CHART, AND A LARGER RANK IS READ THROUGH SECTION
/// SAMPLES. Every one of the seventeen is a rotor on a plane of the chart
/// of `LETTER_RANK` slots, and no slot stands for an axis past the base, so
/// no letter is written for a plane a larger rank opens. A rank-3 sample is
/// refused by the alphabet and read through its sections: one per plane of
/// two of its axes and per coordinate of the third, each a sample the
/// alphabet admits, holding the slice's symbols exactly. A new letter enters
/// only as an adjoined generator of the presentation, never as a slot of
/// the seventeen.
#[test]
fn letters_past_the_section_are_never_written_by_hand() {
    let ids: Vec<LetterId> = LetterId::all().collect();
    assert_eq!(ids.len(), CAYLEY_ALPHABET_LEN, "the seventeen, and no letter more");
    for id in &ids {
        if let Some((i, j)) = plane_of(*id) {
            assert!(usize::from(i) < LETTER_RANK && usize::from(j) < LETTER_RANK, "{id:?} stands on the letters' chart");
        }
        assert!(letter(*id).is_versor(), "{id:?} is an exact rotor of the chart");
    }
    for axis in ALPHABET_SAMPLE_RANK + 1..2 * LETTER_RANK {
        assert_eq!(slot_of_axis(axis), None, "axis {axis} past the base has no slot: no letter is written for its planes");
    }
    let s = deep();
    assert!(alphabet_admits(s.rank()).is_err(), "rank 3 is past the alphabet's section");
    let sections = sections_of(&s).expect("read through its section samples");
    assert_eq!(sections.len(), 2 + 3 + 2, "plane (0,1) at each of 2, (0,2) at each of 3, (1,2) at each of 2");
    for sec in &sections {
        assert!(alphabet_admits(sec.sample.sample().rank()).is_ok(), "each section is a sample the alphabet admits");
        assert_eq!(sec.sample.sample().place(), s.place(), "at the sample's place");
    }
    let first = &sections[0];
    assert_eq!((first.plane, first.at.clone()), ([0, 1], vec![(2, 0)]), "the first section: plane (0, 1) at axis 2's coordinate 0");
    assert_eq!(first.sample.symbols(), &[0, 2, 4, 6, 8, 10], "the slice's symbols exactly");
    let last = &sections[sections.len() - 1];
    assert_eq!((last.plane, last.at.clone()), ([1, 2], vec![(0, 1)]));
    assert_eq!(last.sample.symbols(), &[6, 7, 8, 9, 10, 11]);
    let adjoined = crate::transducer::alphabet::Derivation::adjoined(Rotor::scalar(crate::core::Q::from_integer(1.into())));
    assert!(adjoined.is_adjoined(), "a new letter is an adjoined generator of the presentation");
    assert_eq!(LetterId::all().count(), CAYLEY_ALPHABET_LEN, "and the seventeen are unchanged by it");
}
