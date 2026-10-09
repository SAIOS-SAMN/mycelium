// 78.152 — THE KEEPER'S RECORD: the last reading this court signed, kept in
// its home beside the ledger, so a restarted keeper never signs a height at
// or below one it signed before (an equivocation) and its new line names that
// reading as its parent — peers extend the line rather than re-anchor it.

use super::point::Origin;
use super::reading::TickReading;

/// The record's magic.
pub const MAGIC: &[u8; 4] = b"MYKR";

/// The record's layout version.
pub const VERSION: u16 = 1;

/// THE LAST SIGNED READING, at the origin it was signed at, with its hash
/// (the citation its successor names as parent).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastSigned {
    pub origin: Origin,
    pub hash: [u8; 32],
    pub reading: TickReading,
}

impl LastSigned {
    /// The record for a reading signed at `origin`; `None` where the reading
    /// cannot be written.
    pub fn of(origin: Origin, reading: TickReading) -> Option<LastSigned> {
        let hash = reading.hash(&origin)?;
        Some(LastSigned { origin, hash, reading })
    }

    /// THE RECORD'S BYTES: `MYKR`, the version (u16 LE), the origin's
    /// genesis (32) and epoch (u32 LE), the keeper (32), the reading's hash
    /// (32), then the reading as the wire carries it (tick, parent, cites,
    /// signature). `None` where the reading cannot be written.
    pub fn encode(&self) -> Option<Vec<u8>> {
        let mut out = MAGIC.to_vec();
        out.extend_from_slice(&VERSION.to_le_bytes());
        out.extend_from_slice(&self.origin.genesis);
        out.extend_from_slice(&self.origin.epoch.to_le_bytes());
        out.extend_from_slice(&self.reading.keeper);
        out.extend_from_slice(&self.hash);
        self.reading.put(&mut out)?;
        Some(out)
    }

    /// THE RECORD READ BACK, exactly: the magic and version this build
    /// writes, every field whole, nothing after; the reading signed by its
    /// keeper at the recorded origin and hashing to the recorded hash.
    /// `None` for anything else — a record that does not read is no floor.
    pub fn decode(b: &[u8]) -> Option<LastSigned> {
        if b.get(..4)? != MAGIC.as_slice() {
            return None;
        }
        if u16::from_le_bytes(b.get(4..6)?.try_into().ok()?) != VERSION {
            return None;
        }
        let genesis: [u8; 32] = b.get(6..38)?.try_into().ok()?;
        let epoch = u32::from_le_bytes(b.get(38..42)?.try_into().ok()?);
        let keeper: [u8; 32] = b.get(42..74)?.try_into().ok()?;
        let hash: [u8; 32] = b.get(74..106)?.try_into().ok()?;
        let mut p = 106usize;
        let reading = TickReading::take(keeper, b, &mut p)?;
        if p != b.len() {
            return None;
        }
        let origin = Origin { genesis, epoch };
        if !reading.verifies(&origin) || reading.hash(&origin)? != hash {
            return None;
        }
        Some(LastSigned { origin, hash, reading })
    }
}

/// WHAT A HOME'S KEEPER RECORD READS: none (a keeper that never signed
/// here), the record, or bytes that do not read — which a boot refuses, for
/// a floor it cannot read is one it could sign under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recorded {
    None,
    Last(Box<LastSigned>),
    Unreadable(String),
}

/// The record at `path`, read.
pub fn read(path: &std::path::Path) -> Recorded {
    match std::fs::read(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Recorded::None,
        Err(e) => Recorded::Unreadable(format!("{e}")),
        Ok(b) => match LastSigned::decode(&b) {
            Some(r) => Recorded::Last(Box::new(r)),
            None => Recorded::Unreadable(format!("{} bytes do not read as a {} v{VERSION} keeper record", b.len(), "MYKR")),
        },
    }
}

/// The record written at `path` durably (tmp, synced, renamed, the
/// directory synced). Whether it landed.
pub fn write(path: &std::path::Path, rec: &LastSigned) -> bool {
    rec.encode().is_some_and(|b| crate::chronometry::replace_durably(path, &b))
}
