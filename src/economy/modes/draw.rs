// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! #169 (ruled 2026-11-06) — A PER-LANE AMOUNT DRAWN FROM A HOLDING'S MODES,
//! THE FOUNDING MODES, AND THE READINGS PER LANE.
//!
//! A DRAW of a per-lane amount (a price, a toll, gas, a cover, a slash's
//! cut, a lease's hold) takes it from the modes on that lane, in proportion
//! to their amounts, exactly in ℚ: mode `m` of lane `l` gives
//! `a_m · asked_l / held_l`. No choice is hidden in the payer, nothing is
//! rounded, and the parts drawn sum to the amount asked. A lane is never
//! paid from another lane's modes: where the lane holds less than is asked,
//! the draw is refused by name ([`LaneShort`]) and nothing moves. The modes
//! drawn travel with the transfer.
//!
//! A LANE'S READING is the amount its modes hold, read when asked and never
//! stored ([`Modes::reading`]); the four readings are never summed into
//! one another.

use super::{Mode, Modes, Q, LANES};
use crate::economy::Mu;
use num_traits::{Signed, Zero};

/// WHAT A PER-LANE DRAW LACKED: the lane, what its modes hold, and what was
/// asked of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LaneShort {
    pub lane: u8,
    pub held: Q,
    pub asked: Q,
}

impl Modes {
    /// THE FOUNDING MODES OF `lanes`: each lane's amount at that lane's
    /// founding mode, the lane's identity motion stated explicitly. A lane
    /// at zero holds no mode.
    pub fn founding(lanes: &Mu) -> Modes {
        let mut out = Modes::new();
        for lane in LANES {
            out.add_mode(Mode::Founding { lane }, lanes.lane(usize::from(lane)).clone());
        }
        out
    }

    /// THE MODES A MINT CARRIES (#169): the modes of the motion its proof
    /// cites, `cited`; where no motion it cites reads a mode, the measured
    /// amount at each lane's founding mode — never empty modes for a
    /// measured amount.
    pub fn cited_or_founding(cited: &Modes, measured: &Mu) -> Modes {
        match cited.is_empty() {
            true => Modes::founding(measured),
            false => cited.clone(),
        }
    }

    /// Whether every mode held is a founding mode.
    pub fn is_founding(&self) -> bool {
        self.modes().all(|(m, _)| matches!(m, Mode::Founding { .. }))
    }

    /// WHAT LANE `lane`'S MODES HOLD, read when asked.
    pub fn lane_amount(&self, lane: u8) -> Q {
        let mut total = Q::zero();
        for (_, a) in self.modes().filter(|(m, _)| m.lane() == lane) {
            total += a;
        }
        total
    }

    /// THE READING PER LANE: on each lane what its modes hold, in lane
    /// order. A reading, never the holding, and never stored.
    pub fn reading(&self) -> Mu {
        let lanes = LANES.map(|l| self.lane_amount(l));
        // Every amount held is positive, so no lane reads negative: the
        // refusal's arm cannot fire, and it reads as nothing held.
        match Mu::of(lanes) {
            Ok(m) => m,
            Err(_) => Mu::zero(),
        }
    }

    /// THE MODES OF `lane` SCALED BY `r`, every other lane's untouched.
    fn lane_scaled(&self, lane: u8, r: &Q) -> Modes {
        let mut out = Modes::new();
        for (m, a) in self.modes() {
            let amount = match m.lane() == lane {
                true => a * r,
                false => a.clone(),
            };
            out.add_mode(m.clone(), amount);
        }
        out
    }

    /// THE PART OF THESE MODES A PER-LANE AMOUNT NAMES, without moving
    /// anything: on each lane `asked_l` taken in proportion to the lane's
    /// modes. Refused [`LaneShort`] on the first lane, in lane order, that
    /// holds less than is asked of it.
    pub fn part_at(&self, asked: &Mu) -> Result<Modes, Box<LaneShort>> {
        let mut taken = Modes::new();
        for lane in LANES {
            let want = asked.lane(usize::from(lane));
            if want.is_zero() {
                continue;
            }
            let held = self.lane_amount(lane);
            if &held < want {
                return Err(Box::new(LaneShort { lane, held, asked: want.clone() }));
            }
            let r = want / &held;
            for (m, a) in self.modes().filter(|(m, _)| m.lane() == lane) {
                taken.add_mode(m.clone(), a * &r);
            }
        }
        Ok(taken)
    }

    /// DRAW A PER-LANE AMOUNT, whole or refused: the part [`Modes::part_at`]
    /// names, taken out of these modes and answered, the modes that travel
    /// with the transfer. A refused draw moves nothing.
    pub fn draw(&mut self, asked: &Mu) -> Result<Modes, Box<LaneShort>> {
        let taken = self.part_at(asked)?;
        // The part was read from these modes, so every mode of it is held in
        // at least its amount.
        if let Err(short) = self.take(&taken) {
            return Err(Box::new(LaneShort { lane: short.mode.lane(), held: short.held, asked: short.asked }));
        }
        Ok(taken)
    }

    /// THESE MODES CAPPED BY A PER-LANE AMOUNT: on each lane holding more
    /// than `cap_l`, the lane's modes scaled to `cap_l` in proportion; every
    /// other lane whole.
    pub fn capped(&self, cap: &Mu) -> Modes {
        let mut out = self.clone();
        for lane in LANES {
            let held = out.lane_amount(lane);
            let c = cap.lane(usize::from(lane));
            if &held > c {
                out = out.lane_scaled(lane, &(c / &held));
            }
        }
        out
    }

    /// THESE MODES DIVIDED AT A PER-LANE AMOUNT: the part taken (on each
    /// lane the lesser of the amount and what the lane holds, in proportion
    /// to its modes) and what is kept. The two sum to these modes exactly.
    pub fn split_at(&self, asked: &Mu) -> (Modes, Modes) {
        let taken = self.capped(asked);
        let mut kept = self.clone();
        // `taken` is these modes capped, so it is held whole.
        let _ = kept.take(&taken);
        (taken, kept)
    }

    /// EACH LANE'S MODES SCALED BY ITS OWN FACTOR, `r_l` on lane `l` (the
    /// POWV share, MU-ECONOMY §5.1: the lane's factor scales each mode on
    /// it). A lane whose factor is not positive holds nothing after.
    pub fn scaled_lanes(&self, r: &[Q; 4]) -> Modes {
        let mut out = Modes::new();
        for (m, a) in self.modes() {
            if let Some(f) = r.get(usize::from(m.lane())) {
                out.add_mode(m.clone(), a * f);
            }
        }
        out
    }

    /// `self · r` on every mode, exactly; nothing for `r ≤ 0`.
    pub fn scaled(&self, r: &Q) -> Modes {
        let mut out = Modes::new();
        if !r.is_positive() {
            return out;
        }
        for (m, a) in self.modes() {
            out.add_mode(m.clone(), a * r);
        }
        out
    }

    /// ONE OF `n` EQUAL PARTS, exactly, so `n` parts are the whole. `None`
    /// for no parts.
    pub fn part(&self, n: usize) -> Option<Modes> {
        match n {
            0 => None,
            n => Some(self.scaled(&(Q::from_integer(1.into()) / Q::from_integer(n.into())))),
        }
    }

    /// `self + other`, a new holding.
    pub fn with(&self, other: &Modes) -> Modes {
        let mut out = self.clone();
        out.add(other);
        out
    }

    /// THE HOLDING'S WRITING AS BYTES (`Modes::put`). A held mode always
    /// writes: it was admitted by its coordinates, which its writing shares.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        if self.put(&mut out).is_none() {
            debug_assert!(false, "a held mode has a writing");
        }
        out
    }

    /// One holding read from the whole of `b`, or refused.
    pub fn from_bytes(b: &[u8]) -> Option<Modes> {
        let mut p = 0usize;
        let m = Modes::take_from(b, &mut p)?;
        (p == b.len()).then_some(m)
    }

    /// THE SUM OF HOLDINGS, mode by mode.
    pub fn sum<'a>(it: impl IntoIterator<Item = &'a Modes>) -> Modes {
        let mut total = Modes::new();
        for m in it {
            total.add(m);
        }
        total
    }
}
