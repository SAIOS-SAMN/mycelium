// MU-HOSTING §5a.9, §5a.10 — THE SIGNED REACH: a receipt admitted by
// back-fill is stored by its chain, so the catch-up can link on through it,
// and no reader acts on it until a frame its signer signed at or above its
// place is admitted.

use crate::chronometry::{ActKind, Cite, KStepReceipt, ReceiptChain, Ruled, Stamp};
use crate::cognition::*;
use crate::court::admission::Admitted;
use crate::court::identity::HyphaKey;
use crate::transducer::address::Address;

/// THE WAITING RULING. Hypha a docketed a word, admitting it at its own
/// stamp `at`; court c docketed the same word at its own stamp; a third
/// hypha's ruling upholds the word by a's stamp, which c does not hold, so
/// on c the ruling waits. Answers c, a's key, a's line, `at`, the word and
/// a's word citation.
fn waiting_ruling(name: &str, own: u8) -> (Court, HyphaKey, Vec<KStepReceipt>, Stamp, Address, Cite) {
    let key = super::fixtures::court_key(name);
    let (a, addr) = super::fixtures::docketed(key.verifying_bytes());
    let at = a.admitted_at(addr.as_bytes()).expect("a admitted the word at an act");
    let line: Vec<KStepReceipt> = a.chain().own().collect();
    let word = line
        .iter()
        .find(|r| r.stamp == at)
        .and_then(|r| r.cites.iter().find(|c| c.at() == Some(at)))
        .cloned()
        .expect("a's admitting act names the word at its own stamp");
    let (mut c, same) = super::fixtures::docketed([own; 32]);
    assert_eq!(addr, same, "one word, spoken alike on a and c");
    let ruler = super::fixtures::court_key(&format!("{name} ruler"));
    let mut rules = ReceiptChain::with_signer(ruler.verifying_bytes());
    let ruling = rules
        .append(ActKind::Ruling(Ruled::Upheld), [0; 4], vec![word.clone()], [0; 32], Vec::new())
        .expect("a ruling on the ruler's line")
        .clone();
    let heard = super::fixtures::admit_line(&mut c, &ruler, &[ruling]);
    assert!(matches!(heard.as_slice(), [Admitted::Parent(_)]), "the ruling admits under its signature: {heard:?}");
    assert_eq!(c.clock(None).rulings[Ruled::Upheld.index()], [1, 0, 1, 0], "the ruling waits on a's stamp");
    (c, key, line, at, addr, word)
}

/// The key a receipt is held by on its signer's line.
fn line_key(r: &KStepReceipt) -> Vec<u8> {
    Stamp::line_key(&r.stamp.vk, r.stamp.place())
}

/// §5a.10 — A RULING WAITING ON A STAMP IS NOT HEARD THROUGH A BACK-FILLED
/// RECEIPT no signature reaches: a's admitting act, back-filled, files no
/// alias, the ruling still waits and the word stays on c's docket. a's
/// signed head, admitted, reaches the act: the alias is filed and the
/// ruling is heard and applied.
#[test]
fn backfilled_ruling_waits_for_a_signature() {
    let (mut c, key, line, at, addr, _) = waiting_ruling("reach ruling", 0xc4);
    let upheld = Ruled::Upheld.index();
    let mut budget = c.stroke();
    assert_eq!(c.admit_backfill(&line, &mut budget), (line.len(), None), "a's line links and is stored");
    assert_eq!(c.word_admitted_at(&at), None, "no alias is filed below no signature");
    assert_eq!(c.clock(None).rulings[upheld], [1, 0, 1, 0], "the ruling still waits");
    assert!(c.provisional.contains(addr.as_bytes()), "and the word stays on c's docket");
    let [.., head] = line.as_slice() else { panic!("a acted") };
    let head = head.clone();
    super::fixtures::admit_line(&mut c, &key, &[head]);
    assert_eq!(c.word_admitted_at(&at), Some(&addr), "a's signed head reaches the admitting act: the alias is filed");
    assert_eq!(c.clock(None).rulings[upheld], [1, 1, 0, 0], "and the ruling is heard and applied");
    assert!(!c.provisional.contains(addr.as_bytes()), "the word left c's docket");
}

/// §5a.9 — AN UNSIGNED BACK-FILLED LINE IS STORED AND MARKED UNREACHED: the
/// chain holds it and the signer's head moves (the catch-up keeps its
/// progress and links on through it), every receipt of it is held by its
/// place until a signature reaches it, and no alias is read off it — at
/// restore too, where the mark is derived again from what the court's own
/// line cites.
#[test]
fn unsigned_backfilled_line_is_marked_unreached() {
    let (mut c, key, line, at, _, _) = waiting_ruling("reach unreached", 0xc5);
    let mut budget = c.stroke();
    assert_eq!(c.admit_backfill(&line, &mut budget), (line.len(), None), "a's line links and is stored");
    let [.., head] = line.as_slice() else { panic!("a acted") };
    let head = head.clone();
    assert_eq!(c.known_head(&key.verifying_bytes()), Some((head.stamp.place(), head.chain_hash)), "the head held moves");
    for r in &line {
        assert!(c.unreached.contains(&line_key(r)), "{:?} is held unreached", r.stamp.place());
    }
    assert_eq!(c.word_admitted_at(&at), None, "and read by no reader");
    let mut restored = Court::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    for r in &line {
        assert!(restored.unreached.contains(&line_key(r)), "{:?} is held unreached after restore", r.stamp.place());
    }
    assert_eq!(restored.word_admitted_at(&at), None, "and no alias is re-derived off it");
}

/// §5a.9 — A SIGNATURE RELEASES WHAT IT REACHES, ONCE, IN LINE ORDER: a's
/// frame at the admitting act's place, signed, releases every held receipt
/// at or below that place and none above it, and the ruling waiting on the
/// act is heard. The head signed releases the rest. The line signed again
/// and back-filled again changes nothing: the ruling is heard once and the
/// alias filed once.
#[test]
fn signature_reaching_a_held_receipt_releases_it_once() {
    let (mut c, key, line, at, addr, _) = waiting_ruling("reach once", 0xc6);
    let upheld = Ruled::Upheld.index();
    let mut budget = c.stroke();
    c.admit_backfill(&line, &mut budget);
    let admitting = line.iter().position(|r| r.stamp == at).expect("the admitting act is on a's line");
    super::fixtures::admit_line(&mut c, &key, &line[admitting..=admitting]);
    for r in &line {
        let above = r.stamp.place() > at.place();
        assert_eq!(c.unreached.contains(&line_key(r)), above, "{:?}: released through the signed place, held above it", r.stamp.place());
    }
    assert_eq!(c.word_admitted_at(&at), Some(&addr), "the admitting act is read");
    assert_eq!(c.clock(None).rulings[upheld], [1, 1, 0, 0], "and the ruling heard");
    let [.., head] = line.as_slice() else { panic!("a acted") };
    let head = head.clone();
    super::fixtures::admit_line(&mut c, &key, &[head]);
    assert!(line.iter().all(|r| !c.unreached.contains(&line_key(r))), "the head reaches the rest");
    let read = (c.clock(None).rulings[upheld], c.word_admitted_at(&at).cloned(), c.aliases.get(addr.as_bytes()).cloned());
    super::fixtures::admit_line(&mut c, &key, &line);
    let mut budget = c.stroke();
    c.admit_backfill(&line, &mut budget);
    assert!(line.iter().all(|r| !c.unreached.contains(&line_key(r))), "nothing is held again");
    let again = (c.clock(None).rulings[upheld], c.word_admitted_at(&at).cloned(), c.aliases.get(addr.as_bytes()).cloned());
    assert_eq!(again, read, "nothing is read twice");
}

/// §5a.10 — A FORGED LINE NEVER REACHES A READER: a line anyone wrote under
/// a's key, from its genesis, admitting the word at its own stamp, links
/// from nothing and is stored unreached. It files no alias and hears no
/// ruling, at restore too, and a's own signed frames never reach it: a's
/// genesis, signed, meets it at one place, and the forged receipt is no
/// signature, so a's bond is not burned; a's signed head does not link to
/// it.
#[test]
fn forged_line_never_reaches_a_reader() {
    let (mut c, key, line, _, _, word) = waiting_ruling("reach forged", 0xc8);
    let a = key.verifying_bytes();
    let Cite::Word { chart, class, .. } = word else { panic!("a word citation") };
    let mut forged = ReceiptChain::with_signer(a);
    let fat = forged.next_stamp(&[]).expect("the forged line's first stamp");
    forged
        .append(ActKind::List, [0; 4], vec![Cite::Word { chart, class, at: fat }], [0; 32], Vec::new())
        .expect("a line anyone can write");
    let page: Vec<KStepReceipt> = forged.own().collect();
    let mut budget = c.stroke();
    assert_eq!(c.admit_backfill(&page, &mut budget), (1, None), "the forged genesis links from nothing");
    assert!(c.unreached.contains(&line_key(&page[0])), "and is held unreached");
    assert_eq!(c.word_admitted_at(&fat), None, "it files no alias");
    assert_eq!(c.clock(None).rulings[Ruled::Upheld.index()], [1, 0, 1, 0], "and the ruling still waits");
    let mut restored = Court::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    assert!(restored.unreached.contains(&line_key(&page[0])), "restored, it is held unreached");
    assert_eq!(restored.word_admitted_at(&fat), None, "and files no alias");

    let bond = [3u64, 3, 3, 3];
    let anchor = (Stamp::default(), [0; 32]);
    c.treasury_mut().genesis(a, bond, anchor);
    c.treasury_mut().enroll(a, bond, anchor);
    assert_eq!(c.treasury().bond_of(&a), Some(bond.into()), "a is bonded");
    let genesis = super::fixtures::admit_line(&mut c, &key, &line[..1]);
    assert!(matches!(genesis.as_slice(), [Admitted::Equivocation { .. }]), "a's genesis meets it at one place: {genesis:?}");
    assert_eq!(c.treasury().bond_of(&a), Some(bond.into()), "an unsigned receipt is no proof: nothing is burned");
    let [.., head] = line.as_slice() else { panic!("a acted") };
    let head = head.clone();
    let signed = super::fixtures::admit_line(&mut c, &key, &[head]);
    assert!(!matches!(signed.as_slice(), [Admitted::Parent(_)]), "a's head does not link to it: {signed:?}");
    assert!(c.unreached.contains(&line_key(&page[0])), "no signature reaches it");
    assert_eq!(c.word_admitted_at(&fat), None, "and no reader read it");
}
