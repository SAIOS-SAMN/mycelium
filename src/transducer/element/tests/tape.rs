// 19 — the tape: every carrier of the element survives the wire, and a
// tape of an older generation is refused rather than guessed at.

use crate::core::Q;
use crate::transducer::element::*;
use num_bigint::BigInt;
use crate::transducer::germinate::germinate;

/// γ = (π, Δ): the anchor fiber survives serde exactly; pre-anchor bytes
/// (no tail) parse to the ⊤ carrier — full backward compatibility.
#[test]
fn anchor_fiber_round_trips_and_top_is_default() {
    use crate::transducer::anchor::Sel;
    let g = crate::transducer::element::canonical_chain(&[(0, 0, 1, 2), (0, 1, 1, 2)])
        .expect("one entry per position is A5's chain");
    let plain = from_canonical(&g);
    assert!(plain.anchor().is_none(), "born ⊤ until germination binds");
    let pi = Sel::Minus(Box::new(Sel::Hue(1)), Box::new(Sel::Delta(Box::new(Sel::Hue(1)))));
    let anchored = plain.clone().with_anchor(pi.clone());
    let bytes = anchored.to_bytes();
    let mut pos = 0;
    let back = Element::from_bytes(&bytes, &mut pos).expect("parse");
    assert_eq!(back.anchor(), Some(&pi), "π survives the wire");
    assert_eq!(back.sub(), anchored.sub(), "Δ untouched");
    assert_eq!(back.address(), anchored.address(), "address is BASE-only — π never renames");
    // A pre-anchor serialization (anchor byte and connection count
    // stripped) parses to ⊤ with an empty connection.
    let plain_bytes = plain.to_bytes();
    let mut pos2 = 0;
    let old = Element::from_bytes(&plain_bytes[..plain_bytes.len() - 5], &mut pos2).expect("old form");
    assert!(old.anchor().is_none(), "absent tail = the unconstrained carrier");
    assert!(old.transitions().is_empty(), "absent connection = none recorded");
}

/// The transposition quotient is the CENTERED chain: the same deformation at
/// any position is ONE substance (min-corner framed to origin), while a
/// deformation with the SAME hue tally but DIFFERENT geometry stays DISTINCT —
/// the Step-1 conflation, cured (the tally never decides substance).
/// A connection minted from a transport is a word with two components:
/// the identity motion with a hue swap is a permutation on the hue rows
/// (invertible, a connection), the remainder is its chain, and its
/// address differs from the bare residual's. A non-bijective gauge
/// leaves the hue rows the identity.
#[test]
fn a_connection_is_one_element_with_two_components() {
    let id = crate::transducer::alphabet::Word::identity();
    let remainder = vec![(0, 0, 3u8, 4u8)];
    let w = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &remainder, [5, 2, 0, 0]).expect("the identity spells");
    assert!(w.is_connection(), "invertible versor and a permutation on the hues");
    assert_eq!(w.letters_rederive(), Some(true), "the tape re-derives V");
    assert_eq!(w.letters().map(|l| l.sigma), Some(vec![(1, 2), (2, 1)]), "the gauge rides the tape");
    let mut pos = 0;
    let back = Element::from_bytes(&w.to_bytes(), &mut pos).expect("round trip");
    assert_eq!(back.letters(), w.letters(), "the letters survive the wire");
    assert!(w.solution().is_some(), "the group component inverts");
    assert_eq!(w.sub(), remainder.as_slice(), "the remainder is the chain");
    assert_ne!(w.address(), from_canonical(&canonical_chain(&remainder).expect("one link is A5's chain")).address(), "the connection is not the bare residual");
    let qid = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
    let long = crate::transducer::alphabet::Word::from_letters(&[(qid, false), (qid, false), (qid, false), (qid, false)]);
    assert_eq!(long.key(), crate::transducer::alphabet::Word::identity().key());
    let mut alt = w.clone();
    if let Some(letters) = alt.letters() {
        let spelling = crate::transducer::alphabet::Spelling { word: long };
        let rem = alt.sub().to_vec();
        alt.description =
            connection_description(&spelling, &letters.carriage, &letters.sigma, &rem);
        alt.sync_action_from_tape();
        alt.address.take();
        assert_eq!(alt.address(), w.address(), "q^4 and 1 are one vertex; the tape is not the address");
        assert!(alt.same_vertex(&w), "q^4 and 1 are one vertex; equality is the rotor, not a digest");
    }
    assert!(w.magnitude()[0] >= 5 && w.magnitude()[1] >= 2, "carried volume rides on the lanes");
    let closed = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
    assert!(closed.is_connection() && closed.sub().is_empty(), "an empty remainder is a closed connection");
    let merge = from_connection(&id, &identity_versor(), &[(1, 2)], &Vec::new(), [1, 1, 0, 0]).expect("spells");
    assert!(!merge.is_connection(), "a merging gauge with the identity motion carries no group component");
    assert!(!from_canonical(&canonical_chain(&remainder).expect("one link is A5's chain")).is_connection(), "a residual's tally is never a permutation");
    let unit = from_connection(&id, &identity_versor(), &[], &Vec::new(), [77, 0, 0, 0]).expect("spells");
    assert!(unit.is_identity(), "identity motion and gauge leave no action and no chain");
    assert!(!unit.is_connection(), "the unit is not a connection");
    assert!(unit.sub().is_empty(), "nothing residual to hold as thought");
}

#[test]
fn gen3_round_trips_and_gen2_is_refused() {
    let w = from_connection(&crate::transducer::alphabet::Word::identity(), &identity_versor(), &[(1, 2), (2, 1)], &vec![(0, 0, 3u8, 4)], [5, 2, 0, 0]).expect("spells");
    let bytes = w.to_bytes();
    assert_eq!(
        crate::transducer::address::tag_of(&bytes),
        Some(Element::GEN3_MAGIC),
        "the tape opens at its magic"
    );
    let mut pos = 0;
    let back = Element::from_bytes(&bytes, &mut pos).expect("GEN3");
    assert_eq!(back.address(), w.address());
    assert_eq!(back.letters().map(|l| l.sigma), w.letters().map(|l| l.sigma));
    assert!(back.letters_rederive() == Some(true));
    let mut old = bytes.clone();
    old[0] = Element::GEN2_MAGIC;
    let mut p = 0;
    assert!(Element::from_bytes(&old, &mut p).is_none(), "GEN2 is refused");
}

/// P7: a germ round-trips through bytes losslessly — chain, address, magnitude,
/// and action all survive, so a persisted basis reloads identically.
#[test]
fn a_germ_round_trips_through_bytes() {
    let residual = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap());
    let swap = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]).unwrap());
    for g in [&residual, &swap] {
        let bytes = g.to_bytes();
        let mut pos = 0;
        let back = Element::from_bytes(&bytes, &mut pos).expect("reload");
        assert_eq!(pos, bytes.len(), "consumed exactly");
        assert_eq!(back.address(), g.address(), "the reloaded word is the same citation");
        assert_eq!(back.sub(), g.sub(), "the centered chain survives");
        assert!(back.carriage(g).is_some(), "the thought survives: the identity carriage relates them");
    }
}

/// Torsion survives serde: the cycle (σ, ℓ) round-trips, and the reloaded
/// thought still speaks the unwitnessed orbit member.
#[test]
fn torsion_round_trips_through_bytes() {
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = from_canonical(&germinate(&a, &b).unwrap());
    t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
    t.torse();
    let bytes = t.to_bytes();
    let mut pos = 0;
    let back = Element::from_bytes(&bytes, &mut pos).expect("reload");
    assert_eq!(pos, bytes.len());
    assert_eq!(back.address(), t.address(), "the reloaded word is the same citation");
    assert!(back.matches_chain(&[(0, 0, 3, 1), (0, 1, 1, 2), (0, 2, 2, 3)]), "capacity survives restart");
    assert!(t.transitions().is_empty());
}

/// A17 AND A5 AT THE TAPE. A filed period of zero is not a cycle of no
/// extent and a chain that says two things at one site has no canonical
/// form, so neither is decoded into an element: the readers downstream
/// take `ℓ − 1` whole, and no element reaches the basis whose address
/// would be the form tag and nothing else.
#[test]
fn a_period_of_zero_and_a_chain_that_says_two_things_refuse_the_tape() {
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = from_canonical(&germinate(&a, &b).expect("a germ"));
    t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
    t.torse();
    assert_eq!(t.torsion().len(), 1, "one cycle is filed");
    let bytes = t.to_bytes();
    let mut pos = 0;
    assert!(Element::from_bytes(&bytes, &mut pos).is_some(), "the tape as filed decodes");
    let mut zero = t.clone();
    zero.torsion[0].period = 0;
    let mut pos = 0;
    assert!(
        Element::from_bytes(&zero.to_bytes(), &mut pos).is_none(),
        "a filed period of zero is a malformed tape, not a cycle spanning nothing"
    );

    let one = from_canonical(&canonical_chain(&[(0, 0, 1, 2)]).expect("one link is a chain"));
    let bytes = one.to_bytes();
    let mut pos = 0;
    assert!(Element::from_bytes(&bytes, &mut pos).is_some(), "one transition decodes");
    // The same site saying two different things. It cannot be put on an
    // element at all — `sub` is A5's chain by type — so the claim is made
    // where such bytes could still arrive: the tape. The chain is written
    // as a u32 count then 18 bytes per link, straight after the magic, the
    // word, THE ADDRESS and the four lane words, so the count is at a
    // known offset and a second row at (0,0) goes in behind the first.
    //
    // Two carriers joined the record at 54.5. The ADDRESS, because
    // `Element::address` used to derive a word's name by parsing its tape,
    // so the tape had to be there for a word to know what it was — the
    // dependency retiring the tape has to invert. And the FILING, the
    // speaker's attestation about its form's cycles, which moved off
    // `Form.torsion` because a filing does not factor through the geometry.
    // The address is a u32 length and its bytes; the filing is a u32 count,
    // and this word is a residual with none.
    let at = 1 + 4 + one.description().len() + 4 + one.address().len() + 4 + 32;
    assert!(one.filing().scoped().next().is_none(), "a residual attests no filing");
    assert_eq!(
        u32::from_le_bytes(bytes[at..at + 4].try_into().expect("the chain count")),
        1,
        "the chain count is where the layout says"
    );
    let mut two = bytes.clone();
    let mut clash = bytes[at + 4..at + 4 + 18].to_vec();
    clash[17] = 3;
    two.splice(at + 4 + 18..at + 4 + 18, clash);
    two[at..at + 4].copy_from_slice(&2u32.to_le_bytes());
    let mut pos = 0;
    assert!(
        Element::from_bytes(&two, &mut pos).is_none(),
        "a chain that says two things at one site has no canonical form and no address"
    );
}

#[test]
fn rotated_connection_survives_the_tape() {
    let qid = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let rot = crate::transducer::alphabet::Word::from_letters(&[(qid, false)]);
    let two = Q::from(BigInt::from(2));
    let car = crate::transducer::cga::dilator(&two).geo(&translator_versor(3, 5));
    let w = from_connection(&rot, &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
    let v0 = w.versor().expect("V");
    let bytes = w.to_bytes();
    let mut pos = 0;
    let back = Element::from_bytes(&bytes, &mut pos).expect("reload");
    let v1 = back.versor().expect("V'");
    assert!(same_versor(&v0, &v1), "tape → Element → tape is projective_eq of V");
    let qn = |n: i64| Q::from(BigInt::from(n));
    let p = crate::transducer::cga::point(&qn(1), &qn(0), &qn(0));
    assert_eq!(
        crate::transducer::cga::base(&v0.sandwich(&p)),
        crate::transducer::cga::base(&v1.sandwich(&p)),
        "equal spoken laminae on a rotated, dilated, translated body"
    );
}
