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

/// MU-CLIENT §4.13, §7.1 Phase 3 — WHY THE FIELD LET A LINE GO: the orbit
/// it threaded RETURNED (harmonic, the tension vented; on a tube, flux along
/// its ray closed all that stood open on it), was DISSIPATED on a
/// beat (it never returns), SEALED at its period without returning (78.43);
/// or the tube it threaded CANCELLED its net flux (luminal) or came to REST.
/// The line's owner reads the cause: a grounded thought winds on past its
/// return and lapses by the others.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Released {
    Returned,
    Dissipated,
    Sealed,
    Cancelled,
    AtRest,
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
    /// The strain the reservoirs condensed since the field began, per lane.
    pub condensed: [u64; 4],
}

impl Field {
    pub fn new() -> Field {
        Field::default()
    }

    /// MU-CLIENT §4.5 — THE LINES THE FIELD RELEASED since the last take:
    /// those threading an orbit that RETURNED (harmonic — its tension vented)
    /// and those threading a tube whose net flux CANCELLED (luminal — it
    /// carries no field). Each is a line of the complex whose tension relaxed.
    /// MU-CLIENT §4.5, §7.1 Phase 3 — THE FIELD BEATS WHEREVER IT MOVED: where
    /// any admission moved an orbit or a tube since the last beat, what stood
    /// through the last beat and never returned dissipates into the reservoir
    /// — every boost and null orbit whole, every tube's open flux — the lines
    /// it threaded released, and waits there to re-form; a tube at rest (no
    /// net flux, nothing open) is let go. What the field holds after the beat
    /// has stood through it. So what never returns is dissipated at the beat
    /// AFTER its admission, never in it: a motion stands one beat, for the
    /// lines of the thoughts it moved to thread. The mind beats once at each
    /// vent (`vent_into_field`), the court after each admission; a field
    /// nothing moved does not beat.
    pub fn beat(&mut self) {
        self.beat_under(&[0; 4]);
    }

    /// MU-CLIENT §4.13, 78.168 C2 — THE BEAT UNDER THE COURT'S GRAVITY: on a
    /// lane where the court's exported deficit stands (`deficit`, per lane, a
    /// deferral's lanes kept as the pressure they are), every orbit that
    /// stood through the last beat without returning is dragged in — ringing
    /// or not, it dissipates into the reservoir as a boost does, its lines
    /// released. The drag acts along the orbit's own mode (78.46: drag is the
    /// rate, never the tilt), so an escaping thought that would ring on is
    /// brought to rest and its relation docketed, never carried out past the
    /// court's limit. A lane with no deficit beats as `beat` does.
    ///
    /// MU-CLIENT §4.13 — AND THE RESERVOIRS CONDENSE: before what this beat
    /// dissipates enters them, every record that stood a whole eon untouched
    /// condenses to nothing (`OrbitRegistry::condense`), the strain it held
    /// counted in `condensed` for the field's owner.
    pub fn beat_under(&mut self, deficit: &[u64; 4]) {
        let orbits = self.orbits.take_moved();
        let tubes = self.tubes.take_moved();
        if orbits || tubes {
            if let Some(eon) = self.orbits.eon() {
                let on_orbits = self.orbits.condense(eon);
                let on_tubes = self.tubes.condense(eon);
                for (slot, add) in self.condensed.iter_mut().zip(on_orbits) {
                    let _ = crate::physics::metabolism::step_or_refuse(slot, add);
                }
                let _ = crate::physics::metabolism::step_or_refuse(&mut self.condensed[crate::transducer::rewrite::LANE_TEMPORAL], on_tubes);
            }
            self.orbits.dissipate(deficit);
            self.tubes.dissipate();
            self.tubes.release_at_rest();
            self.orbits.age();
            self.tubes.age();
        }
    }

    /// MU-CLIENT §4.13 — WHAT THE RESERVOIRS HOLD, per lane: the strain
    /// dissipated and neither re-formed nor condensed — 𝒯's off the tubes,
    /// the other lanes' off the orbits. `None` past the carrier.
    pub fn reserved(&self) -> Option<[u64; 4]> {
        let mut out = self.orbits.reserved();
        out[crate::transducer::rewrite::LANE_TEMPORAL] = u64::try_from(self.tubes.reserved()).ok()?;
        Some(out)
    }

    /// FROZEN-IN FLUX: the lines the field's admissions carried since the
    /// last take, each with the increment it moved by, in admission order.
    pub fn take_carried(&mut self) -> Vec<(Vec<u8>, Mv)> {
        std::mem::take(&mut self.orbits.carried)
    }

    /// MU-CLIENT §4.4(4) — THE ORBITS THAT RETURNED since the last take: the
    /// mind's K moves by its returns, and its acts are spoken there.
    pub fn take_returns(&mut self) -> u64 {
        std::mem::take(&mut self.orbits.returns)
    }

    pub fn take_released(&mut self) -> Vec<(Vec<u8>, Released)> {
        let mut out = std::mem::take(&mut self.orbits.released);
        out.append(&mut self.tubes.released);
        out
    }

    /// MU-CLIENT §4.13, §7.1 Phase 3 — A LINE THREADED ON WHAT A MOTION
    /// STANDS ON: `key` threads every orbit and tube that the modes of `motion`
    /// and its hue map `gauge` land on and that the field holds, moving none
    /// of them (the motion was admitted where it was perceived). Answers
    /// whether any held it: a motion whose orbits all returned or dissipated
    /// threads nothing. `None` where the motion's modes do not read.
    pub fn thread(&mut self, motion: &Mv, gauge: &[(u8, u8)], key: &[u8], budget: &mut crate::physics::Budget) -> Option<bool> {
        let mut held = self.orbits.thread_hues(gauge, key);
        let Ok(modes) = cga::motion_modes(motion, budget)? else { return None };
        for flux in super::flux::null_fluxes_of(&modes) {
            held |= self.tubes.thread(&flux, key);
        }
        for m in &modes {
            held |= self.orbits.thread_mode(m, key);
        }
        Some(held)
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
    /// 78.43(3) — THE DISTANCE ALONG IT: the transitions it has been carried
    /// through since it last returned.
    carried: u64,
    /// 78.43 — ITS PERIOD: the least common multiple of the orders of the
    /// motions that moved it since it last returned, each read off the motion
    /// (a mode's order, a hue map's cycles). `None` once a motion of no order
    /// moved it — a rosette, whose winding is content and never seals.
    period: Option<u64>,
    /// Whether it has stood through a beat of the field: what never returns
    /// is dissipated at the beat after the one it stood through.
    aged: bool,
}

/// The least common multiple of two orders; `None` past the carrier.
fn lcm(a: u64, b: u64) -> Option<u64> {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    (a / x).checked_mul(b)
}

/// THE ORDER OF A HUE MAP: the least common multiple of its cycles' lengths
/// (`class::hue_cycles`, the one producer); `None` past the carrier.
fn hue_order(map: &[(u8, u8)]) -> Option<u64> {
    crate::transducer::sentence::class::hue_cycles(map).iter().try_fold(1u64, |l, c| lcm(l, u64::try_from(c.len()).ok()?))
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
    pub(crate) released: Vec<(Vec<u8>, Released)>,
    /// FROZEN-IN FLUX (MU-CLIENT §4.5): each line an admission CARRIED —
    /// every other line wound on the orbit it moved — with the increment it
    /// moved by, until the field's owner takes them.
    pub(crate) carried: Vec<(Vec<u8>, Mv)>,
    reservoir: Keyed<(State, u64)>,
    /// The beats each reservoir record has stood untouched, by its key: at
    /// one eon it condenses (`condense`).
    stood: Keyed<u64>,
    /// Whether an admission moved an orbit since the field last beat.
    moved: bool,
    /// MU-CLIENT §4.4(4) — the orbits that returned since the field's owner
    /// last took them: where a mind's K moves, and its acts are spoken.
    returns: u64,
    /// 78.43, §7.1 Phase 3 — the orbits that sealed at their period without
    /// returning, since the field began.
    sealed: u64,
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

/// THE ORBIT A MODE STANDS ON — its lane and its key, the one derivation the
/// admission and the threading share: a turn on 𝒽, a boost on 𝒮, keyed by
/// its plane (one flow) or, for an isoclinic set, by its ray's planes. `None`
/// for a null or trivial mode (the tubes hold the null ones).
fn orbit_of_mode(mode: &MotionMode) -> Option<(usize, Vec<u8>)> {
    let lane = match mode.kind {
        BivectorKind::Elliptic => LANE_SHELL,
        BivectorKind::Hyperbolic => LANE_SPATIAL,
        BivectorKind::Parabolic | BivectorKind::Trivial => return None,
    };
    let key = match mode.flow() {
        Some(point) => mode_key(lane, point.plane())?,
        None => mode_key(lane, &mode.ray.grade(2))?,
    };
    Some((lane, key))
}

/// The hues a map moves, each with where it goes, and the key of 𝒢's orbit
/// of its support.
type HueOrbit = (Vec<(u8, u8)>, Vec<u8>);

/// THE HUES A MAP MOVES, sorted, and the key of 𝒢's orbit of its support —
/// the one derivation the admission and the threading share. `None` where it
/// moves no hue or is no map (one hue sent two places).
fn orbit_of_hues(gauge: &[(u8, u8)]) -> Option<HueOrbit> {
    let mut moved: Vec<(u8, u8)> = gauge.iter().copied().filter(|(h, to)| h != to).collect();
    moved.sort_unstable();
    moved.dedup();
    if moved.is_empty() || moved.windows(2).any(|p| p[0].0 == p[1].0) {
        return None;
    }
    let mut key = vec![u8::try_from(LANE_GAUGE).ok()?];
    key.extend(moved.iter().map(|(h, _)| *h));
    Some((moved, key))
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

    /// The returns its owner has not yet taken (the mind's observer reads
    /// them; `Field::take_returns` takes them).
    pub fn pending_returns(&self) -> u64 {
        self.returns
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

    /// Whether an admission moved an orbit since the last beat, the flag
    /// taken.
    fn take_moved(&mut self) -> bool {
        std::mem::take(&mut self.moved)
    }

    /// The orbits that sealed at their period without returning.
    pub fn sealed(&self) -> u64 {
        self.sealed
    }

    /// DISSIPATION, THE SINK (MU-CLIENT §4.5): every orbit that never returns
    /// — a boost, a null flow — and has stood through a beat moves into the
    /// reservoir whole, at its own coordinates: its lane's open pressure falls
    /// by exactly the strain it held, and the lines that threaded it relax.
    /// What rings stays, but on a lane the court's `deficit` presses.
    fn dissipate(&mut self, deficit: &[u64; 4]) {
        let pressed = |k: &[u8]| lane_of_key(k).and_then(|l| deficit.get(l)).is_some_and(|d| *d > 0);
        let carriers: Vec<Vec<u8>> =
            self.orbits.scoped().filter(|(k, o)| o.aged && (!o.state.rings() || pressed(k))).map(|(k, _)| k).collect();
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
            self.released.extend(o.lines.into_iter().map(|l| (l, Released::Dissipated)));
            self.reservoir.insert(&k, (o.state, o.open));
            self.stood.insert(&k, 0);
        }
    }

    /// 78.43 — THE FIELD'S EON: the least common multiple of the periods of
    /// the orbits ringing in it, one beat where none rings; `None` past the
    /// carrier, or where a rosette rings (no period: no eon closes).
    fn eon(&self) -> Option<u64> {
        self.orbits.scoped().filter(|(_, o)| o.state.rings()).try_fold(1u64, |e, (_, o)| lcm(e, o.period?))
    }

    /// MU-CLIENT §4.13 — VACUUM CONDENSATION: every reservoir record stands
    /// one beat more, and one that stood a whole eon untouched — no motion
    /// landed on its coordinates to re-form it — condenses to nothing, so
    /// the reservoir is bounded by what re-forms. Answers the strain
    /// condensed, per lane.
    fn condense(&mut self, eon: u64) -> [u64; 4] {
        let mut out = [0u64; 4];
        let keys: Vec<Vec<u8>> = self.stood.scoped().map(|(k, _)| k).collect();
        for k in keys {
            let Some(beats) = self.stood.get(&k).and_then(|b| b.checked_add(1)) else { continue };
            if beats < eon {
                self.stood.insert(&k, beats);
                continue;
            }
            self.stood.remove(&k);
            if let Some((_, open)) = self.reservoir.remove(&k) {
                if let Some(slot) = lane_of_key(&k).and_then(|l| out.get_mut(l)) {
                    let _ = crate::physics::metabolism::step_or_refuse(slot, open);
                }
            }
        }
        out
    }

    /// Every orbit the field holds after a beat has stood through it; and
    /// every reservoir record that entered since the last beat (a seal, in
    /// an admission) begins its count here, as one dissipated at this beat
    /// does — none is counted for a beat it did not stand through.
    fn age(&mut self) {
        let held: Vec<Vec<u8>> = self.orbits.scoped().map(|(k, _)| k).collect();
        for k in held {
            if let Some(o) = self.orbits.get_mut(&k) {
                o.aged = true;
            }
        }
        let entered: Vec<Vec<u8>> = self.reservoir.scoped().map(|(k, _)| k).filter(|k| !self.stood.contains(k)).collect();
        for k in entered {
            self.stood.insert(&k, 0);
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
        self.stood.remove(key);
        if let Some(slot) = self.open.get_mut(lane) {
            let _ = crate::physics::metabolism::step_or_refuse(slot, open);
        }
        // Re-formed, it begins a new age where the motion lands on it.
        self.orbits.insert(key, Orbit { state, open, lines: Vec::new(), carried: 0, period: Some(1), aged: false });
    }

    /// ADMIT ONE MODE of a field line carrying `mass`: a turn onto 𝒽's orbit
    /// of its plane, a boost onto 𝒮's, the mode's point on the plane's flow
    /// composed into where the orbit stands (78.75). `None` for a null mode
    /// (the tubes hold those), and where the mode's point does not read on the
    /// orbit's plane or an amount passes its carrier.
    pub fn admit_mode(&mut self, mode: &MotionMode, mass: u64, line: &Thread) -> Option<(usize, Admitted)> {
        let (lane, key) = orbit_of_mode(mode)?;
        let (key, before, after, increment) = match mode.flow() {
            Some(point) => {
                self.reform(&key, lane);
                let before = match self.orbits.get(&key).map(|o| o.state.clone()) {
                    Some(State::Flow(held)) => held,
                    Some(State::Ray(_) | State::Hues(_)) => return None,
                    None => cga::FlowPoint::origin(point.plane())?,
                };
                let after = before.then(&point)?;
                (key, State::Flow(before), State::Flow(after), point.versor())
            }
            // An isoclinic set: its planes flow together, held as one ray.
            None => {
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
                (key, State::Ray(before), State::Ray(after), mode.ray.clone())
            }
        };
        let lines = match self.orbits.get(&key) {
            Some(o) => o.lines.clone(),
            None => Vec::new(),
        };
        let admitted = self.step(&key, lane, &before, after, mass, line, mode.order.map(u64::from))?;
        // Every OTHER line wound on the orbit is carried by what moved it.
        let own = match line {
            Thread::Wound(k) | Thread::Unwound(k) => Some(k),
            Thread::Free => None,
        };
        self.carried.extend(lines.into_iter().filter(|l| Some(l) != own).map(|l| (l, increment.clone())));
        Some((lane, admitted))
    }

    /// ADMIT ONE HUE MAP carrying `mass` onto 𝒢's orbit of its support.
    /// `None` where it moves no hue (the identity carries no strain), where
    /// it is not a map (one hue sent two places), or an amount passes its
    /// carrier.
    pub fn admit_hues(&mut self, gauge: &[(u8, u8)], mass: u64, line: &Thread) -> Option<Admitted> {
        let (moved, key) = orbit_of_hues(gauge)?;
        let sigma = |h: u8| match moved.binary_search_by_key(&h, |(from, _)| *from) {
            Ok(at) => moved[at].1,
            Err(_) => h,
        };
        self.reform(&key, LANE_GAUGE);
        let held = match self.orbits.get(&key).map(|o| o.state.clone()) {
            Some(State::Hues(map)) => map,
            Some(State::Flow(_) | State::Ray(_)) => return None,
            None => moved.iter().map(|(h, _)| (*h, *h)).collect(),
        };
        let after: Vec<(u8, u8)> = held.iter().map(|(h, at)| (*h, sigma(*at))).collect();
        self.step(&key, LANE_GAUGE, &State::Hues(held), State::Hues(after), mass, line, hue_order(&moved))
    }

    /// `key` THREADED ON THE ORBIT A MODE STANDS ON, where the field holds it;
    /// nothing moves. Answers whether it was held.
    fn thread_mode(&mut self, mode: &MotionMode, key: &[u8]) -> bool {
        let Some((_, at)) = orbit_of_mode(mode) else { return false };
        match self.orbits.get_mut(&at) {
            Some(o) => {
                thread_on(&mut o.lines, &Thread::Wound(key.to_vec()));
                true
            }
            None => false,
        }
    }

    /// `key` THREADED ON 𝒢'S ORBIT OF THE SUPPORT A HUE MAP MOVES, where the
    /// field holds it; nothing moves. Answers whether it was held.
    fn thread_hues(&mut self, gauge: &[(u8, u8)], key: &[u8]) -> bool {
        let Some((_, at)) = orbit_of_hues(gauge) else { return false };
        match self.orbits.get_mut(&at) {
            Some(o) => {
                thread_on(&mut o.lines, &Thread::Wound(key.to_vec()));
                true
            }
            None => false,
        }
    }

    /// ONE ADMISSION ON ONE ORBIT: open, close, or return, exactly; the
    /// admitted line threads or leaves it, and a return releases every line
    /// that threads it. `order` is the order of the motion admitted (`None`
    /// for a motion of no order); the orbit is carried one transition
    /// further, and an orbit carried its whole period without returning
    /// SEALS (78.43): its age complete, its lines released, held whole in
    /// the reservoir at its coordinates to re-form where motion lands.
    #[expect(clippy::too_many_arguments, reason = "78.43 — the motion's order is the admission's own coordinate, read where the motion is split, never stored beside the orbit")]
    fn step(&mut self, key: &[u8], lane: usize, before: &State, after: State, mass: u64, line: &Thread, order: Option<u64>) -> Option<Admitted> {
        let transport = self.transport[lane].checked_add(mass)?;
        let held_open = self.orbits.get(key).map_or(0, |o| o.open);
        let mut lines = match self.orbits.get(key) {
            Some(o) => o.lines.clone(),
            None => Vec::new(),
        };
        let (carried, period, aged) = match self.orbits.get(key) {
            Some(o) => (o.carried, o.period, o.aged),
            None => (0, Some(1), false),
        };
        let carried = carried.checked_add(1)?;
        let period = match (period, order) {
            (Some(p), Some(o)) => Some(lcm(p, o)?),
            _ => None,
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
        // A movement of the field: it beats on it.
        self.moved = true;
        match left {
            None => {
                self.orbits.remove(key);
                // HARMONIC: the orbit returned; its tension vented, and every
                // line threading it relaxed.
                self.released.extend(lines.into_iter().map(|l| (l, Released::Returned)));
                // A count past its carrier is refused and the return still
                // stands.
                let _ = crate::physics::metabolism::step_or_refuse(&mut self.returns, 1);
                self.open[lane] = lane_open;
            }
            // 78.43 — CARRIED ITS WHOLE PERIOD AND NOT HOME: the age is
            // complete and the orbit SEALS — its lines released, its point
            // and its open strain held in the reservoir at its coordinates,
            // the lane's open pressure falling by exactly that strain.
            Some(open) if period.is_some_and(|p| carried >= p) => {
                self.orbits.remove(key);
                self.released.extend(lines.into_iter().map(|l| (l, Released::Sealed)));
                self.reservoir.insert(key, (after, open));
                self.open[lane] = lane_open.checked_sub(open)?;
                let _ = crate::physics::metabolism::step_or_refuse(&mut self.sealed, 1);
            }
            Some(open) => {
                self.orbits.insert(key, Orbit { state: after, open, lines, carried, period, aged });
                self.open[lane] = lane_open;
            }
        }
        self.transport[lane] = transport;
        Some(moved)
    }
}
