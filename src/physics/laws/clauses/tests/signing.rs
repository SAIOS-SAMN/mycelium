// CONCORDANCE §0, SIGNING BY ADJUDICATED READING — a signing act is admitted
// only where the kernel reached the hypha's own reconstruction of every
// fixture; a kernel that did not is refused `NotSignatory`, naming the
// clause and the fixture; a network clause is never vetoed; and modes 7 and
// 8 carry the register and the verdict as the same exact objects.

use crate::physics::laws::clauses::*;
use crate::physics::laws::codec::*;

fn kernel() -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[7u8; 32])
}

fn held() -> Vec<Registered> {
    register().expect("the register stands")
}

/// Every clause of `held`, enabled, with the objects `reach` gives for its
/// fixtures.
fn enabled(held: &[Registered], reach: impl Fn(&Fixture) -> Option<Object>) -> Vec<SignedClause> {
    held.iter()
        .map(|r| SignedClause {
            id: r.clause.id.clone(),
            fixtures: r.clause.fixtures.clone(),
            choice: Choice::Enabled { reached: r.clause.fixtures.iter().map(&reach).collect::<Option<Vec<_>>>().expect("each fixture reached") },
        })
        .collect()
}

/// A SIGNATURE IS ADMITTED ONLY ON THE RECONSTRUCTED FIXTURES: a kernel that
/// reaches every fixture's object is admitted on each clause, enabled, as
/// signed and as carried.
#[test]
fn admit_admits_only_reconstructed_fixtures() {
    let held = held();
    let s = Signing::sign(&kernel(), enabled(&held, reconstruct));
    let admitted: Vec<Admitted> = held.iter().map(|r| Admitted { id: r.clause.id.clone(), assent: Assent::Enabled }).collect();
    assert_eq!(admit(&held, &s), Ok(admitted.clone()));
    assert_eq!(Signing::from_bytes(&s.to_bytes()), Ok(s.clone()));
    assert_eq!(admit_bytes(&s.to_bytes()), Ok(admitted));
}

/// A KERNEL THAT CANNOT READ A CLAUSE IS NOT SIGNATORY: presenting fixture
/// 1's object for passage fixture 2 is refused `NotSignatory`, naming the
/// clause, fixture 2, the object presented and the hypha's own.
#[test]
fn admit_refuses_unreached_fixture_as_not_signatory() {
    let held = held();
    let mut clauses = enabled(&held, reconstruct);
    let passage = held.iter().find(|r| r.clause.id.law == forms::passage::LAW_ID).expect("the passage clause");
    let [one, two, ..] = passage.reached.as_slice() else { panic!("four fixtures") };
    let Some(Choice::Enabled { reached }) = clauses.iter_mut().find(|c| c.id == passage.clause.id).map(|c| &mut c.choice) else { panic!("enabled") };
    reached[1] = one.clone();
    let s = Signing::sign(&kernel(), clauses);
    assert_eq!(
        admit(&held, &s),
        Err(SigningRefusal::NotSignatory {
            id: passage.clause.id.clone(),
            fixture: Box::new(passage.clause.fixtures[1].clone()),
            presented: Box::new(one.clone()),
            reconstructed: Box::new(two.clone()),
        })
    );
}

/// A network clause vetoed is refused by name; a kernel clause vetoed is
/// admitted, named vetoed, with no object presented.
#[test]
fn admit_refuses_network_veto_and_names_kernel_veto() {
    let mut held = held();
    let mut clauses = enabled(&held, reconstruct);
    clauses[0].choice = Choice::Vetoed;
    let s = Signing::sign(&kernel(), clauses.clone());
    assert_eq!(admit(&held, &s), Err(SigningRefusal::NetworkVetoed { id: held[0].clause.id.clone() }));
    held[0].clause.standing = Standing::Kernel;
    let admitted = admit(&held, &s).expect("a kernel clause may be vetoed");
    assert_eq!(admitted[0], Admitted { id: held[0].clause.id.clone(), assent: Assent::Vetoed });
    assert_eq!(admitted[1].assent, Assent::Enabled);
}

/// A clause the register does not hold, by name or by content, is refused
/// `UnknownClause`; a clause signed twice is refused `Repeated`.
#[test]
fn admit_refuses_unknown_and_repeated_clause() {
    let held = held();
    let mut later = enabled(&held, reconstruct);
    later[0].id.version += 1;
    let id = later[0].id.clone();
    assert_eq!(admit(&held, &Signing::sign(&kernel(), later)), Err(SigningRefusal::UnknownClause { id }));
    let mut other = enabled(&held, reconstruct);
    other[0].fixtures.swap(0, 1);
    assert_eq!(admit(&held, &Signing::sign(&kernel(), other)), Err(SigningRefusal::UnknownClause { id: held[0].clause.id.clone() }));
    let mut twice = enabled(&held, reconstruct);
    twice.push(twice[0].clone());
    assert_eq!(admit(&held, &Signing::sign(&kernel(), twice)), Err(SigningRefusal::Repeated { id: held[0].clause.id.clone() }));
}

/// A signature that does not verify under the key is refused `Unsigned`;
/// bytes short of an act, or with one left over, `Malformed`.
#[test]
fn admit_refuses_unsigned_and_malformed() {
    let held = held();
    let mut s = Signing::sign(&kernel(), enabled(&held, reconstruct));
    s.sig[0] ^= 1;
    assert_eq!(admit(&held, &s), Err(SigningRefusal::Unsigned));
    let bytes = Signing::sign(&kernel(), enabled(&held, reconstruct)).to_bytes();
    assert_eq!(admit_bytes(&bytes[..bytes.len() - 1]), Err(SigningRefusal::Malformed));
    let mut longer = bytes.clone();
    longer.insert(bytes.len() - 64, 0);
    assert_eq!(admit_bytes(&longer), Err(SigningRefusal::Malformed));
    assert_eq!(admit_bytes(&[]), Err(SigningRefusal::Malformed));
}

/// Modes 7 and 8 round-trip: the register served reads back as its clauses,
/// the signing act presented as its bytes, and every verdict by name.
#[test]
fn concordance_modes_round_trip() {
    let held = held();
    assert_eq!(decode_laws_request(&encode_concordance_request()), Some(LawsRequest::Concordance));
    let served: Vec<ConcordanceClause> =held.iter().map(|r| r.clause.clone()).collect();
    assert_eq!(decode_concordance_reply(&encode_concordance_reply(&Ok(held.clone()))), Some(Ok(served)));
    let refused = RegisterRefusal { id: held[0].clause.id.clone() };
    assert_eq!(decode_concordance_reply(&encode_concordance_reply(&Err(refused.clone()))), Some(Err(refused)));
    let s = Signing::sign(&kernel(), enabled(&held, reconstruct));
    assert_eq!(decode_laws_request(&encode_signature_request(&s)), Some(LawsRequest::Signature(s.to_bytes())));
    let id = held[0].clause.id.clone();
    let not_signatory = SigningRefusal::NotSignatory {
        id: id.clone(),
        fixture: Box::new(held[0].clause.fixtures[0].clone()),
        presented: Box::new(held[0].reached[1].clone()),
        reconstructed: Box::new(held[0].reached[0].clone()),
    };
    let verdicts = [
        admit(&held, &s),
        Err(SigningRefusal::Malformed),
        Err(SigningRefusal::Unsigned),
        Err(SigningRefusal::UnknownClause { id: id.clone() }),
        Err(SigningRefusal::Repeated { id: id.clone() }),
        Err(SigningRefusal::NetworkVetoed { id: id.clone() }),
        Err(not_signatory),
        Err(SigningRefusal::Unregistered { id }),
        Err(SigningRefusal::Unrecorded),
    ];
    for v in verdicts {
        assert_eq!(decode_signature_reply(&encode_signature_reply(&v)), Some(v));
    }
    assert_eq!(decode_signature_reply(&[1, 8]), None, "a refusal tag no refusal names");
    assert_eq!(decode_concordance_reply(&[2]), None);
}
