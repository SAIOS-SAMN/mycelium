// 18.2 — THE FORM: vertices, faces, loops, and the reads that walk them.
//
//! A face is constant in the word between its two vertices; a loop is a
//! cycle of faces; the holonomy of a loop is the composite around it. All
//! three are structure, walked and never searched.

use super::*;

impl Form {
    /// THE WORDS A SENTENCE IS MADE OF — one holonomy per loop, with the
    /// class it belongs to.
    ///
    /// A sentence has no single spelling: it is a NERVE of germs whose
    /// vertices carry words and whose faces carry the conjugations between
    /// them (18.2). Asking it for `Element::letters` is a level error —
    /// letters spell a word, and a sentence is one level above a word. What
    /// it does have is a word per loop, its holonomy, already composed on
    /// the carrier, and the class that word belongs to.
    ///
    /// Order is a CONJUGACY invariant (15.2, 15.3): "closes in p steps" is a
    /// fact about the class, never about the spelling, which is why the
    /// class travels beside the word here and not the tape.
    pub fn loop_words(&self) -> Vec<(&crate::transducer::alphabet::Word, Option<ClassVector>)> {
        (0..self.loops.len())
            .filter_map(|i| self.loops.get(i).map(|lp| (&lp.holonomy, self.class_vector(i))))
            .collect()
    }
}

/// How an unfolding stops. The step count is an instance coordinate, never
/// stored on the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: u16 },
}

/// A FILED LOOP: a torsion cycle of a FORM, keyed by the class of the loop
/// that carries it. The same three members as an element's `TorsionCycle`
/// with one difference that matters — the third is the CHORD the loop closes
/// on, an index into `faces`, not a fibre index. As one tuple the two were
/// interchangeable by type and only ever kept apart by which file was being
/// read; `Element::adopt_torsion` silently dropped the chord and wrote
/// `FIBER_SPATIAL` in its place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiledLoop {
    /// The hue-map generator: the permutation one step around the loop.
    pub sigma: Vec<(u8, u8)>,
    /// The period it closes at — the order of the loop's holonomy.
    pub period: u32,
    /// The chord it closes on: an index into the form's faces.
    pub chord: u16,
}

/// A vertex of a sentence form (chapter 18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vertex {
    Frame,
    Word {
        address: crate::transducer::address::Address,
        /// What this vertex IS (50.2 step 2, law 12): the generators it
        /// factored through and the residue A22 peeled, the word read
        /// back with `derivation.word()`.
        derivation: crate::transducer::alphabet::Derivation,
        /// A4 carriage. Boxed: a versor is 1024 bytes and every vertex of a
        /// form — Glued, Rebased, Frame — would carry that footprint
        /// whether or not it holds one.
        carriage: Box<Versor>,
        gauge: Vec<(u8, u8)>,
        class: stalk::Invariant,
        /// Stop ring. A form coordinate with the carriage and gauge; not M1.
        contact: Option<alphabet::Contact>,
    },
    Glued {
        address: crate::transducer::address::Address,
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,
    },
    /// Vertex `of` read in its own frame as `by`'s word (3.2): the linear
    /// word is `by`'s, realised at `placement_by + shift`; the body it
    /// speaks to, its hue map and its dilation stay `of`'s own, so a
    /// rebased body keeps its shape, its colours and its scale.
    Rebased {
        by: u16,
        of: u16,
        /// Relative carriage t = C_of · C_by⁻¹.
        carriage: Box<Versor>,
        /// `of`'s hue map, spoken on `of`'s bodies.
        gauge: Vec<(u8, u8)>,
        /// `of`'s chart: the section coordinate naming the bodies it speaks to.
        class: stalk::Invariant,
    },
    Unfolded { measure: Measure },
    /// 50.5: what the speaker could not spell or the eyes could not pair,
    /// one vertex per (kind, chart), carrying the bodies it stands for as
    /// the sentence's remainder. It has no word and reconstructs nothing.
    Demand {
        kind: crate::transducer::sentence::Demand,
        chart: stalk::Invariant,
        bodies: Vec<Cell>,
    },
}

/// Face of a `Form`: two vertices, the transport between them (the
/// coboundary of their placements, 3.1), and the edge transport of 50.3 —
/// the stabilizer element or exchange the pair's rigid motion picks out
/// relative to the body's own; `None` is a remainder edge, a pair no
/// single versor carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub i: u16,
    pub j: u16,
    pub transport: Transport,
    pub edge: Option<Transport>,
}

impl FormFace {
    pub fn new(i: u16, j: u16, word: Word) -> FormFace {
        FormFace {
            i,
            j,
            transport: Transport::from_word(word),
            edge: None,
        }
    }
}

/// Independent loop of a form: the closed vertex walk, the faces along it,
/// the chord (never a star face), and the oriented holonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop {
    pub vertices: Vec<u16>,
    pub faces: Vec<u16>,
    pub chord: u16,
    pub holonomy: Word,
    pub carriage: Versor,
}


/// A sentence as vertices, faces, loops, remainder.
#[derive(Debug, Clone)]
pub struct Form {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<FormFace>,
    pub loops: Vec<Loop>,
    pub remainder: Vec<Cell>,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,
    pub ratio: ((usize, usize), (usize, usize)),
    pub sections: Keyed<Vec<u16>>,
    /// The certified pivots this form's crystallization rests on (A20).
    /// Evidence, replayed by the court; never part of the address.
    pub certificates: Keyed<PivotCertificate>,
    /// Filed torsion cycles, keyed by class key: `(σ, ℓ, chord face)`.
    /// The form owns this carrier; the element reads it whole.
    pub torsion: Keyed<FiledLoop>,
    /// CHART AXES OPENED OVER HELD LOOPS (3.7, 15.5, 16.16 step 3; I6):
    /// the holonomy class opened on → the chord of the loop that is its
    /// section. The relation is all the form carries; the seed versor and
    /// A7's coordinate are re-derived from that loop on every read (law
    /// 12), never stored beside it. On the tape and in the address.
    pub axes: Keyed<u16>,
    /// Cited form address → glued vertex index. Written by [`Form::glue`]
    /// and rebuilt by the decoder from Glued vertices.
    pub glued: Keyed<u16>,
    /// Incomparable discharges held as cells until a glue or ruling.
    /// Outer key is the after class; inner is the discharging rotor A1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// Word vertex address → index. The meet for A6 and Mayer–Vietoris.
    pub by_address: Keyed<u16>,
    /// Chart key → the Word vertices on that chart (18.2): the fiber a
    /// listing body's chart reads when a cite asks what this form holds
    /// there (24.1 as ruled 2026-09-09). Derived by `reindex`, never on
    /// the tape.
    pub by_class: Keyed<Vec<u16>>,
    /// Vertex index key → Frame face index. Star incidence, one face per vertex.
    pub star: Keyed<u16>,
    /// Word vertex address → carriage, filled at construction.
    pub carriages: Keyed<Versor>,
    /// Rebased vertex keyed by the `of` Word address.
    pub by_rebased: Keyed<u16>,
    /// Unfolded vertex keyed by its measure key.
    pub by_unfolded: Keyed<u16>,
    /// Demand vertex keyed by (kind, chart) (50.5): the read that keeps a
    /// sentence to one demand vertex per pair. Written by `reindex`.
    pub demands: Keyed<u16>,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            vertices: vec![Vertex::Frame],
            faces: Vec::new(),
            loops: Vec::new(),
            remainder: Vec::new(),
            depth: 0,
            hue_maps: Keyed::new(),
            ratio: ((1, 1), (1, 1)),
            sections: Keyed::new(),
            certificates: Keyed::new(),
            torsion: Keyed::new(),
            axes: Keyed::new(),
            glued: Keyed::new(),
            vortex: Keyed::new(),
            by_address: Keyed::new(),
            by_class: Keyed::new(),
            star: Keyed::new(),
            carriages: Keyed::new(),
            by_rebased: Keyed::new(),
            by_unfolded: Keyed::new(),
            demands: Keyed::new(),
        }
    }
}

impl Form {
    /// The loop closing on `chord`, by index. A walk of this form's own
    /// loops — its boundary, never a search over anything else.
    pub(crate) fn loop_of_chord(&self, chord: u16) -> Option<usize> {
        for (i, lp) in self.loops.iter().enumerate() {
            if lp.chord == chord {
                return Some(i);
            }
        }
        None
    }

    /// The chord of the loop this loop's class was opened on, if any (I6).
    pub fn opened_axis_of(&self, lp: &Loop) -> Option<u16> {
        let class = lp.holonomy.rotor().class_key()?;
        self.axes.get(&class).copied()
    }

    /// Law 12: the opened axis is a theorem the loop proves — the loop is
    /// the one the relation names, and its holonomy is a versor, so the
    /// seed (A20 `OpenAxis`) re-derives from it. Otherwise the relation is
    /// a stored coordinate the form cannot re-derive, and the loop stays a
    /// defect.
    pub fn axis_rederives(&self, lp: &Loop, chord: u16) -> bool {
        chord == lp.chord && self.axis_seed_of(lp).is_some()
    }

    /// The seed of the axis a loop opens: its own holonomy versor (16.20).
    pub fn axis_seed_of(&self, lp: &Loop) -> Option<Versor> {
        element::versor_of_word(&lp.holonomy)
    }

    /// Open a chart axis over the loop closing on `chord` (A20 `OpenAxis`).
    /// Refuses a chord with no loop, a holonomy with no class, and a
    /// holonomy with no versor to seed the axis.
    pub(crate) fn open_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let lp = self.loops.get(i)?;
        let class = lp.holonomy.rotor().class_key()?;
        self.axis_seed_of(lp)?;
        self.axes.insert(class, chord);
        Some(())
    }

    /// The inverse move: the axis opened over the loop closing on `chord`
    /// leaves the form (the court's un-pivot, A20 step 3).
    pub(crate) fn close_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let class = self.loops.get(i)?.holonomy.rotor().class_key()?;
        self.axes.remove(&class)?;
        Some(())
    }

    /// The row one loop contributes to the rank audit (A18): an opened
    /// axis, or what its holonomy is; `None` when the holonomy has no class.
    pub fn loop_row_at(&self, idx: usize) -> Option<crate::transducer::sentence::LoopRow> {
        let lp = self.loops.get(idx)?;
        if self.opened_axis_of(lp).is_some() {
            return Some(crate::transducer::sentence::LoopRow::Opened);
        }
        crate::transducer::sentence::loop_row(&lp.holonomy)
    }

    pub fn loop_rows(&self) -> Option<Vec<crate::transducer::sentence::LoopRow>> {
        (0..self.loops.len()).map(|i| self.loop_row_at(i)).collect()
    }

    /// A18 over this form's loops. `None` when a holonomy has no class or
    /// the ℤ-rank overflows: unmeasured, and the caller refuses.
    pub fn rank(&self) -> Option<crate::transducer::sentence::Rank> {
        crate::transducer::sentence::rank_of_rows(&self.loop_rows()?)
    }
}

impl Form {
    /// Fill by_address, by_class, star, carriages (by transport-class key)
    /// and hue_maps from the vertices and faces, in one pass over the
    /// form's own vertices. Two gauges on one word class are two hue
    /// sections, each kept. Two carriages on one transport class stay two
    /// sections: that class is refused as one carriage. Two Word vertices
    /// on one chart are both on that chart's fiber (24.1).
    pub fn reindex(&mut self) {
        let mut by_address = Keyed::new();
        let mut by_class: Keyed<Vec<u16>> = Keyed::new();
        let mut by_rebased = Keyed::new();
        let mut by_unfolded = Keyed::new();
        let mut demands = Keyed::new();
        let mut hue_maps = std::mem::take(&mut self.hue_maps);
        let mut places: Keyed<Versor> = Keyed::new();
        let mut place_conflict: Keyed<()> = Keyed::new();
        let mut star = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            match v {
                Vertex::Word {
                    address,
                    derivation,
                    carriage,
                    gauge,
                    class,
                    ..
                } => {
                    let word = derivation.word();
                    by_address.insert(address, i as u16);
                    let chart = class.key();
                    match by_class.get_mut(&chart) {
                        Some(on) => on.push(i as u16),
                        None => {
                            by_class.insert(chart, vec![i as u16]);
                        }
                    }
                    if let Some(hk) = hue_section_key(word, gauge) {
                        hue_maps.insert(hk, gauge.clone());
                    }
                    if let Some(tk) = vertex_transport_key(word, gauge) {
                    match places.get(&tk) {
                        None => {
                            places.insert(tk, (**carriage).clone());
                        }
                        Some(held) if !element::same_versor(held, carriage) => {
                            place_conflict.insert(tk, ());
                        }
                        _ => {}
                    }
                    }
                }
                Vertex::Rebased { of, .. } => {
                    if let Some(Vertex::Word { address, .. }) = self.vertices.get(*of as usize) {
                        by_rebased.insert(address, i as u16);
                    }
                }
                Vertex::Unfolded { measure } => {
                    by_unfolded.insert(unfolded_key(measure), i as u16);
                }
                Vertex::Demand { kind, chart, .. } => {
                    demands.insert(crate::transducer::sentence::demand_key(kind, chart), i as u16);
                }
                _ => {}
            }
        }
        let mut carriages = Keyed::new();
        for (k, p) in places.fiber(ROOT) {
            if !place_conflict.contains(&k) {
                carriages.insert(&k, p.clone());
            }
        }
        // ONE STAR FACE PER VERTEX: the rule's face, the first between the
        // Frame and that vertex. A later face between the same two is a
        // section over some pairs (21 step 2, S12.2) and stays as the
        // bundle spoke it; the rebase pass re-derives star faces from their
        // vertices, and a section it re-derived was a section erased.
        for (fi, f) in self.faces.iter().enumerate() {
            let key = if f.i == 0 {
                vertex_ix_key(f.j)
            } else if f.j == 0 {
                vertex_ix_key(f.i)
            } else {
                continue;
            };
            if !star.contains(&key) {
                star.insert(key, fi as u16);
            }
        }
        self.by_address = by_address;
        self.by_class = by_class;
        self.by_rebased = by_rebased;
        self.by_unfolded = by_unfolded;
        self.demands = demands;
        self.star = star;
        self.carriages = carriages;
        self.hue_maps = hue_maps;
    }
}

impl Vertex {

    /// The word a vertex speaks; a demand vertex speaks none (50.5), so no
    /// face reaches it and it enters no loop.
    pub fn word_in(&self, form: &Form) -> Option<Word> {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Some(Word::identity()),
            Vertex::Word { derivation, .. } => Some(derivation.word().clone()),
            Vertex::Glued { placement, .. } => Some(placement.clone()),
            Vertex::Rebased { by, .. } => form.vertices.get(*by as usize)?.word_in(form),
            Vertex::Demand { .. } => None,
        }
    }

    pub fn class(&self) -> stalk::Invariant {
        match self {
            Vertex::Word { class, .. } | Vertex::Rebased { class, .. } => class.clone(),
            Vertex::Glued { chart, .. } | Vertex::Demand { chart, .. } => chart.clone(),
            Vertex::Frame | Vertex::Unfolded { .. } => stalk::Invariant::default(),
        }
    }

    pub fn depth(&self) -> u16 {
        match self {
            Vertex::Glued { depth, .. } => *depth,
            _ => 0,
        }
    }
}

impl Form {

    pub fn face_of(&self, i: u16, j: u16) -> Option<Transport> {
        let va = self.vertices.get(i as usize)?;
        let vb = self.vertices.get(j as usize)?;
        let wa = va.word_in(self)?;
        let wb = vb.word_in(self)?;
        let word = wb.compose(&wa.inverse()).normal_form();
        let ca = crate::transducer::sentence::transport::vertex_carriage(va, self)?;
        let cb = crate::transducer::sentence::transport::vertex_carriage(vb, self)?;
        let wv = element::versor_of_word(&word)?;
        let inv = wv.versor_inverse()?;
        let conjugated = wv.geo(&ca).geo(&inv);
        let carriage = cb.geo(&conjugated.versor_inverse()?);
        let gauge = relative_gauge(&vertex_gauge(va), &vertex_gauge(vb));
        Some(Transport { word, carriage, gauge })
    }

    pub fn delta1(&self) -> Vec<(u16, Word)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(i, lp)| (i as u16, lp.holonomy.clone()))
            .collect()
    }

    /// `δ¹g = 1` on every loop (10.3), where a loop whose class is an axis
    /// the form opened is coherent on the refined cover: its holonomy is
    /// the transition on that axis, re-derived from the loop (I6, 15.5).
    pub fn coheres(&self) -> bool {
        self.loops.iter().all(|lp| {
            (lp.holonomy.rotor().projective_eq(&Rotor::identity())
                || self.opened_axis_of(lp).is_some_and(|chord| self.axis_rederives(lp, chord)))
                && self.edge_coheres(lp)
        })
    }

    /// Closed: coheres, no remainder, and no demand vertex (2.2, 50.1);
    /// a sentence with demands is valid, never closed.
    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty() && self.demand_vertices().next().is_none()
    }

    /// The demand vertices of this sentence, with their indices (50.5).
    pub fn demand_vertices(&self) -> impl Iterator<Item = (u16, &crate::transducer::sentence::Demand, &stalk::Invariant, &[Cell])> {
        self.vertices.iter().enumerate().filter_map(|(i, v)| match v {
            Vertex::Demand { kind, chart, bodies } => Some((i as u16, kind, chart, bodies.as_slice())),
            _ => None,
        })
    }

    pub fn depth_of(&self) -> u16 {
        let mut d = 0u16;
        for v in &self.vertices {
            if let Vertex::Glued { depth, .. } = v {
                d = d.max(depth.saturating_add(1));
            }
        }
        d
    }

    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() || self.demand_vertices().next().is_some() {
            return false;
        }
        for lp in &self.loops {
            if lp.holonomy.is_identity() {
                continue;
            }
            if let Some(chord) = self.opened_axis_of(lp) {
                if self.axis_rederives(lp, chord) {
                    continue;
                }
                return false;
            }
            match lp.holonomy.order() {
                Order::Infinite => return false,
                Order::Finite(ell) => {
                    // A holonomy that is not a versor has no class (15.3) and
                    // nothing files it: the form is not cauterized. No key of
                    // the rotor's bytes stands in for the class it lacks.
                    let Some(k) = lp.holonomy.rotor().class_key() else {
                        return false;
                    };
                    match self.torsion.get(&k) {
                        Some(filed) if filed.period == ell => {}
                        _ => return false,
                    }
                }
            }
        }
        true
    }
}

pub(crate) fn form_face_step(faces: &[FormFace], fi: usize, from: usize, to: usize) -> Option<Word> {
    let f = faces.get(fi)?;
    if f.i as usize == from && f.j as usize == to {
        Some(f.transport.word.clone())
    } else {
        Some(f.transport.word.inverse())
    }
}

pub(crate) fn form_named_holonomy(faces: &[FormFace], path: &[usize], walked: &[usize]) -> Word {
    let mut w = Word::identity();
    for (win, &fi) in path.windows(2).zip(walked) {
        if let Some(s) = form_face_step(faces, fi, win[0], win[1]) {
            w = w.compose(&s);
        }
    }
    w.normal_form()
}

/// The tree path from `src` to `dst` as its vertices and the faces walked
/// between them, named in the same pass that finds them: each vertex is
/// reached through one face of its own adjacency, so no face is searched
/// for afterwards.
pub(crate) fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if src == dst {
        return Some((vec![src], Vec::new()));
    }
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; adj.len()];
    prev[src] = Some((src, 0));
    let mut q = vec![src];
    let mut qi = 0usize;
    while qi < q.len() {
        let u = q[qi];
        qi += 1;
        if u == dst {
            break;
        }
        for &(v, fi) in &adj[u] {
            if prev[v].is_some() {
                continue;
            }
            prev[v] = Some((u, fi));
            q.push(v);
        }
    }
    prev[dst]?;
    let mut path = vec![dst];
    let mut walked = Vec::new();
    let mut cur = dst;
    while cur != src {
        let (p, fi) = prev[cur]?;
        walked.push(fi);
        path.push(p);
        cur = p;
    }
    path.reverse();
    walked.reverse();
    Some((path, walked))
}
