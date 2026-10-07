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

    fn compose(&self, then: &Self) -> Option<Self> {
        Some(Turned { rotor: then.rotor.geo(&self.rotor), carriage: then.carriage.geo(&self.carriage) })
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
}
