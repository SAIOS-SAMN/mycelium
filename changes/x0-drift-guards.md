### The drift guards: typed digests, `Reconstructed<T>`, the drift register (track 25, X0; #28)
- **Network contract:** no wire byte, act or ledger layout changes. One
  admission result changes: a signed frame whose receipt does not
  re-derive its own chain hash is now refused `REFUSE_RECEIPT` before
  admission reads it. The signed path re-derived the receipt's height and
  never its hash, so such a receipt could be stored.
- **`Reconstructed<T>`** (`court::admission::reconstructed`). An object
  reconstructed under the laws. Its field is private, and only the
  module's reconstructions build it: `Reconstructed::receipt` (the chain
  hash re-derives from the receipt's own coordinates) and
  `Reconstructed::receipts` (a fetched page, up to the first receipt that
  does not). The receipt path's admissions take it and never the claimed
  receipt: `admission::admit`, `admit_backfilled`, `admit_equivocation`,
  `Court::admit_receipt`, `Court::admit_backfill`,
  `ReceiptChain::admit_unreached` and `Court::evict_contradicted`.
- **The digest types** (`court::admission::digest`). The 32 bytes compared
  at admission are typed by sense, and none of the three types has
  `==`. `Locator` (address) is compared by `locates`. `Predecessor`
  (link: the digest a record names the record before it by) is compared
  by `continues`, `forks` and `cites`. `Commitment` is compared by `binds`,
  which takes only a `Reconstructed<T>`. Converted: the receipt line's
  checks in `admit_chain`, the prefix's claim (`one_claim`), the
  equivocation proof, the signed-reach gate's contradiction, the ledger's
  `holds`, `height_of` and era walk, the back-fill chase's held and
  forked citations, a folded citation's recall, and the endowment's
  granted and applied manifests.
- **The drift register** (`xtask/lint/drift.rs`). Each guard records its
  regression, ruling, guarding type and claim. The gate refuses an entry
  whose type or claim is named empty or absent from the tree. It holds
  five guards. By ruling, it records no mutation-audit entry.
- **Renamed:** the founding outcome `Reconstructed` (`Applied`, `Held`,
  `Refused`) is `FoundingEvent`. `Reconstructed<T>` names only the
  reconstructed object.
- **Claims:** `address_has_no_equality`, `predecessor_has_no_equality`,
  `commitment_has_no_equality`, `reconstructed_is_built_only_by_the_laws`
  (`compile_fail`), `commitment_binds_only_a_reconstructed_object`, and
  the gate's `drift_refuses_entry_missing_its_claim` and
  `drift_refuses_entry_missing_its_type`.
- **Open:** the fields stay bare `[u8; 32]` (`Parent::hash`,
  `KStepReceipt::chain_hash`, `SignedPrefix::prev_receipt_hash`,
  `Admitted::Equivocation`), so they can still be compared directly
  outside the converted sites. `ReceiptChain::admit` and the vertex's
  admissions (`admit_uplink`, `admit_chained`, `admit_domain`; X2) do not
  take `Reconstructed<T>` yet. The report of changed document sentences
  is not built. The guards of MU-ANTI-FLATTENING 12 and 14 are not yet in
  the register.
