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

#[test]
fn a6_refuses_overflow_keeps_sign() {
    let neg = BigRational::new(BigInt::from(-3), BigInt::from(7));
    let p = axes_of(&neg).expect("fits");
    assert_eq!(p[0], (-3i64) as u64, "sign is two's complement, not dropped");
    assert_eq!(p[1], 7);
    let huge = BigRational::new(BigInt::from(1) << 80, BigInt::from(1));
    assert!(axes_of(&huge).is_none(), "overflow refuses, never saturates to MAX");
}
