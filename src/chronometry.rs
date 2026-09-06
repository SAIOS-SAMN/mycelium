// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CHRONOMETRY — ported from ipsal-autic `chronometry/chain.rs` + CE's
//! CHRONOMETRY_BRIDGE: the K-STEP INDEX is the mesh's temporal authority —
//! algebraic time, never a wall clock. Every state transition stamps a receipt
//! BLAKE3-linked to its parent (LAW-5 continuity); the chain's integrity is
//! verifiable end to end; and temporal validation is PROOF-OF-CONVERGENCE —
//! independent nodes verging to the same irreducible form, not voting. This is
//! the spine the dimensional ledger (Step 4) commits onto.

/// One K-step receipt: `chain_hash = BLAKE3(receipt bytes)`.
/// The delta is the Address bytes themselves, never a digest of content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KStepReceipt {
    /// Monotonic step index — the algebraic clock.
    pub step_index: u64,
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
    fn bytes(step_index: u64, delta: &[u8], op: &[u8; 32], prev: &[u8; 32]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 2 + delta.len() + 64);
        buf.extend_from_slice(&step_index.to_le_bytes());
        buf.extend_from_slice(&(delta.len() as u32).to_le_bytes());
        buf.extend_from_slice(delta);
        buf.extend_from_slice(op);
        buf.extend_from_slice(prev);
        buf
    }

    fn hash(step_index: u64, delta: &[u8], op: &[u8; 32], prev: &[u8; 32]) -> [u8; 32] {
        *blake3::hash(&Self::bytes(step_index, delta, op, prev)).as_bytes()
    }
}

/// The receipt chain — append-only, parent-linked, integrity-verifiable.
#[derive(Debug, Default, Clone)]
pub struct ReceiptChain {
    receipts: Vec<KStepReceipt>,
}

impl ReceiptChain {
    pub fn new() -> Self {
        ReceiptChain::default()
    }

    /// The next step index — the current K (0 at genesis).
    pub fn next_step_index(&self) -> u64 {
        self.receipts.len() as u64
    }

    /// The head's chain hash (all zeros before genesis).
    pub fn head_hash(&self) -> [u8; 32] {
        self.receipts.last().map_or([0u8; 32], |r| r.chain_hash)
    }

    /// The head receipt's step index — verified consensus depth. Zero
    /// before genesis. Not TUniversal.
    pub fn head_k(&self) -> u64 {
        self.receipts.last().map_or(0, |r| r.step_index)
    }

    /// Append the next receipt — K advances monotonically, the link is sealed.
    /// `delta` is the Address bytes the step committed, not a digest of them.
    pub fn append(&mut self, delta: impl AsRef<[u8]>, operator_hash: [u8; 32]) -> &KStepReceipt {
        let step_index = self.next_step_index();
        let prev = self.head_hash();
        let delta = delta.as_ref().to_vec();
        let chain_hash = KStepReceipt::hash(step_index, &delta, &operator_hash, &prev);
        self.receipts.push(KStepReceipt {
            step_index,
            delta,
            operator_hash,
            prev_receipt_hash: prev,
            chain_hash,
        });
        self.receipts.last().unwrap()
    }

    /// Verify the whole chain: indices monotone from 0, every link's parent hash
    /// matches, every chain hash recomputes. A single tampered byte fails.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = [0u8; 32];
        for (i, r) in self.receipts.iter().enumerate() {
            if r.step_index != i as u64 || r.prev_receipt_hash != prev {
                return false;
            }
            let expect = KStepReceipt::hash(r.step_index, &r.delta, &r.operator_hash, &prev);
            if r.chain_hash != expect {
                return false;
            }
            prev = r.chain_hash;
        }
        true
    }

    pub fn receipts(&self) -> &[KStepReceipt] {
        &self.receipts
    }

    /// Rehydrate a chain from persisted receipts. Integrity is verified
    /// before acceptance — a tampered journal refuses whole (continuity or
    /// nothing), so a restored K-lineage is exactly the sealed one.
    pub fn from_receipts(receipts: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let c = ReceiptChain { receipts };
        if c.verify_chain_integrity() {
            Some(c)
        } else {
            None
        }
    }
}

/// TUNIVERSAL — the mycelium universe's ONE wall clock (operator-ruled).
/// Local timezones mutate across "real world" hosts; the universe replays on
/// EXACTNESS. Every stamp derives from a persisted GENESIS ORIGIN — written
/// once at the universe's first boot, carried forever, shared by every node
/// on the state — and monotonic ticks (whole seconds) since it. Never a
/// local-time render.
///
/// The stamp `XX:XX:XX:XX:XX:XX` is the 48-bit tick count as six octets,
/// read as the {S,G,T} triple — three u16 dimensions on the timetable
/// (S = high pair, G = mid pair, T = low pair), the same three axes the
/// K-step orbits carry around their local suns (v0.4 TUniversal lineage;
/// K remains the ALGEBRAIC clock — TUniversal is the universe's wall).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TUniversal {
    pub origin_unix: u64,
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl Default for TUniversal {
    fn default() -> Self {
        TUniversal { origin_unix: unix_now() }
    }
}

impl TUniversal {
    /// Load the persisted genesis origin, or inscribe it NOW (first boot of
    /// the universe on this state). The file holds one u64 LE — no timezone,
    /// no locale, nothing mutable.
    pub fn from_origin(origin_unix: u64) -> Self {
        TUniversal { origin_unix }
    }

    pub fn load_or_genesis(path: &str) -> Self {
        if let Ok(bytes) = std::fs::read(path) {
            if let Ok(raw) = <[u8; 8]>::try_from(bytes.as_slice()) {
                return TUniversal { origin_unix: u64::from_le_bytes(raw) };
            }
        }
        let now = unix_now();
        let tmp = format!("{path}.tmp");
        if std::fs::write(&tmp, now.to_le_bytes()).is_ok() {
            let _ = std::fs::rename(&tmp, path);
        }
        TUniversal { origin_unix: now }
    }

    /// Ticks since genesis — whole seconds, wrapped into the 48-bit stamp
    /// space (over 8.9 million years of universe life before wrap).
    pub fn ticks(&self) -> u64 {
        self.ticks_at(unix_now())
    }

    /// Ticks at a given unix second — tests and the ledger stamp standing
    /// work without reading the wall twice.
    pub fn ticks_at(&self, unix: u64) -> u64 {
        unix.saturating_sub(self.origin_unix) & 0xFFFF_FFFF_FFFF
    }

    /// The {S,G,T} triple of a tick count — the three timetable dimensions.
    pub fn sgt_of(ticks: u64) -> (u16, u16, u16) {
        (((ticks >> 32) & 0xFFFF) as u16, ((ticks >> 16) & 0xFFFF) as u16, (ticks & 0xFFFF) as u16)
    }

    /// Render a tick count as the universal stamp `XX:XX:XX:XX:XX:XX`.
    pub fn stamp_of(ticks: u64) -> String {
        let b = ticks.to_be_bytes();
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", b[2], b[3], b[4], b[5], b[6], b[7])
    }

    /// This instant's universal stamp.
    pub fn stamp(&self) -> String {
        Self::stamp_of(self.ticks())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The stamp is pure arithmetic on ticks — {S,G,T} decompose exactly and
    /// the render carries no locale.
    #[test]
    fn tuniversal_stamp_is_exact_and_timezone_free() {
        let t = 0x0001_0002_0003u64;
        assert_eq!(TUniversal::sgt_of(t), (1, 2, 3), "{{S,G,T}} = the three u16 pairs");
        assert_eq!(TUniversal::stamp_of(t), "00:01:00:02:00:03");
        assert_eq!(TUniversal::stamp_of(0), "00:00:00:00:00:00", "genesis is the zero stamp");
    }

    /// Genesis persists: a second load reads the SAME origin (replay
    /// exactness — the universe's clock never re-anchors).
    #[test]
    fn tuniversal_genesis_persists() {
        let dir = std::env::temp_dir().join(format!("tuniv-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("tuniversal");
        let p = path.to_string_lossy().to_string();
        let a = TUniversal::load_or_genesis(&p);
        let b = TUniversal::load_or_genesis(&p);
        assert_eq!(a.origin_unix, b.origin_unix, "one genesis, carried forever");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// K advances monotonically, links seal, integrity verifies — and one
    /// tampered byte anywhere breaks the whole chain.
    #[test]
    fn the_chain_is_temporal_authority_and_tamper_evident() {
        let mut c = ReceiptChain::new();
        assert_eq!(c.next_step_index(), 0, "genesis K = 0");
        c.append([1u8], [9; 32]);
        c.append([2u8], [9; 32]);
        let r = c.append([3u8], [8; 32]);
        assert_eq!(r.step_index, 2, "K is the algebraic clock");
        assert_eq!(c.head_k(), 2, "head_k is the last receipt, not TUniversal");
        assert_eq!(c.next_step_index(), 3, "next is one past the head");
        assert!(c.verify_chain_integrity());
        // Tamper: flip one byte of the middle receipt's address.
        let mut t = c.clone();
        t.receipts[1].delta[0] ^= 1;
        assert!(!t.verify_chain_integrity(), "a single byte breaks the continuity law");
    }
}
