// 21 step 3, 50.2 step 2 — A VOICE AHEAD OF THE COURT WAITS. A sentence
// citing a form this court has not ruled is deferred and re-heard, never
// refused: a refusal is sealed by the voice (50.9) and the sentence would
// be lost for being one syllabus step early.

use crate::cognition::*;
use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Form, Vertex};

fn quarter(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A closed one-word sentence whose vertex carries `d`.
fn sentence_with(d: Derivation, tag: u8) -> Element {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        Address::from_bytes([TAG_FORM, tag]),
        d,
        crate::transducer::stalk::Invariant::default(),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        None,
    ));
    f.faces.push(f.face_of(0, 1).expect("star"));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    as_element_form(&f, [1, 0, 0, 0]).expect("a tape")
}

/// THE DEFERRAL, END TO END. A voice speaks a sentence citing a generator
/// this court has not ruled. The court does not refuse it — it says so,
/// dockets it under the address it waits on, and answers `SPEECH_DIALECT`,
/// which is "not heard here, speak again" and seals nothing. When the
/// cited address is ruled, the waiting sentence is heard without the voice
/// having to speak it a second time.
#[test]
fn a_sentence_citing_an_unruled_form_waits_and_is_reheard_when_it_is_ruled() {
    let mut k = Kernel::new();
    let generator = sentence_with(Derivation::spelled(quarter(1)), 1);
    let g_addr = generator.address();

    let citer = sentence_with(
        Derivation::compose(
            &[(g_addr.clone(), Address::from_bytes([TAG_FORM, 1]), 2, quarter(1))],
            Word::identity(),
        ),
        9,
    );

    // The court has ruled nothing, so the citation names an address it
    // does not hold.
    assert!(!k.ruled_contains(g_addr.as_bytes()), "the court holds nothing yet");
    assert_eq!(
        k.unruled_citation(&citer).as_ref().map(|a| a.as_bytes().to_vec()),
        Some(g_addr.as_bytes().to_vec()),
        "the court can say which address the sentence waits on"
    );

    let codes = k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&citer));
    assert_eq!(
        codes,
        vec![crate::anastome::dialogue::SPEECH_DIALECT],
        "not heard here, spoken again later — and NOT refused, which the voice would seal"
    );
    assert!(
        k.refusals.get(citer.address().as_bytes()).is_none(),
        "a wait leaves no refusal receipt: there is nothing for the voice to contract against"
    );

    // The generator is ruled. The waiting sentence is heard again, with
    // no second utterance from the voice.
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&generator)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the court hears the generator itself"
    );
    k.rule_all_provisionals();
    assert!(k.ruled_contains(g_addr.as_bytes()), "and rules it");
    assert!(
        k.unruled_citation(&citer).is_none(),
        "the citation now resolves, so the sentence no longer waits on anything"
    );
}

/// THE WAIT IS PER SENTENCE AND IT REACHES THE VOICE. One frame, two
/// sentences: the first stands and keeps its verdict, the second waits
/// and names the form it waits on. Nothing is sealed for the second — a
/// wait carries no receipt, because no act was admitted — and the voice
/// reads the cited address off the reply, which is what tells it which
/// form to utter first (chapter 6's depth order).
#[test]
fn one_frame_adjudicates_a_stood_sentence_and_a_waiting_one_apart() {
    use crate::anastome::dialogue::{self, Adjudication};
    let mut k = Kernel::new();
    let generator = sentence_with(Derivation::spelled(quarter(1)), 1);
    let g_addr = generator.address();
    let citer = sentence_with(
        Derivation::compose(
            &[(g_addr.clone(), Address::from_bytes([TAG_FORM, 1]), 2, quarter(1))],
            Word::identity(),
        ),
        9,
    );
    // A first sentence with nothing to wait on, spoken in the same frame.
    let plain = sentence_with(Derivation::spelled(quarter(3)), 5);

    let spoken = vec![(plain.clone(), None), (citer.clone(), None)];
    let heard = k.hear_speech([1; 32], &[0; 32], &spoken).expect("the frame is heard");

    // The adjudications come back in the order the sentences were spoken,
    // so index zero is the first sentence's verdict — the frame's own
    // coordinate, read as a slice pattern.
    assert!(
        matches!(heard.as_slice(), [Adjudication::Stood { .. }, ..]),
        "the sentence with nothing to wait on keeps its verdict: {heard:?}"
    );
    match heard.get(1) {
        Some(Adjudication::Awaiting { form }) => assert_eq!(
            form.as_bytes(),
            g_addr.as_bytes(),
            "and the wait names the address the voice must teach first"
        ),
        other => panic!("the citing sentence waits, and says on what: {other:?}"),
    }
    assert_eq!(heard[1].code(), dialogue::SPEECH_DIALECT, "read as: spoken again later");
    assert_eq!(heard[1].receipt(), None, "a wait admitted no act, so it has no chain position");
    assert!(
        k.refusals.get(citer.address().as_bytes()).is_none(),
        "and sealed nothing"
    );

    // The reply survives the codec: the cited address travels where a
    // verdict's receipt would be.
    let wire = dialogue::encode_speech_reply(&heard).expect("the reply fits its count");
    let back = dialogue::decode_speech_reply(&wire).expect("the reply decodes");
    assert_eq!(back, heard, "the wait and the verdict beside it both round trip");
}

/// The docket is keyed by the address waited on and by the sentence
/// inside it: a voice that speaks one sentence twice waits once, and the
/// bucket does not grow with repetition (rule 10).
#[test]
fn one_sentence_waiting_twice_is_docketed_once() {
    let mut k = Kernel::new();
    let g_addr = sentence_with(Derivation::spelled(quarter(1)), 1).address();
    let citer = sentence_with(
        Derivation::compose(
            &[(g_addr.clone(), Address::from_bytes([TAG_FORM, 1]), 2, quarter(1))],
            Word::identity(),
        ),
        9,
    );
    k.await_citation([1; 32], &[0; 32], &citer, &g_addr);
    k.await_citation([1; 32], &[0; 32], &citer, &g_addr);
    assert_eq!(
        k.awaiting_len(g_addr.as_bytes()),
        1,
        "one sentence, one wait, however many times it is spoken"
    );
}
