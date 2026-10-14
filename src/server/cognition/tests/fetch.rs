// 78.100 PS1 (2) — THE READING BEFORE THE COURT-TO-COURT RELAY: a word served
// to a peer court is read against the words that court said it holds on the
// word's chart, and a refracted word's bytes whole are read beside the bytes
// it would take as (held, Γ).

use super::common::*;
use crate::cognition::*;
use crate::transducer::sentence::{as_element_form, from_element_form, Transport};
use crate::cognition::Asker;

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
    let peer = Asker::Court([0x3b; 32]);
    let page = k.serve_ruled(&inv, &[spoken.address()], 1 << 20, peer);
    assert_eq!(page.len(), 1, "the peer is served what it does not hold");
    let acts = k.acts_reading();
    assert_eq!(acts.fetch[axis("words: refracted (held, Γ)")], 1, "{:?}", acts.lines());
    let whole = acts.fetch[axis("bytes refracted, whole")];
    let relay = acts.fetch[axis("bytes refracted, as (held, Γ)")];
    assert!(whole > 0 && relay > 0, "both measured: whole {whole}, as (held, Γ) {relay}");
    assert_eq!(acts.fetch[axis("bytes served")], whole, "the one word served was the refracted one");
    let _ = k.serve_ruled(&inv, &[], 1 << 20, peer);
    assert_eq!(k.acts_reading().fetch[axis("words: unanchored")], 2, "a peer holding nothing anchors nothing");
}

/// 78.60(d), 78.100 — A VOICE'S FETCH IS PAID BY THE VOICE. A voice with no
/// balance at this court is served nothing and counted short — this court's
/// reserve is not drawn; funded, it is served and pays this court the word's
/// magnitude, and the court's reserve stands where it stood.
#[test]
fn a_voice_pays_for_its_fetch_and_a_short_voice_waits() {
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    let mut k = Kernel::new();
    k.bind_identity([0x4a; 32]);
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    let w = k.provisional.scoped().next().expect("a provisional sentence").1.clone();
    let inv = crate::transducer::sentence::chart::chart_of(&w).expect("a chart");
    let addr = w.address();
    k.ruled.admit(&inv, addr.clone());
    k.basis_insert(w.clone(), [1; 32]);
    let mut filing = k.stroke();
    k.file_admitted(&addr, crate::chronometry::Stamp { h: 1, vk: [1; 32], k: 1, i: 0 }, &mut filing);
    let axis = |n: &str| crate::observe::FETCH.iter().position(|a| *a == n).expect("a named axis");
    let voice = [0x4b; 32];
    let reserve = k.physics.reserve.lanes();
    assert!(k.serve_ruled(&inv, &[], 1 << 20, Asker::Voice(voice)).is_empty(), "a voice with nothing at this court waits");
    assert_eq!(k.acts_reading().fetch[axis("deferred: learner short")], 1);
    assert_eq!(k.physics.reserve.lanes(), reserve, "this court's reserve is not drawn for a voice");
    let mag = w.magnitude();
    let cover = k.genesis_cover().expect("measured");
    k.enroll(voice, cover);
    k.fund(voice, mag);
    let before = k.treasury().balance_mu(&voice);
    assert_eq!(k.serve_ruled(&inv, &[], 1 << 20, Asker::Voice(voice)).len(), 1, "funded, it is served");
    let after = k.treasury().balance_mu(&voice);
    assert_eq!(std::array::from_fn::<u64, 4, _>(|l| before[l] - after[l]), mag, "and pays the word's magnitude");
    assert_eq!(k.acts_reading().fetch[axis("served to voices")], 1);
    assert_eq!(k.physics.reserve.lanes(), reserve, "the court's reserve stands where it stood");
    // A have-list that leaves the word out does not charge for it twice:
    // what this court taught this learner is this court's to know.
    k.fund(voice, mag);
    let funded = k.treasury().balance_mu(&voice);
    assert!(k.serve_ruled(&inv, &[], 1 << 20, Asker::Voice(voice)).is_empty(), "a word taught is not sent again");
    assert_eq!(k.treasury().balance_mu(&voice), funded, "nor charged again");
}

/// A PAGE TAKES NO WORD ITS REPLY CANNOT CARRY (2026-10-14). The reply counts
/// its words in one byte across every stratum's page; a page that took past
/// the room left was packed, counted served and drawn from the reserve, and
/// the reply then refused to encode. With room for one of two words, one is
/// served, one counted, and the reserve drawn for one.
#[test]
fn a_page_takes_no_word_its_reply_cannot_carry() {
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    let mut k = Kernel::new();
    k.bind_identity([0x5a; 32]);
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    let spoken = k.provisional.scoped().next().expect("a provisional sentence").1.clone();
    let held = from_element_form(&spoken).expect("a sentence");
    let moved = Transport { word: crate::transducer::alphabet::Word::identity(), carriage: crate::transducer::element::translator_versor(1, 0), gauge: Vec::new() }
        .refract(&held)
        .and_then(|f| as_element_form(&f, spoken.magnitude()))
        .expect("the refracted sentence is a word");
    let inv = crate::transducer::sentence::chart::chart_of(&spoken).expect("a chart");
    let mut filing = k.stroke();
    for (i, w) in [(0u64, &spoken), (1, &moved)] {
        let addr = w.address();
        k.ruled.admit(&inv, addr.clone());
        k.basis_insert(w.clone(), [1; 32]);
        k.file_admitted(&addr, crate::chronometry::Stamp { h: 1, vk: [1; 32], k: 1, i }, &mut filing);
    }
    let axis = |n: &str| crate::observe::FETCH.iter().position(|a| *a == n).expect("a named axis");
    let reserve = k.physics.reserve.lanes();
    let seed = k.syllabus_seed();
    let mut reads = k.physics.stroke();
    let page = k.serve_ruled_on(&inv, &[], (1 << 20, 1), Asker::Court([0x5b; 32]), &seed, &mut reads);
    assert_eq!(page.len(), 1, "room for one: one word");
    assert_eq!(k.acts_reading().fetch[axis("served to courts")], 1, "one counted served");
    let drawn: [u64; 4] = std::array::from_fn(|l| reserve[l] - k.physics.reserve.lanes()[l]);
    let one = [spoken.magnitude(), moved.magnitude()];
    assert!(one.iter().any(|m| *m == drawn), "the reserve drawn for the one word served: {drawn:?}");
}
