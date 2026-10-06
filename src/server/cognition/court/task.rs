// 21, 26 — THE TASK: a listing bound to a corpus family, and what a solve
// against it pays.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

/// WHAT THE COURT READ OF A SOLVE (the actions' physics).
#[derive(Clone, Debug)]
pub enum SolveVerdict {
    /// Replayed clean on every withheld pair; `pay` what it discharged there.
    Solved { pay: [u64; 4] },
    /// Refused by the replay, with its reason, coordinate and kind (78.9).
    Refused(crate::verdict::Refusal),
    /// The solve carried no rule: a raster is no proof of a motion.
    NotARule,
    /// The task holds no withheld answer to replay against.
    Unverifiable,
}

impl Kernel {
    pub fn bind_listing_task(&mut self, listing: impl AsRef<[u8]>, task: impl AsRef<[u8]>) {
        let listing = Address::from_bytes(listing);
        let task = Address::from_bytes(task);
        self.listing_task.insert(&listing, task.clone());
        match self.task_listings.get_mut(task.as_bytes()) {
            Some(v) => {
                if !v.iter().any(|a| a == &listing) {
                    v.push(listing);
                }
            }
            None => {
                self.task_listings.insert(&task, vec![listing]);
            }
        }
    }

    /// The trains a rule must speak to mint at install. Empty trains are ignored.
    pub fn bind_task_family(&mut self, task: impl AsRef<[u8]>, train: Vec<crate::transducer::family::TrainPair>) {
        if !train.is_empty() {
            self.task_family.insert(task, train);
        }
    }

    /// THE SOLVE'S VERDICT, BY ITS ACTION (the actions' physics, 2026-09-29).
    /// A solve is a RULE, and the court replays it on the task's withheld
    /// pairs as it replays any sentence on its family (`blind_replay_at`
    /// with the family: its derivation re-derived, every pair closed — step
    /// 7, `REFUSE_NOT_CLOSED`); the rule's cited forms resolve through this
    /// kernel's ruled cover. Clean, the rule is priced by what it
    /// discharges on those pairs, per lane (`measured_replay`, the deflation
    /// paid only where a pair closes); refused, the refusal names its
    /// reason, coordinate and obstruction kind (78.9). Grids are never
    /// compared: a raster that matches is no proof of a motion.
    pub fn judge_solve(&mut self, rule: &Element, withheld: &crate::transducer::family::Family) -> SolveVerdict {
        use crate::observe::{count, JUDGED};
        let verdict = self.judge_solve_read(rule, withheld);
        // THE ACT, COUNTED WHERE IT HAPPENED: the verdict, a refusal by kind
        // (78.9, `ObstructionKind::ALL`'s order). A solved verdict is
        // counted first or again where it is paid (`pay_solve`).
        let name = match &verdict {
            SolveVerdict::Solved { .. } => None,
            SolveVerdict::Refused(r) => Some(match r.obstruction.kind() {
                crate::transducer::sentence::ObstructionKind::P => "refused P",
                crate::transducer::sentence::ObstructionKind::D => "refused D",
                crate::transducer::sentence::ObstructionKind::C => "refused C",
                crate::transducer::sentence::ObstructionKind::I => "refused I",
                crate::transducer::sentence::ObstructionKind::N => "refused N",
                crate::transducer::sentence::ObstructionKind::Unclassified => "refused ?",
            }),
            SolveVerdict::NotARule => Some("not a rule"),
            SolveVerdict::Unverifiable => Some("unverifiable"),
        };
        if let Some(name) = name {
            count(&mut self.acts.judged, &JUDGED, name);
        }
        verdict
    }

    fn judge_solve_read(&mut self, rule: &Element, withheld: &crate::transducer::family::Family) -> SolveVerdict {
        if withheld.is_empty() {
            return SolveVerdict::Unverifiable;
        }
        if crate::transducer::sentence::from_element_form(rule).is_none() {
            return SolveVerdict::NotARule;
        }
        let mut budget = self.physics.stroke();
        let court: &Kernel = self;
        if let Some(refusal) = crate::verdict::blind_replay_at(
            rule,
            &|a| court.resolve_form(a),
            Some(withheld),
            court,
            &mut budget,
        ) {
            return SolveVerdict::Refused(refusal);
        }
        SolveVerdict::Solved { pay: self.measured_replay(rule, withheld) }
    }

    /// A SOLVE PAID (POUW, the actions' physics). The rule, verdict clean, is
    /// docketed and offered. Every listing bound to the task that holds an
    /// escrow is RELEASED TO THE VOICES ITS ESCROW NAMES — the solve
    /// confirmed their claim — at a Settled act citing that listing
    /// (`mint_claim`, split as the escrow's consilience says); never to the
    /// solver for another voice's claim. The solver is minted what its rule
    /// discharged on the withheld pairs (`pay`), on the task's FIRST closure
    /// only: a repeat is verified and pays nothing — no floor, the decay is
    /// the discharge itself. Answers what the solver was paid, per lane.
    pub fn pay_solve(&mut self, task: &[u8], voice: [u8; 32], rule: &Element, pay: [u64; 4], first: bool) -> [u64; 4] {
        self.docket_connection(rule, voice);
        self.enter_offered(rule.address().as_bytes());
        // A task with no listings bound to it has none: the reading of
        // absence on a keyed carrier.
        let listings: Vec<Address> = match self.task_listings.get(task) {
            Some(l) => l.clone(),
            None => Vec::new(),
        };
        for addr in &listings {
            if self.treasury.escrow_of(addr.as_bytes()).is_none() {
                continue;
            }
            self.append_step(crate::chronometry::ActKind::Settled, addr, voice);
            let settled = self.anchor();
            if let Some(crate::economy::escrow::Escrowed { voices, mu: held, .. }) = self.treasury.release_award(addr) {
                self.treasury.mint_claim(&voices, held, settled);
            }
        }
        let name = if first { "first closures" } else { "closures again" };
        crate::observe::count(&mut self.acts.judged, &crate::observe::JUDGED, name);
        if !first || pay.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.append_step(crate::chronometry::ActKind::Settled, task, voice);
        let settled = self.anchor();
        let paid = self.treasury.settle_solve(voice, pay, settled);
        if let Some(total) = crate::core::lanes::lanes_add(self.acts.paid, paid) {
            self.acts.paid = total;
        }
        paid
    }

    /// THIS COURT'S ACTS AND ITS METABOLISM, read at the question: the
    /// reserve as it stands and its own ledger, and the acts as counted.
    pub fn acts_reading(&self) -> crate::observe::Acts {
        let (drawn, restored, refused) = self.physics.reserve.ledger();
        crate::observe::Acts {
            k: self.chain().head_k(),
            reserve: self.physics.reserve.lanes(),
            drawn,
            restored,
            refused,
            syllabus: self.acts.syllabus,
            fetch: self.acts.fetch,
            judged: self.acts.judged,
            paid: self.acts.paid,
            colours: self.basis.scoped().filter(|(_, w)| w.is_colour_word()).count() as u64,
            colours_derived: self.acts.colours_derived,
            deferred: self.deferred_reading(),
            provisional: self.acts.provisional,
        }
    }

}
