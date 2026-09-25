// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 21, 50.5 — EXTRACTION FINISH: the frames the budget did not reach ride
//! the rule as demands, and the form is closed with its depth, orbits, ratio
//! and sections.

use super::perceive::{frame_of, Grid};
use super::*;
use crate::network::nerve::Keyed;
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

/// The germ of one pair: what the input carries onto the output, as the
/// links of the change. `None` for a pair with no scene.
pub(crate) fn germ_of_pair(a: &Grid, b: &Grid) -> Option<crate::transducer::element::PlacedChain> {
    let cochain = crate::transducer::family::refine::refine_delta(a, b)?;
    if cochain.is_empty() {
        return None;
    }
    crate::transducer::element::place_chain(&cochain)
}

/// The spend one extraction of this family is sized by (rule 10, L8w).
pub fn extraction_spend(family: &[(Grid, Grid)]) -> usize {
    let mut n = 0usize;
    for (a, b) in family {
        for g in [a, b] {
            let (h, w) = frame_of(g);
            #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): the cell counts of resident grids cannot sum past a 64-bit usize")]
            let next = n.saturating_add(h * w);
            n = next;
        }
    }
    n
}

/// The tail of an extraction: the unreached frames ride the rule as the
/// demands of 50.5, and the form is closed out with its depth, orbits, ratio
/// and sections.
pub(crate) fn finish(
    mut form: Form,
    scale: ((usize, usize), (usize, usize)),
    sections: Keyed<Keyed<()>, crate::network::nerve::coord::SectionAt>,
    unreached: &[(Grid, Grid)],
) -> Option<SentenceRule> {
    for (a, b) in unreached {
        let Some(germ) = germ_of_pair(a, b) else { continue };
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
    form.sections = sections;
    form.hue_maps = hue_maps_of_form(&form);
    Some(crate::transducer::sentence::rule_of_form(form))
}
