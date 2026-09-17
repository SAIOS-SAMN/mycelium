// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 21 step 6 — CONTACT: placement differs across pairs, peel ring to Frame is one value.

use super::perceive::{frame_of, Cell3, Grid};
use super::*;
use crate::network::nerve::Keyed;
use crate::physics::Budget;
use super::orbit::positions_of;
use crate::transducer::element;
use crate::transducer::sentence::{Form, Vertex};

/// Context passed to contact pass.
pub(crate) struct ContactContext<'a> {
    pub(crate) class_place_varies: &'a Keyed<()>,
    pub(crate) class_pairs: &'a Keyed<Vec<u16>>,
    pub(crate) pair_member: &'a Keyed<crate::transducer::sentence::Acting>,
    pub(crate) pair_charts: &'a [Keyed<()>],
    pub(crate) n_pairs: usize,
}

/// Step 6 of chapter 21: when placement differs across pairs, check if peel ring to Frame is constant.
pub(crate) fn contact_named_vertices(
    form: &mut Form,
    family: &[(Grid, Grid)],
    ctx: &ContactContext<'_>,
    sections: &mut Keyed<Vec<u16>>,
    budget: &mut Budget,
) -> bool {
    let nverts = form.vertices.len();
    trace::trace_n("extract/6 contact", nverts, budget);
    for vi in 0..nverts {
        if !budget.spend() {
            return false;
        }
        let Some(tc) = transport_class(&form.vertices[vi]) else {
            continue;
        };
        let Some(Vertex::Word { contact: None, .. }) = form.vertices.get(vi) else {
            continue;
        };
        let place_varies = ctx.class_place_varies.contains(&tc.key());
        let mut rings: Option<u8> = None;
        let mut ring_one = true;
        for (_x, y) in family.iter() {
            let fy = frame_of(y);
            let outs: Vec<Cell3> = crate::arc::grid_to_cells(y);
            let (Some(last_row), Some(last_col)) =
                (fy.0.checked_sub(1), fy.1.checked_sub(1))
            else {
                continue;
            };
            let frame_anchor: Vec<(i64, i64)> = (0..fy.1 as i64)
                .map(|c| (0, c))
                .chain((0..fy.1 as i64).map(|c| (last_row as i64, c)))
                .chain((0..fy.0 as i64).map(|r| (r, 0)))
                .chain((0..fy.0 as i64).map(|r| (r, last_col as i64)))
                .collect();
            let bound = (fy.0.saturating_add(fy.1)) as u32;
            match ring_dist_peel(&positions_of(&outs), &frame_anchor, bound) {
                Some(r) => match rings {
                    None => rings = Some(r),
                    Some(h) if h != r => ring_one = false,
                    _ => {}
                },
                None => ring_one = false,
            }
        }
        if place_varies && !ring_one {
            if let Some(tc) = transport_class(&form.vertices[vi]) {
                let tk = tc.key();
                if let (Some(pairs), Vertex::Word { acts, .. }) = (ctx.class_pairs.get(&tk), &form.vertices[vi]) {
                    let matching_pairs: Vec<u16> = pairs
                        .iter()
                        .copied()
                        .filter(|&pi| {
                            acts.scoped().all(|(ck, a)| {
                                let mut pk = tk.clone();
                                crate::transducer::address::put_bytes(&mut pk, &ck);
                                pk.extend_from_slice(&pi.to_le_bytes());
                                match ctx.pair_member.get(&pk) {
                                    None => {
                                        let has_chart = ctx.pair_charts
                                            .get(pi as usize)
                                            .is_some_and(|pc| pc.contains(&ck));
                                        !has_chart
                                    }
                                    Some(pm) => {
                                        element::same_versor(&a.carriage, &pm.carriage)
                                            && crate::transducer::sentence::gauge_agrees(&a.gauge, &pm.gauge)
                                    }
                                }
                            })
                        })
                        .collect();
                    sections.insert(tk.clone(), matching_pairs);
                    for (ck, a) in acts.scoped() {
                        let mut mk = tk.clone();
                        crate::transducer::address::put_bytes(&mut mk, &ck);
                        let mps: Vec<u16> = pairs
                            .iter()
                            .copied()
                            .filter(|&pi| {
                                let mut pk = mk.clone();
                                pk.extend_from_slice(&pi.to_le_bytes());
                                match ctx.pair_member.get(&pk) {
                                    None => false,
                                    Some(pm) => {
                                        element::same_versor(&a.carriage, &pm.carriage)
                                            && crate::transducer::sentence::gauge_agrees(&a.gauge, &pm.gauge)
                                    }
                                }
                            })
                            .collect();
                        if mps.len() < ctx.n_pairs {
                            sections.insert(mk, mps);
                        }
                    }
                }
            }
        }
        if place_varies && ring_one {
            if let Some(r) = rings {
                if let Vertex::Word { contact, acts, .. } = &mut form.vertices[vi] {
                    *contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: r });
                    let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                    for k in charts {
                        if let Some(a) = acts.get_mut(&k) {
                            *a.carriage = element::identity_versor();
                        }
                    }
                }
            }
        }
    }
    true
}
