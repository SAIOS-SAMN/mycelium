// 45, 78.24 — THE RECEIPT: one act at its stamp, the parents it cites by
// their stamps, the frame it measured, and the words it names by their
// coordinates.

use crate::transducer::address::Address;
use crate::transducer::stalk::Invariant;

/// WHERE AN ACT STANDS ON ITS SIGNER'S LINE (78.23 as amended): the orbit
/// `K` and the act's place `i` within it, the finest coordinate. Ordered
/// orbit first, so the line reads in the order its acts were sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Place {
    pub k: u64,
    pub i: u64,
}

impl Place {
    /// A place's coordinates: the orbit, the act's place in it.
    pub const LEN: usize = 8 + 8;

    /// A line's first place: the first act of its first orbit.
    pub const GENESIS: Place = Place { k: 0, i: 0 };

    /// The place written big-endian, `K‖i`, so byte order is line order.
    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.k.to_be_bytes());
        out.extend_from_slice(&self.i.to_be_bytes());
    }

    pub fn take_canonical(b: &[u8], p: &mut usize) -> Option<Place> {
        let k = u64::from_be_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let i = u64::from_be_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        *p += Self::LEN;
        Some(Place { k, i })
    }

    /// The next act in this place's orbit.
    pub fn within(self) -> Place {
        Place { k: self.k, i: self.i + 1 }
    }

    /// The first act of the orbit after this place's.
    pub fn beyond(self) -> Place {
        Place { k: self.k + 1, i: 0 }
    }

    /// Does `next` follow this place on one line: the next act of its
    /// orbit, or the first of the next orbit. Which of the two the line
    /// took is the predecessor's to say; the place alone cannot.
    pub fn is_followed_by(self, next: Place) -> bool {
        next == self.within() || next == self.beyond()
    }

    /// THE PLACE AFTER AN ACT OF THIS KIND AT THIS PLACE: the first of the
    /// next orbit after a close, the next act of the same orbit otherwise
    /// (78.23 as amended). The predecessor's kind says which — the one
    /// statement the writer, the integrity check and admission read.
    pub fn after(self, kind: ActKind) -> Place {
        match kind {
            ActKind::Orbit => self.beyond(),
            _ => self.within(),
        }
    }

    /// A place as it prints, `K.i`.
    pub fn print(self) -> String {
        format!("{}.{}", self.k, self.i)
    }
}

/// THE STAMP (78.21, 78.24, 78.23 as amended): where one act stands in the
/// ledger — the tick `h` (TUniversal, 45.2), the signer's key, the
/// signer's orbit `K` and the act's place `i` in it.
///
/// It is the ledger's KEY. Written coarsest first — `h`, then `vk`, then
/// `K`, then `i` — so a prefix is a stratum: `h` alone is everything the
/// mesh did at one tick, `h‖vk` one signer at that tick. The integers are
/// written BIG-endian, the one place the tree does so, because in a key the
/// byte order IS the order the trie is read in, and big-endian is the
/// order in which a smaller number reads first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stamp {
    pub h: u64,
    pub vk: [u8; 32],
    pub k: u64,
    pub i: u64,
}

impl Stamp {
    /// A stamp's coordinates: the tick, the key, the orbit, the place.
    pub const LEN: usize = 8 + 32 + Place::LEN;

    /// The stamp of one act: its tick, its signer, its place on the line.
    pub fn new(h: u64, vk: [u8; 32], at: Place) -> Stamp {
        Stamp { h, vk, k: at.k, i: at.i }
    }

    /// Where the act stands on its signer's line.
    pub fn place(&self) -> Place {
        Place { k: self.k, i: self.i }
    }

    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.h.to_be_bytes());
        out.extend_from_slice(&self.vk);
        self.place().put_canonical(out);
    }

    /// The stamp as the key it is filed under.
    pub fn key(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Self::LEN);
        self.put_canonical(&mut out);
        out
    }

    pub fn take_canonical(b: &[u8], p: &mut usize) -> Option<Stamp> {
        let h = u64::from_be_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let vk: [u8; 32] = b.get(*p + 8..*p + 40)?.try_into().ok()?;
        let mut q = *p + 40;
        let at = Place::take_canonical(b, &mut q)?;
        *p = q;
        Some(Stamp::new(h, vk, at))
    }

    /// The stamp a key names, exactly or not at all.
    pub fn from_key(b: &[u8]) -> Option<Stamp> {
        let mut p = 0usize;
        let s = Self::take_canonical(b, &mut p)?;
        (p == b.len()).then_some(s)
    }

    /// The signer's own line, keyed by `vk‖K‖i`: the relation the range
    /// readers ask by (78.24 P1). A coordinate, never the act.
    pub fn line_key(vk: &[u8; 32], at: Place) -> Vec<u8> {
        let mut out = Vec::with_capacity(32 + Place::LEN);
        out.extend_from_slice(vk);
        at.put_canonical(&mut out);
        out
    }
}

/// A RECEIPT THIS ACT CITES: its stamp and its chain hash. The stamp says
/// where it stands; the hash is what the citing receipt commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent {
    pub at: Stamp,
    pub hash: [u8; 32],
}

impl Parent {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        self.at.put_canonical(out);
        out.extend_from_slice(&self.hash);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Parent> {
        let at = Stamp::take_canonical(b, p)?;
        let hash: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(Parent { at, hash })
    }
}

/// Height of a receipt in the ledger (45.2). Genesis is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Height(pub u64);

impl Height {
    pub fn get(self) -> u64 {
        self.0
    }

    /// A height reads as a height, `h {n}` as 34.2 writes it — never a
    /// wall clock (law 9).
    pub fn stamp(self) -> String {
        Self::stamp_of(self.0)
    }

    pub fn stamp_of(h: u64) -> String {
        format!("h {h}")
    }
}

/// TUniversal is the height of the mesh receipt ledger.
pub type TUniversal = Height;

/// WHAT A RULING RULED (EC1, 78.33): the arm a court's ruling takes on a
/// word, carried on the Ruling node as its kind byte so a court admitting
/// the receipt applies the same arm — nothing else travels. Upheld,
/// demoted and slashed rule a provisional word; retired rules a word of the
/// ruled language away, re-expressed over fewer letters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ruled {
    Upheld,
    Demoted,
    Slashed,
    Retired,
}

impl Ruled {
    /// Every arm, in the order the rulings reading carries them.
    pub const ALL: [Ruled; 4] = [Ruled::Upheld, Ruled::Demoted, Ruled::Slashed, Ruled::Retired];

    /// Where the arm is carried in the rulings reading.
    pub fn index(self) -> usize {
        match self {
            Ruled::Upheld => 0,
            Ruled::Demoted => 1,
            Ruled::Slashed => 2,
            Ruled::Retired => 3,
        }
    }

    /// The arm the wire's verdict vocabulary names, or none for a byte
    /// that names no arm. Retirement has no verdict byte: it is ruled by a
    /// bid, never by a verdict.
    pub fn of_verdict(ruling: u8) -> Option<Ruled> {
        use crate::anastome::dialogue::{RULING_DEMOTED, RULING_SLASHED, RULING_UPHELD};
        match ruling {
            RULING_UPHELD => Some(Ruled::Upheld),
            RULING_DEMOTED => Some(Ruled::Demoted),
            RULING_SLASHED => Some(Ruled::Slashed),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Ruled::Upheld => "upheld",
            Ruled::Demoted => "demoted",
            Ruled::Slashed => "slashed",
            Ruled::Retired => "retired",
        }
    }
}

/// WHY A PEER'S RULING DID NOT APPLY HERE (EC1, F5): the verdict found no
/// party to release to; a word that speaks for the retired one is not ruled
/// here; the retired word does not decompose over this court's basis without
/// it. Each an I (78.9), counted by arm and by reason on the clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RulingRefused {
    NoParty,
    SpeakerNotRuled,
    DoesNotDecompose,
}

impl RulingRefused {
    /// Every reason, in the order the rulings reading carries them.
    pub const ALL: [RulingRefused; 3] =
        [RulingRefused::NoParty, RulingRefused::SpeakerNotRuled, RulingRefused::DoesNotDecompose];

    /// Where the reason is carried in the rulings reading.
    pub fn index(self) -> usize {
        match self {
            RulingRefused::NoParty => 0,
            RulingRefused::SpeakerNotRuled => 1,
            RulingRefused::DoesNotDecompose => 2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            RulingRefused::NoParty => "no party",
            RulingRefused::SpeakerNotRuled => "speaker not ruled",
            RulingRefused::DoesNotDecompose => "does not decompose",
        }
    }
}

/// THE KIND OF ACT A RECEIPT ADMITS (45.1), 78.94: the ledger's name for
/// the one table of actions (`physics::actions::Action`). Its coordinate
/// byte is the action's own, in the node's chain hash, so a phase (45.9) and
/// a fold (45.8) read the ledger by kind; a byte naming no action a court
/// seals refuses the read (`Action::from_byte`).
pub use crate::physics::actions::Action as ActKind;

/// A19 — A CLASS RECORD: the class key's coordinates and the order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassRecord {
    pub key: Address,
    pub order: u32,
}

impl ClassRecord {
    /// The class record of a word with a rotor: the rotor's class key and
    /// its order (`0` infinite). A form has none (78.24 (2a)): its element
    /// carries the identity and its classes are a set, so its fiber is its
    /// chart alone.
    pub fn of(w: &crate::transducer::element::Element) -> Option<ClassRecord> {
        if w.is_sentence() {
            return None;
        }
        let rotor = w.letters()?.spelling.word.rotor();
        let key = rotor.class_key()?;
        let order = match rotor.order()? {
            crate::transducer::alphabet::Order::Finite(l) => l,
            crate::transducer::alphabet::Order::Infinite => 0,
        };
        Some(ClassRecord { key, order })
    }
}

impl Cite {
    /// A WORD'S CITATION: its fiber, read off the word, and the stamp of
    /// the act that admitted it.
    pub fn word(w: &crate::transducer::element::Element, at: Stamp) -> Cite {
        Cite::word_charted(w, crate::transducer::stalk::invariant_of(w), at)
    }

    /// The same cite with the word's chart already read (`invariant_of(w)`).
    pub fn word_charted(w: &crate::transducer::element::Element, chart: crate::transducer::stalk::Invariant, at: Stamp) -> Cite {
        Cite::Word { chart, class: ClassRecord::of(w), at }
    }
}

/// WHAT AN ACT NAMES (LD0 as ruled, 78.22, 78.24): coordinates the reader
/// already holds the named thing under, fixed in size by the algebra and
/// never a copy of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cite {
    /// A word or a form by its FIBER — its chart, and for a word with a
    /// rotor its class record; a form's fiber is its chart alone — and the
    /// stamp of the act that admitted it, which locates it exactly.
    Word { chart: Invariant, class: Option<ClassRecord>, at: Stamp },
    /// A fold's root (45.8): the stamp of the archived prefix's head and
    /// its chain hash.
    Root { at: Stamp, hash: [u8; 32] },
    /// 78.37, TP1 — THE DOCKET'S HELD CLASS: a ring the orbit's transport
    /// closed at holonomy ≠ 1, by its class vector inline, and its COVER —
    /// the charts its thoughts stand on, by their keys, ascending — whose
    /// containment orders the docket. `None` where the cover did not read:
    /// the entry's enclosure is owed.
    /// Its ROTOR rides beside κ as its ray (78.27(7): the class is the
    /// coordinate, the rotor the residue) — κ does not determine it, and the
    /// taker writes it into the ring's thoughts' horizons.
    Class {
        class: Box<crate::transducer::sentence::ClassVector>,
        rotor: Box<crate::transducer::alphabet::Rotor>,
        cover: Option<Vec<Vec<u8>>>,
    },
    /// THE DOCKET'S DEMAND: an orbit CS2's demand arm left, by its
    /// undischarged class vector inline — its own plane section names the
    /// mode (3.9) — its cover, as a class's, and (78.47) its RESIDUE's
    /// canonical ray, fixed by the algebra's `n`: the coordinate its
    /// constructors are derived from, never the constructors. `None` where
    /// the residue did not read.
    Demand {
        class: Box<crate::transducer::sentence::ClassVector>,
        residue: Option<crate::transducer::cga::Mv>,
        cover: Option<Vec<Vec<u8>>>,
    },
    /// 78.93 B0 — A REFUSAL ON THE RECORD: the refused sentence by its fiber
    /// (its chart) and the kind of the refusal (`RefusalKind::to_byte`). It
    /// is located by the stamp of the act that carries it; the coordinate
    /// inside the sentence went to the voice with the verdict, and the
    /// sentence itself is never copied here (LD0).
    Refusal { chart: Invariant, kind: u8 },
}

impl Cite {
    /// The stamp the citation locates by — a word's or a root's. A docket's
    /// class or demand is located by its coordinates, never by a stamp.
    pub fn at(&self) -> Option<Stamp> {
        match self {
            Cite::Word { at, .. } | Cite::Root { at, .. } => Some(*at),
            Cite::Class { .. } | Cite::Demand { .. } | Cite::Refusal { .. } => None,
        }
    }

    /// A docket entry's cover, where it is one and its cover read.
    pub fn cover(&self) -> Option<&[Vec<u8>]> {
        match self {
            Cite::Class { cover, .. } | Cite::Demand { cover, .. } => cover.as_deref(),
            Cite::Word { .. } | Cite::Root { .. } | Cite::Refusal { .. } => None,
        }
    }

    /// `None` where a class key reaches its u16 wall; nothing is written
    /// then (LD0).
    #[must_use = "a citation that cannot be written is refused by name, never dropped (LD0)"]
    pub fn put_canonical(&self, out: &mut Vec<u8>) -> Option<()> {
        let mut rec = Vec::new();
        match self {
            Cite::Word { chart, class, at } => {
                rec.push(0);
                chart.put_canonical(&mut rec);
                match class {
                    None => rec.push(0),
                    Some(c) => {
                        rec.push(1);
                        crate::transducer::address::put_class_record(&mut rec, &c.key, c.order)?;
                    }
                }
                at.put_canonical(&mut rec);
            }
            Cite::Root { at, hash } => {
                rec.push(1);
                at.put_canonical(&mut rec);
                rec.extend_from_slice(hash);
            }
            Cite::Refusal { chart, kind } => {
                rec.push(4);
                chart.put_canonical(&mut rec);
                rec.push(*kind);
            }
            Cite::Class { class, cover, .. } | Cite::Demand { class, cover, .. } => {
                rec.push(match self {
                    Cite::Class { .. } => 2,
                    _ => 3,
                });
                class.put_canonical(&mut rec);
                match self {
                    Cite::Class { rotor, .. } => rotor.put_canonical(&mut rec),
                    Cite::Demand { residue: None, .. } => rec.push(0),
                    Cite::Demand { residue: Some(r), .. } => {
                        rec.push(1);
                        r.to_bytes(&mut rec);
                    }
                    Cite::Word { .. } | Cite::Root { .. } | Cite::Refusal { .. } => {}
                }
                match cover {
                    None => rec.push(0),
                    Some(charts) => {
                        rec.push(1);
                        crate::transducer::address::put_u32_count(&mut rec, charts.len())?;
                        for c in charts {
                            crate::transducer::address::put_bytes(&mut rec, c)?;
                        }
                    }
                }
            }
        }
        out.append(&mut rec);
        Some(())
    }

    pub fn take_canonical(b: &[u8], p: &mut usize) -> Option<Cite> {
        let tag = *b.get(*p)?;
        *p += 1;
        match tag {
            0 => {
                let chart = Invariant::take_canonical(b, p)?;
                let flag = *b.get(*p)?;
                *p += 1;
                let class = match flag {
                    0 => None,
                    1 => {
                        let (key, order) = crate::transducer::address::take_class_record(b, p)?;
                        Some(ClassRecord { key, order })
                    }
                    _ => return None,
                };
                let at = Stamp::take_canonical(b, p)?;
                Some(Cite::Word { chart, class, at })
            }
            1 => {
                let at = Stamp::take_canonical(b, p)?;
                let hash: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                Some(Cite::Root { at, hash })
            }
            4 => {
                let chart = Invariant::take_canonical(b, p)?;
                let kind = *b.get(*p)?;
                *p += 1;
                Some(Cite::Refusal { chart, kind })
            }
            2 | 3 => {
                let class = Box::new(crate::transducer::sentence::ClassVector::take_canonical(b, p)?);
                let rotor = match tag {
                    2 => Some(Box::new(crate::transducer::alphabet::Rotor::from_bytes(b, p)?)),
                    _ => None,
                };
                // 78.47 — a demand's residue, as its canonical ray.
                let residue = match (tag, rotor.is_some()) {
                    (3, false) => {
                        let flag = *b.get(*p)?;
                        *p += 1;
                        match flag {
                            0 => None,
                            1 => Some(crate::transducer::cga::Mv::from_bytes(b, p)?),
                            _ => return None,
                        }
                    }
                    _ => None,
                };
                let cover = match *b.get(*p)? {
                    0 => {
                        *p += 1;
                        None
                    }
                    1 => {
                        *p += 1;
                        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
                        *p += 4;
                        // Each chart is at least its length; a count the body
                        // cannot hold refuses before anything is reserved.
                        if n > b.len().checked_sub(*p)? / 4 {
                            return None;
                        }
                        let mut charts = Vec::with_capacity(n);
                        for _ in 0..n {
                            charts.push(crate::transducer::address::take_bytes(b, p)?);
                        }
                        Some(charts)
                    }
                    _ => return None,
                };
                Some(match rotor {
                    Some(rotor) => Cite::Class { class, rotor, cover },
                    None => Cite::Demand { class, residue, cover },
                })
            }
            _ => None,
        }
    }
}

/// ONE NODE OF THE LEDGER (78.24): the act at its stamp. What it measured
/// on the four lanes, the signer's previous receipt and the peers it
/// cites, each by stamp and hash — the DAG's edges — and the words it
/// names, by their coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KStepReceipt {
    pub stamp: Stamp,
    /// The act admitted (45.1).
    pub kind: ActKind,
    /// THE FRAME, `S:G:T:H` (45.1, 45.9, 78.21): what the act measured,
    /// per lane, never summed and never compared across lanes (law 5). The
    /// zero vector is a reading, not an absence.
    pub mu: [u64; 4],
    /// This signer's previous receipt; none at genesis.
    pub prev: Option<Parent>,
    /// Peer receipts this act verified.
    pub parents: Vec<Parent>,
    /// What the act names.
    pub cites: Vec<Cite>,
    /// Which operation was applied (an act digest — never reasoned on).
    pub operator_hash: [u8; 32],
    /// This receipt's chain hash.
    pub chain_hash: [u8; 32],
}

/// Receipt payload identity of admitted content bytes (Part IX: the
/// object of the hash is the bytes written on the receipt).
#[expect(
    clippy::disallowed_methods,
    reason = "law 2, Part IX — the object of this hash is the bytes written on the receipt, which are the receipt"
)]
pub fn payload_hash(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

#[expect(
    clippy::disallowed_methods,
    reason = "law 2, Part IX — the object is the two heads' bytes as the receipt chains them"
)]
pub fn operator_hash(voice: &[u8; 32], court_head: &[u8; 32]) -> [u8; 32] {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(voice);
    buf[32..].copy_from_slice(court_head);
    *blake3::hash(&buf).as_bytes()
}

impl KStepReceipt {
    /// The least tape one node takes: its stamp, kind, frame, no
    /// predecessor, no parent, no citation, and the two hashes — what a
    /// reader allots one node when it bounds an untrusted count.
    pub const MIN_TAPE: usize = Stamp::LEN + 1 + 32 + 1 + 4 + 4 + 32 + 32;

    /// SEAL a node: its chain hash taken over its own canonical
    /// coordinates. `None` where a citation cannot be written (LD0).
    pub fn seal(
        stamp: Stamp,
        kind: ActKind,
        mu: [u64; 4],
        prev: Option<Parent>,
        parents: Vec<Parent>,
        cites: Vec<Cite>,
        operator_hash: [u8; 32],
    ) -> Option<KStepReceipt> {
        let mut r = KStepReceipt { stamp, kind, mu, prev, parents, cites, operator_hash, chain_hash: [0; 32] };
        r.chain_hash = r.hash()?;
        Some(r)
    }

    /// The predecessor's hash, or the empty hash at genesis — the reading
    /// of absence a signed prefix carries (47.1).
    pub fn prev_hash(&self) -> [u8; 32] {
        self.prev.map_or([0u8; 32], |p| p.hash)
    }

    /// THE NODE'S CANONICAL COORDINATES (78.24 (c)): its stamp, kind,
    /// frame, predecessor, parents, citations and operator, in that order.
    fn put_body(&self, out: &mut Vec<u8>) -> Option<()> {
        self.stamp.put_canonical(out);
        out.push(self.kind.to_byte());
        for lane in &self.mu {
            out.extend_from_slice(&lane.to_le_bytes());
        }
        match &self.prev {
            None => out.push(0),
            Some(p) => {
                out.push(1);
                p.put(out);
            }
        }
        crate::transducer::address::put_u32_count(out, self.parents.len())?;
        for p in &self.parents {
            p.put(out);
        }
        crate::transducer::address::put_u32_count(out, self.cites.len())?;
        for c in &self.cites {
            c.put_canonical(out)?;
        }
        out.extend_from_slice(&self.operator_hash);
        Some(())
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "law 2, Part IX — the object is the receipt's own canonical coordinates, chained to its parents' hashes"
    )]
    fn hash(&self) -> Option<[u8; 32]> {
        let mut buf = Vec::new();
        self.put_body(&mut buf)?;
        Some(*blake3::hash(&buf).as_bytes())
    }

    /// Does this node's chain hash re-derive from its own coordinates. A
    /// receipt presented by a third party (47.4) is believed only through
    /// this.
    pub fn rederives(&self) -> bool {
        self.hash() == Some(self.chain_hash)
    }

    /// The node written out: its coordinates, then its hash. `None` where
    /// a citation cannot be written (LD0).
    #[must_use = "a node that cannot be written is refused by name, never dropped (LD0)"]
    pub fn write(&self, out: &mut Vec<u8>) -> Option<()> {
        let mut rec = Vec::new();
        self.put_body(&mut rec)?;
        rec.extend_from_slice(&self.chain_hash);
        out.append(&mut rec);
        Some(())
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Self> {
        let stamp = Stamp::take_canonical(b, pos)?;
        let kind = ActKind::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let mut mu = [0u64; 4];
        for lane in mu.iter_mut() {
            *lane = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let flag = *b.get(*pos)?;
        *pos += 1;
        let prev = match flag {
            0 => None,
            1 => Some(Parent::take(b, pos)?),
            _ => return None,
        };
        let count = |b: &[u8], pos: &mut usize, least: usize| -> Option<usize> {
            let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
            *pos += 4;
            (n <= b.len().checked_sub(*pos)? / least).then_some(n)
        };
        let np = count(b, pos, Stamp::LEN + 32)?;
        let mut parents = Vec::with_capacity(np);
        for _ in 0..np {
            parents.push(Parent::take(b, pos)?);
        }
        let nc = count(b, pos, 1 + Stamp::LEN)?;
        let mut cites = Vec::with_capacity(nc);
        for _ in 0..nc {
            cites.push(Cite::take_canonical(b, pos)?);
        }
        let operator_hash: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let chain_hash: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        Some(KStepReceipt { stamp, kind, mu, prev, parents, cites, operator_hash, chain_hash })
    }

    /// CK1 — THE STAMP AS IT PRINTS, `h:S:G:T:H:K.i`: the tick, the frame
    /// the act measured, the signer's orbit and the act's place in it.
    pub fn print(&self) -> String {
        let [s, g, t, h] = self.mu;
        format!("{}:{s}:{g}:{t}:{h}:{}", self.stamp.h, self.stamp.place().print())
    }
}
