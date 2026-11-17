// 78.185 — a hypha attached to the cover by an admitted adjunction, and
// detached by a retraction.

use crate::anastome::dialogue::adjunction::{decode_cover_reply, encode_adjunction, encode_cover_reply, Adjunction, CoverRefused, Covered, Retraction, TetherStated};
use crate::anastome::dialogue::crossing_record::still;
use crate::anastome::dialogue::domain::{decode_domain_request, DomainRequest};
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(b: u8) -> [u8; 32] {
    key(b).verifying_key().to_bytes()
}

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

/// A tether to `node` by the identity motion, at rest or off it.
fn tether(node: u8, rest: bool) -> TetherStated {
    let pole = if rest { q(-1, 3) } else { q(-1, 5) };
    TetherStated { node: vk(node), g: crate::transducer::element::identity_versor(), planes: vec![(q(1, 3), pole)] }
}

fn court() -> Court {
    let mut k = Court::new();
    k.hold_genesis_hyphas(&[vk(1), vk(2), vk(3)]);
    k
}

/// AN ENTRANT TETHERED AT REST TO TWO GENESIS HYPHAS IS ATTACHED, reads back
/// off the wire whole, survives a restart, and is retracted with its stake
/// returned. Off rest it is untethered; a node the nerve does not hold is
/// named; a genesis hypha is the ground and is never retracted.
#[test]
fn a_hypha_attaches_by_adjunction_and_detaches_by_retraction() {
    let mut k = court();
    let entrant = key(9);
    let adj = Adjunction { hypha: vk(9), tethers: vec![tether(1, true), tether(2, false)], nonce: [1; 32] }.sign(still(), &entrant).expect("signs");
    let wire = encode_adjunction(&adj).expect("writes");
    let Some(DomainRequest::Adjoin(back)) = decode_domain_request(&wire) else { panic!("an adjunction") };
    assert_eq!(*back, adj, "the act reads back whole");

    let off = Adjunction { hypha: vk(9), tethers: vec![tether(1, false)], nonce: [2; 32] }.sign(still(), &entrant).expect("signs");
    assert_eq!(k.adjoin(&off), Covered::Refused(CoverRefused::Untethered), "a tether off rest links nothing");
    let stray = Adjunction { hypha: vk(9), tethers: vec![tether(7, true)], nonce: [3; 32] }.sign(still(), &entrant).expect("signs");
    assert_eq!(k.adjoin(&stray), Covered::Refused(CoverRefused::NoNode(vk(7))));
    let forged = Adjunction { hypha: vk(9), tethers: vec![tether(1, true)], nonce: [4; 32] }.sign(still(), &key(8)).expect("signs");
    assert_eq!(k.adjoin(&forged), Covered::Refused(CoverRefused::Unsigned), "signed by the hypha's own key alone");

    let sealed = k.adjoin(&adj);
    assert!(matches!(sealed, Covered::Sealed { .. }), "{sealed:?}");
    assert_eq!(decode_cover_reply(&encode_cover_reply(&sealed)), Some(sealed));
    assert_eq!(k.attached_hyphas(), vec![vk(9)], "attached, read off the admitted adjunction");
    assert_eq!(k.adjoin(&adj), Covered::Refused(CoverRefused::Held), "once");

    let booted = Court::new().reconstruct(&k.snapshot()).expect("a clean reconstruction").into_object();
    assert_eq!(booted.attached_hyphas(), vec![vk(9)], "the attached hypha survives a restart");

    let ground = Retraction { hypha: vk(1), nonce: [5; 32] }.sign(still(), &key(1)).expect("signs");
    assert_eq!(k.retract(&ground), Covered::Refused(CoverRefused::Genesis), "the ground is never retracted");
    let leave = Retraction { hypha: vk(9), nonce: [6; 32] }.sign(still(), &entrant).expect("signs");
    assert!(matches!(k.retract(&leave), Covered::Sealed { .. }));
    assert!(k.attached_hyphas().is_empty(), "detached");
    assert_eq!(k.retract(&leave), Covered::Refused(CoverRefused::Held), "once");
}
