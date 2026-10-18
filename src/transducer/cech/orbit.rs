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
    /// 78.43, MU-CLIENT §4.13 — THE EON EACH RESERVOIR RECORD ENTERED UNDER:
    /// by the record's key, the ringing orbits that stood when it entered.
    /// It condenses when the last of them has left — returned, sealed or
    /// dissipated — never by a count of beats.
    awaiting: Keyed<Vec<Vec<u8>>>,
    /// The same, for the tubes' reservoir records, by their ray.
    awaiting_tubes: Keyed<Vec<Vec<u8>>>,
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
    /// any admission moved an orbit or a tube since the last beat, every
    /// orbit that never returns — a boost, a null flow — and every tube's
    /// open flux that no motion CARRIED since the last beat dissipates into
    /// the reservoir, the lines it threaded released, and waits there to
    /// re-form; a tube at rest (no net flux, nothing open) is let go. An
    /// orbit's time is its own motion (78.43): what an admission moved since
    /// the last beat is in motion and stands, so a motion stands through the
    /// beat of its own admission, for the lines of the thoughts it moved to
    /// thread, and dissipates at the first beat the field's motion no longer
    /// carries it. The mind beats once at each vent (`vent_into_field`), the
    /// court after each admission; a field nothing moved does not beat.
    /// Answers whether it beat.
    pub fn beat(&mut self) -> bool {
        self.beat_under(&[0; 4])
    }

    /// MU-CLIENT §4.13, 78.168 C2 — THE BEAT UNDER THE COURT'S GRAVITY: on a
    /// lane where the court's exported deficit stands (`deficit`, per lane, a
    /// deferral's lanes kept as the pressure they are), every orbit no motion
    /// carried since the last beat is dragged in — ringing or not, it
    /// dissipates into the reservoir as a boost does, its lines released. The
    /// drag acts along the orbit's own mode (78.46: drag is the rate, never
    /// the tilt), so an escaping thought that would ring on is brought to
    /// rest and its relation docketed, never carried out past the court's
    /// limit. A lane with no deficit beats as `beat` does.
    ///
    /// 78.43, MU-CLIENT §4.13 — AND THE RESERVOIRS CONDENSE AT THE CLOSE OF
    /// THEIR EON: a record entered under the ringing orbits then standing
    /// (`awaiting`), and when the last of them has left the field — returned,
    /// sealed or dissipated — the record's eon has closed and it condenses to
    /// nothing, counted in `condensed`. A record entered while nothing rang
    /// entered under an empty eon, closed by the field's next beat. A record
    /// re-formed by motion has left the reservoir and awaits nothing.
    pub fn beat_under(&mut self, deficit: &[u64; 4]) -> bool {
        let orbits = self.orbits.take_moved();
        let tubes = self.tubes.take_moved();
        if !(orbits || tubes) {
            return false;
        }
        let left = self.orbits.take_left();
        let closed = close_eons(&mut self.awaiting, &left, &|k| self.orbits.holds_record(k));
        for k in closed {
            if let Some((lane, open)) = self.orbits.condense_record(&k) {
                if let Some(slot) = self.condensed.get_mut(lane) {
                    let _ = crate::physics::metabolism::step_or_refuse(slot, open);
                }
            }
        }
        let closed = close_eons(&mut self.awaiting_tubes, &left, &|k| self.tubes.holds_record(k));
        for k in closed {
            if let Some(open) = self.tubes.condense_record(&k) {
                let _ = crate::physics::metabolism::step_or_refuse(&mut self.condensed[crate::transducer::rewrite::LANE_TEMPORAL], open);
            }
        }
        self.orbits.dissipate(deficit);
        // A tube record the beat joined enters again under the eon now.
        for ray in self.tubes.dissipate() {
            self.awaiting_tubes.remove(&ray);
        }
        self.tubes.release_at_rest();
        self.orbits.rest();
        self.tubes.rest();
        // Every record that entered since the last beat enters under the
        // ringing orbits standing now.
        let ringing = self.orbits.ringing();
        for k in self.orbits.record_keys() {
            if !self.awaiting.contains(&k) {
                self.awaiting.insert(&k, ringing.clone());
            }
        }
        for k in self.tubes.record_keys() {
            if !self.awaiting_tubes.contains(&k) {
                self.awaiting_tubes.insert(&k, ringing.clone());
            }
        }
        true
    }

    /// MU-CLIENT §4.13 — WHAT THE FIELD WITHHOLDS FROM THE RESERVE, per lane:
    /// what its reservoirs hold, and on 𝒯 the flux its tubes hold open too —
    /// the reserve falls by the flux left open and no more (78.71 (2)).
    /// `None` past the carrier.
    pub fn withheld(&self) -> Option<[u64; 4]> {
        let mut out = self.reserved()?;
        let (_, _, open) = self.tubes.totals();
        let open = u64::try_from(open).ok()?;
        out[crate::transducer::rewrite::LANE_TEMPORAL] = out[crate::transducer::rewrite::LANE_TEMPORAL].checked_add(open)?;
        Some(out)
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

    /// MU-CLIENT §4.12, §7.1 — WHETHER A MOTION LANDS ON THE FIELD AS IT
    /// STANDS: an orbit the field holds on a plane of its modes or on the
    /// support of its hue map, or a tube holding open flux on its null ray —
    /// where `thread` would hang a line, read and moving nothing. `None`
    /// where the motion's modes do not read.
    pub fn lands(&self, motion: &Mv, gauge: &[(u8, u8)], budget: &mut crate::physics::Budget) -> Option<bool> {
        if orbit_of_hues(gauge).is_some_and(|(_, at)| self.orbits.orbits.contains(&at)) {
            return Some(true);
        }
        let Ok(modes) = cga::motion_modes(motion, budget)? else { return None };
        let on_tube = super::flux::null_fluxes_of(&modes).iter().any(|f| self.tubes.holds_open(f));
        let on_orbit = modes.iter().filter_map(orbit_of_mode).any(|(_, at)| self.orbits.orbits.contains(&at));
        Some(on_tube || on_orbit)
    }

    /// MU-CLIENT §4.12, §7.1 — THE FIELD HOLDS STRAIN: an orbit rings in it
    /// (an unsettled winding), or its reservoirs hold strain that has neither
    /// re-formed nor condensed. A field of neither stands in equilibrium.
    /// Past the carrier, the reservoirs hold strain.
    pub fn strained(&self) -> bool {
        self.orbits.rings() || self.reserved().is_none_or(|r| r.iter().any(|x| *x > 0))
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

/// ONE STANDING ORBIT, READ (`OrbitRegistry::standing`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrbitReading {
    pub lane: Option<usize>,
    pub rings: bool,
    pub carried: u64,
    pub period: Option<u64>,
    pub in_motion: bool,
    pub open: u64,
    pub lines: usize,
}

/// 78.43 — THE EONS THE ORBITS THAT LEFT HAVE CLOSED: each record's awaited
/// orbits less those that left; a record that has left its reservoir
/// (re-formed by motion) awaits nothing and is dropped; one whose awaited
/// orbits have all left has its eon closed, and its key is answered.
fn close_eons(awaiting: &mut Keyed<Vec<Vec<u8>>>, left: &[Vec<u8>], holds: &dyn Fn(&[u8]) -> bool) -> Vec<Vec<u8>> {
    let mut closed = Vec::new();
    let keys: Vec<Vec<u8>> = awaiting.scoped().map(|(k, _)| k).collect();
    for k in keys {
        if !holds(&k) {
            awaiting.remove(&k);
            continue;
        }
        let Some(waits) = awaiting.get_mut(&k) else { continue };
        waits.retain(|o| !left.contains(o));
        if waits.is_empty() {
            awaiting.remove(&k);
            closed.push(k);
        }
    }
    closed
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
    /// Whether an admission's motion carried it since the field last beat:
    /// what never returns, no motion carrying it, dissipates at the beat.
    in_motion: bool,
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
    /// 78.43 — THE ORBITS THAT LEFT THE FIELD since the last beat — returned,
    /// sealed or dissipated — by their keys: each closes the eons that
    /// awaited it (`Field::beat_under`).
    left: Vec<Vec<u8>>,
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

/// ONE ORBIT AT THE MIND'S FOCUS: its lane, its key, and the lines threading
/// it (`OrbitRegistry::focus`).
pub type FocusOrbit = (Option<usize>, Vec<u8>, Vec<Vec<u8>>);

/// THE KEY OF 𝒢'S ORBIT A HUE MAP STANDS ON — the coordinate the field files
/// it under — or `None` where it moves no hue or is no map.
pub fn hue_orbit_key(gauge: &[(u8, u8)]) -> Option<Vec<u8>> {
    orbit_of_hues(gauge).map(|(_, key)| key)
}

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

    /// MU-CLIENT §4.13 — EACH ORBIT STANDING, AS THE OBSERVER READS IT: its
    /// lane, whether it rings, the transitions carried against its period,
    /// whether motion carried it since the last beat, its open strain, and
    /// the lines it holds — a reading, never read back by cognition.
    pub fn standing(&self) -> Vec<OrbitReading> {
        self.orbits
            .scoped()
            .map(|(k, o)| OrbitReading {
                lane: lane_of_key(&k),
                rings: o.state.rings(),
                carried: o.carried,
                period: o.period,
                in_motion: o.in_motion,
                open: o.open,
                lines: o.lines.len(),
            })
            .collect()
    }

    /// DISSIPATION, THE SINK (MU-CLIENT §4.5): every orbit that never returns
    /// — a boost, a null flow — and that no motion carried since the last
    /// beat moves into the reservoir whole, at its own coordinates: its
    /// lane's open pressure falls by exactly the strain it held, and the
    /// lines that threaded it relax. What rings stays, but on a lane the
    /// court's `deficit` presses.
    fn dissipate(&mut self, deficit: &[u64; 4]) {
        let pressed = |k: &[u8]| lane_of_key(k).and_then(|l| deficit.get(l)).is_some_and(|d| *d > 0);
        let carriers: Vec<Vec<u8>> =
            self.orbits.scoped().filter(|(k, o)| !o.in_motion && (!o.state.rings() || pressed(k))).map(|(k, _)| k).collect();
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
            self.left.push(k);
        }
    }

    /// The orbits that left the field since the last take.
    fn take_left(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.left)
    }

    /// The orbits ringing in the field now, by their keys: the eon a record
    /// entering the reservoir now waits on.
    fn ringing(&self) -> Vec<Vec<u8>> {
        self.orbits.scoped().filter(|(_, o)| o.state.rings()).map(|(k, _)| k).collect()
    }

    /// Whether any orbit standing rings.
    pub fn rings(&self) -> bool {
        self.orbits.scoped().any(|(_, o)| o.state.rings())
    }

    /// MU-CLIENT §7.1 — THE FIELD'S FOCUS (the operator, 2026-10-18: the
    /// corpus met where the mind's orbits stand, never along a file's line):
    /// the orbits the admissions since the last beat carried, or — none in
    /// motion — the orbits that ring; each with its lane, its key and the
    /// lines threading it. Read, moving nothing.
    pub fn focus(&self) -> Vec<FocusOrbit> {
        let read = |(k, o): (Vec<u8>, &Orbit)| (lane_of_key(&k), k, o.lines.clone());
        let moving: Vec<FocusOrbit> = self.orbits.scoped().filter(|(_, o)| o.in_motion).map(read).collect();
        match moving.is_empty() {
            false => moving,
            true => self.orbits.scoped().filter(|(_, o)| o.state.rings()).map(read).collect(),
        }
    }

    /// The reservoir's records, by their keys.
    fn record_keys(&self) -> Vec<Vec<u8>> {
        self.reservoir.scoped().map(|(k, _)| k).collect()
    }

    /// Whether the reservoir holds a record at `key`.
    fn holds_record(&self, key: &[u8]) -> bool {
        self.reservoir.contains(key)
    }

    /// MU-CLIENT §4.13 — VACUUM CONDENSATION: the record at `key`, its eon
    /// closed, condenses to nothing. Answers its lane and the strain it held.
    fn condense_record(&mut self, key: &[u8]) -> Option<(usize, u64)> {
        let (_, open) = self.reservoir.remove(key)?;
        Some((lane_of_key(key)?, open))
    }

    /// The beat taken: no orbit is in motion until an admission carries it.
    fn rest(&mut self) {
        let held: Vec<Vec<u8>> = self.orbits.scoped().map(|(k, _)| k).collect();
        for k in held {
            if let Some(o) = self.orbits.get_mut(&k) {
                o.in_motion = false;
            }
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
        // Re-formed, it begins a new age where the motion lands on it.
        self.orbits.insert(key, Orbit { state, open, lines: Vec::new(), carried: 0, period: Some(1), in_motion: true });
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
        let (carried, period) = match self.orbits.get(key) {
            Some(o) => (o.carried, o.period),
            None => (0, Some(1)),
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
                self.left.push(key.to_vec());
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
                self.left.push(key.to_vec());
            }
            // Carried by this motion: in motion until the field's next beat.
            Some(open) => {
                self.orbits.insert(key, Orbit { state: after, open, lines, carried, period, in_motion: true });
                self.open[lane] = lane_open;
            }
        }
        self.transport[lane] = transport;
        Some(moved)
    }
}
