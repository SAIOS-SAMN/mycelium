// 78.53 — A COLOUR WORD, END TO END: a mind mints the fibre relation it
// read where the base stands still; the court re-derives it from its own
// copy of the family and stands it by that evidence, refusing a relation
// the family does not carry; a second vantage makes it final; the syllabus
// teaches it; the mind that learns it holds one colour word, as many as its
// court installed. A palette relabelled reads the same word.

use crate::cognition::*;

/// A family recoloured in place: 1 → 2 and 3 → 1, the base standing still.
fn json(a: u8, b: u8, c: u8) -> String {
    format!(
        r#"{{"train":[{{"input":[[{a},{a},0,{c}]],"output":[[{b},{b},0,{a}]]}},
                      {{"input":[[0,{c},{a},0]],"output":[[0,{a},{b},0]]}}],
            "test":[{{"input":[[{a},0,{c},0]],"output":[[{b},0,{a},0]]}}]}}"#
    )
}

#[test]
fn a_colour_word_is_minted_ruled_by_evidence_taught_and_held() {
    let task = crate::arc::parse_task(&json(1, 2, 3)).expect("the task reads");
    let permuted = crate::arc::parse_task(&json(7, 4, 9)).expect("the permuted task reads");
    // THE MIND'S WORD: the fibre relation, as a colour word.
    let sigma = crate::transducer::family::fibre_relation(&task.train).expect("the fibre moved");
    let word = crate::transducer::element::colour_word(&sigma).expect("a relation that moves a hue");
    assert!(word.is_colour_word());
    assert_eq!(word.letters().map(|l| l.sigma.clone()), Some(sigma.clone()), "the word carries the relation");
    let again = crate::transducer::element::colour_word(
        &crate::transducer::family::fibre_relation(&permuted.train).expect("the same fibre"),
    )
    .expect("a word");
    assert_eq!(word.address(), again.address(), "a relabelled palette mints the same word");

    // THE COURT RULES BY EVIDENCE: its own copy of the family.
    let mut k = Kernel::new();
    let tid = [3u8; 32];
    k.bind_task_family(tid, task.train.clone());
    // A relation the family does not carry is refused, whoever says it.
    let wrong = crate::transducer::element::colour_word(&[(1, 3)]).expect("a word");
    assert_ne!(
        k.perceive_speech([1; 32], &tid, std::slice::from_ref(&wrong)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "unwitnessed by the base"
    );
    assert_eq!(
        k.perceive_speech([1; 32], &tid, std::slice::from_ref(&word)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "witnessed: it stands"
    );
    // FINALITY: a second vantage re-derives it and the court installs it.
    k.rule_all_provisionals();
    let cover = [8u64, 8, 8, 8];
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&word.address()), [2; 32]);
    assert!(k.installed_word(word.address().as_bytes()).is_some(), "installed");
    assert_eq!(k.acts_reading().colours, 1, "one colour word installed");

    // TAUGHT AND HELD: the syllabus page carries it; the mind learns it.
    let page = k.serve_syllabus(&[word.address().as_bytes().to_vec()], [7; 32], 1 << 20);
    let [taught] = page.words.as_slice() else { panic!("one word served: {:?}", page.deferred) };
    let mut mind = crate::client::sdk::Entity::open("127.0.0.1:1", crate::client::sdk::Kernel::named("learner"));
    let mut budget = mind.stroke_n(1 << 16);
    assert!(mind.absorb_taught_at(&taught.word, Some(taught.at), &mut budget).is_some(), "learned");
    let senses = mind.senses(&crate::observe::Learning::default());
    assert_eq!(senses.read("language", "colour words"), Some(1), "the mind holds what its court installed");
}
