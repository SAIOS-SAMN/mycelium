// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.1 — THE SIGNED PREFIX: the one byte string every signature in
//! the tree is over, carrying the signer's causal position.

/// The receipt refusals are 34.1's, read here by the name that chapter
/// gives them.
pub use crate::court::verdict::{
    REFUSE_EQUIVOCATION, REFUSE_MALFORMED, REFUSE_NOT_EQUIVOCATION, REFUSE_ORIGIN, REFUSE_RECEIPT,
    REFUSE_RECEIPT_STALE, REFUSE_SIGNATURE,
};


/// 47.1's signed prefix, held as its fields so a reader takes coordinates
/// off it rather than a digest. `bytes()` is the ONLY thing ever signed,
/// in this order and no other:
///
/// ```text
/// H_genesis 32 ‖ E_origin u32 ‖ prev_receipt_hash 32 ‖ K u64 ‖ i u64
///   ‖ height u64 ‖ chart len16 + axes u64… ‖ payload address len16 + bytes
/// ```
///
/// `K ‖ i` is the signer's place on its line: the orbit and the act's
/// place in it (78.23 as amended).
///
/// The payload is an ADDRESS -- the M2 composite (A21) for a word or form,
/// a root for a transmission digest, a listing key for a settlement act --
/// and never a digest of frame bytes, so two spellings of one word share
/// one receipt lineage and a receiver reads coordinates off the address
/// (law 2). `prev_receipt_hash` is a digest of receipt bytes because the
/// receipt bytes are the object there (Part IX).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedPrefix {
    pub genesis: [u8; 32],
    pub epoch: u32,
    pub prev_receipt_hash: [u8; 32],
    pub at: crate::chronometry::Place,
    pub height: u64,
    /// THE CHART'S AXES, WRITTEN OUT — its own key (law 2): each coordinate
    /// kind-tagged and length-prefixed, so the chart is recovered from the
    /// signed bytes and a prefix of them is a coarser stratum. Never a
    /// fingerprint of the chart.
    ///
    /// These were `u64` limbs, which is what an axis was before a
    /// coordinate became whole (A21 as ruled 2026-09-18): a rational
    /// crossed two of them and a signer could not write one without
    /// splitting it.
    pub chart: Vec<u8>,
    /// The payload's ADDRESS bytes (A21), written out and decodable.
    pub payload: Vec<u8>,
}

impl SignedPrefix {
    /// Does this prefix fit the lengths the wire writes for it: its chart
    /// count and payload length in two bytes each, and the whole of
    /// `bytes()` in the two-byte length the receipt section frames it with
    /// (47.1, I33). A prefix that does not fit is refused where it is
    /// built — never signed, never framed — because a length written
    /// `as u16` wraps, and a wrapped section is refused whole by every
    /// receiver while the sender believes it spoke.
    pub fn fits(&self) -> bool {
        let max = u16::MAX as usize;
        self.chart.len() <= max
            && self.payload.len() <= max
            && Self::FIXED + 2 + self.chart.len() + 2 + self.payload.len() <= max
    }

    /// The fixed fields' bytes: origin, epoch, predecessor, place, height.
    const FIXED: usize = 32 + 4 + 32 + 8 + 8 + 8;

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Self::FIXED + 4 + self.chart.len() + self.payload.len());
        out.extend_from_slice(&self.genesis);
        out.extend_from_slice(&self.epoch.to_le_bytes());
        out.extend_from_slice(&self.prev_receipt_hash);
        out.extend_from_slice(&self.at.k.to_le_bytes());
        out.extend_from_slice(&self.at.i.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());
        out.extend_from_slice(&(self.chart.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.chart);
        out.extend_from_slice(&(self.payload.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }

    /// Read the prefix back. Length-prefixed and decodable end to end (A21):
    /// a prefix that does not parse whole is refused, never truncated.
    pub fn read(b: &[u8]) -> Option<Self> {
        let mut p = 0usize;
        let mut take = |n: usize| -> Option<&[u8]> {
            let s = b.get(p..p + n)?;
            p += n;
            Some(s)
        };
        let genesis: [u8; 32] = take(32)?.try_into().ok()?;
        let epoch = u32::from_le_bytes(take(4)?.try_into().ok()?);
        let prev_receipt_hash: [u8; 32] = take(32)?.try_into().ok()?;
        let k = u64::from_le_bytes(take(8)?.try_into().ok()?);
        let i = u64::from_le_bytes(take(8)?.try_into().ok()?);
        let at = crate::chronometry::Place { k, i };
        let height = u64::from_le_bytes(take(8)?.try_into().ok()?);
        let n = u16::from_le_bytes(take(2)?.try_into().ok()?) as usize;
        let chart = take(n)?.to_vec();
        let m = u16::from_le_bytes(take(2)?.try_into().ok()?) as usize;
        let payload = take(m)?.to_vec();
        if p != b.len() {
            return None;
        }
        Some(SignedPrefix { genesis, epoch, prev_receipt_hash, at, height, chart, payload })
    }
}

