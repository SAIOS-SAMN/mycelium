// MU-DERIVATIONS D4.4 (iii) — a decoded predicate is no deeper than the
// language mints, and a tape of a whole frame of nesting is refused without
// the stack it would take.

use super::super::clause::{put_pred, take_pred, Pred, PRED_HEIGHT};

fn tape(p: &Pred) -> Vec<u8> {
    let mut b = Vec::new();
    put_pred(&mut b, p);
    b
}

/// THE LANGUAGE'S DEEPEST PREDICATE READS BACK, ONE LEVEL MORE IS REFUSED:
/// a meet of `Not(Touches(Hue))` (four levels, `PRED_HEIGHT`) round-trips; a
/// fifth level is no word of the language.
#[test]
fn take_pred_refuses_past_language_depth() {
    let deepest = Pred::And(vec![Pred::Not(Box::new(Pred::Touches(Box::new(Pred::Hue(3)))))]);
    assert_eq!(PRED_HEIGHT, 4, "a meet over the vocabulary's deepest atom");
    let b = tape(&deepest);
    let mut p = 0;
    assert!(take_pred(&b, &mut p, 0).is_some() && p == b.len(), "the deepest the language mints reads back whole");
    let past = Pred::Not(Box::new(deepest));
    assert!(take_pred(&tape(&past), &mut 0, 0).is_none(), "one level past it is refused");
}

/// A WHOLE FRAME OF NESTING IS REFUSED, NEVER AN ABORT: `BODY_BUDGET` bytes of
/// the `Not` tag on a thread of the default 2 MiB stack read `None`.
#[test]
fn take_pred_survives_frame_length_nesting() {
    let refused = std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(|| {
            let not = tape(&Pred::Not(Box::new(Pred::All)))[0];
            let b = vec![not; crate::network::wire::BODY_BUDGET];
            take_pred(&b, &mut 0, 0).is_none()
        })
        .expect("spawned")
        .join()
        .expect("no abort");
    assert!(refused);
}
