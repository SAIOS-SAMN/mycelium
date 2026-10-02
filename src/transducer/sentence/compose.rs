// 78.69 — THE SENTENCE AS COMPOSITION: words and the spatial transports
// between their charts, composed in order until the ring closes; the closed
// sentence a relation, and its open path a word of the next rung.
//
//! THE MASTER SENTENCE EQUATION. A sentence of `k` words is the
//! path-ordered product of each word `Wᵢ` and the transport `g_{i→i+1}` that
//! carries its chart onto the next word's (the relative carriage across the
//! face, 78.68's luminal propagator), the last transport carrying the last
//! word's chart back onto the first:
//!
//! ```text
//!   Ω(s) = g_{k→1} · W_k · g_{k−1→k} · W_{k−1} ⋯ g_{1→2} · W₁
//!   closure:  Ω(s)^ℓ ~ 1   (ℓ = 1 filled; finite ℓ congruent; none open)
//! ```
//!
//! DIRECTION IS THE ORDER, AND CONJUGATION CARRIES EVERY WORD INTO ONE CHART:
//! `W̄ᵢ = Γᵢ⁻¹ Wᵢ Γᵢ` with `Γᵢ = g_{i−1→i} ⋯ g_{1→2}` the transport from the
//! first chart to the i-th. In that one chart the words' Cayley bivectors
//! close under the commutator to the sentence's holonomy algebra, whose rank
//! is its curvature's dimension (78.11, `lie_close`). Every lane composes in
//! the one product — turns by tangent addition, translations by sum,
//! dilations by tanh addition, hue maps by permutation (§52.2) — and none is
//! summed against another.
//!
//! THE RUNG. A closed sentence is a relation of the presentation (§1, 37.7's
//! relations row): its open path `P = W_k · g_{k−1→k} ⋯ g_{1→2} · W₁` equals
//! the closing transport's inverse up to its period, so the sentence teaches
//! a shortcut — one word for the whole path. [`next_rung`] reads those
//! shortcuts as the words of the next rung, and a word of rung `n + 1`
//! composes into sentences exactly as a word of rung `n` does: the grammar is
//! this equation applied to its own closures.
//!
//! The sentence is the client's and is never held on a ledger (78.69); what
//! a court keeps is the generators these compositions run on.

use crate::transducer::cga::{self, Mv, Period};

/// ONE TERM OF A SENTENCE: a word, read as its motor, and the transport
/// from that word's chart onto the next word's (the last term's onto the
/// first's).
#[derive(Clone, Debug)]
pub struct Term {
    pub word: Mv,
    pub transport: Mv,
}

/// WHETHER THE SENTENCE CLOSED: at its period ℓ, the ring returns
/// (`Ω^ℓ ~ 1`); or it does not, and why — a mode that turns forever (a
/// rosette), one that never returns (hyperbolic or parabolic), or a motion
/// whose modes did not read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Closure {
    Closes(u64),
    Rosette,
    Unbound,
    Unread,
}

impl Closure {
    pub fn closes(self) -> bool {
        matches!(self, Closure::Closes(_))
    }
}

/// ONE SENTENCE, COMPOSED: its holonomy `Ω`, whether it closed, each word
/// carried into the first word's chart, its open path — the word it teaches
/// the next rung — and the rank of the algebra its carried words generate.
#[derive(Clone, Debug)]
pub struct Composition {
    pub omega: Mv,
    pub closure: Closure,
    pub carried: Vec<Mv>,
    pub path: Mv,
    pub rank: Result<usize, crate::transducer::cech::LieRankRefused>,
}

/// COMPOSE A SENTENCE (78.69): `Ω` path-ordered from the first word, each
/// word carried into the first chart by the transports before it, the open
/// path, the closure read off `Ω`'s own modes (MR1), and the carried words'
/// Lie rank. `None` for an empty sentence, which says nothing.
pub fn compose(terms: &[Term], budget: &mut crate::physics::Budget) -> Option<Composition> {
    if terms.is_empty() {
        return None;
    }
    let n = terms.iter().fold(0u8, |n, t| n.max(t.word.n()).max(t.transport.n()));
    let one = Mv::scalar(num_rational::BigRational::from_integer(1.into())).widened(n);
    // `Γ` — the transport from the first chart to the current word's;
    // `path` — the product so far without the last transport taken.
    let mut gamma = one.clone();
    let mut path = one.clone();
    let mut omega = one;
    let mut carried = Vec::with_capacity(terms.len());
    for t in terms {
        // W̄ᵢ = Γᵢ⁻¹ Wᵢ Γᵢ, as rays: the reverse stands for the inverse.
        carried.push(gamma.reverse().geo(&t.word).geo(&gamma));
        path = t.word.geo(&omega);
        omega = t.transport.geo(&path);
        gamma = t.transport.geo(&gamma);
    }
    // A holonomy that is a non-zero scalar is the identity ray: the ring
    // returned at once (ℓ = 1). Otherwise the period is read off its modes.
    use num_traits::Zero;
    let identity = !omega.coeff(0).is_zero() && (1..omega.slot_count()).all(|k| omega.coeff(k).is_zero());
    let closure = match identity {
        true => Closure::Closes(1),
        false => match cga::motion_modes(&omega, budget) {
            Some(Ok(modes)) => match cga::period_of(&modes) {
                Some(Period::Closes(l)) => Closure::Closes(l),
                Some(Period::Rosette) => Closure::Rosette,
                Some(Period::Unbound) => Closure::Unbound,
                None => Closure::Unread,
            },
            _ => Closure::Unread,
        },
    };
    let holonomies: Vec<Option<&Mv>> = carried.iter().map(Some).collect();
    let rank = crate::transducer::cech::cayley_generators(&holonomies)
        .and_then(|gens| crate::transducer::cech::lie_close(&[], &gens, budget).map(|b| b.len()));
    Some(Composition { omega, closure, carried, path, rank })
}

/// THE NEXT RUNG: each closed sentence's open path, read as one word — the
/// shortcut the relation teaches. A sentence that did not close teaches
/// nothing and is its demand, not a word. A word of the next rung composes
/// with [`compose`] exactly as a word of this one does.
pub fn next_rung(sentences: &[Composition]) -> Vec<Mv> {
    sentences.iter().filter(|s| s.closure.closes()).map(|s| s.path.clone()).collect()
}
