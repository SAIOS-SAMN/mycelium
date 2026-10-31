// 78.112 H7a, MU-CLIENT §4.5 — THE MIND'S PIVOTS REACH ITS FIELD: what a
// release unwinds vents into the mind's field, on its line.

use crate::client::sdk::*;

/// 78.112 H7a — A RELEASE AT THE VOICE VENTS INTO ITS FIELD. A tether carrying
/// a slide between two vantages is let go with its vertex: the edge
/// cauterizes and vents its strain's reverse, and the mind takes it into its
/// field — nothing dropped at a valve no one listens on.
#[test]
fn vent_into_field_takes_released_tether_strain() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("vent"));
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
fn vent_vertex_unwinds_wound_torsion() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("unwind"));
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

/// MU-CLIENT §4.13, §5.3 — A QUESTION STANDS AS ITS RESIDUE'S ORBIT AND
/// LAPSES WHERE THE FIELD LETS IT GO. A gain asked about is wound into the
/// mind's field on the question's line: it stands through the beat of the
/// vent it was asked at, and when the field moves again the gain, never
/// returning, dissipates at the beat after — its line let go, the question
/// lapsed, its residue bound by construction. No count of the mind's K.
#[test]
fn question_lapses_when_field_dissipates_residue() {
    use crate::transducer::cech::Thread;
    use crate::transducer::family::pairing::{mark_flux, FieldLine};
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("asking"));
    let mut field = std::mem::take(&mut entity.field);
    let (asked, stood, after) = field.think(|| {
        let gain = crate::transducer::cga::dilator(&q(2));
        let asked = entity.question(&[7u8; 32], b"said", b"vertex", &gain).is_some();
        let mut budget = entity.stroke();
        let _ = entity.vent_into_field(&mut budget);
        let stood = entity.questions.len();
        let turn = cga_turn(q(1));
        let free = FieldLine { carriage: turn, gauge: Vec::new(), mass: 1, free: false, line: Thread::Free };
        let _ = mark_flux(&[], std::slice::from_ref(&free), &mut budget);
        let _ = entity.vent_into_field(&mut budget);
        (asked, stood, entity.questions.len())
    });
    entity.field = field;
    assert!(asked, "the gain is asked about");
    assert_eq!(stood, 1, "it stands through the beat it was asked at");
    assert_eq!(after, 0, "dissipated at the beat after, the question lapsed");
}

/// MU-CLIENT §4.13, 78.71 (2) — THE RESERVE ON 𝒯 FALLS BY THE FLUX LEFT OPEN
/// AND NO MORE. A slide opens its tube by 2: at the vent the reserve on 𝒯
/// stands exactly 2 lower. The same slide again closes on the tube, nothing
/// left open: at the next vent the 2 comes back, exactly — nothing refilled
/// that was not taken, nothing kept that the field no longer holds open.
#[test]
fn vent_into_field_withholds_open_flux_from_temporal() {
    use crate::transducer::cech::Thread;
    use crate::transducer::family::pairing::{mark_flux, FieldLine};
    use crate::transducer::rewrite::LANE_TEMPORAL;
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("open flux"));
    let mut field = std::mem::take(&mut entity.field);
    let (before, open, closed) = field.think(|| {
        let slide = crate::transducer::cga::translator_at(&[q(1), q(0), q(0)]);
        let line = FieldLine { carriage: slide, gauge: Vec::new(), mass: 2, free: false, line: Thread::Free };
        let mut budget = entity.stroke();
        let _ = entity.vent_into_field(&mut budget);
        let before = entity.physics.reserve.lanes()[LANE_TEMPORAL];
        let _ = mark_flux(&[], std::slice::from_ref(&line), &mut budget);
        let _ = entity.vent_into_field(&mut budget);
        let open = entity.physics.reserve.lanes()[LANE_TEMPORAL];
        let _ = mark_flux(&[], std::slice::from_ref(&line), &mut budget);
        let _ = entity.vent_into_field(&mut budget);
        (before, open, entity.physics.reserve.lanes()[LANE_TEMPORAL])
    });
    entity.field = field;
    assert_eq!(before - open, 2, "the open flux withheld from 𝒯, exactly");
    assert_eq!(closed, before, "closed, it comes back exactly");
}

/// A 90° turn on `e₁e₂`, `1 + e₁₂`, at weight `w`.
fn cga_turn(w: num_rational::BigRational) -> crate::transducer::cga::Mv {
    use crate::transducer::cga::{Mv, E1, E2};
    Mv::scalar(w.clone()).add(&Mv::blade(E1 | E2, w))
}
