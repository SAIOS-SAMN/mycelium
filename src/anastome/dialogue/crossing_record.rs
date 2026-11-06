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
//! - **Holonomy**: each ring the crossing closes, its holonomy a versor,
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
use crate::transducer::family::transduce::Turn;

/// A BODY HELD BY ITS BOUNDARY: per phase of the frame the turns at which
/// the body's boundary breaks, in order, and the share of the frame the body
/// takes, an exact rational in `[0, 1]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnBody {
    pub breakpoints: Vec<Vec<Turn>>,
    pub extent: Q,
}

/// THE MOTION from the state before to the state after.
#[derive(Clone, Debug, PartialEq)]
pub enum Motion {
    /// A versor of `Cl(n+1, 1)`(ℚ).
    Versor(Mv),
    /// One turn per phase where no exact versor exists, the winding kept.
    Turns(Vec<Turn>),
}

/// THE CONTENT: the bodies before and after and the motion between them.
#[derive(Clone, Debug, PartialEq)]
pub struct Content {
    pub before: Vec<TurnBody>,
    pub after: Vec<TurnBody>,
    pub motion: Motion,
}

/// THE PLACE: the act's TUniversal point and its past cone.
#[derive(Clone, Debug)]
pub struct Place {
    pub at: TPoint,
    pub parents: Vec<Parent>,
}

/// THE CLAUSES the act is read under, the final points it cites, and the
/// signing events it stands on, by their stamps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clauses {
    pub read: Vec<ConcordanceClauseId>,
    pub cited: Vec<TPoint>,
    pub signings: Vec<Stamp>,
}

/// THE CROSSING RECORD.
#[derive(Clone, Debug)]
pub struct Crossing {
    pub place: Place,
    pub content: Content,
    pub holonomy: Vec<Mv>,
    pub clauses: Clauses,
}

/// WHY A CROSSING RECORD DOES NOT READ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossingRefusal {
    /// The record carries no content where the object belongs: an address
    /// only (MU-HOSTING §5a.10, ruled #160).
    AddressOnly,
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

fn put_mvs(out: &mut Vec<u8>, ms: &[Mv]) {
    put_key_count(out, ms.len());
    ms.iter().for_each(|m| m.to_bytes(out));
}

fn take_mvs(b: &[u8], p: &mut usize) -> Option<Vec<Mv>> {
    (0..take_key_count(b, p)?).map(|_| Mv::from_bytes(b, p)).collect()
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

fn put_bodies(out: &mut Vec<u8>, bodies: &[TurnBody]) -> Option<()> {
    put_key_count(out, bodies.len());
    bodies.iter().try_for_each(|x| x.put(out))
}

fn take_bodies(b: &[u8], p: &mut usize) -> Option<Vec<TurnBody>> {
    (0..take_key_count(b, p)?).map(|_| TurnBody::take(b, p)).collect()
}

impl Content {
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
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
        let (before, after) = (take_bodies(b, p)?, take_bodies(b, p)?);
        let tag = *b.get(*p)?;
        *p += 1;
        let motion = match tag {
            1 => Motion::Versor(Mv::from_bytes(b, p)?),
            2 => Motion::Turns(take_turns(b, p)?),
            _ => return None,
        };
        Some(Content { before, after, motion })
    }
}

impl Crossing {
    /// THE RECORD WRITTEN: `[place: point, parents][content tag 1][content]
    /// [holonomy: count, versor …][clauses: ids, cited points, signing
    /// stamps]`. `None` where a turn cannot be written.
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        self.place.at.put(out);
        put_key_count(out, self.place.parents.len());
        self.place.parents.iter().for_each(|x| x.put(out));
        out.push(1);
        self.content.put(out)?;
        put_mvs(out, &self.holonomy);
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
        let m = CrossingRefusal::Malformed;
        let at = TPoint::take(b, p).ok_or(m)?;
        let parents = (0..take_key_count(b, p).ok_or(m)?).map(|_| Parent::take(b, p)).collect::<Option<Vec<_>>>().ok_or(m)?;
        let tag = *b.get(*p).ok_or(m)?;
        *p += 1;
        let content = match tag {
            0 => return Err(CrossingRefusal::AddressOnly),
            1 => Content::take(b, p).ok_or(m)?,
            _ => return Err(m),
        };
        let holonomy = take_mvs(b, p).ok_or(m)?;
        let read = (0..take_key_count(b, p).ok_or(m)?).map(|_| take_clause_id(b, p)).collect::<Option<Vec<_>>>().ok_or(m)?;
        let cited = (0..take_key_count(b, p).ok_or(m)?).map(|_| TPoint::take(b, p)).collect::<Option<Vec<_>>>().ok_or(m)?;
        let signings = (0..take_key_count(b, p).ok_or(m)?).map(|_| Stamp::take_canonical(b, p)).collect::<Option<Vec<_>>>().ok_or(m)?;
        Ok(Crossing { place: Place { at, parents }, content, holonomy, clauses: Clauses { read, cited, signings } })
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

/// A RECORD WITH NO CONTENT, as a crossing that carries only an address
/// writes it: its place, the content tag 0, nothing after. What a reader
/// refuses `AddressOnly`.
pub fn put_address_only(at: &TPoint, parents: &[Parent], out: &mut Vec<u8>) {
    at.put(out);
    put_key_count(out, parents.len());
    parents.iter().for_each(|x| x.put(out));
    out.push(0);
}
