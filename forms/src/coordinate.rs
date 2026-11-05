// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE COORDINATE (LAW-COORDINATE v1, proposal 78.173; ruled by both
//! operators 2026-11-05 through MU-RELEASE §3a), the standard every kernel
//! addresses its bodies by. Netstratum's statement is docs/120, "The
//! coordinate: TUniversal inside the universe" (branch `law/coordinate`).
//!
//! THE COORDINATE IS THE STATE. A body stands at
//!
//! ```text
//!     origin / h : S:G:T:H : K
//! ```
//!
//! - the ORIGIN is its enclosure ([`Origin`]): the universe's root (the
//!   genesis hash and the epoch TUniversal already carries, 78.152 TU-5)
//!   and the coordinates of the enclosures it stands in, written out,
//!   outermost first — never a digest of them;
//! - `h` is the bell;
//! - the four LANES are its cumulative change by motion type, each a
//!   signed exact rational, never summed into one number ([`Lane`]): S
//!   spatial (boost and dilation, the reach), G gauge (change of kind), T
//!   temporal (null flux: the beat and the passages), H holonomic (the
//!   turnings between planes);
//! - `K` counts the orbits it closed.
//!
//! THE ORDERED KEY. The canonical order — origin, `h`, the lanes in the
//! order S, G, T, H by exact value, `K` — is the index a kernel addresses
//! its bodies by ([`Bodies`]). Neighbours are a range on one lane in that
//! order ([`Bodies::range`]), found by bisection: never a test of all
//! pairs, never a catalogue walked.
//!
//! THE SEXAGESIMAL READING. A lane's value is read in places of base
//! [`BASE`]: at depth `d` the value lies in exactly one sector
//! `[m·60⁻ᵈ, (m + 1)·60⁻ᵈ)`, `m` the integer the exact comparison fixes
//! ([`Sector`]). Sectors nest: a sector at depth `d + 1` lies within one at
//! depth `d`. [`Sector::side`] and [`Coordinate::within`] decide on the
//! sector's exact bounds; the places ([`Sector::places`]) are display, and
//! nothing is compared, stored or decided by them. The depth is the
//! reading's depth (zoom), never a grain the value is held on: the value
//! stays exact, so this is no floor on a grid. The unit read is the
//! caller's: a lane read in turns sits at depth 0 in its whole turn, and
//! read in degrees (the value times 360) at depth 0 in its whole degree, so
//! xylarium's `Reading::within` at depth `d ≥ 1` on turns is this form's at
//! depth `d − 1` on the value in degrees.
//!
//! ORIGINS ARE NEVER COMPARED. A reading between two bodies of different
//! origins is refused [`CoordinateRefusal::OriginsDiffer`] by name. The
//! key's order puts each origin's bodies together, by the origin written
//! out; it orders writings and measures nothing across them.
//!
//! TUNIVERSAL'S POINT IS THE NON-NEGATIVE CASE. `TPoint` (78.152 TU-5)
//! reads as a coordinate with no enclosures and non-negative lanes: the
//! same fields, the same order and the same writing (claimed in
//! `tests/coordinate.rs`). The map from a body's signed lanes inside a
//! universe to the non-negative issuance reading at the vertex is OPEN and
//! not yet law; nothing here states it. Rewiring `TPoint` onto this form is
//! the next step.
//!
//! MOTION, EVENTS AND MERGERS (78.173 §1, fixtures 7–10; netstratum docs/120
//! §2 and §5). A body ADVANCES in the bell by the closed form
//! `Lᵢ(t) = Lᵢ(h) + rᵢ·(t − h)` under its lanes' [`Rates`], with `K`
//! re-read as the whole turns of H in both senses, `⌊|L_H(t)|⌋`; an advance
//! composes, and a backward one is refused ([`Coordinate::advance`]). AN
//! EVENT'S BELL is the exact first-degree solution `h + (v − L)/r`, which may
//! fall between whole bells, and there is none on an unmoving lane or
//! behind the body's bell ([`Coordinate::bell_of`], [`EventBell`]). A
//! MERGER of two bodies at one origin and one bell is the holding-weighted
//! coordinate, and it keeps each body's offset per lane, whose
//! holding-weighted sum is zero: no residual is discarded ([`merge`],
//! [`Merger`]). FISSION is its inverse: the merged point, the holdings and
//! the offsets give back both bodies ([`fission`]). Re-origin across a
//! throat and gaps are not in this form; they wait on the passage flux on T
//! (78.173 §7).

use crate::Q;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-COORDINATE.
pub const LAW_ID: &str = "FORMS-COORDINATE";

/// The form's version: an amendment, or an operation added to the form,
/// raises it.
/// FORMAT: the shell's version, the form of LAW-COORDINATE v1 (proposal
/// 78.173) with the advance, an event's bell, the merger and fission
/// (fixtures 7–10) held here.
pub const LAW_VERSION: u16 = 2;

/// THE READING'S BASE: the places of the sexagesimal reading.
/// EXTERNAL: LAW-COORDINATE v1 §3 (proposal 78.173, ruled 2026-11-05), the sexagesimal place value of xylarium's coordinate reading and netstratum's READING.
pub const BASE: u32 = 60;

/// THE FOUR LANES, in the canonical order, by the network's meanings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lane {
    /// Spatial: boost and dilation, the reach.
    S,
    /// Gauge: change of kind.
    G,
    /// Temporal: null flux, the beat and the passages.
    T,
    /// Holonomic: the turnings between planes.
    H,
}

impl Lane {
    /// The lanes in the canonical order.
    pub const ORDER: [Lane; 4] = [Lane::S, Lane::G, Lane::T, Lane::H];

    /// The lane's place in the canonical order.
    pub fn place(self) -> usize {
        match self {
            Lane::S => 0,
            Lane::G => 1,
            Lane::T => 2,
            Lane::H => 3,
        }
    }
}

/// A BODY'S PLACE WITHIN ITS ORIGIN: the bell `h`, the four signed exact
/// lanes in the order S, G, T, H, and `K`, the orbits closed. Ordered as
/// written: `h`, each lane by its exact value in lane order, `K`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Local {
    pub h: u64,
    pub lanes: [Q; 4],
    pub k: u64,
}

impl Local {
    /// The value on `lane`.
    pub fn lane(&self, lane: Lane) -> &Q {
        &self.lanes[lane.place()]
    }

    /// The writing `h:S:G:T:H:K`: `h` and `K` in 16 lowercase hex digits,
    /// each lane its exact rational in lowest terms, `n/d` in lowercase hex
    /// with no leading zero, a negative lane's numerator signed `-` (`0/1`
    /// for none) — TPoint's writing (78.152 TU-5) where every lane is
    /// non-negative.
    pub fn written(&self) -> String {
        let mut fields = vec![format!("{:016x}", self.h)];
        fields.extend(self.lanes.iter().map(lane_text));
        fields.push(format!("{:016x}", self.k));
        fields.join(":")
    }
}

/// A BODY'S ENCLOSURE: the universe's root — the genesis hash and the
/// epoch (78.152 TU-5, the one digest the coordinate carries) — and the
/// enclosures it stands in, outermost first, each written out as the
/// coordinate its branch was cut at (TU-6 BRANCH). Ordered as written:
/// genesis, epoch, then the enclosures in turn.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Origin {
    pub genesis: [u8; 32],
    pub epoch: u32,
    pub enclosures: Vec<Local>,
}

impl Origin {
    /// The writing: the genesis in 64 lowercase hex digits, `.`, the epoch
    /// in 8, then `/` and each enclosure's writing, outermost first.
    pub fn written(&self) -> String {
        let mut out: String = self.genesis.iter().map(|b| format!("{b:02x}")).collect();
        out.push_str(&format!(".{:08x}", self.epoch));
        for e in &self.enclosures {
            out.push('/');
            out.push_str(&e.written());
        }
        out
    }
}

/// THE COORDINATE: a body's origin and its place within it — the state.
/// Its order is the canonical key: origin, `h`, the lanes S, G, T, H by
/// exact value, `K`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Coordinate {
    pub origin: Origin,
    pub local: Local,
}

/// WHY A READING BETWEEN COORDINATES IS REFUSED, by name and with what was
/// read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoordinateRefusal {
    /// Two bodies of different origins: never compared. The origin held and
    /// the origin asked, as written.
    OriginsDiffer { held: Box<Origin>, asked: Box<Origin> },
}

/// WHERE A VALUE STANDS AGAINST A SECTOR: below its lower bound, within
/// `[lo, hi)`, or at or past its upper bound — decided exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Below,
    Within,
    Above,
}

impl Coordinate {
    /// The full writing `origin/h:S:G:T:H:K`, the origin's enclosures
    /// between: TPoint's writing (`TPoint::print`) where the origin has no
    /// enclosure and every lane is non-negative.
    pub fn written(&self) -> String {
        format!("{}/{}", self.origin.written(), self.local.written())
    }

    /// The value on `lane`.
    pub fn lane(&self, lane: Lane) -> &Q {
        self.local.lane(lane)
    }

    /// The sexagesimal reading of `lane` at `depth`: the sector holding it.
    pub fn reading(&self, lane: Lane, depth: u32) -> Sector {
        Sector::holding(self.lane(lane), depth)
    }

    /// WHERE THIS BODY STANDS ON `lane` AGAINST THE SECTOR AT `depth` THAT
    /// HOLDS `other`'s value there: decided on the sector's exact bounds,
    /// never on places. Refused `OriginsDiffer` where the two origins
    /// differ.
    pub fn within(&self, other: &Coordinate, lane: Lane, depth: u32) -> Result<Side, CoordinateRefusal> {
        match self.origin == other.origin {
            true => Ok(other.reading(lane, depth).side(self.lane(lane))),
            false => Err(CoordinateRefusal::OriginsDiffer {
                held: Box::new(self.origin.clone()),
                asked: Box::new(other.origin.clone()),
            }),
        }
    }
}

/// ONE SECTOR OF THE SEXAGESIMAL READING: at `depth`, the exact interval
/// `[index·60⁻ᵈᵉᵖᵗʰ, (index + 1)·60⁻ᵈᵉᵖᵗʰ)`. The value read is never
/// replaced by it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sector {
    pub depth: u32,
    pub index: BigInt,
}

/// THE DISPLAY OF A SECTOR: its lower bound as a signed whole part and
/// `depth` places, each in `0..60`, so that `lo = whole + Σ placeᵢ·60⁻ⁱ`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Places {
    pub whole: BigInt,
    pub places: Vec<BigInt>,
}

/// `60ᵈᵉᵖᵗʰ`, exactly.
fn scale(depth: u32) -> BigInt {
    BigInt::from(BASE).pow(depth)
}

impl Sector {
    /// THE SECTOR AT `depth` HOLDING `value`: the one integer `m` with
    /// `m·60⁻ᵈ ≤ value < (m + 1)·60⁻ᵈ`, fixed by exact comparison.
    pub fn holding(value: &Q, depth: u32) -> Sector {
        let index = (value * Q::from_integer(scale(depth))).floor().to_integer();
        Sector { depth, index }
    }

    /// The sector's width, `60⁻ᵈᵉᵖᵗʰ`.
    pub fn width(&self) -> Q {
        Q::new(BigInt::one(), scale(self.depth))
    }

    /// The lower bound, held by the sector.
    pub fn lo(&self) -> Q {
        Q::new(self.index.clone(), scale(self.depth))
    }

    /// The upper bound, not held by the sector.
    pub fn hi(&self) -> Q {
        Q::new(&self.index + BigInt::one(), scale(self.depth))
    }

    /// The sector one depth coarser that holds this one; `None` at depth 0,
    /// the whole unit, which no coarser sector reads.
    pub fn parent(&self) -> Option<Sector> {
        Some(Sector::holding(&self.lo(), self.depth.checked_sub(1)?))
    }

    /// WHERE `value` STANDS against `[lo, hi)`, by exact comparison.
    pub fn side(&self, value: &Q) -> Side {
        match (value.cmp(&self.lo()), value.cmp(&self.hi())) {
            (Ordering::Less, _) => Side::Below,
            (_, Ordering::Less) => Side::Within,
            _ => Side::Above,
        }
    }

    /// THE PLACES, for display: the whole part (the integer at or below the
    /// lower bound) and `depth` places in `0..60`, coarsest first.
    pub fn places(&self) -> Places {
        let s = scale(self.depth);
        let whole = Q::new(self.index.clone(), s.clone()).floor().to_integer();
        let mut rest = &self.index - &whole * &s;
        let mut places = Vec::new();
        for i in (0..self.depth).rev() {
            let unit = scale(i);
            places.push(&rest / &unit);
            rest %= unit;
        }
        Places { whole, places }
    }

    /// THE WRITING, for display only: the whole part in decimal, then `;`
    /// and the places in two decimal digits each, separated by `,` — the
    /// sexagesimal notation, `0;08,34,17` for `1/7` at depth 3. A sector at
    /// depth 0 writes its whole part alone.
    pub fn written(&self) -> String {
        let p = self.places();
        match p.places.is_empty() {
            true => p.whole.to_string(),
            false => {
                let places: Vec<String> = p.places.iter().map(|d| format!("{d:02}")).collect();
                format!("{};{}", p.whole, places.join(","))
            }
        }
    }
}

/// A lane's one writing: `n/d` in lowercase hex, lowest terms, a negative
/// numerator signed `-`.
fn lane_text(q: &Q) -> String {
    format!("{}/{}", q.numer().to_str_radix(16), q.denom().to_str_radix(16))
}

/// THE BODIES A KERNEL HOLDS, in the canonical order: the ordered key is
/// the index, so a neighbour is found by bisection on it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bodies {
    held: Vec<Coordinate>,
}

/// THE BODIES OF ONE RANGE ON ONE LANE, and the probes the two bisections
/// that found it took.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Neighbours<'a> {
    pub bodies: &'a [Coordinate],
    pub probes: u32,
}

/// The first place in `held` whose body does not stand before the bound,
/// by bisection, and the probes it took.
fn bisect(held: &[Coordinate], before: impl Fn(&Coordinate) -> bool) -> (usize, u32) {
    let (mut lo, mut hi, mut probes) = (0usize, held.len(), 0u32);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        probes += 1;
        match held.get(mid).is_some_and(&before) {
            true => lo = mid + 1,
            false => hi = mid,
        }
    }
    (lo, probes)
}

impl Bodies {
    /// No bodies.
    pub fn new() -> Bodies {
        Bodies { held: Vec::new() }
    }

    /// The bodies, in the canonical order.
    pub fn held(&self) -> &[Coordinate] {
        &self.held
    }

    /// HOLD A BODY at its place in the canonical order, after any body at
    /// the same coordinate; returns the place.
    pub fn hold(&mut self, body: Coordinate) -> usize {
        let at = self.held.partition_point(|c| c <= &body);
        self.held.insert(at, body);
        at
    }

    /// THE NEIGHBOURS OF `at` ON `lane` OVER `[lo, hi)`: the bodies at
    /// `at`'s origin and bell whose lanes before `lane` equal `at`'s and
    /// whose value on `lane` lies in `[lo, hi)`. In the canonical order they
    /// stand together, so two bisections find them; no body outside the
    /// range is read past the probes.
    pub fn range(&self, at: &Coordinate, lane: Lane, lo: &Q, hi: &Q) -> Neighbours<'_> {
        let n = lane.place();
        let key = |c: &Coordinate, v: &Q| -> Ordering {
            (&c.origin, c.local.h, &c.local.lanes[..n], &c.local.lanes[n]).cmp(&(&at.origin, at.local.h, &at.local.lanes[..n], v))
        };
        let (start, p) = bisect(&self.held, |c| key(c, lo) == Ordering::Less);
        let (end, q) = bisect(&self.held, |c| key(c, hi) == Ordering::Less);
        // An empty span (`hi ≤ lo`) ends at or before it starts: no body.
        let bodies = match self.held.get(start..end) {
            Some(b) => b,
            None => &[],
        };
        Neighbours { bodies, probes: p + q }
    }
}

/// THE LANES' RATES: each lane's change per bell, in the order S, G, T, H,
/// each a signed exact rational (78.173 fixture 7).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Rates {
    pub lanes: [Q; 4],
}

impl Rates {
    /// The rate on `lane`.
    pub fn lane(&self, lane: Lane) -> &Q {
        &self.lanes[lane.place()]
    }
}

/// WHY AN ADVANCE, A MERGER OR A FISSION IS REFUSED, by name and with what
/// was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MotionRefusal {
    /// An advance to a bell before the body's: there is no backward
    /// advance. The bell held and the bell asked.
    Backward { held: u64, asked: u64 },
    /// A merger of bodies at two origins: origins are never compared. The
    /// origin held and the origin asked, as written.
    OriginsDiffer { held: Box<Origin>, asked: Box<Origin> },
    /// A merger of bodies at two bells. The bell held and the bell asked.
    BellsDiffer { held: u64, asked: u64 },
    /// Holdings whose sum is zero: no holding-weighted coordinate. The two
    /// holdings.
    NoHolding { a: Box<Q>, b: Box<Q> },
    /// Offsets whose holding-weighted sum is not zero on every lane: no
    /// merger left them. The sum on each lane, S, G, T, H.
    Unbalanced { residual: Box<[Q; 4]> },
    /// Whole turns of H past `K`'s carrier, a `u64`. The whole turns read.
    OrbitsPastCarrier { turns: BigInt },
}

/// `K` READ FROM H: the whole turns of H in both senses, `⌊|H|⌋`, refused
/// `OrbitsPastCarrier` where they pass `K`'s carrier.
fn orbits(lanes: &[Q; 4]) -> Result<u64, MotionRefusal> {
    let turns = lanes[Lane::H.place()].abs().floor().to_integer();
    match turns.to_u64() {
        Some(k) => Ok(k),
        None => Err(MotionRefusal::OrbitsPastCarrier { turns }),
    }
}

/// THE BELL AT WHICH A LANE REACHES A VALUE (78.173 fixture 8), a named
/// reading: reached at an exact bell, or none, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventBell {
    /// Reached at this exact bell, at or after the body's: `h + (v − L)/r`.
    /// It may fall between whole bells.
    At(Q),
    /// None: the lane's rate is 0, so the lane never moves to the value.
    Unmoving,
    /// None: `(v − L)/r < 0`, the lane stood at the value only before the
    /// body's bell, at `bell`, which no advance reaches.
    Behind { bell: Q },
}

impl Coordinate {
    /// THE ADVANCE TO BELL `t` (78.173 fixture 7): each lane
    /// `Lᵢ(t) = Lᵢ(h) + rᵢ·(t − h)` under `rates`, exactly, at the same
    /// origin, with `K` re-read as `⌊|L_H(t)|⌋`. It composes: advancing to
    /// `a` and then to `t` is advancing to `t`, for `h ≤ a ≤ t`. Refused
    /// `Backward` for `t < h`, and `OrbitsPastCarrier` where `K` passes its
    /// carrier.
    pub fn advance(&self, rates: &Rates, t: u64) -> Result<Coordinate, MotionRefusal> {
        let span = t.checked_sub(self.local.h).ok_or(MotionRefusal::Backward { held: self.local.h, asked: t })?;
        let span = Q::from_integer(BigInt::from(span));
        let lanes: [Q; 4] = std::array::from_fn(|i| &self.local.lanes[i] + &rates.lanes[i] * &span);
        let k = orbits(&lanes)?;
        Ok(Coordinate { origin: self.origin.clone(), local: Local { h: t, lanes, k } })
    }

    /// THE BELL AT WHICH `lane` REACHES `value` under `rates` (78.173
    /// fixture 8): `h + (v − L)/r`, exact, solved in first degree with no
    /// period assumed. None, by name, where the rate is 0 or the value lies
    /// behind the body's bell.
    pub fn bell_of(&self, lane: Lane, value: &Q, rates: &Rates) -> EventBell {
        let rate = rates.lane(lane);
        match rate.cmp(&Q::zero()) {
            Ordering::Equal => EventBell::Unmoving,
            _ => {
                let interval = (value - self.lane(lane)) / rate;
                let bell = Q::from_integer(BigInt::from(self.local.h)) + &interval;
                match interval.cmp(&Q::zero()) {
                    Ordering::Less => EventBell::Behind { bell },
                    _ => EventBell::At(bell),
                }
            }
        }
    }
}

/// A MERGER (78.173 fixture 9): the merged point, and each body's offset
/// from it per lane, `L_a − L` and `L_b − L`, in the order S, G, T, H. The
/// holding-weighted sum of the offsets is zero on every lane; with the
/// holdings they give both bodies back ([`fission`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Merger {
    pub point: Coordinate,
    pub offset_a: [Q; 4],
    pub offset_b: [Q; 4],
}

/// THE MERGER OF `a` HOLDING `w_a` AND `b` HOLDING `w_b` (78.173 fixture 9):
/// at their one origin and one bell, each lane
/// `Lᵢ = (w_a·L_a,ᵢ + w_b·L_b,ᵢ)/(w_a + w_b)`, `K` re-read from the merged
/// H, and each body's offset kept: no residual is discarded. Refused
/// `OriginsDiffer`, `BellsDiffer`, or `NoHolding` where `w_a + w_b = 0`.
pub fn merge(a: &Coordinate, w_a: &Q, b: &Coordinate, w_b: &Q) -> Result<Merger, MotionRefusal> {
    if a.origin != b.origin {
        return Err(MotionRefusal::OriginsDiffer { held: Box::new(a.origin.clone()), asked: Box::new(b.origin.clone()) });
    }
    if a.local.h != b.local.h {
        return Err(MotionRefusal::BellsDiffer { held: a.local.h, asked: b.local.h });
    }
    let total = w_a + w_b;
    if total.is_zero() {
        return Err(MotionRefusal::NoHolding { a: Box::new(w_a.clone()), b: Box::new(w_b.clone()) });
    }
    let lanes: [Q; 4] = std::array::from_fn(|i| (w_a * &a.local.lanes[i] + w_b * &b.local.lanes[i]) / &total);
    let offset_a: [Q; 4] = std::array::from_fn(|i| &a.local.lanes[i] - &lanes[i]);
    let offset_b: [Q; 4] = std::array::from_fn(|i| &b.local.lanes[i] - &lanes[i]);
    let k = orbits(&lanes)?;
    Ok(Merger { point: Coordinate { origin: a.origin.clone(), local: Local { h: a.local.h, lanes, k } }, offset_a, offset_b })
}

/// FISSION, THE INVERSE OF MERGER (78.173 fixture 10): the merged point
/// and each offset give back the bodies, `L + offset`, at the point's
/// origin and bell, each `K` re-read from its H. Refused `NoHolding` where
/// `w_a + w_b = 0`, and `Unbalanced` where the holding-weighted sum of the
/// offsets is not zero, since no merger at these holdings left them.
pub fn fission(merger: &Merger, w_a: &Q, w_b: &Q) -> Result<(Coordinate, Coordinate), MotionRefusal> {
    if (w_a + w_b).is_zero() {
        return Err(MotionRefusal::NoHolding { a: Box::new(w_a.clone()), b: Box::new(w_b.clone()) });
    }
    let residual: [Q; 4] = std::array::from_fn(|i| w_a * &merger.offset_a[i] + w_b * &merger.offset_b[i]);
    if residual.iter().any(|r| !r.is_zero()) {
        return Err(MotionRefusal::Unbalanced { residual: Box::new(residual) });
    }
    let body = |offset: &[Q; 4]| -> Result<Coordinate, MotionRefusal> {
        let lanes: [Q; 4] = std::array::from_fn(|i| &merger.point.local.lanes[i] + &offset[i]);
        let k = orbits(&lanes)?;
        Ok(Coordinate { origin: merger.point.origin.clone(), local: Local { h: merger.point.local.h, lanes, k } })
    };
    Ok((body(&merger.offset_a)?, body(&merger.offset_b)?))
}
