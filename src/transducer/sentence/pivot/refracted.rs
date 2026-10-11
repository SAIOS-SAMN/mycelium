// 78.6, 78.18 — THE REFRACTED EDGE HALF, CARRIED AS ITS FACTORS: a circuit
// through a held coset or a static contact keeps each transition as crossed,
// never the product of the candidates the held stabilizers multiply to.
//
//! A held `ℤ₂` doubles the candidates at every held transition it is crossed on, so
//! a circuit over `k` held transitions carries `2ᵏ` — a generate-and-test T5
//! refuses beyond small `k`. The circuit keeps the factors, never the product;
//! a circuit with no held transition has ONE candidate, read by
//! [`Refracted::single`]. T5's kernel over `𝔽₂` is the exact reader beyond,
//! the operator's to order (78.18).

use super::*;

/// ONE TRANSITION OF THE EDGE HALF FROM THE FIRST REFRACTED ONE ON, oriented the
/// way the circuit crosses it.
#[derive(Clone, Debug)]
pub(crate) enum RefractedTransition {
    /// A transition with one transport.
    Fixed(Box<Transport>),
    /// A static contact: the tether; the gauge half unread by name.
    Static(element::Versor),
    /// A held coset: several candidate transports, none chosen.
    Coset,
}

/// THE EDGE HALF AS ITS FACTORS: the composite before the first refracted
/// transition, and every transition from it on.
#[derive(Clone, Debug)]
pub(crate) struct Refracted {
    head: Transport,
    transitions: Vec<RefractedTransition>,
}

/// ONE CANDIDATE OF A REFRACTED EDGE HALF: its geometric composite, and its
/// gauge composite (word and hue) where every transition read one (NB1: the two
/// read apart and never summed).
#[derive(Clone, Debug)]
pub(crate) struct EdgeCandidate {
    pub(crate) carriage: element::Versor,
    pub(crate) gauge: Option<(Word, Vec<(u8, u8)>)>,
}

fn crossed(c: &EdgeCandidate, t: &Transport) -> EdgeCandidate {
    EdgeCandidate {
        carriage: t.carriage.geo(&c.carriage),
        gauge: c.gauge.as_ref().map(|(w, h)| (t.word.compose(w), compose_gauge(&t.gauge, h))),
    }
}

impl Refracted {
    /// The edge half at its first refracted transition, the composite so far the
    /// head; `None` where a transition it must invert has no inverse (the cone).
    pub(crate) fn open(head: Transport, r: &Refraction, forward: bool) -> Option<Refracted> {
        Refracted { head, transitions: Vec::new() }.cross(r, forward)
    }

    pub(crate) fn then(mut self, s: RefractedTransition) -> Refracted {
        self.transitions.push(s);
        self
    }

    /// One refracted contact crossed; `None` on the cone.
    pub(crate) fn cross(self, r: &Refraction, forward: bool) -> Option<Refracted> {
        let transition = match (r, forward) {
            (Refraction::Static { tether, .. }, true) => RefractedTransition::Static(tether.clone()),
            // Backward by the reverse, a ray (78.32(1)).
            (Refraction::Static { tether, .. }, false) => {
                crate::transducer::cga::is_versor(tether).then_some(())?;
                RefractedTransition::Static(tether.reverse())
            }
            (Refraction::Coset(_), _) => RefractedTransition::Coset,
        };
        Some(self.then(transition))
    }

    /// The held transitions crossed, `k`.
    pub(crate) fn held_transitions(&self) -> u64 {
        self.transitions.iter().filter(|s| matches!(s, RefractedTransition::Coset)).count() as u64
    }

    /// THE ONE CANDIDATE where no held transition was crossed; `None` where one
    /// was — several candidates, none chosen.
    pub(crate) fn single(&self) -> Option<EdgeCandidate> {
        let mut c = EdgeCandidate {
            carriage: self.head.carriage.clone(),
            gauge: Some((self.head.word.clone(), self.head.gauge.clone())),
        };
        for s in &self.transitions {
            c = match s {
                RefractedTransition::Fixed(t) => crossed(&c, t),
                RefractedTransition::Static(v) => EdgeCandidate { carriage: v.geo(&c.carriage), gauge: None },
                RefractedTransition::Coset => return None,
            };
        }
        Some(c)
    }
}

impl PathCarried {
    /// OR3 — THE CIRCUIT'S WHOLE TRANSPORT, `Ω_edge · Ω_face`: the one reader
    /// every orbit reading takes it through (law 12). `None` where the edge
    /// half is refused — the face half alone is part of the transport, and
    /// read as the whole it classed unread circuits trivial (rule 11) — and
    /// where it rides a held coset: several candidates, none chosen. A
    /// circuit through static contacts only has ONE candidate, and its
    /// geometric composite — the tether holonomy — is read here; its gauge
    /// half is static.
    ///
    /// 78.75 — AND IT IS THE MASTER EQUATION'S ONE PRODUCT. Where every
    /// transition's transport read, the whole transport is `Ω = ∏ cᵢ A(wᵢ)`
    /// in the circuit's order — each crossing its displacement and its word
    /// as one motion (`compose::terms_of`, 78.69). It read `Ω_edge · Ω_face`:
    /// two rings' products of carriages, multiplied after the fact, every
    /// word dropped — the two-field split read as one half, and the ring
    /// composed out of its own order. A transition whose word has no base
    /// action leaves the circuit unread (rule 11). Through a held coset or a
    /// static contact the refraction's own reader stands, as before.
    pub(crate) fn composite(&self) -> Option<element::Versor> {
        if self.refracted.is_none() {
            let terms = crate::transducer::sentence::compose::terms_of(self).ok()?;
            return Some(crate::transducer::sentence::compose::omega_of(&terms));
        }
        match (&self.edge, &self.refracted) {
            (Some(e), _) => Some(e.carriage.geo(&self.face.carriage)),
            (None, Some(r)) => r.single().map(|c| c.carriage.geo(&self.face.carriage)),
            _ => None,
        }
    }

    /// 78.26(5), 78.32 — THE CIRCUIT'S HUE, the fibre's holonomy: the transitions'
    /// hue maps composed along the circuit the way [`PathCarried::composite`]
    /// composes their carriages, `h_edge ∘ h_face`. `None` where the edge half
    /// did not read, rides a held coset, or crossed a static contact, whose
    /// gauge half is not read — unread, never the identity by assumption.
    pub(crate) fn hue(&self) -> Option<Vec<(u8, u8)>> {
        match (&self.edge, &self.refracted) {
            (Some(e), _) => Some(compose_gauge(&e.gauge, &self.face.gauge)),
            (None, Some(r)) => r.single()?.gauge.map(|(_, h)| compose_gauge(&h, &self.face.gauge)),
            _ => None,
        }
    }

    /// Whether the edge half crosses a held coset.
    pub(crate) fn holds_coset(&self) -> bool {
        self.refracted.as_ref().is_some_and(|r| r.held_transitions() > 0)
    }
}
