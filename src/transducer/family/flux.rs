// 78.91 — THE MIND PERCEIVES WHAT IS NO MOTION AS FLUX (the progenitor's
// build, 2026-10-04, deduced from the fleet's first reading after the recut:
// tens of thousands of bodies parting "apart" per mind, refused as no
// motion). The law register publishes continuity (law version 3): mass is
// made only at a recorded source and lost only into a recorded sink. This is
// the sense that reads the world in that law.
//
//! Every change the percept names that no versor carries — an output body
//! nothing carries onto (a CREATION), an input body under nothing (a
//! VANISHING), a body whose image changed its shape (a DEFORMATION), several
//! bodies gathered into one (a SOURCE of multiplicity) — already holds its
//! evidence as links `(r, c, before, after)`. From them the flux is read
//! exactly (`laws::flux_of_links`: mass gained a source at the gained sites'
//! barycentre, mass lost a sink at the lost sites'), and put to the law
//! register (`laws::verify_flux`). What the law admits is perceived as
//! physics — a source or a sink of the mind's own world — and returned to the
//! percept that read it; a reshaping at constant mass is no flux and stays
//! the deformation it is. HOLDING the fluxes across percepts, so the mind
//! relates its sources across a family as it relates its bodies, is the next
//! faculty (78.91's owed row).

use crate::physics::laws::{flux_of_links, verify_flux, Flow, Flux};
use crate::transducer::sentence::{Demand, Link};
use crate::transducer::stalk::Invariant;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// One demand as the percept carries it: its kind, its chart, its links.
pub type Named = (Demand, Invariant, Vec<Link>);

/// THE FLUXES ONE PERCEPT PERCEIVED: every non-motion demand read as the
/// flux its links carry and put to the law; the lawful ones returned, in the
/// demands' order. Each act is paid from the stroke: one spend per demand
/// read; a stroke that ends leaves the rest unread, counted, never zero.
pub fn perceive_fluxes<'a>(demands: impl IntoIterator<Item = &'a Named>, budget: &mut crate::physics::Budget) -> Vec<Flux> {
    let mut lawful = Vec::new();
    for (kind, _, links) in demands {
        if !matches!(kind, Demand::Creation | Demand::TopologicalDeformation { .. } | Demand::Source { .. }) {
            continue;
        }
        if !budget.spend() {
            note(UNREAD, 1);
            continue;
        }
        note(READ, 1);
        let Some(f) = flux_of_links(links) else {
            note(RESHAPED, 1);
            continue;
        };
        let v = verify_flux(&f, budget);
        if !v.admitted() {
            note(REFUSED, 1);
            continue;
        }
        // A mass past the carrier is unread, never capped (rule 11).
        let Ok(mass) = u64::try_from(f.mass.to_integer()) else {
            note(UNREAD, 1);
            continue;
        };
        match f.flow {
            Flow::Source => {
                note(SOURCES, 1);
                note(EMITTED, mass);
            }
            Flow::Sink => {
                note(SINKS, 1);
                note(ABSORBED, mass);
            }
        }
        lawful.push(f);
    }
    lawful
}

const SLOTS: usize = 8;
static TALLY: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];
const READ: usize = 0;
const SOURCES: usize = 1;
const SINKS: usize = 2;
const EMITTED: usize = 3;
const ABSORBED: usize = 4;
const RESHAPED: usize = 5;
const REFUSED: usize = 6;
const UNREAD: usize = 7;

fn note(slot: usize, by: u64) {
    if by > 0 && TALLY[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

/// WHAT THE MIND HAS PERCEIVED AS FLUX since the process began: the
/// non-motion changes it read, the lawful sources and sinks with the mass
/// emitted and absorbed, the reshapings at constant mass (no flux), the
/// fluxes the law refused, and those a spent stroke left unread.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FluxTally {
    pub read: u64,
    pub sources: u64,
    pub sinks: u64,
    pub emitted: u64,
    pub absorbed: u64,
    pub reshaped: u64,
    pub refused: u64,
    pub unread: u64,
}

/// 78.95 — THE TALLY'S NAMES, in [`FluxTally::row`]'s order: the typed
/// reading the voice's Senses carry (FX2 owed it one).
pub const FLUX: [&str; 8] = ["read", "sources", "emitted", "sinks", "absorbed", "reshaped", "refused", "unread"];

impl FluxTally {
    /// In [`FLUX`]'s order.
    pub fn row(&self) -> [u64; 8] {
        [self.read, self.sources, self.emitted, self.sinks, self.absorbed, self.reshaped, self.refused, self.unread]
    }
}

pub fn flux_tally() -> FluxTally {
    let r = |i: usize| TALLY[i].load(Relaxed);
    FluxTally {
        read: r(READ),
        sources: r(SOURCES),
        sinks: r(SINKS),
        emitted: r(EMITTED),
        absorbed: r(ABSORBED),
        reshaped: r(RESHAPED),
        refused: r(REFUSED),
        unread: r(UNREAD),
    }
}
