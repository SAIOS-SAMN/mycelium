// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONSENSUS — the two-voice germ-admission gate (anastomosis). A germinated
//! motor installs to the shared basis only when QUORUM distinct independent
//! voices have derived the SAME germ (consilience). State is a Vec of relations
//! (germ → the distinct voices that raised it), never a digest-keyed map: the
//! admission is a decision on the retained relation, not a bucket dereference.

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
/// it. Vec-of-relations (Law II) — no map in the admission path.
#[derive(Debug, Default, Clone)]
pub struct Admission {
    seen: Vec<(crate::transducer::a21::Address, Vec<[u8; 32]>)>,
}

impl Admission {
    pub fn new() -> Self {
        Admission { seen: Vec::new() }
    }

    /// Record a receipt. Returns `true` at the MOMENT the germ reaches QUORUM
    /// distinct voices (the admission event) — false before, and false on every
    /// receipt after (idempotent: a germ admits once). Duplicate voices for the
    /// same germ do not count twice.
    pub fn record(&mut self, r: Receipt) -> bool {
        let slot = match self.seen.iter_mut().find(|(g, _)| *g == r.germ) {
            Some(s) => s,
            None => {
                self.seen.push((r.germ.clone(), Vec::new()));
                self.seen.last_mut().unwrap()
            }
        };
        if slot.1.contains(&r.voice) {
            return false; // same voice, no new consilience
        }
        let before = slot.1.len();
        slot.1.push(r.voice);
        before + 1 == QUORUM // the crossing event, exactly once
    }

    /// Is this germ admitted (has it reached quorum)?
    pub fn admitted(&self, germ: &[u8]) -> bool {
        self.seen.iter().any(|(g, v)| g.as_bytes() == germ && v.len() >= QUORUM)
    }

    /// Distinct voices on record for a germ.
    pub fn voices(&self, germ: &[u8]) -> usize {
        self.seen.iter().find(|(g, _)| g.as_bytes() == germ).map_or(0, |(_, v)| v.len())
    }

    /// The distinct voices that attested a germ (for the consilience split).
    pub fn voices_of(&self, germ: &[u8]) -> Vec<[u8; 32]> {
        self.seen.iter().find(|(g, _)| g.as_bytes() == germ).map_or_else(Vec::new, |(_, v)| v.clone())
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
