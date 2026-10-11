// 78.53 — A COLOUR WORD, END TO END: a mind mints the fibre relation it
// read where the base stands still; the court re-derives it from its own
// copy of the family and stands it by that evidence, refusing a relation
// the family does not carry; a second vantage makes it final; the syllabus
// teaches it; the mind that learns it holds one colour word, as many as its
// court installed. The detector's channels anchor the names (78.56 II): the
// same colours read the same word for every mind; other colours, another.

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

/// 78.55 — A THEOREM STANDS BY ITS PROOF. With {1→2} and {2→3} installed,
/// the derived word {1→3} — citing them — is the same word a scene would
/// witness (the citation is outside the address), and the court admits it
/// with no task and no scene by composing exactly what it cites. A forged
/// citation (the cited words do not compose to the word) and a citation of
/// words the court never installed are refused.
#[test]
fn a_colour_theorem_stands_by_composing_what_it_cites() {
    use crate::transducer::element::{colour_word, derived_colour_word};
    let mut k = Kernel::new();
    let install = |k: &mut Kernel, task: u8, from: u8, to: u8| {
        let word = colour_word(&[(from, to)]).expect("a colour word");
        k.bind_task_family([task; 32], vec![(vec![vec![from, from]], vec![vec![to, to]])]);
        assert_eq!(k.perceive_speech([1; 32], &[task; 32], std::slice::from_ref(&word)), vec![crate::anastome::dialogue::SPEECH_STOOD]);
        k.rule_all_provisionals();
        k.enroll([2; 32], [8, 8, 8, 8]);
        k.confirm_peer_holds_addrs(std::slice::from_ref(&word.address()), [2; 32]);
        word
    };
    let a = install(&mut k, 20, 1, 2);
    let b = install(&mut k, 21, 2, 3);
    let theorem = derived_colour_word(&a, &b).expect("{2→3} ∘ {1→2}");
    assert_eq!(theorem.address(), colour_word(&[(1, 3)]).expect("a word").address(), "a derivation is a presentation: one word");
    assert!(theorem.colour_citation().is_some(), "it carries its proof");
    assert_eq!(
        k.perceive_speech([3; 32], &[], std::slice::from_ref(&theorem)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "no task, no scene: the court composes what it cites"
    );
    assert_eq!(k.acts_reading().colours_derived, 1, "admitted by derivation");
    // 78.56 IV — AND IT MINTS NOTHING: no claim stands for it, so its
    // install releases nothing; it earns only when cited to close residue.
    assert!(k.treasury().escrow_of(theorem.address().as_bytes()).is_none(), "a theorem holds no claim");
    let minted = k.treasury().minted().expect("minted adds up");
    k.rule_all_provisionals();
    k.confirm_peer_holds_addrs(std::slice::from_ref(&theorem.address()), [2; 32]);
    assert_eq!(k.treasury().minted().expect("minted adds up"), minted, "its admission mints nothing");
    // A FORGED CITATION: the same citation on a word it does not compose to.
    let mut forged = colour_word(&[(3, 1)]).expect("a word");
    forged.set_receipts(crate::transducer::element::ReceiptsEnvelope {
        derivations: vec![crate::transducer::element::DerivationCite {
            vertex: forged.address(),
            generators: vec![(a.address(), a.address(), 1), (b.address(), b.address(), 1)],
            residue: crate::transducer::alphabet::Word::identity(),
        }],
        ..Default::default()
    });
    assert_eq!(k.perceive_speech([3; 32], &[], std::slice::from_ref(&forged)), vec![crate::anastome::dialogue::SPEECH_REFUSED], "a lie about its proof");
    // A CITATION OF WORDS THIS COURT NEVER INSTALLED.
    let stranger = derived_colour_word(&colour_word(&[(4, 5)]).expect("w"), &colour_word(&[(5, 6)]).expect("w")).expect("a theorem");
    assert_eq!(k.perceive_speech([3; 32], &[], std::slice::from_ref(&stranger)), vec![crate::anastome::dialogue::SPEECH_REFUSED], "unproven here");
}

/// 78.55 — FEDERATION BY PROOF, NEVER BY TRUST. A colour word one court
/// stood by a scene carries that task as its witness (outside its address,
/// through the record); a peer court holding the same task admits it by
/// re-deriving the relation from its own copy — no task named, nothing
/// searched — and a court that holds no such task holds it as dialect.
#[test]
fn a_peer_admits_a_colour_word_by_re_deriving_its_witness() {
    use crate::transducer::element::colour_word;
    let family = vec![(vec![vec![1u8, 1]], vec![vec![2u8, 2]])];
    let tid = [30u8; 32];
    let mut home = Kernel::new();
    home.bind_task_family(tid, family.clone());
    let word = colour_word(&[(1, 2)]).expect("a colour word");
    assert_eq!(home.perceive_speech([1; 32], &tid, std::slice::from_ref(&word)), vec![crate::anastome::dialogue::SPEECH_STOOD]);
    let held = home.provisional.get(word.address().as_bytes()).cloned().expect("provisional");
    assert_eq!(held.address(), word.address(), "the witness does not rename the word");
    assert_eq!(held.receipts().witness, vec![crate::transducer::address::Address::from_bytes(tid)], "it carries its witness");
    // THROUGH THE RECORD: what travels is what a peer reads.
    let tape = held.to_bytes().expect("the record writes");
    let sent = Element::from_bytes(&tape, &mut 0).expect("and reads back");
    assert_eq!(sent.receipts().witness, held.receipts().witness);
    // A PEER HOLDING THE TASK re-derives it.
    let mut peer = Kernel::new();
    peer.bind_task_family(tid, family);
    let mut cover = crate::network::nerve::Cover::new();
    let mut glue = peer.stroke();
    assert!(!matches!(peer.glue_fetched(sent.clone(), [1; 32], [2; 32], &mut cover, &mut glue), Glue::Dialect), "proven here");
    // A COURT WITHOUT IT holds it as dialect.
    let mut stranger = Kernel::new();
    let mut glue = stranger.stroke();
    assert!(matches!(stranger.glue_fetched(sent, [1; 32], [3; 32], &mut cover, &mut glue), Glue::Dialect), "unproven here");
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
    let other = crate::transducer::element::colour_word(
        &crate::transducer::family::fibre_relation(&permuted.train).expect("a fibre relation"),
    )
    .expect("a word");
    assert_ne!(word.address(), other.address(), "other channels are other colours: another word (78.56 II)");
    let same = crate::arc::parse_task(&json(1, 2, 3)).expect("the task reads");
    let again = crate::transducer::element::colour_word(
        &crate::transducer::family::fibre_relation(&same.train).expect("the same fibre"),
    )
    .expect("a word");
    assert_eq!(word.address(), again.address(), "the same channels, the same word, for every mind");

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
    assert_eq!(mind.epigenome.genome_words().filter(|w| w.is_colour_word()).count(), 1, "the mind holds what its court installed");
}

/// 78.58 (Closure I) — A MERGE IS UNITARY: hue 2 recoloured into hue 1,
/// which stands still, loses no distinction. The word the scene witnesses
/// keeps 1 still, moves 2 → 1 and rotates 2's distinction into the
/// reservoir e_∅, so it inverts exactly; the relation that drops the
/// record — the injection {2→1} this read as before — is not the scene's,
/// and no word at all is minted for a merge with its distinction lost. No
/// preimage is singled out to go unrecorded (rule 4): of two moved hues
/// merged, both are recorded. It composes: after {1→3}, both hues go to 3,
/// both recorded. The court stands the witnessed merge by its own copy of
/// the family.
#[test]
fn a_merge_rotates_its_distinction_into_the_reservoir() {
    use crate::transducer::element::{colour_word, compose_fibre, fibre_function, invert_fibre, is_fibre_relation, RESERVOIR};
    let family = vec![(vec![vec![1u8, 2, 0]], vec![vec![1u8, 1, 0]])];
    let sigma = crate::transducer::family::fibre_relation(&family).expect("the fibre moved");
    assert_eq!(sigma, vec![(1, 1), (2, 1), (RESERVOIR, 2)], "1 still, 2 → 1: 2's distinction in e_∅");
    assert_eq!(fibre_function(&sigma), vec![(1, 1), (2, 1)], "Π∘U on the empty reservoir: what the scene showed");
    assert!(invert_fibre(&sigma).is_empty(), "1 returns to itself; 2 only with its record");
    assert!(!is_fibre_relation(&[(1, 3), (2, 3)]), "a merge with its distinction lost is no word");
    assert!(!is_fibre_relation(&[(1, 3), (2, 3), (RESERVOIR, 1)]), "no preimage is singled out unrecorded");
    assert!(is_fibre_relation(&[(1, 3), (2, 3), (RESERVOIR, 1), (RESERVOIR, 2)]));
    assert!(!is_fibre_relation(&[(1, 1), (2, 3)]), "a still hue nothing merges into is no row");
    assert!(!is_fibre_relation(&[(2, 1), (RESERVOIR, 3)]), "a record names a hue that moved");
    assert!(!is_fibre_relation(&[(2, 1), (RESERVOIR, 2)]), "a record names a merged hue");
    let both = vec![(1, 3), (2, 3), (RESERVOIR, 1), (RESERVOIR, 2)];
    assert_eq!(compose_fibre(&[(1, 3)], &sigma), Some(both.clone()), "the merge carried through");
    assert!(invert_fibre(&both).is_empty(), "every merged hue returns only with its record");
    assert_eq!(invert_fibre(&[(1, 2), (3, 1)]), vec![(1, 3), (2, 1)], "an injection inverts as before");
    let word = colour_word(&sigma).expect("a unitary merge is a colour word");
    assert!(word.is_colour_word());
    let tape = word.to_bytes().expect("the record writes");
    let sent = Element::from_bytes(&tape, &mut 0).expect("and reads back");
    assert_eq!(sent.letters().map(|l| l.sigma.clone()), Some(sigma.clone()), "the record travels with the word");
    let mut k = Kernel::new();
    let tid = [40u8; 32];
    k.bind_task_family(tid, family);
    let lost = colour_word(&[(2, 1)]).expect("an injection is a word");
    assert_ne!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&lost)), vec![crate::anastome::dialogue::SPEECH_STOOD], "not what the scene showed");
    assert_eq!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&word)), vec![crate::anastome::dialogue::SPEECH_STOOD], "witnessed: it stands");
}
