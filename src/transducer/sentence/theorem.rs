// 78.70 — THE THEOREM: a proof that extrapolates. A sentence's closure, read
// once, certified so that it holds for every n — the ellipsis closed by the
// algebra, never by enumeration.
//
//! A PROOF says one sentence closes (`compose`). A THEOREM says a whole
//! family does, and carries the finite certificate that makes the "for all"
//! exact. Only the algebra supplies such certificates, and there are four:
//!
//! - **Periodic.** `Ω^ℓ ~ 1` proves `Ω^m ~ Ω^{m mod ℓ}` for every `m`: the
//!   family of traversals is `ℤ/ℓ` (the winding carrier's law).
//! - **Nilpotent.** `Ω ∝ 1 + B` with `B² = 0` (a null, parabolic motion)
//!   proves `Ω^m ∝ 1 + m·B` for every `m`: length is a linear formula.
//! - **Saturated.** A Lie basis closed under the bracket proves every nested
//!   bracket of its members, of any depth, lies in its span.
//! - **Normal.** A relation `Ω ~ 1` proves `g Ω g̃ ~ 1` in every chart `g`
//!   carries it to: the same theorem seen from every chart.
//!
//! Where none applies — a rosette, a motion that never returns — no
//! certificate stands and the family is held open, its demand named (exact
//! or refuse). Checking a certificate is finite and exact, so a court can
//! adjudicate a theorem by re-checking it (78.69), never by re-running the
//! family.

use super::compose::{Closure, Composition};
use crate::transducer::cga::Mv;
use num_rational::BigRational;
use num_traits::{One, Zero};

/// WHAT MAKES THE FAMILY'S "FOR ALL" EXACT.
#[derive(Clone, Debug)]
pub enum Certificate {
    /// `Ω^ℓ ~ 1`: every power is the power mod ℓ.
    Periodic { ell: u64 },
    /// `Ω ∝ 1 + B`, `B² = 0`: every power is `1 + m·B`. `B` is held as the
    /// Cayley bivector `⟨Ω⟩₂ / ⟨Ω⟩₀`.
    Nilpotent { generator: Mv },
    /// A basis of a Lie algebra over ℚ, closed under the bracket: every
    /// nested bracket of its members lies in its span.
    Saturated { basis: Vec<Mv> },
}

/// A THEOREM: the holonomy a sentence (or a family of them) composes to,
/// and the certificate that extends one proof to every member of its
/// family.
#[derive(Clone, Debug)]
pub struct Theorem {
    pub omega: Mv,
    pub certificate: Certificate,
}

/// Why a proof extrapolates to no theorem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Open {
    /// A turn of infinite order: no period, no finite certificate.
    Rosette,
    /// A motion that never returns and is not nilpotent (a boost, a screw).
    Unbound,
    /// The motion's modes did not read.
    Unread,
}

/// A non-zero scalar: the value holds the scalar blade and no other.
fn identity_like(v: &Mv) -> bool {
    v.terms().map(|(k, _)| k).eq([0])
}

/// `Ω ∝ 1 + B` with `B² = 0` as a whole multivector: `B = ⟨Ω⟩₂ / ⟨Ω⟩₀`, no
/// grade beyond 2, and its square zero in every grade — not a norm, which
/// vanishes on null 4-vectors in this signature (78.68).
fn nilpotent_generator(omega: &Mv) -> Option<Mv> {
    let a = omega.coeff(0).clone();
    if a.is_zero() || !omega.grade(4).is_zero() || !omega.grade(1).is_zero() || !omega.grade(3).is_zero() {
        return None;
    }
    let b = omega.grade(2).scale(&(BigRational::one() / &a));
    (!b.is_zero() && b.geo(&b).is_zero()).then_some(b)
}

impl Theorem {
    /// THE THEOREM A PROOF EXTRAPOLATES TO: periodic where the sentence
    /// closed at ℓ, nilpotent where its holonomy is a null motion, or why
    /// neither stands.
    pub fn of(proof: &Composition) -> Result<Theorem, Open> {
        let omega = proof.omega.clone();
        if let Closure::Closes(ell) = proof.closure {
            return Ok(Theorem { omega, certificate: Certificate::Periodic { ell } });
        }
        if let Some(generator) = nilpotent_generator(&omega) {
            return Ok(Theorem { omega, certificate: Certificate::Nilpotent { generator } });
        }
        Err(match proof.closure {
            Closure::Rosette => Open::Rosette,
            Closure::Unbound => Open::Unbound,
            Closure::Closes(_) | Closure::Unread => Open::Unread,
        })
    }

    /// THE SATURATED THEOREM over a set of generators: the Lie closure of
    /// their Cayley bivectors, which certifies every nested bracket.
    pub fn saturated(
        holonomies: &[Mv],
        budget: &mut crate::physics::Budget,
    ) -> Result<Theorem, crate::transducer::cech::LieRankRefused> {
        let refs: Vec<Option<&Mv>> = holonomies.iter().map(Some).collect();
        let gens = crate::transducer::cech::cayley_generators(&refs, budget)?;
        let basis = crate::transducer::cech::lie_close(&[], &gens, budget)?;
        let omega = Mv::scalar(BigRational::one());
        Ok(Theorem { omega, certificate: Certificate::Saturated { basis } })
    }

    /// RE-CHECK THE CERTIFICATE — finite and exact, the court's whole work
    /// on a theorem (78.69): `Ω^ℓ ~ 1` by ℓ products; `B² = 0` and `Ω ∝ 1 + B`;
    /// every bracket of two basis members inside the span.
    pub fn check(&self, budget: &mut crate::physics::Budget) -> bool {
        match &self.certificate {
            Certificate::Periodic { ell } => {
                let mut p = self.omega.clone();
                for _ in 1..*ell {
                    if !budget.spend() {
                        return false;
                    }
                    p = p.geo(&self.omega);
                }
                *ell >= 1 && identity_like(&p)
            }
            Certificate::Nilpotent { generator } => nilpotent_generator(&self.omega).is_some_and(|b| b.sub(generator).is_zero()),
            // Closed under the bracket: every bracket of two members lies in
            // the span, one read of the budget per bracket. That finite check
            // is the whole of the infinite family of nested brackets.
            Certificate::Saturated { basis } => {
                for x in basis {
                    for y in basis {
                        if !budget.spend() || !self.holds(&x.geo(y).sub(&y.geo(x))) {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }

    /// THE FAMILY'S MEMBER AT `m`, BY THE CERTIFICATE AND NEVER BY `m`
    /// PRODUCTS: `Ω^{m mod ℓ}` (at most ℓ − 1 products); `1 + m·B` (none).
    /// `None` for a saturated theorem, which certifies brackets, not powers.
    pub fn at(&self, m: u64) -> Option<Mv> {
        let one = Mv::scalar(BigRational::one());
        match &self.certificate {
            Certificate::Periodic { ell } => {
                let r = m.checked_rem(*ell)?;
                Some((0..r).fold(one, |p, _| p.geo(&self.omega)))
            }
            Certificate::Nilpotent { generator } => {
                Some(one.add(&generator.scale(&BigRational::from_integer(m.into()))))
            }
            Certificate::Saturated { .. } => None,
        }
    }

    /// DOES THE SATURATED SPAN HOLD `x` — any nested bracket of the basis, of
    /// any depth, answers true; that is the ellipsis this certificate closes.
    /// A periodic or nilpotent theorem holds the powers [`Theorem::at`] reads.
    pub fn holds(&self, x: &Mv) -> bool {
        match &self.certificate {
            Certificate::Saturated { basis } => {
                if x.is_zero() {
                    return true;
                }
                let mut span = crate::core::span::Span::new();
                for b in basis {
                    span.join(&crate::transducer::cga::ray_of(b));
                }
                span.holds(&crate::transducer::cga::ray_of(x))
            }
            _ => false,
        }
    }

    /// NORMAL CLOSURE: the theorem seen from the chart `g` carries it to,
    /// `g Ω g̃` (a ray), its certificate carried with it — a period and a
    /// nilpotence are conjugation invariants, and a saturated basis is
    /// carried member by member.
    pub fn conjugated(&self, g: &Mv) -> Theorem {
        let carry = |v: &Mv| g.geo(v).geo(&g.reverse());
        let certificate = match &self.certificate {
            Certificate::Periodic { ell } => Certificate::Periodic { ell: *ell },
            Certificate::Nilpotent { .. } => match nilpotent_generator(&carry(&self.omega)) {
                Some(generator) => Certificate::Nilpotent { generator },
                // Conjugation by a non-versor is no change of chart; the
                // theorem stands where it was.
                None => return self.clone(),
            },
            Certificate::Saturated { basis } => Certificate::Saturated { basis: basis.iter().map(carry).collect() },
        };
        Theorem { omega: carry(&self.omega), certificate }
    }
}
