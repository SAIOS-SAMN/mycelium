# MU-ENGINEERING — the system as built and as specified

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside governed worlds — games, scientific simulations and world models run by kernels under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents; *worlds* and *planets* are simulated places, and *peoples*, *civilizations* and *ancients* simulated inhabitants, of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the hypha and client binaries as built. No game term.

**Status:** overview, release 0.2.0. §1, the wire description in §4, and
§5–§8 are checked against the code. §2, §3 and the rest of §4 describe the language, the
court organ and the mesh module by module;
where a part is specified and not wired, the section says so. Later
changes are recorded in [`CHANGELOG.md`](../CHANGELOG.md); plans are in
[`ROADMAP.md`](../ROADMAP.md). Companion documents: `MU-ECONOMY.md`,
`MU-PRODUCT.md`, `MU-XSEPTUM.md`, `MU-ALPHABET.md`.

## 1. Two binaries

- `mycelium-server`, a hypha: the book, the market, the registry of work,
  the genome store, and the court organ. Identity is an ed25519 key
  (`court::identity::HyphaKey`). Peers are a bootstrap list; after the
  first digest the cover trie is the topology.
- `mycelium-client`, a mind: a voice, a genome, a contemplation, and the
  two-stroke metabolism. Bootstraps to one hypha, discovers the rest,
  routes every act by chart.

There is no separate court process. Each process runs with
`--home <dir>`; a home holds its key, its signed mesh record and its
corpus manifests (configuration) and the ledger or genome it writes
(state).

## 2. The language

- **Alphabet** (`transducer/alphabet/`, `MU-ALPHABET.md`): seventeen
  immutable Cayley letters as exact Cl(4,0) rotors. Words are oriented
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

- `blind_replay` (`court/verdict/blind.rs`) re-derives every kind of word
  from its own tape; false speech is slashed with a reason.
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

- **Cover trie** (`network/nerve/cover.rs`): per lane, a trie of chart
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
  - readers any party may send, which admit nothing: `ACT_OBSERVE`,
    `ACT_LAWS`, `ACT_WHY`, `ACT_WALLET` (which also carries signed stake
    and unstake), `ACT_RECEIPTS`, `ACT_ROWS`;
  - hypha to hypha (the court organ and the clock): `ACT_COVER`,
    `ACT_OFFER`, `ACT_INSTALLED`, `ACT_EVIDENCE`, `ACT_ROOT`, `ACT_TICK`,
    `ACT_FOLD`;
  - domains and tiers: `ACT_UPLINK`, `ACT_DOMAIN`, `ACT_CHALLENGE`,
    `ACT_HANDOFF`;
  - the seat surface for hosted games: `ACT_SEAT`;
  - xSEPTUM: `ACT_EQUIVOCATION` (`MU-RECURSION.md` 47.4) is built and the
    rest of its range is reserved;
  - retired acts (`ACT_PERCEPT`, `ACT_GOSSIP`, `ACT_DEMAND`, `ACT_QUOTE`,
    `ACT_TEACH`, `ACT_SETTLE`, `ACT_DOCKET`, `ACT_DEMAND_ROUTED`), whose
    bytes stand only so the door can refuse them by name.

  The receipts a court seals carry the court's own actions, in the court
  range from `COURT_RANGE` upward.

## 5. Physics and time

- `physics/` (`src/physics/`): reserve per lane calibrated per host;
  `Budget` (`physics/budget.rs`) is a linear token minted only by
  `Physics::stroke` and consumed by every cursor and expansion; no
  duration is derived from a reserve count.
- K is a court's own receipt index and orders events inside that court.
  TUniversal is the height of the mesh receipt ledger, read from receipts;
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

The gate is the `gate` binary of `xtask/`, run with
`(cd xtask && cargo run)`; there is no `build.rs`. It takes two readings
and reports both:

1. the source checks of `xtask/lint/`, over `src/` and `xtask/`: the
   reading bound (no `.rs` file over 2500 lines) and the Euclidean
   vocabulary (no code line naming a grid-indexing shape such as a Morton
   curve, a bounding box or a subdivision tree);
2. the warning gate, `cargo clippy --release --all-targets -- -D warnings`,
   with the crate-wide denials of `Cargo.toml` and the disallowed types and
   methods of `clippy.toml`: maps and sets, digests (`blake3::hash`),
   wall-clock types (`Instant`, `SystemTime`, `Duration`), silent scalar
   choice, invented defaults, and members by position.

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
| `MYT2` | a hypha's snapshot (the current layout) | `server/cognition/snapshot/layout.rs` |
| `MYL2` | a court's line | `chronometry/chain/line.rs` |
| `MYE2` | a cold era segment | `chronometry/era.rs` |
| `GEN7` | a mind's genome: the words a court admitted | `client/sdk/epigenome/persist.rs` |
| `TRI1` | an on-disk keyed trie | `network/nerve/keyed/ondisk.rs` |
| `MSH2` | a signed mesh record | `network/mesh.rs` |
| `CUR3`, `INT1` | the registry's snapshot: its dispatch cursor and task states, then its intake cursor | `server/registry.rs` |
| `COC1` | the Čech complexes (addresses and rotors only) | `anastome/dialogue/acts.rs` |

Superseded layouts are refused by name and never migrated: `MYC7` (the
positional tape) and `MYT1` for the snapshot, `GEN4`, `GEN5` and `GEN6` for
the genome. The one exception is the mesh record, which is configuration:
an `MSH1` record is read and written back as `MSH2`. Ledgers and genomes
are rewritten atomically; a restore rebuilds the basis from the lettered
federation and drops what does not replay. Within one layout, residues,
thoughts, and dialects are never deleted (`MU-STANDARDS.md` §10).
