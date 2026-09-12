// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A21 structured keys: canonical coordinates written out, decodable,
//! prefix = stratum. No digest of any mathematical structure.
//!
//! An [`Address`] is an M2 address, a listing key, or a class key. It is
//! not a chain's bytes, not a socket string, and not a verifying key.

use std::ops::Deref;

pub const TAG_ROTOR: u8 = 1;
pub const TAG_VERSOR: u8 = 2;
pub const TAG_WORD: u8 = 3;
pub const TAG_FORM: u8 = 4;
pub const TAG_CHART: u8 = 5;
pub const TAG_CLASS: u8 = 6;
pub const TAG_CERT: u8 = 7;
pub const TAG_LISTING: u8 = 8;
pub const TAG_SEL: u8 = 9;
pub const TAG_HUE: u8 = 10;
pub const TAG_ASSIGN: u8 = 11;
pub const TAG_GAUGE: u8 = 12;
pub const TAG_PLANE: u8 = 13;
pub const TAG_CELL: u8 = 14;
pub const TAG_COCYCLE: u8 = 15;
pub const TAG_TASK: u8 = 16;
pub const TAG_DEMAND: u8 = 17;
/// One demand vertex OF one listing (L8q): what a probe names when it
/// answers a sentence rather than deflating it. 50.5 files one vertex per
/// (kind, chart), so the listing's address alone cannot say which vertex
/// a probe closes.
pub const TAG_DEMAND_AT: u8 = 18;

/// Canonical A21 coordinates of a form, listing, or class. Coarse-first:
/// a proper prefix is a coarser stratum of the same kind, never a sibling.
/// Equality is Address-to-Address only: not a chain's bytes, not a socket
/// string, not a verifying key.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(Vec<u8>);

impl Address {
    pub fn new(bytes: Vec<u8>) -> Address {
        Address(bytes)
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Address {
        Address(bytes.as_ref().to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn tag(&self) -> Option<u8> {
        self.0.first().copied()
    }

    pub fn starts_with(&self, prefix: &[u8]) -> bool {
        self.0.starts_with(prefix)
    }
}

impl Deref for Address {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::borrow::Borrow<[u8]> for Address {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Address {
    fn from(v: Vec<u8>) -> Address {
        Address(v)
    }
}

impl From<&[u8]> for Address {
    fn from(v: &[u8]) -> Address {
        Address(v.to_vec())
    }
}

impl From<Address> for Vec<u8> {
    fn from(a: Address) -> Vec<u8> {
        a.0
    }
}

pub fn put_bytes(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(&(b.len() as u32).to_le_bytes());
    out.extend_from_slice(b);
}

pub fn take_bytes(b: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().checked_sub(*p)? {
        return None;
    }
    let s = b.get(*p..*p + n)?.to_vec();
    *p += n;
    Some(s)
}

/// Wire and tape embedding of an address: `len16 + bytes`. Refuses when
/// the address does not fit in u16 (never truncates). A field of a key
/// uses [`put_bytes`] (u32).
pub fn put_addr(out: &mut Vec<u8>, addr: impl AsRef<[u8]>) -> Option<()> {
    let addr = addr.as_ref();
    let n = u16::try_from(addr.len()).ok()?;
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(addr);
    Some(())
}

pub fn take_addr(b: &[u8], p: &mut usize) -> Option<Address> {
    let n = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?) as usize;
    *p += 2;
    if n > b.len().checked_sub(*p)? {
        return None;
    }
    let s = b.get(*p..*p + n)?.to_vec();
    *p += n;
    Some(Address(s))
}

/// A19 class record: class key ‖ order (0 = infinite, else ℓ).
pub fn put_class_record(out: &mut Vec<u8>, class_key: impl AsRef<[u8]>, order: u32) -> Option<()> {
    put_addr(out, class_key)?;
    out.extend_from_slice(&order.to_le_bytes());
    Some(())
}

pub fn take_class_record(b: &[u8], p: &mut usize) -> Option<(Address, u32)> {
    let key = take_addr(b, p)?;
    let order = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some((key, order))
}

/// No key of one kind is a proper prefix of another of the same kind,
/// except by stratum (shared coarse coordinates). Length-delimited fields
/// make a shorter sibling incomparable as a prefix.
pub fn is_proper_prefix(a: &[u8], b: &[u8]) -> bool {
    a.len() < b.len() && b.starts_with(a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::alphabet::{LetterId, Rotor, Word};
    use crate::transducer::cga::Mv;
    use crate::transducer::element::{from_canonical, listing_key, IDENTITY_FRAME};
    use crate::transducer::germinate::germinate;
    use crate::transducer::stalk::Invariant;

    fn sample_words() -> [Word; 3] {
        let q = LetterId::quarter((0, 1)).expect("quarter");
        let s = LetterId::try_new(0).expect("sheet");
        [
            Word::from_letters(&[(q, false)]),
            Word::from_letters(&[(q, false), (q, false)]),
            Word::from_letters(&[(s, false), (q, false)]),
        ]
    }

    #[test]
    fn every_key_decodes_to_its_object() {
        for w in sample_words() {
            let r = w.rotor();
            let k = r.key();
            let back = Rotor::from_key(&k).expect("rotor key decodes");
            assert!(r.projective_eq(&back), "decode(&key) is the rotor");
            let ck = r.class_key().expect("a lettered rotor has a class");
            assert_eq!(ck.tag(), Some(TAG_CLASS));
        }
        let m = Mv::zero();
        assert!(m.key().is_none(), "the zero versor has no key");
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let t = crate::transducer::cga::translator(&one, &zero);
        let k = t.key().expect("a translator has a key");
        let back = Mv::from_key(&k).expect("mv key decodes");
        assert!(t.projective_eq(&back));
        let e = from_canonical(&germinate(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]).unwrap());
        let addr = e.address();
        assert_eq!(addr.tag(), Some(TAG_FORM));
        assert_eq!(e.address(), addr, "address is the form's coordinates");
    }

    #[test]
    fn no_key_is_a_proper_prefix_of_a_sibling() {
        let mut keys: Vec<Address> = sample_words().into_iter().map(|w| w.rotor().key()).collect();
        for w in sample_words() {
            keys.push(w.rotor().class_key().expect("class"));
        }
        let a = from_canonical(&germinate(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]).unwrap());
        let b = from_canonical(&germinate(&[(0, 0, 1), (1, 0, 1)], &[(0, 0, 3), (1, 0, 3)]).unwrap());
        keys.push(a.address());
        keys.push(b.address());
        keys.push(listing_key(a.sub(), &IDENTITY_FRAME).expect("a held word's chain is A5's"));
        keys.push(Invariant(vec![1, 2]).key());
        keys.push(Invariant(vec![1, 3]).key());
        keys.push(Invariant(vec![2]).key());
        for (i, x) in keys.iter().enumerate() {
            for (j, y) in keys.iter().enumerate() {
                if i == j {
                    continue;
                }
                if x.tag() != y.tag() {
                    continue;
                }
                assert!(
                    !is_proper_prefix(x.as_bytes(), y.as_bytes()),
                    "sibling keys of tag {:?} must not prefix: {x:?} vs {y:?}",
                    x.tag()
                );
            }
        }
    }

    #[test]
    fn class_record_travels_as_coordinates() {
        let w = sample_words()[0].clone();
        let key = w.rotor().class_key().expect("class");
        let mut buf = Vec::new();
        put_class_record(&mut buf, &key, 4).expect("A19 fits");
        let mut p = 0usize;
        let (back, order) = take_class_record(&buf, &mut p).expect("A19 record");
        assert_eq!(back, key);
        assert_eq!(order, 4);
        assert_eq!(p, buf.len());
        assert_eq!(key.as_bytes()[0], TAG_CLASS);
    }
}
