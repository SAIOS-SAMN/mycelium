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
//! - **A missing perspective is a demand** (ruled 2026-11-06 on #32 (2)).
//!   Where an overlap's agreement is a residue, the cocycle does not close
//!   on the sample's 0-cochains: a chart one perspective reads is missing
//!   from the other. The residue is the demand: the cover lists a typed
//!   vertex ([`PerspectiveDemand`]) naming the sample (its place in its
//!   family), the overlap (the two perspectives) and the contacts each
//!   alone reads. Asking is a demand, never a fallback (MU-SENSING §4), and
//!   nothing in the cover chooses the perspective that would answer it.
//!
//! Readings are computed when asked, never stored: the cover holds the
//! sample and its perspectives alone.

use super::{Perspective, PerspectiveRefusal, Role};
use crate::transducer::base::Site;
use crate::transducer::family::transduce::SignedPermutation;
use crate::transducer::sample::{Place, Sample};

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

/// A PERSPECTIVE ASKED FOR (ruled 2026-11-06 on #32 (2)): the typed vertex a
/// cover lists where an overlap's cocycle does not close because a chart is
/// missing. It names the sample by its place in its family and the overlap
/// by its two perspectives, in the cover's order, and carries the residue,
/// which is the demand: the contacts the first reads and the second does
/// not, and those the second reads and the first does not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerspectiveDemand {
    /// The sample's place in its family.
    pub place: Place,
    /// The overlap's first perspective.
    pub first: Perspective,
    /// The overlap's second perspective.
    pub second: Perspective,
    /// The contacts only the first reads: the chart the second is missing.
    pub first_only: Vec<Contact>,
    /// The contacts only the second reads: the chart the first is missing.
    pub second_only: Vec<Contact>,
}

impl PerspectiveDemand {
    /// THE VERTEX'S KEY, its coordinates written out (A21): the place, each
    /// perspective's pose and roles, then each contact's site and axis. Two
    /// demands are one vertex exactly where the key is one.
    pub fn key(&self) -> Vec<u8> {
        use crate::transducer::address::put_key_count;
        let mut out = Vec::new();
        let coordinates = self.place.coordinates();
        put_key_count(&mut out, coordinates.len());
        for c in coordinates {
            out.extend_from_slice(&c.to_le_bytes());
        }
        for p in [&self.first, &self.second] {
            put_key_count(&mut out, p.rank());
            for &(source, sense) in p.pose().images() {
                put_key_count(&mut out, source);
                out.push(u8::from(sense == crate::transducer::family::transduce::Sense::Reversed));
            }
            for &r in p.roles() {
                out.push(match r {
                    Role::Space => 0,
                    Role::Sequence => 1,
                    Role::Value => 2,
                });
            }
        }
        for contacts in [&self.first_only, &self.second_only] {
            put_key_count(&mut out, contacts.len());
            for c in contacts {
                let axes = c.site.axes();
                put_key_count(&mut out, axes.len());
                for a in axes {
                    out.extend_from_slice(&a.to_le_bytes());
                }
                put_key_count(&mut out, c.axis);
            }
        }
        out
    }
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

    /// THE DEMAND THE OVERLAP OF `i` AND `j` LISTS: where its agreement is a
    /// residue, the perspective the cover is missing there, named by the
    /// sample's place and the two perspectives and carrying the residue.
    /// `None` where the two are congruent (no chart is missing) or the cover
    /// holds no such perspective.
    pub fn demand(&self, i: usize, j: usize) -> Option<PerspectiveDemand> {
        let Agreement::Residue { first_only, second_only } = self.overlap(i, j)?.agreement else { return None };
        Some(PerspectiveDemand {
            place: self.sample.place().clone(),
            first: self.perspectives.get(i)?.clone(),
            second: self.perspectives.get(j)?.clone(),
            first_only,
            second_only,
        })
    }

    /// EVERY DEMAND THE COVER LISTS: one vertex per overlap `i < j` whose
    /// cocycle does not close on the 0-cochains, in the cover's order. A
    /// cover whose overlaps are all congruent lists none. Read when asked,
    /// never stored; never discarded (MU-SENSING §4; 24.4).
    pub fn demands(&self) -> Vec<PerspectiveDemand> {
        let n = self.perspectives.len();
        (0..n).flat_map(|i| (i + 1..n).filter_map(move |j| self.demand(i, j))).collect()
    }
}

/// THE DEMANDS A FAMILY'S SAMPLES LIST UNDER `perspectives`: each sample's
/// cover read in turn ([`Cover::demands`]), in the order the samples are
/// handed in, one vertex per key ([`PerspectiveDemand::key`]). `Rank`
/// where a perspective reads another count of axes than a sample shows.
pub fn listed(samples: &[&Sample], perspectives: &[Perspective]) -> Result<Vec<PerspectiveDemand>, PerspectiveRefusal> {
    let mut out: Vec<PerspectiveDemand> = Vec::new();
    for s in samples {
        for d in Cover::of(s, perspectives.to_vec())?.demands() {
            let key = d.key();
            if !out.iter().any(|held| held.key() == key) {
                out.push(d);
            }
        }
    }
    Ok(out)
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
