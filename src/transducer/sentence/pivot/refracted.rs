// 78.6, 78.18 — THE REFRACTED EDGE HALF, CARRIED AS ITS FACTORS: a circuit
// through a held coset or a static contact keeps each transition as crossed, and
// the candidates the held stabilizers multiply to are formed only by a
// reader holding the stroke's budget (rule 10).
//
//! A held `ℤ₂` doubles the candidates at every held transition it is crossed on, so
//! a circuit over `k` held transitions carries `2ᵏ` — a generate-and-test T5
//! refuses beyond small `k`. The circuit keeps the factors, never the product;
//! [`Refracted::candidates`] forms the product one candidate at a time,
//! spending one read of the stroke per candidate formed, and refuses
//! UNBOUNDED by name where the stroke is spent. T5's kernel over `𝔽₂` is the
//! exact reader beyond, the operator's to order (78.18).

use super::*;

/// ONE TRANSITION OF THE EDGE HALF FROM THE FIRST REFRACTED ONE ON, oriented the
/// way the circuit crosses it.
#[derive(Clone, Debug)]
pub(crate) enum RefractedTransition {
    /// A transition with one transport.
    Fixed(Box<Transport>),
    /// A static contact: the tether; the gauge half unread by name.
    Static(element::Versor),
    /// A held coset: every candidate transport.
    Coset(Vec<Transport>),
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

/// HOW A CIRCUIT THROUGH A HELD COSET CLOSES (78.6(1)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CosetClosure {
    /// Every candidate the identity: the held subgroup acts trivially here.
    Closed,
    /// The coset contains the identity: closed modulo the held `ℤ₂ᵏ`.
    ClosedModuloHeld,
    /// No candidate is the identity.
    Open,
    /// The stroke was spent forming the candidates (rule 10): unread by name.
    Unbounded,
}

/// The stroke was spent before every candidate was formed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Unbounded;

/// HOW A COSET'S WHOLE TRANSPORTS CLOSE against the identity ray (EQ1).
pub(crate) fn closure_of(cands: &[element::Versor]) -> CosetClosure {
    let identity = element::identity_versor();
    let closes: Vec<bool> = cands.iter().map(|c| element::same_versor(c, &identity)).collect();
    match (closes.iter().all(|&b| b), closes.iter().any(|&b| b)) {
        (true, _) => CosetClosure::Closed,
        (false, true) => CosetClosure::ClosedModuloHeld,
        (false, false) => CosetClosure::Open,
    }
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
            (Refraction::Coset(c), true) => RefractedTransition::Coset(c.clone()),
            (Refraction::Coset(c), false) => RefractedTransition::Coset(c.iter().map(Transport::reverse).collect()),
        };
        Some(self.then(transition))
    }

    /// The held transitions crossed, `k`.
    pub(crate) fn held_transitions(&self) -> u64 {
        self.transitions.iter().filter(|s| matches!(s, RefractedTransition::Coset(_))).count() as u64
    }

    /// 78.6(2) — a static contact crossed: the gauge half unread by name.
    pub(crate) fn gauge_static(&self) -> bool {
        self.transitions.iter().any(|s| matches!(s, RefractedTransition::Static(_)))
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
                RefractedTransition::Coset(_) => return None,
            };
        }
        Some(c)
    }

    /// EVERY CANDIDATE, formed one at a time under the stroke: one read spent
    /// per candidate a held transition forms, [`Unbounded`] where the stroke is
    /// spent before the last (rule 10).
    pub(crate) fn candidates(&self, budget: &mut crate::physics::Budget) -> Result<Vec<EdgeCandidate>, Unbounded> {
        let mut cands = vec![EdgeCandidate {
            carriage: self.head.carriage.clone(),
            gauge: Some((self.head.word.clone(), self.head.gauge.clone())),
        }];
        for s in &self.transitions {
            cands = match s {
                RefractedTransition::Fixed(t) => cands.iter().map(|c| crossed(c, t)).collect(),
                RefractedTransition::Static(v) => {
                    cands.into_iter().map(|c| EdgeCandidate { carriage: v.geo(&c.carriage), gauge: None }).collect()
                }
                RefractedTransition::Coset(coset) => {
                    let mut next = Vec::with_capacity(cands.len() * coset.len());
                    for c in &cands {
                        for t in coset {
                            if !budget.spend() {
                                return Err(Unbounded);
                            }
                            next.push(crossed(c, t));
                        }
                    }
                    next
                }
            };
        }
        for c in cands.iter_mut() {
            if let Some((w, _)) = c.gauge.as_mut() {
                *w = w.normal_form();
            }
        }
        Ok(cands)
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
    /// half is static, read by [`PathCarried::gauge_static`].
    pub(crate) fn composite(&self) -> Option<element::Versor> {
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

    /// 78.6(1) — EVERY CANDIDATE'S WHOLE TRANSPORT through a held coset, and
    /// the held transitions crossed, formed under the stroke; `None` where the
    /// edge half holds no coset.
    pub(crate) fn coset(
        &self,
        budget: &mut crate::physics::Budget,
    ) -> Option<Result<(Vec<element::Versor>, u64), Unbounded>> {
        let r = self.refracted.as_ref().filter(|r| r.held_transitions() > 0)?;
        Some(r.candidates(budget).map(|cands| {
            (cands.iter().map(|c| c.carriage.geo(&self.face.carriage)).collect(), r.held_transitions())
        }))
    }

    /// HOW THE CIRCUIT CLOSES through its held coset, read against the
    /// identity ray (EQ1) under the stroke; `None` where it holds no coset.
    pub(crate) fn coset_closure(&self, budget: &mut crate::physics::Budget) -> Option<CosetClosure> {
        Some(match self.coset(budget)? {
            Ok((cands, _)) => closure_of(&cands),
            Err(Unbounded) => CosetClosure::Unbounded,
        })
    }

    /// 78.6(2) — the gauge half unread by name: a static contact crossed.
    pub(crate) fn gauge_static(&self) -> bool {
        self.refracted.as_ref().is_some_and(Refracted::gauge_static)
    }
}
