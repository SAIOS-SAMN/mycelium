// 78.12(4), PL1 — THE CIRCUIT'S TRAJECTORY READ STEP BY STEP, off the steps
// the walk kept: each step oriented the way the circuit runs, its ray, its
// class and modes, and the class of the transport composed through it.

use super::*;
use crate::observe::{Carried, Circuit, CircuitStep, Circuits};

thread_local! {
    /// The circuits read since the percept armed the orbits, in the order
    /// the walk read them; `None` while nobody is listening. Armed and
    /// replaced with the orbits' list, so at most one percept's circuits are
    /// ever held.
    static CIRCUITS: std::cell::RefCell<Option<Vec<Circuit>>> = const { std::cell::RefCell::new(None) };
}

/// Start listening, with the orbits.
pub(super) fn arm_circuits() {
    CIRCUITS.with(|c| *c.borrow_mut() = Some(Vec::new()));
}

/// File one kept circuit's trajectory, where a reader is listening.
pub(super) fn note_circuit(r: impl FnOnce() -> Circuit) {
    CIRCUITS.with(|c| {
        if let Some(held) = c.borrow_mut().as_mut() {
            held.push(r());
        }
    });
}

/// The circuits of the last percept, and stop listening. `None` where no
/// percept armed the list since the last take — a reading not taken, never
/// an empty one.
pub fn take_circuits() -> Option<Circuits> {
    CIRCUITS.with(|c| c.borrow_mut().take()).map(|circuits| Circuits { circuits })
}

/// ONE CIRCUIT, read off the steps its walk kept. A step's transport is
/// oriented the way the circuit runs before it is read; the composite runs
/// on through it, and a step that is unread or on the cone leaves every
/// later composite unread (rule 11).
pub(super) fn read_circuit(chord: u16, carried: &PathCarried, faces: &[FormFace]) -> Circuit {
    let mut composite: Option<element::Versor> = Some(element::identity_versor());
    let steps = carried
        .steps
        .iter()
        .map(|s| {
            let refracted = faces.get(usize::from(s.face)).is_some_and(|f| !f.interface.is_empty());
            let oriented = match (&s.transport, s.forward) {
                (None, _) => None,
                (Some(t), true) => Some(Some(t.carriage.clone())),
                (Some(t), false) => Some(t.carriage.versor_inverse()),
            };
            let carried = match oriented {
                None => {
                    composite = None;
                    Carried::Unread
                }
                Some(None) => {
                    composite = None;
                    Carried::Cone
                }
                Some(Some(g)) => {
                    composite = composite.take().map(|m| g.geo(&m));
                    Carried::Read {
                        ray: ray_of(&g),
                        kind: g.class_invariants().map(|(_, _, k)| k),
                        modes: ModesRead::of(&g),
                        after: composite.as_ref().and_then(|m| m.class_invariants()).map(|(_, _, k)| k),
                    }
                }
            };
            CircuitStep { face: s.face, forward: s.forward, refracted, carried }
        })
        .collect();
    Circuit { chord, steps }
}

/// A versor's ray in its canonical coordinates, each non-zero blade and its
/// exact coefficient. Empty where the element has no canonical writing.
fn ray_of(g: &element::Versor) -> Vec<(u16, num_rational::BigRational)> {
    match g.canonical() {
        Some(c) => (0..c.slot_count())
            .filter(|&b| !num_traits::Zero::is_zero(c.coeff(b)))
            .map(|b| (b as u16, c.coeff(b).clone()))
            .collect(),
        None => Vec::new(),
    }
}
