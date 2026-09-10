// 50.2 step 2 — THE COURT READS A PATH: the generators a voice reached a
// word vertex through, composed and checked against the vertex itself.
//
//! The tape carries the vertex and the path carries the proof. Two voices
//! sending one address are told apart here, and each one's generators are
//! paid for the use (26). Nothing is trusted from the frame: the court
//! resolves every cited generator on its own ruled cover and re-derives
//! the composition (1.7).

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// Verify one spoken sentence's paths and pay the generators each one
    /// cites. Returns the paths that composed to their vertex.
    ///
    /// A path that does not compose is not a slash and not a refusal: the
    /// sentence stands or falls on its own replay (23.1), and a path is
    /// evidence for a use, not a claim about the form. It simply pays
    /// nothing, and the line says so.
    pub(crate) fn read_paths(
        &mut self,
        voice: [u8; 32],
        spoken: &Element,
        paths: &[crate::transducer::sentence::Path],
    ) -> usize {
        let Some(form) = crate::transducer::sentence::from_element_form(spoken) else {
            return 0;
        };
        let mut verified = 0usize;
        for p in paths {
            let Some(target) = word_of_vertex(&form, &p.vertex) else {
                continue;
            };
            // Every generator resolved on THIS court's ruled cover, never
            // taken from the frame (1.7): a voice that cites what no court
            // ruled cites nothing.
            let mut basis = Vec::with_capacity(p.generators.len());
            let mut tape = Vec::with_capacity(p.generators.len());
            let mut cited = Vec::with_capacity(p.generators.len());
            for (form_addr, vertex, e) in &p.generators {
                // A path cites the FORM and the VERTEX in it (1.8), because
                // the speaker knows both: the court does one keyed read on
                // its own ruled cover and one on that form's own index.
                // Given the vertex alone it had to walk every chart bucket
                // to find which form held it, which is the enumeration law
                // 3 forbids and which this reader's own comment denied.
                if !self.ruled_contains(form_addr.as_bytes()) {
                    continue;
                }
                let Some(held) = self.word_at(form_addr.as_bytes()) else {
                    continue;
                };
                let Some(f) = crate::transducer::sentence::from_element_form(held) else {
                    continue;
                };
                let Some(w) = crate::transducer::sentence::word_at_vertex(&f, vertex.as_bytes())
                else {
                    continue;
                };
                tape.push((basis.len(), *e));
                basis.push(w.rotor());
                cited.push((form_addr.clone(), held.magnitude()));
            }
            if cited.is_empty() {
                continue;
            }
            // The composition is checked by the crystal the voice used,
            // up to the coset representative of 50.2: the generators
            // composed, then the residue A22 spelled from the identity. A
            // PARTIAL path — the common case — reaches the vertex only
            // with its residue, and checked without it composed to nothing
            // and paid nobody.
            if !crate::transducer::factor::replays_with_residue(
                &tape,
                &basis,
                &p.residue.rotor(),
                &target.rotor(),
            ) {
                let (v0, v1) = head(&p.vertex);
                eprintln!(
                    "  court path refused {v0:02x}{v1:02x}… generators {} do not compose to the vertex",
                    cited.len()
                );
                continue;
            }
            verified += 1;
            let anchor = self.anchor_pub();
            for (addr, mag) in &cited {
                let Some(speaker) = self.discoverer_of(addr.as_bytes()) else {
                    continue;
                };
                let paid = self.treasury.settle_use(voice, speaker, *mag, anchor);
                let ((v0, v1), (g0, g1)) = (head(&p.vertex), head(addr));
                eprintln!(
                    "  court path paid {v0:02x}{v1:02x}… generator {g0:02x}{g1:02x}… {paid:?}"
                );
            }
        }
        verified
    }
}

/// The first two bytes of an address, for a trace line. Every arm is
/// written out: a short address is short, not padded with an invented
/// value (rule 11).
fn head(a: &crate::transducer::address::Address) -> (u8, u8) {
    match a.as_bytes() {
        [x, y, ..] => (*x, *y),
        [x] => (*x, *x),
        [] => (0, 0),
    }
}

/// The word a form's vertex carries, found by the address the path names.
/// A keyed read of the form's own index, never a walk (law 3).
fn word_of_vertex(
    form: &crate::transducer::sentence::Form,
    vertex: &crate::transducer::address::Address,
) -> Option<crate::transducer::alphabet::Word> {
    let i = *form.by_address.get(vertex.as_bytes())?;
    match form.vertices.get(i as usize)? {
        crate::transducer::sentence::Vertex::Word { word, .. } => Some(word.clone()),
        _ => None,
    }
}
