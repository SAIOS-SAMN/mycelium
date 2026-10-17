// 78.148 — THE MIND'S OWN CALL: with no counsel, a seated mind calls from its
// own repertoire — the functions its seat's listing names — at arguments it
// has READ, learned from what the house answered and what each call moved.
//
//! THE REPERTOIRE IS PERCEIVED, never told: after the house answers its take
//! (and again when a call is refused NO SUCH FUNCTION), the arm reads the
//! domain's listing and hands the mind its own seat's functions, each with
//! its arguments' tags (`SeatMind::perceive_listing`).
//!
//! NO ARGUMENT IS ENUMERATED (the operator, 2026-10-17: "every component is a
//! combinatorial algorithm that decomposes according to our pivot machinery
//! … if you end up with search then we have drifted into brute force"). The
//! mind calls a function only at arguments whose call it has read — a call
//! counselled or made before, whose effect on the records it perceived — and
//! never walks a domain upward until the house refuses a value.
//!
//! THE CHOICE: the read calls (a listed function at arguments whose effect
//! was read) compared by the exact dominance `choice` reads — quantity by
//! quantity, no sum — over the records the mind's OWN VALUATION favours
//! (`SeatMind::reach`, the one place it is stated). One call dominates: it,
//! said so. None does, or none is read: the mind PASSES — no call, no count
//! of reads to break the tie (MU-CLIENT §7.1: rest is an equilibrium, never a
//! stall to break). OWED, named: a multivector record's carrier as a call's
//! effect.

use super::*;
use crate::anastome::dialogue::seat::{put_args, take_args, SeatFn, REASON_NO_SUCH_FUNCTION};

/// ONE FUNCTION OF THE MIND'S OWN SEAT, as the listing names it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Own {
    pub id: u16,
    pub tags: Vec<u8>,
}

impl Own {
    /// A listed function.
    pub fn of(f: &SeatFn) -> Own {
        Own { id: f.id, tags: f.tags.clone() }
    }
}

/// A CALL'S COORDINATES: its function and its arguments, written inline —
/// `[fn u16 BE][ARGS]` — so what one call did is held where the call is.
pub fn call_key(fn_id: u16, args: &[Arg]) -> Option<Vec<u8>> {
    let mut k = fn_id.to_be_bytes().to_vec();
    put_args(&mut k, args)?;
    Some(k)
}

/// The call a key spells, read whole or none.
pub fn of_call_key(k: &[u8]) -> Option<(u16, Vec<Arg>)> {
    let id = u16::from_be_bytes(k.get(0..2)?.try_into().ok()?);
    let mut p = 2usize;
    let args = take_args(k, &mut p)?;
    (p == k.len()).then_some((id, args))
}

/// A read call's argument values: a count as itself, a flag as 0 or 1.
pub fn arg_values(args: &[Arg]) -> Option<Vec<u64>> {
    args.iter()
        .map(|a| match a {
            Arg::Count(n) => Some(*n),
            Arg::Flag(f) => Some(u64::from(*f)),
            _ => None,
        })
        .collect()
}

/// ONE READ CALL OF THE MIND'S OWN: its function's place in the listing,
/// the function, its argument values, and what it was seen to do.
struct ReadCall<'a> {
    place: usize,
    fn_id: u16,
    values: Vec<u64>,
    e: &'a Experience,
}

impl SeatMind {
    /// 78.148 — THE LISTING PERCEIVED: its own seat's functions are the
    /// mind's repertoire; one no longer listed is gone from it.
    pub fn perceive_listing(&mut self, fns: &[SeatFn]) {
        self.repertoire = fns.iter().map(Own::of).collect();
    }

    /// The repertoire, in the listing's order.
    pub fn repertoire(&self) -> &[Own] {
        &self.repertoire
    }

    /// What one call — a function at its arguments — was seen to do.
    pub fn call_experience(&self, fn_id: u16, args: &[Arg]) -> Option<&Experience> {
        self.own.get(&call_key(fn_id, args)?)
    }

    /// One call's experience, written at its coordinates (begun where the
    /// call was never seen); none where its arguments cannot be written.
    pub(crate) fn with_call_experience(&mut self, fn_id: u16, args: &[Arg], f: impl FnOnce(&mut Experience)) {
        let Some(key) = call_key(fn_id, args) else { return };
        let mut e = match self.own.remove(&key) {
            Some(e) => e,
            None => Experience { args: Some(args.to_vec()), ..Experience::default() },
        };
        f(&mut e);
        self.own.insert(key, e);
    }

    /// The house's answer to one of the mind's calls, read into its
    /// repertoire: NO SUCH FUNCTION takes the function out of it until the
    /// listing is read again.
    pub(crate) fn own_answered(&mut self, fn_id: u16, applied: bool, reason: u16) {
        if !applied && reason == REASON_NO_SUCH_FUNCTION {
            self.repertoire.retain(|o| o.id != fn_id);
        }
    }

    /// The read calls of listed functions whose arguments are counts and
    /// flags.
    fn read_calls(&self) -> Vec<ReadCall<'_>> {
        let mut calls: Vec<ReadCall<'_>> = self
            .own
            .scoped()
            .filter_map(|(k, e)| {
                let (fn_id, args) = of_call_key(&k)?;
                let place = self.repertoire.iter().position(|o| o.id == fn_id)?;
                (e.read > 0).then_some(ReadCall { place, fn_id, values: arg_values(&args)?, e })
            })
            .collect();
        // The listing's order, then the values upward.
        #[expect(clippy::disallowed_methods, reason = "rule 4 — a canonical key: the house's listing order, then the argument values upward; nothing measured ranks the calls")]
        calls.sort_by(|a, b| (a.place, &a.values).cmp(&(b.place, &b.values)));
        calls
    }

    /// THE MIND'S OWN VALUATION — REACH — where no worth is declared (no
    /// counsel). A mind that is told nothing of what matters values what it
    /// can move: every record quantity one of its own read calls moved is
    /// favoured the way its calls moved it; one its calls moved both ways
    /// is contested and favoured neither way (its experience does not say
    /// which way matters). The choice under it is dominance read quantity
    /// by quantity — the call that moves every favoured record at least as
    /// far as every other call, further on one — never a summed score
    /// (rule 4): no record is weighed against another.
    pub fn reach(&self) -> Vec<(Vec<u8>, Direction)> {
        let mut favoured: Keyed<Direction> = Keyed::new();
        let mut contested: Keyed<()> = Keyed::new();
        for c in self.read_calls() {
            for (k, effect) in c.e.effects.scoped() {
                let Some(moved) = effect.rate().as_ref().and_then(Direction::of) else { continue };
                match favoured.get(&k) {
                    Some(held) if *held != moved => {
                        contested.insert(&k, ());
                    }
                    Some(_) => {}
                    None => {
                        favoured.insert(&k, moved);
                    }
                }
            }
        }
        favoured.scoped().filter(|(k, _)| !contested.contains(k)).map(|(k, d)| (k, *d)).collect()
    }

    /// THE MIND'S OWN CALL, no counsel given (the module's rule).
    pub(crate) fn own_call(&self) -> Decision {
        let calls = self.read_calls();
        let favoured = self.reach();
        let along: Vec<Vec<Q>> = calls.iter().map(|c| favoured.iter().map(|(k, d)| d.along(&c.e.rate_at(k))).collect()).collect();
        let dominates = |a: &[Q], b: &[Q]| a.iter().zip(b).all(|(x, y)| x >= y) && a.iter().zip(b).any(|(x, y)| x > y);
        let others = |i: usize| along.iter().enumerate().filter(move |(j, _)| *j != i).map(|(_, theirs)| theirs);
        let dominant: Vec<usize> = (0..calls.len())
            .filter(|&i| match along.get(i) {
                Some(mine) => mine.iter().any(|x| x.is_positive()) && others(i).all(|theirs| dominates(mine, theirs)),
                None => false,
            })
            .collect();
        let [i] = dominant.as_slice() else { return Decision::NothingToCall };
        let Some(c) = calls.get(*i) else { return Decision::NothingToCall };
        let Some(args) = c.e.args.clone() else { return Decision::NothingToCall };
        let over = calls.iter().enumerate().filter(|(j, _)| j != i).map(|(_, o)| (o.fn_id, o.values.clone())).collect();
        let named = favoured.iter().filter_map(|(k, d)| Some((Locator::of(k)?, *d, c.e.rate_at(k)))).collect();
        Decision::Call(Choice { fn_id: c.fn_id, args, followed: false, why: Why::Reached { valuation: Valuation::Reach, values: c.values.clone(), over, along: named } })
    }
}
