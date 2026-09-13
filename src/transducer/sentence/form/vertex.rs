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
    /// A WORD VERTEX ACTING ON ONE CHART — which is every construction in
    /// this tree today, and the shape D7a preserves exactly: one member, so
    /// `form_vertex_key` writes what it wrote before the fibre existed and
    /// every address in the tree is unchanged.
    ///
    /// D7b builds a vertex with several members by inserting them, and this
    /// stays the constructor for the one-chart case rather than being
    /// widened into a builder: a site that knows one chart says one chart.
    pub fn word_on(
        address: crate::transducer::address::Address,
        derivation: crate::transducer::alphabet::Derivation,
        chart: stalk::Invariant,
        carriage: Box<crate::transducer::element::Versor>,
        gauge: Vec<(u8, u8)>,
        contact: Option<alphabet::Contact>,
    ) -> Vertex {
        let member = super::Acting { chart, carriage, gauge };
        let mut acts = Keyed::new();
        acts.insert(member.key(), member);
        Vertex::Word { address, derivation, acts, contact }
    }

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
            Vertex::Word { .. } => match self.acting() {
                Some(a) => a.chart.clone(),
                // A word vertex acting on no chart speaks to no body, and
                // the root stratum is what that reads as — the same answer
                // the Frame gives, and not a chart invented for it.
                None => stalk::Invariant::default(),
            },
            Vertex::Rebased { class, .. } => class.clone(),
            Vertex::Glued { chart, .. } | Vertex::Demand { chart, .. } => chart.clone(),
            Vertex::Frame | Vertex::Unfolded { .. } => stalk::Invariant::default(),
        }
    }

    /// THE MEMBER A SINGLE-CHART WORD VERTEX ACTS BY, in canonical key
    /// order where it acts on several (rule 15's first case: the order is
    /// the trie's, which is the chart's own content).
    ///
    /// Every vertex this tree builds today acts on exactly one chart, so
    /// this is total and unambiguous on every form now in existence — which
    /// is what makes D7a a representation change and nothing more. D7b
    /// makes several members the normal condition, and a reader that wants
    /// a PARTICULAR chart asks `acts().get(chart.key())` rather than this:
    /// a representative is a representative, and the sites that need one
    /// are the ones writing a single-chart vertex back out.
    pub fn acting(&self) -> Option<&super::Acting> {
        match self {
            Vertex::Word { acts, .. } => acts.scoped().next().map(|(_, a)| a),
            _ => None,
        }
    }

    /// EVERY MEMBER, for a reader that wants a particular chart or all of
    /// them. The empty carrier for any other vertex kind: a Glued or a
    /// Demand acts on nothing by a carriage of its own.
    pub fn acts(&self) -> Option<&Keyed<super::Acting>> {
        match self {
            Vertex::Word { acts, .. } => Some(acts),
            _ => None,
        }
    }

    /// The carriage of the member above, where there is one.
    pub fn carriage_of(&self) -> Option<&crate::transducer::element::Versor> {
        self.acting().map(|a| a.carriage.as_ref())
    }

    /// The gauge of the member above; the empty map where the vertex acts
    /// on nothing, which is a hue map that moves no hue rather than an
    /// absence standing in for one.
    pub fn gauge_of(&self) -> &[(u8, u8)] {
        match self.acting() {
            Some(a) => &a.gauge,
            None => &[],
        }
    }

    /// REPLACE THE CARRIAGE OF THE MEMBER ABOVE, keeping its chart and its
    /// gauge. The carriage was a field and a caller holding `&mut Vertex`
    /// could write it; `acts` is an index and writing through it means
    /// reaching the entry by its key, which is the chart — so the move is
    /// named here once rather than spelled at each caller.
    ///
    /// It does not re-file: the key is the chart, the chart does not change,
    /// and the entry keeps its place. A caller that means to move the vertex
    /// to ANOTHER chart is not replacing a carriage — it is inserting a
    /// member, which is D7b's business.
    pub fn set_carriage(&mut self, carriage: crate::transducer::element::Versor) {
        let Vertex::Word { acts, .. } = self else {
            return;
        };
        let Some(k) = acts.scoped().next().map(|(k, _)| k) else {
            return;
        };
        if let Some(a) = acts.get_mut(&k) {
            *a.carriage = carriage;
        }
    }

    pub fn depth(&self) -> u16 {
        match self {
            Vertex::Glued { depth, .. } => *depth,
            _ => 0,
        }
    }
}
