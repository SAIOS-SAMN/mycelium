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
    /// 78.166 1c — THE CONJUGATOR: where the vertex is a held word
    /// TRANSPORTED, `g · (∏ g_i^{e_i} · r) · g̃`, the rotor `g` that carries
    /// it. Evidence, never identity: the vertex's key is its rotor alone
    /// (1.5), so a conjugate cited this way and one spelled whole reach one
    /// address. `None` for every derivation that is not a transport.
    conjugator: Option<Box<Rotor>>,
}

impl Derivation {
    /// A word spelled from the identity alone: A22's case, no citation.
    pub fn spelled(word: Word) -> Derivation {
        Derivation { generators: Vec::new(), residue: word, composed: None, conjugator: None }
    }

    /// 78.166 1c — THIS DERIVATION TRANSPORTED by `g`: the vertex is
    /// `g · (what it cites) · g̃`. A held word conjugated is a relation of
    /// the dictionary, never a new letter. A spelled derivation cites
    /// nothing to transport and is returned as it is.
    pub fn conjugated(mut self, g: Rotor) -> Derivation {
        if !self.is_spelled() {
            self.conjugator = Some(Box::new(g));
        }
        self
    }

    /// The rotor this derivation is transported by, where it is one.
    pub fn conjugator(&self) -> Option<&Rotor> {
        self.conjugator.as_deref()
    }

    /// AN ADJOINED GENERATOR (37.7): cites nothing, spells nothing, and
    /// IS the rotor it carries. A letter named by content (law 6) with
    /// no escape byte and no place in the seventeen.
    ///
    /// It must carry the rotor in the rotor field and not in the
    /// residue: a `Word` travels as its LETTER TAPE, so a letterless
    /// word carrying only a cached rotor reaches the wire as the
    /// identity. That is why this is its own constructor and not
    /// `spelled(Word::formed(r))`.
    pub fn adjoined(rotor: Rotor) -> Derivation {
        Derivation { generators: Vec::new(), residue: Word::identity(), composed: Some(Box::new(rotor)), conjugator: None }
    }

    /// Whether this is AN ADJOINED GENERATOR (`Derivation::adjoined`): it
    /// cites nothing and carries its rotor in the rotor field — the identity
    /// rotor included, a motion letter's (78.68 row 8), whose whole action is
    /// on its vertex's carriage.
    pub fn is_adjoined(&self) -> bool {
        self.generators.is_empty() && self.composed.is_some()
    }

    /// A word factored through held generators. `composed` is the rotor
    /// the speaker's composition reached; the court re-derives it from
    /// the citations and refuses a disagreement. An empty citation list
    /// is not a factorisation, whatever rotor is offered with it: the
    /// residue alone is then the whole word and carries its own rotor.
    pub fn factored(generators: Vec<Cited>, residue: Word, composed: Rotor) -> Derivation {
        match generators.is_empty() {
            true => Derivation::spelled(residue),
            false => Derivation { generators, residue, composed: Some(Box::new(composed)), conjugator: None },
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
        let w = w.compose(&self.residue);
        // A TRANSPORTED vertex in letters is `g w g⁻¹`, where the alphabet
        // spells `g`; where it does not, there are no letters to give, and
        // the action stays `rotor()` (rule 11: never a smaller word).
        match &self.conjugator {
            None => Some(w.normal_form()),
            Some(g) => match crate::transducer::alphabet::factor_rotor(g) {
                Some(Ok(gw)) => Some(gw.compose(&w).compose(&gw.inverse()).normal_form()),
                _ => None,
            },
        }
    }

    /// The word this vertex spells when it spells one on its own: the
    /// residue, and only when nothing is cited. A cited vertex answers
    /// `None` here and is resolved with [`Derivation::word_with`].
    pub fn word_local(&self) -> Option<&Word> {
        match self.is_spelled() && self.composed.is_none() {
            true => Some(&self.residue),
            false => None,
        }
    }

    /// THE WORD THIS VERTEX CARRIES, resolving nothing: the residue when
    /// it spells, and `Word::formed(rotor)` when it is an ADJOINED
    /// generator — a word that carries an action and no letters (37.7).
    /// `None` only when the vertex CITES, which is the one case that
    /// needs a resolver.
    ///
    /// This is what a reader asking "what generator is at this vertex"
    /// gets. `word_local` alone answered `None` for a letter, so a mind
    /// holding one offered it to nobody and a court could not resolve a
    /// citation of it.
    pub fn word_carried(&self) -> Option<Word> {
        if let Some(w) = self.word_local() {
            return Some(w.clone());
        }
        match self.is_spelled() {
            true => Some(Word::formed(self.rotor())),
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
    /// The resolver answers a cited generator's ROTOR, not its letters
    /// (L14). Composition is by the geometric product, so an adjoined
    /// generator — which has no letters at all — composes exactly as a
    /// spelled one does, and a court can verify a citation of a letter.
    /// Composing words here could not: `Word::compose` concatenates
    /// letter tapes, so a letterless word composes to the identity and
    /// every citation of a letter would read as a disagreement.
    pub fn verify_with<R>(&self, resolve: &R) -> Verified
    where
        R: Fn(&Address, &Address) -> Option<Rotor>,
    {
        if self.is_spelled() {
            return Verified::Composes;
        }
        let mut r = Rotor::identity();
        for (form, vertex, e) in &self.generators {
            let Some(g) = resolve(form, vertex) else {
                return Verified::Unresolved { form: form.clone(), vertex: vertex.clone() };
            };
            let Some(p) = rotor_power(&g, *e) else {
                return Verified::Differs;
            };
            r = r.geo(&p);
        }
        let got = r.geo(&self.residue.rotor());
        // 78.166 1c — a transported vertex composes `g · ∏ · g̃`.
        let got = match &self.conjugator {
            None => got,
            Some(g) => g.geo(&got).geo(&g.reverse()),
        };
        match got.projective_eq(&self.rotor()) {
            true => Verified::Composes,
            false => Verified::Differs,
        }
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) -> Option<()> {
        // The count by its own coordinate (ruled (a)): the derivation is a
        // value, so it has no width wall; only a citation's u16 refuses.
        crate::transducer::address::put_key_count(out, self.generators.len());
        for (form, vertex, e) in &self.generators {
            crate::transducer::address::put_address(out, form)?;
            crate::transducer::address::put_address(out, vertex)?;
            out.extend_from_slice(&e.to_le_bytes());
        }
        // The residue whole, a formed residue's action with it (LD0 step 6).
        self.residue.put_whole(out);
        // The rotor travels when the residue does not determine it: a
        // factored vertex's product, and an adjoined generator's own
        // action. One byte says which, because "no citations" no longer
        // implies "the residue is the word" (37.7). Its canonical ray.
        match &self.composed {
            Some(r) => {
                out.push(1);
                r.put_canonical(out);
            }
            None => out.push(0),
        }
        Some(())
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Derivation> {
        let n = crate::transducer::address::take_key_count(b, pos)?;
        // The loop's boundary is the tape's own bytes (rule 10): every
        // iteration consumes two length-prefixed addresses and an i64, and
        // a byte that is not there refuses the whole read. Nothing is
        // reserved on the strength of the count, so a malformed count
        // allocates nothing and decides nothing (rules 2, 11).
        let mut generators = Vec::new();
        for _ in 0..n {
            let form = crate::transducer::address::take_address(b, pos)?;
            let vertex = crate::transducer::address::take_address(b, pos)?;
            let e = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            generators.push((form, vertex, e));
        }
        let residue = Word::take_whole(b, pos)?;
        let carried = *b.get(*pos)?;
        *pos += 1;
        let composed = match carried {
            0 => None,
            1 => Some(Box::new(Rotor::from_bytes(b, pos)?)),
            _ => return None,
        };
        // The conjugator rides the receipts envelope, never this tape (it is
        // evidence, 78.166 1c): a tape reads back untransported.
        Some(Derivation { generators, residue, composed, conjugator: None })
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
/// `g^e` on the ACTION (L14). The same two bounds as `power_of` and the
/// same reasoning: a finite order reduces the exponent in ℤ/m, an
/// infinite one squares and multiplies over the bits of `|e|`. `None`
/// when a negative exponent meets a rotor with no inverse — exact or
/// refused, never an identity standing in for one (rule 11).
pub fn rotor_power(g: &Rotor, e: i64) -> Option<Rotor> {
    // A negative power is the reverse's power, a ray (78.32(1)).
    let base = match e < 0 {
        true => g.is_versor().then(|| g.reverse())?,
        false => g.clone(),
    };
    let n = e.unsigned_abs();
    let n = match base.order() {
        Some(crate::transducer::alphabet::Order::Finite(m)) if m > 0 => n % u64::from(m),
        _ => n,
    };
    let mut r = Rotor::identity();
    let mut square = base;
    let mut rest = n;
    while rest > 0 {
        if rest & 1 == 1 {
            r = r.geo(&square);
        }
        rest >>= 1;
        if rest > 0 {
            square = square.geo(&square);
        }
    }
    Some(r)
}

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
