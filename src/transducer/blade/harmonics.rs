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
