// 50.2 step 2 — F FACTORS THROUGH THE PRESENTATION. What a voice says
// depends on what it holds; a voice that holds nothing spells from the
// identity, which is the cipher's case and no longer the only one.

use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element;
use crate::transducer::family::{factor_versor, Crystal, NoCrystal};
use crate::transducer::sentence::{Form, Vertex};
use crate::transducer::stalk::Invariant;

fn turn_90(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A closed one-word sentence carrying `w`: a generator as the language
/// holds one since L5 — a `Word` vertex of a FORM, never a v1 connection.
fn sentence_of(w: Word, tag: u8) -> (Address, Form) {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(w),
        Invariant::counts(&[u64::from(tag)]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        [crate::transducer::sentence::tests::common::FRAME],
        None,
    );
    let c = f.stand_member(&v, crate::transducer::sentence::tests::common::FRAME).expect("a generator stands");
    // THE STAR, WHERE ITS PLACE DERIVES (AT1): an overlap's place is measured
    // through the coboundary's versor, and an off-plane word has none on the
    // sensor plane — as `letter_of`'s letter has none — so its star is not
    // put (rule 11). The generator this fixture holds is the vertex, never
    // the face.
    let _star = f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c, None);
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    (Address::from_bytes([TAG_FORM, tag]), f)
}

/// A mind that holds one form, answering `generators_under` through the
/// SAME index the epigenome uses — `admit_generators` filling it and the
/// keyed subtree read draining it — so what this fixture proves about the
/// read is what the live mind does (§3.4).
struct Holds {
    index: crate::transducer::sentence::GeneratorIndex,
    form: Form,
}

impl Holds {
    fn of(w: Word, tag: u8) -> Holds {
        let (address, form) = sentence_of(w, tag);
        let mut index = crate::transducer::sentence::GeneratorIndex::new();
        crate::transducer::sentence::admit_generators(&mut index, &address, &form);
        Holds { index, form }
    }
}

impl Crystal for Holds {
    fn on_class(&self, _c: &Invariant) -> Vec<(&crate::transducer::element::Element, &Form, crate::transducer::stalk::Standing)> {
        Vec::new()
    }
    fn on_body(&self, _c: &Invariant) -> Vec<(&crate::transducer::element::Element, &Form, crate::transducer::stalk::Standing)> {
        Vec::new()
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &crate::transducer::family::Grid,
        placement: &Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }
    fn by_address(&self, _a: &[u8]) -> Option<(&crate::transducer::element::Element, &Form)> {
        None
    }
    fn on_axis(&self, _c: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<Word> {
        Vec::new()
    }
    fn generators_under(
        &self,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(Address, Address, Word)> {
        crate::transducer::sentence::generators_under(&self.index, chart, budget)
            .into_iter()
            .filter_map(|(form, vertex)| {
                let w = crate::transducer::sentence::word_at_vertex(&self.form, vertex.as_bytes())?;
                Some((form, vertex, w))
            })
            .collect()
    }
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke()
}

/// NO GAUGE. Every target in this file is a bare plane versor measured
/// with no hue map beside it, so the `𝒢` lane is unoccupied — which is the
/// truth about these targets, not a placeholder. The claims here are about
/// the presentation on `𝒽`, which is the only lane they occupy.
const NO_GAUGE: &[(u8, u8)] = &[];

/// THE CIPHER'S CASE, kept as the boundary it is: a mind that holds
/// nothing has no presentation to factor through, so the word is the
/// residue A22 peeled from the identity and the derivation cites nobody.
#[test]
fn a_voice_that_holds_nothing_spells_from_the_identity() {
    let v = element::versor_of_word(&turn_90(2)).expect("a plane versor");
    let d = factor_versor(&NoCrystal, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(d.is_spelled(), "no citation, because nothing is held");
    assert!(d.rotor().projective_eq(&turn_90(2).rotor()), "and it reaches the vertex");
}

/// 50.2 step 2, the whole claim of this row: the SAME measured versor,
/// read by a mind that holds the 90° turn, is a derivation THROUGH
/// that generator. A22 from the identity is a function of the versor
/// alone; this is not, and that is the difference between a language and
/// a cipher.
#[test]
fn a_voice_that_holds_a_generator_derives_through_it() {
    let held = Holds::of(turn_90(1), 1);
    let v = element::versor_of_word(&turn_90(2)).expect("a plane versor");
    let d = factor_versor(&held, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(!d.is_spelled(), "the mind factored through what it holds");
    let [(form, _, _)] = d.generators.as_slice() else { panic!("one generator, applied twice") };
    assert_eq!(
        *form,
        Address::from_bytes([TAG_FORM, 1]),
        "cited by the address of the form the mind was taught"
    );
    assert!(d.residue.is_identity(), "the generator reached the whole word");
    assert!(
        d.rotor().projective_eq(&turn_90(2).rotor()),
        "and the derivation composes to the same vertex the cipher reached"
    );
}

/// 1.5 and 13.10 on the wire: two minds whose presentations differ read
/// one measured versor as two derivations and ONE vertex. The address
/// cannot tell them apart and must not; the derivations can.
#[test]
fn two_presentations_read_one_versor_as_two_derivations_of_one_vertex() {
    let v = element::versor_of_word(&turn_90(2)).expect("a plane versor");
    let a = factor_versor(&Holds::of(turn_90(1), 1), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    let b = factor_versor(&Holds::of(turn_90(3), 3), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert_ne!(a.generators, b.generators, "two presentations, two proofs");
    assert!(
        a.rotor().projective_eq(&b.rotor()),
        "one vertex, one address, one ledger entry"
    );
}

/// Rule 6 on the memo, asserted where it belongs — on the OUTPUT, not on
/// a key. The percept is memoised on sensor bytes and the derivation is
/// re-run per mind, so the same two grids read by two minds give two
/// derivations with no cache root anywhere in it. A memo that keyed the
/// presentation would let a digest decide whether F re-derives.
#[test]
fn one_memoised_percept_gives_each_mind_its_own_derivation() {
    let v = element::versor_of_word(&turn_90(2)).expect("a plane versor");
    let a = factor_versor(&Holds::of(turn_90(1), 1), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    let bare = factor_versor(&NoCrystal, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    // Re-read by the first mind after the second: no root was consulted
    // and no entry was overwritten, because there is no entry.
    let again = factor_versor(&Holds::of(turn_90(1), 1), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(bare.is_spelled(), "the mind that holds nothing still spells");
    assert_eq!(a.generators, again.generators, "and the mind that holds one still cites it");
}

/// A generator the ALPHABET DOES NOT SPELL, as a court adjoins one
/// (37.7): a word vertex that cites nothing, spells nothing, and carries
/// `π_p = a + b·i` as the rotor `a + b·e01`. Its block's arc is `π_p²`,
/// of norm `p²`, which is the action a versor carrying that prime stands
/// on.
fn letter_of(a: i64, b: i64, tag: u8) -> (Address, Form) {
    use crate::transducer::alphabet::{Derivation, Rotor};
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let rotor = Rotor::scalar(q(a)).add(&Rotor::blade(0b0011, q(b)));
    let mut f = Form::default();
    f.put_vertex(Vertex::word_on(
        Derivation::adjoined(rotor),
        Invariant::counts(&[u64::from(tag)]),
        Box::new(element::identity_versor()),
        Vec::new(),
        [crate::transducer::sentence::tests::common::FRAME],
        None,
    ))
    .expect("an adjoined letter has a class");
    // No star face: the face would be the letter's own action, which the
    // seventeen cannot spell (L11 (2/2)).
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    (Address::from_bytes([TAG_FORM, tag]), f)
}

impl Holds {
    /// A mind taught an adjoined generator, filling the same index a
    /// taught form fills.
    fn of_letter(a: i64, b: i64, tag: u8) -> Holds {
        let (address, form) = letter_of(a, b, tag);
        let mut index = crate::transducer::sentence::GeneratorIndex::new();
        crate::transducer::sentence::admit_generators(&mut index, &address, &form);
        Holds { index, form }
    }
}

/// The versor whose sensor block is the arc `s + t·i`.
fn versor_of_arc(s: i64, t: i64) -> crate::transducer::element::Versor {
    use crate::transducer::alphabet::Rotor;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    // A rotor `x + y·e01` has block arc `(x² − y²) + 2xy·i`, so the arc
    // `s + t·i` is reached from its square root in ℤ[i].
    let r = Rotor::scalar(q(s)).add(&Rotor::blade(0b0011, q(t)));
    element::versor_of_word(&crate::transducer::alphabet::Word::formed(r))
        .expect("a plane versor")
}

/// L11 (3/3) — THE THIRD FACT: a mind taught a generator the seventeen
/// cannot spell DERIVES THROUGH IT. A12 over letter exponents cannot see
/// 17 however many letters the mind holds; the ring divides it out of the
/// sensor block exactly and cites it with its multiplicity.
///
/// The versor here is `π_17` itself as a rotor, whose block arc is
/// `π_17²`: the generator divides it once and nothing is left, so the
/// residue is the identity.
#[test]
fn a_mind_taught_an_adjoined_generator_derives_through_it() {
    let held = Holds::of_letter(4, 1, 7);
    let v = versor_of_arc(4, 1);
    let d = factor_versor(&held, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the ring reaches it where the peel could not");
    assert!(!d.is_spelled(), "it cites the letter it was taught");
    let [(_, _, exponent)] = d.generators.as_slice() else { panic!("one generator") };
    assert_eq!(*exponent, 1, "applied once: the target IS the generator");
    assert!(d.residue.is_identity(), "and nothing is left for the alphabet");
}

/// The same generator at multiplicity two: `π_17²` as a rotor, whose
/// block arc is `π_17⁴`. The ring divides twice and the exponent says so.
#[test]
fn the_ring_cites_a_generator_at_its_full_multiplicity() {
    let held = Holds::of_letter(4, 1, 7);
    // (4 + i)² = 15 + 8i.
    let v = versor_of_arc(15, 8);
    let d = factor_versor(&held, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the ring reaches it");
    let [(_, _, exponent), ..] = d.generators.as_slice() else { panic!("a generator") };
    assert_eq!(*exponent, 2, "full multiplicity, divided out to exhaustion");
    assert!(d.residue.is_identity(), "and nothing is left over");
}

/// A mind that does NOT hold the letter still cannot say it: the ring has
/// nothing to divide by, and the demand the peel typed stands. This is
/// what makes the two tests above a fact about the LANGUAGE and not about
/// the arithmetic.
#[test]
fn a_mind_without_the_letter_still_demands_it() {
    let v = versor_of_arc(4, 1);
    let demanded = factor_versor(&NoCrystal, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect_err("the seventeen cannot spell it");
    assert!(
        matches!(demanded, crate::transducer::sentence::Demand::RotorPrime { prime: 17, .. }),
        "and the demand names the prime: {demanded:?}"
    );
}

/// A court holding the letter COMPOSES the citation (L14): the resolver
/// answers the generator's action, so a citation of a word the alphabet
/// cannot spell verifies exactly as one it can.
#[test]
fn a_court_holding_the_letter_composes_a_citation_of_it() {
    use crate::transducer::alphabet::Verified;
    let held = Holds::of_letter(4, 1, 7);
    let (letter_addr, letter_form) = letter_of(4, 1, 7);
    let d = factor_versor(&held, &mut budget(), &versor_of_arc(15, 8), NO_GAUGE)
        .expect("a linear block")
        .expect("the ring reaches it");
    let cover = |form: &Address, vertex: &Address| -> Option<crate::transducer::alphabet::Rotor> {
        match form == &letter_addr {
            true => crate::transducer::sentence::rotor_at_vertex(&letter_form, vertex.as_bytes()),
            false => None,
        }
    };
    assert_eq!(d.verify_with(&cover), Verified::Composes, "the court re-derives and agrees");
    // THE CITATION'S VERTEX HALF IS THE VERTEX'S OWN ADDRESS (50.2 step 2,
    // 54.2). It used to be the letter FORM's address, because `word_on`
    // was handed one and the minting site passed the form's; a vertex now
    // derives its address from its coordinates.
    let cited_vertex = letter_form
        .vertices
        .scoped()
        .find_map(|(_, v)| match v {
            crate::transducer::sentence::Vertex::Word { address, .. } => Some(address.clone()),
            _ => None,
        })
        .expect("the letter has a word vertex");
    assert_eq!(
        d.verify_with(&|_: &Address, _: &Address| None),
        Verified::Unresolved { form: letter_addr.clone(), vertex: cited_vertex },
        "a court that has not ruled it waits, as it does for any citation"
    );
}

/// The chart read reaches it: a letter is filed at its own versor chart
/// and a versor carrying that prime finds it, because `generators_under`
/// peels prefixes and the two charts share one. Asserted, because the
/// whole ring branch rests on the mind being ABLE to offer the letter.
#[test]
fn a_versor_carrying_the_prime_finds_the_letter_at_its_chart() {
    use crate::transducer::alphabet::Rotor;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let held = Holds::of_letter(4, 1, 7);
    let target = Rotor::scalar(q(15)).add(&Rotor::blade(0b0011, q(8)));
    let chart = crate::transducer::sentence::versor_chart_of(&target).expect("a chart");
    assert_eq!(
        Crystal::generators_under(&held, &chart, &mut budget()).len(),
        1,
        "the mind offers the letter to a versor whose norm the prime divides"
    );
}

#[test]
fn derivation_factors_offplane_generator_and_verifies_rotor() {
    // A generator on an off-sensor plane (2, 3)
    let offplane_q = LetterId::turn_90((2, 3)).expect("offplane 90° turn");
    let gen_word = Word::from_letters(&[(offplane_q, false)]);
    let held = Holds::of(gen_word.clone(), 99);

    // Deriving the spelled offplane generator cites it directly without requiring a 2D plane form
    let d = crate::transducer::family::derivation_of_spelled(
        &held,
        &mut budget(),
        &crate::transducer::alphabet::Derivation::spelled(gen_word.clone()),
        &[],
    ).expect("derivation succeeds");

    assert!(!d.is_spelled(), "offplane generator is factored rather than left as spelled");
    assert_eq!(d.generators.len(), 1, "cites the held offplane generator");
    assert!(d.rotor().projective_eq(&gen_word.rotor()), "the derivation projectively reproduces the Clifford rotor");
}

#[test]
fn adjoined_basis_discovers_generators_on_offplane_channels() {
    use crate::transducer::alphabet::{Derivation, Rotor};
    use crate::transducer::family::derivation::adjoined_basis;
    use crate::transducer::sentence::CycleType;

    // Construct an adjoined generator on offplane (0, 3) with prime 17 (4 + i e03)
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let mask_03 = (1usize << 0) | (1usize << 3); // plane (0, 3)
    let rotor = Rotor::scalar(q(4)).add(&Rotor::blade(mask_03, q(1)));
    let address = Address::from_bytes([TAG_FORM, 55]);
    let mut f = Form::default();
    f.put_vertex(Vertex::word_on(
        Derivation::adjoined(rotor),
        Invariant::default(),
        Box::new(element::identity_versor()),
        Vec::new(),
        [crate::transducer::sentence::tests::common::FRAME],
        None,
    ))
    .expect("an adjoined generator has a class");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();

    let mut index = crate::transducer::sentence::GeneratorIndex::new();
    crate::transducer::sentence::admit_generators(&mut index, &address, &f);
    let held = Holds { index, form: f };

    // Target versor
    let v = element::identity_versor();
    let basis = adjoined_basis(&held, &mut budget(), &v, &CycleType::of(NO_GAUGE))
        .expect("adjoined basis read succeeds");

    assert_eq!(basis.len(), 1, "unpinned adjoined_basis discovers the (0, 3) prime generator");
    assert_eq!(basis[0].pi.norm(), num_bigint::BigInt::from(289));
}

/// Whether the peel over a basis and the alphabet's own factorization
/// read one rotor alike: both close and reach one vertex, or both stand
/// with the same kind of demand.
fn peels_agree(r: &crate::transducer::alphabet::Rotor) -> bool {
    use crate::transducer::alphabet::factor_rotor;
    use crate::transducer::family::derivation::peel_versor_with_target;
    let by_letters = factor_rotor(r).expect("a versor");
    let by_basis = peel_versor_with_target(&element::identity_versor(), Some(r), Vec::new())
        .expect("a versor");
    match (by_letters, by_basis) {
        (Ok(w), Ok((cited, w2))) => {
            cited.is_empty() && w.rotor().projective_eq(&w2.rotor()) && w2.rotor().projective_eq(r)
        }
        (Err(a), Err(b)) => a.label() == b.label(),
        _ => false,
    }
}

/// 50.4 at the peel over a basis: `peel_versor_with_target` and
/// `factor_rotor` are one factorization — the quaternion split and the
/// three commuting pairs — so on every rotor the letters spell they close
/// together and reach one vertex, across blocks that do not commute.
#[test]
fn the_peel_over_a_basis_and_the_alphabets_own_factorization_agree_on_the_letters() {
    let q01 = LetterId::turn_90((0, 1)).expect("90° turn (0,1)");
    let q02 = LetterId::turn_90((0, 2)).expect("90° turn (0,2)");
    let p23 = LetterId::pythagorean_5((2, 3)).expect("(3,4,5) on (2,3)");
    let p13 = LetterId::pythagorean_5((1, 3)).expect("(3,4,5) on (1,3)");
    let rotors = [
        crate::transducer::alphabet::Rotor::identity(),
        Word::from_letters(&[(q01, false)]).rotor(),
        Word::from_letters(&[(p23, false)]).rotor(),
        Word::from_letters(&[(q01, false), (p23, false), (p13, true)]).rotor(),
        Word::from_letters(&[(q02, false), (q01, false)]).rotor(),
    ];
    for r in &rotors {
        assert!(peels_agree(r), "the two peels read {r:?} differently");
    }
}

/// 37.7, the rotor level: `3 + e₀₁ + e₀₂` is a rotor of weight 11 that no
/// coordinate plane holds, so both peels stand it as a demand and neither
/// spells its sensor shadow for it.
#[test]
fn a_rotor_no_coordinate_plane_holds_stands_as_a_demand_on_both_peels() {
    use crate::transducer::alphabet::{factor_rotor, Rotor, Q};
    use crate::transducer::family::derivation::peel_versor_with_target;
    use crate::transducer::sentence::Demand;
    let q = |n: i64| Q::from_integer(num_bigint::BigInt::from(n));
    let r = Rotor::scalar(q(3))
        .add(&Rotor::blade(0b0011, q(1)))
        .add(&Rotor::blade(0b0101, q(1)));
    assert!(r.weight().is_some(), "a versor");
    assert!(
        matches!(factor_rotor(&r), Some(Err(Demand::Standing { .. }))),
        "the alphabet's own peel stands"
    );
    let by_basis = peel_versor_with_target(&element::identity_versor(), Some(&r), Vec::new())
        .expect("a versor");
    assert!(
        matches!(by_basis, Err(Demand::Standing { .. })),
        "and the peel over a basis stands with it: {by_basis:?}"
    );
}

#[test]
fn peel_versor_with_peels_identity_and_turned_planes() {
    use crate::transducer::family::derivation::peel_versor_with;

    let v = element::versor_of_word(&turn_90(1)).expect("sensor 90° turn versor");
    let res = peel_versor_with(&v, Vec::new()).expect("peel succeeds");
    let (cited, word) = res.expect("closed peel");
    assert!(cited.is_empty(), "no citations");
    assert_eq!(word, turn_90(1), "exact single plane word produced");

    let id_v = element::identity_versor();
    let id_res = peel_versor_with(&id_v, Vec::new()).expect("identity peel succeeds");
    let (id_cited, id_word) = id_res.expect("closed identity peel");
    assert!(id_cited.is_empty());
    assert!(id_word.is_identity());
}
