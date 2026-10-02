// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! D2, directives 1–2 and 4 (68, ruled 2026-09-22) — THE THERMODYNAMIC
//! LEDGER, READ OFF SITES THAT ALREADY EXIST.
//!
//! Nothing here is a new mechanism. Every quantity this counts was already
//! being produced and thrown away:
//!
//! ```text
//!   ENTROPY    a cauterization — `Drained::Cauterized` (dimension 1) and
//!              `Descended::Released` (dimension 2). An IRREVERSIBLE
//!              termination: a loop that cannot be re-entered from the
//!              loop it just left, which is what makes the record it
//!              leaves a record and not a state.
//!   SYNTROPY   a condensation — order minted, a contact carrying its
//!              refraction (BQ5a). NOT counted here:
//!              `ContactTally::condensed` already carries
//!              it, and a second counter for one fact is law 12's defect
//!              inside a single carrier. It is READ beside this line.
//!              AT DIMENSION 2 IT IS COUNTED HERE: `sealed`, an ascent
//!              that sealed a cell (AD1) — the one producer of that fact,
//!              beside `released`, its entropy at the same dimension, and
//!              never netted against it.
//!   ORBIT ℓ    `Order(Rotor)` — the cyclotomic period, a discrete integer
//!              bound, and the structure `μ` is carried BY rather than a
//!              scalar it is flattened into.
//!   RATE       `Reserve::draw` / `restore` per lane. The metabolic flux
//!              is the draw, and it is anisotropic by construction: four
//!              lanes, never a summed scalar (#34).
//! ```
//!
//! WHY `μ` IS NOT A FIELD HERE. `μ = ⟨Ω⟩₀²/⟨ΩΩ̃⟩₀ − 1` is the engine's
//! kinematic charge and `δ` is the inversive distance, related by
//! `μ = (δ−1)/2`. Writing a second quantity called `μ` — `δ²`, say —
//! beside the first would put two readings under one name in one carrier,
//! which is exactly the defect A.12 was raised to fix in the harmonic
//! tower. `μ`'s SIGN is already read as the four-way holonomy class
//! (`orbit_defects`), and its structure is the orbit period counted here.

/// Per-lane and per-event metabolic reading, taken where the event
/// happens. Arrays, never a summed scalar (#34): a lane's flux is that
/// lane's, and netting them would trade one against another.
#[derive(Default, Clone, Copy, Debug)]
pub struct Metabolism {
    /// ENTROPY at dimension 1 — a drain that cauterized rather than
    /// finding vacuum.
    pub cauterized: u64,
    /// ENTROPY at dimension 2 — a descent that released a fill.
    pub released: u64,
    /// SYNTROPY at dimension 2 — an ascent that sealed a cell (AD1). Kept
    /// beside `released`, never netted against it: the pair is history
    /// (62.5 as amended), and a seal and a release are two events.
    pub sealed: u64,
    /// Metabolic draw per lane (𝒮, 𝒢, 𝒯, 𝒽) — the flux itself.
    pub drawn: [u64; 4],
    /// Returned to the reserve: `draw`'s conjugate, and the half that says
    /// whether the engine is spending or cycling.
    pub restored: [u64; 4],
    /// Draws refused, BY THE LANE THAT REFUSED THEM (PHY1): a depleted lane
    /// does not preempt the others, so a mind exhausts a degree of freedom
    /// and not its time, and the reading says which.
    pub depleted: [u64; 4],
    /// Rotors whose orbit closes, and the sum of their periods — `Σℓ`,
    /// which with `orbit_finite` gives the mean period without storing a
    /// histogram.
    pub orbit_finite: u64,
    pub orbit_sum: u64,
    /// Motions that turn at no order the algebra admits — rosettes, which
    /// never close. Kept apart from `ℓ = 0`, because A19 spells infinite
    /// order that way and the two readings are opposite (53.6).
    pub orbit_infinite: u64,
    /// Motions with a mode that does not turn at all — a transvection or a
    /// null shear, UNBOUND by name (PHY1): it carries and never returns, and
    /// was counted closed by the rotation's order alone.
    pub orbit_unbound: u64,
    /// Motions whose modes did not read: no versor, undecomposed, or the
    /// census's stroke spent — unmeasured, never counted closed or open.
    pub orbit_unread: u64,
    /// VS1 — ENTROPY BY LANE: bends whose trial triggered an irreversible
    /// event of the ledger or lost a reading (78.7 as amended), per lane
    /// (𝒮, 𝒢, 𝒯, 𝒽), never summed.
    pub viscous: [u64; 4],
    /// STD1 (rule 11) — steps the ledger REFUSED because the counter would
    /// pass its carrier: the counter stands where it was, never pinned at a
    /// ceiling that would be an invented value.
    pub overflowed: u64,
}

thread_local! {
    static METABOLISM: std::cell::Cell<Metabolism> =
        const { std::cell::Cell::new(Metabolism {
            cauterized: 0, released: 0, sealed: 0, drawn: [0; 4], restored: [0; 4],
            depleted: [0; 4], orbit_finite: 0, orbit_sum: 0, orbit_infinite: 0, orbit_unbound: 0, orbit_unread: 0, viscous: [0; 4],
            overflowed: 0,
        }) };
}

/// ONE STEP OF A COUNTER: `slot + x`, or the step refused and counted where
/// the sum would pass the carrier.
fn step(slot: &mut u64, x: u64, overflowed: &mut u64) {
    match slot.checked_add(x) {
        Some(v) => *slot = v,
        None => *overflowed += 1,
    }
}

fn note(f: impl Fn(&mut Metabolism)) {
    METABOLISM.with(|c| {
        let mut v = c.get();
        f(&mut v);
        c.set(v);
    });
}

/// STD1 — A STEP OF A PHYSICS COUNTER OUTSIDE THIS LEDGER: `slot + x`, or the
/// step refused — the counter standing where it was — and counted on this
/// ledger's `overflowed` where the sum would pass the carrier. Returns
/// whether it stepped.
pub fn step_or_refuse(slot: &mut u64, x: u64) -> bool {
    match slot.checked_add(x) {
        Some(v) => {
            *slot = v;
            true
        }
        None => {
            note(|m| m.overflowed += 1);
            false
        }
    }
}

/// An irreversible termination at dimension 1 (`Drained::Cauterized`).
pub fn note_cauterized() {
    note(|m| m.cauterized += 1);
}

/// An irreversible termination at dimension 2 (`Descended::Released`).
pub fn note_released() {
    note(|m| m.released += 1);
}

/// A cell sealed at dimension 2 by `ascend` (AD1).
pub fn note_sealed() {
    note(|m| m.sealed += 1);
}

/// VS1 — a viscous bend on lane `lane` (𝒮 0, 𝒢 1, 𝒯 2, 𝒽 3): the lane's
/// entropy.
pub fn note_viscous(lane: usize) {
    note(|m| {
        if let Some(slot) = m.viscous.get_mut(lane) {
            step(slot, 1, &mut m.overflowed);
        }
    });
}

/// One draw against the reserve that was afforded, per lane.
pub fn note_draw(cost: &[u64; 4]) {
    note(|m| {
        for (slot, c) in m.drawn.iter_mut().zip(cost.iter()) {
            step(slot, *c, &mut m.overflowed);
        }
    });
}

/// One draw the reserve refused, on the lane that could not cover it.
pub fn note_refused(lane: usize) {
    note(|m| {
        if let Some(slot) = m.depleted.get_mut(lane) {
            step(slot, 1, &mut m.overflowed);
        }
    });
}

/// One return to the reserve, per lane.
pub fn note_restore(cost: &[u64; 4]) {
    note(|m| {
        for (slot, c) in m.restored.iter_mut().zip(cost.iter()) {
            step(slot, *c, &mut m.overflowed);
        }
    });
}

/// One motion's period, read off its own modes (PHY1) — the cyclotomic
/// structure `μ` is carried by, and the motions that never return named by
/// why.
pub fn note_orbit(period: Option<crate::transducer::cga::Period>) {
    use crate::transducer::cga::Period;
    note(|m| match period {
        Some(Period::Closes(n)) => {
            m.orbit_finite += 1;
            step(&mut m.orbit_sum, n, &mut m.overflowed);
        }
        Some(Period::Rosette) => m.orbit_infinite += 1,
        Some(Period::Unbound) => m.orbit_unbound += 1,
        None => m.orbit_unread += 1,
    });
}

/// Read the ledger and clear it.
pub fn take_metabolism() -> Metabolism {
    METABOLISM.with(|c| c.replace(Metabolism::default()))
}

/// The ledger without clearing it, for a per-form line beside a running
/// total (law 12: only one reader may take).
pub fn metabolism() -> Metabolism {
    METABOLISM.with(|c| c.get())
}

impl Metabolism {
    /// THE METABOLIC LINE. Entropy and the orbit structure by name, the
    /// four lanes kept apart, and `syntropy` read from the field's own
    /// counter by the caller rather than duplicated here.
    pub fn line(&self) -> String {
        format!(
            "entropy: cauterized {} released {} | sealed {} | orbit: closed {} Σℓ {} rosette {} unbound {} unread {} \
             | draw {:?} restore {:?} depleted {:?} | overflowed {}",
            self.cauterized,
            self.released,
            self.sealed,
            self.orbit_finite,
            self.orbit_sum,
            self.orbit_infinite,
            self.orbit_unbound,
            self.orbit_unread,
            self.drawn,
            self.restored,
            self.depleted,
            self.overflowed,
        )
    }

    /// THE NET ORDER FLUX, per lane and never summed: what was drawn
    /// against what came back. A lane that draws and restores equally is
    /// CYCLING; one that only draws is spending down.
    ///
    /// Exact over the whole of both counters: the difference of two `u64`
    /// is taken in `i128`, which holds every such difference, so no lane
    /// is ever read at an invented bound (rule 11).
    pub fn flux(&self) -> [i128; 4] {
        std::array::from_fn(|i| i128::from(self.drawn[i]) - i128::from(self.restored[i]))
    }

    /// Total irreversible terminations across both dimensions.
    pub fn entropy(&self) -> u64 {
        self.cauterized + self.released
    }
}
