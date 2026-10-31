use crate::court::convergence::*;

use crate::transducer::element::{self, Element};

/// A distinct object per `n`: the residual of one compartment recoloured to
/// hue `n`, which is a different vertex for each n.
fn obj(n: u8) -> Element {
    element::from_canonical(
        &element::canonical_chain(&[(0i64, 0i64, 1u8, n)]).expect("one link is A5's chain"),
    )
}
fn r(n: u8) -> Rederivation {
    Rederivation { object: obj(n), k: 1, height: 1 }
}
fn always(_: &[u8; 32], _: &[u8; 32]) -> bool {
    true
}
fn never(_: &[u8; 32], _: &[u8; 32]) -> bool {
    false
}

/// A DISAGREEING THIRD ORGAN REFUSES, however late in the walk it sits.
///
/// `state` asked agreement and independence in ONE walk and returned
/// `Witnessed` from inside it, so the first independent pair ended the
/// walk and no pair after it was examined. Two organs that agree and are
/// independent witnessed the object while a third stood there
/// disagreeing — the re-derivation 13.9 step 8 says the court refuses —
/// and whether that was caught depended on where the disagreeing organ
/// happened to sit.
#[test]
fn state_refuses_disagreeing_organ_after_independent_pair() {
    let mut c = Convergence::new();
    // The first two agree and are independent: under the old walk this
    // returned `Witnessed` before the third was ever compared.
    assert!(c.rederived(&[9u8], [1; 32], r(9)));
    assert!(c.rederived(&[9u8], [2; 32], r(9)));
    assert!(c.rederived(&[9u8], [3; 32], r(7)));
    assert_eq!(c.organs(&[9u8]).len(), 3);
    assert!(
        matches!(c.state(&[9u8], &always), Converged::Inexact { .. }),
        "the third organ derived another vertex, and the court refuses the object"
    );
}

/// One vantage dockets. The object waits for a witness, and waiting is
/// its own state — not a count that has not reached two.
#[test]
fn state_awaits_witness_for_one_organ() {
    let mut c = Convergence::new();
    assert!(c.rederived(&[9u8], [1; 32], r(9)));
    assert_eq!(c.state(&[9u8], &always), Converged::AwaitingWitness);
}

/// The same organ twice is one vantage. It was `a_duplicate_voice_does_
/// not_reach_quorum`; the claim survives the carrier and the thing that
/// cannot be duplicated is an ORGAN, not a mouth.
#[test]
fn rederived_counts_duplicate_organ_once() {
    let mut c = Convergence::new();
    assert!(c.rederived(&[9u8], [1; 32], r(9)));
    assert!(!c.rederived(&[9u8], [1; 32], r(9)), "one organ, one vantage");
    assert_eq!(c.organs(&[9u8]).len(), 1);
    assert_eq!(c.state(&[9u8], &always), Converged::AwaitingWitness);
}

/// A1. TWO ORGANS THAT SPELL ONE VERTEX DIFFERENTLY CONVERGE. The test
/// is re-derivation, not bytes: a versor and its negation are ONE
/// projective element and two coefficient lists. `form_key_bytes` once
/// wrote the list, so the two reached different M2 addresses and the
/// court had to read M1 behind the bytes; it writes the A4 output now
/// (17.2), so the two presentations reach ONE address and the court's
/// M1 test and the address agree. An organ that derived the negated
/// sign holds the same vertex and is not called inexact.
#[test]
fn state_converges_differently_spelled_rederivations() {
    use num_rational::BigRational;
    use num_traits::One;
    let v = element::translator_versor(2, 3);
    let neg = v.scale(&-BigRational::one());
    let positive = element::of_versor(v);
    let negated = element::of_versor(neg);
    assert_eq!(
        positive.address(),
        negated.address(),
        "one address (17.2): the codec writes A4, not the sign"
    );
    assert!(positive.same_vertex(&negated), "and M1 agrees with it");

    let mut c = Convergence::new();
    let key = positive.address();
    c.rederived(key.as_bytes(), [1; 32], Rederivation { object: positive, k: 1, height: 1 });
    c.rederived(key.as_bytes(), [2; 32], Rederivation { object: negated, k: 1, height: 1 });
    assert_eq!(
        c.state(key.as_bytes(), &always),
        Converged::Witnessed { home: [1; 32], witness: [2; 32] },
        "two organs, one vertex, two spellings: converged"
    );

    // And a genuinely different vertex still refuses.
    let mut d = Convergence::new();
    d.rederived(&[9u8], [1; 32], r(3));
    d.rederived(&[9u8], [2; 32], r(4));
    assert!(matches!(d.state(&[9u8], &always), Converged::Inexact { .. }));
}

/// Two organs reaching one address is the admission condition.
#[test]
fn state_witnesses_independent_rederivation() {
    let mut c = Convergence::new();
    c.rederived(&[9u8], [1; 32], r(9));
    c.rederived(&[9u8], [2; 32], r(9));
    match c.state(&[9u8], &always) {
        Converged::Witnessed { home, witness } => {
            assert_ne!(home, witness, "two vantages, not one twice");
        }
        other => panic!("two organs at one address converge, got {other:?}"),
    }
    // Independence is 47.3's test and it is the caller's: a witness
    // inside the home's delegation closure is not a second vantage.
    assert_eq!(c.state(&[9u8], &never), Converged::AwaitingWitness);
}

/// Two organs re-deriving one object to DIFFERENT addresses is not a
/// disagreement for a third to settle. It is inconsistent data.
#[test]
fn state_refuses_disagreeing_rederivations_as_inexact() {
    let mut c = Convergence::new();
    c.rederived(&[9u8], [1; 32], r(9));
    c.rederived(&[9u8], [2; 32], r(8));
    match c.state(&[9u8], &always) {
        Converged::Inexact { a, b } => assert_ne!(a, b),
        other => panic!("differing addresses refuse, got {other:?}"),
    }
}

/// A second voice uttering the same residual is a USE and a settlement
/// party. It never moves the object toward admission.
#[test]
fn used_by_records_party_never_witness() {
    let mut c = Convergence::new();
    c.rederived(&[9u8], [1; 32], r(9));
    c.used_by(&[9u8], [7; 32]);
    c.used_by(&[9u8], [8; 32]);
    c.used_by(&[9u8], [7; 32]);
    let mut p = c.parties(&[9u8]);
    p.sort();
    assert_eq!(p.len(), 2, "distinct voices, counted once each");
    assert_eq!(
        c.state(&[9u8], &always),
        Converged::AwaitingWitness,
        "no number of voices is a second organ"
    );
}
