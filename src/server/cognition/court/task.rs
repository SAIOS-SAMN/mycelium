// 21, 26 — THE TASK: a listing bound to a corpus family, and what a solve
// against it pays.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

/// WHAT THE COURT READ OF A SOLVE (the actions' physics).
#[derive(Clone, Debug)]
pub enum SolveVerdict {
    /// Reconstructed clean on every withheld pair; `pay` what it discharged there
    /// as μ, each closed pair's share of its enclosure (78.170 O1, ruled
    /// 2026-11-01), and `modes` the modes the closing form moved on the pairs
    /// it closed (78.162), each exact: S boosts, G hue modes, T null fluxes,
    /// H turns.
    Solved { pay: crate::economy::Mu, modes: crate::economy::modes::Modes },
    /// Refused by the reconstruction, with its reason, coordinate and kind (78.9).
    Refused(crate::verdict::Refusal),
    /// The solve carried no rule: a raster is no proof of a motion.
    NotARule,
    /// The task holds no withheld answer to reconstruct against.
    Unverifiable,
    /// Reconstructed clean but DISCHARGED NOTHING on the withheld pairs: no work,
    /// so no closure. Nothing is released, minted, docketed or offered, and
    /// the task's first closure is not spent (ruled 2026-10-08: μ is never
    /// paid out for the bare possibility of falsification).
    NoWork,
    /// 78.153 — NOT JUDGED YET: the act's one stroke ran out before the
    /// reconstruction ruled. Nothing is refused, released or paid; where the reconstruction
    /// stopped is kept (`reconstruction_cursors`, by the rule and the withheld
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
            // 78.153 — a family that changes under a reconstruction's cursor would
            // resume a reconstruction on pairs it did not start on: THAT TASK'S cursors
            // go (`reconstruction_key` ends with the task). A first binding is a
            // change too: a reconstruction begun with no family passed the family's
            // phases unread. Every other cursor stands — a solve's is keyed by
            // its rule and the withheld pairs it was given, never by a task's
            // binding. Dropping them all on each task listed restarted every
            // resumed solve from nothing, and the court answered UNREAD for
            // ever: 30 of 36 solve verdicts on the development deployment (2026-10-10).
            if self.task_family.get(task.as_ref()) != Some(&train) {
                let mut tail = Vec::new();
                crate::transducer::address::put_key_bytes(&mut tail, task.as_ref());
                let stale: Vec<Vec<u8>> =
                    self.reconstruction_cursors.scoped().map(|(k, _)| k).filter(|k| k.ends_with(&tail)).collect();
                for k in stale {
                    self.reconstruction_cursors.remove(&k);
                }
            }
            self.task_family.insert(task, train);
        }
    }

    /// THE SOLVE'S VERDICT, BY ITS ACTION (the actions' physics, 2026-09-29).
    /// A solve is a RULE, and the court reconstructs it on the task's withheld
    /// pairs as it reconstructs any sentence on its family (`reconstruct_blind_at`
    /// with the family: its derivation re-derived, every pair closed — step
    /// 7, `REFUSE_NOT_CLOSED`); the rule's cited forms resolve through this
    /// court's ruled cover. Clean, the rule is priced by what it
    /// discharges on those pairs, per lane (`measured_discharge`, the deflation
    /// paid only where a pair closes); refused, the refusal names its
    /// reason, coordinate and obstruction kind (78.9). Samples are never
    /// compared: a raster that matches is no proof of a motion.
    ///
    /// 78.153 — RUN TO ITS END, for a reader off the door's one stroke (the
    /// client's seam, the tests): the act asked again while each asking
    /// moves its reconstruction, exactly as a client asking again would see it. A
    /// reconstruction no stroke moves stays `Unread`. The door judges on its one
    /// stroke, [`Court::judge_solve_on_stroke`].
    pub fn judge_solve(&mut self, rule: &Element, withheld: &crate::transducer::family::Family) -> SolveVerdict {
        let key = solve_key(rule, withheld);
        loop {
            let before = self.reconstruction_cursors.get(&key).map(|c| c.progress());
            match self.judge_solve_on_stroke(rule, withheld) {
                SolveVerdict::Unread if self.reconstruction_cursors.get(&key).map(|c| c.progress()) != before => {}
                verdict => return verdict,
            }
        }
    }

    /// 78.153 — THE SOLVE ON THE ACT'S ONE STROKE, the door's: `Unread`
    /// where the stroke ran out before the reconstruction ruled, its cursor kept by
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

    /// 78.153c — A VERDICT THE DOOR GAVE BEFORE ANY JUDGING (unsigned, an
    /// inner seat, no such task, no rule carried), counted on its slot of
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
        // 78.153 — the reconstruction on the act's one stroke, resumed where the last
        // asking of this rule on these pairs stopped.
        let key = solve_key(rule, withheld);
        let mut cursor = match self.reconstruction_cursors.remove(&key) {
            Some(c) => c,
            None => crate::verdict::ReconstructionCursor::default(),
        };
        let out = {
            let court: &Court = self;
            crate::verdict::reconstruct_resume(
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
            crate::verdict::ReconstructionOutcome::Refused(refusal) => return SolveVerdict::Refused(refusal),
            crate::verdict::ReconstructionOutcome::Unread => {
                self.court_unread += 1;
                self.reconstruction_cursors.insert(&key, cursor);
                return SolveVerdict::Unread;
            }
            crate::verdict::ReconstructionOutcome::Clean => {}
        }
        // A CLEAN RECONSTRUCTION THAT DISCHARGES NOTHING IS NO CLOSURE. It was
        // answered Solved with a zero pay, and the zero was checked only
        // after `pay_solve` had released every escrow bound to the task and
        // the door had spent the task's first closure on it.
        let (pay, modes) = self.measured_discharge_read(rule, withheld);
        if pay.is_zero() {
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
    pub fn pay_solve(&mut self, task: &[u8], voice: [u8; 32], rule: &Element, pay: crate::economy::Mu, first: bool) -> crate::economy::Mu {
        // NO WORK, NO ACT: the check stands before every release, mint,
        // docket and offer (see `SolveVerdict::NoWork`).
        if pay.is_zero() {
            return crate::economy::Mu::zero();
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
            // POWV: this court's reconstruction on the withheld pairs verified it.
            let organs = [self.identity];
            let _ = self.release_verified(addr.as_bytes(), pay.clone(), &organs, settled);
        }
        // MU-ECONOMY §5.2 — a verified solve of a leased job pays its lease:
        // the solve's measured discharge, out of the payer's hold.
        if let Ok(job) = <[u8; 32]>::try_from(task) {
            let at = self.anchor();
            let _ = self.pay_lease(voice, &job, pay.clone(), at);
        }
        // MU-ECONOMY §3.4, F2b — ONE CLOSURE, ONE POINT: a point this court's
        // cones already hold, sealed here or on a line it admitted, is closed;
        // its closure here is judged again and mints nothing.
        let rule_address = rule.address();
        let first = first && self.point_held(task, rule_address.as_bytes()).is_none();
        self.acts.judge(if first { crate::observe::Judged::FirstClosure } else { crate::observe::Judged::ClosureAgain });
        if !first || pay.is_zero() {
            return crate::economy::Mu::zero();
        }
        // The settling act seals the point, so every court that back-fills
        // its receipt holds it (`Cite::Closure`).
        let cite = crate::chronometry::Cite::Closure { task: task.to_vec(), rule: rule_address.as_bytes().to_vec() };
        let mu = self.measure_of(task);
        let Some(sealed) = self.seal_step(crate::chronometry::ActKind::Settled, mu, voice, |_| vec![cite.clone()]) else { return crate::economy::Mu::zero() };
        self.note_points(&sealed);
        let settled = self.anchor();
        let paid = self.treasury.settle_solve(voice, pay, settled);
        self.acts.paid.step(&paid);
        self.grow_grants_by_paid();
        paid
    }

    /// 78.155c AS AMENDED (MU-CLIENT §5.6, E4) — WHAT THE COURTS PAID GROWS
    /// THE GRANT. Every POUW and POWC payment since the last take (a solve's,
    /// a word's, a conjugate's, a sentence's, a claim's, a demand closed by
    /// another mind) is counted toward the next growth grant of the universe
    /// whose key was paid — membership, never tier (78.165). Where it was
    /// only first solves, a universe whose minds earned by words and
    /// sentences no task named never grew. The sum is exact.
    pub(crate) fn grow_grants_by_paid(&mut self) {
        let paid = self.treasury.take_paid_out();
        let mut moved = false;
        for (who, mu) in paid {
            let Some(domain) = self.tier_domain.get(&who).copied() else { continue };
            let now = match self.closed_since_grant.get(&domain) {
                Some(h) => h.add(&mu),
                None => mu.clone(),
            };
            self.closed_since_grant.insert(domain, now);
            self.acts.pay("mints counted toward a grant", mu);
            moved = true;
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
            paid: self.acts.paid.clone(),
            colours: self.basis.scoped().filter(|(_, w)| w.is_colour_word()).count() as u64,
            colours_derived: self.acts.colours_derived,
            deferred: self.deferred_reading(),
            provisional: self.acts.provisional,
            pays: self.acts.pays,
            pays_lanes: self.acts.pays_lanes.clone(),
        }
    }

}

/// 78.153 — WHERE A SOLVE'S RECONSTRUCTION IS KEPT: the rule's address and the
/// withheld pairs written out, each sample's content written whole (A21:
/// its extents, place and symbols, `Sample::put_content`), so one rule
/// asked on other pairs is another reconstruction.
fn solve_key(rule: &Element, withheld: &crate::transducer::family::Family) -> Vec<u8> {
    let mut key = Vec::new();
    crate::transducer::address::put_key_bytes(&mut key, rule.address().as_bytes());
    crate::transducer::address::put_key_bytes(&mut key, b"withheld");
    for (x, y) in withheld.iter() {
        for g in [x, y] {
            let mut content = Vec::new();
            g.sample().put_content(&mut content);
            crate::transducer::address::put_key_bytes(&mut key, &content);
        }
    }
    key
}

/// WHY THE COURT DOES NOT RECONSTRUCT UNDER A CLAIM'S STATED GROUND
/// (MU-SENSING §3.4, §6; track 26 S2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatedGroundRefusal {
    /// The stated ground is not one symbol at the task's width.
    NotOneSymbol,
    /// The stated ground is no candidate of the task's ground: it fails
    /// one of the three conditions on the task's train pairs.
    NoCandidate,
}

impl StatedGroundRefusal {
    /// The refusal's reason as the court names it, on the solve and the
    /// speech alike (track 26 S5).
    pub fn reason(self) -> &'static str {
        match self {
            StatedGroundRefusal::NotOneSymbol => crate::court::verdict::REFUSE_GROUND_NOT_ONE_SYMBOL,
            StatedGroundRefusal::NoCandidate => crate::court::verdict::REFUSE_GROUND_NO_CANDIDATE,
        }
    }
}

/// THE GROUND A CLAIM IS RECONSTRUCTED UNDER: the ground the claim states,
/// read against the task as a candidate (`Ground::candidacies`, the three
/// conditions of MU-SENSING §3.4 on the task's train pairs), or refused by
/// name. A claim that states none is reconstructed under no ground
/// (`Ground::None`). The court verifies what the claim states; it never
/// deduces, chooses or searches for a ground of its own: the reading of
/// the stated symbol against the task's conditions is the reconstruction
/// of the claim. Condition 2 reads the bodies around it (whether a body
/// covering a seam is a membrane), and no other symbol is ever adopted as
/// the ground.
pub fn stated_ground(
    train: &crate::transducer::family::Family,
    stated: Option<&[u8]>,
) -> Result<crate::transducer::family::transduce::Ground<u8>, StatedGroundRefusal> {
    use crate::transducer::family::transduce::Ground;
    let Some(bytes) = stated else { return Ok(Ground::None) };
    let &[g] = bytes else { return Err(StatedGroundRefusal::NotOneSymbol) };
    let pairs: Vec<_> = train.iter().map(|(x, y)| (x.sample().clone(), y.sample().clone())).collect();
    stated_ground_of(&pairs, Some(bytes)).map(|_| Ground::One(g))
}

/// THE STATED GROUND READ AT ANY RANK (track 26 S4c): as [`stated_ground`],
/// on the posted samples of the task's train pairs. The stated ground is
/// one symbol at the samples' own width, or refused `NotOneSymbol`; read as
/// a candidate by the three conditions, or refused `NoCandidate`. A family
/// with no pair presents no candidate.
pub fn stated_ground_of(
    train: &[(crate::transducer::sample::Sample, crate::transducer::sample::Sample)],
    stated: Option<&[u8]>,
) -> Result<crate::transducer::family::transduce::Ground<Vec<u8>>, StatedGroundRefusal> {
    use crate::transducer::family::transduce::{Candidacy, Ground};
    let Some(bytes) = stated else { return Ok(Ground::None) };
    if train.is_empty() {
        return Err(StatedGroundRefusal::NoCandidate);
    }
    if train.iter().flat_map(|(x, y)| [x, y]).any(|s| s.width() != bytes.len()) {
        return Err(StatedGroundRefusal::NotOneSymbol);
    }
    match Ground::candidacies(train).into_iter().find(|(s, _)| s.as_slice() == bytes) {
        Some((_, Candidacy::Candidate)) => Ok(Ground::One(bytes.to_vec())),
        _ => Err(StatedGroundRefusal::NoCandidate),
    }
}
