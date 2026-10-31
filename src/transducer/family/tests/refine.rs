use crate::transducer::family::perceive::Grid;
use crate::transducer::family::refine::*;

/// #97 — TWO GRIDS OF UNEQUAL EXTENT READ ONE COMPLEX, exactly.
///
/// A 2×2 doubled to 4×4 has `lcm = 4`: the input embeds two-for-one and
/// the output one-for-one, so a site that changed hue contributes its
/// whole block and one that did not contributes nothing.
#[test]
fn common_refinement_embeds_doubled_grid_exactly() {
    let r = CommonRefinement::of(2, 2, 4, 4).expect("both extents stand");
    assert_eq!((r.lcm_h, r.lcm_w), (4, 4));
    assert_eq!(r.scale_in, (2, 2), "each input site is a 2×2 block of K_lcm");
    assert_eq!(r.scale_out, (1, 1), "each output site is one site of K_lcm");
    assert!(!r.is_trivial());

    // Input all hue 1; output all hue 1 but for its top-left site.
    let a: Grid = vec![vec![1, 1], vec![1, 1]];
    let mut b: Grid = vec![vec![1; 4]; 4];
    b[0][0] = 7;
    let chain = r.bipartite_cochain(&a, &b).expect("both grids are square");
    assert_eq!(chain.len(), 1, "one output site differs, so one site of K_lcm does");
    assert_eq!(chain[0], (0, 0, 1, 7));
}

/// THE RATIO IS EXACT ℚ and never a float: 3 rows to 2 is three halves,
/// which has no binary expansion and must not acquire one.
#[test]
fn ratio_is_exact_rational() {
    let (rh, rw) = CommonRefinement::ratio(2, 4, 3, 2).expect("extents stand");
    assert_eq!(rh, num_rational::BigRational::new(3.into(), 2.into()));
    assert_eq!(rw, num_rational::BigRational::new(2.into(), 4.into()));
}

/// RUNS ARE THE BOUNDARIES OF THE TWO GRIDS, not the sites of the grids' least
/// common multiple. Coprime extents are where that matters: 3 and 4 have
/// `lcm` 12, and the pair changes at six places, not twelve.
#[test]
fn runs_is_bounded_by_grids_not_lcm() {
    let r = CommonRefinement::of(3, 1, 4, 1).expect("extents stand");
    assert_eq!(r.lcm_h, 12);
    let runs = CommonRefinement::runs(12, r.scale_in.0, r.scale_out.0);
    assert!(
        runs.len() <= 3 + 4,
        "at most one run per subdivision boundary of either grid, got {}",
        runs.len()
    );
    // And they tile the axis exactly — no coordinate is skipped or
    // counted twice.
    let total: usize = runs.iter().map(|&(_, n, _, _)| n).sum();
    assert_eq!(total, 12, "the runs cover K_lcm exactly once");
    for &(at, n, i_in, i_out) in &runs {
        assert_eq!(at / r.scale_in.0, i_in, "the run's input site is constant on it");
        assert_eq!(at / r.scale_out.0, i_out, "and so is its output site");
        assert_eq!(
            (at + n - 1) / r.scale_in.0,
            i_in,
            "constant at the run's far end too"
        );
    }
}

/// An extent of zero embeds in nothing, and a refinement of it would be a
/// complex invented for a scene that is not there (rule 11).
#[test]
fn of_refuses_empty_extent() {
    assert!(CommonRefinement::of(0, 2, 2, 2).is_none());
    assert!(CommonRefinement::of(2, 0, 2, 2).is_none());
    assert!(CommonRefinement::of(2, 2, 0, 2).is_none());
    assert!(CommonRefinement::of(2, 2, 2, 0).is_none());
    assert!(CommonRefinement::ratio(0, 2, 2, 2).is_none());
}

/// A ragged grid is not the extent it claims, and reading a hue past its
/// end would be inventing one.
#[test]
fn bipartite_cochain_refuses_ragged_grid() {
    let r = CommonRefinement::of(2, 2, 2, 2).expect("extents stand");
    let a: Grid = vec![vec![1, 1], vec![1]];
    let b: Grid = vec![vec![2, 2], vec![2, 2]];
    assert!(r.bipartite_cochain(&a, &b).is_none());
}

/// Incommensurate frames whose least common multiple exceeds the complex
/// bound refuse (rules 10 and 11) rather than expanding hundreds of
/// thousands of sites.
#[test]
fn of_refuses_incommensurate_frames() {
    assert!(CommonRefinement::of(29, 29, 30, 30).is_none());
    assert!(CommonRefinement::ratio(29, 29, 30, 30).is_none());
}
