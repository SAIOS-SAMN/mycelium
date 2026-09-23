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

/// A reading's tag, the first byte of its writing and the last byte of its
/// key.
pub const TAG_PLANES: u8 = 1;
pub const TAG_LOSS: u8 = 2;
pub const TAG_MEMORY: u8 = 3;
pub const TAG_COURT: u8 = 4;

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
}

impl Planes {
    /// The census a tally holds, read over the planes of its own algebra.
    pub fn of(t: &crate::transducer::cga::PlaneTally) -> Planes {
        let n = t.algebra();
        let mut counts: Vec<(u64, u64)> = crate::transducer::cga::bivector_planes(n)
            .into_iter()
            .map(|p| (p as u64, t.count(p)))
            .filter(|&(_, c)| c > 0)
            .collect();
        counts.sort_unstable();
        Planes { n, read: t.read, coupled: t.coupled, counts }
    }

    /// How often one plane was populated.
    pub fn count(&self, blade: usize) -> u64 {
        match self.counts.binary_search_by_key(&(blade as u64), |&(b, _)| b) {
            Ok(at) => self.counts[at].1,
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
    }

    fn put(&self, out: &mut Vec<u8>) {
        out.push(self.n);
        put_u64(out, self.read);
        put_u64(out, self.coupled);
        out.extend_from_slice(&(self.counts.len() as u32).to_le_bytes());
        for &(blade, c) in &self.counts {
            put_u64(out, blade);
            put_u64(out, c);
        }
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
        Some(Planes { n, read, coupled, counts })
    }
}

/// THE AXES OF [`Loss`], in its writing's order: each counter's name and
/// what it means when it is not zero. The ONE statement of the order, read
/// by the codec and by every renderer (law 12).
pub const LOSS_AXES: [(&str, &str); 18] = [
    ("unheard", "components whose moduli are not read at their n (body_of refused)"),
    ("sectionless", "laminae with no grid plane, left out of a chain (on_grid refused)"),
    ("coarse", "sections filed at their n alone, their topology refused"),
    ("unmeasured", "covers whose share ended before their pieces did"),
    ("off_sheet", "frames that did not stand on the sheet (Axiom B fell back)"),
    ("tau_unfooted", "chord ends carrying no footprint"),
    ("tau_refused", "footprint displacements the Cayley chart could not carry"),
    ("unframed", "meets no proper turn relates"),
    ("half_open", "frames whose half turn nothing closes (a projective class)"),
    ("unspellable", "loops whose holonomy the seventeen do not spell"),
    ("refused", "pieces carried whose rotor the seventeen do not spell"),
    ("unclassed", "meets whose piece carries no transport class"),
    ("undecodable", "keys this reader wrote and could not read back"),
    ("offered", "pairs no versor carried, reaching the mint"),
    ("permanent", "mint gate 1: mirrored or incongruent, proved permanent"),
    ("arcless", "mint gate 2: orients only at the quartic, no arc"),
    ("primeless", "mint gate 3: the arc names no prime the alphabet lacks"),
    ("minted", "RotorPrime demands minted"),
];

/// WHAT PERCEPTION, THE COVER AND THE MINT PASSED OVER, each on its own
/// axis of [`LOSS_AXES`]. Accumulated axis by axis, never summed across
/// axes (law 5).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Loss {
    pub axes: [u64; 18],
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
                cover.frame_half_open,
                cover.unspellable,
                cover.refused,
                cover.unclassed,
                cover.undecodable,
                mint.offered,
                mint.permanent,
                mint.arcless,
                mint.primeless,
                mint.minted,
            ],
        }
    }

    pub fn absorb(&mut self, o: &Loss) {
        for (a, b) in self.axes.iter_mut().zip(o.axes.iter()) {
            *a += b;
        }
    }

    fn put(&self, out: &mut Vec<u8>) {
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
        let mut axes = [0u64; 18];
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
}

impl Court {
    fn put(&self, out: &mut Vec<u8>) {
        put_u64(out, self.k);
        put_u64(out, self.basis);
        put_u64(out, self.treasury);
        out.extend_from_slice(&(self.served.len() as u32).to_le_bytes());
        for &(kind, n) in &self.served {
            out.push(kind);
            put_u64(out, n);
        }
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
        Some(Court { k, basis, treasury, served })
    }
}

/// ONE READING, tagged by what it answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reading {
    Planes(Planes),
    Loss(Loss),
    Memory(Memory),
    Court(Court),
}

impl Reading {
    pub fn tag(&self) -> u8 {
        match self {
            Reading::Planes(_) => TAG_PLANES,
            Reading::Loss(_) => TAG_LOSS,
            Reading::Memory(_) => TAG_MEMORY,
            Reading::Court(_) => TAG_COURT,
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
            (Reading::Court(a), Reading::Court(b)) => *a = b.clone(),
            _ => return false,
        }
        true
    }

    /// The reading written out, its tag first.
    pub fn put(&self, out: &mut Vec<u8>) {
        out.push(self.tag());
        match self {
            Reading::Planes(r) => r.put(out),
            Reading::Loss(r) => r.put(out),
            Reading::Memory(r) => r.put(out),
            Reading::Court(r) => r.put(out),
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
            _ => None,
        }
    }
}
