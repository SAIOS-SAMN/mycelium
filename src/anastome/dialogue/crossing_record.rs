// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// ROADMAP track 25 X1; MU-HOSTING §5a.10; issue #160 (ruled 2026-11-06) —
// THE CROSSING RECORD: what every universe act carries through the universal
// core tube, written as the exact object and never as its digest.
//
//! A crossing has four parts:
//! - **Place**: the act's TUniversal point and its past cone, which is the
//!   receipt's `parents` (the causal partial order, never totalised).
//! - **Content**: the bodies before and after, each held by its boundary
//!   ([`TurnBody`]: breakpoints per phase and an extent share of the frame),
//!   and the motion from one state to the other, a `Cl(n+1,1)`(ℚ) versor or,
//!   where no exact versor exists (Niven), turns per phase with the winding
//!   kept.
//! - **Holonomy**: each ring the crossing closes ([`Ring`]), named by its
//!   vertices' coordinates in order from the least, its holonomy a versor,
//!   always. A turn is a reading of it on a named plane, never its writing.
//! - **Clauses**: the clauses the act is read under, the final points it
//!   cites, and the signing events it stands on (by stamp, §5a.4).
//!
//! A crossing whose content is absent carries only an address where the
//! object belongs, and is refused `AddressOnly` (ruled). Every value is
//! decoded to its exact form; nothing is compared by bytes.

use crate::chronometry::tuniversal::TPoint;
use crate::chronometry::{Parent, Stamp};
use crate::core::Q;
use crate::physics::laws::clauses::writing::{put_clause_id, take_clause_id};
use crate::physics::laws::clauses::ConcordanceClauseId;
use crate::transducer::address::{put_key_count, take_key_count};
use crate::transducer::alphabet::rotor::{put_q, take_q};
use crate::transducer::cga::Mv;
use crate::physics::laws::{verify_turns, TurnBreach, TurnMotion, TurnTransition};
use crate::transducer::family::transduce::{Enclosure, Turn};

/// The domain of the bytes a universe act is signed over with its crossing,
/// so the record is never read as a part of the act before it.
const CROSSED: &[u8] = b"mycelium crossing x1";

/// X1 (#160) — THE BYTES A UNIVERSE ACT IS SIGNED OVER: the act's own
/// signed bytes, then its crossing record under its own domain, written out
/// whole, never a digest of it. `None` where a turn cannot be written.
pub fn crossed(mut act: Vec<u8>, c: &Crossing) -> Option<Vec<u8>> {
    act.extend_from_slice(CROSSED);
    c.put(&mut act)?;
    Some(act)
}

/// A BODY HELD BY ITS BOUNDARY: per phase of the frame the turns at which
/// the body's boundary breaks, in order, and the share of the frame the body
/// takes, an exact rational in `[0, 1]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnBody {
    pub breakpoints: Vec<Vec<Turn>>,
    pub extent: Q,
}

/// THE MOTION from the state before to the state after.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Motion {
    /// A versor of `Cl(n+1, 1)`(ℚ).
    Versor(Mv),
    /// One turn per phase where no exact versor exists, the winding kept.
    Turns(Vec<Turn>),
}

/// A BODY AS THE RECORD CARRIES IT: its place, the origin it stands at on
/// each phase of the frame (one turn per phase), beside its boundary
/// (ruled 2026-11-06, #160: the record is verifiable from itself alone).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub origin: Vec<Turn>,
    pub boundary: TurnBody,
}

/// THE CONTENT: the frame's periods per phase, stated once for the record,
/// the bodies before and after, and the motion between them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Content {
    pub frame: Vec<usize>,
    pub before: Vec<Body>,
    pub after: Vec<Body>,
    pub motion: Motion,
}

/// THE PLACE: the act's TUniversal point and its past cone.
#[derive(Clone, Debug)]
pub struct Place {
    pub at: TPoint,
    pub parents: Vec<Parent>,
}

/// Two places are one where they stand at one point and cite one past cone
/// in one order, each citation compared in its link sense
/// (`Predecessor::cites`, MU-HOSTING §5a.10), never by bytes.
impl PartialEq for Place {
    fn eq(&self, other: &Place) -> bool {
        self.at == other.at
            && self.parents.len() == other.parents.len()
            && self.parents.iter().zip(&other.parents).all(|(a, b)| crate::court::admission::Predecessor::cites(a, b))
    }
}

impl Eq for Place {}

/// THE CLAUSES the act is read under, the final points it cites, and the
/// signing events it stands on, by their stamps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clauses {
    pub read: Vec<ConcordanceClauseId>,
    pub cited: Vec<TPoint>,
    pub signings: Vec<Stamp>,
}

/// A RING THE CROSSING CLOSES, named by its vertices' coordinates written
/// out in order around it (A21: content, never a digest or an index), with
/// its holonomy. The writing starts at the least vertex in the coordinates'
/// canonical order (`TPoint`'s own), so two writings that are rotations of
/// one cycle are one ring; the orientation is kept as stated, so a ring and
/// its reverse are two names, whose holonomies are inverse (ruled
/// 2026-11-06, #160).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ring {
    vertices: Vec<TPoint>,
    pub holonomy: Mv,
}

impl Ring {
    /// THE RING THROUGH `vertices` in the order stated, its writing rotated
    /// to start at the least vertex, the orientation kept. `None` where it
    /// has no vertex or a vertex twice (no least start is one).
    pub fn new(mut vertices: Vec<TPoint>, holonomy: Mv) -> Option<Ring> {
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — canonical key order: a ring's writing starts at its least vertex in TPoint's canonical coordinate order, as ruled (#160); a coordinate order, never a choice by a measure"
        )]
        let least = vertices.iter().enumerate().min_by(|a, b| a.1.cmp(b.1)).map(|(i, _)| i)?;
        vertices.rotate_left(least);
        Ring::canonical(vertices, holonomy).ok()
    }

    /// A writing taken as it stands: refused `RingStart` where it does not
    /// start at its least vertex, `Malformed` where it is empty or holds a
    /// vertex twice.
    fn canonical(vertices: Vec<TPoint>, holonomy: Mv) -> Result<Ring, CrossingRefusal> {
        let (first, rest) = vertices.split_first().ok_or(CrossingRefusal::Malformed)?;
        if rest.iter().any(|v| v == first) {
            return Err(CrossingRefusal::Malformed);
        }
        if rest.iter().any(|v| v < first) {
            return Err(CrossingRefusal::RingStart);
        }
        let mut seen: Vec<&TPoint> = vertices.iter().collect();
        seen.sort();
        if seen.windows(2).any(|w| w[0] == w[1]) {
            return Err(CrossingRefusal::Malformed);
        }
        Ok(Ring { vertices, holonomy })
    }

    /// The vertices in order around the ring, from the least.
    pub fn vertices(&self) -> &[TPoint] {
        &self.vertices
    }

    /// THE RING REVERSED: the same vertices the other way round, from the
    /// least, its holonomy the inverse. `None` where the holonomy has no
    /// exact inverse.
    pub fn reversed(&self) -> Option<Ring> {
        let mut back = self.vertices.clone();
        back.reverse();
        Ring::new(back, crate::transducer::cga::inverse_even(&self.holonomy)?)
    }

    fn put(&self, out: &mut Vec<u8>) {
        put_key_count(out, self.vertices.len());
        self.vertices.iter().for_each(|v| v.put(out));
        self.holonomy.to_bytes(out);
    }

    fn take(b: &[u8], p: &mut usize) -> Result<Ring, CrossingRefusal> {
        const M: CrossingRefusal = CrossingRefusal::Malformed;
        let vertices = (0..take_key_count(b, p).ok_or(M)?).map(|_| TPoint::take(b, p)).collect::<Option<Vec<_>>>().ok_or(M)?;
        let holonomy = Mv::from_bytes(b, p).ok_or(M)?;
        Ring::canonical(vertices, holonomy)
    }
}

/// THE CROSSING RECORD.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crossing {
    pub place: Place,
    pub content: Content,
    pub holonomy: Vec<Ring>,
    pub clauses: Clauses,
}

/// WHY A CROSSING RECORD DOES NOT READ, or does not verify from itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrossingRefusal {
    /// A universe act that carries no crossing record at all (X1: every
    /// universe act carries one).
    Absent,
    /// The content states no frame: no phase, or a phase of period 0
    /// (ruled 2026-11-06, #160).
    NoFrame,
    /// Body `body` (counted through the bodies before, then after) states
    /// no origin (ruled 2026-11-06, #160).
    NoPlace { body: usize },
    /// A motion stated as turns over `before` bodies before and `after`
    /// after: the turns carry each body before onto the one after it, and
    /// two counts pair no bodies.
    Unpaired { before: usize, after: usize },
    /// The motion stated as turns breaks the conformal law
    /// (`verify_turns`), each breach named with what it breaks it by.
    Turns(Vec<TurnBreach>),
    /// The record carries no content where the object belongs: an address
    /// only (MU-HOSTING §5a.10, ruled #160).
    AddressOnly,
    /// A ring's writing does not start at its least vertex: a rotation of
    /// the one writing, refused rather than read as another ring.
    RingStart,
    /// The bytes do not read whole as a record: a part cut short, a byte
    /// left over, a turn whose winding is not its own, an extent outside
    /// `[0, 1]`.
    Malformed,
}

fn put_turns(out: &mut Vec<u8>, turns: &[Turn]) -> Option<()> {
    put_key_count(out, turns.len());
    turns.iter().try_for_each(|t| t.put(out))
}

fn take_turns(b: &[u8], p: &mut usize) -> Option<Vec<Turn>> {
    (0..take_key_count(b, p)?).map(|_| Turn::take(b, p)).collect()
}

fn put_rings(out: &mut Vec<u8>, rings: &[Ring]) {
    put_key_count(out, rings.len());
    rings.iter().for_each(|r| r.put(out));
}

fn take_rings(b: &[u8], p: &mut usize) -> Result<Vec<Ring>, CrossingRefusal> {
    (0..take_key_count(b, p).ok_or(CrossingRefusal::Malformed)?).map(|_| Ring::take(b, p)).collect()
}

impl TurnBody {
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        put_key_count(out, self.breakpoints.len());
        self.breakpoints.iter().try_for_each(|phase| put_turns(out, phase))?;
        put_q(out, &self.extent);
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<TurnBody> {
        let breakpoints = (0..take_key_count(b, p)?).map(|_| take_turns(b, p)).collect::<Option<Vec<_>>>()?;
        let extent = take_q(b, p)?;
        let share = extent >= Q::from_integer(0.into()) && extent <= Q::from_integer(1.into());
        share.then_some(TurnBody { breakpoints, extent })
    }
}

impl Body {
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        put_turns(out, &self.origin)?;
        self.boundary.put(out)
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Body> {
        Some(Body { origin: take_turns(b, p)?, boundary: TurnBody::take(b, p)? })
    }
}

fn put_bodies(out: &mut Vec<u8>, bodies: &[Body]) -> Option<()> {
    put_key_count(out, bodies.len());
    bodies.iter().try_for_each(|x| x.put(out))
}

fn take_bodies(b: &[u8], p: &mut usize) -> Option<Vec<Body>> {
    (0..take_key_count(b, p)?).map(|_| Body::take(b, p)).collect()
}

fn put_frame(out: &mut Vec<u8>, frame: &[usize]) -> Option<()> {
    put_key_count(out, frame.len());
    frame.iter().try_for_each(|h| {
        out.extend_from_slice(&u64::try_from(*h).ok()?.to_le_bytes());
        Some(())
    })
}

fn take_frame(b: &[u8], p: &mut usize) -> Option<Vec<usize>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let h = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            usize::try_from(h).ok()
        })
        .collect()
}

impl Content {
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        put_frame(out, &self.frame)?;
        put_bodies(out, &self.before)?;
        put_bodies(out, &self.after)?;
        match &self.motion {
            Motion::Versor(m) => {
                out.push(1);
                m.to_bytes(out);
            }
            Motion::Turns(ts) => {
                out.push(2);
                put_turns(out, ts)?;
            }
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Content> {
        let frame = take_frame(b, p)?;
        let (before, after) = (take_bodies(b, p)?, take_bodies(b, p)?);
        let tag = *b.get(*p)?;
        *p += 1;
        let motion = match tag {
            1 => Motion::Versor(Mv::from_bytes(b, p)?),
            2 => Motion::Turns(take_turns(b, p)?),
            _ => return None,
        };
        Some(Content { frame, before, after, motion })
    }
}

impl Crossing {
    /// THE RECORD WRITTEN: `[place: point, parents][content tag 1][content]
    /// [holonomy: count, (vertex count, vertex …, versor) …][clauses: ids,
    /// cited points, signing stamps]`. `None` where a turn cannot be
    /// written.
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        self.place.at.put(out);
        put_key_count(out, self.place.parents.len());
        self.place.parents.iter().for_each(|x| x.put(out));
        out.push(1);
        self.content.put(out)?;
        put_rings(out, &self.holonomy);
        put_key_count(out, self.clauses.read.len());
        self.clauses.read.iter().for_each(|id| put_clause_id(out, id));
        put_key_count(out, self.clauses.cited.len());
        self.clauses.cited.iter().for_each(|t| t.put(out));
        put_key_count(out, self.clauses.signings.len());
        self.clauses.signings.iter().for_each(|s| s.put_canonical(out));
        Some(())
    }

    /// A record read from `b` at `p`, each value decoded to its exact form.
    /// A record whose content tag is 0 carries no content and is refused
    /// `AddressOnly`; anything that does not read is `Malformed`.
    pub fn take(b: &[u8], p: &mut usize) -> Result<Crossing, CrossingRefusal> {
        const M: CrossingRefusal = CrossingRefusal::Malformed;
        let at = TPoint::take(b, p).ok_or(M)?;
        let parents = (0..take_key_count(b, p).ok_or(M)?).map(|_| Parent::take(b, p)).collect::<Option<Vec<_>>>().ok_or(M)?;
        let tag = *b.get(*p).ok_or(M)?;
        *p += 1;
        let content = match tag {
            0 => return Err(CrossingRefusal::AddressOnly),
            1 => Content::take(b, p).ok_or(M)?,
            _ => return Err(M),
        };
        let holonomy = take_rings(b, p)?;
        let read = (0..take_key_count(b, p).ok_or(M)?).map(|_| take_clause_id(b, p)).collect::<Option<Vec<_>>>().ok_or(M)?;
        let cited = (0..take_key_count(b, p).ok_or(M)?).map(|_| TPoint::take(b, p)).collect::<Option<Vec<_>>>().ok_or(M)?;
        let signings = (0..take_key_count(b, p).ok_or(M)?).map(|_| Stamp::take_canonical(b, p)).collect::<Option<Vec<_>>>().ok_or(M)?;
        Ok(Crossing { place: Place { at, parents }, content, holonomy, clauses: Clauses { read, cited, signings } })
    }

    /// A CROSSING THAT MOVES NO BODY: at `place`, over a frame of the
    /// periods `frame`, no body before or after, the motion the identity
    /// versor, no ring closed and no clause read. What an act that carries
    /// no body across states.
    pub fn still(place: Place, frame: Vec<usize>) -> Crossing {
        let one = Mv::scalar(Q::from_integer(1.into()));
        Crossing {
            place,
            content: Content { frame, before: Vec::new(), after: Vec::new(), motion: Motion::Versor(one) },
            holonomy: Vec::new(),
            clauses: Clauses { read: Vec::new(), cited: Vec::new(), signings: Vec::new() },
        }
    }

    /// THE RECORD VERIFIED FROM ITSELF ALONE, as admission reads it (ruled
    /// 2026-11-06, #160): the content states its frame (`NoFrame`) and every
    /// body its origin (`NoPlace`); and a motion stated as turns is read by
    /// the conformal law ([`verify_turns`]) over the frame, each body's
    /// origin before carried onto its origin after, every breach named
    /// (`Turns`). A motion that is a versor is reconstructed with X2.
    pub fn verify(&self) -> Result<(), CrossingRefusal> {
        let c = &self.content;
        let frame = Enclosure::new(c.frame.clone()).map_err(|_| CrossingRefusal::NoFrame)?;
        if let Some(body) = c.before.iter().chain(&c.after).position(|x| x.origin.is_empty()) {
            return Err(CrossingRefusal::NoPlace { body });
        }
        let Motion::Turns(turns) = &c.motion else { return Ok(()) };
        if c.before.len() != c.after.len() {
            return Err(CrossingRefusal::Unpaired { before: c.before.len(), after: c.after.len() });
        }
        let bodies = c.before.iter().zip(&c.after).map(|(x, y)| (x.origin.clone(), y.origin.clone())).collect();
        let verdict = verify_turns(&TurnTransition { frame, motion: TurnMotion { turns: turns.clone() }, bodies });
        match verdict.breaches.is_empty() {
            true => Ok(()),
            false => Err(CrossingRefusal::Turns(verdict.breaches.into_iter().map(|(_, b)| b).collect())),
        }
    }

    /// The record read whole from `b`: a byte left over is `Malformed`.
    pub fn from_bytes(b: &[u8]) -> Result<Crossing, CrossingRefusal> {
        let mut p = 0;
        let c = Crossing::take(b, &mut p)?;
        match p == b.len() {
            true => Ok(c),
            false => Err(CrossingRefusal::Malformed),
        }
    }
}

/// The tests' crossing: [`Crossing::still`] at the zero point of a zero
/// origin, over a frame of one phase of period 1, citing nothing.
#[cfg(test)]
pub(crate) fn still() -> Crossing {
    use crate::chronometry::tuniversal::Origin;
    let at = TPoint { origin: Origin { genesis: [0; 32], epoch: 0 }, h: 0, lanes: [0, 0, 0, 0].into(), k: 0 };
    Crossing::still(Place { at, parents: Vec::new() }, vec![1])
}

/// The tests' universe acts signed with [`still`] (X1): each act's own
/// `sign` with the crossing it carries.
#[cfg(test)]
pub(crate) trait SignedStill {
    type Signed;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed;
}

#[cfg(test)]
impl SignedStill for super::atlas::LedgerEntry {
    type Signed = super::atlas::SignedLedger;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed {
        self.sign(still(), key).expect("the tests' crossing writes")
    }
}

#[cfg(test)]
impl SignedStill for super::atlas::MapAct {
    type Signed = super::atlas::SignedMap;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed {
        self.sign(still(), key).expect("the tests' crossing writes")
    }
}

#[cfg(test)]
impl SignedStill for super::universe::Statement {
    type Signed = super::universe::SignedStatement;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed {
        self.sign(still(), key).expect("the tests' crossing writes")
    }
}

#[cfg(test)]
impl SignedStill for super::endowment::Manifest {
    type Signed = Option<super::endowment::SignedManifest>;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed {
        self.sign(still(), key)
    }
}

#[cfg(test)]
impl SignedStill for super::uplink::TierFold {
    type Signed = super::uplink::SignedFold;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed {
        self.sign_crossed(still(), key).expect("the tests' crossing writes")
    }
}

#[cfg(test)]
impl SignedStill for super::handoff::Handoff {
    type Signed = super::handoff::SignedHandoff;
    fn signed(self, key: &ed25519_dalek::SigningKey) -> Self::Signed {
        match self.crosses() {
            true => self.sign_crossed(still(), key).expect("the tests' crossing writes"),
            false => self.sign(key),
        }
    }
}

/// A UNIVERSE ACT'S CROSSING AT ADMISSION: refused `Absent` where the act
/// carries none, else verified from itself ([`Crossing::verify`]).
pub fn verify_carried(c: Option<&Crossing>) -> Result<(), CrossingRefusal> {
    c.ok_or(CrossingRefusal::Absent)?.verify()
}

/// A RECORD WITH NO CONTENT, as a crossing that carries only an address
/// writes it: its place, the content tag 0, nothing after. What a reader
/// refuses `AddressOnly`.
pub fn put_address_only(at: &TPoint, parents: &[Parent], out: &mut Vec<u8>) {
    at.put(out);
    put_key_count(out, parents.len());
    parents.iter().for_each(|x| x.put(out));
    out.push(0);
}
