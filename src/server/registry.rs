// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — THE TASK REGISTRY: the hypha's record of useful work. A task is
//! named by the listing identity of its train pairs and test inputs; the
//! first verified solve takes the bounty and every replay the floor.

use crate::arc::{refine_delta, Grid, Task};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_addr, take_addr, Address};

/// The closed task lifecycle. Settled carries the settle count — the decay's
/// exact denominator, never a boolean "done".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Unsolved,
    /// A count's carrier is its event domain (STD1): `u64`, once per verified
    /// solve. It was a `u32`, never measured against that domain.
    Settled(u64),
}

/// What a submission earned — the decay made explicit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveYield {
    /// First verified solve: the full bounty (per-lane μ⃗ of the task's
    /// boundary tension).
    Bounty([u64; 4]),
    /// A replay of settled work: the verification floor — one unit on the
    /// residual lane, the attestation fee, nothing more.
    VerifyFloor,
    /// Wrong answer or unknown task: nothing (and the door fee already paid).
    Nothing,
}

pub struct TaskRegistry {
    /// (task_id, the task, state) — append-only, content-addressed.
    tasks: Vec<(Address, Task, TaskState)>,
    /// Position of each id in `tasks`. Dispatch still walks the Vec;
    /// lookup, overlay, and settle read here.
    by_id: Keyed<usize>,
    /// Dispatch cursor (rotates so pull requests spread across the book).
    cursor: usize,
    /// Cover-change cursor: a budgeted walk re-homes by the cover trie.
    cover_cursor: usize,
    /// First-time intake cursor: walks the manifest once, then stops.
    intake_cursor: usize,
    intake_done: bool,
}

/// The boundary-root digest: identity from CONTENT — the train coboundaries
/// (refined, so frames are part of identity) and the test inputs.
///
/// Cut A persisted registries keyed the old digest; those ids are
/// unreachable under this M2 form (listing keys of each train pair plus
/// the test-input chain address). Cut B recuts from genesis — overlay of
/// a Cut A snapshot is a miss, not a restore.
/// `None` where a chain of this task has no A5 form. A task whose members
/// cannot be addressed has no identity, and skipping one would give two
/// different tasks one id (law 2).
pub fn task_id(task: &Task) -> Option<Address> {
    let mut buf = vec![crate::transducer::address::TAG_TASK];
    for (a, b) in &task.train {
        if let Some(crate::arc::Refined { delta: chain, frame, .. }) = refine_delta(a, b) {
            crate::transducer::address::put_key_bytes(
                &mut buf,
                &crate::transducer::element::listing_key(&chain, &frame)?,
            );
        }
    }
    for (x, _) in &task.test {
        let chain: Vec<(i64, i64, u8, u8)> = x
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter_map(move |(c, &h)| {
                    (h != 0).then_some((r as i64, c as i64, h, h))
                })
            })
            .collect();
        crate::transducer::address::put_key_bytes(
            &mut buf,
            &crate::transducer::element::from_residual(&chain)?.address(),
        );
    }
    Some(Address::new(buf))
}

/// The task's boundary tension — the per-lane volume of its train
/// coboundaries, MEASURED across all four lanes (spatial/gauge/residual/
/// shell) so the bounty prices the axes the work actually occupies instead
/// of funneling every task onto the residual scalar.
pub fn task_tension(task: &Task) -> [u64; 4] {
    let mut mu = [0u64; 4];
    for (a, b) in &task.train {
        if let Some(crate::arc::Refined { delta: chain, .. }) = refine_delta(a, b) {
            let lanes = crate::transducer::element::measure_lanes(&chain);
            for l in 0..4 {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a sum of chain measures over resident chains cannot pass u64")]
                let m = mu[l].saturating_add(lanes[l]);
                mu[l] = m;
            }
        }
    }
    mu
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskRegistry {
    pub fn new() -> Self {
        TaskRegistry {
            tasks: Vec::new(),
            by_id: Keyed::new(),
            cursor: 0,
            cover_cursor: 0,
            intake_cursor: 0,
            intake_done: false,
        }
    }

    /// Next `n` tasks for first-time listing. Empty once the manifest has
    /// been walked; cover changes re-home through [`next_cover_batch`].
    pub fn next_intake_batch(&mut self, n: usize) -> Vec<(Address, Task)> {
        let len = self.tasks.len();
        if self.intake_done || len == 0 || n == 0 {
            return Vec::new();
        }
        // WHAT IS LEFT IS THE SEATS THEMSELVES, read from the cursor on: a
        // cursor past the manifest names no seats, and the manifest says so
        // rather than a difference of two counts standing in for it.
        let Some(rest) = self.tasks.get(self.intake_cursor..) else {
            self.intake_done = true;
            return Vec::new();
        };
        let take = n.min(rest.len());
        let mut out = Vec::with_capacity(take);
        for (id, task, _) in rest.iter().take(take) {
            out.push((id.clone(), task.clone()));
        }
        self.intake_cursor += take;
        if self.intake_cursor >= len {
            self.intake_done = true;
        }
        out
    }

    pub fn intake_done(&self) -> bool {
        self.intake_done || self.tasks.is_empty()
    }

    /// Next `n` tasks for a cover-change re-home stroke.
    pub fn next_cover_batch(&mut self, n: usize) -> Vec<(Address, Task)> {
        let len = self.tasks.len();
        if len == 0 || n == 0 {
            return Vec::new();
        }
        let take = n.min(len);
        let mut out = Vec::with_capacity(take);
        for _ in 0..take {
            let i = self.cover_cursor % len;
            self.cover_cursor = (self.cover_cursor + 1) % len;
            let (id, task, _) = &self.tasks[i];
            out.push((id.clone(), task.clone()));
        }
        out
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    pub fn unsolved(&self) -> usize {
        self.tasks.iter().filter(|(_, _, s)| *s == TaskState::Unsolved).count()
    }

    pub fn task(&self, id: &[u8]) -> Option<&Task> {
        self.by_id
            .get(id)
            .and_then(|&i| self.tasks.get(i).map(|(_, task, _)| task))
    }

    /// Withheld test pairs (input, held output) the home speaks a rule onto.
    pub fn withheld(&self, id: &[u8]) -> Vec<(Grid, Grid)> {
        // A task this book does not hold withholds nothing.
        match self.task(id) {
            Some(t) => t.test.iter().filter(|(_, y)| !y.is_empty()).cloned().collect(),
            None => Vec::new(),
        }
    }

    /// REGISTER a task (idempotent by content — re-feeding under a new file
    /// name registers nothing). Returns the id, or `None` where a member of
    /// the task has no A5 form and the task therefore has no identity.
    pub fn register(&mut self, task: Task) -> Option<Address> {
        let id = task_id(&task)?;
        if !self.by_id.contains(id.as_bytes()) {
            let i = self.tasks.len();
            self.tasks.push((id.clone(), task, TaskState::Unsolved));
            self.by_id.insert(&id, i);
        }
        Some(id)
    }

    /// DISPATCH a work envelope: the next Unsolved task, rotating so pulls
    /// spread across the book. None when everything settled.
    /// The tasks settled on this book — its proven roots.
    pub fn settled_ids(&self) -> Vec<Address> {
        self.tasks.iter().filter(|(_, _, s)| *s != TaskState::Unsolved).map(|(t, _, _)| t.clone()).collect()
    }

    /// PRIORITIZE a task: the next dispatch starts at it. A proven root
    /// gossiped from another hypha is re-spoken here first, so every voice
    /// on this book is asked to reproduce it. False when the task is not
    /// registered or already settled here.
    pub fn prioritize(&mut self, id: &[u8]) -> bool {
        match self.by_id.get(id) {
            Some(&i) if self.tasks.get(i).is_some_and(|(_, _, s)| *s == TaskState::Unsolved) => {
                self.cursor = i;
                true
            }
            _ => false,
        }
    }

    pub fn dispatch(&mut self) -> Option<(Address, &Task)> {
        let n = self.tasks.len();
        for k in 0..n {
            let i = (self.cursor + k) % n;
            if self.tasks[i].2 == TaskState::Unsolved {
                self.cursor = (i + 1) % n;
                let (id, task, _) = &self.tasks[i];
                return Some((id.clone(), task));
            }
        }
        None
    }

    /// SNAPSHOT the settlement states — `CUR2 [cursor u32][n u32][(id 32)
    /// (settled u8)(count u64)]·n`. The count is a `u64` since STD1, and the
    /// header says so: a snapshot without it is an older layout and is
    /// refused whole, never read at the wrong width. The TASKS re-register from corpus at boot (content is
    /// identity, so ids are stable across restarts); only the STATES need
    /// disk. Without this, a hypha restart resets Settled → Unsolved and a
    /// crash-loop becomes a replay-bounty mint.
    ///
    /// `None` where a cursor, the task count or a task's address passes its
    /// field (STD1 3c): no snapshot is written rather than a short one —
    /// the address's own refusal was dropped here.
    pub fn snapshot(&self) -> Option<Vec<u8>> {
        use crate::transducer::address::put_u32_count;
        let mut out = Vec::with_capacity(8 + self.tasks.len() * 41);
        // The dispatch cursor leads: a restart continues the corpus from
        // where it stood instead of re-dispatching its head.
        out.extend_from_slice(b"CUR2");
        put_u32_count(&mut out, self.cursor)?;
        put_u32_count(&mut out, self.tasks.len())?;
        for (id, _, state) in &self.tasks {
            put_addr(&mut out, id)?;
            match state {
                TaskState::Unsolved => {
                    out.push(0);
                    out.extend_from_slice(&0u64.to_le_bytes());
                }
                TaskState::Settled(n) => {
                    out.push(1);
                    out.extend_from_slice(&n.to_le_bytes());
                }
            }
        }
        out.extend_from_slice(b"INT1");
        put_u32_count(&mut out, self.intake_cursor)?;
        out.push(u8::from(self.intake_done));
        Some(out)
    }

    /// OVERLAY persisted states onto the registered book (call after corpus
    /// registration). Ids that no longer exist in the corpus are skipped —
    /// the corpus is the authority on WHAT exists; the snapshot only on what
    /// was already settled. Returns how many states were restored.
    pub fn overlay(&mut self, bytes: &[u8]) -> usize {
        // An older layout (no header, or `CUR1` with its `u32` counts) is
        // refused whole: read at this width it would restore wrong states.
        if bytes.get(0..4) != Some(b"CUR2") {
            return 0;
        }
        if let Some(c) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) {
            self.cursor = c % self.tasks.len().max(1);
        }
        let mut p = 8usize;
        let Some(n) = bytes.get(p..p + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize)
        else {
            return 0;
        };
        let mut restored = 0usize;
        p += 4;
        for _ in 0..n {
            let Some(id) = take_addr(bytes, &mut p) else { break };
            let (Some(&settled), Some(count)) = (
                bytes.get(p),
                bytes.get(p + 1..p + 9).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes),
            ) else {
                break;
            };
            p += 9;
            if settled == 1 {
                if let Some(&i) = self.by_id.get(id.as_bytes()) {
                    if let Some(entry) = self.tasks.get_mut(i) {
                        entry.2 = TaskState::Settled(count);
                        restored += 1;
                    }
                }
            }
        }
        if bytes.get(p..p + 4) == Some(b"INT1") {
            if let Some(c) = bytes.get(p + 4..p + 8).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize) {
                self.intake_cursor = c.min(self.tasks.len());
            }
            if let Some(&d) = bytes.get(p + 8) {
                self.intake_done = d != 0 || self.intake_cursor >= self.tasks.len();
            }
        }
        restored
    }

    /// VERIFY-AND-SETTLE a submission: predictions must match the held test
    /// outputs EXACTLY (the server is the authority — it holds the answers
    /// for its registered corpus). First verified solve → full bounty;
    /// replays → the verification floor; wrong or unknown → nothing.
    pub fn settle(&mut self, id: &[u8], predictions: &[Grid]) -> SolveYield {
        let Some(&i) = self.by_id.get(id) else {
            return SolveYield::Nothing; // no authoritative origin — rejected
        };
        let Some(entry) = self.tasks.get_mut(i) else {
            return SolveYield::Nothing;
        };
        let expected: Vec<&Grid> =
            entry.1.test.iter().filter(|(_, y)| !y.is_empty()).map(|(_, y)| y).collect();
        if expected.is_empty()
            || predictions.len() != expected.len()
            || predictions.iter().zip(&expected).any(|(p, e)| p != *e)
        {
            return SolveYield::Nothing;
        }
        match &mut entry.2 {
            TaskState::Unsolved => {
                let mu = task_tension(&entry.1);
                entry.2 = TaskState::Settled(1);
                SolveYield::Bounty(mu)
            }
            TaskState::Settled(n) => {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per verified solve: a u64 cannot pass 2⁶⁴ events")]
                let next = n.saturating_add(1);
                *n = next;
                SolveYield::VerifyFloor
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_task() -> Task {
        Task {
            train: vec![(vec![vec![1, 1], vec![0, 0]], vec![vec![2, 2], vec![0, 0]])],
            test: vec![(vec![vec![0, 0], vec![1, 1]], vec![vec![0, 0], vec![2, 2]])],
        }
    }

    /// THE ANTI-REPLAY COLLAPSE: identity is content (re-registration is
    /// idempotent), the first verified solve takes the bounty, every replay
    /// takes only the verification floor, and a wrong answer takes nothing.
    #[test]
    fn first_solve_earns_the_bounty_and_replays_collapse_to_the_floor() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        assert_eq!(
            reg.register(toy_task()).expect("a grid pair's chain is A5's"),
            id,
            "content is identity — no re-feeding"
        );
        assert_eq!(reg.len(), 1);
        let wrong = vec![vec![vec![9u8, 9], vec![9, 9]]];
        assert_eq!(reg.settle(&id, &wrong), SolveYield::Nothing, "wrong earns nothing");
        let right = vec![vec![vec![0u8, 0], vec![2, 2]]];
        match reg.settle(&id, &right) {
            SolveYield::Bounty(mu) => assert!(mu.iter().any(|&m| m > 0), "the first solve prices the tension"),
            other => panic!("first verified solve must take the bounty, got {other:?}"),
        }
        assert_eq!(
            reg.settle(&id, &right),
            SolveYield::VerifyFloor,
            "the replay collapses to the maintenance floor"
        );
        assert_eq!(reg.settle(&[9u8; 32], &right), SolveYield::Nothing, "no origin, no yield");
        assert_eq!(reg.unsolved(), 0);
    }

    /// A restart must not resurrect settled bounties: snapshot → fresh
    /// registry (corpus re-registered) → overlay → the replay still
    /// collapses to the floor.
    #[test]
    fn settled_states_survive_the_restart_overlay() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        let right = vec![vec![vec![0u8, 0], vec![2, 2]]];
        assert!(matches!(reg.settle(&id, &right), SolveYield::Bounty(_)));
        let snap = reg.snapshot().expect("the snapshot writes");

        let mut reborn = TaskRegistry::new();
        reborn.register(toy_task()); // corpus re-registration at boot
        assert_eq!(reborn.overlay(&snap), 1, "one settled state restored");
        assert_eq!(
            reborn.settle(&id, &right),
            SolveYield::VerifyFloor,
            "the restart must not re-mint the bounty"
        );
        assert_eq!(reborn.overlay(&[]), 0, "garbage snapshot restores nothing");
        // STD1 — an older layout, with `u32` counts, is refused whole.
        let mut older = snap.clone();
        older[3] = b'1';
        let mut again = TaskRegistry::new();
        again.register(toy_task());
        assert_eq!(again.overlay(&older), 0, "a CUR1 snapshot is not read at the u64 width");
    }

    /// Dispatch rotates over the unsolved book and dries up when settled.
    #[test]
    fn dispatch_rotates_and_dries_up() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        let (d1, _) = reg.dispatch().expect("work exists");
        assert_eq!(d1, id);
        let right = vec![vec![vec![0u8, 0], vec![2, 2]]];
        assert!(matches!(reg.settle(&id, &right), SolveYield::Bounty(_)));
        assert!(reg.dispatch().is_none(), "a settled book dispatches nothing");
    }

    /// First-time intake walks the manifest once under a budget; further
    /// calls are empty. Cover-change batches still wrap.
    #[test]
    fn intake_exhausts_the_manifest_once() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        let first = reg.next_intake_batch(8);
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].0, id);
        assert!(reg.intake_done());
        assert!(reg.next_intake_batch(8).is_empty(), "the manifest is exhausted");
        assert_eq!(reg.next_cover_batch(1).len(), 1, "cover re-home still walks");
    }
}
