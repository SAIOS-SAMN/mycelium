// 23.1 steps 3–4, 50.2 G steps 2–4 — THE RECEIVER ON THE COURT'S COPY OF
// THE FAMILY: every word vertex reconstructs its body, every edge
// re-derives in the stabilizer, and the remainder is what the demands name.
//
//! Nothing here is trusted from the tape. A vertex is carried onto the
//! court's own copy of each pair and every landed cell is read; an edge is
//! re-measured on the union of the two bodies; a remainder cell stands only
//! where a demand vertex said it would (50.5: valid, not closed). Each
//! refusal names its coordinate as content (50.9): the vertex with its
//! chart, the face's two addresses, the chart with the cells read.

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
    /// reached in part — and WHICH of those, because they are five
    /// different faults and `REFUSE_VERTEX_BODY` said only that one of
    /// them happened. On the fleet at epoch 11 that refusal was the
    /// court's commonest by a factor of five and the journal could not
    /// distinguish a voice whose rotor is wrong from a voice reading a
    /// different frame than the court replays against (rule 11).
    Fails(&'static str),
}

/// What a vertex failed at, for the journal. Coordinates, not a verdict.
pub(crate) const FAIL_NO_VERSOR: &str = "the rotor and carriage lift to no versor";
pub(crate) const FAIL_NO_CARRY: &str = "a cell of its body carries nowhere";
pub(crate) const FAIL_OFF_FRAME: &str = "a cell lands outside the image frame";
pub(crate) const FAIL_HUE: &str = "a cell lands on the wrong hue";
pub(crate) const FAIL_PART: &str = "an image body is reached in part";

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
    let Vertex::Word { derivation, contact, .. } = vertex else {
        return Carried::Silent;
    };
    // THE COURT REPLAYS THE MEMBER the speaker spoke by. One member today;
    // D7b is where a vertex carries several and the court checks each
    // against the bodies on that member's own chart.
    let Some(member) = vertex.acting() else {
        return Carried::Silent;
    };
    let (carriage, gauge, class) = (&member.carriage, &member.gauge, &member.chart);
    let rotor = derivation.rotor();
    // A contact vertex is placed by its orbit, not its carriage; the
    // orbit's own replay (`REFUSE_ORBIT_STILL`) is its reconstruction.
    if contact.is_some() {
        return Carried::Silent;
    }
    let Some(versor) = element::conformal_of_rotor(&rotor, carriage) else {
        return Carried::Fails(FAIL_NO_VERSOR);
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
                return Carried::Fails(FAIL_NO_CARRY);
            };
            // A centre the sensor cannot convert is off the frame (W6).
            let Some((tr, tc)) = element::cell_of(&p) else {
                return Carried::Fails(FAIL_OFF_FRAME);
            };
            let (Ok(ur), Ok(uc)) = (usize::try_from(tr), usize::try_from(tc)) else {
                return Carried::Fails(FAIL_OFF_FRAME);
            };
            let Some(&observed) = y.get(ur).and_then(|row| row.get(uc)) else {
                return Carried::Fails(FAIL_OFF_FRAME);
            };
            if observed != hue_under(gauge, h) {
                return Carried::Fails(FAIL_HUE);
            }
            landed.insert(cell_key(tr, tc), ());
        }
    }
    // Every cell of every image body it touched is reached: the image is
    // a body, not a part of one.
    for image in perceive(y) {
        let touched = image.cells.iter().any(|&(r, c, _)| landed.contains(&cell_key(r, c)));
        if touched && !image.cells.iter().all(|&(r, c, _)| landed.contains(&cell_key(r, c))) {
            return Carried::Fails(FAIL_PART);
        }
    }
    Carried::Carries
}

/// 23.1 step 3 over the family: `REFUSE_VERTEX_BODY` at the vertex, with
/// its chart, for a word vertex that fails on any pair; a vertex silent on
/// every pair is a section nothing here contradicts, and the closure check
/// reads it.
pub(crate) fn vertices_carry_family(f: &Form, family: &[(Grid, Grid)]) -> Option<Refusal> {
    for v in &f.vertices {
        let Vertex::Word { address, .. } = v else {
            continue;
        };
        let Some(class) = v.acting().map(|a| &a.chart) else {
            continue;
        };
        // A VERTEX IS ASKED FOR THE PAIRS ITS RULE CLAIMS. A class whose
        // placement varies across the bundle is a section (18.2), and the
        // rule names the pairs it was measured on; asking it for the rest
        // is asking it to answer for a claim it did not make. This checked
        // every word vertex against every pair, so a section was refused
        // for being one — 80 of 87 vertex-body refusals at epoch 13.
        //
        // A class the rule declares no section for is claimed whole and is
        // checked whole, which is what it says of itself.
        let only = crate::transducer::family::transport_class(v)
            .and_then(|tc| f.sections.get(&tc.key()).cloned());
        for (pair, (x, y)) in family.iter().enumerate() {
            if only.as_ref().is_some_and(|ps| !ps.contains(&(pair as u16))) {
                continue;
            }
            if let Carried::Fails(why) = vertex_carries_pair(v, x, y) {
                // WHICH PAIR AND WHICH FAULT. The refusal's coordinate is
                // the vertex (50.9 contracts there), so the pair and the
                // fault ride the journal instead: a voice whose rotor is
                // wrong and a voice reading a different frame than this
                // court replays against produced the same line, and on the
                // fleet at epoch 11 that line was the commonest refusal
                // the court made.
                let at: String =
                    address.iter().take(2).map(|b| format!("{b:02x}")).collect();
                eprintln!(
                    "  court vertex body {at}… pair {pair} of {}: {why}",
                    family.len()
                );
                return Some(Refusal {
                    reason: REFUSE_VERTEX_BODY,
                    at: sentence::At::Vertex { address: address.clone(), chart: class.clone() },
                });
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
    let (tr, tc) = element::cell_of(&element::carry_center(versor, r, c)?)?;
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
/// sections, read by the closure check. `REFUSE_EDGE` at the face's two
/// addresses when the union has no versor (the speaker claimed an edge
/// where the pair is not rigid) or the transport is not the one measured.
pub(crate) fn edges_rederive_family(f: &Form, family: &[(Grid, Grid)]) -> Option<Refusal> {
    for face in &f.faces {
        let Some(edge) = &face.edge else {
            continue;
        };
        let (Some(vi), Some(vj)) = (f.vertices.get(face.i as usize), f.vertices.get(face.j as usize)) else {
            continue;
        };
        let (
            Vertex::Word { address: ai, derivation: di, contact: None, .. },
            Vertex::Word { address: aj, derivation: dj, contact: None, .. },
        ) = (vi, vj)
        else {
            continue;
        };
        let (Some(mi), Some(mj)) = (vi.acting(), vj.acting()) else {
            continue;
        };
        let (ci, cj) = (&mi.carriage, &mj.carriage);
        let (class_i, class_j) = (&mi.chart, &mj.chart);
        let (ri, rj) = (di.rotor(), dj.rotor());
        let refused = || Refusal { reason: REFUSE_EDGE, at: sentence::At::Face(ai.clone(), aj.clone()) };
        let Some(expected) = element::conformal_of(&edge.word, &edge.carriage) else {
            return Some(refused());
        };
        let (Some(v_i), Some(v_j)) =
            (element::conformal_of_rotor(&ri, ci), element::conformal_of_rotor(&rj, cj))
        else {
            return Some(refused());
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
                return Some(refused());
            };
            let Some(v_ij) = crate::transducer::blade::versor_between(
                &union_shape(a_i, a_j),
                &union_shape(&images[bi], &images[bj]),
            ) else {
                return Some(refused());
            };
            let Some(inv) = v_i.versor_inverse() else {
                return Some(refused());
            };
            if !element::same_versor(&inv.geo(&v_ij), &expected) {
                return Some(refused());
            }
        }
    }
    None
}

/// 23.1 step 7 as ruled (50.5): a remainder cell stands only where a demand
/// vertex named it — unspoken, and its observed hue the one the demand
/// carries — and a sentence with any other remainder does not close the
/// family. The undeclared cells, read as content: empty when every
/// remainder cell is declared.
pub(crate) fn undeclared_remainder(remainders: &[Vec<(i64, i64, u8, u8)>], f: &Form) -> Vec<(i64, i64, u8, u8)> {
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
    let mut out: Vec<(i64, i64, u8, u8)> = remainders
        .iter()
        .flatten()
        .copied()
        .filter(|&(r, c, spoken, observed)| {
            spoken != 0 || !declared.get(&cell_key(r, c)).is_some_and(|hues| hues.contains(&observed))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}
