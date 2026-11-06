// MU-CLIENT §7.1 (#137); MU-RECURSION 47.1, 47.4 — SIGNED RECEIPTS REACH
// THE MIND: the signed position behind each digest is held by the court's
// own checks, and two at one place of one line under two chain hashes are
// presented as the equivocation proof.

use crate::anastome::dialogue::{self, ReceiptSection};
use crate::chronometry::{ActKind, KStepReceipt, ReceiptChain};
use crate::client::sdk::*;
use crate::court::admission::{self, Reconstructed, SignedPrefix};
use crate::court::identity::HyphaKey;
use std::io::{Read, Write};

const ORIGIN: ([u8; 32], u32) = ([7; 32], 2);

pub(super) fn key(name: &str) -> HyphaKey {
    let sk = ed25519_dalek::SigningKey::from_bytes(&HyphaKey::seed_from_name(name));
    HyphaKey { vk: sk.verifying_key(), sk }
}

/// The receipt at `signer`'s first place, measuring `lane` on 𝒮: two lanes
/// give two receipts at one place with two chain hashes.
fn first_receipt(signer: [u8; 32], lane: u64) -> KStepReceipt {
    let mut c = ReceiptChain::with_signer(signer);
    c.append(ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone()
}

/// `n` receipts of one line.
fn line(signer: [u8; 32], n: u64) -> Vec<KStepReceipt> {
    let mut c = ReceiptChain::with_signer(signer);
    (0..n).map(|i| c.append(ActKind::List, [i + 1, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone()).collect()
}

/// A 47.1 position `key` signs at `head`, over `payload`.
pub(super) fn position(key: &HyphaKey, head: &KStepReceipt, payload: &[u8]) -> ReceiptSection {
    let prefix = SignedPrefix {
        genesis: ORIGIN.0,
        epoch: ORIGIN.1,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: vec![],
        payload: payload.to_vec(),
    };
    let sig = key.sign(&prefix.bytes().expect("the prefix frames"));
    ReceiptSection { prefix, sig, receipt: head.clone() }
}

/// 47.2 STEP 2 AND 47.1's ONE CLAIM, ON THE MIND: a section is held only
/// where its signature verifies under the key its receipt names and the
/// prefix names that receipt; a forged signature or a prefix standing at
/// another place is refused by the court's own constant, and nothing is held.
#[test]
fn signed_receipt_held_only_where_the_court_would_admit_it() {
    let k = key("signer");
    let r = first_receipt(k.verifying_bytes(), 1);
    let mut held = SignedReceipts::new();
    let mut forged = position(&k, &r, b"root");
    forged.sig[0] ^= 1;
    assert!(matches!(held.hold(forged), Holding::Refused(why) if why == crate::verdict::REFUSE_SIGNATURE));
    let mut elsewhere = position(&k, &r, b"root");
    elsewhere.prefix.at.i += 1;
    elsewhere.sig = k.sign(&elsewhere.prefix.bytes().expect("frames"));
    assert!(matches!(held.hold(elsewhere), Holding::Refused(why) if why == crate::verdict::REFUSE_RECEIPT));
    assert!(held.head_of(&k.verifying_bytes()).is_none(), "nothing refused is held");
    assert!(matches!(held.hold(position(&k, &r, b"root")), Holding::Opened), "a sound section opens the line");
}

/// ONE RECEIPT SIGNED TWICE IS NOT A PROOF; A LATER PLACE ADVANCES. The
/// same head signed over two payloads (two digests at one head) is a
/// duplicate (47.2 step 3); a later place replaces the head held, and an
/// earlier one is behind it and holds nothing.
#[test]
fn signed_receipt_duplicate_advances_and_falls_behind_by_place() {
    let k = key("signer");
    let l = line(k.verifying_bytes(), 2);
    let mut held = SignedReceipts::new();
    assert!(matches!(held.hold(position(&k, &l[0], b"one")), Holding::Opened));
    assert!(matches!(held.hold(position(&k, &l[0], b"two")), Holding::Duplicate), "one receipt, two payloads");
    assert!(matches!(held.hold(position(&k, &l[1], b"one")), Holding::Advanced { from } if from == l[0].stamp.place()));
    assert!(matches!(held.hold(position(&k, &l[0], b"one")), Holding::Behind { head } if head == l[1].stamp.place()));
}

/// 47.4 READ BY THE MIND: two signed receipts under one key at one place
/// with two chain hashes are the proof, and the frame the mind builds is the
/// one the court's door reads back and its predicate admits.
#[test]
fn two_signed_receipts_at_one_place_are_the_equivocation_proof() {
    let k = key("liar");
    let (a, b) = (first_receipt(k.verifying_bytes(), 1), first_receipt(k.verifying_bytes(), 9));
    assert_eq!(a.stamp.place(), b.stamp.place(), "one place");
    let mut held = SignedReceipts::new();
    assert!(matches!(held.hold(position(&k, &a, b"root")), Holding::Opened));
    let Holding::Equivocated { at, proof } = held.hold(position(&k, &b, b"root")) else {
        panic!("two chain hashes at one place are the proof");
    };
    assert_eq!(at, a.stamp.place());
    let (pa, pb) = dialogue::decode_equivocation(&proof).expect("the door reads the frame");
    let va = admission::admit_signature(ORIGIN, &pa.prefix, &pa.receipt.stamp.vk, &pa.sig).expect("the first half verifies");
    let vb = admission::admit_signature(ORIGIN, &pb.prefix, &pb.receipt.stamp.vk, &pb.sig).expect("the second half verifies");
    let (ra, rb) = (Reconstructed::receipt(&pa.receipt).expect("re-derives"), Reconstructed::receipt(&pb.receipt).expect("re-derives"));
    let proved = admission::admit_equivocation(va, &ra, vb, &rb).expect("the court's predicate admits the proof");
    assert_eq!((proved.vk, proved.at), (k.verifying_bytes(), a.stamp.place()));
}

/// A COURT THAT ANSWERS EVERY EQUIVOCATION IT IS SHOWN `Applied`, handing
/// each frame to the test.
fn burning_court() -> (String, std::sync::mpsc::Receiver<Vec<u8>>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the court listens");
    let addr = listener.local_addr().expect("an address").to_string();
    let (shown, proofs) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(mut s) = conn else { continue };
            loop {
                let mut head = [0u8; 5];
                if s.read_exact(&mut head).is_err() {
                    break;
                }
                let Some((lane, pointed)) = crate::network::wire::read_lane(head[0]) else { break };
                let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
                let mut body = vec![0u8; len];
                if s.read_exact(&mut body).is_err() {
                    break;
                }
                let Some((body, _)) = crate::network::wire::split_point(body, pointed) else { break };
                let _ = shown.send(body);
                let reply = crate::network::wire::Delimited { lane, body: dialogue::encode_ruled(&dialogue::Ruled::Applied).expect("a ruling") };
                let _ = s.write_all(&reply.encode().expect("framed"));
            }
        }
    });
    (addr, proofs)
}

/// A digest `key` signs, bound at `bind`, carrying `section` behind it.
pub(super) fn digest(key: &HyphaKey, bind: &str, section: &ReceiptSection) -> Vec<u8> {
    let (room, roots, ruled) = ([1u64, 0, 0, 0], [[1u8; 32], [2; 32], [3; 32], [4; 32]], [5u8; 32]);
    let sig = key.sign(&dialogue::cover_signed_prefix(&roots, &ruled, &room));
    let mut body = dialogue::encode_cover_digest(&room, &roots, &ruled, &key.verifying_bytes(), bind, &[], &sig).expect("a digest writes");
    body.extend(dialogue::encode_receipt_section(&section.prefix, &section.sig, &section.receipt).expect("a section frames"));
    body
}

/// THE SESSION PRESENTS WHAT ITS DIGESTS PROVE. A hypha whose two digests
/// carry two receipts at one place of its line is caught by the mind that
/// absorbed both: the proof is spoken to the hyphas the session knows, and
/// the frame each receives is 47.4's proof against that signer.
#[test]
fn session_presents_an_equivocation_its_digests_carried() {
    let (court, proofs) = burning_court();
    let k = key("two truths");
    let (a, b) = (first_receipt(k.verifying_bytes(), 1), first_receipt(k.verifying_bytes(), 9));
    let session = Session::open(&court, Voice::named("witness"));
    session.absorb_digest(&digest(&k, &court, &position(&k, &a, b"root")));
    assert!(proofs.try_recv().is_err(), "one receipt proves nothing");
    assert!(session.signed_receipts().head_of(&k.verifying_bytes()).is_some(), "the signed receipt reached the mind");
    session.absorb_digest(&digest(&k, &court, &position(&k, &b, b"root")));
    let shown = proofs.recv().expect("the proof was presented");
    let (pa, pb) = dialogue::decode_equivocation(&shown).expect("an equivocation frame");
    assert_eq!((pa.receipt.stamp.vk, pa.receipt.stamp.place()), (pb.receipt.stamp.vk, pb.receipt.stamp.place()), "one key, one place");
    assert!(pa.receipt.chain_hash.forks(&pb.receipt.chain_hash), "two chain hashes");
}
