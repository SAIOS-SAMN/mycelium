# MU-ENGINEERING — the system as built and as specified

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents; *worlds* and *planets* are simulated places, and *peoples*, *civilizations* and *ancients* simulated inhabitants, of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the hypha and client binaries as built. No game term.

**Status:** overview, release 0.3.0-rc.3. §1, the wire description in §4, and
§5–§8 are checked against the code. §2, §3 and the rest of §4 describe the language, the
court organ and the mesh module by module;
where a part is specified and not wired, the section says so. Later
changes are recorded in [`CHANGELOG.md`](../CHANGELOG.md); plans are in
[`ROADMAP.md`](../ROADMAP.md). Companion documents: `MU-ECONOMY.md`,
`MU-PRODUCT.md`, `MU-XSEPTUM.md`, `MU-ANASTOME.md`.

## 1. Two binaries

- `mycelium-server`, a hypha: the book, the market, the registry of work,
  the genome store, and the court organ. Identity is an ed25519 key
  (`court::identity::HyphaKey`). Peers are a bootstrap list; after the
  first digest the home trie is the topology.
- `mycelium-client`, a mind: a voice, a genome, a contemplation, and the
  two-stroke metabolism. Bootstraps to one hypha, discovers the rest,
  routes every act by chart.

There is no separate court process. Each process runs with
`--home <dir>`; a home holds its key, its signed mesh record and its
corpus manifests (configuration) and the ledger or genome it writes
(state).

## 2. The language

- **Alphabet** (`transducer/alphabet/`, `MU-ANASTOME.md` §2): the
  seventeen seed Cayley letters the tree holds, as exact Cl(4,0) rotors;
  by law the alphabet is reseeded empty and letters are adjoined (U14,
  `MU-ANASTOME.md` §1.1). Words are oriented
  tapes; spelling is a peel; a motion the alphabet cannot spell is a
  remainder, never a word.
- **Element** (`transducer/element/`): a word. Residue germs carry a
  chain; connections carry a letter tape, a dilation, a placement, a gauge
  map, and an optional remainder; sentences carry germs on germinated axes
  with faces as transitions and holonomy as torsion; rewrites and
  translations each carry a tape. The address measures the tape. A
  residue chain is never a word on the wire, the book or the genome: it
  is the remainder of a sentence and travels only as a typed demand
  vertex of one (`MU-RECURSION.md` chapter 50).
- **Sentence** (`transducer/sentence/`): a nerve of germs with a Čech
  complex over them; a rule is a sentence whose germ charts, face words,
  holonomies, and hue maps are constant across a family's pairs, the
  per-pair transports being the gauge.
- **Chart** (`transducer/stalk/`): the atlas; charts are invariant
  vectors whose prefixes are planes; a Keyed trie of strata.
- **Comprehension** (`transducer/family/`, `client/sdk/`): perceive
  bodies, peel shells, carry bodies pair to pair, assemble the nerve,
  extract the rule, speak it onto a test. Hearing a word is deconstructing
  its tape and gluing it against the representative on its chart.
- **The eye is a cover** (`MU-RECURSION.md` 24.4): specified, not yet
  wired. At least two senses over every sample — luminal (blades and
  carriages), harmonic (conformal invariants), holonomic (transitions and
  the transitions' holonomy around rings of bodies) — as cochains whose
  cocycle is checked at perception; the residue is the demand, with its
  verdict and axes named, and only a novel residue reaches the book. Today the senses
  run as a pipeline.

## 3. The court organ

Inside every hypha (`server/cognition/`, `court/verdict/`):

- `reconstruct_blind` (`court/verdict/blind.rs`) re-derives every kind of word from its own
  tape; false speech is slashed with a reason.
- The docket is the field's projection: listings whose closure is not Flat
  with nonzero deflation, read by key under a stroke budget.
- Gluing across hyphae (`glue_fetched`, `server/cognition/court/glue.rs`)
  is the Čech coboundary: identity transition admits, placement attaches,
  any other transition is a dialect row with its versor, and cells at
  higher degree carry classes. The basis grows only by gluing; a listing
  settles when two organs converge on one re-derivation
  (`court::convergence`), never by a count of voices, and convergence
  never installs on its own.
- A word is language when the home ruled it and a peer's ruled root holds
  it; the home installs on that evidence and releases escrow.
- Signed roots: each hypha signs its cover roots and ruled root; nothing
  installs on a signature alone.

## 4. The mesh

- **Home trie** (`network/nerve/home_trie.rs`, `HomeTrie`): per lane, a trie of chart
  prefixes to the hyphae whose active work covers them. Home is the fibre
  intersection on every occupied lane, then per-lane room, then identity.
- **Digest exchange**: signed cover and ruled roots; fetch of discordant
  strata; paced per peer by that peer's Λ counted in the court's own K
  and by the court's own roots moving, never by a timer and never on every
  orbit; a deferral carries the door's per-lane deficit and dwells the
  asker (`MU-RECURSION.md` 45.7, 23.8).
- **Roots**: solved families announced by key, keyed by task and solver.
- **Wire** (`anastome/dialogue/`, `network/wire.rs`): one frame, one act
  byte. The act constants and the frame layout of each are documented in
  [`src/anastome/dialogue/acts.rs`](../src/anastome/dialogue/acts.rs); the
  byte of each act is assigned by `Action::byte` in
  [`src/physics/actions/mod.rs`](../src/physics/actions/mod.rs). The families are:
  - client work and language: `ACT_TASK`, `ACT_SOLVE`, `ACT_SPEECH`,
    `ACT_SYLLABUS`, `ACT_ADOPT`, `ACT_DIGESTS`, `ACT_FETCH`, `ACT_PROBE`,
    `ACT_PEERS`;
  - readers any party may send, which admit nothing: `ACT_QUOTE` (the
    market view), `ACT_OBSERVE`, `ACT_LAWS`, `ACT_WHY`, `ACT_WALLET`
    (which also carries signed stake and unstake), `ACT_RECEIPTS`,
    `ACT_ENTRIES`;
  - hypha to hypha (the court organ and the clock): `ACT_COVER`,
    `ACT_OFFER`, `ACT_EVIDENCE`, `ACT_ROOT`, `ACT_TICK`,
    `ACT_FOLD`;
  - domains and tiers: `ACT_UPLINK`, `ACT_DOMAIN`, `ACT_CHALLENGE`,
    `ACT_HANDOFF`;
  - the seat surface for hosted games: `ACT_SEAT`;
  - xSEPTUM: `ACT_EQUIVOCATION` (`MU-RECURSION.md` 47.4) is built and the
    rest of its range is reserved;
  - retired acts (`ACT_PERCEPT`, `ACT_GOSSIP`, `ACT_DEMAND`,
    `ACT_TEACH`, `ACT_SETTLE`, `ACT_DOCKET`, `ACT_INSTALLED`), whose
    bytes stand only so the door can refuse them by name; the routed
    demand's byte (0x14) stands in `Action::DemandRouted` alone, with no
    constant.

  The receipts a court seals carry the court's own actions, in the court
  range from `COURT_RANGE` upward.

  `ACT_LAWS` reads by mode (`physics::laws::codec::MODE_*`): 0 the
  register, 1 and 2 verification, 3 the lexicon, 4 the core clock, 5 the
  parameters, and, from v0.3.0, 6 the client conformance statement
  ([MU-CONFORMANCE](MU-CONFORMANCE.md)), 7 the Concordance's clause
  register and 8 a kernel's signing act by reconstruction of each
  clause's fixtures, admitted or refused by name (`NotSignatory` naming
  the clause and the fixture; Concordance §0, from v0.4.0, where modes 7
  and 8 replaced v0.3.0's hash and whole-text signature). A hypha that
  predates modes 6–8 answers nothing to them, as it answers any
  `ACT_LAWS` request it does not read.

  **The wire version rule.** `WIRE_VERSION` is `major << 8 | minor`. An
  additive change (a new act, mode or trailing field an older reader
  ignores) steps the minor; a change that moves the layout of an existing
  act or reply steps the major. Either may ride a 0.x minor release. v0.3.0
  writes every μ field as four exact rational lanes (`economy::Mu`), which
  moves the layout of every act and reply carrying μ: wire v2.0. v0.5.0
  carries the crossing record (X1, MU-HOSTING §5a.10) in the signed bytes
  of the seven universe acts, which moves their layouts: `WIRE_VERSION` is
  `0x0300` (wire v3.0).

## 5. Physics and time

- `physics/` (`src/physics/`): reserve per lane calibrated per host;
  `Budget` (`physics/budget.rs`) is a linear token minted only by
  `Physics::stroke` and consumed by every cursor and expansion; no
  duration is derived from a reserve count.
- A court's own receipt index `i` orders events inside that court, and
  `K` counts the orbits the court closed (ruled 2026-11-05: `K` means
  closed orbits everywhere). TUniversal is the height of the
  multi-dimensional ledger, read from receipts;
  it measures how long a carrier has stood at one home and never orders two
  events across nodes. No court and no mind reads a wall clock, a unix clock, or a
  timer; every duration is a count of strokes or heights (`MU-RECURSION.md`
  Part IX). Lane 2 of μ⃗ is temporal extent in heights.
- Lanes: S spatial, G gauge, T temporal, H holonomic. Sections and torsion
  are holonomic content on lane 3.

## 6. Lifecycle

A carrier lives in the hot set while it can still change a verdict and
leaves by relocation to a cold carrier, never by deletion (`MU-ECONOMY.md`
§6 for the ledger side):

| carrier | leaves when | where it goes |
|---|---|---|
| docket / open listing | ruled, slashed, or T-room exhausted | the ruled cover, or the cold field |
| dialect row | superseded by TUniversal, filled by a translation, or chart refined | cold |
| heard | the peer's stratum root moves | dropped, re-fetchable |
| field | Flat on every chart, or relocated | the cold field (`field_cold`) |
| settlements | read by the court | `settlements_cold` |
| basis word | derivable from the rest (`reduce_excluding`) | cavity, address kept |
| folded prefix (work receipts) | its fold co-signed by a witness (`MU-RECURSION.md` 45.8) | an era in `ARC1`, held by the home and the witness |

## 7. The gate and the `Keyed` carrier

The gate is held by the two commands of
[AGENTS › Definition of done](../AGENTS.md#definition-of-done):

1. the source checks, the tests of the workspace crate `gate/`, over
   `src/`, `algebra/src/`, `forms/src/`, `sdk/src/` and `gate/`: the
   reading bound (no `.rs` file over 2500 lines), the Euclidean
   vocabulary (no code line naming a grid-indexing shape such as a Morton
   curve, a bounding box or a subdivision tree), the numbers gate and the
   drift register, and the transport's carrier names (§9,
   [`gate/src/layers.rs`](../gate/src/layers.rs));
2. clippy over every workspace member with warnings denied, under the one
   lint table of `Cargo.toml` and the disallowed types and methods of
   `clippy.toml`: maps and sets, digests (`blake3::hash`), wall-clock
   types (`Instant`, `SystemTime`, `Duration`), silent scalar choice,
   invented defaults, and members by position.

Where older text names "the clock gate", "the blake3 gate" or "the
enumeration gate", it means the `clippy.toml` entry or the type that now
carries that rule. `MU-STANDARDS.md` §4 gives the full table of rules and
lints.

`Keyed` (`network/nerve/keyed/`) exposes `get`, `get_mut`, `insert`,
`remove`, `contains`, `len`, and four ways to obtain more than one entry:
`fibre(prefix)`, `peel(key)`, `link(key)` and `cursor(from, budget)`.
`fibre` takes a `Prefix`, which cannot be empty, so a whole carrier is
read only through `cursor` under a `Budget`. Persistence walks a carrier
through the crate-private `keyed_persist_walk`. The work counters
(`network/nerve/keyed/work.rs`), incremented in test builds on `Keyed` get
and insert, `Rotor::geo` and `cech::extend`, let a test assert what one
fetched vertex, one held thought and one meet cost.

## 8. Persistence and continuity

A hypha's snapshot and a mind's genome are each written as a keyed trie
by its own coordinates and read back by key, never by position. The layout
tags in the code are:

| tag | what | where |
|---|---|---|
| `MYT2` | a hypha's snapshot (the current layout) | `server/cognition/persistence/layout.rs` |
| `MYL2` | a court's line | `chronometry/chain/line.rs` |
| `MYE2` | a cold era segment | `chronometry/era.rs` |
| `GEN7` | a mind's genome: the words a court admitted | `client/sdk/vocabulary/persist.rs` |
| `TRI1` | an on-disk keyed trie | `network/nerve/keyed/ondisk.rs` |
| `MSH2` | a signed mesh record | `network/mesh.rs` |
| `CUR3`, `INT1` | the registry's snapshot: its dispatch cursor and task states, then its intake cursor | `server/registry.rs` |
| `COC1` | the Čech complexes (addresses and rotors only) | `anastome/dialogue/acts.rs` |

Superseded layouts are refused by name and never migrated: `MYC7` (the
positional tape) and `MYT1` for the snapshot, `GEN4`, `GEN5` and `GEN6` for
the genome. The one exception is the mesh record, which is configuration:
an `MSH1` record is read and written back as `MSH2`. Ledgers and genomes
are rewritten atomically; a restore rebuilds the basis (a derived index)
from the lettered federation, and `prune_basis_to_ruled` drops the words
the court has not ruled. Within one layout, residues,
thoughts, and dialects are never deleted (`MU-STANDARDS.md` §10).

## 9. Transport, carrier and coordinate

Three layers, each with one job (coordinator ruling, 2026-11-05). The
gate holds the first boundary: no transport source names a carrier type
([`gate/src/layers.rs`](../gate/src/layers.rs)).

### 9.1 The model

- **Transport** moves bytes between two endpoints: the connection, the
  channel's security (Noise), delimitation (a length) and flow (deferral).
  It never holds meaning.
- **Carrier** is the exact object that holds meaning at full dimension.
  It describes itself (its rank, its signature, its keys), is written
  canonically, and never projects to fewer dimensions.
- **Coordinate** is where something stands. It routes and never
  identifies. TUniversal stamps and never drives.

| Layer | What it is | What it may know | What it must never hold |
|---|---|---|---|
| Transport | The connection; channel security (Noise); delimitation by a length; flow by deferral. Sources: `src/network/wire.rs`, `src/network/noise.rs`, `src/network/channel.rs`, `src/bin/mycelium-server/transport.rs` (listed once, as `TRANSPORT` in the gate) | Byte counts, the body budget, rtt in strokes, peer keys, capacity | Meaning: a lane, a rank, a coordinate, the identity of content, any carrier type |
| Carrier | `Mv` (`Cl(p, q)`, carrying its `n` and signature; `algebra/src/multivector.rs`), `Sample` (rank `n`; `src/transducer/sample.rs`), `Modes` (μ per lane as vectors; `src/economy/modes.rs`), `Coordinate` (lanes as keyed vectors; `forms/src/coordinate.rs`), `SectionSite`, `Turn`, and the passage's hold and reach (`forms/src/passage.rs`) | Its own description: rank, signature, lane keys; its canonical writing | A projection to fewer dimensions: a scalar per lane, a fixed four, a floor, `==` on a digest as identity |
| Coordinate | A body at `origin / h : S:G:T:H : K` (proposal 78.173); an event at a TUniversal height on a signer's line, at its position `i`; a site within a sample | Where a thing stands; the route to it | Identity: a coordinate locates and routes, and never names what stands there. TUniversal stamps an event and never drives an advance |

**Composition.** The transport carries expressions: anastome acts, each
its tag, its content, its signature and the TUniversal point its sender
read. An expression's identity is its content written out
(`MU-RECURSION.md` A21), never a digest of its bytes. Its carriers stand
at coordinates. An admitted expression is an event of the
multi-dimensional ledger (lane × layer × grade). What the transport
carries is anastome: its expressions (assertions, reads, court-to-court
exchanges and universe crossings) and each act's kind are
[MU-ANASTOME](MU-ANASTOME.md) §1.5 and §6.

**Sound is a case of the model** (MU-SENSING §3.10, stated there once).
The transport carries sound's bytes meaning-free, the same for every
receiver, as it carries any other bytes; a carrier (the harmonic sense's
reading) holds the sound's harmonic structure; and whether a reading is
signal or noise is never a transport or carrier property, but is decided
by the reader that holds the structure.

### 9.2 Known violations

Each is a place where a layer holds what belongs to another. The gate
lists the transport's ones as exceptions (`EXCEPTIONS` in
[`gate/src/layers.rs`](../gate/src/layers.rs)), each with the release
that removes it; an exception whose carrier no longer stands in its file
is refused as stale.

| Violation | Where | Fix | When |
|---|---|---|---|
| The frame's lane byte: the transport labels meaning. Every act is written on `Lane::Spatial` and every reply echoes the request's lane; no reader decides on it | `src/network/wire.rs` (`Lane`, `Delimited::lane`), `src/bin/mycelium-server/transport.rs`; about 77 `Delimited` writes, among them `src/client/sdk/session.rs`; the SDK re-exports `Lane`, `read_lane` and `POINTED` (`sdk/src/house.rs`) | The envelope becomes a length and a body; the lane byte is removed | v0.5.0, a breaking wire release |
| The TUniversal point rides the envelope: the pointed frame's flag bit and trailer (78.152 TU-7) | `src/network/wire.rs` (`POINTED`, `encode_at`, `split_point`), `src/network/noise.rs`, `src/bin/mycelium-server/transport.rs` | The point read moves into the expression, beside its tag, content and signature; MU-CONCORDANCE 1.11 says "behind its body on the frame" and changes with it (*Network contract*) | v0.5.0, with the lane byte (proposed) |
| The transport reads and writes content: the frame's encoder reads the act tag for the observer (`act_tag`), and the door writes a refusal body (`encode_outcome`) | `src/network/wire.rs`, `src/bin/mycelium-server/transport.rs` | The observer counts bytes by act above the transport; the refusal is spoken by the judge layer | Open (found by this audit) |
| `Mu` as four scalar lanes | `src/economy/mu.rs` | `Modes` replaces `Mu` (ROADMAP 1b) | v0.5.0 |
| `[u64; 4]` in quotes and deferrals | `src/anastome/dialogue/quote.rs` (`encode_outcome`, `encode_deferral`) | `Modes` on the wire | v0.5.0 |
| `payload_hash` and `==` on digests as identity | `src/chronometry/receipt.rs`, and the sites track 25 X0 lists | Identity is the content written out (A21); a digest only locates, links or commits | Track 25 (X0 drift guards, #28) |
| ~~Noise built and not wired: the mesh runs in plaintext~~ | `src/network/noise.rs` | The door and the session open the channel: `src/network/channel.rs`, the door and every ask in `src/bin/mycelium-server/transport.rs`, `Session::with_noise`; opt-in per peer, a plaintext peer named (MU-PRECURSORS §3.1) | Closed (noise-wired) |
| `home_trie`'s `lane & 3`: a lane folded onto the base four | `src/network/nerve/home_trie.rs` | Lanes read as keys, never masked | In progress |

### 9.3 Names to settle

"Frame" names ten distinct things. By MU-STANDARDS one name has one
meaning, so each is to be renamed by its function. No code identifier is
renamed here; the renames follow the dead-code cleanup of
`src/network/wire.rs`. Counts are of the distinctive identifiers in Rust
sources (`src/`, `algebra/`, `forms/`, `sdk/`, `sources/`, `tests/`,
`gate/`) and of matching phrases in the documents (`docs/`, README,
AGENTS, ROADMAP, GLOSSARY), read 2026-11-05; the bare words `frame`,
`frames` and `Frame` (about 3,550 in code and 1,000 in documents) are
counted under the meaning of their area, not apiece.

| Meaning | Where (examples) | Code identifiers | Document phrases | Proposed name |
|---|---|---|---|---|
| The wire envelope: one act's bytes with their lane byte and length | `WireFrame`, `frame_budget`, `read_frame`, `write_frame`, `hear_frame`, `FrameOverBudget`, `PageEnd::Frame`, `Inflow::Frame`, `receipts_frame`, the era file's `append_frame` | about 254 | about 57 | An expression delimited by its length: `Delimited` for the envelope, `BODY_BUDGET` alone for the budget, `read_delimited`, `PageEnd::Budget`; the era file's record is a `record` |
| One sample of a family along `e₃` (78.75; the GLOSSARY's *Frame* today) | `Frames`, `frame_key`, `frame_spelling`, `put_frame`, `member_frames`, `section_of_frames`, `at_frame_zero`, `perceive_frames`, `edges_rederive_frames`, `HeldPose::OneFrame` | about 254 | about 44, and most of MU-RECURSION's 538 | The family's sample `k`: `sample_key`, `sample_spelling`, `samples_of_section`, `HeldPose::OneSample` |
| The sample's own extent, read as a whole | `Layer::Frame`, `Vertex::Frame`, `VertexAt::Frame`, `frame_fixed_on_ground`, `frame_faces`, `frame_boundary`, `share_of_frame`, `ShapePeriods::InFrame`; "frame fixing" (MU-SENSING §6.1) | about 130 | about 44 | The enclosure, after `transduce::Enclosure`: `Layer::Enclosure`, `Vertex::Enclosure`, `enclosure_faces`, `share_of_enclosure`, "enclosure fixing" |
| Perception's measured `Frame` defect: four counts of a pair, two planar axes fixed | `src/transducer/element/residue.rs` (`Frame { h_in, w_in, h_out, w_out }`), `from_canonical_framed`, `from_residue_framed`, `frame_ratio`, the "frame fibre" ρ | about 40 | about 24 | The re-period, per phase at rank `n`: `Reperiod`, after `Enclosure::reperiod` (track 26 S4 replaces the four counts) |
| The frame acts: panes and tiles | `Act::Panes`, `Act::Tile`, `statement::Spoken.frame`, `Unstated::Unframed` (`src/transducer/family/statement.rs`) | about 23 | about 20 | Enclosure acts: `Spoken.extents`, `Unstated::NoEnclosure` |
| The μ an act or an orbit measured | `frame_of` in `src/server/cognition/cite.rs`, `orbit_frame`, the clock's `unframed` count | about 43 | about 21 | The measure: `measure_of`, `orbit_measure`, `unmeasured` (and `Modes`, not `[u64; 4]`) |
| The passage's `Frame` (hold, reach) and "frame state" | `forms/src/passage.rs` (`Frame`, `PassageRefusal::NotFrame`) | about 38 | about 19 | The passage state: `PassageState { hold, reach }`, `NotOnPolePlane` |
| A frame of reference: an observer's perspective and the conformal map between two | `forms/src/reference.rs` (`Reference`), `InFrame` in closeness, a constellation's `frames` (posed nuclei), a mouth's `from_frame` and `to_frame`, `frame_base_point` | about 65 | about 14 | The reference: module `reference`, a "reference map"; a constellation's `poses`; a mouth's `from_chart` and `to_chart`; `pose_point` |
| A basis of vectors | `src/transducer/cga/dual.rs` (`DualFrame`), `on_frame` in the pose reader | about 26 | about 6 | The reciprocal basis: `ReciprocalBasis`, `on_basis` |
| An external name | "Noise Protocol Framework" | 2 | — | Kept |

"Carrier" is a second name to settle: §9's carrier (the object holding
meaning) stands beside the client's carrier thread and the network's
keyed carrier (GLOSSARY *Carrier*), and `src/anastome/dialogue/carrier.rs`
reads the act tag. The first is the meaning this section fixes.
