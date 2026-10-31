use crate::transducer::element::Element;
use crate::transducer::sheaf::*;
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;

fn word(painted_a: &[(i64, i64, u8)], painted_b: &[(i64, i64, u8)]) -> Element {
    from_canonical(&germinate(painted_a, painted_b).unwrap())
}

/// A ROOT IS NOT ITS POPULATION. This returned the stratum's sorted
/// addresses concatenated, and `digests` returns one per stratum, so
/// every exchange cycle shipped the whole ruled set to every peer —
/// O(P²·C) on the wire before a word was asked for. Thirty-two bytes
/// whatever the stratum holds, over the SET and not the order, and it
/// separates: one member added moves it, which is what discordance
/// reads.
#[test]
fn stratum_root_is_fixed_width_over_set() {
    let small: Vec<Vec<u8>> = (0..2u16).map(|i| i.to_le_bytes().to_vec()).collect();
    let large: Vec<Vec<u8>> = (0..2000u16).map(|i| i.to_le_bytes().to_vec()).collect();
    assert_eq!(stratum_root(&small).len(), 32, "a root is thirty-two bytes");
    assert_eq!(
        stratum_root(&large).len(),
        32,
        "and a stratum a thousand times wider is the same thirty-two"
    );
    let mut reversed = large.clone();
    reversed.reverse();
    assert_eq!(
        stratum_root(&large),
        stratum_root(&reversed),
        "the root is over the set; arrival order is not part of it"
    );
    assert_ne!(stratum_root(&small), stratum_root(&large), "and it separates strata");
    let mut plus = small.clone();
    plus.push(vec![9u8; 4]);
    assert_ne!(
        stratum_root(&small),
        stratum_root(&plus),
        "one member added moves the root — this is what discordance reads"
    );
}

/// Two holders differing by one word in one stratum disagree on exactly
/// that stratum; the fetch carries only the lacking word; after learning
/// it the digests agree everywhere.
#[test]
fn discordant_names_stratum_and_serve_closes_it() {
    let bar = word(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]);
    let bar_hue = word(&[(0, 0, 3), (0, 1, 3)], &[(0, 0, 4), (0, 1, 4)]);
    let square = word(
        &[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)],
        &[(0, 0, 5), (0, 1, 5), (1, 0, 5), (1, 1, 5)],
    );
    let hypha = vec![bar.clone(), bar_hue.clone(), square.clone()];
    let mut entity = vec![bar.clone(), square.clone()];
    let (hs, es) = (atlas(&hypha), atlas(&entity));
    let (hd, ed) = (digests(&hs), digests(&es));
    let d = discordant(&hd, &ed);
    assert_eq!(d.len(), 1, "one stratum disagrees");
    assert_eq!(d[0].inv, chart(&bar), "the bar's chart");
    assert_eq!(d[0].count, 2);
    let entity_has = have(&es, &d[0].inv);
    assert_eq!(entity_has, vec![bar.address()]);
    let mut at_index: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    for (i, w) in hypha.iter().enumerate() {
        at_index.insert(w.address(), i);
    }
    let at = |a: &[u8]| at_index.get(a).copied().and_then(|i| hypha.get(i));
    let page = serve(&hs, at, &d[0].inv, &entity_has, 1 << 20);
    assert_eq!(page.len(), 1, "only the lacking word travels");
    let mut pos = 0usize;
    let learned = Element::from_bytes(&page[0], &mut pos).unwrap();
    assert_eq!(learned.address(), bar_hue.address());
    entity.push(learned);
    let es2 = atlas(&entity);
    assert!(discordant(&hd, &digests(&es2)).is_empty(), "glued everywhere");
    // A grown fibre does not change the root: the base is what glues;
    // a section attached on one side travels by teaching, never as
    // discordance.
    let mut grown = hypha.clone();
    let g = crate::transducer::element::transition(bar_hue.sub(), grown[0].sub(), &mut crate::physics::Physics::default().stroke_n(1 << 20)).unwrap();
    grown[0].attach(&bar_hue, &g);
    let gd = digests(&atlas(&grown));
    assert!(discordant(&gd, &digests(&es2)).is_empty(), "the root is over the base");
    // A budget too small for one word serves nothing and never overflows.
    assert!(serve(&hs, at, &d[0].inv, &entity_has, 8).is_empty());
}
