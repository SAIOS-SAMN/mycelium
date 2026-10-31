// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26, mycelium#14 — A TASK'S ROUTES: each train pair's listing as the
//! intake read it — its residue's address, chart and magnitude, and the
//! pieces the court lists, each with its chart — read once, off the court,
//! and kept in the registry by the task's citation. A cover change
//! re-homes the corpus on these readings: it refines nothing and charts
//! nothing. A task's content is its identity (law 2), so what is read off
//! it stands while the task stands.
//!
//! Every chart is read on the intake's own thread. The cover walk runs on
//! the exchange thread, the court's one logger and stroker, and charted
//! there it went silent for minutes on a task whose chart takes seconds a
//! pair once refinement was no longer capped (MU-DERIVATIONS D3.2).

use crate::arc::Task;
use crate::server::registry::{TaskCitation, TaskRegistry};
use crate::transducer::address::Address;
use crate::transducer::element::Element;
use crate::transducer::stalk::Invariant;
use std::sync::{Arc, Mutex};

/// ONE TRAIN PAIR'S ROUTE: where the pair's listing homes is read off its
/// chart and magnitude; the court lists its pieces where it homes.
pub struct PairRoute {
    /// The residue's address: the listing's coordinates on the court.
    pub address: Address,
    /// The residue's chart.
    pub chart: Invariant,
    /// The residue's magnitude on the lanes S, G, T, H.
    pub magnitude: [u64; 4],
    /// The pieces the court lists, each residue with its chart
    /// (`cognition::prepare_intake`).
    pub pieces: Vec<(Element, Invariant)>,
}

/// A task's routes, one per pair that has one, shared by every walk that
/// hands them out.
pub type TaskRoutes = Arc<Vec<PairRoute>>;

/// THE ROUTES OF A TASK, read: each train pair refined under the stroke its
/// own extents need (no lcm is capped, 78.111), its chain in canonical
/// order, its residue formed in the pair's frame, charted, and cut into the
/// pieces the court lists. A pair with no common refinement or no A5 form
/// cannot be addressed, so it has no route (U14).
pub fn routes_of(task: &Task) -> Vec<PairRoute> {
    let mut out = Vec::with_capacity(task.train.len());
    for (a, b) in &task.train {
        let mut stroke = crate::physics::Physics::default().stroke_n(match crate::transducer::family::refine::need_of(a, b) {
            Some(n) => n,
            None => 0,
        });
        let Ok(crate::arc::Refined { delta: mut chain, frame, .. }) = crate::arc::refine_delta(a, b, &mut stroke) else {
            continue;
        };
        chain.sort_unstable();
        let Some(residue) = crate::transducer::element::from_residual_framed(&chain, &frame) else {
            continue;
        };
        out.push(PairRoute {
            address: residue.address(),
            chart: crate::transducer::stalk::invariant_of(&residue),
            magnitude: residue.magnitude(),
            pieces: crate::cognition::prepare_intake(&chain, &frame),
        });
    }
    out
}

/// A TASK A WALK HANDS THE LISTING STEP: its citation, the task (its train
/// binds the court's family) and its routes.
pub struct Walked {
    pub citation: TaskCitation,
    pub task: Task,
    pub routes: TaskRoutes,
}

/// The task's routes where the registry keeps them; otherwise read here,
/// with no registry lock held, and kept. A poisoned lock keeps nothing.
fn routed(registry: &Mutex<TaskRegistry>, citation: TaskCitation, task: Task) -> Walked {
    let kept = registry.lock().ok().and_then(|r| r.routes(&citation));
    let routes = match kept {
        Some(routes) => routes,
        None => {
            let read: TaskRoutes = Arc::new(routes_of(&task));
            match registry.lock() {
                Ok(mut r) => r.keep_routes(citation, read),
                Err(_) => read,
            }
        }
    };
    Walked { citation, task, routes }
}

/// FIRST-TIME INTAKE: the next `n` tasks of the manifest, each routed and
/// its routes kept.
pub fn intake_walk(registry: &Mutex<TaskRegistry>, n: usize) -> Vec<Walked> {
    let batch = match registry.lock() {
        Ok(mut r) => r.next_intake_batch(n),
        Err(_) => return Vec::new(),
    };
    batch.into_iter().map(|(citation, task)| routed(registry, citation, task)).collect()
}

/// A COVER CHANGE'S WALK: the next `n` tasks of the cover walk that the
/// registry holds routes for, read there — a lookup. A task not routed yet
/// is left to the intake's thread (`route_next`), which re-homes it once
/// routed.
pub fn cover_walk(registry: &Mutex<TaskRegistry>, n: usize) -> Vec<Walked> {
    let Ok(mut r) = registry.lock() else { return Vec::new() };
    let batch = r.next_cover_batch(n);
    batch
        .into_iter()
        .filter_map(|(citation, task)| {
            let routes = r.routes(&citation)?;
            Some(Walked { citation, task, routes })
        })
        .collect()
}

/// THE NEXT TASK NOT YET ROUTED, routed and kept: a court that restored its
/// listings took no intake, and its corpus is routed here, one task at a
/// time, on the intake's thread. `None` once every task is routed.
pub fn route_next(registry: &Mutex<TaskRegistry>) -> Option<Walked> {
    let (citation, task) = registry.lock().ok()?.unrouted()?;
    Some(routed(registry, citation, task))
}
