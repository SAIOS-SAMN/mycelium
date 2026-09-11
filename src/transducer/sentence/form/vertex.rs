// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 50.5 — WHAT A VERTEX SPEAKS: the word it contributes to a face,
//! the chart it stands on, and the depth it carries. A demand vertex
//! speaks no word, so no face reaches it and it enters no loop.
//!
//! Split from `carrier.rs` at the §1.1 600-line reading bound; the items
//! are moved, not rewritten (§1.6). The rebase chain in `word_in` is
//! WALKED rather than recursed into — L20's loop was that recursion, and
//! the walk is what made this file too long to read whole.

use super::*;

impl Vertex {

    /// The word a vertex speaks; a demand vertex speaks none (50.5), so no
    /// face reaches it and it enters no loop.
    pub fn word_in(&self, form: &Form) -> Option<Word> {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Some(Word::identity()),
            // A face reads the ACTION spelled, not the residue: a cited
            // vertex spells its rotor by A22 (L14, 50.2 step 2).
            Vertex::Word { derivation, .. } => derivation.spelling(),
            Vertex::Glued { placement, .. } => Some(placement.clone()),
            // THE CHAIN A REBASE MAKES, walked and never recursed into.
            //
            // This recursed on `by` with nothing bounding it, and a `by`
            // chain returning to a vertex already on the walk never ended —
            // L20's loop, measured on `0607ce86.json`. `Form::rebase` no
            // longer WRITES such a chain (it evaluates the stabilizer
            // orbit instead), but a tape arriving from a peer can carry
            // one, and a court that does not come back is a court an
            // adversary stops (chapter 48).
            //
            // THE WALK RECOGNISES ITS OWN BASEPOINT. A chain that returns
            // to a vertex it has already visited has closed: the loop is
            // absorbed, `rebase(v, 1) ≡ v`, and what the vertex speaks is
            // the identity — not a refusal and not a recursion, the
            // contraction of the cycle to its stationary point.
            Vertex::Rebased { .. } => {
                let mut at = self;
                let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                loop {
                    let Vertex::Rebased { by, of, .. } = at else {
                        return at.word_in(form);
                    };
                    crate::transducer::family::trace::trace_at("word_in/rebased by", *by as usize);
                    if seen.contains(&vertex_ix_key(*of)) {
                        return Some(Word::identity());
                    }
                    seen.insert(vertex_ix_key(*of), ());
                    at = form.vertices.get(*by as usize)?;
                }
            }
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
