// A9 — THE SELECTION TERM: the anchor a family derives, and the act that
// binds it (MU-DERIVATIONS D3.3; ruled 2026-10-31 and 2026-11-06, #158).

use crate::network::nerve::Keyed;
use crate::transducer::anchor::{derive_carrying_all, CarryingRefusal, Selection};
use crate::transducer::family::footprint::{footprint_of, missed, pair_change, Footprint};
use crate::transducer::spelled::SectionSite;

/// π* DERIVED FROM A POOLED EVIDENCE FAMILY, BINDING THE WHOLE COMMON SET
/// (MU-DERIVATIONS D3.3, ruled 2026-10-31): every anchor term common to every
/// training evidence's carrying set, read by key (the intersection move —
/// one evidence never binds; the caller holds families of at least two),
/// held as a SUPERPOSITION and judged by the court's reconstruction over the
/// `withheld` pairs: a term stands where its evaluation on each withheld
/// scene carries that pair's footprint, and every term that stands is kept.
/// Nothing is chosen by a key or a count: the set is returned in its keys'
/// order, which orders without consuming. A spent token refuses the whole
/// derivation, naming how far the evidence it stopped on was read.
pub fn derive_pi(
    family: &[crate::transducer::anchor::Evidence],
    withheld: &[crate::transducer::anchor::Evidence],
    budget: &mut crate::physics::Budget,
) -> Result<Vec<Selection>, CarryingRefusal> {
    let mut sets: Vec<Keyed<Selection>> = Vec::with_capacity(family.len());
    for (fp, scene) in family {
        sets.push(derive_carrying_all(fp, scene, budget)?);
    }
    let Some((first, rest)) = sets.split_first() else { return Ok(Vec::new()) };
    let mut out = Vec::new();
    for (terms_read, (key, t)) in first.scoped().enumerate() {
        if !rest.iter().all(|s| s.contains(&key)) {
            continue;
        }
        let mut stands = true;
        for (fp, scene) in withheld {
            match crate::transducer::anchor::carries_footprint(t, fp, scene, budget) {
                Some(true) => {}
                Some(false) => {
                    stands = false;
                    break;
                }
                None => return Err(CarryingRefusal::Unread { terms_read }),
            }
        }
        if stands {
            out.push(t.clone());
        }
    }
    Ok(out)
}

/// A WITHHELD PAIR THE WORD IS NOT BOUND ON (#158): its index among the
/// task's withheld pairs, and the sites of the pair's change the word's
/// footprint misses there (the whole change where the word's speech of it
/// stands nowhere).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unbound {
    pub pair: usize,
    pub missed: Vec<SectionSite>,
}

/// WHAT THE A9 JUDGEMENT BOUND: the common set π* over the pairs the word
/// is bound on, every standing term kept, and each withheld pair it is not
/// bound on. `terms` is empty (⊤) where no withheld pair binds the word: a
/// common set no withheld pair judged is not π*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiBinding {
    pub terms: Vec<Selection>,
    pub unbound: Vec<Unbound>,
}

/// WHY THE A9 JUDGEMENT BOUND NOTHING: the stroke ran out (in a carrying
/// set, or inside the word's speech of a pair, `pair` its index in the
/// train or the withheld pairs), or the word's speech of a training pair
/// stands nowhere, so its family's evidence is not whole. The whole
/// binding is refused, never a part of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiRefusal {
    Carrying(CarryingRefusal),
    FootprintUnread { withheld: bool, pair: usize },
    TrainUnspoken { pair: usize },
}

/// THE A9 JUDGEMENT (ruled 2026-11-06, #158): π* bound for the word `form`
/// on its family. Each training pair's evidence is the word's footprint on
/// it (`footprint_of`: the sites the word's speech changes, deduced from the
/// word and never declared) on the pair's input scene (its painted sites).
/// Each withheld pair — the pairs fixed by coordinate when the task was
/// registered, never chosen here — binds the word only where the word's
/// footprint there covers the pair's own change; one it misses is named in
/// `unbound` and judges nothing. `derive_pi` binds the common set over the
/// pairs that bind, metered on `budget`.
pub fn bind_pi(
    form: &crate::transducer::sentence::Form,
    train: &crate::transducer::family::Family,
    withheld: &crate::transducer::family::Family,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
) -> Result<PiBinding, PiRefusal> {
    let mut family = Vec::with_capacity(train.len());
    for (i, (x, _)) in train.iter().enumerate() {
        match footprint_of(form, x, Some(i), crystal, budget) {
            Footprint::Read(fp) => family.push((fp, crate::transducer::raster::painted_sites(x))),
            Footprint::Unspoken => return Err(PiRefusal::TrainUnspoken { pair: i }),
            Footprint::Spent => return Err(PiRefusal::FootprintUnread { withheld: false, pair: i }),
        }
    }
    let mut judged = Vec::with_capacity(withheld.len());
    let mut unbound = Vec::new();
    for (j, (x, y)) in withheld.iter().enumerate() {
        let change = pair_change(x, y);
        let fp = match footprint_of(form, x, Some(j), crystal, budget) {
            Footprint::Read(fp) => Some(fp),
            Footprint::Unspoken => None,
            Footprint::Spent => return Err(PiRefusal::FootprintUnread { withheld: true, pair: j }),
        };
        match (fp, change) {
            (Some(fp), Some(change)) => {
                let gone = missed(&fp, &change);
                if gone.is_empty() {
                    judged.push((fp, crate::transducer::raster::painted_sites(x)));
                } else {
                    unbound.push(Unbound { pair: j, missed: gone });
                }
            }
            (None, Some(change)) => unbound.push(Unbound { pair: j, missed: change }),
            // The pair's change cannot be written on the open ground: there
            // is no change to cover, and the pair binds nothing.
            (_, None) => unbound.push(Unbound { pair: j, missed: Vec::new() }),
        }
    }
    if judged.is_empty() {
        return Ok(PiBinding { terms: Vec::new(), unbound });
    }
    let terms = derive_pi(&family, &judged, budget).map_err(PiRefusal::Carrying)?;
    Ok(PiBinding { terms, unbound })
}
