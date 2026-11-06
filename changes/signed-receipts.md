### Signed receipts reach the mind, which presents equivocation proofs (#137)
- The session reads the 47.1 receipt section every hypha's digest carries
  behind it (`sign_cover_digest`), which it skipped before, and holds each
  signer's last one (`client::sdk::signed_receipts::SignedReceipts`). A
  section is held only where the court's own checks pass on it: its
  signature verifies, its receipt re-derives its chain hash, and the prefix
  and the receipt are one claim (`court::admission::one_claim`, now public).
- Two sections under one key at one place of one line with two chain hashes
  are decided by the court's predicate (`admit_equivocation`) and presented
  as `ACT_EQUIVOCATION` to every hypha the session holds an address for
  (MU-RECURSION 47.4; MU-XSEPTUM §5.5: proofs by anyone). Each reading is
  typed (`Holding`: opened, advanced, duplicate, behind, refused by its
  34.1 constant, equivocated). No wire change: the section already rode
  every digest reply.
- Claims: `signed_receipt_held_only_where_the_court_would_admit_it`,
  `signed_receipt_duplicate_advances_and_falls_behind_by_place`,
  `two_signed_receipts_at_one_place_are_the_equivocation_proof`,
  `session_presents_an_equivocation_its_digests_carried`.
