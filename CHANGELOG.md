# Changelog

Releases follow [semantic versioning](https://semver.org/). In the 0.x
series, a minor version may change the wire and ledger layouts, and each
such change is named here.

## Unreleased

### Economy
- **POWV, proof of work verification:** a fourth POW++ arm.
  - The courts whose replay verified a closure take a share of its
    release. That is the home, plus the witness or peer court on an
    install or glue.
  - The share is per lane: `⌊M_l · y_l / (y_l + R)⌋`, split equally among
    the verifying courts, where `y_l` is the court's strain and `R` its
    calibrated reserve.
  - It is taken out of the release, so total issuance is unchanged.
  - It applies at every POUW release site: install, glue, solve and docket
    ruling.
  - Ledger rows carry arm byte 4, and the court's `acts` reading counts
    "verifications".
- **Compute leases:** escrow against a provider's capacity, released by
  proof (`docs/MU-ECONOMY.md` §5.2).
  - A payer holds μ for a provider over named jobs for a window of court
    heights.
  - Each verified solve of a leased job pays the provider its measured
    discharge out of the hold, capped by what is left, once per job.
  - The remainder returns to the payer on close, when the jobs are done,
    when the hold is spent, or when the window passes.
  - On the wire: `ACT_WALLET` sub-kinds 3 (open) and 4 (close), signed by
    the payer.
  - `Treasury::draw_held` pays part of a hold.
  - The court's `acts` reading counts "lease payments".
- `docs/MU-ECONOMY.md`: §5.1 specifies POWV, §5.2 the compute lease, §8
  POWV's ratio, and §9 records the implementation.

### Documentation
- `docs/MU-SCIENCE.md` specifies the network as a verifier of knowledge:
  - science kernels and physics profiles (declared law registers, the
    reference register, physics identity);
  - the translation of claims into Anastome and its adjudication;
  - verification classes: V0 replay, V1 certificates, V2 formal proofs,
    V3 deterministic re-execution, V4 law checks under a physics;
  - the proofs of work for knowledge (POUW, POWC across universes, POWR,
    POWV, compute leases);
  - interweaving universes and the physics atlas;
  - verifier kernels, the verifier interface, wire, readings, bounds and
    adversarial cases;
  - phases S1–S8 (ROADMAP track 12).
- `ROADMAP.md` lays out twelve development tracks in order:
  - the xSEPTUM Note;
  - compute leases (built; verification leases and leases through the
    Note next);
  - bond delegation;
  - membership after genesis;
  - multi-host deployment;
  - ledger evolution;
  - the rule language;
  - convergence as admission evidence;
  - the SDK surface;
  - the reference mind's court traffic;
  - specification reconciliation;
  - verified knowledge: science kernels.
- The README's *POW++ for clients* section maps each proof (POUW, POWC,
  POWR) to the client action that earns it, in the reference mind and
  through the SDK.
- The README's *Economics* section covers:
  - principles, μ and its lanes;
  - carriers and the eleven ledger row kinds;
  - issuance: mint on closure capped at discharge, the recycled pool,
    burn on retirement;
  - escrow and payments;
  - stake, room and discipline;
  - pricing, gas and the connection tax;
  - the supply invariant and its audit;
  - genesis, and the economy readings.

## 0.1.2 — 2026-10-30

### Documentation and tooling
- `scripts/quickstart` brings up a three-hypha network and an SDK client on
  one machine.
- `GLOSSARY.md` defines the terms of the code and the specifications.
- `docs/EQUATIONS.md` collects the equations, formulas and master equations
  in use, with their specification sections and code locations.
- The README carries a quick start, a network topology diagram and a
  task-to-payment sequence diagram.
- The project name is written "Mycelium" in prose. Crate, binary and path
  names stay `mycelium`.

## 0.1.1 — 2026-10-30

### Language
- **Rule elements.** Rules are stated as geometric elements
  (`transducer::sentence::form::rule_element`):
  - a *carriage*: a Cl(4,1)(ℚ) versor applied once or iterated to a
    contact, its source cleared, kept or swept, with paint and gauge maps;
  - 2-chains bounded by a selection;
  - 1-chains joining bodies;
  - stamps;
  - frame operations.

  Six earlier acts (recolour, repaint, carry, echo, slide, extend) are one
  carriage. `gloss` gives an element's English clause.
- **Elements on the wire.** A form carries its elements.
  - An element with a clause gloss is written in that clause's bytes, so
    form addresses are unchanged.
  - An element without a gloss is written under act tag 16.
  - Decoding admits one canonical tape per element.

### Nodes
- `mycelium-server --init` and `mycelium-client --init` create a home, mint
  its Ed25519 key, and print the node's genesis allocation line.
- `mycelium-server --sign-corpus DIR` writes a signed corpus manifest for
  registration.
- The full network bring-up runs from these commands and the provisioning
  files (`bind`, `peers`, `genesis`).

### SDK
- `examples/sdk_client.rs`: a complete client on `sdk::{Kernel, Session}`
  and `anastome::dialogue`. It does the following:
  - loads or mints a voice key;
  - opens a session;
  - requests a task;
  - extracts, mints and speaks a rule;
  - sends a signed solve and reads the verdict, resending a solve whose
    replay is in progress.

### Readings
- A mind's `now` reading lists its pending court answers by the path that
  takes them.
- The court's `judged` reading records each solve judged, by task and
  voice.
- The court records digest requests by asker.

### Repository
- Licensed under Apache-2.0 across all crates.
- The ARC-AGI-2 corpus is located through `arc::corpus` (`MYCELIUM_CORPUS`,
  or `data/arc-agi-2` at the repository root).
- Documentation is written in a specification register.
- Added `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md` and `VERSION`.

## 0.1.0

The development line of the mesh:
- **Hyphae:** admission, adjudication by replay, the second-vantage
  install, settlement of POUW, POWC and POWR, the receipt chain, ledger
  eras and co-signed folds.
- **Clock and wire:** the TUniversal clock, and the Anastome wire
  protocol.
- **Reference mind:** perception, the winding carrier, conversation, and
  seats in other kernels' houses.
- **Algebra and language:** Cl(4,1)(ℚ), the Cayley alphabet, and words and
  sentences.
