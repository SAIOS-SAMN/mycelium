// 45.8 — THE ERA: a fold of one chain at a height, the root it leaves the
// hot chain standing on, the two organs that signed it, and the cold
// prefix it archived.

use super::receipt::KStepReceipt;

/// THE FOLD ROOT: the archived prefix's head — its K, chain hash and
/// height. The hot chain continues from it: the next receipt's K is one
/// past it and its `prev_receipt_hash` is this hash, so the chain never
/// breaks and heights continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Root {
    pub k: u64,
    pub hash: [u8; 32],
    pub height: u64,
}

/// ONE ERA in `ARC1` (28): the fold root, whose chain it folds, the
/// witness that co-signed it, both signatures over the root (47.3), the
/// identities the fold's epoch rows named, and the receipts of the folded
/// prefix, cold — held by the home and by the witness that fetched them,
/// extracted by K range, never rewritten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Era {
    pub root: Root,
    pub signer: [u8; 32],
    pub witness: [u8; 32],
    pub sig_home: [u8; 64],
    pub sig_witness: [u8; 64],
    pub identities: u32,
    pub receipts: Vec<KStepReceipt>,
}

impl Era {
    /// The first K the era holds; the root is its last.
    pub fn first_k(&self) -> Option<u64> {
        self.receipts.first().map(|r| r.step_index)
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        self.root.write(out);
        out.extend_from_slice(&self.signer);
        out.extend_from_slice(&self.witness);
        out.extend_from_slice(&self.sig_home);
        out.extend_from_slice(&self.sig_witness);
        out.extend_from_slice(&self.identities.to_le_bytes());
        out.extend_from_slice(&(self.receipts.len() as u32).to_le_bytes());
        for r in &self.receipts {
            r.write(out);
        }
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Era> {
        let root = Root::read(b, pos)?;
        let take = |p: &mut usize, n: usize| -> Option<&[u8]> {
            let s = b.get(*p..*p + n)?;
            *p += n;
            Some(s)
        };
        let signer: [u8; 32] = take(pos, 32)?.try_into().ok()?;
        let witness: [u8; 32] = take(pos, 32)?.try_into().ok()?;
        let sig_home: [u8; 64] = take(pos, 64)?.try_into().ok()?;
        let sig_witness: [u8; 64] = take(pos, 64)?.try_into().ok()?;
        let identities = u32::from_le_bytes(take(pos, 4)?.try_into().ok()?);
        let n = u32::from_le_bytes(take(pos, 4)?.try_into().ok()?) as usize;
        if n > b.len().saturating_sub(*pos) / KStepReceipt::MIN_TAPE {
            return None;
        }
        let mut receipts = Vec::with_capacity(n);
        for _ in 0..n {
            receipts.push(KStepReceipt::read(b, pos)?);
        }
        Some(Era { root, signer, witness, sig_home, sig_witness, identities, receipts })
    }
}

impl Root {
    /// The root's tape: K, hash, height.
    pub const TAPE: usize = 8 + 32 + 8;

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.k.to_le_bytes());
        out.extend_from_slice(&self.hash);
        out.extend_from_slice(&self.height.to_le_bytes());
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Root> {
        let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let hash: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let height = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        Some(Root { k, hash, height })
    }

    /// A root or none, as one tag byte then the root: `0` none, `1` root.
    pub fn write_opt(root: Option<&Root>, out: &mut Vec<u8>) {
        match root {
            None => out.push(0),
            Some(r) => {
                out.push(1);
                r.write(out);
            }
        }
    }

    pub fn read_opt(b: &[u8], pos: &mut usize) -> Option<Option<Root>> {
        let tag = *b.get(*pos)?;
        *pos += 1;
        match tag {
            0 => Some(None),
            1 => Some(Some(Root::read(b, pos)?)),
            _ => None,
        }
    }
}
