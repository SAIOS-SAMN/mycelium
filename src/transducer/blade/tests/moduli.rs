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
    assert_eq!(turns(&bar.blade.support), 4);
    let l = at_frame_0(&[(0, 0), (1, 0), (2, 0), (2, 1)]);
    let l_shape = shape_of(&l);
    assert_eq!(turns(&l_shape.blade.support), 6);
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
    assert_eq!(carry(&v2, 0, 0, 0), Some((q(0), q(0), q(0))));
    assert_eq!(carry(&v2, 2, 3, 0), Some((q(4), q(6), q(0))));
    // The mirrored L needs the reflection; a rotated L is a rotation; a
    // bar and an L are not similar.
    let l_mirror = at_frame_0(&[(0, 1), (1, 1), (2, 1), (2, 0)]);
    assert!(versor_between(&l_shape, &shape_of(&l_mirror)).is_some(), "reflection found");
    let l_rot = at_frame_0(&[(7, -3), (7, -2), (7, -1), (6, -1)]);
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
    let pin = at_frame_0(&[(1, 1), (1, 2), (2, 2), (0, 1), (1, 0), (2, 1), (1, 3), (3, 2), (0, 3)]);
    let pin_shape = shape_of(&pin);
    let pin_rot: Vec<(i64, i64, i64)> = pin.iter().map(|&(r, c, k)| (c, -r, k)).collect();
    assert_eq!(moduli(&pin_shape.blade), moduli(&shape_of(&pin_rot).blade));
    assert!(versor_between(&pin_shape, &shape_of(&pin_rot)).is_some(), "a quarter turn found");
}

/// 50.2 step 1 as ruled 2026-09-09 (L7c): several versors carry a
/// symmetric body — the coset of its stabilizer on one transport — and the
/// one returned is the coset's geodesic, the member nearest the identity,
/// the same whatever order the candidates were tried in. A dilated square
/// is carried by the dilation and not by the dilation and one of its
/// turns; a dilated bar by the dilation and not by the dilation and its
/// half turn.
#[test]
fn the_versor_between_symmetric_bodies_is_the_cosets_geodesic() {
    let sq = shape_of(&rect(0, 0, 2, 2));
    let sq3 = shape_of(&rect(4, 4, 6, 6));
    let coset = versor_coset(&sq, &sq3);
    assert!(coset.len() > 1, "a square's stabilizer is not trivial: {} members", coset.len());
    let v = versor_between(&sq, &sq3).expect("carried");
    let mut reversed = coset.clone();
    reversed.reverse();
    assert_eq!(
        coset_representative(&reversed).map(|r| r.key()),
        Some(v.key()),
        "the representative does not depend on the order the coset was measured in"
    );
    assert_eq!(carry(&v, 0, 0, 0), Some((q(4), q(4), q(0))), "the dilation alone: (0,0) → (4,4)");
    assert_eq!(carry(&v, 2, 2, 0), Some((q(10), q(10), q(0))), "and (2,2) → (10,10)");
    for (_, member) in &coset {
        assert!(carries(member, &sq, &sq3), "every member of the coset carries the body");
    }
    let bar = shape_of(&rect(0, 0, 2, 3));
    let bar2 = shape_of(&rect(0, 0, 4, 6));
    let coset = versor_coset(&bar, &bar2);
    assert!(coset.len() > 1, "a bar's stabilizer holds its half turn: {} members", coset.len());
    let v2 = versor_between(&bar, &bar2).expect("a dilation");
    assert_eq!(carry(&v2, 0, 0, 0), Some((q(0), q(0), q(0))), "the dilation, not the dilation and a half turn");
    assert_eq!(carry(&v2, 2, 3, 0), Some((q(4), q(6), q(0))));
}

/// 24.4, A21 §2, Stage 12 S12.1a — A CHART'S PREFIX IS THE TOPOLOGICAL
/// STRATUM. Two rectangles of different proportions have the same
/// components, holes, cavities and turns and different moment ratios:
/// their charts differ, share the four coarsest axes, and the atlas read
/// at that depth holds both — the stratum a peel walks out to. The shared
/// stratum is four axes in, and it is read by peel, never by the whole
/// chart.
#[test]
fn a_chart_prefix_is_the_topological_stratum() {
    use crate::transducer::stalk::{Invariant, Stalks};
    let bar_support = rect(0, 0, 2, 3);
    let plank_support = rect(0, 0, 2, 5);
    let (a, b) = (moduli(&shape_of(&bar_support).blade), moduli(&shape_of(&plank_support).blade));
    assert_ne!(a, b, "different proportions, different charts");
    assert_eq!(a[..4], b[..4], "one topology: components, holes, cavities and turns lead the chart");
    assert_eq!(a[..4], [1u64, 0, 0, 4], "one component, no hole, no cavity, four turns");
    let word_of = |support: &[(i64, i64, i64)]| {
        let raw: Vec<(i64, i64, u8, u8)> =
            support.iter().map(|&(r, c, _)| (r, c, 1u8, 2u8)).collect();
        let chain = crate::transducer::element::canonical_chain(&raw)
            .expect("one entry per site is A5's chain");
        crate::transducer::element::from_canonical(&chain).address()
    };
    let (wa, wb) = (word_of(&bar_support), word_of(&plank_support));
    let (ia, ib) = (Invariant(a.clone()), Invariant(b.clone()));
    let mut atlas = Stalks::default();
    atlas.admit(&ia, wa.clone());
    atlas.admit(&ib, wb.clone());
    assert_eq!(atlas.fiber(&ia).len(), 1, "the exact chart holds one word");
    // The stratum is the fibre UNDER the prefix (A21 §2): `fiber` reads one
    // node's own members and holds nothing at a prefix no word sits on.
    let stratum = atlas.fiber_under(&Invariant(a[..4].to_vec()));
    assert!(
        stratum.contains(&wa.clone().into_vec()) && stratum.contains(&wb.clone().into_vec()),
        "the topological stratum holds both: {} members",
        stratum.len()
    );
    let mut at = ia.clone();
    while at.0.len() > 4 {
        at = at.peel();
    }
    assert_eq!(at.0, a[..4], "peel walks out to the topological stratum");
    assert_eq!(atlas.fiber_under(&at).len(), 2, "and the stratum's fibre is read under that prefix");
}

/// N7 (ruled 2026-09-18) — A RATIO WIDER THAN ℤ₆₄ IS A COORDINATE, NOT AN
/// ABSENCE. `axes_of` refused such a ratio, `moduli` read the refusal as an
/// empty axis list and `stalk::invariant` as the empty chart — the root of
/// the trie — so bodies whose only fault was the width of their own moduli
/// were filed together at one coordinate, and with every word that has no
/// support at all.
#[test]
fn a_ratio_wider_than_an_axis_is_written_as_limbs_and_keeps_its_sign() {
    let neg = BigRational::new(BigInt::from(-3), BigInt::from(7));
    let p = axes_of(&neg);
    assert_eq!(p, vec![1, 1, 3, 1, 7], "sign, then each magnitude's limbs by count");
    let pos = BigRational::new(BigInt::from(3), BigInt::from(7));
    assert_ne!(axes_of(&pos), p, "and the sign is a coordinate, not dropped");
    // WIDER THAN ℤ₆₄ IS STILL A COORDINATE, and two distinct wide ratios
    // are two coordinates — which is the collapse this retires.
    let huge = BigRational::new(BigInt::from(1) << 80, BigInt::from(1));
    let wider = BigRational::new(BigInt::from(1) << 81, BigInt::from(1));
    let (a, b) = (axes_of(&huge), axes_of(&wider));
    assert_eq!(a[1], 2, "a ratio past one limb writes two of them");
    assert!(!a.is_empty() && !b.is_empty(), "neither is read as absence");
    assert_ne!(a, b, "and they are distinct coordinates, not one root bucket");
}

/// The same reading one level up: a body whose harmonics exceed ℤ₆₄ has a
/// chart of its own, where it was charted at the root before N7.
#[test]
fn a_body_is_never_charted_at_the_root_for_the_width_of_its_moduli() {
    use crate::transducer::stalk::invariant;
    let l: Vec<(i64, i64, u8, u8)> = vec![(0, 0, 1, 2), (1, 0, 1, 2), (2, 0, 1, 2), (2, 1, 1, 2)];
    let inv = invariant(&l);
    assert!(!inv.0.is_empty(), "a body with support is charted by its moduli");
    // The root belongs to genuine absence alone (`invariant_of`'s no-support
    // arm), which is above this and untouched.
    assert!(invariant(&[]).0.is_empty(), "and no support is still the root");
}
