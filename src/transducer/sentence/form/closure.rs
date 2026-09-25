// 2.2, 50.1, 10.3 — WHAT MAKES A FORM CLOSED: coherence on its orbits, the
// demands it stands with, its depth, its fibre's reach, and cauterization.
//
//! Split from `carrier.rs` at the §1.1 1200-line reading bound; the items
//! are moved, not rewritten (§1.6). They are one reading — every one of
//! them answers some part of "may this form be spoken, stood and ruled" —
//! and `coheres` is the one the court reaches through `REFUSE_NOT_CLOSED`.

use super::*;

impl Form {
    /// `δ¹g = 1` on every orbit (10.3), where an orbit whose class is an axis
    /// the form opened is coherent on the refined cover: its holonomy is
    /// the transition on that axis, re-derived from the orbit (I6, 15.5).
    ///
    /// AND AN ORBIT THAT CLOSES WITH MONODROMY IS CLOSED (54.2, as ruled
    /// 2026-09-20). This read holonomy ≡ 1 on EVERY orbit, which is not the
    /// cocycle condition — that lives on the triple overlaps of the nerve,
    /// `g_ij g_jk g_ki = 1` — but the strictly stronger claim that every
    /// 1-cycle has trivial holonomy. In one word: `H¹ = 0`. And
    /// `is_closed` calls this, only closed forms are stood
    /// (`epigenome::crystal`) and only closed forms survive the court
    /// (`REFUSE_NOT_CLOSED`), so the genome was by construction the
    /// subspace where there is nothing to measure. Measured on the fleet:
    /// 1 870 extracted forms carried one to nine orbits, filed torsion was
    /// 0 bytes on every voice, and `REFUSE_NOT_CLOSED` was 36 of 39
    /// refusals on the wire.
    ///
    /// A closed cycle does not mean a trivial one. The holonomy around a
    /// orbit IS its curvature, and a finite-order rotation about a fixed
    /// point RETURNS: it closes at ℓ, carrying a `ℤ/ℓℤ` class, which is
    /// torsion and is content (law 7, `curvature is content`).
    /// [`Form::orbit_is_congruent`] is that test, through
    /// [`crate::transducer::cech::classify`]'s fixed-point gate, so a
    /// screw — which slips and never returns — is NOT admitted here: a
    /// non-compact free part is an obstruction the form has not resolved,
    /// and that is what a demand is for (52.3).
    ///
    /// `edge_coheres` is untouched and still holds on every orbit: an edge
    /// that does not re-derive is a broken transport, not curvature.
    pub fn coheres(&self) -> bool {
        self.incoherent_orbit().is_none()
    }

    /// CV1 — THE FIRST ORBIT THAT DOES NOT COHERE, the one reader `coheres`
    /// asks, so a refusal can carry the holonomy that refused (78.9).
    pub fn incoherent_orbit(&self) -> Option<&Orbit> {
        self.orbits.iter().find(|lp| {
            !((lp.holonomy.rotor().projective_eq(&Rotor::identity())
                || self.opened_axis_of(lp).is_some_and(|chord| self.axis_rederives(lp, chord))
                || self.orbit_is_congruent(lp))
                && self.edge_coheres(lp))
        })
    }

    /// DOES THIS FORM DECLARE THE SHEAR ON THIS CLASS? (54.5, Directive 4.)
    ///
    /// A free part — a circuit whose transport composes to `T(τ)·R` with
    /// `τ ∈ ker(I − R)` — never returns, so no period closes it and no
    /// filing can. What the form owes it is a `Demand::Dislocation`
    /// carrying the slip: a VERTEX, in the key, saying what the obstruction
    /// is and what would close it.
    ///
    /// THE MATCH IS ON `(chart, τ)` AND NOT ON A NAME. A demand declares a
    /// cycle when it carries that cycle's own Burgers vector ON THE CHART
    /// IT WAS MEASURED, and both halves are load-bearing.
    ///
    /// Matching the CLASS KEY would let one declaration answer for another
    /// shear of the same conjugacy class: two cycles may share a rotor `R`,
    /// hence a class, while slipping along orthogonal directions of
    /// `ker(I − R)` — `τ₁ = (1,0,0)` and `τ₂ = (0,0,1)` are two
    /// obstructions and one of them is not the other.
    ///
    /// Matching τ ALONE would treat a Burgers vector as an ambient
    /// invariant, and it is not one: it is coordinatized by the local
    /// frame, and under a transition `g_AB = D(ρ)·T(t)·R` it transforms as
    /// `τ_B = ρ·R(τ_A)`. A form owes a declaration for each chart boundary
    /// it fails to close, which is why `stand_demand` keys on
    /// `(kind, chart)` and why the emitter mounts one cell per chart.
    ///
    /// A form declaring nothing answers `false`, and that is the honest
    /// reading of a shear nobody mentioned (rule 11).
    pub(crate) fn declares_dislocation(
        &self,
        class: &crate::network::nerve::coord::Holonomy,
    ) -> bool {
        use crate::transducer::sentence::Demand;
        let mut owed: Vec<(stalk::Invariant, [crate::core::Q; 3])> = Vec::new();
        for (i, lp) in self.orbits.iter().enumerate() {
            let Some(hs) = self.edge_holonomies(lp) else { continue };
            for (_, h) in hs.scoped() {
                let rotor = h.word.rotor();
                if rotor.holonomy_key().as_ref() != Some(class) {
                    continue;
                }
                let Some(s) = crate::transducer::element::screw_of(&rotor, &h.carriage) else {
                    continue;
                };
                if s.twist.iter().all(num_traits::Zero::is_zero) {
                    continue;
                }
                for chart in self.charts_of_orbit_at(i) {
                    owed.push((chart, s.twist.clone()));
                }
            }
        }
        if owed.is_empty() {
            return false;
        }
        // EVERY BOUNDARY, NOT ONE OF THEM. An orbit crossing two charts owes
        // two declarations, and answering `true` on the first would admit a
        // form that named one of its shears and hid the other.
        owed.iter().all(|(chart, tau)| {
            self.demand_vertices().any(|(_, kind, at, _)| {
                at == chart
                    && matches!(kind, Demand::Dislocation { tau: t, .. } if **t == *tau)
            })
        })
    }

    /// GRADE 0 OVER THE WHOLE FORM: every vertical transition in every
    /// fibre, with the vertex it stands over.
    ///
    /// IT RETURNS THE TRANSITIONS AND NOT A VERDICT (rule 4). The pivot
    /// reads this to find the `𝒢`-lane coordinates a form is carrying
    /// unopened; the instrument reads it to count them; a caller that
    /// genuinely wants a yes-or-no asks each transition whether it is
    /// vertical. A reader that answered `Vec<u16>` of "bad" vertices would
    /// have collapsed an exact conformal element to an index and a
    /// judgement, which is the scalar this grade exists to stop making.
    #[expect(clippy::cast_possible_truncation, reason = "AT1 debt — a slot number of the form's Vec (a vertex's position in self.vertices); the atlas row replaces the slot by the chart's coordinates")]
    pub fn fiber_transitions(&self) -> Vec<(u16, crate::transducer::sentence::FiberTransition)> {
        self.vertices
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                v.fiber_holonomy().into_iter().map(move |t| (i as u16, t))
            })
            .collect()
    }

    /// The `𝒢`-LANE OBSTRUCTIONS A FORM CARRIES: every vertical transition
    /// that is not a pure displacement, with the class it stands in.
    ///
    /// A `Fill::Congruent` here is an intrinsic `ℤ/ℓℤ` symmetry of the body
    /// — a coordinate the pivot may open, paying `−1` torsion `+1` axes on
    /// `𝒢` under T21(iii). A standing class is a parallax the form has not
    /// absorbed, which is `Demand::Dislocation`'s object. Neither is a
    /// refusal.
    pub fn gauge_obstructions(
        &self,
    ) -> Vec<(u16, crate::transducer::sentence::FiberTransition, crate::transducer::cech::Fill)> {
        self.fiber_transitions()
            .into_iter()
            .filter(|(_, t)| !t.is_vertical())
            .filter_map(|(i, t)| t.fill().map(|f| (i, t, f)))
            .collect()
    }

    /// Closed: coheres, no remainder, and no demand vertex (2.2, 50.1);
    /// a sentence with demands is valid, never closed.
    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty() && self.demand_vertices().next().is_none()
    }

    /// The demand vertices of this sentence, with their indices (50.5),
    /// read off `demands`, which `reindex` built: one entry per (kind,
    /// chart), a keyed read and never a whole-carrier read with a filter.
    pub fn demand_vertices(&self) -> impl Iterator<Item = (u16, &crate::transducer::sentence::Demand, &stalk::Invariant, &[Link])> {
        self.demands.scoped().filter_map(move |(_, &i)| {
            match self.vertices.get(i as usize) {
                Some(Vertex::Demand { kind, chart, bodies }) => {
                    Some((i, kind, chart, bodies.as_slice()))
                }
                _ => None,
            }
        })
    }

    /// One more than the deepest glued vertex; zero where none is glued.
    ///
    /// READS `glued`, WHICH `reindex` BUILT: the same whole-carrier read with a filter, and
    /// the index is keyed by the cited address, so a glued vertex reachable by
    /// the address it cites is reachable here.
    ///
    /// `None` where a glued vertex stands at the carrier's last depth, so one
    /// more is no `u16` (STD1): a depth that cannot be written is refused,
    /// never pinned at the last.
    pub fn depth_of(&self) -> Option<u16> {
        let mut d = 0u16;
        for (_, &i) in self.glued.scoped() {
            if let Some(Vertex::Glued { depth, .. }) = self.vertices.get(i as usize) {
                d = d.max(depth.checked_add(1)?);
            }
        }
        Some(d)
    }

    /// S9.13'S REACH — the span this form's fibre spans ABOVE ITS BASE, which
    /// is what E2 prices as a closure's yield. One filed cycle of period ℓ
    /// reaches ℓ−1 members past the one it opens on, and speaks every one of
    /// them in O(1) storage: that is the capacity a condensation earns.
    ///
    /// THE FORM IS THE CARRIER, so this is where the reading lives. A minted
    /// sentence's element leaves its own two fibre fields empty because the
    /// cycles are on the form, so `Element::fibre_volume` answers 0 for it and
    /// is not this reading — asking the element was how the instrument came to
    /// report 1 everywhere by construction.
    ///
    /// A FORM HAS NO FREE-SECTION CARRIER and none is substituted. `sections`
    /// here is `Keyed<Vec<u16>>` holding PAIR INDICES of the bundle — which
    /// pairs a class was measured on — where `Element.sections` holds
    /// transition ASSIGNMENTS over a base. Two objects, and summing the first
    /// as the second is the fault a rename row is owed on. A form's free part
    /// is absent, not zero.
    ///
    /// A finite period is exact and at least one (A17 — an orbit is filed at
    /// the period it CLOSES at), so the subtraction is total; a class of
    /// infinite order, filed at `0` (52.3), condenses no members and reaches
    /// nothing here. No rounding, nothing saturating: this is a sum over an
    /// exact carrier, and an empty carrier sums to 0 because it is empty.
    /// THE CLASS COMES FROM THE RECEIPT, THE PERIOD FROM THE COMPLEX (54.5,
    /// as ruled 2026-09-20):
    ///
    /// ```text
    ///     yield(F, r) = Σ_{κ ∈ r ∩ dom(orbit_classes F)} (ℓ_derived(κ) − 1)
    /// ```
    ///
    /// This summed `(ℓ − 1)` over `self.torsion` — the form's own filing —
    /// so the claimant supplied both halves of its own price. `Σ(ℓ − 1)` is
    /// reserve regeneration (E2), so an unchecked period mints capacity out
    /// of nothing, and the field was self-supplied, which made the attack
    /// surface invisible rather than absent.
    ///
    /// A receipt is a SUB-SELECTION: it names WHICH cycles this mind's
    /// stroke discharged, and the complex says what each closes at. A class
    /// the form does not carry projects to ZERO and is not an error — an
    /// observer may track cycles outside this form's horizon, which is
    /// pruning and not fraud. A period the receipt states and the complex
    /// contradicts is perjury, and `receipts_price_the_terrain` refuses it
    /// at the court; here it simply cannot inflate the sum, because the sum
    /// never reads it.
    ///
    /// A FINITE PERIOD IS EXACT AND AT LEAST ONE (A17), so the subtraction
    /// is total; a class of infinite order, filed at `0` (52.3), condenses
    /// no members and reaches nothing.
    #[expect(clippy::cast_possible_truncation, reason = "PD1 debt — the yield narrows with fibre_volume and the period, which PD1 widens together")]
    pub fn fibre_volume(
        &self,
        filed: &Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy>,
    ) -> usize {
        let mut derived: Keyed<
            crate::transducer::sentence::ToralPeriod,
            crate::network::nerve::coord::Holonomy,
        > = Keyed::new();
        for lp in &self.orbits {
            for row in self.orbit_classes(lp) {
                derived.put(&row.class, row.periods);
            }
        }
        filed
            .entries()
            .filter_map(|(k, _)| derived.at(&k).copied())
            // THE LABOUR OF BOTH PLANES: `(ℓ₁ − 1) + (ℓ₂ − 1)`. It read
            // `(lcm − 1)`, which is neither plane's (54.7). A plane that
            // does not close earns nothing — that is torsion, charged as
            // shear, never estimated through an invented finite order.
            .map(|p| match p.yield_units() {
                Some(u) => u as usize,
                None => 0,
            })
            .sum()
    }

    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() || self.demand_vertices().next().is_some() {
            return false;
        }
        for lp in &self.orbits {
            if lp.holonomy.is_identity() {
                continue;
            }
            if let Some(chord) = self.opened_axis_of(lp) {
                if self.axis_rederives(lp, chord) {
                    continue;
                }
                return false;
            }
            // Filed under its class at the period its order reads, `0` for
            // infinite order (52.3, A19). A holonomy that is not a versor
            // has no class (15.3) and nothing files it: the form is not
            // cauterized. No key of the rotor's bytes stands in for the
            // class it lacks.
            //
            // THE PERIOD IS ASKED OF THE FILER'S OWN READER (law 12). This
            // read `lp.holonomy.order()` — the ROTOR'S order, with no
            // fixed-point test — while `file_orbit` derives the period
            // through `cech::classify`. A screw files at 0 there and was
            // demanded at 4 here, so a correctly filed form read as
            // uncauterized and a wrongly filed one read as cauterized.
            // Two readers of one orbit, and this was the stale one.
            let rows = self.orbit_classes(lp);
            if rows.is_empty() {
                return false;
            }
            for row in rows {
                match self.torsion.at(&row.class) {
                    // BOTH PLANES, not the envelope: a claim that agrees on
                    // `lcm` and disagrees on the pair is a different orbit
                    // (54.7).
                    Some(filed) if filed.periods == row.periods => {}
                    _ => return false,
                }
            }
        }
        true
    }}
