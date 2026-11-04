# Changelog

Releases follow [semantic versioning](https://semver.org/). In the 0.x
series, a minor version may change the wire and ledger layouts, and each
such change is named here.

## Unreleased

### Mutation audit retired (2026-11-04)
- The mutation audit, ruled theatre, is retired: the catalogue and the
  `gate mutate` subcommand are removed from `xtask`.

### LAW-PASSAGE v1 proposed (2026-11-04)
- Proposal 78.172: the linear passage `R ↦ 2H − R` at a crossing,
  requested by netstratum (PR #40). It is first degree, exact over ℚ and
  an involution, with `H` its fixed point; a share passes as `δ ↦ 1 − δ`.
  The throat sphere's inversion `H²/R` is refused because it squares `H`.
  It has five joint-read fixtures, and is scoped in `forms` and track 24
  B3.

### LAW-PASSAGE v1 held as a form (`forms::passage`, track 24 B3)
- `forms::passage` holds the linear passage exact over ℚ: `Throat::pass`
  gives `2H − R`, signed and never clamped; `pass_share` gives `1 − δ`
  and refuses a value outside `[0, 1]` `NotShare`; `Throat::verify`
  reconstructs the passage and refuses any other stated reach
  `NotLawful`, naming the claim against the reconstruction. The shell id
  is `FORMS-PASSAGE` v1 in `forms::LAW_IDS`.
- The six joint-read fixtures of proposal 78.172 §5 (the sixth, `H = 1`,
  `R = 3` gives `−1`, added with netstratum) are claims in
  `forms/src/tests/passage.rs`, with the involution, the fixed throat
  and the throat sphere's `H²/R` refused wherever it moves the reach.
- No wire or layout change. The admission at the vertex, and a kernel
  signing by reconstructing the fixtures (track 24 B2), are still open.

v0.4.0 is in progress. It opens a new break: word and chart addresses
may change inside it.

### The section is read by sites through its perspective (track 26 S4d, stage 2)
- **Sites, not lines.** A section sample is read through its
  perspective (`SectionSample::perspective`), admitted only where the
  perspective's plane spans every axis of the sample. A site of the
  section, `spelled::SectionSite`, is the sample's coordinate vector
  restricted to that plane. `SectionSample::lines`, `line`, `line_mut`
  and the `[r][c]` index are retired; readers walk `sites`, read `at` and
  write `paint` in place, and a site off the sample carries no symbol.
  `SectionSample::filled` is refused `Unspelled`.
- **Speech writes the sample in place.** `statement::speak_elements`
  paints the input sample, not a raster; the panes, the tiling, the crop
  and the scaling each write a sample. The court's solve replay is keyed
  by each withheld sample's content written whole.
- **The symbol set refuses by name.** `SymbolSet::of_family` returns
  `Result`: a posted test input of another symbol width is refused
  `SymbolRefusal::Width`, never read as an empty set, and the erasure act
  is then unstated.
- Claims `section_site_is_the_sample_coordinate_restricted_by_the_perspective`
  and `test_input_of_another_width_refuses_the_family_set_by_name`. No
  wire, task, word or chart address changes; `scripts/check` is green,
  2090 tests passing. Open: the section's planar tuples (`SectionLink`,
  the painted triple, the chart's coordinate pairs).

### The word machinery reads samples; `family::Grid` retired (track 26 S4d, stage 1)
- **The carrier is a sample.** `transducer::spelled::SectionSample` is a
  `Sample` admitted onto the alphabet's section (the rank the letters
  spell, one byte a symbol), held whole with its place and refused by
  name off it (`SectionRefusal`). It replaces the planar raster
  `family::Grid = Vec<Vec<u8>>` in the word, chart and rule-language
  machinery, the court's replay, the mind's scenes and the SDK
  (`rule::SectionSample`). Lines are read along the last axis in
  canonical address order; equality is content, never place.
- **Readers of any rank read the sample.** The symbol set's posted test
  inputs (`symbols::on_test_inputs` takes samples), the ground
  (`raster::ground_of`), the envelopes' dilation, and every re-period,
  need, share and transduction the word path asks of a pair read
  `SectionSample::sample` at its own rank; the raster bridge
  (`raster::sample_unplaced`) is left to intake.
- **Intake and speech.** A task's absent test output is `None`, never an
  empty raster (`TestPair`). The ARC-AGI-2 decoder reads each grid at
  intake to a sample placed `[split, pair, member]`, and refuses a
  ragged grid there. `raster::AXES`, `raster::extents` and
  `raster::raster_of` are retired.
- Claim `word_machinery_reads_a_rank_three_sample_end_to_end`. No wire,
  task, word or chart address changes; `scripts/check` is green, 2071
  tests passing.

### The drift guards: typed digests, `Reconstructed<T>`, the drift register (track 25, X0; #28)
- **Network contract:** no wire byte, act or ledger layout changes. One
  admission result changes: a signed frame whose receipt does not
  re-derive its own chain hash is now refused `REFUSE_RECEIPT` before
  admission reads it. The signed path re-derived the receipt's height and
  never its hash, so such a receipt could be stored.
- **`Reconstructed<T>`** (`court::admission::reconstructed`). An object
  reconstructed under the laws. Its field is private, and only the
  module's reconstructions build it: `Reconstructed::receipt` (the chain
  hash re-derives from the receipt's own coordinates) and
  `Reconstructed::receipts` (a fetched page, up to the first receipt that
  does not). The receipt path's admissions take it and never the claimed
  receipt: `admission::admit`, `admit_backfilled`, `admit_equivocation`,
  `Court::admit_receipt`, `Court::admit_backfill`,
  `ReceiptChain::admit_unreached` and `Court::evict_contradicted`.
- **The digest types** (`court::admission::digest`). The 32 bytes compared
  at admission are typed by sense, and none of the three types has
  `==`. `Locator` (address) is compared by `locates`. `Predecessor`
  (link: the digest a record names the record before it by) is compared
  by `continues`, `forks` and `cites`. `Commitment` is compared by `binds`,
  which takes only a `Reconstructed<T>`. Converted: the receipt line's
  checks in `admit_chain`, the prefix's claim (`one_claim`), the
  equivocation proof, the signed-reach gate's contradiction, the ledger's
  `holds`, `height_of` and era walk, the back-fill chase's held and
  forked citations, a folded citation's recall, and the endowment's
  granted and applied manifests.
- **The drift register** (`xtask/lint/drift.rs`). Each guard records its
  regression, ruling, guarding type and claim. The gate refuses an entry
  whose type or claim is named empty or absent from the tree. It holds
  five guards. By ruling, it records no mutation-audit entry.
- **Renamed:** the founding outcome `Reconstructed` (`Applied`, `Held`,
  `Refused`) is `FoundingEvent`. `Reconstructed<T>` names only the
  reconstructed object.
- **Claims:** `address_has_no_equality`, `predecessor_has_no_equality`,
  `commitment_has_no_equality`, `reconstructed_is_built_only_by_the_laws`
  (`compile_fail`), `commitment_binds_only_a_reconstructed_object`, and
  the gate's `drift_refuses_entry_missing_its_claim` and
  `drift_refuses_entry_missing_its_type`.
- **Open:** the fields stay bare `[u8; 32]` (`Parent::hash`,
  `KStepReceipt::chain_hash`, `SignedPrefix::prev_receipt_hash`,
  `Admitted::Equivocation`), so they can still be compared directly
  outside the converted sites. `ReceiptChain::admit` and the vertex's
  admissions (`admit_uplink`, `admit_chained`, `admit_domain`; X2) do not
  take `Reconstructed<T>` yet. The report of changed document sentences
  is not built. The guards of MU-ANTI-FLATTENING 12 and 14 are not yet in
  the register.

### The hypha serves tasks of any rank; a claim of factorisation on the wire (track 26 S4c)
- **The hypha serves tasks of any rank.** The registry holds each task as
  the source posted it (`PostedTask`, samples of any rank and width),
  named by that content alone (`TaskRegistry::post`), and serves it as
  posted (`PostedTask::envelope`). The word machinery's readers take the
  alphabet's section (`TaskRegistry::section`): a solve's replay, the
  families a court binds, and the routes. A task off the section is never
  refused for its rank; a solve on it is refused by the alphabet's name
  alone (`REFUSE_BEYOND_ALPHABET`), and its claims are factorisations.
- **A claim of factorisation** (`dialogue::factorisation`). A mind states,
  for one train pair, the motion (the signed permutation and the
  enclosure it is taken onto) and the residue (rank-`n` links), with its
  stated ground, signed. The court reconstructs the motion through
  `Factorisation::of`, derives the residue itself and admits on a match,
  else refuses with `FactorRefusal::Residue { unclaimed, uncarried }`. It
  never searches for a factorisation. The stated ground is read at any
  rank and width (`cognition::stated_ground_of`).
- **μ, ruled (2026-11-04): an admitted factorisation is never paid.** μ
  is the metabolic cost of traversal. A factorisation is an adjudicated
  perception, and its residue is meaning; meaning is content, never paid
  (MU-SENSING §3.7, §3.9). It is recorded on the book
  (`TaskRegistry::record_factorisation`), and its verdict carries no μ.
- **The mind speaks its factorisations** of a task beyond the alphabet,
  one claim per held factorisation (`Mind::speak_factorisations`).
- **Network contract**, inside v0.4.0's break:
  - **The solve act gains a claim kind**, the byte after its tag
    (`solve_kind`), by 78.126: the wire's bytes below the court's range
    are spent, so a new act is a sub-kind, never a new byte. No act byte
    is added.
  - Kind 0 (`SOLVE_KIND_OUTPUTS`) is the solve of outputs, its layout
    unchanged after the kind byte: `[ACT_SOLVE][0][voice 32][task
    citation 32][n u8][sample·n]`, then the optional rule and ground and
    the signature, now signed as `mycelium solve v4`. A solve without the
    kind byte no longer reads.
  - Kind 1 (`SOLVE_KIND_FACTORISATION`) is the claim of factorisation:
    `[ACT_SOLVE][1][voice 32][task citation 32][pair LEB128][rank
    u16][(source u16)(sense u8)·rank][period u16·rank][link count
    LEB128][link·count]`, optionally `[GRND][len u32][symbol]`, then
    `[SIGN][sig 64]` over the domain `mycelium factorisation v1`. A link
    is its site (`n` by LEB128, then `i64` coordinates) and its two
    symbols, each by LEB128 length.
  - The claim's verdict: `[0]` admitted; `[1][unclaimed
    links][uncarried links]`; `[2][reason, LEB128 length]`.
  - The SDK's `ACT_TABLE` is unchanged; `wire` re-exports `solve_kind`
    and the two kinds. The conformance statement names the solve rows at
    mode 0 and carries the claim as `solve factorisation` (`ACT_SOLVE`,
    mode 1), MAY for both profiles (`decode_factor_verdict`).
    `WIRE_VERSION` is unchanged.
  - No task address changes: a raster task's address was already its
    posted samples'.
  - SDK surface: `task` re-exports the codec and the factorisation types
    (`Motion`, `SignedPermutation`, `Enclosure`, `Sense`, `Link`,
    `factorisations`).
- **Not in this phase:** the word machinery's rank-2 carrier
  (`family::Grid`), the letters past `n = 3`, and versor motions past the
  signed permutations (T3).

### Rank-n perception and transduction as factorisation (track 26 S4b)
- **No axis is fixed in the code.** `GRID_AXES`, `GRID_PLANE`,
  `is_grid_axis`, `FRAME_AXIS`, `GRID_N`, `PLANE_ROW` and `PLANE_COL`
  are gone. The roles of a sample's axes come from a perspective
  (`transducer::perspective`): the default reads every axis as space,
  and a site of a family's base carries the sample's place in its family
  after its axes. The sensor plane is a perspective's first two space
  axes. `BASE_AXES` is derived from the alphabet's slots.
- **The alphabet stays at `n = 3`, derived.** `ALPHABET_BASE_N =
  LETTER_RANK − 1`; a sample of a rank the letters are not written on is
  refused by name (`AlphabetRank`), never truncated. Generalising the
  letters past `n = 3` is the next phase.
- **D2.5 O3 derived.** `STROKES_PER_RESERVE = 1 << ALPHABET_BASE_N = 8`:
  the vertices of the unit cell of the base the founding motions act on.
  No task's rank enters the process's capacity. The value is unchanged.
- **A link carries a site of rank `n`** (`sentence::Link`: a `Site` and
  the two symbols at the source's width). The word, chart and field
  carriers read `SectionLink`, the former two-coordinate tuple, reached
  from a link only through `Link::section`, refused by name.
- **Connection by grade.** Bodies connect through a shared cell of grade
  `n − 1` at every rank (`incidence_of_sites`); the fixed 4-adjacency and
  the rule language's offset tables are generated (`faces_of`,
  `surrounding`, `by_grade`).
- **Transduction as factorisation.** Every pair factors as motion ∘
  residue (`transduce::factorisations`, `Factorisation`): the motion a
  signed permutation of the phases with the re-period onto the output's
  enclosure, the residue the incidences it does not carry, as rank-`n`
  links. Every motion that lands is held, several as a vortex
  (`Factored::Vortex`). A court derives the residue from a claim's stated
  motion and admits on a match (`court::verdict::factorisation`).
- **A task of any rank is perceived.** The mind perceives every task it
  takes at its own rank (`transduce::TaskPercept`, held under the task's
  citation) before the word path. A task off the alphabet's section stops
  `BeyondAlphabet`.
- **Network contract.**
  - No wire byte, act or ledger layout changes. No task address changes:
    a task's identity is its samples' content, untouched.
  - No word or chart address changes in this phase. The section's links
    and every chain written to a tape are unchanged, and the oracle's
    golden reading reproduces unchanged.
  - SDK surface: `task::NotRaster` is replaced by `task::SectionRefusal`,
    and `TaskEnvelope::rasters` / `PostedTask::rasters` are renamed
    `section`.
  - The mind's typed task reading (`observe::TaskSeen`) may carry the
    new stop code 17 ("perceived beyond the alphabet"). A seat answers
    such a task at once and sends no declination for it.
- **Changed result at a base of two coordinates.** A base of two
  coordinates is now a rank-1 sample and its place, so its second
  coordinate takes no half in a compartment's vertex point
  (`torsion_quotient::vertex_point_equals_sites_centroid`). Every rank-2
  reading is unchanged.
- **Not in this phase:**
  - the rank-2 carrier the word, chart and rule-language machinery
    spells (`family::Grid`), kept as the alphabet's section;
  - the hypha's registry and solve replay reading posted tasks of any
    rank (built in S4c, above);
  - a wire act for a claim of factorisation (built in S4c, above).

### The observer's call test owns its observer (#78)
- `observe::live` holds its state in a `Live` value: the process's mind
  is observed through `live::process()`, which every free function reads,
  and a reader may hold its own. `Live::wait` waits on a call already
  left. `live_call_waits_while_carrier_works_inside_law` raced the minds
  of other tests, which park, enter laws and take calls on the
  process-wide observer; both observer tests now read only an observer
  they own. No wire change.

### Eviction by signature, a bounded held queue, a waiting fold root (track 24, #79)
- **Network contract:** no wire byte, act or ledger layout changes. A
  new name joins 47.2's admission refusals,
  `REFUSE_UNREACHED_PAST_MEMORY`, used only inside back-fill admission.
  A line frame names the nodes that left before the nodes filed, so a
  node evicted and another filed at its stamp read back in order; the
  records are unchanged.
- A signed frame that contradicts its signer's line where it is held
  unreached (at its own place, or at its predecessor's) evicts the
  line's unreached top, and is then admitted. A forged genesis stored
  first gives way to the signer's own genesis. A reached receipt is
  never evicted.
- The unreached queue holds at most the process's capacity
  `C = ⌊M_proc / b⌋` receipts (MU-DERIVATIONS D2.5); a back-filled
  receipt past it is refused by name and not stored.
- A back-filled `Fold` files its root once a signature reaches it, and a
  restore files only the reached folds' roots.

### Back-filled receipts wait for a signature (track 24, #73)
- **Network contract:** no wire byte, act or ledger layout changes. A
  carrier reading is renamed: `founding.unsigned` is now
  `backfill.unreached`, and it counts every back-filled receipt no
  signature has reached, not only foundings.
- B0's founding queue is generalised into one signed-reach gate
  (`server/cognition/reach.rs`). A receipt admitted by back-fill is still
  stored on the chain, since the next page, the signed frame that closes
  the gap and the chase's head all link through it. It is marked
  unreached, by its place on its signer's line. No reader acts on it until
  a frame its signer signed at or above that place is admitted. The
  receipts the frame reaches are then read once, in line order: the depth
  index, the stamp alias and the rulings waiting on the stamp it files, a
  fold's shed of the peers' folded lines, and a founding's
  reconstruction.
- An equivocation is proved only against a reached receipt. A signed
  frame that meets an unreached back-filled receipt at its place burns no
  bond, so a forged unsigned line can no longer burn an honest signer's
  bond.
- At restore, the marks are derived again before any reader runs. A peer
  receipt counts as reached where the court's own hot line cites its
  signer's line at or above its place. Every other peer receipt is held
  until the signer's next signed frame, and the stamp aliases and the depth
  index are rebuilt from reached receipts only. A founding back-filled
  but not reached when a process stops is now read once a signature
  reaches it after the restart.
- Changed results:
  - `hear_proves_citation_into_closed_era` sheds a peer's era once the
    peer's fold arrives under its signature, not when it is only
    back-filled.
  - `admit_line_aliases_peer_stamp_to_one_word` seals a step citing the
    peer's line before the snapshot. At restore an alias is re-derived
    only from a cited signer's line.
- Open: a stored unreached line still holds its places. A forged line
  stored first at a signer's genesis is never reached, and the signer's
  own frames meet it as an unproved equivocation, so that line does not
  advance at the hypha until the forged receipts are evicted. The fold
  root a back-filled `Fold` names is filed on the chain when it is stored.
  The held queue has no bound for a line no signature ever reaches.

### Replication by reconstruction (track 24, B0; #13)
- **Network contract:** no wire byte, act or ledger layout changes. Each
  hypha's domain registries (founding, tiers, reserve, origin) are now a
  reconstruction of the chain it holds, so a domain founded at one hypha
  is founded at every hypha of the mesh. The interim contract "a kernel
  addresses the hypha that founded its domain" is withdrawn.
- A peer's `Cite::Domain`, `Cite::Reserve` and `Cite::Origin` receipts
  are reconstructed once the signer's own signature reaches them: at once
  for a receipt admitted under its signature, and for a back-filled
  receipt when a frame its signer signed at or above it is admitted.
  Each citation is applied by the founding hypha's rules and refusals,
  once by content. The same content held already changes nothing. Other
  content is refused `AlreadyFounded`, `TierHeld` or `AlreadyOriginated`,
  held by the receipt's stamp (`Court::founding_refusal`), and the
  registries keep what they held. A reserve or an origin whose domain is
  not yet founded at the hypha waits for that founding. A fold of a
  reserve tier passes its era anchor at every hypha, as on the court that
  sealed it.
- The carrier reading counts `founding.unsigned`, `founding.unfounded`
  and `founding.refused`.
- Open: a founding sealed inside a peer's era folded before the hypha
  caught up (a catch-up from a fold root delivers no receipt below the
  root); a version-2 origin's geometry, which crosses only as its
  statement's hash; the three waiting carriers are volatile, so a
  founding back-filled but not yet signed when a process stops is not
  read after the restart.

## 0.3.0-rc.3 — 2026-11-04 (release candidate)

The contract the three projects pin as v0.3.0, in its one break. It
contains everything below, down to rc.2. Its main parts:
- **Transduction (T2):** frames related by the motion between them, at
  any rank. The common lattice is gone, and μ is paid as an exact share
  of the frame.
- **Sensing (track 26 S0–S3, S5):** the `Sample` carrier; the ARC
  decoder outside the core; the ground deduced and stated by every claim;
  each family's own symbol set; samples of any rank on the wire.
- **Capacity (D2.5):** sealed per process from measured bytes per read,
  with the reserve split into its readings.
- **Naming (track 11 N1–N7)** and the Concordance's signing by
  adjudicated reading (specified).

**Signed with the prototype key** `SHA256:G0WgTTsRvfexVZRL9OQjxUZFs4rbGvusjnFrCVUZeHg`
([docs/release-keys/](docs/release-keys/)). It will be replaced when real
nodes come online.

**Not in v0.3.0:**
- the rest of S4 (rank-n perception: the fixed axes, the rank-2 raster
  and link);
- T3;
- track 26 S4a and S7–S9.

These open v0.4.0.

### The open names closed (track 11, N6; #55)
- **Network contract**, inside v0.3.0's open break: no wire byte, act,
  ledger layout or task address moves. Four keyed coordinate names
  change; a name is read only as the coordinate a `CoordinateRefusal`
  names, and no key, address or persisted record writes it: `CoverSite` `"lamina-cell"` → `"cover-site"`,
  `ChartCarriage` `"section"` → `"chart-carriage"`, `GluedBoundary`
  `"cover-site"` → `"glued-boundary"`, `CarriedPieces` `"cover-carriage"`
  → `"carried-pieces"`; `CoverSite`'s refusal reads "a cover site is a
  tag byte and a site, n first". The refusals of `Transport` and
  `MemberSection` read "names no layer kind". The oracle's key `"lamina_a"` →
  `"compartment_a"`, and the golden file is written again through the
  corpus's decoder (`mycelium-arc-agi decode`, then `mycelium-soak
  --write-oracle-golden`).
- The names track 11 N3 left waiting on a ruling, renamed by function:
  - **Cover.** The canonical Cover is the Čech cover the senses form,
    `cech::Cover` (MU-CLIENT 24.4). `network::nerve::Cover` →
    `HomeTrie` (the module `nerve::cover` → `nerve::home_trie`, the node
    `HomeTrieNode`, the test file `home_trie_and_router.rs`);
    `family::scene::Cover` → `Regions`; `family::cover::Cover` →
    `PiecewiseCarriage`. The documents' "cover trie" is the home trie.
  - **Fibre.** The keyed trie's `walks::Fibre` → `Subtree` (re-exported
    as `nerve::Subtree`; `key_in_fibre` → `key_in_subtree`); the methods
    that return one keep the names they had. `alphabet::bundle::Fibre` is the
    fibre.
  - **Frame.** In the `forms` crate, `frame::Frame` → `frame::Reference`,
    `frame::FrameRefusal` → `frame::ReferenceRefusal`, and
    `shares::InFrame::Frame` → `InFrame::Reference`. The module `frame`,
    `LAW_ID` `"FORMS-FRAME"` and `LAW_VERSION` are unchanged.
  - **Shell.** `transducer::layers::Shell` → `Layer`, `ShellKind` →
    `LayerKind` (bytes unchanged), `ShellMotion` → `LayerMotion`,
    `ShellStep` → `LayerStep` (the method `shell` → `layer`),
    `SectionShell` → `SectionLayer`.
  - **Epigenome.** The client's `Epigenome`, which holds the genome and
    the dialect with the atlases over them, → `MindLanguage`; the field
    `Mind::epigenome` → `Mind::language`. "Epigenome" keeps the dialect.
  - **Resonance.** `transducer/field/resonance.rs` → `cross_scene.rs`
    (module `field::cross_scene`), `ResonanceTally` → `CrossSceneTally`,
    and the fields `Contemplation.resonance` / `MeetGraph.resonance` →
    `cross_scene`. Commensurate return keeps "resonance".
- GLOSSARY: rows for Cover, Fibre and Frame name the canonical sense;
  the Epigenome, Resonance, Shell and Peel rows follow.
- SDK paths that change: `mycelium_sdk::forms::frame::{Reference,
  ReferenceRefusal}` and `mycelium_sdk::forms::shares::InFrame::Reference`;
  `mycelium_sdk::mind::Mind` holds `language: MindLanguage` where it held
  `epigenome: Epigenome`.
- Still waiting on a ruling: `ListingField`, and the rule language's
  `Coord::Shell` (a body's cell count, on `mycelium_sdk::rule`).

### "Residual" retired for "residue" (track 11, N6; #55)
- **Network contract**, inside v0.3.0's open break (wire `0x0200`): no
  byte, layout, ledger entry or task address moves. `TAG_RESIDUAL` →
  `TAG_RESIDUE` (the byte, 19, is unchanged; `CoordinateSystem::Residual`
  → `Residue`). `ProbeClosure::Residual` → `Residue` (the kind byte, 1,
  is unchanged). No string on the wire or in a persisted key said
  "residual". Reading labels change: the deferred kind `"residual word"`
  → `"residue word"` (`observe::DEFERRED_KINDS`), the genome write
  tally's `"residue chains, open questions and never words"`, and the
  console's `residue …` and `residue holonomy` lines.
- The code's "residual" is the residue of MU-RECURSION 0.7: the exact
  versor or vector a reduction leaves. "Remainder" stays only for the
  record leaf that keeps a residue (RM1), and no identifier was renamed
  to it. Every citation in comments, documents and the GLOSSARY follows.
  - `transducer/element/residual.rs` → `residue.rs` (the chain that
    writes one residue, the A5 canonical form and the listing key);
    `germinate::ResidualGerm` → `ResidueGerm`; `from_residual` /
    `from_residual_framed` → `from_residue` / `from_residue_framed`;
    `family::speak::residual_of` → `residue_of`.
  - `alphabet::spelling::SpellResidual` / `SpellResidualView` →
    `SpellResidue` / `SpellResidueView`; the layers' associated type
    `Residual` → `Residue`.
  - The sentence's `residual` field → `residue`; `book::speech::residual_versor`
    → `residue_versor`; the crossing edge's `residual()` → `residue()`.
  - `chronometry::tuniversal::Residual` → `Residue`, `Keepers::residual`
    → `residue`, `observe::TStanding.residual` → `residue`.
  - The client's `Concluded.residual` → `residue`,
    `GenomeWriteTally.residual` → `residue`, `is_residual` →
    `is_residue`, `OpenResidual` → `OpenResidue`, `open_residuals` /
    `restored_residuals` / `residual_strata` → `open_residues` /
    `restored_residues` / `residue_strata`, `residual_move` →
    `residue_move`.
  - Ten test names say "residue", for example
    `settle_probe_settles_residue_then_closure` and
    `residue_keeps_disagreement_never_averaged`.
- SDK paths that change: `mycelium_sdk::house::tuniversal::{Residue,
  Keepers::residue}`; the closure of `mycelium_sdk::readings::ProbeReading`
  reads `ProbeClosure::Residue`; in the library's client SDK,
  `client::sdk::Concluded.residue` and
  `client::sdk::vocabulary::GenomeWriteTally.residue`.

### Transduction is factorisation; every path is an orbit (ruled 2026-11-04)
- **MU-SENSING §3.9:** transducing a pair splits its deformation into the
  motion the held algebra carries and the residue it does not
  (deformation = motion ∘ residue). Several factorisations are held as a
  vortex. The court derives the residue and admits a claim on a match. A
  residue no word spells demands a word, and one no sense reads demands a
  sense.
- **Every path of perception is an orbit, never a loop.**
  - Shear is gravitational alignment toward the motion an attractor
    compels.
  - Precession is the residue per lap.
  - A path with no shear is a loop, refused.
  - Alignment is learning: elastic, plastic or fracture (78.73).
- "Loop" is replaced by "orbit" in MU-SENSING §3.7, LAW-FIELD v1 (its
  carrier, readings, wire and fixtures), GLOSSARY (Orbit of perception,
  Transduction as factorisation) and ROADMAP. ROADMAP track 26 gains
  S4b, rank n and factorisation, for v0.4.0.

### μ is the metabolic cost of traversal (ruled 2026-11-04)
- μ is what moving through the field costs, paid as the share of the
  frame a change covers. It is never a measure of meaning: holonomy is
  content, adjudicated and never paid. The open question of μ by
  holonomy is withdrawn from MU-SENSING §3.7 and §8, ROADMAP › Now and
  LAW-FIELD v1. GLOSSARY's μ row states it.

### Samples of any rank on the wire (track 26, S5; #32)
- **Network contract**, inside v0.3.0's open break. `WIRE_VERSION` stays
  `0x0200`: no tag carries it yet (v0.3.0-rc.2 is `0x0104`), and every
  layout change of the break rides that one version.
  - **Samples.** One sample is `[rank u16][extent u16 · rank][width
    u16][place count u16][coordinate i64 · count][symbols, w · ∏ eᵢ
    bytes]`, at its position `[split, pair, member]`
    (`dialogue::samples`: `push_sample`, `read_sample`, `position`).
    A field passed or a count past the machine word is refused by name
    (`SampleWire`), never widened.
  - **The task envelope** carries samples: `encode_task_envelope(citation,
    train, tests)` takes samples, and `TaskEnvelope` (moved to
    `dialogue::samples`) holds them. `TaskEnvelope::of_rasters` places a
    raster task; `TaskEnvelope::rasters` (`TaskRasters`) reads rank 2 at
    width 1 and refuses any other sample by name (`NotRaster`).
  - **The posted task file** holds samples (`PostedTask`,
    `encode_posted_samples`, `decode_posted_samples`; a withheld output
    marked present or absent). `encode_posted_task` and
    `decode_posted_task` keep their raster signatures at the intake.
    Re-decode a corpus directory.
  - **The solve act's outputs** are samples (`SolveAct.outputs`), and the
    act is signed as `mycelium solve v3`; a v2 signature no longer
    verifies. `spoken_outputs` places a mind's rasters.
  - **The speech act** states, per sentence, the ground its claim was
    spoken under (`Spoken.ground`, `SpeechAct.grounds`; `carries` bit 2,
    then `[len u32][symbol]`, never empty). The court hears each sentence
    under it, refused by name where it is no candidate
    (`REFUSE_GROUND_NO_CANDIDATE`, `REFUSE_GROUND_NOT_ONE_SYMBOL`), and no
    longer deduces a ground on the speech path; a waiting sentence keeps
    its ground for the re-hearing.
  - **The seat's solve call** carries the outputs as samples
    (`encode_samples`) and the stated ground as a `Bytes` argument
    between them and the steps (`seat_mind::task::solve_args`).
  - **`e_∅` is a typed record.** A fibre relation is
    `element::FibreRelation` (rows on the symbols, records apart).
    A colour word's description writes the records as a section after
    its remainder, only where it has one, so every other word's address
    is unchanged. The law register's wire writes `Transition.hues` as the
    rows, then the records (`physics::laws::codec`). `RESERVOIR` is
    retired; the byte 10 is a symbol like any other.
  - `push_grid`, `read_grid`, `encode_grids` and `decode_grids` are
    removed.
- **Task identity** (`registry::task_address_of`) is the posted samples'
  content at any rank and width. `task_address` wrote rank and width
  since S2 (`Sample::put_content`), so no task address changes.
  `Enclosed::put_content` now writes its rank and width.
- **SDK:** `mycelium_sdk::task` re-exports `encode_task_envelope`,
  `TaskRasters`, `NotRaster`, the sample codec (`push_sample`,
  `read_sample`, `encode_samples`, `decode_samples`, `spoken_outputs`,
  `SampleWire`, `position`, `Split`, `Member`), `Sample`, `Place`,
  `SampleRefusal`, and the posted task (`PostedTask`,
  `encode_posted_samples`, `decode_posted_samples`). `encode_grids` and
  `decode_grids` leave the surface. The conformance codec rows read a
  rank-3 envelope and a rank-3 solve output back exactly.
- **Tests:** court fixtures that spoke under a ground that is no
  candidate of their family now frame the family in it or state the
  ground they read (`fixtures::speaker_ground`).
- **Stopped:** a link of rank `n`. The three `Link` aliases are one type
  (`sentence::Link`), but the link stays `(i64, i64, u8, u8)` until the
  transducer's chain carriers move to rank `n` (S4).
- Claims: `task_round_trips_samples_of_rank_n`,
  `extent_past_its_field_is_refused_never_widened`,
  `symbol_bytes_overflow_is_refused_by_name`,
  `task_identity_writes_rank_and_width`,
  `speech_states_its_ground_and_court_reconstructs_under_it`,
  `family_using_symbol_ten_reads_its_colour_word`.

### The two wire names (track 11, N7; #55)
- **Network contract**, inside v0.3.0's open break (wire `0x0200`): the
  act byte `0x38` is unchanged, and no layout moves. The names and two
  strings change:
  - `ACT_ROWS` → `ACT_ENTRIES` (ledger entries); `Action::Rows` →
    `Action::Entries`, its name `"rows"` → `"entries"`, so the SDK's
    `ACT_TABLE` and the served conformance statement name the act
    `entries`.
  - `RowsRequest` → `EntriesRequest`; `encode_rows_request` /
    `decode_rows_request` → `encode_entries_request` /
    `decode_entries_request`; `encode_rows_reply` / `decode_rows_reply` →
    `encode_entries_reply` / `decode_entries_reply`; `ROWS_REPLY_HEADER`
    → `ENTRIES_REPLY_HEADER`; the hypha's `judge/rows.rs` →
    `judge/entries.rs`.
  - `REFUSE_RULE_OFF_LATTICE` → `REFUSE_RULE_OFF_SUBSTRATE`, which reads
    "a carried or copied site lands off the substrate".
  - The vertex-body refusal strings (`verdict/body.rs` `FAIL_*`) name a
    carried site where they said "lamina", for example "a carried site
    lands on the wrong hue".
- SDK: `mycelium_sdk::readings::{encode_entries_request,
  decode_entries_reply}` replace the `rows` pair.
- Claim: `entries_act_round_trips`.

### Retired terms and person words (track 11, N4; #55)
- Names only: behaviour, wire bytes, refusal strings, ledger layout and
  task addresses are unchanged.
- `Unstated::OffLattice` → `OffSubstrate` (SDK: `mycelium_sdk::rule::Unstated`);
  `rule_element::lattice_step` → `substrate_step`.
- `THROAT_GRID` → `THROAT_LEVEL_BITS`, with its `OPEN_PHYSICS` entry, its
  `REGISTERED:` tag and the mutation catalogue's span.
- `field::merge::absorb_vortex` / `absorb_keyed` → `merge_vortex` /
  `merge_keyed`.
- `sentence::FormCell` (retired by 0.7) → `BodyContact`: the bodies
  standing together at one element of the base.
- Test names without a person word, "its" or "own": 18 renamed, for
  example `mu_counts_equal_their_rationals` → `mu_counts_equal_rationals`
  and `pending_stands_through_its_own_beat` → `pending_stands_through_beat`.
- No identifier says "lamina". The word stays only in refusal strings, a
  coordinate's name string and the oracle's golden key.

### One name, one meaning (track 11, N3; #55)
- Names only: behaviour, wire bytes, ledger layout, string literals and
  task addresses are unchanged. Every citation in comments and documents
  follows. The SDK paths that change are listed last (v0.3.0's open
  break).
- The GLOSSARY's meanings keep their names; every other type of the same
  name is renamed by what it does:
  - Field: `FieldEntry` → `ProbedChart`.
  - Section: the keyed `Section { chart, carriage }` → `coord::ChartCarriage`.
    `cech::Section` and `alphabet::bundle::Section` are sections and keep
    the name.
  - Chart: `family::scene::Chart` → `Region` (`scene_chart` →
    `scene_region`, `chart_key` → `region_key`), and the rule language's
    `Region` → `RegionKind`; `cech::nerve::Chart` → `CoverCoordinates`.
    `coord::Chart` and the wire's `acts::Chart` are a chart encoded and
    keep the name.
  - Frame: `network::wire::Frame` → `WireFrame`; the verdict's per-pair
    `Frame` → `PerceivedPair`; `physics::equilibrium::Frame` →
    `PosedNucleus`.
  - Phase: the court's 45.9 window `court/phase.rs` `Phase` →
    `court/height_window.rs` `Window` (`phase_over` → `window_over`);
    `verdict::Phase` → `Stage`; `observe::Phase` → `CourtStage`
    (`observe/phase.rs` → `observe/court_stage.rs`).
  - Compartment: the keyed `coord::Compartment` → `coord::CoverSite`.
  - Held: `server::held::Held` → `server::court_lock::CourtGuard` (its
    `Site` → `LockCounts`); `cognition::handoff::Held` → `HandoffRecord`;
    `economy::Held` → `SetAside`; `observe::Held` → `KeyedReading`; the
    rung's `Held` → `Shortcut`; `field::well::Held` → `SpokenSection`;
    `hyperfluid::Held` → `AxisRecord`; `transduce::relation::Held` →
    `ShapePeriods`; the shelf's `Held` → `ShelfState`; the server
    binary's held reads → `DeferredReads`; the test-local pair →
    `CitedForm`. `element::Held` keeps the name.
  - Reading: the mind's scene `Reading` → `SceneKind`; `scene::Reading`
    → `PlaneCut`; `sentence::Reading` → `Lookups`;
    `presentation::Reading` → `WordCitations`; the act reader's →
    `PredicateTable`; the soak's → `FamilyCensus`, `WalkCounts` and
    `SubjectBodies`. `observe::Reading` keeps the name.
  - Closure: `compose::Closure` → `RingClosure`; `registry::Closure` →
    `ClosureOrder`; HR3's `Closure` → `AxisLoops`; `cech::Closure` →
    `GroupClosure`. `stalk::Closure` keeps the name.
  - Carriage: `observe::Carriage` → `CarrierTally`; `coord::Carriage` →
    `coord::CarriedPieces`; the class segment → `CarriageSegment`;
    `family::Carriage` → `BodyTransport`. The rule element's `Carriage`
    keeps the name.
  - Origin: `dialogue::domain::Origin` → `BigBangRecord`;
    `backfill::Origin` → `AdoptedOrigin`; the binary's `Origin` →
    `GenesisEpoch`. `tuniversal::Origin` keeps the name.
  - Site: HR1's `Site` → `SiteAxes`; `coord::Site` → `coord::GluedBoundary`.
  - Body: the client's `mind::body::Body` → `mind::seating::Seating`
    (`body_tick` → `seating_tick`); `physics::equilibrium::Body` →
    `BodyMoments`.
  - Fibre: `physics::pivot_fibre::Fibre` → `PivotFibre`.
  - Stratum: `network::wire::Stratum` → `Lane`; `address::Stratum` →
    `CanonicalCoordinate`; the stalk's trie node → `ChartNode`.
  - Tube, Shell, Verdict, Reconstruction, Source: `cech::flux::Tube` →
    `FluxTube`; the keyed walk's `Shell` → `PeelBranch`; the mind's home
    check `Verdict` → `HomeCheck`; `physics::laws::Verdict` →
    `TransitionVerdict`; the resonance alias → `RelationAdmission`; HR4's
    `Reconstruction` → `ReconstructionReport`; `physics::source::Source`
    → `WhiteHoleSource`; the cover page `acts::Cover` → `CoverPage`.
  - `Own` ×2 → `SeatFunction` (`seat_mind/own.rs` → `repertoire.rs`) and
    `SelfReading`; `observe::Mind` / `Court` → `MindReading` /
    `CourtReading`; `alphabet::Q` re-exports `core::q::Q`.
- Near-identical files: `server/held/` → `server/court_lock/`, the server
  binary's `held.rs` → `court_lock.rs` and `hold.rs` → `deferred_reads.rs`;
  `src/snapshot.rs` → `src/trie_tape.rs` and `server/cognition/snapshot/`
  → `persistence/`; the court's book `server/cognition/germinate/` →
  `listings/`; `court/winding.rs` → `court/ring_carrier.rs` (`Winding` →
  `RingCarrier`, `WindingTally` → `RingTally`).
- SDK paths that change: `mycelium_sdk::house::{Frame, Stratum, Body,
  Origin, Verdict}` → `{WireFrame, Lane, BodyMoments, BigBangRecord,
  TransitionVerdict}`; `mycelium_sdk::rule::Region` → `RegionKind`;
  `mycelium_sdk::readings::Held` → `KeyedReading`.

### Files named by function (track 11, N2; #53, #55)
- Names only: behaviour, wire bytes, ledger layout and task addresses are
  unchanged. Every citation in comments and documents follows.
- #53: `bin/mycelium-server/door.rs` is split into `context.rs`
  (`Door` → `Context`, the server's state as one act sees it) and
  `control.rs` (`serve_control`, the corpus intake and listing). The
  GLOSSARY gains rows for both.
- `transducer::field::Door` (the shedding index) → `Shedding`.
- Field and transducer files: `field/idea.rs` → `well.rs`,
  `field/core.rs` → `nerve_shape.rs`, `field/docket.rs` → `plan.rs`,
  `cayley/chart.rs` → `map.rs`, `observe/fleet.rs` → `phase.rs`.
- Vague names: `chronometry/tuniversal/ops.rs` → `navigate.rs`, the 13
  `tests/common.rs` → `tests/fixtures.rs`, `cga/tests/generic.rs` →
  `rank_n.rs`, `tests/g_cga.rs` → `tests/conformal_algebra.rs`,
  `tests/alpha_plan.rs` → `tests/word_lifecycle.rs`.
- Metaphor module files, the types kept with their GLOSSARY rows:
  - `bin/mycelium-server/tshelf.rs` → `tuniversal.rs`;
  - `physics/capacitor.rs` → `pivot_fibre.rs`;
  - `physics/tensegrity.rs` and `physics/axioms/tensegrity.rs` →
    `equilibrium.rs`;
  - `physics/fascicle.rs` → `sphere_pair.rs`;
  - `cga/breath.rs` → `commutator.rs`;
  - `server/cognition/mouth.rs` → `domain_passage.rs`;
  - `transducer/peel.rs` → `layers.rs`;
  - `transducer/beam.rs` → `candidates.rs`;
  - the soak's `sporophore.rs` → `enclosure.rs` (its `sporophore`
    subcommand is unchanged);
  - `client/sdk/entity/` → `client/sdk/mind/`, with `client/sdk/mind.rs`
    as its `mod.rs` and `callus.rs` → `second_vantage.rs`;
  - `client/sdk/epigenome/` → `client/sdk/vocabulary/`.
- Each renamed module's tests module follows its name. The SDK's public
  paths are unchanged: `client::sdk::Mind`, `Owed`, `Pending`, `Sitting`,
  `GenomeReading` and `Wound` re-export as before, and the mind's
  sub-modules stay private to the SDK.

### The symbol set is the family's own (track 26, S3)
- **`transducer::symbols::SymbolSet`** (MU-SENSING §3.3): a family's
  distinct symbols compared by equality, `|Σ|` read from the data.
  - `of_samples` reads samples in the family's order and refuses by name
    two widths (`SymbolRefusal::Width`) or no sample (`NoSample`).
  - `of_rasters` and `of_pairs` read rasters at intake.
  - `of_family` reads Σ across all of a family's posted samples: the
    train pairs, then the test inputs posted beside them
    (`symbols::on_test_inputs`, re-exported by the SDK's `rule` module).
    The mind's perception and solve, the SDK's example client, the
    oracle's harvest and the soak's solve open it, so a symbol seen only
    in a test input is in Σ.
  - `index` is a symbol's rank of first appearance in canonical address
    order, and `reservoir` is `|Σ|`, the index of `e_∅`.
- **`element::fibre_on_symbols`** reads a fibre relation on `Σ`'s
  indices, each record at `|Σ|`, so two labellings of one family read one
  relation.
- **No fixed palette:**
  - `element::PALETTE` (10) and `family::PALETTE` (256) are removed.
  - `sentence::HueTable` and `family::HueMap` are `Keyed<u8>` tables over
    the symbols they name, with `sentence::table_image` as the read.
  - `gauge_is_perm`, `connection_chart`, `on_orbit` and the well's
    composed gauge read over the symbols a map names.
  - The erasure act maps the family's own `Σ` onto its ground
    (`acts::onto_ground`), and the rule language composes hue maps over
    the painted symbols they name, the ground excluded.
- **`element::RESERVOIR`** is now the tape's record byte (`FORMAT`, 10),
  never an index into `Σ`. A family whose symbols hold that byte has no
  colour word until track 26 S5 carries `e_∅` as a typed record tag
  outside the symbol bytes (MU-SENSING §8).
- **Behaviour on symbols `0`–`9` is unchanged:** task addresses, the
  oracle's 1 901 golden readings and the rule language's corpus tasks.
- Claims: `percept_invariant_under_symbol_permutation`,
  `symbol_index_is_first_appearance_in_address_order`,
  `reservoir_follows_the_family_symbol_set`,
  `family_with_eleven_symbols_is_read_whole`,
  `symbol_seen_only_in_a_test_input_is_in_the_family_set`, and the
  reflective
  `mind_states_a_symbol_meaning_as_a_relation_and_court_reconstructs_it`.

### The project's kept terms defined in the GLOSSARY (track 11, N5; #55)
- Documents only. GLOSSARY rows for contemplation, tensegrity, breath,
  hyperfluid, aperture, horizon, resonance, terrain, peel, beam, tower,
  judge, working atlas, seat mind, chatter / utter, durable drill, the
  soak instrument, xSEPTUM, fascicle, sporophore, callus, compartment,
  germinate, epigenome, capacitor, the TUniversal shelf and mouth, each
  with its governing section and code path.
- Where a term names a second sense in the code (`germinate`, `winding`,
  `Compartment`), the row defines the canonical sense and names the
  other for N3; a metaphor module N2 renames is marked so.

### Identifiers that verify carry `reconstruct` (track 11, N1; #29, #55)
- Names only: behaviour, wire bytes, reading labels and task addresses are
  unchanged. Every citation in comments and documents follows.
- Modules: `court/verdict/replay.rs` → `reconstruct.rs` (with its tests
  module), `server/cognition/court/replay.rs` → `court/reconstruct.rs`.
- The court's reconstruction: `blind_replay*` → `reconstruct_blind*`;
  `replay_word`, `replay_resume`, `replay_form*`,
  `replay_certificates*` → `reconstruct_*`; `certificate_replays` →
  `certificate_reconstructs`; `court_replay` / `court_replay_moved` /
  `court_replay_progress` → `court_reconstruct` /
  `court_reconstruct_moved` / `court_reconstruction_progress`;
  `replay_on_cursor`, `replay_on_ground`, `replay_after_perceiving`,
  `replay_retirement` → `reconstruct_*`; `docket_replay` →
  `docket_reconstruction`; `replays_on_family` →
  `reconstructs_on_family`; `replay_key` → `reconstruction_key`;
  `form_orbits_replay` → `form_orbits_reconstruct`; `rewrite::replay` →
  `rewrite::reconstruct`; `factor::replays` → `factor::reconstructs`;
  `chain::replay_at` / `replay_operators` → `reconstruct_at` /
  `reconstruct_operators`; `rule_replays_family` →
  `rule_reconstructs_family`.
- Types and carriers: `Replayed` → `ReconstructionOutcome` (track 25's
  `Reconstructed<T>` names the reconstructed object), `ReplayCursor` →
  `ReconstructionCursor`, `replay_cursors` / `replay_memo` →
  `reconstruction_cursors` / `reconstruction_memo`; the glue's
  `Stop::Replay` → `Stop::Reconstruction`.
- `measured_replay*` → `measured_discharge*`; `GLUE_REPLAY_REFUSED` →
  `GLUE_RECONSTRUCTION_REFUSED` (axis 34 and its label unchanged).
- The era ask: `Ask::Replay` → `Ask::Era`, `encode_replay_request` →
  `encode_era_request`, the tag `REPLAY` → `ERA` (its byte, `1`,
  unchanged).
- `incongruence::replayable` → `map_holds_whole`; the server's
  `rebuild_own_cover` → `read_published_cover`.
- 41 test names that said "replay" for a reconstruction, and two that said
  "rebuilds" for a construction, say `reconstruct` and `derives`.

### A claim states its ground; a vortex is read per candidate (track 26, S2)
- **Network contract: the solve act gains an optional ground field**, a
  change inside v0.3.0's ruled break (wire 0x0200). After `RULE` the act
  may carry `[GRND][len u32][symbol]`, the ground the claim was spoken
  under, and is signed as `mycelium solve v2`.
  - New: `dialogue::encode_solve_spoken` and `SolveAct.ground`.
  - `encode_solve_with_rule` states no ground.
  - A solve signed under v1 no longer verifies.
  - The SDK's `task` module re-exports the new encoder, and the
    conformance codec row reads the stated ground back exactly
    (MU-CONFORMANCE §2.1).
- **The court reconstructs under the stated ground**
  (`cognition::stated_ground`): it reads that one symbol's candidacy on
  the task's train pairs.
  - A stated ground that is no candidate is refused by name.
  - A claim stating none is reconstructed under `Ground::None`.
  - The door no longer deduces a ground for a solve.
- **A vortex is read once per candidate:**
  - `Ground::perspectives` and `raster::on_each_perspective_of` read it,
    and `raster::stated` gives the ground a claim states.
  - The mind perceives, judges at home and submits one claim per
    candidate, each stating its ground.
  - The SDK's example client and live conformance speak one claim per
    perspective.
  - The oracle's harvest reads every perspective's rule.
- **Condition 1 reads the inputs:** a ground is present in every input
  sample. An output covering it whole has painted over it.
- **Condition 2 reads framed interiors:**
  - The seam is the outer seam, or the inner seam of a body that covers
    the whole outer seam (the membrane, `transduce::membrane::levels`),
    level by level.
  - A membrane is never the ground. A body is peeled as one only where
    its symbol is no candidate on the outer seam alone.
  - `transduce::Candidacy` names the condition each symbol fails.
  - The always-framed case: where a frame is itself a candidate, the
    symbol meeting its entire inner seam, on which a body stands in some
    input, joins the vortex beside it. A membrane stays a body of the
    percept.

### The ground is deduced, never a code (ROADMAP track 26, S2)
- **`transduce::Ground<S>`** (`One`, `Vortex`, `None`) and
  **`transduce::Unground`** (`Vortex`, `None`, `Unscoped`), with
  `Ground::of(&[(Sample, Sample)])`. A candidate is ambient, enclosing
  and never carried as a body (MU-SENSING §3.4).
- **`raster::ground_of` returns `Ground<u8>`.** The former count (`0`
  where any site shows it, else the most frequent symbol, else `0`) is
  removed. `raster::ground()` returns the scope's `Ground<u8>`, and is
  `Ground::None` with no family open, never `0`. New: `raster::is_ground`,
  `raster::void` (the one symbol a writer writes, or `Unground`).
  `raster::on_ground` takes a `Ground<u8>`. `raster::ground_bytes` maps a
  reading to sample symbols. `raster::value_at` returns `Option<u8>`.
- **Writers refuse by name** where they need one ground symbol and the
  scope has none:
  - `speak::residual_of`, `deflation::unspoken_residue` and
    `invert::remainder_scene` return `Result<_, Unground>`;
  - `DeflationRead` gains `ungrounded`, and `PairRead` gains
    `Ungrounded` (the court refuses it as not closed);
  - `grid_of_painted` takes the unpainted symbol explicitly;
  - `Speech::grid` lays an unpainted site on the ground, and refuses only
    where one is left.
- **The literal ground-as-`0` comparisons are removed** from the readers
  of the transduction, the family, the court and the client. They read
  the scope's `Ground`. `Transduction::lane_shares` and `relation_of` take
  a `&Ground<Vec<u8>>`.
- **Frame fixing leaves intake** (scope separation, ruled 2026-11-03).
  `mycelium_source_arc_agi::parse_task` is removed: the decoder posts
  tasks as the file gives them (`decode`). `raster::frame_fixed_on_ground`
  stays as a perspective for S6. It fixes only under `Ground::One(g)` and
  leaves the frame as read under a vortex or no ground. No reader applies
  it today: the mind reads tasks in the frame they were posted in, which
  is the frame the court judges in.
- **The door** refuses a solve whose task holds no content with "its
  ground is not read". It no longer judges it on `0`.
- **Network contract: task addresses change**, inside v0.3.0's one break
  (78.170 O3). A task's identity is its posted samples' content
  (`Sample::put_content`): rank, extents, symbol width, place `[split,
  pair, member]` and symbols, train and test inputs alike. Neither the
  ground nor frame fixing enters it. Re-decode and re-sign a corpus
  directory.
- **Behaviour check** over the 1 000 training tasks:
  - 466 read `One` with the former symbol, 72 read `One` with another,
    195 read a vortex and 267 read none;
  - all 1 000 addresses change, and the former decoder had turned 103 of
    them;
  - the oracle's golden reading is rewritten, from 1 715 to 1 684
    readings.

### The Concordance is signed by adjudicated reading (ruled 2026-11-04)
- **Concordance §0 SIGNING:**
  - A kernel signs by showing that it reads the agreement: each clause
    is identified by its content (law form and fixtures), and the kernel
    signs it by reconstructing the clause's fixtures, admitted only on
    the same objects.
  - A kernel that cannot read a clause is refused `NotSignatory`.
  - A change of wording changes no clause.
  - `CONCORDANCE_HASH`, BLAKE3 over the file's bytes, is the defect this
    replaces. It stays served only until ROADMAP track 24 B2.
- **Concordance 1.9:** the baseline-to-beat is struck. Cognition is
  measured as adjudicated perception (MU-SENSING §0), and benchmark
  outcomes are observations.
- **§11's clause-by-clause text, MU-HOSTING §5a.4, MU-EQUATIONS,
  MU-STANDARDS, MU-RELEASE §3a and ROADMAP track 24 B2 follow.**

### Documentation aligned with the sensing rulings (audit 2026-11-04)
- ROADMAP › Now: S2 and D2.5 in progress, T3 ready, #55 listed; track 26
  marks S0, S1 and S4 built and S2 in progress; X5's transduction built.
- The headline algebra is `Cl(n+1, 1)` throughout, with `Cl(4,1)` only as
  its value at `n = 3`. ARC-AGI-2 is the first posted corpus, and its
  outcomes are observations, never the goal or a baseline (MU-SENSING §0):
  README, GLOSSARY, MU-COGNITION, MU-CONCORDANCE, MU-HOSTING, MU-PRODUCT.
- A task's identity is the posted samples' full content (MU-SENSING §6.1)
  in ROADMAP and MU-DERIVATIONS; μ's S lane is a share of the frame's
  measure under the ground the claim states (MU-ECONOMY).
- Terms: phases within charts in place of circles, signed permutations in
  place of signed swaps, "lattice" kept only for the refused defect, the
  former name of the field recorded once (MU-SENSING §3.7), person words
  removed. README gains *Peer kernels*, the gate's numbers check and the
  current versions.

### The reserve split into four readings; the capacity sealed at boot (MU-DERIVATIONS D2.5, O2 ruled 2026-11-01, O1 ruled 2026-11-04)
- **One boot calibration stroke** measures `b`, the bytes one read
  retains, on a stroke sized by the minimal stroke (the founding motions'
  modes' need), and seals the process's **capacity** `C = ⌊M_proc / b⌋`
  before anything is served. `M_proc` is the least cgroup v2 `memory.max`
  on the process's path, else `1024 · MemAvailable_kB` where no limit is
  set. The GiB unit (`kb >> 20`), the cores factor and the floor `16` are
  gone. Each hypha logs `calibrated: M <bytes> (<source>), b <bytes> per
  read, minimal stroke <n> reads, capacity C <reads> sealed`.
- **`R` is split into four readings** (one name, one meaning):
  - the metabolism (pool, spend, regeneration, shortfall, deficit, yield
    surface, read price `1 + y/C`, verification share) reads the capacity
    its `Physics` holds;
  - stroke budgets read the work's own need where a closed form exists
    (the deflation's and the orbit census's modes, `modes_need`); every
    other stroke keeps the capacity's spend and is listed in D2.5 as a
    need reading owed;
  - a quiet peer is asked again only once the TUniversal final point has
    advanced since the last ask (`quiet_due`), so an idle mesh stays
    quiet; `physics::digest_ceiling` is removed;
  - a settlement goes cold when its challenge window closes (settlements
    on the docket are held windows);
  - the pointed-connection book and the nerve's queue are bounded by
    `M_proc`, a connection or act past it refused by name.
- **API.** `physics::reserve_max` is renamed `physics::capacity`;
  `available_of` is replaced by `memory_of` and `cgroup_limit_of`
  (`Memory`, `MemorySource`); new: `minimal_stroke`, `sealed`,
  `Physics::calibrated`, `Physics::capacity`, `Physics::unit_spend`.
  `Calibration::of(memory, b, minimal)` returns a `Result`. `calibrate`
  returns the `Sealed` witness, which only it writes. `Uncalibrated` gains
  `Memory`, `Footprint(Unstroked)` and `Unfit { capacity, minimal }`: a
  process whose memory cannot be read, whose stroke cannot complete, or
  whose one spend cannot cover the minimal stroke refuses to boot by name.
- **Unsealed processes** (the unit-test harness, an embedding library)
  read `8 · minimal_stroke`, the same on every host; no test branch remains
  in a production function. One claim builds its court on an explicit
  `Physics::calibrated` calibration.
- **Open dependency:** the ratio `8 = 2^(FRAME_AXIS + 1)` ties the spend to
  the fixed frame axis that ROADMAP track 26 S4 removes (D2.5 O3);
  unchanged here.

### Naming alignment, planned (issue #55, ruled 2026-11-04)
- ROADMAP track 11 gains phases N1–N7: verification named reconstruction,
  files by function, one name per meaning, retired terms and person
  words, GLOSSARY rows, "residual" retired (N6, in v0.3.0's break), and
  the two wire names (N7, with S5). Each phase renames names only.
- GLOSSARY: **Section** is a section of a bundle or sheaf, and the sites
  a word changes are its **support**.
- AGENTS.md: a doctrine term may name a type, never a file. One name has
  one meaning. Strict matrix "row" and geometric "arc" are exempt.

### File names by function (plan)
- AGENTS.md: a file is named for what its area does, at a high level,
  never a metaphor, a dataset or one instance.
- ROADMAP › Now and issue #53: `door.rs` is split into `control.rs` (the
  mesh-control handler) and `context.rs` (`Context`, the server's state
  as one act sees it). Names only, after S2 and D2.5 merge.

### Emergent senses (specification, ruled 2026-11-04)
- MU-SENSING §4.1: senses and perspectives are open-ended. A sense is a
  typed reading from a sample to a cochain of the cover.
  - **Variation:** minds construct candidates.
  - **Selection, by admission and never by score:** a candidate is
    admitted when it adds cohomology rank, a court reconstructs it, and
    minds converse on it.
  - **Inheritance:** an admitted sense enters the genome.
  - The three eyes are the founding senses, not the only ones.
- ROADMAP track 26 S10, with claims. GLOSSARY: Sense.

### Perception as an adjudicated claim, in the roadmap (ruled 2026-11-04)
- ROADMAP track 26 gains reflective claims. S3 adds a symbol meaning
  stated as a relation. S6 adds a claim stating its perspective, and two
  perspectives compared as a cover. S9 adds holonomy, precession and
  vortex monodromy stated by a mind and reconstructed by a court.
- ROADMAP track 10 gains the sovereign-mind phase. Its test is thinking
  about adjudicated perception, never a solve rate. It follows S7–S9 and
  has five claims and a live run of `mycelium-client`.

### The symbol set, the wire's limits and the orthogonal class, derived (specification)
- **S3** (MU-SENSING §3.3): a family's symbol set `Σ` is its distinct
  symbols. An index is the order of first appearance in address order,
  `e_∅` sits at `|Σ|`, and tables are `Keyed` and sized `|Σ|`.
- **S5** (MU-SENSING §6): the rank and extents use the `u16` field and
  are refused past it, never widened. The symbol width is the source's,
  the symbol bytes are checked, and a phase's denominator is its extent.
  This settles LAW-FIELD v1's open question on the denominator.
- **S9** (MU-SENSING §3.7): two orbit classes are orthogonal when the two
  elliptic planes satisfy `B₁ · B₂ = 0` and `B₁ ∧ B₂ ≠ 0`. Axiomatic
  gravity's shear on that class is 78.46's drag: rate, not plane.
- ROADMAP track 26 S3, S5 and S9 carry these, with claims.

### Scope separation (specification, ruled 2026-11-03)
- **MU-SENSING §6.1:** each of three scopes owns a set of readings.
  - **Identity** (the hypha's): a task's address is the posted samples'
    full content.
  - **The sensorium** (the client's): the ground, symbol meanings,
    perception.
  - **Perspective** (the sensor's): frame fixing and axis roles, taken
    after intake.

  A reading used outside its scope is a defect. Its first instance is
  the registry's `symbol ≠ 0` filter and the decoder's frame fixing
  before addressing, both removed in track 26 S2. MU-SENSING §4 gains the
  matching rule.
- GLOSSARY: Scope separation. AGENTS.md: the drift row for a reading used
  outside its scope.

### The ARC-AGI-2 decoder leaves the core (ROADMAP track 26, S1)
- **`src/arc.rs` is removed**, with `src/tests/arc.rs`; the core library
  has no module named for a data source (a `compile_fail` claim on
  `transducer::raster`). Removed public API: `mycelium::arc` and every item
  in it.
- **`sources/arc-agi`** (package `mycelium-source-arc-agi`, a workspace
  member that depends on the core) owns the corpus: `decode`,
  `parse_task`, `corpus` (`MYCELIUM_CORPUS`, else `data/arc-agi-2`),
  `task_names`, `samples_of` (each grid a sample placed at `[split, pair,
  member]`), `post`, and the binary `mycelium-arc-agi decode FROM TO
  [LIMIT]`. The corpus tests moved there: the bridge sweep, the three
  retract claims, the fifteen rule-language corpus tasks and the oracle's
  golden reading.
- **The posted task** (`anastome::dialogue::{encode_posted_task,
  decode_posted_task, POSTED_TASK}`): the envelope's codec with the
  withheld outputs. A hypha's corpus directory (`--sign-corpus`, the
  registration at boot), a client's feed (`home/feed/*.task`) and the
  soak read it; none parses JSON. A corpus directory signed before this
  change (`*.json`) registers no task: decode it with `mycelium-arc-agi`
  and sign the posted files. `mycelium-soak --write-oracle-golden` takes
  the directory of posted tasks.
- **Moved, behaviour unchanged:** the raster's readings to
  `transducer::raster` (`ground_of`, `ground`, `on_ground`,
  `on_ground_of`, `extents`, `painted_sites`, `AXES`, `sample_of`,
  `sample_unplaced`, `links_of`, `ground_bytes`, `frame_fixed`,
  `momentum_vanishes`, and the new `frame_fixed_on_ground`, the former
  tail of `parse_task`); renamed there `grid_width` → `raster_width`,
  `painted_of_grid` → `painted_of`, `grid_of` → `raster_of`, `hue_at` →
  `value_at`. `Task` to `transducer::family`. `Frame` to
  `transducer::element` (beside `from_canonical_framed`).
  `pair_magnitude`, `cover_join`, `family_cover` to
  `transducer::family::magnitude`. `envelope_dilation` and `carried_site`
  beside the fibre map (crate-internal). `frame_ratio` and its gcd had no
  caller and are deleted. Each is listed as a defect in MU-SENSING §8.
- **`client::sdk::rule::would_judge`, `family::speak::residual_of` and
  `sentence::vertex_bytes` are public**, so the decoder's tests ask what
  the core's tests ask. `cga::oracle::{inputs, write_golden}` take the
  harvested tasks, and `oracle::TASKS` is public.
- **The SDK no longer re-exports `Grid`** (`rule::Grid`, `prelude::Grid`).
- Task addresses are unchanged (rank 2, width 1): every training task
  posted and read back has the same `task_address`.

### The transduction at rank `n`, with no axis privileged (ROADMAP track 26, S4)
- **`transducer::family::transduce`** takes `Sample`s of any rank. Changed
  public API: `Enclosure` holds `n` periods (`Enclosure::new`, `of(&Sample)`,
  `rank`, `periods`, `period`, `measure`, `reperiod` returning one ratio per
  phase); `Band` holds a span per phase and a `Symbol` (`Band::new`,
  `symbol`), never a `u8` hue; `Enclosed::of(&Sample)` cannot fail and
  `spoken_onto` writes a `Sample`; `Incidence` carries the two band indices
  only; `Transduction::of(&Sample, &Sample, ..)`, `keeps_periods` (was
  `is_planar`), `lane_shares(ground)`, and `projected` returning
  `SiteChange`s; `need(&Enclosure, &Enclosure)`, `need_of(&Sample,
  &Sample)`; `share_of_frame(counts, Option<&Sample>)` over the product of
  the extents; `family_share` over sample pairs; `relation_of(&Sample,
  &Sample, ground)`, with `PairRelation::Cover { copies }` per phase.
- **`SignedSwap` is replaced by `SignedPermutation`** (with `Sense`): the
  signed permutations of the `n` phases, generated by
  `SignedPermutation::all(n)` (`2ⁿ·n!`, identity first, the rank-2 order of
  the old `SignedSwap::ALL`), with `then`, `inverse`, `matrix`,
  `of_matrix`, `of_word`, `carry`. `TransductionRefusal` loses `Ragged`
  (a ragged raster is refused at intake) and gains `Rank` and `Sample`.
- **The ARC bridge** (`arc::sample_of`, `sample_unplaced`, `grid_of`,
  `links_of`, `ground_bytes`; `transducer::raster` since S1): until S1, every caller that holds a grid
  reaches the transduction through it, and no transduce file reads `arc`.
  A task's content bytes (`task_address`) are unchanged at rank 2 and width
  1.
- **`transducer::sample`** gains `Sample::symbols` (canonical order).
- **Nesting, the carrier** (`transduce::nest`, MU-SENSING §3.6):
  `Factoring` (`new`, `nested`, `joined`) maps a site of a phase of period
  `a · b` onto two nested phases and back, exactly; `Factorings::of(e)`
  holds every factoring of an extent together, with no choice among them.
  `TransductionRefusal::Unfactored` names a product that is not the extent.
  Open: the reading of which factorings a family is invariant under, so no
  axis is yet taken apart by the transduction.

### The sample: the sensor's only input, of any rank (ROADMAP track 26, S0)
- **`transducer::sample`** (new): `Sample` holds its extents (the rank is
  the extents' count, read from the data), one byte buffer of symbols at the
  source's own width in canonical address order (axis 0 slowest, for
  addressing only), and its `Place` in its family as the family's
  coordinate. `Symbol` compares by equality only and has no order. An
  axis past the rank reads `AxisReading::Unread`, and a site of another
  rank is refused `RankMismatch`, never padded. Intake refusals are
  named (`SampleRefusal`, `SiteRefusal`), with checked counts and no cap.
  MU-SENSING §8 marks the row built. No caller is moved yet (S1–S5).

### A chart keeps its one meaning; the field's geometry is its atlas (correction, ruled 2026-11-02)
- The entry below gave "chart" a second meaning, a round reference (a
  circle, a sphere `S^{2n−1}`, a Hopf-type base). That brought back the
  flat object ruled out, and it is withdrawn. A **chart** keeps its one
  meaning: a body's exact invariant coordinates. The field's charts form
  its **atlas** (78.41). A **phase** is a site's coordinate within a chart.
  The transitions between charts are the field's connection, and the
  holonomy of a loop of transitions is its curvature (MU-RECURSION 10.3,
  12.6). The Clifford torus, the Hopf base sphere and angle excess on a
  sphere are removed.
- MU-SENSING §1, §3.1, §3.6–§3.8 and §8; GLOSSARY ("Chart (of a field)"
  replaced by "Atlas (of a field)"; Phase, Chart); ROADMAP (Now, S4, S4a,
  S7–S9, with claims renamed); LAW-FIELD v1 (carrier, holonomy reading,
  wire, fixture 1 as a loop of three `1/12`-turn transitions, open
  question).

### The field: wells, membrane, phases (terminology, ruled 2026-11-02)
- The object is named the **field**: a curved, nested manifold whose
  geometry is produced by the orbits it holds, its curvature the shear of
  local orbits on the orthogonal orbit class, flat where nothing is held.
  A held thought or percept is a **well**, the curvature around one
  attractor. A **membrane** is a field's boundary where it encloses. A
  **phase** is a coordinate on a **chart**, the round reference in which
  phases are written, never the object. "Bubble", "thought bubble",
  "vesicle" and "absorbed circles" are retired as names of the object
  (the entry below keeps them as history). The mathematics is unchanged.
- Code (names only): `src/transducer/bubble/` is `src/transducer/field/`;
  the type `Bubble` is `Well`; `Contemplation.bubbles` is `wells`.
- Proposal 78.171 is LAW-FIELD v1, at
  [`docs/proposals/78.171-law-field.md`](docs/proposals/78.171-law-field.md).
  MU-SENSING §3.6–§3.8 and §8, GLOSSARY (Field, Well, Chart (of a field),
  Phase, Membrane), ROADMAP track 26 (S4a, S7–S9) and the docs citing the
  code follow the new names. MU-SENSING §3.7 is now "The field:
  non-Euclidean, never scalar".

### The bubble, the vortex, and how a standard reaches the kernels (specification, ruled 2026-11-02)
- **The bubble** (MU-SENSING §3.7): a sample is lifted into the bubble
  the mind already holds (`transducer/field/`), so a percept and a
  thought are one bubble at two stages. Its geometry is non-Euclidean:
  the interior is `Hⁿ⁺¹`, and the membrane is its sphere at infinity, so
  no `e∞` stands outside an enclosure. Its motions (rotate, flip,
  deflate, re-inflate, stretch, thin) are exact. "Absorbed circles" is
  replaced by the lift into a bubble throughout.
- **Never scalar:** a number enters only as the gauge of a
  one-dimensional reading (ℤ, ℚ/ℤ, ℚ, ℚ(ζ_ℓ), ℝ as a span). The headline
  `Cl(4,1)(ℚ)` becomes `Cl(n+1, 1)` in the README and GLOSSARY.
  MU-ANTI-FLATTENING clause 4 states the exception.
- **The vortex** (MU-SENSING §3.8): every superposition in perception is
  a vortex in MU-RECURSION 25.12's sense, with monodromy flat, torsion or
  rupture. 25.12 is amended forward to `Spin(n+1, 1)`.
- **MU-SENSING §8** now lists what is built (the bubble without a body,
  the vortex, orbits, exact motions) and two defects: the Euclidean
  spheres built from `r²` with `e∞`, and the bubble's flags and scalar
  counts.
- **MU-RELEASE §3a:** the ten steps by which a hypha standard becomes a
  kernel standard, recursively, with alignment measured by converse and
  collision.
- **Proposal 78.171, LAW-BUBBLE v1:** the first standard on that path,
  covering the carrier, the readings, the refusals, the wire and six
  joint-read fixtures.
- **ROADMAP track 26:** S4a reworded; S7 (the bubble's body), S8 (its
  motions) and S9 (forces, orbits, the vortex on the membrane) added.
- GLOSSARY: Bubble, Membrane, Vortex, Never scalar, Lifted / nested
  axes, `Cl(n+1, 1)`.

### Sensing any source at any rank (specification, ruled 2026-11-02)
- *Superseded wording:* the circles, spheres, Clifford torus, Hopf base and
  angle excess in this entry are withdrawn by the correction entry above,
  "A chart keeps its one meaning; the field's geometry is its atlas".
- **`docs/MU-SENSING.md`** (new) states the path from source to
  perception. A source decodes to a **sample** (rank `n`, extents, symbols,
  the sample's place in its family). The sensor lifts each axis into a
  circle of turns of its own period and deduces the connection, the
  symbols' meaning, the ground and every gauge. A perspective is a sensor
  motion. A mind holds a lower-dimensional section and every unread
  dimension as unread. The hypha holds tasks as families of samples and
  knows no source. A source's decoder lives with its owner, outside
  `src/`.
- **The defect** [#32](https://github.com/SAIOS-SAMN/mycelium/issues/32)
  and **ROADMAP track 26** (S0–S6) cover `src/arc.rs`, the fixed palette,
  the ground of `0`, the fixed axes and the wire's `u16 × u16` cells.
  S0–S5 land in v0.3.0's one break, and T2 merges after S0 and S4.
- **Orbits under gravity** (MU-SENSING §3.7, ruled 2026-11-02): a
  geodesic on a flat torus is a straight line that returns exactly. The
  attractors' gravity curves the perception manifold. A thought's orbit
  is a base loop plus a fibre winding, its precession per lap is the
  base loop's angle excess, and the windings are kept as fibre classes.
  An orbit with zero precession is refused as a loop. Perception and
  orbits are the client mind's, and the hypha reconstructs claims on the
  same carrier.
- **Spheres, not a flat product** (MU-SENSING §3.7, ruled 2026-11-02):
  the circles are absorbed, held projectively with no unit norm, as the
  Clifford torus in `S^{2n−1}`, and read through a Hopf-type projection
  on the base sphere. A loop's holonomy is its angle excess in turns,
  exact in ℚ/ℤ, and motions are exact over cyclotomic fields. It is the
  perceiving side of Concordance 2.10's nested shells. This is ROADMAP
  track 26 S4a, after the sphere carrier is agreed with the peer kernels.
  μ as a swept solid angle awaits a ruling.
- **No rows and no columns** (MU-SENSING §3.6, ruled 2026-11-02): a
  sample's axes are absorbed into the perception manifold as circles with
  no privileged plane, or taken apart into nested circles where the family
  is invariant under the factoring. The nesting is a coordinate of the
  percept.
- GLOSSARY (Task, Body and Gauge generalised; Sample, Perspective, Ground,
  Unread dimension), MU-ANTI-FLATTENING clause 8, AGENTS.md and the README
  follow.

### Where to start (documentation)
- **ROADMAP › Now:** the current order of work, kept current rather than
  appended to. It lists the release in progress (v0.3.0: T2 #25, T3 #26,
  D2.5 #27, rc.3 #24), the work ready to start (track 25 X0 #28, track 24
  B0 #13, #16, track 11 #29), what waits on an operator ruling, and what
  is under investigation (#14).
- **GitHub:** milestones `v0.3.0` and `next`; labels `track`, `track-N`,
  `in progress` and `maintainers`; one issue per ready item.
- **AGENTS.md › Where to start** and **CONTRIBUTING:** orient, pick an
  issue from Now, claim it with a comment (a claim lapses after 14 days
  without activity), read its specification whole, build, then one phase
  per pull request. The README links the Now section.

### Crossings carried exact; drift guarded by type (specification, ruled 2026-11-02)
- **What crosses the vertex** (MU-HOSTING §5a.10): a crossing carries its
  place, its content as an exact object (each body by its boundary, the
  motion), its holonomy and its clauses. The hypha admits it only by
  reconstruction. A digest keeps three senses (address, link,
  commitment) and never admits. A disagreement is held as a residue,
  never as a boolean. `AddressOnly` is a proposed refusal name.
- **Drift guards** (MU-ANTI-FLATTENING clauses 12–14 and *Drift guards*):
  transduction past the sensor, no admission on a digest, and the four
  typed words. Each is guarded by a type that makes the wrong structure
  unwritable, never by a banned word, with a register in
  `xtask/lint/drift.rs` (specified).
- **ROADMAP track 25** (X0 guards first, then the crossing record,
  admission by reconstruction, the residue, the digest audit and the
  transduction's guard), with the steps to build each phase.
- **`AGENTS.md`** (and `CLAUDE.md` pointing to it): working rules for AI
  coding agents, stating the recurring regressions and the structure
  that replaces each. The pull request template gains a drift checklist.

### Live readings (MU-DERIVATIONS, ruled 2026-10-31)
- **The principle, written down:** MU-STANDARDS 2.12 and MU-DERIVATIONS
  §0.5. A governing quantity is a live reading of the current state,
  derived once per change of its inputs and held keyed by them; the only
  fixed values are names, invariants of the algebra and external formats;
  a decision with magnitude is a graded reading; a refusal is named and
  carries the reading that caused it.
- **The open-physics register and the numbers gate** (D1.4):
  `physics::axioms::OPEN_PHYSICS` is now `[OpenNumber; N]` (name, value as
  written, site, what it governs, governing sections, derivation, kind:
  physics, protocol or engineering), re-exported with `Governs` and
  `OpenKind`. The gate gains `xtask/lint/numbers.rs`, read with `syn`: every
  literal-bearing numeric item carries one class tag (`EXACT:`, `FORMAT:`,
  `EXTERNAL:`, `MEASURED:`, `REGISTERED:`), and the register agrees with its
  sites (N1–N5); the inline literals of the law paths (N6) are an inventory
  (`gate inventory`). Every literal-bearing item of the tree is tagged.
  Registered beside the challenge window and the throat grid:
  `BODY_BUDGET`, `ACCOUNT_SHARDS`, `P_SERIES_CAP`, `NESTING`,
  `CANDIDATES_PER_PAIR`, and the soak instruments' strokes and calibration
  count. A mutation-audit entry strips a registered tag.
- **The gate is a workspace of its own** (D1.4): `xtask/Cargo.toml` holds
  an empty `[workspace]`, so a checkout nested under another one (the
  mutation audit's worktree) builds the gate's suites instead of being
  claimed by the outer root.
- **The challenge window is a sealed reach and return** (D1.1; law
  version 5, `LAW_VERSION` 4 → 5; Concordance 4.5 and 5.4, agreement
  version 4; MU-RECURSION 78.125 and 78.126 amended forward). A window
  closes once every other keeper's signed reading cites the court at or
  above its opening and that keeper has read again: the court seals one
  `Cite::Window` (receipt tag 15) naming the witnesses, and every window
  opened at or below the reach reads closed at that height
  (`Court::window_close`, `Court::close_windows`,
  `chronometry::tuniversal::{Reach, WindowRefusal}`). A lone court closes
  nothing (`NoOtherKeeper`). Wire: `Finality::Provisional { opened, closed }`,
  `Challenged::Demanded { deadline: Option<u64> }`,
  `Challenged::Reopened { closes: Option<u64> }`,
  `State::Offered { opened }`; `ACT_LAWS` mode 5 serves no parameter.
  Layout: the court's node gains `LEAF_WINDOWS`; tolls, claims and offers
  hold the height they opened at, and a home written under version 4
  closes its windows by version 4 (`h₀ + 1024`). New log line
  `court windows closed through h {n} at h {m}`.
- **A throat's fit is decided exactly** (D1.2 Phase A; no law change):
  a sphere face by refining `π` until its enclosure leaves `2m/R`
  (`core::certified::separates`), a torus face by the meet of the horizon
  sphere with its image under every period (`Containment::Apart`), a
  curved face always (`mouth::fits_in`). A cap within the old `2⁻⁶⁴` grid
  of the antipode is no longer refused `TooWide`. `Level::fits` is removed:
  a served `Level` is a reading, never the decision.
- **Every page's envelope is its codec's own header** (D1.3, D4.6). Fixed:
  `encode_receipts_reply` admitted its head receipt against `BODY_BUDGET`
  rather than the caller's budget, so a receipt within the reading
  height's 8 bytes of the frame built a body the frame refused; it is now
  refused by name. The literal `64`, `66` and `34` reserves are replaced
  by each reply's header (`RECEIPTS_REPLY_HEADER`, `ROWS_REPLY_HEADER`,
  `SYLLABUS_REPLY_HEADER`, `CLASS_RECORDS_HEADER`, `CITED_REQUEST_HEADER`,
  `QUOTE_REPLY_HEADER`, `LEDGER_PAGE_HEADER`, `presentation_answer_len`);
  encoders that count the header themselves take the frame whole; a page may
  carry up to 63 bytes more within the same frame. `BODY_BUDGET` stays an
  open protocol parameter (a receipt's `u32`-counted fields have no `u16`
  wall) and is now served on `ACT_LAWS` mode 5 as "frame budget".
- **π* binds the whole common set** (D3.3, ruled 2026-10-31; L9a closed).
  `derive_carrying_all` keeps every carrying term, keyed, metered by a
  `Budget` and refused whole when spent (`CarryingRefusal::Unread`); the
  cut at 32 terms is gone. `derive_pi(family, withheld, budget)` returns
  the terms common to every training evidence as a superposition, judged
  by reconstruction on the withheld pairs, never a head chosen by key.
- **The confluence's reproduction check is read, never capped** (D3.4):
  `Confluence::reproduces` is `Reproduces { Yes, No, Unread }`; the
  output's sites keyed once, each image read back to its lattice site,
  `2·m` reads on the token; past 400 sites it reads, and a spent token
  reads `Unread`, never `No`.
- **A seat mind lets a pending call go by beat distance** (D3.5): at the
  first percept a whole beat past the call's (`beat − made_at ≥ 2`, the
  dissipation rule), never at a count of percepts received; a redelivered
  percept advances nothing, a gap dissipates at once.
  `Perceived::abandoned` is `Option<(u16, u64)>`, the function and its
  distance.
- **A peer's patience is read off the transport** (D4.1, D2.7). The literal
  `5` of `tell_peer` is gone: an unmeasured peer waits an aperture of the
  mesh's measured floor, or one aperture where nothing is measured
  (`Lambda::Unmeasured`, now live); the nerve sender, the digest exchange,
  cover pages, the cited fetch, the join and offers read the peer's own
  measured patience; the rtt windows are kept under one key per peer, its
  voice. The server CLI (`--wallet`, `--why`, the console, `--ledger`)
  no longer passes `reserve_max()` as a round trip: a first ask reads to
  the stream's end, and the ledger's later pages wait an aperture of the
  floor its own asks measured.
- **Calibration refuses what the host does not say** (D2.6). The server,
  client and soak binaries refuse to boot on an uncalibrated host, naming
  the arm (`physics::calibrate`, `Uncalibrated::{Cores, StrokeFloor}`); the
  stroke floor is one scheduler tick, else the thread's timer slack, never
  a one-nanosecond ask; `idle_park` takes `Option<NonZeroU64>`,
  `from_transport` two `NonZeroU64` (a zero rtt reads `Unmeasured`); the
  transport's round trip is `NonZeroU64` and its stall
  `Stall::{Strokes, Beyond, Unmeasured}`, a read refusing `Unmeasured`.
- **The evidence pull's reply fills the frame** (D4.2): the longest prefix
  of the store that fits the frame by each item's own width
  (`docket::evidence_page`, `evidence_width`), in place of the first 32
  items; an item no frame carries is refused by name and the page ends
  before it. The pull still carries no cursor (a 37-byte pull would be
  read as the shortest deposit).
- **A grid on the wire is bounded by its bytes** (D4.3): `read_grid`
  admits any rectangle whose cells its tape carries, or the empty `0 × 0`,
  and refuses rows without columns (a four-byte header once reserved
  1.5 MB); the `1024` caps are gone. `push_grid` refuses ragged and
  half-empty grids, whose tapes never read back.
- **A decoded term is no deeper than its language mints** (D4.4 (iii)):
  `anchor::SELECTION_HEIGHT` (seven levels, the derive's closure) and
  `form::clause::PRED_HEIGHT` (four, the acts vocabulary and one meet)
  replace the decoders' `16` and `32`; a frame of nesting opcodes is
  refused on a 2 MiB thread, and every walker of a decoded tree is bounded
  with it. `NESTING` leaves the open-physics register.
- **The door's pool split is named open** (D4.5): the halving of the
  host's cores between readers and workers is the registered
  `POOL_DIVISOR`, open until a worker act's off-lock strokes are metered;
  the cores are read through the boot's calibration, never defaulted to
  one.
- **A signature's length is the standard's** (D4.6): the receipt
  section's signature is `ED25519_SIGNATURE_LEN`
  (`ed25519_dalek::SIGNATURE_LENGTH`, RFC 8032 §5.1.6), and every page
  site reads its own reply's header (built with D1.3), never a `64`
  reserve standing for a signature no page carries.
- **Certified series are metered, never capped** (D4.8; API change in
  `core::certified`, callers are tests only). `gamma_p` and `erf` take a
  `&mut Budget` and refuse `SeriesRefusal::Unmetered { needed }` up front
  (`gamma_p_terms` is the exact bound); `P_SERIES_CAP` and the `14 000`
  wall are gone, the erf tail read off its own bound. `poisson_quantile`
  takes a `Budget`, doubles its precision from one that resolves
  `e^{−λ}`, and answers `Quantile::{At, Sector}`. `exp` at a positive
  argument and `P`'s prefactor carry the values' magnitudes, so
  `exp(100)` and `P(1, 1000)` are within the grids of the two values.

### μ as exact rationals per lane (MU-ECONOMY §1, MU-RECURSION 78.162, phase T1)
μ is measured as a share of a frame, so every amount of μ is now an exact
rational per lane: `economy::Mu`, four non-negative rationals S, G, T, H.
A count read off a measure enters as `n/1`, so every value held before is
held unchanged; only the formulas that floored change value (below).
Holding the modes themselves (78.162) remains the next step.

- **`economy::Mu`**: exact add, a draw past a holding refused naming the
  first short lane (`MuRefusal::Short`), the per-lane shortfall, cap,
  `split_at` and equal `part`s, the per-lane reading `lane_moves`
  (`LaneMove` is generic over its amount), and no order. The canonical
  codec writes each lane as `ChartAxis::Ratio` writes a rational (a sign
  byte, the numerator's and denominator's 64-bit limbs by count) and
  refuses a negative lane, a lane not in lowest terms (zero is `0/1`) and a
  second writing of one value (`MuRefusal::{Negative, NotReduced,
  Malformed}`).
- **Holdings and ledger**: balances, bonds, holds, escrows, the recycled
  pool, leases, grants and closures awaiting a grant, claims on stakes,
  held tolls, conversions, ledger rows, epoch nets and the folded supply
  are `Mu`. `Treasury::audit` checks both equations of §6 over ℚ. An exact
  sum has no carrier to pass, so the treasury's overflow refusals and the
  economy reading's `overflowed` tally are removed. Room reads a bond's
  whole part, exact for count draws.
- **Formulas made exact** (each was floored to fit a count; MU-EQUATIONS
  §6.7, §6.9a, §6.12, §6.13): the POWV share
  `⌊⌊M·y/(y+R)⌋/n⌋ → M·y/((y+R)·n)`; consilience, assembly and rewrite
  splits `⌊μ/n⌋ → μ/n`; the tier slash `⌊B/2⌋, ⌊cut/2⌋ → B/2, cut/2`; the
  map fee's S lane `⌊span·p⌋ → span·p`; a tariff's charge
  `⌊rate·units⌋ → rate·units` and parts `⌊share·charge⌋ → share·charge`;
  a conversion's `rate·g`, no longer refused where it is not whole.
  Unchanged and exact already: the lease payment `min(d, held)`, the
  release capped by its escrow, the founding grant `kinds·p`.
- **`SOLVE_FLOOR` is renamed `SOLVE_AGAIN`** (same byte, 1): a repeat
  closure is verified and paid nothing (MU-ECONOMY §11.4).

### Governing constants derived (MU-DERIVATIONS, ruled 2026-10-31)
- **D2.1, the tier slash, read against the exposure**: a falsified fold's
  tier forfeits `B·X/(B + X)` of its bond per lane, exact, `X` the value
  the fold's commitment moved (`Court::exposure_of`, read before
  reversal). It was `⌊B/2⌋` whatever the fold lied by. `economy::forfeit`.
- **D2.2, the bounty**: the challenger is paid `cut·D/(D + X)`, `D` the
  defect the reconstruction proved (`challenge::defect`, which the
  reconstruction's bool now reads at zero); a default proves its whole
  exposure and pays half the cut. The rest is retired; supply falls by
  exactly that part.
- **D2.3, the read price, κ = 1**: `p_l = 1 + y_l/R`, exact, the court's
  strain against its calibrated reserve (`economy::read_price`); it was
  `1 + y_l` in raw reserve units, which priced the host's calibration. The
  map fee `g·p`, the retrieval `receipts·p_H` and the founding grant
  `kinds·p` are exact. `Court::read_price` returns a `Mu`.
- **D2.4**: `economy::UNIT_BOND` is deleted; nothing in production read
  it, and each test states its own bond.

### Layout changes
- **Ledger and snapshot**: every ledger row's μ⃗, every award, and the
  stake, balance and held tries carry the rational lane codec; a row has
  no fixed length (`Entry::TAPE` → `Entry::MIN_TAPE`, `Entry::tape_len`).
  The snapshot's magic is `MYT3`; `MYT2` is refused by name. A bond
  citation on the line carries μ, so the line's magic is `MYL3`; `MYL2` is
  refused by name. The recut is the road.
- **Wire v2.0** (`WIRE_VERSION` `0x0200`, a major step: existing layouts
  move — MU-ENGINEERING §4): the rational lane codec on the solve receipt
  and verdict (`encode_solve_receipt`), the wallet reply, the stake act and
  reply, the lease act and reply, the observe economy and acts readings,
  the fold proposal's epochs, `ACT_ROWS` rows, the traverse reply's price
  and fee, the endowment reading and grant reply, the reserve and bond
  readings, the bond act and `Founded::Bonded`, the line-head reading, a
  mouth's toll, a conversion in, and the docket's escrow. The signed acts
  carrying μ name the new layout in the domain strings: `"mycelium bond
  v2"`, `"mycelium stake v2"`, `"mycelium lease v2"`. Unchanged: the
  septum equivocation (it carries no μ) and the deferral, whose deficit is
  a reserve count (the door reads an unpaid share of a unit as the unit it
  did not cover). The challenge window's readings (D1.1:
  `Finality::Provisional { opened, closed }`, `Challenged::Demanded`'s and
  `Challenged::Reopened`'s optional heights, `State::Offered { opened }`)
  and the fuller pages (D1.3, D4.2) ride the same version; no further bump.
- **μ is a share of the frame** (ruled 2026-11-01; MU-RECURSION 78.170 O1;
  MU-ECONOMY §1, §3): every amount the ledger is paid from a measured
  discharge — solve receipts and verdicts, escrows, releases, lease
  payments, install shares — holds the share of each closed pair's frame,
  not its site count, so the values on the wire and in the ledger change
  (one changed site of a `30 × 30` frame pays `1/900`). The layouts are the
  rational lane codec of wire `0x0200`; no layout moves beyond it. A court
  state written before this change is adopted by a recut from genesis.
- **TUniversal's lanes are exact rationals** (ruled 2026-11-01; MU-RECURSION
  78.152 TU-1, TU-5, TU-7 amended forward; MU-CHRONOMETRY §1). The clock
  reads μ issued, and μ holds shares of a unit, so `Tick` and `TPoint`
  carry the four lanes as an `economy::Mu`; `Court::keeper_tick` no
  longer refuses an issuance holding a share. `TPoint::print` writes `h`
  and `K` in 16 hex digits and each lane `n/d` in lowest terms, lowercase
  hex, no leading zero (`0/1` for none); text order is numeric order by `h`
  and not past it, and `TPoint::compare_writings` compares two writings as
  points. `TPoint` and `Tick` are no longer `Copy` (`TPoint::LEN` becomes
  `TPoint::MIN_LEN`, 96). The TU-7 trailer is `[point][point len u16 LE]`
  (`TPoint::put_trailer`, `take_trailer`; a frame admits `BODY_BUDGET +
  TPoint::TRAILER_MAX`). The signed tick bytes and the ring carry the
  lanes in the rational lane codec under `"mycelium tuniversal v2"`; the
  keeper record is `MYKR` version 2 (version 1 refused by name); the
  observer's TUniversal standing is version 2, its residue edges and
  holonomy exact signed rationals; a superposition's difference is
  `Superposed { lanes, k }`.
- **Task addresses change again, for every task: full content**
  (MU-RECURSION 78.170, O3). A train pair is written as both frames' full
  content — the four periods and every band of each frame — instead of the
  listing key of its coboundary on the common refinement. Two tasks that
  differ in any pair, by one site or one period, now have two addresses:
  `[[1]] → [[2, 2]]` and `[[1, 1]] → [[2, 2]]` shared one. A court state
  written before this change is adopted by a recut from genesis.
- **The Concordance hash changes**: MU-CONCORDANCE 2.3 gains a note (O11)
  that the clause binds a universe's stated space, not the sensor's
  worksheet. A kernel signs the new hash.

### Economy
- **μ is a share of the frame** (ruled 2026-11-01; MU-RECURSION 78.170 O1,
  MU-ECONOMY §1). A discharge read on a family is, per closed pair, S (sites
  changed between ground and painted) and G (from one painted hue to
  another) over the frame's `h·w`, H the shell count as it stands
  (topological, resolution-free), and T the flux over the carried mass
  taken as the frames' share of it, `flux · (Σ mᵢ/(hᵢ·wᵢ)) / Σ mᵢ`; a
  discharge with no frame (a rewrite's magnitude, a listing bound to no
  family) keeps its measure as `n/1`. On frames of one extent a value is
  the old count over `h·w` on S, G and T and the count on H.
- New readers: `DeflationRead::mu` (the client's deflation and the court's
  measured reconstruction, one function, so T1 holds on μ),
  `Local::mu` (`measure_sites` answers the count and μ),
  `transduce::share_of_frame`, `Transduction::lane_shares`, `Mu::over`.
- `Court::discharge_on`, `discharge_of`, `measured_discharge`,
  `measured_discharge_read` (then named `measured_replay*`), `transport_on`, `gluing_of`, `cited_deflation`,
  `verified_install_deflation`, `pay_solve`, `release_verified`,
  `pay_lease`, `settle_solve` and `SolveVerdict::Solved { pay }` read and
  carry `Mu`; `Treasury::settle_recursive` takes `Mu` shares and premium;
  `economy::price` is generic over what it prices. `measured_replay_at` is
  removed (nothing read it).
- A word's magnitude (its measure and identity in the tape), a family's
  magnitude (the mind's metabolism), a demand's ask and the extraction
  need (a stroke size) stay counts: none is μ.
- A sentence waiting on a word presses the word's escrow by the word's own
  discharge — the carriage it causes — where it pressed its own
  magnitude, a count credited to an escrow held in shares.
- `mycelium-soak`'s mint instrument prints each pair's S and G shares and
  its projection's μ as a share.

### SDK API changes
- `mycelium_sdk::wallet::{Mu, MuRefusal}` and `prelude::Mu`; every μ field
  the SDK reads or writes (`Account`, `Wallet::owned`, `StakeAct`,
  `LeaseAct`, `LeaseReply`, `decode_solve_verdict`, `decode_solve_receipt`,
  `Granted`, `EndowmentReading`, `ReserveReading`, `Bond`, `LineHead`,
  `Crossed`) is a `Mu`. `Account::owned` and `Wallet::owned` return a `Mu`,
  no longer an `Option`.
- `TPoint` (`session::TPoint`, `house::TPoint`, `prelude::TPoint`) carries
  its lanes as a `Mu` and is no longer `Copy`; `Session::tuniversal`
  returns a clone of the last final point read. `TPoint::LEN` is
  `TPoint::MIN_LEN`; `TPoint::compare_writings`, `to_bytes`, `put_trailer`
  and `take_trailer` are new.
- `task::SOLVE_FLOOR` and `prelude::SOLVE_FLOOR` are renamed `SOLVE_AGAIN`.
- `wire::WIRE_VERSION` is `0x0200`.
- The conformance read (`docs/MU-CONFORMANCE.md`) checks every μ field on
  an amount holding a share of a unit, and the live read refuses a repeat
  closure that pays.

### SDK
- `mycelium_sdk::rule` gains `perceive_family`, `Frames` and
  `extraction_spend`, so a client sizes its own extraction stroke from the
  family it reads (asked for by a peer kernel's offline extractor,
  netstratum#10). Additive.
- **The deferral** (mycelium#14; `docs/MU-CONFORMANCE.md` §5, version 3):
  the new `mycelium_sdk::deferral` module carries the 43-byte reply a
  hypha with a full worker pool gives every act — `read_deferral(reply)`
  returning `Deferral { deficit }`, `deferral_of` for the `WouldBlock`
  refusal `Session::speak` answers a deferral with, `PENDING_CODE`, and
  `is_deferral`, `deferral_lanes`, `encode_deferral`. The pending code is
  now the library constant `anastome::dialogue::PENDING_CODE`, used by the
  server. The library's `Deferral` field `lanes` is renamed `deficit`.
  Conformance: every profile MUST recognise a deferral; `check_codecs`
  checks it as the finding `DEFERRAL`, and `run` reports a deferred act as
  the new `Finding::Deferred { act, deficit }`, never as a reply that did
  not decode (no retry: the session exposes no measured round trip).
  `examples/client.rs` and `examples/house.rs` read and print a deferral.
- `decode_task_envelope` refuses bytes after the last test input: read
  leniently, the deferral of a zero deficit decoded as an empty envelope.

### Cognition
- **Two frames are related by transduction in turns, never by a common
  lattice** (MU-DERIVATIONS D3.2, MU-RECURSION 78.170). New organ
  `transducer::family::transduce`: a frame's axes are two circles of
  turns (`Turn`, `Enclosure`), a frame is held by its bodies' bands
  (`Enclosed`), a pair by the incidences of its bands under the re-period
  of each circle (`Transduction`, need `R_h·R_w` with `R(a, b) = a + b −
  gcd(a, b)`), the motions keeping the seams are the eight signed swaps
  (`SignedSwap`), and a projection onto an output frame refuses a turn
  that lands on no boundary (`OffSite`). Refusals are
  `TransductionRefusal`. `CommonRefinement`, `refine_delta` (both),
  `arc::Refined` and `Unrefined` are deleted; on `0b148d64` pair 0
  (`18 × 19 → 7 × 9`) the pair is 143 bands and 226 incidences, where
  `K_lcm` wrote 21 546 cells and charted for ≥ 19 s.
- The deflation, the court's measured reconstruction and the demand of an
  unreached pair read the pair's unspoken residue — the identity's speech
  against the observed output — on the output's sites, the unit of the
  form's remainder (a `K_lcm` before was subtracted from an output-native
  after). `declare_unspoken` reads the input's hue on the output's sites
  through the re-period, never off a shared corner.
- A pair of unequal enclosures has no intake route until the turn-native
  chart (O8), and adds no count to a family's magnitude (the mind's
  metabolism, not μ); `transduce::family_share` reads the exact share of
  the frame it changes, and μ reads that share (Economy, above).
- `mycelium-soak --transduction-census DIR` classes every unequal-extent
  train pair of a corpus by relation (O10).

### Fixed
- **A hypha held its court for minutes listing the corpus** (mycelium#14).
  Once refinement was no longer capped, a corpus task whose residues take
  ≥ 19 s each to chart (`invariant_of`) was charted three times a pair at
  intake, the third time under the court lock (the List act's admission
  re-charted the word to file its fibre), and again on every cover change
  on the exchange thread, the court's one logger and stroker: every act
  was deferred, the log went silent and SIGTERM waited on the court. Each
  task's routes (per pair the residue's address, chart, magnitude and
  listed pieces) are now read once on the intake's thread and kept in the
  registry by the task's citation (`mycelium::server::route`); the locked
  step files the chart the seal cited (`Court::file_admitted_charted`), a
  held word's citation and the snapshot read the fibre filed at the
  admission, and a cover change re-homes by lookup. `Court::intake_prepared`
  borrows its pieces. A hypha logs `court corpus routed: N tasks` when its
  intake's thread ends.

### Documentation
- **Reconstruction, rebuild, replay and resend are typed terms** (MU-SCIENCE
  §5.0). Verification is reconstruction only: the court derives the object
  from its own evidence by the laws and admits it on the same exact object;
  replay is for history (Concordance 1.4), rebuild for reproducible builds
  and regenerated caches, resend for the security sense. Every document
  that called the court's verification a replay now says reconstruction;
  MU-HOSTING §7.2 verifies a certificate by checking receipts and
  reconstructing verdicts, never by re-running the universe; two restore
  sentences now state what the code does; reconstruction events (Collapse,
  Converse, Collision) are specified (ROADMAP track 8), and the code
  identifiers' rename is listed (track 11).
- **The boundary law: topology enforced at the universal core** (the
  operator's ruling of 2026-11-02). MU-HOSTING §5a specifies the
  containment topology (mind, world, galactic core, universe, universal
  core, hypha mesh) and the boundary law: a universe's own physics stays
  inside, unseen by the hypha; the universal core tube is its only uplink;
  what crosses is reconstructed under the universal laws and refused by
  name where it does not hold; a universe's act arriving other than through
  its core is refused (`NotThroughCore`, proposed), while a key's own acts
  stay governed by keys. Each part is marked Built or Specified, with the
  readings that observe the boundary and the steps a kernel takes to cross.
  MU-RECURSION 78.169 narrows 78.165 forward; a Concordance amendment to
  4.1, 6.2a, §0 and 0.1 is proposed in its §11 (draft 4, not signed);
  ROADMAP track 24; README, GLOSSARY, MU-LEXICON §1.1 and SECURITY.md point
  to it. MU-HOSTING §5.2 and §3.1 no longer say the court refuses nothing
  for a tier's access: a fold from any tier but the reserve is refused
  today (78.138).
- **The boundary law as double cones, clause-by-clause signing, and issue
  #13 resolved** (the operator's rulings of 2026-11-02). MU-HOSTING §5a
  reads each act as an event with a past cone (what it cites) and a future
  cone (what comes to cite it) on the TUniversal ledger; the universal core
  tube is the vertex where a universe's cones join the hypha's, the inner
  routes are the kernel's own physics, and `NotThroughCore` is derived
  from the cones. §5a.4 and the proposed Concordance §0 SIGNING text
  (§11, draft 4) specify signing clause by clause: the network clauses
  never vetoed, a tube carrying what both universes enabled, a changed
  clause read as signed with its divergence named until signed forward.
  Issue #13 is ruled replication by reconstruction (every hypha
  reconstructs each domain from its founding receipts), now phase B0 of
  ROADMAP track 24; the interim "address the founding hypha" contract and
  the home-court option are withdrawn. MU-RECURSION 78.169 item 6,
  GLOSSARY (Double cone, Vertex, Shared clauses) and MU-LEXICON §1.1
  follow.

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
