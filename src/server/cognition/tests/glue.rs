// 21 step 3, 13.6 — THE GLUE AT THE COURT: a form that cites a held form
// replays its descent through the genome that holds the citation.

use super::common::*;
use crate::cognition::*;
use crate::transducer::element::Element;

/// A glued form's descent is measured through the court genome as Crystal.
#[test]
fn glued_form_replays_descent_through_the_court_genome() {
use crate::transducer::family::{
    apply_form, extract_rule_with, mint_sentence, pair_form, Crystal, Grid, Speech,
};
use crate::transducer::sentence::{as_element_form, Form, Vertex};
use crate::transducer::stalk::{Invariant, Standing};
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
];
let held_form = pair_form(&family[0].0, &family[0].1, None);
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
}
let cover = Held {
    e: held_e.clone(),
    f: held_form,
};
let rule = extract_rule_with(&family, &cover).expect("glues");
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
