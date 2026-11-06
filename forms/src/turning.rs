// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE COMPOSITION OF TURNINGS (LAW-COORDINATE, proposal 78.173 fixture
//! 11b, as netstratum stated it 2026-11-06; netstratum docs/117 §11.2–§11.4),
//! exact in the even subalgebra of `Cl(p, q)` over ℚ, the signature read
//! from the data (`algebra::Clifford`).
//!
//! THE RATE IS THE GENERATOR. A motion's rate is an even generator `B`, a
//! bivector, its components exact on the planes and each plane's square
//! read from the units that name it. Its motion is the Cayley form
//!
//! ```text
//!     R = (1 + B)(1 − B)⁻¹        B = (R − 1)(R + 1)⁻¹
//! ```
//!
//! ([`motion`], [`generator`]), both exact: `1 + B` and `(1 − B)⁻¹` commute,
//! so the order of the product is immaterial, and the generator read back
//! from a motion is the generator it came from. No trigonometric value
//! enters and no rotor is named by its angle.
//!
//! MOTIONS COMPOSE AS `R₁R₂` ([`compose`]), never plane by plane. Two
//! planes that share a unit carry generators whose wedge is zero, so the
//! composite holds no volume: it gains a part on the third plane instead,
//! the turning between the two, whose sign depends on the order. The
//! grade-4 volume arises only from disjoint planes, and the composite keeps
//! it: scalar, generator and volume.
//!
//! The closed forms on one plane, and the degrees as a reading, stand in
//! [`reading`]. Every refusal is named ([`TurningRefusal`]): a generator
//! that is not one, a value that is not even, a motion that is not a unit,
//! and the two poles, `1 − B` or `R + 1` with no inverse (`R = −1` among
//! them).
//!
//! THE INVERSE HERE IS THE CAYLEY FORM'S ALONE (MU-RECURSION 78.32(1)):
//! nothing else in the forms inverts. It solves `u·y = 1` exactly in the
//! subalgebra the blades of `u` generate, so it holds in every signature
//! and at every `n`, never by a closed form fitted to five generators.

pub mod reading;

use crate::Q;
use algebra::{Clifford, SignatureRefusal};
use num_traits::{One, Zero};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the composition of turnings of LAW-COORDINATE.
pub const LAW_ID: &str = "FORMS-TURNING";

/// The form's version: an amendment, or an operation added to the form,
/// raises it.
/// FORMAT: the shell's version, the composition law of LAW-COORDINATE
/// fixture 11b (proposal 78.173 §5, netstratum's statement of 2026-11-06).
pub const LAW_VERSION: u16 = 1;

/// WHY THE COMPOSITION OF TURNINGS REFUSES, by name and with what was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurningRefusal {
    /// A rate holding parts that are not a generator's: the blade masks of
    /// grade other than 2 (mod 4), whose reverse is not their negation.
    NotGenerator { masks: Vec<usize> },
    /// A motion holding parts of odd grade: their blade masks.
    NotEven { masks: Vec<usize> },
    /// A motion `R` with `R·R̃ ≠ 1`: the terms of `R·R̃` as read.
    NotUnit { read: Vec<(usize, Q)> },
    /// The generator's pole: `1 − B` has no inverse, so `B` names no
    /// motion (a hyperbolic plane's `b = ±1`, among others).
    NoMotion,
    /// The motion's pole: `R + 1` has no inverse, so `R` has no generator
    /// (`R = −1`, among others).
    NoGenerator,
    /// Two values whose signatures disagree (the algebra's refusal).
    Mismatched(SignatureRefusal),
}

impl From<SignatureRefusal> for TurningRefusal {
    fn from(r: SignatureRefusal) -> TurningRefusal {
        TurningRefusal::Mismatched(r)
    }
}

/// The grade of a blade: how many generators its mask names.
fn grade(mask: usize) -> u32 {
    mask.count_ones()
}

/// The scalar `c` under the negative mask `negatives`.
fn scalar(negatives: usize, c: Q) -> Result<Clifford, TurningRefusal> {
    Ok(Clifford::blade(negatives, 0, c)?)
}

/// THE SAME VALUE: one signature and the same terms, each blade with its
/// exact coefficient.
pub fn same(a: &Clifford, b: &Clifford) -> bool {
    a.negatives() == b.negatives() && a.terms().eq(b.terms())
}

/// THE REVERSE `ũ`: a blade of grade `g` takes the sign `(−1)^{g(g−1)/2}`,
/// a free involution on the coordinates.
pub fn reversed(u: &Clifford) -> Result<Clifford, TurningRefusal> {
    let mut out = scalar(u.negatives(), Q::zero())?;
    for (mask, c) in u.terms() {
        let g = grade(mask);
        let c = match (g * (g + 1) / 2 - g) & 1 {
            0 => c.clone(),
            _ => -c.clone(),
        };
        out = out.sum(&Clifford::blade(u.negatives(), mask, c)?)?;
    }
    Ok(out)
}

/// THE BLADES OF THE SUBALGEBRA `u` GENERATES: the masks reached by
/// exclusive-or from `u`'s own, ascending (the masks themselves are the
/// canonical order). The product of two blades is a blade of the
/// exclusive-or of their masks, so this span is closed under the product
/// and holds `u⁻¹` wherever it exists.
fn span(u: &Clifford) -> Vec<usize> {
    let mut basis = vec![0usize];
    for (mask, _) in u.terms() {
        if !basis.contains(&mask) {
            let coset: Vec<usize> = basis.iter().map(|b| b ^ mask).collect();
            basis.extend(coset);
        }
    }
    basis.sort_unstable();
    basis
}

/// THE EXACT INVERSE OF `u` in the subalgebra its blades generate: the one
/// `y` with `u·y = 1`, solved over ℚ by elimination on the coefficients of
/// `u·e_g` for each blade `e_g` of the span. `None` where `u` is a zero
/// divisor there, which no exact inverse names.
fn inverse(u: &Clifford) -> Option<Clifford> {
    let negatives = u.negatives();
    let basis = span(u);
    let n = basis.len();
    let mut rows: Vec<Vec<Q>> = vec![vec![Q::zero(); n + 1]; n];
    for (j, g) in basis.iter().enumerate() {
        let column = u.product(&Clifford::blade(negatives, *g, Q::one()).ok()?).ok()?;
        for (mask, c) in column.terms() {
            let i = basis.binary_search(&mask).ok()?;
            *rows.get_mut(i)?.get_mut(j)? = c.clone();
        }
    }
    // The span's first blade is the scalar: the right side is `1`.
    *rows.get_mut(0)?.get_mut(n)? = Q::one();
    for col in 0..n {
        let pivot = (col..n).find(|&r| rows.get(r).and_then(|row| row.get(col)).is_some_and(|c| !c.is_zero()))?;
        rows.swap(col, pivot);
        let lead = rows.get(col)?.clone();
        let p = lead.get(col)?.clone();
        let lead: Vec<Q> = lead.iter().map(|c| c / &p).collect();
        for (r, row) in rows.iter_mut().enumerate() {
            let f = row.get(col)?.clone();
            if r == col || f.is_zero() {
                continue;
            }
            row.iter_mut().zip(&lead).for_each(|(x, l)| *x -= &f * l);
        }
        *rows.get_mut(col)? = lead;
    }
    let mut y = scalar(negatives, Q::zero()).ok()?;
    for (j, g) in basis.iter().enumerate() {
        let c = rows.get(j)?.get(n)?.clone();
        y = y.sum(&Clifford::blade(negatives, *g, c).ok()?).ok()?;
    }
    Some(y)
}

/// The masks of `u`'s terms whose grade `refused` names.
fn masks_where(u: &Clifford, refused: impl Fn(u32) -> bool) -> Vec<usize> {
    u.terms().map(|(m, _)| m).filter(|m| refused(grade(*m))).collect()
}

/// THE MOTION OF THE RATE `B`, `R = (1 + B)(1 − B)⁻¹`, exact. Refused
/// `NotGenerator` where `B` holds a part whose reverse is not its negation
/// (a grade other than 2 mod 4), and `NoMotion` where `1 − B` has no
/// inverse.
pub fn motion(rate: &Clifford) -> Result<Clifford, TurningRefusal> {
    let masks = masks_where(rate, |g| g % 4 != 2);
    if !masks.is_empty() {
        return Err(TurningRefusal::NotGenerator { masks });
    }
    let one = scalar(rate.negatives(), Q::one())?;
    let held = inverse(&one.difference(rate)?).ok_or(TurningRefusal::NoMotion)?;
    Ok(one.sum(rate)?.product(&held)?)
}

/// `R` read as a motion: even, and a unit, `R·R̃ = 1`; refused
/// `NotEven` with its odd blades or `NotUnit` with `R·R̃` as read.
fn unit(r: &Clifford) -> Result<(), TurningRefusal> {
    let masks = masks_where(r, |g| g % 2 == 1);
    if !masks.is_empty() {
        return Err(TurningRefusal::NotEven { masks });
    }
    let norm = r.product(&reversed(r)?)?;
    match same(&norm, &scalar(r.negatives(), Q::one())?) {
        true => Ok(()),
        false => Err(TurningRefusal::NotUnit { read: norm.terms().map(|(m, c)| (m, c.clone())).collect() }),
    }
}

/// THE GENERATOR OF THE MOTION `R`, `B = (R − 1)(R + 1)⁻¹`, exact: the rate
/// [`motion`] carries back to `R`. Since `R·R̃ = 1`, `B̃ = −B`: a
/// generator. Refused `NotEven` or `NotUnit` where `R` is not a motion,
/// and `NoGenerator` where `R + 1` has no inverse (`R = −1` among them).
pub fn generator(r: &Clifford) -> Result<Clifford, TurningRefusal> {
    unit(r)?;
    let one = scalar(r.negatives(), Q::one())?;
    let held = inverse(&r.sum(&one)?).ok_or(TurningRefusal::NoGenerator)?;
    Ok(r.difference(&one)?.product(&held)?)
}

/// TWO MOTIONS COMPOSED, `R₁R₂`: the product in the even algebra, whole,
/// never plane by plane. Each is read as a motion first (refused as
/// [`generator`] refuses `NotEven` and `NotUnit`), and their signatures
/// must agree.
pub fn compose(first: &Clifford, then: &Clifford) -> Result<Clifford, TurningRefusal> {
    unit(first)?;
    unit(then)?;
    Ok(first.product(then)?)
}

/// THE TWO RATES COMPOSED: the generator of `motion(B₁)·motion(B₂)`,
/// refused as [`motion`], [`compose`] and [`generator`] refuse.
pub fn compose_rates(first: &Clifford, then: &Clifford) -> Result<Clifford, TurningRefusal> {
    generator(&compose(&motion(first)?, &motion(then)?)?)
}
