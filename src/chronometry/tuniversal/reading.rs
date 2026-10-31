// 78.152 TU-1, TU-2 — A KEEPER'S READING IS A RECEIPT: the keeper court's
// point at its head, signed, linked to its previous reading by hash, citing
// the last reading it verified from each other keeper.

use super::point::{Origin, TPoint};

/// The signing domain of every keeper reading.
pub const SIGNED: &[u8] = b"mycelium tuniversal v1";

/// A KEEPER'S POINT AT ITS HEAD (TU-1): `h` its head's height, `lanes` its
/// court's cumulative issuance per lane (`Treasury::issued`: the discharge it
/// booked as mint, on the genesis it stood on), `k` the orbits it closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tick {
    pub h: u64,
    pub lanes: [u64; 4],
    pub k: u64,
}

impl Tick {
    /// The tick as a point at an origin.
    pub fn at(&self, origin: Origin) -> TPoint {
        TPoint { origin, h: self.h, lanes: self.lanes, k: self.k }
    }

    /// Did the measure move between two ticks: a lane or `K`. The tick
    /// alone moving is no measured change (TU-1).
    pub fn measure_moved(&self, from: &Tick) -> bool {
        self.lanes != from.lanes || self.k != from.k
    }
}

/// A READING ANOTHER KEEPER VERIFIED, as a reading cites it: that keeper,
/// the reading's hash, and its tick's height.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cited {
    pub keeper: [u8; 32],
    pub reading: [u8; 32],
    pub h: u64,
}

/// ONE SIGNED READING (TU-2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickReading {
    pub keeper: [u8; 32],
    pub tick: Tick,
    /// The hash of this keeper's previous reading; the empty hash where the
    /// keeper's line of readings (re)starts.
    pub parent: [u8; 32],
    pub cites: Vec<Cited>,
    pub sig: [u8; 64],
}

impl TickReading {
    /// The least bytes one reading takes on the wire: its tick, parent, cite
    /// count and signature.
    pub const MIN_WIRE: usize = 8 + 32 + 8 + 32 + 1 + 64;

    /// The bytes the keeper signs: the domain, the ORIGIN (a reading of one
    /// universe never verifies in another), the keeper, the tick, the parent
    /// and the cites. `None` where the cites pass the count byte.
    pub fn signed_bytes(&self, origin: &Origin) -> Option<Vec<u8>> {
        let mut out = SIGNED.to_vec();
        out.extend_from_slice(&origin.genesis);
        out.extend_from_slice(&origin.epoch.to_le_bytes());
        out.extend_from_slice(&self.keeper);
        self.put_body(&mut out)?;
        Some(out)
    }

    /// The tick, parent and cites, as both the signature and the wire carry
    /// them.
    fn put_body(&self, out: &mut Vec<u8>) -> Option<()> {
        out.extend_from_slice(&self.tick.h.to_le_bytes());
        for l in self.tick.lanes {
            out.extend_from_slice(&l.to_le_bytes());
        }
        out.extend_from_slice(&self.tick.k.to_le_bytes());
        out.extend_from_slice(&self.parent);
        out.push(u8::try_from(self.cites.len()).ok()?);
        for c in &self.cites {
            out.extend_from_slice(&c.keeper);
            out.extend_from_slice(&c.reading);
            out.extend_from_slice(&c.h.to_le_bytes());
        }
        Some(())
    }

    /// THE READING'S HASH — its citation, the parent its successor names:
    /// the citation hash of exactly its signed bytes (as a seat call's
    /// reference is), never its identity (law 2).
    pub fn hash(&self, origin: &Origin) -> Option<[u8; 32]> {
        Some(crate::chronometry::payload_hash(&self.signed_bytes(origin)?))
    }

    /// A reading signed by `sk` at `origin`.
    pub fn sign(sk: &ed25519_dalek::SigningKey, origin: &Origin, tick: Tick, parent: [u8; 32], cites: Vec<Cited>) -> Option<TickReading> {
        use ed25519_dalek::Signer as _;
        let mut r = TickReading { keeper: sk.verifying_key().to_bytes(), tick, parent, cites, sig: [0; 64] };
        r.sig = sk.sign(&r.signed_bytes(origin)?).to_bytes();
        Some(r)
    }

    /// Whether its keeper signed it at `origin`.
    pub fn verifies(&self, origin: &Origin) -> bool {
        self.signed_bytes(origin).is_some_and(|m| crate::court::identity::verify(&self.keeper, &m, &self.sig))
    }

    /// The reading on the wire, its keeper left to the frame that carries it.
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        self.put_body(out)?;
        out.extend_from_slice(&self.sig);
        Some(())
    }

    /// A reading read at `p`, signed (the caller verifies) by `keeper`.
    pub fn take(keeper: [u8; 32], b: &[u8], p: &mut usize) -> Option<TickReading> {
        let u64_at = |p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let h = u64_at(p)?;
        let mut lanes = [0u64; 4];
        for l in &mut lanes {
            *l = u64_at(p)?;
        }
        let k = u64_at(p)?;
        let parent: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        let n = usize::from(*b.get(*p)?);
        *p += 1;
        let mut cites = Vec::with_capacity(n);
        for _ in 0..n {
            let keeper: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            let reading: [u8; 32] = b.get(*p + 32..*p + 64)?.try_into().ok()?;
            *p += 64;
            cites.push(Cited { keeper, reading, h: u64_at(p)? });
        }
        let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
        *p += 64;
        Some(TickReading { keeper, tick: Tick { h, lanes, k }, parent, cites, sig })
    }
}

/// A KEEPER'S RING on the cover digest: its recent readings, oldest first,
/// `[n u8]` then each reading. `None` where a reading cannot be written.
pub fn put_ring(out: &mut Vec<u8>, ring: &[TickReading]) -> Option<()> {
    out.push(u8::try_from(ring.len()).ok()?);
    for r in ring {
        r.put(out)?;
    }
    Some(())
}

/// A ring read at `p`, its readings the keeper's.
pub fn take_ring(keeper: [u8; 32], b: &[u8], p: &mut usize) -> Option<Vec<TickReading>> {
    let n = usize::from(*b.get(*p)?);
    *p += 1;
    if n > b.len().checked_sub(*p)? / TickReading::MIN_WIRE {
        return None;
    }
    (0..n).map(|_| TickReading::take(keeper, b, p)).collect()
}
