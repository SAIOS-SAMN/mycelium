// 24.2, 26 — COMPREHENSION: what an utterance becomes here, what is
// learned from the syllabus, and what is retired.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl Epigenome {
    pub fn retire(&mut self, addr: &[u8]) -> bool {
        let Some(i) = self.index.get(addr).copied() else {
            return false;
        };
        let gone = self.genome[i].address();
        let held = self.genome[i].clone();
        self.axis_retract(&held);
        self.genome.remove(i);
        if i < self.expression.len() {
            self.expression.remove(i);
        }
        let drop: Vec<Vec<u8>> = self
            .recollections
            .fiber(crate::network::nerve::ROOT)
            .filter(|(_, (addrs, _))| addrs.iter().any(|a| a.as_slice() == gone.as_bytes()))
            .map(|(k, _)| k)
            .collect();
        for k in drop {
            self.recollections.remove(&k);
        }
        self.atlas.remove(&gone);
        if i < self.shapes.len() {
            self.shapes.remove(i);
        }
        self.atlas_len = self.genome.len();
        self.reindex();
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
            if let Some(f) = crate::transducer::sentence::from_element_form(utterance) {
                return self.comprehend_form(&f);
            }
            return Comprehension::Novel { residual_cells: utterance.sub().len() };
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
            if let Some(spoken) = utterance.letters() {
                let mut words = Vec::new();
                let mut basis = Vec::new();
                for (a, _) in &fibre {
                    let Some(held) = self.working_word(a) else { continue };
                    if held.address() == utterance.address() {
                        continue;
                    }
                    let Some(l) = held.letters() else { continue };
                    words.push(l.spelling.word.clone());
                    basis.push(l.spelling.word.rotor());
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
                        if let crate::transducer::factor::Factored::Understood(tape) =
                            crate::transducer::factor::factor_on_plane(&gf, &forms)
                        {
                            // Verified by re-derivation, never trusted from
                            // the algebra that produced it (16.12 step 4).
                            if crate::transducer::factor::replays(&tape, &basis, &g) {
                                return Comprehension::Held;
                            }
                        }
                    }
                }
            }
            let mut dialect: Option<(crate::transducer::a21::Address, Element)> = None;
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
                        ([], []) => return Comprehension::Held,
                        _ => match &dialect {
                            None => dialect = Some((held.address(), g)),
                            Some((addr, _)) if held.address() < *addr => {
                                dialect = Some((held.address(), g));
                            }
                            _ => {}
                        },
                    },
                }
            }
            return match dialect {
                None => Comprehension::Admitted,
                Some((_, g)) => Comprehension::Dialect(g),
            };
        }
        // Located through the atlas and the versor, never searched. A taught
        // word is one piece: one stroke locates it.
        let located = self.locate(utterance.sub(), None);
        // The remainder decides: no residual and no fiber to deepen is the
        // understood arm; anything left over is the novelty, measured.
        match (located.residual.len(), located.punctures.len()) {
            (0, 0) => {
                let sentence: Vec<(Vec<u8>, Standing)> = located
                    .cites
                    .iter()
                    .filter_map(|(i, _, _)| {
                        let w = self.genome.get(*i)?;
                        Some((w.address().into_vec(), Standing::Ruled))
                    })
                    .collect();
                for (i, _, _) in &located.cites {
                    if let Some(x) = self.expression.get_mut(*i) {
                        *x += 1;
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
            _ => Comprehension::Novel { residual_cells: located.open(self.genome(), utterance.sub()).len() },
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
            if let Some(&i) = self.index.get(&address) {
                if let Some(x) = self.expression.get_mut(i) {
                    *x += 1;
                }
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
            self.working_word(a)
                .and_then(crate::transducer::sentence::from_element_form)
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
        // The APERTURE tunes on what is inherited: each transition refracts a
        // value signal into h[0]; each torsion period a spatial signal into h[1]
        // — the entity's optics drift with its diet (dialect-legitimate).
        for &(_, _, before, after) in e.sub() {
            self.aperture.absorb_value_signal(before, after);
        }
        let ft = crate::transducer::sentence::form_torsion(&e);
        if !ft.is_empty() {
            for (_, (_, l, _)) in ft.fiber(crate::network::nerve::ROOT) {
                self.aperture.absorb_spatial_signal(*l);
            }
        } else {
            for (_, l, _) in e.torsion() {
                self.aperture.absorb_spatial_signal(*l);
            }
        }
        let heard = self.comprehend(&e);
        if let Comprehension::Withheld { missing } = &heard {
            let missing = missing.clone();
            if let Some(da) = self.stand_dialect_at(e, k, now_tu) {
                let old = self
                    .dialect
                    .get(&da)
                    .map(|d| d.missing.clone())
                    .unwrap_or_default();
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
            let uses = self.dialect.get(&da).map(|d| d.uses).unwrap_or(1);
            self.drop_dialect(&da);
            if let Some(i) = self.same_vertex_in_genome(&e) {
                let _ = self.genome[i].merge_fibre(&e);
                if let Some(x) = self.expression.get_mut(i) {
                    *x = x.saturating_add(uses);
                }
            } else {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.axis_admit(&e);
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(uses);
            }
            self.promoted.push(addr.into_vec());
            return Some(heard);
        }
        let arrived = e.clone();
        match self.index.get(&addr).copied() {
            Some(i) => self.genome[i] = e,
            None => {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.axis_admit(&e);
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(0);
            }
        }
        self.resolve_withheld(&arrived);
        Some(heard)
    }

    /// A cited tape that just arrived: withheld sentences naming it
    /// re-comprehend. The waiting-on index is the read.
    pub(crate) fn resolve_withheld(&mut self, arrived: &Element) {
        let addr = arrived.address();
        let citing: Vec<Vec<u8>> = match self.waiting_on.remove(&addr) {
            Some(citers) => citers
                .fiber(crate::network::nerve::ROOT)
                .map(|(k, _)| k)
                .collect(),
            None => return,
        };
        for sa in citing {
            let Some(sentence) = self.dialect.get(&sa).map(|d| d.sentence.clone()) else {
                continue;
            };
            let old_missing = self
                .dialect
                .get(&sa)
                .map(|d| d.missing.clone())
                .unwrap_or_default();
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
