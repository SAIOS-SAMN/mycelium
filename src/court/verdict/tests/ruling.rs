// 42, A9 — the ruling maps the federation, and the selection key is stable
// under a change of debug representation.

use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
use crate::verdict::*;
use super::common::*;

#[test]
fn a9_sel_key_stable_under_debug_change() {
    let s = Sel::Hue(3);
    let mut buf = Vec::new();
    s.to_bytes(&mut buf);
    let k = sel_key(&s);
    assert_eq!(k.as_bytes()[0], crate::transducer::address::TAG_SEL);
    assert_eq!(&k.as_bytes()[1..], buf.as_slice(), "the key is Sel::to_bytes, never a digest");
    assert_ne!(k.as_bytes(), format!("{s:?}").as_bytes());
}

/// The two-docket ruling over a federation is an exhaustive state map:
/// new geometry admits (the element rides the arm), same substance is
/// consilient, a same-geometry sibling demotes.
#[test]
fn the_ruling_maps_the_federation_correctly() {
    let w = clean_word();
    let addr = w.address();
    // Empty federation: new geometry admits — the sealed element is
    // the arm's payload, not a flag.
    match rule(&[], &Keyed::new(), &Stalks::default(), addr.as_bytes(), Some(&w), None) {
        Ruling::Admitted(sealed) => assert_eq!(sealed.address(), addr),
        other => panic!("new geometry must admit, got {other:?}"),
    }
    // Same address, same substance: consilience (no re-admission).
    let fed = vec![(addr.clone(), w.clone())];
    let stalks = stalks_of(&fed);
    assert!(matches!(rule(&fed, &index_of(&fed), &stalks, addr.as_bytes(), Some(&w), None), Ruling::Consilient(_)));
    assert_eq!(rule(&fed, &index_of(&fed), &stalks, addr.as_bytes(), Some(&w), None).code(), dialogue::RULING_UPHELD);
    // Different address, identical base geometry: attached (+axis) by
    // the identity transport.
    let mut other_bytes = addr.as_bytes().to_vec();
    other_bytes[0] ^= 0xFF;
    let other = crate::transducer::address::Address::new(other_bytes);
    match rule(&fed, &index_of(&fed), &stalks, other.as_bytes(), Some(&w), None) {
        Ruling::Attached { base: 0, g } => {
            assert!(element::same_versor(&g, &element::identity_versor()), "the identity transport")
        }
        other => panic!("identical geometry attaches, got {other:?}"),
    }
    // A rotated sibling (the same geometry as a D4 image, other hues)
    // resolves through the chart to the same base with a rotating
    // transport; a genuinely different geometry finds an empty stratum
    // and admits.
    let rotated = from_residual(
        &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
    );
    match rule(&fed, &index_of(&fed), &stalks,&rotated.address(), Some(&rotated), None) {
        Ruling::Attached { base: 0, g } => {
            assert!(!element::same_versor(&g, &element::identity_versor()), "the transport rotates")
        }
        other => panic!("a rotated placement attaches, got {other:?}"),
    }
    let bent = from_residual(
        &germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 2u8), (0, 1, 2), (1, 0, 2)])
            .unwrap(),
    );
    assert!(matches!(rule(&fed, &index_of(&fed), &stalks,&bent.address(), Some(&bent), None), Ruling::Admitted(_)));
}
