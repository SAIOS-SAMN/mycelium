// 26 — SPEECH AGAINST THE BOOK: a voice standing sentences, a probe
// settling against an open listing, and a rewrite offered for one.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

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
        match self.hear_speech(voice, task, &spoken) {
            Ok(adjudicated) => adjudicated.iter().map(|a| a.code()).collect(),
            Err(_) => vec![crate::anastome::dialogue::SPEECH_DIALECT; sentences.len()],
        }
    }

    /// A SPEECH FRAME AS THE JUDGE HANDS IT OVER: every tape decoded, a
    /// tape that does not parse adjudicated a `MalformedComplex` naming no
    /// address, and the rest heard in the order sent.
    pub fn hear_frame(
        &mut self,
        act: &crate::anastome::dialogue::SpeechAct,
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
        let heard = self.hear_speech(act.voice, &act.task, &spoken)?;
        let receipt = self.chain.head_hash();
        let mut out = Vec::with_capacity(act.sentences.len());
        let mut next = heard.into_iter();
        for i in 0..act.sentences.len() {
            if unparsed.contains(&i) {
                out.push(Adjudication::Refused {
                    kind: RefusalKind::MalformedComplex,
                    at: At::Tape(Address::new(Vec::new())),
                    receipt,
                });
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
    ) -> Result<Vec<crate::anastome::dialogue::Adjudication>, [u64; 4]> {
        let mut out = Vec::with_capacity(spoken.len());
        for (s, cited) in spoken {
            out.push(self.adjudicate_one(voice, task, s, cited.as_ref())?);
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
    ) -> Result<crate::anastome::dialogue::Adjudication, [u64; 4]> {
        use crate::anastome::dialogue::Adjudication;
        let addr = s.address();
        if let Some(c) = cited {
            if !self.verify_contraction(c, s) {
                eprintln!(
                    "  court contraction refused {:02x}{:02x}…: not the contraction of {:02x}{:02x}… at {}",
                    addr[0],
                    addr[1],
                    c.before[0],
                    c.before[1],
                    c.at.label()
                );
                let refusal = crate::court::verdict::Refusal::of_tape(crate::court::verdict::REFUSE_NOT_CONTRACTION, s);
                return Ok(self.refuse_sentence(voice, &addr, refusal));
            }
            eprintln!(
                "  court contraction verified {:02x}{:02x}… from {:02x}{:02x}… by {} at {}",
                addr[0],
                addr[1],
                c.before[0],
                c.before[1],
                c.kind.label(),
                c.at.label()
            );
            self.refusals.remove(&c.before);
        }
        let out = match self.hear_sentence(voice, task, s) {
            Err(refusal) => return Ok(self.refuse_sentence(voice, &addr, refusal)),
            Ok(out) => out,
        };
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
                Ok(self.refuse_sentence(voice, &addr, crate::court::verdict::Refusal::of_tape(reason, s)))
            }
            Outcome::Pivot => Ok(self.refuse_sentence(
                voice,
                &addr,
                crate::court::verdict::Refusal::of_tape(crate::verdict::REFUSE_NOT_A_SENTENCE, s),
            )),
            Outcome::Decomposed(_) => {
                let receipt = self.chain.head_hash();
                eprintln!(
                    "  court adjudication {:02x}{:02x}… for {:02x}{:02x}…: ratified receipt {}",
                    addr[0],
                    addr[1],
                    voice[0],
                    voice[1],
                    self.chain.head_k()
                );
                Ok(Adjudication::Ratified { receipt })
            }
            _ => {
                let receipt = self.chain.head_hash();
                eprintln!(
                    "  court adjudication {:02x}{:02x}… for {:02x}{:02x}…: stood receipt {}",
                    addr[0],
                    addr[1],
                    voice[0],
                    voice[1],
                    self.chain.head_k()
                );
                Ok(Adjudication::Stood { receipt })
            }
        }
    }

    /// A refusal answered and recorded (50.9): the adjudication names the
    /// kind and the coordinate, the record keeps them under the refused
    /// address with the receipt.
    fn refuse_sentence(
        &mut self,
        voice: [u8; 32],
        addr: &Address,
        refusal: crate::court::verdict::Refusal,
    ) -> crate::anastome::dialogue::Adjudication {
        let kind = refusal.kind();
        let receipt = self.chain.head_hash();
        eprintln!(
            "  court adjudication {:02x}{:02x}… for {:02x}{:02x}…: refused {} at {} receipt {}",
            addr[0],
            addr[1],
            voice[0],
            voice[1],
            kind.label(),
            refusal.at.label(),
            self.chain.head_k()
        );
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
        match self.hear_sentence(voice, task, s) {
            Ok(out) => out,
            Err(refusal) => Outcome::Refused(refusal.reason),
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
    ) -> Result<Outcome, crate::court::verdict::Refusal> {
        if let Err(deficit) = self.reflex_open(voice) {
            eprintln!(
                "  court speech deferred voice {:02x}{:02x}… lanes {deficit:?}",
                voice[0], voice[1]
            );
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
            let a = s.address();
            eprintln!(
                "  court speech awaiting {:02x}{:02x}…: cited {:02x}{:02x}… is not ruled here",
                a[0], a[1], form[0], form[1]
            );
            self.await_citation(voice, task, s, &form);
            return Ok(Outcome::Awaiting(form));
        }
        let family = self.task_family.get(task).cloned();
        if let Some(refusal) =
            crate::court::verdict::blind_replay_at(s, &|a| self.resolve_form(a), family.as_deref(), self)
        {
            let a = s.address();
            eprintln!("  court refuse speech {:02x}{:02x}…: {} at {}", a[0], a[1], refusal.reason, refusal.at.label());
            return Err(refusal);
        }
        let out = self.list_sentence(s, voice);
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

    /// List a word on the docket if it is not already provisional.
    pub(crate) fn docket_connection(&mut self, word: &Element, ratifier: [u8; 32]) {
        if word.is_identity() && !word.is_rewrite() {
            return;
        }
        let addr = word.address();
        if self.provisional.contains(&addr) {
            return;
        }
        let mag = word.magnitude();
        let mu = if word.is_rewrite() {
            mag
        } else {
            crate::economy::price(&mag, &self.physics.pressure)
        };
        let anchor = self.anchor();
        // THIS COURT re-derived the word to docket it: its own vantage,
        // recorded so a witness's re-derivation can converge with it (47.3).
        // Every path that dockets records one, or the object waits for a
        // second organ it can never reach because the first was never there.
        self.converge_here(word, ratifier);
        self.treasury.escrow_award(&addr, vec![ratifier], mu, anchor);
        self.deflations.insert(&addr, mag);
        let k_now = self.k_step();
        self.provisional_k.insert(&addr, k_now);
        self.provisional.insert(&addr, word.clone());
        // L8y: the voice that spoke it, recorded where the court knows it.
        // `drawers` is cleared by `pending_take` before the install, so the
        // install had no name to credit and wrote the null identity; a use
        // of the word was then paid to nobody (26).
        self.note_discovered(&addr, ratifier);
        self.stamp_standing(&addr);
        self.provisional_h.insert(&addr, self.height());
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

    /// APPLY an attach ruling: the court connected `sibling` to `base` by
    /// the transport g. A provisional listing of the sibling clears with its
    /// award evaporating (it is not a base word). When both words are held
    /// here and g carries the sibling onto the base, the sibling transports
    /// into the bundle as a section and its card retires — its discoverer
    /// earns the section subsidy when the fibre gained volume. Returns the
    /// cards retired (0 or 1).
    pub fn apply_attach(
        &mut self,
        sibling: &[u8],
        base: &[u8],
        g: &element::Versor,
    ) -> usize {
        if self.provisional.remove(sibling).is_some() {
            let _ = self.treasury.revoke_award(sibling, false);
        }
        let Some(base_pos) = self.basis.get(base).map(|w| element::positions(w.sub())) else {
            return 0;
        };
        let Some(sib) = self.basis.get(sibling).map(|w| (**w).clone()) else {
            return 0;
        };
        if element::pullback(&base_pos, sib.sub(), g).is_none() {
            return 0;
        };
        let Some(disc) = self.discoverer_of(sibling) else {
            return 0;
        };
        // A base the basis does not hold gains nothing: nothing was
        // attached to it.
        let gained = match self.word_mut(base) {
            Some(w) => w.update(|e| e.attach(&sib, g)),
            None => 0,
        };
        self.touch_at(base);
        self.basis_remove(sibling);
        if gained > 0 {
            if let Some(after) = self.basis.get(base).map(|w| (**w).clone()) {
                let payload = crate::transducer::rewrite::RewritePayload::Condense {
                    sections: after.sections().to_vec(),
                    gauge: g.clone(),
                };
                let _ = self.mint_rewrite(
                    crate::transducer::rewrite::RewriteKind::Condense,
                    std::slice::from_ref(&sib),
                    &after,
                    payload,
                    disc,
                );
                self.torse_at(base);
            }
        }
        self.physics.syntropy(&[0, 0, 1, 0]);
        1
    }

    /// SETTLE A PROBE: the replayed residual of an open listing against the
    /// speaker's sentence. Closure (empty residual) of a fibre section or a
    /// connection listing pays morphism deflation; a tape covering pays
    /// nothing. A partial descent updates tension and relieves the physics
    /// (the measurement is real) but pays μ⃗ = 0⃗ — covering cells is not
    /// an axial move. Cited words still earn settle_use. Returns
    /// (code, paid μ⃗, remaining μ⃗).
    pub fn settle_probe(
        &mut self,
        voice: [u8; 32],
        target: &[u8],
        residual: Vec<(i64, i64, u8, u8)>,
        cites: &[crate::anastome::dialogue::CiteWire],
        section: bool,
    ) -> (u8, [u64; 4], [u64; 4]) {
        use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
        use crate::transducer::stalk::Closure;
        // A PROBE MAY NAME ONE DEMAND VERTEX of the listing (L8q), and
        // then it is not a deflation of the listing's cells at all: the
        // court replays the cited form onto THAT demand's own scene and
        // settles on what it reproduces.
        if let Some((at, demand)) = crate::transducer::sentence::read_demand_at(target) {
            return self.settle_demand(voice, &at, &demand, cites);
        }
        let Some((listing, _)) = self.pending.get(target) else {
            return (PROBE_NOTHING, [0; 4], [0; 4]);
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
        // The transport a flat loop is recorded under is the composite of
        // the cites that flattened it, by the geometric product, not the
        // first cite's versor with the identity invented when there are
        // none. A versor invented where no transport exists is a claim the
        // court cannot replay (rule 11); `None` records no loop at all.
        let cite_g: Option<element::Versor> = cites
            .iter()
            .map(|c| c.transport.clone())
            .reduce(|acc, g| acc.geo(&g));
        if residual.as_slice().is_empty() {
            let paid = if section || listing.is_connection() {
                self.treasury.settle_subsidy(voice, crate::economy::price(&before, &self.physics.pressure), anchor)
            } else {
                [0; 4]
            };
            for cite in cites {
                let addr = &cite.addr;
                if let Some(w) = self.basis.get(addr) {
                    let mag = w.magnitude();
                    if let Some(disc) = self.discoverer_of(addr) {
                        self.treasury.settle_use(voice, disc, mag, anchor);
                    }
                }
            }
            for chart in charts {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &[0; 4]);
                if let Some(g) = cite_g.clone() {
                    self.record_flat(target, chart, voice, g);
                }
            }
            self.unlist(target, false);
            self.physics.relieve(&mag);
            self.physics.syntropy(&mag);
            self.append_step(crate::chronometry::ActKind::Settled, target, voice);
            return (PROBE_CLOSED, paid, [0; 4]);
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
            return (PROBE_NOTHING, [0; 4], before);
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
            match (base_shape, rem_shape) {
                (Some(sb), Some(sr)) => crate::transducer::blade::versor_between(&sb, &sr),
                _ => None,
            }
        };
        let state = match v_disloc.clone() {
            Some(v) => Closure::Twisted(v),
            None => Closure::Cavity,
        };
        self.write_field(target, rem_inv.clone(), state, residual.clone(), &before, &after);
        self.listing_atlas.admit_also(&rem_inv, target);
        // THE DESCENT IS ON THE FIELD, AND NOWHERE ELSE. `write_field` above
        // carries what the probe left — the residual chain and the closure
        // it stands at — which is the whole of what a descent measures.
        //
        // THIS WROTE `held.0.mag[3] = residual.len()` BESIDE IT, and the
        // court then slashed its own listing on the next circuit. Measured
        // on the fleet, 2026-09-11, hyphae a/b/c after the epoch-7 recut:
        //
        //   court slash 0412… magnitude: chain [0, 15, 0, 0]
        //                               word  [0, 15, 0, 14]
        //                               cells 15 sections 0 torsion 0
        //
        // `blind_replay` re-derives a residual germ's lanes from its own
        // chain and refuses a word claiming more than the chain supports —
        // correctly, since that check is what stops a voice inflating its
        // own measure. A pure recolour of fifteen cells measures shell 0,
        // so the moment a probe descended on a listing, the court wrote 14
        // into that lane and the next orbit slashed the listing for it.
        // Every listing a voice made progress on was destroyed by the
        // court's own write, which is why descent never accumulated.
        //
        // THREE FAULTS IN ONE LINE, and each is its own law. It put a COUNT
        // of leftover cells in a lane that is not a count of that kind —
        // `𝒽` is boundary topology and the rotor's own coordinates (16.16,
        // 42.6), never a tally of cells. It put the same fact on a second
        // carrier while the field already held it (law 12). And it made a
        // word's magnitude stop being a function of its chain, which is the
        // one property the replay checks.
        //
        // The dislocation stays: a versor relating the base shape to the
        // residual's is a fibre fact and belongs on the word.
        if let (Some(held), Some(v)) = (self.pending.get_mut(target), v_disloc) {
            held.0.record_dislocation(element::assignment(&residual), v, element::FIBER_SPATIAL);
        }
        // What the speech SETTLED: the tension it removed, per lane, and
        // nothing on a lane where the residual stands as high as before.
        let settled = crate::economy::shortfall(&before, &after);
        self.forget_spoken(target);
        self.physics.relieve(&settled);
        self.book_gen += 1;
        self.physics.syntropy(&mag);
        self.append_step(crate::chronometry::ActKind::Settled, target, voice);
        (PROBE_DESCENT, [0; 4], after)
    }

}
