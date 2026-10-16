// §8.1, audit of Phase N — THE OBSERVER'S STORE: every reading this process
// holds, keyed by its coordinates, and the act that carries them.
//
//! A reading is keyed `[source kind][vk 32][reading tag]` — the source's
//! own identity, then what the reading answers — so a query by prefix reads
//! one source, or one kind of source, or everything, and never reads more
//! than the fiber it names (rule 1).
//!
//! ONE STORE PER PROCESS, beside the thread-local tallies it is fed from.
//! Those are taken by their one reader where a stroke ends and folded in
//! here; nothing in cognition reads this store back (it is an instrument,
//! §8.1).
//!
//! THE ACT, `ACT_OBSERVE`:
//!
//! ```text
//!   query  [ACT_OBSERVE][0][prefix len u32][prefix]
//!          → [total u32][m u32]([key len u32][key][reading])·m
//! ```
//!
//! Any process QUERIES: the console asks a hypha for everything it holds of
//! its own court, and a mind, on its post, for its own reading.

use super::reading::Reading;
use crate::anastome::dialogue::ACT_OBSERVE;
use crate::network::nerve::Keyed;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// WHO A READING IS OF: a court (a hypha), or a mind, by its key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Court([u8; 32]),
    Mind([u8; 32]),
}

pub const SOURCE_COURT: u8 = 0;
pub const SOURCE_MIND: u8 = 1;

impl Source {
    /// The source's prefix: its kind, then its key.
    pub fn prefix(&self) -> Vec<u8> {
        let (kind, vk) = match self {
            Source::Court(vk) => (SOURCE_COURT, vk),
            Source::Mind(vk) => (SOURCE_MIND, vk),
        };
        let mut k = Vec::with_capacity(34);
        k.push(kind);
        k.extend_from_slice(vk);
        k
    }

    /// The key a reading of this source is held at.
    pub fn key(&self, tag: u8) -> Vec<u8> {
        let mut k = self.prefix();
        k.push(tag);
        k
    }

    /// The source and reading tag a key names; `None` for a key of another
    /// layout.
    pub fn of_key(k: &[u8]) -> Option<(Source, u8)> {
        let [kind, rest @ ..] = k else { return None };
        let (vk, tag) = match rest {
            [v @ .., tag] if v.len() == 32 => (<[u8; 32]>::try_from(v).ok()?, *tag),
            _ => return None,
        };
        match *kind {
            SOURCE_COURT => Some((Source::Court(vk), tag)),
            SOURCE_MIND => Some((Source::Mind(vk), tag)),
            _ => None,
        }
    }
}

/// ONE HELD READING: the key it stands at and the reading.
pub type Held = (Vec<u8>, Reading);

static STORE: LazyLock<Mutex<Keyed<Reading>>> = LazyLock::new(|| Mutex::new(Keyed::new()));

/// Acts served by kind, counted where the dispatch hands one to its judge.
static SERVED: [AtomicU64; 256] = [const { AtomicU64::new(0) }; 256];

/// FOLD a reading this process produced into what it holds for `source`.
/// `false` where the store could not be read (a poisoned lock), which is a
/// reading not taken and is said so to the caller.
pub fn publish(source: Source, reading: Reading) -> bool {
    let Ok(mut s) = STORE.lock() else { return false };
    let k = source.key(reading.tag());
    match s.get_mut(&k) {
        Some(held) => held.absorb(&reading),
        None => {
            s.insert(k, reading);
            true
        }
    }
}

/// HOLD a reading as it stands — a court's present state — in place of whatever was held for `source`.
pub fn replace(source: Source, reading: Reading) -> bool {
    let Ok(mut s) = STORE.lock() else { return false };
    s.insert(source.key(reading.tag()), reading);
    true
}

/// Every reading held under `prefix`, in key order; the empty prefix reads
/// the whole store, which is itself the one fiber the console asks for.
pub fn readings(prefix: &[u8]) -> Vec<Held> {
    let Ok(s) = STORE.lock() else { return Vec::new() };
    let fiber = match s.fiber_at(prefix) {
        Some(f) => f,
        None => s.scoped(),
    };
    fiber.map(|(k, r)| (k.to_vec(), r.clone())).collect()
}

/// Count one act served, by its kind.
pub fn note_served(kind: u8) {
    SERVED[usize::from(kind)].fetch_add(1, Ordering::Relaxed);
}

/// The acts served, each kind that was served with its count.
pub fn served() -> Vec<(u8, u64)> {
    (0..=u8::MAX)
        .map(|kind| (kind, SERVED[usize::from(kind)].load(Ordering::Relaxed)))
        .filter(|&(_, n)| n > 0)
        .collect()
}

/// PH5 — THE LAST PHASE WINDOW THIS PROCESS'S COURT WROTE, and the windows
/// written: the instrument's copy, handed over where the court writes its
/// line, so the court itself keeps no phase (45.9).
static PHASE: LazyLock<Mutex<Option<super::Phase>>> = LazyLock::new(|| Mutex::new(None));

/// Hold one written window, counting it.
pub fn note_phase(mut p: super::Phase) {
    let Ok(mut held) = PHASE.lock() else { return };
    p.windows = held.as_ref().map_or(0, |h| h.windows) + 1;
    *held = Some(p);
}

/// The last window written; `None` before the first.
pub fn last_phase() -> Option<super::Phase> {
    PHASE.lock().ok()?.clone()
}

/// REPLICATION, counted where the exchange and the back-fill classify each
/// frame (CV5): per signer by its key, and apart for a frame no signer
/// could be read off.
static REPLICATION: LazyLock<Mutex<Keyed<[u64; super::REP_AXES]>>> = LazyLock::new(|| Mutex::new(Keyed::new()));
static UNATTRIBUTED: [AtomicU64; super::REP_AXES] = [const { AtomicU64::new(0) }; super::REP_AXES];

/// Count `n` on one axis of [`super::REPLICATION_AXES`] for `signer`, or
/// unattributed where the frame named none. An axis past the list is no
/// axis and counts nothing.
pub fn note_replication(signer: Option<[u8; 32]>, axis: usize, n: u64) {
    match signer {
        None => {
            if let Some(a) = UNATTRIBUTED.get(axis) {
                a.fetch_add(n, Ordering::Relaxed);
            }
        }
        Some(vk) => {
            let Ok(mut r) = REPLICATION.lock() else { return };
            if r.get(&vk).is_none() {
                r.insert(vk, [0; super::REP_AXES]);
            }
            if let Some(slot) = r.get_mut(&vk).and_then(|a| a.get_mut(axis)) {
                *slot += n;
            }
        }
    }
}

/// SET a LEVEL on one axis for `signer` — the lag at the last report
/// ([`super::rep::LAG`]) — which is where the signer stands now, not a
/// count to be summed.
pub fn set_replication(signer: [u8; 32], axis: usize, v: u64) {
    let Ok(mut r) = REPLICATION.lock() else { return };
    if r.get(&signer).is_none() {
        r.insert(signer, [0; super::REP_AXES]);
    }
    if let Some(slot) = r.get_mut(&signer).and_then(|a| a.get_mut(axis)) {
        *slot = v;
    }
}

/// The replication this process has counted, per signer in key order.
pub fn replication() -> super::Replication {
    let signers = match REPLICATION.lock() {
        Ok(r) => r
            .scoped()
            .filter_map(|(k, a)| Some((<[u8; 32]>::try_from(k.as_slice()).ok()?, *a)))
            .collect(),
        Err(_) => Vec::new(),
    };
    let unattributed = std::array::from_fn(|i| UNATTRIBUTED[i].load(Ordering::Relaxed));
    super::Replication { signers, unattributed }
}

/// ONE SENTENCE'S VERDICT at a court, as the adjudication reading counts it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Ratified,
    Stood,
    Awaiting,
    /// The door deferred the act; nothing was heard.
    Deferred,
    /// Refused, by the refusal's own kind (`RefusalKind::to_byte`).
    Refused(u8),
}

static VERDICTS: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
static REFUSED: [AtomicU64; 256] = [const { AtomicU64::new(0) }; 256];
/// CV1 — the court's refusals by the kind of obstruction (78.9), on
/// `ObstructionKind::index`'s axes.
static OBSTRUCTED: [AtomicU64; 6] = [const { AtomicU64::new(0) }; 6];

/// THE FOLD'S ROUND BY OUTCOME, on [`super::FOLD_OUTCOMES`]'s axes (LD0).
static FOLDS: [AtomicU64; super::FOLD_OUTCOMES.len()] = [const { AtomicU64::new(0) }; super::FOLD_OUTCOMES.len()];

/// One outcome of the fold's round.
pub fn note_fold(outcome: super::FoldOutcome) {
    FOLDS[outcome as usize].fetch_add(1, Ordering::Relaxed);
}

/// The fold outcomes this process has counted.
pub fn folds() -> [u64; super::FOLD_OUTCOMES.len()] {
    std::array::from_fn(|i| FOLDS[i].load(Ordering::Relaxed))
}

/// PHASE 1 — the catch-up pages by path and outcome (`Court::catchup`).
static CATCHUP: [[AtomicU64; super::CATCHUP_OUTCOMES.len()]; super::CATCHUP_PATHS.len()] =
    [const { [const { AtomicU64::new(0) }; super::CATCHUP_OUTCOMES.len()] }; super::CATCHUP_PATHS.len()];

/// One catch-up page, by its path and what came back.
pub fn note_catchup(path: super::CatchupPath, outcome: super::CatchupOutcome) {
    CATCHUP[path as usize][outcome as usize].fetch_add(1, Ordering::Relaxed);
}

/// The catch-up pages this process has counted.
pub fn catchups() -> [[u64; super::CATCHUP_OUTCOMES.len()]; super::CATCHUP_PATHS.len()] {
    std::array::from_fn(|p| std::array::from_fn(|o| CATCHUP[p][o].load(Ordering::Relaxed)))
}

/// 2026-10-08 — the serving side of the catch-up pages (`Court::serve`).
static SERVE: [AtomicU64; super::SERVE_AXES.len()] = [const { AtomicU64::new(0) }; super::SERVE_AXES.len()];

/// One serving count, by its axis: added for a count, raised for the level.
pub fn note_serve(axis: super::ServeAxis, n: u64) {
    match axis {
        super::ServeAxis::MostEras | super::ServeAxis::MostPublished => {
            SERVE[axis as usize].fetch_max(n, Ordering::Relaxed);
        }
        _ => {
            SERVE[axis as usize].fetch_add(n, Ordering::Relaxed);
        }
    }
}

/// The serving counts this process has counted.
pub fn serves() -> [u64; super::SERVE_AXES.len()] {
    std::array::from_fn(|i| SERVE[i].load(Ordering::Relaxed))
}

/// THE VERTEX-BODY REFUSALS by the fault their proof names, on
/// [`super::BODY_FAULTS`]'s axes, and by whether the vertex lifted a versor.
static BODY_FAULTS: [AtomicU64; super::BODY_FAULTS.len()] = [const { AtomicU64::new(0) }; super::BODY_FAULTS.len()];
static BODY_LIFTED: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];

/// Count one vertex-body refusal: its fault's slot on [`super::BODY_FAULTS`]
/// and whether its vertex lifted a versor on the chart.
pub fn note_body_fault(slot: usize, lifted: bool) {
    if let Some(n) = BODY_FAULTS.get(slot) {
        n.fetch_add(1, Ordering::Relaxed);
    }
    BODY_LIFTED[usize::from(lifted)].fetch_add(1, Ordering::Relaxed);
}

/// Count one refusal by its kind, where it forms.
pub fn note_obstruction(kind: crate::transducer::sentence::ObstructionKind) {
    OBSTRUCTED[kind.index()].fetch_add(1, Ordering::Relaxed);
}

/// The refusals this process has counted by kind.
pub(crate) fn obstructed() -> [u64; 6] {
    std::array::from_fn(|i| OBSTRUCTED[i].load(Ordering::Relaxed))
}

/// Count one verdict, where the court reaches it.
pub fn note_verdict(v: Verdict) {
    let at = match v {
        Verdict::Ratified => 0,
        Verdict::Stood => 1,
        Verdict::Awaiting => 2,
        Verdict::Deferred => 3,
        Verdict::Refused(kind) => {
            REFUSED[usize::from(kind)].fetch_add(1, Ordering::Relaxed);
            return;
        }
    };
    VERDICTS[at].fetch_add(1, Ordering::Relaxed);
}

/// The verdicts this process has counted.
pub fn adjudication() -> super::Adjudication {
    let v = |i: usize| VERDICTS[i].load(Ordering::Relaxed);
    super::Adjudication {
        ratified: v(0),
        stood: v(1),
        awaiting: v(2),
        deferred: v(3),
        refused: (0..=u8::MAX)
            .map(|k| (k, REFUSED[usize::from(k)].load(Ordering::Relaxed)))
            .filter(|&(_, n)| n > 0)
            .collect(),
        obstructed: obstructed(),
        body_faults: std::array::from_fn(|i| BODY_FAULTS[i].load(Ordering::Relaxed)),
        body_lifted: std::array::from_fn(|i| BODY_LIFTED[i].load(Ordering::Relaxed)),
    }
}

/// THE ACT, decoded.
#[derive(Debug, PartialEq, Eq)]
pub enum ObserveAct {
    Query(Vec<u8>),
}

const MODE_QUERY: u8 = 0;

/// `None` where the prefix passes its four bytes (STD1 3c).
pub fn encode_query(prefix: &[u8]) -> Option<Vec<u8>> {
    let mut out = vec![ACT_OBSERVE, MODE_QUERY];
    crate::transducer::address::put_bytes(&mut out, prefix)?;
    Some(out)
}

fn take_u32(b: &[u8], p: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

/// The act, or `None` for a body that is not one whole.
pub fn decode_act(b: &[u8]) -> Option<ObserveAct> {
    let [ACT_OBSERVE, mode, rest @ ..] = b else { return None };
    let mut p = 0usize;
    let act = match *mode {
        MODE_QUERY => {
            let len = take_u32(rest, &mut p)? as usize;
            let prefix = rest.get(p..p.checked_add(len)?)?.to_vec();
            p += len;
            ObserveAct::Query(prefix)
        }
        _ => return None,
    };
    // The act is read whole or not at all.
    (p == rest.len()).then_some(act)
}

/// The reply to a query: how many readings are held under the prefix, and
/// as many of them, in key order, as fit `budget` bytes. A reply that could
/// not hold them all says so by its total, never by silence.
///
/// STD1 3c: an entry whose key or reading passes the field that carries it
/// refuses the reply WHOLE — a total written beside a skipped entry would
/// be a count the entries do not bear out — and so does a total past its
/// four bytes. Only the budget leaves entries unsent, and the total says so.
pub fn encode_reply(entries: &[Held], budget: usize) -> Option<Vec<u8>> {
    // The total first: `m` counts at most one per entry, so it fits too.
    let total = u32::try_from(entries.len()).ok()?;
    let mut body = Vec::new();
    let mut m = 0u32;
    for (k, r) in entries {
        let mut one = Vec::new();
        crate::transducer::address::put_bytes(&mut one, k)?;
        r.put(&mut one)?;
        if 8 + body.len() + one.len() > budget {
            break;
        }
        body.extend_from_slice(&one);
        m += 1;
    }
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(&total.to_le_bytes());
    out.extend_from_slice(&m.to_le_bytes());
    out.extend_from_slice(&body);
    Some(out)
}

/// The inverse of [`encode_reply`]: the total held, and the readings sent.
pub fn decode_reply(b: &[u8]) -> Option<(u32, Vec<Held>)> {
    let mut p = 0usize;
    let total = take_u32(b, &mut p)?;
    let m = take_u32(b, &mut p)? as usize;
    if b.len().checked_sub(p)? < m {
        return None;
    }
    let mut out = Vec::with_capacity(m);
    for _ in 0..m {
        let len = take_u32(b, &mut p)? as usize;
        let k = b.get(p..p.checked_add(len)?)?.to_vec();
        p += len;
        out.push((k, Reading::take(b, &mut p)?));
    }
    (p == b.len()).then_some((total, out))
}
