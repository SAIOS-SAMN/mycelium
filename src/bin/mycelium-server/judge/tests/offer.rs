use super::super::offer::*;
use mycelium::anastome::dialogue;

fn word() -> mycelium::transducer::element::Element {
    mycelium::transducer::element::from_residue(&vec![mycelium::transducer::sentence::SectionLink::on_plane([0, 0], 1, 2)])
        .expect("one link is A5's chain")
}

/// A word whose magnitude does not re-derive from its chain: the reconstruction
/// refuses it by name, every time it is asked.
fn false_word() -> mycelium::transducer::element::Element {
    let mut w = word();
    assert!(w.set_magnitude([9, 9, 9, 9]), "a residue holds no section, so any lane fits");
    w
}

/// The vantage is the form the witness reconstructed, reaching the offered
/// address — asked once and asked again.
#[test]
fn witness_vantage_reaches_offered_form_each_ask() {
    let court = std::sync::Mutex::new(mycelium::cognition::Court::new());
    let mut cover = mycelium::network::nerve::HomeTrie::new();
    let (home, witness) = ([2u8; 32], [3u8; 32]);
    let w = word();
    for _ in 0..2 {
        // Each offer is its own act: its root, one stroke — its hold's.
        let mut h = crate::court_lock::held!(court).expect("the court");
        let dialogue::Witnessed::Reached { vk, form } =
            h.run(|k, budget| witness_vantage(k, &mut cover, w.clone(), home, witness, budget))
        else {
            panic!("a reconstruction that stood is a vantage");
        };
        assert_eq!(vk, witness);
        let mut p = 0usize;
        let back = mycelium::transducer::element::Element::from_bytes(&form, &mut p)
            .expect("the reply carries the form");
        assert!(back.same_vertex(&w), "the witness reached the offered vertex");
        assert_eq!(back.address(), w.address());
    }
}

/// A form the reconstruction refuses is refused by name — and refused AGAIN when
/// it is offered again, though this court has marked it heard: the
/// heard mark is not a verdict, the reconstruction is.
#[test]
fn witness_vantage_refuses_false_form_each_ask() {
    let court = std::sync::Mutex::new(mycelium::cognition::Court::new());
    let mut cover = mycelium::network::nerve::HomeTrie::new();
    let (home, witness) = ([2u8; 32], [3u8; 32]);
    let w = false_word();
    let first = crate::court_lock::held!(court).expect("the court").run(|k, budget| witness_vantage(k, &mut cover, w.clone(), home, witness, budget));
    let dialogue::Witnessed::Refused { vk, why } = first else {
        panic!("a reconstruction that refused is a refusal");
    };
    assert_eq!(vk, witness);
    assert!(!why.is_empty(), "the refusal is named");
    let heard = crate::court_lock::held!(court).expect("the court").run(|k, _| k.heard_contains(w.address().as_bytes()));
    assert!(heard, "and the address is marked heard");
    // The second offer is a second act: a stroke of its own.
    let again = crate::court_lock::held!(court).expect("the court").run(|k, budget| witness_vantage(k, &mut cover, w, home, witness, budget));
    assert!(
        matches!(again, dialogue::Witnessed::Refused { .. }),
        "heard is not a verdict: the second offer is reconstructed and refused again"
    );
}
