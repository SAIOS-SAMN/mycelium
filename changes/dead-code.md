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
- Sixty functions with no caller and no document are removed, each
  confirmed by its path and type, among them `Clock::k`,
  `observe::live::process`, `cognition::germ_address`, the court's
  `active_cover`, `unlist_foreign`, `open_charts`, `taught`, `classes_on`,
  `cohesion`, `torsion_total`, `is_home_listing`, `quote_frontier`,
  `settlements_since`, `settlements_heard_to`, `set_court_head`,
  `court_head`, `court_federation_len`, `field_cold_len`,
  `settlements_hot_len` and `dynamic_lane_capacity`,
  `Surgery::{bridge_merge, split_cut}`, `cga::{ray_name, parabolic_name}`,
  `Pole::of` and `SpanTally` with its `of`; and the helpers and types only
  they used (`EscrowedAward`, `LaneMembers`, `chart_span`, `orbit_span`,
  `Stratum::child_axes`).
- The idle consolidation worker is retired (clients think, hyphas verify):
  `synthesis::consolidate`, `Court::consolidate`, `syntropic_stroke`,
  `optimize`, `consolidate_stroke`, `retire_redundant` and the
  `syntropic_cursor` they kept are removed. The reflex opening's
  `condense_stroke` stands, and the condensation and torse tests now run
  on it. `physics::equilibrium::consolidate` is unchanged. Proposal 78.157
  marks S3 retired.
