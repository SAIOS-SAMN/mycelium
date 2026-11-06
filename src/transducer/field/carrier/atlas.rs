// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §1–§3 (proposal 78.171; MU-SENSING §3.7) — THE ATLAS: the
//! field's charts, its transitions as its connection, its attractors held
//! with the geodesic orbits they bind and their membranes, and curvature as
//! holonomy per revolution.

use super::enclosing::Membrane;
use super::holonomy::{Holonomy, Motion};
use super::refusal::FieldRefusal;
use super::turning::FramePlanes;
use crate::transducer::stalk::{ChartAxis, Invariant};

/// A CHART (78.171 §2; MU-RECURSION 78.41), its one meaning: a body's exact
/// invariant coordinates, its shape class, coarsest first, written out:
/// the same coordinates a `stalk::Invariant` holds (a count or an exact
/// ratio each), so a well's chart and a lifted body's chart are one kind
/// of object. It is never a surface the body is measured against. Ordered
/// by its coordinates as written: the order routes and never measures.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chart {
    coordinates: Vec<ChartAxis>,
}

impl Chart {
    /// The chart of these invariant coordinates, coarsest first. Built by
    /// the lift from a body and by a statement's reconstruction.
    pub(super) fn of(coordinates: Vec<ChartAxis>) -> Chart {
        Chart { coordinates }
    }

    /// THE CHART AN INVARIANT NAMES: its coordinates, coarsest first, taken
    /// whole. Total: every invariant is a chart.
    pub fn of_invariant(invariant: &Invariant) -> Chart {
        Chart { coordinates: invariant.0.clone() }
    }

    /// The invariant coordinates, coarsest first.
    pub fn coordinates(&self) -> &[ChartAxis] {
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

/// AN ORBIT (78.171 §2, §3.3; MU-SENSING §3.9): a bound geodesic of the
/// field's gravitational curvature, read over one revolution, a path
/// through charts that returns to the chart it left on the base, with the
/// transition at each step. A path whose every step is the identity runs
/// through a flat region and binds no orbit: no `Orbit` holds one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Orbit {
    revolution: Vec<Transition>,
}

/// THE PRECESSION PER REVOLUTION (78.171 §3.3; MU-SENSING §3.9), the
/// residue of a revolution's transitions: an advance on the planes the
/// holonomy turns on, or none, where the closed geodesic returns exactly
/// in a curved field and the orbit is resonant, lawful and never refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Precession {
    Advances(Holonomy),
    Resonant,
}

impl Orbit {
    /// THE ORBIT OF ONE REVOLUTION through `revolution`'s transitions, each
    /// leaving the chart the one before reached and the last reaching the
    /// chart the first left. Refused `NotRevolution` with the step that
    /// breaks the chain, or `0` for no step, and `Unbound` with the charts
    /// where every step is the identity: a path through a flat region, no
    /// attractor binding it.
    pub fn of(revolution: Vec<Transition>) -> Result<Orbit, FieldRefusal> {
        let steps = revolution.len();
        let broken = (0..steps).find(|&i| match (revolution.get(i), revolution.get((i + 1) % steps)) {
            (Some(a), Some(b)) => a.to != b.from,
            _ => true,
        });
        let flat = revolution.iter().all(|t| t.motion.turnings().is_empty());
        match (steps, broken, flat) {
            (0, _, _) => Err(FieldRefusal::NotRevolution { at: 0 }),
            (_, Some(at), _) => Err(FieldRefusal::NotRevolution { at: at + 1 }),
            (_, None, true) => Err(FieldRefusal::Unbound { charts: revolution.iter().map(|t| t.from.clone()).collect() }),
            (_, None, false) => Ok(Orbit { revolution }),
        }
    }

    /// The transitions, in the revolution's order.
    pub fn revolution(&self) -> &[Transition] {
        &self.revolution
    }

    /// The charts the revolution leaves, in order.
    pub fn charts(&self) -> Vec<Chart> {
        self.revolution.iter().map(|t| t.from.clone()).collect()
    }

    /// THE HOLONOMY PER REVOLUTION (78.171 §3.1), the curvature the orbit
    /// reads: the product of its transitions, in order. Refused as
    /// [`Motion::then`] refuses.
    pub fn holonomy(&self) -> Result<Holonomy, FieldRefusal> {
        self.revolution.iter().try_fold(Motion::identity(), |m, t| m.then(&t.motion)).map(Holonomy::of)
    }

    /// THE PRECESSION PER REVOLUTION: the holonomy's advance where it turns
    /// on some plane, `Resonant` where it is the identity on every plane
    /// (`0°`, `K = 0`). Refused as [`Orbit::holonomy`] refuses.
    pub fn precession(&self) -> Result<Precession, FieldRefusal> {
        let h = self.holonomy()?;
        Ok(match h.turnings() {
            [] => Precession::Resonant,
            _ => Precession::Advances(h),
        })
    }
}

/// AN ATTRACTOR HELD (78.171 §2, a well): the chart it stands at, the
/// orbit it binds where it binds one (none in a flat region), and its
/// membrane, the holding's own enclosure.
#[derive(Clone, Debug)]
pub struct Attractor {
    pub centre: Chart,
    pub orbit: Option<Orbit>,
    pub membrane: Membrane,
}

impl Attractor {
    /// THE HOLONOMY OF ITS ORBIT PER REVOLUTION, the curvature around it.
    /// Refused `Unbound` with its centre where it binds no orbit.
    pub fn holonomy(&self) -> Result<Holonomy, FieldRefusal> {
        self.bound()?.holonomy()
    }

    /// THE PRECESSION OF ITS ORBIT PER REVOLUTION. Refused `Unbound` with
    /// its centre where it binds no orbit.
    pub fn precession(&self) -> Result<Precession, FieldRefusal> {
        self.bound()?.precession()
    }

    /// The orbit it binds, or `Unbound` with its centre.
    fn bound(&self) -> Result<&Orbit, FieldRefusal> {
        self.orbit.as_ref().ok_or_else(|| FieldRefusal::Unbound { charts: vec![self.centre.clone()] })
    }

    /// The transitions of the orbit it binds, none where it binds none.
    fn revolution(&self) -> &[Transition] {
        match &self.orbit {
            Some(o) => o.revolution(),
            None => &[],
        }
    }
}

/// THE CURVATURE AROUND ONE ATTRACTOR (78.171 §2, a well): the holonomy
/// of its orbit per revolution, or the refusal that reading met.
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
        attractor.revolution().iter().try_for_each(|t| self.charted(&t.from).and_then(|()| self.charted(&t.to)))?;
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
            .flat_map(Attractor::revolution)
            .filter(|t| &t.from == from && &t.to == to)
            .try_fold(Motion::identity(), |m, t| m.then(&t.motion))
    }

    /// THE GEODESIC THROUGH `charts` IN THE FIELD, one revolution (78.171
    /// §3.1): each step's transition read from the connection, the last
    /// step returning to the first chart. Refused `NotCharted` for a chart
    /// the atlas does not hold, and as [`Orbit::of`] refuses: `Unbound`
    /// where the path runs through a flat region.
    pub fn revolution(&self, charts: &[Chart]) -> Result<Orbit, FieldRefusal> {
        charts.iter().try_for_each(|c| self.charted(c))?;
        let steps = charts.len();
        let revolution = (0..steps)
            .filter_map(|i| Some((charts.get(i)?, charts.get((i + 1) % steps)?)))
            .map(|(from, to)| Ok(Transition { from: from.clone(), to: to.clone(), motion: self.connection(from, to)? }))
            .collect::<Result<Vec<_>, FieldRefusal>>()?;
        Orbit::of(revolution)
    }

    /// THE FIELD'S CURVATURE (78.171 §2): around each attractor held, the
    /// holonomy of its orbit per revolution, in the order held, refused
    /// `Unbound` around one binding no orbit. Flat where nothing is held:
    /// no attractor, no curvature.
    pub fn curvature(&self) -> Vec<Curvature> {
        self.attractors.iter().map(|a| Curvature { centre: a.centre.clone(), holonomy: a.holonomy() }).collect()
    }
}
