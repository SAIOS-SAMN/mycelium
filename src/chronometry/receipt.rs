// 45 — THE RECEIPT: one K-step, the peer receipts it cites, the height it
// carries, and the kind of act it admits.

/// A peer receipt this act verified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parent {
    pub vk: [u8; 32],
    pub k: u64,
    pub hash: [u8; 32],
}

/// Height of a receipt in the ledger (45.2). Genesis is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Height(pub u64);

impl Height {
    pub fn get(self) -> u64 {
        self.0
    }

    /// A height reads as a height. This printed `XX:XX:XX:XX:XX:XX` — the
    /// big-endian bytes of the number, formatted to look like a wall clock
    /// (C6). There is no clock (law 9, Part IX): TUniversal is the height of
    /// the receipt ledger, and a reader who sees a timestamp will reason
    /// about elapsed time that nothing in this system measures. `h {n}`, as
    /// 34.2 writes it.
    pub fn stamp(self) -> String {
        Self::stamp_of(self.0)
    }

    pub fn stamp_of(h: u64) -> String {
        format!("h {h}")
    }
}

/// TUniversal is the height of the mesh receipt ledger.
pub type TUniversal = Height;

/// THE KIND OF ACT A RECEIPT ADMITS (45.1, ruled 2026-09-09; S13.6):
/// written on the tape as one coordinate byte after the parents and in the
/// chain hash, so a phase (45.9) and a fold (45.8) read the ledger by
/// kind, never by grepping a journal. A byte that names no kind refuses
/// the read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActKind {
    Attest,
    List,
    Section,
    Install,
    Ruling,
    Syllabus,
    Lapse,
    Fold,
    Adopted,
    Stood,
    Uttered,
    Learned,
    Settled,
}

impl ActKind {
    /// The kind as the coordinate byte the tape carries.
    pub fn to_byte(self) -> u8 {
        match self {
            ActKind::Attest => 1,
            ActKind::List => 2,
            ActKind::Section => 3,
            ActKind::Install => 4,
            ActKind::Ruling => 5,
            ActKind::Syllabus => 6,
            ActKind::Lapse => 7,
            ActKind::Fold => 8,
            ActKind::Adopted => 9,
            ActKind::Stood => 10,
            ActKind::Uttered => 11,
            ActKind::Learned => 12,
            ActKind::Settled => 13,
        }
    }

    /// The kind a tape byte names; `None` names no kind and refuses.
    pub fn from_byte(b: u8) -> Option<ActKind> {
        Some(match b {
            1 => ActKind::Attest,
            2 => ActKind::List,
            3 => ActKind::Section,
            4 => ActKind::Install,
            5 => ActKind::Ruling,
            6 => ActKind::Syllabus,
            7 => ActKind::Lapse,
            8 => ActKind::Fold,
            9 => ActKind::Adopted,
            10 => ActKind::Stood,
            11 => ActKind::Uttered,
            12 => ActKind::Learned,
            13 => ActKind::Settled,
            _ => return None,
        })
    }

    /// The kind's name, the bytes the mind's operator digest is taken over.
    pub fn name(self) -> &'static [u8] {
        match self {
            ActKind::Attest => b"attest",
            ActKind::List => b"list",
            ActKind::Section => b"section",
            ActKind::Install => b"install",
            ActKind::Ruling => b"ruling",
            ActKind::Syllabus => b"syllabus",
            ActKind::Lapse => b"lapse",
            ActKind::Fold => b"fold",
            ActKind::Adopted => b"adopted",
            ActKind::Stood => b"stood",
            ActKind::Uttered => b"uttered",
            ActKind::Learned => b"learned",
            ActKind::Settled => b"settled",
        }
    }

    /// The operator digest of a mind's own act (never reasoned on).
    pub fn operator(self) -> [u8; 32] {
        payload_hash(self.name())
    }
}

/// One K-step receipt: `chain_hash = blake3(step ‖ height ‖ parents ‖ kind ‖ delta ‖ op ‖ prev)`.
/// The delta is the Address bytes themselves, never a digest of content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KStepReceipt {
    /// Monotonic step index — this kernel's own K, 0 at genesis.
    pub step_index: u64,
    /// Ledger height (45.2).
    pub height: u64,
    /// Peer receipts this act verified.
    pub parents: Vec<Parent>,
    /// The act admitted (45.1).
    pub kind: ActKind,
    /// This kernel's verifying key.
    pub vk: [u8; 32],
    /// The committed address, written out.
    pub delta: Vec<u8>,
    /// Which operation was applied (an act digest — never reasoned on).
    pub operator_hash: [u8; 32],
    /// The parent receipt's chain hash (all zeros at genesis).
    pub prev_receipt_hash: [u8; 32],
    /// This receipt's chain hash.
    pub chain_hash: [u8; 32],
}

/// Receipt payload identity of admitted content bytes (Part IX: the
/// object of the hash is the bytes written on the receipt).
pub fn payload_hash(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

pub fn operator_hash(voice: &[u8; 32], court_head: &[u8; 32]) -> [u8; 32] {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(voice);
    buf[32..].copy_from_slice(court_head);
    *blake3::hash(&buf).as_bytes()
}

impl KStepReceipt {
    fn put_parents(buf: &mut Vec<u8>, parents: &[Parent]) {
        buf.extend_from_slice(&(parents.len() as u32).to_le_bytes());
        for p in parents {
            buf.extend_from_slice(&p.vk);
            buf.extend_from_slice(&p.k.to_le_bytes());
            buf.extend_from_slice(&p.hash);
        }
    }

    fn bytes(
        step_index: u64,
        height: u64,
        parents: &[Parent],
        kind: ActKind,
        delta: &[u8],
        op: &[u8; 32],
        prev: &[u8; 32],
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&step_index.to_le_bytes());
        buf.extend_from_slice(&height.to_le_bytes());
        Self::put_parents(&mut buf, parents);
        buf.push(kind.to_byte());
        buf.extend_from_slice(&(delta.len() as u32).to_le_bytes());
        buf.extend_from_slice(delta);
        buf.extend_from_slice(op);
        buf.extend_from_slice(prev);
        buf
    }

    pub(super) fn hash(
        step_index: u64,
        height: u64,
        parents: &[Parent],
        kind: ActKind,
        delta: &[u8],
        op: &[u8; 32],
        prev: &[u8; 32],
    ) -> [u8; 32] {
        *blake3::hash(&Self::bytes(step_index, height, parents, kind, delta, op, prev)).as_bytes()
    }

    /// Does this receipt's chain hash re-derive from its own fields. A
    /// receipt presented by a third party (47.4) is believed only through
    /// this: the signature covers the prefix, and the prefix names the K,
    /// height and predecessor the receipt claims, so a hash that does not
    /// re-derive is a receipt nobody wrote.
    pub fn rederives(&self) -> bool {
        self.chain_hash
            == Self::hash(
                self.step_index,
                self.height,
                &self.parents,
                self.kind,
                &self.delta,
                &self.operator_hash,
                &self.prev_receipt_hash,
            )
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.step_index.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());
        out.extend_from_slice(&self.vk);
        Self::put_parents(out, &self.parents);
        out.push(self.kind.to_byte());
        out.extend_from_slice(&(self.delta.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.delta);
        out.extend_from_slice(&self.operator_hash);
        out.extend_from_slice(&self.prev_receipt_hash);
        out.extend_from_slice(&self.chain_hash);
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Self> {
        let take32 = |p: &mut usize| -> Option<[u8; 32]> {
            let v: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(v)
        };
        let step_index = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let height = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let vk = take32(pos)?;
        let np = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let mut parents = Vec::with_capacity(np.min(1024));
        for _ in 0..np {
            let vk = take32(pos)?;
            let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            let hash = take32(pos)?;
            parents.push(Parent { vk, k, hash });
        }
        let kind = ActKind::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let delta = b.get(*pos..*pos + n)?.to_vec();
        *pos += n;
        Some(KStepReceipt {
            step_index,
            height,
            parents,
            kind,
            vk,
            delta,
            operator_hash: take32(pos)?,
            prev_receipt_hash: take32(pos)?,
            chain_hash: take32(pos)?,
        })
    }
}
