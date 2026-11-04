// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ANTI-FLATTENING 13, MU-HOSTING §5a.10 — THE RECONSTRUCTED OBJECT:
//! what admission takes, built only by a law's reconstruction.
//!
//! An object is admitted by reconstructing it from its own evidence under
//! the laws, never because a digest it carries equals one held. So every
//! `admit_*` of the receipt path takes a `Reconstructed<T>` and never the
//! claimed `T`. Its one field is private and its only constructors are the
//! reconstructions below, so a value of the type is the proof that the law
//! ran. Outside this module the struct cannot be written
//! (`reconstructed_is_built_only_by_the_laws`):
//!
//! ```compile_fail,E0451
//! fn claimed(r: mycelium::chronometry::KStepReceipt) -> mycelium::court::admission::Reconstructed<mycelium::chronometry::KStepReceipt> {
//!     mycelium::court::admission::Reconstructed { object: r }
//! }
//! ```
//!
//! The lawful path, which compiles:
//!
//! ```
//! fn reconstructed(r: &mycelium::chronometry::KStepReceipt) -> Option<mycelium::court::admission::Reconstructed<mycelium::chronometry::KStepReceipt>> {
//!     mycelium::court::admission::Reconstructed::receipt(r)
//! }
//! ```

use crate::chronometry::{KStepReceipt, Stamp};

/// AN OBJECT RECONSTRUCTED UNDER THE LAWS (ROADMAP track 25, X0): built
/// only by the reconstructions of this module. It reads as the object
/// (`Deref`) and is never written from one.
#[derive(Debug, Clone)]
pub struct Reconstructed<T> {
    object: T,
}

impl<T> Reconstructed<T> {
    /// The object the law reconstructed.
    pub fn object(&self) -> &T {
        &self.object
    }

    /// The object, taken out of the proof: what a carrier stores once
    /// admission has ruled on it.
    pub fn into_object(self) -> T {
        self.object
    }
}

impl<T> std::ops::Deref for Reconstructed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.object
    }
}

impl Reconstructed<KStepReceipt> {
    /// 78.24, 47.2 — A RECEIPT RECONSTRUCTED FROM ITS OWN COORDINATES: its
    /// chain hash re-derives from its stamp, kind, frame, predecessor,
    /// parents, citations and operator (`KStepReceipt::rederives`). `None`
    /// where it does not: the receipt claims a hash its coordinates do not
    /// give, and nothing admits it.
    pub fn receipt(r: &KStepReceipt) -> Option<Reconstructed<KStepReceipt>> {
        r.rederives().then(|| Reconstructed { object: r.clone() })
    }

    /// A PAGE OF RECEIPTS, reconstructed in order up to the first that does
    /// not re-derive: the reconstructed run, and the stamp of the receipt
    /// that stopped it where one did. A back-fill admits the run and stops
    /// where the page stops (47.2 step 3).
    pub fn receipts(page: &[KStepReceipt]) -> (Vec<Reconstructed<KStepReceipt>>, Option<Stamp>) {
        let mut run = Vec::with_capacity(page.len());
        for r in page {
            match Reconstructed::receipt(r) {
                Some(rec) => run.push(rec),
                None => return (run, Some(r.stamp)),
            }
        }
        (run, None)
    }
}
