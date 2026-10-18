// 28, 78.27, 78.49 — THE GENOME ON DISK: the words a court admitted, in T1's
// shape, keyed by their citations — crystallized knowledge and nothing else —
// and a tape that does not read whole refused rather than partly restored.

use crate::client::sdk::*;
use crate::snapshot::{leaf, put_trie, take_trie, Leaves};
use crate::transducer::element::Element;

/// THE GENOME FILE'S MAGIC (78.27, LD3-c, 78.49): the words a court admitted
/// as a trie keyed by `Cite::Word{chart, class, at}`, `at` the court's
/// admitting stamp — and nothing after them: a voice holds no ledger.
pub const GENOME_MAGIC: &[u8; 4] = b"GEN7";

/// THE GENOME THAT CARRIED A VOICE'S LEDGER (GEN6), refused BY NAME and never
/// migrated: its tail was the voice's own receipt chain, and a voice holds
/// none (78.49) — the court seals.
pub const LEDGER_GENOME_MAGIC: &[u8; 4] = b"GEN6";

/// THE POSITIONAL GENOME, refused BY NAME and never migrated: its words
/// stood in a counted list with a `0` tag for a word held with no admitting
/// stamp, and its ledger was read by position.
pub const PRE_TRIE_GENOME_MAGIC: &[u8; 4] = b"GEN5";

/// THE GENOME BEFORE THE ORBIT COORDINATES (ACT1-orbit), refused BY NAME
/// and never migrated, as `Element::PRE_WHOLE_MAGIC` and the snapshot's
/// `PRE_ORBIT_LEDGER_TAG` are: its stamps carry no place in the orbit, so
/// under this reader a stamp's height would be read into the next stamp's
/// orbit. One equality refuses it; there is no reader for it.
pub const PRE_ORBIT_GENOME_MAGIC: &[u8; 4] = b"GEN4";

/// The words trie and its one leaf: the element held once.
const WORDS: &str = "words";
const LEAF_HELD: u8 = 1;

/// What a genome file's magic names where this reader refuses it: the
/// positional and the pre-orbit genomes by their own names, anything else
/// as not a genome of this build. `None` for this build's magic.
pub(crate) fn genome_refused(bytes: &[u8]) -> Option<&'static str> {
    match bytes.get(..4) {
        Some(m) if m == GENOME_MAGIC => None,
        Some(m) if m == LEDGER_GENOME_MAGIC => {
            Some("GEN6, the genome that carried a voice's ledger (78.49): refused by name, the recut is the road")
        }
        Some(m) if m == PRE_TRIE_GENOME_MAGIC => {
            Some("GEN5, the positional genome (78.27): refused by name, the recut is the road")
        }
        Some(m) if m == PRE_ORBIT_GENOME_MAGIC => {
            Some("GEN4, the genome before the orbit coordinates (ACT1-orbit): refused by name, the recut is the road")
        }
        _ => Some("not a GEN7 tape"),
    }
}

/// WHAT THE GENOME WRITER REFUSED (LD0, T1's rule at the genome): each word
/// named once by its reason and counted, never written with a `0` tag.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GenomeRefused {
    /// Words held with no court's admitting stamp: no court admitted them,
    /// so they are not crystallized knowledge and have no coordinates to be
    /// filed at.
    pub unstamped: u64,
    /// Words whose citation cannot be written (a class key at its wall).
    pub unciteable: u64,
    /// Two words at one citation — one fiber, one admitting act.
    pub coincident: u64,
    /// STD1 3c — words whose record passes a length its field writes.
    pub unwritable: u64,
    /// MU-CLIENT §7.1 — residual chains: open questions, never words.
    pub residual: u64,
}

/// WHETHER AN ELEMENT IS A RESIDUAL CHAIN: its address leads with the
/// residual's tag — what perception could not say, an open question and never
/// a word (the operator, 2026-10-18).
pub(crate) fn is_residual(e: &Element) -> bool {
    matches!(e.address().as_bytes(), [tag, ..] if *tag == crate::transducer::address::TAG_RESIDUAL)
}

impl GenomeRefused {
    #[cfg(test)]
    pub(crate) fn total(&self) -> u64 {
        self.unstamped + self.unciteable + self.coincident + self.unwritable
    }

    /// Named once, with each count, where any is not zero.
    pub(crate) fn name(&self) {
        for (what, n) in [
            ("words no court admitted", self.unstamped),
            ("words whose citation cannot be written", self.unciteable),
            ("words coinciding at one citation", self.coincident),
            ("words whose record passes a length its field writes", self.unwritable),
            ("residual chains, open questions and never words", self.residual),
        ] {
            if n > 0 {
                eprintln!("  genome refused {n} {what}");
            }
        }
    }
}

impl Epigenome {
    /// FILE A WORD'S ADMITTING STAMP (78.24 (2b), 78.49): the court's stamp
    /// for a word it admitted — a syllabus page's. A word admitted once keeps
    /// its stamp.
    pub(crate) fn file_admitted(&mut self, addr: &crate::transducer::address::Address, at: crate::chronometry::Stamp) {
        if !self.admitted_at.contains(addr) {
            self.admitted_at.insert_shared(addr.shared(), at);
        }
    }

    /// The court's stamp a word this mind holds was admitted at.
    pub(crate) fn admitted_at(&self, addr: &[u8]) -> Option<crate::chronometry::Stamp> {
        self.admitted_at.get(addr).copied()
    }

    /// 78.158a, 78.157 §3a — A COURT-STAMPED WORD: the held word at `addr`
    /// where a court admitted it (its stamp filed, `admitted_at`), and none
    /// otherwise — never a dialect word, never a word learned with no stamp.
    /// The resolver a home check replays through, so a rule passes at home
    /// only on citations the court holds (`rule::would_judge_with`).
    pub(crate) fn court_word(&self, addr: &[u8]) -> Option<&crate::transducer::element::Held> {
        match self.admitted_at.contains(addr) {
            true => self.word_at(addr),
            false => None,
        }
    }

    /// FOR TESTS: every resident word filed at one court stamp, as a
    /// syllabus page carrying them would file them.
    #[cfg(test)]
    pub(crate) fn admit_all_at(&mut self, at: crate::chronometry::Stamp) {
        let words: Vec<Vec<u8>> = self.resident.scoped().map(|(k, _)| k).collect();
        for w in words {
            self.file_admitted(&crate::transducer::address::Address::from(w), at);
        }
    }

    /// CRYSTALLIZE the genome for persistence — the permanent language only;
    /// the epigenome is memory-only (78.27). The magic, then the words trie:
    /// each word at its citation `Cite::word(element, admitted)`, its one
    /// leaf the element. A word no court admitted has no coordinates and is
    /// refused by name and counted, never written.
    pub(crate) fn crystallize(&self) -> (Vec<u8>, GenomeRefused) {
        let mut out = GENOME_MAGIC.to_vec();
        let mut refused = GenomeRefused::default();
        let mut nodes: crate::network::nerve::Keyed<Leaves> = crate::network::nerve::Keyed::new();
        crate::snapshot::persist(&self.resident, |_, g| {
            let elem = g.held.element();
            // A residual chain is an open question, never crystallized.
            if is_residual(elem) {
                refused.residual += 1;
                return;
            }
            let Some(at) = self.admitted_at(g.held.address().as_bytes()) else {
                refused.unstamped += 1;
                return;
            };
            let mut key = Vec::new();
            if crate::chronometry::Cite::word(elem, at).put_canonical(&mut key).is_none() {
                refused.unciteable += 1;
                return;
            }
            if nodes.contains(&key) {
                refused.coincident += 1;
                return;
            }
            let Some(record) = g.held.to_bytes() else {
                refused.unwritable += 1;
                return;
            };
            nodes.insert(key, vec![(LEAF_HELD, record)]);
        });
        put_trie(&mut out, WORDS, &nodes);
        (out, refused)
    }

    /// Restore GEN6 only, whole or refused: an older file (named by
    /// [`genome_refused`]), a word that does not parse, or a word filed at a
    /// citation that is not its own refuses the tape (rule 11), and nothing
    /// is restored. Each word's admitting stamp is read off its key and
    /// filed; where the words end on the tape is answered.
    pub(crate) fn uncrystallize(&mut self, bytes: &[u8]) -> Option<usize> {
        if genome_refused(bytes).is_some() {
            return None;
        }
        let mut pos = 4usize;
        let words = take_trie(bytes, &mut pos, WORDS, &[LEAF_HELD])?;
        let mut genome = Vec::with_capacity(words.len());
        let mut admitted = Vec::with_capacity(words.len());
        for (key, leaves) in words.scoped() {
            let mut p = 0usize;
            let crate::chronometry::Cite::Word { at, .. } = crate::chronometry::Cite::take_canonical(&key, &mut p)?
            else {
                return None;
            };
            if p != key.len() {
                return None;
            }
            let record = leaf(leaves, LEAF_HELD)?;
            let mut ep = 0usize;
            let e = Element::from_bytes(record, &mut ep)?;
            if ep != record.len() {
                return None;
            }
            // The key is the word's own citation, re-derived.
            let mut again = Vec::new();
            crate::chronometry::Cite::word(&e, at).put_canonical(&mut again)?;
            if again != key {
                return None;
            }
            // MU-CLIENT §7.1 — A RESIDUAL CHAIN IS AN OPEN QUESTION, NEVER A
            // WORD (the operator, 2026-10-18): one an older genome file
            // holds is read whole and handed to the mind's carrier of open
            // questions, never admitted.
            if is_residual(&e) {
                let stratum = crate::transducer::stalk::invariant_of(&e).key().as_bytes().to_vec();
                self.restored_residuals.push((e.address().as_bytes().to_vec(), e.sub().to_vec(), stratum));
                continue;
            }
            admitted.push((e.address(), at));
            // The tape is decoded to a form HERE, once, as the word is
            // taken in — not on the first read of its form (L16).
            genome.push(crate::transducer::element::Held::of(e));
        }
        // AN EMPTY WORDS TRIE IS A GENOME WITH NO WORDS, not a refusal.
        self.genome = crate::network::nerve::Keyed::new();
        self.admitted_at = crate::network::nerve::Keyed::new();
        for (addr, at) in admitted {
            self.file_admitted(&addr, at);
        }
        self.resident = crate::network::nerve::Keyed::new();
        self.unpivoted = crate::network::nerve::Keyed::new();
        self.atlas = crate::transducer::stalk::Stalks::default();
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        // 21 step 3 (L8r): the body-chart index is derived from the genome
        // like the atlas and the axis index, and must be rebuilt with them.
        // Left empty, `on_body` answered nothing after every restart, so a
        // fleet client — which boots from its genome — could not glue until
        // it learned something new, and the index was live only in a
        // process that had never restarted.
        self.by_moduli = crate::network::nerve::Keyed::new();
        self.generators = crate::transducer::sentence::GeneratorIndex::new();
        // THE SAME ADMISSION THE LIVE PATH MAKES, once per word (`admit_word`),
        // so a restored genome and a learned one are indexed identically.
        // This used to file at the sentence's own chart while `axis_admit`
        // files where each vertex ACTS, so every word restored from disk
        // was invisible to the pivot until it happened to be re-admitted
        // live — and a fleet client boots from its genome.
        //
        // A RESTORED WORD IS WORK THE PIVOT HAS NOT REACHED, exactly as a
        // learned one is: `admit_word` puts its coordinate in the pending
        // set. A boot that skipped that left every restored word unpivoted
        // and unrecorded — the pivot could not tell them from words it had
        // already taken, because nothing said which was which.
        for w in genome {
            self.admit_word(w, 0);
        }
        self.atlas_len = self.genome.len();
        Some(pos)
    }

}
