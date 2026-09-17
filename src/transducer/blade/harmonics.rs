// 20, 24.4, A6, 51.3 — THE HARMONICS of a body: the conformal invariants
// of its stack and the hue spectrum, the axes it is heard on, coarsest
// first.
//
//! Exact in ℚ and invariant under every versor `carry_cell` accepts: what a
//! body sounds like, whatever its size, place, frame or orientation. The
//! axes run coarsest first — the topology (components, tunnels, cavities,
//! turns), then the boundary and moment ratios, then the hue spectrum — so
//! a prefix of a chart is a stratum bodies share (A21 §2) and the atlas is
//! read by peel.

use crate::transducer::blade::{self, Blade};
use num_rational::BigRational;
use num_traits::Zero;

/// THE HARMONICS of a body, in axis order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Harmonics {
    /// h₀ — components (β₀).
    pub components: u64,
    /// h₁ — tunnels (β₁): the holes of a lamina.
    pub holes: u64,
    /// h₂ — cavities (β₂), enclosed by the stack along `e₃`.
    pub cavities: u64,
    /// h₃ — the laminae's turning count.
    pub turns: u64,
    /// h₄ — the in-plane boundary's length² over the cells (the
    /// isoperimetric ratio).
    pub isoperimetric: BigRational,
    /// h₅ — the sides open along `e₃` over the cells: two for one lamina,
    /// less for a stack.
    pub frame_boundary: BigRational,
    /// h₆ — det(I)/tr(I)² on the plane (anisotropy of the inertia).
    pub anisotropy: BigRational,
    /// h₇ — the three-coordinate trace over the cells² (spread per area).
    pub spread: BigRational,
    /// h₈ — the isotropic fourth moment over n³.
    pub quartic: BigRational,
    /// h₉… — the hue fractions, sorted: the gauge spectrum.
    pub gauge: Vec<BigRational>,
}

impl Harmonics {
    /// Hear a body.
    pub fn of(b: &Blade) -> Harmonics {
        let (components, holes, cavities) = blade::topology(&b.cells);
        let n = b.shell.clone();
        let count = |v: usize| BigRational::from_integer(num_bigint::BigInt::from(v as i64));
        let edges = count(blade::boundary_length(&b.cells));
        let open = count(blade::frame_faces(&b.cells));
        let (ixx, iyy, ixy, izz) = (&b.inertia[0], &b.inertia[1], &b.inertia[2], &b.inertia[3]);
        let tr = ixx + iyy;
        let det = ixx * iyy - ixy * ixy;
        let mut gauge: Vec<BigRational> = b.gauge.iter().map(|(_, c)| c / &n).collect();
        gauge.sort();
        Harmonics {
            components,
            holes,
            cavities,
            turns: blade::turns(&b.cells) as u64,
            isoperimetric: &edges * &edges / &n,
            frame_boundary: &open / &n,
            anisotropy: match tr.is_zero() {
                true => BigRational::zero(),
                false => &det / (&tr * &tr),
            },
            spread: (&tr + izz) / (&n * &n),
            quartic: &b.quartic[0] / (&n * &n * &n),
            gauge,
        }
    }

    /// The harmonics laid out as the atlas's axes. None if any ratio
    /// exceeds the axis (A6 refusal).
    pub fn axes(&self) -> Option<Vec<u64>> {
        let mut axes = vec![self.components, self.holes, self.cavities, self.turns];
        axes.extend(blade::axes_of(&self.isoperimetric)?);
        axes.extend(blade::axes_of(&self.frame_boundary)?);
        axes.extend(blade::axes_of(&self.anisotropy)?);
        axes.extend(blade::axes_of(&self.spread)?);
        axes.extend(blade::axes_of(&self.quartic)?);
        for f in &self.gauge {
            axes.extend(blade::axes_of(f)?);
        }
        Some(axes)
    }
}

/// HEAR a voice's carried body — one section's chain — lift it through
/// the eye and read its harmonics. None for an empty body.
pub fn hear(body: &[(i64, i64, u8, u8)]) -> Option<Harmonics> {
    let shape = crate::transducer::element::shape_of_chain(body)?;
    Some(Harmonics::of(&shape.blade))
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
        assert_eq!(a.components, 1);
        assert_eq!(a.turns, 6);
        assert_eq!(a.cavities, 0, "a lamina encloses nothing");
        assert_eq!(
            a.frame_boundary,
            BigRational::from_integer(2.into()),
            "a lamina is open on both sides along e₃"
        );
        assert_eq!(
            a.axes().expect("a measured harmonic reading has axes"),
            crate::transducer::stalk::invariant(&l).0
        );
    }

    /// 51.3 — THE STACK IS NOT ITS SHADOW. Three identical laminae read one
    /// component through their adjacency along `e₃`, a frame boundary
    /// smaller than a lamina's, and a spread the shadow does not carry;
    /// the same three laminae one frame apart read three components. A
    /// bar standing along `e₃` reads no chart a lamina reads.
    #[test]
    fn a_stack_reads_its_own_topology_and_not_its_shadows() {
        let lamina: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (0, 1, 0), (1, 0, 0)];
        let stack: Vec<(i64, i64, i64)> =
            (0..3).flat_map(|k| lamina.iter().map(move |&(r, c, _)| (r, c, k))).collect();
        let spaced: Vec<(i64, i64, i64)> =
            (0..3).flat_map(|k| lamina.iter().map(move |&(r, c, _)| (r, c, 2 * k))).collect();
        let one = Harmonics::of(&blade::shape_of(&lamina).blade);
        let three = Harmonics::of(&blade::shape_of(&stack).blade);
        let apart = Harmonics::of(&blade::shape_of(&spaced).blade);
        assert_eq!(three.components, 1, "consecutive frames join along e₃");
        assert_eq!(apart.components, 3, "frames apart do not");
        assert_eq!(three.turns, 3 * one.turns, "each lamina turns as it turns alone");
        assert!(three.frame_boundary < one.frame_boundary, "a stack is open on fewer sides");
        assert_ne!(three.spread, one.spread, "and its spread carries the offsets along e₃");
        assert_eq!(
            blade::shape_of(&stack).blade.inertia[3],
            BigRational::from_integer(6.into()),
            "Izz is the offsets alone: three laminae one frame from the middle"
        );
        let bar_along_k: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (0, 0, 1), (0, 0, 2)];
        let bar_in_plane: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (0, 1, 0), (0, 2, 0)];
        assert_ne!(
            blade::moduli(&blade::shape_of(&bar_along_k).blade),
            blade::moduli(&blade::shape_of(&bar_in_plane).blade),
            "a bar standing along e₃ is not a bar lying in a frame"
        );
        let hollow: Vec<(i64, i64, i64)> = (0..3)
            .flat_map(|r| (0..3).flat_map(move |c| (0..3).map(move |k| (r, c, k))))
            .filter(|&p| p != (1, 1, 1))
            .collect();
        assert_eq!(blade::topology(&hollow), (1, 0, 1), "a hollow cube encloses one cavity");
    }
}
