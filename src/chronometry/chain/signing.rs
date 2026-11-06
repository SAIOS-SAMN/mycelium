// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// ROADMAP track 24 B2/B4, issue #118 — WHERE THE LEDGER'S SIGNING EVENTS
// STAND: the coordinates of every `Cite::Signing` a held node carries, kept
// where a node is filed and where it leaves.
//
//! A kernel's signing act is an event on a `Domain` receipt (`Cite::Signing`).
//! The alignment reading (`Court::alignment`) is derived from those events
//! alone, so the ledger keeps where they stand: `kernel‖stamp‖n`, the kernel
//! the citation names, the receipt's stamp, and the citation's place among
//! the receipt's citations. Coordinates only, written out: the event itself
//! is read from the node at its stamp whenever the reading asks, never held
//! beside it, and no reading or verdict is kept here (Concordance 1.13).
//!
//! The index moves with the nodes: every door that files a node (`append`,
//! `store`, `hold`) files its signings, and the one door a node leaves by
//! (`take_node`) takes them. A kernel's events are one fibre, `kernel‖…`, so
//! the reading of one kernel visits its own events and nothing else.

use crate::chronometry::{ActKind, Cite, KStepReceipt, Stamp};
use super::ReceiptChain;

/// The key a signing citation is filed under: `kernel‖stamp‖n`.
pub(crate) fn signing_key(kernel: &[u8; 32], at: &Stamp, n: u64) -> Vec<u8> {
    let mut key = Vec::with_capacity(32 + Stamp::LEN + 8);
    key.extend_from_slice(kernel);
    at.put_canonical(&mut key);
    key.extend_from_slice(&n.to_be_bytes());
    key
}

/// The stamp and citation place a signing key names, exactly or not at all.
fn signing_at(key: &[u8]) -> Option<(Stamp, u64)> {
    let mut p = 32;
    let at = Stamp::take_canonical(key, &mut p)?;
    let n = u64::from_be_bytes(key.get(p..p + 8)?.try_into().ok()?);
    (p + 8 == key.len()).then_some((at, n))
}

/// Each signing citation of `r` with its place among `r`'s citations: none
/// for a receipt of another kind, as the reading reads none there.
fn signings_of(r: &KStepReceipt) -> impl Iterator<Item = (u64, &[u8; 32])> {
    let domain = r.kind == ActKind::Domain;
    r.cites.iter().enumerate().filter(move |_| domain).filter_map(|(n, c)| match c {
        Cite::Signing { kernel, .. } => Some((n as u64, kernel)),
        _ => None,
    })
}

impl ReceiptChain {
    /// A node filed: each signing it carries filed by its coordinates.
    pub(super) fn file_signings(&mut self, r: &KStepReceipt) {
        for (n, kernel) in signings_of(r) {
            self.signings.insert(signing_key(kernel, &r.stamp, n), ());
        }
    }

    /// A node gone: each signing it carried leaves with it.
    pub(super) fn unfile_signings(&mut self, r: &KStepReceipt) {
        for (n, kernel) in signings_of(r) {
            self.signings.remove(&signing_key(kernel, &r.stamp, n));
        }
    }

    /// WHERE THE SIGNING EVENTS STAND on this ledger, `only`'s where one
    /// kernel is named, every kernel's otherwise: each the stamp of the node
    /// that carries it and its place among that node's citations, in kernel
    /// then stamp order. Reading them costs one visit per event, never one
    /// per node held.
    pub fn signing_stamps(&self, only: Option<&[u8; 32]>) -> Vec<(Stamp, u64)> {
        let walk = match only {
            Some(k) => self.signings.fibre_at(k),
            None => Some(self.signings.scoped()),
        };
        walk.into_iter().flatten().filter_map(|(key, _)| signing_at(&key)).collect()
    }

    /// How many signing citations the held nodes carry.
    pub fn signing_count(&self) -> usize {
        self.signings.len()
    }
}
