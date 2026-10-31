// THE JUDGED-SOLVE OBSERVATION (ruled 2026-10-24) — every solve a court
// judged, by the task's whole citation and the voice that sent it: the last
// verdict and the court's height then, and how many of each verdict. Written
// where the door answers the solve; read only at a question, and never read
// back by the court, so it touches nothing in consensus. The registry keeps
// no solver and no time, and a closure again is spoken to no peer: without
// this, which voice closed what, and when, was read off peer journals.

use crate::network::nerve::Keyed;
use std::sync::{LazyLock, Mutex};

/// The verdicts a solve is answered with, each at its wire code's slot of
/// [`JudgedSolve::times`] (`dialogue::SOLVE_NOTHING` 0, `SOLVE_FLOOR` 1,
/// `SOLVE_BOUNTY` 2, `SOLVE_UNREAD` 3).
pub const VERDICTS: [&str; 4] = ["nothing", "again", "first", "unread"];

/// ONE TASK AS ONE VOICE'S SOLVES OF IT WERE JUDGED: the citation and the
/// voice, each whole; the last verdict's code and the court's height then;
/// and the count of each verdict since boot, by code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JudgedSolve {
    pub citation: [u8; 32],
    pub voice: [u8; 32],
    pub verdict: u8,
    pub height: u64,
    pub times: [u64; 4],
}

/// THE SOLVES THIS COURT JUDGED since boot, one per citation and voice, in
/// that order — the corpus and the voices the solves' bound.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JudgedSolves {
    pub solves: Vec<JudgedSolve>,
}

static JUDGED: LazyLock<Mutex<Keyed<JudgedSolve>>> = LazyLock::new(|| Mutex::new(Keyed::new()));

/// ONE SOLVE JUDGED: `voice`'s solve of the task under `citation`, answered
/// with `verdict` at the court's `height`. A code past the verdicts is no
/// verdict and is not recorded.
pub fn note_judged_solve(citation: [u8; 32], voice: [u8; 32], verdict: u8, height: u64) {
    let slot = usize::from(verdict);
    if slot >= VERDICTS.len() {
        return;
    }
    let mut key = citation.to_vec();
    key.extend_from_slice(&voice);
    let Ok(mut j) = JUDGED.lock() else { return };
    let mut times = match j.get(&key) {
        Some(s) => s.times,
        None => [0; 4],
    };
    // A count that cannot pass its carrier is not invented: the verdict is
    // not recorded (rule 11).
    let Some(t) = times.get_mut(slot) else { return };
    let Some(n) = t.checked_add(1) else { return };
    *t = n;
    j.insert(key, JudgedSolve { citation, voice, verdict, height, times });
}

/// The solves this court judged.
pub fn judged_solves() -> JudgedSolves {
    match JUDGED.lock() {
        Ok(j) => JudgedSolves { solves: j.scoped().map(|(_, s)| *s).collect() },
        Err(_) => JudgedSolves::default(),
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

impl JudgedSolves {
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        crate::transducer::address::put_u32_count(out, self.solves.len())?;
        for s in &self.solves {
            out.extend_from_slice(&s.citation);
            out.extend_from_slice(&s.voice);
            out.push(s.verdict);
            out.extend_from_slice(&s.height.to_le_bytes());
            for n in s.times {
                out.extend_from_slice(&n.to_le_bytes());
            }
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<JudgedSolves> {
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each record is two keys, a code, a height and four counts.
        if n > b.len().checked_sub(*p)? / 105 {
            return None;
        }
        let mut solves = Vec::with_capacity(n);
        for _ in 0..n {
            let citation: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            let voice: [u8; 32] = b.get(*p + 32..*p + 64)?.try_into().ok()?;
            let verdict = *b.get(*p + 64)?;
            let height = u64::from_le_bytes(b.get(*p + 65..*p + 73)?.try_into().ok()?);
            let mut times = [0u64; 4];
            for (i, t) in times.iter_mut().enumerate() {
                let at = *p + 73 + 8 * i;
                *t = u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?);
            }
            *p += 105;
            solves.push(JudgedSolve { citation, voice, verdict, height, times });
        }
        Some(JudgedSolves { solves })
    }

    /// The reading as the console prints it: the verdicts summed per voice,
    /// then one line per task and voice, the citation whole.
    pub fn lines(&self) -> Vec<String> {
        let mut by_voice: Keyed<[u64; 4]> = Keyed::new();
        for s in &self.solves {
            let mut sum = match by_voice.get(&s.voice) {
                Some(t) => *t,
                None => [0; 4],
            };
            for (acc, n) in sum.iter_mut().zip(s.times) {
                if let Some(v) = acc.checked_add(n) {
                    *acc = v;
                }
            }
            by_voice.insert(s.voice, sum);
        }
        let counts = |t: &[u64; 4]| VERDICTS.iter().zip(t).map(|(name, n)| format!("{name} {n}")).collect::<Vec<_>>().join(" · ");
        let mut out = vec![format!("solves judged {} (task, voice)", self.solves.len())];
        out.extend(by_voice.scoped().map(|(v, t)| format!("  voice {} · {}", hex(&v[..4.min(v.len())]), counts(t))));
        out.extend(self.solves.iter().map(|s| {
            let last = match VERDICTS.get(usize::from(s.verdict)) {
                Some(n) => *n,
                None => "?",
            };
            format!("  task {} ← voice {} · last {last} at height {} · {}", hex(&s.citation), hex(&s.voice[..4]), s.height, counts(&s.times))
        }));
        out
    }
}
