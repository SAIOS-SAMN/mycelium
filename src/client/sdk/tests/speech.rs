// 24.3, 45.4 — WHAT THE HYPHA SAID: how a mind reads a speech reply, what
// it does not read into one that says less than it asked about, and where
// it looks to know whether it has spoken a vertex before.

use crate::client::sdk::entity::utter::{read_speech_reply, vertex_stratum};
use crate::anastome::dialogue;

/// 50.6 seam 1 — A WITHHELD REMAINDER IS NEVER A WORD: neither body of a
/// pair remainder stands in the dialect, none is filed as an utterance,
/// and the chain of both never stands; the withheld form itself stands
/// and is held as an idea. (Before this each body stood and was attested
/// as a word of its own — the residual utterance the soak of 2026-09-09
/// read as garbage.)
#[test]
fn a_withheld_remainder_is_never_a_word() {
    use super::common::closed_turn;
    use crate::client::sdk::*;
    use crate::transducer::element::{canonical_chain, from_canonical};
    let chain_word = |links: &[(i64, i64, u8, u8)]| {
        from_canonical(&canonical_chain(links).expect("one entry per position is A5's chain"))
    };
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("bodies"));
    let form = closed_turn(None);
    let remainder = vec![(0i64, 0i64, 1u8, 2u8), (5, 5, 3u8, 4u8)];
    let mut b = entity.stroke();
    entity.withhold_form(form.clone(), &[], std::slice::from_ref(&remainder), [2, 0, 0, 0], &mut b);
    let first = chain_word(&[(0, 0, 1u8, 2u8)]);
    let second = chain_word(&[(5, 5, 3u8, 4u8)]);
    let whole = chain_word(&remainder);
    for (name, w) in [("first body", &first), ("second body", &second), ("chain of both", &whole)] {
        assert!(entity.epigenome.working_word(w.address().as_bytes()).is_none(), "the {name} never stands as a word");
        assert!(!entity.already_taught(w), "the {name} is not an utterance");
    }
    assert!(entity.dialect_len() >= 1, "the withheld form stands");
    assert!(entity.ideas_len() >= 1, "and is held as an idea");
}

/// 24.3, law 3: A MIND LOOKS IN ONE STRATUM, not through everything it has
/// ever said. `already_taught` read the whole of `taught_up` running an
/// exact `same_vertex` against every word this mind had ever uttered, on
/// every sentence of every speak — law 3's "no loop over pairs the carrier
/// does not already name", and its own answer, "the carrier is missing a
/// key and that key is the first thing to write". On a mesh whose voices
/// all produce language, that read grew with the mesh's whole output.
///
/// The key must be NECESSARY, or the mind would speak a word twice: two
/// words that are one vertex must land in one stratum. That is what this
/// asserts, since it is the property a wrong key would break silently.
#[test]
fn one_vertex_lies_in_one_stratum_however_it_is_spelled() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::element::{from_connection, identity_versor, translator_versor};

    let q = LetterId::turn_90((0, 1)).expect("the 90° turn");
    let p5 = LetterId::pythagorean_5((0, 1)).expect("a prime on the same plane");
    let conn = |w: &Word, shift: i64| {
        from_connection(w, &translator_versor(shift, 0), &[], &Vec::new(), [1, 0, 0, 0])
            .expect("a connection on the plane")
    };

    // ONE VERTEX, TWO PLACES. Same rotor, different carriage: not the same
    // vertex, but they must share a stratum or the exact test never sees
    // the pair it has to decide.
    let here = conn(&Word::from_letters(&[(q, false)]), 0);
    let there = conn(&Word::from_letters(&[(q, false)]), 5);
    assert_eq!(
        vertex_stratum(&here),
        vertex_stratum(&there),
        "the same rotor is one stratum wherever it is placed"
    );
    assert!(!here.same_vertex(&there), "and the exact test still tells them apart");

    // A DIFFERENT ROTOR IS A DIFFERENT STRATUM, so the fiber a lookup
    // reads is the words at one vertex and not the mind's whole output.
    let other = conn(&Word::from_letters(&[(p5, false)]), 0);
    assert_ne!(
        vertex_stratum(&here),
        vertex_stratum(&other),
        "another rotor is another stratum"
    );

    // The key is over the ROTOR, not the tape: four 90° turns are the
    // identity projectively, so a word spelled long lands where the word
    // spelled short does. A key over spelling would let the mind utter one
    // vertex twice under two spellings, which is 1.5's fork exactly.
    let long = conn(
        &Word::from_letters(&[(q, false), (q, false), (q, false), (q, false), (q, false)]),
        0,
    );
    assert!(long.same_vertex(&here), "five 90° turns are one, projectively");
    assert_eq!(
        vertex_stratum(&long),
        vertex_stratum(&here),
        "so they lie in one stratum, and the mind does not say it twice"
    );
    assert!(vertex_stratum(&here).is_some(), "a connection has a stratum");

    // A CHAIN A5 REFUSES HAS NO STRATUM. Two transitions on one site
    // describe nothing (16.5 step 3): the word is not a vertex, so no other
    // spelling can share a stratum with it, and it is filed by address
    // alone. It was keyed by its raw bytes — a key invented where the
    // canonical one is refused (rule 11, checklist i).
    // The refusal is now at the constructor: a chain saying two things at
    // one site has no canonical form, so there is no word to ask a stratum
    // of. This read the word A5 refused and then asked whether it had one.
    assert!(
        crate::transducer::element::from_residual(&vec![(0, 0, 1, 2), (0, 0, 3, 4)]).is_none(),
        "a chain A5 refuses mints no word"
    );
    let _ = identity_versor();
}

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

    // A reply that answers about all of them admits the verdicts: a
    // refusal is not an act, and a wait ("spoken again later") is not an
    // admission either — filed as taught, the waiting sentence would never
    // be spoken again (50.9).
    let mut codes = vec![0xff, 0xff, 0xff, 0xff];
    let admitted = read_speech_reply(
        &mut codes,
        &[dialogue::SPEECH_STOOD, dialogue::SPEECH_REFUSED, dialogue::SPEECH_DIALECT, dialogue::SPEECH_RATIFIED],
    );
    assert_eq!(admitted, vec![0, 3], "the refused sentence moved nothing and the waiting one is not admitted");
    assert_eq!(codes, vec![
        dialogue::SPEECH_STOOD,
        dialogue::SPEECH_REFUSED,
        dialogue::SPEECH_DIALECT,
        dialogue::SPEECH_RATIFIED
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

/// 78.52 — A MIND SPEAKS WHAT IT THOUGHT, IN ITS OWN DIALECT. An L seen at
/// its four hours about the grid point (2, 2), held by one mind with no hypha
/// to hear it: the fourth perception closes the thought's return, and the
/// mind speaks the WORD the thought transcribes to in that same act — a
/// connection spelled in the letters stands in the mind's dialect, carrying
/// its return; the thought itself never does. It waits on no word from
/// anywhere.
#[test]
fn a_mind_speaks_its_closed_thought_into_its_own_dialect() {
    use crate::client::sdk::*;
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let grid = |rows: &[&str]| -> Vec<Vec<u8>> { rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect() };
    let hours: [([&str; 5], [&str; 5]); 4] = [
        (["010000", "010000", "011000", "000000", "000000"], ["001000", "001000", "001100", "000000", "000000"]),
        (["000000", "001110", "001000", "000000", "000000"], ["000000", "000111", "000100", "000000", "000000"]),
        (["000000", "000000", "001100", "000100", "000100"], ["000000", "000000", "000110", "000010", "000010"]),
        (["000000", "000000", "001000", "111000", "000000"], ["000000", "000000", "000100", "011100", "000000"]),
    ];
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("bodies"));
    let spoken = |e: &Entity| {
        e.epigenome
            .dialect
            .scoped()
            .map(|(_, d)| d.sentence.element())
            .filter(|w| w.is_connection() && w.letters_rederive() == Some(true) && w.magnitude() == [0, 0, 0, 3])
            .count()
    };
    for (i, (input, output)) in hours.iter().enumerate() {
        let (a, b) = (grid(input), grid(output));
        let frames = perceive_family(&[(a.clone(), b.clone())]);
        let mut budget = entity.stroke_n(1 << 20);
        let f = scene_form_on(&NoCrystal, &mut budget, &a, &b, &frames).expect("the scene reads");
        assert!(entity.hold_perceived(f, &frames, &Vec::new(), &[], &mut budget), "hour {i} is held");
        assert_eq!(spoken(&entity), usize::from(i == 3), "hour {i}: the word stands only once the return closes");
    }
}
