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

/// 78.55 — THE LANGUAGE DERIVES WHAT IT WAS NOT TOLD. A mind taught two
/// ruled colour words, {1→2} and {2→3}, derives {1→3} — their composite in
/// the symmetric inverse monoid — and {2→1}, an inverse, from its language
/// alone; it derives nothing its words do not compose to ({1→4}); and the
/// composite of partial injections never merges two hues.
#[test]
fn a_mind_derives_a_colour_relation_its_ruled_words_compose_to() {
    use crate::transducer::element::{colour_word, compose_fibre, invert_fibre};
    assert_eq!(compose_fibre(&[(2, 3)], &[(1, 2)]), Some(vec![(1, 3)]), "defined where the first moves and the second moves that");
    assert_eq!(compose_fibre(&[(2, 1)], &[(1, 2)]), None, "a hue returned to itself moves nothing");
    assert_eq!(invert_fibre(&[(1, 2), (3, 1)]), vec![(1, 3), (2, 1)]);
    let mut k = Kernel::new();
    let mut mind = crate::client::sdk::Entity::open("127.0.0.1:1", crate::client::sdk::Kernel::named("deducer"));
    for (task, (from, to)) in [(10u8, (1u8, 2u8)), (11, (2, 3))] {
        let family = vec![(vec![vec![from, from]], vec![vec![to, to]])];
        let word = colour_word(&[(from, to)]).expect("a colour word");
        let tid = [task; 32];
        k.bind_task_family(tid, family);
        assert_eq!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&word)), vec![crate::anastome::dialogue::SPEECH_STOOD]);
        k.rule_all_provisionals();
        k.enroll([2; 32], [8, 8, 8, 8]);
        k.confirm_peer_holds_addrs(std::slice::from_ref(&word.address()), [2; 32]);
        let page = k.serve_syllabus(&[word.address().as_bytes().to_vec()], [7; 32], 1 << 20);
        let [taught] = page.words.as_slice() else { panic!("served") };
        let mut budget = mind.stroke_n(1 << 16);
        assert!(mind.absorb_taught_at(&taught.word, Some(taught.at), &mut budget).is_some(), "learned");
    }
    let mut budget = mind.stroke_n(1 << 16);
    assert!(mind.colour_derivation(&[(1, 3)], &mut budget), "the composite is derived");
    assert!(mind.colour_derivation(&[(2, 1)], &mut budget), "an inverse is derived");
    assert!(!mind.colour_derivation(&[(1, 4)], &mut budget), "nothing the words do not compose to");
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
