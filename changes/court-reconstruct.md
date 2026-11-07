### A restore is a reconstruction: `Court::reconstruct` → `Reconstructed<Court>` (#28, track 25 X0)
- Ruled on #28 (2026-11-06, option (a)). `Court::restore(&mut self, &[u8])
  -> bool` is removed. A snapshot is read by
  `Court::reconstruct(self, &[u8]) -> Result<Reconstructed<Court>,
  RestoreRefusal>`: `self` is the court holding its line and eras from
  disk (or a fresh court where the line rides the snapshot), and
  `Reconstructed::court` is the only constructor. Its laws are the checks
  the restore already ran: each receipt re-derives, the line verifies on
  its root, a held line holds the stamp the snapshot names, each row
  continues its receipt, and each word re-derives its citation.
- `RestoreRefusal` names a retired layout by its magic (`Retired`), a tape
  that is no ledger (`NotLedger`), or the trie and byte its reading
  stopped at (`Unread`). The server's boot and `--check` print it.
- Every call site, the server boot and `--check` are migrated, with no
  legacy path (the dev-state recut rule). Claim
  `restore_reconstructs_and_checks_receipts`, in the drift register.
- The drift register notes that the mutation-audit claim is covered by
  `reconstructed_is_built_only_by_the_laws` and
  `vertex_admits_only_a_reconstructed_fold`. The report of changed
  document sentences is retired in ROADMAP X0 as theatre.
- ROADMAP track 25's two sentences on turns keep the winding with each
  phase (LAW-FIELD amendment (c)).
- No wire or ledger layout change.
