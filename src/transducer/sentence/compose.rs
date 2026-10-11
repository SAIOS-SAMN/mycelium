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
    /// 78.105 N1 — EACH TERM'S WORD `Wᵢ`, read apart from its SPACE `gᵢ`
    /// (the transport to the next word, the non-verbal line), in the
    /// sentence's order: what a lexicon reads a term's rung by. They were
    /// one product, `gᵢ · Wᵢ`, and a held word fused with the space around
    /// it was never recognized. The spaces are the terms' transports; their
    /// readers fold into the encounter and production (N5, N6).
    pub words: Vec<Mv>,
    /// 78.112 H6 — EACH TERM'S SPACE `gᵢ`, the transport to the next word, in
    /// the sentence's order: the rays the ring was carried on, which hearing
    /// meets its harmonic with.
    pub spaces: Vec<Mv>,
    /// `Ω`'s modes as the closure read them (MR1); empty where `Ω` is the
    /// identity or its modes did not read.
    pub modes: Vec<cga::MotionMode>,
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
    let identity = omega.terms().map(|(k, _)| k).eq([0]);
    let (closure, modes) = match identity {
        true => (Closure::Closes(1), Vec::new()),
        false => match cga::motion_modes(&omega, budget) {
            Some(Ok(modes)) => (
                match cga::period_of(&modes) {
                    Some(Period::Closes(l)) => Closure::Closes(l),
                    Some(Period::Rosette) => Closure::Rosette,
                    Some(Period::Unbound) => Closure::Unbound,
                    None => Closure::Unread,
                },
                modes,
            ),
            _ => (Closure::Unread, Vec::new()),
        },
    };
    // 78.104 — THE ALGEBRA OF THE SENTENCE IS ITS WORDS' AND ITS HOLONOMY'S.
    // It closed over the carried words alone, and since 78.97 a seen edge is
    // a ray whose word is the identity — its motion rides the transport — so
    // every perceived sentence read rank 0 (every voice, every epoch). `Ω`
    // is what the ring's transports composed to (Ambrose–Singer: the
    // holonomies generate the algebra), so it closes beside the words.
    let holonomies: Vec<Option<&Mv>> = carried.iter().chain(std::iter::once(&omega)).map(Some).collect();
    let rank = crate::transducer::cech::cayley_generators(&holonomies, budget)
        .and_then(|gens| crate::transducer::cech::lie_close(&[], &gens, budget).map(|b| b.len()));
    let words = terms.iter().map(|t| t.word.clone()).collect();
    let spaces = terms.iter().map(|t| t.transport.clone()).collect();
    Some(Composition { omega, closure, carried, path, rank, words, spaces, modes })
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
/// `Ã` and the transport `Ã c̃ A`. Refused by why (78.100) where a
/// transition's transport was not read (rule 11: unread is not flat), its
/// word has no base action, or what it gives is no versor — the first
/// crossing that does not read names the ring's reason.
pub(crate) fn terms_of(carried: &super::circuit::PathCarried) -> Result<Vec<Term>, TermUnread> {
    term_choices(carried)?
        .into_iter()
        .map(|mut choice| match (choice.pop(), choice.is_empty()) {
            (Some(term), true) => Ok(term),
            // Several candidates and none chosen (checklist (r)): read by the
            // composer that keeps every route ([`sentences_of`]).
            _ => Err(TermUnread::HeldCoset),
        })
        .collect()
}

/// 78.6, 78.102 — EACH CROSSING'S TERMS: one for a crossing with its
/// transport, one for a STATIC contact (its tether `V`, the whole motion at
/// one frame, the refraction's gauge half unread by name — 78.6(2)), and one
/// per candidate for a HELD COSET (`θ·H`, never a representative — 78.6(1)).
/// A contact with no refraction at all is `Unrefracted`; a pair no rigid
/// versor carries is `NotRigid`.
fn term_choices(carried: &super::circuit::PathCarried) -> Result<Vec<Vec<Term>>, TermUnread> {
    let one = || Mv::scalar(num_rational::BigRational::from_integer(1.into()));
    // A contact's whole motion as a term, the way the circuit crosses it.
    let whole = |carriage: &Mv, forward: bool| match forward {
        true => Term { word: one(), transport: carriage.clone() },
        false => Term { word: one(), transport: carriage.reverse() },
    };
    carried
        .transitions
        .iter()
        .map(|t| {
            let Some(tr) = t.transport.as_ref() else {
                use super::Refraction;
                return match (&t.refraction, t.whole) {
                    (Some(Refraction::Static { tether, .. }), _) => Ok(vec![whole(tether, t.forward)]),
                    (Some(Refraction::Coset(candidates)), _) => {
                        Ok(candidates.iter().map(|c| whole(&c.carriage, t.forward)).collect())
                    }
                    (None, true) => Err(TermUnread::Unrefracted),
                    (None, false) => Err(edgeless(&t.face)),
                };
            };
            // A contact's carriage is its whole refraction `θ·V` (BQ5a): the
            // crossing is that motion entire, its word a spelling beside it.
            if t.whole {
                return Ok(vec![whole(&tr.carriage, t.forward)]);
            }
            let a = crate::transducer::element::versor_of_word(&tr.word).ok_or(TermUnread::NoBaseAction)?;
            if !cga::is_versor(&a) {
                return Err(TermUnread::NotAVersor);
            }
            Ok(vec![match t.forward {
                true => Term { word: a, transport: tr.carriage.clone() },
                false => {
                    let back = a.reverse();
                    Term { transport: back.geo(&tr.carriage.reverse()).geo(&a), word: back }
                }
            }])
        })
        .collect()
}

/// 78.103 — WHICH FACE LOST ITS EDGE, read off the overlap itself: a star
/// face, a frame step, or a pair of members.
fn edgeless(o: &crate::network::nerve::coord::Overlap) -> TermUnread {
    use crate::network::nerve::coord::ChartAt;
    match (&o.from, &o.to) {
        (ChartAt::Frame, _) | (_, ChartAt::Frame) => TermUnread::StarUnread,
        // One vertex's own step along e₃ — its chart kept, or (AT1 A2) across
        // two of its charts — never a pair of members.
        (ChartAt::At(a), ChartAt::At(b)) if a.vertex == b.vertex => TermUnread::StepUnread,
        _ => TermUnread::NotRigid,
    }
}

/// 78.6(1), 78.44 — EVERY ROUTE A HELD COSET OPENS, formed one at a time
/// under the stroke: each assignment of the crossings' candidates is one
/// sequence of terms, a read of the stroke per sequence beyond the first
/// (rule 10); `None` where the stroke is spent before the last.
fn routes(choices: &[Vec<Term>], budget: &mut crate::physics::Budget) -> Option<Vec<Vec<Term>>> {
    let mut out: Vec<Vec<Term>> = vec![Vec::with_capacity(choices.len())];
    for choice in choices {
        if let [only] = choice.as_slice() {
            for route in &mut out {
                route.push(only.clone());
            }
            continue;
        }
        let mut next = Vec::with_capacity(out.len() * choice.len());
        for route in &out {
            for term in choice {
                if !next.is_empty() && !budget.spend() {
                    return None;
                }
                let mut r = route.clone();
                r.push(term.clone());
                next.push(r);
            }
        }
        out = next;
    }
    Some(out)
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
    Transport(TermUnread),
    Heard,
}

/// 78.100 — WHY A CROSSING'S TERM DID NOT READ, by name: no transport was
/// read for the transition; its word has no base action (`versor_of_word`
/// refused it); or what the word gave is no versor. On epoch 124 every
/// unread ring of every voice was unread here — about 45% of what each mind
/// perceived as rings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TermUnread {
    /// A face between two bodies whose union no rigid versor carries (50.3's
    /// `NotRigid`): no edge was measured. 78.63 reads a rigidity as a symptom
    /// — only a pair no CONFORMAL motion carries is a tear. Since 78.103 a
    /// pair that moved differentially carries its relative motion, so this
    /// is a member pair whose edge did not read.
    NotRigid,
    /// 78.103 — a STAR face (one end the Frame) with no edge read.
    StarUnread,
    /// 78.103 — a FRAME STEP (one vertex's charts at two frames, its chart
    /// kept or — AT1 A2 — across two of its charts) whose motor spelled no
    /// edge.
    StepUnread,
    /// A contact face whose refraction `(θ_bθ_a⁻¹)·V` was refused (BQ5a): no
    /// one transport, no static tether, no held coset.
    Unrefracted,
    NoBaseAction,
    NotAVersor,
    /// 78.6(1) — a held coset's routes, where one sequence of terms was asked
    /// (`terms_of`); [`sentences_of`] composes every one.
    HeldCoset,
    /// A held coset's routes, the stroke spent before every one was formed.
    CosetUnbounded,
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
    let mut out = Vec::with_capacity(rings.len());
    for lp in rings {
        let steps: Vec<crate::network::nerve::coord::Overlap> = lp.transitions.iter().map(|s| s.overlap.clone()).collect();
        // 78.97 — a ring through a chord is heard, never composed.
        if steps.iter().any(is_chord) {
            out.push((steps, Err(RingUnread::Heard)));
            continue;
        }
        let Some(paths) = super::circuit::paths_carried(&form.faces, &form.vertices, lp) else {
            out.push((steps, Err(RingUnread::Traversal)));
            continue;
        };
        let Some((_, p)) = paths.scoped().next() else {
            out.push((steps, Err(RingUnread::Traversal)));
            continue;
        };
        let choices = match term_choices(p) {
            Ok(c) => c,
            Err(why) => {
                out.push((steps, Err(RingUnread::Transport(why))));
                continue;
            }
        };
        // 78.6(1), 78.44 — A HELD COSET'S RING IS EVERY ROUTE IT OPENS, each
        // composed and kept, never a representative chosen.
        let Some(routes) = routes(&choices, budget) else {
            out.push((steps, Err(RingUnread::Transport(TermUnread::CosetUnbounded))));
            continue;
        };
        for terms in routes {
            // A path of no crossings composes nothing: its traversal gave no
            // term.
            out.push((steps.clone(), compose(&terms, budget).ok_or(RingUnread::Traversal)));
        }
    }
    out
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
