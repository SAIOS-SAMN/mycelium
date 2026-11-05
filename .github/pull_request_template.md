## What and why

<!-- What this change does and why. Name its ROADMAP track or issue (e.g. "Track 14", "Closes #12"). -->

## Network contract

- [ ] No change to the Concordance, the wire, the ledger layout or a law version
- [ ] Changes the network contract (operator approval required; see docs/MU-RELEASE.md Part I)

## Checks

- [ ] Definition of done (AGENTS.md) green with main merged in: `cargo clippy --release --workspace --all-targets -- -D warnings` and `cargo test --release --workspace` (quote the results)
- [ ] A change to the wire or the mesh: `scripts/quickstart` ran green, observer and timeline included
- [ ] Documentation in step with the code; every citation of a renamed name follows
- [ ] CHANGELOG fragment: `changes/<branch or topic>.md`, opened by its `### ` heading
- [ ] No `cargo fmt` run; files changed by hand only

## Drift (docs/MU-ANTI-FLATTENING.md clauses 12–14; AGENTS.md)

- [ ] No common grid past the sensor: frames related by the motion between them, each body held by its boundary
- [ ] No object admitted on a digest: reconstructed under the laws; each digest typed as an address, a link or a commitment
- [ ] Each of reconstruction, rebuild, replay and resend used in its own sense (MU-SCIENCE §5.0)
- [ ] No squaring, unenclosed infinity, float or chosen constant; a disagreement held as a residue or a named refusal
