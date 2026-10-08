// 78.148 — THE MIND'S OWN CALL: with no counsel, a seated mind calls from its
// own repertoire — the functions its seat's listing names — with arguments of
// its own, learned from what the house answered and what each call moved.
//
//! THE REPERTOIRE IS PERCEIVED, never told: after the house answers its take
//! (and again when a call is refused NO SUCH FUNCTION), the arm reads the
//! domain's listing and hands the mind its own seat's functions, each with
//! its arguments' tags (`SeatMind::perceive_listing`).
//!
//! ITS OWN ARGUMENTS, NO SCRIPT. A function whose every argument is a Count
//! or a Flag is the mind's to explore. One call per beat, in increasing
//! order: its least arguments (every one zero, a flag false) first; then
//! each argument in turn upward from one, the others at their least. The
//! house's answer is the information: applied (or refused for any other
//! reason) the value is tried; refused BAD ARGUMENTS at `n`, that
//! argument's domain is bounded below `n` and `n` is never called again
//! (refused at the least, no value of the function applies). A flag's
//! domain is two values from the start. A call unanswered within its beat
//! is let go untried, and asked again.
//!
//! THE CHOICE, once every value of every bounded domain is tried: the read
//! calls (a listed function at arguments whose effect was read) compared by
//! the exact dominance `choice` reads — quantity by quantity, no sum — over
//! the records the mind's OWN VALUATION favours (`SeatMind::reach`, the one
//! place it is stated). One call dominates: it, said so. None does: of the
//! undominated calls, the one read fewest times is read again (the earliest
//! explored among equals), so a rate the world's drift may hide is
//! sharpened rather than guessed. OWED, named: a Count domain the house
//! never bounds is explored without end (no constant is put in its place);
//! a multivector record's carrier as a call's effect.

use super::*;
use crate::anastome::dialogue::seat::{put_args, take_args, SeatFn, REASON_BAD_ARGUMENTS, REASON_NO_SUCH_FUNCTION, TAG_COUNT, TAG_FLAG};

/// ONE ARGUMENT'S DOMAIN, as the house's answers taught it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Domain {
    /// A flag (values 0 and 1, written false and true), else a count.
    pub flag: bool,
    /// The least value not yet tried here, the others at their least.
    pub next: u64,
    /// The least value refused BAD ARGUMENTS: the domain lies below it. A
    /// flag's is two from the start.
    pub bound: Option<u64>,
}

impl Domain {
    fn open(&self) -> bool {
        match self.bound {
            Some(b) => self.next < b,
            None => true,
        }
    }
}

/// ONE FUNCTION OF THE MIND'S OWN SEAT, as the listing names it, and what
/// the mind has learned of its arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Own {
    pub id: u16,
    pub tags: Vec<u8>,
    /// Each argument's domain; none where an argument is neither a Count nor
    /// a Flag (the function is not the mind's to explore).
    pub domains: Option<Vec<Domain>>,
    /// Its least arguments were answered.
    pub least_tried: bool,
    /// Its least arguments were refused BAD ARGUMENTS: no value applies.
    pub closed: bool,
}

impl Own {
    /// A listed function, nothing yet learned of it.
    pub fn of(f: &SeatFn) -> Own {
        let domains = f
            .tags
            .iter()
            .map(|t| match *t {
                TAG_COUNT => Some(Domain { flag: false, next: 1, bound: None }),
                TAG_FLAG => Some(Domain { flag: true, next: 1, bound: Some(2) }),
                _ => None,
            })
            .collect::<Option<Vec<_>>>();
        Own { id: f.id, tags: f.tags.clone(), domains, least_tried: false, closed: false }
    }

    /// Its arguments with argument `i` at `v` (`at`), every other at its least.
    fn args_with(&self, at: Option<(usize, u64)>) -> Option<Vec<Arg>> {
        let domains = self.domains.as_ref()?;
        Some(
            domains
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let v = match at {
                        Some((j, v)) if j == i => v,
                        _ => 0,
                    };
                    match d.flag {
                        true => Arg::Flag(v == 1),
                        false => Arg::Count(v),
                    }
                })
                .collect(),
        )
    }

    /// THE NEXT ARGUMENTS NOT YET TRIED, in the module's order, with where
    /// they stand; none once every value in its bounded domains is tried, or
    /// where it is not the mind's to explore.
    pub fn untried(&self) -> Option<(Option<(usize, u64)>, Vec<Arg>)> {
        let domains = self.domains.as_ref()?;
        if self.closed {
            return None;
        }
        if !self.least_tried {
            return Some((None, self.args_with(None)?));
        }
        let (i, d) = domains.iter().enumerate().find(|(_, d)| d.open())?;
        Some((Some((i, d.next)), self.args_with(Some((i, d.next)))?))
    }

    /// Each argument's bound as learned.
    pub fn bounds(&self) -> Vec<Option<u64>> {
        match &self.domains {
            Some(ds) => ds.iter().map(|d| d.bound).collect(),
            None => Vec::new(),
        }
    }

    /// THE HOUSE ANSWERED `args`: where they are this function's next
    /// untried, they are tried — or, refused BAD ARGUMENTS, the domain is
    /// bounded there and the value never called again.
    fn answered(&mut self, args: &[Arg], applied: bool, reason: u16) {
        let Some((at, want)) = self.untried() else { return };
        if want.as_slice() != args {
            return;
        }
        let bad = !applied && reason == REASON_BAD_ARGUMENTS;
        match at {
            None => {
                self.least_tried = true;
                self.closed = bad;
            }
            Some((i, v)) => {
                let Some(d) = self.domains.as_mut().and_then(|ds| ds.get_mut(i)) else { return };
                match (bad, v.checked_add(1)) {
                    (false, Some(next)) => d.next = next,
                    // Refused, or the carrier's last value tried: the domain
                    // ends here.
                    (true, _) | (false, None) => d.bound = Some(v),
                }
            }
        }
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

/// An explored call's argument values: a count as itself, a flag as 0 or 1.
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
    /// mind's repertoire. A function listed again with the same argument
    /// tags keeps what was learned of its domains; one no longer listed is
    /// gone from it.
    pub fn perceive_listing(&mut self, fns: &[SeatFn]) {
        let held = std::mem::take(&mut self.repertoire);
        self.repertoire = fns
            .iter()
            .map(|f| match held.iter().find(|o| o.id == f.id && o.tags == f.tags) {
                Some(o) => o.clone(),
                None => Own::of(f),
            })
            .collect();
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
    /// listing is read again; any other answer teaches its domains.
    pub(crate) fn own_answered(&mut self, fn_id: u16, args: &[Arg], applied: bool, reason: u16) {
        if !applied && reason == REASON_NO_SUCH_FUNCTION {
            self.repertoire.retain(|o| o.id != fn_id);
            return;
        }
        if let Some(o) = self.repertoire.iter_mut().find(|o| o.id == fn_id) {
            o.answered(args, applied, reason);
        }
    }

    /// The read calls of listed explorable functions.
    fn read_calls(&self) -> Vec<ReadCall<'_>> {
        let mut calls: Vec<ReadCall<'_>> = self
            .own
            .scoped()
            .filter_map(|(k, e)| {
                let (fn_id, args) = of_call_key(&k)?;
                let place = self.repertoire.iter().position(|o| o.id == fn_id && o.domains.is_some())?;
                (e.read > 0).then_some(ReadCall { place, fn_id, values: arg_values(&args)?, e })
            })
            .collect();
        // The explored order: the listing's, then the values upward.
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
        for o in &self.repertoire {
            if let Some((at, args)) = o.untried() {
                return Decision::Call(Choice { fn_id: o.id, args, followed: false, why: Why::Exploring { at, bounds: o.bounds() } });
            }
        }
        let calls = self.read_calls();
        if calls.is_empty() {
            return Decision::NothingToCall;
        }
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
        let call_of = |c: &ReadCall<'_>, why: Why| match c.e.args.clone() {
            Some(args) => Decision::Call(Choice { fn_id: c.fn_id, args, followed: false, why }),
            None => Decision::NothingToCall,
        };
        if let [i] = dominant.as_slice() {
            if let Some(c) = calls.get(*i) {
                let over = calls.iter().enumerate().filter(|(j, _)| j != i).map(|(_, o)| (o.fn_id, o.values.clone())).collect();
                let named = favoured.iter().filter_map(|(k, d)| Some((Locator::of(k)?, *d, c.e.rate_at(k)))).collect();
                return call_of(c, Why::Reached { valuation: Valuation::Reach, values: c.values.clone(), over, along: named });
            }
        }
        // None dominates: the undominated, the one read fewest (the earliest
        // explored among equals).
        let front: Vec<usize> = (0..calls.len())
            .filter(|&i| match along.get(i) {
                Some(mine) => !others(i).any(|theirs| dominates(theirs, mine)),
                None => false,
            })
            .collect();
        #[expect(clippy::disallowed_methods, reason = "rule 4 — not a measure of worth: among calls no call dominates, the one this mind has read fewest times is read again (its own count of reads, named on the typed line), the earliest explored among equals")]
        let least = front.iter().filter_map(|i| calls.get(*i)).min_by_key(|c| c.e.read);
        match least {
            Some(c) => call_of(c, Why::Unreached { valuation: Valuation::Reach, values: c.values.clone(), front: front.len(), read: c.e.read }),
            None => Decision::NothingToCall,
        }
    }
}
