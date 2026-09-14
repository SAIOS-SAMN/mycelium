// 18.3 — THE CARRIER: the element's four strata and the fibers it opens.
//
//! `Action` is the four-lane action (S a versor, G a hue permutation, T an
//! extent in receipts, H a shell coordinate) plus the germinated axes;
//! `Element` is tape, action, chain, magnitude, fibre, anchor and connection.

use super::*;

/// Stratum tags on torsion and transitions (not matrix fibre indices).
pub const FIBER_SPATIAL: u16 = 0;
pub const FIBER_GAUGE: u16 = 1;
pub const FIBER_TEMPORAL: u16 = 2;
pub const FIBER_SHELL: u16 = 3;
/// First index of a germinated axis (a versor, never a matrix).
pub const FIBER_GERMINATED: u16 = 4;

/// Hue states on the gauge (ARC palette 0..9; 0 = absent).
pub const PALETTE: usize = 10;

/// ONE TORSION CYCLE of a punctured axis (18.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorsionCycle {
    /// The hue-map generator: the permutation one step around the cycle.
    pub sigma: Vec<(u8, u8)>,
    /// The PERIOD it closes at — the order of that generator, measured, and
    /// the extent the word owes on the T lane.
    pub period: u32,
    /// The fibre it lives on: `FIBER_SPATIAL`, or a germinated axis.
    pub fibre: u16,
}

/// ONE TRANSITION of the connection: how a sibling section was seen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    /// The assignment the sibling was observed under.
    pub asg: Vec<(u8, u8)>,
    /// The versor that carried it into this frame.
    pub gauge: Versor,
    /// The fibre it was observed on.
    pub fibre: u16,
}

/// The four strata of a word's action. S is one versor; G a hue permutation;
/// T an extent in receipts; H the shell coordinates. Germinated axes are
/// further versors. Nothing here is a matrix.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Action {
    /// S: `V = carriage · sensor_action_of(word)`. None is the identity
    /// motion.
    pub(crate) spatial: Option<Versor>,
    /// G: hue permutation as pairs. Empty is identity.
    pub(crate) gauge: Vec<(u8, u8)>,
    /// T: extent in receipts. Zero is none.
    pub(crate) temporal: u64,
    /// H: shell coordinate. Zero is none.
    pub(crate) shell: u64,
    /// Axes opened above the four strata, each a versor, tagged ≥ 4.
    pub(crate) germinated: Vec<(u16, Versor)>,
}

/// A germ as tape, action, chain, and magnitude.
#[derive(Debug, Clone)]
pub struct Element {
    /// Coordinate-exact ANASTOME description (empty for a pure residual chain).
    ///
    /// IT IS NOT CALLED `word`, and that is a ruling applied rather than a
    /// preference. `word` is the document's noun for a path in the Cayley
    /// graph whose meaning is a VERTEX — `alphabet::Word`, the object
    /// `Spelling` and every rotor reader carry. This field holds serialised
    /// bytes. Giving the encoding the vocabulary's most load-bearing noun is
    /// §2.1 at the worst place for it, and the accessor beside it has always
    /// been `description`, so one field carried two names and one of them was
    /// wrong.
    pub(crate) description: Vec<u8>,
    pub(crate) action: Action,
    /// Per-lane support, exact from the description.
    pub(crate) mag: [u64; 4],
    /// The CENTERED COBOUNDARY CHAIN — the residual's substance with geometry
    /// preserved up to translation: `(row, col, before, after)` over the changed
    /// cells, min-corner framed to the origin, sorted. Empty for a closed word.
    ///
    /// A5'S PROOF IS THE TYPE. As a bare `Vec` this field could hold a chain
    /// that says two things at one cell — a chain with no canonical form and
    /// so no address (law 2) — and every such element fell to one key. The
    /// field is the accepted chain itself, so that element cannot be built:
    /// `canonical_chain` is the only constructor, and the tape goes through
    /// it.
    pub(crate) sub: CanonicalChain,
    /// THE PUNCTURED AXIS — sections along the germinated fiber pierced through
    /// this thought: alternative per-cell transition ASSIGNMENTS over the SAME
    /// base positions. A recurring family (same geometry, different transitions)
    /// collapses into sections of this one bundle instead of sibling cards —
    /// growth as +axis (volume), not +card (area). The base `sub`'s own
    /// assignment is the implicit zeroth section. Vec-of-relations; empty until
    /// the axis is pierced. (The FREE part of the fiber; cycles condense into
    /// `torsion` below.)
    pub(crate) sections: Vec<Vec<(u8, u8)>>,
    /// THE TORSION OF THE AXIS — CE's law reaching the germinated fiber ("colors
    /// are torsion fields, not lookup tables"; the torsion_order is the period of
    /// the generating fold). When sections form the ORBIT of one hue-map
    /// generator σ (derived from the witnessed family cell-wise, exactly as peel
    /// derives offsets — never an enumerated plane), the fiber condenses from a
    /// free list to a TORSION CYCLE: `(σ relations from↦to, period ℓ)`. The
    /// orbit {base, σ·base, σ²·base, …} of length ℓ is spoken in O(1) storage —
    /// and a CLOSED cycle (σ^ℓ = id on its support) speaks orbit members never
    /// individually witnessed: capacity earned by exact structure. The fiber is
    /// thus ℤ^r ⊕ ⊕ᵢ ℤ/tᵢ — free sections ⊕ torsion cycles.
    pub(crate) torsion: Vec<TorsionCycle>,
    /// THE ANCHOR FIBER (Step 7) — γ = (π_anchor, Δ_substance): the recursive
    /// selection term restricting WHERE this word's substance applies on a
    /// scene. None = ⊤, the unconstrained carrier (pure topological actions,
    /// and every word born before selection entered the language).
    pub(crate) anchor: Option<crate::transducer::anchor::Sel>,
    /// THE CONNECTION — for every assignment spoken on this fibre (the base's
    /// own, a section, a torsion-orbit member), the versors V ∈ Spin(4,1)(ℚ)
    /// under which a sibling was observed and carried into this frame:
    /// `(assignment, V)`. The parallel-transport record of the bundle; the
    /// set of V reached for one assignment is its holonomy. Vec-of-relations;
    /// excluded from the address (a connection deepens the thought, it does
    /// not rename it).
    pub(crate) transitions: Vec<Transition>,
    /// The memoized address — a measurement of the tape and the action,
    /// taken once and cleared wherever the base mutates. Never serialized.
    pub(crate) address: std::sync::OnceLock<crate::transducer::address::Address>,
    /// THE GAUGE AXIS KEYED BY PROFILE: every assignment this word speaks
    /// (its own, its sections, its torsion orbits) reached by the key of
    /// its gauge charge — one read names the 0 / 1 / many assignments a
    /// body could be spoken with. Memoized; cleared wherever the fibre
    /// mutates. Never serialized.
    pub(crate) profiles: std::sync::OnceLock<Vec<(Vec<u8>, Vec<usize>)>>,
}

// `form: OnceLock<Option<Form>>` was here, with `Element::form()` decoding
// the tape behind it on the first read (L16). It is deleted, not moved: a
// decode cached inside a value that every organ clones is parsed once per
// COPY and not once per word, and the lock had to be taken by hand at
// `set_description` and `substance` — at every site that remembered to.
// The organs hold `element::Held` now, which carries the form as a field
// decoded where the word was taken in, and re-decodes with every mutation
// through `Held::update`. A caller with an element and no held word has
// `sentence::from_element_form`, which is the decode itself and says so.

impl Element {
    /// The identity element — empty tape, identity action, no chain.
    pub fn identity() -> Self {
        Element {
            description: Vec::new(),
            action: Action::default(),
            mag: [0; 4],
            sub: CanonicalChain::default(),
            sections: Vec::new(),
            torsion: Vec::new(),
            transitions: Vec::new(),
            address: std::sync::OnceLock::new(),
            profiles: std::sync::OnceLock::new(),
            anchor: None,
        }
    }

    /// The action's spatial versor: composed from the tape when letters
    /// exist, otherwise the stored S stratum.
    pub fn versor(&self) -> Option<Versor> {
        if let Some(letters) = self.letters() {
            return conformal_of(&letters.spelling.word, &letters.carriage);
        }
        self.action.spatial.clone()
    }

    /// Open a germinated axis holding `v`. Returns the axis index.
    ///
    /// The new index is the successor of the largest germinated axis this
    /// word already carries, which the set determines; a word carrying none
    /// opens at the first germinated seat, which is that set's successor
    /// too and not a value invented for it.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the largest index a word carries is a reading of its own axis set, content"
    )]
    pub fn open_axis(&mut self, v: Versor) -> u16 {
        let i = match self.action.germinated.iter().map(|(i, _)| *i).max() {
            Some(m) => m.saturating_add(1).max(FIBER_GERMINATED),
            None => FIBER_GERMINATED,
        };
        self.action.germinated.push((i, v));
        self.address.take();
        i
    }

    pub fn germinated(&self) -> &[(u16, Versor)] {
        &self.action.germinated
    }

    pub fn gauge(&self) -> &[(u8, u8)] {
        &self.action.gauge
    }

    /// Occupied strata: S when the versor is not identity, G when the
    /// permutation is not empty, germinated axes by index.
    pub fn axes(&self) -> Vec<u16> {
        let mut a = Vec::new();
        if self.versor().is_some_and(|v| !same_versor(&v, &identity_versor())) {
            a.push(FIBER_SPATIAL);
        }
        let g = if let Some(letters) = self.letters() {
            letters.sigma
        } else {
            self.action.gauge.clone()
        };
        if !g.is_empty() {
            a.push(FIBER_GAUGE);
        }
        if self.action.temporal != 0 {
            a.push(FIBER_TEMPORAL);
        }
        if self.action.shell != 0 {
            a.push(FIBER_SHELL);
        }
        for (i, _) in &self.action.germinated {
            a.push(*i);
        }
        a
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the largest axis index this word carries is a reading of the axis set, content; a word with no axes has none and refuses"
    )]
    pub fn max_axis(&self) -> Option<u16> {
        self.axes().into_iter().max()
    }

    /// Set the per-lane support this word carries (an opened axis carries
    /// the listing's measure and the charts it joined).
    pub fn set_magnitude(&mut self, mag: [u64; 4]) {
        self.mag = mag;
    }

    pub fn is_identity(&self) -> bool {
        // A residual with a canceling hue tally has identity action but a
        // real chain — the chain decides. A word with a description tape
        // is never the identity.
        self.axes().is_empty()
            && self.sub.is_empty()
            && !self.is_sentence()
            && !self.is_rewrite()
            && !self.is_translation()
    }

    /// The centered coboundary chain (empty for pure-algebra elements).
    /// The anchor fiber — None is ⊤ (the unconstrained carrier).
    pub fn anchor(&self) -> Option<&crate::transducer::anchor::Sel> {
        self.anchor.as_ref()
    }

    /// Bind the anchor fiber (germination writes it at birth; the court's
    /// refinement rebinds it).
    pub fn with_anchor(mut self, pi: crate::transducer::anchor::Sel) -> Self {
        self.anchor = Some(pi);
        self
    }

    pub fn sub(&self) -> &[(i64, i64, u8, u8)] {
        &self.sub
    }

    /// The ANASTOME description tape (connection or sentence).
    pub fn description(&self) -> &[u8] {
        &self.description
    }

    /// Replace the description tape and drop the memoized address.
    pub fn set_description(&mut self, description: Vec<u8>) {
        self.description = description;
        self.address.take();
    }

    /// The free sections along the punctured axis (cycles live in `torsion`).
    pub fn sections(&self) -> &[Vec<(u8, u8)>] {
        &self.sections
    }

    /// The torsion cycles of the axis: `(generator σ, period ℓ)` each.
    pub fn torsion(&self) -> &[TorsionCycle] {
        &self.torsion
    }

    /// The connection: every (assignment, g) under which a sibling was
    /// transported into this frame.
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    /// Record a dislocation versor on the fibre: (assignment, V, fibre)
    /// excluded from the address. Duplicate (asg, V, fibre) not stored twice.
    pub fn record_dislocation(&mut self, asg: Vec<(u8, u8)>, g: Versor, fibre: u16) {
        if !self
            .transitions
            .iter()
            .any(|t| t.asg == asg && same_versor(&t.gauge, &g) && t.fibre == fibre)
        {
            self.transitions.push(Transition { asg, gauge: g, fibre });
        }
    }

    /// File a torsion cycle `(σ, ℓ)` on fibre `fibre`. Duplicate generators
    /// on the same fibre are not stored twice.
    pub fn record_torsion(&mut self, sigma: Vec<(u8, u8)>, period: u32, fibre: u16) {
        if period == 0 {
            return;
        }
        if !self.torsion.iter().any(|t| t.sigma == sigma && t.fibre == fibre) {
            self.torsion.push(TorsionCycle { sigma, period, fibre });
            self.profiles.take();
        }
    }

    // `adopt_torsion` was here: it took a form's `Keyed<FiledLoop>` and wrote
    // each cycle into `self.torsion` as a `TorsionCycle`. It is deleted, and
    // not because it was unused — because calling it was the defect.
    //
    // A form's filed cycles are ALREADY on the element that carries that form:
    // inside the tape at `word`, written as `n_torsion` and its cycles. This
    // wrote them a second time into a struct field, which is law 12 on one
    // fact, and the copy it wrote held strictly less than the original — the
    // chord was dropped for `FIBER_SPATIAL`, a coordinate it invented. It also
    // moved `Element::magnitude`, which adds `sections.len() + torsion.len()`
    // onto lane 3 and is read by `lanes()` and the descent gate, so a caller
    // would have shifted a payment lane with no refusal firing anywhere.
    //
    // A sentence's cycles are read from its form: `sentence::form_torsion`, or
    // the form the organ already holds (`Held::form`, L16). There is no
    // conversion to make, so there is no converter.

    /// The fibre's span ABOVE ITS BASE — free sections plus every torsion
    /// orbit's members past the one it opens on. The volume metric proper, and
    /// a reader that wants it asks for it here: a caller subtracting the base
    /// off `fiber_span` to recover this is decomposing a composite by
    /// arithmetic, which says the composite was the wrong granularity for that
    /// reader, not that the reader owed a subtraction.
    ///
    /// It sums THIS element's own two fibre fields. A sentence carries its
    /// cycles on its form, leaves both of these empty, and is read through
    /// `sentence::form_torsion` (`Kernel::fibre_volume_total`).
    pub fn fibre_volume(&self) -> usize {
        self.sections.len()
            + self
                .torsion
                .iter()
                // A cycle of period ℓ spans ℓ − 1 beyond its base. A17 files
                // a cycle only at the period it CLOSES at and the tape
                // refuses a zero, so ℓ ≥ 1 here and the subtraction is total.
                .map(|t| t.period as usize - 1)
                .sum::<usize>()
    }

    /// The fiber's total spoken span — base + the volume above it (cycles
    /// count their full period).
    pub fn fiber_span(&self) -> usize {
        1 + self.fibre_volume()
    }

    /// THE TORSE MOVE — extract the torsion of the punctured axis. Candidate
    /// generators are DERIVED from the witnessed family (base → each section,
    /// cell-wise — the same way peel derives offsets; nothing enumerated); the
    /// first σ whose orbit spans enough of the family condenses those sections
    /// into ONE cycle `(σ, ℓ)`. A closure found within the walk (σ^t·base = base)
    /// sets ℓ = t — the FULL cycle, including unwitnessed members: capacity by
    /// exact structure. Returns the number of sections condensed (0 = no torsion
    /// found; the free part stays as it was).
    pub fn torse(&mut self) -> usize {
        const MAX_ORBIT: usize = 32;
        if self.sections.is_empty() {
            return 0;
        }
        let base = assignment(&self.sub);
        for k in 0..self.sections.len() {
            let Some(sigma) = derive_hue_map(&base, &self.sections[k]) else { continue };
            // Walk the orbit: collect members, note closure.
            let mut orbit: Vec<Vec<(u8, u8)>> = Vec::new();
            let mut cur = base.clone();
            let mut period = 0u32;
            for step in 1..=MAX_ORBIT {
                match apply_hue_map(&sigma, &cur) {
                    Some(next) => {
                        if next == base {
                            period = step as u32; // closed: σ^t = id on its support
                            break;
                        }
                        orbit.push(next.clone());
                        cur = next;
                    }
                    None => break,
                }
            }
            let spanned: Vec<usize> = (0..self.sections.len())
                .filter(|&i| orbit.contains(&self.sections[i]))
                .collect();
            // Only a CLOSED orbit is a cycle (σ^t = id on its support, the loop
            // witnessed at the mint): it condenses the sections it spans, even
            // one (a swap). An orbit that leaves its domain is not filed under
            // a period — those sections stay free; the open walk is the
            // remainder the word carries, never a winding it claims.
            if period > 0 && !spanned.is_empty() {
                for &i in spanned.iter().rev() {
                    self.sections.remove(i);
                }
                self.torsion.push(TorsionCycle { sigma, period, fibre: FIBER_SPATIAL });
                self.profiles.take();
                return spanned.len();
            }
        }
        0
    }

    /// PUNCTURE — pierce (or extend) the germinated axis with a new section: a
    /// fresh transition assignment over the same base positions. The Z through
    /// the paper: the thought gains VOLUME; the family member becomes a section,
    /// not a card. Returns false if the assignment is already spoken (or does
    /// not fit the base geometry).
    pub fn puncture(&mut self, asg: Vec<(u8, u8)>) -> bool {
        if asg.len() != self.sub.len()
            || asg == assignment(&self.sub)
            || self.sections.contains(&asg)
            || self.assignments().contains(&asg)
        {
            return false; // already spoken — by base, a section, or a torsion orbit
        }
        self.sections.push(asg);
        self.profiles.take();
        true
    }

    /// ATTACH — connect a congruent sibling to this bundle. Its chain is
    /// transported by g into this frame (g · supp(sibling) = supp(self)), the
    /// transported assignment becomes a section (or is recognised as one the
    /// fibre already speaks), and (assignment, g) joins the connection.
    /// Returns the sections the fibre gained (0 or 1); a g that does not
    /// carry the sibling onto this base changes nothing.
    pub fn attach(&mut self, sibling: &Element, g: &Versor) -> usize {
        let Some(asg) = pullback(&positions(&self.sub), &sibling.sub, g) else { return 0 };
        let gained = self.puncture(asg.clone()) as usize;
        if !self.transitions.iter().any(|t| t.asg == asg && same_versor(&t.gauge, g)) {
            self.transitions.push(Transition { asg, gauge: g.clone(), fibre: FIBER_SPATIAL });
        }
        gained
    }

    /// Merge another placement's fibre into this bundle (adoption of a word
    /// already held at this address): its sections puncture here and its
    /// connection joins. Returns the count of sections newly spoken.
    pub fn merge_fibre(&mut self, other: &Element) -> usize {
        let mut added = 0usize;
        for s in &other.sections {
            if self.puncture(s.clone()) {
                added += 1;
            }
        }
        for Transition { asg: a, gauge: g, fibre: f } in &other.transitions {
            if !self
                .transitions
                .iter()
                .any(|t| &t.asg == a && same_versor(&t.gauge, g) && &t.fibre == f)
            {
                self.transitions.push(Transition {
                    asg: a.clone(),
                    gauge: g.clone(),
                    fibre: *f,
                });
            }
        }
        added
    }

}

/// The perceptual lane projection of a centered coboundary chain — the
/// per-lane magnitude MEASURED from the chain's geometry, never funneled
/// onto one axis:
///   𝒮 spatial:  occupancy transformation — cells where support appears or
///               vanishes (mass moved, scaled, or reshaped).
///   𝒢 gauge:    chromatic content — hue transitions on cells where the
///               support stands on both sides (recolour, palette orbit).
///   𝒯 temporal: standing extent — TUniversal ticks a listing has stood,
///               or the family's scene-state span at mint. The chain has
///               none; 0 here. `Element::magnitude` and the kernel overlay
///               the live T coordinate.
///   𝒽 holonomic: boundary topology — the connected-component and
///               boundary-cochain deltas between the before-support and the
///               after-support (merges, splits, enclosures). Sections and
///               torsion cycles the word holds are added on this lane by
///               `Element::magnitude`.
/// A recolour word is gauge-lane work; a move is spatial; a hole closing is
/// shell. Without these projections the four-lane manifold degenerates to a
/// scalar on 𝒯 and every gradient downstream flatlines.
pub fn measure_lanes(chain: &[(i64, i64, u8, u8)]) -> [u64; 4] {
    let mut spatial = 0u64;
    let mut gauge = 0u64;
    let mut before_pos: Vec<(i64, i64)> = Vec::new();
    let mut after_pos: Vec<(i64, i64)> = Vec::new();
    for &(r, c, b, a) in chain {
        if b != 0 {
            before_pos.push((r, c));
        }
        if a != 0 {
            after_pos.push((r, c));
        }
        match (b != 0, a != 0) {
            (true, true) if b != a => gauge += 1,
            (true, false) | (false, true) => spatial += 1,
            _ => {}
        }
    }
    let sb = crate::transducer::beam::signature(&before_pos, &[]);
    let sa = crate::transducer::beam::signature(&after_pos, &[]);
    let shell = sb.components.abs_diff(sa.components) + sb.boundary.abs_diff(sa.boundary);
    // LANE 2 IS NOT THE CHAIN'S TO REPORT. 𝒯 is temporal extent — the
    // scene-state span in receipts (51.9) — and a chain is a set of cells
    // with no clock in it: nothing here can measure how long anything stood.
    // Reporting zero is therefore a reading of this carrier and not a
    // placeholder, and the three lanes beside it ARE the chain's own.
    //
    // The deferral this line has always stated — "measured at the mint" —
    // was not honoured by anything until R2. `sentence_measure` computed a
    // word's μ⃗ through this function alone, so every word the engine spoke
    // carried `[S, G, 0, H]`: 197 of 197 spoken magnitudes at 820678a. It
    // takes `t_extent` now, `Contemplation::span_of` is the one reader of
    // it, and `bundle_pressure` writes the same quantity to `sum[2]`.
    //
    // So a caller that needs 𝒯 asks the mint or the bundle. One that reads
    // a chain gets what a chain knows, which on this lane is nothing.
    [spatial, gauge, 0, shell]
}

impl Element {
    /// The four lanes this germ carries content on — 𝒮 spatial, 𝒢 gauge, 𝒯
    /// temporal, 𝒽 holonomic — exactly where the coordinate-exact magnitude
    /// is nonzero (never the lossy tally).
    pub fn lanes(&self) -> [bool; 4] {
        let m = self.magnitude();
        std::array::from_fn(|l| m[l] > 0)
    }

    /// The germ's polytonic MAGNITUDE — per-lane support size (‖Δ‖'s |supp|
    /// component), never summed, computed EXACTLY from the description at
    /// germination. A settled demand's residual magnitude is 0⃗; a real germ's is
    /// nonzero — the descent the POW++ gate reads.
    /// Spatial, gauge, and shell from the chain (and what a connection
    /// carries). Lane 2 is temporal extent at mint (scene-state span).
    /// Lane 3 gains the holonomic volume — sections and torsion cycles.
    pub fn magnitude(&self) -> [u64; 4] {
        let mut m = self.mag;
        m[3] = m[3].saturating_add((self.sections.len() + self.torsion.len()) as u64);
        m
    }

    /// The lanes the chain itself carries, before the sections and torsion
    /// cycles the word gained after it was spoken are counted. This is what
    /// re-derives from the chain; a word deepened by `attach` or
    /// `merge_fibre` keeps this fixed while `magnitude` grows on lane 3.
    pub fn base_magnitude(&self) -> [u64; 4] {
        self.mag
    }

    /// The base magnitude on the lanes a chain names: 𝒮 spatial, 𝒢 gauge,
    /// 𝒽 shell. Lane 𝒯 is temporal extent, measured on the standing
    /// carrier and never on the chain (`measure_lanes` writes it zero), so
    /// it is quotiented here by the same law. This is the vector the court
    /// compares with `from_residual(chain).base_magnitude()`, as one
    /// equality, never lane by lane.
    pub fn chain_magnitude(&self) -> [u64; 4] {
        let [spatial, gauge, _, shell] = self.mag;
        [spatial, gauge, 0, shell]
    }

    /// THE TRANSPOSITION QUOTIENT — factor out the translation (the ℤ² orbit) so
    /// the element becomes its position-independent SUBSTANCE
    /// (dilation·rotation·gauge·residual). Two transposition-equivalent demands
    /// (same substance, different position) share ONE substance — the object of
    /// confluent reduction, not the coordinate-exact word. (Gauge S₁₀ canonical
    /// relabel is a further quotient, still to land.)
    pub fn substance(&self) -> Element {
        let mut out = self.clone();
        out.description = Vec::new();
        out.address.take();
        if let Some(v) = out.versor() {
            if let Some(t) = translator_of(&v) {
                if !same_versor(&t, &identity_versor()) {
                    if let Some(inv) = t.versor_inverse() {
                        let d = v.geo(&inv);
                        out.action.spatial = if same_versor(&d, &identity_versor()) {
                            None
                        } else {
                            Some(d)
                        };
                    }
                }
            }
        }
        out.mag = if out.sub.is_empty() {
            out.reduction_magnitude()
        } else {
            measure_lanes(&out.sub)
        };
        out
    }

    /// Descent order: a non-identity versor occupies 𝒮, a nonempty
    /// permutation occupies 𝒢.
    pub fn reduction_magnitude(&self) -> [u64; 4] {
        let spatial = u64::from(self.versor().is_some_and(|v| !same_versor(&v, &identity_versor())));
        let gauge = u64::from(!self.action.gauge.is_empty());
        [spatial, gauge, 0, 0]
    }

}
