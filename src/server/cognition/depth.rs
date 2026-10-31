// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! L15, 45.1, 1.9 — DEPTH IS READ FROM THE RECEIPTS, and stored nowhere.
//!
//! An address's depth is not a number a carrier holds. It is the count of
//! DISTINCT SIGNERS whose receipts of kind `Install`, `Ruling` or
//! `Adopted` name that address — a reading of the causal poset the chain
//! already is, computed at the read (law 12) and never written down.
//!
//! `Standing` is untouched. Standing says which stratum of one mind's own
//! language a word sits in (24.1); depth says how many independent organs
//! have committed to it. A word can be `Ruled` here at depth 1 and the
//! same word `Ruled` elsewhere at depth 3, and neither reading is the
//! other's — 47.3's arity is depth, not standing.
//!
//! What IS held is the index: address → signer ‖ kind → K. Without it
//! the read covers the whole chain for every address, which is the
//! enumeration law 3 forbids. It is derived and rebuilt from the chain
//! at restore, like the axis and moduli indices.

use super::*;

/// The three kinds that COMMIT an organ to an address (47.3). An attest,
/// a listing and a settlement are acts about a word; these three are a
/// vantage standing behind it.
fn commits(kind: crate::chronometry::ActKind) -> bool {
    matches!(
        kind,
        crate::chronometry::ActKind::Install
            | crate::chronometry::ActKind::Ruling(_)
            | crate::chronometry::ActKind::Adopted
    )
}

/// Signer ‖ kind, both written out (A21): the coordinate an attestation
/// is named by. Two acts of one signer on one address at two kinds are
/// two entries and one signer, which is what `depth_of` reads.
fn attestor_key(attestor: &[u8; 32], kind: crate::chronometry::ActKind) -> Vec<u8> {
    let mut k = Vec::with_capacity(33);
    k.extend_from_slice(attestor);
    k.push(kind.byte());
    k
}

impl Court {
    /// Record one receipt on the depth index. Called wherever a receipt
    /// joins this court's chain — its own acts and every peer receipt it
    /// admits — so the index and the chain cannot disagree.
    pub(crate) fn note_attestor(&mut self, r: &crate::chronometry::KStepReceipt) {
        // A node commits its signer to what it CITES, located by the
        // stamp that admitted it (78.24): depth is read off that stamp's
        // signers by prefix, never off a node's bytes.
        let key = attestor_key(&r.stamp.vk, r.kind);
        // A docket's class or demand is located by its coordinates and names no
        // admitting stamp: it commits its signer to no stamped word, and the
        // depth index, read off stamps, holds nothing for it. Depth is read
        // only where this court holds a word, so a stamp no word stands at
        // files nothing — but the act's own stamp, where the act admits what
        // it names and the word is filed as the act lands.
        let own = r.stamp.key();
        for at in r.cites.iter().filter_map(|c| c.at()) {
            let at = at.key();
            if at != own && !self.held_at.contains(&at) {
                continue;
            }
            match self.attestors.get_mut(&at) {
                Some(bucket) => {
                    bucket.insert(&key, r.stamp.place());
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(&key, r.stamp.place());
                    self.attestors.insert(at, bucket);
                }
            }
        }
    }

    /// A PEER ACT'S OWN STAMP, ONCE ITS ALIAS IS READ: where no word here
    /// stands at it, the depth filed there leaves.
    pub(crate) fn settle_own_attestation(&mut self, r: &crate::chronometry::KStepReceipt) {
        let own = r.stamp.key();
        if !self.held_at.contains(&own) {
            self.attestors.remove(&own);
        }
    }

    /// L15 — THE DEPTH OF A WORD THIS COURT HOLDS, read at the stamp that
    /// admitted it. Zero where it holds none.
    pub fn depth_of(&self, address: &[u8]) -> usize {
        match self.admitted_at(address) {
            Some(at) => self.depth_at(&at),
            None => 0,
        }
    }

    /// L15 — THE DEPTH AT AN ADMITTING STAMP: how many distinct organs have
    /// committed to what it admitted. The read is over that stamp's OWN
    /// entries — its boundary, which rule 1 allows — and the distinct
    /// signers are read from a keyed set built for the read and dropped
    /// after it, so nothing about depth is stored (law 12).
    ///
    /// Zero is a true reading: what no organ has committed to has depth
    /// zero, and that is a fact rather than an absence.
    pub fn depth_at(&self, at: &crate::chronometry::Stamp) -> usize {
        let Some(bucket) = self.attestors.get(&at.key()) else {
            return 0;
        };
        let mut signers: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (k, _step) in bucket.scoped() {
            // The key is the signer's 32 bytes followed by the kind byte:
            // read both out, never a prefix guess.
            let [vk @ .., kind] = k.as_slice() else {
                continue;
            };
            let Some(kind) = crate::chronometry::ActKind::of_sealed(*kind) else {
                continue;
            };
            if !commits(kind) {
                continue;
            }
            signers.insert(vk, ());
        }
        signers.scoped().count()
    }

    /// Rebuild the depth index from the ledger this court holds: EVERY
    /// node, its own and the peers' it admitted, exactly as it read them
    /// when they arrived. Derived, never persisted. (It read the court's
    /// own line alone, so depth from peers was lost at every restore.)
    pub(crate) fn rebuild_attestors(&mut self) {
        self.attestors = crate::network::nerve::Keyed::new();
        // Only a node that cites by stamp is filed, and only those are
        // copied out of the ledger.
        let nodes: Vec<crate::chronometry::KStepReceipt> =
            self.chain.nodes().filter(|r| r.cites.iter().any(|c| c.at().is_some())).collect();
        for r in &nodes {
            self.note_attestor(r);
        }
    }

    /// 45.9 — THE GENERATORS THIS COURT HOLDS, BY DEPTH. One row per
    /// depth reached, in depth order, as a Vec of relations: the reading
    /// the instrument prints beside the count of adjoined generators.
    /// Never a total and never an average (law 5).
    pub fn generators_by_depth(&self) -> Vec<(usize, u64)> {
        let mut rows: crate::network::nerve::Keyed<u64> = crate::network::nerve::Keyed::new();
        for (address, ()) in self.ruled_addrs.scoped() {
            let d = self.depth_of(&address);
            let key = (d as u64).to_be_bytes().to_vec();
            match rows.get_mut(&key) {
                Some(n) => *n += 1,
                None => {
                    rows.insert(key, 1);
                }
            }
        }
        rows.scoped()
            .filter_map(|(k, n)| {
                let raw: [u8; 8] = k.as_slice().try_into().ok()?;
                Some((usize::try_from(u64::from_be_bytes(raw)).ok()?, *n))
            })
            .collect()
    }
}
