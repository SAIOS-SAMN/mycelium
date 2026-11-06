// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §4; MU-CLIENT 24.4 — SEVERAL PERSPECTIVES OF ONE SAMPLE ARE A
//! COVER (ROADMAP track 26 S6). Each perspective reads the one sample; the
//! cover compares the readings on the sample's own sites, where every
//! perspective's reading stands, and never through a reading of its own.
//!
//! - **The transition** on the overlap of two perspectives is the sensor's
//!   turn from one to the other ([`Perspective::turn_to`]), a signed
//!   permutation of the phases.
//! - **The cocycle** on three perspectives: the turn from the first to the
//!   second, then from the second to the third, is the turn from the first
//!   to the third. Where it closes the transitions agree around every
//!   triple; where it does not, the turn around the triple is held, named.
//! - **The 0-cochain** of a perspective is the contacts it reads: two
//!   neighbouring sites of one symbol, a step apart along a sample axis the
//!   perspective reads as space, are one body (MU-SENSING §3.2). A sample
//!   axis read as a sequence or a value carries no contact, so a body never
//!   extends along it.
//! - **The agreement** on an overlap: congruent where both read the same
//!   contacts, and otherwise the residue, the contacts each reads that the
//!   other does not. It is a reading, held and never discarded, and never a
//!   gate (MU-SENSING §4; 24.4).
//!
//! Readings are computed when asked, never stored: the cover holds the
//! sample and its perspectives alone.

use super::{Perspective, PerspectiveRefusal, Role};
use crate::transducer::base::Site;
use crate::transducer::family::transduce::SignedPermutation;
use crate::transducer::sample::Sample;

/// A CONTACT OF THE SAMPLE: `site` and its neighbour one step further along
/// the sample's axis `axis`, both of one symbol.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Contact {
    pub site: Site,
    pub axis: usize,
}

/// THE AGREEMENT OF TWO PERSPECTIVES ON ONE SAMPLE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Agreement {
    /// Both read the same contacts: the same bodies on the sample's sites.
    Congruent,
    /// The residue: the contacts only the first reads, and those only the
    /// second reads, each in canonical address order.
    Residue { first_only: Vec<Contact>, second_only: Vec<Contact> },
}

/// ONE OVERLAP OF THE COVER: the transition from the first perspective to
/// the second, and their agreement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Overlap {
    pub turn: SignedPermutation,
    pub agreement: Agreement,
}

/// THE COCYCLE ON THREE PERSPECTIVES.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cocycle {
    /// The turns compose: `g_ij ∘ g_jk = g_ik`.
    Closes,
    /// They do not: the turn around the triple, `g_ij ∘ g_jk ∘ g_ik⁻¹`.
    Obstructed { around: SignedPermutation },
}

/// THE PERSPECTIVES OF ONE SAMPLE, held as a cover.
#[derive(Clone, Debug)]
pub struct Cover<'a> {
    sample: &'a Sample,
    perspectives: Vec<Perspective>,
}

impl<'a> Cover<'a> {
    /// THE COVER OF `sample` BY `perspectives`, or `Rank` naming the first
    /// perspective that reads another count of axes than the sample shows.
    pub fn of(sample: &'a Sample, perspectives: Vec<Perspective>) -> Result<Cover<'a>, PerspectiveRefusal> {
        if let Some(p) = perspectives.iter().find(|p| p.rank() != sample.rank()) {
            return Err(PerspectiveRefusal::Rank { perspective: p.rank(), other: sample.rank() });
        }
        Ok(Cover { sample, perspectives })
    }

    /// The perspectives, in the order the cover was given them.
    pub fn perspectives(&self) -> &[Perspective] {
        &self.perspectives
    }

    /// THE CONTACTS PERSPECTIVE `i` READS, in canonical address order, then
    /// by axis. `None` where the cover holds no perspective `i`.
    pub fn contacts(&self, i: usize) -> Option<Vec<Contact>> {
        let p = self.perspectives.get(i)?;
        Some(contacts_read(self.sample, p))
    }

    /// THE OVERLAP OF PERSPECTIVES `i` AND `j`: the sensor's turn from `i`
    /// to `j`, and the agreement of their contacts. `None` where the cover
    /// holds no such perspective.
    pub fn overlap(&self, i: usize, j: usize) -> Option<Overlap> {
        let (pi, pj) = (self.perspectives.get(i)?, self.perspectives.get(j)?);
        let turn = pi.turn_to(pj).ok()?;
        let (ci, cj) = (contacts_read(self.sample, pi), contacts_read(self.sample, pj));
        let first_only: Vec<Contact> = ci.iter().filter(|c| cj.binary_search(c).is_err()).cloned().collect();
        let second_only: Vec<Contact> = cj.iter().filter(|c| ci.binary_search(c).is_err()).cloned().collect();
        let agreement = match (first_only.is_empty(), second_only.is_empty()) {
            (true, true) => Agreement::Congruent,
            _ => Agreement::Residue { first_only, second_only },
        };
        Some(Overlap { turn, agreement })
    }

    /// THE COCYCLE ON PERSPECTIVES `i`, `j`, `k`. `None` where the cover
    /// holds no such perspective.
    pub fn cocycle(&self, i: usize, j: usize, k: usize) -> Option<Cocycle> {
        let p = |x: usize| self.perspectives.get(x);
        let (pi, pj, pk) = (p(i)?, p(j)?, p(k)?);
        let (gij, gjk, gik) = (pi.turn_to(pj).ok()?, pj.turn_to(pk).ok()?, pi.turn_to(pk).ok()?);
        let composed = gij.then(&gjk).ok()?;
        match composed == gik {
            true => Some(Cocycle::Closes),
            false => Some(Cocycle::Obstructed { around: composed.then(&gik.inverse()).ok()? }),
        }
    }
}

/// THE CONTACTS `p` READS ON `sample`: every site and its neighbour a step
/// further along a sample axis `p` reads as space, where both hold one
/// symbol. Sorted, so an overlap reads membership by search.
fn contacts_read(sample: &Sample, p: &Perspective) -> Vec<Contact> {
    let extents = sample.extents();
    let space: Vec<usize> = (0..sample.rank()).filter(|&k| p.role_of_sample_axis(k) == Some(Role::Space)).collect();
    let mut out = Vec::new();
    for site in sample.sites() {
        let Ok(here) = sample.symbol_at(&site) else { continue };
        for &axis in &space {
            let (Some(c), Some(&e)) = (site.axis(axis), extents.get(axis)) else { continue };
            let Some(next) = c.checked_add(1).filter(|&n| usize::try_from(n).is_ok_and(|n| n < e)) else { continue };
            let Some(there) = site.with_axis(axis, next) else { continue };
            if sample.symbol_at(&there) == Ok(here) {
                out.push(Contact { site: site.clone(), axis });
            }
        }
    }
    out.sort_unstable();
    out
}
