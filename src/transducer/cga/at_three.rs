//! THE CORPUS'S `n = 3`, NAMED FOR THE TESTS (PH2, BQ4). The production
//! embeddings take any `n` ([`super::point_at`], [`super::sphere_at`],
//! [`super::translator_at`], [`super::base_at`],
//! [`super::hyperbolic_compartment`]); a fixture that reads the corpus's three
//! coordinates — row, column, frame — says so by reading through here.

use super::Mv;
use num_rational::BigRational;

/// A point of the corpus's base.
pub fn point(x1: &BigRational, x2: &BigRational, x3: &BigRational) -> Mv {
    super::point_at(&[x1.clone(), x2.clone(), x3.clone()])
}

/// A sphere about a point of the corpus's base.
pub fn sphere(c1: &BigRational, c2: &BigRational, c3: &BigRational, r2: &BigRational) -> Mv {
    super::sphere_at(&[c1.clone(), c2.clone(), c3.clone()], r2)
}

/// A translation by a displacement on the corpus's base.
pub fn translator(d1: &BigRational, d2: &BigRational, d3: &BigRational) -> Mv {
    super::translator_at(&[d1.clone(), d2.clone(), d3.clone()])
}

/// A point read back on the corpus's three coordinates.
pub fn base(p: &Mv) -> Option<(BigRational, BigRational, BigRational)> {
    let x = super::base_at(p, 3)?;
    Some((x[0].clone(), x[1].clone(), x[2].clone()))
}

/// A compartment on the sheet at the corpus's three coordinates.
pub fn hyperbolic_compartment(r: &BigRational, c: &BigRational, k: &BigRational, m: &BigRational) -> Option<Mv> {
    super::hyperbolic_compartment(&[r.clone(), c.clone(), k.clone()], m)
}
