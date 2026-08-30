// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! UNIFIED CONFORMAL MOTOR EXTRACTION — one algebraic solve, no per-operation
//! branches. Given source/target blades, extract the single motor
//! `M = D(λ)·T(d)·R` ∈ (D4 ⋉ ℤ² × ℚˣ) from the geometry itself:
//!   λ  from the inertia-trace ratio (Tr(I_B)/Tr(I_A))^{1/4}, exact ℚ or refuse;
//!   R  the D4 element aligning the inertia tensors (I_B = λ⁴ R I_A Rᵀ, exact);
//!   d  = c_B − λ R c_A.
//! Translation/rotation/scale are cases the math returns — never constructors the
//! implementor hand-picks. Any non-rational/non-D4 component ⇒ None (refuse: the
//! residual becomes a PIVOT for the germination loop, not a wrong answer).

use crate::transducer::blade::Blade;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}

/// The eight planar isometries of D4 as exact integer actions on `(row, col)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum D4 {
    E,
    R90,
    R180,
    R270,
    FlipRow,
    FlipCol,
    Transpose,
    AntiTranspose,
}

impl D4 {
    pub const ALL: [D4; 8] = [
        D4::E, D4::R90, D4::R180, D4::R270,
        D4::FlipRow, D4::FlipCol, D4::Transpose, D4::AntiTranspose,
    ];

    pub fn matrix(&self) -> [[i64; 2]; 2] {
        match self {
            D4::E => [[1, 0], [0, 1]],
            D4::R90 => [[0, -1], [1, 0]],
            D4::R180 => [[-1, 0], [0, -1]],
            D4::R270 => [[0, 1], [-1, 0]],
            D4::FlipRow => [[-1, 0], [0, 1]],
            D4::FlipCol => [[1, 0], [0, -1]],
            D4::Transpose => [[0, 1], [1, 0]],
            D4::AntiTranspose => [[0, -1], [-1, 0]],
        }
    }

    pub fn apply(&self, r: &BigRational, c: &BigRational) -> (BigRational, BigRational) {
        let m = self.matrix();
        (q(m[0][0]) * r + q(m[0][1]) * c, q(m[1][0]) * r + q(m[1][1]) * c)
    }
}

/// A factored exact similarity `M = D(λ)·T(t)·R`; `apply` = `λ·R·x + t`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformalMotor {
    pub dilation: BigRational,
    pub rotation: D4,
    pub translation: (BigRational, BigRational),
}

impl ConformalMotor {
    pub fn apply_point(&self, r: &BigRational, c: &BigRational) -> (BigRational, BigRational) {
        let (rr, cc) = self.rotation.apply(r, c);
        (&self.dilation * &rr + &self.translation.0, &self.dilation * &cc + &self.translation.1)
    }
    pub fn is_identity(&self) -> bool {
        self.rotation == D4::E && self.dilation == q(1) && self.translation.0.is_zero() && self.translation.1.is_zero()
    }
}

fn isqrt(n: &BigInt) -> Option<BigInt> {
    if n.is_negative() {
        return None;
    }
    if n.is_zero() {
        return Some(BigInt::from(0));
    }
    let two = BigInt::from(2);
    let mut x = n.clone();
    let mut y = (&x + BigInt::from(1)) / &two;
    while y < x {
        x = y.clone();
        y = (&x + n / &x) / &two;
    }
    (&x * &x == *n).then_some(x)
}

/// Exact rational square root, or None when irrational (the refusal that keeps
/// scales rational — an irrational scale is a pivot, not a value).
pub fn rational_sqrt(r: &BigRational) -> Option<BigRational> {
    if r.is_negative() {
        return None;
    }
    Some(BigRational::new(isqrt(r.numer())?, isqrt(r.denom())?))
}

// R I Rᵀ for symmetric 2×2 inertia [[Ixx,Ixy],[Ixy,Iyy]].
fn conj_inertia(i: &[[BigRational; 2]; 2], d: &D4) -> [[BigRational; 2]; 2] {
    let m = d.matrix();
    let mq = [[q(m[0][0]), q(m[0][1])], [q(m[1][0]), q(m[1][1])]];
    let mi = [
        [&mq[0][0] * &i[0][0] + &mq[0][1] * &i[1][0], &mq[0][0] * &i[0][1] + &mq[0][1] * &i[1][1]],
        [&mq[1][0] * &i[0][0] + &mq[1][1] * &i[1][0], &mq[1][0] * &i[0][1] + &mq[1][1] * &i[1][1]],
    ];
    [
        [&mi[0][0] * &mq[0][0] + &mi[0][1] * &mq[0][1], &mi[0][0] * &mq[1][0] + &mi[0][1] * &mq[1][1]],
        [&mi[1][0] * &mq[0][0] + &mi[1][1] * &mq[0][1], &mi[1][0] * &mq[1][0] + &mi[1][1] * &mq[1][1]],
    ]
}

/// THE UNIFIED SOLVE. One motor from the blade geometry; None if the motion is
/// not in the discrete conformal group (the residual is then a pivot). Verified
/// by exact centroid reconstruction (topological closure), not byte-replay.
pub fn solve_conformal_motor(a: &Blade, b: &Blade) -> Option<ConformalMotor> {
    if a.shell.is_zero() {
        return None;
    }
    // λ from the AREA (cell-count) ratio: λ² = |B|/|A|, exact ℚ or refuse. This
    // is the topologically-exact scale; the continuum Tr(I)∝λ⁴ relation does not
    // hold on discrete grids (a pixel scaling is generally not an exact motor —
    // it refuses, becoming a pivot). Inertia below only SELECTS the rotation.
    let lambda = rational_sqrt(&(&b.shell / &a.shell))?;
    let l2 = &lambda * &lambda;
    let l4 = &l2 * &l2;
    let ia = [[a.inertia[0].clone(), a.inertia[2].clone()], [a.inertia[2].clone(), a.inertia[1].clone()]];
    let ib = [[b.inertia[0].clone(), b.inertia[2].clone()], [b.inertia[2].clone(), b.inertia[1].clone()]];
    let (ca1, ca2) = a.centroid()?;
    let (cb1, cb2) = b.centroid()?;
    let is_isometry = lambda == q(1);

    // Bounded over the CLOSED D4 group (deduction, not search): keep the R whose
    // motor CLOSES topologically — for an isometry, cell-set coincidence (which
    // disambiguates R when the inertia is symmetric, e.g. the L-tromino); for a
    // dilation, body coincidence (inertia alignment + centroid), since the cells
    // cannot map one-to-one under a scale.
    for &rotation in D4::ALL.iter() {
        let c = conj_inertia(&ia, &rotation);
        let inertia_aligns = &l4 * &c[0][0] == ib[0][0] && &l4 * &c[0][1] == ib[0][1]
            && &l4 * &c[1][0] == ib[1][0] && &l4 * &c[1][1] == ib[1][1];
        if !inertia_aligns {
            continue;
        }
        let (rca1, rca2) = rotation.apply(&ca1, &ca2);
        let translation = (&cb1 - &lambda * &rca1, &cb2 - &lambda * &rca2);
        let motor = ConformalMotor { dilation: lambda.clone(), rotation, translation };
        let (p1, p2) = motor.apply_point(&ca1, &ca2);
        if p1 != cb1 || p2 != cb2 {
            continue; // centroid does not land
        }
        if is_isometry && !closes_cells(&motor, a, b) {
            continue; // an isometry must land every cell (topological coincidence)
        }
        return Some(motor);
    }
    None
}

/// Does the motor carry A's cell set exactly onto B's? Each image must be an
/// integer cell and the mapped set must equal B's (order-free). Only meaningful
/// for isometries (λ=1); a dilation changes the cell count and coincides at the
/// body level instead.
fn closes_cells(m: &ConformalMotor, a: &Blade, b: &Blade) -> bool {
    if a.cells.len() != b.cells.len() {
        return false;
    }
    let mut mapped: Vec<(i64, i64)> = Vec::with_capacity(a.cells.len());
    for (r, c) in &a.cells {
        let (pr, pc) = m.apply_point(&q(*r), &q(*c));
        if !pr.is_integer() || !pc.is_integer() {
            return false;
        }
        mapped.push((pr.to_integer().to_i64().unwrap_or(i64::MAX), pc.to_integer().to_i64().unwrap_or(i64::MAX)));
    }
    mapped.sort_unstable();
    let mut target = b.cells.clone();
    target.sort_unstable();
    mapped == target
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::blade::lift;

    /// One solve returns translation, rotation, and scale as CASES — no branch.
    #[test]
    fn the_solve_returns_each_transform_from_geometry() {
        // L-tromino and its cases.
        let l = |cells: &[(i64, i64, u8)]| lift(cells, 0);
        let a = l(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)]);

        // pure translation (+0,+3)
        let t = l(&[(0, 3, 1), (0, 4, 1), (1, 3, 1)]);
        let m = solve_conformal_motor(&a, &t).expect("translation");
        assert_eq!(m.dilation, q(1));
        assert_eq!(m.translation, (q(0), q(3)));

        // 180° rotation of the L about its own centroid region
        let r = l(&[(0, 1, 1), (1, 0, 1), (1, 1, 1)]);
        let mr = solve_conformal_motor(&a, &r).expect("a rigid rotation");
        assert_eq!(mr.dilation, q(1));
        assert_ne!(mr.rotation, D4::E, "rotation selected by cell closure, not a branch");
        assert!(super::closes_cells(&mr, &a, &r), "the motor lands every cell");
    }

    /// A discrete pixel scaling (2×2 → 4×4) is NOT an exact motor — it refuses
    /// (a pivot for germination), because a filled 4×4 is not the rigid image of
    /// a 2×2 under any D4⋉ℤ²×ℚˣ element (the cells cannot map).
    #[test]
    fn a_discrete_scaling_refuses_as_a_pivot() {
        let s1 = lift(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)], 0);
        let s2 = lift(
            &(0..4).flat_map(|r| (0..4).map(move |c| (r, c, 1u8))).collect::<Vec<_>>(),
            0,
        );
        assert!(solve_conformal_motor(&s1, &s2).is_none(), "λ=2 but no exact motor — pivot");
    }

    /// A non-similar shape change refuses — the residual is a pivot.
    #[test]
    fn a_non_group_motion_refuses() {
        let a = lift(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)], 0);
        let b = lift(&[(0, 0, 1), (0, 1, 1), (0, 2, 1)], 0); // 1×3 bar
        assert!(solve_conformal_motor(&a, &b).is_none(), "not a similarity — refused");
    }
}
