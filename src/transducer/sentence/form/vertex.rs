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
        derivation: crate::transducer::alphabet::Derivation,
        chart: stalk::Invariant,
        carriage: Box<crate::transducer::element::Versor>,
        gauge: Vec<(u8, u8)>,
        contact: Option<alphabet::Contact>,
    ) -> Vertex {
        let member = super::Acting { chart, carriage, gauge };
        let mut acts = Keyed::new();
        acts.insert(member.key(), member);
        // 54.2 — the vertex's address is its own coordinates, derived.
        let address = crate::transducer::sentence::word_vertex_address(&derivation, &acts, contact);
        Vertex::Word { address, derivation, acts, contact }
    }

    /// A REBASED VERTEX ON ONE CHART — `of` read in `by`'s frame, carrying one
    /// body. The counterpart of `word_on`, for the same reason: a site that
    /// knows one chart says one chart, and `Form::rebase` builds the several-
    /// chart case by inserting a member per chart `of` acted on.
    pub fn rebased_on(
        by: u16,
        of: u16,
        chart: stalk::Invariant,
        carriage: Box<crate::transducer::element::Versor>,
        gauge: Vec<(u8, u8)>,
    ) -> Vertex {
        let member = super::Acting { chart, carriage, gauge };
        let mut acts = Keyed::new();
        acts.insert(member.key(), member);
        Vertex::Rebased { by, of, acts }
    }

    /// The word a vertex speaks; a demand vertex speaks none (50.5), so no
    /// face reaches it and it enters no loop.
    pub fn word_in(&self, form: &Form) -> Option<Word> {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Some(Word::identity()),
            // A FACE READS THE ACTION, WHICH IS THE COMPOSED ROTOR (§1.5,
            // 54.5 as ruled 2026-09-20). The line above already said "the
            // ACTION spelled, not the residue" and the code did the
            // opposite for a factored derivation: `spelling()` answers
            // `word_local()` where there is one, and for a vertex that
            // cites generators that is the RESIDUE A22 peeled after
            // factoring them out — not what the vertex does.
            //
            // §1.5: "two vertices with different derivations and one rotor
            // are one vertex, one address", and `form_vertex_key` writes
            // `derivation.rotor()`, the composed product, for exactly that
            // reason. Reading the residue here made the face word depend on
            // the PRESENTATION: two vertices at one address gave two face
            // words, so `face_word` was not a function of the key and a
            // form decoded from its address refused `REFUSE_FACE`.
            //
            // The composition is not taken on trust: `derivations_compose`
            // re-derives the product from the cited generators on the
            // court's own cover and refuses a vertex whose citations do not
            // reach the rotor it carries (50.2 step 2). The action is
            // verified; this reads it.
            Vertex::Word { derivation, .. } => {
                Some(Word::formed(derivation.rotor()))
            }
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
                    crate::transducer::family::trace::trace("word_in/rebased by", crate::transducer::family::trace::Wrote::Key(&crate::transducer::sentence::vertex_ix_key(*by)), None);
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

    /// EVERY CHART THIS VERTEX STANDS AT.
    ///
    /// It was `class() -> Invariant`, one chart, and its one caller builds a
    /// keyed set of the charts around a loop — so a vertex acting on two
    /// charts contributed one of them and the loop's chart set was short by
    /// the rest. A word vertex acting on no chart stands at the root stratum,
    /// which is the Frame's answer and not a chart invented for it.
    pub fn charts(&self) -> Vec<stalk::Invariant> {
        match self {
            // A REBASED VERTEX STANDS WHERE `of` STOOD, on every chart of it.
            Vertex::Word { acts, .. } | Vertex::Rebased { acts, .. } if !acts.is_empty() => {
                acts.scoped().map(|(_, a)| a.chart.clone()).collect()
            }
            Vertex::Glued { chart, .. } | Vertex::Demand { chart, .. } => vec![chart.clone()],
            Vertex::Word { .. }
            | Vertex::Rebased { .. }
            | Vertex::Frame
            | Vertex::Unfolded { .. } => {
                vec![stalk::Invariant::default()]
            }
        }
    }

    /// EVERY MEMBER, for a reader that wants a particular chart or all of
    /// them. The empty carrier for any other vertex kind: a Glued or a
    /// Demand acts on nothing by a carriage of its own.
    ///
    /// THERE IS NO REPRESENTATIVE READER, and there was: `acting()` returned
    /// the member in least-chart-key order, and `carriage_of`/`gauge_of`
    /// returned that member's. While such a reader exists a caller that wants
    /// ONE member takes it — the scalar collapse arrives back through the
    /// accessor, silently, at every site — and no audit finds those sites
    /// reliably. Deleted, the compiler names all of them, exactly as
    /// `Keyed::fiber` taking a non-empty `Prefix` named the walk sites in X3.
    /// The type does the enumeration.
    pub fn acts(&self) -> Option<&Keyed<super::Acting>> {
        match self {
            // A REBASED VERTEX HAS THE SAME FIBRE SHAPE, with the relative in
            // place of the placement at each chart (D10). A reader that wants
            // "what does this vertex act on and by" gets the same answer for
            // both kinds; one that means the PLACEMENT reads
            // `vertex_carriage_at`, which composes the base's in for a Rebased.
            Vertex::Word { acts, .. } | Vertex::Rebased { acts, .. } => Some(acts),
            _ => None,
        }
    }

    /// GRADE 0 — THE VERTICAL TRANSITIONS IN THE FIBRE OVER THIS VERTEX,
    /// AS COORDINATES (54.5, as ruled 2026-09-20).
    ///
    /// IT IS NOT A BOOLEAN AND THAT IS THE WHOLE POINT (rule 4). Written as
    /// `body_coheres(&self) -> bool` — which is how it was written first —
    /// it is a one-bit scalar deciding a branch, and it DISCARDS the exact
    /// quantity it just computed. A gauge twist between two charts of one
    /// body is not an error with an infinite threshold; it is internal
    /// gauge curvature, and under T21(iii) a coordinate is earned by
    /// retiring an obstruction, never by refusing one. A boolean here is
    /// gradient descent with a step of one: it reads structural difference
    /// as a penalty.
    ///
    /// THE OBJECT. `π: M → V` gives the fibre `π⁻¹(v) = {(v,c)}`, and the
    /// vertical automorphism between two of its points is
    ///
    ///     g_{c₁c₂} = C_{v,c₂} · C_{v,c₁}⁻¹ = D(ρ) · T(t) · R_gauge
    ///
    /// exact in `Cl(5,1)(ℚ)`: a dilation, a centroid offset, and the
    /// relative rotation of the two frame bases. `R_gauge ≡ ±1` is a body
    /// merely displaced between charts. `R_gauge` of finite order about a
    /// fixed point is an intrinsic cyclic symmetry — `ℤ/ℓℤ` on the `𝒢`
    /// lane, a coordinate the pivot may open. A slip in `ker(I − R)` is a
    /// perspective parallax no choice of origin absorbs, which is
    /// `Demand::Dislocation`'s object.
    ///
    /// A SPANNING SET, NOT EVERY PAIR. The transitions telescope —
    /// `g_{c₂c₃} = g_{c₁c₃} · g_{c₁c₂}⁻¹` — so the `n−1` against the LEAST
    /// chart generate the rest and the others are composites law 12 does
    /// not store. The least chart is the trie's first, which is the chart
    /// keys sorted and so is content (rule 15).
    ///
    /// A vertex acting on nothing or on one chart has no vertical
    /// transition and yields none; that is an absence, never a flat
    /// reading (rule 11).
    pub fn fiber_holonomy(&self) -> Vec<super::FiberTransition> {
        let Some(acts) = self.acts() else {
            return Vec::new();
        };
        let members: Vec<&super::Acting> = acts.scoped().map(|(_, a)| a).collect();
        let [first, rest @ ..] = members.as_slice() else {
            return Vec::new();
        };
        let Some(inv) = first.carriage.versor_inverse() else {
            return Vec::new();
        };
        rest.iter()
            .map(|m| super::FiberTransition {
                from: first.chart.clone(),
                to: m.chart.clone(),
                carriage: m.carriage.geo(&inv),
                gauge_from: first.gauge.clone(),
                gauge_to: m.gauge.clone(),
            })
            .collect()
    }

    /// GRADE 0 — IS THE FIBRE OVER THIS VERTEX PURE CARRIAGE? (54.5, as
    /// ruled 2026-09-20.)
    ///
    /// THE PREDICATE IS A READING OF [`Self::fiber_holonomy`] AND NEVER A
    /// CARRIER OF ITS OWN (law 12). It exists for the one caller that
    /// genuinely asks a yes-or-no — "is this vertex a single 0-cell" — and
    /// every caller that wants to ACT on the answer takes the transitions
    /// and their classes instead, because that is where the coordinate is.
    ///
    /// Coherence is GRADED and the engine had it at one grade. `coheres`,
    /// `edge_coheres` and `is_cauterized` all read LOOPS — 1-cycles — and
    /// nothing read a 0-cell at all, so a vertex could assert any set of
    /// members and no reader could disagree.
    ///
    /// D7/18.2 makes a `Word` vertex ONE derivation and MANY `Acting`: one
    /// action, several placements, "two bodies of different shapes carried
    /// by one transport correspond". That is a claim, and this is the test
    /// of it. Between two members the vertical transition is
    ///
    ///     τ_{c₁c₂} = C_{v,c₂} · C_{v,c₁}⁻¹
    ///
    /// a 1-cell of the member complex `M` with BOTH ENDPOINTS OVER ONE
    /// VERTEX. For the vertex to be one 0-cell, τ must be pure carriage — a
    /// placement and a dilation, with no rotor. A τ that TURNS means the two
    /// members rotate relative to one another in their own charts, and
    /// calling them one vertex asserts an identity the geometry denies
    /// (§1.5: identity is never by fiat).
    ///
    /// ONE REPRESENTATIVE SUFFICES, and that is a theorem rather than a
    /// shortcut. Translations and dilations form a subgroup, so if τ₁₂ and
    /// τ₁₃ are both rotor-free then τ₂₃ = τ₁₃ · τ₁₂⁻¹ is too. The walk is
    /// `O(n)` against the LEAST chart the vertex stands at — the trie's
    /// first, which is the chart keys sorted and so is content, the same
    /// choice `derivations_compose` makes for the chart it refuses at
    /// (rule 15).
    ///
    /// A vertex acting on nothing, or on one chart, has no vertical
    /// transition and coheres vacuously. A τ whose base action cannot be
    /// READ is refused rather than assumed flat (rule 11).
    pub fn body_coheres(&self) -> bool {
        self.fiber_holonomy().iter().all(|t| t.is_vertical())
    }

    /// THE MEMBER AT A NAMED CHART: a keyed read, which is the only honest
    /// way to ask a fibre for one of its members. `None` is "this vertex does
    /// not act there", which is what a rule says about a body on a chart it
    /// has never seen.
    pub fn acting_on(&self, chart: &stalk::Invariant) -> Option<&super::Acting> {
        self.acts()?.get(chart.key().as_bytes())
    }

    /// THE CHART OF THIS VERTEX WHOSE KEY IS `key`. A keyed read back: a face
    /// records its member pairs as chart KEYS, and a reader re-deriving a
    /// placement needs the `Invariant` those keys name, which the vertex holds.
    /// `None` is "this vertex does not stand there".
    pub fn chart_keyed(&self, key: &[u8]) -> Option<stalk::Invariant> {
        self.charts().into_iter().find(|c| c.key().as_bytes() == key)
    }

    /// THE CHART A FACE READS THIS VERTEX AT, and the only representative
    /// reading left in the tree. `FormFace` holds one `Transport`, so a face
    /// between multi-chart vertices cannot be described without a transport
    /// fibre on the 2-skeleton — a row of its own, on the tape and in the
    /// address. See `transport::vertex_carriage` for the whole reason.
    ///
    /// It is the least chart key: CONTENT, so every node reads the same one
    /// (law 2), never an insertion order. `None` where the vertex stands at no
    /// chart of its own.
    pub fn face_chart(&self) -> Option<stalk::Invariant> {
        match self {
            Vertex::Word { acts, .. } | Vertex::Rebased { acts, .. } => {
                acts.scoped().next().map(|(_, a)| a.chart.clone())
            }
            Vertex::Glued { chart, .. } | Vertex::Demand { chart, .. } => Some(chart.clone()),
            Vertex::Frame | Vertex::Unfolded { .. } => Some(stalk::Invariant::default()),
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
