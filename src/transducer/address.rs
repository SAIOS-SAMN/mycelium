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
/// A simplex of a Čech complex, keyed by its vertices; the court's simplex
/// key on a chart and a remainder's site key open with the same byte.
pub const TAG_SIMPLEX: u8 = 14;
pub const TAG_COCYCLE: u8 = 15;
pub const TAG_TASK: u8 = 16;
pub const TAG_DEMAND: u8 = 17;
/// One demand vertex OF one listing (L8q): what a probe names when it
/// answers a sentence rather than deflating it. 50.5 files one vertex per
/// (kind, chart), so the listing's address alone cannot say which vertex
/// a probe closes.
pub const TAG_DEMAND_AT: u8 = 18;

/// AN ELEMENT THAT IS NOT A CLOSED FORM: a residual 1-chain, a letter, a
/// rewrite — everything `form_key_bytes` addresses.
///
/// It shared `TAG_FORM` with the closed forms, and the two are different
/// algebraic objects: a form is a graded cell complex whose faces and
/// loops satisfy `δ¹g = 1`, written in the vertex-key layout; a residual
/// is an unclosed chain, `∂c ≠ 0`, written as its own bytes. One tag over
/// both meant `from_address` could not tell which layout it held, so a
/// master equation needed a second decoder beside it — and
/// `Element::address` FELL THROUGH from the first to the second whenever
/// a sentence's tape did not decode, which is how a soak's genome reported
/// a 0.9 KB address against a 46 KB tape and looked like a 50x saving.
///
/// Two kinds, two tags. `from_address` reads `TAG_FORM` and refuses this
/// one rather than misreading it.
pub const TAG_RESIDUAL: u8 = 19;

/// §4.4 E8 — THE THREE ADDRESSES OF A COVER, DECLARED (ruled 2026-09-22).
///
/// `pieces.rs` wrote these three by hand and the carriers holding them were
/// `Keyed<V, coord::Any>`: loose bytes that could not tell a piece's address
/// from a carriage's, and a carriage's from a meet's. They are genuine
/// coordinates — a piece IS its shell kind, its hue and its support; a
/// carriage IS its domain and codomain in that order, because a carriage has
/// a direction; a meet IS its two carriages unordered, because a seam does
/// not — so each gets a tag and an invertible reading, and the carriers
/// answer the coordinate instead of bytes.
pub const TAG_PIECE: u8 = 20;
pub const TAG_CARRIAGE: u8 = 21;
pub const TAG_MEET: u8 = 22;

/// Canonical A21 coordinates of a form, listing, or class. Coarse-first:
/// a proper prefix is a coarser stratum of the same kind, never a sibling.
/// Equality is Address-to-Address only: not a chain's bytes, not a socket
/// string, not a verifying key.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
/// Z4c — AN ADDRESS OWNS ITS BYTES ONCE AND SHARES THEM.
///
/// The address IS the object's coordinates written out (law 2, A21), and
/// for a form that is megabytes: `form_vertex_key` writes every vertex,
/// member and gauge, 7.3 MB on the 590-vertex rule. Every carrier that
/// names the form by it held a separate copy — the bubbles trie, `dirty`,
/// `ordinals`, the Čech complex's sections — and a hold paid for all of
/// them.
///
/// `Arc<[u8]>` changes WHO OWNS THE BYTES and nothing else. The coordinates
/// are the same coordinates, `as_bytes` is the same slice, and a reader
/// that compares, peels or descends on it cannot tell the difference —
/// which is the point: the compression is by CITATION, one object written
/// once and named everywhere by reference, not by shortening a coordinate,
/// which would make it a stratum and not the object.
pub struct Address(std::sync::Arc<[u8]>);

impl Address {
    pub fn new(bytes: Vec<u8>) -> Address {
        Address(bytes.into())
    }

    /// The one copy, to hand a carrier that files by this address
    /// (`Keyed::insert_shared`).
    pub fn shared(&self) -> std::sync::Arc<[u8]> {
        std::sync::Arc::clone(&self.0)
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Address {
        Address(std::sync::Arc::from(bytes.as_ref()))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn tag(&self) -> Option<u8> {
        tag_of(&self.0)
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

/// AN ADDRESS THAT FITS ITS CITATION: one whose length writes into the
/// u16 prefix every tape cites an address through. [`Address::cited`] is
/// its only constructor, so a value of this type IS the proof that
/// [`put_cited`] is total — and a key built from cited addresses needs no
/// refusal of its own.
///
/// The refusal it replaces was a `?` inside `Element::address`, which had
/// no answer but the form tag; 47.1 takes it at the constructor instead,
/// where an element that cannot be cited is simply not built.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cited(Address);

impl Cited {
    pub fn addr(&self) -> &Address {
        &self.0
    }

    pub fn into_addr(self) -> Address {
        self.0
    }
}

impl Deref for Cited {
    type Target = Address;
    fn deref(&self) -> &Address {
        &self.0
    }
}

impl AsRef<[u8]> for Cited {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Address {
    /// This address as a citation, or `None` where it does not fit the
    /// prefix: an address too long to cite is one no tape can name, and
    /// the element that would cite it is refused where it is built.
    pub fn cited(&self) -> Option<Cited> {
        u16::try_from(self.0.len()).ok()?;
        Some(Cited(self.clone()))
    }
}

/// Write a citation. TOTAL: the type is the proof that the length fits.
pub fn put_cited(out: &mut Vec<u8>, c: &Cited) {
    let b = c.addr().as_bytes();
    out.extend_from_slice(&(b.len() as u16).to_le_bytes());
    out.extend_from_slice(b);
}

/// Read a citation. What comes back went through the same u16 prefix, so
/// it writes back whole.
pub fn take_cited(b: &[u8], p: &mut usize) -> Option<Cited> {
    take_addr(b, p)?.cited()
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
        Address(v.into())
    }
}

impl From<&[u8]> for Address {
    fn from(v: &[u8]) -> Address {
        Address(std::sync::Arc::from(v))
    }
}

impl From<Address> for Vec<u8> {
    fn from(a: Address) -> Vec<u8> {
        a.0.to_vec()
    }
}

/// A mathematical coordinate with a canonical, self-delimiting boundary (Rule 6).
pub trait Stratum: Eq {
    fn write_canonical(&self, out: &mut Vec<u8>);
    fn read_canonical(bytes: &[u8]) -> Option<(Self, &[u8])>
    where
        Self: Sized;
}

/// The end of a coordinate path.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

impl Stratum for Nil {
    fn write_canonical(&self, _out: &mut Vec<u8>) {}
    fn read_canonical(bytes: &[u8]) -> Option<(Self, &[u8])> {
        Some((Nil, bytes))
    }
}

/// A stratified coordinate path down a filtration (Rule 6 / A21).
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoordinatePath<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}

impl<Head: Stratum, Tail: Stratum> CoordinatePath<Head, Tail> {
    pub fn new(head: Head, tail: Tail) -> Self {
        CoordinatePath { head, tail }
    }

    pub fn to_address(&self) -> Address {
        let mut out = Vec::new();
        self.write_canonical(&mut out);
        Address::new(out)
    }
}

impl<Head: Stratum, Tail: Stratum> Stratum for CoordinatePath<Head, Tail> {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        self.head.write_canonical(out);
        self.tail.write_canonical(out);
    }

    fn read_canonical(bytes: &[u8]) -> Option<(Self, &[u8])> {
        let (head, rest) = Head::read_canonical(bytes)?;
        let (tail, rest2) = Tail::read_canonical(rest)?;
        Some((CoordinatePath { head, tail }, rest2))
    }
}

/// Proof that `Self` is a valid coordinate prefix of target schema `Target`.
pub trait PrefixOf<Target> {
    fn write_prefix(&self, out: &mut Vec<u8>);
}

impl<Target> PrefixOf<Target> for Nil {
    fn write_prefix(&self, _out: &mut Vec<u8>) {}
}

impl<Head: Stratum, Tail: PrefixOf<TailTarget>, TailTarget>
    PrefixOf<CoordinatePath<Head, TailTarget>> for CoordinatePath<Head, Tail>
{
    fn write_prefix(&self, out: &mut Vec<u8>) {
        self.head.write_canonical(out);
        self.tail.write_prefix(out);
    }
}

impl Stratum for Address {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        let b = self.as_bytes();
        out.extend_from_slice(&(b.len() as u32).to_le_bytes());
        out.extend_from_slice(b);
    }

    fn read_canonical(bytes: &[u8]) -> Option<(Self, &[u8])> {
        let len_bytes = bytes.get(0..4)?;
        let len = u32::from_le_bytes(len_bytes.try_into().ok()?) as usize;
        let b = bytes.get(4..4 + len)?;
        let rest = bytes.get(4 + len..)?;
        Some((Address::new(b.to_vec()), rest))
    }
}

/// THE TAG a key or a tape opens with. A21 puts the stratum tag at offset
/// zero, so byte zero is the tape's own coordinate and not the byte that
/// happened to arrive first (rule 15). One carrier for that offset:
/// [`Address::tag`] reads through it, and so does every reader that holds
/// the bytes before they are an [`Address`].
pub fn tag_of(b: &[u8]) -> Option<u8> {
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — wire tag offset: A21 writes the stratum tag at offset zero, and this is the one reader of it"
    )]
    b.first().copied()
}

/// A COUNT IN ONE BYTE, or a refusal. `len().min(255) as u8` wrote 255
/// for any larger count and the writer then emitted 255 members, so the
/// reader took a frame that ended in the middle of the next field and
/// refused the whole act while the sender believed it spoke (7.1). 47.1's
/// rule: a length that does not fit its wire field is refused where the
/// frame is built.
pub fn put_u8_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.push(u8::try_from(n).ok()?);
    Some(())
}

/// A byte string behind a one-byte length, or a refusal. The truncating
/// form silently shortened an address or a bind string and signed the
/// short one.
pub fn put_u8_bytes(out: &mut Vec<u8>, b: &[u8]) -> Option<()> {
    put_u8_count(out, b.len())?;
    out.extend_from_slice(b);
    Some(())
}

/// A COUNT BEHIND TWO BYTES, for a carrier whose width is the data's and
/// not a fixed size: a chart key is as wide as the body's own moduli (A21),
/// and one byte caps it at 255. Measured under `ChartAxis`: a `Count` is 11
/// bytes and a single-limb `Ratio` 28, so a body's chart — four counts,
/// five conformal ratios and one ratio per hue — passes 255 at four hues.
/// Refuses rather than truncates, as the one-byte form does.
pub fn put_u16_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u16::try_from(n).ok()?.to_le_bytes());
    Some(())
}

/// The same count read back. `None` where the two bytes are not there.
pub fn take_u16_count(b: &[u8], p: &mut usize) -> Option<usize> {
    let n = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?) as usize;
    *p += 2;
    Some(n)
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
    let s = std::sync::Arc::from(b.get(*p..*p + n)?);
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

    /// A21 §4 at the kinds that HAVE a decoder: the rotor and the versor.
    ///
    /// THIS TEST WAS `every_key_decodes_to_its_object` AND IT CHECKED NO SUCH
    /// THING FOR A FORM. Its last two lines were
    ///
    /// ```text
    /// let addr = e.address();
    /// assert_eq!(addr.tag(), Some(TAG_FORM));
    /// assert_eq!(e.address(), addr, "address is the form's coordinates");
    /// ```
    ///
    /// — a tag check and `addr == addr`. So A21 §4's decode property had never
    /// been checked at the one kind the engine is made of, under a name that
    /// said it had.
    ///
    /// `Rotor::from_key`, `Mv::from_key` and `ClassVector::from_key` exist.
    /// THERE IS NO DECODER FOR `TAG_FORM`, so the property is not checkable
    /// here and is not claimed: what a form's address DOES hold is checked in
    /// `a_form_address_is_a_function_of_its_content` below, and the tape is the
    /// only thing that reconstructs a form.
    #[test]
    fn the_rotor_and_versor_keys_decode_to_their_objects() {
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
        let t = crate::transducer::cga::translator(&one, &zero, &zero);
        let k = t.key().expect("a translator has a key");
        let back = Mv::from_key(&k).expect("mv key decodes");
        assert!(t.projective_eq(&back));
    }

    /// WHAT A FORM'S ADDRESS ACTUALLY HOLDS, asserted on the object the engine
    /// is made of and not on a tautology: it is a FUNCTION of the form's
    /// coordinates — the same coordinates give the same address — and it
    /// SEPARATES forms that differ in one of them.
    ///
    /// That is the half of A21 an undecodable key keeps. Chapter 1.7's argument
    /// against digests is that they keep equality and destroy every other
    /// operation; an address that cannot be decoded keeps equality AND keeps
    /// the prefix stratum (`no_key_is_a_proper_prefix_of_a_sibling`) and loses
    /// the object. Better than a digest, and not what A21 §4 specifies.
    #[test]
    fn a_form_address_is_a_function_of_its_content() {
        use crate::transducer::alphabet::Derivation;
        use crate::transducer::sentence::{form_vertex_key, Form, Vertex};
        use crate::transducer::stalk::Invariant;
        let build = |gauge: Vec<(u8, u8)>, chart: Invariant| -> Form {
            let mut f = Form::default();
            f.vertices.push(Vertex::word_on(
                Derivation::spelled(sample_words()[0].clone()),
                chart,
                Box::new(crate::transducer::element::identity_versor()),
                gauge,
                None,
            ));
            f.faces.push(f.face_of(0, 1).expect("the star face"));
            f.reindex();
            f
        };
        let a = build(vec![(1, 2)], Invariant::counts(&[3, 5]));
        let again = build(vec![(1, 2)], Invariant::counts(&[3, 5]));
        assert_eq!(
            form_vertex_key(&a),
            form_vertex_key(&again),
            "one form's coordinates give one address, on every node (law 2)"
        );
        assert_eq!(form_vertex_key(&a).tag(), Some(TAG_FORM));
        // One coordinate moved at a time, each of which 17.2 says the address
        // writes: the member's gauge, then the member's chart.
        let other_gauge = build(vec![(1, 3)], Invariant::counts(&[3, 5]));
        assert_ne!(
            form_vertex_key(&a),
            form_vertex_key(&other_gauge),
            "two forms whose vertex recolours differently are two addresses"
        );
        let other_chart = build(vec![(1, 2)], Invariant::counts(&[7]));
        assert_ne!(
            form_vertex_key(&a),
            form_vertex_key(&other_chart),
            "two forms whose vertex acts on a different chart are two addresses"
        );
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
        keys.push(Invariant::counts(&[1, 2]).key());
        keys.push(Invariant::counts(&[1, 3]).key());
        keys.push(Invariant::counts(&[2]).key());
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

    #[test]
    fn coordinate_path_preserves_strata_and_exact_boundaries() {
        let a = Address::from_bytes([TAG_ROTOR, 1, 2, 3]);
        let b = Address::from_bytes([TAG_PLANE, 4, 5]);
        let path = CoordinatePath::new(a.clone(), CoordinatePath::new(b.clone(), Nil));
        let mut bytes = Vec::new();
        path.write_canonical(&mut bytes);
        let (back, rest) = CoordinatePath::<Address, CoordinatePath<Address, Nil>>::read_canonical(&bytes)
            .expect("canonical read");
        assert_eq!(back.head, a);
        assert_eq!(back.tail.head, b);
        assert!(rest.is_empty());

        let prefix = CoordinatePath::new(a.clone(), Nil);
        let mut pbytes = Vec::new();
        PrefixOf::<CoordinatePath<Address, CoordinatePath<Address, Nil>>>::write_prefix(&prefix, &mut pbytes);
        assert!(bytes.starts_with(&pbytes));
    }

    #[test]
    fn canonical_coordinate_writers_unnest_keys_under_rule_6() {
        let w = sample_words()[0].clone();
        let mut rotor_coords = Vec::new();
        w.rotor().put_canonical(&mut rotor_coords);
        let key = w.rotor().key();
        assert_eq!(key.as_bytes()[0], TAG_ROTOR);
        assert_eq!(&key.as_bytes()[1..], &rotor_coords[..]);

        let mut class_coords = Vec::new();
        w.rotor().put_class_coordinates(&mut class_coords).expect("class coords");
        let ckey = w.rotor().class_key().expect("class key");
        assert_eq!(ckey.as_bytes()[0], TAG_CLASS);
        assert_eq!(&ckey.as_bytes()[1..], &class_coords[..]);

        let mut word_coords = Vec::new();
        w.put_canonical(&mut word_coords);
        assert_eq!(word_coords, rotor_coords);
    }
}
