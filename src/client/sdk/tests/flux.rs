// 78.37(3), FX1 — THE PRODUCER PATH: a sealed act credits the filament its
// form was spoken through, with the frame its receipt carries.

use crate::client::sdk::*;

/// Two held thoughts share a chart and a form is spoken through their edge;
/// the entity seals it Uttered, and the edge reads exactly the frame the
/// Uttered receipt carries — once. A second act on a word spoken from no held
/// thought credits nothing and is unfiled.
#[test]
fn a_sealed_uttered_act_credits_the_edge_its_form_was_spoken_through() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("flux"));
    let chart = b"c".to_vec();
    let (a, b) = (vec![1u8; 4], vec![2u8; 4]);
    for v in [&a, &b] {
        let mut cover = crate::network::nerve::Keyed::new();
        cover.insert(&chart, ());
        let bubble = crate::transducer::bubble::Bubble { cover, ..Default::default() };
        let c = &mut entity.contemplation;
        c.admit_incidence(&std::sync::Arc::from(v.as_slice()), &bubble);
        c.bubbles.insert(v, bubble);
        c.complex.set_section(v, Section::at(v.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
    }
    entity.contemplation.complex.set_edge(&a, &b, Rotor::identity(), crate::transducer::element::identity_versor(), [1, 0, 0, 0]);
    let e = super::common::two_body_sentence([&[(1, 2)], &[(3, 4)]]);
    let circuits = crate::transducer::bubble::flux::Circuits { charts: vec![vec![chart]], words: Vec::new(), unnamed: 0 };
    entity.contemplation.file_spoken(e.address().as_bytes(), circuits);
    entity.seal_word(crate::chronometry::ActKind::Uttered, &e);
    let receipt = entity.session.voice().chain().head().expect("the Uttered act sealed").clone();
    assert_eq!(receipt.kind, crate::chronometry::ActKind::Uttered);
    let edge =
        crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts(vec![a.as_slice(), b.as_slice()]));
    assert_eq!(entity.contemplation.flux.get(edge.as_bytes()), Some(&receipt.mu), "the edge reads the receipt's frame");
    assert!(receipt.mu.iter().any(|&m| m > 0), "a frame that flowed");
    let other = super::common::two_body_sentence([&[(5, 6)], &[(7, 8)]]);
    entity.seal_word(crate::chronometry::ActKind::Uttered, &other);
    let f = entity.contemplation.meet_graph().flux;
    assert_eq!((f.credited, f.unfiled), (1, 1), "{f:?}");
    assert_eq!(entity.contemplation.flux.get(edge.as_bytes()), Some(&receipt.mu), "credited once");
}
