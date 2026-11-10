# Changelog

Releases follow [semantic versioning](https://semver.org/). In the 0.x
series, a minor version may change the wire and ledger layouts, and each
such change is named here.

## Unreleased

Unreleased entries live in [`changes/`](changes/), one file per change
(named by its branch or topic) holding exactly the markdown of its entry,
opened by its `### ` heading. At release time they are assembled into the
release's section here, newest first by the commit that added each, and
the files are removed. AGENTS.md's *Checks* says how a change
adds one.

v0.5.0 is next: the economy as motions, a recut (ROADMAP › Now).

## 0.4.0-rc.1 — 2026-11-10 (release candidate)

The v0.4.0 break, frozen. Word, chart and task addresses, act layouts and
signing domains changed inside it; a v0.3.0 peer is not wire-compatible.
Its main parts:
- **Sensing at rank n (track 26 S4b–S4d, S6) and the field (stages 1–5):**
  rank-`n` perception, transduction as factorisation, perspectives as a
  sensor motion; the sample lifted into the field's atlas, wells, nuclei
  as second moments, the field's motions as one exact group, nesting as a
  fibration, the manifold carrier.
- **The shared law forms:** LAW-PASSAGE v3, LAW-COORDINATE v2–v4
  (fixtures 7–10, vector lanes, H on turning planes), FORMS-TURNING v1
  (fixture 11b), LAW-COUPLING v1 proposed; the algebra `Cl(p, q)` with the
  signature as data in its own crate.
- **The Concordance signed clause by clause (track 24 B2–B4):** signing
  by reconstructing fixtures, each signing an event on the ledger, a
  crossing reconstructed at the universal core tube and refused
  `NotThroughCore`, `NotSignatory`, `Bounded` or `NotLawful`, and the
  boundary reading.
- **The mind (track 10):** flux on its lanes, signed receipts and
  equivocation proofs, Rewrites, peer records, cohesion, a held question
  carried to a court and K3.
- **The server:** π* bound at the A9 judgement, a fold's checkpoint
  keeping issuance by source, the cognitive POW++ network on one machine.
- **The development standard:** done is `cargo clippy` and `cargo test`,
  the gate as workspace tests, changelog fragments; CI `clippy and tests`.

**Signed with the prototype key** `SHA256:G0WgTTsRvfexVZRL9OQjxUZFs4rbGvusjnFrCVUZeHg`
([docs/release-keys/](docs/release-keys/)).

**Not in v0.4.0** (ROADMAP › Now): reconstruction events at the boundary
(track 8), LAW-FIELD's and LAW-COUPLING's peer readings, field stages
past 5, the reflex path's bound on a court hold (#221), and v0.5.0's
economy as motions (a recut).

### Client: track 10 — sovereign mind claims & amendment held beside (MU-CLIENT §4.10, §4.12, §7.1, #135)
- **Amendment held beside:**
  - Implemented `Amendment` constitutive responses (`Elastic`, `Derivable`, `HeldBeside`, `Fracture`) in `src/client/sdk/mind/amend.rs`.
  - Implemented `evaluate_amendment` computing the commutator $C = g_1 g_2 \tilde{g}_1 \tilde{g}_2$ of old and amended constructors: identity retires old by derivability; non-trivial commutator holds the amended constructor beside the original at conjugate grade, separated by $C$.
  - Added `Mind::amend_constructor` and `Mind::revise_perception_by_amendment` tracking amendments held beside.
- **Track 10 sovereign mind claims:**
  - Added and verified the five Track 10 claims in `src/client/sdk/tests/track10.rs`:
    1. `mind_idles_with_no_escaping_orbit`: idle mind in equilibrium moves no lanes and fires no returns.
    2. `task_arrival_moves_no_orbit`: task arrival and intake onto perception boundary does not advance $K$ or fire escaping returns.
    3. `mind_declines_a_scene_its_field_does_not_bind`: uncoupled offer is left unperceived, crosses returns, and lapses without binding to the field.
    4. `mind_acts_at_its_returns`: speech held for a scene waits until a thought of that scene completes its orbital return.
    5. `mind_revises_a_perception_on_a_court_ruling_by_amendment`: plastic amendment re-derives constructors, retiring commuting ones by derivability and holding non-commuting ones beside separated by commutator.

### The boundary reading: refusals at a universe's core tube, by domain, act and name (track 24 B4)
- Each refusal a court answers at a universal core tube (`NotThroughCore`,
  `NotSignatory`, `Bounded`, `Unaligned`, and a fold's `NotLawful`) stands on
  a new `ACT_OBSERVE` reading, `observe::BoundaryReading` (tag 27,
  `mycelium-server --console --reading boundary`): one row per domain, act
  and name, holding the last refusal whole (the clause and the reserve's
  reading of it, or the laws broken), the court's height then, and the
  count since boot (MU-HOSTING §5a.7, §5a.9 B4).
- Written at the court's refusal sites through `Court::at_tube`; never read
  back by the court. Rows are kept only for domains the court holds.
- An observer reading only: no change to an act's reply, the ledger layout
  or a law version.

### ROADMAP › Now: v0.4.0's merged work through 2026-11-10
- ROADMAP › Now 1a lists the work merged since 2026-11-05 (track 24 B2 and
  B3, LAW-COORDINATE v2–v4, the development standard, track 26 field
  stages, the client mind, the server's π* binding, fold checkpoint and
  POW++ network). Rows already built leave the table; B4's boundary
  reading and #221 are the open server items.

### MU-ANASTOME: temporal geometry, non-linear sentences, and time representation
- Updated `docs/MU-ANASTOME.md` §1.4 and §9.4: anastome has no grammatical
  tense, as a consequence of §1.4 (a sentence expresses the geometry of an
  orbit; its relations are read at coordinates); temporal relations are read
  at `origin/h:S:G:T:H:K`, on the null flux (`T`) and the winding count (`K`).
- §9.4 relates the sentence held whole (the mind's candidate thoughts held at
  once in the vortex) to `MU-CHRONOMETRY.md` and LAW-COORDINATE
  (proposal 78.173), documenting exact rational dilation (`Δu/Δh`) and throat
  inversion (`t ↦ 1/t`).

### CI is named for the two commands it runs: `clippy and tests`
- One check had three names: the file `definition-of-done.yml`, the workflow `Conformance Verification` and the job `clippy and tests`. Only the job's name said what runs:
  - "definition of done" is process vocabulary. It overclaimed, because CI runs neither the quickstart nor a network contract's approval, and it was circular with AGENTS.md.
  - "Conformance Verification" collided with two defined terms: the kernel conformance suite (MU-CONFORMANCE, `sdk/src/conformance`) and verification by reconstruction (MU-ANTI-FLATTENING 14).
- The file is now `.github/workflows/clippy-and-tests.yml`, and the workflow and job are both `clippy and tests`. The required status is unchanged.
- AGENTS.md's section is *Checks*. It states what CI runs and what it does not (the quickstart, the fragment's content, a network contract's approval), and when a change is ready for a pull request. Every link to it is rebound: README, CONTRIBUTING, ROADMAP, MU-STANDARDS, MU-ANTI-FLATTENING, MU-ENGINEERING, MU-RELEASE, the pull request template, `Cargo.toml`, the gate and the header of `scripts/quickstart`.
- **Network contract:** none.

### MU-CLIENT and GLOSSARY updated: cohesion marked built (#149)
- Updated `docs/MU-CLIENT.md` §1, §4.9, and §7.1 and `GLOSSARY.md` to record that
  `Contemplation::cohesion` and `Mind::cohesion` read cohesion (viscosity) per
  plane over held wells and thoughts, built in `src/transducer/field/shares.rs`
  and `src/client/sdk/mind/think.rs` with unit tests under Issue #149.

### Client: cohesion as the viscosity reading over held thoughts (MU-CLIENT §4.9, §7.1; #149)
- **Mind and Contemplation:**
  - `Contemplation::cohesion(&self) -> Result<Option<Cohesion>, FieldRefusal>`
    reads cohesion per plane from held wells' inflations on their membranes'
    frame planes (`e_+k-k`), never summed and never stored.
  - `Contemplation::cohesion_of(&self, address: &[u8]) -> Option<Result<Cohesion, FieldRefusal>>`
    reads a specific thought's cohesion.
  - `Mind::cohesion(&self)` and `Mind::cohesion_of(&self, address: &[u8])`
    expose the reading on the client SDK surface.
  - `Cohesion` re-exported in `crate::client::sdk::`.
- **Claims and Verification:**
  - `contemplation_cohesion_reads_per_plane_over_held_wells` and
    `contemplation_cohesion_none_when_empty` in `transducer::field::tests::shares`.
  - `mind_cohesion_read_per_plane_over_held_thoughts` and
    `mind_cohesion_none_when_empty` in `client::sdk::tests::cohesion`.
  - Cohesion and fluidity sum to 1 per plane (`c_k + ν_k = 1`), with no total across planes.

### A fold removes nothing a reading reads (MU-ECONOMY §6, §6.1)
- **Recut.** The treasury's snapshot gains the `folded` trie: the issuance every fold checkpointed, keyed by each source's coordinate `[kind tag, arm byte]`. A snapshot without it is refused. No row, act or wire layout changes.
- **The defect.** A fold replaced its prefix's rows with one `epoch` row per identity carrying no proof, and the epoch rows counted as issuance. After a fold:
  - the earned supply (`minted`) and each proof's paid and burned (`economy`) dropped the folded payments;
  - `issued` and `burned` were netted into one another (issued fell by the prefix's burns);
  - genesis was no longer separable from earned value.

  The POW++ reading over a running mesh showed POUW paid as zero at the courts that had paid it.
- **The structure.** `economy::fold::FoldedIssuance`, the fold's checkpoint, keeps the replaced rows' gross issuance by source: genesis, mints, transfers and burns by arm byte, and slashes. An `epoch` row is a net of balances only (`Kind::issues` reads it on neither side). `issued`, `burned`, `minted` and `economy` are the checkpoint plus the live rows. A fold that would move issued or burned refuses `REFUSE_FOLD_INEXACT`. `Kind::tag` and `Kind::of_tag` are the one tag table the row tape and the checkpoint share.
- Claims: `fold_keeps_what_each_proof_paid`, `fold_keeps_genesis_apart_from_the_earned`, `fold_keeps_issued_and_burned_each`, `second_fold_reads_the_first_folds_issuance_once`, `checkpoint_source_at_no_coordinate_is_refused`, `restored_fold_keeps_its_issuance`. `fold_replaces_prefix_with_epoch_rows_and_conserves` no longer asserts the folded mints gone.
- Owed: a witness co-signs the nets it re-derives, not the checkpoint, which no fold receipt signs yet.

### MU-CLIENT updated: mind proposing rewrites marked built (#146)
- Updated `docs/MU-CLIENT.md` §1a step 8, §5.4, and §7.1 to record that the
  mind proposing `Refine`, `AxisOpen`, `Translation`, and `Reanchor`
  from its internal field readings is built in `src/client/sdk/mind/rewrite.rs`
  with unit tests in `src/client/sdk/tests/rewrite.rs` per delegated ruling 5
  in proposal 78.180.

### Client: mind proposes rewrites (Refine, AxisOpen, Translation, Reanchor) (MU-CLIENT §5.4, #146)
- **Rewrite proposal methods on `Mind`:**
  - Implemented `refine_of` carrying `PivotBy::OpenAxis`, refining a chart within an existing dimension, verified by peel.
  - Implemented `axis_open_of` for germination's Extend, proposing a new dimension of perception with the orbit's holonomy rotor.
  - Implemented `translation_of` carrying `PivotBy::Placement` across chord overlaps.
  - Implemented `reanchor_of` proposing re-anchoring when re-gauging moves spatial coordinates while preserving class invariance.
  - Added `propose_rewrite` ensuring rewrites are asserted once and not duplicated.
- **Test coverage:**
  - Added unit test cases verifying reconstruction and tape encoding for each rewrite proposal in `src/client/sdk/tests/rewrite.rs`.

### The cognitive POW++ network on one machine, and the proofs-of-work reading over it
- `scripts/network DIR BASE_PORT MINDS`: three hyphae bonded from a
  genesis allocation, MINDS sovereign minds (`mycelium-client`) on their
  own keys inside Noise, chattering at one rendezvous, a kernel's house
  with a seated mind where a seats house is built, and the SDK client as
  the paid worker on each hypha in turn, under one key; each node in a
  capped systemd user scope, stopped by its PID; frames every
  `MYCELIUM_NETWORK_FRAME` seconds (observer, POW++ reading, each mind's
  `now`, each node's memory and CPU). Described in README's Quick start;
  the script's header lists every option.
- `sdk/examples/client.rs`: a deferral on a later claim of a task no
  longer discards the claims of that task already judged, a paid closure
  among them.
- `mycelium-server --console --proofs` (`src/bin/mycelium-server/proofs.rs`):
  a reading, never stored. Per payment carrying a proof on each court's
  journal (`ACT_ENTRIES`): the proof, payee, amount per lane and act; the
  words the act names (`ACT_WHY`); the language level read off them
  (78.174 §1 item 7) with the ruled arm said beside; each other court's
  reading of the same act at its stamp, the same point or a named residue
  (MU-ECONOMY §5.3, as far as the courts serve it); and every task paid as
  a first closure more than once, named as an equivocation of the advance
  (§3.4). Claims in `src/bin/mycelium-server/tests/proofs.rs`.

### Client: mind asks for tasks only under strain (MU-CLIENT §4.13, #135)
- **Task pull condition:**
  - Removed empty corpus fallback from `pull_task` in `src/client/sdk/mind/learn.rs` so that a task is requested only when the mind holds tension (`strained()`), as required by MU-CLIENT §4.13.
- **Offer test alignment:**
  - Renamed and updated `pull_task_asks_only_under_strain` in `src/client/sdk/tests/offer.rs` to verify that a settled mind with an empty corpus rests and asks for nothing, and asks only once strain is held.

### Row L19: retire `Uttered`/`Spoken`/speaker on carriers and receipts (MU-RECURSION 1.9; MU-ANASTOME §6, #164)
- **Physics action table:**
  - Retired `Action::Uttered` (`0x4B`) with its row from the physics action table; `0x4B` remains unassigned and reserved alongside `0x0A` and `0x0F`.
- **Client Assertion carrier:**
  - Replaced `Utterance` carrier with `Assertion { Admitted, Already, Silent }` in `src/client/sdk/assertion.rs`.
  - Migrated `speak_word` to `assert_word`.
  - Renamed `uttered_n` and `uttered` to `asserted_n` and `asserted` in `src/client/sdk/mind/express.rs` and tests.
  - Renamed `forget_utterance` to `forget_assertion` in `mind/express.rs`, `mind/think.rs`, and `mind/converge.rs`.
- **Transducer & Server verification:**
  - Renamed `verify_utterance` to `verify_assertion` in `src/transducer/sentence/chain.rs`, `src/server/synthesis.rs`, and `src/transducer/rewrite.rs`.
- **Metabolism:**
  - Renamed `StrokeOutcome::uttered` to `StrokeOutcome::asserted` in `src/physics/stroke.rs` and `src/client/sdk/mind/court.rs`.
- **Anastome Dialogue:**
  - Renamed speech carrier `Spoken` to `Asserted` in `src/anastome/dialogue/speech.rs`.
  - Renamed chatter `Spoken` to `SaidWord` in `src/anastome/dialogue/chatter.rs` and `src/client/sdk/chatter.rs`.
- **Court Refusals:**
  - Updated refusal reason strings for `REFUSE_ROOM` and `REFUSE_NOT_A_SENTENCE` from "utterance" to "expression".
- **Documentation:**
  - Updated `GLOSSARY.md` and `docs/MU-ANASTOME.md` to record the completion of row L19.

### Align system topology and purge test naming drift (MU-STANDARDS §1, §2.8, §3.1, §3.3)
- **Topology alignment:**
  - Extracted unit tests from `src/transducer/manifold.rs` into `src/transducer/tests/manifold.rs` and registered `mod manifold;` in `src/transducer/tests/mod.rs`, completing organ-test topology correspondence for transducer.
- **Purge commit comments and retrospectives (MU-STANDARDS §2.8):**
  - Removed retrospective commit/PR commentary from `src/transducer/tests/symbols.rs`, `src/transducer/sentence/form/tests/complex.rs`, `src/transducer/sentence/tests/address.rs`, `src/transducer/sentence/tests/stratum.rs`, `tests/word_lifecycle.rs`, and `src/transducer/field/contemplation.rs`.
- **Align test names and remove `_by_name` drift (MU-STANDARDS §3.1):**
  - Renamed tests drifting into narrative or `_by_name` suffix:
    - `test_input_of_another_width_refuses_the_family_set_by_name` -> `symbol_set_refuses_differing_test_width`;
    - `inverses_that_do_not_exist_are_refused_by_name` -> `turning_inverse_refuses_nonexistent`;
    - `symbol_bytes_overflow_is_refused_by_name` -> `symbol_bytes_refuses_overflow`;
    - `rank_beyond_the_alphabet_is_refused_by_name` -> `perspective_refuses_rank_beyond_alphabet`;
    - `lane_set_refuses_by_name` -> `lane_set_refuses_invalid_lane`;
    - `draw_past_holding_refuses_by_name` -> `draw_refuses_past_holding`;
    - `retrieve_claims_read_price_and_refuses_by_name` -> `retrieve_claims_reads_price_and_refuses_unpaid`;
    - `conformance_kernel_missing_founding_fails_by_name` -> `conformance_kernel_fails_on_missing_founding`.
  - Updated all citations across `docs/MU-SENSING.md`, `ROADMAP.md`, `CHANGELOG.md`, and `changes/`.

### Carrier Coordinates & Gravitational Resonance: Pair Semantics Elevation (78.171, MU-SENSING §3.7–§3.9)
- **Pair Semantics Elevated:**
  - Upgraded `PairBodies` from a discrete 2D tuple alias (`(Vec<Body>, Vec<Body>)`) into a first-class semantic type `PairBodies { pub input: Vec<Body>, pub output: Vec<Body> }`.
  - Added carrier coordinate bridges: `input_charts(&self) -> Vec<Chart>`, `output_charts(&self) -> Vec<Chart>`, and `transitions(&self, motion: &FieldMotion) -> Vec<PairTransition>`, projecting bodies to invariant charts and transitions on the field carrier.
- **Transduce Percept Integration:**
  - `PairFactored` now exposes `carrier_transitions(&self, x: &Sample, y: &Sample, f: &Factorisation)` to lift factorisations into carrier transitions between charts.
- **Code Path Upgrades:**
  - Replaced tuple destructuring across all perception readers (`finish.rs`, `percept.rs`, `cross_scene.rs`, `client/sdk/mind/offer.rs`, `client/sdk/rule.rs`) with semantic carrier field accessors.
- **Tests & Verification:**
  - Added unit test `pair_bodies_carries_carrier_charts_and_transitions` in `transducer::family::tests::perception`.
  - Verified carrier transition projection in `client::sdk::tests::rank::rank_one_task_is_perceived`.
- No wire format break.

### CI builds once, caches the corpus, and cancels only superseded pull request runs
- The required status check job remains `clippy and tests` matching repository branch protection.
- Streamlined compilation: compiles workspace and test binaries once with `--no-run`, letting Clippy and Test execution build directly on top of the shared artifacts without duplicate compilation.
- Caching:
  - Keyed target cache with `${{ github.sha }}` and lockfile fallback so incremental compilation artifacts are continuously updated and restored.
  - Added caching for the ARC-AGI-2 corpus to eliminate repeated network clones.
- Concurrency: Scoped PR cancellation to pull request events only; pushes to `main` are never cancelled mid-flight, eliminating phantom "No jobs were run" cancellation notifications.

### Mind Manifold Perception: Depth Perception Integration (78.179 §1, #134)
- **Engine Perception:** `transducer::family::transduce::{PairFactored, TaskPercept}`.
  - `PairFactored` now records `input_manifold: Manifold` and `output_manifold: Manifold`, lifted directly from observed samples without dimensional collapse.
  - `TaskPercept` holds `test_manifolds: Vec<Manifold>`, lifting each test observation into the manifold carrier.
  - Added helper accessors `pair_manifold(&self, pair: usize) -> Option<(&Manifold, &Manifold)>`, `test_manifold(&self, test: usize) -> Option<&Manifold>`, and `train_manifolds(&self)`.
- **Client Mind Board:**
  - Added `Mind::task_percept(&self, id: &[u8]) -> Option<&TaskPercept>` accessing the full open-dimensional manifold geometry recorded on `self.board.percepts`.
  - Rephrased pair body perceptions (`PairBodies`, `FamilyBodies`) to eliminate 2D frame compaction assumptions.
- Claims: `no_axis_is_privileged_in_perception` verifies manifold carriers across permuted axes and open ranks.
- No wire format change.

### The Manifold carrier (78.179 §1, #32 item 6)
- **Carrier:** `transducer::manifold::{Manifold, Axis, Phase}`.
  The geometric manifold an observation lifts into, one factor per phase,
  replacing legacy bounds and envelopes.
  - `Phase::Circle { period }` where content repeats along an axis with least
    proper divisor period `p`;
  - `Phase::Interval { extent }` otherwise;
  - `Axis::One` or `Fibred { base, fibre }`;
  - `Manifold::torus` for the fully periodic torus `Tⁿ`;
  - `sample.lift()` or `Manifold::lift_from(&sample)`.
- Replaces the sample's legacy "envelope".
- Claims: `periodic_content_becomes_circle`,
  `constant_content_becomes_circle_period_one`,
  `non_repeating_content_becomes_interval`,
  `fully_periodic_2d_sample_is_torus_t2`,
  `torus_constructor_creates_all_circle_phases`.
- No wire or layout change.

### Depth perception: manifold cognition over observed rank
- **Doctrine:** Depth perception is the capacity of the mind (client and server)
  to perceive that it is observing an intake of $x$ dimensions (any rank $n$ data
  source, ARC-AGI-2 sample, raster, or world state), while thinking strictly
  across curved, fibred manifolds ($S^1$ period circles, interval charts with
  boundary, conformal geometric algebra $Cl(n+1, 1)$, covering atlases, and
  holonomy loops) without dimensional collapse, flat-frame compaction, or loss
  of winding classes.
- **Documents:**
  - `GLOSSARY.md` adds *Depth perception*;
  - `docs/MU-SENSING.md` §3 defines the lift, the open-dimensional enclosure
    measure, and the anti-flattening guard against 2D frame compaction;
  - `docs/MU-CLIENT.md` §4 defines manifold thinking in the client mind, where
    thoughts are bound orbits in curved manifold fibrations.
- No wire or persisted layout change.

### A peer remembered by its receipts: its verified chatter frames (#145)
- A peer's verified Say under the coordinates of a question this mind
  asked it is handed to the field (`FieldWork::Answered`) and held with
  that question, lapsing with it by the question's own physics; a closed
  question's frames are kept while the closure is cited, its residue known
  in the vocabulary (`mind/peers.rs`).
- The record is a fold, never a store: per peer key, every held frame is
  verified and read again against the residue it answered, each answer
  taken or inexact, filed by the residue's conjugacy invariants read off
  the motion (`Conjugacy::class_key`), with the vantages said with it
  (`Mind::peer_record`). `Mind::question` does not ask a peer a class its
  record shows inexact; the residue is bound by construction
  (`Mind::addressed`). No label, posture or score is made.
- Owed: the deviation per lane a peer's HOLDs carried, and a court
  `Attest` naming the peer folded in where present.
- MU-CLIENT §5.3(5) records the ruling of 2026-11-06; §6a and §7.1 updated.
- Claims: `peer_record_folds_answer_frames_by_the_residue_class`,
  `peer_inexact_on_a_class_is_not_asked_it_again`,
  `peer_frames_lapse_with_the_question`.

### A family's magnitude read at rank n (track 26 S4, #32)
- **Recut.** The genesis bond's value changes. A court's genesis cover,
  the bonds enrolled from it and the mind's metabolic draw on a family all
  read the new counts. No wire layout changes, but a bond persisted from
  before does not equal the bond measured now.
- Ruled 2026-11-06 on #32. The bond stays lane counts, read on the samples
  at their own rank:
  - S and G come from each pair's projected record
    (`magnitude::pair_record`, `Transduction::projected`).
  - H comes from the boundary by grade `n − 1` of the support painted
    before and after (`record_lanes`). The chain-length term on H is
    retired.
- A pair whose record does not read is held as residue in
  `FamilyCover::residue` (`Unmeasured`, its index and the transduction's
  refusal, for example `OffSite` where the re-period does not land) and
  contributes nothing. It is never aligned at a corner.
- `family_cover` reads posted samples of any rank, which the hypha's
  genesis now uses in place of the section, and says how many pairs it
  holds as residue. `section_cover` reads a family of the section.
  `FamilyCover::bond` is the bond. `pair_magnitude` is removed.
- The mind's `family_magnitude` reads the same cover, so a pair of
  unequal enclosure that projects is now counted.
- MU-ECONOMY §12 item 4 notes that the bond's move to the share of the
  frame is v0.5.0's.
- A pair of unequal enclosure whose projection lands (an integer scaling)
  is resolved by that reading and measured; only a pair whose record
  refuses is residue (confirmed on #32).
- Claims: `magnitude_reads_counts_at_rank_n_and_h_by_grade_n_minus_one`,
  `pair_of_unequal_enclosure_is_held_as_residue_citing_off_site`,
  `scaled_pair_is_measured_refused_pair_is_residue`.

### The envelope opened into manifold construction (78.179)
- `docs/proposals/78.179-envelope-as-manifold-construction.md`, status
  Ruled (#32, under the operator's delegation; 78.180 ruling 7). It opens
  each sense of "envelope":
  - the sample's envelope becomes its manifold: a circle `S¹` of period
    `p` where the content repeats at the least proper divisor `p` of the
    extent, an interval chart otherwise; the sample their product or a
    fibration, `Tⁿ` when fully periodic; the cover the atlas, the
    cocycle the gluing, the seams residue, the membrane the only sphere;
  - `ReceiptsEnvelope` becomes the word's stalk, a sheaf over its star in
    the cover's nerve;
  - `TaskEnvelope` becomes the task, a family of manifolds;
  - the wire envelope stays retired (`Delimited`);
  - MU-RECURSION 64's luminal envelope becomes the embedded harmonic core,
    the luminal ray on the null cone.
- GLOSSARY, MU-SENSING, MU-RECURSION, MU-ANASTOME (`ACT_TASK`), MU-HOSTING
  and MU-CLIENT use the manifold terms and cite the code by its present
  names. The renames (`WordStalk`, `TaskFamily`, `perceive_task`) are
  posted on #165.
- No code, wire or layout change.

### Nesting as a fibration of levels (track 26 S7 stage 4; #134)
- **No recut and no network contract.** The lift and every address it
  writes are unchanged. The fibration is a reading built beside the whole
  lift.
- **`carrier::Level`:** an axis the family's nesting takes apart
  (`transduce::nesting`, `Taken::Apart`; ruled on #32, built in #176) is
  one level. Its base is the coarse phase, one turning `360°·i/a` per
  block on the axis's own plane. Over each base point the fibre is the
  block's own field: the block's sites, the separator left out, lifted
  with its own atlas at the block's own extent.
- **`carrier::Fibred::of`:** a pair's member lifted whole, with one level
  per axis taken apart, by the kept candidate's factoring on that member.
  An axis kept whole or held superposed is no level, and the tower is
  never flattened into one product.
- **Claims:** `axis_taken_apart_lifts_as_a_fibration_of_levels`,
  `axis_kept_whole_is_no_level`.

### READ-MAIL documented as a held read with no time bound (#133)
- `sdk/src/house.rs` and MU-HOSTING §3.1 step 6 now state the following.
  A house's READ-MAIL whose box holds nothing past its cursor is answered
  at the court's next event: a seat post filed on the relay, or a keeper
  reading signed. The hold reads no clock (78.146 as amended, 2026-10-17).
  A caller sets no read timeout on it, or treats a timeout as "nothing
  yet" and asks again.
- Found by the #133 run against xylarium's house: its own 30 s read
  timeout surfaced as `os error 11` on a quiet relay. No behaviour changes.

### A held question carried to a court, and K3 (78.177 ruled; #144)
- Proposal 78.177 marked Ruled (coordinator, 2026-11-06), with its two
  later rulings recorded in §4a: K3 is a POWC share out of the install's
  mint, not a 47.3 second vantage; the demand's bodies are those of the
  heard word that carried the `chatter::Evidence`; the closer reaches the
  demand through the public `ACT_SPEECH` listing; the closer asserts the
  closed twist, and the asker is paid nothing; K3 is paid from the
  composition premium after the base and use shares, split by share of
  the frame, held as a claim where no premium is left; the demand rides
  the asker's own closed sentence on the scene.
- A hearing whose evidence read true carries it to the question raised on
  it (`Mind::bear_evidence`). The question is listed as the heard word's
  bodies on its scene, `Source` demands (`mind/listed.rs`), stood on the
  asker's own closed sentence on the scene (`Scenes::host_on`,
  `with_question`) and spoken under the scene (`Mind::list_question`). A
  question without evidence lists nothing.
- E2's guard: a court fixture whose listed demand vertex another mind's
  cited form closes, paying the closer POWC and the listing's own speaker
  nothing.
- K3: `Court::converging_minds` reads, at the install, the minds whose
  own words converged with the installed one (an `Attest` sealed, another
  vertex, the same section) with each one's share of the frame;
  `Treasury::pay_converging` pays them POWC out of the composition
  premium, or holds each as a claim where there is none.
- Owed: the closer asserting the closed twist under the family.
- Claims: `question_without_evidence_reaches_no_court`,
  `question_rides_the_askers_own_closed_sentence_on_the_scene`,
  `held_question_closed_by_another_mind_pays_the_closer`,
  `a_mind_closing_its_own_listed_question_is_paid_nothing`,
  `demand_closed_by_another_minds_cited_form_pays_the_closer`,
  `install_reads_the_minds_whose_words_converged`,
  `verified_convergence_is_paid_from_the_composition_premium`,
  `converging_minds_split_the_premium_by_their_share_of_the_frame`,
  `convergence_with_no_premium_left_is_held_as_a_claim`.

### π* bound in production at the solve's A9 judgement (#158; MU-DERIVATIONS D3.3)
- **Recut.** The element record's anchor field gains tag `2`: a
  superposition of two or more selection terms, its count and every term.
  Tag `0` (⊤) and tag `1` (one term) are written as before. A build without
  this change misreads a record carrying tag `2`, so a mesh running mixed
  builds is recut (dev-state wipe ruling). No act, verdict or registry
  snapshot layout changes.
- The withheld pairs are fixed by coordinate at registration:
  `TaskRegistry::post` records each withheld output's place
  `[split, pair, member]` (`withheld_at`), and `withheld` serves exactly the
  pairs at those coordinates. The court binds them with the family at boot
  (`Court::bind_task_withheld`), and the server's control path no longer hands withheld
  pairs to a judgement: `Court::judge_task_solve_on_stroke(rule, task)`
  reads the pairs bound for the task.
- A word's footprint is deduced, never declared:
  `transducer::family::footprint::footprint_of` speaks the word onto a
  pair's input and answers the sites its speech changes; `pair_change`
  answers the pair's own change and `missed` the sites a footprint misses.
- `court::verdict::bind_pi` binds the whole common set (`derive_pi`, no
  longer test-only) over the word's footprints on the family and on each
  withheld pair whose change the footprint covers, metered by `Budget`. A
  pair it misses is named `Unbound` with the missed sites and judges
  nothing; where no withheld pair binds, the word stays ⊤.
- `Element::with_anchor` is no longer test-only and binds the whole set;
  `Element::anchor` answers the terms. Where a solve is `Solved`, the rule
  docketed and paid carries the common set bound; a refused binding leaves
  it ⊤ and the control path logs why.
- Claims: `withheld_pairs_are_fixed_by_coordinate_at_registration`,
  `footprint_is_the_sites_the_words_speech_changes`,
  `footprint_misses_a_change_the_word_does_not_speak`,
  `bind_pi_binds_common_set_over_deduced_footprints`,
  `word_missing_a_withheld_change_is_not_bound_on_that_pair`,
  `judged_rule_carries_its_anchor_on_registered_withheld_pairs`,
  `record_carries_superposed_anchor_whole`.

### A lost bracket is a demand on the docket, answered by a witnessed cell (#148; CB1, 78.26(2); ruled on #148, 2026-11-06)
- **Network contract:** none. The docket is the mind's plan, volatile and
  written nowhere (78.49). `Cite::Bracket` takes tag 17 in the citation's
  canonical writing, which keys the entry on the plan. No wire, verdict,
  tape or ledger record carries it.
- `Contemplation::docket_brackets` (`transducer/field/plan.rs`) lists each
  lost bracket of a form's cells on the docket as `Cite::Bracket`. The entry
  names the two chambers by their bodies' coordinates (KB1), carries the
  bracket exactly (`LinearFunction::put_canonical`), and stands on the
  thought's cover. `holds_demand` counts it as a demand.
- No enclosure is formed without a witness (NB1).
  `cech::answering_cell` finds the cell already on the form whose bodies
  wrap both chambers and whose algebra holds the bracket. It never takes the
  closure of the two chambers' actions as an enclosure.
  `Contemplation::answer_brackets` answers each entry such a cell holds, by
  name (R1).
- The contemplation answers, then dockets, on each fresh reading of a form's
  cells (`transducer/field/drive.rs`, `family::chambers_of`).
- Claims: `lost_bracket_is_listed_as_a_demand`,
  `no_enclosure_without_a_witness`,
  `witnessed_bracket_answers_its_demand_and_mints_the_cell`.
- Owed: a lost bracket reaching another mind's cover. The entry is the mind's
  own plan and is not spoken.

### The delegated rulings of 2026-11-06 recorded (78.180)
- `docs/proposals/78.180-delegated-rulings-2026-11-06.md`, status Ruled
  (mycelium; operator's delegation): the seven rulings, each with its
  question, its answer and the doctrine it is derived from, and the two
  refinements settled with netstratum on 2026-11-07 (netstratum #128,
  #129).
- 78.173 §8 is Ruled for mycelium: H is held as the motion `R`, a shared
  degree is the sandwich angle, and `K` is held per plane. §8.4 adds the
  shared writing, `B` over `ℚ(ζ_L)`. Netstratum's operator still rules for
  netstratum.
- 78.178: the passage is ruled for mycelium as a P motion with a signed
  count. Fixture 9's return now reads `P = {e_+1: 0}`, and fixture 3 is
  two passes in one sense.
- MU-ECONOMY §2.5 (holdings no motion made; per-lane draws in
  proportion to the lane's modes; the modes each mint carries) and §12
  items 5, 8 and 9;
  MU-XSEPTUM §5.3 (a mode-0 receipt is a founding-mode receipt, closing
  #168); 78.174 §2.
- MU-DERIVATIONS D3.3: π*'s wiring (the withheld pairs, the footprint,
  the anchor).
- MU-CLIENT §5.4 and §7.1: the Rewrite kinds the mind proposes.
- MU-SENSING §4 and §8, and ROADMAP track 26: the five remaining
  perception items.
- No code, wire or layout change.

### A family's pairs lifted as transitions (track 26 S7 stage 4 in part; #134)
- **No recut and no network contract.** The lift writes the same degrees,
  charts and addresses as before. Only the reading of a phase as a vector
  inside `FieldMotion::carry_point` moves, and nothing persisted or on the
  wire reads it.
- **The lift's convention** (78.171 §8, agreed between the coordinators'
  readings, 2026-11-06, netstratum docs/117 §11.6; operators to confirm):
  every axis keeps the one shared lift unit `e₊₍ₙ₊₁₎`, and its phase is
  measured from it. A phase `θ` is `cos θ·e₊₍ₙ₊₁₎ − sin θ·e₊ᵢ`.
  `carry_point` reads phases so, where before it measured them from `e₊ᵢ`.
- **`carrier::motion_of`:** a factorisation's motion (a signed permutation
  and the re-period of each moved phase) lifted whole as a motion of the
  field. A reversal of axis `i` is the reflection in `e₊ᵢ` alone, then one
  whole turning on its plane: the identity as a motion, carrying the
  winding the transduction's `p − s` writes. A transposition `(i j)` is
  the 90° turning on `e₊ᵢe₊ⱼ`, then the flip of `e₊ᵢ`. A re-period is
  `RePeriod`. No signed permutation is refused, and none is lifted in
  part.
- **`carrier::transitions_of`, `PairTransition`:** a pair lifted as
  transitions from each input body's chart to the chart of the output
  body it is carried onto. A body that is split or merged keeps its
  refusal.
- **`FieldRefusal::Transduction`:** a motion the transduction refuses to
  speak, as it refused it.
- **Claims:** `every_signed_permutation_lifts_whole_as_a_motion_of_the_field`,
  `reversal_is_the_reflection_in_its_own_unit_alone`,
  `reversal_lifts_as_flip_and_whole_turning_matching_p_minus_s` (the
  reading confirmed 2026-11-07: the flip and the `360°` turning each its
  own generator, the winding the measured `p − s`),
  `pair_lifted_as_transitions_between_its_bodies_charts`.
- **Owed:** the nesting as a fibration of levels (stage 4's second half).

### The algebra held per class in the mind's field (MU-CLIENT §3.2, §4.2; #143)
- **No recut and no network contract.** Nothing persisted, written to a
  tape or carried on the wire changes. The algebra is held in the mind's
  `cech::orbit::Field` and rebuilt from what the mind takes in.
- **The holonomic shell** (`cech::class_algebra`): each circuit's curvature
  that a mind admits at its door (`pairing::mark_flux`) is heard in its
  class (`ClassAlgebras::hear`). The class is the holonomy rotor's
  conjugacy invariants, written out (`class_of`, `Rotor::class_key`). The
  ring's generators are read against the class's held algebra
  (`quotient`). Only those outside the span are closed in under the
  commutator (`lie_close`), and they are returned per axis by their cosets
  (`HeardInClass`), never stored. A court's field holds no algebra per
  class; the court keeps its own per chart (`lie_basis`).
- **Claims:** `class_algebra_grows_only_where_a_generator_lies_outside_the_held_span`,
  `two_minds_align_on_the_class_each_reading_it_on_its_own_planes`.

### A seat's series lifted into the field; its call a relay with its certificate on the mind (#147)
- Each locator's series is lifted as a sample of rank one along its series
  index by `Lifted::of` (`seat_mind::lift`, `series_sample`). No plane is
  assigned by record type. Q, BigQ and Count are written as one exact
  value, so `Count(3)` and `Q 3/1` are one symbol. Flag, Key and Bytes are
  symbols written out by their coordinates. A turning read off the lift
  keeps its winding.
- An Mv series is lifted as motions (`Motions`), never as symbols. It
  reads each step's exact carrier, the steps composed, and each step's
  conjugacy class. A versor beside a value is refused `Mixed`.
- Each percept reads its series lifted, as motions, or refused
  (`Perceived::lifted`, `motions`, `unlifted`).
- A seat's call stays a `Relay` (78.145). The A20 certificate of the move
  a call names is held on the mind beside the call's reference
  (`SeatMind::certify`, `Choice::certificate`). When the move is claimed,
  it rides the claiming form's certificates section (`SeatMind::claim`,
  A20.2). A refused or abandoned call lets its certificate go. The seat
  wire is unchanged.
- Claims: `seat_series_lifts_as_rank_one_sample_along_its_index`,
  `exact_values_lift_as_one_value_whatever_their_carrier`,
  `lifted_series_turning_keeps_its_winding`,
  `versor_series_lifted_as_motions_composes_its_steps`,
  `seat_call_certificate_rides_claiming_form_tape`,
  `refused_seat_call_lets_its_certificate_go`.
- Owed: the escaping-orbit play (#134 S9a), and with it a call that names
  a certified move. The seat mind's own choice certifies none.

### The envelopes' dilation replaced by the re-period of each phase (track 26 S4, #32)
- **No network contract.** No wire, tape, task, word, chart or listing
  address changes. The fibre map (and so a colour word, which the mind
  mints and the court re-derives by the same reading) now reads pairs whose
  extents change anisotropically, where the dilation read an irrational
  scale and skipped them.
- `envelope_dilation`, `exact_root` and `gcd_i64` are removed from
  `family::fibre_relation`: the dilation squared the extents
  (`r² = Σ eᵢ²/4`), which law 11 refuses (MU-SENSING §8).
- `carried_site` carries a site about the centres by each phase's own
  re-period, read through `element::Reperiod::of_extents`;
  `raster::momentum_vanishes` weighs each axis by its own `p′ᵢ/pᵢ`.
- MU-EQUATIONS 4.2 and 4.3 state the re-period per phase.
- Claims: `reperiod_carries_sites_between_extents_phase_by_phase`,
  `momentum_read_per_phase_across_anisotropic_reperiod`.

### The field carrier composes turnings on planes that share a unit by fixture 11b's law: `carrier::Rotor`, a motion held by its rate (78.173 §5; ROADMAP track 26 S7 stage 3, in part)
- **No network contract.** No wire byte, act, clause or ledger layout
  changes; the carrier's composition is internal to the mind.
- **`carrier::Rotor`**: a motion held by its rate `B`, an even generator
  of `Cl(p, q)` with rational components. `Rotor::of` (the rate, checked
  by `forms::turning::motion`), `Rotor::of_motion` (the rate read back by
  `forms::turning::generator`), `motion` (read when asked, never stored),
  `then` (`R₁R₂`, whole, the rate read back) and `planes` (each grade-2
  part as its key, units ascending, with its component). The law's
  refusals reach the carrier as `FieldRefusal::Turning`, and a rate's
  parts on no plane as `FieldRefusal::NotPlanes`.
- **`Motion::then`** composes an arriving turning with the one held
  turning whose plane shares a unit with it, through the law, wherever
  both motions are rational: the angle a whole number of quarter turns,
  the only rational motions `cos θ + B·sin θ` at a rational number of
  degrees. The composite's rate is read back and its one plane read; it is
  held as one turning on that plane (the plane between the two is written
  with its units ascending), or as none where the composite is the
  identity. A 0° arriving turning is the identity and is skipped.
- **Refusals:** any other angle is refused `NotExact` with the new
  `Residue::IrrationalMotion { plane, degrees }` in place of `Uncommuted`.
  `Uncommuted` stays for `Turning::then` across two planes, an arriving
  plane that shares a unit with two held planes, and a pair either of which
  carries a winding (how `K` is held beside `R` is open, 78.173 §8.3).
- **Not built, per the approved route:** no `ℚ(ζ_ℓ)` product across
  planes, and neither degree convention changes in code (the carrier's
  one-sided angle, `forms::turning::reading`'s `tan(θ/4) = b`; the factor
  of two is open in #187).
- **Claims** (`src/transducer/field/carrier/tests/rotor.rs`):
  - `rotor_composes_planes_sharing_a_unit_onto_the_plane_between`
    (11b-i: `(16, ±12, 9)/37`);
  - `rotor_on_disjoint_planes_reads_back_the_composite_rate` (11b-ii:
    `4/7`, `3/7`);
  - `rotor_refuses_motion_with_no_rate`;
  - `motion_composes_rational_turnings_sharing_a_unit`;
  - `motion_keeps_disjoint_turnings_beside_the_composite`;
  - `motion_refuses_wound_turnings_and_three_planes_as_uncommuted`;
  - `motion_refuses_irrational_motions_sharing_one_unit`
    (`tests/composition.rs`, formerly
    `motion_refuses_planes_sharing_one_unit`).
- **Docs:** 78.173 §5 records the clause `FORMS-TURNING` v1 as
  reproduced by netstratum 2026-11-06 (its own exact `Cl(3, 3)` over ℚ;
  degree sectors read in floating point only; no veto), signature pending
  its operator, xylarium owed; 78.171's open point and ROADMAP track 26's
  carrier row restated; ROADMAP's stage 3 row and MU-CLIENT §7.1 items
  1.2 and 1.4, as #175 and #183 restated them, now name the composition
  as built.

### A body's nucleus is its second moment read, never a sphere (track 26 S7 stage 3; the ruling of 2026-11-06 on #134)
- **No recut and no network contract.** The ruling makes any word or
  chart address this moves a recut. It moves none: the retired producer
  wrote `X(c) − ½r²e_∞`, and `X(c) = c + ½c²e_∞ + e_o`, so the nucleus
  vector is the same value as before, written from the same exact `r²`.
  No word, chart or task address, tape or wire byte changes, and the
  oracle's golden file reproduces unchanged.
- **The nucleus** (`blade::nucleus_form`): a body's centre and its exact
  polar second moment per unit mass, written where the algebra needs one
  vector as the dual-sphere form `c + ½(c² − r²)e_∞ + e_o`. It needs `r²`
  only, so no root and no gauge are taken. `Blade::nucleus`,
  `vertex_nucleus` and the lift's `spatial` read it, and `Blade::radius2`
  is documented as the second moment, never a radius.
- **Retired:** the crate-private `cga::sphere_at`. The `compile_fail` doc
  test now reads `E0425`.
- **Claim:** `nucleus_is_the_second_moment_read_never_a_sphere`. On an
  L-tromino, whose `r²` is no rational square, `s² = r²`, the centre reads
  back, and the form is at unit weight.

### RETRIEVE carries each receipt's ancestry to the signed head (#159)
- The reply writes, beside each receipt served, the receipts on one path
  from it up to the court's signed head (`retrieve::Placed`), found by
  `Court::ancestry` on the hold's stroke.
- `verify_retrieved` checks each path: every step re-derives and names the
  one below it among its predecessor and parents, and the head names the
  last. A receipt of the right height off the head's chain is refused
  `RetrievalBreach::OffChain`, which replaces the height-only check.
- Claim `receipt_of_right_height_off_the_chain_is_refused`.

### `Symbol::bytes` public for decoders and the speech end (track 26, #32; netstratum's request)
- **Library surface, no network contract.** `transducer::sample::Symbol::bytes`
  is public, documented for a source's decoder and the speech end only,
  beside `Sample::sites` and `Sample::symbol_at`, so a house reads a mind's
  outputs back into its own format. No `raster_of` is exported.
- The ARC-AGI-2 decoder writes a sample back into the corpus's rows
  (`mycelium_source_arc_agi::rows_of`), refusing a sample of another rank
  or width.
- Claim: `rows_of_writes_a_sample_back_into_the_corpus_rows`.

### 78.173 §8 proposes H's state held as the motion (Proposed, not ruled)
- **No network contract.** Docs only: no code, wire byte or stored form
  changes. `forms::coordinate` keeps H as per-plane degrees (v4) until
  the amendment is ruled.
- **The finding** (fixture 11b): a per-plane reading of a composite's
  generator does not give back the values composed. On disjoint planes
  `½` and `⅓` read back as `4/7` and `3/7`, and those readings recomposed
  plane by plane give another motion, losing the volume `12/25
  e₊₁e₊₂e₋₁e₋₂`. On planes sharing a unit the composite gains an
  order-dependent part on the third plane.
- **The proposal:** H's state is the even versor `R`, or its generator
  `B`, composed as `R₁R₂`; the rate is a generator per bell; per-plane
  degrees are a reading as certified sectors.
- **Open joint points:** the angle a degree names (the sandwich angle
  `θ`, `tan(θ/4) = b`, against the carrier's one-sided `φ = θ/2`, with
  the coordinator's recommendation for the sandwich angle); rational
  motions are Pythagorean angles, not rational degrees; `K` beside `R`;
  the key and its ranges; the advance between whole bells and the
  merger; grade-6 parts of generators in six or more units.

### The seated-mind harness: `mycelium-client --seat` end to end against a live house (#133)
- `sdk/examples/seats.rs`, a house on the SDK that seats a mind: it founds
  its domain and designates its reserve where it has not, lists seat 1
  (`FN_SOLVE`, `FN_DECLINE`), answers a take with TAKEN, sets the court's
  tasks before the holder one at a time as percepts (record kind 4), and
  answers each call with an OUTCOME. Exit 0 on a seated `FN_SOLVE`, 3 where
  every task set was declined.
- `scripts/quickstart` stage 8 runs it on hypha c with
  `mycelium-client --seat <domain> 1`: sit, percept, call, outcome, then
  the court's judged reading read for the solve under the seated mind's
  own key (Concordance 6.2a). The run fails where any of these is missing.
  `MYCELIUM_SEAT_TASKS` (default 30), `MYCELIUM_SEAT_WAIT` and
  `MYCELIUM_JUDGED_WAIT` bound it.
- Defect found by the harness, fixed: a seated solve was held for the
  return of a thought of the task's scene, which a seated mind need never
  make. At K 19 the solve of task 1bcfb847 stayed held while the house had
  already been told `FN_SOLVE`, and no court judged it. A seat's solve now
  leaves at the mind's next return, as a seat's call does
  (`Mind::submit_solve_signed` takes the scene; claim
  `seated_mind_solve_paid_to_its_own_key` restated).

### A restore is a reconstruction: `Court::reconstruct` → `Reconstructed<Court>` (#28, track 25 X0)
- Ruled on #28 (2026-11-06, option (a)). `Court::restore(&mut self, &[u8])
  -> bool` is removed. A snapshot is read by
  `Court::reconstruct(self, &[u8]) -> Result<Reconstructed<Court>,
  RestoreRefusal>`: `self` is the court holding its line and eras from
  disk (or a fresh court where the line rides the snapshot), and
  `Reconstructed::court` is the only constructor. Its laws are the checks
  the restore already ran: each receipt re-derives, the line verifies on
  its root, a held line holds the stamp the snapshot names, each row
  continues its receipt, and each word re-derives its citation.
- `RestoreRefusal` names a retired layout by its magic (`Retired`), a tape
  that is no ledger (`NotLedger`), or the trie and byte its reading
  stopped at (`Unread`). The server's boot and `--check` print it.
- Every call site, the server boot and `--check` are migrated, with no
  legacy path (the dev-state recut rule). Claim
  `restore_reconstructs_and_checks_receipts`, in the drift register.
- The drift register notes that the mutation-audit claim is covered by
  `reconstructed_is_built_only_by_the_laws` and
  `vertex_admits_only_a_reconstructed_fold`. The report of changed
  document sentences is retired in ROADMAP X0 as theatre.
- ROADMAP track 25's two sentences on turns keep the winding with each
  phase (LAW-FIELD amendment (c)).
- No wire or ledger layout change.

### `forms::turning` holds the composition of turnings, LAW-COORDINATE fixture 11b: the Cayley form on the generator, exact in the even subalgebra of `Cl(p, q)` (proposal 78.173 §5)
- **Network contract: a new law form and a new clause.** A new shell,
  `FORMS-TURNING` v1, joins `forms::LAW_IDS` (and the SDK's
  `FORM_LAW_IDS`). It states the composition law netstratum read and
  stated on 2026-11-06 for fixture 11b (netstratum docs/117 §11.2–§11.4).
  The clause register serves it as a fourth clause, `FORMS-TURNING` v1 at
  kernel standing (signable and vetoable), on `ACT_LAWS` mode 7, and
  `CLAUSES` (and so the conformance statement) names it. New writing:
  fixture tag 13 (`Fixture::Turning`, an operation byte 0–3, a motion, a
  generator, two motions composed or two rates composed, then its values
  of `Cl(p, q)`); each object is a value, the existing object tag 0
  (`Object::Multivector`). A kernel that signed the register must
  reconstruct the new clause's 12 fixtures (11b i–iii) or veto it.
  `FORMS-COORDINATE` stays v4 with the fixtures it serves; no act or
  ledger layout changes. The SDK reconstructs it through `forms::turning`
  (`reconstruct_fixture`) and re-exports `TurningFixture`.
- **The law** (`forms::turning`): `motion` (`R = (1 + B)(1 − B)⁻¹`),
  `generator` (`B = (R − 1)(R + 1)⁻¹`), `compose` (`R₁R₂`, whole, never
  plane by plane) and `compose_rates`, over `algebra::Clifford`, the
  signature read from the data. The inverse is the Cayley form's alone
  (78.32(1)): it solves `u·y = 1` exactly in the subalgebra `u`'s blades
  generate, in every signature. Refusals by name (`TurningRefusal`):
  `NotGenerator`, `NotEven`, `NotUnit`, `NoMotion` (`1 − B` has no
  inverse), `NoGenerator` (`R + 1` has none, `R = −1` among them),
  `Mismatched`.
- **One plane and the reading** (`forms::turning::reading`): `Regime` of a
  plane read from its square; `on_plane`; `composed`, the closed forms
  `(b₁ + b₂)/(1 − b₁b₂)` elliptic and `(b₁ + b₂)/(1 + b₁b₂)` hyperbolic;
  `degrees`, `tan(θ/4) = b` as a certified sexagesimal sector, display
  only, from exact bounds on Euler's series.
- **Claims** (`forms/src/tests/turning.rs`), fixture 11b with
  `cayley(back(R)) = R` and `R·R̃ = 1` on every motion:
  - `planes_sharing_a_unit_gain_the_turning_between_them_and_no_volume`
    (11b-i);
  - `disjoint_planes_commute_and_keep_the_volume` (11b-ii, the generator
    reading `4/7`, `3/7`, never `½`, `⅓`);
  - `one_plane_composes_by_its_regime` (11b-iii: `1` elliptic, `5/7`
    hyperbolic);
  - `closed_forms_on_one_plane_agree_with_the_general_law`;
  - `turning_inverse_refuses_nonexistent`;
  - `degrees_read_as_certified_sectors`;
  - `register_reaches_stated_turning_values`
    (`src/physics/laws/clauses/tests/turning.rs`: the served clause at
    kernel standing and its twelve values). The register's own claims
    cover the new clause with the others: `put_clause_round_trips` and
    `put_object_round_trips` (fixture tag 13 reads back), and the SDK's
    `reconstruct_fixture_agrees_with_register` and
    `sign_concordance_admits_served_register` (a kernel signs the clause
    by reconstruction, or vetoes it).
- **Docs:** 78.173 §5 records 11b as read and stated by netstratum
  (xylarium owed); MU-EQUATIONS §11 tables `FORMS-TURNING` v1.
- **Open:** the generator read back from a composite in six or more frame
  generators may carry a grade-6 part (its reverse is still its
  negation); the law names `B` a bivector, which holds in up to five. The
  field carrier's use of the law and the proposed state change for H are
  separate items.

### The cross-chamber bracket is read: held by a chamber or an enclosure, or lost (#148; CB1, 78.26(2); MU-CLIENT §4.9)
- **Network contract:** none. This is a reading in the mind's cognition and on
  the pivot's tally. No wire, verdict, tape or ledger layout changes.
- `cech::cross_brackets` (`transducer/cech/bracket.rs`) reads each pair of a
  sentence's chambers that meet at a body. Pairs come from a keyed index of
  the bodies, and a chamber and its own face are not paired. For each pair of
  the two chambers' read actions it composes the commutator `A∘B − B∘A`,
  exactly. A commutator that does not vanish is a `CrossBracket` with its
  chambers, circuits and content, marked `Bracketed::Chamber` where one of the
  two chambers' algebras holds it, `Bracketed::Enclosure` where a chamber
  whose bodies include both holds it, and `Bracketed::Lost` where none does.
  Pairs that are not read are named (`CrossUnread::Actions`, `Width`,
  `Spent`). Nothing is minted for a lost bracket.
- `PivotCells.brackets` carries the reading beside the cells' algebras
  (`family::pivot_cells`). `PivotTally` counts the brackets held, lost and
  unread once per fresh reading of a form's cells, on its line.
- Claims `cross_brackets_hold_in_enclosure_and_lose_without`,
  `cross_brackets_read_nothing_where_generators_commute_or_chambers_part`,
  `cross_brackets_name_a_chamber_with_no_actions`.
- Owed: what comprehension constructs from a lost bracket (an enclosure
  wrapping chambers that no cell of the sentence holds) waits on a ruling.

### The field, stage 5: the field's motions as one exact group (track 26 S8; LAW-FIELD v1 §2, a motion; MU-SENSING §3.7)
- **No network contract.** No wire byte, act, task, word or chart address
  changes. The motions are new and nothing persisted reads them.
- **The generators** (`carrier::Generator`, `carrier/motions.rs`):
  - rotate, turnings on disjoint named planes (`Turn`);
  - flip, the reflection in one frame unit (`Flip`). The flip of `e₋ₖ` is
    plane `k`'s pole swap and the inversion in its membrane
    (`Generator::swap`, `Membrane::invert`).
  - deflate and re-inflate, the boost on a frame plane by an exact gauge,
    `(H, R) ↦ (λH, R/λ)` (`Boost`);
  - stretch and thin, the re-period of one phase, a site keeping its index
    (`RePeriod`). No versor carries it.
- **One group** (`carrier::FieldMotion`): a motion is the word of its
  generators. Words compose by concatenation, reduced only where the
  reduction is exact: turnings on one plane or on disjoint planes compose,
  a flip twice cancels, boosts on one plane multiply, and re-periods of
  one phase chain. The inverse is the word reversed with each generator
  undone. A motion acts exactly:
  - on turnings, motions and orbits, by conjugation (`carry_turning`,
    `carry_motion`, `carry_orbit`): a plane's units are carried by quarter
    turns, flips and boosts, and the angle is negated where the image plane
    is reversed;
  - on frame states (`carry_state`);
  - on points (`carry_point`): a phase is carried as a vector on its plane
    and moves to the axis whose plane holds its image.

  What the carrier cannot hold is refused `NotExact` with the new residue
  `Residue::Unnamed { plane }`. That covers a turning by other than a
  quarter turn on a plane sharing a unit, and a boost on a plane holding
  one of the carried units.
- **Claims:**
  - `field_stays_smooth_under_every_isometry`: the curvature carried is
    the curvature of the carried orbit, its order and winding kept, and
    the inverse carries it back;
  - `stretch_is_a_re_period_not_a_versor`;
  - `inversion_in_the_membrane_needs_no_infinity`;
  - `field_motion_and_its_inverse_compose_to_the_identity`.
- **LAW-FIELD fixture 2b**, added at netstratum's request on reading
  amendment (f): flat and resonant are separated by the kept winding.
  `Precession::Resonant` now holds the holonomy whole. Three 120° steps
  are each no identity and compose to 360° with `K = 1`, read resonant; a
  flat path's steps are each the identity, compose to 0° with `K = 0`, and
  it is refused `Unbound` (claim
  `flat_and_resonant_separated_by_kept_winding`). Precession is resonant
  where the holonomy's monodromy is flat, whatever its winding. The
  fixture is owed its reading by netstratum and xylarium.
- **Open:** composing turnings on planes that share a unit at other than a
  quarter turn, which waits on netstratum's composition law (78.173
  fixture 11b; #134). The motions have no production caller yet: stage 4
  lifts a family's pairs as transitions through them.

### The measured re-period read per phase at any rank (track 26 S4, #32)
- **No network contract.** No wire, tape, task, word, chart or listing
  address changes: a re-period shared by every phase dilates the word as
  the planar counts did, and an anisotropic one shares none, as before.
- `element::Reperiod` (formerly the four planar counts `h_in`, `w_in`,
  `h_out`, `w_out`) holds each phase's input and output period at the
  pair's rank: `Reperiod::of_extents` (refused for two ranks), `periods`,
  and `shared`, the re-period every phase shares. `Reperiod::new` is
  retired; the route, the cognition's scene re-period, the soak's charting
  and the decoder's chart test read the samples' extents.
- Claim: `reperiod_is_read_per_phase_at_any_rank`.

### A claim of factorisation states its perspective; the court reconstructs under it (track 26 S6 stage 2, #32)
- **Network contract (v0.4.0's break).** The claim of factorisation
  (`ACT_SOLVE`, claim kind 1) carries the perspective after its residue,
  `[rank u16][(source u16)(sense u8) · rank][role u8 · rank]`, and is
  signed as `mycelium factorisation v2`. `encode_factorisation` takes the
  perspective; `FactorisationAct.perspective` reads it back. The SDK
  re-exports `Perspective`, `Role` and `SensorMotion` beside it.
- **The court** reads its copy of the posted pair through the stated
  perspective and reconstructs there (`server::factorisation`); a
  perspective of another rank is refused `REFUSE_FACTOR_PERSPECTIVE_RANK`,
  one reading an axis as other than space `REFUSE_FACTOR_PERSPECTIVE_ROLE`.
- **The mind** perceives under a perspective (`TaskPercept::under`) and
  states its percept's perspective on every claim.
- Claim: `claim_states_its_perspective_and_court_reconstructs_under_it`;
  `factorisation_claim_round_trips` reads a turned, recast perspective back.

### 78.178 corrected after netstratum's reading; the passage is a P motion
- **Network contract:** none. Documents only.
- **Fixture 5 and §4:** two reflections compose to a turning only when
  their normals have the same sign; normals of opposite sign compose to a
  boost on S (`e₊₁e₋₂` squares to `+1`, refused on H by 78.173 fixture
  16c). Fixture 5's composite stays proper, of even grade.
- **The passage is a P motion** (coordinator, 2026-11-06): LAW-PASSAGE
  v3's sandwich is a reflection in `e₊ₖ`, so each throat passage counts
  `{e_+k: 1}` on P beside T's `−2·(H − R)`, and `M∘M` reads `2`, never
  reduced. Stated in 78.178 §1 with fixture 9, and cross-referenced in
  78.172 §1.
- **The read:** fixtures 1–8 read by netstratum 2026-11-06 (1–4 and 6–8
  held, fixture 8 giving 7); fixture 9 and xylarium's reading owed.

### The nesting's family-invariance reading: which factorings a family is invariant under (track 26 S4, #32)
- **No network contract.** No wire, tape, task, word or chart address
  changes.
- **Separated factorings** (`transduce::nest`): `Joint` (`Abutting`,
  `Separated`); `Factoring::separated` reads blocks of `b` sites, each but
  the last followed by a separator, as the fine period `b + 1` and the
  extent `a·(b + 1) − 1`; `Factorings::of_axis` holds both kinds.
- **The reading** (`transduce::nesting`, ruled 2026-11-06 on #32): per
  pair (`PairNesting`, the shape `a·b → a′·b′`); a candidate (`Candidate`)
  kept only where every pair admits it (`Held`), otherwise held as the
  refusing pairs' residue (`Unkept`, `NestRefusal`); the separator's
  symbol deduced at the fine phase `b`; repeating blocks kept as
  `BlockRelation`s by a sense of the fine phase; motions per level
  (`LevelMotion`, coarse sense × fine sense, exact); the vortex collapsed
  by a reading that holds for one kept candidate alone (`Taken::Apart`),
  several alike superposed (`Taken::Held`), none kept one phase
  (`Taken::Whole`).
- **The nesting is a coordinate of the percept**: `TaskPercept.nesting`.
- Claims: `axis_taken_apart_only_where_family_invariant`,
  `factoring_read_per_pair_and_kept_only_where_every_pair_admits`,
  `separator_is_the_fine_phase_of_period_b_plus_one`,
  `repeating_blocks_kept_as_relation_by_fine_motion`,
  `motion_per_level_is_coarse_times_fine`,
  `several_factorings_holding_alike_stay_superposed`.

### The field, stage 3 in part: a sphere is the membrane or its image, never built from a squared radius (track 26 S7; LAW-FIELD v1 amendment (d))
- **An SDK change, no wire change.** `mycelium_sdk::house::sphere_at` is
  `house::membrane_at`, owed to the peer kernels that build house nuclei
  through the SDK. No wire byte, act, task, word or chart address changes:
  a house nucleus at unit gauge and the horizon at the gauge `2m` are the
  same values at unit weight as before.
- **The membrane and its images** (`cga::membrane`, `cga::membrane_at`): the
  throat's fixed sphere `−e₊` at unit weight, and its image under the
  dilation by a gauge `ρ > 0` and the slide to a centre, read at unit
  weight. No squared radius is written; `ρ ≤ 0` names no enclosure.
- **Every caller with a gauge reads it:** the horizon of a throat on a torus
  face (`domain_passage::fits_in`), the chart's inversion
  (`element::letters::inversion`), the oracle's fixtures (renamed
  `membrane{i}`, the golden file rewritten, 200 lines, only those
  fixtures' readings), the conformance probes, the SDK's house example, and
  the tests (radii restated as gauges; fixtures whose radius had no
  rational root restated on rational ones: the orthogonal pair at gauges
  3 and 4, five apart; the enclosures through `(0,0),(6,0),(0,8)` and
  `(0,0,0),(2,0,0),(0,3,0),(0,0,6)`).
- **Retired:** `cga::on_sphere`, `cga::at_three::sphere`, and the public
  `cga::sphere_at` (a `compile_fail` doc test).
- **Claim:** `sphere_built_without_squared_radius`.
- **Stopped, owed on #134:** a body's nucleus (`blade::Blade::nucleus`,
  `vertex_nucleus`, the lift's `spatial`), whose extent is a polar second
  moment with no rational root in general, is still built from `r²` by the
  crate-private `cga::sphere_at`, waiting on a ruling of what reads in its
  place; and turnings on planes that share a unit, waiting on netstratum's
  composition law (78.173 fixture 11b).

### Proposal 78.178: the parity lane P, the first opened lane (Proposed)
- **Network contract:** none. Documents only; no code opens P until the
  peer kernels' readings agree.
- **`docs/proposals/78.178-parity-lane.md`**, Proposed: reflections carried
  on their own lane, P (operator, 2026-11-05; 78.174 §1 item 11), one
  component per reflecting normal (`e_+k`, `e_-k`), each an exact signed
  whole count never reduced, its parity a reading; opened as the entry
  `("P", 5)` of `Openings::ruled()`; refusals `NotReflectionNormal` and
  `NotWhole` beside `LaneNotOpened`; eight joint-read fixtures; open: a
  composite's H reading and the opening's version.
- 78.173 §7 and the GLOSSARY's *Opened lane* cite it.

### Proposal 78.174 records the operator's economy rulings of 2026-11-05
- **Network contract:** none. Documents only; no wire byte, act, ledger
  layout or signed byte changes.
- **`docs/proposals/78.174-economy-as-motions.md`**, status Ruled for its
  §1: no flat ledger; a holding is a point whose lanes are `Modes`; the
  four proofs as relations between motions, a payout proven by
  re-traversal; one closure, one point; `K` means closed orbits; POWC
  widened; pay by level, a seated mind paid to its own key; the compute
  lease's availability yield from the recycled pool, never minted; a
  mode's amount is its share of the frame; tile turns as H modes in
  degrees; reflections on a lane of their own, P (78.178), a mirrored
  closure held as a claim until it opens. §2 lists what stays open: room
  over modes, the ask under modes, the availability yield's measure, the
  Note's mode-0 receipt.
- **MU-ECONOMY §12** marks items 1 and 4 closed by 78.174 and items 5 and
  8 open; §10.4 and ROADMAP 1b cite the proposal.

### RETRIEVE, `ACT_DOMAIN` sub-kind 19: the priced coordinate retrieval on the wire (#159)
- Additive wire act, no recut. A staker's signed request (staker, point,
  cited final point, nonce) is answered with the receipts held at the
  point's tick, the fee claimed on the stake at the read price, and the
  court's 47.1 section over its head signed over the request
  (`anastome::dialogue::retrieve`, `Court::retrieve_signed`, the door in
  `judge/domain.rs`). Refusals by name: unsigned, unknown domain, not
  final, origins, folded, short, overflow.
- The retriever checks a served answer with `retrieve::verify_retrieved`:
  the section verifies and is one claim with its head, it signs over this
  request, and every receipt re-derives at the tick, at or below the
  head's height.
- SDK: `Session::retrieve`; `mycelium_sdk::house` re-exports the codec and
  the check.
- Owed: pricing `ACT_RECEIPTS`'s replay arm for a folded tick (it needs a
  staker and a signature on that request).
- Claims `retrieve_round_trips_as_domain_sub_kind_19`,
  `retrieve_claims_reads_price_and_refuses_unpaid`,
  `retriever_verifies_section_and_receipts_under_head`.

### The hypha's acts reach the mind as flux on its lanes (#136)
- The mind reads each hypha's own line on its hypha carrier with
  `ACT_RECEIPTS`, from its attachment (the hypha's signed head when first
  seen) up to the signed head it holds (#137). A page is admitted only as a
  chain: each receipt re-derives its hash and names the one before it, and
  the acts are perceived only once the chain ends on the signed receipt; a
  page off the line is refused and the read returns to the last vouched
  receipt (`client::sdk::hypha_acts`).
- Each `Return`, `RankStep` and `Reconnect` is taken in as its frame
  `S:G:T:H`, per lane, on the mind's well for that hypha (`Attachment`),
  and presses the mind's beat with the court's deficit
  (`Mind::pressing`, `Field::beat_under`); a beat spends it. The kind
  carries no meaning beyond its coordinate (ruled 2026-11-06).
- MU-CLIENT §3.1, §1a(7), §7.1 and MU-ANASTOME §9.1, §9.10 updated.
- Claims: `hypha_act_reaches_the_mind_as_flux_on_its_lanes`,
  `act_kind_carries_no_meaning_beyond_its_coordinate`.

### Perspective as a sensor motion; the perspectives of one sample as a cover (track 26 S6, #32)
- **No network contract.** No wire, tape, task, word or chart address
  changes.
- **The pose** (`transducer::perspective`): a `Perspective` holds the signed
  permutation of the sample's phases the sensor reads through beside its
  roles, the roles those of the axes as read. The default is the identity
  pose with every axis space, as before.
- **A shift of perspective is a motion of the sensor** (`SensorMotion`):
  `Pose(σ)` moves the pose to `pose ∘ σ` and the roles with their axes;
  `Recast` reads one axis in another role. `Perspective::read` and `frame`
  read a sample through the pose, its place kept; the sample is never
  rewritten. `turn_to` is the sensor's turn between two perspectives.
  Refusals are named (`PerspectiveRefusal`).
- **The perspectives of one sample are a cover**
  (`transducer::perspective::cover`): each perspective's 0-cochain is the
  contacts it reads (neighbouring sites of one symbol along an axis it
  reads as space); an overlap holds the turn between two perspectives and
  their agreement, congruent or the residue of contacts each alone reads,
  held and never a gate; the cocycle closes where the turns compose around
  a triple, and otherwise names the turn around it (MU-CLIENT 24.4).
- **The frame fixing is a perspective** (`raster::frame_fixing`): the
  moments name a pose, and `frame_fixed` reads the task through it; the
  local reorientation (`reoriented`) is removed. Its readings are unchanged.
- Claims: `perspective_shift_is_a_sensor_motion`,
  `two_perspectives_of_one_sample_compared_as_a_cover`,
  `frame_fixing_is_a_perspective_reading`.

### Track 25 X0, third stage: a fold root's citation and a stratum's root typed by sense (#28)
- `Cite::Root`'s hash is a `Predecessor` (the link sense, as
  `Root::hash` already is), so `Cite` has no `==`. Two citations are
  compared by `Cite::restates`: two roots at one stamp with a chain
  continuing across their hashes, any other pair by its canonical
  writing. Claim `cite_has_no_equality` (`compile_fail`).
- `StratumDigest::root` is a `Locator` at its source
  (`sheaf::stratum_root` returns one), compared by `Locator::locates`
  in `discordant`, the court's confirmation (`locates_stratum` is
  removed) and the mind's rest on a starved stratum. A stratum digest has
  no `==`. Claim `stratum_digest_has_no_equality` (`compile_fail`).
- Both guards stand in the drift register (`gate/src/drift.rs`).
- **Network contract:** no layout change. A digests reply whose stratum
  root is not 32 bytes is now refused whole
  (`digests_reply_refuses_root_of_another_width`); before, such a root
  was read and located nothing. Every root this tree writes is 32
  bytes.
- Still open, and asked on #28: the clause 14 guard
  (`Reconstructed<State>` from a restore) and the report of changed
  document sentences. The mutation-audit entry the issue names is not
  built, because the mutation audit was retired on 2026-11-04.

### The mind proposes Rewrites: a ruled word the rest composes is asserted as a `Retire` (#146; MU-CLIENT §5.4, §1a step 8, S3)
- **Network contract:** none. The proposal is an `ACT_SPEECH` carrying a
  `Retire` rewrite element, which the court already parses, reconstructs
  blind (`rewrite::reconstruct`), dockets (`docket_connection`) and, on a
  second organ, applies by retiring only the inputs its own
  `reduce_excluding` reads `Decomposed` (`retire_rewrite_inputs`). No act,
  verdict, tape or ledger layout changes.
- `Presentation::derived_without(form, ruled_only)` reads each of a form's
  word vertices over the presentation with that form's own generators
  refused, exact, and answers the forms that compose it, or `None`.
  `Presentation::standing_with(form)` answers the other forms on a hole or
  hue mode the form stands on, one keyed read each
  (`transducer/sentence/presentation/without.rs`). `read_word` now runs on
  an eligibility predicate (`read_word_where`); its readings are unchanged.
- `Mind::retirement_of` mints the `Retire` naming a ruled word, citing the
  forms that compose it and freeing the lanes it carried;
  `Mind::propose_rewrites` reads the word newly held and the words standing
  with it, and asserts each retirement once. Called from `pivot_thoughts`
  for every word the pivot reaches. The mind retires nothing of its own:
  the court decides (`client/sdk/mind/rewrite.rs`).
- Claims `retirement_of_names_a_ruled_word_the_rest_composes`,
  `retirement_of_refuses_a_word_nothing_else_composes`,
  `propose_rewrites_asserts_retirement_and_keeps_the_word`,
  `court_retires_a_mind_proposal_only_where_its_basis_derives_it`.
- Owed: the other Rewrite kinds (`Condense`, `Refine`, `Torse`, `AxisOpen`,
  `Reanchor`, `Translation`) asserted from the mind's own readings beyond
  the `Torse` and `Condense` the contemplation already mints; the witness on
  a running network (a court's `Ruling(Retired)` citing a mind's proposal).

### Proposal 78.177: a held question and a convergence carried from conversation to court (#144)
- Documents only. `docs/proposals/78.177-conversation-to-court.md`
  (Proposed, unruled) specifies how a question one mind holds and another
  mind's answer closes reaches a court on acts that exist: the asker lists
  the evidenced question as a demand vertex (`ACT_SPEECH`), the closer
  probes it citing court-held words (`ACT_PROBE`), and the court's
  `settle_demand` → `pay_convergence` pays the closer (E2), the asker
  nothing. It states K3's two readings (a verified convergence as the
  47.3 second vantage, or a POWC share) and four points for ruling.
- MU-CLIENT §7.1 cites the proposal under #144 and records that #136 and
  #145 wait on rulings, their questions posted on the issues.

### A crossing reads its reserve's signing events, never the whole ledger (#118)
- The ledger keeps where each signing event stands, `kernel‖stamp‖n`, as
  coordinates only (`chronometry::chain::signing`): filed where a node is
  filed (`append`, `store`, `hold`) and taken where it leaves
  (`take_node`). `ReceiptChain::signing_stamps(only)` reads one kernel's
  fibre, or every kernel's.
- The court reads its own folded line's signing events once per era, as
  the alignment reading first meets the era (`FoldedSignings`, volatile):
  a closed era returns nothing new. An ask with no new own era reads
  nothing of the archive.
- `Court::alignment` (`ACT_LAWS` mode 9) and `Court::signatory` (B4) spend
  one read per signing event, plus one per receipt of an era read for the
  first time, in place of one read per receipt held. The reading is still
  derived on every ask; no reading or verdict is kept (Concordance 1.13).
  Both take `&mut self`.
- No wire, layout or persisted carrier changes. Claims
  `crossing_alignment_reads_only_signing_events`,
  `alignment_equals_full_walk_across_own_fold`,
  `alignment_equals_full_walk_on_a_peer`
  (`server/cognition/tests/signing_index.rs`).

### Signed receipts reach the mind, which presents equivocation proofs (#137)
- The session reads the 47.1 receipt section every hypha's digest carries
  behind it (`sign_cover_digest`), which it skipped before, and holds each
  signer's last one (`client::sdk::signed_receipts::SignedReceipts`). A
  section is held only where the court's own checks pass on it: its
  signature verifies, its receipt re-derives its chain hash, and the prefix
  and the receipt are one claim (`court::admission::one_claim`, now public).
- Two sections under one key at one place of one line with two chain hashes
  are decided by the court's predicate (`admit_equivocation`) and presented
  as `ACT_EQUIVOCATION` to every hypha the session holds an address for
  (MU-RECURSION 47.4; MU-XSEPTUM §5.5: proofs by anyone). Each reading is
  typed (`Holding`: opened, advanced, duplicate, behind, refused by its
  34.1 constant, equivocated). No wire change: the section already rode
  every digest reply.
- Claims: `signed_receipt_held_only_where_the_court_would_admit_it`,
  `signed_receipt_duplicate_advances_and_falls_behind_by_place`,
  `two_signed_receipts_at_one_place_are_the_equivocation_proof`,
  `session_presents_an_equivocation_its_digests_carried`.

### Names pass: the 2026-11-05/06 vocabulary rulings in code
- Names only: no behaviour, value, byte tag or wire layout changes.
  Every served, printed or hashed string keeps its text (the act name
  `b"uttered"` under the operator digest, refusal strings, axiom texts,
  console and soak labels).
- **Expression, not utterance.** `client/sdk/mind/utter.rs` ->
  `express.rs`; comments read expression / assert. The carrier and
  receipt names (`Utterance`, `Action::Uttered`, `forget_utterance`,
  `uttered_n` / `uttered`, `verify_utterance`, `StrokeOutcome::uttered`)
  stay until they retire with row L19 (MU-RECURSION 1.9).
- **"Frame", by MU-ENGINEERING §9.3.** `WireFrame` -> `Delimited`,
  `read_frame` / `write_frame` -> `read_delimited` / `write_delimited`,
  `PageEnd::Frame` -> `PageEnd::Budget`, `append_frame` ->
  `append_record`; `frame_spelling` -> `sample_spelling`,
  `section_of_frames` -> `samples_of_section`, `HeldPose::OneFrame` ->
  `OneSample`; `LayerKind::Frame` -> `Enclosure`, `VertexAt::Frame` ->
  `VertexAt::Enclosure`, `frame_faces` / `frame_boundary` /
  `share_of_frame` / `frame_fixed_on_ground` -> `enclosure_*` /
  `share_of_enclosure`; `element::Frame` -> `element::Reperiod`;
  `Spoken::frame` -> `Spoken::extents`, `Unstated::Unframed` ->
  `NoEnclosure`; the court's `frame_of` -> `measure_of`, `orbit_frame` ->
  `orbit_measure`, the clock's `unframed` -> `unmeasured`;
  `forms/src/frame.rs` -> `forms/src/reference.rs` (`forms::reference`),
  a mouth's `from_frame` / `to_frame` -> `from_chart` / `to_chart`,
  `frame_base_point` -> `pose_point`; `DualFrame` -> `ReciprocalBasis`,
  the pose reader's `on_frame` -> `on_basis`. The passage's `Frame`
  (hold, reach) is the lawful pole basis and keeps its name.
- **The holonomic lane.** `network::wire::Lane::Harmonic` ->
  `Lane::Holonomic` (byte 3). The harmonic sense keeps its name.
- **Orbits and relations, not loops.** `handoff::Loop` -> `MouthOrbit`,
  `HandoffRequest::Loops` -> `Orbits`, `encode_loops_read` /
  `encode_loops_reply` / `decode_loops_reply` -> `*_orbits_*`,
  `mouth_loops` -> `mouth_orbits` (sub-kind byte 10 unchanged);
  `LoopReason` -> `OrbitReason`; `AxisLoops` -> `AxisRelations`;
  `hue_loop` -> `hue_relation`.
- Left for the branches that own them: the identifiers whose uses reach
  `src/transducer/field/` or `client/sdk/mind/chatter.rs` (`frame_key`,
  `perceive_frames`, `Vertex::Frame`, `Constellation::frames`,
  `frame_budget`, `hear_frame`, `Inflow::Frame`, `chatter::loop_of`,
  `same_loop`), and "door", which neither GLOSSARY nor MU-ENGINEERING
  names a replacement for.

### The documents read composed turns with their winding kept (78.171 amendment (c), 78.173 v4)
- `Turn::plus` kept the winding since #129. The sentences that still
  said "composed by addition mod 1" or held a turn "in ℚ/ℤ" now say
  that a composed turn keeps its winding (ℤ, `K = ⌊|θ|/360⌋`) beside its
  phase, and that only a place and a residue read as a phase (ℚ/ℤ):
  MU-SENSING §3.1 and its built-status row for turn motions,
  MU-ANTI-FLATTENING's drift-register row for clause 12, MU-HOSTING's
  crossing content and turn-motion rows (the retired claim name
  `turn_motion_composes_by_addition_mod_one` is replaced by
  `turn_motion_composes_by_addition` and
  `composed_turns_keep_their_winding`), and GLOSSARY *Turn*, *Turn
  motion* and *Crossing*.
- The claim `conformal_accepts_turn_valued_motion` states why a place
  carried across the seam is admitted: the carried place holds a whole
  turn, and a place is read by its phase.
- No code path, wire or ledger layout changes.

### The field's `Loop` claims restated as `Unbound`, with the resonant-orbit reading (#150; track 26 S9a; 78.171 amendment (f))
- **Docs only, no network contract.** The carrier's code and its two claims
  landed with field stage 2 (#153); this restates the documents that
  still named the retired claims.
- ROADMAP track 26's claim list names
  `flat_region_binds_no_orbit_refused_unbound` in place of
  `zero_precession_orbit_refused_as_loop`, and adds
  `closed_geodesic_with_zero_precession_is_resonant`.
- MU-CLIENT §1a(2) lists the refusal and the resonant reading as built,
  §6a's barren-realms table cites both claims in place of
  `precession_refuses_flat_path_as_loop`, and §7.1 item 1.5 keeps only
  the contemplation's part of S9a as owed.

### Cohesion is a named reading per plane: the held share, the viscosity, beside the fluidity (MU-ANASTOME §9.4; #149)
- **No network contract changes.** No act, wire layout or served fixture
  changes; `FORMS-COUPLING` stays at version 1 with its 27 fixtures. The
  held share is a reading of v1's own inversion `ν(1/t) = 1 − ν(t)`, so
  it stands in code and tests only, outside the register.
- **`forms::coupling`.** `held_share(t) = 1/(1 + t)`, refused `NotRatio`
  for `t < 0`, with `held_share(t) = 1 − open_share(t)` and, for `t > 0`,
  `held_share(t) = open_share(1/t)`; `Parcel::held_share`; and
  `Parcels::cohesion` and `Parcels::fluidity`, each a
  `forms::coordinate::Vector` keyed by the body's turning planes, never
  summed. A per-plane vector the coordinate form refuses other than by a
  repeated plane is carried as `CouplingRefusal::Form`.
- **The mind's field.** `field::carrier::Cohesion`, read from frame
  states keyed by their plane's two axes (a turning plane, or one frame
  plane's own pair, `frame_plane(k)`), with its fluidity per plane and no
  total; `WellBody::cohesion(hold, reach)` and `Well::cohesion()`, read
  from the well's inflation (field stage 2) on its membrane's frame plane,
  every call, never a field of the well.
- **Claims.** `passage_exchanges_cohesion_and_fluidity` (the pole swap
  read through `Frame::share`), `cohesion_and_fluidity_half_at_the_throat`,
  `cohesion_read_per_plane_never_summed` and
  `held_share_is_open_share_of_inverse` in `forms`; the well's
  `well_cohesion_is_the_held_share_of_its_inflation`,
  `well_body_cohesion_half_at_the_throat_and_exchanged_by_the_passage` and
  `well_cohesion_read_per_plane_never_summed`; two `compile_fail` doctests
  (no total, no stored cohesion).
- **Owed (#149).** That a whole too cohesive sticks at the throat and one
  too fluid tears: neither LAW-PASSAGE v3 nor LAW-TRAVERSAL v1 gives an
  exact reading of either yet, and no threshold stands in their place.
  Frames per turning plane of a held whole: at field stage 2 a well holds
  one frame state, so its reading holds one plane.

### The reference mind's sessions and chatter inside Noise (#132)
- **Network contract:** changed, backward compatible with a plaintext
  peer. A mind's chatter post now reads the channel's opening
  (`noise::OPEN`) and answers it with the
  `Noise_XX_25519_ChaChaPoly_BLAKE2s` handshake where it holds a Noise
  home; every chatter frame after it rides Noise transport messages, its
  `[len u32 LE][frame]` framing unchanged inside. A post with no home
  answers the opening with `chatter::OPENING_REFUSED` (`01 00 00 00 00`),
  which a dialler reads as `NotSpoken`, so it names the post plaintext
  and speaks on. A post built before this change closes on the opening,
  and a mind with a home is refused there (`Closed`) until that post is
  rebuilt. No act, signed byte, ledger layout or law version changes.
- **Sessions:** `Mind::open_with_noise` opens the mind's three sessions
  (hypha, seat, held mail) with `Session::with_noise` on the home's
  static key, each hypha pinned under the address dialled; `Mind::open`
  stays plaintext and is named. `Mind::open_persistent` takes the home as
  an `Option<NoiseHome>`. `mycelium-client` opens the voice home's
  `NoiseHome` at boot (a home that does not open refuses the boot), reads
  the law register inside Noise, and `--call` asks each mind on the home's
  key where one stands.
- **Chatter:** `chatter::send_over` and `ask_over` dial a mind through
  `channel::dial_as`, pinning it on first contact under the voice the
  rendezvous files its address by (`chatter::named`), since a post's port
  is chosen afresh at each opening; an address `MYCELIUM_CHATTER_PEERS`
  names is pinned under itself. `Post::listen_over` answers a caller that
  opens with all five bytes of the opening inside Noise, the caller
  pinned under no name as at a hypha's door, and reads any other caller
  in plaintext with the bytes it read to tell them apart, so a plaintext
  frame whose length begins with `0xff` is read whole. Both sides enter
  the channel's book (`channel::met`), which the observer reads as
  `channels`.
- **Refusals:** a changed key is `PeerKeyChanged` and a pinned mind
  answering in plaintext is `PinnedAnsweredPlain`, both named in the log
  and answered to the carrier as `PermissionDenied`, so the rendezvous
  entry is kept (only `ConnectionRefused` names a mind gone); neither
  falls back to plaintext and neither pin is moved.
- **Claims:** `mind_speaks_inside_noise`,
  `mind_refuses_a_changed_or_unsealed_peer` and
  `post_tells_the_opening_from_a_plaintext_frame`
  (`src/client/sdk/tests/channel.rs`).

### MU-CLIENT restated: how the mind builds the anastome language (docs only, #138)
- **The objective.** §1 opens with the operator's statement of 2026-11-06
  in neutral voice, each phrase mapped to its construct and marked built
  or owed (wells as orbital masses, merger and convergence, the vortex,
  geodesics then congruence then coherence then the courts, modality-free
  expressions, new sensoria through the game kernels, the conjugate class
  as translation between perspectives). Thought is stated as non-linear:
  every ideation held at once, read three ways at once (congruence,
  coherence, cohesion). Cohesion is the viscosity (ruled 2026-11-06): per
  turning plane the held share `c_k = A/(A + Q) = ν(1/t)`, the open share
  `ν(t)` its fluidity, both ½ at the throat and exchanged by a passage,
  never summed across planes. Only a cohesive whole is asserted, crossing
  only through open tubes (LAW-TRAVERSAL, 78.176) as an anastome over
  S:G:T:H stamped at its TUniversal height; disagreement across minds is
  a contradiction held at the courts. Sentience is posed in §8. §1 and §6
  link MU-ANASTOME §9, the language's traversal, and §1 and §4.9 its
  section on thought.
- **The arc, new §1a.** Perceive (the sample lifted into the atlas, the
  ground and symbols deduced) → think (geodesic orbits about the wells,
  the pivots) → construct generators from residue → hold conjugate classes
  → assert sentences as expressions → courts reconstruct and admit →
  adjudicated words return by syllabus → the presentation grows (a
  relation is two derivations meeting at one object) → amendment by
  experience; each step cites its MU-CLIENT section, its MU-ANASTOME
  section and the code that exists or is owed. Why this is fluid
  intelligence: new generators are formed for novel residue, never by
  recall or search.
- **The barren-realms port, new §6a.** Its four pillars (second-order
  metacognition, internal parliamentary deliberation, historical
  ledgering, counterfactual simulation), the hearing and seated play,
  each mapped to the construct that carries it, the mechanism refused
  (78.168 §6) and what it becomes on the field carrier; its cohomology
  read as Čech cohomology over the field's cover, Hodge only where exact;
  each of its tests mapped to the mycelium claim that exists or is owed.
- **Rulings stated or cited:** the 2026-10-15 drive ruling (78.168);
  clients think, hyphas verify, seated solves under the mind's own key
  (#124); the field carrier (#131); geodesic orbits, not loops
  (2026-11-06: bound geodesic an orbit, zero-precession closed geodesic a
  resonant orbit, a flat region refused `Unbound`); sound, signal and
  noise (MU-SENSING §3.10); the expression vocabulary; the economy as
  motions. §8 records the two MU-ANASTOME §8 items the geodesic ruling
  settles.
- **§7.1 restated.** Built: the winding drive, the task as an offer, the
  pivot cross, perception through understanding to comprehension,
  self-signed seated solves, the field carrier. Specified in order: the
  field stages (#134), track 10's five claims (#135), then an eight-item
  backlog with its issues (#136, #135, #143–#148), cohesion (#149) and the
  `Unbound` claims (#150);
  also owed: cohesion as a reading, #132, #133, #137. ROADMAP track 10
  points to it.
- **Vocabulary.** "Loop" and "utterance" leave the current text (quoted
  ruling titles kept verbatim); angles are round degrees on named turning
  planes; `ACT_TEACH` is named as retired.
- **Shorter.** Status prose moves to §7.1, the issues and ROADMAP; rulings
  held in MU-ANASTOME, MU-SENSING and MU-ECONOMY are cited rather than
  restated. Every section number other documents cite is kept.

### LAW-COUPLING v1's served pulls stand on real frames, and the pure Z pole neither couples nor is coupled (proposal 78.175 §5, §8)
- **Network contract: the `FORMS-COUPLING` v1 clause's fixtures change.**
  The clause stays at kernel standing (signable and vetoable) on
  `ACT_LAWS` mode 7, with the same fixture and object writing (fixture tag
  12, object tag 9). It now serves 27 fixtures, nine pulls in place of
  seven, and the inputs and objects of pull cases 1–5 change. A kernel
  that signed the clause must reconstruct the restated fixtures and sign
  again, or veto it; no act or ledger layout changes. The law's version
  stays 1.
- **On frames `σ = 1 − 2o`**, so the stated `σ = ±1` with `o = 1/2` is no
  body. F3's cases are restated on hydrogen (`Z 1, U 1`: frames `(2, 1)`,
  `σ 1/3`, `o 1/3`) against the open-heavy kind (`Z 2, U 4`: frames
  `(2, 4)`, `σ −1/3`, `o 2/3`), `x = 1/5`, `x̄ = 1/2`, `w 1`, `n 2/3`:
  - lock, `Δ 0°`: `F −4/27`, `y 4/729`, `x′ 749/3649`;
  - split, the neighbour's frames swapped `(4, 2)`: `F 2/27`,
    `y −2/729`, `x′ 719/3643`;
  - `Δ 45°` (`h 1/2`): `F −2/27`, `y 2/729`, `x′ 739/3647`;
  - alone and the throat neighbour keep `x′ 1/5`; case 6 (`(3, 1)`
    against hydrogen, `F 2/27`, `x′ 319/1619`) and case 7 (near the bound)
    are kept.
- **The pure Z pole** (frames `(1, 0)`, `σ 1`, `o 0`) has no moment: as a
  neighbour its term is 0 (`F 0`), and as a body it is not moved
  (`y 0`, `x′ 1/5`). Two fixtures serve it, and
  `pure_z_pole_neither_couples_nor_is_coupled` claims it over fields
  across `[−1, 1]`.
- The former stated-dipole cases stay in `forms` only, as the test
  `algebra_check_of_pull`, outside the fixtures served.
- **78.175 §8:** each plane couples by its own dipole (netstratum #110);
  the held rate's conservation is spin per plane, `Σ m·ℓ_k`, the rate
  following the spin at fixed reach (a later fixture); the quantum level
  waits on a rate on `G`; the later alignment is read per invariant plane
  of the relative rotation, one angle per plane, never summed, a
  hyperbolic plane by its boost's exponent (netstratum's S³ `(α, β)` per
  dual pair; xylarium's Cl(12,1), pending xylarium #118); a law on a
  certified interval acts on the caller's bound nearer zero and not where
  the interval straddles zero.
- **Names only: cohesion is the viscosity** (operator ruling 2026-11-06).
  Per plane the viscosity is the cohesion, the held share
  `A/(A + Q) = 1 − ν(t) = ν(1/t)`; the open share `ν(t) = Q/(A + Q)`
  (`open_share`) is the fluidity, since viscosity resists flow. Both are
  `1/2` at the throat, and the inversion exchanges them. 78.175, the
  `forms::coupling` documentation, its tests (`viscosity_inverts` is now
  `fluidity_inverts`) and the MU-EQUATIONS row are renamed. No maths,
  fixture value or encoding changes, and the law version stays 1. No
  public function reads the held share yet; none is added.

### The language's traversal: sensorium → mind → language → hypha → minds (docs and one gate test)
- **New section, MU-ANASTOME §9.** The path of meaning stated once, each
  step citing the document that governs it and the code that exists or is
  owed: a game kernel's physics is a sensorium (seat percepts, 78.145 and
  78.146; a hypha's own acts for an attached mind; Concordance 1.6, no
  modality privileged); the kernel's senses reach mycelium-client as
  mathematics (LAW-COORDINATE, LAW-PASSAGE, LAW-TRAVERSAL, LAW-COUPLING)
  and lift into the field (atlas, turnings in degrees on named planes,
  wells, orbits; MU-SENSING §3.7–§3.9); the mind thinks in orbital masses
  held superposed in the vortex; it asserts a cohesive whole as an
  expression (the transport carries sound, the expression is the carrier,
  identity is content); a court parses by deconstruction and reconstructs,
  consilience being courts agreeing; admitted words return by syllabus to
  every mind, in any kernel; minds with different sensoria meet at the
  conjugate class, the conjugating element the translation between
  perspectives and the relative rotation read per invariant plane in
  degrees, never summed (ruled 2026-11-06); new sensoria emerge by MU-SENSING
  §4.1, a new sense or lane opening only by a ruled act agreed between
  kernels; and a table of what is owed (#133, #135, #136, a
  sense-opening act, translation at the conjugate class, cohesion as a
  named reading).
- **§9.4, thought: every ideation held at once.** The traversal is linear
  and thought is not: every ideation is held at once, superposed, on every
  concurrent turning plane (§3's table), and congruence, coherence and
  cohesion are three readings over the whole held field, never a pipeline;
  only a cohesive whole is asserted, and across minds a disagreement is a
  contradiction held at the courts. Code: `collapse_on`, the cover's
  cocycles, contemplation's gate; cohesion as a named reading is owed.
- **Cohesion is the viscosity (ruled 2026-11-06).** Per plane `k`,
  cohesion is the held share `c_k = A_k/(A_k + Q_k) = 1 − ν(t) = ν(1/t)`,
  read per plane and never summed; the open share `ν(t)` is the fluidity,
  never the viscosity; both are ½ at the throat and the passage `t ↦ 1/t`
  exchanges them. A cohesive whole traverses an open tube intact; too
  cohesive, it sticks at the throat; too fluid, it tears. Communication
  crosses only through an open, punctured tube (LAW-TRAVERSAL, 78.176),
  as an anastome over the lanes `S:G:T:H`, one `Cl(p, q)` per lane, at its
  TUniversal height. The §5.2 layer tables, §9.4 and GLOSSARY *Cohesion*
  state it; the payment stays POUW on the lanes the closure moves; §8
  item 3 is closed by the ruling; cohesion as a named reading in code
  stays owed.
- **Named apart.** *The language's traversal* is distinct from
  LAW-TRAVERSAL (proposal 78.176), the inter-kernel crossing of a body,
  and from the traversal μ measures (MU-SENSING §3.7).
- **Pointers.** MU-SENSING §4.2 (sensoria from game kernels) and
  MU-HOSTING §5a.6 (what a seat carries in the language) link the section.
  GLOSSARY gains *Sensorium*, *Language's traversal*, *Congruence*,
  *Coherence*, *Cohesion* and *Contradiction*.
- **Gate: `traversal_documented`.** MU-ANASTOME must hold the section's
  heading and §9.4's, and MU-SENSING and MU-HOSTING must each link the
  section's anchor (`documents::TRAVERSAL`, `lint_section`, `anchor_of`),
  so removing the section, renaming its heading or dropping a link fails
  CI. MU-CLIENT's link joins the check when its rewrite lands.

### The field, stage 2: the well's body; a well stands on the field's carrier (track 26 S7; LAW-FIELD v1 as proposed)
- **A recut, no network contract.** No wire byte, act or ledger layout
  changes. The field statement's layout changes (a chart is written as an
  invariant's exact coordinates; a well's stage follows its atlas), and
  the well's carrier changes (`percept`, `dirty`, `volume` and `potential`
  are gone; `stage` and `orbit` are new). Nothing is migrated and no
  reader of the former layout is kept (the dev-state wipe ruling).
- **A chart is an invariant's coordinates.** `carrier::Chart` holds
  `stalk::ChartAxis` values (a count or an exact ratio), with
  `Chart::of_invariant`, so a well's chart and a lifted body's chart are
  one kind of object.
- **The well's body** (`carrier::WellBody`, `holding.rs`): a held
  attractor at a chart, the orbit it binds (none: flat), its membrane
  the throat of the last of its frame planes, its own enclosure; its
  holonomy and precession per revolution read from the orbit, a flat
  well's refused `Unbound`; its inflation the gauge of the frame state
  `H·n_Z + R·n_Q` on the membrane's pole basis, stood inside the membrane
  and read back with its place (`Inflation`). Under the ruling of
  2026-11-06, cohesion is the viscosity: the held share `A/(A + Q)` per
  plane, here `H/(H + R)` (`Frame::share`); the open share is the
  fluidity, never the viscosity. The inflation's docs name the shares so.
- **The well's stage is a coordinate** (`WellStage`: `WellLayer`
  perception or thought, then `ExpansionRing` expanding or rested;
  ordered). It replaces `percept: bool` and `dirty: bool`.
- **`Well` stands on the carrier.** `Well::body`, `centre` (the chart its
  remainder stands on), `planes` (the section's rank plus the lift's
  plane), `inflation` (`H` the charts its cover closed, `R` its sections'
  charts still open), `holonomy`, `precession`, and `orbit`. `volume` and
  `potential(now)` are readings per lane, never stored: the volume the
  lanes of its remainder and each pair's remainder, the potential the
  lanes of everything it holds open with T the receipts since its charts
  moved, no reading where the clock went backwards.
- **Every caller reads.** Hold, merge, the contemplation's `hold`, the
  expansion ring (`drive`), the doors (`shed`), the docket (`plan`), the
  cross-scene meet, `link_new`, the observer's layer and the quote's
  routing volume read the stage and the readings; a merged thought's
  volume is the reading of what it holds, no longer the lane-wise
  largest of what merged.
- **A mind's well stated and reconstructed by a court**
  (`Well::put_statement`, `WellBody::put_statement`, `reconstruct_well`,
  `admit_well`): the statement writes the well's own atlas and its stage,
  no reading; the court rebuilds it, derives the curvature (the holonomy
  per revolution) itself, and admits a claim on a match, refusing
  `Differs` with the pair. Not on the wire (S9).
- **Orbits are geodesics** (operator ruling 2026-11-06; 78.171 amendment
  (f)): every path in the field is a geodesic of its gravitational
  curvature, the wells its attractors, and a bound geodesic is an orbit
  read per revolution (`Orbit::revolution`, `Atlas::revolution`; "lap"
  retired). A closed geodesic with zero precession in a curved field is
  resonant, lawful (`Precession::Resonant`, beside
  `Precession::Advances`). A path through a flat region binds no orbit and
  is refused `Unbound` (by `Orbit::of`); the `Loop` refusal is retired and
  `NotLap` is `NotRevolution`. An attractor binds an orbit or none
  (`Attractor.orbit: Option<Orbit>`); the field's curvature around it is
  its holonomy per revolution, `Unbound` where it binds none. LAW-FIELD
  fixture 2 is restated: a flat region binds no orbit.
- **Claims:** `well_stands_on_its_attractor`,
  `well_membrane_is_its_enclosure`,
  `well_volume_and_potential_are_readings_per_lane`,
  `well_stage_is_a_coordinate_not_a_flag`,
  `mind_well_stated_and_reconstructed_by_a_court`,
  `well_body_stated_and_admitted_by_its_curvature`,
  `flat_region_binds_no_orbit_refused_unbound`,
  `closed_geodesic_with_zero_precession_is_resonant`,
  `reconstruct_well_refuses_an_atlas_of_two_attractors`,
  `chart_of_ratio_invariant_round_trips`; a `compile_fail` doc test on
  `Well.percept`.
- **Left for S9a:** the contemplation's orbits (`cech::orbit::Field`)
  attached to wells as their local orbits; until then every well is flat.

### LAW-TRAVERSAL v1 proposed (2026-11-06)
- Proposal 78.176: the inter-kernel crossing through the universal core
  tube, agreed with netstratum. A traversal is two LAW-PASSAGE v3
  passages composed at the court's vertex, one out through the source
  universe's own throat and one in through the destination's,
  reconstructed as a LAW-COORDINATE v4 re-origin read twice; any other
  construction is refused `NotThroughCore`.
- Each kernel declares its polarity as data, and the inter-kernel map
  (polar or antipolar) is one case of the law. The antipolar case
  exchanges a kind's frames, `(z_k, q_k) ↦ (q_k, z_k)`, keeping the whole
  `z_k + q_k` per plane (3Z and a nucleon mass kept), the axis inversion
  `t ↦ 1/t` on the ratio `t = z_k/q_k` fixing only the throat, `t = 1`.
  The asymmetry `σ = (z − q)/(z + q)` and the fraction `o = q/(z + q)`
  carry accordingly, `σ ↦ −σ` and `o ↦ 1 − o`.
- It has five joint-read fixtures. Fixtures 1–4 are read by netstratum
  and xylarium as pure mathematics; fixture 5 restates the existing
  `NotThroughCore` enforcement as the traversal's own refusal. Which
  kernel's universal core is the antipolar face of a given tube is not
  confirmed: xylarium's operator rules it for xylarium's own tubes, and
  no fixture here is marked xylarium-confirmed.
- It is a B3 crossing kind (ROADMAP track 24 B3), to build after this
  proposal is agreed. Open: the wire framing of a traversal (v0.5.0),
  the kernel-pair polarity declaration's form, and the reading of
  alignment across the tube. No code changes; `forms` and the court's
  shells are unchanged.

### MU-ANASTOME: one home for the anastome language (docs only)
- **New document, `docs/MU-ANASTOME.md`.** The language's levels mapped to
  the algebra (letters, words as the whole even element, conjugate classes
  as class vectors with the conjugating element kept, sentences read at
  loop holonomy, expressions); letters adjoined under U14/U15, the
  seventeen seed letters kept as a dated historical note; concurrency
  (turnings commute iff their planes are disjoint; at most ⌊p/2⌋ continuum
  plus ⌊q/2⌋ vacuum, tabled for perception `Cl(4,1)`, two planes, one
  involving `e₊`; coordinate v4 `Cl(3,3)`; netstratum `Cl(4,4)`); growth as
  a presentation (generators adjoined, relations proved, reduction never
  compaction, the forbidden compactions); the vocabulary ruled 2026-11-05
  (expression, assert or express, parse, reconstruct; "utterance"
  retired); every wire act as an assertion, a read, a court-to-court
  exchange or a universe crossing, retired acts marked and the 2026-09-01
  five-utterance algebra noted as historical; the language's state as a
  reading, with what reads it today and what is owed; and §8, two
  conflicts that need a ruling (what a relation is; trivial holonomy
  against `Loop`).
- **Corrected elsewhere.** MU-ENGINEERING §4 lists `ACT_QUOTE` among the
  readers, not the retired acts (it is live: `Action::Quote` is an
  observation with a handler). "Utterance" is replaced by the ruled
  vocabulary in GLOSSARY (*Expression*, *Utterance* marked retired,
  *Chatter / assert*), MU-ENGINEERING §9, AGENTS' transport row and
  MU-RECURSION's *Vocabulary, fixed*; quoted rulings are left verbatim.
  README states four levels and the act kinds of the current wire.
- **Moved, as ruled.** `MU-ALPHABET.md` whole into MU-ANASTOME §2
  (MU-ALPHABET §n is §2.n) and removed; MU-RECURSION §0.7 (the lexicon,
  one sense per word) into §5.2 and §1.8 (the language is a presentation)
  into §4.1, each heading left in MU-RECURSION with a pointer.
  MU-RECURSION is 187 lines shorter.
- **Kept.** `MU-LEXICON.md` stays where it is: the served register's test
  reads it (`src/physics/laws/tests/lexicon.rs`, 78.122), and moving it
  waits on that test's path.
- **Rebound.** Every reference to MU-ALPHABET in README, GLOSSARY,
  MU-ENGINEERING, MU-SCIENCE, MU-SENSING, MU-EQUATIONS, MU-TOPOLOGY,
  MU-COGNITION and MU-RECURSION names MU-ANASTOME; README's document list,
  GLOSSARY, MU-CLIENT, MU-SCIENCE and MU-ENGINEERING §9 link the new
  document. Code comments that cite MU-ALPHABET read through
  MU-ANASTOME §0's numbering and are left for the streams that own `src/`.

### `forms::coupling` holds LAW-COUPLING v1: the dipole parcel per plane, the coupling on a turning plane and the light ledger, exact over ℚ (proposal 78.175)
- **Network contract: a new law form and a new clause.** A new shell,
  `FORMS-COUPLING` v1, joins `forms::LAW_IDS` (and the SDK's
  `FORM_LAW_IDS`), ruled by netstratum's and mycelium's operators
  2026-11-06 through MU-RELEASE §3a, extending LAW-FIELD (netstratum
  docs/121 §9–§13, docs/122 §2). The clause register serves it as a third
  clause, `FORMS-COUPLING` v1 at kernel standing (signable and vetoable),
  on `ACT_LAWS` mode 7, and `CLAUSES` (and so the conformance statement)
  names it. New writing: fixture tag 12 (`Fixture::Coupling`, an
  operation byte 0–5 then its exact values) and object tag 9
  (`Object::Coupling`, a reading byte 0–6). A kernel that signed the
  register must reconstruct the new clause's 25 fixtures or veto it; no
  act or ledger layout changes. The SDK reconstructs it through
  `forms::coupling` (`reconstruct_fixture`).
- **Shared degrees are round** (ruled 2026-11-06): rotation angles on
  Clifford turning planes; an L1 projection is a named reading
  (`hemisphere_l1`, `Parcel::arc_l1`).
- **F1–F2, per plane:** `Parcel` (`Parcel::kind`: `3Z − U` held, `U`
  open) and its `Dipole`, the pole `σ = (z − q)/(z + q)` and the open
  share `o = ν(q/z)`; `open_share` (`ν(t) = t/(1 + t)`),
  `Parcel::inverted` (`ν(1/t) = 1 − ν(t)`); `Parcels`, a body's frames on
  each turning plane, founded by the even split (`Parcels::founded`), a
  bond or peel changing one plane (`Parcels::bond`).
- **F3:** `Alignment` at a round separation on a named turning plane,
  `h = hemisphere_l1(Δ) = 1 − Δ/90°`; `Neighbour`, `nearness`, `field`
  (`F = Σ w·σ·o·n·h / Σ w`), `pull` (`x′ = x ⊕ y`,
  `y = −σ·F·o·(x̄ ⊖ x)`, by `forms::coordinate::flow`), `relative` (the
  flow law's inverse), `Enclosure` and `meets`.
- **F4:** `Face`, `unlocked`, `take`, `ledger`; a ledger that does not
  balance is refused `Unbalanced` with its residue.
- **Fixtures:** 78.175 §5, claimed in `forms/src/tests/coupling.rs`.

### The field's carrier, stage 1: a sample of any rank lifted into an atlas, curvature as holonomy per lap (track 26 S4a and S7; LAW-FIELD v1 as proposed)
- **No network contract.** No wire, tape, task, word or chart address
  changes; the carrier is new and nothing persisted reads it.
- **The carrier** (`src/transducer/field/carrier/`), built against proposal
  78.171 as proposed, amended here if the joint reading changes the law:
  - the field's frame planes, plane `k` held by `e₊ₖ` and `e₋ₖ`, so the
    algebra is `Cl(p, q)` read per plane (`FramePlanes`), and a turning
    plane two units of one square, FORMS-COORDINATE v4's H key
    (`TurningPlane`);
  - a turning: an angle exact in degrees on its named plane, its winding
    `K = ⌊|θ|/360⌋` kept, its motion `ζ_ℓ^t` in `ℚ(ζ_ℓ)` reduced by `Φ_ℓ`,
    never a trigonometric value (`Turning`, `Cyclotomic`);
  - a motion of turnings on disjoint planes, one component per plane; two
    planes sharing one unit refused `NotExact` with both (`Motion`);
  - the atlas: charts, attractors with their local orbits and membranes,
    the connection the held orbits' transitions and flat elsewhere;
    curvature as the holonomy of an orbit's transitions per lap, read per
    plane in degrees with its winding and order; zero precession refused
    `Loop`; monodromy flat, torsion or rupture; an angle asked of several
    planes refused `ScalarCollapse` (`Atlas`, `Orbit`, `Holonomy`);
  - the membrane, the throat of one frame plane on the pole basis, and
    infinity outside every enclosure refused `Unenclosed` (`Membrane`,
    `stand`);
  - the lift of a sample of any rank: `n + 1` frame planes, axis `i` on
    `e₊ᵢe₊₍ₙ₊₁₎`, each body on its chart, each band's corners as points,
    the far seam held as 360° with `K = 1`; an axis taken apart coarsest
    first (`Lifted`, `Nested`).
- **LAW-FIELD v1's six fixtures stand as claims**
  (`src/transducer/field/carrier/tests/law_field.rs`): 30° × 3 on
  `e₊₁e₊₂` precesses 90°, `K = 0`, order 4; a flat path refused `Loop`;
  72° × 5 is 360° with `K = 1` and the identity motion; period 9 taken
  apart as `3 · 3` reads 240° and 120°, recomposed 280°; monodromy orders 1
  and 3; `Unenclosed` outside a membrane, `4/5·n_∞` lawful inside.
- **The field stated as an object a court reconstructs**
  (`Atlas::put_statement`, `reconstruct`, `admit`): the statement writes
  the atlas and each lap's turnings by plane and angle, never a reading;
  the court rebuilds every motion from its stated angle, derives the
  curvature itself and admits a mind's claimed curvature on a match,
  refusing `Differs` with each pair otherwise. Not on the wire (S9).
- **Open, for the joint reading** (78.171 §8): the plane a sample's axis
  turns on at rank `n` (the lift's convention), and the product of
  turnings on planes that share a unit (S7 stage 3).

### Documentation aligned with the operator ruling of 2026-11-05/06 on sound, harmonic structure, signal and noise (docs only)
- **The model is stated once, in MU-SENSING §3.10** (new section). Sound
  is the medium's wave: on the wire the transport's meaning-free bytes,
  the same for every receiver; in perception the samples a sensor takes
  in. Harmonic structure is a reading of sound, never the sound itself —
  periods and their whole-number multiples, phases in degrees with
  windings, re-periods, torsion of period ℓ — computed by the harmonic
  sense and held at the carrier layer. Signal or noise is decided by a
  reader's capacity to reconstruct that structure: its key, its
  language, its cover of eyes, and its two strongest separators, each at
  its own level — **consilience**, independent courts re-deriving one
  thing (MU-RECURSION 37.7, read with verification, POWV), at the
  court's level, and **convergence**, many minds' lineages reaching one
  class (POWC), at the minds' level. Noise is a flat reading for that
  reader, never a substance and never discarded; what does not resolve
  is held as a named residue, the demand — tomorrow's signal. No
  thresholds, no "signal strength" scalar, no denoising or filtering
  that discards information. Audio is one species' band of sound, never
  a basis (Concordance 1.6, species-neutral). The Noise Protocol
  Framework keeps its name: ciphertext's flat reading to an unkeyed
  party and full resolution to the keyed one agrees with this model,
  not by coincidence.
- **Cross-links, not restatements.** MU-ENGINEERING §9 (the transport
  carries sound's bytes meaning-free; a carrier holds the harmonic
  structure read from it; reading, never the layer, decides signal or
  noise), MU-ECONOMY §0 (consilience and convergence restated as the
  court-level and minds-level forms of the one strongest-separator
  claim), and MU-CLIENT §3.3 (the harmonic shell reads harmonic
  structure, a reading, never the medium; signal or noise read against
  the mind's own key, language and cover) each point to MU-SENSING §3.10
  rather than restate it.
- **GLOSSARY.** New entries *Sound*, *Harmonic structure*, *Signal*,
  *Noise* and *Audio* under *Work and adjudication*, beside *Sense*; a
  new *Consilience* entry beside *Convergence*, holding only the
  existing court sense (two independent courts re-deriving one thing,
  MU-RECURSION 37.7, POWV) and cross-referencing *Convergence* as the
  minds' sense (many minds' lineages reaching one class, POWC); *Signal*
  and *Noise* read both. *Noise transport* (under *The network*) is
  cross-referenced to *Noise* and kept by name, the cipher's
  flat-to-unkeyed, resolved-to-keyed reading noted as agreeing with the
  sensing sense rather than being a different one.
- **Swept** docs/, README, AGENTS, CONTRIBUTING, ROADMAP and GLOSSARY
  for wording against the model. No instance of "filter/strip/
  remove/discard noise", "denoise", "noise floor", "signal strength" or
  a threshold on noise was found outside one already-resolved historical
  passage (MU-RECURSION 70.4, a superseded ruling describing what it
  fixed, left as written); no "audio" as a basis; no "coincidence"
  dismissing a shared meaning (the two instances found use the word
  correctly, for an exact spatial coincidence and for two independent
  counts agreeing). The existing doctrine already read signal and noise
  by the reader's own re-derivation (MU-RECURSION 78.44(5), 1271, 1275;
  MU-CLIENT §4.9, §4.9's "noise not yet signal"; 78.76's "noise read as
  a scene, judged by speech") and residue as held content, never
  discarded (MU-RECURSION 2.3's "a residue is congruence and not
  noise"); this ruling states that reading as one model and nothing in
  those passages needed correcting.
- **Not touched, by the task's own scope: src/.** `src/network/wire.rs`
  names the transport's fourth lane `Lane::Harmonic` ("𝒽 harmonic/shell"
  in its doc comment), where every other carrier of the same lane names
  it Holonomic (GLOSSARY *Lane meanings*; MU-ENGINEERING §5;
  `forms::coordinate::Lane::H`); its one other site is
  `src/network/tests/noise.rs:106`. Left for the names pass alongside
  the lane byte's already-tracked removal (MU-ENGINEERING §9.2).

### Noise XX wired into the live transport, opt-in per peer
- **Network contract:** changed, backward compatible. A connection may
  open with the channel's opening, `noise::OPEN` = `ff 4e 58 58 01`,
  answered by the same five bytes and then the
  `Noise_XX_25519_ChaChaPoly_BLAKE2s` handshake; every frame after it
  rides Noise transport messages (`NoiseStream::{write_frame,
  read_frame}`), the frame's own bytes unchanged inside. The observer
  gains the `channels` reading (tag 26). No act, ledger layout, signed
  byte or law version changes.
- **How a plaintext peer interoperates.** A hypha's door accepts both: a
  first byte that is a lane byte is a plaintext frame, read and answered
  as before, and its caller is named (`PLAINTEXT accepted <address>`). A
  dialler holding a Noise home that reaches a door speaking only
  plaintext reads that door's refusal of the opening (a head over the
  wire budget) as `NotSpoken`, dials again in plaintext, and names the
  peer; it is never pinned, and this process dials it in plaintext from
  then on. A client with no home (`Session::open` alone) speaks plaintext
  as before. Noise-only, a door refusing plaintext and a dialler refusing
  `NotSpoken`, is a later release step and a network-contract change of
  its own.
- **Pinning:** a peer's static key is pinned on first contact under the
  address dialled (`noise.pins` in the home). A pinned peer presenting
  another key is refused by name (`PeerKeyChanged`, both keys), and a
  pinned peer answering in plaintext is refused by name
  (`PinnedAnsweredPlain`); neither falls back to plaintext. Re-pinning is
  an operator act.
- **Where it stands:** `src/network/channel.rs` (the opening, the choice
  per peer, the paced read in strokes, the book of peers met);
  `src/bin/mycelium-server/transport.rs` (the door's `accept`, every ask's
  `tell_peer_over`, the listener's deferral `defer_unread` inside the
  caller's channel); `main.rs` opens the hypha's `NoiseHome` at boot, and
  the console asks on the home's key where one stands; every judge takes
  a `Channel`. The SDK session opts in with `Session::with_noise`
  (`NoiseHome` re-exported in `mycelium_sdk::session`), and the example
  client does. Deferral and backpressure are unchanged inside the
  channel; the round trip is still measured in polls from the write.
- **Layers:** `src/network/channel.rs` joins `TRANSPORT` in
  `gate/src/layers.rs`; it names no carrier type, and no exception is
  added.
- **Claims:** `client_and_hypha_speak_inside_noise`,
  `hypha_peers_speak_inside_noise`, `pinned_key_mismatch_refused`,
  `plaintext_peer_named_not_hidden`
  (`src/bin/mycelium-server/tests/channel.rs`) and
  `opening_reads_as_no_plaintext_frame`. The quickstart mesh runs Noise
  between its three hyphae and its SDK client; the conformance suite and
  the house, opened without a home, are named plaintext and conform.
- **Not yet:** the reference mind's sessions (`Mind::open`) and the
  chatter posts between minds stay plaintext.

### H stands only on turning planes: `FORMS-COORDINATE` v4 (proposal 78.173, agreed with netstratum 2026-11-05, docs/117 §10.3)
- **Network contract: a law version and the register's fixtures.**
  `FORMS-COORDINATE` is v4 in `forms::LAW_IDS`, the SDK's
  `FORM_LAW_IDS` and the clause register, so the clause served on
  `ACT_LAWS` mode 7 and named in a kernel's signing act (mode 8) is
  `FORMS-COORDINATE` v4, still at kernel standing (signable and
  vetoable). The coordinate's layout in a clause is unchanged; a key's
  axis may now be a frame unit, written with its sign as the letter byte
  (`+` for `e₊ₖ`, `-` for `e₋ₖ`) and its plane's index, and an H key
  reads back only where its two units square alike. The served fixtures
  1a, 2, 4, 5, 6, 11 and 12 key H on `e₊₂e₊₃` and `e₊₃e₊₁` in place of
  v3's `e_Z1Q1` and `e_Z2Q2`, every value kept, so their addresses
  change. A kernel signing v3's clause must reconstruct v4's fixtures to
  sign again. No act or ledger layout changes.
- **H key validation** (`Lanes::of`): an H component is keyed by a
  generator of two frame units of one square, the continuum's `e₊ₖe₊ₗ`
  or the vacuum's `e₋ₖe₋ₗ` (`Plane::Turning`). A frame plane's own
  `e₊ₖe₋ₖ` (`Plane::Frame`), a mixed pair `e₊ₖe₋ₗ` (`Plane::Mixed`) and a
  generator of axes named by letters (`Plane::Unsigned`) are refused on H
  `NotTurningPlane`, naming the key and the plane. S's keys may name
  boost planes. New: `Axis::unit`, `Square`, `Key::plane`, `Plane`.
- **Fixtures:** 78.173 §5 restates fixtures 7 and 11–16 on the new keys,
  each value kept: v3's `e_Z1Q1` is `e₊₂e₊₃`, `e_Z2Q2` is `e₊₃e₊₁` and
  τ₁₂'s `e_Z1Z2` is `e₊₁e₊₂`. Fixtures 7 and 11–15 reach v3's objects;
  fixture 16's key order follows its keys (`(-1e/1,190/1,438/1)`,
  `K = (0, 1, 3)`), and 16c (the turning-plane refusals) is new. Claimed
  in `forms/src/tests/vectors.rs` (`h_keys_are_turning_planes`).
- **The winding is kept** (78.171 amendment (c)): `Turn::plus` composes on
  the `lcm` of the periods without reducing mod 1, so a composed motion
  holds its whole turns with its phase (`Turn::winding`, `K = ⌊|θ|/360⌋`;
  `Turn::phase`); a place's residue (`Turn::less`) reads the phase.
  `TurnMotion::then` and `verify_turns` follow; every chart reading is
  unchanged (`chart_reads_sections_as_turns`). Claimed by
  `composed_turns_keep_their_winding`.

### LAW-FIELD v1 amended (a–e) for the peer kernels' reading (proposal 78.171, #36)
- Documents only; no wire byte, act, ledger layout or law version changes.
- Proposal 78.171 states the five changes agreed with netstratum and ruled
  2026-11-05: (a) origin and infinity on the pole basis, `n_Z = e₊ + e₋`
  and `n_Q = e₊ − e₋`, the throat's fixed sphere the membrane and `e∞`
  only inside an enclosure; (b) `Cl(p, q)` with the signature read per
  plane, never a fixed `Cl(n+1, 1)`, the pole labelling riding the state;
  (c) each phase exact in degrees on a named plane with its winding
  `K = ⌊|θ|/360⌋` (LAW-COORDINATE v3), never a phase mod 1 alone; (d) a
  sphere only as a holding's own enclosure, netstratum's `S⁷ ⊂ S¹¹` as
  nested enclosures; (e) the orbit's shear and coupling cited from
  netstratum docs/121.
- The six fixtures are restated on the amended law with their values
  unchanged (90° on `e₊₁e₊₂` of order 4; flat refused `Loop`; 5 × 72° the
  identity, `K = 1`; site 7 of 9 as `(2/3, 1/3)`; orders 1 and 3; `e∞`
  outside an enclosure refused `Unenclosed`). Netstratum has read them;
  xylarium's reading is owed. ROADMAP's LAW-FIELD row follows.

### Small ruled capabilities wired to their live callers
- **The thermodynamic ledger is read** (68.5, 78.7 VS1). `note_viscous` is
  back, written by the pivot where a tether the field let go by dissipation
  drains (`Contemplation::release`), on the lanes the tether occupied. The
  client's winding takes the ledger once per stroke (`take_metabolism`, now
  a production reader) into the mind's observe reading: `entropy`, `sealed`,
  `viscous` and `flux` per lane, printed on the mind's reading; the `now`
  call reads the stroke's ledger so far (`metabolism`). The mind's observer
  reading gains those fields at its end; no mesh wire change.
- **Mint refusals are named, not dropped** (78.158a U1, U8). Where the
  speech path's mint refuses a sentence with demands, the mind holds the
  refusal under its task (`Mind::hold_mint_refusal`): its
  `mint_refusal_line` and the withheld form classified by `withheld_of`
  (`Withheld::arm`, `obstruction_kind`, and the new `is_askable`, read off
  the kind). The observer's `refusals` call reads them. `mint_arm`,
  `mint_refusal_line` and `Withheld::obstruction_kind` leave `cfg(test)`.
- **The SDK speaks every act the server judges.** `mycelium_sdk::house`
  re-exports the handoff family (handoff, its read, the loops and tube
  reads), traverse, the universe's ledger and map, the challenge window's
  reads (finality, line head, obstructions), the fold sent up, and the
  equivocation proof (`encode_equivocation`, `decode_ruled`, with
  `equivocation_proof`, which frames two receipt sections only where they
  fork at one place of one signer's line). `mycelium_sdk::session`
  re-exports the mind's own acts (quote, speech, syllabus, digests, fetch,
  peers, probe, evidence). `ACT_TABLE` gains handoff, evidence,
  equivocation, uplink and challenge. No wire change: every codec is the
  library's own. The reference mind sees no signed receipt sections, so
  sending a proof from it is left as the next step.
- **Readings.** A family the mind takes in (the court's task, the corpus
  feed) is read twice side by side on its observe reading: its count
  (`family_magnitude`) and the exact share of the frames its pairs change
  (`transduce::family_share`, now taking the pairs by reference), with the
  families whose share did not read counted. `Court::demands_listed` leaves
  `cfg(test)` and stands on the carriers census the observer serves, as
  `demands_listed`.

### Transport, carrier and coordinate: the three layers documented and gated
- **Network contract:** none. No wire byte, act, ledger layout or signed
  byte changes; no source under `src/` changes.
- **The model** stands in MU-ENGINEERING §9: the transport moves bytes
  (connection, channel security, a length, deferral) and never holds
  meaning; a carrier holds meaning at full dimension and never projects
  to fewer; a coordinate routes and never identifies, and TUniversal
  stamps and never drives. §9.2 lists the known violations, each with its
  fix and release. Linked from AGENTS (*The regressions that recur*);
  GLOSSARY adds *Transport* and *Utterance* and aligns *Carrier* and
  *Frame*.
- **The gate keeps the transport free of meaning** (`gate/src/layers.rs`,
  tree test `transport_names_no_carrier`): the transport sources
  (`src/network/wire.rs`, `src/network/noise.rs`,
  `src/bin/mycelium-server/transport.rs`, listed once) name no carrier
  type (`Mv`, `Modes`, `Mu`, `Coordinate`, `Sample`, `SectionSample`,
  `SectionSite`, `Turn`, `TPoint`, `Lane`), read as tokens so comments
  are not read. Five listed exceptions, each until v0.5.0: `Lane` in the
  wire and the door (the lane byte), and `TPoint` in the wire, the Noise
  channel and the door (the pointed frame's trailer). A stale exception
  is refused. Fixture tests: one refusal, one pass.
- **"Frame" audited:** ten meanings, with counts and a proposed name for
  each by function, in MU-ENGINEERING §9.3 (*Names to settle*). No code
  identifier is renamed yet.

### Dead code removed (2026-11-05)
- The retired acts' codecs are removed: the docket request and reply
  (`encode_docket_request`, `decode_docket_request`, `encode_docket_reply`
  and its `_with` and `_paged` forms, `decode_docket_reply`,
  `decode_settlements`, `decode_cavities`, and the `Docket` record), the
  teach and settle codecs, and `encode_outcome_reply`. The bytes stand in
  `Action` and the door still refuses them by kind; one byte fixture per
  retired act keeps that refusal tested.
- The retired settlement arms and duplicates are removed:
  `Treasury::settle_refine`, `settle_condense` and `settle_assembly` (the
  assembly premium retired with its producer, cut 2026-09-10; MU-EQUATIONS
  6.7 and 6.8 and the MU-ECONOMY table no longer list them),
  `Treasury::release_filled`, `Court::prove_folded` (the live era proof is
  `server/backfill.rs::prove_folded` through `Court::walk_folded`),
  `network::wire::{multiplex, demultiplex}`, and
  `verdict::{reconstruct_certificates_at, reconstruct_form_on}` with the
  `reconstruct_certificates` wrapper only they called.
- The replaced keyed-schema API is removed: `Keyed::{read_fibre,
  truncate_key, fibre_over, for_system}`, `coord::Any::undeclared`,
  `Stratified::{schema, keyed}`, and the never-constructed
  `SchemaRefusal::InvalidDepth` and `SchemaRefusal::Stratum`. The foreign-key
  refusal of `Keyed::read_exact` is tested on a typed carrier.
- Sixty functions with no caller and no document are removed, each
  confirmed by its path and type, among them `Clock::k`,
  `observe::live::process`, `cognition::germ_address`, the court's
  `active_cover`, `unlist_foreign`, `open_charts`, `taught`, `classes_on`,
  `cohesion`, `torsion_total`, `is_home_listing`, `quote_frontier`,
  `settlements_since`, `settlements_heard_to`, `set_court_head`,
  `court_head`, `court_federation_len`, `field_cold_len`,
  `settlements_hot_len` and `dynamic_lane_capacity`,
  `Surgery::{bridge_merge, split_cut}`, `cga::{ray_name, parabolic_name}`,
  `Pole::of` and `SpanTally` with its `of`; and the helpers and types only
  they used (`EscrowedAward`, `LaneMembers`, `chart_span`, `orbit_span`,
  `Stratum::child_axes`).
- The idle consolidation worker is retired (clients think, hyphas verify):
  `synthesis::consolidate`, `Court::consolidate`, `syntropic_stroke`,
  `optimize`, `consolidate_stroke`, `retire_redundant` and the
  `syntropic_cursor` they kept are removed. The reflex opening's
  `condense_stroke` stands, and the condensation and torse tests now run
  on it. `physics::equilibrium::consolidate` is unchanged. Proposal 78.157
  marks S3 retired.
- Retired constants and acts: the constant `ACT_DEMAND_ROUTED` is removed
  (its byte 0x14 stays in `Action::DemandRouted`, refused at the door);
  ruling byte 5 (`RULING_ROOT`, no producer, no reader) is retired as 3
  and 4 are; `ACT_INSTALLED` (no sender, no handler) is of the retired
  kind, so the door refuses byte 0x22 by name, and `decode_installed` and
  its `Installed` record are removed; `TransductionRefusal::Unenclosed`,
  never produced, gives way to the produced `PairRelation::Unenclosed`.
- 168 functions only the library's own tests call move under
  `#[cfg(test)]` where they stand, with the private helpers, types and
  imports only they use: test-only wrappers of a live variant
  (`reconstruct_form`, `encode_quote_reply`, ...), readings and counters
  the tests assert on (`Court::dockets`, `docket_circuits`, `heard_len`,
  ...), and the doc-named theorem, register and math readings, which stay
  in their files as proof objects so their documents' citations hold. No
  test loses a capability.
- SDK surface change. The thin modules `deferral`, `mind` and `wire` are
  folded: `Deferral`, `deferral_of` and `read_deferral` (the one deferral
  reader), `Mind` and `GenomeReading` move to `mycelium_sdk::session`;
  `WIRE_VERSION`, `ACT_TABLE`, `solve_kind`, `SOLVE_KIND_*`, `PENDING_CODE`
  and `encode_deferral` stand at the crate root; `is_deferral` and
  `deferral_lanes` leave the SDK. Duplicate re-exports are dropped so each
  of these items has one path: `session::Voice` (use `voice::Voice`),
  `house::{TPoint, Mv, Enclosure, ReserveReading, HyphaKey, Law, Served,
  decode_register_reply, encode_register_request, MODE_*}` (use
  `session::TPoint`, `rule::Mv`, `task::Enclosure`,
  `readings::ReserveReading`, `voice::VoiceKey` and `laws::`),
  `readings::{decode_clock_reply, encode_clock_request}` (use `house::`),
  the Concordance codecs under `laws::` (use `signing::`) and the modes
  under `signing::` (use `laws::`). A hidden `deferral` module keeps the
  three deferral items for `sdk/examples/house.rs` until that example
  reads them from `session`. The prelude is unchanged.
- Two ceremonial assertions go: `FORM_LAW_IDS == forms::LAW_IDS` in
  `sdk/tests/laws.rs` (equal by re-export) and `assert_eq!(w, w, …)` in
  the sentence address tests. `sdk/tests/surface.rs` keeps its one import
  per item: no smaller form fails the build by name when an item moves.

### The 2026-11-05 audits' confirmed defects are fixed: a lane past the four is refused, a mind's admission names its refusal, a clause-stated closure carries its modes, a seated mind's solve is paid to its own key
- **A lane past the four is refused by name, never wrapped**
  (`lane_past_the_four_refused_never_wrapped`). `HomeTrie` indexed its
  lanes with `(lane as usize) & 3`, so lane 5 was filed, read and rooted
  as lane 1: a peer's cover page naming lane 5 stood a hypha on a lane it
  never named. A lane byte of 4 or more is now `LaneRefused { lane }`
  ("lane 5 refused: the base lanes are S, G, T, H (0 to 3)"). `admit`,
  `retract`, `rehome`, `fibre`, `multiplicity`, `charts` and `root` return
  `Result<_, LaneRefused>`; the exchange and the SDK session refuse a cover
  page at its first chart on a lane past the four and stop its walk; the
  court's own sites, which read only the four lanes, file nothing on a
  refusal. **Network contract: no wire change.** The cover page's bytes
  are unchanged; a lane byte of 4 or more, which was masked, is refused.
- **A mind's admission names its refusal** (`mind_admit_names_its_refusal`;
  reference mind, crate-private). `transducer::mind::admit` answered a
  `bool` its callers dropped with `let _ =`, and dropped each lane step's
  refusal inside it: a long law's admission landing where no mind thought
  (a guard settling perception between ticks) was lost without a word. It
  now answers `Result<Landed, AdmissionRefused>`: `Landed` reads the flux
  readings, words taught and covers joined; `NoMindThinking` hands the
  admission back whole; `LanePastItsCarrier { ledger, lane, held, add }`
  names the lane that would pass a `u64`, checked before anything lands, so
  the lanes land whole or not at all. The mind's two callers land a handed
  back admission by thinking in the mind's own field and report any other
  refusal.
- **A clause-stated closure carries the modes its carriages moved**
  (`paid_verdict_carries_the_modes_paid`). The deflation's motion reader
  (`form_motions`) read word vertices alone, so a form speaking by clauses
  (78.143, `Form::elements`) read as moving nothing: a solve its carriage
  closed was paid with an empty `Modes` (the claim's fixture, before the
  fix: "paid [2/9, 0, 0, 0]; modes no modes"). It now reads each
  `RuleElement::Carriage`'s versor and gauge beside the word vertices', and
  `rule_flux` reads the same motions, so the T flux and the T modes come
  from one set of motions: a clause carriage's measure and pay now carry
  the T flux a word vertex's motion already carried. **Not fixed, a
  contradiction of MU-ECONOMY §1 recorded for a ruling.** The quickstart's
  two paid verdicts ("paid μ [0, 2/3, 0, 0]; modes no modes" on
  `00576224`, "paid μ [11/50, 0, 0, 92]; modes no modes" on `00dbd492`)
  are closures by clause elements on a form with no word vertex, whose
  elements state no motion and no hue map the modes reader can name (on
  `00576224` a `Tile` of mirrored blocks). The closure moved no mode
  of §1's four kinds, while μ was paid from the closed sites' shares of
  the frame. A G-only closure by word vertices whose hue maps moved (for
  example `08ed6ac7`) also carries no modes, because each pair's modes are
  read at the S mass it carried, which a recolour does not move (§12
  item 4). No mode is invented for either.
- **A seated mind's solve is filed under its own key and paid to it**
  (`seated_mind_solve_paid_to_its_own_key`; Concordance 6.2a, 78.165). The
  SDK's seat path (`solve_before`) sent a task's solve to the house alone,
  as the in-game call `FN_SOLVE`, so a house that filed it with the court
  did so under the house's key and was paid for the seat's work. The seat
  path now files the court solve itself, signed by the key the mind sat by
  (`Mind::file_seat_solve`, over the new `Mind::submit_solve_signed`, which
  `submit_solve_rule` now calls with the voice's key), and still calls
  `FN_SOLVE` for houses that want the seat's answer in their game. The
  court pays the key that signs; `FN_SOLVE`'s docs, the house example and
  MU-HOSTING §5a.6 say so. **Network contract: no wire change.** The solve
  is the existing `ACT_SOLVE`; a seated mind now sends one per task it
  solves, held for its return as every solve of the mind is.

### CI runs clippy and the tests on every pull request; corpus tests refuse without a corpus
- **Network contract:** none. No wire byte, act, ledger layout, task,
  word or chart address changes.
- `.github/workflows/clippy-and-tests.yml` (workflow and job
  `clippy and tests`, named for the commands it runs) runs on every
  pull request and every push to main: it fetches the ARC-AGI-2 corpus
  into `MYCELIUM_CORPUS`, then runs
  `cargo clippy --release --workspace --all-targets -- -D warnings` and
  `cargo test --release --workspace`, four build jobs, 60 minutes bound,
  `~/.cargo` and `target/` cached on `Cargo.lock`.
- The four corpus tests that passed having read nothing
  (`sources/arc-agi/tests/retract.rs`'s three, `ground_census.rs`) now
  refuse where the corpus does not read, naming `MYCELIUM_CORPUS`.
- The pull request template is what changed, why and the network
  contract; its self-ticked checklist is gone, as CI runs the checks.
  AGENTS.md, CONTRIBUTING.md and ROADMAP.md say so.

### The client surface speaks the 2026-11-04/05 rulings: angles as degrees on a named plane, the multi-dimensional ledger, holonomy unpriced, the Concordance signed by reconstruction
- **Network contract: wire strings changed; no codec bytes.** The codec
  and wire bytes are unchanged, and the seat's kind codes 1–6 for the
  plane symmetries are unchanged. Refusal reasons carried on the wire
  change wording:
  - a claim of factorisation naming no task held: "no such task on this
    book" becomes "no such task at this court" (`judge_factorisation`);
  - a solve on a task held with no content: "no such task on this book:
    its ground is not read" becomes "no such task at this court: its
    ground is not read";
  - `REFUSE_EQUIVOCATION`: "two receipts at one K with different chain
    hashes" becomes "two receipts at one line position i with different
    chain hashes";
  - `REFUSE_NOT_EQUIVOCATION`: "two receipts that do not share key and K,
    or do not differ" becomes "two receipts that do not share key and line
    position i, or do not differ".
- **SDK surface: `Symmetry`'s rotations are named by plane and degrees.**
  `Symmetry::Rotate90`, `Rotate180` and `Rotate270` are renamed
  `RotateE2E1By90`, `RotateE2E1By180` and `RotateE2E1By270`: the rotation
  by 90°, 180° and 270° in the `e₂e₁` plane of the perspective's two plane
  axes (`e₂` carried onto `e₁`). Re-exported unchanged in place by
  `mycelium_sdk::rule` and the prelude. The mirrors' docs read along a
  plane axis, never a row or a column.
- **Angles as degrees on a named plane.** The house's conformal law on a
  motion stated phase by phase reads each phase's advance as the degrees
  `360°·r/h` on its plane per bell. The conformance probes' rotor is "the
  rotor in the e₁e₂ plane at the rational point (3/5, 4/5)", and the
  conformance report's lines name it so. The mind's docs and the SDK's
  tests name the plane by its axes (`e₁e₂`), never "the sensor plane" or
  a turn without a plane.
- **Lanes as the coordinate's components.** `TPoint` is documented as
  LAW-COORDINATE v3's one-component, non-negative case (each base lane a
  vector of one exact non-negative component). The wire version's μ
  fields, the deferral's deficit and the quote's `[u64; 4]` fields are
  documented as readings per lane, never a holding. `Deferral`'s
  `Display` labels each lane (`deficit S 1, G 0, T 2, H 0`).
  `mycelium-client --init` prints `genesis line: voice <vk> <μ on S> <μ
  on G> <μ on T> <μ on H>`; the fields a genesis file reads are unchanged.
- **SDK surface: `Modes` in the prelude.** `mycelium_sdk::prelude` adds
  `Modes` (the carrier of μ, from `mycelium_sdk::task`) beside `Mu`, and
  `Mu` is documented as retiring in v0.5.0 (MU-ECONOMY §9). No type is
  removed. The wallet is documented as a reading of the holding's
  coordinate on the one multi-dimensional ledger, never "flat"; the
  readings' entries as events of that ledger. The client example and the
  conformance solve line print the verdict's modes beside the μ paid.
  The mind's comments say "the court's open listings" and "recorded" for
  "the open book" and "booked"; the mind's trace label reads "admission:
  the open listings".
- **Holonomy is read, never priced** (the reference mind, crate-private).
  `Mind::price_hearings` is renamed `read_hearings`, `Mind::price`
  `read_loops`, `Carrier::priced` `loops_read`, `Mind::lapse_unpriced`
  `lapse_field_waiting`, the board's `unpriced`/`unpriced_crossed`
  `field_waiting`/`field_waiting_crossed`, and the types `Pricing`,
  `Priced`, `FieldWork::Price` and `Inflow::Priced` `HeardLoops`,
  `LoopsRead`, `FieldWork::ReadLoops` and `Inflow::LoopsRead`. Their docs
  state the loops' holonomy as meaning read; a price attaches only to
  traversal. The mind's trace labels read "admission: hearings read" and
  "admission: a hearing (loops read)".
- **The algebra and the completion test.** The SDK's crate description
  and the rule seam's, the house's and the seat mind's docs name the
  algebra `Cl(p, q)` with the signature as data, `Mv` its conformal case,
  never `Cl(4,1)`. The mind's count of paid solves is documented as an
  observation, never a goal. The README's kernel sections already state
  the Concordance signed clause by clause by reconstructing its fixtures.
- **Replay, grid and K.** `ReceiptsRequest.replay` is documented as an
  era's receipts read as history, each reconstructed apart, never as
  verification. The README's client loop reads `TaskEnvelope::section()`
  (the removed `rasters()` no longer appears), and "raster" stands only at
  intake in the SDK's task docs. Cognition comments say "site" where a
  sample's place was a "cell". The receipt index is the line position `i`
  in the quote act's layout doc, `YieldQuote::k`'s doc, the chronometry
  module doc and the court's head docs; `K` means closed orbits. The
  identifiers `YieldQuote::k`, `k_step`, `head_k` and
  `KStepReceipt::step_index` keep their names for a later rename.
- **A gate guards the client surface** (`gate::surface`,
  `client_surface_holds_no_retired_phrase`): every line of the SDK crate,
  `src/client/`, `src/bin/mycelium-client/` and the README, comments
  included, is read for "quarter turn", "half turn" (either spelling),
  "clockwise", "Concordance hash", "a wallet is flat" and "ledger row",
  each refused with its ruling. The CHANGELOG and `changes/` are history
  and are not read.

### `forms::coordinate` holds each lane as a keyed vector, H in degrees with `K` per component, decides on the caller's intervals and re-origins across a throat: `FORMS-COORDINATE` v3 (proposal 78.173 fixtures 1–16)
- **Network contract: a wire change and a law version.**
  `FORMS-COORDINATE` is v3 in `forms::LAW_IDS`, the SDK's
  `FORM_LAW_IDS` and the clause register, so the clause served on
  `ACT_LAWS` mode 7 and named in a kernel's signing act (mode 8) is
  `FORMS-COORDINATE` v3. The coordinate's writing in a clause changes: a
  local is `[h u64 LE][lane count][(lane name, vector) …][K count][K u64
  LE …]`, a lane name its byte (S 0, G 1, T 2, H 3) or `4`, the name and
  its law version for an opened lane, a vector its components as
  `(key, value)` in canonical key order, a key its axes as `(letter,
  index)`. A lane set or vector out of canonical order is a second writing
  and reads as none. Fixture tags 6 (`within` on a sector at a depth)
  and 7 (a range on one lane) are retired and read as none; `within` on
  stated bounds is tag 10 and a range on one component tag 11. Object tag
  3 (one side) is retired; a side per component is object tag 6, and the
  refusals `ShapesDiffer` and `NoComponent` are object tags 7 and 8. A
  kernel signing v2's clause must reconstruct v3's fixtures to sign
  again. No act or ledger layout changes.
- **Lanes are vectors** (`Vector`, `Key`, `Axis`, `Lanes`): exact signed
  components, each keyed by its axis and held in the order of the keys
  written out. H's components are keyed by generators, the plane named by
  its two axes (`e_Z1Q1`, the turning τ₁₂ as `e_Z1Z2`), stored in degrees,
  with `K_c = ⌊|θ_c| / 360⌋` per component (`DEGREES_PER_TURN`, 360,
  registered `EXTERNAL` as the degree's definition).
- **Open lanes** (`Lane::Opened`, `Openings`): a lane set is a list of
  named lanes; a lane beyond S, G, T and H is admitted only through a
  ruled opening, and is refused `LaneNotOpened` naming its name and law
  version otherwise. None is ruled; the ruled act between kernels is the
  next step.
- **Decisions on the caller's intervals** (`Interval`, `Bounds`, `Sides`,
  `Whole`): `within` reads a side per component against stated bounds,
  within only where every component is; ranges are on one component by
  two bisections (`Bodies::range`), any other component a filter
  (`Neighbours::filter`). `Sector` and `Reading` are the display reading
  only (`BASE` registered `FORMAT`); `Sector::side` is removed.
- **Motion per component** (`Coordinate::advance`, `bell_of`, `merge`,
  `fission`), refused `ShapesDiffer` where two shapes differ. **The
  re-origin** (`Coordinate::pass`, `Through`, `Passed`) through a throat
  by `forms::passage`'s `Poles::cross`: S's span reverses, T gains
  `−2·span`, and the frame reads reach and hold exchanged, `H + R` kept and
  `δ ↦ 1 − δ`; within one enclosure, the face exchange alone. **The flow**
  of two pushes (`flow`), with which dilation invariance is claimed.
- **Fixtures:** 78.173 §5 restates fixtures 1–10 on one-component lanes
  with H in degrees (fixture 7 stands at 2250°, `K = 6`), adds
  netstratum's 11–13 (a two-plane H; a range on one component and a
  filter; a merger and fission per component) and new 14 (re-origin), 15
  (dilation invariance, xylarium's joint fixture) and 16 (the keys, `K`
  per component and an unopened lane refused). Claimed in
  `forms/src/tests/coordinate.rs`, `motion.rs`, `vectors.rs`, `throat.rs`
  and `properties.rs`. The register serves 1a, 2, 4, 5, 6, 11 and 12.
  Fixture 11b waits on netstratum's composition law.
- **Next:** rewire `TPoint` as the form's one-component non-negative case,
  and the ruled act that opens a lane.

### LAW-PASSAGE v3: the passage is the pole swap, `FORMS-PASSAGE` v4 (proposal 78.172; Concordance §12)
- **Network contract:** the clause register on `ACT_LAWS` mode 7 serves
  `FORMS-PASSAGE` v4 in place of v3, with new fixture and object tags. A
  passage fixture is `Fixture::Crossed` (tag 8: the plane's two units,
  the frame state and `T`) or `Fixture::Ridden` (tag 9: the two units and
  the riding state); a crossing reaches `Object::Crossing` (tag 5: the
  frame state past the swap, `S` and `T`). Fixture tags 0–3 (v2's
  product passed twice and once, the commutator, the square) are retired
  and read as none. A kernel signing on mode 8 reconstructs the eight
  passes of v4; a signature on v3 no longer names a served clause. No
  act, ledger layout or L0 law changes.
- **The law** (adopted by both operators 2026-11-05; netstratum docs/117
  §10, §12): a frame state `H·n_Z + R·n_Q` (hold and reach) goes by the
  sandwich `x ↦ e₊·x·e₊`, its span `H − R` reversed and its whole kept;
  the riding state goes one-sided, `q ↦ e₊·q`; one passage writes
  `T ↦ T − 2·(H − R)`; two passes are the identity; at a pole the passage
  is one-way. v2's product `pass(v, q)` is retired as the passage; `v_k`
  stays the plane's direction in the advance.
- **`forms::passage`** (`LAW_VERSION` 3 → 4): `Poles::of` (refusing
  `NotUnit`, `WrongSign`, `Mismatched`), `Poles::state`, `read` (to a
  `Frame`; refusing `NotFrame`), `swap`, `cross` (to a `Crossing`) and
  `ride` (refusing `MixedParity`); `Frame::whole`, `span`, `share`
  (refusing `NoWhole`, `NotShare`) and `place` (`Place`, `Pole`).
  `Throat::verify` and `pass_share` stand. `pass`, `NotDirection` and
  `NotEven` are removed.
- **Fixtures:** the five of 78.172 §5 and the roles' exchange
  (`(H, R) = (4/5, 1/5)` to reach `4/5` about hold `1/5`, the whole kept,
  `δ` `4/5 ↦ 1/5`) are claims in `forms/src/tests/crossing.rs`. v2's
  algebra facts stand as claims on `Clifford::product` in
  `src/transducer/cga/tests/product.rs`.
- **SDK:** `signing::reconstruct_fixture` reconstructs the new fixtures
  through `forms::passage::Poles`; `forms::passage::{Poles, Frame,
  Crossing, Place, Pole}` join the surface; `FORM_LAW_IDS` names
  `FORMS-PASSAGE` 4.
- **Documents:** 78.172 amended to v3; Concordance §12 restates draft
  2.11 on v3, held until xylarium reads the fixtures; MU-EQUATIONS §11.

### What crosses the universal core tube is rebuilt under the universal laws and reaches the hypha only through its core (track 24 B3, B4; Concordance 4.1, 6.2a)
- **Network contract (v0.4.0's break):**
  - **Wire:** `ACT_UPLINK`'s fold gains a transitions section after its
    handoff sides, `[count][transition …]`, each as `ACT_LAWS` mode 1
    writes one, present only where the fold states a transition (the
    children and handoff blocks then stand, empty where empty). A carried
    line's writing ends with its transitions, counted. The transitions
    are signed written out, under their own domain, after the fold's
    commitments. `Uplinked` gains `NotLawful` (tag 11: the path of
    carried lines, the transition's index, each breach with its law's
    register index) and `Unread` (tag 12: the same place and each law
    unread with the stroke it needs).
  - **Ledger layout:** unchanged; the receipt cites the fold's
    coordinates as before.
- **Rebuilt at the vertex:** `Reconstructed::fold` verifies the tier's
  signature and then each transition the fold states, and each of every
  line it carries, under L0 at `LAW_VERSION` (`verify_sized`, the verifier
  of mode 1). The fold names no law, so none is admitted on a declared
  name.
- **`NotThroughCore` (track 24 B4):** a universe's act reaching the hypha
  other than from its sitting reserve (a fold, an anchor, a map, a core
  tube offered, a conversion out, a manifest, a statement) is refused
  `court::admission::Boundary::NotThroughCore`, naming the act
  (`UniverseAct`) and the domain. A manifest or a statement another key
  of the domain signed reads `NotThroughCore` where it read `Unsigned`; a
  signature by no key of the domain stays `Unsigned`. Wire: `Uplinked`
  code 13, `Ledgered`, `Mapped`, `Granted` and `Handed` `[2]`, `Founded`
  21 carry the boundary's refusal (`[tag][act][domain 32]…`); the codes
  `Uplinked` 9, `LedgerRefusal` 1, `MapRefusal` 1 and `GrantRefusal` 3
  (`NotReserve`) are retired. `HandoffRefusal::NotReserve` stays for a
  counterparty that is not a reserve, and `Founded::NotReserve` for a
  bond.
- **`NotSignatory` at the tube (track 24 B4; Concordance §0):** each of
  those acts crosses only while the domain's sitting reserve converses on
  every network clause of the register served, read from the signing
  events (`Court::alignment`) on the act's stroke, never a stored flag
  (`Court::signatory`). Refused `Boundary::NotSignatory` (tag 1: the
  clause and the reading, unread or colliding with the residue). A kernel
  clause the reserve vetoed bounds only the acts it governs,
  `Boundary::Bounded` (tag 2; the coordinate clause governs anchors and
  maps); an alignment not read whole answers `Boundary::Unaligned` (tag
  3). A key's own acts (a solve, a stake, a conversion in) are not read.
  **A kernel must sign the register before its universe crosses.**
- **SDK:** the KERNEL read signs again after its colliding signing and
  checks every clause converses; the house example designates its reserve
  and crosses with a fold stating a lawful transition, and exits 1 where
  the crossing is refused.
- **Not yet reconstructed under a law, listed:** anchors, maps, mouths,
  conversions out, manifests and statements state no transition; a motion
  stated as turns has no wire writing (track 25, X1); no crossing states a
  core form's object; no crossing kind states a passage yet, so none is
  reconstructed under LAW-PASSAGE v3's `Poles::cross`. ROADMAP's open
  admission claims for the passage are renamed for v3
  (`crossing_is_the_pole_swap_with_its_flux_on_t`,
  `crossing_at_a_pole_is_one_way`).

### Documentation aligned with the operator rulings of 2026-11-05 and the rulings audit (docs only)
- **The ledger is one multi-dimensional ledger.** Its axes are lane ×
  register layer × grade, with the modes within each lane, never summed
  (MU-RECURSION 73.9, 78.1, 78.21's four-dimensional amendment of
  2026-09-24). Only the flat ledger is retired, never the word, and the
  ledger's dimensions are never axes of cognition. MU-ECONOMY's model and
  §2.1, the GLOSSARY (new *Ledger* entry; *Receipt chain* as a signer's
  line; *Era / fold* as a checkpoint of a line's prefix) and the README
  say so. "Hash-chained receipt ledger", "a closed segment of the
  ledger" and "ledger row" are struck from the documents; the built
  journal is named by its type, a treasury entry (`Entry`).
- **POW++ by name:** multi-axial convergence settlement, proof of
  reduction, not a single linear difficulty ladder (MU-ECONOMY §0,
  GLOSSARY, README). MU-ECONOMY §6 states the merge as a grow-only union,
  commutative, associative and idempotent, as a law of readings.
- **`K` means closed orbits everywhere; the receipt index is `i`.**
  MU-RECURSION law 9 (with "Lane T … a cost, never a coordinate"
  annotated as amended by LAW-COORDINATE), §0.0, 45.1, 78.23 and TU-1,
  MU-CHRONOMETRY, MU-ENGINEERING, MU-XSEPTUM, MU-LEXICON, MU-ECONOMY
  (§12 item 3 closed) and the GLOSSARY say so. Code identifiers keep their
  names.
- **78.173 §1** records the stamp law (TUniversal stamps, never drives;
  the rate advance runs on a universe's own bell), that an address routes
  and never identifies, base 60 as display only (`within` and ranges
  decide on exact intervals the caller states; code follows in the
  coordinate v3 change), and lanes open beyond S, G, T and H by a ruled
  act agreed between kernels (v3's direction). MU-ECONOMY §1, §2.2 and
  the GLOSSARY carry the same.
- **Economy rulings.** MU-ECONOMY records its deliberate losses (the
  magnitude tuple ‖Δ‖ retired by MU-RECURSION 12.6; 78.93(2)'s flat
  per-lane accounts superseded, an account being a reading of the
  coordinate, and 78.93(2) is marked superseded). A holding is a point,
  estates nest as origins and a region is a reading (§12 item 2 closed).
  One closure, one point: a concurrent second claim is burned as
  equivocation, unordered claims are held as a residue, F2a's equal
  shares are retired (§12 item 7 closed). The rate is per lane and
  belongs to motions, never to a holding (§3.3, §2.2). §6 cites 78.42 and
  §4 cites 46.2. Concordance 5.4 pays the H lane the exact degrees the
  motion traversed, per named plane, with no floor, holonomy never priced
  (ruled 2026-11-05); 78.133 and MU-EQUATIONS §6.12 are annotated.
- **POWC is widened:** lineages that share a cause but disagree before
  meeting count as convergence, and MU-RECURSION 74 (CV4) stands as an
  instance. GLOSSARY, MU-RECURSION (the proofs paragraph, 78.33, the
  POW++ row, now four proofs), MU-HOSTING §2.4, MU-ECONOMY §0 (§12 item 6
  closed), MU-SCIENCE §6.2 and the README say one thing.
- **Concordance §0** no longer reads draft 5's 2.11 as built: 2.11 is
  held pending LAW-PASSAGE v3 (netstratum's pole-swap proposal,
  2026-11-05), and the coordinate clause cites `FORMS-COORDINATE` v2. §12
  says the same, and ROADMAP's coordinate item names v2.
- **The rulings audit's wording.** Angles are stated in degrees on a
  named plane (no "quarter turn"; the H lane as rotations in exact
  degrees). `Cl(p, q)` with the signature as data is the algebra and the
  conformal `Cl(n+1, 1)` perception's case, in the README, GLOSSARY,
  MU-EQUATIONS §1.2, MU-SCIENCE, MU-TOPOLOGY, MU-ALPHABET,
  MU-ANTI-FLATTENING, MU-CLIENT, MU-COGNITION and MU-PRECURSORS, with a
  note on Concordance 2.10 that the amendment is in draft 5. Signing is
  never by hash (README, MU-CHARTER, MU-DERIVATIONS), and MU-EQUATIONS
  §11 gives the Concordance as version 5. The completion test is
  MU-SENSING §0: ARC-AGI-2 closures are observations, never the goal or
  criterion (MU-PRODUCT §4, Concordance 6.2a marked as conforming to the
  ruled test of 2026-11-04, MU-COGNITION, GLOSSARY, MU-PRECURSORS).
  "Replay" for a reconstruction is renamed in ROADMAP, MU-DERIVATIONS and
  MU-HOSTING, and MU-RECURSION's `replay_nerve` (removed) is rebound to
  `witness_torsion`. MU-COGNITION §2.2 names samples and sites, not grids
  and cells.
- **Network contract:** Concordance text only, by operator ruling: 5.4's
  H toll (ruled 2026-11-05), 6.2a's viability test (the ruled completion
  test of 2026-11-04), a note on 2.10 and §0's status line. No wire, layout or law
  version changes; the code follows in later changes.

### Each Concordance signing is an event on the hypha's ledger, and alignment is read from it (track 24 B2; Concordance §0)
- **Network contract (v0.4.0's break):**
  - **Ledger layout:** a new receipt citation, `Cite::Signing` (tag 16):
    the kernel's key, the domain it is the sitting reserve of (`[0]` none
    or `[1][domain 32]`), and the verdict as mode 8 writes it. It is
    sealed under the existing `Domain` act kind. A build before this one
    does not read a receipt that carries it.
  - **Wire:** `ACT_LAWS` mode 8 now takes the court and seals the verdict
    before it answers; a verdict the line cannot seal answers the new
    refusal `Unrecorded` (tag 7). New mode 9, `[ACT_LAWS][9]` or
    `[ACT_LAWS][9][kernel 32]`: the alignment reading. The statement
    gains `laws alignment`, MUST for the kernel profile and MAY for the
    mind profile.
- **Recorded:** a signing act whose signature reconstructs
  (`Reconstructed::signing`) is judged (`clauses::judge`, which takes only
  a reconstructed act) and recorded, admitted or refused
  (`Court::hear_signing`). A `NotSignatory` is held on the ledger as the
  residue naming both sides. An act whose signature does not verify is
  recorded nowhere. A re-signing is a new event.
- **Read:** `Court::alignment` derives, per kernel and clause version,
  converses, vetoed, collides (with the residue) or unread from the
  signing events the hypha holds (its own line, hot and in its eras, and
  every peer receipt a signature reached), the latest event per clause.
  Nothing is stored beside the events. A peer's signing receipt stays hot
  past its signer's fold. A stroke spent answers `StrokeSpent`.
- **History:** v0.3.0's signings (netstratum and xylarium, Concordance
  version 4, by the retired text hash) are served beside the reading as
  history (`clauses::history`), never admitted as signatures.
- **SDK:** `signing` and `house` export the reading and mode 9; the house
  example prints its alignment after signing; the live KERNEL read checks
  that a clause signed with a fixture presented wrong collides with its
  `NotSignatory` residue and that every other clause converses.
- **Next:** `NotSignatory` at the universal core tube (B3, B4).

### MU-ECONOMY specifies the economy as motions, v0.5.0's headline (docs only)
- `docs/MU-ECONOMY.md` is rewritten onto the model ruled 2026-11-05.
  There is no flat ledger: receipts are per-signer lines in a causal
  partial order, never totalised, with TUniversal height the only shared
  extent. A holding is a coordinate `origin / h : S:G:T:H : K` whose lanes
  are vectors of exact modes (`economy::modes::Modes`). A payment is a
  motion's advance, an event citing its causes; a balance and the supply
  are readings. The four proofs (POUW, POWV, POWC, POWR) are relations
  between motions. A payout is proven by re-traversal, and one closure is
  one point network-wide, so a second mint of it is burned. Between courts
  the Note settles; between universes μ crosses only at the universal core
  tube.
- §9 becomes "Built today, and what v0.5.0 changes": `economy::Mu`,
  `Treasury`, `Entry` and its eleven kinds, `Arm`, the escrow, pay,
  settlement, fold and verification paths, each marked with what replaces
  it. §12 lists nine points the model leaves to a ruling. §10.4 announces
  the shared economic law form as proposal 78.174, not drafted.
- The document goes from 936 lines to about 770: restated rulings are
  cited instead (Concordance 5.8 and 5.9 for gas and the map, 78.149 for
  discharge, 47.4 for equivocation), the v0.3.0 defect note and the
  self-correcting paragraphs are struck. Section numbers cited from code
  (§3.1, §3.2, §5.1, §5.2, §7.1) are kept.
- ROADMAP › Now gains 1b, "v0.5.0: the economy as motions".
- **Network contract:** none in this change. v0.5.0 itself changes the
  wire (μ as modes) and the persisted layout.

### `forms::coordinate` advances a body in the bell, solves an event's bell, and merges and splits two bodies exactly: `FORMS-COORDINATE` v2 (proposal 78.173 fixtures 7–10)
- **Network contract:** no wire layout, act or ledger layout changes. A
  law version changes: `FORMS-COORDINATE` is v2 in `forms::LAW_IDS`, the
  SDK's `FORM_LAW_IDS` and the clause register, so the clause served on
  `ACT_LAWS` mode 7 and named in a kernel's signing act (mode 8) is
  `FORMS-COORDINATE` v2. Its fixtures are unchanged (1a, 2, 4, 5, 6).
- **The advance** (`Coordinate::advance`, `Rates`): each lane
  `Lᵢ(t) = Lᵢ(h) + rᵢ·(t − h)`, `K` re-read as `⌊|L_H(t)|⌋`; it composes,
  and `t < h` is refused `Backward`.
- **An event's bell** (`Coordinate::bell_of`, `EventBell`): `h + (v − L)/r`,
  exact and possibly between whole bells; none on an unmoving lane
  (`Unmoving`) or behind the body's bell (`Behind`).
- **The merger** (`merge`, `Merger`): the holding-weighted lanes at one
  origin and one bell, `K` re-read, with each body's offset per lane,
  whose holding-weighted sum is zero; refused `OriginsDiffer`,
  `BellsDiffer` or `NoHolding` (`MotionRefusal`).
- **Fission** (`fission`): the merged point, the holdings and the offsets
  give both bodies back; offsets no merger left are refused `Unbalanced`
  with their sum per lane.
- **Fixtures:** 78.173 §5 states fixtures 7–10 as exact inputs and
  outputs, their statements agreed with netstratum on 2026-11-05;
  claimed in `forms/src/tests/motion.rs`, with the composition and the
  inverse over generated cases in `forms/src/tests/properties.rs`. The
  register does not serve them: its fixture form has no operation or
  object for them, and adding one is a wire change left to the clause's
  drafting.
- **Open:** re-origin across a throat, and gaps (they wait on the passage
  flux on T).

### Concordance draft 4 approved: the boundary law, written into 4.1, 6.2a, §0 and 0.1 as version 5 (docs only)
- The maintainer approved draft 4 (MU-CONCORDANCE §11, proposed
  2026-11-02) on 2026-11-05. Its text is written into the agreement as a
  new version: 4.1 becomes THE BOUNDARY LAW (the universal core, not the
  ancients, is the uplink the hypha enforces at the tube); 6.2a's
  touchpoint splits into keys for a key's own acts and topology for a
  universe's; §0 gains the CLAUSE BY CLAUSE paragraph, noting
  clause-by-clause signing is built (track 24 B2, PR #109) while
  `NotSignatory` at the universal core tube is track 24 B3/B4, not yet
  built; 0.1's Network and Kernel-kept rows and second reading bullet
  move to the new wording. The status line is now version 5
  (2026-11-05), and a kernel binds to it as it signs, at v0.4.0's
  release candidate. §11 is kept as a short record of the approval
  (its "UNCHANGED, and why" list stays); §12 (draft 5) is untouched,
  held for netstratum's pole-basis wording of 2.11.
- ROADMAP, MU-HOSTING §5a and the cross-references they carry no longer
  read draft 4 as pending: track 24's dependency on the ruling is
  resolved, and B3's shared laws and B2's per-domain clause sets proceed
  as engineering.

### A kernel signs the Concordance clause by clause, by reconstructing each clause's fixtures; the byte hash retires (track 24 B2; Concordance §0)
- **Network contract (v0.4.0's break):** `ACT_LAWS` modes 7 and 8 and
  the statement (mode 6) change layout. A v0.3.0 kernel's hash request
  reads the register's bytes, which no hash decoder reads, and its
  signature over a hash reads as no signing act (`Malformed`) or fails to
  verify (`Unsigned`).
  - **Mode 7** serves the clause register: `[0][count][clause …]`, each
    clause `[law id][version u16 LE][standing][fixture count][fixture …]`
    (standing 0 network, 1 kernel); or `[1][clause name]` where the hypha
    cannot read a clause of its own register.
  - **Mode 8** carries a kernel's signing act: `[kernel key 32][count]
    [clause …][sig 64]`, each clause its content address (name and
    fixtures' inputs), its assent (0 enabled, 1 vetoed) and, enabled, the
    kernel's reconstruction of each fixture's object; Ed25519 over
    `"mycelium concordance v2" ‖ key ‖ clauses`. The reply is
    `[0][count][(clause name, assent) …]` or `[1][refusal]`: `Malformed`
    (0), `Unsigned` (1), `UnknownClause` (2), `Repeated` (3),
    `NetworkVetoed` (4), `NotSignatory` (5: the clause, the fixture, the
    object presented and the hypha's), `Unregistered` (6).
  - **The statement** carries `clauses`, `[count][(law id, version u16
    LE) …]`, where it carried the 32-byte hash; the single-profile form
    of v0.3.0-rc.1 is no longer read.
- **The clause register** (`physics::laws::clauses`): each clause is a law
  form's id and version and its joint-read fixtures as exact inputs,
  addressed by that content written out, never a digest (A21); the hypha
  reconstructs each fixture through `forms`. Registered: `FORMS-PASSAGE`
  v3 (LAW-PASSAGE v2's four fixtures) and `FORMS-COORDINATE` v1
  (LAW-COORDINATE v1's order, eight sexagesimal sectors, seven `within`
  readings, two ranges). The passage is a network clause (standing 0),
  never vetoed. The coordinate is registered at kernel standing (1),
  signable and vetoable, until 78.173's fixtures are joint-read agreed
  and its clause is drafted: a clause whose fixtures no other kernel has
  read is not made unvetoable. A wording change of the Concordance
  changes no clause. The types are `ConcordanceClause` and
  `ConcordanceClauseId`, named apart from the rule language's clause
  (`transducer::sentence::form::clause`). The library depends on `forms`.
- **Retired:** `physics::laws::concordance` (`CONCORDANCE_HASH`,
  `CONCORDANCE`, `ConcordanceSignature`, `ConcordanceRefusal`,
  `verify_concordance_signature`, `SIGNATURE_LEN`), `build.rs` and its
  blake3 build dependency, and the statement's `concordance_hash`.
- **SDK:** `mycelium_sdk::signing`, a kernel's helper: `reconstruct_fixture`
  (the SDK's own reconstruction through `forms`) and `sign_concordance`,
  re-exported in `house`. The KERNEL profile's `laws concordance` reads
  the served register equal to the SDK's; `laws signature` signs it by
  reconstruction, admitted on every clause, and presents one fixture's
  object wrong, refused `NotSignatory` naming that clause and fixture.
- **Next:** the signed clause set recorded per domain, and `NotSignatory`
  applied at the universal core tube (B3, B4).

### The receipt records typed by sense (track 25, X0 stage 2; #28)
- **Network contract:** no wire byte, act, ledger layout or signed byte
  changes. Each typed field is the same 32 bytes, read in and written out
  by `of_bytes` and `bytes`.
- **The record fields carry the digest types.** `Parent::hash`,
  `KStepReceipt::chain_hash`, the fold `Root::hash` and the two hashes of
  `Admitted::Equivocation` are `Predecessor` (link).
  `SignedPrefix::prev_receipt_hash` and `KStepReceipt::prev_hash` are
  `Commitment`. None has `==`, so no bare comparison of a record's digest
  can be written.
- **`PartialEq` removed** from `Parent`, `KStepReceipt`, `SignedPrefix`,
  `Root`, `Era`, `Admitted` and the fold `Proposal`. New named readings:
  `Parent::of` (the citation naming a held receipt) and `Root::restates`
  (one place, one height, a chain continuing across the two hashes).
  Moved with the fields: the packed line's anchors and predecessors
  (`chronometry::pack`), the archive's era comparisons (`era_locates`,
  address sense), `verify_chain_integrity`, the fold's witness check,
  and the snapshot's ledger rows read against their receipt.
- **Tests:** whole-record comparisons are rewritten to the named
  methods (`one_receipt`, `one_run`, `one_root`, `one_era`,
  `one_archive`, `one_proposal` and `one_prefix` in the test fixtures),
  or to the coordinates written out. The SDK's receipts conformance
  sample seals its receipt and checks that the decoded one re-derives.
  No claim was dropped.
- **Open:** `Cite::Root`'s hash is still bare.
- **The vertex admits reconstructed acts.** `ReceiptChain::admit` takes
  a `Reconstructed<KStepReceipt>`. `Court::admit_uplink` and
  `admit_chained` take a `Reconstructed<SignedFold>`, and
  `Court::admit_domain` a `Reconstructed<SignedDomain>`. The new
  reconstructions `Reconstructed::fold` and `Reconstructed::domain`
  re-write the signed bytes from the act's own coordinates and verify the
  signature over them. `Court::hear_uplink` and `Court::hear_domain`
  reconstruct the act as heard, answering `Unsigned` where it does not
  reconstruct; the `ACT_UPLINK` and `ACT_DOMAIN` judges call them. The
  answers are unchanged. Claim `vertex_admits_only_a_reconstructed_fold`
  (`compile_fail`), registered in the drift register.
- **Stratum roots compared as locators.** The court's confirmation of a
  peer's ruled digest (`confirm_peer_holds_on`) compares the stratum root
  it recomputes from its members, and the root it recorded for a heard
  word, with the peer's by `Locator::locates` (`locates_stratum`). A mind's
  rest on a stratum fetched for nothing (`CourtPace::starved`) holds the
  court's root as a `Locator`. A root that is not 32 bytes locates
  nothing, where before two such byte strings compared equal.
- **Not registered:** the drift guards of MU-ANTI-FLATTENING clauses 12
  and 14. Neither has both a guarding type and a claim in the tree:
  clause 12's `Turn` has public fields and the refinement it would refuse
  is deleted, and clause 14's `Court::restore` returns `bool`, not a
  `Reconstructed<State>`. MU-ANTI-FLATTENING *Drift guards* says why.

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

### Two cargo commands check every change; the gate is workspace tests (workflow standard)
- Coordinator ruling: every developer is an AI agent and no person is in
  the loop; every change is checked by `cargo clippy --release
  --workspace --all-targets -- -D warnings` and `cargo test --release
  --workspace`, and a change to the wire or the mesh also runs
  `scripts/quickstart` (AGENTS.md, *Checks*).
- The gate moves from `xtask/` (removed) into the workspace member
  `gate/`, read by its tests alone: the reading bound, the Euclidean
  vocabulary, the numbers gate (its N6 inventory written by
  `numbers_refuses_nothing_in_tree`) and the drift register, each a test
  over the tree. Refusals are unchanged: a seeded tree refused by the old
  gate is refused by the new tests, row for row.
- Every member takes the one lint table (`forms` joins it).
- `scripts/check` and `scripts/ship-scoped` are removed. The durable
  drill is the release-only test `tests/durable.rs` (the script and the
  example client are removed); `drill-undurable` is still its negative
  control.
- `scripts/quickstart` runs `scripts/observer once` over its mesh and
  `scripts/timeline` over the client's traced journal, and fails where
  either does. The observer's `once` now fails on a reading that does not
  answer, honours `CARGO_TARGET_DIR` and reads a mesh with no rendezvous;
  the timeline reads the trace line as written today (no wall-clock
  field), costing each reader by its spend.
- A gate test holds MU-EQUATIONS §11's forms rows to `forms::LAW_IDS`.
- A gate test holds the documents to MU-STANDARDS 7.1: every repository
  path cited in backticks in `docs/`, README, AGENTS, CONTRIBUTING,
  ROADMAP and GLOSSARY exists, or its line says it was removed. The 26
  stale citations found are rebound.
- The Unreleased section is fragments: one file per change under
  `changes/`, holding exactly its entry's markdown, assembled into the
  release's section at release time. The entries that stood under
  Unreleased are moved there unchanged. A gate test refuses a fragment
  that is not a markdown entry opened by its `### ` heading.

### A turn has one constructor, `Turn::of(r, h)`: MU-ANTI-FLATTENING 12's guard registered (T3, #26)
- **Network contract:** none. No wire byte, act, ledger layout, task,
  word or chart address changes.
- `transduce::Turn`'s index and period are private; `Turn::new` is
  `Turn::of(r, h)`, and a phase reversed is `Turn::reversed`. A turn is
  read back on a period only by `Turn::lands`. The `compile_fail` claim
  `turn_has_one_constructor` (a turn written by its fields; a field
  read) enters the drift register (`xtask/lint/drift.rs`).
- **Library surface change, inside v0.4.0's break:** a caller writes
  `Turn::of(r, h)` where it wrote `Turn::new(index, period)`.

### The Conformal law accepts a motion stated as turns, checked in ℚ/ℤ (T3, #26; MU-HOSTING §5a.10)
- **Network contract:** none. No wire byte, act, verdict encoding, law
  version or ledger layout changes. A turn-valued motion is read in
  process and through the SDK; its wire form rides the crossing record
  (track 25, X1). The versor transition's encoding is unchanged.
- `physics::laws::verify_turns` reads a `TurnTransition` (a frame's
  periods, a `TurnMotion` of one turn per phase, each body's place before
  and after) under `Law::Conformal`: each place is carried phase by phase
  by addition mod 1 and compared in ℚ/ℤ, and a disagreement is the
  residue `t_claim − t_rec` per phase (`TurnBreach::NotCarried`). No turn
  becomes a rotor; only multiples of a quarter turn have exact rotors
  over ℚ (Niven), and every turn is checked as a turn.
- Refusals by name: `LeavesPeriods { phase, turn, period }` (a turn whose
  index on the phase's period is no whole site), `Rank`, `PlaceRank`, and
  `Unread` where a composed period passes the machine word.
- `Turn::plus` (the sum in ℚ/ℤ on the lcm of the two periods, the period
  of the composed orbit), `Turn::less` and `Turn::is_whole`;
  `TurnMotion::then` and `TurnMotion::carry`.
- Claims `conformal_accepts_turn_valued_motion`,
  `turn_motion_composes_by_addition_mod_one`,
  `verify_turns_refuses_turn_leaving_frame_periods`,
  `verify_turns_names_residue_of_uncarried_place`.
- **SDK surface, additive:** `mycelium_sdk::house` re-exports
  `verify_turns`, `TurnMotion`, `TurnTransition`, `TurnBreach`,
  `TurnVerdict`, `Turn` and `Enclosure`.

### The chart reads a section as turns of its own periods; the conformal lift retires from `section_moduli` (T3, #26)
- **Network contract:** none. No wire byte, act or ledger layout changes,
  and no chart, word, task or clause address moves: every chart reading
  is unchanged, axis for axis, and the oracle golden reproduces.
- `blade::SectionTurns` lifts a section by enclosure: its span along the
  plane is its frame, one phase per plane axis, and each site the turn of
  its offset from the span's least corner (`Turn::of`). The chart
  (`SectionTurns::chart`) reads the cube complex of the support the turns
  land on (`Turn::lands`) and each second moment in the frame's own
  gauge (a turn's offset from the mean turn times the period, with each
  round's extent), laid out by `Harmonics::axes`. `blade::section_moduli`
  reads it; the conformal blade (`lift`'s sphere, quartic moments and
  towers) is no longer built for a chart.
- Claim `chart_reads_sections_as_turns`: all 4 095 non-empty sections of
  a `3 × 4` window, a ring, a bar on each axis and a section at negative
  coordinates, in place and translated, read the lift's chart exactly,
  and each turn lands on a site of its period.
- The soak's chart instrument (`mycelium-soak --chart`) times the turns,
  the topology and the chart, and reads the moments from the turns; its
  trace names the stages `turns` and `chart` where it named `lift` and
  `harmonics`.

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

### The section's planar tuples held as section sites (track 26 S4d, stage 2b)
- **`SectionLink` is a named type.** The tuple `(i64, i64, u8, u8)` is
  now `sentence::SectionLink { site, before, after }`: a section site
  (`spelled::SectionSite`) and the two symbols. It orders by the site,
  then the symbols, as the tuple did. Every carrier that spelled the
  tuple reads it: the element's chain (`CanonicalChain`, `sub`), the
  receipts, the form's demands and remainder, the field, the court, the
  mind and the soak. The keys of a section site (`family::site_key`,
  `sentence::section_site_key`, `field::remainder_site_key`) take the
  site. A chain's least corner is one reader, `element::corner_of`,
  and `ChainAt::corner` is a section site. Claim
  `section_link_holds_section_site_and_transition`. Every codec writes
  the coordinates in the plane's order and then the two symbols, so no
  wire, tape, task, word or chart address changes.
- **`PaintedSite` holds the painted triple.** `(i64, i64, u8)`, its
  alias `family::Painted` and `incongruence::Cell` are now
  `spelled::PaintedSite { site, symbol }`, ordered as the triple was.
  `raster::painted_sites` is the sensor's one reading and returns it;
  `raster::painted_of` is retired. The shells, the layers, the orbit,
  speech (`Speech.painted`, `remainder`, `statement::Spoken.painted`),
  the germ (`germinate`, `apply`), the local section, the incongruence
  reader and `cognition::frame_of` read it, and a scene's span is
  `SectionSite::span_of`. Test fixtures write a scene once through
  `sentence::tests::fixtures::painted`. Claim
  `painted_sites_round_trip_through_their_span`; the test
  `painted_of_omits_ground` is renamed `painted_sites_omits_ground`
  with its unit. No wire, tape, task, word or chart address changes.
- **Open:** the chart's coordinate pairs, the anchor's selection
  algebra (footprint and scene together) and the clause reader's planar
  tables. Stage 3 proposes naming the rule language's `Coord` variants
  by perspective role (ROADMAP track 26 S4d).

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
  and `symbol_set_refuses_differing_test_width`. No
  wire, task, word or chart address changes; `scripts/check` is green,
  2090 tests passing. Open: the section's planar tuples (`SectionLink`,
  the painted triple, the chart's coordinate pairs).

### The rule language's coordinates named by the plane axis they are read along (track 26 S4d, stage 3)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no clause or word address moves: the clause codec writes
  each coordinate in the byte its planar name was written with
  (`Extent` 1 and 2, `Least` 3 and 4, `Greatest` 5 and 6).
- **SDK surface change, inside v0.4.0's break** (coordinator decision).
  `mycelium_sdk::rule::Coord::Height` and `Width` are
  `Coord::Extent(PlaneAxis)`, `Top` and `Left` are `Coord::Least(PlaneAxis)`,
  and `Bottom` and `Right` are `Coord::Greatest(PlaneAxis)`, with
  `PlaneAxis::First` for `Height`, `Top` and `Bottom` and
  `PlaneAxis::Second` for `Width`, `Left` and `Right`. A client writes
  `Coord::Extent(PlaneAxis::First)` where it wrote `Coord::Height`.
- `PlaneAxis` (`transducer::perspective`, re-exported by
  `mycelium_sdk::rule`) names the first or the second space axis of
  `Perspective::plane` by its place in the plane, never as a row or a
  column. Claim `coord_round_trips_and_reads_its_planar_byte_by_role`.
- **`About::Grid` is `About::Sample`** (SDK surface, `mycelium_sdk::rule::About`):
  a carriage or an echo read about the sample's own frame, beside
  `About::Body`. The clause and element codecs keep their bytes (a
  carriage's `about` 0, an echo's tag 9), so no clause or word address
  moves.
- **The section's planar names renamed by function** (SDK surface):
  `mycelium_sdk::task::TaskRasters` is `TaskSection`;
  `TaskEnvelope::of_rasters` and `PostedTask::of_rasters` are
  `of_section`; `mycelium_sdk::rule::Step::grid_motion` is
  `Step::symmetry_motion`; `seat_mind::task::grid_steps` is
  `symmetry_steps`; `family::grid_of_painted` is `sample_of_painted`;
  `painted_within_grid` is `painted_within_sample`. Tests renamed with
  their units: `symmetry_motion_carries_every_site_to_index_image`,
  `symmetry_motion_exchanges_non_square_shape`,
  `symmetry_steps_reads_task_before_seat`.
- The frame acts' planar fields, `statement::Spoken.frame` and the
  "grid" test names were left open here and are built in stage 2d above.

### The chart's pairs, the anchor and the clause tables held as section sites (track 26 S4d, stage 2c)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no task, word, chart or clause address moves. Every codec
  and key writes a section site's coordinates in the plane's order,
  `i64` each (`SectionSite::put_canonical`, `take_canonical`): the bytes
  a planar pair was written in.
- **The chart's supports.** `element::positions` returns section sites,
  and `pullback`, `blade::shape_of_section`, `section_moduli`,
  `sentence::incidence_of`, `orbit::positions_of`, `ring_dist_peel` and
  the confluence reader (`gathered`, `carried_onto`, `read_pair`) take
  them. The blade's sections hold section sites; its plane readers are
  `on_plane` and `off_plane`.
- **`on_grid` retired.** A site of the family's base is read onto the
  section by `SectionSite::of_family`; `scene::restrict` lays a region
  from its least corner by `SectionSite::span_of`. The test
  `on_grid_refuses_site_without_grid_plane` is
  `of_family_refuses_site_without_plane`.
- **The anchor's selection algebra, together.** `anchor::Evidence` and
  `EvidenceOf` hold the footprint as section sites and the scene as
  painted sites; `Selection::eval`, the derive, the carrying set,
  `candidates::signature`, `beam_minima`, `element::measure_lanes`,
  `measure_supports`, `keys::footprint_key` and the evidence reply read
  them. The face neighbours and the extremal codes are generated from
  the plane's rank; `Ext`'s code names an axis of the plane and an order,
  as its byte always read. Claim `section_site_round_trips`.
- **The clause reader's tables.** `statement::Seen` holds each body's
  painted sites and the owner of each site of the sample at its offset in
  canonical address order (`SectionSample::offset_of`), never a table of
  rows. `extent_of` is retired for `statement::corners_of`, the least and
  the greatest corner over `SectionSite::span_of`; the landed lists, the
  regions, the joins and the crop read section sites, and a line of the
  plane is read along each axis, the last first. `acts::pattern_key` and
  `stamped_key` write a centred pattern of painted sites. Claim
  `corners_of_reads_least_and_greatest_on_each_axis`.
- **Open:** the act reading's own index cells (`acts::cells_of`, the
  residue's pieces, `owners`, `step`, the echoes and the bends) read
  extents through `statement::corners_of` and keep `(row, column)`
  indices inside the act reading; the frame acts (panes, tiles, the
  square's symmetries, the scaling) stay planar by their acts.

### The act reading's last planar carriers held by plane axis (track 26 S4d, stage 2d)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no task, word, chart or clause address moves: the panes
  keep their split byte (1 along the plane's first axis, 0 along its
  second) and the tiling its counts in the plane's order.
- The act reading's own cells (`family::acts`) are painted sites and
  section sites: the residue, its pieces (connected through a shared face
  generated at the section's rank), the owner of each site, the
  connectors, the echoes, the slide and the bends. The `(row, column,
  symbol)` adapter is removed.
- `Act::Panes { along_rows: bool }` is `Act::Panes { along: PlaneAxis }`;
  `Act::Tile { rows, cols }` is `Act::Tile { blocks }`, one count per
  plane axis; the speaking of panes, tiles, the square's symmetries and
  the scaling reads the plane through the perspective. They stay rank 2.
  Claims `frame_act_round_trips_and_reads_its_planar_bytes_by_axis`,
  `extract_rule_reads_panes_along_first_plane_axis`.
- `statement::Spoken.frame` is the output's extents per plane axis. Claim
  `speak_clauses_frames_output_by_plane_axis`.
- **SDK surface change, inside v0.4.0's break:** `Speech::grid` is
  `Speech::sample`. A client writes `s.sample()` where it wrote
  `s.grid()`.
- Test fixtures and names by function: `grid(rows)` is `digits(rows)`;
  `speech_sample_refuses_site_outside_frame`,
  `germinate_refuses_unchanged_scenes`,
  `slot_of_axis_stands_for_each_base_axis`.
- **Remaining planar, outside the act reading:** `Speech.frame` and
  `perceive::frame_of` (read with the measured `Frame`, a defect), and the
  ray of `Act::Slide` and `Act::Extend`, a clause field written as a pair.

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

### Mutation audit retired (2026-11-04)
- The mutation audit, ruled theatre, is retired: the catalogue and the
  `gate mutate` subcommand are removed from `xtask`.

### LAW-PASSAGE v2: the linear passage struck (2026-11-05)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes. The shell `FORMS-PASSAGE` steps from version 1 to 2.
- Operator ruling: the reach passage `R ↦ 2H − R` is struck. Proposal
  78.172 is amended to the passage law accepted by both operators: a
  body's state is an even multivector `q`, and one crossing in plane `k`
  is the one-sided product `q ↦ v_k·q` with `v_k = H_k·e_Zk + R_k·e_Qk`,
  `e_Zk² = +1`, `e_Qk² = −1`. Its four fixtures were agreed on
  2026-11-05 by mycelium, netstratum and xylarium (status: Fixtures
  agreed, MU-RELEASE §3a step 4).
- `Throat::pass` and version 1's fixtures 1, 2, 3 and 6 are removed with
  their tests. `Throat::verify` refuses a stated moved reach (`H²/R`
  among them) `NotLawful`, naming the claim against the reach carried
  in. `pass_share` stays as a share read from a plane's other axis, not
  the passage.

### LAW-PASSAGE v2's fixtures are claims in the algebra (2026-11-05)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no law version moves.
- The four joint-read fixtures of proposal 78.172 §5 are claims on
  `Clifford::product` in `src/transducer/cga/tests/passage.rs`, each
  frame plane's closed pole squaring to +1 and its open pole to −1: the
  null direction annuls twice (`v·v·q = 0`); one crossing takes an even
  state to degrees 1 and 3 only; `[e_Z1Q1, e_Z1Q2] = −2·e_Q1Q2`;
  `(e_Z1Z2 + e_Q1Z2)² = 0`.
- Beside them, over every frame plane the algebra holds: every horizon
  is one-way, and passages in different planes anticommute.
- `forms::passage` does not hold the product: `forms` cannot reach the
  algebra without a dependency cycle, and 78.172 says where the
  fixtures stand.

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

### `forms::coordinate` holds LAW-COORDINATE v1, the coordinate a kernel addresses its bodies by (proposal 78.173; track 24 B3)
- **Network contract:** no wire byte, act, ledger layout or reading
  changes. A new shell, `FORMS-COORDINATE` v1, joins `forms::LAW_IDS`
  (and the SDK's `FORM_LAW_IDS`).
- **The coordinate** `origin / h : S:G:T:H : K`: `Coordinate`, `Origin`
  (the genesis and epoch, then the enclosures written out as
  coordinates, never a digest) and `Local` (the bell, four signed exact
  lanes S, G, T, H, and `K`). Its derived order is the canonical key, and
  its writing is TPoint's writing where there is no enclosure and every
  lane is non-negative.
- **The sexagesimal reading:** `Sector`, the exact interval at a depth in
  base 60 (`BASE`, classed `EXTERNAL` by the numbers gate), nesting by
  depth, with places and a writing (`0;08,34,17`) for display only.
  `Coordinate::within` decides on the sector's exact bounds and refuses
  two origins `OriginsDiffer`.
- **Neighbours:** `Bodies` holds coordinates in canonical order, and
  `Bodies::range` returns a range on one lane by two bisections,
  claimed equal to the test of every body.
- **Next and open:** `TPoint` is not rewired onto the form in this
  change. The signed-to-issuance map at the vertex is open.

### The algebra's signature is data: `Cl(n+1, 1)` within `Cl(p, q)` (2026-11-05)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no law version moves. Every value the wire carries is in
  `Cl(n+1, 1)`, and `Mv::from_bytes` reads it there; the encoding carries
  no signature. A value of another signature has no wire writing until
  the court reads one, which will be a v0.4.0 break of its own.
- Operator ruling: the court's multivector is generalised from
  `Cl(n+1, 1)` to `Cl(p, q)`. `Mv` carries the mask of its negative
  generators beside its `n` (`Mv::negatives`), and the product's metric
  sign is read from it: −1 for each shared generator the mask names.
- Every value made by a constructor, a product or the wire carries `e₋`
  alone, so `Cl(n+1, 1)` is unchanged: the product over every blade pair
  at `n ≤ 4` equals the conformal metric's, and the oracle golden
  (`tests/oracle.golden`) reproduces line for line.
- `transducer::cga::Clifford` holds a value of any other signature. It
  composes only through `product`, `sum` and `difference`, each of which
  refuses two signatures that disagree on a generator both algebras hold
  `SignatureRefusal::Mismatched`, naming those generators; a mask past
  `MAX_N` is refused `Malformed`. It has no unchecked product (a
  `compile_fail` doc test).

### The exact algebra stands in its own crate, and `forms::passage` holds LAW-PASSAGE v2 whole (track 24 B3; Concordance §12)
- **Network contract:** no wire byte, act, ledger layout or reading
  changes; the oracle golden reproduces. The coefficient codec, the
  generator masks and the canonical writing are the same bytes. The
  shell version `FORMS-PASSAGE` steps 2 → 3 (the law is unchanged; the
  shell now holds its product). The Concordance's bytes change with the
  §12 landing note, so `CONCORDANCE_HASH` changes with them.
- **The `algebra` crate** (`algebra/`), below both `forms` and the
  library, depends on the exact rationals alone: `Mv`, the generator
  masks, `Clifford`, `SignatureRefusal`, the product, the wire codec of a
  value of `Cl(n+1, 1)`, `projective_ratio`, the key-component codec, and
  `Address`, `Cited`, `tag_of` and `TAG_VERSOR` (which `Mv::key` writes).
  The library re-exports every item at its old path
  (`transducer::cga::*`, `transducer::address::*`).
- **What stayed in the library:** `weight_of`, `is_versor` and the rest of
  `transducer::cga`; and a value's conjugacy class, which reads the versor
  test and writes the network's coordinates, now the trait
  `transducer::cga::Conjugacy` (`class_invariants`, `holonomy_key`,
  `class_key`), implemented for `Mv` with the same bodies.
- **`forms::passage::pass(v, q)`**, the one-sided product `v·q`, refusing
  `NotDirection` (a `v` not degree 1), `NotEven` (a `q` holding an odd
  degree) and `Mismatched` (two signatures disagreeing), each with its
  reading. The four fixtures of 78.172 §5 move to
  `forms/src/tests/crossing.rs` as claims on `pass`. `forms` re-exports
  the algebra (`forms::algebra`) and still builds alone
  (`cargo build --release -p forms`), so a kernel linking it by path gets
  the algebra with it.
- **SDK surface:** every SDK item keeps its path and type (`Mv` is the
  same type). The SDK does not re-export `Conjugacy`, so `Mv`'s three
  class readers are no longer reachable through the SDK alone; a caller
  imports `mycelium::transducer::cga::Conjugacy`.
- **The gate** reads `algebra/src/` with the other roots (reading bound,
  Euclidean vocabulary, numbers, drift).

### A fold root's signatures cover its place and height (track 24, #91)
- **Network contract**, inside v0.4.0's break:
  - **The root's signing domain changes.** The home's and the witness's
    signed positions over a fold root (47.3) carry, as the payload of
    their 47.1 prefix, the root written out as coordinates (A21):
    `"mycelium root v2" ‖ K u64 ‖ i u64 ‖ height u64 ‖ chain hash 32`,
    `K ‖ i` and the height big-endian (`Root::signed`). Version 1,
    unnamed, signed the bare 32-byte chain hash, and no longer verifies:
    a v1 root proof is refused `REFUSE_RECEIPT`, and a v1 co-signature
    seals no fold.
  - **No wire shape changes.** `ACT_FOLD`'s proposal, the witness's
    co-signature, the root answer of `ACT_RECEIPTS` and the `LEAF_ROOTS`
    leaf keep their layouts: each section's payload is length-prefixed,
    and only its content changes, from 32 bytes to 72. Root proofs
    persisted under v1 do not verify; a recut wipes them.
- `Reconstructed::root` reads the root back from each signed position
  (`Root::of_signed`), compares the place and the height the proof
  states with the signed ones as coordinates, and binds the hash. A root
  restated at another place, height or hash is refused `REFUSE_RECEIPT`,
  counted `root_refused`, and adopts nothing, whether or not the court
  holds a receipt at that place. The home (`Court::commit_fold`) and the
  witness (`ACT_FOLD`) check each other's position against the proposed
  root the same way.
- Claims `root_at_another_place_refuses_without_held_receipt`,
  `root_at_another_height_refuses`; the #88 claims stand.

### A gap past the capacity closes through a fold root (track 24, #88)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes. The catch-up ask and the root answer of 78.147 carry it.
- A fold root is admitted only as a `Reconstructed<RootProof>`
  (`Reconstructed::root`): both signed positions verify, and each one's
  commitment binds the root. A forged root is refused by name, counted
  `root_refused`, and adopts, sheds and evicts nothing.
- A back-fill page that stops on the held bound `C` asks the range again
  from below the signer's unreached floor, which a court holding the
  root's proof answers with the root.
- The root's signatures reach the receipt held unreached at its place
  where it is the one the root names, and every receipt held below it;
  they are released and read in line order. Receipts held unreached that
  a root's shed takes off the chain leave the signed-reach gate with it.
- A peer behind by more than `C` receipts reaches the signed head and
  never holds more than `C` unreached.

### The chart of task `0b148d64` re-measured: the intake charts none of its pairs (#87)
- **Network contract:** none. No engine code changes; no wire, address
  or reading moves.
- **Measured on main (5815d4e4), release build,** with the instrument
  `mycelium-soak --chart DIR NAME`, which times the intake's calls on
  each train pair by stage. `route::routes_of` routes none of the three
  pairs (`18 × 19 → 7 × 9`, `21 × 21 → 10 × 10`, `19 × 17 → 9 × 6`):
  their enclosures differ, so they wait for the turn-native chart
  (MU-DERIVATIONS D3.2, O8), and the intake charts nothing for the task.
  The `≥ 19 s` a pair was the chart of the common refinement `K_lcm`
  (14 045 links on pair 0), which no reader forms since D3.2.
- **Each pair's demand** (the output against nothing on the family's
  ground, 37, 72 and 33 links) charts in 8, 15 and 7 ms, most of it the
  lift. Its pieces chart in 6, 15 and 5 ms. The widest exact coordinate
  is 36 bits (the anisotropy's denominator on pair 0). No bignum grows,
  so the algebra is unchanged.
- **Claim** `chart_of_0b148d64_pairs_read_on_metabolic_budget`
  (`sources/arc-agi/tests/intake_chart.rs`), an observation and never a
  gate. It asserts relations only: no pair is routed, each pair's
  projection refuses `OffSite`, and each pair's transduction spends within
  the stroke its closed-form need sizes. The need, the spend, the demand's
  links and its chart are printed, never asserted (read on main: 240 of
  648, 315 of 900 and 239 of 594 reads).

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

### The observer's call test owns its observer (#78)
- `observe::live` holds its state in a `Live` value: the process's mind
  is observed through `live::process()`, which every free function reads,
  and a reader may hold its own. `Live::wait` waits on a call already
  left. `live_call_waits_while_carrier_works_inside_law` raced the minds
  of other tests, which park, enter laws and take calls on the
  process-wide observer; both observer tests now read only an observer
  they own. No wire change.

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

### Track 11's last two names ruled (#55)
- `ListingField` → `ListingCoherence`: "field" now names only the
  object, the curved nested manifold (MU-SENSING §3.7); the struct is a
  listing's coherence reading, its charts and its non-Flat count.
- `Coord::Shell` → `Coord::Cells`: a body's cell count; "shell" was
  retired with `layers::Shell` → `Layer`. The clause codec's byte (`0`)
  is unchanged.
- SDK surface: `mycelium_sdk::rule::Coord::Shell` is `rule::Coord::Cells`.
  No wire or byte change.

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

### The rest of "replay" (verification), and `tests/word_lifecycle.rs`'s "grid": names and strings left by the prose cleanup (#103)
- Names only, no behaviour change. Rounds out the two renames above with
  the instances a first pass missed: the test `assert`/`expect`/`panic`
  strings using "replay" in the verification sense across some twenty
  test files (`server/cognition/tests/{mint,ruling,relator}.rs`,
  `client/sdk/tests/rule.rs`, `bin/mycelium-server/judge/tests/offer.rs`,
  `transducer/alphabet/tests/rotor_key.rs`,
  `transducer/family/tests/speech.rs`,
  `transducer/field/tests/contemplation.rs`,
  `transducer/sentence/{tests/properties,tests/refraction,
  tests/rotor_class,form/tests/complex,pivot/tests/orbit}.rs`,
  `transducer/tests/factor.rs`, `tests/word_lifecycle.rs`), each reworded
  to "reconstruct"/"reconstruction" to match the reconstruction calls
  and doc comments they sit beside.
- The same class as `glue_replay_refused` above, caught on a second
  re-grep: the observed reading `glue_unread`'s description
  (`observe::REPLICATION_AXES`), the SDK example's `SOLVE_UNREAD` label
  (`sdk/examples/client.rs`), a stale trace label on the step that calls
  `docket_reconstruction` (`"court stroke: docket replay"` →
  `"court stroke: docket reconstruction"`,
  `server/cognition/court/reconstruct.rs`), and a census carrier name
  that lagged its own local, already named `reconstruction_memo`
  (`"replay_memo"` → `"reconstruction_memo"`, `server/cognition/
  carriers.rs`).
- `tests/word_lifecycle.rs` also carries the cognition-sense "grid":
  an `expect` string, an `assert_eq!` string, and the local bound to
  `spoken.sample()`, reworded to `sample` alongside the file's own
  `grid`-pair-chain `expect` string, the same pattern fixed in the
  "grid" entry above.
- **Left as comments, not names or strings** (a separate, later PR):
  roughly a dozen remaining `///`/`//` comments still read "replay" in
  the verification sense (`court/verdict/reconstruct.rs`,
  `bin/mycelium-soak/delta.rs`, `economy/tests/ledger.rs`,
  `transducer/mind.rs`, `sources/arc-agi/tests/rule_language.rs`,
  `tests/word_lifecycle.rs`'s two remaining comments, among others);
  these are prose, not the names or strings this issue covers.

### "Replay" (verification) renamed to "reconstruct" — names and strings left by the prose cleanup (#103)
- **Network contract**, inside v0.4.0's break: the wire-visible refusal
  text changes, the refusal names and codes do not. `REFUSE_PIVOT_INEXACT`
  is "pivot certificate does not reconstruct" (was "...does not
  replay"); `REFUSE_UNREAD` is "the reconstruction holds a unit no
  stroke can read" (was "the replay holds..."); the server's two
  `SolveRefusal` reasons for `SolveVerdict::Unread` and
  `SolveVerdict::Unverifiable` reword "replay" to "reconstruction"/
  "reconstruct" (`bin/mycelium-server/control.rs`). A client that
  matches refusal text (rather than the refusal name or verdict code)
  updates the strings it matches.
- **Kept:** `ReceiptsRequest.replay` (`anastome::dialogue::receipts`) is
  the era-history sense (78.147: re-apply recorded receipts in order to
  read history) and is correctly named; this rename does not touch it.
- Names only, no behaviour change otherwise: the observed reading
  `glue_replay_refused` is `glue_reconstruction_refused`
  (`observe::REPLICATION_AXES`).

### "Grid" (cognition) renamed to "sample" — names and strings left by the prose cleanup (#103)
- Names only, no behaviour change. The cognition-sense `grid`/`grids`
  parameters and locals are `sample`/`samples`
  (`transducer::family::perceive`, `transducer::family::deflation`),
  matching the sensor's own `Sample`/`SectionSample` vocabulary (already
  landed for `Speech::grid` → `Speech::sample`, above). The matching test
  `expect` strings are reworded (`server/tests/route.rs`,
  `server/cognition/tests/{mint,composed,carriers}.rs`,
  `anastome/dialogue/tests/task.rs`), as are the `#[expect]` reasons
  (`transducer/family/{contact,finish}.rs`) and the local in
  `physics/axioms/tests/frame.rs`.

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
  `symbol_bytes_refuses_overflow`,
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
