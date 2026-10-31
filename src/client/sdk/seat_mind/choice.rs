// 78.146 — THE MIND CHOOSES ITS OWN CALL, from what its calls were seen to do.
//
//! WHAT A CALL DID is read, never scored: each applied call's effect is the
//! slide of every quantity between the beat it was made at and the first
//! percept after the house applied it, composed per function on each
//! quantity's flow (`Effect`), so a function's RATE on a quantity is its
//! composed slide over the times it was read — exact.
//!
//! WHAT MATTERS is the house's stated worth, read through experience: a
//! counsel whose worth is above zero says the moves its function makes are
//! worth making, one below zero that they are worth undoing; so where the
//! mind has read a counselled function move a quantity, the counsel FAVOURS
//! that quantity's moving the way `sign(worth) · sign(rate)` says. Two
//! counsels favouring one quantity opposite ways leave it contested and
//! favoured neither way.
//!
//! THE CHOICE, stated (a comparison a mind names, Concordance 1.10):
//! - no call's effect read yet: the house's counsel, followed, and said so;
//! - nothing favoured: the counsel, followed;
//! - one read function DOMINATES — on every favoured quantity it moves at
//!   least as far the favoured way as every other read function, further on
//!   one, and the favoured way on one: that function, counselled or not;
//! - every read function moved the favoured quantities against or not at
//!   all, and the house counsels one this mind has not read: that one, tried;
//! - otherwise the counsel, followed — no read function dominates.
//!
//! WITH NO COUNSEL (78.148) the call is the mind's own (`own`): the dominant
//! read call under its own valuation, REACH — or none, and the mind passes.
//! No argument is enumerated (MU-CLIENT §7.1).
//!
//! No sum across quantities is formed: dominance is read quantity by
//! quantity, so no hidden scalar weighs one record against another.

use super::*;

/// WHICH WAY A QUANTITY IS FAVOURED to move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Raise,
    Lower,
}

impl Direction {
    pub(super) fn of(q: &Q) -> Option<Direction> {
        match () {
            _ if q.is_positive() => Some(Direction::Raise),
            _ if q.is_negative() => Some(Direction::Lower),
            _ => None,
        }
    }

    fn flipped(self) -> Direction {
        match self {
            Direction::Raise => Direction::Lower,
            Direction::Lower => Direction::Raise,
        }
    }

    /// A rate read the favoured way: itself to raise, negated to lower.
    pub(super) fn along(self, rate: &Q) -> Q {
        match self {
            Direction::Raise => rate.clone(),
            Direction::Lower => -rate.clone(),
        }
    }
}

/// WHAT ONE FUNCTION'S APPLIED CALLS DID TO ONE QUANTITY: the calls' slides
/// composed on its flow, and how many were read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub composed: FlowPoint,
    pub read: u64,
}

impl Effect {
    /// The function's rate on the quantity: the composed slide per read call.
    pub fn rate(&self) -> Option<Q> {
        Some(self.composed.parameter()? / Q::from_integer(BigInt::from(self.read)))
    }
}

/// WHAT ONE FUNCTION'S CALLS WERE SEEN TO DO.
#[derive(Clone, Debug, Default)]
pub struct Experience {
    /// The arguments it is called with: the house's latest counsel for it,
    /// or this mind's own last call of it.
    pub args: Option<Vec<Arg>>,
    pub applied: u64,
    /// Refusals, by the house's reason.
    pub refused: Vec<(u16, u64)>,
    /// Applied calls whose effect was read.
    pub read: u64,
    pub(crate) effects: Keyed<Effect>,
}

impl Experience {
    /// One applied call's effect: every quantity's slide since it was made.
    pub(crate) fn learn(&mut self, effects: Vec<(Vec<u8>, FlowPoint)>) {
        self.read += 1;
        for (k, slide) in effects {
            let next = match self.effects.get(&k) {
                Some(e) => e.composed.then(&slide).map(|composed| Effect { composed, read: e.read + 1 }),
                None => Some(Effect { composed: slide, read: 1 }),
            };
            if let Some(e) = next {
                self.effects.insert(&k, e);
            }
        }
    }

    /// Its effect on one quantity.
    pub fn effect(&self, at: &Locator) -> Option<&Effect> {
        self.effects.get(&at.bytes()?)
    }

    /// Its rate on the quantity at `k`; zero where it was read and the
    /// quantity was not among those that stood when it was made.
    pub(super) fn rate_at(&self, k: &[u8]) -> Q {
        match self.effects.get(k).and_then(Effect::rate) {
            Some(r) => r,
            None => Q::zero(),
        }
    }

    /// Every call of it was refused: tried, and never applied.
    fn refused_only(&self) -> bool {
        self.applied == 0 && !self.refused.is_empty()
    }
}

/// WHY THE MIND CALLS WHAT IT CALLS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Why {
    /// No call's effect is read yet: the house's counsel.
    NoExperience,
    /// The counsel's worth favours no quantity a read call moved.
    NothingFavoured,
    /// Its read rates along the favoured quantities are at least every other
    /// read function's, above one's, and the favoured way on one.
    Dominates { over: Vec<u16>, along: Vec<(Locator, Direction, Q)> },
    /// Every read function moved the favoured quantities against or not at
    /// all; this one is counselled and not yet read.
    Untried { against: Vec<u16> },
    /// No read function dominates: the counsel.
    NoneDominates,
    /// 78.148 — no counsel: under the mind's own valuation, this read call's
    /// rates along the favoured records are at least every other read call's
    /// (`over`, each a function and its argument values), above one's, and
    /// the favoured way on one.
    Reached { valuation: Valuation, values: Vec<u64>, over: Vec<(u16, Vec<u64>)>, along: Vec<(Locator, Direction, Q)> },
}

/// 78.148 — THE VALUATION a choice is read under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valuation {
    /// No worth declared (no counsel): REACH — what the mind's own calls
    /// were seen to move, favoured the one way they moved it
    /// (`SeatMind::reach`).
    Reach,
}

impl std::fmt::Display for Valuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Valuation::Reach => write!(
                f,
                "valuation REACH (no worth declared: each record the mind's own calls moved, favoured the one way they moved it; a record moved both ways contested)"
            ),
        }
    }
}

impl std::fmt::Display for Why {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Why::NoExperience => write!(f, "no call's outcome read yet — the house's counsel"),
            Why::NothingFavoured => write!(f, "the counsel's worth favours no record a read call moved — the house's counsel"),
            Why::Dominates { over, along } => {
                write!(f, "its read outcomes move")?;
                for (at, d, rate) in along {
                    write!(f, " {at} {d:?} at {rate}/call")?;
                }
                write!(f, ", at least as far the favoured way as fn {over:?} on every one")
            }
            Why::Untried { against } => write!(f, "fn {against:?} moved the favoured records against or not at all — a counselled call not yet read, tried"),
            Why::NoneDominates => write!(f, "no read call dominates on every favoured record — the house's counsel"),
            Why::Reached { valuation, values, over, along } => {
                write!(f, "{valuation}: at {values:?} its read outcomes move")?;
                for (at, d, rate) in along {
                    write!(f, " {at} {d:?} at {rate}/call")?;
                }
                write!(f, ", at least as far the favoured way as calls {over:?} on every one")
            }
        }
    }
}

/// A CALL CHOSEN: the function, its arguments, whether it is the house's
/// first counsel, and why.
#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    pub fn_id: u16,
    pub args: Vec<Arg>,
    pub followed: bool,
    pub why: Why,
}

/// WHAT THE MIND DOES AT A BEAT.
#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    Call(Choice),
    /// Its last call is not answered and read yet.
    Awaiting { fn_id: u16 },
    /// No counsel, and no read call dominates under its own valuation: the
    /// mind passes (MU-CLIENT §7.1).
    NothingToCall,
}

impl std::fmt::Display for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decision::Call(c) => write!(f, "call fn {} followed-counsel {}: {}", c.fn_id, c.followed, c.why),
            Decision::Awaiting { fn_id } => write!(f, "no call: fn {fn_id} awaits its outcome and reading"),
            Decision::NothingToCall => write!(f, "no call: no counsel, and no read call dominates — the mind passes"),
        }
    }
}

impl SeatMind {
    /// THE QUANTITIES THE COUNSEL FAVOURS, each with its direction: the sign
    /// of a counselled function's worth times the sign of its read rate; a
    /// quantity two counsels favour opposite ways is contested and left out.
    pub fn favoured(&self) -> Vec<(Vec<u8>, Direction)> {
        let mut favoured: Keyed<Direction> = Keyed::new();
        let mut contested: Keyed<()> = Keyed::new();
        for c in &self.counsel {
            let Some(worth) = quantity(&c.worth).as_ref().and_then(Direction::of) else { continue };
            let Some(e) = self.experience(c.fn_id) else { continue };
            for (k, effect) in e.effects.scoped() {
                let Some(moved) = effect.rate().as_ref().and_then(Direction::of) else { continue };
                let d = match worth {
                    Direction::Raise => moved,
                    Direction::Lower => moved.flipped(),
                };
                match favoured.get(&k) {
                    Some(held) if *held != d => {
                        contested.insert(&k, ());
                    }
                    Some(_) => {}
                    None => {
                        favoured.insert(&k, d);
                    }
                }
            }
        }
        favoured.scoped().filter(|(k, _)| !contested.contains(k)).map(|(k, d)| (k, *d)).collect()
    }

    /// THE MIND'S CALL AT THIS BEAT (the module's rule, stated).
    pub fn decide(&self) -> Decision {
        if let Some(fn_id) = self.awaiting() {
            return Decision::Awaiting { fn_id };
        }
        // 78.148 — no counsel: the mind's own call, by its own valuation.
        if self.counsel.is_empty() {
            return self.own_call();
        }
        // The house's first counsel, in the house's own record order — the
        // order the wire carries is the house's statement (78.145).
        let counsel = match self.counsel.as_slice() {
            [c, ..] => Some(c),
            [] => None,
        };
        let follow = |why: Why| match counsel {
            Some(c) => Decision::Call(Choice { fn_id: c.fn_id, args: c.args.clone(), followed: true, why }),
            None => Decision::NothingToCall,
        };
        // The read functions this mind can call: an effect read, arguments held.
        let read: Vec<(u16, &Experience)> = self
            .experience
            .scoped()
            .filter_map(|(k, e)| {
                let id = u16::from_be_bytes(k.as_slice().try_into().ok()?);
                (e.read > 0 && e.args.is_some()).then_some((id, e))
            })
            .collect();
        if read.is_empty() {
            return follow(Why::NoExperience);
        }
        let favoured = self.favoured();
        if favoured.is_empty() {
            return follow(Why::NothingFavoured);
        }
        // Each read function's rates, the favoured way, quantity by quantity.
        let along: Vec<(u16, &Experience, Vec<Q>)> =
            read.iter().map(|(id, e)| (*id, *e, favoured.iter().map(|(k, d)| d.along(&e.rate_at(k))).collect())).collect();
        let dominates = |a: &[Q], b: &[Q]| a.iter().zip(b).all(|(x, y)| x >= y) && a.iter().zip(b).any(|(x, y)| x > y);
        let dominant: Vec<&(u16, &Experience, Vec<Q>)> = along
            .iter()
            .filter(|(id, _, rates)| {
                rates.iter().any(|x| x.is_positive()) && along.iter().filter(|(other, _, _)| other != id).all(|(_, _, other_rates)| dominates(rates, other_rates))
            })
            .collect();
        if let [(id, e, _)] = dominant.as_slice() {
            let args = match self.counsel.iter().find(|c| c.fn_id == *id) {
                Some(c) => c.args.clone(),
                None => e.args.clone().into_iter().flatten().collect(),
            };
            let over = along.iter().filter(|(other, _, _)| other != id).map(|(other, _, _)| *other).collect();
            let named = favoured.iter().filter_map(|(k, d)| Some((Locator::of(k)?, *d, e.rate_at(k)))).collect();
            return Decision::Call(Choice {
                fn_id: *id,
                args,
                followed: counsel.is_some_and(|c| c.fn_id == *id),
                why: Why::Dominates { over, along: named },
            });
        }
        let none_forward = along.iter().all(|(_, _, rates)| rates.iter().all(|x| !x.is_positive()));
        if none_forward {
            let untried = self.counsel.iter().find(|c| match self.experience(c.fn_id) {
                Some(e) => e.read == 0 && !e.refused_only(),
                None => true,
            });
            if let Some(c) = untried {
                return Decision::Call(Choice {
                    fn_id: c.fn_id,
                    args: c.args.clone(),
                    followed: counsel.is_some_and(|first| first.fn_id == c.fn_id),
                    why: Why::Untried { against: along.iter().map(|(id, _, _)| *id).collect() },
                });
            }
        }
        follow(Why::NoneDominates)
    }
}
