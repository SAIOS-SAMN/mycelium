// 78.12(4), PL1 — THE CIRCUIT'S TRAJECTORY READ STEP BY STEP, off the steps
// the walk kept: each step oriented the way the circuit runs, its ray, its
// class and modes, and the class of the transport composed through it.

use super::*;
use crate::observe::{ArmRead, Carried, Circuit, CircuitStep, CircuitTest, Circuits, TestArm};

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
    /// Its whole transport by the one composite reader, `None` where the
    /// edge half was refused (OR3) or rides a held coset.
    composite: Option<element::Versor>,
    /// 78.6 — how its held coset closes, and whether its gauge is static.
    coset: Option<super::walk::CosetClosure>,
    gauge_static: bool,
    steps: Vec<PendingStep>,
    /// CT1 — the reuse ladder's arms read off the faces, and whether the
    /// circuit read trivial; `None` where nothing triggered the ladder.
    ladder: Option<(Vec<(TestArm, ArmRead)>, bool)>,
}

/// CT1, 79.3 — THE LADDER ON ONE CIRCUIT, by its refusal's own type, each
/// arm one keyed read of the faces: a refused circuit's lost face — its other
/// chart pairs, its tether, its coset or static refraction — or, on a cone,
/// nothing and permanently. A trivial circuit is seeded empty; its arm, the
/// other circuits crossing its faces, is read over every circuit kept.
/// `None` for a circuit that read a class of its own or rides a coset.
fn ladder_of(carried: &PathCarried, faces: &[FormFace]) -> Option<(Vec<(TestArm, ArmRead)>, bool)> {
    let composite = carried.composite();
    let identity = element::identity_versor();
    if let Some(m) = &composite {
        return element::same_versor(m, &identity).then(|| (Vec::new(), true));
    }
    if carried.coset().is_some() {
        return None;
    }
    if carried.edge_refused == Some(Unread::Cone) {
        return Some((vec![(TestArm::Cone, ArmRead::Permanent)], false));
    }
    let lost = carried
        .edge_lost_at
        .and_then(|k| carried.steps.get(k.checked_sub(1)?))
        .and_then(|s| faces.get(usize::from(s.face)));
    let reads = |n: usize| match n {
        0 => ArmRead::Empty,
        n => ArmRead::Reads(n as u32),
    };
    let places = |pred: &dyn Fn(&FacePlace) -> bool| lost.map_or(0, |f| f.at.scoped().filter(|(_, p)| pred(p)).count());
    let mut arms = vec![(TestArm::OtherPairs, reads(places(&|p| p.edge.is_some())))];
    match carried.edge_refused {
        Some(Unread::NotRigid) => arms.push((TestArm::Tether, reads(places(&|p| p.refraction.is_some())))),
        Some(Unread::Unrefracted) => {
            arms.push((TestArm::Coset, reads(places(&|p| matches!(p.refraction, Some(Refraction::Coset(_)))))));
            arms.push((TestArm::Static, reads(places(&|p| matches!(p.refraction, Some(Refraction::Static(_)))))));
        }
        _ => {}
    }
    Some((arms, false))
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

/// ONE WALKED CIRCUIT: each step's face and whether it was crossed along the
/// face's orientation, the circuit's composite (OR3), how its held coset
/// closes where it rides one, and whether its gauge half is static (78.6).
#[derive(Clone, Debug)]
pub struct WalkedCircuit {
    pub steps: Vec<(u16, bool)>,
    pub composite: Option<element::Versor>,
    pub coset: Option<super::walk::CosetClosure>,
    pub gauge_static: bool,
}

/// THE CIRCUITS THE PERCEPT WALKED, each by the faces its steps crossed and
/// its composite — what a cell's orbit algebra reads (CG1): a step's body
/// is read off the face it crosses (BD2), never derived from its member.
/// Taken off the walk that already ran, never by walking again; empty where
/// nobody armed the list.
pub fn walked_circuits() -> Vec<WalkedCircuit> {
    PENDING.with(|c| match c.borrow().as_ref() {
        Some(held) => held
            .iter()
            .map(|p| WalkedCircuit {
                steps: p.steps.iter().map(|s| (s.face, s.forward)).collect(),
                composite: p.composite.clone(),
                coset: p.coset,
                gauge_static: p.gauge_static,
            })
            .collect(),
        None => Vec::new(),
    })
}

/// READ THE CIRCUITS THE PERCEPT WALKED, each step's modes under the
/// stroke's budget (BD1), and hold them for the observer; stop listening.
/// A percept that armed nothing reads nothing and leaves what is held.
/// Returns the steps whose modes the stroke left unbounded (SP1).
pub fn finish_circuits(budget: &mut crate::physics::Budget) -> u64 {
    let Some(pending) = PENDING.with(|c| c.borrow_mut().take()) else { return 0 };
    // CT1 — EACH TRIGGERED CIRCUIT'S LADDER, finished: a trivial one's other
    // routes are the other kept circuits crossing any of its faces, each read
    // against the identity; an unread one is no route (79.3).
    let identity = element::identity_versor();
    let tests: Vec<Option<CircuitTest>> = pending
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let (mut arms, trivial) = p.ladder.clone()?;
            if trivial {
                let (mut agree, mut disagree) = (0u32, 0u32);
                for (j, q) in pending.iter().enumerate() {
                    if j == i || !q.steps.iter().any(|s| p.steps.iter().any(|t| t.face == s.face)) {
                        continue;
                    }
                    match &q.composite {
                        Some(m) if element::same_versor(m, &identity) => agree += 1,
                        Some(_) => disagree += 1,
                        None => {}
                    }
                }
                let read = match agree + disagree {
                    0 => ArmRead::Empty,
                    _ => ArmRead::Routes { agree, disagree },
                };
                arms.push((TestArm::OtherCircuits, read));
            }
            Some(CircuitTest::classify(arms, trivial))
        })
        .collect();
    let circuits: Vec<Circuit> = pending
        .into_iter()
        .zip(tests)
        .map(|(p, test)| Circuit {
            test,
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
    let unbounded = circuits
        .iter()
        .flat_map(|c| &c.steps)
        .filter(|s| {
            matches!(
                s.carried,
                Carried::Read { modes: ModesRead::Undecomposed(crate::transducer::cga::Undecomposed::Unbounded(_)), .. }
            )
        })
        .count() as u64;
    READ.with(|r| *r.borrow_mut() = Some(Circuits { circuits }));
    unbounded
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
pub(super) fn read_circuit(chord: u16, carried: &PathCarried, faces: &[FormFace]) -> PendingCircuit {
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
    PendingCircuit {
        chord,
        composite: carried.composite(),
        coset: carried.coset_closure(),
        gauge_static: carried.gauge_static(),
        steps,
        ladder: ladder_of(carried, faces),
    }
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
