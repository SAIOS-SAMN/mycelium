// 17.1, 17.2 — M1 and M2: one vertex, and the address that names it.
//
//! Equality of elements is equality of vertices, not of bytes: equal M1
//! coordinates, projectively equal carriages, equal sigma, an A5-equal chain.
//! The address is the M2 composite of that same content.

use crate::transducer::germinate::ResidualGerm;
use super::*;

/// THE SHAPE OF A NON-SENTENCE ADDRESS, written as its first byte so a
/// reader knows what follows (rule 11). `TAG_RESIDUAL` covers four shapes
/// and nothing distinguished them; an inverse could only have guessed.
pub const KIND_REWRITE: u8 = 0;
pub const KIND_LETTERS: u8 = 1;
pub const KIND_SPATIAL: u8 = 2;
pub const KIND_SPATIAL_ROTOR: u8 = 3;
pub const KIND_CHAIN: u8 = 4;

impl Element {
    /// THE CARRIAGE between two thoughts — the versor carrying this one's
    /// base geometry onto the other's, when they sit on one orbit. The chain
    /// is the boundary of a thought, never its identity. A chain-carrying and
    /// a chain-free element are never carried onto each other.
    pub fn carriage(&self, other: &Element, budget: &mut crate::physics::Budget) -> Option<Versor> {
        match (self.sub.as_slice(), other.sub.as_slice()) {
            ([], []) => self.same_action(other).then(identity_versor),
            ([], _) | (_, []) => None,
            (a, b) => transition(a, b, budget),
        }
    }

    /// Group elements: the same action iff the versors agree projectively
    /// and the permutations are equal.
    pub fn same_action(&self, other: &Element) -> bool {
        self.sub.is_empty()
            && other.sub.is_empty()
            && match (self.versor(), other.versor()) {
                (None, None) => true,
                (Some(a), Some(b)) => same_versor(&a, &b),
                _ => false,
            }
            && self.action.gauge == other.action.gauge
            && self.germinated_eq(other)
    }

    fn germinated_eq(&self, other: &Element) -> bool {
        if self.action.germinated.len() != other.action.germinated.len() {
            return false;
        }
        self.action
            .germinated
            .iter()
            .zip(&other.action.germinated)
            .all(|((i, a), (j, b))| i == j && same_versor(a, b))
    }

    /// M1 (17.1): same vertex of the Cayley graph. Equal M1 coordinates of
    /// the letters, PROJECTIVELY equal carriage versors (A4), equal sigma,
    /// A5-equal chain.
    ///
    /// This was `self.address() == other.address()`, which is the M2
    /// invariant read backwards. Address equality does imply the same
    /// vertex (17.2), so it stands as a fast path; but a DIFFERING address
    /// does not imply a different vertex, and reading it as one makes the
    /// codec the arbiter of meaning. A carriage versor and its negation are
    /// one projective element and two byte strings: two organs that derive
    /// carriages of opposite sign hold the same vertex and would have been
    /// called different. Every comparison below is on decoded coordinates
    /// with the predicate that coordinate's algebra provides.
    pub fn same_vertex(&self, other: &Element) -> bool {
        // The fast path, never the test: a differing address falls through.
        if self.address() == other.address() {
            return true;
        }
        // SENTENCE FORMS are addressed by `sentence::vertex_address`, which
        // 17.2 step 3 defines as M1 keys and A4 canonical versors per germ
        // and per face -- canonical coordinates, so equality of those bytes
        // IS the vertex comparison. It writes the A4 output of every
        // carriage (`Mv::put_canonical`), as does `form_key_bytes` below, so
        // a carriage and its negation are one address everywhere and a
        // differing address here is a differing vertex. 17.2's open item is
        // closed; this arm is the whole test for sentence forms.
        if self.is_sentence() || other.is_sentence() {
            return false;
        }
        // A5: the chain is the boundary, and it is canonical before it is
        // compared -- two spellings of one boundary reduce to one chain. A
        // chain with no canonical form is no boundary, and two of them are
        // not one vertex.
        let (Some(mine), Some(theirs)) =
            (canonical_chain(&self.sub), canonical_chain(&other.sub))
        else {
            return false;
        };
        if mine != theirs {
            return false;
        }
        // 17.2 step 2: a rewrite is named by the forms it cites, and those
        // citations are M2 outputs -- coordinates, not digests of content.
        let rw = (
            crate::transducer::rewrite::Rewrite::from_element(self),
            crate::transducer::rewrite::Rewrite::from_element(other),
        );
        match rw {
            (Some(a), Some(b)) => {
                return a.kind == b.kind
                    && a.before == b.before
                    && a.after == b.after
                    && a.claimed == b.claimed;
            }
            (None, None) => {}
            _ => return false,
        }
        // The ACTION is part of the vertex whichever arm names the word:
        // the gauge permutation and the germinated axes are coordinates of
        // it, and comparing only the letters left two elements differing in
        // an opened axis reading as one vertex.
        if self.action.gauge != other.action.gauge || !self.germinated_eq(other) {
            return false;
        }
        match (self.letters(), other.letters()) {
            (Some(a), Some(b)) => {
                // M1 on the letters: the rotor's projective class, which is
                // the same test as key equality without going through bytes.
                if !a.spelling.word.rotor().projective_eq(&b.spelling.word.rotor()) {
                    return false;
                }
                if !same_versor(&a.carriage, &b.carriage) {
                    return false;
                }
                let (mut x, mut y) = (a.sigma.clone(), b.sigma.clone());
                x.sort_unstable();
                y.sort_unstable();
                x == y
            }
            (None, None) => match (self.action.spatial.as_ref(), other.action.spatial.as_ref()) {
                (Some(a), Some(b)) => same_versor(a, b),
                (None, None) => true,
                _ => false,
            },
            _ => false,
        }
    }

    /// THE ROTOR THIS ELEMENT CARRIES, in `Cl(4,0)` over all six bivector
    /// planes. It lives on the tape and nowhere else: `action.spatial` is a
    /// `Versor` in `Cl(3,1)`, which has ONE rotation plane, and
    /// `sensor_action_of` states that the map between them cannot be onto.
    ///
    /// A SENTENCE ANSWERS THE ROTOR ITS WORD VERTICES AGREE ON, or nothing.
    ///
    /// It is one level above a word and `letters()` refuses its tag for
    /// that reason, so this read `None` for every sentence — and the
    /// court's glue composes SPOKEN SENTENCES, which is why carrying the
    /// tape through composition did not on its own give the court an edge
    /// rotor to write.
    ///
    /// A form whose word vertices all speak one rotor up to projective
    /// equality HAS that rotor; one that speaks several has none, and says
    /// so. That is `Bubble::rotor`'s rule exactly, and it is the rule
    /// because a representative picked from several would be a different
    /// motion on a node that took the bodies in another order (rule 15).
    /// A form's orbit holonomies are a different object and stay where they
    /// are: they are the curvature AROUND a cycle, not the transport this
    /// sentence IS.
    pub fn carried_rotor(&self) -> Option<crate::transducer::alphabet::Rotor> {
        if let Some(l) = self.letters() {
            return Some(l.spelling.word.rotor());
        }
        if let Some(r) = self.action.rotor.as_ref() {
            return Some(r.clone());
        }
        if !self.is_sentence() {
            return None;
        }
        let f = crate::transducer::sentence::from_element_form(self)?;
        let mut held: Option<crate::transducer::alphabet::Rotor> = None;
        for (_, v) in f.vertices.scoped() {
            let crate::transducer::sentence::Vertex::Word { derivation, .. } = v else {
                continue;
            };
            let r = derivation.rotor();
            match &held {
                None => held = Some(r),
                Some(h) if h.projective_eq(&r) => {}
                Some(_) => return None,
            }
        }
        held
    }

    /// The tape a composite carries so its rotor survives: the composed
    /// spelling, with the carriage chosen so the element's versor is
    /// exactly the spatial action this composition computed.
    fn carry_tape(&mut self, word: crate::transducer::alphabet::Word) {
        let Some(v) = self.action.spatial.clone() else {
            return;
        };
        // `versor()` reads `conformal_of(word, carriage)` once letters
        // exist, and that is `carriage · S(rotor)`. Solving it for the
        // carriage keeps the composite's transport exactly what it was.
        let Some(sensor) = conformal_of(&word, &identity_versor()) else {
            return;
        };
        // `V S̃`, the carriage as a ray (78.32(1)).
        if !crate::transducer::cga::is_versor(&sensor) {
            return;
        }
        let carriage = v.geo(&sensor.reverse());
        let spelling = crate::transducer::alphabet::Spelling { word };
        let gauge = self.action.gauge.clone();
        self.description = crate::transducer::element::residual::connection_description(
            &spelling,
            &carriage,
            &gauge,
            &Vec::new(),
        );
        self.address.take();
    }

    /// COMPOSE — geometric product on S, permutation composition on G,
    /// geometric product on matching germinated axes. Associative.
    ///
    /// AND THE ROTOR SURVIVES IT: `cech::rotor_of(transition(..))` reads
    /// the tape and nothing else, so a composite carries the composed
    /// spelling, or the court's glue has no rotor to put on a simplex and
    /// `Complex::fill` never reaches `classify`.
    ///
    /// The composition is in `Cl(3,1)` on the spatial action, which has one
    /// rotation plane; the rotor is in `Cl(4,0)` and has six. Composing the
    /// SPELLINGS keeps all six, and the carriage is solved so the transport
    /// is unchanged.
    ///
    /// Only where BOTH operands carry a tape. A sentence carries none by
    /// construction — its rotations are its orbit holonomies, plural — so a
    /// transition between two sentences still answers no single rotor, and
    /// that is the court's path-holonomy question, not this one.
    ///
    /// COMPOSITION FORMS THE TEMPORAL AND SHELL COUNTS, so it is where they
    /// are refused: `None` where either sum would pass its carrier (STD1).
    pub fn compose(&self, other: &Element) -> Option<Element> {
        let mut out = Element::identity();
        out.action.spatial = match (self.versor(), other.versor()) {
            (Some(a), Some(b)) => Some(a.geo(&b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        out.action.gauge = compose_perm(&self.action.gauge, &other.action.gauge);
        out.action.temporal = self.action.temporal.checked_add(other.action.temporal)?;
        out.action.shell = self.action.shell.checked_add(other.action.shell)?;
        let mut germ = self.action.germinated.clone();
        for (j, w) in &other.action.germinated {
            match germ.iter_mut().find(|(i, _)| i == j) {
                Some((_, held)) => *held = held.geo(w),
                None => germ.push((*j, w.clone())),
            }
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — canonical key order: a germinated axis IS its index, and the key writes the axes in it"
        )]
        germ.sort_by_key(|(i, _)| *i);
        out.action.germinated = germ;
        // THE ROTOR SURVIVES, BY A TAPE WHERE ONE SPELLS IT AND BY THE
        // ACTION WHERE NONE DOES. Two words compose to a word, and the
        // composite carries that spelling. Two SENTENCES do not — a
        // sentence is one level above a word — so the composite carries
        // the product on the action instead, which is the only place a
        // six-plane rotor can sit when the alphabet is not spelling it.
        match (self.letters(), other.letters()) {
            (Some(a), Some(b)) => {
                let word = a.spelling.word.compose(&b.spelling.word).normal_form();
                out.carry_tape(word);
            }
            _ => {
                if let (Some(a), Some(b)) = (self.carried_rotor(), other.carried_rotor()) {
                    out.action.rotor = Some(a.geo(&b));
                    out.address.take();
                }
            }
        }
        Some(out)
    }

    /// THE REVERSE — the element read the other way (78.32(1)): the free
    /// involution on every factor it acts by, never an inversion. The
    /// spatial versor and each germinated axis by their reverse (a ray, read
    /// by proportion wherever it is read); the gauge by its inverse
    /// permutation; the spelling by its inverse word. `None` where an
    /// action is no versor — a motion that does not act has no reverse
    /// either way round. It was `solution`, the deduction `g⁻¹`, dividing
    /// by each weight.
    pub fn reverse(&self) -> Option<Element> {
        if !self.sub.is_empty() && self.versor().is_none() && self.action.germinated.is_empty() {
            return None;
        }
        let is_versor = crate::transducer::cga::is_versor;
        let mut out = Element::identity();
        if let Some(v) = self.versor() {
            is_versor(&v).then_some(())?;
            out.action.spatial = Some(v.reverse());
        }
        out.action.gauge = invert_perm(&self.action.gauge)?;
        for (i, w) in &self.action.germinated {
            is_versor(w).then_some(())?;
            out.action.germinated.push((*i, w.reverse()));
        }
        // The inverse carries the inverse spelling, for the reason the
        // composition carries the composed one: a transition is
        // `g_y ∘ g_x⁻¹`, so a solution that drops its tape starves the
        // transition exactly as a composition that drops its own does.
        match self.letters() {
            Some(l) => out.carry_tape(l.spelling.word.inverse().normal_form()),
            None => {
                if let Some(r) = self.carried_rotor().filter(|r| r.is_versor()) {
                    out.action.rotor = Some(r.reverse());
                    out.address.take();
                }
            }
        }
        Some(out)
    }

    /// Inverse: versor inverse when the action is a versor; the reverse
    /// coboundary chain when the substance is a residual germ. `None` when
    /// a versor is not invertible.
    pub fn inverse(&self) -> Option<Element> {
        if !self.sub.is_empty() && self.versor().is_none() && self.action.germinated.is_empty() {
            let rev: ResidualGerm = self
                .sub
                .iter()
                .map(|&(r, c, b, a)| (r, c, a, b))
                .collect();
            // A chain whose reverse says two things at one site has no
            // canonical form, so this element has no inverse to name.
            return from_residual(&rev);
        }
        self.reverse()
    }

    /// Vertex key of the word form: geodesic normal form plus frame
    /// coordinates as exact rationals. A sentence is keyed by its form
    /// (M1, ρ, agreed shift relative to the Frame, contact, gauge, faces).
    /// The body's absolute position is already quotiented on the chain.
    pub fn address(&self) -> crate::transducer::address::Address {
        self.address
            .get_or_init(|| {
                // 54.7 B6 — THE FORM PATH IS GONE FROM HERE, and with it the
                // last reader of a sentence tape.
                //
                // This asked `is_sentence()` — which read the description's
                // magic byte — and then called `vertex_address`, which
                // parsed the tape under a version gate to recover the key.
                // So a word could not know its own name without a tape, and
                // `is_sentence` could not be moved onto the address while
                // the address was derived from the tape: the two were
                // mutually recursive through the byte buffer.
                //
                // A FORM'S ADDRESS IS NEVER DERIVED HERE NOW. The mint sets
                // it (`as_element_form`, 54.5) because the mint has the form
                // in hand and the key IS the form (law 2); the record
                // carries it and refuses an empty one rather than guessing
                // (rule 11). An element that reaches this closure is not a
                // closed form, and says so by its tag.
                let mut buf = vec![crate::transducer::address::TAG_RESIDUAL];
                // W6: AN ADDRESS IS NEVER THE TAG ALONE, and never a tape.
                // Both refusals this reader used to carry are gone by
                // type: `sub` is A5's accepted chain, and a rewrite's
                // names are citations proven to fit their prefix, refused
                // at `mint` where the element is built (47.1).
                crate::transducer::address::put_key_bytes(&mut buf, &self.form_key_bytes());
                crate::transducer::address::Address::new(buf)
            })
            .clone()
    }

    /// Meaning of a non-sentence word: KIND ‖ M1 ‖ carriage ‖ σ ‖ chain
    /// (17.2). ρ is in the carriage, never beside it.
    ///
    /// THE KIND BYTE LEADS, so the shape is readable (rule 11). Four
    /// different shapes were written here with nothing saying which: a
    /// rewrite, a letters word, a spatial action with or without its
    /// rotor, and a bare chain. A reader could not tell a rewrite's tag
    /// byte from a chain's length prefix, so `TAG_RESIDUAL` had the same
    /// collision one level down that `TAG_FORM` had one level up — and an
    /// inverse for it could only have guessed.
    ///
    /// 0 rewrite · 1 letters · 2 spatial · 3 spatial with rotor · 4 chain.
    /// Every residual address in the tree changes; §10, a layout change is
    /// a recut.
    fn form_key_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(rw) = crate::transducer::rewrite::Rewrite::from_element(self) {
            out.push(KIND_REWRITE);
            out.push(REWRITE_TAG);
            out.push(rw.kind as u8);
            crate::transducer::address::put_key_count(&mut out, rw.before.len());
            for a in &rw.before {
                crate::transducer::address::put_cited(&mut out, a);
            }
            crate::transducer::address::put_cited(&mut out, &rw.after);
            for m in &rw.claimed {
                out.extend_from_slice(&m.to_le_bytes());
            }
            return out;
        }
        if let Some(letters) = self.letters() {
            out.push(KIND_LETTERS);
            letters.spelling.word.put_canonical(&mut out);
            // A4 output (17.2): one projective carriage, one writing. Raw
            // coefficients here made a carriage and its negation two
            // addresses, and `blind_replay` slashed on the difference.
            letters.carriage.put_canonical(&mut out);
            let mut sigma = letters.sigma.clone();
            sigma.sort_unstable();
            crate::transducer::address::put_key_count(&mut out, sigma.len());
            for (f, t) in sigma {
                out.push(f);
                out.push(t);
            }
        } else if let Some(v) = self.action.spatial.as_ref() {
            out.push(match self.action.rotor.is_some() {
                true => KIND_SPATIAL_ROTOR,
                false => KIND_SPATIAL,
            });
            // The spatial action is a versor too; the same A4 writing.
            v.put_canonical(&mut out);
            // AND THE ROTOR BESIDE IT, where one is carried. The versor is
            // the sensor's projection of it, so two composites turning in
            // different planes can share a spatial action and would have
            // shared an address (law 2). Canonical coordinates, as A1
            // writes them.
            if let Some(r) = self.action.rotor.as_ref() {
                r.put_canonical(&mut out);
            }
        } else {
            out.push(KIND_CHAIN);
        }
        // The key IS the chain (law 2), and the chain is A5's by type: one
        // that says two things at a site has no centering and is refused
        // where the element is built. Its count is a coordinate of the key,
        // written by its own coordinate (STD1 3c-2, ruled (d)).
        crate::transducer::address::put_key_count(&mut out, self.sub.len());
        for (r, c, b, a) in self.sub.as_slice() {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*b);
            out.push(*a);
        }
        out
    }

}
