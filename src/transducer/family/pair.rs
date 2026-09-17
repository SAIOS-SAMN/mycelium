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
    pub(crate) vertices: Vec<Vertex>,
    /// The support of each MEMBER — vertex and chart (D10) — because a
    /// face relates two BODIES and the support is a body's sites.
    pub(crate) supports: crate::network::nerve::Keyed<Vec<(i64, i64, i64)>>,
}

impl Scenery {
    pub(crate) fn new() -> Self {
        Scenery { vertices: Vec::new(), supports: crate::network::nerve::Keyed::new() }
    }

    /// THE ONE SITE THAT ADDS A VERTEX. It returns the vertex's coordinate,
    /// so a caller that needs to speak about what it placed is given it
    /// rather than computing it from a length. A vertex standing at one
    /// chart files its support under that member; a vertex acting on
    /// several charts is filed by the caller, one support per member.
    pub(crate) fn place(&mut self, vertex: Vertex, support: Vec<(i64, i64, i64)>) -> u16 {
        let ix = self.vertices.len() as u16;
        let member = vertex.face_chart().map(|c| member_key(ix, c.key().as_bytes()));
        self.vertices.push(vertex);
        if let (Some(m), false) = (member, support.is_empty()) {
            self.supports.insert(&m, support);
        }
        ix
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
/// loop is not evaluated, which is what `edge_holonomy` answers `None` for.
/// A rotor the seventeen cannot spell is 37.7's business and not an edge
/// to invent one for (rule 11).
pub(crate) fn meet_edge(
    va: &Vertex,
    sa: &Versor,
    vb: &Vertex,
    sb: &Versor,
) -> Result<crate::transducer::sentence::Transport, EdgeRefusal> {
    let gamma = sb.geo(&sa.versor_inverse().ok_or(EdgeRefusal::NoReading)?);
    let mut edge = edge_of(&gamma)?;
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
/// loops, and the form is built by the extraction.
pub(crate) struct Covered {
    pub(crate) demand: Demanded,
    pub(crate) cover: crate::transducer::family::cover::Cover,
    /// The frame the cover was read on; its pieces' supports stand there.
    pub(crate) frame: i64,
}

/// The links a deformation stands for: the union of both bodies' supports,
/// each with the hue before and after (0 where a body has no lamina).
pub(crate) fn delta_links(a: &Body, b: &Body) -> Vec<Link> {
    let mut before: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for &(r, c, f, h) in &a.laminae {
        before.insert(base_key(r, c, f), h);
    }
    let mut after: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for &(r, c, f, h) in &b.laminae {
        after.insert(base_key(r, c, f), h);
    }
    let mut links: Vec<Link> = Vec::new();
    for &(r, c, f, h) in &a.laminae {
        // A site the image does not hold is absent there, and hue 0 IS
        // absence in this encoding.
        let gone = match after.get(&base_key(r, c, f)) {
            Some(h2) => *h2,
            None => 0,
        };
        links.push((r, c, h, gone));
    }
    for &(r, c, f, h) in &b.laminae {
        if !before.contains(&base_key(r, c, f)) {
            links.push((r, c, 0, h));
        }
    }
    links.sort_unstable();
    links.dedup();
    links
}
