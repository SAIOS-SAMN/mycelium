// The economy's tests, one file per ORGAN they prove. Each file's header
// names the chapter of MU-RECURSION the claim comes from.

mod fold;
mod ledger;
mod pay;
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
    (crate::chronometry::Stamp { h: k, vk: [0; 32], k, i: 0 }, hash)
}

/// A TREASURY THROUGH ITS PERSISTED FORM (78.27): its stakes and balances
/// through their own tries, its rows each through the row codec the ledger
/// nodes carry them in, rebuilt from those parts. `None` where any part
/// refuses.
pub(super) fn round_trip(t: &crate::economy::Treasury) -> Option<crate::economy::Treasury> {
    let mut tries = Vec::new();
    t.put_tries(&mut tries);
    let mut pos = 0usize;
    let (stakes, balances, held) = crate::economy::Treasury::take_tries(&tries, &mut pos)?;
    if pos != tries.len() {
        return None;
    }
    let mut rows = Vec::new();
    for e in t.entries() {
        let mut tape = Vec::new();
        e.write(&mut tape);
        let mut p = 0usize;
        rows.push(crate::economy::Entry::read(&tape, &mut p).filter(|_| p == tape.len())?);
    }
    Some(crate::economy::Treasury::from_parts(stakes, balances, held, rows))
}

/// THE PLACE OF ACT `k` while every act closes its orbit: a fold's head.
pub(super) fn place(k: u64) -> crate::chronometry::Place {
    crate::chronometry::Place { k, i: 0 }
}
