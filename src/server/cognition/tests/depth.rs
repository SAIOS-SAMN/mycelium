// L15, 45.1 — DEPTH IS READ FROM THE RECEIPTS. How many independent
// organs have committed to an address, counted from the causal poset the
// chain already is, and stored nowhere.

use crate::cognition::*;
use crate::chronometry::ActKind;
use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};

fn quarter(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn");
    Word::from_letters(&vec![(q, false); n])
}

fn sentence_with(w: Word, tag: u8) -> Element {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: Address::from_bytes([TAG_FORM, tag]),
        derivation: Derivation::spelled(w),
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: Vec::new(),
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("star"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
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
fn a_word_committed_by_two_organs_reads_depth_two() {
    let mut k = Kernel::new();
    let said = sentence_with(quarter(1), 1);
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
    // address. Admitted onto the chain as a peer receipt is admitted.
    let mut peer = crate::chronometry::ReceiptChain::with_vk([7; 32]);
    let r = peer
        .append(ActKind::Install, [0; 4], addr.as_bytes(), [0; 32], Vec::new())
        .cloned()
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

/// One signer committing TWICE to one address is one vantage. Depth is
/// the distinct signers, not the receipts — an organ cannot deepen an
/// address by acting on it again (47.3).
#[test]
fn one_signer_acting_twice_is_still_one_vantage() {
    let mut k = Kernel::new();
    let addr = sentence_with(quarter(2), 5).address();
    let mut peer = crate::chronometry::ReceiptChain::with_vk([7; 32]);
    for kind in [ActKind::Install, ActKind::Ruling] {
        let r = peer
            .append(kind, [0; 4], addr.as_bytes(), [0; 32], Vec::new())
            .cloned()
            .expect("a receipt");
        k.note_attestor(&r);
    }
    assert_eq!(k.depth_of(addr.as_bytes()), 1, "two acts, one organ, one vantage");
}

/// The three kinds that COMMIT are `Install`, `Ruling` and `Adopted`. An
/// act ABOUT a word — learning it, uttering it, attesting it — is not a
/// vantage standing behind it, and does not deepen it.
#[test]
fn an_act_about_a_word_is_not_a_vantage_behind_it() {
    let mut k = Kernel::new();
    let addr = sentence_with(quarter(3), 6).address();
    let mut mind = crate::chronometry::ReceiptChain::with_vk([8; 32]);
    for kind in [ActKind::Learned, ActKind::Uttered, ActKind::Attest] {
        let r = mind
            .append(kind, [0; 4], addr.as_bytes(), [0; 32], Vec::new())
            .cloned()
            .expect("a receipt");
        k.note_attestor(&r);
    }
    assert_eq!(k.depth_of(addr.as_bytes()), 0, "acts about it commit nobody to it");
    // The same signer, now committing: one vantage, read from the kind.
    let r = mind
        .append(ActKind::Adopted, [0; 4], addr.as_bytes(), [0; 32], Vec::new())
        .cloned()
        .expect("a receipt");
    k.note_attestor(&r);
    assert_eq!(k.depth_of(addr.as_bytes()), 1, "and adopting it is one");
}

/// DERIVED, NEVER PERSISTED. The index is rebuilt from the chain the
/// court holds, so a court that reads its own receipts back reads the
/// same depths. Nothing about depth is written down (law 12).
#[test]
fn the_depth_index_rebuilds_from_the_chain_alone() {
    let mut k = Kernel::new();
    let said = sentence_with(quarter(1), 1);
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
fn the_generators_are_read_by_depth() {
    let mut k = Kernel::new();
    assert!(k.generators_by_depth().is_empty(), "a court that ruled nothing has no rows");
    let said = sentence_with(quarter(1), 1);
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
