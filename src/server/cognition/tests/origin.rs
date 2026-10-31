// 78.132 — UNIVERSE GENESIS: one network universe = one Big Bang = one domain
// (78.86). A domain's origin — the commitment to its Big Bang record — signed
// by the domain's key, held once and never edited, sealed on the line by its
// coordinates, read back with the stamp and receipt that prove precedence.

use crate::anastome::dialogue::domain::{
    decode_domain_reply, decode_domain_request, decode_origin_reading, encode_domain_reply, encode_origin, encode_origin_read,
    encode_origin_reading, DomainGenesis, DomainRequest, Founded, Origin,
};
use crate::chronometry::receipt::Cite;
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn verifying(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn domain() -> SigningKey {
    key(4)
}

fn founded() -> Court {
    let mut k = Court::new();
    let g = DomainGenesis { domain: verifying(&domain()), tiers: vec![verifying(&key(21))] };
    assert!(matches!(k.admit_domain(&g.sign(&domain())), Founded::Sealed { .. }));
    k
}

fn origin(commitment: u8) -> Origin {
    Origin { domain: verifying(&domain()), kind: 1, version: 3, commitment: [commitment; 32], universe: None }
}

/// AN ORIGIN IS THE DOMAIN'S, ONCE: another key's signature, a domain not
/// founded here, and a second origin — even the same one — are refused by
/// name; the first is sealed by its coordinates, read back with its stamp and
/// receipt, and restored.
#[test]
fn record_origin_seals_once_never_edited() {
    let mut k = founded();
    let d = verifying(&domain());
    assert_eq!(k.origin_of(&d), None, "a domain founded from keys alone has no origin");
    assert_eq!(k.record_origin(&origin(9).sign(&key(5))), Founded::Unsigned);
    let stranger = Origin { domain: verifying(&key(5)), ..origin(9) }.sign(&key(5));
    assert_eq!(k.record_origin(&stranger), Founded::NotFounded);
    let Founded::Originated { at, receipt } = k.record_origin(&origin(9).sign(&domain())) else { panic!("sealed") };
    let head = k.chain.head().expect("the origin on the line");
    assert_eq!(head.cites, vec![Cite::Origin { domain: d, kind: 1, version: 3, commitment: [9; 32] }]);
    let held = k.origin_of(&d).expect("held");
    assert_eq!((held.kind, held.version, held.commitment, held.at, held.receipt), (1, 3, [9; 32], at, receipt));
    assert_eq!(k.record_origin(&origin(9).sign(&domain())), Founded::AlreadyOriginated, "once");
    assert_eq!(k.record_origin(&origin(8).sign(&domain())), Founded::AlreadyOriginated, "never an edit");
    assert_eq!(k.origin_of(&d), Some(held), "unchanged");
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.origin_of(&d), Some(held), "the origin survives a restart");
    assert_eq!(booted.record_origin(&origin(8).sign(&domain())), Founded::AlreadyOriginated, "and still once");
}

/// THE WIRE: the origin and its read decode to what was encoded, a byte left
/// over is none, and the replies and the reading round-trip.
#[test]
fn origin_act_and_read_round_trip_on_wire() {
    let signed = origin(9).sign(&domain());
    let frame = encode_origin(&signed);
    assert_eq!(decode_domain_request(&frame), Some(DomainRequest::Origin(signed)));
    let mut long = frame;
    long.push(0);
    assert_eq!(decode_domain_request(&long), None, "a byte left over is none");
    let d = verifying(&domain());
    assert_eq!(decode_domain_request(&encode_origin_read(d)), Some(DomainRequest::ReadOrigin(d)));
    let mut k = founded();
    let sealed = k.record_origin(&origin(9).sign(&domain()));
    for f in [sealed, Founded::AlreadyOriginated] {
        assert_eq!(decode_domain_reply(&encode_domain_reply(&f)), Some(f));
    }
    let held = k.origin_of(&d);
    assert_eq!(decode_origin_reading(&encode_origin_reading(held.as_ref())), Some(held));
    assert_eq!(decode_origin_reading(&encode_origin_reading(None)), Some(None));
    assert_eq!(decode_origin_reading(&[2]), None);
    let cite = Cite::Origin { domain: d, kind: 1, version: 3, commitment: [9; 32] };
    let mut b = Vec::new();
    cite.put_canonical(&mut b).expect("written");
    let mut p = 0;
    assert_eq!(Cite::take_canonical(&b, &mut p), Some(cite), "tag 12, by its coordinates");
    assert_eq!(p, b.len());
}
