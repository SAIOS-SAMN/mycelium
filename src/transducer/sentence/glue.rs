// 54.7 — THE GLUE: a closed form cited inside another, as a BOUNDARY on a
// vertex and an INCIDENCE on the cover.
//
//! A citation is two facts and they live in two places. The host chart
//! states its own open boundary — the rank of chart it mates with, that
//! chart's depth when cited, the placement and the transition versor φ —
//! and every coordinate of that is local, bounded and `O(1)` per citation,
//! so it rides the vertex and goes into `form_vertex_key`. WHICH form the
//! boundary was instantiated against is a fact about the COVER, not about
//! either chart, and it rides [`Form::cover`] as a `NerveEdge` of
//! `N(𝒰)`'s 1-skeleton.
//!
//! Split from `chart.rs` at the 1200-line reading bound (§1.1) when B5a
//! moved the target off the vertex; the seam is the one the ruling drew.

use super::form::*;
use super::chart::chart_of_form;
use crate::network::nerve::coord;
use crate::transducer::alphabet::Word;
use crate::transducer::element;

impl Form {
    /// Glue a closed form in as one vertex, and re-read the orbits and the
    /// index for it. The single-citation reader; a PASS of citations calls
    /// [`Form::glue_append`] and re-reads once at the end (Z2 item 1).
    pub fn glue(&mut self, s: &Form, s_addr: crate::transducer::address::Address, placement: Word, magnitude: [u64; 4]) -> Option<coord::VertexAt> {
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
    ) -> Option<coord::VertexAt> {
        let at = self.glue_append_under(s, s_addr, placement, magnitude, phi)?;
        self.orbits = Self::orbits_of(&self.faces, &self.vertices);
        self.reindex();
        Some(at)
    }

    /// THE CITATION APPENDED AND NOTHING RE-READ (Z2 item 1): the `Glued`
    /// vertex, its placement face, the glue index and the depth — every
    /// coordinate the citation itself writes — and neither `orbits_of` nor
    /// `reindex`.
    ///
    /// The glue pass of `extract_rule_with` cited one form 578 times on
    /// `1d0a4b61`, and each citation rebuilt the cycle basis over 2 850
    /// faces and reindexed the whole form. Measured on the instrument: 4.9 s
    /// of `face_at` inside `orbits_of`, on a 16.8 s pull.
    ///
    /// AN APPEND CANNOT INVALIDATE WHAT THE PASS READS. A `Glued` vertex
    /// carries no transport class, so `by_transport` — the index the pass
    /// asks for the vertices to retract — is untouched by it; the orbits the
    /// appended face closes are read once after the pass, with the
    /// retractions, which rebuilds them anyway.
    pub fn glue_append(&mut self, s: &Form, s_addr: crate::transducer::address::Address, placement: Word, magnitude: [u64; 4]) -> Option<coord::VertexAt> {
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
    ) -> Option<coord::VertexAt> {
        if !s.is_closed() {
            return None;
        }
        // 18: A FORM OPENS AT ITS FRAME, held at the Frame's own coordinate.
        let frame = s.vertices.at(&coord::VertexAt::Frame)?;
        if !frame.word_in(&s.vertices).is_some_and(|w| w.is_identity()) {
            return None;
        }
        // THE DEPTH REFUSAL IS GONE (54.2, as ruled 2026-09-20). It read
        //
        //     let host = self.depth_of();
        //     if host > 0 && d >= host { return None }
        //
        // and was taken for an acyclicity invariant. It was not one. `host`
        // is one more than the DEEPEST CITATION ALREADY APPENDED, so the
        // refusal asked a question about arrival order: the same set of
        // citations was admitted deep-first and refused shallow-first, and a
        // fresh host (`host == 0`) was exempt altogether, so two leaves could
        // always cite each other. It forbade not cycles but a second citation
        // deeper than the first — the 63d6e40 defect in its second home, and
        // `glue_key_ignores_arrival_order` is its reading.
        //
        // WHAT STANDS IN ITS PLACE. Charts of an open cover do not own each
        // other; `U_i` citing `U_j` and `U_j` citing `U_i` is one overlap
        // written from both sides, and it is the 1-skeleton of the nerve
        // whose 2-simplices carry `H²`. The guard against a runaway is not a
        // posetal height but topological closure — `φ_ij φ_jk φ_ki = 1` — and
        // a cycle that does not close carries its composite as its holonomy,
        // which is content and not an error (`curvature is content`).
        //
        // NOTHING RECURSES ON A CITATION. A `Glued` vertex holds the cited
        // form's ADDRESS, never the form: `depth_of` reads each vertex's own
        // stored `depth` field, and `from_address` rebuilds the vertex from
        // the address bytes without resolving them. A cycle is finite data
        // and every reader of it terminates.
        //
        // `depth` stays a COORDINATE OF THE CITATION — the cited form's depth
        // when it was cited — and is no longer a contract about the relation.
        let d = s.depth_of()?;
        // STD1 — the depth this form will stand at once the citation is
        // held, read before anything is written: one past the cited depth
        // with no `u16` for it refuses the citation, never a pinned depth.
        let after = self.depth_of()?.max(d.checked_add(1)?);
        let chart = chart_of_form(s)?;
        if chart.0.is_empty() {
            return None;
        }
        // THE BOUNDARY ON THE VERTEX, THE COVER ON THE FORM (54.7, B5a).
        // The vertex states what this chart has — a rank, a depth, a
        // placement, a φ — and the edge states which form that boundary was
        // instantiated against. `glued` keys by the boundary's own SITE, so
        // it is a coordinate and not a cited address (rule 15).
        let v = Vertex::Glued {
            chart: chart.clone(),
            magnitude,
            depth: d,
            placement: placement.clone(),
            carriage: Box::new(phi.clone()),
        };
        // THE SITE IS THE VERTEX'S COORDINATE (54.7 B5a, AT1): the host's
        // boundary record — never the address of what it cites, which left the
        // vertex with the target.
        let at = v.at()?;
        // ONE EDGE PER INCIDENCE (54.7 B5a). `U_i` overlaps `U_j` under φ
        // is one fact however many times it is stated, and `of_form` files
        // the nerve under site‖target, so a duplicate would be collapsed
        // there anyway — leaving the minted form saying two and the decoded
        // form saying one about the same cover.
        let edge = crate::transducer::element::NerveEdge {
            target: s_addr,
            chart,
            magnitude,
            depth: d,
            placement: placement.clone(),
            carriage: Box::new(phi),
        };
        if !self.cover.contains(&edge) {
            self.cover.push(edge);
        }
        // ONE VERTEX PER SITE, SEVERAL EDGES AT IT (54.7 B5a).
        //
        // Two citations of DIFFERENT forms under one φ from charts of one
        // rank now stand at one site — gauge universality: two isomorphic
        // local boundaries ARE one boundary, and what distinguishes them is
        // incidence in `N(𝒰)`, which is not a property of either chart.
        //
        // THE KEY ALREADY SAID SO. `canonical_vertices` files vertices
        // through a `Keyed` on `vertex_bytes` and `canonical_places` maps
        // every vertex sharing a coordinate to one place — "they are one
        // vertex, so they stand at one place". A form holding two vertices
        // at one site has a key that does not round-trip to it.
        //
        // And pairing them any other way would be pairing by POSITION: the
        // i-th vertex at this site with the i-th edge, which is an index and
        // not a coordinate (rule 15). One vertex, and the cover carries the
        // multiplicity.
        if self.vertices.holds(&at) {
            return Some(at);
        }
        self.vertices.put(&at, v);
        // THE PLACEMENT OVERLAP. A Glued vertex stands at the cited form's
        // chart and the Frame at its tag; a citation is at one frame — the
        // cited form's own, frame 0 — so its star is one overlap carrying the
        // placement (AT1: a face exists only at a member pair and its frame).
        let o = coord::Overlap { from: coord::ChartAt::Frame, to: coord::ChartAt::at(at.clone(), self.vertices.at(&at)?.charts().into_iter().next()?, 0) };
        let place = self.face_place(&o.from, &o.to, None)?;
        self.faces.put(&o, FormFace::placed(place, Producer::Place));
        self.depth = after;
        Some(at)
    }

    /// EVERY FORM THE BOUNDARY AT THIS SITE MATES WITH (54.7 B5a).
    ///
    /// A `Glued` vertex states a boundary; the cover states the incidences.
    /// There may be SEVERAL — two citations of different forms under one φ
    /// from charts of one rank stand at one site — so this answers with all
    /// of them and never with the first. A reader taking one would paint,
    /// comprehend or credit one citation and silently drop the others.
    pub fn edges_at<'a>(&'a self, site: &'a [u8]) -> impl Iterator<Item = &'a crate::transducer::element::NerveEdge> + 'a {
        self.cover.iter().filter(move |e| e.host().as_bytes() == site)
    }
}
