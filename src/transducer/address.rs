// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A21 structured keys: canonical coordinates written out, decodable,
//! prefix = stratum. No digest of any mathematical structure.
//!
//! An [`Address`] is an M2 address, a listing key, or a class key. It is
//! not a chain's bytes, not a socket string, and not a verifying key.


pub const TAG_ROTOR: u8 = 1;
/// THE ADDRESS CARRIER, the citation it fits, the tag reader and the versor
/// tag stand with the exact algebra, whose values are keyed by them (A21).
pub use algebra::address::{tag_of, Address, Cited, TAG_VERSOR};
/// FORMAT: word address tag byte.
pub const TAG_WORD: u8 = 3;
/// FORMAT: form address tag byte.
pub const TAG_FORM: u8 = 4;
/// FORMAT: chart address tag byte.
pub const TAG_CHART: u8 = 5;
/// FORMAT: class address tag byte.
pub const TAG_CLASS: u8 = 6;
/// FORMAT: certificate address tag byte.
pub const TAG_CERT: u8 = 7;
/// FORMAT: listing address tag byte.
pub const TAG_LISTING: u8 = 8;
/// FORMAT: selection address tag byte.
pub const TAG_SEL: u8 = 9;
/// FORMAT: hue address tag byte.
pub const TAG_HUE: u8 = 10;
/// FORMAT: assignment address tag byte.
pub const TAG_ASSIGN: u8 = 11;
/// FORMAT: gauge address tag byte.
pub const TAG_GAUGE: u8 = 12;
/// FORMAT: plane address tag byte.
pub const TAG_PLANE: u8 = 13;
/// A simplex of a Čech complex, keyed by its vertices; the court's simplex
/// key on a chart and a remainder's site key open with the same byte.
/// FORMAT: simplex address tag byte.
pub const TAG_SIMPLEX: u8 = 14;
/// FORMAT: cocycle address tag byte.
pub const TAG_COCYCLE: u8 = 15;
/// FORMAT: task address tag byte.
pub const TAG_TASK: u8 = 16;
/// FORMAT: demand address tag byte.
pub const TAG_DEMAND: u8 = 17;
/// One demand vertex OF one listing (L8q): what a probe names when it
/// answers a sentence rather than deflating it. 50.5 files one vertex per
/// (kind, chart), so the listing's address alone cannot say which vertex
/// a probe closes.
/// FORMAT: demand-vertex address tag byte.
pub const TAG_DEMAND_AT: u8 = 18;

/// AN ELEMENT THAT IS NOT A CLOSED FORM: a residue 1-chain, a letter, a
/// rewrite — everything `form_key_bytes` addresses.
///
/// It shared `TAG_FORM` with the closed forms, and the two are different
/// algebraic objects: a form is a graded cell complex whose faces and
/// orbits satisfy `δ¹g = 1`, written in the vertex-key layout; a residue
/// is an unclosed chain, `∂c ≠ 0`, written as its own bytes. One tag over
/// both meant `from_address` could not tell which layout it held, so a
/// master equation needed a second decoder beside it — and
/// `Element::address` FELL THROUGH from the first to the second whenever
/// a sentence's tape did not decode, which is how a soak's genome reported
/// a 0.9 KB address against a 46 KB tape and looked like a 50x saving.
///
/// Two kinds, two tags. `from_address` reads `TAG_FORM` and refuses this
/// one rather than misreading it.
/// FORMAT: residue-chain address tag byte.
pub const TAG_RESIDUE: u8 = 19;

/// §4.4 E8 — THE THREE ADDRESSES OF A COVER, DECLARED (ruled 2026-09-22).
///
/// `pieces.rs` wrote these three by hand and the carriers holding them were
/// `Keyed<V, coord::Any>`: loose bytes that could not tell a piece's address
/// from a carriage's, and a carriage's from a meet's. They are genuine
/// coordinates — a piece IS its layer kind, its hue and its support; a
/// carriage IS its domain and codomain in that order, because a carriage has
/// a direction; a meet IS its two carriages unordered, because a seam does
/// not — so each gets a tag and an invertible reading, and the carriers
/// answer the coordinate instead of bytes.
/// FORMAT: piece address tag byte.
pub const TAG_PIECE: u8 = 20;
/// FORMAT: carriage address tag byte.
pub const TAG_CARRIAGE: u8 = 21;
/// FORMAT: meet address tag byte.
pub const TAG_MEET: u8 = 22;
/// 78.62 — A LAW OF THE UNIVERSAL REGISTER: `TAG_LAW ‖ index ‖ version`,
/// the coordinate an uplinking kernel fetches it by and a verdict cites.
/// FORMAT: law address tag byte.
pub const TAG_LAW: u8 = 23;

/// Write a citation. TOTAL: the type is the proof that the length fits.
pub fn put_cited(out: &mut Vec<u8>, c: &Cited) {
    let b = c.address().as_bytes();
    #[expect(clippy::cast_possible_truncation, reason = "the u16 prefix's bound is the Cited type: its only constructor, Address::cited, refuses by u16::try_from any address longer than u16::MAX")]
    out.extend_from_slice(&(b.len() as u16).to_le_bytes());
    out.extend_from_slice(b);
}

/// Read a citation. What comes back went through the same u16 prefix, so
/// it writes back whole.
pub fn take_cited(b: &[u8], p: &mut usize) -> Option<Cited> {
    take_address(b, p)?.cited()
}

/// A mathematical coordinate with a canonical, self-delimiting boundary (Rule 6).
pub trait CanonicalCoordinate: Eq {
    fn write_canonical(&self, out: &mut Vec<u8>);
    fn read_canonical(bytes: &[u8]) -> Option<(Self, &[u8])>
    where
        Self: Sized;
}

/// The end of a coordinate path.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

impl CanonicalCoordinate for Nil {
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

impl<Head: CanonicalCoordinate, Tail: CanonicalCoordinate> CoordinatePath<Head, Tail> {
    pub fn new(head: Head, tail: Tail) -> Self {
        CoordinatePath { head, tail }
    }
}

impl<Head: CanonicalCoordinate, Tail: CanonicalCoordinate> CanonicalCoordinate for CoordinatePath<Head, Tail> {
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

impl<Head: CanonicalCoordinate, Tail: PrefixOf<TailTarget>, TailTarget>
    PrefixOf<CoordinatePath<Head, TailTarget>> for CoordinatePath<Head, Tail>
{
    fn write_prefix(&self, out: &mut Vec<u8>) {
        self.head.write_canonical(out);
        self.tail.write_prefix(out);
    }
}

impl CanonicalCoordinate for Address {
    /// A key component (ruled (d)): its length by its own coordinate.
    fn write_canonical(&self, out: &mut Vec<u8>) {
        put_key_bytes(out, self.as_bytes());
    }

    fn read_canonical(bytes: &[u8]) -> Option<(Self, &[u8])> {
        let mut p = 0usize;
        let b = take_key_bytes(bytes, &mut p)?;
        Some((Address::new(b), bytes.get(p..)?))
    }
}

/// A COUNT IN ONE BYTE, or a refusal. `len().min(255) as u8` wrote 255
/// for any larger count and the writer then emitted 255 members, so the
/// reader took a frame that ended in the middle of the next field and
/// refused the whole act while the sender believed it spoke (7.1). 47.1's
/// rule: a length that does not fit its wire field is refused where the
/// frame is built.
#[must_use]
pub fn put_u8_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.push(u8::try_from(n).ok()?);
    Some(())
}

/// A byte string behind a one-byte length, or a refusal. The truncating
/// form silently shortened an address or a bind string and signed the
/// short one.
#[must_use]
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
#[must_use]
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

/// A COUNT BEHIND FOUR BYTES, or a refusal (STD1 3c, LD0): a count past the
/// field is a record refused by name, never a shorter one.
#[must_use]
pub fn put_u32_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u32::try_from(n).ok()?.to_le_bytes());
    Some(())
}

/// A KEY COMPONENT AND ITS LENGTH, each by its own coordinate (STD1 3c-2,
/// ruled (d)): the exact algebra's writing, which a value's coefficients
/// take on the wire.
pub use algebra::component::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};

/// A byte string behind a four-byte length, or a refusal: the `as u32` this
/// wrote truncated the length of a blob past 4 GiB and left its bytes whole
/// behind it, a record no reader could frame.
#[must_use]
pub fn put_bytes(out: &mut Vec<u8>, b: &[u8]) -> Option<()> {
    put_u32_count(out, b.len())?;
    out.extend_from_slice(b);
    Some(())
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

/// Wire and tape embedding of an address: `len16 + bytes`. The u16 is the
/// wall (LD0 as re-ruled, 78.22): an address that reaches it is a blob
/// written where a coordinate or a citation belongs, and it refuses,
/// writing nothing; every caller refuses its whole write by name. A field
/// of a key uses [`put_key_bytes`] (ruled (d)).
#[must_use]
pub fn put_address(out: &mut Vec<u8>, address: impl AsRef<[u8]>) -> Option<()> {
    let address = address.as_ref();
    let n = u16::try_from(address.len()).ok()?;
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(address);
    Some(())
}

pub fn take_address(b: &[u8], p: &mut usize) -> Option<Address> {
    let n = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?) as usize;
    *p += 2;
    if n > b.len().checked_sub(*p)? {
        return None;
    }
    let s = Address::from(b.get(*p..*p + n)?);
    *p += n;
    Some(s)
}

/// A19 class record: class key ‖ order (0 = infinite, else ℓ).
pub fn put_class_record(out: &mut Vec<u8>, class_key: impl AsRef<[u8]>, order: u32) -> Option<()> {
    put_address(out, class_key)?;
    out.extend_from_slice(&order.to_le_bytes());
    Some(())
}

pub fn take_class_record(b: &[u8], p: &mut usize) -> Option<(Address, u32)> {
    let key = take_address(b, p)?;
    let order = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some((key, order))
}

/// No key of one kind is a proper prefix of another of the same kind,
/// except by stratum (shared coarse coordinates). Length-delimited fields
/// make a shorter sibling incomparable as a prefix.
#[cfg(test)]
pub fn is_proper_prefix(a: &[u8], b: &[u8]) -> bool {
    a.len() < b.len() && b.starts_with(a)
}
