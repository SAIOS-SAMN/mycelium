// 20, 24.4, A6, 51.3 — THE HARMONICS of a body: the conformal invariants
// of its stack and the hue spectrum, the axes it is heard on, coarsest
// first.
//
//! Exact in ℚ and invariant under every versor `carry_compartment` accepts: what
//! a body sounds like, whatever its size, place, frame or orientation. The
//! axes run coarsest first — the base's `n`, the topology (the Betti
//! numbers of the union, then the turns), then the boundary and moment
//! ratios, then the hue spectrum — so a prefix of a chart is a stratum
//! bodies share (A21 §2) and the atlas is read by peel.
//!
//! THE CHART ONLY LOCATES (71.2(6)): the physics decides congruence and
//! contact, the tower, the frame and the tether; these axes find the
//! candidates a reader then measures.

use crate::transducer::blade::{self, Blade};
use num_rational::BigRational;
use num_traits::Zero;

/// THE HARMONICS of a body, in axis order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Harmonics {
    /// The base's `n`, the chart's first axis: strata of different `n` are
    /// disjoint by construction (71.2(1), 38.2's `d` prefix).
    pub n: u64,
    /// β₀ … β_{n−1} of the closed union of the body's cubes (38.2, 71.2(6)):
    /// components, tunnels, cavities, and every higher hole the base's `n`
    /// has room for. `n` of them, generated from `n`.
    pub betti: Vec<u64>,
    /// The sections' turning count.
    pub turns: u64,
    /// h₄ — the in-plane boundary's length² over the compartments (the
    /// isoperimetric ratio).
    pub isoperimetric: BigRational,
    /// h₅ — the sides open along `e₃` over the compartments: two for one
    /// section, less for a stack.
    pub frame_boundary: BigRational,
    /// h₆ — det(I)/tr(I)² on the grid plane (anisotropy of the inertia).
    pub anisotropy: BigRational,
    /// h₇ — the trace on every axis over the compartments² (spread per area).
    ///
    /// THE CHART STOPS AT ORDER TWO (ruled 2026-09-26, compartments as rounds): the
    /// isotropic fourth moment stood here as h₈, and it was invariant under a
    /// blow-up only while each compartment was a square, whose `k×k` block is one
    /// square. A body's rounds carry a blow-up exactly through order three
    /// and leave the block's own residue at four, so an order-four axis would
    /// file a body and its blow-up in two fibres and the chart would stop
    /// locating what the physics relates (71.2(6)). The fourth order is the
    /// transition's to read, with its residue.
    pub spread: BigRational,
    /// h₈… — the hue fractions, sorted: the gauge spectrum.
    pub gauge: Vec<BigRational>,
}

impl Harmonics {
    /// Hear a body at its own `n`. `None` where its topology is refused
    /// ([`blade::topology`]): no harmonics are invented for it.
    pub fn of(b: &Blade) -> Option<Harmonics> {
        let betti = blade::topology(&b.support)?;
        let n = b.shell.clone();
        let count = |v: usize| BigRational::from_integer(num_bigint::BigInt::from(v as i64));
        let edges = count(blade::boundary_length(&b.support));
        let open = count(blade::frame_faces(&b.support));
        // The anisotropy is read on the grid plane, named (BQ4); the spread is
        // the trace on every axis the body has.
        let (gi, gj) = crate::transducer::base::GRID_PLANE;
        let (ixx, iyy, ixy) = (b.moment2(gi, gi)?, b.moment2(gj, gj)?, b.moment2(gi, gj)?);
        let tr = ixx + iyy;
        let det = ixx * iyy - ixy * ixy;
        let trace = (0..b.n()).filter_map(|i| b.moment2(i, i)).fold(BigRational::zero(), |s, x| s + x);
        let mut gauge: Vec<BigRational> = b.gauge.iter().map(|(_, c)| c / &n).collect();
        gauge.sort();
        Some(Harmonics {
            n: b.n() as u64,
            betti,
            turns: blade::turns(&b.support) as u64,
            isoperimetric: &edges * &edges / &n,
            frame_boundary: &open / &n,
            anisotropy: match tr.is_zero() {
                true => BigRational::zero(),
                false => &det / (&tr * &tr),
            },
            spread: trace / (&n * &n),
            gauge,
        })
    }

    /// The harmonics laid out as the atlas's axes, coarsest first: the
    /// base's `n`, the `n` Betti numbers and the turns, then the conformal
    /// ratios, then the hue spectrum. Each is ONE coordinate carrying its
    /// own exact value (A21 as ruled 2026-09-18), so a ratio is a
    /// `ChartAxis::Ratio` in ℚ and never a pair of integer limbs that
    /// `peel` could cut between. Strata of different `n` are disjoint by
    /// construction, since `n` leads; within one `n` a prefix is a coarser
    /// stratum (A21 §2), and the count axes are not prefix-stable ACROSS
    /// `n` (71.2(6)).
    pub fn axes(&self) -> Vec<crate::transducer::stalk::ChartAxis> {
        use crate::transducer::stalk::ChartAxis;
        let mut axes: Vec<ChartAxis> = std::iter::once(self.n)
            .chain(self.betti.iter().copied())
            .chain(std::iter::once(self.turns))
            .map(ChartAxis::Count)
            .collect();
        for r in [
            &self.isoperimetric,
            &self.frame_boundary,
            &self.anisotropy,
            &self.spread,
        ] {
            axes.push(ChartAxis::Ratio(r.clone()));
        }
        for f in &self.gauge {
            axes.push(ChartAxis::Ratio(f.clone()));
        }
        axes
    }
}

/// HEAR a voice's carried body — one section's chain — lift it through
/// the eye and read its harmonics. None for an empty body.
pub fn hear(body: &[(i64, i64, u8, u8)]) -> Option<Harmonics> {
    let shape = crate::transducer::element::shape_of_chain(body)?;
    Harmonics::of(&shape.blade)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A body sounds the same at every size, place, and orientation, and
    /// its harmonics are the atlas's axes.
    #[test]
    fn a_body_sounds_the_same_wherever_it_is() {
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
    /// one component through their adjacency along `e₃`, a frame boundary
    /// smaller than a section's, and a spread the shadow does not carry;
    /// the same three sections one frame apart read three components. A
    /// bar standing along `e₃` reads no chart a section reads.
    #[test]
    fn a_stack_reads_its_own_topology_and_not_its_shadows() {
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
    fn a_body_of_n_four_lifts_and_its_chart_has_the_axes_the_harmonics_name() {
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
    fn the_charts_topology_is_the_closed_union_of_the_cubes() {
        use crate::transducer::base::Site;
        let corner: Vec<Site> = [(0i64, 0i64, 0i64), (1, 1, 0)].into_iter().map(Site::from).collect();
        assert_eq!(blade::topology(&corner), Some(vec![1, 0, 0]), "a corner joins the union");
        let apart: Vec<Site> = [(0i64, 0i64, 0i64), (2, 2, 0)].into_iter().map(Site::from).collect();
        assert_eq!(blade::topology(&apart), Some(vec![2, 0, 0]), "a gap does not");
    }
}
