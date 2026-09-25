// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 8, 17.7 — THE WIRE: binary frames, the MTU budget, and the four-lane
//! stratum multiplex. A frame carries a body on exactly one polytonic lane
//! and is transport only — the body is opaque bytes here.

/// The MTU body budget: the wire's hard cap. A body exceeding it is refused
/// (fragmentation is the caller's concern; the wire never silently truncates).
pub const BODY_BUDGET: usize = 768 * 1024;

/// Frame-integrity digest of served bytes (corpus files, signed manifests).
/// Not a key of mathematical structure.
#[expect(
    clippy::disallowed_methods,
    reason = "law 2 — the object is the frame's bytes, and frame integrity is a property of those bytes and of no structure they carry"
)]
pub fn integrity_hash<'a, I>(chunks: I) -> [u8; 32]
where
    I: IntoIterator<Item = &'a [u8]>,
{
    let mut h = blake3::Hasher::new();
    for c in chunks {
        h.update(c);
    }
    *h.finalize().as_bytes()
}

/// Bytes one nerve act may carry.
///
/// A WIRE FRAME IS A TRANSPORT CONSTRUCT, NOT A LOCAL MEMORY CACHE (the
/// wire-MTU ruling, 2026-09-11). This was `spend_unit() * 1024`, and
/// `spend_unit()` is `reserve_max()/8`, read off the host's free memory at
/// boot: three on an eight-core box, so the frame clamped to its 4 KiB
/// floor. Two nodes of the same mesh therefore disagreed about how large a
/// frame is, and whether a chain could be paged AT ALL depended on what
/// else the machine was doing.
///
/// Measured on the fleet at epoch 11: hypha a holds every receipt from 0
/// to 2000 and could serve none of `1111..1112` — two receipts of ~5.5 KiB
/// against a 4 KiB frame — so the back-fill stopped there and could never
/// pass it. Receipts on either side, 1.9 KiB, served fine.
///
/// The budget is now the same on every node of the mesh: one number, so a
/// page that one node can build another can read, and no node's memory
/// pressure changes what the protocol is. `BODY_BUDGET` remains the hard
/// ceiling the codec refuses past.
pub fn frame_budget() -> usize {
    64 * 1024
}

/// The four orthogonal polytonic lanes (𝒮 spatial, 𝒢 gauge, 𝒯 temporal,
/// 𝒽 harmonic/shell). A frame rides exactly one — never summed across lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stratum {
    Spatial,
    Gauge,
    Temporal,
    Harmonic,
}

impl Stratum {
    pub const ALL: [Stratum; 4] = [Stratum::Spatial, Stratum::Gauge, Stratum::Temporal, Stratum::Harmonic];

    pub fn as_u8(self) -> u8 {
        match self {
            Stratum::Spatial => 0,
            Stratum::Gauge => 1,
            Stratum::Temporal => 2,
            Stratum::Harmonic => 3,
        }
    }

    pub fn from_u8(b: u8) -> Option<Stratum> {
        match b {
            0 => Some(Stratum::Spatial),
            1 => Some(Stratum::Gauge),
            2 => Some(Stratum::Temporal),
            3 => Some(Stratum::Harmonic),
            _ => None,
        }
    }
}

/// A lane-tagged wire frame. Body is opaque transport bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub lane: Stratum,
    pub body: Vec<u8>,
}

impl Frame {
    /// `[lane:1][len:4 LE][body]`. `None` if the body exceeds the MTU budget.
    pub fn encode(&self) -> Option<Vec<u8>> {
        if self.body.len() > BODY_BUDGET {
            return None;
        }
        let mut out = Vec::with_capacity(5 + self.body.len());
        out.push(self.lane.as_u8());
        #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — a constant: the body length is at most BODY_BUDGET = 768 · 1024, refused above it three lines up")]
        out.extend_from_slice(&(self.body.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.body);
        // LD1: the frame's bytes, by the act its body opens with.
        if let Some(tag) = crate::anastome::dialogue::act_tag(&self.body) {
            crate::observe::note_frame(tag, out.len() as u64);
        }
        Some(out)
    }

    /// Decode one frame from the head of `bytes`; returns `(frame, bytes_read)`.
    /// `None` on a bad lane tag, a length over budget, or a truncated buffer.
    pub fn decode(bytes: &[u8]) -> Option<(Frame, usize)> {
        if bytes.len() < 5 {
            return None;
        }
        let lane = Stratum::from_u8(bytes[0])?;
        let len = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize;
        if len > BODY_BUDGET || bytes.len() < 5 + len {
            return None;
        }
        Some((Frame { lane, body: bytes[5..5 + len].to_vec() }, 5 + len))
    }
}

/// Multiplex a stream of lane-tagged frames onto one buffer; demultiplex reads
/// them back in order (transport, no reordering).
pub fn multiplex(frames: &[Frame]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    for f in frames {
        out.extend_from_slice(&f.encode()?);
    }
    Some(out)
}

pub fn demultiplex(mut bytes: &[u8]) -> Option<Vec<Frame>> {
    let mut out = Vec::new();
    while !bytes.is_empty() {
        let (f, n) = Frame::decode(bytes)?;
        out.push(f);
        bytes = &bytes[n..];
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_roundtrips_on_its_lane() {
        let f = Frame { lane: Stratum::Gauge, body: vec![1, 2, 3, 9] };
        let (g, n) = Frame::decode(&f.encode().unwrap()).unwrap();
        assert_eq!(f, g);
        assert_eq!(n, 9);
        assert_eq!(g.lane.as_u8(), 1);
    }

    #[test]
    fn an_over_budget_body_is_refused() {
        let f = Frame { lane: Stratum::Spatial, body: vec![0u8; BODY_BUDGET + 1] };
        assert!(f.encode().is_none(), "over MTU — refused, never truncated");
    }

    #[test]
    fn lanes_multiplex_and_demultiplex_in_order() {
        let fs = vec![
            Frame { lane: Stratum::Spatial, body: vec![10] },
            Frame { lane: Stratum::Temporal, body: vec![20, 21] },
            Frame { lane: Stratum::Harmonic, body: vec![] },
        ];
        let round = demultiplex(&multiplex(&fs).unwrap()).unwrap();
        assert_eq!(fs, round, "all four lanes remain distinct and ordered");
    }
}
