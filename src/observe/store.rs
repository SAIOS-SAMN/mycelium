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
//!   push   [ACT_OBSERVE][1][vk 32][m u32]([reading])·m
//!          → [ok u8]
//! ```
//!
//! A voice PUSHES its totals and the hypha holds them under the voice's key
//! as they stand (the voice accumulated them; the hypha does not fold them
//! a second time). Any process QUERIES: the console asks a hypha for
//! everything it holds, its own court's readings and its voices'.

use super::reading::Reading;
use crate::anastome::dialogue::ACT_OBSERVE;
use crate::network::nerve::Keyed;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// WHO A READING IS OF: a court (a hypha) or a voice, by its key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Court([u8; 32]),
    Voice([u8; 32]),
}

pub const SOURCE_COURT: u8 = 0;
pub const SOURCE_VOICE: u8 = 1;

impl Source {
    /// The source's prefix: its kind, then its key.
    pub fn prefix(&self) -> Vec<u8> {
        let (kind, vk) = match self {
            Source::Court(vk) => (SOURCE_COURT, vk),
            Source::Voice(vk) => (SOURCE_VOICE, vk),
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
            SOURCE_VOICE => Some((Source::Voice(vk), tag)),
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

/// HOLD a reading as it stands — a voice's pushed totals, a court's present
/// state — in place of whatever was held for `source`.
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

/// Count one refusal by its kind, where the court refuses.
pub fn note_obstruction(kind: crate::transducer::sentence::ObstructionKind) {
    OBSTRUCTED[kind.index()].fetch_add(1, Ordering::Relaxed);
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
        obstructed: std::array::from_fn(|i| OBSTRUCTED[i].load(Ordering::Relaxed)),
    }
}

/// THE ACT, decoded.
#[derive(Debug, PartialEq, Eq)]
pub enum ObserveAct {
    Query(Vec<u8>),
    Push([u8; 32], Vec<Reading>),
}

const MODE_QUERY: u8 = 0;
const MODE_PUSH: u8 = 1;

pub fn encode_query(prefix: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_OBSERVE, MODE_QUERY];
    out.extend_from_slice(&(prefix.len() as u32).to_le_bytes());
    out.extend_from_slice(prefix);
    out
}

pub fn encode_push(vk: &[u8; 32], readings: &[Reading]) -> Vec<u8> {
    let mut out = vec![ACT_OBSERVE, MODE_PUSH];
    out.extend_from_slice(vk);
    out.extend_from_slice(&(readings.len() as u32).to_le_bytes());
    for r in readings {
        r.put(&mut out);
    }
    out
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
        MODE_PUSH => {
            let vk: [u8; 32] = rest.get(..32)?.try_into().ok()?;
            p = 32;
            let m = take_u32(rest, &mut p)? as usize;
            // A reading is at least its tag; a count the body cannot hold
            // is refused before anything is reserved.
            if rest.len().checked_sub(p)? < m {
                return None;
            }
            let mut readings = Vec::with_capacity(m);
            for _ in 0..m {
                readings.push(Reading::take(rest, &mut p)?);
            }
            ObserveAct::Push(vk, readings)
        }
        _ => return None,
    };
    // The act is read whole or not at all.
    (p == rest.len()).then_some(act)
}

/// The reply to a query: how many readings are held under the prefix, and
/// as many of them, in key order, as fit `budget` bytes. A reply that could
/// not hold them all says so by its total, never by silence.
pub fn encode_reply(entries: &[Held], budget: usize) -> Vec<u8> {
    let mut body = Vec::new();
    let mut m = 0u32;
    for (k, r) in entries {
        let mut one = Vec::new();
        one.extend_from_slice(&(k.len() as u32).to_le_bytes());
        one.extend_from_slice(k);
        r.put(&mut one);
        if 8 + body.len() + one.len() > budget {
            break;
        }
        body.extend_from_slice(&one);
        m += 1;
    }
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    out.extend_from_slice(&m.to_le_bytes());
    out.extend_from_slice(&body);
    out
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
