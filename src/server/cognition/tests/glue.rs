// 21 step 3, 13.6 — THE GLUE AT THE COURT: a form that cites a held form
// replays its descent through the genome that holds the citation.

use super::common::*;
use crate::cognition::*;
use crate::transducer::element::Element;

/// A glued form's descent is measured through the court genome as Crystal.
#[test]
fn glued_form_replays_descent_through_the_court_genome() {
use crate::transducer::family::{
    apply_form, extract_rule_with, mint_sentence, pair_form_uncrystallized, Crystal, Grid, Speech,
};
use crate::transducer::sentence::{as_element_form, Form, Vertex};
use crate::transducer::stalk::{Invariant, Standing};
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
];
let held_form = pair_form_uncrystallized(&family[0].0, &family[0].1, None);
let held_e = as_element_form(&held_form, [1, 0, 0, 0]).expect("inner tape");
struct Held {
    e: Element,
    f: Form,
}
impl Crystal for Held {
    fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
        vec![(self.e.clone(), self.f.clone(), Standing::Held)]
    }
    fn on_body(&self, c: &Invariant) -> Vec<(Element, Form, Standing)> {
        self.on_class(c)
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, addr: &[u8]) -> Option<(Element, Form)> {
        (addr == self.e.address().as_bytes()).then(|| (self.e.clone(), self.f.clone()))
    }
    fn on_axis(
        &self,
        _chart: &Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        let mut index = crate::transducer::sentence::AxisIndex::new();
        crate::transducer::sentence::admit_words(
            &mut index,
            &Invariant::default(),
            &crate::transducer::sentence::words_of_form(&self.f),
        );
        crate::transducer::sentence::on_axis_of(&index, &Invariant::default(), axis)
    }
    /// This fixture's claim is the glue, not the language: it holds one
    /// form to be cited and offers no presentation to derive against.
    fn generators_under(
        &self,
        _chart: &Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        Vec::new()
    }
}
let cover = Held {
    e: held_e.clone(),
    f: held_form,
};
let rule = extract_rule_with(&family, &cover, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("glues");
assert!(
    rule.form
        .vertices
        .iter()
        .any(|v| matches!(v, Vertex::Glued { .. })),
    "the rule cites the held form"
);
let spoken = mint_sentence(&rule, &family, &cover).expect("mint through crystal");
let mut k = Kernel::new();
let inner_addr = held_e.address();
assert_eq!(
    k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&held_e)),
    vec![crate::anastome::dialogue::SPEECH_STOOD]
);
k.rule_all_provisionals();
// 47.3: the witness is a vantage with room. Bonded, its re-derivation
// installs the held form, and the install releases the voice's draw so
// the same voice has room for the sentence that cites it (23.8).
let cover = k.genesis_cover().expect("measured");
k.enroll([2; 32], cover);
k.confirm_peer_holds_addrs(std::slice::from_ref(&inner_addr), [2; 32]);
let task = [9u8; 32];
k.bind_task_family(task, family);
let codes = k.perceive_speech([1; 32], &task, std::slice::from_ref(&spoken));
assert_eq!(
    codes,
    vec![crate::anastome::dialogue::SPEECH_STOOD],
    "glued descent replays through the court genome, not NoCrystal"
);
}

/// 21 step 3 → 26, ON THE LIVE PATH: a voice that stood a form cites it in
/// the rule it extracts for a family that form closes part of, and the
/// court records the citation as a use of the cited address (`cited_by`).
/// This is the chain the index fault of L8r held shut end to end — no
/// `Glued` vertex, so no `cited_by`, so nothing a citation could ever pay
/// for (26, 27) — measured here through the epigenome the fleet runs and
/// not through a stub.
#[test]
fn a_cited_form_is_a_use_at_the_court() {
use crate::client::sdk::Epigenome;
use crate::transducer::family::{extract_rule_with, mint_sentence, pair_form_uncrystallized, sentence_measure, NoCrystal};
use crate::transducer::sentence::{as_element_form, Vertex};
fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
];
// The sibling the voice solved and stood: one pair of the bundle.
let held = pair_form_uncrystallized(&family[0].0, &family[0].1, None);
let mag = sentence_measure(&held, &family[..1], &NoCrystal).expect("the body is spoken");
let held_e = as_element_form(&held, mag).expect("a tape");
let mut epi = Epigenome::new();
epi.stand_dialect_at(held_e.clone(), 1, 1).expect("stood");

// The rule the voice extracts over the whole bundle cites it.
let rule = extract_rule_with(&family, &epi, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family extracts");
assert!(
    rule.form.vertices.iter().any(|v| matches!(v, Vertex::Glued { .. })),
    "the voice cites what it stood"
);
let spoken = mint_sentence(&rule, &family, &epi).expect("minted over the voice's own crystal");

// The court installs the cited form, then the citer.
let mut k = Kernel::new();
let inner = held_e.address();
assert_eq!(
    k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&held_e)),
    vec![crate::anastome::dialogue::SPEECH_STOOD]
);
k.rule_all_provisionals();
let cover = k.genesis_cover().expect("measured");
k.enroll([2; 32], cover);
k.confirm_peer_holds_addrs(std::slice::from_ref(&inner), [2; 32]);
assert!(k.ruled_contains(&inner), "the cited form is language");

let task = [9u8; 32];
k.bind_task_family(task, family);
let outer = spoken.address();
assert_eq!(
    k.perceive_speech([1; 32], &task, std::slice::from_ref(&spoken)),
    vec![crate::anastome::dialogue::SPEECH_STOOD],
    "the citer replays through the genome that holds the citation"
);
k.rule_all_provisionals();
k.confirm_peer_holds_addrs(std::slice::from_ref(&outer), [2; 32]);
assert!(k.ruled_contains(&outer), "and installs");

let uses = k
    .cited_by
    .get(inner.as_bytes())
    .expect("the cited address has citers");
assert!(
    uses.contains(outer.as_bytes()),
    "the court records the citation as a use of the cited address (26)"
);
}
