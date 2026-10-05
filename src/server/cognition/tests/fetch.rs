// 78.100 PS1 (2) — THE READING BEFORE THE COURT-TO-COURT RELAY: a word served
// to a peer court is read against the words that court said it holds on the
// word's chart, and a refracted word's bytes whole are read beside the bytes
// it would take as (held, Γ).

use super::common::*;
use crate::cognition::*;
use crate::transducer::sentence::{as_element_form, from_element_form, Transport};

/// A PEER HOLDING THE WORD IS SERVED ITS REFRACTION, READ AS ONE. This court
/// installs a sentence and the same sentence refracted by a translation; a
/// peer whose have-list names the first is served the second, which the
/// fetch reads as refracted against it, with its bytes whole and as
/// (held, Γ). A peer holding nothing is served both, unanchored.
#[test]
fn a_word_served_to_a_peer_is_read_against_what_the_peer_holds() {
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    let mut k = Kernel::new();
    k.bind_identity([0x3a; 32]);
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    let spoken = k.provisional.scoped().next().expect("a provisional sentence").1.clone();
    let held = from_element_form(&spoken).expect("a sentence");
    let moved = Transport { word: crate::transducer::alphabet::Word::identity(), carriage: crate::transducer::element::translator_versor(1, 0), gauge: Vec::new() }
        .refract(&held)
        .and_then(|f| as_element_form(&f, spoken.magnitude()))
        .expect("the refracted sentence is a word");
    let inv = crate::transducer::sentence::chart::chart_of(&spoken).expect("a chart");
    assert_eq!(crate::transducer::sentence::chart::chart_of(&moved), Some(inv.clone()), "a refraction stands on its anchor's chart");
    let mut filing = k.stroke();
    for (i, w) in [(0u64, &spoken), (1, &moved)] {
        let addr = w.address();
        k.ruled.admit(&inv, addr.clone());
        k.basis_insert(w.clone(), [1; 32]);
        k.file_admitted(&addr, crate::chronometry::Stamp { h: 1, vk: [1; 32], k: 1, i }, &mut filing);
    }
    let axis = |n: &str| crate::observe::FETCH.iter().position(|a| *a == n).expect("a named axis");
    let page = k.serve_ruled(&inv, &[spoken.address()], 1 << 20);
    assert_eq!(page.len(), 1, "the peer is served what it does not hold");
    let acts = k.acts_reading();
    assert_eq!(acts.fetch[axis("words: refracted (held, Γ)")], 1, "{:?}", acts.lines());
    let whole = acts.fetch[axis("bytes refracted, whole")];
    let relay = acts.fetch[axis("bytes refracted, as (held, Γ)")];
    assert!(whole > 0 && relay > 0, "both measured: whole {whole}, as (held, Γ) {relay}");
    assert_eq!(acts.fetch[axis("bytes served")], whole, "the one word served was the refracted one");
    let _ = k.serve_ruled(&inv, &[], 1 << 20);
    assert_eq!(k.acts_reading().fetch[axis("words: unanchored")], 2, "a peer holding nothing anchors nothing");
}
