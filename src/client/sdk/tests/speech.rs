// 24.3, 45.4 — WHAT THE HYPHA SAID: how a mind reads a speech reply, and
// what it does not read into one that says less than it asked about.

use crate::client::sdk::entity::utter::read_speech_reply;
use crate::anastome::dialogue;

/// A CODE THE HYPHA DID NOT SEND IS NOT AN OUTCOME. The loop read
/// `got.get(gi).copied().unwrap_or(SPEECH_STOOD)`, so a reply shorter than
/// the sentences sent — a truncated frame, or a hypha that answered about
/// fewer — was read as the hypha having STOOD every sentence it said
/// nothing about. The mind then stepped its own clock for an act nobody
/// admitted (45.4) and filed the word in `taught_up`, where 24.3's
/// "nothing is uttered twice" would keep it from ever being spoken again:
/// a word lost to a short frame. Rule 11 and checklist (i), and the value
/// it invented was the favourable one.
#[test]
fn a_code_the_hypha_did_not_send_is_not_an_outcome() {
    // Three sentences went on the wire; the reply answers about one.
    let mut codes = vec![0xff, 0xff, 0xff];
    let admitted = read_speech_reply(&mut codes, &[dialogue::SPEECH_STOOD]);
    assert_eq!(admitted, vec![0], "only the sentence it answered about is admitted");
    assert_eq!(codes[0], dialogue::SPEECH_STOOD, "and it says what the hypha said");
    assert_eq!(
        (codes[1], codes[2]),
        (dialogue::SPEECH_DIALECT, dialogue::SPEECH_DIALECT),
        "the two it said nothing about stand here and are unanswered there"
    );

    // A reply that answers about all of them admits all that were not
    // refused, and a refusal is not an act.
    let mut codes = vec![0xff, 0xff, 0xff];
    let admitted = read_speech_reply(
        &mut codes,
        &[dialogue::SPEECH_STOOD, dialogue::SPEECH_REFUSED, dialogue::SPEECH_DIALECT],
    );
    assert_eq!(admitted, vec![0, 2], "the refused sentence moved nothing");
    assert_eq!(codes, vec![
        dialogue::SPEECH_STOOD,
        dialogue::SPEECH_REFUSED,
        dialogue::SPEECH_DIALECT
    ]);

    // Slots already decided before the wire — a sentence this mind had
    // already taught — are not answered about and do not consume a code.
    let mut codes = vec![dialogue::SPEECH_STOOD, 0xff, dialogue::SPEECH_STOOD];
    let admitted = read_speech_reply(&mut codes, &[dialogue::SPEECH_REFUSED]);
    assert!(admitted.is_empty(), "the one fresh sentence was refused");
    assert_eq!(codes[1], dialogue::SPEECH_REFUSED, "and the reply lands in its own slot");

    // An empty reply admits nothing at all, rather than everything.
    let mut codes = vec![0xff, 0xff];
    assert!(
        read_speech_reply(&mut codes, &[]).is_empty(),
        "a hypha that said nothing admitted nothing"
    );
    assert!(
        codes.iter().all(|&c| c == dialogue::SPEECH_DIALECT),
        "and every sentence is unanswered, not stood"
    );
}
