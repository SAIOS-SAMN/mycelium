// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 8, 17.7 — THE WIRE: one envelope, a length and a body, and the MTU
//! budget. The body is opaque bytes here: an expression of anastome, which
//! the transport moves and never reads (MU-ENGINEERING §9.1). The envelope
//! holds no lane and no point: the lane byte was retired by the v0.5.0 recut,
//! and the TUniversal point a speaker read stands inside its expression
//! (`anastome::dialogue::expression`).

/// The MTU body budget: the most bytes one act may carry. An act exceeding
/// it is refused (fragmentation is the caller's concern; the wire never
/// silently truncates).
/// REGISTERED: BODY_BUDGET
pub const BODY_BUDGET: usize = 768 * 1024;

/// THE MOST BYTES ONE ENVELOPE DELIMITS: the act's budget and the tail an
/// expression closes with, at most a u16 count of bytes and the two bytes of
/// that count. The transport refuses a length past it before allocating.
/// FORMAT: an act's budget plus a u16-counted tail.
pub const DELIMITED_MAX: usize = BODY_BUDGET + u16::MAX as usize + 2;

/// The envelope's head: the body's length, a u32 LE.
/// FORMAT: u32 LE length.
pub const HEAD: usize = 4;

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

/// A wire frame: a body delimited by its length. The body is opaque
/// transport bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delimited {
    pub body: Vec<u8>,
}

/// The body length a head names, or `None` where it passes
/// [`DELIMITED_MAX`]: refused before a byte of the body is allocated.
pub fn delimited_len(head: [u8; HEAD]) -> Option<usize> {
    let len = usize::try_from(u32::from_le_bytes(head)).ok()?;
    (len <= DELIMITED_MAX).then_some(len)
}

impl Delimited {
    /// `[len:4 LE][body]`. `None` if the body passes [`DELIMITED_MAX`].
    pub fn encode(&self) -> Option<Vec<u8>> {
        if self.body.len() > DELIMITED_MAX {
            return None;
        }
        let len = u32::try_from(self.body.len()).ok()?;
        let mut out = Vec::with_capacity(HEAD + self.body.len());
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&self.body);
        // LD1: the frame's bytes, by the act its body opens with.
        if let Some(tag) = crate::anastome::dialogue::act_tag(&self.body) {
            crate::observe::note_frame(tag, out.len() as u64);
        }
        Some(out)
    }

    /// Decode one frame from the head of `bytes`; returns `(frame, bytes_read)`.
    /// `None` on a length past [`DELIMITED_MAX`] or a truncated buffer.
    pub fn decode(bytes: &[u8]) -> Option<(Delimited, usize)> {
        let len = delimited_len(bytes.get(..HEAD)?.try_into().ok()?)?;
        let body = bytes.get(HEAD..HEAD.checked_add(len)?)?.to_vec();
        Some((Delimited { body }, HEAD + len))
    }
}
