// 16, A6 — the moduli and the transport are exact and similarity-invariant,
// and an overflow is refused rather than saturated.

use super::common::*;
use crate::transducer::blade::*;

/// The shell: the moduli agree across translation, rotation, reflection,
/// and integer dilation; the transport between two similar bodies is
/// solved from their moments and verified on their supports, while
/// dissimilar bodies refuse.
#[test]
fn moduli_and_transport_are_exact_and_similarity_invariant() {
    let bar = shape_of(&rect(0, 0, 2, 3));
    assert_eq!(turns(&bar.blade.cells), 4);
    let l: Vec<(i64, i64)> = vec![(0, 0), (1, 0), (2, 0), (2, 1)];
    let l_shape = shape_of(&l);
    assert_eq!(turns(&l_shape.blade.cells), 6);
    // Rotated (rows↔cols) and translated bar: same moduli, a rigid versor.
    let bar_rot = shape_of(&rect(5, 7, 3, 2));
    assert_eq!(moduli(&bar.blade), moduli(&bar_rot.blade));
    let v = versor_between(&bar, &bar_rot).expect("a rotation+translation");
    assert!(cga::is_versor(&v).is_some());
    // Dilated bar (2×): same moduli, a versor with ρ = 2 carrying the
    // body's vertex (0,0) to (0,0) and (2,3) to (4,6).
    let bar2 = shape_of(&rect(0, 0, 4, 6));
    assert_eq!(moduli(&bar.blade), moduli(&bar2.blade));
    let v2 = versor_between(&bar, &bar2).expect("a dilation");
    assert_eq!(carry(&v2, 0, 0), Some((q(0), q(0))));
    assert_eq!(carry(&v2, 2, 3), Some((q(4), q(6))));
    // The mirrored L needs the reflection; a rotated L is a rotation; a
    // bar and an L are not similar.
    let l_mirror: Vec<(i64, i64)> = vec![(0, 1), (1, 1), (2, 1), (2, 0)];
    assert!(versor_between(&l_shape, &shape_of(&l_mirror)).is_some(), "reflection found");
    let l_rot: Vec<(i64, i64)> = vec![(7, -3), (7, -2), (7, -1), (6, -1)];
    assert!(versor_between(&l_shape, &shape_of(&l_rot)).is_some(), "rotation found");
    assert_eq!(moduli(&l_shape.blade), moduli(&shape_of(&l_rot).blade));
    assert!(versor_between(&bar, &l_shape).is_none(), "dissimilar bodies refuse");
    // A 3×3 against a 2×2: ρ² = 9/4, rational; a 3×3 against a 2×3 refuses.
    assert!(versor_between(&shape_of(&rect(0, 0, 2, 2)), &shape_of(&rect(1, 1, 3, 3))).is_some());
    assert!(versor_between(&shape_of(&rect(0, 0, 3, 3)), &shape_of(&rect(0, 0, 2, 3))).is_none());
    // An isotropic body (a square) is oriented by its stabilizer: a
    // translated, dilated square carries onto itself.
    let sq = shape_of(&rect(0, 0, 2, 2));
    let sq3 = shape_of(&rect(4, 4, 6, 6));
    assert_eq!(moduli(&sq.blade), moduli(&sq3.blade));
    assert!(versor_between(&sq, &sq3).is_some(), "ρ = 3 on an isotropic body");
    // An isotropic body with an orientation (a pinwheel of four arms)
    // is oriented by the fourth harmonic: its quarter-turn image carries.
    let pin: Vec<(i64, i64)> = vec![(1, 1), (1, 2), (2, 2), (0, 1), (1, 0), (2, 1), (1, 3), (3, 2), (0, 3)];
    let pin_shape = shape_of(&pin);
    let pin_rot: Vec<(i64, i64)> = pin.iter().map(|&(r, c)| (c, -r)).collect();
    assert_eq!(moduli(&pin_shape.blade), moduli(&shape_of(&pin_rot).blade));
    assert!(versor_between(&pin_shape, &shape_of(&pin_rot)).is_some(), "a quarter turn found");
}

/// 24.4, A21 §2, Stage 12 S12.1a — A CHART'S PREFIX IS THE TOPOLOGICAL
/// STRATUM. Two rectangles of different proportions have the same
/// components, holes and turns and different moment ratios: their charts
/// differ, share the three coarsest axes, and the atlas read at that depth
/// holds both — the stratum a peel walks out to. On the fleet 952 of 954
/// charts had thirteen axes and no two shared one; the shared stratum is
/// three axes in, and it is read by peel, never by the whole chart.
#[test]
fn a_chart_prefix_is_the_topological_stratum() {
    use crate::transducer::element::from_residual;
    use crate::transducer::stalk::{Invariant, Stalks};
    let bar_cells = rect(0, 0, 2, 3);
    let plank_cells = rect(0, 0, 2, 5);
    let (a, b) = (moduli(&shape_of(&bar_cells).blade), moduli(&shape_of(&plank_cells).blade));
    assert_ne!(a, b, "different proportions, different charts");
    assert_eq!(a[..3], b[..3], "one topology: components, holes and turns lead the chart");
    assert_eq!(a[..3], [1u64, 0, 4], "one component, no hole, four turns");
    let word_of = |cells: &[(i64, i64)]| {
        from_residual(&cells.iter().map(|&(r, c)| (r, c, 1u8, 2u8)).collect::<Vec<_>>()).address()
    };
    let (wa, wb) = (word_of(&bar_cells), word_of(&plank_cells));
    let (ia, ib) = (Invariant(a.clone()), Invariant(b.clone()));
    let mut atlas = Stalks::default();
    atlas.admit(&ia, wa.clone());
    atlas.admit(&ib, wb.clone());
    assert_eq!(atlas.fiber(&ia).len(), 1, "the exact chart holds one word");
    // The stratum is the fibre UNDER the prefix (A21 §2): `fiber` reads one
    // node's own members and holds nothing at a prefix no word sits on.
    let stratum = atlas.fiber_under(&Invariant(a[..3].to_vec()));
    assert!(
        stratum.contains(&wa.clone().into_vec()) && stratum.contains(&wb.clone().into_vec()),
        "the topological stratum holds both: {} members",
        stratum.len()
    );
    let mut at = ia.clone();
    while at.0.len() > 3 {
        at = at.peel();
    }
    assert_eq!(at.0, a[..3], "peel walks out to the topological stratum");
    assert_eq!(atlas.fiber_under(&at).len(), 2, "and the stratum's fibre is read under that prefix");
}

#[test]
fn a6_refuses_overflow_keeps_sign() {
    let neg = BigRational::new(BigInt::from(-3), BigInt::from(7));
    let p = axes_of(&neg).expect("fits");
    assert_eq!(p[0], (-3i64) as u64, "sign is two's complement, not dropped");
    assert_eq!(p[1], 7);
    let huge = BigRational::new(BigInt::from(1) << 80, BigInt::from(1));
    assert!(axes_of(&huge).is_none(), "overflow refuses, never saturates to MAX");
}
