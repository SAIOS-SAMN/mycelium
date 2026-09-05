# MU-RECURSION task list

Derived from `MU-RECURSION.md` chapter 41 (landing order) and the audits of
2026-09-05. One task per commit; the commit message carries the task id, the
test names, and the `test result` line of one foreground
`cargo test --release`. The auditor reproduces the cold-boot trace before
the next task lands. Status as of `main` abacf53 (2026-09-05). Full suite
at that commit: lib 280, alpha_plan 13, g_cga 6.

Legend: **done** verified on the tree; **partial** landed with a named gap;
**open** not started. "DoD" is the definition of done. "Rule" cites the
coding-standards rule most likely to be broken while doing the task.

## Stage 0 — identity (Part III, chapters 16–17)

| id | task | chapter | files / functions | status | DoD |
|---|---|---|---|---|---|
| 0.1 | M1: A1 canonical rotor (sign step, either-sign `projective_eq`), A3 reduced plane form, A4 canonical `Mv`, A2 run reduction | 16.1–16.4, 17.1 | alphabet.rs `Rotor::{normalised,projective_eq,key}`, `Word::{key,normal_form,same_form,form}`; cga.rs `Mv::{canonical,projective_eq,key}` | **done** | `a1_canonical_is_projective_either_sign`, `a3_form_is_reduced_and_eq_on_reduced`, `a4_projective_eq_either_sign_and_key`, `two_tapes_of_one_vertex_have_one_key` |
| 0.2 | M2: A5 chain canonicalisation; address composition incl. shift and contact | 16.5, 17.2 | element.rs `canonical_chain`, `form_key_bytes`; sentence.rs `germ_class_key`, `form_vertex_key` | **done** (9f545ff) | two shift families → two addresses; `with_contact` changes the address |
| 0.3 | M3: RMP1 migration | 16.8, 17.3, 33 | cognition.rs `remap_restored_addresses`, economy.rs, sdk.rs `uncrystallize` | **closed, not now** — recuts start from genesis with `state/`, ledgers, genomes wiped; reopen before any cut that must keep a ledger | — |
| 0.4 | M4: A6 exact axes with refusal, A7 invariant refinement axis | 16.6, 16.7, 17.4 | blade.rs `axes_of`, ear.rs `Harmonics::axes`, stalk.rs `Stalks::refine(inv,&[u64])`, cognition.rs `refine_chart` | **partial** — A6 done; A7 (class axes replace `Rotor::coordinate`) open | `a7_class_axes_are_invariants_not_hash`; `Rotor::coordinate` deleted |
| 0.5 | M5: class keys on `cocycle_at`, `push_cocycle` by `projective_eq` then key, `a == b` accepted | 15.4, 17.5, 23.4 | cognition.rs `push_cocycle`, `cocycle_at` fifth field, snapshot/restore | **partial** (d6d5e6f keys; `a == b` and replace-representative unverified) | same class key on all three hyphae for one dialect |
| 0.6 | M7: A9 `Sel::to_bytes`, A10 peers by vk with `pending_addrs`, A11 rendezvous by listing key | 16.9–16.11, 17.7 | verdict.rs `sel_key`, sdk.rs/server peers, mycelium-server.rs `judge` | **partial** — A9, A11 done; A10 open | `a10_two_sockets_one_vk_one_peer` |
| 0.7 | M6: A12 comprehension by factorisation (HNF on the plane, budgeted descent off) with canonical remainder | 16.12, 17.6 | sdk.rs `Epigenome::comprehend` connection arm; cognition.rs `reduce_excluding` | **open** | `a12_plane_factorisation_by_hnf`, `a12_offplane_descent_or_novel`, `a12_plane_remainder_is_canonical`, `m6_held_under_other_spelling_is_understood` |
| 0.8 | `Element::eq` removed or delegated to `same_vertex`; no `==` on `Element`/`Versor` in cognition | 17.1, rule 3 | element.rs `impl PartialEq for Element`; every caller | **open** | grep returns none; suite green |
| 0.9 | Wall-clock caps replaced by measured ratios | rule 5 | sdk.rs `rtt_ok`/`rtt_timeout`; mycelium-server.rs `note_rtt_ok`; physics.rs `persist_dwell` | **done** (4b65841, 2f76f33) — `from_secs` gone from `src/`; cap is `persist_dwell(floor)` | `persist_dwell(rtt_floor)`; no `from_secs` outside tests |
| 0.10 | Shed by measured aperture in TUniversal | 24.1, rule 5 | sdk.rs `shed_except`, `aperture_tu`; stroke ring `(stroke, tu)` | **done** on the tree (73ed1a2) — `shed_except` only reads `aperture_tu`; unmeasured aperture sheds nothing. Soak check (`shed` lines < `SYLLABUS`) waits on Cut B | `dialect_enters_on_stood_and_sheds`; nothing sheds in the first aperture after boot |

## Stage 1 — forms (Part IV, chapters 18–29)

| id | task | chapter | files / functions | status | DoD |
|---|---|---|---|---|---|
| 1.1 | Form types beside `Nerve`; `SENTENCE_TAG` v2 tape; `Word` contact and `rebase` | 18, 19 | sentence.rs `Vertex/Form/FormFace/Loop`, `encode_form_v2/decode_form_v2`; alphabet.rs | **done** (9221219, 42423d2, 9f545ff) | `form_tape_v2_round_trips`, `depth_is_one_plus_max_glued`, `rebase_is_normal_form_and_contact_in_key` |
| 1.1b | v2 tape carries `ratio` and `sections`; A20 certificates; torsion keyed by class | 19, 16.20 | sentence.rs `encode_form_v2`/`decode_form_v2` | **done** (c01503b, abacf53) — ratio, sections, certificates, and torsion on the tape; a count that does not fit remaining bytes is refused | `ratio_and_sections_are_on_the_form_key`; `form_tape_v2_round_trips` |
| 1.2 | `pair_form` / `word_form`; cells read once; faces by star + keyed occupancy + ring-1 probe | 20, 21 step 2, rule 1 | family.rs `pair_form`, `star_and_cell_faces`; sentence.rs `assemble` | **done** (0ae2441, abacf53) — occupancy keyed by cell, related pairs a `Keyed` set; `assemble` shares the helper; no `i<j` over germs | `face_work_is_linear_in_occupied_cells`; `a_row_of_three_touching_bodies_has_two_adjacency_faces_and_a_loop` |
| 1.3a | Correspondence by transport class; sections; face constancy by M1; glue with relative placement and cited magnitude; coherence by tree gauge | 21 steps 1–3, 7 | family.rs `extract_rule_with`, `transport_class`, `placement_of_class`; sentence.rs `Form::solve_coboundary` | **done** (0911e33, 037955c, 42423d2) | `correspondence_is_transport_class_not_shape`, `extract_rule_by_class_with_sections`, `extract_rule_glues_a_held_form` |
| 1.3b | Rebase by relative placement `w_j = t·w_i·t⁻¹`; `Rebased{by, of, shift}`; `word_in` realises the shift | 21 step 4, 3.2 | family.rs step 4; sentence.rs `Vertex::word_in` | **open** | a family where B does A's rotation about B's centre extracts one `Rebased` and speaks the test |
| 1.3c | `apply_form` speaks `Glued` via `Crystal::by_address` + `speak_form`, `Rebased` via `word_in`; no all-bodies fallback; `Speech.unspoken` | 21 speak_rule, rule 2 | family.rs `apply_form`, `speak_rule`; `Crystal::{by_address,on_axis}` on `Epigenome` and `Kernel` | **done** (3f2fa43, abacf53) — `speak_rule` speaks the form; empty fiber is unspoken. Rebase extraction remains 1.3b | `empty_fiber_is_unspoken_not_every_body`; `extract_rule_glues_a_held_form` |
| 1.3d | Unfold and contact as moves (present); contact anchor from the frame ring | 21 steps 5–6, 22 | family.rs `grows_to_frame`, `orbit`, step 6 | **done** (3eca3d5, 42423d2) | `unfolding_grow_to_frame`, `orbit_contact_and_no_orbit` |
| 1.3e | Form magnitude measured by speaking over the bundle; `as_element_form(f, mag)`; zero → not spoken | 21, 26, rule 8 | family.rs `form_descent` (from cognition.rs `measured_replay_descent`); sentence.rs `as_element_form` | **open** | no `set_magnitude([0,0,0,1])`; a form with zero descent is withheld |
| 1.4 | `replay_form` on the court; `blind_replay` dispatches on the version byte; `resolve_form`, `replay_memo`, `cited_by`, slash redockets citers | 23.1–23.2 | verdict.rs `replay_form`; cognition.rs | **open** | `replay_form_refuses_unruled_glue`, `witness_fetches_glued_form_before_install`, `replay_memo_reused`, `slash_of_glued_redockets_citers` |
| 1.5 | Working atlas: `working_atlas`, `Standing`, dialect `Stalks`, promotion, standing byte on the wire; `Crystal` over the working atlas | 24.1, 21 | sdk.rs `Epigenome::{working_atlas,working_fiber,touch,learn_one}`; dialogue.rs | **partial** (0b775fc, ee36a3c; standing byte on wire open) | working-atlas traces of chapter 40 |
| 1.6 | `Comprehension::Withheld`, sentence arm with `regauge_overlap`, depth-ordered syllabus | 24.2 | sdk.rs `comprehend`, `pull_syllabus`; cognition.rs `serve_ruled` | **open** | `comprehend_regauges_dialect_glue`, `withheld_then_depth_order` |
| 1.7 | Germination on forms: `pivot_on` glues crystallized forms at constant placement; `glues`/`glued_by`; `bundle` | 25 | bubble.rs | **partial** (a9dd47c wrap; bundle open) | `form glued … remainder before→after` trace |
| 1.8 | Torsion bridge: A17 filing and witness, A18 rank with `REFUSE_RANK_LOST`, A19 class records on the syllabus page and `seed_horizon`, chapter 23.7 on the court | 16.17–16.19, 17.9, 23.7 | sentence.rs, bubble.rs, cognition.rs, dialogue.rs | **partial** — A17 filing writes `Nerve::torsion`/`Form.torsion` as `Keyed<(σ,ℓ)>` by class key (abacf53); the element adopts that carrier whole. A18 open | `torsion_filed_only_when_witnessed_at_period`, `infinite_class_is_never_filed`, `cauterize_filed_torsion_crystallizes`; remaining: `rank_lost_refused` |
| 1.9 | **Pivot loop replaces `twist`**: `class_vector`, `occupied` in fixed order, trials, certified discharges, vortex cells, A20 certificates on the tape and replayed by the court | 3.9, 12.6, 16.16, 16.20, 25.1, law 10 | sentence.rs `pivot_loop`, `ClassVector`, `PivotCertificate`, `Nerve::torsion`; bubble.rs; verdict.rs certificate replay | **partial** (0ae2441, abacf53) — loop, vortex, lane-wise min, keyed torsion, `Crystal::on_axis` landed; court certificate replay still open | `pivot_candidates_are_read_from_occupied_axes_only`, `incomparable_discharges_are_held_as_vortex`, `descent_certifies_never_selects`; remaining: `pivot_certificate_replays_or_refuses` |
| 1.10 | Wire: `SPEECH_WITHHELD`, missing list, `depth_max` in digests, `home(depth)`; economics `settle_recursive`, `use_credit` | 26 | dialogue.rs, sheaf.rs, mycelium-server.rs, economy.rs, cognition.rs `pay_solve` | **open** | `settle_recursive_pays_by_deflation`; one settlement with two speakers on a soak |
| 1.11 | Retirement: `shorter_form`, cavity resolution, `cited_by` as use | 27 | cognition.rs | **open** | `shorter_form_retires_to_cavity_and_resolves` |
| 1.12 | Persistence: `depth` in GEN2, v2 tapes, `cited_by`/`replay_memo` rebuilt at load | 28 | element.rs, cognition.rs `restore`, sdk.rs | **open** | restored genome logs `forms N (v2 K, v1 M)` |
| 1.13 | Extraction memo keyed by `registry::task_id` and the working-atlas root | 21, rule 5, rule 6 | mycelium-client.rs; family.rs `extract_rule_cached` | **done** (3f2fa43) — memo is `(fam, atlas_root)`; client feed/task arms pass `&entity.epigenome` | `extract_rule_cached_spends_once_per_generation` |
| 1.14 | **Switch production callers to the form engine** (chapter 30 step 3): client feed/task arms call `extract_rule_with(family, &entity_crystal)`; speech via `as_element_form`; court replays v2 | 30.2 steps 2–5 | mycelium-client.rs, family.rs `extract_rule`/`speak_rule`, cognition.rs `sentence_rule_of` | **partial** (abacf53) — `extract_rule` is `extract_rule_with`; `agree_up_to_gauge` deleted; `speak_rule` speaks the form. Gated on 1.3b, 1.3e, 1.4 | `sentence formed` lines carry `depth`, `glued`, `sections`; `court refuse speech` histogram empty of parse errors |
| 1.15 | Delete `Nerve`, `Germ`, `Cycle`, adapters, `independent_cycles`, `after_support`, `face_word` | 30.2 step 6 | sentence.rs, family.rs, bubble.rs | **open** — after 1.14 | `cargo build -D warnings` green |

### Cartesian remediations on the sentence object (abacf53)

Closed in one commit against the coding-standards product rule
(`read(C, q) := C|_{key(q)}`; no `X × Y`; Frame is vertex 0). The nerve
extractor `agree_up_to_gauge` is deleted, not repaired.

| site | was | now |
|---|---|---|
| `Form::loops_of` | face search per path step; `form_cycle_holonomy` | faces named by `tree_path` in the same pass |
| `assemble` | `i<j` over germs; `faces.iter().any` | `star_and_cell_faces`; pair key `Keyed<()>` |
| grouping / `agree_up_to_gauge` | nerves × groups × counts | deleted; `extract_rule` = `extract_rule_with` |
| `SentenceRule` hue/shift | `Vec` scan | `Keyed` by chart / transport-class key |
| Frame | `position`/`find` over vertices | index 0 |
| `is_cauterized` / torsion | scan `complex.degrees` | `Nerve::torsion.contains` by class key |
| `try_close_chord` | `cycles.iter().any` after gauge | `n.cycles[idx]` |
| `select_discharge` | pairwise dominance | lane-wise min once; ties are a vortex |
| `as_element` | walk Congruent cells | `adopt_torsion` of the form's carrier |
| germination | `body × fiber × pieces` | `open_by_chart` then locate the body's fibers |
| presence | `.any` on charge / spoken pairs | `Keyed` sets |
| tape codec | silent `min(256)` / `min(4096)` | remaining-bytes bound; refuse |
| `gauge_fibers` | `find` on a `Vec` | `Keyed` by hue-pair key |

Still cartesian when those files are next touched: `pair_scene`/`greedy_within`, `merge_sigma`, `speak_rule` centroid ranking of section hits, `hue_from_neighbors`.

## Stage 2 — cohomology in the court (Parts II and IV)

| id | task | chapter | status | DoD |
|---|---|---|---|---|
| 2.1 | `Form::delta1`, `Complex::delta2`, cochain identity tests | 10, 12.1 | **partial** (`delta1` present) | `cochain_identities`, `regauge_then_delta1_is_conjugate`, `bianchi_holds_for_curvature_of_a_connection` |
| 2.2 | Class keys everywhere; `push_cocycle` as M5 (replace representative, keep key) | 15.2–15.4, 23.4 | **partial** | `conjugacy_key_iff_conjugate`, `rotor_class_key_invariant` |
| 2.3 | Mayer–Vietoris: A13 `Keyed::meet`, A14 `solve_gauge_between`, `Overlap`, `MayerVietoris`, `glue_fetched` by the 13.10 table, `regauge_overlap`, `pivot_on` merges only on one section | 13, 23.3 | **open** | `meet_is_intersection_and_costs_shared_prefixes`, `mv_order_independent`, `citation_lemma_classes_unchanged`, `thoughts_merge_iff_one_section` |
| 2.4 | Goodness: `Cover::charts_of`, `ChartGraph`, A15 components, `goodness`, per-component cells, retraction by `home`, `REFUSE_OVERLAP_CYCLIC` | 14, 23.3 step 1 | **open** | `goodness_*`, `leray_classes_independent_of_cover`, `leray_artefact_disappears_when_cover_made_good` |
| 2.5 | Naturality: `refine_chart` restrict/carry/discharge with rollback; cover-change re-representation | 15.5–15.6, 23.5–23.6 | **open** | `refine_carries_or_discharges_every_class`, `refine_rolls_back_on_not_natural`, `retract_rerepresents_or_orphans` |
| 2.6 | Citation lemma in use: `Form::glue` refusal, memo, mind and bubble instances | 13.6, 23.2 | **partial** (`glue` refuses unclosed) | `court memo hit` on the second citer |
| 2.7 | Rank theorems as debug assertions (R1–R5) | 16.18, 42.2 | **open** | `rank_invariant_*`, `rank_conserved_under_refinement_carried_plus_discharged`, `rank_of_union_is_sum_over_components` |

## Stage 3 — migration window (Part V)

| id | task | status |
|---|---|---|
| 3.1 | Chapter 30 steps 1–6 (see 1.14, 1.15) | **partial** — steps 1–3 on the tree (`Form` beside `Nerve`, v2 tape, `extract_rule` on forms, `agree_up_to_gauge` gone). Steps 4–5 (client/court Form-only reads) and step 6 (delete Nerve) wait on 1.14/1.15 |
| 3.2 | Chapter 33 cutover a, b, c, then clients; `RMP1` retired | **closed, not now** (genesis recuts) |

## Stage 4 — extensions (Part VI), after the ARC soak gate is green

| id | task | status |
|---|---|---|
| 4.1–4.5 | B1 cells → B2 harmonics → B3/B4 frames → B5 `Cl(4,1)` | open |
| 4.6–4.10 | F1 cyclotomic → F2 conductor → F3/F4 letters and spelling → F5 opening by demand | open |

## Cuts and soaks

| cut | contents | gate to start | what the soak reads |
|---|---|---|---|
| **A** (fleet in memory) | `804843d` in the running units: live nerve path with transport-class correspondence, bundle faces, mint by replay, working atlas, fan-out, impedance routing. Tree is ahead at `abacf53`; do not recut until asked | full suite green (it is); `state/`, ledgers, genomes wiped | `court refuse speech` histogram; `sentence formed` > `withheld`; cohesion `c` leaves 0; `promoted` lines; `mint none` lines; RSS flat |
| **B** | Cut A + remaining 1.3b, 1.3e, 1.4, 1.9 court replay, 1.14 soak traces, 0.8. Already on the tree: 1.1b, 1.2, 1.3c, 1.9 loop/vortex, 1.13, 0.9, 0.10 | full suite green; the chapter 40 forms and torsion-bridge traces reproduced on cold boot | `form spoken … depth d glued n`, `pivot … discharged`, `vortex … held`, `court pivot certified`, `court memo hit`, one settlement with two speakers |
| **C** | Cut B + Stage 2 (2.3–2.7), 0.4, 0.5, 0.6, 0.7 | as above plus the cohomology traces | `court goodness`, `court mv`, `class carried/discharged`, class keys equal on all hyphae |
| **D** | Stage 4 | non-ARC corpus present | chapter 40 extension traces |

## Percent complete

Weighted by chapter: about 16% of the document is implemented and verified
on `main` abacf53 (form engine is the only extractor; cartesian scans on
the sentence object are gone; pivot loop and keyed torsion are on the
tree). Cut B corresponds to about 20%, which is the first point at which
a soak exercises words forming on fibre bundles rather than the nerve
path; Cut C to about 45%; Stage 4 to 100%. Next landings before Cut B:
1.3b rebase extraction, 1.3e measured magnitude, 1.4 court `replay_form`,
0.8 `Element::eq`.
