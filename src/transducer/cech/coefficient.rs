// 78.123 A — THE COEFFICIENT GROUPS a cover carries on its edges: the
// group-generic core the court's complex reads cycles by, generalized so a
// domain's ledger tree is read with the same machinery the minds use.
//
//! A COEFFICIENT is an element of a group: an identity, a composition, an
//! inverse, a test for the identity, and one canonical writing. Two groups
//! stand behind it:
//!
//! - [`Lanes`] — the exact ADDITIVE LANE GROUP: per identity, a four-lane
//!   vector `(S, G, T, H)` of exact integers; composition the sum, inverse
//!   the negation, the identity the empty sum. A fold's nets are an element
//!   of it. The lanes are never summed into one, and the dimensions are
//!   identity × lane — never bounded by the algebra's axes (four lanes on
//!   four axes would be Cl(5,1): a new universe, 78.59 Ruling 2).
//! - [`Turned`] — the minds' VERSOR coefficient as the complex holds it: a
//!   rotor and a carriage, composed by the geometric product, read
//!   projectively.
//!
//! Every composition is exact; one that would pass its carrier is refused
//! (`None`, rule 11), never wrapped.

use crate::transducer::alphabet::Rotor;
use crate::transducer::element::Versor;

/// AN ELEMENT OF A COEFFICIENT GROUP.
pub trait Coefficient: Clone + std::fmt::Debug {
    /// The group's identity.
    fn identity() -> Self;
    /// `self`, then `then` — or `None` where the result passes its carrier.
    fn compose(&self, then: &Self) -> Option<Self>;
    /// The inverse, or `None` where it passes its carrier or does not exist.
    fn inverse(&self) -> Option<Self>;
    /// Whether this is the identity.
    fn is_identity(&self) -> bool;
    /// The element's one writing: equal elements write equal bytes.
    fn put_canonical(&self, out: &mut Vec<u8>);
    /// The element read back from its one writing.
    fn take_canonical(b: &[u8], p: &mut usize) -> Option<Self>;
}

/// THE RATES GROUP (78.123 B): per lane a strictly positive exact rational —
/// a translation between units, composed by product, inverted by the
/// reciprocal, the identity all ones. A cycle's holonomy is the product of
/// its rates: one where no arbitrage stands, else the exact gain per lane.
/// By the lexicon (v2) only the G lane translates; the group stays per lane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rates {
    lanes: [num_rational::BigRational; 4],
}

impl Rates {
    /// Rates read off `(numerator, denominator)` per lane, each strictly
    /// positive — `None` for a zero, a negative or a zero denominator.
    pub fn of(lanes: [(i128, i128); 4]) -> Option<Rates> {
        let mut out: [num_rational::BigRational; 4] = std::array::from_fn(|_| num_rational::BigRational::from_integer(1.into()));
        for (o, (n, d)) in out.iter_mut().zip(lanes) {
            if n <= 0 || d <= 0 {
                return None;
            }
            *o = num_rational::BigRational::new(n.into(), d.into());
        }
        Some(Rates { lanes: out })
    }

    /// The rate on each lane.
    pub fn lanes(&self) -> &[num_rational::BigRational; 4] {
        &self.lanes
    }
}

impl Coefficient for Rates {
    fn identity() -> Self {
        Rates { lanes: std::array::from_fn(|_| num_rational::BigRational::from_integer(1.into())) }
    }

    fn compose(&self, then: &Self) -> Option<Self> {
        Some(Rates { lanes: std::array::from_fn(|l| &self.lanes[l] * &then.lanes[l]) })
    }

    fn inverse(&self) -> Option<Self> {
        let mut out = Rates::identity();
        for (o, r) in out.lanes.iter_mut().zip(&self.lanes) {
            if num_traits::Zero::is_zero(r) {
                return None;
            }
            *o = num_traits::Inv::inv(r.clone());
        }
        Some(out)
    }

    fn is_identity(&self) -> bool {
        self.lanes.iter().all(num_traits::One::is_one)
    }

    fn put_canonical(&self, out: &mut Vec<u8>) {
        for r in &self.lanes {
            crate::transducer::address::put_key_bytes(out, &r.numer().to_signed_bytes_le());
            crate::transducer::address::put_key_bytes(out, &r.denom().to_signed_bytes_le());
        }
    }

    fn take_canonical(b: &[u8], p: &mut usize) -> Option<Self> {
        let mut out = Rates::identity();
        for o in &mut out.lanes {
            let n = num_bigint::BigInt::from_signed_bytes_le(&crate::transducer::address::take_key_bytes(b, p)?);
            let d = num_bigint::BigInt::from_signed_bytes_le(&crate::transducer::address::take_key_bytes(b, p)?);
            if num_traits::Zero::is_zero(&d) {
                return None;
            }
            *o = num_rational::BigRational::new(n, d);
        }
        Some(out)
    }
}

/// One identity's net per lane `S:G:T:H`, exact and signed.
pub type LaneNet = ([u8; 32], [i128; 4]);

/// THE ADDITIVE LANE GROUP: per identity, its four-lane net — held in its
/// identities' ascending order, each once, no zero vector held (the one
/// writing).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lanes {
    nets: Vec<LaneNet>,
}

impl Lanes {
    /// The element whose nets these are: summed per identity, zeros dropped,
    /// in ascending order — `None` where a sum passes its carrier.
    pub fn of(nets: &[LaneNet]) -> Option<Lanes> {
        nets.iter().try_fold(Lanes::default(), |acc, n| acc.compose(&Lanes { nets: vec![*n] }))
    }

    /// Its nets, ascending by identity.
    pub fn nets(&self) -> &[LaneNet] {
        &self.nets
    }

    /// One identity's net — zero where it holds none.
    pub fn net_of(&self, who: &[u8; 32]) -> [i128; 4] {
        match self.nets.binary_search_by(|(w, _)| w.cmp(who)) {
            Ok(i) => self.nets[i].1,
            Err(_) => [0; 4],
        }
    }
}

fn add(a: &[i128; 4], b: &[i128; 4]) -> Option<[i128; 4]> {
    let mut out = [0i128; 4];
    for (o, (x, y)) in out.iter_mut().zip(a.iter().zip(b)) {
        *o = x.checked_add(*y)?;
    }
    Some(out)
}

impl Coefficient for Lanes {
    fn identity() -> Self {
        Lanes::default()
    }

    /// The sum, merged in ascending order of identities; an identity whose
    /// net sums to zero leaves the writing.
    fn compose(&self, then: &Self) -> Option<Self> {
        let (mut a, mut b) = (self.nets.iter().peekable(), then.nets.iter().peekable());
        let mut out = Vec::with_capacity(self.nets.len() + then.nets.len());
        loop {
            let next = match (a.peek(), b.peek()) {
                (Some(x), Some(y)) if x.0 == y.0 => {
                    let v = add(&x.1, &y.1)?;
                    let who = x.0;
                    a.next();
                    b.next();
                    (who, v)
                }
                (Some(x), Some(y)) if x.0 < y.0 => *a.next()?,
                (Some(_), Some(_)) => *b.next()?,
                (Some(_), None) => *a.next()?,
                (None, Some(_)) => *b.next()?,
                (None, None) => break,
            };
            if next.1.iter().any(|v| *v != 0) {
                out.push(next);
            }
        }
        Some(Lanes { nets: out })
    }

    fn inverse(&self) -> Option<Self> {
        let nets = self
            .nets
            .iter()
            .map(|(who, v)| {
                let mut n = [0i128; 4];
                for (o, x) in n.iter_mut().zip(v) {
                    *o = x.checked_neg()?;
                }
                Some((*who, n))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Lanes { nets })
    }

    fn is_identity(&self) -> bool {
        self.nets.is_empty()
    }

    fn put_canonical(&self, out: &mut Vec<u8>) {
        crate::transducer::address::put_key_count(out, self.nets.len());
        for (who, v) in &self.nets {
            out.extend_from_slice(who);
            for x in v {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
    }

    fn take_canonical(b: &[u8], p: &mut usize) -> Option<Self> {
        let n = crate::transducer::address::take_key_count(b, p)?;
        let nets = (0..n)
            .map(|_| {
                let who: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                let mut v = [0i128; 4];
                for x in &mut v {
                    *x = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
                    *p += 16;
                }
                Some((who, v))
            })
            .collect::<Option<Vec<_>>>()?;
        Lanes::of(&nets)
    }
}

/// THE VERSOR COEFFICIENT the minds' complex holds on an edge: its rotor and
/// its carriage, composed by the geometric product and read projectively.
#[derive(Clone, Debug)]
pub struct Turned {
    pub rotor: Rotor,
    pub carriage: Versor,
}

impl Coefficient for Turned {
    fn identity() -> Self {
        Turned { rotor: Rotor::identity(), carriage: crate::transducer::element::identity_versor() }
    }

    /// IN PATH ORDER, as the minds compose (78.123 C): `self`, then `then`,
    /// the geometric product left to right along the path — `r_ij · r_jk ·
    /// r_ki` around a triangle, the product `Complex::holonomy` and the
    /// coboundary have always taken. Composed the other way the holonomy is
    /// another element and its class vector another class.
    fn compose(&self, then: &Self) -> Option<Self> {
        Some(Turned { rotor: self.rotor.geo(&then.rotor), carriage: self.carriage.geo(&then.carriage) })
    }

    /// The reverse, which is the inverse up to the weight the projective
    /// reading absorbs.
    fn inverse(&self) -> Option<Self> {
        Some(Turned { rotor: self.rotor.reverse(), carriage: self.carriage.reverse() })
    }

    fn is_identity(&self) -> bool {
        self.rotor.is_identity() && crate::transducer::element::same_versor(&self.carriage, &crate::transducer::element::identity_versor())
    }

    fn put_canonical(&self, out: &mut Vec<u8>) {
        self.rotor.put_canonical(out);
        self.carriage.put_canonical(out);
    }

    fn take_canonical(b: &[u8], p: &mut usize) -> Option<Self> {
        let rotor = Rotor::from_bytes(b, p)?;
        let carriage = Versor::from_bytes(b, p)?;
        Some(Turned { rotor, carriage })
    }
}

/// 78.134 — A MOTION ALONE: the coefficient of an edge that carries a
/// transition of space and no orientation letter — a wormhole's chart
/// transition `to · from⁻¹`, in `Spin⁺(3,1)` where its frames are. Composed
/// in path order by the geometric product, read projectively; no rotor slot
/// pretends to be read where none is held.
#[derive(Clone, Debug)]
pub struct Moved {
    pub motion: Versor,
}

impl Coefficient for Moved {
    fn identity() -> Self {
        Moved { motion: crate::transducer::element::identity_versor() }
    }

    fn compose(&self, then: &Self) -> Option<Self> {
        Some(Moved { motion: self.motion.geo(&then.motion) })
    }

    /// The reverse, the inverse up to the weight the projective reading
    /// absorbs.
    fn inverse(&self) -> Option<Self> {
        Some(Moved { motion: self.motion.reverse() })
    }

    fn is_identity(&self) -> bool {
        crate::transducer::element::same_versor(&self.motion, &crate::transducer::element::identity_versor())
    }

    fn put_canonical(&self, out: &mut Vec<u8>) {
        self.motion.put_canonical(out);
    }

    fn take_canonical(b: &[u8], p: &mut usize) -> Option<Self> {
        Some(Moved { motion: Versor::from_bytes(b, p)? })
    }
}
