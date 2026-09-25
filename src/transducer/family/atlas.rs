// AT1's READING OF ONE FORM (r1, r2, r3, r6) — the table the atlas replaces,
// read off the form's own carriers at the percept's end: which vertices are
// charts, what the faces join, whether each orbit's holonomy recomposes from
// its faces, the member complex's cycle rank against the orbits held, and
// what the ruled overlap keys would cost against the form's address.

use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::form::atlas::{chart_coordinate, FormAtlas};

/// THE VERTEX'S RULED COORDINATE (AT1 Q1), its kind's tag first: a Word by
/// its transport class key, a Rebased by its base's, an Unfolded by its
/// measure, a Demand by (kind, chart), a Glued by its site, the Frame by the
/// tag alone. `None` where a Word or Rebased has no class.
fn vertex_coordinate(vertices: &[Vertex], v: &Vertex) -> Option<Vec<u8>> {
    use crate::network::nerve::Coordinate;
    let class = |w: &Vertex| transport_class(w).map(|t| t.key().encode().as_bytes().to_vec());
    let (tag, body): (u8, Vec<u8>) = match v {
        Vertex::Frame => (0, Vec::new()),
        Vertex::Word { .. } => (1, class(v)?),
        Vertex::Rebased { by, .. } => (2, class(vertices.get(usize::from(*by))?)?),
        Vertex::Unfolded { measure } => {
            (3, crate::transducer::sentence::unfolded_key(measure).encode().as_bytes().to_vec())
        }
        Vertex::Demand { kind, chart, .. } => {
            (4, crate::transducer::sentence::demand_key(kind, chart).encode().as_bytes().to_vec())
        }
        Vertex::Glued { .. } => (5, crate::transducer::sentence::vertex_bytes(v)),
    };
    let mut out = vec![tag];
    out.extend_from_slice(&body);
    Some(out)
}

/// THE FRAMES ONE MEMBER STANDS ON: its section where it was filed on fewer
/// than the family reached, every reached frame otherwise. `None` where the
/// vertex acts on no such chart.
fn member_frames(
    v: &Vertex,
    chart: &[u8],
    sections: &Keyed<Keyed<()>, coord::SectionAt>,
    reached: &[u16],
) -> Option<Vec<u16>> {
    let prefix = crate::transducer::sentence::Acting::chart_prefix_bytes(chart);
    let acting = v.acts()?.scoped().find(|(k, _)| k.starts_with(&prefix)).map(|(_, a)| a.clone())?;
    let mk = member_section_key(v, &acting.chart)?;
    Some(match sections.at(&coord::SectionAt::Member(Box::new(mk))) {
        Some(s) => crate::transducer::sentence::frames_of_section(s),
        None => reached.to_vec(),
    })
}

/// THE MEMBER COMPLEX'S NODES, as `orbits_of` names them: one per
/// `member_key(vertex, chart)`, numbered in the order they are met.
#[derive(Default)]
struct Nodes {
    of: Keyed<usize>,
    count: usize,
}

impl Nodes {
    fn of(&mut self, v: u16, chart: &[u8]) -> usize {
        let k = member_key(v, chart);
        match self.of.get(&k) {
            Some(&ix) => ix,
            None => {
                let ix = self.count;
                self.of.insert(&k, ix);
                self.count += 1;
                ix
            }
        }
    }
}

/// One face's boundary row, `∂(a → b) = b − a`; a face from a node to
/// itself bounds nothing and its row is empty.
fn edge_row(a: usize, b: usize) -> crate::core::lattice::SparseRow {
    match a == b {
        true => Vec::new(),
        false => vec![(a, num_bigint::BigInt::from(-1)), (b, num_bigint::BigInt::from(1))],
    }
}

/// ONE FORM READ AS THE TABLE IT IS (AT1's reading).
pub(super) fn read_form_atlas(
    form: &Form,
    sections: &Keyed<Keyed<()>, coord::SectionAt>,
    reached: &[u16],
) -> FormAtlas {
    let mut r = FormAtlas::default();
    // (r1)
    for v in &form.vertices {
        match v {
            Vertex::Word { .. } | Vertex::Rebased { .. } | Vertex::Glued { .. } => r.charts += 1,
            Vertex::Frame | Vertex::Demand { .. } | Vertex::Unfolded { .. } => r.non_charts += 1,
        }
        // (r7b) a Word vertex's members at the frames each stands on.
        if let Vertex::Word { acts, .. } = v {
            for (_, a) in acts.scoped() {
                if let Some(frames) = member_frames(v, a.chart.key().as_bytes(), sections, reached) {
                    r.chart_frames += frames.len() as u64;
                }
            }
        }
    }
    // (r2) and (r6), face by face; and the member complex (r3) the orbits
    // are built on — a node per (vertex, chart), a face relating no member
    // pair joining its vertices at the unnamed chart, as `orbits_of` builds it.
    let mut joined: Keyed<()> = Keyed::new();
    let mut nodes = Nodes::default();
    let mut boundary: Vec<crate::core::lattice::SparseRow> = Vec::new();
    for f in &form.faces {
        let (Some(vi), Some(vj)) = (form.vertices.get(usize::from(f.i)), form.vertices.get(usize::from(f.j))) else {
            continue;
        };
        if matches!(vi, Vertex::Frame) || matches!(vj, Vertex::Frame) {
            r.star_faces += 1;
        }
        if f.i == f.j {
            r.self_faces += 1;
        }
        let mut ends = f.i.to_be_bytes().to_vec();
        ends.extend_from_slice(&f.j.to_be_bytes());
        match joined.contains(&ends) {
            true => r.section_faces += 1,
            false => {
                joined.insert(&ends, ());
            }
        }
        let (ci_coord, cj_coord) = (vertex_coordinate(&form.vertices, vi), vertex_coordinate(&form.vertices, vj));
        let mut pairs = 0u64;
        for (pk, _) in f.at.scoped() {
            let Some((ci, cj)) = crate::transducer::sentence::read_face_pair_key(&pk) else { continue };
            pairs += 1;
            match ci == cj {
                true => r.pairs_one_chart += 1,
                false => r.pairs_no_chart += 1,
            }
            boundary.push(edge_row(nodes.of(f.i, &ci), nodes.of(f.j, &cj)));
            // (r6) one overlap per frame the pair stands on: the face's
            // section where it has one, the member's frames otherwise.
            // A pair whose frames read on neither member, or whose ends have
            // no ruled coordinate, is counted unread — never given the
            // family's frames in its place (rule 11).
            let frames = match &f.pairs {
                Some(ps) => Some(crate::transducer::sentence::frames_of_section(ps)),
                None => member_frames(vi, &ci, sections, reached).or_else(|| member_frames(vj, &cj, sections, reached)),
            };
            let (Some(frames), Some(xi), Some(xj)) = (frames, &ci_coord, &cj_coord) else {
                r.overlap_unread += 1;
                continue;
            };
            for k in frames {
                let mut key = Vec::new();
                chart_coordinate(xi, &ci, u64::from(k), &mut key);
                chart_coordinate(xj, &cj, u64::from(k), &mut key);
                r.overlaps += 1;
                r.overlap_bytes += key.len() as u64;
                if key.len() as u64 > r.overlap_max {
                    r.overlap_max = key.len() as u64;
                }
            }
        }
        if pairs == 0 {
            r.unnamed_faces += 1;
            boundary.push(edge_row(nodes.of(f.i, &[]), nodes.of(f.j, &[])));
        }
    }
    // (r3) the cycle rank `E − rank ∂` by the exact-rank reader, and each
    // orbit's holonomy recomposed along the faces it names.
    r.cycle_rank = (boundary.len() - crate::core::lattice::sparse_rank(&boundary)) as u64;
    r.orbits = form.orbits.len() as u64;
    for lp in &form.orbits {
        let path: Vec<usize> = lp.vertices.iter().map(|&v| usize::from(v)).collect();
        let crossed: Vec<usize> = lp.faces.iter().map(|&f| usize::from(f)).collect();
        let recomposed = crate::transducer::sentence::form_named_holonomy(&form.faces, &path, &crossed);
        match recomposed == lp.holonomy.normal_form() {
            true => r.holonomy_equal += 1,
            false => r.holonomy_differ += 1,
        }
    }
    r.address_bytes = crate::transducer::sentence::form_vertex_key(form).as_bytes().len() as u64;
    r
}
