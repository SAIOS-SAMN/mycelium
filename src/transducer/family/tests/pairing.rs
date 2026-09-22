// T4, 66 — the condensation field's tally, read at its own bounds.

use crate::transducer::family::pairing::FieldTally;

/// WHAT REACHED CONTACT IS EXACT UP TO THE BOUND, AND A CONTRADICTION PAST
/// IT REFUSES.
///
/// The field's three refusing arms are exclusive, so at most they account
/// for everything offered — at which bound nothing reached contact, and
/// that is a reading of zero. One past it is the instrument contradicting
/// itself, which reads as a refusal and never as zero (rule 11).
#[test]
fn reached_contact_is_exact_at_the_bound_and_refuses_past_it() {
    let at = FieldTally { offered: 9, unlifted: 2, apart: 3, contained: 4, ..Default::default() };
    assert_eq!(at.reached_contact(), Some(0), "every offered pair turned away: zero reached");

    let inside = FieldTally { offered: 10, ..at };
    assert_eq!(inside.reached_contact(), Some(1), "one pair more than the arms account for");

    let past = FieldTally { offered: 8, ..at };
    assert_eq!(
        past.reached_contact(),
        None,
        "arms that turned away more than was offered are a contradiction, not zero"
    );

    let whole = FieldTally { offered: u64::MAX, ..Default::default() };
    assert_eq!(whole.reached_contact(), Some(u64::MAX), "and the counter's own bound reads whole");
}
