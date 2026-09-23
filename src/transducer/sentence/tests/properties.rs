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
    if !same_torsion(a, b) {
        return false;
    }
    a.sections.len() == b.sections.len()
        && a.axes.len() == b.axes.len()
        && a.remainder.placed() == b.remainder.placed()
}

/// THE FILED CYCLES, CLASS BY CLASS. `Form.torsion` is not in the key —
/// it is a theorem the incidence proves — so two forms that key alike must
/// nonetheless carry the same filing, or the inverse is returning a form
/// whose cycles the original had and the recovery does not.
fn same_torsion(a: &Form, b: &Form) -> bool {
    if a.torsion.len() != b.torsion.len() {
        return false;
    }
    for (class, filed) in a.torsion.entries() {
        let Some(other) = b.torsion.at(&class) else {
            return false;
        };
        if filed.periods != other.periods
            || filed.chord != other.chord
            || filed.sigma != other.sigma
        {
            return false;
        }
    }
    true
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
        ("a star of two spokes", star_form(2, Vec::new())),
        ("a star of three spokes", star_form(3, Vec::new())),
    ]
}

/// §1.5 — TWO VERTICES AT ONE COORDINATE ARE ONE VERTEX, and the address
/// says so.
///
/// `≡` is the wrong relation to ask for here and asking for it is what
/// found the defect. A form holding one coordinate twice is not a fixed
/// point of `from_address ∘ address`: the address NORMALISES it, because a
/// vertex's identity is its coordinates and a set does not hold a member
/// twice. What must hold is that the normal form is reached and then
/// stands — `address` is idempotent from the second application on.
///
/// The defect this guards: `form_vertex_key` wrote `f.vertices.len()` as
/// its vertex count and then walked the DISTINCT vertices out of a trie, so
/// a form like this claimed three records and carried two. The address
/// compared and ordered correctly and could not be PARSED, and it stayed
/// invisible for as long as the tape was what the decoder read.
/// `canonical_places` had the same collapse one level down: it kept
/// `coordinate → index`, so of two vertices sharing a coordinate only the
/// last was given a place and the first kept a stale index every face at
/// that end pointed through.
#[test]
fn two_vertices_at_one_coordinate_are_one_vertex() {
    let mut f = Form::default();
    for _ in 0..3 {
        f.vertices.push(word_vertex_on(9, Word::identity(), stalk::Invariant::counts(&[7])));
        let j = (f.vertices.len() - 1) as u16;
        f.faces.push(f.face_of(0, j).expect("star"));
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    assert_eq!(f.vertices.len(), 4, "the form was built holding one coordinate three times");

    let key = form_vertex_key(&f);
    let back = from_address(&key).expect("the address parses — the count is of what was written");
    assert_eq!(
        back.vertices.len(),
        2,
        "and names the frame and ONE vertex, because that is how many coordinates there are"
    );
    // The normal form stands: addressing it reaches the same address, and
    // that address recovers it exactly.
    assert_eq!(form_vertex_key(&back), key, "the normal form keys to the address it came from");
    let again = from_address(&form_vertex_key(&back)).expect("recovers");
    assert!(same_form(&back, &again), "and is a fixed point from there on");
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
        let Some(back) = from_address(&key) else {
            panic!("{what}: the key does not recover its form")
        };
        assert!(same_form(&f, &back), "{what}: recovered form is not the form");
        // And it is stable: addressing the recovery reaches the same key.
        assert_eq!(form_vertex_key(&back), key, "{what}: the address does not re-derive");
    }
}

/// A KEY THAT IS NOT ONE IS REFUSED, never part-read (rule 11), and a
/// truncation of a real key is refused rather than yielding a short form.
///
/// EVERY CLAIM HERE IS MADE THROUGH `Address::new` AND THAT IS THE SHAPE OF
/// THE CLAIM. `from_address` takes an address, not a byte slice (law 2), so
/// a test that wants to hand it a run of bytes has to say out loud that it
/// is calling those bytes a coordinate — which is exactly what is being
/// refused here, and the line now reads as the assertion it is.
#[test]
fn a_key_that_is_not_one_is_refused() {
    use crate::transducer::address::Address;
    assert!(from_address(&Address::new(Vec::new())).is_none(), "no bytes, no form");
    assert!(
        from_address(&Address::new(vec![0xFF, 0, 0, 0, 0])).is_none(),
        "a foreign tag refuses"
    );
    let key = form_vertex_key(&chord_form(&[order4()]));
    let bytes = key.as_bytes();
    for cut in [1, 5, bytes.len() / 3, bytes.len() / 2, bytes.len() - 1] {
        assert!(
            from_address(&Address::new(bytes[..cut].to_vec())).is_none(),
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
        by: PivotBy::Word { face: 0, word: Box::new(Word::identity()), side: Side::Left },
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
        let back = from_address(&key).expect("recovers");
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
/// 54.7 B6 — A SENTENCE IS KNOWN BY ITS COORDINATE, NOT BY A VERSION BYTE.
///
/// `the_tapes_version_is_frozen_and_a_foreign_one_refuses` stood here. It
/// asserted that `description()[1] == FORM_TAPE_VERSION` and that a
/// foreign byte refused, which was the whole job of a magic prefix on a
/// byte buffer. A sentence has no buffer: the kind is `TAG_FORM` at the
/// head of the word's own ADDRESS, and the layout is versioned by §10 — a
/// layout change is a recut, so there is no in-band version to freeze and
/// nothing to bump without one.
///
/// What the byte actually guarded was a foreign LAYOUT reaching a decoder
/// that would misread it. `from_address` refuses that by construction: a
/// key it cannot parse yields no form, and `TAG_RESIDUAL` and `TAG_FORM`
/// were split precisely so one decoder cannot be handed the other's bytes.
#[test]
fn a_sentence_is_known_by_its_tag_and_carries_no_tape() {
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let inner = closed_word_form(Word::from_letters(&[(q, false)]), None);
    let e = as_element_form(&inner, [1, 0, 0, 0]).expect("mints");
    assert!(e.description().is_empty(), "a sentence mints no tape");
    assert_eq!(
        e.address().tag(),
        Some(crate::transducer::address::TAG_FORM),
        "and is a sentence by the tag its own coordinate opens with"
    );
    assert!(e.is_sentence(), "which is what `is_sentence` reads");
    assert!(from_element_form(&e).is_some(), "and the form comes back out of it");
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
        from_address(&r_addr).is_none(),
        "the form reader refuses a residual's address"
    );
    assert!(from_residual(&[]).is_none(), "and refuses nothing at all");
}

/// 54.3 — THE FILED TORSION IS A THEOREM OF THE ADDRESS, and is re-derived
/// by the inverse rather than carried beside it (law 12).
///
/// THE ADDRESS DOES NOT DETERMINE THE FILING, and this is the reading that
/// says so. `Form.torsion` is not in the key, and the obvious repair — have
/// `from_address` re-derive it the way it re-derives the loops — was
/// written and is wrong, because filing is not a function of the incidence.
///
/// `pivot::drive` files in ONE of its four branches, `NoneDischarged`. A
/// loop the stroke DISCHARGED leaves no filing, because the obstruction is
/// gone; a loop that went to `Vortex`, and a loop whose `Starved` budget ran
/// out before its candidates were reached, stand UNFILED with their
/// holonomy intact. Two forms with the same vertices, faces and loops
/// therefore carry different filings depending on what a stroke could
/// afford. A derivation from the incidence alone hands a starved form the
/// filing its stroke never made, and the first fixture that HAS a loop and
/// has never run a stroke is where that shows.
///
/// So the two are held apart and BOTH are asserted: the address round-trips
/// exactly, and the filing does not travel with it. Torsion is `Form ∖ Key`
/// — where the K6.4 genome instrument already counts it — and closing that
/// is a ruling (torsion enters the key, which is a layout change and so a
/// recut; or 54.5's value carries it), never a decoder inventing a theorem.
#[test]
fn the_address_does_not_carry_the_filing() {
    let mut f = chord_form(&[order4()]);
    assert!(!f.loops.is_empty(), "the fixture must have a loop, or nothing is being tested");
    let mut filed = 0usize;
    for idx in 0..f.loops.len() {
        filed += f.file_loop(idx).len();
    }
    assert!(filed > 0, "the fixture's loop must file, or nothing is being measured");
    assert!(!f.torsion.is_empty(), "and the filing lands on the form");

    // THE FILING IS NOT IN THE ADDRESS: a form that has filed and the same
    // form before it filed reach ONE key.
    let before = chord_form(&[order4()]);
    assert!(before.torsion.is_empty(), "the unfiled twin has no filing");
    assert_eq!(
        form_vertex_key(&f),
        form_vertex_key(&before),
        "filing a loop does not move the form's address — torsion is not keyed"
    );

    // AND THE INVERSE RETURNS THE FORM THE KEY NAMES, which is the unfiled
    // one. It does not invent the filing, and it does not pretend to have
    // recovered it (rule 11).
    let key = form_vertex_key(&f);
    let back = from_address(&key).expect("the key recovers its form");
    assert_eq!(form_vertex_key(&back), key, "the recovery re-derives its address");
    assert!(
        back.torsion.is_empty(),
        "the recovery carries no filing, because the key carries none to carry"
    );
    assert!(
        !same_torsion(&f, &back),
        "and the gap is REAL: this is the measurement, not a passing round-trip"
    );
}

/// 54.3 — AND THE PERIOD IT FILES AT IS THE ONE THE LOOP CLOSES AT.
///
/// `file_loop` took its period as a PARAMETER and `drive.rs` passed
/// `hw.order()` — the rotor's order alone. A quarter turn carried along a
/// translation has the rotor order of one about a fixed point and closes at
/// neither: the screw walks, generating a non-compact ℤ rather than a
/// compact ℤ/4ℤ. Filed at four, a theorem stood on the loop that the loop
/// does not satisfy, and `from_address` asserting `torsion == torsion`
/// would have certified it.
///
/// The filer now solves for the fixed point through
/// [`crate::transducer::cech::classify`], whose own gate is proved in
/// `cech::tests::classify::a_turn_about_a_point_files_and_a_screw_stands`.
/// This asserts the FILER reaches it — that the two are connected, which is
/// what was missing — by reading the periods off both arms of that gate.
#[test]
fn the_filer_reaches_the_fixed_point_gate() {
    use crate::transducer::cech::{classify, Fill};
    use crate::transducer::cga;
    use crate::transducer::element::identity_versor;
    let quarter = order4().rotor();
    let period = |carriage: &crate::transducer::element::Versor| {
        match classify(&quarter, carriage, CycleType::Identity) {
            Fill::Congruent(_, _, ell) => ell,
            _ => 0,
        }
    };
    let q = |n: i64| -> crate::core::Q {
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(n))
    };
    let (zero, five) = (q(0), q(5));
    assert_eq!(period(&identity_versor()), 4, "a residue-free quarter turn closes at four");
    assert_eq!(
        period(&cga::translator(&five, &zero, &zero)),
        4,
        "and so does one about a point away from the origin: it is conjugate to the rotor"
    );
    assert_eq!(
        period(&cga::translator(&zero, &zero, &five)),
        0,
        "a screw fixes no point and is non-compact: it files at 0, never at the rotor's order"
    );
}

/// C2, 70.6 — THE INTERFACE IS IN THE ADDRESS. A contact face's
/// coordinates are the set of boundary elements it runs along, so the key
/// writes them and the inverse returns them. Two faces that differ only in
/// their interface are two forms: a flat interface and a ridged one between
/// one pair are two contacts.
#[test]
fn a_face_carries_its_interface_through_its_address() {
    let mut f = star_form(2, vec![(1, 2, Word::identity())]);
    let chord = f.faces.len() - 1;
    use crate::transducer::base::Site;
    let flat = [(1i64, 1i64, 0i64), (3, 3, 0), (5, 5, 1)].map(Site::from).to_vec();
    f.faces[chord].interface = flat.clone();
    let back = from_address(&form_vertex_key(&f)).expect("the address parses");
    assert!(same_form(&f, &back), "the form comes back");
    let interfaces: Vec<&Vec<Site>> =
        back.faces.iter().map(|x| &x.interface).filter(|i| !i.is_empty()).collect();
    assert_eq!(interfaces, vec![&flat], "with its interface, element by element");
    let mut ridged = f.clone();
    ridged.faces[chord].interface = [(1i64, 1i64, 0i64), (3, 5, 0), (5, 5, 1)].map(Site::from).to_vec();
    assert_ne!(form_vertex_key(&f), form_vertex_key(&ridged), "another interface is another form");
}
