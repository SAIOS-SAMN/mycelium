// 47.3, 13.9 — THE HOME INSTALL: what a second vantage installs, the
// citations an installed form notes, and the citers a slashed word
// re-dockets.
//
//! Split from `glue.rs` (MU-STANDARDS 1.1, one organ per file): the glue
//! converges two organs on one object there; the install that convergence
//! rests on is here.

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// 47.3: THE HOME INSTALLS on a second vantage. `witness` is the organ
    /// whose re-derivation the install rests on, and it is the author of the
    /// receipt this act seals (45.1): a book that cannot say which vantage
    /// an install stood on cannot re-derive the install.
    ///
    /// The install is inside the act that witnessed it and draws on that
    /// act's stroke (65.5), threaded to the basis, the ruling, the inputs a
    /// rewrite retires and the sentences re-heard after the close.
    pub(crate) fn home_install(&mut self, addr: &[u8], witness: [u8; 32], budget: &mut crate::physics::Budget) {
        // THE ORBIT CLOSES WHOLE BEFORE WHAT WAITED ON IT IS HEARD. Ruling the
        // word re-hears its citers, and it did so before this act released the
        // word's own draw — so a citer spoken by the same voice met a room
        // still full and was deferred, and the deferral was dropped: the
        // sentence was neither heard nor waiting. The waiting sentences are
        // held across the close and heard after it.
        let waiting = self.awaiting.remove(addr);
        let held = self.word_at(addr).cloned().or_else(|| self.provisional.get(addr).cloned());
        // L8y: the voice whose draw this address occupies is `drawers`,
        // and where `pending_take` already cleared it the speaker recorded
        // when the sentence was heard stands. Written as `[0; 32]`, every
        // word this court ruled had no speaker, so a use of it was credited
        // to the null identity and no speaker of a generator was ever paid
        // (26).
        let drawn = self
            .drawers
            .get(addr)
            .copied()
            .or_else(|| self.discoverer_of(addr));
        let mut federated = None;
        if self.basis.get(addr).is_none() {
            if let Some(w) = held.as_ref() {
                if !(w.is_connection() && w.letters().is_none()) {
                    match drawn {
                        Some(voice) => self.basis_push(w.clone(), voice, budget),
                        // No draw: a word this court did not receive from a
                        // voice — a peer's glue, a restore. It has no
                        // speaker here, which is a fact and not a default
                        // (rule 11).
                        None => self.basis_push(w.clone(), [0; 32], budget),
                    }
                    federated = Some(w.clone());
                }
            }
        }
        // 13.9: the home's install is its OWN word, ruled on a witness's
        // re-derivation, so it becomes this court's section on the chart.
        self.rule_own(addr, budget);
        // 45.1: an install is an ADMITTED ACT, so it is a step of this
        // kernel's clock and a receipt in its chain. Without one the book
        // grew a ruled word at no position: nothing cited the peer receipts
        // admitted since the last step, and a reader replaying the chain
        // found the install nowhere in it. EC1: sealed BEFORE it pays, so
        // every row the install pays stands at it; 78.27 (3): and before
        // the federation files the word, so the word stands at it.
        self.append_step(crate::chronometry::ActKind::Install, addr, witness);
        if let Some(w) = federated {
            self.fed_admit(addr, w);
        }
        // MU-CLIENT §5.4 (A) — THE WORDS IT TEACHES ENTER THE LANGUAGE: a
        // sentence is the court's evidence and section; what its closed
        // rings teach is installed beside it as words, each on its own act.
        let word_shares = match &held {
            Some(w) if !w.is_rewrite() => self.install_taught_words(addr, w, drawn, witness, budget),
            _ => Vec::new(),
        };
        let installed = self.anchor();
        if let Some(w) = held {
            if w.is_rewrite() {
                if self.treasury.release_rewrite(addr, installed).is_none() {
                    return;
                }
                self.retire_rewrite_inputs(&w, budget);
            } else {
                let pay = self.verified_install_deflation(addr, &w);
                if pay.iter().all(|&x| x == 0) {
                    // Closes nothing: installed, and mints none.
                    let _ = self.treasury.revoke_award(addr, crate::economy::escrow::Revocation::Drop);
                } else {
                    // 26 as ruled 2026-09-10: what this form closed is
                    // partly what the forms it CITED close inside it, and
                    // that part is theirs. The shares come out of this
                    // install's own mint before the escrow's voices take
                    // the remainder — never out of the citing voice's
                    // balance, which paid zero whenever its own draw
                    // already occupied the lane (L8t).
                    //
                    // MU-CLIENT §5.4 (D2′) — and the WORDS it taught take
                    // theirs after the words it cited: a held word's
                    // discoverer its use, a new word's its base pay. What the
                    // escrow's voices take after both is the COMPOSITION
                    // PREMIUM — what the sentence discharged beyond its words.
                    let mut shares = self.cited_shares(addr, &w, pay);
                    let word_shares_paid = word_shares.clone();
                    shares.extend(word_shares);
                    // MU-CLIENT §5.6 (S) — a sentence's voices take its
                    // GLUING: what its rings close beyond its words' own
                    // sections. A word's voices take what is left.
                    let premium = match crate::transducer::sentence::from_element_form(&w).is_some_and(|f| !f.faces.is_empty()) {
                        true => {
                            let words = crate::cognition::book::speech::words_of_sentence(&w, budget);
                            Some(self.gluing_of(addr, &w, &words))
                        }
                        false => None,
                    };
                    let taken = self.treasury.settle_recursive(addr, &shares, pay, premium, installed);
                    // The pays by kind, for the observer (MU-CLIENT §5.6).
                    let kind = match (premium.is_some(), w.is_connection(), self.conjugate_of(&w).is_some()) {
                        (true, _, _) => "sentence gluings",
                        (false, true, _) => "word sections",
                        (false, false, true) => "conjugate transports",
                        (false, false, false) => "",
                    };
                    if taken.iter().any(|&m| m > 0) {
                        self.acts.pay(kind, taken);
                    }
                    for (_, share) in &word_shares_paid {
                        self.acts.pay("word sections", *share);
                    }
                    self.grow_grants_by_paid();
                }
            }
        } else {
            let _ = self.treasury.revoke_award(addr, crate::economy::escrow::Revocation::Drop);
        }
        self.unlist(addr, false);
        self.leave_provisional(addr);
        self.court_upheld += 1;
        self.mark_persist();
        if let Some(w) = self.word_at(addr).cloned() {
            self.note_form_citations(addr, &w);
        }
        if let Some(bucket) = waiting {
            self.awaiting.insert(addr, bucket);
            self.rehear_awaiting(addr, budget);
        }
    }

    /// MU-CLIENT §5.4 (A, D2′) — THE WORDS AN INSTALLED SENTENCE TEACHES,
    /// INSTALLED BESIDE IT. Each word its closed rings teach
    /// (`words_of_sentence`) is read for what it discharges on the
    /// sentence's own family (`cited_deflation`). A word this court holds
    /// already is USED: its discoverer's share. A new word is INSTALLED on
    /// its own act — the same witness, its discoverer the sentence's speaker,
    /// bound to the sentence's task — and its BASE share is its speaker's. A
    /// word that discharges nothing takes no share (rule 11), and one with
    /// no speaker here is installed and paid to no one. Answers the shares,
    /// in the order the words were taught.
    fn install_taught_words(
        &mut self,
        addr: &[u8],
        w: &Element,
        drawn: Option<[u8; 32]>,
        witness: [u8; 32],
        budget: &mut crate::physics::Budget,
    ) -> Vec<([u8; 32], [u64; 4])> {
        let task = self.speech_task.get(addr).or_else(|| self.listing_task.get(addr)).cloned();
        let mut shares = Vec::new();
        for word in crate::cognition::book::speech::words_of_sentence(w, budget) {
            let at = word.address();
            let discharge = self.cited_deflation(addr, &word, &crate::transducer::alphabet::Word::identity());
            let who = match self.basis.get(at.as_bytes()).is_some() {
                true => self.discoverer_of(at.as_bytes()),
                false => {
                    // No speaker here is a fact, not a default (rule 11): the
                    // word is held with none, as `home_install` holds one.
                    #[expect(clippy::manual_unwrap_or_default, reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; the null identity is the stated fact of no speaker")]
                    let speaker = match drawn {
                        Some(v) => v,
                        None => [0; 32],
                    };
                    self.basis_push(word.clone(), speaker, budget);
                    if let Some(t) = &task {
                        self.speech_task.insert(&at, t.clone());
                    }
                    self.append_step(crate::chronometry::ActKind::Install, at.as_bytes(), witness);
                    self.fed_admit(at.as_bytes(), word);
                    drawn
                }
            };
            if let Some(who) = who.filter(|_| discharge.iter().any(|&m| m > 0)) {
                shares.push((who, discharge));
            }
        }
        shares
    }

    /// MU-CLIENT §5.4 (A), D3 — THE HELD SENTENCES DECOMPOSED FORWARD: every
    /// sentence this court's language holds brings the words it teaches into
    /// the language beside it, each on its own Install act witnessed by this
    /// court, discovered by the sentence's own discoverer and bound to its
    /// task — as an install brings them now. Paid nothing: each sentence was
    /// paid when it was installed. Never reverted, never recut, and held words
    /// are not installed twice, so a later boot finds nothing new. Each
    /// sentence is read on a stroke of this court's own metabolism (65.5).
    /// Answers how many words entered the language.
    pub fn decompose_held_sentences(&mut self) -> usize {
        let sentences: Vec<(Vec<u8>, Element)> = self
            .basis
            .scoped()
            .filter(|(_, h)| h.form().is_some_and(|f| !f.faces.is_empty()))
            .map(|(a, h)| (a, h.element().clone()))
            .collect();
        #[expect(clippy::manual_unwrap_or_default, reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a court with no voice of its own witnesses as the null identity, a stated fact")]
        let witness = match self.court_voice {
            Some(v) => v,
            None => [0; 32],
        };
        let mut added = 0usize;
        for (addr, w) in sentences {
            let drawn = self.discoverer_of(&addr);
            let mut budget = self.physics.stroke();
            let held = |k: &Kernel, word: &Element| k.basis.get(word.address().as_bytes()).is_some();
            let words = crate::cognition::book::speech::words_of_sentence(&w, &mut budget);
            let new = words.iter().filter(|word| !held(self, word)).count();
            let _ = self.install_taught_words(&addr, &w, drawn, witness, &mut budget);
            added += new;
        }
        if added > 0 {
            self.mark_persist();
        }
        added
    }

    /// 26 AS RULED 2026-09-10 — THE SHARE EACH GLUED FORM TAKES OF THIS
    /// INSTALL'S MINT: a `Glued` vertex citing a whole closed form. A word
    /// vertex's derivation citing a generator earns ROOM, not coin (held
    /// 2026-10-15): it is credited as a use and takes no share here.
    ///
    /// The share is the CITED FORM'S OWN local discharge on the
    /// citer's bound family — what that form closes, measured here on the
    /// family this sentence was spoken over, not a magnitude copied off
    /// its tape. A form whose verdict this court has not ruled takes
    /// nothing (1.7); a form with no discoverer here takes nothing, which
    /// is a fact and not a default (rule 11).
    ///
    /// ONE SHARE PER CITED VERDICT, not per citation: a form cited by two
    /// vertices closes what it closes once. Keyed by the cited address,
    /// so the second citation of one address is not a second share.
    fn cited_shares(&self, addr: &[u8], w: &Element, pay: [u64; 4]) -> Vec<([u8; 32], [u64; 4])> {
        if pay.iter().all(|&m| m == 0) {
            return Vec::new();
        }
        // WHO IS CITED IS READ OFF THE ENVELOPE (54.5 Road (d), B3). This
        // decoded the whole form to read its vertices for citations, and a
        // citation is not geometry — it is a 1-simplex of the nerve and a
        // vertex's presentation. Both ride `Element.receipts`, so the
        // economy's read is now a read of two flat lists and builds no form.
        let env = w.receipts();
        let mut cited: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut out = Vec::new();
        let take = |k: &mut crate::network::nerve::Keyed<()>,
                    out: &mut Vec<([u8; 32], [u64; 4])>,
                    form_addr: &crate::transducer::address::Address,
                    placement: &crate::transducer::alphabet::Word| {
            if k.contains(form_addr.as_bytes()) {
                return;
            }
            k.insert(form_addr.clone(), ());
            if !self.ruled_contains(form_addr.as_bytes()) {
                return;
            }
            let Some(held) = self.word_at(form_addr.as_bytes()).cloned() else {
                return;
            };
            let Some(who) = self.discoverer_of(form_addr.as_bytes()) else {
                return;
            };
            // The cited form's own deflation, measured on the CITER's
            // family: what it closes inside this sentence.
            let share = self.cited_deflation(addr, &held, placement);
            if share.iter().all(|&m| m == 0) {
                return;
            }
            out.push((who, share));
        };
        // A nerve edge names the placement it cited at: the rotational half
        // of the transition it is embedded under.
        for e in &env.nerve {
            take(&mut cited, &mut out, &e.target, &e.placement);
        }
        // 26 AS RULED 2026-09-10, held 2026-10-15 — A DERIVATION CITATION
        // EARNS ROOM, NOT COIN: a generator cited inside a word's derivation
        // contributes an action, and its discoverer's use is credited as
        // standing (`note_form_citations`, `use_credit`), never a share of the
        // citer's mint. Only a GLUED form — a closed form cited whole — takes
        // a share, its own local section on the citer's family. The local
        // measure (MU-CLIENT §5.6 W) reads the glued form and the taught
        // words; it does not turn a derivation's use into coin.
        out
    }

    /// What a cited form closes on the CITER's bound family. The citer's
    /// task names the family (`verified_install_deflation` reads the same
    /// binding for the citer itself), and the cited form is replayed over
    /// it: a generator that closes nothing there is used for nothing
    /// there, whatever it closes elsewhere.
    pub(crate) fn cited_deflation(
        &self,
        citer: &[u8],
        cited: &Element,
        placement: &crate::transducer::alphabet::Word,
    ) -> [u64; 4] {
        let Some(task) = self
            .speech_task
            .get(citer)
            .or_else(|| self.listing_task.get(citer))
            .cloned()
        else {
            return [0; 4];
        };
        let Some(family) = self.task_family.get(&task) else {
            return [0; 4];
        };
        // MU-CLIENT §5.6 (W) — a connection is decompressed by its own
        // letters and measured on its own section of the citer's family; the
        // form decompressor never reached it.
        if cited.is_connection() && !cited.is_colour_word() {
            return crate::transducer::family::local::word_local(cited, family).pay;
        }
        // A CITED FORM'S USE IS ITS LOCAL DISCHARGE too (§5.6 W: "D2′'s base
        // and use shares read it"): what it closes inside the citer at the
        // citer's placement, its section, never the whole-pair closure a
        // cited word inside a sentence rarely meets alone.
        match crate::transducer::sentence::from_element_form(cited) {
            Some(f) => crate::transducer::family::local::form_local(&f, family, self, placement).pay,
            None => [0; 4],
        }
    }

    pub(crate) fn note_form_citations(&mut self, addr: &[u8], w: &Element) {
        // BOTH READINGS COME OFF THE ENVELOPE (54.5 Road (d), B4). This
        // decoded the form to read it for citations and for certificates,
        // and NEITHER is geometry: a citation is a 1-simplex of the nerve
        // and a certificate is what one mind's stroke did against one
        // budget. The ledger the court keeps of who cited whom, and the
        // memo of which pivots it has certified, are both about the
        // SPEAKER, so they are read from the speaker's own attestation.
        let env = w.receipts();
        // 26 as ruled 2026-09-10: BOTH LEVELS are uses. One entry per citer
        // address per cited address, so a form cited by two vertices of one
        // sentence is one use of one verdict and two sentences citing it are
        // two — which is what `use_credit` counts.
        for address in env.cited_addresses() {
            match self.cited_by.get_mut(&address) {
                Some(bucket) => {
                    bucket.insert(addr, ());
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(addr, ());
                    self.cited_by.insert(address, bucket);
                }
            }
        }
        for cert in &env.certificates {
            let k = cert.key();
            match self.replay_memo.get_mut(addr) {
                Some(bucket) => {
                    bucket.insert(k, (true, self.chain.head_place()));
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(k, (true, self.chain.head_place()));
                    self.replay_memo.insert(addr, bucket);
                }
            }
        }
    }

    pub(crate) fn redocket_citers_of(&mut self, cited: &[u8]) {
        let Some(bucket) = self.cited_by.remove(cited) else {
            return;
        };
        let citers: Vec<Vec<u8>> = bucket
            .scoped()
            .map(|(k, _)| k)
            .collect();
        for c in citers {
            if let Some(w) = self.word_at(&c).cloned() {
                // Back on the docket, and its orbit opens again: its ruling
                // stood on the word just slashed, so a second vantage must
                // re-derive it (47.3).
                self.provisional.insert(&c, w);
                self.enter_offered(&c);
            }
        }
    }
}
