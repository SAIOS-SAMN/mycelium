// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.2 step 2, 1.5, law 12 — THE DERIVATION: what a word vertex is.
//!
//! A word vertex does not carry a spelling. It carries the generators it
//! factored through — each cited by the cited form's address and the
//! address of the `Word` vertex inside it, with its exponent, in
//! application order — and the residue in letters, the part A22 peeled
//! from the identity because no held generator reached it.
//!
//! The word is `∏ g_i^{e_i} · r`. A vertex a mind spelled from the
//! identity alone is the case where the citation list is empty and the
//! residue is the whole word; that is the cipher's case, and it is a
//! boundary of the general one rather than the only one.
//!
//! **The composed ROTOR is a coordinate; the composed word is not**
//! (L14, 1.9). A spelling of the composite beside the citations is two
//! carriers for one meaning, and the one that goes stale is the one a
//! court replays. The rotor is not a spelling: 1.5 says the vertex key
//! IS the canonical rotor coordinates, and `Word::key()` reduces to
//! `rotor().key()`, so what the vertex carries is its own identity
//! written out once.
//!
//! It must be carried rather than resolved, and the reason is a proof
//! rather than a preference: a form's address is `form_vertex_key`,
//! computed by `from_element_form` at decode where no resolver exists.
//! Two minds with different derivations must reach ONE address (1.5,
//! 13.10), so the key must be the composite and not the citations. And
//! the deferral of 21 step 3 — a sentence citing an unruled generator is
//! docketed while the syllabus serves it — needs the sentence's own
//! address BEFORE the citation resolves. The court's replay
//! (`verify_with`) is what keeps the carried rotor honest: citations
//! that do not compose to it are a refusal at that vertex.
//!
//! A `Word` for a cited vertex is resolved through the crystal on the
//! mind and the ruled cover on the court (`word_with`). Everything that
//! only needs the action — the sandwich, the class, the chart, the key —
//! reads `rotor()` and resolves nothing.

use crate::transducer::address::Address;
use crate::transducer::alphabet::{Rotor, Word};

/// One cited generator: the form, the `Word` vertex inside it, and the
/// exponent it is applied with.
pub type Cited = (Address, Address, i64);

/// The derivation of a word vertex (50.2 step 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    /// The generators composed, in application order.
    pub generators: Vec<Cited>,
    /// What no held generator reached, in letters.
    pub residue: Word,
    /// The canonical rotor of `∏ g^e · r` (A1), as the speaker composed
    /// it. Written only when there is something to compose: a spelled
    /// word's rotor is its residue's, and writing it twice would be the
    /// two carriers 1.9 refuses. Boxed for the reason the carriage is
    /// boxed — every `Vertex::Word` of every form would carry the field
    /// whether or not it cites anything.
    composed: Option<Box<Rotor>>,
}

impl Derivation {
    /// A word spelled from the identity alone: A22's case, no citation.
    pub fn spelled(word: Word) -> Derivation {
        Derivation { generators: Vec::new(), residue: word, composed: None }
    }

    /// A word factored through held generators. `composed` is the rotor
    /// the speaker's composition reached; the court re-derives it from
    /// the citations and refuses a disagreement. An empty citation list
    /// is not a factorisation, whatever rotor is offered with it: the
    /// residue alone is then the whole word and carries its own rotor.
    pub fn factored(generators: Vec<Cited>, residue: Word, composed: Rotor) -> Derivation {
        match generators.is_empty() {
            true => Derivation::spelled(residue),
            false => Derivation { generators, residue, composed: Some(Box::new(composed)) },
        }
    }

    /// Compose a derivation from the generators' own words: the speaker's
    /// side of 50.2 step 2, where each cited word is in hand.
    pub fn compose(cited: &[(Address, Address, i64, Word)], residue: Word) -> Derivation {
        let mut w = Word::identity();
        for (_, _, e, g) in cited {
            w = w.compose(&power_of(g, *e));
        }
        let composed = w.compose(&residue).rotor();
        let generators = cited.iter().map(|(f, v, e, _)| (f.clone(), v.clone(), *e)).collect();
        Derivation::factored(generators, residue, composed)
    }

    /// THE ACTION THIS VERTEX IS. Its identity (1.5), its key, its class,
    /// its chart and its sandwich all read this and resolve nothing.
    pub fn rotor(&self) -> Rotor {
        match &self.composed {
            Some(r) => (**r).clone(),
            None => self.residue.rotor(),
        }
    }

    /// The word a cited vertex spells, resolved: the generators composed
    /// from what the reader holds, times the residue. `None` for a
    /// citation the reader cannot resolve — which is the deferral of 21
    /// step 3 at the caller, never a smaller word.
    ///
    /// Only a caller that needs LETTERS asks for this. The action is
    /// `rotor()` and costs no resolution.
    pub fn word_with<R>(&self, resolve: &R) -> Option<Word>
    where
        R: Fn(&Address, &Address) -> Option<Word>,
    {
        if self.is_spelled() {
            return Some(self.residue.clone());
        }
        let mut w = Word::identity();
        for (form, vertex, e) in &self.generators {
            w = w.compose(&power_of(&resolve(form, vertex)?, *e));
        }
        Some(w.compose(&self.residue).normal_form())
    }

    /// The word this vertex spells when it spells one on its own: the
    /// residue, and only when nothing is cited. A cited vertex answers
    /// `None` here and is resolved with [`Derivation::word_with`].
    pub fn word_local(&self) -> Option<&Word> {
        match self.is_spelled() {
            true => Some(&self.residue),
            false => None,
        }
    }

    /// A SPELLING OF THIS VERTEX'S ACTION, RECOMPUTED (50.2 step 2: "the
    /// spelling from the identity is recomputed from the derivation and
    /// never stored as the word"). The residue when nothing is cited;
    /// otherwise A22 run on the carried rotor, which is the same vertex
    /// by 1.5 and reached without resolving anything.
    ///
    /// This is what a FACE reads. A face is the coboundary of two
    /// placements (3.1) and must re-derive identically on the speaker and
    /// the court, so it is written from the action both hold and not from
    /// letters only one of them can resolve. `None` when the seventeen
    /// cannot spell the action — which is a demand, not a face.
    pub fn spelling(&self) -> Option<Word> {
        if let Some(w) = self.word_local() {
            return Some(w.clone());
        }
        crate::transducer::alphabet::factor_rotor(&self.rotor())?.ok()
    }

    /// No citation: the whole word is the residue A22 peeled.
    pub fn is_spelled(&self) -> bool {
        self.generators.is_empty()
    }

    /// Re-derive the product from the cited generators and compare it to
    /// the product carried, projectively (1.5). `resolve` answers the
    /// word of a cited `(form, vertex)`; a citation it cannot answer is
    /// not a failure of this check but an absence — the caller defers, as
    /// a `Glued` citation defers (21 step 3) — so it is reported apart.
    pub fn verify_with<R>(&self, resolve: &R) -> Verified
    where
        R: Fn(&Address, &Address) -> Option<Word>,
    {
        if self.is_spelled() {
            return Verified::Composes;
        }
        let mut w = Word::identity();
        for (form, vertex, e) in &self.generators {
            let Some(g) = resolve(form, vertex) else {
                return Verified::Unresolved { form: form.clone(), vertex: vertex.clone() };
            };
            w = w.compose(&power_of(&g, *e));
        }
        let got = w.compose(&self.residue).rotor();
        match got.projective_eq(&self.rotor()) {
            true => Verified::Composes,
            false => Verified::Differs,
        }
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) -> Option<()> {
        let n = u16::try_from(self.generators.len()).ok()?;
        out.extend_from_slice(&n.to_le_bytes());
        for (form, vertex, e) in &self.generators {
            crate::transducer::address::put_addr(out, form)?;
            crate::transducer::address::put_addr(out, vertex)?;
            out.extend_from_slice(&e.to_le_bytes());
        }
        self.residue.to_bytes(out);
        if n > 0 {
            self.rotor().to_bytes(out);
        }
        Some(())
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Derivation> {
        let n = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
        *pos += 2;
        // The loop's boundary is the tape's own bytes (rule 10): every
        // iteration consumes two length-prefixed addresses and an i64, and
        // a byte that is not there refuses the whole read. Nothing is
        // reserved on the strength of the count, so a malformed count
        // allocates nothing and decides nothing (rules 2, 11).
        let mut generators = Vec::new();
        for _ in 0..n {
            let form = crate::transducer::address::take_addr(b, pos)?;
            let vertex = crate::transducer::address::take_addr(b, pos)?;
            let e = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            generators.push((form, vertex, e));
        }
        let residue = Word::from_bytes(b, pos)?;
        if n == 0 {
            return Some(Derivation::spelled(residue));
        }
        let composed = Rotor::from_bytes(b, pos)?;
        Some(Derivation::factored(generators, residue, composed))
    }
}

/// How a derivation replayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verified {
    /// The citations compose to the product carried.
    Composes,
    /// They compose to something else: a refusal at this vertex.
    Differs,
    /// A cited generator the resolver does not hold: the deferral of
    /// 21 step 3, never a refusal.
    Unresolved { form: Address, vertex: Address },
}

/// `g^e` for an exact integer exponent. A zero exponent is the identity,
/// and a citation is never a multiplication by a scalar — the exponent
/// counts applications of a generator (1.8).
///
/// THE EXPONENT COMES OFF THE TAPE, so the loop that applies it must be
/// bounded by content and not by the number a stranger wrote (rule 10).
/// Two bounds, and the generator's own algebra says which:
///
/// - **Finite order `m`** (15.3, `Rotor::order`): `g^m` is the identity,
///   so `g^e = g^(e mod m)` exactly. On the seventeen letters `m ≤ 12`,
///   and a citation with exponent 2⁶² costs the same as one with 3.
/// - **Infinite order**: there is no period to reduce against, so the
///   bound is the carrier's own width — square and multiply over the 63
///   bits of `|e|`, which is the most an `i64` can say.
///
/// Neither bound is a limit on what a voice may cite. Both are exact.
pub fn power_of(g: &Word, e: i64) -> Word {
    let base = match e < 0 {
        true => g.inverse(),
        false => g.clone(),
    };
    let n = e.unsigned_abs();
    let n = match base.rotor().order() {
        // `g^m = 1`, so the exponent is read in ℤ/m: the period is a
        // conjugacy invariant of the generator, not a cap on the tape.
        Some(crate::transducer::alphabet::Order::Finite(m)) if m > 0 => n % u64::from(m),
        _ => n,
    };
    let mut w = Word::identity();
    let mut square = base;
    let mut rest = n;
    while rest > 0 {
        if rest & 1 == 1 {
            w = w.compose(&square);
        }
        rest >>= 1;
        if rest > 0 {
            square = square.compose(&square);
        }
    }
    w
}
