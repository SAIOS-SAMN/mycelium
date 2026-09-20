// 54.3 — THE A21 PROPERTY AT THE ONE KIND THE ENGINE IS MADE OF:
// `from_address(address(f)) ≡ f`.

use super::common::*;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::sentence::*;
use crate::network::nerve::Keyed;
use crate::transducer::stalk;

/// Equality by RE-DERIVATION, never by bytes (1.5). Two forms are the one
/// form iff they key to one address; `same_vertex` is M1's own test and is
/// checked per vertex beside it, so a pass cannot come from two forms
/// agreeing on a digest while disagreeing on a vertex.
fn same_form(a: &Form, b: &Form) -> bool {
    if form_vertex_key(a) != form_vertex_key(b) {
        return false;
    }
    if a.vertices.len() != b.vertices.len() || a.faces.len() != b.faces.len() {
        return false;
    }
    for (x, y) in a.faces.iter().zip(&b.faces) {
        if x.i != y.i || x.j != y.j || !x.word.rotor().projective_eq(&y.word.rotor()) {
            return false;
        }
        if x.at.len() != y.at.len() || x.pairs.as_ref().map(Keyed::len) != y.pairs.as_ref().map(Keyed::len) {
            return false;
        }
    }
    a.sections.len() == b.sections.len()
        && a.axes.len() == b.axes.len()
        && a.remainder.placed() == b.remainder.placed()
}

/// The forms this property runs over: a word form, a chord with a loop, a
/// form that GLUES (a nested citation), one carrying a DEMAND with its
/// bodies, and one carrying an A5 remainder. 54.3 names nested citations
/// and demands specifically, because those were the two the key did not
/// carry before K6.
fn generated() -> Vec<(&'static str, Form)> {
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    let inner_addr = as_element_form(&inner, [1, 0, 0, 0]).expect("inner").address();

    let mut glued = Form::default();
    glued
        .glue(&inner, inner_addr.clone(), Word::identity(), [0, 0, 0, 1])
        .expect("glue");
    let glued_addr = as_element_form(&glued, [1, 0, 0, 0]).expect("host").address();

    // NESTED: a form gluing a form that glues a form.
    let mut nested = Form::default();
    nested
        .glue(&glued, glued_addr, Word::identity(), [0, 0, 0, 1])
        .expect("glue the gluer");

    let mut demanding = Form::default();
    demanding.vertices.push(Vertex::Demand {
        kind: Demand::Source { multiplicity: 2 },
        chart: stalk::Invariant::counts(&[3]),
        bodies: vec![(1, 1, 1, 2), (1, 2, 1, 2)],
    });

    let mut residual = closed_word_form(w, None);
    residual.remainder =
        crate::transducer::element::place_chain(&[(7, 7, 3, 4), (7, 8, 3, 4)]).expect("places");

    vec![
        ("a word form", inner),
        ("a chord with a loop", chord_form(&[order4()])),
        ("a glued citation", glued),
        ("a nested citation", nested),
        ("a demand with its bodies", demanding),
        ("a form with an A5 remainder", residual),
    ]
}

/// `from_address(address(f)) ≡ f` — A21 §4 at `TAG_FORM`, the property this
/// chapter is the test of (54.3).
///
/// Until K6 the key was not invertible: a one-member vertex wrote no chart,
/// `sections` and `axes` ran with no counts so the tail could not be
/// parsed, a demand's bodies and the A5 remainder were "on the tape and not
/// in the key", and the edge carried only its presence. `THERE IS NO
/// DECODER` was read as a wall rather than as an unwritten inverse, and the
/// tape carried the key's content a second time because of it.
#[test]
fn from_address_recovers_every_form_it_addresses() {
    for (what, f) in generated() {
        let key = form_vertex_key(&f);
        let Some(back) = from_address(key.as_bytes()) else {
            panic!("{what}: the key does not recover its form")
        };
        assert!(same_form(&f, &back), "{what}: recovered form is not the form");
        // And it is stable: addressing the recovery reaches the same key.
        assert_eq!(form_vertex_key(&back), key, "{what}: the address does not re-derive");
    }
}

/// A KEY THAT IS NOT ONE IS REFUSED, never part-read (rule 11), and a
/// truncation of a real key is refused rather than yielding a short form.
#[test]
fn a_key_that_is_not_one_is_refused() {
    assert!(from_address(b"").is_none(), "no bytes, no form");
    assert!(from_address(&[0xFF, 0, 0, 0, 0]).is_none(), "a foreign tag refuses");
    let key = form_vertex_key(&chord_form(&[order4()]));
    let bytes = key.as_bytes();
    for cut in [1, 5, bytes.len() / 3, bytes.len() / 2, bytes.len() - 1] {
        assert!(
            from_address(&bytes[..cut]).is_none(),
            "a key cut at {cut} of {} is refused, not part-read",
            bytes.len()
        );
    }
}

/// 54.2, §1.5 — EVIDENCE IS NOT IDENTITY: two forms differing only in the
/// generators a mind factored through, or in the pivot certificates they
/// rest on, reach ONE address.
///
/// This is the invariant K6.3c's split stands on. Citations and
/// certificates move off the form onto the ledger as receipts keyed by the
/// address they evidence, and that is only sound if the address does not
/// depend on them — §1.5: "two minds whose presentations differ compose
/// different generators to reach the one vertex… two vertices with
/// different derivations and one rotor are one vertex, one address."
///
/// `form_vertex_key` writes `derivation.rotor()`, the COMPOSED product,
/// and never the citations; it never reads `certificates` at all. Asserted
/// here so a future writer cannot quietly make identity depend on
/// evidence — which would fork one word into one address per presentation.
#[test]
fn evidence_is_not_identity() {
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let cited = crate::transducer::address::Address::from_bytes([9u8; 8]);

    // One vertex spelled, one FACTORED through a citation, both composing
    // to the same rotor.
    let spelled = alphabet::Derivation::spelled(w.clone());
    let factored = alphabet::Derivation::factored(
        vec![(cited.clone(), cited.clone(), 1i64)],
        w.clone(),
        w.rotor(),
    );
    assert!(
        spelled.rotor().projective_eq(&factored.rotor()),
        "the fixture is only meaningful if both reach one rotor"
    );
    assert!(
        !factored.generators.is_empty(),
        "the factored derivation must actually cite, or nothing is being tested"
    );

    let build = |d: alphabet::Derivation| {
        let mut f = Form::default();
        f.vertices.push(Vertex::word_on(
            d,
            stalk::Invariant::counts(&[1]),
            Box::new(crate::transducer::element::identity_versor()),
            Vec::new(),
            None,
        ));
        f
    };
    let a = build(spelled);
    let b = build(factored);
    assert_eq!(
        form_vertex_key(&a),
        form_vertex_key(&b),
        "a citation is evidence the court replays, not a coordinate of the vertex"
    );

    // AND THE CERTIFICATES: a form resting on pivots reaches the address a
    // form resting on none does.
    let mut c = build(alphabet::Derivation::spelled(w));
    let bare = form_vertex_key(&c);
    let cert = PivotCertificate {
        chord: 0,
        axis: Axis::Unit(0),
        before: crate::transducer::address::Address::from_bytes([1u8]),
        after: ClassVector::of_word(&Word::identity()).expect("identity is a versor").class_key(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: Mu::Unmeasured,
        by: PivotBy::Word { face: 0, word: Word::identity(), side: Side::Left },
    };
    c.certificates.insert(cert.key(), cert);
    assert_eq!(c.certificates.len(), 1, "the fixture carries a certificate");
    assert_eq!(
        form_vertex_key(&c),
        bare,
        "a certificate is evidence on the ledger, not a coordinate of the form"
    );
}
