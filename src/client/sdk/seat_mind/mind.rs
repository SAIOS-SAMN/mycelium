use super::change::{quantity, Change};
use super::perceived::{Counsel, Dilation, Located, Perceived};
use super::series::{Locator, Return, Series};
use super::{Experience, Own};
use crate::anastome::dialogue::seat::{Arg, Outcome, Percept, Record};
use crate::network::nerve::Keyed;
use num_rational::BigRational as Q;

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
    /// v1.4 — the sub-universe the seat stands in; none, the domain's own.
    pub sub_origin: Option<[u8; 32]>,
    pub seat: u32,
    /// 78.156 — where the sub-instance last stood, (TPoint, u).
    pub(crate) located: Option<Located>,
    /// 78.156 — its experienced dilation at the last located percept.
    pub(crate) dilation: Option<Dilation>,
    /// The latest beat perceived (the house's percept sequence).
    pub beat: u64,
    pub(crate) series: Keyed<Series>,
    pub(crate) counsel: Vec<Counsel>,
    pub(crate) experience: Keyed<Experience>,
    pub(crate) pending: Option<Pending>,
    /// 78.148 — its own seat's functions as the listing names them (`own`).
    pub(crate) repertoire: Vec<Own>,
    /// 78.148 — what each call, a function AT ITS ARGUMENTS, was seen to do
    /// (`own::call_key`). The mind calls only what it read here or was
    /// counselled — never a blind probe (a house's `handle`
    /// closes every throat a line holds, `forge` spends ~1e21 J).
    pub(crate) own: Keyed<Experience>,
}

impl SeatMind {
    pub fn new(domain: [u8; 32], seat: u32) -> SeatMind {
        SeatMind::scoped(domain, None, seat)
    }

    /// v1.4 — the mind at a seat named by (domain, sub-origin, seat).
    pub fn scoped(domain: [u8; 32], sub_origin: Option<[u8; 32]>, seat: u32) -> SeatMind {
        SeatMind {
            domain,
            sub_origin,
            seat,
            located: None,
            dilation: None,
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

    /// 78.156 — where the sub-instance last stood: (TPoint, u).
    pub fn located(&self) -> Option<&Located> {
        self.located.as_ref()
    }

    /// 78.156 — THE EXPERIENCED-DILATION READING at this seat: the world's
    /// advance against the final point's between its last two located
    /// percepts, exact.
    pub fn dilation(&self) -> Option<&Dilation> {
        self.dilation.as_ref()
    }

    /// A percept located at (its point, its `u`), where it carries both: the
    /// dilation since the last located percept read. A percept without a
    /// point or a quantity `u` locates nothing and leaves the reading as it
    /// stood.
    fn locate(&mut self, p: &Percept, u: Option<&Q>) -> Option<Dilation> {
        let now = Located { point: p.point?, u: u?.clone() };
        let d = self.located.as_ref().and_then(|before| Dilation::between(before, &now));
        self.located = Some(now);
        self.dilation = d.clone();
        d
    }

    /// A RECORD'S PERIOD: the lcm of its arguments' (a fixed one counts one),
    /// read over the record's fibre; `None` where one of them has not
    /// returned, or the lcm passes its carrier.
    pub fn record_period(&self, kind: u16, keys: &[[u8; 32]], occurrence: u32) -> Option<u64> {
        let prefix = Locator::record_prefix(kind, keys, occurrence)?;
        let mut lcm = 1u64;
        let mut any = false;
        for (_, s) in self.series.fibre_at(&prefix)? {
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
        let dilation = self.locate(p, at.as_ref());
        let mut reading = Perceived { beat: p.seq, records: p.records.len(), dilation, ..Perceived::default() };
        self.counsel.clear();
        // Occurrences among records sharing a kind and keys, by the records'
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
                        // THE SEAT'S BOUND: what one frame could carry of its
                        // values, shared among the series it holds.
                        let bound = crate::network::wire::frame_budget() / self.series.len().max(1);
                        if let Some(s) = self.series.get_mut(&key) {
                            s.observe(p.seq, at.clone(), a.clone(), bound);
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
    /// it (78.148: NO SUCH FUNCTION takes the function out until the listing
    /// is read again). Returns the function it answered.
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
        self.own_answered(fn_id, o.applied, o.reason);
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
