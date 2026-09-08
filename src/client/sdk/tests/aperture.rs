// 24.1, 24.2 — the aperture and the shed: what a mind holds, what it lets
// go, and that it sheds on its own K though the mesh never advances.

use super::common::*;
use crate::client::sdk::*;

/// Shed never removes a dialect entry a live idea glued, and never the genome.
#[test]
fn shed_spares_glued_dialect_and_the_genome() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let e2 = two_body_sentence([&[(1, 3), (1, 3)], &[(3, 5), (3, 5)]]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("shed"));
entity.epigenome.stand_dialect_at(e1.clone(), 0, 0);
entity.epigenome.stand_dialect_at(e2.clone(), 0, 0);
assert_eq!(entity.dialect_len(), 2);
let addr = e1.address();
entity.hold_thought(vec![(9, 0, 1, 2)]);
let idea = entity
    .contemplation
    .bubbles
    .fiber(crate::network::nerve::ROOT)
    .next()
    .map(|(k, _)| k)
    .expect("idea");
entity.contemplation.mark_glue(&idea, addr.clone());
let aperture = crate::physics::aperture_cycle() as u64;
// The first aperture after boot is unmeasured: nothing sheds while
// the stroke ring fills, however idle an entry is.
for tu in 1..aperture {
    entity.shed_at(tu);
    assert_eq!(entity.dialect_len(), 2, "an unmeasured aperture sheds nothing");
}
// The ring is full at this stroke: the aperture is measured, the
// idle unglued entry sheds, the glued one stands.
entity.shed_at(aperture + 1);
assert_eq!(entity.dialect_len(), 1, "the idle unglued entry sheds; the glued one stands");
assert!(entity.epigenome.working_word(&addr).is_some(), "the glued entry stands");
assert!(entity.epigenome.working_word(&e2.address()).is_none(), "the idle entry is gone");
assert_eq!(entity.epigenome.genome_len(), 0, "genome is untouched by shed");
}

/// An expressed word stands at the voice's K and TUniversal, so a shed
/// inside the aperture does not drop the mind's own speech.
#[test]
fn expressed_word_survives_shed_within_the_aperture() {
use crate::transducer::element::from_residual;
let mut epi = Epigenome::new();
let residual = [(0i64, 0i64, 1u8, 2u8)];
assert!(epi.express(&residual, 7, 10), "the residual is claimed");
let addr = from_residual(&residual.to_vec()).address();
let d = epi.dialect.get(&addr).expect("stood");
assert_eq!(d.last_used_k, 7, "k is the voice's K");
assert_eq!(d.last_used_tu, 10, "tu is the standing extent in heights");
// The lifecycle is counted in the mind's OWN K (45.4), so the
// strokes that measure the aperture are K values, not heights.
let aperture = crate::physics::aperture_cycle() as u64;
let now = 7 + aperture - 1;
for k in 7..=now {
    epi.note_stroke(k);
}
let n = epi.shed(now);
assert_eq!(n, 0, "expressed within the aperture stands");
assert_eq!(epi.dialect_len(), 1);
assert!(epi.working_word(&addr).is_some());
}

/// The defect this coordinate change closes. Heights advance only on
/// admitted receipts (45.2), so a mind whose mesh admits nothing has a
/// TUniversal frozen at 0 — and a lifecycle read against it could never
/// fire, holding everything the mind ever stood for the life of the
/// process. A mind's own K advances on its own acts, so it reclaims
/// what it holds whether or not anyone answered it.
#[test]
fn a_mind_sheds_on_its_own_k_though_the_mesh_never_advances() {
use crate::transducer::element::from_residual;
let mut epi = Epigenome::new();
let residual = [(0i64, 0i64, 1u8, 2u8)];
// Stood at the mind's K 1, with TUniversal frozen at 0 throughout:
// no receipt was ever admitted.
assert!(epi.express(&residual, 1, 0), "the residual is claimed");
let addr = from_residual(&residual.to_vec()).address();
assert_eq!(epi.dialect_len(), 1);
let aperture = crate::physics::aperture_cycle() as u64;
let now = 1 + aperture + 1;
for k in 1..=now {
    epi.note_stroke(k);
}
assert_eq!(epi.shed(now), 1, "unused past the aperture, it sheds");
assert_eq!(epi.dialect_len(), 0, "the mind reclaimed what it held");
assert!(epi.working_word(&addr).is_none());
}

/// 45.4 — THE MIND'S K IS ITS OWN ACTS, and no reply moves it. The test
/// above drives the epigenome with K values handed to it, so it proves the
/// shed's arithmetic and says nothing about where K comes from. This one
/// speaks to a hypha that is not there: the wire never answers, and the
/// clock still steps, once per form stood.
#[test]
fn standing_a_form_steps_the_minds_own_k() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("clock"));
assert_eq!(entity.session.voice().k(), 0, "a mind that has acted seats at zero");

// The wire refuses (nothing is listening), so `speak` returns Err — but
// the form STOOD before anything was uttered, and standing is the act.
let _ = entity.speak(std::slice::from_ref(&e1));
assert_eq!(entity.dialect_len(), 1, "the form stands in the dialect");
assert_eq!(entity.session.voice().k(), 1, "standing it stepped the clock once");

// Standing the same form again is a second act of this mind: it is
// already taught, so nothing goes on the wire at all, and the clock still
// steps. Nothing here read a k-step from anyone else.
let _ = entity.speak(std::slice::from_ref(&e1));
assert_eq!(entity.session.voice().k(), 2, "a second act, a second step");
}
