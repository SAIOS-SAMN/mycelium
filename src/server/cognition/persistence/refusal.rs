// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ANTI-FLATTENING 14, ROADMAP track 25 X0 — WHY A SNAPSHOT DID NOT
//! RECONSTRUCT A COURT, by name: a retired layout, a tape that is no ledger,
//! or the trie and byte its reading stopped at. Nothing is partly restored.

/// HOW A SNAPSHOT IS REFUSED (`Court::reconstruct`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreRefusal {
    /// A layout an older build wrote, refused by its magic: the recut is
    /// the road, and the file is not corrupt.
    Retired { magic: &'static str, layout: &'static str },
    /// The tape opens with no ledger's magic: the four bytes it carries,
    /// or `None` where it is shorter than four, with its length.
    NotLedger { found: Option<[u8; 4]>, len: usize },
    /// A trie that does not read whole, or a check of the laws that does
    /// not hold on it: the trie the read stood in and the byte it stopped
    /// at, of the tape's length.
    Unread { trie: &'static str, at: usize, of: usize },
}

impl std::fmt::Display for RestoreRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestoreRefusal::Retired { magic, layout } => {
                write!(f, "ledger refused: {magic}, {layout} — refused by name, the recut is the road")
            }
            RestoreRefusal::NotLedger { found: Some(m), .. } => {
                let want = String::from_utf8_lossy(super::MAGIC);
                match std::str::from_utf8(m) {
                    Ok(s) => write!(f, "ledger refused: the magic is `{s}`, not {want}"),
                    Err(_) => write!(f, "ledger refused: the magic is {m:02x?}, which is not text, not {want}"),
                }
            }
            RestoreRefusal::NotLedger { found: None, len } => {
                write!(f, "ledger refused: the magic is {len} bytes, too short to carry one")
            }
            RestoreRefusal::Unread { trie, at, of } => {
                write!(f, "ledger refused in trie {trie} at byte {at} of {of}: the court starts clean")
            }
        }
    }
}
