// 78.53 — A COLOUR WORD, END TO END: a mind mints the fibre relation it
// read where the base stands still; the court re-derives it from its own
// copy of the family and stands it by that evidence, refusing a relation
// the family does not carry; a second vantage makes it final; the syllabus
// teaches it; the mind that learns it holds one colour word, as many as its
// court installed. The detector's channels anchor the names (78.56 II): the
// same colours read the same word for every mind; other colours, another.

use crate::cognition::*;
use crate::transducer::element::FibreRelation;

/// A family recoloured in place: 1 → 2 and 3 → 1, the base standing still;
/// read through the frame-fixing perspective (`raster::frame_fixed_on_ground`,
/// ROADMAP track 26 S2).
fn task(a: u8, b: u8, c: u8) -> crate::transducer::family::Task {
    crate::transducer::raster::frame_fixed_on_ground(crate::transducer::family::task_of_lines(
        vec![
            (vec![vec![a, a, 0, c]], vec![vec![b, b, 0, a]]),
            (vec![vec![0, c, a, 0]], vec![vec![0, a, b, 0]]),
        ],
        vec![(vec![vec![a, 0, c, 0]], vec![vec![b, 0, a, 0]])],
    ))
}

/// 78.55 — A THEOREM STANDS BY ITS PROOF. With {1→2} and {2→3} installed,
/// the derived word {1→3} — citing them — is the same word a scene would
/// witness (the citation is outside the address), and the court admits it
/// with no task and no scene by composing exactly what it cites. A forged
/// citation (the cited words do not compose to the word) and a citation of
/// words the court never installed are refused.
#[test]
fn perceive_speech_admits_colour_theorem_by_composition() {
    crate::transducer::raster::stand_on(0);
    use crate::transducer::element::{colour_word, derived_colour_word};
    let mut k = Court::new();
    let install = |k: &mut Court, task: u8, from: u8, to: u8| {
        let word = colour_word(&FibreRelation::of_rows(&[(from, to)])).expect("a colour word");
        k.bind_task_family([task; 32], crate::transducer::spelled::pairs_of(vec![(vec![vec![from, from]], vec![vec![to, to]])]));
        // The scene shows no 0: the speaker states no ground (MU-SENSING §6.2).
        assert_eq!(k.perceive_speech_under([1; 32], &[task; 32], std::slice::from_ref(&word), &[None]), vec![crate::anastome::dialogue::SPEECH_STOOD]);
        k.rule_all_provisionals();
        k.enroll([2; 32], [8, 8, 8, 8]);
        k.confirm_peer_holds_addrs(std::slice::from_ref(&word.address()), [2; 32]);
        word
    };
    let a = install(&mut k, 20, 1, 2);
    let b = install(&mut k, 21, 2, 3);
    let theorem = derived_colour_word(&a, &b).expect("{2→3} ∘ {1→2}");
    assert_eq!(theorem.address(), colour_word(&FibreRelation::of_rows(&[(1, 3)])).expect("a word").address(), "a derivation is a presentation: one word");
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
    let minted = k.treasury().minted();
    k.rule_all_provisionals();
    k.confirm_peer_holds_addrs(std::slice::from_ref(&theorem.address()), [2; 32]);
    assert_eq!(k.treasury().minted(), minted, "its admission mints nothing");
    // A FORGED CITATION: the same citation on a word it does not compose to.
    let mut forged = colour_word(&FibreRelation::of_rows(&[(3, 1)])).expect("a word");
    forged.set_receipts(crate::transducer::element::ReceiptsEnvelope {
        derivations: vec![crate::transducer::element::DerivationCite {
            vertex: forged.address(),
            generators: vec![(a.address(), a.address(), 1), (b.address(), b.address(), 1)],
            residue: crate::transducer::alphabet::Word::identity(),
            conjugator: None,
        }],
        ..Default::default()
    });
    assert_eq!(k.perceive_speech([3; 32], &[], std::slice::from_ref(&forged)), vec![crate::anastome::dialogue::SPEECH_REFUSED], "a lie about its proof");
    // A CITATION OF WORDS THIS COURT NEVER INSTALLED.
    let stranger = derived_colour_word(&colour_word(&FibreRelation::of_rows(&[(4, 5)])).expect("w"), &colour_word(&FibreRelation::of_rows(&[(5, 6)])).expect("w")).expect("a theorem");
    assert_eq!(k.perceive_speech([3; 32], &[], std::slice::from_ref(&stranger)), vec![crate::anastome::dialogue::SPEECH_REFUSED], "unproven here");
}

/// 78.55 — FEDERATION BY PROOF, NEVER BY TRUST. A colour word one court
/// stood by a scene carries that task as its witness (outside its address,
/// through the record); a peer court holding the same task admits it by
/// re-deriving the relation from its own copy — no task named, nothing
/// searched — and a court that holds no such task holds it as dialect.
#[test]
fn glue_fetched_admits_colour_word_by_rederived_witness() {
    crate::transducer::raster::stand_on(0);
    use crate::transducer::element::colour_word;
    let family = crate::transducer::spelled::pairs_of(vec![(vec![vec![1u8, 1]], vec![vec![2u8, 2]])]);
    let tid = [30u8; 32];
    let mut home = Court::new();
    home.bind_task_family(tid, family.clone());
    let word = colour_word(&FibreRelation::of_rows(&[(1, 2)])).expect("a colour word");
    // The scene shows no 0: the speaker states no ground (MU-SENSING §6.2).
    assert_eq!(home.perceive_speech_under([1; 32], &tid, std::slice::from_ref(&word), &[None]), vec![crate::anastome::dialogue::SPEECH_STOOD]);
    let held = home.provisional.get(word.address().as_bytes()).cloned().expect("provisional");
    assert_eq!(held.address(), word.address(), "the witness does not rename the word");
    assert_eq!(held.receipts().witness, vec![crate::transducer::address::Address::from_bytes(tid)], "it carries its witness");
    // THROUGH THE RECORD: what travels is what a peer reads.
    let tape = held.to_bytes().expect("the record writes");
    let sent = Element::from_bytes(&tape, &mut 0).expect("and reads back");
    assert_eq!(sent.receipts().witness, held.receipts().witness);
    // A PEER HOLDING THE TASK re-derives it.
    let mut peer = Court::new();
    peer.bind_task_family(tid, family);
    let mut cover = crate::network::nerve::HomeTrie::new();
    assert!(!matches!(super::fixtures::fetched(&mut peer, &sent, [1; 32], [2; 32], &mut cover), Glue::Dialect), "proven here");
    // A COURT WITHOUT IT holds it as dialect — ruled so, never a replay the
    // stroke left unread.
    let mut stranger = Court::new();
    assert!(matches!(super::fixtures::fetched(&mut stranger, &sent, [1; 32], [3; 32], &mut cover), Glue::Dialect), "unproven here");
}

/// A colour word is minted from the fibre relation, ruled by the court's
/// evidence, taught by the syllabus and held by the mind that learns it.
#[test]
fn colour_word_mints_rules_teaches_and_holds() {
    let (task, permuted) = (task(1, 2, 3), task(7, 4, 9));
    // THE MIND'S WORD: the fibre relation, as a colour word, read under the
    // candidate 0 of the family's ground (a vortex of 0 and the third hue,
    // read once per candidate: MU-SENSING §3.4), the ground its speech
    // states (§6.2).
    let on_zero = |f: &crate::transducer::family::Family| {
        crate::transducer::raster::on_ground(crate::transducer::family::transduce::Ground::One(0), || crate::transducer::family::fibre_relation(f))
    };
    let sigma = on_zero(&task.train).expect("the fibre moved");
    let word = crate::transducer::element::colour_word(&sigma).expect("a relation that moves a hue");
    assert!(word.is_colour_word());
    assert_eq!(word.letters().map(|l| l.fibre()), Some(sigma.clone()), "the word carries the relation");
    let other = crate::transducer::element::colour_word(
        &on_zero(&permuted.train).expect("a fibre relation"),
    )
    .expect("a word");
    assert_ne!(word.address(), other.address(), "other channels are other colours: another word (78.56 II)");
    let same = self::task(1, 2, 3);
    let again = crate::transducer::element::colour_word(
        &on_zero(&same.train).expect("the same fibre"),
    )
    .expect("a word");
    assert_eq!(word.address(), again.address(), "the same channels, the same word, for every mind");

    // THE COURT RULES BY EVIDENCE: its own copy of the family.
    let mut k = Court::new();
    let tid = [3u8; 32];
    k.bind_task_family(tid, task.train.clone());
    // The speaker states the candidate 0 its word was read under
    // (MU-SENSING §6.2). A relation the family does not carry is refused,
    // whoever says it.
    let stated = [Some(vec![0u8])];
    let wrong = crate::transducer::element::colour_word(&FibreRelation::of_rows(&[(1, 3)])).expect("a word");
    assert_ne!(
        k.perceive_speech_under([1; 32], &tid, std::slice::from_ref(&wrong), &stated),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "unwitnessed by the base"
    );
    assert_eq!(
        k.perceive_speech_under([1; 32], &tid, std::slice::from_ref(&word), &stated),
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
    let mut mind = crate::client::sdk::Mind::open("127.0.0.1:1", crate::client::sdk::Voice::named("learner"));
    let mut budget = mind.stroke_n(1 << 16);
    assert!(mind.absorb_taught_at(&taught.word, Some(taught.at), &mut budget).is_some(), "learned");
    assert_eq!(mind.language.genome_words().filter(|w| w.is_colour_word()).count(), 1, "the mind holds what its court installed");
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
fn fibre_relation_rotates_merge_into_reservoir() {
    crate::transducer::raster::stand_on(0);
    use crate::transducer::element::{colour_word, compose_fibre, fibre_function, invert_fibre, is_fibre_relation};
    let rel = |rows: &[(u8, u8)], records: &[u8]| FibreRelation { rows: rows.to_vec(), records: records.to_vec() };
    let family = crate::transducer::spelled::pairs_of(vec![(vec![vec![1u8, 2, 0]], vec![vec![1u8, 1, 0]])]);
    let sigma = crate::transducer::family::fibre_relation(&family).expect("the fibre moved");
    assert_eq!(sigma, rel(&[(1, 1), (2, 1)], &[2]), "1 still, 2 → 1: 2's distinction in e_∅");
    assert_eq!(fibre_function(&sigma), vec![(1, 1), (2, 1)], "Π∘U on the empty reservoir: what the scene showed");
    assert!(invert_fibre(&sigma).is_empty(), "1 returns to itself; 2 only with its record");
    assert!(!is_fibre_relation(&rel(&[(1, 3), (2, 3)], &[])), "a merge with its distinction lost is no word");
    assert!(!is_fibre_relation(&rel(&[(1, 3), (2, 3)], &[1])), "no preimage is singled out unrecorded");
    assert!(is_fibre_relation(&rel(&[(1, 3), (2, 3)], &[1, 2])));
    assert!(!is_fibre_relation(&rel(&[(1, 1), (2, 3)], &[])), "a still hue nothing merges into is no row");
    assert!(!is_fibre_relation(&rel(&[(2, 1)], &[3])), "a record names a hue that moved");
    assert!(!is_fibre_relation(&rel(&[(2, 1)], &[2])), "a record names a merged hue");
    let both = rel(&[(1, 3), (2, 3)], &[1, 2]);
    assert_eq!(compose_fibre(&rel(&[(1, 3)], &[]), &sigma), Some(both.clone()), "the merge carried through");
    assert!(invert_fibre(&both).is_empty(), "every merged hue returns only with its record");
    assert_eq!(invert_fibre(&rel(&[(1, 2), (3, 1)], &[])), vec![(1, 3), (2, 1)], "an injection inverts as before");
    let word = colour_word(&sigma).expect("a unitary merge is a colour word");
    assert!(word.is_colour_word());
    let tape = word.to_bytes().expect("the record writes");
    let sent = Element::from_bytes(&tape, &mut 0).expect("and reads back");
    assert_eq!(sent.letters().map(|l| l.fibre()), Some(sigma.clone()), "the record travels with the word");
    let mut k = Court::new();
    let tid = [40u8; 32];
    k.bind_task_family(tid, family);
    let lost = colour_word(&rel(&[(2, 1)], &[])).expect("an injection is a word");
    assert_ne!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&lost)), vec![crate::anastome::dialogue::SPEECH_STOOD], "not what the scene showed");
    assert_eq!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&word)), vec![crate::anastome::dialogue::SPEECH_STOOD], "witnessed: it stands");
}

/// MU-SENSING §0, §3.3; ROADMAP track 26 S3 — A MIND STATES A SYMBOL'S
/// MEANING AS A RELATION AND THE COURT RECONSTRUCTS IT. The meaning of a
/// symbol is the fibre relation the family shows (78.53), never its code:
/// the mind reads it off a family whose symbols are past ten (no palette
/// bounds them) and states it as a colour word; the court reconstructs it
/// from its own copy of the family and stands it, and refuses a relation
/// the family does not show. The same family under another labelling reads
/// one relation on its symbol set's indices: the meaning is the relation,
/// and the codes are labels of it.
#[test]
fn mind_states_a_symbol_meaning_as_a_relation_and_court_reconstructs_it() {
    use crate::transducer::element::{colour_word, fibre_on_symbols};
    use crate::transducer::symbols::SymbolSet;
    let framed = |row: &[u8], g: u8| -> crate::transducer::spelled::SectionSample {
        let mut middle = vec![g];
        middle.extend_from_slice(row);
        middle.push(g);
        vec![vec![g; row.len() + 2], middle, vec![g; row.len() + 2]].into()
    };
    // Each body of `a` takes `b`, and each body of `c` takes `a`.
    let family = |a: u8, b: u8, c: u8, g: u8| {
        vec![
            (framed(&[a, a, g, c], g), framed(&[b, b, g, a], g)),
            (framed(&[c, g, a, g, a], g), framed(&[a, g, b, g, b], g)),
        ]
    };
    let wide = family(11, 12, 13, 20);
    // THE MIND STATES IT: the relation read off the family, as a word.
    let sigma = crate::transducer::family::fibre_relation(&wide).expect("the fibre moved");
    assert_eq!(sigma, FibreRelation::of_rows(&[(11, 12), (13, 11)]), "11 takes 12; 13 takes 11");
    let word = colour_word(&sigma).expect("a colour word over symbols past ten");
    assert_eq!(word.letters().map(|l| l.fibre()), Some(sigma.clone()), "the word states the relation");
    // ONE MEANING UNDER ANOTHER LABELLING: on Σ's indices the relation of
    // the family relabelled is the same relation.
    let narrow = family(1, 2, 3, 0);
    let on = |f: &crate::transducer::family::Family| {
        let s = crate::transducer::family::fibre_relation(f)?;
        let mut rows = fibre_on_symbols(&s, &SymbolSet::of_pairs(f))?;
        rows.sort_unstable();
        Some(rows)
    };
    assert_eq!(on(&wide), on(&narrow), "the relation, not the codes");
    // THE COURT RECONSTRUCTS IT from its own copy of the family.
    let mut k = Court::new();
    let tid = [41u8; 32];
    k.bind_task_family(tid, wide);
    let unshown = colour_word(&FibreRelation::of_rows(&[(11, 13)])).expect("a word");
    assert_ne!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&unshown)), vec![crate::anastome::dialogue::SPEECH_STOOD], "a relation the family does not show");
    let relabelled = colour_word(&FibreRelation::of_rows(&[(1, 2), (3, 1)])).expect("a word");
    assert_ne!(
        k.perceive_speech([1; 32], &tid, std::slice::from_ref(&relabelled)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the codes of another labelling name no relation of this family"
    );
    assert_eq!(k.perceive_speech([1; 32], &tid, std::slice::from_ref(&word)), vec![crate::anastome::dialogue::SPEECH_STOOD], "reconstructed: it stands");
}
