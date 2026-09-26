// 24.3 — THE AXIS ON MULTIPLICITY: the glue between two bodies that
// deflated one listing is the versor between their shapes, minted as a
// connection with the axis opened on it; one body opens nothing, and two
// bodies no rational versor relates are refused by name.

use crate::client::sdk::*;
use crate::transducer::blade::versor_between;
use crate::transducer::element::{from_canonical, measure_lanes, same_versor, shape_of_chain};
use crate::transducer::germinate::germinate;

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

#[test]
fn two_bodies_deflating_one_listing_open_an_axis_on_the_versor_between_them() {
    // A row of two sites and a column of two: the same body turned
    // 90°, two distinct words on the atlas.
    let row = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let column = from_canonical(&germinate(&[(0, 0, 1u8), (1, 0, 1)], &[(0, 0, 2u8), (1, 0, 2)]).unwrap());
    assert_ne!(row.address(), column.address(), "two words");
    let chain = vec![(0, 0, 1u8, 2u8), (0, 1, 1, 2), (0, 2, 1, 2)];
    let germ = Entity::axis_germ(&chain, &[row.sub().to_vec(), column.sub().to_vec()], &mut super::common::budget()).expect("two bodies glue");
    let v = versor_between(&shape_of_chain(row.sub()).unwrap(), &shape_of_chain(column.sub()).unwrap(), &mut super::common::budget())
        .expect("a 90° turn carries the row onto the column");
    assert!(germ.is_connection(), "the axis stands on a connection");
    assert!(same_versor(&germ.versor().expect("a carriage"), &v), "the carriage is the versor between the bodies");
    assert_eq!(germ.opened_axes().len(), 1, "one axis opened on it");
    assert!(germ.sub().is_empty(), "a coordinate, not a remainder");
    assert_eq!(
        germ.magnitude()[3],
        measure_lanes(&chain)[3] + 2,
        "the holonomic lane counts the words that met"
    );
    assert_ne!(germ.address(), row.address());
    assert_ne!(germ.address(), column.address());
    assert_eq!(
        Entity::axis_germ(&chain, std::slice::from_ref(&row.sub().to_vec()), &mut super::common::budget()).err(),
        Some(AXIS_ONE_BODY),
        "one body opens nothing"
    );
    let three = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1), (0, 2, 1)], &[(0, 0, 2u8), (0, 1, 2), (0, 2, 2)]).unwrap());
    assert_eq!(
        Entity::axis_germ(&chain, &[row.sub().to_vec(), three.sub().to_vec()], &mut super::common::budget()).err(),
        Some(AXIS_NO_VERSOR),
        "two sites and three: no rational versor carries one onto the other"
    );
}
