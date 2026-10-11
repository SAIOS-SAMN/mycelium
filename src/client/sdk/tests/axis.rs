// 24.3 — THE AXIS INDEX: a word is filed at the chart and axes its vertex
// acts on, and leaves with the sentence that filed it.

use crate::client::sdk::*;

/// A WORD LEAVES THE AXIS INDEX WHERE IT ENTERED IT.
///
/// `stand_dialect_at` admits through `axis_admit`, which files a
/// sentence's word under every chart its vertex ACTS on — a body chart.
/// `drop_dialect` retracted through `retract_words` at
/// `invariant_of(&d.sentence)`, the SENTENCE's own chart, which leads with
/// a different axis entirely. `retract_words` returns at its first keyed
/// read on that missing outer key, so nothing was ever removed: the words
/// of every shed dialect sentence stayed in the index for the life of the
/// process, and the index is on the stroke path with no other drain.
///
/// The claim is the mirror, coordinate for coordinate: whatever the
/// admission filed, the shed removes.
#[test]
fn a_shed_dialect_word_leaves_the_axis_index_it_was_filed_in() {
        use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::element;
    use crate::transducer::sentence::{as_element_form, on_axis_of, ClassVector, Form, Vertex};
    use crate::transducer::stalk::Invariant;

    let chart = Invariant::counts(&[1, 7, 11]);
    let w = Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("the sensor plane"), false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        Derivation::spelled(w.clone()),
        chart.clone(),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        [0],
        None,
    );
    let c = f.stand_member(&v, 0).expect("a word vertex stands");
    f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c, None).expect("the star face");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    let sentence = as_element_form(&f, [1, 0, 0, 0]).expect("a tape");

    // The coordinates a 90° turn occupies are the slots it is filed
    // under; a word occupying none files nowhere and would prove nothing.
    let axes = ClassVector::of_word(&w)
        .expect("a 90° turn has a class vector")
        .coordinates();
    assert!(!axes.is_empty(), "a 90° turn occupies a coordinate");

    let mut epi = Epigenome::new();
    let addr = epi
        .stand_dialect_at(sentence, 1, 1)
        .expect("the sentence stands in the dialect");
    let (charts, words) = epi.axis_index_counts();
    assert!(charts > 0 && words > 0, "axis_index_counts reports admitted words: charts {charts} words {words}");
    for ax in &axes {
        assert!(
            !on_axis_of(&epi.axis_index, &chart, *ax).is_empty(),
            "the word is filed at the chart its vertex acts on, on {ax:?}"
        );
    }

    epi.drop_dialect(&addr);
    for ax in &axes {
        assert!(
            on_axis_of(&epi.axis_index, &chart, *ax).is_empty(),
            "and it leaves with the sentence, on {ax:?}"
        );
    }
    let (charts_after, words_after) = epi.axis_index_counts();
    assert_eq!(charts_after, 0, "empty chart is pruned from axis index, got {charts_after}");
    assert_eq!(words_after, 0, "empty word bucket is pruned from axis index, got {words_after}");
}
