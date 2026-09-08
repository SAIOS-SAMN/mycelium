// 45 — THE RECEIPT: one K-step, the peer receipts it cites, and the
// height it carries.

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

/// One K-step receipt: `chain_hash = blake3(step ‖ height ‖ parents ‖ delta ‖ op ‖ prev)`.
/// The delta is the Address bytes themselves, never a digest of content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KStepReceipt {
    /// Monotonic step index — this kernel's own K, 0 at genesis.
    pub step_index: u64,
    /// Ledger height (45.2).
    pub height: u64,
    /// Peer receipts this act verified.
    pub parents: Vec<Parent>,
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
        delta: &[u8],
        op: &[u8; 32],
        prev: &[u8; 32],
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&step_index.to_le_bytes());
        buf.extend_from_slice(&height.to_le_bytes());
        Self::put_parents(&mut buf, parents);
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
        delta: &[u8],
        op: &[u8; 32],
        prev: &[u8; 32],
    ) -> [u8; 32] {
        *blake3::hash(&Self::bytes(step_index, height, parents, delta, op, prev)).as_bytes()
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
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let delta = b.get(*pos..*pos + n)?.to_vec();
        *pos += n;
        Some(KStepReceipt {
            step_index,
            height,
            parents,
            vk,
            delta,
            operator_hash: take32(pos)?,
            prev_receipt_hash: take32(pos)?,
            chain_hash: take32(pos)?,
        })
    }
}
