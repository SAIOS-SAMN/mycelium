// 78.139 — A UNIVERSE STATES ITSELF ON THE COURT (the coordinator's rulings on
// (i-b), the operator's ruling that a galactic core is a larger wormhole):
// its space (78.140: its GEOMETRY, curved or a torus) and its parent face on
// its origin (version 2), and — once each, signed by its SITTING RESERVE — its
// geometry (where its
// origin did not state it), its ancients' survival BASKET, and its JOIN to
// its parent's core once that core is found.
//
//! Every horizon has its far face: a universe's Big Bang is the far face of a
//! parent's core, joined by a CORE TUBE (a mouth of kind 16, kept by both
//! reserves). A root universe states its parent face UNWITNESSED until it is
//! joined. The basket is the ancients': stated when they take up the
//! reserve, never at genesis, where nobody knows it.

/// ACT_DOMAIN's sub-kind mark (as `domain.rs` writes it).
const SUB_KIND: [u8; 2] = [0x80, 0x00];
/// 78.140 — sub-kind 8 states a GEOMETRY (it stated a bare curvature before
/// first use).
pub const KIND_GEOMETRY: u8 = 8;
pub const KIND_BASKET: u8 = 9;
pub const KIND_JOIN: u8 = 10;
pub const KIND_READ_UNIVERSE: u8 = 11;

/// A UNIVERSE'S PARENT FACE: the core tube it bounced out of, by the tube's
/// id, or UNWITNESSED — a root, joined when its parent is found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parent {
    Unwitnessed,
    Core([u8; 32]),
}

impl Parent {
    pub fn put(&self, out: &mut Vec<u8>) {
        match self {
            Parent::Unwitnessed => out.push(0),
            Parent::Core(t) => {
                out.push(1);
                out.extend_from_slice(t);
            }
        }
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<Parent> {
        let tag = *b.get(*p)?;
        *p += 1;
        match tag {
            0 => Some(Parent::Unwitnessed),
            1 => {
                let t: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                Some(Parent::Core(t))
            }
            _ => None,
        }
    }
}

fn put_q(out: &mut Vec<u8>, (n, d): (u64, u64)) {
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(&d.to_le_bytes());
}

fn take_q(b: &[u8], p: &mut usize) -> Option<(u64, u64)> {
    let n = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    let d = u64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
    *p += 16;
    Some((n, d))
}

/// An exact rational strictly positive: `n, d > 0`.
pub fn positive((n, d): (u64, u64)) -> bool {
    n > 0 && d > 0
}

/// 78.140 — A TORUS'S METRIC: L1 (the wrapped `|dx|` summed the short way
/// round — netstratum's, which squares no circle on a toroid) or L2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metric {
    L1,
    L2,
}

/// 78.140 — A UNIVERSE'S SPACE (the operator's ruling, the Concordance's
/// 2.3 as amended): a closed CURVED space of radius `R`, or a flat TORUS of
/// axis-aligned periods (one to three) and its metric. Every distance is
/// that space's own geodesic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Geometry {
    Curved((u64, u64)),
    Torus { periods: Vec<(u64, u64)>, metric: Metric },
}

impl Geometry {
    pub fn put(&self, out: &mut Vec<u8>) {
        match self {
            Geometry::Curved(r) => {
                out.push(0);
                put_q(out, *r);
            }
            Geometry::Torus { periods, metric } => {
                out.push(1);
                crate::transducer::address::put_key_count(out, periods.len());
                for p in periods {
                    put_q(out, *p);
                }
                out.push(match metric {
                    Metric::L1 => 0,
                    Metric::L2 => 1,
                });
            }
        }
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<Geometry> {
        let tag = *b.get(*p)?;
        *p += 1;
        match tag {
            0 => Some(Geometry::Curved(take_q(b, p)?)),
            1 => {
                let k = crate::transducer::address::take_key_count(b, p)?;
                let periods = (0..k).map(|_| take_q(b, p)).collect::<Option<Vec<_>>>()?;
                let metric = match *b.get(*p)? {
                    0 => Metric::L1,
                    1 => Metric::L2,
                    _ => return None,
                };
                *p += 1;
                Some(Geometry::Torus { periods, metric })
            }
            _ => None,
        }
    }

    /// Well formed: a positive radius; one to three positive periods.
    pub fn holds(&self) -> bool {
        match self {
            Geometry::Curved(r) => positive(*r),
            Geometry::Torus { periods, .. } => (1..=3).contains(&periods.len()) && periods.iter().all(|p| positive(*p)),
        }
    }

    /// A torus's shortest period, exact; `None` for a curved space.
    pub fn shortest_period(&self) -> Option<num_rational::BigRational> {
        let Geometry::Torus { periods, .. } = self else { return None };
        let mut least: Option<num_rational::BigRational> = None;
        for (n, d) in periods {
            if *d == 0 {
                return None;
            }
            let q = num_rational::BigRational::new((*n).into(), (*d).into());
            least = match least {
                Some(l) if l <= q => Some(l),
                _ => Some(q),
            };
        }
        least
    }
}

/// 78.139 — WHAT AN ORIGIN OF VERSION 2 STATES OF ITS UNIVERSE: its space
/// (78.140: a GEOMETRY, curved or a torus — v2's bytes revised before first
/// use) in clear, and its parent face.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Genesis {
    pub geometry: Geometry,
    pub parent: Parent,
}

impl Genesis {
    pub fn put(&self, out: &mut Vec<u8>) {
        self.geometry.put(out);
        self.parent.put(out);
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<Genesis> {
        Some(Genesis { geometry: Geometry::take(b, p)?, parent: Parent::take(b, p)? })
    }
}

/// THE ANCIENTS' SURVIVAL BASKET: one person-year of their survival at the
/// core, by mass per element (`Z`, mass) — species-neutral, a molecule
/// stated as its elements' masses — in ascending `Z`, each once.
pub type Basket = Vec<(u16, (u64, u64))>;

/// The basket well formed: not empty, `Z` strictly ascending, every mass a
/// positive exact rational.
pub fn basket_holds(b: &Basket) -> bool {
    !b.is_empty() && b.windows(2).all(|w| matches!(w, [x, y] if x.0 < y.0)) && b.iter().all(|(_, m)| positive(*m))
}

/// WHAT A UNIVERSE STATES, once each, by its sitting reserve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stated {
    /// Its space, where its origin did not state it (a v1 origin, or none
    /// yet).
    Geometry(Geometry),
    /// Its ancients' survival basket.
    Basket(Basket),
    /// Its join to its parent's core: a root's open core tube, by its id.
    Join([u8; 32]),
}

impl Stated {
    pub fn kind(&self) -> u8 {
        match self {
            Stated::Geometry(_) => KIND_GEOMETRY,
            Stated::Basket(_) => KIND_BASKET,
            Stated::Join(_) => KIND_JOIN,
        }
    }

    fn put_body(&self, out: &mut Vec<u8>) {
        match self {
            Stated::Geometry(g) => g.put(out),
            Stated::Basket(entries) => {
                crate::transducer::address::put_key_count(out, entries.len());
                for (z, m) in entries {
                    out.extend_from_slice(&z.to_be_bytes());
                    put_q(out, *m);
                }
            }
            Stated::Join(t) => out.extend_from_slice(t),
        }
    }

    /// The statement's one writing hashed: what its receipt cites.
    pub fn hash(&self) -> [u8; 32] {
        let mut b = vec![self.kind()];
        self.put_body(&mut b);
        crate::chronometry::payload_hash(&b)
    }
}

/// A UNIVERSE'S STATEMENT: the domain and what it states.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    pub domain: [u8; 32],
    pub stated: Stated,
}

const STATEMENT: &[u8] = b"mycelium universe statement v1";

impl Statement {
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = STATEMENT.to_vec();
        out.push(self.stated.kind());
        out.extend_from_slice(&self.domain);
        self.stated.put_body(&mut out);
        out
    }

    /// Signed by the domain's sitting reserve.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedStatement {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedStatement { statement: self, sig }
    }

    pub(super) fn take(kind: u8, domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<Statement> {
        let stated = match kind {
            KIND_GEOMETRY => Stated::Geometry(Geometry::take(b, p)?),
            KIND_BASKET => {
                let n = crate::transducer::address::take_key_count(b, p)?;
                let entries = (0..n)
                    .map(|_| {
                        let z = u16::from_be_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
                        *p += 2;
                        Some((z, take_q(b, p)?))
                    })
                    .collect::<Option<Basket>>()?;
                Stated::Basket(entries)
            }
            KIND_JOIN => {
                let t: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                Stated::Join(t)
            }
            _ => return None,
        };
        Some(Statement { domain, stated })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedStatement {
    pub statement: Statement,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][8 | 9 | 10][body][sig 64]`.
pub fn encode_statement(s: &SignedStatement) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&s.statement.domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(s.statement.stated.kind());
    s.statement.stated.put_body(&mut out);
    out.extend_from_slice(&s.sig);
    out
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][11]`.
pub fn encode_universe_read(domain: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(KIND_READ_UNIVERSE);
    out
}

/// 78.139 — WHAT A UNIVERSE HAS STATED, as read: whether it has an origin
/// (it reads as a universe), its space (78.140: its geometry), its parent
/// face (none for a domain with no origin, or a v1 origin never joined), and
/// its basket (empty: not yet stated).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UniverseReading {
    pub originated: bool,
    pub geometry: Option<Geometry>,
    pub parent: Option<Parent>,
    pub basket: Basket,
}

/// `[originated][0 | 1 geometry][0 | 1 parent][basket]`.
pub fn encode_universe_reading(r: &UniverseReading) -> Vec<u8> {
    let mut out = vec![u8::from(r.originated)];
    match &r.geometry {
        None => out.push(0),
        Some(g) => {
            out.push(1);
            g.put(&mut out);
        }
    }
    match &r.parent {
        None => out.push(0),
        Some(p) => {
            out.push(1);
            p.put(&mut out);
        }
    }
    Stated::Basket(r.basket.clone()).put_body(&mut out);
    out
}

pub fn decode_universe_reading(b: &[u8]) -> Option<UniverseReading> {
    let mut p = 0;
    let flag = |b: &[u8], p: &mut usize| -> Option<bool> {
        let f = match *b.get(*p)? {
            0 => false,
            1 => true,
            _ => return None,
        };
        *p += 1;
        Some(f)
    };
    let originated = flag(b, &mut p)?;
    let geometry = match flag(b, &mut p)? {
        false => None,
        true => Some(Geometry::take(b, &mut p)?),
    };
    let parent = match flag(b, &mut p)? {
        false => None,
        true => Some(Parent::take(b, &mut p)?),
    };
    let Stated::Basket(basket) = Statement::take(KIND_BASKET, [0; 32], b, &mut p)?.stated else { return None };
    (p == b.len()).then_some(UniverseReading { originated, geometry, parent, basket })
}

/// 78.139 — A UNIVERSE AS HELD on the court: its space (78.140: its
/// geometry), its parent face, and its basket — each stated once.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Universe {
    pub geometry: Option<Geometry>,
    pub parent: Option<Parent>,
    pub basket: Basket,
}

impl Universe {
    pub fn put(&self, out: &mut Vec<u8>) {
        let r = UniverseReading { originated: false, geometry: self.geometry.clone(), parent: self.parent, basket: self.basket.clone() };
        out.extend_from_slice(&encode_universe_reading(&r)[1..]);
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<Universe> {
        let flag = |b: &[u8], p: &mut usize| -> Option<bool> {
            let f = match *b.get(*p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            *p += 1;
            Some(f)
        };
        let geometry = match flag(b, p)? {
            false => None,
            true => Some(Geometry::take(b, p)?),
        };
        let parent = match flag(b, p)? {
            false => None,
            true => Some(Parent::take(b, p)?),
        };
        let Stated::Basket(basket) = Statement::take(KIND_BASKET, [0; 32], b, p)?.stated else { return None };
        Some(Universe { geometry, parent, basket })
    }
}
