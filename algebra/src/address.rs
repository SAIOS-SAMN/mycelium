// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A21 — THE ADDRESS CARRIER: an object's canonical coordinates written out,
//! owned once and shared, and the citation an address fits. A value's key
//! is its canonical writing under [`TAG_VERSOR`].

use std::ops::Deref;

/// FORMAT: versor address tag byte.
pub const TAG_VERSOR: u8 = 2;

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
/// names the form by it held a separate copy — the wells trie, `dirty`,
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
/// the library's `put_cited` is total — and a key built from cited
/// addresses needs no refusal of its own.
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
