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
fn edge_row(a: usize, b: usize) -> crate::core::lattice::SparseRow {
    match a == b {
        true => Vec::new(),
        false => vec![(a, num_bigint::BigInt::from(-1)), (b, num_bigint::BigInt::from(1))],
    }
}

/// A MEMBER WITH ITS FRAME FORGOTTEN: the node of the old member complex,
/// `(vertex, chart)`, written as the member's chart at frame 0; the Frame's
/// chart is its tag alone. A piece names its member.
fn forgotten(c: &ChartAt) -> ChartAt {
    match c {
        ChartAt::Frame => ChartAt::Frame,
        ChartAt::At(m) => ChartAt::at(m.vertex.clone(), m.chart.clone(), 0),
    }
}

/// One edge of the old member complex: the frames its pair's overlaps stand
/// at (empty for a frameless pair), and the face its reading takes.
struct OldEdge {
    frames: Keyed<()>,
    frameless: bool,
}

/// THE OLD BASIS, DEDUCED IN THE NEW ENGINE (78.46(5)(b)–(c), ruled (A)).
///
/// The old carrier was the member complex with frames forgotten: nodes
/// `(vertex, chart)`, one face per member pair. It is rebuilt here from the
/// coordinate list — every overlap projected to its member pair (a
/// frame-axis overlap or a chord between pieces projects to a member meeting
/// itself and adds no edge), every frameless pair a refused edge carrying
/// the place its two members make at their own frames — and its cycles are
/// read by the one builder, `orbits_of`, on that projection. Each is then
/// read by kind, deduced: its class keys by `orbit_classes`, the engine's
/// one reader, looked up by class key among the atlas's circuits (all of
/// them, or the across-frame ones for a frameless cycle). A reading taken
/// and dropped: nothing here is held.
fn read_old_basis(form: &Form, frameless: &[super::incidence::Frameless], r: &mut FormAtlas) {
    use crate::transducer::sentence::{FormFace, Producer};
    // The atlas's circuits' classes, all and across frames, by class key.
    let mut classes: Keyed<(), crate::network::nerve::coord::Holonomy> = Keyed::new();
    let mut across: Keyed<(), crate::network::nerve::coord::Holonomy> = Keyed::new();
    for lp in &form.orbits {
        let crosses = lp.steps.iter().any(|s| match (s.overlap.from.frame(), s.overlap.to.frame()) {
            (Some(a), Some(b)) => a != b,
            _ => false,
        });
        for row in form.orbit_classes(lp) {
            classes.put(&row.class, ());
            if crosses {
                across.put(&row.class, ());
            }
        }
    }
    // The projection: a clone whose faces are the old complex's edges, each
    // the first overlap of its pair in key order at the forgotten charts.
    let mut old = form.clone();
    old.faces = Keyed::new();
    let mut edges: Keyed<OldEdge, crate::network::nerve::coord::Overlap> = Keyed::new();
    for (o, face) in form.faces.entries() {
        let (a, b) = (forgotten(&o.from), forgotten(&o.to));
        if a == b {
            continue;
        }
        let key = crate::network::nerve::coord::Overlap { from: a.clone(), to: b.clone() };
        let back = key.reversed();
        let at = match (edges.at(&key).is_some(), edges.at(&back).is_some()) {
            (_, true) => back,
            _ => key,
        };
        let frame = o.frame();
        match edges.at_mut(&at) {
            Some(e) => {
                if let Some(k) = frame {
                    e.frames.insert(crate::transducer::sentence::frame_key(k), ());
                }
            }
            None => {
                let mut frames = Keyed::new();
                if let Some(k) = frame {
                    frames.insert(crate::transducer::sentence::frame_key(k), ());
                }
                edges.put(&at, OldEdge { frames, frameless: false });
                old.faces.put(&at, face.clone());
            }
        }
    }
    for refused in frameless {
        let (va, ca, fa) = &refused.a;
        let (vb, cb, fb) = &refused.b;
        let key = crate::network::nerve::coord::Overlap {
            from: ChartAt::at(va.clone(), ca.clone(), 0),
            to: ChartAt::at(vb.clone(), cb.clone(), 0),
        };
        if edges.at(&key).is_some() || edges.at(&key.reversed()).is_some() {
            continue;
        }
        // The place the two members make at their OWN frames, which is what
        // the old face carried where no frame held both.
        let ([ka, ..], [kb, ..]) = (fa.as_slice(), fb.as_slice()) else { continue };
        let place = form.face_place(&ChartAt::at(va.clone(), ca.clone(), *ka), &ChartAt::at(vb.clone(), cb.clone(), *kb), None);
        edges.put(&key, OldEdge { frames: Keyed::new(), frameless: true });
        if let Some(p) = place {
            old.faces.put(&key, FormFace::placed(p, Producer::Place));
        }
    }
    // Each cycle of the old complex, by kind.
    for lp in Form::orbits_of(&old.faces, &old.vertices) {
        r.old_cycles += 1;
        let steps: Vec<&OldEdge> = lp.steps.iter().filter_map(|s| edges.at(&s.overlap)).collect();
        let crosses_frameless = steps.iter().any(|e| e.frameless);
        // The frames every pair of the cycle stands at: the star's pairs
        // stand wherever their member does, so they narrow nothing.
        let mut shared: Option<Keyed<()>> = None;
        for (s, e) in lp.steps.iter().zip(&steps) {
            if matches!(s.overlap.from, ChartAt::Frame) || matches!(s.overlap.to, ChartAt::Frame) {
                continue;
            }
            shared = Some(match shared {
                None => e.frames.clone(),
                Some(held) => held.meet(&e.frames),
            });
        }
        let co_present = shared.is_none_or(|s| !s.is_empty());
        let rows = old.orbit_classes(&lp);
        match (crosses_frameless, co_present, rows.as_slice()) {
            (true, _, []) | (false, true, []) => r.old_unread += 1,
            (true, _, rows) => match rows.iter().all(|row| across.at(&row.class).is_some()) {
                true => r.old_frameless_across += 1,
                false => r.old_frameless_not += 1,
            },
            (false, false, _) => r.old_across += 1,
            (false, true, rows) => match rows.iter().all(|row| classes.at(&row.class).is_some()) {
                true => r.old_present += 1,
                false => r.old_lost += 1,
            },
        }
    }
    // A face with no member pair does not exist under the atlas: refused at
    // construction (Q-B2), so the column reads 0 by construction.
    r.old_slot = 0;
}

/// ONE FORM READ AS THE ATLAS IT IS (AT1's reading). `reached` is the
/// frames the family reached, which a member holding no frames of its own
/// is read at; `frameless` the named pairs no frame placed (78.46(5)(d)).
pub(super) fn read_form_atlas(form: &Form, reached: &[u64], frameless: &[super::incidence::Frameless]) -> FormAtlas {
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
    let mut boundary: Vec<crate::core::lattice::SparseRow> = Vec::new();
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
    // (r3) the cycle rank `E − rank ∂` by the exact-rank reader, and each
    // orbit's holonomy derived along the overlaps it names: `equal` where it
    // derives, `differ` where an overlap it names does not.
    r.cycle_rank = (boundary.len() - crate::core::lattice::sparse_rank(&boundary)) as u64;
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
    read_old_basis(form, frameless, &mut r);
    r
}
