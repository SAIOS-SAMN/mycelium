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
/// orbits satisfy `δ¹g = 1`, written in the vertex-key layout; a residual
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
/// 78.62 — A LAW OF THE UNIVERSAL REGISTER: `TAG_LAW ‖ index ‖ version`,
/// the coordinate an uplinking kernel fetches it by and a verdict cites.
pub const TAG_LAW: u8 = 23;

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
    pub fn address(&self) -> &Address {
        &self.0
    }

    pub fn into_address(self) -> Address {
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

/// A KEY COMPONENT: a coordinate written into a key, never a record a reader
/// frames at a fixed width (STD1 3c-2, ruled (d)).
///
/// ITS LENGTH IS WRITTEN BY ITS OWN COORDINATE: LEB128, seven bits a byte,
/// the high bit saying more follows. Exact for any length, so the writer is
/// total with no cast and no width wall — an address held once is
/// legitimately megabytes, and the only wall against a blob is the
/// citation's u16 (LD0). Minimal and prefix-free, so the key is injective by
/// construction; and a component under 128 bytes takes one length byte, not
/// four.
pub fn put_key_bytes(out: &mut Vec<u8>, b: &[u8]) {
    put_key_count(out, b.len());
    out.extend_from_slice(b);
}

/// A COUNT IN A KEY, written by its own coordinate (STD1 3c-2, ruled (d)):
/// LEB128, seven bits a byte, minimal. A count in a key is a coordinate of
/// the key exactly as a component's length is, so it has no width wall.
pub fn put_key_count(out: &mut Vec<u8>, mut n: usize) {
    loop {
        let low = n.to_le_bytes()[0] & 0x7f;
        n >>= 7;
        if n == 0 {
            out.push(low);
            break;
        }
        out.push(low | 0x80);
    }
}

/// A key count read back. `None` where the spelling is not minimal (a key
/// has one spelling) or the count does not fit a `usize`.
pub fn take_key_count(b: &[u8], p: &mut usize) -> Option<usize> {
    let mut n = 0usize;
    let mut shift = 0u32;
    loop {
        let byte = *b.get(*p)?;
        *p += 1;
        let v = usize::from(byte & 0x7f);
        let placed = v.checked_shl(shift)?;
        // Bits shifted past the carrier are a count no usize holds.
        if placed >> shift != v {
            return None;
        }
        n |= placed;
        if byte & 0x80 == 0 {
            // A final zero byte after the first is an over-long spelling.
            if byte == 0 && shift > 0 {
                return None;
            }
            return Some(n);
        }
        shift += 7;
    }
}

/// A key component read back: its LEB128 length, then that many bytes.
/// `None` where the length is not minimal, does not fit a `usize`, or runs
/// past the key.
pub fn take_key_bytes(b: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let n = take_key_count(b, p)?;
    if n > b.len().checked_sub(*p)? {
        return None;
    }
    let out = b.get(*p..*p + n)?.to_vec();
    *p += n;
    Some(out)
}

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
    let s = std::sync::Arc::from(b.get(*p..*p + n)?);
    *p += n;
    Some(Address(s))
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
pub fn is_proper_prefix(a: &[u8], b: &[u8]) -> bool {
    a.len() < b.len() && b.starts_with(a)
}
