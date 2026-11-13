// MU-CLIENT §4.4, §4.12, §7.2 — THE MIND SEEN AS IT STANDS (ruled
// 2026-10-15: the observer is built into the mind as an API that reads the
// live state and calls functions; it is removable later).
//
//! TWO THINGS, both served on the mind's post by the conversation's carrier,
//! never by the winding carrier, so a mind held inside one law is still
//! read:
//!
//! - WHERE THE WINDING CARRIER STANDS: the law it entered last, the
//!   boundaries it has passed, its strokes, and the mind's K and TUniversal
//!   height when it entered. Written as the carrier enters each law
//!   ([`enter`]); a reading whose boundary does not move between two
//!   questions is a carrier held inside that law, and names it.
//! - CALLS: a question by name (`now`, `thoughts`, `thought N`, `field`,
//!   `held`, `scenes`, `questions`, `calls`), answered by the mind out of
//!   what it holds, at the next law boundary its winding carrier passes
//!   ([`wanted`] / [`answer`]). The asker waits a park at a time while the
//!   carrier moves (law 9: the pace is strokes, never wall time); a park in
//!   which it passed no boundary answers with where the carrier stands and
//!   the last answer the call had.
//!
//! Nothing in cognition reads any of it back. A call reads; none moves the
//! mind (MU-CLIENT §2.4(8)).
//!
//! THE ACT, `ACT_OBSERVE` mode 1:
//!
//! ```text
//!   call  [ACT_OBSERVE][1][line len u32][line utf-8]
//!         → [n u32]([len u32][line utf-8])·n
//! ```

use crate::anastome::dialogue::ACT_OBSERVE;
use crate::network::nerve::Keyed;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, LazyLock, Mutex};

const MODE_CALL: u8 = 1;

/// WHERE THE WINDING CARRIER STANDS.
#[derive(Clone, Copy, Debug, Default)]
pub struct Now {
    /// The law it entered last.
    pub law: &'static str,
    /// The law boundaries it has passed since the mind began.
    pub boundary: u64,
    /// Its strokes since the mind began.
    pub stroke: u64,
    /// The mind's own K, and the TUniversal height its court last carried,
    /// when it entered the law.
    pub k: u64,
    pub h: Option<u64>,
    /// The boundary its present stroke began at.
    pub winding_boundary: u64,
    /// Whether it is parked — resting or blocked between windings — rather
    /// than working inside a law.
    pub parked: bool,
}

struct Slot {
    now: Now,
    /// Calls waiting for the carrier's next boundary, by the calls' line.
    wanted: Keyed<()>,
    /// Each call's last answer, with the boundary it was answered at.
    answers: Keyed<(u64, Vec<String>)>,
}

/// THE OBSERVER OF ONE MIND: where its winding carrier stands, the calls
/// left for it and their answers, and where each carrier beside the
/// winding stands. The process's mind is observed through [`process`],
/// which every free function of this module reads; a reader that holds
/// its own `Live` reads only what it wrote there, never another carrier's
/// standing in the same process.
pub struct Live {
    slot: Mutex<Slot>,
    /// Woken as a call is answered.
    woken: Condvar,
    /// Whether any call waits, read without the lock at every boundary.
    waiting: AtomicBool,
    /// MU-CLIENT §7.1 — THE WINDING'S WAKER: a mind at rest is blocked on
    /// its landings, so a call left for it wakes it (`wake_with`); none
    /// where no mind gave one.
    wake: Mutex<Option<std::sync::mpsc::Sender<()>>>,
    /// WHERE EACH OTHER CARRIER STANDS, by its name: the law it entered
    /// last and how many it has entered.
    others: Mutex<Keyed<(&'static str, u64)>>,
}

impl Default for Live {
    fn default() -> Self {
        Live::new()
    }
}

/// The process's mind's observer.
static PROCESS: LazyLock<Live> = LazyLock::new(Live::new);

impl Live {
    /// An observer no carrier has entered and no call has reached.
    pub fn new() -> Live {
        Live {
            slot: Mutex::new(Slot { now: Now::default(), wanted: Keyed::new(), answers: Keyed::new() }),
            woken: Condvar::new(),
            waiting: AtomicBool::new(false),
            wake: Mutex::new(None),
            others: Mutex::new(Keyed::new()),
        }
    }

    /// The waker a call left for the winding sends on.
    pub fn wake_with(&self, wake: std::sync::mpsc::Sender<()>) {
        if let Ok(mut w) = self.wake.lock() {
            *w = Some(wake);
        }
    }

    /// THE WINDING CARRIER BEGINS A STROKE at the mind's K and height `h`.
    pub fn winding_began(&self, k: u64, h: Option<u64>) {
        let Ok(mut s) = self.slot.lock() else { return };
        s.now.stroke += 1;
        s.now.k = k;
        s.now.h = h;
        s.now.winding_boundary = s.now.boundary;
    }

    /// THE WINDING CARRIER PARKS (`true`, as it rests or blocks) OR WAKES.
    pub fn park(&self, parked: bool) {
        let Ok(mut s) = self.slot.lock() else { return };
        s.now.parked = parked;
    }

    /// THE WINDING CARRIER ENTERS `law`.
    pub fn enter(&self, law: &'static str) {
        let Ok(mut s) = self.slot.lock() else { return };
        s.now.law = law;
        s.now.boundary += 1;
    }

    /// A CARRIER BESIDE THE WINDING (`name`) ENTERS `law`.
    pub fn enter_carrier(&self, name: &'static str, law: &'static str) {
        let Ok(mut c) = self.others.lock() else { return };
        let n = match c.get(name.as_bytes()) {
            Some((_, n)) => n + 1,
            None => 1,
        };
        c.insert(name.as_bytes(), (law, n));
    }

    /// Every other carrier's standing, one line each.
    pub fn carriers(&self) -> Vec<String> {
        let Ok(c) = self.others.lock() else { return Vec::new() };
        c.scoped().map(|(name, (law, n))| format!("{} carrier: in {law} · boundary {n}", String::from_utf8_lossy(&name))).collect()
    }

    /// Where the winding carrier stands.
    pub fn now(&self) -> Now {
        match self.slot.lock() {
            Ok(s) => s.now,
            Err(_) => Now::default(),
        }
    }

    /// The calls waiting for this boundary, taken; none without the lock
    /// where none waits.
    pub fn wanted(&self) -> Vec<String> {
        if !self.waiting.swap(false, Ordering::Relaxed) {
            return Vec::new();
        }
        let Ok(mut s) = self.slot.lock() else { return Vec::new() };
        let lines: Vec<String> = s.wanted.scoped().filter_map(|(k, _)| String::from_utf8(k).ok()).collect();
        s.wanted = Keyed::new();
        lines
    }

    /// One call answered at the carrier's present boundary.
    pub fn answer(&self, line: &str, lines: Vec<String>) {
        let Ok(mut s) = self.slot.lock() else { return };
        let at = s.now.boundary;
        s.answers.insert(line.as_bytes(), (at, lines));
        self.woken.notify_all();
    }
}

/// The waker a call left for the process's winding sends on.
pub fn wake_with(wake: std::sync::mpsc::Sender<()>) {
    PROCESS.wake_with(wake);
}

/// THE PROCESS'S WINDING CARRIER BEGINS A STROKE at the mind's K and
/// height `h`.
pub fn winding_began(k: u64, h: Option<u64>) {
    PROCESS.winding_began(k, h);
}

/// THE PROCESS'S WINDING CARRIER PARKS (`true`) OR WAKES.
pub fn park(parked: bool) {
    PROCESS.park(parked);
}

/// THE PROCESS'S WINDING CARRIER ENTERS `law`.
pub fn enter(law: &'static str) {
    PROCESS.enter(law);
}

/// A CARRIER BESIDE THE PROCESS'S WINDING (`name`) ENTERS `law`.
pub fn enter_carrier(name: &'static str, law: &'static str) {
    PROCESS.enter_carrier(name, law);
}

/// Every other carrier of the process's mind, one line each.
pub fn carriers() -> Vec<String> {
    PROCESS.carriers()
}

/// Where the process's winding carrier stands.
pub fn now() -> Now {
    PROCESS.now()
}

/// The calls waiting for the process's boundary, taken.
pub fn wanted() -> Vec<String> {
    PROCESS.wanted()
}

/// One call answered at the process's carrier's present boundary.
pub fn answer(line: &str, lines: Vec<String>) {
    PROCESS.answer(line, lines);
}

/// The carrier's standing as one line.
fn standing(n: &Now) -> String {
    let h = match n.h {
        Some(h) => format!("h {h}"),
        None => "no height carried".to_string(),
    };
    format!(
        "winding carrier: in {} · boundary {} (winding {} began at boundary {}) · K {} · {h}",
        match n.law {
            "" => "no law yet",
            l => l,
        },
        n.boundary,
        n.stroke,
        n.winding_boundary,
        n.k
    )
}

/// A CALL, ASKED: left for the winding carrier and waited on, a park at a
/// time (`physics::idle_park`, law 9: the pace is strokes), FOR AS LONG AS
/// THE CARRIER WORKS — moving between laws, or inside one (ruled
/// 2026-10-24: a hearing of ~2 s left every call unanswered when the asker
/// gave up after one park). Only a park in which the carrier stood PARKED
/// and passed no boundary ends the wait, answered with where it stands and
/// the last answer the call had, said so; the ask itself wakes a parked
/// mind. A wedged carrier is bounded by the asker's own process, never by
/// a clock here.
pub fn call(line: &str) -> Vec<String> {
    PROCESS.call(line)
}

/// A CALL LEFT for the process's winding carrier's next boundary.
pub fn ask(line: &str) -> Option<u64> {
    PROCESS.ask(line)
}

/// The call's answer as the process's observer holds it.
pub fn read(line: &str, asked: u64) -> Vec<String> {
    PROCESS.read(line, asked)
}

impl Live {
    /// A CALL, ASKED of this observer's carrier and waited on, as [`call`]
    /// waits on the process's.
    pub fn call(&self, line: &str) -> Vec<String> {
        match self.ask(line) {
            Some(asked) => self.wait(line, asked),
            None => vec!["the observer's slot is poisoned".to_string()],
        }
    }

    /// A CALL LEFT at boundary `asked`, waited on a park at a time while
    /// the carrier works, as [`call`] waits.
    pub fn wait(&self, line: &str, asked: u64) -> Vec<String> {
        let Ok(mut s) = self.slot.lock() else { return vec!["the observer's slot is poisoned".to_string()] };
        let fresh = |s: &Slot| s.answers.get(line.as_bytes()).is_some_and(|(at, _)| *at > asked);
        let mut seen = s.now.boundary;
        while !fresh(&s) {
            s = match self.woken.wait_timeout(s, crate::physics::idle_park(None)) {
                Ok((g, _)) => g,
                Err(_) => return vec!["the observer's slot is poisoned".to_string()],
            };
            if fresh(&s) || (s.now.parked && s.now.boundary == seen) {
                break;
            }
            seen = s.now.boundary;
        }
        drop(s);
        self.read(line, asked)
    }

    /// A CALL LEFT for the winding carrier's next boundary: answers the
    /// boundary it was asked at, `None` where the slot does not read.
    pub fn ask(&self, line: &str) -> Option<u64> {
        let Ok(mut s) = self.slot.lock() else { return None };
        s.wanted.insert(line.as_bytes(), ());
        self.waiting.store(true, Ordering::Relaxed);
        if let Ok(w) = self.wake.lock() {
            if let Some(w) = w.as_ref() {
                let _ = w.send(());
            }
        }
        Some(s.now.boundary)
    }

    /// THE CALL'S ANSWER as it stands, against the boundary it was `asked`
    /// at: the carrier's standing, then the answer — fresh, or the last one,
    /// said so.
    pub fn read(&self, line: &str, asked: u64) -> Vec<String> {
        let Ok(s) = self.slot.lock() else { return vec!["the observer's slot is poisoned".to_string()] };
        let mut out = vec![standing(&s.now)];
        out.extend(self.carriers());
        out.extend(answered(&s, line, asked));
        out
    }
}

/// The lines a call's answer reads as, against the boundary it was `asked`
/// at.
fn answered(s: &Slot, line: &str, asked: u64) -> Vec<String> {
    let mut out = Vec::new();
    match s.answers.get(line.as_bytes()) {
        Some((at, lines)) if *at > asked => {
            out.push(format!("{line} — answered at boundary {at}"));
            out.extend(lines.iter().cloned());
        }
        Some((at, lines)) => {
            out.push(format!(
                "{line} — NOT ANSWERED: the carrier passed no boundary in a park, held in the law above; the call waits for its next boundary (ask again); the last answer, at boundary {at}:"
            ));
            out.extend(lines.iter().cloned());
        }
        None => out.push(format!(
            "{line} — NOT ANSWERED: the carrier passed no boundary in a park, held in the law above; the call waits for its next boundary (ask again)"
        )),
    }
    out
}

/// `None` where the line passes its four bytes (STD1 3c).
pub fn encode_call(line: &str) -> Option<Vec<u8>> {
    let mut out = vec![ACT_OBSERVE, MODE_CALL];
    crate::transducer::address::put_bytes(&mut out, line.as_bytes())?;
    Some(out)
}

fn take_u32(b: &[u8], p: &mut usize) -> Option<usize> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    usize::try_from(v).ok()
}

/// The call's line, or `None` for a body that is not one whole call.
pub fn decode_call(b: &[u8]) -> Option<String> {
    let [ACT_OBSERVE, MODE_CALL, rest @ ..] = b else { return None };
    let mut p = 0usize;
    let len = take_u32(rest, &mut p)?;
    let line = rest.get(p..p.checked_add(len)?)?;
    (p + len == rest.len()).then(|| String::from_utf8(line.to_vec()).ok()).flatten()
}

/// The answer's lines, as many as fit `budget` bytes; a line past it is
/// said by the count of what was left out, never by silence.
pub fn encode_lines(lines: &[String], budget: usize) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    let mut n = 0u32;
    for (i, l) in lines.iter().enumerate() {
        let mut one = Vec::new();
        crate::transducer::address::put_bytes(&mut one, l.as_bytes())?;
        // Room kept for the line that says what was cut, were it cut here.
        let cut_at = |at: usize| -> Option<Vec<u8>> {
            let mut c = Vec::new();
            crate::transducer::address::put_bytes(&mut c, format!("… {} lines past the envelope (ask with `from {at}`)", lines.len() - at).as_bytes())?;
            Some(c)
        };
        let cut = cut_at(i)?;
        let room = match i + 1 < lines.len() {
            true => cut_at(i + 1)?.len(),
            false => 0,
        };
        if 4 + body.len() + one.len() + room > budget {
            body.extend_from_slice(&cut);
            n += 1;
            break;
        }
        body.extend_from_slice(&one);
        n += 1;
    }
    let mut out = Vec::with_capacity(4 + body.len());
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(&body);
    Some(out)
}

/// The inverse of [`encode_lines`].
pub fn decode_lines(b: &[u8]) -> Option<Vec<String>> {
    let mut p = 0usize;
    let n = take_u32(b, &mut p)?;
    if n > b.len() / 4 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let len = take_u32(b, &mut p)?;
        out.push(String::from_utf8(b.get(p..p.checked_add(len)?)?.to_vec()).ok()?);
        p += len;
    }
    (p == b.len()).then_some(out)
}
