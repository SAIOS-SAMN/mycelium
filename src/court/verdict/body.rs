// 23.1 steps 3–4, 50.2 G steps 2–4 — THE RECEIVER ON THE COURT'S COPY OF
// THE FAMILY: every word vertex reconstructs its body, every edge
// re-derives in the stabilizer, and the remainder is what the demands name.
//
//! Nothing here is trusted from the tape. A vertex is carried onto the
//! court's own copy of each frame and every landed lamina is read; an edge
//! is re-measured on the union of the two bodies; a remainder link stands
//! only where a demand vertex said it would (50.5: valid, not closed). Each
//! refusal names its coordinate as content (50.9): the vertex with its
//! chart, the face's two addresses, the chart with the links read.

use super::*;
use crate::transducer::family::{on_grid, perceive, Body, Grid, Lamina};

/// What one vertex does at one frame of the family's one body.
pub(crate) enum Carried {
    /// No body of the vertex's chart at this frame: a section elsewhere.
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

/// ONE FRAME OF THE FAMILY, PERCEIVED ONCE (law 12): the bodies of `x`
/// filed by their chart, the images of `y`, and the image each base site
/// belongs to. Every reader of a frame — a vertex's replay, an edge's
/// re-measurement, the coverage of a part-reached image — reads this and
/// perceives nothing itself: the frame's bodies are one fact of the frame,
/// not of the vertex or the face asking about them.
pub(crate) struct Frame<'a> {
    pub(crate) y: &'a Grid,
    pub(crate) bodies: Vec<Body>,
    pub(crate) images: Vec<Body>,
    /// Chart key → the bodies standing on that chart, by index.
    by_chart: Keyed<Vec<usize>>,
    /// Base site → the image standing on it, by index.
    image_at: Keyed<usize>,
}

impl<'a> Frame<'a> {
    pub(crate) fn of(x: &Grid, y: &'a Grid) -> Frame<'a> {
        let bodies = perceive(x);
        let images = perceive(y);
        let mut by_chart: Keyed<Vec<usize>> = Keyed::new();
        for (i, b) in bodies.iter().enumerate() {
            let k = b.moduli.key();
            match by_chart.get_mut(k.as_bytes()) {
                Some(on) => on.push(i),
                None => {
                    by_chart.insert(k.as_bytes(), vec![i]);
                }
            }
        }
        let mut image_at: Keyed<usize> = Keyed::new();
        for (i, b) in images.iter().enumerate() {
            for l in &b.laminae {
                image_at.insert(l.site.key(), i);
            }
        }
        Frame { y, bodies, images, by_chart, image_at }
    }

    /// The bodies of `x` on one chart: a keyed read of the fiber.
    pub(crate) fn on_chart(&self, chart: &stalk::Invariant) -> Vec<&Body> {
        match self.by_chart.get(chart.key().as_bytes()) {
            Some(on) => on.iter().filter_map(|&i| self.bodies.get(i)).collect(),
            None => Vec::new(),
        }
    }

    /// The image of `y` standing on a base site, if one does.
    pub(crate) fn image_at(&self, site: &crate::transducer::base::Site) -> Option<&Body> {
        self.images.get(*self.image_at.get(&site.key())?)
    }
}

/// The family's frames, each perceived once, in the family's own order.
pub(crate) fn frames_of(family: &[(Grid, Grid)]) -> Vec<Frame<'_>> {
    family.iter().map(|(x, y)| Frame::of(x, y)).collect()
}

/// A section's site, written out: the key a landed lamina is filed under.
fn site_key(r: i64, c: i64) -> Vec<u8> {
    let mut k = Vec::with_capacity(16);
    k.extend_from_slice(&r.to_le_bytes());
    k.extend_from_slice(&c.to_le_bytes());
    k
}

/// 50.1 step 1 at one FRAME of the family's one body (51.8): `(V · W · γ) ·
/// B_in = B_out` under exact equality lamina by lamina, for every body of
/// the vertex's chart on the frame.
pub(crate) fn vertex_carries_frame(vertex: &Vertex, here: &Frame<'_>, frame: u64) -> Carried {
    let Vertex::Word { derivation, contact, acts, .. } = vertex else {
        return Carried::Silent;
    };

    let rotor = derivation.rotor();
    // A contact vertex is placed by its orbit, not its carriage; the
    // orbit's own replay (`REFUSE_ORBIT_STILL`) is its reconstruction.
    if contact.is_some() {
        return Carried::Silent;
    }
    let y = here.y;
    let mut landed: Keyed<()> = Keyed::new();
    // THE COURT REPLAYS EVERY MEMBER, each against the bodies on its OWN
    // chart, by the carriage and the gauge it acts by there. Replaying one
    // member for a vertex that carries several would pass a sentence on the
    // strength of the part of it the court happened to read first.
    //
    // A member whose chart this frame carries nothing on is silent, not a
    // fault; a vertex silent on every member is `Silent` exactly as one with
    // an empty fiber was.
    let mut spoke = false;
    // The chart every refusal below names is THE MEMBER'S OWN, so the court
    // refuses at the coordinate that actually failed.
    for (_, member) in acts.scoped() {
        // The member is asked at the frames it holds (AT1).
        if !member.stands_at(frame) {
            continue;
        }
        let at = || member.chart.clone();
        let Some(versor) = element::conformal_of_rotor(&rotor, &member.carriage) else {
            return Carried::Fails(FAIL_NO_VERSOR, at());
        };
        let fiber = here.on_chart(&member.chart);
        if fiber.is_empty() {
            continue;
        }
        spoke = true;
        for body in fiber {
            for l in &body.laminae {
                let h = l.hue;
                let Some(p) = element::carry_site_center(&versor, &l.site) else {
                    return Carried::Fails(FAIL_NO_CARRY, at());
                };

                // A centre the sensor cannot convert is not a lamina at all
                // (W6) — the transport left the sensor's own geometry — and
                // a centre carried into another frame is no lamina of the
                // grid this frame is replayed against.
                let Some(t) = element::site_at(&p) else {
                    return Carried::Fails(FAIL_NO_LAMINA, at());
                };
                let (Some((tr, tc)), true) = (on_grid(&t), same_frame(&t, &l.site)) else {
                    return Carried::Fails(FAIL_NO_LAMINA, at());
                };
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
    for image in &here.images {
        let landed_on = |l: &Lamina| on_grid(&l.site).is_some_and(|(r, c)| landed.contains(&site_key(r, c)));
        let touched = image.laminae.iter().any(landed_on);
        if touched && !image.laminae.iter().all(landed_on) {
            // The image body is reached in part BY THE FIBRE AS A WHOLE, so the
            // chart named is the image's own: no one member is at fault and the
            // court says which image it is, which is the coordinate 50.9 wants.
            return Carried::Fails(FAIL_PART, image.moduli.clone());
        }
    }
    Carried::Carries
}

/// Does the form, spoken at this frame, cover every one of `laminae` with
/// the hue it carries — by a word vertex's carriage or a demand's claim?
fn form_covers_laminae(
    f: &Form,
    frame: u64,
    at: &Frame<'_>,
    laminae: &[Lamina],
) -> bool {
    let mut covered: Keyed<u8> = Keyed::new();
    for (_, _, _, demand_links) in f.demand_vertices() {
        for &(r, c, _, after) in demand_links {
            covered.insert(site_key(r, c), after);
        }
    }
    for (_, v) in f.vertices.scoped() {
        let Vertex::Word { acts, derivation, contact, .. } = v else { continue; };
        if contact.is_some() { continue; }
        if !v.claims_frame(frame) {
            continue;
        }
        let rotor = derivation.rotor();
        for (_, member) in acts.scoped() {
            if !member.stands_at(frame) {
                continue;
            }
            let Some(versor) = element::conformal_of_rotor(&rotor, &member.carriage) else { continue; };
            for b in at.on_chart(&member.chart) {
                for l in &b.laminae {
                    let Some(t) = element::carry_site(&versor, &l.site) else { continue; };
                    let (Some((tr, tc)), true) = (on_grid(&t), same_frame(&t, &l.site)) else {
                        continue;
                    };
                    let Some(want) = crate::transducer::sentence::hue_of(&member.gauge, l.hue) else { continue; };
                    covered.insert(site_key(tr, tc), want);
                }
            }
        }
    }
    laminae.iter().all(|l| {
        on_grid(&l.site).is_some_and(|(r, c)| covered.get(&site_key(r, c)).is_some_and(|&w| w == l.hue))
    })
}

/// Do two sites stand in one frame — every coordinate off the grid axes
/// equal? A lamina carried into another frame is no lamina of the grid this
/// frame is replayed against.
fn same_frame(a: &crate::transducer::base::Site, b: &crate::transducer::base::Site) -> bool {
    a.n() == b.n()
        && a.axes()
            .iter()
            .zip(b.axes())
            .enumerate()
            .all(|(i, (x, y))| crate::transducer::base::is_grid_axis(i) || x == y)
}

/// 23.1 step 3 ALONG `e₃` (51.8): `REFUSE_VERTEX_BODY` at the vertex, with
/// its chart, for a word vertex that fails at any frame of the family's one
/// body; a vertex silent at every frame is a section nothing here
/// contradicts, and the closure check reads it.
///
/// Every rule vertex's support meets every FRAME it claims — the vertex
/// stands along the whole of `e₃` unless it is a section of some frames
/// only — and the claim is a keyed read of its members' frames at the frame's
/// own coordinate (AT1), never a walk of a list.
pub(crate) fn vertices_carry_frames(f: &Form, family: &[(Grid, Grid)]) -> Option<Refusal> {
    let frames = frames_of(family);
    for (vertex, v) in f.vertices.entries() {
        let Vertex::Word { address, .. } = v else {
            continue;
        };
        // A VERTEX IS ASKED FOR THE FRAMES ITS RULE CLAIMS. A class whose
        // placement varies along `e₃` is a section (18.2), and the rule
        // names the frames it was measured at; asking it for the rest is
        // asking it to answer for a claim it did not make, and checking
        // every word vertex at every frame would refuse a section for being
        // one.
        //
        // A class the rule declares no section for is claimed whole and is
        // checked whole, which is what it says of itself.
        let class_sec = v.acts().is_some_and(|a| a.scoped().any(|(_, m)| !m.frames.is_empty()));
        for (frame, at) in frames.iter().enumerate() {
            let Ok(frame) = u64::try_from(frame) else { continue };
            if !v.claims_frame(frame) {
                continue;
            }
            if let Carried::Fails(why, chart) = vertex_carries_frame(v, at, frame) {
                if why == FAIL_PART {
                    let uncovered = at.images.iter().any(|image| {
                        if image.moduli == chart {
                            !form_covers_laminae(f, frame, at, &image.laminae)
                        } else {
                            false
                        }
                    });
                    if !uncovered {
                        continue;
                    }
                }
                // WHICH FRAME AND WHICH FAULT. The refusal's coordinate is
                // the vertex (50.9 contracts there), so the frame and the
                // fault ride the journal instead: a voice whose rotor is
                // wrong and a voice reading a different frame than this
                // court replays against would otherwise produce one line.
                let at: String =
                    address.iter().take(2).map(|b| format!("{b:02x}")).collect();
                let member_sec = v.acting_all_on(&chart).iter().any(|m| !m.frames.is_empty());
                eprintln!(
                    "  court vertex body {at}… frame {frame} of {}: {why} [class_sec: {class_sec}, member_sec: {member_sec}]",
                    family.len()
                );
                // CV1 — THE VERTEX'S OWN VERSOR ON THE CHART, read by the one
                // reader (78.9): none lifted is P with the fault's name.
                let versor = match (v, v.acting_on(&chart)) {
                    (Vertex::Word { derivation, .. }, Some(acting)) if why != FAIL_NO_VERSOR => {
                        element::conformal_of_rotor(&derivation.rotor(), &acting.carriage)
                    }
                    _ => None,
                };
                let key = chart.key();
                return Some(Refusal::closure(
                    REFUSE_VERTEX_BODY,
                    sentence::At::vertex(vertex.clone(), chart.clone()),
                    sentence::Content::Holonomy { versor: versor.as_ref(), chart: Some(key.as_bytes()), proof: why },
                ));
            }
        }
    }
    None
}

/// The body of `y` the image of `body`'s anchor lamina falls in, under
/// `versor`: a keyed read of the frame's occupancy at the carried site. The
/// anchor is the least lamina in the body's own order — a coordinate of
/// the body, never a choice among its laminae.
fn image_body<'a>(body: &Body, versor: &Versor, at: &'a Frame<'_>) -> Option<&'a Body> {
    let [anchor, ..] = body.laminae.as_slice() else {
        return None;
    };
    at.image_at(&element::carry_site(versor, &anchor.site)?)
}

fn union_shape(a: &Body, b: &Body) -> crate::transducer::blade::Shape {
    let mut pos: Vec<crate::transducer::base::Site> =
        a.laminae.iter().chain(b.laminae.iter()).map(|l| l.site.clone()).collect();
    pos.sort_unstable();
    pos.dedup();
    crate::transducer::blade::shape_of(&pos)
}

/// 23.1 step 4, 50.3 ALONG `e₃` (51.8): at every FRAME where each end of a
/// carried edge has exactly one body of its chart, `V_ij` is re-measured
/// on the union of the two bodies and `Γ_ij = V_i⁻¹ · V_ij` must be the
/// transport the face carries. A frame with several bodies of one chart is
/// sections, read by the closure check. `REFUSE_EDGE` at the face's two
/// addresses when the union has no versor (the speaker claimed an edge
/// where the pair is not rigid) or the transport is not the one measured.
///
/// AN OVERLAP STANDS AT ONE FRAME (AT1, Q2): its edge is re-derived at the
/// frame its two charts name, read by coordinate. A frame-axis overlap joins
/// two frames and carries the motor its member's body took between them,
/// not a union edge; it is not re-measured here.
pub(crate) fn edges_rederive_frames(f: &Form, family: &[(Grid, Grid)]) -> Option<Refusal> {
    use crate::network::nerve::coord::ChartAt;
    let frames = frames_of(family);
    for (o, face) in f.faces.entries() {
        let (ChartAt::At(x), ChartAt::At(y)) = (&o.from, &o.to) else {
            continue;
        };
        if x.frame != y.frame {
            continue;
        }
        let (Some(vi), Some(vj)) = (f.vertices.at(&x.vertex), f.vertices.at(&y.vertex)) else {
            continue;
        };
        let (
            Vertex::Word { derivation: di, contact: None, .. },
            Vertex::Word { derivation: dj, contact: None, .. },
        ) = (vi, vj)
        else {
            continue;
        };
        let (ri, rj) = (di.rotor(), dj.rotor());
        // CV1 — BY CONTENT (78.9): where no versor carries what the edge
        // relates, the one reader reads P with the refusing reader's own
        // reason; where the claimed edge is false — it does not lift, or the
        // re-measured transport differs — the claim is I.
        let refused = |o: sentence::ClosureObstruction| Refusal {
            reason: REFUSE_EDGE,
            at: sentence::At::face(x.vertex.clone(), y.vertex.clone()),
            obstruction: o,
        };
        let none = |proof| sentence::read_obstruction(sentence::Content::Holonomy { versor: None, chart: None, proof });
        let false_claim = sentence::ClosureObstruction::Integrity;
        // THE MEMBER PAIR OF THE OVERLAP, re-measured on ITS OWN two bodies
        // (D10 item 8), each member the section standing at the overlap's
        // frame. An overlap with no edge is a remainder edge there and there
        // is nothing to re-derive.
        let Some(edge) = &face.place.edge else {
            continue;
        };
        let (class_i, class_j) = (&x.chart, &y.chart);
        let (Some(mi), Some(mj)) = (vi.acting_at(class_i, x.frame), vj.acting_at(class_j, y.frame)) else {
            continue;
        };
        let (ci, cj) = (&mi.carriage, &mj.carriage);
        let Some(expected) = element::conformal_of(&edge.word, &edge.carriage) else {
            return Some(refused(false_claim));
        };
        let (Some(v_i), Some(v_j)) =
            (element::conformal_of_rotor(&ri, ci), element::conformal_of_rotor(&rj, cj))
        else {
            return Some(refused(none("a vertex's rotor and carriage lift to no versor")));
        };
        // THE OVERLAP'S OWN FRAME, by its coordinate: a frame of the family
        // the court replays against, or none it can ask.
        if let Some(at) = usize::try_from(x.frame).ok().and_then(|k| frames.get(k)) {
            let fiber_i = at.on_chart(class_i);
            let fiber_j = at.on_chart(class_j);
            let ([a_i], [a_j]) = (fiber_i.as_slice(), fiber_j.as_slice()) else {
                continue;
            };
            let (Some(bi), Some(bj)) = (image_body(a_i, &v_i, at), image_body(a_j, &v_j, at)) else {
                return Some(refused(none("a body carries to no body of the image")));
            };
            let Some(v_ij) = crate::transducer::blade::versor_between(
                &union_shape(a_i, a_j),
                &union_shape(bi, bj),
            ) else {
                return Some(refused(none("no versor carries the union onto its image")));
            };
            // `ṽᵢ vᵢⱼ`, compared as a ray (78.32(1)).
            if !crate::transducer::cga::is_versor(&v_i) {
                return Some(refused(none("the vertex's versor has no inverse")));
            }
            if !element::same_versor(&v_i.reverse().geo(&v_ij), &expected) {
                return Some(refused(false_claim));
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
