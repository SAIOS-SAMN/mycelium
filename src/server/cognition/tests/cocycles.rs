// 13 — the Čech cocycles a mind files, and the torsion it condenses: a
// cycle is consilient, filed, or refused, and never invented.

use super::common::speak_pair;
use crate::cognition::*;
use crate::transducer::element::{self, Element};

/// Residual torsion without a readable class is not filed as identity.
#[test]
fn torsion_consilient_or_filed_or_refused() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
let mut spoken = from_canonical(&germinate(&a, &b).unwrap());
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
    .is_some_and(|cx| !cx.congruent(&idk).is_empty());
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

fn filed_v2_form() -> (
    Element,
    crate::transducer::stalk::Invariant,
    crate::network::nerve::coord::Holonomy,
) {
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::sentence::{as_element_form, form_torsion, Form, Vertex};
let word_turn_90 = Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("90° turn"), false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
// AT1 (Q1): the two bodies are ONE vertex of the identity's class, a member
// per chart, each overlapped from the Frame's chart.
let mut charts = Vec::new();
for tag in [2u8, 3u8] {
    let v = Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        // Its own chart: an address is derived from its coordinates (54.2).
        crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
        Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        vec![(1, 2)],
        [0],
        None,
    );
    let c = f.stand_member(&v, 0).expect("a word vertex stands");
    f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c.clone(), None).expect("star");
    charts.push(c);
}
// The chord carries the 90° turn as the pivot writes a word on an
// overlap: its left factor (AT1).
let chord = f.place_overlap(charts[0].clone(), charts[1].clone(), None).expect("a chord between two bodies");
f.apply_move(&crate::transducer::sentence::PivotBy::Word {
    face: chord,
    word: Box::new(word_turn_90),
    side: crate::transducer::sentence::Side::Left,
});
f.orbits = Form::orbits_of(&f.faces, &f.vertices);
let _ = f.pivot_orbit(
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
    .entries()
    .next()
    .map(|(k, _)| k)
    .expect("the form files a class");
(e, inv, class)
}

/// Chapter 23.7 on a v2 form: the court reads Form.torsion by the
/// filed orbit's class key, never the element's empty section.
#[test]
fn court_torsion_on_v2_form_is_consilient_or_filed_or_refused() {
use crate::transducer::sentence::{carries_torsion, torsion_orbit_rotor};
let (spoken, inv, class) = filed_v2_form();
assert!(
    spoken.torsion().is_empty(),
    "v2 does not fill the element torsion section"
);
assert!(carries_torsion(&spoken), "the form carries the filed cycle");
assert_ne!(
    class,
    crate::transducer::alphabet::Rotor::identity()
        .holonomy_key()
        .expect("identity class"),
    "the class is the orbit's, not the identity"
);
// 54.7 — AND THE IDENTITY READS AS TRIVIAL, NOT PARABOLIC. `B = 0` turns
// nothing; a parabolic has `B ≠ 0` and `B² = 0`. They shared a byte.
assert_eq!(
    crate::transducer::alphabet::Rotor::identity()
        .holonomy_key()
        .expect("identity class")
        .kind,
    crate::network::nerve::coord::BivectorKind::Trivial,
    "the identity is the absence of a motion, not a null shear"
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
    .is_some_and(|cx| {
        !cx.congruent(crate::network::nerve::Coordinate::encode(&class).as_bytes())
            .is_empty()
    });
assert!(filed, "court torsion filed by the orbit's class key");
assert_eq!(
    k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    None,
    "second hearing is consilient"
);
let f = crate::transducer::sentence::from_element_form(&spoken).expect("v2 form");
let rep = torsion_orbit_rotor(&f, &class).expect("the orbit names the class");
let mut k2 = Kernel::new();
let mut cx = crate::transducer::cech::Complex::new();
// 54.7 — A COURT CANNOT HOLD ONE CLASS AT ANOTHER PERIOD ANY MORE, and
// that is stronger than refusing one that does.
//
// This filed the SAME rotor with a fabricated `99` and expected
// `REFUSE_NOT_NATURAL`. The complex stores `(Rotor, u32)` and the scalar was
// taken as the period, so a court could be told one. It is now re-derived
// from the held rotor (law 12: the store is a cache, the rotor is the
// object), and `class_key` and `plane_periods` are both functions of the
// same two coordinates `(a, p)` — so SAME CLASS KEY IMPLIES SAME SPECTRUM,
// and the disagreement this guarded against cannot be constructed.
//
// The refusal stands for what it was always for: two courts whose ROTORS
// differ. That is a different fixture and not a fabricated integer.
cx.file_congruent([1; 32], [2; 32], rep.clone(), 99);
k2.complexes.insert(inv.key(), cx);
assert_eq!(
    rep.toral_period(),
    rep.toral_period(),
    "the spectrum is the rotor's and the fabricated 99 reaches no reader"
);
assert_eq!(
    k2.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
    None,
    "a fabricated period is not a disagreement: the court derives it from the rotor"
);
}

/// THE CONDENSATION METRIC READS THE CARRIER THAT HOLDS THE FIBRE. A sentence
/// whose form filed a cycle of period ℓ carries ℓ−1 of volume above its base,
/// and none of it is on the element: `Element::fibre_volume` sums the element's
/// own two fibre fields and a v2 sentence leaves both empty, so the basis's
/// volume read 0 for it. The assertion pair is the point — the element reads
/// none and the basis reads three from the same word — and neither number is a
/// property of any corpus, so this separates the two readers on a fixture
/// whether or not anything in the field has ever filed a cycle.
#[test]
fn a_filed_form_cycle_is_volume_the_basis_counts() {
    let (spoken, _, _) = filed_v2_form();
    assert_eq!(
        spoken.fibre_volume(),
        Some(0),
        "a v2 sentence's own fibre fields are empty; the cycle is on its form"
    );
    let held = std::sync::Arc::new(crate::transducer::element::Held::of(spoken));
    let f = held.form().expect("the v2 tape decodes to a form");
    // THE PERIOD IS THE COMPLEX'S (54.5). This read it off the decoded
    // form's `torsion`, which is empty: a form recovered from its address
    // files nothing, because a filing is not geometry. The period the
    // cycle closes at is a theorem of the faces (`orbit_classes`).
    let ell = f
        .orbits
        .iter()
        .find_map(|lp| match f.orbit_classes(lp).as_slice() {
            // THE LEAST CLASS THIS ORBIT DERIVES, and the slice pattern says
            // so: `orbit_classes` traverses the member paths out of a `Keyed`, so
            // the order is the path keys' own and the first is CONTENT, not
            // arrival (rule 15).
            [row, ..] => Some(row.periods),
            [] => None,
        })
        .expect("the complex derives its own period");
    let mut k = Kernel::new();
    k.basis.insert(held.address(), std::sync::Arc::clone(&held));
    // Three readings of one cycle: the form's own reach, the period it was
    // filed at, and the basis's volume over a basis holding just this word.
    // The form is the carrier, so the first is the reading E2 prices and the
    // third must agree with it rather than compute its own.
    // THE VOLUME IS THE SPECTRUM'S OWN YIELD (54.7). It was `ℓ − 1` on the
    // `lcm`; it is `(ℓ₁ − 1) + (ℓ₂ − 1)`, the labour across both invariant
    // planes. Asserted against the spectrum rather than against a number
    // chosen beside it, so the claim is that the form and the basis AGREE
    // with the geometry — which is what this reading was always for.
    assert!(ell.closes(), "the filed cycle closes on both planes");
    let units = ell.yield_units().expect("a closing spectrum has a yield");
    assert_eq!(
        (f.fibre_volume(held.element().filing()), k.fibre_volume_total()),
        (Some(units), Some(units)),
        "the filed cycle's yield reaches the same above the base, on the form and in the basis"
    );
}

/// THE TORSE STROKE tightens mathematics to physics: a swap family (1→4 and
/// 4→1 at one site) installs as base + section; the syntropic stroke extracts
/// the ℤ/2 cycle — sections condense to (σ, 2) — and the extracted
/// mathematical torsion RELIEVES the physical torsion charge (booked as
/// syntropic order, per lane, never a scalar).
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn the_torse_stroke_extracts_the_cycle_and_relieves_torsion() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 9)];
let b = vec![(0, 0, 4), (0, 1, 9)]; // (0,0): 1→4
speak_pair(&mut k, &a, &b, [1; 32]);
assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
k.glue_local();
let rev = vec![(0, 0, 1), (0, 1, 9)];
let fwd = vec![(0, 0, 4), (0, 1, 9)];
speak_pair(&mut k, &fwd, &rev, [1; 32]);
match speak_pair(&mut k, &fwd, &rev, [2; 32]) {
    Outcome::GermPending => k.glue_local(),
    Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
    o => panic!("{o:?}"),
}
assert!(!k.basis().is_empty(), "the pair forms stand");
k.optimize();
assert!(
    matches!(speak_pair(&mut k, &a, &b, [3; 32]), Outcome::Decomposed(_) | Outcome::GermPending),
    "the forward pair still speaks or its residual pends"
);
// A fourth voice: the third may hold its one claim in flight on this
// lane from the forward pair (§3.1 as ruled, S13.15).
assert!(
    matches!(speak_pair(&mut k, &fwd, &rev, [4; 32]), Outcome::Decomposed(_) | Outcome::GermPending),
    "the reverse pair is the same form or its residual"
);
}

/// G4b. An orbit's obstruction is the product around it (10.3), never a
/// transition read off the end of the fiber. The distinction is not
/// cosmetic on a distributed mesh: the geometric product is not
/// commutative, so a member picked by position is a different object on
/// every node that holds the fiber in a different insertion order, and
/// two nodes would breed two germs from identical mathematics.
#[test]
fn a_held_fiber_germinates_its_holonomy_not_a_member() {
use crate::transducer::alphabet::{LetterId, Word};
let q = LetterId::turn_90((0, 1)).expect("90° turn");
let one = rotor_versor(&Word::from_letters(&[(q, false)]).rotor()).expect("versor");
let inv = rotor_versor(&Word::from_letters(&[(q, true)]).rotor()).expect("inverse versor");
let id = element::identity_versor();

assert!(fiber_holonomy(std::slice::from_ref(&one)).is_none(), "a singleton is not an orbit");
assert!(fiber_holonomy(&[]).is_none(), "an empty fiber is not an orbit");

let closed = fiber_holonomy(&[one.clone(), inv]).expect("an orbit");
assert!(closed.projective_eq(&id), "q then q-inverse closes");

// Three 90° turns on one plane: the product is the obstruction,
// and it is not any member of the fiber.
let held = fiber_holonomy(&[one.clone(), one.clone(), one.clone()]).expect("an orbit");
assert!(!held.projective_eq(&id), "three 90° turns do not close");
assert!(
    !held.projective_eq(&one),
    "the holonomy is the composite, not the transition the fiber ends on"
);
}
