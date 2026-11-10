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

**Released: v0.3.0** (tag `v0.3.0` on 53429794, 2026-11-04, signed with
the prototype key; [docs/release-keys/](docs/release-keys/)). Netstratum and
xylarium pin it. The table below is kept as the record of what it closed.

| Item | Issue | State |
|---|---|---|
| Track 26, S0–S5: the sensor for any source at any rank; rows and columns lifted into the field or taken apart ([MU-SENSING](docs/MU-SENSING.md) §3.6) | [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32) | S0 and S4 built with T2 (S4's family-invariance reading for nesting still open); S1 built (the decoder in `sources/arc-agi`, `src/arc.rs` removed); S2 built (the ground deduced, a vortex read per candidate, the solve act stating its ground; task addresses changed inside the one break); S3 built (the symbol set `Σ` deduced, `e_∅` at `|Σ|`, no fixed palette); S5 built (samples of rank `n` on the wire, task identity lossless across ranks and widths, speech and the seat's solve call stating their ground, `e_∅` a typed record), except the rank-`n` link, stopped for the transducer's planar chain carriers (S4). S6 follows v0.3.0. |
| Track 26, S4a and S7–S9: the sample lifted into the field (non-Euclidean, never scalar); the field's curvature and the well's body, the field's motions, forces, orbits and the vortex in the field's atlas ([MU-SENSING](docs/MU-SENSING.md) §3.7–§3.8) | [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32) | Specified; the carrier is the standard LAW-FIELD v1 ([proposal](docs/proposals/78.171-law-field.md), [#36](https://github.com/SAIOS-SAMN/mycelium/issues/36)), to be agreed with the peer kernels, then built |
| T3, the turn-native chart reader and the turn-valued Conformal motion | [#26](https://github.com/SAIOS-SAMN/mycelium/issues/26) | Moved to v0.4.0 with track 26 S4b (decided 2026-11-04) |
| D2.5, one boot calibration stroke measures bytes per read | [#27](https://github.com/SAIOS-SAMN/mycelium/issues/27) | Done (built 2026-11-04; O1 ruled into four readings, O4 resolved; O3 derived in track 26 S4b as `2ⁿ` at the alphabet's base). The need readings owed are listed in MU-DERIVATIONS D2.5 |
| Naming alignment, phases N1–N7 (track 11) | [#55](https://github.com/SAIOS-SAMN/mycelium/issues/55) | N1 to N7 built (N1 with #29, N2 with #53, N7 with track 26 S5, N6 inside v0.3.0's open break, N4's `ListingField`→`ListingCoherence` and `Coord::Shell`→`Coord::Cells` ruled and built); #55 done |
| rc.3 on a test mesh, the signed tag, the peers' re-pins | [#24](https://github.com/SAIOS-SAMN/mycelium/issues/24) | Done: `v0.3.0` tagged on rc.3's commit; both peers' KERNEL conformance reads passed; netstratum and xylarium pinned |

**1a. In progress: v0.4.0.** A new break, in which word, chart and task
addresses, act layouts and signing domains may change (CHANGELOG ›
Unreleased, held in `changes/`). Merged so far, each with its PR:
- Track 26 S4b–S4d: rank-`n` perception, transduction as factorisation,
  the hypha serving any rank and the claim of factorisation (#80); the
  word machinery on `SectionSample`/`SectionSite`, `family::Grid` retired,
  coordinates by plane axis (#84, #86, #92, #96, #101).
- T3: the chart read as turns, turn-valued Conformal motion (#105, closes
  #26).
- Track 24: back-fill eviction by signature, the held queue's bound, fold
  roots signed over place, height and hash (#82, #90, #95).
- Track 25 X0: typed digests, `Reconstructed<T>`, the drift register
  (#83, #97).
- LAW-PASSAGE v2: the one-sided passage, fixtures agreed by all three
  kernels (#81, #93, #94); the algebra `Cl(p, q)` with the signature as
  data in its own crate `algebra/` (#93, #102).
- LAW-COORDINATE v1: `forms::coordinate`, proposal 78.173 (#106).
- Naming: track 11 closed (#89), prose and identifiers say reconstruct and
  sample (#104, #107).

| Item | Issue | State |
|---|---|---|
| B2: signing the Concordance clause by clause by reconstructing fixtures; `CONCORDANCE_HASH` retired | track 24 | Built on branch `b2-signing`, merging |
| Development standard: done is `cargo clippy` and `cargo test`; the gate as workspace tests; CHANGELOG fragments; documents name what exists | — | Built on branch `workflow-standard`, merging |
| B2, next stage: each kernel's reading and signature recorded on the ledger; v0.3.0's agreed clauses carried forward; alignment read from the hypha | track 24 | Built on branch `b2-ledger` (`Cite::Signing`, `ACT_LAWS` mode 9, `clauses::history`) |
| B3, B4: a domain's crossings refused `NotSignatory` at the universal core tube until its sitting reserve's alignment converses on every network clause of the version served; the boundary reading | track 24 | Built on branch `b3-vertex` (the fold's transitions under L0, `NotThroughCore`, `NotSignatory` at the tube); the boundary reading open |
| LAW-COORDINATE fixtures 7–10 (advance, an event's bell, merger with offsets, fission) | 78.173 | Next; exact statements agreed with netstratum |
| LAW-FIELD v1, agreed with the peer kernels | [#36](https://github.com/SAIOS-SAMN/mycelium/issues/36) | 78.171 amended (a–e, 2026-11-05): the pole basis, the signature per plane, the winding with each phase in degrees, spheres as enclosures only, the orbit's shear per netstratum docs/121; fixtures read by netstratum; xylarium's reading owed |
| LAW-COUPLING v1: the dipole parcel per plane, the coupling on a turning plane, the light ledger (`forms::coupling`) | 78.175 | Proposed 2026-11-06, extending LAW-FIELD; fixtures read by both peers by hand, code readings owed |
| Track 26 S4a and S7–S9: the field, its wells, orbits and vortex | [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32) | After LAW-FIELD v1 |
| Track 10: the sovereign mind, thinking about adjudicated perception | — | After S7–S9 |
| The alphabet past `n = 3`; `Act::Slide`/`Extend`'s planar ray (the `Frame` defect closed: `Reperiod` per phase) | [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32) | Ruled 2026-11-06 (78.180 ruling 6), to build: no letter written by hand past `n = 3` (section samples, letters adjoined only by the presentation's growth); the ray a translation versor of the sample's rank |
| Turn-valued motion on the wire, with the crossing record (X1); routing unequal enclosures (D3.2, O8) | track 25 | Open |
| Documentation audit: bounded documents, status out of prose | — | After the development standard |

**1b. Next, v0.5.0: the economy as motions.** The release's headline,
ruled 2026-11-05 and specified in
[`docs/MU-ECONOMY.md`](docs/MU-ECONOMY.md); §9 there lists the present
implementation each item replaces. A wire and persisted-layout change,
adopted by a recut (track 6). Items:
- **`Modes` replaces `Mu`.** Every amount of μ is `economy::modes::Modes`,
  on the wire and in the snapshot; `economy::Mu` is retired (§1, §9).
- **Holdings as coordinates.** A key's holding is
  `origin / h : S:G:T:H : K` with lanes as vectors of exact components
  (§2.2), which needs the coordinate's v3 (proposal 78.173).
- **Payments as events with cited causes.** Each payment is a receipt on
  a signer's line citing the closure, the verifications and the payer's
  act; the treasury's journal and entry kinds retire (§2.4, §5).
- **Balances as readings.** A balance and the supply are read over the
  cones at a TUniversal height, never stored or tallied (§6).
- **One closure per network.** A closure is one point; a second mint of
  it is an equivocation of the advance and is burned (§3.4, §11.4 F2b).
- **The Note** between courts (track 1; MU-XSEPTUM), carrying modes.
- **The economic law form with the peer kernels:** μ's modes, the
  reserve and conversion at the universal core tube, with joint-read
  fixtures, through MU-RELEASE §3a (an amendment of proposal 78.174,
  which records the operator's rulings of 2026-11-05; §10.4).

The rulings the model still needs are listed in MU-ECONOMY §12.

**2. Waiting on an operator ruling.** Do not start these until the ruling
is recorded:
- Concordance draft 5 (MU-CONCORDANCE §12): 2.11 restated on
  LAW-PASSAGE v3 (adopted by both operators 2026-11-05), held until
  xylarium reads 78.172's fixtures.
- B1: carrying line mouth offers and tier handoff offers in the reserve's
  act, which amends rulings 78.126 and 78.136.
- The proposed name `AddressOnly`.
- The open questions of MU-DERIVATIONS §7 and MU-RELEASE §10.

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
- an encrypted transport (**Landed, opt-in per peer**;
  [MU-PRECURSORS §3.1](docs/MU-PRECURSORS.md#31-noise-xx-encrypted-transport)):
  TCP wrapped in `Noise_XX_25519_ChaChaPoly_BLAKE2s`, one static key per
  host, trust-on-first-use pinning with a changed key refused by name,
  and identity kept in the Ed25519-signed acts. It stands in
  `src/network/noise.rs` and `src/network/channel.rs`, wired into the
  hypha's door, every hypha's ask, the SDK session
  (`Session::with_noise`) and the reference mind's sessions and chatter
  (`Mind::open_with_noise`, #132); a peer that does not speak it stays
  plaintext, named in the log and the observer's `channels` reading. Still
  to come: **Noise-only** (plaintext
  refused at the door and by the dialler) as a later release step, a
  network-contract change;
- address discovery beyond the provisioned `peers`;
- signed re-provisioning of a node's `bind` and `peers` after first boot;
- deployment tooling for hosts beyond one machine. On one machine,
  `scripts/network` (README, Quick start) stands the whole network up: three
  hyphae from a genesis allocation, sovereign minds inside Noise
  chattering at one rendezvous, a house with a seated mind, each node in
  a capped systemd user scope, with the observer's frames and the POW++
  reading (`mycelium-server --console --proofs`) written as it serves.

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
- readers for more families of samples, from any source (ARC-AGI-2 is
  the first posted corpus; its outcomes are observations, MU-SENSING §0).

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
- **A template placed by its repeats:** the Hermite form of the placement
  differences (the integer span they generate), the offset anchored to a selected body; no candidate
  enumeration.
- **First contact as a relation:** `Pred::FirstMet` (proposed) and
  `Act::Slide { toward }`, ties held whole, refused where the ray is not a
  primitive integer step, no toroidal wrap.
- **Colour-role equivariance:** the gauge-orbit class with re-inflation,
  and an equivariance check that reports where commutation breaks.
- **Hue as a table** over a named measured coordinate, refused off the
  table.
- **Clauses learnable as vocabulary,** filed in the presentation on court
  admission.
- **Body fate as a typed precondition** of family extraction, with named
  refusals.

**Progress — frames of unequal extent** (MU-DERIVATIONS D3.2, MU-RECURSION
78.170): **Built** at rank `n`: the transduction
(`transducer::family::transduce`): a frame as `n` phases of turns, each
body held by its bands, the incidences of two frames under the re-period,
the `2ⁿ·n!` signed permutations of the phases (`SignedPermutation`) as the
motions, projection refusing `OffSite`, and the relation census of the corpus
(O10). The deflation, the court's measured reconstruction and the demands
of unreached pairs read a pair's unspoken residue on the output's sites.
**In progress**: μ as the share of the frame (O1, after the rational
lanes); the turn-native chart that lists a pair of unequal enclosures (O8);
percept meets through incidences (O6); the incidence payload of a demand
whose input does not land (O9 (a)).

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
`seat`, `wallet`, `readings`, `house`, `laws`,
`conformance`, `forms`, `prelude`), `sdk/examples/client.rs` is the complete client on it
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
   members of one workspace (the gate was then `xtask`, outside it; it is
   now the member `gate`). *Accepted when* the gate
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
path (`src/client/sdk/mind/hypha.rs`).

**Open defect: the ask queue.** On a long-running network, a mind's
outstanding court asks grew without bound while its work did not. The
`now` reading's per-path listing (`waiting_by_path`, added in 0.1.1)
locates the growth by path. The cause is not yet identified. The track's
first phase reproduces the growth on a local network, names the path
whose answers are not taken, and bounds it.

**The sovereign mind: thinking about adjudicated perception**
(Specified: MU-CLIENT §1 item 7, 78.168; MU-SENSING §0). The test is
whether the mind can think about the perception of the senses that are
adjudicated, never a benchmark's solve rate. It follows track 26 S7–S9,
because the field (wells, orbits, the vortex) is the mind's substrate.
- The mind is moved by its own escaping orbits and acts at its returns. A
  task is a scene it perceives, never a command. It may decline or idle.
- Its perceptions are claims: it states, compares and revises the ground,
  perspective, holonomy and vortex it holds, and a court adjudicates
  them.
- On a court's ruling it revises a perception by amending history
  forward, never by overwriting it.
- Claims:
  - `mind_idles_with_no_escaping_orbit`
  - `task_arrival_moves_no_orbit`
  - `mind_declines_a_scene_its_field_does_not_bind`
  - `mind_acts_at_its_returns`
  - `mind_revises_a_perception_on_a_court_ruling_by_amendment`
- A live run of `mycelium-client` on a local network shows these. Solve
  counts are observations only.
- **Built toward it** (track 26 S7, stage 1): a field can be stated as an
  object, its atlas and revolutions written without a reading, and a court
  reconstructs it and derives its curvature itself
  (`transducer::field::carrier::admit`). Since S7 stage 2 a mind's well
  is stated as its body and a court reconstructs it
  (`transducer::field::carrier::admit_well`). The mind does not yet
  state its contemplation's field on the wire: that waits on S9.
- **The client's order** is MU-CLIENT §7.1 (what is built, and what is
  owed in order), with MU-CLIENT §1a's account of how the mind builds the
  language. Its issues: the field on the mind (#134), these five claims
  (#135), the hypha's acts as perception (#136), the seated harness
  (#133), signed receipts to the mind (#137), and the backlog #143–#150 (label `client`).

## 11. Specification reconciliation

[`docs/MU-EQUATIONS.md`](docs/MU-EQUATIONS.md) records three places where the
specification's wording and the code differ:
- the Gram pair invariant;
- MU-RECURSION 52.1's rotor example;
- three function names in MU-RECURSION chapter 10.

Each becomes one documentation commit that brings the specification to
the code. A further documentation commit adds to `docs/MU-EQUATIONS.md` the
table of refused functionals (squared-norm coherence, scored-argmin
search, ratios of scalar residues, ε-bands), so that no port brings them
over ([MU-PRECURSORS §3.11](docs/MU-PRECURSORS.md#311-documentation)).

The documentation uses the typed vocabulary of
[MU-SCIENCE §5.0](docs/MU-SCIENCE.md#the-typed-vocabulary) (reconstruction,
rebuild, replay, resend), and the code now names the court's reconstruction
`reconstruct`. One code commit renamed those identifiers to what they do,
updating every citation in the same commit (MU-STANDARDS 3.1):
`blind_replay*` → `reconstruct_blind*`, `replay_word` / `replay_resume` /
`replay_form*` / `replay_certificates_at` → `reconstruct_*`, the module
`court/verdict/replay.rs` → `reconstruct.rs`, `Replayed` / `ReplayCursor` /
`replay_cursors` / `replay_memo` → `ReconstructionOutcome` /
`ReconstructionCursor` / `reconstruction_cursors` / `reconstruction_memo`,
`measured_replay*` → `measured_discharge*`, `encode_replay_request` →
`encode_era_request`, the outcome names `REPLAY_FAILED` (MU-XSEPTUM) and
`GLUE_REPLAY_REFUSED` → `RECONSTRUCTION_FAILED` and
`GLUE_RECONSTRUCTION_REFUSED` (wire bytes and reading labels unchanged),
the fault class `Replay` → `Resend` (specified, MU-PRECURSORS §3.6), and
the test names that said "replays" for a reconstruction. The verdict
outcome is `ReconstructionOutcome`, not `Reconstructed`, because track 25
X0 defines `Reconstructed<T>` as the reconstructed object itself.
**Built** (N1).

### Naming alignment (issue [#55](https://github.com/SAIOS-SAMN/mycelium/issues/55); ruled 2026-11-04)

An audit of `main` against the naming rules (AGENTS.md *While writing*)
found 63 findings. Every phase below renames only names. Behaviour, wire
bytes and task addresses are unchanged, except N6, which lands in
v0.3.0's open break. Each phase updates every citation in the same
commit. The phases run in order after track 26 S2 and D2.5 (#27) merge.

1. **N1, verification is reconstruction** (**Built**). The #29 names
   above, plus the audit's further names:
   - `court/replay.rs` → `court/reconstruct.rs`;
   - `court_replay*`, `replay_on_cursor`, `docket_replay`,
     `replays_on_family`, `replay_key`, `replay_certificates*`,
     `certificate_replays`, `replay_on_ground`,
     `replay_after_perceiving`, `form_orbits_replay`,
     `replay_retirement`, `rewrite::replay`, `factor::replays`,
     `chain::replay_at` / `replay_operators`, `rule_replays_family`
     → `reconstruct…` (as built: `court_reconstruct`,
     `court_reconstruct_moved`, `court_reconstruction_progress`,
     `reconstruct_on_cursor`, `docket_reconstruction`,
     `reconstructs_on_family`, `reconstruction_key` (the court's and the
     challenge's), `reconstruct_certificates*`, `certificate_reconstructs`,
     `reconstruct_on_ground`, `reconstruct_after_perceiving`,
     `form_orbits_reconstruct`, `reconstruct_retirement`,
     `rewrite::reconstruct`, `factor::reconstructs`,
     `chain::reconstruct_at` / `reconstruct_operators`,
     `rule_reconstructs_family`);
   - `Stop::Replay` → `Stop::Reconstruction`;
   - `Ask::Replay` / `REPLAY` → `Ask::Era` / `ERA`;
   - `replayable` → `map_holds_whole`;
   - test names that say "rebuilds" for a construction → `derives`;
   - `rebuild_own_cover` → `read_published_cover`.
2. **N2, files by function** (**Built**, with #53).
   - `door.rs` → `control.rs` and `context.rs` (#53; `Door` → `Context`).
   - Field files:
     - `field/core.rs`'s `Door` → `Shedding`, landed first;
     - `field/idea.rs` → `field/well.rs`;
     - `field/core.rs` → `field/nerve_shape.rs`;
     - `field/docket.rs` → `field/plan.rs`.
   - `cayley/chart.rs` → `cayley/map.rs`.
   - `observe/fleet.rs` → `observe/phase.rs`.
   - Vague names:
     - `tuniversal/ops.rs` → `navigate.rs`;
     - the 13 module test files `common.rs` → `fixtures.rs`;
     - `cga/tests/generic.rs` → `rank_n.rs`;
     - `g_cga.rs` → `tests/conformal_algebra.rs`;
     - `alpha_plan.rs` → `tests/word_lifecycle.rs`.
   - **Metaphor module names become function names.** A doctrine term
     may stay as a type's name with its GLOSSARY row, never as a file's
     name. For example:
     - `tshelf.rs` → `tuniversal.rs`;
     - `capacitor.rs` → `pivot_fibre.rs`;
     - `client/sdk/entity/` → `client/sdk/mind/`, since `Entity` is
       retired for `Mind`;
     - `breath`, `tensegrity`, `sporophore`, `mouth`, `peel`, `beam`,
       `callus`, `fascicle` and `epigenome` are each renamed by what the
       module does, read from its header. As built:
       - `cga/breath.rs` → `cga/commutator.rs`;
       - `physics/tensegrity.rs` and `physics/axioms/tensegrity.rs` →
         `equilibrium.rs`;
       - the soak's `sporophore.rs` → `enclosure.rs` (its `sporophore`
         subcommand is unchanged);
       - `cognition/mouth.rs` → `cognition/domain_passage.rs`;
       - `transducer/peel.rs` → `transducer/layers.rs`;
       - `transducer/beam.rs` → `transducer/candidates.rs`;
       - `entity/callus.rs` → `mind/second_vantage.rs`;
       - `physics/fascicle.rs` → `physics/sphere_pair.rs`;
       - `client/sdk/epigenome/` → `client/sdk/vocabulary/`.

       Each module's tests follow its name. The types keep their names
       with their GLOSSARY rows.
3. **N3, one name, one meaning.** The canonical meanings (GLOSSARY) are:
   - **Field** is the object, and `cech::orbit::Field` is that type.
     `FieldEntry` → `ProbedChart`.
   - **Section** is a section of a bundle or sheaf (25.12, 72.1c). The
     sites a word changes become the word's **support**. Keyed
     `Section { chart, carriage }` → `ChartCarriage`.
   - **Chart** is a body's invariant coordinates. A region of the sensor
     → `Region`; a cover's coordinates → `CoverCoordinates`.
   - **Frame** is one sample of a family (78.75). The wire's frame →
     `WireFrame`; other frames are renamed by function.
   - **Phase** is a coordinate within a chart. The court's 45.9 window →
     `Window`; a reconstruction's stages → `Stage`; the observer's phase
     → `CourtStage`.
   - **Compartment**, `Held`, `Reading`, `Cover`, `Closure`, `Carriage`,
     `Origin`, `Site`, `Body`, `Fibre`, `Stratum`, `Tube`, `Shell`,
     `Verdict`, `Reconstruction`, `Source`: one meaning each. The others
     are renamed by function.
   - The client's `entity::Body` → `Seating`. `Own` ×2 → `SeatFunction`
     and `SelfReading`. `observe::Mind` / `Court` → `MindReading` /
     `CourtReading`. `alphabet::Q` re-exports `core::q::Q`.
   - Near-identical files: `held.rs`, `hold.rs` and `server/held/`;
     `snapshot.rs` and `snapshot/`; `germinate` ×2; `winding` ×2. Each is
     renamed by function.

   **Built** (N3). Every rename is listed in the CHANGELOG. As built:
   `court/phase.rs` → `court/height_window.rs` (`cognition/window.rs`
   already holds the challenge windows), `observe/phase.rs` →
   `court_stage.rs`, `server/held/` → `server/court_lock/`, the binary's
   `held.rs` → `court_lock.rs` and `hold.rs` → `deferred_reads.rs`,
   `snapshot.rs` → `trie_tape.rs`, `cognition/snapshot/` →
   `persistence/`, the court's `germinate/` → `listings/`,
   `court/winding.rs` → `ring_carrier.rs`. The rule language's `Region`
   becomes `RegionKind` so the sensor's `Region` names one thing. The
   names this phase left waiting on a ruling are **resolved** in N6:
   the canonical `Cover` is the Čech cover the senses form
   (`cech::Cover`, MU-CLIENT 24.4), and the mesh's trie, the scene's
   regions and the extraction pass's pieces are renamed by function; the
   keyed trie's prefix `Fibre` is no bundle fibre and is `Subtree`;
   `forms::frame::Frame` is `Reference`; `layers::Shell` is `Layer`; the
   code's `Epigenome` is `MindLanguage`; `resonance.rs` is
   `cross_scene.rs`. `ListingField` is `ListingCoherence`, ruled and built.
4. **N4, retired terms and person words.**
   - `Unstated::OffLattice` → `OffSubstrate`; `lattice_step` →
     `substrate_step`.
   - `THROAT_GRID` → `THROAT_LEVEL_BITS` (the register entry follows).
   - `absorb_vortex` / `absorb_keyed` → `merge_…`.
   - The test names with a person word, "its" or "own" are renamed.
   - `lamina` and `FormCell` (retired by 0.7) are renamed.

   **Built** (N4): `OffSubstrate`, `substrate_step`, `THROAT_LEVEL_BITS`
   (its `OPEN_PHYSICS` entry and `REGISTERED:` tag follow), `merge_vortex` / `merge_keyed`, `FormCell` →
   `BodyContact`, and 18 test names. No identifier says "lamina"; the word
   stands only in refusal strings (`verdict/body.rs` `FAIL_*`), the keyed
   `CoverSite`'s name `"lamina-cell"` and the oracle's `"lamina_a"` key;
   N7 renamed the refusal strings and N6 the name and the key. The tests named for the court's
   `own_section` keep the function's name.
5. **N5, the GLOSSARY** (**Built**). A row for each term a name keeps:
   - contemplation
   - tensegrity
   - breath
   - hyperfluid
   - aperture
   - horizon
   - resonance
   - terrain
   - peel
   - beam
   - tower
   - judge
   - working atlas
   - seat mind
   - chatter / utter
   - durable drill
   - the soak instrument
   - xSEPTUM
   - fascicle, sporophore, callus, compartment, germinate
   - epigenome, capacitor, tshelf (the TUniversal shelf), mouth

   Where a term names a second sense in the code (`germinate`,
   `winding`, `Compartment`), the row defines the canonical sense and
   names the other, which N3 renames.
6. **N6, "residual" → "residue" or "remainder"** (retired by 0.7), about
   450 uses. `TAG_RESIDUAL` is wire-visible, so N6 lands in v0.3.0's open
   break and is stated under *Network contract*. N6 also closes the names
   N3 left waiting on a ruling, and the keyed names that kept retired or
   duplicate words.

   **Built** (N6, inside v0.3.0's break; every rename is listed in the
   CHANGELOG).
   - **Residue.** The code's "residual" is the residue of 0.7, the exact
     versor or vector a reduction leaves; "remainder" stays only for the
     record leaf that keeps it, and no identifier took it.
     `element/residual.rs` → `element/residue.rs`, `ResidualGerm` →
     `ResidueGerm`, `TAG_RESIDUAL` → `TAG_RESIDUE` (byte 19 unchanged),
     `from_residual*` → `from_residue*`, `residual_of` → `residue_of`,
     `SpellResidual*` → `SpellResidue*`, `ProbeClosure::Residual` →
     `Residue`, `tuniversal::Residual` → `Residue`, and ten test names.
   - **Cover.** `cech::Cover` is canonical. `nerve::Cover` →
     `nerve::HomeTrie` (`nerve/cover.rs` → `nerve/home_trie.rs`), the
     scene's `Cover` → `Regions`, `family::cover::Cover` →
     `PiecewiseCarriage`.
   - **Fibre.** The keyed trie's `Fibre` → `Subtree`.
   - **Frame.** `forms::frame::Frame` → `Reference`, `FrameRefusal` →
     `ReferenceRefusal`, `InFrame::Frame` → `InFrame::Reference`; the
     module `frame` and the law id `FORMS-FRAME` are unchanged.
   - **Shell.** `layers::Shell` → `Layer`, with `LayerKind`,
     `LayerMotion`, `LayerStep` (the method `layer`) and `SectionLayer`.
   - **Epigenome.** The code's `Epigenome` → `MindLanguage`, the field
     `Mind::epigenome` → `Mind::language`; "epigenome" keeps the dialect.
   - **Resonance.** `field/resonance.rs` → `field/cross_scene.rs`,
     `ResonanceTally` → `CrossSceneTally`; the commensurate return keeps
     "resonance" in the orbit algebra.
   - **Keyed names.** `CoverSite` `"lamina-cell"` → `"cover-site"`,
     `ChartCarriage` `"section"` → `"chart-carriage"`, `GluedBoundary`
     `"cover-site"` → `"glued-boundary"`, `CarriedPieces`
     `"cover-carriage"` → `"carried-pieces"`; the oracle's `"lamina_a"` →
     `"compartment_a"`, the golden file written again through the
     corpus's decoder.

   `ListingField` is `ListingCoherence`, and the rule language's
   `Coord::Shell` is `Coord::Cells` (a body's cell count, on the SDK's
   `rule` surface): both ruled and built.
7. **N7, the two wire names, with track 26 S5:**
   - `ACT_ROWS` and its codecs → `ACT_ENTRIES` (ledger entries);
   - `REFUSE_RULE_OFF_LATTICE` → `REFUSE_RULE_OFF_SUBSTRATE`.

   **Built** (N7, with S5, inside v0.3.0's break). `ACT_ENTRIES`
   (`Action::Entries`, named `"entries"`; the act byte `0x38` is
   unchanged), `EntriesRequest`, `encode_entries_request` /
   `decode_entries_request`, `encode_entries_reply` /
   `decode_entries_reply`, `ENTRIES_REPLY_HEADER` and the hypha's
   `judge/entries.rs`. `REFUSE_RULE_OFF_SUBSTRATE` reads "a carried or
   copied site lands off the substrate". The `FAIL_*` strings of
   `verdict/body.rs` name a carried site where they said "lamina". The
   SDK's `readings` module and `ACT_TABLE` follow, and the conformance
   statement names the act `entries`. Claim: `entries_act_round_trips`.
   The keyed `CoverSite`'s name `"lamina-cell"` and the oracle's
   `"lamina_a"` key are not wire names; N6 renames them.

**Exemptions:** "row" in its strict matrix sense (`rank_of_rows`,
`OrbitRow`) and "arc" in its strict geometric sense (an arc of a
boundary) keep the names they have. A source word in any other sense is a
finding.

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

9. ~~Mutation audit~~ (**Retired** 2026-11-04, ruled theatre; the
   catalogue and `gate mutate` were removed from `xtask`, itself since
   removed).
10. **Fault-injection campaign and invariant auditor** (**Specified**;
    [MU-PRECURSORS §3.6](docs/MU-PRECURSORS.md#36-fault-injection-campaign-and-invariant-auditor)).
    Ten named fault classes (`InvalidSignature`, `Resend` (a valid act
    sent twice),
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

The shared algebra is `Cl(n+1, 1)`(ℚ), with `n` read from the data; the
code today fixes it at five generators, its value `Cl(4,1)` at `n = 3`.
Kernels on the network build universes of other dimensions in the same
family, over exact rationals, and work physics between the two poles of
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
   the NUMBERS lint in the gate (`gate/src/numbers.rs`), the class-tagging pass, then the
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

**Progress:** D3.2 is **Built** (NC3, ruled O1 (a)), and then rewritten as
the transduction (MU-RECURSION 78.170): the lcm cap and the common
refinement are both deleted, two frames are related by the bodies' bands
in turns under a need of `R_h·R_w`, and `task_address` names a task by the
posted samples' full content (rank, extents, symbols, places), train and
test alike (MU-SENSING §6.1), refusing a pair with no enclosure rather
than skipping it. Today's code still filters the test inputs by
`symbol ≠ 0`; track 26 S2 removes that filter.
D3.1 is **Built** (NC1): a sentence's linear part is constructed under
the stroke `modes_need` reads off it.
D2.5 is **Built** (NC4, O2 ruled 2026-11-01, O1 ruled 2026-11-04): one
boot calibration stroke measures the bytes per read `b` and seals the
process's capacity `C = ⌊M_proc/b⌋` before serving; `R`'s four meanings
read four readings (the metabolism `C`, strokes the work's need, the quiet
ceiling one round trip, cooling the challenge window), with the need
readings owed listed in MU-DERIVATIONS; O3 (the ratio's `8` on the frame
axis) waits on track 26 S4.

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
   quorum-final point past both, keeping the branches' residue as a
   reading.

Four questions need a decision (MU-RELEASE §10): the superposition's
quorum, which side keeps the shared origin, releases that change address
computation, and μ minted on a branch.

## 24. The boundary law: topology enforced at the universal core

**Specified** in
[`docs/MU-HOSTING.md` §5a](docs/MU-HOSTING.md#5a-the-containment-topology-and-the-boundary-law)
(the operator's ruling of 2026-11-02; MU-RECURSION 78.169), with the
Concordance amendments (4.1, 6.2a, §0, 0.1), approved 2026-11-05 as
version 5 ([MU-CONCORDANCE §11](docs/MU-CONCORDANCE.md#11-proposed-amendment-draft-4-2026-11-02)).

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
Concordance's clause register served and signatures admitted by
adjudicated reading (B2). Each phase is one commit:

1. **B0, replication by reconstruction** ([issue #13](https://github.com/SAIOS-SAMN/mycelium/issues/13),
   ruled 2026-11-02; **Built**): every hypha reconstructs each domain's
   state (founding, tiers, reserve, origin) from the founding receipts
   back-fill already delivers, checked against the signed chain, once by
   content and with the founding hypha's refusals; any client may reach
   any hypha, and the boundary holds at every hypha. Every later phase
   stands on it. Built in `src/server/cognition/founding.rs`: a
   back-filled founding receipt waits until a frame its signer signed at
   or above it is admitted. Claims in
   `server/cognition/tests/founding.rs`. Open: a founding inside an era
   folded before a hypha caught up, a version-2 origin's geometry (only
   its hash crosses), and the volatile waiting carriers (MU-HOSTING
   §5a.9).

   **Every reader of a back-filled receipt waits for a signature**
   ([issue #73](https://github.com/SAIOS-SAMN/mycelium/issues/73);
   **Built**). B0's founding queue is one signed-reach gate
   (`src/server/cognition/reach.rs`). A back-filled receipt is stored,
   since the catch-up links through it. It is marked unreached, and no
   reader acts on it until a frame its signer signed at or above it is
   admitted: not the depth index, the stamp alias, the ruling
   reconstruction it would wake, a fold's shed, a founding's reconstruction, the
   equivocation burn, or restore. Claims in
   `server/cognition/tests/reach.rs`. Its three open items are built
   ([issue #79](https://github.com/SAIOS-SAMN/mycelium/issues/79)): a
   signed frame that contradicts a stored unreached line evicts its
   unreached top and never a reached receipt; the held queue is bounded
   by the process's capacity `C = ⌊M_proc / b⌋` and refuses past it by
   name; and a back-filled `Fold` files its root once a signature reaches
   it. A gap longer than `C` closes through a fold root
   ([issue #88](https://github.com/SAIOS-SAMN/mycelium/issues/88);
   **Built**): a page stopped at the bound asks the range from below the
   signer's unreached floor, the root is admitted only as a
   `Reconstructed<RootProof>`, its signatures reach the receipt held at
   its place and every one below it, and receipts its shed takes off the
   chain leave the gate. No more than `C` receipts are held unreached
   while the gap closes. Claims in `server/cognition/tests/catchup.rs`
   (MU-HOSTING §5a.9). A fold root's signatures cover its place, height
   and hash ([issue #91](https://github.com/SAIOS-SAMN/mycelium/issues/91);
   **Built**, inside v0.4.0's break): each signed position signs the root
   written out, `"mycelium root v2" ‖ K ‖ i ‖ height ‖ hash`, and a root
   restated at another place or height is refused `REFUSE_RECEIPT` with
   no receipt held at that place. Claims
   `root_at_another_place_refuses_without_held_receipt`,
   `root_at_another_height_refuses`.
2. **B1, the act audit and the boundary's name:** every act kind that
   names a domain classed as a key's own or a universe's; the `NotReserve`
   variants renamed `NotThroughCore` (names only); a line's mouth offer
   (78.136) and a tier's handoff offer or traversal (78.126) carried in the
   reserve's act, after the operator rules on amending those two forward;
   `NotThroughCore` derived from the cones (an act arriving other than at
   its universe's vertex is not an event of that universe's cone at the
   hypha), never from a ruling on routes. **Built with B4** (v0.4.0): the
   reserve checks of the fold, anchor, map, core tube, conversion out,
   manifest and statement refuse `Boundary::NotThroughCore { act, domain }`
   on each act's reply, and their `NotReserve` codes are retired (claim
   `universe_act_off_the_core_refused_not_through_core`). Open: a line's
   mouth offer and a tier's handoff offer carried in the reserve's act.
3. **B2, signatories, clause by clause, by adjudicated reading**
   (Concordance §0, ruled 2026-11-04; **Built**, v0.4.0, a network-contract
   change: `ACT_LAWS` modes 6, 7 and 8):
   - Each clause is identified by its content (its law form and
     joint-read fixtures), never by a hash of the text's bytes. The
     register (`src/physics/laws/clauses/`) holds `FORMS-PASSAGE` v4 (the
     five fixtures of LAW-PASSAGE v3, eight passes) and `FORMS-COORDINATE` v3 (the
     orders of fixtures 1a and 11, eight sexagesimal sectors for display,
     seven `within` readings on stated bounds and three ranges on one
     component, of LAW-COORDINATE v3), with the hypha's own reconstruction of
     each fixture through `forms` (`ConcordanceClause`,
     `ConcordanceClauseId`, named apart from the rule language's clause).
     The passage is a network clause; the coordinate stands at kernel
     standing, signable and vetoable, until 78.173's fixtures are
     joint-read agreed and its clause is drafted. Mode 7 serves it.
   - A kernel signs a clause by presenting its reconstruction of the
     clause's fixtures (mode 8, `clauses::signing::Signing`). The hypha
     admits the signature only on the same exact objects, and a kernel
     that cannot read the clause is refused `NotSignatory`, naming the
     clause and the fixture; a vetoed network clause is refused
     `NetworkVetoed`.
   - `CONCORDANCE_HASH` (BLAKE3 over the file's bytes, computed in
     `build.rs`), the whole-text signature on mode 8, `build.rs` itself and
     the statement's hash are retired.
   - The SDK's kernel helper: `mycelium_sdk::signing` (`reconstruct_fixture`
     through `forms`, `sign_concordance`); the KERNEL profile's
     `laws concordance` and `laws signature` rows read the register and
     sign it by reconstruction, with a fixture presented wrong refused
     `NotSignatory`.
   - Claims (`physics::laws::clauses::tests`):
     `admit_admits_only_reconstructed_fixtures`,
     `admit_refuses_unreached_fixture_as_not_signatory`,
     `register_equals_forms_law_versions` (a wording change changes no
     clause), `admit_refuses_network_veto_and_names_kernel_veto`.
   - **The signings on the ledger** (**Built**, v0.4.0, a network-contract
     change: a new receipt citation and `ACT_LAWS` mode 9). Each signing
     act whose signature reconstructs is judged and sealed on the hypha's
     line as a `Domain` act citing `Cite::Signing` (tag 16: the kernel's
     key, the domain it is the sitting reserve of, the verdict), admitted
     or refused; a `NotSignatory` is held as the residue naming both
     sides; a verdict the line cannot seal answers `Unrecorded`
     (`Court::hear_signing`, MU-HOSTING §5a.4). The alignment reading
     (mode 9, `Court::alignment`, `clauses::read_alignment`) answers per
     kernel and clause version converses, vetoed, collides (with the
     residue) or unread, the latest event per clause, derived from the
     events at every hypha that holds them and never stored. v0.3.0's
     signings (netstratum and xylarium, Concordance version 4, by the
     retired text hash) are served beside it as history
     (`clauses::history`), never admitted. The KERNEL profile gains
     `laws alignment` (MUST). Claims: `server/cognition/tests/signatory.rs`
     (`signing_is_recorded_on_the_ledger_and_reads_converses`,
     `refused_signing_is_recorded_as_collision_with_residue`,
     `resigning_is_a_new_event_and_the_latest_reads`,
     `unverified_signing_is_recorded_nowhere`,
     `alignment_refuses_when_the_stroke_is_spent`,
     `signing_replicates_to_a_second_hypha`) and
     `physics::laws::clauses::tests::alignment`; the doc test
     `judge_takes_only_a_reconstructed_signing`.

   Then a domain's crossings refused `NotSignatory`
   until its reserve has signed every network clause of the version served
   (**Built** with B4, below); open: a tube between two universes carrying
   what both enabled, a vetoed clause's bound named, never a refusal of
   the whole tube; an act read under its clauses as signed, a changed
   clause's divergence held and named until the new version is signed;
   the signatory and shared-clause readings.
4. **B3, the universal laws at admission:** the transitions a crossing
   states verified under L0 at `LAW_VERSION` 5 and the `forms` core shell,
   refused `NotLawful` naming each breach, never admitted on a declared
   law's name; the draft-4 shared laws as each is ruled.
   - **Built** (v0.4.0, a network-contract change on the wire): a tier's
     fold carries the transitions it states (`TierFold::transitions`,
     written out in its signed bytes, never a digest of them), and
     `Reconstructed::fold` rebuilds each, and each of every carried
     line's, under L0 at `LAW_VERSION` (`verify_sized`) before the vertex
     admits it. A breach is refused `Uplinked::NotLawful` (the path of
     carried lines, the transition's index, every breach with its law); a
     law unread is answered `Uplinked::Unread`, neither admitted nor
     refused. Claims `crossing_refused_by_the_law_it_breaks`,
     `lawful_crossing_admitted` (`server/cognition/tests/boundary.rs`)
     and `fold_with_transitions_round_trips_with_lawful_answers`.
   - Crossing kinds with no law reconstruction yet, each listed, none
     invented: an anchor, a map, a mouth, a conversion out, a manifest and
     a statement state no transition; their existing checks stand
     (MU-HOSTING §5a.5). A motion stated as turns (`verify_turns`) has no
     wire writing yet and rides the crossing record (track 25, X1). No
     crossing states a core form's object. No crossing kind states a
     passage yet (a mouth's frames are read as versors of its spaces, not
     as a frame state `H·n_Z + R·n_Q`), so none is reconstructed under
     LAW-PASSAGE v3's `Poles::cross`; a crossing that states one is the
     admission claims below.
   - The first shared crossing law is **LAW-PASSAGE v3** (adopted by
     both operators 2026-11-05): the passage is the pole swap, the
     reflection in the throat's unit `e₊`. A frame state
     `H·n_Z + R·n_Q` (hold and reach, `n_Z = e₊ + e₋`, `n_Q = e₊ − e₋`)
     goes by `x ↦ e₊·x·e₊`, its span `H − R` reversed and its whole kept;
     the riding state goes one-sided, `q ↦ e₊·q`; one passage writes
     `T ↦ T − 2·(H − R)`; two passes are the identity; at a pole the
     passage is one-way ([proposal 78.172](docs/proposals/78.172-law-passage.md);
     requested by netstratum PR #40, derived in netstratum docs/114
     §9–§10 and docs/117 §10, §12). Version 2's product `q ↦ v_k·q` is
     retired as the passage; version 1's `R ↦ 2H − R` was struck on
     2026-11-05.
   - **What `forms` holds** (v0.4.0): `forms/src/passage.rs`, shell id
     `FORMS-PASSAGE` v4, the law whole. `Poles` holds a plane by its two
     units (`NotUnit`, `WrongSign`); `swap`, `cross` (to a `Crossing`
     with `S` and `T`) and `ride` are exact over the algebra
     (`algebra::Clifford`, a crate below both `forms` and the library),
     refusing a frame state off the plane `NotFrame`, a riding state of
     both parities `MixedParity`, and two signatures that disagree
     `Mismatched`. `Frame` reads hold, reach, whole, span, the face
     `δ = H/(H + R)` (`NoWhole`, `NotShare`) and the `Place` (throat,
     pole, interior). `Throat::verify` refuses a crossing that states a
     moved reach (`2H − R` and `H²/R` among them)
     `PassageRefusal::NotLawful`, carrying the `Breach`; `pass_share`
     reads the face from the other pole, `1 − δ`. The five fixtures and
     the roles' exchange are claims in `forms/src/tests/crossing.rs`;
     v2's algebra facts stand as claims on `Clifford::product`
     (`src/transducer/cga/tests/product.rs`).
   - **The signature is data** (v0.4.0; no wire or layout change): the
     multivector carries the mask of its negative generators beside its
     `n` (`Mv::negatives`), and the product reads each square from it.
     Every value of `Cl(n+1, 1)` carries `e₋` alone, its products
     unchanged (the oracle golden reproduces). A value of `Cl(p, q)` is a
     `transducer::cga::Clifford`, composed only through `product`, `sum`
     and `difference`, which refuse two signatures disagreeing on a
     generator both algebras hold `SignatureRefusal::Mismatched`. The
     wire carries `Cl(n+1, 1)` values as acts; a `Clifford` is written only
     as a clause fixture's input or object on `ACT_LAWS` modes 7 and 8
     (`clauses::writing::put_clifford`), where a kernel signs the passage
     by reconstructing them (B2). Open: the admission at the vertex,
     below.
   - Claims (admission, open, once a crossing states a passage):
     `crossing_is_the_pole_swap_with_its_flux_on_t`,
     `crossing_at_a_pole_is_one_way`,
     `crossing_stating_moved_reach_is_refused_not_lawful`.
   - The shared coordinate is **LAW-COORDINATE v1**, ruled by both
     operators on 2026-11-05 from netstratum's docs/120
     ([proposal 78.173](docs/proposals/78.173-law-coordinate.md),
     **Proposed**, at §3a step 2). A body stands at
     `origin / h : S:G:T:H : K`, each lane a vector of exact signed
     components keyed by axis (H's by generator, in degrees, `K` per
     component); the canonical order is the index a kernel addresses its
     bodies by; neighbours are a range on one component; `within` and the
     ranges decide on exact intervals the caller states, and base 60 is
     display only (ruled 2026-11-05). **What `forms` holds**:
     `forms/src/coordinate.rs` and its organs, shell id
     `FORMS-COORDINATE` v3 (v1 the order, writing and reading; v2 the
     advance, an event's bell, the merger and fission; v3 the keyed vector
     lanes, open lanes by ruled opening, the caller's intervals, the
     re-origin across a throat and dilation invariance), with fixtures
     1–16 in `forms/src/tests/`. The clause register serves v3 (a wire
     change to the coordinate's writing, `changes/coordinate-v3.md`).
     Next: rewire `chronometry::tuniversal::TPoint` as the form's
     one-component non-negative case (claimed today by writing and
     order), and the ruled act that opens a lane. Open: the
     signed-to-issuance map at the vertex, xylarium's lane renaming, and
     netstratum's composition law (fixture 11b).
   - **A new B3 crossing kind, the traversal, is proposed:** LAW-TRAVERSAL
     v1 ([proposal 78.176](docs/proposals/78.176-law-traversal.md),
     **Proposed**, agreed with netstratum), the inter-kernel crossing
     through the universal core tube, stated as two LAW-PASSAGE v3
     passages composed at the vertex — one out through the source
     universe's own throat, one in through the destination's — and
     reconstructed by the court as a LAW-COORDINATE v4 re-origin, read
     twice. Each kernel declares its polarity as data; the inter-kernel
     map (polar or antipolar) is one case of the law, never the law
     itself. The antipolar case exchanges a kind's frames,
     `(z_k, q_k) ↦ (q_k, z_k)`, keeping the whole `z_k + q_k` per plane
     (so 3Z and a nucleon mass are kept), the axis inversion `t ↦ 1/t`
     fixing only the throat. To build after 78.176 is agreed: open, the
     wire framing of a traversal, the kernel-pair polarity declaration's
     form, and the reading of alignment across the tube. Which kernel's
     universal core is the antipolar face of any tube is not confirmed;
     xylarium's operator rules it for xylarium's own tubes.
5. **B4, the boundary:** `NotThroughCore` and `NotSignatory` at the
   universal core tube; refusals at the tube by name and domain under
   `ACT_OBSERVE`, and the reconstruction events (Collapse, Converse,
   Collision) at the boundary, with track 8.
   - **Built** (v0.4.0, a network-contract change on the wire):
     `court::admission::Boundary` names the act (`UniverseAct`) and the
     domain on each universe act's reply. `NotThroughCore`: an act off its
     universe's sitting reserve (B1's rename, above). `NotSignatory`
     (`Court::signatory`, `server/cognition/boundary.rs`): a fold, anchor,
     map, mouth, conversion out, manifest or statement crosses only while
     the domain's sitting reserve converses on every network clause of the
     register served, read from `Court::alignment` on the act's stroke;
     it names the clause and the reading (unread, or colliding with the
     residue). Kernel clauses never refuse; a vetoed one bounds only the
     acts it governs (`Bounded`; the coordinate clause governs anchors and
     maps). An alignment cut short answers `Unaligned`. A key's own acts
     are never read. The SDK's KERNEL read signs again after its colliding
     signing; the house example designates its reserve and crosses with a
     lawful fold. Claims (`server/cognition/tests/boundary.rs`):
     `unsigned_reserve_crossing_refused_not_signatory`,
     `key_own_act_unbound_by_the_tube`,
     `kernel_clause_veto_bounds_only_its_clause`,
     `signed_reserve_lawful_crossing_admitted`.
   - Open: the boundary reading under `ACT_OBSERVE`; a tube between two
     universes bounded by both sides' clauses (a mouth's far-end
     acceptance is not read).
   - **Built** (#118): the per-crossing alignment read costs the reserve's
     signing events, never the ledger. The ledger keeps where each signing
     event stands (`kernel‖stamp‖n`, coordinates only,
     `chronometry::chain::signing`), and the court reads its own folded
     line's events once per era (`FoldedSignings`); the reading is derived
     on every ask. Claims `crossing_alignment_reads_only_signing_events`,
     `alignment_equals_full_walk_across_own_fold`,
     `alignment_equals_full_walk_on_a_peer`. Open: the stroke on a
     long-lived mesh, recorded as an observation (D2.5).
6. **B5, nested universes:** a sub-universe's acts only at its parent's
   vertex, with 78.155 R-3.

Draft 4 was approved 2026-11-05 and is written into the Concordance as
version 5; B3's shared laws and B2's clause sets under its §0 SIGNING
text proceed as engineering, no longer a ruling dependency. Issue #13
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
content (each body by its boundary, the motion as a versor or as turns,
each phase with its winding kept), its holonomy and its clauses. The hypha reconstructs the object
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
4. Build the phase, and hold it to
   [AGENTS › Definition of done](AGENTS.md#definition-of-done) before the
   pull request.
5. Register each guard the phase builds in `gate/src/drift.rs`
   (regression, ruling, guarding type, claim); the gate refuses an entry
   whose type or claim is missing.
6. Open the pull request from the template; CI's `check` runs the
   definition of done on it. A
   phase that changes the wire or the ledger layout says so under
   *Network contract*.

Each phase lands on its own branch, one item per commit:

1. **X0, the drift guards.** **Built**, first stage (no wire or ledger
   change).
   - The digest compared at admission is typed by its sense
     (`court::admission::digest`): `Locator` (address), `Predecessor`
     (link; `Link` already names the sentence link) and `Commitment`.
     None implements `PartialEq`. The comparisons are
     `Locator::locates`, `Predecessor::continues` (with `forks` and
     `cites`), and `Commitment::binds(&Reconstructed<T>)`. Converted:
     the receipt line's checks (`admit_chain`), the signed prefix's
     claim, the equivocation proof, the signed-reach gate, the ledger's
     `holds`, `height_of` and era walk, the back-fill chase, a folded
     citation's recall, and the endowment's manifests.
   - `Reconstructed<T>` has a private field, built only by a law's
     reconstruction (`Reconstructed::receipt`, `Reconstructed::receipts`).
     The receipt path's admissions take it: `admission::admit`,
     `admit_backfilled`, `admit_equivocation`, `Court::admit_receipt`,
     `Court::admit_backfill`, `ReceiptChain::admit_unreached`. The
     founding outcome once named `Reconstructed` is `FoundingEvent`.
   - The register `gate/src/drift.rs` lists each guard (regression,
     ruling, guarding type, claim). The gate refuses an entry whose type
     or claim is missing.
   - Claims: `address_has_no_equality`, `predecessor_has_no_equality`,
     `commitment_has_no_equality`, `reconstructed_is_built_only_by_the_laws`
     (`compile_fail`); `commitment_binds_only_a_reconstructed_object`;
     the gate's `drift_refuses_entry_missing_its_claim`.
   - **Built**, second stage: the record fields typed by sense.
     `Parent::hash`, `KStepReceipt::chain_hash`, a fold `Root::hash` and
     `Admitted::Equivocation`'s two hashes are `Predecessor`;
     `SignedPrefix::prev_receipt_hash` (and `KStepReceipt::prev_hash`)
     is `Commitment`. `Parent`, `KStepReceipt`, `SignedPrefix`, `Root`,
     `Era`, `Admitted` and the fold `Proposal` lose `PartialEq`; a
     receipt is compared by `Predecessor::cites` over `Parent::of`, a
     root by `Root::restates`. The packed line's anchors and
     predecessors (`chronometry/pack.rs`) and the archive's era
     comparisons (`cognition/state.rs`, `Locator`) moved with it.
   - **Built**, second stage: `ReceiptChain::admit` takes a
     `Reconstructed<KStepReceipt>`, and the vertex's admissions take a
     reconstructed act: `Court::admit_uplink` and `admit_chained` a
     `Reconstructed<SignedFold>`, `Court::admit_domain` a
     `Reconstructed<SignedDomain>` (`Reconstructed::fold`,
     `Reconstructed::domain`: the signed bytes re-written from the act's
     own coordinates, the signature verified over them).
     `Court::hear_uplink` and `Court::hear_domain` reconstruct the act as
     heard. Claim: `vertex_admits_only_a_reconstructed_fold`
     (`compile_fail`), in the drift register.
   - **Built**, second stage: the stratum root in `court/glue.rs`
     (`locates_stratum`) and the starved root in
     `client/sdk/mind/learn.rs` (`CourtPace::starved` holds a `Locator`)
     are compared in the address sense.
   - Registered with T3 (v0.4.0, #26): the guard of MU-ANTI-FLATTENING
     clause 12. `Turn` has private fields and one constructor,
     `Turn::of(r, h)`, proved by the `compile_fail` claim
     `turn_has_one_constructor`.
   - **Built**, fourth stage (ruled on [#28](https://github.com/SAIOS-SAMN/mycelium/issues/28),
     2026-11-06, option (a)): the guard of clause 14. A restore is
     `Court::reconstruct(self, &[u8]) -> Result<Reconstructed<Court>,
     RestoreRefusal>`, a constructor in place of the in-place
     `restore -> bool`, which is removed. `Reconstructed::court` reads
     the snapshot under its laws: each receipt re-derives, the line
     verifies on its root, a held line holds the stamp the snapshot
     names, each row continues its receipt, each word re-derives its
     citation. `self` is the court holding its line and eras from disk,
     the evidence the snapshot is read against. Every call site and the
     server boot are migrated, with no legacy path. Claim
     `restore_reconstructs_and_checks_receipts`, in the drift register.
   - **Built**, third stage ([#28](https://github.com/SAIOS-SAMN/mycelium/issues/28)):
     `Cite::Root`'s hash is a `Predecessor`, so `Cite` has no
     `PartialEq`; two citations are compared by `Cite::restates` (two
     roots in the link sense, any other pair by its canonical writing).
     The stratum root is typed at its source: `StratumDigest::root` is a
     `Locator` (`sheaf::stratum_root`), compared by `Locator::locates`,
     and a digests reply naming a root that is not 32 bytes is refused
     whole. Claims `cite_has_no_equality`,
     `stratum_digest_has_no_equality` (`compile_fail`, in the drift
     register), `digests_reply_refuses_root_of_another_width`.
   - **Retired** (ruled on #28, 2026-11-06): the report of changed
     document sentences that put a typed term beside an admission verb.
     A report that never refuses and that no reader acts on is theatre,
     the reason the mutation audit was retired (*Test assurance* item 9);
     the gate's heading and link checks guard document sentences.
   - The issue's mutation-audit entry is covered by the `compile_fail`
     claims `reconstructed_is_built_only_by_the_laws` and
     `vertex_admits_only_a_reconstructed_fold` (ruled on #28), as the
     drift register notes.
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
   - `src/client/sdk/mind/learn.rs` (a starved root): an address.
   - Each peer kernel tracks its counterparts in its own repository.
6. **X5, the transduction in place of the common grid.** Two frames
   relate by the motion between them; each keeps its own resolution;
   each body is held by its boundary; incidences are read between boundary arcs; a site
   lifts as turns of its frame's own periods, the angle `360°·s/e` on a
   named plane, and a composed motion keeps its winding beside its phase
   (LAW-FIELD amendment (c)). The transduction is **Built** (the v0.3.0
   transduction work, [#25](https://github.com/SAIOS-SAMN/mycelium/issues/25),
   closed): `src/transducer/family/refine.rs` was removed with
   `CommonRefinement`. X5's guard remains to land, so that no common refinement
   re-enters past the sensor.
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
whole; the claims first; the
[Definition of done](AGENTS.md#definition-of-done); one commit; a pull
request that names #32).

Each phase is one commit:

1. **S0, the sample.** A `Sample { rank, extents, symbols, place }` carrier
   in `src/transducer/` is the sensor's only input. Symbols are compared by
   equality, at the source's own width. `Site` already carries any rank.
   - Claims: `sensor_reads_rank_from_sample`,
     `unread_dimension_carries_no_value`.
   - **Built** (`src/transducer/sample.rs`: `Sample`, `Symbol`, `Place`).
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
   - **Built.** The decoder is the workspace crate `sources/arc-agi`
     (`mycelium-source-arc-agi`, binary `mycelium-arc-agi`), with the
     corpus tests (the bridge sweep, the retract claims, the rule
     language's corpus tasks, the oracle's golden file). The hypha, the
     soak and a client's feed read the posted task
     (`dialogue::encode_posted_task`), the envelope's codec with the
     withheld outputs, which the decoder writes; task addresses are
     unchanged. The raster's readings moved to `transducer::raster`, the
     envelopes' dilation beside the fibre map, `Frame` beside
     `from_canonical_framed`, the magnitudes to `family::magnitude`;
     `frame_ratio` had no caller and is deleted. Each moved reading is
     listed as a defect in MU-SENSING §8.
3. **S2, the ground deduced.** The ground is the reading of MU-SENSING
   §3.4, held per family, with candidates in superposition. The default
   `0` and the literal `h != 0` sites (about 64) are removed.
   - Claim: `ground_deduced_not_symbol_zero`.
   - **Built.** It removed the two scope defects of MU-SENSING §6.1: the
     address's `symbol ≠ 0` filter and the frame fixing before
     addressing. `transduce::Ground` (`One`, `Vortex`, `None`) is read by
     `Ground::of` over a family's samples. A candidate is:
     - ambient: in every sample;
     - enclosing: it meets every sample's seam;
     - never carried as a body by a pair's motion (read from the
       transduction's bodies under the signed permutations).

     The details:
     - The scoped carrier stays (`raster::on_ground_of`). With no family
       open it reads `Ground::None`, and a writer is refused
       `Unground::Unscoped`.
     - Readers ask `is_ground`. Writers that need one symbol (`void`) are
       refused by name under a vortex or `None`.
     - Scope separation (ruled 2026-11-03):
       - A task's identity is its posted samples' content (rank, extents,
         symbols, places). Neither the ground nor frame fixing enters it.
       - The decoder posts tasks as read, and `parse_task` is removed.
       - Frame fixing (`raster::enclosure_fixed_on_ground`, only under `One`)
         is a perspective no reader applies at intake. It becomes one in
         S6.
       - All 1 000 training addresses change, inside v0.3.0's one break.
     - Claims: `task_identity_reads_no_ground`,
       `task_identity_reads_no_frame_fixing`,
       `ground_deduced_not_symbol_zero`,
       `ground_requires_presence_in_every_input`,
       `ground_requires_meeting_every_seam`,
       `ground_carried_as_a_body_is_not_ground`,
       `painting_onto_the_ground_keeps_it_ground`,
       `several_candidates_are_held_as_a_vortex`,
       `no_candidate_reads_no_ground`,
       `unscoped_reader_never_defaults_to_zero`.
     - Behaviour over the 1 000 training tasks: 466 read the former
       symbol, 72 another, 195 a vortex, 267 none (MU-SENSING §8).
     - Corrected conditions (ruled 2026-11-04):
       - A ground is present in every input sample.
       - A framed interior is enclosed through its membrane.
     - A vortex is read once per candidate, and the mind speaks one claim
       per candidate. The solve act states the ground its claim was
       spoken under, and the court reconstructs under it (MU-SENSING
       §6.2). This is a wire change inside v0.3.0's break.
     - Claims: `vortex_ground_read_once_per_candidate`,
       `claim_states_ground_and_court_reconstructs_under_ground`,
       `claim_under_a_non_candidate_ground_is_refused`,
       `solve_round_trips_stated_ground`,
       `ground_absent_from_an_output_is_still_ground`,
       `framed_interior_is_ground_and_frame_is_membrane`,
       `always_framed_interior_holding_bodies_joins_the_vortex`,
       `interior_holding_no_body_is_not_ground`,
       `membrane_is_held_and_membrane_change_is_read`.
     - Open: collapsing a vortex by a relation, with S6.
4. **S3, the symbol set deduced.**
   - `PALETTE` (10, and the conflicting 256), `RESERVOIR` and the 10-slot
     hue tables are replaced by the family's own symbol set `Σ`
     (MU-SENSING §3.3). An index into `Σ` is the order of first
     appearance in address order, and a table over symbols is `Keyed`
     and sized `|Σ|`.
   - `e_∅` is at index `|Σ|`.
   - Colour words are fibre relations.
   - Claims: `percept_invariant_under_symbol_permutation`,
     `symbol_index_is_first_appearance_in_address_order`,
     `reservoir_follows_the_family_symbol_set`.
   - Reflective claim (MU-SENSING §0): `mind_states_a_symbol_meaning_as_a_relation_and_court_reconstructs_it`.
   - **Built.** `transducer::symbols::SymbolSet` reads `Σ` from a
     family's samples or rasters, with `index` (first appearance) and
     `reservoir` (`|Σ|`); `element::fibre_on_symbols` reads a fibre
     relation on `Σ`'s indices, each record at `|Σ|`. Both `PALETTE`s,
     the 10-slot `HueTable`, the 256-slot `HueMap` and the `[u8; 10]`
     images are gone: each table is keyed by the symbols it names, and
     permutations, cycles and orbits are read over those symbols. The
     erasure act maps the family's own `Σ` onto its ground; the rule
     language composes hue maps over the painted symbols they name.
     - `Σ` is over all of a family's posted samples, its test inputs
       included (`SymbolSet::of_family`, `symbols::on_test_inputs`).
     - Claims: the three above,
       `family_with_eleven_symbols_is_read_whole` and
       `symbol_seen_only_in_a_test_input_is_in_the_family_set`.
     - The reflective claim is written against the existing colour-word
       path: the mind states the fibre relation as a colour word, and the
       court reconstructs it from its own copy of the family. No wire
       field was added.
     - Behaviour on symbols `0`–`9`: task addresses and the oracle's
       1 901 golden readings are unchanged (MU-SENSING §8).
     - The tape's record byte (`RESERVOIR = 10` beside one-byte
       symbols, so a family whose symbols held the byte 10 had no colour
       word) was closed in S5: `e_∅` is a typed record.
5. **S4, rank and roles deduced.**
   - `GRID_AXES`, `FRAME_AXIS`, `PLANE_ROW`/`PLANE_COL`, `GRID_N`, the
     3-tuple `Point`, `BASE_AXES` and the planar `(rows, cols)` carriers
     are replaced by the sample's rank and a perspective's roles.
   - The transduction's `Enclosure` holds `n` phases, none of
     them a row or a column: the axes are lifted into the field
     (MU-SENSING §3.6).
   - An axis is taken apart into nested phases where the family is
     invariant under the factoring, with candidates held in superposition.
     The nesting is a coordinate of the percept.
   - Connection is read by grade (MU-RECURSION 71.2 (2)).
   - Claims: `rank_one_and_rank_three_samples_perceived_by_one_path`,
     `no_axis_is_privileged` (the percept is invariant under a
     relabelling of the axes), and
     `axis_taken_apart_only_where_family_invariant`.
   - **Built** with T2 ([PR #41](https://github.com/SAIOS-SAMN/mycelium/pull/41)):
     the transduction at rank `n` (`src/transducer/family/transduce/`) and
     the nested map (`nest.rs`). The reading of which factorings the
     family is invariant under is built (branch `sensing-nest`; ruled
     2026-11-06 on #32): per pair, separators as the fine phase of a period
     `b + 1`, repeating blocks as relations, motions per level, collapse by
     a reading that holds for one candidate alone, several alike
     superposed; the nesting a coordinate of the percept
     (`transduce::nesting`, `TaskPercept.nesting`). Claims
     `axis_taken_apart_only_where_family_invariant`,
     `factoring_read_per_pair_and_kept_only_where_every_pair_admits`,
     `separator_is_the_fine_phase_of_period_b_plus_one`,
     `repeating_blocks_kept_as_relation_by_fine_motion`,
     `motion_per_level_is_coarse_times_fine`,
     `several_factorings_holding_alike_stay_superposed`. Nesting as a
     fibration of levels in the field's lift stays with S4a stage 4.
     The fixed axis constants above are retired in S4b: a perspective
     assigns the roles (`transducer::perspective`), and `BASE_AXES` is
     derived from the alphabet's slots.
   - **Built** (branch `sensing-dilation`): the envelopes' dilation, which
     squared the extents (law 11), is removed; the fibre map's carried
     site and the frame's momentum read the re-period of each phase
     (`element::Reperiod`, `p′ᵢ/pᵢ` axis by axis). Claims
     `reperiod_carries_sites_between_extents_phase_by_phase`,
     `momentum_read_per_phase_across_anisotropic_reperiod`.
   - **Built** (branch `sensing-magnitude`; ruled 2026-11-06 on #32): a
     family's magnitude is read at the samples' own rank, S and G from
     each pair's projected record and H from the boundary by grade
     `n − 1` (the chain-length term retired); a pair whose record does not
     read is held as residue citing its refusal (`OffSite`), contributing
     nothing. The genesis bond and the mind's `family_magnitude` read
     alike; the bond's value changes, a recut. Claims
     `magnitude_reads_counts_at_rank_n_and_h_by_grade_n_minus_one`,
     `pair_of_unequal_enclosure_is_held_as_residue_citing_off_site`,
     `scaled_pair_is_measured_refused_pair_is_residue` (a projection that
     lands resolves an unequal enclosure; confirmed on #32).

   **S4a, the sample lifted into the field** (MU-SENSING §3.7; in the
   same commit series as S4, after LAW-FIELD v1 is agreed with the peer
   kernels):
   - Each body stands on its charts, and each site is held by its phases
     within them, with the scale a gauge. There is no unit norm and no
     sum of squares. The transitions between charts are versors of
     `Cl(n+1, 1)`, so the geometry is non-Euclidean.
   - Nesting is a fibration (25.12: base the class, fibre the gauge). A
     nested axis adds a field within the field.
   - The holonomy of an orbit of perception is the product of its
     transitions over one lap through the charts, read by its modes: an
     exact turn in ℚ/ℤ where elliptic. Motions by rational turns are exact over
     `ℚ(ζ_ℓ)`.
   - One field carrier serves perception and the physics' shells
     (MU-TOPOLOGY §6, Concordance 2.10; MU-RECURSION D4).
   - Claims: `sample_lifts_into_the_field_without_unit_norm`,
     `orbit_holonomy_is_the_product_of_its_transitions_per_lap`,
     `flat_product_reads_no_holonomy_where_curved_field_reads_it`,
     `orbit_precesses_by_the_holonomy_its_attractors_curve`,
     `flat_region_binds_no_orbit_refused_unbound` (restated from
     `zero_precession_orbit_refused_as_loop` under 78.171 amendment (f):
     a flat region binds no orbit, refused `Unbound`),
     `closed_geodesic_with_zero_precession_is_resonant` (zero precession
     in a curved field is a resonant orbit, never refused),
     `windings_kept_as_fibre_classes`,
     `rotation_by_rational_turn_exact_over_cyclotomic_field`.
   - **Built, stage 1** (branch `field-s7`; built against LAW-FIELD v1 as
     proposed, `src/transducer/field/carrier/`). A sample of any rank is
     lifted (`Lifted::of`): each body on its chart (rank, measure,
     boundary faces of grade `n − 1`, coarsest first), each band's two
     corners as points of one phase per axis, exact in degrees on the
     turning plane `e₊ᵢe₊₍ₙ₊₁₎` with its winding (`Turning`), on `n + 1`
     frame planes of `Cl(p, q)` read per plane (`FramePlanes`), the
     sample's membrane the throat of its own plane. A turning's motion is
     `ζ_ℓ^t` in `ℚ(ζ_ℓ)`, reduced by `Φ_ℓ` (`Cyclotomic`), never a
     trigonometric value. The claims above stand as:
     `lift_holds_one_phase_per_axis_on_turning_planes` (without unit
     norm), `precession_of_three_thirty_degree_steps_equals_ninety_degrees`
     (holonomy as the product per revolution), `atlas_revolution_reads_holonomy_only_where_orbit_held`
     (flat against curved, precession by the attractors' curvature),
     `flat_region_binds_no_orbit_refused_unbound` (named
     `precession_refuses_flat_path_as_loop` before amendment (f)),
     `five_seventy_two_degree_turnings_keep_winding_one` and
     `lift_keeps_far_seam_as_whole_turn_with_winding_one` (windings
     kept), `turning_then_agrees_with_motion_product` (exact over the
     cyclotomic field). Open for stage 4: lifting a family's pairs as
     transitions between their bodies' charts, and nesting as a fibration
     of levels in the lift.
6. **S5, samples on the wire.**
   - A task carries samples of rank `n`: `n`, the extents, then the
     symbols at the source's width.
   - A link carries its site's coordinates of rank `n`.
   - The SDK's `task` module follows. This is a wire change.
   - `e_∅` travels as a typed record tag outside the symbol bytes, in
     words' descriptions and the law codec (wire change, v0.3.0's
     break). Until then `RESERVOIR = 10` is the tagged format byte, and a
     family whose symbols hold the byte 10 has no colour word
     (MU-SENSING §8).
   - Speech acts and the seat's solve call state the ground their claim
     was spoken under, as the solve act does since S2 (MU-SENSING §6.2).
     Until then the court's speech path reads the family's deduced
     ground.
   - Limits are derived (MU-SENSING §6): the rank and extents use the
     `u16` field and are refused past it, never widened; the symbol width
     is the source's; the symbol bytes `w · ∏ eᵢ` are checked; a phase's
     denominator is its extent.
   - Claims: `task_round_trips_samples_of_rank_n`,
     `extent_past_its_field_is_refused_never_widened`,
     `symbol_bytes_refuses_overflow`.
   - **Built**, inside v0.3.0's break; `WIRE_VERSION` stays `0x0200`
     (no tag carries it yet: v0.3.0-rc.2 is `0x0104`), and the solve act
     is signed as `mycelium solve v3`.
     - Samples on the wire (`dialogue::samples`): `[rank u16][extent
       u16 · rank][width u16][place count u16][coordinate i64 ·
       count][symbols]`, each at its position `[split, pair, member]`.
       The envelope, the posted task and a solve's outputs carry them;
       `push_grid`, `read_grid`, `encode_grids` and `decode_grids` are
       gone. Limits are refused by name (`SampleWire`); the symbol bytes
       are counted with checked arithmetic. The mind and the court read
       rank 2 at width 1 (`TaskEnvelope::rasters`, `NotRaster`) until S4.
     - Task identity (`registry::task_address_of`) writes each posted
       sample's rank, extents, width, place and symbols; `task_address`
       already wrote rank and width since S2, so no address changes.
       `Enclosed::put_content` now writes its rank and width.
     - Speech and the seat's solve call state their ground
       (`Spoken.ground`, `SpeechAct.grounds`, `solve_args`); the court
       hears each sentence under it, refused by name where it is no
       candidate, and deduces none on the speech path (MU-SENSING §6.2).
     - `e_∅` is a typed record (`element::FibreRelation`) on a word's
       description and the law register's wire (`Transition.hues`);
       `RESERVOIR = 10` is retired.
     - N7 (track 11) landed with it: `ACT_ENTRIES`,
       `REFUSE_RULE_OFF_SUBSTRATE`, the `FAIL_*` strings.
     - Claims: the three above, `task_identity_writes_rank_and_width`,
       `speech_states_its_ground_and_court_reconstructs_under_it`,
       `family_using_symbol_ten_reads_its_colour_word`,
       `entries_act_round_trips`.
     - **Stopped:** a link carrying rank-`n` coordinates
       (`link_carries_rank_n_coordinates`). The three `Link` aliases are
       one type (`sentence::Link`), but the link is still `(i64, i64, u8,
       u8)`: the transducer's chain, chart (`stalk::invariant`) and field
       carriers read its two coordinates in about 40 files, so the
       rank-`n` link is S4's work on those carriers, not a contained wire
       change.
7. **S6, perspective.** A shift of perspective is a sensor motion. The
   perspectives of one sample are a cover, checked by the cocycle of
   MU-CLIENT 24.4. A mind may ask for a perspective as a demand.
   - Claim: `perspective_shift_is_a_sensor_motion`.
   - A claim states the perspective it was perceived under, and the court
     reconstructs under it, as S2 does for the ground (MU-SENSING §6.1,
     §6.2).
   - Reflective claims: `claim_states_its_perspective_and_court_reconstructs_under_it`,
     `two_perspectives_of_one_sample_compared_as_a_cover`.
   - **Built, stage 1** (branch `sensing-s6`, PR #167): a perspective
     holds its pose, the signed permutation the sensor reads through,
     beside its roles; a shift (`SensorMotion::Pose`, `Recast`) moves the
     sensor, and a sample is read through it, never rewritten
     (`Perspective::read`, `turn_to`). The perspectives of one sample are a
     cover (`perspective::cover`): contacts as 0-cochains, the turn on each
     overlap, the cocycle on triples, the residue of contacts held. The
     frame fixing is a perspective (`raster::frame_fixing`). Claims
     `perspective_shift_is_a_sensor_motion`,
     `two_perspectives_of_one_sample_compared_as_a_cover`,
     `frame_fixing_is_a_perspective_reading`. No wire or address change.
     Ruled 2026-11-06 (78.180 ruling 6), to build: a mind asks for a
     perspective as a demand, a typed vertex naming the sample and the
     overlap where the cocycle fails for a missing chart; a ground's vortex
     collapses as the nesting's does, by Stands or Painted on holding for
     one kept candidate alone on every pair, never by Closes (claims
     `ground_vortex_collapses_where_bodies_stand_on_one_candidate`,
     `ground_vortex_collapses_where_residue_paints_one_candidate`,
     `closing_rule_never_collapses_the_ground`).
   - **Built, stage 2** (branch `sensing-s6-claim`, a wire change inside
     v0.4.0's break): a claim of factorisation states the perspective it
     was perceived under (signed `mycelium factorisation v2`); the court
     reads its copy of the pair through it and reconstructs there, refusing
     another rank or a non-space role by name; the mind perceives under a
     perspective (`TaskPercept::under`). Claim
     `claim_states_its_perspective_and_court_reconstructs_under_it`.
     Ruled 2026-11-06 (78.180 ruling 6), to build: the solve of outputs and
     speech state their perspective, and a court reconstructs under it
     (today they read the section in its default).
8. **S7, the field's curvature and the well's body** (MU-SENSING §3.7;
   LAW-FIELD v1):
   - The field's curvature is read from the orbits it holds, flat where
     nothing is held.
   - The existing `Well` (`transducer/field/well.rs`) gains its body:
     its membrane where it encloses, its charts in the field's atlas, its centre
     and its inflation as a gauge.
   - Its stage becomes a coordinate. `percept: bool` and `dirty: bool`
     go.
   - Its `volume` and `potential` (`[u64; 4]` counts) are replaced by
     readings of the body, never scalars.
   - The Euclidean spheres (`cga/embedding.rs` `sphere_at`, `on_sphere`;
     `cga/at_three.rs` `sphere`), built from `r²` with `e∞`, are retired
     for the field's charts and the membrane.
   - Claims: `well_stage_is_a_coordinate`, `well_reads_no_scalar`,
     `sphere_built_without_squared_radius`.
   - **Built, stage 1** (with S4a's stage 1): the atlas (`Atlas`,
     `Chart`), its connection the transitions of the orbits held
     (`Atlas::connection`, flat where none is held), attractors with their
     local orbits and membranes (`Attractor`), curvature as the holonomy
     of an orbit's transitions per revolution (`Orbit::holonomy`,
     `Atlas::curvature`), read per named plane in degrees with its
     winding and order (`Holonomy`), monodromy flat, torsion or rupture,
     and the membrane on the pole basis (`Membrane`, `stand`). LAW-FIELD
     v1's six fixtures stand as claims
     (`src/transducer/field/carrier/tests/law_field.rs`). Turnings on two
     planes that share one unit compose by fixture 11b's law
     (`carrier::Rotor`, 78.173 §5) where both motions are rational, and
     any other angle is refused `NotExact` with `IrrationalMotion`: the
     route approved 2026-11-06, with no `ℚ(ζ_ℓ)` product and neither
     degree convention changed (the factor of two is open in 78.173
     §8.3); claims in `src/transducer/field/carrier/tests/rotor.rs`.
   - **Stages** (each about 1 500 changed lines or less, each leaving the
     tree green; planned 2026-11-06):
     1. Built: the lift and the field's carrier (S4a, S7), and the field
        stated as an object a court reconstructs (`Atlas::put_statement`,
        `reconstruct`, `admit`: the statement writes the atlas and no
        reading; the court rebuilds it through the carrier's
        constructors, derives the curvature and admits a claimed
        curvature on a match, refusing `Differs` with each pair).
     2. Built: the well's body (S7). `Well` stands on the carrier
        (`carrier::WellBody`): its centre an attractor at the chart its
        remainder stands on (`carrier::Chart` now holds an invariant's
        exact coordinates, a count or a ratio each), its local orbit
        (`Well.orbit`, none held flat), its membrane its own enclosure,
        its inflation the gauge of its frame state on the membrane's pole
        basis; its stage the coordinate `Well.stage` (`WellStage`:
        `WellLayer`, `ExpansionRing`;
        `percept: bool` and `dirty: bool` retired); `volume` and
        `potential` readings per lane, never stored; holonomy and
        precession per revolution read from the orbit it binds (none:
        flat, refused `Unbound`). Every caller reads them
        (hold, merge, the expansion ring, the doors, the docket, the
        cross-scene meet, the observer, the quote). The well is stated as
        its body and a court reconstructs it and derives its curvature
        (`Well::put_statement`, `reconstruct_well`, `admit_well`). **A
        recut:** the field statement's layout changed (charts written as
        invariant coordinates, a well's stage after its atlas) and the
        well's carrier changed; no migration and no legacy reader (the
        dev-state wipe ruling). Claims `well_stands_on_its_attractor`,
        `well_membrane_is_its_enclosure`,
        `well_volume_and_potential_are_readings_per_lane`,
        `well_stage_is_a_coordinate_not_a_flag`,
        `mind_well_stated_and_reconstructed_by_a_court`. With it, the
        operator's ruling of 2026-11-06 (78.171 amendment (f)): every
        path is a geodesic of the field's gravitational curvature; a bound
        geodesic is an orbit read per revolution; zero precession in a
        curved field is resonant (`Precession::Resonant`); a path through
        a flat region binds no orbit, refused `Unbound`; `Loop` and
        "lap" retired. Claims `flat_region_binds_no_orbit_refused_unbound`
        (fixture 2 restated), `closed_geodesic_with_zero_precession_is_resonant`.
     3. The Euclidean spheres retired (S7): `sphere_at`, `on_sphere` and
        `at_three::sphere` replaced by charts and the membrane at every
        caller; and turnings on planes that share a unit composed exactly,
        a versor over `ℚ(ζ_ℓ) ⊗ Cl(p, q)`, its angle a named residue
        where no rational degree holds it. Claim
        `sphere_built_without_squared_radius`.
        **Built in part** (branch `field-s3-membrane`): a sphere is the
        membrane `−e₊` or its image under the dilation by a gauge and the
        slide to a centre (`cga::membrane`, `cga::membrane_at`), read by
        the horizon, the chart's inversion, the house's nuclei, the
        conformance probes and the oracle's fixtures; `on_sphere` and
        `at_three::sphere` retired; the SDK's `house::sphere_at` is
        `house::membrane_at`. Claim `sphere_built_without_squared_radius`.
        A body's nucleus is its exact second moment, a reading (ruled on
        #134, 2026-11-06), written as the dual-sphere form
        `c + ½(c² − r²)e_∞ + e_o` from `r²` alone (`blade::nucleus_form`);
        `cga::sphere_at` is retired (branch `field-nucleus-moment`; claim
        `nucleus_is_the_second_moment_read_never_a_sphere`; the values are
        unchanged, so no recut). Turnings on planes that share a unit
        compose by netstratum's composition law (78.173 fixture 11b,
        `carrier::Rotor`) where both motions are rational, a whole number
        of quarter turns; any other angle is refused `IrrationalMotion`,
        and a wound pair `Uncommuted` (78.173 §8.3).
     4. The rest of S4a: a family's pairs lifted as transitions between
        their bodies' charts (the factorisation's motion), and nesting as
        a fibration of levels where the family is invariant.
        **Built in part** (branch `field-s4-transitions`, stacked on
        `field-s8-motions`): on the lift's convention agreed 2026-11-06
        (78.171 §8; netstratum docs/117 §11.6), the phase measured from
        the shared lift unit `e₊₍ₙ₊₁₎`, every signed permutation and its
        re-period lifts whole as a motion of the field
        (`carrier::motion_of`: a reversal the flip of `e₊ᵢ` and one whole
        turning on its plane, the winding the transduction's `p − s`
        writes; a transposition `(i j)` the 90° turning on `e₊ᵢe₊ⱼ` then
        the flip of `e₊ᵢ`), and a pair is lifted as transitions between
        its bodies' charts (`carrier::transitions_of`, `PairTransition`; a
        body carried split or merged keeps its refusal). `carry_point`
        reads the phase origin so. No degree, chart or address moves, so
        no recut. Claims
        `every_signed_permutation_lifts_whole_as_a_motion_of_the_field`,
        `reversal_is_the_reflection_in_its_own_unit_alone`,
        `reversal_lifts_as_flip_and_whole_turning_matching_p_minus_s`,
        `pair_lifted_as_transitions_between_its_bodies_charts`.
        The nesting as a fibration of levels (branch `field-s4-fibration`):
        an axis the family's nesting takes apart (`Taken::Apart`, the
        reading ruled on #32 and built in #176) is a level
        (`carrier::Level`), its base the coarse phase on the axis's own
        plane, one point per block, and over each base point the fibre
        the block's own field, lifted with its own atlas, the separator
        left out; a sample lifts as its whole lift and its levels
        (`carrier::Fibred::of`); an axis kept whole or held superposed is
        no level. Claims `axis_taken_apart_lifts_as_a_fibration_of_levels`,
        `axis_kept_whole_is_no_level`. Stage 4 is then built.
     5. S8, the field's motions as one group: the turning, the reflection
        and the pole swap (`forms::passage::Poles::swap`) and the
        inversion in the membrane, the boost on a frame plane, the
        re-period; `NotExact` for a motion not carried.
     6. S9a: `cech::orbit::Field`'s orbits attached to wells as their
        local orbits; shear and precession per revolution in the
        contemplation, resonance read and an unbound path refused there;
        the court's pressure on the well it faces.
     7. S9b: axiomatic gravity, the drag on the orthogonal class
        (disjoint turning planes, `Meeting::Disjoint`).
     8. S9c: the vortex's monodromy read between charts, and a mind's
        stated holonomy, precession and monodromy reconstructed by a
        court on the wire (a wire change, stated as a Network contract;
        track 10's claims follow).
9. **S8, the field's motions as one exact group:**
   - rotate (a rotor by turns, cyclotomic);
   - flip (a reflection, or an inversion in the membrane);
   - deflate and re-inflate (a hyperbolic versor: a boost or a
     dilation);
   - stretch and thin (a re-period of one phase).

   A motion that cannot be carried exactly is refused `NotExact`.
   - Claims: `field_stays_smooth_under_every_isometry`,
     `stretch_is_a_re_period_not_a_versor`,
     `inversion_in_the_membrane_needs_no_infinity`.
   - **Built** (branch `field-s8-motions`, track 26 S7 stage 5;
     `src/transducer/field/carrier/motions.rs`): the generators
     `Generator::{Turn, Flip, Boost, RePeriod}` (the pole swap, the
     inversion in the membrane, is the flip of `e₋ₖ`, `Generator::swap`,
     `Membrane::invert`); a motion the reduced word of its generators
     (`FieldMotion`, with `then` and `inverse`); acting exactly on
     turnings, motions and orbits by conjugation, on frame states and on
     points, and refused `NotExact` with `Residue::Unnamed` where a plane
     is carried onto no named plane. The three claims above and
     `field_motion_and_its_inverse_compose_to_the_identity`. With it,
     LAW-FIELD fixture 2b (netstratum's request): flat and resonant
     separated by the kept winding (`Precession::Resonant` keeps its
     holonomy; claim `flat_and_resonant_separated_by_kept_winding`).
     Open: turnings sharing a unit at other than a quarter turn (78.173
     fixture 11b, netstratum's composition law).
10. **S9, forces, orbits and the vortex in the field's atlas:**
    - Axiomatic gravity is read as the local orbit's shear on the
      orthogonal orbit class. Two classes are orthogonal when the two
      elliptic planes satisfy `B₁ · B₂ = 0` and `B₁ ∧ B₂ ≠ 0`, and the
      shear is 78.46's commuting part, the drag, which changes rate and
      not plane (MU-SENSING §3.7).
    - Claim: `only_disjoint_plane_orbits_change_rate_under_axiomatic_gravity`.
    - The court's pressure (`Field::beat_under`) is wired to the field's
      shape.
    - `Field` orbits are attached to wells. Shear is gravitational
      alignment toward the motion the attractor compels; precession is
      the residue per revolution; every path is a geodesic, a closed one
      with zero precession in a curved field resonant, a path through a
      flat region refused `Unbound` (MU-SENSING §3.9; 78.171 amendment
      (f)).
    - The vortex's monodromy (25.12) is read in `Spin(n+1, 1)` between
      the charts of the field's atlas, not in `Spin(4,0)(ℚ)`.
    - Claims: `axiomatic_gravity_shears_the_orthogonal_class`,
      `court_pressure_deforms_the_well_it_faces`,
      `vortex_monodromy_read_between_charts`.
    - Perceptions as adjudicated claims (MU-SENSING §0): a mind states a
      well's holonomy, an orbit's precession and a vortex's monodromy,
      and a court reconstructs each, with kernels reconstructing the same
      ones on LAW-FIELD's joint fixtures.
    - Reflective claims: `holonomy_stated_by_a_mind_reconstructed_by_a_court`,
      `precession_stated_by_a_mind_reconstructed_by_a_court`,
      `vortex_stated_with_its_monodromy_reconstructed_by_a_court`.
11. **S4b (v0.4.0), rank n and transduction as factorisation**
    (MU-SENSING §3.9): the rest of S4 (`GRID_AXES`, `FRAME_AXIS`,
    `PLANE_ROW`/`PLANE_COL`, the rank-2 raster and `Link`), so tasks of
    any rank are perceived, not only carried; and every pair factored as
    motion ∘ residue, with several factorisations held as a vortex.
    - Claims: `deformation_factors_as_motion_then_residue`,
      `several_factorisations_held_as_a_vortex`,
      `court_derives_the_residue_and_admits_on_match`,
      `rank_three_task_is_perceived`, `link_carries_rank_n_coordinates`.
    - Track 10 takes 78.73's three responses (elastic, plastic,
      fracture) as the mind's response to a residue.
    - **Built in part** (branch `s4b-rank-n`, four commits):
      - **Perspective.** `GRID_AXES`, `GRID_PLANE`, `is_grid_axis`,
        `FRAME_AXIS`, `GRID_N`, `PLANE_ROW` and `PLANE_COL` are retired.
        `transducer::perspective` assigns roles: the default reads every
        axis as space, and a family's site carries the sample's place
        after its axes. The sensor plane is the perspective's first two
        space axes, and a rank-1 sample has none (`NoPose::Planeless`).
      - **The alphabet** stays at `n = 3`, derived:
        `ALPHABET_BASE_N = LETTER_RANK − 1`, the slots and the sensor
        plane read from it. A higher rank is refused by name
        (`AlphabetRank`). D2.5 O3: `STROKES_PER_RESERVE = 1 <<
        ALPHABET_BASE_N`.
      - **The link.** `sentence::Link` is a `Site` of rank `n` and the
        symbols at the source's width. The word, chart and field
        carriers read `SectionLink`, reached through `Link::section`,
        refused by name.
      - **Connection by grade** (`incidence_of_sites`); the rule
        language's offsets generated.
      - **Factorisation** (`transduce::factorisations`, `Factored`) and
        the court's admission (`court::verdict::factorisation`).
      - **Perception at any rank:** the mind perceives every task it
        takes (`TaskPercept`, `Mind::perceive_envelope`). A task off the
        section stops `BeyondAlphabet`. `NotRaster` is retired for
        `SectionRefusal`.
      - Claims: the five above, `rank_one_task_is_perceived`,
        `no_axis_is_privileged_in_perception`,
        `bodies_connect_by_shared_cell_of_grade_n_minus_one`,
        `perspective_refuses_rank_beyond_alphabet`.
      - No task, word or chart address changes.
    - **Open** (the next phase):
      - the word, chart and rule-language machinery's rank-2 carrier
        (`family::Grid`, about 94 files), held as the alphabet's
        section: retired in S4d, stage 1;
      - the hypha's registry and the court's solve reconstruction, which load
        posted tasks through the section (`decode_posted_task`): built in
        S4c;
      - a wire act for a claim of factorisation: built in S4c;
      - the letters past `n = 3` (a plane a larger `n` opens receives a
        letter by 37.7);
      - versor motions past the signed permutations (T3).

    **S4c (v0.4.0), the hypha at rank n and the claim of factorisation**
    (MU-SENSING §3.9, §6, §6.2; the wire act decided for v0.4.0's break,
    2026-11-04):
    - Claims: `hypha_serves_a_rank_three_task`,
      `factorisation_claim_round_trips`,
      `court_admits_a_matching_factorisation_claim`,
      `court_refuses_a_factorisation_whose_residue_differs`,
      `mind_speaks_factorisations_beyond_the_alphabet`,
      `factorisation_claim_states_its_ground`.
    - **Built** (branch `s4c-rank-n-hypha`, four commits):
      - **The hypha at rank n.** The registry holds each task as posted
        (`PostedTask`), named by its content alone (`post`), served as
        posted (`PostedTask::envelope`). The section
        (`TaskRegistry::section`) feeds only the word machinery: a
        solve's reconstruction, the bound families and the routes (empty for a
        task off it). A solve on a task off the section is refused by
        the alphabet's name alone (`REFUSE_BEYOND_ALPHABET`).
      - **The act.** A claim kind under the solve (78.126: the wire's
        bytes below the court's range are spent, so a new act is a
        sub-kind; decided 2026-11-04): the solve act carries its kind
        after its tag (`solve_kind`), 0 the solve of outputs (signed as
        `mycelium solve v4`), 1 the claim of factorisation
        (`SOLVE_KIND_FACTORISATION`): a train pair, the motion (the
        signed permutation and the periods it is taken onto), the
        residue as rank-`n` links, the stated ground, signed
        (`dialogue::factorisation`). A MAY row of both profiles in
        MU-CONFORMANCE (`solve factorisation`).
      - **Admission by reconstruction only**
        (`server::factorisation::judge_factorisation`): the stated
        ground read at any rank (`cognition::stated_ground_of`), the
        motion rebuilt through `Factorisation::of`, the residue derived,
        admitted on a match or refused with `FactorRefusal::Residue {
        unclaimed, uncarried }`.
      - **μ, ruled (2026-11-04):** recorded
        (`TaskRegistry::record_factorisation`), never paid. μ is the
        metabolic cost of traversal; a factorisation is an adjudicated
        perception and its residue is meaning, which is content and never
        paid (MU-SENSING §3.7, §3.9).
      - **The mind** speaks one claim per held factorisation of a task
        beyond the alphabet (`Mind::speak_factorisations`), stating the
        ground it read where it read one symbol.
      - The fixture: a rank-3 posted task (`server::tests::fixtures`),
        run end to end in process.
    - **Open:** the word machinery's rank-2 carrier (S4d), the letters
      past `n = 3`, and T3's versor motions.

    **S4d (v0.4.0), the word machinery reads a sample** (MU-SENSING §3.6,
    §3.9, §5; the last of S4). Every use of `family::Grid` is read one of
    three ways: (a) a reader of any rank reads the `Sample` itself; (b) a
    reader that is rank 2 by the letters' mathematics reads a sample on
    the alphabet's section, never a carrier that fixes a rank of its own;
    (c) a corpus's own rows stay in its decoder (`sources/arc-agi`).
    - **Stage 1, built** (branch `s4d-rank-n-words`):
      `transducer::spelled::SectionSample`, a `Sample` admitted onto the
      section, held whole with its place and refused by name off it,
      replaces `family::Grid` throughout (about 17 files of class (a), 77
      of (b), one of (c)). The symbol set's posted test inputs, the ground,
      the envelopes' dilation and every re-period, need, share and
      transduction of a pair read the sample at its own rank. An absent
      test output is `None`. `raster::AXES`, `extents` and `raster_of`
      are retired. No wire or address change. Claim
      `word_machinery_reads_a_rank_three_sample_end_to_end`.
    - **Stage 2, readers built** (branch `s4d2-section-readers`): a
      section sample is read through its perspective
      (`SectionSample::perspective`), admitted only where the
      perspective's plane (`Perspective::plane`) spans every axis. A site
      of the section, `spelled::SectionSite`, is the sample's coordinate
      vector restricted to that plane. The line readers (`lines`, `line`,
      `line_mut`, the `[r][c]` index) are retired: readers walk `sites`,
      read `at` and write `paint` in place, in perception, the painted
      readings, the frame fixing, the fibre map, the rule language, the
      court's and the mind's reconstructions, the soak and the tests. Speech's
      canvas (`statement::speak_elements`) is the input sample painted in
      place; panes, tiles, the crop and the scaling write samples. The
      symbol set refuses a test input of another width by name
      (`SymbolSet::of_family` returns `SymbolRefusal::Width`). Claims
      `section_site_is_the_sample_coordinate_restricted_by_the_perspective`,
      `symbol_set_refuses_differing_test_width`. No
      wire, task, word or chart address change.
    - **Stage 2b, the planar tuples held as `SectionSite`** (branch
      `s4d2b-section-tuples`):
      - **Built:** `SectionLink` is a named type, a section site and the
        two symbols (`sentence::SectionLink { site, before, after }`),
        ordered as the tuple was; every carrier, key and codec of a chain
        reads it, and a chain's least corner is `element::corner_of`.
        Claim `section_link_holds_section_site_and_transition`. No wire,
        tape, task, word or chart address change.
      - **Built:** the painted triple is a named type, a section site and
        its symbol (`spelled::PaintedSite { site, symbol }`); `Painted`
        and the incongruence reader's `Cell` are retired, and the sensor's
        one reading is `raster::painted_sites` (`painted_of` retired). The
        shells, layers, orbit, speech (`Speech`, `statement::Spoken`), the
        germ, the local section and the incongruence reader read it; a
        scene's span is `SectionSite::span_of`. Claim
        `painted_sites_round_trip_through_their_span`. No wire, tape,
        task, word or chart address change.
    - **Stage 2c, the chart's pairs, the anchor and the clause tables held
      as section sites** (branch `s4d3-chart-roles`):
      - **Built:** the chart's supports (`element::positions`,
        `pullback`, `blade::shape_of_section`, `section_moduli`,
        `incidence_of`, `orbit::positions_of`, `ring_dist_peel`, the
        confluence reader) read section sites; `perceive::on_grid` is
        retired for `SectionSite::of_family`.
      - **Built:** the anchor's selection algebra, footprint and scene
        together: `anchor::Evidence` holds section sites and painted
        sites, and `Selection::eval`, the derive,
        `candidates::signature`, `keys::footprint_key` and the evidence
        reply read them; the face neighbours and the extremal codes are
        generated from the plane's rank. Claim `section_site_round_trips`.
      - **Built:** the clause reader's tables: `Seen` holds painted sites
        and the owner of each site at its canonical offset;
        `statement::corners_of` replaces `extent_of`; the landed lists,
        regions, joins and crop read section sites; `acts::pattern_key`
        writes a centred pattern of painted sites. Claim
        `corners_of_reads_least_and_greatest_on_each_axis`.
      - No wire, tape, task, word, chart or clause address change: every
        codec writes a section site in the plane's order
        (`SectionSite::put_canonical`).
      - The act reading's own index cells and the frame acts were left
        open here; both are built in stage 2d below.
    - **Stage 3:** the rule-language's `About::Grid` and the remaining
      planar names renamed by function (track 11).
      - **Built** (coordinator decision; branch `s4d3-chart-roles`): the
        rule language's `Coord::Height`, `Width`, `Top`, `Left`, `Bottom`
        and `Right` (and the SDK's `mycelium_sdk::rule::Coord`) are named
        by the plane axis they are read along, never by a row or a
        column: `Extent(PlaneAxis)` for `Height`/`Width`,
        `Least(PlaneAxis)` for `Top`/`Left` and `Greatest(PlaneAxis)`
        for `Bottom`/`Right`, where `perspective::PlaneAxis` is the first
        or second space axis of the perspective's plane
        (`Perspective::plane`). The clause codec keeps each byte
        (`Extent` 1 and 2, `Least` 3 and 4, `Greatest` 5 and 6), so no
        clause or word address moves; the SDK names change with the
        core's, inside v0.4.0's break. Claim
        `coord_round_trips_and_reads_its_planar_byte_by_role`.
      - **Built:** `About::Grid` is `About::Sample`, a motion read about
        the sample's own frame; the codecs keep their bytes.
      - **Built:** the section's planar names by function:
        `dialogue::TaskRasters` is `TaskSection`,
        `TaskEnvelope::of_rasters` and `PostedTask::of_rasters` are
        `of_section`, `family::grid_of_painted` is `sample_of_painted`,
        `painted_within_grid` is `painted_within_sample`, the SDK's
        `Step::grid_motion` is `Step::symmetry_motion` and
        `seat_mind::task::grid_steps` is `symmetry_steps`.
      - The frame acts' planar fields, the statement's output frame and
        the "grid" test names were left open here; they are built in
        stage 2d below.
    - **Stage 2d, the last planar carriers in the act reading** (branch
      `s4d2d-act-cells`; coordinator decision, convert all of it):
      - **Built:** the act reading's own index cells hold painted sites
        and section sites: a body's cells (`painted_of`), the residue and
        its pieces (connected through a shared face, `sentence::faces_of`),
        the owner of each site at its canonical offset (`acts::Owners`),
        the connectors' ends along whichever plane axis a piece is straight
        on, the echoes' shapes, offsets and doubled centre displacements
        (written with `SectionSite::put_canonical`), the slide's ray and
        the bends. The local adapter (`cells_of`, `extent`, `at`, `owners`,
        the planar `step`) is removed.
      - **Built:** the frame acts name the plane: `Act::Panes { along:
        PlaneAxis, .. }` and `Act::Tile { blocks: [u8;
        ALPHABET_SAMPLE_RANK], .. }`, one count per plane axis in the
        plane's order; the codec keeps every byte
        (`clause::panes_byte`: 1 the first axis, 0 the second). The act
        reading reads extents along the plane (`whole_ratio`), speech
        lays tiles at `statement::block_corner`, the square's symmetries
        carry sites by `statement::turned`, and `scaled` reads blocks on
        every axis alike. They stay rank 2, the letters' plane by their
        mathematics. Claims
        `frame_act_round_trips_and_reads_its_planar_bytes_by_axis`,
        `extract_rule_reads_panes_along_first_plane_axis`.
      - **Built:** `statement::Spoken.frame` is the output's extents per
        plane axis. Claim `speak_clauses_frames_output_by_plane_axis`.
      - **Built:** `Speech::grid` is `Speech::sample` (SDK surface, inside
        v0.4.0's break); the tests' `grid(rows)` fixtures are
        `digits(rows)`; `speech_sample_refuses_site_outside_frame`,
        `germinate_refuses_unchanged_scenes`,
        `slot_of_axis_stands_for_each_base_axis`.
      - No wire, tape, task, word, chart or clause address change.
      - **Remaining planar, outside the act reading:** `Speech.frame` and
        `perceive::frame_of` (the measured `Frame`, renamed `Reperiod`, is a
        re-period per phase since branch `sensing-frame`, claim
        `reperiod_is_read_per_phase_at_any_rank`); the slide's and the line's ray
        (`Act::Slide`/`Extend { ray: (i8, i8) }`), a clause field written
        as a pair; and comments elsewhere that say "grid".
    - **Decided from existing rulings** (Law VIII; MU-SENSING §5): the
      letters never spell a rank-3 sample plane by plane, since planes
      are peeled, never iterated. A rank-3 task stays `BeyondAlphabet`
      and is held through its factorisations (S4c) until the letters are
      generalised past `n = 3`.
12. **S10, emergent senses** (MU-SENSING §4.1; after S6 and S7–S9,
    because a sense is built on the field and its perspectives):
    - A sense is a typed object: a reading from a sample to a cochain of
      the cover, composed from held readings and adjoined generators. A
      perspective is a typed object in the same way.
    - A mind constructs candidates. A residue no held sense reads is a
      demand for a sense.
    - Admission needs all of: new rank in the cover's cohomology, a
      court's reconstruction from the construction alone, and minds that
      converse on it. Never a score.
    - An admitted sense enters the genome and joins the cover on equal
      terms with the three founding eyes.
    - Claims: `sense_restating_the_cover_is_not_admitted`,
      `sense_adding_cohomology_rank_is_admitted`,
      `court_reconstructs_a_sense_from_its_construction`,
      `admitted_sense_is_inherited_through_the_genome`,
      `residue_no_sense_reads_is_a_demand_for_a_sense`,
      `court_never_proposes_a_sense`.

Order (ruled 2026-11-02): S0–S5 land in v0.3.0's one break, and T2
merges after S0 and S4. S4a and S7–S9 follow LAW-FIELD v1's agreement
with the peer kernels ([proposal](docs/proposals/78.171-law-field.md),
[#36](https://github.com/SAIOS-SAMN/mycelium/issues/36)), in v0.3.0 if it
is agreed before rc.3. S6 follows v0.3.0. Depends on the peer kernels for
S4a's carrier (Concordance 2.10's nested shells). It serves track 25
(X5, the transduction's guard), T3 (the turn-native chart reader reads
samples within the field's charts), track 19 (the general conformal
algebra) and the reference mind (track 10).
