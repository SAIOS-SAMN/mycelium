// A9 — THE SELECTION TERM: the anchor a family derives, and the key that
// names it.

#[cfg(test)]
use crate::network::nerve::Keyed;
#[cfg(test)]
use crate::transducer::anchor::{derive_carrying_all, CarryingRefusal, Selection};

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
#[cfg(test)]
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
