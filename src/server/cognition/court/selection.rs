// A9, 26 — THE SOLVE'S A9 JUDGEMENT (MU-DERIVATIONS D3.3; ruled 2026-11-06,
// #158): a registered task's withheld pairs bound with its family, and π*
// bound on the rule a solve closes them with.

use crate::cognition::*;
use crate::court::verdict::{bind_pi, PiBinding, PiRefusal};
use crate::transducer::element::Element;
use crate::transducer::family::TrainPair;

/// A SOLVE JUDGED ON A REGISTERED TASK: the verdict, the rule as the court
/// judged it, and what the A9 judgement bound. Where the verdict is
/// `Solved`, `judged` carries the common set `pi` bound (its anchor, the
/// family it was derived from, judged on the task's withheld pairs) and
/// `pi` is read; on every other verdict `judged` is the rule as heard and
/// `pi` is `None`, since nothing was judged to bind. A refused binding
/// leaves the rule ⊤ and names why.
#[derive(Clone, Debug)]
pub struct TaskJudgement {
    pub verdict: SolveVerdict,
    pub judged: Element,
    pub pi: Option<Result<PiBinding, PiRefusal>>,
}

impl Court {
    /// BIND A TASK'S WITHHELD PAIRS as its registration fixed them
    /// (`TaskRegistry::withheld`, by coordinate): read by every judgement of
    /// a solve on the task and never chosen there. A solve's reconstruction
    /// cursor is keyed by the pairs' content, so none is dropped here.
    pub fn bind_task_withheld(&mut self, task: impl AsRef<[u8]>, withheld: Vec<TrainPair>) {
        if !withheld.is_empty() {
            self.task_withheld.insert(task, withheld);
        }
    }

    /// The withheld pairs bound for a task; empty where none is bound.
    pub fn withheld_of(&self, task: &[u8]) -> &[TrainPair] {
        match self.task_withheld.get(task) {
            Some(w) => w.as_slice(),
            None => &[],
        }
    }

    /// THE SOLVE ON A REGISTERED TASK, ON THE ACT'S ONE STROKE (#158): the
    /// rule reconstructed on the withheld pairs this court bound for `task`
    /// at registration ([`Court::judge_solve_on_stroke`]), never on pairs the
    /// asker or the act hands in. Where it is `Solved`, π* is bound at the
    /// same judgement (`bind_pi`): the rule's footprints on the task's
    /// family and on the withheld pairs, the common set judged over the
    /// pairs whose change the rule's footprint covers, metered on a stroke
    /// of its own. The judged rule carries what was bound.
    pub fn judge_task_solve_on_stroke(&mut self, rule: &Element, task: &[u8]) -> TaskJudgement {
        let withheld: Vec<TrainPair> = self.withheld_of(task).to_vec();
        let verdict = self.judge_solve_on_stroke(rule, &withheld);
        if !matches!(verdict, SolveVerdict::Solved { .. }) {
            return TaskJudgement { verdict, judged: rule.clone(), pi: None };
        }
        // `Solved` read the rule as a sentence, so its form decodes.
        let Some(form) = crate::transducer::sentence::from_element_form(rule) else {
            return TaskJudgement { verdict, judged: rule.clone(), pi: None };
        };
        let train: Vec<TrainPair> = match self.task_family.get(task) {
            Some(t) => t.clone(),
            None => Vec::new(),
        };
        let mut budget = self.physics.stroke();
        let pi = bind_pi(&form, &train, &withheld, &*self, &mut budget);
        let judged = match &pi {
            Ok(b) => rule.clone().with_anchor(b.terms.clone()),
            Err(_) => rule.clone(),
        };
        TaskJudgement { verdict, judged, pi: Some(pi) }
    }
}
