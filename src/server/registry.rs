// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — THE TASK REGISTRY: the hypha's record of useful work. A task is
//! named by the listing identity of its train pairs and test inputs; the
//! first verified solve takes the bounty and every replay the floor.

use crate::arc::{refine_delta, Grid, Task};
use crate::network::nerve::Keyed;
use crate::transducer::address::Address;

/// The closed task lifecycle. Settled carries the settle count — the decay's
/// exact denominator, never a boolean "done".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Unsolved,
    /// A count's carrier is its event domain (STD1): `u64`, once per verified
    /// solve. It was a `u32`, never measured against that domain.
    Settled(u64),
}

/// A CLOSURE RECORDED: the task's first, or a closure of a task already
/// closed (verified again, and paid nothing — the decay is the discharge
/// itself, never a floor).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Closure {
    First,
    Again,
}

pub struct TaskRegistry {
    /// (task_id, the task, state) — append-only, content-addressed.
    tasks: Vec<(Address, Task, TaskState)>,
    /// Position of each id in `tasks`. Dispatch still walks the Vec;
    /// lookup, overlay, and settle read here.
    by_id: Keyed<usize>,
    /// 78.146, wire v1.2 — each task's CITATION (`citation`), the 32 bytes
    /// that name it on every wire, to its position. Filled at register.
    by_citation: Keyed<usize>,
    /// Dispatch cursor (rotates so pull requests spread across the book).
    cursor: usize,
    /// 78.153c — dispatches since the cursor's record was last taken
    /// (`cursor_due`): the record is written once a page of them, never per
    /// task.
    dispatched: u64,
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

/// 78.146, WIRE v1.2 — A TASK'S CITATION: the citation hash
/// (`chronometry::payload_hash`, blake3) of its full content address. A
/// task is named by these 32 bytes on every wire — ACT_TASK, ACT_SOLVE, a
/// seat's kind-4 record — and the long address stays court-side, so no
/// length wall applies to a task's name. A citation locates; the task is
/// its content (law 2).
pub fn citation(id: &Address) -> [u8; 32] {
    crate::chronometry::payload_hash(id.as_bytes())
}

// `task_tension` (the train's coboundary volume, the old first-solve
// bounty) is retired: a solve is priced by what its rule discharges on the
// withheld pairs (`Kernel::judge_solve`), never by the size of the train.

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
            by_citation: Keyed::new(),
            cursor: 0,
            dispatched: 0,
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
            self.by_citation.insert(citation(&id), i);
        }
        Some(id)
    }

    /// 78.146 — THE TASK A CITATION NAMES: its content address, the address
    /// every court-side reading takes. `None` where this book holds none.
    pub fn by_citation(&self, cite: &[u8; 32]) -> Option<Address> {
        let &i = self.by_citation.get(cite)?;
        self.tasks.get(i).map(|(id, _, _)| id.clone())
    }

    /// MU-CLIENT §5.4 (E1, finding 8) — EVERY FAMILY THIS BOOK HOLDS, by its
    /// citation: the name a court binds a family under and a mind speaks a
    /// scene by. The book is the one carrier of its tasks (law 12), so a
    /// court restored from its snapshot reads its families here at boot,
    /// never from a copy kept beside them.
    pub fn families(&self) -> impl Iterator<Item = ([u8; 32], &[(Grid, Grid)])> {
        self.tasks.iter().map(|(id, task, _)| (citation(id), task.train.as_slice()))
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
                // One dispatch, toward the cursor's next record; a u64
                // counts one per pull and cannot pass its carrier.
                if let Some(d) = self.dispatched.checked_add(1) {
                    self.dispatched = d;
                }
                let (id, task, _) = &self.tasks[i];
                return Some((id.clone(), task));
            }
        }
        None
    }

    /// SNAPSHOT the settlement states — `CUR3 [cursor u32][n u32][(citation
    /// 32)(settled u8)(count u64)]·n`. A task is named by its CITATION (wire
    /// v1.2), the 32 bytes every reading names it by: `CUR2` wrote each
    /// task's whole content address behind a u16, and 12 of the 1 000
    /// training tasks' addresses pass that wall (185 000 bytes the widest),
    /// so the snapshot refused whole and NO COURT EVER PERSISTED A SETTLED
    /// TASK (read 2026-10-20: no `corpora/registry` on any court). The count
    /// is a `u64` since STD1, and the header says so: a snapshot of another
    /// layout is refused whole, never read at the wrong width. The TASKS
    /// re-register from corpus at boot (content is identity, so citations
    /// are stable across restarts); only the STATES need disk. Without this,
    /// a hypha restart resets Settled → Unsolved and a crash-loop becomes a
    /// replay-bounty mint.
    ///
    /// `None` where a cursor or the task count passes its field (STD1 3c):
    /// no snapshot is written rather than a short one.
    pub fn snapshot(&self) -> Option<Vec<u8>> {
        use crate::transducer::address::put_u32_count;
        let mut out = Vec::with_capacity(8 + self.tasks.len() * 41);
        // The dispatch cursor leads: a restart continues the corpus from
        // where it stood instead of re-dispatching its head.
        out.extend_from_slice(b"CUR3");
        put_u32_count(&mut out, self.cursor)?;
        put_u32_count(&mut out, self.tasks.len())?;
        for (id, _, state) in &self.tasks {
            out.extend_from_slice(&citation(id));
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
        // Another layout (no header, `CUR1` with its `u32` counts, `CUR2`
        // naming tasks by address) is refused whole: read at this width it
        // would restore wrong states.
        if bytes.get(0..4) != Some(b"CUR3") {
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
            let Some(cite) = bytes.get(p..p + 32) else { break };
            p += 32;
            let (Some(&settled), Some(count)) = (
                bytes.get(p),
                bytes.get(p + 1..p + 9).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes),
            ) else {
                break;
            };
            p += 9;
            if settled == 1 {
                if let Some(&i) = self.by_citation.get(cite) {
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

    /// 78.153c — THE DISPATCH CURSOR'S RECORD: `CURS [cursor u32][tasks
    /// u32]`, 12 bytes, kept beside the snapshot (`corpora/cursor`). The
    /// snapshot carries the cursor too, but it is written only on a first
    /// closure, so a book nothing closed re-dispatched its head after every
    /// restart (client@3 pulled 75 times over 22 tasks, 2026-10-09). `None`
    /// where a count passes its field.
    pub fn cursor_record(&self) -> Option<[u8; 12]> {
        let mut out = [0u8; 12];
        out[..4].copy_from_slice(b"CURS");
        out[4..8].copy_from_slice(&u32::try_from(self.cursor).ok()?.to_le_bytes());
        out[8..].copy_from_slice(&u32::try_from(self.tasks.len()).ok()?.to_le_bytes());
        Some(out)
    }

    /// The cursor's record where a PAGE of dispatches (`page`) has passed
    /// since it was last taken, and the count begun again; `None` before.
    /// The caller writes it (tmp + rename): one write a page, never one a
    /// task.
    pub fn cursor_due(&mut self, page: u64) -> Option<[u8; 12]> {
        if self.dispatched < page.max(1) {
            return None;
        }
        let record = self.cursor_record()?;
        self.dispatched = 0;
        Some(record)
    }

    /// RESTORE the dispatch cursor from its record (call after the corpus
    /// registered). A record of another book — another task count — names
    /// other positions and is refused whole, as is any other layout: `false`,
    /// and the cursor stands where it was.
    pub fn restore_cursor(&mut self, bytes: &[u8]) -> bool {
        let word = |at: usize| bytes.get(at..at + 4).and_then(|b| <[u8; 4]>::try_from(b).ok()).map(u32::from_le_bytes);
        if bytes.len() != 12 || bytes.get(..4) != Some(b"CURS") {
            return false;
        }
        let (Some(cursor), Some(n)) = (word(4), word(8)) else { return false };
        let (Ok(cursor), Ok(n)) = (usize::try_from(cursor), usize::try_from(n)) else { return false };
        if n != self.tasks.len() || cursor >= n {
            return false;
        }
        self.cursor = cursor;
        true
    }

    /// IS THIS TASK STILL OPEN — would a closure now be its first? `None`
    /// where this book holds no such task (no authoritative origin).
    pub fn open(&self, id: &[u8]) -> Option<bool> {
        let &i = self.by_id.get(id)?;
        self.tasks.get(i).map(|(_, _, s)| *s == TaskState::Unsolved)
    }

    /// RECORD A CLOSURE THE COURT RULED (the actions' physics, 2026-09-29).
    /// The registry never judges: the court replayed the solve's rule on the
    /// withheld pairs (`Kernel::judge_solve`) and paid it before this is
    /// asked, so a closure is recorded only after its payment and none is
    /// lost between the two. The first closes the task; later ones are
    /// counted. `None` where this book holds no such task.
    pub fn close(&mut self, id: &[u8]) -> Option<Closure> {
        let &i = self.by_id.get(id)?;
        let entry = self.tasks.get_mut(i)?;
        Some(match &mut entry.2 {
            TaskState::Unsolved => {
                entry.2 = TaskState::Settled(1);
                Closure::First
            }
            TaskState::Settled(n) => {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per verified solve: a u64 cannot pass 2⁶⁴ events")]
                let next = n.saturating_add(1);
                *n = next;
                Closure::Again
            }
        })
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

    /// THE REGISTRY RECORDS, NEVER JUDGES: identity is content
    /// (re-registration is idempotent); the first ruled closure closes the
    /// task, later ones are counted again; a task this book does not hold has
    /// no closure.
    #[test]
    fn the_first_closure_closes_the_task_and_later_ones_are_again() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        assert_eq!(
            reg.register(toy_task()).expect("a grid pair's chain is A5's"),
            id,
            "content is identity — no re-feeding"
        );
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.open(&id), Some(true), "open before any closure");
        assert_eq!(reg.close(&id), Some(Closure::First), "the first ruled closure");
        assert_eq!(reg.open(&id), Some(false));
        assert_eq!(reg.close(&id), Some(Closure::Again), "a closure of a closed task");
        assert_eq!(reg.close(&[9u8; 32]), None, "no origin, no closure");
        assert_eq!(reg.unsolved(), 0);
    }

    /// A restart must not resurrect settled bounties: snapshot → fresh
    /// registry (corpus re-registered) → overlay → the replay still
    /// collapses to the floor.
    #[test]
    fn settled_states_survive_the_restart_overlay() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        assert_eq!(reg.close(&id), Some(Closure::First));
        let snap = reg.snapshot().expect("the snapshot writes");

        let mut reborn = TaskRegistry::new();
        reborn.register(toy_task()); // corpus re-registration at boot
        assert_eq!(reborn.overlay(&snap), 1, "one settled state restored");
        assert_eq!(reborn.close(&id), Some(Closure::Again), "the restart must not re-open the task");
        assert_eq!(reborn.overlay(&[]), 0, "garbage snapshot restores nothing");
        // STD1 — an older layout, with `u32` counts, is refused whole.
        let mut older = snap.clone();
        older[3] = b'1';
        let mut again = TaskRegistry::new();
        again.register(toy_task());
        assert_eq!(again.overlay(&older), 0, "a CUR1 snapshot is not read at the u64 width");
    }

    /// A TASK WIDER THAN THE U16 WALL IS SETTLED ACROSS A RESTART (observed on
    /// the development deployment, 2026-10-20: 12 of the 1 000 training tasks have content
    /// addresses past 65 535 bytes, and the address-keyed snapshot refused
    /// the whole book, so no court ever kept a settled task). Named by its
    /// citation, it snapshots, restores, and stays closed.
    #[test]
    fn a_task_wider_than_the_wall_stays_settled_across_a_restart() {
        // Many pairs of busy frames: each pair's listing key is written
        // into the task's address, so the address grows with them.
        let busy = |s: u8| -> Grid { (0..30).map(|r| (0..30).map(|c| u8::try_from((r * 7 + c * 3 + usize::from(s)) % 9).expect("a residue mod 9") + 1).collect()).collect() };
        let wide = Task { train: (0..12u8).map(|s| (busy(s), busy(s + 1))).collect(), test: vec![(busy(40), Vec::new())] };
        let mut reg = TaskRegistry::new();
        reg.register(toy_task());
        let id = reg.register(wide.clone()).expect("the wide task has an identity");
        assert!(id.as_bytes().len() > usize::from(u16::MAX), "the fixture passes the wall: {} bytes", id.as_bytes().len());
        assert_eq!(reg.close(&id), Some(Closure::First));
        let snap = reg.snapshot().expect("a book holding a task wider than the wall still snapshots");
        let mut reborn = TaskRegistry::new();
        reborn.register(toy_task());
        reborn.register(wide);
        assert_eq!(reborn.overlay(&snap), 1, "its settled state restored, by citation");
        assert_eq!(reborn.close(&id), Some(Closure::Again), "and the restart does not re-open it");
    }

    /// Dispatch rotates over the unsolved book and dries up when settled.
    #[test]
    fn dispatch_rotates_and_dries_up() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
        let (d1, _) = reg.dispatch().expect("work exists");
        assert_eq!(d1, id);
        assert_eq!(reg.close(&id), Some(Closure::First));
        assert!(reg.dispatch().is_none(), "a settled book dispatches nothing");
    }

    /// 78.153c — THE DISPATCH CURSOR SURVIVES A RESTART WITH NOTHING CLOSED:
    /// its record is due once a page of dispatches passed, and a registry
    /// re-registered from the corpus and restored from it dispatches the task
    /// after the last one served, never the head. A record of another book is
    /// refused whole.
    #[test]
    fn the_dispatch_cursor_survives_a_restart_with_nothing_closed() {
        let other = Task {
            train: vec![(vec![vec![3, 0], vec![0, 0]], vec![vec![0, 3], vec![0, 0]])],
            test: vec![(vec![vec![0, 0], vec![3, 0]], vec![vec![0, 0], vec![0, 3]])],
        };
        let third = Task {
            train: vec![(vec![vec![4, 4, 0]], vec![vec![0, 4, 4]])],
            test: vec![(vec![vec![0, 4, 4]], vec![vec![4, 4, 0]])],
        };
        let book = || {
            let mut reg = TaskRegistry::new();
            let ids: Vec<Address> = [toy_task(), other.clone(), third.clone()]
                .into_iter()
                .map(|t| reg.register(t).expect("a grid pair's chain is A5's"))
                .collect();
            (reg, ids)
        };
        let (mut reg, ids) = book();
        assert_eq!(ids.len(), 3);
        assert_eq!(reg.dispatch().map(|(id, _)| id), Some(ids[0].clone()));
        assert_eq!(reg.cursor_due(2), None, "one dispatch is not a page of two");
        assert_eq!(reg.dispatch().map(|(id, _)| id), Some(ids[1].clone()));
        let record = reg.cursor_due(2).expect("a page of two dispatches passed");
        assert_eq!(&record[..4], b"CURS");
        assert_eq!(record[4..8], 2u32.to_le_bytes(), "the cursor stands on the third task");
        assert_eq!(record[8..], 3u32.to_le_bytes(), "of a book of three");
        assert_eq!(reg.cursor_due(2), None, "the count begins again");
        assert!(reg.snapshot().is_some() && reg.settled_ids().is_empty(), "nothing closed");

        // THE RESTART: the corpus re-registered, the record restored.
        let (mut reborn, _) = book();
        assert!(reborn.restore_cursor(&record), "the record restores");
        assert_eq!(reborn.dispatch().map(|(id, _)| id), Some(ids[2].clone()), "the next task, not the head");
        let (mut fresh, _) = book();
        assert_eq!(fresh.dispatch().map(|(id, _)| id), Some(ids[0].clone()), "with no record, the head");

        // ANOTHER BOOK'S RECORD, OR ANOTHER LAYOUT, RESTORES NOTHING.
        let mut small = TaskRegistry::new();
        small.register(toy_task());
        assert!(!small.restore_cursor(&record), "a book of one is not a book of three");
        assert!(!small.restore_cursor(&record[..8]), "a short record is refused whole");
        let mut wrong = record;
        wrong[0] = b'X';
        let (mut again, _) = book();
        assert!(!again.restore_cursor(&wrong), "another layout is refused");
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
