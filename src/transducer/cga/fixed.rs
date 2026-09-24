//! 73.2(5), OR1 — THE CENTRE OF A MOTION: the flat or round its holonomy
//! fixes, from the fixed-point equation over ℚ, or refused by name.
//!
//! A versor `M` acts on the vectors of `Cl(n+1, 1)` by its twisted adjoint
//! `x ↦ M̂ x M̃` (78.28(1)) — the sandwich for an even `M`, the sandwich
//! negated for an odd one, which is what a reflection does — and a vector
//! it holds still AT ITS OWN WEIGHT `w = M M̃` satisfies
//!
//! ```text
//!     (L − w) x = 0 ,     L x = M̂ x M̃
//! ```
//!
//! Read with the plain sandwich, an odd motion's fixed set sat at `−w` and
//! this read the wrong kernel (VA1's d1): a reflection holds its mirror, and
//! the plain sandwich negates every vector of it.
//!
//! a linear system over ℚ in `n + 2` unknowns. Its kernel is the flat the
//! motion fixes: a rotation about a point holds that point and `e_∞`, a
//! turn about an axis holds the axis, a translation holds `e_∞` alone.
//!
//! HC1, 75 — A MOTION WHOSE FIXED POINTS SCALE (a dilation, a boost: one
//! hyperbolic mode) holds its two POLES at the eigenvalues `w·λ±`, where
//! `λ± = c ± √(c² − 1)` and `c = 2⟨M⟩₀²/w − 1` is the mode's own. They are
//! exact whenever `c² − 1` is a rational square — a dilation by `ρ` has
//! `λ± = ρ, 1/ρ` — and refused BY THAT NAME, `poles in ℚ(√(c² − 1))`, only
//! when it is not. A motion that scales in more than one mode is refused
//! until the per-mode split (MD1) reads each mode alone. OR1 refused every
//! scaling motion as "a root", including the rational ones (corrected here).
//!
//! A KERNEL VECTOR THAT IS NOT NULL IS A ROUND, not a point. Where `e_∞` is
//! held, a fixed sphere's whole pencil `S + t e_∞` is fixed and its null
//! member, the centre, is a fixed point. Where it is not, the sphere is held
//! and its centre is not: a boost on `e₁∧e₋` holds the unit sphere and moves
//! the origin. OR1 read the centre of any such vector as a fixed point.

use super::embedding::{base_at, e_inf};
use super::group::weight_of;
use super::multivector::{base_axis, one, Mv, EM, EP};
use num_rational::BigRational;
use num_traits::Zero;

/// ONE POLE OF A SCALING MOTION: a point of the base, or the point at
/// infinity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PoleAt {
    Point(Vec<BigRational>),
    Infinity,
}

impl PoleAt {
    fn label(&self) -> String {
        match self {
            PoleAt::Point(p) => format!("({})", p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            PoleAt::Infinity => "∞".into(),
        }
    }
}

/// WHAT A MOTION HOLDS STILL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Centre {
    /// A flat fixed at the motion's own weight, of `dim` independent
    /// vectors, holding a finite point of the base: the one it reads.
    Fixed { dim: usize, point: Vec<BigRational> },
    /// No finite point is fixed and `e_∞` is: a translation or a parabolic
    /// motion, whose centre is at infinity — a reading — with the `dim`
    /// vectors it holds (`e_∞` and the directions it does not move).
    AtInfinity { dim: usize },
    /// HC1 — ONE HYPERBOLIC MODE: its two poles, each with the ratio `λ` it
    /// is held at (`M P M̃ = w·λ·P`), `λ₊ = c + √(c² − 1)` and its inverse.
    /// The pencil the two poles span is the set of spheres whose inversion
    /// exchanges them; which one exchanges `M` with `M⁻¹` (the per-mode
    /// relator) is a conjecture until probed (75), and is not asserted.
    Poles { ratio: BigRational, at_ratio: PoleAt, at_inverse: PoleAt },
    /// One hyperbolic mode whose poles want a root: `c² − 1` is not a
    /// rational square. Refused by that name, with the discriminant.
    Irrational { disc: BigRational },
    /// The fixed points scale in more than one mode, or at eigenvalues no
    /// single mode names: the per-mode split is MD1's. Refused.
    Scales,
    /// The motion is not a versor, so it has no action to fix anything by.
    Unread,
}

impl Centre {
    pub fn label(&self) -> String {
        match self {
            Centre::Fixed { dim, point } => {
                let coords: Vec<String> = point.iter().map(|x| x.to_string()).collect();
                format!("fixed flat of {dim} through ({})", coords.join(", "))
            }
            Centre::AtInfinity { dim } => format!("at infinity (holds {dim})"),
            Centre::Poles { ratio, at_ratio, at_inverse } => format!(
                "poles {} at {ratio} and {} at {}",
                at_ratio.label(),
                at_inverse.label(),
                BigRational::from_integer(1.into()) / ratio
            ),
            Centre::Irrational { disc } => format!("refused: poles in Q(√{disc})"),
            Centre::Scales => "refused: the fixed points scale in more than one mode (MD1's split)".into(),
            Centre::Unread => "refused: not a versor".into(),
        }
    }
}

/// THE CENTRE OF `m` in the algebra of `n` base axes (at least the motion's
/// own): the kernel of `L − w` over ℚ, read as 73.2(5) asks.
pub fn centre_of(m: &Mv, n: u8) -> Centre {
    // The weight is the eigenvalue the fixed set is read at, never a test.
    let Some(w) = weight_of(m) else { return Centre::Unread };
    let n = n.max(m.n());
    let gens: Vec<usize> = (1..=usize::from(n)).map(base_axis).chain([EP, EM]).collect();
    // THE ACTION, column by column: the twisted image of each generator,
    // read on every generator.
    let images: Vec<Mv> = gens.iter().map(|&g| m.twisted(&Mv::blade(g, one()))).collect();
    // The vectors held at eigenvalue `eigen`: the kernel of `L − eigen`.
    let held_at = |eigen: &BigRational| -> Vec<Mv> {
        let rows: Vec<Vec<BigRational>> = gens
            .iter()
            .enumerate()
            .map(|(a, &ga)| {
                images
                    .iter()
                    .enumerate()
                    .map(|(b, img)| match a == b {
                        true => img.coeff(ga) - eigen,
                        false => img.coeff(ga).clone(),
                    })
                    .collect()
            })
            .collect();
        nullspace(rows, gens.len())
            .iter()
            .map(|v| gens.iter().zip(v).fold(Mv::zero(), |acc, (&g, x)| acc.add(&Mv::blade(g, x.clone()))))
            .collect()
    };
    let has_origin_weight = |x: &Mv| !(-x.geo(&e_inf()).scalar_part()).is_zero();
    let basis = held_at(&w);
    let holds_infinity = m.twisted(&e_inf()).sub(&e_inf().scale(&w)).is_zero();
    // A FINITE FIXED POINT. Where `e_∞` is held, any held vector with
    // weight on `e_o` is a round whose whole pencil `S + t e_∞` is held, so
    // its null member — the centre `base_at` reads — is a fixed point.
    // Where it is not, only a NULL held vector is a point; a round held
    // without `e_∞` holds its sphere and not its centre. The basis is in the
    // elimination's own order — its free columns, ascending by generator —
    // so the point read is the content's and not an arrival's (rule 15).
    let point = basis.iter().find_map(|x| {
        let is_point = has_origin_weight(x) && (holds_infinity || x.norm2().is_zero());
        match is_point {
            true => base_at(x, usize::from(n)),
            false => None,
        }
    });
    match (point, holds_infinity) {
        (Some(point), _) => return Centre::Fixed { dim: basis.len(), point },
        (None, true) => return Centre::AtInfinity { dim: basis.len() },
        (None, false) => {}
    }
    // HC1 — NO POINT AND NOT INFINITY AT THE WEIGHT: THE POLES OF ONE
    // HYPERBOLIC MODE, read at `w·λ±` when `c² − 1` is a rational square.
    if !super::spin::bivector_is_simple(m) {
        return Centre::Scales;
    }
    let s0 = m.scalar_part();
    let c = BigRational::from_integer(2.into()) * &s0 * &s0 / &w - one();
    let disc = &c * &c - one();
    if disc <= BigRational::zero() {
        return Centre::Scales;
    }
    let Some(root) = crate::transducer::blade::rational_sqrt(&disc) else {
        return Centre::Irrational { disc };
    };
    let ratio = &c + &root;
    // ONE POLE AT ONE EIGENVALUE: a null held vector with weight on `e_o` is
    // a point; `e_∞` held there is the pole at infinity.
    let pole_at = |lambda: &BigRational| -> Option<PoleAt> {
        let eigen = &w * lambda;
        let held = held_at(&eigen);
        if m.twisted(&e_inf()).sub(&e_inf().scale(&eigen)).is_zero() {
            return Some(PoleAt::Infinity);
        }
        held.iter().find_map(|x| match has_origin_weight(x) && x.norm2().is_zero() {
            true => base_at(x, usize::from(n)).map(PoleAt::Point),
            false => None,
        })
    };
    match (pole_at(&ratio), pole_at(&(one() / &ratio))) {
        (Some(at_ratio), Some(at_inverse)) => Centre::Poles { ratio, at_ratio, at_inverse },
        // The eigenvalues the mode names hold no pole: the motion scales in
        // a way one mode does not describe.
        _ => Centre::Scales,
    }
}

/// A BASIS OF THE KERNEL of a square matrix over ℚ, by Gauss–Jordan
/// elimination: the columns that take no pivot are the free coordinates,
/// and each gives one kernel vector. Exact, no division by zero, no root.
/// Any number of rows; the relative pose (FR1) solves and ranks with it, so
/// the tree holds one elimination.
pub fn nullspace(mut rows: Vec<Vec<BigRational>>, cols: usize) -> Vec<Vec<BigRational>> {
    let mut pivot_col: Vec<usize> = Vec::new();
    let mut r = 0usize;
    for c in 0..cols {
        // The elimination's own pivot: the first row at or below `r` that
        // holds this column, which is the algorithm and not a choice.
        let mut at = None;
        for (i, row) in rows.iter().enumerate().skip(r) {
            if !row[c].is_zero() {
                at = Some(i);
                break;
            }
        }
        let Some(p) = at else { continue };
        rows.swap(r, p);
        let lead = rows[r][c].clone();
        for x in rows[r].iter_mut() {
            *x = &*x / &lead;
        }
        let pivot_row = rows[r].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i == r || row[c].is_zero() {
                continue;
            }
            let f = row[c].clone();
            for (x, y) in row.iter_mut().zip(&pivot_row) {
                *x = &*x - &f * y;
            }
        }
        pivot_col.push(c);
        r += 1;
    }
    (0..cols)
        .filter(|c| !pivot_col.contains(c))
        .map(|free| {
            let mut v = vec![BigRational::zero(); cols];
            v[free] = one();
            for (row, &pc) in rows.iter().zip(&pivot_col) {
                v[pc] = -row[free].clone();
            }
            v
        })
        .collect()
}
