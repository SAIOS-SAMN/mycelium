// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, ROADMAP track 24 B2 — THE SIGNINGS OF THE PAST, CARRIED
// FORWARD AS HISTORY: what the kernels agreed before signing by adjudicated
// reading, stated as fact and never admitted as a signature now.
//
//! At v0.3.0 a kernel signed the Concordance's whole text, by its version
//! and a signature over the hash of the file's bytes (`CONCORDANCE_HASH`,
//! retired in v0.4.0). A hash proves no reading (Concordance §0;
//! MU-ANTI-FLATTENING 13), so those signatures cannot be reconstructed and
//! no clause reading is derived from them: the alignment reading
//! (`alignment::read_alignment`) reads only the signing events on the
//! ledger. They are served beside the reading as history, each naming the
//! kernel, the Concordance version it signed, the release and the mechanism
//! it signed by. Each kernel signs the v0.4.0 register, clause by clause, at
//! the release candidate.

use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};

/// HOW A SIGNING OF THE PAST WAS MADE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mechanism {
    /// A signature over the hash of the whole text's bytes, the version
    /// named beside it: retired in v0.4.0, never reconstructed.
    TextHash,
}

impl Mechanism {
    /// The mechanism's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            Mechanism::TextHash => 0,
        }
    }

    /// The mechanism a byte names, or none.
    pub const fn of(b: u8) -> Option<Mechanism> {
        match b {
            0 => Some(Mechanism::TextHash),
            _ => None,
        }
    }
}

/// ONE SIGNING OF THE PAST: the kernel by its name, the Concordance version
/// it signed, the release it signed at, and the mechanism. A fact of
/// history, never a signature the hypha admits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoricalSigning {
    pub kernel: String,
    pub concordance: u16,
    pub release: String,
    pub mechanism: Mechanism,
}

/// THE SIGNINGS OF THE PAST, in the order the record states them:
/// netstratum and xylarium signed Concordance version 4 at v0.3.0, by the
/// hash of its text.
pub fn history() -> Vec<HistoricalSigning> {
    ["netstratum", "xylarium"]
        .into_iter()
        .map(|kernel| HistoricalSigning { kernel: kernel.to_string(), concordance: 4, release: "v0.3.0".to_string(), mechanism: Mechanism::TextHash })
        .collect()
}

/// The history written: `[count][(kernel, version u16 LE, release, mechanism) …]`.
pub fn put_history(out: &mut Vec<u8>, history: &[HistoricalSigning]) {
    put_key_count(out, history.len());
    for h in history {
        put_key_bytes(out, h.kernel.as_bytes());
        out.extend_from_slice(&h.concordance.to_le_bytes());
        put_key_bytes(out, h.release.as_bytes());
        out.push(h.mechanism.byte());
    }
}

/// The history read back from `p` on.
pub fn take_history(b: &[u8], p: &mut usize) -> Option<Vec<HistoricalSigning>> {
    let n = take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let kernel = String::from_utf8(take_key_bytes(b, p)?).ok()?;
            let concordance = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
            *p += 2;
            let release = String::from_utf8(take_key_bytes(b, p)?).ok()?;
            let mechanism = Mechanism::of(*b.get(*p)?)?;
            *p += 1;
            Some(HistoricalSigning { kernel, concordance, release, mechanism })
        })
        .collect()
}
