// 45.8 — THE ERA: a fold of one chain at a height, the root it leaves the
// hot chain standing on, the two organs that signed it, and the cold
// prefix it archived.

use super::receipt::{KStepReceipt, Place, Stamp};

/// U12 (78.65 as amended 2026-10-09) — WHAT STAYS OPEN ACROSS A FOLD: the
/// kind of asymmetric chain a puncture carries onto the new surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chain {
    /// A claim off the ledger (`Treasury::escrowed`): open until the act
    /// that releases, draws or revokes it.
    Escrow = 1,
    /// A listing on the open book (`Kernel::pending`), its demands
    /// unresolved: open until its ruling or its lapse.
    Demand = 2,
}

impl Chain {
    fn from_byte(b: u8) -> Option<Chain> {
        match b {
            1 => Some(Chain::Escrow),
            2 => Some(Chain::Demand),
            _ => None,
        }
    }
}

/// U12 — A TOPOLOGICAL PUNCTURE: an open chain the fold's coboundary
/// reduction (`δ_causal`) did not close and did not wait for, projected
/// onto the new active surface as a living defect. It is a CITATION, never
/// a copy: the word the chain stands on is named by the stamp of the act
/// that admitted it (78.24, `Kernel::admitted_at`), the chain's opening by
/// its place on this line, and the charge it carries across by its lanes —
/// the escrow's claim, or the listing's measure. Settled in the new era, it
/// settles the chain its citation names and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Puncture {
    pub chain: Chain,
    /// The admitting stamp of the word the open chain stands on.
    pub word: Stamp,
    /// Where the chain opened on this line: an escrow's anchor, a
    /// listing's admitting act. At or below the root it crossed.
    pub opened: Place,
    /// The charge carried across the surface, per lane, exact.
    pub lanes: [u64; 4],
}

impl Puncture {
    /// The puncture's tape: kind, the word's stamp, the opening place, the
    /// four lanes. Fixed in length, so a run of them is read by count.
    pub const TAPE: usize = 1 + Stamp::LEN + Place::LEN + 4 * 8;

    pub fn write(&self, out: &mut Vec<u8>) {
        out.push(self.chain as u8);
        self.word.put_canonical(out);
        self.opened.put_canonical(out);
        for l in self.lanes {
            out.extend_from_slice(&l.to_le_bytes());
        }
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Puncture> {
        let chain = Chain::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let word = Stamp::take_canonical(b, pos)?;
        let opened = Place::take_canonical(b, pos)?;
        let mut lanes = [0u64; 4];
        for l in &mut lanes {
            *l = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        Some(Puncture { chain, word, opened, lanes })
    }

    /// The run of punctures a fold projected, `n` of them, read whole from
    /// `b` or refused.
    pub fn read_run(b: &[u8], n: u64) -> Option<Vec<Puncture>> {
        let n = usize::try_from(n).ok()?;
        (b.len() == n.checked_mul(Self::TAPE)?).then_some(())?;
        let mut p = 0usize;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(Self::read(b, &mut p)?);
        }
        Some(out)
    }

    /// The key a live puncture is found by: its kind, then the word's stamp.
    pub fn key(&self) -> Vec<u8> {
        let mut out = vec![self.chain as u8];
        self.word.put_canonical(&mut out);
        out
    }
}

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
    /// U12 — the open chains on this era's surface that an EARLIER era
    /// projected and that are still open: carried, cited in their own era,
    /// never written twice.
    pub carried: u64,
    /// U12 — how many punctures this fold projected first: the open chains
    /// that opened in its prefix. Written on the descriptor, so a filed
    /// era names them without its puncture run in hand.
    pub punctured: u64,
    /// Where the payload stands in the cold segment. `None` on an era that
    /// has not been filed — one being sealed, or one read off a wire.
    pub at: Option<Filed>,
    /// The payload, present only while the era is being sealed or carried.
    pub receipts: Vec<KStepReceipt>,
    /// U12 — the punctures this fold projected (`punctured` of them),
    /// present while the era is sealed, carried or held in hand; filed, they
    /// stand in the segment just before the payload and are read there.
    /// A witness's copy holds none: an escrow and a listing are the home's
    /// carriers off the ledger, which no witness re-derives (78.65).
    pub punctures: Vec<Puncture>,
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
    /// length, the punctures it projected and its payload, appended once.
    /// U12 moved the record (2026-10-09): `MYE2` held no punctures and is
    /// refused by name (`PRE_PUNCTURE_MAGIC`) — the recut is the road.
    pub const SEGMENT_MAGIC: &'static [u8; 4] = b"MYE3";

    /// The segment before U12: descriptors with no carried count, no
    /// puncture run. Never migrated.
    pub const PRE_PUNCTURE_MAGIC: &'static [u8; 4] = b"MYE2";

    /// The descriptor's tape: root, signer, witness, both signatures, the
    /// identities, the first place, the count, the carried count and the
    /// punctures projected.
    pub const DESCRIPTOR: usize = Root::TAPE + 32 + 32 + 64 + 64 + 8 + Place::LEN + 8 + 8 + 8;

    /// The bytes of the frame before the payload: the descriptor, the
    /// payload's length, and the puncture run. `None` past the carrier.
    pub fn head_len(punctured: u64) -> Option<u64> {
        punctured.checked_mul(Puncture::TAPE as u64)?.checked_add(Self::DESCRIPTOR as u64 + 8)
    }

    /// The first place the era holds; the root is its last.
    pub fn first_place(&self) -> Place {
        self.first
    }

    /// The era with its payload shed: what the court keeps once the
    /// receipts are on disk at `at`.
    pub fn filed_at(mut self, at: Filed) -> Era {
        self.at = Some(at);
        self.receipts = Vec::new();
        self.punctures = Vec::new();
        self
    }

    /// THE ERA AS THE SEGMENT CARRIES IT: its descriptor, its payload's
    /// length, its puncture run, its payload. `None` where a receipt of the
    /// payload does not write (LD0), or where the run is not the count the
    /// descriptor names.
    #[must_use = "a frame that cannot be written is refused by name, never dropped (LD0)"]
    pub fn frame(&self) -> Option<Vec<u8>> {
        (u64::try_from(self.punctures.len()).ok()? == self.punctured).then_some(())?;
        let payload = self.payload()?;
        let head = usize::try_from(Self::head_len(self.punctured)?).ok()?;
        let mut out = Vec::with_capacity(head + payload.len());
        self.root.write(&mut out);
        out.extend_from_slice(&self.signer);
        out.extend_from_slice(&self.witness);
        out.extend_from_slice(&self.sig_home);
        out.extend_from_slice(&self.sig_witness);
        out.extend_from_slice(&self.identities.to_le_bytes());
        self.first.put_canonical(&mut out);
        out.extend_from_slice(&self.count.to_le_bytes());
        out.extend_from_slice(&self.carried.to_le_bytes());
        out.extend_from_slice(&self.punctured.to_le_bytes());
        out.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        for p in &self.punctures {
            p.write(&mut out);
        }
        out.extend_from_slice(&payload);
        Some(out)
    }

    /// A DESCRIPTOR READ BACK with the length of the payload behind it, the
    /// puncture run and the payload left where they stand: `head` is the
    /// descriptor and the length, exactly. The era comes back unfiled; the
    /// caller names where.
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
        let word = |p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(head.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let count = word(&mut p)?;
        let carried = word(&mut p)?;
        let punctured = word(&mut p)?;
        let len = word(&mut p)?;
        (p == head.len()).then_some(())?;
        let era = Era {
            root,
            signer,
            witness,
            sig_home,
            sig_witness,
            identities,
            first,
            count,
            carried,
            punctured,
            at: None,
            receipts: Vec::new(),
            punctures: Vec::new(),
        };
        Some((era, len))
    }

    /// The payload alone, as it is appended to the cold segment: its run of
    /// one line, packed (`pack.rs`) — each node after the first re-derives
    /// its stamp from the one before it — and the anchor on its last node.
    #[must_use = "a payload that cannot be written is refused by name, never dropped (LD0)"]
    pub fn payload(&self) -> Option<Vec<u8>> {
        let mut packer = super::pack::Packer::new();
        let mut out = Vec::new();
        for r in &self.receipts {
            packer.node(&mut out, r)?;
        }
        packer.anchors(&mut out);
        Some(out)
    }

    /// Read a payload back, from `pos` to the end of `b`: its nodes, each
    /// re-derived, the anchor holding.
    pub fn read_payload(b: &[u8], pos: &mut usize) -> Option<Vec<KStepReceipt>> {
        let mut unpacker = super::pack::Unpacker::new();
        let mut out = Vec::new();
        while *pos < b.len() {
            match unpacker.take(b, pos)? {
                super::pack::Record::Node(r) => out.push(*r),
                super::pack::Record::Read => {}
                super::pack::Record::Root(_) | super::pack::Record::Gone(_) => return None,
            }
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
