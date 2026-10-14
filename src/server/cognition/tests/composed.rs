// MU-CLIENT §5.4 (A) — COURTS ADJUDICATE WORDS: a sentence is the court's
// evidence and section, ruled and minted as before; what its closed rings
// teach enters the language beside it as words, each its own install.

use crate::cognition::book::speech::words_of_sentence;
use crate::cognition::*;
use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b.checked_sub(b'0').expect("a grid row is digits")).collect()).collect()
}

/// AN INSTALLED SENTENCE BRINGS ITS WORDS INTO THE LANGUAGE. Spoken with its
/// task, a family's sentence stands; installed on a second vantage, it is held
/// as the court's evidence, and every word its closed rings teach is held
/// beside it — a word, never a sentence — discovered by the voice that spoke
/// the sentence.
#[test]
fn an_installed_sentence_brings_the_words_it_teaches_into_the_language() {
    let mut k = Kernel::new();
    let speaker = [1u8; 32];
    for family in [vec![(grid(&["11"]), grid(&["22"]))], vec![(grid(&["10", "11"]), grid(&["11", "10"]))]] {
        let Some(rule) = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)) else { continue };
        let Some(spoken) = mint_sentence(&rule, &family, &NoCrystal, 0) else { continue };
        let task = crate::registry::task_id(&crate::arc::Task { train: family.clone(), test: Vec::new() }).expect("a task identity");
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
