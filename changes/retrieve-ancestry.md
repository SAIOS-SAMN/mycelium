### RETRIEVE carries each receipt's ancestry to the signed head (#159)
- The reply writes, beside each receipt served, the receipts on one path
  from it up to the court's signed head (`retrieve::Placed`), found by
  `Court::ancestry` on the hold's stroke.
- `verify_retrieved` checks each path: every step re-derives and names the
  one below it among its predecessor and parents, and the head names the
  last. A receipt of the right height off the head's chain is refused
  `RetrievalBreach::OffChain`, which replaces the height-only check.
- Claim `receipt_of_right_height_off_the_chain_is_refused`.
