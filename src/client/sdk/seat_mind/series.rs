use super::change::{carrier, quantity, same_carrier, Change};
use crate::anastome::dialogue::seat::Arg;
use crate::transducer::cga::Mv;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};
use std::collections::VecDeque;

/// WHERE A SERIES STANDS: the record's coordinates and the argument's
/// position. Written `[kind u16 BE][keys u8][key 32…][occurrence u32
/// BE][position u8]` — self-delimiting, so a record's series are one fibre
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
/// the house's local time there (where it is a quantity), and the value.
/// Its change from the beat before — a quantity's `Change`, a versor's
/// carrier — is DERIVED where it is read (`Series::changes`,
/// `Series::carriers`), never held (Concordance 1.13: a change held two flow
/// points of the Cl(4,1) algebra each, and grew a seat mind by hundreds of
/// megabytes, 2026-10-09).
#[derive(Clone, Debug)]
pub struct Seen {
    pub beat: u64,
    pub at: Option<Q>,
    pub value: Arg,
}

/// ONE STEP OF A QUANTITY SERIES, as its change compares: the difference
/// (the slide's parameter) and, where both values stand on one side of zero,
/// the ratio (the scale's). Two changes are one exactly where these are, and
/// one period's slides compose to the identity exactly where its differences
/// sum to zero — slides on the null flow compose by addition (`Change`).
#[derive(Clone, Debug, PartialEq, Eq)]
struct Step {
    delta: Q,
    ratio: Option<Q>,
}

impl Step {
    fn between(from: &Q, to: &Q) -> Step {
        let ratio = match from.is_zero() || to.is_zero() || from.is_negative() != to.is_negative() {
            true => None,
            false => Some(to / from),
        };
        Step { delta: to - from, ratio }
    }
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

    /// Observe `value` at `beat`, held within `bound` bytes of values as they
    /// write — the seat's frame budget shared among its series
    /// (`SeatMind::perceive`) — the oldest dropped and counted past it.
    pub(super) fn observe(&mut self, beat: u64, at: Option<Q>, value: Arg, bound: usize) {
        self.held_bytes += written(&value);
        self.seen.push_back(Seen { beat, at, value });
        while self.held_bytes > bound && self.seen.len() > 1 {
            if let Some(old) = self.seen.pop_front() {
                self.held_bytes -= written(&old.value);
                self.dropped += 1;
            }
        }
    }

    /// Each consecutive pair of observations one beat apart, oldest first.
    fn consecutive(&self) -> impl Iterator<Item = (&Seen, &Seen)> {
        self.seen.iter().zip(self.seen.iter().skip(1)).filter(|(a, b)| a.beat.checked_add(1) == Some(b.beat))
    }

    /// THE QUANTITY'S CHANGES, derived: for each two consecutive beats whose
    /// values are both quantities, the motions of its line carrying one onto
    /// the other, oldest first.
    pub fn changes(&self) -> Vec<Change> {
        self.consecutive()
            .filter_map(|(a, b)| Change::between(&quantity(&a.value)?, &quantity(&b.value)?))
            .collect()
    }

    /// 78.148 — THE VERSOR'S CARRIERS, derived: for each two consecutive
    /// beats whose values are both multivectors, the versor carrying one onto
    /// the other where it is exact (`carrier`), oldest first.
    pub fn carriers(&self) -> Vec<Mv> {
        self.consecutive()
            .filter_map(|(a, b)| match (&a.value, &b.value) {
                (Arg::Mv(x), Arg::Mv(y)) => carrier(x, y),
                _ => None,
            })
            .collect()
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
        let quantities: Option<Vec<Q>> = run.iter().map(|s| quantity(&s.value)).collect();
        // 78.148 — a versor series whose every change is carried exactly.
        let carriers: Option<Vec<Mv>> = match run.iter().all(|s| matches!(s.value, Arg::Mv(_))) {
            true => run
                .iter()
                .zip(run.iter().skip(1))
                .map(|(a, b)| match (&a.value, &b.value) {
                    (Arg::Mv(x), Arg::Mv(y)) => carrier(x, y),
                    _ => None,
                })
                .collect(),
            false => None,
        };
        let period = match (quantities, carriers) {
            (Some(qs), _) => {
                let steps: Vec<Step> = qs.iter().zip(qs.iter().skip(1)).map(|(a, b)| Step::between(a, b)).collect();
                periodic_steps(&steps)?
            }
            (None, Some(carriers)) => {
                let carriers: Vec<&Mv> = carriers.iter().collect();
                let p = periodic_carriers(&carriers)?;
                // A steady carrier that moves the value is an orbit of
                // period one; only the carrier one is standing still.
                let moving = carriers.iter().rev().take(p).any(|c| !same_carrier(c, &Mv::scalar(Q::one())));
                if p == 1 && moving {
                    return Some(Return::Orbit { period: 1, span: span_back(run, 1) });
                }
                p
            }
            (None, None) => {
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
/// last period's slides compose back to the identity: its differences sum
/// to zero.
fn periodic_steps(steps: &[Step]) -> Option<usize> {
    let n = steps.len();
    (1..=n / 2).find(|&p| {
        let tail = &steps[n - 2 * p..];
        let repeats = (0..p).all(|i| tail.get(i) == tail.get(i + p));
        repeats && tail.get(p..).is_some_and(|one| one.iter().fold(Q::zero(), |acc, s| acc + &s.delta).is_zero())
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
