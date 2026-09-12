// 24.1, 24.3 — THE CITE OVER THE WORKING ATLAS: what a stood word, a held
// form's word and a served listing deflate, and what a withheld form does
// not utter.

use super::common::*;
use crate::client::sdk::*;

/// The word of a chain a fixture wrote out: one entry per position, so A5
/// accepts it and the fixture reads the word it names.
fn chain_word(cells: &[(i64, i64, u8, u8)]) -> crate::transducer::element::Element {
    crate::transducer::element::from_canonical(
        &crate::transducer::element::canonical_chain(cells)
            .expect("one entry per position is A5's chain"),
    )
}

/// Quote incidence is the working atlas: genome ∪ dialect ∪ the held
/// forms' words, read at the chart (24.1).
#[test]
fn quote_incidence_is_the_working_atlas() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("atlas"));
assert!(
    !entity.incident_chart(&crate::transducer::stalk::invariant_of(&e1)),
    "empty genome and dialect are not incident"
);
entity.remember_spoken(&e1);
let inv = crate::transducer::stalk::invariant_of(&e1);
assert!(entity.incident_chart(&inv), "a dialect chart is working incidence");
let fiber = entity.working_atlas().fiber(&inv);
assert!(
    fiber.iter().any(|(_, s)| *s == Standing::Stood),
    "a dialect fiber carries Stood, not a boolean"
);
assert!(
    !entity.incident_chart(&crate::transducer::stalk::Invariant(vec![99])),
    "an unseen chart is not incident"
);
let idea_chain = vec![(3, 0, 5, 6), (3, 1, 5, 6)];
entity.hold_chain(idea_chain.clone());
let idea = crate::transducer::stalk::invariant(&idea_chain);
assert!(entity.ideas_len() > 0, "the idea is held");
assert!(entity.incident_chart(&idea), "an idea chart is working incidence");
assert!(
    entity.working_atlas().fiber(&idea).iter().any(|(_, s)| *s == Standing::Held),
    "an idea fiber carries Held"
);
let (ruled, stood, held) = entity.atlas_standing_counts();
assert_eq!(ruled, 0, "genome empty");
assert!(stood >= 1, "a dialect word is stood");
assert!(held >= 1, "an idea is held");
}

/// A stood residual word is cited by the next chain it deflates, and a
/// held remainder is cited with standing Held — never as covering itself.
#[test]
fn cite_reads_stood_and_held_words() {
let residual = vec![(0i64, 0i64, 1u8, 2u8)];
let next = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 3, 4)];
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("cite-stood"));
assert!(
    entity.epigenome.express(&residual, 1, 1),
    "the residual stands in the dialect"
);
let (cites, _rem) = entity.cite(&next);
assert!(
    cites.iter().any(|c| c.standing == Standing::Stood.as_u8()),
    "the stood residual is cited with standing Stood, got {cites:?}"
);

let mut held = Entity::open("127.0.0.1:1", Kernel::named("cite-held"));
held.hold_chain(residual.clone());
let (self_cites, self_rem) = held.cite(&residual);
assert!(
    self_cites.is_empty(),
    "a remainder is not a citation of itself, got {self_cites:?}"
);
assert_eq!(self_rem, residual, "the remainder stays the residual");
let (cites, _rem) = held.cite(&next);
assert!(
    cites.iter().any(|c| c.standing == Standing::Held.as_u8()),
    "the held remainder is cited with standing Held, got {cites:?}"
);
}

/// 24.1 as ruled 2026-09-09 — A HELD FORM'S WORD, NOT ITS CHAIN, CITES
/// THE NEXT LISTING. A recolour held as the pair form of its scene has one
/// Word vertex; the next listing carries the same body recoloured
/// elsewhere beside a body nothing speaks. The cite is read at the body's
/// chart — through the contemplation's `by_chart` and the form's
/// `by_class` — and the vertex's action is evaluated on the listing body:
/// the cite names the vertex by its address (the M2 key of the body the
/// word speaks, A21 §4), carries the body it reconstructed in the
/// listing's own frame, never the listing chain's key; the residual is
/// exactly the body no word spoke; and no element for either chain exists
/// anywhere on the working language, because none was minted to ask.
#[test]
fn a_held_forms_word_and_not_its_chain_cites_the_next_listing() {
use crate::transducer::sentence::Vertex;
let scene = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2)];
let listing = vec![(4i64, 4i64, 1u8, 2u8), (4, 5, 1, 2), (9, 9, 3u8, 4u8)];
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("held-word"));
entity.hold_chain(scene.clone());
let body_chart = crate::transducer::stalk::invariant(&listing[..2]);
let on_chart = entity.contemplation.held_at(&body_chart);
let [Vertex::Word { address: vertex, .. }] = on_chart.as_slice() else {
    panic!("one Word vertex on the recoloured body's chart, got {on_chart:?}");
};
let vertex = vertex.clone();
let (cites, residual) = entity.cite(&listing);
assert_eq!(cites.len(), 1, "one held word deflates the listing, got {cites:?}");
assert_eq!(cites[0].standing, Standing::Held.as_u8(), "with standing Held");
assert_eq!(cites[0].addr, vertex, "and it is the vertex's address: the word");
assert_ne!(cites[0].addr, chain_word(&listing).address(), "never the listing chain's key");
assert_eq!(cites[0].body, listing[..2].to_vec(), "it carries the body the vertex's action reconstructed, in the listing's frame");
assert_eq!(residual, vec![(9, 9, 3, 4)], "the residual is the body no word spoke");
assert!(
    entity.epigenome.working_word(chain_word(&listing).address().as_bytes()).is_none()
        && entity.epigenome.working_word(chain_word(&scene).address().as_bytes()).is_none()
        && entity.epigenome.working_word(vertex.as_bytes()).is_none(),
    "no element for a chain, and none for the vertex, exists on the working language"
);
}

/// A listing is expressed only when a held word deflates it.
#[test]
fn listing_is_expressed_only_when_a_held_word_deflates_it() {
let prefix = vec![(0i64, 0i64, 1u8, 2u8)];
let listing = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 3, 4)];

let mut bare = Entity::open("127.0.0.1:1", Kernel::named("listing-bare"));
let Concluded { cites, residual, .. } = bare.conclude_listing(&listing);
assert!(cites.is_empty(), "nothing deflates an unseen listing, got {cites:?}");
assert_eq!(residual, listing, "equal residual is the chain");
assert_eq!(
    bare.dialect_len(),
    0,
    "the unreduced listing is never expressed"
);
assert!(bare.ideas_len() >= 1, "it is held as thought");

let mut held = Entity::open("127.0.0.1:1", Kernel::named("listing-held"));
held.hold_chain(prefix);
let Concluded { cites, residual, .. } = held.conclude_listing(&listing);
assert!(
    cites.iter().any(|c| c.standing == Standing::Held.as_u8()),
    "a held word deflates the listing, got {cites:?}"
);
let r = crate::transducer::element::measure_lanes(&residual);
let c = crate::transducer::element::measure_lanes(&listing);
assert!(
    (0..4).any(|i| r[i] < c[i]) && (0..4).all(|i| r[i] <= c[i]),
    "residual strictly below the chain on a lane, residual {r:?} chain {c:?}"
);
assert!(
    held.dialect_len() >= 1,
    "the residual is expressed, dialect {}",
    held.dialect_len()
);
}

/// 50.6 seam 1 — A WITHHELD FORM STANDS, IS HELD AS AN IDEA, AND UTTERS
/// NOTHING: no remainder is expressed as a word and no chain is attested;
/// what the form could not close is its demand vertices, spoken with the
/// sentence (L5), or the idea's own work.
#[test]
fn withheld_form_stands_and_holds_the_idea_and_utters_nothing() {
use crate::transducer::family::{extract_rule, form_descent, NoCrystal};
let family = vec![
    (vec![vec![1u8]], vec![vec![2u8]]),
    (vec![vec![1u8, 0]], vec![vec![2u8, 3]]),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("rule");
let (mag, rems) = form_descent(&rule.form, &family, &NoCrystal);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("withhold"));
entity.withhold_form(&rule.form, &family, &rems, mag);
assert!(entity.dialect_len() >= 1, "the withheld form stands, dialect {}", entity.dialect_len());
assert!(entity.ideas_len() >= 1, "and is held as an idea");
assert_eq!(entity.uttered_n(), 0, "nothing is uttered for a remainder, taught_up {}", entity.uttered_n());
for r in rems.iter().filter(|r| !r.is_empty()) {
    let chain = chain_word(r);
    assert!(entity.epigenome.working_word(chain.address().as_bytes()).is_none(), "a remainder never stands as a word");
}
}
