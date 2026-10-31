// MU-CLIENT §5.4 (A) — COURTS ADJUDICATE WORDS: a sentence is the court's
// evidence and section, ruled and minted as before; what its closed rings
// teach enters the language beside it as words, each its own install.

use crate::cognition::book::speech::words_of_sentence;
use crate::cognition::*;
use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b.checked_sub(b'0').expect("a grid row is digits")).collect()).collect()
}

/// D3 — THE HELD SENTENCES DECOMPOSED FORWARD. A court that holds a sentence
/// from before words were installed beside it (here: held directly, its words
/// not) brings every word that sentence teaches into its language on one
/// forward pass, discovered by the sentence's own discoverer; a second pass
/// finds nothing new.
#[test]
fn decompose_held_sentences_adds_words_once() {
    let mut k = Court::new();
    let speaker = [3u8; 32];
    let family = vec![(grid(&["10", "11"]), grid(&["11", "10"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule extracts");
    let spoken = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence");
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    k.basis_push(spoken.clone(), speaker, &mut budget);
    let words = words_of_sentence(&spoken, &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert!(!words.is_empty(), "the rotation's sentence teaches at least one word");
    let first = k.decompose_held_sentences();
    assert_eq!(first, words.iter().filter(|w| w.address() != spoken.address()).count(), "every word the sentence teaches entered");
    for word in &words {
        assert!(k.basis.get(word.address().as_bytes()).is_some(), "held beside the sentence");
        assert_eq!(k.discoverer_of(word.address().as_bytes()), Some(speaker), "under the sentence's discoverer");
    }
    assert_eq!(k.decompose_held_sentences(), 0, "a second pass finds nothing new");
}

/// AN INSTALLED SENTENCE BRINGS ITS WORDS INTO THE LANGUAGE. Spoken with its
/// task, a family's sentence stands; installed on a second vantage, it is held
/// as the court's evidence, and every word its closed rings teach is held
/// beside it — a word, never a sentence — discovered by the voice that spoke
/// the sentence.
#[test]
fn home_install_holds_sentence_words_in_language() {
    let mut k = Court::new();
    let speaker = [1u8; 32];
    for family in [vec![(grid(&["11"]), grid(&["22"]))], vec![(grid(&["10", "11"]), grid(&["11", "10"]))]] {
        let Some(rule) = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)) else { continue };
        let Some(spoken) = mint_sentence(&rule, &family, &NoCrystal, 0) else { continue };
        let task = crate::registry::task_address(&crate::arc::Task { train: family.clone(), test: Vec::new() }).expect("a task identity");
        k.bind_task_family(&task, family);
        let codes = k.perceive_speech(speaker, task.as_bytes(), std::slice::from_ref(&spoken));
        assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD], "the sentence stands");
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
        let addr = spoken.address();
        k.home_install(addr.as_bytes(), [2; 32], &mut budget);
        assert!(k.basis.get(addr.as_bytes()).is_some(), "the sentence is held: the court's evidence and section");
        let words = words_of_sentence(&spoken, &mut crate::physics::Physics::default().stroke_n(1 << 20));
        for word in &words {
            let at = word.address();
            let held = k.basis.get(at.as_bytes()).expect("each word the sentence teaches is in the language");
            assert!(!held.element().is_sentence(), "a word, never a sentence");
            assert_eq!(k.discoverer_of(at.as_bytes()), Some(speaker), "discovered by the voice that spoke the sentence");
        }
    }
}
