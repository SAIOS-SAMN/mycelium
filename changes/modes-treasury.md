### Mints carry the modes their proofs cite: citations, POWC, POWR and POWV (MU-ECONOMY §2.5, §5.1; #169)
- **A use or citation share and POWC carry the cited word's closure modes.**
  `Court::cited_deflation` holds the cited form's local discharge on the
  modes its closure moves on the citer's family at the citer's placement
  (`Mu::carried_by`); `settle_recursive` mints each share on its own
  modes, never on the citer's.
- **POWR and a rewrite carry the modes of the family they absorb.**
  `Court::absorbed_modes` reads each retired word's closure modes on its
  bound family; `Treasury::release_rewrite` takes them and sets the
  escrowed yield on them (the escrow's own modes where none are read).
- **POWV scales each mode on its lane.** `verification_share` holds the
  base's modes, so the verifying courts take the re-traversed closure's
  modes and the closer keeps the same modes in proportion.
- **An install's base keeps the discharge's modes:** the measured
  discharge capped lane-wise by the escrow, as `release_verified` reads it,
  and the rest after POWV drawn from it in proportion. In
  `settle_recursive` the voices' gluing is what is left capped lane-wise at
  the gluing (a reading), and their rest after K3 is drawn from it, so the
  voices hold the closure's modes, never the founding mode. The claims
  `mint_sentence_mints_measured_deflation` and
  `mint_sentence_mints_closed_subcomplex_deflation` now read the mint lane
  by lane and hold that no founding mode is minted.
- Claims: `verification_share_scales_each_mode_on_its_lane`,
  `release_rewrite_carries_the_absorbed_modes`,
  `citation_share_carries_the_cited_modes`,
  `cited_deflation_holds_the_cited_closure_modes`,
  `absorbed_modes_read_the_retired_word_closure`.
- **Network contract:** unchanged in layout; the modes a mint holds change
  in value, riding unreleased `0x0300` with #169's recut.
- MU-ECONOMY §9 records what is built.
