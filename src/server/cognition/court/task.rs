// 21, 26 — THE TASK: a listing bound to a corpus family, and what a solve
// against it pays.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

/// WHAT THE COURT READ OF A SOLVE (the actions' physics).
#[derive(Clone, Debug)]
pub enum SolveVerdict {
    /// Replayed clean on every withheld pair; `pay` what it discharged there,
    /// and `modes` the modes the closing form moved on the pairs it closed
    /// (78.162), each exact: S boosts, G hue modes, T null fluxes, H turns.
    Solved { pay: [u64; 4], modes: crate::economy::modes::Modes },
    /// Refused by the replay, with its reason, coordinate and kind (78.9).
    Refused(crate::verdict::Refusal),
    /// The solve carried no rule: a raster is no proof of a motion.
    NotARule,
    /// The task holds no withheld answer to replay against.
    Unverifiable,
    /// Replayed clean but DISCHARGED NOTHING on the withheld pairs: no work,
    /// so no closure. Nothing is released, minted, docketed or offered, and
    /// the task's first closure is not spent (ruled 2026-10-08: μ is never
    /// paid out for the bare possibility of falsification).
    NoWork,
    /// 78.153 — NOT JUDGED YET: the act's one stroke ran out before the
    /// replay ruled. Nothing is refused, released or paid; where the replay
    /// stopped is kept (`replay_cursors`, by the rule and the withheld
    /// pairs), and the same solve asked again resumes it there. The wire
    /// carries it as `SOLVE_UNREAD` (3): a retry, never the final
    /// `SOLVE_NOTHING`.
    Unread,
}

impl SolveVerdict {
    /// 78.153c — THE SLOT THIS VERDICT IS COUNTED ON where it is read: every
    /// verdict but `Solved`, which is counted first or again where it is
    /// paid (`pay_solve`). `Unread` is counted too — each asking the stroke
    /// ran out on — beside `court_unread`.
    pub fn judged(&self) -> Option<crate::observe::Judged> {
        use crate::observe::Judged;
        match self {
            SolveVerdict::Solved { .. } => None,
            SolveVerdict::Refused(r) => Some(Judged::Refused(r.obstruction.kind())),
            SolveVerdict::NotARule => Some(Judged::NotARule),
            SolveVerdict::Unverifiable => Some(Judged::Unverifiable),
            SolveVerdict::NoWork => Some(Judged::NoWork),
            SolveVerdict::Unread => Some(Judged::Unread),
        }
    }
}

impl Court {
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
            // resume a replay on pairs it did not start on: THAT TASK'S cursors
            // go (`replay_key` ends with the task). A first binding is a
            // change too: a replay begun with no family passed the family's
            // phases unread. Every other cursor stands — a solve's is keyed by
            // its rule and the withheld pairs it was given, never by a task's
            // binding. Dropping them all on each task listed restarted every
            // resumed solve from nothing, and the court answered UNREAD for
            // ever: 30 of 36 solve verdicts on the development deployment (2026-10-10).
            if self.task_family.get(task.as_ref()) != Some(&train) {
                let mut tail = Vec::new();
                crate::transducer::address::put_key_bytes(&mut tail, task.as_ref());
                let stale: Vec<Vec<u8>> =
                    self.replay_cursors.scoped().map(|(k, _)| k).filter(|k| k.ends_with(&tail)).collect();
                for k in stale {
                    self.replay_cursors.remove(&k);
                }
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
    /// stroke, [`Court::judge_solve_on_stroke`].
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
        let verdict = self.judge_solve_read(rule, withheld);
        // THE ACT, COUNTED WHERE IT HAPPENED: the verdict, a refusal by kind
        // (78.9, `ObstructionKind::ALL`'s order), each on its typed slot
        // (78.153c: "no work" was a name the table lacked, and dropped). A
        // solved verdict is counted first or again where it is paid
        // (`pay_solve`).
        if let Some(j) = verdict.judged() {
            self.acts.judge(j);
        }
        verdict
    }

    /// 78.153c — A VERDICT THE DOOR GAVE BEFORE ANY JUDGING (unsigned, not
    /// an ancient, no such task, no rule carried), counted on its slot of
    /// this court's acts.
    pub fn note_judged(&mut self, verdict: crate::observe::Judged) {
        self.acts.judge(verdict);
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
            let court: &Court = self;
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
        let (pay, modes) = self.measured_replay_read(rule, withheld);
        if pay.iter().all(|&x| x == 0) {
            return SolveVerdict::NoWork;
        }
        SolveVerdict::Solved { pay, modes }
    }

    /// A SOLVE PAID (POUW, the actions' physics). The rule, verdict clean, is
    /// docketed and offered. Every listing bound to the task that holds an
    /// escrow is RELEASED TO THE VOICES ITS ESCROW NAMES — the solve
    /// confirmed the voices' claim — at a Settled act citing that listing
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
            // POWV: this court's replay on the withheld pairs verified it.
            let organs = [self.identity];
            let _ = self.release_verified(addr.as_bytes(), pay, &organs, settled);
        }
        // MU-ECONOMY §5.2 — a verified solve of a leased job pays its lease:
        // the solve's measured discharge, out of the payer's hold.
        if let Ok(job) = <[u8; 32]>::try_from(task) {
            let at = self.anchor();
            let _ = self.pay_lease(voice, &job, pay, at);
        }
        self.acts.judge(if first { crate::observe::Judged::FirstClosure } else { crate::observe::Judged::ClosureAgain });
        if !first || pay.iter().all(|&x| x == 0) {
            return [0; 4];
        }
        self.append_step(crate::chronometry::ActKind::Settled, task, voice);
        let settled = self.anchor();
        let paid = self.treasury.settle_solve(voice, pay, settled);
        if let Some(total) = crate::core::lanes::lanes_add(self.acts.paid, paid) {
            self.acts.paid = total;
        }
        self.grow_grants_by_paid();
        paid
    }

    /// 78.155c AS AMENDED (MU-CLIENT §5.6, E4) — WHAT THE COURTS PAID GROWS
    /// THE GRANT. Every POUW and POWC payment since the last take (a solve's,
    /// a word's, a conjugate's, a sentence's, a claim's, a demand closed by
    /// another mind) is counted toward the next growth grant of the universe
    /// whose key was paid — membership, never tier (78.165). Where it was
    /// only first solves, a universe whose minds earned by words and
    /// sentences no task named never grew. A sum past its carrier is refused
    /// and counts nothing there rather than a capped amount (rule 11).
    pub(crate) fn grow_grants_by_paid(&mut self) {
        let paid = self.treasury.take_paid_out();
        let mut moved = false;
        for (who, mu) in paid {
            let Some(domain) = self.tier_domain.get(&who).copied() else { continue };
            let held = match self.closed_since_grant.get(&domain) {
                Some(h) => *h,
                None => [0; 4],
            };
            if let Some(now) = crate::core::lanes::lanes_add(held, mu) {
                self.closed_since_grant.insert(domain, now);
                self.acts.pay("mints counted toward a grant", mu);
                moved = true;
            }
        }
        if moved {
            self.mark_persist();
        }
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
            pays: self.acts.pays,
            pays_lanes: self.acts.pays_lanes,
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
