use crate::transducer::germinate::*;
use crate::transducer::sentence::tests::fixtures::{link, painted};
use crate::transducer::sentence::SectionLink;
use crate::transducer::spelled::PaintedSite;

fn norm(mut v: Vec<PaintedSite>) -> Vec<PaintedSite> {
    v.sort_unstable();
    v
}

/// A non-conformal change (a single site recoloured, others untouched — not a
/// rigid motion of the body) germinates A5's chain, which reproduces A→B
/// when it is placed back at the corner it was centred from.
#[test]
fn germinate_chain_reproduces_demand_when_placed() {
    let a = painted(&[(0, 0, 1), (0, 1, 2), (1, 0, 3)]);
    let b = painted(&[(0, 0, 1), (0, 1, 9), (1, 0, 3)]); // only (0,1): 2→9
    let g = germinate(&a, &b).expect("a change germinates");
    assert_eq!(
        g.as_slice(),
        &[link(0, 0, 2, 9)],
        "A5 centres the chain: the same change anywhere is one substance"
    );
    let placed: Vec<SectionLink> =
        g.iter().map(|l| SectionLink::new(l.site.moved([0, 1]).expect("in range"), l.before, l.after)).collect();
    assert_eq!(norm(apply(&placed, &a)), norm(b.to_vec()), "it closes A→B at its corner");
}

/// Creation/annihilation is carried too (hue 0 on the absent side).
#[test]
fn germinate_carries_creation_as_residue() {
    crate::transducer::raster::stand_on(0);
    let a = painted(&[(0, 0, 1)]);
    let b = painted(&[(0, 0, 1), (0, 1, 5)]); // a site is painted
    let g = germinate(&a, &b).expect("appearance germinates");
    assert_eq!(g.as_slice(), &[link(0, 0, 0, 5)], "centred, as A5 leaves every chain");
    let placed: Vec<SectionLink> =
        g.iter().map(|l| SectionLink::new(l.site.moved([0, 1]).expect("in range"), l.before, l.after)).collect();
    assert_eq!(norm(apply(&placed, &a)), norm(b.to_vec()));
}

/// No net change → no germ owed.
#[test]
fn germinate_refuses_unchanged_grids() {
    let a = painted(&[(0, 0, 1), (0, 1, 2)]);
    assert!(germinate(&a, &a).is_none());
}
