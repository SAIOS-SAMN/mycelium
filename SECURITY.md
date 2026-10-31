# Security

Mycelium moves value between signed identities, slashes bonds, and
finalizes a ledger by co-signature. Defects in those paths are security
defects, and they are handled privately until a fix is released.

## Reporting a vulnerability

Report privately through GitHub's private vulnerability reporting:
**Security → Report a vulnerability** on
[SAIOS-SAMN/mycelium](https://github.com/SAIOS-SAMN/mycelium/security/advisories/new).
Do not open a public issue for a vulnerability.

A report is most useful when it names:
- the affected act, carrier or ledger row, and the release or commit;
- the specification section the behaviour departs from;
- a reproduction: the acts sent, in order, and the reading or verdict
  observed;
- the effect: value created, moved or lost, a bond slashed or spared, a
  verdict forged, or a node stalled.

Reports are acknowledged, and the advisory is published with the fixing
release. Reporters are credited unless they ask otherwise.

## Scope

In scope:
- **Signatures and admission:** Ed25519 verification of every signed act;
  acts admitted unsigned, under the wrong key, or replayed across
  positions.
- **Settlement:** any path that mints, burns or moves μ other than the
  rulings of `docs/MU-ECONOMY.md`, or that breaks the supply invariant
  `supply = genesis + mint − burn = Σbalance + Σbond + Σheld`.
- **Escrow and leases:** held value released to the wrong party, released
  twice, or never returned.
- **Verification:** a verdict, install or POWV share obtained without the
  reconstruction and second vantage the specifications require
  (MU-RECURSION 47.3, `docs/MU-SCIENCE.md` §5).
- **Discipline:** a slash applied to an honest party, or evaded by an
  equivocating one.
- **Ledger and clock:** forged or reordered receipts, folds co-signed
  without the era they claim, TUniversal points made final without the
  keepers' readings.
- **Wire decoding:** a non-canonical encoding admitted, or a payload that
  exhausts a node's memory or stroke budget.

Out of scope: denial of service by volume against a single node on an
open port, and findings that depend on an operator's own key material
being disclosed.

## Security engineering

The work that hardens these paths is ROADMAP track 14: decoy universes, codec
fuzzing (a decode ambiguity is a release-blocking defect), standing μ
bounties on protocol invariants posted as listings, self-proving
equivocation slashing (built for receipts), reproducible builds with
signed build attestation, resource metering for resident minds, external
anchoring of era folds, and machine-checked proofs of the core
invariants.

## Supported versions

Fixes are made on `main` and released as the next version. In the 0.x
series only the latest release is supported.
