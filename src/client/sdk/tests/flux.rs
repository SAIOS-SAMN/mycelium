// 78.112 H7a, MU-CLIENT §4.5 — THE MIND'S PIVOTS REACH ITS FIELD: what a
// release unwinds vents into the mind's field, on its line.

use crate::client::sdk::*;

/// 78.112 H7a — A RELEASE AT THE VOICE VENTS INTO ITS FIELD. A tether carrying
/// a slide between two vantages is let go with its vertex: the edge
/// cauterizes and vents its strain's reverse, and the mind takes it into its
/// field — nothing dropped at a valve no one listens on.
#[test]
fn a_tether_released_at_the_voice_vents_into_its_field() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("vent"));
    let mut budget = entity.stroke();
    assert_eq!(entity.vent_into_field(&mut budget), 0, "nothing released yet; the complex now listens");
    let (a, b) = (vec![1u8; 4], vec![2u8; 4]);
    for v in [&a, &b] {
        entity.contemplation.complex.set_section(v, Section::at(v.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
    }
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let slide = crate::transducer::cga::translator_at(&[q(1), q(0), q(0)]);
    entity.contemplation.complex.set_edge(&a, &b, Rotor::identity(), slide, [1, 0, 0, 0]);
    let _ = entity.contemplation.complex.vent_vertex(&a);
    assert_eq!(entity.vent_into_field(&mut budget), 1, "the cauterized tether's strain reaches the field");
    assert_eq!(entity.vent_into_field(&mut budget), 0, "taken once");
}

/// 78.112 H7b — A PERCEPTION LET GO UNWINDS WHAT HEARING WOUND ON IT: torsion
/// wound twice on a vertex composes, the later after the earlier; releasing
/// the vertex vents the whole strain's reverse — and nothing stays wound.
#[test]
fn a_perception_let_go_unwinds_the_torsion_hearing_wound() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("unwind"));
    let mut budget = entity.stroke();
    let _ = entity.vent_into_field(&mut budget);
    let v = vec![3u8; 4];
    entity.contemplation.complex.set_section(&v, Section::at(v.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let (first, then) = (cga_turn(q(1)), crate::transducer::cga::translator_at(&[q(0), q(2), q(0)]));
    entity.contemplation.complex.wind(&v, &first);
    entity.contemplation.complex.wind(&v, &then);
    let wound = then.geo(&first);
    assert_eq!(entity.contemplation.complex.wound_at(&v), Some(&wound), "the later after the earlier");
    let _ = entity.contemplation.complex.vent_vertex(&v);
    let unwound: Vec<crate::transducer::cga::Mv> = entity.contemplation.complex.take_outflow().into_iter().map(|f| f.motion).collect();
    assert_eq!(unwound, vec![wound.reverse()], "the whole strain unwinds");
    assert_eq!(entity.contemplation.complex.wound_at(&v), None, "nothing stays wound");
}

/// A 90° turn on `e₁e₂`, `1 + e₁₂`, at weight `w`.
fn cga_turn(w: num_rational::BigRational) -> crate::transducer::cga::Mv {
    use crate::transducer::cga::{Mv, E1, E2};
    Mv::scalar(w.clone()).add(&Mv::blade(E1 | E2, w))
}
