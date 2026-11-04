// MU-DERIVATIONS D2.5 — the pointed-connection book is bounded by the
// process's memory: unanswered connections are kept while their held bytes
// fit `M_proc`, and a new one past that is refused by name.

use crate::transport::{admit_pointed, PointedRefusal};
use mycelium::physics::{Memory, MemorySource};

fn speaker(port: u16) -> std::net::SocketAddr {
    std::net::SocketAddr::from(([127, 0, 0, 1], port))
}

/// D2.5 — A POINTED CONNECTION PAST THE PROCESS'S MEMORY IS REFUSED: a
/// memory of exactly one entry's bytes keeps the first speaker and refuses
/// the second by name, carrying the bytes it would have held, and the book
/// keeps the entry it held rather than letting one go.
#[test]
fn admit_pointed_refuses_past_memory() {
    let entry = std::mem::size_of::<std::net::SocketAddr>() as u64;
    let one = Memory::of(std::num::NonZeroU64::new(entry).expect("an entry has bytes"), MemorySource::CgroupMax);
    let mut held = Vec::new();
    assert_eq!(admit_pointed(&mut held, speaker(1), Some(one)), Ok(()), "one entry fits");
    assert_eq!(admit_pointed(&mut held, speaker(2), Some(one)), Err(PointedRefusal::PastMemory { held: Some(entry * 2) }));
    assert_eq!(held, vec![speaker(1)], "the held entry stays");
}

/// D2.5 — AN UNSEALED PROCESS KEEPS NO POINTED CONNECTION: with no memory
/// sealed there is nothing to hold the book against, and the connection is
/// refused `Unsealed`.
#[test]
fn admit_pointed_refuses_unsealed() {
    let mut held = Vec::new();
    assert_eq!(admit_pointed(&mut held, speaker(1), None), Err(PointedRefusal::Unsealed));
    assert!(held.is_empty());
}
