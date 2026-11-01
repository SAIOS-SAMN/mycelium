// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.125 (law version 5, ruled 2026-10-31; MU-DERIVATIONS D1.1) — REACH AND
// RETURN: a window opened on a court closes once every other keeper has
// verified a reading of that court at or above the opening, and spoken again.
//
//! The reading is taken on the keepers' own signed citations (TU-2), never on
//! a count of the court's heights. For each other keeper `k`, among its
//! readings held that have a later one, the reading citing the court highest
//! is `k`'s witness of reach, and its newest reading its witness of return;
//! the cited height is how far `k` has carried the court. The reach of the
//! court is the least of these over every other keeper: every window opened
//! at or below it has reached every keeper and returned. Comparisons of
//! heights only; no sum, product or ratio is formed.

use super::point::Origin;
use super::reading::{Cited, TickReading};

/// WHY A WINDOW DOES NOT CLOSE, by name and with the keeper it waits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowRefusal {
    /// The court holds no keeper but itself: none can witness, and one
    /// keeper alone finalises nothing (TU-3).
    NoOtherKeeper,
    /// This keeper has not yet cited the court and read again after.
    Unwitnessed { keeper: [u8; 32] },
    /// A witness reading does not verify under its keeper at the origin.
    Unsigned { keeper: [u8; 32] },
    /// A witness pair breaks reach and return: the two readings are not one
    /// other keeper's, the reach reading cites the court below the reach
    /// claimed, or the return does not stand above the reach.
    Mismatched { keeper: [u8; 32] },
    /// The keepers' reach stands below the window's opening: not yet carried.
    Unreached { reach: u64 },
    /// A window of law version 4 whose end passes the height carrier: it
    /// never closes (rule 11).
    PastCarrier,
}

/// ONE KEEPER'S WITNESS: the reading that carried the court, and a later
/// reading of the same keeper, which returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    pub reach: TickReading,
    pub back: TickReading,
}

/// THE COURT'S REACH: how far every other keeper has carried the court
/// (`through`, a height of the court), with each keeper's witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reach {
    pub origin: Origin,
    pub court: [u8; 32],
    pub through: u64,
    pub witnesses: Vec<Witness>,
}

/// ONE KEEPER'S READINGS AS A COURT HOLDS THEM, rising by tick.
#[derive(Debug, Clone)]
pub struct KeeperLine<'a> {
    pub keeper: [u8; 32],
    pub readings: Vec<&'a TickReading>,
}

/// The height `reading` cites of `court`, where it cites it.
fn cite_of(reading: &TickReading, court: &[u8; 32]) -> Option<u64> {
    reading.cites.iter().find(|c| c.keeper == *court).map(|c| c.h)
}

impl Reach {
    /// THE REACH OF `court` read off every other keeper's line. Refused by
    /// name where no other keeper is held, or where one has not yet carried
    /// the court and returned.
    pub fn of<'a>(origin: Origin, court: [u8; 32], lines: impl IntoIterator<Item = KeeperLine<'a>>) -> Result<Reach, WindowRefusal> {
        let mut through: Option<u64> = None;
        let mut witnesses = Vec::new();
        for KeeperLine { keeper, readings } in lines {
            if keeper == court {
                continue;
            }
            let Some((back, earlier)) = readings.split_last() else { return Err(WindowRefusal::Unwitnessed { keeper }) };
            // The reading among those with a later one that cites the court
            // highest: how far this keeper has carried it.
            let mut best: Option<(u64, &TickReading)> = None;
            for r in earlier {
                if let Some(c) = cite_of(r, &court) {
                    if best.is_none_or(|(b, _)| c > b) {
                        best = Some((c, r));
                    }
                }
            }
            let Some((carried, reach)) = best else { return Err(WindowRefusal::Unwitnessed { keeper }) };
            if through.is_none_or(|t| carried < t) {
                through = Some(carried);
            }
            witnesses.push(Witness { reach: reach.clone(), back: (*back).clone() });
        }
        match through {
            Some(through) => Ok(Reach { origin, court, through, witnesses }),
            None => Err(WindowRefusal::NoOtherKeeper),
        }
    }

    /// EVERY WITNESS CHECKED, as any verifier holding the readings checks
    /// them: both readings signed by one keeper other than the court at the
    /// origin, the reach reading citing the court at or above `through`, the
    /// return above the reach, and no keeper twice.
    pub fn verify(&self) -> Result<(), WindowRefusal> {
        if self.witnesses.is_empty() {
            return Err(WindowRefusal::NoOtherKeeper);
        }
        for (i, w) in self.witnesses.iter().enumerate() {
            let keeper = w.reach.keeper;
            if !w.reach.verifies(&self.origin) || !w.back.verifies(&self.origin) {
                return Err(WindowRefusal::Unsigned { keeper });
            }
            let carried = cite_of(&w.reach, &self.court);
            let twice = self.witnesses.iter().take(i).any(|v| v.reach.keeper == keeper);
            if keeper == self.court || w.back.keeper != keeper || twice || carried.is_none_or(|c| c < self.through) || w.back.tick.h <= w.reach.tick.h {
                return Err(WindowRefusal::Mismatched { keeper });
            }
        }
        Ok(())
    }

    /// The readings a sealed close names: per witness its reach reading and
    /// its return, each by keeper, hash and tick. `None` where a reading
    /// cannot be hashed.
    pub fn cited(&self) -> Option<Vec<Cited>> {
        let mut out = Vec::with_capacity(2 * self.witnesses.len());
        for w in &self.witnesses {
            for r in [&w.reach, &w.back] {
                out.push(Cited { keeper: r.keeper, reading: r.hash(&self.origin)?, h: r.tick.h });
            }
        }
        Some(out)
    }
}
