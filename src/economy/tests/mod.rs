// The economy's tests, one file per ORGAN they prove. Each file's header
// names the chapter of MU-RECURSION the claim comes from.

mod fold;
mod forfeit;
mod holdings;
mod ledger;
mod modes;
mod mu;
mod pay;
mod settlement;
mod surface;
mod verify;

/// AN AMOUNT READ AS COUNTS, for a claim whose values are whole: each lane
/// must be the rational `n/1`, or the claim fails there. The claims written
/// in counts keep each value exactly (`Mu::counts` is the inverse).
pub(crate) trait Counts {
    type Out;
    fn n(self) -> Self::Out;
}

impl Counts for crate::economy::Mu {
    type Out = [u64; 4];
    fn n(self) -> [u64; 4] {
        self.to_counts().expect("every lane a whole count")
    }
}

impl Counts for &crate::economy::Mu {
    type Out = [u64; 4];
    fn n(self) -> [u64; 4] {
        self.to_counts().expect("every lane a whole count")
    }
}

impl<T: Counts> Counts for Option<T> {
    type Out = Option<T::Out>;
    fn n(self) -> Self::Out {
        self.map(Counts::n)
    }
}

impl<T: Counts, E> Counts for Result<T, E> {
    type Out = Result<T::Out, E>;
    fn n(self) -> Self::Out {
        self.map(Counts::n)
    }
}

impl<K, T: Counts> Counts for Vec<(K, T)> {
    type Out = Vec<(K, T::Out)>;
    fn n(self) -> Self::Out {
        self.into_iter().map(|(k, t)| (k, t.n())).collect()
    }
}

impl<A: Counts, B: Counts> Counts for (A, B) {
    type Out = (A::Out, B::Out);
    fn n(self) -> Self::Out {
        (self.0.n(), self.1.n())
    }
}

impl<A: Counts, B: Counts, C: Counts> Counts for (A, B, C) {
    type Out = (A::Out, B::Out, C::Out);
    fn n(self) -> Self::Out {
        (self.0.n(), self.1.n(), self.2.n())
    }
}

/// A MEASURED AMOUNT AT EACH LANE'S FOUNDING MODE: the modes a claim's
/// fixture mints where the claim is about the treasury's arithmetic and no
/// motion is read (#169: a mint whose proof cites no motion).
pub(crate) fn founding(m: impl Into<crate::economy::Mu>) -> crate::economy::Modes {
    crate::economy::Modes::founding(&m.into())
}

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
/// each through its own trie, its rows each through the row codec the ledger
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
