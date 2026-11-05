// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EQ1, 78.12 — TWO RAYS ARE ONE BY PROPORTION: the projective equality of
//! two multivectors, read by cross multiplication of their coefficients.

use crate::multivector::Mv;
use num_rational::BigRational;

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
    // Two rays are one where they hold the same blades and every coefficient
    // stands to its partner as the first does: the blades either holds are
    // the whole reading, and a blade one holds and the other does not breaks
    // the proportion.
    if a.term_count() != b.term_count() {
        return None;
    }
    let ((pa, ak), (pb, bk)) = (a.terms().next()?, b.terms().next()?);
    if pa != pb {
        return None;
    }
    let proportional = a.terms().zip(b.terms()).all(|((ma, x), (mb, y))| ma == mb && x * bk == ak * y);
    proportional.then(|| ak / bk)
}
