// MU-DERIVATIONS D4.4 (iii) — a decoded selection term is no deeper than the
// derive's closure mints, and a tape of a whole frame of nesting is refused
// without the stack it would take.

use crate::transducer::anchor::*;

/// THE CLOSURE'S HEIGHT IS THE LANGUAGE'S: on a scene of two hues whose
/// footprint meets both, the derive's candidates reach exactly
/// `SELECTION_HEIGHT` levels, and none passes it.
#[test]
fn derive_closure_height_equals_selection_height() {
    let scene: Vec<(i64, i64, u8)> = (0..4).flat_map(|r| (0..4).map(move |c| (r, c, if c < 2 { 1 } else { 2 }))).collect();
    let fp = vec![(0, 1), (0, 2)];
    let terms = candidates(&fp, &scene);
    assert!(terms.iter().all(|t| t.height() <= SELECTION_HEIGHT), "no candidate passes the language's height");
    assert!(terms.iter().any(|t| t.height() == SELECTION_HEIGHT), "and the enclosure over an ordered base reaches it");
}

/// A WHOLE FRAME OF NESTING IS REFUSED, NEVER AN ABORT: `BODY_BUDGET` bytes of
/// one unary opcode (each byte one level deeper) parsed on a thread of the
/// default 2 MiB stack read `None`, and so does one level past the height.
#[test]
fn sel_parse_survives_frame_length_nesting() {
    let parsed = std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(|| {
            let mut deep = Vec::new();
            Selection::Delta(Box::new(Selection::Hue(1))).to_bytes(&mut deep);
            let unary = deep[0];
            let tape = vec![unary; crate::network::wire::BODY_BUDGET];
            let whole = Selection::from_bytes(&tape, &mut 0);
            let mut past = Selection::Hue(1);
            for _ in 0..SELECTION_HEIGHT {
                past = Selection::Delta(Box::new(past));
            }
            let mut bytes = Vec::new();
            past.to_bytes(&mut bytes);
            (whole, Selection::from_bytes(&bytes, &mut 0))
        })
        .expect("spawned")
        .join()
        .expect("no abort");
    assert_eq!(parsed, (None, None));
}
