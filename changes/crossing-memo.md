### A crossing reads its reserve's signing events, never the whole ledger (#118)
- The ledger keeps where each signing event stands, `kernel‖stamp‖n`, as
  coordinates only (`chronometry::chain::signing`): filed where a node is
  filed (`append`, `store`, `hold`) and taken where it leaves
  (`take_node`). `ReceiptChain::signing_stamps(only)` reads one kernel's
  fibre, or every kernel's.
- The court reads its own folded line's signing events once per era, as
  the alignment reading first meets the era (`FoldedSignings`, volatile):
  a closed era returns nothing new. An ask with no new own era reads
  nothing of the archive.
- `Court::alignment` (`ACT_LAWS` mode 9) and `Court::signatory` (B4) spend
  one read per signing event, plus one per receipt of an era read for the
  first time, in place of one read per receipt held. The reading is still
  derived on every ask; no reading or verdict is kept (Concordance 1.13).
  Both take `&mut self`.
- No wire, layout or persisted carrier changes. Claims
  `crossing_alignment_reads_only_signing_events`,
  `alignment_equals_full_walk_across_own_fold`,
  `alignment_equals_full_walk_on_a_peer`
  (`server/cognition/tests/signing_index.rs`).
