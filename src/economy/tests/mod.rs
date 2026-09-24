// The economy's tests, one file per ORGAN they prove. Each file's header
// names the chapter of MU-RECURSION the claim comes from.

mod fold;
mod ledger;
mod settlement;
mod surface;

/// A word's address, as an award is filed under it.
pub(super) fn word(b: impl AsRef<[u8]>) -> crate::transducer::address::Address {
    crate::transducer::address::Address::from_bytes(b)
}

/// AN ANCHOR WHOSE NEXT ACT TAKES `K = n` (78.24): the stamp of the head
/// act it stands at, `n − 1`, and that act's hash. The empty hash stands at
/// no act — before genesis — and its next act takes `K = 0`.
pub(super) fn at(n: u64, hash: [u8; 32]) -> crate::economy::Anchor {
    let k = n.checked_sub(1).map_or(0, |k| k);
    (crate::chronometry::Stamp { h: k, vk: [0; 32], k }, hash)
}
