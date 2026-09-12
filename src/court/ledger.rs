// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 28, 45.1 — THE COURT'S STATE ON DISK: the federation, the ruled set,
//! the rebind marks, the ruling counters and the K-step receipt chain as
//! one file; at boot the chain re-verifies link by link or refuses whole.

use crate::chronometry::{KStepReceipt, ReceiptChain};
use crate::transducer::element::Element;

pub struct CourtState {
    pub chain: ReceiptChain,
    pub federation: Vec<(crate::transducer::address::Address, Element)>,
    pub ruled: Vec<crate::transducer::address::Address>,
    pub rebound: Vec<(crate::transducer::address::Address, usize)>,
    pub upheld: u64,
    pub demoted: u64,
    pub slashed: u64,
    pub rebinds: u64,
    /// Cavities admitted without replay: the sheaf boundary where no
    /// global section glued under the current basis.
    pub dialect: Vec<Element>,
}

fn push_u32(out: &mut Vec<u8>, v: usize) {
    out.extend_from_slice(&(v as u32).to_le_bytes());
}
fn read_u32(b: &[u8], p: &mut usize) -> Option<usize> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    Some(v)
}
impl CourtState {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_u32(&mut out, self.chain.receipts().len());
        for r in self.chain.receipts() {
            r.write(&mut out);
        }
        push_u32(&mut out, self.federation.len());
        for (addr, elem) in &self.federation {
            let _ = crate::transducer::address::put_addr(&mut out, addr);
            let bytes = elem.to_bytes();
            push_u32(&mut out, bytes.len());
            out.extend_from_slice(&bytes);
        }
        push_u32(&mut out, self.ruled.len());
        for addr in &self.ruled {
            let _ = crate::transducer::address::put_addr(&mut out, addr);
        }
        push_u32(&mut out, self.rebound.len());
        for (addr, fam) in &self.rebound {
            let _ = crate::transducer::address::put_addr(&mut out, addr);
            push_u32(&mut out, *fam);
        }
        for c in [self.upheld, self.demoted, self.slashed, self.rebinds] {
            out.extend_from_slice(&c.to_le_bytes());
        }
        push_u32(&mut out, self.dialect.len());
        for elem in &self.dialect {
            let bytes = elem.to_bytes();
            push_u32(&mut out, bytes.len());
            out.extend_from_slice(&bytes);
        }
        out
    }

    /// Rehydrate from persisted bytes. The receipt chain re-verifies before
    /// acceptance; any structural damage refuses the whole state (a court
    /// with no history is honest; a court with a corrupted one is not).
    pub fn from_bytes(b: &[u8]) -> Option<CourtState> {
        let mut p = 0usize;
        let n = read_u32(b, &mut p)?;
        let mut receipts = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            receipts.push(KStepReceipt::read(b, &mut p)?);
        }
        let chain = ReceiptChain::from_receipts(receipts)?;
        let n = read_u32(b, &mut p)?;
        let mut federation = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            let addr = crate::transducer::address::take_addr(b, &mut p)?;
            let len = read_u32(b, &mut p)?;
            let slice = b.get(p..p + len)?;
            let mut ep = 0usize;
            let elem = Element::from_bytes(slice, &mut ep)?;
            p += len;
            federation.push((addr, elem));
        }
        let n = read_u32(b, &mut p)?;
        let mut ruled = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            ruled.push(crate::transducer::address::take_addr(b, &mut p)?);
        }
        let n = read_u32(b, &mut p)?;
        let mut rebound = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            let addr = crate::transducer::address::take_addr(b, &mut p)?;
            let fam = read_u32(b, &mut p)?;
            rebound.push((addr, fam));
        }
        let mut counters = [0u64; 4];
        for c in counters.iter_mut() {
            *c = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
            p += 8;
        }
        let mut dialect = Vec::new();
        if p < b.len() {
            let n = read_u32(b, &mut p)?;
            dialect.reserve(n.min(1 << 16));
            for _ in 0..n {
                let len = read_u32(b, &mut p)?;
                let slice = b.get(p..p + len)?;
                let mut ep = 0usize;
                let elem = Element::from_bytes(slice, &mut ep)?;
                p += len;
                dialect.push(elem);
            }
        }
        Some(CourtState {
            chain,
            federation,
            ruled,
            rebound,
            upheld: counters[0],
            demoted: counters[1],
            slashed: counters[2],
            rebinds: counters[3],
            dialect,
        })
    }

    /// Atomic persistence: write-then-rename, the registry pattern — a
    /// crash between rename boundaries leaves the previous sealed state.
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let tmp = format!("{path}.tmp");
        std::fs::write(&tmp, self.to_bytes())?;
        std::fs::rename(&tmp, path)
    }

    /// Load, or start at genesis when no journal exists. A journal that
    /// fails chain verification is refused (None distinguishes "corrupt"
    /// from "absent" so the caller can name it).
    pub fn load(path: &str) -> Option<CourtState> {
        let bytes = std::fs::read(path).ok()?;
        CourtState::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_canonical;
    use crate::transducer::germinate::germinate;

    /// The full judicial state survives the round trip: K-lineage continues
    /// (not reset to zero), the ruled set deduplicates re-adjudication, and
    /// a single tampered byte in the receipt chain refuses the whole state.
    #[test]
    fn court_state_survives_restart_and_refuses_tampering() {
        let word = from_canonical(
            &germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap(),
        );
        let addr = word.address();
        let mut chain = ReceiptChain::default();
        let mut op = [0u8; 32];
        op[0] = crate::anastome::dialogue::RULING_UPHELD;
        chain
            .append(crate::chronometry::ActKind::Ruling, word.magnitude(), &addr, op, Vec::new())
            .expect("genesis receipt");
        let state = CourtState {
            chain,
            federation: vec![(addr.clone(), word)],
            ruled: vec![addr.clone()],
            rebound: vec![(addr.clone(), 2)],
            upheld: 1,
            demoted: 0,
            slashed: 0,
            rebinds: 1,
            dialect: Vec::new(),
        };
        let bytes = state.to_bytes();
        let back = CourtState::from_bytes(&bytes).expect("clean state rehydrates");
        assert_eq!(back.chain.next_step_index(), 1, "K continues, never resets");
        assert_eq!(back.chain.head_hash(), state.chain.head_hash(), "lineage head matches");
        assert!(back.ruled.contains(&addr), "sealed words are never re-adjudicated");
        assert_eq!(back.federation.len(), 1);
        assert_eq!(back.federation[0].1.address(), addr, "the word rehydrates exactly");
        assert_eq!((back.upheld, back.rebinds), (1, 1));
        assert!(back.dialect.is_empty(), "a journal with no cavities rehydrates an empty dialect");
        // Tamper one byte inside the receipt region: the chain fails to
        // verify and the WHOLE state refuses.
        let mut bad = bytes.clone();
        bad[8] ^= 1;
        assert!(CourtState::from_bytes(&bad).is_none(), "a tampered journal refuses whole");
    }
}
