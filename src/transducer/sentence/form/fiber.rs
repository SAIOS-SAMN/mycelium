// 54.5, grade 0 — THE VERTICAL TRANSITION: one 1-cell of the member
// complex with both endpoints over a single vertex.
//
//! Split from `carrier.rs` at the §1.1 1200-line reading bound; the items
//! are moved, not rewritten (§1.6).
//!
//! `π: M → V` sends a member `(v, c)` to its vertex, and a `Vertex::Word`
//! is ONE derivation with MANY `Acting` — one action, several placements.
//! What relates two of those placements is this: an automorphism of the
//! fibre, carrying the body as one chart sees it onto the body as another
//! does. It is the object grade 0 measures, and the `𝒢` lane is what it
//! feeds.

use super::*;

/// GRADE 0 — ONE VERTICAL TRANSITION IN THE FIBRE OVER A VERTEX (54.5).
///
/// `π: M → V` sends a member `(v, c)` to its vertex. This is a 1-cell of
/// `M` with BOTH ENDPOINTS OVER ONE VERTEX — the automorphism carrying the
/// body as chart `from` sees it onto the body as chart `to` sees it:
///
/// ```text
///     g = C_{v,to} · C_{v,from}⁻¹ = D(ρ) · T(t) · R_gauge
/// ```
///
/// It carries the whole conformal element and NOT a decision about it.
/// Every reading — the gauge rotor, the Cartan split, the class — is taken
/// from `carriage` here (law 12), so nothing downstream can be handed a
/// verdict in place of the geometry that produced it.
/// THE SPATIAL HALF CANCELS AND THE GAUGE HALF DOES NOT (54.5, measured
/// 2026-09-20). This is why the transition carries both and why `𝒢` is the
/// lane it feeds.
///
/// `versor_coset` builds every member as `C = T(c_b)·D(ρ)·R·T(−c_a)`, and
/// `extract_rule`'s `reads` is keyed by TRANSPORT CLASS — one `Vertex::Word`
/// per class — so two members of one vertex carry the SAME `R` and the same
/// `ρ`. Then, using `R T(v) R⁻¹ = T(Rv)` and `D(ρ) T(v) D(ρ)⁻¹ = T(ρv)`:
///
/// ```text
///     τ = C₂ · C₁⁻¹
///       = T(c_b2)·D·R·T(c_a1 − c_a2)·R⁻¹·D⁻¹·T(−c_b1)
///       = T(c_b2 − c_b1 + ρ·R(c_a1 − c_a2))
/// ```
///
/// The rotor cancels IDENTICALLY. Verticality of the spatial part is a
/// theorem about the construction, not a property to be measured, and the
/// corpus reading of `0 twisted of 20 fibred` was forced before it ran.
///
/// So the spatial half is kept as a GUARD — a non-vertical carriage means
/// the transport-class keying itself broke, which is worth refusing — and
/// the `𝒢` content lives in the other half. `Acting.gauge` is the member's
/// own hue permutation, and nothing constrains two members of one vertex to
/// share it: a body carried by one transport that two charts RECOLOUR
/// differently is one action under two frames with an internal symmetry
/// between them. That relative permutation is this fibre's gauge holonomy,
/// and its cycle type is the `ℤ/ℓℤ` the pivot may open on `𝒢`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiberTransition {
    /// The chart the transition leaves: the fibre's least, its own
    /// coordinate order (rule 15).
    pub from: stalk::Invariant,
    /// The chart it arrives at.
    pub to: stalk::Invariant,
    /// The vertical automorphism whole, exact in `Cl(5,1)(ℚ)`. Provably a
    /// pure translation for a vertex whose members share a transport; a
    /// guard, not a measurement.
    pub carriage: Versor,
    /// `σ_to` — the arriving chart's own hue map.
    pub gauge_to: Vec<(u8, u8)>,
    /// `σ_from` — the leaving chart's. The relative permutation is read
    /// from the two (law 12) rather than stored beside them.
    pub gauge_from: Vec<(u8, u8)>,
}

impl FiberTransition {
    /// `R_gauge` — the relative rotation of the two frame bases, over all
    /// six planes of `Cl(4,0)`. `None` where the transition's base action
    /// cannot be read at all, which is a refusal and never a flat rotor
    /// (rule 11).
    pub fn rotor(&self) -> Option<crate::transducer::alphabet::Rotor> {
        crate::transducer::element::base_rotor_of(&self.carriage)
    }

    /// THE CARTAN SPLIT of the vertical transition: where the gauge bends,
    /// and the parallax no choice of origin absorbs. The twist is the
    /// fibre's own Burgers vector and is `Demand::Dislocation`'s quantity.
    pub fn screw(&self) -> Option<crate::transducer::element::Screw> {
        crate::transducer::element::screw_of(&self.rotor()?, &self.carriage)
    }

    /// `σ = σ_to ∘ σ_from⁻¹` — THE FIBRE'S GAUGE HOLONOMY, the recolouring
    /// one chart applies that the other does not.
    ///
    /// This is the `𝒢` content of a 0-cell. `apply_hue_map` is the one
    /// reader of a hue map's action (law 12), and composing through it
    /// keeps this permutation the same object `TorsionCycle.sigma` and
    /// `FiledOrbit.sigma` already are. `None` where the two maps do not
    /// compose — a hue one of them does not name — which is unread and is
    /// never an identity standing in for it (rule 11).
    pub fn gauge(&self) -> Option<Vec<(u8, u8)>> {
        // `derive_hue_map` IS the composer and is the one reader of this
        // relation (law 12): it derives the relabelling carrying one
        // colouring onto another link by link, which is exactly
        // `σ_to ∘ σ_from⁻¹` read as a map between the two charts' hue
        // listings. Two listings of different length do not relate and it
        // answers `None` — unread, never an identity put in its place
        // (rule 11).
        //
        // BOTH EMPTY IS THE IDENTITY AND IS A READING. A chart that moves
        // no hue has an empty listing, and two such charts agree; that is
        // `CycleType::Identity` and not an absence.
        crate::transducer::element::derive_hue_map(&self.gauge_from, &self.gauge_to)
    }

    /// The two charts recolour the body the same way: the fibre carries no
    /// gauge symmetry. An absent map is the identity permutation, which is
    /// what "this chart moves no hue" means.
    pub fn is_ungauged(&self) -> bool {
        matches!(
            self.gauge().map(|g| crate::transducer::sentence::CycleType::of(&g)),
            Some(crate::transducer::sentence::CycleType::Identity)
        )
    }

    /// 12.6 AT A 0-CELL, through the SAME classifier grade 1 and grade 2
    /// use (law 12): flat, a compact `ℤ/ℓℤ` symmetry about a fixed point, a
    /// standing class, or a rotation the alphabet cannot name.
    ///
    /// THE HUE IS AN ARGUMENT AND IT IS THIS FIBRE'S OWN (L12). It passed
    /// `CycleType::of(&[])` — the empty listing, the identity — which
    /// asserts that no 0-cell recolours, and that is exactly the half of
    /// the transition the spatial algebra does NOT make true. A target that
    /// recolours while it turns occupies `𝒢` as well as `𝒽`, and a reading
    /// taken at the identity hue answers for a claim the fibre never made.
    ///
    /// This is what makes grade 0 a pivot input rather than a gate. A
    /// `Congruent(κ, R, ℓ)` here is an INTRINSIC CYCLIC SYMMETRY of the
    /// body — and under T21(iii) that is a coordinate to be opened on the
    /// `𝒢` lane, not a form to be refused.
    pub fn fill(&self) -> Option<crate::transducer::cech::Fill> {
        Some(crate::transducer::cech::classify(
            &self.rotor()?,
            &self.carriage,
            crate::transducer::sentence::CycleType::of(&self.gauge()?),
        ))
    }

    /// The body is merely displaced between the two charts: `R_gauge ≡ ±1`,
    /// a placement and a dilation and no turn. Projective, because a versor
    /// and its negation are one transport (§1.5).
    ///
    /// A transition whose base action cannot be READ is not vertical and is
    /// not assumed to be (rule 11).
    pub fn is_vertical(&self) -> bool {
        self.rotor()
            .is_some_and(|r| r.projective_eq(&crate::transducer::alphabet::Rotor::identity()))
    }
}
