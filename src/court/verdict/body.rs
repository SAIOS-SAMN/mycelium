// 23.1 steps 3–4, 50.2 G steps 2–4 — THE RECEIVER ON THE COURT'S COPY OF
// THE FAMILY: every word vertex reconstructs its body, every edge
// re-derives in the stabilizer, and the remainder is what the demands name.
//
//! Nothing here is trusted from the tape. A vertex is carried onto the
//! court's own copy of each pair and every landed lamina is read; an edge
//! is re-measured on the union of the two bodies; a remainder link stands
//! only where a demand vertex said it would (50.5: valid, not closed). Each
//! refusal names its coordinate as content (50.9): the vertex with its
//! chart, the face's two addresses, the chart with the links read.

use super::*;
use crate::transducer::family::{perceive, Body, Grid, Lamina};

/// What one vertex does on one pair of the family.
pub(crate) enum Carried {
    /// No body of the vertex's chart on this pair: a section elsewhere.
    Silent,
    /// Every lamina of every body of its chart lands on its image with the
    /// gauged hue, and every touched image body is reached whole.
    Carries,
    /// A lamina landed off its image, on the wrong hue, or an image body
    /// was reached in part — and WHICH of those, because they are five
    /// different faults, and a `REFUSE_VERTEX_BODY` that said only that one
    /// of them happened would leave the journal unable to distinguish a
    /// voice whose rotor is wrong from a voice reading a different frame
    /// than the court replays against (rule 11).
    ///
    /// AND WHICH MEMBER FAILED. `At::Vertex` carries a chart under 50.9's
    /// grammar, and it is the chart of the member that failed — never the
    /// vertex's representative member, which would refuse a vertex acting
    /// on two charts at whichever chart the trie orders first (D10 item 13).
    Fails(&'static str, stalk::Invariant),
}

/// What a vertex failed at, for the journal. Coordinates, not a verdict.
pub(crate) const FAIL_NO_VERSOR: &str = "the rotor and carriage lift to no versor";
pub(crate) const FAIL_NO_CARRY: &str = "a lamina of its body carries nowhere";
/// THREE FAULTS, NOT ONE: a carried centre the sensor cannot convert to a
/// lamina at all, a lamina at a negative coordinate, and a lamina past the
/// image's extent say different things about a rule — the first that the
/// transport left the sensor's own geometry, the second that the placement
/// is wrong, the third that the scale or the extent is — and a court that
/// reported one string for all three would tell a voice nothing it can
/// contract on (50.9, rule 11).
pub(crate) const FAIL_NO_LAMINA: &str = "a carried centre is not a lamina of the sensor";
pub(crate) const FAIL_BEFORE_FRAME: &str = "a lamina lands before the image frame's origin";
pub(crate) const FAIL_PAST_FRAME: &str = "a lamina lands past the image frame's extent";
pub(crate) const FAIL_HUE: &str = "a lamina lands on the wrong hue";
pub(crate) const FAIL_PART: &str = "an image body is reached in part";
/// The vertex's gauge names two images for one hue, so it is not a hue map
/// and the lamina has no predicted hue to be right or wrong about. Read by
/// the first matching entry of the listing (rule 15) it would be
/// indistinguishable from `FAIL_HUE`.
pub(crate) const FAIL_GAUGE_AMBIGUOUS: &str = "the gauge names two images for one hue";

/// The bodies of `x` on the vertex's chart, read by the chart's key.
fn fiber_on<'a>(bodies: &'a [Body], class: &stalk::Invariant) -> Vec<&'a Body> {
    let key = class.key();
    bodies.iter().filter(|b| b.moduli.key() == key).collect()
}

/// A section's site, written out: the key a landed lamina is filed under.
fn site_key(r: i64, c: i64) -> Vec<u8> {
    let mut k = Vec::with_capacity(16);
    k.extend_from_slice(&r.to_le_bytes());
    k.extend_from_slice(&c.to_le_bytes());
    k
}

/// 50.1 step 1 on one pair: `(V · W · γ) · B_in = B_out` under exact
/// equality lamina by lamina, for every body of the vertex's chart on `x`.
pub(crate) fn vertex_carries_pair(
    vertex: &Vertex,
    x: &Grid,
    y: &Grid,
    pair: usize,
    sections: &Keyed<Vec<u16>>,
) -> Carried {
    let Vertex::Word { derivation, contact, acts, .. } = vertex else {
        return Carried::Silent;
    };

    let rotor = derivation.rotor();
    // A contact vertex is placed by its orbit, not its carriage; the
    // orbit's own replay (`REFUSE_ORBIT_STILL`) is its reconstruction.
    if contact.is_some() {
        return Carried::Silent;
    }
    let bodies = perceive(x);
    let mut landed: Keyed<()> = Keyed::new();
    // THE COURT REPLAYS EVERY MEMBER, each against the bodies on its OWN
    // chart, by the carriage and the gauge it acts by there. Replaying one
    // member for a vertex that carries several would pass a sentence on the
    // strength of the part of it the court happened to read first.
    //
    // A member whose chart this pair carries nothing on is silent, not a
    // fault; a vertex silent on every member is `Silent` exactly as one with
    // an empty fiber was.
    let mut spoke = false;
    // The chart every refusal below names is THE MEMBER'S OWN, so the court
    // refuses at the coordinate that actually failed.
    for (_, member) in acts.scoped() {
        if let Some(mps) =
            crate::transducer::family::member_section_key(vertex, &member.chart).and_then(|mk| sections.get(&mk))
        {
            if !mps.contains(&(pair as u16)) {
                continue;
            }
        }
        let at = || member.chart.clone();
        let Some(versor) = element::conformal_of_rotor(&rotor, &member.carriage) else {
            return Carried::Fails(FAIL_NO_VERSOR, at());
        };
        let fiber = fiber_on(&bodies, &member.chart);
        if fiber.is_empty() {
            continue;
        }
        spoke = true;
        for body in fiber {
            for &(r, c, k, h) in &body.laminae {
                let Some(p) = element::carry_center(&versor, r, c, k) else {
                    return Carried::Fails(FAIL_NO_CARRY, at());
                };

                // A centre the sensor cannot convert is not a lamina at all
                // (W6) — the transport left the sensor's own geometry — and
                // a centre carried into another frame is no lamina of the
                // grid this pair is replayed against.
                let Some((tr, tc, tk)) = element::site_of(&p) else {
                    return Carried::Fails(FAIL_NO_LAMINA, at());
                };
                if tk != k {
                    return Carried::Fails(FAIL_NO_LAMINA, at());
                }
                // Before the origin, and past the extent, are two different
                // statements about the rule: the first about its placement,
                // the second about its scale.
                let (Ok(ur), Ok(uc)) = (usize::try_from(tr), usize::try_from(tc)) else {
                    return Carried::Fails(FAIL_BEFORE_FRAME, at());
                };
                let Some(&observed) = y.get(ur).and_then(|row| row.get(uc)) else {
                    return Carried::Fails(FAIL_PAST_FRAME, at());
                };
                // The gauge is asked for ONE image and refuses where it
                // names two: a vertex whose listing sends one hue to two is
                // not carrying its body by a hue map at all, and reading the
                // first entry of the list made that a silent choice.
                let Some(want) = crate::transducer::sentence::hue_of(&member.gauge, h) else {
                    return Carried::Fails(FAIL_GAUGE_AMBIGUOUS, at());
                };
                if observed != want {
                    return Carried::Fails(FAIL_HUE, at());
                }
                landed.insert(site_key(tr, tc), ());
            }
        }
    }
    if !spoke {
        return Carried::Silent;
    }
    // Every lamina of every image body it touched is reached: the image is
    // a body, not a part of one. THE UNION OF WHAT EVERY MEMBER LANDED is
    // what this asks about — two members whose images meet in one image body
    // cover it between them, and asking per member would call that coverage
    // a part.
    for image in perceive(y) {
        let touched = image.laminae.iter().any(|&(r, c, _, _)| landed.contains(&site_key(r, c)));
        if touched && !image.laminae.iter().all(|&(r, c, _, _)| landed.contains(&site_key(r, c))) {
            // The image body is reached in part BY THE FIBRE AS A WHOLE, so the
            // chart named is the image's own: no one member is at fault and the
            // court says which image it is, which is the coordinate 50.9 wants.
            return Carried::Fails(FAIL_PART, image.moduli.clone());
        }
    }
    Carried::Carries
}

/// Does the form, spoken on this pair, cover every one of `laminae` with
/// the hue it carries — by a word vertex's carriage or a demand's claim?
fn form_covers_laminae(
    f: &Form,
    pair: usize,
    x: &Grid,
    laminae: &[Lamina],
) -> bool {
    let mut covered: Keyed<u8> = Keyed::new();
    for (_, _, _, demand_links) in f.demand_vertices() {
        for &(r, c, _, after) in demand_links {
            covered.insert(site_key(r, c), after);
        }
    }
    let bodies = perceive(x);
    for v in &f.vertices {
        let Vertex::Word { acts, derivation, contact, .. } = v else { continue; };
        if contact.is_some() { continue; }
        let only = crate::transducer::family::transport_class(v)
            .and_then(|tc| f.sections.get(&tc.key()).cloned());
        if only.as_ref().is_some_and(|ps| !ps.contains(&(pair as u16))) {
            continue;
        }
        let rotor = derivation.rotor();
        for (_, member) in acts.scoped() {
            if let Some(mps) =
                crate::transducer::family::member_section_key(v, &member.chart).and_then(|mk| f.sections.get(&mk))
            {
                if !mps.contains(&(pair as u16)) {
                    continue;
                }
            }
            let Some(versor) = element::conformal_of_rotor(&rotor, &member.carriage) else { continue; };
            for b in fiber_on(&bodies, &member.chart) {
                for &(r, c, k, h) in &b.laminae {
                    let Some(p) = element::carry_center(&versor, r, c, k) else { continue; };
                    let Some((tr, tc, tk)) = element::site_of(&p) else { continue; };
                    if tk != k {
                        continue;
                    }
                    let Some(want) = crate::transducer::sentence::hue_of(&member.gauge, h) else { continue; };
                    covered.insert(site_key(tr, tc), want);
                }
            }
        }
    }
    laminae.iter().all(|&(r, c, _, h)| covered.get(&site_key(r, c)).is_some_and(|&w| w == h))
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
        // A VERTEX IS ASKED FOR THE PAIRS ITS RULE CLAIMS. A class whose
        // placement varies across the bundle is a section (18.2), and the
        // rule names the pairs it was measured on; asking it for the rest
        // is asking it to answer for a claim it did not make, and checking
        // every word vertex against every pair would refuse a section for
        // being one.
        //
        // A class the rule declares no section for is claimed whole and is
        // checked whole, which is what it says of itself.
        let only = crate::transducer::family::transport_class(v)
            .and_then(|tc| f.sections.get(&tc.key()).cloned());
        for (pair, (x, y)) in family.iter().enumerate() {
            if only.as_ref().is_some_and(|ps| !ps.contains(&(pair as u16))) {
                continue;
            }
            if let Carried::Fails(why, chart) = vertex_carries_pair(v, x, y, pair, &f.sections) {
                if why == FAIL_PART {
                    let uncovered = perceive(y).into_iter().any(|image| {
                        if image.moduli == chart {
                            !form_covers_laminae(f, pair, x, &image.laminae)
                        } else {
                            false
                        }
                    });
                    if !uncovered {
                        continue;
                    }
                }
                // WHICH PAIR AND WHICH FAULT. The refusal's coordinate is
                // the vertex (50.9 contracts there), so the pair and the
                // fault ride the journal instead: a voice whose rotor is
                // wrong and a voice reading a different frame than this
                // court replays against would otherwise produce one line.
                let at: String =
                    address.iter().take(2).map(|b| format!("{b:02x}")).collect();
                let class_sec = only.is_some();
                let member_sec = crate::transducer::family::member_section_key(v, &chart)
                    .is_some_and(|mk| f.sections.get(&mk).is_some());
                eprintln!(
                    "  court vertex body {at}… pair {pair} of {}: {why} [class_sec: {class_sec}, member_sec: {member_sec}]",
                    family.len()
                );
                return Some(Refusal {
                    reason: REFUSE_VERTEX_BODY,
                    at: sentence::At::Vertex { address: address.clone(), chart },
                });
            }
        }
    }
    None
}

/// The body of `y` the image of `body`'s anchor lamina falls in, under
/// `versor`. The anchor is the least lamina in the body's own order — a
/// coordinate of the body, never a choice among its laminae.
fn image_body(body: &Body, versor: &Versor, images: &[Body]) -> Option<usize> {
    let [(r, c, k, _), ..] = body.laminae.as_slice() else {
        return None;
    };
    let (r, c, k) = (*r, *c, *k);
    let (tr, tc, tk) = element::site_of(&element::carry_center(versor, r, c, k)?)?;
    images.iter().position(|b| b.laminae.iter().any(|&(br, bc, bk, _)| (br, bc, bk) == (tr, tc, tk)))
}

fn union_shape(a: &Body, b: &Body) -> crate::transducer::blade::Shape {
    let mut pos: Vec<(i64, i64, i64)> =
        a.laminae.iter().chain(b.laminae.iter()).map(|&(r, c, k, _)| (r, c, k)).collect();
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
        let (ri, rj) = (di.rotor(), dj.rotor());
        let refused = || Refusal { reason: REFUSE_EDGE, at: sentence::At::Face(ai.clone(), aj.clone()) };
        // EVERY MEMBER PAIR OF THE FACE, each re-measured on ITS OWN two
        // bodies (D10 item 8). The edge is measured on the union of two bodies,
        // so a face relating several pairs carries an edge at each, and this
        // read one member of each end and re-derived that pair's edge for all
        // of them. A pair with no edge is a remainder edge there and there is
        // nothing to re-derive.
        for (pk, place) in face.at.scoped() {
        let Some(edge) = &place.edge else {
            continue;
        };
        let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
            continue;
        };
        let (Some(class_i), Some(class_j)) = (vi.chart_keyed(&ik), vj.chart_keyed(&jk)) else {
            continue;
        };
        let (Some(mi), Some(mj)) = (vi.acting_on(&class_i), vj.acting_on(&class_j)) else {
            continue;
        };
        let (class_i, class_j) = (&class_i, &class_j);
        let (ci, cj) = (&mi.carriage, &mj.carriage);
        let Some(expected) = element::conformal_of(&edge.word, &edge.carriage) else {
            return Some(refused());
        };
        let (Some(v_i), Some(v_j)) =
            (element::conformal_of_rotor(&ri, ci), element::conformal_of_rotor(&rj, cj))
        else {
            return Some(refused());
        };
        for (pair, (x, y)) in family.iter().enumerate() {
            // A FACE IS ASKED FOR THE PAIRS ITS WORD WAS MEASURED ON. 21
            // step 2 keeps every word the bundle spoke between two ends as
            // a face of its own, each a section over the pairs it holds on
            // (18.2: two carriages on one class are two sections), and
            // re-deriving every face against every pair would refuse each
            // section on the pairs it never claimed — the fault the class
            // path above refuses the same way.
            //
            // A face nothing sectioned claims them all and is checked
            // whole, which is what it says of itself.
            if !face.measured_on(pair) {
                continue;
            }
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
    }
    None
}

/// 23.1 step 7 as ruled (50.5): a remainder link stands only where a demand
/// vertex named it — unspoken, and its observed hue the one the demand
/// carries — and a sentence with any other remainder does not close the
/// family. The undeclared links, read as content: empty when every
/// remainder link is declared.
pub(crate) fn undeclared_remainder(remainders: &[Vec<(i64, i64, u8, u8)>], f: &Form) -> Vec<(i64, i64, u8, u8)> {
    let mut declared: Keyed<Vec<u8>> = Keyed::new();
    for (_, _, _, links) in f.demand_vertices() {
        for &(r, c, _, after) in links {
            let k = site_key(r, c);
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
            spoken != 0 || !declared.get(&site_key(r, c)).is_some_and(|hues| hues.contains(&observed))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}
