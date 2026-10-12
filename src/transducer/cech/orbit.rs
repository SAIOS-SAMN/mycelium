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

/// MU-CLIENT §4.5 — HOW A FIELD LINE STANDS TO A LINE OF THE COMPLEX (a
/// tether or a cell, by its coordinates): WOUND in — it threads every tube
/// and orbit its modes land on; UNWOUND — its reverse lands on the same rays
/// and planes and it leaves them; FREE — it belongs to no line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Thread {
    Free,
    Wound(Vec<u8>),
    Unwound(Vec<u8>),
}

/// The lines threading a tube or an orbit, in key order, as `thread` moves
/// them.
pub(crate) fn thread_on(lines: &mut Vec<Vec<u8>>, thread: &Thread) {
    match thread {
        Thread::Free => {}
        Thread::Wound(k) => {
            if let Err(at) = lines.binary_search(k) {
                lines.insert(at, k.clone());
            }
        }
        Thread::Unwound(k) => {
            if let Ok(at) = lines.binary_search(k) {
                lines.remove(at);
            }
        }
    }
}

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

    /// MU-CLIENT §4.5 — THE LINES THE FIELD RELEASED since the last take:
    /// those threading an orbit that RETURNED (harmonic — its tension vented)
    /// and those threading a tube whose net flux CANCELLED (luminal — it
    /// carries no field). Each is a line of the complex whose tension relaxed.
    /// MU-CLIENT §4.5 — THE FIELD BEATS: where an orbit returned or a tube's
    /// net flux cancelled since the last beat, what never returns dissipates
    /// into the reservoir — every boost and null orbit whole, every tube's
    /// open flux — and waits there to re-form. Read after every admission;
    /// with no return, nothing moves.
    pub fn beat(&mut self) {
        let returned = self.orbits.take_returned();
        let cancelled = self.tubes.take_cancelled();
        if returned || cancelled {
            self.orbits.dissipate();
            self.tubes.dissipate();
        }
    }

    pub fn take_released(&mut self) -> Vec<Vec<u8>> {
        let mut out = std::mem::take(&mut self.orbits.released);
        out.append(&mut self.tubes.released);
        out
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

    /// WHETHER IT RINGS: a turn — of finite order or a rosette — an isoclinic
    /// ray and a hue map can each come back, and ring (HARMONIC); a boost and
    /// a null flow never return, and are what a beat of the field dissipates.
    fn rings(&self) -> bool {
        match self {
            State::Flow(p) => p.kind() == cga::FlowKind::Turn,
            State::Ray(_) | State::Hues(_) => true,
        }
    }
}

#[derive(Clone, Debug)]
struct Orbit {
    state: State,
    open: u64,
    /// The lines of the complex threading it.
    lines: Vec<Vec<u8>>,
}

/// 78.73 — THE ORBITS HELD OVER TIME on 𝒮, 𝒢 and 𝒽, with what stands open
/// and what has moved per lane. A returned orbit is let go, and the lines
/// that threaded it are released; the lane totals keep what it did.
///
/// THE RESERVOIR `e_∅` (78.58, MU-CLIENT §4.5): an orbit that never returns,
/// dissipated on a beat of the field, kept whole at its own coordinates —
/// its point and the strain that stood open on it — until a motion landing
/// there re-forms it.
#[derive(Clone, Debug, Default)]
pub struct OrbitRegistry {
    orbits: Keyed<Orbit>,
    open: [u64; 4],
    transport: [u64; 4],
    pub(crate) released: Vec<Vec<u8>>,
    reservoir: Keyed<(State, u64)>,
    /// Whether an orbit returned since the field last beat.
    returned: bool,
}

/// The lane an orbit's key names: its first coordinate, written by
/// [`mode_key`] and the hue map's key alike.
fn lane_of_key(k: &[u8]) -> Option<usize> {
    match k {
        [lane, ..] => Some(usize::from(*lane)),
        [] => None,
    }
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

    /// Per lane: the strain the reservoir holds, dissipated and not yet
    /// re-formed.
    pub fn reserved(&self) -> [u64; 4] {
        let mut out = [0u64; 4];
        for (k, (_, open)) in self.reservoir.scoped() {
            if let Some(slot) = lane_of_key(&k).and_then(|l| out.get_mut(l)) {
                let _ = crate::physics::metabolism::step_or_refuse(slot, *open);
            }
        }
        out
    }

    /// Whether an orbit returned since the last beat, the flag taken.
    fn take_returned(&mut self) -> bool {
        std::mem::take(&mut self.returned)
    }

    /// DISSIPATION, THE SINK (MU-CLIENT §4.5): every orbit that never returns
    /// — a boost, a null flow — moves into the reservoir whole, at its own
    /// coordinates: its lane's open pressure falls by exactly the strain it
    /// held, and the lines that threaded it relax. What rings stays.
    fn dissipate(&mut self) {
        let carriers: Vec<Vec<u8>> = self.orbits.scoped().filter(|(_, o)| !o.state.rings()).map(|(k, _)| k).collect();
        for k in carriers {
            let Some(o) = self.orbits.remove(&k) else { continue };
            // The lane's open pressure holds this orbit's, so the step back
            // is exact; a key at one place is in the field or the reservoir,
            // never both (`reform` draws the reservoir first).
            if let Some(slot) = lane_of_key(&k).and_then(|l| self.open.get_mut(l)) {
                if let Some(left) = slot.checked_sub(o.open) {
                    *slot = left;
                }
            }
            self.released.extend(o.lines);
            self.reservoir.insert(&k, (o.state, o.open));
        }
    }

    /// REFORMATION, THE SOURCE: a motion landing on coordinates the reservoir
    /// holds draws the record back into the field first — the orbit restored
    /// with the strain it held, the lane's open pressure raised by exactly
    /// that — so the motion is stepped against what re-formed.
    fn reform(&mut self, key: &[u8], lane: usize) {
        if self.orbits.contains(key) {
            return;
        }
        let Some((state, open)) = self.reservoir.remove(key) else { return };
        if let Some(slot) = self.open.get_mut(lane) {
            let _ = crate::physics::metabolism::step_or_refuse(slot, open);
        }
        self.orbits.insert(key, Orbit { state, open, lines: Vec::new() });
    }

    /// ADMIT ONE MODE of a field line carrying `mass`: a turn onto 𝒽's orbit
    /// of its plane, a boost onto 𝒮's, the mode's point on the plane's flow
    /// composed into where the orbit stands (78.75). `None` for a null mode
    /// (the tubes hold those), and where the mode's point does not read on the
    /// orbit's plane or an amount passes its carrier.
    pub fn admit_mode(&mut self, mode: &MotionMode, mass: u64, line: &Thread) -> Option<(usize, Admitted)> {
        let lane = match mode.kind {
            BivectorKind::Elliptic => LANE_SHELL,
            BivectorKind::Hyperbolic => LANE_SPATIAL,
            BivectorKind::Parabolic | BivectorKind::Trivial => return None,
        };
        let (key, before, after) = match mode.flow() {
            Some(point) => {
                let key = mode_key(lane, point.plane())?;
                self.reform(&key, lane);
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
                self.reform(&key, lane);
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
        Some((lane, self.step(&key, lane, &before, after, mass, line)?))
    }

    /// ADMIT ONE HUE MAP carrying `mass` onto 𝒢's orbit of its support.
    /// `None` where it moves no hue (the identity carries no strain), where
    /// it is not a map (one hue sent two places), or an amount passes its
    /// carrier.
    pub fn admit_hues(&mut self, gauge: &[(u8, u8)], mass: u64, line: &Thread) -> Option<Admitted> {
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
        self.reform(&key, LANE_GAUGE);
        let held = match self.orbits.get(&key).map(|o| o.state.clone()) {
            Some(State::Hues(map)) => map,
            Some(State::Flow(_) | State::Ray(_)) => return None,
            None => moved.iter().map(|(h, _)| (*h, *h)).collect(),
        };
        let after: Vec<(u8, u8)> = held.iter().map(|(h, at)| (*h, sigma(*at))).collect();
        self.step(&key, LANE_GAUGE, &State::Hues(held), State::Hues(after), mass, line)
    }

    /// ONE ADMISSION ON ONE ORBIT: open, close, or return, exactly; the
    /// admitted line threads or leaves it, and a return releases every line
    /// that threads it.
    fn step(&mut self, key: &[u8], lane: usize, before: &State, after: State, mass: u64, line: &Thread) -> Option<Admitted> {
        let transport = self.transport[lane].checked_add(mass)?;
        let held_open = self.orbits.get(key).map_or(0, |o| o.open);
        let mut lines = match self.orbits.get(key) {
            Some(o) => o.lines.clone(),
            None => Vec::new(),
        };
        thread_on(&mut lines, line);
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
                // HARMONIC: the orbit returned; its tension vented, and every
                // line threading it relaxed.
                self.released.append(&mut lines);
                // A BEAT of the field: what never returns dissipates on it.
                self.returned = true;
            }
            Some(open) => {
                self.orbits.insert(key, Orbit { state: after, open, lines });
            }
        }
        self.open[lane] = lane_open;
        self.transport[lane] = transport;
        Some(moved)
    }
}
