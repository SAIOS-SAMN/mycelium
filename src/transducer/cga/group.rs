//! A4 — the rational conformal group `Spin(4,1)(ℚ)`: dilation, reflection,
//! and the closed-form tests of versorhood and projective equality.

use super::embedding::{e_inf, e_o, point_at};
use super::multivector::{one, Mv, E1};
use num_rational::BigRational;
use num_traits::Zero;

/// The bivector `E = e_o ∧ e_∞` (the dilation generator's plane).
fn e_plane() -> Mv {
    // e_o e_∞ = e_o·e_∞ + e_o∧e_∞ = −1 + E.
    e_o().geo(&e_inf()).add(&Mv::scalar(one()))
}

/// THE DILATOR by an exact rational ρ > 0, as an UNNORMALIZED versor
/// `(ρ+1) ± (ρ−1) E`: a positive scalar multiple of exp(±½ ln ρ E), so the
/// sandwich carries points on the cone to their ρ-scaled images up to the
/// homogeneous scale, with no logarithm and no root. The orientation of E is
/// settled against the embedding itself (the form that carries X(1,0,0) to
/// X(ρ,0,0)), never by a chosen sign.
pub fn dilator(rho: &BigRational) -> Mv {
    let e = e_plane();
    let plus = Mv::scalar(rho + one()).add(&e.scale(&(rho - one())));
    // Probed on the first base axis alone, which every base has: the
    // dilation's orientation does not depend on `n`.
    let target = point_at(std::slice::from_ref(rho));
    let probe = plus.sandwich(&point_at(&[one()]));
    match projective_ratio(&probe, &target) {
        Some(_) => plus,
        None => Mv::scalar(rho + one()).sub(&e.scale(&(rho - one()))),
    }
}

/// THE REFLECTOR across the plane `e1 = 0` (rows negated): the odd versor
/// `e1`, whose sandwich sends `X(x1, x2, x3)` to `X(−x1, x2, x3)` up to the
/// homogeneous scale.
pub fn reflector() -> Mv {
    Mv::blade(E1, one())
}

/// IS `m` A VERSOR — READ BY ITS ACTION (78.28(1) as amended), never by a
/// sum of squares. `m` is even or odd throughout; its twisted adjoint
/// `m̂ eᵢ m̃` carries every generator `eᵢ` of `Cl(n+1, 1)` to a vector, the
/// images independent (their span's grade is the generators' count,
/// `core::span`); AND it carries the pseudoscalar `I` to its
/// own line, `m̂ I m̃` a multiple of `I` read as a ray equality. `n + 3`
/// actions, no norm formed as a value.
///
/// The two clauses together are the Lipschitz condition exactly. The first
/// alone is not: it takes `m̃` where Lipschitz takes `m⁻¹`, and `1 + e₀₁ +
/// e₂₃` carries every generator to an independent vector while `m m̃ = 3 −
/// 2I` (VA1-reader's finding). Since `m̂ I m̃ = ±I (m m̃)`, and `I (m m̃)` is
/// pure top grade exactly when `m m̃` is a scalar, the second clause closes
/// it. `m m̃` a non-zero scalar and Study's `ad = bc` are theorems of this
/// reader, and the tests read them as a test OF it, never as it.
///
/// Independence refuses what a grade test alone would let through: a null
/// vector sends every generator onto one line. `1 + I` is refused by parity
/// alone — its action is `2x`, the identity's (MT1's test).
///
/// MT1 — A FACT OF THE ELEMENT: definite parity, and its motor's action
/// ([`super::Motor`]) `acts_as_versor` — the same three clauses on the same
/// `n + 3` images, the rank by the exact-rank reader. The action's fact is
/// named apart, and `1 + I` is the witness the two differ.
pub fn is_versor(m: &Mv) -> bool {
    m.parity().is_some() && super::Motor::new(m).vector_reading().acts_as_versor()
}

/// THE WEIGHT `⟨m m̃⟩₀` OF A VERSOR, READ — never the test, never carried
/// (78.28(2), (3)). `None` where the action says `m` is no versor. Its
/// readers are named: a scale-free class ratio read as a class (Axiom C —
/// `I₁/w`, a step's charge, a coset's order), the eigenvalue a motion's
/// fixed set is read at, and a scale that is itself content (the dilator's
/// ρ, the sheet's base point). A site that divides by it to compose,
/// compare or transport takes the reverse instead (VA1-inverse).
pub fn weight_of(m: &Mv) -> Option<BigRational> {
    is_versor(m).then(|| m.geo(&m.reverse()).scalar_part())
}

/// PROJECTIVE EQUALITY — TWO RAYS ARE EQUAL BY PROPORTION (EQ1, 78.12):
/// `a = λ b` for some `λ ∈ ℚ×`, read as proportionality of the two
/// coefficient vectors by cross multiplication, `a_i b_k = a_k b_i` against
/// one pivot slot `k`, exact; returns λ.
///
/// It read `a b̃` having a scalar part only, which holds for a blade and a
/// versor and FAILS for a non-simple bivector — `b b̃` carries a grade-4
/// part there, so such an element compared unequal to itself. Proportion is
/// the definition of one ray, for every element; no product is taken.
pub fn projective_ratio(a: &Mv, b: &Mv) -> Option<BigRational> {
    // Both in one algebra, the larger of the two, so every slot either
    // holds is read.
    let (a, b) = a.common(b);
    let pivot = (0..b.slot_count()).find(|&k| !b.coeff(k).is_zero())?;
    let (ak, bk) = (a.coeff(pivot), b.coeff(pivot));
    if ak.is_zero() {
        return None;
    }
    let proportional = (0..a.slot_count()).all(|i| a.coeff(i) * bk == ak * b.coeff(i));
    proportional.then(|| ak / bk)
}
