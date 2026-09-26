// 21 step 2, 50.3, 50.8 — THE SCENERY OF A FORM AS IT IS BUILT: the
// vertices, the support each stands on by member, what carried each, and
// the edge a cover's meet carries.
//
//! The extraction (52.5) places vertices here — a chart's quotient, a
//! cover's carried piece, a demand — and reads the faces off the supports.
//! Nothing here pairs, ranks or searches.

use super::*;

/// THE SCENERY OF A FORM AS IT IS BUILT: the vertices, and — keyed by
/// each vertex's OWN coordinate, the index the tape carries as `FormFace.i`
/// and `.j` — the support each member stands on.
///
/// A support is not a property every vertex has: a Frame has none, a demand
/// has none here, a quotient and a cover piece have one per member. Keyed,
/// absence is absence — nothing to encode, and no `usize::MAX` reachable as
/// "no member here".
pub(crate) struct Scenery {
    /// The vertices BY THEIR COORDINATES (AT1): the atlas's own carrier.
    pub(crate) vertices: crate::transducer::sentence::Vertices,
    /// The support of each MEMBER — vertex and chart (D10) — because a
    /// face relates two BODIES and the support is a body's sites.
    pub(crate) supports: crate::network::nerve::Keyed<Vec<Site>>,
}

impl Scenery {
    pub(crate) fn new() -> Self {
        Scenery { vertices: crate::network::nerve::Keyed::new(), supports: crate::network::nerve::Keyed::new() }
    }

    /// THE ONE SITE THAT ADDS A VERTEX, at its own coordinate, which it
    /// returns; `None` where the vertex has none (rule 11) and nothing is
    /// placed. A vertex standing at one chart files its support under that
    /// member; a vertex acting on several charts is filed by the caller, one
    /// support per member.
    pub(crate) fn place(&mut self, vertex: Vertex, support: Vec<Site>) -> Option<crate::network::nerve::coord::VertexAt> {
        let at = vertex.at()?;
        let member = vertex.face_chart().map(|c| member_key(&at, c.key().as_bytes()));
        self.vertices.put(&at, vertex);
        if let (Some(m), false) = (member, support.is_empty()) {
            self.supports.insert(&m, support);
        }
        Some(at)
    }
}

/// THE EDGE OF A MEET (50.3, 13.9 step 7): the transition between two
/// carried pieces' OWN frames, `V_b ∘ V_a⁻¹`, split into the three
/// coordinates a `Transport` carries — the rotor as a word by A22's peel,
/// the placement and dilation as the carriage, and the hue transition
/// `σ_b ∘ σ_a⁻¹` as the gauge.
///
/// THE THREE ARE THE THREE LANES a class vector reads (42.6): the word is
/// `𝒽`, the carriage is `𝒮`, the gauge is `𝒢`. Dropping any of them makes
/// the loop above blind on that lane before it is classified.
///
/// A refusal is a remainder edge and says which (`EdgeRefusal`): that meet's
/// orbit is not evaluated, which is what `edge_holonomy` answers `None` for.
/// A rotor the seventeen cannot spell is 37.7's business and not an edge
/// to invent one for (rule 11).
pub(crate) fn meet_edge(
    va: &Vertex,
    sa: &Versor,
    supa: &crate::transducer::family::cover::Supports,
    vb: &Vertex,
    sb: &Versor,
    supb: &crate::transducer::family::cover::Supports,
    budget: &mut crate::physics::Budget,
) -> Result<crate::transducer::sentence::Transport, EdgeRefusal> {
    // `s_b s̃_a`, the relation between the two frames as a ray (78.32(1)).
    crate::transducer::cga::is_versor(sa).then_some(()).ok_or(EdgeRefusal::NoReading)?;
    let gamma = sb.geo(&sa.reverse());
    // A.11 — THE FRAME RIDES THE INTERFACE (56.5, ruled 2026-09-22).
    //
    // `gamma = s_b s_a⁻¹` relates the two frames' VERSORS, and this took
    // it bare. Both supports were in hand at the call site and discarded
    // there as `_`, so the two charts met as unoriented bodies and the
    // word `peel_versor` reads off `gamma` carried `ℓ = 1` on all 186
    // meets — a reading of the pipeline, not of the scene.
    //
    // ```text
    //     θ_x  = relative_pose(domain_x, codomain_x)     the piece's OWN pose
    //     g_ab = (θ_b · θ_a⁻¹) · γ
    // ```
    //
    // EACH PIECE IS COMPARED WITH ITSELF, not with the other. Comparing
    // `a`'s support to `b`'s asks whether two DIFFERENT pieces are
    // congruent — usually false, and no statement about frames when true.
    // A chart's frame is how that chart turned under its own carrying,
    // which is well defined however different the two pieces are.
    //
    // The relative rotor is taken, not an absolute gauge against the grid
    // axes: an absolute frame would need a global reference direction
    // chosen for the base, which is a generator nobody derived. `None` is
    // a piece no proper turn carries onto its own landing — a reflection,
    // a DILATED carry whose domain and codomain are similar but not
    // congruent, or a 45° relation whose half-angle is irrational — and
    // then `gamma` stands unframed rather than being rotated by an
    // invented turn (rule 11). Both outcomes are counted, so "the frame
    // never fired" and "the frame fired and was the identity" cannot read
    // the same on the line, which is the defect that let `ℓ = 1` pass as
    // a measurement of the scene.
    let own_turn = |s: &crate::transducer::family::cover::Supports, budget: &mut crate::physics::Budget| {
        // The pose over the piece's tube (FR1, 72.1c): the sensor's plane
        // where domain and codomain stand in one section, every axis where
        // either extends off it.
        let (d, c) = (
            crate::transducer::blade::shape_of(&s.domain).blade,
            crate::transducer::blade::shape_of(&s.codomain).blade,
        );
        if crate::transducer::blade::on_tube(&d, &c) {
            crate::transducer::family::cover::tally(|t| t.pose_tube += 1);
        }
        crate::transducer::blade::relative_pose(&d, &c, budget)
    };
    let framed = match (own_turn(supa, budget), own_turn(supb, budget)) {
        // `θ_b θ̃_a γ`: the framed relation as a ray (78.32(1)).
        (Ok(ta), Ok(tb)) => match crate::transducer::cga::is_versor(ta.rotor()).then(|| ta.rotor().reverse()) {
            Some(inv) => {
                // A frame either side of which is only fixed mod `π` makes
                // the composite open too, and it is counted apart: an
                // even moment cannot close a 180° turn (56.5), so this is
                // a projective class and not a determined rotor.
                let open = matches!(
                    (&ta, &tb),
                    (crate::transducer::blade::Pose::Open180(_), _)
                        | (_, crate::transducer::blade::Pose::Open180(_))
                );
                crate::transducer::family::cover::tally(|t| match open {
                    true => t.frame_open_180 += 1,
                    false => t.framed += 1,
                });
                tb.rotor().geo(&inv).geo(&gamma)
            }
            None => {
                crate::transducer::family::cover::tally(|t| t.unframed += 1);
                gamma
            }
        },
        _ => {
            crate::transducer::family::cover::tally(|t| t.unframed += 1);
            gamma
        }
    };
    let mut edge = edge_of(&framed)?;
    // AXIOM B — THE INTERFACE TRANSITION IS A GEODESIC TRANSVECTION BETWEEN
    // THE TWO FRAMES' OWN BASE POINTS (54.7, ruled 2026-09-21).
    //
    // `edge_of` reads a DISPLACEMENT out of `γ` and rebuilds it, and it
    // reads it through `cga::point`/`cga::base` — the flat conformal
    // embedding, `e_∞`-normalised, answering three scalars. So the geodesic
    // PLANE was destroyed and then re-synthesised from a triple, which is
    // rank-2 content reconstructed by assumption.
    //
    // Here the two frames are asked where they STAND — `V v₀ Ṽ` back on the
    // sheet — and the transition between those two points is
    // `R = 1 − v_b v_a`, whose generator IS the geodesic 2-blade
    // `v_a ∧ v_b`. No `e_∞` is formed and no coordinate is extracted.
    //
    // Where either frame does not stand on the sheet the reading is refused
    // rather than approximated, and `edge_of`'s own carriage stands (rule
    // 11): that is a frame this chart does not reach, not a zero transition.
    let geodesic = crate::transducer::cga::frame_base_point(sa)
        .zip(crate::transducer::cga::frame_base_point(sb))
        .and_then(|(pa, pb)| crate::transducer::cga::geodesic_transvection(&pa, &pb));
    match geodesic {
        Some(_r) => {
            crate::transducer::family::cover::tally(|t| t.geodesic += 1);
        }
        None => crate::transducer::family::cover::tally(|t| t.off_sheet += 1),
    }
    // Each body's own hue map at the chart its vertex stands at. A cover
    // vertex stands at exactly one, and a vertex standing at none carries no
    // map — which is the empty map, a map that moves no hue, and not a
    // default standing in for an absent reading (rule 11).
    let hue_at = |v: &Vertex| match v.face_chart() {
        Some(c) => crate::transducer::sentence::vertex_gauge_at(v, &c),
        None => Vec::new(),
    };
    edge.gauge = crate::transducer::sentence::relative_gauge(&hue_at(va), &hue_at(vb));
    Ok(edge)
}

/// A DEFORMATION AND THE COVER THAT READ IT, at the frame it was read on.
/// The two travel together because the demand's fate is not decided here:
/// whether it dissolves depends on what the FORM reads off the cover's
/// orbits, and the form is built by the extraction.
pub(crate) struct Covered {
    pub(crate) demand: Demanded,
    pub(crate) cover: crate::transducer::family::cover::Cover,
    /// The frame the cover was read on; its pieces' supports stand there.
    pub(crate) frame: i64,
}

/// The links a deformation stands for: the union of both bodies' supports,
/// each with the hue before and after (0 where a body has no compartment).
pub(crate) fn delta_links(a: &Body, b: &Body) -> Vec<Link> {
    let mut before: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for l in &a.compartments {
        before.insert(l.site.key(), l.hue);
    }
    let mut after: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for l in &b.compartments {
        after.insert(l.site.key(), l.hue);
    }
    let mut links: Vec<Link> = Vec::new();
    for l in &a.compartments {
        // A site the image does not hold is absent there, and hue 0 IS
        // absence in this encoding.
        let gone = match after.get(&l.site.key()) {
            Some(h2) => *h2,
            None => 0,
        };
        if let Some((r, c)) = on_grid(&l.site) {
            links.push((r, c, l.hue, gone));
        }
    }
    for l in &b.compartments {
        if let (false, Some((r, c))) = (before.contains(&l.site.key()), on_grid(&l.site)) {
            links.push((r, c, 0, l.hue));
        }
    }
    links.sort_unstable();
    links.dedup();
    links
}
