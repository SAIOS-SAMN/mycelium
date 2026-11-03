// MU-SENSING §3.6 — an axis taken apart into two nested phases, exactly.

use crate::transducer::family::transduce::*;

/// THE NESTED MAP IS EXACT: on a phase of period 12, each of the four
/// factorings `(2, 6)`, `(3, 4)`, `(4, 3)`, `(6, 2)` carries site `s` to
/// `(⌊s / b⌋ / a, (s mod b) / b)`, one pair per site and every pair once;
/// the pair reads back as the site's own turn `s / 12`, whose value is
/// `coarse + fine / a`; a site past the extent has no image. The four are
/// held together, a prime extent holds none, and a product that is not
/// the extent or a factor of 1 is refused by name.
#[test]
fn factoring_maps_site_exactly() {
    let held = Factorings::of(12);
    let pairs: Vec<(usize, usize)> = held.candidates().iter().map(|f| (f.coarse(), f.fine())).collect();
    assert_eq!(pairs, vec![(2, 6), (3, 4), (4, 3), (6, 2)]);
    for f in held.candidates() {
        let (a, b) = (f.coarse(), f.fine());
        let mut seen = Vec::new();
        for s in 0..12 {
            let (coarse, fine) = f.nested(s).expect("a site of the phase");
            assert_eq!((coarse, fine), (Turn::new(s / b, a), Turn::new(s % b, b)));
            assert!(!seen.contains(&(coarse, fine)), "one pair per site");
            seen.push((coarse, fine));
            assert_eq!(f.joined(coarse, fine), Some(Turn::new(s, 12)), "read back exactly");
            let a_q = crate::core::Q::from_integer(a.into());
            assert_eq!(Turn::new(s, 12).value(), coarse.value() + fine.value() / a_q);
        }
        assert_eq!(seen.len(), a * b);
        assert_eq!(f.nested(12), None, "no site past the extent");
        assert_eq!(f.joined(Turn::new(a, a), Turn::new(0, b)), None, "the seam's far side is no site");
    }
    assert!(Factorings::of(7).candidates().is_empty(), "a prime extent stays one phase");
    assert!(Factorings::of(1).candidates().is_empty());
    assert_eq!(Factoring::new(12, 5, 2), Err(TransductionRefusal::Unfactored { extent: 12, coarse: 5, fine: 2 }));
    assert_eq!(Factoring::new(12, 1, 12), Err(TransductionRefusal::Unfactored { extent: 12, coarse: 1, fine: 12 }));
}
