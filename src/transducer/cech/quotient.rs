// 78.104 — THE QUOTIENT READING: a motion's generators read against the
// algebra a mind holds — inside it, or outside it by their coset.
//
//! A static reading, one-way: what of a motion the held algebra `𝔥 ⊂
//! so(n+1,1)` already spans (the span of every held generator, closed under
//! the commutator by `lie_close`), and what it does not. It is NOT lift and
//! drag (78.46): those are the mutual forces between two motions in contact,
//! each perturbing the other, and this reading neither perturbs nor is
//! perturbed. A new motion's generators (`flow_generators`, root-free where
//! the Cayley chart reads):
//!
//! ```text
//!     B ∈ 𝔥            INSIDE — a composition of what is held
//!     B + 𝔥 ∈ 𝔤/𝔥      OUTSIDE — its class in the quotient: what nothing
//!                      held spans
//! ```
//!
//! THE OUTSIDE PART IS A COSET, HELD WHOLE. No complement of `𝔥` is chosen:
//! the Killing form of `so(4,1)` is degenerate on the null directions a seen
//! ray takes, so an orthogonal complement is no object there, and any other
//! complement is a choice (rule 4). The coset is the object, and its
//! coordinate is the one vector of it that is zero on every axis the span
//! leads on (`Span::residue`) — fixed by `𝔥` alone, exact, null-safe. Two
//! outside parts are one class exactly where their residues agree: they
//! differ by something held.

use crate::core::span::{Span, Vector};
use crate::transducer::cga::{self, Mv};

/// ONE GENERATOR OUTSIDE THE HELD ALGEBRA: the generator, and its coset's
/// coordinate in `𝔤/𝔥`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outside {
    pub generator: Mv,
    pub residue: Vector,
}

/// A MOTION READ AGAINST A HELD ALGEBRA: its generators inside it, and those
/// outside it by their cosets. Both empty for the identity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Quotient {
    pub inside: Vec<Mv>,
    pub outside: Vec<Outside>,
}

impl Quotient {
    /// Every generator lies in the held algebra.
    pub fn spanned(&self) -> bool {
        self.outside.is_empty()
    }
}

/// WHY A MOTION WAS NOT READ: it has no generators to read — odd, no
/// versor, or modes the stroke could not split (`flow_generators`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unread;

/// READ `motion` AGAINST THE ALGEBRA SPANNED BY `held` — a basis closed under
/// the commutator, as `lie_close` returns it. Each generator lies in the span
/// or carries its coset; read exactly, under the stroke only where the
/// motion's modes must split.
pub fn quotient(held: &[Mv], motion: &Mv, budget: &mut crate::physics::Budget) -> Result<Quotient, Unread> {
    let gens = super::flow_generators(motion, budget).ok_or(Unread)?;
    let n = held.iter().chain(&gens).fold(0u8, |n, g| n.max(g.n()));
    let span = Span::of(&held.iter().map(|g| cga::ray_of(&g.widened(n))).collect::<Vec<_>>());
    let mut out = Quotient::default();
    for g in gens {
        let residue = span.residue(&cga::ray_of(&g.widened(n)));
        match residue.is_empty() {
            true => out.inside.push(g),
            false => out.outside.push(Outside { generator: g, residue }),
        }
    }
    Ok(out)
}

/// THE HELD ALGEBRA GROWN BY A MOTION NOW HELD: `held` closed with
/// `motion`'s generators under the commutator. `None` where the motion has
/// no generators to read or the stroke ran out closing; the held basis then
/// stands as it was.
pub fn grow(held: &[Mv], motion: &Mv, budget: &mut crate::physics::Budget) -> Option<Vec<Mv>> {
    let gens = super::flow_generators(motion, budget)?;
    super::lie_close(held, &gens, budget).ok()
}
