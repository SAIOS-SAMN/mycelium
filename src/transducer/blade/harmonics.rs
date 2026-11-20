// 20, 24.4, A6, 51.3, 78.188 — THE HARMONICS of a body: its readings, and
// the chart it is filed under, which holds only what is invariant on Sⁿ.
//
//! Exact in ℚ. THE CHART IS READ ON Sⁿ (78.188; Concordance 1.1, 2.10): the
//! base is the sphere `Sⁿ = ℝⁿ ∪ {∞}`, whose motions are the conformal
//! (Möbius) maps the algebra `Cl(n+1, 1)` carries. A single body has no
//! metric invariant under them: any round can be carried onto any other.
//! So the chart's axes are the base's `n`, the Betti numbers of the body on
//! Sⁿ (homeomorphism invariants), then the hue spectrum (a fraction of the
//! body's own compartments, which no map of the base moves). The turns and
//! the boundary and moment ratios are similarity invariants of the sensor's
//! lattice, not of Sⁿ: they stay readings the physics measures, never chart
//! axes. A prefix of a chart is a stratum bodies share (A21 §2), and the
//! atlas is read by peel.
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
    /// β₀ … β_{n−1} of the body on Sⁿ (78.188): components, tunnels,
    /// cavities, and every higher hole the base's `n` has room for. `n` of
    /// them, generated from `n`. They are read on the sensor's cover of the
    /// body ([`blade::topology`]); being homeomorphism invariants, they do not
    /// depend on that cover, and they close on Sⁿ by Euler–Poincaré and by
    /// Alexander duality with the complement, whose component holding ∞ is
    /// the outside.
    pub betti: Vec<u64>,
    /// The sections' turning count.
    pub turns: u64,
    /// h₄ — the in-plane boundary's length² over the compartments (the
    /// isoperimetric ratio).
    pub isoperimetric: BigRational,
    /// h₅ — the sides open along `e₃` over the compartments: two for one
    /// section, less for a stack.
    pub enclosure_boundary: BigRational,
    /// h₆ — det(I)/tr(I)² on the plane (anisotropy of the inertia).
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
        let open = count(blade::enclosure_faces(&b.support));
        // The anisotropy is read on the plane, named (BQ4); the spread is
        // the trace on every axis the body has.
        let (gi, gj) = crate::transducer::perspective::family_plane(b.n())?;
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
            enclosure_boundary: &open / &n,
            anisotropy: match tr.is_zero() {
                true => BigRational::zero(),
                false => &det / (&tr * &tr),
            },
            spread: trace / (&n * &n),
            gauge,
        })
    }

    /// The harmonics laid out as the atlas's axes: [`sphere_axes`] on this
    /// body's `n`, Betti numbers and hue spectrum. The turns and the four
    /// ratios are similarity invariants of the sensor's lattice and are not
    /// filed on (78.188).
    pub fn axes(&self) -> Vec<crate::transducer::stalk::ChartAxis> {
        sphere_axes(self.n, &self.betti, &self.gauge)
    }
}

/// THE CHART ON Sⁿ (78.188), coarsest first: the base's `n`, the `n` Betti
/// numbers, then the hue spectrum. Each is ONE coordinate carrying its own
/// exact value (A21 as ruled 2026-09-18). Strata of different `n` are
/// disjoint by construction, since `n` leads; within one `n` a prefix is a
/// coarser stratum (A21 §2), and the count axes are not prefix-stable ACROSS
/// `n` (71.2(6)). The ONE producer of a body's chart (law 12): the body's
/// harmonics and a section's turns both write it here.
pub fn sphere_axes(n: u64, betti: &[u64], gauge: &[BigRational]) -> Vec<crate::transducer::stalk::ChartAxis> {
    use crate::transducer::stalk::ChartAxis;
    let mut axes: Vec<ChartAxis> = std::iter::once(n).chain(betti.iter().copied()).map(ChartAxis::Count).collect();
    axes.extend(gauge.iter().cloned().map(ChartAxis::Ratio));
    axes
}

/// HEAR a voice's carried body — one section's chain — lift it through
/// the eye and read its harmonics. None for an empty body.
pub fn hear(body: &[crate::transducer::sentence::SectionLink]) -> Option<Harmonics> {
    let shape = crate::transducer::element::shape_of_chain(body)?;
    Harmonics::of(&shape.blade)
}
