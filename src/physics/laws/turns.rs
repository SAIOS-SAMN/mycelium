// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-HOSTING §5a.10, MU-SENSING §3.7 — THE CONFORMAL LAW ON A MOTION STATED
//! AS TURNS: each phase of a frame advanced by a turn in ℚ/ℤ, checked by
//! turn arithmetic and never by a rotor.
//!
//! A rotation by the turn `t` has an exact rotor over ℚ only where `4t` is
//! whole (Niven); every other rational turn is exact over the cyclotomic
//! field `ℚ(ζ_ℓ)` and never over ℚ. So a motion the kernel states as turns
//! is read as turns, whatever its turns are: composed by addition mod 1
//! ([`Turn::plus`]), a place carried phase by phase, and a disagreement held
//! as the residue `t_claim − t_rec` in ℚ/ℤ (§5a.10), never a flag. No turn
//! is converted to a rotor, and none is approximated by one. A turn the
//! frame's periods do not hold (its index read on the phase's period is no
//! whole site) leaves the frame and is refused by name, `LeavesPeriods`.

use super::Law;
use crate::transducer::family::transduce::{Enclosure, Turn};

/// A MOTION STATED AS TURNS: the turn each phase of a frame advances by, in
/// the frame's canonical axis order (MU-SENSING §3.1). A gauge of a
/// one-dimensional reading per phase, never a scalar of the motion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnMotion {
    pub turns: Vec<Turn>,
}

impl TurnMotion {
    /// THE MOTION `self` THEN `next`: phase by phase, the sum in ℚ/ℤ.
    /// Refused `Rank` where the two state different counts of phases, and
    /// `Unread` where a composed period passes the machine word.
    pub fn then(&self, next: &TurnMotion) -> Result<TurnMotion, TurnBreach> {
        if next.turns.len() != self.turns.len() {
            return Err(TurnBreach::Rank { stated: next.turns.len(), phases: self.turns.len() });
        }
        let turns = self
            .turns
            .iter()
            .zip(&next.turns)
            .enumerate()
            .map(|(phase, (a, b))| a.plus(b).ok_or(TurnBreach::Unread { phase }))
            .collect::<Result<Vec<Turn>, TurnBreach>>()?;
        Ok(TurnMotion { turns })
    }

    /// A PLACE CARRIED: each of its turns advanced by the motion's turn on
    /// the same phase, mod 1. Refused as [`TurnMotion::then`].
    pub fn carry(&self, place: &[Turn]) -> Result<Vec<Turn>, TurnBreach> {
        Ok(self.then(&TurnMotion { turns: place.to_vec() })?.turns)
    }
}

/// ONE TRANSITION WHOSE MOTION IS STATED AS TURNS: the frame it acts on
/// (the periods of its phases), the motion, and each body's place before
/// and after it, one turn per phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnTransition {
    pub frame: Enclosure,
    pub motion: TurnMotion,
    pub bodies: Vec<(Vec<Turn>, Vec<Turn>)>,
}

/// HOW A MOTION STATED AS TURNS BREAKS THE CONFORMAL LAW, with what it
/// breaks it by.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurnBreach {
    /// The motion states `stated` turns where the frame has `phases`
    /// phases.
    Rank { stated: usize, phases: usize },
    /// The motion's turn on `phase` leaves the frame's periods: read on the
    /// phase's `period`, it lands on no site.
    LeavesPeriods { phase: usize, turn: Turn, period: usize },
    /// Body `body`'s place states `stated` turns where the frame has
    /// another count of phases.
    PlaceRank { body: usize, stated: usize },
    /// Body `body`'s place after is not its place before carried by the
    /// motion: `residue` is the claimed place less the carried one, per
    /// phase, in ℚ/ℤ, whole on every phase that agrees.
    NotCarried { body: usize, residue: Vec<Turn> },
    /// A sum on `phase` composes onto a period past the machine word: the
    /// law is unread there, never passed.
    Unread { phase: usize },
}

/// THE CONFORMAL LAW'S ANSWER TO A MOTION STATED AS TURNS: every breach
/// with its law. Admitted exactly where none stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TurnVerdict {
    pub breaches: Vec<(Law, TurnBreach)>,
}

impl TurnVerdict {
    /// The conformal law read, and nothing breached.
    pub fn admitted(&self) -> bool {
        self.breaches.is_empty()
    }
}

/// THE CONFORMAL LAW ON A MOTION STATED AS TURNS: the motion holds one turn
/// per phase of the frame, each landing on the phase's period; and every
/// body's place after is its place before carried by the motion, exactly in
/// ℚ/ℤ. Every breach is named; none is decided by a rotor.
pub fn verify_turns(t: &TurnTransition) -> TurnVerdict {
    let mut v = TurnVerdict::default();
    let periods = t.frame.periods();
    if t.motion.turns.len() != periods.len() {
        v.breaches.push((Law::Conformal, TurnBreach::Rank { stated: t.motion.turns.len(), phases: periods.len() }));
        return v;
    }
    for (phase, (turn, &period)) in t.motion.turns.iter().zip(periods).enumerate() {
        if turn.lands(period).is_err() {
            v.breaches.push((Law::Conformal, TurnBreach::LeavesPeriods { phase, turn: *turn, period }));
        }
    }
    for (body, (before, after)) in t.bodies.iter().enumerate() {
        let off = [before, after].into_iter().find(|place| place.len() != periods.len());
        if let Some(place) = off {
            v.breaches.push((Law::Conformal, TurnBreach::PlaceRank { body, stated: place.len() }));
            continue;
        }
        let carried = match t.motion.carry(before) {
            Ok(carried) => carried,
            Err(breach) => {
                v.breaches.push((Law::Conformal, breach));
                continue;
            }
        };
        let residue = after
            .iter()
            .zip(&carried)
            .enumerate()
            .map(|(phase, (claimed, rec))| claimed.less(rec).ok_or(TurnBreach::Unread { phase }))
            .collect::<Result<Vec<Turn>, TurnBreach>>();
        match residue {
            Ok(residue) if residue.iter().all(Turn::is_whole) => {}
            Ok(residue) => v.breaches.push((Law::Conformal, TurnBreach::NotCarried { body, residue })),
            Err(breach) => v.breaches.push((Law::Conformal, breach)),
        }
    }
    v
}
