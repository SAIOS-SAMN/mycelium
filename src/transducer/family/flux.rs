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

use crate::physics::laws::{flux_of_links, verify_flux, Flux};
use crate::transducer::sentence::{Demand, Link};
use crate::transducer::stalk::Invariant;

/// One demand as the percept carries it: its kind, its chart, its links.
pub type Named = (Demand, Invariant, Vec<Link>);

/// THE FLUXES ONE PERCEPT PERCEIVED: every non-motion demand read as the
/// flux its links carry and put to the law; the lawful ones returned, in the
/// demands' order. Each act is paid from the stroke: one spend per demand
/// read; a stroke that ends leaves the rest unread.
pub fn perceive_fluxes<'a>(demands: impl IntoIterator<Item = &'a Named>, budget: &mut crate::physics::Budget) -> Vec<Flux> {
    let mut lawful = Vec::new();
    for (kind, _, links) in demands {
        if !matches!(kind, Demand::Creation | Demand::TopologicalDeformation { .. } | Demand::Source { .. }) {
            continue;
        }
        if !budget.spend() {
            continue;
        }
        let Some(f) = flux_of_links(links) else {
            continue;
        };
        if !verify_flux(&f, budget).admitted() {
            continue;
        }
        // A mass past the carrier is unread, never capped (rule 11).
        if u64::try_from(f.mass.to_integer()).is_err() {
            continue;
        }
        lawful.push(f);
    }
    lawful
}
