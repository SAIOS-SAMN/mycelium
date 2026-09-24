// EC1, 78.33 — A PEER'S RULING APPLIES AT THE NODE THAT ADMITS ITS RECEIPT:
// the arm its kind carries, on the word its citation locates here; a
// citation not held waits under its stamp; a word the arm does not rule
// here is inert. And every payment row stands at the act that paid it.

use crate::chronometry::{ActKind, Ruled};
use crate::cognition::*;
use crate::court::admission::Admitted;

/// A court that docketed one word on its own book, bound to `me`.
fn docketed(me: [u8; 32]) -> (Kernel, crate::transducer::address::Address) {
    let mut k = Kernel::new();
    k.bind_identity(me);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole: one body, one word (20 as ruled, L7)
    assert_eq!(super::common::speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(super::common::speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
    k.court_stroke(super::common::one_stroke_transport);
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
fn a_peers_ruling_applies_here_through_its_receipt() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let key = super::common::court_key("ruler");
    let (mut a, addr) = docketed(key.vk_bytes());
    let at = a.admitted_at(addr.as_bytes()).expect("the docket admitted it at an act");
    let word = a.provisional.get(addr.as_bytes()).cloned().expect("provisional");
    assert!(a.apply_verdict(addr.as_bytes(), RULING_UPHELD), "a upholds it");
    let ruling = a.chain().head().expect("a ruled").clone();
    assert_eq!(ruling.kind, ActKind::Ruling(Ruled::Upheld), "the arm rides the node's kind");
    let line: Vec<_> = a.chain().own().cloned().collect();
    let upheld = Ruled::Upheld.index();

    let mut b = Kernel::new();
    b.bind_identity([0xb0; 32]);
    b.provisional.insert(addr.clone(), word.clone());
    b.file_admitted(&addr, at);
    let heard = super::common::admit_line(&mut b, &key, &line);
    assert!(heard.iter().all(|h| matches!(h, Admitted::Parent(_))), "a's line admits whole: {heard:?}");
    assert!(!b.provisional.contains(addr.as_bytes()), "the arm applied here: the word left b's docket");
    let mine = b.chain().head().expect("b sealed").clone();
    assert_eq!(mine.kind, ActKind::Ruling(Ruled::Upheld), "b seals its own ruling of the same arm");
    assert!(mine.parents.iter().any(|p| p.at == ruling.stamp), "with a's receipt among its parents");
    assert_eq!(b.clock(None).rulings[upheld], [1, 1, 0, 0], "admitted and applied");

    let mut c = Kernel::new();
    c.bind_identity([0xc0; 32]);
    super::common::admit_line(&mut c, &key, &line);
    assert_eq!(c.clock(None).rulings[upheld], [1, 0, 1, 0], "a stamp c does not hold: the ruling waits");
    c.provisional.insert(addr.clone(), word.clone());
    c.file_admitted(&addr, at);
    assert_eq!(c.clock(None).rulings[upheld], [1, 1, 0, 0], "heard when a word is filed at that stamp");
    assert!(!c.provisional.contains(addr.as_bytes()), "and applied");

    let mut d = Kernel::new();
    d.bind_identity([0xd0; 32]);
    d.basis_insert(word, [1; 32]);
    d.file_admitted(&addr, at);
    super::common::admit_line(&mut d, &key, &line);
    assert_eq!(d.clock(None).rulings[upheld], [0; 4], "a word d holds only ruled: the arm is inert, uncounted");
}

/// LD3-a — THE STAMP ALIAS CLOSES EC1'S GAP. Courts a and c each docketed
/// the same word on their own stroke, so each holds it at its OWN admitting
/// stamp, and a's ruling cites a's. When c admits a's line, a's admitting
/// act names the word by a stamp c never filed; c's fiber read yields
/// exactly one word there, so a's stamp is filed as an alias — one word,
/// two admissions, both stamps held — and a's ruling, located by it,
/// applies at c. The alias is not persisted and comes back at restore,
/// re-derived from the ledger's peer nodes.
#[test]
fn a_peers_stamp_aliases_the_one_word_on_its_fiber() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let key = super::common::court_key("aliased ruler");
    let (mut a, addr) = docketed(key.vk_bytes());
    let theirs = a.admitted_at(addr.as_bytes()).expect("a admitted it");
    let (mut c, same) = docketed([0xc1; 32]);
    assert_eq!(addr, same, "one word, spoken alike at both courts");
    let ours = c.admitted_at(addr.as_bytes()).expect("c admitted it");
    assert_ne!(ours, theirs, "at two stamps: each court's own act");
    let before_ruling: Vec<_> = a.chain().own().cloned().collect();
    super::common::admit_line(&mut c, &key, &before_ruling);
    assert_eq!(c.word_admitted_at(&theirs), Some(&addr), "a's stamp is an alias of the word here");
    assert_eq!(c.admitted_at(addr.as_bytes()), Some(ours), "and c's own stamp still holds it");
    let mut restored = Kernel::new();
    assert!(restored.restore(&c.snapshot()), "clean restore");
    assert_eq!(restored.word_admitted_at(&theirs), Some(&addr), "the alias is re-derived at restore");

    assert!(a.apply_verdict(addr.as_bytes(), RULING_UPHELD), "a upholds it");
    let line: Vec<_> = a.chain().own().cloned().collect();
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
fn several_words_on_one_fiber_file_no_alias() {
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
    let line: Vec<_> = peer.own().cloned().collect();
    super::common::admit_line(&mut c, &key, &line);
    assert_eq!(c.word_admitted_at(&at), None, "no alias: two candidates, and none is chosen");
}

/// EC1 — EVERY PAYMENT ROW STANDS AT THE ACT THAT PAID IT. A word upheld
/// on its own book mints its released claim at the Ruling node, never at
/// the act that escrowed it, and the row carries the arm it pays (POUW).
#[test]
fn an_upheld_claim_is_minted_at_the_ruling_act() {
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
    let economy = k.treasury().economy().expect("measured");
    assert_ne!(economy.paid[crate::economy::Arm::Pouw.index()], [0; 4], "the observer reads it under POUW");
    assert_eq!(economy.paid[crate::economy::Arm::Powr.index()], [0; 4], "and under no other arm");
}
