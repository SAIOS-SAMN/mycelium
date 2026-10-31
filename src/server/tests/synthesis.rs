use crate::server::synthesis::*;
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;


/// Adoption is a deterministic status map: a clean foreign word
/// installs with the ratifier as provenance; re-adoption reads Known;
/// a word the language already derives reads Derivable; tampered
/// speech is Refused with the failed axis named.
#[test]
fn adopt_maps_every_state_deterministically() {
    use crate::cognition::{Adoption, Court};
    let k = std::sync::Mutex::new(Court::new());
    let court = [0xAB; 32];
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
    let word = from_canonical(&germinate(&a, &rot).unwrap());
    assert_eq!(adopt(&k, court, &word), Adoption::Installed, "clean foreign word installs");
    assert_eq!(adopt(&k, court, &word), Adoption::Known, "the genome had converged");
    // The identical deformation germinated far away: same substance —
    // the language already derives it.
    let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
    let dup = from_canonical(&germinate(&a_far, &rot_far).unwrap());
    assert!(
        matches!(adopt(&k, court, &dup), Adoption::Derivable | Adoption::Known),
        "adoption never adds redundancy"
    );
    // Tampered speech is named and refused.
    let mut fat = word.clone();
    fat.mag = [999, 0, 0, 0];
    assert!(matches!(adopt(&k, court, &fat), Adoption::Refused(_)));
    // Provenance is the ratifying voice, not a sentinel.
    let g = k.lock().unwrap();
    assert!(g.word_at(&word.address()).is_some());
}
