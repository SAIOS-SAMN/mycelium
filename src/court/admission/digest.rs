// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ANTI-FLATTENING 13, MU-HOSTING §5a.10 — THE THREE SENSES OF A DIGEST:
//! a digest locates, orders a chain, or binds signed bytes, and it never
//! admits.
//!
//! A digest has no algebra: it cannot be composed, inverted or transported,
//! and comparing two answers only same or not same. So the 32 bytes that
//! cross at admission are typed by their sense, and none of the three types
//! implements `PartialEq`. Each sense permits one comparison, and it is a
//! named method:
//!
//! | Type | Sense | The comparison |
//! |---|---|---|
//! | [`Locator`] | address: locates coordinates held losslessly elsewhere | [`Locator::locates`] |
//! | [`Predecessor`] | link: a record names the record before it on its chain | [`Predecessor::continues`], [`Predecessor::forks`] |
//! | [`Commitment`] | commitment: bytes a party signs | [`Commitment::binds`], over a [`Reconstructed`] object only |
//!
//! The bytes on the wire and the ledger are unchanged: each type is the
//! same 32 bytes, read in and written out by `of_bytes` and `bytes`.

use super::{Reconstructed, SignedPrefix};
use crate::chronometry::{KStepReceipt, Parent};

/// A DIGEST IN THE ADDRESS SENSE: a fetch key or an index entry that
/// locates an object held losslessly elsewhere. Equality of two locators
/// says where the object is, never that it is admitted. The type has no
/// `==` (`address_has_no_equality`):
///
/// ```compile_fail,E0369
/// use mycelium::court::admission::Locator;
/// let (a, b) = (Locator::of_bytes([1; 32]), Locator::of_bytes([1; 32]));
/// let _ = a == b;
/// ```
///
/// The named comparison, which compiles:
///
/// ```
/// use mycelium::court::admission::Locator;
/// assert!(Locator::of_bytes([1; 32]).locates(&Locator::of_bytes([1; 32])));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Locator([u8; 32]);

impl Locator {
    /// The locator a wire or a ledger carries, as its 32 bytes.
    pub fn of_bytes(digest: [u8; 32]) -> Locator {
        Locator(digest)
    }

    /// The 32 bytes, for writing out.
    pub fn bytes(&self) -> [u8; 32] {
        self.0
    }

    /// ADDRESS SENSE: this locator names the place `held` names, so the
    /// object found there is the one sought (a fetch, or a resend met by
    /// what is held already). Never a reason to admit the object.
    pub fn locates(&self, held: &Locator) -> bool {
        self.0 == held.0
    }
}

/// A DIGEST IN THE LINK SENSE: a record of a receipt chain names the
/// record before it by this digest (a citation's hash, a signed prefix's
/// predecessor), and a held record is named by its own chain hash. Equality
/// says the chain is unbroken across the two records; each record's
/// content is reconstructed apart (`Reconstructed::receipt`). The type has
/// no `==` (`predecessor_has_no_equality`):
///
/// ```compile_fail,E0369
/// use mycelium::court::admission::Predecessor;
/// let (a, b) = (Predecessor::of_bytes([1; 32]), Predecessor::of_bytes([1; 32]));
/// let _ = a == b;
/// ```
///
/// The named comparison, which compiles:
///
/// ```
/// use mycelium::court::admission::Predecessor;
/// assert!(Predecessor::of_bytes([1; 32]).continues(&Predecessor::of_bytes([1; 32])));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Predecessor([u8; 32]);

impl Predecessor {
    /// The digest a successor names this held record by: its chain hash.
    pub fn of(r: &KStepReceipt) -> Predecessor {
        Predecessor(r.chain_hash)
    }

    /// The predecessor a citation names.
    pub fn named(p: &Parent) -> Predecessor {
        Predecessor(p.hash)
    }

    /// The digest a wire or a ledger carries, as its 32 bytes.
    pub fn of_bytes(digest: [u8; 32]) -> Predecessor {
        Predecessor(digest)
    }

    /// The 32 bytes, for writing out.
    pub fn bytes(&self) -> [u8; 32] {
        self.0
    }

    /// LINK SENSE: a record that names `self` as its predecessor continues
    /// the chain from the record `held` names: the chain is unbroken
    /// across them.
    pub fn continues(&self, held: &Predecessor) -> bool {
        self.0 == held.0
    }

    /// 47.4: two records held at one place on one line FORK it where they
    /// carry two chain hashes. Read only between records at one place; at
    /// two places the question is `continues`.
    pub fn forks(&self, other: &Predecessor) -> bool {
        self.0 != other.0
    }

    /// LINK SENSE ON A CITATION: `cited` names the record `held` stands
    /// for, at one stamp and with a chain continuing across it.
    pub fn cites(cited: &Parent, held: &Parent) -> bool {
        cited.at == held.at && Predecessor::named(cited).continues(&Predecessor::named(held))
    }
}

/// AN OBJECT A SIGNER COMMITS TO THROUGH A DIGEST: the digest of it that a
/// commitment carries, read off the object once it is reconstructed.
pub trait Committed {
    /// The digest a commitment over this object carries.
    fn committed(&self) -> [u8; 32];
}

/// 47.1: a signed prefix commits its receipt to the receipt's
/// predecessor, by that predecessor's chain hash (the empty hash at
/// genesis). The prefix's place and height are coordinates, compared as
/// coordinates.
impl Committed for KStepReceipt {
    fn committed(&self) -> [u8; 32] {
        self.prev_hash()
    }
}

/// A DIGEST IN THE COMMITMENT SENSE: bytes a party signs (a signature's
/// message, the Concordance text, a clause). It decides that a
/// reconstructed object is the one committed to, after the object is
/// reconstructed and never instead: [`Commitment::binds`] takes only a
/// [`Reconstructed`] object. The type has no `==`
/// (`commitment_has_no_equality`):
///
/// ```compile_fail,E0369
/// use mycelium::court::admission::Commitment;
/// let (a, b) = (Commitment::of_bytes([1; 32]), Commitment::of_bytes([1; 32]));
/// let _ = a == b;
/// ```
///
/// A claimed object, not reconstructed, is not bound:
///
/// ```compile_fail,E0308
/// fn bound(c: &mycelium::court::admission::Commitment, claimed: &mycelium::chronometry::KStepReceipt) -> bool {
///     c.binds(claimed)
/// }
/// ```
///
/// The lawful path, which compiles:
///
/// ```
/// fn bound(
///     c: &mycelium::court::admission::Commitment,
///     claimed: &mycelium::chronometry::KStepReceipt,
/// ) -> Option<bool> {
///     mycelium::court::admission::Reconstructed::receipt(claimed).map(|r| c.binds(&r))
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Commitment([u8; 32]);

impl Commitment {
    /// The commitment a wire or a ledger carries, as its 32 bytes.
    pub fn of_bytes(digest: [u8; 32]) -> Commitment {
        Commitment(digest)
    }

    /// 47.1: what a signed prefix commits its receipt to, its
    /// predecessor's chain hash.
    pub fn signed(prefix: &SignedPrefix) -> Commitment {
        Commitment(prefix.prev_receipt_hash)
    }

    /// The 32 bytes, for writing out.
    pub fn bytes(&self) -> [u8; 32] {
        self.0
    }

    /// COMMITMENT SENSE: the object, reconstructed under the laws, is the
    /// one this commitment was signed over.
    pub fn binds<T: Committed>(&self, object: &Reconstructed<T>) -> bool {
        self.0 == object.object().committed()
    }
}
