use crate::transducer::blade;
use crate::transducer::blade::harmonics::*;
use num_rational::BigRational;

/// A body sounds the same at every size, place, and orientation, and
/// its harmonics are the atlas's axes.
#[test]
fn hear_invariant_under_motion() {
    let l: Vec<(i64, i64, u8, u8)> = vec![(0, 0, 1, 2), (1, 0, 1, 2), (2, 0, 1, 2), (2, 1, 1, 2)];
    let moved: Vec<(i64, i64, u8, u8)> = vec![(7, -3, 1, 2), (7, -2, 1, 2), (7, -1, 1, 2), (6, -1, 1, 2)];
    let (a, b) = (hear(&l).unwrap(), hear(&moved).unwrap());
    assert_eq!(a, b);
    assert_eq!(a.n, 3, "the corpus's base");
    assert_eq!(a.betti, vec![1, 0, 0], "one component, no tunnel, and a section encloses nothing");
    assert_eq!(a.turns, 6);
    assert_eq!(
        a.frame_boundary,
        BigRational::from_integer(2.into()),
        "a section is open on both sides along e₃"
    );
    assert_eq!(
        a.axes(),
        crate::transducer::stalk::invariant(&l).0
    );
}

/// 51.3 — THE STACK IS NOT ITS SHADOW. Three identical sections read
/// one component through the sections' adjacency along `e₃`, a frame boundary
/// smaller than a section's, and a spread the shadow does not carry;
/// the same three sections one frame apart read three components. A
/// bar standing along `e₃` reads no chart a section reads.
#[test]
fn harmonics_of_stack_differ_from_its_shadow() {
    use crate::transducer::base::Site;
    let section: Vec<Site> = [(0, 0, 0), (0, 1, 0), (1, 0, 0)].into_iter().map(Site::from).collect();
    let at_frame = |k: i64| -> Vec<Site> {
        section.iter().map(|s| Site::of([s.axes()[0], s.axes()[1], k])).collect()
    };
    let stack: Vec<Site> = (0..3).flat_map(at_frame).collect();
    let spaced: Vec<Site> = (0..3).flat_map(|k| at_frame(2 * k)).collect();
    let hear = |s: &[Site]| Harmonics::of(&blade::shape_of(s).blade).expect("a body at n = 3");
    let (one, three, apart) = (hear(&section), hear(&stack), hear(&spaced));
    assert_eq!(three.betti[0], 1, "consecutive frames join along e₃");
    assert_eq!(apart.betti[0], 3, "frames apart do not");
    assert_eq!(three.turns, 3 * one.turns, "each section turns as it turns alone");
    assert!(three.frame_boundary < one.frame_boundary, "a stack is open on fewer sides");
    assert_ne!(three.spread, one.spread, "and its spread carries the offsets along e₃");
    assert_eq!(
        blade::shape_of(&stack).blade.inertia[3],
        BigRational::from_integer(6.into()),
        "Izz is the offsets alone: three sections one frame from the middle"
    );
    let bar_along_k = [(0, 0, 0), (0, 0, 1), (0, 0, 2)];
    let bar_in_plane = [(0, 0, 0), (0, 1, 0), (0, 2, 0)];
    assert_ne!(
        blade::moduli(&blade::shape_of(&bar_along_k).blade),
        blade::moduli(&blade::shape_of(&bar_in_plane).blade),
        "a bar standing along e₃ is not a bar lying in a frame"
    );
    let hollow: Vec<Site> = (0..3)
        .flat_map(|r| (0..3).flat_map(move |c| (0..3).map(move |k| (r, c, k))))
        .filter(|&p| p != (1, 1, 1))
        .map(Site::from)
        .collect();
    assert_eq!(blade::topology(&hollow), Some(vec![1, 0, 1]), "a hollow cube encloses one cavity");
}

/// 38.2, BQ4 — A BODY OF `n = 4` LIFTS, AND ITS CHART HAS THE AXES 38.2
/// NAMES: the base's `n` first, then the Betti numbers of the union at
/// that `n` — four of them — then the turns and the ratios. Topology
/// refuses at no `n`: a hollow 4-cube (the 3⁴ block less its centre)
/// encloses one 3-cavity, `β₃ = 1`, and a planar ring carried into ℤ⁴
/// keeps its one tunnel.
#[test]
fn harmonics_axes_at_n_four_lead_with_n_and_betti() {
    use crate::transducer::base::Site;
    use crate::transducer::stalk::ChartAxis;
    // A small body in ℤ⁴: an L on the grid plane standing in two cells
    // of the fourth axis.
    let body: Vec<Site> = [
        [0i64, 0, 0, 0],
        [1, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 1, 0, 0],
        [2, 1, 0, 1],
    ]
    .into_iter()
    .map(Site::of)
    .collect();
    let shape = blade::shape_of(&body);
    assert_eq!(shape.blade.n(), 4, "the body stands on the base of four");
    assert_eq!(shape.blade.centroid().map(|c| c.len()), Some(4), "its centroid is in ℚ⁴");
    assert_eq!(shape.blade.planes().len(), 6, "C(4, 2) planes, generated");
    assert_eq!(shape.blade.quartic.len(), 6, "a fourth-moment triple on each");
    let h = Harmonics::of(&shape.blade).expect("topology refuses at no n");
    assert_eq!(h.n, 4);
    assert_eq!(h.betti, vec![1, 0, 0, 0], "one component and no hole at any grade");
    let axes = h.axes();
    assert_eq!(axes[0], ChartAxis::Count(4), "the chart opens with its n");
    assert_eq!(&axes[1..5], &[1u64, 0, 0, 0].map(ChartAxis::Count), "then β₀ … β₃");
    // The hollow 4-cube: every site of the 3⁴ block but its centre.
    let mut hollow: Vec<Site> = Vec::new();
    for a in 0..3i64 {
        for b in 0..3i64 {
            for c in 0..3i64 {
                for d in 0..3i64 {
                    if (a, b, c, d) != (1, 1, 1, 1) {
                        hollow.push(Site::of([a, b, c, d]));
                    }
                }
            }
        }
    }
    assert_eq!(blade::topology(&hollow), Some(vec![1, 0, 0, 1]), "a hollow 4-cube encloses one 3-cavity");
    // A ring in the grid plane, lifted into ℤ⁴ by the inclusion: one
    // tunnel at every n.
    let ring: Vec<Site> = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]
        .into_iter()
        .map(|(r, c)| Site::of([r, c, 0, 0]))
        .collect();
    assert_eq!(blade::topology(&ring), Some(vec![1, 1, 0, 0]), "a ring has one tunnel at n = 4 too");
}

/// 71.2(6) — THE COMPLEX IS THE UNION'S. Two compartments meeting only at a
/// corner are two bodies (face adjacency) and ONE component of the
/// union; the chart reads the union.
#[test]
fn topology_reads_closed_union_of_cubes() {
    use crate::transducer::base::Site;
    let corner: Vec<Site> = [(0i64, 0i64, 0i64), (1, 1, 0)].into_iter().map(Site::from).collect();
    assert_eq!(blade::topology(&corner), Some(vec![1, 0, 0]), "a corner joins the union");
    let apart: Vec<Site> = [(0i64, 0i64, 0i64), (2, 2, 0)].into_iter().map(Site::from).collect();
    assert_eq!(blade::topology(&apart), Some(vec![2, 0, 0]), "a gap does not");
}
