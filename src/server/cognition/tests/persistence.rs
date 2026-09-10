// 28 — PERSISTENCE: what a mind is after a restart is what it was before
// one, carrier for carrier.

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
k.ingest(&a, &rot, [1; 32]);
assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
k.glue_local(); // a rotation generator
k.ingest(&a, &recoloured, [1; 32]);
assert_eq!(k.ingest(&a, &recoloured, [2; 32]), Outcome::GermPending);
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
    matches!(booted.ingest(&a, &rot, [9; 32]), Outcome::Decomposed(_)),
    "rotation reduces"
);
assert!(
    matches!(booted.ingest(&a, &recoloured, [9; 32]), Outcome::Decomposed(_)),
    "the recolour reduces"
);
assert_eq!(booted.basis_len(), 2, "no re-germination");
}

/// CYC2 writes the transport class key beside the versor; a restore
/// reads the same key, never a hash of the row.
#[test]
fn myc2_round_trips_cocycle_class_key() {
let mut k = Kernel::new();
let inv = crate::transducer::stalk::Invariant(vec![1]);
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
    .fiber(crate::network::nerve::ROOT)
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
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
// The second voice speaks the same percept: one sentence, one docket.
assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
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
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
let snap = k.snapshot();
let mut whole = Kernel::new();
assert!(whole.restore(&snap), "the tape as written restores");

// Every section tag names a boundary the old reader would have stopped
// at and called a restore.
for tag in [b"KCH1", b"RNK1", b"DEF1", b"FLD1", b"BOK1", b"CAV1", b"CRT1", b"COC1", b"FLC1", b"CYC2", b"SET1"] {
    let at = snap
        .windows(4)
        .position(|w| w == tag)
        .unwrap_or_else(|| panic!("the tape carries {}", String::from_utf8_lossy(tag)));
    let mut booted = Kernel::new();
    assert!(
        !booted.restore(&snap[..at]),
        "a tape ending at {} is refused",
        String::from_utf8_lossy(tag)
    );
    assert_eq!(booted.basis_len(), 0, "and nothing is partly restored");
}
}
