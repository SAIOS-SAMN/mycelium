// 78.64 H1 — EVERY CARRIER A WORD FILLS, IT LEAVES: a word's passage
// through the court leaves no carrier grown but the history of the acts it
// took.

use crate::cognition::*;
use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b.checked_sub(b'0').expect("a grid row is digits")).collect()).collect()
}

/// THE HISTORY A PASSAGE LEAVES: the acts it sealed on the ledger and the
/// rows they wrote, which the fold compacts, and the family its task was
/// bound to, which is the corpus.
const HISTORY: [&str; 3] = ["chain.", "treasury.", "task_family"];

/// The census as a table by name.
fn table(c: &crate::observe::Carriers) -> crate::network::nerve::Keyed<u64> {
    let mut t = crate::network::nerve::Keyed::new();
    for (name, n) in &c.held {
        t.insert(name.as_bytes(), *n);
    }
    t
}

/// A SENTENCE LISTED AND THEN UNLISTED leaves every carrier it filled as
/// it found it: the census before and after agree on every carrier but the
/// history. A carrier added to the court without its inverse is counted
/// by the census (which names every field) and fails here.
#[test]
fn a_word_that_passes_through_the_court_leaves_no_carrier_grown() {
    let mut k = Kernel::new();
    let family = vec![(grid(&["11"]), grid(&["22"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a recolour extracts");
    let spoken = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence");
    let task = crate::registry::task_id(&crate::arc::Task { train: family.clone(), test: Vec::new() })
        .expect("a grid pair's chain is A5's, so the task has an identity");
    k.bind_task_family(&task, family);
    let before = table(&k.carriers());

    let codes = k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&spoken));
    assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD], "the sentence is listed");
    // The word stands on the book and on the docket; nothing flows to it,
    // and it lapses (78.37 (3)) — off both, the lapse sealed.
    let docketed: Vec<Vec<u8>> = k.provisional.scoped().map(|(a, _)| a).collect();
    assert!(!docketed.is_empty() && !k.pending.is_empty(), "the word is docketed and listed");
    for addr in &docketed {
        k.lapse_word(addr);
    }
    assert!(k.pending.is_empty() && k.provisional.is_empty(), "the word left every row");

    let after = k.carriers();
    let grown: Vec<String> = after
        .held
        .iter()
        .filter(|(name, _)| !HISTORY.iter().any(|h| name.starts_with(h)))
        .filter(|(name, n)| before.get(name.as_bytes()).copied() != Some(*n))
        .map(|(name, n)| format!("{name} {:?} → {n}", before.get(name.as_bytes()).copied()))
        .collect();
    assert!(grown.is_empty(), "carriers the word left grown: {grown:?}");
}
