// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §1–§3 (proposal 78.171; MU-SENSING §3.7) — THE ATLAS: the
//! field's charts, its transitions as its connection, its attractors held
//! with their local orbits and membranes, and curvature as holonomy per lap.

use super::enclosing::Membrane;
use super::holonomy::{Holonomy, Motion};
use super::refusal::FieldRefusal;
use super::turning::FramePlanes;

/// A CHART (78.171 §2; MU-RECURSION 78.41), its one meaning: a body's exact
/// invariant coordinates, its shape class, coarsest first, written out. It
/// is never a surface the body is measured against. Ordered by its
/// coordinates as written: the order routes and never measures.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chart {
    coordinates: Vec<usize>,
}

impl Chart {
    /// The chart of these invariant coordinates, coarsest first. Built by
    /// the lift from a body and by a statement's reconstruction.
    pub(super) fn of(coordinates: Vec<usize>) -> Chart {
        Chart { coordinates }
    }

    /// The invariant coordinates, coarsest first.
    pub fn coordinates(&self) -> &[usize] {
        &self.coordinates
    }
}

/// A TRANSITION (78.171 §2): the motion that carries one chart onto
/// another. The transitions are the field's connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub from: Chart,
    pub to: Chart,
    pub motion: Motion,
}

/// AN ORBIT'S LAP (78.171 §2, §3.3): a path through charts that returns to
/// the chart it left on the base, with the transition at each step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Orbit {
    lap: Vec<Transition>,
}

impl Orbit {
    /// THE LAP OF `transitions`, each leaving the chart the one before
    /// reached and the last reaching the chart the first left. Refused
    /// `NotLap` with the step that breaks the chain, or `0` for no step.
    pub fn of(lap: Vec<Transition>) -> Result<Orbit, FieldRefusal> {
        let steps = lap.len();
        let broken = (0..steps).find(|&i| match (lap.get(i), lap.get((i + 1) % steps)) {
            (Some(a), Some(b)) => a.to != b.from,
            _ => true,
        });
        match (steps, broken) {
            (0, _) => Err(FieldRefusal::NotLap { at: 0 }),
            (_, Some(at)) => Err(FieldRefusal::NotLap { at: at + 1 }),
            (_, None) => Ok(Orbit { lap }),
        }
    }

    /// The transitions, in the lap's order.
    pub fn lap(&self) -> &[Transition] {
        &self.lap
    }

    /// The charts the lap leaves, in order.
    pub fn charts(&self) -> Vec<Chart> {
        self.lap.iter().map(|t| t.from.clone()).collect()
    }

    /// THE HOLONOMY (78.171 §3.1): the product of the lap's transitions,
    /// in order. Refused as [`Motion::then`] refuses.
    pub fn holonomy(&self) -> Result<Holonomy, FieldRefusal> {
        self.lap.iter().try_fold(Motion::identity(), |m, t| m.then(&t.motion)).map(Holonomy::of)
    }

    /// THE PRECESSION PER LAP (78.171 §3.3; MU-SENSING §3.9): the residue
    /// of the lap's transitions, its holonomy. Refused `Loop` with the
    /// lap's charts where the holonomy is the identity on every plane
    /// (`0°`, `K = 0`): a path that returns exactly is a loop, never an
    /// orbit.
    pub fn precession(&self) -> Result<Holonomy, FieldRefusal> {
        let h = self.holonomy()?;
        match h.turnings() {
            [] => Err(FieldRefusal::Loop { charts: self.charts() }),
            _ => Ok(h),
        }
    }
}

/// AN ATTRACTOR HELD (78.171 §2, a well): the chart it stands at, its local
/// orbit, and its membrane, the holding's own enclosure.
#[derive(Clone, Debug)]
pub struct Attractor {
    pub centre: Chart,
    pub orbit: Orbit,
    pub membrane: Membrane,
}

/// THE CURVATURE AROUND ONE ATTRACTOR (78.171 §2, a well): the holonomy
/// of its local orbit per lap, or the refusal that reading met.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curvature {
    pub centre: Chart,
    pub holonomy: Result<Holonomy, FieldRefusal>,
}

/// THE FIELD'S ATLAS (78.171 §1): its frame planes, its charts (each once,
/// in the order of their coordinates), and the attractors it holds. The
/// connection is the transitions of the orbits held, and nothing else: a
/// step no held orbit takes is flat.
#[derive(Clone, Debug)]
pub struct Atlas {
    frame: FramePlanes,
    charts: Vec<Chart>,
    attractors: Vec<Attractor>,
}

impl Atlas {
    /// THE ATLAS OF `charts` on `frame`, holding no attractor: flat.
    pub fn of(frame: FramePlanes, mut charts: Vec<Chart>) -> Atlas {
        charts.sort();
        charts.dedup();
        Atlas { frame, charts, attractors: Vec::new() }
    }

    /// The frame planes.
    pub fn frame(&self) -> &FramePlanes {
        &self.frame
    }

    /// The charts, in the order of their coordinates.
    pub fn charts(&self) -> &[Chart] {
        &self.charts
    }

    /// The attractors held, in the order held.
    pub fn attractors(&self) -> &[Attractor] {
        &self.attractors
    }

    /// Is `chart` one of the atlas's? Refused `NotCharted` otherwise.
    fn charted(&self, chart: &Chart) -> Result<(), FieldRefusal> {
        match self.charts.binary_search(chart) {
            Ok(_) => Ok(()),
            Err(_) => Err(FieldRefusal::NotCharted { chart: chart.clone() }),
        }
    }

    /// HOLD `attractor`: its centre and every chart its orbit passes must be
    /// the atlas's, refused `NotCharted` naming the first that is not, and
    /// its membrane must stand on one of the frame's planes.
    pub fn hold(&mut self, attractor: Attractor) -> Result<(), FieldRefusal> {
        self.charted(&attractor.centre)?;
        attractor.orbit.lap().iter().try_for_each(|t| self.charted(&t.from).and_then(|()| self.charted(&t.to)))?;
        if attractor.membrane.plane() > self.frame.count() {
            return Err(FieldRefusal::PastFrame { plane: attractor.membrane.plane() });
        }
        self.attractors.push(attractor);
        Ok(())
    }

    /// THE CONNECTION FROM `from` TO `to`: every held orbit's transition on
    /// that step composed, in the order held; the identity where none is
    /// held (flat). Refused as [`Motion::then`] refuses.
    pub fn connection(&self, from: &Chart, to: &Chart) -> Result<Motion, FieldRefusal> {
        self.attractors
            .iter()
            .flat_map(|a| a.orbit.lap())
            .filter(|t| &t.from == from && &t.to == to)
            .try_fold(Motion::identity(), |m, t| m.then(&t.motion))
    }

    /// THE LAP THROUGH `charts` IN THE FIELD (78.171 §3.1): each step's
    /// transition read from the connection, the last step returning to the
    /// first chart. Refused `NotCharted` for a chart the atlas does not
    /// hold, and as [`Orbit::of`] refuses.
    pub fn lap(&self, charts: &[Chart]) -> Result<Orbit, FieldRefusal> {
        charts.iter().try_for_each(|c| self.charted(c))?;
        let steps = charts.len();
        let lap = (0..steps)
            .filter_map(|i| Some((charts.get(i)?, charts.get((i + 1) % steps)?)))
            .map(|(from, to)| Ok(Transition { from: from.clone(), to: to.clone(), motion: self.connection(from, to)? }))
            .collect::<Result<Vec<_>, FieldRefusal>>()?;
        Orbit::of(lap)
    }

    /// THE FIELD'S CURVATURE (78.171 §2): around each attractor held, the
    /// precession of its local orbit per lap, in the order held. Flat
    /// where nothing is held: no attractor, no curvature.
    pub fn curvature(&self) -> Vec<Curvature> {
        self.attractors.iter().map(|a| Curvature { centre: a.centre.clone(), holonomy: a.orbit.precession() }).collect()
    }
}
