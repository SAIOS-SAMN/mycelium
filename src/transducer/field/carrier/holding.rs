// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §2 (a well; proposal 78.171), MU-SENSING §3.7, ROADMAP track
//! 26 S7 stage 2 — A WELL'S BODY: a held attractor at a chart of the field's
//! atlas, its membrane its own enclosure, its stage a coordinate, its
//! inflation the gauge of a frame state read through its pole basis.

use super::atlas::{Atlas, Attractor, Chart, Orbit, Precession};
use super::enclosing::{stand, Labels, Membrane};
use super::holonomy::Holonomy;
use super::refusal::FieldRefusal;
use super::shares::{frame_plane, Cohesion};
use super::turning::FramePlanes;
use crate::core::Q;
use forms::passage::{Frame, Place};

/// THE LAYER A WELL STANDS AT (MU-CLIENT §4, N4): a held percept and a
/// held thought are one well at two stages (78.171 §2), perception first.
/// A perception never passes the unglued door; a thought is formed where
/// two perceptions' bodies are joined by a transition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum WellLayer {
    Perception,
    #[default]
    Thought,
}

/// WHERE A WELL STANDS ON THE CONTEMPLATION'S EXPANSION RING: on it, its
/// charts still to be met (held, re-marked, or its expansion stopped by
/// the budget), or rested, its last expansion having met what it could.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpansionRing {
    Expanding,
    #[default]
    Rested,
}

/// A WELL'S STAGE, a coordinate (ROADMAP track 26 S7): its layer, then its
/// place on the expansion ring, ordered so. Read where a door or the ring
/// asks where the well stands; never a flag beside the well.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WellStage {
    pub layer: WellLayer,
    pub ring: ExpansionRing,
}

impl WellStage {
    /// A PERCEPTION JUST HELD: at the perception layer, on the ring.
    pub fn perceived() -> WellStage {
        WellStage { layer: WellLayer::Perception, ring: ExpansionRing::Expanding }
    }

    /// A THOUGHT JUST HELD: at the thought layer, on the ring.
    pub fn thought() -> WellStage {
        WellStage { layer: WellLayer::Thought, ring: ExpansionRing::Expanding }
    }

    /// THE SAME LAYER, PUT BACK ON THE RING: a class arrived in its horizon
    /// or a hold renewed it, so its charts are met again.
    pub fn reopened(self) -> WellStage {
        WellStage { ring: ExpansionRing::Expanding, ..self }
    }

    /// THE SAME LAYER AT `ring`, where an expansion left it.
    pub fn at(self, ring: ExpansionRing) -> WellStage {
        WellStage { ring, ..self }
    }
}

/// A WELL'S INFLATION (MU-SENSING §3.7, *deflate and re-inflate*): the
/// gauge of the frame state `H·n_Z + R·n_Q` on its membrane's plane, read
/// back through the pole basis, and where it stands: at the throat (the
/// membrane itself, `H = R`), at a pole, or between. The open pole is
/// infinity and stands only because the well's own membrane encloses it.
///
/// THE TWO SHARES OF ITS PLANE (the ruling of 2026-11-06): the held share
/// `H/(H + R)` (`Frame::share`, the closed count `A` over `A + Q`) is the
/// plane's cohesion, its viscosity; the open share `R/(H + R)` is its
/// fluidity, never its viscosity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inflation {
    pub gauge: Frame,
    pub place: Place,
}

/// A WELL'S BODY (78.171 §2, a well): the curvature around one attractor.
/// It holds the frame planes it stands on, the attractor (its centre, a
/// chart; the geodesic orbit it binds, none in a flat region; its
/// membrane, the throat of the planes' own last plane, as a lifted
/// sample's is), and its stage. No mass, radius or count is held: its
/// holonomy, precession and inflation are readings.
#[derive(Clone, Debug)]
pub struct WellBody {
    planes: FramePlanes,
    attractor: Attractor,
    stage: WellStage,
}

impl WellBody {
    /// THE BODY OF A WELL AT `centre` on `planes`: its membrane the throat
    /// of the planes' last plane, the orbit it binds `orbit` (none: flat).
    /// Refused as the membrane refuses its plane.
    pub fn of(planes: FramePlanes, centre: Chart, orbit: Option<Orbit>, stage: WellStage) -> Result<WellBody, FieldRefusal> {
        let membrane = Membrane::of(&planes, planes.count())?;
        WellBody::held(planes, Attractor { centre, orbit, membrane }, stage)
    }

    /// THE BODY OF A HELD ATTRACTOR, as a statement reconstructs it: held
    /// in its own atlas first, refused as [`Atlas::hold`] refuses.
    pub(super) fn held(planes: FramePlanes, attractor: Attractor, stage: WellStage) -> Result<WellBody, FieldRefusal> {
        let body = WellBody { planes, attractor, stage };
        body.atlas()?;
        Ok(body)
    }

    /// THE WELL'S OWN ATLAS: its centre and the charts its orbit passes, on
    /// its planes, holding its one attractor.
    pub fn atlas(&self) -> Result<Atlas, FieldRefusal> {
        let mut charts = vec![self.attractor.centre.clone()];
        charts.extend(self.attractor.orbit.iter().flat_map(Orbit::charts));
        let mut atlas = Atlas::of(self.planes, charts);
        atlas.hold(self.attractor.clone())?;
        Ok(atlas)
    }

    /// The frame planes the well stands on.
    pub fn planes(&self) -> &FramePlanes {
        &self.planes
    }

    /// The attractor the well holds.
    pub fn attractor(&self) -> &Attractor {
        &self.attractor
    }

    /// The chart the well stands at.
    pub fn centre(&self) -> &Chart {
        &self.attractor.centre
    }

    /// The membrane, the well's own enclosure.
    pub fn membrane(&self) -> &Membrane {
        &self.attractor.membrane
    }

    /// Where the well stands.
    pub fn stage(&self) -> WellStage {
        self.stage
    }

    /// THE HOLONOMY OF ITS ORBIT PER REVOLUTION (78.171 §3.1), the
    /// curvature around it. Refused `Unbound` with its centre where it
    /// binds no orbit.
    pub fn holonomy(&self) -> Result<Holonomy, FieldRefusal> {
        self.attractor.holonomy()
    }

    /// THE PRECESSION PER REVOLUTION (78.171 §3.3): its orbit's advance, or
    /// `Resonant` where the closed geodesic returns exactly. Refused
    /// `Unbound` with its centre where it binds no orbit.
    pub fn precession(&self) -> Result<Precession, FieldRefusal> {
        self.attractor.precession()
    }

    /// THE INFLATION AT `hold` AND `reach`: the frame state built on the
    /// membrane's pole basis (its held share the viscosity, its open share
    /// the fluidity), stood inside the membrane (so its open pole,
    /// infinity, is enclosed), and read back as its gauge and place.
    /// Refused as the pole basis refuses the state.
    pub fn inflation(&self, hold: Q, reach: Q) -> Result<Inflation, FieldRefusal> {
        let membrane = &self.attractor.membrane;
        let poles = membrane.poles();
        let state = poles.state(&Frame { hold, reach }).map_err(FieldRefusal::Passage)?;
        let place = stand(&state, poles, Labels::Standard, Some(membrane))?;
        let gauge = poles.read(&state).map_err(FieldRefusal::Passage)?;
        Ok(Inflation { gauge, place })
    }

    /// THE COHESION AT `hold` AND `reach` (MU-ANASTOME §9.4; #149): the
    /// inflation's gauge read per plane it stands on, the membrane's frame
    /// plane, as its held share `H/(H + R)`; a keyed reading, never stored
    /// and never summed. Refused as [`WellBody::inflation`] and
    /// [`Cohesion::of`] refuse.
    pub fn cohesion(&self, hold: Q, reach: Q) -> Result<Cohesion, FieldRefusal> {
        let gauge = self.inflation(hold, reach)?.gauge;
        Cohesion::of(&[(frame_plane(self.membrane().plane())?, gauge)])
    }
}
