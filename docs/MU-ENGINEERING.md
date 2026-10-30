# Engineering — the system as built and as specified

> **Scope.** Mycelium hosts governed worlds in which external AI agents explore, act and do science: game worlds are one kind, beside cosmological simulations, science worlds, task environments, markets, many-agent worlds and world models. Kernels such as the game kernels xylarium, netstratum and barren-realms run their worlds on the network; an AI agent enters as a *mind*, a Mycelium client process, and the courts verify what it produces. In this document, *minds* are those software agents; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated entities in hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the hypha and client binaries as built. No game term.

Status as of v0.1.0. Every section names what is landed and what is not;
later changes are recorded in [`CHANGELOG.md`](../CHANGELOG.md).
Companion documents: `MU-ECONOMY.md`, `MU-PRODUCT.md`, `XSEPTUM.md`,
`ALPHABET.md`.

## 1. Two binaries

- `mycelium-server`, a hypha: the book, the market, the registry of work,
  the genome store, and the court organ. Identity is an ed25519 key
  (`court::identity::HyphaKey`). Peers are a bootstrap list; after the
  first digest the cover trie is the topology.
- `mycelium-client`, a mind: a voice, a genome, a contemplation, and the
  two-stroke metabolism. Bootstraps to one hypha, discovers the rest,
  routes every act by chart.

There is no court process. The lab fleet is three servers and five clients
that differ only by key; per-node flags (`--id`, `--tier`, per-node corpus
paths, kernel names) are lab residue to be removed (§10). Every process
runs with `--home <repository>/var/<unit>` and nothing else; a home holds
its key, its signed mesh record and its corpus manifests (configuration)
and the ledger or genome it writes (state, wiped by a recut).

## 2. The language

- **Alphabet** (`transducer/alphabet/`, `ALPHABET.md`): seventeen
  immutable Cayley letters as exact Cl(4,0) rotors. Words are oriented
  tapes; spelling is a peel; a motion the alphabet cannot spell is a
  remainder, never a word.
- **Element** (`transducer/element/`): a word. Residual germs carry a
  chain; connections carry a letter tape, a dilation, a placement, a gauge
  map, and an optional remainder; sentences carry germs on germinated axes
  with faces as transitions and holonomy as torsion; rewrites and
  translations carry their tapes. The address measures the tape. A
  residual chain is never a word on the wire, the book or the genome: it
  is the remainder of a sentence and travels only as a typed demand
  vertex of one (`MU-RECURSION.md` chapter 50, ruled 2026-09-09).
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
- **The eye is a cover** (`MU-RECURSION.md` 24.4, Stage 10): at least two
  senses over every sample — luminal (blades and carriages), harmonic
  (conformal invariants), holonomic (transitions and their holonomy
  around loops of bodies) — as cochains whose cocycle is checked at
  perception; the residual is the demand, with its verdict (Recognised,
  Reflection, Transposition, Novel) and axes named; only Novel reaches the
  book. Not yet wired: today the senses run as a pipeline and the market
  receives raw residuals.

## 3. The court organ

Inside every hypha (`server/cognition/`, `court/verdict/`):

- `blind_replay` re-derives every kind of word from its own tape; false
  speech is slashed with a reason.
- The docket is the field's projection: listings whose closure is not Flat
  with nonzero deflation, read by key under a stroke budget.
- Gluing across hyphae (`glue_fetched`) is the Čech coboundary: identity
  transition admits, placement attaches, any other transition is a dialect
  row with its versor, and cells at higher degree carry classes. The basis
  grows only by gluing; a listing settles when two organs converge on one
  re-derivation (`court::convergence`), never by a count of voices, and
  convergence never installs on its own.
- A word is language when the home ruled it and a peer's ruled root holds
  it; the home installs on that evidence and releases escrow.
- Signed roots: each hypha signs its cover roots and ruled root; nothing
  installs on a signature alone.

## 4. The mesh

- **Cover trie** (`network/nerve/cover.rs`): per lane, a trie of chart prefixes
  to the hyphae whose active work covers them. Home is the fiber
  intersection on every occupied lane, then per-lane room, then identity.
- **Digest exchange**: signed cover and ruled roots; fetch of discordant
  strata; paced per peer by that peer's Λ counted in the court's own K
  and by the court's own roots moving, never by a timer and never on every
  orbit; a deferral carries the door's per-lane deficit and dwells the
  asker (`MU-RECURSION.md` 45.7, 23.8; ruled 2026-09-09).
- **Roots**: solved families announced by key, keyed by task and solver.
- **Wire** (`anastome/dialogue/`, `network/wire.rs`): one frame, one act
  byte; client acts 0x00–0x16, hypha acts 0x20–0x22, xSEPTUM 0x30–0x35,
  of which `ACT_EQUIVOCATION` (0x35, `MU-RECURSION.md` 47.4) is landed and
  0x30–0x34 are reserved.

## 5. Physics and time

- `physics.rs`: reserve per lane calibrated per host; `Budget` is a linear
  token minted only by `Physics::stroke` and consumed by every cursor and
  expansion; no duration is derived from a reserve count.
- K is a kernel's own receipt index and orders events inside that kernel.
  TUniversal is the height of the mesh receipt ledger, read from receipts;
  it measures how long a carrier has stood at one home and never orders two
  events across nodes. No kernel reads a wall clock, a unix clock, or a
  timer; every duration is a count of strokes or heights (`MU-RECURSION.md`
  Part IX). Lane 2 of μ⃗ is temporal extent in heights.
- Lanes: S spatial, G gauge, T temporal, H holonomic. Sections and torsion
  are holonomic content on lane 3.

## 6. Lifecycle

A carrier lives in the hot set while it can still change a verdict and
leaves by relocation to a cold section, never by deletion (`MU-ECONOMY.md`
§6 for the ledger side):

| carrier | leaves when | cold section |
|---|---|---|
| docket / open listing | ruled, slashed, or T-room exhausted | ruled cover or `CAV1` |
| dialect row | superseded by TUniversal, filled by a translation, or chart refined | `CYC1` |
| heard | the peer's stratum root moves | dropped, re-fetchable |
| field | Flat on every chart, or relocated | `FLC1` |
| settlements | read by the court | `SET1` |
| basis word | derivable from the rest (`reduce_excluding`) | cavity, address kept |
| bubble | merged, or T-room on the entity | `THT1` |
| folded prefix (work receipts) | its fold co-signed by a witness (`MU-RECURSION.md` 45.8) | `ARC1`, held by the home and the witness |

Snapshot sections: `MYC1`, `KCH1`, `RNK1`, `DEF1`, `FLD1`, `BOK1`, `CAV1`,
`CRT1`, `COC1`, `FLC1`, `CYC1`, `SET1`; the genome carries `GEN1` and
`THT1`.

## 7. The enumeration gate

`Keyed` exposes only `get`, `get_mut`, `insert`, `remove`, `contains`,
`len`, and four ways to obtain more than one entry: `fiber(prefix)`,
`peel(key)`, `link(key)`, `cursor(from, budget)`. Persistence iterates
through a sealed trait. `build.rs` fails the build on `.entries()`,
`.for_each(`, and `spend_unit()` in cognition directories, and on
`SystemTime`, `Instant`, and `Duration` in `src/server`, `src/client`,
`src/court`, `src/transducer`, `src/economy/`, and `src/physics.rs`
(the clock gate, `MU-RECURSION.md` 45.4; at 3c02252 the gate still names
`economy.rs` and so misses the directory — TASKS Stage 8c A1). The same
gate fails the build on
`blake3` anywhere outside `src/network`, `src/court/identity.rs`,
`src/chronometry/receipt.rs`, and the persistence loaders (the digest gate,
`MU-RECURSION.md` law 2 and A21: no hash of any mathematical structure),
and on a `(BigRational, BigRational)` or `(i64, i64)` field or parameter
named `placement`, `shift`, or `residue` outside `family::pair_form`,
`family::speak_rule`, and `family::orbit` (the squaring gate, law 11), and
on the shape of a rational matrix — `QMatrix`, `Vec<Vec<Q>>`,
`as_operator`, `realize(` — in `src/transducer`, `src/server`,
`src/client`, and `src/court` (the matrix gate, law 11 and rule 16; F2f
deleted `core/{engine, matrix}.rs`, `core` exports `Q` alone, and
no module carries a matrix for a rank: a dimension is a span's grade,
`core::span`). Two more gates
and one scope rule (`MU-STANDARDS.md` §4): the file-length gate (no source
file over 600 lines; `AWAITING_SPLIT` is empty since F2a) and the clippy
gate (`cargo clippy --release --all-targets -- -D warnings` in
the same foreground run as the suite); every gate's scope is the whole of
`src/` including `src/bin/`, with allow-lists by file or function name and
never by directory alone. Gates still owed for rules 2, 4, 10, 11 and 15
are listed there and checked by a review grep until they exist.
Work-counter tests assert that one fetched vertex, one held thought, one
expansion, one quote, and one thirty-body assembly cost the same at N and
4N.

## 8. Persistence and continuity

Ledgers and genomes are rewritten atomically; a restore rebuilds the basis
from the lettered federation and drops what does not replay; old genome
layouts are not read: before beta a layout change is a recut with a wipe
of every persisted carrier, and no migration or legacy reader exists in
the tree (ruled 2026-09-07, `MU-STANDARDS.md` §10; Part V's migration
window is a beta concern). Within one layout, residuals, thoughts, and
dialects are never deleted.

## 9. The simulator

Specified: `mycelium-membrane`, a seeded in-process fleet of kernels and
voices with a harness-driven TUniversal, the real `Kernel`, `Treasury`,
court organ, and cover trie, a corpus as the work source, and genesis,
stake, unstake, transfer, burn as the only external inputs. Reports supply
per lane, stake ratio, velocity, slash count, home concentration, and the
ledger invariant every stroke. Policy ratios are studied here before any
change on the wire. Reference shape: the predecessor tree's `src/wire/membrane.rs`.

## 10. Landing order to beta

1. Economics: mint as issuance; `price` never zero from an empty bond;
   stake and unstake for voices; genesis; `supply` and `audit`; T accrual
   on open demands.
2. Docket: pending germs stay on the book and enter the docket with their
   deflation; residual germs admitted to `docket_connection`.
3. Rules: faces at nearest ring contact; several hits on one body chart as
   a section; cells never reopened on revisit.
4. Sharding: the registry lists a task only where its key homes; re-home on
   cover change; home counts sum to the corpus.
5. Temporal lane on the surface: standing extent bumped into pressure at
   listing; scene-state span at mint.
6. Identity and wire: voice keys; every client act signed; the xSEPTUM
   acts; Noise transport after signatures.
7. Binary unification: home directory, minted identities, bootstrap-only
   peers, stake-derived tier, corpus manifest, one unit template each.
8. Products: `mycelium-septum` gateway and SDK bindings; the uplink door;
   certification route; explorer reads; starter-genome export.
9. Simulator, then beta: hosted bootstrap hyphae, downloadable client,
   genesis allocations, release gates from the work counters.

## 11. What the wire must show before beta

Supply on every lane equal to genesis plus mint; every mint tied to a
release or a solve; the same rule on two ruled covers from one event;
docket depth oscillating; RSS flat against corpus size over hours on every
host; faces per germ in single digits; the first ARC task closed by a rule
two hyphae replayed and paid from our own bounty to a miner we did not run.
