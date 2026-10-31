// L15, 45.1 — DEPTH IS READ FROM THE RECEIPTS. How many independent
// organs have committed to an address, counted from the causal poset the
// chain already is, and stored nowhere.

use crate::cognition::*;
use crate::chronometry::ActKind;
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Form, Vertex};

fn turn_90(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn");
    Word::from_letters(&vec![(q, false); n])
}

fn sentence_with(w: Word, tag: u8) -> Element {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        Derivation::spelled(w),
        // Its own chart: an address is derived from its coordinates (54.2).
        crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        [0],
        None,
    );
    let c = f.stand_member(&v, 0).expect("a word vertex stands");
    f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c, None).expect("star");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    as_element_form(&f, [1, 0, 0, 0]).expect("a tape")
}

/// L15 — A WORD INSTALLED ON TWO VANTAGES READS DEPTH 2. The court's own
/// install is one signer; a peer's receipt naming the same address is the
/// second, and depth counts the signers rather than the receipts.
///
/// `Standing` is untouched by any of it: the word is `Ruled` here at
/// depth 1 and still `Ruled` at depth 2. Arity is depth, not standing.
#[test]
fn depth_of_counts_two_signers_as_two() {
    let mut k = Court::new();
    let said = sentence_with(turn_90(1), 1);
    let addr = said.address();
    assert_eq!(k.depth_of(addr.as_bytes()), 0, "nothing has committed to it yet");

    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&said)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([9; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [9; 32]);
    assert!(k.ruled_contains(addr.as_bytes()), "this court ruled it");
    let here = k.depth_of(addr.as_bytes());
    assert!(here >= 1, "this court's own commitment is one signer: {here}");
    let standing = k.ruled_contains(addr.as_bytes());

    // A SECOND ORGAN, with its own verifying key, committing to the same
    // word — cited by the stamp that admitted it (78.24).
    let at = k.admitted_at(addr.as_bytes()).expect("the word was admitted here");
    let mut peer = crate::chronometry::ReceiptChain::with_vk([7; 32]);
    let r = peer
        .append(ActKind::Install, [0; 4], vec![cite(at)], [0; 32], Vec::new())
        .expect("the peer's receipt");
    k.note_attestor(&r);

    assert_eq!(
        k.depth_of(addr.as_bytes()),
        here + 1,
        "a second signer is a second vantage, and depth counts signers"
    );
    assert_eq!(
        k.ruled_contains(addr.as_bytes()),
        standing,
        "and standing did not move: depth is not standing"
    );
}

/// A citation of a word admitted at `at`, its fibre the root chart.
fn cite(at: crate::chronometry::Stamp) -> crate::chronometry::Cite {
    crate::chronometry::Cite::Word { chart: crate::transducer::stalk::Invariant::default(), class: None, at }
}

/// One signer committing TWICE to one word is one vantage. Depth is the
/// distinct signers, not the receipts — an organ cannot deepen a word by
/// acting on it again (47.3). Depth is read at the admitting stamp.
#[test]
fn depth_at_counts_repeat_signer_once() {
    let mut k = Court::new();
    let at = crate::chronometry::Stamp { h: 3, vk: [4; 32], k: 2, i: 0 };
    // A word stands at the stamp: depth is read only where one does.
    k.held_at.insert(at.key(), crate::transducer::address::Address::from(vec![9]));
    let mut peer = crate::chronometry::ReceiptChain::with_vk([7; 32]);
    for kind in [ActKind::Install, ActKind::Ruling(crate::chronometry::Ruled::Upheld)] {
        let r = peer.append(kind, [0; 4], vec![cite(at)], [0; 32], Vec::new()).expect("a receipt");
        k.note_attestor(&r);
    }
    assert_eq!(k.depth_at(&at), 1, "two acts, one organ, one vantage");
}

/// DEPTH IS FILED ONLY WHERE A WORD STANDS: a peer's act citing a stamp no
/// word of this court stands at files nothing, so the index never holds a
/// stamp nothing reads it at.
#[test]
fn note_attestor_files_only_where_word_stands() {
    let mut k = Court::new();
    let at = crate::chronometry::Stamp { h: 3, vk: [4; 32], k: 2, i: 0 };
    let mut peer = crate::chronometry::ReceiptChain::with_vk([7; 32]);
    let r = peer.append(ActKind::Install, [0; 4], vec![cite(at)], [0; 32], Vec::new()).expect("a receipt");
    k.note_attestor(&r);
    assert!(k.attestors.is_empty(), "no word stands at the stamp, and nothing is filed there");
}

/// The three kinds that COMMIT are `Install`, `Ruling` and `Adopted`. An
/// act ABOUT a word — listing it, settling against it, attesting it — is not
/// a vantage standing behind it, and does not deepen it. (78.94: the acts
/// about a word a court seals; learning and uttering are sealed by nothing
/// since the voices' chains retired.)
#[test]
fn depth_at_ignores_acts_about_word() {
    let mut k = Court::new();
    let at = crate::chronometry::Stamp { h: 5, vk: [4; 32], k: 4, i: 0 };
    // A word stands at the stamp: depth is read only where one does.
    k.held_at.insert(at.key(), crate::transducer::address::Address::from(vec![9]));
    let mut mind = crate::chronometry::ReceiptChain::with_vk([8; 32]);
    for kind in [ActKind::List, ActKind::Settled, ActKind::Attest] {
        let r = mind.append(kind, [0; 4], vec![cite(at)], [0; 32], Vec::new()).expect("a receipt");
        k.note_attestor(&r);
    }
    assert_eq!(k.depth_at(&at), 0, "acts about it commit nobody to it");
    let r = mind.append(ActKind::Adopted, [0; 4], vec![cite(at)], [0; 32], Vec::new()).expect("a receipt");
    k.note_attestor(&r);
    assert_eq!(k.depth_at(&at), 1, "and adopting it is one");
}

/// DERIVED, NEVER PERSISTED. The index is rebuilt from the chain the
/// court holds, so a court that reads its own receipts back reads the
/// same depths. Nothing about depth is written down (law 12).
#[test]
fn rebuild_attestors_rederives_depth_from_chain() {
    let mut k = Court::new();
    let said = sentence_with(turn_90(1), 1);
    let addr = said.address();
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&said)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    let before = k.depth_of(addr.as_bytes());
    assert!(before >= 1, "the court committed to it");

    // Thrown away and read back off the chain, which is the only place it
    // ever was.
    k.attestors = crate::network::nerve::Keyed::new();
    assert_eq!(k.depth_of(addr.as_bytes()), 0, "the index held it, not the chain's reading");
    k.rebuild_attestors();
    assert_eq!(
        k.depth_of(addr.as_bytes()),
        before,
        "and the chain alone re-derives every depth"
    );
}

/// 45.9: the instrument's reading — the ruled cover by depth, as a Vec of
/// relations. Never a total and never an average (law 5).
#[test]
fn generators_by_depth_reads_ruled_cover() {
    let mut k = Court::new();
    assert!(k.generators_by_depth().is_empty(), "a court that ruled nothing has no rows");
    let said = sentence_with(turn_90(1), 1);
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&said)),
        vec![crate::anastome::dialogue::SPEECH_STOOD]
    );
    k.rule_all_provisionals();
    let rows = k.generators_by_depth();
    assert!(!rows.is_empty(), "and one that ruled something has one");
    assert!(
        rows.iter().all(|(_, n)| *n > 0),
        "every row counts what it names: {rows:?}"
    );
}
