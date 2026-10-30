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
/// Measured on the development deployment at epoch 11: hypha a holds every receipt from 0
/// to 2000 and could serve none of `1111..1112` — two receipts of ~5.5 KiB
/// against a 4 KiB frame — so the back-fill stopped there and could never
/// pass it. Receipts on either side, 1.9 KiB, served fine.
///
/// The budget is the same on every node of the mesh, so a page that one
/// node can build another can read, and no node's memory pressure changes
/// what the protocol is. AND IT IS THE WIRE'S ONE CARRIER (PH1): a nerve act
/// carries what a frame carries, `BODY_BUDGET`, the protocol's coordinate
/// the codec refuses past and the wire contract states (H4) — one number
/// the mesh agrees on, not a second, smaller one chosen beside it.
pub fn frame_budget() -> usize {
    BODY_BUDGET
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

/// 78.152 TU-7 — THE LANE BYTE OF A FRAME THAT CARRIES A POINT: the lane's
/// own byte with this bit set, and the frame's last `TPoint::LEN` bytes are
/// the last FINAL point its speaker read. WIRE-CHANGE: a decoder older than
/// 78.152 reads no lane in the byte and refuses the frame whole.
pub const POINTED: u8 = 0x80;

/// A lane byte read: the lane, and whether the frame carries a point.
pub fn read_lane(byte: u8) -> Option<(Stratum, bool)> {
    Some((Stratum::from_u8(byte & !POINTED)?, byte & POINTED != 0))
}

/// A FRAME'S BODY AND ITS POINT: where the lane byte said the frame carries
/// one, the body's last `TPoint::LEN` bytes are the point and leave the body.
/// `None` where a pointed body is shorter than a point.
pub fn split_point(
    mut body: Vec<u8>,
    pointed: bool,
) -> Option<(Vec<u8>, Option<crate::chronometry::tuniversal::TPoint>)> {
    if !pointed {
        return Some((body, None));
    }
    let at = body.len().checked_sub(crate::chronometry::tuniversal::TPoint::LEN)?;
    let mut p = at;
    let point = crate::chronometry::tuniversal::TPoint::take(&body, &mut p)?;
    body.truncate(at);
    Some((body, Some(point)))
}

impl Frame {
    /// `[lane:1][len:4 LE][body]`. `None` if the body exceeds the MTU budget.
    pub fn encode(&self) -> Option<Vec<u8>> {
        self.encode_at(None)
    }

    /// 78.152 TU-7 — the frame carrying the last final point its speaker
    /// read: `[lane | POINTED][len:4 LE][body][point]`, the length counting
    /// the point; with none, `encode`'s frame. `None` if the body exceeds the
    /// MTU budget.
    pub fn encode_at(&self, point: Option<&crate::chronometry::tuniversal::TPoint>) -> Option<Vec<u8>> {
        if self.body.len() > BODY_BUDGET {
            return None;
        }
        let extra = match point {
            Some(_) => crate::chronometry::tuniversal::TPoint::LEN,
            None => 0,
        };
        let len = u32::try_from(self.body.len().checked_add(extra)?).ok()?;
        let mut out = Vec::with_capacity(5 + self.body.len() + extra);
        out.push(match point {
            Some(_) => self.lane.as_u8() | POINTED,
            None => self.lane.as_u8(),
        });
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&self.body);
        if let Some(p) = point {
            p.put(&mut out);
        }
        // LD1: the frame's bytes, by the act its body opens with.
        if let Some(tag) = crate::anastome::dialogue::act_tag(&self.body) {
            crate::observe::note_frame(tag, out.len() as u64);
        }
        Some(out)
    }

    /// Decode one frame from the head of `bytes`; returns `(frame, bytes_read)`.
    /// `None` on a bad lane tag, a length over budget, or a truncated buffer.
    /// A point the frame carries is read and set aside (`decode_at`).
    pub fn decode(bytes: &[u8]) -> Option<(Frame, usize)> {
        Self::decode_at(bytes).map(|(f, _, n)| (f, n))
    }

    /// One frame from the head of `bytes`, with the point it carries.
    pub fn decode_at(bytes: &[u8]) -> Option<(Frame, Option<crate::chronometry::tuniversal::TPoint>, usize)> {
        // The lane byte is the frame's own first coordinate (rule 15: a wire
        // tag's offset is content).
        let (&tag, _) = bytes.split_first()?;
        let (lane, pointed) = read_lane(tag)?;
        let len = u32::from_le_bytes(bytes.get(1..5)?.try_into().ok()?) as usize;
        if len > BODY_BUDGET.checked_add(crate::chronometry::tuniversal::TPoint::LEN)? {
            return None;
        }
        let body = bytes.get(5..5 + len)?.to_vec();
        let (body, point) = split_point(body, pointed)?;
        Some((Frame { lane, body }, point, 5 + len))
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

    /// 78.152 TU-7 — a frame carries its speaker's last final point behind
    /// its body, read back exactly; an older decoder's lane read refuses it.
    #[test]
    fn a_frame_carries_the_last_final_point() {
        use crate::chronometry::tuniversal::{Origin, TPoint};
        let point = TPoint { origin: Origin { genesis: [7; 32], epoch: 146 }, h: 40_961, lanes: [3, 0, 9, 1], k: 812 };
        let f = Frame { lane: Stratum::Temporal, body: vec![0x53, 1, 2] };
        let bytes = f.encode_at(Some(&point)).unwrap();
        assert_eq!(Stratum::from_u8(bytes[0]), None, "a pre-78.152 lane read refuses the pointed frame");
        let (g, at, n) = Frame::decode_at(&bytes).unwrap();
        assert_eq!((g, at, n), (f.clone(), Some(point), 5 + 3 + TPoint::LEN));
        let (h, none, _) = Frame::decode_at(&f.encode().unwrap()).unwrap();
        assert_eq!((h, none), (f, None), "a frame with no point reads with none");
    }
}
