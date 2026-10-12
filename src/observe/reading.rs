// §8.1, audit of Phase N — THE READINGS AN OBSERVER CARRIES, each a whole
// typed object in its own coordinates and written out by its own codec.
//
//! The readings a court publishes about itself, one per question the live
//! fleet is asked:
//!
//! - [`Memory`] — WHAT IS HELD: the allocator's bytes at the first and the
//!   latest sample, keyed by the source's own `K`, and the peak — so growth
//!   is read per unit of the source's own clock, never wall time (law 9).
//! - [`Court`] — WHAT A COURT HOLDS: its `K`, basis and treasury, and the
//!   acts it has served by kind.

/// A reading's tag, the first byte of its writing and the last byte of its
/// key.
pub const TAG_MEMORY: u8 = 3;
pub const TAG_COURT: u8 = 4;
pub const TAG_REPLICATION: u8 = 5;
pub const TAG_ADJUDICATION: u8 = 6;
pub const TAG_PHASE: u8 = 13;
pub const TAG_CARRIAGE: u8 = 14;
pub const TAG_CLOCK: u8 = 15;
pub const TAG_ECONOMY: u8 = 16;
/// 2026-09-29 — a court's acts and metabolism.
pub const TAG_ACTS: u8 = 18;
pub const TAG_CARRIERS: u8 = 19;
/// 78.145 — the seat relay's standing and its posts filed and refused.
pub const TAG_RELAY: u8 = 20;
/// MU-CLIENT §5.3 — a mind's conversation and what it built.
pub const TAG_MIND: u8 = 21;
/// MU-CLIENT §5.5 — the agreed language a court attested, per pair.
pub const TAG_AGREEMENTS: u8 = 22;
/// The age of a court's published standing, in questions (`Reading::Asked`).
pub const TAG_ASKED: u8 = 23;

/// CQ1 — EVERY READING'S NAME, by its tag: the ONE statement the console's
/// `--reading NAME` is read against (law 12).
pub const READING_NAMES: [(&str, u8); 14] = [
    ("asked", TAG_ASKED),
    ("mind", TAG_MIND),
    ("agreements", TAG_AGREEMENTS),
    ("carriers", TAG_CARRIERS),
    ("relay", TAG_RELAY),
    ("clock", TAG_CLOCK),
    ("economy", TAG_ECONOMY),
    ("acts", TAG_ACTS),
    ("memory", TAG_MEMORY),
    ("court", TAG_COURT),
    ("replication", TAG_REPLICATION),
    ("adjudication", TAG_ADJUDICATION),
    ("phase", TAG_PHASE),
    ("carriage", TAG_CARRIAGE),
];

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn take_u32(b: &[u8], p: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

/// Does the body from `at` hold `m` entries of `item` bytes each?
fn holds(b: &[u8], at: usize, m: usize, item: usize) -> bool {
    match (b.len().checked_sub(at), m.checked_mul(item)) {
        (Some(rest), Some(need)) => rest >= need,
        _ => false,
    }
}

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

/// THE ALLOCATOR'S BYTES OVER THE SOURCE'S OWN CLOCK: the first sample and
/// the latest, each at the `K` it was taken, the peak resident, and how
/// many samples. Constant in size however long the source runs, so the
/// observer holds no history of its own to leak.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pub first_k: u64,
    pub first_allocated: u64,
    pub first_resident: u64,
    pub last_k: u64,
    pub last_allocated: u64,
    pub last_resident: u64,
    pub peak_resident: u64,
    pub samples: u64,
}

impl Memory {
    /// One sample of the allocator at the source's `K`. `None` where the
    /// allocator does not answer (a process not running jemalloc).
    pub fn now(k: u64) -> Option<Memory> {
        tikv_jemalloc_ctl::epoch::advance().ok()?;
        let allocated = tikv_jemalloc_ctl::stats::allocated::read().ok()? as u64;
        let resident = tikv_jemalloc_ctl::stats::resident::read().ok()? as u64;
        Some(Memory {
            first_k: k,
            first_allocated: allocated,
            first_resident: resident,
            last_k: k,
            last_allocated: allocated,
            last_resident: resident,
            peak_resident: resident,
            samples: 1,
        })
    }

    /// Keep the first sample, take the later one as the latest, and the
    /// larger peak.
    pub fn absorb(&mut self, o: &Memory) {
        if self.samples == 0 {
            *self = o.clone();
            return;
        }
        if o.last_k >= self.last_k {
            self.last_k = o.last_k;
            self.last_allocated = o.last_allocated;
            self.last_resident = o.last_resident;
        }
        self.peak_resident = self.peak_resident.max(o.peak_resident);
        self.samples += o.samples;
    }

    fn fields(&self) -> [u64; 8] {
        [
            self.first_k,
            self.first_allocated,
            self.first_resident,
            self.last_k,
            self.last_allocated,
            self.last_resident,
            self.peak_resident,
            self.samples,
        ]
    }

    fn put(&self, out: &mut Vec<u8>) {
        for v in self.fields() {
            put_u64(out, v);
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Memory> {
        let mut f = [0u64; 8];
        for v in f.iter_mut() {
            *v = take_u64(b, p)?;
        }
        let [first_k, first_allocated, first_resident, last_k, last_allocated, last_resident, peak_resident, samples] =
            f;
        Some(Memory {
            first_k,
            first_allocated,
            first_resident,
            last_k,
            last_allocated,
            last_resident,
            peak_resident,
            samples,
        })
    }
}

/// WHAT A COURT HOLDS at the moment it is asked: its chain's `K`, its ruled
/// basis, its treasury entries, and each act kind it has served with the
/// count, ascending by kind.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Court {
    pub k: u64,
    pub basis: u64,
    pub treasury: u64,
    pub served: Vec<(u8, u64)>,
    // The head's stamp and the rulings count are the clock's (ACT1,
    // [`super::Clock`]), read there and never copied here (law 12).
    /// 78.26(6), TP1 — the closed rings of the cover's nerve a moved gauge
    /// reopened at the glue, by what each cut: `[filled, congruent]`.
    pub reopened: [u64; 2],
    /// RM1 — the court's remainders over its complexes, a chart per complex;
    /// and the words carrying a torsion cycle `file_torsion` recorded — ℓ
    /// alone, κ and g dropped — against the Congruent simplices they came
    /// from.
    pub remainders: crate::transducer::cech::RemainderReading,
    pub torsion_words: u64,
    /// STD1 — the room reads that did not answer, each by its own name (a
    /// `None` never stands for two facts, 78.26(4)): a lane drawn past its
    /// cover (a broken count), a cover or a credit past its carrier; and
    /// the court's own operations and readings refused past their carrier
    /// (an occupy, a recall owed, the book's T pressure, a relocation's
    /// total, a solve's paid sum).
    pub room_broken: u64,
    pub room_overflowed: u64,
    pub overflowed: u64,
    /// THE FOLD'S ROUND BY OUTCOME, cumulative (LD0: a refusal is never
    /// discarded): proposed, refused open, refused inexact, committed,
    /// refused by a witness or at the commit, unmeasured, refused idle (no
    /// work in the prefix), refused with no transition (no return and no rank
    /// step in the prefix, 78.68).
    pub folds: [u64; FOLD_OUTCOMES.len()],
    /// PHASE 1 (78.59) — THE CATCH-UP FETCHES BY PATH AND OUTCOME: a line's
    /// back-fill and a fold witness's prefix, each asked of the court that
    /// owns the line, on [`CATCHUP_PATHS`] × [`CATCHUP_OUTCOMES`]. Both
    /// read one `None` for four different failures; the four are counted
    /// apart (a `None` never stands for two facts, 78.26(4)).
    pub catchup: [[u64; CATCHUP_OUTCOMES.len()]; CATCHUP_PATHS.len()],
    /// 2026-10-08 — THE SERVING SIDE OF A CATCH-UP PAGE, on [`SERVE_AXES`]:
    /// what this court spent answering a range of a line (the cold archive
    /// paged, in eras and receipts — the court's own work, never a clock) and
    /// whether the answer was WRITTEN, or found the asker gone (a page that
    /// missed the asker's wait). The asking side is `catchup`.
    pub serve: [u64; SERVE_AXES.len()],
    /// 78.152 — THE DOOR'S QUORUM-FINAL TUNIVERSAL POINT, its advances, the
    /// keepers as this court holds them and the residual (TU-3, TU-4, TU-7),
    /// read off the shelf and the keepers outside the kernel.
    pub tuniversal: super::TStanding,
}

/// The axes of [`Court::serve`]: counts, and one level (the most eras one
/// page paged).
/// 78.147: a catch-up opening below the signer's fold root is answered with
/// the root ("served root"); the cold archive answers only an era replay
/// ("replays"); a catch-up below a root this court holds no proof of is
/// answered hot alone ("unproved root").
/// 78.151: the reading the court stroke publishes for the door — each
/// publication, the receipts, eras, accounts and per-signer keys it copied
/// and their bytes, the most entries one publication copied (a level), a
/// publication whose count passed its carrier; the reads answered off it,
/// and the pages it answered empty (a range it did not yet reach).
pub const SERVE_AXES: [&str; 20] = [
    "pages",
    "hot only",
    "cold",
    "eras paged",
    "receipts cold",
    "unwritten",
    "most eras one page",
    "served root",
    "replays",
    "unproved root",
    "publishes",
    "published receipts",
    "published eras",
    "published accounts",
    "published keys",
    "published bytes",
    "most copied one publish",
    "publish count passed",
    "reads off published",
    "empty off published",
];

/// Where each serving count is written on [`SERVE_AXES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeAxis {
    Pages = 0,
    HotOnly = 1,
    Cold = 2,
    ErasPaged = 3,
    ReceiptsCold = 4,
    Unwritten = 5,
    /// A LEVEL: raised to the most eras one page paged, never summed.
    MostEras = 6,
    ServedRoot = 7,
    Replays = 8,
    UnprovedRoot = 9,
    Publishes = 10,
    PublishedReceipts = 11,
    PublishedEras = 12,
    PublishedAccounts = 13,
    PublishedKeys = 14,
    PublishedBytes = 15,
    /// A LEVEL: raised to the most entries one publication copied.
    MostPublished = 16,
    PublishPassed = 17,
    ReadsPublished = 18,
    EmptyPublished = 19,
}

/// The catch-up paths of [`Court::catchup`].
pub const CATCHUP_PATHS: [&str; 2] = ["back-fill", "fold witness"];
/// The outcomes of one catch-up page asked of the owning court.
pub const CATCHUP_OUTCOMES: [&str; 5] = ["page", "unanswered", "deferred", "undecoded", "empty"];

/// Where one catch-up page is counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatchupPath {
    Backfill = 0,
    FoldWitness = 1,
}

/// What one catch-up page came back as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatchupOutcome {
    Page = 0,
    Unanswered = 1,
    Deferred = 2,
    Undecoded = 3,
    Empty = 4,
}

/// THE AXES OF [`Court::folds`], in its writing's order.
pub const FOLD_OUTCOMES: [&str; 8] =
    ["proposed", "open", "inexact", "folded", "refused", "unmeasured", "idle", "no-transition"];

/// Where each fold outcome is counted on [`FOLD_OUTCOMES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldOutcome {
    Proposed = 0,
    Open = 1,
    Inexact = 2,
    Folded = 3,
    Refused = 4,
    Unmeasured = 5,
    Idle = 6,
    NoTransition = 7,
}

impl Court {
    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        put_u64(out, self.k);
        put_u64(out, self.basis);
        put_u64(out, self.treasury);
        crate::transducer::address::put_u32_count(out, self.served.len())?;
        for &(kind, n) in &self.served {
            out.push(kind);
            put_u64(out, n);
        }
        for n in self.reopened {
            put_u64(out, n);
        }
        self.remainders.put(out)?;
        put_u64(out, self.torsion_words);
        put_u64(out, self.room_broken);
        put_u64(out, self.room_overflowed);
        put_u64(out, self.overflowed);
        for n in self.folds {
            put_u64(out, n);
        }
        for n in self.catchup.iter().flatten() {
            put_u64(out, *n);
        }
        for n in self.serve {
            put_u64(out, n);
        }
        self.tuniversal.put(out)
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Court> {
        let (k, basis, treasury) = (take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?);
        let m = take_u32(b, p)? as usize;
        if !holds(b, *p, m, 9) {
            return None;
        }
        let mut served = Vec::with_capacity(m);
        for _ in 0..m {
            served.push((take_u8(b, p)?, take_u64(b, p)?));
        }
        let reopened = [take_u64(b, p)?, take_u64(b, p)?];
        let remainders = crate::transducer::cech::RemainderReading::take(b, p)?;
        let torsion_words = take_u64(b, p)?;
        let (room_broken, room_overflowed, overflowed) = (take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?);
        let mut folds = [0u64; FOLD_OUTCOMES.len()];
        for n in &mut folds {
            *n = take_u64(b, p)?;
        }
        let mut catchup = [[0u64; CATCHUP_OUTCOMES.len()]; CATCHUP_PATHS.len()];
        for n in catchup.iter_mut().flatten() {
            *n = take_u64(b, p)?;
        }
        let mut serve = [0u64; SERVE_AXES.len()];
        for n in &mut serve {
            *n = take_u64(b, p)?;
        }
        let tuniversal = super::TStanding::take(b, p)?;
        Some(Court {
            k,
            basis,
            treasury,
            served,
            reopened,
            remainders,
            torsion_words,
            room_broken,
            room_overflowed,
            overflowed,
            folds,
            catchup,
            serve,
            tuniversal,
        })
    }
}

/// THE AXES OF [`Replication`], in its writing's order (CV5): what a court
/// did with the receipts of one signer. The ONE statement of the order
/// (law 12), read by the counters, the codec and every renderer.
pub const REPLICATION_AXES: [(&str, &str); REP_AXES] = [
    ("parent", "receipts admitted from a digest, cited as a parent"),
    ("backfill", "receipts admitted by back-fill, page by page"),
    ("gap", "frames that reported a hole in a chain (the back-fill chases it)"),
    ("stale", "frames at a K already held with the same hash"),
    ("equivocation", "two hashes at one K on one chain (47.4)"),
    ("refused_origin", "receipts of another genesis"),
    ("refused_signature", "receipts whose signature did not verify"),
    ("refused_receipt", "receipts refused on their chain"),
    ("no_section", "digests carrying no receipt section"),
    ("malformed", "receipt sections that did not parse whole"),
    ("no_origin", "frames heard before this court adopted an origin"),
    ("digest_unreadable", "digests that were not one cover digest"),
    ("digest_signature", "digests whose cover signature did not verify"),
    // ACT1-hold-b (45.2, LD0's defect 3b): what the chase of a gap did.
    ("backfill_reached", "back-fill passes whose chain reached the reported K − 1"),
    ("backfill_budget", "back-fill passes that ended on their budget or closed nothing, short of it"),
    ("parent_after_backfill", "reported heads admitted as a parent once a back-fill closed their gap"),
    ("lag", "the K lag at the last report: the reported K against the head held (a level, not a count)"),
    // Phase 1 — WHY A BACK-FILL PAGE STOPPED short of its range, each apart.
    ("backfill_unlinked", "back-filled receipts past the head held: the page did not open at it"),
    ("backfill_refused", "back-filled receipts refused on their chain"),
    ("backfill_equivocation", "back-filled receipts at a held place with another hash"),
    // Phase 1 item 1 — AGREEMENT, per peer, at its last digest (levels).
    ("strata", "strata of ruled words the peer's last digest named (a level)"),
    ("discordant", "of them, strata whose ruled sections differ from this court's (a level)"),
    // Phase 1 item 1, 78.62 step 4 — WHAT A PEER'S DIFFERING SENTENCE IS to
    // this court's own section on its chart, read exactly (`relation`, 78.60):
    // the one word, a refraction of it, its conjugate with the placement
    // apart, related by a conjugator the alphabet cannot spell, or a word no
    // one motion relates.
    ("relation_same", "fetched sentences that are this court's own section on their chart"),
    ("relation_refracted", "fetched sentences this court's section refracts onto by one Γ"),
    ("relation_conjugate", "fetched sentences one spelled word conjugates onto, the placement apart"),
    ("relation_unspellable", "fetched sentences one rotor conjugates onto and the alphabet cannot spell"),
    ("relation_distinct", "fetched sentences no one motion relates to this court's section"),
    // 78.61 C1 — THE SAME, PERCEIVED: both words lifted into perception
    // and compared lane by lane on TUniversal {S, G, T, H}: COHERENT where
    // the four deltas are congruent; otherwise each diverging lane counted
    // apart (never summed), and what could not be read apart from both.
    ("perceived_coherent", "fetched sentences coherent with this court's section: congruent on all four lanes"),
    ("perceived_unread", "fetched sentences whose lift could not be read whole on one side or both"),
    ("residue_s", "fetched sentences diverging on the spatial lane (the motion's class and modes)"),
    ("residue_g", "fetched sentences diverging on the gauge lane (the hue cycle types)"),
    ("residue_t", "fetched sentences diverging on the temporal lane (the returns)"),
    ("residue_h", "fetched sentences diverging on the harmonic lane (the rotor classes)"),
    // 78.63 — WHICH PATH OF THE GLUE EACH FETCHED WORD LEFT BY, one per
    // word: the refusals held to the test (does it name a law?).
    ("glue_heard", "fetched words already heard on their chart, held as dialect"),
    ("glue_replay_refused", "fetched words this court's replay refused"),
    ("glue_colour_unproven", "fetched colour words whose proof does not stand here yet"),
    ("glue_citation_missing", "fetched words citing a word this court does not hold"),
    ("glue_gate_refused", "fetched sections the gate refused (no exact rotation of finite order)"),
    ("glue_torsion_refused", "fetched words the torsion check refused"),
    ("glue_class_dialect", "fetched words held as dialect on a class simplex"),
    ("glue_attached", "fetched words attached to a section (ruled, not installed)"),
    ("glue_admitted", "fetched words admitted"),
    // H2 (2026-10-03) — A CITATION INTO A FOLDED RANGE: a reported receipt
    // naming as its parent a receipt at or below the root this court holds
    // for the cited signer's own fold, which `shed_folded` removed and no
    // back-fill can bring back below a head.
    ("folded_parent", "reported receipts citing a parent at or below its signer's held fold root"),
    // 78.65, 78.87 R2 — the citation into a closed era PROVED by the era's
    // own chain, walked from it up to the signer's fold root.
    ("folded_proved", "citations into a closed era proved by the era's chain up to its root"),
    // 2026-10-08 — NO SILENT VERDICT (the coordinator's ruling on hypha c's
    // isolation): a citation of another signer's receipt this court does not
    // hold is counted apart from a hole in the signer's own line, and a
    // back-fill page that stops on one says so, under the CITED signer.
    ("missing_parent", "frames citing another signer's receipt this court does not hold (the back-fill chases it)"),
    ("backfill_missing_parent", "back-fill pages stopped on a cited receipt of this signer that this court does not hold"),
    ("backfill_missing_unrooted", "of them, citations of a signer whose fold root this court does not hold (no era proof can run)"),
    ("missing_depth", "the cited orbit against the head held for this signer at the last such stop (a level, not a count)"),
    // 78.147 — CATCH-UP FROM A FOLD ROOT: the signer's root, answered for a
    // range opening below it, adopted on its two signatures and its line
    // continued hot from it; or refused, the proof not standing.
    ("root_adopted", "fold roots of this signer adopted on their two signatures, its line continued hot from them"),
    ("root_refused", "fold roots of this signer answered and refused: a signature or the root's hash did not verify"),
];

/// How many axes [`REPLICATION_AXES`] holds: the one statement of the
/// reading's width, read by the codec, the store and every renderer.
pub const REP_AXES: usize = 50;

/// The axes of [`REPLICATION_AXES`] by name, for the sites that count them.
pub mod rep {
    pub const PARENT: usize = 0;
    pub const BACKFILL: usize = 1;
    pub const GAP: usize = 2;
    pub const STALE: usize = 3;
    pub const EQUIVOCATION: usize = 4;
    pub const REFUSED_ORIGIN: usize = 5;
    pub const REFUSED_SIGNATURE: usize = 6;
    pub const REFUSED_RECEIPT: usize = 7;
    pub const NO_SECTION: usize = 8;
    pub const MALFORMED: usize = 9;
    pub const NO_ORIGIN: usize = 10;
    pub const DIGEST_UNREADABLE: usize = 11;
    pub const DIGEST_SIGNATURE: usize = 12;
    pub const BACKFILL_REACHED: usize = 13;
    pub const BACKFILL_BUDGET: usize = 14;
    pub const PARENT_AFTER_BACKFILL: usize = 15;
    /// A LEVEL: set at each report, never summed.
    pub const LAG: usize = 16;
    pub const BACKFILL_UNLINKED: usize = 17;
    pub const BACKFILL_REFUSED: usize = 18;
    pub const BACKFILL_EQUIVOCATION: usize = 19;
    /// LEVELS, set at each digest, never summed.
    pub const STRATA: usize = 20;
    pub const DISCORDANT: usize = 21;
    pub const RELATION_SAME: usize = 22;
    pub const RELATION_REFRACTED: usize = 23;
    pub const RELATION_CONJUGATE: usize = 24;
    pub const RELATION_UNSPELLABLE: usize = 25;
    pub const RELATION_DISTINCT: usize = 26;
    pub const PERCEIVED_COHERENT: usize = 27;
    pub const PERCEIVED_UNREAD: usize = 28;
    /// The first of the four lane residues, S G T H in order.
    pub const RESIDUE_S: usize = 29;
    pub const GLUE_HEARD: usize = 33;
    pub const GLUE_REPLAY_REFUSED: usize = 34;
    pub const GLUE_COLOUR_UNPROVEN: usize = 35;
    pub const GLUE_CITATION_MISSING: usize = 36;
    pub const GLUE_GATE_REFUSED: usize = 37;
    pub const GLUE_TORSION_REFUSED: usize = 38;
    pub const GLUE_CLASS_DIALECT: usize = 39;
    pub const GLUE_ATTACHED: usize = 40;
    pub const GLUE_ADMITTED: usize = 41;
    pub const FOLDED_PARENT: usize = 42;
    pub const FOLDED_PROVED: usize = 43;
    pub const MISSING_PARENT: usize = 44;
    pub const BACKFILL_MISSING_PARENT: usize = 45;
    pub const BACKFILL_MISSING_UNROOTED: usize = 46;
    /// A LEVEL: set at each stop, never summed.
    pub const MISSING_DEPTH: usize = 47;
    pub const ROOT_ADOPTED: usize = 48;
    pub const ROOT_REFUSED: usize = 49;
}

/// REPLICATION PER SIGNER (CV5): each signer this court heard from, by key,
/// with what it did with that signer's receipts on every axis of
/// [`REPLICATION_AXES`]; and the frames no signer could be read off,
/// apart, never attributed to one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replication {
    pub signers: Vec<([u8; 32], [u64; REP_AXES])>,
    pub unattributed: [u64; REP_AXES],
}

/// Nothing heard: no signer, every unattributed axis zero. Written out
/// because the axes outgrew the array width `Default` is derived for.
impl Default for Replication {
    fn default() -> Self {
        Replication { signers: Vec::new(), unattributed: [0; REP_AXES] }
    }
}

impl Replication {
    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — a constant table: REPLICATION_AXES, REP_AXES = 22 entries")]
        out.push(REPLICATION_AXES.len() as u8);
        crate::transducer::address::put_u32_count(out, self.signers.len())?;
        for (vk, axes) in &self.signers {
            out.extend_from_slice(vk);
            for v in axes {
                put_u64(out, *v);
            }
        }
        for v in &self.unattributed {
            put_u64(out, *v);
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Replication> {
        if usize::from(take_u8(b, p)?) != REPLICATION_AXES.len() {
            return None;
        }
        let m = take_u32(b, p)? as usize;
        if !holds(b, *p, m, 32 + 8 * REP_AXES) {
            return None;
        }
        let axes = |b: &[u8], p: &mut usize| -> Option<[u64; REP_AXES]> {
            let mut a = [0u64; REP_AXES];
            for v in a.iter_mut() {
                *v = take_u64(b, p)?;
            }
            Some(a)
        };
        let mut signers = Vec::with_capacity(m);
        for _ in 0..m {
            let vk: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            signers.push((vk, axes(b, p)?));
        }
        Some(Replication { signers, unattributed: axes(b, p)? })
    }
}

/// ADJUDICATION BY KIND (CV5): the sentences this court ratified, stood,
/// docketed to await a citation, and deferred, and those it refused by the
/// refusal's own kind (`RefusalKind::to_byte`), ascending.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Adjudication {
    pub ratified: u64,
    pub stood: u64,
    pub awaiting: u64,
    pub deferred: u64,
    pub refused: Vec<(u8, u64)>,
    /// CV1 — the same refusals by the kind of obstruction (78.9): P, D, C,
    /// I, N, unclassified.
    pub obstructed: [u64; 6],
}

impl Adjudication {
    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        for v in [self.ratified, self.stood, self.awaiting, self.deferred] {
            put_u64(out, v);
        }
        crate::transducer::address::put_u32_count(out, self.refused.len())?;
        for &(kind, n) in &self.refused {
            out.push(kind);
            put_u64(out, n);
        }
        for v in self.obstructed {
            put_u64(out, v);
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Adjudication> {
        let (ratified, stood, awaiting, deferred) = (take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?);
        let m = take_u32(b, p)? as usize;
        if !holds(b, *p, m, 9) {
            return None;
        }
        let mut refused = Vec::with_capacity(m);
        for _ in 0..m {
            refused.push((take_u8(b, p)?, take_u64(b, p)?));
        }
        let mut obstructed = [0u64; 6];
        for v in obstructed.iter_mut() {
            *v = take_u64(b, p)?;
        }
        Some(Adjudication { ratified, stood, awaiting, deferred, refused, obstructed })
    }
}

/// ONE READING, tagged by what it answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reading {
    Memory(Memory),
    Court(Box<Court>),
    Replication(Box<Replication>),
    Adjudication(Adjudication),
    /// PH5 — a court's last phase window (78.17).
    Phase(super::Phase),
    /// LD1 — the bytes per award, per receipt delta and per frame (78.22).
    Carriage(super::Carriage),
    /// ACT1 — a node on the TUniversal clock, with its three counts
    /// (78.21, 78.27(1), EC1).
    Clock(super::Clock),
    /// EC1, 78.33 — a court's payments by proof of work, per lane.
    Economy(Box<super::Economy>),
    /// A court's acts and metabolism.
    Acts(Box<super::Acts>),
    /// 78.64 H1 — what each of a court's carriers holds, at its `K`.
    Carriers(Box<super::Carriers>),
    /// 78.145 — the seat relay: its standing, and per domain the posts it
    /// filed by kind and refused by reason since boot.
    Relay(Box<super::Relay>),
    /// MU-CLIENT §5.3 — a mind's conversation and what it built.
    Mind(Box<super::Mind>),
    /// MU-CLIENT §5.5 — the agreed language a court attested, per pair.
    Agreements(Box<super::Agreements>),
    /// THE AGE OF THE STANDING, IN QUESTIONS: how many questions this court
    /// was asked since its stroke last published its standing, this one
    /// included. One: the standing was published after the last question was
    /// asked. More: strokes missed that many less one, and every reading the
    /// stroke publishes is older than they are.
    Asked(u64),
}

impl Reading {
    pub fn tag(&self) -> u8 {
        match self {
            Reading::Memory(_) => TAG_MEMORY,
            Reading::Court(_) => TAG_COURT,
            Reading::Replication(_) => TAG_REPLICATION,
            Reading::Adjudication(_) => TAG_ADJUDICATION,
            Reading::Phase(_) => TAG_PHASE,
            Reading::Carriage(_) => TAG_CARRIAGE,
            Reading::Clock(_) => TAG_CLOCK,
            Reading::Economy(_) => TAG_ECONOMY,
            Reading::Acts(_) => TAG_ACTS,
            Reading::Carriers(_) => TAG_CARRIERS,
            Reading::Relay(_) => TAG_RELAY,
            Reading::Mind(_) => TAG_MIND,
            Reading::Agreements(_) => TAG_AGREEMENTS,
            Reading::Asked(_) => TAG_ASKED,
        }
    }

    /// Fold a later reading of the same kind into this one; a court's
    /// reading is what it holds now, so the later one stands. `false` where
    /// the two are of different kinds, which is no fold.
    pub fn absorb(&mut self, o: &Reading) -> bool {
        match (self, o) {
            (Reading::Memory(a), Reading::Memory(b)) => a.absorb(b),
            // A court's standing, its replication and its adjudications are
            // read whole at the question; the later reading stands.
            (Reading::Court(a), Reading::Court(b)) => *a = b.clone(),
            (Reading::Replication(a), Reading::Replication(b)) => *a = b.clone(),
            (Reading::Adjudication(a), Reading::Adjudication(b)) => *a = b.clone(),
            // A court's phase is its last window; the later stands.
            (Reading::Phase(a), Reading::Phase(b)) => *a = b.clone(),
            // The carriage is measured whole at the question; the later
            // stands.
            (Reading::Carriage(a), Reading::Carriage(b)) => *a = b.clone(),
            // A node's clock is where it stands now; the later stands.
            (Reading::Clock(a), Reading::Clock(b)) => *a = b.clone(),
            // A court's economy is read off its journal whole; the later
            // stands.
            (Reading::Economy(a), Reading::Economy(b)) => *a = b.clone(),
            // A court's acts are read whole at its `K`; the later stands.
            (Reading::Acts(a), Reading::Acts(b)) => *a = b.clone(),
            // A census is read whole at its `K`; the later stands.
            (Reading::Carriers(a), Reading::Carriers(b)) => *a = b.clone(),
            // The relay is read whole at the question; the later stands.
            (Reading::Relay(a), Reading::Relay(b)) => *a = b.clone(),
            // A mind's reading and a court's agreements are read whole at
            // the question; the later stands.
            (Reading::Mind(a), Reading::Mind(b)) => *a = b.clone(),
            (Reading::Agreements(a), Reading::Agreements(b)) => *a = b.clone(),
            (Reading::Asked(a), Reading::Asked(b)) => *a = *b,
            _ => return false,
        }
        true
    }

    /// The reading written out, its tag first; `None` where a reading passes
    /// a length its field writes (STD1 3c), refused whole.
    #[must_use]
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.push(self.tag());
        // The fixed-width readings are total; the rest refuse by the counts
        // and lengths they write.
        match self {
            Reading::Court(r) => r.put(out),
            Reading::Replication(r) => r.put(out),
            Reading::Adjudication(r) => r.put(out),
            Reading::Carriage(r) => r.put(out),
            Reading::Clock(r) => r.put(out),
            Reading::Memory(r) => {
                r.put(out);
                Some(())
            }
            Reading::Phase(r) => {
                r.put(out);
                Some(())
            }
            Reading::Economy(r) => r.put(out),
            Reading::Acts(r) => {
                r.put(out);
                Some(())
            }
            Reading::Carriers(r) => r.put(out),
            Reading::Relay(r) => r.put(out),
            Reading::Mind(r) => r.put(out),
            Reading::Agreements(r) => r.put(out),
            Reading::Asked(n) => {
                put_u64(out, *n);
                Some(())
            }
        }
    }

    /// The inverse of [`Reading::put`]; `None` for a tag this build does not
    /// read or a body that does not hold its reading.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Reading> {
        match take_u8(b, p)? {
            TAG_MEMORY => Some(Reading::Memory(Memory::take(b, p)?)),
            TAG_COURT => Some(Reading::Court(Box::new(Court::take(b, p)?))),
            TAG_REPLICATION => Some(Reading::Replication(Box::new(Replication::take(b, p)?))),
            TAG_ADJUDICATION => Some(Reading::Adjudication(Adjudication::take(b, p)?)),
            TAG_PHASE => Some(Reading::Phase(super::Phase::take(b, p)?)),
            TAG_CARRIAGE => Some(Reading::Carriage(super::Carriage::take(b, p)?)),
            TAG_CLOCK => Some(Reading::Clock(super::Clock::take(b, p)?)),
            TAG_ECONOMY => Some(Reading::Economy(Box::new(super::Economy::take(b, p)?))),
            TAG_ACTS => Some(Reading::Acts(Box::new(super::Acts::take(b, p)?))),
            TAG_CARRIERS => Some(Reading::Carriers(Box::new(super::Carriers::take(b, p)?))),
            TAG_RELAY => Some(Reading::Relay(Box::new(super::Relay::take(b, p)?))),
            TAG_MIND => Some(Reading::Mind(Box::new(super::Mind::take(b, p)?))),
            TAG_AGREEMENTS => Some(Reading::Agreements(Box::new(super::Agreements::take(b, p)?))),
            TAG_ASKED => Some(Reading::Asked(take_u64(b, p)?)),
            _ => None,
        }
    }
}
