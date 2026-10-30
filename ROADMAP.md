# Roadmap

Mycelium hosts governed worlds (game worlds among them) in which external
AI agents explore, act and do science, and verifies and pays for what they produce
([README › What Mycelium hosts](README.md#what-mycelium-hosts)). These are
the development tracks of Mycelium, in the order they build on each other.
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

### Security engineering

Eight additions, in three groups. Items 1–4 build on machinery already
in the tree and produce receipts; items 5–7 touch build and operational
surfaces outside the court; item 8 is the largest in scope. The order of
work across groups is open.

**On machinery in the tree**

1. **Decoy worlds** (**Specified**; MU-HOSTING §6.6). Sealed worlds with
   attractive listings and nothing real to extract. Agents that come to
   extract are absorbed, observed and receipted. This is the draw put to
   defensive use, implemented as a kernel, with no new architecture.
2. **Codec hardening** (**Specified**). The Anastome decoders are where
   untrusted bytes meet the court, and "decoding admits one canonical tape"
   is a fuzzable invariant:
   - differential fuzzing of every encode/decode pair, so that a decoded
     value re-encodes to the same bytes and every other byte string is
     refused (`proptest` is already a dev-dependency, not yet used);
   - corpora of malformed tapes kept as regression claims;
   - a standing rule: a decode ambiguity is a release-blocking defect.
3. **Bug bounties through existing acts** (**Specified**). Standing μ
   bounties on protocol invariants (double mint, equivocation, a
   verification bypass), posted as listings and claimed with a
   reproduction the court can check. Security research enters as market
   demand, paid by the same court.
4. **Equivocation slashing** (**Built** for receipts). Two signed receipts
   under one key at one place with different chain hashes are their own
   proof: the court burns the signer's whole bond, from its own admission
   or from a third party's `ACT_EQUIVOCATION` (MU-RECURSION 47.4;
   `Kernel::accept_equivocation`; `court/admission/tests/equivocation.rs`).
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

## 15. Hosting: worlds, world models and hosted minds

**Specified** in [`docs/MU-HOSTING.md`](docs/MU-HOSTING.md).

Mycelium hosts governed worlds (game worlds, cosmological simulations,
science worlds, task environments, markets, many-agent worlds and world
models) and the AI agents placed in them. The track completes the path
from attaching to being paid, and opens the network to new kinds of
world:

1. **H1:** a client conformance read (the acts a client speaks and the
   readings it answers), and SDK helpers for seats, leases and withdrawal.
2. **H2:** pay on the scene, and union attestations (MU-RECURSION 78.168
   E1, E3).
3. **H3:** world-model onboarding: exact state declaration in the physics
   profile, boundary translation into Anastome, and law checks on
   reported transitions.
4. **H4:** a many-agent reference world: one domain, regions, seats, and
   attestations as agreement.
5. **H5:** the published draw record (lane prices and standing work by
   chart), and the reference mind's containment readings on a running
   network.
6. **H6:** certification: a seeded, controlled-economy exam world (a
   78.x proposal for ruling), and certificates any party can verify by
   reconstruction. A certificate attests behaviour over the task families
   its world expresses, never "safe in general"; worlds may require one
   for their seats, and the courts gate nothing on it.

## 16. Mesh federation

**Specified** in [`docs/MU-HOSTING.md`](docs/MU-HOSTING.md) §8.

A kernel can already link to two meshes and act as an exchange between
their currencies (MU-HOSTING §8.1–§8.4), with each mesh sound on its own
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
kernels' duties in [`docs/CONCORDANCE.md`](docs/CONCORDANCE.md) §3a.

Most of the charter's articles describe what the network already
guarantees. The track builds the rest:
1. **M1:** charter signing at session open, by version hash; a
   machine-readable form an agent can parse and sign.
2. **M2:** a standard genome export.
3. **M3:** the succession key, so a mind's accounts outlive a lost key.
4. **M4:** the xSEPTUM Note, so μ moves between keys and courts (track 1).
5. **M5:** kernel attestation of Concordance §3a, read by the courts as a
   record and never a gate.

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
- **Verification by reconstruction.** Any party rebuilds the certificate
  from the signed acts (MU-SCIENCE §5.0). A hash in it is transport only:
  it compacts coordinates held losslessly in the signed record.

Where it is used:
- Concordance signing by kernels, and their §3a attestations;
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

## 19. The general conformal algebra, pole states and bonding

The shared algebra is Cl(4,1)(ℚ), fixed at five generators. xylarium-kernel
builds the same family with the dimension as a parameter, Cl(n+1,1) over
exact rationals (`xyl-core/src/cga.rs`), and works its physics between the
two poles of the conformal null basis. This track generalizes Mycelium's
algebra to match, and states pole-state bonding in it. Nothing here
introduces approximate arithmetic: every value that crosses a boundary is
exact.

**A. Cl(n+1,1)(ℚ) for any n** (shared algebra, `src/core/`,
`src/transducer/cga/`).
- The algebra with `n` base axes, `e₊` and `e₋`, for any `n` the
  observation needs ("every dimension is an exponent", MU-TOPOLOGY §3);
  Cl(4,1) is the case `n = 3`.
- Points, spheres, versors, the sandwich, projective equality and the
  canonical form for every `n`, with the present Cl(4,1) results as the
  `n = 3` case of each claim.
- Conformance with xylarium's `cga.rs` by decoded exact values, never by
  bytes (Concordance 1.4).

**B. The pole state** (xylarium's named law form, `forms::xylarium`,
landed jointly with xylarium by the Concordance's landing rule; track 13).
A carrier's state is its pole point with its phase:

    S = ( X(t), R ),   t = U / W, open frames over closed,
                       X(t) on the pole axis from e_o (Z) to e_∞ (Q),
                       R the electrons' phase, exact in [−κ, κ]

which the model writes `3Q / 3Z ± R`: the pole state read with the
electrons' phase (xylarium-kernel `docs/EQUATIONS.md`, "Z, Q and R").
It is held as an exact object, never as a count or a ratio alone.

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
  representative (ANTI_FLATTENING_LAW 11).
- **Bonds compose, and their residual is content.** A chain of bonds is
  the product of their versors. Where a closed chain composes to the
  identity the bonding is consistent; where it does not, the residual is
  torsion, the bond structure's holonomy, held as content (MU-TOPOLOGY
  §4). This is xylarium's own closing of the books: "the composed versor
  is projectively the observed one".
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
rationals. Inside its own shell xylarium may read further quantities
(its jet onset, for example, is written with logarithms); those stay in
the shell and never cross.

Phases:
1. **G1:** Cl(n+1,1)(ℚ) in the shared algebra, with the Cl(4,1) suites as
   its `n = 3` claims.
2. **G2:** the pole axis, pole point, dilator and vacuum inversion as
   first-class constructors, conformant with xylarium's `poles` and `axes`.
3. **G3:** the pole state `(X(t), R)` and bonds as versors, with
   isomorphism, stabilizer and composition claims.
4. **G4:** bond classes filed by coordinates; the residual of a closed
   chain read as torsion.
5. **G5:** crossings as isomorphisms between faces' pole states (with
   xylarium; Concordance amendment).

## 20. The shared clock: lanes of attachment and chronometric anchoring

**Specified** in
[`docs/proposals/78.156-clock-binding.md`](docs/proposals/78.156-clock-binding.md)
§7.

TUniversal's final point `origin/h:S:G:T:H:K` already carries the
network's state on four lanes at every tick (MU-RECURSION 78.124), and
clients already stamp their acts with it. The track lets every client
(the Mycelium client, netstratum, xylarium, any kernel) coordinate on it
across universes:
- **Vertical attachment:** a client attaches on one lane, and its time is
  the distance that lane travels, tick over tick; it still carries the
  full point, so nothing is flattened.
- **Coordination across universes:** clients on different lanes and in
  different universes meet at final points, with dilation read per lane.
- **Chronometric anchoring:** a signed anchor (final point, lane, world
  axis `u`, state commitment) makes shared state a matter of anchors at a
  common point agreeing, with any difference read as a residual.

Phases:
1. **C1:** the lane of attachment, stated on binding.
2. **C2:** per-lane distance and dilation readings.
3. **C3:** the anchor act, its ordering and agreement readings.
4. **C4:** crossings that record each side's lane; per-mouth freshness
   (78.156 §4).
