/// The key of a site of one section — a grid's or a chain's, the sensor's
/// planar token (16.5). One producer with `sentence::section_site_key` (law
/// 12): the section's site at frame `0`, keyed by `Site::key`; a site of
/// the base is keyed by `Site::key` directly.
pub(crate) fn site_key(r: i64, c: i64) -> Vec<u8> {
    crate::transducer::sentence::section_site_key(r, c)
}

/// A MEMBER OF THE SCENE'S COMPLEX: a vertex and the chart it stands at
/// there (D10). The supports a pairing reads are keyed by this, not by the
/// vertex alone — a face exists because two BODIES' supports meet (21 step 2),
/// and a vertex covering several bodies on several charts is several occupants
/// of the sites they sit on.
///
/// Both halves are length-delimited so they read apart, and the vertex leads so
/// the key order is the vertex order, which is content. AT1 — THE VERTEX BY
/// ITS COORDINATE, never by its place in a list.
pub(crate) fn member_key(vertex: &crate::network::nerve::coord::VertexAt, chart: &[u8]) -> Vec<u8> {
    use crate::network::nerve::Coordinate;
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, vertex.encode().as_bytes());
    crate::transducer::address::put_key_bytes(&mut k, chart);
    k
}

// `section_key` (member ‖ carriage, 54.7) STOOD HERE AND RETIRED WITH THE
// SLOTS (AT1, ruled (a) on the chords): two pieces of one member are told
// apart by the PIECE-CHARTS — nucleus and shape at the member's frame —
// and two sections of one chart by the frames each holds, so no key widened
// by the carriage names either.

pub(crate) fn read_member_key(k: &[u8]) -> Option<(crate::network::nerve::coord::VertexAt, Vec<u8>)> {
    use crate::network::nerve::Coordinate;
    let mut p = 0usize;
    let v = crate::transducer::address::take_key_bytes(k, &mut p)?;
    let chart = crate::transducer::address::take_key_bytes(k, &mut p)?;
    Some((crate::network::nerve::coord::VertexAt::decode(&v).ok()?, chart))
}

/// The unordered pair of two members, and the two read back out.
pub(crate) fn member_pair_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, lo);
    crate::transducer::address::put_key_bytes(&mut k, hi);
    k
}

pub(crate) fn read_member_pair_key(k: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let mut p = 0usize;
    let a = crate::transducer::address::take_key_bytes(k, &mut p)?;
    let b = crate::transducer::address::take_key_bytes(k, &mut p)?;
    Some((a, b))
}
