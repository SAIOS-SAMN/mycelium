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
    /// Replayed clean but DISCHARGED NOTHING on the withheld pairs: no work,
    /// so no closure. Nothing is released, minted, docketed or offered, and
    /// the task's first closure is not spent (the operator, 2026-10-08:
    /// "we shouldn't be giving away mu incorrectly for the possibility of
    /// falsification").
    NoWork,
    /// 78.153 — NOT JUDGED YET: the act's one stroke ran out before the
    /// replay ruled. Nothing is refused, released or paid; where the replay
    /// stopped is kept (`replay_cursors`, by the rule and the withheld
    /// pairs), and the same solve asked again resumes it there. The wire
    /// carries it as `SOLVE_UNREAD` (3): a retry, never the final
    /// `SOLVE_NOTHING`.
    Unread,
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
            // 78.153 — a family that changes under a replay's cursor would
            // resume a replay on pairs it did not start on: every cursor goes
            // (they are volatile, and each replay starts over). A first
            // binding is a change too: a replay begun with no family passed
            // the family's phases unread.
            if self.task_family.get(task.as_ref()) != Some(&train) {
                self.replay_cursors = crate::network::nerve::Keyed::new();
            }
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
    ///
    /// 78.153 — RUN TO ITS END, for a reader off the door's one stroke (the
    /// client's seam, the tests): the act asked again while each asking
    /// moves its replay, exactly as a client asking again would see it. A
    /// replay no stroke moves stays `Unread`. The door judges on its one
    /// stroke, [`Kernel::judge_solve_on_stroke`].
    pub fn judge_solve(&mut self, rule: &Element, withheld: &crate::transducer::family::Family) -> SolveVerdict {
        let key = solve_key(rule, withheld);
        loop {
            let before = self.replay_cursors.get(&key).map(|c| c.progress());
            match self.judge_solve_on_stroke(rule, withheld) {
                SolveVerdict::Unread if self.replay_cursors.get(&key).map(|c| c.progress()) != before => {}
                verdict => return verdict,
            }
        }
    }

    /// 78.153 — THE SOLVE ON THE ACT'S ONE STROKE, the door's: `Unread`
    /// where the stroke ran out before the replay ruled, its cursor kept by
    /// the rule and the withheld pairs, so the same solve asked again
    /// resumes.
    pub fn judge_solve_on_stroke(&mut self, rule: &Element, withheld: &crate::transducer::family::Family) -> SolveVerdict {
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
            SolveVerdict::NoWork => Some("no work"),
            // Not judged: no act happened to count; the replays the court's
            // stroke ran out on are `court_unread`.
            SolveVerdict::Unread => None,
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
        // 78.153 — the replay on the act's one stroke, resumed where the last
        // asking of this rule on these pairs stopped.
        let key = solve_key(rule, withheld);
        let mut cursor = match self.replay_cursors.remove(&key) {
            Some(c) => c,
            None => crate::verdict::ReplayCursor::default(),
        };
        let out = {
            let court: &Kernel = self;
            crate::verdict::replay_resume(
                rule,
                &|a| court.resolve_form_ref(a),
                Some(withheld),
                court,
                &mut budget,
                &mut cursor,
                true,
            )
        };
        match out {
            crate::verdict::Replayed::Refused(refusal) => return SolveVerdict::Refused(refusal),
            crate::verdict::Replayed::Unread => {
                self.court_unread += 1;
                self.replay_cursors.insert(&key, cursor);
                return SolveVerdict::Unread;
            }
            crate::verdict::Replayed::Clean => {}
        }
        // A CLEAN REPLAY THAT DISCHARGES NOTHING IS NO CLOSURE. It was
        // answered Solved with a zero pay, and the zero was checked only
        // after `pay_solve` had released every escrow bound to the task and
        // the door had spent the task's first closure on it.
        let pay = self.measured_replay(rule, withheld);
        if pay.iter().all(|&x| x == 0) {
            return SolveVerdict::NoWork;
        }
        SolveVerdict::Solved { pay }
    }

    /// A SOLVE PAID (POUW, the actions' physics). The rule, verdict clean, is
    /// docketed and offered. Every listing bound to the task that holds an
    /// escrow is RELEASED TO THE VOICES ITS ESCROW NAMES — the solve
    /// confirmed their claim — at a Settled act citing that listing
    /// (`mint_claim`, split as the escrow's consilience says), paid the
    /// solve's measured discharge capped lane-wise by the escrow (78.149
    /// (b), `release_discharged`); never to the
    /// solver for another voice's claim. The solver is minted what its rule
    /// discharged on the withheld pairs (`pay`), on the task's FIRST closure
    /// only: a repeat is verified and pays nothing — no floor, the decay is
    /// the discharge itself. Answers what the solver was paid, per lane.
    pub fn pay_solve(&mut self, task: &[u8], voice: [u8; 32], rule: &Element, pay: [u64; 4], first: bool) -> [u64; 4] {
        // NO WORK, NO ACT: the check stands before every release, mint,
        // docket and offer (see `SolveVerdict::NoWork`).
        if pay.iter().all(|&x| x == 0) {
            return [0; 4];
        }
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
            // 78.149 (b): the claim is paid the CLOSER'S measured discharge
            // on the withheld pairs, `pay`, lane-wise capped by what its
            // escrow held; nothing above the discharge is minted.
            let _ = self.treasury.release_discharged(addr.as_bytes(), pay, settled);
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

/// 78.153 — WHERE A SOLVE'S REPLAY IS KEPT: the rule's address and the
/// withheld pairs written out, each grid's rows length-delimited (A21), so
/// one rule asked on other pairs is another replay.
fn solve_key(rule: &Element, withheld: &crate::transducer::family::Family) -> Vec<u8> {
    let mut key = Vec::new();
    crate::transducer::address::put_key_bytes(&mut key, rule.address().as_bytes());
    crate::transducer::address::put_key_bytes(&mut key, b"withheld");
    for (x, y) in withheld.iter() {
        for g in [x, y] {
            let mut rows = Vec::new();
            for row in g {
                crate::transducer::address::put_key_bytes(&mut rows, row);
            }
            crate::transducer::address::put_key_bytes(&mut key, &rows);
        }
    }
    key
}
