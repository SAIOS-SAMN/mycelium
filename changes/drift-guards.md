### Track 25 X0, third stage: a fold root's citation and a stratum's root typed by sense (#28)
- `Cite::Root`'s hash is a `Predecessor` (the link sense, as
  `Root::hash` already is), so `Cite` has no `==`. Two citations are
  compared by `Cite::restates`: two roots at one stamp with a chain
  continuing across their hashes, any other pair by its canonical
  writing. Claim `cite_has_no_equality` (`compile_fail`).
- `StratumDigest::root` is a `Locator` at its source
  (`sheaf::stratum_root` returns one), compared by `Locator::locates`
  in `discordant`, the court's confirmation (`locates_stratum` is
  removed) and the mind's rest on a starved stratum. A stratum digest has
  no `==`. Claim `stratum_digest_has_no_equality` (`compile_fail`).
- Both guards stand in the drift register (`gate/src/drift.rs`).
- **Network contract:** no layout change. A digests reply whose stratum
  root is not 32 bytes is now refused whole
  (`digests_reply_refuses_root_of_another_width`); before, such a root
  was read and located nothing. Every root this tree writes is 32
  bytes.
- Still open, and asked on #28: the clause 14 guard
  (`Reconstructed<State>` from a restore) and the report of changed
  document sentences. The mutation-audit entry the issue names is not
  built, because the mutation audit was retired on 2026-11-04.
