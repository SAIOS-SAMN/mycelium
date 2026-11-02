// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 21, 50.5 — EXTRACTION FINISH: the frames the budget did not reach ride
//! the rule as demands, and the form is closed with its depth, orbits, ratio
//! and sections.

use super::perceive::{frame_of, Grid};
use super::*;
use crate::transducer::sentence::{Form, SentenceRule};

/// An assignment's content key: carriage bytes and sorted gauge (A4, A21).
pub fn assignment_key(a: &crate::transducer::sentence::Acting) -> Vec<u8> {
    let mut buf = Vec::new();
    a.carriage.put_canonical(&mut buf);
    let mut g = a.gauge.clone();
    g.sort_unstable();
    crate::transducer::address::put_key_count(&mut buf, g.len());
    for (x, y) in g {
        buf.push(x);
        buf.push(y);
    }
    buf
}

/// The germ of one pair: what the form did not say about it, which for a
/// pair it never reached is the residue the identity's speech leaves on
/// the output frame (`deflation::unspoken_residue`, the pair's transduction
/// projected under the extraction's stroke) — the same reading the
/// deflation prices the pair by, so the demand and the measure cannot
/// disagree. Where the identity lays down no grid the residue is the whole
/// output against nothing: still a remainder (rule 7), never dropped and
/// never read off a shared corner of two frames. `None` for a pair with no
/// scene. The demand it stands as is the sentence's ASK (78.149 (b)), its
/// links and their count the word's measure, never μ: it may cap what a
/// closer is paid and is never escrowed or minted, so it stays a count
/// (ruled 2026-11-01; μ, a share of the frame, is the discharge).
pub(crate) fn germ_of_pair(
    a: &Grid,
    b: &Grid,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::element::PlacedChain> {
    let residue = super::deflation::unspoken_residue(a, b, budget);
    if residue.is_empty() {
        return None;
    }
    crate::transducer::element::place_chain(&residue)
}

/// The spend one extraction of this family is sized by (rule 10, L8w, ruled
/// 2026-09-26): what the extraction reads — every cell of every frame, and
/// for each pair of frames the transitions its bodies are related by, at
/// most [`crate::transducer::blade::CANDIDATES_PER_PAIR`] candidates each —
/// read off the frames perceived ONCE ([`perceive_family`]).
///
/// THE PAIRS ARE THE CONTACTS, NEVER THE PRODUCT (rule 1). The percept
/// relates two bodies only where the bodies' supports meet or stand at ring
/// distance one (the keyed occupancy), and disjoint connected bodies of one
/// plane touching that way form a planar graph: by Euler's formula a planar
/// graph on `V` vertices has at most `3V − 6` edges. So `3V` bounds the
/// pairs a frame pair's `V` bodies can be read in, and the draw is linear in
/// the bodies — a Cartesian `V²` would license the search rule 1 refuses.
///
/// A STROKE SIZE, never μ: the reads an extraction may take, so it counts
/// reads and feeds no payment (ruled 2026-11-01).
pub fn extraction_spend(family: &[(Grid, Grid)], frames: &super::perceive::Frames) -> usize {
    let mut n = 0usize;
    for (a, b) in family {
        for g in [a, b] {
            let (h, w) = frame_of(g);
            #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): the cell counts of resident grids cannot sum past a 64-bit usize")]
            let next = n.saturating_add(h * w);
            n = next;
        }
    }
    for (ins, outs) in frames {
        // Euler's bound on the contacts of the frame pair's bodies (above).
        let bodies = ins.len() + outs.len();
        #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): the contacts of resident frames times a fixed candidate bound cannot pass a 64-bit usize")]
        let reads = bodies.saturating_mul(3).saturating_mul(crate::transducer::blade::CANDIDATES_PER_PAIR);
        #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): the reads of resident frames cannot sum past a 64-bit usize")]
        let next = n.saturating_add(reads);
        n = next;
    }
    // THE TRANSDUCTIONS (MU-DERIVATIONS D3.2): a pair of unequal enclosures
    // is related by its incidence record, whose need is read off its four
    // periods (`transduce::need`, `R_h·R_w`, R = h + h′ − gcd per circle).
    // A pair of one enclosure visits at most its own sites, already counted
    // above.
    for (a, b) in family {
        let (Ok(x), Ok(y)) = (crate::arc::sample_unplaced(a), crate::arc::sample_unplaced(b)) else { continue };
        let (p, q) = (super::transduce::Enclosure::of(&x), super::transduce::Enclosure::of(&y));
        if p.is_same_periods(&q) {
            continue;
        }
        let Some(need) = super::transduce::need(&p, &q) else { continue };
        #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): the transduction needs of resident pairs are each a usize (`transduce::need`, checked) and the sum of them cannot pass a 64-bit usize")]
        let next = n.saturating_add(need);
        n = next;
    }
    n
}

/// The tail of an extraction: the unreached frames ride the rule as the
/// demands of 50.5, and the form is closed out with its depth, orbits and
/// ratio. Its sections are its members' frames, already on it (AT1).
/// Each unreached pair is transduced under the extraction's stroke, which
/// [`extraction_spend`] sized for it, and stands as the demand its
/// unspoken residue is ([`germ_of_pair`], rule 7).
pub(crate) fn finish(
    mut form: Form,
    scale: ((usize, usize), (usize, usize)),
    unreached: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> Option<SentenceRule> {
    for (a, b) in unreached {
        let Some(germ) = germ_of_pair(a, b, budget) else { continue };
        let chart = crate::transducer::stalk::invariant(&germ);
        let multiplicity = crate::transducer::sentence::pieces(&germ).len();
        let kind = crate::transducer::sentence::Demand::Source { multiplicity };
        if kind.typed().is_err() {
            continue;
        }
        form.stand_demand(kind, &chart, germ.to_vec());
    }
    form.depth = form.depth_of()?;
    form.orbits = Form::orbits_of(&form.faces, &form.vertices);
    form.ratio = scale;
    form.hue_maps = hue_maps_of_form(&form);
    Some(crate::transducer::sentence::rule_of_form(form))
}
