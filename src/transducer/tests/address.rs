use crate::transducer::address::*;
use crate::transducer::alphabet::{LetterId, Rotor, Word};
use crate::transducer::cga::Mv;
use crate::transducer::element::{from_canonical, listing_key, IDENTITY_FRAME};
use crate::transducer::germinate::germinate;
use crate::transducer::stalk::Invariant;

fn sample_words() -> [Word; 3] {
    let q = LetterId::turn_90((0, 1)).expect("90° turn");
    let s = LetterId::try_new(0).expect("sheet");
    [
        Word::from_letters(&[(q, false)]),
        Word::from_letters(&[(q, false), (q, false)]),
        Word::from_letters(&[(s, false), (q, false)]),
    ]
}

/// A21 §4 at the kinds that HAVE a decoder: the rotor and the versor.
///
/// THIS TEST WAS `every_key_decodes_to_its_object` AND IT CHECKED NO SUCH
/// THING FOR A FORM. Its last two lines were
///
/// ```text
/// let addr = e.address();
/// assert_eq!(addr.tag(), Some(TAG_FORM));
/// assert_eq!(e.address(), addr, "address is the form's coordinates");
/// ```
///
/// — a tag check and `addr == addr`. So A21 §4's decode property had never
/// been checked at the one kind the engine is made of, under a name that
/// said it had.
///
/// `Rotor::from_key`, `Mv::from_key` and `ClassVector::from_key` exist.
/// THERE IS NO DECODER FOR `TAG_FORM`, so the property is not checkable
/// here and is not claimed: what a form's address DOES hold is checked in
/// `form_vertex_key_is_function_of_content` below, and the tape is the
/// only thing that reconstructs a form.
#[test]
fn from_key_decodes_rotor_and_versor_keys() {
    for w in sample_words() {
        let r = w.rotor();
        let k = r.key();
        let back = Rotor::from_key(&k).expect("rotor key decodes");
        assert!(r.projective_eq(&back), "decode(&key) is the rotor");
        let ck = r.class_key().expect("a lettered rotor has a class");
        assert_eq!(ck.tag(), Some(TAG_CLASS));
    }
    let m = Mv::zero();
    assert!(m.key().is_none(), "the zero versor has no key");
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let t = crate::transducer::cga::at_three::translator(&one, &zero, &zero);
    let k = t.key().expect("a translator has a key");
    let back = Mv::from_key(&k).expect("mv key decodes");
    assert!(t.projective_eq(&back));
}

/// WHAT A FORM'S ADDRESS ACTUALLY HOLDS, asserted on the object the engine
/// is made of and not on a tautology: it is a FUNCTION of the form's
/// coordinates — the same coordinates give the same address — and it
/// SEPARATES forms that differ in one of them.
///
/// That is the half of A21 an undecodable key keeps. Chapter 1.7's argument
/// against digests is that they keep equality and destroy every other
/// operation; an address that cannot be decoded keeps equality AND keeps
/// the prefix stratum (`is_proper_prefix_is_false_for_sibling_keys`) and loses
/// the object. Better than a digest, and not what A21 §4 specifies.
#[test]
fn form_vertex_key_is_function_of_content() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::sentence::tests::common::{stand_with_star, FRAME};
    use crate::transducer::sentence::{form_vertex_key, Form, Vertex};
    use crate::transducer::stalk::Invariant;
    let build = |gauge: Vec<(u8, u8)>, chart: Invariant| -> Form {
        let mut f = Form::default();
        let v = Vertex::word_on(
            Derivation::spelled(sample_words()[0].clone()),
            chart,
            Box::new(crate::transducer::element::identity_versor()),
            gauge,
            [FRAME],
            None,
        );
        // The member stood at the fixture frame, with its star overlap.
        stand_with_star(&mut f, &v);
        f.reindex();
        f
    };
    let a = build(vec![(1, 2)], Invariant::counts(&[3, 5]));
    let again = build(vec![(1, 2)], Invariant::counts(&[3, 5]));
    assert_eq!(
        form_vertex_key(&a),
        form_vertex_key(&again),
        "one form's coordinates give one address, on every node (law 2)"
    );
    assert_eq!(form_vertex_key(&a).tag(), Some(TAG_FORM));
    // One coordinate moved at a time, each of which 17.2 says the address
    // writes: the member's gauge, then the member's chart.
    let other_gauge = build(vec![(1, 3)], Invariant::counts(&[3, 5]));
    assert_ne!(
        form_vertex_key(&a),
        form_vertex_key(&other_gauge),
        "two forms whose vertex recolours differently are two addresses"
    );
    let other_chart = build(vec![(1, 2)], Invariant::counts(&[7]));
    assert_ne!(
        form_vertex_key(&a),
        form_vertex_key(&other_chart),
        "two forms whose vertex acts on a different chart are two addresses"
    );
}

/// No key is a proper prefix of a sibling key of the same tag.
#[test]
fn is_proper_prefix_is_false_for_sibling_keys() {
    let mut keys: Vec<Address> = sample_words().into_iter().map(|w| w.rotor().key()).collect();
    for w in sample_words() {
        keys.push(w.rotor().class_key().expect("class"));
    }
    let a = from_canonical(&germinate(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]).unwrap());
    let b = from_canonical(&germinate(&[(0, 0, 1), (1, 0, 1)], &[(0, 0, 3), (1, 0, 3)]).unwrap());
    keys.push(a.address());
    keys.push(b.address());
    keys.push(listing_key(a.sub(), &IDENTITY_FRAME).expect("a held word's chain is A5's"));
    keys.push(Invariant::counts(&[1, 2]).key());
    keys.push(Invariant::counts(&[1, 3]).key());
    keys.push(Invariant::counts(&[2]).key());
    for (i, x) in keys.iter().enumerate() {
        for (j, y) in keys.iter().enumerate() {
            if i == j {
                continue;
            }
            if x.tag() != y.tag() {
                continue;
            }
            assert!(
                !is_proper_prefix(x.as_bytes(), y.as_bytes()),
                "sibling keys of tag {:?} must not prefix: {x:?} vs {y:?}",
                x.tag()
            );
        }
    }
}

/// A class record travels as coordinates: its key and order read back whole.
#[test]
fn put_class_record_round_trips_key_and_order() {
    let w = sample_words()[0].clone();
    let key = w.rotor().class_key().expect("class");
    let mut buf = Vec::new();
    put_class_record(&mut buf, &key, 4).expect("A19 fits");
    let mut p = 0usize;
    let (back, order) = take_class_record(&buf, &mut p).expect("A19 record");
    assert_eq!(back, key);
    assert_eq!(order, 4);
    assert_eq!(p, buf.len());
    assert_eq!(key.as_bytes()[0], TAG_CLASS);
}

/// A coordinate path preserves its strata and exact boundaries, and its
/// prefix writes a prefix of its bytes.
#[test]
fn coordinate_path_round_trips_with_exact_boundaries() {
    let a = Address::from_bytes([TAG_ROTOR, 1, 2, 3]);
    let b = Address::from_bytes([TAG_PLANE, 4, 5]);
    let path = CoordinatePath::new(a.clone(), CoordinatePath::new(b.clone(), Nil));
    let mut bytes = Vec::new();
    path.write_canonical(&mut bytes);
    let (back, rest) = CoordinatePath::<Address, CoordinatePath<Address, Nil>>::read_canonical(&bytes)
        .expect("canonical read");
    assert_eq!(back.head, a);
    assert_eq!(back.tail.head, b);
    assert!(rest.is_empty());

    let prefix = CoordinatePath::new(a.clone(), Nil);
    let mut pbytes = Vec::new();
    PrefixOf::<CoordinatePath<Address, CoordinatePath<Address, Nil>>>::write_prefix(&prefix, &mut pbytes);
    assert!(bytes.starts_with(&pbytes));
}

/// The canonical coordinate writers write a key's coordinates unnested,
/// under rule 6.
#[test]
fn put_canonical_writes_key_coordinates_unnested() {
    let w = sample_words()[0].clone();
    let mut rotor_coords = Vec::new();
    w.rotor().put_canonical(&mut rotor_coords);
    let key = w.rotor().key();
    assert_eq!(key.as_bytes()[0], TAG_ROTOR);
    assert_eq!(&key.as_bytes()[1..], &rotor_coords[..]);

    let mut class_coords = Vec::new();
    w.rotor().put_class_coordinates(&mut class_coords).expect("class coords");
    let ckey = w.rotor().class_key().expect("class key");
    assert_eq!(ckey.as_bytes()[0], TAG_CLASS);
    assert_eq!(&ckey.as_bytes()[1..], &class_coords[..]);

    let mut word_coords = Vec::new();
    w.put_canonical(&mut word_coords);
    assert_eq!(word_coords, rotor_coords);
}
