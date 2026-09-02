// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TASK REGISTRY — the hypha's authoritative record of USEFUL work
//! (PoUW integrity, operator-ruled). Without an authority deciding what
//! counts, a node ingests garbage and calls it work, or replays a solved
//! puzzle and harvests continuous yield — the economics collapse into an
//! extractive puzzle mill. Here:
//!
//! - Every task is IMMUTABLY identified by its boundary root digest:
//!   task_id = BLAKE3(⊕ train coboundaries ‖ test inputs). Content is
//!   identity; provenance cannot be forged and duplicates cannot be re-fed
//!   under new names.
//! - THE ANTI-REPLAY YIELD COLLAPSE: the FIRST court-verifiable solve
//!   captures the full bounty; every subsequent submission of the same
//!   task earns only the asymptotic verification floor — real once,
//!   maintenance thereafter. Solving what is solved is not useful work.
//! - The registry is Vec-of-relations (Law II), states form the closed
//!   lifecycle Unsolved → Settled(n).

use crate::arc::{refine_delta, Grid, Task};

/// The closed task lifecycle. Settled carries the settle count — the decay's
/// exact denominator, never a boolean "done".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Unsolved,
    Settled(u32),
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
    tasks: Vec<([u8; 32], Task, TaskState)>,
    /// Dispatch cursor (rotates so pull requests spread across the book).
    cursor: usize,
}

/// The boundary-root digest: identity from CONTENT — the train coboundaries
/// (refined, so frames are part of identity) and the test inputs.
pub fn task_id(task: &Task) -> [u8; 32] {
    let mut buf = Vec::new();
    for (a, b) in &task.train {
        if let Some((chain, _, frame)) = refine_delta(a, b) {
            buf.extend_from_slice(&(frame.0 as u32).to_le_bytes());
            buf.extend_from_slice(&(frame.2 as u32).to_le_bytes());
            for (r, c, x, y) in chain {
                buf.extend_from_slice(&r.to_le_bytes());
                buf.extend_from_slice(&c.to_le_bytes());
                buf.push(x);
                buf.push(y);
            }
        }
    }
    for (x, _) in &task.test {
        for row in x {
            buf.extend_from_slice(row);
            buf.push(0xFF); // row separator — shape is identity too
        }
        buf.push(0xFE);
    }
    *blake3::hash(&buf).as_bytes()
}

/// The task's boundary tension — the per-lane volume of its train
/// coboundaries: what the full bounty prices.
pub fn task_tension(task: &Task) -> [u64; 4] {
    let mut mu = [0u64; 4];
    for (a, b) in &task.train {
        if let Some((chain, _, _)) = refine_delta(a, b) {
            mu[2] = mu[2].saturating_add(chain.len() as u64);
        }
    }
    mu
}

impl TaskRegistry {
    pub fn new() -> Self {
        TaskRegistry { tasks: Vec::new(), cursor: 0 }
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

    /// REGISTER a task (idempotent by content — re-feeding under a new file
    /// name registers nothing). Returns the id.
    pub fn register(&mut self, task: Task) -> [u8; 32] {
        let id = task_id(&task);
        if !self.tasks.iter().any(|(t, _, _)| *t == id) {
            self.tasks.push((id, task, TaskState::Unsolved));
        }
        id
    }

    /// DISPATCH a work envelope: the next Unsolved task, rotating so pulls
    /// spread across the book. None when everything settled.
    pub fn dispatch(&mut self) -> Option<([u8; 32], &Task)> {
        let n = self.tasks.len();
        for k in 0..n {
            let i = (self.cursor + k) % n;
            if self.tasks[i].2 == TaskState::Unsolved {
                self.cursor = (i + 1) % n;
                let (id, task, _) = &self.tasks[i];
                return Some((*id, task));
            }
        }
        None
    }

    /// VERIFY-AND-SETTLE a submission: predictions must match the held test
    /// outputs EXACTLY (the server is the authority — it holds the answers
    /// for its registered corpus). First verified solve → full bounty;
    /// replays → the verification floor; wrong or unknown → nothing.
    pub fn settle(&mut self, id: &[u8; 32], predictions: &[Grid]) -> SolveYield {
        let Some(entry) = self.tasks.iter_mut().find(|(t, _, _)| t == id) else {
            return SolveYield::Nothing; // no authoritative origin — rejected
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
                *n = n.saturating_add(1);
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
        let id = reg.register(toy_task());
        assert_eq!(reg.register(toy_task()), id, "content is identity — no re-feeding");
        assert_eq!(reg.len(), 1);
        let wrong = vec![vec![vec![9u8, 9], vec![9, 9]]];
        assert_eq!(reg.settle(&id, &wrong), SolveYield::Nothing, "wrong earns nothing");
        let right = vec![vec![vec![0u8, 0], vec![2, 2]]];
        match reg.settle(&id, &right) {
            SolveYield::Bounty(mu) => assert!(mu[2] > 0, "the first solve prices the tension"),
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

    /// Dispatch rotates over the unsolved book and dries up when settled.
    #[test]
    fn dispatch_rotates_and_dries_up() {
        let mut reg = TaskRegistry::new();
        let id = reg.register(toy_task());
        let (d1, _) = reg.dispatch().expect("work exists");
        assert_eq!(d1, id);
        let right = vec![vec![vec![0u8, 0], vec![2, 2]]];
        assert!(matches!(reg.settle(&id, &right), SolveYield::Bounty(_)));
        assert!(reg.dispatch().is_none(), "a settled book dispatches nothing");
    }
}
