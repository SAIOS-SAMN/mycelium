// 2.2, 50.1, 10.3 — WHAT MAKES A FORM CLOSED: coherence on its loops, the
// demands it stands with, its depth, its fibre's reach, and cauterization.
//
//! Split from `carrier.rs` at the §1.1 1200-line reading bound; the items
//! are moved, not rewritten (§1.6). They are one reading — every one of
//! them answers some part of "may this form be spoken, stood and ruled" —
//! and `coheres` is the one the court reaches through `REFUSE_NOT_CLOSED`.

use super::*;

impl Form {
    /// `δ¹g = 1` on every loop (10.3), where a loop whose class is an axis
    /// the form opened is coherent on the refined cover: its holonomy is
    /// the transition on that axis, re-derived from the loop (I6, 15.5).
    ///
    /// AND A LOOP THAT CLOSES WITH MONODROMY IS CLOSED (54.2, as ruled
    /// 2026-09-20). This read holonomy ≡ 1 on EVERY loop, which is not the
    /// cocycle condition — that lives on the triple overlaps of the nerve,
    /// `g_ij g_jk g_ki = 1` — but the strictly stronger claim that every
    /// 1-cycle has trivial holonomy. In one word: `H¹ = 0`. And
    /// `is_closed` calls this, only closed forms are stood
    /// (`epigenome::crystal`) and only closed forms survive the court
    /// (`REFUSE_NOT_CLOSED`), so the genome was by construction the
    /// subspace where there is nothing to measure. Measured on the fleet:
    /// 1 870 extracted forms carried one to nine loops, filed torsion was
    /// 0 bytes on every voice, and `REFUSE_NOT_CLOSED` was 36 of 39
    /// refusals on the wire.
    ///
    /// A closed cycle does not mean a trivial one. The holonomy around a
    /// loop IS its curvature, and a finite-order rotation about a fixed
    /// point RETURNS: it closes at ℓ, carrying a `ℤ/ℓℤ` class, which is
    /// torsion and is content (law 7, `curvature is content`).
    /// [`Form::loop_is_congruent`] is that test, through
    /// [`crate::transducer::cech::classify`]'s fixed-point gate, so a
    /// screw — which walks and never returns — is NOT admitted here: a
    /// non-compact free part is an obstruction the form has not resolved,
    /// and that is what a demand is for (52.3).
    ///
    /// `edge_coheres` is untouched and still holds on every loop: an edge
    /// that does not re-derive is a broken transport, not curvature.
    pub fn coheres(&self) -> bool {
        self.loops.iter().all(|lp| {
            (lp.holonomy.rotor().projective_eq(&Rotor::identity())
                || self.opened_axis_of(lp).is_some_and(|chord| self.axis_rederives(lp, chord))
                || self.loop_is_congruent(lp))
                && self.edge_coheres(lp)
        })
    }

    /// Closed: coheres, no remainder, and no demand vertex (2.2, 50.1);
    /// a sentence with demands is valid, never closed.
    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty() && self.demand_vertices().next().is_none()
    }

    /// The demand vertices of this sentence, with their indices (50.5),
    /// read off `demands`, which `reindex` built: one entry per (kind,
    /// chart), a keyed read and never a walk with a filter.
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
    /// READS `glued`, WHICH `reindex` BUILT: the same walk with a filter, and
    /// the index is keyed by the cited address, so a glued vertex reachable by
    /// the address it cites is reachable here.
    pub fn depth_of(&self) -> u16 {
        let mut d = 0u16;
        for (_, &i) in self.glued.scoped() {
            if let Some(Vertex::Glued { depth, .. }) = self.vertices.get(i as usize) {
                d = d.max(depth.saturating_add(1));
            }
        }
        d
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
    /// A finite period is exact and at least one (A17 — a loop is filed at
    /// the period it CLOSES at), so the subtraction is total; a class of
    /// infinite order, filed at `0` (52.3), condenses no members and reaches
    /// nothing here. No rounding, nothing saturating: this is a sum over an
    /// exact carrier, and an empty carrier sums to 0 because it is empty.
    pub fn fibre_volume(&self) -> usize {
        self.torsion
            .scoped()
            .map(|(_, l)| match Order::from_u32(l.period) {
                Order::Finite(ell) => ell as usize - 1,
                Order::Infinite => 0,
            })
            .sum()
    }

    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() || self.demand_vertices().next().is_some() {
            return false;
        }
        for lp in &self.loops {
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
            // fixed-point test — while `file_loop` derives the period
            // through `cech::classify`. A screw files at 0 there and was
            // demanded at 4 here, so a correctly filed form read as
            // uncauterized and a wrongly filed one read as cauterized.
            // Two readers of one loop, and this was the stale one.
            let rows = self.loop_classes(lp);
            if rows.is_empty() {
                return false;
            }
            for row in rows {
                match self.torsion.get(&row.class) {
                    Some(filed) if filed.period == row.period => {}
                    _ => return false,
                }
            }
        }
        true
    }}
