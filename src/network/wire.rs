// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! WIRE — binary frame encoding, MTU budget, and 4-lane stratum multiplexing.
//! A frame carries a body on exactly one polytonic lane; the codec is exact and
//! reversible, and refuses any body over the MTU budget (a frame is transport,
//! never a place cognition reasons — the body is opaque bytes here).

/// The MTU body budget: 768 KiB. A body exceeding it is refused (fragmentation
/// is the caller's concern; the wire never silently truncates).
pub const BODY_BUDGET: usize = 768 * 1024;

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
        out.extend_from_slice(&(self.body.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.body);
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
