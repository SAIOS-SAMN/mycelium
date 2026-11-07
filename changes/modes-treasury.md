### v0.5.0: the treasury holds modes
- **Network contract:** changed, a recut. A journal row (`Entry`, on the
  ledger, in the snapshot and on `ACT_ENTRIES`) carries its modes in the
  `Modes` codec, never four lanes; a row's fewest bytes are 126. The
  stakes, balances and held tries carry modes. A fold proposal's epochs
  (`ACT_FOLD`) are nets of modes. Persisted treasuries read before the
  recut are not migrated.
- **The founding mode** (#169, ruled 2026-11-06): `Mode::Founding { lane }`,
  the lane's identity motion stated explicitly, written as the lane byte
  with the high bit set (`modes::FOUNDING`). A genesis allocation sits at
  it; so does a mint whose proof cites no motion (a subsidy).
- **Every carrier is `Modes`:** balances, bonds, held value, the journal,
  and the issued and burned folds. The audit compares the two equations of
  §6 mode by mode. Per-lane answers (`balance_mu`, `bond_of`, `supply`,
  `account`) are readings of the modes.
- **Per-lane draws in proportion** (`Modes::draw`, `part_at`, `capped`,
  `split_at`): a per-lane amount is drawn from the modes on its lane in
  proportion to their amounts, exactly; a short lane is refused
  `LaneShort` by name; the modes drawn travel with the transfer.
- **A bond holds the modes it was staked from**; an unstake, a slash and a
  burn draw from the bond's modes.
- **Mints carry the modes their proofs cite:** a solve's closure modes
  (`pay_solve` takes them), a release's re-traversed closure
  (`release_verified`, `release_discharged`), an install's closure and its
  citation shares (the cited word's closure modes, `cited_modes`), POWC
  (`settle_convergence`), POWR and a rewrite (the absorbed family's modes,
  `closure_modes_of`), POWV (`verification_modes`, the lane's factor on
  each mode).
- **Owed:** a simplex fill's volume, a torsion cycle and a refill's twist
  are minted at the founding mode, since their motions are not read as
  modes at the pay site; a connection word's local section has no modes
  reader and falls back to the founding mode.
- **Claims** (`economy/tests/holdings.rs`):
  `genesis_allocation_sits_at_the_founding_mode`,
  `founding_mode_is_keyed_by_its_lane_and_round_trips`,
  `draw_takes_a_lane_in_proportion_to_its_modes`,
  `draw_refuses_a_short_lane_and_moves_nothing`,
  `bond_holds_the_modes_it_was_staked_from`,
  `transfer_carries_the_modes_drawn_from_the_payer`,
  `mint_carries_cited_modes_and_subsidy_the_founding_mode`,
  `audit_holds_mode_by_mode_through_the_persisted_form`.
- MU-ECONOMY §1 states the rulings; ROADMAP 1b records the build.
