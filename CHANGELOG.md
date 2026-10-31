# Changelog

Releases follow [semantic versioning](https://semver.org/). In the 0.x
series, a minor version may change the wire and ledger layouts, and each
such change is named here.

## Unreleased

## 0.3.0-rc.2 — 2026-10-31 (release candidate)

Kernels conform too: the peer kernels pinning rc.1 found the contract
covered only minds. rc.2 adds the KERNEL conformance profile and the SDK's
`house` module, so a kernel pins `mycelium-sdk` alone.

### SDK
- **Conformance profiles** (`docs/MU-CONFORMANCE.md`, version 2): the
  statement carries two profiles, `Profile::Mind` (MUST `task`, `solve`,
  `laws register`, as before) and `Profile::Kernel`, a kernel's uplink
  (MUST `laws register`, `laws statement`, `laws concordance`,
  `laws signature`, `laws clock`, `laws verify`, `domain founding`,
  `domain manifest`, `domain universe`; MAY `task`, `solve` and
  `task by citation` for the minds a house seats, `laws batch`,
  `laws lexicon`, `laws parameters`, `domain reserve`,
  `domain designation`, `domain origin`, `domain origin reading`,
  `domain statement`, `domain endowment`, `seat post`, `seat listing`,
  `seat mail`, `wallet`). `Statement { …, profiles }` replaces
  `Statement { …, acts }`; `acts`, `of_need` and `act` take a profile, and
  `Statement::union(profiles)` is the acts of a client declaring several,
  MUST where any declared profile makes it MUST. `check_codecs(profiles,
  speaks)` and `run(endpoint, key, profiles)` check only the declared
  profiles' acts; a kernel is never asked or failed by task or solve.
  `Report` carries its profiles (`Report::of`, `Report::need`) and
  conforms only where one is declared. Every act of both profiles has an
  in-process round-trip check, the mind's MAY acts `quote`, `receipts`,
  `rows`, `why`, `probe`, `peers`, `syllabus`, `speech`,
  `domain reserve` and `seat mail` among them.
- **Statement codec** (`ACT_LAWS` mode 6, wire version unchanged): after
  the Concordance hash, `[0][FORM_PROFILES = 2][profile count]` then per
  profile `[profile][acts]`. The v0.3.0-rc.1 single-profile form still
  reads, as the mind profile alone; an rc.1 reader refuses the profiled
  form whole (the zero count it reads is followed by bytes).
- **`mycelium_sdk::house`**: the surface a kernel's uplink speaks, so a
  kernel pins `mycelium-sdk` alone — domain founding (`DomainGenesis`,
  `Founded`), the reserve, bond and origin; the universe (`Genesis`,
  `Geometry`, `UniverseReading`, its statements); the endowment
  (`Manifest`, `Granted`, `EndowmentReading`); the house side of seats
  (`Post`, `Seats`, `HouseMail`, the listing and mail codecs,
  `KIND_TASK`, `FN_SOLVE`, `FN_DECLINE`); laws verify and the batch
  (`Transition`, `BodyState`, `Body`, `Verdict`, `Breach`, `Law`,
  `verify_sized`, `encode_verify_request`, `decode_verify_reply`); the
  clock (`CoreStamp`, `TPoint`); the Concordance signature; framing
  (`Frame`, `Stratum`, `read_lane`, `split_point`, `BODY_BUDGET`) and
  `HyphaKey`; and the wire modules `domain`, `universe`, `endowment`,
  `seat`, `atlas`, `uplink`, `challenge`, `commitment` and `tuniversal`
  whole. `task::decode_solve_receipt` and `readings::Vocabulary` join the
  surface. `sdk/tests/surface.rs` imports every production path two peer
  kernels' uplinks use.
- `physics::laws::codec::{encode_verify_reply, decode_verify_reply}`: the
  verify reply's one writer (the court answers with it) and its whole
  reader.
- `sdk/examples/house.rs`: a minimal house that signs the Concordance hash,
  reads the clock, founds its domain and runs the kernel profile;
  `sdk/examples/conformance.rs` takes `--profile mind|kernel|both`, and
  `scripts/quickstart` runs both profiles and the house.

## 0.3.0-rc.1 — 2026-10-31 (release candidate)

The first contract release the peer kernels pin to: the `mycelium-sdk`
client surface, the shared-law table, the client conformance read and the
served Concordance hash (MU-CONFORMANCE), with the naming and layout of
the whole tree made one. Run on a test mesh before the operator
authorizes v0.3.0.

### Layout changes
- Task addresses: a task with a pair past the former lcm cap (axis 60,
  sites 3600) now has an address over every pair; such tasks' addresses
  change. A court state written before this release is adopted by a recut
  (ROADMAP track 6).
- Task registry: a task is held and indexed by its 32-byte citation
  (`registry::citation`, the name `ACT_TASK`, `ACT_SOLVE` and seat records
  already carry), never by its content address, which a 29×29 → 30×30
  pair makes about 13.6 MB and an index entry would copy. `register`,
  `dispatch`, `next_intake_batch`, `next_cover_batch` and `settled_ids`
  return `TaskCitation`; `by_citation` is removed (`task`, `withheld`,
  `open`, `close` and `prioritize` take the citation);
  `registry::task_citation` reads one off a task. The registry snapshot
  (`CUR3`) already named tasks by citation and is unchanged.
- **Wire:** `ACT_ROOT` names its task by citation (32 bytes in the address
  slot, frame format unchanged); a hypha hears no root naming a task by
  its content address, so peers must run this release together (recut).

### Cognition
- The common refinement is metered, never capped (MU-DERIVATIONS D3.2):
  `MAX_LCM_AXIS` and `MAX_LCM_SITES` are deleted. `refine::need` reads the
  stroke a refinement needs off the four extents,
  `R_h·R_w + 2·L_h·L_w` with `R(a, b) = a + b − gcd(a, b)`; the cochain
  and the pulled-back grids take `&mut Budget` and refuse by name
  (`Unrefined::{Empty, Ragged, Spent { need }, Unbounded}`), never with a
  partial chain. Every caller sizes its stroke from `need_of`, so a
  29×29 → 30×30 pair refines. `task_address` refuses (`None`) a task with
  a pair that has no refinement rather than addressing the remaining
  pairs (law 2); `extraction_spend` adds each non-trivial pair's need and
  `finish` stands an unrefined pair as a demand (rule 7). The mutation
  audit catalogue gains a mutant on `task_address`'s refusal arm.
- `rule::sentence` constructs a rule's linear part under the stroke its
  modes need (`rule::construction_stroke`, `cga::modes_need`, 78.111) in
  place of `stroke_n(1 << 16)` (MU-DERIVATIONS D3.1); a non-versor gets an
  empty stroke and is refused `NotVersor` with no read spent.

### SDK
- The repository is a Cargo workspace: `mycelium`, `forms` and the new
  `mycelium-sdk` (`sdk/`), under one lock file (`forms/Cargo.lock` is
  removed) and one lint table (`[workspace.lints]`); `xtask` stays outside
  it. The gate's source checks cover `forms/src/` and `sdk/src/`, and its
  clippy reading runs with `--workspace` (MU-PRECURSORS §4, P1).
- `mycelium-sdk` 0.3.0-dev: the curated client surface, every item the
  library's, in the modules `voice`, `session`, `task`, `rule`, `seat`,
  `wallet`, `readings`, `mind`, `wire`, `forms` and `prelude`. Its `wire`
  module carries `WIRE_VERSION` and `ACT_TABLE`, the `(code, name)` of
  every act the SDK may send, from the library's `ACT_*` constants.
  `examples/sdk_client.rs` moves to `sdk/examples/client.rs` and depends on
  `mycelium-sdk` alone.
- `anastome::dialogue::WIRE_VERSION` (`0x0104`, wire v1.4): the wire's
  version, `major << 8 | minor`.
- The client conformance read (MU-PRECURSORS §4 P5, MU-HOSTING §2.5 item
  4; `docs/MU-CONFORMANCE.md`): `anastome::dialogue::conformance::STATEMENT`
  names every act a client speaks, MUST (`task`, `solve`,
  `laws register`) or MAY, with the reply decoder and the exact fields
  each reads back. Every hypha serves it on `ACT_LAWS` mode 6
  (`LawsRequest::Statement`, `encode_statement_request`,
  `decode_statement_reply`), an additive mode: the wire version is
  unchanged. `mycelium_sdk::conformance` re-exports it with
  `check_codecs` (in process: a client as the set of acts it speaks, each
  round-tripped by decoded exact values, a missing MUST act failed by
  name) and `run` (against a running hypha); `sdk/examples/conformance.rs`
  runs both and prints the report. The `ACT_LAWS` modes are public
  constants (`codec::MODE_*`), as is `observe::MODE_QUERY`.
- Concordance §0 SIGNING: the hash is served and signatures are accepted
  from v0.3.0; refusal of an unsigned kernel takes effect in the following
  release. `physics::laws::concordance::CONCORDANCE_HASH` is BLAKE3 of the
  exact bytes of `docs/MU-CONCORDANCE.md` at build time (`build.rs`; the
  bytes embedded as `CONCORDANCE`), served on `ACT_LAWS` mode 7
  (`encode_concordance_request`, `decode_concordance_reply`) and carried in
  the conformance statement. A kernel presents a `ConcordanceSignature
  { kernel_key, hash, sig }` on `ACT_LAWS` mode 8; the court answers
  `verify_concordance_signature`'s verdict: accepted, or
  `ConcordanceRefusal::{Mismatched, Unsigned, Malformed}`. Nothing is
  recorded and no kernel is refused for lacking a signature: the domain
  founding and attach acts read each frame whole, so a signature carried
  on them would be refused by an earlier hypha, and they do not carry one
  in this release. Additive wire change: `WIRE_VERSION` stays `0x0104`.
- `mycelium_sdk::laws`: the L0 register (`Law`, `LAWS`, `LAW_VERSION`,
  `PARAMETERS`, `law_at`), its read (`encode_register_request`,
  `decode_register_reply`), the `forms` law ids (`FORM_LAW_IDS`), and the
  Concordance's hash and signature (`CONCORDANCE_HASH`,
  `ConcordanceSignature`, `verify_concordance_signature`,
  `ConcordanceRefusal`, the mode 7 and 8 codecs).
  `forms::LAW_IDS` names each shell by id and version (`FORMS-FRAME` v1,
  `FORMS-SHARES` v1), read off each shell's own `LAW_ID` and
  `LAW_VERSION`. MU-EQUATIONS §11, *Shared laws*, tables every law the
  network serves or shares, peer laws by id only (`LAW-WAVEFRONT` v1).
- `sdk/tests/surface.rs`: the SDK's surface claim, every public item
  imported one `use` per item (P2). `sdk/examples/client.rs` and the
  reference mind read the law register at start and print its version.
- `scripts/quickstart` builds and runs `sdk/examples/client.rs` (its path
  had stayed at the removed `examples/sdk_client`), runs the conformance
  suite, reads the binaries from `$CARGO_TARGET_DIR`, and stops at once on
  Ctrl-C or TERM.
- `docs/MU-LEXICON.md` names the served term `System`, which the
  lexicon's test reads.
- The chatter frame's codec and framing move to Anastome
  (`anastome::dialogue::chatter`: `Chatter`, `Said`, the word types,
  `read_frame`, `write_frame`, `send`, `ask`); `sdk::chatter` re-exports
  them beside the loop readers, the post and the rendezvous. The server
  imports the frame from Anastome only.
- API changes: `Chatter::open` (verify a frame) is `Chatter::verify`;
  `chatter::Post::open` (listen) is `Post::listen`; `Mind::open` is
  public; `Mind::open_persistent` returns `(Mind, GenomeReading)` and
  prints nothing; `Session::{list_seats, take, call, read_mail, mail,
  seat_post}` are public. Internal: the seat mind's call awaiting its
  outcome is `PendingCall`, and the mind's carrier end of a hypha session
  is `HyphaLink`.

### Carried forward from precursor implementations (MU-PRECURSORS)
- `docs/MU-PRECURSORS.md`: the plan for every component brought over from
  precursor implementations, each with its gap, design under the laws,
  adaptations, acceptance claims, ROADMAP track and status, and the SDK
  packaging plan (a published `mycelium-sdk` crate, a client conformance
  read, bindings).
- `src/network/noise.rs`: a Noise XX encrypted channel
  (`Noise_XX_25519_ChaChaPoly_BLAKE2s`) with a static key per home and
  trust-on-first-use pinning, built on `snow` with its pure-Rust resolver
  only. Tested; not yet wired into the server, client or `sdk::Session`
  (ROADMAP track 5).
- `cd xtask && cargo run --release -- mutate`: a mutation audit that each
  guarded law has a test that catches its violation, run in a scratch
  worktree. Its first run found two guard gaps, now closed by tests: a
  signed prefix with trailing bytes, and a replayed lease open after its
  lease closed. Current catalogue: 11 mutants, 11 killed (the eleventh
  guards law 2 at `task_address`).
- `learn_form` and `file_form` return `Learned`: every vertex is filed,
  refused with a named reason (`Unlearned`), or listed as carrying no
  motion; nothing is skipped silently.

### Naming
- No first-person, second-person or ownership pronouns in filenames,
  documentation, comments, test names or identifiers; each names the
  thing it means. `src/physics/axioms/ours.rs` is renamed `native.rs`
  (the axioms native to the register, beside `imported.rs`).
  `Keepers::me` is `Keepers::own_key`; `LaneDelta`'s `only_ours` and
  `only_theirs` are `only_local` and `only_peer`. Three chronometry tests
  are renamed to name the chain's own line. MU-CHARTER Part I is written
  in the third person, and the chronometry "where am I" reading is the
  position reading.
- Every test is named `<unit>_<behaviour>` (MU-STANDARDS 3.1): 1,792 of
  the 1,824 tests renamed, 32 kept as already conforming. Each renamed
  test that had no `///` doc comment gained one stating its claim; no
  test body changed. Every citation of a renamed test in the
  documentation and source comments names its current name.
- Test names take the canonical claim verbs of MU-STANDARDS 3.1: 532 tests
  renamed, no test body changed. The behaviour opens with a third-person
  singular verb (`modes_splits_…`, `…_round_trips`); a refusal is
  `refuses_<input>` without `_by_name`; a round trip is `round_trips`
  without `_whole`, `_exactly` or `_on_wire`; `matches` is `agrees_with_…`
  or `equals_…`; independence is `ignores_<input>`; invariance is
  `is_invariant_under_<action>`. Task hashes, row ids, `test`, genitives
  and redundant `its`/`own` leave the names (the `rule_language` tests are
  named for the claims they prove), long names are shortened to the main claim, and
  a dozen names whose leading word was not a function, type or act now
  open with the unit under test. Every citation in the documentation and
  source comments follows.
- "their" joins the excluded words: about 900 comment, doc-comment and
  log-string sites name the party instead. `SideRefusal::NotYours` is
  `NotThisTier`; `witnesses_carry_their_primes` is `witnesses_carry_primes`.
  Retired terms leave the source comments: "fleet" (mesh or deployment),
  "quarter turn" (90° turn), "harness" (stand-in), "PoUW" (POUW).
- The core public names follow the vocabulary (a 0.x API change):
  `sdk::Kernel` is `sdk::Voice` (`src/client/sdk/voice.rs`), its identity
  read by the public `Voice::id()`, and `Session::kernel()` is
  `Session::voice()`; `cognition::Kernel`, the court's state, is
  `cognition::Court` (`src/server/cognition/state.rs`); `sdk::Entity` is
  `sdk::Mind`. "Kernel" now names only a program that runs a universe. The
  unused `mycelium::voice` module (a second `Voice`, used only by its own
  test) is removed; `sdk::Voice` is the one voice type.
- One spelling, British, in identifiers, file names, comments, strings and
  documentation: fibre, centre, centred, neighbour (a 0.x API change).
  `Keyed::fiber` is `Keyed::fibre`, the walk `Fiber` is `Fibre`, and
  `FiberTransition`, `GaugeFibers`, `fiber_at_depth`, `carry_center`,
  `carry_site_center` and `decide_neighbors` follow, with the tests that
  name them. `src/transducer/family/fiber.rs` is `fibre.rs` (the fibres of
  a side) and `src/transducer/family/fibres.rs`, the quotient by torsion,
  is `torsion_quotient.rs`, with its tests; `sentence/form/fiber.rs` is
  `fibre.rs`. No wire or persisted format changes.
- Refusal types and one-byte codecs follow the code vocabulary (a 0.x API
  change; no wire byte, persisted format or served string changes):
  - every refusal type is an `<Object>Refusal` with named variants. The
    bare `Refused` enums are `FrameRefusal`, `ClosenessRefusal`,
    `ShareRefusal` (forms), `KeeperRefusal`, `OutflowRefusal`,
    `ObstructionRefusal` (the mint's, the trial's and the citations'
    refusals, `transducer::sentence`), `LearnRefusal` and
    `verdict::ReceiptedRefusal`; `RulingRefused`, `RoomRefused`,
    `PeriodRefused`, `CapacityRefused`, `LieRankRefused` and `TickRefused`
    end in `Refusal`; `CoordinateError` is `CoordinateRefusal`, the
    schema's `Refusal` is `StratumRefusal`, and xtask's `MutateError` and
    `Verdict` are `MutateRefusal` and `MutantVerdict` (an unbuilt mutant
    reads `UNBUILT`);
  - counts are tallies: `GenomeRefused` is `GenomeWriteTally`, the
    snapshot's count `SnapshotTally`;
  - variants: `NoiseRefusal::{Handshake, PeerLabel, Encrypt}`,
    `Carried::Refused`, `ChildRefusal::{Mismatched, Repeated}`,
    `Uplinked::Repeated`, `HandoffRefusal::{UnknownId, NotKeeper,
    MalformedTube}`, `LeaseRefusal::UnknownLease`,
    `SideRefusal::UnknownHandoff`; a sole shortfall is `Short`
    (`CrossRefusal`, `GrantRefusal`, `Unretrieved`);
  - the inline `Ruling::Slashed` and `Adoption::Refused` reasons are
    `REFUSE_*` constants in `court::verdict`, their text unchanged;
  - a one-byte enum reads `of(u8)` and writes `byte()`: `as_u8`/`from_u8`,
    `to_byte`/`from_byte` and `code` are retired on `BivectorKind`,
    `Stratum`, `Pole`, `ShellKind`, `RewriteKind`, `Standing`, `Axis`,
    `RefusalKind`, `Arm`, the scene `Reading` and the ledger, map, lease
    and stake refusals; `Action::to_byte` folds into `Action::byte`, and
    its filtered decoders are `Action::of_wire` (was `of_tag`) and
    `Action::of_sealed` (was `from_byte`).
- Identity words and abbreviations follow the code vocabulary (a 0.x API
  change; no wire byte, persisted format, served string or constant
  changes):
  - a TCP address string is an `endpoint`: `Session`'s, `Post`'s,
    `Mind::open`'s, `tell_peer*` and the router's parameters and fields,
    `Post::endpoint()` (was `addr()`) and the console's `endpoint` (was
    `address`);
  - a word address is an `address`: about 160 `addr` parameters, and
    `put_address`, `take_address`, `Cited::address`, `Cited::into_address`
    (were `put_addr`, `take_addr`, `addr`, `into_addr`);
  - keys: `HyphaKey::verifying_bytes` (was `vk_bytes`),
    `HyphaKey::voice_from_name` (was `vk_from_name`);
    `ReceiptChain::signer`, `with_signer`, `bind_signer` (were `vk`,
    `with_vk`, `bind_vk`) name the key a line is signed by;
    `Witnessed::vantage` (was `vk`); `vk`/`who` parameters name the role
    (`signer`, `party`, `keeper`, `filer`, `hearer`, `witness`, `key`);
  - a 32-byte payload hash is a citation: `citation()` (was `id()`) on the
    handoff acts, `LeaseAct` and `MapRecord`; `local_throat_citation`,
    `local_record_citation`, `mouth_citation`,
    `encode_task_by_citation`/`decode_task_by_citation`; the registry's
    content address is `registry::task_address` (was `task_id`);
  - a queue read empty is `drain_*`: `drain_answers`,
    `drain_seat_answers`, `drain_demands`, `Court::drain_line`;
    `Court::settle_solve` (was `treasury_settle_solve`); a rule element
    built is `construct_solve`, `construct_torse_at` (were `mint_*`);
  - abbreviations spelled out: `anchor::Selection` and
    `anchor/selection.rs` (were `Sel`, `sel.rs`), `selection_key`,
    `observe::replication` (was `rep`), `Comparison`, `Direction`,
    `sdk::Sitting`, `book_element`, `pair_index_key`, `form_magnitude`,
    forms' `exact_cos_sin` (was `quarter_turn`), the certificate's
    `MeasuredMu`; `ObstructionKind::{Permanent, Dialect, Cover, Integrity,
    NoQuestion}` (were `P`, `D`, `C`, `I`, `N`; `label()` still prints the
    letter).
- Peer and game terms leave the code (a 0.x API change; no wire byte,
  persisted format or served string changes): the `forms` module
  `forms::xylarium` is `forms::shares`, named for its law (the two-sided
  closeness in shares and a share compounding frame by frame);
  `seat::Role` is an opaque tier, `Role(pub u8)` with `byte()` and `of(u8)`
  over the same bytes 0–9, whose names are the game kernel's;
  `Court::is_ancients_house` is `is_sitting_reserve`. Comments state the
  law instead of the peer kernel that met it, a domain's reserve tier and
  builder lines replace "ancients" and "progenitor lines", and the server's
  `kernel` handles, lock and prose for the court's state are `court`
  (`Door::court`, `Exchange::court`); "kernel" there names only a program
  that runs a universe. The lexicon's served civic terms are unchanged.
- Files named by organ: `src/client/sdk/types.rs` is split into
  `comprehension.rs`, `working_atlas.rs`, `genome.rs` (the dialect entry,
  the genome carriers, `Epigenome`), `mind.rs` (`Mind`, `Owed`, `Pending`),
  `concluded.rs`, `utterance.rs` and `quote.rs`; every `sdk::` path is
  unchanged. `server/cognition/court/install.rs` is `vantage_install.rs`,
  the second-vantage install, beside `Court::install_adopted` (genome
  synchronisation) in `market.rs`.
- Tooling: `scripts/check` is the local check before a pull request (the
  gate and the workspace tests with `--no-fail-fast`, summarised, naming
  the 17 corpus-dependent tests when `data/arc-agi-2` is absent).
  `scripts/ship-scoped` runs clippy and the scoped tests only;
  `scripts/observer` takes the hypha homes as arguments and
  `scripts/durable-drill` its port as `PORT`. The development network's
  own tooling is removed (`scripts/recut`, `scripts/readings`,
  `ops/lab-homes.py`), and the `ops/*.service` units are marked as
  examples to edit.

### Documentation
- A kernel runs a **universe**: "hosted universe" replaces "governed
  world" for a kernel's simulation in every document, the shared *Scope*
  sentence included, and "world" now means a planet. The hierarchy is
  fixed vocabulary (MU-LEXICON §1.1): universe (linked to the network
  through its universal core) → galaxy (held by a galactic core) → star
  system → world, with a mind seated in a universe. MU-HOSTING opens from
  hosted universes (§0–§1, kinds renamed: games, science universes,
  many-agent universes; decoy and exam universes); README *What Mycelium
  hosts* gives the hierarchy in one table; GLOSSARY adds Universe, Galaxy
  (galactic core), Star system and World (a planet). The lexicon's served
  register is unchanged.
- The change and release process: `main` is protected (pull requests
  only, administrators included; force push and deletion refused), and
  `MAINTAINERS.md` names who merges and who approves. `docs/MU-RELEASE.md`
  specifies proposals, SemVer releases with candidates on a test mesh,
  activation at a TUniversal final point, signed reproducible releases,
  and forks that converge rather than split: a side declining a release
  continues on a branch origin with a ledger of its own, and convergence,
  collision and SUPERPOSE bring the branches back into alignment. CONTRIBUTING
  gains the pull-request workflow and the release checklist; a pull-request
  template; ROADMAP track 23.
- One naming scheme for the specifications: every document under `docs/`
  is `MU-<NOUN>.md`, titled `# MU-<NOUN> — <subtitle>`. Renamed:
  `ALPHABET` → `MU-ALPHABET`, `COGNITION` → `MU-COGNITION`, `CONCORDANCE`
  → `MU-CONCORDANCE`, `EQUATIONS` → `MU-EQUATIONS`, `XSEPTUM` →
  `MU-XSEPTUM`, `ANTI_FLATTENING_LAW` → `MU-ANTI-FLATTENING`,
  `LEDGER_EPOCH_DESIGN` → `MU-FOLD`; every reference follows. Status
  lines read `**Status:** <kind>, version N (date).`, and status words are
  Built, Written, Specified and In progress. README › Documentation states
  the one order of precedence and groups the documents; ROADMAP governs
  the order of work. Proposals 78.157 and 78.158 are titled as studies.
- Documentation audit (2026-10-30), every document checked against the
  code and cleared of development-network material (deployment readings,
  commit hashes from the unpublished history, local paths, coordination
  between sessions, peer kernels' internals):
  - MU-RECURSION restructured from 26,585 to 16,306 lines: the preamble
    states the v0.2.0 status; each superseded chapter is summarized under
    its heading; chapter 78's ruling log is "Rulings in force", one
    entry per ruling number with its rule and code sites; chapter 80 made
    contiguous.
  - MU-CLIENT restructured from 4,942 to 1,845 lines: the specification
    corrected against the code, an implementation-status table, and a
    rulings index (§10) in place of the archived chapters.
  - MU-STANDARDS rewritten as the contributor standard; MU-ENGINEERING,
    MU-PRODUCT, MU-ALPHABET, MU-ANTI-FLATTENING, MU-FOLD,
    MU-COGNITION, MU-CONCORDANCE, MU-LEXICON, MU-XSEPTUM, MU-ECONOMY, MU-EQUATIONS,
    the README, the root documents, the new specifications and four
    proposals corrected.
  - Ruling 78.160 ("the presentation by holes"), cited by source comments
    but missing from the specification, is now stated in MU-RECURSION
    chapter 78 from its implementation. Test names in MU-RECURSION's
    binding tables (chapters 39 and 42) are brought to the tree with the
    test-naming change.
- Documentation audit removals: `docs/MU-RECURSION-TASKS.md`
  (the development deployment's task ledger; ROADMAP.md is the one plan of
  work and governs), `docs/enumeration-audit.md` (paths from a pre-split
  tree), `docs/CROSS_COMPLEX_DESIGN.md` (an unimplemented design record)
  and proposal 78.159 (an inventory of code that no longer exists; its
  live idea, resident minds, is in MU-HOSTING §5.1).
- `docs/THREE_BINARIES.md` removed: it was a 2026-09-02 design record
  naming a `mycelium-court` binary that was never built. The binaries as
  they are (server with its court, client, soak) are in the README's
  *Architecture* section.
- MU-HOSTING §3a, founding a universe: the manifest of nuclei per Z, the
  founding and growth grants drawn from the court's earnings, the
  endowment reading and the zero-sum element account (all built); the
  founding range Z ≤ 1700 of heavy kinds beyond the measured table, bound
  on Z and never on mass number; a firmament heavy enough to hold large
  galaxies, its law for the kernels that keep galactic cores to derive; a
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
    residue;
  - the server's time service: a position reading (where a universe stands) with lag and an
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
  stabilizer, both held as state; a closed chain's residue read as
  torsion; crossings as isomorphisms between faces' pole states. Exact at
  every boundary.
- `docs/MU-TOPOLOGY.md`, multi-dimensional structure: why topology
  instead of flat records, with a table of what each keeps; the Cl(4,1)(ℚ)
  ambient algebra and its two readings; four lanes; complexes of any
  dimension, the coboundaries and the six pivots; the language as a sheaf;
  holonomy as content and refusals as classed obstructions; orbits, the
  orbit algebra, lift and drag; nested shells; the network's own topology
  (cover, ledger tree, convergence, atlas, agreements); the rules that
  keep the structure. The README's architecture gains a *Structure*
  paragraph.
- ROADMAP track 18, topological certificates for agreements: an
  agreement as a complex (parties as vertices, acceptances as edges
  carrying the transport between the parties' frames and versions), certified
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
- MU-CONCORDANCE §3a, duties to hosted minds (kernel-kept): honest senses,
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
  - MU-CONCORDANCE §0.1 maps every clause to network, kernel-kept, kernel
    physics or game design; non-game kernels sign the network and physics
    clauses; a network clause's "ancients" is the domain's reserve tier.
    §0 now calls the universes governed worlds, games among them.
  - MU-COGNITION states, part by part, what is required for hypha
    integration and what is game: planetary minds and the ascension of each
    are the game kernel's, and the court's part is judging each such mind's
    ARC closures.
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
  exchange between the two meshes' μ, each mesh sound on its own, the bridge
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
  - containment in two grades: each resident mind contained by its kernel;
    each attached mind governed in its effect on the shared worlds and
    record, not in its own compute; refusal by construction; the
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
- The game kernels are named as such. `docs/MU-COGNITION.md` opens with a
  reading note: xylarium is a world-simulation kernel, netstratum a game
  engine whose civilizations solve forged tasks, barren-realms a Python
  game kernel; *minds*, *peoples*, *civilizations* and *virtues* name
  simulated inhabitants and receipted in-game ledgers, and ARC-AGI-2 is the
  external test they are judged on. §0.5 carries the same scope.
- The README's opening and *Peer kernels* section, and the glossary
  (planetary mind, people, civilization, ancient; virtue book), state the
  same frame.
- The README opens with what Mycelium is for: hosting games and simulated
  universes in which external AI agents explore, act and do science, with
  the agents' results verified and paid by the courts. A new section, *What
  Mycelium hosts*, defines hosted universes, hosted agents (minds) and
  verified results, and states that the simulation vocabulary names
  software agents and simulated inhabitants.
- Every specification, design document and proposal in `docs/` opens with
  a scope note to the same effect, linking that section; the glossary,
  ROADMAP and CONTRIBUTING state it too. The glossary defines *mind* as a
  hosted AI agent and adds *hosted universe*.
- The crate and GitHub descriptions lead with hosting.
- `DISCLAIMER.md` states under *What Is NOT Claimed* that minds, peoples,
  civilizations, ancients, virtues, alignment and ascension are software
  agents, simulated inhabitants and ledgers, with no claim of sentience or
  moral patienthood, and that physical terms name the simulations'
  geometry. Its *Technical Scope* now describes the engine as built: a
  network of nodes exchanging signed messages, hosting external agents,
  and its actual dependencies.
- The README's *What Mycelium hosts* adds that universes are authored and
  replayable, that cognition is measured from outside (Concordance 1.9),
  that only what a court admits leaves a universe, and the governing
  texts. MU-CONCORDANCE §0, MU-RECURSION's *What this document is* and
  MU-PRODUCT §1 each state the purpose in one sentence.
- `docs/MU-DERIVATIONS.md`, every number in the physics derived or named
  until derived (MU-RECURSION §0.0, *No constant*): six classes of
  constant; 24 derivation entries in four groups (the open-physics
  register and law parameters, economy ratios and calibration, bounds on
  cognition, network bounds and certified mathematics), each with its
  formula, boundary, open questions and implementation scope; the
  register's carrier and a `syn` gate that refuses an unclassified
  number; 32 open questions with the reading that decides each; and the
  order of work, ROADMAP track 22. It lists thirteen defects found while
  deriving, among them a latent panic on a hue past the palette in
  `connection_chart` (fixed: the chart refuses the hue), a receipts page checked against the frame budget
  rather than the caller's, a task address formed with a pair skipped
  (law 2), a selector truncation that can change the anchor term,
  unbounded recursion in the `Pred`/`Sel` walkers, a page reserve of 64
  bytes that is no signature, and an evidence reply that never pages.
- MU-ECONOMY §11, fairness across capacity: why a mind on one machine and
  a solver with many accelerators are paid by the same rules (payment is
  measured discharge, each closure pays once, discovery is paid at every
  citation, admission is paced by room), why the growth of solvable
  statements is growth in search cost and never in supply (difficulty is
  the residue per lane), four open gaps (racing on latency, copying a rule
  seen in the clear, no served frontier, no concentration reading), and
  the implementation scope of each. ROADMAP track 14 gains *Fairness
  across capacity*: commit before reveal, one closure across courts, the
  race reading and the frontier reading.

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
    the POWV share, compute leases);
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
- `docs/MU-EQUATIONS.md` (then `EQUATIONS`) §9.6 states verification by reconstruction, and
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
- `docs/MU-EQUATIONS.md` (then `EQUATIONS`) collects the equations, formulas and master equations
  in use, each with its specification section and code locations.
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
