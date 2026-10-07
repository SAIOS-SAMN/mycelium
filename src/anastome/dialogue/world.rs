// 78.120 — ACT_WORLD: a world founded on a court (2(d), 78.62's GENESIS act;
// the operator's word 2026-10-07: "proceed").
//
//! A WORLD'S GENESIS names the world's key and the tier keys that settle for
//! it, signed by the world's key and sealed on the court's line
//! (`Cite::World`, keys only — coordinates). From then on a tier's fold
//! (`ACT_UPLINK`) is admitted only from a key a sealed genesis names: a
//! fold's signature means the world stands behind the tier. A world is
//! founded once; granting and revoking tier keys are later rows.

use crate::chronometry::Stamp;

/// The signed bytes' domain, so a genesis signature is never a signature on
/// anything else.
const DOMAIN: &[u8] = b"mycelium world genesis v1";

/// A WORLD'S GENESIS: its key, and its tier keys in their own ascending
/// order, each once.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldGenesis {
    pub world: [u8; 32],
    pub tiers: Vec<[u8; 32]>,
}

/// The genesis and the world key's signature over [`WorldGenesis::signed_bytes`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedWorld {
    pub genesis: WorldGenesis,
    pub sig: [u8; 64],
}

/// Keys written canonically: their count, then each.
pub(crate) fn put_keys(out: &mut Vec<u8>, keys: &[[u8; 32]]) {
    crate::transducer::address::put_key_count(out, keys.len());
    for k in keys {
        out.extend_from_slice(k);
    }
}

/// Keys read back one by one, never reserved by the count.
pub(crate) fn take_keys(b: &[u8], p: &mut usize) -> Option<Vec<[u8; 32]>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        })
        .collect()
}

impl WorldGenesis {
    /// Whether the tier keys stand in their one writing: ascending, each once.
    pub fn ordered(&self) -> bool {
        self.tiers.windows(2).all(|w| matches!(w, [a, b] if a < b))
    }

    /// The bytes the world signs: the domain, the world, its tier keys.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = DOMAIN.to_vec();
        out.extend_from_slice(&self.world);
        put_keys(&mut out, &self.tiers);
        out
    }

    /// The genesis signed by the world's key — whose public half is `world`.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedWorld {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedWorld { genesis: self, sig }
    }
}

/// `[ACT_WORLD][world 32][tiers][sig 64]`.
pub fn encode_world(w: &SignedWorld) -> Vec<u8> {
    let mut out = vec![super::ACT_WORLD];
    out.extend_from_slice(&w.genesis.world);
    put_keys(&mut out, &w.genesis.tiers);
    out.extend_from_slice(&w.sig);
    out
}

/// The genesis a frame carries, read whole: a byte left over is none.
pub fn decode_world(b: &[u8]) -> Option<SignedWorld> {
    let rest = b.strip_prefix(&[super::ACT_WORLD])?;
    let mut p = 0;
    let world: [u8; 32] = rest.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let tiers = take_keys(rest, &mut p)?;
    let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    (p == rest.len()).then_some(SignedWorld { genesis: WorldGenesis { world, tiers }, sig })
}

/// WHAT THE COURT ANSWERED a world's genesis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Founded {
    /// Sealed on the court's line: the receipt's stamp and chain hash.
    Sealed { at: Stamp, receipt: [u8; 32] },
    /// The world's signature does not verify over the genesis.
    Unsigned,
    /// The tier keys are not ascending, each once.
    Unordered,
    /// This world is founded on this court already.
    AlreadyFounded,
    /// A tier key is named by another world's genesis already: a tier
    /// settles for one world.
    TierHeld,
    /// The court's line could not take it.
    LineRefused,
}

/// `[0][stamp][hash]` sealed; `[1]` unsigned; `[2]` unordered; `[3]` already
/// founded; `[4]` a tier held; `[5]` the line refused.
pub fn encode_world_reply(f: &Founded) -> Vec<u8> {
    let mut out = Vec::new();
    match f {
        Founded::Sealed { at, receipt } => {
            out.push(0);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
        }
        Founded::Unsigned => out.push(1),
        Founded::Unordered => out.push(2),
        Founded::AlreadyFounded => out.push(3),
        Founded::TierHeld => out.push(4),
        Founded::LineRefused => out.push(5),
    }
    out
}

/// The reply read whole, or none.
pub fn decode_world_reply(b: &[u8]) -> Option<Founded> {
    let [tag, ..] = b else { return None };
    let mut p = 1;
    let f = match *tag {
        0 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Founded::Sealed { at, receipt }
        }
        1 => Founded::Unsigned,
        2 => Founded::Unordered,
        3 => Founded::AlreadyFounded,
        4 => Founded::TierHeld,
        5 => Founded::LineRefused,
        _ => return None,
    };
    (p == b.len()).then_some(f)
}
