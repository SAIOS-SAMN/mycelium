// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 17.4, 3.3, 31 — THE CHART OF A FORM: its M4 coordinates, the A21 key the
//! atlas is written out as, the coboundary gauge between two forms, and the
//! `SentenceRule` a form is spoken as.
//!
//! AT1 (ii) — THE ADDRESS IS THE ATLAS WRITTEN OUT: the vertices in their own
//! coordinates' order, the overlaps in theirs, each by what it is and never
//! by a position in a list. What the key derives — the face words, the
//! sections, the orbits — is not written.

use super::address::from_element_form;
use super::form::*;
use crate::network::nerve::{coord, Coordinate, Keyed};
use crate::transducer::alphabet::Word;
use crate::transducer::element::{Element, Versor};

impl Form {
    /// DROP THE NAMED VERTICES, BY THEIR COORDINATES (AT1), and every overlap
    /// with an end at one of them; the cover loses the bonds whose ports went,
    /// a filed orbit or an opened axis whose chord went is dropped with it,
    /// and the orbits and fibres are re-read. The Frame is never dropped: a
    /// form opens at its frame (18).
    ///
    /// NOTHING IS RENUMBERED, because nothing was numbered: every carrier the
    /// form has names its vertices and overlaps by what they are, so the ones
    /// that remain are where they were.
    ///
    /// `false` where the retracted form's depth does not read (STD1): the form
    /// is no form this build can write, and the caller refuses it.
    #[must_use]
    pub fn retract_vertices(&mut self, drop: &[coord::VertexAt]) -> bool {
        for v in drop {
            if matches!(v, coord::VertexAt::Frame) {
                continue;
            }
            for o in self.overlaps_at(v) {
                self.faces.drop_at(&o);
                self.face_bodies.drop_at(&o);
            }
            self.vertices.drop_at(v);
        }
        // AND THE COVER LOSES THE BONDS WHOSE PORTS WENT (54.7 B5a): an edge
        // belongs exactly when its host is a Glued vertex still held.
        let held = |host: &crate::transducer::address::Address| {
            coord::Site::decode(host.as_bytes()).is_ok_and(|s| self.vertices.holds(&coord::VertexAt::Glued(s)))
        };
        let cover: Vec<crate::transducer::element::NerveEdge> =
            self.cover.iter().filter(|e| held(&e.host())).cloned().collect();
        self.cover = cover;
        // A FILING OR AN AXIS NAMES ITS CHORD; a chord that went takes it. A
        // fibre filing names no overlap and no retraction takes it.
        let torsion: Vec<coord::Holonomy> = self
            .torsion
            .entries()
            .filter(|(_, f)| f.chord.overlap().is_some_and(|o| !self.faces.holds(o)))
            .map(|(k, _)| k)
            .collect();
        for k in torsion {
            self.torsion.drop_at(&k);
        }
        let axes: Vec<coord::Holonomy> = self
            .axes
            .entries()
            .filter(|(_, c)| c.as_ref().is_some_and(|c| !self.faces.holds(c)))
            .map(|(k, _)| k)
            .collect();
        for k in axes {
            self.axes.drop_at(&k);
        }
        self.orbits = Self::orbits_of(&self.faces, &self.vertices);
        let Some(depth) = self.depth_of() else {
            return false;
        };
        self.depth = depth;
        self.reindex();
        true
    }

    // `regauge` AND `solve_coboundary` STOOD HERE AND RETIRED (AT1, ruled
    // (a)). The tree gauge was a choice — the spanning forest rooted at the
    // least vertex (rule 15) — held beside the coordinates as rewritten face
    // words, which is 78.41(2′)'s table; a face's word is derived from its
    // ends now and there is nowhere to write a gauge. Holonomies are compared
    // by CLASS, which a regauge never moved (78.44(2)).
}

#[derive(Debug, Clone)]
pub struct SentenceRule {
    pub form: Form,
    /// Family frame ratio ((h_out, h_in), (w_out, w_in)).
    pub ratio: ((usize, usize), (usize, usize)),
}

impl SentenceRule {
    /// Classes and members present on some frames only: the sections along
    /// `e₃` (51.8), READ OFF THE FORM'S MEMBERS (AT1) — never a copy held
    /// beside the form.
    pub fn sections(&self) -> Keyed<Keyed<()>, coord::SectionAt> {
        self.form.sections()
    }

    pub fn section_count(&self) -> usize {
        self.sections().scoped().map(|(_, v)| v.len()).sum()
    }

    /// EVERY RECOLOURING SECTION THIS TRANSPORT CLASS HAS BEEN SPOKEN WITH:
    /// the stratum of `hue_maps` the class opens, since the gauge is inside
    /// the full key and a read keyed by the whole of it can only answer a
    /// question whose answer the asker already holds.
    pub fn hue_sections_of(&self, rotor: &crate::transducer::alphabet::Rotor) -> Vec<Vec<(u8, u8)>> {
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

/// A form as a `SentenceRule` (chapter 31).
pub fn rule_of_form(mut form: Form) -> SentenceRule {
    form.reindex();
    let ratio = form.ratio;
    SentenceRule { form, ratio }
}

/// THE FORM'S ADDRESS: THE ATLAS WRITTEN OUT (AT1 (ii), 54.2, A21).
///
/// `TAG_FORM`, the base's `n` (71.2(1)), then the vertices — each by its own
/// record, in the order of their coordinates, which is the carrier's key
/// order — then the overlaps in theirs, each its key, its measured place and
/// the pivot's factors where a pivot wrote one; the frame ratio; the opened
/// axes' classes; and the remainder by its coordinates. Every count by its own
/// spelling. Nothing the key derives is written: a face's word, the sections
/// (the members' frames are in their records), the orbits.
pub fn form_vertex_key(f: &Form) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM];
    buf.push(f.n);
    crate::transducer::address::put_key_count(&mut buf, f.vertices.len());
    // THE CARRIER'S OWN ORDER IS THE CANONICAL ORDER (law 2, rule 15): the
    // trie walks its keys in their byte order and the key is the vertex's
    // coordinate. The Frame's key is its tag alone, `0`, so it leads.
    for (_, v) in f.vertices.scoped() {
        put_vertex(&mut buf, v);
    }
    crate::transducer::address::put_key_count(&mut buf, f.faces.len());
    for (k, face) in f.faces.scoped() {
        crate::transducer::address::put_key_bytes(&mut buf, &k);
        put_face(&mut buf, face);
    }
    let ((hn, hd), (wn, wd)) = f.ratio;
    for x in [hn, hd, wn, wd] {
        buf.extend_from_slice(&(x as u64).to_le_bytes());
    }
    // An opened axis is a coordinate of the form (I6): the class it was
    // opened on, written out, key order. Its chord is the orbit's, found by
    // the class on read, and not written.
    crate::transducer::address::put_key_count(&mut buf, f.axes.len());
    for (class, _) in f.axes.scoped() {
        crate::transducer::address::put_key_bytes(&mut buf, &class);
    }
    // 54.2, AT1 — THE REMAINDER BY ITS COORDINATES: what the form does not
    // close is identity, but its chain is evidence and rides the record.
    let at = match f.remainder_at {
        Some(at) => Some(at),
        None => ChainAt::of(&f.remainder.placed()),
    };
    put_chain_at(&mut buf, at.as_ref());
    // 78.143 — THE ELEMENTS, LAST AND ONLY WHERE THERE ARE ANY: a form that
    // states none keeps the address it had, byte for byte; one whose
    // elements an act names writes the clause's bytes.
    if !f.elements.is_empty() {
        super::form::rule_element::put_elements(&mut buf, &f.elements);
    }
    crate::transducer::address::Address::new(buf)
}

/// A chain's coordinates as the key writes them: a flag, then the measure per
/// lane, the pieces and the corner.
pub(crate) fn put_chain_at(buf: &mut Vec<u8>, at: Option<&ChainAt>) {
    let Some(at) = at else {
        buf.push(0);
        return;
    };
    buf.push(1);
    for lane in at.measure {
        buf.extend_from_slice(&lane.to_le_bytes());
    }
    buf.extend_from_slice(&at.pieces.to_le_bytes());
    buf.extend_from_slice(&at.corner.0.to_le_bytes());
    buf.extend_from_slice(&at.corner.1.to_le_bytes());
}

/// ONE OVERLAP'S MEASURED AND PRODUCED CONTENT (AT1): its producer, its
/// place — the carriage, the sorted gauge, 50.3's edge whole — the pivot's
/// factors behind a flag each, written only where one is not the identity,
/// and the interface behind a flag.
fn put_face(buf: &mut Vec<u8>, face: &FormFace) {
    buf.push(match face.producer {
        Producer::Place => 0,
        Producer::Motor => 1,
    });
    let place = &face.place;
    place.carriage.put_canonical(buf);
    put_gauge(buf, &place.gauge);
    match &place.edge {
        None => buf.push(0),
        Some(t) => {
            buf.push(1);
            t.word.put_canonical(buf);
            t.carriage.put_canonical(buf);
            put_gauge(buf, &t.gauge);
        }
    }
    for factor in [&face.left, &face.right] {
        match factor.is_identity() {
            true => buf.push(0),
            false => {
                buf.push(1);
                factor.put_canonical(buf);
            }
        }
    }
    match &face.interface {
        Some(i) => {
            buf.push(1);
            i.put(buf);
        }
        None => buf.push(0),
    }
    // 78.108 — THE REFRACTION, MEASURED CONTENT AS THE EDGE IS: a held
    // coset's candidate transports, each whole, or a static contact's tether.
    // The poses a mind read the refraction off stay with the mind (what
    // crosses is the motion, never the perception of the bodies).
    match &place.refraction {
        None => buf.push(0),
        Some(super::Refraction::Coset(candidates)) => {
            buf.push(1);
            crate::transducer::address::put_key_count(buf, candidates.len());
            for t in candidates {
                t.word.put_canonical(buf);
                t.carriage.put_canonical(buf);
                put_gauge(buf, &t.gauge);
            }
        }
        Some(super::Refraction::Static { tether, .. }) => {
            buf.push(2);
            tether.put_canonical(buf);
        }
    }
}

/// A hue map sorted, by its own count: content order, so two nodes holding
/// one map write one run.
fn put_gauge(buf: &mut Vec<u8>, gauge: &[(u8, u8)]) {
    let mut g = gauge.to_vec();
    g.sort_unstable();
    crate::transducer::address::put_key_count(buf, g.len());
    for (a, b) in g {
        buf.push(a);
        buf.push(b);
    }
}

/// THE BASE OF THE BUNDLE (A21 §2, A18): a form's chart is its RANK and
/// nothing else — torsion, free, off-plane, axes opened, four integers off
/// one keyed read behind the leading tag. `None` where the rank is
/// unmeasured.
pub(crate) fn chart_of_form(f: &Form) -> Option<crate::transducer::stalk::Invariant> {
    let r = f.rank()?;
    Some(crate::transducer::stalk::Invariant(
        [2u64, r.torsion, r.free, r.offplane, r.axes]
            .into_iter()
            .map(crate::transducer::stalk::ChartAxis::Count)
            .collect(),
    ))
}

/// Chart of a sentence element. None when the form does not decode or a
/// vertex is off-plane. Never `[2,1,0,0]`.
pub fn chart_of(e: &Element) -> Option<crate::transducer::stalk::Invariant> {
    chart_of_form(&from_element_form(e)?)
}

/// TWO FORMS WHOSE COCHAINS DIFFER BY A COBOUNDARY δγ: the gauge that takes
/// `a` onto `b`, one word per vertex, `γ_v = w_b,v ∘ w_a,v⁻¹` (AT1, ruled).
///
/// THE VERTICES ARE COMPARED DIRECTLY and the face check holds by
/// construction: a face's word is derived from its ends, so two forms with
/// one vertex set and one overlap set have faces that differ exactly by the
/// vertices' γ. What is checked is that the two forms hold the same vertices
/// and the same overlaps, by coordinate. `None` where they do not.
pub fn coboundary_gauge(a: &Form, b: &Form) -> Option<Keyed<Word, coord::VertexAt>> {
    let mut gamma: Keyed<Word, coord::VertexAt> = Keyed::new();
    for (at, va) in a.vertices.entries() {
        let vb = b.vertices.at(&at)?;
        gamma.put(&at, vb.word_in(&b.vertices)?.compose(&va.word_in(&a.vertices)?.inverse()));
    }
    if a.vertices.len() != b.vertices.len() || a.faces.len() != b.faces.len() {
        return None;
    }
    for (o, _) in a.faces.entries() {
        b.faces.at(&o)?;
    }
    Some(gamma)
}

/// ONE VERTEX'S RECORD IN THE KEY, written ONCE (law 12): what it is, from
/// which its coordinate is read back. `pub(crate)` for the readers of a
/// citation's site, which IS a Glued vertex's record ([`vertex_bytes`]).
pub(crate) fn put_vertex(buf: &mut Vec<u8>, v: &Vertex) {
    match v {
        Vertex::Frame => buf.push(0),
        // 1.5: the key is the COMPOSED product's canonical rotor, so two
        // minds whose derivations differ reach one address. The citations
        // are content the court replays (50.2 step 2) and are not keyed.
        Vertex::Word { derivation, acts, contact, .. } => {
            buf.push(1);
            put_word_vertex(buf, derivation, acts, *contact, true);
        }
        // 54.7 (B5) — A CITATION IS A BOUNDARY: the placement, the depth,
        // φ, and the chart the citation stands at (the cited form's rank,
        // which `from_address` cannot re-derive). Which form it mates with
        // is the cover's fact, on the record.
        Vertex::Glued { placement, depth, carriage, chart, .. } => {
            buf.push(2);
            placement.put_canonical(buf);
            buf.extend_from_slice(&depth.to_le_bytes());
            carriage.put_canonical(buf);
            crate::transducer::address::put_key_bytes(buf, chart.key().as_bytes());
        }
        // AT1 — the base BY ITS COORDINATE, the vertex's own former class,
        // and the members with the relative each carries.
        Vertex::Rebased { by, class, acts } => {
            buf.push(3);
            crate::transducer::address::put_key_bytes(buf, by.encode().as_bytes());
            crate::transducer::address::put_key_bytes(buf, class.encode().as_bytes());
            put_acts(buf, acts, None, true);
        }
        Vertex::Unfolded { measure } => {
            buf.push(4);
            crate::transducer::address::put_key_bytes(buf, unfolded_key(measure).encode().as_bytes());
        }
        // 50.5, AT1 ROAD (d) — a demand is its kind, its chart, and its
        // chain's coordinates; the bodies are evidence and ride the record.
        Vertex::Demand { kind, chart, measure, pieces, .. } => {
            buf.push(5);
            crate::transducer::address::put_key_bytes(buf, kind.key().as_bytes());
            crate::transducer::address::put_key_bytes(buf, chart.key().as_bytes());
            for lane in measure {
                buf.extend_from_slice(&lane.to_le_bytes());
            }
            buf.extend_from_slice(&pieces.to_le_bytes());
        }
    }
}

/// A VERTEX'S OWN RECORD, as the key writes it: a Glued vertex's SITE is
/// this (54.7 B5a).
pub(crate) fn vertex_bytes(v: &Vertex) -> Vec<u8> {
    let mut b = Vec::new();
    put_vertex(&mut b, v);
    b
}

/// THE MEMBERS AS THE KEY WRITES THEM, in the carrier's key order: each its
/// chart, its carriage by its canonical coordinates (A4), the vertex's
/// contact where it has one, its sorted gauge, and its frames (AT1, ruled —
/// the charts are the members at their frames, held here and nowhere else).
///
/// `frames` says whether each member's frames are written: in the FORM's key
/// they are (AT1, ruled — a member holds its frames, written once with it);
/// in a word vertex's OWN address they are not, because the frame is
/// family-local (r7b) and a word's address is what other forms cite — the
/// frames it stood on in one family are no coordinate of the word.
fn put_acts(
    buf: &mut Vec<u8>,
    acts: &Keyed<Acting, coord::Section>,
    contact: Option<Option<crate::transducer::alphabet::Contact>>,
    frames: bool,
) {
    crate::transducer::address::put_key_count(buf, acts.len());
    for (_, a) in acts.scoped() {
        crate::transducer::address::put_key_bytes(buf, a.chart.key().as_bytes());
        a.carriage.put_canonical(buf);
        if let Some(contact) = contact {
            match contact {
                None => buf.push(0),
                Some(c) => {
                    buf.push(1);
                    buf.extend_from_slice(&c.anchor.to_le_bytes());
                    buf.push(c.ring);
                }
            }
        }
        put_gauge(buf, &a.gauge);
        if frames {
            crate::transducer::address::put_key_count(buf, a.frames.len());
            for (k, ()) in a.frames.scoped() {
                buf.extend_from_slice(&k);
            }
        }
    }
}

/// THE BYTES A `Word` VERTEX CONTRIBUTES TO A KEY (54.2), written ONCE.
fn put_word_vertex(
    buf: &mut Vec<u8>,
    derivation: &crate::transducer::alphabet::Derivation,
    acts: &Keyed<Acting, coord::Section>,
    contact: Option<crate::transducer::alphabet::Contact>,
    frames: bool,
) {
    crate::transducer::address::put_key_bytes(buf, &derivation.rotor().key());
    put_acts(buf, acts, Some(contact), frames);
}

/// A `Word` VERTEX'S OWN ADDRESS, DERIVED FROM ITS COORDINATES (54.2): held
/// ONCE, on the vertex, and never copied into another key (AT1 Q1: the
/// vertex is keyed by its class).
pub fn word_vertex_address(
    derivation: &crate::transducer::alphabet::Derivation,
    acts: &Keyed<Acting, coord::Section>,
    contact: Option<crate::transducer::alphabet::Contact>,
) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM, 1];
    put_word_vertex(&mut buf, derivation, acts, contact, false);
    crate::transducer::address::Address::new(buf)
}
