// AT1 — THE ATLAS'S CHART COORDINATES: a frame's spelling, and one ruled
// chart coordinate framed.

/// THE FRAME'S SPELLING (AT1 Q4): a length byte, then the minimal
/// big-endian bytes — unbounded, and ordered by (length, bytes), which is
/// the numeric order the carrier reads the succession by (law 2).
pub fn frame_spelling(k: u64) -> Vec<u8> {
    let mut out = Vec::new();
    crate::network::nerve::coord::put_frame(k, &mut out);
    out
}

/// One ruled chart coordinate, framed: the vertex's coordinate, the chart
/// Invariant, the frame's spelling.
pub fn chart_coordinate(vertex: &[u8], chart: &[u8], frame: u64, out: &mut Vec<u8>) {
    let mut c = vertex.to_vec();
    crate::transducer::address::put_key_bytes(&mut c, chart);
    c.extend_from_slice(&frame_spelling(frame));
    crate::transducer::address::put_key_bytes(out, &c);
}
