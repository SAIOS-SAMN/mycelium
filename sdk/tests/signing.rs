// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0 — THE KERNEL'S SIGNING HELPER: the SDK's own reconstruction
// of each served fixture reaches the hypha's object, and a signing act over
// the served register is admitted on every clause.

use mycelium_sdk::signing::{decode_concordance_reply, reconstruct_fixture, sign_concordance, Admitted, Assent, ConcordanceClause, SigningRefusal, Standing};

/// The register as a hypha serves it, read back.
fn served() -> Option<Vec<mycelium_sdk::signing::ConcordanceClause>> {
    let reply = mycelium::physics::laws::codec::encode_concordance_reply(&mycelium::physics::laws::clauses::register());
    decode_concordance_reply(&reply)?.ok()
}

/// The SDK's reconstruction of every served fixture agrees with the
/// hypha's own.
#[test]
fn reconstruct_fixture_agrees_with_register() {
    let held = mycelium::physics::laws::clauses::register().expect("the register stands");
    for r in &held {
        let ours: Vec<_> = r.clause.fixtures.iter().map(reconstruct_fixture).collect();
        let theirs: Vec<_> = r.reached.iter().cloned().map(Some).collect();
        assert_eq!(ours, theirs, "{}", r.clause.id.law);
    }
}

/// A signing act over the served register, every clause enabled, is
/// admitted on each; with every clause vetoed it is refused by the network
/// clause's name; with only the kernel clause vetoed it is admitted, that
/// clause named vetoed.
#[test]
fn sign_concordance_admits_served_register() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[11u8; 32]);
    let served = served().expect("the register served and read back");
    let signed = sign_concordance(&key, &served, |_| Assent::Enabled).expect("every fixture reconstructs");
    let admitted: Vec<Admitted> = served.iter().map(|c| Admitted { id: c.id.clone(), assent: Assent::Enabled }).collect();
    assert_eq!(mycelium::physics::laws::clauses::admit_bytes(&signed.to_bytes()), Ok(admitted));
    let vetoed = sign_concordance(&key, &served, |_| Assent::Vetoed).expect("a veto reconstructs nothing");
    let id = served.iter().find(|c| c.standing == Standing::Network).map(|c| c.id.clone()).expect("a network clause");
    assert_eq!(mycelium::physics::laws::clauses::admit_bytes(&vetoed.to_bytes()), Err(SigningRefusal::NetworkVetoed { id }));
    let by_standing = |c: &ConcordanceClause| match c.standing {
        Standing::Network => Assent::Enabled,
        Standing::Kernel => Assent::Vetoed,
    };
    let partial = sign_concordance(&key, &served, by_standing).expect("every enabled fixture reconstructs");
    let admitted: Vec<Admitted> = served.iter().map(|c| Admitted { id: c.id.clone(), assent: by_standing(c) }).collect();
    assert!(admitted.iter().any(|a| a.assent == Assent::Vetoed), "a kernel clause is registered");
    assert_eq!(mycelium::physics::laws::clauses::admit_bytes(&partial.to_bytes()), Ok(admitted));
}
