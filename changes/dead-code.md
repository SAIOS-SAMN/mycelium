### Dead code removed (2026-11-05)
- The retired acts' codecs are removed: the docket request and reply
  (`encode_docket_request`, `decode_docket_request`, `encode_docket_reply`
  and its `_with` and `_paged` forms, `decode_docket_reply`,
  `decode_settlements`, `decode_cavities`, and the `Docket` record), the
  teach and settle codecs, and `encode_outcome_reply`. The bytes stand in
  `Action` and the door still refuses them by kind; one byte fixture per
  retired act keeps that refusal tested.
- The retired settlement arms and duplicates are removed:
  `Treasury::settle_refine`, `settle_condense` and `settle_assembly` (the
  assembly premium retired with its producer, cut 2026-09-10; MU-EQUATIONS
  6.7 and 6.8 and the MU-ECONOMY table no longer list them),
  `Treasury::release_filled`, `Court::prove_folded` (the live era proof is
  `server/backfill.rs::prove_folded` through `Court::walk_folded`),
  `network::wire::{multiplex, demultiplex}`, and
  `verdict::{reconstruct_certificates_at, reconstruct_form_on}` with the
  `reconstruct_certificates` wrapper only they called.
- The replaced keyed-schema API is removed: `Keyed::{read_fibre,
  truncate_key, fibre_over, for_system}`, `coord::Any::undeclared`,
  `Stratified::{schema, keyed}`, and the never-constructed
  `SchemaRefusal::InvalidDepth` and `SchemaRefusal::Stratum`. The foreign-key
  refusal of `Keyed::read_exact` is tested on a typed carrier.
