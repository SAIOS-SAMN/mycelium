// 22 — THE ORBIT: where a vertex's versor carries a body, and when there
// is no orbit at all.

use crate::transducer::family::*;

/// A WORD THAT MOVES THE BODY moves it; a still word does not; a contact
/// word with a placement moves it — read by one exact application (78.62).
#[test]
fn a_vertex_moves_the_body_or_stands_still() {
let zero = q(0);
let one = q(1);
let word = |carriage: element::Versor, contact| {
    Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(crate::transducer::alphabet::Word::identity()),
        Invariant::default(),
        Box::new(carriage),
        Vec::new(),
        [0u64],
        contact,
    )
};
let painted = vec![(0i64, 0i64, 1u8)];
assert!(moves(&word(element::carriage(&one, &(zero.clone(), one.clone())), None), &painted), "a placement moves the body");
assert!(!moves(&word(element::carriage(&one, &(zero.clone(), zero.clone())), None), &painted), "a still word has no orbit");
let contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 0 });
assert!(moves(&word(element::carriage(&q(1), &(q(0), q(1))), contact), &painted), "a contact word must move the body");
assert!(!moves(&word(element::carriage(&one, &(zero, one.clone())), None), &[]), "an empty body has nothing to move");
}
