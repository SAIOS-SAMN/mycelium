// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONSENSUS — the two-voice germ-admission gate (anastomosis). A germinated
//! motor installs to the shared basis only when QUORUM distinct independent
//! voices have derived the SAME germ (consilience). State is a keyed relation
//! (germ → the distinct voices that raised it).

use crate::network::nerve::{Keyed, ROOT};

/// Two independent mouths must meet for a germ to install (the anastomosis law).
pub const QUORUM: usize = 2;

/// A consilience receipt: an independent voice attesting it derived this germ.
/// The germ id is a wire encoding of the motor (transport identity), matched for
/// counting only — never reasoned over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub voice: [u8; 32],
    pub germ: crate::transducer::a21::Address,
}

/// The admission ledger: for each germ, the distinct voices that have attested
/// it. Keyed by germ, voices keyed by vk — no first-found scan.
#[derive(Debug, Default, Clone)]
pub struct Admission {
    seen: Keyed<Keyed<()>>,
}

impl Admission {
    pub fn new() -> Self {
        Admission { seen: Keyed::new() }
    }

    /// Record a receipt. Returns `true` at the MOMENT the germ reaches QUORUM
    /// distinct voices (the admission event) — false before, and false on every
    /// receipt after (idempotent: a germ admits once). Duplicate voices for the
    /// same germ do not count twice.
    pub fn record(&mut self, r: Receipt) -> bool {
        if let Some(voices) = self.seen.get_mut(r.germ.as_bytes()) {
            if voices.contains(&r.voice) {
                return false;
            }
            voices.insert(r.voice, ());
            return voices.len() == QUORUM;
        }
        let mut voices = Keyed::new();
        voices.insert(r.voice, ());
        self.seen.insert(r.germ.as_bytes(), voices);
        false
    }

    /// Is this germ admitted (has it reached quorum)?
    pub fn admitted(&self, germ: &[u8]) -> bool {
        self.seen.get(germ).is_some_and(|v| v.len() >= QUORUM)
    }

    /// Distinct voices on record for a germ.
    pub fn voices(&self, germ: &[u8]) -> usize {
        self.seen.get(germ).map(|v| v.len()).unwrap_or(0)
    }

    /// The distinct voices that attested a germ (for the consilience split).
    pub fn voices_of(&self, germ: &[u8]) -> Vec<[u8; 32]> {
        let Some(v) = self.seen.get(germ) else {
            return Vec::new();
        };
        v.fiber(ROOT)
            .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(voice: u8, germ: u8) -> Receipt {
        Receipt { voice: [voice; 32], germ: crate::transducer::a21::Address::from_bytes(&[germ]) }
    }

    #[test]
    fn a_germ_admits_only_on_the_second_independent_voice() {
        let mut a = Admission::new();
        let g = [7u8];
        assert!(!a.record(r(1, 7)), "one tip is not consilience");
        assert!(!a.admitted(&g));
        assert!(a.record(r(2, 7)), "the second independent tip admits it");
        assert!(a.admitted(&g));
        assert!(!a.record(r(3, 7)), "admission fires once, not per later receipt");
    }

    #[test]
    fn a_duplicate_voice_does_not_reach_quorum() {
        let mut a = Admission::new();
        assert!(!a.record(r(1, 7)));
        assert!(!a.record(r(1, 7)), "same voice twice is still one mouth");
        assert_eq!(a.voices(&[7u8]), 1);
        assert!(!a.admitted(&[7u8]));
    }
}
