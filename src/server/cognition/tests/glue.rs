// 21 step 3, 13.6 — THE GLUE AT THE COURT: a form that cites a held form
// replays its trajectory through the genome that holds the citation.

use super::common::*;
use crate::cognition::*;
use crate::transducer::element::Element;

/// A glued form's trajectory is measured through the court genome as Crystal.
#[test]
fn perceive_speech_replays_glued_form_through_court_genome() {
use crate::transducer::family::{
    apply_form, extract_rule_with, mint_sentence, cipher_form, Crystal, Grid, Speech,
};
use crate::transducer::sentence::{as_element_form, Form, Vertex};
use crate::transducer::stalk::{Invariant, Standing};
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
];
let held_form = cipher_form(&family[0].0, &family[0].1);
let held_e = as_element_form(&held_form, [1, 0, 0, 0]).expect("inner tape");
struct Held {
    e: Element,
    f: Form,
}
impl Crystal for Held {
    fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        vec![(&self.e, &self.f, Standing::Held)]
    }
    fn on_body(&self, c: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        self.on_class(c)
    }
    /// 21 step 3 — the question the glue asks: the held form, at exactly the
    /// acts its own word vertices perform (`act_key`), as the mind's crystal
    /// files a closed form (`act_keys_of`) — and at no other act.
    fn on_act(&self, act: &[u8]) -> Vec<(&Element, &Form, Standing)> {
        let acts_so = self.f.vertices.scoped().any(|(_, v)| {
            let Vertex::Word { derivation, acts, .. } = v else { return false };
            acts.scoped().any(|(_, a)| {
                crate::transducer::sentence::presentation::act_key(&a.chart, &derivation.rotor(), &a.carriage, &a.gauge).as_deref() == Some(act)
            })
        });
        match acts_so && self.f.is_closed() {
            true => vec![(&self.e, &self.f, Standing::Held)],
            false => Vec::new(),
        }
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
        (addr == self.e.address().as_bytes()).then(|| (&self.e, std::borrow::Cow::Borrowed(&self.f)))
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
            &crate::transducer::sentence::acting_of_form(&self.f),
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
        .entries()
        .any(|(_, v)| matches!(v, Vertex::Glued { .. })),
    "the rule cites the held form"
);
let spoken = mint_sentence(&rule, &family, &cover, 0).expect("mint through crystal");
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
    "the glued trajectory replays through the court genome, not NoCrystal"
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
fn cited_by_records_glued_citation_as_use() {
use crate::client::sdk::Epigenome;
use crate::transducer::family::{extract_rule_with, mint_sentence, cipher_form, sentence_measure, NoCrystal};
use crate::transducer::sentence::{as_element_form, Vertex};
fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
];
// The sibling the voice solved and stood: one pair of the bundle.
let held = cipher_form(&family[0].0, &family[0].1);
let mag = sentence_measure(&held, &family[..1], &NoCrystal, 0).expect("the body is spoken");
let held_e = as_element_form(&held, mag).expect("a tape");
let mut epi = Epigenome::new();
epi.stand_dialect_at(held_e.clone(), 1, 1).expect("stood");

// The rule the voice extracts over the whole bundle cites it.
let rule = extract_rule_with(&family, &epi, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family extracts");
assert!(
    rule.form.vertices.entries().any(|(_, v)| matches!(v, Vertex::Glued { .. })),
    "the voice cites what it stood"
);
let spoken = mint_sentence(&rule, &family, &epi, 0).expect("minted over the voice's own crystal");

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

/// ONE CONSTRUCTION OF A STRATUM'S ROOT, ACROSS THREE SITES. `heard_mark`
/// stores the root of the chart an address was fetched under, and
/// `confirm_peer_holds` drops a heard address when the PEER's digest for
/// that chart shows a different root. Home and peer therefore have to build
/// that root the same way — and they built it in three hand-rolled copies:
/// `sheaf::root_of`, one inlined in `confirm_peer_holds`, and
/// `Kernel::stratum_root_of`.
///
/// Nothing asserted they agreed. When the root became a root, the copy in
/// `stratum_root_of` still returned the concatenation, so every heard
/// address compared a concatenation against a hash, differed, and was
/// dropped on every confirm — and the suite stayed green, because the
/// install path used the other two copies and this drop path had no test.
#[test]
fn confirm_peer_holds_keeps_heard_on_agreeing_root() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
k.rule_all_provisionals();
let strata = k.ruled.strata();
let Some((inv, _)) = strata.into_iter().next() else {
    panic!("the ruled word stands in a stratum");
};

// An address heard under that chart, marked with the root as it stands.
let overheard = vec![0xABu8; 32];
k.heard_mark(&overheard, &inv);
assert!(k.heard.contains(&overheard), "it is heard");

// A digest that AGREES: the court's own, so the root is by construction
// the one `heard_mark` stored. The three constructions must agree here or
// the address is dropped for a disagreement that does not exist.
let agreeing = k.ruled_digests();
k.confirm_peer_holds(&agreeing, [2; 32]);
assert!(
    k.heard.contains(&overheard),
    "a confirm that agrees drops nothing: the stored root and the digest's are one construction"
);

// And a digest that genuinely disagrees still drops it.
let mut moved = k.ruled_digests();
if let [d, ..] = moved.as_mut_slice() {
    d.root = vec![0u8; 32];
}
k.confirm_peer_holds(&moved, [2; 32]);
assert!(
    !k.heard.contains(&overheard),
    "a root that moved drops the address heard under the old one"
);
}

/// A HAVE-LIST IS ONE CHART'S, NOT THE WHOLE OF `heard`. `fetch_have`
/// unioned every heard address into every chart's have-list, and it is
/// called once per stratum, per peer, per exchange cycle. A peer serving
/// the fetch skips an address only when it stands in the fiber over that
/// `inv` (`sheaf::serve`), so an address heard under another chart could
/// never match — it filtered nothing and cost wire, allocation and a read
/// of a set that grows with peers × corpus.
#[test]
fn fetch_have_carries_only_this_charts_heard() {
use crate::transducer::stalk::Invariant;
let mut k = Kernel::new();
let here = Invariant::counts(&[1]);
let elsewhere = Invariant::counts(&[2]);
let heard_here = vec![0xA1u8; 32];
let other = vec![0xB2u8; 32];
k.heard_mark(&heard_here, &here);
k.heard_mark(&other, &elsewhere);

let have = k.fetch_have(&here);
assert!(have.contains(&heard_here), "the address heard under this chart is held: {have:?}");
    assert!(!have.contains(&other), "the address heard under another chart could never match a fetch here: {have:?}");

    // Re-marked under another chart, an address leaves the old fiber: an
    // index that only grows answers a stratum with addresses no longer heard
    // under it.
    k.heard_mark(&heard_here, &elsewhere);
    let have = k.fetch_have(&here);
    assert!(
        !have.contains(&heard_here),
        "it moved charts and left this one's fiber: {have:?}"
    );
    assert!(k.fetch_have(&elsewhere).contains(&heard_here), "and joined the other's");
}

/// R76: A GLUED SPOKEN WORD SECTIONS AND EDGES WITH TRUE GEOMETRY.
/// `cx.set_section` and `cx.set_edge` receive the carried rotor and versor
/// from the spoken element and transition.
#[test]
fn glue_fetched_records_true_section_geometry() {
    let own_vk = [0x0au8; 32];
    let peer_id = [0x0bu8; 32];
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];

    let mut own = Kernel::new();
    own.bind_identity(own_vk);
    assert_eq!(speak_pair(&mut own, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(speak_pair(&mut own, &a, &b, [2; 32]), Outcome::GermPending);
    let spoken = own.provisional.scoped().next().unwrap().1.clone();
    let inv = crate::transducer::stalk::invariant_of(&spoken);

    // Ruling own provisional creates own_section with spoken's geometry
    // (78.153: on every stroke the replay takes).
    own.court_settle();
    let cx = own.complexes().get(&inv.key()).expect("complex exists");
    let own_sec = cx.sections.get(own_vk.as_slice()).expect("own section exists");
    let exp_gamma = match spoken.carried_rotor() {
        Some(r) => r,
        None => crate::transducer::alphabet::Rotor::identity(),
    };
    let exp_carriage = match spoken.versor() {
        Some(c) => c,
        None => crate::transducer::element::identity_versor(),
    };
    assert_eq!(own_sec.gamma, exp_gamma);
    assert_eq!(own_sec.carriage, exp_carriage);

    // A peer arrives with spoken
    let mut heard = Kernel::new();
    heard.bind_identity(own_vk);
    let mut cover = crate::network::nerve::Cover::new();
    let outcome = fetched(&mut heard, &spoken, peer_id, own_vk, &mut cover);
    assert!(matches!(outcome, Glue::Admitted));

    let cx_h = heard.complexes().get(&inv.key()).expect("complex exists");
    let peer_sec = cx_h.sections.get(peer_id.as_slice()).expect("peer section exists");
    assert_eq!(peer_sec.gamma, exp_gamma);
    assert_eq!(peer_sec.carriage, exp_carriage);
}


/// PHASE 1 ITEM 1 — A PEER'S SENTENCE IS READ AGAINST THIS COURT'S SECTION ON
/// ITS CHART, exactly, and counted per peer: this court's own word read back
/// is the same word; the same word refracted by a translation reads as a
/// refraction (the instrument the court-triangle balance is read from).
#[test]
fn glue_fetched_reads_peer_sentence_against_own_section() {
    use crate::observe::rep;
    use crate::transducer::sentence::{as_element_form, from_element_form, Transport};
    let own_vk = [0x1au8; 32];
    let (same_peer, refracting_peer) = ([0x1bu8; 32], [0x1cu8; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    let mut own = Kernel::new();
    own.bind_identity(own_vk);
    assert_eq!(speak_pair(&mut own, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(speak_pair(&mut own, &a, &b, [2; 32]), Outcome::GermPending);
    let spoken = own.provisional.scoped().next().unwrap().1.clone();
    // 78.153: every stroke the docket's replay takes.
    own.court_settle();
    let axes = |peer: [u8; 32]| {
        crate::observe::replication().signers.into_iter().find(|(k, _)| *k == peer).map(|(_, a)| a)
    };
    let mut cover = crate::network::nerve::Cover::new();
    // 78.153: a stroke that holds the glue's replay.
    let mut glue = crate::physics::Physics::default().stroke_n(1 << 20);
    let _ = own.glue_fetched(spoken.clone(), same_peer, own_vk, &mut cover, &mut glue);
    assert_eq!(axes(same_peer).map(|a| a[rep::RELATION_SAME]), Some(1), "its own word is the same word: {:?}", axes(same_peer));
    assert_eq!(axes(same_peer).map(|a| a[rep::PERCEIVED_COHERENT]), Some(1), "and is coherent on all four lanes: {:?}", axes(same_peer));
    let paths = |a: [u64; crate::observe::REP_AXES]| (rep::GLUE_HEARD..=rep::GLUE_ADMITTED).map(|i| a[i]).sum::<u64>();
    assert_eq!(axes(same_peer).map(paths), Some(1), "the word left the glue by exactly one path: {:?}", axes(same_peer));
    let held = from_element_form(&spoken).expect("a sentence");
    let moved = Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage: crate::transducer::element::translator_versor(1, 0),
        gauge: Vec::new(),
    }
    .refract(&held)
    .and_then(|f| as_element_form(&f, spoken.magnitude()))
    .expect("the refracted sentence is a word");
    let outcome = own.glue_fetched(moved, refracting_peer, own_vk, &mut cover, &mut glue);
    assert_eq!(
        axes(refracting_peer).map(|a| a[rep::RELATION_REFRACTED]),
        Some(1),
        "a translation of it reads as a refraction ({outcome:?}): {:?}",
        axes(refracting_peer)
    );
    assert_eq!(axes(refracting_peer).map(|a| a[rep::PERCEIVED_COHERENT]), Some(1), "and is coherent with it: {:?}", axes(refracting_peer));
}

/// 78.63 — AN ATTACHED WORD IS HELD. A peer's word the same as this court's
/// section but at another placement glues as Attached; it was left unheld,
/// so the peer sent it again on every pass and the orbit never advanced.
/// Now it is held — in the basis and the ruled cover, so the next fetch's
/// have-list carries it and the same shear is not read again.
#[test]
fn glue_fetched_holds_attached_word() {
    let own_vk = [0x2au8; 32];
    let peer = [0x2bu8; 32];
    let word = |x: i64| {
        crate::transducer::element::from_connection(
            &crate::transducer::alphabet::Word::identity(),
            &crate::transducer::element::translator_versor(x, 0),
            &[(1, 2), (2, 1)],
            &Vec::new(),
            [1, 1, 0, 0],
        )
        .expect("a connection")
    };
    let mut own = Kernel::new();
    own.bind_identity(own_vk);
    let held = word(1);
    assert_eq!(own.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&held)), vec![crate::anastome::dialogue::SPEECH_STOOD]);
    own.court_stroke();
    let peer_word = word(2);
    let mut cover = crate::network::nerve::Cover::new();
    let outcome = fetched(&mut own, &peer_word, peer, own_vk, &mut cover);
    if outcome == Glue::Attached {
        assert!(own.word_at(peer_word.address().as_bytes()).is_some(), "the attached word is held");
        let inv = crate::transducer::stalk::invariant_of(&peer_word);
        assert!(own.fetch_have(&inv).iter().any(|a| a.as_slice() == peer_word.address().as_bytes()), "and the next fetch's have-list carries it");
    } else {
        panic!("the fixture must attach: it glued {outcome:?}");
    }
}
