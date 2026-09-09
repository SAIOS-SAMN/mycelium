// 23.8, I36 — a deferral on a hypha-to-hypha reply is impedance, never a
// vantage and never a refusal.

use crate::transport::is_deferred;
use mycelium::anastome::dialogue;

/// The deferral a full reflex pool answers every act with is read as
/// impedance before any decoder sees it. Read as an offer reply the same
/// bytes are arm 2, a refusal by key `0000…` — the misread the guard exists
/// for. A pending outcome that carries a height or a lane is an answer.
#[test]
fn a_deferral_frame_is_read_as_impedance_not_a_witness_refusal() {
    let code = mycelium::cognition::outcome_code(&mycelium::cognition::Outcome::GermPending);
    let deferral = dialogue::encode_deferral(code);
    assert!(is_deferred(&deferral), "the deferral is impedance");
    assert!(
        matches!(
            dialogue::decode_offer_reply(&deferral),
            Some(dialogue::Witnessed::Refused { vk, .. }) if vk == [0; 32]
        ),
        "decoded as an offer reply the same bytes read as a refusal by key 0000…"
    );
    let answer = dialogue::encode_outcome_reply(code, 3, &[1, 0, 0, 0], &[]);
    assert!(
        !is_deferred(&answer),
        "a pending outcome carrying a height or a lane is an answer, not a deferral"
    );
}
