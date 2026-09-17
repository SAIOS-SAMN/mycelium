// 50.9 — THE CONTRACTION MOVES ON THE FORM: a still orbit and a face that
// does not re-derive each shrink the closed sub-complex, and the court's
// reader recognises each.

use super::common::*;
use crate::transducer::alphabet::Contact;
use crate::transducer::sentence::*;
use crate::transducer::stalk;

/// 50.9 move 5 — A STILL ORBIT CONTRACTS ITS CONTACT WORD TO A DEMAND. The
/// refused contact vertex becomes a demand over its chart carrying the
/// links it was to reach, its star face leaves with it, and the court's
/// reader finds the contraction at that coordinate; a coordinate the form
/// does not hold contracts nothing.
#[test]
fn a_still_orbit_contracts_its_contact_word_to_a_demand() {
    let mut f = closed_word_form(order4(), Some(Contact { anchor: 0, ring: 1 }));
    let Some(Vertex::Word { address, contact: Some(_), .. }) = f.vertices.get(1).cloned() else {
        panic!("the fixture's word is a contact word");
    };
    let chart = stalk::Invariant(vec![3, 5]);
    let at = At::Vertex { address: address.clone(), chart: chart.clone() };
    let remainder = vec![(0i64, 1i64, 1u8, 2u8)];
    assert!(f.faces.iter().any(|face| face.j == 1), "the contact word has its star face");
    assert_eq!(f.contract(RefusalKind::OrbitStill, &at, &remainder), Some(Contracted::Moved));
    assert!(
        matches!(f.vertices.get(1), Some(Vertex::Demand { kind: Demand::Creation, chart: c, bodies }) if *c == chart && *bodies == remainder),
        "the contact word is now a demand over its chart with the links it was to reach: {:?}",
        f.vertices
    );
    assert!(!f.faces.iter().any(|face| face.j == 1), "and its faces left with it");
    assert!(f.is_contraction_at(RefusalKind::OrbitStill, &at), "the court finds the contraction at the coordinate");
    let elsewhere = At::Vertex { address: form_addr(9), chart };
    let mut g = closed_word_form(order4(), Some(Contact { anchor: 0, ring: 1 }));
    assert_eq!(g.contract(RefusalKind::OrbitStill, &elsewhere, &[]), None, "a vertex the form does not hold contracts nothing");
}

/// 50.9 move 6 — A FACE THAT DOES NOT RE-DERIVE IS DROPPED. The chord between
/// two words leaves the form, the loop through it dissolves, both words
/// still act, the court's reader finds no face between the two, and the
/// same coordinate contracts nothing a second time.
#[test]
fn a_face_that_does_not_rederive_is_dropped_by_its_contraction() {
    let mut f = chord_form(&[order4()]);
    assert_eq!(f.loops.len(), 1, "the chord closes one loop");
    let at = At::Face(form_addr(1), form_addr(2));
    assert_eq!(f.contract(RefusalKind::FaceDerivation, &at, &[]), Some(Contracted::Moved));
    assert!(
        !f.faces.iter().any(|face| (face.i == 1 && face.j == 2) || (face.i == 2 && face.j == 1)),
        "no face stands between the two words"
    );
    assert!(f.loops.is_empty(), "the loop through the chord dissolved");
    assert!(f.acts(), "both words still act");
    assert!(f.is_contraction_at(RefusalKind::FaceDerivation, &at), "the court finds no face between them");
    assert_eq!(f.contract(RefusalKind::FaceDerivation, &at, &[]), None, "a face already gone contracts nothing");
}
