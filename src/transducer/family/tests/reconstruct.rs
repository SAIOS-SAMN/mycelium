// 78.82 — RECONSTRUCTION BY HOLONOMY (`family/reconstruct.rs`, HR1–HR4): a
// site read by its coordinates on every axis, its transport in the pair's
// output, the loops across pairs closed on the axis that carries the rule,
// and each pair rebuilt from the others by replay.

use super::common::grid;
use crate::transducer::family::reconstruct::{field, holonomy, reconstruct, transports};
use num_rational::BigRational;

/// HR1 — CENTRALITY IS THE SPHERE'S OWN ALGEBRA. On a bar of three, the site's
/// weight against the body's sphere over the sphere's square, `M·(P·C)/C²`:
/// ½ at the barycentre and 5/4 at either end — read off the sums, never a root.
#[test]
fn field_reads_centrality_against_body_sphere() {
    let f = field(&grid(&["111"]));
    let half = BigRational::new(1.into(), 2.into());
    let end = BigRational::new(5.into(), 4.into());
    assert_eq!(f[1].centrality, Some(half), "the barycentre");
    assert_eq!((f[0].centrality.clone(), f[2].centrality.clone()), (Some(end.clone()), Some(end)));
    assert_eq!((f[0].body_mass, f[0].local.mass, f[1].local.mass), (3, 2, 3));
}

/// HR3, HR4 — THE AXIS THAT CARRIES THE RULE. Each pair holds a stroke
/// leaning one way and a stroke leaning the other, both of 5; the first turns
/// 2 and the second 8. On the hue axis one key (5) is sent two ways, so its
/// loops do not close; by mass, density, centrality and body the two strokes
/// are alike and do not close either. The local inertia form's cross term
/// has opposite signs on the two strokes: LOCAL DIRECTION closes, and each
/// pair left out is rebuilt from the other two by it.
#[test]
fn reconstruct_rebuilds_each_pair_on_closing_axis() {
    let family = vec![
        (
            grid(&["500000050", "050000500", "005005000", "000000000", "000000000"]),
            grid(&["200000080", "020000800", "002008000", "000000000", "000000000"]),
        ),
        (
            grid(&["000000000", "500000050", "050000500", "005005000", "000000000"]),
            grid(&["000000000", "200000080", "020000800", "002008000", "000000000"]),
        ),
        (
            grid(&["000000000", "000000000", "050000005", "005000050", "000500500"]),
            grid(&["000000000", "000000000", "020000008", "002000080", "000200800"]),
        ),
    ];
    let fields: Vec<_> = family.iter().map(|(x, y)| (field(x), transports(x, y).expect("one frame"))).collect();
    let pairs: Vec<_> = fields.iter().map(|(s, t)| (s.as_slice(), t.as_slice())).collect();
    let h = holonomy(&pairs);
    assert!(!h[0].closes(), "the hue axis sends 5 two ways");
    assert!(!h[1].closes() && !h[3].closes() && !h[7].closes(), "mass, density and centrality see the strokes alike");
    assert!(h[2].closes(), "local direction closes every loop");
    let rec = reconstruct(&family);
    assert_eq!((rec.framed, rec.reproduced), (3, 3));
    assert_eq!(rec.by_axis[2], 3, "every pair carried by local direction, the most compact axis that closes");
    assert!(rec.whole);
}
