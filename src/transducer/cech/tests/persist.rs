// 13, 78.27 — THE COMPLEX'S TRIES ARE A BIJECTION, and a bad byte is a
// refusal.

use super::common::{turn_90, v, vol};
use crate::transducer::cech::{Complex, Fill};
use crate::transducer::element::identity_versor;

fn round_trip(cx: &Complex) -> Complex {
    let mut bytes = Vec::new();
    cx.put_tries(&mut bytes);
    let mut pos = 0usize;
    // `take_tries` leaves `pos` where it stopped, so a refusal names the
    // byte it refused at rather than only that it refused.
    match Complex::take_tries(&bytes, &mut pos) {
        Some(back) => {
            assert_eq!(pos, bytes.len(), "the tries are read exactly as they were written");
            back
        }
        None => panic!("a written complex must read: refused at byte {pos} of {}", bytes.len()),
    }
}

fn fills(cx: &Complex) -> Vec<String> {
    let mut out = Vec::new();
    for layer in &cx.degrees {
        for (_, s) in layer.scoped() {
            out.push(match &s.fill {
                Fill::Open => "open".to_string(),
                Fill::Filled(..) => "filled".to_string(),
                Fill::Class(..) => "class".to_string(),
                Fill::Congruent(_, _, ell) => format!("congruent:{ell}"),
                Fill::Unspellable(..) => "unspellable".to_string(),
                Fill::Unread(why) => format!("unread:{why:?}"),
            });
        }
    }
    out.sort();
    out
}

/// The empty complex first, so a failure above it is localised to the
/// tries' frames rather than to a simplex's leaves.
#[test]
fn take_tries_round_trips_empty_complex() {
    let cx = Complex::new();
    let back = round_trip(&cx);
    assert_eq!(back.degrees.len(), cx.degrees.len(), "the grading survives");
}

/// ONE edge next, so a failure is localised to a single simplex's leaves.
#[test]
fn take_tries_round_trips_one_edge() {
    let mut cx = Complex::new();
    cx.set_edge(v(1), v(2), turn_90(), identity_versor(), vol());
    let before = fills(&cx);
    let back = round_trip(&cx);
    assert_eq!(fills(&back), before, "the stored fill returns as itself");
}

/// `x ≡ take(put(x))` on a complex carrying real simplices — the stored
/// states and the states' payloads both survive, and each simplex is filed under
/// its own key.
#[test]
fn take_tries_round_trips_complex() {
    let mut cx = Complex::new();
    cx.set_edge(v(1), v(2), turn_90(), identity_versor(), vol());
    cx.set_edge(v(2), v(3), turn_90(), identity_versor(), vol());
    let before = fills(&cx);
    assert!(!before.is_empty(), "the fixture carries simplices to test");

    let back = round_trip(&cx);
    assert_eq!(fills(&back), before, "every stored fill returns as itself");
    assert_eq!(back.degrees.len(), cx.degrees.len(), "the grading survives");
    assert_eq!(back.sections.len(), cx.sections.len(), "the sections survive");
    let (mut again, mut before) = (Vec::new(), Vec::new());
    back.put_tries(&mut again);
    cx.put_tries(&mut before);
    assert_eq!(again, before, "every simplex is filed back under its own key: the tries re-write byte for byte");
}

/// 78.26(5) — AN UNMEASURED FILL RETURNS BY ITS REASON: never as an open
/// cell, never as another reason.
#[test]
fn take_tries_round_trips_unread_fill_reason() {
    for why in crate::transducer::cech::FillUnread::ALL {
        let mut cx = Complex::new();
        cx.set_edge(v(1), v(2), turn_90(), identity_versor(), vol());
        let keys: Vec<Vec<u8>> = cx.degrees[1].scoped().map(|(k, _)| k).collect();
        for k in &keys {
            if let Some(s) = cx.degrees[1].get_mut(k) {
                s.fill = Fill::Unread(why);
            }
        }
        let before = fills(&cx);
        assert!(before.contains(&format!("unread:{why:?}")), "the fixture carries the reason");
        assert_eq!(fills(&round_trip(&cx)), before, "returned by its reason");
    }
}

/// AN UNKNOWN FILL TAG IS A REFUSAL, NOT AN OPEN CELL.
///
/// A decoder that ended `_ => Fill::Open` let a truncated or bit-flipped
/// byte FABRICATE an unfilled simplex — and an open 2-cell is a 1-hole, so a
/// bad read invented homology the corpus does not have. Every byte of a
/// valid stream is flipped to an out-of-range tag in turn; not one of them
/// may yield a complex that claims more open cells than the original wrote.
#[test]
fn take_tries_fabricates_no_open_cell_from_corrupt_byte() {
    let mut cx = Complex::new();
    cx.set_edge(v(1), v(2), turn_90(), identity_versor(), vol());
    let mut bytes = Vec::new();
    cx.put_tries(&mut bytes);
    let honest = fills(&cx).iter().filter(|f| *f == "open").count();
    // THE BASELINE, or this test is vacuous: if the reader refused every
    // stream it would report zero fabrications while measuring nothing.
    assert!(honest > 0, "the fixture writes an open cell to be fabricated");
    round_trip(&cx);

    let mut fabricated = 0usize;
    let mut read = 0usize;
    for i in 0..bytes.len() {
        let mut corrupt = bytes.clone();
        // 9 is outside the allocated tag range 0..=5 wherever a fill tag
        // stands, and nonsense wherever one does not.
        corrupt[i] = 9;
        let mut pos = 0usize;
        if let Some(back) = Complex::take_tries(&corrupt, &mut pos) {
            read += 1;
            if fills(&back).iter().filter(|f| *f == "open").count() > honest {
                fabricated += 1;
            }
        }
    }
    assert_eq!(
        fabricated, 0,
        "a corrupt byte must refuse, never synthesise an open boundary \
         the writer did not write"
    );
    // Some corruptions land on bytes the reader does not constrain — a
    // coordinate inside a ray, a volume lane — so a few streams legitimately
    // still read. Reporting that count keeps the assertion above from
    // passing merely because everything refused.
    assert!(
        read > 0,
        "not one corrupted stream read at all, so the assertion above \
         measured nothing"
    );
}
