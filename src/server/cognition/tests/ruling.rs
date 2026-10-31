// EC1, 78.33 — A PEER'S RULING APPLIES AT THE NODE THAT ADMITS ITS RECEIPT:
// the arm its kind carries, on the word its citation locates here; a
// citation not held waits under its stamp; a word the arm does not rule
// here is inert. And every payment row stands at the act that paid it.

use crate::chronometry::{ActKind, Ruled};
use crate::cognition::*;
use crate::court::admission::Admitted;

/// A court that docketed one word on its own book, bound to `own_vk`.
fn docketed(own_vk: [u8; 32]) -> (Kernel, crate::transducer::address::Address) {
    let mut k = Kernel::new();
    k.bind_identity(own_vk);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole: one body, one word (20 as ruled, L7)
    assert_eq!(super::common::speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(super::common::speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
    k.court_stroke();
    let addr = k.provisional.scoped().next().expect("docketed").0;
    (k, crate::transducer::address::Address::from(addr))
}

/// EC1 — THE RECEIPT PATH. Court a upholds a word on its own book; its
/// Ruling node carries the arm in its kind. Court b, holding the same word
/// provisionally at the same admitting stamp (the stamp a home's offer
/// carries, 78.24 (2b)), admits a's line: the arm applies HERE — the word
/// leaves b's provisional row — and b seals its own `Ruling(Upheld)` with
/// a's receipt among its parents. A court that does not hold the stamp
/// yet WAITS and is heard when a word is filed there; a court holding the
/// word only ruled is INERT for this arm. On the clock, per arm,
/// admitted = applied + awaiting + refused.
#[test]
fn admit_line_applies_peer_ruling_through_receipt() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let key = super::common::court_key("ruler");
    let (mut a, addr) = docketed(key.vk_bytes());
    let at = a.admitted_at(addr.as_bytes()).expect("the docket admitted it at an act");
    let word = a.provisional.get(addr.as_bytes()).cloned().expect("provisional");
    assert!(a.apply_verdict(addr.as_bytes(), RULING_UPHELD), "a upholds it");
    let ruling = a.chain().head().expect("a ruled").clone();
    assert_eq!(ruling.kind, ActKind::Ruling(Ruled::Upheld), "the arm rides the node's kind");
    let line: Vec<_> = a.chain().own().collect();
    let upheld = Ruled::Upheld.index();

    let mut b = Kernel::new();
    b.bind_identity([0xb0; 32]);
    b.provisional.insert(addr.clone(), word.clone());
    let mut filing = b.stroke();
    b.file_admitted(&addr, at, &mut filing);
    let heard = super::common::admit_line(&mut b, &key, &line);
    assert!(heard.iter().all(|h| matches!(h, Admitted::Parent(_))), "a's line admits whole: {heard:?}");
    assert!(!b.provisional.contains(addr.as_bytes()), "the arm applied here: the word left b's docket");
    let own_head = b.chain().head().expect("b sealed").clone();
    assert_eq!(own_head.kind, ActKind::Ruling(Ruled::Upheld), "b seals its own ruling of the same arm");
    assert!(own_head.parents.iter().any(|p| p.at == ruling.stamp), "with a's receipt among its parents");
    assert_eq!(b.clock(None).rulings[upheld], [1, 1, 0, 0], "admitted and applied");

    let mut c = Kernel::new();
    c.bind_identity([0xc0; 32]);
    super::common::admit_line(&mut c, &key, &line);
    assert_eq!(c.clock(None).rulings[upheld], [1, 0, 1, 0], "a stamp c does not hold: the ruling waits");
    c.provisional.insert(addr.clone(), word.clone());
    let mut filing = c.stroke();
    c.file_admitted(&addr, at, &mut filing);
    assert_eq!(c.clock(None).rulings[upheld], [1, 1, 0, 0], "heard when a word is filed at that stamp");
    assert!(!c.provisional.contains(addr.as_bytes()), "and applied");

    let mut d = Kernel::new();
    d.bind_identity([0xd0; 32]);
    d.basis_insert(word, [1; 32]);
    let mut filing = d.stroke();
    d.file_admitted(&addr, at, &mut filing);
    super::common::admit_line(&mut d, &key, &line);
    assert_eq!(d.clock(None).rulings[upheld], [0; 4], "a word d holds only ruled: the arm is inert, uncounted");
}

/// LD3-a — THE STAMP ALIAS CLOSES EC1'S GAP. Courts a and c each docketed
/// the same word on its own stroke, so each holds it at its OWN admitting
/// stamp, and a's ruling cites a's. When c admits a's line, a's admitting
/// act names the word by a stamp c never filed; c's fiber read yields
/// exactly one word there, so a's stamp is filed as an alias — one word,
/// two admissions, both stamps held — and a's ruling, located by it,
/// applies at c. The alias is not persisted and comes back at restore,
/// re-derived from the ledger's peer nodes.
#[test]
fn admit_line_aliases_peer_stamp_to_one_word() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let key = super::common::court_key("aliased ruler");
    let (mut a, addr) = docketed(key.vk_bytes());
    let a_stamp = a.admitted_at(addr.as_bytes()).expect("a admitted it");
    let (mut c, same) = docketed([0xc1; 32]);
    assert_eq!(addr, same, "one word, spoken alike at both courts");
    let c_stamp = c.admitted_at(addr.as_bytes()).expect("c admitted it");
    assert_ne!(c_stamp, a_stamp, "at two stamps: each court's own act");
    let before_ruling: Vec<_> = a.chain().own().collect();
    super::common::admit_line(&mut c, &key, &before_ruling);
    assert_eq!(c.word_admitted_at(&a_stamp), Some(&addr), "a's stamp is an alias of the word here");
    assert_eq!(c.admitted_at(addr.as_bytes()), Some(c_stamp), "and c's own stamp still holds it");
    let mut restored = Kernel::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    assert_eq!(restored.word_admitted_at(&a_stamp), Some(&addr), "the alias is re-derived at restore");

    assert!(a.apply_verdict(addr.as_bytes(), RULING_UPHELD), "a upholds it");
    let line: Vec<_> = a.chain().own().collect();
    let [.., ruling] = line.as_slice() else { panic!("a ruled") };
    super::common::admit_line(&mut c, &key, std::slice::from_ref(ruling));
    assert_eq!(c.clock(None).rulings[Ruled::Upheld.index()], [1, 1, 0, 0], "a's ruling, located by the alias, applies at c");
    assert!(!c.provisional.contains(addr.as_bytes()), "the word left c's docket");
}

/// LD3-a — SEVERAL WORDS ON ONE FIBER ARE NEVER CHOSEN BETWEEN. A peer's
/// admitting act naming a fiber this court holds two words on files no
/// alias: the word waits for the syllabus to teach it with its home stamp
/// (78.24 (2b)), and nothing is picked by arrival or order (rule 15).
#[test]
fn admit_line_files_no_alias_on_shared_fiber() {
    use crate::chronometry::{Cite, ReceiptChain, Stamp};
    let mut c = Kernel::new();
    c.bind_identity([0xc2; 32]);
    let chart = crate::transducer::stalk::Invariant::counts(&[2, 1]);
    // Two words on the chart, each at its own stamp.
    for (n, word) in [(1u8, [0x71u8; 3]), (2, [0x72; 3])] {
        let addr = crate::transducer::address::Address::from_bytes(word);
        let at = Stamp::new(u64::from(n), [0xc2; 32], crate::chronometry::Place { k: 0, i: u64::from(n) });
        let mut key = Vec::new();
        Cite::Word { chart: chart.clone(), class: None, at }.put_canonical(&mut key).expect("a citation");
        c.file_fiber(&addr, at, key);
    }
    assert_eq!(c.fiber_read(&chart, &None).len(), 2, "two words on the fiber");
    let key = super::common::court_key("ambiguous peer");
    let mut peer = ReceiptChain::with_vk(key.vk_bytes());
    let at = peer.next_stamp(&[]).expect("its first stamp");
    let cite = Cite::Word { chart: chart.clone(), class: None, at };
    peer.append(ActKind::List, [0; 4], vec![cite], [0; 32], Vec::new()).expect("it admits by its own act");
    let line: Vec<_> = peer.own().collect();
    super::common::admit_line(&mut c, &key, &line);
    assert_eq!(c.word_admitted_at(&at), None, "no alias: two candidates, and none is chosen");
}

/// LD3-a, 78.64 H1 — ONE ALIAS PER SIGNER. A peer that admits the one word
/// on a fiber twice, by two acts, cites it by the later stamp from then on:
/// the later alias replaces the earlier, which leaves with its depth, so a
/// word's aliases are bounded by its signers and not by the signers' acts.
#[test]
fn admit_line_later_admission_replaces_alias() {
    use crate::chronometry::{Cite, ReceiptChain, Stamp};
    let mut c = Kernel::new();
    c.bind_identity([0xc3; 32]);
    let chart = crate::transducer::stalk::Invariant::counts(&[3, 1]);
    let addr = crate::transducer::address::Address::from_bytes([0x73u8; 3]);
    let own_stamp = Stamp::new(1, [0xc3; 32], crate::chronometry::Place { k: 0, i: 1 });
    let mut fiber = Vec::new();
    Cite::Word { chart: chart.clone(), class: None, at: own_stamp }.put_canonical(&mut fiber).expect("a citation");
    c.file_fiber(&addr, own_stamp, fiber);
    let key = super::common::court_key("twice-admitting peer");
    let mut peer = ReceiptChain::with_vk(key.vk_bytes());
    let mut stamps = Vec::new();
    for _ in 0..2 {
        let at = peer.next_stamp(&[]).expect("its next stamp");
        let cite = Cite::Word { chart: chart.clone(), class: None, at };
        peer.append(ActKind::List, [0; 4], vec![cite], [0; 32], Vec::new()).expect("it admits by its own act");
        stamps.push(at);
    }
    let line: Vec<_> = peer.own().collect();
    super::common::admit_line(&mut c, &key, &line);
    let [first, second] = stamps.as_slice() else { panic!("two admissions") };
    assert_eq!(c.word_admitted_at(second), Some(&addr), "the later admission is the alias");
    assert_eq!(c.word_admitted_at(first), None, "the earlier one left");
    assert_eq!(c.aliases.get(addr.as_bytes()).map(Vec::len), Some(1), "one alias for one signer");
}

/// EC1 — EVERY PAYMENT ROW STANDS AT THE ACT THAT PAID IT. A word upheld
/// on its own book mints its released claim at the Ruling node, never at
/// the act that escrowed it, and the row carries the arm it pays (POUW).
#[test]
fn apply_verdict_mints_upheld_claim_at_ruling() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let (mut k, addr) = docketed([0x5a; 32]);
    let escrowed = k.treasury().escrow_of(addr.as_bytes()).map(|e| e.anchor.0);
    assert!(k.apply_verdict(addr.as_bytes(), RULING_UPHELD), "upheld");
    let ruling = k.chain().head().expect("ruled").stamp;
    let mints: Vec<_> = k.treasury().entries().iter().filter(|e| e.kind == crate::economy::Kind::Mint).collect();
    assert!(!mints.is_empty(), "the claim was minted");
    assert!(
        mints.iter().all(|e| e.at == ruling && e.arm == Some(crate::economy::Arm::Pouw)),
        "at the Ruling act (not the escrow's {escrowed:?}), paying POUW"
    );
    let economy = k.economy().expect("measured");
    assert_ne!(economy.paid[crate::economy::Arm::Pouw.index()], [0; 4], "the observer reads it under POUW");
    assert_eq!(economy.paid[crate::economy::Arm::Powr.index()], [0; 4], "and under no other arm");
}

/// A PEER'S STAMP FILED OVER A WORD THIS COURT HOLDS FILES NOTHING, and a
/// refusal withdraws only what its filing placed. The witness and the
/// exchange file the offering home's stamp before judging and withdrew it
/// on every refusal; where the court already held the word at its own
/// stamp the filing was a no-op and the withdrawal erased the court's own —
/// a provisional word left unstamped, every fold prefix open behind it
/// (court a, epoch 99: unstamped provisional 0 → 4, folded never).
#[test]
fn file_admitted_keeps_courts_own_stamp() {
    use crate::chronometry::Stamp;
    let (mut k, addr) = docketed([0x5a; 32]);
    let own = k.admitted_at(addr.as_bytes()).expect("the docket admitted it at an act");
    let final_before = k.final_prefix_head();
    let peer = Stamp::new(1, [0x9e; 32], crate::chronometry::Place { k: 0, i: 1 });
    let mut filing = k.stroke();
    assert!(!k.file_admitted(&addr, peer, &mut filing), "the word stands at its own stamp: this filing placed nothing");
    assert_eq!(k.admitted_at(addr.as_bytes()), Some(own), "and the court's own stamp is untouched");
    assert_eq!(k.unstamped(), [0; 5], "no row holds a word without a stamp");
    assert_eq!(k.final_prefix_head(), final_before, "and what was final stays final");
    // A word this court did not hold is filed at the peer's stamp, and that
    // filing is what a refusal withdraws.
    let (other, stranger) = docketed([0x6b; 32]);
    let word = other.provisional.get(stranger.as_bytes()).cloned().expect("provisional");
    let mut c = Kernel::new();
    c.bind_identity([0xc0; 32]);
    c.pending.insert(stranger.clone(), word);
    let mut filing = c.stroke();
    assert!(c.file_admitted(&stranger, peer, &mut filing), "a word held at no stamp here takes the peer's");
    assert_eq!(c.admitted_at(stranger.as_bytes()), Some(peer));
}

/// ONE READER OF THE WORDS A COURT REPLAYS AGAINST (3.8, ch. 8): a cavity
/// address resolves for replay and is never taught. The citation check
/// (`resolve_form`) admitted the cavity while the speech's crystal
/// (`by_address`) read the basis alone, so a sentence citing a cavity word
/// passed the check and spoke nothing at the citation. The two are one
/// reader now: whatever the check admits, the speech speaks.
#[test]
fn resolve_form_and_by_address_agree_on_cavity_word() {
    use crate::transducer::family::Crystal;
    let (mut k, addr) = docketed([0x5a; 32]);
    assert!(k.resolve_form(addr.as_bytes()).is_none(), "a provisional word resolves to nothing");
    assert!(k.by_address(addr.as_bytes()).is_none(), "and the speech reads it the same");
    k.relocate_standing(addr.as_bytes());
    assert!(k.cavity_book.contains(addr.as_bytes()), "relocated onto the cavity");
    let resolved = k.resolve_form(addr.as_bytes()).expect("a cavity address resolves for replay");
    let (_, spoken) = k.by_address(addr.as_bytes()).expect("and the speech's crystal holds it");
    assert_eq!(
        crate::transducer::sentence::form_vertex_key(&spoken),
        crate::transducer::sentence::form_vertex_key(&resolved),
        "one form, read by one reader"
    );
}
