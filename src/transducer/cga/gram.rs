// 78.14, GR1 — THE CONSTRUCTORS' GRAM FORM: the shape of a configuration of
// constructors up to motion, and the certain refusals it gives with no
// iteration.
//
//! `G_ab = ⟨s_a s_b⟩₀`, rational, no root. It is invariant under every
//! isometry applied to all the constructors at once, so it reads SHAPE; the
//! meet reads where the round is (78.14). Two readings of it:
//!
//! - THE SPAN SIGNATURE `(p, q, z)` by congruent symmetric elimination over
//!   ℚ (Sylvester: the signature is the invariant, the pivots are not),
//!   beside the constructors' RANK as vectors. A singular Gram has two
//!   sources and only one is affine: `z − (r − (p + q))` zero directions come
//!   from linear dependence, and `r − (p + q)` from a null direction in the
//!   span's own metric — the degenerate, affine case.
//! - EACH PAIR'S REGIME by `δ² = G_ab² / (G_aa G_bb)`, the inversive strain's
//!   own carrier (65), law 11 untouched: the pair's product has finite order
//!   `m` iff `δ² ∈ {0, 1/4, 1/2, 3/4}` (`m = 2, 3, 4, 6`); `δ² = 1` is
//!   tangent or parallel, parabolic; `δ² > 1` or `δ² < 0` — disjoint spheres,
//!   or a real sphere against an imaginary one not orthogonal — a boost; any
//!   other value in `(0, 1)` a rosette. Every open regime is CERTAIN and read
//!   with no iteration; acceptance as finite needs a Coxeter system (GR2).

use super::multivector::Mv;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// WHAT ONE PAIR OF CONSTRUCTORS GENERATES.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairRegime {
    /// The pair's product has this finite order `m ∈ {2, 3, 4, 6}`.
    Finite(u8),
    /// `δ² = 1`: tangent or parallel — a parabolic product, infinite.
    Parabolic,
    /// `δ² > 1` or `δ² < 0`: a boost, infinite.
    Boost,
    /// `δ²` in `(0, 1)` off the crystallographic set: a turn of no finite
    /// order over ℚ, a rosette.
    Rosette,
    /// A constructor with `s² = 0` in the pair: it reflects nothing.
    Null,
}

/// THE GRAM FORM'S READING OF ONE CONFIGURATION.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GramReading {
    /// `(p, q, z)`: positive, negative and zero directions of the form.
    pub signature: (usize, usize, usize),
    /// The constructors' rank as vectors.
    pub rank: usize,
    /// Each pair `(a, b)`, `a < b`, with its `δ²` and regime, in order.
    pub pairs: Vec<(usize, usize, Q, PairRegime)>,
}

impl GramReading {
    /// Zero directions from a null direction in the span's metric: the
    /// degenerate — affine — part.
    pub fn degenerate(&self) -> usize {
        let (p, q, _) = self.signature;
        self.rank - (p + q)
    }

    /// Zero directions from linear dependence among the constructors.
    pub fn dependent(&self) -> usize {
        let (_, _, z) = self.signature;
        z - self.degenerate()
    }

    /// Whether every pair's product is of finite order.
    pub fn crystallographic(&self) -> bool {
        self.pairs.iter().all(|(_, _, _, r)| matches!(r, PairRegime::Finite(_)))
    }
}

/// `G_ab = ⟨s_a s_b⟩₀`.
pub fn gram_form(constructors: &[Mv]) -> Vec<Vec<Q>> {
    constructors.iter().map(|a| constructors.iter().map(|b| a.geo(b).scalar_part()).collect()).collect()
}

/// THE SIGNATURE OF A SYMMETRIC RATIONAL FORM, `(p, q, z)`, by congruent
/// symmetric elimination: a non-zero diagonal pivot where one stands; where
/// every standing diagonal is zero and an off-diagonal `a_ij` is not, row and
/// column `j` are added to `i` (a congruence, diagonal `2 a_ij`). The
/// pivots' signs are counted; the pivots themselves are not invariants.
pub fn signature(form: &[Vec<Q>]) -> (usize, usize, usize) {
    let mut a: Vec<Vec<Q>> = form.to_vec();
    let d = a.len();
    let (mut p, mut q) = (0usize, 0usize);
    let mut live: Vec<usize> = (0..d).collect();
    loop {
        let pivot = match live.iter().copied().find(|&i| !a[i][i].is_zero()) {
            Some(i) => i,
            None => {
                let pair = live.iter().flat_map(|&i| live.iter().map(move |&j| (i, j))).find(|&(i, j)| i != j && !a[i][j].is_zero());
                let Some((i, j)) = pair else { break };
                let row_j = a[j].clone();
                for (x, y) in a[i].iter_mut().zip(&row_j) {
                    *x = &*x + y;
                }
                for row in a.iter_mut() {
                    let v = &row[i] + &row[j];
                    row[i] = v;
                }
                i
            }
        };
        let pv = a[pivot][pivot].clone();
        match pv.is_positive() {
            true => p += 1,
            false => q += 1,
        }
        live.retain(|&x| x != pivot);
        let pivot_row = a[pivot].clone();
        for &r in &live {
            let f = &a[r][pivot] / &pv;
            for (x, y) in a[r].iter_mut().zip(&pivot_row) {
                *x = &*x - &f * y;
            }
        }
        for &c in &live {
            a[pivot][c] = Q::zero();
            a[c][pivot] = Q::zero();
        }
    }
    (p, q, d - p - q)
}

/// The constructors' rank as vectors, by the one elimination (`nullspace`).
pub fn vector_rank(constructors: &[Mv]) -> usize {
    let slots = constructors.iter().fold(0usize, |s, v| s.max(v.slot_count()));
    let rows: Vec<Vec<Q>> = constructors.iter().map(|v| (0..slots).map(|k| v.coeff(k).clone()).collect()).collect();
    slots - super::nullspace(rows, slots).len()
}

/// ONE PAIR'S REGIME off its `δ²`.
pub fn pair_regime(g: &[Vec<Q>], a: usize, b: usize) -> (Q, PairRegime) {
    let den = &g[a][a] * &g[b][b];
    if den.is_zero() {
        return (Q::zero(), PairRegime::Null);
    }
    let d2 = &g[a][b] * &g[a][b] / den;
    let quarter = |n: i64| Q::new(n.into(), 4.into());
    let regime = match () {
        _ if d2.is_zero() => PairRegime::Finite(2),
        _ if d2 == quarter(1) => PairRegime::Finite(3),
        _ if d2 == quarter(2) => PairRegime::Finite(4),
        _ if d2 == quarter(3) => PairRegime::Finite(6),
        _ if d2 == quarter(4) => PairRegime::Parabolic,
        _ if d2 > quarter(4) || d2.is_negative() => PairRegime::Boost,
        _ => PairRegime::Rosette,
    };
    (d2, regime)
}

/// THE GRAM FORM'S READING of a configuration of constructors (GR1).
pub fn read_gram(constructors: &[Mv]) -> GramReading {
    let g = gram_form(constructors);
    let pairs = (0..constructors.len())
        .flat_map(|a| ((a + 1)..constructors.len()).map(move |b| (a, b)))
        .map(|(a, b)| {
            let (d2, r) = pair_regime(&g, a, b);
            (a, b, d2, r)
        })
        .collect();
    GramReading { signature: signature(&g), rank: vector_rank(constructors), pairs }
}
