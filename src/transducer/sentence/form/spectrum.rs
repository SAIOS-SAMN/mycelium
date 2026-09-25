// 54.5 — COHERENCE IS A SPECTRUM: what a form sounds like across its
// grades, its planes and its lanes, with nothing summed.
//
//! `coheres() -> bool` asked a form for a permission slip. Replacing it
//! with a count or a rational would be the same mistake with more digits:
//! a single number that rises as paths agree is an objective function, and
//! two forms of entirely different structure would reach it by different
//! routes and become indistinguishable — which destroys identity (§1.5).
//!
//! Coherence is POLYTONE. It is read on three orthogonal axes at once and
//! they are never mixed:
//!
//!   **Grade** — 0 the fibre over a vertex, 1 the cycles of `Z₁(M)`, 2 the
//!   triple overlaps of the nerve. A reading at one grade says nothing
//!   about another; a fibre that rings and a cover that shears are two
//!   facts and not a net position.
//!
//!   **Plane** — a rotation in four dimensions is a DOUBLE rotation,
//!   `Ω = R₁(θ₁)·R₂(θ₂)·T(τ)`, so a cycle does not have "a period", it has
//!   two, on orthogonal planes, plus the slip in `ker(I − R)`.
//!   [`crate::transducer::alphabet::Rotor::plane_periods`] is where that
//!   pair survives; `order()` is their lcm and is a reading of them.
//!
//!   **Lane** — `μ⃗ = ⟨S : G : T : H⟩`, and law 5 forbids summing them.
//!   Grade 0's gauge permutations are `G`; grade 1's rotations and slips
//!   are `S` and `T`; grade 2's cover classes are `H`.
//!
//! WHY MORE FIBRATIONS MEAN MORE COHERENCE, and why a boolean inverted it.
//! Under `P₁ ∧ P₂ ∧ … ∧ Pₙ` every additional cycle is another way to fail,
//! so the gate rewarded impoverished one-orbit trees. A single fibration
//! between two nodes carries no constraint at all — there is no cycle and
//! nothing to check. Two create an interference pattern. Dozens
//! OVERDETERMINE the geometry, and over exact ℚ an overdetermined system
//! either locks into one discrete lattice or refuses to close: there is no
//! float to absorb the difference. A thought many independent paths agree
//! on is not probable, it is forced.
//!
//! So this type counts nothing and totals nothing. It reports what was
//! heard, note by note, and the caller reads the chord.

use super::*;
use crate::transducer::alphabet::Order;
use crate::transducer::sentence::CycleType;

/// ONE CYCLE'S READING, on its own planes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyclePeriods {
    /// The chord the cycle closes on: which cycle this is, by the overlap's
    /// own coordinate (AT1).
    pub chord: coord::Overlap,
    /// The two commuting plane periods of the holonomy, uncollapsed. A
    /// plane at `Order::Infinite` is a rotation that never returns on that
    /// plane; the other may still be compact and is not hidden by it.
    pub planes: [Order; 2],
    /// The projective order — `lcm` of the two, or twice it. Kept beside
    /// them because it is what a `ℤ/ℓℤ` filing is keyed at, and NOT in
    /// place of them.
    pub projective: Order,
}

/// A SLIP NOTHING ABSORBS: the Burgers vector of a circuit that never closes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dislocation {
    /// Where it stands.
    pub chart: stalk::Invariant,
    /// `τ ∈ ker(I − R)`, exact. The translation `T(−τ)` that would close
    /// the circuit.
    pub tau: [crate::core::Q; 3],
    /// Whether the form DECLARED it — a `Demand::Dislocation` vertex
    /// carrying this slip. An undeclared shear is a form describing itself
    /// incompletely; a declared one is content, carried in the key.
    pub declared: bool,
}

/// WHAT A FORM SOUNDS LIKE. Every field is a listing; there is no total
/// anywhere and no field is comparable to another (law 5).
///
/// NOT `PartialEq`. Two spectra are not compared: a spectrum is a reading
/// taken of one form at one moment, and an equality on it would invite
/// exactly the ranking this type exists to prevent. Read the notes.
#[derive(Debug, Clone, Default)]
pub struct CoherenceSpectrum {
    /// GRADE 0, `𝒢` — the hue permutation across each vertex's fibre, by
    /// its cycle type. `CycleType::Identity` is two charts that colour the
    /// body alike; a `Type([…])` is an internal discrete phase the two
    /// charts see it under, which is a coordinate the pivot may open.
    pub gauge_cycles: Vec<(coord::VertexAt, CycleType)>,
    /// GRADE 0, `𝒮` — a fibre whose vertical transition is NOT a pure
    /// displacement. Provably empty for a vertex whose members share a
    /// transport (see `FiberTransition`), so an entry here means the
    /// transport-class keying itself broke.
    pub twisted_fibres: Vec<coord::VertexAt>,
    /// GRADE 1, `𝒮` and `𝒯` — each cycle's periods, on its own planes.
    pub cycles: Vec<CyclePeriods>,
    /// GRADE 1 and 0 — the shears no origin absorbs, with whether the form
    /// declared each.
    pub dislocations: Vec<Dislocation>,
    /// GRADE 2, `ℋ` — the triple overlaps of the nerve and what closed
    /// them. Empty until a 2-cell carrier populates it; an empty listing
    /// is "this form stands in no triple overlap", never "its cover
    /// closes".
    pub nerve: Vec<crate::transducer::cech::Fill>,
}

impl CoherenceSpectrum {
    /// EVERY CYCLE RINGS: both planes of every cycle are compact. A plane
    /// at `Infinite` is a rotation that never returns, and it is named as
    /// such rather than being folded into a score.
    pub fn compact(&self) -> bool {
        self.cycles
            .iter()
            .all(|c| c.planes.iter().all(|p| matches!(p, Order::Finite(l) if *l > 0)))
    }

    /// THE SHEARS THE FORM DID NOT DECLARE. This is the only thing in the
    /// spectrum that is a fault rather than a reading: a dislocation is
    /// content and is admitted when it is CARRIED, as a
    /// `Demand::Dislocation` vertex in the key. One the form does not
    /// mention is a description that omits its own defect.
    pub fn undeclared(&self) -> Vec<&Dislocation> {
        self.dislocations.iter().filter(|d| !d.declared).collect()
    }
}

impl Form {
    /// THE FORM'S SPECTRUM, derived and never stored (law 12).
    ///
    /// Nothing here consults a filing. Every note is a theorem of the
    /// complex: `orbit_classes` reads the holonomy and classifies it
    /// through `cech::classify`'s fixed-point gate, `plane_periods` splits
    /// the rotor into its two commuting factors, and `fiber_holonomy`
    /// reads the vertical transitions. A `Form.torsion` entry records that
    /// a stroke happened, which is mind-state, and it is not asked.
    pub fn spectrum(&self) -> CoherenceSpectrum {
        let mut s = CoherenceSpectrum::default();
        // GRADE 0. The gauge half is `𝒢`; the spatial half is a guard.
        for (i, t) in self.fiber_transitions() {
            if let Some(g) = t.gauge() {
                let ct = CycleType::of(&g);
                if !matches!(ct, CycleType::Identity) {
                    s.gauge_cycles.push((i.clone(), ct));
                }
            }
            if !t.is_vertical() {
                s.twisted_fibres.push(i);
            }
        }
        // GRADE 1. Each cycle on its own planes, and the slips beside them.
        let declared: Vec<(stalk::Invariant, [crate::core::Q; 3])> = self
            .demand_vertices()
            .filter_map(|(_, kind, _, _)| match kind {
                crate::transducer::sentence::Demand::Dislocation { chart, tau } => {
                    Some((chart.clone(), (**tau).clone()))
                }
                _ => None,
            })
            .collect();
        for lp in &self.orbits {
            let Some(hs) = self.edge_holonomies(lp) else {
                continue;
            };
            for (_, h) in hs.scoped() {
                let rotor = h.word.rotor();
                if rotor.projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                    continue;
                }
                let Some(planes) = rotor.plane_periods() else {
                    continue;
                };
                let Some(projective) = rotor.order() else {
                    continue;
                };
                s.cycles.push(CyclePeriods { chord: lp.chord.clone(), planes, projective });
                // THE SLIP, where the circuit never closes. `screw_of` splits the
                // transport exactly over ℚ; a zero twist is a turn that
                // returns and is not a dislocation.
                if let Some(screw) =
                    crate::transducer::element::screw_of(&rotor, &h.carriage)
                {
                    if screw.twist.iter().any(|q| !num_traits::Zero::is_zero(q)) {
                        // A CHART THAT DOES NOT READ IS NOT THE ROOT CHART
                        // (rule 11). `unwrap_or_default` put `Invariant([])`
                        // — which `stalk` makes the ROOT of the chart trie —
                        // in place of a reading that failed, so a slip whose
                        // site could not be named would have been filed at
                        // the coordinate every unsupported body already sits
                        // at. Unread is skipped.
                        //
                        // AT1 — THE SLIP STANDS AT THE CHORD'S OWN MEMBER
                        // CHART. This read `vertices.get(lp.chord)`, a FACE
                        // position used as a VERTEX position — two lists'
                        // slots confused, which a coordinate cannot be.
                        let Some(chart) = lp.chord.from.invariant().or_else(|| lp.chord.to.invariant()).cloned()
                        else {
                            continue;
                        };
                        let declared_here = declared
                            .iter()
                            .any(|(c, t)| *c == chart && t[..] == screw.twist[..]);
                        s.dislocations.push(Dislocation {
                            chart,
                            tau: screw.twist,
                            declared: declared_here,
                        });
                    }
                }
            }
        }
        s
    }
}
