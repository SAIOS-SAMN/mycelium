// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE COORDINATE (LAW-COORDINATE v3, proposal 78.173; ruled by both
//! operators 2026-11-05 through MU-RELEASE §3a, v3's points agreed with
//! netstratum and xylarium the same day), the standard every kernel
//! addresses its bodies by. Netstratum's statement is docs/120, "The
//! coordinate: TUniversal inside the universe" (§8b: a lane is a vector).
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
//! - the LANES are its cumulative change by motion type ([`Lanes`]): S
//!   spatial (boost and dilation, the reach), G gauge (change of kind), T
//!   temporal (null flux: the beat and the passages), H holonomic (the
//!   turnings between planes);
//! - `K` counts the orbits it closed, one count per H component.
//!
//! A LANE IS A VECTOR ([`Vector`]): exact signed components, each keyed by
//! its axis ([`Key`]), never summed into one number. H's components are
//! keyed by their GENERATORS, the plane each turns in named by its two axes
//! (`e_Z1Q1` a frame plane, `e_Z1Z2` the turning τ₁₂ between two planes,
//! a component of its own), and stored in DEGREES, exact: a component is a
//! motion, its state at a bell and a rate in degrees per bell, advanced in
//! closed form, `θ(t) = θ(h) + ω·(t − h)`, on the universe's own bell
//! (TUniversal stamps an act and never drives it). `K_c = ⌊|θ_c| / 360⌋`
//! per component, 360 the degree's definition ([`DEGREES_PER_TURN`]). S, G
//! and T hold their components in their own units.
//!
//! OPEN LANES. S, G, T and H are the base lanes. A lane set is a list of
//! named lanes, never a fixed four; a further lane ([`Lane::Opened`], named
//! with its law version) is admitted only through a ruled opening
//! ([`Openings`]), never by convention, and is refused `LaneNotOpened` by
//! name otherwise. No opening is ruled yet: the ruled act, agreed between
//! kernels, is the next step (78.173 §7).
//!
//! THE ORDERED KEY. The canonical order — origin, `h`, the lanes in order
//! (S, G, T, H, then any opened lane), each lane's components by their keys
//! written out, each by exact value, then `K` — is the index a kernel
//! addresses its bodies by ([`Bodies`]). AN ADDRESS ROUTES AND NEVER
//! IDENTIFIES: the order locates a body and finds its neighbours, and a
//! body after another in it is neither more nor less than it. Neighbours
//! are a range on one component ([`Bodies::range`]), found by bisection,
//! any other component a filter within it: never a test of all pairs.
//!
//! DECISIONS ON THE CALLER'S INTERVALS. [`Coordinate::within`] and the
//! ranges decide on exact intervals the caller states ([`Interval`],
//! [`Bounds`]): a side per component ([`Sides`]), within only where every
//! component is within. THE SEXAGESIMAL READING IS DISPLAY ONLY
//! ([`Sector`], [`Reading`]): at depth `d` a value lies in one sector
//! `[m·60⁻ᵈ, (m + 1)·60⁻ᵈ)`, the components of a lane written joined by
//! ` × `, an H component read directly as degrees;minutes,seconds
//! (`51;25,42`). Nothing is compared, stored or decided by it, and no
//! number base is assumed (Concordance 1.6).
//!
//! ORIGINS ARE NEVER COMPARED. A reading between two bodies of different
//! origins is refused [`CoordinateRefusal::OriginsDiffer`] by name. The
//! key's order puts each origin's bodies together, by the origin written
//! out; it orders writings and measures nothing across them.
//!
//! TUNIVERSAL'S POINT IS THE NON-NEGATIVE CASE. `TPoint` (78.152 TU-5)
//! reads as a coordinate with no enclosures and one-component non-negative
//! lanes: the same writing and the same order (claimed in
//! `tests/coordinate.rs`). The map from a body's signed lanes inside a
//! universe to the non-negative issuance reading at the vertex is OPEN and
//! not yet law. Rewiring `TPoint` onto this form is the next step.
//!
//! MOTION, PER COMPONENT (78.173 fixtures 7–10, 13–15; [`Coordinate::advance`],
//! [`Coordinate::bell_of`], [`merge`], [`fission`]): the advance composes and
//! a backward one is refused; an event's bell is the exact first-degree
//! solution; a merger keeps each body's offsets, whose holding-weighted sum
//! is zero; fission inverts it. A PASSAGE ([`Coordinate::pass`]) through a
//! throat goes by `forms::passage`'s pole swap: across a throat it is the
//! RE-ORIGIN, S's span reversed with `|S|` kept and `T` gaining
//! `−2·span`, the new origin reading reach and hold exchanged, `H + R`
//! kept and `δ ↦ 1 − δ`; within one enclosure it is that face exchange
//! alone. The share is invariant under dilation, `δ(λH, λR) = δ(H, R)`,
//! and so is the [`flow`] between two such shares.

mod lanes;
mod motion;
mod reading;

pub use lanes::{Axis, FormRefusal, Key, Lane, Lanes, Openings, Shapes, Vector};
pub use motion::{fission, flow, merge, EventBell, Merger, MotionRefusal, Passed, Rates, Through};
pub use reading::{Bodies, Bounds, Interval, Neighbours, Places, Reading, Sector, Sides, Whole};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-COORDINATE.
pub const LAW_ID: &str = "FORMS-COORDINATE";

/// The form's version: an amendment, or an operation added to the form,
/// raises it.
/// FORMAT: the shell's version, the form of LAW-COORDINATE v3 (proposal
/// 78.173): lanes as keyed vectors, H in degrees with `K` per component,
/// open lanes, decisions on the caller's intervals, the re-origin.
pub const LAW_VERSION: u16 = 3;

/// THE DISPLAY'S BASE: the places of the sexagesimal reading.
/// FORMAT: display only (LAW-COORDINATE v3 §1, ruled 2026-11-05): the
/// sexagesimal writing of a reading; no decision reads it.
pub const BASE: u32 = 60;

/// THE DEGREES OF ONE WHOLE TURN about a named plane: `K_c` counts H's
/// whole turns by it.
/// EXTERNAL: the degree's definition, a whole turn is 360 degrees
/// (LAW-COORDINATE v3 §1, operator 2026-11-05: angles in degrees with the
/// plane named).
pub const DEGREES_PER_TURN: u32 = 360;

/// A BODY'S PLACE WITHIN ITS ORIGIN: the bell `h`, the lane set, and `K`,
/// the orbits closed, one per H component in H's key order. Ordered as
/// written: `h`, the lanes, `K`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Local {
    pub h: u64,
    pub lanes: Lanes,
    pub k: Vec<u64>,
}

/// `K`'s writing: one count bare, in 16 lowercase hex digits; any other
/// number of counts `(…,…)`.
fn orbits_text(k: &[u64]) -> String {
    match k {
        [one] => format!("{one:016x}"),
        all => {
            let parts: Vec<String> = all.iter().map(|c| format!("{c:016x}")).collect();
            format!("({})", parts.join(","))
        }
    }
}

impl Local {
    /// The vector on `lane`.
    pub fn lane(&self, lane: &Lane) -> Option<&Vector> {
        self.lanes.get(lane)
    }

    /// The writing `h:S:G:T:H:K`, any opened lane after H in lane order:
    /// `h` in 16 lowercase hex digits, each lane its vector's exact writing
    /// (a one-component lane bare, `n/d`; a vector `(n/d,n/d)`), and `K`
    /// as its counts are — TPoint's writing (78.152 TU-5) where every lane
    /// holds one non-negative component.
    pub fn written(&self) -> String {
        let mut fields = vec![format!("{:016x}", self.h)];
        fields.extend(self.lanes.held().iter().map(|(_, v)| v.written()));
        fields.push(orbits_text(&self.k));
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
/// Its order is the canonical key: origin, `h`, the lanes in order with
/// each lane's components by key, `K`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub origin: Origin,
    pub local: Local,
}

impl Coordinate {
    /// The full writing `origin/h:S:G:T:H:K`, the origin's enclosures
    /// between: TPoint's writing (`TPoint::print`) where the origin has no
    /// enclosure and every lane holds one non-negative component.
    pub fn written(&self) -> String {
        format!("{}/{}", self.origin.written(), self.local.written())
    }

    /// The vector on `lane`.
    pub fn lane(&self, lane: &Lane) -> Option<&Vector> {
        self.local.lane(lane)
    }
}

/// WHY A READING BETWEEN COORDINATES IS REFUSED, by name and with what was
/// read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoordinateRefusal {
    /// Two bodies, or a body and the bounds stated at an origin, of
    /// different origins: never compared. The origin held and the origin
    /// asked, as written.
    OriginsDiffer { held: Box<Origin>, asked: Box<Origin> },
    /// Bounds whose keys are not the lane's: the lane, the keys the body
    /// holds on it and the keys asked.
    ShapesDiffer { lane: Lane, held: Vec<Key>, asked: Vec<Key> },
    /// A range on a component the body it is read at does not hold.
    NoComponent { lane: Lane, key: Key },
}

/// WHERE A VALUE STANDS AGAINST AN INTERVAL `[lo, hi)`: below its lower
/// bound, within, or at or past its upper bound — decided exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Below,
    Within,
    Above,
}
