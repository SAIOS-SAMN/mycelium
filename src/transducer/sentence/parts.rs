// Z12's instrument — WHAT A WORD'S TAPE IS MADE OF, by component, against
// the genome's size at the moment the word was absorbed.
//
//! THE CURVE MOVED DOWN AND DID NOT CHANGE SHAPE. Z9 took the duplicated
//! addresses out of the carriers and the fleet's heap fell nine per cent at
//! the same number of words held; the per-word cost still climbs linearly
//! in the genome's size, so the total is still the square of what the mind
//! knows and still ends in the OOM. Measured on one voice at three
//! readings: a word's own address went 14.7 → 30.7 → 47.0 KB as the genome
//! went 19 → 120 → 218 words, and its tape 102.3 → 136.0 → 177.1 KB.
//!
//! ONE COMPONENT OF THE TAPE SCALES WITH THE GENOME AND THIS NAMES WHICH.
//! The doctrine already fixes what each may be, so the reading lands in
//! exactly one of three places and each has its own fix:
//!
//!   - THE CITATIONS growing with the atlas is a fault in WHAT IS CITED.
//!     50.2 step 2: a citation is the cited form's address and the cited
//!     vertex's address, and a word cites what it was derived through —
//!     a count that should not track the vocabulary's size.
//!   - THE ADDRESSES growing is a fault in THE FORMS. An address is the M2
//!     composite over a form's vertices, and under 53.6 a form is tens of
//!     vertices, so an address that grows without the form growing is an
//!     address carrying something that is not the form.
//!   - THE TAPE CARRYING THE PRESENTATION is law 12's second carrier:
//!     A12's tape carries only the non-zero exponents, and anything beyond
//!     them is the derivation stored where it should be re-derived.
//!
//! NOTHING HERE DECIDES. It measures and prints; no branch in the engine
//! reads a part, and the components are taken by ENCODING each piece with
//! the tape's own writers and reading the length back, so the arithmetic is
//! the tape's and not a model of it. `rest` closes the sum against the
//! whole tape, so a component the reading forgot shows up as a gap rather
//! than as a silence.

use super::form::*;
use crate::transducer::element::Element;

/// One word's tape, decomposed. Every field is bytes of that tape except
/// `cite_n`, which is the citation COUNT — law 5: a count and a size are
/// two readings and are never summed into one.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TapeParts {
    /// The whole tape, as `encode_form_v2` writes it.
    pub total: usize,
    /// The Word vertices' own addresses, written whole (`put_addr`).
    pub addr_word: usize,
    /// The GLUED vertices' addresses — the cited form's address, written
    /// whole into the citing tape. 1.7: identity is not shortened.
    pub addr_glued: usize,
    /// Glued vertices on this form, and how many DISTINCT forms they cite.
    /// The gap between them is repetition WITHIN one tape, which 53.4's
    /// quantified citation removes in kind (K2) and which nothing here
    /// shortens.
    pub glued_n: usize,
    pub glued_distinct: usize,
    /// The rotor each derivation carries: its residue word's letter tape
    /// and the composed rotor where one travels (A1, 37.7).
    pub rotor: usize,
    /// The derivation's citations, as ADDRESS BYTES: per citation the
    /// cited form's address and the cited vertex's address (50.2 step 2).
    pub cites: usize,
    /// The citations' COUNT, summed over the form's word vertices.
    pub cite_n: usize,
    /// The members' carriages — one exact conformal versor each.
    pub carriage: usize,
    /// The members' gauges, two bytes a hue.
    pub gauge: usize,
    /// The charts the members act on (`put_inv`), on vertices and faces.
    pub charts: usize,
    /// The faces: their words and their per-pair transport.
    pub faces: usize,
    /// The sections a face was measured on (51.8, 21 step 2).
    pub sections: usize,
    /// The filed torsion classes.
    pub torsion: usize,
    /// The loops: their vertex and face lists, holonomies and carriages.
    pub loops: usize,
    /// The A5 remainder the form carries.
    pub remainder: usize,
    /// THE DEMAND VERTICES' BODIES (50.5): eighteen bytes a link, and the
    /// demand's kind. Restructuring the address accounting dropped these
    /// and `rest` went to 53% — which is what the field is for.
    pub demands: usize,
    /// What a Glued vertex carries besides its address: the magnitude it
    /// was glued at, its depth, and its placement word.
    pub glue_body: usize,
    /// The certified pivots the crystallization rests on (A20), which the
    /// tape carries WHOLE. This was the reading's first large gap: 35% of
    /// the tape sat in `rest` until the certificates were named.
    pub certs: usize,
    /// The chart axes opened over held loops (3.7, 15.5).
    pub axes: usize,
    /// The tape's fixed frame: the count ahead of each carrier and the
    /// per-item tags between them.
    pub frame: usize,
    /// THE WORD'S OWN KEY, in bytes — not part of the tape, and the thing
    /// 1.7 part 1 is about. Identity is not shortened, so this carries the
    /// cited forms' whole addresses and always will; the question part 1
    /// asks is whether those bytes are HELD once or once per citer.
    pub key_total: usize,
    /// How many of `key_total`'s bytes ARE cited addresses — what a
    /// shared-segment address would stop duplicating. `key_total` minus
    /// this is the form's own contribution to its name.
    pub key_cited: usize,
    /// Whatever the components above do not account for. SIGNED: a large
    /// positive `rest` is a component this reading forgot, and a NEGATIVE
    /// one is a component counted twice. Clamping it at zero would hide
    /// the second case, and the whole purpose of the field is to be the
    /// reading's own check on itself (rule 11).
    pub rest: i64,
}

/// Bytes a writer produces, measured by running it.
fn bytes_of(f: impl FnOnce(&mut Vec<u8>)) -> usize {
    let mut b = Vec::new();
    f(&mut b);
    b.len()
}

/// Decompose the tape of a word that carries a form. `None` for a word
/// with no form — there is no tape of this shape to take apart.
pub fn tape_parts(e: &Element) -> Option<TapeParts> {
    let f = crate::transducer::sentence::from_element_form(e)?;
    let mut p = TapeParts { total: e.description().len(), ..TapeParts::default() };
    // HOW MANY DISTINCT FORMS THE GLUED VERTICES CITE, read beside how
    // many there are. The gap is repetition within one tape and it is K2's
    // to remove by quantifying the citation, not the encoder's to shorten.
    let mut glued_names: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for v in &f.vertices {
        if let Vertex::Glued { address, .. } = v {
            glued_names.insert(address, ());
            p.glued_n += 1;
            p.addr_glued += 2 + address.len();
        }
    }
    p.glued_distinct = glued_names.len();
    for v in &f.vertices {
        match v {
            Vertex::Word { address, derivation, acts, .. } => {
                p.addr_word += 2 + address.len();
                p.rotor += bytes_of(|b| derivation.residue.to_bytes(b));
                // The composed rotor travels only where the residue does
                // not determine it (A1, 37.7); its absence is one byte,
                // which the `rest` bucket accounts for.
                p.rotor += bytes_of(|b| derivation.rotor().to_bytes(b));
                for (form, vertex, _) in &derivation.generators {
                    p.cite_n += 1;
                    p.cites += bytes_of(|b| {
                        crate::transducer::address::put_addr(b, form);
                    });
                    p.cites += bytes_of(|b| {
                        crate::transducer::address::put_addr(b, vertex);
                    });
                }
                for (_, a) in acts.scoped() {
                    p.carriage += bytes_of(|b| a.carriage.to_bytes(b));
                    p.gauge += a.gauge.len() * 2;
                    p.charts += 2 + a.chart.key().as_bytes().len();
                }
            }
            Vertex::Rebased { acts, .. } => {
                for (_, a) in acts.scoped() {
                    p.carriage += bytes_of(|b| a.carriage.to_bytes(b));
                    p.gauge += a.gauge.len() * 2;
                    p.charts += 2 + a.chart.key().as_bytes().len();
                }
            }
            Vertex::Glued { chart, placement, .. } => {
                p.charts += 2 + chart.key().as_bytes().len();
                // magnitude (four u64) and depth, then the placement word.
                p.glue_body += 32 + 2 + bytes_of(|b| placement.to_bytes(b));
            }
            Vertex::Demand { chart, kind, bodies } => {
                p.charts += 2 + chart.key().as_bytes().len();
                p.demands += bytes_of(|b| kind.to_bytes(b)) + 4 + bodies.len() * 18;
            }
            Vertex::Frame | Vertex::Unfolded { .. } => {}
        }
    }
    for face in &f.faces {
        p.faces += bytes_of(|b| face.word.to_bytes(b));
        for (pk, place) in face.at.scoped() {
            p.charts += 2 + pk.len();
            p.carriage += bytes_of(|b| place.carriage.to_bytes(b));
            p.gauge += place.gauge.len() * 2;
            if let Some(t) = &place.edge {
                p.faces += bytes_of(|b| t.word.to_bytes(b));
                p.faces += bytes_of(|b| t.carriage.to_bytes(b));
                p.gauge += t.gauge.len() * 2;
            }
        }
        if let Some(ps) = &face.pairs {
            p.sections += ps.len() * 2;
        }
    }
    for lp in &f.loops {
        p.loops += lp.vertices.len() * 2 + lp.faces.len() * 2 + 2;
        p.loops += bytes_of(|b| lp.holonomy.to_bytes(b));
        for (k, c) in lp.carriages.scoped() {
            p.loops += k.len();
            p.loops += bytes_of(|b| c.to_bytes(b));
        }
    }
    for (k, v) in f.sections.scoped() {
        p.sections += 2 + k.len() + 2 + v.len() * 2;
    }
    for (k, t) in f.torsion.scoped() {
        // `(σ, ℓ, chord)`: the cycle type as its hue pairs, the period
        // and the chord face.
        p.torsion += 2 + k.len() + 2 + t.sigma.len() * 2 + 4;
    }
    p.remainder = 4 + f.remainder.placed().len() * 18;
    // THE CERTIFICATES RIDE THE TAPE WHOLE (A20) and were the gap: 35% of
    // every tape sat unaccounted in `rest` until they were named here.
    for (_, c) in f.certificates.scoped() {
        p.certs += bytes_of(|b| c.to_bytes(b));
    }
    for (k, _) in f.axes.scoped() {
        p.axes += 2 + k.len() + 2;
    }
    // The tape's fixed frame: one count ahead of each carrier and the
    // per-item tags the encoder writes between them. Named so `rest` is a
    // reading of what is MISSING and not of what was never counted.
    p.frame = 2 + f.vertices.len()
        + 2 + f.faces.len() * 7
        + 2 + f.loops.len() * 8
        + 2 + 2 + 8 + 2 + 2 + 2;
    for v in &f.vertices {
        if let Vertex::Word { derivation, acts, contact, .. } = v {
            // generators count, acts count, contact tag (+ its body).
            p.frame += 2 + 2 + if contact.is_some() { 10 } else { 1 };
            let _ = (derivation, acts);
        }
    }
    // 1.7 part 1's prize, measured: the word's own key and how much of it
    // is the cited forms' addresses written inline by `form_vertex_key`'s
    // Glued arm. `put_bytes` writes a u32 length ahead of each.
    p.key_total = e.address().len();
    for v in &f.vertices {
        if let Vertex::Glued { address, .. } = v {
            p.key_cited += 4 + address.len();
        }
    }
    let named = p.addr_word
        + p.addr_glued
        + p.frame
        + p.rotor
        + p.cites
        + p.carriage
        + p.gauge
        + p.charts
        + p.faces
        + p.sections
        + p.torsion
        + p.loops
        + p.remainder
        + p.demands
        + p.glue_body
        + p.certs
        + p.axes;
    // A component this reading does not name shows as a positive gap; one
    // counted twice shows as a negative. Both are reported.
    p.rest = p.total as i64 - named as i64;
    Some(p)
}

impl TapeParts {
    /// The line the journal carries, read at the word's own coordinate.
    pub fn line(&self, at: &[u8], verts: usize) -> String {
        let id: String = at.iter().take(4).map(|b| format!("{b:02x}")).collect();
        format!(
            "z12 word {id}… verts {verts} \
             tape {} = addrW {} addrG {} glued {}/{} rotor {} cites {}/{} carriage {} gauge {} \
             charts {} faces {} sections {} torsion {} loops {} rem {} \
             certs {} axes {} demands {} glue {} frame {} rest {} | key {} cited {}",
            self.total,
            self.addr_word,
            self.addr_glued,
            self.glued_distinct,
            self.glued_n,
            self.rotor,
            self.cite_n,
            self.cites,
            self.carriage,
            self.gauge,
            self.charts,
            self.faces,
            self.sections,
            self.torsion,
            self.loops,
            self.remainder,
            self.certs,
            self.axes,
            self.demands,
            self.glue_body,
            self.frame,
            self.rest,
            self.key_total,
            self.key_cited,
        )
    }
}
