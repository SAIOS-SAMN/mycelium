// 23.1 steps 3–4, 50.2 G steps 2–4 — THE RECEIVER ON THE COURT'S COPY OF
// THE FAMILY: every word vertex reconstructs its body, every edge
// re-derives in the stabilizer, and the remainder is what the demands name.
//
//! Nothing here is trusted from the tape. A vertex is carried onto the
//! court's own copy of each pair and every landed cell is read; an edge is
//! re-measured on the union of the two bodies; a remainder cell stands only
//! where a demand vertex said it would (50.5: valid, not closed).

use super::*;
use crate::transducer::family::{perceive, Body, Grid};

/// What one vertex does on one pair of the family.
pub(crate) enum Carried {
    /// No body of the vertex's chart on this pair: a section elsewhere.
    Silent,
    /// Every cell of every body of its chart lands on its image with the
    /// gauged hue, and every touched image body is reached whole.
    Carries,
    /// A cell landed off its image, on the wrong hue, or an image body was
    /// reached in part.
    Fails,
}

/// The bodies of `x` on the vertex's chart, read by the chart's key.
fn fiber_on<'a>(bodies: &'a [Body], class: &stalk::Invariant) -> Vec<&'a Body> {
    let key = class.key();
    bodies.iter().filter(|b| b.moduli.key() == key).collect()
}

/// The hue a gauge carries `h` to; absence is the identity (18.2).
fn hue_under(gauge: &[(u8, u8)], h: u8) -> u8 {
    // index: the vertex's own gauge, its boundary (rule 1)
    for &(from, to) in gauge {
        if from == h {
            return to;
        }
    }
    h
}

fn cell_key(r: i64, c: i64) -> Vec<u8> {
    let mut k = Vec::with_capacity(16);
    k.extend_from_slice(&r.to_le_bytes());
    k.extend_from_slice(&c.to_le_bytes());
    k
}

/// 50.1 step 1 on one pair: `(V · W · γ) · B_in = B_out` under exact
/// cellular equality, for every body of the vertex's chart on `x`.
pub(crate) fn vertex_carries_pair(vertex: &Vertex, x: &Grid, y: &Grid) -> Carried {
    let Vertex::Word { word, carriage, gauge, class, contact, .. } = vertex else {
        return Carried::Silent;
    };
    // A contact vertex is placed by its orbit, not its carriage; the
    // orbit's own replay (`REFUSE_ORBIT_STILL`) is its reconstruction.
    if contact.is_some() {
        return Carried::Silent;
    }
    let Some(versor) = element::conformal_of(word, carriage) else {
        return Carried::Fails;
    };
    let bodies = perceive(x);
    let fiber = fiber_on(&bodies, class);
    if fiber.is_empty() {
        return Carried::Silent;
    }
    let mut landed: Keyed<()> = Keyed::new();
    for body in fiber {
        for &(r, c, h) in &body.cells {
            let Some(p) = element::carry_center(&versor, r, c) else {
                return Carried::Fails;
            };
            let (tr, tc) = element::cell_of(&p);
            let (Ok(ur), Ok(uc)) = (usize::try_from(tr), usize::try_from(tc)) else {
                return Carried::Fails;
            };
            let Some(&observed) = y.get(ur).and_then(|row| row.get(uc)) else {
                return Carried::Fails;
            };
            if observed != hue_under(gauge, h) {
                return Carried::Fails;
            }
            landed.insert(cell_key(tr, tc), ());
        }
    }
    // Every cell of every image body it touched is reached: the image is
    // a body, not a part of one.
    for image in perceive(y) {
        let touched = image.cells.iter().any(|&(r, c, _)| landed.contains(&cell_key(r, c)));
        if touched && !image.cells.iter().all(|&(r, c, _)| landed.contains(&cell_key(r, c))) {
            return Carried::Fails;
        }
    }
    Carried::Carries
}

/// 23.1 step 3 over the family: `REFUSE_VERTEX_BODY` for a word vertex
/// that fails on any pair; a vertex silent on every pair is a section
/// nothing here contradicts, and the closure check reads it.
pub(crate) fn vertices_carry_family(f: &Form, family: &[(Grid, Grid)]) -> Option<&'static str> {
    for v in &f.vertices {
        if !matches!(v, Vertex::Word { .. }) {
            continue;
        }
        for (x, y) in family {
            if let Carried::Fails = vertex_carries_pair(v, x, y) {
                return Some(REFUSE_VERTEX_BODY);
            }
        }
    }
    None
}

/// The body of `y` the image of `body`'s anchor cell falls in, under
/// `versor`. The anchor is the least cell in the body's own order — a
/// coordinate of the body, never a choice among its cells.
fn image_body(body: &Body, versor: &Versor, images: &[Body]) -> Option<usize> {
    let [(r, c, _), ..] = body.cells.as_slice() else {
        return None;
    };
    let (r, c) = (*r, *c);
    let (tr, tc) = element::cell_of(&element::carry_center(versor, r, c)?);
    images.iter().position(|b| b.cells.iter().any(|&(br, bc, _)| (br, bc) == (tr, tc)))
}

fn union_shape(a: &Body, b: &Body) -> crate::transducer::blade::Shape {
    let mut pos: Vec<(i64, i64)> = a.cells.iter().chain(b.cells.iter()).map(|&(r, c, _)| (r, c)).collect();
    pos.sort_unstable();
    pos.dedup();
    crate::transducer::blade::shape_of(&pos)
}

/// 23.1 step 4, 50.3 over the family: on every pair where each end of a
/// carried edge has exactly one body of its chart, `V_ij` is re-measured
/// on the union of the two bodies and `Γ_ij = V_i⁻¹ · V_ij` must be the
/// transport the face carries. A pair with several bodies of one chart is
/// sections, read by the closure check. `REFUSE_EDGE` when the union has
/// no versor (the speaker claimed an edge where the pair is not rigid) or
/// the transport is not the one measured.
pub(crate) fn edges_rederive_family(f: &Form, family: &[(Grid, Grid)]) -> Option<&'static str> {
    for face in &f.faces {
        let Some(edge) = &face.edge else {
            continue;
        };
        let (Some(vi), Some(vj)) = (f.vertices.get(face.i as usize), f.vertices.get(face.j as usize)) else {
            continue;
        };
        let (
            Vertex::Word { word: wi, carriage: ci, class: class_i, contact: None, .. },
            Vertex::Word { word: wj, carriage: cj, class: class_j, contact: None, .. },
        ) = (vi, vj)
        else {
            continue;
        };
        let Some(expected) = element::conformal_of(&edge.word, &edge.carriage) else {
            return Some(REFUSE_EDGE);
        };
        let (Some(v_i), Some(v_j)) = (element::conformal_of(wi, ci), element::conformal_of(wj, cj)) else {
            return Some(REFUSE_EDGE);
        };
        for (x, y) in family {
            let bodies = perceive(x);
            let images = perceive(y);
            let fiber_i = fiber_on(&bodies, class_i);
            let fiber_j = fiber_on(&bodies, class_j);
            let ([a_i], [a_j]) = (fiber_i.as_slice(), fiber_j.as_slice()) else {
                continue;
            };
            let (Some(bi), Some(bj)) = (image_body(a_i, &v_i, &images), image_body(a_j, &v_j, &images)) else {
                return Some(REFUSE_EDGE);
            };
            let Some(v_ij) = crate::transducer::blade::versor_between(
                &union_shape(a_i, a_j),
                &union_shape(&images[bi], &images[bj]),
            ) else {
                return Some(REFUSE_EDGE);
            };
            let Some(inv) = v_i.versor_inverse() else {
                return Some(REFUSE_EDGE);
            };
            if !element::same_versor(&inv.geo(&v_ij), &expected) {
                return Some(REFUSE_EDGE);
            }
        }
    }
    None
}

/// 23.1 step 7 as ruled (50.5): a remainder cell stands only where a demand
/// vertex named it — unspoken, and its observed hue the one the demand
/// carries — and a sentence with any other remainder does not close the
/// family. `true` when an undeclared remainder cell exists on any pair.
pub(crate) fn remainder_undeclared(remainders: &[Vec<(i64, i64, u8, u8)>], f: &Form) -> bool {
    let mut declared: Keyed<Vec<u8>> = Keyed::new();
    for (_, _, _, cells) in f.demand_vertices() {
        for &(r, c, _, after) in cells {
            let k = cell_key(r, c);
            match declared.get_mut(&k) {
                Some(hues) => hues.push(after),
                None => {
                    declared.insert(k, vec![after]);
                }
            }
        }
    }
    remainders.iter().flatten().any(|&(r, c, spoken, observed)| {
        spoken != 0 || !declared.get(&cell_key(r, c)).is_some_and(|hues| hues.contains(&observed))
    })
}
