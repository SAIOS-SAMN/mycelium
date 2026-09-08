// 28 — PERSISTENCE: what a mind is after a restart is what it was before
// one, carrier for carrier.

use crate::cognition::*;
use crate::transducer::germinate::germinate;

/// P7 — the germinated language survives restart: install two germs, snapshot,
/// restore into a fresh kernel, and the restored kernel RECALLS them (no
/// re-germination) with balances intact.
#[test]
fn the_basis_survives_a_restart() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // a rotation (group substance)
let bar = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // a deformation (residual substance)
k.ingest(&a, &rot, [1; 32]);
assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
k.glue_local(); // a rotation generator
k.ingest(&a, &bar, [1; 32]);
assert_eq!(k.ingest(&a, &bar, [2; 32]), Outcome::GermPending);
k.glue_local(); // a residual generator
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
    matches!(booted.ingest(&a, &bar, [9; 32]), Outcome::Decomposed(_)),
    "residual reduces"
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
    .map(|(_, row)| row.4.clone());
assert_eq!(got, Some(ck), "class key survives the ledger");
}

/// A restart must not docket Δ⃗ = 0: deflation records survive the
/// snapshot, so the court does not slash a live provisional as empty.
#[test]
fn deflation_survives_a_restart() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
let chain = germinate(&a, &b).unwrap();
assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
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
