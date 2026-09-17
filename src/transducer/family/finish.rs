// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 21, 50.5 — EXTRACTION FINISH AND DEMAND PACKING:
//! Demands ride the rule, unreached pairs ride as demands, and the form
//! is closed with its depth, loops, ratio and sections.

use super::perceive::{frame_of, perceive, Grid};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::{Form, SentenceRule};
use crate::transducer::stalk::Invariant;

/// An assignment's content key: carriage bytes and sorted gauge.
pub fn assignment_key(a: &crate::transducer::sentence::Acting) -> Vec<u8> {
    let mut buf = Vec::new();
    a.carriage.put_canonical(&mut buf);
    let mut g = a.gauge.clone();
    g.sort_unstable();
    buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
    for (x, y) in g {
        buf.push(x);
        buf.push(y);
    }
    buf
}

/// Demand raised by an unresolvable multiplicity at one chart within one pair (50.5).
pub(crate) fn multiplicity_demand(
    family: &[(Grid, Grid)],
    chart: &Invariant,
    multiplicity: u32,
) -> Option<(
    crate::transducer::sentence::Demand,
    Invariant,
    Vec<crate::transducer::sentence::Cell>,
)> {
    let key = chart.key();
    let mut cells: Vec<crate::transducer::sentence::Cell> = Vec::new();
    for (x, y) in family {
        for body in perceive(x) {
            if body.moduli.key() != key {
                continue;
            }
            for &(r, c, _, h) in &body.cells {
                let (Ok(ur), Ok(uc)) = (usize::try_from(r), usize::try_from(c)) else {
                    continue;
                };
                let observed = match y.get(ur).and_then(|row| row.get(uc)) {
                    Some(h) => *h,
                    None => 0,
                };
                if observed != h {
                    cells.push((r, c, h, observed));
                }
            }
        }
    }
    cells.sort_unstable();
    cells.dedup();
    if cells.is_empty() {
        return None;
    }
    let kind = crate::transducer::sentence::Demand::Source { multiplicity };
    kind.typed().ok()?;
    Some((kind, chart.clone(), cells))
}

/// The germ of one pair: what the input carries onto the output, as the
/// cells of the change. `None` for a pair with no scene.
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
            n = n.saturating_add(h * w);
        }
    }
    n
}

/// The tail of an extraction: the demands ride the rule and the form is
/// closed out with its depth, loops, ratio and sections.
pub(crate) fn finish(
    mut form: Form,
    pair_forms: &[Form],
    scale: ((usize, usize), (usize, usize)),
    sections: Keyed<Vec<u16>>,
    unreached: &[(Grid, Grid)],
    multiplicities: &[(
        crate::transducer::sentence::Demand,
        Invariant,
        Vec<crate::transducer::sentence::Cell>,
    )],
) -> Option<SentenceRule> {
    let mut by_demand: Keyed<Demanded> = Keyed::new();
    for pf in pair_forms {
        for (_, kind, chart, cells) in pf.demand_vertices() {
            let key = crate::transducer::sentence::demand_key(kind, chart);
            match by_demand.get_mut(&key) {
                Some((_, _, held)) => held.extend_from_slice(cells),
                None => {
                    by_demand.insert(key, (kind.clone(), chart.clone(), cells.to_vec()));
                }
            }
        }
    }
    for (kind, chart, cells) in multiplicities {
        let key = crate::transducer::sentence::demand_key(kind, chart);
        match by_demand.get_mut(&key) {
            Some((_, _, held)) => held.extend_from_slice(cells),
            None => {
                by_demand.insert(key, (kind.clone(), chart.clone(), cells.clone()));
            }
        }
    }
    for (a, b) in unreached {
        let Some(germ) = germ_of_pair(a, b) else { continue };
        let chart = crate::transducer::stalk::invariant(&germ);
        let multiplicity = crate::transducer::sentence::pieces(&germ).len() as u32;
        let kind = crate::transducer::sentence::Demand::Source { multiplicity };
        if kind.typed().is_err() {
            continue;
        }
        let key = crate::transducer::sentence::demand_key(&kind, &chart);
        match by_demand.get_mut(&key) {
            Some((_, _, held)) => held.extend_from_slice(&germ),
            None => {
                by_demand.insert(key, (kind, chart, germ.to_vec()));
            }
        }
    }
    for (_, (kind, chart, cells)) in by_demand.scoped() {
        let mut cells = cells.clone();
        cells.sort_unstable();
        cells.dedup();
        form.vertices.push(demand_vertex(kind.clone(), chart, cells));
    }
    form.depth = form.depth_of();
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    form.ratio = scale;
    form.sections = sections;
    form.hue_maps = hue_maps_of_form(&form);
    Some(crate::transducer::sentence::rule_of_form(form))
}
