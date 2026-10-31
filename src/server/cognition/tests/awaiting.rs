// 21 step 3, 50.2 step 2 — A VOICE AHEAD OF THE COURT WAITS. A sentence
// citing a form this court has not ruled is deferred and re-heard, never
// refused: a refusal is sealed by the voice (50.9) and the sentence would
// be lost for being one syllabus step early.

use crate::cognition::*;
use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Form, Vertex};

fn turn_90(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A closed one-word sentence whose vertex carries `d`.
fn sentence_with(d: Derivation, tag: u8) -> Element {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        d,
        // Its own chart: an address is derived from its coordinates (54.2).
        crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        [0],
        None,
    );
    let c = f.stand_member(&v, 0).expect("a word vertex stands");
    f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c, None).expect("star");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
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
fn perceive_speech_defers_unruled_citation_until_ruled() {
    let mut k = Court::new();
    let generator = sentence_with(Derivation::spelled(turn_90(1)), 1);
    let g_addr = generator.address();

    let citer = sentence_with(
        Derivation::compose(
            &[(g_addr.clone(), Address::from_bytes([TAG_FORM, 1]), 2, turn_90(1))],
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
fn hear_speech_adjudicates_stood_and_awaiting_apart() {
    use crate::anastome::dialogue::{self, Adjudication};
    let mut k = Court::new();
    let generator = sentence_with(Derivation::spelled(turn_90(1)), 1);
    let g_addr = generator.address();
    let citer = sentence_with(
        Derivation::compose(
            &[(g_addr.clone(), Address::from_bytes([TAG_FORM, 1]), 2, turn_90(1))],
            Word::identity(),
        ),
        9,
    );
    // A first sentence with nothing to wait on, spoken in the same frame.
    let plain = sentence_with(Derivation::spelled(turn_90(3)), 5);

    let spoken = vec![(plain.clone(), None), (citer.clone(), None)];
    // 78.153: a stroke that holds the frame's replays.
    let mut frame = crate::physics::Physics::default().stroke_n(1 << 20);
    let heard = k.hear_speech([1; 32], &[0; 32], &spoken, &mut frame).expect("the frame is heard");

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

/// A RULING THAT DID NOT HAPPEN REHEARS NOTHING.
///
/// `ruled_admit` inserts into the ruled cover only where this court holds
/// the word — `word_at` answers, or the whole body is skipped — while the
/// re-hearing beneath it ran unconditionally. So an address "ruled" for a
/// word this court does not hold woke every sentence waiting on that
/// citation, and each found it still unruled and docketed again. That is
/// the backlog: `court speech awaiting` 98 → 160 → 206 over twenty
/// minutes on ca485cc, every line reading that a cited form is not ruled
/// here.
/// THE MOVE IS THAT THE DOCKET IS NOT SPENT, and it is read on a sentence
/// that would NOT re-docket. A sentence citing the address it waits on
/// simply waits again, so the carrier looks the same either way and says
/// nothing about whether an adjudication ran. One with nothing left to
/// wait on is heard, kept, and gone from the docket — so its presence
/// after the call is the reading (§3.2: the move, never the outcome).
#[test]
fn ruled_admit_absent_word_rehears_nothing() {
    let mut k = Court::new();
    // An address this court holds no word at, and a sentence with nothing
    // to wait on, docketed under it.
    let absent = sentence_with(Derivation::spelled(turn_90(1)), 1).address();
    let plain = sentence_with(Derivation::spelled(turn_90(3)), 5);
    k.await_citation([1; 32], &[0; 32], &plain, &absent);
    assert_eq!(k.awaiting_len(absent.as_bytes()), 1, "the sentence is docketed");
    assert!(
        k.word_at(absent.as_bytes()).is_none(),
        "this court holds no such word, so there is nothing to rule"
    );

    let mut ruling = k.stroke();
    k.ruled_admit(absent.as_bytes(), &mut ruling);

    assert!(
        !k.ruled_contains(absent.as_bytes()),
        "nothing was ruled — the cover does not carry it"
    );
    assert_eq!(
        k.awaiting_len(absent.as_bytes()),
        1,
        "and the docket was not spent on a ruling that did not happen"
    );

    // And the ruling that DOES happen still spends it, or the gate would
    // have closed the path instead of correcting it.
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&plain)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the court hears the sentence itself"
    );
    k.rule_all_provisionals();
    let held = plain.address();
    k.await_citation([1; 32], &[0; 32], &plain, &held);
    assert_eq!(k.awaiting_len(held.as_bytes()), 1, "docketed under an address it holds");
    let mut ruling = k.stroke();
    k.ruled_admit(held.as_bytes(), &mut ruling);
    // 78.153: a re-hearing the ruling's stroke could not replay waits again
    // and the court's strokes hear it.
    k.court_settle();
    assert_eq!(
        k.awaiting_len(held.as_bytes()),
        0,
        "a real ruling rehears, and the docket is spent"
    );
}

/// A RULED WORD IS FILED WHERE IT ACTS, so the one reader of the axis
/// index can find it.
///
/// `ruled_admit` filed at `invariant_of(w)` — the sentence's own chart,
/// `chart_of_form`, which leads with `2` and is a projection of what the
/// form holds. `candidates_for` asks the crystal under the charts an
/// orbit's face vertices act on, which are body charts leading `1`, and
/// `on_axis_of` peels rather than descending a fibre, so no peel from a
/// body chart can ever reach a form chart. Everything this court filed
/// was unreachable by the only reader of the carrier.
///
/// The claim is the coordinate: the word answers at the chart its vertex
/// acts on.
#[test]
fn rule_all_provisionals_files_word_at_vertex_chart() {
    use crate::transducer::sentence::{on_axis_of, ClassVector};
    use crate::transducer::stalk::Invariant;
    let chart = Invariant::counts(&[1, 7, 11]);
    let w = turn_90(1);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        Derivation::spelled(w.clone()),
        chart.clone(),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        [0],
        None,
    );
    let c = f.stand_member(&v, 0).expect("a word vertex stands");
    f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c, None).expect("star");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    let sentence = as_element_form(&f, [1, 0, 0, 0]).expect("a tape");

    let axes = ClassVector::of_word(&w)
        .expect("a 90° turn has a class vector")
        .coordinates();
    assert!(!axes.is_empty(), "a 90° turn occupies a coordinate");

    let mut k = Court::new();
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&sentence)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the court hears it"
    );
    k.rule_all_provisionals();
    assert!(k.ruled_contains(sentence.address().as_bytes()), "and rules it");

    for ax in &axes {
        assert!(
            !on_axis_of(&k.axis_index, &chart, *ax).is_empty(),
            "the ruled word answers at the chart its vertex acts on, on {ax:?}"
        );
    }
}

/// A sentence citing `word`'s own word vertex once — the cited form's real
/// coordinates, read off its address by the court's decoder — and turning
/// on a plane of its own beside it, so it says more than the word it cites
/// and is not merely a use of it.
fn citer_of(word: &Element, tag: u8) -> Element {
    let beyond = LetterId::turn_90((0, 3)).expect("a 90° turn on another plane of the base");
    let f = crate::transducer::sentence::from_element_form(word).expect("the cited form decodes");
    let vertex = f
        .vertices
        .entries()
        .find_map(|(_, v)| match v {
            Vertex::Word { address, .. } => Some(address.clone()),
            _ => None,
        })
        .expect("the cited form has a word vertex");
    sentence_with(Derivation::compose(&[(word.address(), vertex, 1, turn_90(1))], Word::from_letters(&[(beyond, false)])), tag)
}

/// 47.3, 76, 78.42 — A PROVISIONAL WORD'S ORBIT, PRESSURED AND CLOSED. The
/// word's admission emits its offer. A sentence waiting on it is its
/// pressure: the sentence's frame is transported to the word's escrow and the
/// word is carried again, the carriage drawing the word's own frame. The
/// witness's re-derivation, read by the word's key, converges it; the ruling
/// closes the orbit, the word leaves the offered set, and the sentence that
/// waited (D) is heard, stands, and is itself witnessed and ruled.
#[test]
fn witness_rederived_fresh_rules_pressured_word_and_citer() {
    let mut k = Court::new();
    let word = sentence_with(Derivation::spelled(turn_90(1)), 1);
    let w = word.address();
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&word)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the court hears the word"
    );
    let admitted = k.take_offers();
    assert!(admitted.iter().any(|(a, _)| *a == w), "its admission emitted its offer");
    let escrow_before = k.treasury().escrow_of(w.as_bytes()).map(|e| e.mu).expect("the word is under escrow");
    let citer = citer_of(&word, 9);
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&citer)),
        vec![crate::anastome::dialogue::SPEECH_DIALECT],
        "the citer waits on the word (D)"
    );
    // THE PRESSURE, in the algebra of the lanes: the citer's frame arrived
    // and one carriage drew the word's own, exactly — its frame read in the
    // escrow's currency, the word's discharge (78.149), which a word heard
    // for no task measures as nothing.
    let escrow_after = k.treasury().escrow_of(w.as_bytes()).map(|e| e.mu).expect("still under escrow");
    let drawn = k.discharge_of(w.as_bytes(), &word);
    let expected: [u64; 4] = std::array::from_fn(|l| escrow_before[l] + citer.magnitude()[l] - drawn[l]);
    assert_eq!(escrow_after, expected, "escrow + citer's frame − the carriage's draw, per lane");
    assert!(k.take_offers().iter().any(|(a, _)| *a == w), "and the carriage emitted the word's offer again");
    // A bonded second vantage re-derives the word; the ruling closes its orbit.
    let witness = [2u8; 32];
    k.enroll(witness, [12, 3, 0, 20]);
    k.witness_rederived_fresh(&w, witness, word.clone());
    // 78.153: a re-hearing its ruling's stroke could not replay is heard
    // again on the court's strokes.
    k.court_settle();
    assert!(k.ruled_contains(w.as_bytes()), "the word is ruled");
    assert!(!k.in_flight(w.as_bytes()), "and its orbit closed: it left the offered set");
    assert_eq!(k.awaiting_len(w.as_bytes()), 0, "the citer was heard when its word was ruled");
    // The citer, heard, stands: its own admission offered it, and a second
    // vantage rules it.
    assert!(k.take_offers().iter().any(|(a, _)| *a == citer.address()), "the citer's admission offered it");
    k.witness_rederived_fresh(&citer.address(), witness, citer.clone());
    assert!(k.ruled_contains(citer.address().as_bytes()), "D → upheld");
}

/// 78.37 (3), FX1 — A WORD NOTHING FLOWS TO FOR A WHOLE ORBIT LAPSES by
/// `ZeroFlux`. No witness answers it. Through the orbit it was admitted in
/// and the one after, the citer's pressure among them, it stands; when an
/// orbit of the court closes with nothing having flowed to it on any lane,
/// it lapses there, read by key: what escrow is left returns at the door,
/// the lapse is the court's act, the word leaves the docket, and the
/// sentence that waited on it is released as the demand it is (D), refused
/// by name and recorded.
#[test]
fn orbit_lapses_zero_flux_word_and_releases_citer() {
    let mut k = Court::new();
    let word = sentence_with(Derivation::spelled(turn_90(1)), 1);
    let w = word.address();
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&word)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    let citer = citer_of(&word, 9);
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&citer)),
        vec![crate::anastome::dialogue::SPEECH_DIALECT]
    );
    let elsewhere = Address::from_bytes([TAG_FORM, 7]);
    k.orbit();
    assert!(k.provisional.contains(w.as_bytes()), "the orbit that carried its flux keeps it");
    k.note_served(&elsewhere, [3; 32]);
    let before = k.chain().own_len();
    k.orbit();
    assert!(!k.provisional.contains(w.as_bytes()), "a whole orbit with no flux: it lapses");
    assert!(k.treasury().escrow_of(w.as_bytes()).is_none(), "its escrow returned at the door");
    assert!(!k.in_flight(w.as_bytes()), "its orbit ended");
    assert!(k.chain().own_len() > before, "and the lapse is the court's own act");
    assert_eq!(k.awaiting_len(w.as_bytes()), 0, "no sentence waits on a word that lapsed");
    let released = k.refusals.get(citer.address().as_bytes()).expect("the citer was answered, and recorded");
    assert_eq!(
        crate::court::verdict::Refusal::of_tape(crate::verdict::REFUSE_GLUED_UNRULED, &citer).kind(),
        released.kind,
        "released as the demand it is: a word this court does not hold"
    );
}

/// The docket is keyed by the address waited on and by the sentence
/// inside it: a voice that speaks one sentence twice waits once, and the
/// bucket does not grow with repetition (rule 10).
#[test]
fn await_citation_dockets_repeated_sentence_once() {
    let mut k = Court::new();
    let g_addr = sentence_with(Derivation::spelled(turn_90(1)), 1).address();
    let citer = sentence_with(
        Derivation::compose(
            &[(g_addr.clone(), Address::from_bytes([TAG_FORM, 1]), 2, turn_90(1))],
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
