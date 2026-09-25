// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 17.4, 3.3, 31 — THE CHART OF A FORM: its M4 coordinates, the glue of a
//! crystallized form into it, the coboundary gauge between two forms, the
//! A21 vertex key, and the `SentenceRule` a form is spoken as.

use super::form::*;
use super::address::from_element_form;
use super::transport::*;
use crate::network::nerve::{Coordinate, Keyed};
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, Element, Versor};

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
    /// Drop the named vertices (keyed by old index), drop faces that touched
    /// them, remap surviving faces through the old-to-new map, rebuild orbits.
    ///
    /// `false` where the retracted form's depth does not read (STD1,
    /// `depth_of`): the form is no form this build can write, and the caller
    /// refuses it rather than hold it at a stale depth.
    #[must_use]
    pub fn retract_vertices(&mut self, drop: &Keyed<(), crate::network::nerve::coord::VertexIx>) -> bool {
        #[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
        RETRACTIONS.with(|c| c.set(c.get().saturating_add(1)));
        let n = self.vertices.len();
        if n == 0 {
            return true;
        }
        let mut old_to_new: Vec<Option<u16>> = vec![None; n];
        let mut new_verts = Vec::new();
        for (i, v) in self.vertices.iter().enumerate() {
            if i != 0 && drop.holds(&vertex_ix_key(i as u16)) {
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
                interface: f.interface.clone(),
            });
        }
        // RENUMBERING A FORM TOUCHES EVERY CARRIER IT HAS. Each of the four
        // below is this form's own — glued vertices, sections, torsion,
        // opened axes — and each is rebuilt whole against the new indices,
        // because an entry left behind names a vertex that no longer
        // exists. There is no prefix to follow: the form IS the scope.
        let mut glued: Keyed<u16, crate::network::nerve::coord::Site> = Keyed::new();
        for (k, &old_i) in self.glued.entries() {
            if let Some(&Some(ni)) = old_to_new.get(old_i as usize) {
                glued.put(&k, ni);
            }
        }
        // AND THE COVER LOSES THE BONDS WHOSE PORTS WENT (54.7 B5a).
        //
        // `glued` is remapped above and `cover` was not, so retracting a
        // `Glued` vertex left its `NerveEdge` standing at a site no
        // boundary reaches — a PHANTOM EDGE, which is the direction that
        // does not fail safe: it would resolve, replay and be paid for
        // against a boundary the complex no longer holds.
        //
        // Kept by SITE and not by index: `glued` above is the surviving
        // sites, and an edge belongs exactly when its host is one of them
        // (rule 15).
        self.cover.retain(|e| {
            crate::network::nerve::coord::Site::decode(e.host().as_bytes())
                .is_ok_and(|s| glued.holds(&s))
        });
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
        let mut torsion: Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy> = Keyed::new();
        for (k, FiledOrbit { sigma, periods, chord }) in self.torsion.entries() {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            // THE HUE MAP IS NOT RENUMBERED. `sigma` is the hue-map
            // generator — a permutation of HUES — and this ran every entry
            // through `old_to_new`, the VERTEX table: colours relabelled as
            // if they were vertices, and any cycle naming a hue above the
            // vertex count dropped the whole filed orbit through the
            // `else { continue }`. Renumbering a form's vertices moves no
            // hue. The chord is a face index and is remapped, above.
            torsion.put(&k, FiledOrbit { sigma: sigma.clone(), periods: *periods, chord: nc });
        }
        let mut axes: Keyed<u16, crate::network::nerve::coord::Holonomy> = Keyed::new();
        for (k, chord) in self.axes.entries() {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            axes.put(&k, nc);
        }
        self.vertices = new_verts;
        self.faces = new_faces;
        self.glued = glued;
        self.sections = sections;
        self.torsion = torsion;
        self.axes = axes;
        self.orbits = Self::orbits_of(&self.faces, &self.vertices);
        let Some(depth) = self.depth_of() else {
            return false;
        };
        self.depth = depth;
        self.reindex();
        true
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
            // `g p g̃`: the carriage conjugated as a ray (78.32(1)).
            crate::transducer::cga::is_versor(&gv).then_some(())?;
            let inv = gv.reverse();
            let mut at = Keyed::new();
            for (k, p) in f.at.scoped() {
                at.insert(
                    &k,
                    FacePlace {
                        carriage: gv.geo(&p.carriage).geo(&inv),
                        gauge: p.gauge.clone(),
                        edge: p.edge.clone(),
                        refraction: p.refraction.clone(),
                    },
                );
            }
            next.push((word, at));
        }
        for (f, (word, at)) in self.faces.iter_mut().zip(next) {
            f.word = word;
            f.at = at;
        }
        self.orbits = Self::orbits_of(&self.faces, &self.vertices);
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
    pub sections: Keyed<Keyed<()>, crate::network::nerve::coord::SectionAt>,
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



// 54.7 B6 — `vertex_address` IS DELETED, AND IT WAS THE LAST READER OF A
// SENTENCE TAPE IN THE TREE.
//
// It recovered a form's key by parsing `Element.description` under the
// `FORM_TAPE_VERSION` gate, for an element assembled by hand from a tape
// and never told its name. That path is what made a word depend on a byte
// buffer to know its own coordinate, and it is what kept `is_sentence`
// reading a magic byte: the address was derived from the tape, so the
// kind could not be read off the address.
//
// A form's address is SET by the mint (`as_element_form`, 54.5) because
// the mint holds the form and the key IS the form (law 2), and the GEN3
// record carries it and refuses an empty one rather than re-deriving at a
// guess (rule 11). An element with neither is not a word this build holds,
// and answering one for it would be inventing a coordinate.

pub fn form_vertex_key(f: &Form) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM];
    // 71.2(1), BQ1 — THE BASE'S `n` FIRST. It is the coarsest coordinate a
    // form has: two forms read on bases of two `n` share no stratum below
    // the tag, and every form of one base shares this prefix.
    buf.push(f.n);
    // THE COUNT IS OF WHAT IS WRITTEN, and until 54.5 it was not.
    //
    // It read `f.vertices.len()` while the body below walks
    // `canonical_vertices`, which reads distinct vertices out of a trie
    // keyed by each one's own coordinate bytes. Those two numbers differ
    // whenever a form holds two vertices with ONE coordinate — which §1.5
    // says ARE one vertex — and then the key claimed n records and carried
    // fewer. The address compared and ordered correctly and could not be
    // PARSED: `from_address` read the count, ran out of bytes on the last
    // record and refused the whole form.
    //
    // The tape hid it for as long as the tape was the decoder. `star_form`
    // builds its vertices from one fixture, so two of them are one
    // coordinate, and every reading taken through the tape's decoder (its
    // codec retired at LD3-c) was
    // fine while the same form's ADDRESS was unreadable. Two writers of one
    // fact, and the one that was never read is the one that was wrong
    // (law 12).
    let vs = canonical_vertices(f);
    // Every count in this key is written by its own coordinate (STD1 3c-2,
    // ruled (d)): LEB128, no width wall, read back by the inverse.
    crate::transducer::address::put_key_count(&mut buf, vs.len());
    // THE VERTICES IN THEIR OWN CANONICAL ORDER, not the order they were
    // appended (law 2, rule 15).
    //
    // This walked `f.vertices` as a `Vec`, so two forms holding ONE set of
    // vertices reached two addresses if they were built in two orders —
    // arrival in the address, measured at `63d6e40`. The order is now each
    // vertex's own coordinate bytes, read back from a `Keyed`: the trie's
    // order IS the coordinates' order and is content, the same reading
    // every other carrier in the tree takes.
    //
    // THE FRAME STAYS FIRST. "A form opens at its frame" (18) and vertex
    // zero is the frame by the form's own shape — `glue_append` reads it
    // with `[frame @ Vertex::Frame, ..]`. It is the basepoint, not a member
    // of the set being ordered.
    for v in vs {
        put_vertex(&mut buf, v);
    }
    crate::transducer::address::put_key_count(&mut buf, f.faces.len());
    // A face names its ends by their place in the CANONICAL order, so the
    // incidence is read against the same ordering the vertices were
    // written in and not against the vector they happen to sit in.
    let at = canonical_places(f);
    // AND THE FACES THEMSELVES IN THEIR OWN ORDER, not the order they were
    // pushed (law 2, rule 15). `ba645e3` canonicalised the VERTICES and
    // left this walk over `f.faces` as a `Vec`, so arrival order stayed in
    // the address one level down: two hosts citing one pair of forms in two
    // orders reached two addresses, with the same vertices at the same
    // canonical places and their two star faces written in opposite order.
    // `a_citation_set_is_not_an_arrival_order` is the reading.
    for face in canonical_faces(f, &at) {
        put_face(&mut buf, face, &at);
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
    crate::transducer::address::put_key_count(&mut buf, f.sections.len());
    for (class, members) in f.sections.scoped() {
        crate::transducer::address::put_key_bytes(&mut buf, &class);
        crate::transducer::address::put_key_count(&mut buf, members.len());
        // The inner key IS the frame and its order IS the succession, so
        // the identity is spelled in coordinate order with nothing sorted.
        //
        // THE FRAME IS WRITTEN AS ITS KEY, AND READ AS ITS KEY (54.5, as
        // ruled 2026-09-20). `frame_key` is `to_be_bytes` and that is
        // load-bearing: the field's own doc says "the inner key is
        // big-endian, so the trie's own order is the frame's order and a
        // reader that enumerates a section gets the frames in succession
        // without sorting them (law 2)".
        //
        // The writer wrote those bytes and `from_address` read them with
        // `take_u16k`, which is LITTLE-endian, then re-encoded big. So
        // `0x0001` went out as `[0,1]`, came back as 256, and re-keyed as
        // `[1,0]`: the inverse was not a retract for any form carrying a
        // section, and every such form had two addresses. The corpus guard
        // found it on the first task that has one.
        //
        // Writing little-endian here would have agreed with that reader and
        // given a frame TWO spellings — the carrier's and the key's — which
        // is the defect one level up. The reader reads the key instead.
        for (m, ()) in members.scoped() {
            buf.extend_from_slice(&m);
        }
    }
    // An opened axis is a coordinate of the form (I6): the class it was
    // opened on, written out, key order.
    crate::transducer::address::put_key_count(&mut buf, f.axes.len());
    for (class, _) in f.axes.scoped() {
        crate::transducer::address::put_key_bytes(&mut buf, &class);
    }
    // 54.2 — THE REMAINDER CHAIN IS IDENTITY, NOT EVIDENCE: it is what the
    // form does not close, and a form that closes and one that does not are
    // two forms. Carried as that chain's own address, as a demand's is.
    put_chain_address(&mut buf, &f.remainder.placed());
    crate::transducer::address::Address::new(buf)
}

/// ONE FACE'S OWN BYTES — its ends at their canonical places, its word, its
/// members and the section it was measured on. Factored out of
/// [`form_vertex_key`] so the canonical ORDER of the faces is read from the
/// same bytes the key writes, and the two cannot drift (law 12).
fn put_face(buf: &mut Vec<u8>, face: &FormFace, at: &[u16]) {
    // `canonical_places` covers every index a face names, so this is a
    // total read and there is no missing value to stand in for (rule 11).
    buf.extend_from_slice(&at[face.i as usize].to_le_bytes());
    buf.extend_from_slice(&at[face.j as usize].to_le_bytes());
    {
        // THE WORD ONCE, then THE MEMBERS IN CHART-PAIR KEY ORDER — the
        // trie's, which is content (law 2), so two nodes holding one face
        // reach one address. The count is written because a face with two
        // member pairs SAYS something a face with one does not, and the
        // address is what says it (D10).
        face.word.put_canonical(buf);
        crate::transducer::address::put_key_count(buf, face.at.len());
        for (pk, place) in face.at.scoped() {
            crate::transducer::address::put_key_bytes(buf, &pk);
            place.carriage.put_canonical(buf);
            let mut g = place.gauge.clone();
            g.sort_unstable();
            crate::transducer::address::put_key_count(buf, g.len());
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
                    t.word.put_canonical(buf);
                    t.carriage.put_canonical(buf);
                    let mut eg = t.gauge.clone();
                    eg.sort_unstable();
                    crate::transducer::address::put_key_count(buf, eg.len());
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
                crate::transducer::address::put_key_count(buf, ps.len());
                // Coordinate order: the inner key IS the frame, and it is
                // written AS that key — big-endian, the one spelling the
                // carrier uses (54.5). It went out through
                // `frames_of_section` and `to_le_bytes`, which is a second
                // spelling of a frame and the thing that let the sections
                // run above disagree with its own reader.
                for (m, ()) in ps.scoped() {
                    buf.extend_from_slice(&m);
                }
            }
        }
        // BQ5b, 72.2 — THE INTERFACE, WHOLE: `V`'s ray by its canonical
        // coordinates (VA1), then the contact's grade, behind a flag. The
        // shared-cell set is the witness of whether and enters no address.
        // A face that is not a contact, or whose WHAT did not read, writes 0.
        match &face.interface {
            Some(i) => {
                buf.push(1);
                i.put(buf);
            }
            None => buf.push(0),
        }
    }
}

/// ONE FACE'S BYTES, for ordering by them.
fn face_bytes(face: &FormFace, at: &[u16]) -> Vec<u8> {
    let mut b = Vec::new();
    put_face(&mut b, face, at);
    b
}

/// The form's faces in canonical order: each by its own written bytes, read
/// back from the trie whose order those bytes are — the same reading
/// [`canonical_vertices`] takes one level up.
///
/// TWO FACES MAY WRITE THE SAME BYTES and both are kept. A `Keyed` holds one
/// value per key, so the index is from the bytes to the LIST of faces that
/// write them; a form with a face repeated says something a form with it
/// once does not, and the count written ahead of this run is what says so.
fn canonical_faces<'a>(f: &'a Form, at: &[u16]) -> Vec<&'a FormFace> {
    let mut by: Keyed<Vec<u16>> = Keyed::new();
    for (i, face) in f.faces.iter().enumerate() {
        let k = face_bytes(face, at);
        match by.get_mut(&k) {
            Some(v) => v.push(i as u16),
            None => {
                by.insert(&k, vec![i as u16]);
            }
        }
    }
    let mut out: Vec<&FormFace> = Vec::with_capacity(f.faces.len());
    for (_, ixs) in by.scoped() {
        for i in ixs {
            if let Some(face) = f.faces.get(*i as usize) {
                out.push(face);
            }
        }
    }
    out
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
    if chain.is_empty() {
        buf.push(0);
        return;
    }
    match element::from_residual(&chain.to_vec()) {
        Some(e) => {
            buf.push(1);
            crate::transducer::address::put_key_bytes(buf, e.address().as_bytes());
            let (r, c) = corner_of(chain);
            buf.extend_from_slice(&r.to_le_bytes());
            buf.extend_from_slice(&c.to_le_bytes());
        }
        // A CHAIN A5 REFUSES IS WRITTEN OUT, NOT DROPPED (54.5, as ruled
        // 2026-09-20). `canonical_chain` answers `None` where two distinct
        // transitions stand on one site (A5 step 3), and that refusal is
        // right — such a chain has no centring and therefore no address.
        //
        // This wrote the EMPTY key for it, which is an absence standing for
        // a refusal: a demand whose bodies A5 cannot address keyed exactly
        // as one with no bodies at all, so two demands carrying different
        // unaddressable chains reached ONE address. Measured on the second
        // corpus form the extractor builds: a demand of 25 links came back
        // with 0 (`from_address_retracts_every_corpus_form`).
        //
        // The links ARE the coordinate, so they are written out (law 2,
        // rule 6) — the same road 51.3 takes for a lamina. It costs 18
        // bytes a link and it keeps the form's identity, which is not
        // optional.
        None => {
            buf.push(2);
            crate::transducer::address::put_key_count(buf, chain.len());
            for &(r, c, before, after) in chain {
                buf.extend_from_slice(&r.to_le_bytes());
                buf.extend_from_slice(&c.to_le_bytes());
                buf.push(before);
                buf.push(after);
            }
        }
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

/// THE BASE OF THE BUNDLE (A21 §2, A18): a form's chart is its RANK and
/// nothing else — torsion, free, off-plane, axes opened, four integers off
/// one keyed read behind the leading tag. Five coordinates, always, whatever
/// the form holds.
///
/// π: P → B. The orbits' classes are the FIBER over this base, not deeper
/// coordinates of it. They were appended here, one packed `u64` per
/// distinct orbit, which made the base key grow with the orbit count and put
/// every fiber element on its own leaf: a reader wanting the orbits of one
/// form followed a prefix into the trie once per orbit and coarsened back through the
/// root between them, `n·d` where the bundle says `d + n`. The R6 failure
/// is the same shape — a key that grows with the object, copied by every
/// entry that names it.
///
/// T2 says gauge cannot change a class, so a pivot, a rebase and a regauge
/// leave this chart where it is; that was true of the old chart and is
/// true a fortiori of this one. Many complexes compose by rank (R5 — the
/// rank of a union is the sum of the parts) and never by enumeration.
///
/// `None` where the RANK is unmeasured. It also refused where any orbit's
/// class was unmeasured, which is now the fiber's refusal to make where
/// the class is read (15.3): a form whose rank is measured has a base
/// point, and whether an orbit over it carries a class is a fact about that
/// orbit.
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

/// One coordinate's canonical bytes, for a site that must hold several
/// systems' keys side by side and compare them as bytes.
fn encoded<C: crate::network::nerve::Coordinate>(c: &C) -> Vec<u8> {
    c.encode().as_bytes().to_vec()
}

/// THE KEY A VERTEX IS MATCHED BY, ACROSS THE FIVE KINDS.
///
/// Loose bytes deliberately: this is a union over five different coordinate
/// systems — an ordinal for the Frame, a rotor address for a Word, a site
/// for a citation, a measure for an Unfolded, a demand vertex for a Demand —
/// and it answers "is this the same vertex", not "where does this stand".
/// Each arm leaves its own coordinate at the point it does so (54.7 R5).
pub(crate) fn vertex_match_key(form: &Form, i: u16) -> Option<Vec<u8>> {
    match form.vertices.get(i as usize)? {
        Vertex::Frame => Some(encoded(&vertex_ix_key(0))),
        Vertex::Word { address, .. } => Some(address.clone().into_vec()),
        // 54.7 B5a — A CITATION IS MATCHED BY ITS SITE. It was matched by
        // the form it cites, which is the cover's fact and no longer on the
        // vertex; the boundary's own record is what this chart holds.
        v @ Vertex::Glued { .. } => Some(vertex_bytes(v)),
        Vertex::Rebased { of, .. } => match form.vertices.get(*of as usize) {
            Some(Vertex::Word { address, .. }) => Some(address.clone().into_vec()),
            _ => None,
        },
        Vertex::Unfolded { measure } => Some(encoded(&unfolded_key(measure))),
        Vertex::Demand { kind, chart, .. } => {
            Some(encoded(&crate::transducer::sentence::demand_key(kind, chart)))
        }
    }
}

pub(crate) fn ends_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_key_bytes(&mut buf, a);
    crate::transducer::address::put_key_bytes(&mut buf, b);
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
                let j = *b
                    .by_address
                    .at(&crate::network::nerve::coord::Member::decode(address.as_bytes()).ok()?)?;
                b.vertices.get(j as usize)?
            }
            v @ Vertex::Glued { .. } => {
                let j = *b
                    .glued
                    .at(&crate::network::nerve::coord::Site::decode(&vertex_bytes(v)).ok()?)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Rebased { of, .. } => {
                let Vertex::Word { address, .. } = a.vertices.get(*of as usize)? else {
                    return None;
                };
                let j = *b
                    .by_rebased
                    .at(&crate::network::nerve::coord::Member::decode(address.as_bytes()).ok()?)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Unfolded { measure } => {
                let j = *b.by_unfolded.at(&unfolded_key(measure))?;
                b.vertices.get(j as usize)?
            }
            Vertex::Demand { kind, chart, .. } => {
                let j = *b.demands.at(&crate::transducer::sentence::demand_key(kind, chart))?;
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

/// ONE VERTEX'S CONTRIBUTION TO A KEY, written ONCE (law 12). It stood
/// inline in `form_vertex_key`'s loop, and `vertex_bytes` needs the same
/// bytes to order the vertices by their own coordinates; two writers of
/// one fact would let the order disagree with what is written.
///
/// `pub(crate)` for the third reader of that fact: [`NerveEdge::host`] is
/// the site a nerve edge re-attaches at, which IS a `Glued` vertex's key
/// contribution. It hand-wrote the record beside this arm until 2026-09-21,
/// and two byte runs that agree is identity by bytes — law 2, §1.5.
pub(crate) fn put_vertex(buf: &mut Vec<u8>, v: &Vertex) {
    match v {
        Vertex::Frame => buf.push(0),
        // 1.5: the key is the COMPOSED product's canonical rotor, so
        // two minds whose derivations differ reach one address. The
        // citations are content the court replays (50.2 step 2) and
        // are deliberately not keyed: keying them would fork one
        // vertex into one address per presentation.
        Vertex::Word { derivation, acts, contact, .. } => {
            buf.push(1);
            put_word_vertex(buf, derivation, acts, *contact);
        }
        // 54.7 (B5) — A CITATION IS A BOUNDARY, AND THE KEY WILL WRITE THE
        // BOUNDARY ALONE (Road (d), as ruled 2026-09-21). THE EVICTION IS
        // NOT CUT HERE YET, AND THE ORDER IS THE WHOLE POINT.
        //
        // Inlining `A_c` makes the address space PROVABLY ACYCLIC. If
        // `key(A)` contains `key(B)` then `|A| > |B|`, so by well-ordering
        // of lengths no cycle `A → … → A` can exist at all — the cognitive
        // manifold is forced to be a directed acyclic forest by its own
        // naming scheme, and `b₁` of the nerve is zero by construction.
        // `two_charts_may_cite_each_other` is the reading: an edge must be
        // its own object because `key(U) ⊃ key(V)` in both directions is
        // impossible.
        //
        // WHAT THE HOST SAYS AND WHAT THE COVER SAYS ARE TWO FACTS. The
        // host form states "I have an open boundary that mates under φ at
        // a chart of this rank and this depth" — every coordinate of that
        // is LOCAL and bounded. WHICH form the boundary was instantiated
        // against is the cover's fact, not the chart's, and it rides
        // `ReceiptsEnvelope.nerve` as a `NerveEdge` carrying the full
        // affine transition.
        //
        // WHY THE ADDRESS IS STILL WRITTEN. Cut on 2026-09-21 and run, the
        // eviction took ten readings down with ONE cause: a `Form` decoded
        // from a key can no longer name what it cites, and six live readers
        // still asked the form. The sharpest is `rule_matches_tests` — the
        // court speaking a submitted rule onto withheld tests — whose form
        // comes from `sentence_rule_of` → `from_element_form`, so a rule
        // that cites would paint nothing at the court while the client
        // painted it, and every citing solve would fail verification.
        //
        // AND THE COVER IS NOT RECOVERABLE HERE, not even in multiplicity:
        // `canonical_vertices` files vertices by `vertex_bytes`, so once
        // the target leaves, two citations of two forms at one site are ONE
        // vertex with one record. Re-attaching an envelope edge to it on
        // decode would be pairing by position, and a position is not a
        // coordinate (rule 15). The readers must move to the envelope; the
        // form cannot be made to answer.
        //
        // So the eviction lands AFTER the readers, not before: while both
        // carriers hold the cover, each migrated reader is checked against
        // the key's answer, which is a guard neither carrier gives alone.
        // The eviction and its two readings are parked at
        // `scratchpad/B5-full.patch`.
        //
        // Measured before the eviction on var/1 at 288 sentences: key
        // 64 923 B/word, of which 97.94% is cited addresses — 621
        // citations naming 440 distinct forms at ~29 490 B each, against
        // 1 336 B for a form's own cells.
        Vertex::Glued {
            placement,
            depth,
            carriage,
            chart,
            ..
        } => {
            buf.push(2);
            placement.put_canonical(buf);
            buf.extend_from_slice(&depth.to_le_bytes());
            carriage.put_canonical(buf);
            // AND THE CHART THE CITATION STANDS AT (54.5, as ruled
            // 2026-09-20). It is the cited form's RANK — five counts,
            // `[2, torsion, free, offplane, axes]` — and `from_address`
            // cannot re-derive it, because that needs the cited FORM and
            // the decoder holds only its address.
            //
            // Law 12 asks for a theorem rather than a stored fact, and this
            // is not one of this key: re-deriving it requires an external
            // resolver, so it is a coordinate OF THE CITATION — what kind
            // of chart is being cited — and the key writes it out.
            //
            // It was defaulted on decode, so every citing form came back
            // with `Invariant::default()` on its `Glued` vertices and the
            // court refused the placement face: "a vertex does not act on
            // the chart the pair names". Bounded, `O(1)` per citation.
            crate::transducer::address::put_key_bytes(buf, chart.key().as_bytes());
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
            crate::transducer::address::put_key_count(buf, acts.len());
            for (_, a) in acts.scoped() {
                // THE CHART, NOT THE CARRIER'S KEY (54.7 R1) — the same
                // rule as the Word arm: `take_acts` parses an `Invariant`
                // out of what is written here, and `Acting::key` is
                // `chart ‖ carriage`.
                crate::transducer::address::put_key_bytes(buf, a.chart.key().as_bytes());
                // A4 output here as in the Word arm (17.2).
                a.carriage.put_canonical(buf);
                let mut g = a.gauge.clone();
                g.sort_unstable();
                crate::transducer::address::put_key_count(buf, g.len());
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
            crate::transducer::address::put_key_bytes(buf, kind.key().as_bytes());
            crate::transducer::address::put_key_bytes(buf, chart.key().as_bytes());
            put_chain_address(buf, bodies);
        }
    }
}

/// A VERTEX'S OWN BYTES — what it contributes to a key, and therefore its
/// coordinate within the form.
/// A VERTEX'S OWN COORDINATES, as the key writes them (A21). `pub(crate)`
/// since 54.7 B5a: a `Glued` vertex no longer carries the form it cites,
/// so its SITE — this record — is what identifies it, and `reindex`,
/// `vertex_match_key` and `coboundary_gauge` all ask for it by name
/// instead of each writing it again (law 12).
pub(crate) fn vertex_bytes(v: &Vertex) -> Vec<u8> {
    let mut b = Vec::new();
    put_vertex(&mut b, v);
    b
}

/// The form's vertices in canonical order: the Frame, then every other
/// vertex by its own coordinate bytes, read back from the trie whose order
/// those bytes are.
fn canonical_vertices(f: &Form) -> Vec<&Vertex> {
    let mut at: Keyed<u16> = Keyed::new();
    let mut out: Vec<&Vertex> = Vec::with_capacity(f.vertices.len());
    for (i, v) in f.vertices.iter().enumerate() {
        match v {
            Vertex::Frame => out.push(v),
            _ => {
                at.insert(vertex_bytes(v), i as u16);
            }
        }
    }
    for (_, i) in at.scoped() {
        if let Some(v) = f.vertices.get(*i as usize) {
            out.push(v);
        }
    }
    out
}

/// Where each vertex of `f` stands in the canonical order, indexed by its
/// place in the form's own vector.
fn canonical_places(f: &Form) -> Vec<u16> {
    // Sized to cover every index a FACE names as well as every vertex, so
    // the read at each face's ends is total. A face naming an index the
    // form does not hold keeps that index, which is the only thing true of
    // it; nothing is invented for it (rule 11).
    let mut n = f.vertices.len();
    for face in &f.faces {
        n = n.max(face.i as usize + 1).max(face.j as usize + 1);
    }
    let mut out: Vec<u16> = (0..n).map(|i| i as u16).collect();
    // EVERY VERTEX SHARING A COORDINATE GETS THAT COORDINATE'S PLACE
    // (§1.5, 54.5).
    //
    // This kept `coordinate → INDEX` in the trie, so two vertices with one
    // coordinate overwrote each other and only the LAST index was ever
    // given a place. The first kept its identity mapping `out[i] = i` from
    // the initialisation above — a stale index into a walk that no longer
    // has that many entries — and every face naming it pointed somewhere
    // else. Keying `coordinate → PLACE` and then mapping each vertex
    // through its own coordinate says the thing that is true: they are one
    // vertex, so they stand at one place, and a face at either end names
    // that place.
    let mut n = 0u16;
    // The frames first and in the form's own order. "A form opens at its
    // frame" (18) — the basepoint is not one of the coordinates being
    // ordered, so it is not in the trie.
    for (i, v) in f.vertices.iter().enumerate() {
        if matches!(v, Vertex::Frame) {
            if let Some(slot) = out.get_mut(i) {
                *slot = n;
            }
            n += 1;
        }
    }
    let mut distinct: Keyed<()> = Keyed::new();
    for v in &f.vertices {
        if !matches!(v, Vertex::Frame) {
            distinct.insert(vertex_bytes(v), ());
        }
    }
    let mut place: Keyed<u16> = Keyed::new();
    for (k, ()) in distinct.scoped() {
        place.insert(&k, n);
        n += 1;
    }
    for (i, v) in f.vertices.iter().enumerate() {
        if matches!(v, Vertex::Frame) {
            continue;
        }
        if let (Some(&p), Some(slot)) = (place.get(&vertex_bytes(v)), out.get_mut(i)) {
            *slot = p;
        }
    }
    out
}

/// THE BYTES A `Word` VERTEX CONTRIBUTES TO A KEY (54.2), written ONCE.
/// Two writers of one fact is law 12's defect, and an address that drifted
/// from the key containing it would be a vertex the form could not find.
fn put_word_vertex(
    buf: &mut Vec<u8>,
    derivation: &crate::transducer::alphabet::Derivation,
    acts: &Keyed<Acting, crate::network::nerve::coord::Section>,
    contact: Option<crate::transducer::alphabet::Contact>,
) {
    crate::transducer::address::put_key_bytes(buf, &derivation.rotor().key());

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
    crate::transducer::address::put_key_count(buf, acts.len());
    for (_, a) in acts.scoped() {
        // THE CHART, NOT THE CARRIER'S KEY (54.7 R1). `Acting::key` is
        // `chart ‖ carriage` and `take_acts` parses an `Invariant` out of
        // what is written here, so writing the map's key stops 25 forms
        // decoding. The address carries the chart and the carriage as its
        // own fields; the carrier's key is re-derived on insert, by the one
        // function that owns it.
        crate::transducer::address::put_key_bytes(buf, a.chart.key().as_bytes());
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
        crate::transducer::address::put_key_count(buf, g.len());
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
    acts: &Keyed<Acting, crate::network::nerve::coord::Section>,
    contact: Option<crate::transducer::alphabet::Contact>,
) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM, 1];
    put_word_vertex(&mut buf, derivation, acts, contact);
    crate::transducer::address::Address::new(buf)
}
