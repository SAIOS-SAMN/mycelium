// 45.8 — THE ERA: a fold of one chain at a height, the root it leaves the
// hot chain standing on, the two organs that signed it, and the cold
// prefix it archived.

use super::receipt::{KStepReceipt, Place};

/// THE FOLD ROOT: the archived prefix's head — its place, chain hash and
/// height. The hot chain continues from it: the next receipt's place
/// follows it and its `prev_receipt_hash` is this hash, so the chain never
/// breaks and heights continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Root {
    pub at: Place,
    pub hash: [u8; 32],
    pub height: u64,
}

/// ONE ERA in `ARC1` (28): the fold root, whose chain it folds, the
/// witness that co-signed it, both signatures over the root (47.3), the
/// identities the fold's epoch rows named, and the receipts of the folded
/// prefix, cold — held by the home and by the witness that fetched them,
/// extracted by K range, never rewritten.
///
/// COLD MEANS OFF THE HEAP. `receipts` is the payload only while an era is
/// being sealed or carried on the wire; once it is filed, the court holds
/// the DESCRIPTOR — root, signer, witness, signatures, identities, and
/// `first`/`count`/`at`, the K range and the byte offset of the payload in
/// the cold segment — and `receipts` is empty.
///
/// A fold that moved receipts from one in-memory `Vec` to another, and
/// then wrote every one of them into the hot ledger on every persist, is
/// not an eviction: it is a rename. Measured on the fleet, 2026-09-11,
/// after genesis adoption unblocked folding — 71 folds on one court in a
/// single boot, `ARC1` 3561 MiB of a 3567 MiB ledger, and two courts at
/// 4146 MiB and 2343 MiB of resident memory within 106 seconds of restore.
/// That is what crashed the machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Era {
    pub root: Root,
    pub signer: [u8; 32],
    pub witness: [u8; 32],
    pub sig_home: [u8; 64],
    pub sig_witness: [u8; 64],
    pub identities: u32,
    /// The first place this era holds; `root.at` is its last. Written
    /// out rather than read off `receipts`, so a descriptor whose payload
    /// is on disk still names its own range.
    pub first: Place,
    /// How many receipts the payload holds.
    pub count: u32,
    /// Where the payload begins in the cold segment. `None` on an era
    /// that has not been filed — one being sealed, or one read off a wire.
    pub at: Option<u64>,
    /// The payload, present only while the era is being sealed or carried.
    pub receipts: Vec<KStepReceipt>,
}

impl Era {
    /// The first place the era holds; the root is its last.
    pub fn first_place(&self) -> Place {
        self.first
    }

    /// The era with its payload shed: what the court keeps once the
    /// receipts are on disk at `at`.
    pub fn filed_at(mut self, at: u64) -> Era {
        self.at = Some(at);
        self.receipts = Vec::new();
        self
    }

    /// The payload alone, as it is appended to the cold segment: the count
    /// and each node by the ledger's own writer.
    #[must_use = "a payload that cannot be written is refused by name, never dropped (LD0)"]
    pub fn payload(&self) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        crate::transducer::address::put_u32_count(&mut out, self.receipts.len())?;
        for r in &self.receipts {
            r.write(&mut out)?;
        }
        Some(out)
    }

    /// Read a payload back: the count and the receipts behind it.
    pub fn read_payload(b: &[u8], pos: &mut usize) -> Option<Vec<KStepReceipt>> {
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if n > b.len().checked_sub(*pos)? / KStepReceipt::MIN_TAPE {
            return None;
        }
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(KStepReceipt::read(b, pos)?);
        }
        Some(out)
    }
}

impl Root {
    /// The root's tape: place, hash, height.
    pub const TAPE: usize = Place::LEN + 32 + 8;

    pub fn write(&self, out: &mut Vec<u8>) {
        self.at.put_canonical(out);
        out.extend_from_slice(&self.hash);
        out.extend_from_slice(&self.height.to_le_bytes());
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Root> {
        let at = Place::take_canonical(b, pos)?;
        let hash: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let height = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        Some(Root { at, hash, height })
    }
}
