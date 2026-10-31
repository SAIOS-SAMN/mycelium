// THE DISPATCH OBSERVATION (2026-10-22) — every court task that
// leaves a court, by its whole citation: the voice it last went to, the
// court's height when it left, and how often it has left. Written where the
// door answers a voice's pull; read only at a question, and never read back
// by the court, so it touches nothing in consensus.

use crate::network::nerve::Keyed;
use std::sync::{LazyLock, Mutex};

/// ONE TASK AS IT LEFT THIS COURT: its citation and the voice it last went
/// to, each whole; the court's height at that dispatch; and its dispatches
/// since boot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sent {
    pub citation: [u8; 32],
    pub voice: [u8; 32],
    pub height: u64,
    pub times: u64,
}

/// THE TASKS THIS COURT DISPATCHED since boot, one per citation, in citation
/// order — the corpus the tasks' bound.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dispatched {
    pub sent: Vec<Sent>,
}

static DISPATCHED: LazyLock<Mutex<Keyed<Sent>>> = LazyLock::new(|| Mutex::new(Keyed::new()));

/// ONE DISPATCH, where the door answered `voice`'s pull with the task under
/// `citation`, at the court's `height`.
pub fn note_dispatch(citation: [u8; 32], voice: [u8; 32], height: u64) {
    let Ok(mut d) = DISPATCHED.lock() else { return };
    // A count that cannot pass its carrier is not invented: the dispatch is
    // not recorded (rule 11).
    let Some(times) = d.get(&citation).map_or(0, |s| s.times).checked_add(1) else { return };
    d.insert(citation, Sent { citation, voice, height, times });
}

/// The tasks this court dispatched.
pub fn dispatched() -> Dispatched {
    match DISPATCHED.lock() {
        Ok(d) => Dispatched { sent: d.scoped().map(|(_, s)| *s).collect() },
        Err(_) => Dispatched::default(),
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

impl Dispatched {
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        crate::transducer::address::put_u32_count(out, self.sent.len())?;
        for s in &self.sent {
            out.extend_from_slice(&s.citation);
            out.extend_from_slice(&s.voice);
            out.extend_from_slice(&s.height.to_le_bytes());
            out.extend_from_slice(&s.times.to_le_bytes());
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Dispatched> {
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each dispatch is two keys and two counts.
        if n > b.len().checked_sub(*p)? / 80 {
            return None;
        }
        let mut sent = Vec::with_capacity(n);
        for _ in 0..n {
            let citation: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            let voice: [u8; 32] = b.get(*p + 32..*p + 64)?.try_into().ok()?;
            let height = u64::from_le_bytes(b.get(*p + 64..*p + 72)?.try_into().ok()?);
            let times = u64::from_le_bytes(b.get(*p + 72..*p + 80)?.try_into().ok()?);
            *p += 80;
            sent.push(Sent { citation, voice, height, times });
        }
        Some(Dispatched { sent })
    }

    /// The reading as the console prints it: one line per task, its citation
    /// whole.
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![format!("tasks dispatched {}", self.sent.len())];
        out.extend(
            self.sent
                .iter()
                .map(|s| format!("  task {} → voice {} at height {} · dispatched {}", hex(&s.citation), hex(&s.voice[..4]), s.height, s.times)),
        );
        out
    }
}
