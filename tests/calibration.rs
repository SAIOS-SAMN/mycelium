#![allow(clippy::expect_used, reason = "§2.4 binds cognition; this crate is a test")]
//! MU-DERIVATIONS D2.5 — THE BOOT CALIBRATION STROKE, in a process of its
//! own that allocates through jemalloc, as the server, client and soak
//! binaries do: the stroke measures the bytes per read and seals the
//! process's capacity, once.

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use mycelium::physics::{calibrate, capacity, minimal_stroke, sealed, spend_unit, Calibration, STROKES_PER_RESERVE};

/// D2.5 — THE CAPACITY IS SEALED FROM THE MEASURED BYTES PER READ BEFORE
/// THE FIRST ACT IS SERVED. Unsealed, one spend is the minimal stroke. The
/// calibration reads `M_proc` (the process's cgroup limit, else the host's
/// available memory), measures `b > 0` on the boot stroke and seals `C =
/// ⌊M_proc / b⌋`; every reader then reads the sealed `C`, which is
/// reconstructed from its own readings by the derivation alone; and a
/// second calibration returns the same seal, with no second stroke. What
/// serves (the server's nerve) is built from the `Sealed` witness, which
/// only `calibrate` writes, so nothing is served before this seal.
#[test]
fn reserve_sealed_from_measured_bytes_per_read_before_serving() {
    let minimal = minimal_stroke().expect("the founding motions' modes have a need");
    assert_eq!(spend_unit(), minimal.get(), "unsealed, one spend is the minimal stroke");
    assert_eq!(sealed(), None, "nothing sealed before the calibration");
    let seal = calibrate().expect("a process that says, through a counted allocator, seals");
    let c = seal.calibration();
    assert_eq!(capacity(), seal.capacity(), "every reader reads the seal");
    assert_eq!(sealed(), Some(c));
    assert_eq!(c.capacity().get(), c.memory().bytes().get() / c.footprint().get(), "C = ⌊M_proc / b⌋");
    assert_eq!(Calibration::of(c.memory(), c.footprint(), c.minimal()), Ok(c), "the seal reconstructs from its readings");
    assert!(spend_unit() >= minimal.get(), "the minimal stroke fits one spend");
    assert!(c.capacity().get() / STROKES_PER_RESERVE >= minimal.get());
    assert_eq!(calibrate(), Ok(seal), "sealed once");
}
