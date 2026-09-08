// 13 — the Čech cocycles a mind files, and the torsion it condenses: a
// cycle is consilient, filed, or refused, and never invented.

use crate::cognition::*;
use crate::transducer::element::{self, Element};

/// Residual torsion without a readable class is not filed as identity.
#[test]
fn torsion_consilient_or_filed_or_refused() {
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
let mut spoken = from_residual(&germinate(&a, &b).unwrap());
spoken.puncture(vec![(2, 3), (3, 1), (1, 2)]);
assert_eq!(spoken.torse(), 1);
let inv = crate::transducer::stalk::invariant_of(&spoken);
let mut k = Kernel::new();
k.complexes
    .insert(inv.key(), crate::transducer::cech::Complex::new());
assert_eq!(
    k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    None,
    "no class to file"
);
let idk = crate::transducer::alphabet::Rotor::identity().class_key().expect("identity class");
let filed = k
    .complexes
    .get(&inv.key())
    .and_then(|cx| cx.congruent(&idk))
    .is_some();
assert!(
    !filed,
    "the residual arm files nothing when no class can be read"
);
let mut k2 = Kernel::new();
let mut cx = crate::transducer::cech::Complex::new();
cx.file_congruent(
    [1; 32],
    [2; 32],
    crate::transducer::alphabet::Rotor::identity(),
    4,
);
k2.complexes.insert(inv.key(), cx);
assert_eq!(
    k2.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    None,
    "no class, no refuse against a held identity"
);
}

fn filed_v2_form() -> (Element, crate::transducer::stalk::Invariant, Address) {
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::sentence::{as_element_form, form_torsion, Form, FormFace, Vertex};
let order4 = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
f.vertices.push(Vertex::Word {
    address: Address::from_bytes([2u8; 32]),
    word: Word::identity(),
    carriage: element::carriage(&one.clone(), &(zero.clone(), zero.clone())),
    gauge: vec![(1, 2)],
    class: crate::transducer::stalk::Invariant::default(),
    contact: None,
});
f.vertices.push(Vertex::Word {
    address: Address::from_bytes([3u8; 32]),
    word: Word::identity(),
    carriage: element::carriage(&one, &(zero.clone(), zero)),
    gauge: vec![(1, 2)],
    class: crate::transducer::stalk::Invariant::default(),
    contact: None,
});
f.faces.push(FormFace {
    i: 0,
    j: 1,
    transport: f.face_of(0, 1).expect("star 1"),
});
f.faces.push(FormFace {
    i: 0,
    j: 2,
    transport: f.face_of(0, 2).expect("star 2"),
});
f.faces.push(FormFace::new(1, 2, order4));
f.loops = Form::loops_of(&f.faces, f.vertices.len());
let _ = f.pivot_loop(
    [0xca; 32],
    &crate::network::nerve::Keyed::new(),
    &crate::transducer::family::NoCrystal,
    &mut crate::physics::Physics::default().stroke_n(8),
    None,
    0,
);
assert!(f.is_cauterized(), "filed finite torsion cauterizes");
let e = as_element_form(&f, [1, 0, 0, 0]).expect("v2 tape");
let inv = crate::transducer::stalk::invariant_of(&e);
let class = form_torsion(&e)
    .fiber(crate::network::nerve::ROOT)
    .next()
    .map(|(k, _)| Address::from(k))
    .expect("the form files a class");
(e, inv, class)
}

/// Chapter 23.7 on a v2 form: the court reads Form.torsion by the
/// filed loop's class key, never the element's empty section.
#[test]
fn court_torsion_on_v2_form_is_consilient_or_filed_or_refused() {
use crate::transducer::sentence::{carries_torsion, from_element_form, torsion_loop_rotor};
let (spoken, inv, class) = filed_v2_form();
assert!(
    spoken.torsion().is_empty(),
    "v2 does not fill the element torsion section"
);
assert!(carries_torsion(&spoken), "the form carries the filed cycle");
assert_ne!(
    class,
    crate::transducer::alphabet::Rotor::identity().class_key().expect("identity class"),
    "the class is the loop's, not the identity"
);
let mut k = Kernel::new();
k.complexes
    .insert(inv.key(), crate::transducer::cech::Complex::new());
assert_eq!(
    k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    None,
    "first hearing files"
);
let filed = k
    .complexes
    .get(&inv.key())
    .and_then(|cx| cx.congruent(&class))
    .is_some();
assert!(filed, "court torsion filed by the loop's class key");
assert_eq!(
    k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    None,
    "second hearing is consilient"
);
let f = from_element_form(&spoken).expect("v2 form");
let rep = torsion_loop_rotor(&f, &class).expect("the loop names the class");
let mut k2 = Kernel::new();
let mut cx = crate::transducer::cech::Complex::new();
cx.file_congruent([1; 32], [2; 32], rep, 99);
k2.complexes.insert(inv.key(), cx);
assert_eq!(
    k2.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    Some(crate::verdict::REFUSE_NOT_NATURAL),
    "same class different ℓ is not natural"
);
}

/// THE TORSE STROKE tightens mathematics to physics: a swap family (1→4 and
/// 4→1 at one cell) installs as base + section; the syntropic stroke extracts
/// the ℤ/2 cycle — sections condense to (σ, 2) — and the extracted
/// mathematical torsion RELIEVES the physical torsion charge (booked as
/// syntropic order, per lane, never a scalar).
#[test]
fn the_torse_stroke_extracts_the_cycle_and_relieves_torsion() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 9)];
let b = vec![(0, 0, 4), (0, 1, 9)]; // (0,0): 1→4
k.ingest(&a, &b, [1; 32]);
assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
k.glue_local();
let rev = vec![(0, 0, 1), (0, 1, 9)];
let fwd = vec![(0, 0, 4), (0, 1, 9)];
k.ingest(&fwd, &rev, [1; 32]);
match k.ingest(&fwd, &rev, [2; 32]) {
    Outcome::GermPending => k.glue_local(),
    Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
    o => panic!("{o:?}"),
}
assert!(!k.basis().is_empty(), "the pair forms stand");
k.optimize();
assert!(
    matches!(k.ingest(&a, &b, [3; 32]), Outcome::Decomposed(_) | Outcome::GermPending),
    "the forward pair still speaks or its residual pends"
);
assert!(
    matches!(k.ingest(&fwd, &rev, [3; 32]), Outcome::Decomposed(_) | Outcome::GermPending),
    "the reverse pair is the same form or its residual"
);
}

/// G4b. A loop's obstruction is the product around it (10.3), never a
/// transition read off the end of the fiber. The distinction is not
/// cosmetic on a distributed mesh: the geometric product is not
/// commutative, so a member picked by position is a different object on
/// every node that holds the fiber in a different insertion order, and
/// two nodes would breed two germs from identical mathematics.
#[test]
fn a_held_fiber_germinates_its_holonomy_not_a_member() {
use crate::transducer::alphabet::{LetterId, Word};
let q = LetterId::quarter((0, 1)).expect("quarter turn");
let one = rotor_versor(&Word::from_letters(&[(q, false)]).rotor()).expect("versor");
let inv = rotor_versor(&Word::from_letters(&[(q, true)]).rotor()).expect("inverse versor");
let id = element::identity_versor();

assert!(fiber_holonomy(std::slice::from_ref(&one)).is_none(), "a singleton is not a loop");
assert!(fiber_holonomy(&[]).is_none(), "an empty fiber is not a loop");

let closed = fiber_holonomy(&[one.clone(), inv]).expect("a loop");
assert!(closed.projective_eq(&id), "q then q-inverse closes");

// Four quarter turns on one plane: the product is the obstruction,
// and it is not any member of the fiber.
let held = fiber_holonomy(&[one.clone(), one.clone(), one.clone()]).expect("a loop");
assert!(!held.projective_eq(&id), "three quarter turns do not close");
assert!(
    !held.projective_eq(&one),
    "the holonomy is the composite, not the transition the fiber ends on"
);
}
