### The receipt records typed by sense (track 25, X0 stage 2; #28)
- **Network contract:** no wire byte, act, ledger layout or signed byte
  changes. Each typed field is the same 32 bytes, read in and written out
  by `of_bytes` and `bytes`.
- **The record fields carry the digest types.** `Parent::hash`,
  `KStepReceipt::chain_hash`, the fold `Root::hash` and the two hashes of
  `Admitted::Equivocation` are `Predecessor` (link).
  `SignedPrefix::prev_receipt_hash` and `KStepReceipt::prev_hash` are
  `Commitment`. None has `==`, so no bare comparison of a record's digest
  can be written.
- **`PartialEq` removed** from `Parent`, `KStepReceipt`, `SignedPrefix`,
  `Root`, `Era`, `Admitted` and the fold `Proposal`. New named readings:
  `Parent::of` (the citation naming a held receipt) and `Root::restates`
  (one place, one height, a chain continuing across the two hashes).
  Moved with the fields: the packed line's anchors and predecessors
  (`chronometry::pack`), the archive's era comparisons (`era_locates`,
  address sense), `verify_chain_integrity`, the fold's witness check,
  and the snapshot's ledger rows read against their receipt.
- **Tests:** whole-record comparisons are rewritten to the named
  methods (`one_receipt`, `one_run`, `one_root`, `one_era`,
  `one_archive`, `one_proposal` and `one_prefix` in the test fixtures),
  or to the coordinates written out. The SDK's receipts conformance
  sample seals its receipt and checks that the decoded one re-derives.
  No claim was dropped.
- **Open:** `Cite::Root`'s hash is still bare.
- **The vertex admits reconstructed acts.** `ReceiptChain::admit` takes
  a `Reconstructed<KStepReceipt>`. `Court::admit_uplink` and
  `admit_chained` take a `Reconstructed<SignedFold>`, and
  `Court::admit_domain` a `Reconstructed<SignedDomain>`. The new
  reconstructions `Reconstructed::fold` and `Reconstructed::domain`
  re-write the signed bytes from the act's own coordinates and verify the
  signature over them. `Court::hear_uplink` and `Court::hear_domain`
  reconstruct the act as heard, answering `Unsigned` where it does not
  reconstruct; the `ACT_UPLINK` and `ACT_DOMAIN` judges call them. The
  answers are unchanged. Claim `vertex_admits_only_a_reconstructed_fold`
  (`compile_fail`), registered in the drift register.
- **Stratum roots compared as locators.** The court's confirmation of a
  peer's ruled digest (`confirm_peer_holds_on`) compares the stratum root
  it recomputes from its members, and the root it recorded for a heard
  word, with the peer's by `Locator::locates` (`locates_stratum`). A mind's
  rest on a stratum fetched for nothing (`CourtPace::starved`) holds the
  court's root as a `Locator`. A root that is not 32 bytes locates
  nothing, where before two such byte strings compared equal.
- **Not registered:** the drift guards of MU-ANTI-FLATTENING clauses 12
  and 14. Neither has both a guarding type and a claim in the tree:
  clause 12's `Turn` has public fields and the refinement it would refuse
  is deleted, and clause 14's `Court::restore` returns `bool`, not a
  `Reconstructed<State>`. MU-ANTI-FLATTENING *Drift guards* says why.
