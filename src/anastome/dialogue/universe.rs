// 78.139 — A UNIVERSE STATES ITSELF ON THE COURT (the rulings on
// (i-b), and the ruling that a galactic core is a larger wormhole):
// its space (78.140: its GEOMETRY, curved or a torus) and its parent face on
// its origin (version 2), and — once each, signed by its SITTING RESERVE — its
// geometry (where its
// origin did not state it), its reserve's survival BASKET, and its JOIN to
// its parent's core once that core is found.
//
//! Every horizon has its far face: a universe's Big Bang is the far face of a
//! parent's core, joined by a CORE TUBE (a mouth of kind 16, kept by both
//! reserves). A root universe states its parent face UNWITNESSED until it is
//! joined. The basket is the reserve's: stated when it takes up the
//! reserve, never at genesis, where nobody knows it.

use num_bigint::BigUint;
use num_traits::Zero;

/// ACT_DOMAIN's sub-kind mark (as `domain.rs` writes it).
/// FORMAT: sub-kind mark bytes for ACT_DOMAIN.
const SUB_KIND: [u8; 2] = [0x80, 0x00];
/// 78.140 — sub-kind 8 states a GEOMETRY (it stated a bare curvature before
/// first use).
/// FORMAT: sub-kind byte of ACT_DOMAIN for a geometry statement.
pub const KIND_GEOMETRY: u8 = 8;
/// FORMAT: sub-kind byte of ACT_DOMAIN for a basket statement.
pub const KIND_BASKET: u8 = 9;
/// FORMAT: sub-kind byte of ACT_DOMAIN for a join.
pub const KIND_JOIN: u8 = 10;
/// FORMAT: sub-kind byte of ACT_DOMAIN for a universe read.
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
/// round, which squares no circle on a toroid) or L2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metric {
    L1,
    L2,
}

/// 78.140 — A UNIVERSE'S SPACE (as ruled, the Concordance's
/// 2.3 as amended): a CURVED space of radius `R` — HYPERBOLIC `ℍ³`, of
/// negative curvature (78.134: its frames in `Spin⁺(3,1)`, its places on the
/// sheet `v² = −1`, a throat's level `cosh(2m/R)`), never a sphere; a
/// closed SPHERE `S³` of radius `R` (2026-10-22: its frames in `Spin(4)`,
/// its places on `v² = +1` about `e₊`, a throat's level `cos(2m/R)` — the
/// kind a closed universe is, which before it existed could be declared
/// only `Curved` and so stated as hyperbolic); or a flat TORUS
/// of axis-aligned periods (one to three) and its metric. Every distance is
/// that space's own geodesic. `R` and the periods are lengths in GEOMETRIZED
/// MEGAPARSECS (Concordance 1.8, 2.3): G = c = 1, the same unit a throat's
/// mass is written in, so `2m/R` and `2m/L_min` read true.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Geometry {
    Curved((u64, u64)),
    Torus { periods: Vec<(u64, u64)>, metric: Metric },
    Sphere((u64, u64)),
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
            Geometry::Sphere(r) => {
                out.push(2);
                put_q(out, *r);
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
            2 => Some(Geometry::Sphere(take_q(b, p)?)),
            _ => None,
        }
    }

    /// Well formed: a positive radius; one to three positive periods.
    pub fn holds(&self) -> bool {
        match self {
            Geometry::Curved(r) | Geometry::Sphere(r) => positive(*r),
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

/// ONE ELEMENT OF A SURVIVAL BASKET (#238; 78.182 item 10, with the
/// operator's ruling of 2026-11-12 that the counts are the portion's
/// totals): the element, named by the protons `Z` of one of its nuclei, and
/// the person-year portion of it, held as its dipole by four whole counts:
/// - `protons`, the portion's captured proton charge, `Z_total`;
/// - `neutrons`, its neutron count `Q_total`, an integer the reserve states
///   (the universe's own rest point read back to whole nucleons; per
///   universe, checked against no table, Concordance 1.13);
/// - `bound_electrons`, `E_bound = 3·Z_total − U`, the held electron states
///   screening the cores;
/// - `mass`, `M = 3(Z_total + Q_total)`, the portion's whole nucleon hold in
///   first-degree units.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasketEntry {
    pub element: u16,
    pub protons: BigUint,
    pub neutrons: BigUint,
    pub bound_electrons: BigUint,
    pub mass: BigUint,
}

impl BasketEntry {
    /// THE PORTION OF `nuclei` NUCLEI of the element of `element` protons,
    /// carrying `neutrons` neutrons and `bound_electrons` held electron
    /// states in all: its protons and its mass derived, never stated twice.
    pub fn of(element: u16, nuclei: BigUint, neutrons: BigUint, bound_electrons: BigUint) -> BasketEntry {
        let protons = BigUint::from(element) * nuclei;
        let mass = BigUint::from(3u8) * (&protons + &neutrons);
        BasketEntry { element, protons, neutrons, bound_electrons, mass }
    }

    /// The entry well formed: an element with protons; the portion whole
    /// nuclei of it (`protons` a multiple of `element`); the bound electrons
    /// at most the frames the protons hold (`E_bound ≤ 3·Z_total`, so
    /// `U ≥ 0`); and the mass the nucleons' first-degree hold exactly,
    /// `M = 3(Z_total + Q_total)`.
    pub fn holds(&self) -> bool {
        let three = BigUint::from(3u8);
        let element = BigUint::from(self.element);
        self.element > 0
            && !self.protons.is_zero()
            && (&self.protons % &element).is_zero()
            && self.bound_electrons <= &three * &self.protons
            && self.mass == &three * (&self.protons + &self.neutrons)
    }

    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.element.to_be_bytes());
        for n in [&self.protons, &self.neutrons, &self.bound_electrons, &self.mass] {
            crate::transducer::address::put_key_bytes(out, &n.to_bytes_le());
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<BasketEntry> {
        let element = u16::from_be_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
        *p += 2;
        let mut count = || crate::transducer::address::take_key_bytes(b, p).map(|bytes| BigUint::from_bytes_le(&bytes));
        Some(BasketEntry { element, protons: count()?, neutrons: count()?, bound_electrons: count()?, mass: count()? })
    }
}

/// THE RESERVE'S SURVIVAL BASKET (Concordance 5.1): one person-year of
/// survival at the core for the people that holds the reserve, one entry
/// per element held as its dipole — species-neutral, a molecule stated as
/// its elements' portions — in ascending element, each once.
pub type Basket = Vec<BasketEntry>;

/// The basket well formed: not empty, the elements strictly ascending, every
/// entry well formed ([`BasketEntry::holds`]).
pub fn basket_holds(b: &Basket) -> bool {
    !b.is_empty() && b.windows(2).all(|w| matches!(w, [x, y] if x.element < y.element)) && b.iter().all(BasketEntry::holds)
}

/// WHAT A UNIVERSE STATES, once each, by its sitting reserve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stated {
    /// Its space, where its origin did not state it (a v1 origin, or none
    /// yet).
    Geometry(Geometry),
    /// Its reserve's survival basket.
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
                for e in entries {
                    e.put(out);
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

const STATEMENT: &[u8] = b"mycelium universe statement v2";

impl Statement {
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = STATEMENT.to_vec();
        out.push(self.stated.kind());
        out.extend_from_slice(&self.domain);
        self.stated.put_body(&mut out);
        out
    }

    /// Signed by the domain's sitting reserve, with the crossing it carries
    /// (X1, #160). `None` where a turn of the record cannot be written.
    pub fn sign(self, crossing: super::crossing_record::Crossing, key: &ed25519_dalek::SigningKey) -> Option<SignedStatement> {
        use ed25519_dalek::Signer;
        let sig = key.sign(&super::crossing_record::crossed(self.signed_bytes(), &crossing)?).to_bytes();
        Some(SignedStatement { statement: self, crossing: Box::new(crossing), sig })
    }

    pub(super) fn take(kind: u8, domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<Statement> {
        let stated = match kind {
            KIND_GEOMETRY => Stated::Geometry(Geometry::take(b, p)?),
            KIND_BASKET => {
                let n = crate::transducer::address::take_key_count(b, p)?;
                let entries = (0..n).map(|_| BasketEntry::take(b, p)).collect::<Option<Basket>>()?;
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
    /// X1 (#160) — the crossing the statement carries.
    pub crossing: Box<super::crossing_record::Crossing>,
    pub sig: [u8; 64],
}

impl SignedStatement {
    /// THE BYTES THE SIGNATURE IS OVER: the statement's own, then its
    /// crossing written out whole.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        super::crossing_record::crossed(self.statement.signed_bytes(), &self.crossing)
    }
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][8 | 9 | 10][body][crossing][sig 64]`.
/// `None` where a turn of the record cannot be written.
pub fn encode_statement(s: &SignedStatement) -> Option<Vec<u8>> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&s.statement.domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(s.statement.stated.kind());
    s.statement.stated.put_body(&mut out);
    s.crossing.put(&mut out)?;
    out.extend_from_slice(&s.sig);
    Some(out)
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
