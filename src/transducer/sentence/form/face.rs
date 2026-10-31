// 18.2, 21 step 2, 50.3 — THE FACES OF A FORM, READ: the word across an
// overlap, the place a pair of charts makes, and the overlaps a pair of
// vertices makes at the member pairs and frames the incidence names.
//
// AT1 — A FACE IS AN OVERLAP OF TWO CHARTS, keyed by the two (Q2): one per
// member pair per frame. Its word is DERIVED from its ends and the pivot's
// factors; its place is MEASURED where the overlap is made.

use super::*;

/// EVERY OVERLAP LEAVING A VERTEX'S CHARTS in a face carrier — a fibre read
/// at the vertex's framed coordinate, which every chart of it opens with; the
/// Frame's chart is its tag alone.
pub(crate) fn overlaps_leaving(faces: &Faces, v: &coord::VertexAt) -> Vec<coord::Overlap> {
    let mut prefix = vec![1u8];
    crate::transducer::address::put_key_bytes(&mut prefix, v.encode().as_bytes());
    match v {
        coord::VertexAt::Frame => faces.fibre_at(&[0u8]),
        _ => faces.fibre_at(&prefix),
    }
    .into_iter()
    .flatten()
    .filter_map(|(k, _)| coord::Overlap::decode(&k).ok())
    .collect()
}

impl Form {
    /// THE WORD ACROSS AN OVERLAP THIS FORM HOLDS, derived: the pivot's
    /// factors around the coboundary of the two ends ([`face_word_on`]).
    pub fn face_word(&self, o: &coord::Overlap) -> Option<Word> {
        face_word_on(&self.vertices, o, self.faces.at(o)?)
    }

    /// THE COBOUNDARY BETWEEN TWO VERTICES, `w_b ∘ w_a⁻¹`: the word an
    /// unpivoted overlap from `a` to `b` derives.
    pub fn coboundary(&self, a: &coord::VertexAt, b: &coord::VertexAt) -> Option<Word> {
        let wa = self.vertices.at(a)?.word_in(&self.vertices)?;
        let wb = self.vertices.at(b)?.word_in(&self.vertices)?;
        Some(wb.compose(&wa.inverse()).normal_form())
    }

    /// WHAT AN OVERLAP CARRIES BETWEEN TWO CHARTS: the relative carriage of
    /// the body at `to` against the body at `from` under the coboundary
    /// word, `c_b (w c_a w̃)~`, and the relative hue map. The edge is not
    /// derived here — 50.3's edge is measured on the two bodies' union by
    /// whatever made the overlap — so it is carried in by the caller.
    pub fn face_place(&self, from: &coord::ChartAt, to: &coord::ChartAt, edge: Option<Transport>) -> Option<FacePlace> {
        use crate::transducer::sentence::transport::{vertex_carriage_at, vertex_gauge_at};
        let (a, b) = (from.vertex(), to.vertex());
        let va = self.vertices.at(&a)?;
        let vb = self.vertices.at(&b)?;
        // The Frame's chart is its tag alone: it stands at the root stratum,
        // which is what the Frame's placement is read at (the identity, at
        // any chart), not a chart invented for it.
        let root = stalk::Invariant::default();
        let at = |c: &coord::ChartAt| match c.invariant() {
            Some(inv) => inv.clone(),
            None => root.clone(),
        };
        let (ci, cj) = (at(from), at(to));
        // A WORD'S MEMBER AT A FRAME is the section standing on that frame:
        // two sections of one chart are two members, told apart by the
        // frames each holds (AT1, ruled), so the chart alone would read the
        // first section's carriage at the other's frames.
        let member = |v: &Vertex, c: &coord::ChartAt, inv: &stalk::Invariant| match (v, c.frame()) {
            (Vertex::Word { .. }, Some(k)) => v.acting_at(inv, k).map(|a| ((*a.carriage).clone(), a.gauge.clone())),
            _ => Some((vertex_carriage_at(v, self, inv)?, vertex_gauge_at(v, inv))),
        };
        let (ca, ga) = member(va, from, &ci)?;
        let (cb, gb) = member(vb, to, &cj)?;
        let wv = element::versor_of_word(&self.coboundary(&a, &b)?)?;
        let is_versor = crate::transducer::cga::is_versor;
        (is_versor(&wv) && is_versor(&ca)).then_some(())?;
        let conjugated = wv.sandwich(&ca);
        let carriage = cb.geo(&conjugated.reverse());
        let gauge = relative_gauge(&ga, &gb);
        Some(FacePlace { carriage, gauge, edge, refraction: None })
    }

    /// THE FRAMES A MEMBER STANDS ON: its own, held on it (AT1, ruled) —
    /// every section of the chart's, since two sections of one chart stand
    /// on different frames and are told apart by them. In the frames' own
    /// order; empty where the vertex does not act on the chart.
    pub fn member_frames(&self, v: &coord::VertexAt, chart: &stalk::Invariant) -> Vec<u64> {
        let Some(x) = self.vertices.at(v) else { return Vec::new() };
        let mut all: Keyed<()> = Keyed::new();
        for a in x.acting_all_on(chart) {
            for (k, ()) in a.frames.scoped() {
                all.insert(&k, ());
            }
        }
        frames_of_section(&all)
    }

    /// THE OVERLAPS TWO VERTICES MAKE AT THE MEMBER PAIRS THE INCIDENCE
    /// NAMES, one per frame both members stand on.
    ///
    /// `pairs` is the incidence: which member of `a` relates to which member
    /// of `b`. It comes from the site occupancy that created the face or from
    /// the overlaps already held; it is never the product of the two fibres
    /// (21 step 2). A member pair whose place does not derive is left out
    /// rather than given a versor it does not have (rule 11).
    pub fn overlaps_between(
        &self,
        a: &coord::VertexAt,
        b: &coord::VertexAt,
        pairs: &[(stalk::Invariant, stalk::Invariant)],
    ) -> Vec<(coord::Overlap, FormFace)> {
        let mut out = Vec::new();
        for (ci, cj) in pairs {
            let there: Keyed<()> = section_of_frames(self.member_frames(b, cj));
            for k in self.member_frames(a, ci) {
                if !there.contains(&frame_key(k)) {
                    continue;
                }
                let o = coord::Overlap {
                    from: coord::ChartAt::at(a.clone(), ci.clone(), k),
                    to: coord::ChartAt::at(b.clone(), cj.clone(), k),
                };
                if let Some(p) = self.face_place(&o.from, &o.to, None) {
                    out.push((o, FormFace::placed(p, Producer::Place)));
                }
            }
        }
        out
    }

    /// THE MEMBER PAIRS TWO VERTICES HAVE where one of them stands at a
    /// single chart — the star case, and every face this tree makes outside
    /// a cover's meets. `None` where both carry several: the pairing is the
    /// scene's fact and this does not have it.
    pub fn face_pairs(&self, a: &coord::VertexAt, b: &coord::VertexAt) -> Option<Vec<(stalk::Invariant, stalk::Invariant)>> {
        let ci = self.vertices.at(a)?.charts();
        let cj = self.vertices.at(b)?.charts();
        match (ci.as_slice(), cj.as_slice()) {
            ([one], many) => Some(many.iter().map(|c| (one.clone(), c.clone())).collect()),
            (many, [one]) => Some(many.iter().map(|c| (c.clone(), one.clone())).collect()),
            _ => None,
        }
    }

    /// THE STAR OF A VERTEX: an overlap from the Frame's chart — its tag
    /// alone (Q-B1) — to every member of the vertex at every frame it stands
    /// on, oriented Frame-first.
    pub fn star_overlaps(&self, v: &coord::VertexAt) -> Vec<(coord::Overlap, FormFace)> {
        let Some(vertex) = self.vertices.at(v) else { return Vec::new() };
        let mut out = Vec::new();
        for chart in vertex.charts() {
            for k in self.member_frames(v, &chart) {
                let o = coord::Overlap { from: coord::ChartAt::Frame, to: coord::ChartAt::at(v.clone(), chart.clone(), k) };
                if let Some(p) = self.face_place(&o.from, &o.to, None) {
                    out.push((o, FormFace::placed(p, Producer::Place)));
                }
            }
        }
        out
    }

    /// EVERY OVERLAP LEAVING A VERTEX'S CHARTS — a fibre read at the
    /// vertex's framed coordinate, which every chart of it opens with.
    pub fn overlaps_from(&self, v: &coord::VertexAt) -> Vec<coord::Overlap> {
        overlaps_leaving(&self.faces, v)
    }

    /// EVERY OVERLAP WITH AN END AT THIS VERTEX, either way. The arriving
    /// half has no prefix to follow, so it is read off the whole carrier;
    /// the one reader of that, for the contraction moves that retract a
    /// vertex and everything touching it.
    pub fn overlaps_at(&self, v: &coord::VertexAt) -> Vec<coord::Overlap> {
        self.faces
            .entries()
            .filter(|(o, _)| &o.from_vertex() == v || &o.to_vertex() == v)
            .map(|(o, _)| o)
            .collect()
    }

    /// Put a vertex under its own coordinate; `None` where it has none
    /// (rule 11) and nothing is put.
    pub fn put_vertex(&mut self, v: Vertex) -> Option<coord::VertexAt> {
        let at = v.at()?;
        self.vertices.put(&at, v);
        Some(at)
    }

    /// Put an overlap's face.
    pub fn put_face(&mut self, o: coord::Overlap, f: FormFace) {
        self.faces.put(&o, f);
    }

    /// STAND A WORD'S MEMBERS AT FRAME `k`: the vertex at its class's
    /// coordinate, its members written with the frame (AT1, ruled: a member
    /// holds its frames). A vertex of a class already held JOINS it — its
    /// members are that vertex's members, a member of one section key
    /// standing on the frames of both — and never replaces it (Q1: one vertex
    /// per class). Returns the chart of the vertex's first member at `k`;
    /// `None` for a vertex with no coordinate, no members, or a coordinate
    /// held by another kind.
    pub fn stand_member(&mut self, v: &Vertex, k: u64) -> Option<coord::ChartAt> {
        let at = v.at()?;
        let own = v.acts()?;
        let fk = frame_key(k);
        match self.vertices.at_mut(&at) {
            // ONE VERTEX PER CLASS, its derivation the one whose rotor key is
            // least (content order, law 2) — which arrived first is never
            // written — and its address DERIVED from the derivation and every
            // member it now holds (54.2), as `from_address` derives it.
            Some(Vertex::Word { acts, address, derivation, contact }) => {
                if let Vertex::Word { derivation: d, .. } = v {
                    if d.rotor().key() < derivation.rotor().key() {
                        *derivation = d.clone();
                    }
                }
                for (sk, a) in own.entries() {
                    let mut a = match acts.drop_at(&sk) {
                        Some(held) => held,
                        None => a.clone(),
                    };
                    a.frames.insert(&fk, ());
                    acts.put(&sk, a);
                }
                *address = crate::transducer::sentence::word_vertex_address(derivation, acts, *contact);
            }
            Some(Vertex::Rebased { acts, .. }) => {
                for (sk, a) in own.entries() {
                    let mut a = match acts.drop_at(&sk) {
                        Some(held) => held,
                        None => a.clone(),
                    };
                    a.frames.insert(&fk, ());
                    acts.put(&sk, a);
                }
            }
            Some(_) => return None,
            None => {
                let mut v = v.clone();
                if let Vertex::Word { acts, .. } | Vertex::Rebased { acts, .. } = &mut v {
                    let held: Vec<_> = acts.entries().map(|(sk, a)| (sk, a.clone())).collect();
                    for (sk, mut a) in held {
                        a.frames.insert(&fk, ());
                        acts.put(&sk, a);
                    }
                }
                self.vertices.put(&at, v);
            }
        }
        let (_, first) = own.scoped().next()?;
        Some(coord::ChartAt::at(at, first.chart.clone(), k))
    }

    /// PLACE AN OVERLAP between two charts the form holds: its place
    /// measured from the members at the overlap's frame, the edge carried in
    /// by whatever measured it, producer `Place`. `None` where the place does
    /// not derive, and nothing is put (rule 11).
    pub fn place_overlap(&mut self, from: coord::ChartAt, to: coord::ChartAt, edge: Option<Transport>) -> Option<coord::Overlap> {
        let place = self.face_place(&from, &to, edge)?;
        let o = coord::Overlap { from, to };
        self.faces.put(&o, FormFace::placed(place, Producer::Place));
        Some(o)
    }

    /// The orbit holonomies as the pivot's δ¹ reads them: the orbit's chord
    /// and the word it composes.
    pub fn delta1(&self) -> Vec<(coord::Overlap, Word)> {
        self.orbits
            .iter()
            .filter_map(|lp| Some((lp.chord.clone(), orbit_word_on(&self.faces, &self.vertices, lp)?)))
            .collect()
    }
}
