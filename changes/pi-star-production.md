### π* bound in production at the solve's A9 judgement (#158; MU-DERIVATIONS D3.3)
- **Recut.** The element record's anchor field gains tag `2`: a
  superposition of two or more selection terms, its count and every term.
  Tag `0` (⊤) and tag `1` (one term) are written as before. A build without
  this change misreads a record carrying tag `2`, so a mesh running mixed
  builds is recut (dev-state wipe ruling). No act, verdict or registry
  snapshot layout changes.
- The withheld pairs are fixed by coordinate at registration:
  `TaskRegistry::post` records each withheld output's place
  `[split, pair, member]` (`withheld_at`), and `withheld` serves exactly the
  pairs at those coordinates. The court binds them with the family at boot
  (`Court::bind_task_withheld`), and the door no longer hands withheld
  pairs to a judgement: `Court::judge_task_solve_on_stroke(rule, task)`
  reads the pairs bound for the task.
- A word's footprint is deduced, never declared:
  `transducer::family::footprint::footprint_of` speaks the word onto a
  pair's input and answers the sites its speech changes; `pair_change`
  answers the pair's own change and `missed` the sites a footprint misses.
- `court::verdict::bind_pi` binds the whole common set (`derive_pi`, no
  longer test-only) over the word's footprints on the family and on each
  withheld pair whose change the footprint covers, metered by `Budget`. A
  pair it misses is named `Unbound` with the missed sites and judges
  nothing; where no withheld pair binds, the word stays ⊤.
- `Element::with_anchor` is no longer test-only and binds the whole set;
  `Element::anchor` answers the terms. Where a solve is `Solved`, the rule
  docketed and paid carries the common set bound; a refused binding leaves
  it ⊤ and the door logs why.
- Claims: `withheld_pairs_are_fixed_by_coordinate_at_registration`,
  `footprint_is_the_sites_the_words_speech_changes`,
  `footprint_misses_a_change_the_word_does_not_speak`,
  `bind_pi_binds_common_set_over_deduced_footprints`,
  `word_missing_a_withheld_change_is_not_bound_on_that_pair`,
  `judged_rule_carries_its_anchor_on_registered_withheld_pairs`,
  `record_carries_superposed_anchor_whole`.
