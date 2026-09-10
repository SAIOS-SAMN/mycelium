# Resume notes (continuity across sessions)

Last update: 2026-09-10, Stage 14 through L9 (`impl/soak` at 3e2cf93:
the derivation recut, tape v7, five commits c50c8b4…3e2cf93, after the
"sentences only" ruling d2a9c2e, L8p, L8, L8r, L8w, L8y and their
corrections; `main` stands at 685a07c, before Stage 14, and is
fast-forwarded on the operator's word). The trajectory is ruled in
`MU-RECURSION.md` 1.9 (the ladder of re-derivation, the tape retired as
content) and carried by rows L14–L19; the implementor runs the relay
list under "Immediate next actions". Two agents work this tree: the
implementor (code, rows marked `landed`) and the auditor (reproduces,
rules from the document, closes rows).
Living task list: `docs/MU-RECURSION-TASKS.md` (Stage 14 rows L0–L19 and
the pre-L7 rows L7a–L9a). Spec: `docs/MU-RECURSION.md`. Standard:
`docs/MU-STANDARDS.md`. Historical alpha landings: `docs/TRAJECTORY.md`.

## Boot check

```
cd /opt/mycelium-v0.1.0
git log --oneline -1 main impl/soak          # impl/soak is ahead of main
systemctl --user list-units 'mycelium-server@*' 'mycelium-client@*' --no-legend
RUSTFLAGS='-D warnings' cargo test --release 2>&1 | grep -E "test result|FAILED"
(cd xtask && cargo run)                        # gates, then clippy -D warnings
# expected at 3e2cf93: lib 466, bins 9, alpha_plan 13, g_cga 6; xtask exit 0
(cd xtask && cargo test)                       # the gates' own tests: 32
```

## The fleet

Units `mycelium-server@{a,b,c}` (hyphae, `127.0.0.1:7500/7501/7502`) and
`mycelium-client@{1..5}` (voices), user units in
`~/.config/systemd/user/`. ExecStart is
`/opt/mycelium-v0.1.0/target/release/mycelium-{server,client} --home
/opt/mycelium-v0.1.0/var/%i`, cwd `/opt/mycelium-v0.1.0`. Homes are
`var/<unit>` under the repository (moved out of the operator's home on
2026-09-09; `.gitignore` covers `var/`). Configuration in a home: `hypha.key`
or `voice.key`, the signed `mesh` record (bind, peers, genesis, epoch), and
`corpora/*.manifest`. Everything else there is state and is wiped by a
recut. `scripts/recut` stops the units, wipes state, drops each hypha's
origin, mints the genesis in a one-shot run of the binary and starts the
units. Do not `pkill`; use `systemctl --user`. Do not recut or soak unless
asked. The fleet is STOPPED and the binaries in `target/release` are the
ones the last verification built; a soak needs the operator's word.

Other `mycelium-*` units in that directory are earlier fleets
(`mycelium-hypha*`, `mycelium-client-{a,b,c}`, `mycelium-v010-*`,
`mycelium-treasury`, `mycelium-uplink-sod`); they are not this fleet and
were not touched.

## State at handoff

- **2026-09-10, the ring instance of the adjunction is done and audited
  (65edee0).** The third fact holds: a mind taught a generator the
  seventeen cannot spell derives through it (`family/ring.rs`, on
  `factor_versor`'s `Err` arm), the letter travels the syllabus page and
  is held as a generator, a court holding it composes the citation. L13
  measured: 1554 arcs of the alphabet's own primes, 0 standing. The
  first corpus reading (250feeb, twelve tasks, in process): held 0,
  adjoined 0, retired 0, cover rank 0, comprehended 0; 83 demands, all
  source, creation or deformation, none a prime — the corpus asks for
  the cover instance (L11c). L8v re-measured: task `0607ce86` does not
  return under L8w's bound, the loop is L20. `atlas_root` keys the
  extraction cache by digest, L21. Auditor's reorder (§8.1): L15 · L11c
  · L20 · L21 · L16 · L12 · L17 · L18 · L19. 487 / 0 / 9 / 0 / 13 / 6,
  xtask 0, the gates' own tests 32, reproduced; the twelve-task walk
  reproduced line for line.
- **2026-09-10, the run-time adjoined count is 1 (d35ac33).** A court
  hearing a `RotorPrime` demand reads the Gaussian factor off the
  witness (`core::gaussian`, 7c61827), dockets it as a provisional
  generator (one word vertex, no citations, the factor as its rotor
  under a presence byte, no star face), and a second court's agreement
  installs it under `ActKind::Ruling`; two voices, two courts of
  distinct identity, one address, `adjoined()` 1 on each, read from
  content. In process, no corpus. Still owed on L11: the syllabus
  teaching the letter to a mind whose generator index answers it, and
  (3/3) the ring branch in `derivation_of`. 480 / 0 / 9 / 0 / 13 / 6,
  xtask 0, reproduced by the auditor. L9 closed (d05ba08, 5c4c4eb).
- **2026-09-10, the trajectory ruling (1.9) and the ladder.** After
  L9 the auditor read the tree against the ladder of re-derivation and
  the claim that the architecture was "built already". The reading, by
  row, with what replaces each gap:

  | claim | state at 3e2cf93 | where, or the row |
  |---|---|---|
  | depth 0, the peel from the identity | built, kept as the base case | `peel_versor`, A22 |
  | depth 1, a derivation through held generators | built | `Derivation`, `derive_against`, A12; the ordered-peel limit is L13 |
  | depth 2, replay on a second ledger | built | `replay_form_at`, `derivations_compose`, `verify_with` |
  | depth 3, two courts re-derive one object | built for sentences, not for generators | 47.3 second-vantage install; 37.7 absent, adjoined count 0: L11 |
  | depth 4, the relator from two proofs' holonomy | machinery built, adjunction not | `cech::holonomy`, `coheres`: L17 |
  | depth 5, rewrites as 2-cells | five kinds in the host, no adjunction | `rewrite.rs`: L17 |
  | depth 6, procedures as content | not built; Part III amended to admit it | L18, doctrine first |
  | addresses as canonical invariants, never digests | built | `form_vertex_key`, rule 2 |
  | receipts as a causal poset naming addresses | built | `KStepReceipt { parents, delta }`, 45.1 as amended |
  | depth read from receipts, uncollapsed | not built | L15; `Standing` is its three-valued projection |
  | the composed word stored on a vertex | present at v7, struck | L14: the canonical rotor replaces it |
  | the tape as the organs' working memory | present: `Element.word` decoded on every read | L16 |
  | the residue as a spelling in letters | kept: depth 0 | 1.9 |
  | the glued form as a generator one level up | built | `Form::glue`, `Glued`, the generator index |
  | sentences as pushouts with zero mismatch | built | faces with transports, `coheres`, `cech::extend` |
  | disjoint bodies compose lane-wise | built as a rule, not as a tensor | 16.16 |
  | termination | built by `Budget` and `Unmeasured`, not by strong normalisation | Part III |
  | loops, curvature refusals, torsion | built | chapter 12, `Closure`, `REFUSE_LOOP`, `REFUSE_EDGE` |
  | separated ledgers, no global state, partial order | built | separate kernels, law 9 |
  | utterance as a speech act | present | L19: propagation; the room of 23.8 stays |
  | one constructor at every depth | doctrine (1.9, 37.7) | L11, L17, L18 |
  | Knuth–Bendix, critical pairs | refused | chapter 44, restated in 1.9 |
  | hashed addresses | refused | rule 2 |

  Corrections applied to the text that proposed the ladder: the organs
  of consilience are courts, never voices (law 6); threads add nothing,
  two kernels exchanging frames through the codec are the isolation; the
  remainder is the Gaussian factor or the HNF residue, not a formula on
  versors; the dwell is the room, not a speech trope; the signature of
  the algebra is not named by the code and is not asserted.
- **2026-09-10, L9 — the derivation recut (c50c8b4 doctrine, 7dabdea,
  67176a5, c189be8, ed813ee, 3e2cf93).** A word vertex carries its
  derivation (cited form, cited vertex, exponent; residue in letters;
  and, struck by 1.9 and open as L14, the composed word); tape v7; both
  rotor demands carry the residue block as witness; F factors through
  the mind's generators inside `pair_form` with the crystal, per call,
  with the percept memo keyed on grids only; G composes citations on the
  ruled cover and refuses at the vertex; a cited generator is paid at
  install once per cited form; an unruled citation waits in the
  awaiting docket instead of being refused; Unmeasured propagates as
  `None`; `power_of` is bounded by the rotor's order. Open from the
  audit, in order: the per-sentence Awaiting adjudication naming the
  cited form, `rehear_awaiting` through `adjudicate_one`, a payment
  test at install, L14, the proof through the codec with two minds and
  two courts, the instrument's engine coordinates, then L11. 466 / 0 /
  9 / 0 / 13 / 6, xtask 0 at 3e2cf93.
- **2026-09-10, L8 — the contraction protocol (a488f00; doctrine
  feecbf6).** The wire is a medium, not an echo: the court answers one
  `Adjudication` per sentence (Stood / Ratified / Refused { kind, at,
  receipt } with the coordinate as content), the mind seals a refused
  address on its own chain and in its utterance index and executes the
  move the refusal names on the held form (`Form::contract`: a refused
  word becomes a `Creation` demand carrying the remainder it leaves on
  the mind's family; a refused edge a remainder edge; undeclared cells a
  `Source` demand; a malformed complex is held inert), holds the
  contracted form with its `Contraction` certificate, and utters the
  contracted sentence next stroke citing the receipt; the court that
  issued the receipt verifies `S' = C(S)` at the coordinate before it
  replays. The intelligibility test runs three round trips through the
  codec (VertexBody → RemainderUndeclared → Stood) and a second court of
  its own identity reaches the same address and standing on the same
  frame. Found and closed on the way: the speech frame carried no task,
  so no court had ever replayed a voice's sentence on its copy of the
  family. Not in this landing (row L8q): the sentence-native quote and
  probe, and the corpus intake's chain listings — a codec change, the
  recut with L9. Limits named in the commit: the refusal record, the
  sealed set and the certificates are in-memory; the record is unbounded
  by address; move 1 speaks `Creation` until L11 reads harmonics. PAUSED
  here on the operator's word. Next, on that word: L8q, then L9 (the
  recut and the cold-boot reading of 50.7/50.9), L10, L11; rulings still
  asked: L7d, L7f, L9a.
- **2026-09-10, "sentences only" and L8p.** The operator's critique: the
  work was single-ended emission, and the court composing by F (the
  percept act: `percept_elements` → `reduce_*` → `commit_germ`) was the
  root of every misalignment with law 8. Ruled and written (d2a9c2e): law
  8 sharpened, law 6 and 37.7 P3 corrected to two courts, 20 corrected,
  23.8/26 sentence-native, 50.9 the contraction protocol, rows L8p and L8.
  L8p landed after it: the percept, gossip, demand and routed-demand acts
  are refused at the door (`REFUSE_NOT_A_SENTENCE`), `judge/percept.rs`
  deleted; `ingest`, `adjudicate`, `attest`, `commit_germ`,
  `reduce_all/germ/proven`, `percept_elements/family`, `resolve_cites`,
  `taught_anchored_words`, the client's `submit`/`attest`, the seat's
  percept/demand forms, the demand codec and the per-pair feed loop are
  gone; `perceive_speech` → `perceive_sentence` (door, replay by G, then
  `list_sentence`: a use of held language, a second voice as a party, the
  room and reserve arms, listing + demands + escrow + docket + Offer) is
  the one path from an utterance to the book; the second voice's use
  re-escrows with every party. Tests reach the docket through the
  fixture's voice-side F (`common::speak_pair`, `sentence_of`,
  `strip_words`). Departures named in the commit: five chain-era tests
  deleted with their claims (rebind/anchor, puncture-before-mint,
  compound chain peeling, the projective leap, the wire judge), two more
  retired with the codec (demand roundtrip, verdict decode), the
  attest-idempotence claim moves to L8 (50.9's sealed address), the
  probe/drain fixtures list a chain by `book_list`/`intake` (the corpus
  intake still lists chains until L8's sentence-native quote), and the
  kernel door's per-lane deferral is unreachable after the reflex
  opening regenerates (the test now asserts the heard sentence and no
  door fee for the broke speaker). 439/0/9/13/6/0, xtask 0. Next: L8.
- **2026-09-10, the ruling on generative capacity and the pre-L7 set.**
  The operator ruled that the seams are severed first (L6→L8), that no beam
  search returns, and that chapter 37's opening of a letter is the court's
  own transition: law 6 amended (immutable to a voice, grown only by the
  court on arity-2 consilience), 36.3 the prime wall, 37.7 Upgrade P, 50.5
  and 50.8 (4cd2203); rows L10 (prime letters, a recut) and L11 (the cover
  for a deformation) after L9. The scalar rulings of 2026-09-09 opened as
  rows L7a–L7f, L8a, L9a (017db17); landed: L7a d19f2fb, L7b b792e8b, L7c
  2173a5a (doctrine 3a0766a), L7e dd52a74 — the four §4.4 gates with the
  OWED ratchet (605 hits in 246 rows measured at 2173a5a; the tables only
  shrink). Rulings still asked: L7d (recolour pairing), L7f (Infinite
  order on lane H), L9a (anchor::derive). 440/0/9/13/6/0, xtask 0, xtask
  tests 31 at dd52a74. Next: L7, then L8, then L9 on the operator's word,
  then L10, L11.
- **Stage 14 through L6b landed 2026-09-09** on the operator's word (see
  "Immediate next actions" for the hashes and what each did). Departures
  named in the commits: the held cite's wire form is the identity
  transport and the reconstructed body until L7/L8 carry the sentence;
  the Held stratum yields to a ruled or stood word on the same chart
  (24.1's order); the connection-glue arm of `axis_germ` is gone (no held
  connection exists once ideas are forms); `cited_word`'s "citation is a
  use" touch moved into `cite`; L6b re-asserted twenty-seven tests where
  the row said seven; a percept-born sentence mints nothing on install
  until L7 binds its pair as its family. The court's replay with a family
  refuses any remainder (`REFUSE_NOT_CLOSED`), which under 50.5 must not
  refuse a sentence for the remainder its demand vertices name: L7.
- **R1–R7 landed 2026-09-09** on the operator's word, one item per commit,
  suite green on each (421/0/9/13/6/0, xtask 0 at d6e19a1): doctrine
  de29cab; R1 4f3337c (the docket orbit passes over ruled and cavity
  listings); R2 634b3d0 (`court nerve deferred by {addr} acts {n}`, once
  per aperture per destination); R3 fbba134 (`yield_surface` = reserve
  deficit + entropic flux, the percept reply carries it); R4 dc58f86
  (`backfill::pages` pages until the chain closes, then re-admits the
  reporting frame); R5 c99ba16 (fold refusal names both origins; a witness
  with no origin says so); R6-prep 6d824f1 (`court/install.rs`), R6
  a9c9b54 (`listings_on(inv)` = `chart_listings[inv] ∩ open`,
  `volume_of_cell(inv)`; identity edge for one body at two vantages;
  remainder edge for a versor-related pair no rotor spells); R7 d6e19a1
  (phase read when the docket orbit count crosses a multiple of
  `aperture_cycle()`). Rows in the task list. The fleet was relaunched on
  d6e19a1 (no tape change since ffed51c, so a restart keeps the ledger),
  read for one aperture, and STOPPED on the operator's word. The
  operator's reading of that soak: the market is flooded with garbage, not
  dialects — voices list unreduced residual chains as if they were words
  (226 remainders expressed against 0 forms spoken, 235 held against 23
  reduced, genomes of 250–420 ruled words that comprehend 0 of 2 listings,
  no axis opened, no class, no refinement), and the court rules and
  sections them. The next work is the language itself, not the court's
  plumbing. New faults from that reading: the back-fill fetches from the
  reporter rather than the signer (`… 1083..1083 admitted 0` repeats, every
  hypha's view of its peers' chains at K ~1k against heads at 10–12k);
  back-fill throughput (~3 receipts/s) below the receipt rate (~10/s);
  the phase window thin on a period-1 docket (b `h 12262..12262 0/0`);
  `axes_opened()` is a run counter; b at 204 % CPU with unnamed threads.
- **Earlier pause 2026-09-09** after V1 (9d66c45) and V2 (ffed51c) landed
  and the fleet was recut on ffed51c at epoch 6 (genesis be05…), every
  commit of the pass in one soak.
- Clean-slate readings an aperture after the recut (`scratchpad/launch.sh`),
  in kernel coordinates: V2 holds — `[h N]` climbs on every voice (3177,
  6230, 6137, 6312, 6151); V1 holds — `axis opened` 1 on voices 2, 3 and 5,
  the first axes ever, with `already spoken` refusals thereafter; two-organ
  folds sealed (`court fold witnessed` on all three); `home installed` 1 on
  b and 5 on c; `court phase … congruence 13/48, 23/68, 11/53` (charts with
  two sections against strata); back-fill pages admitted (3, 26, 36
  receipts). Faults found, to land next, one item each: (1) the docket
  orbit re-rules every ruled listing on every circuit — period 1, 35k
  circuits, `court section 0428…` 20,800 times on a, 90–130 % CPU; (2) an
  unpaced digest poll in `main.rs:248` — 376k `court exchange deferred by`
  on a in one aperture, outside S13.1's pacing; (3) the voices' dwell is
  the raw pressure surface (`dwell [47728, 26923, 0, 53465]` on b's
  voices), so they stop asking for tens of thousands of strokes; (4)
  `court receipts parent 0` still — one back-fill page per gap reported
  is slower than the peer's K, so the gap never closes; (5) fold proposals
  refused `REFUSE_ORIGIN` by the witness after a first co-sign (a refusing
  c 26 times, c refusing a 46) — the origin compared or carried differs on
  later proposals; (6) `cells filled/class 0` although 13–23 charts hold
  two sections — the fills have no input; (7) the phase window per orbit
  is a receipt or none, so comprehension reads 0/0.
- Stage 13 (velocity, phase, eras; ratified 2026-09-09) landed through
  S13.10, one item per commit, suite green on each (416/0/9/13/6/0, xtask
  0 at 69983cd): doctrine c46117c; S13.1 f42c3fe, S13.5 7da548a, S13.4
  ba38941, S13.2 04b4561, S13.3 23bd123…0e03f47 (eight, one per site),
  S13.15 34c84cf, S13.6 c6a3324 (tape v6: `ActKind`, `MYC4`), S13.7
  1601971, S13.8 6be34ad, S13.9 4795613, S13.10a dbe813d (`ACT_RECEIPTS`,
  5.2g closed), S13.10b 69983cd (two-organ fold); rows cdbe41a; `main`
  fast-forwarded there.
- Open in Stage 13, in the ratified order: S13.11 (adoption from a fold
  root; the era must carry the two signed prefixes for a joiner to verify
  them), S13.12 (extraction), S13.13 (mesh record fold roots), S13.14
  (simulator).
- Departures to ratify, each named in its commit: `Settled` as a
  thirteenth `ActKind` (6296cce); the read token `Physics::stroke` not
  priced per lane (04b4561); the fold trigger read as the reserve
  shortfall on a completed docket orbit (936a849); a recall record closing
  a lane only while a claim is in flight and clearing when the voice's
  occupancy returns to zero, a listing drawn once (34c84cf); epoch rows
  anchored at the booking K; a fold witness as any bonded peer other than
  the home (delegation closure unread).
- The fleet still soaks on 2eff0de; the rebuild onto impl/soak is a recut
  (tape v6, `MYC4`, `KCH1` root, `ARC1`, rows on the wire) and waits on
  the operator's word.
- Read on the soak of 2026-09-09 at the stopping point (journal tail only
  — journald retains 281 MB, hypha a's tail spans K 3544–3574, b's
  6899–7158, c's 1229–1270): `court exchange deferred` 602,720 on a at
  cycle 2²⁰ (S13.1's fault, not yet deployed); b `organs reached 0 silent
  2` at 87 % CPU and 354 MB (its reflex pool defers every act); `court
  receipts parent 0` everywhere (5.2g, S13.10a not yet deployed); `axes
  0`, `axis opened 0`, `chart refined 0`, `court cell class 0` on every
  unit; voices 1, 3 and 5 logged 31, 69 and 91 quote closings with two or
  more held cites and opened no axis — `open_axis` never fires though its
  shape is met (a fault to find: `cited_word` on held cites, or the glue);
  every voice's stroke line reads `[h 0]` — `set_height` is called once at
  boot with zero and never from the book (a fault); voice RSS 181–370 MB
  from 25–53 MB at the baseline.
- Stage 12 landed on 2026-09-09, one item per commit, suite green on each:
  S12.4a fda0778, S12.4b b340ec0, S12.4c 5316fed/c7646b4, S12.4d 013cb09,
  S12.5a dc47ac1, S12.5b 14bf2e0, S12.5c 31abfd5, S12.2 470f324, I35
  d5d2792, S12.1a 72f30f2, S12.3a a42b5a9, S12.6a 1cf2c1e; rows ba531d5
  and the docs commit after 1cf2c1e.
- Open in Stage 12: S12.1c/S12.1d (Stage 10 P3–P6, the percept carries its
  verdict; a codec change and a recut), S12.3b (Stage 2 Mayer–Vietoris at
  the court), S12.6b (the treasury fold; MU-ECONOMY §2 `epoch` kind to be
  ruled first), S12.6a's remaining sites (`expressed*`, `Entity::ideas`,
  dirty bubbles). Rulings asked: S12.5d (room of an unbonded voice, §3.1),
  S12.6c (may a receipt chain checkpoint), I22 (slash and supply; recall
  shortfall).
- Stage 11 still open: I30 (`conclude` seals nothing), I32 (`--recut` on a
  node with no origin does not bump the epoch), 5.2g (receipt back-fill).

## Immediate next actions

**Stage 14 — language formation** (ruled 2026-09-09, `MU-RECURSION.md`
chapter 50, TASKS Stage 14 rows L0–L9): the sentence is the unit of
speech; F measures versors, spells by the ordered factorization A22,
connects edges in the stabilizer, emits demands as typed vertices; G
factors, replays, checks congruence by class vector and coherence by
holonomy; the three seams (`withhold_form`'s attest loop,
`body_only_vertex`, the court's `Irreducible(from_residual*)` listing) are
severed in the order L4, L6, L8. L1 is tape v6, a recut. Landed, one item
per commit, suite green on each: L0 2ac81e2, L1 0b5cf2b, L2 78af4e5, L3
634b3d0, L4 15a3a08, L5 6e37e0a, the L6 rulings 2a978d9, the order ruling
3f365f8, L6 85bdcc5 (the Held stratum read at the coordinate:
`Form.by_class`, `Contemplation::held_at`, `Reading.held_at`,
`sentence::vertex_cites`, `blade::versor_carrying`, `HeldCite`;
`Entity::ideas`, `hold_thought`, `conclude`, `cited_word` deleted; the
cite tests in `client/sdk/tests/cite.rs`), its binding 0f85a4b, L2b
d37d082 (`Rotor::order` from the class invariants; `Word::order`
delegates; `cech::extend` reads the transition's own order), L6b 2686272
(a body is one hue; the court lists one sentence per percept; the
puncture-before-mint move on residual chains only; twenty-seven tests
re-asserted, none deleted). 437/0/9/13/6/0, xtask 0 at 2686272.

Landed since: L7 (a0da3de and its rows), L8p (4b5e44c), L8 (a488f00),
L8r, L8w, L8y and their corrections, and L9 (3e2cf93) — see "State at
handoff".

**Relay list for the implementor, in order, one commit each** (the
awaiting protocol, the rehear through `adjudicate_one` and the payment
test at install are directives 1–3 of the audit at 3e2cf93 and precede
this list):

1. L14: the canonical rotor replaces the composed word on the vertex;
   `form_vertex_key` from the rotor; `verify_with` against it; `Word`
   for a cited vertex resolved through the crystal or the cover.
2. L9's proof through the codec: two `Epigenome` minds taught different
   ruled forms by one court, one family extracted with the mind as
   crystal, `speak_with` to `hear_frame` on two courts of distinct
   identity, one address, two derivations, both installs paying each
   teacher under their own key; vertices compared by `same_vertex`.
   The auditor closes L9 on reproduction.
3. The instrument prints the engine's coordinates: generators held,
   adjoined, retired, cover rank, families comprehended per generator,
   adjoined as a printed zero.
4. L11, the ring instance: `core::gaussian` (gcd over ℤ[i] read off the
   witness block), `ACT_LETTER` with its receipt kind and syllabus
   page, the two-court effector; two voices' `RotorPrime` demands with
   one witness, two courts agree by A1, the generator adjoined at run
   time with no commit, the count prints 1, a third organ derives
   through it by citation (no escape byte: L14). No corpus.
5. L13: measure the ordered-peel limit against the adjoined basis.
6. L15: depth read from receipts; `Standing` as its projection; the
   instrument by depth.
7. L16: the organs hold forms, not tapes; `from_element_form` call
   sites outside the boundary reach zero, in green splits.
8. L12: doctrine first (42.6, the stratum per lane, the meet read); the
   draft goes to the operator; no code before the ruling.
9. L17: depth 4 and 5 by the one constructor, in process, no corpus.
10. L18: depth 6, doctrine first; the carrier's shape is the operator's
    ruling.
11. L19: propagation replaces utterance; the `ActKind` rename waits for
    a recut.

Held until L12 lands and on the operator's word: L8q, the wire beyond
what v7 and the Awaiting adjudication force, the recut of the fleet,
any soak. The fleet is stopped.

The earlier reading list (R1–R7 on d6e19a1): `court orbit docket …`
period rising above 1 once ruled listings are passed over (R1); `court
nerve deferred by …` once per aperture per destination in place of the
per-act line (R2); voices' `dwell` as the surface, tens not tens of
thousands (R3); `court receipt backfill … admitted` pages and `court
receipts parent N` rising (R4); `court fold refused … [origin … carried
…]` naming the two origins where a witness still refuses (R5); `court
cell filled`/`court cell class` once two sections stand on a chart (R6);
`court phase …` once per aperture of docket orbits with `comprehension
n/m` over a span (R7). Then S13.11 onward, and the item R6 names: spelling
a versor-related pair's versor as an alphabet rotor. The older reading
list below stands for the items it names.

Earlier list (Stage 12 recut): read: `cites > 0` on attests (bodies as words, S12.3a),
shared strata in the cover (fewer unique charts), `pivot … discharged` or
`vortex` lines (forms with loops, S12.2), `[h N]` climbing on stroke lines
(I39), `ACT_DIGESTS` served per hypha in ten minutes (I37), `court offer
deferred` in place of `court offer refused … by 0000…` (I36), `audit` green
on every hypha (I38), no `court room refused` from the intake (I31), voice
RSS over hours (S12.6a). Then S12.1c/d, S12.3b, S12.6b in the task list's
order.

## Standing operational rules

- Read every file whole with Read before touching it; change files only
  with Edit/Write (the operator's rule of 2026-09-09: "or else you are
  going to miss context"). Bash for git, cargo, systemctl and
  grep-to-locate; never sed/perl/awk over source or docs.
- Load every file in `docs/` whole after a compaction before doctrine work.
- One item per commit (§5); the document before the code (§5.5); one
  foreground `cargo test --release` then `cd xtask && cargo run` (§6.1).
- Commit locally only; trailer `Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>`.
- Nothing outside the repository changes without the operator's word.
