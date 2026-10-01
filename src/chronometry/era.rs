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
/// `first`/`count`/`at`, the K range and the span of the payload in the
/// cold segment — and `receipts` is empty.
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
    /// Counts of the mesh's events, so `u64` (STD1 3b's rule, the
    /// provenance ruling): a fold names as many identities and holds as
    /// many receipts as the mesh brought it, and no width is a ceiling on
    /// that.
    pub identities: u64,
    /// The first place this era holds; `root.at` is its last. Written
    /// out rather than read off `receipts`, so a descriptor whose payload
    /// is on disk still names its own range.
    pub first: Place,
    /// How many receipts the payload holds.
    pub count: u64,
    /// Where the payload stands in the cold segment. `None` on an era that
    /// has not been filed — one being sealed, or one read off a wire.
    pub at: Option<Filed>,
    /// The payload, present only while the era is being sealed or carried.
    pub receipts: Vec<KStepReceipt>,
}

/// WHERE A FILED ERA'S PAYLOAD STANDS in the cold segment: its first byte
/// and its length, written beside it, so a read is of its own bytes and
/// nothing is found by scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filed {
    pub offset: u64,
    pub len: u64,
}

impl Era {
    /// THE COLD SEGMENT'S MAGIC: each era is its descriptor, its payload's
    /// length and its payload, appended once.
    pub const SEGMENT_MAGIC: &'static [u8; 4] = b"MYE2";

    /// The descriptor's tape: root, signer, witness, both signatures, the
    /// identities, the first place and the count.
    pub const DESCRIPTOR: usize = Root::TAPE + 32 + 32 + 64 + 64 + 8 + Place::LEN + 8;

    /// The first place the era holds; the root is its last.
    pub fn first_place(&self) -> Place {
        self.first
    }

    /// The era with its payload shed: what the court keeps once the
    /// receipts are on disk at `at`.
    pub fn filed_at(mut self, at: Filed) -> Era {
        self.at = Some(at);
        self.receipts = Vec::new();
        self
    }

    /// THE ERA AS THE SEGMENT CARRIES IT: its descriptor, its payload's
    /// length, its payload. `None` where a receipt of the payload does not
    /// write (LD0).
    #[must_use = "a frame that cannot be written is refused by name, never dropped (LD0)"]
    pub fn frame(&self) -> Option<Vec<u8>> {
        let payload = self.payload()?;
        let mut out = Vec::with_capacity(Self::DESCRIPTOR + 8 + payload.len());
        self.root.write(&mut out);
        out.extend_from_slice(&self.signer);
        out.extend_from_slice(&self.witness);
        out.extend_from_slice(&self.sig_home);
        out.extend_from_slice(&self.sig_witness);
        out.extend_from_slice(&self.identities.to_le_bytes());
        self.first.put_canonical(&mut out);
        out.extend_from_slice(&self.count.to_le_bytes());
        out.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        out.extend_from_slice(&payload);
        Some(out)
    }

    /// A DESCRIPTOR READ BACK with the length of the payload behind it, the
    /// payload left where it stands: `head` is the descriptor and the
    /// length, exactly. The era comes back unfiled; the caller names where.
    pub fn read_descriptor(head: &[u8]) -> Option<(Era, u64)> {
        let mut p = 0usize;
        let root = Root::read(head, &mut p)?;
        let mut take = |n: usize| -> Option<&[u8]> {
            let v = head.get(p..p + n)?;
            p += n;
            Some(v)
        };
        let signer: [u8; 32] = take(32)?.try_into().ok()?;
        let witness: [u8; 32] = take(32)?.try_into().ok()?;
        let sig_home: [u8; 64] = take(64)?.try_into().ok()?;
        let sig_witness: [u8; 64] = take(64)?.try_into().ok()?;
        let identities = u64::from_le_bytes(take(8)?.try_into().ok()?);
        let first = Place::take_canonical(head, &mut p)?;
        let count = u64::from_le_bytes(head.get(p..p + 8)?.try_into().ok()?);
        let len = u64::from_le_bytes(head.get(p + 8..p + 16)?.try_into().ok()?);
        (p + 16 == head.len()).then_some(())?;
        let era = Era { root, signer, witness, sig_home, sig_witness, identities, first, count, at: None, receipts: Vec::new() };
        Some((era, len))
    }

    /// The payload alone, as it is appended to the cold segment: the count
    /// and each node by the ledger's own writer.
    #[must_use = "a payload that cannot be written is refused by name, never dropped (LD0)"]
    pub fn payload(&self) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        // The receipt count is `count`'s, the mesh's, so `u64` as the
        // descriptor carries it: no width refuses a fold the mesh made.
        out.extend_from_slice(&(self.receipts.len() as u64).to_le_bytes());
        for r in &self.receipts {
            r.write(&mut out)?;
        }
        Some(out)
    }

    /// Read a payload back: the count and the receipts behind it.
    pub fn read_payload(b: &[u8], pos: &mut usize) -> Option<Vec<KStepReceipt>> {
        let n = usize::try_from(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?)).ok()?;
        *pos += 8;
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
