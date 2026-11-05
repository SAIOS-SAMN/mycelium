// AT1 — THE ATLAS'S CHART COORDINATES: a frame's spelling, and one ruled
// chart coordinate framed.

/// THE FRAME'S SPELLING (AT1 Q4): a length byte, then the minimal
/// big-endian bytes — unbounded, and ordered by (length, bytes), which is
/// the numeric order the carrier reads the succession by (law 2).
#[cfg(test)]
pub fn frame_spelling(k: u64) -> Vec<u8> {
    let mut out = Vec::new();
    crate::network::nerve::coord::put_frame(k, &mut out);
    out
}

