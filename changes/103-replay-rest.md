### The rest of "replay" (verification), and `tests/word_lifecycle.rs`'s "grid": names and strings left by the prose cleanup (#103)
- Names only, no behaviour change. Rounds out the two renames above with
  the instances a first pass missed: the test `assert`/`expect`/`panic`
  strings using "replay" in the verification sense across some twenty
  test files (`server/cognition/tests/{mint,ruling,relator}.rs`,
  `client/sdk/tests/rule.rs`, `bin/mycelium-server/judge/tests/offer.rs`,
  `transducer/alphabet/tests/rotor_key.rs`,
  `transducer/family/tests/speech.rs`,
  `transducer/field/tests/contemplation.rs`,
  `transducer/sentence/{tests/properties,tests/refraction,
  tests/rotor_class,form/tests/complex,pivot/tests/orbit}.rs`,
  `transducer/tests/factor.rs`, `tests/word_lifecycle.rs`), each reworded
  to "reconstruct"/"reconstruction" to match the reconstruction calls
  and doc comments they sit beside.
- The same class as `glue_replay_refused` above, caught on a second
  re-grep: the observed reading `glue_unread`'s description
  (`observe::REPLICATION_AXES`), the SDK example's `SOLVE_UNREAD` label
  (`sdk/examples/client.rs`), a stale trace label on the step that calls
  `docket_reconstruction` (`"court stroke: docket replay"` →
  `"court stroke: docket reconstruction"`,
  `server/cognition/court/reconstruct.rs`), and a census carrier name
  that lagged its own local, already named `reconstruction_memo`
  (`"replay_memo"` → `"reconstruction_memo"`, `server/cognition/
  carriers.rs`).
- `tests/word_lifecycle.rs` also carries the cognition-sense "grid":
  an `expect` string, an `assert_eq!` string, and the local bound to
  `spoken.sample()`, reworded to `sample` alongside the file's own
  `grid`-pair-chain `expect` string, the same pattern fixed in the
  "grid" entry above.
- **Left as comments, not names or strings** (a separate, later PR):
  roughly a dozen remaining `///`/`//` comments still read "replay" in
  the verification sense (`court/verdict/reconstruct.rs`,
  `bin/mycelium-soak/delta.rs`, `economy/tests/ledger.rs`,
  `transducer/mind.rs`, `sources/arc-agi/tests/rule_language.rs`,
  `tests/word_lifecycle.rs`'s two remaining comments, among others);
  these are prose, not the names or strings this issue covers.
