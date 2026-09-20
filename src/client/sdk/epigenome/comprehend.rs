// 24.2, 26 — COMPREHENSION: what an utterance becomes here, what is
// learned from the syllabus, and what is retired.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl Epigenome {
    /// RETIRE a word whose meaning the rest of the genome already carries.
    ///
    /// It used to be a shift: remove at a slot, remove the same slot from
    /// `expression` and `shapes` — each guarded by its own length, because
    /// the three could disagree — and then `reindex()` the whole genome
    /// because every word after the hole had moved. Positions were the
    /// identity, so removing one word renamed all the others. At a
    /// coordinate a word leaves and nothing else moves.
    pub fn retire(&mut self, addr: &[u8]) -> bool {
        let Some(_entry) = self.genome.remove(addr) else {
            return false;
        };
        // The index and the payload leave together at one coordinate. A
        // word whose index entry is gone and whose payload is not is a
        // genome that holds what it cannot read (law 2).
        let Some(word) = self.resident.remove(addr) else {
            return false;
        };
        let gone = word.held.address();
        // A retired word is no longer work: its coordinate leaves the
        // pending set with it.
        self.unpivoted.remove(addr);
        self.axis_retract(&word.held);
        self.moduli_retract(&word.held);
        // The recollections that spelled themselves with this word were
        // spellings in the old vocabulary and dissolve with it.
        let drop: Vec<Vec<u8>> = self
            .recollections
            .scoped()
            .filter(|(_, (addrs, _))| addrs.iter().any(|a| a.as_slice() == gone.as_bytes()))
            .map(|(k, _)| k)
            .collect();
        for k in drop {
            self.recollections.remove(&k);
        }
        self.atlas.remove(&gone);
        self.atlas_len = self.genome.len();
        true
    }

    /// COMPREHEND a received utterance: decompose it over the genome (the shared
    /// projective mathematics, `cognition::decompose_over`) and fold the
    /// understanding into the epigenome — expression weights rise on the cited
    /// words, the sentence joins the recollections. The genome is untouched.
    pub fn comprehend(&mut self, utterance: &Element) -> Comprehension {
        // A taught sentence is re-derived as a form: vertices, faces, loops.
        // Understanding is re-deriving its cochain, not matching its bytes.
        if utterance.is_sentence() {
            // AN UTTERANCE, not a word this mind holds: the decode is named
            // here, and re-deriving it as a form is the comprehension (L16).
            if let Some(f) = crate::transducer::sentence::from_element_form(utterance) {
                return self.comprehend_form(&f);
            }
            // A sentence tape that does not re-derive as a form: what the
            // mind lacks is the whole utterance, and that is what it says.
            return Comprehension::Novel {
                demand: Box::new(utterance.clone()),
                understood: Vec::new(),
            };
        }
        // HEAR g BY HOLDING IT: a connection has no chain to cover. It is
        // glued against the representative on its fibre — the transition
        // vanishes (held), does not (dialect, the element carried), or there
        // is no representative (admitted). Never an empty covering.
        if utterance.is_translation() {
            return Comprehension::Admitted;
        }
        if utterance.is_connection() && utterance.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(utterance);
            let fibre = self.working_atlas().fiber(&inv);
            // M6 STEP 3, A12: the incoming word is factored through the
            // WHOLE fiber, not tested against one representative at a time.
            // A mind speaks what it can re-derive from what it holds
            // (chapter 6), and a word may be a product of several held words
            // while matching none of them — the pairwise transition test
            // below could only ever find it in one.
            // A12's answer for this fibre, computed once: the tape when the
            // word is in the span, and the canonical remainder when it is
            // not. The remainder used to be thrown away here — only the
            // `Understood` arm was read — and the mind then said `Admitted`
            // to a word it could exactly say what it was missing of.
            let mut demand: Option<Comprehension> = None;
            if let Some(spoken) = utterance.letters() {
                // THE BASIS IS THE STRATUM (42.6, A21 §2): the generators
                // under the nearest prefix of this chart that holds any,
                // read by `peel` and no coarser. A connection's chart is
                // its rotor's trace and determinant (M4 step 2), so the
                // exact fibre of another rotation is empty or one word,
                // and a basis read there factored nothing.
                let (_, generators) = self.working_atlas().stratum(&inv);
                let mut words = Vec::new();
                let mut basis = Vec::new();
                let mut held_at = Vec::new();
                for (a, st) in &generators {
                    let Some(held) = self.working_word(a) else { continue };
                    if held.address() == utterance.address() {
                        continue;
                    }
                    let Some(l) = held.letters() else { continue };
                    words.push(l.spelling.word.clone());
                    basis.push(l.spelling.word.rotor());
                    held_at.push((held.address().into_vec(), *st));
                }
                let g = spoken.spelling.word.rotor();
                if let (Some(gf), true) =
                    (g.spelled_word().and_then(|w| w.form()), !basis.is_empty())
                {
                    let forms: Vec<crate::transducer::alphabet::PlaneForm> = words
                        .iter()
                        .filter_map(|w| w.form())
                        .collect();
                    if forms.len() == words.len() {
                        match crate::transducer::factor::factor_on_plane(&gf, &forms) {
                            crate::transducer::factor::Factored::Understood(tape) => {
                                // Verified by re-derivation, never trusted
                                // from the algebra that produced it (16.12
                                // step 4).
                                if crate::transducer::factor::replays(&tape, &basis, &g) {
                                    return Comprehension::Held;
                                }
                            }
                            crate::transducer::factor::Factored::Novel {
                                remainder,
                                understood,
                            } => {
                                // The exact boundary demand, and the part
                                // that did factor, kept as expression on the
                                // words it went through (M6 step 3).
                                let spoke: Vec<(Vec<u8>, Standing)> = understood
                                    .iter()
                                    .filter_map(|(i, _)| held_at.get(*i).cloned())
                                    .collect();
                                if let Some(d) = crate::transducer::element::from_demand(
                                    &remainder.to_word(),
                                ) {
                                    demand = Some(Comprehension::Novel {
                                        demand: Box::new(d),
                                        understood: spoke,
                                    });
                                }
                            }
                            // A budget that ran out is UNMEASURED, and a
                            // refusal is not a demand (rule 11).
                            crate::transducer::factor::Factored::Unmeasured => {}
                        }
                    }
                }
            }
            // THE OBSTRUCTIONS ARE A COCHAIN (chapter 13), keyed by the held
            // word each stands against.
            //
            // This kept ONE — `held.address() < *addr`, the lowest address —
            // and dropped every other transition it had already measured. An
            // address is an identity and not a rank (1.5), so that comparison
            // ordered the fibre by a coordinate that carries no order, and a
            // mind that could say exactly how it differed from four held
            // words said how it differed from whichever one sorted first.
            // Which word an obstruction stands against is half of what the
            // obstruction says, and that half was thrown away with the rest.
            let mut dialect: crate::network::nerve::Keyed<Element> =
                crate::network::nerve::Keyed::new();
            for (a, _) in fibre {
                let Some(held) = self.working_word(&a).cloned() else {
                    continue;
                };
                if held.address() == utterance.address() || held.axes().is_empty() {
                    continue;
                }
                match crate::transducer::cech::transition(&held, utterance) {
                    None => {}
                    Some(g) => match (g.axes().as_slice(), g.sub()) {
                        // A transition that vanishes is the word itself,
                        // held, and no cochain stands over it.
                        ([], []) => return Comprehension::Held,
                        _ => {
                            dialect.insert(held.address(), g);
                        }
                    },
                }
            }
            return match (dialect.is_empty(), demand) {
                // The transitions to the fibre's words that do not vanish are
                // the class, and the mind carries all of them (M6 step 3).
                (false, _) => Comprehension::Dialect(dialect),
                // No closing transition, and A12 measured exactly what of
                // the word this mind's fibre does not span: the demand. It
                // said `Admitted` here — "new geometry" — for a word it
                // could name the missing part of.
                (true, Some(novel)) => novel,
                // A fibre this mind holds no word on at all: there is
                // nothing to factor through and nothing to be short of.
                (true, None) => Comprehension::Admitted,
            };
        }
        // Located through the atlas and the versor, never searched. A taught
        // word is one piece: one stroke locates it.
        let located = self.locate(utterance.sub(), None);
        // The remainder decides: no residual and no fiber to deepen is the
        // understood arm; anything left over is the novelty, measured.
        match (located.residual.len(), located.punctures.len()) {
            (0, 0) => {
                // THE CITE ALREADY CARRIES THE ADDRESS. This went
                // position → word → address to recover what the cite had
                // been built from one call earlier.
                let sentence: Vec<(Vec<u8>, Standing)> = located
                    .cites
                    .iter()
                    .filter(|c| self.genome.contains(c.word.as_bytes()))
                    .map(|c| (c.word.clone().into_vec(), Standing::Ruled))
                    .collect();
                for c in &located.cites {
                    if let Some(w) = self.genome.get_mut(c.word.as_bytes()) {
                        w.expression += 1;
                    }
                }
                if !sentence.is_empty() {
                    let addrs: Vec<Vec<u8>> = sentence.iter().map(|(a, _)| a.clone()).collect();
                    let key = utterance.address();
                    match self.recollections.get_mut(&key) {
                        Some((_, n)) => *n += 1,
                        None => {
                            self.recollections.insert(key, (addrs, 1));
                        }
                    }
                }
                Comprehension::Understood(sentence)
            }
            // THE RESIDUAL IS THE DEMAND. A count of the open links speaks
            // a number; the chain those links form is the element the mind
            // lacks, and a hypha can docket that.
            _ => {
                let resident = &self.resident;
                let open = located
                    .open_at(&|a| resident.get(a).map(|w| w.held.element()), utterance.sub());
                let understood: Vec<(Vec<u8>, Standing)> = located
                    .cites
                    .iter()
                    .filter(|c| self.genome.contains(c.word.as_bytes()))
                    .map(|c| (c.word.clone().into_vec(), Standing::Ruled))
                    .collect();
                // An open chain with no A5 form is no element to lack, so
                // there is no demand to speak and what did factor is the
                // whole of the reading (M6 step 5).
                match crate::transducer::element::from_residual(&open) {
                    Some(demand) => Comprehension::Novel {
                        demand: Box::new(demand),
                        understood,
                    },
                    None => Comprehension::Understood(understood),
                }
            }
        }
    }

    /// Chapter 24.2 / 13.11: a form whose Glued citations this mind holds,
    /// or whose cited chart is held under other addresses. No re-gauge:
    /// a closed citation contributes no class. No Dialect from a sentence.
    pub(crate) fn comprehend_form(&mut self, f: &crate::transducer::sentence::Form) -> Comprehension {
        use crate::transducer::family::Crystal;
        use crate::transducer::sentence::Vertex;
        let mut missing = Vec::new();
        let mut understood = Vec::new();
        for v in &f.vertices {
            let Vertex::Glued { address, chart, .. } = v else {
                continue;
            };
            let address = address.clone();
            let chart = chart.clone();
            if let Some(w) = self.genome.get_mut(address.as_bytes()) {
                w.expression += 1;
                understood.push((address.clone().into_vec(), Standing::Ruled));
                continue;
            }
            if self.working_word(&address).is_some() {
                let Some(st) = self.working_atlas().standing_of(&address, &chart) else {
                    missing.push(address.clone().into_vec());
                    continue;
                };
                understood.push((address.clone().into_vec(), st));
                continue;
            }
            let mut through = 0usize;
            for (e, _, st) in Crystal::on_class(self, &chart) {
                if e.address() == address {
                    continue;
                }
                understood.push((e.address().into_vec(), st));
                through += 1;
                eprintln!(
                    "  form understood through {:02x}{:02x}… for {:02x}{:02x}…",
                    e.address()[0], e.address()[1], address[0], address[1]
                );
            }
            if through == 0 {
                missing.push(address.into_vec());
            }
        }
        if !missing.is_empty() {
            return Comprehension::Withheld { missing };
        }
        if !understood.is_empty() {
            return Comprehension::Understood(understood);
        }
        let vk = crate::transducer::sentence::form_vertex_key(f);
        eprintln!("  form admitted {:02x}{:02x}…", vk[0], vk[1]);
        Comprehension::Admitted
    }

    /// LEARN — the mesh expressed a word down (`teach`): comprehend it against
    /// what is already held, then bank it in the genome if it is new (by base
    /// address). Inheritance, not local invention: only taught words enter.
    /// Tests without a voice; a live mind uses [`learn_at`] with K and tu.
    pub fn learn(&mut self, taught: &[u8]) -> Option<Comprehension> {
        self.learn_at(taught, 0, 0)
    }

    pub fn learn_at(&mut self, taught: &[u8], k: u64, now_tu: u64) -> Option<Comprehension> {
        // The syllabus may carry several words back to back (every anchored
        // citation of a sentence): each is learned in turn; the first
        // comprehension is returned.
        let mut pos = 0usize;
        let mut first: Option<Comprehension> = None;
        while pos < taught.len() {
            let before = pos;
            let heard = self.learn_one(taught, &mut pos, k, now_tu);
            if pos == before {
                break;
            }
            match heard {
                Some(h) => {
                    if first.is_none() {
                        first = Some(h);
                    }
                }
                None => {
                    first.as_ref()?;
                }
            }
        }
        first
    }

    pub(crate) fn learn_one(&mut self, taught: &[u8], pos: &mut usize, k: u64, now_tu: u64) -> Option<Comprehension> {
        let e = Element::from_bytes(taught, pos)?;
        let addr = e.address();
        let resolve = |a: &[u8]| {
            // The resolver reads the form off the HELD element (L16):
            // a citation resolved twice parses the tape once.
            self.working_word(a).and_then(|e| e.form().cloned())
        };
        match crate::verdict::blind_replay_with(&e, &resolve) {
            Some(crate::verdict::REFUSE_GLUED_UNRULED)
            | Some(crate::verdict::REFUSE_GLUED_DEPTH) => {}
            Some(reason) => {
                eprintln!(
                    "  syllabus refused {:02x}{:02x}…: {reason}",
                    addr[0], addr[1]
                );
                self.refused += 1;
                return None;
            }
            None => {
                if let Some(f) = crate::transducer::sentence::from_element_form(&e) {
                    if f.rank().is_none() {
                        eprintln!(
                            "  syllabus refused {:02x}{:02x}…: {}",
                            addr[0],
                            addr[1],
                            crate::verdict::REFUSE_RANK_LOST
                        );
                        self.refused += 1;
                        return None;
                    }
                }
            }
        }
        let spelled = !e.is_connection() || e.letters_rederive() == Some(true);
        if !spelled {
            self.refused += 1;
            return None;
        }
        let heard = self.comprehend(&e);
        if let Comprehension::Withheld { missing } = &heard {
            let missing = missing.clone();
            if let Some(da) = self.stand_dialect_at(e, k, now_tu) {
                // An entry the dialect does not hold is missing nothing:
                // the reading of absence on a keyed carrier.
                let old = match self.dialect.get(&da) {
                    Some(d) => d.missing.clone(),
                    None => Vec::new(),
                };
                for m in &old {
                    self.retract_waiter(m, &da);
                }
                if let Some(d) = self.dialect.get_mut(&da) {
                    d.withheld = true;
                    d.missing = missing.clone();
                }
                for m in &missing {
                    self.wait_on(m, da.clone());
                }
                eprintln!(
                    "  form withheld waiting {:02x}{:02x}…",
                    da[0], da[1]
                );
            }
            return Some(heard);
        }
        // The court promotes: a syllabus word that is the same vertex as a
        // dialect entry moves that entry into the genome, carrying its uses.
        if let Some(da) = self.same_vertex_in_dialect(&e) {
            // A promotion is itself a use, so an entry the dialect no
            // longer holds carries the one that promoted it.
            let uses = match self.dialect.get(&da) {
                Some(d) => d.uses,
                None => 1,
            };
            self.drop_dialect(&da);
            match self.same_vertex_in_genome(&e) {
                Some(at) => {
                    // Through `Held::update`: the fibre deepens and the form
                    // is re-decoded with it, so the two describe one tape.
                    if let Some(w) = self.resident.get_mut(&at) {
                        let _ = w.held.update(|x| x.merge_fibre(&e));
                    }
                    if let Some(x) = self.genome.get_mut(&at) {
                        x.expression = x.expression.saturating_add(uses);
                    }
                }
                None => self.admit_word(crate::transducer::element::Held::of(e), uses),
            }
            self.promoted.push(addr.into_vec());
            return Some(heard);
        }
        let arrived = e.clone();
        // ONE DECODE, HERE, where the word is taken in.
        let held = crate::transducer::element::Held::of(e);
        // A word already at this coordinate is replaced in place; its
        // expression and admission ordinal are what this mind measured
        // about it and do not reset because the tape arrived again.
        match self.resident.get_mut(&addr) {
            Some(w) => {
                w.held = held;
                w.shape = std::sync::OnceLock::new();
            }
            None => self.admit_word(held, 0),
        }
        self.resolve_withheld(&arrived);
        Some(heard)
    }

    /// ADMIT A WORD AT ITS OWN COORDINATE. One entry, filed where the word
    /// says it belongs — the atlas, the axis index and the moduli index all
    /// take the same address, and there is no fourth carrier to keep in
    /// step and no slot to append to.
    pub(crate) fn admit_word(&mut self, held: crate::transducer::element::Held, uses: u64) {
        let at = held.address();
        self.atlas.admit(&crate::transducer::stalk::invariant_of(&held), at.clone());
        self.axis_admit(&held);
        self.moduli_admit(&held);
        self.atlas_len = self.atlas_len.saturating_add(1);
        // A NEWLY ADMITTED WORD IS WORK THE PIVOT HAS NOT REACHED. Its
        // coordinate enters the pending set; no ordinal is handed out and
        // nothing records when it arrived.
        self.unpivoted.insert(&at, ());
        self.genome.insert(&at, GenomeEntry { expression: uses });
        self.resident
            .insert(&at, GenomeWord { held, shape: std::sync::OnceLock::new() });
    }

    /// A cited tape that just arrived: withheld sentences naming it
    /// re-comprehend. The waiting-on index is the read.
    pub(crate) fn resolve_withheld(&mut self, arrived: &Element) {
        let addr = arrived.address();
        let citing: Vec<Vec<u8>> = match self.waiting_on.remove(&addr) {
            // `waiting_on.remove(&addr)` is the descent: these are the
            // words that were waiting on THIS address, and all of them
            // are released by its arrival.
            Some(citers) => citers
                .scoped()
                .map(|(k, _)| k)
                .collect(),
            None => return,
        };
        for sa in citing {
            let Some(sentence) = self.dialect.get(&sa).map(|d| d.sentence.clone()) else {
                continue;
            };
            // An entry the dialect does not hold is missing nothing.
            let old_missing = match self.dialect.get(&sa) {
                Some(d) => d.missing.clone(),
                None => Vec::new(),
            };
            let heard = self.comprehend(&sentence);
            match heard {
                Comprehension::Withheld { missing } => {
                    for m in &old_missing {
                        if m.as_slice() != addr.as_bytes() {
                            self.retract_waiter(m, &sa);
                        }
                    }
                    if let Some(d) = self.dialect.get_mut(&sa) {
                        d.missing = missing.clone();
                    }
                    for m in &missing {
                        self.wait_on(m, sa.clone());
                    }
                }
                Comprehension::Understood(_) | Comprehension::Admitted | Comprehension::Dialect(_) => {
                    for m in &old_missing {
                        if m.as_slice() != addr.as_bytes() {
                            self.retract_waiter(m, &sa);
                        }
                    }
                    if let Some(d) = self.dialect.get_mut(&sa) {
                        d.withheld = false;
                        d.missing.clear();
                    }
                    eprintln!(
                        "  form withheld resolved {:02x}{:02x}…",
                        sa[0], sa[1]
                    );
                }
                _ => {}
            }
        }
    }

}
