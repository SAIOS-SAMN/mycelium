// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE Δ CORE — the exact-arithmetic types every live organ stands on.
//!
//! What remains of the convergence engine after the executor door was
//! excised (2026-09-03): the rational scalar, the antisymmetric relational
//! field Δ, and the operator that conjugates it. Nothing here walks a
//! residual, hashes an orbit, or runs a K-step; the transducer's Element
//! and the hypha's Session are those organs.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

/// `Q` — exact rational scalar. No floating point substitution permitted.
pub type Q = BigRational;

/// `QVec` — a rational vector (one entry of Δ, a fibre coordinate).
pub type QVec = Vec<Q>;

/// `QMatrix` — a row-major rational matrix (`matrix[i][j]` is entry (i,j)).
pub type QMatrix = Vec<Vec<Q>>;

/// The antisymmetric relational field: Δ_ij ∈ ℚ^m over n states.
///
/// Invariants, enforced on every write: Δ_ij = −Δ_ji and Δ_ii = 0.
#[derive(Debug, Clone, PartialEq)]
pub struct Delta {
    /// Number of states n; Δ is n×n.
    pub dim: usize,
    /// Coordinate dimension m of each entry.
    pub m: usize,
    /// Row-major storage: entry (i,j,l) = `entries[i][j][l]`.
    pub entries: Vec<Vec<QVec>>,
}

impl Delta {
    /// The zero field.
    pub fn zero(dim: usize, m: usize) -> Self {
        let zero_vec = vec![Q::zero(); m];
        let entries = vec![vec![zero_vec; dim]; dim];
        Delta { dim, m, entries }
    }

    /// Expand the fibre dimension m → m+1; the new coordinate starts empty.
    pub fn expand_fiber(&mut self) {
        for i in 0..self.dim {
            for j in 0..self.dim {
                self.entries[i][j].push(Q::zero());
            }
        }
        self.m += 1;
    }

    /// Entry Δ_ij.
    pub fn get(&self, i: usize, j: usize) -> &QVec {
        &self.entries[i][j]
    }

    /// Set Δ_ij = v and Δ_ji = −v. Panics on a nonzero diagonal.
    pub fn set_antisym(&mut self, i: usize, j: usize, v: QVec) {
        assert!(i < self.dim && j < self.dim, "index out of range");
        assert_eq!(v.len(), self.m, "dimension mismatch");
        if i == j {
            assert!(v.iter().all(|x| x.is_zero()), "Δ_ii must be 0");
        }
        let neg_v: QVec = v.iter().map(|x| -x).collect();
        self.entries[i][j] = v;
        self.entries[j][i] = neg_v;
    }

    /// Σ(Δ_ij + Δ_ji)² + Σ(Δ_ii)²: zero is exact conservation; the
    /// magnitude is the distance to it.
    pub fn conservation_measurement(&self) -> Q {
        let mut violation = Q::zero();
        for i in 0..self.dim {
            for l in 0..self.m {
                let d = &self.entries[i][i][l];
                violation += d * d;
            }
            for j in (i + 1)..self.dim {
                for l in 0..self.m {
                    let sum = &self.entries[i][j][l] + &self.entries[j][i][l];
                    violation += &sum * &sum;
                }
            }
        }
        violation
    }
}

/// N(Δ) = A · Δ · A⁻¹ for A ∈ GL_n^+(ℚ): the group action on the field.
#[derive(Debug, Clone)]
pub struct Operator {
    pub id: usize,
    /// A, row-major n×n.
    pub matrix: QMatrix,
    /// A⁻¹, held alongside.
    pub matrix_inv: QMatrix,
    /// det(A) > 0, verified on construction.
    pub det_sign_positive: bool,
}

impl Operator {
    /// Conjugate the field: (A·Δ·A⁻¹)_ij = Σ_{k,l} A_ik Δ_kl (A⁻¹)_lj,
    /// entry-wise over the fibre. None when A is not in the group.
    pub fn apply(&self, delta: &Delta) -> Option<Delta> {
        if !self.det_sign_positive {
            return None;
        }
        let n = delta.dim;
        let m = delta.m;
        assert_eq!(self.matrix.len(), n, "operator dimension mismatch");
        let mut result = Delta::zero(n, m);
        let mut intermediate = vec![vec![vec![Q::zero(); m]; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let a_ik = &self.matrix[i][k];
                    if a_ik.is_zero() {
                        continue;
                    }
                    for l in 0..m {
                        intermediate[i][j][l] += a_ik * &delta.entries[k][j][l];
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let ainv_kj = &self.matrix_inv[k][j];
                    if ainv_kj.is_zero() {
                        continue;
                    }
                    for l in 0..m {
                        result.entries[i][j][l] += ainv_kj * &intermediate[i][k][l];
                    }
                }
            }
        }
        Some(result)
    }
}

/// A rational from an integer.
pub fn q(n: i64) -> Q {
    Q::from(BigInt::from(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_field_stays_antisymmetric_under_writes_and_conjugation() {
        let mut d = Delta::zero(3, 2);
        d.set_antisym(0, 1, vec![q(5), q(-1)]);
        d.set_antisym(1, 2, vec![q(2), q(7)]);
        assert!(d.conservation_measurement().is_zero());
        d.expand_fiber();
        assert_eq!(d.m, 3);
        assert_eq!(d.get(2, 1), &vec![q(-2), q(-7), q(0)]);
        // A permutation of the states conjugates the field and keeps it antisymmetric.
        let p: QMatrix = vec![vec![q(0), q(1), q(0)], vec![q(0), q(0), q(1)], vec![q(1), q(0), q(0)]];
        let p_inv: QMatrix = vec![vec![q(0), q(0), q(1)], vec![q(1), q(0), q(0)], vec![q(0), q(1), q(0)]];
        let op = Operator { id: 0, matrix: p, matrix_inv: p_inv, det_sign_positive: true };
        let out = op.apply(&d).expect("in the group");
        assert!(out.conservation_measurement().is_zero());
        assert_eq!(out.get(0, 1), d.get(1, 2), "the permutation carries Δ_12 to Δ_01");
        let bad = Operator { det_sign_positive: false, ..op };
        assert!(bad.apply(&d).is_none(), "an orientation-reversing A is refused");
    }
}
