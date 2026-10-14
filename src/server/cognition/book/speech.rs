// 26 — SPEECH AGAINST THE BOOK: a voice standing sentences, a probe
// settling against an open listing, and a rewrite offered for one.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

/// 78.53, 78.55 — WHAT PROVES A COLOUR WORD AT THIS COURT
/// (`Kernel::colour_proof`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColourProof {
    /// A scene this court holds shows the relation: the task that did.
    Witnessed(Address),
    /// It composes exactly the two installed colour words it cites.
    Derived,
    Unproven,
}

impl ColourProof {
    /// Whether the word stands by this proof.
    pub fn stands(&self) -> bool {
        !matches!(self, ColourProof::Unproven)
    }
}

/// One adjudication counted on the observer by its verdict (CV5).
fn note_verdict_of(a: &crate::anastome::dialogue::Adjudication) {
    use crate::anastome::dialogue::Adjudication;
    use crate::observe::{note_verdict, Verdict};
    note_verdict(match a {
        Adjudication::Ratified { .. } => Verdict::Ratified,
        Adjudication::Stood { .. } => Verdict::Stood,
        Adjudication::Awaiting { .. } => Verdict::Awaiting,
        Adjudication::Refused { kind, .. } => Verdict::Refused(kind.to_byte()),
    });
}

impl Kernel {
    /// PERCEIVE A FAMILY'S SPEECH — the one door an utterance has (law 8,
    /// ruled 2026-09-10) — read as codes: `SPEECH_STOOD` for a sentence
    /// listed, `SPEECH_RATIFIED` for one already language, `SPEECH_REFUSED`
    /// for one the replay refused, and `SPEECH_DIALECT` on every sentence
    /// of an act the door deferred — stood there, unanswered here, spoken
    /// again later. The adjudications themselves are `hear_speech`.
    pub fn perceive_speech(&mut self, voice: [u8; 32], task: &[u8], sentences: &[Element]) -> Vec<u8> {
        let spoken: Vec<(Element, Option<crate::transducer::sentence::Contraction>)> =
            sentences.iter().map(|s| (s.clone(), None)).collect();
        // THE ACT'S ROOT (65.5): one stroke of this court's own metabolism
        // for the whole speech, threaded to every sentence it hears.
        //
        // 78.153 — a frame deferred on no lane is a replay its stroke could
        // not finish: the voice speaks it again, and this reading does, while
        // the frame's replays move on.
        let keys: Vec<Vec<u8>> = sentences
            .iter()
            .map(|s| crate::cognition::court::replay::replay_key(s, (!s.is_colour_word()).then_some(task)))
            .collect();
        let progress = |k: &Kernel| -> Vec<Option<(crate::verdict::Phase, usize, usize)>> {
            keys.iter().map(|key| k.replay_cursors.get(key).map(|c| c.progress())).collect()
        };
        loop {
            let before = progress(self);
            let mut budget = self.physics.stroke();
            match self.hear_speech(voice, task, &spoken, &mut budget) {
                Ok(adjudicated) => return adjudicated.iter().map(|a| a.code()).collect(),
                Err(lanes) if lanes == [0; 4] && progress(self) != before => {}
                Err(_) => return vec![crate::anastome::dialogue::SPEECH_DIALECT; sentences.len()],
            }
        }
    }

    /// A SPEECH FRAME AS THE JUDGE HANDS IT OVER: every tape decoded, a
    /// tape that does not parse adjudicated a `MalformedComplex` naming no
    /// address, and the rest heard in the order sent.
    pub fn hear_frame(
        &mut self,
        act: &crate::anastome::dialogue::SpeechAct,
    ) -> Result<Vec<crate::anastome::dialogue::Adjudication>, [u64; 4]> {
        // THE ACT'S ROOT (65.5): the wire hands the frame here, and one
        // stroke is the whole frame's draw, threaded to every sentence.
        let mut budget = self.physics.stroke();
        self.hear_frame_on(act, &mut budget)
    }

    /// [`Kernel::hear_frame`] on the stroke handed in (78.153: a fixture
    /// that asks what one frame hears hands it a stroke that holds it).
    pub fn hear_frame_on(
        &mut self,
        act: &crate::anastome::dialogue::SpeechAct,
        budget: &mut crate::physics::Budget,
    ) -> Result<Vec<crate::anastome::dialogue::Adjudication>, [u64; 4]> {
        use crate::anastome::dialogue::Adjudication;
        use crate::transducer::sentence::{At, RefusalKind};
        let mut spoken = Vec::with_capacity(act.sentences.len());
        let mut unparsed: Vec<usize> = Vec::new();
        for (i, tape) in act.sentences.iter().enumerate() {
            let mut p = 0usize;
            match Element::from_bytes(tape, &mut p) {
                Some(e) => spoken.push((e, act.contractions.get(i).cloned().flatten())),
                None => unparsed.push(i),
            }
        }
        let heard = self.hear_speech(act.voice, &act.task, &spoken, budget)?;
        let receipt = self.chain.head_hash();
        let mut out = Vec::with_capacity(act.sentences.len());
        let mut next = heard.into_iter();
        for i in 0..act.sentences.len() {
            if unparsed.contains(&i) {
                let refused = Adjudication::Refused {
                    kind: RefusalKind::MalformedComplex,
                    at: At::Tape(Address::new(Vec::new())),
                    receipt,
                };
                note_verdict_of(&refused);
                // CV1 — a tape that does not parse is integrity (78.9).
                crate::observe::note_obstruction(crate::transducer::sentence::ObstructionKind::I);
                out.push(refused);
                continue;
            }
            match next.next() {
                Some(a) => out.push(a),
                None => break,
            }
        }
        // 50.2 step 2, 26: the paths beside the tapes, read AFTER the
        // adjudications and only for a sentence that stood or was
        // ratified. A path is evidence for a USE of a generator, and a use
        // rests on a sentence the court admitted: read before the
        // adjudication, a refused sentence's generators were paid for a
        // form the court had just refused.
        let mut si = 0usize;
        for i in 0..act.sentences.len() {
            if unparsed.contains(&i) {
                continue;
            }
            let Some((e, _)) = spoken.get(si).cloned() else {
                break;
            };
            si += 1;
            // 78.102 — EVERY SENTENCE HEARD, closed or open, stood or not:
            // its rings re-derived and the letters they need offered.
            self.hear_open(&e, act.voice, budget);
            let stood = matches!(
                out.get(i),
                Some(Adjudication::Stood { .. }) | Some(Adjudication::Ratified { .. })
            );
            if !stood {
                continue;
            }
            // The citations a stood sentence carries are paid at its
            // install, from the derivations the court replayed (26, 50.2
            // step 2); nothing is read from beside the tape here. L9 (4/5)
            // wires the payment at `home_install`.
            let _ = &e;
        }
        Ok(out)
    }

    /// HEAR A FAMILY'S SPEECH (50.9): one adjudication per sentence sent,
    /// in the order sent. `Err` is the door's deferral on the lanes it
    /// names, for the whole act — nothing was heard — as it is for a
    /// sentence whose draw the voice's room does not cover (23.8): the
    /// voice dwells on those lanes and speaks again.
    pub fn hear_speech(
        &mut self,
        voice: [u8; 32],
        task: &[u8],
        spoken: &[(Element, Option<crate::transducer::sentence::Contraction>)],
        budget: &mut crate::physics::Budget,
    ) -> Result<Vec<crate::anastome::dialogue::Adjudication>, [u64; 4]> {
        let mut out = Vec::with_capacity(spoken.len());
        for (s, cited) in spoken {
            // CV5 — EVERY VERDICT COUNTED WHERE IT IS REACHED, by its kind,
            // for the observer's adjudication reading.
            match self.adjudicate_one(voice, task, s, cited.as_ref(), budget) {
                Ok(a) => {
                    note_verdict_of(&a);
                    out.push(a);
                }
                Err(lanes) => {
                    crate::observe::note_verdict(crate::observe::Verdict::Deferred);
                    return Err(lanes);
                }
            }
        }
        Ok(out)
    }

    /// ONE SENTENCE ADJUDICATED. A cited contraction is verified first
    /// (`verify_contraction`) and a sentence that is not the contraction it
    /// claims is refused as a `MalformedComplex` at the tape. Then the
    /// sentence is heard (`hear_sentence`): a refusal is recorded by the
    /// refused address with the receipt answered, so the next utterance's
    /// certificate can be verified; a listing is `Stood`, a use of held
    /// language `Ratified`. The receipt is this court's chain head after
    /// the act (47.1).
    pub(crate) fn adjudicate_one(
        &mut self,
        voice: [u8; 32],
        task: &[u8],
        s: &Element,
        cited: Option<&crate::transducer::sentence::Contraction>,
        budget: &mut crate::physics::Budget,
    ) -> Result<crate::anastome::dialogue::Adjudication, [u64; 4]> {
        use crate::anastome::dialogue::Adjudication;
        let addr = s.address();
        if let Some(c) = cited {
            if !self.verify_contraction(c, s) {
                let refusal = crate::court::verdict::Refusal::of_tape(crate::court::verdict::REFUSE_NOT_CONTRACTION, s);
                return Ok(self.refuse_sentence(voice, s, refusal));
            }
            self.refusals.remove(&c.before);
        }
        // 78.53 — A COLOUR WORD STANDS BY EVIDENCE, NEVER BY WHO SAID IT: the
        // court re-derives the fibre relation from the base of the family it
        // holds for this task (`fibre_relation`, the one reader the mind
        // minted with) and hears the word only where the two are the same.
        // Nothing is stored for the evidence: it is the court's own copy of
        // the family.
        // The word as this court will hold it: a colour word witnessed here
        // carries its witnessing task (outside its address).
        let held: Element;
        let mut theorem = false;
        let s = if s.is_colour_word() {
            let proof = self.colour_proof(s, Some(task));
            if proof == ColourProof::Derived {
                self.acts.colours_derived += 1;
                theorem = true;
            }
            match proof {
                ColourProof::Unproven => {
                    let refusal = crate::court::verdict::Refusal::of_tape(crate::court::verdict::REFUSE_FIBRE_UNWITNESSED, s);
                    return Ok(self.refuse_sentence(voice, s, refusal));
                }
                ColourProof::Witnessed(t) if !s.receipts().witness.contains(&t) => {
                    let mut w = s.clone();
                    let mut env = w.receipts().clone();
                    env.witness.push(t);
                    w.set_receipts(env);
                    held = w;
                    &held
                }
                _ => s,
            }
        } else {
            s
        };
        let out = match self.hear_sentence(voice, task, s, budget) {
            Err(refusal) => return Ok(self.refuse_sentence(voice, s, refusal)),
            Ok(out) => out,
        };
        // 78.56 IV — A THEOREM MINTS NOTHING AT ADMISSION: a symmetry of the
        // vacuum does no work until a solver cites it to annihilate residue,
        // and then it earns as a share of that payout (the cited-generator
        // shares of an install, `settle_recursive`). Its claim is dropped,
        // never slashed: the word stands, and its install releases nothing.
        if theorem {
            let _ = self.treasury.revoke_award(addr.as_bytes(), crate::economy::escrow::Revocation::Drop);
        }
        match out {
            Outcome::Deferred(lanes) => Err(lanes),
            // Per sentence: this one waits, the rest of the frame is
            // adjudicated as normal and keeps its verdicts.
            Outcome::Awaiting(form) => Ok(Adjudication::Awaiting { form }),
            Outcome::Refused(reason) if reason == crate::verdict::REFUSE_ROOM => {
                // A room that cannot be read defers on every lane: the
                // voice is told to wait, not told which lane was short by
                // a reading nobody took.
                Err(match self.room_shortfall(&voice, &s.magnitude()) {
                    Some(short) => short,
                    None => s.magnitude(),
                })
            }
            Outcome::Refused(reason) => {
                Ok(self.refuse_sentence(voice, s, crate::court::verdict::Refusal::of_tape(reason, s)))
            }
            Outcome::Pivot => Ok(self.refuse_sentence(
                voice,
                s,
                crate::court::verdict::Refusal::of_tape(crate::verdict::REFUSE_NOT_A_SENTENCE, s),
            )),
            // 78.93 B0 — and where the word stands: the stamp that admitted it.
            Outcome::Decomposed(_) => Ok(Adjudication::Ratified { receipt: self.chain.head_hash(), at: self.admitted_at(addr.as_bytes()) }),
            _ => Ok(Adjudication::Stood { receipt: self.chain.head_hash(), at: self.admitted_at(addr.as_bytes()) }),
        }
    }

    /// 78.53, 78.55 — WHAT PROVES A COLOUR WORD HERE, read and never searched
    /// for: WITNESSED by a scene — the task it was spoken with, or a task it
    /// carries as its witness — whose base this court re-derives from its own
    /// copy of the family (`fibre_relation`, the one reader the mind minted
    /// with); or DERIVED — it cites two colour words this court installed
    /// whose composite it is (law 8: the court composes what is cited, never
    /// peels); or UNPROVEN.
    pub(crate) fn colour_proof(&self, s: &Element, task: Option<&[u8]>) -> ColourProof {
        let Some(sigma) = s.letters().map(|l| l.sigma.clone()) else { return ColourProof::Unproven };
        let witnesses = |t: &[u8]| {
            self.task_family
                .get(t)
                .and_then(|family| crate::transducer::family::fibre_relation(family))
                .is_some_and(|seen| seen == sigma)
        };
        if let Some(t) = task.filter(|t| witnesses(t)) {
            return ColourProof::Witnessed(Address::from_bytes(t));
        }
        if let Some(t) = s.receipts().witness.iter().find(|t| witnesses(t.as_bytes())) {
            return ColourProof::Witnessed(t.clone());
        }
        let derived = s.colour_citation().is_some_and(|(first, after)| {
            match (self.installed_word(first.as_bytes()), self.installed_word(after.as_bytes())) {
                (Some(f), Some(a)) if f.is_colour_word() && a.is_colour_word() => match (f.letters(), a.letters()) {
                    (Some(fl), Some(al)) => {
                        crate::transducer::element::compose_fibre(&al.sigma, &fl.sigma).as_ref() == Some(&sigma)
                    }
                    _ => false,
                },
                _ => false,
            }
        });
        match derived {
            true => ColourProof::Derived,
            false => ColourProof::Unproven,
        }
    }

    /// A refusal answered and recorded (50.9): the adjudication names the
    /// kind and the coordinate, the record keeps them under the refused
    /// address with the receipt. 78.93 B0 — and the refusal is an act on
    /// this court's ledger (`seal_refusal`), its receipt the one the voice
    /// is answered with and contracts against.
    pub(super) fn refuse_sentence(
        &mut self,
        voice: [u8; 32],
        s: &Element,
        refusal: crate::court::verdict::Refusal,
    ) -> crate::anastome::dialogue::Adjudication {
        let addr = &s.address();
        let kind = refusal.kind();
        let receipt = match self.seal_refusal(s, kind, voice) {
            Some(r) => r.chain_hash,
            None => self.chain.head_hash(),
        };
        self.refusals.insert(
            addr,
            crate::court::verdict::Refused { kind, at: refusal.at.clone(), receipt },
        );
        crate::anastome::dialogue::Adjudication::Refused { kind, at: refusal.at, receipt }
    }

    /// 50.9: a cited contraction is verified against what THIS court
    /// refused — the kind, the coordinate and the receipt it answered with
    /// — and the sentence is the contraction at that coordinate. A court
    /// that never refused the cited address holds no receipt to verify and
    /// replays the sentence as a sentence: the certificate is a claim on
    /// another court's chain.
    pub(crate) fn verify_contraction(&self, c: &crate::transducer::sentence::Contraction, s: &Element) -> bool {
        let Some(held) = self.refusals.get(&c.before) else {
            return true;
        };
        if held.kind != c.kind || held.at != c.at || held.receipt != c.receipt {
            return false;
        }
        // An arriving sentence, decoded by name (L16).
        crate::transducer::sentence::from_element_form(s)
            .is_some_and(|f| f.is_contraction_at(c.kind, &c.at))
    }

    /// ONE SENTENCE AT THE COURT (50.2 G, 23.1), the outcome alone — what
    /// the tests read. `hear_sentence` carries the refusal's coordinate.
    #[cfg(test)]
    pub(crate) fn perceive_sentence(&mut self, voice: [u8; 32], task: &[u8], s: &Element) -> Outcome {
        // The test's own act: its root, one stroke — spoken again, as the
        // voice speaks again, while the court defers it on no lane with its
        // replay moved on (78.153: a replay longer than one stroke resumes).
        let key = crate::cognition::court::replay::replay_key(s, (!s.is_colour_word()).then_some(task));
        loop {
            let before = self.replay_cursors.get(&key).map(|c| c.progress());
            let mut budget = self.physics.stroke();
            match self.hear_sentence(voice, task, s, &mut budget) {
                Ok(Outcome::Deferred(lanes))
                    if lanes == [0; 4] && self.replay_cursors.get(&key).map(|c| c.progress()) != before => {}
                Ok(out) => return out,
                Err(refusal) => return Outcome::Refused(refusal.reason),
            }
        }
    }

    /// ONE SENTENCE HEARD. The door is read first (`reflex_open`):
    /// deferred, the lanes are written and nothing is heard. The tape is
    /// then replayed against the family of the task it was spoken for —
    /// every word vertex carried onto the court's own copy of each pair,
    /// every edge re-derived, the remainder what its demands name — and
    /// refused with the reason and its coordinate in the journal (`court
    /// refuse speech {addr}: {reason}`); what stands is listed
    /// (`list_sentence`) and bound to the task, so the stroke replays it on
    /// the same family and the install mints what it closes there. The
    /// court composes nothing here.
    pub(crate) fn hear_sentence(
        &mut self,
        voice: [u8; 32],
        task: &[u8],
        s: &Element,
        budget: &mut crate::physics::Budget,
    ) -> Result<Outcome, crate::court::verdict::Refusal> {
        if let Err(deficit) = self.reflex_open(voice) {
            return Ok(Outcome::Deferred(deficit));
        }
        // 21 step 3, 50.2 step 2 — A CITATION THIS COURT HAS NOT RULED IS
        // A WAIT, NOT A REFUSAL, and it is decided BEFORE the replay so
        // no refusal is ever formed for it. The voice spoke one syllabus
        // step ahead of this court's cover, which is what a syllabus is
        // for; a refusal here would be sealed by the voice
        // (`receive_adjudication`) and the sentence lost permanently for
        // being early. The sentence is docketed under the address it
        // waits on and heard again when that address is ruled.
        if let Some(form) = self.unruled_citation(s) {
            self.await_citation(voice, task, s, &form);
            return Ok(Outcome::Awaiting(form));
        }
        // 78.53 — A COLOUR WORD'S EVIDENCE IS ITS FIBRE RELATION, re-derived
        // from this family at adjudication (`adjudicate_one`); it carries no
        // chain by construction, so its replay against the family is that
        // evidence, and here it is replayed for integrity alone.
        let family_task = match s.is_colour_word() {
            true => None,
            false => Some(task),
        };
        // The caller's stroke (65.5): a sentence is heard inside the act that
        // hears it — a speech frame, or a ruling re-hearing what waited.
        //
        // 78.153 — ON THAT ONE STROKE, RESUMED: a replay the stroke could not
        // finish is NOT HEARD — deferred on no lane, the voice speaks again,
        // and the replay resumes where it stopped. Never a refusal.
        match self.replay_on_cursor(s, family_task, true, budget).0 {
            crate::verdict::Replayed::Refused(refusal) => {
                // CV1 — every refusal carries its kind (78.9), and the observer
                // counts it by kind.
                crate::observe::note_obstruction(refusal.obstruction.kind());
                return Err(refusal);
            }
            crate::verdict::Replayed::Unread => return Ok(Outcome::Deferred([0; 4])),
            crate::verdict::Replayed::Clean => {}
        }
        // 78.149 — WHAT THE SENTENCE DISCHARGES, measured on the family of
        // the task it was heard for (the family its binding below will name,
        // so the install reads the same): its escrow is priced from this and
        // nothing else.
        let heard_on = match task.is_empty() || task == [0u8; 32] {
            true => None,
            false => self.task_family.get(task).map(Vec::as_slice),
        };
        let discharge = self.discharge_on(s.address().as_bytes(), s, heard_on);
        let out = self.list_sentence(s, voice, discharge);
        // A SENTENCE SPOKEN FOR ITSELF NAMES NO TASK. `Entity::speak` sends
        // an empty task for a word spoken on its own — an axis germ, a
        // crystallized idea, an adjoined letter — and `[]` is not `[0; 32]`,
        // so the gate let it through: the listing was bound to the empty
        // task, filed under it in `task_listings`, and every reader of the
        // binding asked `task_family` for a family the empty task has none
        // of. An absent task is absence (rule 11), not a task of no bytes.
        if !matches!(out, Outcome::Refused(_) | Outcome::Pivot) && !task.is_empty() && task != [0u8; 32] {
            let addr = s.address();
            self.speech_task.insert(&addr, Address::from(task));
            self.bind_listing_task(&addr, task);
        }
        Ok(out)
    }

    /// List a word on the docket if it is not already provisional, escrowed
    /// at its discharge on the family it is bound to (78.149,
    /// `discharge_of`).
    pub(crate) fn docket_connection(&mut self, word: &Element, ratifier: [u8; 32]) {
        let discharge = self.discharge_of(word.address().as_bytes(), word);
        self.docket_discharged(word, ratifier, discharge);
    }

    /// The docket, escrowed at a discharge the caller measured — the
    /// hearing's own, where the word is not bound to its family yet. The
    /// escrow is priced from the DISCHARGE alone (78.149): never the span
    /// the word stood, never what it asks, nothing for a word that names no
    /// body. Its magnitude is still what the field and the deflation record
    /// carry: the word's measure, not its price.
    pub(crate) fn docket_discharged(&mut self, word: &Element, ratifier: [u8; 32], discharge: [u64; 4]) {
        if word.is_identity() && !word.is_rewrite() {
            return;
        }
        let addr = word.address();
        if self.provisional.contains(&addr) {
            return;
        }
        let mag = word.magnitude();
        // A rewrite's discharge is its yield, escrowed as read; any other
        // word's is priced on the lane (§5).
        let mu = if word.is_rewrite() {
            discharge
        } else {
            crate::economy::price(&discharge, &self.physics.pressure)
        };
        let anchor = self.anchor();
        // THIS COURT re-derived the word to docket it: its own vantage,
        // recorded so a witness's re-derivation can converge with it (47.3).
        // Every path that dockets records one, or the object waits for a
        // second organ it can never reach because the first was never there.
        self.converge_here(word, ratifier);
        self.treasury.escrow_award(&addr, vec![ratifier], mu, anchor);
        self.deflations.insert(&addr, mag);
        self.provisional.insert(&addr, word.clone());
        // L8y: the voice that spoke it, recorded where the court knows it.
        // `drawers` is cleared by `pending_take` before the install, so the
        // install had no name to credit and wrote the null identity; a use
        // of the word was then paid to nobody (26).
        self.note_discovered(&addr, ratifier);
        self.stamp_standing(&addr);
        self.mark_persist();
        let inv = crate::transducer::stalk::invariant_of(word);
        if self.field.get(&addr).is_none() {
            self.write_field(
                &addr,
                inv,
                crate::transducer::stalk::Closure::Cavity,
                word.sub().to_vec(),
                &mag,
                &mag,
            );
        }
        self.book_gen += 1;
        self.append_step(crate::chronometry::ActKind::Attest, &addr, ratifier);
    }


    /// The court's voice, heard on its verdicts.
    pub fn set_court_voice(&mut self, voice: [u8; 32]) {
        self.court_voice = Some(voice);
    }

    /// SETTLE A PROBE: the replayed residual of an open listing against the
    /// speaker's sentence. Closure (empty residual) of a fibre section or a
    /// connection listing pays morphism deflation; a tape covering pays
    /// nothing. A partial residual updates tension and relieves the physics
    /// (the measurement is real) but pays μ⃗ = 0⃗ — covering links is not
    /// an axial move. Cited words still earn settle_use. Returns
    /// (code, paid μ⃗, remaining μ⃗).
    pub fn settle_probe(
        &mut self,
        voice: [u8; 32],
        target: &[u8],
        residual: Vec<(i64, i64, u8, u8)>,
        cites: &[crate::anastome::dialogue::CiteWire],
        section: bool,
    ) -> crate::anastome::dialogue::ProbeReading {
        use crate::anastome::dialogue::{ProbeClosure, ProbeReading};
        use crate::transducer::stalk::Closure;
        // A PROBE MAY NAME ONE DEMAND VERTEX of the listing (L8q), and
        // then it is not a deflation of the listing's links at all: the
        // court replays the cited form onto THAT demand's own scene and
        // settles on what it reproduces.
        if let Some((at, demand)) = crate::transducer::sentence::read_demand_at(target) {
            return self.settle_demand(voice, &at, &demand, cites);
        }
        let Some(listing) = self.pending.get(target) else {
            // Gone from the book in the state it was quoted.
            return ProbeReading::gone();
        };
        let listing = listing.clone();
        self.record_spoken(Address::from(target), voice);
        // A listing with no recorded tension stands at its own magnitude:
        // nothing has been settled against it yet.
        let before = match self.tension.get(target) {
            Some(t) => *t,
            None => listing.magnitude(),
        };
        let rem = element::measure_lanes(&residual);
        let after = rem;
        let anchor = self.anchor();
        let mag = listing.magnitude();
        let mut charts_at: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant> =
            crate::network::nerve::Keyed::new();
        for cite in cites {
            let inv = crate::transducer::stalk::invariant(&cite.body);
            charts_at.insert(inv.key(), inv);
        }
        let mut charts: Vec<crate::transducer::stalk::Invariant> = charts_at
            .scoped()
            .map(|(_, inv)| inv.clone())
            .collect();
        if charts.is_empty() {
            charts.push(crate::transducer::stalk::invariant(listing.sub()));
        }
        // The transport a flat orbit is recorded under is the composite of
        // the cites that flattened it, by the geometric product, not the
        // first cite's versor with the identity invented when there are
        // none. A versor invented where no transport exists is a claim the
        // court cannot replay (rule 11); `None` records no orbit at all.
        let cite_g: Option<element::Versor> = cites
            .iter()
            .map(|c| c.transport.clone())
            .reduce(|acc, g| acc.geo(&g));
        if residual.as_slice().is_empty() {
            // The subsidy a section or a connection earns is the treasury's,
            // written on its own rows; the reading carries the tension the
            // closure removed.
            if section || listing.is_connection() {
                self.treasury.settle_subsidy(voice, crate::economy::price(&before, &self.physics.pressure), anchor);
            }
            for chart in charts {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &[0; 4]);
                if let Some(g) = cite_g.clone() {
                    self.record_flat(target, chart, voice, g);
                }
            }
            // 78.24 (2c): the listing is cited as it stood, read before it
            // is unlisted.
            let cite = self.cite_of(target);
            self.unlist(target, false);
            self.physics.relieve(&mag);
            self.physics.syntropy(&mag);
            self.append_after_removal(crate::chronometry::ActKind::Settled, target, cite, voice);
            // EC1: every cited generator's use is paid AT the Settled act
            // that carries it (POWC), sealed first.
            let settled = self.anchor();
            for cite in cites {
                let addr = &cite.addr;
                if let Some(w) = self.basis.get(addr) {
                    // 78.149 (the coordinator, 2026-10-08): a use pays the
                    // used word's DISCHARGE, never its magnitude — a letter's
                    // holonomic unit earns nothing.
                    let mag = self.discharge_of(addr, w);
                    if let Some(disc) = self.discoverer_of(addr) {
                        self.treasury.settle_use(voice, disc, mag, settled);
                    }
                }
            }
            return ProbeReading { settled: before, closure: ProbeClosure::Closed, remaining: [0; 4] };
        }
        let mut listed = listing.sub().to_vec();
        listed.sort_unstable();
        if residual == listed {
            // What the field already stands on, and this residual where the
            // field stands on nothing.
            let standing = match self
                .field
                .get(target)
                .map(|f| union_remainders(&f.entries))
                .filter(|r| !r.is_empty())
            {
                Some(r) => r,
                None => residual.clone(),
            };
            for chart in charts {
                self.write_field(target, chart, Closure::Cavity, standing.clone(), &before, &before);
            }
            return ProbeReading { settled: [0; 4], closure: ProbeClosure::Nothing, remaining: before };
        }
        let rem_inv = crate::transducer::stalk::invariant(&residual);
        for chart in &charts {
            if *chart != rem_inv {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &after);
                if let Some(g) = cite_g.clone() {
                    self.record_flat(target, chart.clone(), voice, g);
                }
            }
        }
        let v_disloc = {
            let base_shape = element::shape_of_chain(listing.sub());
            let rem_shape = element::shape_of_chain(&residual);
            // One stroke of this court's own metabolism reads the candidates.
            let mut budget = self.physics.stroke();
            match (base_shape, rem_shape) {
                (Some(sb), Some(sr)) => crate::transducer::blade::versor_between(&sb, &sr, &mut budget).ok(),
                _ => None,
            }
        };
        let state = match v_disloc.clone() {
            Some(v) => Closure::Twisted(Box::new(v)),
            None => Closure::Cavity,
        };
        self.write_field(target, rem_inv.clone(), state, residual.clone(), &before, &after);
        self.listing_atlas.admit_also(&rem_inv, target);
        // THE RESIDUAL IS ON THE FIELD, AND NOWHERE ELSE. `write_field` above
        // carries what the probe left — the residual chain and the closure
        // it stands at — which is the whole of what a probe measures.
        //
        // NOTHING IS WRITTEN TO THE LISTING'S MAGNITUDE. `blind_replay`
        // re-derives a residual germ's lanes from its own chain and refuses
        // a word claiming more than the chain supports, which is what stops
        // a voice inflating its own measure; a court that wrote a count of
        // leftover links into a lane would slash its own listing on the
        // next circuit, and every listing a voice made progress on would be
        // destroyed by the court's own write. Three laws say so: `𝒽` is
        // boundary topology and the rotor's own coordinates (16.16, 42.6),
        // never a tally of links; the field already holds the residual, and
        // a second carrier for it drifts (law 12); and a word's magnitude is
        // a function of its chain, which is the one property the replay
        // checks.
        //
        // The dislocation stays: a versor relating the base shape to the
        // residual's is a fibre fact and belongs on the word.
        if let (Some(held), Some(v)) = (self.pending.get_mut(target), v_disloc) {
            held.record_dislocation(element::assignment(&residual), v, element::FIBER_SPATIAL);
        }
        // What the speech SETTLED: the tension it removed, per lane, and
        // nothing on a lane where the residual stands as high as before.
        let settled = crate::economy::shortfall(&before, &after);
        self.forget_spoken(target);
        self.physics.relieve(&settled);
        self.book_gen += 1;
        self.physics.syntropy(&mag);
        self.append_step(crate::chronometry::ActKind::Settled, target, voice);
        ProbeReading {
            settled,
            closure: ProbeClosure::Residual(residual_versor(listing.sub(), &residual)),
            remaining: after,
        }
    }

}

/// MU-CLIENT §5.4 — THE WORDS OF A SENTENCE, the dictionary it is composed
/// of: first the word each FACE applies — its letters across the overlap
/// (`Form::face_word`) at the face's carriage with its hue map, the
/// algorithm that carries one chart onto the next — then the word each
/// closed RING teaches for its whole path, read by its coordinates
/// (`sentence::word::taught`) and spelled at its carriage carrying its return
/// (`spelled_word`). Each once, faces first. A face that applies nothing (its
/// word, carriage and hue map the identity) is no word; a ring's word the
/// alphabet does not spell is offered as a letter by `hear_open`, which every
/// heard sentence passes.
pub(crate) fn words_of_sentence(s: &Element, budget: &mut crate::physics::Budget) -> Vec<Element> {
    use crate::transducer::sentence::compose::{sentences_of, Closure};
    let Some(form) = crate::transducer::sentence::from_element_form(s) else { return Vec::new() };
    if form.faces.is_empty() {
        return Vec::new();
    }
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut out = Vec::new();
    for (o, face) in form.faces.entries() {
        let Some(word) = form.face_word(&o) else { continue };
        let carriage = &face.place.carriage;
        let gauge = &face.place.gauge;
        let still = word.is_identity()
            && carriage.projective_eq(&crate::transducer::element::identity_versor())
            && crate::transducer::sentence::transport::gauge_eq(gauge, &[]);
        if still {
            continue;
        }
        let Some(e) = crate::transducer::element::from_connection(&word, carriage, gauge, &Vec::new(), [0, 0, 0, 0]) else { continue };
        if seen.insert(e.address(), ()).is_none() {
            out.push(e);
        }
    }
    let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
    let rings: Vec<crate::transducer::sentence::Orbit> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
    for (_, read) in sentences_of(&form, &rings, budget) {
        let Ok(comp) = read else { continue };
        let Some(word) = crate::transducer::sentence::word::taught(&comp) else { continue };
        let period = match comp.closure {
            Closure::Closes(l) => l,
            Closure::Rosette | Closure::Unbound | Closure::Unread => 1,
        };
        let Some(e) = spelled_word(&word.word, period) else { continue };
        if seen.insert(e.address(), ()).is_none() {
            out.push(e);
        }
    }
    out
}

/// A WORD SPELLED (78.52, 78.100 D1): its motion factored into rotation and
/// carriage, the rotation spelled by the seventeen, the word that spelling's
/// connection at that carriage carrying its return (`ℓ − 1` on the shell) —
/// the one spelling the mind's transcriber writes. `None` where it does not
/// factor or the alphabet does not spell it.
pub(crate) fn spelled_word(motion: &crate::transducer::cga::Mv, period: u64) -> Option<Element> {
    let f = crate::transducer::element::conformal_factors(motion).ok()?;
    let rotor = crate::transducer::element::rotor_of_factors(&f).ok()?;
    let word = match crate::transducer::alphabet::factor_rotor(&rotor) {
        Some(Ok(w)) => w,
        _ => return None,
    };
    crate::transducer::element::from_connection(&word, &f.carriage(), &[], &Vec::new(), [0, 0, 0, period.checked_sub(1)?])
}

/// PR1 — THE VERSOR A RESIDUAL STANDS OFF THE LISTING BY: the tether between
/// the listing's body and the residual's, `V = s_b s_a` over their nuclei
/// (`blade::tether`, the one producer). `None` — the residual's versor unread
/// by name — where either body does not lift or its nucleus does not read.
pub(crate) fn residual_versor(listing: &[(i64, i64, u8, u8)], residual: &[(i64, i64, u8, u8)]) -> Option<crate::transducer::cga::Mv> {
    let lift = |links: &[(i64, i64, u8, u8)]| {
        crate::transducer::blade::lift(
            &links.iter().map(|&(r, c, _, h)| crate::transducer::blade::Compartment::from((r, c, 0i64, h))).collect::<Vec<_>>(),
        )
    };
    crate::transducer::blade::tether(&lift(listing), &lift(residual))
}
