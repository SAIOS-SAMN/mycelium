// 78.12(4), PL1 — THE CIRCUIT'S TRAJECTORY READ STEP BY STEP, off the steps
// the walk kept: each step oriented the way the circuit runs, its ray, its
// class and modes, and the class of the transport composed through it.

use super::*;
use crate::observe::{Carried, Circuit, CircuitStep, Circuits};

/// ONE STEP AS THE WALK LEFT IT: oriented, its composite's class read, its
/// modes owed to the reader that holds the stroke's budget (BD1).
struct PendingStep {
    face: u16,
    forward: bool,
    refracted: bool,
    /// `None` unread, `Some(None)` on the cone, `Some(Some(g))` read.
    oriented: Option<Option<element::Versor>>,
    after: Option<crate::network::nerve::coord::BivectorKind>,
}

pub(super) struct PendingCircuit {
    chord: u16,
    /// The circuit's members, in walk order — where a cell reads whether the
    /// circuit crosses it (CG1).
    members: Vec<Vec<u8>>,
    /// Its whole transport by the one composite reader, `None` where the
    /// edge half was refused (OR3).
    composite: Option<element::Versor>,
    steps: Vec<PendingStep>,
}

thread_local! {
    /// The circuits walked since the percept armed the orbits, in the order
    /// the walk read them; `None` while nobody is listening. Armed and
    /// replaced with the orbits' list, so at most one percept's circuits are
    /// ever held.
    static PENDING: std::cell::RefCell<Option<Vec<PendingCircuit>>> = const { std::cell::RefCell::new(None) };
    /// The last percept's circuits, read whole under its budget, until the
    /// observer takes them.
    static READ: std::cell::RefCell<Option<Circuits>> = const { std::cell::RefCell::new(None) };
}

/// Start listening, with the orbits.
pub(super) fn arm_circuits() {
    PENDING.with(|c| *c.borrow_mut() = Some(Vec::new()));
}

/// File one kept circuit's trajectory, where a reader is listening.
pub(super) fn note_circuit(r: impl FnOnce() -> PendingCircuit) {
    PENDING.with(|c| {
        if let Some(held) = c.borrow_mut().as_mut() {
            held.push(r());
        }
    });
}

/// THE CIRCUITS THE PERCEPT WALKED, each by its members and its composite —
/// what a cell's orbit algebra reads (CG1), taken off the walk that already
/// ran and never by walking again. Empty where nobody armed the list.
pub fn walked_circuits() -> Vec<(Vec<Vec<u8>>, Option<element::Versor>)> {
    PENDING.with(|c| match c.borrow().as_ref() {
        Some(held) => held.iter().map(|p| (p.members.clone(), p.composite.clone())).collect(),
        None => Vec::new(),
    })
}

/// READ THE CIRCUITS THE PERCEPT WALKED, each step's modes under the
/// stroke's budget (BD1), and hold them for the observer; stop listening.
/// A percept that armed nothing reads nothing and leaves what is held.
pub fn finish_circuits(budget: &mut crate::physics::Budget) {
    let Some(pending) = PENDING.with(|c| c.borrow_mut().take()) else { return };
    let circuits = pending
        .into_iter()
        .map(|p| Circuit {
            chord: p.chord,
            steps: p
                .steps
                .into_iter()
                .map(|s| CircuitStep {
                    face: s.face,
                    forward: s.forward,
                    refracted: s.refracted,
                    carried: match s.oriented {
                        None => Carried::Unread,
                        Some(None) => Carried::Cone,
                        Some(Some(g)) => Carried::Read {
                            ray: ray_of(&g),
                            kind: g.class_invariants().map(|(_, _, k)| k),
                            modes: ModesRead::of(&g, budget),
                            after: s.after,
                        },
                    },
                })
                .collect(),
        })
        .collect();
    READ.with(|r| *r.borrow_mut() = Some(Circuits { circuits }));
}

/// The circuits of the last percept, and stop holding them. `None` where no
/// percept read any since the last take — a reading not taken, never an
/// empty one.
pub fn take_circuits() -> Option<Circuits> {
    READ.with(|r| r.borrow_mut().take())
}

/// ONE CIRCUIT, read off the steps its walk kept. A step's transport is
/// oriented the way the circuit runs before it is read; the composite runs
/// on through it, and a step that is unread or on the cone leaves every
/// later composite unread (rule 11).
pub(super) fn read_circuit(
    chord: u16,
    members: &[Vec<u8>],
    carried: &PathCarried,
    faces: &[FormFace],
) -> PendingCircuit {
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
            composite = match (&oriented, composite.take()) {
                (Some(Some(g)), Some(m)) => Some(g.geo(&m)),
                _ => None,
            };
            let after = composite.as_ref().and_then(|m| m.class_invariants()).map(|(_, _, k)| k);
            PendingStep { face: s.face, forward: s.forward, refracted, oriented, after }
        })
        .collect();
    PendingCircuit { chord, members: members.to_vec(), composite: carried.composite(), steps }
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
