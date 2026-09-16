// 17.1, 17.2 — M1 and M2: one vertex, and the address that names it.
//
//! Equality of elements is equality of vertices, not of bytes: equal M1
//! coordinates, projectively equal carriages, equal sigma, an A5-equal chain.
//! The address is the M2 composite of that same content.

use crate::transducer::germinate::ResidualGerm;
use super::*;

impl Element {
    /// THE CARRIAGE between two thoughts — the versor carrying this one's
    /// base geometry onto the other's, when they sit on one orbit. The chain
    /// is the boundary of a thought, never its identity. A chain-carrying and
    /// a chain-free element are never carried onto each other.
    pub fn carriage(&self, other: &Element) -> Option<Versor> {
        match (self.sub.as_slice(), other.sub.as_slice()) {
            ([], []) => self.same_action(other).then(identity_versor),
            ([], _) | (_, []) => None,
            (a, b) => transition(a, b),
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
    /// `None` for a sentence, which is one level above a word and whose
    /// rotations are its loop holonomies, plural (`Form::loop_words`).
    pub fn carried_rotor(&self) -> Option<crate::transducer::alphabet::Rotor> {
        self.letters().map(|l| l.spelling.word.rotor())
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
        let Some(inv) = sensor.versor_inverse() else {
            return;
        };
        let carriage = v.geo(&inv);
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
    /// AND THE ROTOR SURVIVES IT. This wrote only `action.*` and left
    /// `description` empty, so `letters()` answered `None` for every
    /// composite and `cech::rotor_of(transition(..))` — which reads the
    /// tape and nothing else — answered `None` for every transition in the
    /// tree. The court's glue then had no rotor to put on a cell, wrote
    /// `Rotor::identity()` on the one edge it could, and `Complex::fill`
    /// short-circuited on an identity coefficient before reaching
    /// `classify`, which is the only producer of `Fill::Class` and
    /// `Fill::Unspellable`. That is `court cell` 0, `horizon seeded` 0 and
    /// the `ruled` term of the span reading, on every fleet ever run.
    ///
    /// The composition is in `Cl(3,1)` on the spatial action, which has one
    /// rotation plane; the rotor is in `Cl(4,0)` and has six. Composing the
    /// SPELLINGS keeps all six, and the carriage is solved so the transport
    /// is unchanged.
    ///
    /// Only where BOTH operands carry a tape. A sentence carries none by
    /// construction — its rotations are its loop holonomies, plural — so a
    /// transition between two sentences still answers no single rotor, and
    /// that is the court's path-holonomy question, not this one.
    pub fn compose(&self, other: &Element) -> Element {
        let mut out = Element::identity();
        out.action.spatial = match (self.versor(), other.versor()) {
            (Some(a), Some(b)) => Some(a.geo(&b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        out.action.gauge = compose_perm(&self.action.gauge, &other.action.gauge);
        out.action.temporal = self.action.temporal.saturating_add(other.action.temporal);
        out.action.shell = self.action.shell.saturating_add(other.action.shell);
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
        if let (Some(a), Some(b)) = (self.letters(), other.letters()) {
            let word = a.spelling.word.compose(&b.spelling.word).normal_form();
            out.carry_tape(word);
        }
        out
    }

    /// SOLUTION — the inverse deduction `g⁻¹`. Versor inverse on S and on
    /// each germinated axis; inverse permutation on G. `None` when a versor
    /// is not invertible.
    pub fn solution(&self) -> Option<Element> {
        if !self.sub.is_empty() && self.versor().is_none() && self.action.germinated.is_empty() {
            return None;
        }
        let mut out = Element::identity();
        if let Some(v) = self.versor() {
            out.action.spatial = Some(v.versor_inverse()?);
        }
        out.action.gauge = invert_perm(&self.action.gauge)?;
        for (i, w) in &self.action.germinated {
            out.action.germinated.push((*i, w.versor_inverse()?));
        }
        // The inverse carries the inverse spelling, for the reason the
        // composition carries the composed one: a transition is
        // `g_y ∘ g_x⁻¹`, so a solution that drops its tape starves the
        // transition exactly as a composition that drops its own does.
        if let Some(l) = self.letters() {
            let word = l.spelling.word.inverse().normal_form();
            out.carry_tape(word);
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
        self.solution()
    }

    /// Canonical ANASTOME bytes: the BASE identity (word, dim, fibers) followed
    /// by the punctured axis's sections — the FULL bundle, so a taught thought
    /// carries its interior volume across the nerve.
    pub fn canonical(&self) -> Vec<u8> {
        let mut out = self.base_canonical();
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        for s in &self.sections {
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            for (b, a) in s {
                out.push(*b);
                out.push(*a);
            }
        }
        // The torsion cycles: (generator relations, period).
        out.extend_from_slice(&(self.torsion.len() as u32).to_le_bytes());
        for TorsionCycle { sigma, period: l, fibre } in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        // The connection: (assignment, g, fibre) relations.
        out.extend_from_slice(&(self.transitions.len() as u32).to_le_bytes());
        for Transition { asg: a, gauge: g, fibre } in &self.transitions {
            out.extend_from_slice(&(a.len() as u32).to_le_bytes());
            for (b, f) in a {
                out.push(*b);
                out.push(*f);
            }
            g.to_bytes(&mut out);
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        out
    }

    /// The BASE canonical form: tape and chain. The action is on the tape
    /// or composed from it; germinated axes are listed by index.
    fn base_canonical(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.description.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.description);
        out.extend_from_slice(&(self.sub.len() as u32).to_le_bytes());
        for (r, c, before, after) in self.sub.as_slice() {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*before);
            out.push(*after);
        }
        for (i, _) in &self.action.germinated {
            out.extend_from_slice(&i.to_le_bytes());
        }
        out
    }

    /// Vertex key of the word form: geodesic normal form plus frame
    /// coordinates as exact rationals. A sentence is keyed by its form
    /// (M1, ρ, agreed shift relative to the Frame, contact, gauge, faces).
    /// The body's absolute position is already quotiented on the chain.
    pub fn address(&self) -> crate::transducer::address::Address {
        self.address
            .get_or_init(|| {
                if self.is_sentence() {
                    if let Some(k) = crate::transducer::sentence::vertex_address(self) {
                        return k;
                    }
                }
                let mut buf = vec![crate::transducer::address::TAG_FORM];
                // W6: AN ADDRESS IS NEVER THE TAG ALONE, and never a tape.
                // Both refusals this reader used to carry are gone by
                // type: `sub` is A5's accepted chain, and a rewrite's
                // names are citations proven to fit their prefix, refused
                // at `mint` where the element is built (47.1).
                crate::transducer::address::put_bytes(&mut buf, &self.form_key_bytes());
                crate::transducer::address::Address::new(buf)
            })
            .clone()
    }

    /// Meaning of a non-sentence word: M1 ‖ carriage ‖ σ ‖ chain (17.2).
    /// ρ is in the carriage, never beside it.
    fn form_key_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(rw) = crate::transducer::rewrite::Rewrite::from_element(self) {
            out.push(REWRITE_TAG);
            out.push(rw.kind as u8);
            out.extend_from_slice(&(rw.before.len() as u16).to_le_bytes());
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
            crate::transducer::address::put_bytes(&mut out, letters.spelling.word.key().as_bytes());
            // A4 output (17.2): one projective carriage, one writing. Raw
            // coefficients here made a carriage and its negation two
            // addresses, and `blind_replay` slashed on the difference.
            letters.carriage.put_canonical(&mut out);
            let mut sigma = letters.sigma.clone();
            sigma.sort_unstable();
            out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
            for (f, t) in sigma {
                out.push(f);
                out.push(t);
            }
        } else if let Some(v) = self.action.spatial.as_ref() {
            // The spatial action is a versor too; the same A4 writing.
            v.put_canonical(&mut out);
        }
        // The key IS the chain (law 2), and the chain is A5's by type: one
        // that says two things at a site has no centering and is refused
        // where the element is built.
        out.extend_from_slice(&(self.sub.len() as u32).to_le_bytes());
        for (r, c, b, a) in self.sub.as_slice() {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*b);
            out.push(*a);
        }
        out
    }

}
