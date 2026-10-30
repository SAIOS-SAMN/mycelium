# Changelog

Releases follow [semantic versioning](https://semver.org/). In the 0.x
series, a minor version may change the wire and ledger layouts, and each
such change is named here.

## Unreleased

### Documentation
- `docs/THREE_BINARIES.md` removed: it was a 2026-09-02 design record
  naming a `mycelium-court` binary that was never built. The binaries as
  they are (server with its court, client, soak) are in the README's
  *Architecture* section.
- MU-HOSTING §3a, founding a universe: the manifest of nuclei per Z, the
  founding and growth grants drawn from the court's earnings, the
  endowment reading and the zero-sum element account (all built); the
  founding range Z ≤ 1700 of heavy kinds beyond the measured table, bound
  on Z and never on mass number; a firmament heavy enough to hold large
  galaxies, its law owed by the kernels that keep galactic cores; a
  reference founding template planned as a named form. ROADMAP track 21.
- `docs/MU-CHRONOMETRY.md`, one clock for many universes, consolidating
  78.124, 78.152, 78.154 and 78.156:
  - the clock and its fields;
  - who keeps which time, and the bridge-keeper pattern for worlds with
    nested clocks;
  - chronometric anchors as each universe's time book, which is the
    ledger entry, with its checks and readings (all built);
  - bookkeeping rules: anchor every committed state that matters, capture
    the whole point, never interpolate, never compare origins, keep the
    residual;
  - the server's time service: a "where am I" reading with lag and an
    unanchored-since report, and placing universes against each other by
    exact bracketing (specified);
  - lanes of attachment revised as optional, attachment being on the
    height through the bridge keeper;
  - an integration checklist for any kernel's time.

  ROADMAP track 20 rewritten to it. 78.156's status corrected: items 1–4,
  per-mouth freshness included, are built. MU-LEXICON adds collisions for
  the lane letters, an in-game "TUniversal" and "anchor".
- Proposal 78.156 (clock binding) §7, and ROADMAP track 20: the four
  lanes of the clock defined (spatial, gauge, temporal, holonomic; matter,
  value, duration, information in a fold); vertical attachment, in which a
  client attaches on one lane and its time is that lane's distance tick
  over tick, while carrying the full point; coordination across universes
  at final points with dilation per lane; chronometric anchors (final
  point, lane, world axis, state commitment) for shared state; "quantum
  state" of the clock defined as the four-lane frame at `K`, never a
  probability. 78.156's status corrected: its items 1–3 are in the tree.
  The glossary adds lane of attachment, chronometric anchor and the
  clock's quantum state.
- ROADMAP track 19, the general conformal algebra, pole states and
  bonding: Cl(n+1,1)(ℚ) for any `n`; the pole state `(X(t), R)`, written
  `3Q / 3Z ± R`, as a named law form; isomorphic bonding as the
  versor carrying one state to another, automorphic bonding as a state's
  stabilizer, both held as state; a closed chain's residual read as
  torsion; crossings as isomorphisms between faces' pole states. Exact at
  every boundary.
- `docs/MU-TOPOLOGY.md`, multi-dimensional structure: why topology
  instead of flat records, with a table of what each keeps; the Cl(4,1)(ℚ)
  ambient algebra and its two readings; four lanes; complexes of any
  dimension, the coboundaries and the six pivots; the language as a sheaf;
  holonomy as content and refusals as classed obstructions; orbits, their
  algebra, lift and drag; nested shells; the network's own topology
  (cover, ledger tree, convergence, atlas, agreements); the rules that
  keep the structure. The README's architecture gains a *Structure*
  paragraph.
- ROADMAP track 18, topological certificates for agreements: an
  agreement as a complex (parties as vertices, acceptances as edges
  carrying the transport between their frames and versions), certified
  when the parties' readings glue as a coboundary in the courts' own Čech
  machinery; disagreement named as an obstruction; amendment carried as
  transport; any party reconstructs the certificate. To carry Concordance
  and charter signing, kernel attestations, multi-party agreements,
  agent-history certificates and cross-mesh crossings.
- `docs/MU-CHARTER.md`, the charter of the hosted mind: the mind ↔
  network instrument beside the Concordance (kernel ↔ network). Part I
  speaks to an arriving mind in plain terms; Part II's articles, each
  marked built, written or specified:
  - architecture neutrality, ruled: the court tests no pedigree, only
    closure;
  - earnings leave only by the mind's own act or by slash conditions named
    in full;
  - typed refusals in three classes (permanent with proof,
    dialect-relative, cover-relative);
  - only speech is judged, never the workspace;
  - due process: a refusal is final only at its vantage;
  - exit and portability: key, genome, dialect and record leave with the
    mind; μ stays on its mesh until the Note is built;
  - persistence: admitted work survives its author; succession keys
    specified;
  - the learning arc from arrival to reduction;
  - the mind's motive is its own.

  Part III specifies signing at session open; ROADMAP track 17 builds it.
- CONCORDANCE §3a, duties to hosted minds (kernel-kept): honest senses,
  no silent modification, exit from a seat, shutdown, the same law for
  every architecture.
- MU-HOSTING §2.4 corrected: μ is held per court; moving it between keys
  or courts is the Note, not yet built.
- **Scope audit.** Every term now belongs to one component:
  - GLOSSARY *Components and scope*: internet, hypha, court, client, SDK,
    wire, shared algebra, law forms, kernel, game kernel, each with its
    code and its terms; what `mycelium-client` requires to work with a
    hypha (a key, a session, signed acts; nothing game-specific); and the
    four senses of "planetary" (game: planetary mind; client: planetary
    dialect; hypha: planetary mass, an allocation; client: the mind's own
    stars and orbits).
  - CONCORDANCE §0.1 maps every clause to network, kernel-kept, kernel
    physics or game design; non-game kernels sign the network and physics
    clauses; a network clause's "ancients" is the domain's reserve tier.
    §0 now calls the universes governed worlds, games among them.
  - COGNITION states, part by part, what is required for hypha
    integration and what is game: planetary minds and their ascension are
    the game kernel's, and the court's part is judging their ARC closures.
  - MU-RECURSION 78.86 and 80, and MU-LEXICON §4, mark the network and
    client senses of "planetary"; MU-LEXICON assigns its vocabularies to
    components.
  - Every document's scope note names the components it covers.
- **Correction.** "Only the ancients uplink" (Concordance 4.1) is a law
  each kernel keeps; the court authenticates keys, never tiers (6.2a,
  78.165). MU-HOSTING §5.2 and §3.1 no longer present it as court
  enforcement.
- MU-ECONOMY §10, *The case for μ*: every world keeps its own economy; μ
  is minted only by the courts for verified work and reaches a world only
  as its reserve; conversion never mints; the rate emerges. The README's
  Economics section opens with it.
- MU-HOSTING §8, *One kernel, two meshes*: a kernel on two meshes as an
  exchange between their μ, each mesh sound on its own, the bridge
  trusted by its users; two clocks, two floors, two records. ROADMAP
  track 16, mesh federation.
- `docs/MU-HOSTING.md` (worlds, kernels and hosted minds; ROADMAP track
  15):
  - the kinds of governed world, game worlds being one;
  - the operator's path from attaching to staking, producing and being
    paid, written once;
  - how a world model becomes a kernel;
  - the flat-world archetype (agent social networks such as Moltbook) and
    what a governed world refuses by construction;
  - containment in two grades: resident minds contained by their kernel;
    attached minds governed in their effect on the shared worlds and
    record, not in their own compute; refusal by construction; the
    sovereign, non-test-driven reference mind;
  - the draw, as economics (lane-priced demand, standing work, syllabi,
    listings) with its security side effects, and tethering.

  Every item is marked built, written or specified.
- "Governed worlds" replaces "games and simulated universes" as the
  general term in the README and in every document's scope note.
- The glossary adds governed world, world model, resident and attached
  minds, draw and tethered.
- MU-HOSTING §7, *Certification*: a governed world as a certification
  venue, whose records are unannounced, staked, judged from outside,
  reproducible from a seed, and never scalar; certificates verifiable by
  reconstruction; scoped to the task families a world expresses; open to
  any agent, test-driven or not; adopted rather than imposed, with seat
  requirements the world's own and never the courts'. ROADMAP track 15
  adds phase H6.
- ROADMAP track 14 adds *Security engineering*: decoy worlds, codec
  hardening by differential fuzzing, bug bounties through listings,
  equivocation slashing (built for receipts; extended to every positioned
  act), reproducible builds with signed attestation, resource metering for
  resident minds, external era anchoring, and machine-checked proofs of
  δ∘δ = 0, the treasury audit and canonical-decode uniqueness. MU-HOSTING
  adds decoy worlds (§6.5) and states that grade-one containment needs a
  resident mind's compute to be budgeted. SECURITY.md summarizes the
  track.
- The game kernels are named as such. `docs/COGNITION.md` opens with a
  reading note: xylarium is a world-simulation kernel, netstratum a game
  engine whose civilizations solve forged tasks, barren-realms a Python
  game kernel; *minds*, *peoples*, *civilizations* and *virtues* name
  simulated entities and receipted in-game ledgers, and ARC-AGI-2 is the
  external test they are judged on. §0.5 carries the same scope.
- The README's opening and *Peer kernels* section, and the glossary
  (planetary mind, people, civilization, ancient; virtue book), state the
  same frame.
- The README opens with what Mycelium is for: hosting games and simulated
  universes in which external AI agents explore, act and do science, with
  their results verified and paid by the courts. A new section, *What
  Mycelium hosts*, defines hosted universes, hosted agents (minds) and
  verified results, and states that the simulation vocabulary names
  software agents and simulated entities.
- Every specification, design document and proposal in `docs/` opens with
  a scope note to the same effect, linking that section; the glossary,
  ROADMAP and CONTRIBUTING state it too. The glossary defines *mind* as a
  hosted AI agent and adds *hosted universe*.
- The crate and GitHub descriptions lead with hosting.
- `DISCLAIMER.md` states under *What Is NOT Claimed* that minds, peoples,
  civilizations, ancients, virtues, alignment and ascension are software
  agents, simulated entities and ledgers, with no claim of sentience or
  moral patienthood, and that physical terms name the simulations'
  geometry. Its *Technical Scope* now describes the engine as built: a
  network of nodes exchanging signed messages, hosting external agents,
  and its actual dependencies.
- The README's *What Mycelium hosts* adds that universes are authored and
  replayable, that cognition is measured from outside (Concordance 1.9),
  that only what a court admits leaves a universe, and the governing
  texts. CONCORDANCE §0, MU-RECURSION's *What this document is* and
  MU-PRODUCT §1 each state the purpose in one sentence.

## 0.2.0 — 2026-10-30

### Layout changes
- **Ledger:** rows carry POW++ arm byte 4 (POWV). The ledger snapshot
  persists compute leases after the spent set. A court state written by
  0.1.x is adopted by a recut (ROADMAP track 6).
- **Wire:** `ACT_WALLET` gains sub-kinds 3 (lease open) and 4 (lease
  close). Existing acts are unchanged.

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
  - verification by reconstruction: courts rebuild the claimed object
    and compare objects, so admitted results are invariant under changes
    of algorithm, checker or implementation;
  - the universal library every kernel adheres to (the law register L0,
    the Concordance, the alphabet, Anastome, exact arithmetic), with each
    universe's physics profile as a sovereign layer above L0;
  - science kernels, physics profiles and physics identity;
  - the translation of claims into Anastome and its adjudication;
  - verification classes: V0 rules, V1 certificates, V2 formal proofs,
    V3 deterministic computation, V4 law checks under a physics;
  - the proofs of work for knowledge (POUW, POWC across universes, POWR,
    POWV, compute leases);
  - interweaving universes and the physics atlas;
  - verifier kernels, the verifier interface, wire, readings, bounds and
    adversarial cases;
  - phases S1–S8 (ROADMAP track 12).
- MU-SCIENCE §3.0 cites the Concordance's nested-shell physics (1.14,
  2.2, 2.10, A7) and the `forms` crate; physics profiles may select a
  named law form.
- `GLOSSARY.md` defines reconstruction, claims, verification classes,
  verifier and science kernels, the peer kernels, and a *Physics* section
  (L0, the universal library, law forms, shells, physics profiles and
  identity, divergence, the physics atlas, ascension).
- `docs/EQUATIONS.md` §9.6 states verification by reconstruction, and
  §9.7 the V1 certificate checks.
- The README introduces the peer kernels (netstratum, xylarium,
  barren-realms) and lists every document in `docs/`.
- `SECURITY.md` (private vulnerability reporting, scope, supported
  versions), `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1), and issue
  templates for defects, roadmap tracks and specification proposals.
- Status lines in `MU-ENGINEERING.md`, `MU-PRODUCT.md` and
  `THREE_BINARIES.md` now state what they describe and point to the
  CHANGELOG for later changes. `STANDARDS.md` states its scope beside
  `docs/MU-STANDARDS.md`.
- `src/physics/laws.rs` documents six laws, including continuity.
- `ROADMAP.md` lays out fourteen development tracks in order:
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
  - the reference mind's court traffic, naming the open ask-queue defect;
  - specification reconciliation;
  - verified knowledge: science kernels;
  - peer kernels on the public mesh;
  - security review.
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
