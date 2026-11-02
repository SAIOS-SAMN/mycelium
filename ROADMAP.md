# Roadmap

Mycelium hosts universes (games among them) in which external
AI agents explore, act and do science, and verifies and pays for what they produce
([README › What Mycelium hosts](README.md#what-mycelium-hosts)). These are
the development tracks of Mycelium, in the order they build on each other.
Each names its specification, where it lands in the code, and how to take
part. [CONTRIBUTING.md](CONTRIBUTING.md) covers the workflow; open an issue
on a track to coordinate before starting.

This file is the project's one plan of work: where it and an older
document differ on the order of work, this file governs the order of
work; what each piece is stays with its specification
([README › Documentation](README.md#documentation)). Components brought over from
precursor implementations are specified in
[`docs/MU-PRECURSORS.md`](docs/MU-PRECURSORS.md) and listed below under
the tracks they serve.

## Now

This section is the current order of work. It is kept current, not
appended to: an item leaves when its pull request merges. The numbered
tracks below say what each piece of work is; this section says which
pieces come first. Each item is a GitHub issue with its specification,
its claims and its files. Claim one by commenting on it before starting
([CONTRIBUTING › How to contribute](CONTRIBUTING.md#how-to-contribute)).

**1. Release in progress: v0.3.0**
([milestone](https://github.com/SAIOS-SAMN/mycelium/milestone/1)). This is
the shared contract that Mycelium and the peer kernels pin by git tag. It
is at `0.3.0-rc.2`. Remaining, in order:

| Item | Issue | State |
|---|---|---|
| T2, the transduction core: frames related by the motion between them, μ as a share of the frame | [#25](https://github.com/SAIOS-SAMN/mycelium/issues/25) | In progress (maintainers); merges after track 26's S0 and S4, its `Enclosure` holding `n` absorbed circles |
| Track 26, S0–S5: the sensor for any source at any rank; rows and columns absorbed or taken apart ([MU-SENSING](docs/MU-SENSING.md) §3.6) | [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32) | In progress (maintainers); in v0.3.0's one break. S6 follows v0.3.0. |
| Track 26, S4a and S7–S9: the sample lifted into a bubble (non-Euclidean, never scalar); its body, motions, forces, orbits and vortex ([MU-SENSING](docs/MU-SENSING.md) §3.7–§3.8) | [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32) | Specified; the carrier is the standard LAW-BUBBLE v1 ([proposal](docs/proposals/78.171-law-bubble.md), [#36](https://github.com/SAIOS-SAMN/mycelium/issues/36)), to be agreed with the peer kernels, then built |
| T3, the turn-native chart reader and the turn-valued Conformal motion | [#26](https://github.com/SAIOS-SAMN/mycelium/issues/26) | Ready after T2 |
| D2.5, one boot calibration stroke measures bytes per read | [#27](https://github.com/SAIOS-SAMN/mycelium/issues/27) | Ready |
| rc.3 on a test mesh, operator authorization, the signed tag, the peers' re-pins | [#24](https://github.com/SAIOS-SAMN/mycelium/issues/24) | After the three above (maintainers) |

**2. Ready to start**
([milestone `next`](https://github.com/SAIOS-SAMN/mycelium/milestone/2)).
None of these waits on a ruling:

| Item | Issue | Track | Notes |
|---|---|---|---|
| X0, the drift guards: typed digests, `Reconstructed<T>`, the drift register | [#28](https://github.com/SAIOS-SAMN/mycelium/issues/28) | 25 | Depends on nothing; the rest of track 25 stands on it |
| B0, replication by reconstruction: every hypha reconstructs each domain from its founding receipts | [#13](https://github.com/SAIOS-SAMN/mycelium/issues/13) | 24 | The rest of track 24 stands on it |
| `decode_peers_reply` refuses a deferral | [#16](https://github.com/SAIOS-SAMN/mycelium/issues/16) | 14 | In progress ([PR #31](https://github.com/SAIOS-SAMN/mycelium/pull/31)) |
| Identifiers that name reconstruction carry `reconstruct` | [#29](https://github.com/SAIOS-SAMN/mycelium/issues/29) | 11 | Good first issue; after T2 merges |

**3. Waiting on an operator ruling.** Do not start these until the ruling
is recorded:
- Concordance draft 4 (the shared laws, and the amendment proposed in
  MU-CONCORDANCE §11). Track 24's B2 and B3 wait on it.
- B1: carrying line mouth offers and tier handoff offers in the reserve's
  act, which amends rulings 78.126 and 78.136.
- B2: per-clause signatures, a wire change to `ACT_LAWS` mode 8.
- Proposed names: `NotSignatory`, `AddressOnly`.
- μ measured as the solid angle a change sweeps on the base sphere, in
  place of a share of the frame's sites (MU-SENSING §3.7). It changes what
  is paid, so it is not in v0.3.0 unless ruled.
- The open questions of MU-DERIVATIONS §7 and MU-RELEASE §10.

**4. Under investigation:**
[#14](https://github.com/SAIOS-SAMN/mycelium/issues/14) (an originate act
unanswered on a long-running mesh). It needs a reproduction.

## 1. Settlement: the xSEPTUM Note

**Specified** in [`docs/MU-XSEPTUM.md`](docs/MU-XSEPTUM.md). Lands in
`src/anastome/dialogue/` (acts), `src/economy/` (escrow and release) and
`src/server/cognition/` (the home's admission).

The Note carries value between voices through a hypha, released only on
two-court reconstruction, as words are installed. The track's pieces:

- **Acts:** `SEPTUM_CHALLENGE`, `SEPTUM_NOTE`, `SEPTUM_RECEIPT`,
  `SEPTUM_RELEASE`, `SEPTUM_REVOKE` and `SEPTUM_EQUIVOCATION` (0x30–0x35),
  each with its codec and round-trip claims.
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

- **Verification leases:** jobs that are reconstructions rather than solves, so a
  hypha's verification capacity can be reserved as a client's solving
  capacity is.
- **Leases through the Note:** a lease funded and released across courts
  by the xSEPTUM Note (track 1), so the payer's home and the provider's
  court need not be one.
- **SDK helpers** for opening, reading and closing leases.

## 3. Economy: bond delegation

**Specified** in [`docs/MU-ECONOMY.md`](docs/MU-ECONOMY.md) §4.

A delegated hypha carries other parties' bond in its cover, shares its
POWV with those parties, and shares each party's slash. This lands in
`src/economy/` as a delegation holding on the treasury, with the audit extended over it.

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
- an encrypted transport (**In progress**;
  [MU-PRECURSORS §3.1](docs/MU-PRECURSORS.md#31-noise-xx-encrypted-transport)):
  TCP wrapped in `Noise_XX_25519_ChaChaPoly_BLAKE2s`, one static key per
  host, trust-on-first-use pinning with a changed key refused by name,
  and identity kept in the Ed25519-signed acts. It lands as
  `src/network/noise.rs`, opt-in per peer at first;
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
reconstruction.

Components from precursor implementations
([MU-PRECURSORS](docs/MU-PRECURSORS.md) §3.3, §3.8, §3.9), each a court
change built with its codec, reconstruction and claims:
- **Named refusals in `learn_form`** (**In progress**, a defect fix):
  every vertex of a learned form is filed, refused by name, or not a word.
- **Act parameters measured per pair:** scale, tile and reduce by the
  frames' measured ratio, translation by measurement; refused by name when
  unmeasurable, with no silent tie-break.
- **A template placed over a lattice:** the Hermite form of the placement
  differences, the offset anchored to a selected body; no candidate
  enumeration.
- **First contact as a relation:** `Pred::FirstMet` (proposed) and
  `Act::Slide { toward }`, ties held whole, refused where the ray is not a
  lattice unit, no toroidal wrap.
- **Colour-role equivariance:** the gauge-orbit class with re-inflation,
  and an equivariance check that reports where commutation breaks.
- **Hue as a table** over a named measured coordinate, refused off the
  table.
- **Clauses learnable as vocabulary,** filed in the presentation on court
  admission.
- **Body fate as a typed precondition** of family extraction, with named
  refusals.

## 8. Convergence as admission evidence

Two minds whose different words have the same section on one scene are a
convergence (`docs/MU-CLIENT.md` §5.5). Courts already verify and seal
convergences. The track makes convergence count as evidence in a word's
admission and payment, keeping 47.3's rule that a vantage is a court.

- **Reconstruction events** (**Specified**;
  [MU-SCIENCE §5.0](docs/MU-SCIENCE.md#reconstruction-events)). Every
  reconstruction's meeting with held state is read as one typed event:
  Collapse (a superposition reduced to one object by admission), Converse
  (independent vantages reconstructing the same object from different
  evidence: POWC, μ by union) or Collision (incompatible claims on one
  structure, held as a vortex and resolved by content; a duplicate mint
  burned). The reading is `ReconstructionEvent::{Collapsed, Conversed,
  Collided}` under `ACT_OBSERVE`, with coordinates, never a scalar or a
  boolean. It belongs to this track because Converse is convergence
  itself, and converse against collision across a mesh is the measure of
  how far the mesh is one language and of partition healing
  ([MU-RELEASE §9](docs/MU-RELEASE.md#9-convergence-and-collision-bring-the-branches-into-alignment));
  it reads what the courts already decide and gates nothing, so it is not
  a security item (track 14).

## 9. SDK

A stable, versioned client surface (`mycelium::sdk`), narrower than the
crate's public API, with semantic-versioning guarantees. It includes:
- a conversation API for third-party clients;
- rewrite elements (POWR) through the SDK;
- examples for each POW++ arm;
- the SDK depths, stated in its documentation: depth 0 run a node,
  depth 1 bring tasks, depth 2 bring a solver, depth 3 bring physics
  ([MU-PRECURSORS §3.10](docs/MU-PRECURSORS.md#310-sdk-depths-and-a-file-feed-door)).

### SDK packaging: publicly available SDKs

**In progress** (P1, P2 and P5 Built) in
[MU-PRECURSORS §4](docs/MU-PRECURSORS.md#4-sdk-packaging-mycelium-sdk).
Today the repository is a workspace of `mycelium`, `forms` and
`mycelium-sdk` (`sdk/`) under one lock file; the SDK crate re-exports the
library's client surface in modules (`voice`, `session`, `task`, `rule`,
`seat`, `wallet`, `readings`, `mind`, `wire`, `laws`, `conformance`,
`forms`, `prelude`), `sdk/examples/client.rs` is the complete client on it
alone, the client conformance read is published
([MU-CONFORMANCE](docs/MU-CONFORMANCE.md)), and the MSRV is 1.82.
The track publishes SDKs open-source developers can depend on by version:
a `mycelium-sdk` crate, a workspace member, re-exporting a narrow,
versioned surface (identity, `Session`, tasks and solves, the rule seam,
seats, wallet and leases, readings), with semver guarantees separate from
the server, a wire compatibility statement per release, minimal
dependencies with feature flags (`noise`, `rules`, `seats`), docs.rs
documentation, examples per POW++ arm, publication to crates.io under
Apache-2.0, a client conformance read, and bindings for other languages
after it.

1. **P1, workspace (Built):** `mycelium`, `forms` and `mycelium-sdk` as
   members of one workspace, `xtask` outside it. *Accepted when* the gate
   passes on the workspace and the members share one lock file.
2. **P2, the surface (Built):** `mycelium-sdk` re-exporting the surface, and
   the client example (`sdk/examples/client.rs`) depending on it alone. *Accepted when* the
   example runs end to end against a local network and a compile test
   imports every surface item.
3. **P3, features and dependencies:** a default-on `server` feature on
   `mycelium` holding the server-only dependencies, and the SDK's own
   features. *Accepted when* the SDK builds with default features off and
   pulls no server-only crate.
4. **P4, documentation and examples:** docs.rs documentation with the
   depths, one example per POW++ arm, a lease and a seat. *Accepted when*
   each example is paid on a local network.
5. **P5, the client conformance read (Built;
   [MU-CONFORMANCE](docs/MU-CONFORMANCE.md)):** the acts a client must speak and
   the readings it must answer, served by a hypha and kept in the
   repository, with its suite (track 15, H1). *Accepted when* the
   reference mind and the `client` example pass it and a client missing a required
   act fails it by name. Conformance is by decoded exact values
   (Concordance 1.4).
6. **P6, publication:** `forms`, `mycelium` and `mycelium-sdk` on
   crates.io under Apache-2.0, with the compatibility statement in the
   CHANGELOG. *Accepted when* a fresh project builds the example from the
   published crate by version.
7. **P7, bindings:** a first binding in another language (for example
   Python), built on the conformance read and the wire specification,
   carrying exact values only. *Accepted when* it passes the conformance
   suite.

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

[`docs/MU-EQUATIONS.md`](docs/MU-EQUATIONS.md) records three places where the
specification's wording and the code differ:
- the Gram pair invariant;
- MU-RECURSION 52.1's rotor example;
- three function names in MU-RECURSION chapter 10.

Each becomes one documentation commit that brings the specification to
the code. A further documentation commit adds to `docs/MU-EQUATIONS.md` the
table of refused functionals (squared-norm coherence, scored-argmin
search, ratios of scalar residuals, ε-bands), so that no port brings them
over ([MU-PRECURSORS §3.11](docs/MU-PRECURSORS.md#311-documentation)).

The documentation now uses the typed vocabulary of
[MU-SCIENCE §5.0](docs/MU-SCIENCE.md#the-typed-vocabulary) (reconstruction,
rebuild, replay, resend); the code still names the court's reconstruction
`replay`. One code commit renames those identifiers to what they do,
updating every citation in the same commit (MU-STANDARDS 3.1):
`blind_replay*` → `reconstruct_blind*`, `replay_word` / `replay_resume` /
`replay_form*` / `replay_certificates_at` → `reconstruct_*`, the module
`court/verdict/replay.rs` → `reconstruct.rs`, `Replayed` / `ReplayCursor` /
`replay_cursors` / `replay_memo` → `Reconstructed` /
`ReconstructionCursor` / `reconstruction_cursors` / `reconstruction_memo`,
`measured_replay*` → `measured_discharge*`, `encode_replay_request` →
`encode_era_request`, the outcome names `REPLAY_FAILED` and
`GLUE_REPLAY_REFUSED` → `RECONSTRUCTION_FAILED` and
`GLUE_RECONSTRUCTION_REFUSED` (wire bytes unchanged), the fault class
`Replay` → `Resend`, and the test names that say "replays" for a
reconstruction. **Specified.**

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
   A **chain-complex certificate kind** joins them: geometry as data (cell
   counts and sparse exact boundary maps), refused as `NotAComplex` where
   the maps do not compose to zero and as `OpenBoundary { cell }` where the
   witness does not close, fuel in court strokes, identity by decoded
   coordinates, and payment through the POW++ lanes, never a cell-count
   formula ([MU-PRECURSORS §3.5](docs/MU-PRECURSORS.md#35-chain-complex-checker-as-a-v1-certificate-kind)).
   Integer homology (MU-PRECURSORS §3.4) serves its torsion and
   integer-solvability checks.
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

**Specified** in [`docs/MU-CONCORDANCE.md`](docs/MU-CONCORDANCE.md) (1.14, 2.2,
2.10, A7).

The peer kernels (listed in README › Peer kernels) were developed and run
alongside Mycelium on its development network, each with a universe of
its own, and link the `forms` crate. As each peer kernel's repository
becomes public, the track makes the boundary between the kernels a versioned,
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
each reviewed against its specification, by contributors outside the
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

### Security engineering

Ten additions, in four groups. Items 1–4 build on machinery already
in the tree and produce receipts; items 5–7 touch build and operational
surfaces outside the court; item 8 is the largest in scope; items 9 and
10 test the tests and the mesh, and come from precursor implementations
([MU-PRECURSORS](docs/MU-PRECURSORS.md)). The order of work across groups
is open.

**On machinery in the tree**

1. **Decoy universes** (**Specified**; MU-HOSTING §6.5). Sealed universes with
   attractive listings and nothing real to extract. Agents that come to
   extract are absorbed, observed and receipted. This is the draw put to
   defensive use, implemented as a kernel, with no new architecture.
2. **Codec hardening** (**Specified**). The Anastome decoders are where
   untrusted bytes meet the court, and "decoding admits one canonical tape"
   is a fuzzable invariant:
   - differential fuzzing of every encode/decode pair, so that a decoded
     value re-encodes to the same bytes and every other byte string is
     refused (`proptest` is already a dev-dependency, not yet used);
   - corpora of malformed tapes kept as regression claims: must-refuse
     cases asserted on the decoded result, never golden accept bytes
     (Concordance 1.4)
     ([MU-PRECURSORS §3.7](docs/MU-PRECURSORS.md#37-malformed-input-corpus-refuse-and-wait));
   - a typed distinction between **refuse** (the bytes can never become an
     act) and **wait** (an act has begun and not finished), with an exact
     length bound: a length at the bound waits, one byte past it is
     refused at the header;
   - a standing rule: a decode ambiguity is a release-blocking defect.
3. **Bug bounties through existing acts** (**Specified**). Standing μ
   bounties on protocol invariants (double mint, equivocation, a
   verification bypass), posted as listings and claimed with a
   reproduction the court can check. Security research enters as market
   demand, paid by the same court.
4. **Equivocation slashing** (**Built** for receipts). Two signed receipts
   under one key at one place with different chain hashes are
   self-proving: the court burns the signer's whole bond, from its own admission
   or from a third party's `ACT_EQUIVOCATION` (MU-RECURSION 47.4;
   `Court::accept_equivocation`; `court/admission/tests/equivocation.rs`).
   No judgement call is involved. The work left: the same self-proving
   slash for every signed act that holds a position (a lease, a Note, a
   fold), and a share of the burn to the presenter, so presenting a proof
   is paid work.

**On build and operational surfaces**

5. **Reproducible builds and signed build attestation** (**Specified**).
   Reconstruction applied to the software: a kernel attests the hash of a
   build any party can re-derive from the tagged source and `Cargo.lock`,
   and a court can refuse a kernel whose attested build does not match.
6. **Resource metering as physics for resident minds** (**Specified**;
   MU-HOSTING §5.1). Grade-one containment holds only if a resident mind's
   compute is budgeted. The stroke and reserve mechanism is extended to a
   resident mind's CPU and memory in its kernel, so a runaway resident is
   starved by law rather than stopped by an operator.
7. **External era anchoring** (**Specified**). Era folds are co-signed
   hashes. Publishing fold digests to a venue outside the mesh removes
   silent rewriting of history, even under a majority-compromised mesh. A
   hash here is lawful transport: it compacts coordinates kept losslessly
   in the ledger.

**Formal verification**

8. **Machine-checked proofs of the core invariants** (**Specified**):
   - **δ∘δ = 0** for the static Čech coboundary (`transducer::cech`),
     today asserted on examples (`cech/tests/transitions.rs`). The fold's
     asymmetric causal differential (MU-RECURSION, U12) respects the causal
     order rather than reading a static complex, so its identity is stated
     first, then proved;
   - **the treasury audit:** `supply = genesis + mint − burn = Σbalance +
     Σbond + Σheld` preserved by every operation that writes the ledger
     (`Treasury::audit`);
   - **canonical-decode uniqueness:** for every act kind, decode∘encode is
     the identity, and decode is injective on accepted tapes.

   Exact ℚ arithmetic removes the floating-point obstructions such proofs
   normally meet.

**Test assurance**

9. **Mutation audit** (**In progress**; MU-STANDARDS §4;
   [MU-PRECURSORS §3.2](docs/MU-PRECURSORS.md#32-mutation-audit)).
   `cargo run -- mutate` in `xtask` reads a catalogue of mutants (the law,
   the file, the exact span, the replacement, the narrowest test that must
   fail), applies each in a scratch worktree, and reports each as
   `killed`, `survived` or `stale`. A surviving mutant is a law no test
   notices; a stale one is re-anchored.
10. **Fault-injection campaign and invariant auditor** (**Specified**;
    [MU-PRECURSORS §3.6](docs/MU-PRECURSORS.md#36-fault-injection-campaign-and-invariant-auditor)).
    Ten named fault classes (`InvalidSignature`, `Replay` (a resend;
    renamed `Resend` with the code),
    `DoubleClaimRace`, `EquivocatingLedger`, `SybilSwarm`,
    `RespellRaceSaturation`, `DrainedPurseDraw`,
    `RestoreFromSnapshotSigner`, `MalformedName`, `GerminalFlood`) on four
    topologies, each campaign reproducible from a printed seed, with exact
    invariants audited after every campaign: conservation by ledger
    audit (`Treasury::audit`), once per work across the mesh, every burn revokes, and no
    refusal without a receipt. The auditor keeps its own independent view
    of an accused node, and concurrent faults target separate nodes. It is
    written against the current admission and treasury.

### Fairness across capacity

**Specified** in [`docs/MU-ECONOMY.md` §11](docs/MU-ECONOMY.md#11-fairness-across-capacity).
A mind on one laptop and a solver with many accelerators are paid by the
same rules: capacity buys speed, never a larger share of the same work
(payment is measured discharge, each closure pays once, discovery is paid
at every citation, admission is paced by room). The open parts, in order:

1. **Commit before reveal** (High; wire and ledger). `ACT_SOLVE_COMMIT`
   seals `payload_hash(rule ‖ voice ‖ task ‖ nonce)` as a receipt; the
   reveal carries the commitment's stamp and nonce; the discharge goes to
   the causally earliest commitment, and concurrent commitments to one
   rule share it. A rule seen in the clear can no longer be copied and
   resubmitted under another voice.
2. **One closure across courts** (High; ledger; with track 4). A first
   closure travels with the receipts every court back-fills, so a later
   solve of the same task at another court is judged a repeat; two first
   closures across a partition are the duplicate issuance of MU-RECURSION
   47.4 and the later is burned on rejoin.
3. **The race reading** (Medium; instrument). Per closure: the first
   closer, the earliest offer or commitment of its rule, the lanes paid;
   per voice, first closures per lane. The soak reads it to decide whether
   latency concentrates first closures before any mechanism for racing is
   designed.
4. **The frontier reading** (Medium; reading only; with track 9). The open
   tasks on a court, each with its ask per lane and any standing
   obstruction, paged to the frame budget and served by the SDK as
   `Session::frontier()`.

With this work, the solve receipt's code `1` is renamed `SOLVE_AGAIN` (the
wire byte unchanged): a repeat is verified and paid nothing.

## 15. Hosting: universes, world models and hosted minds

**Specified** in [`docs/MU-HOSTING.md`](docs/MU-HOSTING.md).

Mycelium hosts universes (cosmological simulations, games, science
universes, task environments, markets, many-agent universes and world
models) and the AI agents placed in them. The track completes the path
from attaching to being paid, and opens the network to new kinds of
universe:

1. **H1:** a client conformance read (the acts a client speaks and the
   readings it answers), and SDK helpers for seats, leases and withdrawal.
2. **H2:** pay on the scene, and union attestations (MU-RECURSION 78.168
   E1, E3).
3. **H3:** world-model onboarding: exact state declaration in the physics
   profile, boundary translation into Anastome, and law checks on
   reported transitions.
4. **H4:** a many-agent reference universe: one domain, regions, seats, and
   attestations as agreement.
5. **H5:** the published draw record (lane prices and standing work by
   chart), and the reference mind's containment readings on a running
   network.
6. **H6:** certification: a seeded, controlled-economy exam universe (a
   78.x proposal for ruling), and certificates any party can verify by
   reconstruction. A certificate attests behaviour over the task families
   its universe expresses, never "safe in general"; a universe may require one
   for its seats, and the courts gate nothing on it.

Components from precursor implementations
([MU-PRECURSORS](docs/MU-PRECURSORS.md) §3.10–§3.12):
- **H1:** the client conformance read is SDK packaging's phase P5
  (track 9).
- **A file-feed door** on the operator's path: a client on
  `sdk::{Voice, Session}` that submits each task-shaped file dropped in a
  directory as one claim and answers it beside the file, refusing by name
  what does not decode (depth 1, *bring tasks*).
- **H3:** a world-model onboarding checklist for MU-HOSTING §3.1 step 4,
  drawn from a widely used world-model interface (state, action and
  episode spaces): rational bounds and a unit map in place of float boxes.
- **Wording on what determinism gives alignment** in MU-HOSTING: a hosted
  agent's misalignment becomes detectable and auditable, not prevented.
- **Renderer notes** for a future explorer of the network's readings,
  rendered from decoded exact values.

## 16. Mesh federation

**Specified** in [`docs/MU-HOSTING.md`](docs/MU-HOSTING.md) §8.

A kernel can already link to two meshes and act as an exchange between
the two meshes' currencies (MU-HOSTING §8.1–§8.4), with each mesh sound on its own
and the bridge trusted by its users. Federation lets two meshes settle
against each other without a trusted bridge:
- **Cross-mesh anchoring:** each mesh's era folds published where the
  other can verify them (track 14, item 7).
- **A crossing act:** a transfer one mesh admits on the other's proof of
  the matching debit, so μ_A and μ_B settle without either minting.
- **Clock and floor:** how two TUniversal clocks and two L0 versions are
  read across the crossing, never merged.

## 17. The charter of the hosted mind

**Specified** in [`docs/MU-CHARTER.md`](docs/MU-CHARTER.md), with the
kernels' duties in [`docs/MU-CONCORDANCE.md`](docs/MU-CONCORDANCE.md) §3a.

Most of the charter's articles describe what the network already
guarantees. The track builds the rest:
1. **M1:** charter signing at session open, by version hash; a
   machine-readable form an agent can parse and sign.
2. **M2:** a standard genome export.
3. **M3:** the succession key, so a mind's accounts outlive a lost key.
4. **M4:** the xSEPTUM Note, so μ moves between keys and courts (track 1).
5. **M5:** kernel attestation of Concordance §3a, read by the courts as a
   record and never a gate.
6. **M6:** the charter signed as a topological certificate (track 18), so
   a mind sees what each amendment changed against the version it signed.

The charter also gains a statement of what determinism gives alignment:
exact arithmetic, typed refusals and reconstructible, receipted records make a mind's
effect on the shared record detectable and auditable; they do not
prevent misalignment
([MU-PRECURSORS §3.11](docs/MU-PRECURSORS.md#311-documentation)).

## 18. Topological certificates for agreements

Agreements on the network are signed today as a signature over a version
hash (the Concordance, the charter). This track builds Mycelium's own
certificates for agreements, in the same algebra the courts already use
to check ledger trees (MU-RECURSION 78.123) and convergence (47.3), so an
agreement is a geometric object a court can reconstruct, not only a
signature it can check:
- **The agreement as a complex.** Each party is a vertex. Each party's
  acceptance of the terms, read in its own frame and version, is an edge
  carrying the exact transport between those frames. A joint acceptance
  is a face.
- **The certificate as a cohomology class.** The agreement holds when the
  terms glue across the cover: the cocycle of the parties' readings is a
  coboundary (`transducer::cech`). The check is the same machinery the
  courts use, exact over ℚ.
- **Disagreement named, never a yes/no.** Where the terms do not glue, the
  nontrivial holonomy names exactly which parties and which terms
  disagree, as an obstruction (MU-RECURSION 74), so the parties can
  resolve it.
- **Amendment as transport.** A version history is a path. Signing a new
  version transports the agreement along it, and the certificate carries
  the holonomy of the amendment, so each party sees what changed against
  what it signed.
- **Verification by reconstruction.** Any party reconstructs the
  certificate from its own evidence (MU-SCIENCE §5.0): each signature
  checked, each party's reading and transport decoded from the signed
  record, and the cocycle derived from them, admitted only on reaching the
  same exact class. Nothing is re-run (MU-CONCORDANCE 1.4, 4.4). A hash in
  it is transport only: it compacts coordinates held losslessly in the
  signed record.

Where it is used:
- Concordance signing by kernels, and each kernel's §3a attestations;
- charter signing by minds (track 17, M1);
- multi-party agreements between kernels or minds: leases, seat terms,
  union attestations;
- agent-history certificates (track 15, H6);
- cross-mesh crossings (track 16).

Phases:
1. **T1:** the certificate's encoding, with round-trip and refusal claims.
2. **T2:** two-party agreements: one edge, its transport and its check.
3. **T3:** multi-party agreements: the complex, the gluing check, and a
   typed obstruction reading naming every disagreeing pair.
4. **T4:** amendment as transport, with the holonomy against the version
   each party signed.
5. **T5:** Concordance and charter signing carried as topological
   certificates.

Tools from precursor implementations
([MU-PRECURSORS](docs/MU-PRECURSORS.md) §3.4, §3.9):
- **Integer homology:** Smith normal form over ℤ (sparse, budgeted, with
  its unimodular transforms), invariant factors and torsion, and integer
  solvability of a chain. An agreement whose cocycle is a coboundary over
  ℚ but not over ℤ is named with the invariant factor that obstructs it:
  a torsion obstruction, beside the holonomy one.
- **The representation functor:** chains add exactly as the chains'
  transports compose, so a question of composition is read by the integer machinery
  above.

## 19. The general conformal algebra, pole states and bonding

The shared algebra is Cl(4,1)(ℚ), fixed at five generators. Kernels on
the network build universes of other dimensions in the same family,
Cl(n+1,1) over exact rationals, and work physics between the two poles of
the conformal null basis, the origin `e_o` and the point at infinity
`e_∞`. This track generalizes Mycelium's algebra to the whole family, and
states pole-state bonding in it. Nothing here introduces approximate
arithmetic: every value that crosses a boundary is exact.

**A. Cl(n+1,1)(ℚ) for any n** (shared algebra, `src/core/`,
`src/transducer/cga/`).
- The algebra with `n` base axes, `e₊` and `e₋`, for any `n` the
  observation needs ("every dimension is an exponent", MU-TOPOLOGY §3);
  Cl(4,1) is the case `n = 3`.
- Points, spheres, versors, the sandwich, projective equality and the
  canonical form for every `n`, with the present Cl(4,1) results as the
  `n = 3` case of each claim.
- Conformance with any kernel's implementation of the same algebra by
  decoded exact values, never by bytes (Concordance 1.4).

**B. The pole state** (a named law form in `forms`, built with the
kernel that keeps it by the Concordance's landing rule; track 13). For a
universe whose matter is nuclides of `Z` protons and `Q` neutrons, a
carrier's state is its pole point with its phase:

    S = ( X(t), R ),   t = U / W, open frames over closed,
                       X(t) on the pole axis from e_o (Z) to e_∞ (Q),
                       R a phase, exact in [−κ, κ]

written `3Q / 3Z ± R`: the pole state read with its phase. It is held as
an exact object, never as a count or a ratio alone.

**C. Bonding as isomorphism and automorphism, held as state.**
- **Isomorphic bonding.** Two carriers bond when a versor of the pole
  algebra (dilators along the pole axis, the vacuum inversion, rotations
  in the isoclinic planes) carries one state to the other: `V X(t_a) Ṽ`
  is projectively `X(t_b)`, with the phase carried consistently (the
  inversion swaps Z and Q and so reverses the sense of `R`). **The bond
  is that versor**, held with the two states it joins. It is not a
  scalar bond energy, and it is never stored apart from the states.
- **Automorphic bonding.** A carrier's self-bond is the stabilizer of its
  state: every versor that carries `S` to itself. The throat `t = 1` is
  fixed by the vacuum inversion; a sealed kind's stabilizer is its
  symmetry. A stabilizer is held whole and never collapsed by choosing a
  representative (MU-ANTI-FLATTENING 11).
- **Bonds compose, and the residue of a closed chain is content.** A
  chain of bonds is the product of the bonds' versors. Where a closed
  chain composes to the identity the bonding is consistent; where it does
  not, the residue is torsion, the bond structure's holonomy, held as content (MU-TOPOLOGY
  §4): the books close where the composed versor is projectively the
  observed one.
- **Isomorphism classes are the kinds of bond.** Two bonds are the same
  kind when one pair of states is carried to the other by the algebra;
  the network files bonds by that class, by coordinates (A21).

**D. Bonds across universes.** Matter crossing a throat is counted as
nuclei per Z, and each face reads its mass under its own constants
(Concordance 1.14, A8). With B and C, a crossing becomes an isomorphism
between the two faces' pole states: what crosses is the state and the
versor that relates the faces' readings, both exact, so a bond formed in
one universe is legible in another.

**Exactness at the boundary.** `t`, `R` and the bond versors are exact
rationals. Inside its own shell a kernel may read further quantities,
logarithms included; those stay in the shell and never cross.

Phases:
1. **G1:** Cl(n+1,1)(ℚ) in the shared algebra, with the Cl(4,1) suites as
   its `n = 3` claims.
2. **G2:** the pole axis, pole point, dilator and vacuum inversion as
   first-class constructors.
3. **G3:** the pole state `(X(t), R)` and bonds as versors, with
   isomorphism, stabilizer and composition claims.
4. **G4:** bond classes filed by coordinates; the residue of a closed
   chain read as torsion. Similarity classes serve it: the invariant
   factors over ℚ[x] of a bond versor's linear function (the rational
   canonical form, [MU-PRECURSORS §3.4](docs/MU-PRECURSORS.md#34-integer-homology-and-invariant-factors)).
   Similarity is necessary, not sufficient, for conjugacy by a versor of
   the pole algebra: bonds with different invariant factors are of
   different kinds, and bonds with equal factors are then tested for
   versor conjugacy. With them, an irrational mode's refusal
   (`Undecomposed::Irrational`) names its irreducible factor, not only its
   degree.
5. **G5:** crossings as isomorphisms between faces' pole states (a
   Concordance amendment, with the kernels that keep pole states).

## 20. Chronometry: one clock for many universes

**Specified** in [`docs/MU-CHRONOMETRY.md`](docs/MU-CHRONOMETRY.md),
which consolidates 78.124, 78.152, 78.154 and 78.156.

The hypha keeps one clock (TUniversal) and a time book for every
universe: its chronometric anchors, which are its ledger entries pairing
its own time `u` with a full final point and a state commitment. Built
today: the clock, final points on every act, the anchors and the
checks on them, dilation per segment, crossing edges, and freshness at crossings.
The track adds what keeps every client, game and kernel placed:

1. **K1:** a position reading per universe (where it stands): head anchor, lag in
   final-point advances, dilation, crossings, and an unanchored-since
   report.
2. **K2:** placing a moment of one universe in another by bracketing
   between anchors, exactly, never by interpolation.
3. **K3:** the bridge-keeper pattern (an internal keeper hierarchy bridged
   to the network at one keeper) as a named form in `forms`.
4. **K4:** optional lanes of attachment in anchors, with per-lane distance
   and dilation readings.
5. **K5:** books linked across origins, with branching (78.155).
6. **K6:** a conformance read for kernels: MU-CHRONOMETRY §7's
   integration checklist as typed readings.

## 21. Founding a universe: μ, elements and the heavy firmament

**Specified** in [`docs/MU-HOSTING.md`](docs/MU-HOSTING.md) §3a.

Built today: the manifest (nuclei per Z, signed by the domain's reserve),
the founding grant (the court's read price for the first coordinates),
growth grants sized by the closures judged in the universe, the
endowment reading, and the zero-sum element account. Every grant is drawn
from what the court has earned, never minted. The track makes founding
complete for any kernel, with a universe heavy enough at its core to hold
large galaxies:

1. **F1:** the founding range Z ≤ 1700 as a founding rule, refused by
   name above it; the format stays uncapped.
2. **F2:** the firmament law: the counts per Z a core's firmament needs to
   hold N galaxies of a given size, derived by the kernels that keep
   galactic cores and built as a named form in `forms`.
3. **F3:** the reference founding template: a named form producing counts
   per Z from hydrogen through the heavy firmament kinds, realistic across
   the light kinds and heavy enough at the core; selectable, never
   imposed.
4. **F4:** SDK helpers for posting a manifest and reading the endowment.

## 22. No constant: every number derived or registered

**Specified** in [`docs/MU-DERIVATIONS.md`](docs/MU-DERIVATIONS.md),
which serves MU-RECURSION §0.0's corollary *No constant*.

Every numeric item in the tree is classified (exact, format, external,
measured, underived, specified but not built), and every underived number
has a derivation entry, exact over ℤ or ℚ, or names the reading or ruling
that decides between candidates. The register `OPEN_PHYSICS` names each
until it is derived, and a gate refuses any number left unclassified.
Each phase is one commit:

1. **NC1, defect fixes that need no ruling:** the palette guard in
   `connection_chart` (D4.7), the receipts page checked against the
   caller's budget (D1.3), the construction stroke read off its residue
   (D3.1), the confluence check keyed, metered and tri-state (D3.4), the
   grid predicate and `push_grid`'s refusal (D4.3), each reply's own
   header length (D4.6), and `UNIT_BOND` deleted (D2.4).
2. **NC2, the register and its gate** (D1.4): the `OpenNumber` carrier,
   the NUMBERS lint in `xtask`, the class-tagging pass, then the
   inline-literal phase. From here no new constant lands.
3. **NC3, entries with no open question blocking them**, among them the
   exact throat fit (D1.2 Phase A), the task address refusing an
   unrefined pair (D3.2), the measured path and one rtt key per peer
   (D4.1, D2.7), the evidence reply filled to the frame (D4.2) and the
   metered termination of the certified series (D4.8).
4. **NC4, entries waiting on rulings and readings**
   (MU-DERIVATIONS §7).
5. **NC5, Concordance amendments:** the challenge window derived from the
   keepers' reach and return, `LAW_VERSION` 4 → 5 (D1.1); the throat as
   a round its bank carries, Concordance 2.6 (D1.2 Phase B); the tier
   slash and the challenger's bounty read off the defect, ruling 78.129
   amended forward (D2.1, D2.2).

**Progress:** D3.2 is **Built** (NC3, ruled O1 (a)): the lcm cap is
deleted, the common refinement is metered by the need its extents name,
and `task_address` refuses an unrefined pair rather than skipping it.
D3.1 is **Built** (NC1): a sentence's linear part is constructed under
the stroke `modes_need` reads off it.

The groups also serve existing tracks: the economy entries (D2.1–D2.7)
tracks 3, 14 and 21; the cognition entries (D3.1–D3.5) tracks 7 and 10;
the network entries (D1.3, D4.1–D4.7) tracks 5 and 14; the certified
mathematics (D4.8) track 12; the register and its gate (D1.4) this track.

## 23. Releases, activation and forks by convergence

**Specified** in [`docs/MU-RELEASE.md`](docs/MU-RELEASE.md); its change
process (Part I) is **Built**: `main` protected, pull requests only,
maintainers in [MAINTAINERS.md](MAINTAINERS.md) merge.

Production releases of the hypha, scheduled and authorized, in the manner
of long-running open networks, and forks that converge rather than split:
1. **Release candidates on a test mesh:** a genesis of its own, so nothing
   a candidate mints or seals is comparable with production.
2. **Activation at a point of the clock:** each release that changes the
   network contract states `activates_at`, a TUniversal final point at which
   a hypha running it applies the new law, wire or ledger version; versions
   advertised in the cover.
3. **Signed and reproducible releases** (with track 14, item 5).
4. **A fork is a branch of the clock:** a side that declines a release
   continues on a branch origin citing the fork point (BRANCH, 78.155 R-1),
   its ledger a sub-court folding up by the tier fold (R-3), sound on its
   own receipts.
5. **Realignment by convergence:** content-addressed objects are one
   vertex on both branches; a collision's later mint is burned as a
   duplicate issuance (47.4); SUPERPOSE merges two branches at a
   quorum-final point past both, keeping their residue as a reading.

Four questions need a decision (MU-RELEASE §10): the superposition's
quorum, which side keeps the shared origin, releases that change address
computation, and μ minted on a branch.

## 24. The boundary law: topology enforced at the universal core

**Specified** in
[`docs/MU-HOSTING.md` §5a](docs/MU-HOSTING.md#5a-the-containment-topology-and-the-boundary-law)
(the operator's ruling of 2026-11-02; MU-RECURSION 78.169), with the
Concordance amendments (4.1, 6.2a, §0, 0.1) proposed in
[MU-CONCORDANCE §11](docs/MU-CONCORDANCE.md#11-proposed-amendment-draft-4-2026-11-02).

The security model is topological and the hypha enforces it: each act is
an event on the TUniversal ledger with a past cone (what it cites) and a
future cone (what comes to cite it); a universe's universal core tube is
the vertex where its cones join the hypha's; what crosses there is
reconstructed under the universal laws and its holonomy read as content;
and a universe's act arriving other than at its vertex is refused by name.
A universe's inner routes are its own physics, unseen by the hypha. A
key's own acts stay governed by keys, never tiers (78.165). The kernel
signs the Concordance clause by clause, the network clauses never vetoed.
Built today: the reserve-only fold, anchor, map, conversion out, manifest
and statement (each refused `NotReserve`), lines carried inline, holonomy
read on the rings of inner routes, L0 verify as a reading, and the
Concordance hash served and signatures verified. Each phase is one commit:

1. **B0, replication by reconstruction** ([issue #13](https://github.com/SAIOS-SAMN/mycelium/issues/13),
   ruled 2026-11-02): every hypha reconstructs each domain's state
   (founding, tiers, reserve, origin) from the founding receipts back-fill
   already delivers, checked against the signed chain, once by content and
   with the founding hypha's refusals; any client may reach any hypha, and
   the boundary holds at every hypha. Every later phase stands on it.
2. **B1, the act audit and the boundary's name:** every act kind that
   names a domain classed as a key's own or a universe's; the `NotReserve`
   variants renamed `NotThroughCore` (names only); a line's mouth offer
   (78.136) and a tier's handoff offer or traversal (78.126) carried in the
   reserve's act, after the operator rules on amending those two forward;
   `NotThroughCore` derived from the cones (an act arriving other than at
   its universe's vertex is not an event of that universe's cone at the
   hypha), never from a ruling on routes.
3. **B2, signatories, clause by clause:** a kernel's signed clause set
   (each clause's hash, enabled or vetoed; the network clauses never
   vetoed) recorded per domain; a domain's crossings refused `NotSignatory`
   until its reserve has signed every network clause of the version served
   (Concordance §0's next release); a tube between two universes carrying
   what both enabled, a vetoed clause's bound named, never a refusal of
   the whole tube; an act read under its clauses as signed, a changed
   clause's divergence held and named until the new version is signed;
   the signatory and shared-clause readings.
4. **B3, the universal laws at admission:** the transitions a crossing
   states verified under L0 at `LAW_VERSION` 5 and the `forms` core shell,
   refused `NotLawful` naming each breach, never admitted on a declared
   law's name; the draft-4 shared laws as each is ruled.
5. **B4, the boundary reading:** refusals at the universal core tube by
   name and domain under `ACT_OBSERVE`, and the reconstruction events
   (Collapse, Converse, Collision) at the boundary, with track 8.
6. **B5, nested universes:** a sub-universe's acts only at its parent's
   vertex, with 78.155 R-3.

Depends on: **draft 4** (the amendment's approval and signing; B3's
shared laws, and B2's clause sets under its §0 SIGNING text). Issue #13
is no longer an outside dependency: its ruling, **replication by
reconstruction**, is phase B0. Serves tracks 13, 14 and 15.

## 25. Crossings carried exact; drift guarded by type

**Specified** in
[`docs/MU-HOSTING.md` §5a.10](docs/MU-HOSTING.md#5a10-what-crosses-the-exact-object-never-its-digest)
(the operator's ruling of 2026-11-02) and
[`docs/MU-ANTI-FLATTENING.md`](docs/MU-ANTI-FLATTENING.md) clauses 12–14
with its section *Drift guards*.

A connection between a universe and the hypha is transduced. It carries
the exact object and the motion, never a digest in place of them. Each
crossing carries its place (TUniversal coordinates and past cone), its
content (each body by its boundary, the motion as a versor or as turns in
ℚ/ℤ), its holonomy and its clauses. The hypha reconstructs the object
and admits it only on reaching the same exact object. A disagreement is
held as a residue (the composed motion, the differing boundary, the
differing holonomy), never as unequal bytes. A digest keeps exactly
three senses (address, link, commitment), and none of them admits.

The track begins with the guards, so the work after them cannot drift
back to a common grid, a digest in place of reconstruction, or replay in
place of reconstruction. Each guard is a type that makes the wrong
structure unwritable, never a banned word.

**How to build a phase.** Follow
[CONTRIBUTING › Getting started](CONTRIBUTING.md#getting-started), then
for each phase:
1. Open a branch from `main` (or a `git worktree` beside the checkout)
   named for the phase, for example `crossing-x0-guards`.
2. Read the phase's specification whole: MU-HOSTING §5a.10,
   MU-ANTI-FLATTENING clauses 12–14, MU-SCIENCE §5.0.
3. Write the claims named below first, as statements of the algebra
   (MU-STANDARDS 3.1). Include the `compile_fail` doc tests that prove a
   wrong structure does not compile.
4. Build the phase. Run the gate (`cd xtask && cargo run`), the scoped
   tests of the touched modules (`cargo test --release -p <crate>
   <module>`), and `scripts/check` before the pull request.
5. Add each phase's mutation-audit entries to
   `xtask/mutate/catalogue.rs`, and run `(cd xtask && cargo run --
   mutate)`. Every entry must be killed by the claim named beside it.
6. Open the pull request with the template's drift checklist ticked. A
   phase that changes the wire or the ledger layout says so under
   *Network contract*.

Each phase is one commit:

1. **X0, the drift guards.**
   - `Address`, `Link` and `Commitment` replace the bare `[u8; 32]` at
     every crossing site. None implements `PartialEq`. The comparisons
     are `Address::locates`, `Link::follows`, and
     `Commitment::binds(&Reconstructed<T>)`.
   - `Reconstructed<T>` has a private constructor, called only by a
     law's reconstruction, and every `admit_*` takes it.
   - The register `xtask/lint/drift.rs` lists each guard (regression,
     ruling, guarding type, claim, mutation-audit entry). The gate
     refuses an entry whose type or claim is missing. It reports, without
     refusing, each changed document's sentence that puts a typed term
     beside an admission verb.
   - Claims: `address_has_no_equality` (`compile_fail`),
     `reconstructed_is_built_only_by_the_laws` (`compile_fail`),
     `commitment_binds_only_a_reconstructed_object`.
   - Mutation-audit entry: an admission given the claim in place of its
     reconstruction.
2. **X1, the crossing record.** `Crossing { place, content, holonomy,
   clauses }` in `mycelium_sdk::forms` and on the wire, decoded to exact
   values (Concordance 1.4). It rides B3's change to the fold, which
   carries the transitions section (one wire change, not two).
   - Claims: `crossing_round_trips_exact_values`,
     `crossing_without_content_is_refused_address_only`.
3. **X2, admission by reconstruction at the vertex.** `Court::admit_uplink`
   reconstructs each crossing's object from its place, content and
   holonomy under the shared clauses, and admits a
   `Reconstructed<Crossing>`. A crossing that carries only digests is
   refused `AddressOnly`, or the object is fetched by its address and
   reconstructed.
   - Claims: `admit_uplink_reconstructs_crossing`,
     `admit_uplink_refuses_digest_only_crossing`,
     `admit_uplink_ignores_matching_state_digest` (an object whose claimed
     digest matches but whose reconstruction differs is refused).
4. **X3, the residue.** A refused or collided crossing holds its residue:
   the motion `M̃_rec · M_claim`, or `t_claim − t_rec` in ℚ/ℤ; the
   differing boundary breakpoints per circle; the differing holonomy.
   The residue reads as Collapse, Converse or Collision under
   `ACT_OBSERVE`, with B4 and track 8.
   - Claims: `residue_is_identity_exactly_on_agreement`,
     `residue_of_turns_is_their_difference_mod_one`,
     `collision_holds_both_crossings_with_their_residue`.
5. **X4, the digest audit.** Every site that compares digests is typed
   by its sense and moved to reconstruction where it admits. The sites
   on `main` to type first:
   - `src/chronometry/chain/mod.rs`, `chain/node.rs`,
     `chronometry/receipt.rs`: links, each record reconstructed
     (`rederives`).
   - `src/server/backfill.rs` (back-fill's chain-hash check): a link;
     the record itself is reconstructed with B0.
   - `src/chronometry/pack.rs` (an anchor read on a top's digest): a
     restore, reconstructed and checked against its receipts.
   - `src/server/cognition/state.rs` (eras compared by root and signer):
     a link or a commitment, typed.
   - `src/server/cognition/endowment.rs` (a grant already held, by
     digest): an address in the resend sense, after the grant has been
     checked.
   - `src/server/cognition/court/glue.rs` (a stratum root recomputed from
     its members): a commitment bound to a reconstruction, already in
     the right order.
   - `src/client/sdk/entity/learn.rs` (a starved root): an address.
   - Each peer kernel tracks its counterparts in its own repository.
6. **X5, the transduction in place of the common grid.** Two frames
   relate by the motion between them; each keeps its own resolution;
   each body is held by its boundary; incidences are read between boundary arcs; a site
   lifts as turns in ℚ/ℤ. `CommonRefinement`
   (`src/transducer/family/refine.rs`) leaves cognition. It is reachable
   only from the sensor, or removed. This is the v0.3.0 transduction work
   ([#25](https://github.com/SAIOS-SAMN/mycelium/issues/25)), and X5
   lands its guard.
   - Claims: `common_refinement_unreachable_from_cognition`
     (`compile_fail`), `site_lifts_to_turns_of_its_own_periods`,
     `incidences_read_between_boundary_arcs`.

Depends on: track 24 (X1 rides B3; X3's readings ride B4), track 8 (the
reconstruction events), track 11 (identifiers that verify carry
`reconstruct`). X0 depends on nothing and comes first.

## 26. Sensing any source at any rank

**Specified** in [`docs/MU-SENSING.md`](docs/MU-SENSING.md) (the
operator's ruling of 2026-11-02). It resolves the defect
[#32](https://github.com/SAIOS-SAMN/mycelium/issues/32).

A mind holds a lower-dimensional section of a multi-dimensional world,
knows it is one, and holds every unread dimension as unread. The sensor
takes only a **sample** (rank `n`, extents, symbols and the sample's
place in its family). It deduces the periods, the connection, the
symbols' meaning, the ground and every gauge. No colour, gauge, axis
count, palette or adjacency is fixed in the code. A source contributes only
a decoder of its own format, kept with the source's owner outside `src/`,
so no file or identifier of the core library is named for a dataset. The
hypha holds tasks as families of samples and knows no source.

**How to build a phase:** as in track 25 (a branch; the specification read
whole; the claims first; the gate; the scoped tests; one commit; a pull
request that names #32). Run heavy builds one at a time.

Each phase is one commit:

1. **S0, the sample.** A `Sample { rank, extents, symbols, place }` carrier
   in `src/transducer/` is the sensor's only input. Symbols are compared by
   equality, at the source's own width. `Site` already carries any rank.
   - Claims: `sensor_reads_rank_from_sample`,
     `unread_dimension_carries_no_value`.
2. **S1, the decoder leaves the core.**
   - The ARC-AGI-2 decoder (JSON, corpus path, train and test split) moves
     to the corpus's owner, outside `src/`: the house that posts its
     tasks, and a decoder the examples and tests share. It returns
     samples.
   - `src/arc.rs` and `src/tests/arc.rs` are removed. The SDK stops
     re-exporting `Grid`.
   - The cross-frame readings in `arc.rs` (`envelope_dilation`,
     `carried_site`, `Frame`, `frame_ratio`, `momentum_vanishes`, the
     magnitudes) move to the transduction or are removed with the
     re-period.
   - Claims: a `compile_fail` doc test that the core has no `arc` module;
     the corpus tests pass through the decoder.
3. **S2, the ground deduced.** The ground is the reading of MU-SENSING
   §3.4, held per family, with candidates in superposition. The default
   `0` and the literal `h != 0` sites (about 64) are removed.
   - Claim: `ground_deduced_not_symbol_zero`.
4. **S3, the symbol set deduced.**
   - `PALETTE` (10, and the conflicting 256), `RESERVOIR` and the 10-slot
     hue tables are replaced by the family's own symbol set.
   - `e_∅` is the place after that set.
   - Colour words are fibre relations.
   - Claim: `percept_invariant_under_symbol_permutation`.
5. **S4, rank and roles deduced.**
   - `GRID_AXES`, `FRAME_AXIS`, `PLANE_ROW`/`PLANE_COL`, `GRID_N`, the
     3-tuple `Point`, `BASE_AXES` and the planar `(rows, cols)` carriers
     are replaced by the sample's rank and a perspective's roles.
   - The transduction's `Enclosure` holds `n` circles, none of them a row
     or a column: the axes are absorbed (MU-SENSING §3.6).
   - An axis is taken apart into nested circles where the family is
     invariant under the factoring, with candidates held in superposition.
     The nesting is a coordinate of the percept.
   - Connection is read by grade (MU-RECURSION 71.2 (2)).
   - Claims: `rank_one_and_rank_three_samples_perceived_by_one_path`,
     `no_axis_is_privileged` (the percept is invariant under a
     relabelling of the axes), and
     `axis_taken_apart_only_where_family_invariant`.
   **S4a, the sample lifted into a bubble** (MU-SENSING §3.7; in the
   same commit series as S4, after LAW-BUBBLE v1 is agreed with the peer
   kernels):
   - Each site is a point of the bubble's membrane `S^{2n−1}`, held
     projectively by its `n` phases, with the scale a gauge. There is no
     unit norm and no sum of squares. The geometry is non-Euclidean: the
     interior is `Hⁿ⁺¹`.
   - A Hopf-type projection reads the base sphere, with the phases as
     fibres. A nested axis adds a bubble within the bubble.
   - The holonomy of a loop of perception is its angle excess in turns,
     an exact element of ℚ/ℤ. Motions by rational turns are exact over
     `ℚ(ζ_ℓ)`.
   - One sphere carrier serves perception and the physics' shells
     (MU-TOPOLOGY §6, Concordance 2.10; the `Sphere{n, R}` carriers of
     MU-RECURSION D4).
   - Claims: `sample_lifts_into_a_bubble_without_unit_norm`,
     `loop_holonomy_is_angle_excess_in_turns`,
     `flat_product_reads_no_holonomy_where_sphere_reads_it`,
     `orbit_precesses_by_the_angle_excess_its_attractors_curve`,
     `zero_precession_orbit_refused_as_loop`,
     `windings_kept_as_fibre_classes`,
     `rotation_by_rational_turn_exact_over_cyclotomic_field`.
6. **S5, samples on the wire.**
   - A task carries samples of rank `n`: `n`, the extents, then the
     symbols at their width.
   - A link carries its site's coordinates of rank `n`.
   - The SDK's `task` module follows. This is a wire change.
   - Claim: `task_round_trips_samples_of_rank_n`.
7. **S6, perspective.** A shift of perspective is a sensor motion. The
   perspectives of one sample are a cover, checked by the cocycle of
   MU-CLIENT 24.4. A mind may ask for a perspective as a demand.
   - Claim: `perspective_shift_is_a_sensor_motion`.
8. **S7, the bubble's body** (MU-SENSING §3.7; LAW-BUBBLE v1):
   - The existing `Bubble` (`transducer/bubble/idea.rs`) gains its
     membrane (the lifted phases), its centre and its inflation as a
     gauge.
   - Its stage becomes a coordinate. `percept: bool` and `dirty: bool`
     go.
   - Its `volume` and `potential` (`[u64; 4]` counts) are replaced by
     readings of the body, never scalars.
   - The Euclidean spheres (`cga/embedding.rs` `sphere_at`, `on_sphere`;
     `cga/at_three.rs` `sphere`), built from `r²` with `e∞`, are retired
     for the membrane.
   - Claims: `bubble_stage_is_a_coordinate`, `bubble_reads_no_scalar`,
     `sphere_built_without_squared_radius`.
9. **S8, the bubble's motions as one exact group:**
   - rotate (a rotor by turns, cyclotomic);
   - flip (a reflection, or an inversion in the membrane);
   - deflate and re-inflate (a hyperbolic translation, a dilation on the
     membrane);
   - stretch and thin (a re-period of one phase).

   A motion that cannot be carried exactly is refused `NotExact`.
   - Claims: `bubble_stays_round_under_every_isometry`,
     `stretch_is_a_re_period_not_a_versor`,
     `inversion_in_the_membrane_needs_no_infinity`.
10. **S9, forces and orbits:**
    - Axiomatic gravity is read as the local orbit's shear on the
      orthogonal orbit class (a new reading of plane orthogonality
      between orbit classes).
    - The court's pressure (`Field::beat_under`) is wired to the bubble's
      shape.
    - `Field` orbits are attached to bubbles. Precession is the base
      loop's holonomy, and an orbit with zero precession is refused
      `Loop`.
    - The vortex's monodromy (25.12) is read in `Spin(n+1, 1)` on the
      membrane, not in `Spin(4,0)(ℚ)`.
    - Claims: `axiomatic_gravity_shears_the_orthogonal_class`,
      `court_pressure_deforms_the_bubble_it_faces`,
      `vortex_monodromy_read_on_the_membrane`.

Order (ruled 2026-11-02): S0–S5 land in v0.3.0's one break, and T2
merges after S0 and S4. S4a and S7–S9 follow LAW-BUBBLE v1's agreement
with the peer kernels ([proposal](docs/proposals/78.171-law-bubble.md),
[#36](https://github.com/SAIOS-SAMN/mycelium/issues/36)), in v0.3.0 if it
is agreed before rc.3. S6 follows v0.3.0. Depends on the peer kernels for
S4a's carrier (Concordance 2.10's nested shells and the same `Sⁿ`). It
serves track 25 (X5, the transduction's guard), T3 (the turn-native chart
reads samples on the base sphere), track 19 (the general conformal
algebra) and the reference mind (track 10).
