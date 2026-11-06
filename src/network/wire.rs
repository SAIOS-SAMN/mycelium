// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 8, 17.7 — THE WIRE: binary frames, the MTU budget, and the four-lane
//! stratum multiplex. A frame carries a body on exactly one polytonic lane
//! and is transport only — the body is opaque bytes here.

/// The MTU body budget: the wire's hard cap. A body exceeding it is refused
/// (fragmentation is the caller's concern; the wire never silently truncates).
/// REGISTERED: BODY_BUDGET
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
pub enum Lane {
    Spatial,
    Gauge,
    Temporal,
    Harmonic,
}

impl Lane {
    pub const ALL: [Lane; 4] = [Lane::Spatial, Lane::Gauge, Lane::Temporal, Lane::Harmonic];

    pub fn byte(self) -> u8 {
        match self {
            Lane::Spatial => 0,
            Lane::Gauge => 1,
            Lane::Temporal => 2,
            Lane::Harmonic => 3,
        }
    }

    pub fn of(b: u8) -> Option<Lane> {
        match b {
            0 => Some(Lane::Spatial),
            1 => Some(Lane::Gauge),
            2 => Some(Lane::Temporal),
            3 => Some(Lane::Harmonic),
            _ => None,
        }
    }
}

/// A lane-tagged wire frame. Body is opaque transport bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delimited {
    pub lane: Lane,
    pub body: Vec<u8>,
}

/// 78.152 TU-7 — THE LANE BYTE OF A FRAME THAT CARRIES A POINT: the lane's
/// own byte with this bit set, and the frame ends in the TRAILER of the last
/// FINAL point its speaker read: the point's wire bytes (its lanes exact
/// rationals, so of no fixed length), then their count as a u16 LE
/// (`TPoint::put_trailer`, ruled 2026-11-01). WIRE-CHANGE: a decoder older
/// than 78.152 reads no lane in the byte and refuses the frame whole.
/// FORMAT: lane byte flag bit.
pub const POINTED: u8 = 0x80;

/// A lane byte read: the lane, and whether the frame carries a point.
pub fn read_lane(byte: u8) -> Option<(Lane, bool)> {
    Some((Lane::of(byte & !POINTED)?, byte & POINTED != 0))
}

/// A FRAME'S BODY AND ITS POINT: where the lane byte said the frame carries
/// one, the body's trailer is the point and leaves the body. `None` where a
/// pointed body's trailer does not read whole.
pub fn split_point(
    mut body: Vec<u8>,
    pointed: bool,
) -> Option<(Vec<u8>, Option<crate::chronometry::tuniversal::TPoint>)> {
    if !pointed {
        return Some((body, None));
    }
    let (point, at) = crate::chronometry::tuniversal::TPoint::take_trailer(&body)?;
    body.truncate(at);
    Some((body, Some(point)))
}

impl Delimited {
    /// `[lane:1][len:4 LE][body]`. `None` if the body exceeds the MTU budget.
    pub fn encode(&self) -> Option<Vec<u8>> {
        self.encode_at(None)
    }

    /// 78.152 TU-7 — the frame carrying the last final point its speaker
    /// read: `[lane | POINTED][len:4 LE][body][point][point len:2 LE]`, the
    /// length counting the trailer; with none, `encode`'s frame. `None` if
    /// the body exceeds the MTU budget or the point passes its u16 count.
    pub fn encode_at(&self, point: Option<&crate::chronometry::tuniversal::TPoint>) -> Option<Vec<u8>> {
        if self.body.len() > BODY_BUDGET {
            return None;
        }
        let mut trailer = Vec::new();
        if let Some(p) = point {
            p.put_trailer(&mut trailer)?;
        }
        let len = u32::try_from(self.body.len().checked_add(trailer.len())?).ok()?;
        let mut out = Vec::with_capacity(5 + self.body.len() + trailer.len());
        out.push(match point {
            Some(_) => self.lane.byte() | POINTED,
            None => self.lane.byte(),
        });
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&self.body);
        out.extend_from_slice(&trailer);
        // LD1: the frame's bytes, by the act its body opens with.
        if let Some(tag) = crate::anastome::dialogue::act_tag(&self.body) {
            crate::observe::note_frame(tag, out.len() as u64);
        }
        Some(out)
    }

    /// Decode one frame from the head of `bytes`; returns `(frame, bytes_read)`.
    /// `None` on a bad lane tag, a length over budget, or a truncated buffer.
    /// A point the frame carries is read and set aside (`decode_at`).
    pub fn decode(bytes: &[u8]) -> Option<(Delimited, usize)> {
        Self::decode_at(bytes).map(|(f, _, n)| (f, n))
    }

    /// One frame from the head of `bytes`, with the point it carries.
    pub fn decode_at(bytes: &[u8]) -> Option<(Delimited, Option<crate::chronometry::tuniversal::TPoint>, usize)> {
        // The lane byte is the frame's own first coordinate (rule 15: a wire
        // tag's offset is content).
        let (&tag, _) = bytes.split_first()?;
        let (lane, pointed) = read_lane(tag)?;
        let len = u32::from_le_bytes(bytes.get(1..5)?.try_into().ok()?) as usize;
        if len > BODY_BUDGET.checked_add(crate::chronometry::tuniversal::TPoint::TRAILER_MAX)? {
            return None;
        }
        let body = bytes.get(5..5 + len)?.to_vec();
        let (body, point) = split_point(body, pointed)?;
        Some((Delimited { lane, body }, point, 5 + len))
    }
}

