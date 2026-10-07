// 78.128 — A WORMHOLE BETWEEN DOMAINS: a mouth pair built by two domains'
// reserves; traversals through it alone, tolled to its keeper; closed by
// either, what is in flight returning; loops of mouths read as curvature.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Mouth, Offer, State};
use crate::server::cognition::Kernel;
use crate::transducer::cga::Mv;
use ed25519_dalek::SigningKey;
use num_rational::BigRational;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// Domain `d`'s key and its reserve tier.
fn domain(d: u8) -> SigningKey {
    key(d)
}
fn reserve(d: u8) -> SigningKey {
    key(d + 20)
}

/// Domains 4, 5 and 6, each its own reserve, and a second tier of 4's (24 + 1).
fn founded() -> Kernel {
    let mut k = Kernel::new();
    for d in [4u8, 5, 6] {
        let mut tiers = vec![vk(&reserve(d))];
        if d == 4 {
            tiers.push(vk(&key(30)));
            tiers.sort_unstable();
        }
        let g = DomainGenesis { domain: vk(&domain(d)), tiers };
        assert!(matches!(k.admit_domain(&g.sign(&domain(d))), Founded::Sealed { .. }));
        let r = Reservation { domain: vk(&domain(d)), tier: vk(&reserve(d)), nonce: [0; 32] };
        assert!(matches!(k.designate_reserve(&r.sign(&domain(d))), Founded::Reserved { .. }));
    }
    k
}

fn one() -> Mv {
    crate::transducer::element::identity_versor()
}

/// A turn in the `e₁e₂` plane: `2 + e₁₂`, a versor.
fn turn() -> Mv {
    Mv::scalar(BigRational::from_integer(2.into())).add(&Mv::blade(0b0011, BigRational::from_integer(1.into())))
}

fn mouth(a: u8, b: u8, from_frame: Mv, to_frame: Mv, toll: [u64; 4]) -> Mouth {
    Mouth { from_tier: vk(&reserve(a)), to_tier: vk(&reserve(b)), keeper: vk(&reserve(a)), from_frame, to_frame, toll, nonce: [a; 32] }
}

fn open(k: &mut Kernel, m: Mouth, a: u8, b: u8) -> [u8; 32] {
    let id = m.id();
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(a))), Handed::Sealed { .. }));
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(b))), Handed::Sealed { state: State::Accepted, .. }));
    id
}

/// One of matter from domain `a`'s reserve to `b`'s.
fn cargo(a: u8, b: u8, nonce: u8) -> Offer {
    Offer {
        from_tier: vk(&reserve(a)),
        to_tier: vk(&reserve(b)),
        out_nets: vec![([1; 32], [-1, 0, 0, 0])],
        in_nets: vec![([2; 32], [1, 0, 0, 0])],
        charter: [0; 32],
        nonce: [nonce; 32],
    }
}

/// THE BUILDERS ARE THE ANCIENTS, THE FRAMES REAL: a pair offered by a tier
/// that is not its domain's reserve, to a tier that is not the other's,
/// within one domain, kept by a stranger, or with a zero frame — refused.
#[test]
fn a_mouth_pair_is_built_by_two_reserves_on_real_frames() {
    let mut k = founded();
    let offer = |k: &mut Kernel, m: Mouth, by: &SigningKey| match k.handoff(&Handoff::Mouth(Box::new(m)).sign(by)) {
        Handed::Refused(r) => r,
        other => panic!("refused: {other:?}"),
    };
    let not_reserve = Mouth { from_tier: vk(&key(30)), ..mouth(4, 5, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, not_reserve, &key(30)), HandoffRefusal::NotReserve);
    let at_home = Mouth { to_tier: vk(&key(30)), ..mouth(4, 5, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, at_home, &reserve(4)), HandoffRefusal::Unbound);
    let stranger = Mouth { keeper: [9; 32], ..mouth(4, 5, one(), one(), [0; 4]) };
    assert_eq!(offer(&mut k, stranger, &reserve(4)), HandoffRefusal::BadKeeper);
    assert_eq!(offer(&mut k, mouth(4, 5, Mv::zero(), one(), [0; 4]), &reserve(4)), HandoffRefusal::Degenerate, "no horizon carried");
    assert_eq!(offer(&mut k, mouth(4, 5, one(), one(), [0; 4]), &reserve(5)), HandoffRefusal::NotTheTier);
}

/// THROUGH THE MOUTH ALONE, TOLLED TO ITS KEEPER: no mouth, no crossing; a
/// mouth joining other domains is not this crossing's; through the open
/// mouth either way, the toll escrowed from the sender and paid to the keeper
/// on acceptance.
#[test]
fn a_traversal_crosses_through_an_open_mouth_and_pays_the_keeper() {
    let mut k = founded();
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve(5)), [10, 0, 0, 0], anchor);
    assert!(matches!(k.handoff(&Handoff::Offer(Box::new(cargo(4, 5, 1))).sign(&reserve(4))), Handed::Refused(HandoffRefusal::NoMouth)));
    let id = open(&mut k, mouth(4, 5, one(), turn(), [3, 0, 0, 0]), 4, 5);
    let other = open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    let wrong = Handoff::Traverse { offer: Box::new(cargo(4, 5, 1)), mouth: other };
    assert!(matches!(k.handoff(&wrong.sign(&reserve(4))), Handed::Refused(HandoffRefusal::NotTheMouth)));
    // Y to X, back through the mouth X built: the toll is X's to keep.
    let back = Handoff::Traverse { offer: Box::new(cargo(5, 4, 2)), mouth: id };
    let Handed::Sealed { id: crossing, .. } = k.handoff(&back.sign(&reserve(5))) else { panic!("crossing") };
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(5)))[0], 7, "the toll escrowed from the sender");
    assert!(matches!(k.handoff(&Handoff::Accept { id: crossing }.sign(&reserve(4))), Handed::Sealed { state: State::Accepted, .. }));
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4)))[0], 3, "paid to the keeper");
}

/// CLOSED BY EITHER BUILDER, WHAT IS IN FLIGHT RETURNS: a crossing offered
/// and unanswered returns when the mouth closes, its toll refunded; a closed
/// mouth carries nothing.
#[test]
fn a_mouth_closed_returns_what_is_in_flight() {
    let mut k = founded();
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve(4)), [5, 0, 0, 0], anchor);
    let id = open(&mut k, mouth(4, 5, one(), one(), [2, 0, 0, 0]), 4, 5);
    let go = Handoff::Traverse { offer: Box::new(cargo(4, 5, 1)), mouth: id };
    let Handed::Sealed { id: crossing, .. } = k.handoff(&go.sign(&reserve(4))) else { panic!() };
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4)))[0], 3);
    assert!(matches!(k.handoff(&Handoff::Close { id }.sign(&reserve(5))), Handed::Sealed { state: State::Returned, .. }), "either builder");
    assert_eq!(k.handoff_read(&crossing), Some((State::Returned, false, false)), "in flight, returned");
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4)))[0], 5, "its toll refunded");
    let again = Handoff::Traverse { offer: Box::new(cargo(4, 5, 2)), mouth: id };
    assert!(matches!(k.handoff(&again.sign(&reserve(4))), Handed::Refused(HandoffRefusal::NoMouth)));
}

/// LOOPS OF MOUTHS ARE CURVATURE, RECORDED: three domains joined in a cycle
/// whose chart transitions compose to the identity — nothing recorded; one
/// whose transitions turn — the loop recorded at the mouth that closed it,
/// with its exact holonomy, and traversal still open.
#[test]
fn a_loop_of_mouths_that_turns_is_recorded_as_curvature() {
    let mut k = founded();
    let r = turn();
    open(&mut k, mouth(4, 5, r.clone(), one(), [0; 4]), 4, 5);
    open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    let closing = open(&mut k, mouth(6, 4, one(), one(), [0; 4]), 6, 4);
    let loops = k.mouth_loops();
    let [l] = loops.as_slice() else { panic!("one loop: {loops:?}") };
    assert_eq!(l.mouth, closing, "recorded at the mouth that closed it");
    assert!(!l.holonomy.projective_eq(&one()), "it turns");
    assert!(matches!(
        k.handoff(&Handoff::Traverse { offer: Box::new(cargo(6, 4, 1)), mouth: closing }.sign(&reserve(6))),
        Handed::Sealed { .. }
    ), "curvature never refuses a crossing");
    // Consistent: the third mouth carries the turn back.
    let mut k = founded();
    open(&mut k, mouth(4, 5, r.clone(), one(), [0; 4]), 4, 5);
    open(&mut k, mouth(5, 6, one(), one(), [0; 4]), 5, 6);
    open(&mut k, mouth(6, 4, one(), r, [0; 4]), 6, 4);
    assert!(k.mouth_loops().is_empty(), "the transitions compose to the identity");
}
