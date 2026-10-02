// §8.1, audit of Phase N — THE READINGS AN OBSERVER CARRIES, each a whole
// typed object in its own coordinates and written out by its own codec.
//
//! Four readings, one per question the live fleet is asked:
//!
//! - [`Planes`] — WHICH DIMENSIONS ARE USED: the bivector census in the
//!   algebra it was read in, `n` first, each populated plane by its own
//!   blade. The declared planes are generated from `n` by whoever renders
//!   it, never carried, so a census cannot disagree with its algebra.
//! - [`Loss`] — WHAT WAS PASSED OVER: every refusal the engine counts where
//!   a reading did not reach content, on its own axis ([`LOSS_AXES`]).
//! - [`Memory`] — WHAT IS HELD: the allocator's bytes at the first and the
//!   latest sample, keyed by the source's own `K`, and the peak — so growth
//!   is read per unit of the source's own clock, never wall time (law 9).
//! - [`Court`] — WHAT A COURT HOLDS: its `K`, basis and treasury, and the
//!   acts it has served by kind.
//! - [`super::Learning`] — WHAT A MIND HAS LEARNED (LC1): its curve at its
//!   own `K`, honest at zero, the fields still owed named by their rows.
//! - [`super::Circuits`] — HOW A CIRCUIT IS BUILT (PL1): every kept circuit
//!   of the last percept, step by step.

/// A reading's tag, the first byte of its writing and the last byte of its
/// key.
pub const TAG_PLANES: u8 = 1;
pub const TAG_LOSS: u8 = 2;
pub const TAG_MEMORY: u8 = 3;
pub const TAG_COURT: u8 = 4;
pub const TAG_REPLICATION: u8 = 5;
pub const TAG_ADJUDICATION: u8 = 6;
pub const TAG_LEARNING: u8 = 7;
pub const TAG_CIRCUIT: u8 = 8;
pub const TAG_CELLS: u8 = 9;
pub const TAG_VISCOSITY: u8 = 10;
pub const TAG_SPEND: u8 = 11;
pub const TAG_PROGRESS: u8 = 12;
pub const TAG_PHASE: u8 = 13;
pub const TAG_CARRIAGE: u8 = 14;
pub const TAG_CLOCK: u8 = 15;
pub const TAG_ECONOMY: u8 = 16;
/// 2026-09-29 — a mind's faculties, and a court's acts and metabolism.
pub const TAG_SENSES: u8 = 17;
pub const TAG_ACTS: u8 = 18;
pub const TAG_CARRIERS: u8 = 19;

/// CQ1 — EVERY READING'S NAME, by its tag: the ONE statement the console's
/// `--reading NAME` is read against (law 12).
pub const READING_NAMES: [(&str, u8); 19] = [
    ("carriers", TAG_CARRIERS),
    ("clock", TAG_CLOCK),
    ("economy", TAG_ECONOMY),
    ("senses", TAG_SENSES),
    ("acts", TAG_ACTS),
    ("planes", TAG_PLANES),
    ("loss", TAG_LOSS),
    ("memory", TAG_MEMORY),
    ("court", TAG_COURT),
    ("replication", TAG_REPLICATION),
    ("adjudication", TAG_ADJUDICATION),
    ("learning", TAG_LEARNING),
    ("circuits", TAG_CIRCUIT),
    ("cells", TAG_CELLS),
    ("viscosity", TAG_VISCOSITY),
    ("spend", TAG_SPEND),
    ("progress", TAG_PROGRESS),
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

/// A PLANE CENSUS'S CLASSIFICATION ([`Planes::turning`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turning {
    Unread,
    Unturned,
    /// Every turn on the sensor's plane; `other` harmonic planes declared and
    /// never turned.
    Flattened { other: usize },
    Turns { turned: usize, of: usize },
}

/// THE PLANE CENSUS AS A WHOLE OBJECT (D1, 68, 71.2(3)): the algebra's `n`,
/// the transitions read and those coupling both halves, and each populated
/// plane with its count, ascending by blade. A plane that never turned is
/// absent here and present in `cga::bivector_planes(n)`, which is where a
/// reader learns what was declared.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Planes {
    pub n: u8,
    pub read: u64,
    pub coupled: u64,
    pub counts: Vec<(u64, u64)>,
    /// PC1 — each populated PARABOLIC plane, `(base axis, pole, count)`,
    /// ascending: a null bivector counted once, never as a turn beside a
    /// boost.
    pub parabolic: Vec<(u8, u8, u64)>,
}

impl Planes {
    /// The census a tally holds, read over the planes of its own algebra.
    pub fn of(t: &crate::transducer::cga::PlaneTally) -> Planes {
        use crate::transducer::cga::Pole;
        let n = t.algebra();
        let mut counts: Vec<(u64, u64)> = crate::transducer::cga::bivector_planes(n)
            .into_iter()
            .map(|p| (p as u64, t.count(p)))
            .filter(|&(_, c)| c > 0)
            .collect();
        counts.sort_unstable();
        let parabolic: Vec<(u8, u8, u64)> = (1..=n)
            .flat_map(|i| [Pole::Infinity, Pole::Origin].map(|p| (i, p)))
            .map(|(i, p)| (i, p.as_u8(), t.count_parabolic(usize::from(i), p)))
            .filter(|&(_, _, c)| c > 0)
            .collect();
        Planes { n, read: t.read, coupled: t.coupled, counts, parabolic }
    }

    /// WHAT THE CENSUS TURNED ON, read off its counts and never stored: no
    /// transition read; nothing turned (every transition a boost, a slip or
    /// the identity); FLATTENED — every turn on the sensor's plane `e1e2`;
    /// or turns on `turned` of the `of` harmonic planes the algebra declares.
    pub fn turning(&self) -> Turning {
        let harmonic = crate::transducer::cga::harmonic_planes(self.n);
        let turned: Vec<usize> = harmonic.iter().copied().filter(|&b| self.count(b) > 0).collect();
        let grid = crate::transducer::cga::E1 | crate::transducer::cga::E2;
        match (self.read, turned.as_slice()) {
            (0, _) => Turning::Unread,
            (_, []) => Turning::Unturned,
            (_, [only]) if *only == grid => Turning::Flattened { other: harmonic.len() - 1 },
            _ => Turning::Turns { turned: turned.len(), of: harmonic.len() },
        }
    }

    /// How often one plane was populated.
    pub fn count(&self, blade: usize) -> u64 {
        match self.counts.binary_search_by_key(&(blade as u64), |&(b, _)| b) {
            Ok(at) => self.counts[at].1,
            Err(_) => 0,
        }
    }

    /// How often one parabolic plane was populated.
    pub fn count_parabolic(&self, axis: u8, pole: crate::transducer::cga::Pole) -> u64 {
        match self.parabolic.binary_search_by_key(&(axis, pole.as_u8()), |&(a, p, _)| (a, p)) {
            Ok(at) => self.parabolic[at].2,
            Err(_) => 0,
        }
    }

    /// Fold a later census into this one: the larger algebra, and each
    /// plane's count added to its own — never to another plane's (law 5).
    pub fn absorb(&mut self, o: &Planes) {
        self.n = self.n.max(o.n);
        self.read += o.read;
        self.coupled += o.coupled;
        for &(blade, c) in &o.counts {
            match self.counts.binary_search_by_key(&blade, |&(b, _)| b) {
                Ok(at) => self.counts[at].1 += c,
                Err(at) => self.counts.insert(at, (blade, c)),
            }
        }
        for &(axis, pole, c) in &o.parabolic {
            match self.parabolic.binary_search_by_key(&(axis, pole), |&(a, p, _)| (a, p)) {
                Ok(at) => self.parabolic[at].2 += c,
                Err(at) => self.parabolic.insert(at, (axis, pole, c)),
            }
        }
    }

    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.push(self.n);
        put_u64(out, self.read);
        put_u64(out, self.coupled);
        crate::transducer::address::put_u32_count(out, self.counts.len())?;
        for &(blade, c) in &self.counts {
            put_u64(out, blade);
            put_u64(out, c);
        }
        crate::transducer::address::put_u32_count(out, self.parabolic.len())?;
        for &(axis, pole, c) in &self.parabolic {
            out.push(axis);
            out.push(pole);
            put_u64(out, c);
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Planes> {
        let n = take_u8(b, p)?;
        let (read, coupled) = (take_u64(b, p)?, take_u64(b, p)?);
        let m = take_u32(b, p)? as usize;
        // Each entry is sixteen bytes; a count the body cannot hold is
        // refused before anything is reserved for it.
        if !holds(b, *p, m, 16) {
            return None;
        }
        let mut counts = Vec::with_capacity(m);
        for _ in 0..m {
            counts.push((take_u64(b, p)?, take_u64(b, p)?));
        }
        let m = take_u32(b, p)? as usize;
        if !holds(b, *p, m, 10) {
            return None;
        }
        let mut parabolic = Vec::with_capacity(m);
        for _ in 0..m {
            parabolic.push((take_u8(b, p)?, take_u8(b, p)?, take_u64(b, p)?));
        }
        Some(Planes { n, read, coupled, counts, parabolic })
    }
}

/// THE AXES OF [`Loss`], in its writing's order: each counter's name and
/// what it means when it is not zero. The ONE statement of the order, read
/// by the codec and by every renderer (law 12).
pub const LOSS_AXES: [(&str, &str); 26] = [
    ("unheard", "components whose moduli are not read at their n (body_of refused)"),
    ("sectionless", "compartments with no grid plane, left out of a chain (on_grid refused)"),
    ("coarse", "sections filed at their n alone, their topology refused"),
    ("unmeasured", "covers whose share ended before their pieces did"),
    ("off_sheet", "frames that did not stand on the sheet (Axiom B fell back)"),
    ("tau_unfooted", "chord ends carrying no footprint"),
    ("tau_refused", "footprint displacements the Cayley chart could not carry"),
    ("unframed", "meets no proper turn relates"),
    ("open_180", "frames whose 180° turn nothing closes (a projective class)"),
    ("unspellable", "orbits whose holonomy the seventeen do not spell"),
    ("orbit_unread", "orbits evaluated and not measured: a rotation no versor"),
    ("refused", "pieces carried whose rotor the seventeen do not spell"),
    ("unclassed", "meets whose piece carries no transport class"),
    ("undecodable", "keys this reader wrote and could not read back"),
    ("offered", "pairs no versor carried, reaching the mint"),
    ("permanent", "mint gate 1, P: mirrored, incongruent, isotropic or irrational, proved permanent"),
    ("bodiless", "mint gate 1, I: a bodiless pair, nothing to pose"),
    ("held", "mint gate 1, no refusal: an unoriented pair, a held stabilizer"),
    ("turned", "mint gate 1, no refusal: the pair reads a turn of the base"),
    ("unclassified", "mint gate 1: a kind no pose refusal carries, a defect"),
    ("arcless", "mint gate 2: orients only at the quartic, no arc"),
    ("primeless", "mint gate 3: the arc names no prime the alphabet lacks"),
    ("minted", "RotorPrime demands minted"),
    ("unkeyed", "pole demands of a kind with no coordinates to stand by"),
    ("untyped", "pole demands their own type refuses"),
    ("merged", "pole demands already standing, stood once"),
];

/// WHAT PERCEPTION, THE COVER AND THE MINT PASSED OVER, each on its own
/// axis of [`LOSS_AXES`]. Accumulated axis by axis, never summed across
/// axes (law 5).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Loss {
    pub axes: [u64; LOSS_AXES.len()],
}

impl Loss {
    /// The losses one stroke read, taken from the counters that hold them.
    pub fn of(
        perception: [u64; 3],
        cover: &crate::transducer::family::cover::CoverTally,
        mint: &crate::transducer::family::perceive::DemandTally,
    ) -> Loss {
        let [unheard, sectionless, coarse] = perception;
        Loss {
            axes: [
                unheard,
                sectionless,
                coarse,
                cover.unmeasured,
                cover.off_sheet,
                cover.tau_unfooted,
                cover.tau_refused,
                cover.unframed,
                cover.frame_open_180,
                cover.unspellable,
                cover.orbit_unread,
                cover.refused,
                cover.unclassed,
                cover.undecodable,
                mint.offered,
                mint.permanent,
                mint.bodiless,
                mint.held,
                mint.turned,
                mint.unclassified,
                mint.arcless,
                mint.primeless,
                mint.minted,
                mint.unkeyed,
                mint.untyped,
                mint.merged,
            ],
        }
    }

    pub fn absorb(&mut self, o: &Loss) {
        for (a, b) in self.axes.iter_mut().zip(o.axes.iter()) {
            *a += b;
        }
    }

    fn put(&self, out: &mut Vec<u8>) {
        #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — a constant table: the axes array is sized LOSS_AXES.len(), 26 entries")]
        out.push(self.axes.len() as u8);
        for v in &self.axes {
            put_u64(out, *v);
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Loss> {
        // The count of axes is written, so a reader of another build's
        // layout refuses rather than reading one axis as another.
        if usize::from(take_u8(b, p)?) != LOSS_AXES.len() {
            return None;
        }
        let mut axes = [0u64; LOSS_AXES.len()];
        for a in axes.iter_mut() {
            *a = take_u64(b, p)?;
        }
        Some(Loss { axes })
    }
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
    /// work in the prefix), refused unamortized (less history than framing).
    pub folds: [u64; FOLD_OUTCOMES.len()],
    /// PHASE 1 (78.59) — THE CATCH-UP FETCHES BY PATH AND OUTCOME: a line's
    /// back-fill and a fold witness's prefix, each asked of the court that
    /// owns the line, on [`CATCHUP_PATHS`] × [`CATCHUP_OUTCOMES`]. Both
    /// read one `None` for four different failures; the four are counted
    /// apart (a `None` never stands for two facts, 78.26(4)).
    pub catchup: [[u64; CATCHUP_OUTCOMES.len()]; CATCHUP_PATHS.len()],
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
    ["proposed", "open", "inexact", "folded", "refused", "unmeasured", "idle", "unamortized"];

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
    Unamortized = 7,
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
        Some(())
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
];

/// How many axes [`REPLICATION_AXES`] holds: the one statement of the
/// reading's width, read by the codec, the store and every renderer.
pub const REP_AXES: usize = 42;

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
    Planes(Planes),
    Loss(Loss),
    Memory(Memory),
    Court(Court),
    Replication(Replication),
    Adjudication(Adjudication),
    /// LC1 — the learning curve of one mind (78.1).
    Learning(super::Learning),
    /// PL1 — the circuits of the last percept, step by step (78.12(4)).
    Circuit(super::Circuits),
    /// CO1 — the cells of the last percept as their orbit algebras (78.15).
    Cells(super::Cells),
    /// VO1 — VS1's arms per pair of planes per lane (78.16).
    Viscosity(super::Viscosity),
    /// SP1 — the stroke by reader (78.16).
    Spend(super::Spend),
    /// PG1 — E1's done-when (78.16).
    Progress(super::Progress),
    /// PH5 — a court's last phase window (78.17).
    Phase(super::Phase),
    /// LD1 — the bytes per award, per receipt delta and per frame (78.22).
    Carriage(super::Carriage),
    /// ACT1 — a node on the TUniversal clock, with its three counts
    /// (78.21, 78.27(1), EC1).
    Clock(super::Clock),
    /// EC1, 78.33 — a court's payments by proof of work, per lane.
    Economy(Box<super::Economy>),
    /// A mind's faculties: sight, cognition, speech, hearing, language,
    /// understanding, solving, and its growing carriers.
    Senses(Box<super::Senses>),
    /// A court's acts and metabolism.
    Acts(Box<super::Acts>),
    /// 78.64 H1 — what each of a court's carriers holds, at its `K`.
    Carriers(Box<super::Carriers>),
}

impl Reading {
    pub fn tag(&self) -> u8 {
        match self {
            Reading::Planes(_) => TAG_PLANES,
            Reading::Loss(_) => TAG_LOSS,
            Reading::Memory(_) => TAG_MEMORY,
            Reading::Court(_) => TAG_COURT,
            Reading::Replication(_) => TAG_REPLICATION,
            Reading::Adjudication(_) => TAG_ADJUDICATION,
            Reading::Learning(_) => TAG_LEARNING,
            Reading::Circuit(_) => TAG_CIRCUIT,
            Reading::Cells(_) => TAG_CELLS,
            Reading::Viscosity(_) => TAG_VISCOSITY,
            Reading::Spend(_) => TAG_SPEND,
            Reading::Progress(_) => TAG_PROGRESS,
            Reading::Phase(_) => TAG_PHASE,
            Reading::Carriage(_) => TAG_CARRIAGE,
            Reading::Clock(_) => TAG_CLOCK,
            Reading::Economy(_) => TAG_ECONOMY,
            Reading::Senses(_) => TAG_SENSES,
            Reading::Acts(_) => TAG_ACTS,
            Reading::Carriers(_) => TAG_CARRIERS,
        }
    }

    /// Fold a later reading of the same kind into this one; a court's
    /// reading is what it holds now, so the later one stands. `false` where
    /// the two are of different kinds, which is no fold.
    pub fn absorb(&mut self, o: &Reading) -> bool {
        match (self, o) {
            (Reading::Planes(a), Reading::Planes(b)) => a.absorb(b),
            (Reading::Loss(a), Reading::Loss(b)) => a.absorb(b),
            (Reading::Memory(a), Reading::Memory(b)) => a.absorb(b),
            // A court's standing, its replication and its adjudications are
            // read whole at the question; the later reading stands.
            (Reading::Court(a), Reading::Court(b)) => *a = b.clone(),
            (Reading::Replication(a), Reading::Replication(b)) => *a = b.clone(),
            (Reading::Adjudication(a), Reading::Adjudication(b)) => *a = b.clone(),
            // A mind's curve is what it holds at its latest `K`.
            (Reading::Learning(a), Reading::Learning(b)) => *a = b.clone(),
            // The circuits are the last percept's; the later percept stands.
            (Reading::Circuit(a), Reading::Circuit(b)) => *a = b.clone(),
            (Reading::Cells(a), Reading::Cells(b)) => *a = b.clone(),
            // A voice's viscosity, spend and progress accumulate over its
            // percepts, each count on its own axis.
            (Reading::Viscosity(a), Reading::Viscosity(b)) => a.absorb(b),
            (Reading::Spend(a), Reading::Spend(b)) => a.absorb(b),
            (Reading::Progress(a), Reading::Progress(b)) => a.absorb(b),
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
            // A mind's faculties and a court's acts are read whole at their
            // `K`; the later stands.
            (Reading::Senses(a), Reading::Senses(b)) => *a = b.clone(),
            (Reading::Acts(a), Reading::Acts(b)) => *a = b.clone(),
            // A census is read whole at its `K`; the later stands.
            (Reading::Carriers(a), Reading::Carriers(b)) => *a = b.clone(),
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
            Reading::Planes(r) => r.put(out),
            Reading::Court(r) => r.put(out),
            Reading::Replication(r) => r.put(out),
            Reading::Adjudication(r) => r.put(out),
            Reading::Learning(r) => r.put(out),
            Reading::Circuit(r) => r.put(out),
            Reading::Cells(r) => r.put(out),
            Reading::Viscosity(r) => r.put(out),
            Reading::Carriage(r) => r.put(out),
            Reading::Clock(r) => r.put(out),
            Reading::Loss(r) => {
                r.put(out);
                Some(())
            }
            Reading::Memory(r) => {
                r.put(out);
                Some(())
            }
            Reading::Spend(r) => {
                r.put(out);
                Some(())
            }
            Reading::Progress(r) => {
                r.put(out);
                Some(())
            }
            Reading::Phase(r) => {
                r.put(out);
                Some(())
            }
            Reading::Economy(r) => r.put(out),
            Reading::Senses(r) => {
                r.put(out);
                Some(())
            }
            Reading::Acts(r) => {
                r.put(out);
                Some(())
            }
            Reading::Carriers(r) => r.put(out),
        }
    }

    /// The inverse of [`Reading::put`]; `None` for a tag this build does not
    /// read or a body that does not hold its reading.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Reading> {
        match take_u8(b, p)? {
            TAG_PLANES => Some(Reading::Planes(Planes::take(b, p)?)),
            TAG_LOSS => Some(Reading::Loss(Loss::take(b, p)?)),
            TAG_MEMORY => Some(Reading::Memory(Memory::take(b, p)?)),
            TAG_COURT => Some(Reading::Court(Court::take(b, p)?)),
            TAG_REPLICATION => Some(Reading::Replication(Replication::take(b, p)?)),
            TAG_ADJUDICATION => Some(Reading::Adjudication(Adjudication::take(b, p)?)),
            TAG_LEARNING => Some(Reading::Learning(super::Learning::take(b, p)?)),
            TAG_CIRCUIT => Some(Reading::Circuit(super::Circuits::take(b, p)?)),
            TAG_CELLS => Some(Reading::Cells(super::Cells::take(b, p)?)),
            TAG_VISCOSITY => Some(Reading::Viscosity(super::Viscosity::take(b, p)?)),
            TAG_SPEND => Some(Reading::Spend(super::Spend::take(b, p)?)),
            TAG_PROGRESS => Some(Reading::Progress(super::Progress::take(b, p)?)),
            TAG_PHASE => Some(Reading::Phase(super::Phase::take(b, p)?)),
            TAG_CARRIAGE => Some(Reading::Carriage(super::Carriage::take(b, p)?)),
            TAG_CLOCK => Some(Reading::Clock(super::Clock::take(b, p)?)),
            TAG_ECONOMY => Some(Reading::Economy(Box::new(super::Economy::take(b, p)?))),
            TAG_SENSES => Some(Reading::Senses(Box::new(super::Senses::take(b, p)?))),
            TAG_ACTS => Some(Reading::Acts(Box::new(super::Acts::take(b, p)?))),
            TAG_CARRIERS => Some(Reading::Carriers(Box::new(super::Carriers::take(b, p)?))),
            _ => None,
        }
    }
}
