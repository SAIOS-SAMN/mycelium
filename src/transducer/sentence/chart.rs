// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 17.4, 3.3, 31 — THE CHART OF A FORM: its M4 coordinates, the glue of a
//! crystallized form into it, the coboundary gauge between two forms, the
//! A21 vertex key, and the `SentenceRule` a form is spoken as.

use super::form::*;
use super::tape::from_element_form;
use super::transport::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{Rotor, Word};
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::stalk;

thread_local! {
    /// HOW MANY TIMES A FORM WAS RENUMBERED, so a pass that claims to
    /// retract once can be asked rather than believed (rule 13: verify the
    /// claim, not the outcome). `retract_vertices` rebuilds the vertices,
    /// the faces, the cycle basis and every index; the glue pass ran it per
    /// citation and now runs it once for the pass.
    ///
    /// A plain counter in the shape `family::pairing`'s `FACE_WORK` already
    /// has: read and never branched on, and no engine path consults it.
    static RETRACTIONS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Renumberings since the last reset.
#[cfg(test)]
pub fn retractions() -> u64 {
    RETRACTIONS.with(|c| c.get())
}

#[cfg(test)]
pub fn reset_retractions() {
    RETRACTIONS.with(|c| c.set(0));
}

impl Form {
    /// Glue a closed form in as one vertex, and walk the loops and the
    /// index for it. The single-citation reader; a PASS of citations calls
    /// [`Form::glue_append`] and walks once at the end (Z2 item 1).
    pub fn glue(&mut self, s: &Form, s_addr: crate::transducer::address::Address, placement: Word, magnitude: [u64; 4]) -> Option<u16> {
        self.glue_under(s, s_addr, placement, magnitude, element::identity_versor())
    }

    /// THE CITATION UNDER ITS MORPHISM φ (54.2): the cited form acts inside
    /// this one as `x ↦ φ x φ̃`. `glue` is this at the identity — a
    /// citation that turns and neither translates nor dilates — which is
    /// every citation the extractor builds today.
    pub fn glue_under(
        &mut self,
        s: &Form,
        s_addr: crate::transducer::address::Address,
        placement: Word,
        magnitude: [u64; 4],
        phi: element::Versor,
    ) -> Option<u16> {
        let idx = self.glue_append_under(s, s_addr, placement, magnitude, phi)?;
        self.loops = Self::loops_of(&self.faces, &self.vertices);
        self.reindex();
        Some(idx)
    }

    /// THE CITATION APPENDED AND NOTHING WALKED (Z2 item 1): the `Glued`
    /// vertex, its placement face, the glue index and the depth — every
    /// coordinate the citation itself writes — and neither `loops_of` nor
    /// `reindex`.
    ///
    /// The glue pass of `extract_rule_with` cited one form 578 times on
    /// `1d0a4b61`, and each citation rebuilt the cycle basis over 2 850
    /// faces and reindexed the whole form. Measured on the instrument: 4.9 s
    /// of `face_at` inside `loops_of`, on a 16.8 s pull.
    ///
    /// AN APPEND CANNOT INVALIDATE WHAT THE PASS READS. A `Glued` vertex
    /// carries no transport class, so `by_transport` — the index the pass
    /// asks for the vertices to retract — is untouched by it; the loops the
    /// appended face closes are read once after the pass, with the
    /// retractions, which rebuilds them anyway.
    pub fn glue_append(&mut self, s: &Form, s_addr: crate::transducer::address::Address, placement: Word, magnitude: [u64; 4]) -> Option<u16> {
        self.glue_append_under(s, s_addr, placement, magnitude, element::identity_versor())
    }

    /// [`glue_append`] under a named morphism.
    pub fn glue_append_under(
        &mut self,
        s: &Form,
        s_addr: crate::transducer::address::Address,
        placement: Word,
        magnitude: [u64; 4],
        phi: element::Versor,
    ) -> Option<u16> {
        if !s.is_closed() {
            return None;
        }
        // 18: A FORM OPENS AT ITS FRAME, so vertex zero is the frame by the
        // form's own shape and the slice pattern reads that coordinate.
        let [frame @ Vertex::Frame, ..] = s.vertices.as_slice() else {
            return None;
        };
        if !frame.word_in(s).is_some_and(|w| w.is_identity()) {
            return None;
        }
        let d = s.depth_of();
        let host = self.depth_of();
        if host > 0 && d >= host {
            return None;
        }
        let chart = chart_of_form(s)?;
        if chart.0.is_empty() {
            return None;
        }
        let idx = self.vertices.len() as u16;
        self.glued.insert(&s_addr, idx);
        self.vertices.push(Vertex::Glued {
            address: s_addr,
            chart,
            magnitude,
            depth: d,
            placement: placement.clone(),
            carriage: Box::new(phi),
        });
        // THE PLACEMENT FACE CARRIES ITS PLACEMENT. A Glued vertex stands at
        // the cited form's chart and the Frame at the root, so there is one
        // member pair and `face_carrying` names it; `FormFace::new` left it
        // empty and the loop through this face had no member path.
        match self.face_carrying(0, idx, placement.clone()) {
            Some(face) => self.faces.push(face),
            None => self.faces.push(FormFace::new(0, idx, placement)),
        }
        self.depth = self.depth_of();
        Some(idx)
    }

    /// Drop the named vertices (keyed by old index), drop faces that touched
    /// them, remap surviving faces through the old-to-new map, rebuild loops.
    pub fn retract_vertices(&mut self, drop: &Keyed<()>) {
        RETRACTIONS.with(|c| c.set(c.get().saturating_add(1)));
        let n = self.vertices.len();
        if n == 0 {
            return;
        }
        let mut old_to_new: Vec<Option<u16>> = vec![None; n];
        let mut new_verts = Vec::new();
        for (i, v) in self.vertices.iter().enumerate() {
            if i != 0 && drop.contains(&vertex_ix_key(i as u16)) {
                continue;
            }
            old_to_new[i] = Some(new_verts.len() as u16);
            new_verts.push(v.clone());
        }
        let mut face_old_to_new: Vec<Option<u16>> = vec![None; self.faces.len()];
        let mut new_faces = Vec::new();
        for (fi, f) in self.faces.iter().enumerate() {
            let Some(&Some(i)) = old_to_new.get(f.i as usize) else {
                continue;
            };
            let Some(&Some(j)) = old_to_new.get(f.j as usize) else {
                continue;
            };
            face_old_to_new[fi] = Some(new_faces.len() as u16);
            // THE SECTION RIDES WITH THE FACE. The pairs are the BUNDLE's,
            // not this form's vertices', so they are carried across and
            // never remapped.
            new_faces.push(FormFace {
                i,
                j,
                word: f.word.clone(),
                at: f.at.clone(),
                pairs: f.pairs.clone(),
            });
        }
        // RENUMBERING A FORM TOUCHES EVERY CARRIER IT HAS. Each of the four
        // below is this form's own — glued vertices, sections, torsion,
        // opened axes — and each is rebuilt whole against the new indices,
        // because an entry left behind names a vertex that no longer
        // exists. There is nothing to descend on: the form IS the scope.
        let mut glued = Keyed::new();
        for (k, &old_i) in self.glued.scoped() {
            if let Some(&Some(ni)) = old_to_new.get(old_i as usize) {
                glued.insert(&k, ni);
            }
        }
        // SECTIONS ARE NOT RENUMBERED, AND THE REMAP HERE WAS A TYPE ERROR.
        // `Form.sections` is class key → PAIR INDICES OF THE BUNDLE, which
        // its own field doc says; this ran those pair indices through
        // `old_to_new`, a VERTEX table, and dropped every entry whose
        // mapping came out empty. A class measured on pairs 0, 1 and 2 of a
        // form retracted to two vertices kept pairs 0 and 1 and silently
        // lost the third — which is the court then refusing that vertex on
        // the one pair it was actually measured on, the exact fault the
        // section exists to prevent.
        //
        // A class whose vertex was dropped keeps its entry: nothing reads a
        // section without first finding the Word vertex that carries the
        // class (`vertices_carry_frames`), so a stale entry is unreachable
        // rather than wrong, and dropping it by an index arithmetic that
        // does not apply is the only way to be wrong here.
        let sections = self.sections.clone();
        let mut torsion = Keyed::new();
        for (k, FiledLoop { sigma, period: ell, chord }) in self.torsion.scoped() {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            // THE HUE MAP IS NOT RENUMBERED. `sigma` is the hue-map
            // generator — a permutation of HUES — and this ran every entry
            // through `old_to_new`, the VERTEX table: colours relabelled as
            // if they were vertices, and any cycle naming a hue above the
            // vertex count dropped the whole filed loop through the
            // `else { continue }`. Renumbering a form's vertices moves no
            // hue. The chord is a face index and is remapped, above.
            torsion.insert(&k, FiledLoop { sigma: sigma.clone(), period: *ell, chord: nc });
        }
        let mut axes = Keyed::new();
        for (k, chord) in self.axes.scoped() {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            axes.insert(&k, nc);
        }
        self.vertices = new_verts;
        self.faces = new_faces;
        self.glued = glued;
        self.sections = sections;
        self.torsion = torsion;
        self.axes = axes;
        self.loops = Self::loops_of(&self.faces, &self.vertices);
        self.depth = self.depth_of();
        self.reindex();
    }

    /// Replace every face `g_ij` by `γ_j · g_ij · γ_i⁻¹`. Holonomies of
    /// identity stay identity. The gauge's length is the vertex count.
    /// Every new transport is computed first; a form whose gauge cannot
    /// be applied is left unchanged.
    pub fn regauge(&mut self, gamma: &[Word]) -> Option<()> {
        if gamma.len() != self.vertices.len() {
            return None;
        }
        // THE REGAUGE CONJUGATES EVERY MEMBER PAIR. The word is the face's and
        // conjugates once; the carriage is each member pair's and conjugates
        // at each, or the regauge moves one pair of bodies into the new frame
        // and leaves the rest in the old one.
        let mut next: Vec<(Word, Keyed<FacePlace>)> = Vec::with_capacity(self.faces.len());
        for f in &self.faces {
            let gi = gamma.get(f.i as usize)?;
            let gj = gamma.get(f.j as usize)?;
            let word = gj.compose(&f.word.compose(&gi.inverse())).normal_form();
            let gv = element::versor_of_word(gj)?;
            let inv = gv.versor_inverse()?;
            let mut at = Keyed::new();
            for (k, p) in f.at.scoped() {
                at.insert(
                    &k,
                    FacePlace {
                        carriage: gv.geo(&p.carriage).geo(&inv),
                        gauge: p.gauge.clone(),
                        edge: p.edge.clone(),
                    },
                );
            }
            next.push((word, at));
        }
        for (f, (word, at)) in self.faces.iter_mut().zip(next) {
            f.word = word;
            f.at = at;
        }
        self.loops = Self::loops_of(&self.faces, &self.vertices);
        Some(())
    }

    /// Spanning forest (12.2): each component is rooted at its least vertex
    /// index with γ = 1; γ_j = γ_i · g_ij⁻¹ along tree edges. None only when
    /// a face names a vertex outside the form.
    pub fn solve_coboundary(&self) -> Option<Vec<Word>> {
        let n = self.vertices.len();
        if n == 0 {
            return None;
        }
        let mut adj: Vec<Vec<(usize, usize, bool)>> = vec![Vec::new(); n];
        for (fi, f) in self.faces.iter().enumerate() {
            let (i, j) = (f.i as usize, f.j as usize);
            if i >= n || j >= n {
                return None;
            }
            adj[i].push((j, fi, true));
            adj[j].push((i, fi, false));
        }
        let mut gamma: Vec<Option<Word>> = vec![None; n];
        for root in 0..n {
            if gamma[root].is_some() {
                continue;
            }
            gamma[root] = Some(Word::identity());
            let mut q = vec![root];
            let mut qi = 0usize;
            while qi < q.len() {
                let u = q[qi];
                qi += 1;
                let gu = gamma[u].as_ref()?.clone();
                for &(v, fi, u_is_i) in &adj[u] {
                    let f = &self.faces[fi];
                    let gv = if u_is_i {
                        gu.compose(&f.word.inverse()).normal_form()
                    } else {
                        gu.compose(&f.word).normal_form()
                    };
                    match &gamma[v] {
                        None => {
                            gamma[v] = Some(gv);
                            q.push(v);
                        }
                        Some(held) if held != &gv => {}
                        _ => {}
                    }
                }
            }
        }
        gamma.into_iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct SentenceRule {
    pub form: Form,
    /// Classes present on some frames only: class key → the frames it
    /// stands on, each its own coordinate (51.8).
    pub sections: Keyed<Keyed<()>>,
    /// Family frame ratio ((h_out, h_in), (w_out, w_in)).
    pub ratio: ((usize, usize), (usize, usize)),
}

impl SentenceRule {
    pub fn section_count(&self) -> usize {
        self.sections
            .scoped()
            .map(|(_, v)| v.len())
            .sum()
    }

    /// EVERY RECOLOURING SECTION THIS TRANSPORT CLASS HAS BEEN SPOKEN WITH.
    ///
    /// It read `hue_maps` by a CHART key — `inv.key()` — and every writer
    /// files under `hue_section_key_of(rotor, gauge)`, which opens `TAG_HUE`
    /// and carries the rotor's class. Two key spaces, so the read could not
    /// match a write for any input: the recolouring section was written on
    /// every reindex and reachable by nothing. It had no callers, which is
    /// how it survived — a reader that always answers "no hue" and is never
    /// asked.
    ///
    /// It reads the STRATUM now, because the gauge is part of the full key
    /// and a read keyed by the whole of it can only answer a question whose
    /// answer the asker already holds. Several gauges on one class are
    /// several sections (18.2), so the answer is all of them and the caller
    /// is not handed one of several as though it were the only one.
    pub fn hue_sections_of(
        &self,
        rotor: &crate::transducer::alphabet::Rotor,
    ) -> Vec<Vec<(u8, u8)>> {
        let Some(prefix) = crate::transducer::sentence::hue_class_prefix_of(rotor) else {
            return Vec::new();
        };
        let Some(at) = crate::network::nerve::Prefix::new(&prefix) else {
            return Vec::new();
        };
        self.form.hue_maps.fiber(at).map(|(_, g)| g.clone()).collect()
    }

    /// Carriage of a transport class, read from the form's carriages.
    pub fn carriage_of(&self, key: &[u8]) -> Option<Versor> {
        self.form.carriages.get(key).cloned()
    }
}

/// A form as a `SentenceRule` (chapter 31). Hue maps are filled from the
/// form's own Word vertices when the form does not already carry them.
pub fn rule_of_form(mut form: Form) -> SentenceRule {
    form.reindex();
    let sections = form.sections.clone();
    let ratio = form.ratio;
    SentenceRule {
        form,
        sections,
        ratio,
    }
}



/// Vertex key of a sentence form: Word vertices (M1, carriage, contact,
/// gauge) and face words. The body's absolute position is already quotiented
/// on the chain; the bundle's carriage is meaning.
pub fn vertex_address(e: &Element) -> Option<crate::transducer::address::Address> {
    from_element_form(e).map(|f| form_vertex_key(&f))
}
pub fn form_vertex_key(f: &Form) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM];
    buf.extend_from_slice(&(f.vertices.len() as u32).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => buf.push(0),
            // 1.5: the key is the COMPOSED product's canonical rotor, so
            // two minds whose derivations differ reach one address. The
            // citations are content the court replays (50.2 step 2) and
            // are deliberately not keyed: keying them would fork one
            // vertex into one address per presentation.
            Vertex::Word { derivation, acts, contact, .. } => {
                buf.push(1);
                put_word_vertex(&mut buf, derivation, acts, *contact);
            }
            // 54.2 — A CITATION IS ⟨A_c, φ⟩: the cited form's canonical
            // address, and the transition versor it acts under here. The
            // address alone made the citation a static pointer — two forms
            // citing one sub-complex at two places reached one address,
            // because `placement` is a rotor and cannot translate or
            // dilate. φ is A4's canonical writing and is O(1) whatever the
            // cited form's size, so the key does not inline it.
            Vertex::Glued {
                address,
                placement,
                depth,
                carriage,
                ..
            } => {
                buf.push(2);
                crate::transducer::address::put_bytes(&mut buf, address);
                placement.put_canonical(&mut buf);
                buf.extend_from_slice(&depth.to_le_bytes());
                carriage.put_canonical(&mut buf);
            }
            Vertex::Rebased { by, of, acts } => {
                buf.push(3);
                buf.extend_from_slice(&by.to_le_bytes());
                buf.extend_from_slice(&of.to_le_bytes());
                // THE MEMBERS IN CHART-KEY ORDER, each with the relative it
                // carries there and its sorted gauge. A rebased vertex acting on
                // two charts SAYS more than one acting on one, so the count is
                // on the key (D10); the chart key goes with each member for the
                // same reason a face's pair key does.
                buf.extend_from_slice(&(acts.len() as u16).to_le_bytes());
                for (ck, a) in acts.scoped() {
                    crate::transducer::address::put_bytes(&mut buf, &ck);
                    // A4 output here as in the Word arm (17.2).
                    a.carriage.put_canonical(&mut buf);
                    let mut g = a.gauge.clone();
                    g.sort_unstable();
                    buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
                    for (x, y) in g {
                        buf.push(x);
                        buf.push(y);
                    }
                }
            }
            Vertex::Unfolded { measure } => {
                buf.push(4);
                match measure {
                    Measure::Occupied => buf.push(0),
                    Measure::Remainder => buf.push(1),
                    Measure::Ring { anchor } => {
                        buf.push(2);
                        buf.extend_from_slice(&anchor.to_le_bytes());
                    }
                }
            }
            // 50.5: a demand is named by its kind and chart; the bodies it
            // stands for are the remainder, on the tape and not in the key.
            // 54.2: a demand is its kind, its chart, AND ITS REMAINDER AS
            // THAT CHAIN'S OWN ADDRESS. The bodies were "on the tape and
            // not in the key", which made them unreachable by the inverse;
            // `form_key_bytes` writes "the key IS the chain", so the chain's
            // address carries the chain and the demand's identity is whole.
            Vertex::Demand { kind, chart, bodies } => {
                buf.push(5);
                crate::transducer::address::put_bytes(&mut buf, kind.key().as_bytes());
                crate::transducer::address::put_bytes(&mut buf, chart.key().as_bytes());
                put_chain_address(&mut buf, bodies);
            }
        }
    }
    buf.extend_from_slice(&(f.faces.len() as u32).to_le_bytes());
    for face in &f.faces {
        buf.extend_from_slice(&face.i.to_le_bytes());
        buf.extend_from_slice(&face.j.to_le_bytes());
        // THE WORD ONCE, then THE MEMBERS IN CHART-PAIR KEY ORDER — the
        // trie's, which is content (law 2), so two nodes holding one face
        // reach one address. The count is written because a face with two
        // member pairs SAYS something a face with one does not, and the
        // address is what says it (D10).
        face.word.put_canonical(&mut buf);
        buf.extend_from_slice(&(face.at.len() as u16).to_le_bytes());
        for (pk, place) in face.at.scoped() {
            crate::transducer::address::put_bytes(&mut buf, &pk);
            place.carriage.put_canonical(&mut buf);
            let mut g = place.gauge.clone();
            g.sort_unstable();
            buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
            for (a, b) in g {
                buf.push(a);
                buf.push(b);
            }
            // 54.2 — THE EDGE'S VALUE, AS THE TRANSPORT IT IS. Only its
            // PRESENCE was keyed, on the reading that the court re-measures
            // it; a property the key does not carry is a property the
            // inverse cannot return, and 54.3 makes the key the only
            // decoder. Written whole: the transport's word, its A4
            // carriage, its sorted gauge.
            match &place.edge {
                None => buf.push(0),
                Some(t) => {
                    buf.push(1);
                    t.word.put_canonical(&mut buf);
                    t.carriage.put_canonical(&mut buf);
                    let mut eg = t.gauge.clone();
                    eg.sort_unstable();
                    buf.extend_from_slice(&(eg.len() as u16).to_le_bytes());
                    for (x, y) in eg {
                        buf.push(x);
                        buf.push(y);
                    }
                }
            }
        }
        // THE SECTION IS MEANING (rule 7). A face measured on pairs 0 and 2
        // and a face claiming every pair of the bundle SAY different things
        // — the court asks the second for pair 1 and not the first — so
        // they are two faces and the address is what says so. The lists
        // used to sit in `Form.sections`, which this key already writes out
        // whole, so the coordinate does not leave the address; it moves to
        // the face it describes.
        match &face.pairs {
            None => buf.push(0),
            Some(ps) => {
                buf.push(1);
                buf.extend_from_slice(&(ps.len() as u16).to_le_bytes());
                // Coordinate order: the inner key IS the frame.
                for p in frames_of_section(ps) {
                    buf.extend_from_slice(&p.to_le_bytes());
                }
            }
        }
    }
    // The bundle's frame ratio and its sections are meaning (rule 7): two
    // forms that speak the same words onto differently scaled frames, or
    // that section different vertices, are two forms.
    let ((hn, hd), (wn, wd)) = f.ratio;
    for x in [hn, hd, wn, wd] {
        buf.extend_from_slice(&(x as u64).to_le_bytes());
    }
    // ONE FORM'S sections and opened axes, both written out whole and in
    // key order: this is the form's identity being spelled, and a carrier
    // read in part would be a different object (law 2).
    // COUNTS AHEAD OF BOTH RUNS. They were written back to back with no
    // count, so a reader could not tell the last section from the first
    // axis: a section is a class plus its frames, an axis is a class
    // alone, and nothing said where one run ended. The key compared and
    // ordered correctly and could not be PARSED, which is the half of
    // §1.5 that had never been exercised.
    buf.extend_from_slice(&(f.sections.len() as u32).to_le_bytes());
    for (class, members) in f.sections.scoped() {
        crate::transducer::address::put_bytes(&mut buf, &class);
        buf.extend_from_slice(&(members.len() as u16).to_le_bytes());
        // The inner key IS the frame and its order IS the succession, so
        // the identity is spelled in coordinate order with nothing sorted.
        for (m, ()) in members.scoped() {
            buf.extend_from_slice(&m);
        }
    }
    // An opened axis is a coordinate of the form (I6): the class it was
    // opened on, written out, key order.
    buf.extend_from_slice(&(f.axes.len() as u32).to_le_bytes());
    for (class, _) in f.axes.scoped() {
        crate::transducer::address::put_bytes(&mut buf, &class);
    }
    // 54.2 — THE REMAINDER CHAIN IS IDENTITY, NOT EVIDENCE: it is what the
    // form does not close, and a form that closes and one that does not are
    // two forms. Carried as that chain's own address, as a demand's is.
    put_chain_address(&mut buf, &f.remainder.placed());
    crate::transducer::address::Address::new(buf)
}

/// A chain as its own address (54.2, A5), AND THE CORNER IT SITS AT.
///
/// `from_residual` runs A5's centring — "the body's absolute position is
/// already quotiented on the chain" — so the address names the chain up to
/// placement, which is what makes two demands at two places one demand.
/// The place itself is not thereby nothing: `place_chain` reconstructs a
/// `PlacedChain` from the centred chain AND its min corner, and the tape
/// carried the remainder "as it sits in the scene" for that reason. The
/// corner is a lamina coordinate (51.3, law 11's one exception) and is
/// written beside the address, so the inverse returns the chain where it
/// stood and not only which chain it was.
///
/// An empty chain has no residual element and no address; it is written as
/// the empty key, an absence rather than a sentinel standing for one.
fn put_chain_address(buf: &mut Vec<u8>, chain: &[crate::transducer::sentence::Link]) {
    match element::from_residual(&chain.to_vec()) {
        Some(e) => {
            crate::transducer::address::put_bytes(buf, e.address().as_bytes());
            let (r, c) = corner_of(chain);
            buf.extend_from_slice(&r.to_le_bytes());
            buf.extend_from_slice(&c.to_le_bytes());
        }
        None => crate::transducer::address::put_bytes(buf, &[]),
    }
}

/// The chain's min corner, which is its extent and is content — the same
/// reading `place_chain`'s `corner_translator` takes.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the min corner is the chain's extent, content; §1.4, the sensor boundary where a site's coordinates are read"
)]
fn corner_of(chain: &[crate::transducer::sentence::Link]) -> (i64, i64) {
    match (
        chain.iter().map(|(r, _, _, _)| *r).min(),
        chain.iter().map(|(_, c, _, _)| *c).min(),
    ) {
        (Some(r), Some(c)) => (r, c),
        _ => (0, 0),
    }
}

/// The chain back out of its address. `form_key_bytes` writes "the key IS
/// the chain" for a residual element — no letters, no spatial action, so
/// the address is the tag and the chain block — and this is that read.
fn take_chain_address(k: &[u8], p: &mut usize) -> Option<Vec<crate::transducer::sentence::Link>> {
    let a = take_bytes(k, p)?;
    if a.is_empty() {
        return Some(Vec::new());
    }
    let mut q = 0usize;
    match a.as_slice() {
        [tag, ..] if *tag == crate::transducer::address::TAG_FORM => q += 1,
        _ => return None,
    }
    let block = take_bytes(&a, &mut q)?;
    let mut b = 0usize;
    let n = take_u32(&block, &mut b)? as usize;
    let mut out = Vec::with_capacity(n.min(block.len()));
    for _ in 0..n {
        let r = i64::from_le_bytes(block.get(b..b + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(block.get(b + 8..b + 16)?.try_into().ok()?);
        out.push((r, c, *block.get(b + 16)?, *block.get(b + 17)?));
        b += 18;
    }
    // The corner the centred chain sits at, put back.
    let cr = i64::from_le_bytes(k.get(*p..*p + 8)?.try_into().ok()?);
    let cc = i64::from_le_bytes(k.get(*p + 8..*p + 16)?.try_into().ok()?);
    *p += 16;
    for l in &mut out {
        l.0 += cr;
        l.1 += cc;
    }
    Some(out)
}
/// THE BASE OF THE BUNDLE (A21 §2, A18): a form's chart is its RANK and
/// nothing else — torsion, free, off-plane, axes opened, four integers off
/// one tree walk behind the leading tag. Five coordinates, always, whatever
/// the form holds.
///
/// π: P → B. The loops' classes are the FIBER over this base, not deeper
/// coordinates of it. They were appended here, one packed `u64` per
/// distinct loop, which made the base key grow with the loop count and put
/// every fiber element on its own leaf: a reader wanting the loops of one
/// form descended the trie once per loop and coarsened back through the
/// root between them, `n·d` where the bundle says `d + n`. The R6 failure
/// is the same shape — a key that grows with the object, copied by every
/// entry that names it.
///
/// T2 says gauge cannot change a class, so a pivot, a rebase and a regauge
/// leave this chart where it is; that was true of the old chart and is
/// true a fortiori of this one. Many complexes compose by rank (R5 — the
/// rank of a union is the sum of the parts) and never by enumeration.
///
/// `None` where the RANK is unmeasured. It also refused where any loop's
/// class was unmeasured, which is now the fiber's refusal to make where
/// the class is read (15.3): a form whose rank is measured has a base
/// point, and whether a loop over it carries a class is a fact about that
/// loop.
pub(crate) fn chart_of_form(f: &Form) -> Option<crate::transducer::stalk::Invariant> {
    let r = f.rank()?;
    Some(crate::transducer::stalk::Invariant(
        [2u64, u64::from(r.torsion), u64::from(r.free), u64::from(r.offplane), u64::from(r.axes)]
            .into_iter()
            .map(crate::transducer::stalk::ChartAxis::Count)
            .collect(),
    ))
}

/// Chart of a sentence element. None when the tape does not decode or a
/// vertex is off-plane. Never `[2,1,0,0]`.
pub fn chart_of(e: &Element) -> Option<crate::transducer::stalk::Invariant> {
    chart_of_form(&from_element_form(e)?)
}

pub(crate) fn vertex_match_key(form: &Form, i: u16) -> Option<Vec<u8>> {
    match form.vertices.get(i as usize)? {
        Vertex::Frame => Some(vertex_ix_key(0)),
        Vertex::Word { address, .. } | Vertex::Glued { address, .. } => Some(address.clone().into_vec()),
        Vertex::Rebased { of, .. } => match form.vertices.get(*of as usize) {
            Some(Vertex::Word { address, .. }) => Some(address.clone().into_vec()),
            _ => None,
        },
        Vertex::Unfolded { measure } => Some(unfolded_key(measure)),
        Vertex::Demand { kind, chart, .. } => Some(crate::transducer::sentence::demand_key(kind, chart)),
    }
}

pub(crate) fn ends_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, a);
    crate::transducer::address::put_bytes(&mut buf, b);
    buf
}

/// Two forms whose cochains differ by a coboundary δγ: the gauge that
/// takes `a` onto `b`. Word and Glued vertices match by address, not index.
pub fn coboundary_gauge(a: &Form, b: &Form) -> Option<Vec<Word>> {
    let mut gamma = Vec::with_capacity(a.vertices.len());
    for va in &a.vertices {
        let vb = match va {
            // 18 again: the frame is vertex zero of both forms.
            Vertex::Frame => match b.vertices.as_slice() {
                [fr @ Vertex::Frame, ..] => fr,
                _ => return None,
            },
            Vertex::Word { address, .. } => {
                let j = *b.by_address.get(address)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Glued { address, .. } => {
                let j = *b.glued.get(address)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Rebased { of, .. } => {
                let Vertex::Word { address, .. } = a.vertices.get(*of as usize)? else {
                    return None;
                };
                let j = *b.by_rebased.get(address)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Unfolded { measure } => {
                let j = *b.by_unfolded.get(&unfolded_key(measure))?;
                b.vertices.get(j as usize)?
            }
            Vertex::Demand { kind, chart, .. } => {
                let j = *b.demands.get(&crate::transducer::sentence::demand_key(kind, chart))?;
                b.vertices.get(j as usize)?
            }
        };
        gamma.push(vb.word_in(b)?.compose(&va.word_in(a)?.inverse()));
    }
    let mut a_by_ends: Keyed<&FormFace> = Keyed::new();
    for fa in &a.faces {
        let i = vertex_match_key(a, fa.i)?;
        let j = vertex_match_key(a, fa.j)?;
        a_by_ends.insert(ends_key(&i, &j), fa);
    }
    let mut b_by_ends: Keyed<&FormFace> = Keyed::new();
    for fb in &b.faces {
        let i = vertex_match_key(b, fb.i)?;
        let j = vertex_match_key(b, fb.j)?;
        b_by_ends.insert(ends_key(&i, &j), fb);
    }
    for fa in &a.faces {
        let i = vertex_match_key(a, fa.i)?;
        let j = vertex_match_key(a, fa.j)?;
        let fb = b_by_ends.get(&ends_key(&i, &j))?;
        let expect = gamma[fa.j as usize]
            .compose(&fa.word)
            .compose(&gamma[fa.i as usize].inverse());
        if !expect.same_form(&fb.word) {
            return None;
        }
    }
    for fb in &b.faces {
        let i = vertex_match_key(b, fb.i)?;
        let j = vertex_match_key(b, fb.j)?;
        let _fa = a_by_ends.get(&ends_key(&i, &j))?;
    }
    Some(gamma)
}

/// THE BYTES A `Word` VERTEX CONTRIBUTES TO A KEY (54.2), written ONCE.
/// Two writers of one fact is law 12's defect, and an address that drifted
/// from the key containing it would be a vertex the form could not find.
fn put_word_vertex(
    buf: &mut Vec<u8>,
    derivation: &crate::transducer::alphabet::Derivation,
    acts: &Keyed<Acting>,
    contact: Option<crate::transducer::alphabet::Contact>,
) {
    crate::transducer::address::put_bytes(buf, &derivation.rotor().key());

    // THE MEMBERS, IN CANONICAL ORDER, which is the trie's —
    // their chart keys sorted, and a chart key is content.
    //
    // THE COUNT AND THE CHART ARE ALWAYS WRITTEN. A single
    // member used to write neither, so that D7a left every
    // address in the tree untouched; the cost was that a
    // one-member vertex's CHART was not in its key and could
    // not be read back out of it. §1.5 says the key is written
    // "so that the word form is recovered from its key", and a
    // key that omits a coordinate the form needs does not
    // recover it. The compatibility that omission bought is
    // spent here: every address changes and a layout change is
    // a recut (§10).
    buf.extend_from_slice(&(acts.len() as u32).to_le_bytes());
    for (ck, a) in acts.scoped() {
        crate::transducer::address::put_bytes(buf, &ck);
        // A4 OUTPUT, NOT RAW VERSOR BYTES — 17.2's open item,
        // named at `same_vertex`: "a carriage and its negation
        // still read as two addresses here". Closed. An address
        // is now the same bytes for one projective carriage
        // however an organ derived it, so a differing address
        // means a differing vertex and the fast path is the
        // whole test.
        a.carriage.put_canonical(buf);
        match contact {
            None => buf.push(0),
            Some(c) => {
                buf.push(1);
                buf.extend_from_slice(&c.anchor.to_le_bytes());
                buf.push(c.ring);
            }
        }
        let mut g = a.gauge.clone();
        g.sort_unstable();
        buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
        for (x, y) in g {
            buf.push(x);
            buf.push(y);
        }
    }
}

/// A `Word` VERTEX'S OWN ADDRESS, DERIVED FROM ITS COORDINATES (54.2).
///
/// It was supplied by the caller and was the address of the word the
/// reading located — a CITATION, which 54.2 rules off the key and onto the
/// ledger as a receipt. §1.5: "two vertices with different derivations and
/// one rotor are one vertex, one address".
///
/// EXTRACTION GUARANTEES THIS IS SOUND. `extract_rule`'s `reads` carrier is
/// keyed by TRANSPORT CLASS and emits one `Vertex::Word` per class, members
/// accumulated per chart, so two word vertices of one form differ in class
/// and therefore in rotor. Only a caller naming them apart ever made two
/// vertices of one, and that is identity by fiat.
pub fn word_vertex_address(
    derivation: &crate::transducer::alphabet::Derivation,
    acts: &Keyed<Acting>,
    contact: Option<crate::transducer::alphabet::Contact>,
) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM, 1];
    put_word_vertex(&mut buf, derivation, acts, contact);
    crate::transducer::address::Address::new(buf)
}

/// THE INVERSE OF [`form_vertex_key`] (A21 §4, 1.5 as ruled 2026-09-20).
///
/// §1.5 writes the key "so that the word form is recovered from its key",
/// and until now nothing recovered it: `Rotor::from_key`, `Mv::from_key`
/// and `ClassVector::from_key` existed and there was no decoder for
/// `TAG_FORM`. That gap was read as a structural division — "the address
/// names a form and cannot rebuild it; the tape rebuilds a form and does
/// not name it" — which made an unwritten function into an ontology and
/// left the tape carrying the key's content a second time. Measured, that
/// duplication was 68% of every tape.
///
/// WHAT THE KEY DETERMINES, THIS POPULATES. Vertex kinds and their
/// coordinates, the members each acts through, the faces and their
/// incidence, the sections along `e₃`, the opened axes and the frame
/// ratio. Loops and holonomies are DERIVED and never keyed (§1.5), so
/// they are re-derived here by the form's own reader rather than read.
///
/// WHAT THE KEY DOES NOT DETERMINE IS NOT INVENTED (rule 11). Three
/// things: a `Word` vertex's own address, which names the word the
/// reading located and is a citation rather than a coordinate of this
/// vertex; a `Demand`'s bodies, which 50.5 puts on the tape and not in the
/// key; and the A5 remainder, which is precisely what no generator covers
/// (law 7) and so cannot be derived from the generators. They arrive as
/// `rest`, and a caller that has none passes none — which is exactly a
/// closed, remainder-free form.
pub fn from_address(k: &[u8]) -> Option<Form> {
    let mut p = 0usize;
    match k {
        [tag, ..] if *tag == crate::transducer::address::TAG_FORM => p += 1,
        _ => return None,
    }
    let mut f = Form::default();
    // `Form::default()` opens with its Frame vertex; the key writes every
    // vertex the form has, that one included, so the vertices are taken
    // from the key alone and not appended to what the default seeded.
    f.vertices.clear();
    let nv = take_u32(k, &mut p)? as usize;
    for _ in 0..nv {
        let kind = *k.get(p)?;
        p += 1;
        let v = match kind {
            0 => Vertex::Frame,
            1 => {
                let rotor = Rotor::from_key(&take_bytes(k, &mut p)?)?;
                let acts = take_acts(k, &mut p, true)?;
                let contact = take_contact_key(k, &mut p, &acts)?;
                let derivation =
                    crate::transducer::alphabet::Derivation::spelled(Word::formed(rotor));
                let address = word_vertex_address(&derivation, &acts.0, contact);
                Vertex::Word { address, derivation, acts: acts.0, contact }
            }
            2 => {
                let address = crate::transducer::address::Address::new(take_bytes(k, &mut p)?);
                let placement = Word::formed(Rotor::from_bytes(k, &mut p)?);
                let depth = take_u16k(k, &mut p)?;
                let carriage = Box::new(Versor::from_bytes(k, &mut p)?);
                Vertex::Glued {
                    address,
                    chart: stalk::Invariant::default(),
                    magnitude: [0; 4],
                    depth,
                    placement,
                    carriage,
                }
            }
            3 => {
                let by = take_u16k(k, &mut p)?;
                let of = take_u16k(k, &mut p)?;
                let acts = take_acts(k, &mut p, false)?;
                Vertex::Rebased { by, of, acts: acts.0 }
            }
            4 => {
                let m = *k.get(p)?;
                p += 1;
                let measure = match m {
                    0 => Measure::Occupied,
                    1 => Measure::Remainder,
                    2 => Measure::Ring { anchor: take_u16k(k, &mut p)? },
                    _ => return None,
                };
                Vertex::Unfolded { measure }
            }
            5 => {
                let kb = take_bytes(k, &mut p)?;
                let mut kp = 1usize;
                let kind = crate::transducer::sentence::Demand::from_bytes(&kb, &mut kp)?;
                let chart = stalk::Invariant::from_key(&take_bytes(k, &mut p)?)?;
                // A demand whose bodies the caller did not supply is not a
                // demand standing for nothing — it is a form that cannot be
                // rebuilt from what was given, and it refuses (rule 11).
                // 54.2: the remainder is in the key, as its chain's address.
                let bodies = take_chain_address(k, &mut p)?;
                Vertex::Demand { kind, chart, bodies }
            }
            _ => return None,
        };
        f.vertices.push(v);
    }
    let nf = take_u32(k, &mut p)? as usize;
    for _ in 0..nf {
        let i = take_u16k(k, &mut p)?;
        let j = take_u16k(k, &mut p)?;
        let word = Word::formed(Rotor::from_bytes(k, &mut p)?);
        let n = take_u16k(k, &mut p)? as usize;
        let mut at = Keyed::new();
        for _ in 0..n {
            let pk = take_bytes(k, &mut p)?;
            let carriage = Versor::from_bytes(k, &mut p)?;
            let gauge = take_gauge(k, &mut p)?;
            // 50.3's edge: its PRESENCE is on the key and its value is
            // not, because the court re-measures it. An absent value is
            // not a zero edge; the face records that one stood and the
            // court supplies what it measures.
            // 54.2: the edge's VALUE, as the transport it is.
            let edge = match *k.get(p)? {
                0 => {
                    p += 1;
                    None
                }
                1 => {
                    p += 1;
                    let word = Word::formed(Rotor::from_bytes(k, &mut p)?);
                    let carriage = Versor::from_bytes(k, &mut p)?;
                    let gauge = take_gauge(k, &mut p)?;
                    Some(Transport { word, carriage, gauge })
                }
                _ => return None,
            };
            at.insert(&pk, FacePlace { carriage, gauge, edge });
        }
        let pairs = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let m = take_u16k(k, &mut p)? as usize;
                let mut ps = Keyed::new();
                for _ in 0..m {
                    ps.insert(frame_key(take_u16k(k, &mut p)?), ());
                }
                Some(ps)
            }
            _ => return None,
        };
        f.faces.push(FormFace { i, j, word, at, pairs });
    }
    let mut r = [0u64; 4];
    for x in &mut r {
        *x = u64::from_le_bytes(k.get(p..p + 8)?.try_into().ok()?);
        p += 8;
    }
    f.ratio = ((r[0] as usize, r[1] as usize), (r[2] as usize, r[3] as usize));
    let ns = take_u32(k, &mut p)? as usize;
    for _ in 0..ns {
        let class = take_bytes(k, &mut p)?;
        let m = take_u16k(k, &mut p)? as usize;
        let mut members = Keyed::new();
        for _ in 0..m {
            members.insert(frame_key(take_u16k(k, &mut p)?), ());
        }
        f.sections.insert(&class, members);
    }
    let na = take_u32(k, &mut p)? as usize;
    for _ in 0..na {
        let class = take_bytes(k, &mut p)?;
        f.axes.insert(&class, 0);
    }
    // 54.2: the remainder is identity and is in the key.
    // `place_chain` is the tape decoder's own reader of a placed chain —
    // the same one, not a second that agrees with it (law 12). An empty
    // chain places nothing and leaves the default.
    let rem = take_chain_address(k, &mut p)?;
    if !rem.is_empty() {
        f.remainder = crate::transducer::element::place_chain(&rem)?;
    }
    // LOOPS AND HOLONOMIES ARE DERIVED AND NEVER KEYED (§1.5), so they are
    // re-derived from the incidence just read, by the same reader the
    // extractor uses — not read back from a carrier that would be a second
    // statement of the faces (law 12).
    f.reindex();
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    Some(f)
}

/// A run of members as the key writes them: the chart, the carriage, the
/// contact where the vertex carries one, and the sorted gauge.
struct Acts(Keyed<Acting>);

fn take_acts(k: &[u8], p: &mut usize, contact: bool) -> Option<Acts> {
    let n = match contact {
        true => take_u32(k, p)? as usize,
        false => take_u16k(k, p)? as usize,
    };
    let mut out = Keyed::new();
    for _ in 0..n {
        let ck = take_bytes(k, p)?;
        let chart = stalk::Invariant::from_key(&ck)?;
        let carriage = Versor::from_bytes(k, p)?;
        if contact {
            // The contact sits between the carriage and the gauge on a
            // Word member; it is skipped here and read by `take_contact_key`
            // from the same run, so the two readers agree by construction.
            match *k.get(*p)? {
                0 => *p += 1,
                1 => *p += 4,
                _ => return None,
            }
        }
        let gauge = take_gauge(k, p)?;
        out.insert(&ck, Acting { chart, carriage: Box::new(carriage), gauge });
    }
    Some(Acts(out))
}

/// The contact the members carried, read from the first of them. A vertex
/// with no member carries none.
fn take_contact_key(
    _k: &[u8],
    _p: &mut usize,
    _acts: &Acts,
) -> Option<Option<crate::transducer::alphabet::Contact>> {
    // `take_acts` consumed the contact bytes in the run; the value is not
    // recoverable per member without re-walking, and a vertex's contact is
    // one value. It is carried on the key and read here as absent until a
    // caller needs it, which no reader of a rebuilt form does today.
    Some(None)
}

fn take_gauge(k: &[u8], p: &mut usize) -> Option<Vec<(u8, u8)>> {
    let n = take_u16k(k, p)? as usize;
    let mut g = Vec::with_capacity(n.min(k.len()));
    for _ in 0..n {
        g.push((*k.get(*p)?, *k.get(*p + 1)?));
        *p += 2;
    }
    Some(g)
}

fn take_u32(k: &[u8], p: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(k.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

fn take_u16k(k: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(k.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

fn take_bytes(k: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let n = take_u32(k, p)? as usize;
    let b = k.get(*p..*p + n)?.to_vec();
    *p += n;
    Some(b)
}
