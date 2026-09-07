// 18.2 — THE FORM: vertices, faces, loops, and the reads that walk them.
//
//! A face is constant in the word between its two vertices; a loop is a
//! cycle of faces; the holonomy of a loop is the composite around it. All
//! three are structure, walked and never searched.

use super::*;

/// How an unfolding stops. The step count is an instance coordinate, never
/// stored on the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: u16 },
}

/// A vertex of a sentence form (chapter 18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vertex {
    Frame,
    Word {
        address: crate::transducer::a21::Address,
        word: Word,
        carriage: Versor,
        gauge: Vec<(u8, u8)>,
        class: stalk::Invariant,
        /// Stop ring. A form coordinate with the carriage and gauge; not M1.
        contact: Option<alphabet::Contact>,
    },
    Glued {
        address: crate::transducer::a21::Address,
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
        carriage: Versor,
        /// `of`'s hue map, spoken on `of`'s bodies.
        gauge: Vec<(u8, u8)>,
        /// `of`'s chart: the section coordinate naming the bodies it speaks to.
        class: stalk::Invariant,
    },
    Unfolded { measure: Measure },
}

/// Face of a `Form`: two vertices and the transport between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub i: u16,
    pub j: u16,
    pub transport: Transport,
}

impl FormFace {
    pub fn new(i: u16, j: u16, word: Word) -> FormFace {
        FormFace {
            i,
            j,
            transport: Transport::from_word(word),
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
    pub torsion: Keyed<(Vec<(u8, u8)>, u32, u16)>,
    /// Cited form address → glued vertex index. Written by [`Form::glue`]
    /// and rebuilt by the decoder from Glued vertices.
    pub glued: Keyed<u16>,
    /// Incomparable discharges held as cells until a glue or ruling.
    /// Outer key is the after class; inner is the discharging rotor A1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// Word vertex address → index. The meet for A6 and Mayer–Vietoris.
    pub by_address: Keyed<u16>,
    /// Vertex index key → Frame face index. Star incidence, one face per vertex.
    pub star: Keyed<u16>,
    /// Word vertex address → carriage, filled at construction.
    pub carriages: Keyed<Versor>,
    /// Rebased vertex keyed by the `of` Word address.
    pub by_rebased: Keyed<u16>,
    /// Unfolded vertex keyed by its measure key.
    pub by_unfolded: Keyed<u16>,
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
            glued: Keyed::new(),
            vortex: Keyed::new(),
            by_address: Keyed::new(),
            star: Keyed::new(),
            carriages: Keyed::new(),
            by_rebased: Keyed::new(),
            by_unfolded: Keyed::new(),
        }
    }
}

impl Form {
    /// Fill by_address, star, carriages (by transport-class key) and
    /// hue_maps from the vertices and faces. Two gauges on one word class
    /// are two hue sections, each kept. Two carriages on one transport
    /// class stay two sections: that class is refused as one carriage.
    pub fn reindex(&mut self) {
        let mut by_address = Keyed::new();
        let mut by_rebased = Keyed::new();
        let mut by_unfolded = Keyed::new();
        let mut hue_maps = std::mem::take(&mut self.hue_maps);
        let mut places: Keyed<Versor> = Keyed::new();
        let mut place_conflict: Keyed<()> = Keyed::new();
        let mut star = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            match v {
                Vertex::Word {
                    address,
                    word,
                    carriage,
                    gauge,
                    class: _,
                    ..
                } => {
                    by_address.insert(address, i as u16);
                    if let Some(hk) = hue_section_key(word, gauge) {
                        hue_maps.insert(hk, gauge.clone());
                    }
                    if let Some(tk) = vertex_transport_key(word, gauge) {
                    match places.get(&tk) {
                        None => {
                            places.insert(tk, carriage.clone());
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
                _ => {}
            }
        }
        let mut carriages = Keyed::new();
        for (k, p) in places.fiber(ROOT) {
            if !place_conflict.contains(&k) {
                carriages.insert(&k, p.clone());
            }
        }
        for (fi, f) in self.faces.iter().enumerate() {
            if f.i == 0 {
                star.insert(vertex_ix_key(f.j), fi as u16);
            } else if f.j == 0 {
                star.insert(vertex_ix_key(f.i), fi as u16);
            }
        }
        self.by_address = by_address;
        self.by_rebased = by_rebased;
        self.by_unfolded = by_unfolded;
        self.star = star;
        self.carriages = carriages;
        self.hue_maps = hue_maps;
    }
}

impl Vertex {

    pub fn word_in(&self, form: &Form) -> Option<Word> {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Some(Word::identity()),
            Vertex::Word { word, .. } => Some(word.clone()),
            Vertex::Glued { placement, .. } => Some(placement.clone()),
            Vertex::Rebased { by, .. } => form.vertices.get(*by as usize)?.word_in(form),
        }
    }

    pub fn class(&self) -> stalk::Invariant {
        match self {
            Vertex::Word { class, .. } | Vertex::Rebased { class, .. } => class.clone(),
            Vertex::Glued { chart, .. } => chart.clone(),
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

    pub fn coheres(&self) -> bool {
        self.delta1()
            .into_iter()
            .all(|(_, w)| w.rotor().projective_eq(&Rotor::identity()))
    }

    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty()
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
        if !self.remainder.is_empty() {
            return false;
        }
        for lp in &self.loops {
            if lp.holonomy.is_identity() {
                continue;
            }
            match lp.holonomy.order() {
                Order::Infinite => return false,
                Order::Finite(ell) => {
                    let k = lp.holonomy.rotor().class_or_rotor_key();
                    match self.torsion.get(&k) {
                        Some((_, e, _)) if *e == ell => {}
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
