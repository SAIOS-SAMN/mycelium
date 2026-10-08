// 78.146 — THE MIND AT ITS SEAT: what a seated mind perceives of its game and
// remembers of it, read off the house's typed exact records, and (in
// `choice`) the call it chooses from what its own calls were seen to do.
//
//! A PERCEPT IS A SERIES, NEVER A PICTURE. Each record of a percept is
//! located by its own coordinates — its kind, the keys it carries (the
//! house's own names for what it is about), its occurrence among the records
//! sharing both, and the argument's position — and every non-key argument is
//! one SERIES over the house's percept sequence (the seat's beats) and its
//! local time. Nothing is painted into sites it is not.
//!
//! A QUANTITY'S CHANGE IS A MOTION OF ITS COORDINATE. A Q, a BigQ or a Count
//! is a point of its own line ℚ¹, and the change between two beats is held as
//! the motions of that line that carry one value to the other, exact:
//! - THE SLIDE by `Δ = to − from`, the translator's point on its null flow
//!   (`translator_at`, read on the plane `translator_at([1])` itself, so
//!   `t = Δ` and two slides compose by addition);
//! - THE SCALE by `ρ = to / from` where both stand on one side of zero, the
//!   dilator's point on its boost flow (`dilator`, read on `dilator(2)`'s
//!   plane, `t = (ρ − 1)/(ρ + 1)`, so two scales compose by product).
//!
//! A multivector argument is held as the geometric object it is — a full
//! Cl(4,1) versor, odd ones (reflections) included; a flag as itself. Two
//! values are one where they write one run of bytes (`Arg ==`), so a
//! multivector is compared coefficient for coefficient and never only
//! projectively.
//!
//! 78.148 — A VERSOR'S CHANGE IS THE VERSOR THAT CARRIES IT (the operator's
//! field law): between consecutive beats the change of a multivector series
//! is `c = v₊ · ṽ / ⟨v ṽ⟩₀`, the exact carrier with `c · v = v₊` — divided
//! by the earlier value's weight (`weight_of`, read), so it is normalised
//! exactly — or REFUSED where either value is no versor or the weight is
//! zero (`carrier`). It is never flattened to numbers.
//!
//! AN ORBIT IS A RETURN, READ. A series returns with period `p` where its
//! last `2p` changes repeat with period `p` and the slides of one period
//! compose to the flow's identity (`FlowPoint::returned`); a multivector's
//! where its last `2p` CARRIERS repeat with period `p` (a steady carrier is
//! the orbit of period one; a carrier that is the scalar one, a value that
//! stands still, is fixed); a flag's — or a multivector series whose
//! carrier was refused — where its last `2p` values repeat; period 1 is a
//! fixed value. A record's period is the lcm of its arguments' (time is distance
//! along an orbit, 78.43), and the period's span in the house's local time is
//! read off the two beats, never assumed.
//!
//! BOUNDED AS THE RELAY IS (a sink keeps its bound): a series holds what one
//! frame could carry of its values as they write (`frame_budget`), the oldest
//! dropped and counted past it. OWED, named: a bound on how many series a
//! seat holds (the house's record locators, which a game may churn), and the
//! hand-over of a recognised orbit to the contemplation as a form.

mod choice;
mod own;
pub mod task;
pub use choice::*;
pub use own::*;

use crate::anastome::dialogue::seat::{Arg, Outcome, Percept, Record};
use crate::network::nerve::Keyed;
use crate::transducer::cga::{dilator, is_versor, translator_at, weight_of, FlowPoint, Mv};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};
use std::collections::VecDeque;

/// A QUANTITY'S EXACT VALUE: a Q, a BigQ or a Count; none for a key, a flag,
/// a multivector, or a denominator of zero.
pub fn quantity(a: &Arg) -> Option<Q> {
    match a {
        Arg::Q { num, den } => (*den != 0).then(|| Q::new(BigInt::from(*num), BigInt::from(*den))),
        Arg::BigQ { num, den } => (!den.is_zero()).then(|| Q::new(num.clone(), BigInt::from(den.clone()))),
        Arg::Count(n) => Some(Q::from_integer(BigInt::from(*n))),
        // A key, a flag, a multivector — and any argument a later wire tag
        // adds — is no quantity.
        _ => None,
    }
}

/// THE SLIDE'S PLANE on a quantity's line: the translator by one, read off
/// the algebra (`translator_at`), so a slide by `Δ` stands at `t = Δ`.
pub fn slide_plane() -> Mv {
    translator_at(&[Q::one()]).grade(2)
}

/// THE SCALE'S PLANE: the dilator by two, its orientation settled against
/// the embedding (`dilator`), so a scale by `ρ` stands at `(ρ − 1)/(ρ + 1)`.
pub fn scale_plane() -> Mv {
    dilator(&Q::from_integer(BigInt::from(2))).grade(2)
}

/// ONE OBSERVED CHANGE OF A QUANTITY between two beats, as the motions of
/// its line that carry one value onto the other.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// The slide by the difference, on its null flow.
    pub slide: FlowPoint,
    /// The scale by the ratio, on its boost flow, where both values stand on
    /// one side of zero; none across or at zero (no dilation carries them).
    pub scale: Option<FlowPoint>,
}

impl Change {
    /// The change from `from` to `to`; `None` only where the algebra reads no
    /// flow point (a plane that is not simple — never on these planes).
    pub fn between(from: &Q, to: &Q) -> Option<Change> {
        let d = to - from;
        let slide = match d.is_zero() {
            true => FlowPoint::origin(&slide_plane())?,
            false => FlowPoint::of_motion(&translator_at(std::slice::from_ref(&d)))?.against(&slide_plane())?,
        };
        let scale = match from.is_zero() || to.is_zero() || from.is_negative() != to.is_negative() {
            true => None,
            false => {
                let rho = to / from;
                match rho.is_one() {
                    true => FlowPoint::origin(&scale_plane()),
                    false => FlowPoint::of_motion(&dilator(&rho)).and_then(|p| p.against(&scale_plane())),
                }
            }
        };
        Some(Change { slide, scale })
    }

    /// The exact difference the slide carries.
    pub fn delta(&self) -> Option<Q> {
        self.slide.parameter()
    }
}

/// 78.148 — THE VERSOR CARRYING `from` ONTO `to`: `to · from̃ / ⟨from from̃⟩₀`,
/// so that `carrier · from = to` exactly (checked, coefficient for
/// coefficient). Refused — `None` — where either is no versor (even or odd),
/// where `from`'s weight is zero, or where the product does not carry it
/// exactly: a refusal, never a zero change (rule 11).
pub fn carrier(from: &Mv, to: &Mv) -> Option<Mv> {
    let w = weight_of(from)?;
    if w.is_zero() || !is_versor(to) {
        return None;
    }
    let c = to.geo(&from.reverse()).scale(&(Q::one() / w));
    c.geo(from).sub(to).is_zero().then_some(c)
}

/// Two carriers are one where they agree coefficient for coefficient.
fn same_carrier(a: &Mv, b: &Mv) -> bool {
    a.sub(b).is_zero()
}

/// WHERE A SERIES STANDS: the record's coordinates and the argument's
/// position. Written `[kind u16 BE][keys u8][key 32…][occurrence u32
/// BE][position u8]` — self-delimiting, so a record's series are one fiber
/// under its prefix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locator {
    pub kind: u16,
    pub keys: Vec<[u8; 32]>,
    pub occurrence: u32,
    pub position: u8,
}

impl Locator {
    /// The record's prefix: every series of one record hangs under it.
    pub fn record_prefix(kind: u16, keys: &[[u8; 32]], occurrence: u32) -> Option<Vec<u8>> {
        let mut out = kind.to_be_bytes().to_vec();
        out.push(u8::try_from(keys.len()).ok()?);
        keys.iter().for_each(|k| out.extend_from_slice(k));
        out.extend_from_slice(&occurrence.to_be_bytes());
        Some(out)
    }

    pub fn bytes(&self) -> Option<Vec<u8>> {
        let mut out = Self::record_prefix(self.kind, &self.keys, self.occurrence)?;
        out.push(self.position);
        Some(out)
    }

    /// The locator a key spells, read whole or none.
    pub fn of(b: &[u8]) -> Option<Locator> {
        let kind = u16::from_be_bytes(b.get(0..2)?.try_into().ok()?);
        let n = usize::from(*b.get(2)?);
        let mut p = 3usize;
        let mut keys = Vec::new();
        for _ in 0..n {
            keys.push(b.get(p..p + 32)?.try_into().ok()?);
            p += 32;
        }
        let occurrence = u32::from_be_bytes(b.get(p..p + 4)?.try_into().ok()?);
        let position = *b.get(p + 4)?;
        (b.len() == p + 5).then_some(Locator { kind, keys, occurrence, position })
    }
}

impl std::fmt::Display for Locator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kind {}[{}]", self.kind, self.position)?;
        if !self.keys.is_empty() {
            write!(f, " of {} key(s)", self.keys.len())?;
        }
        if self.occurrence > 0 {
            write!(f, " #{}", self.occurrence)?;
        }
        Ok(())
    }
}

/// ONE OBSERVATION of a series: the house's percept sequence it stood at,
/// the house's local time there (where it is a quantity), the value, and —
/// for a quantity seen at the beat before too — the change that carried it.
#[derive(Clone, Debug)]
pub struct Seen {
    pub beat: u64,
    pub at: Option<Q>,
    pub value: Arg,
    pub change: Option<Change>,
    /// 78.148 — for a versor seen at the beat before too, the versor that
    /// carried it there (`carrier`); none where either is no versor.
    pub carrier: Option<Mv>,
}

/// HOW A SERIES RETURNS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Return {
    /// It stands still: period one.
    Fixed,
    /// It returns every `period` beats; `span` is that period in the house's
    /// local time, read off the last return's two beats where both are
    /// quantities.
    Orbit { period: u64, span: Option<Q> },
}

impl Return {
    pub fn period(&self) -> u64 {
        match self {
            Return::Fixed => 1,
            Return::Orbit { period, .. } => *period,
        }
    }
}

/// ONE ARGUMENT'S SERIES over the seat's beats.
#[derive(Clone, Debug, Default)]
pub struct Series {
    seen: VecDeque<Seen>,
    held_bytes: usize,
    /// Observations dropped past the bound, oldest first.
    pub dropped: u64,
}

fn written(a: &Arg) -> usize {
    let mut out = Vec::new();
    match a.put(&mut out) {
        Some(()) => out.len(),
        // An argument that cannot be written was read off the wire whole, so
        // this is never reached; it is held at its tag's byte.
        None => 1,
    }
}

impl Series {
    /// The observations held, oldest first.
    pub fn seen(&self) -> impl Iterator<Item = &Seen> {
        self.seen.iter()
    }

    /// The latest observation.
    pub fn latest(&self) -> Option<&Seen> {
        self.seen.back()
    }

    /// Observe `value` at `beat`: its change from the beat before where both
    /// are quantities, then held within the frame's budget.
    fn observe(&mut self, beat: u64, at: Option<Q>, value: Arg) {
        let prev = self.seen.back().filter(|prev| prev.beat.checked_add(1) == Some(beat));
        let change = match (prev, quantity(&value)) {
            (Some(prev), Some(now)) => quantity(&prev.value).and_then(|before| Change::between(&before, &now)),
            _ => None,
        };
        let carried = match (prev.map(|p| &p.value), &value) {
            (Some(Arg::Mv(before)), Arg::Mv(now)) => carrier(before, now),
            _ => None,
        };
        self.held_bytes += written(&value);
        self.seen.push_back(Seen { beat, at, value, change, carrier: carried });
        while self.held_bytes > crate::network::wire::frame_budget() && self.seen.len() > 1 {
            if let Some(old) = self.seen.pop_front() {
                self.held_bytes -= written(&old.value);
                self.dropped += 1;
            }
        }
    }

    /// THE SERIES' RETURN, read on its contiguous tail: the least period whose
    /// last two turns repeat — for a quantity, the changes repeat and one
    /// period's slides compose to the identity; otherwise the values repeat.
    /// `None` while no return has been seen twice.
    pub fn orbit(&self) -> Option<Return> {
        let all: Vec<&Seen> = self.seen.iter().collect();
        // The contiguous tail: the beats run without a gap.
        let mut start = all.len().checked_sub(1)?;
        while start > 0 && all[start - 1].beat.checked_add(1) == Some(all[start].beat) {
            start -= 1;
        }
        let run = all.get(start..)?;
        let quantities = run.iter().all(|s| quantity(&s.value).is_some());
        // 78.148 — a versor series whose every change was carried exactly.
        let carriers: Vec<&Mv> = run.iter().skip(1).filter_map(|s| s.carrier.as_ref()).collect();
        let versors = run.iter().all(|s| matches!(s.value, Arg::Mv(_))) && carriers.len() == run.len().checked_sub(1)?;
        let period = match (quantities, versors) {
            (true, _) => {
                let changes: Vec<&Change> = run.iter().skip(1).filter_map(|s| s.change.as_ref()).collect();
                (changes.len() == run.len().checked_sub(1)?).then(|| periodic_changes(&changes)).flatten()?
            }
            (false, true) => {
                let p = periodic_carriers(&carriers)?;
                // A steady carrier that moves the value is an orbit of
                // period one; only the carrier one is standing still.
                let moving = carriers.iter().rev().take(p).any(|c| !same_carrier(c, &Mv::scalar(Q::one())));
                if p == 1 && moving {
                    return Some(Return::Orbit { period: 1, span: span_back(run, 1) });
                }
                p
            }
            (false, false) => {
                let values: Vec<&Arg> = run.iter().map(|s| &s.value).collect();
                periodic_values(&values)?
            }
        };
        if period == 1 {
            return Some(Return::Fixed);
        }
        Some(Return::Orbit { period: u64::try_from(period).ok()?, span: span_back(run, period) })
    }
}

/// The house's local time across the last `period` beats of a run.
fn span_back(run: &[&Seen], period: usize) -> Option<Q> {
    let last = run.len().checked_sub(1)?;
    match (run.get(last).and_then(|s| s.at.as_ref()), run.get(last.checked_sub(period)?).and_then(|s| s.at.as_ref())) {
        (Some(now), Some(then)) => Some(now - then),
        _ => None,
    }
}

/// 78.148 — the least `p` whose last `2p` carriers repeat with period `p`,
/// coefficient for coefficient.
fn periodic_carriers(carriers: &[&Mv]) -> Option<usize> {
    let n = carriers.len();
    (1..=n / 2).find(|&p| {
        let tail = &carriers[n - 2 * p..];
        (0..p).all(|i| match (tail.get(i), tail.get(i + p)) {
            (Some(a), Some(b)) => same_carrier(a, b),
            _ => false,
        })
    })
}

/// The least `p` whose last `2p` changes repeat with period `p` and whose
/// last period's slides compose back to the identity.
fn periodic_changes(changes: &[&Change]) -> Option<usize> {
    let n = changes.len();
    (1..=n / 2).find(|&p| {
        let tail = &changes[n - 2 * p..];
        let repeats = (0..p).all(|i| tail.get(i) == tail.get(i + p));
        repeats
            && tail
                .get(p..)
                .and_then(|one| {
                    one.iter().try_fold(None::<FlowPoint>, |acc, c| match acc {
                        None => Some(Some(c.slide.clone())),
                        Some(a) => a.then(&c.slide).map(Some),
                    })
                })
                .flatten()
                .is_some_and(|composed| composed.returned())
    })
}

/// The least `p` whose last `2p` values repeat with period `p`.
fn periodic_values(values: &[&Arg]) -> Option<usize> {
    let n = values.len();
    (1..=n / 2).find(|&p| {
        let tail = &values[n - 2 * p..];
        (0..p).all(|i| tail.get(i) == tail.get(i + p))
    })
}

/// A COUNSEL the house gave at the latest beat: the call its in-game mind
/// would make, with the worth and cost it states.
#[derive(Clone, Debug, PartialEq)]
pub struct Counsel {
    pub fn_id: u16,
    pub args: Vec<Arg>,
    pub worth: Arg,
    pub cost: Arg,
}

/// WHAT ONE PERCEPT CAME TO.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Perceived {
    /// The house's percept sequence: the seat's beat.
    pub beat: u64,
    /// Records and counsel read.
    pub records: usize,
    pub counsel: usize,
    /// Series held, of them standing still, and returning with a period.
    pub series: usize,
    pub fixed: usize,
    pub orbits: usize,
    /// The distinct periods the returning series hold.
    pub periods: Vec<u64>,
    /// A call whose outcome this percept's records were read against: its
    /// function, and whether the house applied it.
    pub learned: Option<(u16, bool)>,
    /// A call whose outcome never came within its beat.
    pub abandoned: Option<u16>,
}

impl std::fmt::Display for Perceived {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "beat {} records {} counsel {} | series {} fixed {} orbits {} periods {:?}",
            self.beat, self.records, self.counsel, self.series, self.fixed, self.orbits, self.periods
        )?;
        if let Some((fn_id, applied)) = self.learned {
            write!(f, " | learned fn {fn_id} (applied {applied})")?;
        }
        if let Some(fn_id) = self.abandoned {
            write!(f, " | fn {fn_id} unanswered within its beat")?;
        }
        Ok(())
    }
}

/// A CALL MADE AND NOT YET READ: what the quantities stood at when it was
/// made, and the house's answer once it comes.
#[derive(Clone, Debug)]
pub(crate) struct Pending {
    pub(crate) fn_id: u16,
    pub(crate) args: Vec<Arg>,
    pub(crate) reference: [u8; 32],
    pub(crate) before: Vec<(Vec<u8>, Q)>,
    pub(crate) applied: Option<bool>,
    pub(crate) percepts_since: u64,
}

/// THE MIND AT ONE SEAT: its series, the house's latest counsel, what each
/// of its calls was seen to do, and the call it awaits.
pub struct SeatMind {
    pub domain: [u8; 32],
    pub seat: u32,
    /// The latest beat perceived (the house's percept sequence).
    pub beat: u64,
    pub(crate) series: Keyed<Series>,
    pub(crate) counsel: Vec<Counsel>,
    pub(crate) experience: Keyed<Experience>,
    pub(crate) pending: Option<Pending>,
    /// 78.148 — its own seat's functions as the listing names them, and
    /// what it learned of their arguments (`own`).
    pub(crate) repertoire: Vec<Own>,
    /// 78.148 — what each call, a function AT ITS ARGUMENTS, was seen to do
    /// (`own::call_key`).
    pub(crate) own: Keyed<Experience>,
}

impl SeatMind {
    pub fn new(domain: [u8; 32], seat: u32) -> SeatMind {
        SeatMind {
            domain,
            seat,
            beat: 0,
            series: Keyed::new(),
            counsel: Vec::new(),
            experience: Keyed::new(),
            pending: None,
            repertoire: Vec::new(),
            own: Keyed::new(),
        }
    }

    /// The series at a locator.
    pub fn series(&self, at: &Locator) -> Option<&Series> {
        self.series.get(&at.bytes()?)
    }

    /// Every series held, at its locator.
    pub fn every_series(&self) -> impl Iterator<Item = (Option<Locator>, &Series)> {
        self.series.scoped().map(|(k, s)| (Locator::of(&k), s))
    }

    /// The house's counsel at the latest beat, in the house's order.
    pub fn counsel(&self) -> &[Counsel] {
        &self.counsel
    }

    /// A RECORD'S PERIOD: the lcm of its arguments' (a fixed one counts one),
    /// read over the record's fiber; `None` where one of them has not
    /// returned, or the lcm passes its carrier.
    pub fn record_period(&self, kind: u16, keys: &[[u8; 32]], occurrence: u32) -> Option<u64> {
        let prefix = Locator::record_prefix(kind, keys, occurrence)?;
        let mut lcm = 1u64;
        let mut any = false;
        for (_, s) in self.series.fiber_at(&prefix)? {
            any = true;
            let p = s.orbit()?.period();
            let g = num_integer::Integer::gcd(&lcm, &p);
            lcm = (lcm / g).checked_mul(p)?;
        }
        any.then_some(lcm)
    }

    /// PERCEIVE ONE PERCEPT: every non-key argument of every record observed
    /// on its series at the house's sequence; the counsel kept; and a call
    /// whose outcome came read against what moved since it was made.
    pub fn perceive(&mut self, p: &Percept) -> Perceived {
        self.beat = p.seq;
        let at = quantity(&p.local_time);
        let mut reading = Perceived { beat: p.seq, records: p.records.len(), ..Perceived::default() };
        self.counsel.clear();
        // Occurrences among records sharing a kind and keys, by their record
        // prefix: the house's order within one percept is its statement.
        let mut occurrences: Keyed<u32> = Keyed::new();
        for r in &p.records {
            match r {
                Record::Counsel { fn_id, args, worth, cost } => {
                    self.counsel.push(Counsel { fn_id: *fn_id, args: args.clone(), worth: worth.clone(), cost: cost.clone() });
                    let a = args.clone();
                    self.with_experience(*fn_id, |e| e.args = Some(a));
                }
                Record::Plain { kind, args } => {
                    let keys: Vec<[u8; 32]> = args
                        .iter()
                        .filter_map(|a| match a {
                            Arg::Key(k) => Some(*k),
                            _ => None,
                        })
                        .collect();
                    let Some(prefix) = Locator::record_prefix(*kind, &keys, 0) else { continue };
                    let occurrence = match occurrences.get(&prefix) {
                        Some(n) => *n,
                        None => 0,
                    };
                    occurrences.insert(&prefix, occurrence + 1);
                    for (i, a) in args.iter().enumerate() {
                        if matches!(a, Arg::Key(_)) {
                            continue;
                        }
                        let Ok(position) = u8::try_from(i) else { continue };
                        let loc = Locator { kind: *kind, keys: keys.clone(), occurrence, position };
                        let Some(key) = loc.bytes() else { continue };
                        if self.series.get(&key).is_none() {
                            self.series.insert(&key, Series::default());
                        }
                        if let Some(s) = self.series.get_mut(&key) {
                            s.observe(p.seq, at.clone(), a.clone());
                        }
                    }
                }
            }
        }
        reading.counsel = self.counsel.len();
        self.read_pending(&mut reading);
        for (_, s) in self.series.scoped() {
            reading.series += 1;
            match s.orbit() {
                Some(Return::Fixed) => reading.fixed += 1,
                Some(Return::Orbit { period, .. }) => {
                    reading.orbits += 1;
                    if !reading.periods.contains(&period) {
                        reading.periods.push(period);
                    }
                }
                None => {}
            }
        }
        reading.periods.sort_unstable();
        reading
    }

    /// THE HOUSE'S ANSWER TO A CALL: the call it names, if it is the one
    /// this mind awaits, marked applied or refused, and counted on its
    /// function and on the call at its arguments; the repertoire taught by
    /// it (78.148: a BAD ARGUMENTS refusal bounds a domain, NO SUCH FUNCTION
    /// takes the function out until the listing is read again). Returns the
    /// function it answered.
    pub fn outcome(&mut self, o: &Outcome) -> Option<u16> {
        let pending = self.pending.as_mut().filter(|p| p.reference == o.call)?;
        pending.applied = Some(o.applied);
        let (fn_id, args) = (pending.fn_id, pending.args.clone());
        let count = |e: &mut Experience| match o.applied {
            true => e.applied += 1,
            false => match e.refused.iter_mut().find(|(r, _)| *r == o.reason) {
                Some((_, n)) => *n += 1,
                None => e.refused.push((o.reason, 1)),
            },
        };
        self.with_experience(fn_id, count);
        self.with_call_experience(fn_id, &args, count);
        self.own_answered(fn_id, &args, o.applied, o.reason);
        Some(fn_id)
    }

    /// THE MIND CALLED: what its quantities stand at now is what the call's
    /// effect is read against.
    pub fn called(&mut self, fn_id: u16, args: Vec<Arg>, reference: [u8; 32]) {
        let beat = self.beat;
        let before = self
            .series
            .scoped()
            .filter_map(|(k, s)| {
                let last = s.latest().filter(|l| l.beat == beat)?;
                Some((k, quantity(&last.value)?))
            })
            .collect();
        let held = args.clone();
        self.with_experience(fn_id, |e| e.args = Some(held));
        self.pending = Some(Pending { fn_id, args, reference, before, applied: None, percepts_since: 0 });
    }

    /// The call this mind awaits an answer or a reading for.
    pub fn awaiting(&self) -> Option<u16> {
        self.pending.as_ref().map(|p| p.fn_id)
    }

    /// What one function's calls were seen to do.
    pub fn experience(&self, fn_id: u16) -> Option<&Experience> {
        self.experience.get(&fn_id.to_be_bytes())
    }

    /// One function's experience, written: taken from its coordinate (or
    /// begun, where the function was never seen) and put back there.
    pub(crate) fn with_experience<R>(&mut self, fn_id: u16, f: impl FnOnce(&mut Experience) -> R) -> R {
        let key = fn_id.to_be_bytes();
        let mut e = match self.experience.remove(&key) {
            Some(e) => e,
            None => Experience::default(),
        };
        let r = f(&mut e);
        self.experience.insert(key, e);
        r
    }

    /// A pending call at a new percept: answered and applied, its effect is
    /// every quantity's change since it was made; answered and refused, it
    /// moved nothing of its own; unanswered past its beat, it is let go.
    fn read_pending(&mut self, reading: &mut Perceived) {
        let Some(mut pending) = self.pending.take() else { return };
        pending.percepts_since += 1;
        match pending.applied {
            Some(applied) => {
                if applied {
                    let mut effects = Vec::new();
                    for (k, before) in &pending.before {
                        let Some(now) = self.series.get(k).and_then(|s| s.latest()).filter(|l| l.beat == self.beat) else { continue };
                        let Some(now) = quantity(&now.value) else { continue };
                        if let Some(c) = Change::between(before, &now) {
                            effects.push((k.clone(), c.slide));
                        }
                    }
                    let at_args = effects.clone();
                    self.with_experience(pending.fn_id, |e| e.learn(effects));
                    self.with_call_experience(pending.fn_id, &pending.args, |e| e.learn(at_args));
                }
                reading.learned = Some((pending.fn_id, applied));
            }
            // The house answers a call within the beat it was made in; a
            // second percept with no answer lets it go.
            None if pending.percepts_since >= 2 => reading.abandoned = Some(pending.fn_id),
            None => self.pending = Some(pending),
        }
    }
}
