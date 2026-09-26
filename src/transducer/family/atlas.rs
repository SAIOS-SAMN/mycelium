// AT1's READING OF ONE FORM (r1, r2, r3, r6) — the atlas read off the form's
// own carriers at the percept's end: which vertices are charts, what the
// overlaps join, whether each orbit's holonomy derives from its overlaps,
// the chart complex's cycle rank against the orbits held, and what the
// overlap keys cost against the form's address.
//
// The form IS the atlas since AT1 (i)+(ii): the vertices are keyed by their
// coordinates and the faces by their overlaps, so every count here is a read
// of a carrier and nothing is costed at a key it would have had.

use super::*;
use crate::network::nerve::coord::ChartAt;
use crate::network::nerve::{Coordinate, Keyed};
use crate::transducer::sentence::form::atlas::FormAtlas;

/// THE CHART COMPLEX'S NODES, as `orbits_of` names them: one per chart,
/// numbered in the order they are met.
#[derive(Default)]
struct Nodes {
    of: Keyed<usize, ChartAt>,
    count: usize,
}

impl Nodes {
    fn of(&mut self, c: &ChartAt) -> usize {
        match self.of.at(c) {
            Some(&ix) => ix,
            None => {
                let ix = self.count;
                self.of.put(c, ix);
                self.count += 1;
                ix
            }
        }
    }
}

/// One overlap's boundary row, `∂(a → b) = b − a`; an overlap from a chart
/// to itself bounds nothing and its row is empty.
fn edge_row(a: usize, b: usize) -> crate::core::span::Vector {
    match a == b {
        true => Vec::new(),
        false => vec![(a, num_bigint::BigInt::from(-1)), (b, num_bigint::BigInt::from(1))],
    }
}

// THE OLD-BASIS DEDUCTION STOOD HERE (78.46(5)(b)–(c)) AND RETIRED after
// its one reading post-A2 — 9 cycles, present 6, frameless 3 (across 2 /
// not 1), across 0, lost 0: it was the coordinator's acceptance instrument,
// not the minds' (77(1)).

/// ONE FORM READ AS THE ATLAS IT IS (AT1's reading). `reached` is the
/// frames the family reached, which a member holding no frames of its own
/// is read at.
pub(super) fn read_form_atlas(form: &Form, reached: &[u64]) -> FormAtlas {
    let mut r = FormAtlas::default();
    // (r1)
    for (_, v) in form.vertices.scoped() {
        match v {
            Vertex::Word { .. } | Vertex::Rebased { .. } | Vertex::Glued { .. } => r.charts += 1,
            Vertex::Frame | Vertex::Demand { .. } | Vertex::Unfolded { .. } => r.non_charts += 1,
        }
        // (r7b) a Word vertex's members at the frames each stands on.
        if let Vertex::Word { acts, .. } = v {
            for (_, a) in acts.scoped() {
                r.chart_frames += match a.frames.len() {
                    0 => reached.len() as u64,
                    n => n as u64,
                };
            }
        }
    }
    // (r2) and (r6), overlap by overlap; and the chart complex (r3) the
    // orbits are built on — a node per chart.
    let mut joined: Keyed<()> = Keyed::new();
    let mut nodes = Nodes::default();
    let mut boundary: Vec<crate::core::span::Vector> = Vec::new();
    for (o, _) in form.faces.entries() {
        let (a, b) = (o.from_vertex(), o.to_vertex());
        match (&o.from, &o.to) {
            (ChartAt::Frame, _) | (_, ChartAt::Frame) => r.star_faces += 1,
            (ChartAt::At(x), ChartAt::At(y)) => match x.chart == y.chart {
                true => r.pairs_one_chart += 1,
                false => r.pairs_no_chart += 1,
            },
        }
        if a == b {
            r.self_faces += 1;
        }
        // A SECTION FACE: a later overlap on an ordered pair of vertices
        // already joined — another member pair, or another frame.
        let mut ends = Vec::new();
        crate::transducer::address::put_key_bytes(&mut ends, a.encode().as_bytes());
        crate::transducer::address::put_key_bytes(&mut ends, b.encode().as_bytes());
        match joined.contains(&ends) {
            true => r.section_faces += 1,
            false => {
                joined.insert(&ends, ());
            }
        }
        boundary.push(edge_row(nodes.of(&o.from), nodes.of(&o.to)));
        // (r6) the overlap's own key.
        let bytes = o.encode().as_bytes().len() as u64;
        r.overlaps += 1;
        r.overlap_bytes += bytes;
        if bytes > r.overlap_max {
            r.overlap_max = bytes;
        }
    }
    // (r2) `unnamed_faces` and (r6) `overlap_unread` stay zero: an overlap is
    // its member pair, and a face with none is refused where it would be
    // made (AT1, ruled) — the slot artefact the row named has no carrier.
    //
    // (r3) the cycle rank `E − dim ∂`, the boundaries' span read as its
    // blade's grade, and each orbit's holonomy derived along the overlaps it
    // names: `equal` where it derives, `differ` where an overlap it names
    // does not.
    r.cycle_rank = (boundary.len() - crate::core::span::grade_of(&boundary)) as u64;
    r.orbits = form.orbits.len() as u64;
    let identity = crate::transducer::element::identity_versor();
    for lp in &form.orbits {
        match crate::transducer::sentence::orbit_word_on(&form.faces, &form.vertices, lp) {
            Some(_) => r.holonomy_equal += 1,
            None => r.holonomy_differ += 1,
        }
        // THE CIRCUIT'S KIND: across frames where a step's two charts stand
        // at two frames (a frame-axis overlap), within one frame otherwise;
        // TURNED where any member path's whole transport reads and is not the
        // identity ray, the reading `contact_circuits` takes (78.6).
        let across = lp.steps.iter().any(|s| match (s.overlap.from.frame(), s.overlap.to.frame()) {
            (Some(a), Some(b)) => a != b,
            _ => false,
        });
        let turned = crate::transducer::sentence::circuit::paths_carried(&form.faces, &form.vertices, lp).is_some_and(|c| {
            c.scoped().any(|(_, p)| p.composite().is_some_and(|m| !crate::transducer::element::same_versor(&m, &identity)))
        });
        match (across, turned) {
            (false, true) => r.within_turned += 1,
            (false, false) => r.within_flat += 1,
            (true, true) => r.across_turned += 1,
            (true, false) => r.across_flat += 1,
        }
    }
    r.address_bytes = crate::transducer::sentence::form_vertex_key(form).as_bytes().len() as u64;
    r
}
