//! 73.2(5), OR1 — THE CENTRE OF A MOTION: the flat or round its holonomy
//! fixes, from the fixed-point equation over ℚ, or refused by name.
//!
//! A versor `M` acts on the vectors of `Cl(n+1, 1)` by `x ↦ M x M̃`, and a
//! vector it holds still AT ITS OWN WEIGHT `w = M M̃` satisfies
//!
//! ```text
//!     (L − w) x = 0 ,     L x = M x M̃
//! ```
//!
//! a linear system over ℚ in `n + 2` unknowns. Its kernel is the flat the
//! motion fixes: a rotation about a point holds that point and `e_∞`, a
//! turn about an axis holds the axis, a translation holds `e_∞` alone. A
//! motion whose fixed points SCALE — a dilation, a boost — holds them at an
//! eigenvalue other than `w`, a root of the characteristic polynomial, and
//! that is refused by name, never solved for (law 1, rule 11).

use super::embedding::{base_at, e_inf};
use super::group::is_versor;
use super::multivector::{base_axis, one, Mv, EM, EP};
use num_rational::BigRational;
use num_traits::Zero;

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
    /// Nothing is fixed at the weight: the fixed points scale, at an
    /// eigenvalue the equation would have to root. Refused.
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
            Centre::Scales => "refused: the fixed points scale (a root)".into(),
            Centre::Unread => "refused: not a versor".into(),
        }
    }
}

/// THE CENTRE OF `m` in the algebra of `n` base axes (at least the motion's
/// own): the kernel of `L − w` over ℚ, read as 73.2(5) asks.
pub fn centre_of(m: &Mv, n: u8) -> Centre {
    let Some(w) = is_versor(m) else { return Centre::Unread };
    let n = n.max(m.n());
    let gens: Vec<usize> = (1..=usize::from(n)).map(base_axis).chain([EP, EM]).collect();
    // THE MATRIX, column by column: the image of each generator, read on
    // every generator, less the weight on the diagonal.
    let images: Vec<Mv> = gens.iter().map(|&g| m.sandwich(&Mv::blade(g, one()))).collect();
    let rows: Vec<Vec<BigRational>> = gens
        .iter()
        .enumerate()
        .map(|(a, &ga)| {
            images
                .iter()
                .enumerate()
                .map(|(b, img)| match a == b {
                    true => img.coeff(ga) - &w,
                    false => img.coeff(ga).clone(),
                })
                .collect()
        })
        .collect();
    let kernel = nullspace(rows, gens.len());
    if kernel.is_empty() {
        return Centre::Scales;
    }
    let vector = |v: &[BigRational]| {
        gens.iter().zip(v).fold(Mv::zero(), |acc, (&g, x)| acc.add(&Mv::blade(g, x.clone())))
    };
    let basis: Vec<Mv> = kernel.iter().map(|v| vector(v)).collect();
    // A FINITE POINT is a kernel vector with weight on `e_o`, `−(x·e_∞) ≠ 0`;
    // its base point is read by `base_at`, which a multiple of `e_∞` added
    // to it does not move. The basis is in the elimination's own order —
    // its free columns, ascending by generator — so the point read is the
    // content's and not an arrival's (rule 15).
    let point = basis.iter().find_map(|x| match (-x.geo(&e_inf()).scalar_part()).is_zero() {
        true => None,
        false => base_at(x, usize::from(n)),
    });
    // THE FIXED SET HAS NO FINITE POINT. A motion that holds `e_∞` at its
    // weight — a translation, a parabolic motion — holds its centre at
    // infinity, and the planes it holds are the directions it does not
    // move. One that does not hold `e_∞` either scales its fixed points
    // (a dilation holds directions and moves both its poles by `ρ`).
    let holds_infinity = m.sandwich(&e_inf()).sub(&e_inf().scale(&w)).is_zero();
    match (point, holds_infinity) {
        (Some(point), _) => Centre::Fixed { dim: basis.len(), point },
        (None, true) => Centre::AtInfinity { dim: basis.len() },
        (None, false) => Centre::Scales,
    }
}

/// A BASIS OF THE KERNEL of a square matrix over ℚ, by Gauss–Jordan
/// elimination: the columns that take no pivot are the free coordinates,
/// and each gives one kernel vector. Exact, no division by zero, no root.
fn nullspace(mut rows: Vec<Vec<BigRational>>, cols: usize) -> Vec<Vec<BigRational>> {
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
