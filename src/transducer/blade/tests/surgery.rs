use super::common::{at_frame_0, sites};
use crate::transducer::blade::{body::*, surgery::*};
use num_rational::BigRational;

#[test]
fn fill_hole_surgery_reduces_genus() {
    // 3x3 ring with center (1, 1) empty: beta_0 = 1, beta_1 = 1
    let ring = at_frame_0(&[
        (0, 0), (0, 1), (0, 2),
        (1, 0),         (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]);
    let t_ring = topology(&ring);
    assert_eq!(t_ring, Some(vec![1, 1, 0]), "ring must have beta_0=1, beta_1=1");

    // 3x3 solid square: beta_0 = 1, beta_1 = 0
    let solid = at_frame_0(&[
        (0, 0), (0, 1), (0, 2),
        (1, 0), (1, 1), (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]);
    let t_solid = topology(&solid);
    assert_eq!(t_solid, Some(vec![1, 0, 0]), "solid square must have beta_0=1, beta_1=0");

    let surg = Surgery::detect(&ring, &solid).expect("detect fill hole");
    assert_eq!(surg.kind, SurgeryKind::FillHole);
    assert_eq!(surg.delta_betti, (0, -1));
    assert_eq!(surg.support_delta, sites(&[(1, 1, 0)]));

    let applied = surg.apply(&ring);
    assert_eq!(applied, solid);
    assert_eq!(topology(&applied), Some(vec![1, 0, 0]));
}

#[test]
fn puncture_surgery_increases_genus() {
    let solid = at_frame_0(&[
        (0, 0), (0, 1), (0, 2),
        (1, 0), (1, 1), (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]);
    let ring = at_frame_0(&[
        (0, 0), (0, 1), (0, 2),
        (1, 0),         (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]);

    let surg = Surgery::detect(&solid, &ring).expect("detect puncture");
    assert_eq!(surg.kind, SurgeryKind::Puncture);
    assert_eq!(surg.delta_betti, (0, 1));
    assert_eq!(surg.support_delta, sites(&[(1, 1, 0)]));

    let applied = surg.apply(&solid);
    assert_eq!(applied, ring);
    assert_eq!(topology(&applied), Some(vec![1, 1, 0]));
}

#[test]
fn bridge_merge_connects_components() {
    // Two 1x2 blocks at rows 0 and 2
    let two_blocks = at_frame_0(&[
        (0, 0), (0, 1),
        (2, 0), (2, 1),
    ]);
    assert_eq!(topology(&two_blocks), Some(vec![2, 0, 0]));

    // Connected by a bridge at (1, 0)
    let connected = at_frame_0(&[
        (0, 0), (0, 1),
        (1, 0),
        (2, 0), (2, 1),
    ]);
    assert_eq!(topology(&connected), Some(vec![1, 0, 0]));

    let surg = Surgery::detect(&two_blocks, &connected).expect("detect bridge merge");
    assert_eq!(surg.kind, SurgeryKind::BridgeMerge);
    assert_eq!(surg.delta_betti, (-1, 0));
    assert_eq!(surg.support_delta, sites(&[(1, 0, 0)]));

    let applied = surg.apply(&two_blocks);
    assert_eq!(applied, connected);
    assert_eq!(topology(&applied), Some(vec![1, 0, 0]));
}

#[test]
fn split_cut_disconnects_components() {
    let connected = at_frame_0(&[
        (0, 0), (0, 1),
        (1, 0),
        (2, 0), (2, 1),
    ]);
    let two_blocks = at_frame_0(&[
        (0, 0), (0, 1),
        (2, 0), (2, 1),
    ]);

    let surg = Surgery::detect(&connected, &two_blocks).expect("detect split cut");
    assert_eq!(surg.kind, SurgeryKind::SplitCut);
    assert_eq!(surg.delta_betti, (1, 0));
    assert_eq!(surg.support_delta, sites(&[(1, 0, 0)]));

    let applied = surg.apply(&connected);
    assert_eq!(applied, two_blocks);
    assert_eq!(topology(&applied), Some(vec![2, 0, 0]));
}

#[test]
fn monoid_composition_sums_betti_deltas() {
    let s1 = Surgery::fill_hole(sites(&[(1, 1, 0)]));
    let s2 = Surgery::fill_hole(sites(&[(4, 4, 0)]));
    let comp = s1.compose(&s2);
    assert_eq!(comp.delta_betti, (0, -2));
    assert_eq!(comp.support_delta, sites(&[(1, 1, 0), (4, 4, 0)]));
}

#[test]
fn factor_composite_deformation_phi_is_versor_circ_surgery() {
    let ring = at_frame_0(&[
        (0, 0), (0, 1), (0, 2),
        (1, 0),         (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]);
    let solid = at_frame_0(&[
        (0, 0), (0, 1), (0, 2),
        (1, 0), (1, 1), (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]);

    let src = shape_of(&ring);
    let dst = shape_of(&solid);

    let (surg, versor) = Surgery::factor(&src, &dst, &mut super::common::budget()).expect("factor Phi into V circ S");
    assert_eq!(surg.kind, SurgeryKind::FillHole);
    assert_eq!(surg.delta_betti, (0, -1));
    // The core after surgery is the solid square at identical location, so versor is identity
    let id = crate::transducer::cga::Mv::scalar(BigRational::from_integer(1.into()));
    assert_eq!(versor, id);
}
