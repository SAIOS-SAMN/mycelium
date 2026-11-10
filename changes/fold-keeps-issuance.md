### A fold removes nothing a reading reads (MU-ECONOMY §6, §6.1)
- **Recut.** The treasury's snapshot gains the `folded` trie: the issuance every fold checkpointed, keyed by each source's coordinate `[kind tag, arm byte]`. A snapshot without it is refused. No row, act or wire layout changes.
- **The defect.** A fold replaced its prefix's rows with one `epoch` row per identity carrying no proof, and the epoch rows counted as issuance. After a fold:
  - the earned supply (`minted`) and each proof's paid and burned (`economy`) dropped the folded payments;
  - `issued` and `burned` were netted into one another (issued fell by the prefix's burns);
  - genesis was no longer separable from earned value.

  The POW++ reading over a running mesh showed POUW paid as zero at the courts that had paid it.
- **The structure.** `economy::fold::FoldedIssuance`, the fold's checkpoint, keeps the replaced rows' gross issuance by source: genesis, mints, transfers and burns by arm byte, and slashes. An `epoch` row is a net of balances only (`Kind::issues` reads it on neither side). `issued`, `burned`, `minted` and `economy` are the checkpoint plus the live rows. A fold that would move issued or burned refuses `REFUSE_FOLD_INEXACT`. `Kind::tag` and `Kind::of_tag` are the one tag table the row tape and the checkpoint share.
- Claims: `fold_keeps_what_each_proof_paid`, `fold_keeps_genesis_apart_from_the_earned`, `fold_keeps_issued_and_burned_each`, `second_fold_reads_the_first_folds_issuance_once`, `checkpoint_source_at_no_coordinate_is_refused`, `restored_fold_keeps_its_issuance`. `fold_replaces_prefix_with_epoch_rows_and_conserves` no longer asserts the folded mints gone.
- Owed: a witness co-signs the nets it re-derives, not the checkpoint, which no fold receipt signs yet.
