use crate::transducer::family::perceive::Grid;
use crate::transducer::family::refine::*;

/// A stroke of exactly `n` reads.
fn stroke(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

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
    let chain = r.bipartite_cochain(&a, &b, &mut stroke(r.need)).expect("both grids are square");
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

/// THE RUN COUNT IS `R(a, b) = a + b − gcd(a, b)`: the boundaries of both
/// grids less the ones they share, plus one. The runs walked on `K_lcm`
/// number exactly that, on commensurate, coprime and equal extents.
#[test]
fn runs_count_equals_sum_less_gcd() {
    for (a, b, r) in [(3, 4, 6), (29, 30, 58), (2, 4, 4), (6, 9, 12), (5, 5, 5), (1, 7, 7), (30, 29, 58)] {
        assert_eq!(runs_count(a, b), Some(r), "R({a}, {b})");
        let refn = CommonRefinement::of(a, 1, b, 1).expect("extents stand");
        let walked = CommonRefinement::runs(refn.lcm_h, refn.scale_in.0, refn.scale_out.0);
        assert_eq!(walked.len(), r, "the runs walked on lcm({a}, {b})");
    }
}

/// An extent of zero embeds in nothing, and a refinement of it would be a
/// complex invented for a scene that is not there (rule 11).
#[test]
fn of_refuses_empty_extent() {
    for (hi, wi, ho, wo) in [(0, 2, 2, 2), (2, 0, 2, 2), (2, 2, 0, 2), (2, 2, 2, 0)] {
        assert!(matches!(CommonRefinement::of(hi, wi, ho, wo), Err(Unrefined::Empty)));
        assert_eq!(need(hi, wi, ho, wo), None, "no refinement, nothing to need");
    }
    assert!(CommonRefinement::ratio(0, 2, 2, 2).is_none());
}

/// A ragged grid is not the extent it claims, and reading a hue past its
/// end would be inventing one.
#[test]
fn bipartite_cochain_refuses_ragged_grid() {
    let r = CommonRefinement::of(2, 2, 2, 2).expect("extents stand");
    let a: Grid = vec![vec![1, 1], vec![1]];
    let b: Grid = vec![vec![2, 2], vec![2, 2]];
    assert_eq!(r.bipartite_cochain(&a, &b, &mut stroke(r.need)), Err(Unrefined::Ragged));
    assert_eq!(refine_delta(&a, &b, &mut stroke(r.need)), Err(Unrefined::Ragged));
}

/// MU-DERIVATIONS D3.2 — 29×29 → 30×30 IS THE EXACT RATIO 30/29 and refines,
/// no lcm capped. Its need is read off the four extents:
/// `R = 29 + 30 − 1 = 58` runs per axis, `L = 870`, so
/// `need = 58·58 + 2·870² = 1 517 164`. A pair that differs at every site
/// spends exactly that need (the cochain writes every cell of `K_lcm` and
/// the pulled-back input every cell again), and the chain is exact: one
/// output site that changed is its own `29 × 29` block of `K_lcm`.
#[test]
fn common_refinement_reads_coprime_thirty() {
    assert_eq!(runs_count(29, 30), Some(58));
    assert_eq!(need(29, 29, 30, 30), Some(58 * 58 + 2 * 870 * 870));
    assert_eq!(need(29, 29, 30, 30), Some(1_517_164));
    let r = CommonRefinement::of(29, 29, 30, 30).expect("30/29 is an exact ratio");
    assert_eq!((r.lcm_h, r.lcm_w, r.scale_in, r.scale_out), (870, 870, (30, 30), (29, 29)));

    // Every site differs: the whole need is spent, and nothing past it.
    let a: Grid = vec![vec![1; 29]; 29];
    let b: Grid = vec![vec![2; 30]; 30];
    let mut whole = stroke(1_517_164);
    let refined = crate::arc::refine_delta(&a, &b, &mut whole).expect("the need is the stroke");
    assert_eq!(whole.remaining(), 0, "a pair differing everywhere spends its need exactly");
    assert_eq!(refined.delta.len(), 870 * 870);
    assert!(refined.delta.iter().all(|&(_, _, before, after)| (before, after) == (1, 2)));
    assert_eq!(refined.pulled_back.len(), 870);
    assert!(refined.pulled_back.iter().all(|row| row.len() == 870 && row.iter().all(|&h| h == 1)));

    // One output site changed: its block of K_lcm and no other cell.
    let mut c: Grid = vec![vec![1; 30]; 30];
    c[1][2] = 5;
    let chain = refine_delta(&a, &c, &mut stroke(r.need)).expect("refines");
    let mut block: Vec<(i64, i64, u8, u8)> = Vec::new();
    for row in 29..58 {
        for col in 58..87 {
            block.push((row, col, 1, 5));
        }
    }
    let mut read = chain;
    read.sort_unstable();
    assert_eq!(read, block, "the changed site's 29×29 block, exactly");
}

/// A STROKE ONE SHORT OF THE NEED REFUSES BY NAME, and returns no chain and
/// no grid: the refinement is atomic by rollback of its result. On a 2×2 →
/// 3×3 pair differing everywhere, `need = R(2, 3)² + 2·6² = 16 + 72 = 88`.
#[test]
fn common_refinement_refuses_spent_stroke() {
    let a: Grid = vec![vec![1; 2]; 2];
    let b: Grid = vec![vec![2; 3]; 3];
    let need = need_of(&a, &b).expect("both extents stand");
    assert_eq!(need, 88);
    assert!(matches!(
        crate::arc::refine_delta(&a, &b, &mut stroke(need - 1)),
        Err(Unrefined::Spent { need: 88 })
    ));
    let mut whole = stroke(need);
    assert!(crate::arc::refine_delta(&a, &b, &mut whole).is_ok(), "the need suffices");
    assert_eq!(whole.remaining(), 0, "and is spent whole");
}
