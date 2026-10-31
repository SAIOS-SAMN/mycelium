## What and why

<!-- What this change does and why. Name its ROADMAP track or issue (e.g. "Track 14", "Closes #12"). -->

## Network contract

- [ ] No change to the Concordance, the wire, the ledger layout or a law version
- [ ] Changes the network contract (operator approval required; see docs/MU-RELEASE.md Part I)

## Checks

- [ ] Gate green: `cd xtask && cargo run --release`
- [ ] Scoped tests pass (name them, and any known exceptions)
- [ ] Documentation in step with the code; every citation of a renamed name follows
- [ ] CHANGELOG entry under Unreleased
- [ ] No `cargo fmt` run; files changed by hand only
