// 78.12(4), CS2 — A HELD FORM'S CIRCUITS, read off the steps its orbits'
// member paths cross: each step's face and the way it runs, the circuit's
// composite (OR3) and its hue, and why a composite did not read.

use super::*;

/// ONE TRAVERSED CIRCUIT: each step's face and whether it was crossed along the
/// face's orientation, and the circuit's composite (OR3).
#[derive(Clone, Debug)]
pub struct TraversedCircuit {
    pub steps: Vec<(coord::Overlap, bool)>,
    pub composite: Option<element::Versor>,
    /// 78.26(5) — the circuit's hue, its steps' hue maps composed as the
    /// composite composes their carriages; `None` where it did not read.
    pub hue: Option<Vec<(u8, u8)>>,
    /// Where the composite did not read, why: OR3's reason (78.26(4)).
    pub unread: Option<crate::transducer::cech::CircuitUnread>,
}

/// ONE CIRCUIT, read off the steps its traversal kept: each step's face and
/// the way it runs, the whole transport by the one composite reader, and
/// OR3's reason where it did not read — the edge half refused by name, a
/// held coset, or a refusal left unnamed (78.26(4)).
fn read_circuit(carried: &PathCarried) -> TraversedCircuit {
    let composite = carried.composite();
    let unread = composite.is_none().then(|| match (carried.holds_coset(), carried.edge_refused) {
        (true, _) => crate::transducer::cech::CircuitUnread::Coset,
        (false, Some(why)) => circuit_unread(why),
        (false, None) => crate::transducer::cech::CircuitUnread::Unnamed,
    });
    TraversedCircuit {
        steps: carried.transitions.iter().map(|s| (s.face.clone(), s.forward)).collect(),
        composite,
        hue: carried.hue(),
        unread,
    }
}

/// OR3's reason as the cell reads it.
fn circuit_unread(why: Unread) -> crate::transducer::cech::CircuitUnread {
    use crate::transducer::cech::CircuitUnread as Why;
    match why {
        Unread::Cone => Why::Cone,
        Unread::NotRigid => Why::NotRigid,
        Unread::Unrefracted => Why::Unrefracted,
        Unread::NoFace => Why::NoFace,
    }
}

/// CS2 — A HELD FORM'S CIRCUITS, read at the pivot off its own orbits: each
/// member path's steps, composite and OR3's reason, by the orbit's chord. An
/// orbit whose traversal refused stands as one unread circuit over its
/// faces, by its reason.
pub fn circuits_of(form: &Form) -> Vec<(coord::Overlap, TraversedCircuit)> {
    let mut out = Vec::new();
    for lp in &form.orbits {
        match paths_carried_why(&form.faces, &form.vertices, lp) {
            Ok(carried) => {
                for (_, c) in carried.scoped() {
                    out.push((lp.chord.clone(), read_circuit(c)));
                }
            }
            Err(why) => out.push((
                lp.chord.clone(),
                TraversedCircuit {
                    steps: lp.transitions.iter().map(|s| (s.overlap.clone(), s.forward)).collect(),
                    composite: None,
                    hue: None,
                    unread: Some(circuit_unread(why)),
                },
            )),
        }
    }
    out
}
