// 37.7, L14 — A LETTER IS A WORD OF ITS FORM. An adjoined generator carries
// an action and no letters; every reader that lists a form's words must
// list it, or a mind holding a letter offers it to nobody.

use super::common::*;
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::*;
use crate::transducer::stalk;

/// `words_of_element` on a generator form yields the one word it carries,
/// whose rotor is the generator's; a form whose vertex cites still yields
/// nothing for that vertex until the citation resolves.
#[test]
fn an_adjoined_generator_is_among_its_forms_words() {
    let r = order4().rotor();
    let letter = generator_form(&r, &stalk::Invariant(vec![3, 5]), &[]).expect("a rotor mints a generator");
    let words = words_of_element(&letter);
    let [w] = words.as_slice() else {
        panic!("one letter, one word, got {}", words.len())
    };
    assert!(w.rotor().projective_eq(&r), "the word carried is the action");
    assert!(w.letters().is_empty(), "and it spells no letter of the seventeen");
    // The axis index files it by that rotor, so the pivot can be offered it.
    assert!(versor_chart(w).is_some(), "it has the chart the generator index is keyed by");

    // A spelled vertex still yields its residue.
    let spelled = closed_word_form(order4(), None);
    let e = as_element_form(&spelled, [0, 0, 0, 1]).expect("a tape");
    let ws = words_of_element(&e);
    assert!(ws.iter().any(|w| w.rotor().projective_eq(&order4().rotor())));
    let _ = Word::identity();
}
