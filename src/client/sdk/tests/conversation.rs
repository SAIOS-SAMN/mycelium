// MU-CLIENT §5.3, §5.5 — TWO MINDS FORM A SHARED WORD: one says a word, the
// other re-derives it through what it holds, the speaker reads the loop, the
// hearer holds the same loop, and the thing said is agreed language.

use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::identity_versor;
use crate::transducer::element::residual::from_connection;

/// A connection word spelled by `letters` at the identity carriage.
fn word(letters: &[(LetterId, bool)]) -> crate::transducer::element::Element {
    from_connection(&Word::from_letters(letters), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection")
}

/// Each mind takes the frames that reached its post, until the conversation
/// has nothing more in flight: the frames travel on their own connections,
/// so a stroke may find its post still empty and the next one hears them.
fn converse(minds: &mut [&mut Entity]) {
    for _ in 0..400 {
        for m in minds.iter_mut() {
            let mut b = super::common::budget();
            m.chatter_stroke(&mut b);
        }
        std::thread::park_timeout(std::time::Duration::from_millis(2));
    }
}

/// THE SHARED WORD, BY THE ALGEBRA. Both minds hold the court-taught quarter
/// turn; A says the half turn (the turn twice). B re-derives it through its
/// own generator — the turn at exponent two — and says back what its
/// language made of it; A reads the loop said → heard on every word, B
/// reads the same loop, and the thing A said is AGREED: one meaning reached
/// by two derivations in two minds.
#[test]
fn two_minds_form_a_shared_word() {
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let dir = std::env::temp_dir().join(format!("mycelium-conversation-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Entity::open("127.0.0.1:1", Kernel::named("speaker"));
    let mut b = Entity::open("127.0.0.1:1", Kernel::named("hearer"));
    let taught = word(&[(q, false)]);
    for m in [&mut a, &mut b] {
        assert!(m.absorb_taught(&taught.to_bytes().expect("the record writes")).is_some(), "the court's word is learned");
        m.open_chatter(&dir).expect("the post opens");
    }
    let said = word(&[(q, false), (q, false)]);
    a.say(&said);
    assert!(a.chat.as_ref().is_some_and(|c| c.said(said.address().as_bytes())), "A said it");
    converse(&mut [&mut a, &mut b]);
    assert!(
        a.chat.as_ref().is_some_and(|c| c.agreed(said.address().as_bytes())),
        "A holds what it said as agreed language: {:?}",
        a.chat_tally()
    );
    let _ = std::fs::remove_dir_all(&dir);
}
