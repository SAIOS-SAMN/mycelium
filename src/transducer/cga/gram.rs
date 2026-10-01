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
//! - EACH PAIR'S REGIME, read in the algebra and never as a squared scalar
//!   (78.59: no flat square exponent — the δ² = G_ab²/(G_aa G_bb) this read
//!   is retired). The pair's blade `B = s_a ∧ s_b` squares in the geometric
//!   product: `B B > 0` — disjoint spheres, or a real sphere against an
//!   imaginary one not orthogonal — a boost; `B B = 0` tangent or parallel,
//!   parabolic; `B B < 0` the spheres cut, and the pair's rotor
//!   `V = s_a s_b` turns by twice their dihedral angle: its double-angle
//!   point `⟨V V⟩₀ / ⟨V Ṽ⟩₀ = cos 2φ` is rational, and the product has finite
//!   order `m` iff it is `−1, −½, 0, ½` (`m = 2, 3, 4, 6`); any other point a
//!   rosette. Every open regime is CERTAIN and read with no iteration;
//!   acceptance as finite needs a Coxeter system (GR2).

use super::multivector::Mv;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// WHAT ONE PAIR OF CONSTRUCTORS GENERATES.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairRegime {
    /// The pair's product has this finite order `m ∈ {2, 3, 4, 6}`.
    Finite(u8),
    /// `B B = 0`: tangent or parallel — a parabolic product, infinite.
    Parabolic,
    /// `B B > 0`: a boost, infinite.
    Boost,
    /// The spheres cut at a double-angle point off the crystallographic
    /// set: a turn of no finite order over ℚ, a rosette.
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
    /// Each pair `(a, b)`, `a < b`, with its regime, in order.
    pub pairs: Vec<(usize, usize, PairRegime)>,
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
        self.pairs.iter().all(|(_, _, r)| matches!(r, PairRegime::Finite(_)))
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

/// The constructors' rank as vectors: the grade of their span, each read as
/// the ray it is (`core::span`).
pub fn vector_rank(constructors: &[Mv]) -> usize {
    crate::core::span::grade_of(&constructors.iter().map(super::linear::ray_of).collect::<Vec<_>>())
}

/// ONE PAIR'S REGIME, in the algebra (78.59): the blade's geometric square,
/// then the rotor's double-angle point. No scalar is squared.
pub fn pair_regime(constructors: &[Mv], a: usize, b: usize) -> PairRegime {
    let (sa, sb) = (&constructors[a], &constructors[b]);
    if sa.geo(sa).scalar_part().is_zero() || sb.geo(sb).scalar_part().is_zero() {
        return PairRegime::Null;
    }
    let rotor = sa.geo(sb);
    // ORTHOGONAL: the rotor is a pure bivector, so `V V` is a scalar and the
    // product is of order two whatever the sign of the blade's square — a
    // real sphere against an imaginary one included.
    if rotor.scalar_part().is_zero() {
        return PairRegime::Finite(2);
    }
    let blade = rotor.grade(2);
    let bb = blade.geo(&blade).scalar_part();
    if bb.is_positive() {
        return PairRegime::Boost;
    }
    if bb.is_zero() {
        return PairRegime::Parabolic;
    }
    let turn = rotor.geo(&rotor).scalar_part() / rotor.geo(&rotor.reverse()).scalar_part();
    let half = |n: i64| Q::new(n.into(), 2.into());
    match () {
        _ if turn == half(-2) => PairRegime::Finite(2),
        _ if turn == half(-1) => PairRegime::Finite(3),
        _ if turn.is_zero() => PairRegime::Finite(4),
        _ if turn == half(1) => PairRegime::Finite(6),
        _ => PairRegime::Rosette,
    }
}

/// THE GRAM FORM'S READING of a configuration of constructors (GR1).
pub fn read_gram(constructors: &[Mv]) -> GramReading {
    let g = gram_form(constructors);
    let pairs = (0..constructors.len())
        .flat_map(|a| ((a + 1)..constructors.len()).map(move |b| (a, b)))
        .map(|(a, b)| (a, b, pair_regime(constructors, a, b)))
        .collect();
    GramReading { signature: signature(&g), rank: vector_rank(constructors), pairs }
}
