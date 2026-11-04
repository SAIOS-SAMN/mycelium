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
    assert_eq!(c.admit_backfill(&super::fixtures::page(&line), &mut budget), (line.len(), None), "a's line links and is stored");
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
    assert_eq!(c.admit_backfill(&super::fixtures::page(&line), &mut budget), (line.len(), None), "a's line links and is stored");
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
    c.admit_backfill(&super::fixtures::page(&line), &mut budget);
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
    c.admit_backfill(&super::fixtures::page(&line), &mut budget);
    assert!(line.iter().all(|r| !c.unreached.contains(&line_key(r))), "nothing is held again");
    let again = (c.clock(None).rulings[upheld], c.word_admitted_at(&at).cloned(), c.aliases.get(addr.as_bytes()).cloned());
    assert_eq!(again, read, "nothing is read twice");
}

/// A FORGED GENESIS ON a's LINE, admitting the word at its own stamp,
/// back-filled into c: a line anyone can write under a's key. Answers the
/// forged page and the stamp it admits the word at.
fn forged_genesis(c: &mut Court, a: [u8; 32], word: &Cite) -> (Vec<KStepReceipt>, Stamp) {
    let Cite::Word { chart, class, .. } = word else { panic!("a word citation") };
    let mut forged = ReceiptChain::with_signer(a);
    let fat = forged.next_stamp(&[]).expect("the forged line's first stamp");
    forged
        .append(ActKind::List, [0; 4], vec![Cite::Word { chart: chart.clone(), class: class.clone(), at: fat }], [0; 32], Vec::new())
        .expect("a line anyone can write");
    let page: Vec<KStepReceipt> = forged.own().collect();
    let mut budget = c.stroke();
    assert_eq!(c.admit_backfill(&super::fixtures::page(&page), &mut budget), (1, None), "the forged genesis links from nothing");
    assert!(c.unreached.contains(&line_key(&page[0])), "and is held unreached");
    (page, fat)
}

/// §5a.10 — A FORGED LINE NEVER REACHES A READER: a line anyone wrote under
/// a's key, from its genesis, admitting the word at its own stamp, links
/// from nothing and is stored unreached. It files no alias and hears no
/// ruling, at restore too, and a's signed head does not link to it: the
/// head names another receipt at the forged place as its predecessor, and
/// the forged receipt gives way (§5a.9's eviction by signature).
#[test]
fn forged_line_never_reaches_a_reader() {
    let (mut c, key, line, _, _, word) = waiting_ruling("reach forged", 0xc8);
    let (page, fat) = forged_genesis(&mut c, key.verifying_bytes(), &word);
    assert_eq!(c.word_admitted_at(&fat), None, "it files no alias");
    assert_eq!(c.clock(None).rulings[Ruled::Upheld.index()], [1, 0, 1, 0], "and the ruling still waits");
    let mut restored = Court::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    assert!(restored.unreached.contains(&line_key(&page[0])), "restored, it is held unreached");
    assert_eq!(restored.word_admitted_at(&fat), None, "and files no alias");

    let [.., head] = line.as_slice() else { panic!("a acted") };
    let head = head.clone();
    let signed = super::fixtures::admit_line(&mut c, &key, std::slice::from_ref(&head));
    assert!(!matches!(signed.as_slice(), [Admitted::Parent(_)]), "a's head does not link to it: {signed:?}");
    let forged = crate::chronometry::Parent { at: page[0].stamp, hash: page[0].chain_hash };
    assert_eq!(head.prev.map(|p| p.at.place()), Some(page[0].stamp.place()), "a's head names a predecessor at the forged place");
    assert!(!c.chain().holds(&forged), "no signature reaches it: it gave way to the head's predecessor");
    assert!(!c.unreached.contains(&line_key(&page[0])), "and nothing waits there");
    assert_eq!(c.word_admitted_at(&fat), None, "and no reader read it");
}

/// §5a.9 — EVICTION BY SIGNATURE: a forged genesis stored first at a's
/// place holds it unreached. a's own genesis, signed, contradicts it there:
/// the forged receipt gives way and a's genesis is admitted in its place.
/// a's signed line then advances at c and its admitting act is read, and a
/// restored c holds a's genesis at the place.
#[test]
fn signed_line_evicts_forged_unreached_genesis() {
    let (mut c, key, line, at, addr, word) = waiting_ruling("reach evict", 0xc9);
    let a = key.verifying_bytes();
    let (page, _) = forged_genesis(&mut c, a, &word);
    let genesis = super::fixtures::admit_line(&mut c, &key, &line[..1]);
    assert!(matches!(genesis.as_slice(), [Admitted::Parent(_)]), "a's genesis takes the forged one's place: {genesis:?}");
    assert_ne!(page[0].chain_hash, line[0].chain_hash, "two receipts at one place");
    assert_eq!(
        c.chain().at(&a, line[0].stamp.place()).map(|r| r.chain_hash),
        Some(line[0].chain_hash),
        "the place holds a's signed genesis"
    );
    assert!(!c.unreached.contains(&line_key(&page[0])), "and nothing waits there");

    let rest = super::fixtures::admit_line(&mut c, &key, &line[1..]);
    assert!(rest.iter().all(|v| matches!(v, Admitted::Parent(_))), "a's signed line advances: {rest:?}");
    assert_eq!(c.word_admitted_at(&at), Some(&addr), "a's admitting act is read");
    let mut restored = Court::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    assert_eq!(
        restored.chain().at(&a, line[0].stamp.place()).map(|r| r.chain_hash),
        Some(line[0].chain_hash),
        "restored, the place holds a's signed genesis"
    );
}

/// A LINE OF `n` ACTS under `key`, citing nothing, its operator `op`.
fn plain_line(key: &HyphaKey, n: usize, op: u8) -> (ReceiptChain, Vec<KStepReceipt>) {
    let mut chain = ReceiptChain::with_signer(key.verifying_bytes());
    let line = (0..n)
        .map(|_| chain.append(ActKind::List, [0; 4], Vec::new(), [op; 32], Vec::new()).expect("an act").clone())
        .collect();
    (chain, line)
}

/// §5a.9 — EVICTION NEVER REACHES WHAT A SIGNATURE REACHED: a's line held
/// at c through r1, reached by a's signed r1, then a fork f2, f3 linking
/// on r1 back-filled and held unreached. a's signed r2 contradicts f2:
/// f3 and f2 leave and r2 is admitted, while r0 and r1 stay, and a's bond
/// is not burned, since the fork was no signature. A frame a signed
/// against the reached r1 evicts nothing: it is an equivocation proved by
/// two signatures, and the bond burns.
#[test]
fn evict_contradicted_keeps_reached_receipts() {
    let key = super::fixtures::court_key("reach keeps reached");
    let a = key.verifying_bytes();
    let (mut honest, real) = plain_line(&key, 2, 9);
    let mut fork = honest.clone();
    let r2 = honest.append(ActKind::List, [0; 4], Vec::new(), [9; 32], Vec::new()).expect("r2").clone();
    let forged: Vec<KStepReceipt> =
        (0..2).map(|_| fork.append(ActKind::List, [0; 4], Vec::new(), [8; 32], Vec::new()).expect("a forged act").clone()).collect();
    let mut c = Court::new();
    let mut budget = super::fixtures::budget();
    assert_eq!(c.admit_backfill(&super::fixtures::page(&real), &mut budget), (2, None), "r0, r1 link and are stored");
    super::fixtures::admit_line(&mut c, &key, &real[1..]);
    assert!(real.iter().all(|r| !c.unreached.contains(&line_key(r))), "a's signed r1 reaches r0 and r1");
    assert_eq!(c.admit_backfill(&super::fixtures::page(&forged), &mut budget), (2, None), "the fork links on r1 and is stored");
    assert!(forged.iter().all(|r| c.unreached.contains(&line_key(r))), "held unreached");
    let bond = [3u64, 3, 3, 3];
    let anchor = (Stamp::default(), [0; 32]);
    c.treasury_mut().genesis(a, bond, anchor);
    c.treasury_mut().enroll(a, bond, anchor);
    assert_eq!(c.treasury().bond_of(&a), Some(bond.into()), "a is bonded");

    let signed = super::fixtures::admit_line(&mut c, &key, std::slice::from_ref(&r2));
    assert!(matches!(signed.as_slice(), [Admitted::Parent(_)]), "r2 takes the fork's place: {signed:?}");
    assert_eq!(c.treasury().bond_of(&a), Some(bond.into()), "an unsigned fork is no proof: nothing is burned");
    for r in real.iter().chain([&r2]) {
        assert_eq!(c.chain().at(&a, r.stamp.place()).map(|h| h.chain_hash), Some(r.chain_hash), "{:?} stands", r.stamp.place());
    }
    assert!(c.chain().at(&a, forged[1].stamp.place()).is_none(), "the fork's top left");
    assert!(forged.iter().all(|r| !c.unreached.contains(&line_key(r))), "and nothing of the fork waits");

    let mut other = ReceiptChain::with_signer(a);
    other.append(ActKind::List, [0; 4], Vec::new(), [7; 32], Vec::new()).expect("a's other genesis");
    let against = other.append(ActKind::List, [0; 4], Vec::new(), [7; 32], Vec::new()).expect("a's other r1").clone();
    let verdict = super::fixtures::admit_line(&mut c, &key, &[against]);
    assert!(matches!(verdict.as_slice(), [Admitted::Equivocation { .. }]), "a signed fork of the reached r1: {verdict:?}");
    assert_eq!(c.chain().at(&a, real[1].stamp.place()).map(|h| h.chain_hash), Some(real[1].chain_hash), "r1 stays");
    assert_ne!(c.treasury().bond_of(&a), Some(bond.into()), "and two signatures at one place burn the bond");
}

/// D2.5 — THE UNREACHED QUEUE IS BOUNDED BY THE PROCESS'S MEMORY: a court
/// whose capacity is eight reads (`C = ⌊M_proc / b⌋` at eight bytes and one
/// byte a read) holds eight back-filled receipts unreached, and refuses the
/// ninth by name: it is not stored and the page stops. A signature that
/// reaches the eight releases them, and the ninth is then held.
#[test]
fn unreached_queue_refuses_past_process_memory() {
    use crate::physics::{Calibration, Memory, MemorySource, Physics};
    let one = std::num::NonZeroU64::MIN;
    let eight = std::num::NonZeroU64::new(8).expect("eight bytes");
    let calibration = Calibration::of(Memory::of(eight, MemorySource::CgroupMax), one, one).expect("eight reads of one byte");
    let key = super::fixtures::court_key("reach bounded");
    let (_, line) = plain_line(&key, 9, 9);
    let mut c = Court::new();
    c.physics = Physics::calibrated(&calibration);
    let mut budget = super::fixtures::budget();
    assert_eq!(c.admit_backfill(&super::fixtures::page(&line), &mut budget), (8, None), "eight are held, the page stops at the ninth");
    assert_eq!(c.unreached.len(), 8, "eight held unreached");
    assert_eq!(c.unreached_refusal(), Some(crate::court::verdict::REFUSE_UNREACHED_PAST_MEMORY), "a ninth is refused by name");
    assert!(c.chain().at(&key.verifying_bytes(), line[8].stamp.place()).is_none(), "and not stored");
    super::fixtures::admit_line(&mut c, &key, &line[7..8]);
    assert_eq!(c.unreached.len(), 0, "a's signed eighth reaches all eight");
    assert_eq!(c.admit_backfill(&super::fixtures::page(&line[8..]), &mut budget), (1, None), "and the ninth is held");
}

/// §5a.9 — A BACK-FILLED FOLD FILES ITS ROOT ONCE A SIGNATURE REACHES IT:
/// e's line, folded at its close, back-filled into c with its `Fold`, files
/// no root for e — at restore too — and e's signed fold, admitted, files
/// it.
#[test]
fn fold_root_waits_for_a_signature() {
    let key = super::fixtures::court_key("reach fold root");
    let e = key.verifying_bytes();
    let (mut chain, mut line) = plain_line(&key, 2, 9);
    let (_, frame) = chain.open_orbit().expect("an orbit is open");
    let shut = chain.append(ActKind::Orbit, frame, Vec::new(), [9; 32], Vec::new()).expect("the close").clone();
    let (_, root) = chain.fold_before(shut.stamp.place()).expect("the orbit folds");
    let cites = vec![Cite::Root { at: Stamp::new(root.height, e, root.at), hash: root.hash }];
    let folded = chain.append(ActKind::Fold, [0; 4], cites, [9; 32], Vec::new()).expect("the fold node").clone();
    line.extend([shut.clone(), folded.clone()]);
    let mut c = Court::new();
    let mut budget = super::fixtures::budget();
    assert_eq!(c.admit_backfill(&super::fixtures::page(&line), &mut budget), (line.len(), None), "e's line and its fold are stored");
    assert_eq!(c.chain().peer_root(&e), None, "the fold's root waits");
    let mut restored = Court::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    assert_eq!(restored.chain().peer_root(&e), None, "restored, it still waits");
    super::fixtures::admit_line(&mut c, &key, &[folded]);
    assert_eq!(c.chain().peer_root(&e), Some(shut.stamp.place()), "e's signed fold files its root");
}
