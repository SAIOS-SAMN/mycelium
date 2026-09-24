// 28, 78.27 — THE GENOME ON DISK: the voice's words in T1's shape, keyed by
// their citations, and a tape that does not read whole refused rather than
// partly restored.

use crate::client::sdk::*;
use crate::snapshot::{leaf, put_trie, take_trie, Leaves};
use crate::transducer::element::Element;

/// THE GENOME FILE'S MAGIC (78.27, LD3-c): the voice's words as a trie keyed
/// by `Cite::Word{chart, class, at}`, then the voice's ledger as its tries —
/// the signer and its nodes by stamp.
pub const GENOME_MAGIC: &[u8; 4] = b"GEN6";

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
pub fn genome_refused(bytes: &[u8]) -> Option<&'static str> {
    match bytes.get(..4) {
        Some(m) if m == GENOME_MAGIC => None,
        Some(m) if m == PRE_TRIE_GENOME_MAGIC => {
            Some("GEN5, the positional genome (78.27): refused by name, the recut is the road")
        }
        Some(m) if m == PRE_ORBIT_GENOME_MAGIC => {
            Some("GEN4, the genome before the orbit coordinates (ACT1-orbit): refused by name, the recut is the road")
        }
        _ => Some("not a GEN6 tape"),
    }
}

/// WHAT THE GENOME WRITER REFUSED (LD0, T1's rule at the genome): each word
/// named once by its reason and counted, never written with a `0` tag.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GenomeRefused {
    /// Words held with no admitting stamp: no act admitted them, so they
    /// have no coordinates to be filed at.
    pub unstamped: u64,
    /// Words whose citation cannot be written (a class key at its wall).
    pub unciteable: u64,
    /// Two words at one citation — one fiber, one admitting act.
    pub coincident: u64,
}

impl GenomeRefused {
    pub fn total(&self) -> u64 {
        self.unstamped + self.unciteable + self.coincident
    }

    /// Named once, with each count, where any is not zero.
    pub fn name(&self) {
        for (what, n) in [
            ("words held with no admitting stamp", self.unstamped),
            ("words whose citation cannot be written", self.unciteable),
            ("words coinciding at one citation", self.coincident),
        ] {
            if n > 0 {
                eprintln!("  genome refused {n} {what}");
            }
        }
    }
}

/// What a restored genome hands back beside its words: each word's
/// admitting stamp (78.24 (2b)), read off its own key, and where the words
/// end on the tape.
pub struct Uncrystallized {
    pub admitted: Vec<(crate::transducer::address::Address, crate::chronometry::Stamp)>,
    pub end: usize,
}

impl Epigenome {
    /// CRYSTALLIZE the genome for persistence — the permanent language only;
    /// the epigenome is memory-only (78.27). The magic, then the words trie:
    /// each word at its citation `Cite::word(element, admitted)`, its one
    /// leaf the element. A word with no admitting stamp has no coordinates
    /// and is refused by name and counted, never written.
    pub fn crystallize(
        &self,
        admitted: impl Fn(&[u8]) -> Option<crate::chronometry::Stamp>,
    ) -> (Vec<u8>, GenomeRefused) {
        let mut out = GENOME_MAGIC.to_vec();
        let mut refused = GenomeRefused::default();
        let mut nodes: crate::network::nerve::Keyed<Leaves> = crate::network::nerve::Keyed::new();
        crate::snapshot::persist(&self.resident, |_, g| {
            let elem = g.held.element();
            let Some(at) = admitted(g.held.address().as_bytes()) else {
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
            nodes.insert(key, vec![(LEAF_HELD, g.held.to_bytes())]);
        });
        put_trie(&mut out, WORDS, &nodes);
        (out, refused)
    }

    /// Restore GEN6 only, whole or refused: an older file (named by
    /// [`genome_refused`]), a word that does not parse, or a word filed at a
    /// citation that is not its own refuses the tape (rule 11), and nothing
    /// is restored. Each word's admitting stamp is read off its key.
    pub fn uncrystallize(&mut self, bytes: &[u8]) -> Option<Uncrystallized> {
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
            admitted.push((e.address(), at));
            // The tape is decoded to a form HERE, once, as the word is
            // taken in — not on the first read of its form (L16).
            genome.push(crate::transducer::element::Held::of(e));
        }
        if genome.is_empty() {
            return None;
        }
        self.genome = crate::network::nerve::Keyed::new();
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
        Some(Uncrystallized { admitted, end: pos })
    }

}
