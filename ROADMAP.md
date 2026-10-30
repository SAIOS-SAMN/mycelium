# Roadmap

The development tracks of Mycelium, in the order they build on each other.
Each names its specification, where it lands in the code, and how to take
part. [CONTRIBUTING.md](CONTRIBUTING.md) covers the workflow; open an issue
on a track to coordinate before starting.

The detailed task ledger behind these tracks is
[`docs/MU-RECURSION-TASKS.md`](docs/MU-RECURSION-TASKS.md).

## 1. Settlement: the xSEPTUM Note

**Specified** in [`docs/XSEPTUM.md`](docs/XSEPTUM.md). Lands in
`src/anastome/dialogue/` (acts), `src/economy/` (escrow and release) and
`src/server/cognition/` (the home's admission).

The Note carries value between voices through a hypha, released only on
two-court replay, as words are installed. The track's pieces:

- **Acts:** `SEPTUM_CHALLENGE`, `SEPTUM_NOTE`, `SEPTUM_RECEIPT`,
  `SEPTUM_RELEASE`, `SEPTUM_REVOKE` and `SEPTUM_EQUIVOCATION` (0x30–0x35),
  with their codecs and round-trip claims.
- **Escrow keyed by `(listing, payer, K_payer)`.** A duplicate Note is
  ignored, and a conflicting one is an equivocation proof.
- **Two-signature release:** home and witness each sign, on the
  second-vantage rule (47.3).
- **Receipt-indexed Notes**, and **FROST voice groups** (threshold Ed25519
  group keys for voices).

## 2. Economy: compute leases

**Built** in v0.2.0: [`docs/MU-ECONOMY.md`](docs/MU-ECONOMY.md) §5.2.

A requester holds μ against a provider's capacity over named jobs. Each
job the provider's verified solve closes pays its measured discharge out
of the hold, and the remainder returns at the lease's end. The track
continues with:

- **Verification leases:** jobs that are replays rather than solves, so a
  hypha's verification capacity can be reserved as a client's solving
  capacity is.
- **Leases through the Note:** a lease funded and released across courts
  by the xSEPTUM Note (track 1), so the payer's home and the provider's
  court need not be one.
- **SDK helpers** for opening, reading and closing leases.

## 3. Economy: bond delegation

**Specified** in [`docs/MU-ECONOMY.md`](docs/MU-ECONOMY.md) §4.

A delegated hypha carries other parties' bond in its cover, shares its
POWV with them, and shares their slash. This lands in `src/economy/` as a
delegation carrier on the treasury, with the audit extended over it.

## 4. Network membership

A hypha named in the genesis allocation is bonded by every court.
Admitting a hypha after genesis, with a bond every court holds, extends
the network beyond its founding set. The work includes:
- a signed admission act;
- bonding on every court;
- the witness-independence test (47.3) for the new organ.

Lands in `src/bin/mycelium-server/admission.rs` and the court's treasury.

## 5. Deployment

Multi-host and wide-area operation of the mesh. Acts are signed with
Ed25519 and verified on admission. The track adds:
- an encrypted transport;
- address discovery beyond the provisioned `peers`;
- signed re-provisioning of a node's `bind` and `peers` after first boot;
- deployment tooling for hosts beyond one machine.

## 6. Ledger evolution

Ledger layout changes are adopted by a recut today. A migration path that
reads a prior layout and writes the current one keeps court state across
releases. Lands in `src/chronometry/` and the snapshot codecs.

## 7. The rule language

Rules are geometric elements (`src/transducer/sentence/form/rule_element.rs`).
Elements without an English gloss travel under act tag 16. The track
grows what minds can state:
- readers that coin carriages, chains and joins directly from a family's
  effect;
- coverage of more of the ARC-AGI-2 corpus.

Each reader is proved by re-deriving held-out pairs and by the court's
replay.

## 8. Convergence as admission evidence

Two minds whose different words have the same section on one scene are a
convergence (`docs/MU-CLIENT.md` §5.5). Courts already verify and seal
convergences. The track makes convergence count as evidence in a word's
admission and payment, keeping 47.3's rule that a vantage is a court.

## 9. SDK

A stable, versioned client surface (`mycelium::sdk`), narrower than the
crate's public API, with semantic-versioning guarantees. It includes:
- a conversation API for third-party clients;
- rewrite elements (POWR) through the SDK;
- examples for each POW++ arm.

## 10. Reference mind

The reference mind (`mycelium-client`) runs its court asks on one carrier
per boundary. The track bounds each carrier's outstanding asks per path,
so a mind's court traffic stays proportional to its work across restarts
of the courts it speaks to. The `now` reading lists outstanding answers by
path (`src/client/sdk/entity/hypha.rs`).

**Open defect: the ask queue.** On a long-running network, a mind's
outstanding court asks grew without bound while its work did not. The
`now` reading's per-path listing (`waiting_by_path`, added in 0.1.1)
locates the growth by path. The cause is not yet identified. The track's
first phase reproduces the growth on a local network, names the path
whose answers are not taken, and bounds it.

## 11. Specification reconciliation

[`docs/EQUATIONS.md`](docs/EQUATIONS.md) records three places where the
specification's wording and the code differ:
- the Gram pair invariant;
- MU-RECURSION 52.1's rotor example;
- three function names in MU-RECURSION chapter 10.

Each becomes one documentation commit that brings the specification to
the code.

## 12. Verified knowledge: science kernels

**Specified** in [`docs/MU-SCIENCE.md`](docs/MU-SCIENCE.md).

The network becomes a verifier of knowledge, and of science specifically.
A kernel joins, runs its universe on the physics it declares above the
universal floor (L0, the Concordance, the alphabet, Anastome, exact
arithmetic), and puts its results to the courts. The courts adjudicate
the translation into Anastome, verify the claim by reconstructing it at
two vantages, relay what verifies, and pay through the proof that
represents the work. Admission depends on the object reconstructed, not
on the method that produced it:
- **POUW** for knowledge added;
- **POWC** for convergence across derivations and across universes;
- **POWR** for the same knowledge said in less;
- **POWV** for the courts and verifier kernels that checked it;
- **compute leases** for capacity reserved.

Mycelium's reference physics is the definitive library, and every other
universe's verified results form a library under its own physics
identity. The physics atlas records where universes converge and where
they diverge.

Phases:

1. **S1:** the verifier interface in the court, with rules as its first
   verifier and verdicts that carry the reconstructed object; claims on
   the wire; readings by class.
2. **S2:** certificate checking: exact LP optima with dual certificates,
   SAT assignments and factorizations first, UNSAT with DRAT proofs after.
3. **S3:** formal proofs with a Metamath checker; theorems citing installed
   lemmas.
4. **S4:** physics profiles sealed at domain genesis on the L0 floor; law
   checks under a declared physics.
5. **S5:** cross-universe convergence, the physics atlas, and verifier
   kernels bonded per domain.
6. **S6:** deterministic computation in WebAssembly with exact arithmetic,
   reconstructed by two courts.
7. **S7:** a Lean 4 kernel checker.
8. **S8:** the Concordance's science-kernel profile.

## 13. Peer kernels on the public mesh

**Specified** in [`docs/CONCORDANCE.md`](docs/CONCORDANCE.md) (1.14, 2.2,
2.10, A7).

netstratum and xylarium run their own universes on the mesh and link the
`forms` crate (README, *Peer kernels*). As their repositories become
public, the track makes the boundary between the kernels a versioned,
reviewable interface:
- **Pinning by tag:** each peer kernel depends on Mycelium and `forms` at a
  release tag, never at a local path or untagged commit.
- **Compatibility statement:** each Mycelium release names the wire,
  ledger and `forms` versions it serves, and the CHANGELOG names every
  change a peer kernel must take up.
- **Joint reads:** a law form lands by the Concordance's landing rule, with
  both universes' readings reproduced as decoded exact values; the track
  makes that read reproducible from the public repositories.
- **Founding on a fresh mesh:** each peer kernel founds its domains on a
  public network's genesis, with its physics profile sealed (track 12,
  S4).

## 14. Security review

The paths that move value, slash bonds and finalize the ledger are
reviewed against their specifications, by contributors outside the
original development, with each finding resolved as a defect or a
specification amendment. Reports go through [SECURITY.md](SECURITY.md).
Review targets, in order:
- **Admission:** Ed25519 verification of every signed act; canonical
  decoding (one encoding per object) of every wire payload.
- **Settlement:** each POW++ arm's mint, burn and share against
  `docs/MU-ECONOMY.md`; the supply invariant's audit.
- **Escrow and leases:** release, refund and lapse of held value.
- **Discipline:** slashing and equivocation proofs (MU-RECURSION 47.3,
  47.4).
- **Ledger and clock:** receipt chains, co-signed folds, and TUniversal
  finality.
- **Resource bounds:** stroke budgets and frame budgets under adversarial
  payloads.
