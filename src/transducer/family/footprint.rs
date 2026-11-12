// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A9 (MU-DERIVATIONS D3.3; ruled 2026-11-06, #158) — A WORD'S FOOTPRINT ON
//! A PAIR: the support of its description, the sites where the word's
//! speech of the pair's input carries a change between input and output.
//! It is deduced from the word itself, by speaking it, and never declared.
//! Beside it, the pair's own change: the sites where the pair's input and
//! output differ. The court binds π* from the two (`court::verdict::bind_pi`).

use super::speak::{apply_form_metered, residue_of, Spoken};
use super::Crystal;
use crate::transducer::sentence::Form;
use crate::transducer::spelled::{SectionSample, SectionSite};

/// WHAT A WORD'S SPEECH OF A PAIR'S INPUT CAME TO, read for its footprint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Footprint {
    /// The sites the speech changes on the input, sorted: the word's
    /// support there. Empty where the word changes nothing.
    Read(Vec<SectionSite>),
    /// No speech stands: it does not fit its own frame, it cites a form the
    /// crystal does not hold, or its change cannot be written on the open
    /// ground. Nothing is deduced, so nothing is bound.
    Unspoken,
    /// The stroke ran out inside the speech: unread, never empty.
    Spent,
}

/// THE SITES OF A CHAIN, sorted: where its links stand.
fn sites_of(links: &[crate::transducer::sentence::SectionLink]) -> Vec<SectionSite> {
    let mut sites: Vec<SectionSite> = links.iter().map(|l| l.site).collect();
    sites.sort_unstable();
    sites.dedup();
    sites
}

/// THE WORD'S FOOTPRINT ON INPUT `x`: `form` spoken onto `x` on the stroke
/// (one read per cited form spoken afresh, as the court's deflation reads
/// it), and the sites where the speech differs from `x`. `pair` is the
/// pair's index in its family, as the deflation speaks it.
pub fn footprint_of(
    form: &Form,
    x: &SectionSample,
    pair: Option<usize>,
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Footprint {
    let here = crate::transducer::alphabet::Word::identity();
    let said = match apply_form_metered(form, x, &here, crystal, pair, budget, None) {
        Spoken::Spent => return Footprint::Spent,
        Spoken::Unfit => return Footprint::Unspoken,
        Spoken::Said(s) => s.sample(),
    };
    let Some(said) = said else { return Footprint::Unspoken };
    match residue_of(Some(x), &said) {
        Ok(change) => Footprint::Read(sites_of(&change)),
        Err(_) => Footprint::Unspoken,
    }
}

/// THE PAIR'S OWN CHANGE: the sites where input `x` and output `y` differ,
/// sorted. `None` where the change cannot be written on the open ground.
pub fn pair_change(x: &SectionSample, y: &SectionSample) -> Option<Vec<SectionSite>> {
    residue_of(Some(x), y).ok().map(|change| sites_of(&change))
}

/// THE SITES OF A CHANGE A FOOTPRINT MISSES, sorted: the pair changes them
/// and the word does not. A footprint that misses any does not bind the
/// word on that pair, and these are what it missed.
pub fn missed(footprint: &[SectionSite], change: &[SectionSite]) -> Vec<SectionSite> {
    change.iter().copied().filter(|s| footprint.binary_search(s).is_err()).collect()
}
