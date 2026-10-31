// 23.8, I36, S13.4 — a deferral on a hypha-to-hypha reply is impedance,
// never a vantage and never a refusal, and it carries the door's deficit.

use crate::transport::is_deferred;
use mycelium::anastome::dialogue;

/// The deferral a full reflex pool answers every act with is read as
/// impedance before any decoder sees it. Read as an offer reply the same
/// bytes are arm 2, a refusal by key `0000…` — the misread the guard exists
/// for. A pending outcome that carries a height is an answer.
#[test]
fn is_deferred_reads_deferral_as_impedance_not_refusal() {
    let code = mycelium::cognition::outcome_code(&mycelium::cognition::Outcome::GermPending);
    let deferral = dialogue::encode_deferral(code, &[0; 4]);
    assert!(is_deferred(&deferral), "the deferral is impedance");
    assert!(
        matches!(
            dialogue::decode_offer_reply(&deferral),
            Some(dialogue::Witnessed::Refused { vk, .. }) if vk == [0; 32]
        ),
        "decoded as an offer reply the same bytes read as a refusal by key 0000…"
    );
    let answer = dialogue::encode_outcome(code, 3, &[1, 0, 0, 0]);
    assert!(
        !is_deferred(&answer),
        "a pending outcome carrying a height is an answer, not a deferral"
    );
}

/// 23.8 (ruled 2026-09-09), S13.4 — THE DEFERRAL CARRIES THE DOOR'S
/// DEFICIT. The lanes come back through the codec's one reader of the
/// outcome layout; a deferral with lanes is still a deferral; a frame that
/// teaches, or carries a height, or is short, is not one.
#[test]
fn deferral_lanes_carries_door_deficit_per_lane() {
    let code = mycelium::cognition::outcome_code(&mycelium::cognition::Outcome::GermPending);
    let deficit = [2u64, 0, 1, 0];
    let deferral = dialogue::encode_deferral(code, &deficit);
    assert!(is_deferred(&deferral), "a deferral with lanes is a deferral");
    assert_eq!(
        dialogue::deferral_lanes(&deferral, code),
        Some(deficit),
        "the lanes are the deficit the door named"
    );
    let read = dialogue::decode_outcome_reply(&deferral).expect("the codec reads its own layout");
    assert_eq!((read.code, read.height, read.pressure), (code, 0, deficit));
    assert!(read.taught.is_empty() && read.rest.is_empty());
    let taught = dialogue::encode_outcome_reply(code, 0, &deficit, &[7]).expect("one taught byte fits");
    assert_eq!(dialogue::deferral_lanes(&taught, code), None, "a frame that teaches is an answer");
    assert_eq!(
        dialogue::deferral_lanes(&deferral[..deferral.len() - 1], code),
        None,
        "a short frame is refused, never read as zeros"
    );
}
