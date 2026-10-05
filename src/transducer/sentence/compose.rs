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
    /// Each term's whole motion, `transport · word`, in the sentence's order:
    /// what a lexicon reads a term's rung by (78.75).
    pub motions: Vec<Mv>,
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
    let rank = crate::transducer::cech::cayley_generators(&holonomies, budget)
        .and_then(|gens| crate::transducer::cech::lie_close(&[], &gens, budget).map(|b| b.len()));
    let motions = terms.iter().map(|t| t.transport.geo(&t.word)).collect();
    Some(Composition { omega, closure, carried, path, rank, motions })
}

/// 78.75 — A RING OF A FORM, READ AS A SENTENCE: each transition its word and
/// its spatial transport, `Term { word: A(w), transport: c }`, so the ring's
/// holonomy is the master equation's one product `Ω = ∏ cᵢ A(wᵢ)` (78.69).
///
/// A transition's transport holds its two halves apart — the word `w` (the
/// rotation, `versor_of_word`) and the carriage `c` (the displacement, `D·T`)
/// — and the circuit's composite multiplied the carriages alone, so every
/// word fell out of the holonomy: the two-field split, read as a product of
/// one half. Here each crossing is the one motion `c·A`; a contact's crossing,
/// whose carriage already holds its refraction whole, is that carriage alone
/// (`CarriedTransition::whole`); a crossing against
/// the face's orientation is its exact inverse `Ã c̃ = (Ã c̃ A)·Ã`, the word
/// `Ã` and the transport `Ã c̃ A`. `None` where a transition's transport was
/// not read (rule 11: unread is not flat) or its word has no base action.
pub(crate) fn terms_of(carried: &super::circuit::PathCarried) -> Option<Vec<Term>> {
    carried
        .transitions
        .iter()
        .map(|t| {
            let tr = t.transport.as_ref()?;
            // A contact's carriage is its whole refraction `θ·V` (BQ5a): the
            // crossing is that motion entire, its word a spelling beside it.
            if t.whole {
                let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
                return Some(match t.forward {
                    true => Term { word: one, transport: tr.carriage.clone() },
                    false => Term { word: one, transport: tr.carriage.reverse() },
                });
            }
            let a = crate::transducer::element::versor_of_word(&tr.word)?;
            cga::is_versor(&a).then_some(())?;
            Some(match t.forward {
                true => Term { word: a, transport: tr.carriage.clone() },
                false => {
                    let back = a.reverse();
                    Term { transport: back.geo(&tr.carriage.reverse()).geo(&a), word: back }
                }
            })
        })
        .collect()
}

/// THE SENTENCE'S HOLONOMY ALONE, `Ω = ∏ transport · word` in the terms'
/// order — [`compose`]'s `Ω`, for a reader that wants the product and not the
/// period or the rank.
pub(crate) fn omega_of(terms: &[Term]) -> Mv {
    terms.iter().fold(Mv::scalar(num_rational::BigRational::from_integer(1.into())), |omega, t| t.transport.geo(&t.word.geo(&omega)))
}

/// WHY A RING WAS NOT COMPOSED: its traversal refused (OR3's reason), a
/// transition's transport did not read, or (78.97) it crosses a chord and is
/// HEARD — a harmonic relation, never a sentence of rays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingUnread {
    Traversal,
    Transport,
    Heard,
}

/// 78.97 — A CHORD IS HEARD, NOT TRAVERSED (the operator, 2026-10-04:
/// "chords are harmonic - you cant "see" chords : you "see" rays through
/// luminal - our layering is incorrect"). A chord is the overlap between two
/// sections' PIECES — two copies of one piece, one spectrum sounded at two
/// places (AT1, ruled (a): a piece-chart is a meet chord's end and nothing
/// else). It is the harmonic eye's relation, an interval; nothing travels
/// along it, so it is no factor of a sentence's holonomy, which is composed
/// of what was seen — the luminal rays a body took. A ring that crosses one
/// is heard, never spoken.
pub fn is_chord(o: &crate::network::nerve::coord::Overlap) -> bool {
    use crate::network::nerve::coord::ChartAt;
    let piece = |c: &ChartAt| matches!(c, ChartAt::At(m) if m.piece.is_some());
    piece(&o.from) || piece(&o.to)
}

/// 78.75 — EVERY RING OF A FORM COMPOSED BY THE MASTER EQUATION: each ring's
/// one member path read as terms ([`terms_of`]) and composed ([`compose`]),
/// the overlaps it crosses kept beside it (what its faces carried is its
/// mass). The rings are the caller's: a percept's kept orbits and its vacua,
/// a court's re-derived ones.
pub fn sentences_of(
    form: &super::Form,
    rings: &[super::Orbit],
    budget: &mut crate::physics::Budget,
) -> Vec<(Vec<crate::network::nerve::coord::Overlap>, Result<Composition, RingUnread>)> {
    rings
        .iter()
        .map(|lp| {
            let steps: Vec<crate::network::nerve::coord::Overlap> = lp.transitions.iter().map(|s| s.overlap.clone()).collect();
            // 78.97 — a ring through a chord is heard, never composed.
            if steps.iter().any(is_chord) {
                return (steps, Err(RingUnread::Heard));
            }
            let read = match super::circuit::paths_carried_quiet(&form.faces, &form.vertices, lp) {
                None => Err(RingUnread::Traversal),
                Some(paths) => match paths.scoped().next() {
                    None => Err(RingUnread::Traversal),
                    Some((_, p)) => match terms_of(p) {
                        None => Err(RingUnread::Transport),
                        Some(terms) => compose(&terms, budget).ok_or(RingUnread::Transport),
                    },
                },
            };
            (steps, read)
        })
        .collect()
}

/// 78.69, 78.97, 78.68 — THE WORD A SENTENCE TEACHES, the one rule (78.100
/// D2) both [`next_rung`] and the mind's hearing read: a periodic theorem its
/// open path, one word for the whole path; a nilpotent theorem its holonomy
/// `Ω = 1 + N`, the one null motion per traversal (its open path is the
/// identity wherever the slip rides the closing transport); a named one — a
/// hyperbolic factor or a rosette, which never returns and does not
/// condense — its holonomy, held by name; a closed sentence with no
/// certificate its open path; an unread sentence nothing (rule 11).
pub fn teaches<'a>(s: &'a Composition, theorem: &Result<super::theorem::Theorem, super::theorem::Open>) -> Option<&'a Mv> {
    use super::theorem::{Certificate, Open, Theorem};
    match theorem {
        Ok(Theorem { certificate: Certificate::Periodic { .. }, .. }) => Some(&s.path),
        Ok(Theorem { certificate: Certificate::Nilpotent { .. }, .. }) | Err(Open::Rosette | Open::Unbound) => Some(&s.omega),
        Ok(Theorem { certificate: Certificate::Saturated { .. }, .. }) | Err(Open::Unread) => s.closure.closes().then_some(&s.path),
    }
}

/// THE NEXT RUNG: each sentence's taught word ([`teaches`]) — the shortcut
/// the relation teaches. A sentence that teaches nothing is its demand, not
/// a word. A word of the next rung composes with [`compose`] exactly as a
/// word of this one does.
pub fn next_rung(sentences: &[Composition]) -> Vec<Mv> {
    sentences.iter().filter_map(|s| teaches(s, &super::theorem::Theorem::of(s)).cloned()).collect()
}
