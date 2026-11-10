### RETRIEVE, `ACT_DOMAIN` sub-kind 19: the priced coordinate retrieval on the wire (#159)
- Additive wire act, no recut. A staker's signed request (staker, point,
  cited final point, nonce) is answered with the receipts held at the
  point's tick, the fee claimed on the stake at the read price, and the
  court's 47.1 section over its head signed over the request
  (`anastome::dialogue::retrieve`, `Court::retrieve_signed`, the door in
  `judge/domain.rs`). Refusals by name: unsigned, unknown domain, not
  final, origins, folded, short, overflow.
- The retriever checks a served answer with `retrieve::verify_retrieved`:
  the section verifies and is one claim with its head, it signs over this
  request, and every receipt re-derives at the tick, at or below the
  head's height.
- SDK: `Session::retrieve`; `mycelium_sdk::house` re-exports the codec and
  the check.
- Owed: pricing `ACT_RECEIPTS`'s replay arm for a folded tick (it needs a
  staker and a signature on that request).
- Claims `retrieve_round_trips_as_domain_sub_kind_19`,
  `retrieve_claims_reads_price_and_refuses_unpaid`,
  `retriever_verifies_section_and_receipts_under_head`.
