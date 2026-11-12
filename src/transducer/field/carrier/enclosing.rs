// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §1a, §4 `Unenclosed` (proposal 78.171 amendments (a), (d)) —
//! THE MEMBRANE: a holding's own enclosure, the throat's fixed sphere on one
//! frame plane's pole basis; infinity, a plane's pole, stands only inside one.

use super::refusal::FieldRefusal;
use super::turning::FramePlanes;
use crate::core::Q;
use crate::transducer::cga::Clifford;
use forms::coordinate::Square;
use forms::passage::{PassageState, Place, Pole, Poles};
use num_traits::Zero;

/// WHICH POLE OF A PLANE IS INFINITY, read from the state (amendment (b)):
/// as the standard labels it, `n_∞ = n_Q = e₊ − e₋` (the open pole), or
/// past the state's pole swap `e₋ ↦ −e₋`, where infinity is `n_Z`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Labels {
    Standard,
    Swapped,
}

impl Labels {
    /// The pole these labels make infinity.
    pub fn infinity(self) -> Pole {
        match self {
            Labels::Standard => Pole::Open,
            Labels::Swapped => Pole::Closed,
        }
    }
}

/// A MEMBRANE (78.171 §1a, §2): the throat of one frame plane, `H = R`,
/// the frame state on `e₊` alone, the fixed sphere of the pole swap and
/// never null. It is a holding's own enclosure (a well's, a sample's),
/// never a reference surface anything is measured against. A pole of the
/// plane it stands on is enclosed by it.
#[derive(Clone, Debug)]
pub struct Membrane {
    plane: u32,
    poles: Poles,
    throat: Clifford,
}

impl Membrane {
    /// THE MEMBRANE ON FRAME PLANE `k` of `frame`: the throat state at the
    /// gauge `H = R = ½`, which is `e₊ₖ` itself. Refused `PastFrame` past
    /// the frame, and as the pole basis refuses the plane's units.
    pub fn of(frame: &FramePlanes, k: u32) -> Result<Membrane, FieldRefusal> {
        let (positive, negative) = (frame.unit_value(Square::Positive, k)?, frame.unit_value(Square::Negative, k)?);
        let poles = Poles::of(&positive, &negative).map_err(FieldRefusal::Passage)?;
        let half = algebra::multivector::half();
        let throat = poles.state(&PassageState { hold: half.clone(), reach: half }).map_err(FieldRefusal::Passage)?;
        Ok(Membrane { plane: k, poles, throat })
    }

    /// The frame plane the membrane stands on.
    pub fn plane(&self) -> u32 {
        self.plane
    }

    /// The plane's pole basis.
    pub fn poles(&self) -> &Poles {
        &self.poles
    }

    /// The throat state, `e₊` of its plane.
    pub fn throat(&self) -> &Clifford {
        &self.throat
    }

    /// THE INVERSION IN THE MEMBRANE (MU-SENSING §3.7, *flip*; ROADMAP track
    /// 26 S8): a frame state on its plane carried by the throat's sandwich,
    /// `x ↦ e₊·x·e₊`, read from the membrane alone, so no infinity is
    /// written outside an enclosure: it is the pole swap, `n_Z ↔ n_Q`.
    /// Refused as the pole basis refuses `x`.
    pub fn invert(&self, x: &Clifford) -> Result<Clifford, FieldRefusal> {
        self.poles.read(x).map_err(FieldRefusal::Passage)?;
        self.throat.product(x).and_then(|tx| tx.product(&self.throat)).map_err(FieldRefusal::Mismatched)
    }

    /// WHERE THE THROAT STANDS, read through the pole basis: at the throat,
    /// always (`membrane_is_the_throat_never_null`).
    pub fn place(&self) -> Result<Place, FieldRefusal> {
        self.poles.read(&self.throat).map(|f| f.place()).map_err(FieldRefusal::Passage)
    }
}

/// A FRAME STATE ON `poles`' PLANE STOOD IN THE FIELD (78.171 §4,
/// `Unenclosed`): read through the pole basis, its place returned. Where it
/// stands at the pole `labels` make infinity, it is lawful only inside an
/// enclosure: `enclosure` must be a membrane on the same plane; refused
/// `Unenclosed` with the pole otherwise. Every other place stands
/// anywhere. Refused as the pole basis refuses the state.
pub fn stand(state: &Clifford, poles: &Poles, labels: Labels, enclosure: Option<&Membrane>) -> Result<Place, FieldRefusal> {
    let place = poles.read(state).map_err(FieldRefusal::Passage)?.place();
    let infinity = labels.infinity();
    match (place, enclosure.filter(|m| m.poles() == poles)) {
        (Place::Pole(pole), None) if pole == infinity => Err(FieldRefusal::Unenclosed { pole }),
        _ => Ok(place),
    }
}

/// THE STATE `c·n` AT ONE POLE of `poles`' plane: `c·(e₊ + e₋)` closed,
/// `c·(e₊ − e₋)` open (78.172 §1), at the gauge `c`.
pub fn at_pole(poles: &Poles, pole: Pole, c: Q) -> Result<Clifford, FieldRefusal> {
    let frame = match pole {
        Pole::Closed => PassageState { hold: c, reach: Q::zero() },
        Pole::Open => PassageState { hold: Q::zero(), reach: c },
    };
    poles.state(&frame).map_err(FieldRefusal::Passage)
}
