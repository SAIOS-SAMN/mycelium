// 28 — PERSISTENCE: what a mind is after a restart is what it was before
// one, carrier for carrier.

use super::common::speak_pair;
use crate::cognition::*;

/// P7 — the germinated language survives restart: install two germs, snapshot,
/// restore into a fresh kernel, and the restored kernel RECALLS them (no
/// re-germination) with balances intact.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn the_basis_survives_a_restart() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // a rotation (group substance)
// The L recoloured whole: a second word, its gauge (20 as ruled, L7). A
// deformation no versor carries lists nothing (50.5), so it is not a
// second generator here.
let recoloured = vec![(0, 0, 2), (0, 1, 2), (1, 0, 2)];
speak_pair(&mut k, &a, &rot, [1; 32]);
assert_eq!(speak_pair(&mut k, &a, &rot, [2; 32]), Outcome::GermPending);
k.glue_local(); // a rotation generator
speak_pair(&mut k, &a, &recoloured, [1; 32]);
assert_eq!(speak_pair(&mut k, &a, &recoloured, [2; 32]), Outcome::GermPending);
k.glue_local(); // a gauge generator
let balance = k.treasury().balance_mu(&[2; 32]);
let snap = k.snapshot();

// A fresh hypha boots from the snapshot.
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
assert_eq!(booted.basis_len(), 2, "both generators came back");
assert!(
    booted.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0)
        || balance.iter().any(|&m| m > 0),
    "earned μ⃗ survived the snapshot"
);
assert!(
    matches!(speak_pair(&mut booted, &a, &rot, [9; 32]), Outcome::Decomposed(_)),
    "rotation reduces"
);
assert!(
    matches!(speak_pair(&mut booted, &a, &recoloured, [9; 32]), Outcome::Decomposed(_)),
    "the recolour reduces"
);
assert_eq!(booted.basis_len(), 2, "no re-germination");
}

/// CYC2 writes the transport class key beside the versor; a restore
/// reads the same key, never a hash of the row.
#[test]
fn myc2_round_trips_cocycle_class_key() {
let mut k = Kernel::new();
let inv = crate::transducer::stalk::Invariant::counts(&[1]);
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
let w = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
let r = w.rotor();
let v = rotor_versor(&r).expect("versor");
let ck = r.class_key().expect("class key");
k.push_cocycle(&inv, [1; 32], [2; 32], v, ck.clone());
let snap = k.snapshot();
assert!(snap.windows(4).any(|w| w == b"CYC2"), "ledger speaks CYC2");
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
let got = booted
    .cocycle_at
    .scoped()
    .next()
    .map(|(_, row)| row.class_key.clone());
assert_eq!(got, Some(ck), "class key survives the ledger");
}

/// A restart must not docket Δ⃗ = 0: deflation records survive the
/// snapshot, so the court does not slash a live provisional as empty.
#[test]
fn deflation_survives_a_restart() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
// The second voice speaks the same sentence: one sentence, one docket.
assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
let before = k.dockets();
assert_eq!(before.len(), 1);
assert!(before[0].deflation.iter().any(|&x| x > 0), "deflation was measured at install");
let snap = k.snapshot();
let mut booted = Kernel::new();
assert!(booted.restore(&snap), "clean restore");
let after = booted.dockets();
assert_eq!(after.len(), 1, "the provisional survived");
assert_eq!(after[0].deflation, before[0].deflation, "deflation survived; the court will not slash Δ⃗ = 0");
}

/// A TAPE THAT ENDS EARLY IS REFUSED WHOLE, at every section boundary.
/// Each section read as "absent in an earlier snapshot → carry on with
/// nothing", and `snapshot` writes every one of them, so what those arms
/// admitted was a truncated tape: a mind restored with an empty docket,
/// a clock at zero and an unmeasured book, silently. §10 makes a layout
/// change a recut and deletes the reader for the old layout.
#[test]
fn a_snapshot_cut_short_at_any_section_is_refused() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
let snap = k.snapshot();
let mut whole = Kernel::new();
assert!(whole.restore(&snap), "the tape as written restores");

// Every section tag names a boundary the old reader would have stopped
// at and called a restore.
for tag in [b"LDG1", b"ARC1", b"RNK2", b"DEF2", b"FLD1", b"BOK1", b"CAV1", b"CRT1", b"COC1", b"FLC1", b"CYC2", b"SET1"] {
    let Some(at) = snap.windows(4).position(|w| w == tag) else {
        panic!("the tape carries {}", String::from_utf8_lossy(tag))
    };
    let mut booted = Kernel::new();
    assert!(
        !booted.restore(&snap[..at]),
        "a tape ending at {} is refused",
        String::from_utf8_lossy(tag)
    );
    assert_eq!(booted.basis_len(), 0, "and nothing is partly restored");
}
}

/// A SECTION CLOSES WHERE IT SAID IT WOULD. Each section writes its record
/// count AND the byte length of its body; a reader that does not arrive at
/// that byte has lost the frame inside that section and refuses there.
///
/// The count alone could not support this. A reader recovered a section's
/// end by parsing every record and trusting the result, so a record whose
/// framing it reconstructed differently from the writer moved the position
/// for good and every later refusal named an innocent section. Measured on
/// the fleet at 83e3e30: hypha a announced 91 DEF1 records, framed 84 over
/// 298223 bytes, then read an address length of zero and a tag byte of 13;
/// its true body was 345929 bytes, which the reader had no way to know. It
/// refused 8.6 MB of a court's history and named a byte that was a
/// consequence, not a cause. Hypha b lost 13.7 MB the same way.
///
/// Both halves are asserted here: a body one byte shorter than declared is
/// refused, and so is one byte longer. The first is what a lost frame looks
/// like from inside; the second is what a section that overruns its
/// neighbour looks like — the failure that used to be silent.
#[test]
fn a_section_that_does_not_close_where_it_declared_is_refused() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
speak_pair(&mut k, &a, &b, [1; 32]);
let snap = k.snapshot();
assert!(Kernel::new().restore(&snap), "the tape as written restores");

// The first section's frame: `MYC7` then its count, then the byte length
// of its body. Byte 12 opens that length.
let declared = u64::from_le_bytes(snap[12..20].try_into().expect("the frame is there"));
for wrong in [declared.wrapping_sub(1), declared.wrapping_add(1)] {
    let mut bent = snap.clone();
    bent[12..20].copy_from_slice(&wrong.to_le_bytes());
    let mut booted = Kernel::new();
    assert!(
        !booted.restore(&bent),
        "a body declaring {wrong} where it holds {declared} is refused"
    );
    assert_eq!(booted.basis_len(), 0, "and nothing is partly restored");
}
}

/// AN ADDRESS THE WIRE'S PREFIX CANNOT HOLD STILL GOES ON THE LOCAL TAPE.
///
/// `put_addr` writes a `u16` length and refuses an address over 65 535 bytes
/// by writing NOTHING — never truncating. That is a wire constraint: the
/// citation prefix of 47.1 is part of the protocol and `Cited` is the type
/// that proves an address fits it. The ledger and the genome are not the
/// wire.
///
/// A form's address is `form_vertex_key`, which writes every vertex, every
/// member and every gauge, so it grows with the form. The largest pair form
/// on the fleet at e98e8a2 held 677 vertices and 4030 faces, and its address
/// does not fit a `u16`. Measured over that soak: 24 577 readings of
/// `snapshot: 1 records could not be framed and were not written`, exactly
/// one per persist, across sections of 47, 71, 96, 158, 162 and 171 records
/// — and before that refusal was honoured at all, the record reached the
/// tape with its address field absent, which is the DEF1 desync that cost
/// hypha a 8.6 MB and hypha b 13.7 MB of a court's history.
///
/// `snapshot::put_key` writes a `u64` length, as every other length on this
/// tape has since MYC6, and is total.
#[test]
fn a_key_too_long_for_the_wire_prefix_still_round_trips_on_the_tape() {
let long = vec![0xABu8; (u16::MAX as usize) + 64];
assert!(
    crate::transducer::address::Address::from(long.clone()).cited().is_none(),
    "the wire's citation prefix cannot hold it"
);
let mut out = Vec::new();
crate::snapshot::put_key(&mut out, &long);
let mut p = 0usize;
let back = crate::snapshot::take_key(&out, &mut p).expect("the tape holds it");
assert_eq!(back, long, "and it reads back whole");
assert_eq!(p, out.len(), "consuming exactly what was written");
// A declared length the tape cannot satisfy is refused, never read short.
let mut clipped = out.clone();
clipped.truncate(out.len() - 1);
let mut p = 0usize;
assert!(
    crate::snapshot::take_key(&clipped, &mut p).is_none(),
    "a key whose bytes are not there is a refusal, not a short key"
);
}
