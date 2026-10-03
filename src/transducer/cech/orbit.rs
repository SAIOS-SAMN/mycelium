// 78.73 — THE ELASTIC VENT OF 𝒮, 𝒢 AND 𝒽: every deformation a field line
// carries is held on its orbit until the orbit returns, and the return is
// the vent. The tubes (`flux.rs`) are 𝒯's; these are the other three lanes'.
//
//! A medium under stress deforms ELASTICALLY (it returns), PLASTICALLY (it
//! keeps the new shape — a generator admitted) or FRACTURES (a draw refused).
//! The elastic part is read here, on each mode's own orbit and never on the
//! sensor's grid:
//!
//! - **A turn (𝒽) or a boost (𝒮)** is a mode of a motion (MR1) on its plane,
//!   and the orbit is the plane's FLOW (78.75): it holds the point the flow
//!   has reached, every admitted mode composed into it exactly
//!   ([`cga::FlowPoint::then`]) — any amount, any sign, never a count of one
//!   fixed angle. A turn of no finite order (a rosette) keeps flowing and its
//!   point is content, never capped (78.43). The orbit has returned where its
//!   point is the identity; it stands nearer its return where `|1 − c|` fell.
//!   A mode spanning several planes at once (an isoclinic set) is held as its
//!   accumulated ray, returned where that ray is the identity.
//! - **A hue map (𝒢)** is a map of the body's hue fibre. Maps moving one
//!   support compose; the orbit has returned when the composite is the
//!   identity. Its strain is the hues it holds displaced — never the cells
//!   a recolour covers, which is area on the lattice.
//!
//! Each admission either OPENS (the orbit moves away from its return, or no
//! nearer), CLOSES (it moves nearer: relieving what stood open, up to its own
//! mass), or RETURNS (the orbit closed: everything that stood open on it is
//! relieved and the orbit is let go, so the carrier keeps its bound). Open
//! is pressure, closure is order, a return is the vent.

use super::flux::Admitted;
use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use crate::transducer::cga::{self, MotionMode, Mv};
use crate::transducer::rewrite::{LANE_GAUGE, LANE_SHELL, LANE_SPATIAL};
use num_rational::BigRational;
use num_traits::Zero;

/// 78.73 — A FIELD HELD OVER TIME: 𝒯's tubes and the orbits of 𝒮, 𝒢 and
/// 𝒽, one per mind (a voice's process, a court's kernel).
#[derive(Clone, Debug, Default)]
pub struct Field {
    pub tubes: super::flux::TubeRegistry,
    pub orbits: OrbitRegistry,
}

impl Field {
    pub fn new() -> Field {
        Field::default()
    }

    /// 78.73 (4) — THE ELASTIC STRAIN'S TWO TERMS PER LANE, weighed by mass:
    /// what stands open and what has moved — 𝒯's off its tubes, the other
    /// lanes' off their orbits. `ε = open / transport`, exact.
    pub fn elastic(&self) -> [(num_bigint::BigInt, num_bigint::BigInt); 4] {
        let (open, transport) = self.orbits.totals();
        let (_, tube_transport, tube_open) = self.tubes.totals();
        std::array::from_fn(|l| match l {
            crate::transducer::rewrite::LANE_TEMPORAL => (tube_open.clone(), tube_transport.clone()),
            _ => (open[l].into(), transport[l].into()),
        })
    }
}

/// Where an orbit stands.
#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    /// A turn or a boost on one plane: the point its flow has reached.
    Flow(cga::FlowPoint),
    /// A mode on several planes at once: its accumulated ray.
    Ray(Mv),
    /// A hue map: where each hue of the support has been carried.
    Hues(Vec<(u8, u8)>),
}

impl State {
    /// HOW FAR THE ORBIT STANDS FROM ITS RETURN, exact: `|1 − c|` on the
    /// flow; for a ray, none at the identity and one away from it (open until
    /// it returns); the hues the composite holds displaced. Zero is the
    /// return. `None` where the flow's point has no value (a boost on its
    /// plane's light cone).
    fn distance(&self) -> Option<BigRational> {
        match self {
            State::Flow(p) => p.distance(),
            State::Ray(r) => Some(match r.projective_eq(&Mv::scalar(BigRational::from_integer(1.into()))) {
                true => BigRational::zero(),
                false => BigRational::from_integer(1.into()),
            }),
            State::Hues(map) => Some(BigRational::from_integer(map.iter().filter(|(h, to)| h != to).count().into())),
        }
    }
}

#[derive(Clone, Debug)]
struct Orbit {
    state: State,
    open: u64,
}

/// 78.73 — THE ORBITS HELD OVER TIME on 𝒮, 𝒢 and 𝒽, with what stands open
/// and what has moved per lane. A returned orbit is let go; the lane totals
/// keep what it did.
#[derive(Clone, Debug, Default)]
pub struct OrbitRegistry {
    orbits: Keyed<Orbit>,
    open: [u64; 4],
    transport: [u64; 4],
}

/// A mode's key: its lane and its plane's canonical coordinates — ONE ORBIT
/// PER PLANE, every amount flowing on it composed into one point (78.75). It
/// keyed the rational point too, so two turns of different angle in one
/// plane stood on two orbits and neither could ever bring the other back.
fn mode_key(lane: usize, plane: &Mv) -> Option<Vec<u8>> {
    let mut key = vec![u8::try_from(lane).ok()?];
    plane.canonical()?.put_canonical(&mut key);
    Some(key)
}

impl OrbitRegistry {
    pub fn new() -> OrbitRegistry {
        OrbitRegistry::default()
    }

    /// How many orbits stand open.
    pub fn len(&self) -> usize {
        self.orbits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.orbits.is_empty()
    }

    /// Per lane: what stands open, and what has moved, weighed by mass.
    pub fn totals(&self) -> ([u64; 4], [u64; 4]) {
        (self.open, self.transport)
    }

    /// ADMIT ONE MODE of a field line carrying `mass`: a turn onto 𝒽's orbit
    /// of its plane, a boost onto 𝒮's, the mode's point on the plane's flow
    /// composed into where the orbit stands (78.75). `None` for a null mode
    /// (the tubes hold those), and where the mode's point does not read on the
    /// orbit's plane or an amount passes its carrier.
    pub fn admit_mode(&mut self, mode: &MotionMode, mass: u64) -> Option<(usize, Admitted)> {
        let lane = match mode.kind {
            BivectorKind::Elliptic => LANE_SHELL,
            BivectorKind::Hyperbolic => LANE_SPATIAL,
            BivectorKind::Parabolic | BivectorKind::Trivial => return None,
        };
        let (key, before, after) = match mode.flow() {
            Some(point) => {
                let key = mode_key(lane, point.plane())?;
                let before = match self.orbits.get(&key).map(|o| o.state.clone()) {
                    Some(State::Flow(held)) => held,
                    Some(State::Ray(_) | State::Hues(_)) => return None,
                    None => cga::FlowPoint::origin(point.plane())?,
                };
                let after = before.then(&point)?;
                (key, State::Flow(before), State::Flow(after))
            }
            // An isoclinic set: its planes flow together, held as one ray.
            None => {
                let key = mode_key(lane, &mode.ray.grade(2))?;
                let before = match self.orbits.get(&key).map(|o| o.state.clone()) {
                    Some(State::Ray(held)) => held,
                    Some(State::Flow(_) | State::Hues(_)) => return None,
                    None => Mv::scalar(BigRational::from_integer(1.into())),
                };
                let after = before.geo(&mode.ray);
                let after = match after.canonical() {
                    Some(ray) => ray,
                    None => after,
                };
                (key, State::Ray(before), State::Ray(after))
            }
        };
        Some((lane, self.step(&key, lane, &before, after, mass)?))
    }

    /// ADMIT ONE HUE MAP carrying `mass` onto 𝒢's orbit of its support.
    /// `None` where it moves no hue (the identity carries no strain), where
    /// it is not a map (one hue sent two places), or an amount passes its
    /// carrier.
    pub fn admit_hues(&mut self, gauge: &[(u8, u8)], mass: u64) -> Option<Admitted> {
        let mut moved: Vec<(u8, u8)> = gauge.iter().copied().filter(|(h, to)| h != to).collect();
        moved.sort_unstable();
        moved.dedup();
        if moved.is_empty() || moved.windows(2).any(|p| p[0].0 == p[1].0) {
            return None;
        }
        let sigma = |h: u8| match moved.binary_search_by_key(&h, |(from, _)| *from) {
            Ok(at) => moved[at].1,
            Err(_) => h,
        };
        let mut key = vec![u8::try_from(LANE_GAUGE).ok()?];
        key.extend(moved.iter().map(|(h, _)| *h));
        let held = match self.orbits.get(&key).map(|o| o.state.clone()) {
            Some(State::Hues(map)) => map,
            Some(State::Flow(_) | State::Ray(_)) => return None,
            None => moved.iter().map(|(h, _)| (*h, *h)).collect(),
        };
        let after: Vec<(u8, u8)> = held.iter().map(|(h, at)| (*h, sigma(*at))).collect();
        self.step(&key, LANE_GAUGE, &State::Hues(held), State::Hues(after), mass)
    }

    /// ONE ADMISSION ON ONE ORBIT: open, close, or return, exactly.
    fn step(&mut self, key: &[u8], lane: usize, before: &State, after: State, mass: u64) -> Option<Admitted> {
        let transport = self.transport[lane].checked_add(mass)?;
        let held_open = self.orbits.get(key).map_or(0, |o| o.open);
        let (d0, d1) = (before.distance()?, after.distance()?);
        // What stands open on the orbit after, `None` where it returned.
        let (moved, left) = match () {
            // THE RETURN: the orbit closed; all that stood open on it is
            // relieved and the orbit is let go.
            _ if d1.is_zero() => (Admitted { closed: mass, opened: 0, relieved: held_open, transport: mass }, None),
            // NEARER ITS RETURN: order, relieving up to its own mass.
            _ if d1 < d0 => {
                let relieved = held_open.min(mass);
                (Admitted { closed: mass, opened: 0, relieved, transport: mass }, Some(held_open - relieved))
            }
            // AWAY, OR NO NEARER: the deformation stands open.
            _ => (Admitted { closed: 0, opened: mass, relieved: 0, transport: mass }, Some(held_open.checked_add(mass)?)),
        };
        // Every total is stepped before anything is written, so an amount
        // past its carrier leaves the registry as it stood (rule 11).
        let lane_open = self.open[lane].checked_add(moved.opened)?.checked_sub(moved.relieved)?;
        match left {
            None => {
                self.orbits.remove(key);
            }
            Some(open) => {
                self.orbits.insert(key, Orbit { state: after, open });
            }
        }
        self.open[lane] = lane_open;
        self.transport[lane] = transport;
        Some(moved)
    }
}
