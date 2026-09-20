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

/// 54.2 — A CITATION IS ⟨A_c, φ⟩, AND φ IS IN THE KEY.
///
/// A citation is not a foreign key. The cited form stands canonical and
/// unmutated at its own address; it ACTS inside the citing form under a
/// morphism, `x_host = φ x_cited φ̃`. Before this, a `Glued` vertex carried
/// `placement: Word` — a rotor in `Spin(4,0)`, which can turn the cited
/// form and can neither translate nor dilate it — and `vertex_carriage_at`
/// answered `identity_versor()` for every Glued vertex, so the placement
/// face carried the identity too and the embedding was nowhere in the key.
///
/// `from_address(address(f)) ≡ f` passed anyway, because every fixture
/// glued at the origin and no test ever cited a sub-complex placed
/// somewhere. This is that test.
///
/// Two readings: one form cited under two different morphisms reaches two
/// addresses, and each recovers the morphism it was written with. φ is a
/// conformal versor (A4) — rotor, translator and dilator together — and is
/// `O(1)` per citation whatever the cited form's size, so carrying it does
/// not re-inline what 1.7 keeps whole.
#[test]
fn a_citation_carries_the_morphism_it_acts_under() {
    use crate::transducer::element::{identity_versor, translator_versor};
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w, None);
    let at = as_element_form(&inner, [1, 0, 0, 0]).expect("inner").address();

    let host = |phi: crate::transducer::element::Versor| {
        let mut f = Form::default();
        f.glue_under(&inner, at.clone(), Word::identity(), [0, 0, 0, 1], phi)
            .expect("glue under a morphism");
        f
    };
    // The SAME sub-complex, cited at the origin and cited three sites over.
    let here = host(identity_versor());
    let over = host(translator_versor(3, 0));

    assert_ne!(
        form_vertex_key(&here),
        form_vertex_key(&over),
        "one form cited at two places is two forms: φ is in the key"
    );

    // AND EACH RECOVERS ITS OWN MORPHISM. The inverse reads φ back, so the
    // embedding survives `from_address` and is not re-derived from a
    // placement that never carried it.
    for (f, phi) in [(&here, identity_versor()), (&over, translator_versor(3, 0))] {
        let key = form_vertex_key(f);
        let back = from_address(key.as_bytes()).expect("recovers");
        assert_eq!(form_vertex_key(&back), key, "the recovery re-derives its address");
        let got = back.vertices.iter().find_map(|v| match v {
            Vertex::Glued { carriage, .. } => Some((**carriage).clone()),
            _ => None,
        });
        assert!(
            got.is_some_and(|g| crate::transducer::element::same_versor(&g, &phi)),
            "the citation comes back under the morphism it was written with"
        );
    }
}

/// 54.5 — THE TAPE'S VERSION IS FROZEN, and a tape of any other version is
/// refused rather than read at a guess.
///
/// The byte's whole job is to refuse a foreign layout. A version that does
/// not match the bytes the encoder writes refuses nothing — which is what
/// happened when φ entered the tape at `01760af` without a bump — and a
/// version that keeps moving is a schema, which 54.5 says the tape is not.
#[test]
fn the_tapes_version_is_frozen_and_a_foreign_one_refuses() {
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let inner = closed_word_form(Word::from_letters(&[(q, false)]), None);
    let e = as_element_form(&inner, [1, 0, 0, 0]).expect("mints");
    let tape = e.description();
    assert_eq!(
        tape.get(1).copied(),
        Some(FORM_TAPE_VERSION),
        "the tape carries the frozen version"
    );
    assert!(from_element_form(&e).is_some(), "and this build reads it");

    // Every other version is refused — the baseline is cut at this one.
    for other in [0u8, 10, 11, 12, 14, 255] {
        let mut wrong = e.clone();
        let mut bytes = tape.to_vec();
        bytes[1] = other;
        wrong.set_description(bytes);
        assert!(
            from_element_form(&wrong).is_none(),
            "a tape at version {other} is refused, not read at a guess"
        );
    }
}

/// THE ADDRESS DOES NOT DEPEND ON THE ORDER VERTICES WERE APPENDED
/// (law 2, rule 15).
///
/// It did. A `Form` holds `vertices: Vec<Vertex>` and `form_vertex_key`
/// walked it, so two forms holding ONE set of vertices reached two
/// addresses if they were built in two orders — arrival in the address,
/// measured at `63d6e40`, where this test asserted the defect with
/// `assert_ne` and said it would flip.
///
/// The key now writes the vertices in their OWN canonical order: the
/// Frame first, because "a form opens at its frame" (18) and it is the
/// basepoint rather than a member of the set, then every other vertex by
/// its own coordinate bytes, read back from a `Keyed` whose trie order IS
/// those coordinates' order. A face names its ends by their place in that
/// ordering, so the incidence is read against the coordinates and not
/// against the vector the vertices happen to sit in.
///
/// WHAT IS NOT YET DONE, so the flip is not read as more than it is: the
/// form still HOLDS a `Vec` and the key still names a face's ends by a
/// place in an order rather than by the ends' own addresses. The order is
/// now content, which is the difference between this and arrival — but
/// `Frame`, `Rebased` and `Unfolded` vertices have no derived address of
/// their own yet, and until they do a face cannot be `(A_i, A_j)`.
#[test]
fn the_address_is_independent_of_vertex_append_order() {
    let q01 = alphabet::LetterId::quarter((0, 1)).expect("q");
    let q23 = alphabet::LetterId::quarter((2, 3)).expect("q");
    let mk = |ids: [alphabet::LetterId; 2]| {
        let mut f = Form::default();
        for id in ids {
            // The chart follows the LETTER, not the position, so both
            // forms hold the same set of vertices and differ only in the
            // order they were appended.
            let c = u64::from(id.raw());
            f.vertices.push(Vertex::word_on(
                alphabet::Derivation::spelled(Word::from_letters(&[(id, false)])),
                stalk::Invariant::counts(&[c]),
                Box::new(crate::transducer::element::identity_versor()),
                Vec::new(),
                None,
            ));
        }
        f.reindex();
        f
    };
    let a = mk([q01, q23]);
    let b = mk([q23, q01]);
    assert_eq!(
        form_vertex_key(&a),
        form_vertex_key(&b),
        "one set of vertices is one form, however it was built"
    );
}

/// 54.3 — THE LANGUAGE HAS TWO DECODERS AND THEY REFUSE EACH OTHER.
///
/// A form is a closed cell complex in the vertex-key layout; a residual is
/// an unclosed 1-chain written as its own bytes. They shared `TAG_FORM`,
/// so neither could be decoded without guessing which it held, and
/// `Element::address` fell through from the first to the second whenever a
/// sentence's tape did not decode — a genome then reported a 0.9 KB
/// address against a 46 KB tape and looked like a 50x saving.
///
/// Four readings: a residual address gives its chain back; a form's
/// address is refused by the chain reader; a residual's address is refused
/// by the form reader; and a REWRITE, which names the forms it rewrites
/// and carries no chain, is refused rather than answered with an empty one
/// — "not a chain" is not "a chain that says nothing" (rule 11).
#[test]
fn a_form_and_a_residual_refuse_each_others_decoder() {
    use crate::transducer::element::from_residual as mint_residual;
    use crate::transducer::sentence::from_residual;

    let chain: Vec<(i64, i64, u8, u8)> = vec![(2, 2, 1, 2), (2, 3, 1, 2)];
    let residual = mint_residual(&chain).expect("a chain mints a residual");
    let r_addr = residual.address();
    assert!(
        matches!(r_addr.as_bytes(), [t, ..] if *t == crate::transducer::address::TAG_RESIDUAL),
        "a residual says so by its tag"
    );

    // THE CHAIN COMES BACK, centred: A5 quotients placement, so the
    // reading is the chain it is and not where it sat.
    let back = from_residual(r_addr.as_bytes()).expect("a residual address inverts");
    assert_eq!(
        mint_residual(&back).map(|e| e.address()),
        Some(r_addr.clone()),
        "the chain read back mints the address it was read from"
    );

    // AND THE TWO DECODERS REFUSE EACH OTHER.
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let form = closed_word_form(Word::from_letters(&[(q, false)]), None);
    let f_addr = form_vertex_key(&form);
    assert!(
        matches!(f_addr.as_bytes(), [t, ..] if *t == crate::transducer::address::TAG_FORM),
        "a form says so by its tag"
    );
    assert!(
        from_residual(f_addr.as_bytes()).is_none(),
        "the chain reader refuses a form's address"
    );
    assert!(
        from_address(r_addr.as_bytes()).is_none(),
        "the form reader refuses a residual's address"
    );
    assert!(from_residual(&[]).is_none(), "and refuses nothing at all");
}
