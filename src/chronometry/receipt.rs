// 45, 78.24 — THE RECEIPT: one act at its stamp, the parents it cites by
// their stamps, the frame it measured, and the words it names by their
// coordinates.

use crate::transducer::address::Address;
use crate::transducer::stalk::Invariant;

/// THE STAMP (78.21, 78.24): where one act stands in the ledger — the tick
/// `h` (TUniversal, 45.2), the signer's key, and the signer's own `K`.
///
/// It is the ledger's KEY. Written coarsest first — `h`, then `vk`, then
/// `K` — so a prefix is a stratum: `h` alone is everything the mesh did at
/// one tick, `h‖vk` one signer at that tick. The two integers are written
/// BIG-endian, the one place the tree does so, because in a key the byte
/// order IS the order the trie is read in, and big-endian is the order in
/// which a smaller number reads first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stamp {
    pub h: u64,
    pub vk: [u8; 32],
    pub k: u64,
}

impl Stamp {
    /// A stamp's coordinates: the tick, the key, the step.
    pub const LEN: usize = 8 + 32 + 8;

    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.h.to_be_bytes());
        out.extend_from_slice(&self.vk);
        out.extend_from_slice(&self.k.to_be_bytes());
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
        let k = u64::from_be_bytes(b.get(*p + 40..*p + 48)?.try_into().ok()?);
        *p += Self::LEN;
        Some(Stamp { h, vk, k })
    }

    /// The stamp a key names, exactly or not at all.
    pub fn from_key(b: &[u8]) -> Option<Stamp> {
        let mut p = 0usize;
        let s = Self::take_canonical(b, &mut p)?;
        (p == b.len()).then_some(s)
    }

    /// The signer's own line, keyed by `vk‖K`: the relation the range
    /// readers ask by (78.24 P1). A coordinate, never the act.
    pub fn line_key(vk: &[u8; 32], k: u64) -> Vec<u8> {
        let mut out = Vec::with_capacity(40);
        out.extend_from_slice(vk);
        out.extend_from_slice(&k.to_be_bytes());
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
    fn put(&self, out: &mut Vec<u8>) {
        self.at.put_canonical(out);
        out.extend_from_slice(&self.hash);
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Parent> {
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

/// THE KIND OF ACT A RECEIPT ADMITS (45.1): one coordinate byte of the
/// node, in its chain hash, so a phase (45.9) and a fold (45.8) read the
/// ledger by kind. A byte that names no kind refuses the read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActKind {
    Attest,
    List,
    Section,
    Install,
    Ruling,
    Syllabus,
    Lapse,
    Fold,
    Adopted,
    Stood,
    Uttered,
    Learned,
    Settled,
    /// 50.9 (L8): a court refused this mind's sentence, and the mind
    /// contracted it.
    Refused,
}

impl ActKind {
    /// The kind as the coordinate byte the node carries.
    pub fn to_byte(self) -> u8 {
        match self {
            ActKind::Attest => 1,
            ActKind::List => 2,
            ActKind::Section => 3,
            ActKind::Install => 4,
            ActKind::Ruling => 5,
            ActKind::Syllabus => 6,
            ActKind::Lapse => 7,
            ActKind::Fold => 8,
            ActKind::Adopted => 9,
            ActKind::Stood => 10,
            ActKind::Uttered => 11,
            ActKind::Learned => 12,
            ActKind::Settled => 13,
            ActKind::Refused => 14,
        }
    }

    /// The kind a byte names; `None` names no kind and refuses.
    pub fn from_byte(b: u8) -> Option<ActKind> {
        Some(match b {
            1 => ActKind::Attest,
            2 => ActKind::List,
            3 => ActKind::Section,
            4 => ActKind::Install,
            5 => ActKind::Ruling,
            6 => ActKind::Syllabus,
            7 => ActKind::Lapse,
            8 => ActKind::Fold,
            9 => ActKind::Adopted,
            10 => ActKind::Stood,
            11 => ActKind::Uttered,
            12 => ActKind::Learned,
            13 => ActKind::Settled,
            14 => ActKind::Refused,
            _ => return None,
        })
    }

    /// The kind's name, the bytes the mind's operator digest is taken over.
    pub fn name(self) -> &'static [u8] {
        match self {
            ActKind::Attest => b"attest",
            ActKind::List => b"list",
            ActKind::Section => b"section",
            ActKind::Install => b"install",
            ActKind::Ruling => b"ruling",
            ActKind::Syllabus => b"syllabus",
            ActKind::Lapse => b"lapse",
            ActKind::Fold => b"fold",
            ActKind::Adopted => b"adopted",
            ActKind::Stood => b"stood",
            ActKind::Uttered => b"uttered",
            ActKind::Learned => b"learned",
            ActKind::Settled => b"settled",
            ActKind::Refused => b"refused",
        }
    }

    /// The operator digest of a mind's own act (never reasoned on).
    pub fn operator(self) -> [u8; 32] {
        payload_hash(self.name())
    }
}

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
        Cite::Word { chart: crate::transducer::stalk::invariant_of(w), class: ClassRecord::of(w), at }
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
}

impl Cite {
    /// The stamp the citation locates by.
    pub fn at(&self) -> Stamp {
        match self {
            Cite::Word { at, .. } | Cite::Root { at, .. } => *at,
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
        out.extend_from_slice(&(self.parents.len() as u32).to_le_bytes());
        for p in &self.parents {
            p.put(out);
        }
        out.extend_from_slice(&(self.cites.len() as u32).to_le_bytes());
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

    /// CK1 — THE STAMP AS IT PRINTS, `h:S:G:T:H:k`: the tick, the frame
    /// the act measured, the signer's `K`.
    pub fn print(&self) -> String {
        let [s, g, t, h] = self.mu;
        format!("{}:{s}:{g}:{t}:{h}:{}", self.stamp.h, self.stamp.k)
    }
}
